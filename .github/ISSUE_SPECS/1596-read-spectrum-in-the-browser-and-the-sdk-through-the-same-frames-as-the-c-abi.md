# Read spectrum in the browser and the SDK through the same frames as the C ABI

Slice S31b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and C13). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13):
one implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

Split from the planner's S31: #1595 serves the reads on the C ABI; this slice sends the same frames
from the browser and both SDKs; #1597 deletes the browser's own spectrum exports, records and
analysis Worker.

## Product outcome

The browser and the headless SDK configure and read spectrum with the same three frames as a
mobile app: `SPECTRUM_CONFIGURE`, `SPECTRUM_CAPTURE` and `SPECTRUM_STREAM`. They go through the
browser's one command entry to the same `SessionState::command`, and the SDK decodes the replies
with its one frame codec. The browser's spectrum readers move to the control half, beside the
C ABI's. A browser page can now also change its taps at run time, as a mobile app can.

## Context

- **The browser's command entry.** #1562 D1: `miso_engine_web_v1_control_request_ptr`,
  `_command`, `_response_ptr` and `_response_bytes` on the control half (the Worker; the worklet's
  message handler on a page without cross-origin isolation), and the internal `control` port
  message. It accepts any frame `SessionState::command` accepts. #1570 makes the browser serve
  every protocol command through it.
- **The SDK's frame codec.** #1563 D1: `sdk/src/core/control-frame.ts`; D7: the browser posts the
  `control` message, the headless engine calls the `miso_engine_web_v1_control_*` exports
  in-process.
- **The SDK's spectrum paths today.**
  - Browser engine (`sdk/src/browser/engine.ts`): the API (`:219-221`), the boot options
    (`:492-495`; `policy.spectrumHopFrames`, `sdk/src/browser/policy.ts:41`), the SDK's own target
    and channel checks (`validateSpectrumQuery` `:337`, `validateSpectrumCollectionQuery`
    `:363-379`), `selectSpectrum` (`:676`), `selectSpectrumStream` (`:728`),
    `startSpectrumStream` (`:793`), `querySpectrum` (`:1149`), `subscribeSpectrum` (`:1301`).
  - Headless engine (`sdk/src/headless/engine.ts`): `subscribeSpectrum` (`:237`), `armSpectrum`
    (`:247`), `readSpectrum` (`:252`), `cancelSpectrum` (`:257`); its boundary
    (`sdk/src/core/boundary.ts`: `armSpectrum` `:826`, `selectSpectrum` `:840` with its own entry
    check `:847-854`, `readSpectrum` `:925`).
  - The analysis Worker: `BrowserSpectrum` (`sdk/src/browser/response.ts:399-676`), its messages
    (`sdk/src/browser/response-worker.ts:103-110`), `SpectrumModule`
    (`sdk/src/core/spectrum.ts:563`).
  - The worklet host's spectrum methods (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1482-1581`)
    and the worklet's handler (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1267-1457`).
  - The subscription manager's transport hooks
    (`sdk/src/core/observation-subscriptions.ts:166-179`).
