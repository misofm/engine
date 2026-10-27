# A second planner pass over the insert nodes the chain pass left per node

Draft slice 8 (#2138) of the banking design. #163 phase 1's eligibility rule (bank by the effect's
homogeneous bank kernel contract, never by placement) is unchanged; its placement is widened.
Class A.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Problem

Inserts are offered to the planner as whole per-strip chains (`banks.rs:174-225`). A strip whose
insert order matches no leader (`[comp, eq]` beside `[eq, comp]`), or an insert remainder below
`min_padded_insert_members`, renders per node although the same effect with the same key sits in
a padded bank with free lanes on another strip, or on a submix.

## Smallest closable slice

- After the chain pass (`plan_bank_groups` over `Dynamic` chains, `crates/graph-compiler/src/banks.rs:174-310`;
  bucketed by level today and by window after #2136), every insert effect node that is
  not bound is re-offered as a one-slot candidate `(EffectProgramKey, window of the node)` and the
  sweep runs again over these; groups pad from `min_padded_insert_members` as today; each bound
  group is a one-slot `BankChain` placed by slice 6a (#2134)'s rule. A node still unbanked renders per
  node, as today.
- The chain pass is unchanged for the strips it banks; a strip whose insert chain is partly banked
  by pass 2 pays the planar round trip it pays today between its pre and post chain segments.

## Gates (one test per claim, red first)

1. `crates/graph-compiler/tests/`: a session of six tracks with `[eq, comp]`, `[comp, eq]` and
   `[comp]` inserts twice each at W=4: pass 1 banks the `[eq, comp]` leader with the `[comp]`
   subsequence members; pass 2 banks the two `[comp, eq]` strips' compressors with each other as a
   one-slot bank (their EQs likewise, one level later); the per-node effect count falls from 4 to
   0. Red on a second pass that groups by chain. Test value: red when a
   bankable insert with a free same-key lane in its window renders per node.
2. Bit identity against the per-node twin (the existing per-node differential, one new fixture
   row).
3. The irregular-graph row of slice 0 (#2132), recorded: one `bench console --bank-report` run
   (native) and one W=4 counter run (#2143); the row's `banks`, lane use (46.4 % today) and per-node
   effect count before and after.

## Paths

`crates/graph-compiler/src/banks.rs`, `crates/graph-compiler/src/lib.rs` (candidates per node),
`crates/graph/src/runtime.rs` only if a one-slot chain with a `Dynamic` location needs a unit
form it lacks (expected not: #96's per-effect one-slot banks were this shape), tests, this spec.

## Dependencies

#2131 is merged (the law that makes a regrouping class A). #2136 is recommended first so the
pass runs once on windows (it is sequencing, not substance: the pass takes the candidates however
they are bucketed). If #2136 is deferred, this slice runs on levels with no other change.


## Amendment (finish map, 2026-10-26; tree main `26d8f97c8` + #2142 PR #2154 `4f5d448eb`)

- Dependency on #2136 restated as sequencing. Schedule: stream A step 5, after #2136 (or after
  #2135 if 6a/6b are deferred). The claim is on user graphs (ragged inserts), measured by the
  irregular row only; C1 has no inserts on tracks.

## Evidence (attempt 1, 2026-10-27, branch `codex/batch-b14-leftover`, base #2136 attempt 4 `bf05356df` (merged with its final head `244033a01`))

### What changed

- `crates/graph-compiler/src/banks.rs` `leftover_pass`, called from `bind_rack_banks_placed`
  after the chain pass and #971's move: every `Dynamic` chain node no bound bank carries, whose
  own key does not block banking, is a one-slot candidate. The candidates are placed by
  `rack_compiler::place_over_windows` (the chain pass's sweep and its merge counter, keyed by
  program key and pool class) over a sub-problem: the candidates and every graph neighbour of one,
  fixed at its placement level (`PlacementLevels::neighbours`, recorded in `placement_levels`). So
  a move keeps every edge strictly upward and the contracted graph acyclic. The merge counter is
  the sweep's `banks_of` callback, answered by `plan_bank_groups` over the set's one-slot
  candidates: a cross-level merge is taken only when it lowers the planner's own group count
  (coordinator direction 2026-10-27: no second count).
  The placed candidates are bucketed by level, planned by `plan_bank_groups`, bound by
  `bind_group_banks`, and appended to the plan; a group below `min_padded_insert_members` does
  not bind and renders per node.
- `crates/graph-compiler/src/lib.rs`: `RackChainId::slot` (`None` a whole chain, `Some(k)` the
  leftover candidate of slot `k`; its `chains` entry is that one node); `scalar_in` dedups.
- No runtime change: a one-slot `Dynamic` bank is #96's shape.

### Gates

| gate | test | red on |
|---|---|---|
| 1 + 2 | `graph-compiler` `tests/bank_levels.rs` `leftover_inserts_bank_by_key_in_their_window` (new row of the scalar-oracle differential) | no second pass (a second pass that groups by chain reproduces pass 1): per node `3`, expected `0` |

Gate 1 as written does not hold on the tree: the chain pass's leader is `[comp, eq]` (#99's
order), and at W=4 the two `[comp, eq]` strips already pad, so the six-track session leaves two
EQs per node at W=4 and nothing pass 2 can bank at W=8 (below its threshold of 3). The row is
nine tracks (three of each program): the chain pass leaves the three `[comp, eq]` EQs per node at
both widths, pass 2 binds them as one padded bank, per node 3 -> 0 at W=4 and W=8. The W=4
count (5 banks) is derived from the plan (this x86-64-v3 host's factories decline four lanes);
the arm64 leg runs it.

Re-pinned, each because pass 2 binds what the chain pass left per node (`bank_levels.rs`):
64-track console less one EQ (three rows and the mono row) 22/46 -> 24/48 (the mixed group's EQs
and compressors, `ch00`'s early compressor stays per node); less nine EQs 21/44 -> 23/46; the
soft-clip row 6/10 -> 7/11; the realigning row 9/18 -> 13/22 (B's EQs, A's and B's compressors,
A's transients, pooled over both four-lane A/B groups); the reduced #970 console W=4 3 -> 4.
`src/lib.rs` `chains_of_different_depths_share_a_cohort_through_identity_slots` and
`bank_membership_is_independent_of_entry_order`: slot 1's half now binds as a leftover bank.
#2136's `assert_placement_counts_the_plan` leaves leftover groups (`slot: Some(_)`) out: they are
placed by their own sub-problem after the placement's count.

### Bit identity (one-time, not committed)

All 17 `BANK_REPORT_WORKLOADS` rows, 600 blocks through `console_workload::SessionRuntime`, FNV-1a
of every host word: identical on `bf05356df` and on this branch, including the irregular row,
whose plan moved.

### Gate 3: the irregular row and C1 (descriptive)

`bench console --bank-report`, native AVX2 (W=8), `taskset -c 31`, one invocation per tree, one
warmup and two measured rounds, no retries; round 2 shown; base `bf05356df` then branch. Per-node
inserts from a scratch compile of the fixture (`GraphRackBankReport::scalar_in(Dynamic)`), effect
banks from the bound plan. W=4 counters from the #2143 wasm leg (`simd128` under wasmtime), the
irregular row added to the counter list in a scratch run only (not committed).

| row | banks | lane use | static BSR | executed BSR/block | per-node inserts | p50 us |
|---|---|---|---|---|---|---|
| irregular graph, W=8 | 15 -> 17 | 62.9 % -> 62.1 % | 28 -> 30 | 28.0 -> 30.0 | 13 -> 5 (effect banks 28 -> 30) | 193.34 -> 171.58 (-11.3 %) |
| irregular graph, W=4 | 23 -> 24 | | | 11516 -> 11772 runs / 256 blocks | | |
| C1-16 AllActive | 9 -> 9 | 60.5 % | 19 | 16.66 | 0 | 96.81 -> 97.71 (+0.9 %) |
| C1-32 AllActive | 13 -> 13 | 72.2 % | 27 | 24.66 | 0 | 144.94 -> 147.85 (+2.0 %) |
| C1-64 AllActive | 21 -> 21 | 82.6 % | 43 | 40.66 | 0 | 248.47 -> 247.08 (-0.6 %) |

The W=4 irregular row also moves `mergeable_partial_pairs` 0 -> 1 and
`bank_slot_runs_track_group` 8704 -> 8960. The C1 rows have no track inserts, so their plans
do not move (every bank-report counter and output digest is equal), and their p50 deltas are
run-to-run noise on one plan. The irregular row's 46.4 % lane use in the spec is #2132's figure;
#2136 had already raised it to 62.9 %. Pass 2 adds two padded leftover banks there (which
effects they hold was not recorded), so lane use falls a little
while 8 per-node renders become banked lanes; the row is 11.3 % faster.

### Not done

- The W=4 pins (gate 1 row, re-pinned `bank_levels.rs` rows) are derived from the plan, not run:
  this host's factories decline four lanes. The arm64 leg runs them.
- The irregular row is not added to `W4_COUNTER_WORKLOADS` (its W=4 counters above come from a
  scratch run only).

## Attempt 2 (2026-10-27): the verify's findings

- **Blocker 1, the W=4 irregular row's mergeable pair, was a metric over-count.** The pair was a
  chain-pass `[eq, deess, shape]` group (`ch09`, `ch21`, running only `eq`, `eq` bound) and the
  leftover `[shape]` bank (`ch03`, `ch15`). Merging them leaves every slot with a lane that skips
  it, so #96 F7 (`bindable_slot_members`) binds none of them and the `eq` bank is lost. No partial
  chain-pass bank of key `shape` exists on the row, so no leftover can fill a free lane. **Owner
  decision 2026-10-27:** `mergeable_partial_pairs` counts a pair only when the merge still binds
  every slot either bank bound under F7, through the binder's own predicate (`every_member_runs`,
  `banks.rs`, now shared by `bindable_slot_members` and `BankUtilisation::of`). The finish line
  (zero pairs) and `console_banking.rs`'s assertion are unchanged. No plan and no bit moves.
  Test: `console_banking.rs` `a_pair_is_mergeable_only_when_the_merge_still_binds` (W=4 and W=8,
  pure), red on the subsequence-only count: `W=4: the [comp] lane would skip the bound eq (F7)`,
  left 1, right 0.
- **Major 3, the leftover count matches the bind.** A key class with fewer candidates than the
  padding threshold is dropped without asking its factory; every other key's factory is asked once
  (a full and a padded probe request, the bind's own shape) and a key whose factory declines (the
  delay, the reverb, a foreign width) offers no candidate. `banks_of` counts only groups that will
  bind (full, or padded from the threshold where the factory pads; a one-slot group satisfies F7).
  The report's `leftover_levels` is that count's binding set, and `bank_levels.rs`'
  `assert_placement_counts_the_plan` holds it to the leftover banks actually bound on every
  compile in that file. Red on the attempt-1 count (every group): C1 counted `aux.a/delay` and
  `master/limiter` as binding, bound none.
- **Major 2, W=4 pins run.** A scratch probe in `wasm-gate-guest` (not committed) compiles each
  `bank_levels.rs` session (written as canonical JSON) at `Simd4` in the `simd128` guest (shipped
  nightly, `-Zbuild-std`) under Node 22: every pin matches (48 four rows, 46, 11, 22, 5, reduced
  #970 4).
- **Minor 6:** the two `src/lib.rs` tests assert the chain pass's groups only again.
- **Not covered:** bypass and un-bypass on a strip split between a chain bank and a leftover bank
  (low risk: no runtime change; the bank's shunt is per lane as for any bank).
- **Rows checked after the change:** W=8 bank-report preflight, all 17 rows: banks, pairs (0),
  lane use and static BSR equal to attempt 1. W=4: the C1 counter pins pass (wasm leg); the
  irregular row (W=4 probe) 1 -> 0 pairs, banks 24 unchanged; every `bank_levels.rs` session 0
  pairs. Bit identity, 17 rows against `bf05356df`: identical. The plans did not move, so the
  attempt-1 bench record stands.

### Attempt 2 verdict (bounded opus-high verify, 2026-10-27): PASS

- `merge_binds` and `bindable_slot_members` share one F7 predicate (`every_member_runs`); the old subsequence-only count turns `a_pair_is_mergeable_only_when_the_merge_still_binds` red, and the zero-pairs finish line holds on every `bank_levels` row.
- The factory probe keeps no state past the compile; skipping it below the padding threshold is the same behaviour, since no group there can bind.
- The leftover invariant compares the count's prediction with the leftover banks `bind_group_banks` actually bound; the verifier's mutants (filter always true, probe always binds) turn it red.
- Folded MINOR: a factory error on the probe now refuses the compile, as the bind's own call does, instead of reading as a decline.
- MINOR recorded: the probe asks with W clones of one candidate's request; a factory that declined by request content would make the count wrong, which the leftover invariant check would catch.
