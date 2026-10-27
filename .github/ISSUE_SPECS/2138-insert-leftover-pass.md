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

## Evidence (attempt 1, 2026-10-27, branch `codex/batch-b14-leftover`, base #2136 `BASE_HEAD`)

### What changed

- `crates/graph-compiler/src/banks.rs` `leftover_pass`, called from `bind_rack_banks_placed`
  after the chain pass and #971's move: every `Dynamic` chain node no bound bank carries, whose
  own key does not block banking, is a one-slot candidate. The candidates are placed by
  `rack_compiler::place_over_windows` (the chain pass's sweep and its merge counter, keyed by
  program key and pool class) over a sub-problem: the candidates and every graph neighbour of one,
  fixed at its placement level (`PlacementLevels::neighbours`, recorded in `placement_levels`). So
  a move keeps every edge strictly upward and the contracted graph acyclic, and a cross-level
  merge is taken only when it saves a bank (coordinator direction 2026-10-27: no second count).
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

### Bit identity (one-time, not committed)

All 17 `BANK_REPORT_WORKLOADS` rows, 600 blocks through `console_workload::SessionRuntime`, FNV-1a
of every host word: identical on `BASE_HEAD` and on this branch, including the irregular row,
whose plan moved.

### Gate 3: the irregular row and C1 (descriptive)

BENCH_TABLE
