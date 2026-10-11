//! Issue #1504: every live drain of a graph plan receives the block's live snapshot `S`, and runs
//! once in every rendered block.
//!
//! The plan is rendered through the exchange (`RealtimePlanOwner`) so that `S` is what render
//! read from the plan's own revision gate. Revisions are published on that gate between blocks.
//! Four drain kinds are probed: the builtin bank (the bank stage forwards to it), the per-node
//! processors, and the live route (through the graph's route-drain test hooks). The rack crate's
//! own tests probe the bank chain's hand-over to a bare `BankStage`.
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
        .attach_route_controls(NonZeroUsize::new(4).expect("depth"))
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
        let (_gate, mut owner, _exchange) = exchange(plan);
        let mute = RouteControlRecord::new([0.0; 4], true, 0).expect("a legal mute");
        let (mut drains, mut mixes) = (0, 0);
        for block in 0..BLOCKS {
            if block == 3 {
                producer.try_push(mute).expect("room");
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
