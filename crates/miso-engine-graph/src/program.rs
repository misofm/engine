//! The executable form of a prepared graph: an [`ExecutionProgram`].
//!
//! `GraphSpec` is a *semantic* graph. It materialises seven `TrackStage` nodes per track and one
//! edge per hop because that is the vocabulary sends, taps, PDC and diagnostics are expressed in.
//! It is not a schedule. Executed literally -- one schedule item per node, one contribution buffer
//! per edge, one pairwise reduction per node -- a track with no effects costs seven node visits
//! and six full buffer copies per block to move its input to its output unchanged (#99 F2).
//!
//! Lowering keeps the semantic graph exactly as it is and *derives* an executable program from it:
//!
//! * node ids are interned to [`NodeIndex`], so nothing on the render path compares strings;
//! * a `TrackStage` boundary that is a pure pass-through becomes a **buffer alias** ([`Tap`]) with
//!   no schedule item at all -- observers still attach to it, immediately after the op that last
//!   wrote the buffer;
//! * a consumer with exactly one undelayed input reads its producer's buffer **in place**, so the
//!   copy disappears;
//! * a `Sum` -- the per-frame `balanced_pairwise_sum` -- exists only where fan-in is genuinely
//!   greater than one;
//! * buffers are liveness-coloured over *ops*, so the arena is proportional to the graph's live
//!   width rather than to its edge count.
//!
//! [`lower`] is a pure function of `(spec, schedule, levels, delays)`: the program cannot disagree
//! with the semantic graph, because it is computed from it. Everything is derived from sorted
//! inputs and is deterministic; nothing here reads a clock, a CPU or an environment.
//!
//! ## What lowering must not change
//!
//! Reduction shape is frozen by master plan #83 D9: pairwise, left-to-right, in stable edge-ID
//! order, the same tree in both executors. Lowering preserves it exactly -- `Op::inputs` is a
//! half-open range into [`ExecutionProgram::inputs`] filled in `spec.edges` order, which is sorted
//! by `GraphEdgeId`. A single-input "reduction" was already the identity (`values[0]`), so folding
//! it into an in-place read is bit-preserving by construction, not by tolerance.

use crate::{
    DependencyLevel, GraphEdgeId, GraphNodeId, GraphPortKind, GraphSpec, InsertedDelay, RackId,
    TrackStage,
};

/// Position of a node in `GraphSpec::nodes`, which is sorted by `GraphNodeId`.
pub type NodeIndex = u32;
/// Position of an op in [`ExecutionProgram::ops`].
pub type OpIndex = u32;

/// A physical audio buffer in the executor's arena.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BufferRef(pub u32);

/// A compensation delay applied to one input on the way into its consumer.
///
/// `line` indexes [`ExecutionProgram::delays`]; `staging` is the buffer the delayed copy lands in,
/// which is live only for the duration of the op that reads it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DelayRef {
    pub line: u32,
    pub staging: BufferRef,
}

/// One input of an op: where to read it, and whether it is delayed on the way in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputRef {
    pub buffer: BufferRef,
    pub delay: Option<DelayRef>,
}

/// A compensation delay line, in samples.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DelaySpec {
    pub samples: u64,
}

/// One executable step.
///
/// `inputs` is a half-open range into [`ExecutionProgram::inputs`], in stable edge-ID order (D9).
/// `in_place` means the op's single input already lives in `output` and must not be copied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Op {
    pub node: NodeIndex,
    pub level: u64,
    pub inputs: (u32, u32),
    pub sidechain: Option<InputRef>,
    pub output: BufferRef,
    pub in_place: bool,
}

impl Op {
    /// Number of main inputs. A `Sum` is needed only when this is greater than one.
    #[must_use]
    pub const fn input_count(&self) -> u32 {
        self.inputs.1 - self.inputs.0
    }
}

