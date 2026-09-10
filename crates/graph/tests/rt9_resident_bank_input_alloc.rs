//! Actual prepared-plan allocation and differential evidence for adjacent resident input.

use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
use effect_contract::{BankWidth, ChannelSymmetryWitness, LatencySamples, TailSamples};
use engine::{
    QuantumFrames,
    realtime::{self, PlanarBufferMut, PreparedRenderPlan, RenderEnvelope, RenderError},
};
use graph::*;
use lane::Backend;
use std::sync::{
    Arc,
    atomic::{AtomicU32, AtomicU64, Ordering::Relaxed},
};

#[derive(Default)]
struct Probe {
    trace: AtomicU64,
    command: AtomicU32,
    observed: [AtomicU32; 4],
}
impl Probe {
    fn event(&self, event: u64) {
        self.trace
            .store(self.trace.load(Relaxed) * 16 + event, Relaxed);
    }
    fn snapshot(&self) -> (u64, u32, [u32; 4]) {
        (
            self.trace.load(Relaxed),
            self.command.load(Relaxed),
            self.observed.each_ref().map(|v| v.load(Relaxed)),
        )
    }
}

struct Source {
    lane: usize,
    mono: bool,
}
impl GraphRuntimeProcessor for Source {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        for (frame, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            *left = if frame == 0 {
                -0.0
            } else {
                (self.lane + frame + block.first_sample as usize) as f32 * 0.03125
            };
            *right = if self.mono { *left } else { -*left * 0.5 };
        }
        Ok(())
    }
}

struct Stage {
    id: usize,
    lanes: usize,
    population: usize,
    state: [f32; 2],
    mono: bool,
    fail: u32,
    probe: Arc<Probe>,
}
impl Stage {
    fn apply(
        &mut self,
        left: &mut [f32],
        right: Option<&mut [f32]>,
        first: u64,
    ) -> Result<(), RenderError> {
        self.probe.event(2 + self.id as u64 * 4);
        if first == 0 && self.fail == 2 + self.id as u32 * 2 {
            return Err(RenderError::InvalidEnvelope);
        }
        let delta = self.state[0];
        for frame in left.chunks_exact_mut(self.lanes) {
            for word in &mut frame[..self.population] {
                *word += delta;
            }
        }
        self.state[0] += 0.015625;
        if let Some(right) = right {
            for frame in right.chunks_exact_mut(self.lanes) {
                for word in &mut frame[..self.population] {
                    *word += self.state[1];
                }
            }
            self.state[1] += 0.015625;
        }
        Ok(())
    }
}
impl GraphPreparedBuiltinBankProcessor for Stage {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn begin_block(&mut self, first: u64) -> Result<(), RenderError> {
        self.probe.event(1 + self.id as u64 * 4);
        if self.id == 1 {
            let command = self.probe.command.swap(0, Relaxed);
            self.state[0] += command as f32 * 0.125;
            self.state[1] += command as f32 * 0.125;
        }
        if first == 0 && self.fail == 1 + self.id as u32 * 2 {
            Err(RenderError::InvalidEnvelope)
        } else {
            Ok(())
        }
    }
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        _: u32,
        first: u64,
    ) -> Result<(), RenderError> {
        self.apply(left, Some(right), first)
    }
    fn supports_mono_collapse(&self) -> bool {
        self.mono
    }
    fn lane_symmetry(&self, _: usize) -> ChannelSymmetryWitness {
        ChannelSymmetryWitness::SYMMETRIC
    }
    fn process_mono(&mut self, left: &mut [f32], _: u32, first: u64) -> Result<(), RenderError> {
        self.apply(left, None, first)
    }
    fn desymmetrize(&mut self) {
        self.state[1] = self.state[0];
    }
    fn channels_agree(&self) -> bool {
        self.state[0].to_bits() == self.state[1].to_bits()
    }
    fn qualification_counters(&self) -> [u64; 2] {
        [
            u64::from(self.state[0].to_bits()),
            u64::from(self.state[1].to_bits()),
        ]
    }
}
struct Observer {
    ordinal: usize,
    fail: bool,
    probe: Arc<Probe>,
}
impl GraphRuntimeObserver for Observer {
    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
        self.probe.event(3 + self.ordinal as u64);
        self.probe.observed[self.ordinal * 2]
            .store(block.left[block.left.len() - 1].to_bits(), Relaxed);
        self.probe.observed[self.ordinal * 2 + 1]
            .store(block.right[block.right.len() - 1].to_bits(), Relaxed);
        self.probe.command.store(1, Relaxed);
        if self.fail && block.first_sample == 0 {
            Err(RenderError::InvalidEnvelope)
        } else {
            Ok(())
        }
    }
}

