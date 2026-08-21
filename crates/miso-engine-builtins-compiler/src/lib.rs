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
    GraphBindingBlock, GraphNodeId, GraphNodeObserverBinding, GraphObservationBlock,
    GraphRuntimeObserver, GraphRuntimeProcessor, StableGraphId, TrackStage,
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
        estimated_meter_bytes = match u64::try_from(core::mem::size_of::<MeterSnapshot>())
            .ok()
            .and_then(|bytes| bytes.checked_mul(slots))
            .and_then(|bytes| estimated_meter_bytes.checked_add(bytes))
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
    }
    let track_count = match u64::try_from(session.normalized_model().tracks.len()) {
        Ok(value) => value,
        Err(_) => {
            diagnostics.push(diag("builtin.resource.arithmetic_overflow", "$.tracks"));
            0
        }
    };
    let estimated_state = u64::try_from(core::mem::size_of::<BuiltinChain>())
        .ok()
        .and_then(|bytes| bytes.checked_mul(track_count))
        .unwrap_or_else(|| {
            diagnostics.push(diag("builtin.resource.arithmetic_overflow", "$.tracks"));
            u64::MAX
        });
    if estimated_state > caps.maximum_total_state_bytes
        || estimated_items > caps.maximum_total_meter_items
        || estimated_meter_bytes > caps.maximum_total_meter_bytes
        || estimated_state.max(estimated_meter_bytes) > caps.maximum_single_allocation_bytes
    {
        diagnostics.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
    }

    let mut prepared = Vec::new();
    let mut tails = Vec::new();
    for track in &session.normalized_model().tracks {
        match track_parameters(track, caps.maximum_smoothing_samples) {
            Ok(parameters) => match BuiltinChain::new(session.sample_rate().0, parameters) {
                Ok(chain) => {
                    tails.push((track.id.as_str().to_owned(), chain.tail()));
                    prepared.push((track.id.as_str().to_owned(), chain.into_sections()));
                }
                Err(error) => diagnostics.push(parameter_diagnostic(track, error)),
            },
            Err(error) => diagnostics.push(parameter_diagnostic(track, error)),
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
    })
}

struct InputProcessor(InputBuiltins);
impl GraphRuntimeProcessor for InputProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let _ = self.0.process(DualMonoBlock {
            left: block.left,
            right: block.right,
            first_sample: block.first_sample,
        });
        Ok(())
    }
}
struct FaderProcessor(FaderMuteBuiltins);
impl GraphRuntimeProcessor for FaderProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let _ = self.0.process(DualMonoBlock {
            left: block.left,
            right: block.right,
            first_sample: block.first_sample,
        });
        Ok(())
    }
}
struct MatrixProcessor(MatrixBuiltins);
impl GraphRuntimeProcessor for MatrixProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let _ = self.0.process(DualMonoBlock {
            left: block.left,
            right: block.right,
            first_sample: block.first_sample,
        });
        Ok(())
    }
}
struct MeterObserver(MeterAccumulator);
impl GraphRuntimeObserver for MeterObserver {
    fn observe(&mut self, block: GraphObservationBlock<'_>) {
        self.0.observe(block.left, block.right, block.first_sample);
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
fn parameter_diagnostic(track: &Track, error: BuiltinParameterError) -> BuiltinDiagnostic {
    let code = match error {
        BuiltinParameterError::GainDomain => "builtin.gain.domain",
        BuiltinParameterError::FilterCutoff => "builtin.filter.cutoff",
        BuiltinParameterError::FilterOrder => "builtin.filter.order",
        BuiltinParameterError::FilterCoefficients => "builtin.filter.coefficients",
        BuiltinParameterError::MatrixCoefficient => "builtin.matrix.coefficient",
        BuiltinParameterError::MatrixSmoothing => "builtin.matrix.smoothing",
        _ => "builtin.resource.arithmetic_overflow",
    };
    diag(code, &format!("$.tracks[id={}].builtins", track.id))
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
}
