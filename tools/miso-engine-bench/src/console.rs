//! Issue #149 item 3: the standing console qualification benchmark.
//!
//! # Why this subject exists
//!
//! The number the sprint inherited is a ragged nine-track EQ-only session. Nine tracks at the
//! launch eight-lane width is one full bank plus a one-track scalar tail -- the worst shape a bank
//! can be asked to render, and not the shape of a mixing session. This subject measures the shape
//! a session actually has: sixty-four tracks, each carrying the full channel strip (input
//! trim/HPF/LPF, a parametric EQ in SIMD rack 1, a compressor in the dynamic rack, a fader and a
//! pan matrix), rendered through a real [`PreparedRenderPlan`] at 48 kHz and a 128-frame quantum.
//!
//! Sixty-four tracks is eight full banks and no tail, so the per-track cost reported here is the
//! cost of a full bank rather than the cost of a remainder. The nine-track fixture is kept as a
//! workload so the two numbers are produced by one binary, on one host, in one run -- the only way
//! the comparison means anything.
//!
//! # The measurement discipline
//!
//! Frozen before timing and not tuned afterwards (AGENTS.md): fixed fixtures, fixed observation
//! count, one warmup pass and two measured rounds supplied by the runner. Percentiles are the
//! shared nearest-rank implementation, and the timed region is `timing::timed`, which panics if
//! the body hashed anything -- so evidence collection cannot drift inside the clock (#104 F1).
//!
//! ## Paired alternation
//!
//! The hoist arms are **interleaved observation by observation**, not run one after the other. A
//! benchmark that renders all of arm A and then all of arm B measures the difference between two
//! points in time as much as the difference between two arms: the host's clock ramps, another
//! tenant lands on the sibling core, a thermal limit engages. Alternating means every drift a run
//! suffers is shared by both arms, and the per-observation pairing makes the delta a distribution
//! rather than a difference of two summaries.
//!
//! ## What the hoist arms measure
//!
//! Issue #144 item 6 elides the smoothing window a parameter opens when it is retargeted to the
//! value it already holds. The two arms deliver the *same* automation traffic and differ only in
//! whether that traffic is stationary:
//!
//! * `restated` sends every parameter its current value on every block. The hoist settles it.
//! * `moving` alternates each parameter between two nearby values on every block, so a real
//!   window is open at all times and the hoist can never fire.
//!
//! A first attempt used a one-ULP move for the control arm and it did **not** work, which is worth
//! recording: a one-ULP change in a dB gain designs to the *same* `f32` SVF coefficient words, so
//! the EQ's hoist fired on the designed words and the "control" arm was hoisted too. That is a
//! real property of the optimisation -- it catches redundancy at the coefficient level, not only
//! at the parameter level -- but it makes one ULP useless as a control. The moving arm therefore
//! uses the smallest step that actually changes the designed words.
//!
//! The two arms deliver the same number of spans and do the same control-plane work, so the delta
//! between them is the ramping decision and the window it opens. `moving` is a faithful cost
//! stand-in for the pre-hoist behaviour of `restated`, which opened exactly that window with a
//! step of `+0.0` -- the same ramped kernel over the same lanes for the same sixty-four samples.
//! Their *audio* differs, and only `quiet` and `restated` are asserted bit-identical.
//!
//! `quiet` delivers no automation at all and is the standing baseline row.
//!
//! # The decomposition rows (issue #163 item 0c)
//!
//! One number for a whole console strip says how long the block took and nothing about where the
//! time went. The five rows added here answer that by subtraction, and they can only be subtracted
//! because every one of them is the *same* fixture with part of the strip removed in code:
//! `sixty_four_track_eq_only`, `sixty_four_track_compressor_only`, `sixty_four_track_builtins_only`,
//! `sixty_four_track_dispatch_only` and `sixty_four_track_idle`. Same tracks, same parameters, same
//! sources, same binary, same run.
//!
//! They are derived from the checked-in model by [`apply_strip`] rather than being five more
//! 900-line TOMLs, for the reason the 128-track stretch fixture already gives: nothing about an
//! emptied rack is a new *shape* to review, and five near-duplicate fixtures would be five files
//! that can drift apart from the one they were copied from. Every derived row says
//! `synthetic_fixture: true` and names what its strip carries, and the validator pins that pairing
//! per workload kind, so a row cannot quietly claim a rack it emptied.
//!
//! # The console-facility arms (issue #163 item 0d)
//!
//! Every session row above renders a plan with no meter stream, no live-console control channel
//! and no observation capacity -- which is not what a running console is. `console_meters` and
//! `console_observation` measure what those facilities cost, under the same paired-alternation
//! protocol as the hoist arms, and through the production entry points rather than a hand-built
//! stand-in: a meter is a `MeterRequest` handed to `prepare_session_builtins`, and observation is
//! `attach_effect_observation_v1` plus an `EffectControlRecordV1::Observe` pushed through the same
//! bounded queue a host pushes it through.
//!
//! `console_observation` measures the issue #143 two-level zero rather than restating it: `absent`
//! has no lane at all, `unarmed` has a lane with nothing armed, `armed` has every declared tap
//! armed. All three carry the control channel, so the deltas are the lane and the arming and never
//! the queue drain.
//!
//! Both records carry a class-A statement asserted in-run: observing a console must not change
//! what the console renders, so every arm of both measurements must produce byte-identical output.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use miso_engine_bench_support::alloc as bench_alloc;
use miso_engine_bench_support::digest::Sha256Sink;
use miso_engine_bench_support::json::escape as json_escape;
use miso_engine_bench_support::metadata::Metadata;
use miso_engine_bench_support::stats;
use miso_engine_bench_support::timing;
use miso_engine_builtins::{MeterConfig, MeterHandle, MeterTap};
use miso_engine_builtins_compiler::{MeterConsumer, MeterRequest};
use miso_engine_core::realtime::{
    PlanarBufferMut, PreparedRenderPlan, RenderIo, RenderTime, audit,
};
use miso_engine_effect_compiler::{
    EffectCompileCaps, EffectControlProducerV1, EffectObservationHandleV1,
    attach_effect_console_v1, attach_effect_observation_v1, launch_native_effect_registry_v1,
    prepare_native_session_effects,
};
use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectControlRecordV1, EffectQuality,
    InitialParameterValue, LinkMode, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedNativeEffectBank, PreparedPortsV1, PreparedSidechainPort,
};
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_lane::Backend;
use miso_engine_session::{
    CompileCaps, DualMonoFader, MatrixOrPan, SessionTomlV1, StableId, compile_session,
    parse_session_toml,
};

const SAMPLE_RATE_HZ: u32 = 48_000;
const QUANTUM: usize = 128;
const OBSERVATIONS: usize = 1_000;
const ISSUE: u32 = 149;

/// Blocks per published meter window and per published observation window.
///
/// Deliberately one number for both: a gain-reduction value and the peak beside it in one console
/// frame have to describe the same span of samples, which is the rule
/// `attach_effect_observation_v1` states and the rule a host follows.
const WINDOW_BLOCKS: u32 = 4;
/// Bounded depth of each effect's live-console control channel in the observation arms.
const CONTROL_QUEUE_DEPTH: usize = 8;
/// Cap on declared observation taps per effect, passed to the observation attach.
const MAXIMUM_OBSERVATION_TAPS: u32 = 8;
/// Bounded depth of each meter stream. Drained outside the clock after every observation.
const METER_QUEUE_DEPTH: usize = 8;

const NINE_TRACK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");
const SIXTY_FOUR_TRACK: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track.toml");

