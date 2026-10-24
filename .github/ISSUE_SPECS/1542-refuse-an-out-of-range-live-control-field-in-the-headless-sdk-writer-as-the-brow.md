# Refuse an out-of-range live-control field in the headless SDK writer, as the browser host does

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`).
Code anchors verified on `codex/d15-stream-e2` at `fcc5c890e` (on `main` `70804d3c2`). Found by the
stream E2 batch verifier (`docs/handoffs/decision-15-2026-10-05/verdicts/stream-e2/`, batch verdict 2, m2).
The defect was on `main` before stream E2.

## Product outcome

A live-control edit with an out-of-range field is refused with the same typed error on the headless
engine as on the browser. Today the headless writer wraps the value silently and the engine applies
an edit to a different track, lane or parameter than the caller named.

## Context

- **The defect.** `encodeLaneEdits` (`sdk/src/core/writer.ts:170-174`) writes `rack` and `channel`
  with `setUint8`, and `trackIndex`, `effectIndex` and `parameterId` with `setUint32`, with no check.
  `EngineLiveControls.submit` on the headless engine (`sdk/src/headless/engine.ts:271-273`) adds none.
  A probe on `fcc5c890e` showed: `trackIndex: 1.5` writes track 1; `channel: 1.5` writes 1;
  `channel: -1` writes 255; `parameterId: -1` writes `0xFFFFFFFF`; `rack: 256` writes 0.
- **The browser refuses each of them.** The worklet host's `validCommand`
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:265-273`) requires: `kind` in
  `COMMAND_KINDS`; `rack` an effect rack or 255; `channel` a `u32` that is at most 2 or is 255;
  `trackIndex`, `effectIndex`, `parameterId` and `smoothingSamples` each a `u32` (`validU32`, `:532`);
  `values` exactly four finite numbers.
- **The same class, already fixed for one field.** Stream E2 (#1364, batch m4) made the writer
  refuse a non-`u32` `smoothingSamples` with `MisoUsageError` through `requireU32` in
  `sdk/src/internal/ramp.ts:43-64`. The other fields were left as they were.

## Decisions frozen for this slice

- **D1. One validator.** A single SDK-internal function (in `sdk/src/internal/`, not exported by any
  package subpath) checks one `LaneEdit` against exactly the rules of `validCommand` above, with
  `requireU32` for every `u32` field. It throws `MisoUsageError` naming the field
  (`"<field> must be ..."`), before any byte is written.
- **D2. Where it runs.** `encodeLaneEdits` calls it for every edit before it allocates the record
  buffer, so a batch with one bad edit writes nothing. `LiveControlWriter.stage` calls it too, so a
  bad edit never waits in the pending map. The browser transport (`browserCommand`,
  `sdk/src/browser/live-controls.ts`) calls the same function, so both SDK paths refuse with the same
  text before the host sees the command.
- **D3. No wire change.** Every edit the browser host accepts today encodes to the same bytes.
- **D4. Acked-batch question.** The refusal happens before submission; nothing is queued or acked.
  No ack can precede a drop.

## Deliverables

1. D1 in `sdk/src/internal/` and D2's three call sites.
2. The evals below.

## Authorized paths

- `sdk/src/internal/` (the new validator), `sdk/src/core/writer.ts`, `sdk/src/browser/live-controls.ts`,
  `sdk/test/`

## Non-goals

- No change to the worklet host's `validCommand` or to the engine's decode.
- No change to the `smoothingSamples` rules of #1364.

## Objective gates

1. **Refusal table.** For each field, an eval submits through `encodeLaneEdits`, through
   `LiveControlWriter.stage` and through the browser transport one value of each refused class:
   a fraction, a negative, `2^32` (or 256 for `rack` and `channel`), `NaN`, and for `channel` the value
   3. Each throws `MisoUsageError` with the same text on every path and writes no bytes. For `values`:
   three entries, five entries and one `NaN`.
2. **Acceptance unchanged.** For every kind, a valid edit at each field's bounds (0, the maximum
   `u32`, `rack` and `channel` 255) encodes to the same bytes as on the base commit.
3. **Atomic batch.** A batch of three edits whose second is invalid writes no bytes.
4. Commands: `bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-headless.sh <artifacts>`,
   `bash scripts/check-sdk-generated.sh <artifacts>`, `python3 -B scripts/check-sdk-deletions.py`,
   `bash scripts/sdk-package.sh check <artifacts>` (the tarball smoke also refuses the new validator
   name on every subpath).

## Test value

- Gate 1 turns red if any field is written unchecked, so the headless engine applies an edit to a
  track, lane or parameter the caller did not name, which the browser would have refused.
- Gate 2 turns red if the validator refuses a value the browser host accepts.
- Gate 3 turns red if the writer validates while it writes, so a batch commits its first edit and
  then throws.

## Dependencies

- *Resolve an absent live ramp to the session default on the browser and in the SDK* (#1364), on
  `main` after stream E2 merges (`requireU32`, the internal-module pattern).

**Decision 17 amendment (2026-10-13).** Superseded by #1563. The SDK no longer writes 48-byte value records and checks no value domain of its own: every value edit is a `SET_CONTROL` addressed by stable ID, and the core refuses an out-of-range value with `control.domain` on the browser and on the headless engine alike (#1563 gate 2). The SDK keeps only encodability checks (a `u32` ramp, a lane token, a string ID).

**Scope narrowing (2026-10-24, scope note accepted by root, R1 = B).** Of the five original fields,
`trackIndex`, `channel`, `effectIndex` and `parameterId` no longer exist on a `LaneEdit`: #1563
superseded them (see the amendment above). The one field left unchecked was `rack`, written by
`setUint8` so `257` landed as inserts and `1.5` as `1`. Fixed here: `checkLaneEdit`
(`sdk/src/core/writer.ts`) accepts only the rack codes the SDK spells (inserts `1`, console `3`,
from the generated `racks` table through `requireRack` in the new internal `sdk/src/internal/rack.ts`,
which now also owns `RACKS`); `encodeLaneEdits` checks every edit before it allocates; `stage` and
`browserCommand` call the same check. No wire change. Gate: the #1677 eval in
`sdk/test/writer-evals.mjs` refuses `rack` `257`, `1.5` and `0` through `encodeLaneEdits` and `257`
through `stage`.

**Core `unknownRack` coverage (root, 2026-10-24, option b).** C ABI callers can still send rack `0`,
so the core's refusal stays covered: the `unknownRack` row of `sdk/test/structure-evals.mjs` ("a key
the plan does not have is refused with its typed reason") now sends a raw record with rack byte `0`
through `engine.submitCommands`, past the SDK encoder, and still expects the core's `unknownRack`.
