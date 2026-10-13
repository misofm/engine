# Qualify the browser observation wait against the real AudioWorklet in three browsers

Successor of #1345 (decision 15, stream B batch 3), from the attempt-3 verdict's finding m2
(`submix-verdicts/1345-attempt3.md`). Qualification work only: no SDK or engine change.
Code anchors verified on `codex/d15-stream-b3` at `ca5c5e60d` (the #1345 tree). Verify them again
on `main` after #1345 merges, before the first edit.

## Product outcome

The browser SDK's wait on observation backpressure (#1345 D2 Amendment 2) is proven against the
shipped AudioWorklet in Chromium, Firefox and WebKit, not only against a scripted host. In the
`browser` qualification legs:

1. On a context that does not render, an awaited managed `close()` of a just-armed tap ends in the
   typed `MisoEngineError` code `backpressure` at the engine's `requestDeadlineMs`.
2. After the context renders, the same `close()` resolves, and the worklet's status reports the
   rendered blocks in `renderedQuanta`.
3. A `close()` that was refused before a render resolves by itself when the context starts to
   render: the SDK sees `renderedQuanta` move and sends the same records again one time, and that
   resend is admitted.

## Context

- **The contract.** #1345 Amendment 4, D2 Amendment 2
  (`.github/ISSUE_SPECS/1345-hold-effect-parameter-bypass-and-eq-target-values-in-latest-target-cells.md:333-344`):
  the browser engine waits for the next render and sends again only on `backpressure`, bounded by
  `requestDeadlineMs`; at the deadline it ends in the same typed `backpressure` error.
- **The SDK wait.** `sdk/src/core/observation-subscriptions.ts:143-158` (the transport's
  `renderWait: { boundMs, next(timeoutMs) }`) and `:879-913` (`#submit`: a resend only on
  `reasonName === "backpressure"`; one deadline fixed at the first refusal, `deadline ??=` at
  `:907`; `next(remaining)` returns `false` -> the typed refusal is thrown; at most 8 resends).
- **The browser side of the wait.** `sdk/src/browser/engine.ts:999-1032`: `boundMs` is
  `options.requestDeadlineMs ?? 5_000` (`:1009`); `next` reads `host.status().renderedQuanta`, then
  polls it every 4 ms and returns `true` when it is larger (`:1022-1028`), `false` at
  `timeoutMs` or when the engine is closed. The public doc is `:204-213`.
- **Where commands go.** `sdk/src/browser/live-controls.ts:57` sends each managed submission as one
  `host.command({ commands })`. `command` is a prototype method of the shipped host
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1329-1338`), and the host instance is a
  plain `new MisoAudioWorkletHost(...)` (`:1747`), not frozen. `status()` is `:1309-1310`.
- **What the worklet reports.** The status reply is built on the audio thread in the worklet's
  message handler (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:1076-1084`) from the status
  view (`:945-960`, `renderedQuanta` at offset 40). `render_next`
  (`hosts/host-web/src/lib.rs:3258-3290`) adds 1 to `rendered_quanta` after
  `ready.host.plan.render(..)` returns, that is, after the block's observation drain. Today the
  status read and command admission both run on the audio thread between quanta, so "a moved
  `renderedQuanta` means the slot is drained" holds by construction. H #1332 and H #1562 move
  admission and status into a Worker, where it can stop being true.
- **Codes.** `backpressure` is result 6 and command reason 8
  (`sdk/assets/miso-engine-v1-abi-layout.json:620,626`).
- **The only test today.** `sdk/test/observation-subscription-evals.mjs:475-660` runs the real
  `createEngine` over a scripted host that writes its own `renderedQuanta` and frees its slot in
  the same step (cases at `:558`, `:574`, `:644`). Its comment at `:479-480` says the CI browser
  legs run the same case; they do not yet. This issue makes that true.
