//! Stored fader automation at preparation (#1646): its bytes are charged as builtin bank payload
//! (gate 2), and a fader automation keeps a mono track's collapse (gate 3). Gate 1 (flat renders
//! the static bits) is on the two hosts: `crates/capi/src/runtime/tests.rs` and
//! `hosts/host-web/src/tests.rs`. The render plumbing (#1647a): each automated fader lane takes
//! its `a(n)` and, per block, its node-time mapping and the fader row's jump length, with no
//! render allocation.

use automation::ProgramBuilder;
use builtins_compiler::fader_automation_sizes;
use engine::realtime::{PlanUnitEligibility, PlanarBufferMut, RenderIo, RenderTime};
use host_core::{
    HostPrepareCaps, HostPrepareReport, HostShapePolicy, PreparedHost, SourceSubmission,
    prepare_host_session,
};
use session::{
    Automation, AutomationSegment, AutomationShape, ControlAddress, ControlSmoothing, Effect,
    EffectIdentity, EffectParam, EffectQuality, LinkMode, Owner, ParameterChannel, ParameterUnit,
    Section, SessionModel, SidechainDeclaration, StableId, canonical_session_json,
    parse_session_json,
};

/// Eight tracks on one two-channel source, one banked parametric EQ each as the console slot.
const BANK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-bank-console.json");
const QUANTUM: usize = 128;
const RATE: u32 = 48_000;
const BLOCKS: usize = 16;
/// The fader row (builtin parameter 5).
const FADER_DB: u32 = 5;

fn caps(maximum_graph_session_plus_plan_bytes: u64) -> HostPrepareCaps {
    HostPrepareCaps {
        shape: HostShapePolicy::AnyLaunchRate,
        source_ring_frames: 4_096,
        maximum_automation_spans_per_block: 128,
        maximum_tracks: 100,
        maximum_submixes: 100,
        maximum_vcas: 100,
        maximum_vca_reach_pairs: u64::MAX,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 64,
        maximum_meter_items: 1 << 16,
        maximum_meter_bytes: 1 << 24,
    }
}

fn id(text: &str) -> StableId {
    StableId::parse(text).expect("stable id")
}

fn bank_model() -> SessionModel {
    let mut model = parse_session_json(BANK).expect("bank fixture parses");
    model.automation.clear();
    model
}

/// A flat fader automation at `value` dB on `lane` of track `track`: one `linear` segment from
/// `value` to `value`.
fn flat_fader(
    model: &SessionModel,
    track: usize,
    lane: ParameterChannel,
    value: f32,
) -> Automation {
    Automation {
        id: id(&format!("fader-{track}")),
        target: ControlAddress {
            owner: Owner::Strip(model.tracks[track].id.clone()),
            section: Section::Builtins,
            control: FADER_DB,
            lane,
        },
        segments: vec![AutomationSegment {
            shape: AutomationShape::Linear,
            start_sample: 0,
            end_sample: 48_000,
            start_value: value,
            end_value: value,
            unit: ParameterUnit::Db,
        }],
        offset: session::AutomationOffset::default(),
    }
}

fn prepare(model: &SessionModel, maximum: u64) -> Result<PreparedHost, String> {
    let document = canonical_session_json(model).expect("canonical");
    prepare_host_session(&document, &caps(maximum))
        .map(|(_, prepared)| prepared)
        .map_err(|failure| String::from_utf8_lossy(failure.as_bytes()).into_owned())
}

/// The bytes the host admits against `maximum_graph_session_plus_plan_bytes` for a plan prepared
/// with no live controls and no spectrum: the graph row, the compiled model and the carry program.
fn admitted(report: &HostPrepareReport) -> u64 {
    report.graph_session_plus_plan_bytes
        + report.session_model_bytes
        + report.spectrum_capture_retained_bytes
        + report.route_control_resources.total_bytes
        + report.carry_program_retained_bytes
}

