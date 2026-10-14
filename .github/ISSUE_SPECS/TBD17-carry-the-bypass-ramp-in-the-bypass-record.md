# Carry the bypass ramp in the bypass record

Successor of #1341a1 (batch 9 review), decision 17. Code anchors verified on the batch 9 branch `codex/controls-batch-9`. Issue number: assigned at filing (written here as TBD17). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A bypass has one spelling from the classifier to the cell: the record carries its ramp, so no
side field travels beside it.

## Context

- `EffectControlRecord::Bypass(bool)` (`crates/effect-contract/src/live.rs:144`, mirrored in
  `symmetry.rs:144`) is the step spelling; the ramp-carrying unit is `BypassSwitch`, written by
  `write_bypass`/`preflight_bypass` (`live.rs:542`). The classifier carries the session ramp in a
  side field, `LiveEffectRecords::bypass_ramp_samples` (`crates/host-core/src/live_delta.rs:134`).
- #1341a1 kept the record's shape because the browser host matched it; since #1565A the browser has
  no bypass command, and #1393 closes with no work, so the reason is gone.

## Decisions frozen for this slice

- **D1.** `EffectControlRecord::Bypass` carries `BypassSwitch { bypassed, ramp_samples }`;
  `bypass_ramp_samples` is deleted; one write path remains.
- **D2.** No rendered bit moves (the #1341 crossfade tests and `audit capi` pcm digest unchanged).

## Objective gates

1. Every #1341a1/a2 test passes unchanged in meaning; `git grep bypass_ramp_samples` is empty in
   code.
2. `cargo test --locked -p effect-contract -p host-core --all-features -p control-plane
   --features test-support -p capi -p host-web --features host-web/test-support`; clippy.

## Authorized paths

- `crates/effect-contract/src/{live.rs,symmetry.rs}`, `crates/host-core/src/live_delta.rs`,
  `crates/control-plane/src/control.rs`, their tests, this spec's record.

## Dependencies

- #1341a1, #1341a2.

## Size

Half a day.
