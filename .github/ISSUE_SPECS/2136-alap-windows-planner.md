# Plan cohorts over ASAP/ALAP windows

Draft slice 6b (#2136) of the banking design. Amends #96 F12 ("a bank never crosses a level") to "a bank's
members share a feasible level and the bank is placed at one"; restates #966's per-slot level
alignment check as a window check (a bankable chain whose slots are not consecutive in ASAP is
still an invariant violation and still fails the compile); closes #1098's H2 note (`post_insert` split by insert
count). Class A under #2131's per-lane un-bypass law: a cohort is a lane regrouping; the fold order
stays canonical.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Problem

Chains are bucketed by the longest-path level of their first slot (`banks.rs:250-301`). On C1 the
groups, the auxes and the master sit at three or four levels and bind 13 bank-slot runs for 7
strips (audit section 2); tracks with different insert counts split `post_insert` into one group
per level (`console_banking.rs:468-493`). Many of these segments have windows with a common level.

## Smallest closable slice

- **The strip DAG**: chain segments as nodes; an edge wherever any op path leads from one segment
  to another (routes, sends, sidechain taps, PDC staging, a per-node insert between two segments
  of one strip). ASAP is the existing longest-path level; ALAP is the reverse longest path to the
  session output's segment.
- `rack_compiler::plan_bank_groups` takes `CohortWindow { asap, alap }` per candidate in place of
  `CohortLevel`, plus the strip-DAG edges among candidates, and emits groups with a `placed`
  level. The sweep: topological ready set; per key class sort by `(alap asc, active_count desc,
  id asc)` (earliest deadline first; F5.2 as the secondary key); stab at the first candidate's
  `alap`; fill `W` in sort order; `placed = max asap'`; commit; consumers' `asap'` rise.
  Deterministic in the multiset; a session whose windows are all single points produces today's
  plan group for group (gate 2).
- Lane order inside a bank stays strictly ascending by id (#206); the sweep decides membership
  only. The route fold is matched, not forced: when a merged submix cohort's id order is the
  master's contributor order the fold applies, else it declines and the report counts it.
- A member whose segment ends early (aux A's pre segment before its per-node delay) joins by
  prefix; the cohort's fusion stops where its dataflow stops (the lane-wise `chains_into` rule),
  and the bank pays one planar round trip there. Planner decision: the round trip (H2) is cheaper
  than a padded three-slot bank (H5, about 9.3 us at W=8); slice 0 (#2132)'s rows measure it.
- `graph-compiler`: the per-slot `level + k` alignment check (`banks.rs:264-283`, #966) is
  replaced by the window; the sidechain-lifted case becomes an ordinary ASAP. `bind_group_banks`
  binds a slot when every member runs it (unchanged). The lowering places the unit through slice
  6a's contracted order.
- Slice 0 (#2132)'s report gains `mergeable_partial_pairs`: the count of pairs of partial banks of one key
  class (same rack, same leader program or a subsequence, same pool class, same fold target) whose
  windows intersect and whose union fits `W`, a bank's window being the intersection over its
  members of `[asap'_m, ALAP_m]` after placement (so no pair the sweep could not merge is
  reported). The finish line is zero.

## Gates (one test per claim, red first)

1. `crates/rack-compiler` unit test `windows_merge_what_levels_split`: candidates with windows
   `[0,0]`, `[1,3]`, `[2,3]`, `[1,2]`, `[4,4]` of one key at W=4 form three banks (`{a}`,
   `{b,c,d}` placed at 2, `{e}`) and every edge goes upward; a feeding pair never shares a bank.
   Red on a sweep that ignores `asap'` after a commit (producer `[0,5]` placed at 5 with consumer
   `[1,6]` placed at 1). Test value: red when the planner places a consumer at or before its
   producer.
2. `crates/rack-compiler`: single-point windows reproduce today's plan group for group on the
   seeded corpus (`invariants_hold_on_seeded_corpus`, `crates/rack-compiler/src/lib.rs:1024`,
   extended). Red on a changed tie-break.