/// The bank width the plan was built at: the widest bank chain's lane count.
fn width(units: &[PlanUnitEligibility]) -> u64 {
    units
        .iter()
        .filter(|unit| unit.banked)
        .map(|unit| unit.lane_tracks.len() as u64)
        .max()
        .expect("the session banks")
}

/// Gate 2: the same session with and without one automated fader lane. `builtin_bank_bytes`,
/// `graph_incremental_plan_bytes` and `graph_session_plus_plan_bytes` each differ by exactly the
/// lane's charge: the builder's byte report for its program and the program's shared allocation,
/// one automation slot per lane of its fader bank, and its offsets cell (D4). A cap one byte below
/// the admitted total refuses the session with `host.graph.resource.limit`, and the cap at the
/// total prepares it.
///
/// Test value: red if any term of the charge (the program, the bank slots or the offsets cell)
/// escapes the graph rows, the caller's cap or (through `graph_session_plus_plan_bytes`) the C
/// ABI's replacement peak, or if a session without automation pays for it.
#[test]
fn a_fader_automation_is_charged_to_the_graph_plan_rows_and_the_cap() {
    let plain = bank_model();
    let mut automated = plain.clone();
    let entry = flat_fader(&automated, 2, ParameterChannel::Left, -7.25);
    let segments = controls::stored_segments(&entry);
    automated.automation = vec![entry];

    let base = prepare(&plain, u64::MAX).expect("the plain session prepares");
    let with = prepare(&automated, u64::MAX).expect("the automated session prepares");
    let sizes = fader_automation_sizes();
    let lanes = width(&with.plan.unit_eligibility());
    let program = ProgramBuilder::bytes(&[controls::FADER_CELL_RULE.describe(&segments)]);
    assert!(program > 0, "the program holds the segment");
    let charge = program
        + sizes.program_overhead_bytes
        + lanes * sizes.lane_slot_bytes
        + sizes.offsets_cell_bytes;
    for (row, before, after) in [
        (
            "builtin_bank_bytes",
            base.report.builtin_bank_bytes,
            with.report.builtin_bank_bytes,
        ),
        (
            "graph_incremental_plan_bytes",
            base.report.graph_incremental_plan_bytes,
            with.report.graph_incremental_plan_bytes,
        ),
        (
            "graph_session_plus_plan_bytes",
            base.report.graph_session_plus_plan_bytes,
            with.report.graph_session_plus_plan_bytes,
        ),
    ] {
        assert_eq!(
            after - before,
            charge,
            "{row}: {before} -> {after}, the charge is {charge} at {lanes} lanes"
        );
    }

    let total = admitted(&with.report);
    assert!(
        admitted(&base.report) < total - 1,
        "the plain session fits the cap the automated one exceeds"
    );
    drop((base, with));
    prepare(&automated, total).expect("the cap at the admitted total prepares");
    let Err(refusal) = prepare(&automated, total - 1) else {
        panic!("one byte below the admitted total refuses");
    };
    assert!(
        refusal.contains("host.graph.resource.limit"),
        "refused with the graph cap's code: {refusal}"
    );
}

/// A deterministic broadband signal: every sample differs, so a copied plane cannot hide.
fn signal(index: usize) -> f32 {
    let mut state = (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15) ^ 0x1234_5678;
    state ^= state >> 33;
    state = state.wrapping_mul(0xff51_afd7_ed55_8ccd);
    state ^= state >> 29;
    (((state >> 40) as f32) / 16_777_216.0) - 0.5
}

