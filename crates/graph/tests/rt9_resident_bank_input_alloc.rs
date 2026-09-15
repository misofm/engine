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
    observed: [[AtomicU32; 17]; 16],
    visits: [AtomicU32; 16],
    activation_on: [AtomicU32; 16],
    activation_off: [AtomicU32; 16],
    invalidations: [AtomicU32; 16],
}
impl Probe {
    fn event(&self, event: u64) {
        self.trace.store(
            self.trace
                .load(Relaxed)
                .wrapping_mul(16)
                .wrapping_add(event),
            Relaxed,
        );
    }
    fn snapshot(&self) -> (u64, u32, [[u32; 17]; 4]) {
        (
            self.trace.load(Relaxed),
            self.command.load(Relaxed),
            std::array::from_fn(|row| {
                std::array::from_fn(|frame| self.observed[row][frame].load(Relaxed))
            }),
        )
    }
    fn observation_snapshot(&self) -> [[u32; 17]; 16] {
        std::array::from_fn(|row| {
            std::array::from_fn(|frame| self.observed[row][frame].load(Relaxed))
        })
    }
    fn visits(&self) -> [u32; 16] {
        self.visits.each_ref().map(|value| value.load(Relaxed))
    }
    fn reset_visits(&self) {
        for visit in &self.visits {
            visit.store(0, Relaxed);
        }
    }
    fn activation_changes(&self) -> ([u32; 16], [u32; 16]) {
        (
            self.activation_on
                .each_ref()
                .map(|value| value.load(Relaxed)),
            self.activation_off
                .each_ref()
                .map(|value| value.load(Relaxed)),
        )
    }
    fn invalidations(&self) -> [u32; 16] {
        self.invalidations
            .each_ref()
            .map(|value| value.load(Relaxed))
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
    scalar_lane: Option<usize>,
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
            let command = match self.scalar_lane {
                Some(lane) => {
                    self.probe.command.fetch_and(!(1 << lane), Relaxed) & (1 << lane) != 0
                }
                None => self.probe.command.swap(0, Relaxed) != 0,
            };
            self.state[0] += u32::from(command) as f32 * 0.125;
            self.state[1] += u32::from(command) as f32 * 0.125;
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
impl GraphRuntimeProcessor for Stage {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        GraphPreparedBuiltinBankProcessor::begin_block(self, block.first_sample)?;
        self.apply(block.left, Some(block.right), block.first_sample)
    }
}

struct Observer {
    command_mask: u32,
    ordinal: usize,
    fail: bool,
    resident: ResidentMode,
    probe: Arc<Probe>,
}
#[derive(Clone, Copy, PartialEq)]
enum ResidentMode {
    Decline,
    Accept,
    Error,
}
impl GraphRuntimeObserver for Observer {
    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
        self.probe.visits[self.ordinal].fetch_add(1, Relaxed);
        self.probe.event(3 + self.ordinal as u64);
        for (channel, plane) in [block.left, block.right].into_iter().enumerate() {
            for (destination, word) in self.probe.observed[self.ordinal * 2 + channel]
                .iter()
                .zip(plane)
            {
                destination.store(word.to_bits(), Relaxed);
            }
        }
        self.probe.command.store(self.command_mask, Relaxed);
        if self.fail && block.first_sample == 0 {
            Err(RenderError::InvalidEnvelope)
        } else {
            Ok(())
        }
    }
    fn activation_changed(&mut self, active: bool, _generation: u64, _first_sample: u64) {
        let counters = if active {
            &self.probe.activation_on
        } else {
            &self.probe.activation_off
        };
        counters[self.ordinal].fetch_add(1, Relaxed);
    }
    fn observe_resident(
        &mut self,
        block: GraphResidentObservationBlock<'_>,
    ) -> Option<Result<(), RenderError>> {
        match self.resident {
            ResidentMode::Decline => None,
            ResidentMode::Accept | ResidentMode::Error => {
                self.probe.visits[self.ordinal].fetch_add(1, Relaxed);
                self.probe.event(3 + self.ordinal as u64);
                let width = block.lane.width().lanes() as usize;
                let lane = block.lane.lane();
                for frame in 0..(block.lane.frames() as usize).min(17) {
                    let source = frame * width + lane;
                    self.probe.observed[self.ordinal * 2][frame]
                        .store(block.lane.left()[source].to_bits(), Relaxed);
                    self.probe.observed[self.ordinal * 2 + 1][frame]
                        .store(block.lane.right()[source].to_bits(), Relaxed);
                }
                if self.resident == ResidentMode::Accept {
                    Some(Ok(()))
                } else {
                    Some(Err(RenderError::InvalidEnvelope))
                }
            }
        }
    }
    fn invalidate_after_failure(&mut self, _failed_sample: u64) {
        self.probe.invalidations[self.ordinal].fetch_add(1, Relaxed);
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

#[derive(Clone, Copy, PartialEq)]
enum Control {
    Resident,
    Scalar,
    Intervening,
    Incompatible,
}

#[derive(Clone, Copy)]
struct FixtureOptions {
    observer_count: usize,
    controlled: bool,
    command_mask: u32,
    split_observers: bool,
    failing_observer: Option<usize>,
    resident: ResidentMode,
}

impl FixtureOptions {
    fn legacy(population: usize) -> Self {
        Self {
            observer_count: 2,
            controlled: false,
            command_mask: (1 << population) - 1,
            split_observers: false,
            failing_observer: None,
            resident: ResidentMode::Decline,
        }
    }
}

fn prepared(
    width: BankWidth,
    population: usize,
    frames: u32,
    alias: bool,
    mono: u8,
    fail: u32,
    control: Control,
) -> (PreparedRenderPlan, Arc<Probe>) {
    let (plan, probe, _) = prepared_with_options(
        width,
        population,
        frames,
        alias,
        mono,
        fail,
        control,
        FixtureOptions::legacy(population),
    );
    (plan, probe)
}

#[expect(
    clippy::too_many_arguments,
    reason = "extends the existing seven-parameter audio fixture with observer-only options"
)]
fn prepared_with_options(
    width: BankWidth,
    population: usize,
    frames: u32,
    alias: bool,
    mono: u8,
    fail: u32,
    control: Control,
    options: FixtureOptions,
) -> (
    PreparedRenderPlan,
    Arc<Probe>,
    Option<GraphObservationController>,
) {
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
    let send_id = StableGraphId::parse("send").expect("send");
    let send_node = GraphNodeId::Route {
        route_id: send_id.clone(),
    };
    let mut schedule: Vec<_> = groups.iter().flatten().cloned().collect();
    schedule.insert(
        if control == Control::Intervening {
            population * 3
        } else {
            schedule.len()
        },
        send_node.clone(),
    );
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
    for (lane, _) in groups[0].iter().enumerate() {
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
        send_node.clone(),
        GraphEdgeId::RouteSource {
            route_id: send_id.clone(),
        },
    );
    connect(
        send_node.clone(),
        output.clone(),
        GraphEdgeId::RouteDestination {
            route_id: send_id.clone(),
        },
    );
    let probe = Arc::new(Probe::default());
    let banks = [1, 3]
        .into_iter()
        .enumerate()
        .filter(|_| control != Control::Scalar)
        .map(|(id, group)| {
            let bank_width = if control == Control::Incompatible && id == 1 {
                BankWidth::Eight
            } else {
                width
            };
            GraphPreparedBuiltinBank {
                backend: if bank_width == BankWidth::Four {
                    Backend::Simd4
                } else {
                    Backend::Simd8
                },
                members: groups[group].clone().into_boxed_slice(),
                processor: Box::new(Stage {
                    id,
                    scalar_lane: None,
                    lanes: bank_width.lanes() as usize,
                    population,
                    state: [0.0; 2],
                    mono: mono & (1 << id) != 0,
                    fail,
                    probe: Arc::clone(&probe),
                }),
                scratch: rack::AoSoaScratch::new(bank_width, frames).expect("scratch"),
            }
        })
        .collect();
    let required: Vec<_> = groups[0]
        .iter()
        .chain(groups[1].iter())
        .chain(groups[3].iter())
        .chain(core::iter::once(&output))
        .cloned()
        .collect();
    let mut level_groups = groups.clone();
    level_groups.insert(
        if control == Control::Intervening {
            3
        } else {
            4
        },
        vec![send_node.clone()],
    );
    level_groups.push(vec![output.clone()]);
    let levels = level_groups
        .into_iter()
        .enumerate()
        .map(|(level, nodes)| DependencyLevel {
            level: level as u64,
            nodes,
        })
        .collect();
    let send_edge = GraphEdgeId::RouteSource {
        route_id: send_id.clone(),
    };
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
        inserted_delays: vec![InsertedDelay {
            node: GraphNodeId::CompensationDelay {
                edge_id: Box::new(send_edge.clone()),
            },
            edge_id: send_edge,
            samples: LatencySamples(2),
        }],
        buffer_assignments: vec![],
        estimate: estimate(),
        envelope,
        required_bindings: required,
        routes: vec![PreparedRoute {
            node: send_node,
            transform: RouteTransform {
                gain: 0.5,
                ll: 0.75,
                lr: -0.25,
                rl: 0.5,
                rr: 1.25,
            },
        }],
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
    if control == Control::Scalar {
        for (id, group) in [1, 3].into_iter().enumerate() {
            for (lane, node) in groups[group].iter().cloned().enumerate() {
                bindings.push(GraphNodeBinding::new(
                    node,
                    Box::new(Stage {
                        id,
                        scalar_lane: Some(lane),
                        lanes: 1,
                        population: 1,
                        state: [0.0; 2],
                        mono: false,
                        fail,
                        probe: Arc::clone(&probe),
                    }),
                ));
            }
        }
    }
    bindings.push(GraphNodeBinding::identity(output));
    let observers = (0..options.observer_count)
        .map(|ordinal| {
            let observed_group = if options.split_observers && ordinal >= options.observer_count / 2
            {
                3
            } else if alias {
                2
            } else {
                1
            };
            let observer = Box::new(Observer {
                command_mask: options.command_mask,
                ordinal,
                fail: fail == 5 || options.failing_observer == Some(ordinal),
                resident: options.resident,
                probe: Arc::clone(&probe),
            });
            if options.controlled {
                GraphNodeObserverBinding::controlled(
                    groups[observed_group][0].clone(),
                    (ordinal + 1) as u64,
                    observer,
                )
            } else {
                GraphNodeObserverBinding::new(
                    groups[observed_group][0].clone(),
                    (options.observer_count - ordinal) as u64,
                    observer,
                )
            }
        })
        .collect();
    let runtime_bindings = GraphRuntimeBindings {
        envelope,
        nodes: bindings,
        observers,
    };
    let (mut plan, controller) = if options.controlled {
        let (plan, controller) = graph
            .bind_with_observation_activation(
                runtime_bindings,
                GraphObservationActivationConfig {
                    maximum_active_observers: options.observer_count,
                    maximum_retained_bytes: u64::MAX,
                },
            )
            .unwrap_or_else(|failure| panic!("controlled bind {}", failure.code));
        (plan, Some(controller))
    } else {
        let plan = graph
            .bind(runtime_bindings)
            .unwrap_or_else(|failure| panic!("bind {}", failure.code));
        (plan, None)
    };
    plan.arm_mono_collapse(&|_| mono != 0);
    (plan, probe, controller)
}

