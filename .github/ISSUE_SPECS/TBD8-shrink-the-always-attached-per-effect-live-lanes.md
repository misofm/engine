# Shrink the always-attached per-effect live lanes

Successor of #1381a and #1555, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C6: the core attaches live lanes on every plan). Code anchors verified on `846b7d240` (the flip branch, #1381a).
Issue number: assigned at filing (written here as #TBD8). Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review. The realtime gates below are mandatory
all the same.

## Product outcome

The live lanes that C6 attaches on every plan cost much less memory, so the browser's boot budget
returns to its 20 MiB pin (multiplier 20, 20,971,520 bytes for the 1 MiB worst document) and the
C ABI retains less. Every plan still carries its lanes, every live value still works, and render
stays allocation-free with no higher per-block cost.

## Context

- **The measurement (flip branch, root, 2026-10-14).** On the 1 MiB worst browser document
  (`hosts/host-web/examples/worst_boot_document.rs`: 192 tracks, 768 effects), attaching the C6
  lanes in the browser raised the wasm high-water growth from 16,121,856 to 25,427,968 bytes and
  native retained memory from 6,450,366 to 13,449,640 bytes. The growth is:
  - per-effect live lanes from `attach_effect_live_controls`, about 4.0 MB: 768 effects x about
    5.2 KB, in blocks of 2,304 + 720 + 720 + 672 + 384 + 192 + 96 + 96 + 60 bytes;
  - render-side bank live lanes, about 2.4 MB: 14,080, 1,760 or 1,280 bytes per bank effect x 96,
    plus 4,096 bytes per strip x 192, twice;
  - the control plane's fixed rows, about 0.4 MB; per-strip input lanes, about 0.17 MB.
- **The pin.** `scripts/check-web-boot-budget.mjs` pins growth to the document length times
  `PARSE_TRANSIENT_MULTIPLIER` plus one page (`:9`, `:53-58`). Root ruled the multiplier from 20
  to 25 for now.
- **The per-effect lane.** `attach_effect_live_controls`
  (`crates/effect-compiler/src/prepare.rs:1727-1800`) builds one lane per prepared effect with
  `effect_control_lane` (`crates/effect-contract/src/live.rs:623-692`). The lane's shape is
  `EffectLaneShape` (`live.rs:164-268`: cells, groups and a `first` table as long as the last live
  parameter index). The shared `EffectCells` (`live.rs:349-373`) hold one `LatestCell<1>` per
  parameter cell, one `LatestCell<PREPARED_EFFECT_TARGET_WORDS>` per target cell and the dirty
  words. The writer holds one `CellWriter` per cell; the lane (`EffectControlLane`, `live.rs:695`)
  holds one `CellReader` per cell, one `DirtyReader` and one `pending` word per 64 cells, and one
  `PreparedEffectTarget` per target cell. `retained_resources` (`live.rs:568`, `:818`) charges them.
- **The bank lanes.** `LiveControlEffectBankStage` (`crates/rack/src/lib.rs:1151-1185`) holds a
  lane per bank lane, a `staging` window of the bank's `automation_capacity` spans and a `packed`
  array, all sized at bind.
- **The C ABI ceiling.** `capi_retained_bytes` is 216,832 at both widths
  (`crates/capi/tests/resource_lifecycle.rs:1317-1322`).

## Decisions frozen for this slice

- **D1. C6 stands.** Every plan carries a lane for every live value, as today. No lane becomes
  lazy at commit time and no lane is built on the render thread.
- **D2. The 2,304-byte block first.** The implementer names the owner of each per-effect block
  from a measurement (a per-field `retained_resources` breakdown on one EQ, one compressor and
  one delay) before changing anything, and records it. Then the per-effect layout is sized from
  the effect's own descriptor: its live parameter count and policy, its target slots and its
  observation taps, with no array sized by a crate-wide maximum or by the last parameter index
  where a dense map serves. Arrays that the writer and the reader each hold per cell may be
  merged or made dense when the cell protocol is unchanged.
- **D3. Bank lanes sized by need.** A bank stage's `staging` and `packed` arrays and the per-strip
  arrays are sized at bind from what the bank's lanes can actually stage (their live cells and
  targets), not from the bank effect's full `automation_capacity`, where the two differ. Sizing
  stays at bind (control thread); render reads fixed lengths.
- **D4. Same behaviour.** Every live value, refusal, superseded count, observation and rendered
  bit is unchanged. Lane memory may change only in size and layout.
- **D5. The pins follow the measurement.** If the growth reaches 20 x the document length plus
  one page, the multiplier returns to 20. Otherwise it is set to the new measured growth divided
  by the document length, rounded up to an integer, and the shortfall is recorded with its named
  cause. `capi_retained_bytes` moves to the new measured value plus 10 %, rounded up to 64, with
  the file's before-and-after doc paragraph.

## Deliverables

1. The D2 measurement and D2's layout change in `crates/effect-contract/src/live.rs` and
   `crates/effect-compiler/src/prepare.rs`.
2. D3 in `crates/rack/src/lib.rs`.
3. The new pins (D5) and their records.
4. The before and after figures and one descriptive render measurement in this spec's decision
   record.

## Authorized paths

- `crates/effect-contract/src/live.rs`, `crates/effect-compiler/src/prepare.rs`
  (`attach_effect_live_controls` and its helpers only), `crates/rack/src/lib.rs` (the live bank
  stage only).
- `scripts/check-web-boot-budget.mjs` (the multiplier only).
- `crates/capi/tests/resource_lifecycle.rs` and `hosts/host-web/tests/retained_ceilings.rs` (the
  affected ceilings and their records only).
- This spec's decision record.

## Non-goals

- No change to C6, to which plans carry lanes or to the control interface.
- No change to the cell protocol (`LatestCell`, the dirty words, the superseded count).
- No change to the control catalog or the control plane's fixed rows (a separate successor if
  they need one).
- No optimisation of render arithmetic.

## Objective gates

Commands: `cargo test --locked -p effect-contract`, `cargo test --locked -p effect-compiler`,
`cargo test --locked -p rack`, `cargo test --locked -p capi`, `cargo test --locked -p host-web`,
`node scripts/check-web-boot-budget.mjs <the release browser module>`,
`cargo build --locked --release -p realtime-policy && ./target/release/realtime_policy`.

1. **Boot budget.** `check-web-boot-budget.mjs` passes on the worst document at the D5
   multiplier. Record the growth before and after.
2. **C ABI retained.** `capi_retained_bytes` passes at its new ceiling; every other row of
   `resource_lifecycle.rs` stays within its ceiling, and the charge-equals-freed tests stay exact.
   Record native retained bytes for the worst document before and after.
3. **No realtime regression.** The existing render allocation counters stay at exactly zero, and
   `realtime_policy` passes.
4. **Same behaviour.** Every existing live-control, automation, observation and bank-parity test
   stays green unchanged.
5. **Per-block cost (descriptive).** One invocation, one warmup and two measured rounds of the
   existing reference render benchmark before and after; record both. Not a gate unless a named
   release budget is missed.

## Test value

- Gate 1 turns red if the lanes do not shrink, or grow back, on the document the browser pins.
- Gate 2 turns red if a lane array is charged twice or not at all after the layout change.
- Gate 4 turns red if a dense map loses a parameter or a bank's smaller staging window drops a
  span.

## Dependencies

- The flip batch: #1381a, #1562a, #1570b, #1563B and #1565A.

## Size

One implementer-day, one sonnet-high implementer.
