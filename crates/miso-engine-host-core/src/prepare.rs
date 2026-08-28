//! The shared host preparation pipeline: TOML in, a prepared plan and a source control set out.
//!
//! This is the ~300 lines that the C ABI host and the browser host each carried a private copy of.
//! Everything that differed between the two copies is a field of [`HostPrepareCaps`]; everything
//! else was identical, down to the order of the checks.

use core::num::{NonZeroU32, NonZeroUsize};
use std::collections::BTreeSet;

use miso_engine_builtins::{MeterConfig, MeterHandle, MeterTap};
use miso_engine_builtins_compiler::{
    BuiltinCompileCaps, MeterConsumer, MeterRequest, TrackControlProducer, TrackControlRequest,
    prepare_session_builtins_with_console, session_structural_symmetry,
};
use miso_engine_core::{SampleRateHz, realtime::PreparedRenderPlan};
use miso_engine_effect_compiler::{
    EffectCompileCaps, EffectControlProducer, EffectObservationHandle, attach_effect_console,
    attach_effect_observation, launch_native_effect_registry, prepare_native_session_effects,
};
use miso_engine_effect_contract::TailSamples;
use miso_engine_graph::{
    GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, StableGraphId,
    TrackStage,
};
use miso_engine_graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_session::{
    CompileCaps, CompiledSession, SessionToml, compile_session, parse_session_toml,
};
use miso_engine_source::{
    PcmSourceRing, PcmSourceRingConfig, SourceFrame, SourceGeneration, SourceGraphSource,
    SourceGraphTrackMapping, prepare_graph_source_set,
};

use crate::diagnostics::{PrepareDiagnostics, PrepareRejection, diagnostic_lines};
use crate::source::{ControlSourceBuilder, SourceControlSet};

/// The launch sample-rate set (issue 032). A host that does not pin one rate accepts these four.
pub const LAUNCH_SAMPLE_RATES_HZ: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// How strictly a host constrains the session's external render shape.
///
/// The C ABI host compiles whatever rate the session declares, as long as it is a launch rate; the
/// browser host is handed a fixed `AudioContext` rate and quantum and must reject anything else.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostShapePolicy {
    /// Accept any rate in [`LAUNCH_SAMPLE_RATES_HZ`], and whatever quantum the session declares.
    AnyLaunchRate,
    /// Accept exactly this rate and quantum, and reject every other session.
    Exact {
        /// Required session sample rate.
        sample_rate_hz: u32,
        /// Required session quantum in frames.
        quantum_frames: u32,
    },
}

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
            HostShapePolicy::AnyLaunchRate => {
                if !LAUNCH_SAMPLE_RATES_HZ.contains(&compiled.sample_rate().0) {
                    return Err(shape("host.sample_rate.unsupported"));
                }
            }
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
    /// Engine-owned bytes the compiled plan's observation lanes and slots retain (issue #143 R7).
    ///
    /// Exactly zero for a session that named no observation capacity, and that zero is *walked*
    /// over the built runtime rather than computed from the request.
    pub observation_retained_bytes: u64,
    /// Total source ID text bytes.
    pub source_id_bytes: u64,
    /// Largest single engine-owned allocation: the maximum over the graph, source and builtin
    /// reports. It deliberately excludes the compiled session model and any host-private row, so a
    /// host folds its own rows in without double-counting.
    pub largest_engine_allocation_bytes: u64,
}

