//! The shared host preparation pipeline: JSON in, a prepared plan and a source control set out.
//!
//! This is the ~300 lines that the C ABI host and the browser host each carried a private copy of.
//! Everything that differed between the two copies is a field of [`HostPrepareCaps`]; everything
//! else was identical, down to the order of the checks.

use core::num::{NonZeroU32, NonZeroUsize};
use std::collections::BTreeSet;

use builtins::{
    BuiltinLaneSelector, InputBoundCache, MeterConfig, MeterHandle, MeterMetricSet, MeterTap,
    checked_fader_gain,
};
use builtins_compiler::{
    BuiltinCompileCaps, LiveValuesSuperseded, MeterConsumer, MeterRequest, SelectedMeterRequest,
    TrackControlProducer, TrackControlRecord, TrackControlRequest, TrackFaderRecord,
    lower_matrix_or_pan, prepare_selected_session_builtins_between_render_calls,
    prepare_selected_session_builtins_with_live_controls,
    prepare_session_builtins_between_render_calls, prepare_session_builtins_with_live_controls,
    session_structural_symmetry,
};
use effect_compiler::{
    EffectCompileCaps, EffectControlProducer, EffectControlResources, EffectObservationHandle,
    LiveEffectAddress, attach_effect_live_controls, attach_effect_observation,
    effect_control_resources, launch_native_effect_registry, prepare_native_session_effects,
};
use effect_contract::TailSamples;
// `LiveRampRow` and `LiveRamps` moved to `effect_contract::control` and `controls` (decision 17);
// these re-exports keep the old paths, so no call site changes.
pub use controls::LiveRamps;
pub use effect_contract::control::LiveRampRow;
use engine::realtime::{GATE_REVISION_MAX, PreparedRenderPlan};
use graph::{
    GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, StableGraphId,
    TrackStage,
};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{
    CompileCaps, CompiledSession, EditRampRow, EditRampTarget, EditRamps, MatrixOrPan,
    SessionModel, StripKind, StripRef, compile_session, parse_session_json,
};
use source::{
    PcmSourceRing, PcmSourceRingConfig, SourceFrame, SourceGeneration, SourceGraphSource,
    SourceGraphTrackMapping, prepare_graph_source_set,
};
use std::collections::BTreeMap;

use crate::diagnostics::{PrepareDiagnostics, PrepareRejection, diagnostic_lines};
use crate::route_controls::{RouteControlProducer, RouteControlResources};
use crate::source::{ControlSourceBuilder, SourceControlSet};
use crate::spectrum::{
    SpectrumCapture, SpectrumCaptureCollection, SpectrumCaptureCollectionRequest,
    SpectrumCaptureRequest, SpectrumCaptureResources, SpectrumPrepareError,
    prepare_capture_collection, spectrum_capture_collection_resources,
};

#[derive(Clone, Copy)]
enum SpectrumPreparationRequest<'a> {
    Single(&'a SpectrumCaptureRequest),
    Collection(&'a SpectrumCaptureCollectionRequest),
}

enum PreparedSpectrumCapture {
    Single(SpectrumCapture),
    Collection(SpectrumCaptureCollection),
}

/// The longest producer-thread stall the default source ring hides without an underrun.
pub const SOURCE_STALL_TOLERANCE_MS: u32 = 100;

/// Derive the default per-source ring capacity from the document's rate and quantum.
///
/// The ring covers `ceil(100 ms * fs / quantum)` quanta plus one quantum held by the consumer and
/// one in the recycle path. The result is therefore always a whole number of quanta. A zero
/// quantum returns zero; callers validate the document before using the answer.
#[must_use]
pub const fn default_source_ring_frames(sample_rate_hz: u32, quantum_frames: u32) -> u32 {
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
    /// Optional per-source channel-count cap. `None` accepts whatever the session declares.
    pub maximum_source_channels: Option<u32>,
    /// Automation spans the effect compiler may schedule in one block.
    pub maximum_automation_spans_per_block: u32,
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
    /// Compiled output tail.
    pub output_tail: TailSamples,
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
    /// Meter window in frames for every meter (default or selected), or `None` for no meters.
    pub meter_period_frames: Option<NonZeroU32>,
    /// Bounded per-meter snapshot queue depth.
    pub meter_queue_depth: NonZeroUsize,
    /// Which chain boundary the default per-track meters observe.
    pub meter_tap: MeterTap,
    /// Maximum declared observation taps to bind per effect instance, or `0` for **no observation
    /// capacity at all** (issue #143 D3, level 1).
    ///
    /// Zero is the honest form of "observation off": `attach_effect_observation` is never
    /// called, so the compiled plan holds no lane, no accumulator and no conflating cell — not a
    /// disabled one, none — and `observation_retained_bytes` is zero. Nonzero requires a control
    /// channel, because a subscription is a record in the effect's own observation queue, which only
    /// the effect's control lane drains.
    ///
    /// The default observation period is derived from `meter_period_frames`; individual taps
    /// can use other periods. Latest gain-reduction observations can be independently aged,
    /// so a browser frame's peak interval does not establish the gain-reduction interval.
    pub observation_taps: u32,
    /// The strip whose gain reduction is reported as the master's, or `None` (issue #143 D6): an
    /// index into [`HostLiveControlHandles::strips`], tracks first, then submixes (issue #1208
    /// D3).
    ///
    /// The master reading is a *designation*, not a discovery: no strip is structurally the
    /// master. Issue #1208 widens that stopgap from tracks to every strip without retiring it, so
    /// a bus whose strip carries a limiter can now be designated. The field, its refusal code
    /// `host.observation.master_track` and the browser and SDK spellings keep their track-era
    /// names (DESIGN P17).
    pub master_track: Option<u32>,
}

impl Default for HostLiveControlRequest {
    fn default() -> Self {
        Self {
            meter_period_frames: None,
            meter_queue_depth: NonZeroUsize::MIN,
            meter_tap: MeterTap::PostMatrix,
            observation_taps: 0,
            master_track: None,
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
    /// One lane per route into a submix; a live route loses the route fold.
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
    /// The designated master strip index, echoed back after validation against `strips.len()`
    /// (issue #1208 D3): tracks first, then submixes. Still a designation, not a discovery.
    pub master_track: Option<u32>,
    /// The `live_values_superseded` count this plan's strip and route cell readers add to
    /// (#1312 D2): the session's own count when the host passed one to
    /// [`prepare_host_runtime_with_live_lanes`], else a fresh count this preparation made. Read
    /// it with [`LiveValuesSuperseded::get`].
    pub live_values_superseded: LiveValuesSuperseded,
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
/// `strip_id_bytes` long in total (#1256 D3).
///
/// This is the mirror a host's pre-flight resource projection reads, as `control_table_bytes` is
/// for the source producers: a function of the producer's own layout, so the host never spells
/// it. The rings the producers share with the plan are not here; builtins charges them in
/// `builtin_retained_payload_bytes`.
#[must_use]
pub fn strip_control_table_bytes(strip_count: usize, strip_id_bytes: usize) -> Option<u64> {
    let table = core::alloc::Layout::array::<TrackControlProducer>(strip_count).ok()?;
    let ids = core::alloc::Layout::array::<u8>(strip_id_bytes).ok()?;
    u64::try_from(table.size())
        .ok()?
        .checked_add(u64::try_from(ids.size()).ok()?)
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
    /// at least one fader, mute or matrix stage or console effect slot does not carry, so it
    /// starts at rest (#1277 D4, D6; #1279 D1): a prepared value differs, the two plans bank the
    /// stage in different forms or attach different control kinds, or a console slot has no
    /// effect bank lane without live control in both plans. A strip whose effect chain changed (an
    /// insert added, removed or reordered, or a session console slot added, removed or moved) is
    /// listed too, even when its remaining owners carry (D15-9). A strip whose surviving insert
    /// (an insert present in both plans' chains) has a lane that does not carry is listed as well:
    /// its insert starts at rest on the swap, and no insert lane carries until #1281 and #1282
    /// (root amendment of 2026-10-12 to #1277 D6 and #1279). Sorted. Added strips are not
    /// listed, and neither is any strip of a plan prepared without a predecessor.
    /// *Duck-swap a strip whose state cannot continue across a plan swap* (#1324) consumes it.
    #[must_use]
    pub fn restarted_strips(&self) -> &[Box<str>] {
        &self.inventory.restarted
    }
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
    /// The strips this plan restarts relative to the plan it was prepared to displace (#1277 D6):
    /// present in both, with at least one owner that does not carry. Sorted by strip ID. Empty
    /// for a plan prepared without a predecessor.
    restarted: Box<[Box<str>]>,
}

/// One strip stage of a prepared plan, owner key `(strip ID, PostFader)` or `(strip ID,
/// PostMatrix)`, as its successor's preparation sees it (issue #1277 D1).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StripStageInventoryRow {
    /// The strip ID: a track's or a submix's.
    pub(crate) id: Box<str>,
    /// Whose state the row holds.
    pub(crate) owner: graph::BuiltinStripOwner,
    /// The kind of bank the lane sits in. A fused bank holds both owners at one location; a
    /// split one holds each owner in a bank of its own.
    pub(crate) stage: graph::BuiltinStripStage,
    /// Where the lane sits in this plan. Meaningful only to this plan's successor's carry
    /// program, which names it as the lane to copy from.
    pub(crate) location: graph::GraphLaneLocation,
    /// Whether this plan attached latest-target cells to the strip's fader, mute and matrix
    /// stages (the control kind of #1277 D4).
    pub(crate) live: bool,
}

/// One banked effect lane of a prepared plan, owner key `(strip ID, rack, effect ID)` (issue
/// #1279 D1): a lane of a bank stage with no live control lane. A console slot's effect ID is its
/// slot ID, and its rack is `Simd1` in `pre_insert` and `Simd2` in `post_insert`. A lane of a
/// bank with a live control lane, a lowered bypass or no bank at all has no row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EffectLaneInventoryRow {
    /// The strip ID: a track's or a submix's.
    pub(crate) id: Box<str>,
    /// The rack the effect sits in.
    pub(crate) rack: graph::RackId,
    /// The effect ID: a console slot's slot ID.
    pub(crate) effect_id: Box<str>,
    /// Where the lane sits in this plan. Meaningful only to this plan's successor's carry
    /// program, which names it as the lane to copy from.
    pub(crate) location: graph::GraphLaneLocation,
    /// What the successor's lane must share with this one for the state to move.
    pub(crate) layout: graph::EffectLaneLayout,
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

    /// Strip stage rows, one per banked fader or matrix lane (issue #1277).
    #[must_use]
    pub fn strip_stage_count(&self) -> usize {
        self.strip_stages.len()
    }

    /// Banked effect lane rows, one per lane of an effect bank stage with no live control lane
    /// (issue #1279).
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

    /// Heap bytes this inventory retains: its row tables and their ID text.
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
            + self.restarted.iter().map(|id| id.len()).sum::<usize>();
        u64::try_from(
            core::mem::size_of_val::<[SourceInventoryRow]>(&self.sources)
                + core::mem::size_of_val::<[InputSectionInventoryRow]>(&self.input_sections)
                + core::mem::size_of_val::<[StripStageInventoryRow]>(&self.strip_stages)
                + core::mem::size_of_val::<[EffectLaneInventoryRow]>(&self.effect_lanes)
                + core::mem::size_of_val::<[Box<str>]>(&self.restarted)
                + ids,
        )
        .unwrap_or(u64::MAX)
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
/// slot (#1279) carries when no prepared value of it differs, its two plans bank it in an effect
/// stage with no live control lane, and the prepared layouts of the two lanes are equal; a
/// strip whose effect chain changed restarts even if its remaining slots carry.
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
}

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
        false,
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
        false,
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
/// `bound_cache` (#1471) is a design-bound cache the host keeps on its control thread across
/// preparations (#1457): a design it holds is not computed again, and every prepared value is the
/// same with and without it. `None` computes every design, as before.
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
    bound_cache: Option<&mut InputBoundCache>,
    superseded: &LiveValuesSuperseded,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let (prepared, handles, _) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        None,
        false,
        Backend::current(),
        None,
        None,
        lanes,
        bound_cache,
        Some(superseded),
    )?;
    Ok((prepared, handles))
}