- **The qualification.** `hosts/host-web/qualification/sdk-response-entry.ts`:
  `createSdkEngine` (`:79-91`, the only `createEngine` call site, #1476 D1),
  `createResidentObservationBrowser` (`:1564-1596`, an `OfflineAudioContext` of
  `OBSERVATION_FRAMES = 2_048` frames, injected `scratchBoot`, the gate fixture
  `observationDocumentWithGate`), `runResidentObservationQualification` (`:1598-1735`; it
  subscribes before `startRendering()` at `:1637`, so the shipped worklet admits commands while an
  offline context has not started, in all three browsers), and `runSdkObservationQualification`
  (`:1737-1738`, which calls the resident flow first; its result is `observations` at `:2145`).
  The resident flow makes no same-tap operation before its render, so the new bound does not
  change it (verdict m2).
- **The runner.** `hosts/host-web/qualification/run.mjs`: the result-mutation list `MUTATIONS`
  (`:23-43`), the frozen SDK instance labels `SDK_RENDER_ALLOCATION_WORKLOADS` (`:46-65`, checked
  in order by `validateSdkRenderAllocations`, `:308-323`), the resident gates (`:484-523`),
  `mutate` (`:707-764`) and `mutationProofs` (`:766-799`). `--self-test-mutations` runs every
  `MUTATIONS` entry against a copy of each browser's real result and requires the validator to
  throw. CI runs `npm run qualify -- --artifacts ... --sdk-root sdk --browser <name> --check-matrix
  --self-test-mutations` (`.github/workflows/qualification.yml:445`); `--sdk-root` bundles
  `sdk/src`, so an SDK source mutation reaches the browser run (the #1480 rows of
  `hosts/host-web/MUTATIONS.md` did this).
- **Injected boot.** With an injected `scratchBoot`, `requestDeadlineMs` bounds no boot step
  (`sdk/src/browser/engine.ts:516-523`); in these instances it bounds only the observation wait.
- **Local browsers.** Playwright and Chromium, Firefox and WebKit binaries are installed on the
  development box (`hosts/host-web/qualification/node_modules/playwright`,
  `~/.cache/ms-playwright`). If the pinned revision is missing, `npx playwright install <name>`.

## Decisions frozen for this slice

- **D1. Not rendering = an `OfflineAudioContext` before `startRendering()`.** It renders nothing
  until `startRendering()` in every browser, needs no user gesture, and has no autoplay policy. The
  "resume" is `startRendering()`. No real-time `AudioContext`, no `suspend()`/`resume()`, no
  `OfflineAudioContext.suspend(time)` (not in every browser). An offline context renders once, so
  each case that needs a context that has not rendered gets its own instance.
- **D2. Two new SDK instances**, both made by `createResidentObservationBrowser` with three new
  optional arguments (`workload`, `requestDeadlineMs`, a command log); the existing call keeps its
  defaults and its behaviour.
  - `observation-wait-deadline`: `requestDeadlineMs: 250`.
  - `observation-wait-render`: `requestDeadlineMs: 10_000`.
  Both are run by one new function, `runObservationWaitQualification`, called in
  `runSdkObservationQualification` right after `runResidentObservationQualification`. Its result
  goes to `observations.wait = { deadline, render }`. Both labels go into
  `SDK_RENDER_ALLOCATION_WORKLOADS` right after `"resident-observation"`, in that order (their
  counts are read before `track-response-subscription`'s). Each instance is closed only by
  `closeSdkEngine`/`finishSdkEngine`.
- **D3. The subscription.** One selection, `observationSelection("comp")`, `windowBlocks: 2`,
  `cadenceMs: 60_000` (the bound's maximum, so the cadence timer never reads during the case), no
  `onUpdate`. Submit the 16 source blocks as `runResidentObservationQualification` does
  (`:1624-1636`; a shared helper is allowed) before any render.
- **D4. The command log.** In the `createHost` callback, after `createDefaultHost` resolves, set an
  own `command` property on the host instance that calls the prototype method, appends
  `{ result, reason, records }` (`records` is a bigint-safe JSON string of `request.commands`) and
  returns the ack unchanged. Nothing else in the host is wrapped.
- **D5. Time.** `performance.now()` just before `close()` is called and when it settles. Every
  awaited `close()` is raced against a guard timer (deadline instance 10_000 ms, render instance
  15_000 ms); a guard that fires records `settled: false` and the case goes on, so a defect gives a
  named red gate, never a page timeout.
- **D6. Bounds that cannot flake.** The deadline case passes when
  `225 <= elapsedMs < 4_000`. The lower bound is the 250 ms deadline less 25 ms for clock grain
  (the SDK fixes the deadline after the first refusal's round trip, so the true elapsed time is
  above 250). The upper bound is far above 250 ms of timer delay under load and below the SDK's
  5_000 ms default, so a bound that ignores `requestDeadlineMs` is red. The render instance's
  10_000 ms deadline leaves the offline render about 100 times the time it needs to start.
- **D7. Exact counts.** An `OfflineAudioContext` of 2_048 frames renders 16 quanta of 128, so
  `renderedQuanta` reads `"16"` after `startRendering()` resolves. If a browser reports another
  count, stop and report it to root; do not loosen the gate.
- **D8. What the offline render can and cannot show.** The offline render runs its 16 blocks in a
  burst. Gate 3 shows that the real status moves and that the first resend after it is admitted.
  It does not show a skew of one block between the status and the drain inside that burst. That
  needs a real-time context and is not in this slice (Non-goals; open question for root).

## Deliverables

1. `runObservationWaitQualification` in `sdk-response-entry.ts` and its two instances (D1-D5).
   - **Deadline instance.** Subscribe (`comp`); read `renderedQuantaBefore`; `await close()` and
     record `settled`, `typed` (`error instanceof MisoEngineError`), `errorCode`, `elapsedMs`;
     read `renderedQuantaAfterRefusal`; record `handleOpenAfterRefusal` (`readLatest()` does not
     throw); `await browser.context.startRendering()`; `await close()` again and record
     `closeAfterRender` (`resolved`, `elapsedMs`); read `renderedQuantaAfterRender`; record the
     command log.
   - **Render instance.** Subscribe (`comp`); read `renderedQuantaBefore`; call `close()` without
     awaiting it; wait (at most 5_000 ms, polling the log every 1 ms) until the log holds the
     close's first refusal, record `refusalSeenBeforeRender`; call `startRendering()`; await the
     close (D5 guard) and record `settled`, `resolved`, `elapsedMs`; read
     `renderedQuantaAtResolve` right after; await the render; read `renderedQuantaAfterRender`;
     record the command log and `resendIdentical` (the refused close's `records` equal the
     admitted close's).
2. The `sdk-observation-wait` gate in `run.mjs` (the predicates of gates 1-3 below), the two
   labels in `SDK_RENDER_ALLOCATION_WORKLOADS`, and the eight result mutations of gate 4 in
   `MUTATIONS` and `mutate`.
3. A new section in `hosts/host-web/MUTATIONS.md`: one row per result mutation (gate 4) and one row
   per source mutation with its observed red (gate 5), in the format of the #1476 and #1480 rows.
4. This spec's Attempt record: the soak counts (gate 6) and the per-browser
   `sdk-render-allocations` lines that now name the two instances.

## Authorized paths

- `hosts/host-web/qualification/sdk-response-entry.ts`
- `hosts/host-web/qualification/run.mjs`
- `hosts/host-web/MUTATIONS.md` (the new section only)
- This spec.
- Nothing else. A gate that is red on the unmutated tree because of the SDK or the engine is a
  finding: stop, record it, and report to root for an amendment. Do not change `sdk/`,
  `hosts/host-web/web/`, `hosts/host-web/src/` or `crates/`.

## Non-goals

- No SDK or engine change, and no change to the scripted-host evals. The verdict's m1 (a test that
  one deadline spans all resends), NIT 2 (an engine close ends the wait) and m3 (the README
  sentence) are not in this slice.
- No real-time `AudioContext` case and no one-block ordering proof (D8).
- No change to the resident flow's own steps or gates, to `BROWSER_DEPLOYMENT_MATRIX.md`,
  `results.json` or `normalizedRow` (the matrix row has no per-gate SDK entry, so `--check-matrix`
  does not move).
- No fix for #1522's Firefox spectrum failures.
- No headless case (the #1345 headless evals hold that contract).

## Objective gates

Gate names are the `gate(...)` names in `run.mjs`. "Command log" entries are
`{ result, reason }` in send order; `ok` is result 0, `bp` is result 6 with reason 8.

1. **`sdk-observation-wait`, deadline.** `settled === true`, `typed === true`,
   `errorCode === "backpressure"`, `225 <= elapsedMs < 4_000`,
   `renderedQuantaBefore === "0"` and `renderedQuantaAfterRefusal === "0"` (the context did not
   render, so the refusal is the wait's and not a render's), `handleOpenAfterRefusal === true`, and
   the log before the render is exactly `[ok, bp]` (one close send, no resend without a render).
   *Test value:* turns red if, on the shipped worklet, the wait ends early (a status reply whose
   `renderedQuanta` moves with no render, so the owner resends into a slot no block drained and
   ends in `backpressure` within milliseconds) or ends late or never (a bound that ignores
   `requestDeadlineMs`); the scripted host writes its own `renderedQuanta`, so no existing test
   reads the real reply or a real browser's timers.
2. **`sdk-observation-wait`, after the render.** On the deadline instance, `closeAfterRender.resolved
   === true`, `renderedQuantaAfterRender === "16"`, and the whole log is exactly `[ok, bp, ok]`.
   *Test value:* turns red if the shipped worklet's status does not report the blocks the context
   rendered (`renderedQuanta` stays 0 or counts something else), which would end every browser
   wait on a live context in `backpressure` at the deadline, or if a refused close leaves the
   worklet's tap slot unusable after a drain; the scripted host fabricates both.
3. **`sdk-observation-wait`, the wait renders.** On the render instance,
   `refusalSeenBeforeRender === true`, `settled === true`, `resolved === true`,
   `elapsedMs < 10_000`, `renderedQuantaBefore === "0"`, `renderedQuantaAtResolve` is above `"0"`,
   `renderedQuantaAfterRender === "16"`, the log is exactly `[ok, bp, ok]`, and
   `resendIdentical === true`.
   *Test value:* turns red if the browser owner does not resend after a real render, or if a block
   becomes visible in `renderedQuanta` before its observation drain (the first resend is refused
   again, so the log holds two `bp`); today this order holds by construction on the audio thread,
   and the gate keeps it when H #1332 and H #1562 move admission and status into a Worker; the
   scripted host frees its slot in the same step that moves its count, so it cannot show the order.
4. **Result mutations (`run.mjs --self-test-mutations`).** Each entry is added to `MUTATIONS`,
   made in `mutate`, and must make `validate` throw with `<browser>: sdk-observation-wait`:
   - `sdk-observation-wait-unbounded`: `wait.deadline.settled = false`;
   - `sdk-observation-wait-late`: `wait.deadline.elapsedMs = 5_000`;
   - `sdk-observation-wait-early`: `wait.deadline.elapsedMs = 0`;
   - `sdk-observation-wait-untyped`: `wait.deadline.errorCode = "invalidArgument"`;
   - `sdk-observation-wait-rendered`: `wait.deadline.renderedQuantaAfterRefusal = "1"`;
   - `sdk-observation-wait-after-render`: `wait.deadline.closeAfterRender.resolved = false`;
   - `sdk-observation-wait-resend`: a second `bp` entry inserted before the last `ok` of
     `wait.render`'s log;
   - `sdk-observation-wait-quanta`: `wait.render.renderedQuantaAtResolve = "0"`.
   *Test value:* turns red if a predicate of gates 1-3 is dropped or weakened, which the real
   green result alone cannot show.
5. **Source mutations (PR evidence, rows in `MUTATIONS.md`; not committed tests).** Apply one at a
   time in `sdk/src`, run the qualify command in Chromium, record the red gate and its message,
   revert. Run S2 and S3 also in Firefox and WebKit.
   - S1: delete the `limit` timer in `next` (`browser/engine.ts:1020`, `setTimeout(() =>
     finish(false), timeoutMs)`; keep `limit` defined so it compiles) -> gate 1 red (`settled: false` at the guard).
   - S2: `boundMs: 5_000` regardless of `requestDeadlineMs` (`:1009`) -> gate 1 red (late).
   - S3: in `#submit`, throw whatever `next` returns, so it never resends
     (`observation-subscriptions.ts:912`, drop `if (!rendered)`) -> gate 3 red.
   - S4: `current.renderedQuanta >= started` in `next` (`:1028`, a wait that returns with no
     render) -> gate 1 red (early, and more than one close send in the log).
   - S5: the browser engine passes no `renderWait` (`:1008-1032` removed) -> gates 1 and 3 red.
   Each red must name `sdk-observation-wait`. A mutation that stays green is reported, not hidden.
6. **Soak.** 5 consecutive passes of the full leg (`--check-matrix --self-test-mutations`) in each
   of Chromium, Firefox and WebKit, and 10 Firefox passes with 24 CPU hogs running (the #1522
   method). A run that fails only in a gate #1522 owns (`sdk-spectrum-*`) is recorded and counted
   apart; any `sdk-observation-wait` failure is a failed soak.
7. `bash scripts/check-workspace-policy.sh` passes. `sdk-render-allocations` stays green with 19
   rows in the frozen order, all 0.

## Dependencies

- **#1345 on `main`.** The browser `renderWait` and the one-slot-per-tap observation queue exist
  only on `codex/d15-stream-b3` today.
- **Before H #1332 and H #1562.** Gate 3 exists to hold the order those two slices can break.
- **Same files as #1522** (`sdk-response-entry.ts`, `run.mjs`): in any order, the later slice
  rebases. #1476 and #1480, the files' hot-file row entries, are on `main`.

## Placement

Recommended: **stream H**, as a qualification slice after B #1345 reaches `main` and before H
#1332. Reasons:
- The three files are stream H's (`hosts/host-web`), and their hot-file rows in `STREAMS.md` name
  only H and J slices (`sdk-response-entry.ts`/`run.mjs`: H #1476, J #1480;
  `MUTATIONS.md`: H, A #1479 row `:563`, J). In H the slice needs no named exception, and the
  rows only gain "H #NNNN".
- "Before H #1332" is then an order inside one stream, which the H coordinator controls, and not a
  cross-stream constraint like B #1524's and B #1525's.
- Stream B's batch 3 branch is in active edit and its queue after #1345 (#1523, #1524, #1525) is
  already set; this slice changes no B-owned file.
- The cost is a wait for #1345 on `main`, which is the dependency in any case.

Alternative: stream B, in batch 3 right after #1345, with a named exception for the three paths
and a "B #NNNN" entry in both hot-file rows. Choose it only if root wants m2 closed in the same
batch merge as #1345.
