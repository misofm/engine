//! Issue #1504: every live drain of a graph plan receives the block's live snapshot `S`, and runs
//! once in every rendered block.
//!
//! The plan is rendered through the exchange (`RealtimePlanOwner`) so that `S` is what render
//! read from the plan's own revision gate. Revisions are published on that gate between blocks.
//! Three drain kinds are probed: the builtin bank (the bank stage forwards to it), the per-node
//! processors, and the live route (through the graph's route-drain test hooks). The rack crate's
//! own tests probe the bank chain's hand-over to a bare `BankStage`, the fourth.
#![cfg(feature = "test-support")]

use core::num::NonZeroUsize;
use effect_contract::{BankWidth, LatencySamples, TailSamples};
use engine::{
    QuantumFrames, SampleRateHz,
    realtime::{
        self, LiveSnapshot, PlanExchangeConfig, PlanarBufferMut, PreparedRenderPlan,
        RealtimePlanOwner, RenderEnvelope, RenderError, plan_exchange_at_revision,
    },
};
use graph::*;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering::Relaxed},
};

const FRAMES: usize = 4;
const BLOCKS: usize = 8;
/// Populated lanes: fewer than either bank width, so the probe bank is a partial bank and its
/// remaining lanes are inactive.
const LANES: usize = 3;

/// What each drain kind saw, per block. Atomics only: render runs on the test thread, but the
/// probes are shared with the plan they were built into.
#[derive(Default)]
struct Recorder {
    /// Builtin bank `begin_block` calls and the snapshot revision of the last one, per block.
    bank_calls: [AtomicU64; BLOCKS],
    bank_snapshot: [AtomicU64; BLOCKS],
    /// Per-node processor `process` calls and snapshot revision, per lane and block.
    node_calls: [[AtomicU64; BLOCKS]; LANES],
    node_snapshot: [[AtomicU64; BLOCKS]; LANES],
}

fn block_of(first_sample: u64) -> usize {
    usize::try_from(first_sample / FRAMES as u64).expect("block index")
}

/// A source whose blocks listed in `silent` are all `+0.0`.
struct Source {
    lane: usize,
    silent: u32,
}
impl GraphRuntimeProcessor for Source {
    fn retained_bytes(&self) -> u64 {
        0
    }
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let silent = self.silent & (1 << block_of(block.first_sample)) != 0;
        for (frame, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            *left = if silent {
                0.0
            } else {
                (self.lane + frame + 1) as f32 * 0.125
            };
            *right = if silent { 0.0 } else { -*left };
        }
        Ok(())
    }
}

/// The per-node live processor probe: it leaves the audio alone and records the snapshot its
/// block carried.
struct NodeProbe {
    lane: usize,
    recorder: Arc<Recorder>,
}
impl GraphRuntimeProcessor for NodeProbe {
    fn retained_bytes(&self) -> u64 {
        engine::realtime::retained::arc(&self.recorder)
    }
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let at = block_of(block.first_sample);
        self.recorder.node_calls[self.lane][at].fetch_add(1, Relaxed);
        self.recorder.node_snapshot[self.lane][at].store(block.live_snapshot.revision(), Relaxed);
        Ok(())
    }
}