/// The standing session workloads, in emission order.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Workload {
    /// The inherited ragged baseline: nine tracks, EQ only, one full bank plus a scalar tail.
    ///
    /// Kept because it is the fixture the sprint's prior numbers were taken on. It carries no
    /// compressor, so its per-track cost is **not** comparable with the console workloads; that is
    /// what `NineTrackRaggedStrip` is for.
    NineTrackBaseline,
    /// The same channel strip as the console fixture, truncated to nine tracks.
    ///
    /// This is the honest ragged-versus-full comparison: identical strip, identical parameters,
    /// nine tracks (one full eight-lane bank plus a one-track tail) against sixty-four (eight full
    /// banks, no tail). Any per-track difference between this row and the console row is the cost
    /// of the ragged shape and nothing else.
    NineTrackRaggedStrip,
    /// The qualification session: sixty-four full channel strips, eight full banks, no tail.
    SixtyFourTrackConsole,
    /// The stretch fixture: the same strip at 128 tracks, synthesised from the 64-track model.
    OneTwentyEightTrackStretch,
    /// Decomposition (#163 item 0c): the console strip with the dynamic rack emptied.
    ///
    /// Sixty-four tracks of EQ and nothing else. `NineTrackBaseline` is also EQ-only, but at nine
    /// ragged tracks on a different fixture, so it cannot be subtracted from the console row. This
    /// row can: it is the *same* fixture, the same parameters and the same track count, with one
    /// rack emptied, so `sixty_four_track_console - sixty_four_track_eq_only` is the compressor's
    /// share of the block and nothing else.
    SixtyFourTrackEqOnly,
    /// Decomposition: the console strip with SIMD rack 1 emptied. Compressor and builtins only.
    SixtyFourTrackCompressorOnly,
    /// Decomposition: every rack emptied. Input trim/HPF/LPF, fader and pan matrix only.
    SixtyFourTrackBuiltinsOnly,
    /// Decomposition: every rack emptied **and** every builtin asked for its identity.
    ///
    /// Polarity off, trim 0 dB, HPF and LPF at 0 Hz, fader 0 dB unmuted, pan hard identity with no
    /// smoothing.
    ///
    /// What this row is **not**: it is not the cost of dispatch alone, and the record says
    /// `identity` rather than `dispatch` for that reason. A builtin filter at 0 Hz is *disabled*,
    /// and `SvfSection::design` implements disabled by designing an identity section -- `m0 = 1`,
    /// `m1 = m2 = 0`, `k = 0` (`miso-engine-builtins`, the version-1 cutoff contract). The
    /// `enabled` flag it sets is consulted only when the plan computes its tail. The arithmetic
    /// still runs, over the same lanes, every block. The same is true of a 0 dB fader and an
    /// identity pan matrix.
    ///
    /// So this row measures: source fill, per-node graph dispatch, buffer plumbing, route
    /// summation, **and** the whole builtins/fader/matrix chain executing identity kernels. Its
    /// near-equality with `sixty_four_track_builtins_only` is the evidence for exactly that
    /// reading -- the two rows differ only in the *coefficients* the same kernels run, so a large
    /// gap between them would mean this description is wrong.
    SixtyFourTrackDispatchOnly,
    /// The idle row: the full console strip rendering silence.
    ///
    /// Honest statement of what this measures, because the name invites a stronger reading than
    /// the number supports. The plan is the unmodified sixty-four-track console. Every effect,
    /// builtin, fader and matrix is prepared and armed exactly as in `sixty_four_track_console`.
    /// The only difference is the input: every track's source binding writes zeros instead of a
    /// tone, and the arm is warmed for long enough that every recursive filter and every detector
    /// has settled to its silent steady state before the clock starts.
    ///
    /// It is therefore **not** "the cost of a stopped engine". No transport gate, silence gate or
    /// early-out exists on any render path in this tree (#163 phase 4 is the issue that would add
    /// one), so a prepared console renders silence through the entire chain at very nearly the
    /// cost of rendering music. That equality is the finding; this row is the number that states
    /// it. Nothing here is scheduled, decoded or transported: the row measures render only.
    SixtyFourTrackIdle,
}

const WORKLOADS: [Workload; 9] = [
    Workload::NineTrackBaseline,
    Workload::NineTrackRaggedStrip,
    Workload::SixtyFourTrackConsole,
    Workload::OneTwentyEightTrackStretch,
    Workload::SixtyFourTrackEqOnly,
    Workload::SixtyFourTrackCompressorOnly,
    Workload::SixtyFourTrackBuiltinsOnly,
    Workload::SixtyFourTrackDispatchOnly,
    Workload::SixtyFourTrackIdle,
];

/// What a decomposition row does to the fixture's channel strip before it is compiled.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Strip {
    /// The fixture is compiled exactly as written (after any track-count synthesis).
    AsWritten,
    /// The dynamic rack is emptied; SIMD rack 1 keeps its EQ.
    EqOnly,
    /// SIMD rack 1 is emptied; the dynamic rack keeps its compressor.
    CompressorOnly,
    /// Every rack is emptied; the builtins, fader and matrix are left as written.
    BuiltinsOnly,
    /// Every rack is emptied and every builtin, fader and matrix is set to its identity.
    Identity,
}

impl Workload {
    const fn kind(self) -> &'static str {
        match self {
            Self::NineTrackBaseline => "nine_track_baseline",
            Self::NineTrackRaggedStrip => "nine_track_ragged_strip",
            Self::SixtyFourTrackConsole => "sixty_four_track_console",
            Self::OneTwentyEightTrackStretch => "one_twenty_eight_track_stretch",
            Self::SixtyFourTrackEqOnly => "sixty_four_track_eq_only",
            Self::SixtyFourTrackCompressorOnly => "sixty_four_track_compressor_only",
            Self::SixtyFourTrackBuiltinsOnly => "sixty_four_track_builtins_only",
            Self::SixtyFourTrackDispatchOnly => "sixty_four_track_dispatch_only",
            Self::SixtyFourTrackIdle => "sixty_four_track_idle",
        }
    }
    const fn tracks(self) -> u32 {
        match self {
            Self::NineTrackBaseline | Self::NineTrackRaggedStrip => 9,
            Self::OneTwentyEightTrackStretch => 128,
            _ => 64,
        }
    }
    const fn fixture_id(self) -> &'static str {
        match self {
            Self::NineTrackBaseline => "fixtures/session/v1/parametric-eq-nine-track.toml",
            _ => "fixtures/session/v1/console-sixty-four-track.toml",
        }
    }
    /// `true` when the rendered model was derived in code from the named fixture.
    ///
    /// Two derivations qualify and both must say so: cloning the strips to a different track
    /// count, and emptying or neutralising part of the strip for a decomposition row. A derived
    /// model reported as a checked-in fixture would be exactly the "measuring a fiction" failure
    /// the bench discipline exists to catch, so the flag is pinned per kind in the validator.
    const fn synthetic(self) -> bool {
        !matches!(self, Self::NineTrackBaseline | Self::SixtyFourTrackConsole)
    }
    /// The edit this row makes to the fixture's channel strip.
    const fn strip(self) -> Strip {
        match self {
            Self::SixtyFourTrackEqOnly => Strip::EqOnly,
            Self::SixtyFourTrackCompressorOnly => Strip::CompressorOnly,
            Self::SixtyFourTrackBuiltinsOnly => Strip::BuiltinsOnly,
            Self::SixtyFourTrackDispatchOnly => Strip::Identity,
            _ => Strip::AsWritten,
        }
    }
    /// What every track of this row actually carries, named in the record.
    ///
    /// Derived from the fixture for the `AsWritten` rows -- the nine-track fixture's dynamic rack
    /// is empty as written, which is why it reads `eq` rather than `eq+compressor`.
    const fn strip_content(self) -> &'static str {
        match self {
            Self::NineTrackBaseline | Self::SixtyFourTrackEqOnly => "eq",
            Self::SixtyFourTrackCompressorOnly => "compressor",
            Self::SixtyFourTrackBuiltinsOnly => "builtins",
            Self::SixtyFourTrackDispatchOnly => "identity",
            _ => "eq+compressor",
        }
    }
    /// What every track's source binding writes into the graph.
    const fn input_signal(self) -> &'static str {
        match self {
            Self::SixtyFourTrackIdle => "silence",
            _ => "tone",
        }
    }
    /// Untimed blocks rendered before the clock starts.
    ///
    /// The idle row needs a real settling period rather than a token one: every SVF, every
    /// smoother and every compressor detector has to reach its silent steady state, or the row
    /// would report the decay rather than the floor.
    const fn warmup_blocks(self) -> usize {
        match self {
            Self::SixtyFourTrackIdle => 512,
            _ => 0,
        }
    }
}

