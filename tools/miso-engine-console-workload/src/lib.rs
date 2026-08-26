//! The console benchmark's *subject*, shared by the native bench and the wasm guest.
//!
//! # Why this crate exists
//!
//! Issue [#163](https://github.com/misofm/engine-v2/issues/163) phase 2 opens with the owner
//! ruling that the unfused multiply-add contract change must be confirmed **at console level**,
//! not at kernel level. Confirming it at console level means running the console workloads under
//! the target the product ships on, and `docs/rulings/wasm-kernel-timing-interim.md` recorded why
//! that was not reachable in phase 0: `tools/miso-engine-bench/src/console.rs` and every compiler
//! it drives were absent from a `wasm32` build of the bench crate.
//!
//! The recorded blocker turned out to be **bench-tool-level, not crate-level**. All four
//! compilers -- `miso-engine-session`, `miso-engine-builtins-compiler`,
//! `miso-engine-effect-compiler` and `miso-engine-graph-compiler` -- build for
//! `wasm32-unknown-unknown` today, unchanged. The `cfg(not(target_arch = "wasm32"))` gates were
//! entries in the *bench manifest*, expressing that the bench binary is a native tool, and not a
//! statement that the subject could not target wasm. So the port needed no crate change at all:
//! it needed the subject to live somewhere both a native binary and a wasm guest can link it.
//!
//! That is this crate. It holds the nine console workloads, the model derivation, and the
//! prepared-plan runtime that renders them -- lifted verbatim out of `console.rs`, which now links
//! it. Nothing was reimplemented for wasm and nothing is conditional on the target, so the wasm
//! guest and the native bench execute **the same subject**: the same fixtures, the same strip
//! edits, the same caps, the same source bindings, the same `PreparedRenderPlan`, the same render
//! call. A number taken through the guest and a number taken through the bench differ in the
//! target that executed them and in nothing else, which is the only condition under which their
//! ratio means anything.
//!
//! This follows the shape gate G5 already established: `miso-engine-wasm-gate-corpus` is an
//! `rlib` precisely so the native leg links the identical code the `cdylib` guest does.
//!
//! # What is deliberately *not* here
//!
//! The measurement. There is no clock in this crate, no percentile, no record and no statistic.
//! `wasm32-unknown-unknown` cannot construct a `std::time::Instant`, so a subject that timed
//! itself could not be linked into the guest at all -- and a subject that times itself on one
//! target and is timed from outside on another is two subjects. Timing belongs to whichever
//! driver owns a clock: `console.rs` for the native bench, the wasmtime host for the guest.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};

use miso_engine_bench_support::digest::Sha256Sink;
use miso_engine_builtins::{MeterConfig, MeterHandle, MeterTap};
use miso_engine_builtins_compiler::{MeterConsumer, MeterRequest};
use miso_engine_core::realtime::{PlanarBufferMut, PreparedRenderPlan, RenderIo, RenderTime};
use miso_engine_effect_compiler::{
    EffectCompileCaps, EffectControlProducerV1, EffectObservationHandleV1,
    attach_effect_console_v1, attach_effect_observation_v1, launch_native_effect_registry_v1,
    prepare_native_session_effects,
};
use miso_engine_effect_contract::EffectControlRecordV1;
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

/// Sample rate every console workload is prepared and rendered at.
pub const SAMPLE_RATE_HZ: u32 = 48_000;
/// Frames per rendered block.
pub const QUANTUM: usize = 128;
/// Plan identifier handed to the graph compiler. Issue #149, the console qualification issue.
pub const PLAN_ID: u64 = 149;
/// Blocks per published meter window and per published observation window.
///
/// Deliberately one number for both: a gain-reduction value and the peak beside it in one console
/// frame have to describe the same span of samples, which is the rule
/// `attach_effect_observation_v1` states and the rule a host follows.
pub const WINDOW_BLOCKS: u32 = 4;
/// Bounded depth of each effect's live-console control channel in the observation arms.
pub const CONTROL_QUEUE_DEPTH: usize = 8;
/// Cap on declared observation taps per effect, passed to the observation attach.
pub const MAXIMUM_OBSERVATION_TAPS: u32 = 8;
/// Bounded depth of each meter stream. Drained outside the clock after every observation.
pub const METER_QUEUE_DEPTH: usize = 8;

