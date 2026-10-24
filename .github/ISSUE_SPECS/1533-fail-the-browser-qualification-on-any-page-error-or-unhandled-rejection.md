# Fail the browser qualification on any page error or unhandled rejection

Stream H qualification slice, from the stream B batch-3 B2 diagnosis (#1345, "B2 diagnosis and
fix", finding (b)). Qualification work only: no SDK or engine change. Code anchors verified on
`codex/d15-stream-b3` at `ebe706208`. Verify them again on `main` after #1345 merges, before the
first edit.

## Product outcome

A browser qualification run that has a page error is red, with a named gate, in each of the
Chromium, Firefox and WebKit legs. Today a page can throw, or let a promise reject with no
handler, and the leg is still green if every result gate passes. The error is printed only when
some other gate fails.

## Context

- **How `run.mjs` collects page errors today.** `qualifyBrowser`
  (`hosts/host-web/qualification/run.mjs:898-978`) adds two handlers that push into one
  `diagnostics` array: `console` (`:908`) and Playwright `pageerror` (`:909`). The array is printed
  only in two failure messages: the `browser-execution` gate (`:957-958`) and the "page did not
  finish or offer a control to click" error (`:952`). After `validate` (`:959`) and
  `mutationProofs` (`:960`) the run prints "all qualification gates passed" with no check of
  `diagnostics`. There is no page-side `unhandledrejection` or `error` handler: `index.html` has
  no script, and `qualification.js` is loaded by `page.evaluate` (`:915-928`).
- **The defect this would have caught.** `hosts/host-web/qualification/sdk-response-entry.ts:1098-1108`
  (the deliberate `spectrum-query-closed` check): `pending = closedBrowser.querySpectrum(..)`
  (`:1098`) rejects when `closeSdkEngine` closes the engine (`:1100`), and its `catch` is attached
  only after that await (`:1101-1105`). The rejection is unhandled for one turn. The B2 diagnosis
  saw it as `pageerror: MisoUsageError: the browser engine is closed` in WebKit
  (`/tmp/claude-1002/b2diag/logs/a-unl-webkit.log:22-24`; the same line is in the printed
  diagnostics of 43 WebKit logs in that directory, for example `bar-i1-webkit.log` and
  `bar-i8-webkit.log`), and as a page `unhandledrejection` in all three browsers (the diagnosis'
  own page handler). The diagnostics are printed only on a failure, so no Chromium or Firefox log
  there shows them (none of those runs failed at `browser-execution`). No gate read the error in
  any run.
- **B3 fixes the handler.** Stream B batch 3 attaches the handler before the close (the diagnosis
  checked `pending.then(() => false, () => true)` before the close removes the event in WebKit and
  Chromium, `logs/b-unl-*.log`). This issue is the guard that keeps it fixed.
- **Channels.** Playwright's `pageerror` reports uncaught exceptions of the page's main thread.
  Whether it reports an unhandled rejection in Chromium and Firefox is not verified on this
  stack; WebKit reports it (the logs above). A page-side `unhandledrejection` handler reports it
  in all three (the diagnosis). So the guard reads both channels.
- **The self-test.** `MUTATIONS` (`run.mjs:23-43`), `mutate` (`:728-787`) and `mutationProofs`
  (`:789-824`): each entry is applied to a copy of the real result and `validate` must throw
  `<browser>:`. Entries that do not start with `sdk-` run in every leg (`:790-792`).
- **CI.** `.github/workflows/qualification.yml:445` runs `npm run qualify -- --artifacts ...
  --sdk-root sdk --browser <name> --check-matrix --self-test-mutations` for each browser
  (`:376-385`).

## Decisions frozen for this slice

- **D1. Two channels, one gate.** Before `page.goto` (`run.mjs:914`), `run.mjs` calls
  `page.addInitScript` with a script that defines a non-writable
  `window.__qualificationPageErrors` array and adds `unhandledrejection` and `error` handlers on
  `window` that push one string each (`unhandledrejection: <name>: <message>` from
  `event.reason`, `error: <message> at <filename>:<lineno>`). The `pageerror` handler pushes into
  a new `pageErrors` array as well as into `diagnostics` (`pageerror: <name>: <message>`). No
  change to `qualification.js`, `index.html` or `sdk-response-entry.ts`.
- **D2. When the lists are read.** After the click loop ends and `execution` has settled
  (`:956`), `run.mjs` reads the page list with one `page.evaluate` that waits two `setTimeout(0)`
  turns and then returns a copy of `window.__qualificationPageErrors`. Two turns let a rejection
  that the last microtask checkpoint left unhandled reach its event. It then sets
  `result.pageErrors = { playwright: [...pageErrors], window: [...pageList] }`. A Playwright
  `page.evaluate` has no timeout of its own, so the read is raced against a 10 s timer; at the
  timer the run fails with `<browser>: page-errors: the page did not answer the error read within
  10 s; page diagnostics: ...`. No retry.
- **D3. The rule.** In `validate`, right after `result-schema` (`:161-162`) and before the
  attestation early return (`:171`), a gate `page-errors` passes only if `result.pageErrors` is an
  object, both lists are arrays, and both are empty. Its message is the JSON of both lists. A
  rejection that a later handler takes (the B2 shape) fails the gate: it was unhandled at a
  checkpoint, and that is the defect.
- **D4. Order of failures.** `browser-execution` stays first (`:957`); `page-errors` is the
  second gate of `validate`, so a run with both a page error and a failed result gate names the
  page error.
- **D5. No allow-list.** No message, browser or source is exempt. If an unloaded run of the
  unmutated tree shows a page error other than the B2 one, stop, record it in the Attempt record
  with its log, and report to root. Do not filter it.
- **D6. Not in this slice.** `console.error` messages, errors in a Worker or in the
  `AudioWorkletGlobalScope` (`processorerror`), and HTTP failures stay diagnostics only.

## Deliverables

1. The handlers, the read and `result.pageErrors` in `qualifyBrowser` (D1, D2).
2. The `page-errors` gate in `validate` (D3, D4).
3. Three entries in `MUTATIONS` and `mutate` (gate 2).
4. A new section in `hosts/host-web/MUTATIONS.md`: one row per result mutation and one row per
   page mutation of gate 3, with the observed red, in the format of the #1476 rows.
5. This spec's Attempt record: the runs of gates 1 and 4.

## Authorized paths

- `hosts/host-web/qualification/run.mjs`
- `hosts/host-web/MUTATIONS.md` (the new section only)
- This spec.
- Nothing else. A page error on the unmutated tree is a finding (D5), not a reason to edit the
  page code.

## Non-goals

- No fix of any page error. The B2 handler fix is B3's (Dependencies).
- No `console` gate and no Worker or worklet error gate (D6). These go to a named successor (stream
  H, after this slice): **"Fail the browser qualification on Worker and worklet errors and on
  `console.error`"**. Its scope:
  - a `processorerror` event on every `AudioWorkletNode` the page creates (the shipped worklet,
    the PCM-feed worklet and the SDK's nodes);
  - an `error` or `messageerror` event on every page Worker (the SDK's scratch, response and
    spectrum Workers);
  - `console.error` messages, with a decision on noise that a browser logs by itself.

  Root files it when this slice closes. Until then, these errors stay in `diagnostics` only.
- No change to `normalizedRow`, `results.json` or `BROWSER_DEPLOYMENT_MATRIX.md`: the matrix row
  does not change.
- No retry and no wait beyond the two turns of D2.

## Objective gates

1. **`page-errors` green on the real runs.** The CI command passes with an empty
   `result.pageErrors` in Chromium, Firefox and WebKit, 3 unloaded runs each, with the B3 handler
   fix on the tree.
   *Test value:* this is the baseline that makes gates 2 and 3 mean something; on its own it
   shows that no other page error hides in today's green runs (D5).
2. **Result mutations (`--self-test-mutations`).** Each must make `validate` throw with
   `<browser>: page-errors`:
   - `page-errors`: push `"pageerror: Error: planted"` into `pageErrors.playwright`;
   - `page-errors-window`: push `"unhandledrejection: Error: planted"` into `pageErrors.window`;
   - `page-errors-missing`: delete `result.pageErrors`.
   *Test value:* turns red if the gate stops reading one of the two channels, or passes a result
   that `run.mjs` never filled (a refactor that drops D2's assignment), which a green real run
   cannot show.
3. **Page mutations (PR evidence, rows in `MUTATIONS.md`; not committed tests).** Apply one at a
   time, run the CI command unloaded in all three browsers, record the red gate and its message,
   revert:
   - P1: restore the B2 order in `sdk-response-entry.ts:1098-1108` (the `catch` attached after
     `closeSdkEngine`) -> `page-errors` red with `the browser engine is closed` in every browser.
     A browser where it stays green is recorded with the channel that missed it.
   - P2: P1, with the `addInitScript` handler removed -> record per browser whether the
     Playwright channel alone turns it red. This is the evidence for D1's second channel.
   - P3: add `void Promise.reject(new Error("planted"));` as the first line of
     `runQualification` in `qualification.js` -> `page-errors` red in all three browsers, also in
     a run without `--sdk-root`.
   *Test value:* P1 turns red if the B2 ordering defect, or any rejection a later handler takes,
   comes back; today the WebKit leg passes with it whenever no other gate fails.
4. **Load.** 5 WebKit runs with 32 busy loops on 32 CPUs (the B2 method,
   `/tmp/claude-1002/b2diag/load.sh`). A run that fails `page-errors` is a failed gate. A run that
   fails only in another gate is recorded and counted apart.
5. `bash scripts/check-workspace-policy.sh` passes.

## Dependencies

- **B3's handler fix in `sdk-response-entry.ts:1098-1108` on `main`.** Without it gate 1 is red
  in every WebKit run (and in the other browsers if their channel reports it). This is a hard
  prerequisite.
- **#1345 on `main`.** Its `observation-unsubscribe` gate and its `run.mjs` edits are on
  `codex/d15-stream-b3` only.
  Root note (2026-10-13): the real-browser observation-unsubscribe gate was removed by #1345 Amendment 7 and moves to #1535.
- **Same file as #1522 and #1530** (`run.mjs`): in any order, the later slice rebases.

## Placement

**Stream H**, after B #1345 (with B3's handler fix) reaches `main`. `run.mjs` and `MUTATIONS.md`
are stream H's (`STREAMS.md:108`, `:110`); the hot-file rows gain "H #NNNN (the `page-errors`
gate, its handlers and three mutations)" and "H #NNNN (its own rows)". In any order with H #1522
and H #1530; the later slice rebases. It needs no named exception.

## Attempt record

**Attempt 1 (2026-10-24, on `main` at `1eedbf419`).** Scope note `scope/1533.md` accepted by root
(R1): the anchors moved (`qualifyBrowser`, `validate`, `MUTATIONS`, `mutate` re-verified); D1-D5
implemented as written. One refinement of D2/D4: the read's evaluate error is caught too, and a
timeout or read failure is raised after the `browser-execution` gate, so a dead page names its own
error first; `result.pageErrors` is set only when the read answered, so `page-errors-missing`
models a dropped read.

- Gate 1 (one unloaded run per browser, R1; not three): green in Chromium 151.0.7922.34, Firefox
  153.0, WebKit 26.5 (Playwright 1.62.1), with `--sdk-root --check-matrix
  --self-test-mutations` and the locally built artifact. No page error on the unmutated tree (D5).
- Gate 2: the three result mutations are red in all three legs (`<browser>: page-errors:`).
- Gate 3: P3 red in all three browsers (WebKit also without `--sdk-root`). Both channels caught
  it in every browser: Playwright 1.62.1 reports an unhandled rejection as `pageerror` in Chromium
  and Firefox too (WebKit prefixes `Unhandled Promise Rejection:`). P1 and P2 move to #1605.
- Gate 4 (5 loaded WebKit runs) moves to #1605 (sprint mode, root R1).
- Gate 5: `bash scripts/check-workspace-policy.sh` passes.
