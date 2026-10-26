# The executor order is a topological order of the op graph contracted by cohort

Draft slice 6a (#2134) of the banking design. Prepares the lowering and the colouring for cohorts whose
members sit at different dependency levels (M2). Class A. No production plan changes shape in
this slice (no cross-level cohort exists until slice 6b (#2136)); the gates construct them.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Problem

`runtime::units_of` emits a bank as one unit at its first member's op position, and the #169
colouring argument rests on "a single-slot bank may not cross a dependency level (#96 F12) and ops
are level-major" (`crates/graph/src/program.rs:530-560`). A cohort whose members have different
ASAP levels breaks that premise in two ways: hoisting the later member runs it before its
producers; sinking the earlier member to the later one's position leaves the earlier member's
external consumers (routes, sends, meters, taps, PDC staging) at their canonical positions, before
the sunk unit. Neither hoist nor sink alone is sound.

## The rule

The executor's unit order is a **topological order of the op graph contracted by cohort**: each
cohort is one node, every other op is its own node. Kahn's algorithm with a `BTreeSet` ready set
keyed by the **canonical schedule index** (a cohort's key is its first member's index). Kahn
guarantees soundness whatever the key; this key reproduces today's unit order exactly on every
plan whose cohorts are single-level, because today's order is the canonical order with each bank
at its first member's position. Every non-member consumer of a member sinks transitively with the unit.

Why not decline the cohort when a member has an external consumer before the placed position: a
member's external consumers are the ops that connect it to later segments; declining loses every
merge in which one member's consumers sit canonically before another member's producers, which on
any graph with two submix depths is most merges (on C1: the tracks' routes into the groups sit
between aux A's and the groups' canonical positions). Under contraction the consumers move with
the unit and nothing is declined.

Acyclicity: an edge between two cohorts goes strictly upward in `L` (the planner's sweep); an edge
through plain ops is a strip-DAG edge, which the sweep also ordered; so the contracted graph is
acyclic, and the test below checks it on random graphs rather than assuming it.

## Smallest closable slice

- `units_of` / `build_sequential`: build the contracted graph from the planned units and the op
  dataflow (`op_dataflow`, `taps_by_op`), run the keyed Kahn, emit units in that order. On every
  fixture the emitted order equals today's (asserted by a count of positions that moved: zero).
- `program::lower`: colour buffers over the **executed** order, not over the canonical schedule
  plus a window permutation. Within a cohort unit the member ops are gathered before the kernel
  and scattered after, so the intra-unit permutation is the one `render_bounds` already handles;
  across units nothing is permuted relative to the coloured order. The #169 section of the doc is
  re-made on that basis; the sentence "no physical slot may be recycled inside a bank window
  except to an op proven to render after the run that last touched it" keeps its meaning with the
  window being the unit.
- `cohort_runs`' "a successor is always later in unit order" is re-made on the executed order.
- `route_fold` unchanged: a cohort whose lane order does not realise the master's association
  list declines the fold (bits identical, counter lower).

## Gates (one test per claim, red first)

1. `crates/graph/src/program/tests.rs`, `contracted_order_preserves_dataflow_on_random_graphs`:
   seeded DAGs where a test-support cohort former groups nodes of one key across levels when
   their windows intersect, with plain consumer ops (routes, meters) hanging off early members;
   the executed order is interpreted against the schedule's dataflow: every op reads its
   producers' values, every consumer runs after its producer, no slot is recycled early. Red on
   the first-member hoist (the audit's aux/group shape, constructed as
   `a_members_route_never_renders_before_the_sunk_cohort`), and red on a sink that leaves a
   member's meter at its canonical position. Test value: red when any op renders before one of
   its producers under a cross-level cohort.
2. `the_banked_console_arena_equals_the_bank_unaware_arena` (graph-compiler) unchanged and
   green; the one-time `to_bits` comparison of the console fixtures' output is PR evidence.
3. `crates/graph` unit test: on every standing fixture the contracted order equals today's unit
   order (moved count 0). Red on a key that sinks a same-level bank past a consumer. Test value:
   red when the new order moves a unit that did not need to move.

## Paths

`crates/graph/src/runtime.rs` (`units_of`, `build_sequential`, `cohort_runs`),
`crates/graph/src/program.rs` (colouring over the executed order, `render_bounds`, the #169 doc
section), `crates/graph/src/program/tests.rs`, `docs/rulings/` (a short ruling: "a bank is
placed at one level its members share; the executor order is the contracted topological order;
#96 F12 restated"), this spec.

## Dependencies

None. Unblocks slice 6b (#2136). Independent of #2131 and slice 4 (#2133) (disjoint files).