3. `crates/graph-compiler/tests/console_banking.rs` on the C1 fixture at 16 tracks: at W=8 the
   submix pre cohort is one bank of six (four groups, aux B, aux A's pre segment), the drive
   cohort one bank of five, aux A's drive alone, the master alone: 17 BSR against today's
   **measured** 20 (#2132; the design's 21 was a prediction); `post_insert` of tracks with 0, 1 and
   2 inserts is one group (an irregular-graph claim, not a C1 one: every C1 track has no insert);
   `mergeable_partial_pairs == 0`. At W=4 the same session is 29 BSR (measured today, #2143) and
   also `mergeable_partial_pairs == 0`; the W=4 pins' `banks` stays 15. Red on a level bucket. Test value:
   red when two partial banks of one key with intersecting windows are left unmerged. The existing
   "one group per (pool class, level)" assertion is rewritten to the window claim in the same PR.
4. Bit identity: the console, C1 and irregular fixtures rendered before and after, including the
   `Toggle` run, every host word equal by `to_bits` (PR evidence); the route-fold counter recorded
   per fixture.
5. CPU, descriptive: slice 0 (#2132)'s rows, one native run and one W=4 counter run. Expected on
   C1 static BSR: W=8/16 20 -> 17 (-15 %), W=8/32 28 -> 25 (-11 %), W=8/64 44 -> 41 (-7 %); W=4
   unchanged (29, 45, 77). **Executed** BSR on the all-active row moves only about -3 %: the banks
   the merge absorbs are mostly aux A's, which already skip on silence (3.34 of 20 today). In
   `Typical` a six-lane bank skips a slot less often than a five-lane plus a one-lane bank, which
   is why this slice runs after #2135. The irregular row is where W=4 gains; record it.

## Paths

`crates/rack-compiler/src/lib.rs`, `crates/graph-compiler/src/banks.rs`, `crates/graph-compiler/
src/lib.rs` (strip DAG, ALAP, the report), `crates/graph-compiler/src/tests/console_banking.rs`,
`crates/graph-compiler/tests/`, `docs/rulings/` (the 6a ruling extended with the planner), this
spec.

## Dependencies

Slice 6a (#2134), immediately before this one in the same stream; #2131 is merged (the law that
makes a regrouping class A). Runs after #2135. Unblocks the static finish line and slice 8 (#2138);
slice 7 (#2137) is deferred.

## Complexity

`O(N + E)` for the two level passes, `O(N log N)` for the sorts, `O(N x slots)` for subsequence
masks, `O(P + edges)` for the contracted order; no `MAX_TRACKS`.


## Amendment (finish map, 2026-10-26; tree main `26d8f97c8` + #2142 PR #2154 `4f5d448eb`)

- **Settle this first (not in the original spec).** #2143's W=4 plan pools aux A's pre segment with
  the groups (pre: 2 banks over six strips, 6 BSR; 29 BSR total) while #2132's W=8 plan binds aux
  A's pre apart (12 submix BSR). Both are "today". Find the rule that splits them at W=8 (the
  console planner's pre/post alignment for chain fusion, `console_banking.rs:468-493`'s "one
  padded group per (pool class, level)", and the H2 round trip against the H5 padded bank) before
  writing the sweep: part of the W=8/16 gain may be a fusion decision, not a window. The spec's
  "planner decision: the round trip is cheaper than a padded three-slot bank" is measured on the
  C1 all-active row in this PR, and the cheaper plan is kept.
- Value, plainly: -15 % static BSR at W=8/16, 0 % at W=4 on C1; about -3 % executed on the
  all-active row. The owner may defer this slice (with #2134 and #2138) after S2 and #2135; the
  static finish line is then recorded as met under the one-level rule with this expectation.
- The `Toggle` row is in the one-time `to_bits` evidence (every host word, 600 blocks, the #2142
  method). The fold counter (`route_folds`) is recorded per fixture: a merged submix cohort whose
  id order is not the master's contributor order declines the fold (same bits, more cost).