/// Applies a decomposition row's edit to a parsed session model.
///
/// Every edit is a *removal or a neutralisation*, never an addition: a row can only ever measure a
/// subset of what `sixty_four_track_console` measures, which is what makes the differences between
/// the rows subtractions rather than comparisons of two different sessions.
fn apply_strip(model: &mut SessionTomlV1, strip: Strip) {
    if strip == Strip::AsWritten {
        return;
    }
    for track in &mut model.tracks {
        match strip {
            Strip::AsWritten => unreachable!("returned above"),
            Strip::EqOnly => track.dynamic.effects.clear(),
            Strip::CompressorOnly => track.simd1.effects.clear(),
            Strip::BuiltinsOnly => {
                track.simd1.effects.clear();
                track.dynamic.effects.clear();
            }
            Strip::Identity => {
                track.simd1.effects.clear();
                track.dynamic.effects.clear();
                for channel in [&mut track.builtins.left, &mut track.builtins.right] {
                    channel.polarity_invert = false;
                    channel.trim_db = 0.0;
                    // Zero is how a builtin filter is disabled: `InputBuiltins::prepare` designs
                    // the section from the declared frequency and treats zero as "not enabled".
                    channel.hpf_hz = 0.0;
                    channel.lpf_hz = 0.0;
                }
                track.fader = DualMonoFader {
                    left_db: 0.0,
                    right_db: 0.0,
                    left_mute: false,
                    right_mute: false,
                };
                track.matrix_or_pan = MatrixOrPan::Pan {
                    left: 1.0,
                    right: 1.0,
                    smoothing_samples: 0,
                };
            }
        }
        track.simd2.effects.clear();
    }
}

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    assert_eq!(
        std::env::args_os().count(),
        1,
        "benchmark accepts no arguments"
    );
    let round = round_from_runner();
    // Feature detection ends here, before every timed observation.
    let backend = Backend::current();
    let metadata = &Metadata::gather();

    for workload in WORKLOADS {
        let session = SessionMeasurement::run(workload);
        println!("{}", session.record(workload, round, backend, metadata));
    }
    for workload in [
        Workload::NineTrackRaggedStrip,
        Workload::SixtyFourTrackConsole,
    ] {
        let hoist = HoistMeasurement::run(workload, backend);
        println!("{}", hoist.record(workload, round, backend, metadata));
    }
    let meters = FacilityMeasurement::run(&METER_CONFIGS);
    println!("{}", meters.meters_record(round, backend, metadata));
    let observation = FacilityMeasurement::run(&OBSERVATION_CONFIGS);
    println!(
        "{}",
        observation.observation_record(round, backend, metadata)
    );
}

fn round_from_runner() -> u32 {
    match Metadata::gather().var("MISO_ENGINE_BENCH_ROUND").as_deref() {
        Ok("warmup") => 0,
        Ok("1") => 1,
        Ok("2") => 2,
        _ => panic!("the console benchmark must be launched by its fixed runner"),
    }
}

// ---------------------------------------------------------------------------------------------
// The standing session measurement: a real prepared plan, rendered block by block.
// ---------------------------------------------------------------------------------------------

struct SessionMeasurement {
    ns_per_block: Vec<u64>,
    output_sha256: String,
    audit: audit::AuditSnapshot,
    render_errors: u64,
}

impl SessionMeasurement {
    fn run(workload: Workload) -> Self {
        let mut runtime = SessionRuntime::new(workload);
        let mut durations = Vec::with_capacity(OBSERVATIONS);
        let mut output_hash = Sha256Sink::new();
        let mut render_errors = 0_u64;
        // Untimed settling. Only the idle row asks for any, and it asks for a lot: see
        // `Workload::warmup_blocks`.
        for observation in 0..workload.warmup_blocks() {
            let _ = runtime.render(observation as u64);
        }
        audit::warm_up();
        audit::reset();
        for observation in 0..OBSERVATIONS {
            // The timed region is one block of the production render entry and nothing else. The
            // output identity is taken outside it, which `timing::timed` enforces structurally.
            let (elapsed_ns, result) = timing::timed(|| runtime.render(observation as u64));
            if result.is_err() {
                render_errors += 1;
            }
            runtime.hash_output(&mut output_hash);
            durations.push(elapsed_ns);
        }
        Self {
            ns_per_block: durations,
            output_sha256: output_hash.finish_hex(),
            audit: audit::snapshot(),
            render_errors,
        }
    }

    fn record(
        &self,
        workload: Workload,
        round: u32,
        backend: Backend,
        metadata: &Metadata,
    ) -> String {
        let percentiles = Percentiles::from_samples(&self.ns_per_block);
        let tracks = f64::from(workload.tracks());
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_session\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"synthetic_fixture\":{synthetic},",
                "\"strip_content\":\"{strip}\",\"input_signal\":\"{signal}\",",
                "\"fixture_id\":\"{fixture}\",\"round\":{round},\"backend\":\"{backend}\",",
                "\"sample_rate_hz\":{rate},\"quantum_frames\":{quantum},\"observations\":{obs},",
                "\"units\":\"us_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"min_us_per_block\":{min},\"p50_us_per_block\":{p50},",
                "\"p95_us_per_block\":{p95},\"p99_us_per_block\":{p99},",
                "\"max_us_per_block\":{max},\"p50_us_per_block_per_track\":{per_track},",
                "\"min_ns_per_block\":{min_ns},\"p50_ns_per_block\":{p50_ns},",
                "\"p95_ns_per_block\":{p95_ns},\"p99_ns_per_block\":{p99_ns},",
                "\"max_ns_per_block\":{max_ns},\"output_sha256\":\"{digest}\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"nearest-rank percentiles over per-block nanoseconds; ",
                "one warmup pass and two measured rounds; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = workload.kind(),
            tracks = workload.tracks(),
            synthetic = workload.synthetic(),
            strip = workload.strip_content(),
            signal = workload.input_signal(),
            fixture = json_escape(workload.fixture_id()),
            round = round,
            backend = backend_name(backend),
            rate = SAMPLE_RATE_HZ,
            quantum = QUANTUM,
            obs = OBSERVATIONS,
            min = microseconds(percentiles.min),
            p50 = microseconds(percentiles.p50),
            p95 = microseconds(percentiles.p95),
            p99 = microseconds(percentiles.p99),
            max = microseconds(percentiles.max),
            per_track = format_f64(percentiles.p50 as f64 / 1_000.0 / tracks),
            min_ns = percentiles.min,
            p50_ns = percentiles.p50,
            p95_ns = percentiles.p95,
            p99_ns = percentiles.p99,
            max_ns = percentiles.max,
            digest = self.output_sha256,
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata_fields(metadata),
        )
    }
}

/// Which console-side facilities a prepared arm carries.
///
/// The session rows all use [`PlanConfig::BASELINE`], which is what the console benchmark has
/// always measured: no meter streams, no live-console control channel, no observation capacity.
/// The #163 item 0d arms differ from it in exactly one field each, so the paired delta between two
/// arms is the cost of that one facility.
#[derive(Clone, Copy, PartialEq, Eq)]
struct PlanConfig {
    /// One meter stream per track at the post-matrix tap, as a production console prepares.
    meters: bool,
    /// One bounded live-console control channel per prepared effect.
    control: bool,
    /// Effect observation capacity, and whether its taps are armed.
    observation: ObservationArm,
}

/// The three points of the issue #143 two-level zero, as benchmark arms.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ObservationArm {
    /// Level 1: no lane exists. `attach_effect_observation_v1` is never called.
    Absent,
    /// Level 2: the lane exists and no tap is armed. One predicted branch per effect per block.
    Unarmed,
    /// Every declared tap of every observed effect is armed.
    Armed,
}

