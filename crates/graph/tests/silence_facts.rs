//! Issue #2133: silence is a propagated fact. A submix input's flag is the AND of its routes'
//! facts: a route is silent when its source is, or when it is muted and its ramp has settled
//! (#1217 D4: its destination then leaves it out of the sum). Nothing reads a word to decide it
//! but the track inputs, which this plan feeds through host processors, the one writer the engine
//! has no fact for.
#![cfg(feature = "test-support")]

use core::num::NonZeroUsize;
use effect_contract::LatencySamples;
use engine::{
    QuantumFrames, SampleRateHz,
    realtime::{self, PlanarBufferMut, RenderEnvelope, RenderError},
};
use graph::*;

const FRAMES: usize = 4;
/// The mute's ramp: two blocks.
const RAMP: u32 = 8;

/// A track input whose blocks listed in `silent` are all `+0.0`, the rest a tone on both planes.
struct Source {
    silent: u32,
}
impl GraphRuntimeProcessor for Source {
    fn retained_bytes(&self) -> u64 {
        0
    }
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let at = block.first_sample / FRAMES as u64;
        let silent = self.silent & (1 << at) != 0;
        for (frame, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            *left = if silent {
                0.0
            } else {
                (frame + 1) as f32 * 0.125
            };
            *right = *left;
        }
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

/// Two tracks, each sending through a live route into one submix input, which feeds the output.
/// `silent[t]` lists the blocks in which track `t`'s input is `+0.0`. Returns the plan and the
/// two routes' producers.
fn prepared(silent: [u32; 2]) -> (realtime::PreparedRenderPlan, Vec<GraphRouteControlProducer>) {
    let envelope = RenderEnvelope {
        sample_rate: SampleRateHz(48_000),
        quantum: QuantumFrames(FRAMES as u32),
        output_channels: NonZeroUsize::new(2).expect("stereo"),
    };
    let input = |name: &str| GraphNodeId::TrackStage {
        track_id: StableGraphId::parse(name).expect("track"),
        stage: TrackStage::Input,
    };
    let route_id = |name: &str| StableGraphId::parse(name).expect("route");
    let tracks = [input("track0"), input("track1")];
    let routes = [
        GraphNodeId::Route {
            route_id: route_id("send0"),
        },
        GraphNodeId::Route {
            route_id: route_id("send1"),
        },
    ];
    let bus = input("bus");
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("main").expect("output"),
    };
    let schedule: Vec<_> = tracks
        .iter()
        .chain(&routes)
        .chain([&bus, &output])
        .cloned()
        .collect();
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
    let mut connect = |source: &GraphNodeId, destination: &GraphNodeId, id| {
        edges.push(GraphEdge {
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
            path: "$.silence_facts".to_owned(),
        })
    };
    for (index, (track, route)) in tracks.iter().zip(&routes).enumerate() {
        let id = route_id(&format!("send{index}"));
        connect(
            track,
            route,
            GraphEdgeId::RouteSource {
                route_id: id.clone(),
            },
        );
        connect(route, &bus, GraphEdgeId::RouteDestination { route_id: id });
    }
    connect(
        &bus,
        &output,
        GraphEdgeId::RouteSource {
            route_id: route_id("busout"),
        },
    );
    let levels = [
        tracks.to_vec(),
        routes.to_vec(),
        vec![bus.clone()],
        vec![output.clone()],
    ]
    .into_iter()
    .enumerate()
    .map(|(level, nodes)| DependencyLevel {
        level: level as u64,
        nodes,
    })
    .collect();
    let unity = RouteTransform {
        gain: 1.0,
        ll: 1.0,
        lr: 0.0,
        rl: 0.0,
        rr: 1.0,
    };
    let mut graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id: 2133,
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
        required_bindings: tracks.iter().chain([&bus, &output]).cloned().collect(),
        routes: routes
            .iter()
            .map(|node| PreparedRoute {
                node: node.clone(),
                transform: unity,
                gate: RouteGate::OPEN,
            })
            .collect(),
        track_delays: vec![],
        effects: vec![],
        effect_controls: vec![],
        effect_observations: vec![],
        banks: vec![],
        builtin_banks: vec![],
        observers: vec![],
    });
    let producers = graph
        .attach_route_controls(&realtime::LiveValuesSuperseded::new())
        .expect("both routes into the submix are live");
    assert_eq!(producers.len(), 2);
    let mut bindings: Vec<_> = tracks
        .iter()
        .zip(silent)
        .map(|(node, silent)| GraphNodeBinding::new(node.clone(), Box::new(Source { silent })))
        .collect();
    bindings.push(GraphNodeBinding::identity(bus));
    bindings.push(GraphNodeBinding::identity(output));
    let plan = graph
        .bind(GraphRuntimeBindings {
            envelope,
            nodes: bindings,
            observers: vec![],
        })
        .unwrap_or_else(|failure| panic!("bind {}", failure.code));
    (plan, producers)
}

