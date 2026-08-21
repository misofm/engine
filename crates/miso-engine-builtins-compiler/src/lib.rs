//! Off-render preparation adapter for issue-007 builtins.
#![allow(missing_docs)]

use core::num::NonZeroU64;
use std::collections::BTreeSet;

use miso_engine_builtins::{
    BuiltinChain, BuiltinParameterError, BuiltinParameters, BuiltinTail, ChannelParameters,
    DualMonoBlock, FaderMuteBuiltins, InputBuiltins, Matrix2x2, MatrixBuiltins, MeterAccumulator,
    MeterConfig, MeterConfigError, MeterHandle, MeterSnapshot, MeterTap, PreparedMeter, pan_matrix,
};
use miso_engine_core::realtime::{Consumer, RenderError};
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphNodeObserverBinding,
    GraphObservationBlock, GraphRuntimeObserver, GraphRuntimeProcessor, StableGraphId, TrackStage,
};
use miso_engine_session::{CompiledSession, MatrixOrPan, Track};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinCompileCaps {
    pub maximum_total_state_bytes: u64,
    pub maximum_total_meter_items: u64,
    pub maximum_total_meter_bytes: u64,
    pub maximum_single_allocation_bytes: u64,
    pub maximum_meter_streams: u64,
    pub maximum_period_frames: u32,
    pub maximum_peak_hold_frames: u32,
    pub maximum_smoothing_samples: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeterRequest {
    pub track_id: String,
    pub tap: MeterTap,
    pub config: MeterConfig,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuiltinDiagnostic {
    pub code: &'static str,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltinDiagnosticSet(pub Vec<BuiltinDiagnostic>);

impl BuiltinDiagnosticSet {
    pub fn sorted(mut values: Vec<BuiltinDiagnostic>) -> Self {
        values.sort();
        values.dedup();
        Self(values)
    }
}

pub struct MeterConsumer {
    pub handle: MeterHandle,
    pub track_id: String,
    pub tap: MeterTap,
    pub consumer: Consumer<MeterSnapshot>,
}

pub struct PreparedBuiltinsSession {
    pub session: CompiledSession,
    pub processors: Vec<miso_engine_graph::GraphNodeBinding>,
    pub observers: Vec<GraphNodeObserverBinding>,
    pub meter_consumers: Vec<MeterConsumer>,
    pub tails: Vec<(String, BuiltinTail)>,
    pub resources: BuiltinResourceEstimate,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BuiltinResourceEstimate {
    pub retained_processor_bytes: u64,
    pub retained_meter_bytes: u64,
    pub meter_items: u64,
    pub largest_allocation_bytes: u64,
}

pub fn prepare_session_builtins(
    session: &CompiledSession,
    requests: &[MeterRequest],
    caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet> {
    let mut diagnostics = Vec::new();
    if [
        caps.maximum_total_state_bytes,
        caps.maximum_total_meter_items,
        caps.maximum_total_meter_bytes,
        caps.maximum_single_allocation_bytes,
        caps.maximum_meter_streams,
    ]
    .into_iter()
    .any(|value| value == 0)
    {
        diagnostics.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
    }
    if requests.len() as u64 > caps.maximum_meter_streams {
        diagnostics.push(diag("builtin.resource.limit", "$.meter_requests"));
    }
    let mut request_keys = BTreeSet::new();
    for request in requests {
        let key = (request.track_id.clone(), request.tap);
        if !request_keys.insert(key) {
            diagnostics.push(diag("builtin.meter.duplicate", &meter_path(request)));
        }
        if request.config.period_frames.get() > caps.maximum_period_frames
            || request.config.peak_hold_frames > caps.maximum_peak_hold_frames
        {
            diagnostics.push(diag("builtin.resource.limit", &meter_path(request)));
        }
        if !request.config.peak_decay_db_per_second.is_finite()
            || !(0.0..=120.0).contains(&request.config.peak_decay_db_per_second)
        {
            diagnostics.push(diag("builtin.meter.config", &meter_path(request)));
        }
    }
    let known_tracks: BTreeSet<_> = session
        .normalized_model()
        .tracks
        .iter()
        .map(|track| track.id.as_str())
        .collect();
    for request in requests {
        if !known_tracks.contains(request.track_id.as_str()) {
            diagnostics.push(diag("builtin.meter.unknown_track", &meter_path(request)));
        }
    }
    let mut estimated_items = 0_u64;
    let mut estimated_meter_bytes = 0_u64;
    let mut largest_allocation_bytes = 0_u64;
    for request in requests {
        let slots = match u64::try_from(request.config.queue_capacity.get())
            .ok()
            .and_then(|value| value.checked_add(1))
        {
            Some(value) => value,
            None => {
                diagnostics.push(diag(
                    "builtin.resource.arithmetic_overflow",
                    &meter_path(request),
                ));
                continue;
            }
        };
        estimated_items = match estimated_items.checked_add(slots) {
            Some(value) => value,
            None => {
                diagnostics.push(diag(
                    "builtin.resource.arithmetic_overflow",
                    "$.meter_requests",
                ));
                continue;
            }
        };
        let queue_payload_bytes = match u64::try_from(core::mem::size_of::<MeterSnapshot>())
            .ok()
            .and_then(|bytes| bytes.checked_mul(slots))
        {
            Some(value) => value,
            None => {
                diagnostics.push(diag(
                    "builtin.resource.arithmetic_overflow",
                    "$.meter_requests",
                ));
                continue;
            }
        };
        largest_allocation_bytes = largest_allocation_bytes.max(queue_payload_bytes);
        let meter_endpoint_bytes = checked_type_bytes::<MeterObserver>(1)
            .and_then(|bytes| bytes.checked_add(checked_type_bytes::<MeterConsumer>(1)?))
            .and_then(|bytes| {
                bytes.checked_add(checked_type_bytes::<GraphNodeObserverBinding>(1)?)
            });
        estimated_meter_bytes = match meter_endpoint_bytes
            .and_then(|bytes| queue_payload_bytes.checked_add(bytes))
            .and_then(|bytes| estimated_meter_bytes.checked_add(bytes))
        {
            Some(value) => value,
            None => {
                diagnostics.push(diag(
                    "builtin.resource.arithmetic_overflow",
                    &meter_path(request),
                ));
                continue;
            }
        };
    }
    let track_count = match u64::try_from(session.normalized_model().tracks.len()) {
        Ok(value) => value,
        Err(_) => {
            diagnostics.push(diag("builtin.resource.arithmetic_overflow", "$.tracks"));
            0
        }
    };
    let estimated_state = checked_type_bytes::<BuiltinChain>(track_count)
        .and_then(|bytes| bytes.checked_add(checked_type_bytes::<InputProcessor>(track_count)?))
        .and_then(|bytes| bytes.checked_add(checked_type_bytes::<FaderProcessor>(track_count)?))
        .and_then(|bytes| bytes.checked_add(checked_type_bytes::<MatrixProcessor>(track_count)?))
        .and_then(|bytes| {
            bytes.checked_add(checked_type_bytes::<GraphNodeBinding>(
                track_count.checked_mul(3)?,
            )?)
        })
        .and_then(|bytes| {
            bytes.checked_add(checked_type_bytes::<(String, BuiltinTail)>(track_count)?)
        })
        .unwrap_or_else(|| {
            diagnostics.push(diag("builtin.resource.arithmetic_overflow", "$.tracks"));
            u64::MAX
        });
    largest_allocation_bytes = largest_allocation_bytes
        .max(
            checked_type_bytes::<GraphNodeBinding>(track_count.saturating_mul(3))
                .unwrap_or(u64::MAX),
        )
        .max(
            checked_type_bytes::<GraphNodeObserverBinding>(requests.len() as u64)
                .unwrap_or(u64::MAX),
        )
        .max(checked_type_bytes::<MeterConsumer>(requests.len() as u64).unwrap_or(u64::MAX))
        .max(checked_type_bytes::<(String, BuiltinTail)>(track_count).unwrap_or(u64::MAX));
    if estimated_state > caps.maximum_total_state_bytes
        || estimated_items > caps.maximum_total_meter_items
        || estimated_meter_bytes > caps.maximum_total_meter_bytes
        || largest_allocation_bytes > caps.maximum_single_allocation_bytes
    {
        diagnostics.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
    }

    let mut prepared = Vec::with_capacity(session.normalized_model().tracks.len());
    let mut tails = Vec::with_capacity(session.normalized_model().tracks.len());
    for track in &session.normalized_model().tracks {
        match track_parameters(track, caps.maximum_smoothing_samples) {
            Ok(parameters) => match BuiltinChain::new(session.sample_rate().0, parameters) {
                Ok(chain) => {
                    tails.push((track.id.as_str().to_owned(), chain.tail()));
                    prepared.push((track.id.as_str().to_owned(), chain.into_sections()));
                }
                Err(error) => {
                    diagnostics.push(parameter_diagnostic(track, error, session.sample_rate().0))
                }
            },
            Err(error) => {
                diagnostics.push(parameter_diagnostic(track, error, session.sample_rate().0))
            }
        }
    }
    if !diagnostics.is_empty() {
        return Err(BuiltinDiagnosticSet::sorted(diagnostics));
    }

    let mut processors = Vec::with_capacity(prepared.len().saturating_mul(3));
    for (track_id, (input, fader, matrix)) in prepared {
        let graph_id = StableGraphId::parse(&track_id).expect("accepted session IDs are graph IDs");
        processors.push(miso_engine_graph::GraphNodeBinding::new(
            stage_node(graph_id.clone(), TrackStage::PostInputBuiltins),
            Box::new(InputProcessor(input)),
        ));
        processors.push(miso_engine_graph::GraphNodeBinding::new(
            stage_node(graph_id.clone(), TrackStage::PostFader),
            Box::new(FaderProcessor(fader)),
        ));
        processors.push(miso_engine_graph::GraphNodeBinding::new(
            stage_node(graph_id, TrackStage::PostMatrix),
            Box::new(MatrixProcessor(matrix)),
        ));
    }
    let mut observers = Vec::with_capacity(requests.len());
    let mut meter_consumers = Vec::with_capacity(requests.len());
    for (index, request) in requests.iter().enumerate() {
        let handle = MeterHandle(
            NonZeroU64::new(
                u64::try_from(index)
                    .expect("usize fits u64")
                    .checked_add(1)
                    .expect("request count bounded"),
            )
            .expect("one-based"),
        );
        let PreparedMeter {
            accumulator,
            consumer,
        } = MeterAccumulator::prepare(handle, request.config, session.sample_rate().0).map_err(
            |error| BuiltinDiagnosticSet::sorted(vec![meter_diagnostic(request, error)]),
        )?;
        let graph_id = StableGraphId::parse(&request.track_id).expect("known accepted session ID");
        observers.push(GraphNodeObserverBinding::new(
            stage_node(graph_id, stage(request.tap)),
            handle.0.get(),
            Box::new(MeterObserver(accumulator)),
        ));
        meter_consumers.push(MeterConsumer {
            handle,
            track_id: request.track_id.clone(),
            tap: request.tap,
            consumer,
        });
    }
    tails.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(PreparedBuiltinsSession {
        session: session.clone(),
        processors,
        observers,
        meter_consumers,
        tails,
        resources: BuiltinResourceEstimate {
            retained_processor_bytes: estimated_state,
            retained_meter_bytes: estimated_meter_bytes,
            meter_items: estimated_items,
            largest_allocation_bytes,
        },
    })
}

fn checked_type_bytes<T>(items: u64) -> Option<u64> {
    u64::try_from(core::mem::size_of::<T>())
        .ok()?
        .checked_mul(items)
}

struct InputProcessor(InputBuiltins);
impl GraphRuntimeProcessor for InputProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process(
                DualMonoBlock::new(block.left, block.right, block.first_sample)
                    .map_err(render_error)?,
            )
            .map(|_| ())
            .map_err(render_error)
    }
}
struct FaderProcessor(FaderMuteBuiltins);
impl GraphRuntimeProcessor for FaderProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process(
                DualMonoBlock::new(block.left, block.right, block.first_sample)
                    .map_err(render_error)?,
            )
            .map(|_| ())
            .map_err(render_error)
    }
}
struct MatrixProcessor(MatrixBuiltins);
impl GraphRuntimeProcessor for MatrixProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process(
                DualMonoBlock::new(block.left, block.right, block.first_sample)
                    .map_err(render_error)?,
            )
            .map(|_| ())
            .map_err(render_error)
    }
}
struct MeterObserver(MeterAccumulator);
impl GraphRuntimeObserver for MeterObserver {
    fn observe(&mut self, block: GraphObservationBlock<'_>) {
        let _ = self.0.observe(block.left, block.right, block.first_sample);
    }
}

