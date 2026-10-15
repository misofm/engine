# Allocate an effect lane's carried slots only when a successor carries

Successor of #1280a, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`). Code
anchors verified on the batch 10 branch `codex/controls-batch-10`. Issue number: assigned at filing
(written here as TBD23). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a
batch review. Root, 2026-10-14: the first lane-K slice of batch 11, before #1282.

## Product outcome

An effect lane holds no carry storage until a plan swap carries it, so a first boot pays nothing for
the carry: the browser's boot growth regains its margin under the 20 MiB pin, and the C ABI retains
less.

## Context

- #1280a gave every effect lane preallocated carried slots (`carried_targets:
  Box<[CarriedCell<PREPARED_EFFECT_TARGET_WORDS>]>` and `carried_parameters`, built in
  `effect_control_lane`, `crates/effect-contract/src/live.rs`). On the 1 MiB worst browser document
  that is 672 B x 768 EQ lanes = 516 KB, plus lane-struct growth; boot growth went from 20,447,232
  to 20,971,520 bytes (exactly at the pin after batch 10's #1281 fix).
- The slots serve only a successor that carries: `EffectControlLane::carry_from` fills them and
  `stage` resolves them in the successor's first block.
- `graph::install_effect_carry` already allocates the carry program's storage (`carry_scratch`) on
  the control thread for the lanes in `moves`, charged through `effect_carry_retained_bytes`.

## Decisions frozen for this slice

- **D1.** The carried slots are allocated in `install_effect_carry`, on the control thread, for the
  successor lanes in `moves` only, and charged through `effect_carry_retained_bytes`. A lane that no
  successor carries holds none.
- **D2.** `EffectControlLane::carry_from` reads a missing predecessor slot as empty, and
  `can_carry_from` refuses a successor lane whose slots were not installed, so a carry can never
  write into storage that does not exist.
- **D3.** The swap block stays allocation-, lock- and syscall-free; render reads no new state.

## Objective gates

1. Boot budget: `node scripts/check-web-boot-budget.mjs` growth measured below the 20 MiB pin, with
   the margin recorded in the decision record (batch 10: 20,971,520, zero margin). The multiplier
   does not change.
2. `capi_retained_bytes` is tightened to the new measurement (measured + 10 %, rounded up to 64), at
   both lane widths from the CI print.
3. Every carry gate of #1280a (and #1280b/c, #1281) passes unchanged, including zero allocations and
   frees on the swap block; the charge-equals-freed resource tests hold.
4. `cargo test --locked -p effect-contract -p graph -p graph-compiler -p rack -p host-core
   --all-features -p capi -p host-web --features host-web/test-support`; clippy; realtime-policy.

## Authorized paths

- `crates/effect-contract/src/live.rs`, `crates/graph/src/{lib.rs,runtime.rs}`,
  `crates/graph-compiler/src/estimate.rs`, their tests, `crates/capi/tests/resource_lifecycle.rs`
  (the ceilings), `hosts/host-web/tests/browser-v1/expected.json` (measured memory rows only), this
  spec's decision record.

## Dependencies

- #1280a. Before #1282.

## Size

Half a day to a day.
