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

use std::collections::BTreeMap;

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

/// A node whose output buffer is never returned to the free list.
///
/// The SIMD-rack effects and the post-input builtin stage, unchanged since #99: the same rule
/// `GraphExecutor::new` once hard-coded by re-buffering members after colouring. It is a
/// conservative classification by node *kind*, and it costs whatever it costs -- a dedicated
/// buffer cannot be consumed in place, so its consumer pays a copy.
///
/// It is **not** what makes a homogeneous bank safe, despite predating banks and looking like it
/// should be. A bank's hazard is a *window*, not a node; the window is handled by
/// [`bank_windows`], and [`lower`] records why extending this predicate to bank members was
/// measured and rejected (issue #169).
const fn is_dedicated(node: &GraphNodeId) -> bool {
    match node {
        GraphNodeId::Effect(id) => !matches!(id.rack, RackId::Dynamic),
        GraphNodeId::TrackStage { stage, .. } => matches!(stage, TrackStage::PostInputBuiltins),
        _ => false,
    }
}

/// The node whose storage a node's first main input reads, resolved through elided aliases.
///
/// This is the lowered program's `first_producer` stated on the semantic graph: the buffer an op's
/// first main input names is written by the op of the node this returns, because an elided stage
/// boundary owns no storage of its own and an in-place op keeps its producer's colour under its
/// own ownership. `runtime::op_dataflow` derives the same edge from the colouring; the two have to
/// agree, and `chainable_bank_groups` is the only place the lowering needs the answer before the
/// colouring exists.
fn first_main_producer(
    spec: &GraphSpec,
    elided: &[bool],
    main_in: &[Vec<usize>],
    node: usize,
) -> Option<usize> {
    let edge = *main_in.get(node)?.first()?;
    let mut source = node_index(spec, &spec.edges[edge].source.node)? as usize;
    while elided[source] {
        source = node_index(spec, &spec.edges[*main_in[source].first()?].source.node)? as usize;
    }
    Some(source)
}

/// Bank member lists with every pair of banks a cohort chain may fuse unioned into one entry.
///
/// A merged run renders as **one unit at its first slot's op position** (`runtime::cohort_runs`),
/// so the op range the schedule is permuted over is the union of every slot's ops -- not each
/// slot's own range. `bank_windows` needs that union, and this is where it is formed.
///
/// The pairing condition is exactly `runtime::chains_into`'s first clause, restated on the
/// semantic graph: two banks may fuse only when they cover the same number of lanes and, for
/// **every** lane `i`, the later bank's lane `i` reads the earlier bank's lane `i`. Everything
/// else `chains_into` demands -- sole readership, no observed alias, no sidechain, no delay, not
/// the session output -- can only *decline* a merge, and a window wider than the permutation that
/// actually happens is always safe. So this is a superset of what the runtime will do, computed
/// without the runtime's bindings, which the lowering does not have.
///
/// It is deliberately not "every bank connected to another bank by any edge". That coarser union
/// would fold whole racks of unrelated cohorts into one span on sessions where nothing can fuse,
/// and every op inside a span is an op whose physical slot may not be recycled: an over-wide
/// window is sound but it costs arena buffers, and this keeps the cost proportional to the merges
/// the runtime can actually take.
fn chainable_bank_groups(
    banks: &[Vec<GraphNodeId>],
    spec: &GraphSpec,
    elided: &[bool],
    main_in: &[Vec<usize>],
) -> Vec<Vec<GraphNodeId>> {
    // A bank with an id that is not a node of this spec pairs with nothing: the lane alignment
    // below is positional, so a silently dropped member would compare the wrong lanes. Such an
    // id is already documented as ignorable ("a stale id can only widen a window"), and leaving
    // its bank unpaired keeps that true.
    let interned: Vec<Option<Vec<usize>>> = banks
        .iter()
        .map(|members| {
            members
                .iter()
                .map(|id| node_index(spec, id).map(|index| index as usize))
                .collect::<Option<Vec<usize>>>()
        })
        .collect();
    let mut bank_lane: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    for (bank, members) in interned.iter().enumerate() {
        for (lane, node) in members.iter().flatten().enumerate() {
            bank_lane.insert(*node, (bank, lane));
        }
    }
    let producer = |node: usize| first_main_producer(spec, elided, main_in, node);
    let mut parent: Vec<usize> = (0..banks.len()).collect();
    fn root(parent: &mut [usize], mut bank: usize) -> usize {
        while parent[bank] != bank {
            parent[bank] = parent[parent[bank]];
            bank = parent[bank];
        }
        bank
    }
    for (later, members) in interned.iter().enumerate() {
        let Some(members) = members else { continue };
        // The candidate predecessor is whichever bank owns lane 0's producer; a bank owns each of
        // its members exactly once, so there is at most one candidate and no search.
        let Some(first) = members.first().copied().and_then(producer) else {
            continue;
        };
        let Some((earlier, lane)) = bank_lane.get(&first).copied() else {
            continue;
        };
        let Some(Some(before)) = interned.get(earlier) else {
            continue;
        };
        if lane != 0 || earlier == later || before.len() != members.len() {
            continue;
        }
        if !members
            .iter()
            .zip(before.iter())
            .all(|(after, before)| producer(*after) == Some(*before))
        {
            continue;
        }
        let (a, b) = (root(&mut parent, earlier), root(&mut parent, later));
        parent[a] = b;
    }
    let mut grouped: BTreeMap<usize, Vec<GraphNodeId>> = BTreeMap::new();
    for (bank, members) in banks.iter().enumerate() {
        let group = root(&mut parent, bank);
        grouped
            .entry(group)
            .or_default()
            .extend(members.iter().cloned());
    }
    grouped.into_values().collect()
}

