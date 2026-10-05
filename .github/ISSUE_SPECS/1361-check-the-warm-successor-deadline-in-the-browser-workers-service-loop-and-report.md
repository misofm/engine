# Check the warm-successor deadline in the browser Worker's service loop and report its outcome

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-10, D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287). Code anchors
verified on `main` at `6fb211594`.

## Product outcome

In the browser, adding a latent effect during playback swaps seamlessly, on a cross-origin-isolated
page and on a page that is not isolated alike. The AudioWorklet's render adopts the primed warm
successor at the first ready block. The Worker's service loop (or, on a non-isolated page, the
worklet's control handler) only checks the deadline and runs the transition fallback once it has
passed. Both page modes report `EXACT` through the status watermark. Warm growth needs no
cross-origin isolation.

## Context

- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332): the
  `worker` and `single` modes (D1).
- *Swap and retire browser plans through the Worker's service loop* (#1381): the
  `miso_engine_web_v1_service` export (D5) and the service loop in both modes (D6), which calls
  `SessionState::service`.
- *Replace the running browser session in the Rust host* (#1290): the browser's structural apply is
  `control_plane::SessionState`'s transaction apply, so classification is #1360 D1 with no
  browser-specific code.
- *Check the warm-successor deadline in miso_engine_v1_service and report its outcome* (#1360) D2
  adds the deadline check to `SessionState::service`, so the browser's service loop runs it with no
  browser-specific engine code.
- *Publish the applied-revision watermark in the browser status* (#1349): `WebStatus`'s watermark
  words.
- The browser feeds sources from its PCM pump, which fills every ring until it is full
  (`pumpUntilFull`, `hosts/host-web/web/stem-store/pcm-pump.js:189`). Ring headroom covers `P_MAX`
  plus one quantum (#1358), so a pump that keeps up keeps the `P + q` frames that an exact growth
  needs queued.
- The worklet's render-locked allocation counter (*Gate AudioWorklet render against allocation
  statically and at runtime*, #1333 D1-D3) spans all of `miso_engine_web_v1_render`, so it covers
  the readiness check and the prime.
- The browser qualification runs `npm run qualify` in `hosts/host-web/qualification` (the
  `browser` job of `.github/workflows/qualification.yml:362`); result mutations live in `mutate()`
  (`hosts/host-web/qualification/run.mjs:616`).

## Decisions frozen for this slice

- **D1. One path in both modes.** The browser classifies, publishes `Primed` and checks the
  deadline exactly as the C ABI does (#1360 D1-D2), through `SessionState`. There is no catch-up,
  no off-thread executor and no page-mode choice of warm path: `RenderOnly` and the mode selection
  it belonged to are deleted, never added. A non-isolated page runs the same step from its
  single-mode service tick (#1381 D6).
- **D2. Render does the adoption.** The readiness check, the claim and the raw-frame prime run in
  the worklet's `miso_engine_web_v1_render` (#1355). The worklet runs no catch-up, copy or
  pre-roll. The render-locked allocation counter stays exactly 0 across the readiness check and a
  prime (#1333's runtime gate).
- **D3. Outcome report.** The status words carry the outcome (#1349 D2): `EXACT`,
  `TRANSITION_FALLBACK`, or with `SUPERSEDED` set. The browser has no pre-roll outcome.
- **D4. Acked-batch question.** As #1360 D7, unchanged.

## Deliverables

1. The native test binary `hosts/host-web/tests/latency_growth.rs` (new) for gate 3.
2. One browser qualification case per page mode for gates 1 and 2, in
   `hosts/host-web/qualification/`, each with a red mutation in `mutate()`.
3. Any browser wiring that D1 finds missing in `hosts/host-web/src/`.

## Authorized paths

- `hosts/host-web/src/`, `hosts/host-web/tests/latency_growth.rs` (new)
- `hosts/host-web/qualification/`

## Non-goals

- SDK surface changes beyond what #1349 and stream H expose. The deadline step itself (#1360) and
  the prime (#1355).

## Objective gates

1. **Isolated, three engines.** In the browser qualification, on the isolated leg, a playing
   session gains a muted limiter track.
   - The captured output equals, bit for bit, the native render of the same fixture, quantum and
     rate continued with no swap.
   - The status watermark reaches the revision with `appliedOutcome === 1n` (`EXACT`).
   - The worklet's render-allocation counter is 0.
   - Mutation `warm-growth-outcome` (outcome set to `TRANSITION_FALLBACK`) must fail.
2. **Non-isolated, three engines.** The same case on the leg without COOP/COEP (`single` mode)
   gives the same output and `EXACT`, and the render-allocation counter is 0.
3. **Deadline in the browser host (native, `host-web/test-support`).** In
   `hosts/host-web/tests/latency_growth.rs`, the control half (#1381's `ControlHalf`) and the
   render half run on two threads. The growth transaction is applied while each source is fed
   exactly one quantum ahead. Render runs past the deadline with no `service` call: the revision
   stays pending, and every block equals the predecessor continued. The next
   `ControlHalf::service()` publishes the transition, and the status words report
   `TRANSITION_FALLBACK`. The same script with each source fed `P + q` ahead reports `EXACT` with
   no `service` call.
4. Commands:
   - `cargo test --locked -p host-web --features host-web/test-support`
   - the artifact build and `bash scripts/check-web-audioworklet.sh` in both forms that #1332
     gate 7 spells; `bash scripts/test-web-audioworklet.sh`
   - in `hosts/host-web/qualification`: `npm run qualify -- --artifacts ... --sdk-root ...
     --browser <b> --check-matrix --self-test-mutations`, for chromium, firefox and webkit
   - `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets -- -D warnings`

## Test value

- Gate 1: a browser path that publishes the growth as an ordinary successor, or that adopts it
  through a control call instead of render, gives a gap, a shifted stream or a pending revision;
  a prime that allocates counts a worklet allocation. Red.
- Gate 2: a non-isolated page that is denied the warm path (a leftover render-only mode) reports
  `TRANSITION_FALLBACK` instead of `EXACT`. Red.
- Gate 3: a browser loop that never reaches the deadline step leaves the revision pending forever,
  and a deadline counted in wall time or in control calls falls back at the wrong point or with no
  render. Red.

## Dependencies

- *Check the warm-successor deadline in miso_engine_v1_service and report its outcome* (#1360).
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Publish the applied-revision watermark in the browser status* (#1349).
- *Gate AudioWorklet render against allocation statically and at runtime* (#1333).
- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331).
- *Replace the running browser session in the Rust host* (#1290).
- *Export transaction apply and anchored seek from the browser engine module* (#1293).
- *Send a session transaction to the browser control plane* (#1294).
