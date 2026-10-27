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
   order (moved count 0), **and `test_only_pack_groups` on the C1-16 fixture is unchanged (same
   chains, same rows)**: #2142's `LaneMap::discover` builds lane chains and pack groups from unit
   adjacency and order (`group_layout_holds`: a plain unit inside a group's span sits between two
   whole rows; resident pairing needs adjacency), so a moved unit is also a changed pack group. Red
   on a key that sinks a same-level bank past a consumer. Test value: red when the new order moves
   a unit that did not need to move.

## Paths

`crates/graph/src/runtime.rs` (`units_of` at about `:9169`, `build_sequential` `:9658`,
`cohort_runs` `:13878` and its "a successor is always later in unit order" sentence at about
`:13870`, `op_dataflow` `:13720`, `taps_by_op` `:9142`, all on the #2142 branch),
`crates/graph/src/program.rs` (colouring over the executed order, `render_bounds`, the #169 doc
section), `crates/graph/src/program/tests.rs`, `docs/rulings/` (a short ruling: "a bank is
placed at one level its members share; the executor order is the contracted topological order;
#96 F12 restated"), this spec.

## Dependencies

None (#2131, #2133 and #2142 are merged). Unblocks slice 6b (#2136). Runs immediately before #2136 in the same stream, after #2135; never alone (it changes no production plan).


## Amendment (finish map, 2026-10-26; tree main `26d8f97c8` + #2142 PR #2154 `4f5d448eb`)

- Dependencies refreshed: #2131, #2133, #2142 merged. Gate 3 gains the pack-group clause (above),
  because #2142 made unit order a pack-group input. Line cites refreshed to the #2142 branch.
- Value statement: no production plan changes shape in this slice; it is worth doing only as
  #2136's precondition. Schedule: stream A step 3, after #2135.

## Evidence (attempt 1, 2026-10-27, branch `codex/batch-b14-windows` on the #2135 head `f08c3a40e`)

### Where the contraction lives

The contracted order is computed once, in `graph::program::lower` (`contracted_order`), and the
program's op list is emitted in it: the op order **is** the executor order. `runtime::units_of`
and `build_sequential` are unchanged in code (a bank is still one unit at its first member's op),
and colouring, `render_bounds`, `chainable_bank_groups`, `op_dataflow` and `cohort_runs` all read
op positions, so they are over the executed order with no second permutation to track. This is
the spec's rule with one Kahn instead of two (lowering and the runtime would otherwise have to
agree on it). Doing the Kahn on nodes rather than units is equivalent: a bank's members all enter
the ready set together once every member's producers are out, and a bank of one is a plain node.

- `crates/graph/src/program.rs`: `contracted_order`, `ProgramError::CohortCycle`, the #169 section
  re-made on the executed order (`lower`'s doc), the `ops` field doc.
- `crates/graph/src/lib.rs` `has_valid_structural_layout` (and, with #2136, `with_builtin_banks`):
  the one-level-per-bank and source-before-first-member clauses are gone; bind still refuses unsorted or overlapping member
  lists, and a cyclic contraction fails lowering (`lowered`) with the same
  `graph.scheduler.layout`.
- `crates/graph/src/runtime.rs`: `units_of`'s doc and `cohort_runs`' "a successor is always later
  in unit order", re-made on the topological op order.
- `docs/rulings/bank-placement-contracted-order.md`: the ruling.

### Gates

| gate | test | red on |
|---|---|---|
| 1 | `graph` `program::tests::contracted_order_preserves_dataflow_on_random_graphs` (1500 seeds; 416 with a cross-level bank, 372 of them reordered, 14 cyclic and refused) | M1 lowering in the canonical order (the first-member hoist): `graph 0: op 58 rendered a foreign value`; M2 a sink that gates the bank but leaves members' consumers at their canonical positions: `graph 0: NotTopological` |
| 2 | `the_banked_console_arena_equals_the_bank_unaware_arena` unchanged | (unchanged, green) |
| 3 | `graph-compiler` `tests/console_banking.rs` `the_contracted_order_is_the_schedule_on_every_one_level_fixture` (11 session fixtures at 6a; since #2136 the C1 and sends fixtures bank across levels and are left out, the four consoles without submixes are required in) | M3 emitting a bank's members contiguously: `console-c1-16: the executor order moved 5 ops` (at 6a) |

The structural `assert_no_slot_is_recycled_inside_a_bank_window` is not asked of the cross-level
corpus: its premise (the op later in op order is the recycler) needs a window in which no op reads
another's output, and a sunk consumer of an early member breaks it without a defect. The symbolic
interpreter reads every value in render order, so an early recycle is a foreign value there.

Gate 3's pack-group clause: with no op moved the program, its units and therefore #2142's lane
chains and pack groups are the base plan's. The one-time comparison of `test_only_pack_groups` on
C1-16/32/64 was not taken: the harness keeps the bound plan private and 6a moved no op, so the
units are the base's by construction. The 6a + 6b bit identity is in `2136-alap-windows-planner.md`.