impl ObservationArm {
    const fn name(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Unarmed => "unarmed",
            Self::Armed => "armed",
        }
    }
}

impl PlanConfig {
    /// What every `console_session` row measures, and what the console bench has always measured.
    const BASELINE: Self = Self {
        meters: false,
        control: false,
        observation: ObservationArm::Absent,
    };
}

/// The parsed, edited session model a workload renders.
///
/// Split out of `SessionRuntime` so the meter and observation arms build the *same* model the
/// `sixty_four_track_console` row builds, through the same code, rather than a second transcription
/// of it.
fn console_model(workload: Workload) -> SessionTomlV1 {
    let text = match workload {
        Workload::NineTrackBaseline => NINE_TRACK,
        _ => SIXTY_FOUR_TRACK,
    };
    let mut model = parse_session_toml(text).expect("frozen console session fixture");
    model.automation.clear();
    if model.tracks.len() != workload.tracks() as usize {
        synthesise_tracks(&mut model, workload.tracks() as usize);
    }
    apply_strip(&mut model, workload.strip());
    assert_eq!(
        model.tracks.len(),
        workload.tracks() as usize,
        "{}: the fixture must carry exactly the declared track count",
        workload.kind()
    );
    model
}

/// One meter stream per track at the post-matrix tap, in canonical track order.
///
/// This is the shape `miso-engine-host-core` prepares for a real console session: handles are
/// `index + 1` so they are nonzero and stable, the tap is the one a console meters by default, and
/// the window is [`WINDOW_BLOCKS`] blocks. Nothing here is a benchmark convenience -- an arm that
/// metered a shape no host prepares would report a cost nobody pays.
fn meter_requests(model: &SessionTomlV1) -> Vec<MeterRequest> {
    let config = MeterConfig {
        period_frames: NonZeroU32::new(WINDOW_BLOCKS * QUANTUM as u32).expect("nonzero period"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(METER_QUEUE_DEPTH).expect("nonzero depth"),
        reset_generation: 0,
    };
    model
        .tracks
        .iter()
        .enumerate()
        .map(|(index, track)| MeterRequest {
            handle: MeterHandle(NonZeroU64::new(index as u64 + 1).expect("nonzero handle")),
            track_id: track.id.as_str().to_owned(),
            tap: MeterTap::PostMatrix,
            config,
        })
        .collect()
}

struct SessionRuntime {
    plan: PreparedRenderPlan,
    output: Vec<f32>,
    /// Control-side halves, held for the arms that attached them. Never touched inside the clock.
    meter_consumers: Vec<MeterConsumer>,
    controls: Vec<EffectControlProducerV1>,
    observations: Vec<EffectObservationHandleV1>,
}

impl SessionRuntime {
    fn new(workload: Workload) -> Self {
        Self::build(workload, PlanConfig::BASELINE)
    }

    fn build(workload: Workload, config: PlanConfig) -> Self {
        let model = console_model(workload);
        let session = compile_session(&model, compile_caps()).expect("compiled console session");
        let meters = if config.meters {
            meter_requests(&model)
        } else {
            Vec::new()
        };
        let builtins = miso_engine_builtins_compiler::prepare_session_builtins(
            &session,
            &meters,
            builtin_caps(),
        )
        .expect("prepared console builtins");
        let registry = launch_native_effect_registry_v1().expect("launch effect registry");
        let mut effects = prepare_native_session_effects(&session, &registry, effect_caps())
            .expect("prepared console effects");
        // Both attaches are the production entry points, called in the order a host calls them.
        // The control channel is attached for every observation arm including `Absent`, so the
        // paired delta between the arms is the observation lane and not the control queue drain.
        let controls = if config.control {
            attach_effect_console_v1(
                &mut effects,
                NonZeroUsize::new(CONTROL_QUEUE_DEPTH).expect("nonzero depth"),
            )
            .expect("live-console control channels")
        } else {
            Vec::new()
        };
        let observations = if config.observation == ObservationArm::Absent {
            Vec::new()
        } else {
            attach_effect_observation_v1(&mut effects, MAXIMUM_OBSERVATION_TAPS, WINDOW_BLOCKS)
                .expect("effect observation capacity")
        };
        let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            dispatch: Backend::current(),
            plan_id: u64::from(ISSUE),
            effects,
            builtins,
            caps: graph_caps(),
        })
        .unwrap_or_else(|_| panic!("{}: production console graph", workload.kind()));

        let envelope = artifact.envelope();
        let silent = workload.input_signal() == "silence";
        let nodes = artifact
            .external_binding_nodes()
            .map(|node| GraphNodeBinding::new(node.clone(), source_binding(node, silent)))
            .collect();
        // `observers` stays empty on purpose: it is the *external* observer slot. A meter observer
        // is compiler-owned and is appended to this vector by the sealed builtins artifact inside
        // `into_bound`, which is why `meters: true` is expressed as a meter *request* and not as a
        // hand-built observer. Driving the real path is the whole point of the arm.
        let bound = artifact
            .into_bound(GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes,
                observers: Vec::new(),
            })
            .unwrap_or_else(|_| panic!("{}: console graph bindings", workload.kind()));
        assert_eq!(
            bound.meter_consumers.len(),
            meters.len(),
            "{}: every requested meter stream must reach the plan",
            workload.kind()
        );

        let mut runtime = Self {
            plan: bound.plan,
            output: vec![0.0; QUANTUM * 2],
            meter_consumers: bound.meter_consumers,
            controls,
            observations,
        };
        if config.observation == ObservationArm::Armed {
            runtime.arm_observation();
        }
        runtime
    }

    /// Arms every declared tap of every prepared effect, the way a subscribing console does.
    ///
    /// Off the clock, and through the same bounded control queue a host pushes: the records are
    /// drained by the render thread at the top of the next block, so the arm must render at least
    /// one untimed block after this before it is actually armed. Its warmup does.
    fn arm_observation(&mut self) {
        for producer in &mut self.controls {
            for tap_index in 0..producer.descriptor.observations.len() as u32 {
                producer
                    .producer
                    .try_push(EffectControlRecordV1::Observe {
                        tap_index,
                        armed: true,
                        window_blocks: WINDOW_BLOCKS,
                    })
                    .expect("room in the bounded control queue");
            }
        }
    }

    /// Declared observation taps across every prepared lane. Zero for an `Absent` arm.
    fn observation_taps(&self) -> usize {
        self.observations
            .iter()
            .map(|handle| handle.readers.len())
            .sum()
    }

    /// Observation windows this arm has published and not yet acknowledged. Outside the clock.
    fn published_windows(&self) -> u64 {
        self.observations
            .iter()
            .flat_map(|handle| handle.readers.iter())
            .filter_map(|reader| reader.read())
            .map(|window| window.sequence)
            .sum()
    }

    /// Meter frames drained from every stream. Outside the clock, like every evidence step.
    fn drain_meters(&mut self) -> u64 {
        let mut frames = 0;
        for stream in &mut self.meter_consumers {
            while stream.consumer.try_pop().is_ok() {
                frames += 1;
            }
        }
        frames
    }

    fn render(&mut self, observation: u64) -> Result<(), ()> {
        self.plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut self.output, 2, QUANTUM, QUANTUM)
                        .map_err(|_| ())?,
                },
                RenderTime {
                    absolute_sample: observation * QUANTUM as u64,
                },
            )
            .map(|_| ())
            .map_err(|_| ())
    }

    fn hash_output(&self, hash: &mut Sha256Sink) {
        for value in &self.output {
            hash.update(value.to_bits().to_le_bytes());
        }
    }
}

