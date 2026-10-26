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

- After the chain pass (slice 6b (#2136)'s sweep over `Dynamic` chains), every insert effect node that is
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
3. The irregular-graph row of slice 0 (#2132), recorded.

## Paths

`crates/graph-compiler/src/banks.rs`, `crates/graph-compiler/src/lib.rs` (candidates per node),
`crates/graph/src/runtime.rs` only if a one-slot chain with a `Dynamic` location needs a unit
form it lacks (expected not: #96's per-effect one-slot banks were this shape), tests, this spec.

## Dependencies

Slice 6b (#2136), and #2131 (`reset_lane` and the per-lane un-bypass law): a node moving from per node
into a bank is a regrouping, class A only under that law.
