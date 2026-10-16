//! Stored fader automation at preparation (#1646): its bytes are charged as builtin bank payload
//! (gate 2), and a fader automation keeps a mono track's collapse (gate 3). Gate 1 (flat renders
//! the static bits) is on the two hosts: `crates/capi/src/runtime/tests.rs` and
//! `hosts/host-web/src/tests.rs`. The render plumbing (#1647a): each automated fader lane takes
//! its `a(n)` and, per block, its node-time mapping and the fader row's jump length, with no
//! render allocation. The events on the fader (#1647b): completion samples, jumps, latency, the
//! session seek, quantum invariance, allocation and the scalar oracle's parity (#1647 gates 1-6,
//! R17-4).
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

    assert_eq!(
        with_retained - base_retained,
        charge,
        "the heap the automation retains is its charge: {base_retained} -> {with_retained}"
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
    /// new generation's, from that frame.
    fn seek(&mut self, timeline: u64) {
        self.generation += 1;
        self.prepared
            .sources
            .seek_session(self.generation, timeline, None)
            .expect("the seek is admitted");
        self.frame = timeline;
    }

    /// Render one block; returns its first render sample.
    fn block(&mut self) -> u64 {
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

/// #1647 gate 1: a linear dB fade from `-40` to `0` dB over 48,000 samples on a DC source of
/// `1.0`, `control_smoothing` explicit 0. At each completion sample `64k + 63` the output equals
/// `checked_fader_gain(curve(64k + 63))` exactly, and every sample equals the stage driven
/// directly with the grid's retargets (the ramp kernel's interpolation between them).
///
/// Test value: red if a completion is off by one, if the grid follows render samples instead of
/// node time, if an event's gain is not composed and converted as the static fader's, or if the
/// first grid event is lost to the seed.
#[test]
fn a_fader_fade_reaches_the_curve_at_each_completion_sample() {
    const FRAMES: u64 = 48_000 + 1_024;
    let mut model = plain(1, 128);
    model.control_smoothing = smoothing(0.0);
    let entry = fader_entry(&model, 0, ParameterChannel::Both, fade());
    model.automation = vec![entry.clone()];
    let mut rig = Rig::new(&model);
    rig.until(FRAMES);
    for k in 0..FRAMES / 64 {
        let t = 64 * k + 63;
        for plane in &rig.out {
            assert_eq!(
                plane[t as usize].to_bits(),
                gain(curve(&entry, t as i64)).to_bits(),
                "completion sample {t}"
            );
        }
    }
    let expected = reference(-40.0, &grid_operations(&entry, FRAMES), FRAMES);
    for plane in &rig.out {
        assert_eq!(
            first_difference(&plane[..FRAMES as usize], &expected),
            None,
            "every sample is the ramp kernel's"
        );
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
/// length of 480; a write of 960 to the plan's jump-length cell (#1639) at the block entry at
/// 1,152, while the first jump ramps. The first completes over 480 and the second ramps over 960,
/// bit for bit the stage driven directly with those two retargets.
///
/// Test value: red if a written length cuts the jump in flight, or if the length is read only at
/// preparation or a block late.
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
    let mut rig = Rig::live(&model);
    while rig.rendered < 1_152 {
        rig.block();
    }
    rig.write_fader_jump(&model, 960);
    rig.until(FRAMES);
    let retarget = |db: f32, samples: u32| builtins::FaderOperation::RetargetGain {
        gain: gain(db),
        samples,
    };
    let expected = reference(
        0.0,
        &[(1_000, retarget(-6.0, 480)), (3_000, retarget(-12.0, 960))],
        FRAMES,
    );
    for plane in &rig.out {
        assert_eq!(first_difference(&plane[..FRAMES as usize], &expected), None);
        assert_eq!(plane[1_479].to_bits(), gain(-6.0).to_bits());
        assert_eq!(plane[3_959].to_bits(), gain(-12.0).to_bits());
        assert_ne!(plane[3_958].to_bits(), gain(-12.0).to_bits());
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
/// its left lane) allocate and free nothing after a warm-up.
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
            segments.push(segment(AutomationShape::Step, 60_000, 90_000, 0.0, -12.0));
            segments.push(segment(
                AutomationShape::Linear,
                90_000,
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
    let mut lanes = std::collections::BTreeSet::new();
    for _ in 0..1_000 {
        rig.block();
        let (events, _) = builtins_compiler::test_only_take_fader_events();
        lanes.extend(events.iter().map(|event| (event.lane, event.channel)));
    }
    assert_eq!(lanes.len(), 8, "events on eight lanes: {lanes:?}");
    assert_eq!(rig.allocations, (0, 0, 0), "1,000 blocks of fader events");
}

/// The session of the oracle parity test: a fade on both lanes of track 0, steps on the left lane
/// and a fade on the right lane of track 1, no automation on track 2; the default jump length.
#[cfg(feature = "test-support")]
fn parity_model() -> SessionModel {
    let mut model = plain(3, 128);
    let mut right = fade();
    right[0].start_value = -12.0;
    model.automation = vec![
        fader_entry(&model, 0, ParameterChannel::Both, fade()),
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
/// 1's left lane at block 10 and an unmute ramp at block 20, so a held ramp is reported (D1.2).
/// With handles, the offsets cells take two writes too (#1649 D3): track 0's two lanes a new `S`
/// at block 15 (a jump of 480 samples), and track 1's left lane one at block 20, in the block
/// whose drain starts the unmute ramp, so its jump meets a held gain.
#[cfg(feature = "test-support")]
fn render_parity(mut rig: Rig, blocks: usize) -> [Vec<f32>; 2] {
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
                if block == 20 {
                    handles.strip_controls[1].fader.write_offsets(
                        stamp,
                        0,
                        builtins_compiler::FaderOffsetsRecord {
                            offsets_sum: -3.0,
                            smoothing_samples: 200,
                        },
                    );
                }
            });
        }
        if let (Some(gate), Some(handles)) = (gate.as_mut(), rig.handles.as_mut())
            && block == 15
        {
            gate.write_published(|stamp| {
                for channel in 0..2 {
                    handles.strip_controls[0].fader.write_offsets(
                        stamp,
                        channel,
                        builtins_compiler::FaderOffsetsRecord {
                            offsets_sum: -6.0,
                            smoothing_samples: 480,
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
/// automated strip binds the ramped fader), with live controls (the live fader owner and its pair
/// with the matrix), and prepared between render calls (the split pair), each with a live mute
/// and unmute ramp on an automated lane.
///
/// The live variants also write the offsets cells (#1649): a jump on a free lane and one under a
/// held unmute ramp, read by the fader bank and by the oracle's live fader owner alike.
///
/// Test value: red if the oracle's fader owners ignore the program, take no node time or `a(n)`,
/// defer or fuse an automated fader past its events, or convert an event differently from the
/// bank: "a placement change moves no bit" would fail for automated sessions. Red too if the
/// oracle does not read the offsets cells, or jumps (or forgets a held lane's target) otherwise
/// than the bank.
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
        render_parity(Rig::from(prepared, &model), BLOCKS)
    };
    let live_on = |backend, between: bool| {
        let (prepared, handles) = if between {
            host_core::prepare::test_only_prepare_host_runtime_between_render_calls_on(
                &compiled, &caps, &request, backend,
            )
        } else {
            host_core::test_only_prepare_host_runtime_with_live_controls_on(
                &compiled, &caps, &request, backend,
            )
        }
        .unwrap_or_else(|failure| panic!("{}", String::from_utf8_lossy(failure.as_bytes())));
        let mut rig = Rig::from(prepared, &model);
        rig.handles = Some(handles);
        render_parity(rig, BLOCKS)
    };
    let mut unautomated = model.clone();
    unautomated.automation.clear();
    let without = render_parity(Rig::new(&unautomated), BLOCKS);
    for (label, bank, oracle) in [
        (
            "no live control",
            plain_on(Backend::Simd4),
            plain_on(Backend::Scalar),
        ),
        (
            "live controls",
            live_on(Backend::Simd4, false),
            live_on(Backend::Scalar, false),
        ),
        (
            "between render calls",
            live_on(Backend::Simd4, true),
            live_on(Backend::Scalar, true),
        ),
    ] {
        assert_ne!(bank, without, "{label}: the automation is heard");
        if label != "no live control" {
            assert_ne!(
                bank,
                plain_on(Backend::Simd4),
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

/// #1647 D2 (departure 4): a lane a successor adds starts at its prepared value, the curve's
/// first value, and its first block lands a `Set` when the curve at that block's node time
/// differs. Track 1 gains a fader step from `0` to `-9` dB at timeline 300 in a successor swapped
/// in at block 6 (render sample 768, no seek): from the swap block the output equals, bit for bit,
/// the successor's session rendered fresh from sample 0, whose lane took the jump at 300 (over
/// 240 samples) and holds `-9` dB.
///
/// Test value: red if a restarted or added lane keeps its prepared value into the successor (it
/// would glide from `0` dB on the first grid event instead of standing at the curve).
#[cfg(feature = "test-support")]
#[test]
fn a_lane_a_successor_adds_starts_at_its_curve() {
    use successor::{Feed, SWAP_BLOCK, Session, Successor, reference_run, swapped_run};
    let mut base = plain(2, 128);
    // A 240-sample jump: the fresh reference's ramp at 300 completes before the swap block.
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
    for backend in [
        #[cfg(target_feature = "avx2")]
        graph_compiler::Backend::Simd8,
        graph_compiler::Backend::Simd4,
    ] {
        let swapped = swapped_run(&a, &b, &[&feed], &[&feed], backend, Successor::Carry);
        let reference = reference_run(&b, &[&feed], backend);
        let without = reference_run(&a, &[&feed], backend);
        for (block, (swapped, reference)) in swapped.blocks[SWAP_BLOCK..]
            .iter()
            .zip(&reference[SWAP_BLOCK..])
            .enumerate()
        {
            assert_eq!(
                swapped.bits,
                reference.bits,
                "{backend:?}: block {}",
                SWAP_BLOCK + block
            );
        }
        assert_ne!(
            reference[SWAP_BLOCK].bits, without[SWAP_BLOCK].bits,
            "the ride is heard"
        );
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
    /// [`VCA_MEMBER_DB`]. Every route carries its own non-0 dB gain (TBD48: `compile_graph`
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
    /// entry of host-core's `prepare_host_runtime_with_live_controls_policy_and_spectrum`, TBD48).
    #[test]
    fn a_hostile_caller_environment_prepares_the_canonical_faders() {
        let (model, decibels, route_decibels) = fixture();

        let moved = under_hostile_fp_env(|| {
            decibels
                .iter()
                .filter(|&&db| {
                    let gain = builtins::checked_fader_gain(db).expect("in domain");
                    let pinned = {
                        let _pin = lane::CanonicalFpEnv::enter();
                        builtins::checked_fader_gain(db).expect("in domain")
                    };
                    gain.to_bits() != pinned.to_bits()
                })
                .count()
        });
        assert!(
            moved > 0,
            "the gate is vacuous: no fixture fader converts differently under the hostile word"
        );
        // TBD48: a route gain the hostile word converts differently, so the guard at the entry
        // of host-core's preparation (around `compile_graph`) is discriminated too.
        let routes_moved = under_hostile_fp_env(|| {
            route_decibels
                .iter()
                .filter(|&&db| {
                    let gain = math::db_to_gain_f32(core::hint::black_box(db));
                    let pinned = {
                        let _pin = lane::CanonicalFpEnv::enter();
                        math::db_to_gain_f32(core::hint::black_box(db))
                    };
                    gain.to_bits() != pinned.to_bits()
                })
                .count()
        });
        assert!(
            routes_moved > 0,
            "the gate is vacuous: no fixture route gain converts differently under the hostile word"
        );
        let offset = f64::from(VCA_DB);
        let composed = session::vca_compose_db(VCA_MEMBER_DB, offset);
        let rounded = under_hostile_fp_env(|| session::vca_compose_db(VCA_MEMBER_DB, offset));
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