/// [`prepare_host_runtime_with_live_lanes`] for a successor of a running plan (issues #1053 and
/// #1269): the selected live lanes attach exactly as there, and every source `base` left
/// unchanged is prepared vacant for the carry program exactly as in
/// [`prepare_host_runtime_successor`]. `bound_cache` is as in
/// [`prepare_host_runtime_with_live_lanes`].
pub fn prepare_host_runtime_with_live_lanes_successor(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    lanes: HostLiveLanes,
    base: SuccessorBase<'_>,
    bound_cache: Option<&mut InputBoundCache>,
    superseded: &LiveValuesSuperseded,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let (prepared, handles, _) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        None,
        false,
        Backend::current(),
        None,
        Some(base),
        lanes,
        bound_cache,
        Some(superseded),
    )?;
    Ok((prepared, handles))
}

/// Prepare a host with live-control handles and one selected, fixed-size spectrum observer.
///
/// Meter configuration remains part of [`HostLiveControlRequest`], so this entry keeps the
/// live-control, meter and spectrum resources in one transactional preparation boundary.
///
/// `superseded` is the session's `live_values_superseded` count, as in
/// [`prepare_host_runtime_with_live_lanes`].
pub fn prepare_host_runtime_with_live_controls_and_spectrum(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    spectrum: &SpectrumCaptureRequest,
    lanes: HostLiveLanes,
    superseded: &LiveValuesSuperseded,
) -> Result<(PreparedHost, HostLiveControlHandles, SpectrumCapture), PrepareDiagnostics> {
    let (prepared, handles, capture) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        None,
        false,
        Backend::current(),
        Some(SpectrumPreparationRequest::Single(spectrum)),
        None,
        lanes,
        None,
        Some(superseded),
    )?;
    let capture = match capture.ok_or_else(|| resource("host.spectrum.capture"))? {
        PreparedSpectrumCapture::Single(capture) => capture,
        PreparedSpectrumCapture::Collection(_) => {
            return Err(resource("host.spectrum.capture"));
        }
    };
    Ok((prepared, handles, capture))
}

/// Prepare a host with live-control handles and several atomically selectable spectrum observers.
///
/// `superseded` is the session's `live_values_superseded` count, as in
/// [`prepare_host_runtime_with_live_lanes`].
pub fn prepare_host_runtime_with_live_controls_and_spectrum_collection(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    spectrum: &SpectrumCaptureCollectionRequest,
    lanes: HostLiveLanes,
    superseded: &LiveValuesSuperseded,
) -> Result<
    (
        PreparedHost,
        HostLiveControlHandles,
        SpectrumCaptureCollection,
    ),
    PrepareDiagnostics,
> {
    let (prepared, handles, capture) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        None,
        false,
        Backend::current(),
        Some(SpectrumPreparationRequest::Collection(spectrum)),
        None,
        lanes,
        None,
        Some(superseded),
    )?;
    let capture = match capture.ok_or_else(|| resource("host.spectrum.capture"))? {
        PreparedSpectrumCapture::Collection(capture) => capture,
        PreparedSpectrumCapture::Single(_) => return Err(resource("host.spectrum.capture")),
    };
    Ok((prepared, handles, capture))
}

/// [`prepare_host_runtime_with_live_controls_and_spectrum`] for a successor of a running plan
/// (issue #1401 D1): every source `base` left unchanged is prepared vacant for the carry program
/// exactly as in [`prepare_host_runtime_successor`], and control delivery is concurrent.
///
/// The successor gets a fresh capture, sized and validated as at boot and allocated here, on the
/// control thread; carrying capture state across the swap is #1395. A spectrum target the
/// successor does not have is refused with `host.spectrum.target`, before anything of `base` moves.
pub fn prepare_host_runtime_with_live_controls_and_spectrum_successor(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    spectrum: &SpectrumCaptureRequest,
    lanes: HostLiveLanes,
    superseded: &LiveValuesSuperseded,
    base: SuccessorBase<'_>,
) -> Result<(PreparedHost, HostLiveControlHandles, SpectrumCapture), PrepareDiagnostics> {
    let (prepared, handles, capture) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        None,
        false,
        Backend::current(),
        Some(SpectrumPreparationRequest::Single(spectrum)),
        Some(base),
        lanes,
        None,
        Some(superseded),
    )?;
    let capture = match capture.ok_or_else(|| resource("host.spectrum.capture"))? {
        PreparedSpectrumCapture::Single(capture) => capture,
        PreparedSpectrumCapture::Collection(_) => {
            return Err(resource("host.spectrum.capture"));
        }
    };
    Ok((prepared, handles, capture))
}