/// The builtin bank probe; its audio pass is the identity.
struct BankProbe(Arc<Recorder>);
impl GraphPreparedBuiltinBankProcessor for BankProbe {
    fn retained_bytes(&self) -> u64 {
        engine::realtime::retained::arc(&self.0)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn begin_block(
        &mut self,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<(), RenderError> {
        let at = block_of(first_sample);
        self.0.bank_calls[at].fetch_add(1, Relaxed);
        self.0.bank_snapshot[at].store(snapshot.revision(), Relaxed);
        Ok(())
    }
    fn process(&mut self, _: &mut [f32], _: &mut [f32], _: u32, _: u64) -> Result<(), RenderError> {
        Ok(())
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
        effect_bank_processor_bytes: 0,
        builtin_bank_bytes: 0,
        builtin_bank_scratch_bytes: 0,
        builtin_bank_count: 0,
        largest_allocation_bytes: 0,
        incremental_plan_bytes: 0,
        session_plus_plan_bytes: 0,
    }
}

/// The plan: `LANES` tracks of source -> probe bank -> (alias) -> per-node probe, each feeding the
/// output, and lane 0 also sending through a live route into a submix whose input feeds the
/// output.
fn prepared(
    width: BankWidth,
    silent: u32,
) -> (PreparedRenderPlan, Arc<Recorder>, GraphRouteControlProducer) {
    let envelope = RenderEnvelope {
        sample_rate: SampleRateHz(48_000),
        quantum: QuantumFrames(FRAMES as u32),
        output_channels: NonZeroUsize::new(2).expect("stereo"),
    };
    let track = |name: &str, stage| GraphNodeId::TrackStage {
        track_id: StableGraphId::parse(name).expect("track"),
        stage,
    };
    let stages = [
        TrackStage::Input,
        TrackStage::PostInputBuiltins,
        TrackStage::PostSimd1,
        TrackStage::PostFader,
    ];
    let groups: Vec<Vec<GraphNodeId>> = stages
        .into_iter()
        .map(|stage| {
            (0..LANES)
                .map(|lane| track(&format!("track{lane}"), stage))
                .collect()
        })
        .collect();
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("main").expect("output"),
    };
    let bus = track("bus", TrackStage::Input);
    let send_id = StableGraphId::parse("send").expect("send");
    let send = GraphNodeId::Route {
        route_id: send_id.clone(),
    };

    let mut schedule: Vec<_> = groups.iter().flatten().cloned().collect();
    schedule.extend([send.clone(), bus.clone(), output.clone()]);
    let mut nodes: Vec<_> = schedule
        .iter()
        .cloned()
        .map(|id| GraphNode {
            id,
            latency: LatencySamples(0),
            bound: effect_contract::NodeTailBound::ZERO,
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
            path: "$.live_snapshot".to_owned(),
        })
    };
    #[allow(clippy::needless_range_loop)] // `lane` indexes every stage group and names the route
    for lane in 0..LANES {
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
    connect(
        groups[3][0].clone(),
        send.clone(),
        GraphEdgeId::RouteSource {
            route_id: send_id.clone(),
        },
    );
    connect(
        send.clone(),
        bus.clone(),
        GraphEdgeId::RouteDestination {
            route_id: send_id.clone(),
        },
    );
    connect(
        bus.clone(),
        output.clone(),
        GraphEdgeId::RouteSource {
            route_id: StableGraphId::parse("busout").expect("route"),
        },
    );

    let recorder = Arc::new(Recorder::default());
    let bank = GraphPreparedBuiltinBank {
        backend: width.backend(),
        members: groups[1].clone().into_boxed_slice(),
        processor: Box::new(BankProbe(Arc::clone(&recorder))),
        scratch: rack::AoSoaScratch::new(width, FRAMES as u32).expect("scratch"),
    };
    let required: Vec<_> = groups[0]
        .iter()
        .chain(groups[1].iter())
        .chain(groups[3].iter())
        .chain([&bus, &output])
        .cloned()
        .collect();
    let mut level_groups = groups.clone();
    level_groups.extend([vec![send.clone()], vec![bus.clone()], vec![output.clone()]]);
    let levels = level_groups
        .into_iter()
        .enumerate()
        .map(|(level, nodes)| DependencyLevel {
            level: level as u64,
            nodes,
        })
        .collect();
    let mut graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id: 1504,
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
        routes: vec![PreparedRoute {
            node: send,
            transform: RouteTransform {
                gain: 1.0,
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
            },
            gate: RouteGate::OPEN,
        }],
        track_delays: vec![],
        effects: vec![],
        effect_controls: vec![],
        effect_observations: vec![],
        banks: vec![],
        builtin_banks: vec![bank],
        observers: vec![],
    });
    let mut producers = graph
        .attach_route_controls(&realtime::LiveValuesSuperseded::new())
        .expect("the send route into the submix is live");
    assert_eq!(producers.len(), 1);
    let mut bindings: Vec<_> = groups[0]
        .iter()
        .cloned()
        .enumerate()
        .map(|(lane, node)| GraphNodeBinding::new(node, Box::new(Source { lane, silent })))
        .collect();
    bindings.extend(groups[3].iter().cloned().enumerate().map(|(lane, node)| {
        GraphNodeBinding::new(
            node,
            Box::new(NodeProbe {
                lane,
                recorder: Arc::clone(&recorder),
            }),
        )
    }));
    bindings.push(GraphNodeBinding::identity(bus));
    bindings.push(GraphNodeBinding::identity(output));
    let plan = graph
        .bind(GraphRuntimeBindings {
            envelope,
            nodes: bindings,
            observers: vec![],
        })
        .unwrap_or_else(|failure| panic!("bind {}", failure.code));
    (plan, recorder, producers.remove(0))
}

