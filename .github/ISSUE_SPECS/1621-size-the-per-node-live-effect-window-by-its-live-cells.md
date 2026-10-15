# Size the per-node live effect window by its live cells

Successor of #1611 (batch 9 review), decision 17. Code anchors verified on the batch 9 branch `codex/controls-batch-9`. Issue number: assigned at filing (written here as #1621). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

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

## Decision record: #1621 (batch 14b)

**Shipped.** `LiveControlEffect::new` (`crates/graph/src/runtime.rs`) sizes its staging window by
`control.parameter_cells()`: an unbanked EQ (prepared-target owner, no parameter cell) and a
channel-less lane hold no span storage; a compressor-shaped lane holds exactly its cells. The estimate
(`effect_control_resource`, `crates/graph-compiler/src/estimate.rs`) charges the same
`parameter_cells * size_of::<PreparedAutomationSpan>()`. Per-lane arithmetic is untouched.

**Departure (one, mechanical).** The bind-time check `EffectProcessBlock::check_automation_window`
demanded `window == automation_capacity`, which the cell-sized window cannot meet. The banked stage
already replaced that equality with "the lanes' cells never exceed the capacity" (#1611 D3). The same
rule now holds per node: `check_automation_window` (in `crates/effect-contract/src/lib.rs`) accepts a
window `<= automation_capacity`; the other half of the contract, that the window holds every cell the
lane reads (no drop, #1345 D4), stays `EffectControlLane::check_window` and still runs at bind. The
pairing-rule argument is unchanged: the hazard (#1012) was a window larger than the capacity, which
could stage a twin pair across the effect's `span_index < automation_capacity` cut-off; a window no
longer than the capacity cannot. Doc comments in `effect-contract/src/lib.rs` and `live.rs` say so.
Ack-before-drop: no. A cell-sized window holds one span per cell, `check_window` refuses a shorter
one at bind, and preparation raises `automation_capacity` to at least the descriptor's cells.

**Gates and tests.**
- Gate 1: `graph` `tests::a_per_node_window_is_its_lanes_cells_not_the_capacity` (EQ-shaped lane: 0;
  compressor-shaped lane: 4 = its cells; gain fixture lane: 2; channel-less lane: 0; capacity 4,096).
  `graph-compiler` `bypass_resources::a_per_node_live_window_does_not_grow_with_the_automation_capacity`
  (real EQ, compressor and limiter inserts; allocator-measured retained bytes and the estimate are equal
  at 128 and 4,096 spans). Both are red when the window returns to the capacity (checked by mutation).
- Gate 2: `graph-compiler` `effect_control_resource_uses_independent_queue_and_owner_arithmetic` follows
  the new window formula (scalar-target window 0). Its `expected.largest > plain` assertion for the
  scalar case was removed: it held only because the capacity-sized window was the largest owner
  allocation; no per-node owner allocation outgrows the plain plan's largest any more, and the
  exact-cap accept/refuse legs and the `largest` equality stay. The charge-equals-freed test
  `a_bypass_or_live_controls_are_charged_at_least_what_they_retain`, `bypass_shunt_identity` and
  `bypass_cohorts` stay green.
- Gate 3: `cargo test --locked` for graph, graph-compiler, rack, effect-contract, effect-compiler,
  control-plane, controls, host-core (all features), host-web (all features), capi (all, incl.
  `resource_lifecycle`) green; clippy `-D warnings` and fmt clean; `realtime-policy` ok;
  `audit capi` `pcm_digest` `bf7dcedd1d92621f`; `check-browser-expected-resources.py` green.

**Measured numbers.** No pinned row or ceiling moved, so none was re-pinned. Before/after on the same
tree: `resource_lifecycle` nine-track EQ, soft-clip and browser-identity rows identical (EQ plan 218,401
observed, ceilings unchanged); the browser fixture's eight budget rows identical (for example
`graphIncrementalPlanBytes` 30,154). The one printed row that moved is the unpinned "routed submix"
plan row printed by `capi_retained_bytes_charge_every_byte_the_compile_retains`: observed 104,589 ->
98,781 and its estimate 196,180 -> 190,372 (-5,808 each, slack 91,591 unchanged; the per-node EQ/
compressor windows of that session, 128-span capacity).
