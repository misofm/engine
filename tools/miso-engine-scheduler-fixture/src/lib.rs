//! Shared Issue-039 q128 production-fixture preparation.
//!
//! The fixture is a qualification harness, not a graph or scheduler implementation API.  It
//! builds the one frozen production graph exclusively through the ordinary session, effect,
//! builtin, graph-compiler, and native-binding boundaries.

#![allow(missing_docs)]

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use core::num::NonZeroUsize;
    use std::sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    };

    use miso_engine_builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
    use miso_engine_conformance::DualAccumulatorDelayFactory;
    use miso_engine_core::realtime::{PlanarBufferMut, RenderError, RenderIo, RenderTime};
    use miso_engine_effect_compiler::{EffectCompileCaps, prepare_native_session_effects};
    use miso_engine_effect_contract::NativeEffectRegistry;
    use miso_engine_graph::{
        GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphNodeObserverBinding,
        GraphObservationBlock, GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor,
        NativeGraphBindConfigV1, NativeGraphPreparedMetadataV1, NativeGraphRenderModeV1,
        NativeSchedulerConfigV1, TrackStage,
    };
    use miso_engine_graph_compiler::{
        GraphBuiltinsCompileRequest, GraphCompileReport, GraphCompiler,
    };
    use miso_engine_session::{
        ChannelMatrix, CompileCaps, EffectIdentity, EffectParam, ParameterChannel, ParameterUnit,
        RouteDestination, RouteSource, SendTap, Sidechain, SidechainDeclaration, StableId, Submix,
        compile_session, parse_session_toml,
    };

    pub const Q128_QUANTUM_FRAMES: usize = 128;
    pub const Q128_TRACK_COUNT: usize = 12;
    pub const Q128_FIXTURE_ID: &str = "issue039-q128-production-v1";

    const SESSION_FIXTURE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");
    const OBSERVER_POST_SIMD1: u64 = 0x0390_0001;
    const OBSERVER_POST_MATRIX: u64 = 0x0390_0002;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Q128RenderMode {
        Sequential,
        DependencyWaves,
    }

    impl Q128RenderMode {
        const fn native_mode(self) -> NativeGraphRenderModeV1 {
            match self {
                Self::Sequential => NativeGraphRenderModeV1::SingleThread,
                Self::DependencyWaves => NativeGraphRenderModeV1::DependencyWaves,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Q128ObserverRecord {
        pub sample_time: u64,
        pub node_token: u64,
        pub handle: u64,
        pub boundary: u8,
        pub value_bits: u32,
    }

    struct Transcript {
        next: AtomicUsize,
        fields: Box<[AtomicU64]>,
    }

    impl Transcript {
        fn new(capacity: usize) -> Self {
            Self {
                next: AtomicUsize::new(0),
                fields: (0..capacity.saturating_mul(5))
                    .map(|_| AtomicU64::new(0))
                    .collect(),
            }
        }

        fn record(&self, record: Q128ObserverRecord) -> Result<(), RenderError> {
            let index = self.next.fetch_add(1, Ordering::Relaxed);
            let base = index.checked_mul(5).ok_or(RenderError::InvalidEnvelope)?;
            let fields = self
                .fields
                .get(base..base.saturating_add(5))
                .ok_or(RenderError::InvalidEnvelope)?;
            fields[0].store(record.sample_time, Ordering::Relaxed);
            fields[1].store(record.node_token, Ordering::Relaxed);
            fields[2].store(record.handle, Ordering::Relaxed);
            fields[3].store(u64::from(record.boundary), Ordering::Relaxed);
            fields[4].store(u64::from(record.value_bits), Ordering::Relaxed);
            Ok(())
        }

        fn records(&self) -> Vec<Q128ObserverRecord> {
            let capacity = self.fields.len() / 5;
            let count = self.next.load(Ordering::Relaxed).min(capacity);
            (0..count)
                .map(|index| {
                    let base = index * 5;
                    Q128ObserverRecord {
                        sample_time: self.fields[base].load(Ordering::Relaxed),
                        node_token: self.fields[base + 1].load(Ordering::Relaxed),
                        handle: self.fields[base + 2].load(Ordering::Relaxed),
                        boundary: self.fields[base + 3].load(Ordering::Relaxed) as u8,
                        value_bits: self.fields[base + 4].load(Ordering::Relaxed) as u32,
                    }
                })
                .collect()
        }

        fn count(&self) -> usize {
            self.next.load(Ordering::Relaxed)
        }
    }

    struct Observer {
        record: Q128ObserverRecord,
        transcript: Arc<Transcript>,
    }

    impl GraphRuntimeObserver for Observer {
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            self.transcript.record(Q128ObserverRecord {
                sample_time: block.first_sample,
                value_bits: block.left.first().copied().unwrap_or(0.0).to_bits(),
                ..self.record
            })
        }
    }

    struct Source {
        left: f32,
        right: f32,
        phase: u64,
    }

    impl GraphRuntimeProcessor for Source {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for (frame, (left, right)) in block
                .left
                .iter_mut()
                .zip(block.right.iter_mut())
                .enumerate()
            {
                let sample = self.phase.wrapping_add(frame as u64) as f32;
                *left = self.left + sample * 0.000_031_25;
                *right = self.right - sample * 0.000_015_625;
            }
            self.phase = self.phase.saturating_add(block.left.len() as u64);
            Ok(())
        }
    }

    struct Identity;

    impl GraphRuntimeProcessor for Identity {
        fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    pub struct PreparedQ128Fixture {
        pub plan: miso_engine_core::realtime::PreparedRenderPlan,
        pub metadata: NativeGraphPreparedMetadataV1,
        pub report: GraphCompileReport,
        pub pdc_samples: u64,
        transcript: Arc<Transcript>,
    }

    impl PreparedQ128Fixture {
        pub fn render(
            &mut self,
            output: &mut [f32],
            absolute_sample: u64,
        ) -> Result<(), RenderError> {
            self.plan.render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(
                        output,
                        2,
                        Q128_QUANTUM_FRAMES,
                        Q128_QUANTUM_FRAMES,
                    )?,
                },
                RenderTime { absolute_sample },
            )?;
            Ok(())
        }

        #[must_use]
        pub fn observer_records(&self) -> Vec<Q128ObserverRecord> {
            self.transcript.records()
        }

        #[must_use]
        pub fn observer_record_count(&self) -> usize {
            self.transcript.count()
        }
    }

    pub fn prepare_q128_fixture(
        sample_rate_hz: u32,
        render_lanes: usize,
        render_mode: Q128RenderMode,
        plan_id: u64,
        transcript_capacity: usize,
    ) -> Result<PreparedQ128Fixture, String> {
        let mut model = parse_session_toml(SESSION_FIXTURE).map_err(|_| "q128.parse".to_owned())?;
        model.sample_rate_hz = sample_rate_hz;
        model.quantum_frames = Q128_QUANTUM_FRAMES as u32;
        model.sources[0].sample_rate_hz = sample_rate_hz;
        model.automation.clear();
        let base_track = model.tracks[0].clone();
        let base_route = model.routes[0].clone();
        model.tracks = (0..Q128_TRACK_COUNT)
            .map(|index| {
                let mut track = base_track.clone();
                track.id = stable(&format!("qtrack{index:02}"));
                track.simd1.effects.clear();
                track.dynamic.effects.clear();
                track.simd2.effects.clear();
                track.builtins.left.polarity_invert = index % 3 == 0;
                track.builtins.left.trim_db = -1.5 + index as f32 * 0.125;
                track.builtins.left.hpf_hz = 23.0 + index as f32;
                track.builtins.left.lpf_hz = 19_500.0 - index as f32 * 11.0;
                track.builtins.right.polarity_invert = index % 4 == 0;
                track.builtins.right.trim_db = 0.75 - index as f32 * 0.1;
                track.builtins.right.hpf_hz = 31.0 + index as f32 * 0.5;
                track.builtins.right.lpf_hz = 18_900.0 - index as f32 * 13.0;
                track.fader.left_db = -0.25 * index as f32;
                track.fader.right_db = -0.125 * index as f32;
                if index % 2 == 0 {
                    track.matrix_or_pan = miso_engine_session::MatrixOrPan::Matrix {
                        ll: 1.0,
                        lr: 0.025,
                        rl: -0.015,
                        rr: 0.975,
                        smoothing_samples: 16,
                    };
                }
                if index != 0 && index != Q128_TRACK_COUNT - 1 {
                    track
                        .simd1
                        .effects
                        .push(delay_effect(index, SidechainDeclaration::None));
                }
                if index == Q128_TRACK_COUNT - 1 {
                    track.dynamic.effects.push(delay_effect(
                        index,
                        SidechainDeclaration::Routed(Sidechain {
                            source: RouteSource::Track {
                                track_id: stable("qtrack00"),
                                tap: SendTap::PostMatrix,
                            },
                            port_id: stable("sidechain-in"),
                        }),
                    ));
                }
                track
            })
            .collect();
        model.submixes = vec![Submix {
            id: stable("qsubmix"),
        }];
        model.routes = model
            .tracks
            .iter()
            .enumerate()
            .map(|(index, track)| {
                let mut route = base_route.clone();
                route.id = stable(&format!("qsend{index:02}"));
                route.source = RouteSource::Track {
                    track_id: track.id.clone(),
                    tap: SendTap::PostMatrix,
                };
                route.destination = RouteDestination::SubmixInput {
                    submix_id: stable("qsubmix"),
                };
                route.channel_matrix = ChannelMatrix {
                    ll: 0.9,
                    lr: 0.1,
                    rl: -0.05,
                    rr: 0.95,
                };
                route.gain_db = -0.1 * index as f32;
                route
            })
            .chain(core::iter::once({
                let mut route = base_route.clone();
                route.id = stable("qsubmix-main");
                route.source = RouteSource::SubmixOutput {
                    submix_id: stable("qsubmix"),
                };
                route.destination = RouteDestination::OutputInput {
                    output_id: stable("main-out"),
                };
                route.channel_matrix = ChannelMatrix {
                    ll: 1.0,
                    lr: 0.0,
                    rl: 0.0,
                    rr: 1.0,
                };
                route.gain_db = 0.0;
                route
            }))
            .collect();

        let session = compile_session(&model, unbounded_session_caps())
            .map_err(|_| "q128.session".to_owned())?;
        let registry =
            NativeEffectRegistry::new([Box::new(DualAccumulatorDelayFactory::correct())
                as Box<dyn miso_engine_effect_contract::NativeEffectFactory>])
            .map_err(|_| "q128.registry".to_owned())?;
        let effects = prepare_native_session_effects(&session, &registry, effect_caps())
            .map_err(|_| "q128.effects".to_owned())?;
        let builtins = prepare_session_builtins(&session, &[], builtin_caps())
            .map_err(|_| "q128.builtins".to_owned())?;
        let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
            plan_id,
            effects,
            builtins,
            caps: graph_caps(),
        })
        .map_err(|_| "q128.graph".to_owned())?;
        if artifact.prepared_builtin_bank_count() == 0 {
            return Err("q128.builtin_bank".to_owned());
        }
        let report = artifact.report().clone();
        let pdc_samples = report
            .inserted_delays
            .iter()
            .map(|delay| delay.samples.0)
            .sum();
        if pdc_samples == 0 {
            return Err("q128.pdc".to_owned());
        }
        let transcript = Arc::new(Transcript::new(transcript_capacity));
        let envelope = artifact.envelope();
        let nodes = artifact
            .external_binding_nodes()
            .cloned()
            .map(|node| {
                let processor: Box<dyn GraphRuntimeProcessor> = match &node {
                    GraphNodeId::TrackStage {
                        track_id,
                        stage: TrackStage::Input,
                    } => Box::new(Source {
                        left: 0.11 + track_index(track_id.as_str()) as f32 * 0.017,
                        right: -0.19 - track_index(track_id.as_str()) as f32 * 0.013,
                        phase: 0,
                    }),
                    _ => Box::new(Identity),
                };
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let observers = vec![
            observer_binding(
                "qtrack01",
                TrackStage::PostSimd1,
                OBSERVER_POST_SIMD1,
                17,
                Arc::clone(&transcript),
            ),
            observer_binding(
                "qtrack11",
                TrackStage::PostMatrix,
                OBSERVER_POST_MATRIX,
                91,
                Arc::clone(&transcript),
            ),
        ];
        let bound = artifact
            .into_bound_native(
                GraphRuntimeBindings {
                    envelope,
                    nodes,
                    observers,
                },
                NativeGraphBindConfigV1 {
                    render_mode: render_mode.native_mode(),
                    scheduler: NativeSchedulerConfigV1 {
                        render_lanes: NonZeroUsize::new(render_lanes)
                            .ok_or_else(|| "q128.lanes".to_owned())?,
                        enabled: true,
                    },
                    maximum_retained_bytes: 1 << 29,
                },
            )
            .map_err(|failure| failure.code.to_owned())?;
        let metadata = bound.prepared.metadata;
        Ok(PreparedQ128Fixture {
            plan: bound.prepared.into_plan(),
            metadata,
            report,
            pdc_samples,
            transcript,
        })
    }

    fn observer_binding(
        track_id: &str,
        stage: TrackStage,
        node_token: u64,
        handle: u64,
        transcript: Arc<Transcript>,
    ) -> GraphNodeObserverBinding {
        GraphNodeObserverBinding::new(
            GraphNodeId::TrackStage {
                track_id: miso_engine_graph::StableGraphId::parse(track_id)
                    .expect("fixture track ID"),
                stage,
            },
            handle,
            Box::new(Observer {
                record: Q128ObserverRecord {
                    sample_time: 0,
                    node_token,
                    handle,
                    boundary: stage as u8,
                    value_bits: 0,
                },
                transcript,
            }),
        )
    }

    fn delay_effect(index: usize, sidechain: SidechainDeclaration) -> miso_engine_session::Effect {
        miso_engine_session::Effect {
            id: stable("qdelay"),
            identity: EffectIdentity::Native {
                effect_id: stable("conformance.delay"),
            },
            quality: miso_engine_session::EffectQuality::Normal,
            bypass: false,
            link_mode: miso_engine_session::LinkMode::DualMono,
            params: vec![
                EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Left,
                    unit: ParameterUnit::Linear,
                    value: 0.8 + index as f32 * 0.025,
                },
                EffectParam {
                    parameter_id: 1,
                    channel: ParameterChannel::Right,
                    unit: ParameterUnit::Linear,
                    value: 1.2 - index as f32 * 0.02,
                },
            ],
            sidechain,
        }
    }

    fn track_index(track_id: &str) -> usize {
        track_id
            .strip_prefix("qtrack")
            .and_then(|value| value.parse().ok())
            .expect("fixture track ID")
    }

    fn stable(value: &str) -> StableId {
        StableId::parse(value).expect("static fixture ID")
    }

    fn unbounded_session_caps() -> CompileCaps {
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        }
    }

    fn effect_caps() -> EffectCompileCaps {
        EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 32,
        }
    }

    fn builtin_caps() -> BuiltinCompileCaps {
        BuiltinCompileCaps {
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

    fn graph_caps() -> miso_engine_graph::GraphCompileCaps {
        miso_engine_graph::GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 100_000_000,
            maximum_plan_bytes: 200_000_000,
            maximum_single_allocation_bytes: 100_000_000,
            maximum_finite_tail_samples: 10_000_000,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
    const BLOCKS: u64 = 3;
    const OBSERVERS_PER_BLOCK: usize = 2;

    #[test]
    fn q128_is_byte_identical_at_every_launch_rate_and_lane_count() {
        for rate in RATES {
            let mut sequential = fixture(rate, 1, Q128RenderMode::Sequential, 39_001);
            let mut two_lane = fixture(rate, 2, Q128RenderMode::DependencyWaves, 39_002);
            let mut four_lane = fixture(rate, 4, Q128RenderMode::DependencyWaves, 39_003);
            assert_eq!(sequential.report.sha256, two_lane.report.sha256);
            assert_eq!(sequential.report.sha256, four_lane.report.sha256);
            assert_eq!(sequential.pdc_samples, two_lane.pdc_samples);
            assert_eq!(sequential.pdc_samples, four_lane.pdc_samples);
            assert!(sequential.pdc_samples > 0);

            for block in 0..BLOCKS {
                let absolute_sample = block * Q128_QUANTUM_FRAMES as u64;
                let sequential_pcm = render(&mut sequential, absolute_sample);
                let two_lane_pcm = render(&mut two_lane, absolute_sample);
                let four_lane_pcm = render(&mut four_lane, absolute_sample);
                assert_pcm_eq(&sequential_pcm, &two_lane_pcm, rate, block, "two_lane");
                assert_pcm_eq(&sequential_pcm, &four_lane_pcm, rate, block, "four_lane");
            }

            assert_eq!(
                sequential.plan.qualification_counters(),
                two_lane.plan.qualification_counters(),
                "builtin qualification counters at {rate} Hz two lane"
            );
            assert_eq!(
                sequential.plan.qualification_counters(),
                four_lane.plan.qualification_counters(),
                "builtin qualification counters at {rate} Hz four lane"
            );
            assert_eq!(
                sequential.observer_records(),
                two_lane.observer_records(),
                "observer transcript at {rate} Hz two lane"
            );
            assert_eq!(
                sequential.observer_records(),
                four_lane.observer_records(),
                "observer transcript at {rate} Hz four lane"
            );
            assert_eq!(
                sequential.observer_record_count(),
                BLOCKS as usize * OBSERVERS_PER_BLOCK,
                "complete observer transcript at {rate} Hz"
            );
        }
    }

    fn fixture(rate: u32, lanes: usize, mode: Q128RenderMode, plan_id: u64) -> PreparedQ128Fixture {
        prepare_q128_fixture(
            rate,
            lanes,
            mode,
            plan_id,
            BLOCKS as usize * OBSERVERS_PER_BLOCK,
        )
        .unwrap_or_else(|error| panic!("q128 fixture preparation failed at {rate} Hz: {error}"))
    }

    fn render(plan: &mut PreparedQ128Fixture, absolute_sample: u64) -> Vec<f32> {
        let mut pcm = vec![0.0_f32; Q128_QUANTUM_FRAMES * 2];
        plan.render(&mut pcm, absolute_sample)
            .unwrap_or_else(|error| panic!("q128 render failed: {error:?}"));
        pcm
    }

    fn assert_pcm_eq(left: &[f32], right: &[f32], rate: u32, block: u64, mode: &str) {
        assert_eq!(left.len(), right.len());
        for (index, (left, right)) in left.iter().zip(right).enumerate() {
            assert_eq!(
                left.to_bits(),
                right.to_bits(),
                "PCM differs at {rate} Hz, block {block}, {mode}, sample {index}"
            );
        }
    }
}
