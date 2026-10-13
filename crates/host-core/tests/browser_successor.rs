//! Issue #1401: every browser preparation branch prepares with concurrent control delivery, at
//! boot and as the successor of a running plan.
//!
//! The three branches are one spectrum capture, a spectrum collection and the browser's selected
//! per-strip meters with no spectrum. Each successor entry is the boot entry with a
//! [`SuccessorBase`], so the cases below render a predecessor, swap the successor in through the
//! plan exchange and compare every block with the post-edit session prepared fresh through the
//! matching boot entry.
//!
//! * **Gate 1, gap-free, each branch.** Adding a muted track renders every block bit-identical to
//!   the reference, and the swap block reports `Carried`. The same run with a fresh successor
//!   differs at the swap block, so the oracle can fail.
//! * **Gate 2, the successor's capture works.**
//! * **Gate 3, concurrent and the same bits.** The new selected-meter boot entry forms no fused
//!   fader and matrix bank and renders what the `_between_render_calls` entry renders.
//! * **Gate 4, refusal.** A successor without its spectrum target is refused with
//!   `host.spectrum.target`, and the predecessor's producers still feed the predecessor.
//! * **Gate 5, realtime.** The swap block of a spectrum plan allocates and frees nothing.
//!
//! Session A is the console-free shape of `successor_swap.rs`: the full fixture has a console EQ
//! slot, whose state carries only with #1279, so a successor of it restarts that state and the
//! comparison would fail for a reason this slice does not own.

#![cfg(feature = "test-support")]

#[path = "support/successor.rs"]
mod successor;

use core::num::{NonZeroU32, NonZeroUsize};

use builtins::{MeterMetricSet, MeterTap};
use builtins_compiler::{test_only_fader_matrix_witness, test_only_reset_fader_matrix_witness};
use dsp_reference::randomized::Draw;
use engine::realtime::{CarryOutcome, PlanarBufferMut, RenderIo, RenderTime, SwapOutcome};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes, HostMeterRequest,
    LiveValuesSuperseded, PlanStateInventory, PrepareDiagnostics, PreparedHost,
    SPECTRUM_WINDOW_FRAMES, SourceSubmission, SpectrumCapture, SpectrumCaptureCollection,
    SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionRequest, SpectrumCaptureRequest,
    SpectrumChannels, SpectrumTarget, SuccessorBase, compile_host_session,
    prepare_host_runtime_with_live_controls_and_spectrum,
    prepare_host_runtime_with_live_controls_and_spectrum_collection,
    prepare_host_runtime_with_live_controls_and_spectrum_collection_successor,
    prepare_host_runtime_with_live_controls_and_spectrum_successor,
    prepare_host_runtime_with_selected_meters,
    prepare_host_runtime_with_selected_meters_between_render_calls,
    prepare_host_runtime_with_selected_meters_successor, spectrum_capture_collection_resources,
    spectrum_capture_resources_for,
};
use session::{EditRamps, RouteSource, SessionModel, StableId, parse_session_json};
use successor::{
    BLOCKS, Feed, QUANTUM, SUCCESSOR_REVISION, SWAP_BLOCK, Session, Successor, caps,
    first_difference, reference_run_of, render, swapped_run_with,
};

const FIXTURE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const SOURCE: &str = "fixture-source";
const MUTED_TRACK: &str = "a-muted";

fn id(value: &str) -> StableId {
    StableId::parse(value).expect("stable ID")
}

/// Session A: tracks `eq0` and `eq1` on [`SOURCE`], both console sections empty, no inserts, input
/// filters off (the shape of `successor_swap.rs`'s `session_a`).
fn session_a() -> SessionModel {
    let mut model = parse_session_json(FIXTURE).expect("fixture parses");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model
        .tracks
        .retain(|track| matches!(track.id.as_str(), "eq0" | "eq1"));
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
        }
    }
    model
        .routes
        .retain(|route| matches!(route.id.as_str(), "eq0-main" | "eq1-main"));
    model
}