/// Clones the fixture's strips up to `tracks`, keeping every track's parameters distinct.
///
/// The stretch fixture is synthetic and says so in its record. It is a clone of the 64-track
/// model rather than a second checked-in session because nothing about 128 tracks is a new
/// *shape* -- it is sixteen full banks instead of eight -- and a second 288 KiB fixture would be
/// 128 tracks of duplicated text to review for no additional coverage.
fn synthesise_tracks(model: &mut miso_engine_session::SessionTomlV1, tracks: usize) {
    let template: Vec<_> = model.tracks.clone();
    let route = model.routes[0].clone();
    model.tracks.clear();
    model.routes.clear();
    for index in 0..tracks {
        let mut track = template[index % template.len()].clone();
        track.id = StableId::parse(&format!("ch{index:03}")).expect("synthetic track id");
        let mut next = route.clone();
        next.id = StableId::parse(&format!("ch{index:03}-main")).expect("synthetic route id");
        next.source = miso_engine_session::RouteSource::Track {
            track_id: track.id.clone(),
            tap: miso_engine_session::SendTap::PostMatrix,
        };
        model.tracks.push(track);
        model.routes.push(next);
    }
}

// ---------------------------------------------------------------------------------------------
// The hoist measurement: paired alternation between a stationary and a moving arm.
// ---------------------------------------------------------------------------------------------

/// Which arm of the paired comparison an observation belongs to.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Arm {
    /// No automation traffic at all: the standing baseline.
    Quiet,
    /// Every parameter restated at the value it already holds. The hoist settles it.
    Restated,
    /// Every parameter alternating between two nearby values. A window is always open.
    Moving,
}

const ARMS: [Arm; 3] = [Arm::Quiet, Arm::Restated, Arm::Moving];

/// The dB step the moving arm alternates over.
///
/// Small enough to be ordinary console traffic, large enough that the designed `f32` coefficient
/// words genuinely change -- a one-ULP step does not, which is why it cannot be the control.
const MOVING_STEP_DB: f32 = 0.25;

impl Arm {
    const fn name(self) -> &'static str {
        match self {
            Self::Quiet => "quiet",
            Self::Restated => "restated",
            Self::Moving => "moving",
        }
    }
}

struct HoistMeasurement {
    ns_per_block: [Vec<u64>; 3],
    digests: [String; 3],
}

impl HoistMeasurement {
    /// Runs the three arms **interleaved**, one observation each, round-robin.
    fn run(workload: Workload, backend: Backend) -> Self {
        let lanes = BankWidth::for_backend(backend)
            .expect("a native bank width")
            .lanes() as usize;
        // The arm carries the workload's whole track count, as the banks a session would form:
        // nine tracks is two banks (one full, one holding the ragged remainder), sixty-four is
        // eight full banks. Measuring one bank for both would report the same number twice.
        let banks = (workload.tracks() as usize).div_ceil(lanes);
        let mut arms: Vec<HoistArm> = ARMS
            .iter()
            .map(|arm| HoistArm::new(*arm, backend, lanes, banks))
            .collect();

        // Warm every arm before any of them is timed, so no arm pays first-touch inside the clock.
        for arm in &mut arms {
            for observation in 0..64 {
                arm.fill_input();
                arm.render(observation);
            }
        }

        let mut samples: [Vec<u64>; 3] = [
            Vec::with_capacity(OBSERVATIONS),
            Vec::with_capacity(OBSERVATIONS),
            Vec::with_capacity(OBSERVATIONS),
        ];
        // Paired alternation: arm 0, arm 1, arm 2, arm 0, ... Every drift the run suffers is
        // shared by all three arms, and the deltas are per-observation pairs rather than a
        // difference of two summaries taken minutes apart.
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.fill_input();
                let (elapsed_ns, ()) = timing::timed(|| arm.render(observation));
                arm.absorb_output();
                samples[index].push(elapsed_ns);
            }
        }

        let mut arms = arms.into_iter();
        let digests = [
            arms.next().expect("quiet arm").finish_digest(),
            arms.next().expect("restated arm").finish_digest(),
            arms.next().expect("moving arm").finish_digest(),
        ];
        // The measurement is its own class-A gate: the quiet arm and the restated arm must render
        // byte-identical output, because a restated parameter is by construction a no-op. If the
        // hoist ever changed a rendered bit, this run would say so before any number is reported.
        assert_eq!(
            digests[0],
            digests[1],
            "{}: restating a parameter changed rendered output -- the hoist is not bit-identical",
            workload.kind()
        );
        assert_ne!(
            digests[1],
            digests[2],
            "{}: the moving arm must actually move, or it is not a control arm",
            workload.kind()
        );

        Self {
            ns_per_block: samples,
            digests,
        }
    }

    fn record(
        &self,
        workload: Workload,
        round: u32,
        backend: Backend,
        metadata: &Metadata,
    ) -> String {
        let quiet = Percentiles::from_samples(&self.ns_per_block[0]);
        let restated = Percentiles::from_samples(&self.ns_per_block[1]);
        let moving = Percentiles::from_samples(&self.ns_per_block[2]);
        // The paired delta: moving minus restated, per observation, then summarised. This is the
        // ramp arithmetic the hoist elides, measured on pairs taken microseconds apart.
        let paired_delta = paired_median(&self.ns_per_block[2], &self.ns_per_block[1]);

        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_hoist\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"round\":{round},",
                "\"backend\":\"{backend}\",\"bank_boundary\":\"effect_bank\",",
                "\"observations\":{obs},\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\",\"{arm2}\"],",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"quiet_p50_ns\":{q50},\"quiet_p99_ns\":{q99},",
                "\"restated_p50_ns\":{r50},\"restated_p95_ns\":{r95},\"restated_p99_ns\":{r99},",
                "\"moving_p50_ns\":{n50},\"moving_p95_ns\":{n95},\"moving_p99_ns\":{n99},",
                "\"paired_delta_median_ns\":{delta},",
                "\"quiet_output_sha256\":\"{qd}\",\"restated_output_sha256\":\"{rd}\",",
                "\"moving_output_sha256\":\"{nd}\",",
                "\"bit_identity\":\"quiet == restated, asserted in-run\",",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"three arms alternated per observation; nearest-rank ",
                "percentiles over per-block nanoseconds; paired delta is moving minus restated ",
                "per observation; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = workload.kind(),
            tracks = workload.tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            arm0 = ARMS[0].name(),
            arm1 = ARMS[1].name(),
            arm2 = ARMS[2].name(),
            q50 = quiet.p50,
            q99 = quiet.p99,
            r50 = restated.p50,
            r95 = restated.p95,
            r99 = restated.p99,
            n50 = moving.p50,
            n95 = moving.p95,
            n99 = moving.p99,
            delta = paired_delta,
            qd = self.digests[0],
            rd = self.digests[1],
            nd = self.digests[2],
            metadata = metadata_fields(metadata),
        )
    }
}

/// One arm: a bank of parametric EQs and a bank of compressors, driven with one traffic pattern.
struct HoistArm {
    width: BankWidth,
    banks: Vec<Box<dyn PreparedNativeEffectBank>>,
    left: Vec<f32>,
    right: Vec<f32>,
    spans: Vec<PreparedAutomationSpan>,
    offsets: Vec<u32>,
    alternate_spans: Vec<PreparedAutomationSpan>,
    alternate_offsets: Vec<u32>,
    /// One frozen input block, copied in before every observation.
    ///
    /// The bank renders in place, so without this the arm would be filtering its own output a
    /// thousand times over and every arm would decay to the same silence -- which is exactly what
    /// the first version of this subject did, and why its digests compared equal.
    source_left: Vec<f32>,
    source_right: Vec<f32>,
    lanes: usize,
    digest: Sha256Sink,
}

