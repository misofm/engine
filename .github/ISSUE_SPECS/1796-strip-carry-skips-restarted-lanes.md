# #1796 Test that the strip carry does not drain the lanes a swap restarts

Found by the #1538 verifier (2026-10-24, mutant M2).

## Problem

No host-core test goes red when the strip carry drains lanes that the swap restarts. Mutant: make
`carried_strip_lane_mask` (`crates/graph/src/runtime.rs`) return `u8::MAX`. Then the carry drain
also drains a restarted lane's pending writes and counts them in `live_values_superseded`. #1312 D2
forbids this. Every strip-carry superseded test in `crates/host-core/tests/successor_swap.rs`
asserts an empty restart set; the only restart-plus-count test covers the input carry
(`carried_input_lane_mask`), so the mutant survives.

## Smallest closable slice

One more run, and one assertion, in the existing `a_carried_lane_counts_its_replaced_writes`
(no new test fn). The run commits a base without `MOVED` (`eq0`), so `MOVED`'s stage has no join
and restarts while `CARRIED` (`eq1`), its bank-mate, carries. Both get pending writes at the swap
block. Assert its `carry_superseded` equals the `CARRIED`-only run's count.

Authorized paths: `crates/host-core/tests/successor_swap.rs` (that one test fn and its doc
comment) and this spec. No production code; `crates/graph` untouched.

## Gates

1. `cargo test -p host-core --features test-support --test successor_swap a_carried_lane_counts_its_replaced_writes` green on main at both widths.
2. The `u8::MAX` mask mutant turns it red (PR evidence).
3. Test value: red on a strip carry drain that counts a non-carried lane's replaced writes; no
   existing strip-carry test has a non-carried lane in a drained bank.

## Decision record

- D1 (root, from the scope note): fold into the existing test, not a new fn (one test per claim).
- D2: the no-join shape (`without_track(strip_state_session(), MOVED)` as the committed base) was
  used; the scope note's fallback (a prepared fader difference) was not needed. A dry run showed
  `restarted_strips()` is empty for this run (no join, so no listed restart), so the run asserts
  only `CarryOutcome::Carried` and the count, not `restarted`. The other three runs keep their
  empty-restart assert.

## Evidence

- Gate 1: green at Simd4 and Simd8 (x86-64-v3 host); the new run counts 6 (= the `CARRIED` run).
- Gate 2: with `carried_strip_lane_mask` returning `u8::MAX`, red at both widths: `left: 10,
  right: 6` (the drain also consumes `MOVED`'s three fader writes: two cells, two replaced each).
  Restored with `git checkout` + `touch`.
