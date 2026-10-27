# Bank placement: the executor order is the graph contracted by bank

Planner decision, issues #2134 and #2136 (banking design slices 6a and 6b, `docs/BANKING_DESIGN.md` section 2.2, M2).
Class A: it moves no rendered bit.

## Ruling

1. **A bank is placed at one level its members share.** #96 F12 ("a bank never crosses a
   dependency level") is restated: the members of a bank need not sit at one canonical dependency
   level. They must have no path between them, and the plan's banks together must admit an
   executor order. Which members share a bank is the planner's decision (slice 6b, #2136).
2. **The executor order is the contracted topological order.** `graph::program::lower` emits ops
   in the topological order of the op graph contracted by bank: each bank is one node, every
   other node is its own. Kahn's algorithm runs with a ready set keyed by canonical schedule
   position. A bank is ready when every producer of every member has been emitted; then each
   member enters the ready set under its own position. `runtime::units_of` places a whole bank at
   its first member's op, so the bank renders after every member's producers, and every
   non-member consumer of a member sinks with it.
3. **Today's order is reproduced exactly.** On a plan whose banks each sit at one level the
   schedule is already a contracted order, min-key Kahn returns it, and no op moves. Lane chains
   and pack groups (#2142's `LaneMap::discover`), which read unit adjacency and order, are
   therefore unchanged on every such plan.
4. **No order means no plan.** A bank whose members feed one another, or a set of banks whose
   contraction is cyclic, fails lowering with `ProgramError::CohortCycle`, which bind reports as
   `graph.scheduler.layout`. Bind-time structural validation no longer requires one level per
   bank; it still requires sorted, disjoint member lists.
5. **Colouring is over the executed order.** The positions `render_bounds` bounds are op
   positions in that order, and the #169 sentence -- no physical slot may be recycled inside a
   bank window except to an op proven to render after the run that last touched it -- keeps its
   meaning with the window being the unit.

## Why not decline the cohort instead

A member's external consumers (routes, sends, meters, taps, compensation staging) connect it to
later segments. Declining every cohort in which one member's consumers sit canonically before
another member's producers would lose most merges on a graph with two submix depths. Under
contraction the consumers move with the unit and nothing is declined.

## Evidence

- `graph::program::tests::contracted_order_preserves_dataflow_on_random_graphs`: seeded graphs with
  cross-level cohorts, submixes and sends off early members render the semantic graph's value in
  executor order; cyclic plans are refused exactly when an independent search finds the cycle.
- `graph-compiler` `the_contracted_order_is_the_schedule_on_every_one_level_fixture`: no op moves
  on any standing session fixture whose banks each sit at one level.

## The planner (issue #2136, slice 6b)

6. **Cohorts are placed over level windows.** Each graph node's window is `[ASAP, ALAP]`, ALAP
   anchored at the session output. `rack_compiler::place_over_windows` places the keyed nodes --
   every post-input builtin stage, every rack chain's first slot, every fader no bankable effect
   feeds -- earliest deadline first within their key class (stage or rack, and pool class), and
   every other node as early as its producers allow. Both bank planners bucket by this one
   placement instead of by dependency level; the graph's levels and schedule are unchanged.
7. **A stage a chain feeds follows it.** A chain's later slots, a fader behind a bankable chain and
   every matrix stage are not placed on their own, so a strip's stages keep one lane set and its
   chains still fuse. Merging one stage while its neighbours stay apart costs a planar round trip
   per bank (H2), which on C1 cost what the merged bank saved (#2136 evidence, plans M1 and M2).
8. **The #966 invariant is a window check.** A bankable chain whose slots are not consecutive in
   ASAP still fails the compile; a bank slot still binds only when its members share a placement
   level at that slot.
9. **A merge that saves no bank is refused** (attempts 2 to 4). The sweep commits a stab's
   members at one level only when that adds fewer banks than the level plan would. The count is
   the bank planners' own: `place_over_windows` takes a `banks_of` callback, and the graph
   compiler's counts every node a planner will group -- a rack chain's first slot as the rack
   planner's candidate (rack, program, pool class), a builtin strip stage by (stage, pool class) --
   through `rack_compiler::plan_bank_groups` itself, whether the sweep keyed the node for
   placement or placed it as a follower. Merged at `M` the delta is `banks(P_M + members) -
   banks(P_M)`; split, that delta summed over the members' own floors. **Invariant: the set
   counted at (planner key, level) is the members the planner will group there, minus the nodes
   later stabs place at that level** (a limit of the online greedy). The report's
   `planned_levels` exposes the set and `bank_levels.rs` holds it to the plan on every compile.
   One more limit: #971 may move a stranded mono track to the stereo pool after placement.
   Otherwise each member keeps its own floor, as the level plan places it. A merge that saves no
   bank only moves lanes between banks: a strip that was alone at its level (a silent aux beside
   active groups) would run inside a bank of active lanes, with its fader and matrix behind it,
   instead of skipping with a bank of its own -- more work on silence for nothing (the owner's
   skip-work-on-silence rule).
10. **Accepted trade: a merge that saves a bank may still run more banks on silence.** At W=4,
    floors holding 3 silent, 3 silent and 2 active nodes merge into 2 banks from 3; if the two
    active lanes land in different banks, executed banks on a silent-heavy block go 1 -> 2. The
    planner optimises static banks (dense blocks); the #2135 pattern packer moves lanes by state
    at run time and is the place where activity is grouped.