impl HoistArm {
    fn new(arm: Arm, backend: Backend, lanes: usize, banks: usize) -> Self {
        let width = BankWidth::for_backend(backend).expect("a native bank width");
        // The factory comes from the launch registry rather than from a direct dependency on the
        // effect crate: this subject measures what a session would actually instantiate, and the
        // bench crate keeps the dependency boundary it already had.
        let registry = launch_native_effect_registry_v1().expect("launch effect registry");
        let eq = registry
            .get_shared_ascii("miso.parametric-eq")
            .expect("the launch registry carries the parametric EQ");
        let eq_values: Vec<_> = (0..lanes)
            .map(|track| eq_track_values(&*eq, track))
            .collect();
        let eq_requests: Vec<_> = eq_values.iter().map(|values| eq_request(values)).collect();
        let prepared_banks: Vec<Box<dyn PreparedNativeEffectBank>> = (0..banks)
            .map(|_| {
                eq.bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend,
                    width,
                    requests: &eq_requests,
                })
                .expect("valid eq bank request")
                .expect("the native width must bind an eq bank")
            })
            .collect();

        // Both span sets are built once, before any timing, and selected by block parity inside
        // the render call. The timed region allocates nothing.
        let build = |offset_db: f32| {
            let mut spans = Vec::new();
            let mut offsets = vec![0_u32];
            for track in 0..lanes {
                if arm != Arm::Quiet {
                    let value = eq_band0_gain(track) + offset_db;
                    spans.push(PreparedAutomationSpan {
                        kind: AutomationSpanKind::Point,
                        channel: ParameterChannel::Left,
                        // Index 3 is band 1's gain: the parameter the arms restate or move.
                        parameter_index: 3,
                        start_sample: 0,
                        end_sample: 0,
                        start_value: value,
                        end_value: value,
                    });
                }
                offsets.push(spans.len() as u32);
            }
            (spans, offsets)
        };
        let (spans, offsets) = build(0.0);
        let (alternate_spans, alternate_offsets) = match arm {
            Arm::Moving => build(MOVING_STEP_DB),
            _ => build(0.0),
        };

        Self {
            width,
            banks: prepared_banks,
            left: vec![0.0; QUANTUM * lanes * banks],
            right: vec![0.0; QUANTUM * lanes * banks],
            spans,
            offsets,
            alternate_spans,
            alternate_offsets,
            source_left: (0..QUANTUM * lanes * banks)
                .map(|index| ((index as f32) * 0.017).sin() * 0.4)
                .collect(),
            source_right: (0..QUANTUM * lanes * banks)
                .map(|index| ((index as f32) * 0.017).sin() * -0.4)
                .collect(),
            lanes,
            digest: Sha256Sink::new(),
        }
    }

    /// Restores the frozen input. Outside the clock, like every other evidence step.
    fn fill_input(&mut self) {
        self.left.copy_from_slice(&self.source_left);
        self.right.copy_from_slice(&self.source_right);
    }

    /// Folds this observation's output into the arm's running identity. Outside the clock.
    fn absorb_output(&mut self) {
        for value in self.left.iter().chain(self.right.iter()) {
            self.digest.update(value.to_bits().to_le_bytes());
        }
    }

    fn render(&mut self, observation: u64) {
        let first_sample = observation * QUANTUM as u64;
        // A point span is only admitted when it lands on the block's first sample, so the sample
        // stamp is refreshed in place each block. In place, because this runs inside the timed
        // region and the region must not allocate.
        let (spans, offsets) = if observation.is_multiple_of(2) {
            (&mut self.spans, &self.offsets)
        } else {
            (&mut self.alternate_spans, &self.alternate_offsets)
        };
        for span in spans.iter_mut() {
            span.start_sample = first_sample;
            span.end_sample = first_sample;
        }
        let spans = &*spans;
        let stride = QUANTUM * self.lanes;
        for (index, bank) in self.banks.iter_mut().enumerate() {
            let range = index * stride..(index + 1) * stride;
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut self.left[range.clone()],
                    &mut self.right[range],
                    None,
                    QUANTUM as u32,
                    self.width,
                    first_sample,
                    spans,
                    offsets,
                    128,
                )
                .expect("console hoist block"),
            );
        }
    }

    fn finish_digest(self) -> String {
        self.digest.finish_hex()
    }
}

/// Band 0's left gain for `track`, the parameter the hoist arms restate or nudge.
fn eq_band0_gain(track: usize) -> f32 {
    -7.5 + (track % 15) as f32
}

/// A prepare request for one lane of the hoist bank, at the launch rate and quantum.
fn eq_request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    PrepareEffectRequest {
        sample_rate: SAMPLE_RATE_HZ,
        quantum: QUANTUM as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 16,
            maximum_scratch_bytes: 1 << 16,
            maximum_automation_spans_per_block: 48,
        },
    }
}

/// Band 1 enabled as a peaking filter, with its left gain at this track's held value.
///
/// Band 1 has to be **enabled explicitly**: the descriptor's default leaves it disabled, and a
/// disabled band ignores its gain entirely -- an earlier version of this subject left the defaults
/// alone and both hoist arms rendered identical audio because the parameter they were moving was
/// not in the signal path at all.
fn eq_track_values(factory: &dyn NativeEffectFactory, track: usize) -> Vec<InitialParameterValue> {
    let mut values = Vec::new();
    for (index, parameter) in factory.descriptor().parameters.iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            let value = match (index, channel) {
                // Band 1: enabled, peaking, at a per-track frequency.
                (0, _) => 1.0,
                (1, _) => 3.0,
                (2, _) => 400.0 + track as f32 * 37.0,
                (3, ParameterChannel::Left) => eq_band0_gain(track),
                (4, _) => 0.9,
                _ => parameter.default_value,
            };
            values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel,
                value,
            });
        }
    }
    values
}

// ---------------------------------------------------------------------------------------------
// The console-facility measurements (#163 item 0d): meters on/off, observation armed/unarmed.
// ---------------------------------------------------------------------------------------------

/// The workload every facility arm renders. One row, the qualification session.
const FACILITY_WORKLOAD: Workload = Workload::SixtyFourTrackConsole;

/// The meter arms, in emission order.
const METER_CONFIGS: [PlanConfig; 2] = [
    PlanConfig::BASELINE,
    PlanConfig {
        meters: true,
        ..PlanConfig::BASELINE
    },
];

/// The observation arms, in emission order: the two levels of the #143 zero, then armed.
const OBSERVATION_CONFIGS: [PlanConfig; 3] = [
    PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Absent,
    },
    PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Unarmed,
    },
    PlanConfig {
        meters: false,
        control: true,
        observation: ObservationArm::Armed,
    },
];

/// A set of prepared arms of one workload, alternated observation by observation.
///
/// The #104 paired-alternation protocol, the same one the hoist arms use and for the same reason:
/// running all of arm A and then all of arm B measures the difference between two points in time
/// as much as the difference between two arms. Every arm here is a *complete* sixty-four-track
/// console plan built by `SessionRuntime::build` from one `console_model`, so two arms differ in
/// exactly the one `PlanConfig` field that separates them and in nothing else.
struct FacilityMeasurement {
    ns_per_block: Vec<Vec<u64>>,
    digests: Vec<String>,
    audit: audit::AuditSnapshot,
    render_errors: u64,
    meter_frames: u64,
    observation_lanes: usize,
    observation_taps: usize,
    published_windows: Vec<u64>,
}