fn estimate() -> GraphResourceEstimate {
    GraphResourceEstimate {
        logical_nodes: 0,
        materialized_nodes: 0,
        edges: 0,
        schedule_items: 0,
        dependency_levels: 0,
        reductions: 0,
        routes: 0,
        effects: 0,
        audio_buffer_samples: 0,
        total_delay_samples: 0,
        delay_bytes: 0,
        graph_metadata_bytes: 0,
        declared_effect_bytes: 0,
        effect_bank_count: 0,
        effect_bank_scratch_bytes: 0,
        effect_bank_runtime_buffer_bytes: 0,
        effect_bank_metadata_bytes: 0,
        builtin_bank_bytes: 0,
        builtin_bank_scratch_bytes: 0,
        builtin_bank_count: 0,
        largest_allocation_bytes: 0,
        incremental_plan_bytes: 0,
        session_plus_plan_bytes: 0,
    }
}

fn prepared(
    width: BankWidth,
    population: usize,
    frames: u32,
    alias: bool,
    mono: u8,
    fail: u32,
) -> (PreparedRenderPlan, Arc<Probe>) {
    let envelope = RenderEnvelope {
        sample_rate: engine::SampleRateHz(48_000),
        quantum: QuantumFrames(frames),
        input_channels: None,
        output_channels: core::num::NonZeroUsize::new(2).expect("dual mono"),
    };
    let node = |lane, stage| GraphNodeId::TrackStage {
        track_id: StableGraphId::parse(&format!("track{lane}")).expect("track"),
        stage,
    };
    let stages = [
        TrackStage::Input,
        TrackStage::PostInputBuiltins,
        TrackStage::PostSimd1,
        TrackStage::PostFader,
    ];
    let groups: Vec<Vec<_>> = stages
        .into_iter()
        .map(|stage| (0..population).map(|lane| node(lane, stage)).collect())
        .collect();
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("main").expect("output"),
    };
    let mut schedule: Vec<_> = groups.iter().flatten().cloned().collect();
    schedule.push(output.clone());
    let mut nodes: Vec<_> = schedule
        .iter()
        .cloned()
        .map(|id| GraphNode {
            id,
            latency: LatencySamples(0),
            tail: TailSamples::Finite(0),
        })
        .collect();
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    let mut edges = Vec::new();
    let mut connect = |source: GraphNodeId, destination: GraphNodeId, id| {
        edges.push(GraphEdge {
            id,
            source: GraphPortId {
                node: source,
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: destination,
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.rt9".to_owned(),
        })
    };
    for lane in 0..population {
        for stage in 1..4 {
            connect(
                groups[stage - 1][lane].clone(),
                groups[stage][lane].clone(),
                GraphEdgeId::TrackMain {
                    target: groups[stage][lane].clone(),
                },
            );
        }
        connect(
            groups[3][lane].clone(),
            output.clone(),
            GraphEdgeId::RouteSource {
                route_id: StableGraphId::parse(&format!("main{lane}")).expect("route"),
            },
        );
    }
    // A positive extra reader retains A's planar scatter and the output's stable reduction.
    connect(
        groups[2][0].clone(),
        output.clone(),
        GraphEdgeId::RouteSource {
            route_id: StableGraphId::parse("send").expect("send"),
        },
    );
    let probe = Arc::new(Probe::default());
    let banks = [1, 3]
        .into_iter()
        .enumerate()
        .map(|(id, group)| GraphPreparedBuiltinBank {
            backend: if width == BankWidth::Four {
                Backend::Simd4
            } else {
                Backend::Simd8
            },
            members: groups[group].clone().into_boxed_slice(),
            processor: Box::new(Stage {
                id,
                lanes: width.lanes() as usize,
                population,
                state: [0.0; 2],
                mono: mono & (1 << id) != 0,
                fail,
                probe: Arc::clone(&probe),
            }),
            scratch: rack::AoSoaScratch::new(width, frames).expect("scratch"),
        })
        .collect();
    let required: Vec<_> = groups[0]
        .iter()
        .chain(groups[1].iter())
        .chain(groups[3].iter())
        .chain(core::iter::once(&output))
        .cloned()
        .collect();
    let mut levels: Vec<_> = groups
        .iter()
        .enumerate()
        .map(|(level, nodes)| DependencyLevel {
            level: level as u32,
            nodes: nodes.clone(),
        })
        .collect();
    levels.push(DependencyLevel {
        level: 4,
        nodes: vec![output.clone()],
    });
    let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id: 713,
        spec: GraphSpec {
            nodes,
            ports: vec![],
            edges,
        },
        sequential_schedule: schedule,
        dependency_levels: levels,
        route_timings: vec![],
        inserted_delays: vec![],
        buffer_assignments: vec![],
        estimate: estimate(),
        envelope,
        required_bindings: required,
        routes: vec![],
        track_delays: vec![],
        effects: vec![],
        effect_controls: vec![],
        effect_observations: vec![],
        banks: vec![],
        builtin_banks: banks,
        observers: vec![],
    });
    let mut bindings: Vec<_> = groups[0]
        .iter()
        .cloned()
        .enumerate()
        .map(|(lane, node)| {
            GraphNodeBinding::new(
                node,
                Box::new(Source {
                    lane,
                    mono: mono != 0,
                }),
            )
        })
        .collect();
    bindings.push(GraphNodeBinding::identity(output));
    let observers = (0..2)
        .map(|ordinal| {
            GraphNodeObserverBinding::new(
                groups[if alias { 2 } else { 1 }][0].clone(),
                (2 - ordinal) as u64,
                Box::new(Observer {
                    ordinal,
                    fail: fail == 5,
                    probe: Arc::clone(&probe),
                }),
            )
        })
        .collect();
    let mut plan = graph
        .bind(GraphRuntimeBindings {
            envelope,
            nodes: bindings,
            observers,
        })
        .unwrap_or_else(|failure| panic!("bind {}", failure.code));
    plan.arm_mono_collapse(&|_| mono != 0);
    (plan, probe)
}

