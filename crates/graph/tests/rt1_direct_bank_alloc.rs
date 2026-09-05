//! Isolated allocator proof for the public four-lane graph bank path.

use bench_support::alloc::{Mode, assert_installed, mode, set_mode};
use effect_contract::{BankWidth, LatencySamples, TailSamples};
use engine::{
    QuantumFrames,
    realtime::{self, PlanarBufferMut, RenderEnvelope, RenderError},
};
use graph::*;
use lane::Backend;

struct RestoreMode(Mode);
impl Drop for RestoreMode {
    fn drop(&mut self) {
        set_mode(self.0);
    }
}

struct Source(u32);
impl GraphRuntimeProcessor for Source {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        block.left.fill((self.0 + 1) as f32);
        block.right.fill(-((1_u32 << self.0) as f32));
        Ok(())
    }
}

#[derive(Default)]
struct IdentityBank(u64);
impl GraphPreparedBuiltinBankProcessor for IdentityBank {
    fn process(
        &mut self,
        _left: &mut [f32],
        _right: &mut [f32],
        _frames: u32,
        _first_sample: u64,
    ) -> Result<(), RenderError> {
        self.0 += 1;
        Ok(())
    }

    fn qualification_counters(&self) -> [u64; 2] {
        [self.0, self.0]
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

fn prepared_plan(folded: bool) -> engine::realtime::PreparedRenderPlan {
    const FRAMES: u32 = 11;
    let envelope = RenderEnvelope {
        sample_rate: engine::SampleRateHz(48_000),
        quantum: QuantumFrames(FRAMES),
        input_channels: None,
        output_channels: core::num::NonZeroUsize::new(2).expect("stereo"),
    };
    let inputs: Vec<_> = (0..4)
        .map(|lane| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("track{lane}")).expect("track id"),
            stage: TrackStage::Input,
        })
        .collect();
    let members: Vec<_> = (0..4)
        .map(|lane| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("track{lane}")).expect("track id"),
            stage: TrackStage::PostInputBuiltins,
        })
        .collect();
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("main").expect("output id"),
    };
    let routes: Vec<_> = (0..4)
        .map(|lane| GraphNodeId::Route {
            route_id: StableGraphId::parse(&format!("route{lane}")).expect("route id"),
        })
        .collect();
    let mut schedule = inputs.clone();
    schedule.extend(members.iter().cloned());
    if folded {
        schedule.extend(routes.iter().cloned());
    }
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
    nodes.sort_by(|left, right| left.id.cmp(&right.id));
    let mut edges = Vec::new();
    for lane in 0..4 {
        edges.push(GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: members[lane].clone(),
            },
            source: GraphPortId {
                node: inputs[lane].clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: members[lane].clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: format!("$.tracks[{lane}].builtin"),
        });
        let route_id = StableGraphId::parse(&format!("route{lane}")).expect("route id");
        edges.push(GraphEdge {
            id: GraphEdgeId::RouteSource {
                route_id: route_id.clone(),
            },
            source: GraphPortId {
                node: members[lane].clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: if folded {
                    routes[lane].clone()
                } else {
                    output.clone()
                },
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: format!("$.routes[{lane}]"),
        });
        if folded {
            edges.push(GraphEdge {
                id: GraphEdgeId::RouteDestination { route_id },
                source: GraphPortId {
                    node: routes[lane].clone(),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: output.clone(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: format!("$.routes[{lane}].destination"),
            });
        }
    }
    let mut required = inputs.clone();
    required.extend(members.iter().cloned());
    required.push(output.clone());
    let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id: 399,
        spec: GraphSpec {
            nodes,
            ports: Vec::new(),
            edges,
        },
        sequential_schedule: schedule,
        dependency_levels: vec![
            DependencyLevel {
                level: 0,
                nodes: inputs.clone(),
            },
            DependencyLevel {
                level: 1,
                nodes: members.clone(),
            },
            DependencyLevel {
                level: 2,
                nodes: if folded {
                    routes.clone()
                } else {
                    vec![output.clone()]
                },
            },
            DependencyLevel {
                level: 3,
                nodes: if folded {
                    vec![output.clone()]
                } else {
                    Vec::new()
                },
            },
        ]
        .into_iter()
        .filter(|level| !level.nodes.is_empty())
        .collect(),
        route_timings: Vec::new(),
        inserted_delays: Vec::new(),
        buffer_assignments: Vec::new(),
        estimate: estimate(),
        envelope,
        required_bindings: required.clone(),
        routes: if folded {
            routes
                .iter()
                .cloned()
                .map(|node| PreparedRoute {
                    node,
                    transform: RouteTransform {
                        gain: 1.0,
                        ll: 1.0,
                        lr: 0.0,
                        rl: 0.0,
                        rr: 1.0,
                    },
                })
                .collect()
        } else {
            Vec::new()
        },
        track_delays: Vec::new(),
        effects: Vec::new(),
        effect_controls: Vec::new(),
        effect_observations: Vec::new(),
        banks: Vec::new(),
        builtin_banks: vec![GraphPreparedBuiltinBank {
            backend: Backend::Simd4,
            members: members.clone().into_boxed_slice(),
            processor: Box::<IdentityBank>::default(),
            scratch: rack::AoSoaScratch::new(BankWidth::Four, FRAMES).expect("scratch"),
        }],
        observers: Vec::new(),
    });
    let nodes = required
        .into_iter()
        .filter(|node| !members.contains(node) && !routes.contains(node))
        .map(|node| match &node {
            GraphNodeId::TrackStage {
                track_id,
                stage: TrackStage::Input,
            } => {
                let lane = track_id
                    .as_str()
                    .strip_prefix("track")
                    .expect("prefix")
                    .parse()
                    .expect("lane");
                GraphNodeBinding::new(node, Box::new(Source(lane)))
            }
            _ => GraphNodeBinding::identity(node),
        })
        .collect();
    graph
        .bind(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        })
        .unwrap_or_else(|failure| panic!("bind: {}", failure.code))
}