impl FacilityMeasurement {
    fn run(configs: &[PlanConfig]) -> Self {
        let mut arms: Vec<SessionRuntime> = configs
            .iter()
            .map(|config| SessionRuntime::build(FACILITY_WORKLOAD, *config))
            .collect();
        let mut hashes: Vec<Sha256Sink> = configs.iter().map(|_| Sha256Sink::new()).collect();

        // Warm every arm before any of them is timed, so no arm pays first-touch inside the clock
        // -- and so the armed arm's queued subscriptions are drained by a render that is not
        // measured. Sixty-four blocks is sixteen full observation windows.
        for arm in &mut arms {
            for observation in 0..64 {
                let _ = arm.render(observation);
            }
            arm.drain_meters();
        }

        let mut samples: Vec<Vec<u64>> = configs
            .iter()
            .map(|_| Vec::with_capacity(OBSERVATIONS))
            .collect();
        let mut render_errors = 0_u64;
        let mut meter_frames = 0_u64;
        audit::warm_up();
        audit::reset();
        for observation in 0..OBSERVATIONS as u64 {
            for (index, arm) in arms.iter_mut().enumerate() {
                let (elapsed_ns, result) = timing::timed(|| arm.render(observation));
                if result.is_err() {
                    render_errors += 1;
                }
                samples[index].push(elapsed_ns);
            }
            // Every evidence step is outside the clock and after the whole round-robin, so one
            // arm's bookkeeping never lands between another arm's two timed blocks.
            for (index, arm) in arms.iter_mut().enumerate() {
                arm.hash_output(&mut hashes[index]);
                meter_frames += arm.drain_meters();
            }
        }
        let snapshot = audit::snapshot();

        let observation_lanes = arms
            .iter()
            .map(|arm| arm.observations.len())
            .max()
            .unwrap_or(0);
        let observation_taps = arms
            .iter()
            .map(SessionRuntime::observation_taps)
            .max()
            .unwrap_or(0);
        let published_windows: Vec<u64> =
            arms.iter().map(SessionRuntime::published_windows).collect();
        let digests: Vec<String> = hashes.into_iter().map(Sha256Sink::finish_hex).collect();

        // The class-A statement, asserted in-run rather than only claimed in a record: attaching a
        // meter stream, attaching observation capacity and arming a tap are all *observation*. None
        // of them may change a rendered bit. If one ever did, this run says so before it reports a
        // number.
        for (index, digest) in digests.iter().enumerate() {
            assert_eq!(
                digest, &digests[0],
                "arm {index} rendered different output: an observation facility changed the signal"
            );
        }
        Self {
            ns_per_block: samples,
            digests,
            audit: snapshot,
            render_errors,
            meter_frames,
            observation_lanes,
            observation_taps,
            published_windows,
        }
    }

    fn meters_record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let off = Percentiles::from_samples(&self.ns_per_block[0]);
        let on = Percentiles::from_samples(&self.ns_per_block[1]);
        // The meter arm must actually have metered. A silently empty meter set would report a
        // delta of nothing and look like a wonderful result.
        assert!(
            self.meter_frames > 0,
            "the meters-on arm published no meter frame: it is not measuring meters"
        );
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_meters\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"round\":{round},",
                "\"backend\":\"{backend}\",\"observations\":{obs},",
                "\"pairing\":\"alternating_per_observation\",\"arms\":[\"meters_off\",\"meters_on\"],",
                "\"meter_streams\":{streams},\"meter_tap\":\"post_matrix\",",
                "\"meter_window_blocks\":{window},\"meter_frames_drained\":{frames},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"meters_off_p50_ns\":{o50},\"meters_off_p95_ns\":{o95},\"meters_off_p99_ns\":{o99},",
                "\"meters_on_p50_ns\":{n50},\"meters_on_p95_ns\":{n95},\"meters_on_p99_ns\":{n99},",
                "\"paired_delta_median_ns\":{delta},",
                "\"meters_off_output_sha256\":\"{od}\",\"meters_on_output_sha256\":\"{nd}\",",
                "\"bit_identity\":\"meters_off == meters_on, asserted in-run\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"two arms alternated per observation; nearest-rank ",
                "percentiles over per-block nanoseconds; paired delta is meters_on minus ",
                "meters_off per observation; descriptive only; no threshold\"}}"
            ),
            issue = ISSUE,
            kind = FACILITY_WORKLOAD.kind(),
            tracks = FACILITY_WORKLOAD.tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            streams = FACILITY_WORKLOAD.tracks(),
            window = WINDOW_BLOCKS,
            frames = self.meter_frames,
            o50 = off.p50,
            o95 = off.p95,
            o99 = off.p99,
            n50 = on.p50,
            n95 = on.p95,
            n99 = on.p99,
            delta = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]),
            od = self.digests[0],
            nd = self.digests[1],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata_fields(metadata),
        )
    }

    fn observation_record(&self, round: u32, backend: Backend, metadata: &Metadata) -> String {
        let absent = Percentiles::from_samples(&self.ns_per_block[0]);
        let unarmed = Percentiles::from_samples(&self.ns_per_block[1]);
        let armed = Percentiles::from_samples(&self.ns_per_block[2]);
        // The two halves of the honesty gate. An unarmed lane that published anything would not be
        // unarmed, and an armed lane that published nothing would be measuring the unarmed cost
        // twice and reporting the difference as noise.
        assert_eq!(
            self.published_windows[1], 0,
            "the unarmed arm published an observation window"
        );
        assert!(
            self.published_windows[2] > 0,
            "the armed arm published no observation window: it is not actually armed"
        );
        assert!(
            self.observation_lanes > 0 && self.observation_taps > 0,
            "no observation lane was prepared: the arms are all the level-1 zero"
        );
        format!(
            concat!(
                "{{\"schema_version\":1,\"issue\":{issue},\"record\":\"console_observation\",",
                "\"workload_kind\":\"{kind}\",\"tracks\":{tracks},\"round\":{round},",
                "\"backend\":\"{backend}\",\"observations\":{obs},",
                "\"pairing\":\"alternating_per_observation\",",
                "\"arms\":[\"{arm0}\",\"{arm1}\",\"{arm2}\"],",
                "\"observation_lanes\":{lanes},\"observation_taps\":{taps},",
                "\"observation_window_blocks\":{window},",
                "\"unarmed_windows_published\":{uwin},\"armed_windows_published\":{awin},",
                "\"units\":\"ns_per_block\",\"percentile_method\":\"nearest_rank\",",
                "\"absent_p50_ns\":{a50},\"absent_p95_ns\":{a95},\"absent_p99_ns\":{a99},",
                "\"unarmed_p50_ns\":{u50},\"unarmed_p95_ns\":{u95},\"unarmed_p99_ns\":{u99},",
                "\"armed_p50_ns\":{r50},\"armed_p95_ns\":{r95},\"armed_p99_ns\":{r99},",
                "\"paired_capacity_delta_median_ns\":{capacity},",
                "\"paired_arm_delta_median_ns\":{arming},",
                "\"absent_output_sha256\":\"{ad}\",\"unarmed_output_sha256\":\"{ud}\",",
                "\"armed_output_sha256\":\"{rd}\",",
                "\"bit_identity\":\"absent == unarmed == armed, asserted in-run\",",
                "\"render_errors\":{errors},\"render_total_forbidden_operations\":{forbidden},",
                "{metadata}",
                "\"descriptive_only\":true,",
                "\"statistical_method\":\"three arms alternated per observation; nearest-rank ",
                "percentiles over per-block nanoseconds; capacity delta is unarmed minus absent ",
                "and arm delta is armed minus unarmed, per observation; descriptive only; no ",
                "threshold\"}}"
            ),
            issue = ISSUE,
            kind = FACILITY_WORKLOAD.kind(),
            tracks = FACILITY_WORKLOAD.tracks(),
            round = round,
            backend = backend_name(backend),
            obs = OBSERVATIONS,
            arm0 = ObservationArm::Absent.name(),
            arm1 = ObservationArm::Unarmed.name(),
            arm2 = ObservationArm::Armed.name(),
            lanes = self.observation_lanes,
            taps = self.observation_taps,
            window = WINDOW_BLOCKS,
            uwin = self.published_windows[1],
            awin = self.published_windows[2],
            a50 = absent.p50,
            a95 = absent.p95,
            a99 = absent.p99,
            u50 = unarmed.p50,
            u95 = unarmed.p95,
            u99 = unarmed.p99,
            r50 = armed.p50,
            r95 = armed.p95,
            r99 = armed.p99,
            capacity = paired_median(&self.ns_per_block[1], &self.ns_per_block[0]),
            arming = paired_median(&self.ns_per_block[2], &self.ns_per_block[1]),
            ad = self.digests[0],
            ud = self.digests[1],
            rd = self.digests[2],
            errors = self.render_errors,
            forbidden = self.audit.total(),
            metadata = metadata_fields(metadata),
        )
    }
}

/// The median of the per-observation differences `left[i] - right[i]`.
///
/// Pairs taken microseconds apart, then summarised -- never a difference of two summaries taken
/// minutes apart, which is the whole point of alternating the arms.
fn paired_median(left: &[u64], right: &[u64]) -> i64 {
    let mut paired: Vec<i64> = left
        .iter()
        .zip(right)
        .map(|(left, right)| *left as i64 - *right as i64)
        .collect();
    paired.sort_unstable();
    paired[paired.len() / 2]
}

