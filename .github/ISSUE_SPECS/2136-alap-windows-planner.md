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

## Evidence (attempt 1, 2026-10-27, branch `codex/batch-b14-windows`, base main `3ef18112f`)

### The open point, settled

**There is no width-dependent rule.** Both widths plan C1's submixes by the same levels (probe of
the compiled plan at W=8, and #2143's W=4 counters agree with it): the two auxes take the tracks'
post-*fader* sends, the groups their post-*pan* routes, so the auxes' strips start one level before
the groups'. `pre_insert` is `{aux A, aux B}` at level 13 and `{4 groups}` at 14 at both widths;
the drive is `{aux B}` at 18 and `{aux A, 4 groups}` at 19 (aux A's delay brings its drive level
with the groups'). #2143's "2 banks over six strips" is those two pre banks at W=4, not one pooled
bank, and the finish map's reading (aux A's pre with the groups, aux A's drive alone) had the two
auxes swapped. So the spec's gate-3 plan (one pre bank of six, 17 BSR) rested on that misreading.

**Under the spec's windows aux A's pre cannot join the groups.** ALAP is anchored at the session
output, and aux A's delay puts aux A on the longest path: its pre window is the point `[13, 13]`
(groups `[14, 14]`, aux B `[13, 14]`). What the windows can merge on C1 is the drive: aux B's
`[18, 19]` meets aux A's and the groups' `[19, 19]`.

**Measured trade (C1, `bench console --bank-report`, `taskset -c 31`, one run each, one warmup,
two rounds; p50 us, round 2; plans forced by a scratch level override, not committed; every plan
bit-identical on all 17 rows):**

| plan | static BSR W=8 16/32/64 | all-active p50 16 / 32 / 64 |
|---|---|---|
| S: level bucketing (today) | 20 / 28 / 44 | 99.30 / 147.44 / 245.03 |
| M1: drives merged, builtin stages left by level (the spec's effect-only windows) | 19 / 27 / 43 | 98.81 / 149.46 / 245.47 |
| M2: drives merged and aux B's fader and matrix with them (one placement for both planners) | 19 / 27 / 43 | 96.23 / 145.70 / 245.03 |
| M3: all six submixes aligned at every stage (needs the depth to grow past ALAP) | 16 / 24 / 40 | 96.29 / 146.44 / 245.20 |

M1 merges the drive but leaves aux B's fader and matrix banks apart from the groups', so the drive
no longer fuses into the fader (one H2 round trip for the whole bank): it gains nothing (+1.4 % at
32). M2 keeps the fusion and is the cheaper plan. M3's extra 3 BSR are aux banks that skip on
silence anyway on these rows; it buys nothing measurable and is outside the window definition.

**The rule kept:** one placement, computed once per compile, feeds both bank planners (rack chains
and the builtin strip stages), and a stage that a bankable chain feeds directly *follows* that
chain instead of being placed on its own. The keyed nodes are every post-input builtin stage,
every rack chain's first slot and every fader no bankable effect feeds; a chain's later slots, a
fader behind a chain and every matrix stage follow their producer. That is what keeps a strip's
stages in one lane set and its chains fused. A stage-wise rule with every fader keyed (the first
try) merged the faders of bare and stripped tracks in `misaligned_lane_sets_decline_the_merge`
and broke seven effect-to-fader fusions there; a whole-segment rule (a cohort per strip segment
signature) kept every fusion but put aux B with the groups' pre and cost 2 BSR at W=4 (31 vs 29).

### What changed

- `crates/rack-compiler/src/lib.rs`: `CohortWindow`, `PlacementNode`, `Placement`,
  `place_over_windows` (the sweep: keyed nodes earliest deadline first within their key class,
  placed at the largest floor of the stab's members; every other node at its floor;
  `RackCompileError::Cyclic`). `plan_bank_groups` is unchanged: it is fed placement levels in
  place of dependency levels (one bucket planner, gate 2 then holds by construction).
- `crates/graph-compiler/src/banks.rs`: `placement_levels` (ASAP, ALAP anchored at the output,
  the keys above), `PlacementLevels`, `bind_rack_banks_placed`; the #966 path check stays on the
  graph's levels (a bankable chain whose slots are not consecutive still fails the compile), the
  bucketing and the bind's one-level-per-slot check read the placement.
- `crates/graph-compiler/src/compile.rs`: one placement per compile, handed to the rack planner and
  to the builtin strip planner (resource preflight, scalar owners and the attach).
- `crates/graph-compiler/src/lib.rs`: `BankUtilisation::of` intersects windows
  (`GraphRackBankReport::windows`: a keyed stage's `[asap', ALAP]`, a follower's placement alone).
- `crates/graph/src/lib.rs` `with_builtin_banks`: the one-level-per-builtin-bank clause is gone
  (#2134's rule; a cyclic contraction still fails at bind).

### Gates

| gate | test | red on |
|---|---|---|
| 1 | `rack-compiler` `windows_merge_what_levels_split` | sweep that ignores `asap'` after a commit: `p at 5 feeds q at 1` |
| 2 | `rack-compiler` `invariants_hold_on_seeded_corpus` extended (single-point windows reproduce the plan) | stab that takes every waiting node of the class: `case=0: single-point windows changed the plan` |
| 3 | `graph-compiler` `tests/console_banking.rs` `the_c1_submix_drives_are_one_cohort_over_level_windows` (C1-16/32/64 at the build's width, W-generic; irregular pairs 0) | level bucket (no keyed node): `C1-16: the submix console groups` (`{aux B}` drive apart) |
| 3 | `src/tests/console_banking.rs` `every_console_slot_binds_banked_at_every_track_count` rewritten: one group per (class, placement level), `post_insert` at one level per pool whatever the insert count | (the level bucket fails its window claim) |
| - | `misaligned_lane_sets_decline_the_merge` (unchanged) | every fader keyed: `24` builtin banks for `26` (fusion broken) |

Rewritten for the new plan: `bank_levels.rs` (no "cross-level bank" count: a cross-level bank is
now a plan bind accepts and renders at the scalar bits, which the file still asserts; the
misaligned-slot count reads each lane's level above its own chain's first slot; bank pins 47/23 ->
46/22, 45/22 -> 44/21, 11/7 -> 10/6: the early limiter now banks with the others); the C1 #2134
order test keeps only the one-level fixtures; `a_hot_padded_post_insert_limiter...` asserts a mixed
padded bank at every width over the track counts rather than at each one. The W=4 pins in
`bank_levels.rs` were derived, not run (this host binds C1's console at eight lanes only): a
scratch probe counted the bindable slots of the same plan at four lanes and reproduced the base
tree's W=4 pins (47, 45, 11) before giving the new ones. The arm64 leg confirms them.

### Bit identity (one-time, not committed)

All 17 `BANK_REPORT_WORKLOADS` rows (C1 16/32/64 in Load, Typical, AllActive, Sparse, Toggle, the
reverb row, the irregular graph), 600 blocks through `console_workload::SessionRuntime`, FNV-1a of
every host word: identical on main `3ef18112f` and on this branch. `route_folds` is 0 on every row
on both.

### W=4 pins (`tools/wasm-gates`)

`scripts/run-wasm-gates.sh` (wasm leg): `banks` 15 -> 14 on all four rows (the spec predicted no
move; that prediction rested on the misread plan above). `silent_skips` 260/260/259/259 ->
4/4/3/3: aux B's drive, fader and matrix were a one-lane chain that skipped on silence each block;
they are now lanes of banks that hold the groups, which run. `bank_slot_runs_total` and every
other counter are unchanged. Re-pinned from the wasm leg; the arm64 native leg was not run here.
`w4_counters_describe_the_designs_plan` restated: 28 BSR, six submix banks.

### BSR and CPU (descriptive; `artifacts/steps/bank-design-6b/`)

| row | static BSR | executed BSR/block | banks | p50 us main -> branch |
|---|---|---|---|---|
| C1-16 Load | 20 -> 19 | 0 -> 0 | 10 -> 9 | 21.16 -> 19.97 |
| C1-16 Typical | 20 -> 19 | 10.00 -> 10.00 | | 62.84 -> 60.98 |
| C1-16 AllActive | 20 -> 19 | 16.66 -> 16.66 | | 98.32 -> 97.47 (-0.9 %) |
| C1-32 AllActive | 28 -> 27 | 24.66 -> 24.66 | 14 -> 13 | 147.49 -> 145.28 (-1.5 %) |
| C1-64 AllActive | 44 -> 43 | 40.66 -> 40.66 | 22 -> 21 | 247.27 -> 246.72 (-0.2 %) |
| C1-64 Typical | 44 -> 43 | 24.00 -> 24.00 | | 153.32 -> 153.80 (+0.3 %) |
| irregular graph | 38 -> 28 | 38.00 -> 28.00 | 23 -> 15 | 265.82 -> 192.93 (-27.4 %) |

W=8 C1 lands at 19/27/43, not the spec's 17/25/41 (see the open point). Executed BSR on C1 does not
move: the merged drive bank absorbs aux B, which skipped on silence. The irregular row is where
the windows pay: `post_insert` of tracks with different insert counts and the submix strips of
different depths are one cohort per pool. `mergeable_partial_pairs` is 0 on every row.

## Attempt 2 (2026-10-27, after the bounded verify's FAIL)

- **MAJOR fixed: a merge that saves no bank is refused.** `place_over_windows` commits a stab's
  members at one level only when `ceil(n / W)` over them is below the sum, over their own floors,
  of `ceil(count / W)`; otherwise every member keeps its own floor (the level plan). Why: such a
  merge only moves lanes between banks. On C1 without aux A at W=4 (aux B plus four groups) the
  windows would have built `{aux B, drums, fx, music}` and `{vox}`, still two drive banks, and with
  aux B silent and the groups active, two banks would run where one ran and aux B's bank skipped
  (its fader and matrix follow it) -- more work on silence for nothing. The width reaches the
  sweep from the compile's dispatch; with no bank width nothing merges.
- Test: `rack-compiler` `a_merge_that_saves_no_bank_is_refused` (one node alone at its floor plus
  `W` others reaching it: refused; plus `W - 1`: taken; at four and eight lanes). Red on the
  mutant "merge whenever windows intersect": `W=4, 4 others: x merges only when a bank is saved`
  (left 1, right 0).
- C1's drive merge still saves a bank (W=4 3 -> 2, W=8 2 -> 1): every row's plan (banks, pairs,
  static BSR and every pack group's units) is the attempt-1 plan, so no pin and no benchmark moved
  and the attempt-1 bank-report records stand. Bit identity re-run: all 17 rows identical to main
  `3ef18112f`.
- MINOR 1: `assert_no_slot_is_recycled_inside_a_bank_window` accepts a pair in one fused run in
  chain order and runs on #2134's cross-level corpus again.
- MINOR 2: `w4_counters_describe_the_designs_plan` no longer asserts the constant 28; the static
  count's one owner is `the_c1_submix_drives_are_one_cohort_over_level_windows`.

## Attempt 3 (2026-10-27, after the second verify's FAIL)

- **MAJOR fixed:** the no-bank-saved rule now counts the key's nodes already placed at each level
  (`placed_at`, per key and level): merged at `M`, `ceil((P_M + n) / W) - ceil(P_M / W)`; split,
  the same difference summed over the members' floors; merge only when the merged delta is
  strictly smaller. Nodes a later stab places at those levels are not seen (a limit of the online
  greedy). Reproducer added as the second case of `a_merge_that_saves_no_bank_is_refused` (W-1
  nodes already at level 2; late `x` floor 1, `y` floor 2), at W=4 and W=8. Red on the attempt-2
  rule: `W=4: x keeps its floor when the merged level's banks would not drop` (left (2, 2), right
  (1, 2)).
- MINOR: the slot-recycle checker's same-run exemption also requires that the later op reads the
  earlier op's output.
- Ruling point 10 records the accepted trade: a merge that saves a static bank may still run more
  banks on a silent-heavy block; the #2135 packer groups by state at run time.
