# Carry the bypass ramp in the bypass record

Successor of #1341a1 (batch 9 review), decision 17. Code anchors verified on the batch 9 branch `codex/controls-batch-9`. Issue number: assigned at filing (written here as #1622). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

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

## Decision record: #1622 (batch 10)

Shipped. `EffectControlRecord::Bypass` now carries `BypassSwitch { bypassed, ramp_samples }`.
`LiveEffectRecords::bypass_ramp_samples` is deleted; the classifier builds the record with the
session's `BypassCrossfade` ramp, and the control plane writes the record as it is.

- **One write path.** `EffectCellWriter::write_bypass`, `EffectControlProducer::write_bypass` and
  `EffectControlProducer::preflight_bypass` are deleted. `EffectCellWriter::address` runs
  `BypassSwitch::check`, so `check`/`preflight`/`write` refuse a ramp over
  `INDEXED_RAMP_LENGTH_MAXIMUM` as `EffectWriteRefusal::Ramp` with nothing written, as
  `write_bypass` did. A step is `BypassSwitch::step(bool)`.
- **Departure from the filed text.** Anchors had moved (#1280a added carry code): the carry code
  reads the cell words, not the record, so it needed no change. The mirror in `symmetry.rs` is only
  `SymmetryEvent::Bypass(bool)` (a separate type, kept); `EffectControlRecord::symmetry_event` now
  reads `switch.bypassed`. Callers outside the authorized paths were updated mechanically (test
  and workload call sites that spelled `Bypass(bool)` or `write_bypass`): `capi` live_tests,
  `graph`, `graph-compiler`, `rack`, `host-core` tests, `effect-contract` tests/support,
  `host-core/src/limiter_linked_session.rs`, `effect-compiler/src/prepare.rs` and
  `tools/console-workload`. #1280c's `write_bypass` call in host-core `prepare.rs` becomes
  `write(stamp, EffectControlRecord::Bypass(switch))` at merge.
- **Gates.** `a_bypass_flip_carries_the_sessions_mute_ramp_and_nothing_else` now reads the ramp
  from the record. effect-contract, rack, host-core (all features), control-plane (test-support),
  capi, host-web (test-support), and graph-compiler `bypass_shunt_identity` + `bypass_cohorts`
  pass; clippy `-D warnings` and fmt clean. `git grep bypass_ramp_samples` is empty in code.
- **D2.** `audit capi` `pcm_digest` is `9bc5ec3d8a4c223c` both before (stash of this change, same
  tree) and after; the digest `76525e7ef9e78cf4` named in the brief does not hold on this base.
  No rendered bit moved by this change.
