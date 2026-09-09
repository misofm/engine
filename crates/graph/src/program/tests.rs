use super::*;
use crate::{GraphEdge, GraphNode, GraphPortId, StableGraphId};
use effect_contract::{LatencySamples, TailSamples};

fn gid(value: &str) -> StableGraphId {
    StableGraphId::parse(value).expect("static id")
}
fn stage_node(track: &str, stage: TrackStage) -> GraphNodeId {
    GraphNodeId::TrackStage {
        track_id: gid(track),
        stage,
    }
}
fn node(id: GraphNodeId) -> GraphNode {
    GraphNode {
        id,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
    }
}
fn port(node: GraphNodeId, kind: GraphPortKind) -> GraphPortId {
    GraphPortId {
        node,
        kind,
        effect_port: None,
    }
}
fn main_edge(id: GraphEdgeId, source: GraphNodeId, destination: GraphNodeId) -> GraphEdge {
    GraphEdge {
        id,
        source: port(source, GraphPortKind::MainOutput),
        destination: port(destination, GraphPortKind::MainInput),
        path: "$".to_owned(),
    }
}
/// A spec plus the level-major schedule the graph compiler would emit for it.
///
/// The compiler sorts nodes and edges by id and emits levels in ascending node-id order, so
/// the fixtures do the same rather than assuming a hand-written order is canonical.
fn build(
    mut nodes: Vec<GraphNode>,
    mut edges: Vec<GraphEdge>,
) -> (GraphSpec, Vec<GraphNodeId>, Vec<DependencyLevel>) {
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    edges.sort_by(|a, b| a.id.cmp(&b.id));
    let mut level_of: std::collections::BTreeMap<GraphNodeId, u64> =
        nodes.iter().map(|node| (node.id.clone(), 0_u64)).collect();
    // Longest-path levels; the fixtures are small, so iterate to a fixed point.
    for _ in 0..nodes.len() {
        for edge in &edges {
            let source = level_of[&edge.source.node];
            let destination = level_of.get_mut(&edge.destination.node).expect("node");
            *destination = (*destination).max(source + 1);
        }
    }
    let depth = level_of.values().copied().max().unwrap_or(0);
    let levels: Vec<DependencyLevel> = (0..=depth)
        .map(|level| DependencyLevel {
            level,
            nodes: nodes
                .iter()
                .filter(|node| level_of[&node.id] == level)
                .map(|node| node.id.clone())
                .collect(),
        })
        .filter(|level| !level.nodes.is_empty())
        .collect();
    let schedule: Vec<GraphNodeId> = levels
        .iter()
        .flat_map(|level| level.nodes.iter().cloned())
        .collect();
    let spec = GraphSpec {
        ports: Vec::new(),
        nodes,
        edges,
    };
    (spec, schedule, levels)
}

/// One track, no effects, one route, one output: the shape that cost seven schedule items and
/// six full buffer copies per block before lowering.
fn plain_track(track: &str, routes: &[&str]) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    let stages = [
        TrackStage::Input,
        TrackStage::PostInputBuiltins,
        TrackStage::PostSimd1,
        TrackStage::PostDynamic,
        TrackStage::PostSimd2PreFader,
        TrackStage::PostFader,
        TrackStage::PostMatrix,
    ];
    let mut nodes: Vec<GraphNode> = stages
        .iter()
        .map(|stage| node(stage_node(track, *stage)))
        .collect();
    let mut edges: Vec<GraphEdge> = stages
        .windows(2)
        .map(|pair| {
            main_edge(
                GraphEdgeId::TrackMain {
                    target: stage_node(track, pair[1]),
                },
                stage_node(track, pair[0]),
                stage_node(track, pair[1]),
            )
        })
        .collect();
    nodes.push(node(GraphNodeId::Output {
        output_id: gid("out"),
    }));
    for route in routes {
        let route_node = GraphNodeId::Route {
            route_id: gid(route),
        };
        nodes.push(node(route_node.clone()));
        edges.push(main_edge(
            GraphEdgeId::RouteSource {
                route_id: gid(route),
            },
            stage_node(track, TrackStage::PostMatrix),
            route_node.clone(),
        ));
        edges.push(main_edge(
            GraphEdgeId::RouteDestination {
                route_id: gid(route),
            },
            route_node,
            GraphNodeId::Output {
                output_id: gid("out"),
            },
        ));
    }
    (nodes, edges)
}

/// The headline #99 F2 result: nine semantic nodes and eight edges become six ops, three
/// aliases and a two-buffer arena.
///
/// Before lowering this graph executed nine schedule items, allocated one contribution buffer
/// per edge (eight) plus nine coloured node outputs, copied every edge every block, and ran a
/// one-element pairwise reduction at every node.
#[test]
fn chain_of_seven_stages_lowers_to_six_ops_three_taps_and_two_buffers() {
    let (nodes, edges) = plain_track("t", &["r"]);
    let (spec, schedule, levels) = build(nodes, edges);
    assert_eq!(spec.nodes.len(), 9);
    assert_eq!(spec.edges.len(), 8);
    let program = lower(&spec, &schedule, &levels, &[], &[]).expect("lowers");

    assert_eq!(program.ops.len(), 6);
    assert_eq!(program.taps.len(), 3);
    assert_eq!(program.buffers, 2);
    // Nothing needs a per-frame reduction: every op has exactly one input.
    assert_eq!(program.reduction_count(), 0);
    assert_eq!(program.delayed_input_count(), 0);

    let op_nodes: Vec<&GraphNodeId> = program
        .ops
        .iter()
        .map(|op| &spec.nodes[op.node as usize].id)
        .collect();
    assert_eq!(
        op_nodes,
        vec![
            &stage_node("t", TrackStage::Input),
            &stage_node("t", TrackStage::PostInputBuiltins),
            &stage_node("t", TrackStage::PostFader),
            &stage_node("t", TrackStage::PostMatrix),
            &GraphNodeId::Route { route_id: gid("r") },
            &GraphNodeId::Output {
                output_id: gid("out")
            },
        ]
    );
    // Input has no producer; the builtin stage is bank-eligible so it must own its buffer;
    // everything downstream of it reads in place.
    let in_place: Vec<bool> = program.ops.iter().map(|op| op.in_place).collect();
    assert_eq!(in_place, vec![false, false, false, true, true, true]);

    // The three elided stages alias the builtin stage's buffer and observe right after it.
    for tap in &program.taps {
        assert_eq!(
            tap.buffer,
            program.node_buffer[program.ops[1].node as usize]
        );
        assert_eq!(tap.after_op, 1);
        assert!(program.node_op[tap.node as usize].is_none());
    }
    assert_eq!(program.output, program.ops[5].output);
}

/// Two routes off one tap: the shared buffer has two readers, so neither route may consume it
/// in place, and the output op keeps a genuine two-input reduction.
#[test]
fn fan_out_blocks_in_place_and_fan_in_keeps_its_reduction() {
    let (nodes, edges) = plain_track("t", &["ra", "rb"]);
    let (spec, schedule, levels) = build(nodes, edges);
    let program = lower(&spec, &schedule, &levels, &[], &[]).expect("lowers");

    assert_eq!(program.taps.len(), 3);
    assert_eq!(program.reduction_count(), 1);
    let output_op = program.ops.last().expect("output op");
    assert_eq!(output_op.input_count(), 2);
    assert!(!output_op.in_place);
    // The two route inputs of the output are distinct buffers, in stable edge-ID order.
    let output_inputs = program.inputs_of(output_op);
    assert_ne!(output_inputs[0].buffer, output_inputs[1].buffer);

    let routes: Vec<&Op> = program
        .ops
        .iter()
        .filter(|op| matches!(spec.nodes[op.node as usize].id, GraphNodeId::Route { .. }))
        .collect();
    assert_eq!(routes.len(), 2);
    for route in routes {
        assert!(
            !route.in_place,
            "a route reading a two-reader buffer must not alias it"
        );
    }
}