/// `[left, right]` output bits over [`BLOCKS`] blocks, the source's two planes carrying the same
/// PCM, and the plan's `[blocks rendered collapsed, chains armed with a prefix]`.
fn render(model: &SessionModel) -> ([Vec<u32>; 2], [u64; 2]) {
    let mut prepared = prepare(model, 100_000_000).expect("prepares");
    let mut planes = [Vec::new(), Vec::new()];
    for block in 0..BLOCKS {
        let base = block * QUANTUM;
        let plane: Vec<f32> = (0..QUANTUM).map(|frame| signal(base + frame)).collect();
        prepared
            .sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: base as u64,
                    sample_rate_hz: RATE,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        prepared
            .plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: base as u64,
                },
            )
            .expect("render");
        planes[0].extend(samples[..QUANTUM].iter().map(|value| value.to_bits()));
        planes[1].extend(samples[QUANTUM..].iter().map(|value| value.to_bits()));
    }
    (planes, prepared.plan.bank_collapse_counters())
}

/// Gate 3 (D3): a mono track (source channels `0, 0`) with a flat `left`-only fader automation at
/// a value its right lane does not hold keeps its collapse: the plan renders as many collapsed
/// blocks as the same session without the automation, and its output equals, bit for bit, the
/// dual render of a twin whose track reads channels `0, 1`, which carry the same PCM.
///
/// Test value: red if fader automation declines the collapse (a plane per track for no audible
/// change), or if the collapsed render of the automated lane differs from its dual render.
#[test]
fn a_left_only_fader_automation_keeps_a_mono_tracks_collapse() {
    let mut mono = bank_model();
    for track in &mut mono.tracks {
        track.right_source_channel = track.left_source_channel;
    }
    mono.tracks[0].fader.right_db = -2.0;
    let mut automated = mono.clone();
    automated.automation = vec![flat_fader(&automated, 0, ParameterChannel::Left, -9.5)];
    let mut twin = automated.clone();
    twin.tracks[0].right_source_channel = 1;
    assert_ne!(
        twin.tracks[0].left_source_channel, twin.tracks[0].right_source_channel,
        "the twin reads two channels"
    );

    let (plain, plain_counters) = render(&mono);
    let (collapsed, counters) = render(&automated);
    let (dual, _) = render(&twin);
    assert!(counters[0] > 0, "the mono session collapses: {counters:?}");
    assert_ne!(plain[0], collapsed[0], "the automated lane is heard");
    assert_eq!(
        counters, plain_counters,
        "the fader automation changes no collapse"
    );
    assert!(
        dual[0] != dual[1],
        "the automated left lane differs from the right, so a copied plane cannot hide"
    );
    for plane in 0..2 {
        let first = collapsed[plane]
            .iter()
            .zip(&dual[plane])
            .position(|(a, b)| a != b);
        assert_eq!(
            first, None,
            "plane {plane}: the collapsed render equals the twin's dual render"
        );
    }
}

/// `model` with a true-peak limiter insert of 5 ms lookahead on track `track`: a latent node
/// before the fader.
fn with_limiter(mut model: SessionModel, track: usize) -> SessionModel {
    let param = |parameter_id, unit, value| EffectParam {
        parameter_id,
        channel: ParameterChannel::Both,
        unit,
        value,
    };
    let strip = &mut model.tracks[track];
    strip.inserts.effects.push(Effect {
        id: id(&format!("limiter-{track}")),
        identity: EffectIdentity::Native {
            effect_id: id("miso.true-peak-limiter"),
        },
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::Maximum,
        params: vec![
            param(1, ParameterUnit::Db, -8.0),
            param(2, ParameterUnit::Milliseconds, 60.0),
            param(3, ParameterUnit::Milliseconds, 5.0),
        ],
        sidechain: SidechainDeclaration::None,
    });
    model
}

/// A moving fader automation on the left lane of track `track`: `-40` to `0` dB over a second.
fn fader_ride(model: &SessionModel, track: usize) -> Automation {
    let mut entry = flat_fader(model, track, ParameterChannel::Left, -40.0);
    entry.segments[0].end_value = 0.0;
    entry
}

