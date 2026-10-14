# Shrink the always-attached per-effect live lanes

Successor of #1381a and #1555, decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C6: the core attaches live lanes on every plan). Code anchors verified on `846b7d240` (the flip branch, #1381a).
Issue number: assigned at filing (written here as #1611). Feature sprint mode (owner,
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

## Amendment (controls coordinator, batch 9, 2026-10-14)

Checked against `main` at `7feb27800`.

- **Anchors held** (`prepare.rs:1727`; `live.rs:164`, `:349`, `:568`, `:623`, `:695`, `:818`;
  `rack/src/lib.rs:1151`; `resource_lifecycle.rs:1317-1322`, 216,832).
- **The multiplier is in three places, and they move together.** `scripts/check-web-boot-budget.mjs:9`
  (25); `PARSE_TRANSIENT_MULTIPLIER` in `hosts/host-web/src/lib.rs:94` (25), the host's typed
  pre-parse refusal `host.budget.parse_projection`, which the script's second block checks at the pin
  minus one; and `hosts/host-web/src/tests.rs:195`, which pins it. The two host-web lines are added
  authorized paths.
- **The figure has not moved since the flip.** A release browser module built after batch 7
  (`codex/controls-batch-8w`, 2026-10-14 12:56 UTC) grows 25,427,968 bytes (388 pages, 24.25 times
  the document) on the 1 MiB worst document. That equals the flip-branch figure: #1599, #1600, #1612
  and #1613 did not change it. The 20 pin plus one page is 21,037,056 bytes (321 pages), so this slice
  must remove at least 4,390,912 bytes (67 pages) of high-water growth to reach D5's first branch.
- **#1341a1 and #1341a2 change this slice's files** (batch 9): `effect-contract/src/live.rs`
  (`BypassSwitch` in a `LatestCell<2>`, the crossfade state on `EffectControlLane`) and the bank stage
  in `rack/src/lib.rs` (`BankCrossfade`, inline arrays, no heap). They are a dependency. Measure every
  "before" figure on `main` after both merge, not on the flip-branch figures: the D2 per-field
  breakdown (one EQ, one compressor, one delay); the boot growth with the release module
  (`scripts/build-web-audioworklet.sh`, then `check-web-boot-budget.mjs`); the native retained bytes
  of the worst document; `capi_retained_bytes`; and the one descriptive render run.
- **Edit order.** After #1341a1/a2 (`live.rs`, `rack/src/lib.rs`, `effect-compiler/src/prepare.rs`)
  and before #1566 in `effect-compiler/src/prepare.rs` (different functions). After #1602 and
  before #1574 in `hosts/host-web/src/lib.rs`.

## Decision record: lane shrink and the boot pin (batch 9, implementer)

Measured on the branch head (main `7feb27800` + #1341a1/a2 + #1602 + batch-9 docs), x86-64 eight
lanes, release module, 1 MiB worst document (192 tracks, 768 parametric EQs).

**The cause the spec did not name.** The lane layout is not what drives the wasm high-water. A
counting allocator on the native boot (every request of 200 KB or more, with its stack) found
`controls::live_lanes::live_lanes`, called by `control_plane::compile::prepare_runtime`, collecting
the whole control list (`controls::list`: 53,763 rows of 232 bytes) into one `Vec` grown by
doubling to 15,204,352 bytes, only to read three booleans (`LiveLanes::of_rows`). On wasm, dlmalloc
does not reuse a freed block above its mmap threshold, so the whole doubling chain (118,784 up to
15,204,352 bytes) counts toward the growth. Evidence: D3 alone removed 1.5 MB of retained bytes and
moved the growth by 0 pages (25,427,968 before and after); streaming `live_lanes` one owner at a
time (the way `ControlCatalog::build` already walks the list) moved it to 20,447,232.

**Boot growth (`check-web-boot-budget.mjs`, shipped simd128 module).**

| Step | Growth | Pages |
|---|---|---|
| Before (branch head) | 25,427,968 | 388 |
| D3 (bank windows) + shape (D2) only | 25,427,968 | 388 |
| + `live_lanes` streamed (final) | 20,447,232 | 312 |

The 20 pin plus one page is 21,037,056 bytes (321 pages); the final growth is 589,824 bytes (9
pages) under it. The multiplier is 20 in the three places (`scripts/check-web-boot-budget.mjs`,
`PARSE_TRANSIENT_MULTIPLIER` in `hosts/host-web/src/lib.rs` with its doc paragraph,
`hosts/host-web/src/tests.rs`).

**D2 measurement (per-field `retained_resources` and allocation sites).** One parametric EQ lane
(target owner, 6 slots = 12 target cells): shared allocation 192; target cells 12 x 192 = 2,304
(`LatestCell<12>`, three slots of 64 bytes); dirty word 8; readers 12 x 8 = 96; dirty reader 8;
pending 8; staged targets 12 x 56 = 672 (lane total 3,288); writers 12 x 32 = 384; the control owner
(`effect-compiler/src/control.rs`) holds two 720-byte parameter copies and a 60-byte dirty array
per EQ. The 2,304 + 384 + 96 + 672 are fixed by the frozen cell protocol (one cell per
`(slot, channel)`), so the EQ document gains nothing from a denser per-cell layout. One compressor:
1,900 (cells 14 x 72 = 1,008, records 14 x 8 + 7 x 8 + 7 x 4 = 196). One delay: 1,068. The shape
(`EffectLaneShape`) no longer holds a per-cell record array or a table as long as the last live
index: one 12-byte group per live parameter, found by binary search on the control thread; a
compressor lane loses 112 bytes, a delay lane 72, an EQ lane none.

**D3.** `LiveControlEffectBankStage::new` sizes `staging` and `packed` by the widest lane's
`parameter_cells()` (refused with `AutomationWindow` when above `automation_capacity`), not by the
bank's capacity. A prepared-target owner holds no span storage: per EQ bank 15,840 bytes
(staging 1,760 + packed 14,080) removed, 96 banks = 1.52 MB. The cap estimate
(`graph-compiler/src/estimate.rs`) charges the same window. Native retained after boot for the worst
document: 13,446,520 -> 11,907,448 bytes (-1,539,072); native peak 19,869,264 -> 19,002,476.

**Ceilings.** `capi_retained_bytes` is 197,521 before and after: no re-pin (the spec's D5 bullet
assumed it moves). The three graph rows of the reference session moved at eight lanes, so
`resource_lifecycle.rs` re-pins them to measured + 10 % rounded up to 64: `graph_session_plus_plan_bytes`
407,294 -> 314,918 (ceiling 421,248 -> 346,432), `graph_incremental_plan_bytes` 407,222 -> 314,846
(421,248 -> 346,368), `graph_metadata_bytes` 199,049 -> 106,673 (203,648 -> 117,376). The four-lane
ceilings stand until CI prints the AArch64 values. `retained_ceilings.rs` (browser fixture) did not
move; no edit.

**Gates.** `cargo test --locked` (all features) on effect-contract, effect-compiler, rack,
graph-compiler (including `bypass_shunt_identity`, `bypass_cohorts`, `bypass_resources`), capi,
controls, host-core, control-plane, graph and host-web: green. New test
`a_live_slot_sizes_its_spans_by_its_lanes_cells_not_the_capacity` (rack, red on a capacity-sized
window: a multi-gigabyte allocation). `effect_control_resource_uses_independent_queue_and_owner_arithmetic`
(graph-compiler) now computes the bank window from the lanes' cells, and takes its
"largest allocation outgrows the plain plan" witness from the per-node owner (`scalar-target`,
window of the 4,096-span capacity), because a bank's window is no longer capacity-sized.
`check-web-audioworklet.sh`, `check-effect-runtime-fixtures.sh`, `check-console-fixtures.sh`,
`check-cross-targets.sh`, `realtime-policy`, clippy `-D warnings` on the touched crates and
`check-sdk-headless.sh` (527 tests): pass. The #1341a2 bank-equals-node tests are unchanged and green.

**Per-block cost (descriptive, one warmup of 300 blocks, two rounds of 2,000).** The worst document
natively, `render_next`: 103.71 and 103.72 us/block before, 105.49 and 105.81 after (+1.8 %, no
render code changed; the stage reads a shorter window array). Not a gate.

**Departures and edits outside the authorized paths (all listed for root).**
- `crates/controls/src/live_lanes.rs` (the cause above; behaviour identical, booleans only).
- `crates/graph-compiler/src/estimate.rs` and the arithmetic test in `crates/graph-compiler/src/lib.rs`
  (mirror of D3, required by the charge-equals-freed tests).
- `hosts/host-web/src/lib.rs` and `tests.rs` (the multiplier, as the amendment authorizes).

**Successors found.**
- `EffectBankProcessBlock::check_automation_window` has no production caller now (the rack was its
  last); delete it with its row in `tools/console-workload/tests/paired_spans.rs`.
- `graph`'s per-node `LiveControlEffect` still sizes its window by the capacity (`runtime.rs`
  ~1273) and the estimate charges it so; the same sizing by cells applies to an unbanked EQ.
- `EffectControlOwner` keeps `committed` and `candidate` copies and a dirty array per EQ (1,500
  bytes, 1.15 MB on the worst document); `candidate` and `dirty` could be built at `begin`.
- `ControlRow` is 232 bytes and `controls::list` still collects the whole list for its callers
  (tests); the production callers now walk per owner.