/// What a live console asks preparation to attach (issue #137 D1/D2).
///
/// Both halves are optional and independent, and the default attaches neither, so
/// [`prepare_host_runtime`] is exactly `prepare_host_runtime_with_console(.., &Default::default())`
/// and a host that wants no console allocates nothing extra.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostConsoleRequest {
    /// Bounded per-channel control-queue depth, or `None` for no live control channel.
    ///
    /// Issue #140 turns this into the depth of *every* live channel a track owns: the matrix/pan
    /// queue #137 shipped, the fader/mute queue, and one queue per prepared effect instance. Each
    /// effect's queue is capped at that effect's own `automation_capacity`, which is what makes
    /// the render-side staging window unable to overflow.
    pub control_queue_depth: Option<NonZeroUsize>,
    /// Per-track meter window in frames, or `None` for no meters.
    pub meter_period_frames: Option<NonZeroU32>,
    /// Bounded per-track meter snapshot queue depth.
    pub meter_queue_depth: NonZeroUsize,
    /// Which chain boundary the per-track meters observe.
    pub meter_tap: MeterTap,
    /// Maximum declared observation taps to bind per effect instance, or `0` for **no observation
    /// capacity at all** (issue #143 D3, level 1).
    ///
    /// Zero is the honest form of "observation off": `attach_effect_observation` is never
    /// called, so the compiled plan holds no lane, no accumulator and no conflating cell — not a
    /// disabled one, none — and `observation_retained_bytes` is zero. Nonzero requires a control
    /// channel, because a subscription rides the effect's existing command queue.
    ///
    /// The published window is the *meter* window: it is derived from `meter_period_frames`, not
    /// configured separately, so a gain-reduction value and the peak beside it in one
    /// `miso.meter.v1` frame describe the same span of samples.
    pub observation_taps: u32,
    /// The track whose gain reduction is reported as the master's, or `None` (issue #143 D6).
    ///
    /// V1 has no structural master bus — submixes and outputs carry no effect racks — so the
    /// master reading is a *designation* rather than a discovery. The successor is effect racks on
    /// submixes; until then this is twenty lines and an honest name.
    pub master_track: Option<u32>,
}

impl Default for HostConsoleRequest {
    fn default() -> Self {
        Self {
            control_queue_depth: None,
            meter_period_frames: None,
            meter_queue_depth: NonZeroUsize::MIN,
            meter_tap: MeterTap::PostMatrix,
            observation_taps: 0,
            master_track: None,
        }
    }
}

/// The control-side halves of an attached live console, in canonical track order.
///
/// `tracks` is the compiled session's normalized track order and is the addressing authority: a
/// host addresses a track by its index in this vector, and `track_controls[i]` / `meters[i]`
/// belong to `tracks[i]` whenever they are present.
pub struct HostConsoleHandles {
    /// Canonical normalized track identities.
    pub tracks: Vec<Box<str>>,
    /// One control producer per track, in `tracks` order; empty when no channel was requested.
    ///
    /// Each carries all three of a track's builtin channels: the matrix/pan queue (#137 D1), the
    /// fader/mute queue (#140 B) and the input trim/polarity queue (#210 phase 3).
    pub track_controls: Vec<TrackControlProducer>,
    /// One control producer per prepared effect instance (#140 A); empty when no channel was
    /// requested. Addressed by `(track_id, rack, effect_index)`, where `effect_index` is the
    /// effect's position within its rack in session declaration order.
    pub effect_controls: Vec<EffectControlProducer>,
    /// One meter consumer per track, in `tracks` order; empty when no meters were requested.
    pub meters: Vec<MeterConsumer>,
    /// One reader set per prepared effect instance that declares an observation tap (issue #143).
    ///
    /// Empty when the request named no observation capacity, and empty for every effect whose
    /// descriptor declares no tap. Addressed by `(track_id, rack, effect_index)`, exactly as
    /// [`Self::effect_controls`] is.
    pub effect_observations: Vec<EffectObservationHandle>,
    /// The designated master track index, echoed back after validation against `tracks`.
    pub master_track: Option<u32>,
}