/// An elided node: it has no op, and its output is an alias of `buffer`.
///
/// `after_op` is the op that last wrote `buffer` before any consumer reads it. An observer bound
/// to an elided node fires there -- immediately after that op's own observers and before the next
/// op runs. Attaching it to the *consumer* instead would let it observe mutated data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tap {
    pub node: NodeIndex,
    pub buffer: BufferRef,
    pub after_op: OpIndex,
}

/// The executable form of a prepared graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionProgram {
    /// Level-major, node-id-sorted within a level: the sequential schedule minus elided nodes.
    pub ops: Box<[Op]>,
    pub inputs: Box<[InputRef]>,
    pub delays: Box<[DelaySpec]>,
    /// `NodeIndex` -> the buffer carrying that node's output.
    pub node_buffer: Box<[BufferRef]>,
    /// `NodeIndex` -> its op, or `None` when the node was elided into an alias.
    pub node_op: Box<[Option<OpIndex>]>,
    /// One entry per elided node.
    pub taps: Box<[Tap]>,
    /// Size of the coloured arena.
    pub buffers: u32,
    /// Buffer carrying the sole session output.
    pub output: BufferRef,
}

impl ExecutionProgram {
    /// Main inputs of one op, in stable edge-ID order.
    #[must_use]
    pub fn inputs_of(&self, op: &Op) -> &[InputRef] {
        &self.inputs[op.inputs.0 as usize..op.inputs.1 as usize]
    }
    /// Ops that still need a per-frame pairwise reduction: fan-in greater than one.
    #[must_use]
    pub fn reduction_count(&self) -> usize {
        self.ops.iter().filter(|op| op.input_count() > 1).count()
    }
    /// Inputs that carry a compensation delay, and therefore a staging buffer.
    #[must_use]
    pub fn delayed_input_count(&self) -> usize {
        self.inputs
            .iter()
            .filter(|input| input.delay.is_some())
            .count()
            + self
                .ops
                .iter()
                .filter(|op| op.sidechain.is_some_and(|side| side.delay.is_some()))
                .count()
    }
}

/// Why a spec could not be lowered. Every variant is an internal invariant violation: the compiler
/// produced these inputs itself, so a caller maps them to `graph.internal.invariant`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProgramError {
    /// An id in the schedule, the levels or an edge endpoint is not in `spec.nodes`.
    UnknownNode,
    /// `spec.nodes` is not sorted by id, so interning by binary search is not valid.
    SpecUnsorted,
    /// The schedule is not the concatenation of the levels, or is not a permutation of the nodes.
    ScheduleMismatch,
    /// An edge runs backwards in the schedule.
    NotTopological,
    /// A count did not fit its index type.
    Overflow,
}

/// Intern one node id. `spec.nodes` is sorted by id, so this is a binary search.
///
/// # Errors
/// Returns `None` when the id is not a node of this spec.
#[must_use]
pub fn node_index(spec: &GraphSpec, id: &GraphNodeId) -> Option<NodeIndex> {
    spec.nodes
        .binary_search_by(|node| node.id.cmp(id))
        .ok()
        .and_then(|index| u32::try_from(index).ok())
}

/// A stage boundary that carries signal without ever changing it.
///
/// These three are the *internal* rack boundaries. They are never effects, never bank members and
/// never appear in `required_bindings`, so nothing can bind a processor to them; their only role
/// is to be a stable observation and send-tap point. The other four stages
/// (`Input`, `PostInputBuiltins`, `PostFader`, `PostMatrix`) are all bindable and keep their ops.
const fn is_alias_candidate(node: &GraphNodeId) -> bool {
    matches!(
        node,
        GraphNodeId::TrackStage {
            stage: TrackStage::PostSimd1 | TrackStage::PostDynamic | TrackStage::PostSimd2PreFader,
            ..
        }
    )
}