fn render_error(error: BuiltinParameterError) -> RenderError {
    match error {
        BuiltinParameterError::SampleTimeOverflow => RenderError::TimeOverflow,
        _ => RenderError::InvalidEnvelope,
    }
}

fn track_parameters(
    track: &Track,
    maximum_smoothing: u32,
) -> Result<BuiltinParameters, BuiltinParameterError> {
    let left = ChannelParameters {
        polarity_invert: track.builtins.left.polarity_invert,
        trim_db: track.builtins.left.trim_db,
        hpf_hz: track.builtins.left.hpf_hz,
        lpf_hz: track.builtins.left.lpf_hz,
        fader_db: track.fader.left_db,
        muted: track.fader.left_mute,
    };
    let right = ChannelParameters {
        polarity_invert: track.builtins.right.polarity_invert,
        trim_db: track.builtins.right.trim_db,
        hpf_hz: track.builtins.right.hpf_hz,
        lpf_hz: track.builtins.right.lpf_hz,
        fader_db: track.fader.right_db,
        muted: track.fader.right_mute,
    };
    let (matrix, smoothing_samples) = match track.matrix_or_pan {
        MatrixOrPan::Pan {
            left,
            right,
            smoothing_samples,
        } => (pan_matrix(left, right)?, smoothing_samples),
        MatrixOrPan::Matrix {
            ll,
            lr,
            rl,
            rr,
            smoothing_samples,
        } => (Matrix2x2 { ll, lr, rl, rr }.checked()?, smoothing_samples),
    };
    if smoothing_samples > maximum_smoothing {
        return Err(BuiltinParameterError::MatrixSmoothing);
    }
    Ok(BuiltinParameters {
        left,
        right,
        matrix,
        smoothing_samples,
    })
}