/// Session B: A plus a muted copy of `eq0`, which sorts first and contributes exact `+0.0`.
fn session_b() -> SessionModel {
    let mut model = session_a();
    let mut added = model.tracks[0].clone();
    added.id = id(MUTED_TRACK);
    added.fader.left_mute = true;
    added.fader.right_mute = true;
    model.tracks.push(added);
    let mut route = model
        .routes
        .iter()
        .find(|route| route.id.as_str() == "eq0-main")
        .expect("template route")
        .clone();
    route.id = id(&format!("{MUTED_TRACK}-main"));
    let RouteSource::Track { track_id, .. } = &mut route.source else {
        panic!("a track route");
    };
    *track_id = id(MUTED_TRACK);
    model.routes.push(route);
    model
}

/// A without track `eq1` and its route.
fn session_without_eq1() -> SessionModel {
    let mut model = session_a();
    model.tracks.retain(|track| track.id.as_str() != "eq1");
    model.routes.retain(|route| route.id.as_str() != "eq1-main");
    model
}

#[derive(Clone, Copy, Debug)]
enum Branch {
    Single,
    Collection,
    Meters,
}

const BRANCHES: [Branch; 3] = [Branch::Single, Branch::Collection, Branch::Meters];

fn single_request(track: &str) -> SpectrumCaptureRequest {
    let target = SpectrumTarget::TrackPostMatrix(track.into());
    SpectrumCaptureRequest {
        maximum_capture_bytes: spectrum_capture_resources_for(&target).retained_bytes,
        target,
        channels: SpectrumChannels::Stereo,
    }
}

fn collection_request(tracks: [&str; 2]) -> SpectrumCaptureCollectionRequest {
    let entries = vec![
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostMatrix(tracks[0].into()),
            channels: SpectrumChannels::Stereo,
        },
        SpectrumCaptureCollectionEntry {
            target: SpectrumTarget::TrackPostInputBuiltins(tracks[1].into()),
            channels: SpectrumChannels::Stereo,
        },
    ];
    let resources = spectrum_capture_collection_resources(&entries).expect("collection resources");
    SpectrumCaptureCollectionRequest {
        entries,
        maximum_capture_bytes: resources.retained_bytes,
    }
}

/// The browser's selected meters: one `PostMatrix` sample-peak meter per strip.
fn meter_requests(session: &Session) -> Vec<HostMeterRequest> {
    session
        .compiled
        .normalized_model()
        .tracks
        .iter()
        .map(|track| HostMeterRequest {
            strip_id: track.id.as_str().into(),
            tap: MeterTap::PostMatrix,
            metrics: MeterMetricSet::SAMPLE_PEAK,
        })
        .collect()
}

fn live_controls(branch: Branch) -> HostLiveControlRequest {
    match branch {
        Branch::Meters => HostLiveControlRequest {
            meter_period_frames: NonZeroU32::new(256),
            meter_queue_depth: NonZeroUsize::new(4).expect("depth"),
            ..HostLiveControlRequest::default()
        },
        Branch::Single | Branch::Collection => HostLiveControlRequest::default(),
    }
}

/// What a preparation hands back besides the plan, kept alive for the whole run.
#[allow(dead_code)] // The handles stay alive with the plan; only the captures are read.
enum Kept {
    None(HostLiveControlHandles),
    Single(HostLiveControlHandles, SpectrumCapture),
    Collection(HostLiveControlHandles, SpectrumCaptureCollection),
}