/// The floored input arrival `a(n)` of track `track`'s fader node in `prepared`'s plan: its row in
/// the plan inventory's input floors (#1285), or 0.
fn fader_arrival(prepared: &PreparedHost, model: &SessionModel, track: usize) -> u64 {
    let track_id = model.tracks[track].id.as_str();
    prepared
        .inventory
        .input_floors()
        .iter()
        .find(|(node, _)| {
            matches!(
                node,
                graph::GraphNodeId::TrackStage {
                    track_id: id,
                    stage: graph::TrackStage::PostFader,
                } if id.as_str() == track_id
            )
        })
        .map_or(0, |(_, arrival)| *arrival)
}

/// #1647a (R17-2, R17-3): two automated fader lanes, the first behind a latent insert. At every
/// block each lane's bank holds its `a(n)` (the plan's floored arrival), the node-time mapping
/// `timeline_block(r + ΣP - a(n))` (`-a(n)` at render sample 0 of a fresh plan, then the render
/// clock behind by `a(n)`, with no step) and the fader row's jump length from the session's
/// `fader_ms`; rendering those blocks after a warm-up allocates and frees nothing.
///
/// Test value: red if the graph compiler does not hand `a(n)` to the bank (or hands the natural
/// arrival or another member's), if the graph does not set a block's context before the chain
/// runs (a stale or missing span), if the signed read clamps a negative node sample to 0, if the
/// jump length is read from the mute or pan key, or if the context-set path allocates.
#[test]
fn each_automated_fader_lane_takes_its_arrival_and_node_time_per_block() {
    const PROBED: usize = 12;
    let mut model = with_limiter(bank_model(), 0);
    model.control_smoothing = Some(ControlSmoothing {
        mute_ms: 5.0,
        fader_ms: 20.0,
        pan_ms: 10.0,
    });
    model.automation = vec![fader_ride(&model, 0), fader_ride(&model, 1)];
    let mut prepared = prepare(&model, 100_000_000).expect("prepares");
    let latent = fader_arrival(&prepared, &model, 0);
    let plain = fader_arrival(&prepared, &model, 1);
    assert!(latent > 0, "the limiter delays track 0's fader");
    assert_eq!(plain, 0, "track 1's fader has no latency before it");
    let fader_jump = controls::LiveRamps::for_session(&model).fader_samples;
    assert_eq!(fader_jump, 960, "20 ms at 48 kHz");

    let _ = builtins_compiler::test_only_take_fader_lane_times();
    let mut allocations = (0, 0, 0);
    for block in 0..PROBED {
        let base = block * QUANTUM;
        let plane: Vec<f32> = (0..QUANTUM).map(|frame| signal(base + frame)).collect();
        prepared
            .sources
            .submit(
                b"fixture-source",
                SourceSubmission {
                    generation: 1,
                    start_frame: base as u64,
                    sample_rate_hz: RATE,
                    planes: &[&plane, &plane],
                    frames: QUANTUM as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = [0.0_f32; QUANTUM * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, QUANTUM, QUANTUM).expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        prepared
            .plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: base as u64,
                },
            )
            .expect("render");
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        if block >= 2 {
            allocations.0 += delta.allocations;
            allocations.1 += delta.reallocations;
            allocations.2 += delta.deallocations;
        }
        let (times, overflow) = builtins_compiler::test_only_take_fader_lane_times();
        assert!(!overflow, "block {block}: the probe holds every lane");
        assert_eq!(
            times.len(),
            2,
            "block {block}: one entry per automated lane"
        );
        let mut arrivals: Vec<u64> = times.iter().map(|time| time.arrival).collect();
        arrivals.sort_unstable();
        assert_eq!(arrivals, [plain, latent], "block {block}: each lane's a(n)");
        for time in &times {
            let start = base as i64 - time.arrival as i64;
            assert_eq!(
                time.span,
                Some(graph::TimelineSpan { start, step: None }),
                "block {block}, lane {}: node time r + ΣP - a(n)",
                time.lane
            );
            assert_eq!(time.fader_jump, Some(fader_jump), "block {block}");
        }
    }
    assert_eq!(
        allocations,
        (0, 0, 0),
        "the context-set path allocates and frees nothing"
    );
}