fn stage(tap: MeterTap) -> TrackStage {
    match tap {
        MeterTap::Input => TrackStage::Input,
        MeterTap::PostInputBuiltins => TrackStage::PostInputBuiltins,
        MeterTap::PostSimd1 => TrackStage::PostSimd1,
        MeterTap::PostDynamic => TrackStage::PostDynamic,
        MeterTap::PostSimd2PreFader => TrackStage::PostSimd2PreFader,
        MeterTap::PostFader => TrackStage::PostFader,
        MeterTap::PostMatrix => TrackStage::PostMatrix,
    }
}
fn stage_node(track_id: StableGraphId, stage: TrackStage) -> GraphNodeId {
    GraphNodeId::TrackStage { track_id, stage }
}
fn diag(code: &'static str, path: &str) -> BuiltinDiagnostic {
    BuiltinDiagnostic {
        code,
        path: path.to_owned(),
    }
}
fn meter_path(request: &MeterRequest) -> String {
    format!(
        "$.meters[track_id={},tap={:?}]",
        request.track_id, request.tap
    )
}
fn parameter_diagnostic(
    track: &Track,
    error: BuiltinParameterError,
    sample_rate: u32,
) -> BuiltinDiagnostic {
    let code = match error {
        BuiltinParameterError::GainDomain => "builtin.gain.domain",
        BuiltinParameterError::FilterCutoff => "builtin.filter.cutoff",
        BuiltinParameterError::FilterOrder => "builtin.filter.order",
        BuiltinParameterError::FilterCoefficients => "builtin.filter.coefficients",
        BuiltinParameterError::MatrixCoefficient => "builtin.matrix.coefficient",
        BuiltinParameterError::MatrixSmoothing => "builtin.matrix.smoothing",
        _ => "builtin.resource.arithmetic_overflow",
    };
    let track_path = format!("$.tracks[id={}]", track.id);
    let path = match error {
        BuiltinParameterError::GainDomain => gain_path(track, &track_path),
        BuiltinParameterError::FilterCutoff => cutoff_path(track, &track_path, sample_rate),
        BuiltinParameterError::FilterOrder => filter_order_path(track, &track_path),
        BuiltinParameterError::MatrixCoefficient => matrix_path(track, &track_path),
        BuiltinParameterError::MatrixSmoothing => {
            format!("{track_path}.matrix_or_pan.smoothing_samples")
        }
        _ => format!("{track_path}.builtins"),
    };
    diag(code, &path)
}

