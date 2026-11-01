# Check the warm-successor deadline in the browser Worker's service loop and report its outcome

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-10, D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287). Code anchors
refreshed on `main` at `7d89eecc5` (#1360 attempt 1).

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
  `control_plane::SessionState`'s transaction apply, so classification is #1403 D2 with the
  `WarmConfig` of #1360 D1, with no browser-specific code.
- *Check the warm-successor deadline in miso_engine_v1_service and report its outcome* (#1360) D2
  adds the deadline check to `SessionState::service`, so the browser's service loop runs it with no
  browser-specific engine code.
- *Publish the applied-revision watermark in the browser status* (#1349): `WebStatus`'s watermark
  words.
- The browser feeds sources from its PCM pump, which fills every ring until it is full
  (`pumpUntilFull`, `hosts/host-web/web/stem-store/pcm-pump.js:241`). Ring headroom covers `P_MAX`
  plus one quantum, and the browser host's per-source in-flight bound follows that ring (*Grow the
  default source ring by the warm-prime headroom*, #1406 D2 and D4), so a pump that keeps up keeps
  the `P + q` frames that an exact growth needs queued.
- The worklet's render-locked allocation counter (*Gate AudioWorklet render against allocation
  statically and at runtime*, #1333 D1-D3) spans all of `miso_engine_web_v1_render`, so it covers
  the readiness check and the prime.
- The browser qualification runs `npm run qualify` in `hosts/host-web/qualification` (the
  `browser` job of `.github/workflows/qualification.yml:460`); result mutations live in `mutate()`
  (`hosts/host-web/qualification/run.mjs:1337`).

## Decisions frozen for this slice

- **D1. One path in both modes.** The browser classifies (#1403 D2, with the `WarmConfig` of
  #1360 D1), publishes `Primed` (#1403 D3) and checks the deadline (#1360 D2) exactly as the C ABI
  does, through `SessionState`. The warm path has no
  page-mode choice: isolated and non-isolated pages take the same path. A non-isolated page runs
  the same step from its single-mode service tick (#1381 D6).
- **D2. Render does the adoption.** The readiness check, the claim and the raw-frame prime run in
  the worklet's `miso_engine_web_v1_render` (#1355); the worklet renders nothing ahead and copies
  no plan state. The render-locked allocation counter stays exactly 0 across the readiness check and
  a prime (#1333's runtime gate).
- **D3. Outcome report.** The status words carry the outcome (#1349 D2): `EXACT`,
  `TRANSITION_FALLBACK`, or with `SUPERSEDED` set.
- **D4. Acked-batch question.** As #1360 D7, unchanged.

## Deliverables

1. The native test binary `hosts/host-web/tests/latency_growth.rs` (new) for gate 3.
2. One browser qualification case per page mode for gates 1 and 2, in
   `hosts/host-web/qualification/`, each with a red mutation in `mutate()`.
3. One wiring change in `hosts/host-web/src/`: where the browser host builds its
   `control_plane::SessionState` (#1290), it sets #1403 D1's `warm` field to the production
   `WarmConfig` (#1360 D1) for the session's rate and quantum, the same value the C ABI injects.
   Gates 1 and 2 are red without it. This slice adds no other browser wiring: a missing piece of
   the classification, the publication, the deadline step, the prime, the service loop or the
   status words is a defect of its owner (#1403, #1360, #1355, #1381, #1349), and this slice stops
   and reports it.

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
- *Grow the default source ring by the warm-prime headroom* (#1406): the ring and the host's
  in-flight bound.
- *Classify a latency-growth edit and publish its warm successor from the control plane* (#1403).
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332).
- *Swap and retire browser plans through the Worker's service loop* (#1381).
- *Publish the applied-revision watermark in the browser status* (#1349).
- *Gate AudioWorklet render against allocation statically and at runtime* (#1333).
- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331).
- *Replace the running browser session in the Rust host* (#1290).
- *Export transaction apply and anchored seek from the browser engine module* (#1293).
- *Send a session transaction to the browser control plane* (#1294).

## Amendment (#1360 attempt 1, 2026-11-01: root rulings Q2, Q3)

- **Deliverable 3 is dropped.** #1360 D1 is central (root ruling Q2): the control plane's public
  `compile_children` passes `host_core::warm::production_config` for the session's rate and
  quantum, and the browser boots through it (`hosts/host-web/src/lib.rs`), so every browser session
  already holds the production `WarmConfig` and this slice adds no browser wiring. Gates 1 and 2
  stay; their red without the config is now #1360's.
- **Firefox is out** (owner ruling 2026-10-25): gate 4's qualification runs Chromium and WebKit
  only. Gates 1 and 2 read "two engines".
- **The browser already runs the deadline step.** The Worker's service loop and the single-mode tick
  call `SessionState::service` (`hosts/host-web/src/lib.rs:2010`), which runs #1360 D2's step. A rig
  that feeds each source only the block it renders and services after each block takes the
  transition at the deadline (#1360's evidence: `the_plan_latency_record_follows_the_active_plan`).


## Evidence (attempt 1, 2026-11-01)

Branch `codex/issue-1361-browser-warm-deadline`, from #1360's head `63a690aea`.

- **No host code changed.** The browser host already reports the outcome: its render half copies
  each advancing render report into the status words (`hosts/host-web/src/lib.rs:3224`), and its
  `service` runs `SessionState::service`. No gap was found, so no rendered bit moves.
- **Gate 3** (`hosts/host-web/tests/latency_growth.rs`, new). The control half on the test thread,
  the released render half (`RenderHost`) rendering on its own thread. The growth is
  `crates/capi/tests/latency_growth.rs`'s (a muted limiter track, `P` = 512) on the nine-track
  fixture, default ring.
  - `a_growth_fed_ahead_is_adopted_exact_with_no_service_call`: the source queued 4,096 frames
    (more than `2P + 2q`) past the clock; 16 blocks with no control call; the status words read the
    revision with `EXACT`; the output equals the predecessor rendered on. Red with no production
    config (diverges at block 6).
  - `a_growth_never_ready_falls_back_at_the_next_service_call`: fed one quantum ahead; 110 blocks
    (twice the 7,040-sample deadline) with no call: pending, bits equal to the predecessor; the next
    `service()` publishes the transition, adopted at `ceil_q(clock + q + 480)` = 15,488, status
    `TRANSITION_FALLBACK`. Red with the step taken out of `SessionState::service` (`(42, 0, 1)`
    against `(43, 15488, 2)`), red with no production config (completed with no service call), and
    red with the status's outcome copy removed (`1` against `2`).
- **Gates 1 and 2** (`warm-growth`, one case run on both legs). A one-track session on a 72-block
  stem, the whole stem prefilled into a ring that holds it. Two offline renders of 64 quanta: one
  with no edit, one that suspends after 8 quanta, commits the growth through the SDK's
  transaction builder, reads the status, and resumes. The gate holds: reply `ok`, path `rebuild`;
  pending at the commit, which came after 1,024 rendered samples; status and watermark at the
  revision, outcome `1`; the capture's digest equal to the no-edit capture's; both instances' render
  allocations 0 (two new rows, `RAW_INSTANCES` 17 to 19).
  - Why exact and not the fallback: render adopts a ready warm successor at its first block with no
    control call, so a prefilled stem makes the block and the outcome timing-free. The fallback
    needs a service step past the deadline, and the Worker loop and the single-mode tick run on
    wall-clock timers while an offline context renders as fast as it can. Gate 3 owns the fallback.
  - What it adds to `console-limits`: that row's bus add takes the transition; this row holds the
    warm path's exact continuation through the browser.
  - Red: mutations `warm-growth-outcome` (outcome `2`, `TRANSITION_FALLBACK`) and
    `warm-growth-output` (digest) under `--self-test-mutations`; and a module built with no
    production config fails the gate on Chromium `single` and `worker` (the ordinary successor
    reports `1` but its digest differs: `edf985f1...` against `25784310...`).
- **Browser.** `npm run qualify -- --check-matrix --self-test-mutations`, `sdk/dist` deleted:
  Chromium 151 and WebKit 26.5, `single` and `worker`, all pass; the same capture digest on all four.
- **Stated observation (not investigated).** With the default ring, a 72-block prefill is accepted
  on the `worker` leg (the Worker holds chunks above the ring) but refused `BACKPRESSURE` at block
  57 on the `single` leg. The case uses an explicit ring that holds the stem.
- **Checks.** fmt; workspace clippy (`--all-targets --all-features -D warnings`); release tests of
  host-web (`--all-features`); `check-workspace-policy.sh`, `realtime_policy`,
  `check-test-support-ci.py`; web build and `check-web-audioworklet.sh` in both forms;
  `test-web-audioworklet.sh`: all pass.