/// `session` prepared through `branch`'s boot entry, or as the successor of `base` when given.
/// `track` is the capture target of the spectrum branches.
fn prepare(
    branch: Branch,
    session: &Session,
    track: &str,
    base: Option<SuccessorBase<'_>>,
) -> Result<(PreparedHost, Kept), PrepareDiagnostics> {
    let compiled = &session.compiled;
    let request = live_controls(branch);
    let superseded = LiveValuesSuperseded::default();
    let lanes = HostLiveLanes::ALL;
    Ok(match (branch, base) {
        (Branch::Single, None) => {
            let (host, handles, capture) = prepare_host_runtime_with_live_controls_and_spectrum(
                compiled,
                &caps(),
                &request,
                &single_request(track),
                lanes,
                &superseded,
            )?;
            (host, Kept::Single(handles, capture))
        }
        (Branch::Single, Some(base)) => {
            let (host, handles, capture) =
                prepare_host_runtime_with_live_controls_and_spectrum_successor(
                    compiled,
                    &caps(),
                    &request,
                    &single_request(track),
                    lanes,
                    &superseded,
                    base,
                )?;
            (host, Kept::Single(handles, capture))
        }
        (Branch::Collection, None) => {
            let (host, handles, capture) =
                prepare_host_runtime_with_live_controls_and_spectrum_collection(
                    compiled,
                    &caps(),
                    &request,
                    &collection_request([track, "eq1"]),
                    lanes,
                    &superseded,
                )?;
            (host, Kept::Collection(handles, capture))
        }
        (Branch::Collection, Some(base)) => {
            let (host, handles, capture) =
                prepare_host_runtime_with_live_controls_and_spectrum_collection_successor(
                    compiled,
                    &caps(),
                    &request,
                    &collection_request([track, "eq1"]),
                    lanes,
                    &superseded,
                    base,
                )?;
            (host, Kept::Collection(handles, capture))
        }
        (Branch::Meters, None) => {
            let (host, handles) = prepare_host_runtime_with_selected_meters(
                compiled,
                &caps(),
                &request,
                &meter_requests(session),
                lanes,
                &superseded,
            )?;
            (host, Kept::None(handles))
        }
        (Branch::Meters, Some(base)) => {
            let (host, handles) = prepare_host_runtime_with_selected_meters_successor(
                compiled,
                &caps(),
                &request,
                &meter_requests(session),
                lanes,
                &superseded,
                base,
            )?;
            (host, Kept::None(handles))
        }
    })
}

fn base<'a>(
    inventory: &'a PlanStateInventory,
    committed: &'a SessionModel,
    edit_ramps: &'a EditRamps,
) -> SuccessorBase<'a> {
    SuccessorBase {
        inventory,
        committed,
        revision: SUCCESSOR_REVISION,
        edit_ramps,
    }
}

/// Gate 1's run for `branch`: A through the boot entry, B as its successor (or fresh).
fn run(
    branch: Branch,
    a: &Session,
    b: &Session,
    mode: Successor,
) -> (successor::SwappedRun, Kept, Kept) {
    let (predecessor, a_kept) = prepare(branch, a, "eq0", None)
        .unwrap_or_else(|failure| panic!("predecessor: {failure:?}"));
    let feed = Feed::new(SOURCE, 1);
    let mut b_kept = None;
    let edit_ramps = EditRamps::new();
    let swapped = swapped_run_with(
        predecessor,
        |inventory| {
            let base = (mode == Successor::Carry).then(|| base(inventory, &a.model, &edit_ramps));
            let (host, kept) = prepare(branch, b, "eq0", base)
                .unwrap_or_else(|failure| panic!("successor: {failure:?}"));
            b_kept = Some(kept);
            host
        },
        &[&feed],
        &[&feed],
        mode,
    );
    (swapped, a_kept, b_kept.expect("the successor prepared"))
}

fn reference(branch: Branch, b: &Session) -> Vec<successor::Block> {
    let (host, _kept) =
        prepare(branch, b, "eq0", None).unwrap_or_else(|failure| panic!("reference: {failure:?}"));
    reference_run_of(host, &[&Feed::new(SOURCE, 1)])
}

/// Gate 1. Red if a successor entry ignores its base (every ring restarts and the stem jumps), or
/// prepares a different graph than its boot entry.
#[test]
fn every_branch_swaps_in_without_a_gap() {
    let a = Session::compile(session_a());
    let b = Session::compile(session_b());
    for branch in BRANCHES {
        let reference = reference(branch, &b);
        let (swapped, _a_kept, _b_kept) = run(branch, &a, &b, Successor::Carry);
        assert_eq!(swapped.adopted, 1, "{branch:?}: the producer moved");
        assert_eq!(
            (
                swapped.blocks[SWAP_BLOCK].swap,
                swapped.blocks[SWAP_BLOCK].carry
            ),
            (SwapOutcome::Applied, CarryOutcome::Carried),
            "{branch:?}: the swap block"
        );
        assert_eq!(
            first_difference(&swapped.blocks, &reference),
            None,
            "{branch:?}: every block equals the post-edit session rendered from frame 0"
        );
        assert!(
            reference[SWAP_BLOCK].bits.iter().any(|bits| *bits != 0),
            "{branch:?}: the reference plays at the swap block, or the comparison is vacuous"
        );

        // The oracle can fail: a successor prepared with no base restarts its rings.
        let (fresh, _a_kept, _b_kept) = run(branch, &a, &b, Successor::Fresh);
        assert_eq!(
            first_difference(&fresh.blocks, &reference),
            Some(SWAP_BLOCK),
            "{branch:?}: a fresh successor differs at the swap block"
        );
    }
}

