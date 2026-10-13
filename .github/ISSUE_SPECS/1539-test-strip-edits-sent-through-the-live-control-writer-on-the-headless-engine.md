# Test strip edits sent through the live-control writer on the headless engine

Stream H of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). Successor to the
known gap (2) that *Hold strip input-lane values in latest-target cells* (#1346) and *Hold effect
parameter, bypass and EQ-target values in latest-target cells* (#1345) recorded
(`.github/ISSUE_SPECS/1346-*.md:477-481`, `1345-*.md:1226-1230`), from stream B batch 3 verdict 2
(NIT) and verdict 3 (NIT). Anchors verified on `codex/d15-stream-b3` at `711847ce3`. Re-verify
every anchor before you start.


## Product outcome

An app that moves a fader stages each position into the SDK's `LiveControlWriter` and flushes it
to the engine. Two things must hold on the shipped module: the block renders the last position of
each lane, exactly as an engine that was sent only those values, and the status's
`liveValuesSuperseded` count is the number of values that a later flush replaced before the block
read them. Today no eval sends a strip edit through the writer to an engine, so a writer defect
that loses or repeats a strip edit, or a change in how the module counts them, is not seen at the
SDK boundary. After this issue one eval proves both on the headless engine, which runs the same
`simd128` module as the browser.

## Context

- **The gap.** Commit `e975e2eb8` deleted the "playing" writer eval, the last one that flushed
  `faderEdit` records into an engine. The only use of `faderEdit` left
  (`sdk/test/writer-evals.mjs:99-110`) is the engine-free coalescing test (`:336-347`), which never
  submits.
- **The writer** (`sdk/src/core/writer.ts`). `stage` (`:220-229`) keeps one pending edit for each
  `coalescingKey` (`:144-149`: kind, track, rack, channel, effect index, parameter id); a newer
  edit replaces the older one and counts in `stats.coalesced`. `flush` (`:259-265`, `#flushOnce`
  `:267-332`) submits up to `maximumBatch` pending edits in one call. With
  `submit: (records, count) => engine.submitCommands(records, count)` the edits go through
  `encodeLaneEdits` (`:151-169`).
- **The headless engine** (`sdk/src/headless/engine.ts`). `createOfflineEngine` boots the module
  that `scripts/check-sdk-headless.sh` builds (`sdk/test/support.mjs:24-40`). `submitCommands`
  (`:328-330`), `status()` (`:201-203`), `liveControls()` with the edit builder (`:271-277`).
- **The edit builder.** `engine.liveControls().edit.track("t")` returns `faderDb(db, { channel })`
  and `mute(enabled, { channel })` `LaneEdit`s (`sdk/src/core/live-controls.ts:579-593`). An app
  stages these into the writer.
- **The cells and the count.** A fader record writes the left cell, the right cell, or both for
  `channel: "both"` (`crates/builtins-compiler/src/lib.rs:303-336`). Each write that replaces a
  value no block has read adds one to `liveValuesSuperseded` when the block reads the cell. The
  contract text: `sdk/src/browser/shipped-host.d.ts:890-897`.
- **Evals that exist and what they do not cover.**
  - `writer-evals.mjs:157` "a paused host takes any number of effect edits ...": effect edits only,
    each address staged once per flush; no count.
  - `capability-evals.mjs:162-196`: two left-lane `faderDb` submits through `liveControls.submit`,
    not the writer; it reads the count (`1n`) but compares no PCM.
  - `hosts/host-web/src/tests.rs:3305` (Rust): 40 both-lane fader submits, count 78; no SDK, no
    writer.
- **Helpers in `writer-evals.mjs`.** `pausedEngine()` (`:118-123`: one track `t` with one EQ
  console slot, live controls on), `feedAndRender(engine)` (`:140-154`: feeds one block of ramp PCM
  and renders it). The session's faders start at 0 dB and its pan is hard left and hard right
  (`sdk/test/support.mjs:55-105`), so a left fader changes only the left plane.

## Decisions frozen

- **D1. Real engine, real writer.** Every engine is `pausedEngine()`; every edit comes from that
  engine's `liveControls().edit.track("t")`; the writer is
  `new LiveControlWriter({ submit: (records, count) => engine.submitCommands(records, count), maximumBatch: 8 })`
  unless a case says otherwise. No fake submit.
- **D2. Three engines per case.** `heard` gets the case's edits through the writer; `twin` gets only
  the final value of each lane through its own writer (stage, `drain()`); `untouched` gets no edit.
  Each renders exactly one block with `feedAndRender` after its edits.
- **D3. PCM compare.** Compare float bits: `new Uint32Array(Float32Array.from(plane).buffer)` for
  each of `left` and `right`, with `assert.deepEqual`.
- **D4. Values.** Left fader `-3, -6, -12` dB, right fader `-2, -4, -9` dB, both-lane fader
  `-1, -2, -3, -7` dB. Each value changes the rendered plane from `untouched`.
- **D5. Expected counts** (each derived from the cell rule in "Context"; write the derivation in a
  comment next to each assertion): case 1, `4n` (two replaced writes in the left cell, two in the
  right); case 2, `0n` (the writer coalesced; one write per cell); case 3, `6n` (three replaced
  writes in each of the two cells a both-lane record writes); case 4, `0n`. `twin` and `untouched`
  read `0n`. Every engine reads `renderedQuanta === 1n`.
- **D6. File.** One new `describe("the writer contract -- strip edits on the headless engine")` in
  `sdk/test/writer-evals.mjs`, after "the writer contract -- paused". No new file, no change to the
  existing helpers.

## Deliverables

1. The `describe` of D6, with the four cases of gate 1.

## Authorized paths

- `sdk/test/writer-evals.mjs`: the new `describe` only.
- This issue's own spec file.

## Non-goals

- Any change to `sdk/src/` (`writer.ts`, `live-controls.ts`, `headless/engine.ts`) or to the
  module. A defect that this eval finds goes to a new issue; this issue does not fix it.
- The browser AudioWorklet path (the writer over the worklet port). The headless engine runs the
  same module; a real-browser twin is a separate issue if root wants one.
- Pan, matrix, input-section, send and effect edits.

## Objective gates

1. **The eval.** Four cases (four `test`s in the D6 `describe`):
   1. *Separate flushes.* For `k` in `0..3`, stage `faderDb(left[k], { channel: "left" })` and
      `faderDb(right[k], { channel: "right" })` on `heard`'s writer and flush; each outcome has
      `refused === false` and `admitted === 2`. Render the three engines. Assert: `heard` equals
      `twin` (D3); `heard.left` differs from `untouched.left` and `heard.right` from
      `untouched.right` in at least one sample each; `heard`'s count is `4n` (D5); writer stats
      `admitted === 6`, `coalesced === 0`, `refusals === 0`, `escalations === 0`, `pending === 0`.
      Run this case twice: once with the D1 submit and once with
      `submit: async (records, count) => engine.submitCommands(records, count)`, on fresh engines;
      both runs pass the same assertions.
   2. *One flush.* Stage all three left and all three right values, then flush once: `admitted === 2`,
      `stats.coalesced === 4`; `heard` equals `twin`; `heard`'s count is `0n`.
   3. *Both lanes.* For `k` in `0..4`, stage `faderDb(both[k], { channel: "both" })` and flush.
      `heard` equals a `twin` sent only `-7` on both lanes; `heard`'s count is `6n`.
   4. *Two kinds on one lane.* Stage `faderDb(-12, { channel: "left" })` then
      `mute(false, { channel: "left" })` and flush once: `admitted === 2`, `coalesced === 0`.
      `heard` equals a `twin` that received the same two edits through
      `twin.liveControls().submit(fader, mute)` (not through a writer); `heard.left` differs from
      `untouched.left`; `heard`'s count is `0n`.
   - *Test value:* red if strip edits that an app stages into the `LiveControlWriter` do not reach
     the shipped module as exactly the last value of each lane, once per flush: the writer keeps an
     older pending value, folds two kinds on one lane into one, submits a batch twice, or encodes a
     lane wrongly. Each shows as a block that differs from the twin or as a
     `liveValuesSuperseded` count other than the cell writes the later flushes replaced. No
     existing eval sends a strip edit through the writer to an engine.
   - *Mutations (PR evidence; apply each to `sdk/src/core/writer.ts` in the working tree only, run
     the whole `node --test 'test/*-evals.mjs'`, record every eval that goes red, then revert):*
     M1, `stage` keeps the older pending edit for a key that is present (it still counts the
     coalesce): red at case 2. M2, `coalescingKey` without `edit.kind`: red at case 4. M3,
     `#flushOnce` calls `this.#submit(edits)` twice and uses the second report: red at case 1 (the
     count is `10n`, the PCM still matches). M4, `encodeLaneEdits` writes channel `2` for every
     record: red at case 1. For each mutation, the attempt record says whether any other eval also
     goes red; a mutation that only this eval catches is the unique catch.
2. **SDK job.** The same commands as `qualification.yml:276-280`, with `A` an empty directory:
   - `bash scripts/build-web-audioworklet.sh A`;
   - `bash scripts/check-sdk-generated.sh A`;
   - `python3 -B scripts/check-sdk-deletions.py`;
   - `bash scripts/check-sdk-types.sh` (after `npm ci` in `sdk/`);
   - `bash scripts/check-sdk-headless.sh A` (runs `node --test 'test/*-evals.mjs'`, `:91`);
   - `bash scripts/sdk-package.sh check A`.
   - One case alone: `cd sdk && MISO_ENGINE_SDK_ARTIFACTS_HEX=$(printf %s "$A" | od -An -v -tx1 | tr -d '[:space:]') node --test --test-name-pattern "strip edits on the headless engine" test/writer-evals.mjs`.
3. **Workspace.** `bash scripts/check-workspace-policy.sh`.

## Dependencies

- #1345 and #1346 on `main` (stream B batch 3): strip and effect values in cells, and the
  `liveValuesSuperseded` status word (#1399, on `main`).
- None in stream H.

## Placement (root edits `STREAMS.md`)

- Stream H owns `sdk/`. A new row after #1536: dependency "B #1345, #1346 on `main`; before H #1562"
  (early). #1562 moves browser live-edit admission into the Worker; this eval then guards the
  headless path while #1562 changes the admission code.
- `sdk/test/writer-evals.mjs` has no hot-file row. B #1524 edits it (`commandQueueRecords` in
  `pausedEngine` and `observedEngine`, and the default `maximumBatch`; `1524-*.md:121`, `:160-161`).
  Add a row: "`sdk/test/writer-evals.mjs` | B #1524 (the live-control option and the default batch)
  and H #NNNN (one new `describe`) in any order, the later slice rebases". The new cases build
  engines only through `pausedEngine()` and pass `maximumBatch: 8`, so #1524's rename touches none
  of their lines.
- No change to the `sdk/src/core/live-controls.ts` row (`STREAMS.md:103`): this issue does not edit
  that file.

## Attempt record

- Attempt 1: not started.

## Root rulings (2026-10-13)

- **R1. Stream H**, after #1536, before H #1562. The `writer-evals.mjs` hot-file row is
  accepted (B #1524 in any order; the later slice rebases).
- **R2. No real-browser twin** in this slice (non-goal stands).
- **R3. Test value.** The attempt record runs M1-M4 and names, for each, which existing evals also
  go red; a mutation with no unique catch is removed or its case is changed until it has one.

**Decision 17 amendment (2026-10-13).** Superseded by #1563. The 48-byte writer for values is replaced by a per-engine `SET_CONTROL` transaction queue that coalesces by address; #1563 gate 4 tests its coalescing and order, and gate 3 tests that each edit renders as an engine booted from the committed document.