/// [`prepare_host_runtime_with_live_controls_and_spectrum_collection`] for a successor of a
/// running plan: see [`prepare_host_runtime_with_live_controls_and_spectrum_successor`].
pub fn prepare_host_runtime_with_live_controls_and_spectrum_collection_successor(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    spectrum: &SpectrumCaptureCollectionRequest,
    lanes: HostLiveLanes,
    superseded: &LiveValuesSuperseded,
    base: SuccessorBase<'_>,
) -> Result<
    (
        PreparedHost,
        HostLiveControlHandles,
        SpectrumCaptureCollection,
    ),
    PrepareDiagnostics,
> {
    let (prepared, handles, capture) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        Some(live_controls),
        None,
        false,
        Backend::current(),
        Some(SpectrumPreparationRequest::Collection(spectrum)),
        Some(base),
        lanes,
        None,
        Some(superseded),
    )?;
    let capture = match capture.ok_or_else(|| resource("host.spectrum.capture"))? {
        PreparedSpectrumCapture::Collection(capture) => capture,
        PreparedSpectrumCapture::Single(_) => return Err(resource("host.spectrum.capture")),
    };
    Ok((prepared, handles, capture))
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
        false,
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
        false,
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
        false,
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
        false,
        backend,
        Some(base),
    )
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
    prepare_host_runtime_with_live_controls_policy(compiled, caps, None, false, backend, None)
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
    prepare_host_runtime_with_live_controls_policy(compiled, caps, None, false, backend, Some(base))
        .map(|(prepared, _)| prepared)
}

/// Test-support width seam (issue #1277 gate 3): [`prepare_host_runtime_between_render_calls`] on
/// an explicit bank width, the browser's form, whose fader and matrix banks fuse into one stage.
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_prepare_host_runtime_between_render_calls_on(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    backend: Backend,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    prepare_host_runtime_with_live_controls_policy(
        compiled,
        caps,
        Some(live_controls),
        true,
        backend,
        None,
    )
}

/// Test-support width seam (issue #1277 gate 3): the successor of a running plan, prepared between
/// render calls. No host prepares a successor in that form today; the seam proves the fused stage
/// carries (D8, D9).
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_prepare_host_runtime_between_render_calls_successor_on(
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
        true,
        backend,
        Some(base),
    )
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
        false,
        backend,
        None,
    )
}

/// Prepare a host whose caller retains every returned producer endpoint and admits records only
/// between exclusive render calls, with no concurrent enqueue while render is running.
///
/// Selecting this entry is the caller's declaration of that ownership and scheduling contract; it
/// does not add synchronization or enforce the contract at runtime.
#[allow(clippy::too_many_lines)]
pub fn prepare_host_runtime_between_render_calls(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    prepare_host_runtime_between_render_calls_with_backend(
        compiled,
        caps,
        live_controls,
        Backend::current(),
    )
}

/// Prepare the serialized builtin lowering on an explicit backend.
///
/// The only caller is [`prepare_host_runtime_between_render_calls`], with [`Backend::current`].
/// Its other caller, #587's builtin batch endpoint, was deleted as unused with the rest of #140's
/// unwired delivery stack (#1056). Folding this back into its caller is left for a change that
/// may move the shipped artifact: panic locations below this line would shift.
pub(crate) fn prepare_host_runtime_between_render_calls_with_backend(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: &HostLiveControlRequest,
    backend: Backend,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    prepare_host_runtime_with_live_controls_policy(
        compiled,
        caps,
        Some(live_controls),
        true,
        backend,
        None,
    )
}

/// Prepare a serialized host with exactly the caller-selected meter observers.
///
/// `superseded` is the session's `live_values_superseded` count, as in
/// [`prepare_host_runtime_with_live_lanes`].
#[allow(clippy::too_many_lines)]
pub fn prepare_host_runtime_with_selected_meters_between_render_calls(
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
        true,
        Backend::current(),
        None,
        None,
        lanes,
        None,
        Some(superseded),
    )?;
    Ok((prepared, handles))
}

/// Prepare a host with exactly the caller-selected meter observers and `Concurrent` control
/// delivery (issue #1401 D2): the form a control plane uses while another thread renders.
///
/// Same session and feed as [`prepare_host_runtime_with_selected_meters_between_render_calls`],
/// the same rendered bits, but no fused fader and matrix bank is formed, because a producer may
/// push a record while render runs.
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
        false,
        Backend::current(),
        None,
        None,
        lanes,
        None,
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
        false,
        Backend::current(),
        None,
        Some(base),
        lanes,
        None,
        Some(superseded),
    )?;
    Ok((prepared, handles))
}