/// The op ranges over which a bank reorders the schedule, merged into disjoint spans.
///
/// A bank's window runs from its first member's op to its last: `runtime::units_of` emits the
/// whole bank at the first position, so every op in between executes in some other order than the
/// one colouring saw. A bank with fewer than two ops reorders nothing and contributes no window.
///
/// Overlapping windows are merged, because two banks that interleave at one level reorder each
/// other's ops as well as their own; the merged span is the range over which no slot may be
/// recycled. Returns `(defer_release, closes_here)`, both indexed by op:
///
/// * `defer_release[o]` -- a slot whose last reader is op `o` must be *held*, not freed, because
///   some op in the same span still executes after `o` does;
/// * `closes_here[o]` -- op `o` is the last op of a span, so everything held may be released once
///   `o` is behind us.
fn bank_windows(
    node_op: &[Option<OpIndex>],
    banks: &[Vec<GraphNodeId>],
    spec: &GraphSpec,
    op_count: usize,
) -> (Vec<bool>, Vec<bool>) {
    let mut spans: Vec<(usize, usize)> = Vec::with_capacity(banks.len());
    for bank in banks {
        let (mut first, mut last) = (usize::MAX, 0usize);
        let mut seen = 0usize;
        for id in bank {
            let Some(index) = node_index(spec, id) else {
                continue;
            };
            let Some(op) = node_op[index as usize] else {
                continue;
            };
            first = first.min(op as usize);
            last = last.max(op as usize);
            seen += 1;
        }
        if seen > 1 && first < last {
            spans.push((first, last));
        }
    }
    spans.sort_unstable();
    let mut defer_release = vec![false; op_count];
    let mut closes_here = vec![false; op_count];
    let mut merged: Vec<(usize, usize)> = Vec::with_capacity(spans.len());
    for (first, last) in spans {
        match merged.last_mut() {
            Some(open) if first <= open.1 => open.1 = open.1.max(last),
            _ => merged.push((first, last)),
        }
    }
    for (first, last) in merged {
        // The releasing op is the span's *reader*: a slot whose last reader is `last` is safe to
        // reuse afterwards, because the span is over by then. Hence the half-open range.
        for slot in defer_release.iter_mut().take(last).skip(first) {
            *slot = true;
        }
        closes_here[last] = true;
    }
    (defer_release, closes_here)
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
/// `banks` is one entry per homogeneous bank this plan will render, each listing that bank's
/// member nodes in any order and from any rack. It is the *plan's* answer, not a guess from the
/// node id, and it is used for one thing: the op ranges those banks reorder. Ids that are not
/// nodes of this spec are ignored -- a stale id can only widen a window, never unsound one.
///
/// ## Banks reorder the schedule, and colouring has to survive it (issue #169)
///
/// `runtime::units_of` emits a whole bank as **one unit at its first member's op position**, and
/// that unit gathers every member before the kernel runs. Over the op range from a bank's first
/// member to its last -- its *window* -- execution is therefore a permutation of the schedule
/// this function coloured: a member scheduled after the first member runs *earlier*, and a
/// non-member op scheduled between two members runs *later*.
///
/// A *single-slot* bank may not cross a dependency level (#96 F12) and ops are level-major, so
/// every op in such a window sits at one level and no op in it reads another's output. Each op in
/// the window therefore reads only values produced before the window and is read only after it.
/// Its inputs are live entering the window and its output is dead until the window ends, so the
/// whole soundness condition collapses to one sentence:
///
/// > **No physical slot may be recycled inside a bank window.**
///
/// ## A cohort chain's window spans levels, and the sentence still holds (issues #181, #202)
///
/// `runtime::cohort_runs` renders consecutive slots of one cohort chain as **one** unit, so a
/// window can cover slot `k` at level `L` and slot `k + 1` at level `L + 1`, and the clause "no op
/// in a window reads another's output" is false there: slot `k + 1`'s op names slot `k`'s buffer.
/// The argument has to be re-made rather than reused, and it comes out the same way:
///
/// * A later slot's op is **not executed**. The chain computes it over the resident AoSoA block,
///   so the read that would have gone through the arena never happens. What was an inter-op edge
///   inside the window becomes a value passed between two slots in registers and scratch.
/// * `runtime::chains_into` merges only when the later slot's op has exactly one main input, no
///   sidechain, and no compensation-delay staging, and when the earlier slot's output is read by
///   **nothing else** -- no second consumer, no observer, no *observed* alias, and not the session
///   output. So the earlier slot's buffer, which now holds the chain's *input* rather than that
///   slot's output, has no reader inside or outside the window that could tell.
/// * Every other op in the window is still read only after the window and written only before it,
///   exactly as above.
///
/// Issue #202 rec 2 widens which pairs may fuse -- across rack locations, and into the post-input
/// builtin bank -- and none of the argument above depends on where a slot sat. What it does change
/// is how far a window reaches: a chain that runs `builtins -> EQ -> compressor -> limiter` for one
/// cohort executes that whole strip at the cohort's *first* op position, so its window spans every
/// dependency level the strip crosses. The obligation is unchanged -- no physical slot may be
/// recycled inside it -- and it is simply held over a longer range, which costs arena buffers and
/// buys the round-trips.
///
/// `chainable_bank_groups` is what makes the window the merged one. It is handed one entry per
/// bound bank and unions the pairs that could fuse, using the same lane-wise producer/consumer
/// relation `chains_into` proves, so the span this function holds is a superset of the span the
/// runtime actually permutes. A window wider than the permutation is always safe; one narrower is
/// the defect this machinery exists to prevent.
///
/// `bank_windows` computes those ranges and pass 2 *holds* every slot freed inside one until
/// the window closes, instead of returning it to the free list where an op the bank hoists past
/// its releaser could take it. `a_bank_window_never_recycles_a_physical_slot` constructs the
/// smallest graph that reaches the defect and
/// `bank_window_hoisting_preserves_dataflow_on_random_graphs` interprets seeded graphs in the
/// order the executor actually runs them.
///
/// ## The rejected alternative: dedication by bank membership
///
/// The obvious-looking fix is to extend `is_dedicated` to bank members, so a member's output is
/// never returned to the free list. **It was implemented, measured and deliberately not taken.**
/// Do not re-propose it without new evidence, because:
///
/// * **It does not fix the defect.** Dedication governs what colouring *returns*; the hazard is
///   what colouring *takes*. `take` draws from the free list with no notion of a window, so a
///   slot released inside one still reaches a member hoisted past its releaser. Over the corpus
///   `bank_window_hoisting_preserves_dataflow_on_random_graphs` draws from, 285 of 3617 graphs
///   diverge today and the window hold takes that to zero.
/// * **On its own it makes matters worse** -- 528 divergences, up from 285. A dedicated member
///   cannot fold into its producer in place, so it allocates, and the slot it allocates may be
///   one a hoisted op still needs.
/// * **It is redundant once windows hold**, and it is not free: on
///   `fixtures/session/v1/console-sixty-four-track.json` it costs 64 arena buffers (193 -> 257)
///   and 64 stereo block copies per render block, one per dynamic member whose consumer can no
///   longer consume it in place. The window hold costs nothing there --
///   `banking_a_dynamic_rack_costs_no_arena_buffers` pins the 193.
///
/// The invariant the doc on `is_dedicated` used to claim for bank members -- "no op may consume
/// a member's buffer in place" -- is not needed and is not held. A member's consumer sits at a
/// strictly later dependency level, so it runs after the whole bank unit, including the
/// observers; overwriting a member's output there is safe.
///
/// # Errors
/// See [`ProgramError`]; every variant means the caller's own invariants were violated.
pub fn lower(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    delays: &[InsertedDelay],
    banks: &[Vec<GraphNodeId>],
) -> Result<ExecutionProgram, ProgramError> {
    lower_with(spec, schedule, levels, delays, banks, true)
}

/// [`lower`], with the cohort-chain window union switched off.
///
/// The only caller is the counterfactual arm of
/// `cohort_chain_merging_preserves_dataflow_on_random_graphs`: it measures how many seeded graphs a
/// per-bank window gets wrong, which is the measurement of what [`chainable_bank_groups`] buys.
/// There is no production path to it, and there must not be -- a per-bank window is unsound for a
/// merged chain.
#[cfg(test)]
fn lower_with_per_bank_windows(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    delays: &[InsertedDelay],
    banks: &[Vec<GraphNodeId>],
) -> Result<ExecutionProgram, ProgramError> {
    lower_with(spec, schedule, levels, delays, banks, false)
}

#[allow(clippy::too_many_lines)]
fn lower_with(
    spec: &GraphSpec,
    schedule: &[GraphNodeId],
    levels: &[DependencyLevel],
    delays: &[InsertedDelay],
    banks: &[Vec<GraphNodeId>],
    chain_windows: bool,
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
        // of the aliasing is dedicated storage.
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
    // Issue #202 rec 2: the window a merged cohort chain permutes is the union of every slot's
    // ops, and a chain may now fuse across rack locations and into a builtin bank, so the union is
    // derived from the graph's own dataflow rather than from the cohort planner's grouping.
    let chained = if chain_windows {
        chainable_bank_groups(banks, spec, &elided, &main_in)
    } else {
        banks.to_vec()
    };
    let (defer_release, closes_here) = bank_windows(&node_op, &chained, spec, ops.len());
    let mut physical = vec![u32::MAX; lifetimes.len()];
    let mut free: std::collections::BTreeSet<u32> = std::collections::BTreeSet::new();
    // Slots freed inside a bank window, released together once the window closes (#169).
    let mut held: Vec<u32> = Vec::new();
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
        // Slots retire one op late: a buffer whose last reader is `op_index - 1` is only free
        // once that read has happened.
        if op_index > 0 {
            // The window that op `op_index - 1` sat in (if any) is behind us now, so the slots it
            // held are safe to hand out again.
            if closes_here[op_index - 1] {
                free.extend(held.drain(..));
            }
            // What op `op_index - 1` gave up. A dedicated buffer gives up nothing -- that is what
            // dedication is. A staging scratch belongs to the op that filled it, so it retires
            // with that op's outputs.
            //
            // Inside a bank window those slots are *held* rather than freed (#169): the bank
            // hoists some of the window's ops past op `op_index - 1`, so a slot it no longer
            // needs may be one they still do.
            let retired = expire[op_index - 1]
                .drain(..)
                .filter(|buffer| !lifetimes[*buffer as usize].dedicated)
                .map(|buffer| physical[buffer as usize])
                .chain(staging_release.drain(..));
            if defer_release[op_index - 1] {
                held.extend(retired);
            } else {
                free.extend(retired);
            }
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
mod tests;