#[test]
fn sixty_four_plain_tracks_keep_one_ordered_non_aliasing_master_reduction() {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for index in 0..64 {
        let track = format!("track{index:02}");
        let route = format!("route{index:02}");
        let (mut track_nodes, mut track_edges) = plain_track(&track, &[&route]);
        track_nodes.retain(|node| !matches!(node.id, GraphNodeId::Output { .. }));
        nodes.append(&mut track_nodes);
        edges.append(&mut track_edges);
    }
    nodes.push(node(GraphNodeId::Output {
        output_id: gid("out"),
    }));
    let (spec, schedule, levels) = build(nodes, edges);
    let program = lower(&spec, &schedule, &levels, &[], &[]).expect("plumbing lowers");
    let master = program.ops.last().expect("master output");
    let inputs = program.inputs_of(master);
    assert_eq!(inputs.len(), 64);
    assert!(!master.in_place);
    assert!(inputs.iter().all(|input| input.buffer != master.output));
    let expected: Vec<_> = (0..64)
        .map(|index| {
            let id = GraphNodeId::Route {
                route_id: gid(&format!("route{index:02}")),
            };
            let node = spec
                .nodes
                .iter()
                .position(|candidate| candidate.id == id)
                .expect("route node");
            program.node_buffer[node]
        })
        .collect();
    assert_eq!(
        inputs.iter().map(|input| input.buffer).collect::<Vec<_>>(),
        expected
    );
}

/// A PDC edge stages into a scratch buffer that is returned to the arena after its op, and its
/// consumer is never in place -- the delayed samples are not the producer's buffer.
#[test]
fn delayed_edge_gets_staging_buffer_and_blocks_in_place() {
    let (nodes, edges) = plain_track("t", &["r"]);
    let (spec, schedule, levels) = build(nodes, edges);
    let delayed = InsertedDelay {
        node: GraphNodeId::Route { route_id: gid("r") },
        edge_id: GraphEdgeId::RouteSource { route_id: gid("r") },
        samples: LatencySamples(64),
    };
    let program = lower(
        &spec,
        &schedule,
        &levels,
        std::slice::from_ref(&delayed),
        &[],
    )
    .expect("lowers");

    assert_eq!(program.delays.len(), 1);
    assert_eq!(program.delays[0].samples, 64);
    assert_eq!(program.delayed_input_count(), 1);
    let route_op = program
        .ops
        .iter()
        .find(|op| matches!(spec.nodes[op.node as usize].id, GraphNodeId::Route { .. }))
        .expect("route op");
    assert!(
        !route_op.in_place,
        "a delayed input cannot be consumed in place"
    );
    let input = program.inputs_of(route_op)[0];
    let delay = input.delay.expect("delayed input");
    assert_eq!(delay.line, 0);
    assert_ne!(delay.staging, input.buffer);
    assert!(delay.staging.0 < program.buffers);
}

/// An elided stage is exactly a stage with one undelayed, un-sidechained input: give the same
/// stage a PDC edge and it keeps its op.
#[test]
fn a_delayed_stage_boundary_is_not_elided() {
    let (nodes, edges) = plain_track("t", &["r"]);
    let (spec, schedule, levels) = build(nodes, edges);
    let delayed = InsertedDelay {
        node: stage_node("t", TrackStage::PostSimd1),
        edge_id: GraphEdgeId::TrackMain {
            target: stage_node("t", TrackStage::PostSimd1),
        },
        samples: LatencySamples(8),
    };
    let program = lower(
        &spec,
        &schedule,
        &levels,
        std::slice::from_ref(&delayed),
        &[],
    )
    .expect("lowers");
    assert_eq!(program.taps.len(), 2);
    assert_eq!(program.ops.len(), 7);
    let index = node_index(&spec, &stage_node("t", TrackStage::PostSimd1)).expect("node");
    assert!(program.node_op[index as usize].is_some());
}

/// A dynamic-rack effect is not bank-eligible, so the two stage boundaries after it are pure
/// aliases of *its* buffer and the fader downstream still reads that buffer in place.
///
/// This is the fixture that makes the "a tap is not a reader" rule observable. In the
/// effect-free chain every alias chain is rooted at the bank-eligible builtin stage, whose
/// buffer is never consumed in place for an unrelated reason, so miscounting taps as readers
/// changes nothing there. Here the root is an ordinary effect: count the two taps as readers
/// and the fader stops aliasing, costing a buffer and a full copy per block.
#[test]
fn taps_are_not_readers_so_an_alias_chain_still_folds_into_its_producer() {
    let track = "t";
    let effect = GraphNodeId::Effect(crate::EffectNodeId {
        track_id: gid(track),
        rack: RackId::Dynamic,
        effect_id: gid("d"),
    });
    let (mut nodes, mut edges) = plain_track(track, &["r"]);
    nodes.push(node(effect.clone()));
    // Splice the effect between PostSimd1 and PostDynamic.
    edges.retain(|edge| {
        edge.id
            != GraphEdgeId::TrackMain {
                target: stage_node(track, TrackStage::PostDynamic),
            }
    });
    edges.push(main_edge(
        GraphEdgeId::TrackMain {
            target: effect.clone(),
        },
        stage_node(track, TrackStage::PostSimd1),
        effect.clone(),
    ));
    edges.push(main_edge(
        GraphEdgeId::TrackMain {
            target: stage_node(track, TrackStage::PostDynamic),
        },
        effect.clone(),
        stage_node(track, TrackStage::PostDynamic),
    ));
    let (spec, schedule, levels) = build(nodes, edges);
    let program = lower(&spec, &schedule, &levels, &[], &[]).expect("lowers");

    let effect_index = node_index(&spec, &effect).expect("effect node");
    let effect_op = program.node_op[effect_index as usize].expect("effect keeps its op");
    assert!(!program.ops[effect_op as usize].in_place);

    // PostDynamic and PostSimd2PreFader alias the effect's buffer and observe right after it.
    let aliased: Vec<&Tap> = program
        .taps
        .iter()
        .filter(|tap| tap.buffer == program.node_buffer[effect_index as usize])
        .collect();
    assert_eq!(aliased.len(), 2);
    for tap in aliased {
        assert_eq!(tap.after_op, effect_op);
    }

    // The fader is the effect buffer's only real reader, so it consumes it in place.
    let fader_index = node_index(&spec, &stage_node(track, TrackStage::PostFader)).expect("fader");
    let fader_op = program.node_op[fader_index as usize].expect("fader keeps its op");
    assert!(
        program.ops[fader_op as usize].in_place,
        "an alias chain with one real reader must fold into its producer's buffer"
    );
    assert_eq!(
        program.ops[fader_op as usize].output,
        program.node_buffer[effect_index as usize]
    );
}

/// A symbolic value: what a buffer holds, as an expression over node outputs.
///
/// `Sum` keeps its operand order, so a reduction whose inputs were reordered is a different
/// value, not an equal one.
#[derive(Clone, Debug, Eq, PartialEq)]
enum Expr {
    Silence,
    Node(NodeIndex, Box<Expr>),
    Delayed(Box<Expr>, u64),
    Sum(Vec<Expr>),
}

/// Evaluate the *semantic* graph the naive way: every node consumes the ordered, individually
/// delayed outputs of its incoming main edges. An identity stage boundary is transparent,
/// because that is exactly what the executor's `RuntimeNodeKind::Identity` does with a
/// single-input pairwise sum (`balanced_pairwise_sum` of one element returns it unchanged).
fn evaluate_spec(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    delays: &[InsertedDelay],
) -> Vec<Expr> {
    let mut value = vec![Expr::Silence; spec.nodes.len()];
    for id in schedule {
        let index = node_index(spec, id).expect("node") as usize;
        let mut operands = Vec::new();
        for edge in &spec.edges {
            if &edge.destination.node != id || edge.destination.kind != GraphPortKind::MainInput {
                continue;
            }
            let source = node_index(spec, &edge.source.node).expect("node") as usize;
            let samples = delays
                .iter()
                .find(|delay| delay.edge_id == edge.id)
                .map(|delay| delay.samples.0)
                .filter(|samples| *samples != 0);
            operands.push(match samples {
                Some(samples) => Expr::Delayed(Box::new(value[source].clone()), samples),
                None => value[source].clone(),
            });
        }
        let combined = match operands.len() {
            0 => Expr::Silence,
            1 => operands.remove(0),
            _ => Expr::Sum(operands),
        };
        // An identity stage boundary carries its input unchanged whether or not lowering
        // elides it: elision removes the schedule item, never a transformation. Modelling it
        // as transparent on both sides is what makes the delayed case (which keeps its op)
        // comparable to the undelayed case (which becomes an alias).
        value[index] = if is_alias_candidate(id) {
            combined
        } else {
            Expr::Node(u32::try_from(index).expect("index"), Box::new(combined))
        };
    }
    value
}