- **The browser's readers** live in the plan's `RenderCompanions` (#1400 D5) and are paired in the
  swap block (#1381 D3, its spectrum bullet). #1593 left them there.

## Decisions frozen for this slice

- **D1. One codec, one entry.** `sdk/src/core/control-frame.ts` gains the three payloads of
  #1594 D2-D3 and #1595 D2-D3, encode and decode. The browser posts them as #1562's `control`
  message; the headless engine calls the `control` exports. Neither decodes a spectrum record of
  its own after this slice.
- **D2. Boot.** The options `spectrum`, `spectrumCollection` and `policy.spectrumHopFrames` stay.
  After boot and before `createEngine` resolves, the SDK sends one `SPECTRUM_CONFIGURE` (single
  form for `spectrum`, collection form for `spectrumCollection`, `hop_frames` from the policy) and
  waits, bounded by `requestDeadlineMs`, until a spectrum reply reports
  `adopted_epoch == configured_epoch`. The browser's Worker boot passes no spectrum to
  `compile_children`; #1593 D2's parameter is deleted. The extra preparation at boot is recorded
  once as evidence on the 64-track console fixture, not as a gate. The SDK learns that the engine
  serves spectrum, hop included, from capability flag bit 16 of `CAPABILITIES_GET` (#1594 D10); it
  no longer probes the `spectrum_hop_capability` export.
- **D3. The readers move to the control half.** `WebRuntimePreparer` returns its captures to the
  session, as `CapiPreparer` does (#1593 D3); `RenderCompanions` loses its spectrum field, and the
  swap block no longer pairs spectrum readers. Both hosts pair at promotion (#1593 D5). The Worker
  reads the same shared memory that the worklet's observer writes.
- **D4. The analysis runs where the session runs** (#1595 D4): in the control Worker, or, on a page
  without cross-origin isolation, in the worklet's message handler outside `process()`, counted
  with the single-mode control allocations. The SDK no longer starts its spectrum analysis Worker.
- **D5. The SDK API.** Names and result types stay: `querySpectrum`, `subscribeSpectrum`,
  `armSpectrum`, `readSpectrum`, `cancelSpectrum`, `SpectrumResult`, `SpectrumStreamMetadata`.
  Both SDKs gain `configureSpectrum(request: SpectrumQuery | SpectrumCollection, options?:
  { hopFrames?: 256 | 512 | 1024 | 2048 }): Promise<SpectrumConfigured>` with
  `SpectrumConfigured { configuredEpoch: bigint, adoptedEpoch: bigint, rebuild: boolean,
  planEpoch?: bigint }`, a thin builder of `SPECTRUM_CONFIGURE`. A core refusal reaches the caller
  as `MisoEngineError` with the core's code (for example `spectrum.busy`).
- **D6. No SDK check that the core makes.** Delete `validateSpectrumQuery`,
  `validateSpectrumCollectionQuery` and the boundary's entry check: the core refuses an unknown
  entry or mask (`spectrum.entry.unknown`, `spectrum.channels`). Keep only what makes a frame
  encodable (a known target kind, a string ID, a known mask, a finite smoothing number), with
  `MisoUsageError` before any byte.
- **D7. The subscription manager** keeps its handles, timers, callbacks and delivery budgets. Its
  transport hooks call D1's frames. Whether its job sharing moves into the core is #1590's
  question, not this slice's.
- **D8. The old exports stay** in this slice, and no SDK path calls them. #1597 deletes them.

## Deliverables

1. D1 and D5-D7 in `sdk/src/core/`, `sdk/src/browser/`, `sdk/src/headless/`.
2. D2 and D3 in `hosts/host-web/src/` (the preparer, the companions and the Worker boot) and in the
   Worker and worklet scripts.
3. The evals and the qualification rows below.

## Authorized paths

- `sdk/src/**`, `sdk/test/**`.
- `hosts/host-web/src/`, `hosts/host-web/web/`, `hosts/host-web/tests/`,
  `hosts/host-web/qualification/`.
- `crates/control-plane/src/{compile.rs,spectrum.rs}` (D2's parameter removal only).

## Non-goals

- No deletion of a browser export, record or ABI layout row (#1597).
- No change to a frame, a status or the analysis maths.

## Objective gates

Commands: `bash scripts/check-sdk-types.sh`, `bash scripts/check-sdk-headless.sh`,
`bash scripts/check-sdk-generated.sh <artifacts>`, `bash scripts/sdk-package.sh check <artifacts>`,
`cargo test --locked -p host-web --features host-web/test-support`,
`cargo test --locked -p control-plane --features test-support`,
`bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`, and
`npm run qualify -- --artifacts ... --sdk-root ... --browser <b> --check-matrix
--self-test-mutations` in `hosts/host-web/qualification` for chromium, firefox and webkit.

1. **Same bytes.** For a table of requests (each op of both read messages, a single and a
   collection configure with a hop), the SDK encoder's bytes equal the Rust codec's for the same
   request, pinned by one shared fixture that a Rust test and an SDK eval both check (a wire
   format, so a byte fixture is allowed).
2. **The evals.** `spectrum-evals.mjs` and `spectrum-browser-evals.mjs` pass; an expectation
   changes only where the core's code replaces an SDK message, and each such change is listed in
   the PR.
3. **No old export.** A headless eval wraps the instance's exports and fails if a
   `miso_engine_web_v1_spectrum_*` export is called during a one-shot query, a stream subscription
   and a reconfiguration.
4. **Run-time reconfiguration.** While a subscription streams `t1`, `configureSpectrum` adds `t2`:
   `t1`'s stream does not restart (no `gap`, `failed` or `warming`; the same `captureEpoch`), and a
   new subscription on `t2` delivers.
5. **Browser qualification.** The spectrum rows (one-shot, continuous, explicit hop, collection
   switching) pass in chromium, firefox and webkit, on the isolated and the non-isolated leg;
   `miso_engine_web_v1_render_allocation_count` stays 0, and on the non-isolated leg the
   single-mode control allocation count grows with the spectrum reads.
6. **Cross-host parity through the entry.** #1595 gate 5, with the browser leg sending the frames
   through #1562's entry: the bins are equal word for word.

## Test value

- Gate 1 turns red if the SDK and the engine disagree on a field or a code of the three frames.
- Gate 2 turns red if the SDK's public results change for a reason other than the core's codes.
- Gate 3 turns red if any SDK path still uses the browser's own spectrum exports.
- Gate 4 turns red if a run-time configuration restarts a kept stream in the browser.
- Gate 5 turns red if the Worker-side reader or analysis breaks a browser, allocates in render, or
  hides single-mode work.
- Gate 6 turns red if the browser's frame path analyzes differently from the C ABI.

## Dependencies

- #1595, #1594, #1593.
- #1562 (the command entry), #1563 (the frame codec), #1570 (the browser serves every command
  through the core).
- *Run the browser control plane in a Worker and keep the AudioWorklet render-only* (#1332), for
  the two modes.
- #1609 (make the spectrum one-shot handoff safe across threads), root 2026-10-14.

## Supersedes and shrinks

- Amends #1400 D5 and #1381 D3 (the spectrum companion and its swap-block pairing): see the
  amendment entries.

## Decision record

- 2026-10-25, scope (root): split in three. #1596a the browser session serves spectrum (Rust only);
  #1596b the SDK codec and the headless engine; #1596c the browser SDK and the three-browser
  qualification. R1: the old spectrum exports read the session's one service through the existing
  accessors until #1597 (no forwarding export). R2: D2's `compile_children` spectrum parameter
  deletion moves to #1597 (its last callers are the kept boot exports). R3: the browser's 16 KiB
  control frame stays. R4: gate 6 lands in #1596a. R5: D3's "RenderCompanions loses its spectrum
  field" was already true (#1290d2); #1332 is not a dependency (single mode is the product).
- #1596a facts: the core's session had only test-support accessors to its spectrum monitor, so R1
  needs `SessionState::spectrum_monitor`/`spectrum_monitor_mut` in
  `crates/control-plane/src/control.rs` (the two `test_spectrum*` accessors, ungated and renamed).
  #1597 must re-gate (`test-support`) or delete `spectrum_monitor_mut` when it deletes the old
  exports, its only production caller.
- #1596a attempt 2 (verifier MAJOR 1): the core runs its service step, and so promotes, at the
  start of every control command, `service`, `declare_discontinuity`, the event dequeue, both
  session seeks, the source seek (`seek_at`) and the seek report (only the browser's source submit
  runs unserviced). The attempt-1 eager end in `run_control` and `service` missed the event
  dequeue, the anchored session seek, the source seek and the seek report. Root ruling: one lazy
  mechanism. Every use of the spectrum staging (`with_spectrum_staging`) first ends a staging
  stream that the live host's session no longer runs, drops the staged window and writes the
  record from the service with the staging analysis's facts; the `analysis` parameters of
  `service`, `control_command`, `declare_discontinuity` and `session_seek`, and the ffi's
  `with_promotion_analysis`, are deleted. Stated limit until #1597: a stream started through
  `SPECTRUM_STREAM` after a promotion ended the old one is the session analysis's; the staging
  analysis follows only streams the old exports start. The lazy check sees only whether a stream
  runs, so a command that ends the old-export stream and starts a `SPECTRUM_STREAM` stream in one
  call leaves the staging analysis on the new stream. The capture epoch cannot tell the two apart:
  it does survive a carried promotion, but it also advances inside one kept stream on every render
  failure (`continuous_fail`) and every re-selection (`restart_continuous`), so a mismatch would end
  a live staging stream at a failure; and a promotion's fresh capture starts its epochs at 0 again,
  so the new stream's first epoch can equal the old one's. A true stream identity needs a start
  generation in `SpectrumService` (host-core); #1597 deletes the staging analysis, which ends the
  case.
- #1596b facts (attempt 1): the codec is `sdk/src/core/spectrum-frame.ts`, held to
  `crates/protocol/tests/fixtures/spectrum-frames.json` (`spectrum_frames.rs`); no schema change.
  The headless boot still stages its taps (R2's kept path): a headless engine adopts a plan only
  when its caller renders, so taps sent after boot could not be in effect before `createEngine`
  resolves. The boot's one `SPECTRUM_CONFIGURE` is then equal (no change) or retunes the hop
  without a plan, and `adopted_epoch == configured_epoch` at once. Root ruling (2026-10-25): this
  boot choice stands for #1596b; #1597 owns the boot route once the staging exports go, and must
  give a headless boot its taps in the first plan (sample 0 armable). A refused spectrum call raises `MisoEngineError` with
  `code` the result class of the status (as `responseRefusalResult`) and the core's code as
  `diagnosticCode`; a `spectrum.*` `MisoErrorCode` member needs `errors.ts`. A stream reply has
  no floor or token: a stream `SpectrumResult` reports the analysis floor (-120 dB) and
  `snapshotToken = windows`. `resultBytes` keeps the old record's formula. Headless result changes: with no
  taps, `armSpectrum`/`cancelSpectrum` return `invalidArgument` (the core's `NOT_FOUND`,
  `spectrum.unprepared`) where they returned `unsupported`, and `readSpectrum` throws
  `MisoEngineError` (`spectrum.unprepared`) where it threw `MisoUsageError`;
  `readSpectrumStream` throws on every refusal where it returned `{ metadata }` on
  `backpressure` or `renderRejected`; `startSpectrumStream` throws where it returned `ok: false`. Gate 3 is in
  `spectrum-evals.mjs`; the #852 hop-probe boot evals in `boot-evals.mjs` became the bit-16 and
  copied-hop evals.
