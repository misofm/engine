//! Shared Issue-039 q128 production-fixture preparation.
//!
//! The fixture is a qualification harness, not a graph or scheduler implementation API.  It
//! builds the one frozen production graph exclusively through the ordinary session, effect,
//! builtin, graph-compiler, and native-binding boundaries.

#![allow(missing_docs)]

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use core::num::NonZeroUsize;
    use miso_engine_graph_compiler::Backend;
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
        NativeGraphWorkerLeaseV1, NativeGraphWorkerPoolV1, NativeSchedulerConfigV1,
        NativeWorkerPoolConfigV1, NativeWorkerPoolShapeV1, TrackStage,
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

    /// Read-only handle for the fixture's preallocated observer transcript after render disarms.
    #[derive(Clone)]
    pub struct Q128ObserverTranscript {
        transcript: Arc<Transcript>,
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

        /// Records in a canonical order.
        ///
        /// Since issue 100 an observer runs on the worker that rendered its node, so the
        /// *arrival* order across nodes in different parcels is unspecified by design: an
        /// observer is still invoked exactly once per block per node, and its own audio is
        /// exactly what the sequential executor sees. The transcript is therefore sorted by
        /// `(sample_time, node_token, handle)` before anything compares or hashes it.
        fn records(&self) -> Vec<Q128ObserverRecord> {
            let capacity = self.fields.len() / 5;
            let count = self.next.load(Ordering::Relaxed).min(capacity);
            let mut records: Vec<Q128ObserverRecord> = (0..count)
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
                .collect();
            records.sort_by_key(|record| (record.sample_time, record.node_token, record.handle));
            records
        }

        fn count(&self) -> usize {
            self.next.load(Ordering::Relaxed)
        }

        /// Address-free hash of the canonical (sorted) transcript. See [`Transcript::records`].
        fn stable_hash(&self) -> u64 {
            let mut hash = 0xcbf2_9ce4_8422_2325_u64;
            for record in self.records() {
                for value in [
                    record.sample_time,
                    record.node_token,
                    record.handle,
                    u64::from(record.boundary),
                    u64::from(record.value_bits),
                ] {
                    for byte in value.to_le_bytes() {
                        hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
                    }
                }
            }
            hash
        }
    }

    impl Q128ObserverTranscript {
        /// Number of records written into fixed transcript storage.
        #[must_use]
        pub fn record_count(&self) -> usize {
            self.transcript.count()
        }

        /// Address-free stable hash of the completed observer transcript.
        #[must_use]
        pub fn stable_hash(&self) -> u64 {
            self.transcript.stable_hash()
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
        /// The fixture's own worker pool. It outlives the plan, so it is dropped last.
        pub pool: Option<NativeGraphWorkerPoolV1>,
        pub metadata: NativeGraphPreparedMetadataV1,
        pub report: GraphCompileReport,
        /// The semantic graph hash, taken while the artifact still owned its plan (#99 F5): the
        /// report no longer carries it, and the plan is consumed into `plan` on the way out.
        pub graph_sha256: String,
        pub pdc_samples: u64,
        pub prepared_builtin_bank_count: usize,
        pub prepared_builtin_bank_member_count: usize,
        /// Lanes of the host-selected builtin bank width, or `0` when the host has none.
        pub prepared_builtin_bank_lanes: usize,
        pub scalar_builtin_tail_count: usize,
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

        /// Clone a read-only transcript handle before moving this fixture plan into an exchange.
        #[must_use]
        pub fn observer_transcript(&self) -> Q128ObserverTranscript {
            Q128ObserverTranscript {
                transcript: Arc::clone(&self.transcript),
            }
        }
    }

    pub fn prepare_q128_fixture(
        sample_rate_hz: u32,
        render_lanes: usize,
        render_mode: Q128RenderMode,
        plan_id: u64,
        transcript_capacity: usize,
    ) -> Result<PreparedQ128Fixture, String> {
        prepare_fixture_with_track_count(
            sample_rate_hz,
            render_lanes,
            render_mode,
            plan_id,
            transcript_capacity,
            PoolChoice::Own,
            Q128_TRACK_COUNT,
            1 << 29,
            true,
        )
    }

    /// Where a prepared fixture's auxiliary workers come from.
    pub enum PoolChoice {
        /// The fixture starts and owns its own pool.
        Own,
        /// The caller owns the pool. A fixture prepared without a lease renders sequentially
        /// until the block-boundary hand-over gives it one.
        External(NativeWorkerPoolShapeV1, Option<NativeGraphWorkerLeaseV1>),
    }

    /// Prepare the q128 graph against a caller-owned worker pool.
    ///
    /// The audit uses this to prove one persistent pool serves two successive plans: the initial
    /// plan holds the lease, the replacement is prepared without one, and the swap hands it over.
    #[doc(hidden)]
    pub fn prepare_q128_fixture_with_pool(
        sample_rate_hz: u32,
        render_lanes: usize,
        render_mode: Q128RenderMode,
        plan_id: u64,
        transcript_capacity: usize,
        pool: PoolChoice,
    ) -> Result<PreparedQ128Fixture, String> {
        prepare_fixture_with_track_count(
            sample_rate_hz,
            render_lanes,
            render_mode,
            plan_id,
            transcript_capacity,
            pool,
            Q128_TRACK_COUNT,
            1 << 29,
            true,
        )
    }

    /// Prepare the same production fixture topology at one generated track count for the
    /// scheduler preparation matrix. This is qualification-only support, not a product graph API.
    #[doc(hidden)]
    pub fn prepare_q128_fixture_for_track_count(
        track_count: usize,
        render_lanes: usize,
        plan_id: u64,
        maximum_retained_bytes: usize,
    ) -> Result<PreparedQ128Fixture, String> {
        prepare_fixture_with_track_count(
            48_000,
            render_lanes,
            Q128RenderMode::DependencyWaves,
            plan_id,
            0,
            PoolChoice::Own,
            track_count,
            maximum_retained_bytes,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn prepare_fixture_with_track_count(
        sample_rate_hz: u32,
        render_lanes: usize,
        render_mode: Q128RenderMode,
        plan_id: u64,
        transcript_capacity: usize,
        pool_choice: PoolChoice,
        track_count: usize,
        maximum_retained_bytes: usize,
        require_q128_bank_and_pdc: bool,
    ) -> Result<PreparedQ128Fixture, String> {
        if track_count == 0 {
            return Err("q128.track_count".to_owned());
        }
        let mut model = parse_session_toml(SESSION_FIXTURE).map_err(|_| "q128.parse".to_owned())?;
        model.sample_rate_hz = sample_rate_hz;
        model.quantum_frames = Q128_QUANTUM_FRAMES as u32;
        model.sources[0].sample_rate_hz = sample_rate_hz;
        model.automation.clear();
        let base_track = model.tracks[0].clone();
        let base_route = model.routes[0].clone();
        model.tracks = (0..track_count)
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
                if index != 0 && index != track_count - 1 {
                    track
                        .simd1
                        .effects
                        .push(delay_effect(index, SidechainDeclaration::None));
                }
                if track_count > 1 && index == track_count - 1 {
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
            dispatch: Backend::current(),
            plan_id,
            effects,
            builtins,
            caps: graph_caps(),
        })
        .map_err(|_| "q128.graph".to_owned())?;
        let prepared_builtin_bank_count = artifact.prepared_builtin_bank_count();
        let prepared_builtin_bank_member_count = artifact
            .prepared_builtin_banks()
            .map(|bank| bank.members.len())
            .sum();
        let prepared_builtin_bank_lanes = artifact
            .prepared_builtin_banks()
            .next()
            .map_or(0, |bank| bank.width.lanes() as usize);
        let scalar_builtin_tail_count = track_count
            .checked_sub(prepared_builtin_bank_member_count)
            .ok_or_else(|| "q128.builtin_bank_members".to_owned())?;
        if require_q128_bank_and_pdc && prepared_builtin_bank_count == 0 {
            return Err("q128.builtin_bank".to_owned());
        }
        let report = artifact.report().clone();
        let graph_sha256 = GraphCompiler::sha256(artifact.graph(), artifact.report());
        // #99 F5: the plan owns the schedule vectors; the report no longer duplicates them.
        let pdc_samples = artifact
            .graph()
            .inserted_delays
            .iter()
            .map(|delay| delay.samples.0)
            .sum();
        if require_q128_bank_and_pdc && pdc_samples == 0 {
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
        let post_simd_track = if track_count > 1 {
            "qtrack01"
        } else {
            "qtrack00"
        };
        let post_matrix_track = format!("qtrack{:02}", track_count - 1);
        let observers = vec![
            observer_binding(
                post_simd_track,
                TrackStage::PostSimd1,
                OBSERVER_POST_SIMD1,
                17,
                Arc::clone(&transcript),
            ),
            observer_binding(
                &post_matrix_track,
                TrackStage::PostMatrix,
                OBSERVER_POST_MATRIX,
                91,
                Arc::clone(&transcript),
            ),
        ];
        let lanes = NonZeroUsize::new(render_lanes).ok_or_else(|| "q128.lanes".to_owned())?;
        let (pool, lease, pool_shape) = match (pool_choice, render_mode) {
            (PoolChoice::External(shape, lease), _) => (None, lease, shape),
            (PoolChoice::Own, Q128RenderMode::Sequential) => {
                (None, None, NativeWorkerPoolShapeV1::default())
            }
            (PoolChoice::Own, Q128RenderMode::DependencyWaves) => {
                // The fixture owns its own pool, so two fixtures in one process share nothing.
                // The fixture accepts absurd lane counts so binding can reject them; it must not
                // try to start that many threads.
                let requested = render_lanes.saturating_sub(1).min(
                    std::thread::available_parallelism()
                        .map(NonZeroUsize::get)
                        .unwrap_or(1),
                );
                let (pool, lease) = NativeGraphWorkerPoolV1::start(NativeWorkerPoolConfigV1 {
                    requested_workers: NonZeroUsize::new(requested),
                    ..NativeWorkerPoolConfigV1::default()
                })
                .map_err(|_| "q128.pool".to_owned())?;
                let shape = pool.shape();
                (Some(pool), Some(lease), shape)
            }
        };
        let bound = artifact
            .into_bound_native(
                GraphRuntimeBindings {
                    #[cfg(not(target_arch = "wasm32"))]
                    worker_lease: lease,
                    envelope,
                    nodes,
                    observers,
                },
                NativeGraphBindConfigV1 {
                    render_mode: render_mode.native_mode(),
                    // The fixture is a determinism/allocation harness: a descheduled worker must
                    // never be mistaken for a dead one here. The bounded recovery deadline is
                    // proved by fault injection in `miso-engine-graph`, not by this fixture.
                    scheduler: NativeSchedulerConfigV1::new(lanes, true, pool_shape)
                        .with_recovery_deadline_ns(5_000_000_000),
                    maximum_retained_bytes,
                },
            )
            .map_err(|failure| failure.code.to_owned())?;
        let metadata = bound.prepared.metadata;
        Ok(PreparedQ128Fixture {
            plan: bound.prepared.into_plan(),
            pool,
            metadata,
            report,
            graph_sha256,
            pdc_samples,
            prepared_builtin_bank_count,
            prepared_builtin_bank_member_count,
            prepared_builtin_bank_lanes,
            scalar_builtin_tail_count,
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
    use miso_engine_graph::{FallbackReasonV1, SchedulerSelectionV1};

    const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
    const BLOCKS: u64 = 3;
    const OBSERVERS_PER_BLOCK: usize = 2;
    const PREPARATION_TRACK_COUNTS: [usize; 6] = [1, 3, 4, 5, 12, 17];

    /// E1. The production q128 session renders identical bits at every launch rate and every
    /// worker-lane count, PCM and observer transcript alike.
    ///
    /// Red mutation (`MUTATIONS.md`): return from `recover_issued` before popping any completion
    /// -- the coordinator reads a parcel the worker is still writing and the PCM diverges.
    #[test]
    fn q128_is_byte_identical_at_every_launch_rate_and_lane_count() {
        let cores = std::thread::available_parallelism()
            .map(core::num::NonZeroUsize::get)
            .unwrap_or(1);
        let lane_counts: Vec<usize> = [2_usize, 4, 8]
            .into_iter()
            .filter(|lanes| {
                if cores >= *lanes {
                    true
                } else {
                    eprintln!("skipping the {lanes}-lane column: {cores} cores available");
                    false
                }
            })
            .collect();
        assert!(
            lane_counts.len() >= 2,
            "the lane matrix needs at least the two- and four-lane columns"
        );
        for rate in RATES {
            let mut sequential = fixture(rate, 1, Q128RenderMode::Sequential, 39_001);
            assert!(sequential.pdc_samples > 0);
            let mut parallel: Vec<(usize, PreparedQ128Fixture)> = lane_counts
                .iter()
                .enumerate()
                .map(|(index, lanes)| {
                    (
                        *lanes,
                        fixture(
                            rate,
                            *lanes,
                            Q128RenderMode::DependencyWaves,
                            39_002 + index as u64,
                        ),
                    )
                })
                .collect();
            for (lanes, candidate) in &parallel {
                assert_eq!(
                    sequential.graph_sha256, candidate.graph_sha256,
                    "{lanes} lanes: the semantic graph must not depend on the lane count"
                );
                assert_eq!(sequential.pdc_samples, candidate.pdc_samples);
            }

            for block in 0..BLOCKS {
                let absolute_sample = block * Q128_QUANTUM_FRAMES as u64;
                let sequential_pcm = render(&mut sequential, absolute_sample);
                for (lanes, candidate) in parallel.iter_mut() {
                    let pcm = render(candidate, absolute_sample);
                    assert_pcm_eq(&sequential_pcm, &pcm, rate, block, &format!("{lanes}_lane"));
                }
            }

            for (lanes, candidate) in &parallel {
                assert_eq!(
                    candidate.plan.dispatch_counters()[1],
                    0,
                    "{lanes} lanes at {rate} Hz: a worker missed its deadline, so this \
                     comparison would be measuring a degraded block"
                );
                assert_eq!(
                    sequential.plan.qualification_counters(),
                    candidate.plan.qualification_counters(),
                    "builtin qualification counters at {rate} Hz, {lanes} lanes"
                );
                assert_eq!(
                    sequential.observer_records(),
                    candidate.observer_records(),
                    "observer transcript at {rate} Hz, {lanes} lanes"
                );
                assert_eq!(
                    candidate.observer_record_count(),
                    BLOCKS as usize * OBSERVERS_PER_BLOCK,
                    "complete observer transcript at {rate} Hz, {lanes} lanes"
                );
            }
            assert_eq!(
                sequential.observer_record_count(),
                BLOCKS as usize * OBSERVERS_PER_BLOCK,
                "complete observer transcript at {rate} Hz"
            );
        }
    }

    /// E5, descriptive. Renders the production q128 session at 1/2/4/8 lanes and prints the mean
    /// and worst nanoseconds per block plus the implied serial fraction. It is not a gate: it is
    /// run once before and once after the change and both tables are pasted into the issue.
    ///
    /// Nothing is hashed inside a timed interval and no plan is prepared inside one.
    #[test]
    #[ignore = "descriptive measurement, run explicitly"]
    fn q128_speedup_descriptive() {
        const WARMUP: u64 = 2_000;
        const MEASURED: u64 = 20_000;
        let cores = std::thread::available_parallelism()
            .map(core::num::NonZeroUsize::get)
            .unwrap_or(1);
        let mut baseline_ns = 0.0_f64;
        println!("lanes  mean ns/block  worst ns/block  speed-up  serial fraction");
        for lanes in [1_usize, 2, 4, 8] {
            if lanes > cores {
                println!("{lanes:>5}  (skipped: {cores} cores)");
                continue;
            }
            let mode = if lanes == 1 {
                Q128RenderMode::Sequential
            } else {
                Q128RenderMode::DependencyWaves
            };
            // The transcript is preallocated for every observation this run makes, so no render
            // inside a timed interval can fail on capacity.
            let mut prepared = prepare_q128_fixture(
                48_000,
                lanes,
                mode,
                41_000 + lanes as u64,
                (WARMUP + MEASURED) as usize * OBSERVERS_PER_BLOCK,
            )
            .unwrap_or_else(|error| panic!("speed-up fixture: {error}"));
            let mut pcm = vec![0.0_f32; Q128_QUANTUM_FRAMES * 2];
            for block in 0..WARMUP {
                prepared
                    .render(&mut pcm, block * Q128_QUANTUM_FRAMES as u64)
                    .expect("warm-up render");
            }
            let mut worst = 0_u128;
            let started = std::time::Instant::now();
            for block in 0..MEASURED {
                let block_start = std::time::Instant::now();
                prepared
                    .render(&mut pcm, (WARMUP + block) * Q128_QUANTUM_FRAMES as u64)
                    .expect("measured render");
                worst = worst.max(block_start.elapsed().as_nanos());
            }
            let mean = started.elapsed().as_nanos() as f64 / MEASURED as f64;
            if lanes == 1 {
                baseline_ns = mean;
            }
            let speedup = baseline_ns / mean;
            let workers = lanes as f64;
            let serial = if lanes == 1 || speedup <= 0.0 {
                f64::NAN
            } else {
                (workers / speedup - 1.0) / (workers - 1.0)
            };
            println!("{lanes:>5}  {mean:>13.1}  {worst:>14}  {speedup:>8.2}  {serial:>15.3}");
            assert_eq!(
                prepared.plan.dispatch_counters()[1],
                0,
                "{lanes} lanes: a deadline miss would make this measurement meaningless"
            );
        }
    }

    #[test]
    fn q128_preparation_matrix_is_exact_for_100_runs_and_generated_track_counts() {
        let mut transcript_by_count = [None; PREPARATION_TRACK_COUNTS.len()];
        let mut resources_by_count = [None; PREPARATION_TRACK_COUNTS.len()];
        let mut count_observations = [0_u8; PREPARATION_TRACK_COUNTS.len()];
        let mut reference = None;
        let mut aggregate_hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut accepted_preparations = 0_u64;
        for preparation in 0..100_u64 {
            let count_index = preparation as usize % PREPARATION_TRACK_COUNTS.len();
            let track_count = PREPARATION_TRACK_COUNTS[count_index];
            let prepared = matrix_fixture(track_count, 4, 39_500 + preparation, usize::MAX);
            accepted_preparations = accepted_preparations.saturating_add(1);
            count_observations[count_index] = count_observations[count_index].saturating_add(1);
            let transcript = prepared.metadata.test_preparation_transcript;
            if let Some(expected) = transcript_by_count[count_index] {
                assert_eq!(
                    transcript, expected,
                    "fresh preparation {preparation} changed the immutable transcript"
                );
            } else {
                transcript_by_count[count_index] = Some(transcript);
            }
            if let Some(expected) = resources_by_count[count_index] {
                assert_eq!(
                    prepared.metadata.resources, expected,
                    "fresh preparation {preparation} changed retained resource accounting"
                );
            } else {
                resources_by_count[count_index] = Some(prepared.metadata.resources);
            }
            assert_eq!(
                prepared.prepared_builtin_bank_member_count + prepared.scalar_builtin_tail_count,
                track_count,
                "retained builtin banks and scalar tails cover every track"
            );
            // #86 F3: on a vector host every post-input node is a bank member, the last bank of
            // the level is padded with identity lanes, and no scalar tail survives.  Hand table
            // for [1, 3, 4, 5, 12, 17]: W8 -> [1, 1, 1, 1, 2, 3], W4 -> [1, 1, 1, 2, 3, 5].
            if prepared.prepared_builtin_bank_lanes == 0 {
                assert_eq!(prepared.prepared_builtin_bank_count, 0);
                assert_eq!(prepared.scalar_builtin_tail_count, track_count);
            } else {
                assert_eq!(
                    prepared.prepared_builtin_bank_count,
                    track_count.div_ceil(prepared.prepared_builtin_bank_lanes),
                    "padded bank count for track count {track_count}"
                );
                assert_eq!(
                    prepared.prepared_builtin_bank_member_count, track_count,
                    "every track is a bank member for track count {track_count}"
                );
                assert_eq!(
                    prepared.scalar_builtin_tail_count, 0,
                    "no scalar post-input tail for track count {track_count}"
                );
            }
            assert_eq!(
                transcript.retained_builtin_bank_units, prepared.prepared_builtin_bank_count,
                "builtin banks stay indivisible for track count {track_count}"
            );
            assert_eq!(
                transcript.retained_builtin_bank_members,
                prepared.prepared_builtin_bank_member_count,
                "builtin bank membership stays intact for track count {track_count}"
            );
            assert!(
                transcript.partitions_are_canonical,
                "track count {track_count}"
            );
            // Issue 100 F2: the split balances prepared cost, not unit count. The greedy
            // longest-processing-time-first rule only ever places a unit on the least-loaded bin,
            // whose load is at most the mean at that moment, so no bin can end heavier than
            // `ceil(total / bins) + heaviest unit`. A count split violates this in every wave
            // that mixes an eight-member bank with scalar tails.
            assert!(
                transcript.weighted_partitions_are_balanced,
                "cost-weighted partitions for track count {track_count}"
            );
            // An independent lower bound on the folded cost: every track input weighs at least
            // one, and every post-input builtin-bank member contributes four processing slots
            // (polarity/trim, HPF, LPF, trim/mute) plus its one incoming edge.
            assert!(
                transcript.total_unit_weight
                    >= (track_count + 5 * prepared.prepared_builtin_bank_member_count) as u64,
                "prepared cost for track count {track_count} is below its structural floor"
            );
            assert_resource_accounting(&prepared);
            if track_count == 1 {
                assert_eq!(transcript.largest_wave_width, 1);
                assert_eq!(
                    prepared.metadata.selection,
                    SchedulerSelectionV1::Sequential(FallbackReasonV1::InsufficientWaveWidth)
                );
            } else {
                assert_eq!(prepared.metadata.selection, SchedulerSelectionV1::Parallel);
            }
            aggregate_hash = preparation_matrix_hash(aggregate_hash, track_count, &prepared);
            if track_count == 12 && reference.is_none() {
                reference = Some(prepared);
            }
        }

        assert_eq!(accepted_preparations, 100);
        assert!(count_observations.iter().all(|count| *count != 0));
        let q128_transcript = transcript_by_count[4].expect("twelve-track transcript");
        // Both constants are re-derived structurally, twice.
        //
        // #86 F3 moved the builtin bank/tail counts these transcripts fold (12 tracks at W8:
        // 1 full bank + 4 scalar tails -> 2 banks, the second padded, 0 tails; the table above
        // is the hand count for every track count).
        //
        // #98 F2/F5 moves them again: the native executor is now built from the lowered
        // `ExecutionProgram`, so a `TrackStage` boundary that is a pure alias carries no op and
        // therefore no scheduling unit, and a dependency level made only of aliases carries no
        // wave. Twelve tracks lose their three internal rack boundaries each, so the unit count
        // this hash folds falls by 36 and the retained wave count falls with it; `graph_job_bytes`
        // (folded by the aggregate) falls for the same reason. Neither hash is pinned from
        // production output: every structural quantity the transcript reports is asserted against
        // an independently hand-counted expectation immediately above -- retained builtin bank
        // units and members equal the prepared counts, partitions stay canonical,
        // `largest_wave_width` is 1 for a single track -- and `assert_resource_accounting` checks
        // the byte report against its own arithmetic. The PCM gates in this crate --
        // `q128_is_byte_identical_at_every_launch_rate_and_lane_count` and the perturbation
        // suite -- are unchanged and green.
        //
        // #100 F2 moves them once more: a wave's units are now ordered by the cost-weighted
        // longest-processing-time-first split rather than by unit key, and the transcript folds
        // each unit's prepared weight. Neither hash is pinned from production output: every
        // structural quantity the transcript reports is asserted against an independent
        // expectation immediately above -- retained builtin bank units and members equal the
        // prepared counts, partitions stay canonical *and* cost-balanced (no bin heavier than
        // `ceil(total / bins) + heaviest unit`, which a count split cannot satisfy), the folded
        // weight clears its structural floor, and `largest_wave_width` is 1 for a single track.
        // The PCM gates are unchanged and green: the q128 lane matrix now runs 1/2/4/8 lanes at
        // every launch rate, and `miso-engine-graph`'s 50 generated DAGs run 1/2/4/7.
        //
        //   q128 transcript: 0x1364_823e_5403_eca7 -> 0x645b_3eb0_778d_96dd -> 0x49ff_221a_5d9f_385e
        //   aggregate:       0xebbc_a7d9_be93_d1ca -> 0x386f_8720_9810_7e32
        //                    -> 0x1ba7_2d17_1383_6e52 (#103's plan clock) -> 0xff58_81b9_d2b5_42d9
        assert_eq!(
            q128_transcript.hash, 0x49ff_221a_5d9f_385e,
            "frozen q128 native wave/unit/partition transcript"
        );
        // Re-derived structurally again for audit #103 W4-4: the matrix folds
        // `resources.total_retained_bytes`, and `PreparedRenderPlan` gained one `u64` when the
        // sample clock moved into the plan.  The q128 transcript above is unaffected, and every
        // PCM gate in this crate is unchanged and green.
        //
        //   aggregate: 0x386f_8720_9810_7e32 -> 0x1ba7_2d17_1383_6e52
        //
        // Re-derived structurally for #84 phases B/C: the matrix folds
        // `resources.total_retained_bytes`, the ring header grew 72 -> 256 with one cache line
        // per cursor, each endpoint carries a cached peer cursor (+8), and the plan dropped its
        // unused parameter/event store (-96). The q128 transcript above is again unaffected.
        //
        //   aggregate: 0xff58_81b9_d2b5_42d9 -> 0x9f68_63fb_bffe_3301
        assert_eq!(
            aggregate_hash, 0x9f68_63fb_bffe_3301,
            "frozen exact-100 preparation matrix transcript after the cost-weighted split"
        );
        let reference = reference.expect("one twelve-track preparation");
        assert!(reference.prepared_builtin_bank_count > 0);
        assert_eq!(
            reference.scalar_builtin_tail_count, 0,
            "twelve tracks are two padded banks on a vector host, with no scalar tail"
        );

        let cap = reference
            .metadata
            .resources
            .total_retained_bytes
            .checked_sub(1)
            .expect("prepared native graph retained bytes");
        let cap_error = prepare_q128_fixture_for_track_count(12, 4, 40_101, cap)
            .err()
            .expect("one-byte-short native retained cap must reject");
        assert_eq!(cap_error, "graph.scheduler.cap");

        // An absurd lane request is now clamped by the pool the host actually started rather
        // than overflowing: effective lanes are `pool workers + 1`. The checked-overflow path
        // itself is gated in `miso-engine-native-scheduler`
        // (`an_impossible_pool_shape_is_rejected_before_publication`).
        let clamped = prepare_q128_fixture_for_track_count(12, usize::MAX, 40_102, usize::MAX)
            .expect("an oversized lane request is clamped to the started pool");
        assert!(
            clamped.metadata.resources.scheduler.selected_lanes
                <= std::thread::available_parallelism()
                    .map(core::num::NonZeroUsize::get)
                    .unwrap_or(1)
                    + 1
        );
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

    fn matrix_fixture(
        track_count: usize,
        lanes: usize,
        plan_id: u64,
        maximum_retained_bytes: usize,
    ) -> PreparedQ128Fixture {
        prepare_q128_fixture_for_track_count(track_count, lanes, plan_id, maximum_retained_bytes)
            .unwrap_or_else(|error| {
                panic!("matrix fixture preparation failed for {track_count} tracks: {error}")
            })
    }

    fn assert_resource_accounting(prepared: &PreparedQ128Fixture) {
        let resources = prepared.metadata.resources;
        assert_eq!(
            resources.total_retained_bytes,
            resources
                .graph_job_bytes
                .checked_add(resources.scheduler.retained_queue_bytes)
                .expect("preflighted retained resource sum")
        );
        assert!(resources.graph_job_bytes > 0);
        assert!(resources.scheduler.unit_count > 0);
        assert!(resources.scheduler.partition_count > 0);
    }

    fn preparation_matrix_hash(
        mut hash: u64,
        track_count: usize,
        prepared: &PreparedQ128Fixture,
    ) -> u64 {
        let transcript = prepared.metadata.test_preparation_transcript;
        let resources = prepared.metadata.resources;
        for value in [
            track_count as u64,
            transcript.hash,
            transcript.largest_wave_width as u64,
            transcript.retained_bank_units as u64,
            transcript.retained_bank_members as u64,
            transcript.retained_builtin_bank_units as u64,
            transcript.retained_builtin_bank_members as u64,
            u64::from(transcript.partitions_are_canonical),
            resources.scheduler.selected_lanes as u64,
            resources.scheduler.worker_count as u64,
            resources.scheduler.wave_count as u64,
            resources.scheduler.unit_count as u64,
            resources.scheduler.partition_count as u64,
            resources.scheduler.retained_queue_bytes as u64,
            resources.graph_job_bytes as u64,
            resources.total_retained_bytes as u64,
            prepared.prepared_builtin_bank_count as u64,
            prepared.prepared_builtin_bank_member_count as u64,
            prepared.scalar_builtin_tail_count as u64,
        ] {
            for byte in value.to_le_bytes() {
                hash = (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        hash
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
