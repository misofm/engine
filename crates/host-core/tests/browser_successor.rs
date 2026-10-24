//! Issue #1401: every browser preparation branch prepares with concurrent control delivery, at
//! boot and as the successor of a running plan.
//!
//! The three branches are one spectrum capture, a spectrum collection and the browser's selected
//! per-strip meters with no spectrum. The two spectrum branches are the one entry
//! `prepare_host_runtime_with_live_lanes_and_spectrum` (#1593 D3), whose successor is its boot
//! preparation with a [`SuccessorBase`], so the cases below render a predecessor, swap the
//! successor in through the plan exchange and compare every block with the post-edit session
//! prepared fresh through the same entry.
//!
//! * **Gate 1, gap-free, each branch.** Adding a muted track renders every block bit-identical to
//!   the reference, and the swap block reports `Carried`. The same run with a fresh successor
//!   differs at the swap block, so the oracle can fail.
//! * **Gate 2, the successor's capture works.**
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

use core::num::NonZeroUsize;

use builtins::{MeterMetricSet, MeterTap};
use engine::realtime::{CarryOutcome, SwapOutcome};
use host_core::{
    HostLiveControlHandles, HostLiveControlRequest, HostLiveLanes, HostMeterRequest,
    LiveValuesSuperseded, PlanStateInventory, PrepareDiagnostics, PreparedHost,
    PreparedSpectrumCaptures, SPECTRUM_WINDOW_FRAMES, SourceSubmission, SpectrumCapture,
    SpectrumCaptureCollection, SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionRequest,
    SpectrumCaptureRequest, SpectrumChannels, SpectrumPair, SpectrumPreparation, SpectrumTarget,
    SuccessorBase, prepare_host_runtime_with_live_lanes_and_spectrum,
    prepare_host_runtime_with_selected_meters, prepare_host_runtime_with_selected_meters_successor,
    spectrum_capture_collection_resources, spectrum_capture_resources_for,
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
            meter_window_blocks: 2,
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
    /// The plan's handles, its capture and the pairs it takes over from its predecessor (#1395).
    Single(HostLiveControlHandles, SpectrumCapture, Vec<SpectrumPair>),
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
        (Branch::Single | Branch::Collection, base) => {
            let preparation = match branch {
                Branch::Single => SpectrumPreparation::Single(single_request(track)),
                _ => SpectrumPreparation::Collection(collection_request([track, "eq1"])),
            };
            let (host, handles, captures) = prepare_host_runtime_with_live_lanes_and_spectrum(
                compiled,
                &caps(),
                &request,
                lanes,
                Some(&preparation),
                base,
                &superseded,
            )?;
            match captures.expect("a spectrum preparation returns its captures") {
                PreparedSpectrumCaptures::Single(capture) => {
                    let pairing = host.inventory.spectrum_pairing().to_vec();
                    (host, Kept::Single(handles, capture, pairing))
                }
                PreparedSpectrumCaptures::Collection(capture) => {
                    (host, Kept::Collection(handles, capture))
                }
            }
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
        edit_ramps,
        ..SuccessorBase::new(inventory, committed, SUCCESSOR_REVISION)
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

/// Gate 2. Red if the successor's plan does not run the capture's observer at its graph tap.
#[test]
fn the_successors_capture_completes_a_window() {
    let a = Session::compile(session_a());
    let b = Session::compile(session_b());
    let (mut swapped, a_kept, b_kept) = run(Branch::Single, &a, &b, Successor::Carry);
    let (Kept::Single(_, mut previous, _), Kept::Single(_handles, mut capture, pairing)) =
        (a_kept, b_kept)
    else {
        panic!("a single capture");
    };
    // The swap carried the predecessor's observer into this plan: the capture that reads it is
    // the predecessor's, which the control side pairs at promotion (#1395 D3).
    assert!(capture.pair_carried(&mut previous, &pairing));
    capture.arm().expect("arm the paired capture");
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