/// Gate 2. Track 1 is silent until block 5. Track 0 plays from block 1, and a mute ramping over
/// [`RAMP`] frames is published on its route before block 2, so the route mixes blocks 2 and 3 on
/// its way down and is inactive (settled) from block 4. Track 1 is silent again from block 6, and
/// its route is retargeted while it is: before block 6 to a left row of `-1.0` and `-0.0`, which
/// mixes its silence to `-0.0`; before block 8 to a left row of `1.0` and `1.0`; before block 9 to
/// a ramp of both left columns over [`RAMP`] frames to a `+0.0` target, whose coefficients can
/// pass the target. The submix input is flagged exactly in blocks 0 (both sources silent), 4
/// (track 0 muted and settled, track 1 silent) and 8 (a positive row).
///
/// Test value: red when a submix is flagged silent while a route still carries signal: a route
/// counted by its mute bit instead of its settled activity (blocks 2 and 3), a destination that
/// leaves no muted route out (block 4 not set), a sum that ignores one of its inputs (blocks 1 and
/// 5), a route whose row is negative on both columns counted silent (blocks 6 and 7), or a ramp
/// whose row moves on both columns counted silent (blocks 9 and 10).
#[test]
fn a_submix_input_is_silent_iff_every_route_into_it_is() {
    const BLOCKS: usize = 11;
    test_only_state_every_silence_fact(true);
    let (mut plan, mut producers) = prepared([1 << 0, 0b111_1101_1111]);
    let mut gate = plan.take_revision_gate().expect("a fresh plan's gate");
    let record = |target, mute, length| {
        RouteControlRecord::new(target, mute, length).expect("a legal route record")
    };
    let writes = [
        (2, 0, record([0.0; 4], true, RAMP)),
        (6, 1, record([-1.0, -0.0, 0.0, 1.0], false, 0)),
        (8, 1, record([1.0, 1.0, 0.0, 1.0], false, 0)),
        (9, 1, record([0.0, 0.0, 0.0, 1.0], false, RAMP)),
    ];
    let mut flags = Vec::new();
    for block in 0..BLOCKS {
        for &(at, route, write) in &writes {
            if at == block {
                let revision = gate.last_published() + 1;
                let stamp = gate.stamp(revision).expect("a legal revision");
                producers[route].write(stamp, write);
                gate.publish(revision);
            }
        }
        let mut pcm = vec![1.0_f32; 2 * FRAMES];
        plan.render(
            realtime::RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, FRAMES, FRAMES).expect("output"),
            },
            realtime::RenderTime {
                absolute_sample: (block * FRAMES) as u64,
            },
        )
        .expect("render");
        flags.push(test_only_route_destination_silence(&plan));
    }
    test_only_state_every_silence_fact(false);
    assert_eq!(
        flags,
        [
            true, false, false, false, true, false, false, false, true, false, false
        ]
        .map(|flag| vec![flag]),
        "the submix input's flag, block by block"
    );
}