#[test]
fn direct_bank_graph_render_is_allocation_free_and_bit_exact() {
    const FRAMES: usize = 11;
    assert_installed();
    let _restore = RestoreMode(mode());
    set_mode(Mode::Count);

    realtime::audit::warm_up();
    realtime::audit::reset();
    realtime::audit::in_render_scope(|| {
        let probe = Vec::<u8>::with_capacity(core::hint::black_box(64));
        core::hint::black_box(&probe);
        drop(probe);
    });
    let live = realtime::audit::snapshot();
    assert!(live.allocations > 0 && live.deallocations > 0);

    let mut plan = prepared_plan(false);
    let mut pcm = [f32::from_bits(0x7fc0_3990); FRAMES * 2];
    realtime::audit::reset();
    for block in 0..16 {
        let output = PlanarBufferMut::try_new(&mut pcm, 2, FRAMES, FRAMES).expect("output");
        plan.render(
            realtime::RenderIo {
                input: None,
                output,
            },
            realtime::RenderTime {
                absolute_sample: (block * FRAMES) as u64,
            },
        )
        .expect("render");
    }
    let measured = realtime::audit::snapshot();
    assert_eq!((measured.allocations, measured.deallocations), (0, 0));
    assert!(
        pcm[..FRAMES]
            .iter()
            .all(|sample| sample.to_bits() == 10.0_f32.to_bits())
    );
    assert!(
        pcm[FRAMES..]
            .iter()
            .all(|sample| sample.to_bits() == (-15.0_f32).to_bits())
    );
    assert_eq!(plan.qualification_counters(), [16, 16]);

    let mut folded = prepared_plan(true);
    realtime::audit::reset();
    for block in 0..16 {
        let output = PlanarBufferMut::try_new(&mut pcm, 2, FRAMES, FRAMES).expect("output");
        folded
            .render(
                realtime::RenderIo {
                    input: None,
                    output,
                },
                realtime::RenderTime {
                    absolute_sample: (block * FRAMES) as u64,
                },
            )
            .expect("folded render");
    }
    let folded_measured = realtime::audit::snapshot();
    assert_eq!(
        (folded_measured.allocations, folded_measured.deallocations),
        (0, 0)
    );
    assert!(
        pcm[..FRAMES]
            .iter()
            .all(|sample| sample.to_bits() == 10.0_f32.to_bits())
    );
    assert!(
        pcm[FRAMES..]
            .iter()
            .all(|sample| sample.to_bits() == (-15.0_f32).to_bits())
    );
    assert_eq!(folded.qualification_counters(), [16, 16]);
}