/// Gate 2. Red if the successor's capture is not wired to its plan's graph taps.
#[test]
fn the_successors_capture_completes_a_window() {
    let a = Session::compile(session_a());
    let b = Session::compile(session_b());
    let (mut swapped, _a_kept, b_kept) = run(Branch::Single, &a, &b, Successor::Carry);
    let Kept::Single(_handles, mut capture) = b_kept else {
        panic!("a single capture");
    };
    capture.arm().expect("arm the successor's capture");
    let rate = swapped.successor.sample_rate_hz;
    let plane = vec![0.25_f32; QUANTUM];
    let window_blocks = SPECTRUM_WINDOW_FRAMES / QUANTUM;
    // One block of lead-in, then the window: the first captured block starts the window.
    for block in BLOCKS..BLOCKS + window_blocks + 2 {
        swapped
            .successor_sources
            .submit(
                SOURCE.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: (block * QUANTUM) as u64,
                    sample_rate_hz: rate,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .unwrap_or_else(|error| panic!("block {block}: {error:?}"));
        render(&mut swapped.owner);
    }
    let window = capture.try_read().expect("a completed window");
    assert!(window.first_sample >= (BLOCKS * QUANTUM) as u64);
    assert_eq!(window.channels, SpectrumChannels::Stereo);
    assert!(window.left.iter().any(|value| *value != 0.0));
    assert!(window.right.iter().any(|value| *value != 0.0));
}

fn feed(frames: usize) -> [Vec<f32>; 2] {
    let mut draw = Draw::new(0x1401);
    let mut plane = || -> Vec<f32> { (0..frames).map(|_| draw.noise(0.5)).collect() };
    [plane(), plane()]
}

/// `BLOCKS_GATE_3` blocks from sample 0, feeding `planes`, as output bits.
fn render_bits(host: &mut PreparedHost, planes: &[Vec<f32>; 2]) -> Vec<u32> {
    let quantum = host.report.quantum_frames as usize;
    let rate = host.report.sample_rate_hz;
    let mut bits = Vec::new();
    for block in 0..BLOCKS_GATE_3 {
        let range = block * quantum..(block + 1) * quantum;
        host.sources
            .submit(
                SOURCE.as_bytes(),
                SourceSubmission {
                    generation: 1,
                    start_frame: range.start as u64,
                    sample_rate_hz: rate,
                    planes: &[&planes[0][range.clone()], &planes[1][range.clone()]],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = vec![0.0_f32; quantum * 2];
        let output = PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("planes");
        host.plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: range.start as u64,
                },
            )
            .expect("render");
        bits.extend(samples.iter().map(|sample| sample.to_bits()));
    }
    bits
}

const BLOCKS_GATE_3: usize = 64;

/// Gate 3. Red if the selected-meter path stays `BetweenRenderCalls` (a Worker would then push
/// records while the worklet renders under a contract that forbids it), or if dropping the fusion
/// moves a bit.
#[test]
fn the_concurrent_selected_meter_entry_forms_no_fused_bank_and_renders_the_same_bits() {
    let compiled = compile_host_session(FIXTURE, &caps()).expect("compiled fixture");
    let strips: Vec<HostMeterRequest> = compiled
        .normalized_model()
        .tracks
        .iter()
        .map(|track| HostMeterRequest {
            strip_id: track.id.as_str().into(),
            tap: MeterTap::PostMatrix,
            metrics: MeterMetricSet::SAMPLE_PEAK,
        })
        .collect();
    let request = live_controls(Branch::Meters);
    let superseded = LiveValuesSuperseded::default();
    let planes = feed(QUANTUM * BLOCKS_GATE_3);

    test_only_reset_fader_matrix_witness();
    let (mut serialized, _handles) =
        prepare_host_runtime_with_selected_meters_between_render_calls(
            &compiled,
            &caps(),
            &request,
            &strips,
            HostLiveLanes::ALL,
            &superseded,
        )
        .unwrap_or_else(|failure| panic!("serialized: {failure:?}"));
    assert!(
        test_only_fader_matrix_witness().factory_calls > 0,
        "the serialized entry fuses banks, so the witness can see a fusion"
    );

    test_only_reset_fader_matrix_witness();
    let (mut concurrent, _handles) = prepare_host_runtime_with_selected_meters(
        &compiled,
        &caps(),
        &request,
        &strips,
        HostLiveLanes::ALL,
        &superseded,
    )
    .unwrap_or_else(|failure| panic!("concurrent: {failure:?}"));
    assert_eq!(
        test_only_fader_matrix_witness().factory_calls,
        0,
        "Concurrent delivery forms no fused fader-matrix bank"
    );

    let serialized_bits = render_bits(&mut serialized, &planes);
    let concurrent_bits = render_bits(&mut concurrent, &planes);
    assert!(serialized_bits.iter().any(|bits| *bits != 0));
    assert!(
        serialized_bits == concurrent_bits,
        "dropping the fusion moved a bit"
    );
}

/// Gate 4. Red if the refusal runs after the producers moved.
#[test]
fn a_successor_without_its_spectrum_target_is_refused_and_the_predecessor_keeps_playing() {
    let a = Session::compile(session_a());
    let b = Session::compile(session_without_eq1());
    for branch in [Branch::Single, Branch::Collection] {
        // The capture targets track `eq1`, which B removed.
        let (mut predecessor, _kept) = prepare(branch, &a, "eq1", None)
            .unwrap_or_else(|failure| panic!("{branch:?} predecessor: {failure:?}"));
        let rate = predecessor.report.sample_rate_hz;
        let feed = Feed::new(SOURCE, 1);
        feed.submit(&mut predecessor.sources, rate, 0);
        feed.submit(&mut predecessor.sources, rate, 1);
        let edit_ramps = EditRamps::new();
        let refused = prepare(
            branch,
            &b,
            "eq1",
            Some(base(&predecessor.inventory, &a.model, &edit_ramps)),
        )
        .err()
        .unwrap_or_else(|| panic!("{branch:?}: the missing target was accepted"));
        assert!(
            String::from_utf8_lossy(refused.as_bytes()).contains("host.spectrum.target"),
            "{branch:?}: {}",
            String::from_utf8_lossy(refused.as_bytes())
        );
        // The producer never moved: the predecessor still takes PCM and plays it.
        feed.submit(&mut predecessor.sources, rate, 2);
        let mut output = [0.0_f32; QUANTUM * 2];
        predecessor
            .plan
            .render(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM)
                        .expect("planes"),
                },
                RenderTime { absolute_sample: 0 },
            )
            .expect("the predecessor renders");
        assert!(output.iter().any(|sample| *sample != 0.0));
    }
}

/// Gate 5. Red if the swap block of a spectrum plan allocates or frees on render.
#[test]
fn the_spectrum_swap_block_allocates_and_frees_nothing() {
    use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
    /// Count a render-scope allocation instead of aborting, so a violation reads as this test's
    /// assertion; restored on exit.
    struct RestoreMode(Mode);
    impl Drop for RestoreMode {
        fn drop(&mut self) {
            set_mode(self.0);
        }
    }
    assert_installed();
    let _restore = RestoreMode(mode());
    set_mode(Mode::Count);
    engine::realtime::audit::warm_up();
    let a = Session::compile(session_a());
    let b = Session::compile(session_b());
    let (swapped, _a_kept, _b_kept) = run(Branch::Single, &a, &b, Successor::Carry);
    assert_eq!(swapped.blocks[SWAP_BLOCK].carry, CarryOutcome::Carried);
    for (index, block) in swapped.blocks.iter().enumerate().skip(1) {
        assert_eq!(block.allocator, (0, 0, 0), "block {index}: allocator calls");
        assert_eq!(block.audit, (0, 0), "block {index}: render audit");
    }
}
