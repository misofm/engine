# Build the effect control owner's candidate at begin

Successor of #1611 (its decision record), decision 17. Code anchors verified on the batch 9 branch `codex/controls-batch-9`. Issue number: assigned at filing (written here as #1624). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

An effect control owner (the EQ's target owner) keeps no candidate copy and dirty array between
transactions, so the browser's boot and retained memory shrink by about 1.15 MB on the 1 MiB worst
document.

## Context

- `EffectControlOwner` keeps a `candidate` copy and a dirty array per EQ, about 780 bytes each,
  about 600 KB to 1.15 MB on the worst document (#1611's decision record).
- Both are used only while a transaction stages a value.

## Decisions to take (the implementer proposes, the coordinator rules)

- **Option A.** Build the candidate and the dirty array in `begin` from one control-thread scratch
  the control plane owns (one per session, sized to the largest owner), and drop them at commit.
- **Option B.** Keep them per owner but allocate on first use and charge them.
- Recommendation: A. Render is not affected; the owner is control-thread state.

## Objective gates

1. Boot growth (`scripts/check-web-boot-budget.mjs`) and native retained bytes fall by the
   measured amount, recorded; the multiplier 20 holds.
2. The EQ live-edit tests (C ABI and browser parity) pass unchanged.
3. `cargo test --locked -p effect-contract -p control-plane --features test-support -p capi -p
   host-web --features host-web/test-support`.

## Authorized paths

- `crates/effect-contract/src/live.rs` (the owner), `crates/control-plane/src/control.rs`, their
  tests, resource ceilings with measured numbers, this spec's record.

## Dependencies

- #1611.

## Size

Half a day.
