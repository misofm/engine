//! Stored fader automation at preparation (#1646): its bytes are charged as builtin bank payload
//! (gate 2), and a fader automation keeps a mono track's collapse (gate 3). Gate 1 (flat renders
//! the static bits) is on the two hosts: `crates/capi/src/runtime/tests.rs` and
//! `hosts/host-web/src/tests.rs`. The render plumbing (#1647a): each automated fader lane takes
//! its `a(n)` and, per block, its node-time mapping and the fader row's jump length, with no
//! render allocation. The events on the fader (#1647b): completion samples, jumps, latency, the
//! session seek, quantum invariance, allocation and the scalar oracle's parity (#1647 gates 1-6,
//! R17-4). The held-ramp obligation (#1647c, gate 8).
//! Preparation bakes every fader in the canonical floating-point environment, whatever the
//! caller's thread has set (D2).

#[cfg(feature = "test-support")]
#[path = "support/successor.rs"]
mod successor;

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

/// [`prepare`], and the heap bytes the preparation left live on this thread (the audited
/// allocator's thread-scoped counters: requested minus released), which the prepared host holds:
/// the compiled model is dropped inside the window.
fn prepare_retained(model: &SessionModel) -> (PreparedHost, u64) {
    let document = canonical_session_json(model).expect("canonical");
    let mark = bench_support::alloc::current_thread_counters();
    let prepared = prepare_host_session(&document, &caps(u64::MAX))
        .map(|(_, prepared)| prepared)
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let moved = bench_support::alloc::current_thread_delta_since(mark);
    (prepared, moved.requested_bytes - moved.released_bytes)
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
/// one automation slot per lane of its fader bank, and its offsets cell (D4). The charge is also
/// checked against the heap itself: the bytes the automated prepared host holds live, less the
/// plain one's, are exactly the charge, measured with the audited allocator's thread-scoped
/// counters (the compiled model is dropped inside the window). A cap one byte below the admitted total
/// refuses the session with `host.graph.resource.limit`, and the cap at the total prepares it.
///
/// Test value: red if any term of the charge (the program, the bank slots or the offsets cell)
/// escapes the graph rows, the caller's cap or (through `graph_session_plus_plan_bytes`) the C
/// ABI's replacement peak, if the charged sizes disagree with what lowering actually retains, or
/// if a session without automation pays for it.
#[test]
fn a_fader_automation_is_charged_to_the_graph_plan_rows_and_the_cap() {
    let plain = bank_model();
    let mut automated = plain.clone();
    let entry = flat_fader(&automated, 2, ParameterChannel::Left, -7.25);
    let segments = controls::stored_segments(&entry);
    automated.automation = vec![entry];

    bench_support::alloc::assert_installed();
    // Warm the process statics first, so neither measured preparation pays for them.
    drop(prepare_retained(&plain));
    let (base, base_retained) = prepare_retained(&plain);
    let (with, with_retained) = prepare_retained(&automated);
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

    // The control catalog charges its own bytes: the automated lane's `fader_db` row carries
    // `automated` (#1650 D4), a descriptor of its own, which the catalog's resources count. The
    // catalog exists only with `control-provider`.
    #[cfg(feature = "control-provider")]
    let catalog = with.control_catalog.resources().retained_bytes
        - base.control_catalog.resources().retained_bytes;
    #[cfg(not(feature = "control-provider"))]
    let catalog = 0;
    assert_eq!(
        with_retained - base_retained - catalog,
        charge,
        "the heap the automation retains is its charge: {base_retained} -> {with_retained}, \
         the catalog's own {catalog}"
    );

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
    render_prepared(prepare(model, 100_000_000).expect("prepares"))
}

/// [`render`] of an already prepared host.
fn render_prepared(mut prepared: PreparedHost) -> ([Vec<u32>; 2], [u64; 2]) {
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

// #1647b: the events on the fader.

const NINE: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");
const NINE_SOURCE: &[u8] = b"fixture-source";

/// The first `tracks` tracks of the nine-track fixture at render quantum `quantum`: no console,
/// no insert, no input filter, each routed to the output, on the fixture's one source made long
/// enough for every test. On a DC source of `1.0` a track's output is its fader gain.
fn plain(tracks: usize, quantum: u32) -> SessionModel {
    let mut model = parse_session_json(NINE).expect("fixture parses");
    model.console.pre_insert.clear();
    model.console.post_insert.clear();
    model.tracks.truncate(tracks);
    let kept: Vec<StableId> = model.tracks.iter().map(|track| track.id.clone()).collect();
    model.routes.retain(|route| match &route.source {
        session::RouteSource::Track { track_id, .. } => kept.contains(track_id),
        _ => false,
    });
    for track in &mut model.tracks {
        track.console.clear();
        track.inserts.effects.clear();
        for channel in [&mut track.builtins.left, &mut track.builtins.right] {
            channel.hpf_hz = 0.0;
            channel.lpf_hz = 0.0;
        }
        track.matrix_or_pan = session::MatrixOrPan::Matrix {
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
            smoothing_samples: 0,
        };
    }
    model.sources[0].frames = 1 << 20;
    model.quantum_frames = quantum;
    model
}

/// `control_smoothing` explicit with fader length `fader_ms` (mute and pan 5 ms).
fn smoothing(fader_ms: f32) -> Option<ControlSmoothing> {
    Some(ControlSmoothing {
        mute_ms: 5.0,
        fader_ms,
        pan_ms: 5.0,
    })
}

/// A fader entry on `lane` of track `track` with `segments`.
fn fader_entry(
    model: &SessionModel,
    track: usize,
    lane: ParameterChannel,
    segments: Vec<AutomationSegment>,
) -> Automation {
    let mut entry = flat_fader(model, track, lane, 0.0);
    entry.id = id(&format!("ride-{track}"));
    entry.segments = segments;
    entry
}

/// A segment of `shape` from `start` to `end` and `start_value` to `end_value` dB.
fn segment(
    shape: AutomationShape,
    start: u64,
    end: u64,
    start_value: f32,
    end_value: f32,
) -> AutomationSegment {
    AutomationSegment {
        shape,
        start_sample: start,
        end_sample: end,
        start_value,
        end_value,
        unit: ParameterUnit::Db,
    }
}

/// The linear dB fade of gate 1: `-40` to `0` dB over 48,000 samples.
fn fade() -> Vec<AutomationSegment> {
    vec![segment(AutomationShape::Linear, 0, 48_000, -40.0, 0.0)]
}

/// Steps at each `(sample, dB)` of `steps`, from `0` dB.
fn steps(steps: &[(u64, f32)]) -> Vec<AutomationSegment> {
    let mut from = (0_u64, 0.0_f32);
    steps
        .iter()
        .map(|&(at, value)| {
            let step = segment(AutomationShape::Step, from.0, at, from.1, value);
            from = (at, value);
            step
        })
        .collect()
}

/// The curve of `entry` at node time `t`, through the one evaluator render uses.
fn curve(entry: &Automation, t: i64) -> f32 {
    automation::value_at(&controls::stored_segments(entry), t)
}

/// The fader gain of `db` with no VCA (`S = +0.0`, #1646 D1).
fn gain(db: f32) -> f32 {
    builtins::checked_fader_gain(session::vca_compose_db(db, 0.0)).expect("fader domain")
}

/// The caps a [`Rig`] prepares under: a source ring of 32 quanta.
fn rig_caps(model: &SessionModel) -> HostPrepareCaps {
    HostPrepareCaps {
        source_ring_frames: 32 * model.quantum_frames,
        ..caps(1_000_000_000)
    }
}

/// A prepared plan, rendered block by block on a DC source of `1.0`.
struct Rig {
    prepared: PreparedHost,
    handles: Option<host_core::HostLiveControlHandles>,
    gate: Option<engine::realtime::PlanRevisionGate>,
    quantum: usize,
    /// The source generation and the source frame the next block submits.
    generation: u64,
    frame: u64,
    rendered: u64,
    out: [Vec<f32>; 2],
    /// The render calls' allocations, reallocations and frees since the last take.
    allocations: (u64, u64, u64),
}

impl Rig {
    fn from(prepared: PreparedHost, model: &SessionModel) -> Self {
        Self {
            prepared,
            handles: None,
            gate: None,
            quantum: model.quantum_frames as usize,
            generation: 1,
            frame: 0,
            rendered: 0,
            out: [Vec::new(), Vec::new()],
            allocations: (0, 0, 0),
        }
    }

    fn new(model: &SessionModel) -> Self {
        let document = canonical_session_json(model).expect("canonical");
        let prepared = prepare_host_session(&document, &rig_caps(model))
            .map(|(_, prepared)| prepared)
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
        Self::from(prepared, model)
    }

    /// With live controls on every strip and the plan's revision gate, for a jump-length write.
    fn live(model: &SessionModel) -> Self {
        let document = canonical_session_json(model).expect("canonical");
        let compiled = host_core::compile_host_session(&document, &rig_caps(model))
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
        let (mut prepared, handles) = host_core::prepare_host_runtime_with_live_lanes(
            &compiled,
            &rig_caps(model),
            &host_core::HostLiveControlRequest::default(),
            host_core::HostLiveLanes::ALL,
            &host_core::LiveValuesSuperseded::default(),
        )
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
        let gate = prepared.plan.take_revision_gate();
        let mut rig = Self::from(prepared, model);
        rig.handles = Some(handles);
        rig.gate = gate;
        rig
    }

    /// Write `fader` samples as the plan's fader jump length (mute and pan unchanged), applied
    /// at the next block entry (#1639).
    fn write_fader_jump(&mut self, model: &SessionModel, fader: u32) {
        use builtins_compiler::TestOnlyWriteGate;
        let mut lengths = host_core::session_jump_lengths(model);
        lengths.fader_samples = fader;
        let producer = self
            .handles
            .as_mut()
            .and_then(|handles| handles.jump_lengths.as_mut())
            .expect("the plan's jump-length cell");
        self.gate
            .as_mut()
            .expect("the plan's gate")
            .write_published(|stamp| producer.write(stamp, lengths));
    }

    /// A plain session seek to timeline sample `timeline` (#1641): the source's next PCM is the
    /// new generation's, from that frame. The rig submits the seek's runway at once (#2205), so
    /// the seek lands at the next block, and keeps it queued ahead.
    fn seek(&mut self, timeline: u64) {
        self.generation += 1;
        self.prepared
            .sources
            .seek_session(self.generation, timeline, None)
            .expect("the seek is admitted");
        self.frame = timeline;
        let runway = source::seek_runway_blocks(RATE, self.quantum as u32, 32);
        for _ in 0..runway {
            self.submit_one();
        }
    }

    /// Submit the source's next block of DC `1.0`.
    fn submit_one(&mut self) {
        let quantum = self.quantum;
        let plane = vec![1.0_f32; quantum];
        self.prepared
            .sources
            .submit(
                NINE_SOURCE,
                SourceSubmission {
                    generation: self.generation,
                    start_frame: self.frame,
                    sample_rate_hz: RATE,
                    planes: &[&plane, &plane],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        self.frame += quantum as u64;
    }

    /// Render one block; returns its first render sample.
    fn block(&mut self) -> u64 {
        let quantum = self.quantum;
        self.submit_one();
        let mut samples = vec![0.0_f32; quantum * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("output planes");
        let first = self.rendered;
        let mark = bench_support::alloc::current_thread_counters();
        self.prepared
            .plan
            .render(
                RenderIo { output },
                RenderTime {
                    absolute_sample: first,
                },
            )
            .expect("render");
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        self.allocations.0 += delta.allocations;
        self.allocations.1 += delta.reallocations;
        self.allocations.2 += delta.deallocations;
        self.out[0].extend_from_slice(&samples[..quantum]);
        self.out[1].extend_from_slice(&samples[quantum..]);
        self.rendered += quantum as u64;
        first
    }

    /// Render until at least `frames` samples are out.
    fn until(&mut self, frames: u64) {
        while self.rendered < frames {
            self.block();
        }
    }
}

/// The stage driven directly: a width-one ramped fader from `start` dB, given each
/// `(sample, operation)` of `operations` at its sample (#1638), over `frames` samples of DC `1.0`.
fn reference(start: f32, operations: &[(u64, builtins::FaderOperation)], frames: u64) -> Vec<f32> {
    let mut stage = builtins::FaderMuteRampBuiltins::new(builtins::BuiltinParameters::default())
        .expect("default parameters");
    stage
        .apply_fader_operation(
            builtins::BuiltinLaneSelector::Both,
            builtins::FaderOperation::SetGain { gain: gain(start) },
        )
        .expect("prepared gain");
    let mut out = Vec::with_capacity(frames as usize);
    let mut next = operations.iter().peekable();
    let mut at = 0_u64;
    while at < frames {
        while let Some(&&(sample, operation)) = next.peek()
            && sample == at
        {
            stage
                .apply_fader_operation(builtins::BuiltinLaneSelector::Both, operation)
                .expect("operation");
            next.next();
        }
        let end = next
            .peek()
            .map_or(frames, |&&(sample, _)| sample.min(frames));
        let length = (end - at) as usize;
        let mut left = vec![1.0_f32; length];
        let mut right = vec![1.0_f32; length];
        stage.process(builtins::DualMonoBlock::new(&mut left, &mut right, at).expect("block"));
        out.extend_from_slice(&left);
        at = end;
    }
    out
}

/// The grid retargets of a curve with no discontinuity, from node time 0 to `frames`, as #1637
/// emits them on a cell seeded with the curve's first value: at each `64k`, toward the curve at
/// `64k + 63` over 64 samples, when that value changes.
fn grid_operations(entry: &Automation, frames: u64) -> Vec<(u64, builtins::FaderOperation)> {
    let mut held = curve(entry, 0);
    (0..frames)
        .step_by(64)
        .filter_map(|at| {
            let value = curve(entry, at as i64 + 63);
            (value.to_bits() != held.to_bits()).then(|| {
                held = value;
                (
                    at,
                    builtins::FaderOperation::RetargetGain {
                        gain: gain(value),
                        samples: 64,
                    },
                )
            })
        })
        .collect()
}

/// The first sample index at which `a` and `b` differ by bits.
fn first_difference(a: &[f32], b: &[f32]) -> Option<usize> {
    a.iter()
        .zip(b)
        .position(|(a, b)| a.to_bits() != b.to_bits())
}

/// #1647 gate 1 and D1.4 (MAJOR-3 of the U2 review): a linear dB fade from `-40` to `0` dB over
/// 48,000 samples on a DC source of `1.0`, `control_smoothing` explicit 0, on a track in no VCA
/// (`S = +0.0`) and on the member of a VCA at `-6` dB. At each completion sample `64k + 63` the
/// output equals `checked_fader_gain(vca_compose_db(curve(64k + 63), S))` exactly, and every
/// sample equals the stage driven directly with the composed grid retargets (the ramp kernel's
/// interpolation between them).
///
/// Test value: red if a completion is off by one, if the grid follows render samples instead of
/// node time, if an event's gain is not composed and converted as the static fader's, if the
/// first grid event is lost to the seed, or if render converts an event's value without `S` (the
/// member's ride would ignore its VCA once the curve moves).
#[test]
fn a_fader_fade_reaches_the_curve_at_each_completion_sample() {
    const FRAMES: u64 = 48_000 + 1_024;
    for vca in [None, Some(-6.0_f32)] {
        let mut model = plain(1, 128);
        model.control_smoothing = smoothing(0.0);
        let entry = fader_entry(&model, 0, ParameterChannel::Both, fade());
        model.automation = vec![entry.clone()];
        if let Some(db) = vca {
            model.vcas = vec![session::Vca {
                id: id("vca-ride"),
                fader: session::DualMonoFader {
                    left_db: db,
                    right_db: db,
                    left_mute: false,
                    right_mute: false,
                },
                members: vec![model.tracks[0].id.clone()],
            }];
        }
        let offsets = vca.map_or(0.0, f64::from);
        let mut rig = Rig::new(&model);
        rig.until(FRAMES);
        let mut held = curve(&entry, 0);
        let operations: Vec<_> = (0..FRAMES)
            .step_by(64)
            .filter_map(|at| {
                let value = curve(&entry, at as i64 + 63);
                (value.to_bits() != held.to_bits()).then(|| {
                    held = value;
                    (
                        at,
                        builtins::FaderOperation::RetargetGain {
                            gain: composed(value, offsets),
                            samples: 64,
                        },
                    )
                })
            })
            .collect();
        let expected = reference_from(composed(curve(&entry, 0), offsets), &operations, FRAMES);
        for plane in &rig.out {
            for t in (63..FRAMES).step_by(64) {
                assert_eq!(
                    plane[t as usize].to_bits(),
                    composed(curve(&entry, t as i64), offsets).to_bits(),
                    "VCA {vca:?}: completion sample {t}"
                );
            }
            assert_eq!(
                first_difference(&plane[..FRAMES as usize], &expected),
                None,
                "VCA {vca:?}: every sample is the stage's, composed with S"
            );
        }
    }
}

/// #1647 gate 2: a `step` from `0` to `-6` dB at sample 1,000 (not a grid sample). With
/// `control_smoothing` explicit 0 the output changes exactly at sample 1,000; with the default
/// table the ramp starts at 1,000 and ends at `1,000 + J - 1`, `J` the session's fader length.
///
/// Test value: red if a jump snaps to the grid, ignores `control_smoothing`, starts a sample late
/// or early, or completes off by one.
#[test]
fn a_fader_step_jumps_at_its_exact_sample() {
    const FRAMES: u64 = 4_096;
    for fader_ms in [None, Some(0.0)] {
        let mut model = plain(1, 128);
        model.control_smoothing = fader_ms.and_then(smoothing);
        let ramp = controls::LiveRamps::for_session(&model).fader_samples;
        model.automation = vec![fader_entry(
            &model,
            0,
            ParameterChannel::Both,
            steps(&[(1_000, -6.0)]),
        )];
        let mut rig = Rig::new(&model);
        rig.until(FRAMES);
        let expected = reference(
            0.0,
            &[(
                1_000,
                builtins::FaderOperation::RetargetGain {
                    gain: gain(-6.0),
                    samples: ramp,
                },
            )],
            FRAMES,
        );
        for plane in &rig.out {
            assert_eq!(plane[999].to_bits(), 1.0_f32.to_bits(), "{fader_ms:?}");
            assert_ne!(plane[1_000].to_bits(), 1.0_f32.to_bits(), "{fader_ms:?}");
            let end = 1_000 + ramp.max(1) as usize - 1;
            assert_eq!(plane[end].to_bits(), gain(-6.0).to_bits(), "{fader_ms:?}");
            if ramp > 1 {
                assert_ne!(
                    plane[end - 1].to_bits(),
                    gain(-6.0).to_bits(),
                    "{fader_ms:?}"
                );
            }
            assert_eq!(
                first_difference(&plane[..FRAMES as usize], &expected),
                None,
                "{fader_ms:?}: the stage driven directly"
            );
        }
    }
}

/// #1647 gate 2, the next jump and not the one in flight: steps at 1,000 and 3,000 with a fader
/// length of 480; a write of 960 to the plan's jump-length cell (#1639) at a block entry: at
/// 1,152, while the first jump ramps, and at 2,944, the entry of the block that holds the second
/// step. In each the first completes over 480 and the second ramps over 960, bit for bit the stage
/// driven directly with those two retargets.
///
/// Test value: red if a written length cuts the jump in flight, or if the length is read only at
/// preparation or a block late (the write at 2,944 would reach the second step at 480).
#[test]
fn a_written_jump_length_reaches_the_next_jump_not_the_one_in_flight() {
    const FRAMES: u64 = 4_608;
    let mut model = plain(1, 128);
    model.control_smoothing = smoothing(10.0);
    assert_eq!(controls::LiveRamps::for_session(&model).fader_samples, 480);
    model.automation = vec![fader_entry(
        &model,
        0,
        ParameterChannel::Both,
        steps(&[(1_000, -6.0), (3_000, -12.0)]),
    )];
    let retarget = |db: f32, samples: u32| builtins::FaderOperation::RetargetGain {
        gain: gain(db),
        samples,
    };
    let expected = reference(
        0.0,
        &[(1_000, retarget(-6.0, 480)), (3_000, retarget(-12.0, 960))],
        FRAMES,
    );
    for write_at in [1_152, 2_944] {
        let mut rig = Rig::live(&model);
        while rig.rendered < write_at {
            rig.block();
        }
        rig.write_fader_jump(&model, 960);
        rig.until(FRAMES);
        for plane in &rig.out {
            assert_eq!(
                first_difference(&plane[..FRAMES as usize], &expected),
                None,
                "written at {write_at}"
            );
            assert_eq!(plane[1_479].to_bits(), gain(-6.0).to_bits());
            assert_eq!(plane[3_959].to_bits(), gain(-12.0).to_bits());
            assert_ne!(plane[3_958].to_bits(), gain(-12.0).to_bits());
        }
    }
}

/// #1647 gate 3: two tracks on one source, the first behind a true-peak limiter (latency `λ`)
/// before its fader, each with a fader step at timeline 4,800 (`control_smoothing` explicit 0).
/// The event probe puts each step at render sample `4,800 + a(n)`, so the two are `λ` apart, and
/// in the output both steps meet the source frame 4,800 they act on: the output equals the
/// session with both faders at `0` dB before that frame's output sample and the session with both
/// at `-6` dB from it.
///
/// Test value: red if node time ignores `a(n)`: the latent track's automation would lead its
/// audio by `λ`.
#[test]
fn a_fader_step_meets_its_source_frame_through_a_latent_insert() {
    const FRAMES: u64 = 8_192;
    let mut base = with_limiter(plain(2, 128), 0);
    base.control_smoothing = smoothing(0.0);
    let mut automated = base.clone();
    automated.automation = (0..2)
        .map(|track| {
            fader_entry(
                &automated,
                track,
                ParameterChannel::Both,
                steps(&[(4_800, -6.0)]),
            )
        })
        .collect();
    let mut low = base.clone();
    for track in &mut low.tracks {
        (track.fader.left_db, track.fader.right_db) = (-6.0, -6.0);
    }

    let mut rig = Rig::new(&automated);
    let latent = fader_arrival(&rig.prepared, &automated, 0);
    let plain_arrival = fader_arrival(&rig.prepared, &automated, 1);
    assert!(latent > 0 && plain_arrival == 0, "{latent} {plain_arrival}");
    let _ = builtins_compiler::test_only_take_fader_events();
    rig.until(FRAMES);
    let (events, overflow) = builtins_compiler::test_only_take_fader_events();
    assert!(!overflow);
    let mut samples: Vec<u64> = events.iter().map(|event| event.sample).collect();
    samples.sort_unstable();
    samples.dedup();
    assert_eq!(samples, [4_800, 4_800 + latent], "{events:?}");

    let mut high = Rig::new(&base);
    high.until(FRAMES);
    let mut low = Rig::new(&low);
    low.until(FRAMES);
    let reach = high.out[0]
        .iter()
        .position(|value| *value != 0.0)
        .expect("the source reaches the output") as u64;
    let step = (4_800 + reach) as usize;
    for plane in 0..2 {
        assert_eq!(
            first_difference(&rig.out[plane][..step], &high.out[plane][..step]),
            None,
            "plane {plane}: before the step"
        );
        assert_eq!(
            first_difference(
                &rig.out[plane][step..FRAMES as usize],
                &low.out[plane][step..FRAMES as usize]
            ),
            None,
            "plane {plane}: from the step"
        );
    }
}

/// #1647 gate 4 (host-core): after a session seek to `T` through #1641's producer, the seek
/// reaches the fader at the render sample `r` of the block it applies at (no latency here), and
/// the lane equals `checked_fader_gain(curve(T))` exactly from `r`, with no ramp: the probe shows
/// one `Set` at `r` with the curve's value.
///
/// Test value: red if a seek ramps the lane, lands at the wrong sample, or the cell repositions
/// without a `Set`.
#[test]
fn a_session_seek_sets_the_fader_lane_exactly() {
    const T: u64 = 24_000;
    let mut model = plain(1, 128);
    let entry = fader_entry(&model, 0, ParameterChannel::Both, fade());
    model.automation = vec![entry.clone()];
    let mut rig = Rig::new(&model);
    rig.until(1_280);
    let _ = builtins_compiler::test_only_take_fader_events();
    rig.seek(T);
    let r = rig.block();
    let (events, _) = builtins_compiler::test_only_take_fader_events();
    let sets: Vec<_> = events
        .iter()
        .filter(|event| event.kind == automation::EventKind::Set)
        .map(|event| (event.channel, event.sample, event.value.to_bits()))
        .collect();
    let value = curve(&entry, T as i64).to_bits();
    assert_eq!(sets, [(0, r, value), (1, r, value)], "{events:?}");
    for plane in &rig.out {
        assert_eq!(
            plane[r as usize].to_bits(),
            gain(curve(&entry, T as i64)).to_bits(),
            "exact at r"
        );
    }
}

/// The output of `model` at `quantum`, over `frames`, with an optional session seek to timeline
/// `seek.1` pushed before the block that starts at render sample `seek.0`.
fn render_at(
    model: &SessionModel,
    quantum: u32,
    frames: u64,
    seek: Option<(u64, u64)>,
) -> [Vec<f32>; 2] {
    let mut model = model.clone();
    model.quantum_frames = quantum;
    let mut rig = Rig::new(&model);
    while rig.rendered < frames {
        if let Some((at, timeline)) = seek
            && rig.rendered == at
        {
            rig.seek(timeline);
        }
        rig.block();
    }
    rig.out.map(|mut plane| {
        plane.truncate(frames as usize);
        plane
    })
}

/// #1647 gate 5: the same session at quantum 128 and at quantum 100 gives the same output bits,
/// sample for sample from sample 0, with and without a session seek (at render sample 3,200, a
/// block entry at both quanta). The ride holds a fade, steps and the default jump length.
///
/// Test value: red if any value depends on the quantum (README blocker B1): a grid on render
/// blocks, a jump or a seek landing on a block boundary.
#[test]
fn the_fader_events_do_not_depend_on_the_quantum() {
    const FRAMES: u64 = 9_600;
    let mut model = plain(1, 128);
    let mut segments = vec![segment(AutomationShape::Linear, 0, 2_000, -30.0, -3.0)];
    segments.push(segment(AutomationShape::Step, 2_000, 4_321, -3.0, -18.0));
    segments.push(segment(AutomationShape::Linear, 4_321, 9_000, -18.0, 2.0));
    model.automation = vec![fader_entry(&model, 0, ParameterChannel::Left, segments)];
    for seek in [None, Some((3_200, 7_777))] {
        let a = render_at(&model, 128, FRAMES, seek);
        let b = render_at(&model, 100, FRAMES, seek);
        for plane in 0..2 {
            assert_eq!(
                first_difference(&a[plane], &b[plane]),
                None,
                "{seek:?} plane {plane}"
            );
        }
        assert_ne!(a[0], a[1], "the left lane moves");
    }
}

/// #1647 gate 6: 1,000 blocks with events on eight lanes (eight tracks, each a fade with steps on
/// its left lane) allocate and free nothing after a warm-up. Track `t`'s step ends at
/// `90,000 + 37 t`, so its jump's sample names the track: every one of the eight tracks gives its
/// jump, whatever the bank width (eight lanes in one bank on AVX2, two banks of four on NEON and
/// Wasm, where `event.lane` is the lane inside its bank).
///
/// Test value: red if render allocates for events.
#[test]
fn rendering_fader_events_allocates_nothing() {
    let mut model = plain(8, 128);
    model.automation = (0..8)
        .map(|track| {
            let mut segments = vec![segment(
                AutomationShape::Linear,
                0,
                60_000,
                -40.0 + track as f32,
                0.0,
            )];
            let jump = 90_000 + 37 * track as u64;
            segments.push(segment(AutomationShape::Step, 60_000, jump, 0.0, -12.0));
            segments.push(segment(
                AutomationShape::Linear,
                jump,
                128_000,
                -12.0,
                -40.0,
            ));
            fader_entry(&model, track, ParameterChannel::Left, segments)
        })
        .collect();
    let mut rig = Rig::new(&model);
    rig.until(4 * 128);
    let _ = builtins_compiler::test_only_take_fader_events();
    rig.allocations = (0, 0, 0);
    let mut jumps = std::collections::BTreeSet::new();
    for _ in 0..1_000 {
        rig.block();
        let (events, _) = builtins_compiler::test_only_take_fader_events();
        jumps.extend(
            events
                .iter()
                .filter(|event| event.kind == automation::EventKind::Jump)
                .map(|event| event.sample),
        );
    }
    let expected: std::collections::BTreeSet<u64> =
        (0..8).map(|track| 90_000 + 37 * track).collect();
    assert_eq!(jumps, expected, "one jump on each of the eight tracks");
    assert_eq!(rig.allocations, (0, 0, 0), "1,000 blocks of fader events");
}

/// The session of the oracle parity test: a fade on both lanes of track 0, which sits behind a
/// true-peak limiter (a latent insert, so its fader's node time is `a(n)` behind), steps on the
/// left lane and a fade on the right lane of track 1, no automation on track 2; the default jump
/// length. Every ride ends by 6,000 samples, so the later blocks have no fader event.
#[cfg(feature = "test-support")]
fn parity_model() -> SessionModel {
    let mut model = with_limiter(plain(3, 128), 0);
    let mut right = fade_from(-12.0, 3_000);
    right[0].end_value = 0.0;
    model.automation = vec![
        fader_entry(&model, 0, ParameterChannel::Both, fade_from(-40.0, 2_000)),
        fader_entry(
            &model,
            1,
            ParameterChannel::Left,
            steps(&[(700, -6.0), (2_900, -24.0), (5_000, 3.0)]),
        ),
        {
            let mut entry = fader_entry(&model, 1, ParameterChannel::Right, right);
            entry.id = id("ride-1-right");
            entry
        },
    ];
    model
}

/// Render `rig` for `blocks` blocks; with handles, a live mute ramp of 300 samples starts on track
/// 1's left lane at block 10 and an unmute ramp at block 20, so the lane holds its gain.
/// With handles, the offsets cells take two writes too (#1649 D3): track 0's two lanes a new `S`
/// at block 15 (a jump of 480 samples), and track 1's left lane one at block 20, in the block
/// whose drain starts the unmute ramp, so its jump meets a held gain. With `offsets` false the
/// offsets cells take no write and the mute ramps run as they do with it.
#[cfg(feature = "test-support")]
fn render_parity(mut rig: Rig, blocks: usize, offsets: bool) -> [Vec<f32>; 2] {
    use builtins_compiler::TestOnlyWriteGate;
    let mut gate = rig.handles.is_some().then(|| {
        rig.prepared
            .plan
            .take_revision_gate()
            .expect("the plan's gate")
    });
    for block in 0..blocks {
        if let (Some(gate), Some(handles)) = (gate.as_mut(), rig.handles.as_mut())
            && (block == 10 || block == 20)
        {
            gate.write_published(|stamp| {
                handles.strip_controls[1].fader.write(
                    stamp,
                    builtins_compiler::TrackFaderRecord::Mute {
                        lanes: builtins::BuiltinLaneSelector::Left,
                        muted: block == 10,
                        smoothing_samples: 300,
                    },
                );
                if block == 20 && offsets {
                    handles.strip_controls[1].fader.write_offsets(
                        stamp,
                        0,
                        builtins_compiler::FaderOffsetsRecord {
                            offsets_sum: -3.0,
                            smoothing_samples: 200,
                            own_db: None,
                        },
                    );
                }
            });
        }
        if let (Some(gate), Some(handles)) = (gate.as_mut(), rig.handles.as_mut())
            && block == 15
            && offsets
        {
            gate.write_published(|stamp| {
                for channel in 0..2 {
                    handles.strip_controls[0].fader.write_offsets(
                        stamp,
                        channel,
                        builtins_compiler::FaderOffsetsRecord {
                            offsets_sum: -6.0,
                            smoothing_samples: 480,
                            own_db: None,
                        },
                    );
                }
            });
        }
        rig.block();
    }
    rig.out
}

/// R17-4: the test-only scalar oracle renders the stored fader events through the one event
/// driver, so a `Backend::Scalar` plan renders the bank's bits: with no live control (the oracle's
/// automated strip binds the ramped fader) and with live controls (the live fader owner), each
/// with a live mute and unmute ramp on an automated lane.
///
/// The live variant also writes the offsets cells (#1649): a jump on a free lane and one under a
/// held unmute ramp, read by the fader bank and by the oracle's live fader owner alike.
///
/// Test value: red if the oracle's fader owners ignore the program, take no node time or `a(n)`
/// (track 0's fader sits behind a latent insert), defer an automated fader past its events, or
/// convert an event differently from the bank: "a placement change moves no bit" would fail for
/// automated sessions. Red too if the oracle does not read the offsets cells, or jumps (or
/// forgets a held lane's target) otherwise than the bank.
#[cfg(feature = "test-support")]
#[test]
fn the_scalar_oracle_renders_the_banks_fader_events() {
    use graph_compiler::Backend;
    const BLOCKS: usize = 64;
    let model = parity_model();
    let document = canonical_session_json(&model).expect("canonical");
    let caps = rig_caps(&model);
    let compiled = host_core::compile_host_session(&document, &caps)
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let request = host_core::HostLiveControlRequest::default();
    let plain_on = |backend| {
        let prepared = host_core::test_only_prepare_host_runtime_on(&compiled, &caps, backend)
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
        render_parity(Rig::from(prepared, &model), BLOCKS, true)
    };
    let live_with = |backend, offsets: bool| {
        let (prepared, handles) = host_core::test_only_prepare_host_runtime_with_live_controls_on(
            &compiled, &caps, &request, backend,
        )
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
        let mut rig = Rig::from(prepared, &model);
        rig.handles = Some(handles);
        render_parity(rig, BLOCKS, offsets)
    };
    let live_on = |backend| live_with(backend, true);
    let mut unautomated = model.clone();
    unautomated.automation.clear();
    let without = render_parity(Rig::new(&unautomated), BLOCKS, true);
    for (label, bank, oracle) in [
        (
            "no live control",
            plain_on(Backend::Simd4),
            plain_on(Backend::Scalar),
        ),
        (
            "live controls",
            live_on(Backend::Simd4),
            live_on(Backend::Scalar),
        ),
    ] {
        assert_ne!(bank, without, "{label}: the automation is heard");
        if label != "no live control" {
            // U3 fold, MINOR 7: against the same live rig with no offsets write, so only the
            // writes can make the difference.
            assert_ne!(
                bank,
                live_with(Backend::Simd4, false),
                "{label}: the offsets writes are heard"
            );
        }
        for plane in 0..2 {
            assert_eq!(
                first_difference(&bank[plane], &oracle[plane]),
                None,
                "{label}: plane {plane}"
            );
        }
    }
}

/// #1648 D2, "a cell gained" (superseding #1647b's restart `Set` for a lane whose stage
/// carries): a lane a successor adds to stored fader automation on a carried stage is not set
/// exactly; it jumps from the carried gain to its curve over the fader jump length. Track 1 gains a
/// fader step from `0` to `-9` dB at timeline 300 in a successor swapped in at block 6 (render
/// sample 768, no seek), with a 240-sample jump: the swap block differs from the successor's
/// session rendered fresh from sample 0 (whose lane took its own jump at 300 and stands at `-9`
/// dB), and from the block after the adoption jump completes every block equals it bit for bit.
///
/// Test value: red if the gained cell keeps its prepared value (`0` dB, the curve's first value)
/// into the successor, or lands a `Set` instead of the adoption jump.
#[cfg(feature = "test-support")]
#[test]
fn a_lane_a_successor_adds_jumps_to_its_curve() {
    use successor::{Feed, QUANTUM, SWAP_BLOCK, Session, Successor, reference_run, swapped_run};
    let mut base = plain(2, 128);
    // A 240-sample jump: the fresh reference's ramp at 300 completes before the swap block, and
    // the adoption jump inside the swap block's successor.
    base.control_smoothing = smoothing(5.0);
    let mut ride = base.clone();
    ride.automation = vec![fader_entry(
        &ride,
        1,
        ParameterChannel::Both,
        steps(&[(300, -9.0)]),
    )];
    let a = Session::compile(base);
    let b = Session::compile(ride);
    let feed = Feed::new("fixture-source", 7);
    let settled = SWAP_BLOCK + 240_usize.div_ceil(QUANTUM);
    assert!(settled < successor::BLOCKS);
    for backend in [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
    ] {
        let swapped = swapped_run(&a, &b, &[&feed], &[&feed], backend, Successor::Carry);
        assert!(
            swapped.restarted.is_empty(),
            "{backend:?}: the stage carries"
        );
        let reference = reference_run(&b, &[&feed], backend);
        let without = reference_run(&a, &[&feed], backend);
        assert_eq!(
            swapped.blocks[SWAP_BLOCK - 1].bits,
            without[SWAP_BLOCK - 1].bits,
            "{backend:?}: the predecessor"
        );
        assert_ne!(
            swapped.blocks[SWAP_BLOCK].bits, reference[SWAP_BLOCK].bits,
            "{backend:?}: the swap block ramps, it is not set"
        );
        assert_ne!(
            swapped.blocks[SWAP_BLOCK].bits, without[SWAP_BLOCK].bits,
            "{backend:?}: the swap block moves toward the curve"
        );
        for (block, (swapped, reference)) in swapped
            .blocks
            .iter()
            .zip(&reference)
            .enumerate()
            .skip(settled)
        {
            assert_eq!(swapped.bits, reference.bits, "{backend:?}: block {block}");
        }
    }
}

/// #1648 D2, departure 4 (batch 17 U3 fold): on plans with no live cells a VCA edit cannot reach a
/// carried automated lane as an offsets record, so the lane's first block takes the adoption rule
/// (`Adoption::Compare` in the builtins compiler): track 1 rides its fader linearly under VCA
/// `v`, and a successor with `v` at -6 dB (the same ride, so the table continues) is swapped in
/// at block 6 with a 240-sample fader jump. The swap block differs from both sessions rendered
/// fresh (the lane ramps), and from the block after the jump and its first grid ramp every block
/// equals the successor's session rendered fresh.
///
/// Test value: red if the carried lane keeps the `S` its stage heard when no record moves it
/// (it would stay 6 dB high), or sets it at once.
#[cfg(feature = "test-support")]
#[test]
fn a_carried_lane_whose_s_moves_with_no_record_takes_the_adoption_rule() {
    use successor::{Feed, QUANTUM, SWAP_BLOCK, Session, Successor, reference_run, swapped_run};
    let mut base = plain(2, 128);
    base.control_smoothing = smoothing(5.0);
    base.automation = vec![fader_entry(
        &base,
        1,
        ParameterChannel::Both,
        vec![segment(AutomationShape::Linear, 0, 1 << 20, 0.0, -12.0)],
    )];
    base.vcas = vec![session::Vca {
        id: id("v"),
        fader: session::DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![base.tracks[1].id.clone()],
    }];
    let mut moved = base.clone();
    moved.vcas[0].fader.left_db = -6.0;
    moved.vcas[0].fader.right_db = -6.0;
    let a = Session::compile(base);
    let b = Session::compile(moved);
    let feed = Feed::new("fixture-source", 7);
    let settled = SWAP_BLOCK + (240_usize + 64).div_ceil(QUANTUM);
    assert!(settled < successor::BLOCKS);
    for backend in [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
    ] {
        let swapped = swapped_run(&a, &b, &[&feed], &[&feed], backend, Successor::Carry);
        assert!(
            swapped.restarted.is_empty(),
            "{backend:?}: the stage carries"
        );
        let reference = reference_run(&b, &[&feed], backend);
        let without = reference_run(&a, &[&feed], backend);
        assert_eq!(
            swapped.blocks[SWAP_BLOCK - 1].bits,
            without[SWAP_BLOCK - 1].bits,
            "{backend:?}: the predecessor"
        );
        assert_ne!(
            swapped.blocks[SWAP_BLOCK].bits, reference[SWAP_BLOCK].bits,
            "{backend:?}: the swap block ramps, it is not set"
        );
        assert_ne!(
            swapped.blocks[SWAP_BLOCK].bits, without[SWAP_BLOCK].bits,
            "{backend:?}: the swap block moves toward the new S"
        );
        for (block, (swapped, reference)) in swapped
            .blocks
            .iter()
            .zip(&reference)
            .enumerate()
            .skip(settled)
        {
            assert_eq!(swapped.bits, reference.bits, "{backend:?}: block {block}");
        }
    }
}

// #1647c: the held-ramp obligation (D1.6, batch 13b amendment, gate 8).

/// The render sample at which gate 8's unmute ramp ends: `E`, the first sample the stage reports
/// the gain free. A live unmute of `UNMUTE` samples is drained at render sample 0, a block entry at
/// every quantum, and `E` is a block entry at quantum 100 only (the move is then due at a block's
/// first sample).
const UNMUTE: u32 = 1_000;

/// Gate 8's quanta.
const QUANTA: [u32; 4] = [128, 100, 64, 37];

/// One track prepared muted on both lanes, with `segments` on its fader and fader length
/// `fader_ms`.
fn muted_ride(segments: Vec<AutomationSegment>, fader_ms: f32) -> (SessionModel, Automation) {
    let mut model = plain(1, 128);
    model.control_smoothing = smoothing(fader_ms);
    (
        model.tracks[0].fader.left_mute,
        model.tracks[0].fader.right_mute,
    ) = (true, true);
    let entry = fader_entry(&model, 0, ParameterChannel::Both, segments);
    model.automation = vec![entry.clone()];
    (model, entry)
}

/// The output of `model` at `quantum` over `frames`: a live unmute ramp of [`UNMUTE`] samples on
/// track 0 is drained at render sample 0, after an optional session seek to timeline `seek`.
fn unmuted_at(model: &SessionModel, quantum: u32, frames: u64, seek: Option<u64>) -> [Vec<f32>; 2] {
    let mut model = model.clone();
    model.quantum_frames = quantum;
    unmuted(Rig::live(&model), frames, seek)
}

/// [`unmuted_at`] on the test-only scalar oracle (R17-4) at the model's quantum.
#[cfg(feature = "test-support")]
fn unmuted_on_the_oracle(model: &SessionModel, frames: u64, seek: Option<u64>) -> [Vec<f32>; 2] {
    unmuted(oracle_rig(model), frames, seek)
}

/// A rig on the test-only scalar oracle (R17-4) with live controls and its plan's gate.
#[cfg(feature = "test-support")]
fn oracle_rig(model: &SessionModel) -> Rig {
    let document = canonical_session_json(model).expect("canonical");
    let caps = rig_caps(model);
    let compiled = host_core::compile_host_session(&document, &caps)
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let (mut prepared, handles) = host_core::test_only_prepare_host_runtime_with_live_controls_on(
        &compiled,
        &caps,
        &host_core::HostLiveControlRequest::default(),
        graph_compiler::Backend::Scalar,
    )
    .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let gate = prepared.plan.take_revision_gate();
    let mut rig = Rig::from(prepared, model);
    rig.handles = Some(handles);
    rig.gate = gate;
    rig
}

/// Run `rig` (with live controls and its plan's gate) as [`unmuted_at`] describes.
fn unmuted(mut rig: Rig, frames: u64, seek: Option<u64>) -> [Vec<f32>; 2] {
    use builtins_compiler::TestOnlyWriteGate;
    if let Some(timeline) = seek {
        rig.seek(timeline);
    }
    let handles = rig.handles.as_mut().expect("live controls");
    rig.gate
        .as_mut()
        .expect("the plan's gate")
        .write_published(|stamp| {
            handles.strip_controls[0].fader.write(
                stamp,
                builtins_compiler::TrackFaderRecord::Mute {
                    lanes: builtins::BuiltinLaneSelector::Both,
                    muted: false,
                    smoothing_samples: UNMUTE,
                },
            );
        });
    rig.until(frames);
    rig.out.map(|mut plane| {
        plane.truncate(frames as usize);
        plane
    })
}

/// Gate 8's run of `model`: the same bits at every quantum of [`QUANTA`] and on the scalar oracle
/// (returned at the first quantum), and the unmute ramp's end `E` at the gain it aimed for, the
/// prepared value (`entry`'s curve at node time 0: no latency here).
fn gate_eight(
    model: &SessionModel,
    entry: &Automation,
    frames: u64,
    seek: Option<u64>,
) -> [Vec<f32>; 2] {
    let first = unmuted_at(model, QUANTA[0], frames, seek);
    for &quantum in &QUANTA[1..] {
        let other = unmuted_at(model, quantum, frames, seek);
        for plane in 0..2 {
            assert_eq!(
                first_difference(&first[plane], &other[plane]),
                None,
                "quantum {quantum} against {}: plane {plane}",
                QUANTA[0]
            );
        }
    }
    #[cfg(feature = "test-support")]
    {
        let oracle = unmuted_on_the_oracle(model, frames, seek);
        for plane in 0..2 {
            assert_eq!(
                first_difference(&first[plane], &oracle[plane]),
                None,
                "the scalar oracle: plane {plane}"
            );
        }
    }
    for plane in &first {
        assert!(plane[0] < 0.01, "the unmute ramp starts at sample 0");
        assert_eq!(
            plane[UNMUTE as usize - 1].to_bits(),
            gain(curve(entry, 0)).to_bits(),
            "the unmute ramp ends at the pre-unmute gain"
        );
    }
    first
}

/// The stage driven directly from gain `start` (its gain at `E - 1`), given `operations` at
/// offsets from `E`, over `frames` samples of DC `1.0`.
fn reference_from(
    start: f32,
    operations: &[(u64, builtins::FaderOperation)],
    frames: u64,
) -> Vec<f32> {
    let mut stage = builtins::FaderMuteRampBuiltins::new(builtins::BuiltinParameters::default())
        .expect("default parameters");
    stage
        .apply_fader_operation(
            builtins::BuiltinLaneSelector::Both,
            builtins::FaderOperation::SetGain { gain: start },
        )
        .expect("start gain");
    let mut out = Vec::with_capacity(frames as usize);
    let mut next = operations.iter().peekable();
    let mut at = 0_u64;
    while at < frames {
        while let Some(&&(sample, operation)) = next.peek()
            && sample == at
        {
            stage
                .apply_fader_operation(builtins::BuiltinLaneSelector::Both, operation)
                .expect("operation");
            next.next();
        }
        let end = next
            .peek()
            .map_or(frames, |&&(sample, _)| sample.min(frames));
        let mut left = vec![1.0_f32; (end - at) as usize];
        let mut right = left.clone();
        stage.process(builtins::DualMonoBlock::new(&mut left, &mut right, at).expect("block"));
        out.extend_from_slice(&left);
        at = end;
    }
    out
}

/// Assert that `out[E..E + J]` equals the stage driven directly from `out[E - 1]` with
/// `operations` (offsets from `E`), `E` = [`UNMUTE`].
fn assert_move(
    out: &[Vec<f32>; 2],
    jump: u32,
    operations: &[(u64, builtins::FaderOperation)],
    label: &str,
) {
    for plane in out {
        assert_move_at(plane, UNMUTE as usize, jump, operations, label);
    }
}

/// Assert that `plane[e..e + J]` equals the stage driven directly from `plane[e - 1]` with
/// `operations` (offsets from `e`).
fn assert_move_at(
    plane: &[f32],
    e: usize,
    jump: u32,
    operations: &[(u64, builtins::FaderOperation)],
    label: &str,
) {
    let span = u64::from(jump.max(1));
    let expected = reference_from(plane[e - 1], operations, span);
    assert_eq!(
        first_difference(&plane[e..e + span as usize], &expected),
        None,
        "{label}: samples [E, E + J - 1] from E = {e}"
    );
}

/// The move at `E`: one `RetargetGain` to `db` over `jump`.
fn retarget(db: f32, jump: u32) -> (u64, builtins::FaderOperation) {
    (
        0,
        builtins::FaderOperation::RetargetGain {
            gain: gain(db),
            samples: jump,
        },
    )
}

/// Assert that the lane equals `gain(db)` exactly from `E + J - 1` (from `E` with `J = 0`) to
/// the end of `out`, and not before: it settles at the cell's target one fader jump length after
/// `E`, never at the pre-unmute gain.
fn assert_settles(out: &[Vec<f32>; 2], jump: u32, db: f32, label: &str) {
    let settled = UNMUTE as usize + jump.max(1) as usize - 1;
    for plane in out {
        assert!(
            plane[settled..]
                .iter()
                .all(|value| value.to_bits() == gain(db).to_bits()),
            "{label}: the lane equals the composed target from E + J - 1"
        );
        if jump > 1 {
            assert_ne!(plane[settled - 1].to_bits(), gain(db).to_bits(), "{label}");
        }
    }
}

/// #1647 gate 8 (a) and (b): a `Jump` (a step at timeline 400) and a seek `Set` (a session seek
/// to timeline 6,000 drained with the unmute) inside an unmute ramp on a curve flat after it.
/// The stage only remembers each; at `E` the lane moves to the gain composed from the cell's
/// target, so it equals that gain exactly from `E + J - 1` (from `E` with `control_smoothing`
/// explicit 0), the same bits at quantum 128, 100, 64 and 37.
///
/// Test value: red if a remembered `Jump` or seek `Set` is never moved to at `E` (the lane
/// settles at the pre-unmute gain), if the move follows a block entry instead of `E`, or if the
/// move's length is not the fader jump length.
#[test]
fn a_remembered_jump_or_seek_set_moves_the_lane_at_the_unmute_ramps_end() {
    const FRAMES: u64 = 2_048;
    for fader_ms in [10.0, 0.0] {
        let (model, entry) = muted_ride(steps(&[(400, -6.0)]), fader_ms);
        let jump = controls::LiveRamps::for_session(&model).fader_samples;
        let out = gate_eight(&model, &entry, FRAMES, None);
        let label = format!("(a) {fader_ms} ms");
        assert_move(&out, jump, &[retarget(-6.0, jump)], &label);
        assert_settles(&out, jump, -6.0, &label);

        let (model, entry) = muted_ride(steps(&[(5_000, -9.0)]), fader_ms);
        assert_eq!(curve(&entry, 6_000).to_bits(), (-9.0_f32).to_bits());
        let out = gate_eight(&model, &entry, FRAMES, Some(6_000));
        let label = format!("(b) {fader_ms} ms");
        assert_move(&out, jump, &[retarget(-9.0, jump)], &label);
        assert_settles(&out, jump, -9.0, &label);
    }
}

/// #1647 gate 8 (c): a `Jump` at timeline 990 whose own 480-sample ramp ends after `E`, with a
/// block entry between the `Jump` and `E` at quantum 37 only. Samples `[E, E + J - 1]` equal the
/// stage driven directly with one `RetargetGain` at `E` over `J`: no grid event cuts the move's
/// ramp, and the bits are the same at every quantum.
///
/// Test value: red if the move does not take the cell's target or if it is built at a block
/// entry. The curve is flat after the `Jump`, so no grid event can cut the move here; (e) gates
/// that.
#[test]
fn a_jump_ramping_past_the_unmute_ramps_end_is_moved_to_at_its_end() {
    const FRAMES: u64 = 2_048;
    let (model, entry) = muted_ride(steps(&[(990, -6.0)]), 10.0);
    let jump = controls::LiveRamps::for_session(&model).fader_samples;
    assert_eq!(jump, 480);
    let out = gate_eight(&model, &entry, FRAMES, None);
    assert_move(&out, jump, &[retarget(-6.0, jump)], "(c)");
    assert_settles(&out, jump, -6.0, "(c)");
}

/// #1647 gate 8 (d): a curve that goes flat inside the last 64 samples of the unmute ramp (a
/// linear fade from `0` to `-12` dB that ends at timeline 960), with no `Jump` or `Set`: only grid
/// events. The last grid point in the ramp commits `-12` dB, which the stage only remembers; at
/// `E` the lane moves there and settles from `E + J - 1`.
///
/// Test value: red if a remembered grid event is never moved to at `E` (the lane stays at the
/// pre-unmute gain on the flat curve). The curve is flat after `E`, so no grid event can cut the
/// move here; (e) gates that.
#[test]
fn a_curve_flat_inside_the_unmute_ramps_last_grid_point_moves_the_lane() {
    const FRAMES: u64 = 2_048;
    for fader_ms in [10.0, 0.0] {
        let (model, entry) = muted_ride(
            vec![segment(AutomationShape::Linear, 0, 960, 0.0, -12.0)],
            fader_ms,
        );
        let jump = controls::LiveRamps::for_session(&model).fader_samples;
        let out = gate_eight(&model, &entry, FRAMES, None);
        let label = format!("(d) {fader_ms} ms");
        assert_move(&out, jump, &[retarget(-12.0, jump)], &label);
        assert_settles(&out, jump, -12.0, &label);
    }
}

/// The grid operations of the move's ramp at `E` (node time `node`) over `jump` on a curve with no
/// discontinuity after it: the move to `target` dB (R17-9: the curve at `node + jump - 1`), then
/// the one grid point in the ramp's last 64 samples, which #1637 commits as after the cell's own
/// `Jump` (every earlier grid point is held by the move's ramp, batch 13b constraint 3).
fn move_operations(
    entry: &Automation,
    node: i64,
    target: f32,
    jump: u32,
) -> Vec<(u64, builtins::FaderOperation)> {
    let last = (node + i64::from(jump) - 1).div_euclid(64) * 64;
    vec![
        retarget(target, jump),
        (
            (last - node) as u64,
            builtins::FaderOperation::RetargetGain {
                gain: gain(curve(entry, last + 63)),
                samples: 64,
            },
        ),
    ]
}

/// #1647 gate 8 (e): grid events that are not held lie between `E` and the next block entry: a
/// `Jump` whose ramp ends before `E` on a curve that moves after it (the grid point at timeline
/// 1,024 lies before the next entry at quantum 100 and 37 only), and a seek `Set` (a session seek
/// to timeline 2,000 drained with the unmute) on a moving curve. Samples `[E, E + J - 1]` equal the
/// stage driven directly with one `RetargetGain` at `E` over `J`, aimed at the curve at
/// `E + J - 1` (R17-9), and the one grid point #1637 commits in that ramp's last 64 samples
/// (departure 1 of #1647c, amended gate 8 (e)): no other grid event cuts the move's ramp, at any
/// quantum.
///
/// Test value: red if the move does not hold the grid for its own ramp (the grid events after
/// `E` cut it into 64-sample retargets), if it follows a block entry instead of `E`, or if it
/// aims at the cell's target (the curve one grid ramp ahead of the last grid point) instead of
/// the curve at its own completion.
#[test]
fn grid_events_after_the_unmute_ramps_end_do_not_cut_the_move() {
    const FRAMES: u64 = 2_048;
    let jump = 480;
    // A step to -6 dB at timeline 300 (its ramp ends at 779), then a fade to -30 dB.
    let (model, entry) = muted_ride(
        vec![
            segment(AutomationShape::Step, 0, 300, 0.0, -6.0),
            segment(AutomationShape::Linear, 300, 5_000, -6.0, -30.0),
        ],
        10.0,
    );
    let out = gate_eight(&model, &entry, FRAMES, None);
    // R17-9: the move lands at the curve at its completion, `E + J - 1`.
    let target = curve(&entry, i64::from(UNMUTE + jump) - 1);
    assert_ne!(target.to_bits(), curve(&entry, 960 + 63).to_bits());
    assert_move(
        &out,
        jump,
        &move_operations(&entry, i64::from(UNMUTE), target, jump),
        "(e) jump",
    );

    let (model, entry) = muted_ride(
        vec![segment(AutomationShape::Linear, 0, 10_000, -30.0, 0.0)],
        10.0,
    );
    let out = gate_eight(&model, &entry, FRAMES, Some(2_000));
    let node = 2_000 + i64::from(UNMUTE);
    let target = curve(&entry, node + i64::from(jump) - 1);
    assert_move(
        &out,
        jump,
        &move_operations(&entry, node, target, jump),
        "(e) seek",
    );
}

// The U2 fold of #1647 (batch 17): the gates the U2 review found missing, and rulings R17-8,
// R17-9 and R17-10.

/// Write a live mute record on track 0's `lanes` (window `window`), applied at the next block
/// entry.
fn live_mute(rig: &mut Rig, lanes: builtins::BuiltinLaneSelector, muted: bool, window: u32) {
    use builtins_compiler::TestOnlyWriteGate;
    let handles = rig.handles.as_mut().expect("live controls");
    rig.gate
        .as_mut()
        .expect("the plan's gate")
        .write_published(|stamp| {
            handles.strip_controls[0].fader.write(
                stamp,
                builtins_compiler::TrackFaderRecord::Mute {
                    lanes,
                    muted,
                    smoothing_samples: window,
                },
            );
        });
}

/// Write a live offsets record `S = offsets_sum` (ramp `window`) on both channels of track 0,
/// applied at the next block entry.
fn live_offsets(rig: &mut Rig, offsets_sum: f64, window: u32) {
    use builtins_compiler::TestOnlyWriteGate;
    let handles = rig.handles.as_mut().expect("live controls");
    rig.gate
        .as_mut()
        .expect("the plan's gate")
        .write_published(|stamp| {
            for channel in 0..2 {
                handles.strip_controls[0].fader.write_offsets(
                    stamp,
                    channel,
                    builtins_compiler::FaderOffsetsRecord {
                        offsets_sum,
                        smoothing_samples: window,
                        own_db: None,
                    },
                );
            }
        });
}

/// The output of `rig` over `frames`, with `script` called before each block (it reads
/// `rig.rendered` and `rig.quantum`).
fn scripted(mut rig: Rig, frames: u64, script: &dyn Fn(&mut Rig)) -> [Vec<f32>; 2] {
    while rig.rendered < frames {
        script(&mut rig);
        rig.block();
    }
    rig.out.map(|mut plane| {
        plane.truncate(frames as usize);
        plane
    })
}

/// [`scripted`] on a live rig of `model` at each quantum of `quanta` (every time the script acts
/// at is a block entry of each), the same bits at each, and on the scalar oracle at the first
/// (with `host-core/test-support`); returns the first.
fn scripted_across(
    model: &SessionModel,
    quanta: &[u32],
    frames: u64,
    script: &dyn Fn(&mut Rig),
) -> [Vec<f32>; 2] {
    let at = |quantum: u32| {
        let mut model = model.clone();
        model.quantum_frames = quantum;
        model
    };
    let first = scripted(Rig::live(&at(quanta[0])), frames, script);
    for &quantum in &quanta[1..] {
        let other = scripted(Rig::live(&at(quantum)), frames, script);
        for plane in 0..2 {
            assert_eq!(
                first_difference(&first[plane], &other[plane]),
                None,
                "quantum {quantum} against {}: plane {plane}",
                quanta[0]
            );
        }
    }
    #[cfg(feature = "test-support")]
    {
        let oracle = scripted(oracle_rig(&at(quanta[0])), frames, script);
        for plane in 0..2 {
            assert_eq!(
                first_difference(&first[plane], &oracle[plane]),
                None,
                "the scalar oracle: plane {plane}"
            );
        }
    }
    first
}

/// The floored arrival `a(n)` of track 0's fader in `model`'s plan.
fn arrival_of(model: &SessionModel) -> u64 {
    let rig = Rig::new(model);
    fader_arrival(&rig.prepared, model, 0)
}

/// `model` with no automation, unmuted, and track 0's fader static at `db` on both lanes: the
/// plan whose lane holds `gain(db)` exactly.
fn static_at(model: &SessionModel, db: f32) -> SessionModel {
    let mut model = model.clone();
    model.automation.clear();
    let fader = &mut model.tracks[0].fader;
    (fader.left_db, fader.right_db) = (db, db);
    (fader.left_mute, fader.right_mute) = (false, false);
    model
}

/// The `Set` events the probe shows in the block that holds render sample `sample`, on a live
/// rig of `model` (at its quantum) with `script` called before each block: `(channel, render
/// sample, value bits)`.
fn sets_in_block_of(
    model: &SessionModel,
    script: &dyn Fn(&mut Rig),
    sample: u64,
) -> Vec<(usize, u64, u32)> {
    let mut rig = Rig::live(model);
    while rig.rendered + rig.quantum as u64 <= sample {
        script(&mut rig);
        rig.block();
    }
    let _ = builtins_compiler::test_only_take_fader_events();
    script(&mut rig);
    rig.block();
    let (events, overflow) = builtins_compiler::test_only_take_fader_events();
    assert!(!overflow, "the probe holds every event");
    events
        .iter()
        .filter(|event| event.kind == automation::EventKind::Set)
        .map(|event| (event.channel, event.sample, event.value.to_bits()))
        .collect()
}

/// #1647 batch 13b constraint 7: the move at `E` replaces the gain operation the cell gives at
/// `E`. A step exactly at `E` (to `-15` dB at timeline 1,000) after a fade the unmute ramp
/// remembers, and a grid point exactly at `E` (an unmute of 1,024 on a fade): in each the lane
/// takes the move alone, the same bits at every quantum and on the oracle.
///
/// Test value: red if the cell's event at `E` is pushed beside the move (the stage's sink refuses
/// the second operation at one offset, so the move is lost; a debug build fails the refusal's
/// `debug_assert`), or if the move is not the landing at `E`.
#[test]
fn the_move_at_the_unmute_ramps_end_replaces_the_cells_event_there() {
    const FRAMES: u64 = 2_048;
    let jump = 480;
    let (model, entry) = muted_ride(
        vec![
            segment(AutomationShape::Linear, 0, 900, 0.0, -6.0),
            segment(AutomationShape::Step, 900, 1_000, -6.0, -15.0),
        ],
        10.0,
    );
    let out = gate_eight(&model, &entry, FRAMES, None);
    assert_move(&out, jump, &[retarget(-15.0, jump)], "a step at E");
    assert_settles(&out, jump, -15.0, "a step at E");

    const E: u32 = 1_024;
    let (model, entry) = muted_ride(fade_from(-30.0, 10_000), 10.0);
    let out = scripted_across(&model, &QUANTA, FRAMES, &|rig| {
        if rig.rendered == 0 {
            live_mute(rig, builtins::BuiltinLaneSelector::Both, false, E);
        }
    });
    let target = curve(&entry, i64::from(E + jump) - 1);
    for plane in &out {
        assert_move_at(
            plane,
            E as usize,
            jump,
            &move_operations(&entry, i64::from(E), target, jump),
            "a grid point at E",
        );
    }
}

/// A linear dB fade from `from` to `0` dB over `[0, end]`.
fn fade_from(from: f32, end: u64) -> Vec<AutomationSegment> {
    vec![segment(AutomationShape::Linear, 0, end, from, 0.0)]
}

/// R17-10 (MAJOR-5 of the U2 review): a live write that frees a held lane discharges the
/// obligation and releases the cell's hold in the same block. The track is prepared muted on a
/// fade from `-30` to `0` dB; a live unmute of 8,000 samples is drained at 0, an offsets write
/// `S = -6` dB (ramp 200) at 3,200 under that ramp (the held branch of the offsets jump: the cell
/// forgets its target and holds to 7,999; the fade's grid events after it leave a move owed). Then
/// the production trigger (#1651 U1 fold 1, MINOR-5): the shared commit's `Mute` records of a hand
/// mute and an exact hand unmute, `Mute { true, 0 }` at the block entry before 6,400 (the lane is
/// held through that block) and `Mute { false, 0 }` at 6,400, which frees the lane at that entry.
/// From 6,400 the lane follows the composed curve: at each completion sample `64k + 63` it equals
/// `checked_fader_gain(vca_compose_db(curve, S))` exactly, at quantum 128, 100 and 64 and on the
/// oracle.
///
/// Test value: red if the cell keeps its hold of the ended ramp (the lane stays at the gain the
/// exact unmute left while the curve moves, until the hold's last grid point), or if the release
/// is made only when nothing is owed.
#[test]
fn a_live_write_that_frees_a_held_lane_releases_its_hold() {
    const FRAMES: u64 = 8_000;
    let (model, entry) = muted_ride(fade_from(-30.0, 48_000), 10.0);
    let script = |rig: &mut Rig| match rig.rendered {
        0 => live_mute(rig, builtins::BuiltinLaneSelector::Both, false, 8_000),
        3_200 => live_offsets(rig, -6.0, 200),
        6_400 => live_mute(rig, builtins::BuiltinLaneSelector::Both, false, 0),
        at if at + rig.quantum as u64 == 6_400 => {
            live_mute(rig, builtins::BuiltinLaneSelector::Both, true, 0);
        }
        _ => {}
    };
    for quantum in [128, 100, 64] {
        let mut at = model.clone();
        at.quantum_frames = quantum;
        #[cfg_attr(not(feature = "test-support"), allow(unused_mut))]
        let mut outs = vec![("bank", scripted(Rig::live(&at), FRAMES, &script))];
        #[cfg(feature = "test-support")]
        outs.push(("oracle", scripted(oracle_rig(&at), FRAMES, &script)));
        for (form, out) in &outs {
            for plane in out {
                for t in (6_400 + 63..FRAMES).step_by(64) {
                    assert_eq!(
                        plane[t as usize].to_bits(),
                        composed(curve(&entry, t as i64), -6.0).to_bits(),
                        "quantum {quantum}, {form}: completion sample {t}"
                    );
                }
            }
        }
    }
}

/// The render forms [`acted`] compares: the live bank at eight lanes (on an AVX2 target only, the
/// one that has them) and four, and the scalar oracle (R17-4).
#[cfg(feature = "test-support")]
const FORMS: &[graph_compiler::Backend] = &[
    #[cfg(target_feature = "avx2")]
    graph_compiler::Backend::Simd8,
    graph_compiler::Backend::Simd4,
    graph_compiler::Backend::Scalar,
];

/// A rig of `model` at `quantum` with live controls and its plan's gate, on one of the
/// [`FORMS`].
#[cfg(feature = "test-support")]
fn form_rig(model: &SessionModel, quantum: u32, backend: graph_compiler::Backend) -> Rig {
    let mut model = model.clone();
    model.quantum_frames = quantum;
    let document = canonical_session_json(&model).expect("canonical");
    let caps = rig_caps(&model);
    let compiled = host_core::compile_host_session(&document, &caps)
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let request = host_core::HostLiveControlRequest::default();
    let (mut prepared, handles) = host_core::test_only_prepare_host_runtime_with_live_controls_on(
        &compiled, &caps, &request, backend,
    )
    .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
    let gate = prepared.plan.take_revision_gate();
    let mut rig = Rig::from(prepared, &model);
    rig.handles = Some(handles);
    rig.gate = gate;
    rig
}

/// One scripted act on track 0: a live mute record `(lanes, muted, window)`, an offsets record
/// `(S, ramp)` on both channels, and a fader jump length, each optional, written together at the
/// first block entry at or after `at`.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy)]
struct Act {
    at: u64,
    mute: Option<(builtins::BuiltinLaneSelector, bool, u32)>,
    fader: Option<(builtins::BuiltinLaneSelector, f32, u32)>,
    offsets: Option<(f64, u32)>,
    jump: Option<u32>,
}

/// An [`Act`] at `at` that writes nothing until its fields are set.
#[cfg(feature = "test-support")]
const fn act(at: u64) -> Act {
    Act {
        at,
        mute: None,
        fader: None,
        offsets: None,
        jump: None,
    }
}

/// The output of `model` at `quantum` over `frames` with `acts`, the same bits on each of the
/// [`FORMS`] the target provides (three on AVX2); and the block entry each act landed on.
#[cfg(feature = "test-support")]
fn acted(
    model: &SessionModel,
    quantum: u32,
    frames: u64,
    acts: &[Act],
) -> ([Vec<f32>; 2], Vec<u64>) {
    use builtins_compiler::TestOnlyWriteGate;
    let mut outs = Vec::new();
    let mut entries = Vec::new();
    for &form in FORMS {
        let mut rig = form_rig(model, quantum, form);
        entries.clear();
        while rig.rendered < frames {
            let q = rig.quantum as u64;
            for act in acts {
                if rig.rendered < act.at || rig.rendered >= act.at + q {
                    continue;
                }
                entries.push(rig.rendered);
                if let Some(fader) = act.jump {
                    rig.write_fader_jump(model, fader);
                }
                let handles = rig.handles.as_mut().expect("live controls");
                rig.gate
                    .as_mut()
                    .expect("the plan's gate")
                    .write_published(|stamp| {
                        if let Some((lanes, muted, window)) = act.mute {
                            handles.strip_controls[0].fader.write(
                                stamp,
                                builtins_compiler::TrackFaderRecord::Mute {
                                    lanes,
                                    muted,
                                    smoothing_samples: window,
                                },
                            );
                        }
                        if let Some((lanes, db, window)) = act.fader {
                            handles.strip_controls[0].fader.write(
                                stamp,
                                builtins_compiler::TrackFaderRecord::FaderDb {
                                    lanes,
                                    db,
                                    smoothing_samples: window,
                                },
                            );
                        }
                        if let Some((offsets_sum, smoothing_samples)) = act.offsets {
                            for channel in 0..2 {
                                handles.strip_controls[0].fader.write_offsets(
                                    stamp,
                                    channel,
                                    builtins_compiler::FaderOffsetsRecord {
                                        offsets_sum,
                                        smoothing_samples,
                                        own_db: None,
                                    },
                                );
                            }
                        }
                    });
            }
            rig.block();
        }
        outs.push(rig.out.map(|mut plane| {
            plane.truncate(frames as usize);
            plane
        }));
    }
    for (form, other) in outs.iter().enumerate().skip(1) {
        for plane in 0..2 {
            assert_eq!(
                first_difference(&outs[0][plane], &other[plane]),
                None,
                "quantum {quantum}: form {form} against the eight-lane bank, plane {plane}"
            );
        }
    }
    (outs.swap_remove(0), entries)
}

/// #1711 G4 (A1): an exact live unmute that frees a lane held over a moving ride discharges the
/// move the hold owed. Track 0 (prepared unmuted, fader length 20 ms = 960 samples) rides
/// linearly from `0` to `-6` dB over `[0, 1_280)`, holds `-6` dB to 2,320, then rides on to
/// `-12` dB. A live mute (window 256) at 512 and an exact live unmute (window 0) at 1,024 hold
/// the lane over moving grid points (a move is owed, then the unmute frees the lane at that
/// entry); a mute (256) at 1,536 and an unmute (200) at 2,048 hold it again where the ride is
/// flat, so nothing is remembered in that hold, whose end `E` is 2,248. From 1,152 on both lanes
/// equal the rig that takes only the second hold, at quantum 128 on every render form (Simd8 on
/// AVX2, Simd4 and the scalar oracle).
///
/// Test value: red if the free write at the unmute's entry does not discharge the owed move (the
/// `owed = false` of `HeldRampObligation::block`): the stale obligation then lands a move over
/// `J` at `E` with nothing remembered.
#[cfg(feature = "test-support")]
#[test]
fn an_exact_unmute_that_frees_a_held_lane_discharges_the_owed_move() {
    use builtins::BuiltinLaneSelector::Both;
    const FRAMES: u64 = 5_120;
    let (model, _) = muted_ride(
        vec![
            segment(AutomationShape::Linear, 0, 1_280, 0.0, -6.0),
            segment(AutomationShape::Linear, 1_280, 2_320, -6.0, -6.0),
            segment(AutomationShape::Linear, 2_320, 1 << 20, -6.0, -12.0),
        ],
        20.0,
    );
    let model = never_muted(&model);
    let mute = |at: u64, muted: bool, window: u32| Act {
        mute: Some((Both, muted, window)),
        ..act(at)
    };
    let second = [mute(1_536, true, 256), mute(2_048, false, 200)];
    let both: Vec<Act> = [mute(512, true, 256), mute(1_024, false, 0)]
        .into_iter()
        .chain(second)
        .collect();
    let (out, entries) = acted(&model, 128, FRAMES, &both);
    assert_eq!(
        entries,
        [512, 1_024, 1_536, 2_048],
        "each act lands at its own entry"
    );
    let (reference, _) = acted(&model, 128, FRAMES, &second);
    for plane in 0..2 {
        assert_eq!(
            first_difference(&out[plane][1_152..], &reference[plane][1_152..]),
            None,
            "plane {plane}: first difference after 1,152"
        );
    }
}

/// `model` with track 0 prepared unmuted: the never-muted twin.
#[cfg(feature = "test-support")]
fn never_muted(model: &SessionModel) -> SessionModel {
    let mut model = model.clone();
    (
        model.tracks[0].fader.left_mute,
        model.tracks[0].fader.right_mute,
    ) = (false, false);
    model
}

/// R17-10 with nothing owed (MAJOR-A of the U2 fold review, U2 fold 3 F2): a live write that
/// frees a lane the stage held through the end of the previous block releases the cell's hold
/// there, whether or not a move is owed. The track is prepared muted on a curve flat at `-30` dB
/// over `[0, 2_000]` and then fading to `0` dB at 10,000; at quantum 37 a live unmute of 4,000
/// samples is drained at 0, an offsets write `S = -6` dB (ramp 200) at 962 under that ramp (the
/// held branch of the offsets jump: the cell forgets its target and holds to 3,999), and a live
/// fader write of `-10` dB over window 0 at 999 frees the lane (a repeated unmute would change
/// nothing, #1651 batch 13b). No grid point lies between 962 and 999, so the cell
/// remembers no event and owes nothing. From 1,087 the lane follows the composed curve: at each
/// completion sample `64k + 63` it equals `checked_fader_gain(vca_compose_db(curve, S))` exactly,
/// on every render form; both lanes, and a left-only form whose right lane stays muted.
///
/// Test value: red if the release is made only when a move is owed (the cell keeps the offsets
/// jump's hold to 3,999, so the fade's grid events from 2,000 are only remembered on a free
/// stage and the lane stays at the gain the unmute left while the curve moves).
#[cfg(feature = "test-support")]
#[test]
fn a_live_write_that_frees_a_lane_with_nothing_owed_releases_its_hold() {
    use builtins::BuiltinLaneSelector::{Both, Left};
    const FRAMES: u64 = 6_144;
    let (model, entry) = muted_ride(
        vec![
            segment(AutomationShape::Linear, 0, 2_000, -30.0, -30.0),
            segment(AutomationShape::Linear, 2_000, 10_000, -30.0, 0.0),
        ],
        10.0,
    );
    for (lanes, left_only) in [(Both, false), (Left, true)] {
        let (out, entries) = acted(
            &model,
            37,
            FRAMES,
            &[
                Act {
                    mute: Some((lanes, false, 4_000)),
                    ..act(0)
                },
                Act {
                    offsets: Some((-6.0, 200)),
                    ..act(962)
                },
                Act {
                    fader: Some((lanes, -10.0, 0)),
                    ..act(999)
                },
            ],
        );
        assert_eq!(entries, [0, 962, 999], "each act lands at its own entry");
        for t in (1_024 + 63..FRAMES).step_by(64) {
            let at = t as usize;
            assert_eq!(
                out[0][at].to_bits(),
                composed(curve(&entry, t as i64), -6.0).to_bits(),
                "left only {left_only}: left completion sample {t}"
            );
            let right = if left_only { 0.0 } else { out[0][at] };
            assert_eq!(
                out[1][at].to_bits(),
                right.to_bits(),
                "left only {left_only}: right at {t}"
            );
        }
    }
}

/// U2 fold 3, F1: an offsets jump on the free lane at the block entry whose live write frees it
/// keeps its own hold. The track is prepared muted on a curve flat at `-30` dB to 1,100 and then
/// fading to `0` dB at 10,000; at the entry 1,024 a live unmute of window 0 and an offsets write
/// `S = -6` dB with a 480-sample ramp land together (and, in a second form, a live unmute of
/// 4,000 samples is in progress from 0, and a live fader write of `-30` dB frees the lane in place
/// of the repeated unmute). From 1,024 both lanes equal the never-muted session that
/// takes the same offsets write at 1,024, at quantum 128, 64 and 37, on every render form.
///
/// Test value: red if the R17-10 release of the ended unmute overwrites the hold the offsets
/// jump set at the same entry (the fade's grid events from 1,100 cut the 480-sample ramp).
#[cfg(feature = "test-support")]
#[test]
fn an_offsets_jump_at_the_entry_that_frees_the_lane_keeps_its_ramp() {
    use builtins::BuiltinLaneSelector::Both;
    const FRAMES: u64 = 4_096;
    let (model, _) = muted_ride(
        vec![
            segment(AutomationShape::Linear, 0, 1_100, -30.0, -30.0),
            segment(AutomationShape::Linear, 1_100, 10_000, -30.0, 0.0),
        ],
        10.0,
    );
    let twin = never_muted(&model);
    let freeing = Act {
        mute: Some((Both, false, 0)),
        offsets: Some((-6.0, 480)),
        ..act(1_024)
    };
    for quantum in [128, 64, 37] {
        let (reference, _) = acted(
            &twin,
            quantum,
            FRAMES,
            &[Act {
                offsets: Some((-6.0, 480)),
                ..act(1_024)
            }],
        );
        for ramping in [false, true] {
            let mut acts = vec![freeing];
            if ramping {
                // A repeated unmute would leave the 4,000-sample ramp running (#1651 batch 13b),
                // so a live fader write at the curve's `-30` dB frees the lane.
                acts[0] = Act {
                    mute: None,
                    fader: Some((Both, -30.0, 0)),
                    ..freeing
                };
                acts.insert(
                    0,
                    Act {
                        mute: Some((Both, false, 4_000)),
                        ..act(0)
                    },
                );
            }
            let (out, entries) = acted(&model, quantum, FRAMES, &acts);
            let free = *entries.last().expect("the freeing act") as usize;
            for plane in 0..2 {
                assert_eq!(
                    first_difference(&out[plane][free..], &reference[plane][free..])
                        .map(|at| at + free),
                    None,
                    "quantum {quantum}, ramping {ramping}: plane {plane} renders the never-muted \
                     lane from {free}"
                );
            }
        }
    }
}

/// Root decision B17-R11 (U2 fold 3, F3): an unchanged jump inside the hold owes no move. The
/// fader jump length is 16; the curve is flat at `-20` dB to 1,024, linear from `-21` to `-13` dB
/// over `[1_024, 1_144]` (exactly `-20` at 1,039, the jump's completion, so the jump at 1,024
/// changes nothing) and flat at `-13` after. The track is prepared muted and a live unmute of
/// 1,060 samples is drained at 0: the hold ends at `E = 1_060` with no grid point between 1,024
/// and `E`. From `E` both lanes equal the never-muted session at quantum 128, 64 and 37, on every
/// render form.
///
/// Test value: red if a held ramp the live drain starts is reported to the cell as a hold of its
/// own (the removed D1.2): the unchanged jump is then remembered, a move is owed at `E`, and the
/// lane departs from the never-muted session.
#[cfg(feature = "test-support")]
#[test]
fn an_unchanged_jump_inside_the_hold_owes_no_move() {
    use builtins::BuiltinLaneSelector::Both;
    const FRAMES: u64 = 2_048;
    const E: usize = 1_060;
    let (model, entry) = muted_ride(
        vec![
            segment(AutomationShape::Linear, 0, 1_024, -20.0, -20.0),
            segment(AutomationShape::Linear, 1_024, 1_144, -21.0, -13.0),
            segment(AutomationShape::Linear, 1_144, 4_000, -13.0, -13.0),
        ],
        10.0,
    );
    assert_eq!(
        curve(&entry, 1_039).to_bits(),
        (-20.0_f32).to_bits(),
        "the jump at 1,024 completes on the value it starts from"
    );
    let twin = never_muted(&model);
    for quantum in [128, 64, 37] {
        let (out, _) = acted(
            &model,
            quantum,
            FRAMES,
            &[Act {
                mute: Some((Both, false, E as u32)),
                jump: Some(16),
                ..act(0)
            }],
        );
        let (reference, _) = acted(
            &twin,
            quantum,
            FRAMES,
            &[Act {
                jump: Some(16),
                ..act(0)
            }],
        );
        for plane in 0..2 {
            assert_eq!(
                first_difference(&out[plane][E..], &reference[plane][E..]).map(|at| at + E),
                None,
                "quantum {quantum}: plane {plane} renders the never-muted lane from E"
            );
        }
    }
}

/// `model` with a true-peak limiter (5 ms lookahead) on track 0, and its fader's `a(n)`.
fn latent(model: SessionModel) -> (SessionModel, u64) {
    let model = with_limiter(model, 0);
    let arrival = arrival_of(&model);
    assert!(arrival > 0, "the limiter delays the fader");
    (model, arrival)
}

/// R17-8: a seek `Set` that lands exactly at `E` sets the lane there, with no ramp, and
/// discharges the obligation. Behind a latent insert (`a(n)` = λ) the seek at render 3,200 (to
/// timeline 8,000) reaches the fader at `3,200 + λ`, and the unmute of `3,200 + λ` samples drained
/// at 0 ends there, inside a block at quantum 128, 100 and 64. The probe shows one `Set` per
/// channel at `E`, and from `E` the output equals the plan whose fader is static at the curve's
/// value at 8,000 (until the next grid point), the same bits at each quantum and on the oracle.
///
/// Test value: red if the seek's `Set` at `E` is replaced by the move (a ramp over `J` to the
/// same value, settled only at `E + J - 1`).
#[test]
fn a_seek_set_exactly_at_the_unmute_ramps_end_sets_the_lane() {
    const FRAMES: u64 = 4_480;
    const T: u64 = 8_000;
    let (model, entry) = muted_ride(fade_from(-30.0, 48_000), 10.0);
    let (model, arrival) = latent(model);
    let e = 3_200 + arrival;
    let script = |rig: &mut Rig| {
        if rig.rendered == 0 {
            live_mute(rig, builtins::BuiltinLaneSelector::Both, false, e as u32);
        }
        if rig.rendered == 3_200 {
            rig.seek(T);
        }
    };
    let out = scripted_across(&model, &[128, 100, 64], FRAMES, &script);
    let value = curve(&entry, T as i64).to_bits();
    assert_eq!(
        sets_in_block_of(&model, &script, e),
        [(0, e, value), (1, e, value)],
        "a Set per channel at E"
    );
    let reference = scripted(
        Rig::new(&static_at(&model, curve(&entry, T as i64))),
        FRAMES,
        &|rig| {
            if rig.rendered == 3_200 {
                rig.seek(T);
            }
        },
    );
    let e = e as usize;
    for plane in 0..2 {
        assert_eq!(
            first_difference(&out[plane][e..e + 64], &reference[plane][e..e + 64]),
            None,
            "plane {plane}: exact from E"
        );
    }
}

/// #1647 gate 4 behind a latent insert (MAJOR-4 of the U2 review): a session seek to 24,000 at
/// render 3,200 reaches a fader behind a true-peak limiter at `r = 3,200 + a(n)`, inside a node
/// block at quantum 128 and 100. The probe shows one `Set` per channel at `r` with the curve's
/// value; before `r` the output is the unseeked run's, and from `r` to the next grid point it
/// equals the plan whose fader is static at the curve's value at 24,000, the same bits at both
/// quanta.
///
/// Test value: red if a seek inside a node block lands at the block's first sample instead of its
/// step, or ramps.
#[test]
fn a_session_seek_sets_a_latent_fader_lane_exactly_where_it_reaches_the_node() {
    const FRAMES: u64 = 4_480;
    const T: u64 = 24_000;
    let mut model = plain(1, 128);
    let entry = fader_entry(&model, 0, ParameterChannel::Both, fade());
    model.automation = vec![entry.clone()];
    let (model, arrival) = latent(model);
    let r = 3_200 + arrival;
    let seek = |rig: &mut Rig| {
        if rig.rendered == 3_200 {
            rig.seek(T);
        }
    };
    let out = scripted_across(&model, &[128, 100], FRAMES, &seek);
    let value = curve(&entry, T as i64).to_bits();
    assert_eq!(
        sets_in_block_of(&model, &seek, r),
        [(0, r, value), (1, r, value)],
        "a Set per channel at r"
    );
    let unseeked = scripted(Rig::live(&model), FRAMES, &|_| {});
    let reference = scripted(
        Rig::new(&static_at(&model, curve(&entry, T as i64))),
        FRAMES,
        &seek,
    );
    let r = r as usize;
    for plane in 0..2 {
        assert_eq!(
            first_difference(&out[plane][..r], &unseeked[plane][..r]),
            None,
            "plane {plane}: before r"
        );
        assert_eq!(
            first_difference(&out[plane][r..r + 64], &reference[plane][r..r + 64]),
            None,
            "plane {plane}: exact from r"
        );
    }
}

/// #1647 gate 8 with a seek step after `E` in `E`'s block (MAJOR-4 of the U2 review): behind a
/// latent insert, the unmute of 1,000 samples ends at `E` = 1,000, and a session seek (to 8,000)
/// at a block entry `r` reaches the fader at `r + a(n)`, after `E` in the same block (quantum 100:
/// `E` at the block's first sample; quantum 37: inside it). The lane moves at `E`, and from the
/// seek's step it is set exactly: the probe shows its `Set` there, and the output equals the plan
/// whose fader is static at the curve's value at 8,000, on the bank and on the oracle.
///
/// Test value: red if the cut at `E` drops a seek step that lies after it in the block.
#[test]
fn a_seek_step_after_the_unmute_ramps_end_in_its_block_still_lands() {
    const FRAMES: u64 = 2_048;
    const T: u64 = 8_000;
    let (model, entry) = muted_ride(fade_from(-30.0, 48_000), 10.0);
    let (model, arrival) = latent(model);
    let e = u64::from(UNMUTE);
    for quantum in [100_u32, 37] {
        let q = u64::from(quantum);
        let block_end = (e / q + 1) * q;
        let r = (0..=e / q)
            .map(|k| k * q)
            .find(|&r| r + arrival > e && r + arrival < block_end)
            .expect("a block entry whose seek reaches the fader after E in E's block");
        let step = r + arrival;
        let script = move |rig: &mut Rig| {
            if rig.rendered == 0 {
                live_mute(rig, builtins::BuiltinLaneSelector::Both, false, UNMUTE);
            }
            if rig.rendered == r {
                rig.seek(T);
            }
        };
        let out = scripted_across(&model, &[quantum], FRAMES, &script);
        let value = curve(&entry, T as i64).to_bits();
        let mut at_quantum = model.clone();
        at_quantum.quantum_frames = quantum;
        assert_eq!(
            sets_in_block_of(&at_quantum, &script, step),
            [(0, step, value), (1, step, value)],
            "quantum {quantum}: a Set per channel at the step"
        );
        let mut reference_model = static_at(&model, curve(&entry, T as i64));
        reference_model.quantum_frames = quantum;
        let reference = scripted(Rig::new(&reference_model), FRAMES, &|rig| {
            if rig.rendered == r {
                rig.seek(T);
            }
        });
        let step = step as usize;
        for plane in 0..2 {
            assert_ne!(
                out[plane][step - 1].to_bits(),
                reference[plane][step - 1].to_bits(),
                "quantum {quantum}: the move ramps before the step"
            );
            assert_eq!(
                first_difference(
                    &out[plane][step..step + 64],
                    &reference[plane][step..step + 64]
                ),
                None,
                "quantum {quantum}, plane {plane}: exact from the step"
            );
        }
    }
}

/// The fader gain of `db` composed with the VCA offset sum `offsets` (#1646 D1).
fn composed(db: f32, offsets: f64) -> f32 {
    builtins::checked_fader_gain(session::vca_compose_db(db, offsets)).expect("fader domain")
}

/// #1647 batch 13b constraint 4 and departure 3 (MINOR-11 of the U2 review).
///
/// - A ramp that ends muted keeps the obligation: a live mute of 300 samples at 0 on a fade the
///   muted lane remembers, the curve flat at `-12` dB through the later unmute ramp (drained at
///   3,200 over 1,000 samples, so no event falls in it) and moving after `E` = 4,200. The lane
///   lands at `E`.
/// - A hold with nothing remembered ends with no move: prepared muted, a curve flat at `-6` dB up
///   to 1,100 and moving after it, unmuted over 1,000 samples at 0. From `E` the lane renders
///   what the session prepared unmuted renders.
///
/// The same bits at each quantum and on the oracle.
///
/// Test value: red if a muted block entry discharges the obligation (no move at the later
/// unmute's end), or if a hold with nothing remembered moves the lane at its end.
#[test]
fn a_ramp_that_ends_muted_keeps_the_obligation_and_a_hold_with_nothing_remembered_does_not_move() {
    let jump = 480;
    let mut model = plain(1, 128);
    model.control_smoothing = smoothing(10.0);
    let entry = fader_entry(
        &model,
        0,
        ParameterChannel::Both,
        vec![
            segment(AutomationShape::Linear, 0, 1_500, 0.0, -12.0),
            segment(AutomationShape::Linear, 1_500, 4_300, -12.0, -12.0),
            segment(AutomationShape::Linear, 4_300, 8_000, -12.0, -30.0),
        ],
    );
    model.automation = vec![entry.clone()];
    let e = 4_200_u32;
    let out = scripted_across(&model, &[128, 100, 64], 5_120, &|rig| match rig.rendered {
        0 => live_mute(rig, builtins::BuiltinLaneSelector::Both, true, 300),
        3_200 => live_mute(rig, builtins::BuiltinLaneSelector::Both, false, e - 3_200),
        _ => {}
    });
    let target = curve(&entry, i64::from(e + jump) - 1);
    for plane in &out {
        assert_eq!(
            plane[3_199].to_bits(),
            0.0_f32.to_bits(),
            "muted until 3,200"
        );
        assert_move_at(
            plane,
            e as usize,
            jump,
            &move_operations(&entry, i64::from(e), target, jump),
            "ends muted",
        );
    }

    const FRAMES: u64 = 2_048;
    let (model, _) = muted_ride(
        vec![
            segment(AutomationShape::Linear, 0, 1_100, -6.0, -6.0),
            segment(AutomationShape::Linear, 1_100, 6_000, -6.0, -30.0),
        ],
        10.0,
    );
    let out = gate_eight(&model, &model.automation[0], FRAMES, None);
    let mut unmuted_model = model.clone();
    (
        unmuted_model.tracks[0].fader.left_mute,
        unmuted_model.tracks[0].fader.right_mute,
    ) = (false, false);
    let mut free = Rig::new(&unmuted_model);
    free.until(FRAMES);
    let e = UNMUTE as usize;
    for (plane, (out, free)) in out.iter().zip(&free.out).enumerate() {
        assert_eq!(
            first_difference(&out[e..], &free[e..FRAMES as usize]),
            None,
            "nothing remembered: plane {plane} renders the free lane from E"
        );
    }
}

/// #1647 batch 13b, dual mono (MINOR-11 of the U2 review): a track prepared with its left lane
/// muted and both lanes on one fade; a live unmute of 1,000 samples on the left lane only, at 0.
/// The left lane lands at `E`; the right lane, free all along, renders the stage driven directly
/// with the grid's retargets, untouched by the left lane's hold. The same bits at each quantum and
/// on the oracle.
///
/// Test value: red if one channel's hold or obligation is read for the other (the free right lane
/// would be moved at the left lane's `E`).
#[test]
fn a_left_only_unmute_moves_the_left_lane_alone() {
    const FRAMES: u64 = 2_048;
    let jump = 480;
    let mut model = plain(1, 128);
    model.control_smoothing = smoothing(10.0);
    model.tracks[0].fader.left_mute = true;
    let entry = fader_entry(&model, 0, ParameterChannel::Both, fade_from(-30.0, 10_000));
    model.automation = vec![entry.clone()];
    let out = scripted_across(&model, &QUANTA, FRAMES, &|rig| {
        if rig.rendered == 0 {
            live_mute(rig, builtins::BuiltinLaneSelector::Left, false, UNMUTE);
        }
    });
    let target = curve(&entry, i64::from(UNMUTE + jump) - 1);
    assert_move_at(
        &out[0],
        UNMUTE as usize,
        jump,
        &move_operations(&entry, i64::from(UNMUTE), target, jump),
        "left",
    );
    let free = reference(-30.0, &grid_operations(&entry, FRAMES), FRAMES);
    assert_eq!(
        first_difference(&out[1], &free),
        None,
        "the right lane renders the grid alone"
    );
}

/// #1648b (the U2 review's carry finding, required gate): a lane that gains automation on a
/// carried stage moves from the stage's gain, not from its prepared seed. The predecessor holds
/// track 1's fader static at `-20` dB; the successor, swapped in at block 6 with the stage
/// carried, adds a ride flat at `0` dB (its prepared value, the seed). With no seek, the lane
/// jumps from `-20` to `0` dB over the fader jump length, and from the block after that jump
/// every block equals the successor rendered fresh from sample 0.
///
/// Test value: red if adoption compares the curve with the cell's seed instead of the gain the
/// carried stage holds (the lane would keep `-20` dB under a ride at `0` dB, every block).
#[cfg(feature = "test-support")]
#[test]
fn a_ride_gained_on_a_carried_static_fader_jumps_from_the_stages_gain() {
    use successor::{Feed, QUANTUM, SWAP_BLOCK, Session, Successor, reference_run, swapped_run};
    let mut base = plain(2, 128);
    base.control_smoothing = smoothing(5.0);
    (base.tracks[1].fader.left_db, base.tracks[1].fader.right_db) = (-20.0, -20.0);
    let mut ride = base.clone();
    ride.automation = vec![flat_fader(&ride, 1, ParameterChannel::Both, 0.0)];
    let a = Session::compile(base);
    let b = Session::compile(ride);
    let feed = Feed::new("fixture-source", 7);
    let settled = SWAP_BLOCK + 240_usize.div_ceil(QUANTUM) + 1;
    assert!(settled < successor::BLOCKS);
    for backend in [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
    ] {
        let swapped = swapped_run(&a, &b, &[&feed], &[&feed], backend, Successor::Carry);
        assert!(
            swapped.restarted.is_empty(),
            "{backend:?}: the stage carries"
        );
        let reference = reference_run(&b, &[&feed], backend);
        assert_ne!(
            swapped.blocks[SWAP_BLOCK].bits, reference[SWAP_BLOCK].bits,
            "{backend:?}: the swap block ramps from -20 dB"
        );
        for (block, (swapped, reference)) in swapped
            .blocks
            .iter()
            .zip(&reference)
            .enumerate()
            .skip(settled)
        {
            assert_eq!(swapped.bits, reference.bits, "{backend:?}: block {block}");
        }
    }
}

// #1711 G1: a live level move in flight across a table-only rebuild.

/// The block a [`level_move_run`] commits its live move at.
#[cfg(feature = "test-support")]
const MOVE_BLOCK: usize = 4;

/// The length of the ride of [`level_move_model`], from `0` to `-12` dB.
#[cfg(feature = "test-support")]
const RIDE_FRAMES: u64 = 48_000;

/// The live move of [`level_move_model`] at [`MOVE_BLOCK`]: `S` to `-6` dB over 4,000 samples.
#[cfg(feature = "test-support")]
const MOVE: (usize, (f64, u32)) = (MOVE_BLOCK, (-6.0, 4_000));

/// Two tracks (two bank lanes) at quantum 128 with fader length 20 ms (`J` = 960), track 0
/// riding linearly from `0` to `-12` dB over [`RIDE_FRAMES`] (split in two at `split.0` with the
/// second segment ending at `split.1` dB when given: `-12` keeps the ride), with its entry's
/// stored offset `hand` dB and, for a VCA move, a VCA of track 0 at `vca` dB.
#[cfg(feature = "test-support")]
fn level_move_model(split: Option<(u64, f32)>, hand: f32, vca: Option<f32>) -> SessionModel {
    let lin = AutomationShape::Linear;
    let segments = match split {
        None => vec![segment(lin, 0, RIDE_FRAMES, 0.0, -12.0)],
        Some((at, end_db)) => {
            let k = -12.0 * at as f32 / RIDE_FRAMES as f32;
            vec![
                segment(lin, 0, at, 0.0, k),
                segment(lin, at, RIDE_FRAMES, k, end_db),
            ]
        }
    };
    move_model(segments, hand, vca)
}

/// [`level_move_model`]'s session with track 0's ride `segments`.
#[cfg(feature = "test-support")]
fn move_model(segments: Vec<AutomationSegment>, hand: f32, vca: Option<f32>) -> SessionModel {
    let mut model = plain(2, 128);
    model.control_smoothing = smoothing(20.0);
    let mut entry = fader_entry(&model, 0, ParameterChannel::Both, segments);
    (entry.offset.left_db, entry.offset.right_db) = (hand, hand);
    model.automation = vec![entry];
    if let Some(db) = vca {
        model.vcas = vec![session::Vca {
            id: id("vca-move"),
            fader: session::DualMonoFader {
                left_db: db,
                right_db: db,
                left_mute: false,
                right_mute: false,
            },
            members: vec![model.tracks[0].id.clone()],
        }];
    }
    model
}

/// What a [`level_move_run`] rendered.
#[cfg(feature = "test-support")]
struct LevelMoveRun {
    /// Each block's output bits, both planes.
    blocks: Vec<Vec<u32>>,
    /// The render calls' `(allocations, reallocations, frees)`, summed.
    allocations: (u64, u64, u64),
    /// The block whose render adopted the successor, if one did.
    adopted: Option<usize>,
    /// The successor's `restarted_strips()`.
    restarted: Vec<Box<str>>,
}

/// `a` prepared with live controls on `backend` and rendered for `blocks` blocks of DC `1.0`;
/// at each `(block, record)` of `moves`, that block's entry writes the live offsets record
/// `record` (`S` and its ramp) on both channels of track 0, to the plan that renders it. With `b = Some((next, committed, swap))`, `next` is prepared as the successor of
/// the plan on the committed model `committed` and swapped in at block `swap` (the C ABI's
/// carry: `adopt_persisting`, the successor's gate published at the next revision).
#[cfg(feature = "test-support")]
fn level_move_run(
    a: &SessionModel,
    b: Option<(&SessionModel, &SessionModel, usize)>,
    moves: &[(usize, (f64, u32))],
    backend: graph_compiler::Backend,
    blocks: usize,
) -> LevelMoveRun {
    level_move_run_muted(a, b, moves, &[], backend, blocks)
}

/// [`level_move_run`] with live hand mutes: at each `(block, (muted, window))` of `mutes`, that
/// block's entry also writes the live mute record `muted` (ramp `window`) on both lanes of track
/// 0, after the block's live moves, to the plan that renders it.
#[cfg(feature = "test-support")]
fn level_move_run_muted(
    a: &SessionModel,
    b: Option<(&SessionModel, &SessionModel, usize)>,
    moves: &[(usize, (f64, u32))],
    mutes: &[LiveMute],
    backend: graph_compiler::Backend,
    blocks: usize,
) -> LevelMoveRun {
    use builtins_compiler::TestOnlyWriteGate;
    use engine::realtime::{PlanExchangeConfig, SwapOutcome, plan_exchange};
    let quantum = a.quantum_frames as usize;
    let caps = rig_caps(a);
    let request = host_core::HostLiveControlRequest::default();
    let fail = |failure: host_core::PrepareDiagnostics| -> ! {
        panic!("prepare: {}", String::from_utf8_lossy(failure.as_bytes()))
    };
    let compiled = |model: &SessionModel| {
        let document = canonical_session_json(model).expect("canonical");
        host_core::compile_host_session(&document, &caps)
            .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())))
    };
    let (prepared, mut handles) = host_core::test_only_prepare_host_runtime_with_live_controls_on(
        &compiled(a),
        &caps,
        &request,
        backend,
    )
    .unwrap_or_else(|failure| fail(failure));
    let PreparedHost {
        mut plan,
        mut sources,
        inventory,
        ..
    } = prepared;
    let mut gate = plan.take_revision_gate().expect("the plan's gate");
    let (mut publisher, mut owner, _retirer) = plan_exchange(
        plan,
        PlanExchangeConfig {
            retirement_capacity: core::num::NonZeroUsize::MIN,
        },
    )
    .unwrap_or_else(|_| panic!("plan exchange"));
    bench_support::alloc::assert_installed();
    let plane = vec![1.0_f32; quantum];
    let mut run = LevelMoveRun {
        blocks: Vec::with_capacity(blocks),
        allocations: (0, 0, 0),
        adopted: None,
        restarted: Vec::new(),
    };
    let mut successor_sources = None;
    let mut successor_gate = None;
    for block in 0..blocks {
        if let Some((next, committed, swap)) = b
            && block == swap
        {
            let committed = compiled(committed).normalized_model().clone();
            let revision = gate.last_published() + 1;
            let base = host_core::SuccessorBase::new(&inventory, &committed, revision);
            let (mut prepared, next_handles) =
                host_core::test_only_prepare_host_runtime_with_live_controls_successor_on(
                    &compiled(next),
                    &caps,
                    &request,
                    base,
                    backend,
                )
                .unwrap_or_else(|failure| fail(failure));
            prepared.sources.adopt_persisting(&mut sources);
            run.restarted = prepared.restarted_strips().to_vec();
            let PreparedHost {
                mut plan,
                sources: next_sources,
                ..
            } = prepared;
            let mut next_gate = plan.take_revision_gate().expect("the successor's gate");
            next_gate.publish(revision);
            successor_gate = Some((next_handles, next_gate));
            publisher
                .reserve_replacement(plan, engine::realtime::PlanAdoption::Next)
                .unwrap_or_else(|_| panic!("reserve the successor"))
                .commit(revision);
            successor_sources = Some(next_sources);
        }
        for &(_, (offsets_sum, smoothing_samples)) in moves.iter().filter(|(at, _)| *at == block) {
            let (handles, gate) = match successor_gate.as_mut() {
                Some((handles, gate)) => (handles, gate),
                None => (&mut handles, &mut gate),
            };
            gate.write_published(|stamp| {
                for channel in 0..2 {
                    handles.strip_controls[0].fader.write_offsets(
                        stamp,
                        channel,
                        builtins_compiler::FaderOffsetsRecord {
                            offsets_sum,
                            smoothing_samples,
                            own_db: None,
                        },
                    );
                }
            });
        }
        for &(_, (muted, smoothing_samples)) in mutes.iter().filter(|(at, _)| *at == block) {
            let (handles, gate) = match successor_gate.as_mut() {
                Some((handles, gate)) => (handles, gate),
                None => (&mut handles, &mut gate),
            };
            gate.write_published(|stamp| {
                handles.strip_controls[0].fader.write(
                    stamp,
                    builtins_compiler::TrackFaderRecord::Mute {
                        lanes: builtins::BuiltinLaneSelector::Both,
                        muted,
                        smoothing_samples,
                    },
                );
            });
        }
        let at = block * quantum;
        successor_sources
            .as_mut()
            .unwrap_or(&mut sources)
            .submit(
                NINE_SOURCE,
                SourceSubmission {
                    generation: 1,
                    start_frame: at as u64,
                    sample_rate_hz: RATE,
                    planes: &[&plane, &plane],
                    frames: quantum as u32,
                    end_of_region: false,
                },
            )
            .expect("source block");
        let mut samples = vec![0.0_f32; quantum * 2];
        let output =
            PlanarBufferMut::try_new(&mut samples, 2, quantum, quantum).expect("output planes");
        let mark = bench_support::alloc::current_thread_counters();
        let report = owner
            .render_contiguous(RenderIo { output }, at as u64)
            .unwrap_or_else(|error| panic!("render: {error:?}"));
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        run.allocations.0 += delta.allocations;
        run.allocations.1 += delta.reallocations;
        run.allocations.2 += delta.deallocations;
        if report.swap == SwapOutcome::Applied {
            run.adopted = Some(block);
        }
        run.blocks
            .push(samples.iter().map(|sample| sample.to_bits()).collect());
    }
    drop(handles);
    drop(successor_gate);
    run
}

/// #1711 G1 on host-core, at both bank widths. A live level move (`S` to `-6` dB over its own
/// 4,000 samples, from block 4) on the riding fader, heard 1, 12, 24, 26 and 30 blocks (3,872,
/// 2,464, 928, 672 and 160 samples left; `J` = 960), then a table-only rebuild: the same ride
/// split in two at 10,000, or a change only in its last 2,000 samples. The move is a VCA move
/// (the committed model's VCA moves) or a hand move (the ride's stored offset moves, R17-1).
/// Every block, before and after the swap, is bit-equal to the uninterrupted engine's, the
/// render is allocation-free, the successor is adopted at the swap block and the strip carries.
///
/// Test value: red if the adoption rule compares the curve with the stage's fader gain rather
/// than its ramp's target (#1711 F0: the held grid events only remember a gain inside the move's
/// ramp, so the fader gain follows the ride and every heard move lands at the swap), or if it
/// compares at the block's first sample rather than at the ramp's end.
#[cfg(feature = "test-support")]
#[test]
fn a_level_move_in_flight_across_a_table_only_rebuild_keeps_its_bits() {
    const AFTER: usize = 44;
    let changes = [
        ("the ride split at 10,000", (10_000, -12.0)),
        (
            "a change in the last 2,000 samples",
            (RIDE_FRAMES - 2_000, -6.0),
        ),
    ];
    // (label, the stored hand offset after the move, the VCA before and after it)
    let moves = [
        ("a VCA move", 0.0, Some((0.0_f32, -6.0_f32))),
        ("a hand move", -6.0, None),
    ];
    for backend in [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
    ] {
        for (what, hand, vca) in moves {
            let a = level_move_model(None, 0.0, vca.map(|vca| vca.0));
            let committed = level_move_model(None, hand, vca.map(|vca| vca.1));
            for heard in [1, 12, 24, 26, 30] {
                let swap = MOVE_BLOCK + heard;
                let blocks = swap + AFTER;
                let reference = level_move_run(&a, None, &[MOVE], backend, blocks);
                assert_eq!(reference.allocations, (0, 0, 0));
                for (change, split) in changes {
                    let label = format!("{backend:?}, {what}, heard {heard}, {change}");
                    let next = level_move_model(Some(split), hand, vca.map(|vca| vca.1));
                    let run = level_move_run(
                        &a,
                        Some((&next, &committed, swap)),
                        &[MOVE],
                        backend,
                        blocks,
                    );
                    assert_eq!(run.adopted, Some(swap), "{label}: adopted at the swap");
                    assert!(run.restarted.is_empty(), "{label}: the strip carries");
                    assert_eq!(run.allocations, (0, 0, 0), "{label}: allocation-free");
                    assert_ne!(
                        reference.blocks[swap - 1],
                        level_move_run(&committed, None, &[], backend, swap).blocks[swap - 1],
                        "{label}: the move is still heard before the swap"
                    );
                    let first = run
                        .blocks
                        .iter()
                        .zip(&reference.blocks)
                        .position(|(run, reference)| run != reference);
                    assert_eq!(first, None, "{label}: first block that differs");
                }
            }
        }
    }
}

// #1711 G2 and R19-1: a live level move whose target a table-only rebuild moves.

/// The first free sample of [`MOVE`]'s ramp: the block-4 entry plus its 4,000 samples.
#[cfg(feature = "test-support")]
const MOVE_FREE: u64 = MOVE_BLOCK as u64 * 128 + MOVE.1.1 as u64;

/// A step ride of track 0 through `(start, end, dB)` pieces.
#[cfg(feature = "test-support")]
fn steps_of(pieces: &[(u64, u64, f32)]) -> Vec<AutomationSegment> {
    pieces
        .iter()
        .map(|&(start, end, db)| segment(AutomationShape::Step, start, end, db, db))
        .collect()
}

/// The moved-target setup (#1711 A3 G2): the ride flat at -3 dB; the rebuild's table -5 dB to
/// 6,000, then -3 dB (the move's target moves from -3 to -5 dB); the reference table, -3 dB to the
/// move's first free sample [`MOVE_FREE`], -5 dB to 6,000, then -3 dB.
#[cfg(feature = "test-support")]
fn moved_target_tables() -> [Vec<AutomationSegment>; 3] {
    [
        steps_of(&[(0, RIDE_FRAMES, -3.0)]),
        steps_of(&[(0, 6_000, -5.0), (6_000, RIDE_FRAMES, -3.0)]),
        steps_of(&[
            (0, MOVE_FREE, -3.0),
            (MOVE_FREE, 6_000, -5.0),
            (6_000, RIDE_FRAMES, -3.0),
        ]),
    ]
}

/// One moved-target case at `backend`: [`MOVE`] heard `heard` blocks on the flat ride, then the
/// table-only rebuild of [`moved_target_tables`], and the `later` live moves after the swap. The
/// rebuilt run is compared with a fresh plan prepared with `reference` (the reference table,
/// or the rebuild's own with later moves) and given the same moves, from sample `from` on: every
/// sample bit-equal, allocation-free, adopted at the swap block, the strip carries. Returns the
/// rebuilt run's left plane.
#[cfg(feature = "test-support")]
fn moved_target_case(
    backend: graph_compiler::Backend,
    case: MovedTargetMove,
    heard: usize,
    later: &[(usize, (f64, u32))],
    from: u64,
) -> Vec<f32> {
    let [flat, table, fresh] = moved_target_tables();
    let reference = if later.is_empty() {
        fresh
    } else {
        table.clone()
    };
    rebuilt_case(
        backend,
        case,
        heard,
        [flat, table, reference],
        (later, &[]),
        from,
    )
}

/// A live move of [`level_move_run`]: `(block, (S, window))`.
#[cfg(feature = "test-support")]
type LiveMove = (usize, (f64, u32));

/// A live hand mute of [`level_move_run_muted`]: `(block, (muted, window))`.
#[cfg(feature = "test-support")]
type LiveMute = (usize, (bool, u32));

/// [`moved_target_case`] over `[base, rebuild, reference]` tables with `later` live moves and
/// live hand `mutes` (both given to the rebuilt run and to the fresh reference plan).
#[cfg(feature = "test-support")]
fn rebuilt_case(
    backend: graph_compiler::Backend,
    case: MovedTargetMove,
    heard: usize,
    tables: [Vec<AutomationSegment>; 3],
    moves: (&[LiveMove], &[LiveMute]),
    from: u64,
) -> Vec<f32> {
    rebuilt_case_over(backend, case, heard, tables, moves, from, None)
}

/// `model` with track 0 also in VCA `vca-ride` (0 dB), whose `offset_db` rides `ride` on both
/// lanes (#1655): track 0's fader channels are then composed lanes over its own curve and the
/// VCA's.
#[cfg(feature = "test-support")]
fn with_vca_ride(mut model: SessionModel, ride: &[AutomationSegment]) -> SessionModel {
    let vca = id("vca-ride");
    model.vcas.push(session::Vca {
        id: vca.clone(),
        fader: session::DualMonoFader {
            left_db: 0.0,
            right_db: 0.0,
            left_mute: false,
            right_mute: false,
        },
        members: vec![model.tracks[0].id.clone()],
    });
    model.automation.push(Automation {
        id: id("vca-ride-offset"),
        target: ControlAddress {
            owner: Owner::Vca(vca),
            section: Section::Own,
            control: 1,
            lane: ParameterChannel::Both,
        },
        segments: ride.to_vec(),
        offset: session::AutomationOffset::default(),
    });
    model
}

/// [`rebuilt_case`], with every model's track 0 also reached by the VCA ride `ride` when given
/// ([`with_vca_ride`]).
#[cfg(feature = "test-support")]
fn rebuilt_case_over(
    backend: graph_compiler::Backend,
    (what, hand, vca): MovedTargetMove,
    heard: usize,
    [flat, table, fresh]: [Vec<AutomationSegment>; 3],
    (later, mutes): (&[LiveMove], &[LiveMute]),
    from: u64,
    ride: Option<&[AutomationSegment]>,
) -> Vec<f32> {
    const AFTER: usize = 44;
    let swap = MOVE_BLOCK + heard;
    let blocks = (swap + AFTER).max(later.iter().map(|(at, _)| at + 20).max().unwrap_or(0));
    let ridden = |model: SessionModel| match ride {
        Some(ride) => with_vca_ride(model, ride),
        None => model,
    };
    let a = ridden(move_model(flat.clone(), 0.0, vca.map(|vca| vca.0)));
    let committed = ridden(move_model(flat, hand, vca.map(|vca| vca.1)));
    let next = ridden(move_model(table.clone(), hand, vca.map(|vca| vca.1)));
    let reference = ridden(move_model(fresh, 0.0, vca.map(|vca| vca.0)));
    let moves: Vec<_> = core::iter::once(MOVE)
        .chain(later.iter().copied())
        .collect();
    let blocks = blocks.max(mutes.iter().map(|(at, _)| at + 20).max().unwrap_or(0));
    let label = format!(
        "{backend:?}, {what}, heard {heard}, {} later moves, mutes {mutes:?}",
        later.len()
    );
    let run = level_move_run_muted(
        &a,
        Some((&next, &committed, swap)),
        &moves,
        mutes,
        backend,
        blocks,
    );
    let expected = level_move_run_muted(&reference, None, &moves, mutes, backend, blocks);
    assert_eq!(run.adopted, Some(swap), "{label}: adopted at the swap");
    assert!(run.restarted.is_empty(), "{label}: the strip carries");
    assert_eq!(run.allocations, (0, 0, 0), "{label}: allocation-free");
    let planes = |run: &LevelMoveRun, plane: usize| -> Vec<u32> {
        run.blocks
            .iter()
            .flat_map(|block| block[plane * 128..(plane + 1) * 128].iter().copied())
            .collect()
    };
    for plane in 0..2 {
        let (x, y) = (planes(&run, plane), planes(&expected, plane));
        let first = (from as usize..x.len()).find(|&sample| x[sample] != y[sample]);
        assert_eq!(
            first, None,
            "{label}: plane {plane}, first sample that differs"
        );
    }
    planes(&run, 0).into_iter().map(f32::from_bits).collect()
}

/// One move of the moved-target cases: (label, the stored hand offset after the move, the VCA
/// before and after it).
#[cfg(feature = "test-support")]
type MovedTargetMove = (&'static str, f32, Option<(f32, f32)>);

/// The two moves of the moved-target cases.
#[cfg(feature = "test-support")]
const MOVED_TARGET_MOVES: [MovedTargetMove; 2] = [
    ("a VCA move", 0.0, Some((0.0, -6.0))),
    ("a hand move", -6.0, None),
];

/// #1711 G2 and R19-1 on host-core, at both bank widths. [`MOVE`] heard 1, 12 and 30 blocks
/// (3,872, 2,464 and 160 samples left; `J` = 960) on a ride flat at -3 dB, then a table-only
/// rebuild to -5 dB up to 6,000: the move's target moves, so the cell jumps over `J` at the move's
/// first free sample (4,512), not at the swap; a move with at least `J` left runs to its end as the
/// uninterrupted engine runs it (R19-1). Every sample is bit-equal to a fresh plan whose table has
/// that jump as a real discontinuity at 4,512 and that is given the same move. Case (c), heard 12
/// and 30: a second live move (`S` to -9 dB over 1,200 samples from block 35, before 4,512, ending
/// after `4,512 + J`) clears the deferral: from its last sample on, every sample is bit-equal to a
/// fresh plan with the new table given both moves, and the lane still moves on that move's last
/// samples.
///
/// Test value: red if the adoption lands a moved target at the swap instead of deferring its jump
/// (the old landing; checked once), if it retargets the move at the swap (over its remaining
/// frames, or over `J`) instead of deferring the jump to the ramp's end, if the grid is not held
/// until the deferral fires (the grid point 32 samples before the move's end restarts the ramp
/// toward -5 dB; checked once), or, case (c), if the newer move does not clear the deferral (the
/// deferred jump then cuts the move short at `4,512 + J`; checked once). The comparison point
/// `at = end` is not observable here (on a flat table both points read -5 dB); G1
/// (`a_level_move_in_flight_across_a_table_only_rebuild_keeps_its_bits`) holds it.
#[cfg(feature = "test-support")]
#[test]
fn a_moved_target_jumps_over_the_fader_length_at_the_moves_first_free_sample() {
    const SECOND: (usize, (f64, u32)) = (35, (-9.0, 1_200));
    let second_last = SECOND.0 as u64 * 128 + u64::from(SECOND.1.1) - 1;
    for backend in [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
    ] {
        for case in MOVED_TARGET_MOVES {
            for heard in [1, 12, 30] {
                moved_target_case(backend, case, heard, &[], 0);
            }
            for heard in [12, 30] {
                let left = moved_target_case(backend, case, heard, &[SECOND], second_last);
                let last = second_last as usize;
                assert_ne!(
                    left[last - 1].to_bits(),
                    left[last].to_bits(),
                    "{backend:?}, {}, heard {heard}: the second move keeps its own length",
                    case.0
                );
            }
        }
    }
}

/// #1711 U1 fold 1 (MAJOR-1) on host-core, at both bank widths. [`MOVE`] heard 1 and 4 blocks
/// on the ride flat at -3 dB, then a table-only rebuild to -5 dB up to 2,000, a linear -8 to
/// -14 dB over 2,000 to 8,000, then -14 dB: the move's target moves, so the cell defers its jump
/// to the move's first free sample (4,512); the table's step at 2,000 lies inside the move's
/// remaining ramp, and its jump replaces the ramp the deferral waits for. Every sample is
/// bit-equal to a fresh plan whose table is -3 dB before 2,000 (the same table after it), given
/// the same move.
///
/// Test value: red if a discontinuity jump on a restartable cell leaves a pending deferral in
/// place (its grid hold then freezes the lane on the step's target until 4,512; at the fold's
/// base the first difference was sample 2,944, max 2.9e-2).
#[cfg(feature = "test-support")]
#[test]
fn a_table_step_inside_a_deferred_moves_ramp_drops_the_deferral() {
    let stepped = |first: f32| {
        let mut segments = steps_of(&[(0, 2_000, first)]);
        segments.push(segment(AutomationShape::Linear, 2_000, 8_000, -8.0, -14.0));
        segments.extend(steps_of(&[(8_000, RIDE_FRAMES, -14.0)]));
        segments
    };
    let [flat, ..] = moved_target_tables();
    for backend in [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
    ] {
        for case in MOVED_TARGET_MOVES {
            for heard in [1, 4] {
                rebuilt_case(
                    backend,
                    case,
                    heard,
                    [flat.clone(), stepped(-5.0), stepped(-3.0)],
                    (&[], &[]),
                    0,
                );
            }
        }
    }
}

/// #1655 U3 fold 1 (MAJOR-1): the composed twin of
/// [`a_table_step_inside_a_deferred_moves_ramp_drops_the_deferral`], at both bank widths. Track
/// 0 is also in VCA `vca-ride`, whose `offset_db` rides linearly from 0 to -6 dB over 48,000
/// samples (the same in every plan), so its fader channels are composed lanes over its own curve
/// and the VCA's. The rebuild moves the own table; the adoption holds every cursor through the
/// move's ramp and defers every cursor's jump to 4,512; the own table's step at 2,000 replaces
/// that ramp on the whole lane, as one cell's step does. Every sample is bit-equal to the fresh
/// plan whose own table is -3 dB before 2,000, given the same move and the same VCA ride.
///
/// Test value: red if a composed lane's jump drops only the stepping cursor's deferral and hold
/// (the VCA cursor then freezes the lane until 4,512 while the fresh plan follows the ride).
#[cfg(feature = "test-support")]
#[test]
fn a_table_step_inside_a_deferred_moves_ramp_releases_every_cursor_of_a_composed_lane() {
    let stepped = |first: f32| {
        let mut segments = steps_of(&[(0, 2_000, first)]);
        segments.push(segment(AutomationShape::Linear, 2_000, 8_000, -8.0, -14.0));
        segments.extend(steps_of(&[(8_000, RIDE_FRAMES, -14.0)]));
        segments
    };
    let ride = [segment(AutomationShape::Linear, 0, RIDE_FRAMES, 0.0, -6.0)];
    let [flat, ..] = moved_target_tables();
    for backend in [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
    ] {
        for case in MOVED_TARGET_MOVES {
            for heard in [1, 4] {
                rebuilt_case_over(
                    backend,
                    case,
                    heard,
                    [flat.clone(), stepped(-5.0), stepped(-3.0)],
                    (&[], &[]),
                    0,
                    Some(&ride),
                );
            }
        }
    }
}

/// #1711 U1 fold 1 (MINOR-2) on host-core, at both bank widths. The G2 setup heard 12 blocks
/// (swap at 2,048, the deferred jump due at 4,512), with a live hand mute at 2,560 and an unmute
/// at 3,072 (inside the deferral) or at 5,120 (after it), each ramp 200 samples or exact. The
/// mute holds the lane and its release clears the deferral, so the lane lands on the new table at
/// the unmute: from the mute's end on, every sample is bit-equal to a fresh plan with the
/// rebuild's table given the same move and mutes. An unmute after 4,512 leaves the deferred jump
/// as it was: every sample is bit-equal to the fresh plan with that jump at 4,512.
///
/// Test value: red if the release of a held lane leaves a pending deferral in place (its grid
/// hold then freezes the lane after the unmute until 4,512).
#[cfg(feature = "test-support")]
#[test]
fn a_hand_mute_inside_a_pending_deferral_lands_the_lane_on_the_new_table_at_the_unmute() {
    const MUTE: usize = 20;
    let [flat, _, fresh] = moved_target_tables();
    let mut table = vec![segment(AutomationShape::Linear, 0, 6_000, -5.0, -8.0)];
    table.extend(steps_of(&[(6_000, RIDE_FRAMES, -3.0)]));
    let [_, deferred_table, _] = moved_target_tables();
    for backend in [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
    ] {
        for case in MOVED_TARGET_MOVES {
            for window in [200, 0] {
                let muted_from = MUTE as u64 * 128 + u64::from(window);
                for (unmute, rebuild, reference, from) in [
                    (24, &table, &table, muted_from),
                    (40, &deferred_table, &fresh, 0),
                ] {
                    rebuilt_case(
                        backend,
                        case,
                        12,
                        [flat.clone(), rebuild.clone(), reference.clone()],
                        (&[], &[(MUTE, (true, window)), (unmute, (false, window))]),
                        from,
                    );
                }
            }
        }
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
mod canonical_preparation {
    use super::*;
    use lane::fpenv::under_hostile_fp_env;

    /// Runs `body` in the canonical environment (the guard restores the thread's word).
    fn canonical<T>(body: impl FnOnce() -> T) -> T {
        let _canonical = lane::CanonicalFpEnv::enter();
        body()
    }

    /// A member dB whose composition with [`VCA_DB`] rounds differently under round-toward-zero,
    /// and whose two compositions convert to different gains (asserted).
    const VCA_MEMBER_DB: f32 = -0.01384;
    /// The left offset of the fixture's VCA.
    const VCA_DB: f32 = 1.0 / 3.0;

    /// Static faders of distinct dB on every lane, input trims and filters, no effect (an effect's
    /// own preparation is outside this gate), a flat automated lane on track 2 and a moving
    /// automated pair on track 5. One VCA reaches tracks 1, 2 and 5; track 1's left lane is
    /// [`VCA_MEMBER_DB`]. Every route carries its own non-0 dB gain (#1709: `compile_graph`
    /// converts it), returned third.
    fn fixture() -> (SessionModel, Vec<f32>, Vec<f32>) {
        let mut model = bank_model();
        model.console.pre_insert.clear();
        model.console.post_insert.clear();
        let mut decibels = Vec::new();
        for (index, track) in model.tracks.iter_mut().enumerate() {
            let k = index as f32;
            track.console.clear();
            track.fader.left_db = -0.37 - 1.13 * k;
            track.fader.right_db = 0.71 - 0.89 * k;
            track.builtins.left.trim_db = -1.3 + 0.21 * k;
            track.builtins.left.hpf_hz = 37.3 + 3.1 * k;
            track.builtins.right.lpf_hz = 15_013.0 - 271.0 * k;
            decibels.extend([track.fader.left_db, track.fader.right_db]);
        }
        let flat = flat_fader(&model, 2, ParameterChannel::Left, -7.31);
        let mut moving = flat_fader(&model, 5, ParameterChannel::Both, -3.17);
        moving.segments[0].end_value = 2.5;
        decibels.extend([-7.31, -3.17]);
        model.automation = vec![flat, moving];
        model.tracks[1].fader.left_db = VCA_MEMBER_DB;
        let members = [1, 2, 5]
            .map(|track| model.tracks[track].id.clone())
            .to_vec();
        model.vcas = vec![session::Vca {
            id: id("vca-a"),
            fader: session::DualMonoFader {
                left_db: VCA_DB,
                right_db: -0.1,
                left_mute: false,
                right_mute: false,
            },
            members,
        }];
        let mut route_decibels = Vec::new();
        for (index, route) in model.routes.iter_mut().enumerate() {
            route.gain_db = -0.77 - 0.53 * index as f32;
            route_decibels.push(route.gain_db);
        }
        (model, decibels, route_decibels)
    }

    /// D2: a caller thread with FTZ, DAZ and round-toward-zero set prepares the plan a canonical
    /// thread prepares. Each arm prepares under its word; both render under the canonical word, so
    /// any difference is a prepared value. The fixture's dB-to-gain conversions and one VCA
    /// composition round differently under round-toward-zero, by a step that reaches the gain
    /// (both asserted), so the gate is not vacuous.
    ///
    /// Test value: red if preparation composes a strip's effective fader, or bakes a fader gain or
    /// an input coefficient, outside `CanonicalFpEnv` (remove the guard around host-core's
    /// `controls::effective`, in the builtins preparation body or in
    /// `into_graph_artifact_with_banks`), so a flat automated lane and a static fader would take
    /// the caller's rounding; or if a route gain is converted outside it (remove the guard at the
    /// entry of host-core's `prepare_host_runtime_with_live_controls_policy_and_spectrum`, #1709).
    #[test]
    fn a_hostile_caller_environment_prepares_the_canonical_faders() {
        let (model, decibels, route_decibels) = fixture();

        let moved = under_hostile_fp_env(|| {
            decibels
                .iter()
                .filter(|&&db| {
                    // `black_box` on the input and both outputs: under `--release` LLVM
                    // otherwise folds the two conversions across the guard.
                    let gain = core::hint::black_box(
                        builtins::checked_fader_gain(core::hint::black_box(db)).expect("in domain"),
                    );
                    let pinned = {
                        let _pin = lane::CanonicalFpEnv::enter();
                        core::hint::black_box(
                            builtins::checked_fader_gain(core::hint::black_box(db))
                                .expect("in domain"),
                        )
                    };
                    gain.to_bits() != pinned.to_bits()
                })
                .count()
        });
        assert!(
            moved > 0,
            "the gate is vacuous: no fixture fader converts differently under the hostile word"
        );
        // #1709: a route gain the hostile word converts differently, so the guard at the entry
        // of host-core's preparation (around `compile_graph`) is discriminated too.
        let routes_moved = under_hostile_fp_env(|| {
            route_decibels
                .iter()
                .filter(|&&db| {
                    let gain =
                        core::hint::black_box(math::db_to_gain_f32(core::hint::black_box(db)));
                    let pinned = {
                        let _pin = lane::CanonicalFpEnv::enter();
                        core::hint::black_box(math::db_to_gain_f32(core::hint::black_box(db)))
                    };
                    gain.to_bits() != pinned.to_bits()
                })
                .count()
        });
        assert!(
            routes_moved > 0,
            "the gate is vacuous: no fixture route gain converts differently under the hostile word"
        );
        let offset = core::hint::black_box(f64::from(VCA_DB));
        let member = core::hint::black_box(VCA_MEMBER_DB);
        let composed = core::hint::black_box(session::vca_compose_db(member, offset));
        let rounded = core::hint::black_box(under_hostile_fp_env(|| {
            session::vca_compose_db(core::hint::black_box(member), core::hint::black_box(offset))
        }));
        assert_ne!(
            builtins::checked_fader_gain(composed).map(f32::to_bits),
            builtins::checked_fader_gain(rounded).map(f32::to_bits),
            "the gate is vacuous: the VCA composition's rounding does not reach the gain"
        );

        let prepared_canonical = canonical(|| prepare(&model, 100_000_000));
        let prepared_hostile = under_hostile_fp_env(|| prepare(&model, 100_000_000));
        let render_arm = |prepared: Result<PreparedHost, String>| {
            canonical(|| render_prepared(prepared.expect("prepares")).0)
        };
        let reference = render_arm(prepared_canonical);
        let candidate = render_arm(prepared_hostile);
        assert!(
            reference[0].iter().any(|&bits| bits != 0),
            "the fixture renders sound"
        );
        for plane in 0..2 {
            let first = reference[plane]
                .iter()
                .zip(&candidate[plane])
                .position(|(a, b)| a != b);
            assert_eq!(
                first, None,
                "plane {plane}: the hostile caller's preparation renders the canonical bits"
            );
        }
    }
}