#[allow(clippy::too_many_lines)]
fn prepare_host_runtime_with_live_controls_policy(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: Option<&HostLiveControlRequest>,
    between_render_calls: bool,
    backend: Backend,
    successor: Option<SuccessorBase<'_>>,
) -> Result<(PreparedHost, HostLiveControlHandles), PrepareDiagnostics> {
    let (prepared, handles, _) = prepare_host_runtime_with_live_controls_policy_and_spectrum(
        compiled,
        caps,
        live_controls,
        None,
        between_render_calls,
        backend,
        None,
        successor,
        HostLiveLanes::ALL,
        None,
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
        false,
        Backend::current(),
        Some(SpectrumPreparationRequest::Single(request)),
        None,
        HostLiveLanes::ALL,
        None,
        None,
    )?;
    debug_assert!(handles.strip_controls.is_empty() && handles.meters.is_empty());
    let capture = match capture.ok_or_else(|| resource("host.spectrum.capture"))? {
        PreparedSpectrumCapture::Single(capture) => capture,
        PreparedSpectrumCapture::Collection(_) => {
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
            false,
            Backend::current(),
            Some(SpectrumPreparationRequest::Collection(request)),
            None,
            HostLiveLanes::ALL,
            None,
            None,
        )?;
    let capture = match capture.ok_or_else(|| resource("host.spectrum.capture"))? {
        PreparedSpectrumCapture::Collection(capture) => capture,
        PreparedSpectrumCapture::Single(_) => return Err(resource("host.spectrum.capture")),
    };
    Ok((prepared, capture))
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn prepare_host_runtime_with_live_controls_policy_and_spectrum(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    live_controls: Option<&HostLiveControlRequest>,
    selected_meters: Option<&[HostMeterRequest]>,
    between_render_calls: bool,
    backend: Backend,
    spectrum_request: Option<SpectrumPreparationRequest<'_>>,
    successor: Option<SuccessorBase<'_>>,
    lanes: HostLiveLanes,
    bound_cache: Option<&mut InputBoundCache>,
    superseded: Option<&LiveValuesSuperseded>,
) -> Result<
    (
        PreparedHost,
        HostLiveControlHandles,
        Option<PreparedSpectrumCapture>,
    ),
    PrepareDiagnostics,
> {
    // `None` is the plain entry: no live controls, and a request of defaults for the meter and
    // observation fields below. Every `Some` attaches live controls.
    let default_request = HostLiveControlRequest::default();
    let attach_live = live_controls.is_some();
    let live_controls = live_controls.unwrap_or(&default_request);
    // #1277 D10: a successor commits as a revision its own gate can stamp. Checked before this
    // preparation allocates anything.
    if successor.is_some_and(|base| !(1..=GATE_REVISION_MAX).contains(&base.revision)) {
        return Err(successor_revision_refusal());
    }
    // #1312 D2: the session's count when the host passed one, else this preparation's own, which
    // the handles hand back. Made before any observed allocation of the preparation.
    let live_superseded = superseded.cloned().unwrap_or_default();
    let model = compiled.normalized_model();
    check_counts(model, caps)?;
    // The counts the report carries; `check_counts` has proved them inside the caps.
    let track_count = u64::try_from(model.tracks.len()).map_err(|_| platform("host.count"))?;
    let submix_count = u64::try_from(model.submixes.len()).map_err(|_| platform("host.count"))?;
    let vca_count = u64::try_from(model.vcas.len()).map_err(|_| platform("host.count"))?;
    let source_count = u64::try_from(model.sources.len()).map_err(|_| platform("host.count"))?;
    let route_count = u64::try_from(model.routes.len()).map_err(|_| platform("host.count"))?;
    let effect_count = count_effects(model)?;
    caps.validate_shape(compiled)?;

    let source_id_bytes = model.sources.iter().try_fold(0_usize, |total, source| {
        total
            .checked_add(source.id.as_str().len())
            .ok_or_else(|| resource("host.resource.arithmetic"))
    })?;
    let mut graph_sources = Vec::new();
    graph_sources
        .try_reserve_exact(compiled.source_count())
        .map_err(|_| resource("host.resource.allocation"))?;
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
        if caps
            .maximum_source_channels
            .is_some_and(|maximum| u32::from(source.channels) > maximum)
        {
            return Err(shape("host.source.channels"));
        }
        let index = u32::try_from(index).map_err(|_| platform("host.count"))?;
        let ring = PcmSourceRingConfig {
            channel_count: u32::from(source.channels),
            quantum_frames: compiled.quantum(),
            frame_capacity: u64::from(caps.source_ring_frames),
            initial_generation: SourceGeneration(1),
        };
        // Issue #1272 D3: carry when the ID is in both models, the declaration is unchanged and
        // the ring the successor would allocate is the one the predecessor renders from.
        let carried_from = successor.and_then(|base| {
            let row = base.inventory.source(source.id.as_str())?;
            let committed = committed_sources.get(source.id.as_str())?;
            (**committed == *source && row.ring == ring).then_some(row.index)
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
        } else {
            let (producer, consumer, resources) =
                PcmSourceRing::prepare_host_region(ring, SourceFrame(0))
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
    // Issue #1276 D1: the strip input sections that carry, with the predecessor lane each one
    // copies from. Every strip of this plan attaches the same control kind.
    let live_input = attach_live && lanes.strip_input;
    let carried_inputs: Vec<(&str, graph::GraphLaneLocation)> = match successor {
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
            model
                .strips()
                .filter_map(|strip| {
                    let row = base.inventory.input_section(strip.id.as_str())?;
                    let before = committed_input_section(base.committed, &strip)?;
                    (row.live == live_input && same_input_section(before, strip.builtins))
                        .then_some((strip.id.as_str(), row.location))
                })
                .collect()
        }
    };
    // Issue #1277 D3, D4: how each strip's fader/mute and matrix values differ between the
    // predecessor's base and this model: the join's model half, resolved against the plans' rows
    // once this plan is bound.
    let live_strips = attach_live;
    let strip_joins: Vec<StripJoin<'_>> = match successor {
        None => Vec::new(),
        Some(base) => join_strips(
            base.committed,
            model,
            live_strips,
            base.edit_ramps,
            base.monitor,
        ),
    };
    // Issue #1279 D1: how each strip's console slots and effect chain differ between the
    // predecessor's base and this model, resolved against the plans' lanes once this plan is
    // bound.
    let chain_joins: Vec<ChainJoin<'_>> = match successor {
        None => Vec::new(),
        Some(base) => join_chains(base.committed, model),
    };
    // The hand-over the successor runs at its swap block, built here so the plan charge below
    // counts it; installed after bind.
    let carry_program = match successor {
        Some(base)
            if !carried_sources.is_empty()
                || !carried_inputs.is_empty()
                || strip_joins.iter().any(StripJoin::may_carry)
                || chain_joins.iter().any(ChainJoin::may_carry) =>
        {
            Some(graph::GraphCarryProgram {
                predecessor: base.inventory.plan_identity(),
                sources: carried_sources.into_boxed_slice(),
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
    let sources = builder.finish();

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
    let mut effects = match prepare_native_session_effects(
        compiled,
        registry,
        EffectCompileCaps {
            maximum_total_state_bytes: caps.maximum_effect_state_bytes,
            maximum_scratch_bytes: caps.maximum_effect_scratch_bytes,
            maximum_automation_spans_per_block: caps.maximum_automation_spans_per_block,
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
    if effect_state_bytes > caps.maximum_effect_state_bytes
        || effect_scratch_bytes > caps.maximum_effect_scratch_bytes
    {
        return Err(resource("host.effect.resource.limit"));
    }
    #[cfg(feature = "control-provider")]
    let control_catalog = crate::ControlCatalog::build(
        compiled.normalized_model(),
        successor.map_or(&NO_OVERLAY, |base| base.monitor.next),
        compiled.sample_rate().0,
    )
    .map_err(|_| resource("host.control_provider.allocation"))?;

    // Issue #140 A: one live-control lane per prepared strip effect instance (a submix strip's
    // included, #1207 D5): its latest-target cells and, for an effect that declares taps when the
    // host asked for observation capacity, its observation queue (decision 15, D15-2). This is
    // the only thing that creates one; a host that asks for no live controls attaches nothing and
    // the plan renders the byte-identical live-control-free path.
    let mut effect_controls: Vec<EffectControlProducer> = if attach_live && lanes.effects {
        attach_effect_live_controls(
            &mut effects,
            &live_superseded,
            live_controls.observation_taps,
        )
        .map_err(effect_diagnostics)?
    } else {
        Vec::new()
    };
    // Issue #143 D3, level 1. Observation capacity is attached only when it was asked for, and
    // only alongside a control channel: a subscription is a record in the effect's own observation
    // queue, which only the effect's control lane drains, so "observe without live controls" has
    // no delivery path and is a rejection rather than a silent
    // half-attach. The published window is the meter window, derived rather than configured.
    let observation_window_blocks = match (live_controls.meter_period_frames, compiled.quantum().0)
    {
        (Some(period), quantum) if quantum > 0 => (period.get() / quantum).max(1),
        _ => 1,
    };
    let effect_observations: Vec<EffectObservationHandle> = match live_controls.observation_taps {
        0 => Vec::new(),
        // Issue #1254 D2: without effect lanes there is no queue for a subscription to ride.
        taps if !attach_live || !lanes.effects => {
            let _ = taps;
            return Err(shape("host.observation.live_controls"));
        }
        taps => attach_effect_observation(&mut effects, taps, observation_window_blocks)
            .map_err(effect_diagnostics)?,
    };

    // Issue #137 D1/D2: the live-control requests are derived here, once, from the canonical strip
    // order, so `HostLiveControlHandles::strips` and the requested channels cannot disagree. The
    // strip list (issue #1207 D1) appends the submixes after the tracks. Every strip gets a control
    // request (issue #1211 D1). Source semantics: tracks only for the default meters below, so
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
    let live_control_tracks = &live_control_strips[..live_control_track_count];
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
    if selected_meters.is_some_and(|meters| !meters.is_empty())
        && live_controls.meter_period_frames.is_none()
    {
        return Err(shape("host.meter.period"));
    }
    // A selected meter may name any strip (issue #1208 D1); the default set stays one per track.
    let meter_tracks: Vec<(&str, MeterTap, MeterMetricSet)> = match selected_meters {
        Some(meters) => meters
            .iter()
            .map(|meter| (meter.strip_id.as_ref(), meter.tap, meter.metrics))
            .collect(),
        None => live_control_tracks
            .iter()
            .map(|track| (track.as_ref(), live_controls.meter_tap, MeterMetricSet::ALL))
            .collect(),
    };
    let meter_requests: Vec<SelectedMeterRequest> = match live_controls.meter_period_frames {
        None => Vec::new(),
        Some(period) => meter_tracks
            .iter()
            .enumerate()
            .map(|(index, (track, tap, metrics))| {
                // Handles are `index + 1`: nonzero, stable in request order (canonical track order
                // for the default set); nothing outside this function invents a meter handle.
                let handle = u64::try_from(index)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .and_then(core::num::NonZeroU64::new)
                    .ok_or_else(|| platform("host.count"))?;
                Ok(SelectedMeterRequest {
                    request: MeterRequest {
                        handle: MeterHandle(handle),
                        track_id: track.to_string(),
                        tap: *tap,
                        config: MeterConfig {
                            period_frames: period,
                            peak_hold_frames: 0,
                            peak_decay_db_per_second: 0.0,
                            queue_capacity: live_controls.meter_queue_depth,
                            reset_generation: 0,
                        },
                    },
                    metrics: *metrics,
                })
            })
            .collect::<Result<Vec<_>, PrepareDiagnostics>>()?,
    };
    let builtin_caps = BuiltinCompileCaps {
        maximum_total_state_bytes: caps.maximum_builtin_retained_bytes,
        maximum_total_retained_payload_bytes: caps.maximum_builtin_retained_bytes,
        maximum_total_meter_items: caps.maximum_meter_items,
        maximum_total_meter_bytes: caps.maximum_meter_bytes,
        maximum_single_allocation_bytes: caps.maximum_named_allocation_bytes,
        maximum_meter_streams: caps.maximum_meter_streams,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    };
    // Decision 17, C7: every effective value is computed once per preparation, here, under the
    // overlay the successor renders with (#1559 D9); a first plan has no solo.
    let overlay = successor.map_or(&NO_OVERLAY, |base| base.monitor.next);
    let effective = controls::effective(compiled.normalized_model(), overlay);
    let builtins = if selected_meters.is_some() {
        // #1401 D2: a selected-meter request follows the caller's delivery contract, like every
        // other request. The serialized form keeps its fused fader and matrix banks.
        if between_render_calls {
            prepare_selected_session_builtins_between_render_calls(
                compiled,
                &effective,
                &meter_requests,
                &control_requests,
                builtin_caps,
            )
        } else {
            prepare_selected_session_builtins_with_live_controls(
                compiled,
                &effective,
                &meter_requests,
                &control_requests,
                builtin_caps,
            )
        }
    } else if between_render_calls {
        prepare_session_builtins_between_render_calls(
            compiled,
            &effective,
            &meter_requests
                .iter()
                .map(|item| item.request.clone())
                .collect::<Vec<_>>(),
            &control_requests,
            builtin_caps,
        )
    } else {
        prepare_session_builtins_with_live_controls(
            compiled,
            &effective,
            &meter_requests
                .iter()
                .map(|item| item.request.clone())
                .collect::<Vec<_>>(),
            &control_requests,
            builtin_caps,
            bound_cache,
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
    let mut artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: backend,
        plan_id: 1,
        effects,
        builtins,
        caps: GraphCompileCaps {
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
        },
    })
    .map_err(|failure| {
        PrepareDiagnostics::new(
            PrepareRejection::Graph,
            diagnostic_lines(
                failure
                    .diagnostics
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| (diagnostic.code, &diagnostic.path)),
            ),
        )
    })?;
    // Issue #1221 D1: one live lane per route into a submix, attached before bind and before the
    // caps below, so its charge is refused by the same checks as the effect channels. Without a
    // control channel nothing is attached and the plan binds exactly what it bound before.
    let graph_route_controls = if attach_live && lanes.routes {
        // A route's lane is a latest-target cell with no depth (#1347).
        artifact
            .attach_route_live_controls(&live_superseded)
            .map_err(|error| match error {
                graph::GraphRouteControlError::AlreadyAttached
                | graph::GraphRouteControlError::BoundRoute => {
                    graph_failure("host.route_control.attach")
                }
            })?
    } else {
        Vec::new()
    };
    let route_control_resources = graph::route_control_resources(&graph_route_controls);
    let output_latency = artifact.report().output_latency;
    let output_tail = artifact.report().output_tail;
    let mut graph_resources = artifact.graph_resource_estimate().clone();
    // The plan's own allocations that the graph row does not count: its revision gate (#1502 D1)
    // and, when this preparation made its own `live_values_superseded` count rather than taking the
    // session's, and strips and routes hold a clone of it in their cell readers, that count (#1312
    // D2; a session's count is the host's to charge once). A preparation with no strip controls
    // keeps no reader of the count (a route lane exists only beside strips), so the plan retains
    // none and charges none. Charged here so every host, the C ABI and the browser, counts them in
    // the one row.
    graph_resources.session_plus_plan_bytes = graph_resources
        .session_plus_plan_bytes
        .checked_add(engine::realtime::plan_revision_gate_retained_bytes())
        .and_then(|bytes| {
            bytes.checked_add(if superseded.is_none() && !control_requests.is_empty() {
                LiveValuesSuperseded::retained_bytes()
            } else {
                0
            })
        })
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let session_resources = compiled.resource_estimate();
    let spectrum_resources: Option<SpectrumCaptureResources> = match spectrum_request {
        None => None,
        Some(SpectrumPreparationRequest::Single(request)) => Some(
            crate::spectrum::spectrum_capture_resources_for(&request.target),
        ),
        Some(SpectrumPreparationRequest::Collection(request)) => Some(
            spectrum_capture_collection_resources(&request.entries)
                .map_err(spectrum_diagnostics)?,
        ),
    };
    let spectrum_capture_retained_bytes =
        spectrum_resources.map_or(0, |resources| resources.retained_bytes);
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
    let source_set = prepare_graph_source_set(artifact.envelope(), graph_sources, mappings)
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
        Some(SpectrumPreparationRequest::Single(request)) => {
            let (observer, capture) = crate::spectrum::prepare_capture(
                request,
                graph_nodes.as_deref().expect("spectrum graph nodes"),
                caps.maximum_named_allocation_bytes,
            )
            .map_err(spectrum_diagnostics)?;
            (
                vec![observer],
                Some(PreparedSpectrumCapture::Single(capture)),
            )
        }
        Some(SpectrumPreparationRequest::Collection(request)) => {
            let (observers, capture) = prepare_capture_collection(
                request,
                graph_nodes.as_deref().expect("spectrum graph nodes"),
                caps.maximum_named_allocation_bytes,
            )
            .map_err(spectrum_diagnostics)?;
            (
                observers,
                Some(PreparedSpectrumCapture::Collection(capture)),
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

    // Issue #143 D6, widened to strips by issue #1208 D3: a designated master must name a strip
    // this session actually has, or the frame would report a master reading nobody can address.
    let master_track = match live_controls.master_track {
        Some(index) if (index as usize) < live_control_strips.len() => Some(index),
        Some(_) => return Err(shape("host.observation.master_track")),
        None => None,
    };

    // Issue #143 R7: walked over the built runtime, not derived from the request. A plan that
    // bound nothing reports zero because it *holds* nothing.
    let observation_retained_bytes = bound.plan.observation_retained_bytes();
    let control_retained_bytes = sources
        .retained_bytes()
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let mut plan = bound.plan;
    // Issue #1272 D4: the join's program goes in before the plan leaves preparation.
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
    if !carried_inputs.is_empty() {
        let moves: Vec<graph::GraphLaneMove> = carried_inputs
            .iter()
            .filter_map(|(id, predecessor)| {
                input_section_row(&input_sections, id).map(|row| graph::GraphLaneMove {
                    successor: row.location,
                    predecessor: *predecessor,
                })
            })
            .collect();
        // Every strip banks its input section in a shipped plan, so every carried strip finds
        // its lane here; one that did not would silently restart at rest.
        debug_assert_eq!(moves.len(), carried_inputs.len());
        graph::install_builtin_input_carry(&mut plan, moves)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
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
    let strip_carry = match successor {
        Some(base) => resolve_strip_carry(base.inventory, &strip_joins, &strip_stages, live_strips),
        None => StripCarry::default(),
    };
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
        })
        .collect();
    effect_lanes.sort_unstable_by(|left, right| {
        left.id
            .cmp(&right.id)
            .then(left.rack.cmp(&right.rack))
            .then_with(|| left.effect_id.cmp(&right.effect_id))
    });
    let effect_carry = match successor {
        Some(base) => resolve_effect_carry(base.inventory, &chain_joins, &effect_lanes),
        None => EffectCarry::default(),
    };
    // The effect moves' table and payload scratch are charged at their exact size, against the
    // same cap as the rest of the plan, and the scratch is one allocation (#1279 D3).
    let effect_carry_bytes =
        graph::effect_carry_retained_bytes(effect_carry.moves.len(), effect_carry.scratch_bytes)
            .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let effect_carry_scratch_bytes = effect_carry.scratch_bytes;
    if effect_carry_scratch_bytes > caps.maximum_named_allocation_bytes {
        return Err(resource("host.resource.limit"));
    }
    if admitted_graph_and_model
        .checked_add(strip_carry_bytes)
        .and_then(|bytes| bytes.checked_add(effect_carry_bytes))
        .ok_or_else(|| resource("host.resource.arithmetic"))?
        > caps.maximum_graph_session_plus_plan_bytes
    {
        return Err(resource("host.graph.resource.limit"));
    }
    if !strip_carry.moves.is_empty() {
        graph::install_builtin_strip_carry(&mut plan, strip_carry.moves)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
    if !effect_carry.moves.is_empty() {
        graph::install_effect_carry(&mut plan, effect_carry.moves)
            .map_err(|_| graph_failure("host.carry.install"))?;
    }
    let carry_program_retained_bytes = graph::carry_program_retained_bytes(&mut plan);
    debug_assert_eq!(
        carry_program_retained_bytes,
        carry_program_bytes + strip_carry_bytes + effect_carry_bytes
    );
    inventory_sources.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    let mut restarted = strip_carry.restarted;
    restarted.extend(effect_carry.restarted);
    restarted.sort_unstable();
    restarted.dedup();
    let inventory = PlanStateInventory {
        plan_identity: graph::plan_identity(&mut plan)
            .ok_or_else(|| graph_failure("host.inventory.identity"))?,
        sources: inventory_sources.into_boxed_slice(),
        input_sections: input_sections.into_boxed_slice(),
        strip_stages: strip_stages.into_boxed_slice(),
        effect_lanes: effect_lanes.into_boxed_slice(),
        restarted: restarted.into_boxed_slice(),
    };
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
        output_tail,
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
        inventory_retained_bytes: inventory.retained_bytes(),
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
    if let (Some(base), false) = (successor, strip_carry.retargets.is_empty()) {
        let Some(Ok(stamp)) = plan.stamp_held_gate(base.revision) else {
            return Err(successor_revision_refusal());
        };
        for retarget in &strip_carry.retargets {
            let Some(&index) = canonical_index.get(retarget.id) else {
                return Err(graph_failure("host.control.order"));
            };
            let producer = &mut strip_controls[index];
            for record in retarget.fader.iter().map_while(|record| *record) {
                producer.fader.write(stamp, record);
            }
            if let Some(record) = retarget.matrix {
                producer.producer.write(stamp, record);
            }
        }
    }

    Ok((
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
            route_controls: graph_route_controls
                .into_iter()
                .map(RouteControlProducer::new)
                .collect(),
            master_track,
            live_values_superseded: live_superseded,
        },
        spectrum_capture,
    ))
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
/// filter cutoffs, per channel.
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

/// The per-edit ramp lengths of one strip's four record rows, looked up once from the
/// transaction's [`EditRamps`] (#1394 D6): the `(Strip, Fader)`, `(Strip, Mute)`,
/// `(Strip, Matrix)` and `(Strip, Input)` entries. `None` is "the edit gave no length": the session's applies.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct StripEditRamps {
    fader: Option<u32>,
    mute: Option<u32>,
    matrix: Option<u32>,
    /// The input trim and polarity (#1556 D3).
    pub(crate) input: Option<u32>,
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
        }
    }
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
/// # Errors
///
/// [`OutOfDomain`] when either fader is outside the domain [`checked_fader_gain`] accepts.
pub(crate) fn fader_records(
    before: &session::DualMonoFader,
    after: &session::DualMonoFader,
    lengths: LevelLengths,
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
    let gain_changed = [0, 1].map(|lane| gains_before[lane] != gains_after[lane]);
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

/// A strip's effective fader and mute as a [`session::DualMonoFader`], the shape its stage's
/// records are derived from.
/// Whether a strip's effective fader or mute differs, as bits: the one test of "this strip's level
/// moved", for the dense diff and the local one.
#[cfg(feature = "control-provider")]
pub(crate) fn level_differs(
    before: &controls::EffectiveStrip,
    after: &controls::EffectiveStrip,
) -> bool {
    before.fader_db.map(f32::to_bits) != after.fader_db.map(f32::to_bits)
        || before.mute != after.mute
}

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
#[derive(Clone, Copy)]
pub(crate) struct LevelRules<'a> {
    pub(crate) current: &'a SessionModel,
    /// The model after the change, read through a view so that a transaction of values need not
    /// build it (#1613).
    pub(crate) next: controls::ModelView<'a>,
    pub(crate) ramps: LiveRamps,
    pub(crate) edit_ramps: &'a EditRamps,
    /// The ramp length of a mute record that only the overlay change produced (#1559 D7).
    pub(crate) solo_ramp: Option<u32>,
}

impl LevelRules<'_> {
    /// The ramp length of each lane's `FaderDb` and `Mute` record of one strip (#1558 D7).
    ///
    /// A lane's fader length is that of the strip's own `fader_db` edit when the strip's own value
    /// on the lane changed (row `Fader`, the edit's length); else that of the first reaching VCA,
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
            if db(own_before).to_bits() != db(own_after).to_bits() {
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
        }
    }

    #[cfg(feature = "control-provider")]
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
    /// `monitor.next` (decision 17, C7; #1559 D6).
    pub(crate) fn new(
        current: &'a SessionModel,
        next: &'a SessionModel,
        ramps: LiveRamps,
        edit_ramps: &'a EditRamps,
        monitor: MonitorBase<'_>,
    ) -> Self {
        Self {
            rules: LevelRules {
                current,
                next: controls::ModelView::whole(next),
                ramps,
                edit_ramps,
                solo_ramp: monitor.ramp,
            },
            before: controls::effective(current, monitor.committed),
            after: controls::effective(next, monitor.next),
            reach: next.vca_reach(),
            reach_before: current.vca_reach(),
            solo_before: solo_terms(current, monitor.committed),
            solo_after: solo_terms(next, monitor.next),
        }
    }

    /// What the ramp rules read about the strip at `before` in `current` and `after` in `next`.
    fn change(&self, before: usize, after: usize) -> StripChange<'_> {
        let after_strip = self.rules.next.strip(after);
        StripChange {
            own_before: strip_at(self.rules.current, before).fader,
            own_after: after_strip.fader,
            id: after_strip.id,
            reach_before: &self.reach_before[before],
            reach: &self.reach[after],
            solo_before: self.solo_before[before],
            solo_after: self.solo_after[after],
        }
    }

    #[cfg(feature = "control-provider")]
    /// Whether the strip's effective fader or mute differs, as bits, between the two models.
    pub(crate) fn level_changed(&self, before: usize, after: usize) -> bool {
        level_differs(&self.before.strips[before], &self.after.strips[after])
    }

    /// The level records of one strip: its effective values in the two models, with the lengths
    /// the ramp rules give.
    ///
    /// # Errors
    ///
    /// [`OutOfDomain`] as [`fader_records`].
    pub(crate) fn level_records(
        &self,
        before: usize,
        after: usize,
    ) -> Result<[Option<TrackFaderRecord>; 4], OutOfDomain> {
        fader_records(
            &effective_fader(&self.before.strips[before]),
            &effective_fader(&self.after.strips[after]),
            self.rules.lengths(&self.change(before, after)),
        )
    }

    #[cfg(feature = "control-provider")]
    /// The effective gate of route `index`, per source lane, in `[current, next]`.
    pub(crate) fn gates(&self, index: usize) -> [[bool; 2]; 2] {
        [
            self.before.routes[index].gate,
            self.after.routes[index].gate,
        ]
    }

    #[cfg(feature = "control-provider")]
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

    #[cfg(feature = "control-provider")]
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

/// The index in [`SessionModel::strips`] of the strip `route` takes its source from, or `None` if
/// the model has no such strip.
pub(crate) fn route_source_strip(model: &SessionModel, route: &session::Route) -> Option<usize> {
    let id = match &route.source {
        session::RouteSource::Track { track_id, .. } => track_id,
        session::RouteSource::Submix { submix_id, .. } => submix_id,
    };
    controls::strip_index(model, id)
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

/// The refusal of a successor whose revision its gate cannot stamp (#1277 D10).
fn successor_revision_refusal() -> PrepareDiagnostics {
    PrepareDiagnostics::fixed(PrepareRejection::Session, "host.successor.revision")
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
    fader: FamilyJoin<[Option<TrackFaderRecord>; 4]>,
    matrix: FamilyJoin<TrackControlRecord>,
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

/// Whether `committed` has a strip of any kind named `id`.
fn committed_has_strip(committed: &SessionModel, id: &str) -> bool {
    committed
        .tracks
        .binary_search_by(|track| track.id.as_str().cmp(id))
        .is_ok()
        || committed
            .submixes
            .binary_search_by(|submix| submix.id.as_str().cmp(id))
            .is_ok()
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
    monitor: MonitorBase<'a>,
) -> Vec<StripJoin<'a>> {
    let ramps = LiveRamps::for_session(model);
    let diff = EffectiveDiff::new(committed, model, ramps, edit_ramps, monitor);
    let mut joins = Vec::new();
    for (index, strip) in model.strips().enumerate() {
        let id = strip.id.as_str();
        let Some((position, _, before_matrix)) = committed_strip(committed, &strip) else {
            if committed_has_strip(committed, id) {
                joins.push(StripJoin {
                    id,
                    fader: FamilyJoin::Prepared,
                    matrix: FamilyJoin::Prepared,
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
        let fader_records = diff.level_records(position, index);
        let matrix_record = matrix_record(before_matrix, strip.matrix_or_pan, ramps, edit);
        let fader_live = live && fader_records.is_ok();
        let matrix_live = live && matrix_record.is_ok();
        let fader = match fader_records {
            Err(_) => FamilyJoin::Prepared,
            Ok(records) if records[0].is_none() => FamilyJoin::Same,
            Ok(_) if !fader_live => FamilyJoin::Prepared,
            Ok(records) => FamilyJoin::Live(records),
        };
        let matrix = match matrix_record {
            Err(_) => FamilyJoin::Prepared,
            Ok(None) => FamilyJoin::Same,
            Ok(Some(_)) if !matrix_live => FamilyJoin::Prepared,
            Ok(Some(record)) => FamilyJoin::Live(record),
        };
        joins.push(StripJoin { id, fader, matrix });
    }
    joins
}

/// One carried strip's retarget records, to write into the successor's cells (#1277 D5).
struct StripRetarget<'a> {
    id: &'a str,
    fader: [Option<TrackFaderRecord>; 4],
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
/// kind, and no prepared value of it differs. A fused bank holds both owners in one stage, so it
/// carries both or neither. A strip with a stage that does not carry starts that stage at rest
/// and is listed in `restarted`.
fn resolve_strip_carry<'a>(
    predecessor: &PlanStateInventory,
    joins: &[StripJoin<'a>],
    own: &[StripStageInventoryRow],
    live: bool,
) -> StripCarry<'a> {
    use graph::BuiltinStripOwner::{Fader, Matrix};
    let mut carry = StripCarry::default();
    for join in joins {
        let rows = [Fader, Matrix].map(|owner| {
            let own = strip_stage_row(own, join.id, owner)?;
            let before = predecessor.strip_stage(join.id, owner)?;
            (own.stage == before.stage && before.live == live).then_some((own, before))
        });
        let families = [!join.fader.is_prepared(), !join.matrix.is_prepared()];
        let mut carries = [0, 1].map(|owner| rows[owner].is_some() && families[owner]);
        if rows
            .iter()
            .flatten()
            .any(|(row, _)| row.stage == graph::BuiltinStripStage::FaderMatrix)
        {
            let both = carries[0] && carries[1];
            carries = [both, both];
        }
        for ((owner, row), carries) in [Fader, Matrix].into_iter().zip(rows).zip(carries) {
            if let (true, Some((own, before))) = (carries, row) {
                carry.moves.push(graph::GraphStripMove {
                    owner,
                    successor: own.location,
                    predecessor: before.location,
                });
            }
        }
        if !(carries[0] && carries[1]) {
            carry.restarted.push(join.id.into());
        }
        let fader = match join.fader {
            FamilyJoin::Live(records) if carries[0] => records,
            _ => [None; 4],
        };
        let matrix = match join.matrix {
            FamilyJoin::Live(record) if carries[1] => Some(record),
            _ => None,
        };
        if fader[0].is_some() || matrix.is_some() {
            carry.retargets.push(StripRetarget {
                id: join.id,
                fader,
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

/// One console slot of a strip present in both models, and whether the successor's model leaves
/// every prepared value of it as the predecessor's base holds it (#1279 D1).
struct ConsoleOwnerJoin<'a> {
    /// `Simd1` for a `pre_insert` slot, `Simd2` for a `post_insert` one.
    rack: graph::RackId,
    /// The slot ID, which is the effect ID of the owner key.
    slot: &'a str,
    /// The slot's identity, quality and link mode (session-level), and this strip's bypass and
    /// parameter values for it, are bit-equal in both models, and the slot sits in the same
    /// section. An effect bank stage without a live control lane has no live value, so every
    /// difference is prepared (D2).
    same: bool,
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
    /// The IDs of the inserts the strip has in both models (the surviving inserts), in the
    /// successor's chain order. An insert's owner key is the strip, [`graph::RackId::Dynamic`]
    /// and this ID.
    surviving_inserts: Vec<&'a str>,
}

impl ChainJoin<'_> {
    /// Whether any console slot of the strip could carry.
    fn may_carry(&self) -> bool {
        self.owners.iter().any(|owner| owner.same)
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

/// Whether two strips' knobs for one console slot are bit-equal: bypass and every parameter.
fn same_console_entry(left: &session::ConsoleEntry, right: &session::ConsoleEntry) -> bool {
    left.bypass == right.bypass
        && left.params.len() == right.params.len()
        && left.params.iter().zip(&right.params).all(|(left, right)| {
            left.parameter_id == right.parameter_id
                && left.channel == right.channel
                && left.unit == right.unit
                && left.value.to_bits() == right.value.to_bits()
        })
}

/// The console entries and inserts `strip` had in `committed`, among strips of its own kind
/// (#1279 D1). A strip whose kind changed is not found.
fn committed_chain<'a>(
    committed: &'a SessionModel,
    strip: &StripRef<'_>,
) -> Option<(&'a [session::ConsoleEntry], &'a [session::Effect])> {
    let id = strip.id.as_str();
    match strip.kind {
        StripKind::Track(_) => committed
            .tracks
            .binary_search_by(|track| track.id.as_str().cmp(id))
            .ok()
            .map(|index| {
                let track = &committed.tracks[index];
                (&*track.console, &*track.inserts.effects)
            }),
        StripKind::Submix(_) => committed
            .submixes
            .binary_search_by(|submix| submix.id.as_str().cmp(id))
            .ok()
            .map(|index| {
                let submix = &committed.submixes[index];
                (&*submix.console, &*submix.inserts.effects)
            }),
    }
}

/// The model half of the console-effect join (#1279 D1): for every strip of `model` that
/// `committed` also has, whether its effect chain changed and which console slots keep every
/// prepared value.
///
/// An insert added, removed or reordered changes the chain, and an insert present in both models
/// is recorded as a surviving insert, an owner whose lane the lane half resolves (it carries with
/// #1281 and #1282). A strip whose kind changed has no counterpart:
/// its chain is changed and no slot carries.
fn join_chains<'a>(committed: &SessionModel, model: &'a SessionModel) -> Vec<ChainJoin<'a>> {
    let committed_slots = console_slots(committed);
    let slots = console_slots(model);
    let console_changed = committed_slots.len() != slots.len()
        || committed_slots
            .iter()
            .zip(&slots)
            .any(|((before_rack, before), (rack, slot))| {
                before_rack != rack || before.slot != slot.slot
            });
    let mut joins = Vec::new();
    for strip in model.strips() {
        let id = strip.id.as_str();
        let Some((before_console, before_inserts)) = committed_chain(committed, &strip) else {
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
        let inserts_changed = before_inserts.len() != inserts.len()
            || before_inserts.iter().zip(inserts).any(|(before, insert)| {
                before.id != insert.id || before.identity != insert.identity
            });
        let owners = slots
            .iter()
            .zip(strip.console)
            .map(|((rack, slot), entry)| {
                let same = committed_slots
                    .iter()
                    .position(|(_, before)| before.slot == slot.slot)
                    .and_then(|index| {
                        let (before_rack, before_slot) = committed_slots[index];
                        let before_entry = before_console.get(index)?;
                        Some(
                            before_rack == *rack
                                && before_slot == *slot
                                && same_console_entry(before_entry, entry),
                        )
                    })
                    .unwrap_or(false);
                ConsoleOwnerJoin {
                    rack: *rack,
                    slot: slot.slot.as_str(),
                    same,
                }
            })
            .collect();
        joins.push(ChainJoin {
            id,
            chain_changed: console_changed || inserts_changed,
            owners,
            surviving_inserts: inserts
                .iter()
                .filter(|insert| before_inserts.iter().any(|before| before.id == insert.id))
                .map(|insert| insert.id.as_str())
                .collect(),
        });
    }
    joins
}

/// What the console-effect join resolved against both plans' lanes.
#[derive(Default)]
struct EffectCarry {
    moves: Vec<graph::GraphEffectMove>,
    restarted: Vec<Box<str>>,
    /// The largest payload of a carried lane: the scratch every move goes through.
    scratch_bytes: u64,
}

/// The lane half of the console-effect join (#1279 D1, D2): which console slot of which strip
/// moves, and which strips restart.
///
/// A slot carries when no prepared value of it differs and both plans hold it as a lane of an
/// effect bank stage with no live control lane, with equal layouts (prepared bypass, latency, state
/// sizes and state layout version). A lane of a stage with a live control lane or a lowered bypass
/// has no row: it does not carry here (#1280) and its strip restarts. A surviving insert carries
/// when its owner key (strip, [`graph::RackId::Dynamic`], insert ID) is among the keys carried:
/// no insert lane has a carry until #1281 and #1282, which add their keys to `carried` when their
/// move covers the lane (root amendment, 2026-10-12). A strip with a slot or a surviving insert
/// that does not carry, or whose chain changed, is listed in `restarted`.
fn resolve_effect_carry(
    predecessor: &PlanStateInventory,
    joins: &[ChainJoin<'_>],
    own: &[EffectLaneInventoryRow],
) -> EffectCarry {
    let mut carry = EffectCarry::default();
    for join in joins {
        let mut restart = join.chain_changed;
        // The owner keys of this strip that carry: (rack, effect ID).
        let mut carried: Vec<(graph::RackId, &str)> = Vec::new();
        for owner in &join.owners {
            let rows = effect_lane_row(own, join.id, owner.rack, owner.slot)
                .zip(predecessor.effect_lane(join.id, owner.rack, owner.slot));
            match rows {
                Some((own, before)) if owner.same && own.layout == before.layout => {
                    carry.scratch_bytes = carry.scratch_bytes.max(own.layout.payload_bytes());
                    carry.moves.push(graph::GraphEffectMove {
                        successor: own.location,
                        predecessor: before.location,
                    });
                    carried.push((owner.rack, owner.slot));
                }
                _ => restart = true,
            }
        }
        restart |= join
            .surviving_inserts
            .iter()
            .any(|insert| !carried.contains(&(graph::RackId::Dynamic, insert)));
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
            location: graph::GraphLaneLocation {
                unit: 0,
                slot: 0,
                lane,
            },
            layout,
        }
    }

    fn inventory_with(row: super::EffectLaneInventoryRow) -> super::PlanStateInventory {
        super::PlanStateInventory {
            plan_identity: 1,
            sources: Box::default(),
            input_sections: Box::default(),
            strip_stages: Box::default(),
            effect_lanes: Box::new([row]),
            restarted: Box::default(),
        }
    }

    /// D1: the slot's model values are equal and both plans bank it, but the predecessor's lane has
    /// another prepared layout (state sizes, state layout version, prepared bypass or latency).
    /// The lane is not moved and its strip restarts. A join that trusts the existence of two rows
    /// moves a payload the successor's bank would refuse, or read at another size.
    #[test]
    fn a_lane_of_another_prepared_layout_is_not_moved() {
        let join = |same| super::ChainJoin {
            id: "eq0",
            chain_changed: false,
            surviving_inserts: Vec::new(),
            owners: vec![super::ConsoleOwnerJoin {
                rack: graph::RackId::Simd1,
                slot: SLOT,
                same,
            }],
        };
        let own = [lane_row(3, layout())];
        let carry = super::resolve_effect_carry(
            &inventory_with(lane_row(1, layout())),
            &[join(true)],
            &own,
        );
        assert_eq!(carry.moves.len(), 1);
        assert_eq!(carry.moves[0].successor.lane, 3);
        assert_eq!(carry.moves[0].predecessor.lane, 1);
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
        );
        assert!(carry.moves.is_empty());
        assert_eq!(carry.restarted, vec![Box::<str>::from("eq0")]);
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

    /// #1457 gate 3 (native): a design-bound cache changes no prepared value. The session's first
    /// three strips carry distinct near-top designs whose charges cross the preparation budget, so
    /// the third reports the live bound; the report (the output tail included) is the same with
    /// no cache, a cold cache and the same cache warm, and the cold preparation fills the cache, so
    /// the policy function hands its cache to `builtins-compiler` rather than dropping it.
    #[test]
    fn a_design_bound_cache_changes_no_prepared_value() {
        use super::{
            Backend, HostLiveLanes, HostPrepareCaps, HostShapePolicy, InputBoundCache,
            compile_host_model, parse_host_session,
            prepare_host_runtime_with_live_controls_policy_and_spectrum,
        };
        use effect_contract::TailSamples;

        let caps = HostPrepareCaps {
            shape: HostShapePolicy::AnyLaunchRate,
            source_ring_frames: 256,
            maximum_source_channels: None,
            maximum_automation_spans_per_block: 4096,
            maximum_tracks: u64::MAX,
            maximum_submixes: u64::MAX,
            maximum_vcas: u64::MAX,
            maximum_vca_reach_pairs: u64::MAX,
            maximum_sources: u64::MAX,
            maximum_routes: u64::MAX,
            maximum_effects: u64::MAX,
            maximum_graph_session_plus_plan_bytes: u64::MAX,
            maximum_source_total_bytes: u64::MAX,
            maximum_source_overhead_bytes: u64::MAX,
            maximum_effect_state_bytes: u64::MAX,
            maximum_effect_scratch_bytes: u64::MAX,
            maximum_builtin_retained_bytes: u64::MAX,
            maximum_named_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_meter_items: u64::MAX,
            maximum_meter_bytes: u64::MAX,
        };
        let mut model = parse_host_session(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("EQ fixture");
        // No effect, so the output tail is the strips' own (the EQ states no finite tail yet).
        model.console.pre_insert.clear();
        model.console.post_insert.clear();
        for submix in &mut model.submixes {
            submix.console.clear();
            submix.inserts.effects.clear();
        }
        let rate = model.sample_rate_hz;
        let maximum = builtins::builtin_filter_cutoff_maximum_hz(rate).expect("launch rate");
        let below = |steps: u32| f32::from_bits(maximum.to_bits() - steps);
        let designs = [
            (below(1), maximum),
            (below(2), maximum),
            (below(3), maximum),
        ];
        for (index, track) in model.tracks.iter_mut().enumerate() {
            track.console.clear();
            track.inserts.effects.clear();
            if let Some(&(hpf, lpf)) = designs.get(index) {
                for channel in [&mut track.builtins.left, &mut track.builtins.right] {
                    channel.hpf_hz = hpf;
                    channel.lpf_hz = lpf;
                    channel.trim_db = 24.0;
                }
            }
        }
        // The premise: the first two designs fit the budget, the three together do not, and each
        // design's own tail is below the live bound's, so only the fallback reaches it.
        let charged: Vec<builtins::ChargedInputBound> = designs
            .iter()
            .map(|&(hpf, lpf)| {
                let channel = builtins::ChannelParameters {
                    trim_db: 24.0,
                    hpf_hz: hpf,
                    lpf_hz: lpf,
                    ..builtins::ChannelParameters::default()
                };
                let parameters = builtins::BuiltinParameters {
                    left: channel,
                    right: channel,
                    ..builtins::BuiltinParameters::default()
                };
                builtins::input_section_bound_charged(rate, parameters).expect("bound")
            })
            .collect();
        let fits = charged[0].charge + charged[1].charge;
        assert!(fits <= builtins::INPUT_BOUND_BUDGET_FRAMES);
        assert!(fits + charged[2].charge > builtins::INPUT_BOUND_BUDGET_FRAMES);
        let compiled = compile_host_model(
            &model,
            caps.compile_caps(model.sources.len()).expect("caps"),
        )
        .expect("compile");
        let prepare = |cache: Option<&mut InputBoundCache>| {
            prepare_host_runtime_with_live_controls_policy_and_spectrum(
                &compiled,
                &caps,
                None,
                None,
                false,
                Backend::current(),
                None,
                None,
                HostLiveLanes::ALL,
                cache,
                None,
            )
            .expect("prepare")
            .0
            .report
        };
        let none = prepare(None);
        // The third design's live bound reaches the report.
        let live = builtins::input_section_live_bound_table(rate).expect("launch rate");
        let (TailSamples::Finite(output), TailSamples::Finite(live)) =
            (none.output_tail, live.tail)
        else {
            panic!("finite tails: {:?}", none.output_tail);
        };
        assert!(output >= live, "{output} < {live}");
        for design in &charged {
            assert!(
                matches!(design.bound.tail, TailSamples::Finite(tail) if tail < live),
                "{design:?}"
            );
        }
        let mut cache = InputBoundCache::new();
        assert_eq!(prepare(Some(&mut cache)), none, "cold cache");
        assert!(!cache.is_empty());
        assert_eq!(prepare(Some(&mut cache)), none, "warm cache");
    }
}