/// Interpret the *program* over an arena of symbolic buffers and compare, node by node.
///
/// This is the check that colouring is sound: if an op were given storage another op still
/// needs, the later read returns the wrong expression and the comparison fails. It is also the
/// check that in-place folding and aliasing preserve dataflow.
fn assert_program_matches_spec(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    delays: &[InsertedDelay],
    program: &ExecutionProgram,
) {
    let expected = evaluate_spec(spec, schedule, delays);
    let mut arena = vec![Expr::Silence; program.buffers as usize];
    let mut taps_by_op: std::collections::BTreeMap<OpIndex, Vec<&Tap>> =
        std::collections::BTreeMap::new();
    for tap in &program.taps {
        taps_by_op.entry(tap.after_op).or_default().push(tap);
    }
    for (op_index, op) in program.ops.iter().enumerate() {
        let mut operands = Vec::new();
        for input in program.inputs_of(op) {
            let value = arena[input.buffer.0 as usize].clone();
            operands.push(match input.delay {
                Some(delay) => {
                    Expr::Delayed(Box::new(value), program.delays[delay.line as usize].samples)
                }
                None => value,
            });
        }
        let combined = match operands.len() {
            0 => Expr::Silence,
            1 => operands.remove(0),
            _ => Expr::Sum(operands),
        };
        if op.in_place {
            // The single input already lives in `output`; nothing is copied.
            assert_eq!(
                op.output,
                program.inputs_of(op)[0].buffer,
                "an in-place op must write its own input buffer"
            );
        }
        // Same rule as the reference side: an identity stage boundary that kept its op
        // (because its input is delayed) still transforms nothing.
        let id = &spec.nodes[op.node as usize].id;
        arena[op.output.0 as usize] = if is_alias_candidate(id) {
            combined
        } else {
            Expr::Node(op.node, Box::new(combined))
        };
        assert_eq!(
            arena[op.output.0 as usize], expected[op.node as usize],
            "op {op_index} produced the wrong value"
        );
        // Every alias attached here must already read the value its node has semantically.
        for tap in taps_by_op.get(&(op_index as OpIndex)).into_iter().flatten() {
            assert_eq!(
                arena[tap.buffer.0 as usize], expected[tap.node as usize],
                "alias for node {} observes the wrong value",
                tap.node
            );
        }
    }
    let output_node = spec
        .nodes
        .iter()
        .position(|node| matches!(node.id, GraphNodeId::Output { .. }))
        .expect("output");
    assert_eq!(arena[program.output.0 as usize], expected[output_node]);
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

/// #99 F2, the load-bearing eval: over 300 seeded multi-track graphs with effects, fan-out,
/// fan-in and PDC, the lowered program computes exactly the value the semantic graph does,
/// and its arena is bounded by the graph's live width rather than its edge count.
///
/// The comparison is symbolic, so it does not depend on either executor existing yet, and the
/// reference side never looks at the program: it walks `spec`/`schedule`/`delays` naively,
/// node by node, exactly as the pre-#99 render loop did.
#[test]
fn lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs() {
    let mut state = 0x5deb_c0de_1234_9e37_u64;
    for graph in 0..300_u32 {
        let track_count = (xorshift(&mut state) % 4) as usize + 1;
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut route_index = 0usize;
        for track in 0..track_count {
            let name = format!("t{track:02}");
            let routes: Vec<String> = (0..(xorshift(&mut state) % 2) + 1)
                .map(|_| {
                    route_index += 1;
                    format!("r{route_index:02}")
                })
                .collect();
            let borrowed: Vec<&str> = routes.iter().map(String::as_str).collect();
            let (track_nodes, track_edges) = plain_track(&name, &borrowed);
            // Splice an effect into one rack, sometimes bank-eligible and sometimes not.
            let rack = match xorshift(&mut state) % 3 {
                0 => None,
                1 => Some((RackId::Dynamic, TrackStage::PostDynamic)),
                _ => Some((RackId::Simd1, TrackStage::PostSimd1)),
            };
            let mut track_nodes = track_nodes;
            let mut track_edges = track_edges;
            if let Some((rack, stage)) = rack {
                let upstream = match stage {
                    TrackStage::PostSimd1 => TrackStage::PostInputBuiltins,
                    _ => TrackStage::PostSimd1,
                };
                let effect = GraphNodeId::Effect(crate::EffectNodeId {
                    track_id: gid(&name),
                    rack,
                    effect_id: gid("fx"),
                });
                track_nodes.push(node(effect.clone()));
                track_edges.retain(|edge| {
                    edge.id
                        != GraphEdgeId::TrackMain {
                            target: stage_node(&name, stage),
                        }
                });
                track_edges.push(main_edge(
                    GraphEdgeId::TrackMain {
                        target: effect.clone(),
                    },
                    stage_node(&name, upstream),
                    effect.clone(),
                ));
                track_edges.push(main_edge(
                    GraphEdgeId::TrackMain {
                        target: stage_node(&name, stage),
                    },
                    effect,
                    stage_node(&name, stage),
                ));
            }
            nodes.extend(track_nodes.into_iter().filter(|candidate| {
                !matches!(candidate.id, GraphNodeId::Output { .. }) || track == 0
            }));
            edges.extend(track_edges);
        }
        let (spec, schedule, levels) = build(nodes, edges);
        // PDC on a random subset of edges.
        let mut delays: Vec<InsertedDelay> = Vec::new();
        for edge in &spec.edges {
            if !xorshift(&mut state).is_multiple_of(4) {
                continue;
            }
            delays.push(InsertedDelay {
                node: edge.destination.node.clone(),
                edge_id: edge.id.clone(),
                samples: LatencySamples(xorshift(&mut state) % 128 + 1),
            });
        }
        let program = lower(&spec, &schedule, &levels, &delays, &[])
            .unwrap_or_else(|error| panic!("graph {graph}: {error:?}"));

        assert_program_matches_spec(&spec, &schedule, &delays, &program);

        // Structure: every node is either an op or an alias, never both and never neither.
        assert_eq!(
            program.ops.len() + program.taps.len(),
            spec.nodes.len(),
            "graph {graph}: nodes are neither op nor alias"
        );
        for (index, op) in program.node_op.iter().enumerate() {
            let tapped = program.taps.iter().any(|tap| tap.node as usize == index);
            assert_eq!(op.is_none(), tapped, "graph {graph}: node {index}");
        }
        // Ops stay level-major, and within a level in ascending node id.
        assert!(
            program
                .ops
                .windows(2)
                .all(|pair| (pair[0].level, pair[0].node) < (pair[1].level, pair[1].node)),
            "graph {graph}: ops are not level-major"
        );
        // Once a dedicated node has written its buffer, nothing else may write it.
        //
        // Dedication is forward-only, and deliberately so: inheriting storage a *dead* buffer
        // used earlier is fine and is what keeps the arena small. What it forbids is handing
        // the slot on once the node owns it. The symbolic interpreter above cannot see this,
        // because it compares values op by op and a recycled slot is only wrong once someone
        // reads it, so it is checked structurally here.
        //
        // This is *not* the bank invariant, despite `is_dedicated` predating banks and
        // covering most bank-eligible nodes. A bank's hazard is its reordering window; see
        // `bank_window_hoisting_preserves_dataflow_on_random_graphs` and `lower` (#169).
        let mut open: std::collections::BTreeMap<BufferRef, usize> =
            std::collections::BTreeMap::new();
        for (at, op) in program.ops.iter().enumerate() {
            if let Some(since) = open.get(&op.output) {
                panic!(
                    "graph {graph}: op {at} writes buffer {:?}, held by a bank-eligible node \
                     since op {since}",
                    op.output
                );
            }
            for input in program.inputs_of(op) {
                if let Some(delay) = input.delay {
                    assert!(
                        !open.contains_key(&delay.staging),
                        "graph {graph}: op {at} stages PDC into open bank storage"
                    );
                }
            }
            if is_dedicated(&spec.nodes[op.node as usize].id) {
                open.insert(op.output, at);
            }
        }

        // Arena bound: at most one buffer per op, and strictly fewer than the pre-#99 model,
        // which allocated one contribution buffer per edge on top of a coloured output each.
        assert!(
            program.buffers as usize <= program.ops.len(),
            "graph {graph}: arena is larger than the op count"
        );
        assert!(
            (program.buffers as usize) < spec.edges.len() + spec.nodes.len(),
            "graph {graph}: arena is no smaller than the per-edge model it replaces"
        );
    }
}

/// The three rejections a caller maps to `graph.internal.invariant`.
#[test]
fn malformed_inputs_are_rejected_rather_than_lowered() {
    let (nodes, edges) = plain_track("t", &["r"]);
    let (spec, schedule, levels) = build(nodes, edges);
    assert_eq!(
        lower(&spec, &schedule[..schedule.len() - 1], &levels, &[], &[]),
        Err(ProgramError::ScheduleMismatch)
    );
    let mut swapped = schedule.clone();
    swapped.swap(0, 1);
    assert_eq!(
        lower(&spec, &swapped, &levels, &[], &[]),
        Err(ProgramError::ScheduleMismatch)
    );
    let mut unsorted = spec.clone();
    unsorted.nodes.swap(0, 1);
    assert_eq!(
        lower(&unsorted, &schedule, &levels, &[], &[]),
        Err(ProgramError::SpecUnsorted)
    );
}

// ---- issue #169: colouring across a bank's reordering window ---------------------------

/// A track whose dynamic rack carries one effect, spliced between the two stage boundaries
/// that surround it, plus one route to the shared output.
fn dynamic_track(track: &str, route: &str) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    let (mut nodes, mut edges) = plain_track(track, &[route]);
    let effect = GraphNodeId::Effect(crate::EffectNodeId {
        track_id: gid(track),
        rack: RackId::Dynamic,
        effect_id: gid("fx"),
    });
    nodes.push(node(effect.clone()));
    edges.retain(|edge| {
        edge.id
            != GraphEdgeId::TrackMain {
                target: stage_node(track, TrackStage::PostDynamic),
            }
    });
    edges.push(main_edge(
        GraphEdgeId::TrackMain {
            target: effect.clone(),
        },
        stage_node(track, TrackStage::PostSimd1),
        effect.clone(),
    ));
    edges.push(main_edge(
        GraphEdgeId::TrackMain {
            target: stage_node(track, TrackStage::PostDynamic),
        },
        effect,
        stage_node(track, TrackStage::PostDynamic),
    ));
    (nodes, edges)
}

