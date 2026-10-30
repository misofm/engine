//! The shared host preparation pipeline: JSON in, a prepared plan and a source control set out.
//!
//! This is the ~300 lines that the C ABI host and the browser host each carried a private copy of.
//! Everything that differed between the two copies is a field of [`HostPrepareCaps`]; everything
//! else was identical, down to the order of the checks.

use core::num::{NonZeroU32, NonZeroUsize};
use std::collections::BTreeSet;

use builtins::{
    BuiltinLaneSelector, MeterConfig, MeterHandle, MeterMetricSet, MeterTap,
    PreparedInputFilterTarget, checked_fader_gain, checked_trim_gain, validate_input_filter_pair,
};
use builtins_compiler::{
    BuiltinCompileCaps, FaderOffsetsRecord, LiveValuesSuperseded, MeterConsumer, MeterRequest,
    SelectedMeterRequest, TrackControlProducer, TrackControlRecord, TrackControlRequest,
    TrackFaderRecord, TrackInputRecord, lower_matrix_or_pan,
    prepare_selected_session_builtins_with_live_controls,
    prepare_session_builtins_with_live_controls, session_structural_symmetry,
};
use effect_compiler::{
    EffectCompileCaps, EffectControlProducer, EffectControlResources, EffectObservationHandle,
    EffectStateProjection, LiveEffectAddress, LiveEffectRack,
    attach_projected_effect_live_controls, attach_projected_effect_observation,
    effect_control_resources, launch_native_effect_registry, plan_projected_effect_live_controls,
    prepare_native_session_effects, project_native_session_effects, resolve_initial_values,
};
use effect_contract::{
    AutomationRate, BypassSwitch, EffectControlRecord, NativeEffectFactory,
    PREPARED_EFFECT_TARGET_WORDS, ParameterChannel, PreparedEffectTarget, TailSamples,
};
// `LiveRampRow` and `LiveRamps` moved to `effect_contract::control` and `controls` (decision 17);
// these re-exports keep the old paths, so no call site changes.
pub use controls::LiveRamps;
use controls::RebuildReason;
pub use effect_contract::control::LiveRampRow;
use engine::realtime::{GATE_REVISION_MAX, PreparedRenderPlan};
use graph::{
    GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, StableGraphId,
    TrackStage,
};
pub use graph::{GraphJumpLengthProducer, GraphJumpLengths, jump_length_retained_bytes};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler, GraphProjectionRequest};
use lane::kernels::INDEXED_RAMP_LENGTH_MAXIMUM;
use session::{
    CompileCaps, CompiledSession, EditRampRow, EditRampTarget, EditRamps, Effect, EffectIdentity,
    EffectParam, MatrixOrPan, SessionModel, StripKind, StripRef, compile_session,
    parse_session_json,
};
use source::{
    PcmSourceRing, PcmSourceRingConfig, PublishedGeneration, SourceFrame, SourceGeneration,
    SourceGraphSource, SourceGraphTrackMapping, SourceTimeline, prepare_session_graph_source_set,
};
use std::collections::BTreeMap;

use crate::control_preparation::{
    EQ_TARGET_CAPACITY, EQ_VALUE_COUNT, EqTargetEdit, EqTargetPreparer, EqTargetPreparerError,
    INPUT_FILTER_TARGET_CAPACITY, InputFilterEdit, InputFilterPreparer,
};
use crate::diagnostics::{PrepareDiagnostics, PrepareRejection, diagnostic_lines};
use crate::duck::DuckOverlay;
use crate::route_controls::{
    RouteControlError, RouteControlProducer, RouteControlRecord, RouteControlResources,
    route_producer_position, route_record,
};
use crate::source::{ControlSourceBuilder, SourceControlSet};
use crate::spectrum::{
    SpectrumCapture, SpectrumCaptureCollection, SpectrumCaptureCollectionRequest,
    SpectrumCaptureRequest, SpectrumCaptureResources, SpectrumPrepareError,
    prepare_capture_collection, spectrum_capture_collection_resources,
};
use crate::spectrum_service::{PreparedSpectrumCaptures, SpectrumPreparation};

/// The jump lengths a plan's cell holds for `model` (#1639 D1): its `control_smoothing` keys in
/// samples, the values of [`LiveRamps::for_session`]. Preparation seeds every plan's cell with
/// them, and a live `control_smoothing` change writes them (`LiveDelta::jump_lengths`, feature
/// `control-provider`).
#[must_use]
pub fn session_jump_lengths(model: &SessionModel) -> GraphJumpLengths {
    let ramps = LiveRamps::for_session(model);
    GraphJumpLengths {
        fader_samples: ramps.fader_samples,
        mute_samples: ramps.mute_samples,
        pan_samples: ramps.pan_samples,
    }
}

/// The longest producer-thread stall the default source ring hides without an underrun.
pub const SOURCE_STALL_TOLERANCE_MS: u32 = 100;

/// Derive the default per-source ring capacity from the document's rate and quantum (#1406 D2).
///
/// The ring is [`stall_ring_frames`] (the stall tolerance in whole quanta, plus one quantum held
/// by the consumer and one in the recycle path), plus [`p_max_samples`] (the largest `ΣP` a warm
/// successor reads ahead), plus one quantum: so a producer that keeps the ring full through one
/// stall still has the `(k + 1) · q` frames, `k · q <= p_max_samples`, that a warm successor's
/// readiness asks to be queued. Every term is a whole number of quanta, so the result is too. A
/// zero quantum returns zero, and so does a ring above `u32::MAX` frames; callers validate the
/// document before using the answer.
#[must_use]
pub const fn default_source_ring_frames(sample_rate_hz: u32, quantum_frames: u32) -> u32 {
    let stall = stall_ring_frames(sample_rate_hz, quantum_frames) as u64;
    let prime = p_max_samples(sample_rate_hz, quantum_frames) as u64;
    if stall == 0 || prime == 0 {
        return 0;
    }
    let frames = stall + prime + quantum_frames as u64;
    if frames > u32::MAX as u64 {
        0
    } else {
        frames as u32
    }
}

/// The seek runway and deadline `R = D` of a source ring of `ring_frames` at `sample_rate_hz` and
/// `quantum_frames` (#2205, root ruling 2): `min(ceil(50 ms / quantum), ring blocks / 2)` blocks,
/// [`source::seek_runway_blocks`]. Every host prepares it. The browser's control Worker sizes its
/// admission headroom from the same `R` (root ruling 3) with a parallel JS formula, since the
/// engine reports no value of it; the ABI layout's `sourceRing.seekRunwayQuanta` rows carry this
/// function's values at every launch rate, and the Worker harness pins its formula to them. A zero
/// quantum returns zero.
#[must_use]
pub const fn source_seek_runway_blocks(
    sample_rate_hz: u32,
    quantum_frames: u32,
    ring_frames: u32,
) -> u32 {
    if quantum_frames == 0 {
        return 0;
    }
    source::seek_runway_blocks(
        sample_rate_hz,
        quantum_frames,
        ring_frames as u64 / quantum_frames as u64,
    )
}

/// The ring that hides one producer stall (#1354 D1): `ceil(100 ms * fs / quantum)` quanta plus
/// one quantum held by the consumer and one in the recycle path, a whole number of quanta.
///
/// It is the fixed baseline a warm successor's ring headroom is measured against
/// ([`warm_lead`], D2 step 7), so that check does not move when the default ring grows (#1406
/// D2). A zero quantum returns zero, and so does a ring above `u32::MAX` frames.
#[must_use]
pub const fn stall_ring_frames(sample_rate_hz: u32, quantum_frames: u32) -> u32 {
    if quantum_frames == 0 {
        return 0;
    }
    let stall_frames = (sample_rate_hz as u64 * SOURCE_STALL_TOLERANCE_MS as u64) / 1_000;
    let quanta = stall_frames.div_ceil(quantum_frames as u64) + 2;
    let frames = quanta * quantum_frames as u64;
    if frames > u32::MAX as u64 {
        0
    } else {
        frames as u32
    }
}

/// The largest source-read offset `ΣP` a warm successor may reach, in samples (#1406 D1, written
/// by #1354a): `4 · ceil_q(L_max(fs))`, with `L_max(fs) = fs / 100 + 6`, the true-peak limiter's
/// latency (`N + 6` with `N = fs / 100`, `crates/true-peak-limiter`), the largest latency a launch
/// effect declares. *Record the swap block's cost on the 64-track console* (#1286) D3 item 1 is
/// the derivation, and its record row measures the adoption block at this value. It is taken at
/// the session's quantum, so it is a whole number of quanta. A zero quantum returns zero, and so
/// does a value above `u32::MAX`.
#[must_use]
pub const fn p_max_samples(sample_rate_hz: u32, quantum_frames: u32) -> u32 {
    if quantum_frames == 0 {
        return 0;
    }
    let latency = sample_rate_hz as u64 / 100 + 6;
    let frames = 4 * latency.div_ceil(quantum_frames as u64) * quantum_frames as u64;
    if frames > u32::MAX as u64 {
        0
    } else {
        frames as u32
    }
}

/// Why a structural edit that grows a carried node's arrival has no warm successor (#1354 D1).
///
/// A value, not a refusal: the edit still commits and takes the transition (D5). Distinct from
/// [`PrepareDiagnostics`], which refuses an edit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WarmUnavailable {
    /// Every predecessor strip that reaches the output would restart whole: that is the
    /// transition's audio.
    Misaligned,
    /// `ΣP + P` exceeds [`WarmConfig::p_max`], or a carried source ring has fewer than `P + q`
    /// frames above [`stall_ring_frames`].
    LeadBound,
    /// The adoption block's prime would move more than [`WarmConfig::prime_bytes_max`] bytes.
    PrimeBudget,
}

/// A warm successor's lead (#1354 D1): every carried node of the successor arrives at exactly its
/// predecessor arrival plus [`Self::lead_samples`].
///
/// Built only by [`warm_lead`]; a host prepares the successor with it through
/// [`SuccessorBase::warm`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WarmLead {
    /// `P`, a whole number of quanta. `0` is an ordinary rebuild that duck-swaps
    /// [`Self::restart_whole`].
    pub lead_samples: u64,
    /// The strips the lead restarts whole (every node of the strip), sorted by ID. An added
    /// strip is never listed.
    pub restart_whole: Box<[String]>,
    /// `R`'s join part (#1354a ruling 2, #1354b): the strips the ordinary successor's carry join
    /// restarts and the strips D2 step 2 adds on the lead's compile. With `restart_whole`, the set
    /// `R` the floors are derived from; the warm successor restarts each. Sorted.
    restarted: Box<[Box<str>]>,
    /// D3's exact byte count.
    prime_bytes: u64,
}

impl WarmLead {
    /// The bytes render writes and reads in the adoption block to prime this lead (#1354 D3):
    /// `4 · (P · Σ channels(s)` over the carried sources `+` the words the prime's fill writes
    /// into every source-claim line (L2) and every `input`-tap sidechain line from a restarted
    /// track into a carried node (L3, #1354b)`)`. Exact: the warm successor reports the same
    /// count over its own lines ([`PreparedHost::prime_bytes`]).
    #[must_use]
    pub const fn prime_bytes(&self) -> u64 {
        self.prime_bytes
    }
}

#[cfg(feature = "test-support")]
impl WarmLead {
    /// Test-support seam (#1397a): the lead [`warm_lead`] answers when the strips it restarts
    /// whole confine a growth (`Warm` with `lead_samples == 0`), for the edit whose ordinary
    /// successor's inventory is `ordinary`, so a host can gate its routing of that answer on a
    /// session family where no edit reaches it. `restart_whole` is sorted by the seam.
    #[doc(hidden)]
    #[must_use]
    pub fn test_only_zero_lead(ordinary: &PlanStateInventory, restart_whole: &[&str]) -> Self {
        let mut whole: Vec<String> = restart_whole.iter().map(|id| (*id).to_owned()).collect();
        whole.sort_unstable();
        Self {
            lead_samples: 0,
            restart_whole: whole.into_boxed_slice(),
            restarted: ordinary.restarted_strips().into(),
            prime_bytes: 0,
        }
    }
}

/// What [`warm_lead`] answers for a structural edit (#1354 D1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WarmDecision {
    /// No carried node's arrival grows and nothing restarts whole: today's rebuild.
    Ordinary,
    /// A warm successor (or, with `lead_samples == 0`, an ordinary rebuild that restarts the
    /// listed strips whole).
    Warm(WarmLead),
    /// The edit takes the transition.
    Unavailable(WarmUnavailable),
}

/// The bounds a warm successor is checked against (#1354 D1). It has no default: every caller
/// passes both. #1360 builds the production value from [`p_max_samples`] and `PRIME_BYTES_MAX`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WarmConfig {
    /// The largest `ΣP + P`, in samples.
    pub p_max: u64,
    /// The largest [`WarmLead::prime_bytes`].
    pub prime_bytes_max: u64,
}

/// How strictly a host constrains the session's external render shape.
///
/// The C ABI host compiles whatever rate the session declares, as long as it is a launch rate; the
/// browser host is handed a fixed `AudioContext` rate and quantum and must reject anything else.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostShapePolicy {
    /// Accept any rate already validated by the session compiler, and whatever quantum the
    /// session declares.
    AnyLaunchRate,
    /// Accept exactly this rate and quantum, and reject every other session.
    Exact {
        /// Required session sample rate.
        sample_rate_hz: u32,
        /// Required session quantum in frames.
        quantum_frames: u32,
    },
}

/// The VCA groups a session may declare when a host bounds them the way the browser does (ruling
/// V-Q3): the browser's `maximum_vcas`.
pub const MAXIMUM_VCAS_BOUNDED: u64 = 256;

/// The (strip, VCA) reach pairs a session may declare (ruling V-Q3): the bound on one VCA move's
/// work, the same on every host.
pub const MAXIMUM_VCA_REACH_PAIRS: u64 = 16_384;

/// Everything one host declares it can drive, as bounds checked during preparation.
///
/// Every field is a hard cap: preparation fails rather than allocating past it. Hosts spell these
/// once, from their own ABI configuration struct, and never re-check them afterwards.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostPrepareCaps {
    /// External render shape the host can drive.
    pub shape: HostShapePolicy,
    /// Per-source ring capacity in frames; must be a nonzero multiple of the session quantum.
    pub source_ring_frames: u32,
    /// Maximum tracks in the session.
    pub maximum_tracks: u64,
    /// Maximum submix strips in the session (#1206). A configured bound of its own, never folded
    /// into `maximum_tracks`: a host that sizes tracks exactly can still bound submixes apart.
    pub maximum_submixes: u64,
    /// Maximum VCA groups in the session (#1243). A configured bound of its own, never folded into
    /// `maximum_tracks` or `maximum_submixes`: a VCA is a control group, not a strip.
    pub maximum_vcas: u64,
    /// Maximum (strip, VCA) reach pairs in the session (ruling V-Q3, batch 7 review M1). A count
    /// of VCAs does not bound a VCA move's work, which grows with the pairs; every host applies
    /// the same bound, [`MAXIMUM_VCA_REACH_PAIRS`], because the move composes every reached
    /// member wherever the control plane runs.
    pub maximum_vca_reach_pairs: u64,
    /// Maximum sources in the session.
    pub maximum_sources: u64,
    /// Maximum routes in the session.
    pub maximum_routes: u64,
    /// Maximum effect instances across all racks.
    pub maximum_effects: u64,
    /// Maximum bytes for the compiled graph, its plan and the compiled session model together.
    pub maximum_graph_session_plus_plan_bytes: u64,
    /// Maximum engine-owned source bytes (rings plus overhead).
    pub maximum_source_total_bytes: u64,
    /// Maximum engine-owned source overhead bytes.
    pub maximum_source_overhead_bytes: u64,
    /// Maximum total effect state bytes.
    pub maximum_effect_state_bytes: u64,
    /// Maximum total effect scratch bytes.
    pub maximum_effect_scratch_bytes: u64,
    /// Maximum engine-owned builtin retained payload bytes.
    pub maximum_builtin_retained_bytes: u64,
    /// Maximum size of any single named allocation.
    pub maximum_named_allocation_bytes: u64,
    /// Maximum builtin meter streams.
    pub maximum_meter_streams: u64,
    /// Maximum builtin meter items.
    pub maximum_meter_items: u64,
    /// Maximum builtin meter bytes.
    pub maximum_meter_bytes: u64,
}

impl HostPrepareCaps {
    /// Check the compiled session's external shape against this host's policy.
    ///
    /// Exposed separately because a host with its own pre-flight resource projection runs that
    /// projection between the shape check and the rest of preparation; [`prepare_host_runtime`]
    /// calls it again, and the check is pure, so calling it twice is free and cannot disagree.
    pub fn validate_shape(&self, compiled: &CompiledSession) -> Result<(), PrepareDiagnostics> {
        match self.shape {
            HostShapePolicy::AnyLaunchRate => {}
            HostShapePolicy::Exact {
                sample_rate_hz,
                quantum_frames,
            } => {
                if compiled.sample_rate().0 != sample_rate_hz
                    || compiled.quantum().0 != quantum_frames
                {
                    return Err(shape("host.session.shape"));
                }
            }
        }
        if self.source_ring_frames < compiled.quantum().0
            || !self.source_ring_frames.is_multiple_of(compiled.quantum().0)
        {
            return Err(shape("host.source.ring_frames"));
        }
        Ok(())
    }

    /// Translate the byte caps into the session compiler's own caps.
    ///
    /// `source_count` is the parsed model's source count: the aggregate source-ring frame cap is
    /// per-session, not per-source.
    ///
    /// Since #241 only `max_compiled_model_bytes` and `max_single_allocation_bytes` can actually
    /// refuse here: the session estimate reports zero for the runtime/queue/ring terms, so the
    /// other four rows are inert (see `CompileCaps`). The host's real ring and source budgets are
    /// enforced below against the resources the chosen ring actually retains, which is the #240
    /// S3.7 ordering -- check the budget after the choice, not against a document word.
    pub fn compile_caps(&self, source_count: usize) -> Result<CompileCaps, PrepareDiagnostics> {
        let source_count = u64::try_from(source_count).map_err(|_| platform("host.count"))?;
        let aggregate_ring_frames = source_count
            .checked_mul(u64::from(self.source_ring_frames))
            .ok_or_else(|| resource("host.resource.arithmetic"))?;
        Ok(CompileCaps {
            max_compiled_model_bytes: self.maximum_graph_session_plus_plan_bytes,
            max_requested_runtime_bytes: self.maximum_graph_session_plus_plan_bytes,
            max_single_allocation_bytes: self.maximum_named_allocation_bytes,
            max_queue_items: u64::MAX,
            max_source_ring_frames: aggregate_ring_frames,
            max_source_ring_bytes: self.maximum_source_total_bytes,
        })
    }
}

/// Address-free resource and shape facts about one prepared session.
///
/// Hosts project their own ABI report from this; the facade deliberately reports engine-owned rows
/// only, so a host's private rows (a Wasm bridge buffer, a C protocol queue) stay the host's.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostPrepareReport {
    /// Session sample rate.
    pub sample_rate_hz: u32,
    /// Session quantum in frames.
    pub quantum_frames: u32,
    /// Sources in the session.
    pub source_count: u64,
    /// Tracks in the session.
    pub track_count: u64,
    /// Submix strips in the session (#1206).
    pub submix_count: u64,
    /// VCA groups in the session (#1243).
    pub vca_count: u64,
    /// Routes in the session.
    pub route_count: u64,
    /// Effect instances in the session.
    pub effect_count: u64,
    /// Compiled output latency in samples.
    pub latency_samples: u64,
    /// The plan's reported output tail: the graph's `output_tail_every_peak` (#1379 Amendment 1
    /// H4, H10), from which the output is below `P * 10^(-144/20)` for every graph input peak `P`
    /// at or above the graph's flush floor and exactly `+-0.0` below it. It is never the graph's
    /// decay value `output_tail`, which holds only at or above that floor.
    pub output_tail_every_peak: TailSamples,
    /// Graph plus plan bytes, excluding the compiled session model.
    pub graph_session_plus_plan_bytes: u64,
    /// Incremental plan bytes.
    pub graph_incremental_plan_bytes: u64,
    /// Graph metadata bytes.
    pub graph_metadata_bytes: u64,
    /// Compensation-delay bytes.
    pub graph_delay_bytes: u64,
    /// Effect bank scratch bytes.
    pub effect_bank_scratch_bytes: u64,
    /// Effect bank runtime buffer bytes.
    pub effect_bank_runtime_buffer_bytes: u64,
    /// Effect bank metadata bytes.
    pub effect_bank_metadata_bytes: u64,
    /// Builtin bank bytes.
    pub builtin_bank_bytes: u64,
    /// Builtin bank scratch bytes.
    pub builtin_bank_scratch_bytes: u64,
    /// Source PCM payload bytes already charged to the graph.
    pub source_pcm_payload_bytes: u64,
    /// Source overhead bytes.
    pub source_overhead_bytes: u64,
    /// Total engine-owned source bytes.
    pub source_total_bytes: u64,
    /// Engine-owned bytes of the source rings this plan carries from its predecessor (issue #1272
    /// D4): allocated by the predecessor, moved in at the swap block, and absent from the three
    /// rows above. The single-plan source caps count them with those rows. Zero for a plan
    /// prepared without a successor base.
    pub carried_source_total_bytes: u64,
    /// The overhead part of [`Self::carried_source_total_bytes`].
    pub carried_source_overhead_bytes: u64,
    /// Bytes the installed swap-block carry program retains in the plan
    /// (`graph::carry_program_retained_bytes`), charged against
    /// `maximum_graph_session_plus_plan_bytes`. Zero without a successor base or a carried owner.
    pub carry_program_retained_bytes: u64,
    /// Bytes [`PreparedHost::inventory`] retains ([`PlanStateInventory::retained_bytes`]).
    pub inventory_retained_bytes: u64,
    /// Summed effect state bytes.
    pub effect_scalar_state_bytes: u64,
    /// Summed effect scratch bytes.
    pub effect_scalar_scratch_bytes: u64,
    /// Engine-owned builtin processor payload bytes.
    pub builtin_processor_payload_bytes: u64,
    /// Engine-owned builtin meter payload bytes.
    pub builtin_meter_payload_bytes: u64,
    /// Engine-owned builtin retained payload bytes.
    pub builtin_retained_payload_bytes: u64,
    /// Compiled session model bytes.
    pub session_model_bytes: u64,
    /// Largest single allocation the compiled session model made.
    pub session_largest_allocation_bytes: u64,
    /// Bytes retained by the returned [`SourceControlSet`] (endpoint table plus ID arena).
    pub control_retained_bytes: u64,
    /// Native effect-control table and transferred owner payload allocations.
    pub effect_control_resources: EffectControlResources,
    /// The live send lanes' cells, owners, producer table, route IDs and any route-activity table
    /// they add (issue #1221 D4): `graph::route_control_resources` of the attached producers.
    /// Charged with the effect channels against `maximum_graph_session_plus_plan_bytes` and the
    /// named-allocation cap. All zero when no control channel was requested.
    pub route_control_resources: RouteControlResources,
    /// Engine-owned bytes the compiled plan's observation lanes and slots retain (issue #143 R7).
    ///
    /// Exactly zero for a session that named no observation capacity, and that zero is *walked*
    /// over the built runtime rather than computed from the request.
    pub observation_retained_bytes: u64,
    /// Bytes retained by one prepared spectrum capture, including its graph observer binding.
    /// Zero when this preparation did not request a spectrum capture.
    pub spectrum_capture_retained_bytes: u64,
    /// Total source ID text bytes.
    pub source_id_bytes: u64,
    /// Largest single prepared allocation: the maximum over the graph, source, builtin and
    /// spectrum-capture reports. It excludes the compiled session model, which has its own row
    /// above.
    pub largest_engine_allocation_bytes: u64,
}

/// What a live-control preparation attaches besides the lanes (issue #137 D1/D2): the meters and
/// the observation capacity.
///
/// Every entry that takes a request attaches live controls. A host that wants none calls the plain
/// entry, [`prepare_host_runtime`], which allocates nothing extra. The request has no depth: every
/// lane a strip, an effect or a route owns is a set of latest-target cells with no depth (#1312,
/// #1345, #1346, #1347), and an effect's observation queue holds one slot per tap it declares.
///
/// Which lanes are attached is the caller's [`HostLiveLanes`] (issue #1254 D2): the input cells
/// when [`HostLiveLanes::strip_input`] is set, the effect cells when [`HostLiveLanes::effects`] is
/// set, and the route cells when [`HostLiveLanes::routes`] is set. A strip's fader/mute and
/// matrix/pan cells are always attached (a submix's too, issue #1211 D1). Every entry takes the
/// selection from its caller, except the ones with no `lanes` argument, which attach
/// [`HostLiveLanes::ALL`]. The C ABI's selection is [`controls::live_lanes`] (decision 17, C6).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostLiveControlRequest {
    /// Meter window in blocks for every meter (default or selected), or `0` for the core default,
    /// [`default_meter_window_blocks`] at the plan's rate and quantum (issue #1581 D2). There is
    /// no "no meters" form: every entry that takes a request meters every strip
    /// ([`meter_selection`]) unless the caller selects its own meters.
    ///
    /// [`default_meter_window_blocks`]: crate::meters::default_meter_window_blocks
    /// [`meter_selection`]: crate::meters::meter_selection
    pub meter_window_blocks: u32,
    /// Bounded per-meter snapshot queue depth.
    pub meter_queue_depth: NonZeroUsize,
    /// Maximum declared observation taps to bind per effect instance, or `0` for **no observation
    /// capacity at all** (issue #143 D3, level 1).
    ///
    /// Zero is the honest form of "observation off": `attach_effect_observation` is never
    /// called, so the compiled plan holds no lane, no accumulator and no conflating cell — not a
    /// disabled one, none — and `observation_retained_bytes` is zero. Nonzero requires a control
    /// channel, because a subscription is a record in the effect's own observation queue, which only
    /// the effect's control lane drains.
    ///
    /// The default observation period is the meter window; individual taps
    /// can use other periods. Latest gain-reduction observations can be independently aged,
    /// so a browser frame's peak interval does not establish the gain-reduction interval.
    pub observation_taps: u32,
}

/// The most observation taps any launch native effect declares (#1584 D7): the
/// [`HostLiveControlRequest::observation_taps`] that binds an observation queue slot for every
/// declared tap of every effect. A host whose control plane serves the `Observe` edit asks for it.
/// `0` when the registry failed to build (no effect then prepares).
#[must_use]
pub fn launch_maximum_observation_taps() -> u32 {
    effect_compiler::launch_native_effect_registry().map_or(0, |registry| {
        registry
            .descriptors()
            .map(|descriptor| u32::try_from(descriptor.observations.len()).unwrap_or(u32::MAX))
            .max()
            .unwrap_or(0)
    })
}

impl Default for HostLiveControlRequest {
    fn default() -> Self {
        Self {
            meter_window_blocks: 0,
            meter_queue_depth: NonZeroUsize::MIN,
            observation_taps: 0,
        }
    }
}

/// Which live lanes, beyond every strip's fader/mute and matrix/pan lanes, a live-control
/// preparation attaches (issue #1254 D2).
///
/// It matters only to an entry that takes a [`HostLiveControlRequest`]; the plain entry,
/// [`prepare_host_runtime`], attaches nothing whatever lanes it would be given.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostLiveLanes {
    /// Each strip's input trim/polarity/filter lane. A strip with this lane reports the live
    /// input bound (#1329 D5), finite at every launch rate and valid for every history of live
    /// trim, polarity and filter targets (a live target can enable a filter the session left
    /// off), instead of its prepared design's bound; preparation computes no design bound for it.
    pub strip_input: bool,
    /// One lane per prepared strip effect instance; observation taps ride these.
    pub effects: bool,
    /// One lane per route with live rows (`controls::route_rows_live`: into a submix, or from a
    /// tap before its strip's fader); a live route loses the route fold.
    pub routes: bool,
}

impl HostLiveLanes {
    /// Every lane: what an entry with no `lanes` argument attaches.
    pub const ALL: Self = Self {
        strip_input: true,
        effects: true,
        routes: true,
    };
    /// Only each strip's fader/mute and matrix/pan lanes.
    pub const FADER_AND_MATRIX: Self = Self {
        strip_input: false,
        effects: false,
        routes: false,
    };
}

impl From<controls::LiveLanes> for HostLiveLanes {
    fn from(lanes: controls::LiveLanes) -> Self {
        let controls::LiveLanes {
            strip_input,
            effects,
            routes,
        } = lanes;
        Self {
            strip_input,
            effects,
            routes,
        }
    }
}

/// One caller-selected engine meter observer. Ordering is retained in returned meter handles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostMeterRequest {
    /// Stable compiled identity of the observed strip: a track or a submix (issue #1208 D1). A
    /// submix offers the same seven boundaries a track does.
    pub strip_id: Box<str>,
    /// Chain boundary observed by this meter.
    pub tap: MeterTap,
    /// Numeric groups computed and marked present in each snapshot.
    pub metrics: MeterMetricSet,
}

/// The control-side halves of attached live controls, in canonical strip order.
///
/// `strips` is the addressing authority (issue #1207 D1): every strip of the compiled session,
/// the tracks first in canonical normalized ID order, then the submixes in canonical normalized ID
/// order. A host addresses a strip by its index in this vector; the first `track_count` entries are
/// the tracks. `strip_controls[i]` belongs to `strips[i]` whenever it is present, a submix
/// included (issue #1211 D1), and default `meters[i]` to `strips[i]` (a track). Because the tracks
/// lead, a track's index is the same whether or not the session carries submixes.
pub struct HostLiveControlHandles {
    /// Canonical normalized strip identities: the tracks, then the submixes, each in canonical ID
    /// order. The concatenation as a whole is **not** sorted, so a lookup must search the two
    /// segments apart.
    pub strips: Vec<Box<str>>,
    /// How many leading entries of `strips` are tracks.
    pub track_count: usize,
    /// One control producer per strip, parallel to `strips` -- the tracks, then the submixes
    /// (issue #1211 D1); empty when no channel was requested. A host's track lanes are
    /// `strip_controls[..track_count]`.
    ///
    /// Each carries a strip's builtin channels: always the matrix/pan cells (#137 D1) and the
    /// fader/mute cells (#140 B), and the input trim/polarity/filter cells (#210 phase 3, #1346)
    /// only when [`HostLiveLanes::strip_input`] was set -- under [`HostLiveLanes::ALL`], which an
    /// entry with no `lanes` argument attaches. With [`HostLiveLanes::FADER_AND_MATRIX`] its
    /// `input` is `None`. A submix's producer carries the submix ID in `track_id`.
    pub strip_controls: Vec<TrackControlProducer>,
    /// One control producer per prepared strip effect instance (#140 A); empty when no channel was
    /// requested or [`HostLiveLanes::effects`] was not set (issue #1254). A submix strip's effects
    /// get one exactly as a track's do (issue #1207 D5), and `track_id` then carries the submix
    /// ID. Addressed by `(track_id, address)` in the session's own terms (decision 12, issue
    /// #1096): a console slot by its index in the session's slot order (`pre_insert`, then
    /// `post_insert`), an insert by its index in the strip's `inserts`
    /// ([`Self::effect_control_mut`]).
    pub effect_controls: Vec<EffectControlProducer>,
    /// The meter consumers. With the default set, one per track, in `strips[..track_count]` order;
    /// with a caller selection, one per selected meter in the caller's order, each on any strip
    /// (issue #1208 D1). Empty when no meters were requested.
    pub meters: Vec<MeterConsumer>,
    /// One reader set per prepared strip effect instance that declares an observation tap (issue
    /// #143), a submix strip's effects included (issue #1207 D5).
    ///
    /// Empty when the request named no observation capacity, and empty for every effect whose
    /// descriptor declares no tap. Addressed by `(track_id, address)`, exactly as
    /// [`Self::effect_controls`] is.
    pub effect_observations: Vec<EffectObservationHandle>,
    /// One live send producer per route into a submix, in canonical route-ID order -- the order
    /// the render plane's lanes were attached in (issue #1221 D3). Empty when no channel was
    /// requested or [`HostLiveLanes::routes`] was not set (issue #1254), and routes into the
    /// output have none: they keep their prepared constants and their fold.
    ///
    /// A browser session that opted in to live commands prepares with [`HostLiveLanes::ALL`] and
    /// gets these; one that did not prepares with [`HostLiveLanes::FADER_AND_MATRIX`] (#1326 D3)
    /// and gets none. A C ABI plan prepares with the lanes [`controls::live_lanes`] chooses from
    /// the session's live rows (decision 17, C6): a session with a route into a submix gets these
    /// (#1557). A settled record's bits are a static route's by construction,
    /// because [`RouteControlProducer::record`] takes its target from
    /// `graph_compiler::route_coefficients`, the function the compiler lowers a prepared
    /// route's constants with.
    pub route_controls: Vec<RouteControlProducer>,
    /// One seven-word producer per following route (#1652 D1): a route into a submix whose
    /// source strip automates a lane's mute, in canonical route-ID order. Every plan has them,
    /// whatever the live options; such a route has no entry in [`Self::route_controls`]. Found
    /// with [`follow_route_producer_position`].
    pub follow_route_controls: Vec<graph::GraphFollowRouteControlProducer>,
    /// The `live_values_superseded` count this plan's strip and route cell readers add to
    /// (#1312 D2): the session's own count when the host passed one to
    /// [`prepare_host_runtime_with_live_lanes`], else a fresh count this preparation made. Read
    /// it with [`LiveValuesSuperseded::get`].
    pub live_values_superseded: LiveValuesSuperseded,
    /// The writer of the plan's jump-length cell (#1639 D1), which preparation seeded with
    /// [`session_jump_lengths`]. Every prepared plan has the cell; `None` only in a handle set a
    /// caller built without a preparation.
    pub jump_lengths: Option<GraphJumpLengthProducer>,
}

impl HostLiveControlHandles {
    /// The live-control channel of one effect instance, by its session address (issue #1096).
    ///
    /// `None` when the track has no such console slot or insert, or when no channel was
    /// requested.
    pub fn effect_control_mut(
        &mut self,
        track_id: &str,
        address: LiveEffectAddress,
    ) -> Option<&mut EffectControlProducer> {
        let index = effect_producer_position(
            &self.effect_controls,
            |id| {
                strip_position(
                    self.strips.len(),
                    self.track_count,
                    |index| &*self.strips[index],
                    id,
                )
            },
            track_id,
            address,
        )?;
        self.effect_controls.get_mut(index)
    }
}

/// The position of the seven-word producer of route `id` in `producers`, which are in canonical
/// route-ID order ([`HostLiveControlHandles::follow_route_controls`]): a binary search, so it
/// allocates nothing.
#[must_use]
pub fn follow_route_producer_position(
    producers: &[graph::GraphFollowRouteControlProducer],
    id: &str,
) -> Option<usize> {
    producers
        .binary_search_by(|producer| (*producer.route_id).cmp(id))
        .ok()
}

/// The position of the seven-word producer that may take `record` for route `id`: the producer
/// of [`follow_route_producer_position`], when the record carries the automated lanes its plan
/// bound (flags bits 4-5, #1653 D5). Render takes them from its own cursors, so a record whose
/// bits differ is an internal fault: the commit refuses it before any write, and no ack precedes
/// a drop. Allocation-free.
#[must_use]
pub fn follow_route_producer_for(
    producers: &[graph::GraphFollowRouteControlProducer],
    id: &str,
    record: &graph::FollowRouteControlRecord,
) -> Option<usize> {
    follow_route_producer_position(producers, id)
        .filter(|&index| producers[index].automated() == record.flags().automated)
}

/// The position of strip `id` in the canonical strip order: the tracks, then the submixes, each
/// segment in ascending ID order, as [`HostLiveControlHandles::strips`] lists them (#1554 D6).
/// `id_at(i)` is the ID of the strip at position `i` of `count`, the first `track_count` of
/// which are tracks. Two binary searches, one per segment; allocation-free.
#[must_use]
pub fn strip_position<'s>(
    count: usize,
    track_count: usize,
    id_at: impl Fn(usize) -> &'s str,
    id: &str,
) -> Option<usize> {
    let search = |mut low: usize, mut high: usize| {
        while low < high {
            let middle = low + (high - low) / 2;
            match id_at(middle).cmp(id) {
                core::cmp::Ordering::Less => low = middle + 1,
                core::cmp::Ordering::Greater => high = middle,
                core::cmp::Ordering::Equal => return Some(middle),
            }
        }
        None
    };
    search(0, track_count.min(count)).or_else(|| search(track_count.min(count), count))
}

/// The position of the producer of the instance `address` of strip `strip_id` in `producers`,
/// which are sorted by `(strip position, address)` (#1554 D6; preparation sorts them).
/// `strip_rank` gives a strip's position in the strip order ([`strip_position`]).
#[must_use]
pub fn effect_producer_position(
    producers: &[EffectControlProducer],
    strip_rank: impl Fn(&str) -> Option<usize>,
    strip_id: &str,
    address: LiveEffectAddress,
) -> Option<usize> {
    let rank = strip_rank(strip_id)?;
    let index = producers.partition_point(|producer| {
        (
            strip_rank(&producer.track_id).unwrap_or(usize::MAX),
            producer.address,
        ) < (rank, address)
    });
    producers
        .get(index)
        .filter(|producer| &*producer.track_id == strip_id && producer.address == address)
        .map(|_| index)
}

/// Bytes a host that keeps the strip producers as one exactly sized boxed slice retains: the
/// slice of `strip_count` [`TrackControlProducer`]s plus each producer's `track_id`, whose text is
/// `strip_id_bytes` long in total (#1256 D3), plus each producer's own heap owners
/// (`producer_heap_bytes`: [`TrackControlProducer::retained_heap_bytes`] summed by the caller over
/// the built producers, #1916).
///
/// This is the mirror a host's pre-flight resource projection reads, as `control_table_bytes` is
/// for the source producers: a function of the producers' layout, so the host never spells it.
/// The rings the producers share with the plan are not here; builtins charges them in
/// `builtin_retained_payload_bytes`.
#[must_use]
pub fn strip_control_table_bytes(
    strip_count: usize,
    strip_id_bytes: usize,
    producer_heap_bytes: u64,
) -> Option<u64> {
    let table = core::alloc::Layout::array::<TrackControlProducer>(strip_count).ok()?;
    let ids = core::alloc::Layout::array::<u8>(strip_id_bytes).ok()?;
    u64::try_from(table.size())
        .ok()?
        .checked_add(u64::try_from(ids.size()).ok()?)?
        .checked_add(producer_heap_bytes)
}

/// One prepared session: the render plan, the control-side source set, and the resource report.
///
/// Field order is the drop order: the plan (which owns the source consumers) drops before the
/// control producers, so a producer never outlives its ring.
///
/// `Send` and deliberately **not** `Sync` (crate-level `# Host callback contract (V1)`): the plan is
/// rendered from exactly one thread and the sources are fed from exactly one thread, so a host that
/// could share this across threads could render from two at once. A shared reference is refused at
/// compile time. rustc reports E0277 (`Sync` is not implemented); the fence carries no code
/// because stable rustdoc does not check one.
///
/// ```compile_fail
/// fn requires<T: Sync>() {}
/// requires::<host_core::PreparedHost>();
/// ```
///
/// Moving it, on the other hand, is exactly how a host hands preparation to the render thread.
/// This twin differs from the doctest above in the bound only, so a renamed type turns it red
/// (issue #1422 D2):
///
/// ```
/// fn requires<T: Send>() {}
/// requires::<host_core::PreparedHost>();
/// ```
pub struct PreparedHost {
    /// The exclusive render plan. Move it to the render thread once and render there only.
    pub plan: PreparedRenderPlan,
    /// The control-side source producers. Feed PCM from one control thread.
    pub sources: SourceControlSet,
    /// Address-free resource facts about this preparation.
    pub report: HostPrepareReport,
    /// What this plan holds, for preparing its successor ([`SuccessorBase`]). Control-side plain
    /// data; never read on the render thread.
    pub inventory: PlanStateInventory,
    /// The control list of the compiled session (decision 17, C4), built before graph lowering
    /// consumes the prepared entries.
    #[cfg(feature = "control-provider")]
    pub control_catalog: crate::ControlCatalog,
}

impl PreparedHost {
    /// The strip IDs present in both this plan and the plan it was prepared to displace for which
    /// at least one fader, mute or matrix stage, console effect slot or insert does not carry, so
    /// it starts at rest (#1277 D4, D6; #1279 D1; #1280): a prepared or live value differs, the
    /// two plans bank the stage in different forms or attach different control kinds, or an
    /// effect has no banked lane of one kind in both plans. A strip whose effect chain changed (an
    /// insert added, removed or reordered, or a session console slot added, removed or moved) is
    /// listed too, even when its remaining owners carry (D15-9). A strip whose surviving insert
    /// (an insert present in both plans' chains) is not banked in both plans, or differs, is
    /// listed as well: its insert starts at rest on the swap. So is a strip a warm lead restarts
    /// whole, and, closed over the sidechain edges, a strip with an insert keyed from a listed
    /// strip's tap behind a restarted owner (#1324 D1). Sorted. Added strips are not listed, and
    /// neither is any strip of a plan prepared without a predecessor.
    /// *Duck-swap a strip whose state cannot continue across a plan swap* (#1324) consumes it.
    #[must_use]
    pub fn restarted_strips(&self) -> &[Box<str>] {
        self.inventory.restarted_strips()
    }

    /// `P`, how far this plan's carried nodes lead the plan it was prepared to displace (#1354
    /// D4): its [`WarmLead::lead_samples`] for a warm successor, 0 for every other plan.
    #[must_use]
    pub const fn lead_samples(&self) -> u64 {
        self.inventory.lead_samples
    }

    /// The bytes this plan's adoption block primes (#1354 D3, [`WarmLead::prime_bytes`]),
    /// counted over this plan's own compiled lines and carried sources; 0 for a plan that is not
    /// a warm successor with a lead.
    #[must_use]
    pub const fn prime_bytes(&self) -> u64 {
        self.inventory.prime_bytes
    }
}

impl PlanStateInventory {
    /// The strips this plan starts at rest when it displaces the plan it was prepared against:
    /// [`PreparedHost::restarted_strips`], kept with the inventory so the control plane can count
    /// them when render adopts the plan (#1768). Each strip is listed once, whichever carries
    /// restart it. Control-side plain data.
    #[must_use]
    pub fn restarted_strips(&self) -> &[Box<str>] {
        &self.restarted
    }

    /// Whether this plan's send join carries a route that follows an automated source mute in
    /// the plan it was prepared to displace into a route that follows none in this plan (a
    /// seven-word route into a six-word one, #1652 D3). The join flags every such carry by the
    /// route's kind alone: a carried route whose source strip automates a mute lane in the
    /// committed plan and automates none in this one (the rebuild removes the strip's last mute
    /// entry, or retargets it to another strip). `false` for a plan prepared without a
    /// predecessor and for a declared discontinuity, which carries nothing.
    ///
    /// **Conservative (#1652 U2a fold 2, MAJOR-A; #1713 removes it).** A six-word successor holds
    /// one ramp, so render may re-time a carried column's move: a join record composed with the
    /// heard curve reopens the route over the session mute length `J` instead of the record's own
    /// length, and two columns' moves fold into one ramp over the longer remaining length. The
    /// control plane completes such a rebuild as [`engine::realtime::CandidateOutcome::TransitionFallback`]
    /// so the watermark never reports it exact. It over-reports: a rebuild that carries a
    /// following route into a six-word one is flagged whether or not a column is moving at the
    /// swap.
    #[must_use]
    pub fn carries_a_following_route_into_a_six_word_route(&self) -> bool {
        self.follow_into_six_word
    }

    /// Whether this plan's send join carries a following route of the plan it was prepared to
    /// displace into a following route of its own that no longer hears one of the predecessor's
    /// curves: a source lane's mute curve, its own or a reaching VCA's (#1654, root ruling
    /// B19-R7). `false` for a plan prepared without a predecessor and for a
    /// declared discontinuity, which carries nothing.
    ///
    /// Render lands such a column at once over the removed curve's own length at the swap; the
    /// control plane folds that landing into its send-move window. The transition fallback reads
    /// the wider [`Self::carries_a_following_route_into_a_following_route`] (B19-R8).
    #[must_use]
    pub fn carries_a_following_route_that_loses_a_curve(&self) -> bool {
        self.follow_loses_a_curve
    }

    /// Whether this plan's send join carries a following route of the plan it was prepared to
    /// displace into a following route of its own (a seven-word route in both plans; #1654, root
    /// ruling B19-R8). `false` for a plan prepared without a predecessor and for a declared
    /// discontinuity, which carries nothing.
    ///
    /// **Conservative (B19-R8; #1718 removes it).** Render may land a carried column at once at
    /// the swap (a removed follow curve, or a column that composes otherwise), so a live send
    /// move still ramping on it is cut. The control plane completes such a
    /// rebuild as [`engine::realtime::CandidateOutcome::TransitionFallback`] while a live send move
    /// may still ramp. It over-reports: the join cannot tell which column lands at once or whether
    /// that column moves at the swap. It includes
    /// [`Self::carries_a_following_route_that_loses_a_curve`].
    #[must_use]
    pub fn carries_a_following_route_into_a_following_route(&self) -> bool {
        self.follow_into_following
    }

    /// The longest ramp of the records this plan's send join writes to its carried following
    /// routes (a live send move the successor starts at its first block, B19-R7), `0` when it
    /// writes none.
    #[must_use]
    pub fn following_route_retarget_ramp(&self) -> u32 {
        self.follow_retarget_ramp
    }
}

/// Set `host`'s source-read offset (#1396 deliverable 2): the plan's
/// ([`PreparedRenderPlan::set_source_read_offset`]) and its inventory row's together, so a
/// successor prepared from the inventory inherits it. Call it before the plan is published. The
/// one production writer of a nonzero offset is a warm successor's preparation (#1355).
#[cfg(feature = "test-support")]
pub fn test_only_set_source_read_offset(host: &mut PreparedHost, offset: u64) {
    host.plan.set_source_read_offset(offset);
    host.inventory.source_read_offset = offset;
}

/// One source of a prepared plan, as its successor's preparation sees it (issue #1272 D1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceInventoryRow {
    /// The session source ID.
    pub id: Box<str>,
    /// The configuration of the ring this plan renders the source from, whether it allocated
    /// the ring or carries it.
    pub ring: PcmSourceRingConfig,
    /// The source's index in the plan's graph source set.
    pub index: u32,
}

/// The state a prepared plan holds, recorded at the end of its preparation (issue #1272 D1).
///
/// A successor's preparation joins it with the committed and the new session models to decide
/// what carries ([`SuccessorBase`]). Plain data, `Send`, built and read on control threads only.
/// This slice records sources; later slices add one row list per state family.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanStateInventory {
    plan_identity: u64,
    /// Sorted by ID.
    sources: Box<[SourceInventoryRow]>,
    /// Sorted by strip ID.
    input_sections: Box<[InputSectionInventoryRow]>,
    /// Sorted by strip ID, then owner.
    strip_stages: Box<[StripStageInventoryRow]>,
    /// Sorted by strip ID, rack, then effect ID.
    effect_lanes: Box<[EffectLaneInventoryRow]>,
    /// One row per meter, in the order of the plan's meter readers
    /// ([`HostLiveControlHandles::meters`]), so a row's `entry` is its position (#1327 D1).
    meters: Box<[MeterInventoryRow]>,
    /// One row per effect instance that has observation taps, in the order of the plan's
    /// observation readers ([`HostLiveControlHandles::effect_observations`]), so a row's `entry`
    /// is its position (#1327 D1).
    observations: Box<[ObservationInventoryRow]>,
    /// The session rate and render quantum the plan runs at: a spectrum capture carries only
    /// between plans of equal shape (#1395 D1).
    sample_rate_hz: u32,
    quantum_frames: u32,
    /// One row per spectrum capture, in the order of the plan's captures, so a row's `entry` is
    /// its position (#1395 D1): a single capture's is 0, a collection's is its entry index.
    spectrum: Box<[SpectrumInventoryRow]>,
    /// Whether the captures are a collection (#1395b). A single capture and a collection never
    /// carry into each other: the control side holds one or the other, so a plan whose capture
    /// kind changed starts its captures fresh.
    spectrum_collection: bool,
    /// The captures this plan takes over from its predecessor (#1395 D3), built at preparation.
    spectrum_pairing: Box<[crate::spectrum::SpectrumPair]>,
    /// The meters this plan takes over from its predecessor (#1674 R1), built at preparation.
    meter_pairing: Box<[engine::realtime::ObserverCarry]>,
    /// One row per compensation line of the plan (#1283 D1), sorted by edge ID.
    delay_lines: Box<[graph::GraphDelayLine]>,
    /// One row per strip input alignment line of the plan (#1284 D1), sorted by strip ID.
    strip_delays: Box<[graph::GraphStripDelayLine]>,
    /// One row per live send of the plan (#1284 D2), sorted by route ID.
    live_routes: Box<[graph::GraphLiveRoute]>,
    /// Whether this plan's send join carries a following route of the plan it was prepared to
    /// displace into a six-word route of its own (#1652 U2a fold 2, MAJOR-A; see
    /// [`Self::carries_a_following_route_into_a_six_word_route`]). `false` for a plan prepared
    /// without a predecessor.
    follow_into_six_word: bool,
    /// Whether this plan's send join carries a following route into a following route (B19-R8;
    /// see [`Self::carries_a_following_route_into_a_following_route`]).
    follow_into_following: bool,
    /// Whether this plan's send join carries a following route into a following route that hears
    /// fewer curves (B19-R7; see [`Self::carries_a_following_route_that_loses_a_curve`]).
    follow_loses_a_curve: bool,
    /// The longest ramp of the join's following-route records (B19-R7; see
    /// [`Self::following_route_retarget_ramp`]).
    follow_retarget_ramp: u32,
    /// The strips this plan restarts relative to the plan it was prepared to displace (#1277 D6):
    /// present in both, with at least one owner that does not carry. Sorted by strip ID. Empty
    /// for a plan prepared without a predecessor.
    restarted: Box<[Box<str>]>,
    /// The strips and routes whose fade-in this plan's table holds, sorted by ID, then owner
    /// (issues #1392 D1, #1877 D1): "this plan armed it", not "the fade still waits", which only
    /// render knows.
    fade_arms: Box<[FadeArmInventoryRow]>,
    /// Which entries of this plan's fade table render has fired (issue #1392, root ruling of
    /// attempt 2): a successor drops the row of a fired entry, so rows never accumulate and a
    /// fired fade is neither carried nor re-armed. #1953b: a route row whose entry fired but has
    /// not settled is re-armed and carried, so its ramp continues.
    fade_watch: graph::GraphFadeArmWatch,
    /// The fading copies this plan compiles (#1953b), sorted by copy ID: each removed or
    /// re-pointed lane-less route's copy, its fade-out entry in the fade table, until that entry
    /// settles. A successor keeps each one that has not settled, carrying its op and its lines.
    route_copies: Box<[RouteCopyRow]>,
    /// The input arrival this plan renders at every node where it is nonzero (#1285 D2): the
    /// floored `max` of PDC, in samples, sorted by node ID. A successor is compiled with these as
    /// its input floors, so no surviving node's latency drops across a structural transaction.
    /// They are the floored values, not the natural ones, so a chain of successors keeps the
    /// first plan's timing until a declared discontinuity (#1323).
    input_floors: Box<[(graph::GraphNodeId, u64)]>,
    /// The natural input arrival of every node whose floor raised it (#1323 D3), sorted by node
    /// ID: its latest incoming arrival, below its floored arrival in `input_floors`. Empty when the
    /// plan runs at its natural timing.
    raised_arrivals: Box<[(graph::GraphNodeId, u64)]>,
    /// `ΣP`, how far the plan reads its sources ahead of its render clock (#1396 D2): its
    /// predecessor's for an ordinary successor, 0 for a first plan and at a declared
    /// discontinuity.
    source_read_offset: u64,
    /// The session's published seek generation (#1642 B16-R2): the one word every source
    /// producer, the timeline producer and every plan's source-set driver of the session share. A
    /// first plan allocates it; a successor takes its predecessor's from this row, so every plan of
    /// one session, and every consumer it carries or creates, is fenced by the same object.
    published_generation: PublishedGeneration,
    /// `P` of a warm successor (#1354 D4), 0 for every other plan. Not inherited.
    lead_samples: u64,
    /// The adoption block's prime bytes of a warm successor (#1354 D3), 0 for every other plan.
    prime_bytes: u64,
}

/// One fade-in entry of a prepared plan's table (issues #1392 D1, #1877 D1): the strip or route
/// and the entry's index, the index a successor's carry pair names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FadeArmInventoryRow {
    /// The strip ID (a track's or a submix's) or the route ID, as `owner` says.
    pub(crate) id: Box<str>,
    /// The entry's index in the plan's fade table.
    pub(crate) entry: u32,
    /// What the entry fades in: a strip's fader or a route.
    pub(crate) owner: FadeArmOwner,
}

/// One fading copy of a prepared plan (#1953b): the copy's route, under its copy ID, at the
/// values of the route it replaces, and its fade-out entry in the plan's fade table.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RouteCopyRow {
    /// The copy, as the plan compiled it: the removed route's source, destination and values,
    /// under an ID no route of the plan's model holds.
    pub(crate) route: session::Route,
    /// The copy's fade-out entry in the plan's fade table.
    pub(crate) entry: u32,
}

/// A compiled route's gain and matrix are finite (the session refuses a NaN), so equality is an
/// equivalence on every row a plan holds.
impl Eq for RouteCopyRow {}

impl RouteCopyRow {
    /// Heap bytes of the row's ID text: the copy's ID, its source strip's and its destination's.
    fn heap_bytes(&self) -> usize {
        let source = match &self.route.source {
            session::RouteSource::Track { track_id, .. } => track_id.as_str().len(),
            session::RouteSource::Submix { submix_id, .. } => submix_id.as_str().len(),
        };
        let destination = match &self.route.destination {
            session::RouteDestination::SubmixInput { submix_id } => submix_id.as_str().len(),
            session::RouteDestination::OutputInput { output_id } => output_id.as_str().len(),
        };
        self.route.id.as_str().len() + source + destination
    }
}

/// What a fade-in entry of [`FadeArmInventoryRow`] fades in (#1877 D1).
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum FadeArmOwner {
    /// A strip's fader lane (#1288, #1392).
    Strip,
    /// A route's live op, of either kind (#1363, #2011).
    Route,
}

/// One strip stage of a prepared plan, owner key `(strip ID, PostFader)` or `(strip ID,
/// PostMatrix)`, as its successor's preparation sees it (issue #1277 D1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StripStageInventoryRow {
    /// The strip ID: a track's or a submix's.
    pub(crate) id: Box<str>,
    /// Whose state the row holds.
    pub(crate) owner: graph::BuiltinStripOwner,
    /// The kind of bank the lane sits in: each owner has a bank of its own.
    pub(crate) stage: graph::BuiltinStripStage,
    /// Where the lane sits in this plan. Meaningful only to this plan's successor's carry
    /// program, which names it as the lane to copy from.
    pub(crate) location: graph::GraphLaneLocation,
    /// Whether this plan attached latest-target cells to the strip's fader, mute and matrix
    /// stages (the control kind of #1277 D4).
    pub(crate) live: bool,
}

/// One effect instance of a prepared plan that a swap can move, owner key `(strip ID, rack, effect
/// ID)` (issues #1279 D1, #1280, #1281): a lane of a bank stage, with or without a live control
/// lane, or a live-controlled per-node instance. A console slot's effect ID is its slot ID, and
/// its rack is `Simd1` in `pre_insert` and `Simd2` in `post_insert`; an insert's effect ID is its
/// insert ID, in `Dynamic`. A per-node effect with no control lane has no row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectLaneInventoryRow {
    /// The strip ID: a track's or a submix's.
    pub(crate) id: Box<str>,
    /// The rack the effect sits in.
    pub(crate) rack: graph::RackId,
    /// The effect ID: a console slot's slot ID.
    pub(crate) effect_id: Box<str>,
    /// Where the instance sits in this plan: a bank lane, or a per-node instance (#1281).
    /// Meaningful only to this plan's successor's carry program, which names it as the instance to
    /// copy from.
    pub(crate) location: graph::GraphEffectSite,
    /// What the successor's lane must share with this one for the state to move.
    pub(crate) layout: graph::EffectLaneLayout,
    /// The kind of stage and control lane the lane has: the successor's must be the same (#1280).
    pub(crate) kind: graph::EffectLaneKind,
}

/// One effect instance of a prepared plan whose observation taps a swap can carry, owner key `(strip
/// ID, rack, effect ID)` (#1327 D1): the instance's taps follow the instance, so the key is the
/// effect lane's.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ObservationInventoryRow {
    /// The reader set's index in the plan's observation readers.
    pub(crate) entry: usize,
    /// Where the effect instance sits in this plan: the key a successor's effect move names it by.
    /// Meaningful only to this plan's successor's carry program.
    pub(crate) location: graph::GraphEffectSite,
    /// How many taps (readers) the instance declares.
    pub(crate) taps: usize,
}

/// One spectrum capture of a prepared plan, owner key `(target, channels)`, as its successor's
/// preparation sees it (#1395 D1). The observer's binding handle is positional, so it is not part
/// of the key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SpectrumInventoryRow {
    /// The boundary the capture observes.
    pub(crate) target: crate::spectrum::SpectrumTarget,
    /// The planes the capture copies.
    pub(crate) channels: crate::spectrum::SpectrumChannels,
    /// The capture's index in the plan's captures.
    pub(crate) entry: usize,
    /// Where the capture's observer sits in this plan. Meaningful only to this plan's successor's
    /// carry program, which names it as the observer to take.
    pub(crate) location: graph::GraphObserverLocation,
}

/// A meter's [`MeterConfig`] as plain comparable data: equal exactly when the configurations are
/// equal word for word (#1327 D1), which `MeterConfig` itself cannot say because it holds a float.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct MeterConfigKey {
    period_frames: u32,
    peak_hold_frames: u32,
    peak_decay_bits: u32,
    queue_capacity: usize,
    reset_generation: u64,
}

impl MeterConfigKey {
    pub(crate) fn of(config: &MeterConfig) -> Self {
        Self {
            period_frames: config.period_frames.get(),
            peak_hold_frames: config.peak_hold_frames,
            peak_decay_bits: config.peak_decay_db_per_second.to_bits(),
            queue_capacity: config.queue_capacity.get(),
            reset_generation: config.reset_generation,
        }
    }
}

/// One meter of a prepared plan, owner key `(strip ID, tap, metric set)` with its configuration,
/// as its successor's preparation sees it (#1327 D1). Two meters of one key (a caller may select
/// the same tap twice) are told apart by their order, `entry`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MeterInventoryRow {
    /// The strip ID: a track's or a submix's.
    pub(crate) id: Box<str>,
    /// The chain boundary the meter observes.
    pub(crate) tap: MeterTap,
    /// The numeric groups the meter keeps.
    pub(crate) metrics: MeterMetricSet,
    /// The meter's configuration.
    pub(crate) config: MeterConfigKey,
    /// The meter's index in the plan's meter readers.
    pub(crate) entry: usize,
    /// Where the meter's observer sits in this plan. Meaningful only to this plan's successor's
    /// carry program, which names it as the observer to take.
    pub(crate) location: graph::GraphObserverLocation,
}

/// One banked strip input section of a prepared plan, owner key `(strip ID,
/// PostInputBuiltins)`, as its successor's preparation sees it (issue #1276 D1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct InputSectionInventoryRow {
    /// The strip ID: a track's or a submix's.
    pub(crate) id: Box<str>,
    /// Where the section's lane sits in this plan. Meaningful only to this plan's successor's
    /// carry program, which names it as the lane to copy from.
    pub(crate) location: graph::GraphLaneLocation,
    /// Whether this plan attached a live-control queue to the section.
    pub(crate) live: bool,
}

impl PlanStateInventory {
    /// The plan's process-unique graph identity (`graph::plan_identity`), which the successor's
    /// carry program names.
    #[must_use]
    pub const fn plan_identity(&self) -> u64 {
        self.plan_identity
    }

    /// One row per session source, sorted by source ID.
    #[must_use]
    pub fn sources(&self) -> &[SourceInventoryRow] {
        &self.sources
    }

    /// The row of source `id`, if the plan has that source.
    #[must_use]
    pub fn source(&self, id: &str) -> Option<&SourceInventoryRow> {
        self.sources
            .binary_search_by(|row| (*row.id).cmp(id))
            .ok()
            .map(|index| &self.sources[index])
    }

    /// Banked strip input sections, one row each (issue #1276).
    #[must_use]
    pub fn input_section_count(&self) -> usize {
        self.input_sections.len()
    }

    /// The row of strip `id`'s input section, if the plan banks one.
    #[must_use]
    pub(crate) fn input_section(&self, id: &str) -> Option<&InputSectionInventoryRow> {
        input_section_row(&self.input_sections, id)
    }

    /// Meter rows, one per meter reader (#1327).
    #[must_use]
    pub fn meter_count(&self) -> usize {
        self.meters.len()
    }

    /// Each meter reader's index, strip ID, tap and metrics, in reader order (#1588 D3): meter
    /// `i`'s window is strip `i` of a [`crate::MeterFrame`] the plan's delivery publishes, so a
    /// reader of the frame resolves a strip to its meter here, whatever selection the plan was
    /// prepared with.
    pub fn meter_targets(&self) -> impl Iterator<Item = (usize, &str, MeterTap, MeterMetricSet)> {
        self.meters
            .iter()
            .map(|row| (row.entry, &*row.id, row.tap, row.metrics))
    }

    /// Observation rows, one per effect observation reader set (#1327).
    #[must_use]
    pub fn observation_count(&self) -> usize {
        self.observations.len()
    }

    /// Spectrum capture rows, one per capture (#1395).
    #[must_use]
    pub fn spectrum_count(&self) -> usize {
        self.spectrum.len()
    }

    /// The captures this plan takes over from the plan it was prepared to displace (#1395 D1,
    /// D3): the pairs the control side applies at promotion with
    /// [`SpectrumService::adopt_successor`](crate::SpectrumService::adopt_successor), when the
    /// adoption carried. Built at preparation from each capture's target and channels, never from
    /// positions; it names exactly the observers the plan's swap block moves, so the swap block
    /// pairs nothing itself. Empty for a plan prepared without a predecessor, one whose
    /// predecessor has no equal capture, one of another sample rate or quantum, and one whose capture
    /// kind (single or collection) differs from its predecessor's. A collection's pairs are in the
    /// successor's entry order.
    #[must_use]
    pub fn spectrum_pairing(&self) -> &[crate::spectrum::SpectrumPair] {
        &self.spectrum_pairing
    }

    /// The strip meter readers this plan takes over from the plan it was prepared to displace
    /// (#1674 R1): the records the control side pairs its meter consumers by at promotion, with
    /// [`pair_carried_meters`], when the adoption carried. Built at preparation from the meter
    /// moves, so its rows equal the swap block's record's meter entries, in order: the observer
    /// carry is all or nothing and runs every meter move. It holds no spectrum or tap row; a tap's
    /// carry also depends on its effect's restore at the swap, so observation readers pair from the
    /// record's tap entries, never from a preparation-time list. Empty for a plan prepared without
    /// a predecessor or one that carries no meter.
    #[must_use]
    pub fn meter_pairing(&self) -> &[engine::realtime::ObserverCarry] {
        &self.meter_pairing
    }

    /// Compensation line rows, one per delayed edge of the plan (#1283).
    #[must_use]
    pub fn delay_line_count(&self) -> usize {
        self.delay_lines.len()
    }

    /// Compensation line rows, sorted by edge ID (#1283; read for a strip transition's `C`, #1325).
    #[must_use]
    pub fn delay_lines(&self) -> &[graph::GraphDelayLine] {
        &self.delay_lines
    }

    /// Strip input alignment line rows, one per strip that declares a `delay_samples` (#1284).
    #[must_use]
    pub fn strip_delay_count(&self) -> usize {
        self.strip_delays.len()
    }

    /// Live send rows, one per route into a submix with a live lane (#1284).
    #[must_use]
    pub fn live_route_count(&self) -> usize {
        self.live_routes.len()
    }

    /// The fading copies whose fade entry render has not settled (#1953b): the ones a successor
    /// keeps, so a phase 1 that ducks a copy's source strip waits for their lines.
    #[cfg(feature = "control-provider")]
    pub(crate) fn unsettled_route_copies(&self) -> impl Iterator<Item = &session::Route> {
        self.route_copies
            .iter()
            .filter(|row| !self.fade_watch.settled(row.entry))
            .map(|row| &row.route)
    }

    /// The input floors a successor of this plan is compiled with (#1285 D2): every node whose
    /// input arrival in this plan is nonzero, with that arrival in samples, sorted by node ID.
    #[must_use]
    pub fn input_floors(&self) -> &[(graph::GraphNodeId, u64)] {
        &self.input_floors
    }

    /// The natural input arrival of every node a floor raised in this plan (#1323 D3), sorted by
    /// node ID; each node's floored arrival is its row in [`Self::input_floors`].
    #[must_use]
    pub fn raised_arrivals(&self) -> &[(graph::GraphNodeId, u64)] {
        &self.raised_arrivals
    }

    /// Whether this plan runs behind its natural timing (#1323 D2): a floor raised some node's
    /// input arrival above its natural one, or the plan reads its sources ahead of its render
    /// clock (a [`Self::source_read_offset`] above 0, #1396 D2). A host-declared discontinuity
    /// rebuilds only such a plan. The offset term matters because a plan can lead with no raised
    /// floor: after a warm growth a later edit can make every natural arrival equal its floor.
    #[must_use]
    pub fn runs_behind_natural_timing(&self) -> bool {
        !self.raised_arrivals.is_empty() || self.source_read_offset > 0
    }

    /// `ΣP`, how far this plan reads its sources ahead of its render clock (#1396 D2): the
    /// [`PreparedRenderPlan::source_read_offset`] its preparation set.
    #[must_use]
    pub const fn source_read_offset(&self) -> u64 {
        self.source_read_offset
    }

    /// The base inventory of a successor at a host-declared discontinuity (#1323 D3): this
    /// inventory's identity, rate, quantum and source rows, and nothing else. A successor prepared
    /// against it has no floors and carries only its source consumers (ring, generation and read
    /// position); every other owner starts at rest.
    fn at_discontinuity(&self) -> Self {
        Self {
            plan_identity: self.plan_identity,
            sources: self.sources.clone(),
            input_sections: Box::default(),
            strip_stages: Box::default(),
            effect_lanes: Box::default(),
            meters: Box::default(),
            observations: Box::default(),
            sample_rate_hz: self.sample_rate_hz,
            quantum_frames: self.quantum_frames,
            spectrum: Box::default(),
            spectrum_collection: false,
            spectrum_pairing: Box::default(),
            meter_pairing: Box::default(),
            delay_lines: Box::default(),
            strip_delays: Box::default(),
            live_routes: Box::default(),
            follow_into_six_word: false,
            follow_into_following: false,
            follow_loses_a_curve: false,
            follow_retarget_ramp: 0,
            restarted: Box::default(),
            fade_arms: Box::default(),
            fade_watch: graph::GraphFadeArmWatch::default(),
            route_copies: Box::default(),
            input_floors: Box::default(),
            raised_arrivals: Box::default(),
            // #1396 D2: no read-ahead.
            source_read_offset: 0,
            // #1642 B16-R2: the same session.
            published_generation: self.published_generation.clone(),

            lead_samples: 0,
            prime_bytes: 0,
        }
    }

    /// An inventory that holds this inventory's input floors (#1285, deliverable 2) and its join
    /// rows, but no source, line or route row: a successor prepared against it and rendered
    /// without adopting a predecessor carries no state, so a test renders it from frame 0 as the
    /// fresh reference of a plan compiled with the same floors.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn test_only_floors_only(&self) -> Self {
        Self {
            plan_identity: self.plan_identity,
            sources: Box::default(),
            // The strip, input-section, effect and strip-delay rows stay, so a successor's joins
            // restart no strip it keeps and arm none (#1324 D3): rendered without adopting a
            // predecessor, it runs no move and plays as a fresh plan.
            input_sections: self.input_sections.clone(),
            strip_stages: self.strip_stages.clone(),
            effect_lanes: self.effect_lanes.clone(),
            meters: Box::default(),
            observations: Box::default(),
            sample_rate_hz: self.sample_rate_hz,
            quantum_frames: self.quantum_frames,
            spectrum: Box::default(),
            spectrum_collection: false,
            spectrum_pairing: Box::default(),
            meter_pairing: Box::default(),
            delay_lines: Box::default(),
            strip_delays: self.strip_delays.clone(),
            live_routes: Box::default(),
            follow_into_six_word: false,
            follow_into_following: false,
            follow_loses_a_curve: false,
            follow_retarget_ramp: 0,
            restarted: Box::default(),
            fade_arms: Box::default(),
            fade_watch: graph::GraphFadeArmWatch::default(),
            route_copies: Box::default(),
            input_floors: self.input_floors.clone(),
            raised_arrivals: self.raised_arrivals.clone(),
            source_read_offset: 0,
            // A fresh reference: a session of its own.
            published_generation: PublishedGeneration::new(SourceGeneration(1)),

            lead_samples: 0,
            prime_bytes: 0,
        }
    }

    /// [`Self::test_only_floors_only`] with `floors` as its input floors (#1402 D3): a successor
    /// prepared against it is compiled with these floors and carries no state, so a gate can
    /// prepare a plan with lead floors before #1354 computes them. `floors` is sorted by node ID,
    /// one row per node, as [`Self::input_floors`] is.
    #[cfg(feature = "test-support")]
    #[must_use]
    pub fn test_only_with_input_floors(&self, floors: &[(graph::GraphNodeId, u64)]) -> Self {
        Self {
            input_floors: floors.into(),
            ..self.test_only_floors_only()
        }
    }

    /// Strip stage rows, one per banked fader or matrix lane (issue #1277).
    #[must_use]
    pub fn strip_stage_count(&self) -> usize {
        self.strip_stages.len()
    }

    /// Banked effect lane rows, one per lane of an effect bank stage (issues #1279, #1280).
    #[must_use]
    pub fn effect_lane_count(&self) -> usize {
        self.effect_lanes.len()
    }

    /// The row of effect `effect_id` in `rack` of strip `id`, if the plan banks one.
    #[must_use]
    pub(crate) fn effect_lane(
        &self,
        id: &str,
        rack: graph::RackId,
        effect_id: &str,
    ) -> Option<&EffectLaneInventoryRow> {
        effect_lane_row(&self.effect_lanes, id, rack, effect_id)
    }

    /// The row of `owner`'s stage of strip `id`, if the plan banks one.
    #[must_use]
    pub(crate) fn strip_stage(
        &self,
        id: &str,
        owner: graph::BuiltinStripOwner,
    ) -> Option<&StripStageInventoryRow> {
        strip_stage_row(&self.strip_stages, id, owner)
    }

    /// Heap bytes this inventory retains: its row tables, their ID text and the session's
    /// published seek generation (#1642 B16-R2).
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        let ids: usize = self.sources.iter().map(|row| row.id.len()).sum::<usize>()
            + self
                .input_sections
                .iter()
                .map(|row| row.id.len())
                .sum::<usize>()
            + self
                .strip_stages
                .iter()
                .map(|row| row.id.len())
                .sum::<usize>()
            + self
                .effect_lanes
                .iter()
                .map(|row| row.id.len() + row.effect_id.len())
                .sum::<usize>()
            + self.meters.iter().map(|row| row.id.len()).sum::<usize>()
            + self
                .spectrum
                .iter()
                .map(|row| row.target.id().len())
                .sum::<usize>()
            + self
                .delay_lines
                .iter()
                .map(graph::GraphDelayLine::heap_bytes)
                .sum::<usize>()
            + self
                .strip_delays
                .iter()
                .map(graph::GraphStripDelayLine::heap_bytes)
                .sum::<usize>()
            + self
                .live_routes
                .iter()
                .map(graph::GraphLiveRoute::heap_bytes)
                .sum::<usize>()
            + self.restarted.iter().map(|id| id.len()).sum::<usize>()
            + self.fade_arms.iter().map(|row| row.id.len()).sum::<usize>()
            + self.fade_watch.heap_bytes()
            + self
                .route_copies
                .iter()
                .map(RouteCopyRow::heap_bytes)
                .sum::<usize>()
            + self
                .input_floors
                .iter()
                .chain(self.raised_arrivals.iter())
                .map(|(node, _)| graph_node_heap_bytes(node))
                .sum::<usize>();
        u64::try_from(
            core::mem::size_of_val::<[SourceInventoryRow]>(&self.sources)
                + core::mem::size_of_val::<[InputSectionInventoryRow]>(&self.input_sections)
                + core::mem::size_of_val::<[StripStageInventoryRow]>(&self.strip_stages)
                + core::mem::size_of_val::<[EffectLaneInventoryRow]>(&self.effect_lanes)
                + core::mem::size_of_val::<[MeterInventoryRow]>(&self.meters)
                + core::mem::size_of_val::<[ObservationInventoryRow]>(&self.observations)
                + core::mem::size_of_val::<[SpectrumInventoryRow]>(&self.spectrum)
                + core::mem::size_of_val::<[crate::spectrum::SpectrumPair]>(&self.spectrum_pairing)
                + core::mem::size_of_val::<[engine::realtime::ObserverCarry]>(&self.meter_pairing)
                + core::mem::size_of_val::<[graph::GraphDelayLine]>(&self.delay_lines)
                + core::mem::size_of_val::<[graph::GraphStripDelayLine]>(&self.strip_delays)
                + core::mem::size_of_val::<[graph::GraphLiveRoute]>(&self.live_routes)
                + core::mem::size_of_val::<[Box<str>]>(&self.restarted)
                + core::mem::size_of_val::<[FadeArmInventoryRow]>(&self.fade_arms)
                + core::mem::size_of_val::<[RouteCopyRow]>(&self.route_copies)
                + core::mem::size_of_val::<[(graph::GraphNodeId, u64)]>(&self.input_floors)
                + core::mem::size_of_val::<[(graph::GraphNodeId, u64)]>(&self.raised_arrivals)
                // #1642 B16-R2: the session's published generation, charged where it is held
                // across plans (every inventory of the session holds the one word).
                + PublishedGeneration::retained_bytes()
                + ids,
        )
        .unwrap_or(u64::MAX)
    }
}

/// Bytes of ID text a graph node ID owns on the heap.
fn graph_node_heap_bytes(node: &graph::GraphNodeId) -> usize {
    fn edge(id: &graph::GraphEdgeId) -> usize {
        match id {
            graph::GraphEdgeId::TrackMain { target } => graph_node_heap_bytes(target),
            graph::GraphEdgeId::RouteSource { route_id }
            | graph::GraphEdgeId::RouteDestination { route_id } => route_id.as_str().len(),
            graph::GraphEdgeId::EffectSidechain { effect, port } => {
                effect.track_id.as_str().len() + effect.effect_id.as_str().len() + port.len()
            }
        }
    }
    match node {
        graph::GraphNodeId::TrackStage { track_id, .. } => track_id.as_str().len(),
        graph::GraphNodeId::Effect(effect) => {
            effect.track_id.as_str().len() + effect.effect_id.as_str().len()
        }
        graph::GraphNodeId::Route { route_id } => route_id.as_str().len(),
        graph::GraphNodeId::Submix { submix_id } => submix_id.as_str().len(),
        graph::GraphNodeId::Output { output_id } => output_id.as_str().len(),
        graph::GraphNodeId::CompensationDelay { edge_id } => {
            core::mem::size_of::<graph::GraphEdgeId>() + edge(edge_id)
        }
    }
}

impl PlanStateInventory {
    /// The base inventory a replacement B of the withdrawn successor A is prepared from, this
    /// inventory being P0's, the plan render runs and A was prepared to displace (#1698, amending
    /// #1344 D1 and D2). Control-side plain data, built once per supersession.
    ///
    /// Every owner row is P0's own, so an owner B keeps unchanged from P0 carries from P0 as a
    /// direct P0 -> B swap carries it, whatever A did to it. A source row is kept only when A
    /// carries that source from P0 (A's plan holds it vacant, `can_prepare_source_seek` false on
    /// its index): the host feeds the newest committed session, and a source A added or
    /// redeclared restarted the host's feed at generation 1, frame 0 against A, so B, which
    /// declares it differently from A, restarts it the same way (`CONTROL_PROTOCOL_SEMANTICS.md`,
    /// "Sources across a structural transaction"). A source A created fresh and B keeps as A
    /// declares it moves by the donation ([`crate::check_donation`]), never through this base.
    ///
    /// Outside phase 1 of a strip removal: [`Self::across_withdrawn_in_phase_one`] with an empty
    /// overlay.
    #[must_use]
    pub fn across_withdrawn(
        &self,
        withdrawn: &PlanStateInventory,
        withdrawn_plan: &PreparedRenderPlan,
    ) -> Self {
        self.across_withdrawn_in_phase_one(withdrawn, withdrawn_plan, &NO_DUCK)
    }

    /// [`Self::across_withdrawn`] while P0's cells hold the duck overlay `duck` (#1325 D6): a
    /// source the overlay lists keeps its row too. A's transaction removed it, but P0 still
    /// renders it and its producer is in P0's set while phase 1 runs, so a B that restores it
    /// with the same declaration and ring configuration carries P0's ring (every PCM frame the
    /// host was acked for in phase 1 included), and the control plane moves P0's producer into
    /// B's set after the commit. A B that declares it otherwise starts it fresh.
    #[must_use]
    pub fn across_withdrawn_in_phase_one(
        &self,
        withdrawn: &PlanStateInventory,
        withdrawn_plan: &PreparedRenderPlan,
        duck: &DuckOverlay,
    ) -> Self {
        let mut base = self.clone();
        base.sources = self
            .sources
            .iter()
            .filter(|row| {
                duck.has_source(row.id.as_bytes())
                    || withdrawn.source(&row.id).is_some_and(|carried| {
                        !withdrawn_plan.can_prepare_source_seek(carried.index as usize)
                    })
            })
            .cloned()
            .collect();
        base
    }
}

/// The plan a successor is prepared to displace (issue #1272 D2, umbrella #1269 P1).
///
/// `committed` is the predecessor's base at the moment the successor is prepared. An input
/// section carries only when the transaction left its values equal between `committed` and the
/// model the successor is compiled from, and its prepared layout equal to the predecessor's
/// `inventory` row. A fader, mute or matrix stage (#1277) carries when no prepared value of it
/// differs, and its layout and control kind equal the inventory row's; a difference in its live
/// values is written to the successor's cells as a retarget stamped with `revision`, so the
/// stage sounds as "live edit, then structural edit" (D15-7). A source carries when its ID is in
/// both models, its declaration (content, channels, bit depth, frames) is equal, and its ring
/// configuration is the inventory row's; which tracks read it does not matter. A console effect
/// slot (#1279) or a surviving insert (#1280) carries when no prepared value of it differs, its
/// two plans hold it in an effect bank stage of one kind, and the prepared layouts of the two lanes
/// are equal; a difference in its live values (a bypass, a `Block`-rate parameter) is written to
/// the successor's cells as a retarget stamped with `revision`, as a strip's is. A strip whose
/// effect chain changed restarts even if its remaining slots carry.
///
/// **Across a withdrawn plan (#1344 D2, amended by #1698).** When the plan render runs (P0) has a
/// committed successor A that was taken back before render adopted it, A's replacement B is
/// prepared against P0 with this same entry: `inventory` is
/// [`PlanStateInventory::across_withdrawn`] of P0's inventory and A, and `committed` is P0's
/// committed model. Every row of that inventory is a P0 row, so the joins run unchanged and B
/// carries from P0 every owner it leaves unchanged against P0's model, as a direct P0 -> B swap
/// does, including an owner A restarted or removed. Sources carry only as A carried them: a
/// source A added, redeclared or removed is fresh in B. The rings A created fresh move to B by a
/// checked donation ([`crate::check_donation`]), never through this base.
#[derive(Clone, Copy, Debug)]
pub struct SuccessorBase<'a> {
    /// The predecessor's [`PreparedHost::inventory`].
    pub inventory: &'a PlanStateInventory,
    /// The predecessor's base: the model it was prepared from plus every live record pushed to
    /// it (#1277 D2), normalized ([`CompiledSession::normalized_model`]): strips are looked up in
    /// it by ID.
    pub committed: &'a SessionModel,
    /// `r`, the revision the successor commits as (#1277 D10). Its carry retargets are stamped
    /// with it on the successor's own gate pin ([`PreparedRenderPlan::stamp_held_gate`]), and the
    /// host that commits the successor publishes it on the successor's gate handle. Precondition:
    /// `1 <= revision <= GATE_REVISION_MAX`; a preparation refuses any other with
    /// `host.successor.revision` before it allocates. It has no neutral value.
    pub revision: u64,
    /// The ramp lengths the transaction's edits gave their rows (#1394 D2): a carry retarget
    /// takes its length from here as the classifier's record of the same edit would, else from
    /// the session's `control_smoothing`. Transaction metadata; [`EditRamps::new`] is "none".
    pub edit_ramps: &'a EditRamps,
    /// The monitor overlay of the predecessor's base and of this successor (decision 17, C8,
    /// #1559 D9). [`MonitorBase::NONE`] for a host without a solo.
    pub monitor: MonitorBase<'a>,
    /// The taps the host has armed, as the successor renders under them (#1584 D6a): the
    /// predecessor's bindings after the transaction, so a binding whose effect the transaction
    /// removed is already gone. Each successor tap named here is armed at preparation and opens
    /// its first window at its first rendered block; a tap that carries from the predecessor
    /// takes the predecessor's own state instead, and is never restarted by this.
    /// [`NO_OBSERVATION`] for a host without subscriptions.
    pub observation: &'a controls::ObservationBindings,
    /// The host declared a discontinuity (#1323 D3): the successor need not continue the
    /// predecessor's output. It is compiled with no input floors, so it runs at its natural
    /// timing, and its join carries only the source consumers (ring, generation and read
    /// position), so no frame is repeated or skipped; every other owner starts at rest, with no
    /// continuity constraint. `false` is the neutral value: a successor continues its
    /// predecessor.
    pub discontinuity: bool,
    /// The successor is a monitor rebuild at the unchanged committed revision (#1594 D6, R1):
    /// only the session's spectrum taps differ from the predecessor, so `committed` is the model
    /// the successor is prepared from and `revision` is the committed revision, which may be 0
    /// (`0..=GATE_REVISION_MAX`). Such a successor has no carry retarget to stamp; a preparation
    /// that finds one refuses with `host.successor.monitor`. `false` is the neutral value.
    pub unchanged_revision: bool,
    /// The duck overlay the predecessor's cells hold (#1325 D6): what phase 1 of a pending strip
    /// removal ramped to `+0.0` on the predecessor. The preparation composes it into `committed`
    /// ([`DuckOverlay::apply_to`]: each overlay strip's channels and each overlay route muted),
    /// so the carry joins the predecessor's values as its cells hold them, and a strip a newer
    /// transaction restores in phase 1 carries with a retarget from its ducked level instead of
    /// keeping it. Only a base across a withdrawn candidate holds one: the overlay lives while a
    /// successor waits. [`DuckOverlay::new`] (empty) is the neutral value.
    pub duck: &'a DuckOverlay,
    /// The lead [`warm_lead`] returned for this edit (#1354 D4): the successor is compiled with
    /// its floors (every carried node at its predecessor arrival plus `P`, every restarted node at
    /// its arrival, no floor on an added node), every strip of [`WarmLead::restart_whole`] starts
    /// its nodes before the fader and its fader at rest and is listed in
    /// [`PreparedHost::restarted_strips`], and a source-claim line that grew by exactly `P` keeps
    /// its strip carried (its fill is #1355's). A lead that is not a whole number of quanta is
    /// refused with `host.successor.warm_lead`; a successor whose join or arrivals differ from
    /// the lead's is refused with `host.successor.warm_invariant`. `None` is the neutral value: an
    /// ordinary successor.
    pub warm: Option<&'a WarmLead>,
    /// The strips the transition fallback restarts (#1397 D2 step 1): strip IDs, sorted, the duck
    /// set `transition::grown_strips` computed from the ordinary compile (a `control-provider`
    /// item, so not linked from here). Preparation restarts each one it holds as it restarts a
    /// strip its carry join restarts (#1324 D2-D3: every owner before the fader and the fader
    /// start at rest, and the fader is armed), and lists it in
    /// [`PreparedHost::restarted_strips`]. An added strip in it is ignored. Empty, the neutral
    /// value, for every other successor.
    pub forced_restart: &'a [String],
}

impl<'a> SuccessorBase<'a> {
    /// The base of a successor of the plan whose inventory is `inventory`, prepared from
    /// `committed` and committing as `revision` (#1323 D3, #1277 D10). Every other field takes
    /// its neutral value: no edit ramps, no monitor overlay ([`MonitorBase::NONE`]), no armed
    /// tap ([`NO_OBSERVATION`]) and no discontinuity. A field added later extends this with its
    /// own neutral value; a base that needs another value sets it with struct update syntax.
    #[must_use]
    pub fn new(
        inventory: &'a PlanStateInventory,
        committed: &'a SessionModel,
        revision: u64,
    ) -> Self {
        Self {
            inventory,
            committed,
            revision,
            edit_ramps: &NO_EDIT_RAMPS,
            monitor: MonitorBase::NONE,
            observation: &NO_OBSERVATION,
            discontinuity: false,
            unchanged_revision: false,
            duck: &NO_DUCK,
            warm: None,
            forced_restart: &[],
        }
    }
}

/// The overlay of a base outside phase 1: nothing ducks ([`SuccessorBase::new`]).
static NO_DUCK: DuckOverlay = DuckOverlay::new();

/// The bindings that arm nothing, for a [`SuccessorBase`] of a host without subscriptions.
pub static NO_OBSERVATION: controls::ObservationBindings = controls::ObservationBindings::new();

/// The edit ramps of a successor whose transaction gave no row a length ([`SuccessorBase::new`]).
static NO_EDIT_RAMPS: EditRamps = EditRamps::new();

/// The host-local monitor values (solo) a successor is prepared under (decision 17, C8, #1559).
///
/// The successor bakes `controls::effective(model, next)`, so it renders the solo from its first
/// block and a strip added while another is soloed is solo-muted; the carry joins the effective
/// values of `(committed model, committed)` and `(model, next)`, so a carried strip's solo change
/// is a retarget with the length `ramp` gives. The overlay is not part of the session: a snapshot
/// never holds it.
#[derive(Clone, Copy, Debug)]
pub struct MonitorBase<'a> {
    /// The overlay the predecessor's cells hold.
    pub committed: &'a controls::MonitorOverlay,
    /// The overlay the successor renders under: `committed` plus the transaction's writes.
    pub next: &'a controls::MonitorOverlay,
    /// The ramp length, in samples, of a record that only the overlay change produced; `None` is
    /// the session's `mute_ms` ([`LiveRampRow::Solo`]).
    pub ramp: Option<u32>,
}

/// The overlay that solos nothing, for [`MonitorBase::NONE`].
static NO_OVERLAY: controls::MonitorOverlay = controls::MonitorOverlay::new();

impl MonitorBase<'static> {
    /// No solo before and none after: the base of a host without a monitor overlay.
    pub const NONE: Self = Self {
        committed: &NO_OVERLAY,
        next: &NO_OVERLAY,
        ramp: None,
    };
}

/// The strips a warm successor restarts, as preparation and [`warm_lead`] read them (#1354a
/// rulings 1-2): `join`, the ordinary successor's carry join restarts (its nodes before the fader
/// and its fader, and its routes from a tap before the fader), and `whole`, the strips the lead
/// restarts whole (every node of the strip).
struct WarmRestart<'a> {
    join: BTreeSet<&'a str>,
    whole: BTreeSet<&'a str>,
    /// The successor's routes by ID: a route node belongs to its source strip.
    routes: BTreeMap<&'a str, &'a session::Route>,
}

impl<'a> WarmRestart<'a> {
    fn new(
        model: &'a SessionModel,
        join: impl IntoIterator<Item = &'a str>,
        whole: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        Self {
            join: join.into_iter().collect(),
            whole: whole.into_iter().collect(),
            routes: model
                .routes
                .iter()
                .map(|route| (route.id.as_str(), route))
                .collect(),
        }
    }

    /// The strip `node` belongs to ([`strip_owner`]).
    fn owner<'n>(&'n self, node: &'n GraphNodeId) -> Option<(&'n str, bool)> {
        strip_owner(&self.routes, node)
    }

    /// Whether `node` is in `R`: a node of a strip restarted whole, or a node before the fader
    /// (or the fader) of a strip the join restarts.
    fn restarts(&self, node: &GraphNodeId) -> bool {
        self.owner(node).is_some_and(|(strip, before_fader)| {
            self.whole.contains(strip) || (before_fader && self.join.contains(strip))
        })
    }
}

/// The strip `node` belongs to, and whether it sits before the strip's fader or is its fader
/// (a strip's `PostMatrix` stage and its routes from a `post_fader` or `post_pan` tap do not).
/// A route node belongs to its source strip, looked up in `routes` (the successor's routes by
/// ID); the output belongs to no strip.
pub(crate) fn strip_owner<'n>(
    routes: &'n BTreeMap<&str, &session::Route>,
    node: &'n GraphNodeId,
) -> Option<(&'n str, bool)> {
    match node {
        GraphNodeId::TrackStage { track_id, stage } => {
            Some((track_id.as_str(), *stage != TrackStage::PostMatrix))
        }
        GraphNodeId::Effect(effect) => Some((effect.track_id.as_str(), true)),
        GraphNodeId::Route { route_id } => {
            let route = routes.get(route_id.as_str())?;
            let (strip, tap) = match &route.source {
                session::RouteSource::Track { track_id, tap } => (track_id, *tap),
                session::RouteSource::Submix { submix_id, tap } => (submix_id, *tap),
            };
            Some((strip.as_str(), tap_before_fader(tap)))
        }
        GraphNodeId::Submix { .. }
        | GraphNodeId::Output { .. }
        | GraphNodeId::CompensationDelay { .. } => None,
    }
}

/// The whole-strip restart filter (#1354a decision 2, widened to every restarted strip by #1324
/// D2): the strips whose owners before the fader and whose fader a successor starts at rest. A
/// strip in [`PreparedHost::restarted_strips`] is duck-swapped, so no carry may leave a hole in
/// front of a restarted owner: its input section, its effects, its fader, the compensation lines
/// on its chain before its matrix and into its effects' sidechains, and its routes from a tap
/// before the fader (their lines and their live lanes) all restart. Its matrix, its post-fader
/// and post-pan routes and their lines carry as the join says; they carry the ducked tail.
struct StripRestart<'a> {
    strips: BTreeSet<&'a str>,
    /// The successor's routes by ID: a route node belongs to its source strip.
    routes: BTreeMap<&'a str, &'a session::Route>,
}

impl<'a> StripRestart<'a> {
    fn new(model: &'a SessionModel, strips: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            strips: strips.into_iter().collect(),
            routes: model
                .routes
                .iter()
                .map(|route| (route.id.as_str(), route))
                .collect(),
        }
    }

    /// Whether `id` names a restarted strip.
    fn contains(&self, id: &str) -> bool {
        self.strips.contains(id)
    }

    /// Whether the route `id` leaves a restarted strip from a tap before its fader: its line and
    /// its live lane restart.
    fn restarts_route(&self, id: &str) -> bool {
        let Some(route_id) = StableGraphId::parse(id) else {
            return false;
        };
        strip_owner(&self.routes, &GraphNodeId::Route { route_id })
            .is_some_and(|(strip, before_fader)| before_fader && self.contains(strip))
    }

    /// Whether the filter drops this compensation line's carry: a line on a restarted strip's
    /// chain before its matrix, into one of its effects' sidechains, or on its routes from a tap
    /// before the fader. Matrix and post-fader lines still carry.
    fn drops_line(&self, line: &graph::GraphDelayLine) -> bool {
        match &line.edge {
            graph::GraphEdgeId::TrackMain { target } => strip_owner(&self.routes, target)
                .is_some_and(|(strip, before_fader)| before_fader && self.contains(strip)),
            graph::GraphEdgeId::EffectSidechain { effect, .. } => {
                self.contains(effect.track_id.as_str())
            }
            graph::GraphEdgeId::RouteSource { route_id }
            | graph::GraphEdgeId::RouteDestination { route_id } => {
                self.restarts_route(route_id.as_str())
            }
        }
    }
}

/// Whether a send tap reads a strip before its fader.
fn tap_before_fader(tap: session::SendTap) -> bool {
    !matches!(tap, session::SendTap::PostFader | session::SendTap::PostPan)
}

/// The graph nodes a plan compiled from `model` has (#1354a): every strip's seven stages and
/// its effects, every route and every output. The compiler emits no other node kind
/// (`graph-compiler` `compile_graph`; a compensation line is not a node of the schedule).
pub(crate) fn session_graph_nodes(model: &SessionModel) -> BTreeSet<GraphNodeId> {
    const STAGES: [TrackStage; 7] = [
        TrackStage::Input,
        TrackStage::PostInputBuiltins,
        TrackStage::PostSimd1,
        TrackStage::PostDynamic,
        TrackStage::PostSimd2PreFader,
        TrackStage::PostFader,
        TrackStage::PostMatrix,
    ];
    let mut nodes = BTreeSet::new();
    for strip in model.strips() {
        let Some(id) = StableGraphId::parse(strip.id.as_str()) else {
            continue;
        };
        for stage in STAGES {
            nodes.insert(GraphNodeId::TrackStage {
                track_id: id.clone(),
                stage,
            });
        }
        let lowered = model.lower_strip(&strip);
        let racks = [
            graph::RackId::Simd1,
            graph::RackId::Dynamic,
            graph::RackId::Simd2,
        ];
        for (rack, effects) in racks.into_iter().zip(lowered.in_chain_order()) {
            for effect in effects {
                if let Some(effect_id) = StableGraphId::parse(effect.id.as_str()) {
                    nodes.insert(GraphNodeId::Effect(graph::EffectNodeId {
                        track_id: id.clone(),
                        rack,
                        effect_id,
                    }));
                }
            }
        }
    }
    for route in &model.routes {
        if let Some(route_id) = StableGraphId::parse(route.id.as_str()) {
            nodes.insert(GraphNodeId::Route { route_id });
        }
    }
    for output in &model.outputs {
        if let Some(output_id) = StableGraphId::parse(output.id.as_str()) {
            nodes.insert(GraphNodeId::Output { output_id });
        }
    }
    nodes
}

/// `node`'s row in `rows`, sorted by node ID, else 0: a node absent from an arrival list arrives
/// at 0.
fn arrival_in(rows: &[(GraphNodeId, u64)], node: &GraphNodeId) -> u64 {
    rows.binary_search_by(|(row, _)| row.cmp(node))
        .map_or(0, |index| rows[index].1)
}

/// D2 step 5's floors (#1354 D2, D4): `a(n) + lead` on every predecessor node outside `R`,
/// `a(n)` on every node in `R`, and none on an added node. `a` is the predecessor's floors. A
/// floor that names no node of the successor is ignored by the compiler.
fn warm_floors(
    a: &[(GraphNodeId, u64)],
    predecessor: &BTreeSet<GraphNodeId>,
    restart: &WarmRestart<'_>,
    lead: u64,
) -> Result<BTreeMap<GraphNodeId, effect_contract::LatencySamples>, PrepareDiagnostics> {
    let mut floors = BTreeMap::new();
    for node in predecessor {
        let arrival = arrival_in(a, node);
        let floor = if restart.restarts(node) {
            arrival
        } else {
            arrival
                .checked_add(lead)
                .ok_or_else(|| resource("host.resource.arithmetic"))?
        };
        if floor > 0 {
            floors.insert(node.clone(), effect_contract::LatencySamples(floor));
        }
    }
    Ok(floors)
}

/// The builtin caps a preparation compiles with.
fn builtin_compile_caps(caps: &HostPrepareCaps) -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: caps.maximum_builtin_retained_bytes,
        maximum_total_retained_payload_bytes: caps.maximum_builtin_retained_bytes,
        maximum_total_meter_items: caps.maximum_meter_items,
        maximum_total_meter_bytes: caps.maximum_meter_bytes,
        maximum_single_allocation_bytes: caps.maximum_named_allocation_bytes,
        maximum_meter_streams: caps.maximum_meter_streams,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

/// The graph caps a preparation compiles with.
fn graph_compile_caps(caps: &HostPrepareCaps) -> GraphCompileCaps {
    GraphCompileCaps {
        maximum_nodes: u64::MAX,
        maximum_edges: u64::MAX,
        maximum_schedule_items: u64::MAX,
        maximum_dependency_levels: u64::MAX,
        maximum_audio_buffer_samples: u64::MAX,
        maximum_delay_samples_per_edge: u64::MAX,
        maximum_total_delay_samples: u64::MAX,
        maximum_graph_bytes: caps.maximum_graph_session_plus_plan_bytes,
        maximum_plan_bytes: caps.maximum_graph_session_plus_plan_bytes,
        maximum_single_allocation_bytes: caps.maximum_named_allocation_bytes,
        maximum_finite_tail_samples: u64::MAX,
    }
}

fn graph_compile_diagnostics(
    failure: &graph_compiler::GraphBuiltinsCompileFailure,
) -> PrepareDiagnostics {
    graph_diagnostics(&failure.diagnostics)
}

/// A graph compile's or projection's diagnostics as a preparation's.
fn graph_diagnostics(diagnostics: &graph::GraphDiagnosticSet) -> PrepareDiagnostics {
    PrepareDiagnostics::new(
        PrepareRejection::Graph,
        diagnostic_lines(
            diagnostics
                .diagnostics()
                .iter()
                .map(|diagnostic| (diagnostic.code, &diagnostic.path)),
        ),
    )
}

/// A compile of `compiled` with `floors` for its arrivals and schedule only (#1354a ruling 1):
/// no live control, no meter, fixed gains. A node's latency, the graph's edges and its schedule
/// do not depend on them.
fn compile_for_arrivals(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    overlay: &controls::MonitorOverlay,
    floors: BTreeMap<GraphNodeId, effect_contract::LatencySamples>,
) -> Result<graph_compiler::PreparedGraphBuiltinsArtifact, PrepareDiagnostics> {
    // Issue #2194: the effect state inside the caps before any instance is allocated.
    check_effect_projection(project_effects(compiled)?, caps)?;
    let registry =
        launch_native_effect_registry().map_err(|_| effect_failure("host.effect.registry"))?;
    let effects = prepare_native_session_effects(
        compiled,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: caps.maximum_effect_state_bytes,
            maximum_scratch_bytes: caps.maximum_effect_scratch_bytes,
            maximum_automation_spans_per_block: u32::MAX,
        },
    )
    .map_err(effect_diagnostics)?;
    let effective = {
        let _canonical = lane::CanonicalFpEnv::enter();
        controls::effective(compiled.normalized_model(), overlay)
    };
    let builtins = prepare_session_builtins_with_live_controls(
        compiled,
        &effective,
        &[],
        &[],
        builtin_compile_caps(caps),
    )
    .map_err(|diagnostics| {
        PrepareDiagnostics::new(
            PrepareRejection::Builtin,
            diagnostic_lines(
                diagnostics
                    .0
                    .iter()
                    .map(|diagnostic| (diagnostic.code, &diagnostic.path)),
            ),
        )
    })?;
    GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
        plan_id: 1,
        effects,
        builtins,
        input_floors: floors,
        caps: graph_compile_caps(caps),
        gain_liveness: graph_compiler::GainLiveness::FIXED,
    })
    .map_err(|failure| graph_compile_diagnostics(&failure))
}

/// The arrivals a compiled artifact reports, as `(node, samples)` rows sorted by node ID.
fn artifact_arrivals(
    artifact: &graph_compiler::PreparedGraphBuiltinsArtifact,
) -> Vec<(GraphNodeId, u64)> {
    artifact
        .report()
        .input_arrivals
        .iter()
        .map(|(node, arrival)| (node.clone(), arrival.0))
        .collect()
}

/// The first carried node of `graph`, in its schedule order, whose arrival is not its
/// predecessor arrival plus `lead` (C1, #1354 D2 step 6), with the arrival it has.
fn first_late_node<'g>(
    graph: &'g graph::PreparedGraphPlan,
    arrivals: &[(GraphNodeId, u64)],
    a: &[(GraphNodeId, u64)],
    predecessor: &BTreeSet<GraphNodeId>,
    restart: &WarmRestart<'_>,
    lead: u64,
) -> Option<&'g GraphNodeId> {
    graph.sequential_schedule.iter().find(|node| {
        predecessor.contains(*node)
            && !restart.restarts(node)
            && arrival_in(arrivals, node) != arrival_in(a, node).saturating_add(lead)
    })
}

/// The invariant error of a warm successor (#1354 D2 step 6, D4).
fn warm_invariant() -> PrepareDiagnostics {
    PrepareDiagnostics::fixed(PrepareRejection::Graph, "host.successor.warm_invariant")
}

/// The source rings a successor of `base` carries under #1272 D3, with the channels of each: the
/// ID is in both models, the declaration is unchanged and the ring the successor would allocate
/// at `caps` is the one the predecessor renders from.
fn carried_rings<'b>(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    base: &SuccessorBase<'b>,
) -> Vec<(&'b SourceInventoryRow, u32)> {
    compiled
        .normalized_model()
        .sources
        .iter()
        .filter_map(|source| {
            let ring = source_ring_config(compiled, caps, source);
            let row = base.inventory.source(source.id.as_str())?;
            let committed = base
                .committed
                .sources
                .iter()
                .find(|committed| committed.id == source.id)?;
            source_carries(row, committed, source, ring).then_some((row, ring.channel_count))
        })
        .collect()
}

/// The ring a preparation at `caps` allocates for `source`.
fn source_ring_config(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    source: &session::Source,
) -> PcmSourceRingConfig {
    PcmSourceRingConfig {
        channel_count: u32::from(source.channels),
        quantum_frames: compiled.quantum(),
        frame_capacity: u64::from(caps.source_ring_frames),
        initial_generation: SourceGeneration(1),
        // #2205, root ruling 2: the one runway and deadline, fixed at prepare.
        seek_runway_blocks: source_seek_runway_blocks(
            compiled.sample_rate().0,
            compiled.quantum().0,
            caps.source_ring_frames,
        ),
    }
}

/// The #1272 D3 carry rule for a source present in both models: its declaration is unchanged
/// (`committed` against `source`) and `ring`, the ring the successor would allocate, is the one
/// the predecessor's `row` renders from.
fn source_carries(
    row: &SourceInventoryRow,
    committed: &session::Source,
    source: &session::Source,
    ring: PcmSourceRingConfig,
) -> bool {
    *committed == *source && row.ring == ring
}

/// D3's prime byte count (#1355 D5 narrows it to the lines the fill writes): `4 · (lead ·
/// channels + Σ words)`, with `words` the words the prime's fill writes into each track line of
/// `lines` (the successor's compiled strip delays, claim lines included) on a source of `carried`:
/// a lane that grew by the lead takes its whole ring (the predecessor's pending words, then the
/// lead), and a lane that moves takes the lead, up to its ring. An added track's line, a line on a
/// source that does not carry, and a restarted lane take nothing. `before` is the predecessor's
/// strip delay rows, sorted by strip ID. `None` on overflow. D3's L3 term is
/// [`sidechain_fill_words`]'s.
fn prime_bytes_of(
    lead: u64,
    channels: u64,
    committed: &SessionModel,
    model: &SessionModel,
    carried: &BTreeSet<&str>,
    before: &[graph::GraphStripDelayLine],
    lines: &[graph::PreparedTrackDelay],
) -> Option<u64> {
    let mut words = 0_u64;
    for join in join_strip_delays(committed, model, carried) {
        if !(join.track && join.same_source) {
            continue;
        }
        let node = claim_node(join.id);
        let Some(after) = lines.iter().find(|line| line.node == node) else {
            continue;
        };
        let after = [after.left_samples, after.right_samples];
        let before = before
            .binary_search_by(|row| (*row.strip).cmp(join.id))
            .ok()
            .map(|index| before[index].samples);
        let (moved, grown) = strip_delay_lanes(join.equal, before, after, lead);
        for (lane, bit) in [graph::STRIP_DELAY_LEFT, graph::STRIP_DELAY_RIGHT]
            .into_iter()
            .enumerate()
        {
            let ring = u64::from(after[lane]);
            let written = if grown & bit != 0 {
                ring
            } else if moved & bit != 0 {
                ring.min(lead)
            } else {
                0
            };
            words = words.checked_add(written)?;
        }
    }
    lead.checked_mul(channels)?
        .checked_add(words)?
        .checked_mul(4)
}

/// Whether `compiled`, a structural edit of the plan `base` describes, gets a warm successor
/// (#1354 D2): the only place `P` is computed. Control thread only; it compiles the successor's
/// graph once per pass and changes no queue (D5).
///
/// `ordinary` is the inventory of the ordinary successor of `base` prepared from `compiled` with
/// the host's own entry (#1354a ruling 1): its [`PlanStateInventory::restarted_strips`] starts
/// `R`'s join part and its [`PlanStateInventory::input_floors`] are step 3's arrivals. A host
/// prepares it anyway, as the answer to [`WarmDecision::Ordinary`].
///
/// Step 2 (#1354b) runs on each pass's compile, before C1 is checked: the strips the lead's strip
/// delay join restarts (a track re-pointed to a source that does not carry, whose claim line
/// grows) and #1324 D1's sidechain closure over `R` (the strips restarted whole included) join
/// `R`'s join part. On the lead's compile the closure's submix `input`-tap rule is step 6's M1
/// rule: the sidechain line changes length exactly when the stage misses `a(n) + P`. A strip that
/// joins `R` only on a lead's compile makes the answer `Warm` even when `P` falls to 0 (an ordinary
/// rebuild that duck-swaps it).
///
/// At a declared discontinuity nothing leads, so the answer is [`WarmDecision::Ordinary`].
///
/// It reads `base.committed`, not the model with [`SuccessorBase::duck`] applied: the overlay
/// changes only mutes, which nothing here reads. A later reader of mutes must use the overlaid
/// model, as preparation does.
///
/// # Errors
///
/// A compile refusal of the successor, or an invariant error when the step-6 walk reaches no
/// carried node (unreachable by the D2 proof; a debug assertion fires).
pub fn warm_lead(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    base: SuccessorBase<'_>,
    ordinary: &PlanStateInventory,
    config: &WarmConfig,
) -> Result<WarmDecision, PrepareDiagnostics> {
    if base.discontinuity {
        return Ok(WarmDecision::Ordinary);
    }
    caps.validate_shape(compiled)?;
    let _canonical = lane::CanonicalFpEnv::enter();
    let quantum = u64::from(compiled.quantum().0);
    let model = compiled.normalized_model();
    let predecessor = session_graph_nodes(base.committed);
    let a = base.inventory.input_floors();
    let ordinary_arrivals = ordinary.input_floors();
    let predecessor_strips: BTreeSet<&str> = base
        .committed
        .strips()
        .map(|strip| strip.id.as_str())
        .collect();
    let rings = carried_rings(compiled, caps, &base);
    let carried: BTreeSet<&str> = rings.iter().map(|(row, _)| &*row.id).collect();
    let delay_joins = join_strip_delays(base.committed, model, &carried);
    // `R`'s join part: the ordinary successor's restart set, then every strip step 2 and the
    // lead's strip delay join add. A strip enters it, and `whole`, at most once.
    let mut joined: BTreeSet<&str> = ordinary
        .restarted_strips()
        .iter()
        .filter_map(|id| predecessor_strips.get(&**id).copied())
        .collect();
    let ordinary_set = joined.clone();
    let mut whole: BTreeSet<&str> = BTreeSet::new();
    // Each pass adds a strip to `joined` or to `whole`, or returns, so the loop ends within twice
    // the strip count.
    let passes = 2 * predecessor_strips.len() + 1;
    let mut found = None;
    for _ in 0..passes {
        let restart = WarmRestart::new(model, joined.iter().copied(), whole.iter().copied());
        // Step 3: `Δ` over C, from the ordinary successor's arrivals.
        let delta = predecessor
            .iter()
            .filter(|node| !restart.restarts(node))
            .map(|node| arrival_in(ordinary_arrivals, node).saturating_sub(arrival_in(a, node)))
            .max()
            .unwrap_or(0);
        // Step 4, with step 2 at `P = 0`: the lead's compile is the ordinary successor's.
        if delta == 0 {
            let before = &base.inventory.strip_delays;
            let after = |id: &str| {
                ordinary
                    .strip_delays
                    .binary_search_by(|row| (*row.strip).cmp(id))
                    .ok()
                    .map(|index| ordinary.strip_delays[index].samples)
            };
            let added = close_warm_restart(
                &base,
                model,
                &ordinary.delay_lines,
                delay_restarted(&delay_joins, before, after, 0),
                &joined,
                &whole,
            );
            if !added.is_empty() {
                joined.extend(added);
                continue;
            }
            return Ok(if whole.is_empty() && joined == ordinary_set {
                WarmDecision::Ordinary
            } else {
                WarmDecision::Warm(WarmLead {
                    lead_samples: 0,
                    restart_whole: whole.iter().map(|id| (*id).to_owned()).collect(),
                    restarted: joined.iter().map(|id| Box::from(*id)).collect(),
                    prime_bytes: 0,
                })
            });
        }
        // Step 5.
        let lead = delta
            .div_ceil(quantum)
            .checked_mul(quantum)
            .ok_or_else(|| resource("host.resource.arithmetic"))?;
        let floors = warm_floors(a, &predecessor, &restart, lead)?;
        let artifact = compile_for_arrivals(compiled, caps, base.monitor.next, floors)?;
        let arrivals = artifact_arrivals(&artifact);
        let graph = artifact.graph();
        // Step 2 on the lead's compile: the strips its strip delay join restarts (a track's claim
        // line that grows on a source that does not carry, #1354b), and the sidechain closure
        // with the submix `input`-tap rule (M1) over its lines, join `R` before C1 is checked.
        let lines = compiled_sidechain_lines(graph);
        let after = |id: &str| {
            let node = claim_node(id);
            graph
                .track_delays()
                .iter()
                .find(|row| row.node == node)
                .map(|row| [row.left_samples, row.right_samples])
        };
        let added = close_warm_restart(
            &base,
            model,
            &lines,
            delay_restarted(&delay_joins, &base.inventory.strip_delays, after, lead),
            &joined,
            &whole,
        );
        if !added.is_empty() {
            joined.extend(added);
            continue;
        }
        // Step 6, C1.
        let Some(late) = first_late_node(graph, &arrivals, a, &predecessor, &restart, lead) else {
            found = Some((lead, artifact, lines));
            break;
        };
        let reached = walk_back(
            graph,
            &arrivals,
            late,
            arrival_in(a, late) + lead,
            &predecessor,
            &restart,
        );
        let added: Vec<&str> = reached
            .into_iter()
            .filter(|strip| !whole.contains(strip.as_str()))
            .filter_map(|strip| predecessor_strips.get(strip.as_str()).copied())
            .collect();
        if added.is_empty() {
            debug_assert!(
                false,
                "#1354 D2 step 6: the walk from {late:?} reaches no carried node"
            );
            return Err(warm_invariant());
        }
        whole.extend(added);
        // Misaligned: every predecessor strip that reaches the output restarts whole.
        let reaching = strips_reaching_output(graph, &restart);
        let reaching: Vec<&str> = reaching
            .into_iter()
            .filter_map(|strip| predecessor_strips.get(strip).copied())
            .collect();
        if !reaching.is_empty() && reaching.iter().all(|strip| whole.contains(strip)) {
            return Ok(WarmDecision::Unavailable(WarmUnavailable::Misaligned));
        }
    }
    let Some((lead, artifact, lines)) = found else {
        debug_assert!(
            false,
            "#1354 D2: the walk ends within twice the strip count"
        );
        return Err(warm_invariant());
    };
    // Step 7, C3: the lead.
    if base
        .inventory
        .source_read_offset()
        .checked_add(lead)
        .is_none_or(|total| total > config.p_max)
    {
        return Ok(WarmDecision::Unavailable(WarmUnavailable::LeadBound));
    }
    let stall = u64::from(stall_ring_frames(
        compiled.sample_rate().0,
        compiled.quantum().0,
    ));
    if rings
        .iter()
        .any(|(row, _)| row.ring.frame_capacity.saturating_sub(stall) < lead + quantum)
    {
        return Ok(WarmDecision::Unavailable(WarmUnavailable::LeadBound));
    }
    // Step 8, C3: the prime, its claim lines (L2) and its `input`-tap sidechain lines (L3).
    let channels = rings.iter().map(|(_, channels)| u64::from(*channels)).sum();
    let restarted: BTreeSet<&str> = joined.union(&whole).copied().collect();
    let fills = sidechain_fills(&base, &lines, &delay_joins, &restarted);
    let prime_bytes = prime_bytes_of(
        lead,
        channels,
        base.committed,
        model,
        &carried,
        &base.inventory.strip_delays,
        artifact.graph().track_delays(),
    )
    .and_then(|bytes| {
        sidechain_fill_words(&base, &lines, &fills, lead)?
            .checked_mul(4)?
            .checked_add(bytes)
    })
    .ok_or_else(|| resource("host.resource.arithmetic"))?;
    if prime_bytes > config.prime_bytes_max {
        return Ok(WarmDecision::Unavailable(WarmUnavailable::PrimeBudget));
    }
    // Step 9.
    Ok(WarmDecision::Warm(WarmLead {
        lead_samples: lead,
        restart_whole: whole.iter().map(|id| (*id).to_owned()).collect(),
        restarted: joined.iter().map(|id| Box::from(*id)).collect(),
        prime_bytes,
    }))
}

/// D2 step 2 on one compile of the warm successor (#1354b): the strips that join `R`'s join part.
/// The compile's strip delay join restarts `delay_restarted` (sorted); #1324 D1's sidechain
/// closure ([`close_over_sidechains`], the rule preparation applies after its joins) then runs
/// over `R` (`joined`, `whole` and `delay_restarted`) with the compile's sidechain lines `after`
/// against the predecessor's. On the lead's compile a submix `input`-tap line changes length
/// exactly when the stage arrives at `a(n) + g` with `g ≠ P`, so the closure is step 6's M1 rule
/// too. Returns the strips of the predecessor it adds, outside `joined` and `whole`.
fn close_warm_restart<'m>(
    base: &SuccessorBase<'_>,
    model: &'m SessionModel,
    after: &[graph::GraphDelayLine],
    delay_restarted: Vec<Box<str>>,
    joined: &BTreeSet<&str>,
    whole: &BTreeSet<&str>,
) -> Vec<&'m str> {
    let mut set: Vec<Box<str>> = joined
        .iter()
        .chain(whole)
        .map(|id| Box::from(*id))
        .chain(delay_restarted.iter().cloned())
        .collect();
    set.sort_unstable();
    set.dedup();
    close_over_sidechains(
        base.committed,
        model,
        &base.inventory.delay_lines,
        after,
        &delay_restarted,
        &mut set,
    );
    model
        .strips()
        .map(|strip| strip.id.as_str())
        .filter(|id| set.binary_search_by(|held| (**held).cmp(id)).is_ok())
        .filter(|id| !joined.contains(id) && !whole.contains(id))
        .filter(|id| committed_has_strip(base.committed, id))
        .collect()
}

/// The strips the strip delay join restarts ([`strip_delay_restarts`]), sorted: `before` is the
/// predecessor's strip delay rows (sorted by strip ID) and `after` a strip's lane rings in the
/// successor, `None` when it holds no line.
fn delay_restarted(
    joins: &[StripDelayJoin<'_>],
    before: &[graph::GraphStripDelayLine],
    after: impl Fn(&str) -> Option<[u32; 2]>,
    lead: u64,
) -> Vec<Box<str>> {
    let mut restarted: Vec<Box<str>> = joins
        .iter()
        .filter(|join| {
            let earlier = before
                .binary_search_by(|row| (*row.strip).cmp(join.id))
                .ok()
                .map(|index| before[index].samples);
            strip_delay_restarts(join, earlier, after(join.id), lead)
        })
        .map(|join| Box::from(join.id))
        .collect();
    restarted.sort_unstable();
    restarted
}

/// The sidechain compensation lines of a compiled, unbound graph, from its inserted delays (the
/// lines its program lowers, one per delayed edge), each with its edge's endpoints. `line` is the
/// row's index in the result: a bound plan's lines are read after bind
/// ([`graph::take_compensation_lines`]), so these rows serve [`warm_lead`]'s closure and its L3
/// count only.
fn compiled_sidechain_lines(graph: &graph::PreparedGraphPlan) -> Vec<graph::GraphDelayLine> {
    graph
        .inserted_delays
        .iter()
        .filter(|delay| matches!(delay.edge_id, graph::GraphEdgeId::EffectSidechain { .. }))
        .filter_map(|delay| {
            let edge = graph
                .spec
                .edges
                .iter()
                .find(|edge| edge.id == delay.edge_id)?;
            Some((edge, delay.samples.0))
        })
        .zip(0_u32..)
        .map(|((edge, samples), line)| graph::GraphDelayLine {
            edge: edge.id.clone(),
            source: edge.source.node.clone(),
            destination: edge.destination.node.clone(),
            line,
            samples,
        })
        .collect()
}

/// D3's L3 term, in words (#1354b): what the prime's fill writes into each `input`-tap sidechain
/// line of `fills` (lines of `lines`). Each channel of a line of length `λ'` takes the head of the
/// stream the predecessor's consumer would have heard, up to `λ'`: the predecessor's pending words
/// of the same edge, then the source track's claim pending on that lane, then the `lead` words of
/// the prime (#1355 D5). `None` on overflow.
fn sidechain_fill_words(
    base: &SuccessorBase<'_>,
    lines: &[graph::GraphDelayLine],
    fills: &[graph::GraphSidechainFill],
    lead: u64,
) -> Option<u64> {
    let mut words = 0_u64;
    for fill in fills {
        let length = lines.iter().find(|line| line.line == fill.line)?.samples;
        let edge = fill.predecessor.map_or(0, |line| {
            base.inventory
                .delay_lines
                .iter()
                .find(|earlier| earlier.line == line)
                .map_or(0, |earlier| earlier.samples)
        });
        let claim = fill.claim_line.map_or([0; 2], |line| {
            base.inventory
                .strip_delays
                .iter()
                .find(|row| row.line == line)
                .map_or([0; 2], |row| row.samples)
        });
        for pending in claim {
            let available = edge.checked_add(u64::from(pending))?.checked_add(lead)?;
            words = words.checked_add(length.min(available))?;
        }
    }
    Some(words)
}

/// D2 step 6's walk from the late carried node `late`, whose lead arrival is `target`: back from
/// each input of `late` that arrives after `target`, through restarted and added nodes only,
/// along every edge kind (chain, route and sidechain), following from each such node the inputs
/// that hold its arrival. Returns the strips of the carried nodes it reaches (a route node counts
/// as its source strip).
fn walk_back<'g>(
    graph: &'g graph::PreparedGraphPlan,
    arrivals: &[(GraphNodeId, u64)],
    late: &'g GraphNodeId,
    target: u64,
    predecessor: &BTreeSet<GraphNodeId>,
    restart: &WarmRestart<'_>,
) -> BTreeSet<String> {
    let latency: BTreeMap<&GraphNodeId, u64> = graph
        .spec
        .nodes
        .iter()
        .map(|node| (&node.id, node.latency.0))
        .collect();
    let leaves = |node: &GraphNodeId| {
        arrival_in(arrivals, node).saturating_add(latency.get(node).copied().unwrap_or(0))
    };
    let mut incoming: BTreeMap<&GraphNodeId, Vec<&'g GraphNodeId>> = BTreeMap::new();
    for edge in &graph.spec.edges {
        incoming
            .entry(&edge.destination.node)
            .or_default()
            .push(&edge.source.node);
    }
    let inputs = |node: &GraphNodeId| incoming.get(node).cloned().unwrap_or_default();
    let mut reached = BTreeSet::new();
    let mut visited: BTreeSet<&GraphNodeId> = BTreeSet::new();
    let mut stack: Vec<&GraphNodeId> = Vec::new();
    let mut visit = |node: &'g GraphNodeId, stack: &mut Vec<&'g GraphNodeId>| {
        if predecessor.contains(node) && !restart.restarts(node) {
            if let Some((strip, _)) = restart.owner(node) {
                reached.insert(strip.to_owned());
            }
        } else if visited.insert(node) {
            stack.push(node);
        }
    };
    for input in inputs(late) {
        if leaves(input) > target {
            visit(input, &mut stack);
        }
    }
    while let Some(node) = stack.pop() {
        let arrival = arrival_in(arrivals, node);
        for input in inputs(node) {
            if leaves(input) == arrival {
                visit(input, &mut stack);
            }
        }
    }
    reached
}

/// The strips with a node from which the output is reachable in `graph`.
fn strips_reaching_output<'g>(
    graph: &'g graph::PreparedGraphPlan,
    restart: &'g WarmRestart<'_>,
) -> BTreeSet<&'g str> {
    let mut reaches: BTreeSet<&GraphNodeId> = graph
        .spec
        .nodes
        .iter()
        .filter(|node| matches!(node.id, GraphNodeId::Output { .. }))
        .map(|node| &node.id)
        .collect();
    for node in graph.sequential_schedule.iter().rev() {
        if graph
            .spec
            .edges
            .iter()
            .any(|edge| edge.source.node == *node && reaches.contains(&edge.destination.node))
        {
            reaches.insert(node);
        }
    }
    reaches
        .into_iter()
        .filter_map(|node| restart.owner(node).map(|(strip, _)| strip))
        .collect()
}

impl core::fmt::Debug for PreparedHost {
    /// Address-free: neither the plan nor the producers are printable, so only the report and the
    /// source count appear.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("PreparedHost")
            .field("sources", &self.sources.len())
            .field("report", &self.report)
            .finish_non_exhaustive()
    }
}

/// Parse one session JSON document.
pub fn parse_host_session(document: &str) -> Result<SessionModel, PrepareDiagnostics> {
    parse_session_json(document).map_err(session_diagnostics)
}

fn session_diagnostics(value: session::DiagnosticSet) -> PrepareDiagnostics {
    PrepareDiagnostics::new(
        PrepareRejection::Session,
        diagnostic_lines(
            value
                .diagnostics()
                .iter()
                .map(|diagnostic| (diagnostic.code.as_str(), &diagnostic.path)),
        ),
    )
}

fn effect_diagnostics(value: effect_compiler::EffectDiagnosticSet) -> PrepareDiagnostics {
    PrepareDiagnostics::new(
        PrepareRejection::Effect,
        diagnostic_lines(
            value
                .0
                .iter()
                .map(|diagnostic| (diagnostic.code, &diagnostic.path)),
        ),
    )
}

/// Parse and compile one session JSON document under this host's caps.
///
/// A host that also needs the transactional `SessionStore` (from `protocol`) (the C ABI host
/// does, for the control protocol) calls [`parse_host_session`] and
/// [`HostPrepareCaps::compile_caps`] instead and builds the store itself: the facade never depends
/// on the control protocol.
pub fn compile_host_session(
    document: &str,
    caps: &HostPrepareCaps,
) -> Result<CompiledSession, PrepareDiagnostics> {
    let model = parse_host_session(document)?;
    let compile_caps = caps.compile_caps(model.sources.len())?;
    compile_host_model(&model, compile_caps)
}

/// Compile an already parsed session model under explicit compiler caps.
///
/// Boot v1 parses once, derives the physical host shape from that model, and then compiles the
/// same model. Keeping the diagnostic mapping here prevents that path from growing a second
/// session-compiler adapter.
pub fn compile_host_model(
    model: &SessionModel,
    caps: CompileCaps,
) -> Result<CompiledSession, PrepareDiagnostics> {
    compile_session(model, caps).map_err(session_diagnostics)
}

/// Parse, compile and prepare one session in a single call.
///
/// Returns the compiled session alongside the prepared host so a host that wants to answer session
/// queries later can retain it; a host that does not simply drops it.
pub fn prepare_host_session(
    document: &str,
    caps: &HostPrepareCaps,
) -> Result<(CompiledSession, PreparedHost), PrepareDiagnostics> {
    let compiled = compile_host_session(document, caps)?;
    let prepared = prepare_host_runtime(&compiled, caps)?;
    Ok((compiled, prepared))
}

/// Parse, compile and prepare one session with live controls attached (issue #137 D1/D2).
pub fn prepare_host_session_with_live_controls(
    document: &str,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
) -> Result<(CompiledSession, PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let compiled = compile_host_session(document, caps)?;
    let (prepared, handles) =
        prepare_host_runtime_with_live_controls(&compiled, caps, live_controls)?;
    Ok((compiled, prepared, handles))
}

/// Prepare the render plan and source control set for an already compiled session.
///
/// This is the whole shared pipeline: counts, shape, source rings, effects, builtins, the graph
/// compile, the source binding, the identity bindings and the resource report, in that fixed order.
/// A rejection always names the first rule the session broke.
pub fn prepare_host_runtime(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
) -> Result<PreparedHost, PrepareDiagnostics> {
    let (prepared, handles) = prepare_host_runtime_with_live_controls_policy(
        compiled,
        caps,
        None,
        Backend::current(),
        None,
    )?;
    debug_assert!(handles.strip_controls.is_empty() && handles.meters.is_empty());
    Ok(prepared)
}

/// Prepare the render plan, source control set and live-control handles for a compiled session.
///
/// Issue #137 D1/D2. The live-control halves are prepared inside the same transaction as the plan,
/// so a session that cannot carry the requested live controls is rejected before anything is
/// published: there are no partially attached live controls. Requesting nothing is exactly
/// [`prepare_host_runtime`].
#[allow(clippy::too_many_lines)]
pub fn prepare_host_runtime_with_live_controls(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    prepare_host_runtime_with_live_controls_policy(
        compiled,
        caps,
        Some(live_controls),
        Backend::current(),
        None,
    )
}

/// Prepare a host whose live controls attach only the selected lanes (issue #1254 D2), with
/// `Concurrent` delivery like [`prepare_host_runtime_with_live_controls`].
///
/// Every strip always gets its fader/mute and
/// matrix/pan lanes; `lanes` decides whether it also gets the input lane, whether each effect
/// instance gets one, and whether each route into a submix gets one. [`HostLiveLanes::ALL`] is
/// exactly [`prepare_host_runtime_with_live_controls`]. [`HostLiveLanes::FADER_AND_MATRIX`] leaves
/// every strip's builtin tail at its chain's own value (only a live input lane can enable a
/// filter), attaches no effect or route lane, and keeps every route's fold. Observation taps ride
/// the effect lanes, so `observation_taps > 0` with `effects: false` is refused with
/// `host.observation.live_controls`.
///
/// `superseded` is the session's `live_values_superseded` count (#1312 D2): every strip and route
/// cell reader of this plan, and of every plan the session prepares, adds to it. The plan does not
/// charge it, because the host makes it once and charges it once. The entries that take no count
/// make a fresh one for their plan, charge it to that plan's graph row, and return it in
/// [`HostLiveControlHandles::live_values_superseded`].
pub fn prepare_host_runtime_with_live_lanes(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    lanes: HostLiveLanes,
    superseded: &LiveValuesSuperseded,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let (prepared, handles, _) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        None,
        Backend::current(),
        None,
        None,
        lanes,
        Some(superseded),
    )?;
    Ok((prepared, handles))
}

/// [`prepare_host_runtime_with_live_lanes`] for a successor of a running plan (issues #1053 and
/// #1269): the selected live lanes attach exactly as there, and every source `base` left
/// unchanged is prepared vacant for the carry program exactly as in
/// [`prepare_host_runtime_successor`].
pub fn prepare_host_runtime_with_live_lanes_successor(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    lanes: HostLiveLanes,
    base: SuccessorBase<'_>,
    superseded: &LiveValuesSuperseded,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let (prepared, handles, _) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        None,
        Backend::current(),
        None,
        Some(base),
        lanes,
        Some(superseded),
    )?;
    Ok((prepared, handles))
}

/// Prepare a host with the selected live lanes and the session's spectrum observers (#1593 D3):
/// the one entry a session's preparer calls for its first plan (`successor` `None`) and for every
/// successor (`Some`), with or without spectrum.
///
/// `spectrum` `None` prepares exactly [`prepare_host_runtime_with_live_lanes`] or
/// [`prepare_host_runtime_with_live_lanes_successor`] and returns no captures. `Some` prepares
/// its observers into the same transaction, sized, validated and allocated here, on the control
/// thread, and returns their captures; a successor's capture whose key `successor` has carries
/// its observer across the swap (#1395). A target the plan does not have is refused
/// `host.spectrum.target`. `superseded` is the session's `live_values_superseded` count, as in
/// [`prepare_host_runtime_with_live_lanes`].
pub fn prepare_host_runtime_with_live_lanes_and_spectrum(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    lanes: HostLiveLanes,
    spectrum: Option<&SpectrumPreparation>,
    successor: Option<SuccessorBase<'_>>,
    superseded: &LiveValuesSuperseded,
) -> Result<
    (
        PreparedHost,
        HostLiveControlHandles,
        Option<PreparedSpectrumCaptures>,
    ),
    PrepareDiagnostics,
> {
    prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        None,
        Backend::current(),
        spectrum,
        successor,
        lanes,
        Some(superseded),
    )
}

/// Prepare a successor of a running plan (issue #1272 D2).
///
/// Exactly [`prepare_host_runtime`], except that every source the transaction left unchanged
/// ([`SuccessorBase`]) is prepared vacant: the successor allocates no ring for it, and the plan's
/// installed carry program moves the predecessor's ring in at the swap block, with its queued
/// PCM, its generation and its read position. Its producer moves with
/// [`SourceControlSet::adopt_persisting`], which the host calls after its last fallible step.
/// The successor must displace exactly the plan `base.inventory` describes.
pub fn prepare_host_runtime_successor(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    base: SuccessorBase<'_>,
) -> Result<PreparedHost, PrepareDiagnostics> {
    let (prepared, handles) = prepare_host_runtime_with_live_controls_policy(
        compiled,
        caps,
        None,
        Backend::current(),
        Some(base),
    )?;
    debug_assert!(handles.strip_controls.is_empty() && handles.meters.is_empty());
    Ok(prepared)
}

/// [`prepare_host_runtime_with_live_controls`] for a successor of a running plan: see
/// [`prepare_host_runtime_successor`].
pub fn prepare_host_runtime_with_live_controls_successor(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    base: SuccessorBase<'_>,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    prepare_host_runtime_with_live_controls_policy(
        compiled,
        caps,
        Some(live_controls),
        Backend::current(),
        Some(base),
    )
}

/// Test-support width seam (issue #1272 D2): [`prepare_host_runtime_with_live_controls`] on an
/// explicit bank width, so width tests can live in `tests/`. Production uses
/// [`Backend::current`].
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_prepare_host_runtime_with_live_controls_on(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    backend: Backend,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    prepare_host_runtime_with_live_controls_policy(
        compiled,
        caps,
        Some(live_controls),
        backend,
        None,
    )
}

/// Test-support width seam (issue #1272 D2): [`prepare_host_runtime_with_live_controls_successor`]
/// on an explicit bank width.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_prepare_host_runtime_with_live_controls_successor_on(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    base: SuccessorBase<'_>,
    backend: Backend,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    prepare_host_runtime_with_live_controls_policy(
        compiled,
        caps,
        Some(live_controls),
        backend,
        Some(base),
    )
}

/// Test-support width seam (#1327): [`prepare_host_runtime_with_selected_meters`], or its
/// successor form with `base`, on an explicit bank width.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_prepare_host_runtime_with_selected_meters_on(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    meters: &[HostMeterRequest],
    base: Option<SuccessorBase<'_>>,
    backend: Backend,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let (prepared, handles, _) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        Some(meters),
        backend,
        None,
        base,
        HostLiveLanes::ALL,
        None,
    )?;
    Ok((prepared, handles))
}

/// Test-support width seam: [`prepare_host_runtime`] on an explicit bank width, with no live
/// controls.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_prepare_host_runtime_on(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    backend: Backend,
) -> Result<PreparedHost, PrepareDiagnostics> {
    prepare_host_runtime_with_live_controls_policy(compiled, caps, None, backend, None)
        .map(|(prepared, _)| prepared)
}

/// Test-support width seam: [`prepare_host_runtime_successor`] on an explicit bank width, with no
/// live controls.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_prepare_host_runtime_successor_on(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    base: SuccessorBase<'_>,
    backend: Backend,
) -> Result<PreparedHost, PrepareDiagnostics> {
    prepare_host_runtime_with_live_controls_policy(compiled, caps, None, backend, Some(base))
        .map(|(prepared, _)| prepared)
}

/// Test-only preparation seam for exercising the scalar lowering against the native bank.
/// Production callers remain pinned to [`Backend::current`].
#[cfg(test)]
pub(crate) fn prepare_host_runtime_with_live_controls_backend(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    backend: Backend,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    prepare_host_runtime_with_live_controls_policy(
        compiled,
        caps,
        Some(live_controls),
        backend,
        None,
    )
}

/// Prepare a host with exactly the caller-selected meter observers (issue #1401 D2): a producer
/// may push a record while another thread renders.
///
/// `superseded` is the session's `live_values_superseded` count, as in
/// [`prepare_host_runtime_with_live_lanes`].
pub fn prepare_host_runtime_with_selected_meters(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    meters: &[HostMeterRequest],
    lanes: HostLiveLanes,
    superseded: &LiveValuesSuperseded,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let (prepared, handles, _) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        Some(meters),
        Backend::current(),
        None,
        None,
        lanes,
        Some(superseded),
    )?;
    Ok((prepared, handles))
}

/// [`prepare_host_runtime_with_selected_meters`] for a successor of a running plan: see
/// [`prepare_host_runtime_successor`].
pub fn prepare_host_runtime_with_selected_meters_successor(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    meters: &[HostMeterRequest],
    lanes: HostLiveLanes,
    superseded: &LiveValuesSuperseded,
    base: SuccessorBase<'_>,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let (prepared, handles, _) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        Some(meters),
        Backend::current(),
        None,
        Some(base),
        lanes,
        Some(superseded),
    )?;
    Ok((prepared, handles))
}

#[allow(clippy::too_many_lines)]
fn prepare_host_runtime_with_live_controls_policy(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: Option<&HostLiveControlRequest>,
    backend: Backend,
    successor: Option<SuccessorBase<'_>>,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let (prepared, handles, _) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        live_controls,
        None,
        backend,
        None,
        successor,
        HostLiveLanes::ALL,
        None,
    )?;
    Ok((prepared, handles))
}

/// Prepare the normal host path with one selected, fixed-size spectrum observer.
pub fn prepare_host_runtime_with_spectrum(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    request: &SpectrumCaptureRequest,
) -> Result<(PreparedHost, SpectrumCapture), PrepareDiagnostics> {
    let (prepared, handles, capture) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        None,
        None,
        Backend::current(),
        Some(&SpectrumPreparation::Single(request.clone())),
        None,
        HostLiveLanes::ALL,
        None,
    )?;
    debug_assert!(handles.strip_controls.is_empty() && handles.meters.is_empty());
    let capture = match capture.ok_or_else(|| resource("host.spectrum.capture"))? {
        PreparedSpectrumCaptures::Single(capture) => capture,
        PreparedSpectrumCaptures::Collection(_) => {
            return Err(resource("host.spectrum.capture"));
        }
    };
    Ok((prepared, capture))
}

/// Prepare the normal host path with several atomically selectable spectrum observers.
pub fn prepare_host_runtime_with_spectrum_collection(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    request: &SpectrumCaptureCollectionRequest,
) -> Result<(PreparedHost, SpectrumCaptureCollection), PrepareDiagnostics> {
    let (prepared, _handles, capture) =
        prepare_host_runtime_with_live_controls_policy_and_spectrum(
            compiled,
            caps,
            None,
            None,
            Backend::current(),
            Some(&SpectrumPreparation::Collection(request.clone())),
            None,
            HostLiveLanes::ALL,
            None,
        )?;
    let capture = match capture.ok_or_else(|| resource("host.spectrum.capture"))? {
        PreparedSpectrumCaptures::Collection(capture) => capture,
        PreparedSpectrumCaptures::Single(_) => return Err(resource("host.spectrum.capture")),
    };
    Ok((prepared, capture))
}

#[allow(clippy::too_many_arguments)]
fn prepare_host_runtime_with_live_controls_policy_and_spectrum(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: Option<&HostLiveControlRequest>,
    selected_meters: Option<&[HostMeterRequest]>,
    backend: Backend,
    spectrum_request: Option<&SpectrumPreparation>,
    successor: Option<SuccessorBase<'_>>,
    lanes: HostLiveLanes,
    superseded: Option<&LiveValuesSuperseded>,
) -> Result<
    (
        PreparedHost,
        HostLiveControlHandles,
        Option<PreparedSpectrumCaptures>,
    ),
    PrepareDiagnostics,
> {
    match prepare_or_project(
        compiled,
        caps,
        live_controls,
        selected_meters,
        backend,
        spectrum_request,
        successor,
        lanes,
        superseded,
        PrepareMode::Prepare,
    )? {
        PrepareOutcome::Prepared(prepared) => Ok(*prepared),
        PrepareOutcome::Projected(_) => Err(platform("host.graph.projection")),
    }
}

/// Whether [`prepare_or_project`] prepares the plan or stops at its projection (issue #2199).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PrepareMode {
    Prepare,
    Project,
}

/// What [`prepare_or_project`] returns: the prepared host, or the projection it was admitted on.
enum PrepareOutcome {
    Prepared(
        Box<(
            PreparedHost,
            HostLiveControlHandles,
            Option<PreparedSpectrumCaptures>,
        )>,
    ),
    Projected(Box<HostProjection>),
}

/// The one preparation (issue #2199): every entry's, run to its projection and the projection's
/// admission, then, in [`PrepareMode::Prepare`], on to the plan.
///
/// Up to the projection it allocates the session's projected effect entries, its prepared
/// builtins and the graph front's tables, and no effect processor, bank, plan, source ring or
/// live-control lane: in [`PrepareMode::Project`] it returns the projection there.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn prepare_or_project(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: Option<&HostLiveControlRequest>,
    selected_meters: Option<&[HostMeterRequest]>,
    backend: Backend,
    spectrum_request: Option<&SpectrumPreparation>,
    successor: Option<SuccessorBase<'_>>,
    lanes: HostLiveLanes,
    superseded: Option<&LiveValuesSuperseded>,
    mode: PrepareMode,
) -> Result<PrepareOutcome, PrepareDiagnostics> {
    let projecting = mode == PrepareMode::Project;
    // #1709 D1: the whole preparation runs in the canonical floating-point environment, so a
    // route or send gain (`compile_graph`'s `db_to_gain_f32`), a native effect's coefficients and
    // every value composed below are what a canonical thread prepares, whatever the caller has
    // set. The inner guards (builtins body, lowerings, `controls::effective`, the catalog) nest:
    // each restores the canonical word this one installed, and this one restores the caller's.
    let _canonical = lane::CanonicalFpEnv::enter();
    // `None` is the plain entry: no live controls, and a request of defaults for the meter and
    // observation fields below. Every `Some` attaches live controls.
    let default_request = HostLiveControlRequest::default();
    let attach_live = live_controls.is_some();
    let live_controls = live_controls.unwrap_or(&default_request);
    // #1277 D10: a successor commits as a revision its own gate can stamp. Checked before this
    // preparation allocates anything.
    // #1594 D6: a monitor rebuild stays at the committed revision, which may be 0.
    if successor.is_some_and(|base| {
        let lowest = u64::from(!base.unchanged_revision);
        !(lowest..=GATE_REVISION_MAX).contains(&base.revision)
    }) {
        return Err(successor_revision_refusal());
    }
    // #1323 D3: at a host-declared discontinuity the successor joins against its predecessor's
    // sources only, so it has no input floors and every owner but a source consumer starts at
    // rest. Every join below reads the base's inventory, so this one substitution is the whole
    // rule.
    let discontinuity_inventory = successor
        .filter(|base| base.discontinuity)
        .map(|base| base.inventory.at_discontinuity());
    let successor = successor.map(|base| match discontinuity_inventory.as_ref() {
        Some(inventory) => SuccessorBase { inventory, ..base },
        None => base,
    });
    // #1354 D4: a warm successor's lead is a whole number of quanta, and a declared discontinuity
    // has no lead. Checked before this preparation allocates anything.
    let warm = successor.and_then(|base| base.warm);
    if successor.is_some_and(|base| {
        base.warm.is_some_and(|lead| {
            base.discontinuity
                || !lead
                    .lead_samples
                    .is_multiple_of(u64::from(compiled.quantum().0.max(1)))
        })
    }) {
        return Err(PrepareDiagnostics::fixed(
            PrepareRejection::Session,
            "host.successor.warm_lead",
        ));
    }
    // #1325 D6: the base the predecessor's cells hold, with the duck overlay laid over its
    // model. Every join below reads `committed`, so this one substitution composes the overlay
    // into the effective values the carry starts from. A control-thread copy that drops with
    // this call.
    let ducked_committed = successor.filter(|base| !base.duck.is_empty()).map(|base| {
        let mut ducked = base.committed.clone();
        base.duck.apply_to(&mut ducked);
        ducked
    });
    let successor = successor.map(|base| match ducked_committed.as_ref() {
        Some(committed) => SuccessorBase { committed, ..base },
        None => base,
    });
    // #1354a rulings 1-2: the restart set the lead's floors were derived from, and the nodes of
    // the predecessor's graph.
    let warm_restart = warm.map(|lead| {
        WarmRestart::new(
            compiled.normalized_model(),
            lead.restarted.iter().map(|id| &**id),
            lead.restart_whole.iter().map(String::as_str),
        )
    });
    let warm_predecessor = successor
        .filter(|_| warm.is_some())
        .map(|base| session_graph_nodes(base.committed));
    let restarts_whole = |id: &str| {
        warm_restart
            .as_ref()
            .is_some_and(|restart| restart.whole.contains(id))
    };
    // #1312 D2: the session's count when the host passed one, else this preparation's own, which
    // the handles hand back. Made before any observed allocation of the preparation.
    let live_superseded = superseded.cloned().unwrap_or_default();
    let model = compiled.normalized_model();
    check_counts(model, caps)?;
    // Issue #2194: the session's effect state and scratch, projected from the descriptors, inside
    // the caps before this preparation allocates any of it.
    let effect_projection = project_effects(compiled)?;
    check_effect_projection(effect_projection, caps)?;
    // The counts the report carries; `check_counts` has proved them inside the caps.
    let track_count = u64::try_from(model.tracks.len()).map_err(|_| platform("host.count"))?;
    let submix_count = u64::try_from(model.submixes.len()).map_err(|_| platform("host.count"))?;
    let vca_count = u64::try_from(model.vcas.len()).map_err(|_| platform("host.count"))?;
    let source_count = u64::try_from(model.sources.len()).map_err(|_| platform("host.count"))?;
    let route_count = u64::try_from(model.routes.len()).map_err(|_| platform("host.count"))?;
    let effect_count = count_effects(model)?;
    caps.validate_shape(compiled)?;
    // Issue #1581 D1/D2: one window for every meter of the plan, and the core's strip selection
    // unless the caller chose its own. The plain entry (no request) meters nothing.
    let meter_config = requested_meter_config(live_controls, compiled)?;
    let default_meters = if attach_live && selected_meters.is_none() {
        crate::meters::meter_selection(model)
    } else {
        Vec::new()
    };
    let selected_meters = selected_meters.or(attach_live.then_some(default_meters.as_slice()));
    // The overlay the plan renders and lists under (#1559 D9): a successor's own, which holds its
    // master by strip ID (#1683); a first plan's solos nothing and designates no master (#1689 D3):
    // the app writes the strip's `master` row after boot.
    let boot_overlay = controls::MonitorOverlay::new();
    let overlay = successor.map_or(&boot_overlay, |base| base.monitor.next);

    let source_id_bytes = model.sources.iter().try_fold(0_usize, |total, source| {
        total
            .checked_add(source.id.as_str().len())
            .ok_or_else(|| resource("host.resource.arithmetic"))
    })?;
    let mut graph_sources = Vec::new();
    graph_sources
        .try_reserve_exact(compiled.source_count())
        .map_err(|_| resource("host.resource.allocation"))?;
    // #1642 B16-R2: the session's published seek generation. A first plan allocates it here, on
    // the control thread; a successor shares its predecessor's, so every plan of the session and
    // every producer feeding it publishes to, and every driver loads, the same word.
    let published_generation = successor.map_or_else(
        || PublishedGeneration::new(SourceGeneration(1)),
        |base| base.inventory.published_generation.clone(),
    );
    let mut builder = ControlSourceBuilder::with_capacity(source_id_bytes, compiled.source_count())
        .map_err(|()| resource("host.resource.allocation"))?;
    // Issue #1272 D3: the predecessor's committed source declarations, by ID.
    let committed_sources: BTreeMap<&str, &session::Source> = successor
        .map(|base| {
            base.committed
                .sources
                .iter()
                .map(|source| (source.id.as_str(), source))
                .collect()
        })
        .unwrap_or_default();
    let mut inventory_sources = Vec::with_capacity(model.sources.len());
    let mut carried_sources: Vec<(u32, u32)> = Vec::new();
    let mut carried_total_bytes = 0_u64;
    let mut carried_overhead_bytes = 0_u64;
    let mut carried_largest_bytes = 0_u64;
    for (index, source) in model.sources.iter().enumerate() {
        let index = u32::try_from(index).map_err(|_| platform("host.count"))?;
        let ring = source_ring_config(compiled, caps, source);
        // Issue #1272 D3: carry when the ID is in both models, the declaration is unchanged and
        // the ring the successor would allocate is the one the predecessor renders from.
        let carried_from = successor.and_then(|base| {
            let row = base.inventory.source(source.id.as_str())?;
            let committed = committed_sources.get(source.id.as_str())?;
            source_carries(row, committed, source, ring).then_some(row.index)
        });
        if let Some(predecessor_index) = carried_from {
            // D4: no ring here; the single-plan caps below count the carried one.
            let carried = PcmSourceRing::resource_report(ring)
                .map_err(|_| resource("host.source.prepare"))?;
            carried_total_bytes = carried_total_bytes
                .checked_add(carried.total_engine_owned_bytes)
                .ok_or_else(|| resource("host.resource.arithmetic"))?;
            carried_overhead_bytes = carried_overhead_bytes
                .checked_add(carried.overhead_bytes)
                .ok_or_else(|| resource("host.resource.arithmetic"))?;
            carried_largest_bytes = carried_largest_bytes.max(carried.largest_allocation_bytes);
            builder.push(
                source.id.as_str(),
                compiled.sample_rate().0,
                u32::from(source.channels),
                0,
                source.frames,
                None,
            );
            graph_sources.push(SourceGraphSource::vacant(u32::from(source.channels)));
            carried_sources.push((index, predecessor_index));
        } else if projecting {
            // Issue #2199: the ring this source would allocate, charged without allocating it.
            let resources = PcmSourceRing::resource_report(ring)
                .map_err(|_| resource("host.source.prepare"))?;
            builder.push(
                source.id.as_str(),
                compiled.sample_rate().0,
                u32::from(source.channels),
                0,
                source.frames,
                None,
            );
            graph_sources.push(SourceGraphSource::projected(
                u32::from(source.channels),
                resources,
            ));
        } else {
            let (producer, consumer, resources) = PcmSourceRing::prepare_host_region(
                ring,
                SourceFrame(0),
                published_generation.clone(),
            )
            .map_err(|_| resource("host.source.prepare"))?;
            builder.push(
                source.id.as_str(),
                compiled.sample_rate().0,
                u32::from(source.channels),
                0,
                source.frames,
                Some(producer.into_host_chunk_provider(compiled.sample_rate())),
            );
            graph_sources.push(SourceGraphSource::new(consumer, resources, 0, 0));
        }
        inventory_sources.push(SourceInventoryRow {
            id: source.id.as_str().into(),
            ring,
            index,
        });
    }
    // #1402 R-B, #1355 D5: the sources whose rings carry under #1272 D3, by ID.
    let carried_ids: BTreeSet<&str> = carried_sources
        .iter()
        .map(|&(index, _)| model.sources[index as usize].id.as_str())
        .collect();
    // #1354 D3: the channels of the carried sources a warm successor's prime replays.
    let carried_channels: u64 = carried_sources
        .iter()
        .map(|&(index, _)| u64::from(model.sources[index as usize].channels))
        .sum();
    // Issue #1276 D1, #1630 D1: the strip input sections that carry, with the predecessor lane
    // each one copies from and the records that retarget it. Every strip of this plan attaches
    // the same control kind.
    let live_input = attach_live && lanes.strip_input;
    let mut carried_inputs: Vec<InputJoin<'_>> = match successor {
        None => Vec::new(),
        Some(base) => {
            // `committed_input_section` binary-searches: a model that is not normalized would
            // silently restart every strip at rest.
            debug_assert!(
                base.committed
                    .tracks
                    .windows(2)
                    .all(|pair| pair[0].id < pair[1].id)
                    && base
                        .committed
                        .submixes
                        .windows(2)
                        .all(|pair| pair[0].id < pair[1].id),
                "a successor's committed model is normalized: strips in canonical ID order"
            );
            // #1630 D1: the join keys on the section's structure. Every strip's input section has
            // the same stages (trim, polarity, two filter sections per channel), so the structure
            // is the control kind and the rate its designs are for; a live difference carries and
            // retargets, and a plan without input cells carries only bit-equal values.
            let ramps = LiveRamps::for_session(model);
            let rate = compiled.sample_rate().0;
            let same_rate = base.inventory.sample_rate_hz == rate;
            model
                .strips()
                .filter_map(|strip| {
                    let row = base.inventory.input_section(strip.id.as_str())?;
                    let before = committed_input_section(base.committed, &strip)?;
                    if !same_rate || row.live != live_input {
                        return None;
                    }
                    let retarget = if live_input {
                        // A retarget is the record of the same edit (#1394 D6): the live path's
                        // records, so the carried lane ramps as a live edit would. A value the
                        // live path refuses restarts the section.
                        let edit = StripEditRamps::of(base.edit_ramps, strip.id);
                        // A compiled model can hold values the live path refuses: a high-pass
                        // above its low-pass compiles, and is a `Domain` rebuild with no live
                        // record. The section then restarts at rest; it is no internal fault.
                        input_records(before, strip.builtins, rate, ramps, edit).ok()?
                    } else if same_input_section(before, strip.builtins) {
                        Vec::new()
                    } else {
                        return None;
                    };
                    Some(InputJoin {
                        id: strip.id.as_str(),
                        predecessor: row.location,
                        retarget,
                    })
                })
                // #1354a decision 2: a strip restarted whole starts its input section at rest.
                .filter(|join| !restarts_whole(join.id))
                .collect()
        }
    };
    // Issue #1277 D3, D4: how each strip's fader/mute and matrix values differ between the
    // predecessor's base and this model: the join's model half, resolved against the plans' rows
    // once this plan is bound.
    let live_strips = attach_live;
    // #1844: the automation table of each model is walked once, here, and the strip join, the
    // send join, the following routes and their count look each owner up.
    let model_automation = controls::AutomationIndex::of(model);
    let view = controls::ModelView::whole(model).indexed(&model_automation);
    let committed_automation = successor.map(|base| controls::AutomationIndex::of(base.committed));
    // The effective values of both models, computed once: the strip join and the send join read
    // the same rules (#1558 D1).
    let level_diff = successor
        .zip(committed_automation.as_ref())
        .map(|(base, committed)| {
            EffectiveDiff::new(
                base.committed,
                model,
                LiveRamps::for_session(model),
                base.edit_ramps,
                base.monitor,
                [
                    AutomationReads::indexed(base.committed, committed),
                    AutomationReads::indexed(model, &model_automation),
                ],
            )
        });
    let strip_joins: Vec<StripJoin<'_>> = match (successor, &level_diff) {
        (Some(base), Some(diff)) => {
            join_strips(base.committed, model, live_strips, base.edit_ramps, diff)
        }
        _ => Vec::new(),
    };
    // Issue #1284 D1, D2: how each strip's input alignment delay and each send differ between the
    // predecessor's base and this model, resolved against the plans' lines and live routes once
    // this plan is bound.
    let delay_joins: Vec<StripDelayJoin<'_>> = match successor {
        None => Vec::new(),
        // #1402 R-B: the sources whose rings carry under #1272 D3.
        Some(base) => join_strip_delays(base.committed, model, &carried_ids),
    };
    let route_joins: Vec<RouteJoin<'_>> = match (successor, &level_diff) {
        (Some(base), Some(diff)) => join_routes(base.committed, model, base.edit_ramps, diff),
        _ => Vec::new(),
    };
    // Issue #1279 D1: how each strip's console slots and effect chain differ between the
    // predecessor's base and this model, resolved against the plans' lanes once this plan is
    // bound.
    let chain_joins: Vec<ChainJoin<'_>> = match successor {
        None => Vec::new(),
        // #1354a decision 2: a strip restarted whole starts its effects at rest.
        Some(base) => join_chains(base.committed, model, base.edit_ramps)
            .into_iter()
            .filter(|join| !restarts_whole(join.id))
            .collect(),
    };
    // The hand-over the successor runs at its swap block, built here so the plan charge below
    // counts it; installed after bind.
    // Issue #1327 D1: a meter carries when its key and configuration are the predecessor's; the
    // joins that follow bind find the observers, this only decides that a program is needed.
    let meters_may_carry = match successor {
        Some(base) => meters_may_carry(
            &base.inventory.meters,
            &requested_meter_owners(selected_meters),
            MeterConfigKey::of(&meter_config),
            |id| same_strip_id_kind(base.committed, model, id),
        ),
        None => false,
    };
    // Issue #1395 D1: a spectrum capture carries when its target and channels are the
    // predecessor's and the two plans run at one rate and quantum; as for meters, this only decides
    // that a program is needed.
    let spectrum_may_carry = match (successor, &spectrum_request) {
        (Some(base), Some(SpectrumPreparation::Single(request))) => {
            base.inventory.sample_rate_hz == compiled.sample_rate().0
                && base.inventory.quantum_frames == compiled.quantum().0
                && !base.inventory.spectrum_collection
                && base
                    .inventory
                    .spectrum
                    .iter()
                    .any(|row| row.target == request.target && row.channels == request.channels)
        }
        (Some(base), Some(SpectrumPreparation::Collection(request))) => {
            base.inventory.sample_rate_hz == compiled.sample_rate().0
                && base.inventory.quantum_frames == compiled.quantum().0
                && base.inventory.spectrum_collection
                && request.entries.iter().any(|entry| {
                    base.inventory
                        .spectrum
                        .iter()
                        .any(|row| row.target == entry.target && row.channels == entry.channels)
                })
        }
        _ => false,
    };
    // #1641 D3: every successor carries the session timeline, so every successor has a program.
    // Its history is the successor's own, of its own `K`; the move is a swap and a bounded copy.
    // Both hosts refuse a rate or quantum change (#1290a), so the guard only keeps the carry
    // defined for a caller that prepares one by hand: such a successor starts a fresh timeline.
    let timeline_carries = successor.is_some_and(|base| {
        base.inventory.sample_rate_hz == compiled.sample_rate().0
            && base.inventory.quantum_frames == compiled.quantum().0
    });
    let carry_program = match successor {
        Some(base)
            if timeline_carries
                || !carried_sources.is_empty()
                || !carried_inputs.is_empty()
                || meters_may_carry
                || spectrum_may_carry
                || strip_joins.iter().any(StripJoin::may_carry)
                || chain_joins.iter().any(ChainJoin::may_carry) =>
        {
            Some(graph::GraphCarryProgram {
                predecessor: base.inventory.plan_identity(),
                sources: carried_sources.into_boxed_slice(),
                timeline: timeline_carries,
            })
        }
        _ => None,
    };
    // The input moves' table is charged at its full length: every strip banks its input section
    // in a shipped plan, so each carried strip finds its successor lane after bind.
    let carry_program_bytes = carry_program
        .as_ref()
        .map_or(0, graph::GraphCarryProgram::retained_bytes)
        .checked_add(
            u64::try_from(
                carried_inputs
                    .len()
                    .checked_mul(core::mem::size_of::<graph::GraphLaneMove>())
                    .ok_or_else(|| resource("host.resource.arithmetic"))?,
            )
            .map_err(|_| resource("host.resource.arithmetic"))?,
        )
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let mut sources = builder.finish(published_generation.clone());

    // Source semantics: tracks only.
    let mappings = model
        .tracks
        .iter()
        .map(|track| {
            let source_index = compiled
                .source_index(&track.source_id)
                .and_then(|value| usize::try_from(value).ok())
                .ok_or_else(|| graph_failure("host.source.mapping"))?;
            Ok(SourceGraphTrackMapping {
                node: GraphNodeId::TrackStage {
                    track_id: StableGraphId::parse(track.id.as_str())
                        .ok_or_else(|| graph_failure("host.source.mapping"))?,
                    stage: TrackStage::Input,
                },
                source_index,
                left_channel: u32::from(track.left_source_channel),
                right_channel: u32::from(track.right_source_channel),
            })
        })
        .collect::<Result<Vec<_>, PrepareDiagnostics>>()?;

    // #1953b: the fading copies this successor compiles beside its model, on the terms of an
    // added route's arm. Only the graph sees them: the effects, the builtins and the graph are
    // prepared from the model with the copies added; every join, the catalog and the report read
    // `model`.
    let route_copies: Vec<RouteCopy> = match successor {
        Some(base) if attach_live && !base.discontinuity => route_copies(&base, model),
        _ => Vec::new(),
    };
    let copied_session = if route_copies.is_empty() {
        None
    } else {
        let mut copied = model.clone();
        copied
            .routes
            .extend(route_copies.iter().map(|copy| copy.route.clone()));
        let copy_caps = caps.compile_caps(copied.sources.len())?;
        Some(compile_host_model(&copied, copy_caps).map_err(|_| graph_failure("host.fade.copy"))?)
    };
    let graph_session = copied_session.as_ref().unwrap_or(compiled);
    let registry =
        launch_native_effect_registry().map_err(|_| effect_failure("host.effect.registry"))?;
    // The stored automation is held to the row it names here, beside the effect diagnostics, so
    // every host refuses a session alike (#1566 D7).
    let automation: Vec<effect_compiler::EffectDiagnostic> =
        controls::automation_diagnostics(compiled.normalized_model(), compiled.sample_rate().0)
            .into_iter()
            .map(|diagnostic| effect_compiler::EffectDiagnostic {
                code: diagnostic.code,
                path: diagnostic.path,
            })
            .collect();
    // Issue #2199: projected, not prepared: the graph compile prepares a processor only for an
    // instance its bank plan leaves per node, after the projection below is admitted.
    let mut effects = match project_native_session_effects(
        graph_session,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: caps.maximum_effect_state_bytes,
            maximum_scratch_bytes: caps.maximum_effect_scratch_bytes,
            maximum_automation_spans_per_block: u32::MAX,
        },
    ) {
        Ok(effects) if automation.is_empty() => effects,
        Ok(_) => {
            return Err(effect_diagnostics(
                effect_compiler::EffectDiagnosticSet::sorted(automation),
            ));
        }
        Err(set) => {
            return Err(effect_diagnostics(
                effect_compiler::EffectDiagnosticSet::sorted(
                    set.0.into_iter().chain(automation).collect(),
                ),
            ));
        }
    };
    let (effect_state_bytes, effect_scratch_bytes) =
        effects
            .entries
            .iter()
            .try_fold((0_u64, 0_u64), |total, entry| {
                Ok::<_, PrepareDiagnostics>((
                    total
                        .0
                        .checked_add(
                            entry
                                .metadata
                                .state_sizes
                                .total()
                                .ok_or_else(|| resource("host.effect.resource.arithmetic"))?,
                        )
                        .ok_or_else(|| resource("host.effect.resource.arithmetic"))?,
                    total
                        .1
                        .checked_add(entry.metadata.scratch_bytes)
                        .ok_or_else(|| resource("host.effect.resource.arithmetic"))?,
                ))
            })?;
    // Issue #2194: the prepared instances' declared rows are the projection's, which
    // `check_effect_projection` admitted before anything was allocated; preparation holds each
    // instance's metadata to its descriptor row (`effect.metadata.mismatch`), so a difference here
    // is an internal fault, never a session's.
    if effect_state_bytes != effect_projection.state_bytes
        || effect_scratch_bytes != effect_projection.scratch_bytes
    {
        return Err(platform("host.effect.projection"));
    }

    // Issue #140 A: one live-control lane per prepared strip effect instance (a submix strip's
    // included, #1207 D5): its latest-target cells and, for an effect that declares taps when the
    // host asked for observation capacity, its observation queue (decision 15, D15-2). This is
    // the only thing that creates one; a host that asks for no live controls attaches nothing and
    // the plan renders the byte-identical live-control-free path.
    //
    // Issue #2199: the lanes are laid out here and attached once the projection is admitted, so
    // the projection charges them before one exists.
    let attach_effect_lanes = attach_live && lanes.effects;
    let projected_effect_controls = if attach_effect_lanes {
        plan_projected_effect_live_controls(&mut effects, live_controls.observation_taps)
            .map_err(effect_diagnostics)?
    } else {
        EffectControlResources::default()
    };
    // Issue #143 D3, level 1. Observation capacity is attached only when it was asked for, and
    // only alongside a control channel: a subscription is a record in the effect's own observation
    // queue, which only the effect's control lane drains, so "observe without live controls" has
    // no delivery path and is a rejection rather than a silent
    // half-attach. The published window is the meter window, derived rather than configured.
    let observation_window_blocks = meter_config.period_frames.get() / compiled.quantum().0.max(1);
    let effect_observations: Vec<EffectObservationHandle> = match live_controls.observation_taps {
        0 => Vec::new(),
        // Issue #1254 D2: without effect lanes there is no queue for a subscription to ride.
        taps if !attach_live || !lanes.effects => {
            let _ = taps;
            return Err(shape("host.observation.live_controls"));
        }
        taps => {
            // #1584 D6a: the taps the host armed before this rebuild are armed in the successor.
            let arms: Vec<effect_compiler::ObservationArm<'_>> = successor
                .map(|base| base.observation.bindings())
                .unwrap_or_default()
                .iter()
                .map(|binding| effect_compiler::ObservationArm {
                    strip_id: binding.key.strip.as_str(),
                    console: binding.key.section.is_console(),
                    effect_id: binding.key.section.id().as_str(),
                    tap_id: binding.key.tap_id,
                    window_blocks: binding.window_blocks,
                    arm_tag: binding.arm_tag,
                })
                .collect();
            attach_projected_effect_observation(
                &mut effects,
                taps,
                observation_window_blocks,
                &arms,
            )
            .map_err(effect_diagnostics)?
        }
    };

    // Issue #137 D1/D2: the live-control requests are derived here, once, from the canonical strip
    // order, so `HostLiveControlHandles::strips` and the requested channels cannot disagree. The
    // strip list (issue #1207 D1) appends the submixes after the tracks. Every strip gets a control
    // request (issue #1211 D1), and the default meters are this list's order (#1581 D1);
    // `live_control_tracks` is exactly the strip list's track prefix.
    let live_control_strips: Vec<Box<str>> = model
        .tracks
        .iter()
        .map(|track| Box::<str>::from(track.id.as_str()))
        .chain(
            model
                .submixes
                .iter()
                .map(|submix| Box::<str>::from(submix.id.as_str())),
        )
        .collect();
    let live_control_track_count = model.tracks.len();
    // #1554 D6: the producer table is sorted by (strip position, address), so a commit finds an
    // instance's producer by binary search. Preparation attaches them in `(strip ID, rack, effect
    // ID)` order, which is neither. The keys are unique, so the unstable sort is deterministic.
    debug_assert!(
        live_control_strips[..live_control_track_count].is_sorted()
            && live_control_strips[live_control_track_count..].is_sorted(),
        "each segment of the strip list is in ascending ID order"
    );
    let strip_rank = |id: &str| {
        strip_position(
            live_control_strips.len(),
            live_control_track_count,
            |index| &*live_control_strips[index],
            id,
        )
    };
    let control_requests: Vec<TrackControlRequest> = if attach_live {
        live_control_strips
            .iter()
            .map(|strip| TrackControlRequest {
                track_id: strip.to_string(),
                superseded: live_superseded.clone(),
                input_lane: lanes.strip_input,
            })
            .collect()
    } else {
        Vec::new()
    };
    // A selected meter may name any strip (issue #1208 D1); the default set is one per strip.
    let meter_tracks = requested_meter_owners(selected_meters);
    let meter_requests: Vec<SelectedMeterRequest> = meter_tracks
        .iter()
        .enumerate()
        .map(|(index, (track, tap, metrics))| {
            // Handles are `index + 1`: nonzero, stable in request order (canonical strip order
            // for the default set); nothing outside this function invents a meter handle.
            let handle = u64::try_from(index)
                .ok()
                .and_then(|value| value.checked_add(1))
                .and_then(core::num::NonZeroU64::new)
                .ok_or_else(|| platform("host.count"))?;
            Ok(SelectedMeterRequest {
                request: MeterRequest {
                    handle: MeterHandle(handle),
                    track_id: (*track).to_string(),
                    tap: *tap,
                    config: meter_config,
                },
                metrics: *metrics,
            })
        })
        .collect::<Result<Vec<_>, PrepareDiagnostics>>()?;
    let builtin_caps = builtin_compile_caps(caps);
    // Decision 17, C7: every effective value is computed once per preparation, here, under the
    // overlay the successor renders with (#1559 D9); a first plan has no solo.
    // #1646 D2: composed in the canonical floating-point environment, as the builtins
    // preparation composes an automated lane, so a flat automated lane and a static fader round
    // alike whatever the caller's thread has set.
    let effective = {
        let _canonical = lane::CanonicalFpEnv::enter();
        controls::effective(graph_session.normalized_model(), overlay)
    };
    let builtins = if selected_meters.is_some() {
        prepare_selected_session_builtins_with_live_controls(
            graph_session,
            &effective,
            &meter_requests,
            &control_requests,
            builtin_caps,
        )
    } else {
        prepare_session_builtins_with_live_controls(
            graph_session,
            &effective,
            &meter_requests
                .iter()
                .map(|item| item.request.clone())
                .collect::<Vec<_>>(),
            &control_requests,
            builtin_caps,
        )
    }
    .map_err(|diagnostics| {
        PrepareDiagnostics::new(
            PrepareRejection::Builtin,
            diagnostic_lines(
                diagnostics
                    .0
                    .iter()
                    .map(|diagnostic| (diagnostic.code, &diagnostic.path)),
            ),
        )
    })?;
    let builtin_resources = builtins.resource_report();
    // #1652 D1 (#1652c): the source of every following route, read before lowering consumes the
    // builtins plan: the plan's one automation program, the source lanes' mute cells and the
    // seeded flags word. `terms` is the source lane's effective mute without its curve (OQ1 = C);
    // `automated` is the plan's. The arrival `a(tap)` is the compile report's, below.
    let mut follow_cells = Vec::new();
    for route in &model.routes {
        let Some(automated) = following_route_lanes(view, route) else {
            continue;
        };
        let Some(source) = route_source_strip(model, route) else {
            return Err(graph_failure("host.route_control.attach"));
        };
        // `following_route_lanes` and the builtins plan read the same rendered mute entries, so
        // a following route always has the cells its lanes name, in the plan's one program.
        // #1655 D4: a send whose source lanes only VCA mutes reach takes the VCA cells' program
        // below.
        let (mut program, cells) = match builtins.strip_mute_cells(route_source_id(route).as_str())
        {
            Some((program, cells)) => (Some(program), cells),
            None => (None, [None; 2]),
        };
        // #1655 D4: a source lane's curve is its own mute cell, then the reaching automated VCA
        // mute cells in ascending VCA ID; `automated` names the lanes that have one. The route op
        // ORs the VCA mute cursors into the lane's follow term at `a(tap)`.
        let vca_mutes = follow_vca_mute_cells(
            |vca| builtins.vca_cells(vca),
            model,
            &model_automation,
            route_source_id(route),
            &mut program,
        )
        .ok_or_else(|| graph_failure("host.route_control.attach"))?;
        let Some(program) = program else {
            return Err(graph_failure("host.route_control.attach"));
        };
        let curves = [0, 1].map(|lane| cells[lane].is_some() || !vca_mutes[lane].is_empty());
        if curves != automated {
            return Err(graph_failure("host.route_control.attach"));
        }
        let flags = follow_route_flags(route, effective.strips[source].mute, automated);
        // #1655 D6: a source lane whose own and reaching VCA mute tables continue carries its
        // VCA mute cursors verbatim.
        let continued_vca =
            successor
                .zip(level_diff.as_ref())
                .map_or([false; 2], |(base, diff)| {
                    let id = route_source_id(route);
                    diff.continued_mute_lanes(controls::strip_index(base.committed, id), source, id)
                });
        follow_cells.push((
            route.id.as_str(),
            (program, cells, vca_mutes, flags, continued_vca),
        ));
    }
    // Issue #2199: the graph projected before any processor, bank or plan exists; its estimate is
    // the one the compiled plan reports.
    let mut graph_projection = GraphCompiler::project(GraphProjectionRequest {
        dispatch: backend,
        plan_id: 1,
        effects,
        builtins: &builtins,
        // #1285 D2: a successor keeps every surviving node's input arrival; a first plan has no
        // floors. Across a withdrawn candidate the base is P0's inventory, so these are P0's.
        // #1354 D4: a warm successor takes the lead's floors instead.
        input_floors: {
            let mut floors = match (successor, warm, &warm_restart, &warm_predecessor) {
                (Some(base), Some(lead), Some(restart), Some(predecessor)) => warm_floors(
                    &base.inventory.input_floors,
                    predecessor,
                    restart,
                    lead.lead_samples,
                )?,
                _ => successor
                    .map(|base| {
                        base.inventory
                            .input_floors
                            .iter()
                            .map(|(node, floor)| {
                                (node.clone(), effect_contract::LatencySamples(*floor))
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
            };
            // #1953b: a copy's op arrives where the route it replaces arrived, so its lines are
            // that route's and carry by alias.
            for copy in &route_copies {
                let node = |id: &str| {
                    StableGraphId::parse(id).map(|route_id| GraphNodeId::Route { route_id })
                };
                if let (Some(alias), Some(own)) = (node(&copy.alias), node(copy.route.id.as_str()))
                    && let Some(floor) = floors.get(&alias).copied()
                {
                    floors.insert(own, floor);
                }
            }
            floors
        },
        caps: graph_compile_caps(caps),
        // #1379 Amendment 1 H3: with a control queue every strip's fader, mute and matrix lanes
        // are attached; a route into a submix gets its lane after the compile, when asked.
        gain_liveness: graph_compiler::GainLiveness {
            strip_fader_matrix: attach_live,
            routes: attach_live && lanes.routes,
        },
    })
    .map_err(|failure| graph_diagnostics(&failure.diagnostics))?;
    // The plan's own cells the graph row does not count: its revision gate (#1502 D1), its
    // jump-length cell (#1639 D1, attached after bind below) and, when this preparation made its
    // own `live_values_superseded` count rather than taking the session's, that count (#1312 D2;
    // a session's count is the host's to charge once): the jump-length cell's reader holds a
    // clone of it in every plan. Charged here so every host, the C ABI and the browser, counts
    // them in the one row.
    let plan_cell_bytes = engine::realtime::plan_revision_gate_retained_bytes()
        .checked_add(graph::jump_length_retained_bytes())
        .and_then(|bytes| {
            bytes.checked_add(if superseded.is_none() {
                LiveValuesSuperseded::retained_bytes()
            } else {
                0
            })
        })
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    // #1953b: the session the graph was compiled from, fading copies included.
    let session_resources = graph_session.resource_estimate();
    let spectrum_resources: Option<SpectrumCaptureResources> = match spectrum_request {
        None => None,
        Some(SpectrumPreparation::Single(request)) => Some(
            crate::spectrum::spectrum_capture_resources_for(&request.target),
        ),
        Some(SpectrumPreparation::Collection(request)) => Some(
            spectrum_capture_collection_resources(&request.entries)
                .map_err(spectrum_diagnostics)?,
        ),
    };
    let spectrum_capture_retained_bytes =
        spectrum_resources.map_or(0, |resources| resources.retained_bytes);
    let six_word = attach_live && lanes.routes;
    #[cfg_attr(not(feature = "control-provider"), allow(unused_mut))]
    let mut host_projection = {
        let arithmetic = || resource("host.resource.arithmetic");
        let estimate = graph_projection.resource_estimate();
        // The send lanes the attach below gives: every live route that does not follow when the
        // plan attaches six-word lanes, and every following route; a successor's transition
        // routes at their bound, every route of the model and every fading copy.
        let six_ids: Vec<&str> = if six_word {
            model
                .routes
                .iter()
                .filter(|route| {
                    controls::route_rows_live(route) && !follows_automated_mute(view, route)
                })
                .map(|route| route.id.as_str())
                .collect()
        } else {
            Vec::new()
        };
        let following: Vec<graph::FollowingRouteBound<'_>> = follow_cells
            .iter()
            .map(|(id, (_, cells, vca_mutes, _, _))| {
                (
                    *id,
                    *cells,
                    [vca_mutes[0].as_slice(), vca_mutes[1].as_slice()],
                )
            })
            .collect();
        let transition: Vec<&str> = match successor {
            Some(base) if attach_live && !base.discontinuity => model
                .routes
                .iter()
                .map(|route| route.id.as_str())
                .chain(route_copies.iter().map(|copy| copy.route.id.as_str()))
                .collect(),
            _ => route_copies
                .iter()
                .map(|copy| copy.route.id.as_str())
                .collect(),
        };
        let routes = graph::route_control_resources_bound(
            &six_ids,
            &following,
            &transition,
            u64::try_from(graph_projection.routes().len()).map_err(|_| arithmetic())?,
            u64::try_from(graph_projection.nodes().len()).map_err(|_| arithmetic())?,
            graph_projection
                .routes()
                .iter()
                .any(|route| route.gate.silences()),
        );
        // The sources: each ring as the loop above charged it, the mappings and the timeline,
        // whose history covers the largest floored input arrival (#1641 D1).
        let largest_arrival = graph_projection
            .input_arrivals()
            .iter()
            .map(|(_, arrival)| arrival.0)
            .max()
            .unwrap_or(0);
        let history_blocks = largest_arrival
            .div_ceil(u64::from(compiled.quantum().0.max(1)))
            .checked_add(1)
            .and_then(|blocks| usize::try_from(blocks).ok())
            .and_then(NonZeroUsize::new)
            .ok_or_else(arithmetic)?;
        let source_report = source::project_session_graph_source_set(
            compiled.quantum(),
            history_blocks,
            timeline_carries,
            &graph_sources,
            &mappings,
        )
        .map_err(|_| graph_failure("host.source.graph.prepare"))?;
        // A carried timeline's command queue and seek report join the carried rings' bytes, as
        // the preparation below counts them (#1641 D3, #1704 D4).
        let carried_timeline = if timeline_carries {
            let report = SourceTimeline::resource_report(compiled.quantum(), history_blocks)
                .map_err(|_| resource("host.source.prepare"))?;
            report
                .command_queue_bytes
                .checked_add(report.seek_report_bytes)
                .ok_or_else(arithmetic)?
        } else {
            0
        };
        let inventory = |successor: bool| {
            projected_inventory_bytes(
                model,
                &graph_projection,
                meter_requests.len(),
                effect_observations.len(),
                match spectrum_request {
                    None => 0,
                    Some(SpectrumPreparation::Single(_)) => 1,
                    Some(SpectrumPreparation::Collection(request)) => request.entries.len(),
                },
                successor,
                &route_copies,
            )
            .ok_or_else(arithmetic)
        };
        let carry_bytes = match successor {
            None => carry_program_bytes,
            Some(_) => successor_carry_bound(
                model,
                &graph_projection,
                carry_program_bytes,
                meter_requests.len(),
                spectrum_resources.is_some(),
                &route_copies,
            )
            .ok_or_else(arithmetic)?,
        };
        HostProjection {
            effects: effect_projection,
            graph_session_plus_plan_bytes: estimate
                .session_plus_plan_bytes
                .checked_add(plan_cell_bytes)
                .ok_or_else(arithmetic)?,
            graph_declared_effect_bytes: estimate.declared_effect_bytes,
            builtin_retained_payload_bytes: builtin_resources.engine_owned_retained_payload_bytes,
            session_model_bytes: session_resources.compiled_model_bytes,
            spectrum_capture_retained_bytes,
            effect_control_resources: projected_effect_controls,
            route_controls: routes,
            carry_bytes,
            carry_program_bytes,
            inventory_bytes: inventory(successor.is_some())?,
            control_retained_bytes: sources.retained_bytes().ok_or_else(arithmetic)?,
            source_total_bytes: source_report.total_engine_owned_bytes,
            source_overhead_bytes: source_report.overhead_bytes,
            carried_source_total_bytes: carried_total_bytes
                .checked_add(carried_timeline)
                .ok_or_else(arithmetic)?,
            carried_source_overhead_bytes: carried_overhead_bytes
                .checked_add(carried_timeline)
                .ok_or_else(arithmetic)?,
            strip_heap_bytes: builtins.track_control_heap_bytes(),
            strips: model.strips().count(),
            strip_id_bytes: u64::try_from(
                model
                    .strips()
                    .map(|strip| strip.id.as_str().len())
                    .sum::<usize>(),
            )
            .map_err(|_| arithmetic())?,
            meters: meter_requests.len(),
            effect_instances: graph_projection.effects().entries.len(),
            effect_producers: if attach_effect_lanes {
                graph_projection.effects().entries.len()
            } else {
                0
            },
            effect_producer_id_bytes: if attach_effect_lanes {
                u64::try_from(
                    graph_projection
                        .effects()
                        .entries
                        .iter()
                        .map(|entry| entry.track_id.len() + entry.effect_id.len())
                        .sum::<usize>(),
                )
                .map_err(|_| arithmetic())?
            } else {
                0
            },
            observation_handles: effect_observations.len(),
            observation_handle_bytes: u64::try_from(effect_observations.iter().fold(
                core::mem::size_of_val(effect_observations.as_slice()),
                |total, handle| {
                    total
                        + handle.track_id.len()
                        + handle.effect_id.len()
                        + core::mem::size_of_val(&*handle.readers)
                },
            ))
            .map_err(|_| arithmetic())?,
            observation_taps: effect_observations
                .iter()
                .map(|handle| handle.readers.len())
                .sum(),
            observation_slots: if attach_effect_lanes && live_controls.observation_taps != 0 {
                graph_projection
                    .effects()
                    .entries
                    .iter()
                    .map(|entry| entry.factory.descriptor().observations.len())
                    .sum()
            } else {
                0
            },
            // A preparation builds the catalog itself once admitted, and records its resources.
            #[cfg(feature = "control-provider")]
            catalog: if projecting {
                crate::ControlCatalog::project(
                    compiled.normalized_model(),
                    overlay,
                    compiled.sample_rate().0,
                )
                .map_err(|_| resource("host.control_provider.allocation"))?
            } else {
                crate::CatalogResources::default()
            },
        }
    };
    // Issue #2199: the projection's admission, before any processor, bank, plan or lane exists:
    // the rows the exact admission below charges, at their projected bytes.
    if host_projection
        .admitted_graph_and_model()
        .ok_or_else(|| resource("host.resource.arithmetic"))?
        > caps.maximum_graph_session_plus_plan_bytes
    {
        return Err(resource("host.graph.resource.limit"));
    }
    if projecting {
        return Ok(PrepareOutcome::Projected(Box::new(host_projection)));
    }
    #[cfg(feature = "control-provider")]
    let control_catalog = crate::ControlCatalog::build(
        compiled.normalized_model(),
        overlay,
        compiled.sample_rate().0,
    )
    .map_err(|_| resource("host.control_provider.allocation"))?;
    #[cfg(feature = "control-provider")]
    {
        host_projection.catalog = control_catalog.resources();
    }
    let mut effect_controls: Vec<EffectControlProducer> = if attach_effect_lanes {
        attach_projected_effect_live_controls(
            graph_projection.effects_mut(),
            &live_superseded,
            live_controls.observation_taps,
        )
        .map_err(effect_diagnostics)?
    } else {
        Vec::new()
    };
    effect_controls.sort_unstable_by_key(|producer| {
        (
            strip_rank(&producer.track_id).unwrap_or(usize::MAX),
            producer.address,
        )
    });
    debug_assert!(
        effect_controls
            .iter()
            .all(|producer| strip_rank(&producer.track_id).is_some())
            && effect_controls.iter().all(|producer| {
                effect_producer_position(
                    &effect_controls,
                    strip_rank,
                    &producer.track_id,
                    producer.address,
                )
                .is_some_and(|index| core::ptr::eq(&effect_controls[index], producer))
            }),
        "every effect producer is found by its sorted position"
    );
    let mut artifact = GraphCompiler::compile_projected(graph_projection, builtins)
        .map_err(|failure| graph_diagnostics(&failure.diagnostics))?;
    // #1354 D4: the warm successor re-checks C1 against its own compile, and counts its prime
    // over its own source-claim lines (D3).
    let warm_prime_bytes = match (successor, warm, &warm_restart, &warm_predecessor) {
        (Some(base), Some(lead), Some(restart), Some(predecessor)) => {
            let arrivals = artifact_arrivals(&artifact);
            if first_late_node(
                artifact.graph(),
                &arrivals,
                &base.inventory.input_floors,
                predecessor,
                restart,
                lead.lead_samples,
            )
            .is_some()
            {
                return Err(warm_invariant());
            }
            if lead.lead_samples == 0 {
                0
            } else {
                prime_bytes_of(
                    lead.lead_samples,
                    carried_channels,
                    base.committed,
                    model,
                    &carried_ids,
                    &base.inventory.strip_delays,
                    artifact.graph().track_delays(),
                )
                .ok_or_else(|| resource("host.resource.arithmetic"))?
            }
        }
        _ => 0,
    };
    // Issue #1221 D1: one live lane per route into a submix, attached before bind and before the
    // caps below, so its charge is refused by the same checks as the effect channels. Without a
    // control channel no six-word lane is attached. #1652 D1 (#1652c): a following route gets
    // its seven-word lane on every plan, whatever the live options, and no six-word lane. A plan
    // with neither binds exactly what it bound before.
    // Issue #1363 D1 (a), (c): the routes this successor adds during playback fade in, on the
    // same terms as #1288's strips (a plan that attaches live lanes, never at a declared
    // discontinuity). Each one this plan gives no lane binds as a transition route (D2).
    let route_fades: Vec<RouteFade<'_>> = match successor {
        Some(base) if attach_live && !base.discontinuity => {
            let added = added_strips(base.committed, model);
            let watch = &base.inventory.fade_watch;
            let unfired = |owner: FadeArmOwner| {
                base.inventory
                    .fade_arms
                    .iter()
                    .filter(move |row| row.owner == owner && !watch.fired(row.entry))
            };
            let waiting: Vec<&str> = unfired(FadeArmOwner::Strip).map(|row| &*row.id).collect();
            // #1877 D2: the routes whose predecessor entry has not fired, re-armed here.
            // #1953b: and a lane-less route whose entry fired but has not settled, kept unchanged:
            // its op still ramps, and a fold would step from its ramp to its constant. Re-armed
            // and carried, it takes the displaced op's state (#1877 D6) and settles where the
            // displaced entry would have. One kept at other values is not re-armed: its fading
            // copy takes the op mid-ramp and the kept route fades in afresh (#2077).
            let in_flight = |row: &&FadeArmInventoryRow| {
                row.owner == FadeArmOwner::Route
                    && watch.fired(row.entry)
                    && !watch.settled(row.entry)
                    && model.routes.iter().any(|route| {
                        route.id.as_str() == &*row.id
                            && !controls::route_rows_live(route)
                            && base.committed.routes.contains(route)
                    })
            };
            let waiting_routes: Vec<(&str, u32)> = unfired(FadeArmOwner::Route)
                .chain(base.inventory.fade_arms.iter().filter(in_flight))
                .map(|row| (&*row.id, row.entry))
                .collect();
            // #2069: a muted route's mute is live when the route binds a lane: a six-word one,
            // or a following one.
            let live_mute = |route: &session::Route| {
                (six_word && controls::route_rows_live(route))
                    || follow_cells.iter().any(|(id, _)| *id == route.id.as_str())
            };
            added_route_fades(
                base.committed,
                model,
                &added,
                &waiting,
                &waiting_routes,
                live_mute,
                |route| crossfades_values(&base, route),
            )
        }
        _ => Vec::new(),
    };
    // #1877 D6: a re-armed route the send join does not list (a transition route, which has no
    // lane) joins too when its values are unchanged, so its op carries the displaced op's state:
    // a fire the predecessor ran after this preparation read its watch then continues mid-ramp.
    let route_joins: Vec<RouteJoin<'_>> = match successor {
        Some(base) => {
            let mut joins = route_joins;
            for fade in route_fades
                .iter()
                .filter(|fade| fade.carried.is_some() && !controls::route_rows_live(fade.route))
            {
                let id = fade.route.id.as_str();
                let unchanged = base
                    .committed
                    .routes
                    .iter()
                    .any(|before| before == fade.route);
                if unchanged && !joins.iter().any(|join| join.id == id) {
                    joins.push(RouteJoin { id, retarget: None });
                }
            }
            joins
        }
        None => route_joins,
    };
    // #1953b: every fading copy binds as a transition route too.
    let transition: Vec<StableGraphId> = route_fades
        .iter()
        .filter(|fade| !(six_word && controls::route_rows_live(fade.route)))
        .map(|fade| fade.route.id.as_str())
        .chain(route_copies.iter().map(|copy| copy.route.id.as_str()))
        .filter_map(StableGraphId::parse)
        .collect();
    let (graph_route_controls, follow_route_controls, transition_routes) =
        if six_word || !follow_cells.is_empty() || !transition.is_empty() {
            let arrivals = artifact.report().input_arrivals.clone();
            let mut sources: BTreeMap<&str, _> = follow_cells.into_iter().collect();
            artifact
                .attach_route_live_controls_following(
                    &live_superseded,
                    six_word,
                    &transition,
                    |route_id| {
                        let (program, cells, vca_mutes, flags, continued_vca) =
                            sources.remove(route_id.as_str())?;
                        let node = GraphNodeId::Route {
                            route_id: route_id.clone(),
                        };
                        graph::GraphFollowRouteSource::new(
                            program,
                            cells,
                            vca_mutes,
                            graph::floored_input_arrival(&arrivals, &node),
                            follow_route_jumps,
                            flags,
                        )
                        .map(|source| source.with_continued_vca_mutes(continued_vca))
                    },
                )
                .map_err(|error| match error {
                    graph::GraphRouteControlError::AlreadyAttached
                    | graph::GraphRouteControlError::BoundRoute
                    | graph::GraphRouteControlError::CursorTable(_)
                    | graph::GraphRouteControlError::UnknownTransitionRoute => {
                        graph_failure("host.route_control.attach")
                    }
                })
                .and_then(|attached| {
                    // Every following route is a route into a submix, so the attach asks for each
                    // source once, and `new` refuses none (its cells are the program's).
                    if sources.is_empty() {
                        Ok(attached)
                    } else {
                        Err(graph_failure("host.route_control.attach"))
                    }
                })?
        } else {
            (
                Vec::new(),
                Vec::new(),
                graph::GraphTransitionRoutes::default(),
            )
        };
    if follow_route_controls.len()
        != model
            .routes
            .iter()
            .filter(|route| follows_automated_mute(view, route))
            .count()
    {
        return Err(graph_failure("host.route_control.attach"));
    }
    // #1391 decision 1: the graph keys its six-word lanes on the stage before the fader and the
    // model its live rows on `controls::route_rows_live`. The two must name the same routes: every
    // live route that does not follow, in canonical route-ID order, and no other.
    if six_word {
        let mut expected: Vec<&str> = model
            .routes
            .iter()
            .filter(|route| {
                controls::route_rows_live(route) && !follows_automated_mute(view, route)
            })
            .map(|route| route.id.as_str())
            .collect();
        expected.sort_unstable();
        if !graph_route_controls
            .iter()
            .map(|producer| &*producer.route_id)
            .eq(expected)
        {
            return Err(graph_failure("host.route_control.attach"));
        }
    }
    let route_control_resources =
        graph::route_control_resources(&graph_route_controls, &follow_route_controls)
            .with_transition_routes(&transition_routes);
    // The producers in canonical route-ID order; a carried send's retarget is written through them.
    let mut route_controls: Vec<RouteControlProducer> = graph_route_controls
        .into_iter()
        .map(RouteControlProducer::new)
        .collect();
    let mut follow_route_controls = follow_route_controls;
    // #1285 D4: the floored output arrival, the one the plan renders.
    let output_latency = artifact.report().output_latency;
    let output_tail_every_peak = artifact.report().output_tail_every_peak;
    let input_floors: Box<[(graph::GraphNodeId, u64)]> = artifact
        .report()
        .input_arrivals
        .iter()
        .map(|(node, arrival)| (node.clone(), arrival.0))
        .collect();
    let raised_arrivals: Box<[(graph::GraphNodeId, u64)]> = artifact
        .report()
        .raised_arrivals
        .iter()
        .map(|(node, arrival)| (node.clone(), arrival.0))
        .collect();
    let mut graph_resources = artifact.graph_resource_estimate().clone();
    graph_resources.session_plus_plan_bytes = graph_resources
        .session_plus_plan_bytes
        .checked_add(plan_cell_bytes)
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    // Issue #2199: the plan's graph row and builtins row are the projection's, exactly (the
    // compile lowers the plan the projection estimated, from the builtins it projected with).
    if graph_resources.session_plus_plan_bytes != host_projection.graph_session_plus_plan_bytes
        || builtin_resources.engine_owned_retained_payload_bytes
            != host_projection.builtin_retained_payload_bytes
    {
        return Err(platform("host.graph.projection"));
    }
    let graph_and_model_without_effect_controls = graph_resources
        .session_plus_plan_bytes
        .checked_add(session_resources.compiled_model_bytes)
        .and_then(|bytes| bytes.checked_add(spectrum_capture_retained_bytes))
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let native_effect_control_resources = effect_control_resources(&effect_controls)
        .map_err(|_| resource("host.effect.resource.arithmetic"))?;
    let admitted_graph_and_model = graph_and_model_without_effect_controls
        .checked_add(
            native_effect_control_resources
                .total_bytes()
                .ok_or_else(|| resource("host.effect.resource.arithmetic"))?,
        )
        .and_then(|bytes| bytes.checked_add(route_control_resources.total_bytes))
        .and_then(|bytes| bytes.checked_add(carry_program_bytes))
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    if admitted_graph_and_model > caps.maximum_graph_session_plus_plan_bytes {
        return Err(resource("host.graph.resource.limit"));
    }
    if native_effect_control_resources
        .largest_allocation_bytes()
        .max(route_control_resources.largest_allocation_bytes)
        > caps.maximum_named_allocation_bytes
    {
        return Err(resource("host.resource.limit"));
    }
    // #1641 D1: the timeline's history covers the largest floored input arrival, `K = ⌈A_max /
    // q⌉ + 1` blocks (one block for a plan with no latency).
    let quantum = artifact.envelope().quantum;
    let largest_arrival = artifact
        .report()
        .input_arrivals
        .iter()
        .map(|(_, arrival)| arrival.0)
        .max()
        .unwrap_or(0);
    let history_blocks = largest_arrival
        .div_ceil(u64::from(quantum.0.max(1)))
        .checked_add(1)
        .and_then(|blocks| usize::try_from(blocks).ok())
        .and_then(NonZeroUsize::new)
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let timeline = if timeline_carries {
        // D3, D7: a vacant timeline owns only its history; the command queue and the seek-report
        // record (#1704 D4) its carry brings are counted with the carried source overhead, as a
        // carried ring's are.
        let (consumer, report) =
            SourceTimeline::prepare_vacant(quantum, history_blocks, published_generation.clone())
                .map_err(|_| resource("host.source.prepare"))?;
        let carried_timeline = report
            .command_queue_bytes
            .checked_add(report.seek_report_bytes)
            .ok_or_else(|| resource("host.resource.arithmetic"))?;
        carried_total_bytes = carried_total_bytes
            .checked_add(carried_timeline)
            .ok_or_else(|| resource("host.resource.arithmetic"))?;
        carried_overhead_bytes = carried_overhead_bytes
            .checked_add(carried_timeline)
            .ok_or_else(|| resource("host.resource.arithmetic"))?;
        carried_largest_bytes = carried_largest_bytes
            .max(report.command_queue_largest_allocation_bytes)
            .max(report.seek_report_bytes);
        consumer
    } else {
        let (producer, consumer, _) =
            SourceTimeline::prepare(quantum, history_blocks, published_generation.clone())
                .map_err(|_| resource("host.source.prepare"))?;
        sources.set_timeline(producer);
        consumer
    };
    let source_set = prepare_session_graph_source_set(
        artifact.envelope(),
        graph_sources,
        timeline,
        published_generation.clone(),
        mappings,
    )
    .map_err(|_| graph_failure("host.source.graph.prepare"))?;
    let source_resources = source_set.resource_report();
    // Issue #1272 D4: a plan's source caps count the rings it allocated plus the rings it will
    // carry, so the active plan never exceeds them after the swap.
    let source_total_with_carried = source_resources
        .total_engine_owned_bytes
        .checked_add(carried_total_bytes)
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let source_overhead_with_carried = source_resources
        .overhead_bytes
        .checked_add(carried_overhead_bytes)
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    if source_total_with_carried > caps.maximum_source_total_bytes
        || source_overhead_with_carried > caps.maximum_source_overhead_bytes
    {
        return Err(resource("host.source.resource.limit"));
    }
    let largest_engine_allocation_bytes = graph_resources
        .largest_allocation_bytes
        .max(graph::jump_length_largest_allocation_bytes())
        .max(source_resources.largest_allocation_bytes)
        .max(carried_largest_bytes)
        .max(builtin_resources.maximum_single_allocation_bytes)
        .max(spectrum_resources.map_or(0, |resources| resources.largest_allocation_bytes));
    if largest_engine_allocation_bytes.max(session_resources.single_allocation_bytes)
        > caps.maximum_named_allocation_bytes
        || builtin_resources.engine_owned_retained_payload_bytes
            > caps.maximum_builtin_retained_bytes
    {
        return Err(resource("host.resource.limit"));
    }

    let graph_nodes = spectrum_request.is_some().then(|| {
        artifact
            .graph()
            .spec
            .nodes
            .iter()
            .map(|node| node.id.clone())
            .collect::<Vec<_>>()
    });
    let (spectrum_observers, spectrum_capture) = match spectrum_request {
        None => (Vec::new(), None),
        Some(SpectrumPreparation::Single(request)) => {
            let (observer, capture) = crate::spectrum::prepare_capture(
                request,
                graph_nodes.as_deref().expect("spectrum graph nodes"),
                caps.maximum_named_allocation_bytes,
            )
            .map_err(spectrum_diagnostics)?;
            (
                vec![observer],
                Some(PreparedSpectrumCaptures::Single(capture)),
            )
        }
        Some(SpectrumPreparation::Collection(request)) => {
            let (observers, capture) = prepare_capture_collection(
                request,
                graph_nodes.as_deref().expect("spectrum graph nodes"),
                caps.maximum_named_allocation_bytes,
            )
            .map_err(spectrum_diagnostics)?;
            (
                observers,
                Some(PreparedSpectrumCaptures::Collection(capture)),
            )
        }
    };

    let bindings = GraphRuntimeBindings {
        envelope: artifact.envelope(),
        nodes: artifact
            .external_binding_nodes()
            .filter(|node| {
                !matches!(
                    node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    }
                )
            })
            .cloned()
            .map(GraphNodeBinding::identity)
            .collect(),
        observers: spectrum_observers,
    };
    let bound = artifact
        .into_bound_with_source_set(bindings, source_set)
        .map_err(|failure| graph_failure(failure.code))?;
    // Issue #137 D2: exactly the requested live-control halves come back, in canonical track order.
    // A session that produced a meter or control channel nobody asked for is still a hard
    // rejection -- that was the pre-#137 rule and it is what keeps an unrequested observer from
    // silently entering a production plan.
    if bound.meter_consumers.len() != meter_requests.len()
        || bound.track_controls.len() != control_requests.len()
    {
        return Err(graph_failure("host.meter.unexpected"));
    }
    // Canonical normalized track order is the live controls' addressing authority, so both halves
    // are permuted into it here rather than into whatever order preparation happened to build them.
    // A channel whose track is not in that order at all is a hard rejection, never a silent drop.
    // It covers every strip (issue #1207 D1): every strip has a control (issue #1211 D1), and a
    // selected meter may observe a submix (issue #1208 D1).
    let canonical_index: std::collections::BTreeMap<&str, usize> = live_control_strips
        .iter()
        .enumerate()
        .map(|(index, track)| (&**track, index))
        .collect();
    let mut strip_controls = bound.track_controls;
    let mut meters = bound.meter_consumers;
    if strip_controls
        .iter()
        .any(|value| !canonical_index.contains_key(&*value.track_id))
    {
        return Err(graph_failure("host.control.order"));
    }
    if meters
        .iter()
        .any(|value| !canonical_index.contains_key(&*value.track_id))
    {
        return Err(graph_failure("host.meter.order"));
    }
    strip_controls.sort_by_key(|value| canonical_index[&*value.track_id]);
    if selected_meters.is_some() {
        if meters
            .iter()
            .zip(&meter_requests)
            .any(|(actual, requested)| {
                actual.handle != requested.request.handle
                    || actual.track_id.as_ref() != requested.request.track_id
                    || actual.tap != requested.request.tap
            })
        {
            return Err(graph_failure("host.meter.order"));
        }
    } else {
        meters.sort_by_key(|value| canonical_index[&*value.track_id]);
    }

    // Issue #143 R7: walked over the built runtime, not derived from the request. A plan that
    // bound nothing reports zero because it *holds* nothing.
    let observation_retained_bytes = bound.plan.observation_retained_bytes();
    let control_retained_bytes = sources
        .retained_bytes()
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let mut plan = bound.plan;
    // #1639 D1: the plan's jump-length cell, seeded with the session's lengths, so the plan holds
    // them from its first block. A successor is seeded from the committed model, which holds any
    // edit of them (D4): nothing carries.
    let jump_lengths =
        graph::attach_jump_lengths(&mut plan, session_jump_lengths(model), &live_superseded)
            .map_err(|_| graph_failure("host.jump_lengths.attach"))?;
    // Issue #1272 D4: the join's program goes in before the plan leaves preparation.
    let has_carry_program = carry_program.is_some();
    if let Some(program) = carry_program {
        graph::install_carry_program(&mut plan, program)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
    // Issue #1276 D6: this plan's own input lanes, sorted by strip ID: the inventory its
    // successor joins with, and the table the join below looks this plan's lanes up in.
    let mut input_sections: Vec<InputSectionInventoryRow> = graph::builtin_input_lanes(&mut plan)
        .ok_or_else(|| graph_failure("host.inventory.identity"))?
        .into_iter()
        .map(|(id, location)| InputSectionInventoryRow {
            id,
            location,
            live: live_input,
        })
        .collect();
    input_sections.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    // Issue #1327 D1: this plan's own meters, in reader order: the inventory its successor joins
    // with, and the rows the join below looks this plan's observers up in.
    let observer_rows =
        graph::observer_rows(&mut plan).ok_or_else(|| graph_failure("host.inventory.identity"))?;
    let meter_rows: Vec<MeterInventoryRow> = {
        let locations: BTreeMap<u64, graph::GraphObserverLocation> = observer_rows
            .iter()
            .filter(|row| row.kind == engine::realtime::ObserverKind::Meter)
            .map(|row| (row.handle, row.location))
            .collect();
        let requested: BTreeMap<u64, &SelectedMeterRequest> = meter_requests
            .iter()
            .map(|item| (item.request.handle.0.get(), item))
            .collect();
        meters
            .iter()
            .enumerate()
            .filter_map(|(entry, consumer)| {
                let handle = consumer.handle.0.get();
                let item = requested.get(&handle)?;
                Some(MeterInventoryRow {
                    id: consumer.track_id.clone(),
                    tap: consumer.tap,
                    metrics: item.metrics,
                    config: MeterConfigKey::of(&item.request.config),
                    entry,
                    location: *locations.get(&handle)?,
                })
            })
            .collect()
    };
    debug_assert_eq!(
        meter_rows.len(),
        meters.len(),
        "every meter has an observer"
    );
    let meter_carry = match successor {
        Some(base) => resolve_meter_carry(&base.inventory.meters, &meter_rows, |id| {
            same_strip_id_kind(base.committed, compiled.normalized_model(), id)
        }),
        None => MeterCarry::default(),
    };
    // Issue #1395 D1: this plan's own spectrum captures, in capture order: the inventory its
    // successor joins with, and the rows the join below looks this plan's observers up in. A
    // single capture is entry 0; a collection's entries are its entries in order.
    let spectrum_locations: BTreeMap<u64, graph::GraphObserverLocation> = observer_rows
        .iter()
        .filter(|row| row.kind == engine::realtime::ObserverKind::Spectrum)
        .map(|row| (row.handle, row.location))
        .collect();
    let spectrum_row = |entry: usize,
                        target: &crate::spectrum::SpectrumTarget,
                        channels: crate::spectrum::SpectrumChannels|
     -> Result<SpectrumInventoryRow, PrepareDiagnostics> {
        let location = crate::spectrum::observer_handle(entry)
            .and_then(|handle| spectrum_locations.get(&handle).copied())
            .ok_or_else(|| graph_failure("host.inventory.identity"))?;
        Ok(SpectrumInventoryRow {
            target: target.clone(),
            channels,
            entry,
            location,
        })
    };
    let spectrum_rows: Vec<SpectrumInventoryRow> = match &spectrum_capture {
        Some(PreparedSpectrumCaptures::Single(capture)) => {
            vec![spectrum_row(0, capture.target(), capture.channels())?]
        }
        Some(PreparedSpectrumCaptures::Collection(collection)) => (0..collection.len())
            .map(|entry| {
                let (target, channels) = collection
                    .entry(entry)
                    .ok_or_else(|| graph_failure("host.inventory.identity"))?;
                spectrum_row(entry, target, channels)
            })
            .collect::<Result<_, _>>()?,
        None => Vec::new(),
    };
    let spectrum_collection = matches!(
        &spectrum_capture,
        Some(PreparedSpectrumCaptures::Collection(_))
    );
    let spectrum_carry = match successor {
        Some(base) => resolve_spectrum_carry(
            base.inventory,
            &spectrum_rows,
            spectrum_collection,
            compiled.sample_rate().0,
            compiled.quantum().0,
            |target| same_strip_kind(target, base.committed, compiled.normalized_model()),
        ),
        None => SpectrumCarry::default(),
    };
    // Issue #1277 D1: this plan's own fader and matrix lanes, sorted by strip ID and owner: the
    // inventory its successor joins with, and the table the join below looks this plan's lanes up
    // in.
    let mut strip_stages: Vec<StripStageInventoryRow> = graph::builtin_strip_lanes(&mut plan)
        .ok_or_else(|| graph_failure("host.inventory.identity"))?
        .into_iter()
        .map(|(id, lane)| StripStageInventoryRow {
            id,
            owner: lane.owner,
            stage: lane.stage,
            location: lane.location,
            live: live_strips,
        })
        .collect();
    strip_stages
        .sort_unstable_by(|left, right| left.id.cmp(&right.id).then(left.owner.cmp(&right.owner)));
    let mut strip_carry = match successor {
        Some(base) => resolve_strip_carry(
            base.inventory,
            &strip_joins,
            &strip_stages,
            live_strips,
            &restarts_whole,
        ),
        None => StripCarry::default(),
    };
    // Issue #1279 D1: this plan's own effect bank lanes, sorted by owner key: the inventory its
    // successor joins with, and the table the join below looks this plan's lanes up in.
    let mut effect_lanes: Vec<EffectLaneInventoryRow> = graph::bank_effect_lanes(&mut plan)
        .ok_or_else(|| graph_failure("host.inventory.identity"))?
        .into_iter()
        .map(|(id, lane)| EffectLaneInventoryRow {
            id,
            rack: lane.rack,
            effect_id: lane.effect_id,
            location: lane.location,
            layout: lane.layout,
            kind: lane.kind,
        })
        .collect();
    effect_lanes.sort_unstable_by(|left, right| {
        left.id
            .cmp(&right.id)
            .then(left.rack.cmp(&right.rack))
            .then_with(|| left.effect_id.cmp(&right.effect_id))
    });
    // Issue #1280 D4: a carried lane's live difference is opened against the successor's own
    // producer here, so a lane whose producer has no cell for a record restarts instead of
    // writing it (and a preparation that fails later writes nothing).
    let mut effect_carry = match successor {
        Some(base) => resolve_effect_carry(
            base.inventory,
            &chain_joins,
            &effect_lanes,
            &mut |strip_id, address, instance| {
                let position =
                    effect_producer_position(&effect_controls, strip_rank, strip_id, address)?;
                prepare_effect_retarget(&mut effect_controls[position], position, instance)
            },
        ),
        None => EffectCarry::default(),
    };
    // Issue #1327 D1: this plan's own effect observation readers, in reader order: the inventory
    // its successor joins with. A tap follows its effect instance, so a row is keyed by the
    // instance's lane, and its successor's carry follows the effect moves resolved above.
    let observation_rows: Vec<ObservationInventoryRow> = effect_observations
        .iter()
        .enumerate()
        .filter_map(|(entry, handle)| {
            let racks: &[graph::RackId] = match handle.address.rack {
                LiveEffectRack::Console => &[graph::RackId::Simd1, graph::RackId::Simd2],
                LiveEffectRack::Inserts => &[graph::RackId::Dynamic],
            };
            let lane = racks.iter().find_map(|&rack| {
                effect_lane_row(&effect_lanes, &handle.track_id, rack, &handle.effect_id)
            })?;
            Some(ObservationInventoryRow {
                entry,
                location: lane.location,
                taps: handle.readers.len(),
            })
        })
        .collect();
    // Issue #1283 D1: this plan's own compensation lines, sorted by edge ID: the inventory its
    // successor joins with, and the lines its own moves are resolved from.
    let mut delay_lines = graph::take_compensation_lines(&mut plan)
        .ok_or_else(|| graph_failure("host.inventory.identity"))?;
    delay_lines.sort_unstable_by(|left, right| left.edge.cmp(&right.edge));
    let mut delay_carry = match successor {
        Some(base) => resolve_delay_carry(&base.inventory.delay_lines, &delay_lines, |id| {
            same_strip_id_kind(base.committed, compiled.normalized_model(), id)
        }),
        None => Vec::new(),
    };
    // #1953b: each fading copy's lines are the lines of the route it replaces.
    if let Some(base) = successor {
        alias_copy_lines(
            &base.inventory.delay_lines,
            &delay_lines,
            &route_copies,
            &mut delay_carry,
        );
    }
    // Issue #1284 D1, D2: this plan's own strip input alignment lines and live sends, sorted by ID:
    // the inventory its successor joins with, and the rows its own moves are resolved from.
    let mut strip_delays = graph::take_strip_delay_lines(&mut plan)
        .ok_or_else(|| graph_failure("host.inventory.identity"))?;
    strip_delays.sort_unstable_by(|left, right| left.strip.cmp(&right.strip));
    let mut live_routes = graph::take_live_routes(&mut plan)
        .ok_or_else(|| graph_failure("host.inventory.identity"))?;
    live_routes.sort_unstable_by(|left, right| left.route_id.cmp(&right.route_id));
    let mut strip_delay_carry = match successor {
        Some(base) => resolve_strip_delay_carry(
            &base.inventory.strip_delays,
            &strip_delays,
            &delay_joins,
            warm.map_or(0, |lead| lead.lead_samples),
        ),
        None => StripDelayCarry::default(),
    };
    // #1324 D1, D2: the strips this successor restarts, in one place, after every join: each
    // strip a join restarts (#1277 D6), and each strip a warm lead restarts whole (#1354a ruling
    // 2). Each is duck-swapped, so the whole-strip restart filter ([`StripRestart`]) drops every
    // carry in front of its fader and of its fader itself: a partial carry upstream of a
    // restarted owner would leave a hole with a step at each edge. The joins' own rules stay as
    // they are; only their outputs are filtered here.
    let mut restarted = core::mem::take(&mut strip_carry.restarted);
    restarted.extend(core::mem::take(&mut effect_carry.restarted));
    // #1324 D1: a strip whose delay line restarts steps its `input` tap at `S`.
    let mut delay_restarted = core::mem::take(&mut strip_delay_carry.restarted);
    delay_restarted.sort_unstable();
    delay_restarted.dedup();
    restarted.extend(delay_restarted.iter().cloned());
    restarted.sort_unstable();
    restarted.dedup();
    if let Some(lead) = warm {
        // #1354a ruling 2, #1354b: every strip of the lead's `R` restarts, its join part (whose
        // floors are its arrivals) whatever this join did, and a strip restarted whole whatever
        // its owners did.
        restarted.extend(
            lead.restarted
                .iter()
                .filter(|id| committed_has_strip(model, id))
                .cloned()
                .chain(
                    lead.restart_whole
                        .iter()
                        .filter(|id| committed_has_strip(model, id))
                        .map(|id| Box::<str>::from(id.as_str())),
                ),
        );
        restarted.sort_unstable();
        restarted.dedup();
    }
    // #1397 D2 step 1: the transition fallback's duck set, restarted as the join's restarts are.
    if let Some(base) = successor.filter(|base| !base.forced_restart.is_empty()) {
        restarted.extend(
            base.forced_restart
                .iter()
                .filter(|id| {
                    committed_has_strip(base.committed, id) && committed_has_strip(model, id)
                })
                .map(|id| Box::<str>::from(id.as_str())),
        );
        restarted.sort_unstable();
        restarted.dedup();
    }
    // #1324 D1: a carried effect keyed from a restarted strip's tap behind a restarted owner
    // would read the old chain until `S` and the restarted chain's at-rest output after it, so
    // its strip joins the set, until no strip is added. A warm lead's `R` is closed already
    // (`warm_lead`'s step 2 runs this rule on the same compile).
    if let Some(base) = successor {
        close_over_sidechains(
            base.committed,
            model,
            &base.inventory.delay_lines,
            &delay_lines,
            &delay_restarted,
            &mut restarted,
        );
    }
    // #1354a ruling 2: neither the joins nor the closure restart a strip outside the lead's `R`,
    // whose floors are derived from it.
    if let (Some(lead), Some(restart)) = (warm, &warm_restart)
        && restarted
            .iter()
            .any(|id| !restart.whole.contains(&**id) && lead.restarted.binary_search(id).is_err())
    {
        return Err(warm_invariant());
    }
    let restart = StripRestart::new(model, restarted.iter().map(|id| &**id));
    let carried_input_count = carried_inputs.len();
    let route_joins: Vec<RouteJoin<'_>> = if restarted.is_empty() {
        route_joins
    } else {
        // The fader's move and its retarget; the matrix's stay.
        for id in &restarted {
            if let Some(lane) = strip_stage_row(&strip_stages, id, graph::BuiltinStripOwner::Fader)
            {
                let location = lane.location;
                strip_carry.moves.retain(|step| {
                    !(step.owner == graph::BuiltinStripOwner::Fader && step.successor == location)
                });
            }
        }
        strip_carry.retargets.retain_mut(|retarget| {
            if restart.contains(retarget.id) {
                retarget.fader = [None; 4];
                retarget.offsets = [None; 2];
            }
            retarget.fader[0].is_some()
                || retarget.offsets != [None; 2]
                || retarget.matrix.is_some()
        });
        // Every effect of the strip sits before its fader (console slots and inserts). A dropped
        // retarget that opened an owner transaction on its producer discards it, so the producer
        // is left as preparation built it.
        let restarted_sites: Vec<graph::GraphEffectSite> = effect_lanes
            .iter()
            .filter(|row| restart.contains(&row.id))
            .map(|row| row.location)
            .collect();
        effect_carry
            .moves
            .retain(|step| !restarted_sites.contains(&step.successor));
        effect_carry.retargets.retain(|retarget| {
            let producer = &mut effect_controls[retarget.producer];
            let keep = !restart.contains(&producer.track_id);
            if !keep && retarget.targets.is_some() {
                let _ = producer.discard_owner();
            }
            keep
        });
        effect_carry.scratch_bytes = effect_carry
            .moves
            .iter()
            .filter(|step| {
                !matches!(
                    (step.successor, step.predecessor),
                    (
                        graph::GraphEffectSite::Node(_),
                        graph::GraphEffectSite::Node(_)
                    )
                )
            })
            .filter_map(|step| {
                effect_lanes
                    .iter()
                    .find(|row| row.location == step.successor)
            })
            .map(|row| row.layout.payload_bytes())
            .max()
            .unwrap_or(0);
        carried_inputs.retain(|join| !restart.contains(join.id));
        let dropped: BTreeSet<u32> = delay_lines
            .iter()
            .filter(|line| restart.drops_line(line))
            .map(|line| line.line)
            .collect();
        delay_carry.retain(|step| !dropped.contains(&step.successor));
        // A route from a tap before the fader restarts its live lane: phase 1 ducked it, and the
        // successor arms it (#1363 D1 (c)).
        route_joins
            .into_iter()
            .filter(|join| !restart.restarts_route(join.id))
            .collect()
    };
    let mut route_carry = match successor {
        Some(base) => resolve_route_carry(&base.inventory.live_routes, &live_routes, &route_joins),
        None => RouteCarry::default(),
    };
    // #1953b: a fading copy takes the op state of the route it replaces when the predecessor
    // gave that route a live op (a transition route in flight, or a kept copy), so its fade-out
    // starts where that route is. A copy out of a restarted strip takes none: phase 1 ducked the
    // strip, so its tap is silent at the swap and the copy mutes at once.
    if let Some(base) = successor {
        let row = |rows: &[graph::GraphLiveRoute], id: &str| {
            rows.binary_search_by(|row| (*row.route_id).cmp(id))
                .ok()
                .map(|index| (rows[index].unit, rows[index].following))
        };
        for copy in &route_copies {
            let (strip, _, _) = source_strip(&copy.route.source);
            if restart.contains(strip) {
                continue;
            }
            if let (Some((successor, _)), Some((predecessor, follows))) = (
                row(&live_routes, copy.route.id.as_str()),
                row(&base.inventory.live_routes, &copy.alias),
            ) {
                route_carry.moves.push(graph::GraphRouteMove {
                    successor,
                    predecessor,
                    predecessor_follows: follows,
                });
            }
        }
    }
    // Issue #1324 D3, #1363 D1 (c): a duck-swapped strip's routes from a tap before its fader
    // are armed with it and wait for its fade, as an added strip's are. Only a route with a
    // live op can arm, a six-word one (the route lanes of #1391) or one that follows an
    // automated source mute (#2011).
    // The duck-swapped strips the successor arms, on the terms of an added strip's arm (#1288
    // D1, D2): a plan that attaches live lanes, never at a declared discontinuity.
    let armed_restarts: BTreeSet<&str> = match successor {
        Some(base) if attach_live && !base.discontinuity => {
            restarted.iter().map(|id| &**id).collect()
        }
        _ => BTreeSet::new(),
    };
    let route_fades: Vec<RouteFade<'_>> = match successor {
        Some(base) if !armed_restarts.is_empty() => {
            let mut fades = route_fades;
            let added = added_strips(base.committed, model);
            for route in &model.routes {
                let (source, tap) = match &route.source {
                    session::RouteSource::Track { track_id, tap } => (track_id.as_str(), *tap),
                    session::RouteSource::Submix { submix_id, tap } => (submix_id.as_str(), *tap),
                };
                if !armed_restarts.contains(source) || !tap.precedes_fader() {
                    continue;
                }
                if let Some(fade) = fades
                    .iter_mut()
                    .find(|fade| fade.route.id.as_str() == route.id.as_str())
                {
                    fade.strip = Some(source);
                    // #1877, #1392 D2: a restarted strip's pending entry is replaced by the fresh
                    // arm, never carried beside it; the send's entry goes with the fader's.
                    fade.carried = None;
                    continue;
                }
                let into_added = matches!(&route.destination,
                    session::RouteDestination::SubmixInput { submix_id }
                        if added.contains(&submix_id.as_str()));
                let has_op = live_routes
                    .binary_search_by(|row| (*row.route_id).cmp(route.id.as_str()))
                    .is_ok();
                // #2069: a route the session mutes arms too; its op holds a lane (a muted route
                // is never a transition route outside `route_fades`), so its mute is live.
                if into_added || !has_op {
                    continue;
                }
                fades.push(RouteFade {
                    route,
                    strip: Some(source),
                    carried: None,
                });
            }
            fades
        }
        _ => route_fades,
    };
    // Issue #1288 D1: a strip the successor adds during playback fades in. Only a plan that
    // attaches live lanes arms (decision 1: the plain entry binds a fader that cannot fade), and
    // never at a declared discontinuity (decision 2: every owner restarts, and a seek is not
    // declicked).
    // Issue #1324 D3: a duck-swapped strip (one in `restarted`, present in both plans) is armed
    // afresh, with this plan's `D`: its fader restarts (D2).
    // Issue #1392 D2: a strip whose fade entry the predecessor's table holds is armed again, and
    // takes that entry's render state at the swap block, exactly when its fader lane carries. A
    // restarted strip's fader does not carry, so its pending entry is replaced by the fresh arm
    // above, never carried beside it. Each strip is paired with the predecessor entry it
    // carries, `None` for an added or a restarted strip.
    let fade_strips: Vec<(&str, Option<u32>)> = match successor {
        Some(base) if attach_live && !base.discontinuity => {
            let mut strips: Vec<(&str, Option<u32>)> = added_strips(base.committed, model)
                .into_iter()
                .map(|id| (id, None))
                .collect();
            strips.extend(armed_restarts.iter().map(|id| (*id, None)));
            // A row whose entry render has fired is dropped: the strip's lane imports `armed =
            // false`, so the successor needs no entry and never fades it again.
            let watch = &base.inventory.fade_watch;
            strips.extend(base.inventory.fade_arms.iter().filter_map(|row| {
                if row.owner != FadeArmOwner::Strip || watch.fired(row.entry) {
                    return None;
                }
                let lane =
                    strip_stage_row(&strip_stages, &row.id, graph::BuiltinStripOwner::Fader)?;
                strip_carry
                    .moves
                    .iter()
                    .any(|step| {
                        step.owner == graph::BuiltinStripOwner::Fader
                            && step.successor == lane.location
                    })
                    .then_some((&*row.id, Some(row.entry)))
            }));
            strips
        }
        _ => Vec::new(),
    };
    let ids: Vec<&str> = fade_strips.iter().map(|(id, _)| *id).collect();
    let fade_arms = fade_arms_for(model, &ids, &strip_stages, &input_floors);
    let carried_arms = fade_strips
        .iter()
        .filter(|(_, from)| from.is_some())
        .count()
        + route_fades
            .iter()
            .filter(|fade| fade.carried.is_some())
            .count();
    // #1363 D3: the route arms join the strips' table after them, one entry each.
    // #1953b: and each fading copy's fade-out after them.
    let carried_arms = carried_arms
        + route_copies
            .iter()
            .filter(|copy| copy.carried.is_some())
            .count();
    let fade_arm_bytes = graph::fade_arm_table_bytes(
        fade_arms.len() + route_fades.len() + route_copies.len(),
        carried_arms,
    )
    .ok_or_else(|| resource("host.resource.arithmetic"))?;
    // The strip moves' table is charged at its exact length: which strips carry is known only
    // once both plans' lanes are, so it is checked here, against the same cap as the rest of the
    // plan, instead of before bind.
    let strip_carry_bytes = u64::try_from(
        strip_carry
            .moves
            .len()
            .checked_mul(core::mem::size_of::<graph::GraphStripMove>())
            .ok_or_else(|| resource("host.resource.arithmetic"))?,
    )
    .map_err(|_| resource("host.resource.arithmetic"))?;
    if admitted_graph_and_model
        .checked_add(strip_carry_bytes)
        .ok_or_else(|| resource("host.resource.arithmetic"))?
        > caps.maximum_graph_session_plus_plan_bytes
    {
        return Err(resource("host.graph.resource.limit"));
    }
    let observation_carry = match successor {
        Some(base) => resolve_observation_carry(
            &base.inventory.observations,
            &observation_rows,
            &effect_carry.moves,
        ),
        None => Vec::new(),
    };
    let meter_carry_bytes = graph::observer_carry_retained_bytes(
        meter_carry.moves.len() + spectrum_carry.moves.len(),
        meter_carry.joins.len(),
        observation_carry.len(),
    )
    .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let delay_carry_bytes = graph::delay_carry_retained_bytes(delay_carry.len())
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    if !carried_inputs.is_empty() {
        let moves: Vec<graph::GraphLaneMove> = carried_inputs
            .iter()
            .filter_map(|join| {
                input_section_row(&input_sections, join.id).map(|row| graph::GraphLaneMove {
                    successor: row.location,
                    predecessor: join.predecessor,
                })
            })
            .collect();
        // Every strip banks its input section in a shipped plan, so every carried strip finds
        // its lane here; one that did not would silently restart at rest.
        debug_assert_eq!(moves.len(), carried_inputs.len());
        graph::install_builtin_input_carry(&mut plan, moves)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
    // #1355 D5, L3: a warm successor's `Input`-tap sidechain lines from a restarted track on a
    // moved source into a carried strip. `restarted` is every join's restart set and the lead's
    // whole restarts (#1324a), gathered above.
    let sidechain_fills = match (successor, warm) {
        (Some(base), Some(lead)) if lead.lead_samples != 0 => {
            let restarted: BTreeSet<&str> = restarted
                .iter()
                .map(|id| &**id)
                .chain(lead.restart_whole.iter().map(String::as_str))
                .collect();
            sidechain_fills(&base, &delay_lines, &delay_joins, &restarted)
        }
        _ => Vec::new(),
    };
    // D3's L3 term (#1354b): the words the prime writes into those lines.
    let warm_prime_bytes = match (successor, warm) {
        (Some(base), Some(lead)) if lead.lead_samples != 0 => {
            sidechain_fill_words(&base, &delay_lines, &sidechain_fills, lead.lead_samples)
                .and_then(|words| words.checked_mul(4)?.checked_add(warm_prime_bytes))
                .ok_or_else(|| resource("host.resource.arithmetic"))?
        }
        _ => warm_prime_bytes,
    };
    let prime_carry_bytes =
        graph::prime_carry_retained_bytes(strip_delay_carry.fills.len(), sidechain_fills.len())
            .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let strip_delay_carry_bytes =
        graph::strip_delay_carry_retained_bytes(strip_delay_carry.moves.len())
            .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let route_carry_bytes = graph::route_carry_retained_bytes(
        route_carry.moves.len(),
        route_carry
            .moves
            .iter()
            .filter(|step| step.predecessor_follows)
            .count(),
    )
    .ok_or_else(|| resource("host.resource.arithmetic"))?;
    // The effect moves' table and payload scratch are charged at their exact size, against the
    // same cap as the rest of the plan, and the scratch is one allocation (#1279 D3).
    // The carried slots of the successor lanes the moves name are allocated with the table, and
    // only for those lanes (#1633 D1).
    let effect_carry_slot_bytes = graph::effect_carry_slot_bytes(&mut plan, &effect_carry.moves)
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let effect_carry_bytes = graph::effect_carry_retained_bytes(
        effect_carry.moves.len(),
        effect_carry.scratch_bytes,
        effect_carry_slot_bytes,
    )
    .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let effect_carry_scratch_bytes = effect_carry.scratch_bytes;
    if effect_carry_scratch_bytes > caps.maximum_named_allocation_bytes {
        return Err(resource("host.resource.limit"));
    }
    if admitted_graph_and_model
        .checked_add(strip_carry_bytes)
        .and_then(|bytes| bytes.checked_add(effect_carry_bytes))
        .and_then(|bytes| bytes.checked_add(meter_carry_bytes))
        .and_then(|bytes| bytes.checked_add(delay_carry_bytes))
        .and_then(|bytes| bytes.checked_add(strip_delay_carry_bytes))
        .and_then(|bytes| bytes.checked_add(prime_carry_bytes))
        .and_then(|bytes| bytes.checked_add(route_carry_bytes))
        .and_then(|bytes| bytes.checked_add(fade_arm_bytes))
        .ok_or_else(|| resource("host.resource.arithmetic"))?
        > caps.maximum_graph_session_plus_plan_bytes
    {
        return Err(resource("host.graph.resource.limit"));
    }
    if !strip_carry.moves.is_empty() {
        graph::install_builtin_strip_carry(&mut plan, strip_carry.moves)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
    // Issue #1363 D3: each added route's arm, on its live route op. #1877 D1: each takes an
    // inventory row, so a later successor re-arms it and carries its entry, as a strip's.
    let route_arms = route_fade_arms(
        model,
        &route_fades,
        &live_routes,
        &delay_lines,
        &input_floors,
    )
    .ok_or_else(|| graph_failure("host.fade.install"))?;
    // #1953b: each fading copy's fade-out, from its first block, over the session mute ramp; it
    // settles once its destination line has delivered the fade. A copy out of a restarted strip
    // mutes at once (ramp 0): its tap is silent at the swap.
    let copy_arms = copy_fade_arms(model, &route_copies, &live_routes, &delay_lines, |strip| {
        restart.contains(strip)
    })
    .ok_or_else(|| graph_failure("host.fade.install"))?;
    let mut fade_arm_rows = Vec::new();
    let mut route_copy_rows = Vec::new();
    let mut fade_watch = graph::GraphFadeArmWatch::default();
    if !fade_arms.is_empty() || !route_arms.is_empty() || !copy_arms.is_empty() {
        let (strips, mut arms): (Vec<_>, Vec<_>) = fade_arms.into_iter().unzip();
        let route_count = route_arms.len();
        arms.extend(route_arms);
        arms.extend(copy_arms);
        let (entries, watch) = graph::install_fade_arms(&mut plan, arms)
            .map_err(|_| graph_failure("host.fade.install"))?;
        fade_watch = watch;
        let mut pairs = Vec::new();
        let strip_count = strips.len();
        for (id, entry) in strips.into_iter().zip(&entries) {
            let Some(entry) = *entry else {
                continue;
            };
            if let Some(&(_, Some(predecessor))) =
                fade_strips.iter().find(|(strip, _)| *strip == id)
            {
                pairs.push(graph::GraphFadeArmPair {
                    successor: entry,
                    predecessor,
                });
            }
            fade_arm_rows.push(FadeArmInventoryRow {
                id: id.into(),
                entry,
                owner: FadeArmOwner::Strip,
            });
        }
        // #1877 D1, D2: the route arms follow the strips' in `route_fades` order.
        for (fade, entry) in route_fades.iter().zip(entries.iter().skip(strip_count)) {
            let Some(entry) = *entry else {
                continue;
            };
            if let Some(predecessor) = fade.carried {
                pairs.push(graph::GraphFadeArmPair {
                    successor: entry,
                    predecessor,
                });
            }
            fade_arm_rows.push(FadeArmInventoryRow {
                id: fade.route.id.as_str().into(),
                entry,
                owner: FadeArmOwner::Route,
            });
        }
        // #1953b: the copies' arms follow the routes'.
        for (copy, entry) in route_copies
            .iter()
            .zip(entries.iter().skip(strip_count + route_count))
        {
            let Some(entry) = *entry else {
                continue;
            };
            if let Some(predecessor) = copy.carried {
                pairs.push(graph::GraphFadeArmPair {
                    successor: entry,
                    predecessor,
                });
            }
            route_copy_rows.push(RouteCopyRow {
                route: copy.route.clone(),
                entry,
            });
        }
        route_copy_rows.sort_unstable_by(|left: &RouteCopyRow, right: &RouteCopyRow| {
            left.route.id.as_str().cmp(right.route.id.as_str())
        });
        if !pairs.is_empty() {
            graph::install_fade_arm_carry(&mut plan, pairs)
                .map_err(|_| graph_failure("host.carry.install"))?;
        }
        fade_arm_rows
            .sort_unstable_by(|left, right| (&left.id, left.owner).cmp(&(&right.id, right.owner)));
    }
    // #1674 R1: the meter readers the control side pairs at promotion, in the swap block's record
    // order, read before the moves go to the plan.
    let meter_pairing: Box<[engine::realtime::ObserverCarry]> = meter_carry
        .moves
        .iter()
        .map(|step| engine::realtime::ObserverCarry {
            kind: step.kind,
            successor: step.successor_entry,
            predecessor: step.predecessor_entry,
        })
        .collect();
    if !effect_carry.moves.is_empty() {
        graph::install_effect_carry(&mut plan, effect_carry.moves, effect_carry_scratch_bytes)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
    if !meter_carry.moves.is_empty()
        || !meter_carry.joins.is_empty()
        || !spectrum_carry.moves.is_empty()
    {
        // One section holds every kind's moves (#1327 D2, #1395 D2); the meters' come first.
        let mut moves = meter_carry.moves;
        moves.extend(spectrum_carry.moves);
        graph::install_observer_carry(&mut plan, moves, meter_carry.joins)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
    if !observation_carry.is_empty() {
        graph::install_observation_carry(&mut plan, observation_carry)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
    if let Some(base) = successor
        && !(delay_carry.is_empty()
            && strip_delay_carry.moves.is_empty()
            && route_carry.moves.is_empty())
    {
        // A line or a send carries between two plans whose strips all restart or differ in a
        // prepared value, so the join above may have found nothing else to carry: the program then
        // names the predecessor and moves no source (it retains no byte).
        if !has_carry_program {
            graph::install_carry_program(
                &mut plan,
                graph::GraphCarryProgram {
                    predecessor: base.inventory.plan_identity(),
                    sources: Box::default(),
                    timeline: false,
                },
            )
            .map_err(|_| graph_failure("host.carry.install"))?;
        }
        if !delay_carry.is_empty() {
            graph::install_delay_carry(&mut plan, delay_carry)
                .map_err(|_| graph_failure("host.carry.install"))?;
        }
        if !strip_delay_carry.moves.is_empty() {
            graph::install_strip_delay_carry(&mut plan, strip_delay_carry.moves)
                .map_err(|_| graph_failure("host.carry.install"))?;
        }
        if !route_carry.moves.is_empty() {
            graph::install_route_carry(&mut plan, route_carry.moves)
                .map_err(|_| graph_failure("host.carry.install"))?;
        }
    }
    // #1355 D5: a warm successor fills its grown lines whenever it is adopted with a carry, so
    // none of its lines can stay at rest. Its prime replays `P / q` blocks.
    if let Some(lead) = warm.filter(|lead| lead.lead_samples != 0) {
        let blocks = u32::try_from(lead.lead_samples / u64::from(compiled.quantum().0))
            .map_err(|_| resource("host.resource.arithmetic"))?;
        graph::install_prime_carry(&mut plan, blocks, strip_delay_carry.fills, sidechain_fills)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
    let carry_program_retained_bytes = graph::carry_program_retained_bytes(&mut plan);
    // The admission charged the input moves of every strip the input join carries; the
    // restart filter (#1324 D2) may have dropped some since, and the program retains exactly
    // the rest.
    debug_assert_eq!(
        carry_program_retained_bytes,
        carry_program_bytes
            - ((carried_input_count - carried_inputs.len())
                * core::mem::size_of::<graph::GraphLaneMove>()) as u64
            + strip_carry_bytes
            + effect_carry_bytes
            + meter_carry_bytes
            + delay_carry_bytes
            + strip_delay_carry_bytes
            + prime_carry_bytes
            + route_carry_bytes
    );
    inventory_sources.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    // #1396 D2: an ordinary successor reads its sources as far ahead of its render clock as its
    // predecessor does, so the source-read clock never moves backwards across a rebuild. A first
    // plan reads on its render clock, and so does a discontinuity successor, whose base inventory
    // is `at_discontinuity`'s.
    // #1355 D1: a warm successor reads its sources its lead further ahead, `O + P`.
    let source_read_offset = successor.map_or(0, |base| {
        base.inventory.source_read_offset + warm.map_or(0, |lead| lead.lead_samples)
    });
    plan.set_source_read_offset(source_read_offset);
    let inventory = PlanStateInventory {
        plan_identity: graph::plan_identity(&mut plan)
            .ok_or_else(|| graph_failure("host.inventory.identity"))?,
        sources: inventory_sources.into_boxed_slice(),
        input_sections: input_sections.into_boxed_slice(),
        strip_stages: strip_stages.into_boxed_slice(),
        effect_lanes: effect_lanes.into_boxed_slice(),
        meters: meter_rows.into_boxed_slice(),
        observations: observation_rows.into_boxed_slice(),
        sample_rate_hz: compiled.sample_rate().0,
        quantum_frames: compiled.quantum().0,
        spectrum: spectrum_rows.into_boxed_slice(),
        spectrum_collection,
        spectrum_pairing: spectrum_carry.pairing.into_boxed_slice(),
        meter_pairing,
        delay_lines: delay_lines.into_boxed_slice(),
        strip_delays: strip_delays.into_boxed_slice(),
        live_routes: live_routes.into_boxed_slice(),
        follow_into_six_word: route_carry.follow_into_six_word,
        follow_into_following: route_carry.follow_into_following,
        follow_loses_a_curve: route_carry.follow_loses_a_curve,
        follow_retarget_ramp: route_carry.follow_retarget_ramp,
        restarted: restarted.into_boxed_slice(),
        fade_arms: fade_arm_rows.into_boxed_slice(),
        fade_watch,
        route_copies: route_copy_rows.into_boxed_slice(),
        input_floors,
        raised_arrivals,
        source_read_offset,
        published_generation: published_generation.clone(),
        lead_samples: warm.map_or(0, |lead| lead.lead_samples),
        prime_bytes: warm_prime_bytes,
    };
    // Issue #2199: the rows the projection was admitted on, against what the plan holds. The
    // exact rows are the projection's own; a ceiling row the plan exceeds, or an exact one it
    // misses, is an internal fault, never a session's: the admission charged less than the plan.
    let inventory_retained_bytes = inventory.retained_bytes();
    if native_effect_control_resources != host_projection.effect_control_resources
        || source_resources.total_engine_owned_bytes != host_projection.source_total_bytes
        || source_resources.overhead_bytes != host_projection.source_overhead_bytes
        || carried_total_bytes != host_projection.carried_source_total_bytes
        || carried_overhead_bytes != host_projection.carried_source_overhead_bytes
        || control_retained_bytes != host_projection.control_retained_bytes
    {
        return Err(platform("host.graph.projection"));
    }
    if route_control_resources.total_bytes > host_projection.route_controls.total_bytes
        || route_control_resources.largest_allocation_bytes
            > host_projection.route_controls.largest_allocation_bytes
        || carry_program_retained_bytes
            .checked_add(fade_arm_bytes)
            .is_none_or(|bytes| bytes > host_projection.carry_bytes)
        || inventory_retained_bytes > host_projection.inventory_bytes
    {
        return Err(platform("host.control.projection"));
    }
    let report = HostPrepareReport {
        sample_rate_hz: compiled.sample_rate().0,
        quantum_frames: compiled.quantum().0,
        source_count,
        track_count,
        submix_count,
        vca_count,
        route_count,
        effect_count,
        latency_samples: output_latency.0,
        output_tail_every_peak,
        graph_session_plus_plan_bytes: graph_resources.session_plus_plan_bytes,
        graph_incremental_plan_bytes: graph_resources.incremental_plan_bytes,
        graph_metadata_bytes: graph_resources.graph_metadata_bytes,
        graph_delay_bytes: graph_resources.delay_bytes,
        effect_bank_scratch_bytes: graph_resources.effect_bank_scratch_bytes,
        effect_bank_runtime_buffer_bytes: graph_resources.effect_bank_runtime_buffer_bytes,
        effect_bank_metadata_bytes: graph_resources.effect_bank_metadata_bytes,
        builtin_bank_bytes: graph_resources.builtin_bank_bytes,
        builtin_bank_scratch_bytes: graph_resources.builtin_bank_scratch_bytes,
        source_pcm_payload_bytes: source_resources.pcm_payload_already_charged_bytes,
        source_overhead_bytes: source_resources.overhead_bytes,
        source_total_bytes: source_resources.total_engine_owned_bytes,
        carried_source_total_bytes: carried_total_bytes,
        carried_source_overhead_bytes: carried_overhead_bytes,
        carry_program_retained_bytes,
        inventory_retained_bytes,
        effect_scalar_state_bytes: effect_state_bytes,
        effect_scalar_scratch_bytes: effect_scratch_bytes,
        builtin_processor_payload_bytes: builtin_resources.engine_owned_processor_payload_bytes,
        builtin_meter_payload_bytes: builtin_resources.engine_owned_meter_payload_bytes,
        builtin_retained_payload_bytes: builtin_resources.engine_owned_retained_payload_bytes,
        session_model_bytes: session_resources.compiled_model_bytes,
        session_largest_allocation_bytes: session_resources.single_allocation_bytes,
        control_retained_bytes,
        effect_control_resources: native_effect_control_resources,
        route_control_resources,
        observation_retained_bytes,
        spectrum_capture_retained_bytes,
        source_id_bytes: u64::try_from(source_id_bytes).map_err(|_| platform("host.count"))?,
        largest_engine_allocation_bytes: largest_engine_allocation_bytes
            .max(effect_carry_scratch_bytes),
    };

    // The mono collapse's structural join (mono-collapse M2). This is the one place a host has
    // both halves of the channel-symmetry witness in hand: `session_structural_symmetry`
    // answers per **track id** from the compiled session, and the built plan's bank chains are
    // keyed by anonymous **lanes**. The plan's own rows carry the relation, so the join is a call
    // and not a re-derivation.
    //
    // It is the *arming*, not a tuning knob. `BankChain::collapse_source` defaults to declining,
    // so a plan nobody joins never collapses at all -- which is what makes forgetting this call a
    // missed optimisation rather than wrong audio, and it is why the default is that way round.
    // On a session whose tracks read two source channels, which is every stereo session there is,
    // this arms nothing.
    // Source semantics: tracks only.
    let mono_source: BTreeSet<Box<str>> = session_structural_symmetry(compiled)
        .into_iter()
        .filter(|(_, witness)| witness.eligible())
        .map(|(track, _)| track)
        .collect();
    plan.arm_mono_collapse(&|track: &str| mono_source.contains(track));

    // Issue #1277 D5: the carried strips' live differences, written to this plan's own cells,
    // stamped with the revision it commits as. This is preparation's last fallible step, so a
    // preparation that fails writes nothing, and nothing writes the retargets again: the host's
    // commit stamps `revision` once more on the handle that inherits the open stamp, and
    // publishes it. A preparation that writes no retarget does not stamp.
    if let (Some(base), false) = (
        successor,
        strip_carry.retargets.is_empty()
            && effect_carry.retargets.is_empty()
            && route_carry.retargets.is_empty()
            && carried_inputs.iter().all(|join| join.retarget.is_empty()),
    ) {
        // #1594 D6: a monitor rebuild's model equals its predecessor's base, so nothing retargets;
        // a retarget would need a revision the commit never publishes.
        if base.unchanged_revision {
            return Err(PrepareDiagnostics::fixed(
                PrepareRejection::Session,
                "host.successor.monitor",
            ));
        }
        let Some(Ok(stamp)) = plan.stamp_held_gate(base.revision) else {
            return Err(successor_revision_refusal());
        };
        // Issue #1280 D4: the carried effect lanes' live differences, through the successor's own
        // producers: a bypass with the session's crossfade ramp, the parameter records as they
        // are, an EQ's targets through its owner's transaction. Every record passed its
        // producer's preflight when the lane was resolved, so a refusal here is an internal
        // fault.
        for retarget in &effect_carry.retargets {
            let producer = &mut effect_controls[retarget.producer];
            let fault = || graph_failure("host.control.order");
            for &record in &retarget.records {
                producer.write(stamp, record).map_err(|_| fault())?;
            }
            if let Some((base_revision, targets)) = &retarget.targets {
                producer
                    .publish_candidate_targets(stamp, *base_revision, targets)
                    .map_err(|_| fault())?;
                producer.commit_owner().map_err(|_| fault())?;
            }
        }
        // Issue #1284 D3: the carried sends' live differences, through the successor's own
        // producers, in the same block as the strips', with the ramp the record carries. A write
        // to a latest-target cell cannot fail (#1347), and every send of the table has a producer.
        // #1652 D3 (#1652c): a seven-word successor's record goes to its follow producer. The
        // join builds the record kind from the model the successor was prepared from, so each
        // record has its producer.
        for &(route_id, record) in &route_carry.retargets {
            match record {
                SendRecord::Folded(record) => {
                    let Some(position) = route_producer_position(&route_controls, route_id) else {
                        return Err(graph_failure("host.control.order"));
                    };
                    route_controls[position].write(stamp, record);
                }
                SendRecord::Following(record) => {
                    let Some(position) =
                        follow_route_producer_position(&follow_route_controls, route_id)
                    else {
                        return Err(graph_failure("host.control.order"));
                    };
                    follow_route_controls[position].write(stamp, record);
                }
            }
        }
        for retarget in &strip_carry.retargets {
            let Some(&index) = canonical_index.get(retarget.id) else {
                return Err(graph_failure("host.control.order"));
            };
            let producer = &mut strip_controls[index];
            for record in retarget.fader.iter().map_while(|record| *record) {
                producer.fader.write(stamp, record);
            }
            // #1648 D2 (U3 fold): an automated lane's new `S`, with the edit's length, through
            // the successor's own offsets cell, as the live path writes it. The record exists only
            // for a lane the successor automates, whose cell a plan with live cells attaches.
            for (channel, record) in retarget.offsets.iter().enumerate() {
                if let Some(record) = *record {
                    if !producer.fader.has_offsets(channel) {
                        return Err(graph_failure("host.control.order"));
                    }
                    producer.fader.write_offsets(stamp, channel, record);
                }
            }
            if let Some(record) = retarget.matrix {
                producer.producer.write(stamp, record);
            }
        }
        // #1630 D2: the carried input sections' live differences, through the successor's own
        // input cells, in the same block as the strips'. A carried lane with a retarget exists
        // only on a plan that attaches input cells, and every record is the live path's (#1556
        // D5): a trim in domain and a validated filter target.
        for join in carried_inputs
            .iter()
            .filter(|join| !join.retarget.is_empty())
        {
            let Some(input) = canonical_index
                .get(join.id)
                .and_then(|&index| strip_controls[index].input.as_mut())
            else {
                return Err(graph_failure("host.control.order"));
            };
            for &record in &join.retarget {
                input.write(stamp, record);
            }
        }
    }

    Ok(PrepareOutcome::Prepared(Box::new((
        PreparedHost {
            plan,
            sources,
            report,
            inventory,
            #[cfg(feature = "control-provider")]
            control_catalog,
        },
        HostLiveControlHandles {
            strips: live_control_strips,
            track_count: live_control_track_count,
            strip_controls,
            effect_controls,
            meters,
            effect_observations,
            route_controls,
            follow_route_controls,
            live_values_superseded: live_superseded,
            jump_lengths: Some(jump_lengths),
        },
        spectrum_capture,
    ))))
}

/// The meters a preparation builds, as `(strip ID, tap, metric set)` in reader order (#1327): the
/// caller's selection or the core's ([`crate::meters::meter_selection`]), and none for the plain
/// entry.
fn requested_meter_owners(
    selected_meters: Option<&[HostMeterRequest]>,
) -> Vec<(&str, MeterTap, MeterMetricSet)> {
    selected_meters
        .unwrap_or_default()
        .iter()
        .map(|meter| (meter.strip_id.as_ref(), meter.tap, meter.metrics))
        .collect()
}

/// The configuration every meter of a preparation shares (issue #1581 D2): the request's window,
/// or the core default when it is `0`, in frames of the plan's quantum.
fn requested_meter_config(
    live_controls: &HostLiveControlRequest,
    compiled: &CompiledSession,
) -> Result<MeterConfig, PrepareDiagnostics> {
    let quantum = compiled.quantum().0;
    let blocks = NonZeroU32::new(live_controls.meter_window_blocks).unwrap_or_else(|| {
        crate::meters::default_meter_window_blocks(compiled.sample_rate().0, quantum)
    });
    let period_frames = blocks
        .checked_mul(NonZeroU32::new(quantum).ok_or_else(|| shape("host.meter.period"))?)
        .ok_or_else(|| shape("host.meter.period"))?;
    Ok(MeterConfig {
        period_frames,
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: live_controls.meter_queue_depth,
        reset_generation: 0,
    })
}

/// Whether any of `owners` has a meter in `predecessor` with its key and `config` (#1327 D1) on a
/// strip `same_strip` says is of one kind in both plans' models (#1891), as
/// [`resolve_meter_carry`] pairs them.
fn meters_may_carry(
    predecessor: &[MeterInventoryRow],
    owners: &[(&str, MeterTap, MeterMetricSet)],
    config: MeterConfigKey,
    same_strip: impl Fn(&str) -> bool,
) -> bool {
    let keys: BTreeSet<(&str, MeterTap, MeterMetricSet)> = predecessor
        .iter()
        .filter(|row| row.config == config)
        .map(|row| (row.id.as_ref(), row.tap, row.metrics))
        .collect();
    owners
        .iter()
        .any(|owner| same_strip(owner.0) && keys.contains(owner))
}

/// What a successor's preparation takes from the meters of its predecessor (#1327).
#[derive(Default)]
struct MeterCarry {
    moves: Vec<graph::GraphObserverMove>,
    joins: Vec<graph::GraphObserverJoin>,
}

/// Join the successor's meter rows `own` with the predecessor's (#1327 D1, D6).
///
/// A meter carries when its strip ID, tap, metric set and configuration equal a predecessor
/// meter's and `same_strip` says its ID names a strip of one kind in both plans' models (#1891: a
/// track ID reused by a submix is another strip); two meters of one key pair in reader order.
/// Every other meter is fresh, and a fresh meter whose configuration equals a carried one's joins
/// the window grid of the first such carried meter, the one with the lowest successor entry (D6):
/// a grid, not state, so a strip of another kind joins it too.
fn resolve_meter_carry(
    predecessor: &[MeterInventoryRow],
    own: &[MeterInventoryRow],
    same_strip: impl Fn(&str) -> bool,
) -> MeterCarry {
    let mut available: BTreeMap<(&str, MeterTap, MeterMetricSet), Vec<&MeterInventoryRow>> =
        BTreeMap::new();
    for row in predecessor {
        available
            .entry((row.id.as_ref(), row.tap, row.metrics))
            .or_default()
            .push(row);
    }
    // Each key's candidates are taken in reader order: reverse once so `pop` yields the lowest.
    for rows in available.values_mut() {
        rows.sort_unstable_by_key(|row| core::cmp::Reverse(row.entry));
    }
    let mut carry = MeterCarry::default();
    let mut donors: BTreeMap<MeterConfigKey, graph::GraphObserverLocation> = BTreeMap::new();
    let mut fresh: Vec<&MeterInventoryRow> = Vec::new();
    let mut ordered: Vec<&MeterInventoryRow> = own.iter().collect();
    ordered.sort_unstable_by_key(|row| row.entry);
    for row in ordered {
        let taken = if same_strip(row.id.as_ref()) {
            available
                .get_mut(&(row.id.as_ref(), row.tap, row.metrics))
                .and_then(Vec::pop)
        } else {
            None
        };
        match taken {
            Some(before) if before.config == row.config => {
                carry.moves.push(graph::GraphObserverMove {
                    kind: engine::realtime::ObserverKind::Meter,
                    successor: row.location,
                    predecessor: before.location,
                    successor_entry: row.entry,
                    predecessor_entry: before.entry,
                });
                donors.entry(row.config).or_insert(row.location);
            }
            _ => fresh.push(row),
        }
    }
    for row in fresh {
        if let Some(&donor) = donors.get(&row.config) {
            carry.joins.push(graph::GraphObserverJoin {
                fresh: row.location,
                donor,
            });
        }
    }
    carry
}

/// What a successor's preparation takes from the spectrum captures of its predecessor (#1395).
#[derive(Default)]
struct SpectrumCarry {
    /// The observer moves the successor's swap block runs.
    moves: Vec<graph::GraphObserverMove>,
    /// The same pairs as the control side applies them at promotion (D3): the swap block's record
    /// and this list name one set of entries.
    pairing: Vec<crate::spectrum::SpectrumPair>,
}

/// The strip `id` names in `model`, a normalized model (tracks and submixes each in canonical ID
/// order): a track, a submix or none (#1593 T2b, #1891).
fn strip_kind_of<'m>(model: &'m SessionModel, id: &str) -> Option<StripKind<'m>> {
    if let Ok(index) = model
        .tracks
        .binary_search_by(|track| track.id.as_str().cmp(id))
    {
        return Some(StripKind::Track(&model.tracks[index]));
    }
    model
        .submixes
        .binary_search_by(|submix| submix.id.as_str().cmp(id))
        .ok()
        .map(|index| StripKind::Submix(&model.submixes[index]))
}

/// Whether `id` names a strip of one kind in `before` and `after` (#1593 T2b, #1891): a track in
/// both, a submix in both, or neither in both. An ID that a transaction moved from a track to a
/// submix (or back) names another strip, so no state keyed by the ID alone may carry onto it:
/// not a spectrum capture, a meter or a route's compensation line.
fn same_strip_id_kind(before: &SessionModel, after: &SessionModel, id: &str) -> bool {
    let kind = |model| strip_kind_of(model, id).map(|kind| core::mem::discriminant(&kind));
    kind(before) == kind(after)
}

/// Whether `target`'s ID names a strip of one kind in `before` and `after` (#1593 T2b): a track
/// tap's ID is a track in both or a submix in both ([`same_strip_id_kind`]). An output target is
/// always the same kind.
fn same_strip_kind(
    target: &crate::spectrum::SpectrumTarget,
    before: &SessionModel,
    after: &SessionModel,
) -> bool {
    use crate::spectrum::SpectrumTarget;
    match target {
        SpectrumTarget::TrackPostInputBuiltins(id) | SpectrumTarget::TrackPostMatrix(id) => {
            same_strip_id_kind(before, after, id)
        }
        SpectrumTarget::Output(_) => true,
    }
}

/// Join the successor's spectrum rows `own` with the predecessor's (#1395 D1).
///
/// A capture carries when its target and channels equal a predecessor capture's, both plans run at
/// one sample rate and quantum, both hold a single capture or both a collection (the control
/// side holds one kind and pairs within it), and `same_strip` says its target's ID names a strip
/// of one kind in both plans' models (#1593 T2b: a track ID reused by a submix is another strip);
/// nothing else matters, and nothing in it is a live value, so
/// there is no retarget. Matching is by key, never by entry or handle. Every other capture is
/// fresh.
fn resolve_spectrum_carry(
    predecessor: &PlanStateInventory,
    own: &[SpectrumInventoryRow],
    own_is_collection: bool,
    sample_rate_hz: u32,
    quantum_frames: u32,
    same_strip: impl Fn(&crate::spectrum::SpectrumTarget) -> bool,
) -> SpectrumCarry {
    let mut carry = SpectrumCarry::default();
    if predecessor.sample_rate_hz != sample_rate_hz
        || predecessor.quantum_frames != quantum_frames
        || predecessor.spectrum_collection != own_is_collection
    {
        return carry;
    }
    let mut taken = vec![false; predecessor.spectrum.len()];
    for row in own.iter().filter(|row| same_strip(&row.target)) {
        let found = predecessor
            .spectrum
            .iter()
            .enumerate()
            .find(|(index, before)| {
                !taken[*index] && before.target == row.target && before.channels == row.channels
            });
        let Some((index, before)) = found else {
            continue;
        };
        taken[index] = true;
        carry.moves.push(graph::GraphObserverMove {
            kind: engine::realtime::ObserverKind::Spectrum,
            successor: row.location,
            predecessor: before.location,
            successor_entry: row.entry,
            predecessor_entry: before.entry,
        });
        carry.pairing.push(crate::spectrum::SpectrumPair {
            successor: row.entry,
            predecessor: before.entry,
        });
    }
    carry
}

/// Join the successor's effect observation rows `own` with the predecessor's (#1327 D1): a tap
/// set carries when its effect instance carried (`moves` names the successor instance's site) and
/// both instances declare as many taps, with a row on each side. Every other instance's taps start
/// fresh.
fn resolve_observation_carry(
    predecessor: &[ObservationInventoryRow],
    own: &[ObservationInventoryRow],
    moves: &[graph::GraphEffectMove],
) -> Vec<graph::GraphTapMove> {
    let from: BTreeMap<graph::GraphEffectSite, graph::GraphEffectSite> = moves
        .iter()
        .map(|step| (step.successor, step.predecessor))
        .collect();
    let before: BTreeMap<graph::GraphEffectSite, &ObservationInventoryRow> =
        predecessor.iter().map(|row| (row.location, row)).collect();
    own.iter()
        .filter_map(|row| {
            let site = *from.get(&row.location)?;
            let earlier = before.get(&site)?;
            (earlier.taps == row.taps).then_some(graph::GraphTapMove {
                successor: row.location,
                predecessor: site,
                successor_entry: row.entry,
                predecessor_entry: earlier.entry,
            })
        })
        .collect()
}

/// The compensation-line moves of a successor (#1283 D1): a line carries when the predecessor
/// has a line on an edge of the same ID whose endpoints are the same. Both slices are sorted by
/// edge ID. A line has no live value, so there is no retarget; a re-pointed route has other
/// endpoints, so its line starts at rest and nothing about its strip changes. Lines of other
/// lengths carry too: the swap block aligns their heads (D4).
///
/// A line holds its source's audio, delayed. The graph names a submix's stage and effects as a
/// track's (`TrackStage`, `Effect`), so a line whose source strip changed kind under one ID keeps
/// its endpoints but holds the removed strip's audio: it carries only if `same_strip` passes its
/// source's strip ID (#1891). The destination's kind does not matter: the content is the
/// source's, so a key or route line from an unchanged source carries into a strip of another kind.
fn resolve_delay_carry(
    predecessor: &[graph::GraphDelayLine],
    own: &[graph::GraphDelayLine],
    same_strip: impl Fn(&str) -> bool,
) -> Vec<graph::GraphDelayMove> {
    let same_source = |node: &graph::GraphNodeId| match node {
        graph::GraphNodeId::TrackStage { track_id, .. } => same_strip(track_id.as_str()),
        // A strip's main edge into an effect with a late key starts at the strip's own node.
        graph::GraphNodeId::Effect(effect) => same_strip(effect.track_id.as_str()),
        _ => true,
    };
    own.iter()
        // Not the destination: a route's `TrackStage` destination is a submix's input on both
        // sides (a route never targets a track), and a line's content is its source's.
        .filter(|line| same_source(&line.source))
        .filter_map(|line| {
            let index = predecessor
                .binary_search_by(|earlier| earlier.edge.cmp(&line.edge))
                .ok()?;
            let earlier = &predecessor[index];
            (earlier.source == line.source && earlier.destination == line.destination).then_some(
                graph::GraphDelayMove {
                    successor: line.line,
                    predecessor: earlier.line,
                },
            )
        })
        .collect()
}

// REALTIME_POLICY_BEGIN: #1327 D3, the pairing runs in the plan-swap block.
/// Swap each carried effect observation reader set between the plans' reader vectors, at the swap
/// block (#1327 D3): for each record of kind [`engine::realtime::ObserverKind::Observation`], the
/// successor handle at `successor` and the predecessor handle at `predecessor` exchange their
/// readers, so a reader the host already holds follows the tap state that renders on, and the
/// retiring predecessor holds the successor's fresh one. Each handle keeps its own track, address
/// and effect ID: they are equal for a carried pair.
///
/// Allocation-free, drops nothing, and may run on the render thread. A record whose index is out of
/// range of its vector is skipped.
pub fn pair_carried_observations(
    successor: &mut [EffectObservationHandle],
    predecessor: &mut [EffectObservationHandle],
    carries: &[engine::realtime::ObserverCarry],
) {
    for carry in carries {
        if carry.kind != engine::realtime::ObserverKind::Observation {
            continue;
        }
        if let (Some(into), Some(from)) = (
            successor.get_mut(carry.successor),
            predecessor.get_mut(carry.predecessor),
        ) {
            core::mem::swap(&mut into.readers, &mut from.readers);
        }
    }
}
// REALTIME_POLICY_END

// REALTIME_POLICY_BEGIN: #1327 D3, the pairing runs in the plan-swap block.
/// Swap each carried meter's reader between the plans' reader vectors, at the swap block
/// (#1327 D3): for each record of kind [`engine::realtime::ObserverKind::Meter`], the successor
/// reader at `successor` and the predecessor reader at `predecessor` exchange their consumers, so
/// a reader the host already holds follows the observer that renders on, and the retiring
/// predecessor holds the successor's fresh one. Each reader keeps its own handle, track and tap:
/// the carried observer takes the successor's handle at the swap, so its snapshots name the
/// reader that now receives them.
///
/// Allocation-free, drops nothing, and may run on the render thread. A record whose index is out
/// of range of its vector is skipped.
pub fn pair_carried_meters(
    successor: &mut [MeterConsumer],
    predecessor: &mut [MeterConsumer],
    carries: &[engine::realtime::ObserverCarry],
) {
    for carry in carries {
        if carry.kind != engine::realtime::ObserverKind::Meter {
            continue;
        }
        if let (Some(into), Some(from)) = (
            successor.get_mut(carry.successor),
            predecessor.get_mut(carry.predecessor),
        ) {
            core::mem::swap(&mut into.consumer, &mut from.consumer);
        }
    }
}
// REALTIME_POLICY_END

/// One strip input section that carries (#1276 D1, #1630 D1): the predecessor lane it copies
/// from and the records that retarget it to this model's values, empty when they are equal.
struct InputJoin<'a> {
    id: &'a str,
    predecessor: graph::GraphLaneLocation,
    retarget: Vec<TrackInputRecord>,
}

/// Strip `id`'s row of `rows`, which are sorted by strip ID.
fn input_section_row<'a>(
    rows: &'a [InputSectionInventoryRow],
    id: &str,
) -> Option<&'a InputSectionInventoryRow> {
    rows.binary_search_by(|row| (*row.id).cmp(id))
        .ok()
        .map(|index| &rows[index])
}

/// The input section `strip` had in the predecessor's committed model, looked up among strips of
/// its own kind (issue #1276 D1): a track among the tracks, a submix among the submixes, so a
/// strip whose kind changed is not found and does not carry.
///
/// `committed` is a normalized model, whose tracks and submixes are each in canonical ID order;
/// a strip a search does not find starts at rest, never takes another strip's state.
fn committed_input_section<'a>(
    committed: &'a SessionModel,
    strip: &session::StripRef<'_>,
) -> Option<&'a session::DualMonoBuiltins> {
    let id = strip.id.as_str();
    match strip.kind {
        session::StripKind::Track(_) => committed
            .tracks
            .binary_search_by(|track| track.id.as_str().cmp(id))
            .ok()
            .map(|index| &committed.tracks[index].builtins),
        session::StripKind::Submix(_) => committed
            .submixes
            .binary_search_by(|submix| submix.id.as_str().cmp(id))
            .ok()
            .map(|index| &committed.submixes[index].builtins),
    }
}

/// Whether two strips' input sections are bit-equal (issue #1276 D1): polarity, trim and both
/// filter cutoffs, per channel. Only a plan without input cells, which cannot retarget a carried
/// section, needs it (#1630 D1).
fn same_input_section(left: &session::DualMonoBuiltins, right: &session::DualMonoBuiltins) -> bool {
    [(&left.left, &right.left), (&left.right, &right.right)]
        .into_iter()
        .all(|(left, right)| {
            left.polarity_invert == right.polarity_invert
                && left.trim_db.to_bits() == right.trim_db.to_bits()
                && left.hpf_hz.to_bits() == right.hpf_hz.to_bits()
                && left.lpf_hz.to_bits() == right.lpf_hz.to_bits()
        })
}

/// `owner`'s row of strip `id` in `rows`, which are sorted by strip ID and then owner.
fn strip_stage_row<'a>(
    rows: &'a [StripStageInventoryRow],
    id: &str,
    owner: graph::BuiltinStripOwner,
) -> Option<&'a StripStageInventoryRow> {
    rows.binary_search_by(|row| (*row.id).cmp(id).then(row.owner.cmp(&owner)))
        .ok()
        .map(|index| &rows[index])
}

/// The per-edit ramp lengths of one strip's five record rows, looked up once from the
/// transaction's [`EditRamps`] (#1394 D6): the `(Strip, Fader)`, `(Strip, Mute)`,
/// `(Strip, Matrix)`, `(Strip, Input)` and `(Strip, Link)` entries. `None` is "the edit gave no
/// length": the session's applies.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct StripEditRamps {
    fader: Option<u32>,
    mute: Option<u32>,
    matrix: Option<u32>,
    /// The input trim and polarity (#1556 D3).
    pub(crate) input: Option<u32>,
    /// Every link-mode row of the strip's effect instances (#1371 D3): one key per strip, so two
    /// link writes of one transaction to one strip resolve one length (`ControlError::RampConflict`
    /// refuses two).
    pub(crate) link: Option<u32>,
}

impl StripEditRamps {
    /// The lengths the transaction gave strip `id`'s rows. The common transaction carries none:
    /// then no lookup key is built.
    pub(crate) fn of(edit_ramps: &EditRamps, id: &session::StableId) -> Self {
        if edit_ramps.is_empty() {
            return Self::default();
        }
        let target = EditRampTarget::Strip(id.clone());
        Self {
            fader: edit_ramps.get(&target, EditRampRow::Fader),
            mute: edit_ramps.get(&target, EditRampRow::Mute),
            matrix: edit_ramps.get(&target, EditRampRow::Matrix),
            input: edit_ramps.get(&target, EditRampRow::Input),
            link: edit_ramps.get(&target, EditRampRow::Link),
        }
    }
}

/// The per-edit ramp lengths of one route's three rows, looked up once from the transaction's
/// [`EditRamps`] (#1557 D3): the `(Route, RouteGain)`, `(Route, RouteMute)` and
/// `(Route, RouteMatrix)` entries. `None` is "the edit gave no length": the session's applies.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct RouteEditRamps {
    pub(crate) gain: Option<u32>,
    pub(crate) mute: Option<u32>,
    pub(crate) matrix: Option<u32>,
}

impl RouteEditRamps {
    pub(crate) fn of(edit_ramps: &EditRamps, id: &session::StableId) -> Self {
        if edit_ramps.is_empty() {
            return Self::default();
        }
        let target = EditRampTarget::Route(id.clone());
        Self {
            gain: edit_ramps.get(&target, EditRampRow::RouteGain),
            mute: edit_ramps.get(&target, EditRampRow::RouteMute),
            matrix: edit_ramps.get(&target, EditRampRow::RouteMatrix),
        }
    }
}

/// The record that brings a send from `before`'s live values to `after`'s (#1557 D2, D3), when the
/// target the render plane holds changes: the per-route function of a send (#1284 D3). The
/// classifier derives a live delta's route records with it, and a successor's preparation derives
/// its carried sends' retargets with it, so the two cannot disagree.
///
/// The target is `graph_compiler::route_coefficients` of the route's gain, matrix and mute with
/// its effective gate ([`route_record`]), exactly what preparation folds into the route's
/// constants, so a settled send is bit-identical to a prepared one. It is compared as bits, with
/// the gate: a gain or matrix edit on a muted send leaves the silenced `+0.0` target as it was and
/// gives no record, and the later unmute carries the new values.
///
/// `gates` is the route's effective gate per source lane, in `[before, after]`
/// (`controls::effective`). The ramp is `gate_length` when the gate changes in either lane
/// (`EffectiveDiff::gate_length`, #1558 D7), else the longest of the changed rows' lengths
/// (`SendGain`, `SendMatrix`), each through [`LiveRamps::resolve`] (an explicit `0` is a step).
///
/// # Errors
///
/// [`LiveRebuild::Domain`] when either value set is outside the route domain, or a ramp is longer
/// than the cell's maximum.
pub(crate) fn route_record_between(
    ramps: LiveRamps,
    before: &session::Route,
    after: &session::Route,
    gates: [[bool; 2]; 2],
    gate_length: u32,
    edit: RouteEditRamps,
) -> Result<Option<RouteControlRecord>, LiveRebuild> {
    let matrix = |route: &session::Route| {
        let matrix = &route.channel_matrix;
        [matrix.ll, matrix.lr, matrix.rl, matrix.rr]
    };
    let build = |route: &session::Route, gate: [bool; 2], length: u32| {
        route_record(route.gain_db, matrix(route), route.mute, gate, length).map_err(|error| {
            match error {
                RouteControlError::Domain | RouteControlError::Length => LiveRebuild::Domain,
            }
        })
    };
    let prior = build(before, gates[0], 0)?;
    let current = build(after, gates[1], 0)?;
    let bits = |record: &RouteControlRecord| record.target().map(f32::to_bits);
    let gate_changed = gates[0] != gates[1];
    if prior.mute() == current.mute() && bits(&prior) == bits(&current) {
        return Ok(None);
    }
    let length = if gate_changed {
        gate_length
    } else {
        let gain = (before.gain_db.to_bits() != after.gain_db.to_bits())
            .then(|| ramps.resolve(LiveRampRow::SendGain, edit.gain));
        let matrix = (matrix(before).map(f32::to_bits) != matrix(after).map(f32::to_bits))
            .then(|| ramps.resolve(LiveRampRow::SendMatrix, edit.matrix));
        gain.into_iter()
            .chain(matrix)
            .max()
            .unwrap_or_else(|| ramps.resolve(LiveRampRow::SendMute, edit.mute))
    };
    build(after, gates[1], length).map(Some)
}

/// The record a send's cell takes (#1652 D1): the six folded words of #1347, or, for a route a
/// plan gives the seven-word cell (a route into a submix whose source strip automates its mute),
/// its open transform and flags. A host writes a `Folded` record through the route's
/// [`RouteControlProducer`] and a `Following` one through its
/// [`GraphFollowRouteControlProducer`](graph::GraphFollowRouteControlProducer) (#1653).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SendRecord {
    /// A six-word route's record (`route_record_between`).
    Folded(RouteControlRecord),
    /// A following route's record (`follow_route_record_between`).
    Following(graph::FollowRouteControlRecord),
}

impl SendRecord {
    /// The six-word record, or `None` for a following route's.
    #[must_use]
    pub const fn folded(&self) -> Option<&RouteControlRecord> {
        match self {
            Self::Folded(record) => Some(record),
            Self::Following(_) => None,
        }
    }

    /// The seven-word record, or `None` for a six-word route's.
    #[must_use]
    pub const fn following(&self) -> Option<&graph::FollowRouteControlRecord> {
        match self {
            Self::Following(record) => Some(record),
            Self::Folded(_) => None,
        }
    }
}

/// The VCA mute curves of strip `source`'s lanes that a following route's op ORs into its follow
/// term at `a(tap)` (#1655 D4, R19-2): per source lane `[left, right]`, the program cell index of
/// each reaching VCA's automated `mute` lane (`PreparedBuiltinsSession::vca_cells`, row 1, the
/// lane's own index), in ascending VCA ID (the order of [`SessionModel::vca_reach_of`]).
/// `vca_cells` is the plan's `PreparedBuiltinsSession::vca_cells`. Empty lanes when no entry drives
/// a VCA `mute` row. `program` is the plan's one automation program when the caller holds it (else
/// it takes the VCA cells' own). `None` when a reaching VCA's entry has no cell in that program: a
/// plan that disagrees with its model. `automation` is the index of `model`'s automation table
/// (#1844). Control side; it allocates the two lists.
pub(crate) fn follow_vca_mute_cells<P>(
    vca_cells: impl Fn(&str) -> Option<(std::sync::Arc<P>, builtins_compiler::VcaCells)>,
    model: &SessionModel,
    automation: &controls::AutomationIndex<'_>,
    source: &session::StableId,
    program: &mut Option<std::sync::Arc<P>>,
) -> Option<[Vec<u32>; 2]> {
    let mut lanes = [Vec::new(), Vec::new()];
    if !automation.any_vca_mute() {
        return Some(lanes);
    }
    for index in model.vca_reach_of(source) {
        let vca = &model.vcas.get(index)?.id;
        let driven = automation.vca_mute(vca);
        if driven.iter().all(Option::is_none) {
            continue;
        }
        let (cells_program, cells) = vca_cells(vca.as_str())?;
        match program {
            Some(program) if !std::sync::Arc::ptr_eq(&cells_program, program) => return None,
            Some(_) => {}
            None => *program = Some(cells_program),
        }
        for (lane, list) in lanes.iter_mut().enumerate() {
            match (driven[lane], cells[1][lane]) {
                (Some(_), Some(cell)) => list.push(cell),
                (None, None) => {}
                _ => return None,
            }
        }
    }
    Some(lanes)
}

/// The flags word of a following route (#1652 D1): the route's `mute` and `follows_mute`, the
/// source lanes' `terms` and the plan's `automated` lanes (bits 4-5).
pub(crate) const fn follow_route_flags(
    route: &session::Route,
    terms: [bool; 2],
    automated: [bool; 2],
) -> graph::RouteFollowFlags {
    graph::RouteFollowFlags {
        mute: route.mute,
        follows_mute: route.follows_mute,
        terms,
        automated,
    }
}

/// The seven-word record of a following route (#1652 D1) as `route` holds it: the open transform
/// (`graph_compiler::route_values`, the derivation the compiler lowers the plan's transform
/// with, so a record's words and the plan's are one bit pattern), the flags word
/// ([`follow_route_flags`]) and `length`.
///
/// # Errors
///
/// [`LiveRebuild::Domain`] when the gain or matrix is outside the route domain, or `length` is
/// longer than the cell's maximum.
pub(crate) fn follow_route_record(
    route: &session::Route,
    terms: [bool; 2],
    automated: [bool; 2],
    length: u32,
) -> Result<graph::FollowRouteControlRecord, LiveRebuild> {
    let matrix = &route.channel_matrix;
    let transform =
        graph_compiler::route_values(route.gain_db, [matrix.ll, matrix.lr, matrix.rl, matrix.rr])
            .map_err(|_| LiveRebuild::Domain)?;
    let flags = follow_route_flags(route, terms, automated);
    graph::FollowRouteControlRecord::new(transform, flags, length).ok_or(LiveRebuild::Domain)
}

/// The seven-word record that brings a following route from `before`'s committed values to
/// `after`'s (#1652 D1, D3), the counterpart of [`route_record_between`] for a route the next plan
/// gives the seven-word cell. The classifier (#1653) and a successor's send join call it, so the
/// two cannot disagree.
///
/// `terms` is the source lane's effective mute without its curve per model, in `[before,
/// after]` (`controls::effective`, OQ1 = C); `automated` is the next plan's. A record is built only
/// when a word the models hold changes: the transform's bits, `mute`, `follows_mute` or `terms`.
/// `automated` is the plan's, not the model's, so a route that only gains or loses its curve
/// gives no record (render compares the carried ramp with its own target, #1652 D3).
///
/// The ramp is the send rule of [`route_record_between`]: `gate_length` when the effective gate
/// changes in either lane, else the `SendFollowsMute` length when only `follows_mute` changes (the
/// curve can make that a change of target), else the longest of the changed `SendGain` and
/// `SendMatrix` lengths, else the `SendMute` length.
///
/// # Errors
///
/// [`LiveRebuild::Domain`] as [`follow_route_record`].
#[allow(clippy::too_many_arguments)]
pub(crate) fn follow_route_record_between(
    ramps: LiveRamps,
    before: &session::Route,
    after: &session::Route,
    terms: [[bool; 2]; 2],
    automated: [bool; 2],
    gates: [[bool; 2]; 2],
    gate_length: u32,
    edit: RouteEditRamps,
) -> Result<Option<graph::FollowRouteControlRecord>, LiveRebuild> {
    let prior = follow_route_record(before, terms[0], automated, 0)?;
    let current = follow_route_record(after, terms[1], automated, 0)?;
    let transform_bits = |record: &graph::FollowRouteControlRecord| {
        let graph::RouteTransform {
            gain,
            ll,
            lr,
            rl,
            rr,
        } = record.transform();
        [gain, ll, lr, rl, rr].map(f32::to_bits)
    };
    if transform_bits(&prior) == transform_bits(&current) && prior.flags() == current.flags() {
        return Ok(None);
    }
    let matrix = |route: &session::Route| {
        let matrix = &route.channel_matrix;
        [matrix.ll, matrix.lr, matrix.rl, matrix.rr].map(f32::to_bits)
    };
    let length = if gates[0] != gates[1] {
        gate_length
    } else if before.follows_mute != after.follows_mute {
        ramps.resolve(LiveRampRow::SendFollowsMute, edit.mute)
    } else {
        let gain = (before.gain_db.to_bits() != after.gain_db.to_bits())
            .then(|| ramps.resolve(LiveRampRow::SendGain, edit.gain));
        let matrix = (matrix(before) != matrix(after))
            .then(|| ramps.resolve(LiveRampRow::SendMatrix, edit.matrix));
        gain.into_iter()
            .chain(matrix)
            .max()
            .unwrap_or_else(|| ramps.resolve(LiveRampRow::SendMute, edit.mute))
    };
    follow_route_record(after, terms[1], automated, length).map(Some)
}

/// A fader or pan value outside the domain its render-side setter accepts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct OutOfDomain;

/// The ramp length of each record of a strip's fader/mute stage, per lane (decision 17, C6 step 4,
/// #1558 D7).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LevelLengths {
    /// The `FaderDb` record of each lane.
    pub(crate) fader: [u32; 2],
    /// The `Mute` record of each lane.
    pub(crate) mute: [u32; 2],
}

/// The fader and mute records that bring a strip's fader/mute stage from `before`'s values to
/// `after`'s, in push order: `FaderDb` records first, then `Mute` records, each left before
/// right; `None` after the last (#1053 D1).
///
/// `before` and `after` are the strip's *effective* values ([`effective_fader`]), never its own
/// (#1558 D1): the stage holds what render sees, so a VCA's move, a membership change and an own
/// edit are one difference. The per-owner function of #1277 D5: the classifier derives a live
/// delta's records with it, and a successor's preparation derives its carry retargets with it, so
/// the two cannot disagree. A record is emitted only when the value the render plane holds changes:
/// a `FaderDb` per lane whose gain bits change (one `Both` when both change to the same dB bits
/// and the same length) and a `Mute` per lane whose mute changes (one `Both` when both change to
/// the same value and the same length). A fader move on a lane that stays muted still gets its
/// record, so the stage remembers the gain for a later unmute.
///
/// A lane that `automated` names (the fader event lanes of the post-change model,
/// [`FaderEventLanes::events`]) gets no `FaderDb` (#1648 D3, #1655 D3): its cell renders the curve
/// (its own, or a reaching VCA's composed with the member's `own_db`, which the lane's offsets
/// record carries) and the static value is only its fallback. That is no error, so a successor's join reads the
/// strip as unchanged when nothing else moved, and its stage carries. Its `Mute` records are
/// unchanged. `automated` is called only when a lane's gain bits change.
///
/// # Errors
///
/// [`OutOfDomain`] when either fader is outside the domain [`checked_fader_gain`] accepts.
pub(crate) fn fader_records(
    before: &session::DualMonoFader,
    after: &session::DualMonoFader,
    lengths: LevelLengths,
    automated: impl FnOnce() -> [bool; 2],
) -> Result<[Option<TrackFaderRecord>; 4], OutOfDomain> {
    let gain = |db| {
        checked_fader_gain(db)
            .map(f32::to_bits)
            .map_err(|_| OutOfDomain)
    };
    let gains_before = [gain(before.left_db)?, gain(before.right_db)?];
    let gains_after = [gain(after.left_db)?, gain(after.right_db)?];
    let mutes_before = [before.left_mute, before.right_mute];
    let mutes_after = [after.left_mute, after.right_mute];
    let mut fader = [None; 4];
    let mut used = 0;
    let mut push = |record| {
        fader[used] = Some(record);
        used += 1;
    };
    let db_after = [after.left_db, after.right_db];
    let mut gain_changed = [0, 1].map(|lane| gains_before[lane] != gains_after[lane]);
    // #1648 D3: an automated lane renders its curve; its static value is the fallback, so a change
    // of it is no record. Asked only when a gain changed.
    if gain_changed.contains(&true) {
        let automated = automated();
        for lane in 0..2 {
            gain_changed[lane] &= !automated[lane];
        }
    }
    for (lanes, lane) in lane_records(
        gain_changed,
        db_after[0].to_bits() == db_after[1].to_bits() && lengths.fader[0] == lengths.fader[1],
    ) {
        push(TrackFaderRecord::FaderDb {
            lanes,
            db: db_after[lane],
            smoothing_samples: lengths.fader[lane],
        });
    }
    let mute_changed = [0, 1].map(|lane| mutes_before[lane] != mutes_after[lane]);
    for (lanes, lane) in lane_records(
        mute_changed,
        mutes_after[0] == mutes_after[1] && lengths.mute[0] == lengths.mute[1],
    ) {
        push(TrackFaderRecord::Mute {
            lanes,
            muted: mutes_after[lane],
            smoothing_samples: lengths.mute[lane],
        });
    }
    Ok(fader)
}

/// The level records of one strip: the [`fader_records`] of its fader/mute stage and the
/// offsets-cell writes of its automated fader lanes (#1649 D4), `[left, right]`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct LevelRecords {
    pub(crate) fader: [Option<TrackFaderRecord>; 4],
    pub(crate) offsets: [Option<FaderOffsetsRecord>; 2],
}

/// The fader lanes of one strip that a plan of the post-change model renders by events (#1655 D2,
/// D3): `events`, the fader event lanes ([`controls::fader_event_lanes`]: an own fader entry, or
/// an automated VCA `offset_db` lane that reaches the lane), and `own`, the lanes of `events` whose
/// own fader is automated ([`controls::automated_fader_lanes`]). An event lane outside `own` hears
/// the member's static fader dB (`own_db`) through its offsets cell.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct FaderEventLanes {
    pub(crate) events: [bool; 2],
    pub(crate) own: [bool; 2],
}

impl FaderEventLanes {
    /// The event lanes of strip `strip` in the committed model of `model`, `reach` its VCA reach
    /// there (indices into its `vcas`, [`SessionModel::vca_reach`]). Lookups when `model` is
    /// indexed (#1844).
    pub(crate) fn of(
        model: controls::ModelView<'_>,
        strip: &session::StableId,
        reach: &[usize],
    ) -> Self {
        Self {
            events: model.fader_event_lanes(strip, reach),
            own: model.automated_fader_lanes(strip),
        }
    }

    /// Whether `lane` is an event lane whose own fader is not automated: its offsets cell holds
    /// `own_db` (#1655 D3).
    const fn static_own(self, lane: usize) -> bool {
        self.events[lane] && !self.own[lane]
    }
}

/// The level records that bring a strip from `before`'s effective values to `after`'s: its
/// [`fader_records`] with the fader event lanes (`lanes().events`) as the automated lanes, so no
/// `FaderDb` reaches a lane that renders by events (#1648 D3, #1655 D3), and one offsets-cell write
/// per event lane whose `S` ([`controls::EffectiveStrip::offsets_sum`]) differs in bits, or, on an
/// event lane whose own fader is not automated, whose static own fader dB differs in bits (`own`,
/// the strip's own faders before and after). The write carries `after`'s `S` and the lane's fader
/// length (#1649 D4, departure 1: the one length rule, so an automated and a plain member of one
/// VCA edit ramp alike; a hand fader move takes the `Fader` row's length, `LevelRules::lengths`),
/// and on such a lane `own_db = Some(after's own fader dB)` (#1655 D3), else `None`. A VCA ride, a
/// membership change, a nested VCA, an added and a removed VCA each change only `S` on an event
/// lane. `lanes` is asked at most once.
///
/// # Errors
///
/// [`OutOfDomain`] as [`fader_records`].
pub(crate) fn level_records_between(
    before: &controls::EffectiveStrip,
    after: &controls::EffectiveStrip,
    own: [&session::DualMonoFader; 2],
    lengths: LevelLengths,
    lanes: impl Fn() -> FaderEventLanes,
) -> Result<LevelRecords, OutOfDomain> {
    let asked = core::cell::OnceCell::new();
    let lanes = || *asked.get_or_init(&lanes);
    let fader = fader_records(
        &effective_fader(before),
        &effective_fader(after),
        lengths,
        || lanes().events,
    )?;
    let mut offsets = [None; 2];
    let own_db =
        |fader: &session::DualMonoFader, lane: usize| [fader.left_db, fader.right_db][lane];
    let sum_moved =
        [0, 1].map(|lane| before.offsets_sum[lane].to_bits() != after.offsets_sum[lane].to_bits());
    let own_moved =
        [0, 1].map(|lane| own_db(own[0], lane).to_bits() != own_db(own[1], lane).to_bits());
    if sum_moved.contains(&true) || own_moved.contains(&true) {
        let lanes = lanes();
        for lane in 0..2 {
            let static_own = lanes.static_own(lane);
            if lanes.events[lane] && (sum_moved[lane] || (static_own && own_moved[lane])) {
                offsets[lane] = Some(FaderOffsetsRecord {
                    offsets_sum: after.offsets_sum[lane],
                    smoothing_samples: lengths.fader[lane],
                    own_db: static_own.then(|| own_db(own[1], lane)),
                });
            }
        }
    }
    Ok(LevelRecords { fader, offsets })
}

/// Whether a strip's effective fader, mute or offset sum `S`, or its own static fader dB (`own`,
/// before and after), differs, as bits: the one test of "this strip's level moved", for the dense
/// diff and the local one. `S` moves alone when a VCA edit leaves the effective fader's bits (a
/// clamp, a zero's sign), and an automated lane hears `S` (#1649). The own fader moves alone at a
/// clamp, and a fader event lane whose own fader is not automated hears it (`own_db`, #1655 D3)
/// composed with the VCA curves, which can leave the clamp; on any other lane such a move gives
/// no record ([`level_records_between`]).
#[cfg(feature = "control-provider")]
pub(crate) fn level_differs(
    before: &controls::EffectiveStrip,
    after: &controls::EffectiveStrip,
    own: [&session::DualMonoFader; 2],
) -> bool {
    let db = |fader: &session::DualMonoFader| [fader.left_db, fader.right_db].map(f32::to_bits);
    before.fader_db.map(f32::to_bits) != after.fader_db.map(f32::to_bits)
        || before.mute != after.mute
        || before.offsets_sum.map(f64::to_bits) != after.offsets_sum.map(f64::to_bits)
        || db(own[0]) != db(own[1])
}

/// A strip's effective fader and mute as a [`session::DualMonoFader`], the shape its stage's
/// records are derived from.
pub(crate) fn effective_fader(effective: &controls::EffectiveStrip) -> session::DualMonoFader {
    session::DualMonoFader {
        left_db: effective.fader_db[0],
        right_db: effective.fader_db[1],
        left_mute: effective.mute[0],
        right_mute: effective.mute[1],
    }
}

/// The strip at `index` of [`SessionModel::strips`] order (the tracks, then the submixes), in
/// `O(1)`.
pub(crate) fn strip_at(model: &SessionModel, index: usize) -> StripRef<'_> {
    match model.tracks.get(index) {
        Some(track) => StripRef::track(track),
        None => StripRef::submix(&model.submixes[index - model.tracks.len()]),
    }
}

/// What the ramp rules of a level record read about one strip besides the models (#1558 D7): the
/// strip's own fader in each model, its ID in `next`, the VCAs that reach it in each model and its
/// solo term in each. A dense diff and a local one both build it, so the rules below are written
/// once.
#[derive(Clone, Copy)]
pub(crate) struct StripChange<'s> {
    pub(crate) own_before: &'s session::DualMonoFader,
    pub(crate) own_after: &'s session::DualMonoFader,
    /// The hand-move offset of each automated fader lane in each model
    /// ([`controls::EffectiveStrip::fader_offset`], #1650): a hand move on an automated fader
    /// writes it, so it is the strip's own fader edit there.
    pub(crate) offset_before: [Option<f32>; 2],
    pub(crate) offset_after: [Option<f32>; 2],
    pub(crate) id: &'s session::StableId,
    /// [`SessionModel::vca_reach`] of the strip in `current`, then in `next`.
    pub(crate) reach_before: &'s [usize],
    pub(crate) reach: &'s [usize],
    /// The solo term of the strip's mute ([`controls::solo_term`]) under each overlay.
    pub(crate) solo_before: bool,
    pub(crate) solo_after: bool,
}

/// The ramp rules of the level and gate records of two models (decision 17, C6 step 4; #1558 D7;
/// #1559 D7): which length a record takes. Written once; [`EffectiveDiff`] (every strip) and the
/// classifier's local diff (the strips an edit reaches) both ask it.
pub(crate) struct LevelRules<'a> {
    pub(crate) current: &'a SessionModel,
    /// The model after the change, read through a view so that a transaction of values need not
    /// build it (#1613).
    pub(crate) next: controls::ModelView<'a>,
    /// Where the automation entries of `[current, next]` are read from (#1844, #1864): the VCA
    /// offset entries of [`Self::vca_offset_moved`], and the event lanes of the diff that owns the
    /// rules.
    pub(crate) automation: [AutomationReads<'a>; 2],
    pub(crate) ramps: LiveRamps,
    pub(crate) edit_ramps: &'a EditRamps,
    /// The ramp length of a mute record that only the overlay change produced (#1559 D7).
    pub(crate) solo_ramp: Option<u32>,
}

impl LevelRules<'_> {
    /// The ramp length of each lane's `FaderDb` and `Mute` record of one strip (#1558 D7).
    ///
    /// A lane's fader length is that of the strip's own `fader_db` edit when the strip's own value
    /// on the lane changed, or its hand-move offset did on an automated lane (#1650 D5) (row
    /// `Fader`, the edit's length); else that of the first reaching VCA,
    /// in ascending VCA-ID order, whose offset on the lane changed (row `VcaOffset`, the VCA's
    /// edit's length); else row `VcaOffset` with no edit length (a membership change, or a VCA
    /// added or removed). The mute is the same with rows `Mute` and `VcaMute`.
    pub(crate) fn lengths(&self, strip: &StripChange<'_>) -> LevelLengths {
        let (own_before, own_after) = (strip.own_before, strip.own_after);
        let edit = StripEditRamps::of(self.edit_ramps, strip.id);
        let reaching = |lane: usize, mute: bool| {
            strip
                .reach
                .iter()
                .map(|&index| (index, self.next.vca(index)))
                .find(|&(index, vca)| self.vca_changed(index, vca, lane, mute))
                .map(|(_, vca)| vca)
                .and_then(|vca| {
                    let row = if mute {
                        EditRampRow::VcaMute
                    } else {
                        EditRampRow::VcaFader
                    };
                    (!self.edit_ramps.is_empty())
                        .then(|| {
                            self.edit_ramps
                                .get(&EditRampTarget::Vca(vca.id.clone()), row)
                        })
                        .flatten()
                })
        };
        let fader = [0, 1].map(|lane| {
            let db = |fader: &session::DualMonoFader| [fader.left_db, fader.right_db][lane];
            // A hand move on an automated lane (#1650 D5) is the strip's own fader edit.
            let offset = |offsets: [Option<f32>; 2]| offsets[lane].map(f32::to_bits);
            if db(own_before).to_bits() != db(own_after).to_bits()
                || offset(strip.offset_before) != offset(strip.offset_after)
            {
                self.ramps.resolve(LiveRampRow::Fader, edit.fader)
            } else {
                self.ramps
                    .resolve(LiveRampRow::VcaOffset, reaching(lane, false))
            }
        });
        // A model edit's length wins: the strip's own mute, then a reaching VCA's. A mute that
        // only the overlay moved takes the solo's (#1559 D7).
        let solo_moved = strip.solo_before != strip.solo_after;
        let mute = [0, 1].map(|lane| {
            let muted = |fader: &session::DualMonoFader| [fader.left_mute, fader.right_mute][lane];
            if muted(own_before) != muted(own_after) {
                self.ramps.resolve(LiveRampRow::Mute, edit.mute)
            } else if solo_moved && !self.vca_mute_moved(strip, lane) {
                self.ramps.resolve(LiveRampRow::Solo, self.solo_ramp)
            } else {
                self.ramps
                    .resolve(LiveRampRow::VcaMute, reaching(lane, true))
            }
        });
        LevelLengths { fader, mute }
    }

    /// Whether a VCA moved the strip's mute on `lane`: a reaching VCA's mute differs from the
    /// committed model's, or a change in reach (a member added or removed, a VCA added or removed)
    /// changes whether any reaching VCA holds the lane muted. Both are the model's cause (#1559
    /// D7: a change in VCA reach is the model's), so the mute record takes the model's length
    /// beside a solo with its own.
    fn vca_mute_moved(&self, strip: &StripChange<'_>, lane: usize) -> bool {
        let holds = |vca: &session::Vca| [vca.fader.left_mute, vca.fader.right_mute][lane];
        strip
            .reach
            .iter()
            .any(|&vca| self.vca_changed(vca, self.next.vca(vca), lane, true))
            || strip
                .reach_before
                .iter()
                .any(|&vca| holds(&self.current.vcas[vca]))
                != strip.reach.iter().any(|&vca| holds(self.next.vca(vca)))
    }

    /// Whether `vca`'s offset (or its mute) on `lane` differs from the committed model's, or the
    /// committed model has no such VCA. `index` is the VCA's position in the model after the
    /// change; the committed model usually holds the same VCA at the same position, which is
    /// tried first, because a search by ID per reaching VCA is quadratic in the VCA count for a
    /// strip that every VCA reaches.
    fn vca_changed(&self, index: usize, vca: &session::Vca, lane: usize, mute: bool) -> bool {
        let Some(prior) = self
            .current
            .vcas
            .get(index)
            .filter(|prior| prior.id == vca.id)
            .or_else(|| self.current.vcas.iter().find(|prior| prior.id == vca.id))
        else {
            return true;
        };
        if mute {
            [prior.fader.left_mute, prior.fader.right_mute][lane]
                != [vca.fader.left_mute, vca.fader.right_mute][lane]
        } else {
            [prior.fader.left_db, prior.fader.right_db][lane].to_bits()
                != [vca.fader.left_db, vca.fader.right_db][lane].to_bits()
                || self.vca_offset_moved(&vca.id, lane)
        }
    }

    /// Whether the hand offset of the entry that drives VCA `vca`'s `offset_db` on `lane` differs
    /// between the models (#1655 D5): a hand move of an automated VCA offset writes it, so it is
    /// the VCA's own offset edit there, and its members' records take the VCA's edit length.
    fn vca_offset_moved(&self, vca: &session::StableId, lane: usize) -> bool {
        if self.current.automation.is_empty() {
            return false;
        }
        let offset = |entries: [Option<&session::Automation>; 2]| {
            entries[lane].map(|entry| [entry.offset.left_db, entry.offset.right_db][lane].to_bits())
        };
        let [current, next] = &self.automation;
        offset(current.view().vca_automation(vca))
            != offset(next.view_of(self.next).vca_automation(vca))
    }

    /// The ramp length of a route record whose gate changed (#1558 D7): the route's own `mute` or
    /// `follows_mute` edit when one of them changed (rows `SendMute`, `SendFollowsMute`, with
    /// `own_edit` the route's `RouteMute` length), else the length that its source strip's `Mute`
    /// record took, the longest of the lanes whose gate changed, so a strip and its following
    /// sends ramp together. `source` is the route's source strip.
    pub(crate) fn gate_length(
        &self,
        before: &session::Route,
        after: &session::Route,
        own_edit: Option<u32>,
        source: &StripChange<'_>,
        gates: [[bool; 2]; 2],
    ) -> u32 {
        if before.mute != after.mute {
            return self.ramps.resolve(LiveRampRow::SendMute, own_edit);
        }
        if before.follows_mute != after.follows_mute {
            return self.ramps.resolve(LiveRampRow::SendFollowsMute, own_edit);
        }
        let lengths = self.lengths(source);
        (0..2)
            .filter(|&lane| gates[0][lane] != gates[1][lane])
            .map(|lane| lengths.mute[lane])
            .max()
            .unwrap_or_else(|| self.ramps.resolve(LiveRampRow::SendMute, own_edit))
    }
}

/// The effective values of two models and the ramp rules that go with them (decision 17, C6 step 3
/// and 4, C7; #1558 D1, D7): the one place a strip's level records, and the length of a route's
/// gate record, are decided. The classifier asks it for the live path and `join_strips` asks it
/// for a successor's carry, so a VCA move, a membership change, an own edit, a followed mute and a
/// following send cannot be composed two ways.
///
/// It owns both [`controls::Effective`] values, computed once, on the control thread. Strip
/// indices count [`SessionModel::strips`] of the model they name and route indices count
/// `model.routes`. The classifier's fast path uses the strips-it-reaches form of the same rules
/// (`live_delta`'s local diff), over the same [`LevelRules`].
pub(crate) struct EffectiveDiff<'a> {
    rules: LevelRules<'a>,
    before: controls::Effective,
    after: controls::Effective,
    /// [`SessionModel::vca_reach`] of `next`: per strip, the reaching VCAs' indices in ascending
    /// VCA-ID order.
    reach: Vec<Vec<usize>>,
    /// The same for `current`, per strip of `current`: the reach the strip had before the change.
    reach_before: Vec<Vec<usize>>,
    /// Per strip of each model, whether the solo term of its mute holds ([`controls::solo_term`]).
    solo_before: Vec<bool>,
    solo_after: Vec<bool>,
}

/// One model read for its automation entries, with one strip's VCA reach there (#1864).
type ReachRead<'r> = (controls::ModelView<'r>, &'r [usize]);

/// Where a diff reads the automation entries of one model's rendered rows from (#1844): the
/// [`controls::AutomationIndex`] its caller built once, or the model's table, scanned per read
/// until more than `threshold` reads, then indexed once. A preparation reads every strip and
/// following route, so it builds the index up front; the live classifier reads the few owners an
/// edit reaches, where the scan is cheaper than the index (#1710's one-owner measurement).
pub(crate) struct AutomationReads<'a> {
    model: &'a SessionModel,
    built: Option<&'a controls::AutomationIndex<'a>>,
    threshold: usize,
    reads: core::cell::Cell<usize>,
    lazy: core::cell::OnceCell<controls::AutomationIndex<'a>>,
}

impl<'a> AutomationReads<'a> {
    /// The reads of `model` through `index`, its index.
    pub(crate) fn indexed(
        model: &'a SessionModel,
        index: &'a controls::AutomationIndex<'a>,
    ) -> Self {
        Self {
            model,
            built: Some(index),
            threshold: 0,
            reads: core::cell::Cell::new(0),
            lazy: core::cell::OnceCell::new(),
        }
    }

    /// The reads of `model`, indexed once past `threshold` reads.
    #[cfg(feature = "control-provider")]
    pub(crate) fn lazy(model: &'a SessionModel, threshold: usize) -> Self {
        Self {
            model,
            built: None,
            threshold,
            reads: core::cell::Cell::new(0),
            lazy: core::cell::OnceCell::new(),
        }
    }

    /// The model, read whole, for one read: indexed when the caller built the index or once the
    /// reads pass the threshold.
    pub(crate) fn view(&self) -> controls::ModelView<'_> {
        self.view_of(controls::ModelView::whole(self.model))
    }

    /// `base`, a view whose committed model is this one (an edited view of it among them, #1864),
    /// for one read, indexed as [`Self::view`] is.
    pub(crate) fn view_of<'v>(&'v self, base: controls::ModelView<'v>) -> controls::ModelView<'v> {
        if let Some(index) = self.built {
            return base.indexed(index);
        }
        let reads = self.reads.get().saturating_add(1);
        self.reads.set(reads);
        if reads > self.threshold {
            base.indexed(
                self.lazy
                    .get_or_init(|| controls::AutomationIndex::of(self.model)),
            )
        } else {
            base
        }
    }
}

/// The solo term of every strip of `model` under `overlay`, in [`SessionModel::strips`] order.
fn solo_terms(model: &SessionModel, overlay: &controls::MonitorOverlay) -> Vec<bool> {
    use controls::EffectiveInputs as _;
    let inputs = controls::ModelInputs::new(model, overlay);
    let any_solo = controls::any_solo(&inputs);
    (0..inputs.strip_count())
        .map(|strip| controls::solo_term(&inputs, strip, any_solo))
        .collect()
}

impl<'a> EffectiveDiff<'a> {
    /// The effective values of `current` under `monitor.committed` and of `next` under
    /// `monitor.next` (decision 17, C7; #1559 D6). `automation` reads the event lanes of
    /// `[current, next]`.
    pub(crate) fn new(
        current: &'a SessionModel,
        next: &'a SessionModel,
        ramps: LiveRamps,
        edit_ramps: &'a EditRamps,
        monitor: MonitorBase<'_>,
        automation: [AutomationReads<'a>; 2],
    ) -> Self {
        Self {
            rules: LevelRules {
                current,
                next: controls::ModelView::whole(next),
                ramps,
                edit_ramps,
                solo_ramp: monitor.ramp,
                automation,
            },
            before: controls::effective(current, monitor.committed),
            after: controls::effective(next, monitor.next),
            reach: next.vca_reach(),
            reach_before: current.vca_reach(),
            solo_before: solo_terms(current, monitor.committed),
            solo_after: solo_terms(next, monitor.next),
        }
    }

    /// The continued fader and mute lanes of strip `id` ([`controls::continued_fader_lanes`],
    /// [`controls::continued_mute_lanes`]), at `before` in `current` (`None` when `current` has
    /// no strip `id`) and `after` in `next`: lookups through both models' reads and the reaches
    /// computed once, not two table scans and two reach walks per strip (#1864).
    pub(crate) fn continued_lanes(
        &self,
        before: Option<usize>,
        after: usize,
        id: &session::StableId,
    ) -> [[bool; 2]; 2] {
        let [current, next] = self.continued_reads(before, after);
        [
            controls::continued_fader_lanes_in(current, next, id),
            controls::continued_mute_lanes_in(current, next, id),
        ]
    }

    /// The mute half of [`Self::continued_lanes`], for a following route's source.
    pub(crate) fn continued_mute_lanes(
        &self,
        before: Option<usize>,
        after: usize,
        id: &session::StableId,
    ) -> [bool; 2] {
        let [current, next] = self.continued_reads(before, after);
        controls::continued_mute_lanes_in(current, next, id)
    }

    /// Both models' reads with the strip's reach in each, for [`Self::continued_lanes`].
    fn continued_reads(&self, before: Option<usize>, after: usize) -> [ReachRead<'_>; 2] {
        let [current, next] = &self.rules.automation;
        let reach_before = before.map_or(&[][..], |before| &self.reach_before[before]);
        [
            (current.view(), reach_before),
            (next.view(), &self.reach[after][..]),
        ]
    }

    /// The post-change model, read for its automation entries (#1844).
    pub(crate) fn automation(&self) -> controls::ModelView<'_> {
        self.rules.automation[1].view()
    }

    /// What the ramp rules read about the strip at `before` in `current` and `after` in `next`.
    fn change(&self, before: usize, after: usize) -> StripChange<'_> {
        let after_strip = self.rules.next.strip(after);
        StripChange {
            own_before: strip_at(self.rules.current, before).fader,
            own_after: after_strip.fader,
            offset_before: self.before.strips[before].fader_offset,
            offset_after: self.after.strips[after].fader_offset,
            id: after_strip.id,
            reach_before: &self.reach_before[before],
            reach: &self.reach[after],
            solo_before: self.solo_before[before],
            solo_after: self.solo_after[after],
        }
    }

    #[cfg(feature = "control-provider")]
    /// Whether the strip's effective fader or mute (or its own static fader, [`level_differs`])
    /// differs, as bits, between the two models.
    pub(crate) fn level_changed(&self, before: usize, after: usize) -> bool {
        let change = self.change(before, after);
        level_differs(
            &self.before.strips[before],
            &self.after.strips[after],
            [change.own_before, change.own_after],
        )
    }

    /// The level records of one strip, its effective values in the two models with the lengths
    /// the ramp rules give, and its offsets-cell writes (#1649 D4).
    ///
    /// # Errors
    ///
    /// [`OutOfDomain`] as [`fader_records`].
    pub(crate) fn strip_level_records(
        &self,
        before: usize,
        after: usize,
    ) -> Result<LevelRecords, OutOfDomain> {
        let change = self.change(before, after);
        level_records_between(
            &self.before.strips[before],
            &self.after.strips[after],
            [change.own_before, change.own_after],
            self.rules.lengths(&change),
            || FaderEventLanes::of(self.automation(), change.id, change.reach),
        )
    }

    /// The carry retargets of one strip's fader/mute stage at a successor's strip join (#1277 D5):
    /// its [`Self::strip_level_records`], the offsets-cell writes with them (#1648 D2, U3 fold:
    /// an `S` the transaction moves reaches a carried automated lane as the live path's record,
    /// with the edit's length), plus the release of each lane whose stored fader automation
    /// the change removes (#1648 D2, "a cell lost"). Such a lane's carried stage holds the curve's
    /// last target, which no model holds, so the static values of the two models cannot show the
    /// difference; a `FaderDb` of the lane's effective value in `next` moves it there over the
    /// lane's fader length, as a live write does (the stage's live-write rule then holds the
    /// batch 13b obligation: inside an unmute ramp the write retargets, while muted the stage
    /// remembers it for the unmute). A released lane whose static value changed already has its
    /// record. The live classifier never reaches a removal (#1648 D1 rebuilds it), so this is the
    /// join's alone.
    ///
    /// #1651 T4, a mute cell lost: each lane whose stored mute automation the change removes gets
    /// a `Mute` record of its effective mute in `next` (`controls::EffectiveStrip::mute`, the
    /// static mute with its VCA and solo terms) over the lane's mute length, for the same reason:
    /// its carried stage holds `curve || terms`. The stage's batch 13b rule makes the record a
    /// no-op when the stage already agrees. A released lane whose effective mute changed already
    /// has its record.
    ///
    /// # Errors
    ///
    /// [`OutOfDomain`] as [`fader_records`].
    pub(crate) fn join_level_records(
        &self,
        before: usize,
        after: usize,
    ) -> Result<LevelRecords, OutOfDomain> {
        let mut level = self.strip_level_records(before, after)?;
        let records = &mut level.fader;
        let change = self.change(before, after);
        let [before_reads, next_reads] = &self.rules.automation;
        let lost =
            |lanes: [bool; 2], kept: [bool; 2]| [0, 1].map(|lane| lanes[lane] && !kept[lane]);
        // #1655 D6: an event lane (an own curve, or a reaching automated VCA's) that the change
        // makes a plain lane is released; a lane that stays an event lane keeps its composed
        // stage, which the carry repositions.
        let fader_released = lost(
            before_reads
                .view()
                .fader_event_lanes(change.id, change.reach_before),
            next_reads.view().fader_event_lanes(change.id, change.reach),
        );
        let mute_released = lost(
            before_reads
                .view()
                .mute_event_lanes(change.id, change.reach_before),
            next_reads.view().mute_event_lanes(change.id, change.reach),
        );
        if fader_released == [false; 2] && mute_released == [false; 2] {
            return Ok(level);
        }
        let mut released = [fader_released, mute_released];
        for record in records.iter().flatten() {
            let (kind, lanes) = match record {
                TrackFaderRecord::FaderDb { lanes, .. } => (0, lanes),
                TrackFaderRecord::Mute { lanes, .. } => (1, lanes),
            };
            let covered = match lanes {
                BuiltinLaneSelector::Left => [true, false],
                BuiltinLaneSelector::Right => [false, true],
                BuiltinLaneSelector::Both => [true, true],
            };
            for (released, covered) in released[kind].iter_mut().zip(covered) {
                *released &= !covered;
            }
        }
        let [fader_released, mute_released] = released;
        let lengths = self.rules.lengths(&change);
        let strip = &self.after.strips[after];
        let (db, muted) = (strip.fader_db, strip.mute);
        // The array holds at most one `FaderDb` and one `Mute` per lane, and a released lane has
        // no record of its kind yet, so a free slot is always there.
        let mut free = records.iter_mut().filter(|record| record.is_none());
        for (lanes, lane) in lane_records(
            fader_released,
            db[0].to_bits() == db[1].to_bits() && lengths.fader[0] == lengths.fader[1],
        ) {
            if let Some(slot) = free.next() {
                *slot = Some(TrackFaderRecord::FaderDb {
                    lanes,
                    db: db[lane],
                    smoothing_samples: lengths.fader[lane],
                });
            }
        }
        for (lanes, lane) in lane_records(
            mute_released,
            muted[0] == muted[1] && lengths.mute[0] == lengths.mute[1],
        ) {
            if let Some(slot) = free.next() {
                *slot = Some(TrackFaderRecord::Mute {
                    lanes,
                    muted: muted[lane],
                    smoothing_samples: lengths.mute[lane],
                });
            }
        }
        Ok(level)
    }

    #[cfg(feature = "control-provider")]
    /// The effective gate of route `index`, per source lane, in `[current, next]`.
    pub(crate) fn gates(&self, index: usize) -> [[bool; 2]; 2] {
        self.route_gates(index, index)
    }

    /// The effective gate of the route at `before` in `current` and at `after` in `next`, per
    /// source lane, in `[current, next]` (#1284 D3): a successor's routes need not sit at the same
    /// index in the two models.
    pub(crate) fn route_gates(&self, before: usize, after: usize) -> [[bool; 2]; 2] {
        [
            self.before.routes[before].gate,
            self.after.routes[after].gate,
        ]
    }

    /// The ramp length of a route record whose gate changed (#1558 D7); see
    /// [`LevelRules::gate_length`].
    pub(crate) fn gate_length(
        &self,
        before: &session::Route,
        after: &session::Route,
        own_edit: Option<u32>,
        source: (usize, usize),
        gates: [[bool; 2]; 2],
    ) -> u32 {
        self.rules.gate_length(
            before,
            after,
            own_edit,
            &self.change(source.0, source.1),
            gates,
        )
    }

    /// The effective mute of a strip per lane, at `before` in `current` and at `after` in `next`,
    /// in `[current, next]`: a following route's `terms` (#1652 D1, OQ1 = C; it holds no curve).
    pub(crate) fn strip_mutes(&self, before: usize, after: usize) -> [[bool; 2]; 2] {
        [
            self.before.strips[before].mute,
            self.after.strips[after].mute,
        ]
    }

    /// The strip index of a route's source in the two models, or `None` if it names no strip in
    /// one of them.
    pub(crate) fn source_strips(
        &self,
        before: &session::Route,
        after: &session::Route,
    ) -> Option<(usize, usize)> {
        Some((
            route_source_strip(self.rules.current, before)?,
            route_source_strip(self.rules.next.structure(), after)?,
        ))
    }
}

/// The ID of the strip `route` takes its source from.
fn route_source_id(route: &session::Route) -> &session::StableId {
    match &route.source {
        session::RouteSource::Track { track_id, .. } => track_id,
        session::RouteSource::Submix { submix_id, .. } => submix_id,
    }
}

/// Whether a plan of `model` gives `route` the seven-word cell of a following route (#1652 D1,
/// #1655 D4): it goes into a submix and a lane of its source strip is a mute event lane
/// ([`controls::mute_event_lanes`]: its own mute is automated, the entries the builtins plan
/// compiles mute cells from, or an automated VCA `mute` lane reaches it). Every other route keeps
/// the six-word cell. A route into the output never follows.
#[must_use]
pub(crate) fn follows_automated_mute(
    model: controls::ModelView<'_>,
    route: &session::Route,
) -> bool {
    following_route_lanes(model, route).is_some()
}

/// The source lanes whose mute a plan of `model` automates for `route` (own, or a reaching
/// automated VCA's), when that plan gives `route` the seven-word cell of a following route
/// ([`follows_automated_mute`]), else `None`: the `automated` bits of the route's records (#1652
/// D1, #1653 D1, #1655 D4). The source's VCA reach is read only when an entry drives a VCA `mute`
/// row and an own mute lane is not automated. `model`'s committed model is read; lookups when it
/// is indexed (#1844).
#[must_use]
pub(crate) fn following_route_lanes(
    model: controls::ModelView<'_>,
    route: &session::Route,
) -> Option<[bool; 2]> {
    if !matches!(
        route.destination,
        session::RouteDestination::SubmixInput { .. }
    ) || model.structure().automation.is_empty()
    {
        return None;
    }
    let source = route_source_id(route);
    // The own mute lanes: the event lanes with no reach.
    let own = model.mute_event_lanes(source, &[]);
    let lanes = if own == [true; 2] || !model.any_vca_mute() {
        own
    } else {
        model.mute_event_lanes(source, &model.structure().vca_reach_of(source))
    };
    lanes.contains(&true).then_some(lanes)
}

/// The jump lengths of a following route's cursor rows (#1652 D2, #1652b, #1655 D4): the row
/// rules over the block's session lengths, the rules the strip's own stages resolve
/// ([`LiveRamps::for_row`] of [`LiveRampRow::Mute`] for a source lane's own mute and of
/// [`LiveRampRow::VcaMute`] for a reaching VCA's mute), so the strip ramp and the send ramp have
/// one length. Called on render (through the route's `GraphRouteJumpRule`): `const fn` calls, a
/// struct build and `match` reads, allocation-free and panic-free.
// REALTIME_POLICY_BEGIN: the follow curves' jump rule a following route calls once per block.
fn follow_route_jumps(lengths: GraphJumpLengths) -> graph::GraphRouteJumpLengths {
    let ramps = LiveRamps::from_lengths(
        lengths.fader_samples,
        lengths.mute_samples,
        lengths.pan_samples,
    );
    graph::GraphRouteJumpLengths {
        mute: ramps.for_row(LiveRampRow::Mute),
        vca_mute: ramps.for_row(LiveRampRow::VcaMute),
    }
}
// REALTIME_POLICY_END

/// The index in [`SessionModel::strips`] of the strip `route` takes its source from, or `None` if
/// the model has no such strip.
pub(crate) fn route_source_strip(model: &SessionModel, route: &session::Route) -> Option<usize> {
    controls::strip_index(model, route_source_id(route))
}

/// The matrix record that brings a strip's matrix/pan stage from `before`'s value to `after`'s,
/// when a lowered coefficient's bits change (the per-owner function of #1277 D5). A change of the
/// smoothing window alone is no record.
///
/// # Errors
///
/// [`OutOfDomain`] when either value does not lower through [`lower_matrix_or_pan`].
pub(crate) fn matrix_record(
    before: &MatrixOrPan,
    after: &MatrixOrPan,
    ramps: LiveRamps,
    edit: StripEditRamps,
) -> Result<Option<TrackControlRecord>, OutOfDomain> {
    let bits = |matrix: builtins::Matrix2x2| {
        [matrix.ll, matrix.lr, matrix.rl, matrix.rr].map(f32::to_bits)
    };
    let (matrix_before, _) = lower_matrix_or_pan(before).map_err(|_| OutOfDomain)?;
    let (matrix_after, smoothing_after) = lower_matrix_or_pan(after).map_err(|_| OutOfDomain)?;
    Ok(
        (bits(matrix_before) != bits(matrix_after)).then_some(TrackControlRecord {
            matrix: matrix_after,
            // #1394 D5: the edit's own ramp; else #1054 D5, the post-commit model's own window if
            // it has one, else the session's `pan_ms` ramp. A pan and a raw matrix share the
            // row's key.
            smoothing_samples: if edit.matrix.is_none() && smoothing_after != 0 {
                smoothing_after
            } else {
                ramps.resolve(
                    match after {
                        MatrixOrPan::Pan { .. } => LiveRampRow::Pan,
                        MatrixOrPan::Matrix { .. } => LiveRampRow::Matrix,
                    },
                    edit.matrix,
                )
            },
        }),
    )
}

/// The lane records for one per-lane change mask: one `Both` when both lanes change and
/// `same_value`, else one record per changed lane, left first. Each entry names the lane whose
/// post-commit value the record carries.
pub(crate) fn lane_records(
    changed: [bool; 2],
    same_value: bool,
) -> impl Iterator<Item = (BuiltinLaneSelector, usize)> {
    let records: [Option<(BuiltinLaneSelector, usize)>; 2] = match changed {
        [true, true] if same_value => [Some((BuiltinLaneSelector::Both, 0)), None],
        [left, right] => [
            left.then_some((BuiltinLaneSelector::Left, 0)),
            right.then_some((BuiltinLaneSelector::Right, 1)),
        ],
    };
    records.into_iter().flatten()
}

/// The input records that bring a strip's input section from `before`'s values to `after`'s
/// (#1556 D2-D4), in write order: trims, then polarities, then filter targets, each left before
/// right. A record is emitted only when the value the render plane holds changes.
///
/// - `TrimDb`: one per lane whose [`checked_trim_gain`] bits change, or one `Both` when both lanes
///   change to the same `trim_db` bits. The comparison is of the gain's bits, so `0.0` to `-0.0`
///   (the same gain) gives no record.
/// - `PolarityInvert`: one per lane whose value changes, or one `Both` when both change to the
///   same value.
/// - `PreparedFilter`: the targets [`InputFilterPreparer`] designs, off the render thread, from the
///   pre-commit normalized pairs and the changed values (see [`filter_edits`]).
///
/// # Errors
///
/// [`OutOfDomain`] when a trim is outside the domain, or a pair or a designed target is
/// refused.
pub(crate) fn input_records(
    before: &session::DualMonoBuiltins,
    after: &session::DualMonoBuiltins,
    sample_rate_hz: u32,
    ramps: LiveRamps,
    edit: StripEditRamps,
) -> Result<Vec<TrackInputRecord>, OutOfDomain> {
    let lanes_before = [&before.left, &before.right];
    let lanes_after = [&after.left, &after.right];
    let mut records = Vec::with_capacity(4 + INPUT_FILTER_TARGET_CAPACITY);

    let gain_bits = |db: f32| {
        checked_trim_gain(db)
            .map(f32::to_bits)
            .map_err(|_| OutOfDomain)
    };
    let mut trim_changed = [false; 2];
    for lane in 0..2 {
        trim_changed[lane] =
            gain_bits(lanes_before[lane].trim_db)? != gain_bits(lanes_after[lane].trim_db)?;
    }
    for (lanes, lane) in lane_records(
        trim_changed,
        lanes_after[0].trim_db.to_bits() == lanes_after[1].trim_db.to_bits(),
    ) {
        records.push(TrackInputRecord::TrimDb {
            lanes,
            db: lanes_after[lane].trim_db,
            smoothing_samples: ramps.resolve(LiveRampRow::InputTrim, edit.input),
        });
    }

    let polarity_changed =
        [0, 1].map(|lane| lanes_before[lane].polarity_invert != lanes_after[lane].polarity_invert);
    for (lanes, lane) in lane_records(
        polarity_changed,
        lanes_after[0].polarity_invert == lanes_after[1].polarity_invert,
    ) {
        records.push(TrackInputRecord::PolarityInvert {
            lanes,
            inverted: lanes_after[lane].polarity_invert,
            smoothing_samples: ramps.resolve(LiveRampRow::PolarityInvert, edit.input),
        });
    }

    // The filters: compare the values each lane's pair normalizes to, so `-0.0` to `0.0` is no
    // change (#1556 D4).
    let pair = |lane: &session::ChannelBuiltins| {
        validate_input_filter_pair(sample_rate_hz, lane.hpf_hz, lane.lpf_hz)
            .map_err(|_| OutOfDomain)
    };
    let pairs_before = [pair(lanes_before[0])?, pair(lanes_before[1])?];
    let pairs_after = [pair(lanes_after[0])?, pair(lanes_after[1])?];
    let mut changed = [(false, false); 2];
    for lane in 0..2 {
        changed[lane] = (
            pairs_before[lane].hpf_hz.to_bits() != pairs_after[lane].hpf_hz.to_bits(),
            pairs_before[lane].lpf_hz.to_bits() != pairs_after[lane].lpf_hz.to_bits(),
        );
    }
    let edits = filter_edits(changed, pairs_after);
    if !edits.is_empty() {
        let seeds = [
            pairs_before[0].hpf_hz,
            pairs_before[0].lpf_hz,
            pairs_before[1].hpf_hz,
            pairs_before[1].lpf_hz,
        ];
        let mut out = [PreparedInputFilterTarget {
            lanes: BuiltinLaneSelector::Both,
            section: 0,
            pair: [0.0; 2],
            coefficients: [0.0; 6],
        }; INPUT_FILTER_TARGET_CAPACITY];
        let (_, count) = InputFilterPreparer
            .prepare(sample_rate_hz, &seeds, &edits, &mut out)
            .map_err(|_| OutOfDomain)?;
        records.extend(
            out[..count]
                .iter()
                .map(|&target| TrackInputRecord::PreparedFilter { target }),
        );
    }
    Ok(records)
}

/// The semantic edits that carry a strip's changed filter values (#1556 D4): per lane, `3` or `4`
/// for a one-sided change and the atomic pair `0` when both of the lane's values change (so a
/// pair that two one-sided edits would refuse, an HPF raised past the old LPF while the LPF
/// rises, is one valid edit). The same edit on both lanes is one `Both` edit, which is how the
/// preparer later merges equal targets.
fn filter_edits(
    changed: [(bool, bool); 2],
    after: [builtins::InputFilterPair; 2],
) -> Vec<InputFilterEdit> {
    let lane_edit = |lane: usize| {
        let pair = after[lane];
        match changed[lane] {
            (false, false) => None,
            (true, false) => Some((3, pair.hpf_hz, 0.0)),
            (false, true) => Some((4, pair.lpf_hz, 0.0)),
            (true, true) => Some((0, pair.hpf_hz, pair.lpf_hz)),
        }
    };
    let (left, right) = (lane_edit(0), lane_edit(1));
    let edit = |(parameter_id, value0, value1): (u32, f32, f32), channel| InputFilterEdit {
        parameter_id,
        channel,
        value0,
        value1,
    };
    match (left, right) {
        (Some(left), Some(right))
            if left.0 == right.0
                && left.1.to_bits() == right.1.to_bits()
                && left.2.to_bits() == right.2.to_bits() =>
        {
            vec![edit(left, ParameterChannel::Both)]
        }
        _ => left
            .map(|left| edit(left, ParameterChannel::Left))
            .into_iter()
            .chain(right.map(|right| edit(right, ParameterChannel::Right)))
            .collect(),
    }
}

/// The refusal of a successor whose revision its gate cannot stamp (#1277 D10).
fn successor_revision_refusal() -> PrepareDiagnostics {
    PrepareDiagnostics::fixed(PrepareRejection::Session, "host.successor.revision")
}

/// The records that bring one effect instance from `before` to `after`, and the targets of an
/// instance that rides prepared targets (#1264 D2, #1265 D1, #1266 D1).
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct EffectInstanceRecords {
    /// One [`EffectControlRecord::Bypass`] first when the bypass changes (it carries the session's
    /// `mute_ms` crossfade, [`LiveRampRow::BypassCrossfade`], #1341 D1), then one
    /// [`EffectControlRecord::Link`] when the resolved link mode of an effect on
    /// [`controls::LIVE_LINK_EFFECTS`] changes (its glide is the edit's link ramp, else
    /// [`LiveRampRow::LinkGlide`], #1371 D3), then one `Parameter` record per
    /// `(parameter_index, channel)` whose resolved value changes, in descriptor order, `Left`
    /// before `Right`; never empty. For an instance with `targets` the `Parameter` records
    /// are its owner's edits, never written as they are.
    pub(crate) records: Vec<EffectControlRecord>,
    /// The targets [`EqTargetPreparer`] designed from the pre-commit values and the records.
    pub(crate) targets: Option<Vec<PreparedEffectTarget>>,
}

/// Why a delta needs a plan rebuild. It lives here, beside `effect_instance_records`, because the
/// successor join uses it in every build; `live_delta` (feature `control-provider`) re-exports it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveRebuild {
    /// The models differ in something that is not a control value: the strip, route, slot or
    /// insert set or order, an effect's identity, quality or sidechain, a profile's other fields,
    /// or any other field no control row describes.
    Structure,
    /// A fader, pan/matrix or input value is outside the domain its render-side setter accepts, a
    /// send's gain or matrix is outside the route domain or its ramp is too long, or an
    /// effect's pre- or post-commit `params` do not resolve. The rebuild then reports the same
    /// preparation diagnostic it always has.
    Domain,
    /// A control whose row is rebuild-only changes, for the reason its row states (decision 17,
    /// C3). This replaces the former `Prepared` and `PreparedBypass`: an effect parameter the plan
    /// keeps prepared (an EQ band's `enabled` and `kind` among them, decision 14 F2), the bypass
    /// of the multiband compressor (decision 14 F4), an input value, a route value.
    RebuildOnly(RebuildReason),
    /// The stored automation changes and one of `next`'s automations names a row that its rule
    /// refuses ([`controls::automation_diagnostics`] is not empty, #1566 D6, decision 15 E1). The
    /// rebuild's preparation refuses it with the row's code; the classifier only routes, so the
    /// refusal has one author and no live commit can ack it first.
    AutomationTarget,
    /// The stored automation of a row that renders ([`controls::automation_row_renders`]: a
    /// strip's fader and mute, #1648, #1651) changes: an entry is added, removed, retargeted or
    /// given other segments (#1648 D1,
    /// [`controls::rendered_automation_changed`]). A plan compiles its cells from the entries at
    /// preparation, so only a successor plan holds the new curve. An entry's hand offset (#1650) is
    /// not part of this comparison.
    Automation,
}

/// The one per-instance function that decides whether a difference between two lowered instances
/// is live or prepared, and derives the records when it is live (decision 17, #1280 D1, D4). The
/// classifier lowers a live commit with it, and a successor's join asks it for every surviving
/// instance, so a difference is live in the carry exactly when it is live on the live path.
///
/// `Ok(None)` is no difference in any value the plan keeps. `Ok(Some(_))` is a live difference:
/// the bypass of an effect that lowers it to a lane shunt, the resolved link mode of an effect on
/// [`controls::LIVE_LINK_EFFECTS`] (#1371 D4), and each `Block`-rate parameter whose resolved
/// value bits change. `Err` is a prepared difference: another structure (identity, quality,
/// sidechain, or the link mode of any other effect), a rebuild-only row (the multiband compressor
/// bypass, a parameter that is not `Block`-rate), a parameter that does not resolve, or a ramp
/// that the indexed law refuses.
///
/// `link_edit` is the length the transaction gave the strip's link row (`(Strip, Link)`, #1394
/// D3), or `None` for the session's `fader_ms` ([`LiveRamps::resolve`]).
pub(crate) fn effect_instance_records(
    before: &Effect,
    after: &Effect,
    sample_rate_hz: u32,
    ramps: LiveRamps,
    link_edit: Option<u32>,
) -> Result<Option<EffectInstanceRecords>, LiveRebuild> {
    if before.id != after.id
        || before.identity != after.identity
        || before.quality != after.quality
        || before.sidechain != after.sidechain
    {
        return Err(LiveRebuild::Structure);
    }
    // This lowering carries a bypass, a link mode and parameters. A changed link mode of an effect
    // whose row is not live has no record here, so a live classification of it would be acked
    // and then dropped: it is a rebuild (D15-9 duck-swap).
    let link = if before.link_mode == after.link_mode {
        None
    } else {
        let live = matches!(
            &after.identity,
            EffectIdentity::Native { effect_id }
                if controls::LIVE_LINK_EFFECTS.contains(&effect_id.as_str())
        );
        if !live {
            return Err(LiveRebuild::Structure);
        }
        let ramp_samples = ramps.resolve(LiveRampRow::LinkGlide, link_edit);
        if ramp_samples > effect_contract::LINK_GLIDE_MAX_SAMPLES {
            return Err(LiveRebuild::Domain);
        }
        Some(EffectControlRecord::Link(effect_contract::LinkSwitch {
            mode: crate::response::contract_link_mode(after.link_mode),
            ramp_samples,
        }))
    };
    let bypass = (before.bypass != after.bypass).then_some(after.bypass);
    let (mut records, targets) = if same_params(&before.params, &after.params) {
        (Vec::new(), None)
    } else {
        parameter_records(before, after, sample_rate_hz)?
    };
    // Drain order (#1371 D2): bypass, link, parameters.
    if let Some(link) = link {
        records.insert(0, link);
    }
    if let Some(bypassed) = bypass {
        // The bypass crossfade has no per-edit length (#1341 D1): the session's `mute_ms`.
        let ramp_samples = ramps.for_row(LiveRampRow::BypassCrossfade);
        if ramp_samples > INDEXED_RAMP_LENGTH_MAXIMUM {
            return Err(LiveRebuild::Domain);
        }
        records.insert(
            0,
            EffectControlRecord::Bypass(BypassSwitch {
                bypassed,
                ramp_samples,
            }),
        );
    }
    Ok((!records.is_empty()).then_some(EffectInstanceRecords { records, targets }))
}

/// The parameter records of one instance, and the targets of a target-capable one.
type ParameterRecords = (Vec<EffectControlRecord>, Option<Vec<PreparedEffectTarget>>);

/// The parameter records that bring one instance from `before`'s resolved values to `after`'s,
/// and, for a target-capable instance (the EQ), the targets that carry them (#1265 D1).
fn parameter_records(
    before: &Effect,
    after: &Effect,
    sample_rate_hz: u32,
) -> Result<ParameterRecords, LiveRebuild> {
    // A third-party or unknown identity never prepared; the rebuild reports it.
    let EffectIdentity::Native { effect_id } = &after.identity else {
        return Err(LiveRebuild::Structure);
    };
    let registry = launch_native_effect_registry().map_err(|_| LiveRebuild::Structure)?;
    let factory = registry
        .get_shared_ascii(effect_id.as_str())
        .ok_or(LiveRebuild::Structure)?;
    let descriptor = factory.descriptor();
    let values_before =
        resolve_initial_values(descriptor, &before.params).map_err(|_| LiveRebuild::Domain)?;
    let values_after =
        resolve_initial_values(descriptor, &after.params).map_err(|_| LiveRebuild::Domain)?;
    let mut records = Vec::new();
    for (value_before, value_after) in values_before.iter().zip(&values_after) {
        if value_before.value.to_bits() == value_after.value.to_bits() {
            continue;
        }
        let parameter = &descriptor.parameters[value_after.parameter_index as usize];
        // A row is live only when the plan keeps a block-rate cell for it, and the control rows
        // say so: only a `Block`-rate parameter is `Live` (`controls::effect_rows`), a `None`-rate
        // one is prepared. This guard keeps the classifier from trusting a row that ever claimed
        // more than the plan holds.
        if parameter.automation_rate != AutomationRate::Block {
            return Err(LiveRebuild::RebuildOnly(RebuildReason::Correctness));
        }
        records.push(EffectControlRecord::Parameter {
            parameter_index: value_after.parameter_index,
            channel: value_after.channel,
            value: value_after.value,
        });
    }
    // The EQ's parameters ride its owner's prepared targets: its producer refuses a bare
    // parameter record (`EffectControlProducer::preflight`), so the records become the owner's
    // edits and the targets are designed here, on the control thread (#1265 D1).
    if records.is_empty() || factory.target_preparation().is_none() {
        return Ok((records, None));
    }
    let seeds: Vec<f32> = values_before.iter().map(|value| value.value).collect();
    let targets = design_targets(factory, sample_rate_hz, &seeds, &records)?;
    Ok((records, Some(targets)))
}

/// Designs a target-capable instance's targets with [`EqTargetPreparer`], from its pre-commit
/// `seeds` (canonical row order) and its changed `records` (#1265 D1).
fn design_targets(
    factory: std::sync::Arc<dyn NativeEffectFactory>,
    sample_rate_hz: u32,
    seeds: &[f32],
    records: &[EffectControlRecord],
) -> Result<Vec<PreparedEffectTarget>, LiveRebuild> {
    let descriptor = factory.descriptor();
    // A capability other than the 60-row EQ's has no control-thread designer: rebuild.
    let preparer = EqTargetPreparer::new(factory)
        .map_err(|_| LiveRebuild::RebuildOnly(RebuildReason::Correctness))?;
    if seeds.len() != EQ_VALUE_COUNT {
        return Err(LiveRebuild::RebuildOnly(RebuildReason::Correctness));
    }
    let edits: Vec<EqTargetEdit> = records
        .iter()
        .map(|record| match *record {
            EffectControlRecord::Parameter {
                parameter_index,
                channel,
                value,
            } => Ok(EqTargetEdit {
                parameter_id: descriptor.parameters[parameter_index as usize].id.0,
                channel,
                value,
            }),
            _ => Err(LiveRebuild::RebuildOnly(RebuildReason::Correctness)),
        })
        .collect::<Result<_, _>>()?;
    let mut out = [PreparedEffectTarget {
        slot: 0,
        channel: ParameterChannel::Both,
        words: [0; PREPARED_EFFECT_TARGET_WORDS],
    }; EQ_TARGET_CAPACITY];
    let (_, count) = preparer
        .prepare(sample_rate_hz, seeds, &edits, &mut out)
        .map_err(|error| match error {
            EqTargetPreparerError::Unsupported => {
                LiveRebuild::RebuildOnly(RebuildReason::Correctness)
            }
            _ => LiveRebuild::Domain,
        })?;
    Ok(out[..count].to_vec())
}

/// Whether two `params` lists are the same entries, value bits included.
pub(crate) fn same_params(before: &[EffectParam], after: &[EffectParam]) -> bool {
    before.len() == after.len()
        && before.iter().zip(after).all(|(left, right)| {
            left.parameter_id == right.parameter_id
                && left.channel == right.channel
                && left.unit == right.unit
                && left.value.to_bits() == right.value.to_bits()
        })
}

/// How one family of a strip's values differs between the predecessor's base and the successor's
/// model (#1277 D3): by the records the classifier would emit for the change.
#[derive(Clone, Copy, Debug)]
enum FamilyJoin<T> {
    /// No record: the stage's values are what the base holds.
    Same,
    /// The change is live: the records bring the carried stage to the successor's values.
    Live(T),
    /// The change is prepared (or its domain is refused): the stage starts at rest.
    Prepared,
}

impl<T> FamilyJoin<T> {
    const fn is_prepared(&self) -> bool {
        matches!(self, Self::Prepared)
    }
}

/// One strip present in both models, with how its fader/mute and matrix families differ.
struct StripJoin<'a> {
    id: &'a str,
    fader: FamilyJoin<LevelRecords>,
    matrix: FamilyJoin<TrackControlRecord>,
    /// The fader lanes whose stored automation table both models hold bit for bit (#1648 D2,
    /// [`controls::continued_fader_lanes`]), bit 0 left and bit 1 right, and the mute lanes of
    /// the same rule (#1651 T4, [`controls::continued_mute_lanes`]), bit 2 left and bit 3 right:
    /// the fader move's [`graph::GraphStripMove::continued_cells`].
    continued_cells: u8,
}

impl StripJoin<'_> {
    /// Whether any of the strip's stages could carry.
    fn may_carry(&self) -> bool {
        !self.fader.is_prepared() || !self.matrix.is_prepared()
    }
}

/// The strip `strip` had in `committed`, among strips of its own kind (#1277 D1): its position in
/// the model's strip order and its fader and matrix. A strip whose kind changed is not found.
fn committed_strip<'a>(
    committed: &'a SessionModel,
    strip: &StripRef<'_>,
) -> Option<(usize, &'a session::DualMonoFader, &'a MatrixOrPan)> {
    let id = strip.id.as_str();
    match strip.kind {
        StripKind::Track(_) => committed
            .tracks
            .binary_search_by(|track| track.id.as_str().cmp(id))
            .ok()
            .map(|index| {
                let track = &committed.tracks[index];
                (index, &track.fader, &track.matrix_or_pan)
            }),
        StripKind::Submix(_) => committed
            .submixes
            .binary_search_by(|submix| submix.id.as_str().cmp(id))
            .ok()
            .map(|index| {
                let submix = &committed.submixes[index];
                (
                    committed.tracks.len() + index,
                    &submix.fader,
                    &submix.matrix_or_pan,
                )
            }),
    }
}

/// The strips of `model` that `committed` has no strip of any kind for (issue #1288 D1): the
/// strips a successor adds. A strip whose ID `committed` holds as the other kind is an edited
/// strip (#1324), not an added one.
fn added_strips<'a>(committed: &SessionModel, model: &'a SessionModel) -> Vec<&'a str> {
    model
        .strips()
        .map(|strip| strip.id.as_str())
        .filter(|id| !committed_has_strip(committed, id))
        .collect()
}

/// One route a successor arms to fade in (#1363 D1 (a), (c)), and the strip whose fade the
/// route's fire waits for: its source strip when that strip's fader is armed too (it is added, or
/// it holds an unfired fade entry of the predecessor), else `None`.
struct RouteFade<'a> {
    route: &'a session::Route,
    strip: Option<&'a str>,
    /// The predecessor's unfired entry for the route, whose render state this arm takes at the
    /// swap block (#1877 D2); `None` for a route armed afresh.
    carried: Option<u32>,
}

/// The routes of `model` a successor arms to fade in (#1363 D1 (a), (c)), in `model`'s route
/// order: (a) a route `committed` does not hold, or holds under its ID with another source, tap
/// or destination (a re-point, #1953a; from a lane-less route too, #1953b, whose old half fades
/// out as a fading copy), whose source strip and destination it does hold, and (c) a route out
/// of an added strip whose tap precedes the fader. A route from a
/// strip whose fader is armed (`added`, or `waiting`: a strip with an unfired predecessor entry)
/// and whose tap follows the fader takes none (the fader's fade shapes it), and neither does a
/// route whose destination strip is added (its fader's fade does). A route the session mutes
/// arms as any other when its mute is live in this plan (`live_mute`, #2069): its fire goes to
/// its prepared mute, so an unmute before the fire opens it at the fire, over the fade; one whose
/// mute is not live (no lane: no value can unmute it before the fire) arms nothing. A route that
/// follows an automated source mute ([`follows_automated_mute`]) arms as any other (#2011): its
/// fire composes with the curve it hears. A tap before an armed fader waits for that strip's
/// fade. #2077: (a) also takes a lane-less route `model` keeps with the same endpoints and other
/// values (`retargeted`, [`crossfades_values`]): it fades in at its new values as its old ones
/// fade out in a fading copy ([`route_copies`]); a kept route this model mutes arms nothing.
///
/// #1877 D2: a route of `waiting_routes` (one whose predecessor entry has not fired, with that
/// entry) that `committed` holds with the same source, tap and destination is armed again on the
/// terms of (a), carrying the entry, whatever its live op's kind in either plan, a route this
/// model mutes included (#2069); one this model re-points is not (the displaced arm then fires
/// at the swap, #1363 D3a).
fn added_route_fades<'a>(
    committed: &SessionModel,
    model: &'a SessionModel,
    added: &[&str],
    waiting: &[&str],
    waiting_routes: &[(&str, u32)],
    live_mute: impl Fn(&session::Route) -> bool,
    retargeted: impl Fn(&session::Route) -> bool,
) -> Vec<RouteFade<'a>> {
    // #1953a (D15-9): a route `committed` holds under the same ID with another source, tap or
    // destination is a removal plus an addition, so it is not known: it arms as added. #1953b:
    // so is one whose old route has no lane; its removal half fades out as a fading copy
    // ([`route_copies`]). #2077: a known lane-less route whose values change arms as added too
    // (`retargeted`); its old values fade out in a fading copy.
    let same_endpoints = |route: &session::Route| {
        committed.routes.iter().any(|before| {
            before.id == route.id
                && before.source == route.source
                && before.destination == route.destination
        })
    };
    let known = same_endpoints;
    let mut fades = Vec::new();
    for route in &model.routes {
        if route.mute && !live_mute(route) {
            continue;
        }
        let (source, tap) = match &route.source {
            session::RouteSource::Track { track_id, tap } => (track_id.as_str(), *tap),
            session::RouteSource::Submix { submix_id, tap } => (submix_id.as_str(), *tap),
        };
        let destination_known = match &route.destination {
            session::RouteDestination::SubmixInput { submix_id } => {
                if added.contains(&submix_id.as_str()) {
                    continue;
                }
                committed_has_strip(committed, submix_id.as_str())
            }
            session::RouteDestination::OutputInput { output_id } => committed
                .outputs
                .iter()
                .any(|output| output.id.as_str() == output_id.as_str()),
        };
        let armed_source = added.contains(&source) || waiting.contains(&source);
        let carried = waiting_routes
            .iter()
            .find(|(id, _)| *id == route.id.as_str())
            .filter(|_| same_endpoints(route))
            .map(|&(_, entry)| entry);
        let fade = if carried.is_some() {
            // #1877 D2: an armed route that keeps its endpoints, armed again.
            destination_known && (!armed_source || tap.precedes_fader())
        } else if added.contains(&source) {
            // (c): a send of an added strip, from a tap before its fader.
            tap.precedes_fader()
        } else {
            // (a): an added route between two strips that keep playing, or (#2077) a kept
            // lane-less route at new values, whose source `retargeted` never names armed (R2).
            (!known(route) || retargeted(route))
                && destination_known
                && (!armed_source || tap.precedes_fader())
        };
        if fade {
            fades.push(RouteFade {
                route,
                strip: armed_source.then_some(source),
                carried,
            });
        }
    }
    fades
}

/// One fading copy a successor compiles (#1953b): a lane-less route (`post_fader` or `post_pan`
/// into the output, decision 14's master fold, D15-9) that the transaction removes from, or
/// re-points on, a strip the successor keeps, or (#2077) whose values (gain, matrix or mute) it
/// changes ([`crossfades_values`]). The folded route has no live op to ramp in phase 1,
/// so the successor compiles the old route beside its own model, under a copy ID, as a transition
/// route at the predecessor's coefficients, armed to fade *out* from its first block (`f = 0`:
/// the destination line sits after the op). The copy takes the predecessor route's op state and
/// its lines (by alias), and later successors keep it, carrying both, until its fade entry has
/// settled: its ramp ended and its destination line delivered it.
struct RouteCopy {
    /// The copy: the old route's source, destination and values under the copy ID, unmuted.
    route: session::Route,
    /// The ID of the predecessor's route whose op and lines the copy takes: the removed route's
    /// for a fresh copy, the predecessor's copy's for a kept one.
    alias: Box<str>,
    /// The predecessor's fade-out entry a kept copy carries (#1392 D3); `None` for a fresh copy.
    carried: Option<u32>,
}

/// Whether a successor of `base` crossfades a value change of `after` (#2077): `after` is a
/// lane-less route (no live rows, so no lane can ramp it) that `committed` holds under its ID with
/// the same source and destination and other values (gain, matrix or mute: R3, a change of `mute`
/// alone is a value change). Its old values fade out in a fading copy when `committed` does not
/// mute it ([`route_copies`]) and its new values fade in when `after` is not muted
/// ([`added_route_fades`]), both from the swap block: the route's own arm waits for nothing
/// (`D = 0`), so the swap is its `S` and nothing carries (R1).
///
/// R2: never when the source strip's own fade owns the route: a strip with an unfired fade entry
/// of the predecessor (an added strip still waiting; its fader is shut until its fire, which is
/// pair-carried) or one a duck of the transaction already silences (the duck overlay's strips and
/// the transition fallback's).
///
/// A route whose predecessor arm had not fired at preparation (a plan replaced before it renders)
/// takes both halves too: its copy aliases the predecessor's op, which is mid-ramp when that arm
/// fired before the swap and settled at `[+0.0; 4]`, muted, when it did not (the copy then fades
/// out silence for one ramp), and the kept route is re-armed at its new values, carrying the entry
/// (#1877 D2).
fn crossfades_values(base: &SuccessorBase<'_>, after: &session::Route) -> bool {
    if controls::route_rows_live(after) {
        return false;
    }
    let Some(before) = base
        .committed
        .routes
        .iter()
        .find(|before| before.id == after.id)
    else {
        return false;
    };
    if before.source != after.source || before.destination != after.destination || before == after {
        return false;
    }
    let (strip, _, _) = source_strip(&after.source);
    let watch = &base.inventory.fade_watch;
    let waiting = base.inventory.fade_arms.iter().any(|row| {
        row.owner == FadeArmOwner::Strip && &*row.id == strip && !watch.fired(row.entry)
    });
    !base.duck.has_strip(strip) && !base.forced_restart.iter().any(|id| id == strip) && !waiting
}

/// The copy ID of a route `original` (#1953b): the shortest `original` followed by hyphens that
/// `taken` does not hold, so the copy sorts next to where `original` sorted among a destination's
/// routes (no ID lies between `original` and `original-`; `-` is the smallest ID byte). An ID at
/// the length limit takes `-` and a number on a shortened prefix instead. `None` only when no
/// such ID is free, which `taken` (a finite set) never makes so.
fn copy_route_id(original: &str, taken: impl Fn(&str) -> bool) -> Option<session::StableId> {
    let mut candidate = String::from(original);
    while candidate.len() < session::STABLE_ID_MAXIMUM_BYTES {
        candidate.push('-');
        if !taken(&candidate) {
            return session::StableId::parse(&candidate);
        }
    }
    (0_u64..).take(1 << 16).find_map(|number| {
        let suffix = format!("-{number}");
        let keep = session::STABLE_ID_MAXIMUM_BYTES.saturating_sub(suffix.len());
        let prefix: String = original.chars().take(keep).collect();
        let candidate = format!("{prefix}{suffix}");
        (!taken(&candidate))
            .then(|| session::StableId::parse(&candidate))
            .flatten()
    })
}

/// The fading copies a successor of `base` compiles beside `model` (#1953b), on the terms of an
/// added route's arm (a plan that attaches live lanes, never at a declared discontinuity, which
/// the caller checks): first each copy of the predecessor whose fade entry has not settled and
/// whose source strip `model` keeps, then a fresh copy of each lane-less route of `committed` that
/// `model` lacks, or holds under its ID with another source, tap or destination (a re-point,
/// D15-9: the removal half), or (#2077) holds with the same endpoints at other values (its old
/// values, [`crossfades_values`]), that the session does not mute, from a strip `model` keeps and that
/// no duck of the transaction already silences (the duck overlay's strips and the transition
/// fallback's). Each copy ID is free in both models and among the copies before it.
fn route_copies(base: &SuccessorBase<'_>, model: &SessionModel) -> Vec<RouteCopy> {
    // The copy's source strip, of the same kind: a strip replaced by one of the other kind under
    // its ID is a removal and an addition (#1324), whose routes its phase-1 duck silences.
    let keeps_source = |route: &session::Route| match &route.source {
        session::RouteSource::Track { track_id, .. } => model
            .tracks
            .binary_search_by(|track| track.id.as_str().cmp(track_id.as_str()))
            .is_ok(),
        session::RouteSource::Submix { submix_id, .. } => model
            .submixes
            .binary_search_by(|submix| submix.id.as_str().cmp(submix_id.as_str()))
            .is_ok(),
    };
    let mut copies: Vec<RouteCopy> = Vec::new();
    let taken = |copies: &[RouteCopy], id: &str| {
        model.routes.iter().any(|route| route.id.as_str() == id)
            || base
                .committed
                .routes
                .iter()
                .any(|route| route.id.as_str() == id)
            || copies.iter().any(|copy| copy.route.id.as_str() == id)
    };
    let watch = &base.inventory.fade_watch;
    for row in &base.inventory.route_copies {
        if watch.settled(row.entry) || !keeps_source(&row.route) {
            continue;
        }
        let held = row.route.id.as_str();
        let id = if taken(&copies, held) {
            copy_route_id(held, |id| taken(&copies, id))
        } else {
            Some(row.route.id.clone())
        };
        let Some(id) = id else {
            continue;
        };
        let mut route = row.route.clone();
        route.id = id;
        copies.push(RouteCopy {
            route,
            alias: held.into(),
            carried: Some(row.entry),
        });
    }
    for before in &base.committed.routes {
        if controls::route_rows_live(before) || before.mute || !keeps_source(before) {
            continue;
        }
        let (strip, _, _) = source_strip(&before.source);
        if base.duck.has_strip(strip) || base.forced_restart.iter().any(|id| id == strip) {
            continue;
        }
        // #2077: a kept route at other values is copied too, at its old values (a value change
        // fades out as a removal does; its kept route fades in, [`added_route_fades`]).
        let kept = model.routes.iter().any(|after| {
            after.id == before.id
                && after.source == before.source
                && after.destination == before.destination
                && !crossfades_values(base, after)
        });
        let destination_kept = match &before.destination {
            session::RouteDestination::OutputInput { output_id } => model
                .outputs
                .iter()
                .any(|output| output.id.as_str() == output_id.as_str()),
            session::RouteDestination::SubmixInput { .. } => false,
        };
        if kept || !destination_kept {
            continue;
        }
        let Some(id) = copy_route_id(before.id.as_str(), |id| taken(&copies, id)) else {
            continue;
        };
        let mut route = before.clone();
        route.id = id;
        copies.push(RouteCopy {
            route,
            alias: before.id.as_str().into(),
            carried: None,
        });
    }
    copies
}

/// The carry of each fading copy's lines (#1953b): each edge of a copy (its source and its
/// destination edge) takes the predecessor's line on the same edge of the route it replaces
/// (`alias`), when its endpoints are that line's; that line goes to the copy and to nothing else,
/// so a route re-pointed under the old ID (whose endpoints differ anyway) never takes it. Both
/// slices are sorted by edge.
fn alias_copy_lines(
    predecessor: &[graph::GraphDelayLine],
    own: &[graph::GraphDelayLine],
    copies: &[RouteCopy],
    moves: &mut Vec<graph::GraphDelayMove>,
) {
    let find = |lines: &[graph::GraphDelayLine], edge: &graph::GraphEdgeId| {
        lines
            .binary_search_by(|line| line.edge.cmp(edge))
            .ok()
            .map(|index| lines[index].clone())
    };
    for copy in copies {
        let (Some(id), Some(alias)) = (
            StableGraphId::parse(copy.route.id.as_str()),
            StableGraphId::parse(&copy.alias),
        ) else {
            continue;
        };
        let edges = [
            (
                graph::GraphEdgeId::RouteSource {
                    route_id: id.clone(),
                },
                graph::GraphEdgeId::RouteSource {
                    route_id: alias.clone(),
                },
            ),
            (
                graph::GraphEdgeId::RouteDestination { route_id: id },
                graph::GraphEdgeId::RouteDestination { route_id: alias },
            ),
        ];
        for (edge, alias_edge) in edges {
            let (Some(line), Some(earlier)) = (find(own, &edge), find(predecessor, &alias_edge))
            else {
                continue;
            };
            if earlier.source != line.source || earlier.destination != line.destination {
                continue;
            }
            moves.retain(|step| step.successor != line.line && step.predecessor != earlier.line);
            moves.push(graph::GraphDelayMove {
                successor: line.line,
                predecessor: earlier.line,
            });
        }
    }
}

/// The fade-out arm of each fading copy of `copies` (#1953b), in order: its target is its
/// transition route's unit in `live_routes` (sorted by route ID); it waits for nothing (`D = 0`:
/// the copy's destination line sits after its op, so the fade enters it at the first block); its
/// ramp is the session mute ramp, or 0 for a copy whose source strip `restarted` names; its tail
/// is its destination edge's line in `delay_lines` (sorted by edge). `None` when a copy has no
/// live route row (preparation then fails: every copy binds as a transition route).
fn copy_fade_arms(
    model: &SessionModel,
    copies: &[RouteCopy],
    live_routes: &[graph::GraphLiveRoute],
    delay_lines: &[graph::GraphDelayLine],
    restarted: impl Fn(&str) -> bool,
) -> Option<Vec<graph::GraphFadeArm>> {
    let ramp_samples = LiveRamps::for_session(model).mute_samples;
    copies
        .iter()
        .map(|copy| {
            let id = copy.route.id.as_str();
            let unit = live_routes
                .binary_search_by(|row| (*row.route_id).cmp(id))
                .ok()
                .map(|index| live_routes[index].unit)?;
            let edge = graph::GraphEdgeId::RouteDestination {
                route_id: StableGraphId::parse(id)?,
            };
            let tail_samples = delay_lines
                .binary_search_by(|line| line.edge.cmp(&edge))
                .ok()
                .map_or(Some(0), |index| {
                    u32::try_from(delay_lines[index].samples).ok()
                })?;
            let (strip, _, _) = source_strip(&copy.route.source);
            Some(graph::GraphFadeArm {
                target: graph::GraphFadeTarget::RouteOut(unit),
                source: None,
                delay_samples: 0,
                ramp_samples: if restarted(strip) { 0 } else { ramp_samples },
                tail_samples,
            })
        })
        .collect()
}

/// The fade arm of each route of `fades` (#1363 D3, D6): its target is its live route's unit in
/// `live_routes` (sorted by route ID), of either kind (#2011); `None` when one has no such row
/// (preparation then fails: every route of `fades` binds one).
///
/// `D` is the samples before the route's own op that start at rest: for a route out of a strip
/// whose fader is armed, the route's input arrival minus the strip's `Input` arrival (every node
/// latency and compensation line from the input to the tap and the route's source-edge line,
/// [`fade_arms_for`]'s rule, with a submix's largest route line into it), and its claim is the
/// strip's; for a route from a strip that keeps playing, only its source-edge line (in
/// `delay_lines`, sorted by edge), whose source is warm. A route's destination-edge line sits
/// after its op: the fade enters it, so it delays the fade as a whole and is not waited for. The
/// fade's length is the session mute ramp.
fn route_fade_arms(
    model: &SessionModel,
    fades: &[RouteFade<'_>],
    live_routes: &[graph::GraphLiveRoute],
    delay_lines: &[graph::GraphDelayLine],
    arrivals: &[(GraphNodeId, u64)],
) -> Option<Vec<graph::GraphFadeArm>> {
    let arrival = |node: &GraphNodeId| {
        arrivals
            .binary_search_by(|(row, _)| row.cmp(node))
            .map_or(0, |row| arrivals[row].1)
    };
    let ramp_samples = LiveRamps::for_session(model).mute_samples;
    let mut arms = Vec::with_capacity(fades.len());
    for fade in fades {
        let row = live_routes
            .binary_search_by(|row| (*row.route_id).cmp(fade.route.id.as_str()))
            .ok()
            .map(|index| &live_routes[index])?;
        let route_id = StableGraphId::parse(fade.route.id.as_str())?;
        let route_node = GraphNodeId::Route {
            route_id: route_id.clone(),
        };
        let (source, delay) = match fade.strip {
            Some(strip) => {
                let strip_id = StableGraphId::parse(strip)?;
                let input = GraphNodeId::TrackStage {
                    track_id: strip_id,
                    stage: TrackStage::Input,
                };
                let input_arrival = arrival(&input);
                let mut delay = arrival(&route_node).saturating_sub(input_arrival);
                let is_track = model
                    .tracks
                    .binary_search_by(|track| track.id.as_str().cmp(strip))
                    .is_ok();
                if !is_track {
                    delay = delay.saturating_add(
                        model
                            .routes
                            .iter()
                            .filter(|route| {
                                matches!(&route.destination,
                                    session::RouteDestination::SubmixInput { submix_id }
                                        if submix_id.as_str() == strip)
                            })
                            .filter_map(|route| StableGraphId::parse(route.id.as_str()))
                            .map(|route_id| {
                                input_arrival
                                    .saturating_sub(arrival(&GraphNodeId::Route { route_id }))
                            })
                            .max()
                            .unwrap_or(0),
                    );
                }
                (is_track.then_some(input), delay)
            }
            None => {
                let edge = graph::GraphEdgeId::RouteSource { route_id };
                let line = delay_lines
                    .binary_search_by(|line| line.edge.cmp(&edge))
                    .map_or(0, |index| delay_lines[index].samples);
                (None, line)
            }
        };
        arms.push(graph::GraphFadeArm {
            tail_samples: 0,
            target: graph::GraphFadeTarget::Route(row.unit),
            source,
            delay_samples: u32::try_from(delay).unwrap_or(u32::MAX),
            ramp_samples,
        });
    }
    Some(arms)
}

/// The fade arm of each strip of `strips` that has a fader lane in `stages` (sorted by strip ID,
/// then owner; issue #1288 D1, D4, D5): the one entry the arms of an added strip, a duck-swap (#1324) and a restore over a duck
/// (#1325) are built through.
///
/// `D` is read off PDC's own floored input arrivals (`arrivals`, sorted by node ID; decision 3):
/// the fader's arrival minus the strip's `Input` arrival is every node latency and compensation
/// line from the input to the fader, a sidechain's included, since PDC takes each node's arrival
/// as the maximum over its incoming edges. A submix adds the largest compensation line of a route
/// into it, the input arrival minus the route's. The fade's length is the session mute ramp.
fn fade_arms_for<'a>(
    model: &SessionModel,
    strips: &[&'a str],
    stages: &[StripStageInventoryRow],
    arrivals: &[(GraphNodeId, u64)],
) -> Vec<(&'a str, graph::GraphFadeArm)> {
    let arrival = |node: &GraphNodeId| {
        arrivals
            .binary_search_by(|(row, _)| row.cmp(node))
            .map_or(0, |row| arrivals[row].1)
    };
    let ramp_samples = LiveRamps::for_session(model).mute_samples;
    let mut arms = Vec::with_capacity(strips.len());
    for &id in strips {
        // `stages` is sorted by strip ID, then owner.
        let Ok(row) = stages.binary_search_by(|row| {
            row.id
                .as_ref()
                .cmp(id)
                .then(row.owner.cmp(&graph::BuiltinStripOwner::Fader))
        }) else {
            continue;
        };
        let row = &stages[row];
        let Some(track_id) = StableGraphId::parse(id) else {
            continue;
        };
        let node = |stage| GraphNodeId::TrackStage {
            track_id: track_id.clone(),
            stage,
        };
        let input = node(TrackStage::Input);
        let input_arrival = arrival(&input);
        let mut delay = arrival(&node(TrackStage::PostFader)).saturating_sub(input_arrival);
        let is_track = model
            .tracks
            .binary_search_by(|track| track.id.as_str().cmp(id))
            .is_ok();
        if !is_track {
            let into_submix = model
                .routes
                .iter()
                .filter(|route| {
                    matches!(&route.destination, session::RouteDestination::SubmixInput { submix_id }
                        if submix_id.as_str() == id)
                })
                .filter_map(|route| StableGraphId::parse(route.id.as_str()))
                .map(|route_id| {
                    input_arrival.saturating_sub(arrival(&GraphNodeId::Route { route_id }))
                })
                .max()
                .unwrap_or(0);
            delay = delay.saturating_add(into_submix);
        }
        // #2080: a strip the session mutes is armed too; its fire goes to its prepared mute, so
        // an unmute record before the fire opens it at the fire, over the fade.
        arms.push((
            id,
            graph::GraphFadeArm {
                tail_samples: 0,
                target: graph::GraphFadeTarget::Fader(row.location),
                source: is_track.then_some(input),
                delay_samples: u32::try_from(delay).unwrap_or(u32::MAX),
                ramp_samples,
            },
        ));
    }
    arms
}

/// Whether `committed` has a strip of any kind named `id`.
pub(crate) fn committed_has_strip(committed: &SessionModel, id: &str) -> bool {
    committed
        .tracks
        .binary_search_by(|track| track.id.as_str().cmp(id))
        .is_ok()
        || committed
            .submixes
            .binary_search_by(|submix| submix.id.as_str().cmp(id))
            .is_ok()
}

/// The strip a route or a sidechain reads, and its tap; whether that strip is a submix.
pub(crate) fn source_strip(source: &session::RouteSource) -> (&str, session::SendTap, bool) {
    match source {
        session::RouteSource::Track { track_id, tap } => (track_id.as_str(), *tap, false),
        session::RouteSource::Submix { submix_id, tap } => (submix_id.as_str(), *tap, true),
    }
}

/// The length, in `lines`, of the compensation line on the sidechain of effect `effect` of strip
/// `strip` into its port `port`; 0 when it has none.
pub(crate) fn sidechain_line_samples(
    lines: &[graph::GraphDelayLine],
    strip: &str,
    effect: &str,
    port: &str,
) -> u64 {
    lines
        .iter()
        .find(|line| {
            matches!(&line.edge, graph::GraphEdgeId::EffectSidechain { effect: node, port: at }
                if node.track_id.as_str() == strip
                    && node.effect_id.as_str() == effect
                    && at == port)
        })
        .map_or(0, |line| line.samples)
}

/// #1324 D1's exemption of a submix's `input` tap whatever its arrival: every line into submix
/// `submix`'s `Input` stage, in `committed` and in `model`, comes from a ducked strip (`ducked`,
/// which holds the removed strips) or, in `model`, from an added one. Its input then holds exact
/// `+0.0` from the end of the duck until the fire: a route of `model` from a ducked or added
/// strip's tap before its fader is armed, a following one too (#2011).
pub(crate) fn submix_input_is_quiet(
    committed: &SessionModel,
    model: &SessionModel,
    submix: &str,
    ducked: &dyn Fn(&str) -> bool,
) -> bool {
    let into = |route: &&session::Route| {
        matches!(&route.destination, session::RouteDestination::SubmixInput { submix_id }
            if submix_id.as_str() == submix)
    };
    committed
        .routes
        .iter()
        .filter(into)
        .all(|route| ducked(source_strip(&route.source).0))
        && model.routes.iter().filter(into).all(|route| {
            let source = source_strip(&route.source).0;
            !committed_has_strip(committed, source) || ducked(source)
        })
}

/// #1324 D1's sidechain closure, on the restart set `restarted` (sorted) of a successor of
/// `committed` prepared from `model`: a strip of both plans with an insert keyed from a tap of a
/// restarted strip that follows a restarted owner with state (`post_input`, `insert_send`,
/// `insert_return`, `pre_fader`) joins the set, repeated until no strip is added. A track's
/// `input` tap is its raw source, and a `post_fader` or `post_pan` tap holds exact `+0.0` from
/// the end of the duck until the fire, so neither adds its consumer, unless the strip's own delay
/// line restarts (`delay_restarted`, sorted): the `input` tap is that line's output, which then
/// starts at rest at `S`, so it adds its consumer, a track's or a submix's. Otherwise a submix's
/// `input` tap adds its consumer only when the sidechain's line changes length (`before` is the
/// predecessor's lines, `after` the successor's) and the submix's input is not quiet
/// ([`submix_input_is_quiet`]). Console slots have no sidechain.
fn close_over_sidechains(
    committed: &SessionModel,
    model: &SessionModel,
    before: &[graph::GraphDelayLine],
    after: &[graph::GraphDelayLine],
    delay_restarted: &[Box<str>],
    restarted: &mut Vec<Box<str>>,
) {
    loop {
        let held =
            |set: &[Box<str>], id: &str| set.binary_search_by(|held| (**held).cmp(id)).is_ok();
        let set: &[Box<str>] = restarted;
        let ducked = |id: &str| held(set, id) || !committed_has_strip(model, id);
        let keyed = |strip: &str, effect: &session::Effect| {
            let session::SidechainDeclaration::Routed(sidechain) = &effect.sidechain else {
                return false;
            };
            let (source, tap, submix) = source_strip(&sidechain.source);
            if !held(set, source) {
                return false;
            }
            match tap {
                session::SendTap::PostFader | session::SendTap::PostPan => false,
                session::SendTap::Input if held(delay_restarted, source) => true,
                session::SendTap::Input => {
                    let port = sidechain.port_id.as_str();
                    let id = effect.id.as_str();
                    submix
                        && sidechain_line_samples(before, strip, id, port)
                            != sidechain_line_samples(after, strip, id, port)
                        && !submix_input_is_quiet(committed, model, source, &ducked)
                }
                session::SendTap::PostInput
                | session::SendTap::InsertSend
                | session::SendTap::InsertReturn
                | session::SendTap::PreFader => true,
            }
        };
        let joined: Vec<Box<str>> = model
            .strips()
            .map(|strip| (strip.id.as_str(), strip.inserts))
            .filter(|(id, _)| !held(set, id) && committed_has_strip(committed, id))
            .filter(|(id, inserts)| inserts.effects.iter().any(|effect| keyed(id, effect)))
            .map(|(id, _)| Box::from(id))
            .collect();
        if joined.is_empty() {
            return;
        }
        restarted.extend(joined);
        restarted.sort_unstable();
        restarted.dedup();
    }
}

/// The model half of the strip join (#1277 D3, D4): for every strip of `model` that `committed`
/// also has, how its fader/mute and matrix values differ.
///
/// A family's difference is live when the classifier would emit a record for it on a plan that
/// attaches the control kind (`live`): the strip is a track or a submix (#1554) and its values are
/// in domain. The fader/mute family is judged on the strip's *effective* values in the two models
/// ([`EffectiveDiff`], decision 17 C7, #1558 D4), exactly as the classifier judges them: a VCA
/// move, a membership change and a followed mute are live differences like an own edit, and a
/// strip whose effective values do not change is `Same`. Each family is judged on its own values
/// (D4 decides per stage): an out-of-domain fader makes only the fader/mute family prepared, and
/// the matrix family of the same strip still carries and retargets. Every other difference is
/// prepared.
///
/// A change of the matrix's smoothing window alone is `Same` (as on the live path, whose
/// classifier emits no record for it): a carried matrix stage keeps the predecessor's window,
/// and every retarget passes its own. A strip whose kind changed has no counterpart and both
/// families are prepared.
fn join_strips<'a>(
    committed: &'a SessionModel,
    model: &'a SessionModel,
    live: bool,
    edit_ramps: &'a EditRamps,
    diff: &EffectiveDiff<'a>,
) -> Vec<StripJoin<'a>> {
    let ramps = LiveRamps::for_session(model);
    let mut joins = Vec::new();
    for (index, strip) in model.strips().enumerate() {
        let id = strip.id.as_str();
        let Some((position, _, before_matrix)) = committed_strip(committed, &strip) else {
            if committed_has_strip(committed, id) {
                joins.push(StripJoin {
                    id,
                    fader: FamilyJoin::Prepared,
                    matrix: FamilyJoin::Prepared,
                    continued_cells: 0,
                });
            }
            continue;
        };
        // D3 decides per value, D4 per stage: a refusal of one family's value makes only that
        // family prepared. The classifier refuses a whole delta, but that does not turn the other
        // stage's own live change into a prepared one.
        // A retarget is the record of the same edit, so it carries the length that edit gave its
        // row (#1394 D6), else the session's.
        let edit = StripEditRamps::of(edit_ramps, strip.id);
        let fader_records = diff.join_level_records(position, index);
        let matrix_record = matrix_record(before_matrix, strip.matrix_or_pan, ramps, edit);
        let fader_live = live && fader_records.is_ok();
        let matrix_live = live && matrix_record.is_ok();
        // An offsets-cell write alone (an `S` the transaction moves on an automated lane) is a
        // live difference only where the successor has cells to take it; on a plan with no live
        // cells the carried lane takes the change by the adoption rule (#1648 D2), so the stage
        // still carries as `Same`.
        let fader = match fader_records {
            Err(_) => FamilyJoin::Prepared,
            Ok(records)
                if records.fader[0].is_none() && (!fader_live || records.offsets == [None; 2]) =>
            {
                FamilyJoin::Same
            }
            Ok(_) if !fader_live => FamilyJoin::Prepared,
            Ok(records) => FamilyJoin::Live(records),
        };
        let matrix = match matrix_record {
            Err(_) => FamilyJoin::Prepared,
            Ok(None) => FamilyJoin::Same,
            Ok(Some(_)) if !matrix_live => FamilyJoin::Prepared,
            Ok(Some(record)) => FamilyJoin::Live(record),
        };
        let [continued, continued_mute] = diff.continued_lanes(Some(position), index, strip.id);
        let continued_cells = u8::from(continued[0])
            | (u8::from(continued[1]) << 1)
            | (u8::from(continued_mute[0]) << 2)
            | (u8::from(continued_mute[1]) << 3);
        joins.push(StripJoin {
            id,
            fader,
            matrix,
            continued_cells,
        });
    }
    joins
}

/// One strip present in both models, with the lanes whose `delay_samples` are equal (#1284 D1).
struct StripDelayJoin<'a> {
    id: &'a str,
    /// The lanes whose declared delay is the same in both models: [`graph::STRIP_DELAY_LEFT`] and
    /// [`graph::STRIP_DELAY_RIGHT`] bits. Each carries its pending samples when both plans hold the
    /// strip's line, whose lane rings are equally long, and the strip reads the same source.
    equal: u8,
    /// Whether the strip reads the same input in both models (#1402 R-B): a track whose
    /// `source_id` is unchanged and whose source's ring carries (#1272 D3), or a submix. A track's
    /// line holds its source's raw frames (its declared delay and, since #1287 R1, its
    /// source-claim line), so a track that reads another source, or a source redeclared under its
    /// ID (a fresh ring from frame 0), carries no lane.
    same_source: bool,
    /// Whether the strip is a track: only a track's line holds source frames, so only a track's
    /// line takes a warm successor's prime (#1355 D5).
    track: bool,
}

/// The model half of the strip delay join (#1284 D1): for every strip of `model` that `committed`
/// also has as a strip of the same kind, which lanes declare the same `delay_samples`, and whether
/// a track reads the same source, one of `carried`, the source IDs whose rings carry. A strip whose
/// kind changed has no counterpart and is not listed: it is a new strip.
fn join_strip_delays<'a>(
    committed: &'a SessionModel,
    model: &'a SessionModel,
    carried: &BTreeSet<&str>,
) -> Vec<StripDelayJoin<'a>> {
    model
        .strips()
        .filter_map(|strip| {
            let before = committed_input_section(committed, &strip)?;
            let lanes = [
                (&before.left, &strip.builtins.left),
                (&before.right, &strip.builtins.right),
            ];
            let equal = lanes
                .iter()
                .enumerate()
                .filter(|(_, (before, after))| before.delay_samples == after.delay_samples)
                .fold(0, |mask, (lane, _)| mask | (1_u8 << lane));
            let same_source = match strip.kind {
                session::StripKind::Track(track) => committed
                    .tracks
                    .binary_search_by(|before| before.id.as_str().cmp(track.id.as_str()))
                    .is_ok_and(|index| {
                        committed.tracks[index].source_id == track.source_id
                            && carried.contains(track.source_id.as_str())
                    }),
                session::StripKind::Submix(_) => true,
            };
            Some(StripDelayJoin {
                id: strip.id.as_str(),
                equal,
                same_source,
                track: matches!(strip.kind, session::StripKind::Track(_)),
            })
        })
        .collect()
}

/// What the strip delay join resolved against both plans' lines (#1284 D1).
#[derive(Default)]
struct StripDelayCarry {
    moves: Vec<graph::GraphStripDelayMove>,
    restarted: Vec<Box<str>>,
    /// A warm successor's source-claim fills (#1355 D5): every track line on a moved source with
    /// a lane that moves or grew by the lead. Empty without a lead.
    fills: Vec<graph::GraphClaimFill>,
}

/// The lanes of one strip's delay line that continue across a swap (#1284 D1, #1354a ruling 3):
/// `(moved, grown)`. A needed lane (equal declared delay, a nonempty ring in the successor) moves
/// when the predecessor holds the line with a ring of the same length, and grew when its ring is
/// exactly `lead` longer than the predecessor's (0 for a line it does not hold). `before` is
/// `None` when the predecessor holds no line; `lead` is 0 for every successor but a warm one.
fn strip_delay_lanes(equal: u8, before: Option<[u32; 2]>, after: [u32; 2], lead: u64) -> (u8, u8) {
    let mut moved = 0;
    let mut grown = 0;
    for (lane, bit) in [graph::STRIP_DELAY_LEFT, graph::STRIP_DELAY_RIGHT]
        .into_iter()
        .enumerate()
    {
        if equal & bit == 0 || after[lane] == 0 {
            continue;
        }
        if before.is_some_and(|before| before[lane] == after[lane]) {
            moved |= bit;
        }
        if lead != 0
            && u64::from(after[lane]) == u64::from(before.map_or(0, |before| before[lane])) + lead
        {
            grown |= bit;
        }
    }
    (moved, grown)
}

/// The lanes of strip `join`'s line in the successor (`after`, its lane rings) that hold pending
/// samples worth carrying: a nonempty ring on a lane whose declared delay is equal in both models.
fn needed_lanes(join: &StripDelayJoin<'_>, after: Option<[u32; 2]>) -> u8 {
    after.map_or(0, |after| {
        [graph::STRIP_DELAY_LEFT, graph::STRIP_DELAY_RIGHT]
            .into_iter()
            .zip(after)
            .filter(|&(bit, samples)| join.equal & bit != 0 && samples != 0)
            .fold(0, |mask, (bit, _)| mask | bit)
    })
}

/// Whether the strip delay join restarts strip `join` (#1284 D1, #1354a ruling 3): a lane's
/// declared delay changed, or a needed lane ([`needed_lanes`]) can neither move nor grow by
/// exactly `lead` (its ring changed otherwise, the predecessor holds no line for it, or the strip
/// reads another source). `before` and `after` are the strip's lane rings in the predecessor and
/// the successor, `None` for a plan that holds no line for it. [`resolve_strip_delay_carry`] and
/// [`warm_lead`] (on the lead's compile, before bind) read this one rule.
fn strip_delay_restarts(
    join: &StripDelayJoin<'_>,
    before: Option<[u32; 2]>,
    after: Option<[u32; 2]>,
    lead: u64,
) -> bool {
    if join.equal != graph::STRIP_DELAY_LEFT | graph::STRIP_DELAY_RIGHT {
        return true;
    }
    let needed = needed_lanes(join, after);
    match (after, join.same_source) {
        _ if needed == 0 => false,
        (Some(after), true) => {
            let (moved, grown) = strip_delay_lanes(join.equal, before, after, lead);
            moved | grown != needed
        }
        _ => true,
    }
}

/// The line half of the strip delay join (#1284 D1, #1402 D1): a lane carries when its
/// `delay_samples` is the same in both models, the strip reads the same source, and both plans
/// hold the strip's line with rings of one length on that lane. A lane is needed by its ring, not
/// by its declared delay: since #1287 R1 a track's ring is its declared delay plus its source-claim
/// line, so a lane that declares no delay can still hold `P` pending frames. A strip with a needed
/// lane that cannot carry (its delay, its ring length or its source changed) starts that lane at
/// rest and is listed in `restarted` (decision 15, D15-9): the duck-swap removes the click of the
/// alignment that moves. Both slices are sorted by strip ID.
///
/// `lead` is a warm successor's `P` (#1354a ruling 3), 0 for every other successor: a needed lane
/// whose declared delay is equal, whose source is unchanged and whose ring grew by exactly `lead`
/// (from a lane the predecessor did not hold, too) keeps its strip carried and moves nothing. With
/// a lead, every track line on a moved source with a lane that moves or grew is a fill of the
/// prime (#1355 D5, `fills`): the grown lanes take the predecessor's pending words, and every such
/// lane then takes the prime.
fn resolve_strip_delay_carry(
    predecessor: &[graph::GraphStripDelayLine],
    own: &[graph::GraphStripDelayLine],
    joins: &[StripDelayJoin<'_>],
    lead: u64,
) -> StripDelayCarry {
    let row = |rows: &[graph::GraphStripDelayLine], id: &str| {
        rows.binary_search_by(|row| (*row.strip).cmp(id))
            .ok()
            .map(|index| rows[index].clone())
    };
    let mut carry = StripDelayCarry::default();
    for join in joins {
        let after = row(own, join.id);
        let before = row(predecessor, join.id);
        let restarts = strip_delay_restarts(
            join,
            before.as_ref().map(|before| before.samples),
            after.as_ref().map(|after| after.samples),
            lead,
        );
        // A lane whose ring is empty in the successor has nothing to take; a lane of a line the
        // predecessor never had, or of a strip that reads another source, starts at rest.
        let needed = needed_lanes(join, after.as_ref().map(|after| after.samples));
        if let (Some(after), true) = (after.filter(|_| needed != 0), join.same_source) {
            let (moved, grown) = strip_delay_lanes(
                join.equal,
                before.as_ref().map(|before| before.samples),
                after.samples,
                lead,
            );
            if let Some(before) = &before
                && moved != 0
            {
                carry.moves.push(graph::GraphStripDelayMove {
                    successor: after.line,
                    predecessor: before.line,
                    lanes: moved,
                });
            }
            // #1355 D5: a warm successor's track line on a moved source takes the prime on every
            // lane that continues, and the predecessor's pending words first on the lanes that
            // grew.
            if lead != 0 && join.track && moved | grown != 0 {
                carry.fills.push(graph::GraphClaimFill {
                    line: after.line,
                    claim: claim_node(join.id),
                    predecessor: before.as_ref().map(|before| before.line),
                    pending_lanes: grown & !moved,
                    lanes: moved | grown,
                });
            }
        }
        if restarts {
            carry.restarted.push(join.id.into());
        }
    }
    carry
}

/// The `Input`-tap sidechain lines a warm successor fills (#1355 D5, L3): every compensation line
/// of a sidechain edge from the `Input` stage of a track of `restarted` that reads a moved source
/// (its join's `same_source`) into a strip outside `restarted`, with the predecessor's line of
/// the same edge and the track's strip delay line in the predecessor, when it held them.
fn sidechain_fills(
    base: &SuccessorBase<'_>,
    lines: &[graph::GraphDelayLine],
    joins: &[StripDelayJoin<'_>],
    restarted: &BTreeSet<&str>,
) -> Vec<graph::GraphSidechainFill> {
    fn strip_of(node: &graph::GraphNodeId) -> Option<&str> {
        match node {
            graph::GraphNodeId::TrackStage { track_id, .. } => Some(track_id.as_str()),
            graph::GraphNodeId::Effect(effect) => Some(effect.track_id.as_str()),
            _ => None,
        }
    }
    lines
        .iter()
        .filter_map(|line| {
            let graph::GraphEdgeId::EffectSidechain { .. } = &line.edge else {
                return None;
            };
            let graph::GraphNodeId::TrackStage {
                track_id,
                stage: graph::TrackStage::Input,
            } = &line.source
            else {
                return None;
            };
            let track = track_id.as_str();
            let into = strip_of(&line.destination)?;
            let moved = joins
                .iter()
                .any(|join| join.id == track && join.track && join.same_source);
            if !restarted.contains(track) || restarted.contains(into) || !moved {
                return None;
            }
            let predecessor = base
                .inventory
                .delay_lines
                .binary_search_by(|earlier| earlier.edge.cmp(&line.edge))
                .ok()
                .map(|index| &base.inventory.delay_lines[index])
                .filter(|earlier| {
                    earlier.source == line.source && earlier.destination == line.destination
                })
                .map(|earlier| earlier.line);
            let claim_line = base
                .inventory
                .strip_delays
                .binary_search_by(|row| (*row.strip).cmp(track))
                .ok()
                .map(|index| base.inventory.strip_delays[index].line);
            Some(graph::GraphSidechainFill {
                line: line.line,
                claim: line.source.clone(),
                predecessor,
                claim_line,
            })
        })
        .collect()
}

/// Track `id`'s `Input` node, the node its source claim feeds.
fn claim_node(id: &str) -> graph::GraphNodeId {
    graph::GraphNodeId::TrackStage {
        track_id: graph::StableGraphId::parse(id).expect("a strip ID is a stable graph ID"),
        stage: graph::TrackStage::Input,
    }
}

/// One send present in both models whose source and destination are the same and whose values the
/// plans can continue from (#1284 D2, D3): the route with a live lane, and the record that brings
/// the carried send from the predecessor's live values to the successor's, when they differ.
struct RouteJoin<'a> {
    id: &'a str,
    retarget: Option<SendRecord>,
}

/// The model half of the send join (#1284 D2, D3): for every route of `model` with a live lane
/// (`controls::route_rows_live`) that `committed` also has with the same source (strip and tap)
/// and the same destination, the record of the difference of their live values. A re-pointed
/// route is not listed: it is a route removed and a route added, which #1363 ramps at route
/// level, and its source strip is not restarted.
///
/// The difference is judged on the effective values of both models ([`EffectiveDiff`]; the route's
/// gate follows its source strip's effective mute, #1558 D1) by [`route_record_between`], the
/// function the classifier calls for the same send, so a carry cannot disagree with a live commit.
/// A send whose values the function refuses is prepared: it is not listed and starts at its prepared
/// value.
fn join_routes<'a>(
    committed: &'a SessionModel,
    model: &'a SessionModel,
    edit_ramps: &'a EditRamps,
    diff: &EffectiveDiff<'a>,
) -> Vec<RouteJoin<'a>> {
    let ramps = LiveRamps::for_session(model);
    let before_at: BTreeMap<&str, usize> = committed
        .routes
        .iter()
        .enumerate()
        .map(|(index, route)| (route.id.as_str(), index))
        .collect();
    let mut joins = Vec::new();
    for (after_index, after) in model.routes.iter().enumerate() {
        // #1391: every route with a live lane, into a submix or from before its strip's fader.
        if !controls::route_rows_live(after) {
            continue;
        }
        let Some(&before_index) = before_at.get(after.id.as_str()) else {
            continue;
        };
        let before = &committed.routes[before_index];
        if before.source != after.source || before.destination != after.destination {
            continue;
        }
        let gates = diff.route_gates(before_index, after_index);
        let edit = RouteEditRamps::of(edit_ramps, &after.id);
        let gate_length = if gates[0] == gates[1] {
            0
        } else {
            let Some(length) = diff
                .source_strips(before, after)
                .map(|source| diff.gate_length(before, after, edit.mute, source, gates))
            else {
                continue;
            };
            length
        };
        // #1652 D3 (#1652c): a route the successor gives the seven-word cell takes the
        // seven-word record, written only when a word the models hold changes; any other route
        // keeps today's record.
        let retarget = if let Some(automated) = following_route_lanes(diff.automation(), after) {
            let Some(source) = diff.source_strips(before, after) else {
                continue;
            };
            follow_route_record_between(
                ramps,
                before,
                after,
                diff.strip_mutes(source.0, source.1),
                automated,
                gates,
                gate_length,
                edit,
            )
            .map(|record| record.map(SendRecord::Following))
        } else {
            route_record_between(ramps, before, after, gates, gate_length, edit)
                .map(|record| record.map(SendRecord::Folded))
        };
        let Ok(retarget) = retarget else {
            continue;
        };
        joins.push(RouteJoin {
            id: after.id.as_str(),
            retarget,
        });
    }
    joins
}

/// What the send join resolved against both plans' live routes (#1284 D2, D3).
#[derive(Default)]
struct RouteCarry<'a> {
    moves: Vec<graph::GraphRouteMove>,
    /// The carried sends whose live values differ, by route ID, with the record to write to the
    /// successor's cell.
    retargets: Vec<(&'a str, SendRecord)>,
    /// Whether some move carries a following predecessor route into a six-word successor route
    /// (#1652 U2a fold 2, MAJOR-A): the rebuild then completes as the transition fallback
    /// ([`PlanStateInventory::carries_a_following_route_into_a_six_word_route`]).
    follow_into_six_word: bool,
    /// Whether some move carries a following route into a following route (B19-R8,
    /// [`PlanStateInventory::carries_a_following_route_into_a_following_route`]).
    follow_into_following: bool,
    /// Whether some move carries a following route into a following route that no longer hears
    /// a source mute curve of the predecessor's (B19-R7,
    /// [`PlanStateInventory::carries_a_following_route_that_loses_a_curve`]).
    follow_loses_a_curve: bool,
    /// The longest ramp of the following-route records among `retargets` (B19-R7).
    follow_retarget_ramp: u32,
}

/// Whether a following route of automated lanes `before` loses a curve in `after`: a source
/// lane's mute curve (own or a reaching VCA's) that `before` automates and `after` does not
/// (B19-R7).
fn loses_a_curve(before: [bool; 2], after: [bool; 2]) -> bool {
    before.iter().zip(&after).any(|(&had, &has)| had && !has)
}

/// The route half of the send join (#1284 D2): a send carries when both plans hold a live route of
/// its ID. Both slices are sorted by route ID.
fn resolve_route_carry<'a>(
    predecessor: &[graph::GraphLiveRoute],
    own: &[graph::GraphLiveRoute],
    joins: &[RouteJoin<'a>],
) -> RouteCarry<'a> {
    let row = |rows: &[graph::GraphLiveRoute], id: &str| {
        rows.binary_search_by(|row| (*row.route_id).cmp(id))
            .ok()
            .map(|index| {
                (
                    rows[index].unit,
                    rows[index].following,
                    rows[index].automated,
                )
            })
    };
    let mut carry = RouteCarry::default();
    for join in joins {
        let (Some((before, follows, heard)), Some((after, successor_follows, hears))) =
            (row(predecessor, join.id), row(own, join.id))
        else {
            continue;
        };
        // #1652 U2a fold 2 (MAJOR-A; #1713 deletes it): a following route carried into a
        // six-word one holds one ramp, so render may re-time a column's move (a record composed
        // with the heard curve reopens over `J`, not its own length; two columns' moves fold into
        // one ramp over the longer remaining length). The join cannot tell whether a move is in
        // flight, so it flags every such carry: the conservative rule.
        carry.follow_into_six_word |= follows && !successor_follows;
        // B19-R7 (#1718 deletes it): a carried following route that stays seven-word and loses
        // a source mute curve lands that column at once over the removed curve's length, cutting
        // a live send move in flight there. The join flags every such carry;
        // the control plane completes it as the fallback while a send move may still ramp.
        carry.follow_loses_a_curve |= follows && successor_follows && loses_a_curve(heard, hears);
        // B19-R8 (#1718 deletes it): any carried following route that stays seven-word may land
        // a column at once at the swap, cutting a live send move in flight there, whether or not
        // it loses a curve. The control plane completes it as the fallback while a send move may
        // still ramp.
        carry.follow_into_following |= follows && successor_follows;
        carry.moves.push(graph::GraphRouteMove {
            successor: after,
            predecessor: before,
            predecessor_follows: follows,
        });
        if let Some(record) = join.retarget {
            if let SendRecord::Following(record) = record {
                carry.follow_retarget_ramp = carry.follow_retarget_ramp.max(record.length());
            }
            carry.retargets.push((join.id, record));
        }
    }
    carry
}

/// One carried strip's retarget records, to write into the successor's cells (#1277 D5).
struct StripRetarget<'a> {
    id: &'a str,
    fader: [Option<TrackFaderRecord>; 4],
    /// The automated lanes' offsets-cell writes, `[left, right]` (#1648 D2, U3 fold).
    offsets: [Option<FaderOffsetsRecord>; 2],
    matrix: Option<TrackControlRecord>,
}

/// What the strip join resolved against both plans' lanes.
#[derive(Default)]
struct StripCarry<'a> {
    moves: Vec<graph::GraphStripMove>,
    restarted: Vec<Box<str>>,
    retargets: Vec<StripRetarget<'a>>,
}

/// The lane half of the strip join (#1277 D4, D6, D9): which stage of which strip moves, which
/// strips restart, and which carried strips retarget.
///
/// A stage carries when both plans have its lane in a bank of one kind, attach the same control
/// kind, and no prepared value of it differs. A strip with a stage that does not carry starts that
/// stage at rest and is listed in `restarted`.
///
/// `restarts_whole` names the strips a warm successor restarts whole (#1354a decision 2): their
/// fader does not carry.
fn resolve_strip_carry<'a>(
    predecessor: &PlanStateInventory,
    joins: &[StripJoin<'a>],
    own: &[StripStageInventoryRow],
    live: bool,
    restarts_whole: &dyn Fn(&str) -> bool,
) -> StripCarry<'a> {
    use graph::BuiltinStripOwner::{Fader, Matrix};
    let mut carry = StripCarry::default();
    for join in joins {
        let rows = [Fader, Matrix].map(|owner| {
            let own = strip_stage_row(own, join.id, owner)?;
            let before = predecessor.strip_stage(join.id, owner)?;
            (own.stage == before.stage && before.live == live).then_some((own, before))
        });
        let families = [
            !join.fader.is_prepared() && !restarts_whole(join.id),
            !join.matrix.is_prepared(),
        ];
        let carries = [0, 1].map(|owner| rows[owner].is_some() && families[owner]);
        for ((owner, row), carries) in [Fader, Matrix].into_iter().zip(rows).zip(carries) {
            if let (true, Some((own, before))) = (carries, row) {
                carry.moves.push(graph::GraphStripMove {
                    owner,
                    successor: own.location,
                    predecessor: before.location,
                    continued_cells: if owner == Fader {
                        join.continued_cells
                    } else {
                        0
                    },
                });
            }
        }
        if !(carries[0] && carries[1]) {
            carry.restarted.push(join.id.into());
        }
        let LevelRecords { fader, offsets } = match join.fader {
            FamilyJoin::Live(records) if carries[0] => records,
            _ => LevelRecords::default(),
        };
        let matrix = match join.matrix {
            FamilyJoin::Live(record) if carries[1] => Some(record),
            _ => None,
        };
        if fader[0].is_some() || offsets != [None; 2] || matrix.is_some() {
            carry.retargets.push(StripRetarget {
                id: join.id,
                fader,
                offsets,
                matrix,
            });
        }
    }
    carry
}

/// Effect `effect_id` in `rack` of strip `id` in `rows`, which are sorted by strip ID, rack, then
/// effect ID.
fn effect_lane_row<'a>(
    rows: &'a [EffectLaneInventoryRow],
    id: &str,
    rack: graph::RackId,
    effect_id: &str,
) -> Option<&'a EffectLaneInventoryRow> {
    rows.binary_search_by(|row| {
        (*row.id)
            .cmp(id)
            .then(row.rack.cmp(&rack))
            .then_with(|| (*row.effect_id).cmp(effect_id))
    })
    .ok()
    .map(|index| &rows[index])
}

/// One console slot of a strip present in both models, and how the successor's model differs from
/// the predecessor's base in everything the slot's lane holds (#1279 D1, #1280 D1).
struct ConsoleOwnerJoin<'a> {
    /// `Simd1` for a `pre_insert` slot, `Simd2` for a `post_insert` one.
    rack: graph::RackId,
    /// The slot ID, which is the effect ID of the owner key.
    slot: &'a str,
    /// The slot's live address in the strip.
    address: LiveEffectAddress,
    /// [`FamilyJoin::Same`] when the slot's declaration (identity, quality and link mode) and the
    /// strip's bypass and parameter values for it are equal in both models, and the slot sits in
    /// the same section; [`FamilyJoin::Live`] when only live values differ, as
    /// [`effect_instance_records`] decides; [`FamilyJoin::Prepared`] otherwise.
    state: FamilyJoin<EffectInstanceRecords>,
}

/// One strip present in both models, with how its console slots and effect chain differ.
struct ChainJoin<'a> {
    id: &'a str,
    /// An insert was added, removed or reordered, or a session console slot was added, removed or
    /// moved: the chain's processing changed, so the strip restarts even when every remaining
    /// owner carries (D15-9).
    chain_changed: bool,
    /// The strip's console slots in the successor's model.
    owners: Vec<ConsoleOwnerJoin<'a>>,
    /// The inserts the strip has in both models (the surviving inserts), in the successor's chain
    /// order. An insert's owner key is the strip, [`graph::RackId::Dynamic`] and its ID.
    surviving_inserts: Vec<InsertOwnerJoin<'a>>,
}

/// One insert of a strip present in both models, and how the successor's model differs from the
/// predecessor's base (#1280).
struct InsertOwnerJoin<'a> {
    /// The insert ID, which is the effect ID of the owner key.
    id: &'a str,
    /// The insert's live address in the strip.
    address: LiveEffectAddress,
    /// [`FamilyJoin::Same`] when its identity, quality, bypass, link mode, sidechain and every
    /// parameter value are equal in both models and it sits at the same position;
    /// [`FamilyJoin::Live`] when only live values differ; [`FamilyJoin::Prepared`] otherwise.
    state: FamilyJoin<EffectInstanceRecords>,
}

impl ChainJoin<'_> {
    /// Whether any console slot or surviving insert of the strip could carry.
    fn may_carry(&self) -> bool {
        self.owners.iter().any(|owner| !owner.state.is_prepared())
            || self
                .surviving_inserts
                .iter()
                .any(|insert| !insert.state.is_prepared())
    }
}

/// The session console slots in chain order, each with its rack.
fn console_slots(model: &SessionModel) -> Vec<(graph::RackId, &session::ConsoleSlot)> {
    model
        .console
        .pre_insert
        .iter()
        .map(|slot| (graph::RackId::Simd1, slot))
        .chain(
            model
                .console
                .post_insert
                .iter()
                .map(|slot| (graph::RackId::Simd2, slot)),
        )
        .collect()
}

/// The strip `strip` was in `committed`, among strips of its own kind (#1279 D1). A strip whose
/// kind changed is not found.
fn committed_strip_ref<'a>(
    committed: &'a SessionModel,
    strip: &StripRef<'_>,
) -> Option<StripRef<'a>> {
    let id = strip.id.as_str();
    match strip.kind {
        StripKind::Track(_) => committed
            .tracks
            .binary_search_by(|track| track.id.as_str().cmp(id))
            .ok()
            .map(|index| StripRef::track(&committed.tracks[index])),
        StripKind::Submix(_) => committed
            .submixes
            .binary_search_by(|submix| submix.id.as_str().cmp(id))
            .ok()
            .map(|index| StripRef::submix(&committed.submixes[index])),
    }
}

/// How one instance differs between the two models (#1280 D1): the per-instance function's answer
/// as a join state.
fn instance_join(
    before: Option<&session::Effect>,
    after: Option<&session::Effect>,
    sample_rate_hz: u32,
    ramps: LiveRamps,
    link_edit: Option<u32>,
) -> FamilyJoin<EffectInstanceRecords> {
    let (Some(before), Some(after)) = (before, after) else {
        return FamilyJoin::Prepared;
    };
    match effect_instance_records(before, after, sample_rate_hz, ramps, link_edit) {
        Err(_) => FamilyJoin::Prepared,
        Ok(None) => FamilyJoin::Same,
        Ok(Some(records)) => FamilyJoin::Live(records),
    }
}

/// The model half of the console-effect join (#1279 D1, #1280 D1): for every strip of `model` that
/// `committed` also has, whether its effect chain changed and, for each console slot and each
/// surviving insert, how its declaration and values differ ([`effect_instance_records`]).
///
/// An insert added, removed or reordered changes the chain, and an insert present in both models
/// is recorded as a surviving insert, an owner whose lane the lane half resolves. A strip whose
/// kind changed has no counterpart: its chain is changed and no slot carries.
///
/// `edit_ramps` gives a live link difference the length the transaction gave the strip's link row
/// (#1371 D3).
fn join_chains<'a>(
    committed: &SessionModel,
    model: &'a SessionModel,
    edit_ramps: &EditRamps,
) -> Vec<ChainJoin<'a>> {
    let committed_slots = console_slots(committed);
    let slots = console_slots(model);
    let console_changed = committed_slots.len() != slots.len()
        || committed_slots
            .iter()
            .zip(&slots)
            .any(|((before_rack, before), (rack, slot))| {
                before_rack != rack || before.slot != slot.slot
            });
    let ramps = LiveRamps::for_session(model);
    let sample_rate_hz = model.sample_rate_hz;
    let mut joins = Vec::new();
    for strip in model.strips() {
        let id = strip.id.as_str();
        let link_edit = StripEditRamps::of(edit_ramps, strip.id).link;
        let Some(before) = committed_strip_ref(committed, &strip) else {
            if committed_has_strip(committed, id) {
                joins.push(ChainJoin {
                    id,
                    chain_changed: true,
                    owners: Vec::new(),
                    surviving_inserts: Vec::new(),
                });
            }
            continue;
        };
        let inserts = &strip.inserts.effects;
        let before_inserts = &before.inserts.effects;
        let inserts_changed = before_inserts.len() != inserts.len()
            || before_inserts.iter().zip(inserts).any(|(before, insert)| {
                before.id != insert.id || before.identity != insert.identity
            });
        // The strip's console slots as instances, in slot order (`pre_insert`, then `post_insert`):
        // the lowering the live path reads (`LoweredRacks`).
        let racks_before = committed.lower_strip(&before);
        let racks_after = model.lower_strip(&strip);
        let lowered_before: Vec<&session::Effect> = racks_before
            .pre_insert
            .iter()
            .chain(&racks_before.post_insert)
            .collect();
        let lowered_after: Vec<&session::Effect> = racks_after
            .pre_insert
            .iter()
            .chain(&racks_after.post_insert)
            .collect();
        let owners = slots
            .iter()
            .enumerate()
            .map(|(position, (rack, slot))| {
                let state = committed_slots
                    .iter()
                    .position(|(_, before)| before.slot == slot.slot)
                    // A slot whose section or declaration differs is prepared.
                    .filter(|&index| {
                        let (before_rack, before_slot) = committed_slots[index];
                        before_rack == *rack && before_slot == *slot
                    })
                    .map_or(FamilyJoin::Prepared, |index| {
                        let state = instance_join(
                            lowered_before.get(index).copied(),
                            lowered_after.get(position).copied(),
                            sample_rate_hz,
                            ramps,
                            link_edit,
                        );
                        // A slot at another position has another live address: its live
                        // difference is prepared (the chain changed, so the strip restarts).
                        match state {
                            FamilyJoin::Live(_) if index != position => FamilyJoin::Prepared,
                            state => state,
                        }
                    });
                ConsoleOwnerJoin {
                    rack: *rack,
                    slot: slot.slot.as_str(),
                    address: LiveEffectAddress::console(position as u32),
                    state,
                }
            })
            .collect();
        joins.push(ChainJoin {
            id,
            chain_changed: console_changed || inserts_changed,
            owners,
            surviving_inserts: inserts
                .iter()
                .enumerate()
                .filter_map(|(position, insert)| {
                    let index = before_inserts
                        .iter()
                        .position(|before| before.id == insert.id)?;
                    let state = instance_join(
                        Some(&before_inserts[index]),
                        Some(insert),
                        sample_rate_hz,
                        ramps,
                        link_edit,
                    );
                    // An insert at another position has another live address: its live
                    // difference is prepared.
                    let state = match state {
                        FamilyJoin::Live(_) if index != position => FamilyJoin::Prepared,
                        state => state,
                    };
                    Some(InsertOwnerJoin {
                        id: insert.id.as_str(),
                        address: LiveEffectAddress::insert(position as u32),
                        state,
                    })
                })
                .collect(),
        });
    }
    joins
}

/// One carried lane's retarget, to write into the successor's cells (#1280 D4): the records of the
/// instance's live difference, preflighted against the successor's producer.
struct EffectRetarget {
    /// The instance's position in the successor's sorted effect producers.
    producer: usize,
    /// The bypass record (it carries its crossfade ramp, #1341 D1, #1622) first, then the parameter
    /// records, all written as they are; the parameter records are empty for an instance with
    /// `targets`.
    records: Vec<EffectControlRecord>,
    /// For an instance that rides prepared targets: the owner's committed revision its open
    /// candidate began at, and the targets to publish.
    targets: Option<(u64, Vec<PreparedEffectTarget>)>,
}

/// What the console-effect join resolved against both plans' lanes.
#[derive(Default)]
struct EffectCarry {
    moves: Vec<graph::GraphEffectMove>,
    restarted: Vec<Box<str>>,
    /// The largest payload of a carried lane: the scratch every move goes through.
    scratch_bytes: u64,
    /// The carried lanes' live differences (#1280 D4).
    retargets: Vec<EffectRetarget>,
}

/// Opens the successor producer's side of `instance`'s live difference: every record preflighted
/// (a record the lane has no cell for refuses the whole instance), an EQ's owner transaction begun,
/// edited and preflighted with its targets. `None` leaves the producer untouched, and the lane
/// restarts.
fn prepare_effect_retarget(
    producer: &mut EffectControlProducer,
    position: usize,
    instance: &EffectInstanceRecords,
) -> Option<EffectRetarget> {
    let mut records = Vec::new();
    for &record in &instance.records {
        match record {
            EffectControlRecord::Bypass(_) | EffectControlRecord::Link(_) => {
                producer.preflight(record).ok()?;
                records.push(record);
            }
            // An instance with targets takes its parameters as the owner's edits.
            EffectControlRecord::Parameter { .. } if instance.targets.is_some() => {}
            EffectControlRecord::Parameter { .. } => {
                producer.preflight(record).ok()?;
                records.push(record);
            }
            _ => return None,
        }
    }
    let targets = match &instance.targets {
        None => None,
        Some(targets) => {
            let base_revision = producer.owner()?.committed_revision();
            producer.begin_owner(base_revision).ok()?;
            let opened = instance
                .records
                .iter()
                .try_for_each(|&record| match record {
                    EffectControlRecord::Parameter {
                        parameter_index,
                        channel,
                        value,
                    } => producer.edit_owner(parameter_index, channel, value),
                    _ => Ok(()),
                })
                .and_then(|()| producer.preflight_candidate_targets(base_revision, targets));
            if opened.is_err() {
                let _ = producer.discard_owner();
                return None;
            }
            Some((base_revision, targets.clone()))
        }
    };
    Some(EffectRetarget {
        producer: position,
        records,
        targets,
    })
}

/// The lane half of the effect join (#1279 D1, D2; #1280): which console slot or insert of which
/// strip moves, and which strips restart.
///
/// An owner carries when its difference is not a prepared one, and both plans hold it as a lane
/// of an effect bank stage or as a live-controlled per-node instance, of one kind (a plain stage,
/// or a live-control stage whose lane has cells, only a lowered bypass or nothing), with equal
/// layouts (prepared bypass, latency, state sizes and state layout version). An owner that one
/// plan holds with live controls and the other without has no row on one side, and restarts. A live difference also needs `retarget` to accept it (the
/// successor's producer has a cell for every record); a lane it refuses restarts. A console slot's
/// key is (strip, `Simd1` or `Simd2`, slot ID), a surviving insert's is (strip,
/// [`graph::RackId::Dynamic`], insert ID). A strip with an owner that does not carry, or whose
/// chain changed, is listed in `restarted`.
fn resolve_effect_carry(
    predecessor: &PlanStateInventory,
    joins: &[ChainJoin<'_>],
    own: &[EffectLaneInventoryRow],
    retarget: &mut dyn FnMut(
        &str,
        LiveEffectAddress,
        &EffectInstanceRecords,
    ) -> Option<EffectRetarget>,
) -> EffectCarry {
    let mut carry = EffectCarry::default();
    for join in joins {
        let mut restart = join.chain_changed;
        let owners = join
            .owners
            .iter()
            .map(|owner| (owner.rack, owner.slot, owner.address, &owner.state))
            .chain(join.surviving_inserts.iter().map(|insert| {
                (
                    graph::RackId::Dynamic,
                    insert.id,
                    insert.address,
                    &insert.state,
                )
            }));
        for (rack, effect_id, address, state) in owners {
            let rows = effect_lane_row(own, join.id, rack, effect_id)
                .zip(predecessor.effect_lane(join.id, rack, effect_id));
            let Some((own, before)) = rows.filter(|(own, before)| {
                !state.is_prepared() && own.layout == before.layout && own.kind == before.kind
            }) else {
                restart = true;
                continue;
            };
            let retargeted = match state {
                FamilyJoin::Live(instance) => {
                    let Some(retargeted) = retarget(join.id, address, instance) else {
                        restart = true;
                        continue;
                    };
                    Some(retargeted)
                }
                _ => None,
            };
            // Two per-node instances change places by pointer (#1282 D2): their state goes through
            // no scratch, so they add nothing to it.
            if !matches!(
                (own.location, before.location),
                (
                    graph::GraphEffectSite::Node(_),
                    graph::GraphEffectSite::Node(_)
                )
            ) {
                carry.scratch_bytes = carry.scratch_bytes.max(own.layout.payload_bytes());
            }
            carry.moves.push(graph::GraphEffectMove {
                successor: own.location,
                predecessor: before.location,
            });
            carry.retargets.extend(retargeted);
        }
        if restart {
            carry.restarted.push(join.id.into());
        }
    }
    carry
}

/// Whether `model` is inside the count caps `caps` states: tracks, submixes, VCAs, sources, routes
/// and effects (`host.resource.count`).
///
/// The one count check (decision 17, #1558 D6). Preparation runs it on every plan, and a host that
/// commits a live delta runs it on the prospective model, so a VCA added live is refused as a
/// rebuild would refuse it and a committed model never exceeds what its own rebuild accepts.
///
/// # Errors
///
/// `host.resource.count` when a count exceeds its cap, or `host.count` when one does not fit a
/// `u64`.
pub fn check_counts(
    model: &SessionModel,
    caps: &HostPrepareCaps,
) -> Result<(), PrepareDiagnostics> {
    let count = |len: usize| u64::try_from(len).map_err(|_| platform("host.count"));
    let track_count = count(model.tracks.len())?;
    let submix_count = count(model.submixes.len())?;
    let vca_count = count(model.vcas.len())?;
    let source_count = count(model.sources.len())?;
    let route_count = count(model.routes.len())?;
    let effect_count = count_effects(model)?;
    if track_count > caps.maximum_tracks
        || submix_count > caps.maximum_submixes
        || vca_count > caps.maximum_vcas
        || source_count > caps.maximum_sources
        || route_count > caps.maximum_routes
        || effect_count > caps.maximum_effects
    {
        return Err(resource("host.resource.count"));
    }
    // The pairs are counted only for a session that declares VCAs, and the count stops at the cap.
    if vca_count != 0
        && model.vca_reach_pairs(caps.maximum_vca_reach_pairs) > caps.maximum_vca_reach_pairs
    {
        return Err(resource("host.resource.vca_reach_pairs"));
    }
    Ok(())
}

/// The session's native effect instances, projected without preparing any (issue #2194):
/// [`effect_compiler::project_session_effects`] against the launch registry.
///
/// # Errors
///
/// `host.effect.registry` when the launch registry failed to build, and
/// `host.effect.resource.arithmetic` when a sum overflows.
pub fn project_effects(
    compiled: &CompiledSession,
) -> Result<EffectStateProjection, PrepareDiagnostics> {
    let registry =
        launch_native_effect_registry().map_err(|_| effect_failure("host.effect.registry"))?;
    effect_compiler::project_session_effects(compiled, registry)
        .ok_or_else(|| resource("host.effect.resource.arithmetic"))
}

/// Refuse a projection whose declared effect state or scratch exceeds its cap (issue #2194):
/// `host.effect.resource.limit`, naming the projected bytes, the cap and the instances, at
/// `$.maximum_effect_state_bytes` or `$.maximum_effect_scratch_bytes`. Preparation runs it before
/// it allocates, and a control plane may run it on a candidate before it prepares one.
///
/// # Errors
///
/// `host.effect.resource.limit` as above.
pub fn check_effect_projection(
    projection: EffectStateProjection,
    caps: &HostPrepareCaps,
) -> Result<(), PrepareDiagnostics> {
    for (field, projected, limit) in [
        (
            "maximum_effect_state_bytes",
            projection.state_bytes,
            caps.maximum_effect_state_bytes,
        ),
        (
            "maximum_effect_scratch_bytes",
            projection.scratch_bytes,
            caps.maximum_effect_scratch_bytes,
        ),
    ] {
        if projected > limit {
            return Err(PrepareDiagnostics::new(
                PrepareRejection::Resource,
                format!(
                    "host.effect.resource.limit\t$.{field}[projected_bytes={projected},limit_bytes={limit},instances={}]\n",
                    projection.instances
                )
                .into_bytes(),
            ));
        }
    }
    Ok(())
}

/// What a preparation of a session retains, projected before it allocates any effect processor,
/// effect bank, plan, source ring or live-control lane (issue #2199).
///
/// [`project_host_session`] returns it, and every preparation computes it at the same point and
/// is admitted on it ([`Self::admitted_graph_and_model`] against
/// `maximum_graph_session_plus_plan_bytes`) before it prepares a processor. The rows marked exact
/// are the prepared report's own, and preparation refuses a plan whose report differs from them
/// (`host.graph.projection`); the rows marked as a ceiling bound the report's from above
/// (`host.control.projection` when one does not), at their count bound for what only a successor
/// holds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostProjection {
    /// The effect state, scratch and processor bytes (issue #2194), from the descriptors.
    pub effects: EffectStateProjection,
    /// The report's `graph_session_plus_plan_bytes`, exact: the graph's capped estimate, which
    /// holds the declared effect state and scratch and the builtins' banks and owners, and the
    /// plan's fixed cells.
    pub graph_session_plus_plan_bytes: u64,
    /// The declared effect state and scratch inside [`Self::graph_session_plus_plan_bytes`].
    pub graph_declared_effect_bytes: u64,
    /// The report's `builtin_retained_payload_bytes`, exact.
    pub builtin_retained_payload_bytes: u64,
    /// The report's `session_model_bytes`, exact: the model the graph is compiled from.
    pub session_model_bytes: u64,
    /// The report's `spectrum_capture_retained_bytes`, exact.
    pub spectrum_capture_retained_bytes: u64,
    /// The report's `effect_control_resources`, exact.
    pub effect_control_resources: EffectControlResources,
    /// A ceiling of the report's `route_control_resources` (every byte row, and its largest
    /// allocation).
    pub route_controls: graph::RouteControlResources,
    /// A ceiling of the report's `carry_program_retained_bytes`: the carry program's exact bytes
    /// and, for a successor, every later carry section at its count bound (root ruling Q2).
    pub carry_bytes: u64,
    /// The carry program's own bytes, exact: the part of [`Self::carry_bytes`] known before the
    /// plan is bound. The admissions before preparation charge it; the later sections are
    /// resolved against the bound plan's lanes and charged exactly by the admission after bind.
    pub carry_program_bytes: u64,
    /// A ceiling of the report's `inventory_retained_bytes`.
    pub inventory_bytes: u64,
    /// The report's `control_retained_bytes`, exact: the source control table and ID arena.
    pub control_retained_bytes: u64,
    /// The report's `source_total_bytes`, exact.
    pub source_total_bytes: u64,
    /// The report's `source_overhead_bytes`, exact.
    pub source_overhead_bytes: u64,
    /// The report's `carried_source_total_bytes`, exact: the rings and timeline a successor
    /// carries from its predecessor (issue #1272 D4), outside the three source rows.
    pub carried_source_total_bytes: u64,
    /// The report's `carried_source_overhead_bytes`, exact.
    pub carried_source_overhead_bytes: u64,
    /// The heap the strip producers own beyond their table, exact: what the control plane charges
    /// its strip producer table from ([`crate::strip_control_table_bytes`]).
    pub strip_heap_bytes: u64,
    /// The plan's strips (tracks, then submixes) and their ID text, exact.
    pub strips: usize,
    /// The ID text of the plan's strips, exact.
    pub strip_id_bytes: u64,
    /// The plan's meter requests, exact.
    pub meters: usize,
    /// The plan's effect instances, exact.
    pub effect_instances: usize,
    /// The effect producers the plan attaches (one per instance with a lane), exact.
    pub effect_producers: usize,
    /// Their strip and effect ID text, exact.
    pub effect_producer_id_bytes: u64,
    /// The effect observation handles the plan's readers are, exact.
    pub observation_handles: usize,
    /// The handles' heap, exact: the handle table, each handle's ID text and its readers.
    pub observation_handle_bytes: u64,
    /// The readers of every handle (one per declared tap), exact.
    pub observation_taps: usize,
    /// The observation slots of the effect producers' queues, exact.
    pub observation_slots: usize,
    /// The provider catalog's resources, exact ([`crate::ControlCatalog::project`]).
    #[cfg(feature = "control-provider")]
    pub catalog: crate::CatalogResources,
}

impl HostProjection {
    /// What preparation admits against `maximum_graph_session_plus_plan_bytes` before it prepares
    /// anything: the exact rows of the admission after bind, which are the graph row, the
    /// compiled model, the spectrum capture, the effect producers' table and payload and the carry
    /// program. The ceiling rows (the send lanes, and a successor's later carry sections) are not
    /// in it: the admission after bind charges them exactly, so this admission never refuses a
    /// session that the exact one admits. `None` on overflow.
    #[must_use]
    pub fn admitted_graph_and_model(&self) -> Option<u64> {
        self.graph_session_plus_plan_bytes
            .checked_add(self.session_model_bytes)?
            .checked_add(self.spectrum_capture_retained_bytes)?
            .checked_add(self.effect_control_resources.total_bytes()?)?
            .checked_add(self.carry_program_bytes)
    }

    /// Every engine-owned byte the prepared host retains, at most: the admitted rows, the send
    /// lanes and a successor's later carry sections at their bound, the effect processors' boxes
    /// and undeclared heap beyond the graph's declared state, the state inventory, the source
    /// control set and the sources. `None` on overflow.
    #[must_use]
    pub fn retained_bytes(&self) -> Option<u64> {
        self.admitted_graph_and_model()?
            .checked_add(self.route_controls.total_bytes)?
            .checked_add(self.carry_bytes.checked_sub(self.carry_program_bytes)?)?
            .checked_add(self.effects.inline_bytes)?
            .checked_add(self.effects.undeclared_heap_bytes)?
            .checked_add(self.inventory_bytes)?
            .checked_add(self.control_retained_bytes)?
            .checked_add(self.source_total_bytes)
    }
}

/// What a projection takes (issue #2199): the inputs of the preparation it projects.
#[derive(Clone, Copy)]
pub struct HostProjectionRequest<'a> {
    /// The live-control request, or `None` for the plain entry, as for preparation.
    pub live_controls: Option<&'a HostLiveControlRequest>,
    /// The caller's meters, or `None` for the core's selection.
    pub selected_meters: Option<&'a [HostMeterRequest]>,
    /// The bank dispatch.
    pub backend: Backend,
    /// The spectrum preparation.
    pub spectrum: Option<&'a SpectrumPreparation>,
    /// The base the plan would succeed, or `None` for a first plan.
    pub successor: Option<SuccessorBase<'a>>,
    /// The live lanes the plan would attach.
    pub lanes: HostLiveLanes,
    /// The session's `live_values_superseded` count, or `None` for the preparation's own.
    pub superseded: Option<&'a LiveValuesSuperseded>,
}

/// Project what preparing `compiled` under `caps` and `request` retains, before allocating any
/// effect processor, effect bank, plan, source ring or live-control lane (issue #2199): the
/// preparation itself, stopped at its projection. It prepares the builtins and builds the graph's
/// front, as preparation does, and drops them.
///
/// # Errors
///
/// Every refusal preparation makes up to its projection, its admission included
/// (`host.graph.resource.limit`).
pub fn project_host_session(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    request: HostProjectionRequest<'_>,
) -> Result<HostProjection, PrepareDiagnostics> {
    match prepare_or_project(
        compiled,
        caps,
        request.live_controls,
        request.selected_meters,
        request.backend,
        request.spectrum,
        request.successor,
        request.lanes,
        request.superseded,
        PrepareMode::Project,
    )? {
        PrepareOutcome::Projected(projection) => Ok(*projection),
        PrepareOutcome::Prepared(_) => Err(platform("host.graph.projection")),
    }
}

/// A ceiling of what a successor's carry sections and fade arms retain (issue #2199, root ruling
/// Q2): `carry_program_bytes` (its carry program and input moves, known before the compile) and
/// every later section at its count bound, as though every owner carried: a strip move per strip
/// stage, an effect move per instance with the largest declared state as scratch and every
/// planned lane's carry slots, an observer move per meter and spectrum entry and a join per meter,
/// a tap move per instance, a line move and a prime fill per compensation line, a strip-delay
/// move and fill per strip delay, a send move per route, and a fade arm per strip, route and copy.
fn successor_carry_bound(
    model: &SessionModel,
    projection: &graph_compiler::GraphProjection,
    carry_program_bytes: u64,
    meters: usize,
    spectrum: bool,
    route_copies: &[RouteCopy],
) -> Option<u64> {
    let strips = model.strips().count();
    let entries = &projection.effects().entries;
    let scratch = entries
        .iter()
        .map(|entry| entry.metadata.state_sizes.total())
        .try_fold(0_u64, |largest, total| Some(largest.max(total?)))?;
    let slots = entries
        .iter()
        .filter_map(|entry| entry.lane_layout())
        .try_fold(0_u64, |total, lane| {
            total.checked_add(lane.carry_slot_bytes)
        })?;
    let lines = projection.inserted_delays().len();
    let strip_delays = projection.track_delays().len();
    let routes = model.routes.len();
    let arms = strips + routes + route_copies.len();
    let spectrum = usize::from(spectrum);
    [
        Some(carry_program_bytes),
        u64::try_from(
            strips
                .checked_mul(2)?
                .checked_mul(size_of::<graph::GraphStripMove>())?,
        )
        .ok(),
        graph::effect_carry_retained_bytes(entries.len(), scratch, slots),
        graph::observer_carry_retained_bytes(meters + spectrum, meters, entries.len()),
        graph::delay_carry_retained_bytes(lines),
        graph::strip_delay_carry_retained_bytes(strip_delays),
        graph::prime_carry_retained_bytes(strip_delays, lines),
        graph::route_carry_retained_bytes(routes, routes),
        graph::fade_arm_table_bytes(arms, arms),
    ]
    .into_iter()
    .try_fold(0_u64, |total, row| total.checked_add(row?))
}

/// Bytes of ID text a graph edge ID owns on the heap.
fn graph_edge_heap_bytes(id: &graph::GraphEdgeId) -> usize {
    match id {
        graph::GraphEdgeId::TrackMain { target } => graph_node_heap_bytes(target),
        graph::GraphEdgeId::RouteSource { route_id }
        | graph::GraphEdgeId::RouteDestination { route_id } => route_id.as_str().len(),
        graph::GraphEdgeId::EffectSidechain { effect, port } => {
            effect.track_id.as_str().len() + effect.effect_id.as_str().len() + port.len()
        }
    }
}

/// A ceiling of the [`PlanStateInventory::retained_bytes`] of a plan of `model` whose graph is
/// `projection` (issue #2199): every row at its count bound, sized as the inventory charges it.
/// `meters`, `observations` and `spectrum` are the plan's meter requests, observation handles and
/// spectrum entries; a successor's rows (restarts, fade arms and their watch, pairings) are
/// charged at their bound, and `route_copies` are its fading copies.
fn projected_inventory_bytes(
    model: &SessionModel,
    projection: &graph_compiler::GraphProjection,
    meters: usize,
    observations: usize,
    spectrum: usize,
    successor: bool,
    route_copies: &[RouteCopy],
) -> Option<u64> {
    let strips = model.strips().count();
    let strip_ids: usize = model.strips().map(|strip| strip.id.as_str().len()).sum();
    let longest_strip_id = model
        .strips()
        .map(|strip| strip.id.as_str().len())
        .max()
        .unwrap_or(0);
    let source_ids: usize = model
        .sources
        .iter()
        .map(|source| source.id.as_str().len())
        .sum();
    let route_ids: usize = model
        .routes
        .iter()
        .map(|route| route.id.as_str().len())
        .sum();
    let (effects, effect_ids) = model
        .strips()
        .fold((0_usize, 0_usize), |(count, ids), strip| {
            let lowered = model.lower_strip(&strip);
            lowered
                .in_chain_order()
                .iter()
                .flat_map(|rack| rack.iter())
                .fold((count, ids), |(count, ids), effect| {
                    (
                        count + 1,
                        ids + strip.id.as_str().len() + effect.id.as_str().len(),
                    )
                })
        });
    let longest_node = projection
        .nodes()
        .iter()
        .map(|node| graph_node_heap_bytes(&node.id))
        .max()
        .unwrap_or(0);
    let delay_ids: usize = projection
        .inserted_delays()
        .iter()
        .map(|delay| graph_edge_heap_bytes(&delay.edge_id) + 2 * longest_node)
        .sum();
    let arrivals: usize = projection
        .input_arrivals()
        .iter()
        .chain(projection.raised_arrivals())
        .map(|(node, _)| graph_node_heap_bytes(node))
        .sum();
    let copies = route_copies.len();
    let copy_ids: usize = route_copies
        .iter()
        .map(|copy| {
            let source = match &copy.route.source {
                session::RouteSource::Track { track_id, .. } => track_id.as_str().len(),
                session::RouteSource::Submix { submix_id, .. } => submix_id.as_str().len(),
            };
            let destination = match &copy.route.destination {
                session::RouteDestination::SubmixInput { submix_id } => submix_id.as_str().len(),
                session::RouteDestination::OutputInput { output_id } => output_id.as_str().len(),
            };
            copy.route.id.as_str().len() + source + destination
        })
        .sum();
    // A successor's own rows: each strip restarted, a fade arm per strip, route and copy, the
    // watch's flags, and the meter and spectrum pairings.
    let (restarted, arms, pairings_meters, pairings_spectrum, restarted_ids, arm_ids) = if successor
    {
        (
            strips,
            strips + model.routes.len() + copies,
            meters,
            spectrum,
            strip_ids,
            strip_ids + route_ids,
        )
    } else {
        (0, copies, 0, 0, 0, 0)
    };
    let watch = if arms == 0 {
        0
    } else {
        arms * core::mem::size_of::<core::sync::atomic::AtomicU8>() + 2 * size_of::<usize>()
    };
    let rows = [
        model
            .sources
            .len()
            .checked_mul(size_of::<SourceInventoryRow>())?,
        strips.checked_mul(size_of::<InputSectionInventoryRow>())?,
        strips.checked_mul(2 * size_of::<StripStageInventoryRow>())?,
        effects.checked_mul(size_of::<EffectLaneInventoryRow>())?,
        meters.checked_mul(size_of::<MeterInventoryRow>())?,
        observations.checked_mul(size_of::<ObservationInventoryRow>())?,
        spectrum.checked_mul(size_of::<SpectrumInventoryRow>())?,
        pairings_spectrum.checked_mul(size_of::<crate::spectrum::SpectrumPair>())?,
        pairings_meters.checked_mul(size_of::<engine::realtime::ObserverCarry>())?,
        projection
            .inserted_delays()
            .len()
            .checked_mul(size_of::<graph::GraphDelayLine>())?,
        projection
            .track_delays()
            .len()
            .checked_mul(size_of::<graph::GraphStripDelayLine>())?,
        model
            .routes
            .len()
            .checked_mul(size_of::<graph::GraphLiveRoute>())?,
        restarted.checked_mul(size_of::<Box<str>>())?,
        arms.checked_mul(size_of::<FadeArmInventoryRow>())?,
        copies.checked_mul(size_of::<RouteCopyRow>())?,
        projection
            .input_arrivals()
            .len()
            .checked_add(projection.raised_arrivals().len())?
            .checked_mul(size_of::<(graph::GraphNodeId, u64)>())?,
        PublishedGeneration::retained_bytes(),
        // ID text: the sources', each strip's input and its two stages', each effect lane's, a
        // meter's and a spectrum row's strip, the lines', the strip delays', every route's, a
        // successor's restarts and arms, the copies' and the arrivals' nodes.
        source_ids,
        strip_ids.checked_mul(3)?,
        effect_ids,
        meters
            .checked_add(spectrum)?
            .checked_mul(longest_strip_id)?,
        delay_ids,
        projection
            .track_delays()
            .len()
            .checked_mul(longest_strip_id)?,
        route_ids,
        restarted_ids,
        arm_ids,
        copy_ids,
        arrivals,
        watch,
    ];
    rows.into_iter()
        .try_fold(0_usize, usize::checked_add)
        .and_then(|bytes| u64::try_from(bytes).ok())
}

/// Total effect instances across every strip: one per console entry, since each lowers to one
/// effect instance on its strip (decision 12), plus the strip's inserts.
pub fn count_effects(model: &SessionModel) -> Result<u64, PrepareDiagnostics> {
    model.strips().try_fold(0_u64, |total, strip| {
        let count = strip
            .console
            .len()
            .checked_add(strip.inserts.effects.len())
            .ok_or_else(|| resource("host.resource.arithmetic"))?;
        total
            .checked_add(u64::try_from(count).map_err(|_| platform("host.count"))?)
            .ok_or_else(|| resource("host.resource.arithmetic"))
    })
}

fn shape(code: &str) -> PrepareDiagnostics {
    PrepareDiagnostics::fixed(PrepareRejection::Shape, code)
}

fn resource(code: &str) -> PrepareDiagnostics {
    PrepareDiagnostics::fixed(PrepareRejection::Resource, code)
}

fn platform(code: &str) -> PrepareDiagnostics {
    PrepareDiagnostics::fixed(PrepareRejection::Platform, code)
}

fn graph_failure(code: &str) -> PrepareDiagnostics {
    PrepareDiagnostics::fixed(PrepareRejection::Graph, code)
}

fn effect_failure(code: &str) -> PrepareDiagnostics {
    PrepareDiagnostics::fixed(PrepareRejection::Effect, code)
}

fn spectrum_diagnostics(error: SpectrumPrepareError) -> PrepareDiagnostics {
    match error {
        SpectrumPrepareError::UnknownTarget => shape("host.spectrum.target"),
        SpectrumPrepareError::CaptureBudget => resource("host.spectrum.capture_budget"),
        SpectrumPrepareError::AllocationBudget => resource("host.spectrum.allocation_budget"),
        SpectrumPrepareError::QueueCapacity => resource("host.spectrum.queue"),
        SpectrumPrepareError::NoChannels => shape("host.spectrum.channels"),
        SpectrumPrepareError::DuplicateEntry => shape("host.spectrum.duplicate"),
        SpectrumPrepareError::CollectionCapacity => resource("host.spectrum.collection"),
    }
}

#[cfg(test)]
mod tests {
    use super::{effect_diagnostics, session_diagnostics};
    use effect_compiler::{EffectDiagnostic, EffectDiagnosticSet};
    use session::{DiagnosticCode, parse_session_json};

    /// #1354a decision 2, #1324 D2: the whole-strip restart filter drops exactly a restarted strip's
    /// compensation lines before its matrix (its chain, its effects' sidechains, its routes from a
    /// tap before the fader), and keeps its matrix and post-fader lines and every other strip's.
    #[test]
    fn the_whole_strip_filter_drops_only_the_pre_fader_lines() {
        let mut model = parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("fixture parses");
        let mut pre = model.routes[0].clone();
        pre.id = session::StableId::parse("eq0-pre").expect("ID");
        pre.source = session::RouteSource::Track {
            track_id: session::StableId::parse("eq0").expect("ID"),
            tap: session::SendTap::PreFader,
        };
        model.routes.push(pre);
        let restart = super::StripRestart::new(&model, ["eq0"]);
        let gid = |text: &str| graph::StableGraphId::parse(text).expect("graph ID");
        let stage = |strip: &str, stage| graph::GraphNodeId::TrackStage {
            track_id: gid(strip),
            stage,
        };
        let line = |edge| graph::GraphDelayLine {
            edge,
            source: stage("eq0", graph::TrackStage::Input),
            destination: stage("eq0", graph::TrackStage::Input),
            line: 0,
            samples: 1,
        };
        let main = |strip: &str, at| graph::GraphEdgeId::TrackMain {
            target: stage(strip, at),
        };
        let effect = |strip: &str| graph::EffectNodeId {
            track_id: gid(strip),
            rack: graph::RackId::Simd1,
            effect_id: gid("eq"),
        };
        let cases = [
            (main("eq0", graph::TrackStage::PostFader), true),
            (
                graph::GraphEdgeId::TrackMain {
                    target: graph::GraphNodeId::Effect(effect("eq0")),
                },
                true,
            ),
            (
                graph::GraphEdgeId::EffectSidechain {
                    effect: effect("eq0"),
                    port: "sidechain-in".into(),
                },
                true,
            ),
            (
                graph::GraphEdgeId::RouteDestination {
                    route_id: gid("eq0-pre"),
                },
                true,
            ),
            (
                graph::GraphEdgeId::RouteSource {
                    route_id: gid("eq0-pre"),
                },
                true,
            ),
            (main("eq0", graph::TrackStage::PostMatrix), false),
            (
                graph::GraphEdgeId::RouteDestination {
                    route_id: gid("eq0-main"),
                },
                false,
            ),
            (main("eq1", graph::TrackStage::PostFader), false),
            (
                graph::GraphEdgeId::EffectSidechain {
                    effect: effect("eq1"),
                    port: "sidechain-in".into(),
                },
                false,
            ),
        ];
        for (edge, drops) in cases {
            assert_eq!(restart.drops_line(&line(edge.clone())), drops, "{edge:?}");
        }
    }

    const SLOT: &str = "comp";

    fn layout() -> graph::EffectLaneLayout {
        graph::EffectLaneLayout {
            effect_id: effect_contract::EffectId::parse("miso.compressor").expect("effect ID"),
            state_layout_version: 1,
            state_sizes: effect_contract::StatePayloadSizes {
                common_bytes: 8,
                left_bytes: 64,
                right_bytes: 64,
            },
            bypass: false,
            latency: effect_contract::LatencySamples(0),
        }
    }

    fn lane_row(lane: u16, layout: graph::EffectLaneLayout) -> super::EffectLaneInventoryRow {
        super::EffectLaneInventoryRow {
            id: "eq0".into(),
            rack: graph::RackId::Simd1,
            effect_id: SLOT.into(),
            location: graph::GraphEffectSite::Lane(graph::GraphLaneLocation {
                unit: 0,
                slot: 0,
                lane,
            }),
            layout,
            kind: graph::EffectLaneKind::Plain,
        }
    }

    fn stable(id: &str) -> graph::StableGraphId {
        graph::StableGraphId::parse(id).expect("graph ID")
    }

    fn strip(id: &str) -> graph::GraphNodeId {
        graph::GraphNodeId::TrackStage {
            track_id: stable(id),
            stage: graph::TrackStage::PostMatrix,
        }
    }

    fn submix(id: &str) -> graph::GraphNodeId {
        graph::GraphNodeId::Submix {
            submix_id: stable(id),
        }
    }

    /// A line of the route `route`, whose endpoints are `from` and `to`, at index `line`.
    fn route_line(
        route: &str,
        from: &str,
        to: &str,
        line: u32,
        samples: u64,
    ) -> graph::GraphDelayLine {
        graph::GraphDelayLine {
            edge: graph::GraphEdgeId::RouteDestination {
                route_id: stable(route),
            },
            source: strip(from),
            destination: submix(to),
            line,
            samples,
        }
    }

    fn strip_delay(strip: &str, line: u32, samples: [u32; 2]) -> graph::GraphStripDelayLine {
        graph::GraphStripDelayLine {
            strip: strip.into(),
            line,
            samples,
        }
    }

    /// #1284 D1, #1402 D1: a lane carries when its delay is the same in both models, the strip
    /// reads the same source and both plans hold the strip's line; a strip with a lane that
    /// changed, whose line one plan lacks, or whose source changed, restarts. A lane with an empty
    /// ring has nothing to take.
    #[test]
    fn strip_delay_lanes_join_by_strip_and_lane() {
        use super::{StripDelayJoin, resolve_strip_delay_carry};
        let (left, right) = (graph::STRIP_DELAY_LEFT, graph::STRIP_DELAY_RIGHT);
        let before = [
            strip_delay("eq0", 0, [37, 53]),
            strip_delay("eq2", 1, [0, 19]),
            strip_delay("eq4", 2, [8, 8]),
            strip_delay("eq6", 3, [8, 8]),
        ];
        let own = [
            strip_delay("eq0", 1, [37, 60]),
            strip_delay("eq2", 0, [0, 19]),
            strip_delay("eq4", 3, [8, 8]),
            strip_delay("eq5", 2, [4, 4]),
            strip_delay("eq6", 4, [8, 10]),
        ];
        let joins = [
            // The right lane changed: the left carries and the strip restarts.
            StripDelayJoin {
                id: "eq0",
                equal: left,
                same_source: true,
                track: true,
            },
            // Unchanged, with no delay on the left: only the right takes its ring.
            StripDelayJoin {
                id: "eq2",
                equal: left | right,
                same_source: true,
                track: true,
            },
            // Unchanged and undelayed: no line on either side, nothing to do.
            StripDelayJoin {
                id: "eq3",
                equal: left | right,
                same_source: true,
                track: true,
            },
            // Equal lines, but the track reads another source: it restarts and takes nothing.
            StripDelayJoin {
                id: "eq4",
                equal: left | right,
                same_source: false,
                track: true,
            },
            // Unchanged, but the predecessor has no line: it restarts.
            StripDelayJoin {
                id: "eq5",
                equal: left | right,
                same_source: true,
                track: true,
            },
            // Equal declared delays, but the right ring grew (a claim line on one lane): the left
            // carries and the strip restarts.
            StripDelayJoin {
                id: "eq6",
                equal: left | right,
                same_source: true,
                track: true,
            },
        ];
        let carry = resolve_strip_delay_carry(&before, &own, &joins, 0);
        assert_eq!(
            carry.moves,
            vec![
                graph::GraphStripDelayMove {
                    successor: 1,
                    predecessor: 0,
                    lanes: left,
                },
                graph::GraphStripDelayMove {
                    successor: 0,
                    predecessor: 1,
                    lanes: right,
                },
                graph::GraphStripDelayMove {
                    successor: 4,
                    predecessor: 3,
                    lanes: left,
                },
            ]
        );
        assert_eq!(
            carry.restarted,
            vec![
                Box::<str>::from("eq0"),
                Box::<str>::from("eq4"),
                Box::<str>::from("eq5"),
                Box::<str>::from("eq6")
            ]
        );
    }

    /// #1402 R-B: a track whose source keeps its ID but is redeclared (another `frames`) reads a
    /// fresh ring from frame 0 (#1272 D3), so its line's pending frames are the old declaration's
    /// and it is not the same source; a track on a carried source is.
    #[test]
    fn a_redeclared_source_is_not_the_same_source() {
        use std::collections::BTreeSet;
        let mut committed = parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("fixture");
        // A second source, kept as declared, for the last track.
        let mut kept = committed.sources[0].clone();
        kept.id = session::StableId::parse("kept-source").expect("ID");
        kept.content = kept.content.replace("7e94", "0a0a");
        committed.sources.push(kept.clone());
        committed.sources.sort_by(|a, b| a.id.cmp(&b.id));
        committed.tracks.last_mut().expect("a track").source_id = kept.id;
        let mut model = committed.clone();
        let redeclared = model
            .sources
            .iter_mut()
            .find(|source| source.id.as_str() == "fixture-source")
            .expect("the fixture's source");
        redeclared.frames += 1;
        // The sources whose rings carry, as #1272 D3 decides: an unchanged declaration.
        let carried: BTreeSet<&str> = model
            .sources
            .iter()
            .filter(|source| committed.sources.contains(source))
            .map(|source| source.id.as_str())
            .collect();
        let joins = super::join_strip_delays(&committed, &model, &carried);
        let redeclared = "fixture-source";
        let tracks: Vec<(&str, bool)> = model
            .tracks
            .iter()
            .map(|track| {
                let join = joins
                    .iter()
                    .find(|join| join.id == track.id.as_str())
                    .expect("a joined track");
                (track.source_id.as_str(), join.same_source)
            })
            .collect();
        assert!(tracks.iter().any(|(source, _)| *source == redeclared));
        assert!(tracks.iter().any(|(source, _)| *source != redeclared));
        for (source, same_source) in tracks {
            assert_eq!(same_source, source != redeclared, "a track on {source}");
        }
    }

    /// #1284 D2: a send carries when both plans hold a live route of its ID, whatever unit it sits
    /// in; its retarget travels with it, and a send one plan lacks is neither moved nor written.
    #[test]
    fn live_routes_join_by_route_id() {
        use super::{RouteJoin, resolve_route_carry};
        let live = |route: &str, unit| graph::GraphLiveRoute {
            route_id: route.into(),
            unit,
            following: false,
            automated: [false; 2],
        };
        let before = [live("a-main", 3), live("b-main", 5)];
        let own = [live("a-main", 4), live("b-main", 2), live("c-main", 9)];
        let record = super::SendRecord::Folded(
            crate::route_controls::route_record(-6.0, [1.0, 0.0, 0.0, 1.0], false, [false; 2], 64)
                .expect("a record"),
        );
        let joins = [
            RouteJoin {
                id: "a-main",
                retarget: Some(record),
            },
            RouteJoin {
                id: "b-main",
                retarget: None,
            },
            // Present in the successor only.
            RouteJoin {
                id: "c-main",
                retarget: Some(record),
            },
            // Present in the predecessor's model but not a live route of either plan.
            RouteJoin {
                id: "d-main",
                retarget: Some(record),
            },
        ];
        let carry = resolve_route_carry(&before, &own, &joins);
        assert_eq!(
            carry.moves,
            vec![
                graph::GraphRouteMove {
                    successor: 4,
                    predecessor: 3,
                    predecessor_follows: false,
                },
                graph::GraphRouteMove {
                    successor: 2,
                    predecessor: 5,
                    predecessor_follows: false,
                },
            ]
        );
        assert_eq!(carry.retargets, vec![("a-main", record)]);
        assert!(
            !carry.follow_into_six_word,
            "no route follows in either plan"
        );
    }

    /// #1652 U2a fold 2 (MAJOR-A): the join flags a carry of a following predecessor route into a
    /// six-word successor route, and only that kind pair: six -> six, six -> seven and seven ->
    /// seven moves leave the flag clear, and a following route present in one plan only moves
    /// nothing and flags nothing.
    ///
    /// Test value: red if the flag reads the predecessor's kind alone (seven -> seven flagged:
    /// a rebuild that keeps the mute entry would over-report), the successor's alone, or ignores
    /// both (the seven -> six re-timing then completes exact).
    #[test]
    fn the_join_flags_a_following_route_carried_into_a_six_word_one() {
        use super::{RouteJoin, resolve_route_carry};
        let live = |route: &str, unit, following| graph::GraphLiveRoute {
            route_id: route.into(),
            unit,
            following,
            automated: [false; 2],
        };
        let join = |id| RouteJoin { id, retarget: None };
        // (case, predecessor following, successor following, flagged)
        for (case, before, after, flagged) in [
            ("six -> six", false, false, false),
            ("six -> seven", false, true, false),
            ("seven -> seven", true, true, false),
            ("seven -> six", true, false, true),
        ] {
            let predecessor = [live("a-main", 1, false), live("b-main", 2, before)];
            let own = [live("a-main", 3, false), live("b-main", 4, after)];
            let carry = resolve_route_carry(&predecessor, &own, &[join("a-main"), join("b-main")]);
            assert_eq!(carry.moves.len(), 2, "{case}: both sends carry");
            assert_eq!(carry.follow_into_six_word, flagged, "{case}");
        }
        // A following route the successor lacks is not a move.
        let predecessor = [live("a-main", 1, true)];
        let carry = resolve_route_carry(&predecessor, &[], &[join("a-main")]);
        assert!(carry.moves.is_empty() && !carry.follow_into_six_word);
    }

    /// B19-R7: the join flags a carry of a following route into a following route whose
    /// automated lanes lack a source lane's mute curve of the predecessor's, and only then: an
    /// unchanged or a larger set, and a kind change, leave it clear.
    ///
    /// Test value: red if the flag compares the sets for equality (an added curve would flag), or
    /// ignores the kinds.
    #[test]
    fn the_join_flags_a_following_route_that_loses_a_curve() {
        use super::{RouteJoin, resolve_route_carry};
        let left = [true, false];
        let right = [false, true];
        let both = [true, true];
        let none = [false; 2];
        let live = |unit, following, automated| graph::GraphLiveRoute {
            route_id: "a-main".into(),
            unit,
            following,
            automated,
        };
        // (case, predecessor, successor, flagged)
        for (case, before, after, flagged) in [
            ("same set", live(1, true, left), live(2, true, left), false),
            (
                "adds a lane",
                live(1, true, left),
                live(2, true, both),
                false,
            ),
            (
                "loses a lane",
                live(1, true, both),
                live(2, true, left),
                true,
            ),
            (
                "swaps a lane",
                live(1, true, left),
                live(2, true, right),
                true,
            ),
            (
                "seven -> six",
                live(1, true, left),
                live(2, false, none),
                false,
            ),
            (
                "six -> seven",
                live(1, false, none),
                live(2, true, left),
                false,
            ),
        ] {
            let carry = resolve_route_carry(
                &[before],
                &[after],
                &[RouteJoin {
                    id: "a-main",
                    retarget: None,
                }],
            );
            assert_eq!(carry.follow_loses_a_curve, flagged, "{case}");
            // B19-R8: every seven-to-seven carry, whatever its sets.
            assert_eq!(carry.follow_into_following, !case.contains("->"), "{case}");
        }
    }

    /// #1283 D1: lines join by edge ID and endpoints, never by index. A line moves when its
    /// edge exists on both sides with the same endpoints, whatever its length and whatever
    /// index either side gave it; a route re-pointed to another destination or another source
    /// keeps its route ID, and its line does not move; an edge with a line on one side only
    /// moves nothing. Red if the join keys by index (the added track shifts every index) or by
    /// route ID alone (a re-pointed route's line would continue into the wrong bus).
    #[test]
    fn delay_lines_join_by_edge_and_endpoints() {
        let mut before = vec![
            route_line("a-main", "a", "bus", 0, 480),
            route_line("b-main", "b", "bus", 1, 480),
            route_line("c-main", "c", "bus", 2, 480),
            route_line("d-main", "d", "bus", 3, 480),
            route_line("e-main", "e", "bus", 4, 480),
        ];
        let mut own = vec![
            // New route: no predecessor line.
            route_line("new-main", "new", "bus", 0, 480),
            // Unchanged, but at another index and another length.
            route_line("a-main", "a", "bus", 1, 512),
            // Re-pointed to another destination.
            route_line("b-main", "b", "other", 2, 480),
            // Re-pointed to another source.
            route_line("c-main", "x", "bus", 3, 480),
            // Unchanged.
            route_line("d-main", "d", "bus", 4, 480),
            // `e-main` has no line in this plan.
        ];
        before.sort_by(|l, r| l.edge.cmp(&r.edge));
        own.sort_by(|l, r| l.edge.cmp(&r.edge));
        let moves = super::resolve_delay_carry(&before, &own, |_| true);
        let mut moves: Vec<(u32, u32)> = moves
            .iter()
            .map(|step| (step.successor, step.predecessor))
            .collect();
        moves.sort_unstable();
        assert_eq!(moves, vec![(1, 0), (4, 3)]);
    }

    fn inventory_with(row: super::EffectLaneInventoryRow) -> super::PlanStateInventory {
        super::PlanStateInventory {
            plan_identity: 1,
            sources: Box::default(),
            input_sections: Box::default(),
            strip_stages: Box::default(),
            effect_lanes: Box::new([row]),
            meters: Box::default(),
            observations: Box::default(),
            sample_rate_hz: 48_000,
            quantum_frames: 128,
            spectrum: Box::default(),
            spectrum_collection: false,
            spectrum_pairing: Box::default(),
            meter_pairing: Box::default(),
            delay_lines: Box::default(),
            strip_delays: Box::default(),
            live_routes: Box::default(),
            follow_into_six_word: false,
            follow_into_following: false,
            follow_loses_a_curve: false,
            follow_retarget_ramp: 0,
            restarted: Box::default(),
            fade_arms: Box::default(),
            fade_watch: graph::GraphFadeArmWatch::default(),
            route_copies: Box::default(),
            input_floors: Box::default(),
            raised_arrivals: Box::default(),
            // #1396 D2: no read-ahead.
            source_read_offset: 0,
            published_generation: source::PublishedGeneration::new(source::SourceGeneration(1)),

            lead_samples: 0,
            prime_bytes: 0,
        }
    }

    fn spectrum_row(
        target: crate::spectrum::SpectrumTarget,
        channels: crate::spectrum::SpectrumChannels,
        entry: usize,
        unit: u32,
    ) -> super::SpectrumInventoryRow {
        super::SpectrumInventoryRow {
            target,
            channels,
            entry,
            location: graph::GraphObserverLocation {
                unit,
                member: graph::GraphObserverLocation::NO_MEMBER,
                index: 0,
            },
        }
    }

    /// #1395 D1: a capture carries on an equal target and channel mask at an equal rate and
    /// quantum, found by key and not by location; any other difference starts it fresh.
    #[test]
    fn a_spectrum_capture_carries_by_key_at_one_shape() {
        use crate::spectrum::{SpectrumChannels, SpectrumTarget};
        let target = || SpectrumTarget::TrackPostMatrix("eq0".into());
        let mut before = inventory_with(lane_row(1, layout()));
        before.spectrum = Box::new([spectrum_row(target(), SpectrumChannels::Stereo, 0, 7)]);
        let own = [spectrum_row(target(), SpectrumChannels::Stereo, 0, 2)];
        let carry = super::resolve_spectrum_carry(&before, &own, false, 48_000, 128, |_| true);
        assert_eq!(carry.moves.len(), 1);
        assert_eq!(
            carry.moves[0].predecessor.unit, 7,
            "named by its own location"
        );
        assert_eq!(carry.moves[0].successor.unit, 2);
        assert_eq!(carry.pairing.len(), 1);
        for (label, own, rate, quantum) in [
            (
                "channels",
                [spectrum_row(target(), SpectrumChannels::Left, 0, 2)],
                48_000,
                128,
            ),
            (
                "target",
                [spectrum_row(
                    SpectrumTarget::TrackPostInputBuiltins("eq0".into()),
                    SpectrumChannels::Stereo,
                    0,
                    2,
                )],
                48_000,
                128,
            ),
            (
                "rate",
                [spectrum_row(target(), SpectrumChannels::Stereo, 0, 2)],
                96_000,
                128,
            ),
            (
                "quantum",
                [spectrum_row(target(), SpectrumChannels::Stereo, 0, 2)],
                48_000,
                256,
            ),
        ] {
            let carry =
                super::resolve_spectrum_carry(&before, &own, false, rate, quantum, |_| true);
            assert!(
                carry.moves.is_empty() && carry.pairing.is_empty(),
                "a changed {label} starts fresh"
            );
        }
    }

    /// #1395b D1: a collection's entries are matched by key in any order, an entry whose key is
    /// absent from the predecessor is fresh, and a single capture and a collection never carry
    /// into each other.
    #[test]
    fn a_spectrum_collection_carries_by_key_in_any_order() {
        use crate::spectrum::{SpectrumChannels, SpectrumTarget};
        let t = || SpectrumTarget::TrackPostMatrix("t".into());
        let u = || SpectrumTarget::TrackPostInputBuiltins("u".into());
        let mut before = inventory_with(lane_row(1, layout()));
        before.spectrum_collection = true;
        before.spectrum = Box::new([
            spectrum_row(t(), SpectrumChannels::Stereo, 0, 10),
            spectrum_row(u(), SpectrumChannels::Left, 1, 11),
        ]);
        // Reordered, with a changed key and an added entry.
        let own = [
            spectrum_row(u(), SpectrumChannels::Left, 0, 20),
            spectrum_row(t(), SpectrumChannels::Stereo, 1, 21),
            spectrum_row(u(), SpectrumChannels::Right, 2, 22),
        ];
        let carry = super::resolve_spectrum_carry(&before, &own, true, 48_000, 128, |_| true);
        let pairs: Vec<(usize, usize)> = carry
            .pairing
            .iter()
            .map(|pair| (pair.successor, pair.predecessor))
            .collect();
        assert_eq!(pairs, vec![(0, 1), (1, 0)], "by key, not by position");
        let moves: Vec<(u32, u32)> = carry
            .moves
            .iter()
            .map(|step| (step.successor.unit, step.predecessor.unit))
            .collect();
        assert_eq!(
            moves,
            vec![(20, 11), (21, 10)],
            "named by their own locations"
        );
        let none = super::resolve_spectrum_carry(&before, &own, false, 48_000, 128, |_| true);
        assert!(
            none.moves.is_empty() && none.pairing.is_empty(),
            "a collection does not carry into a single capture"
        );
        let mut single = inventory_with(lane_row(1, layout()));
        single.spectrum = Box::new([spectrum_row(t(), SpectrumChannels::Stereo, 0, 10)]);
        let none = super::resolve_spectrum_carry(&single, &own, true, 48_000, 128, |_| true);
        assert!(
            none.moves.is_empty() && none.pairing.is_empty(),
            "a single capture does not carry into a collection"
        );
    }

    /// D1: the slot's model values are equal and both plans bank it, but the predecessor's lane has
    /// another prepared layout (state sizes, state layout version, prepared bypass or latency).
    /// The lane is not moved and its strip restarts. A join that trusts the existence of two rows
    /// moves a payload the successor's bank would refuse, or read at another size.
    #[test]
    fn a_lane_of_another_prepared_layout_is_not_moved() {
        let join = |same: bool| super::ChainJoin {
            id: "eq0",
            chain_changed: false,
            surviving_inserts: Vec::new(),
            owners: vec![super::ConsoleOwnerJoin {
                rack: graph::RackId::Simd1,
                slot: SLOT,
                address: super::LiveEffectAddress::console(0),
                state: if same {
                    super::FamilyJoin::Same
                } else {
                    super::FamilyJoin::Prepared
                },
            }],
        };
        let own = [lane_row(3, layout())];
        let carry = super::resolve_effect_carry(
            &inventory_with(lane_row(1, layout())),
            &[join(true)],
            &own,
            &mut |_, _, _| None,
        );
        assert_eq!(carry.moves.len(), 1);
        assert_eq!(
            carry.moves[0].successor,
            graph::GraphEffectSite::Lane(graph::GraphLaneLocation {
                unit: 0,
                slot: 0,
                lane: 3
            })
        );
        assert_eq!(
            carry.moves[0].predecessor,
            graph::GraphEffectSite::Lane(graph::GraphLaneLocation {
                unit: 0,
                slot: 0,
                lane: 1
            })
        );
        assert_eq!(carry.scratch_bytes, 136);
        assert!(carry.restarted.is_empty());
        let other = |edit: fn(&mut graph::EffectLaneLayout)| {
            let mut other = layout();
            edit(&mut other);
            other
        };
        let layouts = [
            other(|layout| layout.state_sizes.left_bytes += 4),
            other(|layout| layout.state_layout_version += 1),
            other(|layout| layout.bypass = true),
            other(|layout| layout.latency = effect_contract::LatencySamples(5)),
        ];
        for (index, different) in layouts.into_iter().enumerate() {
            let carry = super::resolve_effect_carry(
                &inventory_with(lane_row(1, different)),
                &[join(true)],
                &own,
                &mut |_, _, _| None,
            );
            assert!(carry.moves.is_empty(), "layout {index}");
            assert_eq!(
                carry.restarted,
                vec![Box::<str>::from("eq0")],
                "layout {index}"
            );
            assert_eq!(carry.scratch_bytes, 0, "layout {index}");
        }
        // A changed model value restarts the strip on equal layouts, too.
        let carry = super::resolve_effect_carry(
            &inventory_with(lane_row(1, layout())),
            &[join(false)],
            &own,
            &mut |_, _, _| None,
        );
        assert!(carry.moves.is_empty());
        assert_eq!(carry.restarted, vec![Box::<str>::from("eq0")]);
    }

    /// #1280 D1: a live difference carries the lane only when the successor's producer accepts its
    /// records. A producer that refuses one (no cell for it) restarts the strip and moves nothing,
    /// so a record is never written to a cell that is not there; one that accepts gives the lane
    /// its retarget.
    #[test]
    fn a_live_difference_carries_only_when_the_successor_accepts_its_records() {
        let live = || super::ChainJoin {
            id: "eq0",
            chain_changed: false,
            surviving_inserts: Vec::new(),
            owners: vec![super::ConsoleOwnerJoin {
                rack: graph::RackId::Simd1,
                slot: SLOT,
                address: super::LiveEffectAddress::console(0),
                state: super::FamilyJoin::Live(super::EffectInstanceRecords {
                    records: vec![effect_contract::EffectControlRecord::Bypass(
                        effect_contract::BypassSwitch {
                            bypassed: true,
                            ramp_samples: 64,
                        },
                    )],
                    targets: None,
                }),
            }],
        };
        let own = [lane_row(3, layout())];
        let inventory = inventory_with(lane_row(1, layout()));
        let refused = super::resolve_effect_carry(&inventory, &[live()], &own, &mut |_, _, _| None);
        assert!(refused.moves.is_empty() && refused.retargets.is_empty());
        assert_eq!(refused.restarted, vec![Box::<str>::from("eq0")]);
        let mut asked = Vec::new();
        let accepted = super::resolve_effect_carry(
            &inventory,
            &[live()],
            &own,
            &mut |strip, address, instance| {
                asked.push((strip.to_owned(), address, instance.records[0]));
                Some(super::EffectRetarget {
                    producer: 7,
                    records: Vec::new(),
                    targets: None,
                })
            },
        );
        assert_eq!(accepted.moves.len(), 1);
        assert_eq!(accepted.retargets.len(), 1);
        assert_eq!(accepted.retargets[0].producer, 7);
        assert!(accepted.restarted.is_empty());
        assert_eq!(
            asked,
            vec![(
                "eq0".to_owned(),
                super::LiveEffectAddress::console(0),
                effect_contract::EffectControlRecord::Bypass(effect_contract::BypassSwitch {
                    bypassed: true,
                    ramp_samples: 64,
                })
            )]
        );
    }

    /// The carry program is built only when some owner may carry, and the effect carry refuses a
    /// plan without one (`NoEffectCarryProgram`). A strip whose only non-prepared owner is a
    /// surviving insert must therefore say it may carry; a join that looked at the console slots
    /// alone would refuse a valid transaction with `host.carry.install`. (No swap reaches this
    /// today: a strip present in both models also joins its fader, which is never prepared when
    /// live controls attach, so the program is built anyway. The check defends the invariant at the
    /// join that owns it.)
    #[test]
    fn a_surviving_insert_alone_makes_a_chain_join_carry() {
        let insert = |state| super::InsertOwnerJoin {
            id: "lim",
            address: super::LiveEffectAddress::insert(0),
            state,
        };
        let chain = |state| super::ChainJoin {
            id: "eq0",
            chain_changed: false,
            owners: Vec::new(),
            surviving_inserts: vec![insert(state)],
        };
        assert!(chain(super::FamilyJoin::Same).may_carry());
        assert!(!chain(super::FamilyJoin::Prepared).may_carry());
    }

    #[test]
    fn typed_diagnostic_adapters_preserve_projection_and_bound() {
        let effects = EffectDiagnosticSet(vec![
            EffectDiagnostic {
                code: "effect.first",
                path: "$.tracks[id=one]".into(),
            },
            EffectDiagnostic {
                code: "effect.second",
                path: "$.tracks[id=two].effects".into(),
            },
        ]);
        let projected = effect_diagnostics(effects);
        assert_eq!(projected.kind(), super::PrepareRejection::Effect);
        assert_eq!(
            projected.as_bytes(),
            b"effect.first\t$.tracks[id=one]\neffect.second\t$.tracks[id=two].effects\n"
        );
        assert_eq!(
            effect_diagnostics(EffectDiagnosticSet(Vec::new())).as_bytes(),
            b""
        );
        let many = EffectDiagnosticSet(
            (0..65)
                .map(|index| EffectDiagnostic {
                    code: "effect.too_many",
                    path: format!("$.effects[index={index:02}]"),
                })
                .collect(),
        );
        assert_eq!(many.0.len(), 65);
        assert!(many.0.iter().enumerate().all(|(index, diagnostic)| {
            diagnostic.code == "effect.too_many"
                && diagnostic.path == format!("$.effects[index={index:02}]")
        }));
        let bounded = effect_diagnostics(many).into_bytes();
        let mut expected = String::new();
        for index in 0..64 {
            expected.push_str(&format!("effect.too_many\t$.effects[index={index:02}]\n"));
        }
        assert_eq!(bounded, expected.into_bytes());

        let invalid = include_str!("../../../fixtures/session/v1/canonical.json")
            .replace("\"sample_rate_hz\": 48000", "\"sample_rate_hz\": 123")
            .replace("\"quantum_frames\": 128", "\"quantum_frames\": 0");
        let session = parse_session_json(&invalid).expect_err("invalid shape");
        assert_eq!(session.diagnostics().len(), 2);
        assert!(
            session
                .diagnostics()
                .iter()
                .all(|diagnostic| { diagnostic.span.is_some() && !diagnostic.message.is_empty() })
        );
        let projected = session_diagnostics(session);
        assert_eq!(projected.kind(), super::PrepareRejection::Session);
        assert_eq!(
            projected.as_bytes(),
            b"capacity.zero\t$.quantum_frames\nsample_rate.unsupported_at_launch\t$.sample_rate_hz\n"
        );

        let canonical = include_str!("../../../fixtures/session/v1/canonical.json");
        let mut many_document = canonical[..canonical.len() - 2].to_owned();
        for index in 0..65 {
            many_document.push_str(&format!(",\n  \"unexpected_{index:02}\": null"));
        }
        many_document.push_str("\n}\n");
        let many = parse_session_json(&many_document).expect_err("65 unknown fields");
        assert_eq!(many.diagnostics().len(), 64);
        assert!(
            many.diagnostics()
                .iter()
                .enumerate()
                .all(|(index, diagnostic)| {
                    diagnostic.code == DiagnosticCode::UnknownField
                        && diagnostic.path.to_string() == format!("$.unexpected_{index:02}")
                })
        );
        let bounded = session_diagnostics(many).into_bytes();
        let mut expected = String::new();
        for index in 0..64 {
            expected.push_str(&format!("schema.unknown_field\t$.unexpected_{index:02}\n"));
        }
        assert_eq!(bounded, expected.into_bytes());
    }
}