const NINE_TRACK: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");
const SIXTY_FOUR_TRACK: &str =
    include_str!("../../../fixtures/session/v1/console-sixty-four-track.toml");

/// The standing session workloads, in emission order.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Workload {
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

/// The standing session workloads, in the order their records are emitted.
pub const WORKLOADS: [Workload; 9] = [
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
    /// The record-side name of this workload.
    pub const fn kind(self) -> &'static str {
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
    /// How many console tracks this workload renders.
    pub const fn tracks(self) -> u32 {
        match self {
            Self::NineTrackBaseline | Self::NineTrackRaggedStrip => 9,
            Self::OneTwentyEightTrackStretch => 128,
            _ => 64,
        }
    }
    /// The checked-in fixture this workload's model is derived from.
    pub const fn fixture_id(self) -> &'static str {
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
    pub const fn synthetic(self) -> bool {
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
    pub const fn strip_content(self) -> &'static str {
        match self {
            Self::NineTrackBaseline | Self::SixtyFourTrackEqOnly => "eq",
            Self::SixtyFourTrackCompressorOnly => "compressor",
            Self::SixtyFourTrackBuiltinsOnly => "builtins",
            Self::SixtyFourTrackDispatchOnly => "identity",
            _ => "eq+compressor",
        }
    }
    /// What every track's source binding writes into the graph.
    pub const fn input_signal(self) -> &'static str {
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
    pub const fn warmup_blocks(self) -> usize {
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
/// Which console-side facilities a prepared arm carries.
///
/// The session rows all use [`PlanConfig::BASELINE`], which is what the console benchmark has
/// always measured: no meter streams, no live-console control channel, no observation capacity.
/// The #163 item 0d arms differ from it in exactly one field each, so the paired delta between two
/// arms is the cost of that one facility.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PlanConfig {
    /// One meter stream per track at the post-matrix tap, as a production console prepares.
    pub meters: bool,
    /// One bounded live-console control channel per prepared effect.
    pub control: bool,
    /// Effect observation capacity, and whether its taps are armed.
    pub observation: ObservationArm,
}

/// The three points of the issue #143 two-level zero, as benchmark arms.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObservationArm {
    /// Level 1: no lane exists. `attach_effect_observation_v1` is never called.
    Absent,
    /// Level 2: the lane exists and no tap is armed. One predicted branch per effect per block.
    Unarmed,
    /// Every declared tap of every observed effect is armed.
    Armed,
}

impl ObservationArm {
    /// The record-side name of this arm.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Absent => "absent",
            Self::Unarmed => "unarmed",
            Self::Armed => "armed",
        }
    }
}