/// A node whose output buffer may never be shared with anything else.
///
/// A homogeneous bank gathers *all* of its members' outputs, runs, and scatters them back, so
/// every member's output is live from the first member's gather to the last member's scatter --
/// across every op scheduled between them. Colouring must therefore never hand a bank member's
/// storage to an op in that window, and no op may consume a member's buffer in place. The
/// bank-eligible nodes are the SIMD-rack effects and the post-input builtin stage; this is the
/// same rule `GraphExecutor::new` previously hard-coded by re-buffering members after colouring.
const fn is_dedicated(node: &GraphNodeId) -> bool {
    match node {
        GraphNodeId::Effect(id) => !matches!(id.rack, RackId::Dynamic),
        GraphNodeId::TrackStage { stage, .. } => matches!(stage, TrackStage::PostInputBuiltins),
        _ => false,
    }
}

/// One logical buffer's lifetime, in op indices.
struct Lifetime {
    def_op: usize,
    last_use: usize,
    dedicated: bool,
}

/// Lower a semantic graph into an executable program.
///
/// `schedule` must be the concatenation of `levels`, `spec.nodes` must be sorted by id, and every
/// edge must run forwards in the schedule -- all three are properties the graph compiler
/// establishes before it calls this.
///
/// # Errors
/// See [`ProgramError`]; every variant means the caller's own invariants were violated.
#[allow(clippy::too_many_lines)]
pub fn lower(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    delays: &[InsertedDelay],
) -> Result<ExecutionProgram, ProgramError> {
    if spec.nodes.windows(2).any(|pair| pair[0].id >= pair[1].id) {
        return Err(ProgramError::SpecUnsorted);
    }
    let node_count = spec.nodes.len();
    if schedule.len() != node_count {
        return Err(ProgramError::ScheduleMismatch);
    }
    // The schedule is the concatenation of the levels, and each node carries its level.
    let mut level_of = vec![None; node_count];
    let mut cursor = 0usize;
    for level in levels {
        for id in &level.nodes {
            if schedule.get(cursor) != Some(id) {
                return Err(ProgramError::ScheduleMismatch);
            }
            let index = node_index(spec, id).ok_or(ProgramError::UnknownNode)? as usize;
            if level_of[index].is_some() {
                return Err(ProgramError::ScheduleMismatch);
            }
            level_of[index] = Some(level.level);
            cursor += 1;
        }
    }
    if cursor != node_count {
        return Err(ProgramError::ScheduleMismatch);
    }

    // Schedule position per node, and the interned schedule.
    let mut position = vec![usize::MAX; node_count];
    let mut order = Vec::with_capacity(node_count);
    for (slot, id) in schedule.iter().enumerate() {
        let index = node_index(spec, id).ok_or(ProgramError::UnknownNode)? as usize;
        position[index] = slot;
        order.push(index);
    }

    let delay_of: std::collections::BTreeMap<&GraphEdgeId, u64> = delays
        .iter()
        .map(|delay| (&delay.edge_id, delay.samples.0))
        .collect();

    // Inputs per node, in `spec.edges` order -- which is sorted by `GraphEdgeId` (D9).
    let mut main_in: Vec<Vec<usize>> = vec![Vec::new(); node_count];
    let mut side_in: Vec<Option<usize>> = vec![None; node_count];
    for (edge_index, edge) in spec.edges.iter().enumerate() {
        let source = node_index(spec, &edge.source.node).ok_or(ProgramError::UnknownNode)? as usize;
        let destination =
            node_index(spec, &edge.destination.node).ok_or(ProgramError::UnknownNode)? as usize;
        if position[source] >= position[destination] {
            return Err(ProgramError::NotTopological);
        }
        match edge.destination.kind {
            GraphPortKind::MainInput => main_in[destination].push(edge_index),
            GraphPortKind::SidechainInput => side_in[destination] = Some(edge_index),
            GraphPortKind::MainOutput => return Err(ProgramError::NotTopological),
        }
    }

    let edge_delay = |edge_index: usize| {
        delay_of
            .get(&spec.edges[edge_index].id)
            .copied()
            .filter(|s| *s != 0)
    };

    // A stage boundary is elided when it is a pure alias: one main input, no sidechain, no PDC.
    let elided: Vec<bool> = (0..node_count)
        .map(|index| {
            is_alias_candidate(&spec.nodes[index].id)
                && main_in[index].len() == 1
                && side_in[index].is_none()
                && edge_delay(main_in[index][0]).is_none()
        })
        .collect();

    // A read is an edge whose *destination* is not elided: an edge into an elided node is an
    // alias, not a consumption, and a tap is not a reader either (it fires immediately after the
    // producing op, before any consumer). Counting either would block every in-place op in a
    // chain.
    let mut reads_of = vec![0usize; node_count];
    for edge in &spec.edges {
        let destination =
            node_index(spec, &edge.destination.node).ok_or(ProgramError::UnknownNode)? as usize;
        if elided[destination] {
            continue;
        }
        let mut source =
            node_index(spec, &edge.source.node).ok_or(ProgramError::UnknownNode)? as usize;
        // Resolve through the alias chain to the node that actually produces the storage.
        while elided[source] {
            source = node_index(spec, &spec.edges[main_in[source][0]].source.node)
                .ok_or(ProgramError::UnknownNode)? as usize;
        }
        reads_of[source] += 1;
    }

    // ---- pass 1: ops, logical buffers, aliases ---------------------------------------------
    let mut ops: Vec<Op> = Vec::with_capacity(node_count);
    let mut inputs: Vec<InputRef> = Vec::new();
    let mut delay_specs: Vec<DelaySpec> = Vec::new();
    let mut taps: Vec<Tap> = Vec::new();
    let mut node_op: Vec<Option<OpIndex>> = vec![None; node_count];
    let mut logical_of_node: Vec<Option<u32>> = vec![None; node_count];
    let mut after_op_of_node: Vec<Option<OpIndex>> = vec![None; node_count];
    let mut lifetimes: Vec<Lifetime> = Vec::new();
    // Which node currently owns a logical buffer, so an in-place chain keeps one reader count.
    let mut owner_of_logical: Vec<usize> = Vec::new();

    for &index in &order {
        let id = &spec.nodes[index].id;
        if elided[index] {
            let producer = node_index(spec, &spec.edges[main_in[index][0]].source.node)
                .ok_or(ProgramError::UnknownNode)? as usize;
            let buffer = logical_of_node[producer].ok_or(ProgramError::NotTopological)?;
            let after = after_op_of_node[producer].ok_or(ProgramError::NotTopological)?;
            logical_of_node[index] = Some(buffer);
            after_op_of_node[index] = Some(after);
            taps.push(Tap {
                node: u32::try_from(index).map_err(|_| ProgramError::Overflow)?,
                buffer: BufferRef(buffer),
                after_op: after,
            });
            continue;
        }

        let op_index = u32::try_from(ops.len()).map_err(|_| ProgramError::Overflow)?;
        let first_input = u32::try_from(inputs.len()).map_err(|_| ProgramError::Overflow)?;
        for &edge_index in &main_in[index] {
            let source = node_index(spec, &spec.edges[edge_index].source.node)
                .ok_or(ProgramError::UnknownNode)? as usize;
            let buffer = logical_of_node[source].ok_or(ProgramError::NotTopological)?;
            let delay = edge_delay(edge_index).map(|samples| {
                delay_specs.push(DelaySpec { samples });
                (delay_specs.len() - 1) as u32
            });
            inputs.push(InputRef {
                buffer: BufferRef(buffer),
                // The staging buffer is filled in by pass 2, which knows the physical arena.
                delay: delay.map(|line| DelayRef {
                    line,
                    staging: BufferRef(u32::MAX),
                }),
            });
            lifetimes[buffer as usize].last_use = ops.len();
        }
        let last_input = u32::try_from(inputs.len()).map_err(|_| ProgramError::Overflow)?;
        let sidechain = match side_in[index] {
            None => None,
            Some(edge_index) => {
                let source = node_index(spec, &spec.edges[edge_index].source.node)
                    .ok_or(ProgramError::UnknownNode)? as usize;
                let buffer = logical_of_node[source].ok_or(ProgramError::NotTopological)?;
                let delay = edge_delay(edge_index).map(|samples| {
                    delay_specs.push(DelaySpec { samples });
                    (delay_specs.len() - 1) as u32
                });
                lifetimes[buffer as usize].last_use = ops.len();
                Some(InputRef {
                    buffer: BufferRef(buffer),
                    delay: delay.map(|line| DelayRef {
                        line,
                        staging: BufferRef(u32::MAX),
                    }),
                })
            }
        };
        let dedicated = is_dedicated(id);
        // In place iff this op is the *only* reader of a single undelayed input, and neither end
        // of the aliasing is a bank member.
        let single = (last_input - first_input == 1)
            && sidechain.is_none()
            && inputs[first_input as usize].delay.is_none();
        let in_place = single && !dedicated && {
            let buffer = inputs[first_input as usize].buffer.0 as usize;
            let owner = owner_of_logical[buffer];
            reads_of[owner] == 1 && !lifetimes[buffer].dedicated
        };
        let output = if in_place {
            let buffer = inputs[first_input as usize].buffer.0;
            owner_of_logical[buffer as usize] = index;
            buffer
        } else {
            let buffer = u32::try_from(lifetimes.len()).map_err(|_| ProgramError::Overflow)?;
            lifetimes.push(Lifetime {
                def_op: ops.len(),
                last_use: ops.len(),
                dedicated,
            });
            owner_of_logical.push(index);
            buffer
        };
        lifetimes[output as usize].last_use = lifetimes[output as usize].last_use.max(ops.len());

        ops.push(Op {
            node: u32::try_from(index).map_err(|_| ProgramError::Overflow)?,
            level: level_of[index].ok_or(ProgramError::ScheduleMismatch)?,
            inputs: (first_input, last_input),
            sidechain,
            output: BufferRef(output),
            in_place,
        });
        node_op[index] = Some(op_index);
        logical_of_node[index] = Some(output);
        after_op_of_node[index] = Some(op_index);
    }

    // The sole session output survives the last op: the executor copies it out afterwards.
    let output_node = spec
        .nodes
        .iter()
        .position(|node| matches!(node.id, GraphNodeId::Output { .. }))
        .ok_or(ProgramError::UnknownNode)?;
    let output_logical = logical_of_node[output_node].ok_or(ProgramError::UnknownNode)?;
    lifetimes[output_logical as usize].last_use = ops.len();

    // ---- pass 2: liveness colouring over ops -------------------------------------------------
    let mut expire: Vec<Vec<u32>> = vec![Vec::new(); ops.len() + 1];
    for (buffer, life) in lifetimes.iter().enumerate() {
        expire[life.last_use].push(u32::try_from(buffer).map_err(|_| ProgramError::Overflow)?);
    }
    let mut physical = vec![u32::MAX; lifetimes.len()];
    let mut free: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
    let mut next_physical = 0u32;
    let mut staging_release: Vec<u32> = Vec::new();
    let take = |free: &mut std::collections::BTreeSet<u32>, next: &mut u32| -> u32 {
        if let Some(buffer) = free.pop_first() {
            buffer
        } else {
            let buffer = *next;
            *next += 1;
            buffer
        }
    };
    for op_index in 0..ops.len() {
        if op_index > 0 {
            for buffer in expire[op_index - 1].drain(..) {
                // A dedicated buffer is never returned: a bank keeps every member's output live
                // across the whole gather/process/scatter window.
                if !lifetimes[buffer as usize].dedicated {
                    free.insert(physical[buffer as usize]);
                }
            }
        }
        for buffer in staging_release.drain(..) {
            free.insert(buffer);
        }
        // A delayed input stages into a scratch buffer that lives only for this op.
        let (first, last) = ops[op_index].inputs;
        for input in &mut inputs[first as usize..last as usize] {
            if let Some(delay) = &mut input.delay {
                let buffer = take(&mut free, &mut next_physical);
                delay.staging = BufferRef(buffer);
                staging_release.push(buffer);
            }
        }
        if let Some(delay) = ops[op_index]
            .sidechain
            .as_mut()
            .and_then(|side| side.delay.as_mut())
        {
            let buffer = take(&mut free, &mut next_physical);
            delay.staging = BufferRef(buffer);
            staging_release.push(buffer);
        }
        let output = ops[op_index].output.0;
        if lifetimes[output as usize].def_op == op_index && physical[output as usize] == u32::MAX {
            physical[output as usize] = take(&mut free, &mut next_physical);
        }
        ops[op_index].output = BufferRef(physical[output as usize]);
    }
    // Rewrite every logical reference to its physical buffer.
    for input in &mut inputs {
        input.buffer = BufferRef(physical[input.buffer.0 as usize]);
    }
    for op in &mut ops {
        if let Some(side) = &mut op.sidechain {
            side.buffer = BufferRef(physical[side.buffer.0 as usize]);
        }
    }
    let node_buffer: Vec<BufferRef> = (0..node_count)
        .map(|index| {
            logical_of_node[index]
                .map(|buffer| BufferRef(physical[buffer as usize]))
                .ok_or(ProgramError::UnknownNode)
        })
        .collect::<Result<_, _>>()?;
    for tap in &mut taps {
        tap.buffer = node_buffer[tap.node as usize];
    }

    let output = node_buffer[output_node];
    Ok(ExecutionProgram {
        ops: ops.into_boxed_slice(),
        inputs: inputs.into_boxed_slice(),
        delays: delay_specs.into_boxed_slice(),
        node_buffer: node_buffer.into_boxed_slice(),
        node_op: node_op.into_boxed_slice(),
        taps: taps.into_boxed_slice(),
        buffers: next_physical,
        output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GraphEdge, GraphNode, GraphPortId, StableGraphId};
    use miso_engine_effect_contract::{LatencySamples, TailSamples};

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
        let program = lower(&spec, &schedule, &levels, &[]).expect("lowers");

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
        let program = lower(&spec, &schedule, &levels, &[]).expect("lowers");

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
        let program =
            lower(&spec, &schedule, &levels, std::slice::from_ref(&delayed)).expect("lowers");

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
        let program =
            lower(&spec, &schedule, &levels, std::slice::from_ref(&delayed)).expect("lowers");
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
        let program = lower(&spec, &schedule, &levels, &[]).expect("lowers");

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
        let fader_index =
            node_index(&spec, &stage_node(track, TrackStage::PostFader)).expect("fader");
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

    /// The three rejections a caller maps to `graph.internal.invariant`.
    #[test]
    fn malformed_inputs_are_rejected_rather_than_lowered() {
        let (nodes, edges) = plain_track("t", &["r"]);
        let (spec, schedule, levels) = build(nodes, edges);
        assert_eq!(
            lower(&spec, &schedule[..schedule.len() - 1], &levels, &[]),
            Err(ProgramError::ScheduleMismatch)
        );
        let mut swapped = schedule.clone();
        swapped.swap(0, 1);
        assert_eq!(
            lower(&spec, &swapped, &levels, &[]),
            Err(ProgramError::ScheduleMismatch)
        );
        let mut unsorted = spec.clone();
        unsorted.nodes.swap(0, 1);
        assert_eq!(
            lower(&unsorted, &schedule, &levels, &[]),
            Err(ProgramError::SpecUnsorted)
        );
    }
}