// ---------------------------------------------------------------------------------------------
// Shared plumbing.
// ---------------------------------------------------------------------------------------------

struct Percentiles {
    min: u64,
    p50: u64,
    p95: u64,
    p99: u64,
    max: u64,
}

impl Percentiles {
    fn from_samples(samples: &[u64]) -> Self {
        let mut sorted = samples.to_vec();
        sorted.sort_unstable();
        assert!(!sorted.is_empty(), "measured observations");
        let rank = |numerator: usize, denominator: usize| {
            stats::nearest_rank(&sorted, numerator, denominator)
        };
        Self {
            min: sorted[0],
            p50: rank(50, 100),
            p95: rank(95, 100),
            p99: rank(99, 100),
            max: *sorted.last().expect("nonempty"),
        }
    }
}

fn microseconds(nanoseconds: u64) -> String {
    format_f64(nanoseconds as f64 / 1_000.0)
}

fn format_f64(value: f64) -> String {
    format!("{value:.3}")
}

fn backend_name(backend: Backend) -> &'static str {
    match backend {
        Backend::Scalar => "Scalar",
        Backend::Simd4 => "Simd4",
        Backend::Simd8 => "Simd8",
    }
}

/// The eleven runner-supplied metadata names, in the order they appear in a record.
const METADATA_NAMES: [&str; 11] = [
    "MISO_ENGINE_BENCH_CPU_MODEL",
    "MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE",
    "MISO_ENGINE_BENCH_RUST_VERSION",
    "MISO_ENGINE_BENCH_LLVM_VERSION",
    "MISO_ENGINE_BENCH_TARGET_TRIPLE",
    "MISO_ENGINE_BENCH_TARGET_FEATURES",
    "MISO_ENGINE_BENCH_PROFILE",
    "MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE",
    "MISO_ENGINE_BENCH_MEASUREMENT_CONTROL",
    "MISO_ENGINE_BENCH_CPU_AFFINITY",
    "MISO_ENGINE_BENCH_CANDIDATE_COMMIT",
];

/// The record-side name of a metadata variable.
fn metadata_key(name: &str) -> &str {
    name.strip_prefix("MISO_ENGINE_BENCH_")
        .expect("every metadata name carries the shared prefix")
}

/// The sorted list of metadata names this run could not resolve.
///
/// #104 F2: a runner that forgets to export a name produced records whose every metadata field was
/// null and which still passed validation. Naming the gaps in the record is what makes a silent
/// export failure visible instead of invisible.
fn missing_metadata(metadata: &Metadata) -> Vec<String> {
    let mut missing: Vec<String> = METADATA_NAMES
        .iter()
        .filter(|name| metadata.var(name).is_err())
        .map(|name| metadata_key(name).to_ascii_lowercase())
        .collect();
    missing.sort();
    missing
}

fn metadata_fields(metadata: &Metadata) -> String {
    let field = |name: &str| match metadata.var(name) {
        Ok(value) => format!("\"{}\"", json_escape(&value)),
        Err(_) => "null".to_string(),
    };
    let missing = missing_metadata(metadata)
        .into_iter()
        .map(|name| format!("\"{name}\""))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "\"cpu_model\":{cpu},\"os\":\"{os}\",\"governor_or_power_mode\":{governor},",
            "\"rust_version\":{rust},\"llvm_version\":{llvm},\"target_triple\":{triple},",
            "\"target_features\":{features},\"profile\":{profile},",
            "\"background_load_note\":{load},\"measurement_control\":{control},",
            "\"cpu_affinity\":{affinity},\"candidate_commit\":{commit},",
            "\"missing_metadata\":[{missing}],",
        ),
        cpu = field("MISO_ENGINE_BENCH_CPU_MODEL"),
        os = std::env::consts::OS,
        governor = field("MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE"),
        rust = field("MISO_ENGINE_BENCH_RUST_VERSION"),
        llvm = field("MISO_ENGINE_BENCH_LLVM_VERSION"),
        triple = field("MISO_ENGINE_BENCH_TARGET_TRIPLE"),
        features = field("MISO_ENGINE_BENCH_TARGET_FEATURES"),
        profile = field("MISO_ENGINE_BENCH_PROFILE"),
        load = field("MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE"),
        control = field("MISO_ENGINE_BENCH_MEASUREMENT_CONTROL"),
        affinity = field("MISO_ENGINE_BENCH_CPU_AFFINITY"),
        commit = field("MISO_ENGINE_BENCH_CANDIDATE_COMMIT"),
        missing = missing,
    )
}

fn compile_caps() -> CompileCaps {
    CompileCaps {
        max_compiled_model_bytes: u64::MAX,
        max_requested_runtime_bytes: u64::MAX,
        max_single_allocation_bytes: u64::MAX,
        max_queue_items: u64::MAX,
        max_source_ring_frames: u64::MAX,
        max_source_ring_bytes: u64::MAX,
    }
}

fn builtin_caps() -> miso_engine_builtins_compiler::BuiltinCompileCaps {
    miso_engine_builtins_compiler::BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

fn effect_caps() -> EffectCompileCaps {
    EffectCompileCaps {
        maximum_total_state_bytes: 1 << 28,
        maximum_scratch_bytes: 1 << 28,
        maximum_automation_spans_per_block: 32,
    }
}

fn graph_caps() -> miso_engine_graph::GraphCompileCaps {
    miso_engine_graph::GraphCompileCaps {
        maximum_nodes: 100_000,
        maximum_edges: 100_000,
        maximum_schedule_items: 100_000,
        maximum_dependency_levels: 100_000,
        maximum_audio_buffer_samples: 100_000_000,
        maximum_delay_samples_per_edge: 1_000_000,
        maximum_total_delay_samples: 100_000_000,
        maximum_graph_bytes: 100_000_000,
        maximum_plan_bytes: 1_000_000_000,
        maximum_single_allocation_bytes: 100_000_000,
        maximum_finite_tail_samples: 10_000_000,
    }
}

/// Every track input is a frozen block per observation; nothing is decoded on the render path.
struct FrozenGraphSource {
    left: [f32; QUANTUM],
    right: [f32; QUANTUM],
}

impl FrozenGraphSource {
    /// `silent` writes exact zeros rather than a scaled-down tone.
    ///
    /// Exact zeros because "quiet" and "silent" are different measurements: a very small nonzero
    /// signal keeps every filter and every detector working, and on some hosts pushes them into
    /// denormal arithmetic, which would make the idle row report a cost *higher* than the console
    /// row for reasons that have nothing to do with idling.
    fn new(track: usize, silent: bool) -> Self {
        let mut left = [0.0; QUANTUM];
        let mut right = [0.0; QUANTUM];
        if !silent {
            for frame in 0..QUANTUM {
                let value = ((frame as f32) * 0.017 + track as f32 * 0.31).sin() * 0.6;
                left[frame] = value;
                right[frame] = -value * 0.75;
            }
        }
        Self { left, right }
    }
}

impl GraphRuntimeProcessor for FrozenGraphSource {
    fn process(
        &mut self,
        block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        block.left.copy_from_slice(&self.left);
        block.right.copy_from_slice(&self.right);
        Ok(())
    }
}

struct GraphIdentity;

impl GraphRuntimeProcessor for GraphIdentity {
    fn process(
        &mut self,
        _block: GraphBindingBlock<'_>,
    ) -> Result<(), miso_engine_core::realtime::RenderError> {
        Ok(())
    }
}

fn source_binding(node: &GraphNodeId, silent: bool) -> Box<dyn GraphRuntimeProcessor> {
    if let GraphNodeId::TrackStage {
        track_id,
        stage: TrackStage::Input,
    } = node
    {
        let id = track_id.as_str();
        let track = id
            .trim_start_matches(|c: char| !c.is_ascii_digit())
            .parse()
            .unwrap_or(0);
        Box::new(FrozenGraphSource::new(track, silent))
    } else {
        Box::new(GraphIdentity)
    }
}