fn gain_path(track: &Track, track_path: &str) -> String {
    for (lane, builtins, fader) in [
        ("left", &track.builtins.left, track.fader.left_db),
        ("right", &track.builtins.right, track.fader.right_db),
    ] {
        if !builtins.trim_db.is_finite() || !(-144.0..=24.0).contains(&builtins.trim_db) {
            return format!("{track_path}.builtins.{lane}.trim_db");
        }
        if !fader.is_finite() || !(-144.0..=24.0).contains(&fader) {
            return format!("{track_path}.fader.{lane}_db");
        }
    }
    format!("{track_path}.builtins")
}

fn cutoff_path(track: &Track, track_path: &str, sample_rate: u32) -> String {
    for (lane, builtins) in [
        ("left", &track.builtins.left),
        ("right", &track.builtins.right),
    ] {
        if invalid_cutoff(builtins.hpf_hz, sample_rate) {
            return format!("{track_path}.builtins.{lane}.hpf_hz");
        }
        if invalid_cutoff(builtins.lpf_hz, sample_rate) {
            return format!("{track_path}.builtins.{lane}.lpf_hz");
        }
    }
    format!("{track_path}.builtins")
}

fn filter_order_path(track: &Track, track_path: &str) -> String {
    if track.builtins.left.hpf_hz > 0.0
        && track.builtins.left.lpf_hz > 0.0
        && track.builtins.left.hpf_hz >= track.builtins.left.lpf_hz
    {
        format!("{track_path}.builtins.left.lpf_hz")
    } else {
        format!("{track_path}.builtins.right.lpf_hz")
    }
}

