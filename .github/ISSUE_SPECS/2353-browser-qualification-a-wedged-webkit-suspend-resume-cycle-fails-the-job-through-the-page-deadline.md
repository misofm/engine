# Browser qualification: a wedged WebKit suspend/resume cycle fails the job through the page deadline, though the owner ruled the row reported, not gated

## Problem

The owner ruled on #2245 (2026-11-05) that Chromium gates the `suspend-resume` row and that WebKit reports it, "reported, not gated". In CI, though, a wedged WebKit resume still fails the required `qualification` check. It never reaches the row's own reporting: the page stops offering its next declared control, `run.mjs`'s 300 s `PAGE_DEADLINE_MS` fires, and the job fails with empty page diagnostics:

```
Error: webkit single: the page did not finish or offer a declared control to click (button[data-qualification-resume="suspend-resume cycle N again"]; K of 25 clicks done) within 300000 ms: total deadline; page diagnostics:
```

Seen on 2026-11-07 alone, always on "webkit single", at different cycles. Each run passed on a re-run, and none of the PRs changed this path:

| Run / job | Cycle | Clicks |
|---|---|---|
| 42371462688 / 128401370496 (PR #2341) | 8 | 22/25 |
| 42371462688 / 128405310447 (PR #2341, re-run) | 4 | 14/25 |
| 42378815500 / 128422903388 (PR #2347) | 2 | 10/25 |

It was also seen earlier on PRs #2294, #2325 and main d441cd408.

The PR #2341 investigation found that the page sat silent for about 5.5 minutes before the cycle's second click. That is not a slow render: every in-cycle wait is bounded at 10 s, and a render stall would have been recorded as a stall signature. So the wedge sits where the page waits for the resume to take, outside the bounded waits, and nothing reports what it was waiting on.

## Scope

- Find where a WebKit cycle can wait without bound: the page's resume handling in `hosts/host-web/qualification/qualification.js` `runSuspendResumeCycle`, or `run.mjs`'s click plan.
- Bound that wait. On WebKit, a wedged cycle must end as a reported stall, per the ruling: the row records it, the remaining cycles and rows still run, and the job does not fail on it.
- When the page deadline does fire, the page names what it was awaiting, so the diagnostics are not empty.

## Non-goals

- No change to the Chromium gate: it stays gated and must still fail on a stall.
- No engine change unless the bounded wait proves the engine is at fault.

## Gates

- A mutation that wedges a WebKit cycle (a resume that never resolves) ends as a reported stall and does not fail the job. The same mutation on Chromium fails the gate.
- 20 consecutive WebKit single and worker runs with no job failure from this row.
- `--self-test-mutations` and the hermetic web tests pass.

## Diagnosis (attempt 1, 2026-11-07)

**Where the page waits without bound.** Nowhere in the page's code: every wait of a cycle is
`bounded` (10 s), `requireSuspended` (10 s) or `renderUntil` (10 s), and each throws a named
timeout. The click plan clicks only after the page offers a control. A bounded wait whose timer
cannot fire is the only way to the 300 s deadline: the page's **event loop stopped**. Then no timer
fires, no evaluate answers (so the harness could read no diagnostics), and the page offers nothing.

- All three CI failures stopped between a cycle's first click and its second: clicks done =
  5 (3 SDK, growth, unsubscribe) + 2N + 1 for cycle N (22 for cycle 8, 14 for cycle 4, 10 for
  cycle 2). The steps there are the first segment's render wait, `context.suspend()`, the
  suspended poll and a status read.
- #2245 already recorded that a `suspend()` on Linux WebKit, on a context whose output has
  stalled, hangs the whole page, and the cycle avoids calling one after the second resume for that
  reason. Its first-segment suspend is still exposed: the first resume's output can stall after
  its first burst, as the second resume's does.
- Local probes (WebKit 26.5 WPE, GStreamer, PulseAudio null sink; scratch, not committed):
  - With a broken GStreamer sink, a click whose handler calls `resume()` never returned to
    Playwright (`click` timed out at "performing click action"): WebKit's audio destination can
    block the page's main thread synchronously.
  - A console message logged just before the page's event loop blocks reaches the harness
    (`console` event 240 ms after start, then nothing); `browser.close()` on that page returns
    after 11 s, and a new browser launches normally.

So the bound must be the harness's, and the record of what the page awaited must already be out of
the page when it stops.

## Decision

1. **The page says what it awaits.** Every wait primitive (`bounded`, `requireSuspended`,
   `renderUntil`) logs `qualification awaiting: <step>` once per change of step, and a
   one-second `qualification heartbeat` shows the event loop still runs. `run.mjs`'s
   `pageWatch` keeps the last of each (they are left out of the page diagnostics), and every page
   deadline message now names them: `the page last logged that it awaited "<step>", N ms ago, and
   its last heartbeat N ms ago`.
2. **The row runs one cycle per evaluate, after the page's other rows.** `runSuspendResumeQualification`
   leaves `runQualification`; `runSuspendResumeCyclePage(cycle)` is one cycle, with its own
   two-click plan (`suspendResumeCycleClickPlan`). A page that stops can lose only the cycle in
   flight: every earlier row and cycle has already returned.
3. **A wedge is bounded and recorded, never a hang.**
   - In the page: a timeout from the cycle's first click on (a resume that never resolves, a
     suspend that never settles, a first segment that never renders, a dispose or close that never
     returns) is the cycle's `wedge` (`by: "page"`, the step and the message). The cycle still
     reads its render-allocation count, disposes its host and closes its context, each bounded.
     A timeout before the first click still fails the leg, as before.
   - In the harness: once a cycle's first control is clicked, a page that logs nothing (no step,
     no heartbeat) for `SUSPEND_RESUME_SILENCE_MS` = 20 s, or runs one cycle past
     `SUSPEND_RESUME_CYCLE_DEADLINE_MS` = 120 s, is the cycle's `wedge` (`by: "harness"`, with the
     step it last awaited and both ages). The harness closes that browser and runs the next cycle
     on a new page in a new browser. A page that stops before the cycle's first click fails the
     leg by name. A healthy cycle takes about 3 s, a stalled one about 13 s (measured on WebKit).