/// One prepared session: the render plan, the control-side source set, and the resource report.
///
/// Field order is the drop order: the plan (which owns the source consumers) drops before the
/// control producers, so a producer never outlives its ring.
///
/// `Send` and deliberately **not** `Sync` (crate-level `# Host callback contract (V1)`): the plan is
/// rendered from exactly one thread and the sources are fed from exactly one thread, so a host that
/// could share this across threads could render from two at once. A shared reference is refused at
/// compile time:
///
/// ```compile_fail
/// fn requires_sync<T: Sync>() {}
/// requires_sync::<miso_engine_host_core::PreparedHost>();
/// ```
///
/// Moving it, on the other hand, is exactly how a host hands preparation to the render thread:
///
/// ```
/// fn requires_send<T: Send>() {}
/// requires_send::<miso_engine_host_core::PreparedHost>();
/// ```
pub struct PreparedHost {
    /// The exclusive render plan. Move it to the render thread once and render there only.
    pub plan: PreparedRenderPlan,
    /// The control-side source producers. Feed PCM from one control thread.
    pub sources: SourceControlSet,
    /// Address-free resource facts about this preparation.
    pub report: HostPrepareReport,
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

/// Parse one session TOML document.
pub fn parse_host_session(toml: &str) -> Result<SessionToml, PrepareDiagnostics> {
    parse_session_toml(toml).map_err(|value| {
        PrepareDiagnostics::new(
            PrepareRejection::Session,
            diagnostic_lines(
                value
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| (diagnostic.code.as_str(), &diagnostic.path)),
            ),
        )
    })
}

/// Parse and compile one session TOML document under this host's caps.
///
/// A host that also needs the transactional `SessionStore` (from `miso-engine-protocol`) (the C ABI host
/// does, for the control protocol) calls [`parse_host_session`] and
/// [`HostPrepareCaps::compile_caps`] instead and builds the store itself: the facade never depends
/// on the control protocol.
pub fn compile_host_session(
    toml: &str,
    caps: &HostPrepareCaps,
) -> Result<CompiledSession, PrepareDiagnostics> {
    let model = parse_host_session(toml)?;
    let compile_caps = caps.compile_caps(model.sources.len())?;
    compile_session(&model, compile_caps).map_err(|value| {
        PrepareDiagnostics::new(
            PrepareRejection::Session,
            diagnostic_lines(
                value
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| (diagnostic.code.as_str(), &diagnostic.path)),
            ),
        )
    })
}

/// Parse, compile and prepare one session in a single call.
///
/// Returns the compiled session alongside the prepared host so a host that wants to answer session
/// queries later can retain it; a host that does not simply drops it.
pub fn prepare_host_session(
    toml: &str,
    caps: &HostPrepareCaps,
) -> Result<(CompiledSession, PreparedHost), PrepareDiagnostics> {
    let compiled = compile_host_session(toml, caps)?;
    let prepared = prepare_host_runtime(&compiled, caps)?;
    Ok((compiled, prepared))
}

/// Parse, compile and prepare one session with a live console attached (issue #137 D1/D2).
pub fn prepare_host_session_with_console(
    toml: &str,
    caps: &HostPrepareCaps,
    console: &HostConsoleRequest,
) -> Result<(CompiledSession, PreparedHost, HostConsoleHandles), PrepareDiagnostics> {
    let compiled = compile_host_session(toml, caps)?;
    let (prepared, handles) = prepare_host_runtime_with_console(&compiled, caps, console)?;
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
    let (prepared, handles) =
        prepare_host_runtime_with_console(compiled, caps, &HostConsoleRequest::default())?;
    debug_assert!(handles.track_controls.is_empty() && handles.meters.is_empty());
    Ok(prepared)
}