fn render(
    plan: &mut PreparedRenderPlan,
    pcm: &mut [f32],
    frames: usize,
    block: u64,
) -> Result<realtime::RenderReport, RenderError> {
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
                            let (mut old, old_probe) = prepared(
                                width,
                                population,
                                frames,
                                alias,
                                mono,
                                fail,
                                Control::Resident,
                            );
                            let (mut new, new_probe) = prepared(
                                width,
                                population,
                                frames,
                                alias,
                                mono,
                                fail,
                                Control::Resident,
                            );
                            assert_eq!(old.bank_shape(), new.bank_shape());
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
#[cfg(feature = "test-support")]
fn rt9_crossfeed_delayed_send_matches_scalar_and_admission_controls() {
    for control in [
        Control::Resident,
        Control::Intervening,
        Control::Incompatible,
    ] {
        let (mut scalar, scalar_probe) =
            prepared(BankWidth::Four, 3, 17, true, 0, 0, Control::Scalar);
        let (mut candidate, probe) = prepared(BankWidth::Four, 3, 17, true, 0, 0, control);
        assert_eq!(scalar.bank_shape(), [0, 0]);
        let mut reference = [0.0; 34];
        let mut output = [0.0; 34];
        for block in 0..3 {
            test_only_resident_input_reset(false);
            let scalar_report =
                render(&mut scalar, &mut reference, 17, block).expect("scalar render");
            assert_eq!(test_only_resident_input_counts(), [0, 0], "scalar decline");
            test_only_resident_input_reset(false);
            let candidate_report =
                render(&mut candidate, &mut output, 17, block).expect("candidate render");
            assert_eq!(candidate_report, scalar_report);
            assert_eq!(output.map(f32::to_bits), reference.map(f32::to_bits));
            assert_eq!(
                probe.snapshot().2,
                scalar_probe.snapshot().2,
                "all observed words"
            );
            assert_eq!(
                test_only_resident_input_counts()[1],
                u64::from(control == Control::Resident),
                "adjacency/backend control"
            );
        }
    }
    test_only_resident_input_reset(false);
}

#[test]
#[cfg(feature = "test-support")]
fn rt9_controlled_observation_sets_match_fixed_bank_audio_and_visit_only_active_rows() {
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

    let options = FixtureOptions {
        observer_count: 6,
        controlled: false,
        command_mask: 0,
        split_observers: true,
        failing_observer: None,
        resident: ResidentMode::Decline,
    };
    let (mut baseline, baseline_probe, baseline_controller) = prepared_with_options(
        BankWidth::Four,
        3,
        17,
        false,
        0,
        0,
        Control::Resident,
        options,
    );
    assert!(baseline_controller.is_none());
    let (mut candidate, candidate_probe, mut controller) = prepared_with_options(
        BankWidth::Four,
        3,
        17,
        false,
        0,
        0,
        Control::Resident,
        FixtureOptions {
            controlled: true,
            ..options
        },
    );
    let mut controller = controller.take().expect("controlled fixture controller");
    assert_eq!(baseline.bank_shape(), candidate.bank_shape());
    assert_eq!(controller.resources().maximum_active_observers, 6);
    assert_eq!(
        controller
            .resources()
            .maximum_transition_entry_visits_per_block,
        24
    );

    let empty: &[u64] = &[];
    let one = [1];
    let all = [1, 2, 3, 4, 5, 6];
    let reactivate = [2];
    let churn_a = [1, 3, 5];
    let churn_b = [2, 4, 6];
    let selections: [&[u64]; 7] = [empty, &one, &all, empty, &reactivate, &churn_a, &churn_b];
    let expected_changes = [0, 1, 6, 12, 13, 17, 23];
    let mut baseline_pcm = [f32::from_bits(0x7fc0_aaaa); 34];
    let mut candidate_pcm = [f32::from_bits(0x7fc0_bbbb); 34];
    let mut saw_nonzero = false;

    for (block, selection) in selections.into_iter().enumerate() {
        baseline_probe.reset_visits();
        candidate_probe.reset_visits();
        test_only_meter_input_reset(false);
        let baseline_result = render(&mut baseline, &mut baseline_pcm, 17, block as u64);
        assert!(baseline_result.is_ok());
        test_only_meter_input_reset(false);
        realtime::audit::reset();
        let (accepted, candidate_result, applied) = realtime::audit::in_render_scope(|| {
            let accepted = controller
                .replace(selection)
                .expect("controlled selection admission");
            let result = render(&mut candidate, &mut candidate_pcm, 17, block as u64);
            let applied = controller.try_applied();
            (accepted, result, applied)
        });
        assert_eq!(candidate_result, baseline_result);
        assert_eq!(
            applied,
            Some(GraphObservationApplied {
                revision: accepted.revision,
                first_sample: block as u64 * 17,
            })
        );
        let audit = realtime::audit::snapshot();
        assert_eq!((audit.allocations, audit.deallocations), (0, 0));
        assert_eq!(
            candidate_pcm.map(f32::to_bits),
            baseline_pcm.map(f32::to_bits)
        );
        saw_nonzero |= candidate_pcm.iter().any(|sample| *sample != 0.0);
        assert_eq!(
            candidate.qualification_counters(),
            baseline.qualification_counters(),
            "activation must not reset bank DSP state"
        );
        assert_eq!(
            candidate.bank_collapse_counters(),
            baseline.bank_collapse_counters()
        );
        assert_eq!(
            candidate.bank_collapse_transitions(),
            baseline.bank_collapse_transitions()
        );

        let visits = candidate_probe.visits();
        assert_eq!(
            visits.iter().map(|count| *count as usize).sum::<usize>(),
            selection.len()
        );
        for handle in selection {
            let ordinal = (*handle - 1) as usize;
            assert_eq!(visits[ordinal], 1, "selected observer must be visited once");
            let baseline_observed = baseline_probe.observation_snapshot();
            let candidate_observed = candidate_probe.observation_snapshot();
            assert_eq!(
                candidate_observed[ordinal * 2],
                baseline_observed[ordinal * 2],
                "selected left observer output"
            );
            assert_eq!(
                candidate_observed[ordinal * 2 + 1],
                baseline_observed[ordinal * 2 + 1],
                "selected right observer output"
            );
        }
        let counts = test_only_meter_input_counts();
        if selection.is_empty() {
            assert_eq!(
                counts,
                [0, 0, 0],
                "dormant catalog rows must not acquire audio"
            );
        } else {
            assert!(counts[0] <= selection.len() as u64);
            assert!(counts[1] <= selection.len() as u64);
            assert_eq!(
                counts[2], 0,
                "declining resident observers use planar fallback"
            );
        }
        let (on, off) = candidate_probe.activation_changes();
        let changed = on
            .iter()
            .zip(off.iter())
            .map(|(on, off)| (*on + *off) as usize)
            .sum::<usize>();
        assert_eq!(
            changed, expected_changes[block],
            "activation transitions at block {block}"
        );
    }
    assert!(saw_nonzero, "fixture must render nonzero PCM");
    test_only_meter_input_reset(false);
}

#[test]
#[cfg(feature = "test-support")]
fn rt9_controlled_observation_preserves_resident_acceptance_fallback_and_error_once() {
    for resident in [
        ResidentMode::Accept,
        ResidentMode::Decline,
        ResidentMode::Error,
    ] {
        let (mut plan, probe, mut controller) = prepared_with_options(
            BankWidth::Four,
            3,
            17,
            false,
            0,
            0,
            Control::Resident,
            FixtureOptions {
                observer_count: 1,
                controlled: true,
                command_mask: 0,
                split_observers: false,
                failing_observer: None,
                resident,
            },
        );
        let mut controller = controller.take().expect("controlled fixture controller");
        let accepted = controller.replace(&[1]).expect("one observer admission");
        let mut pcm = [f32::from_bits(0x7fc0_cccc); 34];
        test_only_meter_input_reset(false);
        let result = render(&mut plan, &mut pcm, 17, 0);
        let counts = test_only_meter_input_counts();
        assert_eq!(
            controller.try_applied(),
            Some(GraphObservationApplied {
                revision: accepted.revision,
                first_sample: 0,
            })
        );
        assert_eq!(
            probe.visits()[0],
            1,
            "resident error must not retry planar input"
        );
        match resident {
            ResidentMode::Accept => {
                assert!(result.is_ok());
                assert_eq!(counts, [0, 1, 1]);
            }
            ResidentMode::Decline => {
                assert!(result.is_ok());
                assert_eq!(counts, [1, 1, 0]);
            }
            ResidentMode::Error => {
                assert_eq!(result, Err(RenderError::InvalidEnvelope));
                assert_eq!(counts, [0, 1, 1]);
                assert_eq!(probe.invalidations()[0], 1);
            }
        }
        test_only_meter_input_reset(false);
    }
}

#[test]
#[cfg(feature = "test-support")]
fn rt9_controlled_observation_failure_invalidates_active_later_rows_only() {
    let (mut plan, probe, mut controller) = prepared_with_options(
        BankWidth::Four,
        3,
        17,
        false,
        0,
        0,
        Control::Resident,
        FixtureOptions {
            observer_count: 4,
            controlled: true,
            command_mask: 0,
            split_observers: true,
            failing_observer: Some(0),
            resident: ResidentMode::Decline,
        },
    );
    let mut controller = controller.take().expect("controlled fixture controller");
    let accepted = controller.replace(&[1, 3]).expect("active failure rows");
    let before = plan.qualification_counters();
    let mut pcm = [f32::from_bits(0x7fc0_dddd); 34];
    test_only_meter_input_reset(false);
    let result = render(&mut plan, &mut pcm, 17, 0);
    assert_eq!(result, Err(RenderError::InvalidEnvelope));
    assert_eq!(
        controller.try_applied(),
        Some(GraphObservationApplied {
            revision: accepted.revision,
            first_sample: 0,
        })
    );
    assert_ne!(
        plan.qualification_counters(),
        before,
        "observer failure must not reset executed DSP state"
    );
    let visits = probe.visits();
    assert_eq!(visits[0], 1, "the failing observer is reached");
    assert_eq!(visits[2], 0, "a later active observer is after the failure");
    let invalidations = probe.invalidations();
    assert_eq!(invalidations[0], 1, "failed observer is invalidated once");
    assert_eq!(
        invalidations[2], 1,
        "later active observer is invalidated once"
    );
    assert_eq!(
        invalidations[1], 0,
        "inactive observer is never invalidated"
    );
    assert_eq!(
        invalidations[3], 0,
        "inactive observer is never invalidated"
    );
    assert_eq!(test_only_meter_input_counts(), [1, 1, 0]);
    test_only_meter_input_reset(false);
}

#[test]
#[cfg(feature = "test-support")]
fn rt9_controlled_observation_queued_revisions_preserve_order_while_reader_stalls() {
    let (mut plan, probe, mut controller) = prepared_with_options(
        BankWidth::Four,
        3,
        17,
        false,
        0,
        0,
        Control::Resident,
        FixtureOptions {
            observer_count: 2,
            controlled: true,
            command_mask: 0,
            split_observers: false,
            failing_observer: None,
            resident: ResidentMode::Decline,
        },
    );
    let mut controller = controller.take().expect("controlled fixture controller");
    let ordinary = controller.replace(&[1]).expect("ordinary admission");
    let removal = controller.remove_to(&[]).expect("removal admission");
    assert_eq!(ordinary.revision + 1, removal.revision);
    assert_eq!(
        controller.replace(&[2]),
        Err(GraphObservationAdmissionError::Backpressure)
    );

    let mut pcm = [f32::from_bits(0x7fc0_eeee); 34];
    for block in 0..3 {
        test_only_meter_input_reset(false);
        render(&mut plan, &mut pcm, 17, block).expect("queued boundary render");
        assert_eq!(test_only_meter_input_counts(), [0, 0, 0]);
        assert_eq!(probe.visits().iter().sum::<u32>(), 0);
        if block == 0 {
            assert_eq!(
                controller.remove_to(&[]),
                Err(GraphObservationAdmissionError::Backpressure),
                "retirement reader is deliberately stalled"
            );
        }
    }
    assert_eq!(
        controller.try_applied(),
        Some(GraphObservationApplied {
            revision: ordinary.revision,
            first_sample: 0,
        })
    );
    assert_eq!(
        controller.try_applied(),
        Some(GraphObservationApplied {
            revision: removal.revision,
            first_sample: 0,
        })
    );
    assert_eq!(controller.try_applied(), None);
    let next = controller.replace(&[2]).expect("credits recycle in order");
    test_only_meter_input_reset(false);
    render(&mut plan, &mut pcm, 17, 3).expect("reactivation render");
    assert_eq!(test_only_meter_input_counts()[2], 0);
    assert_eq!(probe.visits()[1], 1);
    assert_eq!(
        controller.try_applied(),
        Some(GraphObservationApplied {
            revision: next.revision,
            first_sample: 51,
        })
    );
    test_only_meter_input_reset(false);
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
                let (mut plan, probe) =
                    prepared(width, population, 17, true, mono, 0, Control::Resident);
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
