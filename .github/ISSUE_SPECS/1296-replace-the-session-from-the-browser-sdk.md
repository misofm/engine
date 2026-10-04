# Replace the session from the browser SDK

Slice B7 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

**Blocked on owner question Q6** of the umbrella: decision 14's C1 wants callers to see one edit
API, and the browser has none yet (its F6). Do not start until the owner rules whether
`replaceSession` is public now or internal until #1057.

## Product outcome

A browser app (and the headless SDK engine) can call `replaceSession(session)` while audio plays:
adding a track on an existing source, adding or removing an effect, rerouting, or removing a track and
its source. Unchanged tracks continue with whatever state the engine carries (all of it once slices
7-14 have landed), and the live controls, measurement and response APIs address the new session
afterwards. A track on a **new** source is *Feed a source that a browser replacement adds* (B8).

## Context

- `createEngine` (`sdk/src/browser/engine.ts:483`) validates a document in a scratch Worker before it
  builds the AudioContext (`scratchBootInWorker`, `:1302`; rationale `:91-118`), then boots the
  worklet. The headless engine (`sdk/src/headless/engine.ts`) calls the module's exports directly
  through `WasmBoundary` (`sdk/src/core/boundary.ts`).
- The live controls hold the booted document byte for byte and resolve stable IDs against it
  (`sdk/src/core/live-controls.ts:267-290`, `#booted` at `:981`); a handle keeps addressing strip
  indices of that document.
- The PCM feed is attached for a fixed source list (`attachEngineFeed`,
  `sdk/src/browser/pcm-feed.ts:285`).
- The worklet message comes from *Qualify a browser session replacement in real browsers* (#1295) and
  its predecessors; the exports from B4.
- SDK checks: `bash scripts/check-sdk-headless.sh <artifacts>` renders the real module in Node;
  `sdk/test/browser-evals.mjs` drives the browser engine against test doubles.

## Decisions frozen for this slice (pending Q6)

- **D1. API.** `replaceSession(session: SessionLike | Uint8Array)` on the browser and headless
  engines. It resolves with the new session map, or rejects with a typed `MisoEngineError` carrying
  the engine result and diagnostic.
- **D2. Validate off the audio thread.** The browser engine first boots the document in the scratch
  Worker, as `createEngine` does, and requires the same sample rate and quantum. Only an accepted
  document is posted to the worklet.
- **D3. Rebind and invalidate.** On success the engine's live controls, measurement feeds and response
  helpers rebind to the new document (`#booted` and the session map). A handle obtained before the
  replacement is invalidated: its next call rejects with a stale-handle usage error, so it never
  addresses a shifted strip index.
- **D3a. Commands during a replacement.** From the moment the engine posts `miso.replace.v1` until it
  resolves, live commands, observation subscriptions and response requests reject with a typed
  "replacement pending" usage error, so none is built from the old maps and applied to shifted strip
  indices.
- **D4. Sources.** The new session's source set must be the old one or a subset; a document that adds
  a source rejects with a usage error that names B8's API. A removed source is detached from the PCM
  feed after the replacement resolves.
- **D5. Documentation.** The SDK README and API docs: what carries, what restarts (meters; values the
  document changed), and that a replacement blocks the audio thread for about the time B1 measured.

## Deliverables

1. D1-D5 in `sdk/src/core/boundary.ts`, `sdk/src/headless/engine.ts`, `sdk/src/browser/engine.ts`,
   `sdk/src/core/live-controls.ts`, `sdk/src/browser/live-controls.ts`,
   `sdk/src/browser/measurement.ts`, and the docs.
2. Tests (below).

## Authorized paths

- `sdk/src/`, `sdk/test/`, `sdk/README.md`, `scripts/check-sdk-headless.sh` (only to add the case)

## Non-goals

- No added sources (B8). No change to the worklet or the Rust host.

## Objective gates

1. **Gap-free acceptance, headless, real module.** In the headless SDK check: a 4-track session
   renders; `replaceSession` adds a fifth track on an existing source, muted, with an EQ insert; the rendered PCM equals a fresh headless engine created from the new session and fed the
   same PCM, bit for bit.
2. **Browser protocol.** In `browser-evals.mjs`: a document the scratch boot refuses (another sample
   rate) rejects before anything is posted to the worklet; an accepted one posts exactly one
   `miso.replace.v1`.
3. **Handles.** A live-controls handle from before the replacement rejects as stale; a new handle's
   fader command on the new track succeeds; a command on a removed track rejects as unknown.
4. **No command during a replacement.** A live fader command issued after the replace is posted and
   before it resolves rejects with the pending error and posts nothing.
5. Commands: the SDK package qualification as in `qualification.yml` (`check-sdk-generated.sh`,
   `python3 -B scripts/check-sdk-deletions.py`, `check-sdk-types.sh`, `check-sdk-headless.sh`,
   `sdk-package.sh check`), and the umbrella's inherited gates.

## Test value

- Gate 1: an SDK path that reboots instead of replacing, or that rebinds before the replacement
  resolves, turns it red.
- Gate 2: a refusal discovered only on the audio thread blocks it for nothing; it turns red.
- Gate 3: a stale handle that keeps addressing old strip indices edits the wrong strip; it turns red.
- Gate 4: a command built from the old maps and acked after the swap edits a shifted strip; it turns
  red.

## Dependencies

- *Qualify a browser session replacement in real browsers* (#1295).
- Owner question Q6 of *Swap a rebuilt plan without an audio gap* (#1269).
