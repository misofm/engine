# Apply session transactions from the browser SDK

Stream H(c) of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-3, D15-11, D15-17).
Formerly slice B7 of *Swap a rebuilt plan without an audio gap* (#1269).
Code anchors verified on `main` at `6fb211594`.

The former blocker, owner question Q6 of #1269 (is `replaceSession` a public second edit API?), is
answered by decision 15 D15-11: the SDK's one edit API is `engine.apply(transaction)`, and
`replaceSession` is only a convenience over it.

Scope: the shared SDK core and the browser engine. The headless engine's `apply` is the successor
*Apply session transactions from the headless SDK engine* (#1389).

## Product outcome

A browser app edits a playing session through one API:
`engine.apply(transaction)` resolves with `{ revision, path }` as soon as the control plane commits
(`path` is `live`, `model_only` or `rebuild`). `engine.replaceSession(session)`
diffs the given session against the committed model into one transaction and applies it. The app
observes completion through the watermark: `engine.watermark()` returns the latest
`{ revision, sample, outcome }`, and `engine.applied(revision)` resolves when the watermark covers
that revision. After every commit, live controls, measurement and response helpers address the
committed session.

## Context

- `createEngine` (`sdk/src/browser/engine.ts:483`) boots through a scratch Worker first
  (`scratchBootInWorker`, `:1302`; rationale from `:93`). The headless engine
  (`sdk/src/headless/engine.ts`, `OfflineEngine` at `:114`) calls the module's exports directly
  through `WasmBoundary` (`sdk/src/core/boundary.ts:326`). `OfflineEngine.sessionMap()`
  (`headless/engine.ts:205`) delegates to `WasmBoundary.sessionMap()` (`boundary.ts:568`).
- `EngineLiveControls` (`sdk/src/core/live-controls.ts:976`) resolves stable IDs against a session
  map and holds a `withSession` builder to the booted document byte for byte (`#booted` at `:981`,
  `assertBootedSession` at `:276`). A handle keeps addressing strip indices of that document.
- The SDK has no session-transaction type or encoder today; only the live command records
  (`boundary.ts:1211`). *Build and encode session transactions in the SDK* (#1383) adds
  `SessionTransaction` and its builder, encoding the protocol's `SESSION_TRANSACTION_APPLY` frame
  (`crates/protocol/src/session_wire.rs`), whose expected revision must be `Exact`
  (`session_wire.rs:74`).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332) adds
  `engine.instanceMode` (`"worker" | "single"`). *Admit browser live edits in the Worker through the
  committed model* (#1382) commits every live edit to the committed model, so live edits advance
  the committed revision too. The watermark reaches the page through *Publish the applied-revision
  watermark in the browser status* (#1349) and #1294's `watermark` event.
- The document diff behind `replaceSession` runs in Rust in the control half: *Diff a replacement
  document against the committed model and export replace from the browser engine module* (#1386).
  #1383 covers source, track and strip-effect edits; *Encode the session, submix, output, route,
  automation and VCA edits in the SDK* (#1385) covers the rest.
- Errors: `MisoEngineError` (`sdk/src/core/errors.ts:88`), `MisoUsageError` (`:127`).
- The wrapper methods, the `miso.edit.v1` reply, the committed-document request and the watermark
  event come from *Send a session transaction to the browser control plane* (#1294); the exports from
  *Export transaction apply and anchored seek from the browser engine module* (#1293). Real-browser evidence is
  *Qualify a structural browser edit in real browsers* (#1295).
- SDK checks: `bash scripts/check-sdk-headless.sh <artifacts>` runs `node --test 'test/*-evals.mjs'`
  in `sdk/` against the real module (`check-sdk-headless.sh:91`), including
  `sdk/test/browser-evals.mjs` (browser engine over test doubles).

## Decisions frozen for this slice

- **D1. API.** On the browser engine (the types live in `sdk/src/core/` so the headless successor
  reuses them):
  - `apply(transaction: SessionTransaction): Promise<EditOutcome>` with
    `EditOutcome = { revision: bigint; path: EditPath }`. A refusal rejects with a
    `MisoEngineError` carrying the engine result and diagnostic; nothing was committed.
  - `replaceSession(session: SessionLike | Uint8Array): Promise<EditOutcome>`, through the
    `replace` path (the diff runs in the engine against the committed model, D15-11).
  - `revision: bigint`, the latest committed revision this engine has seen.
  - `watermark(): { revision; sample; outcome }` and `applied(revision): Promise<{ sample;
    outcome }>`. `outcome` exposes the flags `exact`, `transitionFallback` and
    `superseded` (D15-17). `applied` never rejects for a committed revision; it rejects only when the
    engine is disposed. While the AudioContext is suspended it stays pending: the warm-successor
    deadline counts render samples (D15-17).
  `SessionTransaction` and its builder come from #1383. `revision` follows every committed reply,
  from `apply`, `replaceSession` and live edits (#1382).
- **D2. No validation pass outside the control plane.** The browser engine does not scratch-boot an
  edit: the control plane validates and prepares off the audio thread (D15-10). A sample rate or
  quantum change is refused by the engine with its typed result.
- **D3. Ordering and rebinding.** The SDK keeps one ordered control channel. While an `apply` or
  `replaceSession` is in flight, a live submit waits for its reply before it is posted.
  - The map epoch changes only when the plan shape changes: a reply with path `rebuild` bumps it.
    A `live` or `model_only` reply, and every live edit, advance `revision` but leave the epoch,
    the session map and every resolved edit object valid.
  - After a `rebuild` reply the SDK re-reads the session map and the committed document
    (`committedDocument()`) before the next live edit is resolved. An edit object resolved under
    an older epoch is rejected with a typed stale-edit `MisoUsageError` and posts nothing; a fresh
    `edit.track(id)` resolves against the new map.
  - `withSession` holds its builder to the committed document, read through
    `committedDocument()` when it is called, instead of the booted one.
- **D4. Sources.** A transaction that adds a source starts it as the engine contract says
  (generation 1, frame 0, until the app seeks it). A removed source keeps playing until its strip's
  phase-2 swap (*Remove a strip in two phases: ramp out, then a scheduled swap*, #1325). Feeding and
  retiring sources through the SDK PCM feed is *Feed and retire the sources a browser edit adds or removes*
  (#1297).
- **D5. Documentation.** The SDK README gains "Edit a playing session": `apply`, the three paths,
  pending until the watermark covers a revision, the outcome flags, `replaceSession` as a
  convenience, what carries across a rebuild and what restarts, and that a non-isolated page's
  structural edit blocks the audio thread and is counted (D15-10).
- **Superseded.** The old D2 scratch boot of every replacement, the old D3a "replacement pending"
  refusal of live commands, and the old D4 refusal of a document that adds a source are superseded
  by decision 15 D15-10, D15-11 and D15-17.

## Deliverables

1. D1-D4 in `sdk/src/core/boundary.ts`, `sdk/src/browser/engine.ts`,
   `sdk/src/core/live-controls.ts`, `sdk/src/browser/live-controls.ts`,
   `sdk/src/browser/measurement.ts`, and their type exports.
2. D5 in `sdk/README.md`.
3. Tests (below).

## Authorized paths

- `sdk/src/core/`, `sdk/src/browser/`, `sdk/src/index.ts` and the barrel type exports,
  `sdk/test/browser-evals.mjs`, `sdk/test/live-controls-evals.mjs`, `sdk/README.md`

## Non-goals

- No transaction encoder (#1383). No PCM feed change (#1297). No worklet, Worker or Rust
  change.
- No headless `apply` (`sdk/src/headless/`): that is *Apply session transactions from the
  headless SDK engine* (#1389), which reuses this slice's core types and D3 rebinding and proves
  the bits on the real module.

## Objective gates

All behavioural gates run in `sdk/test/browser-evals.mjs` over the browser engine's test doubles
(real bits in a browser are #1295).

1. **Structural edit.** `replaceSession` of a document that adds a fifth track posts exactly one
   `miso.replace.v1`; a `rebuild` reply with revision +1 resolves it with `{ revision, path:
   "rebuild" }`, and `revision` reads the new value.
2. **Live path and watermark.** `apply` of a fader-only transaction posts one `miso.apply.v1` and
   resolves with path `live`. `applied(revision)` stays pending until a `miso.watermark.v1` covers
   that revision, then resolves with that event's `sample` and `outcome`; a watermark event for an
   older revision does not resolve it. After `dispose()` a pending `applied` rejects.
3. **Refusal.** A refusal reply rejects with the result and diagnostic and leaves `revision`
   unchanged; the next `apply` carries the unchanged expected revision.
4. **Ordering and stale edits.** A live submit issued while an `apply` is in flight posts nothing
   until the reply. After a `rebuild` reply, an edit resolved before it rejects as stale and posts
   nothing; a new `edit.track(id)` on the added track succeeds; one on a removed track rejects as
   unknown. After a `live` reply (and after a live edit) an edit object resolved before it still
   posts, unchanged, and the session map is not re-read.
5. **withSession.** After a `rebuild` commit, `withSession(oldBuilder)` rejects and
   `withSession(newBuilder)` succeeds.
6. Commands, with an artifact directory built as in `qualification.yml`'s `artifact` job and
   `npm ci` in `sdk/`: `bash scripts/check-sdk-generated.sh <artifacts>`,
   `python3 -B scripts/check-sdk-deletions.py`, `bash scripts/check-sdk-types.sh`,
   `bash scripts/check-sdk-headless.sh <artifacts>`, `bash scripts/sdk-package.sh check <artifacts>`.

## Test value

- Gate 1: an SDK path that reboots instead of applying, or that leaves `revision` at the old value,
  turns it red.
- Gate 2: an `applied` that resolves at the commit reply instead of the watermark, or on a
  watermark for an older revision, turns it red.
- Gate 3: a refusal that advances `revision` makes the next `apply` carry a wrong expected revision.
- Gate 4: an edit resolved under the old map and posted after a rebuild addresses a shifted strip
  index and is acknowledged on the wrong strip; it turns red. An epoch bumped on every revision
  would reject ordinary fader drags after any live edit; it turns red too.
- Gate 5: a `withSession` still held to the booted document binds insert IDs of a session that no
  longer runs.

## Dependencies

- *Send a session transaction to the browser control plane* (#1294).
- *Qualify a structural browser edit in real browsers* (#1295).
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Diff a replacement document against the committed model and export replace from the browser
  engine module* (#1386).
- *Encode the session, submix, output, route, automation and VCA edits in the SDK* (#1385).
- *Hold live values in latest-target cells on both hosts* (#1312).
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325).
- *Give every browser plan live strip fader and mute lanes* (#1326).
- *Build and encode session transactions in the SDK* (#1383).
- *Admit browser live edits in the Worker through the committed model* (#1382).
- *Publish the applied-revision watermark in the browser status* (#1349).

Dependent: *Apply session transactions from the headless SDK engine* (#1389).