impl PlanConfig {
    /// What every `console_session` row measures, and what the console bench has always measured.
    pub const BASELINE: Self = Self {
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

/// A block the plan refused, or an output buffer it could not be given.
///
/// Deliberately opaque, and deliberately not a `Result<_, ()>`: a driver counts these into a
/// record's `render_errors` rather than branching on why one happened, and the plan's own error
/// taxonomy is not something a benchmark record reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderFailed;

/// One prepared console arm: a real [`PreparedRenderPlan`] and the buffer it renders into.
///
/// Built by [`SessionRuntime::build`] from a [`Workload`] and a [`PlanConfig`], through the
/// production compile and attach entry points in the order a host calls them. The only thing a
/// driver may do inside a clock is [`SessionRuntime::render`]; every other method on this type is
/// evidence collection and belongs outside it.
pub struct SessionRuntime {
    plan: PreparedRenderPlan,
    output: Vec<f32>,
    /// Control-side halves, held for the arms that attached them. Never touched inside the clock.
    meter_consumers: Vec<MeterConsumer>,
    controls: Vec<EffectControlProducerV1>,
    observations: Vec<EffectObservationHandleV1>,
}

impl SessionRuntime {
    /// The arm every `console_session` row renders: [`PlanConfig::BASELINE`] at the backend this
    /// build detected.
    pub fn new(workload: Workload) -> Self {
        Self::build(workload, PlanConfig::BASELINE)
    }

    /// Compiles, prepares and binds one console arm.
    ///
    /// # Panics
    ///
    /// Panics if any stage of the production pipeline refuses the frozen fixture. Every such
    /// refusal is a defect in the subject rather than a condition a measurement may report
    /// around, so this fails loudly instead of recording a number for a plan it did not build.
    pub fn build(workload: Workload, config: PlanConfig) -> Self {
        Self::build_with_dispatch(workload, config, Backend::current())
    }

    /// Compiles, prepares and binds one console arm at an explicitly chosen lane width.
    ///
    /// [`SessionRuntime::build`] dispatches at [`Backend::current()`], which is what the engine
    /// does in production and what every native console record was taken at. This entry point
    /// exists for one job: the #163 phase 2 wasm console arm compares a `wasm32` target against a
    /// native one, and those two targets do not offer the same lane width. `simd128` is four
    /// lanes; the native backend this host records on is eight. A ratio taken across that pair
    /// confounds *which target executed the code* with *how wide its vectors were*.
    ///
    /// Driving the native leg at `Simd4` as well as at `Backend::current()` separates the two:
    /// wasm-at-four against native-at-four is one target difference at one width, and native-at-
    /// eight stays in the table as the backend the product actually records on. This is the
    /// discipline phase 0b's kernel arm already used, applied to the console subject.
    ///
    /// # Panics
    ///
    /// As [`SessionRuntime::build`].
    pub fn build_with_dispatch(workload: Workload, config: PlanConfig, dispatch: Backend) -> Self {
        Self::build_full(workload, config, dispatch, SourceSignal::Local)
    }

    /// Compiles, prepares and binds one console arm, choosing both the lane width and where its
    /// input samples come from.
    ///
    /// The source choice exists for exactly one reason, recorded on [`source_block`]: the tone is
    /// a libm sine, and libm differs between the native and wasm targets. A driver comparing two
    /// targets injects one target's samples into both, so a digest difference between the legs is
    /// a difference in how the *engine* computed and never a difference in what it was asked to
    /// compute.
    ///
    /// # Panics
    ///
    /// As [`SessionRuntime::build`], and additionally if an injected table does not cover every
    /// track the workload binds.
    pub fn build_full(
        workload: Workload,
        config: PlanConfig,
        dispatch: Backend,
        source: SourceSignal,
    ) -> Self {
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
            dispatch,
            plan_id: PLAN_ID,
            effects,
            builtins,
            caps: graph_caps(),
        })
        .unwrap_or_else(|_| panic!("{}: production console graph", workload.kind()));

        let envelope = artifact.envelope();
        let silent = workload.input_signal() == "silence";
        let nodes = artifact
            .external_binding_nodes()
            .map(|node| GraphNodeBinding::new(node.clone(), source_binding(node, silent, &source)))
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

    /// Prepared observation lanes. Zero for an `Absent` arm.
    ///
    /// A method rather than a public field: the arms hold control-side halves that must never be
    /// reachable from a timed region, and an accessor is what keeps that boundary reviewable.
    pub fn observation_lanes(&self) -> usize {
        self.observations.len()
    }

    /// Declared observation taps across every prepared lane. Zero for an `Absent` arm.
    pub fn observation_taps(&self) -> usize {
        self.observations
            .iter()
            .map(|handle| handle.readers.len())
            .sum()
    }

    /// Observation windows this arm has published and not yet acknowledged. Outside the clock.
    pub fn published_windows(&self) -> u64 {
        self.observations
            .iter()
            .flat_map(|handle| handle.readers.iter())
            .filter_map(|reader| reader.read())
            .map(|window| window.sequence)
            .sum()
    }

    /// Meter frames drained from every stream. Outside the clock, like every evidence step.
    pub fn drain_meters(&mut self) -> u64 {
        let mut frames = 0;
        for stream in &mut self.meter_consumers {
            while stream.consumer.try_pop().is_ok() {
                frames += 1;
            }
        }
        frames
    }

    /// Renders exactly one block. This is the whole of what a driver may time.
    ///
    /// # Errors
    ///
    /// Returns [`RenderFailed`] if the output buffer could not be described or the plan refused
    /// the
    /// block. The caller counts these into `render_errors` rather than panicking, so a run that
    /// fails to render still reports how often it failed.
    pub fn render(&mut self, observation: u64) -> Result<(), RenderFailed> {
        self.plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut self.output, 2, QUANTUM, QUANTUM)
                        .map_err(|_| RenderFailed)?,
                },
                RenderTime {
                    absolute_sample: observation * QUANTUM as u64,
                },
            )
            .map(|_| ())
            .map_err(|_| RenderFailed)
    }

    /// Folds this block's rendered output into a digest. Outside the clock, always.
    pub fn hash_output(&self, hash: &mut Sha256Sink) {
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

/// Values in one track's frozen input block: `QUANTUM` left samples then `QUANTUM` right.
pub const SOURCE_BLOCK_VALUES: usize = QUANTUM * 2;

/// One track's frozen input block, left channel followed by right.
///
/// # Why this is public
///
/// Because a cross-target driver has to be able to compute it on **one** target and hand the
/// result to the other. The tone is a sine, `f32::sin` is a libm call, and libm is not the same
/// implementation on `x86_64-unknown-linux-gnu` as it is on `wasm32-unknown-unknown`. Measured on
/// the #163 phase 2 arm: with this tone computed locally on each target, four of the nine console
/// rows rendered different bits on wasm than on native; with the identical tone injected into
/// both, all nine rows agree to the byte, at both lane widths.
///
/// So that difference was never the engine's. It was the benchmark's *input*, and it would have
/// been reported as a cross-target numeric divergence by anyone who did not look. Nothing
/// downstream of this function calls libm.
#[must_use]
pub fn source_block(track: usize, silent: bool) -> Vec<f32> {
    let mut values = vec![0.0; SOURCE_BLOCK_VALUES];
    if !silent {
        for frame in 0..QUANTUM {
            let value = ((frame as f32) * 0.017 + track as f32 * 0.31).sin() * 0.6;
            values[frame] = value;
            values[QUANTUM + frame] = -value * 0.75;
        }
    }
    values
}

/// Where a prepared arm's input samples come from.
///
/// Both variants produce the *same numbers* when the driver supplies what [`source_block`] would
/// have produced. The distinction exists so that a cross-target comparison can guarantee that
/// rather than assume it.
pub enum SourceSignal {
    /// Computed by [`source_block`] on whichever target renders. What the native bench uses, and
    /// what every recorded native console number was taken with.
    Local,
    /// Supplied by the driver: `tracks * SOURCE_BLOCK_VALUES` values in track-major order.
    Injected(Vec<f32>),
}

impl SourceSignal {
    /// This signal's block for one track, or `None` if an injected table does not cover it.
    fn block(&self, track: usize, silent: bool) -> Option<Vec<f32>> {
        match self {
            Self::Local => Some(source_block(track, silent)),
            Self::Injected(table) => table
                .get(track * SOURCE_BLOCK_VALUES..(track + 1) * SOURCE_BLOCK_VALUES)
                .map(<[f32]>::to_vec),
        }
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
    fn from_block(block: &[f32]) -> Self {
        let mut left = [0.0; QUANTUM];
        let mut right = [0.0; QUANTUM];
        left.copy_from_slice(&block[..QUANTUM]);
        right.copy_from_slice(&block[QUANTUM..SOURCE_BLOCK_VALUES]);
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

fn source_binding(
    node: &GraphNodeId,
    silent: bool,
    source: &SourceSignal,
) -> Box<dyn GraphRuntimeProcessor> {
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
        let block = source
            .block(track, silent)
            .unwrap_or_else(|| panic!("the injected source table must cover track {track}"));
        Box::new(FrozenGraphSource::from_block(&block))
    } else {
        Box::new(GraphIdentity)
    }
}
