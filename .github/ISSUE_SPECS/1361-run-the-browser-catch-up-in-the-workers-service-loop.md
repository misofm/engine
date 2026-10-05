# Run the browser catch-up in the Worker's service loop

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-10, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

In the browser, adding a latent effect during playback swaps seamlessly. The Worker catches the
successor up while the AudioWorklet keeps rendering, and the SDK's watermark shows `EXACT` at the
adoption sample. A non-isolated page keeps the same API: it takes the render-only path and reports
its outcome (`PREROLL_FALLBACK` or `TRANSITION_FALLBACK`), counted.

## Context

- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332): one
  shared memory, with the control plane in a Worker; single-instance mode on a non-isolated page.
- *Swap and retire browser plans through the Worker's service loop* (#1381): the Worker calls
  `SessionState::service`.
- *Publish the applied-revision watermark in the browser status* (#1349): `WebStatus`'s watermark
  words.
- The catch-up runs inside `SessionState::service` (#1360 D2), so the Worker gets it with no
  browser-specific engine code.
- On Wasm, `CanonicalFpEnv` is empty and the FP behaviour is fixed (`crates/lane/src/fpenv.rs:370`).
- The allocation gates of *Gate AudioWorklet render against allocation statically and at runtime*
  (#1333) cover the worklet's render, which now includes the copy at B, a pre-roll and adoption.

## Decisions frozen for this slice

- **D1. Isolated page.** The Worker's service loop already calls `service`, which runs the
  catch-up slice. The slice size is `CATCH_UP_SLICE_BLOCKS` (#1360 D2), and the loop yields between
  calls.
- **D2. Non-isolated page.** The control plane runs in single-instance mode with no off-thread
  executor, so classification uses `CatchUpMode::RenderOnly` (#1358 D5).
- **D3. Allocation.** The worklet's render-locked allocation counter stays at exactly 0 across the
  copy, a pre-roll and adoption (#1333's runtime gate).
- **D4. Acked-batch question.** As #1360 D6, unchanged.

## Deliverables

1. D1-D2 in the control-plane mode selection and the Worker host code.
2. One browser qualification case per page mode, in `hosts/host-web/qualification/`.

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/web/` (the Worker file), `hosts/host-web/qualification/`
- `crates/control-plane/src/` (mode selection only)

## Non-goals

- SDK surface changes beyond what #1349 and stream H expose.

## Objective gates

1. **Isolated, three engines.** In the browser qualification (`npm run qualify` in
   `hosts/host-web/qualification`, the `browser` job of `qualification.yml`), a playing session
   gains a muted limiter track.
   - The captured output equals the native reference of #1360 gate 1 for the same fixture,
     quantum and rate.
   - The status watermark reports `EXACT`.
   - The worklet allocation counter is 0.
2. **Non-isolated.** The same page without COOP/COEP reports `PREROLL_FALLBACK` (or
   `TRANSITION_FALLBACK` when the bound is exceeded), and the matching counter rises by 1.
3. Commands:
   - `bash scripts/check-web-audioworklet.sh` with the qualification job's arguments
   - the `browser` job's `npm run qualify` for chromium, firefox and webkit
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets -- -D warnings`

## Test value

- Gate 1: a Worker loop that never services after the transaction, or a catch-up run in the
  worklet, leaves the revision pending or counts a worklet allocation. Red.
- Gate 2: a non-isolated page that waits for a catch-up that cannot run stalls the edit. Red.

## Dependencies

- *Run the C ABI catch-up from miso_engine_v1_service and report its outcome* (#1360).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Publish the applied-revision watermark in the browser status* (#1349).
- *Gate AudioWorklet render against allocation statically and at runtime* (#1333).
- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331).
- *Replace the running browser session in the Rust host* (#1290), *Export transaction apply and
  anchored seek from the browser engine module* (#1293) and *Send a session transaction to the
  browser control plane* (#1294): the browser path that submits the structural transaction this
  issue's gates apply during playback.