fn render(
    plan: &mut PreparedRenderPlan,
    pcm: &mut [f32],
    frames: usize,
    block: u64,
) -> Result<(), RenderError> {
    plan.render(
        realtime::RenderIo {
            input: None,
            output: PlanarBufferMut::try_new(pcm, 2, frames, frames).expect("output"),
        },
        realtime::RenderTime {
            absolute_sample: block * frames as u64,
        },
    )
}

#[test]
#[cfg(feature = "test-support")]
fn rt9_resident_observers_extra_reader_failures_and_modes_match_old_acquisition() {
    for width in [BankWidth::Four, BankWidth::Eight] {
        for population in [width.lanes() as usize - 1, width.lanes() as usize] {
            for frames in [1, width.lanes() - 1, width.lanes(), width.lanes() + 1, 17] {
                for alias in [false, true] {
                    for mono in 0..4 {
                        for fail in 0..6 {
                            let (mut old, old_probe) =
                                prepared(width, population, frames, alias, mono, fail);
                            let (mut new, new_probe) =
                                prepared(width, population, frames, alias, mono, fail);
                            assert_eq!(old.bank_chain_count(), new.bank_chain_count());
                            let mut a = vec![f32::from_bits(0x7fc0_aaaa); frames as usize * 2];
                            let mut b = a.clone();
                            for block in 0..3 {
                                old_probe.trace.store(0, Relaxed);
                                new_probe.trace.store(0, Relaxed);
                                // Exercise independent disengage and recovery at successive boundaries.
                                old.force_mono_collapse_off(block == 1);
                                new.force_mono_collapse_off(block == 1);
                                test_only_resident_input_reset(true);
                                let old_result = render(&mut old, &mut a, frames as usize, block);
                                let old_reads = test_only_resident_input_counts()[0];
                                test_only_resident_input_reset(false);
                                let new_result = render(&mut new, &mut b, frames as usize, block);
                                let [new_reads, residents] = test_only_resident_input_counts();
                                assert_eq!(new_result, old_result);
                                assert!(a.iter().zip(&b).all(|(a, b)| a.to_bits() == b.to_bits()));
                                assert_eq!(new_probe.snapshot(), old_probe.snapshot());
                                assert_eq!(
                                    new.qualification_counters(),
                                    old.qualification_counters()
                                );
                                assert_eq!(new.bank_transposes(), old.bank_transposes());
                                assert_eq!(
                                    new.bank_collapse_counters(),
                                    old.bank_collapse_counters()
                                );
                                assert_eq!(
                                    new.bank_collapse_transitions(),
                                    old.bank_collapse_transitions()
                                );
                                if old_result.is_ok() {
                                    assert_eq!(
                                        residents, 1,
                                        "positive adjacent observed/send boundary"
                                    );
                                    assert!(
                                        new_reads < old_reads,
                                        "physical resident acquisition must remove successor gather"
                                    );
                                } else if fail <= 2 || fail == 5 {
                                    assert_eq!(
                                        residents, 0,
                                        "producer/observer failure prevents successor drains"
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    test_only_resident_input_reset(false);
}

#[test]
fn rt9_resident_prepared_plan_render_allocates_and_frees_nothing() {
    struct Restore(Mode);
    impl Drop for Restore {
        fn drop(&mut self) {
            set_mode(self.0);
        }
    }
    assert_installed();
    let _restore = Restore(mode());
    set_mode(Mode::Count);
    realtime::audit::warm_up();
    realtime::audit::reset();
    realtime::audit::in_render_scope(|| {
        let live = Vec::<u8>::with_capacity(core::hint::black_box(64));
        core::hint::black_box(&live);
        drop(live);
    });
    let live = realtime::audit::snapshot();
    assert!(live.allocations > 0 && live.deallocations > 0);
    for width in [BankWidth::Four, BankWidth::Eight] {
        for population in [width.lanes() as usize - 1, width.lanes() as usize] {
            for mono in 0..4 {
                let (mut plan, probe) = prepared(width, population, 17, true, mono, 0);
                let mut pcm = [f32::from_bits(0x7fc0_bbbb); 34];
                #[cfg(feature = "test-support")]
                test_only_resident_input_reset(false);
                realtime::audit::reset();
                for block in 0..3 {
                    probe.trace.store(0, Relaxed);
                    render(&mut plan, &mut pcm, 17, block).expect("render");
                }
                let count = realtime::audit::snapshot();
                assert_eq!((count.allocations, count.deallocations), (0, 0));
                #[cfg(feature = "test-support")]
                assert_eq!(
                    test_only_resident_input_counts()[1],
                    3,
                    "allocator gate must execute resident acquisition"
                );
            }
        }
    }
}