4. **`wedge` is a row check outside the setup checks.** `suspendResumeFailures` adds `wedge` and
   holds a wedged cycle to nothing past the steps it reached (a page record still to `context`).
   So Chromium fails on it, and WebKit reports it (owner ruling 2026-11-05). The per-cycle host
   mode, render-allocation count and resumes are held by the `instance-mode`,
   `render-allocations` and `resume-clicks` gates as before, from the cycle's own record; a cycle
   whose page the harness lost has none, and only its `wedge` speaks for it.
5. No engine change: the engine is not implicated (the #2245 no-engine repro stands).

## Evidence (attempt 1)

Local stack: Ubuntu 24.04, Playwright 1.62.1, WebKit 26.5 (`webkit-2336`), Chromium
151.0.7922.34, a private PulseAudio null sink at 48 kHz as in CI. The WebKit system libraries were
unpacked into a scratch prefix (`apt-get download` + `dpkg -x`, no system change). Artifacts built
from this tree by `scripts/build-web-audioworklet.sh`; no Rust changed. Each run is the CI command
(`--check-matrix --self-test-mutations`, `--sdk-root` the tree's `sdk`).

Two scratch mutations (not committed), applied to the base and to the fix with the same text:

- **frozen**: cycle 3's first-segment `suspend()` call first blocks the page's event loop for 400 s,
  the CI symptom.
- **resume**: cycle 3's first click calls `resume()`, but the page never sees it resolve (the
  issue's mutation).

### Red on base, green on the fix (one run each, WebKit single unless named)

| Tree | Mutation | Browser | Result |
|---|---|---|---|
| base `8e76a1ae3` | frozen | WebKit | **fails** after 337 s, exactly as CI: `the page did not finish or offer a declared control to click (button[data-qualification-resume="suspend-resume cycle 3 again"]; 12 of 25 clicks done) within 300000 ms: total deadline; page diagnostics: ` (empty) |
| base | resume | WebKit | **fails** (75 s): `webkit single: browser-execution: ... suspend-resume qualification failed: resume timeout at suspend-resume cycle 3 first (click) after 10000 ms` (the row throws; nothing after it runs) |
| fix | frozen | WebKit | **passes** (116 s). `suspend-resume cycle 3: the page stopped after 1 of 2 clicks (the page logged nothing for 20165 ms; the page last logged that it awaited "suspend the first segment's suspend (suspend-resume cycle 3)", 20165 ms ago, and its last heartbeat 20165 ms ago); recorded as the cycle's wedge, and the next cycle runs in a new browser`. The row reports `1 wedged (cycles 3)`, and cycles 4 to 9 ran in the new browser; every later gate passed. |
| fix | frozen | Chromium | **fails** (66 s): `chromium single: suspend-resume: wedge: ...` |
| fix | resume | WebKit | **passes** (152 s). Cycle 3's page record: `{"by":"page","awaiting":"resume suspend-resume cycle 3 first (click)","message":"resume timeout at suspend-resume cycle 3 first (click) after 10000 ms: ..."}`; the other nine cycles ran (7 stalled after the second resume, as #2245 records). |
| fix | resume | Chromium | **fails** (52 s): `chromium single: suspend-resume: wedge: ...` (the same page record) |
| fix | none | WebKit | passes (164 s): 9 of 10 cycles stalled after the second resume, all full signature, 0 wedged |
| fix | none | Chromium | passes (44 s): 0 stalled, 0 wedged |

### Mutation proofs of the fix
- `--self-test-mutations`: new table rows (`wedged in the page`, `... context running`, `... at the close`,
  `wedged by the harness`); the record mutation `suspend-resume-wedged` (a harness-wedged last cycle,
  its render-allocation row gone) is caught by `suspend-resume` on Chromium, and held green by
  `assert.doesNotThrow` on WebKit. That green proof caught a real gap during attempt 1: the first
  version of the mutation left the cycle's render-allocation row in place and the
  `render-allocations` count refused it, so the mutation now models what a lost page leaves.
- Self-mutations (scratch, reverted): dropping `failures.add("wedge")`, and letting a wedged cycle
  fall through to the later checks (`if (wedged) continue;` removed), each turn the table red
  (`suspend-resume: table row "wedged in the page"`) before any browser starts.

### Test value
- Table rows: a `suspendResumeFailures` that stops naming a wedged cycle, or that holds a wedged cycle
  to steps it never reached (on WebKit, a setup check such as `suspended` is gated, which would fail
  the job), turns them red.
- `suspend-resume-wedged` (Chromium, red): a harness wedge that is filtered out on Chromium too.
  (WebKit, green): any gate (`instance-mode`, `render-allocations`, `resume-clicks`) that still
  demands a lost cycle's host, count or clicks, which would fail the WebKit job the ruling reports.
- Nothing is deleted: the earlier rows still each defend one check.

### Other gates
- `--check-matrix --self-test-mutations`: Chromium single and WebKit single pass (the runs above).
  The worker legs were not run locally (CPU budget); they share every changed line with the single
  legs (the cycle evaluate, the wedge bounds and the gates are leg-independent).
- `scripts/test-web-audioworklet.sh` (hermetic web tests): pass; the boot-contract test logs no
  awaited step (the heartbeat timer is `unref`'d in Node).
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- No Rust, SDK or artifact change.

### Not measured here
- The issue's 20 consecutive WebKit single and worker runs. Each WebKit leg takes about 2.5 min
  here under CPU pressure, so 40 legs were not run locally; the count is left to CI on this
  branch's PR (the WebKit legs run on every PR) and recorded there.

### Observed in passing (not acted on)
- In the frozen run, the six cycles that ran in the fresh browser after the wedge stalled 0 times
  after the second resume, against 1 of 4 before it and 7 to 9 of 10 in the unmutated runs, where
  every cycle shares the page that ran the other rows. One run; it may say that WebKit's stall
  rate depends on how long the page has been rendering. Not a gate here.