fn invalid_cutoff(value: f32, sample_rate: u32) -> bool {
    !value.is_finite()
        || value < 0.0
        || (value > 0.0 && (value < 10.0 || value >= sample_rate as f32 * 0.5))
}

fn matrix_path(track: &Track, track_path: &str) -> String {
    match track.matrix_or_pan {
        MatrixOrPan::Pan { left, .. } if !left.is_finite() || !(-1.0..=1.0).contains(&left) => {
            format!("{track_path}.matrix_or_pan.left")
        }
        MatrixOrPan::Pan { .. } => format!("{track_path}.matrix_or_pan.right"),
        MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => {
            for (field, value) in [("ll", ll), ("lr", lr), ("rl", rl), ("rr", rr)] {
                if !value.is_finite() || !(-1.0..=1.0).contains(&value) {
                    return format!("{track_path}.matrix_or_pan.{field}");
                }
            }
            format!("{track_path}.matrix_or_pan")
        }
    }
}
fn meter_diagnostic(request: &MeterRequest, error: MeterConfigError) -> BuiltinDiagnostic {
    diag(
        match error {
            MeterConfigError::DecayDomain => "builtin.meter.config",
            MeterConfigError::Queue => "builtin.resource.arithmetic_overflow",
        },
        &meter_path(request),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::{NonZeroU32, NonZeroUsize};
    use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};

    fn session() -> CompiledSession {
        let document = include_str!("../../../fixtures/session/v1/canonical.toml");
        compile_session(
            &parse_session_toml(document).expect("parse"),
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile")
    }
    fn caps() -> BuiltinCompileCaps {
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        }
    }
    #[test]
    fn prepares_three_sections_and_each_named_meter_tap() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 8,
            peak_decay_db_per_second: 12.0,
            queue_capacity: NonZeroUsize::new(4).expect("constant"),
            reset_generation: 3,
        };
        let requests: Vec<_> = [
            MeterTap::Input,
            MeterTap::PostInputBuiltins,
            MeterTap::PostSimd1,
            MeterTap::PostDynamic,
            MeterTap::PostSimd2PreFader,
            MeterTap::PostFader,
            MeterTap::PostMatrix,
        ]
        .into_iter()
        .map(|tap| MeterRequest {
            track_id: "vocal".to_owned(),
            tap,
            config,
        })
        .collect();
        let prepared = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
        assert_eq!(prepared.processors.len(), 3);
        assert_eq!(prepared.observers.len(), 7);
        assert_eq!(prepared.meter_consumers.len(), 7);
        assert_eq!(prepared.resources.meter_items, 35);
        assert!(prepared.resources.retained_processor_bytes > 0);
        assert!(
            prepared.resources.retained_meter_bytes
                > 35 * core::mem::size_of::<MeterSnapshot>() as u64
        );
        assert!(
            prepared.resources.largest_allocation_bytes
                >= 5 * core::mem::size_of::<MeterSnapshot>() as u64
        );
        assert_eq!(
            prepared.tails,
            vec![("vocal".to_owned(), BuiltinTail::Infinite)]
        );
    }
    #[test]
    fn rejects_duplicate_and_unknown_meter_transactionally() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 0,
        };
        let result = prepare_session_builtins(
            &session(),
            &[
                MeterRequest {
                    track_id: "missing".to_owned(),
                    tap: MeterTap::Input,
                    config,
                },
                MeterRequest {
                    track_id: "missing".to_owned(),
                    tap: MeterTap::Input,
                    config,
                },
            ],
            caps(),
        );
        let Err(error) = result else {
            panic!("must reject");
        };
        assert_eq!(
            error.0.iter().map(|item| item.code).collect::<Vec<_>>(),
            vec!["builtin.meter.duplicate", "builtin.meter.unknown_track"]
        );
    }

    #[test]
    fn resource_estimate_enforces_the_actual_largest_retained_payload() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 0,
        };
        let requests = [MeterRequest {
            track_id: "vocal".to_owned(),
            tap: MeterTap::PostMatrix,
            config,
        }];
        let baseline = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
        let mut constrained = caps();
        constrained.maximum_single_allocation_bytes = baseline
            .resources
            .largest_allocation_bytes
            .saturating_sub(1);
        let Err(error) = prepare_session_builtins(&session(), &requests, constrained) else {
            panic!("largest retained payload must be capped");
        };
        assert_eq!(
            error.0,
            vec![diag("builtin.resource.limit", "$.builtin_compile_caps")]
        );
    }
}