fn render(owner: &mut RealtimePlanOwner, block: usize) -> (realtime::RenderReport, Vec<f32>) {
    let mut pcm = vec![1.0_f32; 2 * FRAMES];
    let report = owner
        .render(
            realtime::RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, FRAMES, FRAMES).expect("output"),
            },
            realtime::RenderTime {
                absolute_sample: (block * FRAMES) as u64,
            },
        )
        .expect("render");
    (report.render, pcm)
}

/// The exchange around `plan`, whose gate handle the caller keeps. The publisher and the retirer
/// stay alive with the owner: no plan is replaced in this file.
type Exchange = (
    realtime::PlanRevisionGate,
    RealtimePlanOwner,
    (realtime::PlanPublisher, realtime::PlanRetirer),
);

fn exchange(mut plan: PreparedRenderPlan) -> Exchange {
    let mut gate = plan.take_revision_gate().expect("a fresh plan's gate");
    let (publisher, owner, retirer) = plan_exchange_at_revision(
        plan,
        &mut gate,
        1,
        PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("exchange");
    (gate, owner, (publisher, retirer))
}

/// One transaction: stamp the next revision on the plan's gate, write `record` to the route's
/// cell under it, and publish the revision.
fn write_published(
    gate: &mut realtime::PlanRevisionGate,
    producer: &mut GraphRouteControlProducer,
    record: RouteControlRecord,
) {
    let revision = gate.last_published() + 1;
    let stamp = gate.stamp(revision).expect("a legal revision");
    producer.write(stamp, record);
    gate.publish(revision);
}

/// Gate 1: every drain kind of a block receives the `S` the executor received that block, and `S`
/// is what the plan's own gate held when the block began.
///
/// Test value: a drain kind that is handed a fixed or another block's snapshot (the bank chain's
/// hand-over, the builtin stage's forward, `GraphBindingBlock`'s field, the route op's drain).
#[test]
fn every_drain_kind_receives_the_blocks_snapshot() {
    // The revision published on the gate before each block, if any.
    const PUBLISH: [Option<u64>; BLOCKS] =
        [None, Some(2), None, Some(5), Some(6), None, Some(9), None];
    for &width in BankWidth::ALL {
        let (plan, recorder, _producer) = prepared(width, 0);
        let (mut gate, mut owner, _exchange) = exchange(plan);
        let mut current = 1;
        let mut seen = Vec::new();
        for (block, publish) in PUBLISH.iter().enumerate() {
            if let Some(revision) = publish {
                gate.publish(*revision);
                current = *revision;
            }
            let (report, _) = render(&mut owner, block);
            let executor = report.live_snapshot.revision();
            assert_eq!(
                executor, current,
                "{width:?} block {block}: S is the gate's"
            );
            assert_eq!(
                recorder.bank_snapshot[block].load(Relaxed),
                executor,
                "{width:?} block {block}: the builtin bank's S"
            );
            for lane in 0..LANES {
                assert_eq!(
                    recorder.node_snapshot[lane][block].load(Relaxed),
                    executor,
                    "{width:?} block {block} lane {lane}: the per-node processor's S"
                );
            }
            assert_eq!(
                test_only_route_drain_snapshots(),
                [executor],
                "{width:?} block {block}: the live route's S"
            );
            seen.push(executor);
        }
        assert_eq!(
            seen,
            [1, 2, 2, 5, 6, 6, 9, 9],
            "the schedule really varied S"
        );
    }
}

/// Gate 2: every live drain runs exactly once in every rendered block, including blocks whose
/// source is silent and blocks of a route that is inactive.
///
/// Silent blocks are 1, 2 and 5. A mute record queued before block 3 makes the route inactive
/// from that block on, so the route op returns before it mixes. The bank is partial: `LANES` of
/// its lanes are populated and the rest are inactive.
///
/// Test value: a silence or activity skip that also skips a drain. Under `S` that makes the
/// watermark early, and no other test counts drains per block.
#[test]
fn every_live_drain_runs_once_in_every_block_silent_or_not() {
    const SILENT: u32 = (1 << 1) | (1 << 2) | (1 << 5);
    for &width in BankWidth::ALL {
        let (plan, recorder, mut producer) = prepared(width, SILENT);
        let (mut gate, mut owner, _exchange) = exchange(plan);
        let mute = RouteControlRecord::new([0.0; 4], true, 0).expect("a legal mute");
        let (mut drains, mut mixes) = (0, 0);
        for block in 0..BLOCKS {
            if block == 3 {
                write_published(&mut gate, &mut producer, mute);
            }
            let (_, pcm) = render(&mut owner, block);
            let silent = SILENT & (1 << block) != 0;
            assert_eq!(
                pcm.iter().all(|word| word.to_bits() == 0),
                silent,
                "{width:?} block {block}: the block is silent exactly when its source is"
            );
            assert_eq!(
                recorder.bank_calls[block].load(Relaxed),
                1,
                "{width:?} block {block}: the builtin bank drained once"
            );
            for lane in 0..LANES {
                assert_eq!(
                    recorder.node_calls[lane][block].load(Relaxed),
                    1,
                    "{width:?} block {block} lane {lane}: the per-node processor drained once"
                );
            }
            let (now_drains, now_mixes) = (
                test_only_route_drain_calls()[0],
                test_only_route_mix_counts()[0],
            );
            assert_eq!(
                now_drains - drains,
                1,
                "{width:?} block {block}: the live route drained once"
            );
            // The route mixes while it is active: blocks 0-2, not after the mute.
            assert_eq!(
                now_mixes - mixes,
                u64::from(block < 3),
                "{width:?} block {block}: the route's activity is what the test drove"
            );
            (drains, mixes) = (now_drains, now_mixes);
        }
        assert_eq!(drains, BLOCKS as u64);
    }
}

// ---------------------------------------------------------------------------------------------
// Decision 15, #1345: an effect bank with one live-bypassed lane (deferred from #1504, m1).
// ---------------------------------------------------------------------------------------------

mod effect_bank {
    use super::*;
    use effect_contract::{
        BankProcessReport, EffectBankProcessBlock, EffectCellWriter, EffectControlRecord, EffectId,
        EffectLaneShape, EffectLaneSpec, EffectProgramKey, EffectQuality, LinkMode,
        PreparedBankMetadata, PreparedNativeEffectBank, PreparedPorts, PreparedSidechainPort,
        ResetKind, RestBound, StatePayloadError, StatePayloadInput, StatePayloadOutput,
        StatePayloadSizes, effect_control_lane,
    };

    /// Frames of latency the mock bank declares: the bypass shunt delays the dry signal by it, so
    /// a bypassed lane and a wet lane differ in the first `LATENCY` samples of a *silent* block.
    const LATENCY: usize = 2;
    /// The mock bank's gain, so wet and dry differ.
    const GAIN: f32 = 0.5;
    /// The lane that carries the live bypass.
    const LIVE_LANE: usize = 1;

    /// A bank that gains every lane by `GAIN` and delays it by `LATENCY` frames, per lane.
    struct GainBank {
        width: usize,
        line: Vec<[f32; 2]>,
    }

    impl PreparedNativeEffectBank for GainBank {
        fn retained_bytes(&self) -> u64 {
            engine::realtime::retained::vec(&self.line)
        }
        fn reset(&mut self, _: ResetKind) {}
        fn reset_lane(&mut self, _: usize, _: ResetKind) {}
        fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
            for frame in 0..block.frames as usize {
                for lane in 0..self.width {
                    let index = frame * self.width + lane;
                    let wet = [block.left[index] * GAIN, block.right[index] * GAIN];
                    let slot = &mut self.line[(frame % LATENCY) * self.width + lane];
                    let held = *slot;
                    *slot = wet;
                    block.left[index] = held[0];
                    block.right[index] = held[1];
                }
            }
            BankProcessReport::empty(block.width)
        }
        fn snapshot_track_state_payload(
            &self,
            _: u32,
            _: StatePayloadOutput<'_>,
        ) -> Result<(), StatePayloadError> {
            Ok(())
        }
        fn restore_track_state_payload(
            &mut self,
            _: u32,
            _: u32,
            _: StatePayloadInput<'_>,
        ) -> Result<(), StatePayloadError> {
            Ok(())
        }
    }

    fn program_key() -> EffectProgramKey {
        EffectProgramKey {
            effect_id: EffectId::parse("fixture.gain").expect("id"),
            contract_major: 1,
            state_layout_version: 1,
            sample_rate: 48_000,
            quantum: FRAMES as u32,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            latency: LatencySamples(LATENCY as u64),
            tail: TailSamples::Finite(0),
            tail_every_peak: TailSamples::Infinite,
            rest: RestBound::Unstated,
            composition: effect_contract::CompositionBound::Unstated,
            state_sizes: StatePayloadSizes {
                common_bytes: 0,
                left_bytes: 0,
                right_bytes: 0,
            },
            scratch_bytes: 0,
            automation_capacity: 1,
        }
    }

    /// `LANES` tracks of source -> gain bank -> output, the bank one slot of width `width`, and a
    /// live control lane (a bypass cell) on `LIVE_LANE` only.
    fn plan(width: BankWidth, silent: u32) -> (PreparedRenderPlan, EffectCellWriter) {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(FRAMES as u32),
            output_channels: NonZeroUsize::new(2).expect("stereo"),
        };
        let track = |name: &str| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(name).expect("track"),
            stage: TrackStage::Input,
        };
        let member = |name: &str| EffectNodeId {
            track_id: StableGraphId::parse(name).expect("track"),
            rack: RackId::Dynamic,
            effect_id: StableGraphId::parse("gain").expect("effect"),
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output"),
        };
        let names: Vec<String> = (0..LANES).map(|lane| format!("track{lane}")).collect();
        let inputs: Vec<GraphNodeId> = names.iter().map(|name| track(name)).collect();
        let members: Vec<EffectNodeId> = names.iter().map(|name| member(name)).collect();
        let effect_nodes: Vec<GraphNodeId> =
            members.iter().cloned().map(GraphNodeId::Effect).collect();
        let mut schedule: Vec<GraphNodeId> = inputs.clone();
        schedule.extend(effect_nodes.iter().cloned());
        schedule.push(output.clone());
        let mut nodes: Vec<GraphNode> = schedule
            .iter()
            .cloned()
            .map(|id| GraphNode {
                id,
                latency: LatencySamples(0),
                bound: effect_contract::NodeTailBound::ZERO,
            })
            .collect();
        nodes.sort_by(|a, b| a.id.cmp(&b.id));
        let edge = |source: &GraphNodeId, destination: &GraphNodeId, id: GraphEdgeId| GraphEdge {
            id,
            source: GraphPortId {
                node: source.clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: destination.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.effect_bank".to_owned(),
        };
        let mut edges = Vec::new();
        for lane in 0..LANES {
            edges.push(edge(
                &inputs[lane],
                &effect_nodes[lane],
                GraphEdgeId::TrackMain {
                    target: effect_nodes[lane].clone(),
                },
            ));
            edges.push(edge(
                &effect_nodes[lane],
                &output,
                GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse(&format!("main{lane}")).expect("route"),
                },
            ));
        }
        let width_lanes = width.lanes() as usize;
        let mut active_mask = vec![false; width_lanes];
        active_mask[..LANES].fill(true);
        let superseded = realtime::LiveValuesSuperseded::new();
        let (writer, control) = effect_control_lane(
            EffectLaneSpec {
                shape: EffectLaneShape::from_live_parameters([], 0),
                bypass: false,
                observation_depth: 0,
                link: None,
            },
            &superseded,
        )
        .expect("a bypass-only lane");
        let bank = GraphPreparedEffectBank {
            members: members.clone().into_boxed_slice(),
            active_mask: active_mask.into_boxed_slice(),
            processor: Box::new(GainBank {
                width: width_lanes,
                line: vec![[0.0; 2]; LATENCY * width_lanes],
            }),
            metadata: PreparedBankMetadata {
                width,
                program_key: program_key(),
            },
            response_snapshot_declared: false,
            native_id: "fixture.gain",
            scratch: rack::AoSoaScratch::new(width, FRAMES as u32).expect("scratch"),
            cohort: GraphBankCohort { group: 0, slot: 0 },
        };
        let mut required = inputs.clone();
        required.push(output.clone());
        let mut levels = vec![DependencyLevel {
            level: 0,
            nodes: inputs.clone(),
        }];
        levels.push(DependencyLevel {
            level: 1,
            nodes: effect_nodes.clone(),
        });
        levels.push(DependencyLevel {
            level: 2,
            nodes: vec![output.clone()],
        });
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 1345,
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
            effect_controls: vec![GraphEffectControlBinding {
                node: members[LIVE_LANE].clone(),
                control: Box::new(control),
            }],
            effect_observations: vec![],
            banks: vec![bank],
            builtin_banks: vec![],
            observers: vec![],
        });
        let mut bindings: Vec<_> = inputs
            .iter()
            .cloned()
            .enumerate()
            .map(|(lane, node)| GraphNodeBinding::new(node, Box::new(Source { lane, silent })))
            .collect();
        bindings.push(GraphNodeBinding::identity(output));
        let plan = graph
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bindings,
                observers: vec![],
            })
            .unwrap_or_else(|failure| panic!("bind {}", failure.code));
        (plan, writer)
    }

    /// What `Source` writes into sample `n` of `lane` (the left plane), `0.0` in a silent block.
    fn input(lane: usize, silent: u32, n: usize) -> f32 {
        if silent & (1 << (n / FRAMES)) != 0 {
            0.0
        } else {
            (lane + n % FRAMES + 1) as f32 * 0.125
        }
    }

    /// Gate 2 of #1504, with the live bypassed lane the batch note deferred to #1345: a lane whose
    /// bypass is a cell is drained in every block, silent or not, under the block's snapshot, so a
    /// bypass lands in exactly the block whose snapshot covers its revision.
    ///
    /// The plan is a partial bank (three of its lanes populated) rendered through the exchange.
    /// Lane 1 is the live lane. A bypassed and an un-bypassed lane differ in the first `LATENCY`
    /// samples of a *silent* block (the dry line holds the previous block's input at unity, the wet
    /// line the same input at `GAIN`), so a drain that runs only on audible blocks, or reads
    /// without the snapshot, changes the rendered bits. The timeline: bypass lands before block 1;
    /// the un-bypass is written during block 3 but published only before block 5 (blocks 3 and 4
    /// snapshot below it); and block 6 is silent with the un-bypass long in effect.
    ///
    /// Test value: a bank stage that reads the cells without the block's snapshot, which applies a
    /// revision before the watermark reports it; this is the test's unique catch (it alone goes
    /// red). A stage that skips a bypassed lane's drain also turns it red, with rack's tests. A
    /// stand-in for skipping the drains of a silent block was run against it and left it green, so
    /// the silent blocks here are not claimed as a catch.
    #[test]
    fn a_live_bypassed_lane_is_drained_every_block_under_the_blocks_snapshot() {
        const SILENT: u32 = (1 << 2) | (1 << 4) | (1 << 6);
        const TOTAL: usize = 8;
        for &width in BankWidth::ALL {
            let (plan, mut writer) = plan(width, SILENT);
            let (mut gate, mut owner, _exchange) = exchange(plan);
            // The bypass is in effect from the first block whose snapshot covers its revision, and
            // the un-bypass from the first block after its publication.
            let bypassed_blocks: [bool; TOTAL] =
                core::array::from_fn(|block| (1..5).contains(&block));
            let mut observed: Vec<Vec<f32>> = Vec::new();
            // The revision of the un-bypass, written during block 3 and published before block 5.
            let mut unpublished = None;
            for block in 0..TOTAL {
                let mut write = |gate: &mut realtime::PlanRevisionGate, bypass: bool| {
                    let revision = gate.last_published() + 1;
                    let stamp = gate.stamp(revision).expect("a legal revision");
                    writer
                        .write(
                            stamp,
                            EffectControlRecord::Bypass(effect_contract::BypassSwitch::step(
                                bypass,
                            )),
                        )
                        .expect("a bypass cell");
                    revision
                };
                if block == 1 {
                    let revision = write(&mut gate, true);
                    gate.publish(revision);
                }
                // Written during block 3, published before block 5: blocks 3 and 4 snapshot
                // below its revision.
                if block == 3 {
                    unpublished = Some(write(&mut gate, false));
                }
                if block == 5 {
                    gate.publish(unpublished.take().expect("written during block 3"));
                }
                observed.push(render(&mut owner, block).1);
            }
            for (block, pcm) in observed.iter().enumerate() {
                for (frame, sample) in pcm.iter().enumerate().take(FRAMES) {
                    let n = block * FRAMES + frame;
                    let mut expected = 0.0_f32;
                    for lane in 0..LANES {
                        let delayed = if n >= LATENCY {
                            input(lane, SILENT, n - LATENCY)
                        } else {
                            0.0
                        };
                        let bypassed = lane == LIVE_LANE && bypassed_blocks[block];
                        expected += if bypassed { delayed } else { delayed * GAIN };
                    }
                    assert_eq!(
                        sample.to_bits(),
                        expected.to_bits(),
                        "{width:?} block {block} frame {frame}: lane {LIVE_LANE} is {}",
                        if bypassed_blocks[block] { "dry" } else { "wet" }
                    );
                }
            }
        }
    }
}