fn dynamic_effect(track: &str) -> GraphNodeId {
    GraphNodeId::Effect(crate::EffectNodeId {
        track_id: gid(track),
        rack: RackId::Dynamic,
        effect_id: gid("fx"),
    })
}

/// `node index -> (bank, lane)`, the shape `runtime::BankMembership` has.
fn member_lanes(
    spec: &GraphSpec,
    banks: &[Vec<GraphNodeId>],
) -> std::collections::BTreeMap<u32, (usize, usize)> {
    let mut lanes = std::collections::BTreeMap::new();
    for (bank, members) in banks.iter().enumerate() {
        for (lane, id) in members.iter().enumerate() {
            lanes.insert(
                node_index(spec, id).expect("member is a node"),
                (bank, lane),
            );
        }
    }
    lanes
}

/// The op groups the executor will run, in the order it will run them.
///
/// This mirrors `runtime::units_of` exactly, and it is the whole reason #169 exists: a bank
/// becomes **one unit at its first member's position**, so a member scheduled later is hoisted
/// forward and every non-member op between the members is deferred past the bank.
fn units_in_runtime_order(
    program: &ExecutionProgram,
    lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
) -> Vec<Vec<usize>> {
    let mut units: Vec<Vec<usize>> = Vec::with_capacity(program.ops.len());
    let mut emitted: std::collections::BTreeSet<usize> = std::collections::BTreeSet::new();
    for (index, op) in program.ops.iter().enumerate() {
        match lanes.get(&op.node) {
            None => units.push(vec![index]),
            Some((bank, _)) => {
                if !emitted.insert(*bank) {
                    continue;
                }
                let mut members: Vec<(usize, usize)> = program
                    .ops
                    .iter()
                    .enumerate()
                    .filter_map(|(other, candidate)| {
                        lanes.get(&candidate.node).and_then(|(other_bank, lane)| {
                            (*other_bank == *bank).then_some((*lane, other))
                        })
                    })
                    .collect();
                members.sort_unstable();
                units.push(members.into_iter().map(|(_, op)| op).collect());
            }
        }
    }
    units
}