/// Prepare the render plan, source control set and live-console handles for a compiled session.
///
/// Issue #137 D1/D2. The console halves are prepared inside the same transaction as the plan, so a
/// session that cannot carry the requested console is rejected before anything is published: there
/// is no partially attached console. Requesting nothing is exactly [`prepare_host_runtime`].
#[allow(clippy::too_many_lines)]
pub fn prepare_host_runtime_with_console(
    compiled: &CompiledSession,
    caps: &HostPrepareCaps,
    console: &HostConsoleRequest,
) -> Result<(PreparedHost, HostConsoleHandles), PrepareDiagnostics> {
    let model = compiled.normalized_model();
    let track_count = u64::try_from(model.tracks.len()).map_err(|_| platform("host.count"))?;
    let source_count = u64::try_from(model.sources.len()).map_err(|_| platform("host.count"))?;
    let route_count = u64::try_from(model.routes.len()).map_err(|_| platform("host.count"))?;
    let effect_count = count_effects(model)?;
    if track_count > caps.maximum_tracks
        || source_count > caps.maximum_sources
        || route_count > caps.maximum_routes
        || effect_count > caps.maximum_effects
    {
        return Err(resource("host.resource.count"));
    }
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
    for source in &model.sources {
        if source.sample_rate_hz != compiled.sample_rate().0 {
            return Err(shape("host.source.rate.mismatch"));
        }
        if caps
            .maximum_source_channels
            .is_some_and(|maximum| u32::from(source.mapping.channel_count) > maximum)
        {
            return Err(shape("host.source.channels"));
        }
        let region_end = source
            .mapping
            .region
            .start_sample
            .checked_add(source.mapping.region.length_samples)
            .ok_or_else(|| resource("host.source.region.overflow"))?;
        let (producer, consumer, resources) = PcmSourceRing::prepare_host_region(
            PcmSourceRingConfig {
                channel_count: u32::from(source.mapping.channel_count),
                quantum_frames: compiled.quantum(),
                frame_capacity: u64::from(caps.source_ring_frames),
                initial_generation: SourceGeneration(1),
            },
            SourceFrame(source.mapping.region.start_sample),
        )
        .map_err(|_| resource("host.source.prepare"))?;
        builder.push(
            source.id.as_str(),
            source.sample_rate_hz,
            u32::from(source.mapping.channel_count),
            source.mapping.region.start_sample,
            region_end,
            producer.into_host_chunk_provider(SampleRateHz(source.sample_rate_hz)),
        );
        graph_sources.push(SourceGraphSource::new(consumer, resources, 0, 0));
    }
    let sources = builder.finish();

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
    let mut effects = prepare_native_session_effects(
        compiled,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: caps.maximum_effect_state_bytes,
            maximum_scratch_bytes: caps.maximum_effect_scratch_bytes,
            maximum_automation_spans_per_block: caps.maximum_automation_spans_per_block,
        },
    )
    .map_err(|diagnostics| {
        PrepareDiagnostics::new(
            PrepareRejection::Effect,
            diagnostic_lines(
                diagnostics
                    .0
                    .iter()
                    .map(|diagnostic| (diagnostic.code, &diagnostic.path)),
            ),
        )
    })?;
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

    // Issue #140 A: one bounded live-control channel per prepared effect instance, at the same
    // depth the builtin channels use and capped at each effect's own automation capacity. This is
    // the only thing that creates one; a host that asks for no console attaches nothing and the
    // plan renders the byte-identical console-free path.
    let effect_controls: Vec<EffectControlProducer> = match console.control_queue_depth {
        None => Vec::new(),
        Some(depth) => attach_effect_console(&mut effects, depth).map_err(|diagnostics| {
            PrepareDiagnostics::new(
                PrepareRejection::Effect,
                diagnostic_lines(
                    diagnostics
                        .0
                        .iter()
                        .map(|diagnostic| (diagnostic.code, &diagnostic.path)),
                ),
            )
        })?,
    };

    // Issue #143 D3, level 1. Observation capacity is attached only when it was asked for, and
    // only alongside a control channel: a subscription rides the effect's own command queue, so
    // "observe without a console" has no delivery path and is a rejection rather than a silent
    // half-attach. The published window is the meter window, derived rather than configured.
    let observation_window_blocks = match (console.meter_period_frames, compiled.quantum().0) {
        (Some(period), quantum) if quantum > 0 => (period.get() / quantum).max(1),
        _ => 1,
    };
    let effect_observations: Vec<EffectObservationHandle> = match console.observation_taps {
        0 => Vec::new(),
        taps if console.control_queue_depth.is_none() => {
            let _ = taps;
            return Err(shape("host.observation.console"));
        }
        taps => attach_effect_observation(&mut effects, taps, observation_window_blocks).map_err(
            |diagnostics| {
                PrepareDiagnostics::new(
                    PrepareRejection::Effect,
                    diagnostic_lines(
                        diagnostics
                            .0
                            .iter()
                            .map(|diagnostic| (diagnostic.code, &diagnostic.path)),
                    ),
                )
            },
        )?,
    };

    // Issue #137 D1/D2: the console requests are derived here, once, from the canonical track
    // order, so `HostConsoleHandles::tracks` and the requested channels cannot disagree.
    let console_tracks: Vec<Box<str>> = model
        .tracks
        .iter()
        .map(|track| Box::<str>::from(track.id.as_str()))
        .collect();
    let control_requests: Vec<TrackControlRequest> = match console.control_queue_depth {
        None => Vec::new(),
        Some(depth) => console_tracks
            .iter()
            .map(|track| TrackControlRequest {
                track_id: track.to_string(),
                queue_capacity: depth,
            })
            .collect(),
    };
    let meter_requests: Vec<MeterRequest> = match console.meter_period_frames {
        None => Vec::new(),
        Some(period) => console_tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                // Handles are `index + 1` so they are nonzero and stable in canonical track
                // order; nothing outside this function invents a meter handle.
                let handle = u64::try_from(index)
                    .ok()
                    .and_then(|value| value.checked_add(1))
                    .and_then(core::num::NonZeroU64::new)
                    .ok_or_else(|| platform("host.count"))?;
                Ok(MeterRequest {
                    handle: MeterHandle(handle),
                    track_id: track.to_string(),
                    tap: console.meter_tap,
                    config: MeterConfig {
                        period_frames: period,
                        peak_hold_frames: 0,
                        peak_decay_db_per_second: 0.0,
                        queue_capacity: console.meter_queue_depth,
                        reset_generation: 0,
                    },
                })
            })
            .collect::<Result<Vec<_>, PrepareDiagnostics>>()?,
    };
    let builtins = prepare_session_builtins_with_console(
        compiled,
        &meter_requests,
        &control_requests,
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
        },
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
    let builtin_resources = builtins.resource_report();
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch: Backend::current(),
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
    let graph_report = artifact.report().clone();
    let graph_resources = artifact.graph_resource_estimate().clone();
    let session_resources = compiled.resource_estimate();
    let admitted_graph_and_model = graph_resources
        .session_plus_plan_bytes
        .checked_add(session_resources.compiled_model_bytes)
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    if admitted_graph_and_model > caps.maximum_graph_session_plus_plan_bytes {
        return Err(resource("host.graph.resource.limit"));
    }
    let source_set = prepare_graph_source_set(artifact.envelope(), graph_sources, mappings)
        .map_err(|_| graph_failure("host.source.graph.prepare"))?;
    let source_resources = source_set.resource_report();
    if source_resources.total_engine_owned_bytes > caps.maximum_source_total_bytes
        || source_resources.overhead_bytes > caps.maximum_source_overhead_bytes
    {
        return Err(resource("host.source.resource.limit"));
    }
    let largest_engine_allocation_bytes = graph_resources
        .largest_allocation_bytes
        .max(source_resources.largest_allocation_bytes)
        .max(builtin_resources.maximum_single_allocation_bytes);
    if largest_engine_allocation_bytes.max(session_resources.single_allocation_bytes)
        > caps.maximum_named_allocation_bytes
        || builtin_resources.engine_owned_retained_payload_bytes
            > caps.maximum_builtin_retained_bytes
    {
        return Err(resource("host.resource.limit"));
    }

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
        observers: Vec::new(),
    };
    let bound = artifact
        .into_bound_with_source_set(bindings, source_set)
        .map_err(|failure| graph_failure(failure.code))?;
    // Issue #137 D2: exactly the requested console halves come back, in canonical track order.
    // A session that produced a meter or control channel nobody asked for is still a hard
    // rejection -- that was the pre-#137 rule and it is what keeps an unrequested observer from
    // silently entering a production plan.
    if bound.meter_consumers.len() != meter_requests.len()
        || bound.track_controls.len() != control_requests.len()
    {
        return Err(graph_failure("host.meter.unexpected"));
    }
    // Canonical normalized track order is the console's addressing authority, so both halves are
    // permuted into it here rather than into whatever order preparation happened to build them.
    // A channel whose track is not in that order at all is a hard rejection, never a silent drop.
    let canonical_index: std::collections::BTreeMap<&str, usize> = console_tracks
        .iter()
        .enumerate()
        .map(|(index, track)| (&**track, index))
        .collect();
    let mut track_controls = bound.track_controls;
    let mut meters = bound.meter_consumers;
    if track_controls
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
    track_controls.sort_by_key(|value| canonical_index[&*value.track_id]);
    meters.sort_by_key(|value| canonical_index[&*value.track_id]);

    // Issue #143 D6: a designated master must name a track this session actually has, or the
    // frame would report a master reading nobody can address.
    let master_track = match console.master_track {
        Some(index) if (index as usize) < console_tracks.len() => Some(index),
        Some(_) => return Err(shape("host.observation.master_track")),
        None => None,
    };
    // Issue #143 R7: walked over the built runtime, not derived from the request. A plan that
    // bound nothing reports zero because it *holds* nothing.
    let observation_retained_bytes = bound.plan.observation_retained_bytes();
    let control_retained_bytes = sources
        .retained_bytes()
        .ok_or_else(|| resource("host.resource.arithmetic"))?;
    let report = HostPrepareReport {
        sample_rate_hz: compiled.sample_rate().0,
        quantum_frames: compiled.quantum().0,
        source_count,
        track_count,
        route_count,
        effect_count,
        latency_samples: graph_report.output_latency.0,
        output_tail: graph_report.output_tail,
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
        effect_scalar_state_bytes: effect_state_bytes,
        effect_scalar_scratch_bytes: effect_scratch_bytes,
        builtin_processor_payload_bytes: builtin_resources.engine_owned_processor_payload_bytes,
        builtin_meter_payload_bytes: builtin_resources.engine_owned_meter_payload_bytes,
        builtin_retained_payload_bytes: builtin_resources.engine_owned_retained_payload_bytes,
        session_model_bytes: session_resources.compiled_model_bytes,
        session_largest_allocation_bytes: session_resources.single_allocation_bytes,
        control_retained_bytes,
        observation_retained_bytes,
        source_id_bytes: u64::try_from(source_id_bytes).map_err(|_| platform("host.count"))?,
        largest_engine_allocation_bytes,
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
    let mut plan = bound.plan;
    let mono_source: BTreeSet<Box<str>> = session_structural_symmetry(compiled)
        .into_iter()
        .filter(|(_, witness)| witness.eligible())
        .map(|(track, _)| track)
        .collect();
    plan.arm_mono_collapse(&|track: &str| mono_source.contains(track));

    Ok((
        PreparedHost {
            plan,
            sources,
            report,
        },
        HostConsoleHandles {
            tracks: console_tracks,
            track_controls,
            effect_controls,
            meters,
            effect_observations,
            master_track,
        },
    ))
}

/// Total effect instances across every rack of every track.
pub fn count_effects(model: &SessionToml) -> Result<u64, PrepareDiagnostics> {
    model.tracks.iter().try_fold(0_u64, |total, track| {
        let count = track
            .simd1
            .effects
            .len()
            .checked_add(track.dynamic.effects.len())
            .and_then(|value| value.checked_add(track.simd2.effects.len()))
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
