# Size the per-node live effect window by its live cells

Successor of #1611 (batch 9 review), decision 17. Code anchors verified on the batch 9 branch `codex/controls-batch-9`. Issue number: assigned at filing (written here as TBD16). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

An effect that renders per node reserves span storage for its live parameter cells only, as a
banked effect does since #1611, so an unbanked EQ (no parameter cells) holds no span window.

## Context

- #1611 sized the live bank stage's window by the widest lane's `parameter_cells()`
  (`crates/rack/src/lib.rs`, `LiveControlEffectBankStage::new`) and the bank charge with it
  (`crates/graph-compiler/src/estimate.rs`).
- The per-node `LiveControlEffect` window in `crates/graph/src/runtime.rs` (near `:1225-1273`) and
  its estimate still reserve `PreparedEffectMetadata::automation_capacity` spans for any lane with
  a live channel.

## Decisions frozen for this slice

- **D1.** The per-node window holds the lane's parameter cells, the same rule as the bank stage;
  a lane with no parameter cells holds none. The estimate charges exactly that.
- **D2.** Per-lane arithmetic does not change; bank equals node stays bit-exact (#1341a2 tests).

## Objective gates

1. A graph test shows an unbanked EQ's per-node window is empty and a compressor's equals its
   cells; it fails if the window returns to the capacity (an assertion, not an allocation abort).
2. The charge-equals-freed tests and `bypass_shunt_identity`/`bypass_cohorts` stay green;
   affected resource ceilings are re-pinned only from measured numbers, recorded.
3. `cargo test --locked -p graph -p graph-compiler -p rack -p capi`; clippy `-D warnings`.

## Authorized paths

- `crates/graph/src/runtime.rs` (the per-node window), `crates/graph-compiler/src/estimate.rs`, their
  tests, `crates/capi/tests/resource_lifecycle.rs` (affected ceilings), this spec's record.

## Dependencies

- #1611.

## Size

Half a day.