/// A small independent contract for the runtime's bank-unit emission order (#488).
///
/// The randomized interpreters below intentionally model the runtime schedule so they can
/// check larger dataflow properties. This fixture instead keeps the expected schedule as
/// handwritten literals and compares it directly with `runtime::units_of`, so a shared
/// scheduling mistake cannot make the assertion pass.
#[test]
fn runtime_units_match_literal_bank_schedules() {
    fn program_with_ops(count: usize) -> ExecutionProgram {
        let ops = (0..count)
            .map(|index| Op {
                // Deliberately nonmonotonic and unrelated to the op position.
                node: [41, 7, 93, 18, 66, 12, 87, 3][index],
                level: 0,
                inputs: (0, 0),
                sidechain: None,
                output: BufferRef(index as u32),
                in_place: false,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        ExecutionProgram {
            ops,
            inputs: Box::new([]),
            delays: Box::new([]),
            node_buffer: Box::new([]),
            node_op: Box::new([]),
            taps: Box::new([]),
            buffers: count as u32,
            output: BufferRef(0),
        }
    }

    let empty = program_with_ops(0);
    assert_eq!(
        crate::runtime::units_of(&empty, &crate::runtime::BankMembership::new()),
        Vec::<crate::runtime::PlannedUnit>::new()
    );

    let no_members = program_with_ops(8);
    assert_eq!(
        crate::runtime::units_of(&no_members, &crate::runtime::BankMembership::new()),
        (0..8).map(|index| (None, vec![index])).collect::<Vec<_>>()
    );

    let program = program_with_ops(8);
    let mut membership = crate::runtime::BankMembership::new();
    membership.insert(66, (crate::runtime::Membership::Effect(0), 0));
    membership.insert(7, (crate::runtime::Membership::Effect(0), 1));
    membership.insert(12, (crate::runtime::Membership::Builtin(0), 0));
    membership.insert(93, (crate::runtime::Membership::Builtin(0), 1));
    membership.insert(87, (crate::runtime::Membership::Effect(1), 0));

    let expected = vec![
        (None, vec![0]),
        (Some(crate::runtime::Membership::Effect(0)), vec![4, 1]),
        (Some(crate::runtime::Membership::Builtin(0)), vec![5, 2]),
        (None, vec![3]),
        (Some(crate::runtime::Membership::Effect(1)), vec![6]),
        (None, vec![7]),
    ];
    assert_eq!(crate::runtime::units_of(&program, &membership), expected);

    let effect_runtime_membership = membership
        .iter()
        .filter_map(|(node, (kind, lane))| match kind {
            crate::runtime::Membership::Effect(_) => Some((*node, (*kind, *lane))),
            crate::runtime::Membership::Builtin(_) => None,
        })
        .collect();
    let effect_membership = membership
        .iter()
        .filter_map(|(node, (kind, lane))| match kind {
            crate::runtime::Membership::Effect(bank) => Some((*node, (*bank, *lane))),
            crate::runtime::Membership::Builtin(_) => None,
        })
        .collect();
    let expected_effect_only = vec![
        vec![0],
        vec![4, 1],
        vec![2],
        vec![3],
        vec![5],
        vec![6],
        vec![7],
    ];
    let actual_effect_only = crate::runtime::units_of(&program, &effect_runtime_membership)
        .into_iter()
        .map(|(_, ops)| ops)
        .collect::<Vec<_>>();
    assert_eq!(actual_effect_only, expected_effect_only);
    assert_eq!(
        units_in_runtime_order(&program, &effect_membership),
        expected_effect_only
    );
}

/// `runtime::op_dataflow`, restated for the interpreter: readers of each op, and the op that
/// produced each op's first main input.
fn op_dataflow_model(program: &ExecutionProgram) -> (Vec<Vec<usize>>, Vec<Option<usize>>) {
    let mut owner: Vec<Option<usize>> = vec![None; program.buffers as usize];
    let mut readers: Vec<Vec<usize>> = vec![Vec::new(); program.ops.len()];
    let mut first: Vec<Option<usize>> = vec![None; program.ops.len()];
    for (index, op) in program.ops.iter().enumerate() {
        for (position, input) in program.inputs_of(op).iter().enumerate() {
            if let Some(producer) = owner[input.buffer.0 as usize] {
                readers[producer].push(index);
                if position == 0 {
                    first[index] = Some(producer);
                }
            }
        }
        if let Some(Some(producer)) = op.sidechain.map(|side| owner[side.buffer.0 as usize]) {
            readers[producer].push(index);
        }
        owner[op.output.0 as usize] = Some(index);
    }
    (readers, first)
}

/// `runtime::chains_into`, restated for the interpreter.
///
/// The observer clauses are the two this model omits, and it omits them on purpose: a program
/// -level fixture binds no observers at all, so both are vacuously satisfied. Omitting them
/// makes the model *more* permissive than the runtime, which is the safe direction for an
/// oracle -- it interprets at least every merge the runtime can take.
fn chains_into_model(
    program: &ExecutionProgram,
    readers: &[Vec<usize>],
    first_producer: &[Option<usize>],
    earlier: &[usize],
    later: &[usize],
) -> bool {
    if earlier.len() != later.len() || earlier.is_empty() {
        return false;
    }
    earlier.iter().zip(later.iter()).all(|(before, after)| {
        let producer = &program.ops[*before];
        let consumer = &program.ops[*after];
        consumer.input_count() == 1
            && consumer.sidechain.is_none()
            && program.inputs_of(consumer)[0].delay.is_none()
            && first_producer[*after] == Some(*before)
            && readers[*before].len() == 1
            && readers[*before][0] == *after
            && producer.output != program.output
    })
}

/// `runtime::cohort_runs`, restated for the interpreter: the units of
/// [`units_in_runtime_order`], grouped into the multi-slot chains the executor will build.
///
/// Each entry is one rendered unit as a slot-major list: `run[slot][lane]` is an op index.
fn runs_in_runtime_order(
    program: &ExecutionProgram,
    lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
) -> Vec<Vec<Vec<usize>>> {
    let units = units_in_runtime_order(program, lanes);
    let (readers, first_producer) = op_dataflow_model(program);
    let mut unit_of_op: Vec<Option<usize>> = vec![None; program.ops.len()];
    for (index, ops) in units.iter().enumerate() {
        if !lanes.contains_key(&program.ops[ops[0]].node) {
            continue;
        }
        for op in ops {
            unit_of_op[*op] = Some(index);
        }
    }
    let mut successor: std::collections::BTreeMap<usize, usize> = std::collections::BTreeMap::new();
    for (earlier, ops) in units.iter().enumerate() {
        if !lanes.contains_key(&program.ops[ops[0]].node) {
            continue;
        }
        let Some(later) = ops
            .first()
            .and_then(|lane| readers[*lane].first())
            .and_then(|reader| unit_of_op[*reader])
        else {
            continue;
        };
        if later != earlier
            && chains_into_model(program, &readers, &first_producer, ops, &units[later])
        {
            successor.insert(earlier, later);
        }
    }
    let merged: std::collections::BTreeSet<usize> = successor.values().copied().collect();
    let mut runs = Vec::with_capacity(units.len());
    for index in 0..units.len() {
        if merged.contains(&index) {
            continue;
        }
        let mut run = vec![units[index].clone()];
        let mut cursor = index;
        while let Some(next) = successor.get(&cursor) {
            run.push(units[*next].clone());
            cursor = *next;
        }
        runs.push(run);
    }
    runs
}

/// `runtime::bank_gather_source`, restated for the interpreter: the buffer a first-slot
/// member's gather reads **instead of** the member's own output, or `None` when it must run.
///
/// Modelling this is what closes the gap #194's verification flagged. A redirected member
/// never writes its own output buffer during the gather, so an interpreter that always wrote
/// it was describing a store the executor does not make -- and could therefore not tell that
/// buffer apart from one another member's gather still needs.
fn gather_source_model(program: &ExecutionProgram, op: &Op) -> Option<u32> {
    if op.sidechain.is_some() {
        return None;
    }
    match program.inputs_of(op) {
        [single] if single.delay.is_none() && single.buffer != op.output => Some(single.buffer.0),
        _ => None,
    }
}

/// `runtime::scatter_target`, restated for the interpreter.
///
/// The observer clauses are omitted for the reason [`chains_into_model`] omits them: a
/// program-level fixture binds none, so both are vacuously satisfied and omitting them makes
/// the model at least as permissive as the runtime.
fn scatter_target_model(
    program: &ExecutionProgram,
    lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
    readers: &[Vec<usize>],
    first_producer: &[Option<usize>],
    run: &[usize],
    first_op: usize,
    producer: usize,
) -> Option<usize> {
    if readers[producer].len() != 1 {
        return None;
    }
    let consumer = readers[producer][0];
    let producer_op = &program.ops[producer];
    let consumer_op = &program.ops[consumer];
    if consumer_op.input_count() != 1
        || consumer_op.sidechain.is_some()
        || program.inputs_of(consumer_op)[0].delay.is_some()
        || first_producer[consumer] != Some(producer)
        || consumer_op.output == producer_op.output
        || producer_op.output == program.output
        || lanes.contains_key(&consumer_op.node)
    {
        return None;
    }
    let target = consumer_op.output;
    let names = |op: &Op| {
        let hit = |input: &InputRef| {
            input.buffer == target || input.delay.is_some_and(|delay| delay.staging == target)
        };
        op.output == target
            || program.inputs_of(op).iter().any(hit)
            || op.sidechain.as_ref().is_some_and(hit)
    };
    for (index, op) in program.ops.iter().enumerate().take(consumer).skip(first_op) {
        if !run.contains(&index) && names(op) {
            return None;
        }
    }
    Some(consumer)
}

/// `runtime::scatter_redirects`, restated for the interpreter: last-slot op -> the consumer op
/// whose buffer the chain scatters into instead of its own.
fn redirects_in_runtime_order(
    program: &ExecutionProgram,
    lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
    runs: &[Vec<Vec<usize>>],
) -> std::collections::BTreeMap<usize, usize> {
    let (readers, first_producer) = op_dataflow_model(program);
    let mut redirects = std::collections::BTreeMap::new();
    for run in runs {
        let Some(first_op) = run.iter().flatten().min().copied() else {
            continue;
        };
        let flat: Vec<usize> = run.iter().flatten().copied().collect();
        let last = run.last().expect("a run has at least one slot");
        if last.len() < 2 && !lanes.contains_key(&program.ops[last[0]].node) {
            continue;
        }
        let lane_targets: Vec<Option<usize>> = last
            .iter()
            .map(|producer| {
                scatter_target_model(
                    program,
                    lanes,
                    &readers,
                    &first_producer,
                    &flat,
                    first_op,
                    *producer,
                )
            })
            .collect();
        let scattered: Vec<BufferRef> = last
            .iter()
            .zip(lane_targets.iter())
            .map(|(producer, target)| program.ops[target.unwrap_or(*producer)].output)
            .collect();
        let distinct: std::collections::BTreeSet<_> = scattered.iter().collect();
        if distinct.len() != scattered.len() {
            continue;
        }
        for (producer, target) in last.iter().zip(lane_targets) {
            if let Some(consumer) = target {
                redirects.insert(*producer, consumer);
            }
        }
    }
    redirects
}

/// Interpret the program the way the executor runs it -- banks hoisted -- and compare against
/// the semantic graph, node by node.
///
/// This is [`assert_program_matches_spec`]'s check moved from schedule order to *unit* order,
/// which is the order that actually renders. It models the two things schedule-order
/// interpretation cannot see: a bank gathers every member before its kernel runs, and a
/// delayed input is physically written into its staging slot.
fn divergence_in_runtime_order(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    delays: &[InsertedDelay],
    program: &ExecutionProgram,
    lanes: &std::collections::BTreeMap<u32, (usize, usize)>,
) -> Option<String> {
    let expected = evaluate_spec(spec, schedule, delays);
    let mut arena = vec![Expr::Silence; program.buffers as usize];
    // One op's reduction: stage every delayed input into its scratch, then combine.
    let gather = |arena: &mut Vec<Expr>, op: &Op| {
        let mut operands = Vec::new();
        for input in program.inputs_of(op) {
            let value = arena[input.buffer.0 as usize].clone();
            match input.delay {
                Some(delay) => {
                    let staged =
                        Expr::Delayed(Box::new(value), program.delays[delay.line as usize].samples);
                    arena[delay.staging.0 as usize] = staged.clone();
                    operands.push(staged);
                }
                None => operands.push(value),
            }
        }
        match operands.len() {
            0 => Expr::Silence,
            1 => operands.remove(0),
            _ => Expr::Sum(operands),
        }
    };
    let transform = |op: &Op, gathered: Expr| {
        if is_alias_candidate(&spec.nodes[op.node as usize].id) {
            gathered
        } else {
            Expr::Node(op.node, Box::new(gathered))
        }
    };
    let runs = runs_in_runtime_order(program, lanes);
    let redirect_of_lane = redirects_in_runtime_order(program, lanes, &runs);
    for run in runs {
        let first = &run[0];
        // A bank gathers *every* member before the kernel touches any of them, so a member's
        // storage must survive every other member's gather.
        let banked = first.len() > 1 || lanes.contains_key(&program.ops[first[0]].node);
        if !banked {
            let op = &program.ops[first[0]];
            // A redirected consumer does not reduce: the chain already scattered its input
            // into this very buffer, which is what `reduce_plane` does for a lone input that
            // is its own output.
            let gathered = if redirect_of_lane.values().any(|op| *op == first[0]) {
                arena[op.output.0 as usize].clone()
            } else {
                gather(&mut arena, op)
            };
            arena[op.output.0 as usize] = transform(op, gathered);
            if arena[op.output.0 as usize] != expected[op.node as usize] {
                return Some(format!(
                    "op {} (node {}) rendered a foreign value",
                    first[0], op.node
                ));
            }
            continue;
        }
        // Gather: the first slot's lanes, in lane order. A lane whose whole reduction is the
        // dedication copy is not executed at all -- the gather reads its producer's buffer and
        // its own output buffer is left alone (`runtime::bank_gather_source`).
        let mut resident: Vec<Expr> = Vec::with_capacity(first.len());
        for index in first {
            let op = &program.ops[*index];
            match gather_source_model(program, op) {
                Some(source) => resident.push(arena[source as usize].clone()),
                None => {
                    let gathered = gather(&mut arena, op);
                    arena[op.output.0 as usize] = gathered.clone();
                    resident.push(gathered);
                }
            }
        }
        // Every slot of the chain runs over the resident block, in cascade order. A later
        // slot's op is never executed: its value is passed between slots, not through the
        // arena, so its input buffer is never read and only the *last* slot's output buffer is
        // written by the scatter.
        for slot in &run {
            for (lane, index) in slot.iter().enumerate() {
                let op = &program.ops[*index];
                resident[lane] = transform(op, resident[lane].clone());
            }
        }
        let last = run.last().expect("a run has at least one slot");
        for (lane, index) in last.iter().enumerate() {
            let op = &program.ops[*index];
            if resident[lane] != expected[op.node as usize] {
                return Some(format!(
                    "op {index} (node {}) rendered a foreign value",
                    op.node
                ));
            }
            // The scatter lands in the consumer's buffer when this lane was redirected, and
            // the last slot's own buffer is then never written at all.
            let target = redirect_of_lane
                .get(index)
                .map_or(op.output, |consumer| program.ops[*consumer].output);
            arena[target.0 as usize] = resident[lane].clone();
        }
    }
    let output_node = spec
        .nodes
        .iter()
        .position(|node| matches!(node.id, GraphNodeId::Output { .. }))
        .expect("output");
    (arena[program.output.0 as usize] != expected[output_node])
        .then(|| "the session output rendered a foreign value".to_owned())
}

/// Buffers one op writes (its output, plus every staging slot it fills) and reads.
fn touched(program: &ExecutionProgram, op: &Op) -> (Vec<u32>, Vec<u32>) {
    let mut writes = vec![op.output.0];
    let mut reads = Vec::new();
    for input in program.inputs_of(op) {
        reads.push(input.buffer.0);
        if let Some(delay) = input.delay {
            writes.push(delay.staging.0);
        }
    }
    if let Some(side) = op.sidechain {
        reads.push(side.buffer.0);
        if let Some(delay) = side.delay {
            writes.push(delay.staging.0);
        }
    }
    (writes, reads)
}

/// The #169 invariant, checked structurally: **no physical slot is recycled inside a bank
/// window**.
///
/// A window is the op range from a bank's first member to its last. Execution over that range
/// is a permutation of the schedule -- members hoisted forward, non-members deferred -- so
/// colouring is sound there only if no op in the window writes storage another op in the
/// window reads or writes. An op writing its own input in place is the one exception, because
/// that is one op, not two.
fn assert_no_slot_is_recycled_inside_a_bank_window(
    program: &ExecutionProgram,
    spec: &GraphSpec,
    banks: &[Vec<GraphNodeId>],
    label: &str,
) {
    for members in banks {
        let positions: Vec<usize> = members
            .iter()
            .filter_map(|id| node_index(spec, id))
            .filter_map(|index| program.node_op[index as usize])
            .map(|op| op as usize)
            .collect();
        let (Some(first), Some(last)) = (positions.iter().min(), positions.iter().max()) else {
            continue;
        };
        for here in *first..=*last {
            let (writes, _) = touched(program, &program.ops[here]);
            for there in *first..=*last {
                if there == here {
                    continue;
                }
                let (other_writes, other_reads) = touched(program, &program.ops[there]);
                for slot in &writes {
                    assert!(
                        !other_writes.contains(slot),
                        "{label}: ops {here} and {there} both write buffer {slot} inside a \
                         bank window"
                    );
                    assert!(
                        !other_reads.contains(slot),
                        "{label}: op {here} writes buffer {slot}, which op {there} reads \
                         inside the same bank window"
                    );
                }
            }
        }
    }
}

/// Issue #169, the minimal reproduction: a slot released inside a bank window was handed
/// straight back to a member of that same bank.
///
/// Three tracks, one dynamic effect each, and a bank over the *outer* two -- the shape #166
/// made reachable, because a dynamic rack banks by cohort signature, so the tracks that share
/// a compressor bank need not be adjacent. Track `t01`'s effect is not in the bank and sits
/// between the two members in the schedule, so the executor defers it past the bank; its
/// delayed input needs a staging slot, and colouring released that slot one op before `t02`'s
/// member allocated its output.
///
/// In schedule order that is correct: the staging slot is dead the moment `t01`'s op finishes.
/// In *execution* order the bank runs first, `t02`'s member writes the slot, and then the
/// deferred `t01` stages over the top of it -- so `t02` renders `t01`'s delayed input.
///
/// The second half of the test is what keeps the first half honest: lowered with no bank
/// declared -- which is exactly what the pre-#169 signature could express -- the same graph
/// really does hand the slot over.
#[test]
fn a_bank_window_never_recycles_a_physical_slot() {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for (index, track) in ["t00", "t01", "t02"].iter().enumerate() {
        let (track_nodes, track_edges) = dynamic_track(track, &format!("r{index:02}"));
        nodes.extend(
            track_nodes.into_iter().filter(|candidate| {
                !matches!(candidate.id, GraphNodeId::Output { .. }) || index == 0
            }),
        );
        edges.extend(track_edges);
    }
    let (spec, schedule, levels) = build(nodes, edges);
    // The one PDC edge: into the effect of the track the bank does *not* contain.
    let delays = vec![InsertedDelay {
        node: dynamic_effect("t01"),
        edge_id: GraphEdgeId::TrackMain {
            target: dynamic_effect("t01"),
        },
        samples: LatencySamples(64),
    }];
    let banks = vec![vec![dynamic_effect("t00"), dynamic_effect("t02")]];
    let lanes = member_lanes(&spec, &banks);

    // Not vacuous: told nothing about the bank, colouring recycles the slot and the render
    // diverges. This is the pre-#169 behaviour, and the fixture exists to reach it.
    let unaware = lower(&spec, &schedule, &levels, &delays, &[]).expect("lowers");
    assert_program_matches_spec(&spec, &schedule, &delays, &unaware);
    let member = node_index(&spec, &dynamic_effect("t02")).expect("member");
    let outsider = node_index(&spec, &dynamic_effect("t01")).expect("outsider");
    let outsider_op = &unaware.ops[unaware.node_op[outsider as usize].expect("op") as usize];
    let staging = unaware.inputs_of(outsider_op)[0]
        .delay
        .expect("the outsider's input is delayed")
        .staging;
    assert_eq!(
        unaware.node_buffer[member as usize], staging,
        "the fixture must reach the collision it is here to pin"
    );
    assert!(
        divergence_in_runtime_order(&spec, &schedule, &delays, &unaware, &lanes).is_some(),
        "and that collision must actually change what renders"
    );

    // Told about the bank, the slot is held until the window closes.
    let program = lower(&spec, &schedule, &levels, &delays, &banks).expect("lowers");
    assert_program_matches_spec(&spec, &schedule, &delays, &program);
    assert_no_slot_is_recycled_inside_a_bank_window(&program, &spec, &banks, "minimal");
    assert_eq!(
        divergence_in_runtime_order(&spec, &schedule, &delays, &program, &lanes),
        None
    );
    // And it costs nothing here: holding a slot across the window changes *which* slot each
    // op gets, not how many exist. The window hold buys the collision away out of the
    // colouring's own slack -- which is why it, and not dedication, is the fix that shipped.
    assert_eq!((unaware.buffers, program.buffers), (7, 7));
}

/// Issue #169's structural half: **no physical slot is recycled inside a merged bank
/// window**, which is the whole soundness condition (see [`lower`]).
///
/// Four tracks and two interleaved cohorts -- the shape a dynamic rack produces when
/// neighbouring tracks carry different chains, which is what #166 made reachable. Each bank's
/// window contains the other's members, so the two merge into one span: what makes them
/// unsafe is not either bank alone but that each reorders the other's ops.
///
/// The overlap assertion is the non-vacuity guard. If the fixture ever stopped interleaving,
/// the windows would stop merging and this would pass on a graph that no longer poses the
/// question.
#[test]
fn no_slot_is_recycled_inside_a_merged_bank_window() {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for (index, track) in ["t00", "t01", "t02", "t03"].iter().enumerate() {
        let (track_nodes, track_edges) = dynamic_track(track, &format!("r{index:02}"));
        nodes.extend(
            track_nodes.into_iter().filter(|candidate| {
                !matches!(candidate.id, GraphNodeId::Output { .. }) || index == 0
            }),
        );
        edges.extend(track_edges);
    }
    let (spec, schedule, levels) = build(nodes, edges);
    let banks = vec![
        vec![dynamic_effect("t00"), dynamic_effect("t02")],
        vec![dynamic_effect("t01"), dynamic_effect("t03")],
    ];
    let program = lower(&spec, &schedule, &levels, &[], &banks).expect("lowers");
    assert_program_matches_spec(&spec, &schedule, &[], &program);

    // The two windows really do interleave, so `bank_windows` really does merge them.
    let window = |members: &[GraphNodeId]| {
        let ops: Vec<usize> = members
            .iter()
            .map(|id| {
                let index = node_index(&spec, id).expect("member is a node");
                program.node_op[index as usize].expect("a member is never elided") as usize
            })
            .collect();
        (
            *ops.iter().min().expect("first"),
            *ops.iter().max().expect("last"),
        )
    };
    let (first_start, first_end) = window(&banks[0]);
    let (second_start, second_end) = window(&banks[1]);
    assert!(
        first_start < second_start && second_start < first_end && first_end < second_end,
        "the cohorts must interleave: {first_start}..{first_end} and \
         {second_start}..{second_end}"
    );

    assert_no_slot_is_recycled_inside_a_bank_window(&program, &spec, &banks, "merged window");
    let lanes = member_lanes(&spec, &banks);
    assert_eq!(
        divergence_in_runtime_order(&spec, &schedule, &[], &program, &lanes),
        None
    );
}

/// Issue #169, the load-bearing eval: over seeded multi-track graphs with dynamic rack chains,
/// interleaved cohorts, fan-out and PDC, the program renders the value the semantic graph does
/// **in the order the executor actually runs it**, with banks hoisted to their first member.
///
/// `lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs` checks the same thing
/// in schedule order, which is not the order that renders once a bank exists. The `unaware`
/// arm counts how many of these graphs the pre-#169 lowering got wrong, so the corpus cannot
/// quietly stop constructing the hazard.
#[test]
fn bank_window_hoisting_preserves_dataflow_on_random_graphs() {
    let mut state = 0x1234_5678_9abc_def1_u64;
    let mut banked_graphs = 0usize;
    let mut unaware_divergences = 0usize;
    for graph in 0..4000_u32 {
        let track_count = (xorshift(&mut state) % 5) as usize + 2;
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut cohort_of_track: Vec<usize> = Vec::new();
        let mut route_index = 0usize;
        for track in 0..track_count {
            let name = format!("t{track:02}");
            let routes: Vec<String> = (0..(xorshift(&mut state) % 2) + 1)
                .map(|_| {
                    route_index += 1;
                    format!("r{route_index:02}")
                })
                .collect();
            let borrowed: Vec<&str> = routes.iter().map(String::as_str).collect();
            let (mut track_nodes, mut track_edges) = plain_track(&name, &borrowed);
            // A dynamic rack chain of one or two slots, and the cohort it belongs to.
            let slots = (xorshift(&mut state) % 2) as usize + 1;
            cohort_of_track.push((xorshift(&mut state) % 2) as usize);
            track_edges.retain(|edge| {
                edge.id
                    != GraphEdgeId::TrackMain {
                        target: stage_node(&name, TrackStage::PostDynamic),
                    }
            });
            let mut upstream = stage_node(&name, TrackStage::PostSimd1);
            for slot in 0..slots {
                let effect = GraphNodeId::Effect(crate::EffectNodeId {
                    track_id: gid(&name),
                    rack: RackId::Dynamic,
                    effect_id: gid(&format!("fx{slot}")),
                });
                track_nodes.push(node(effect.clone()));
                track_edges.push(main_edge(
                    GraphEdgeId::TrackMain {
                        target: effect.clone(),
                    },
                    upstream,
                    effect.clone(),
                ));
                upstream = effect;
            }
            track_edges.push(main_edge(
                GraphEdgeId::TrackMain {
                    target: stage_node(&name, TrackStage::PostDynamic),
                },
                upstream,
                stage_node(&name, TrackStage::PostDynamic),
            ));
            nodes.extend(track_nodes.into_iter().filter(|candidate| {
                !matches!(candidate.id, GraphNodeId::Output { .. }) || track == 0
            }));
            edges.extend(track_edges);
        }
        let (spec, schedule, levels) = build(nodes, edges);
        let mut delays: Vec<InsertedDelay> = Vec::new();
        for edge in &spec.edges {
            if !xorshift(&mut state).is_multiple_of(6) {
                continue;
            }
            delays.push(InsertedDelay {
                node: edge.destination.node.clone(),
                edge_id: edge.id.clone(),
                samples: LatencySamples(xorshift(&mut state) % 128 + 1),
            });
        }
        // Banks: dynamic effects, bucketed by (level, cohort), which is what the rack
        // compiler's cohort planner produces. Cohorts interleave by track, so one bank's
        // window contains the other's members.
        let mut buckets: std::collections::BTreeMap<(u64, usize), Vec<GraphNodeId>> =
            std::collections::BTreeMap::new();
        for level in &levels {
            for id in &level.nodes {
                let GraphNodeId::Effect(effect) = id else {
                    continue;
                };
                let track: usize = effect.track_id.as_str()[1..].parse().expect("track");
                buckets
                    .entry((level.level, cohort_of_track[track]))
                    .or_default()
                    .push(id.clone());
            }
        }
        let banks: Vec<Vec<GraphNodeId>> = buckets
            .into_values()
            .filter(|members| members.len() > 1)
            .collect();
        if banks.is_empty() {
            continue;
        }
        banked_graphs += 1;

        let program = lower(&spec, &schedule, &levels, &delays, &banks).expect("lowers");
        let lanes = member_lanes(&spec, &banks);
        assert_program_matches_spec(&spec, &schedule, &delays, &program);
        assert_no_slot_is_recycled_inside_a_bank_window(&program, &spec, &banks, "random");
        assert_eq!(
            divergence_in_runtime_order(&spec, &schedule, &delays, &program, &lanes),
            None,
            "graph {graph}"
        );

        let unaware = lower(&spec, &schedule, &levels, &delays, &[]).expect("lowers");
        if divergence_in_runtime_order(&spec, &schedule, &delays, &unaware, &lanes).is_some() {
            unaware_divergences += 1;
        }
    }
    // Both numbers are pinned, not bounded, because `lower`'s documentation quotes them: the
    // corpus is seeded and deterministic, so a change here means the corpus moved and the
    // claims that rest on it need re-measuring rather than re-pinning. The second number is
    // also the non-vacuity guard -- without it the arm above could pass on a corpus that no
    // longer constructs the hazard at all.
    assert_eq!(banked_graphs, 3617, "the banked corpus moved");
    assert_eq!(
        unaware_divergences, 285,
        "the number of graphs reaching the pre-#169 defect moved"
    );
}

/// Issue #202 rec 2, the load-bearing eval: over seeded multi-track graphs whose cohorts form
/// **multi-slot chains**, the program renders the value the semantic graph does in the order
/// the executor actually runs it -- chains merged, later slots never executed, and first-slot
/// gathers redirected past their dedication copy.
///
/// [`bank_window_hoisting_preserves_dataflow_on_random_graphs`] is the same eval for #169's
/// per-bank hoisting, and its corpus gives each *track* an independent slot count, so a
/// cohort's two levels rarely hold the same lane set and almost nothing merges. This corpus
/// fixes the slot count per **cohort**, which is what a rack chain actually looks like: every
/// lane of a cohort runs the same program, so slot `k` and slot `k + 1` cover the same lanes in
/// the same order and `runtime::chains_into` can prove the fusion.
///
/// Three arms, and the second and third are what make the first mean something:
///
/// * **Merged.** Lowered with the bank member lists production passes, so
///   [`chainable_bank_groups`] unions the fusible pairs and the window spans the whole chain.
///   No graph may diverge.
/// * **Narrow window.** The same lowering with [`chainable_bank_groups`] switched off
///   ([`lower_with_per_bank_windows`]), so every bank holds only its own span. That is
///   precisely the window set a merge-unaware lowering would hold, and the count of graphs it
///   gets wrong is the measurement of what the union buys.
/// * **No window at all.** The pre-#169 arm, kept so the corpus cannot quietly stop
///   constructing a hazard at all.
///
/// Every count is pinned rather than bounded, for the reason the #169 eval gives: the corpus is
/// seeded and deterministic, so a change means the corpus moved and the claims resting on it
/// need re-measuring rather than re-pinning.
#[test]
fn cohort_chain_merging_preserves_dataflow_on_random_graphs() {
    let mut state = 0x0fed_cba9_8765_4321_u64;
    let mut chained_graphs = 0usize;
    let mut merged_runs = 0usize;
    let mut redirected_lanes = 0usize;
    let mut narrow_divergences = 0usize;
    let mut unaware_divergences = 0usize;
    for graph in 0..4000_u32 {
        let track_count = (xorshift(&mut state) % 11) as usize + 2;
        // Four cohorts, each with its own chain length: every lane of a cohort runs the same
        // program, which is what makes slot `k` and slot `k + 1` the same lane set. Four
        // rather than two because a merged run hoists its whole chain past *every* other
        // cohort's ops at the same level, so the pressure on the colouring grows with the
        // cohort count -- the standing console fixture runs eight.
        let cohort_slots = [
            (xorshift(&mut state) % 3) as usize + 1,
            (xorshift(&mut state) % 3) as usize + 1,
            (xorshift(&mut state) % 3) as usize + 1,
            (xorshift(&mut state) % 3) as usize + 1,
        ];
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut cohort_of_track: Vec<usize> = Vec::new();
        let mut route_index = 0usize;
        for track in 0..track_count {
            let name = format!("t{track:02}");
            let routes: Vec<String> = (0..(xorshift(&mut state) % 2) + 1)
                .map(|_| {
                    route_index += 1;
                    format!("r{route_index:02}")
                })
                .collect();
            let borrowed: Vec<&str> = routes.iter().map(String::as_str).collect();
            let (mut track_nodes, mut track_edges) = plain_track(&name, &borrowed);
            let cohort = (xorshift(&mut state) % 4) as usize;
            cohort_of_track.push(cohort);
            track_edges.retain(|edge| {
                edge.id
                    != GraphEdgeId::TrackMain {
                        target: stage_node(&name, TrackStage::PostDynamic),
                    }
            });
            let mut upstream = stage_node(&name, TrackStage::PostSimd1);
            // One cohort's members are dedicated storage and one's are not, which is the
            // difference that decides whether a member folds into its producer in place.
            // A member that folds allocates nothing, so a corpus of only those never reaches
            // the recycling hazard a merged window exists to hold off.
            let rack = if cohort.is_multiple_of(2) {
                RackId::Simd1
            } else {
                RackId::Dynamic
            };
            for slot in 0..cohort_slots[cohort] {
                let effect = GraphNodeId::Effect(crate::EffectNodeId {
                    track_id: gid(&name),
                    rack,
                    effect_id: gid(&format!("fx{slot}")),
                });
                track_nodes.push(node(effect.clone()));
                track_edges.push(main_edge(
                    GraphEdgeId::TrackMain {
                        target: effect.clone(),
                    },
                    upstream,
                    effect.clone(),
                ));
                upstream = effect;
            }
            track_edges.push(main_edge(
                GraphEdgeId::TrackMain {
                    target: stage_node(&name, TrackStage::PostDynamic),
                },
                upstream,
                stage_node(&name, TrackStage::PostDynamic),
            ));
            nodes.extend(track_nodes.into_iter().filter(|candidate| {
                !matches!(candidate.id, GraphNodeId::Output { .. }) || track == 0
            }));
            edges.extend(track_edges);
        }
        let (spec, schedule, levels) = build(nodes, edges);
        let mut delays: Vec<InsertedDelay> = Vec::new();
        for edge in &spec.edges {
            if !xorshift(&mut state).is_multiple_of(6) {
                continue;
            }
            delays.push(InsertedDelay {
                node: edge.destination.node.clone(),
                edge_id: edge.id.clone(),
                samples: LatencySamples(xorshift(&mut state) % 128 + 1),
            });
        }
        let mut buckets: std::collections::BTreeMap<(u64, usize), Vec<GraphNodeId>> =
            std::collections::BTreeMap::new();
        for level in &levels {
            for id in &level.nodes {
                let GraphNodeId::Effect(effect) = id else {
                    continue;
                };
                let track: usize = effect.track_id.as_str()[1..].parse().expect("track");
                buckets
                    .entry((level.level, cohort_of_track[track]))
                    .or_default()
                    .push(id.clone());
            }
        }
        let banks: Vec<Vec<GraphNodeId>> = buckets
            .into_values()
            .filter(|members| members.len() > 1)
            .collect();
        if banks.is_empty() {
            continue;
        }
        chained_graphs += 1;

        let program = lower(&spec, &schedule, &levels, &delays, &banks).expect("lowers");
        let lanes = member_lanes(&spec, &banks);
        assert_program_matches_spec(&spec, &schedule, &delays, &program);
        assert_eq!(
            divergence_in_runtime_order(&spec, &schedule, &delays, &program, &lanes),
            None,
            "graph {graph}"
        );
        let realised = runs_in_runtime_order(&program, &lanes);
        merged_runs += realised.iter().filter(|run| run.len() > 1).count();
        let modelled = redirects_in_runtime_order(&program, &lanes, &realised);
        redirected_lanes += modelled.len();
        // The model is an oracle only while it and the runtime agree, and the model cannot
        // check that by itself. Issue #202's adversarial verification found the gap: shortening
        // `runtime::scatter_target`'s in-between scan by one op -- the unsound direction --
        // reddened nothing, while the same one-token change to `scatter_target_model` reddened
        // this corpus at graph 0. The corpus was building the hazard and then only ever asking
        // the model about it. So the corpus now drives the runtime's own clauses too, and every
        // clause it exercises has this eval as its red test on both sides of the pair.
        assert_eq!(
            crate::runtime::scatter_redirects_over_program(&program, &spec, &lanes, &realised),
            modelled,
            "graph {graph}: the runtime and the model disagree about which lanes redirect"
        );

        // The narrow-window arm: every bank's own span, and no union across banks.
        let narrow = lower_with_per_bank_windows(&spec, &schedule, &levels, &delays, &banks)
            .expect("lowers");
        if divergence_in_runtime_order(&spec, &schedule, &delays, &narrow, &lanes).is_some() {
            narrow_divergences += 1;
        }

        let unaware = lower(&spec, &schedule, &levels, &delays, &[]).expect("lowers");
        if divergence_in_runtime_order(&spec, &schedule, &delays, &unaware, &lanes).is_some() {
            unaware_divergences += 1;
        }
    }
    assert_eq!(chained_graphs, 3563, "the chained corpus moved");
    assert_eq!(
        merged_runs, 3752,
        "the number of realised multi-slot chains moved: if this ever falls to zero the arm \
         above is passing on a corpus where nothing merges"
    );
    assert_eq!(
        redirected_lanes, 1669,
        "the number of lanes whose scatter is redirected into their consumer moved (#202 \
         rec 3): a zero here would mean this eval never interprets one"
    );
    assert_eq!(
        narrow_divergences, 885,
        "the number of graphs a per-bank window gets wrong moved -- this is the measurement \
         of what the cohort-chain window union buys, and a zero here would mean the union is \
         held for nothing"
    );
    assert_eq!(
        unaware_divergences, 1665,
        "the number of graphs reaching the pre-#169 defect moved"
    );
}
