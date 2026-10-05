# Apply session transactions from the headless SDK engine

Stream H of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-3, D15-11, D15-17).
Split from *Apply session transactions from the browser SDK* (#1296), which keeps the shared core
types and the browser engine. Code anchors verified on `main` at `6fb211594`.

## Product outcome

A Node or test app edits a headless session through the same API as the browser:
`engine.apply(transaction)` resolves with `{ revision, path }` when the engine commits, and
`engine.replaceSession(session)` diffs a document against the committed model into one
transaction. `revision`, `watermark()` and `applied(revision)` behave as in the browser. A
structural edit keeps every unchanged source and node playing: the rendered blocks equal a fresh
engine booted from the committed session and fed the same PCM from frame 0.

## Context

- `OfflineEngine` (`sdk/src/headless/engine.ts:114`) calls the module's exports in-process through
  `WasmBoundary` (`sdk/src/core/boundary.ts:326`). It boots with `create` (`:143`), renders with
  `render` (`:317`), submits live records with `submitCommands` (`:321`), and switches mixes with
  `loadSession` (`:332`), a reboot on the same instance (`WasmBoundary.reboot`, `boundary.ts:391`).
  `liveControls()` (`:264`) hands `EngineLiveControls` the session map and the booted document
  (`#booted`, `:119`). `sessionMap()` is at `:205`.
- The caller drives render: nothing renders between two `render()` calls.
- The exports this slice calls, all on the control half:
  - `miso_engine_web_v1_edit_ptr`, `_apply`, `_edit_outcome_ptr`, `_committed_document_ptr` and
    `_committed_document_bytes`: *Export transaction apply and anchored seek from the browser
    engine module* (#1293) D1-D4;
  - `miso_engine_web_v1_replace`: *Diff a replacement document against the committed model and
    export replace from the browser engine module* (#1386) D3;
  - `miso_engine_web_v1_service` and `miso_engine_web_v1_watermark_read` with its record:
    *Swap and retire browser plans through the Worker's service loop* (#1381) D5.
- #1296 adds the shared types in `sdk/src/core/` (`EditOutcome`, `EditPath`, the outcome flags,
  the stale-edit `MisoUsageError`) and its D3 rebind rule: only a `rebuild` reply bumps the map
  epoch; `live` and `model_only` replies and live edits advance `revision` only.
- `SessionTransaction` and its builder: *Build and encode session transactions in the SDK* (#1383)
  and *Encode the session, submix, output, route, automation and VCA edits in the SDK* (#1385).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332) defines
  `single` mode: one instance that holds both halves and runs control work outside `process()`.
  The headless engine is that shape, with the caller's thread in place of the message handler.
- SDK checks: `bash scripts/check-sdk-headless.sh <artifacts>` runs `node --test
  'test/*-evals.mjs'` in `sdk/` against the real module (`scripts/check-sdk-headless.sh:91`).

## Decisions frozen for this slice

- **D1. API.** `OfflineEngine` gains the browser engine's members with #1296's types:
  `apply(transaction)`, `replaceSession(session)`, `revision`, `watermark()` and
  `applied(revision)`. `apply` and `replaceSession` return promises for symmetry with the browser,
  but the work is synchronous: the promise is settled before the call returns control to the
  event loop. A refusal rejects with a `MisoEngineError` carrying the result and diagnostic, and
  nothing is committed.
- **D2. Boundary.** `WasmBoundary` gains one method per export of the context: `applyTransaction`,
  `replace`, `committedDocument`, `service` and `readWatermark`. Each stages through the edit
  buffer, reads the fixed records, and maps results as the existing methods do.
- **D3. Service and completion.** The headless engine calls `service()` after each control call
  (`apply`, `replaceSession`, `submitCommands`, `submitSource`, `seekSource`) and after each
  `render()` returns, never inside the render export. After each `render()` it reads the watermark
  and settles every pending `applied` it covers. `applied` stays pending while the caller renders
  nothing: completion counts render samples (D15-17).
- **D4. Rebinding.** #1296 D3 applies unchanged. After a `rebuild` reply the engine re-reads the
  session map and the committed document; `liveControls()` and `withSession` bind to the committed
  document, not `#booted`.
- **D5. loadSession.** `loadSession` stays the reboot verb. It rejects every pending `applied` with
  a `MisoUsageError` naming the reboot, resets `revision` to 0 and bumps the map epoch, so an edit
  object resolved before it is stale.
- **D6. Documentation.** The SDK README's "Render offline" section gains "Edit a headless
  session": `apply`, the three paths, that `applied` needs `render()` calls, and the difference
  between `replaceSession` (an edit, sources keep playing) and `loadSession` (a reboot).

## Deliverables

1. D1, D3-D5 in `sdk/src/headless/engine.ts` and its exports in `sdk/src/headless/index.ts`.
2. D2 in `sdk/src/core/boundary.ts`.
3. D6 in `sdk/README.md`.
4. `sdk/test/headless-apply-evals.mjs` (new).

## Authorized paths

- `sdk/src/headless/engine.ts`, `sdk/src/headless/index.ts`, `sdk/src/core/boundary.ts` (D2
  only), `sdk/test/headless-apply-evals.mjs` (new), `sdk/README.md`

## Non-goals

- No browser engine change and no core type change (#1296). No encoder (#1383, #1385).
- No Rust, export or worklet change. No new export.

## Hazards

- **The instance shape.** #1332 names the browser's two modes, not the headless engine. If the
  module's boot export no longer gives one instance both halves, gate 1 fails at the first
  `apply` with `RESULT_REFUSED_LIFECYCLE` (#1293 D1). Report that to root; do not add a boot
  variant here.

## Objective gates

All behavioural gates run in `sdk/test/headless-apply-evals.mjs` against the real module.

1. **Structural edit, real bits.** A four-track session renders 6 blocks. `replaceSession` adds a
   fifth muted track on an existing source with an EQ insert, and resolves with path `rebuild` and
   revision +1. The engine renders until `applied` resolves, then 6 more blocks. Every block
   equals a fresh `OfflineEngine` booted from the new session and fed the same PCM from frame 0.
2. **Live path and watermark.** `apply` of a fader-only transaction resolves with path `live`.
   `applied(revision)` resolves after the next `render()` with outcome `exact` and that block's
   first sample. That block already differs from a run without the call: the ramp toward the new
   value has begun.
3. **Refusal.** A transaction that changes the sample rate rejects with the engine's typed result.
   `revision` is unchanged, and the next 4 blocks are bit-identical to a run without the call.
4. **Stale edits.** After gate 1's `rebuild`, an edit object resolved before it rejects as stale
   and submits nothing; `edit.track(id)` on the added track succeeds. After gate 2's `live` reply,
   an edit object resolved before it still submits.
5. **loadSession.** A pending `applied` rejects at `loadSession`, and `revision` reads 0 after it.
6. Commands, with an artifact directory built as in `qualification.yml`'s `artifact` job and
   `npm ci` in `sdk/`: `bash scripts/check-sdk-generated.sh <artifacts>`,
   `python3 -B scripts/check-sdk-deletions.py`, `bash scripts/check-sdk-types.sh`,
   `bash scripts/check-sdk-headless.sh <artifacts>`, `bash scripts/sdk-package.sh check <artifacts>`.

## Test value

- Gate 1: turns red if `replaceSession` reboots instead of applying (the stem restarts at frame 0
  of a fresh ring at the swap), or if the successor restarts any unchanged node's state.
- Gate 2: turns red if `applied` resolves at the commit instead of the watermark, if the headless
  engine never services or reads the watermark (it never resolves), or if the live value starts
  later than the first block after the call.
- Gate 3: turns red if a refusal advances `revision` or touches the running plan.
- Gate 4: turns red if the headless engine does not rebind after a rebuild (an edit lands on a
  shifted strip), or rebinds on every revision (fader drags fail after a live edit).
- Gate 5: turns red if a reboot leaves `applied` pending forever or keeps the old revision.

## Dependencies

- *Apply session transactions from the browser SDK* (#1296).
- *Export transaction apply and anchored seek from the browser engine module* (#1293).
- *Diff a replacement document against the committed model and export replace from the browser
  engine module* (#1386).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Admit browser live edits in the Worker through the committed model* (#1382).
- *Build and encode session transactions in the SDK* (#1383).
- *Encode the session, submix, output, route, automation and VCA edits in the SDK* (#1385).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
