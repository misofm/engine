PASS

# #1476 attempt 1 follow-ups: adversarial verdict

Commit under review: `45b87c94a`. Its direct parent is `ea2def177` (#1481's follow-ups, one spec
file), and `ea2def177`'s parent is `93987443c`. I reviewed `git show 45b87c94a`. The range
`93987443c..45b87c94a` also contains #1481's spec edit, which is not part of this review.
Worktree `/home/bl/misofm/wt-d15-misc`, not touched. Exported with `git archive` into
`/tmp/claude-1002/v1476fu/tree`. Verifier: opus-xhigh, 2026-10-08. Logs:
`/tmp/claude-1002/v1476fu/logs/` and `/tmp/claude-1002/v1476fu/plant/`.

No BLOCKER, no MAJOR and no MINOR. Two NITs. The commit touches only
`hosts/host-web/qualification/sdk-response-entry.ts` and the #1476 spec (authorized paths).

## Checks

### n4: `createEngine` is called at one site

`createEngine(` appears once, in `createSdkEngine` (`sdk-response-entry.ts:78`). The other
references are the import (`:11`), the two types (`:65-66`) and the doc comment (`:60`). All seven
builders pass their options to `createSdkEngine`. Each builder's `node.connect` was removed, and
the one connect in `createSdkEngine` (`:82`) replaces them. The order (create, then connect) is the
same as before.

### Every created instance is read and closed on every path

Every `finally` in the entry calls `finishSdkEngine` (`:849`, `:977`, `:1540`) or `closeSdkEngine`
(`:1062`, `:1089`, `:1267`, `:1713`, `:1880`, `:2028`). The code between a builder's return and
its body's `try` (`:646-678`, `:864-890`, the track-response headless boot) is covered by the run
backstop (`:2135-2140`). I built a copy of the entry with planted throws and an event log (read,
suspend, close per instance; the log is attached to the run error), and ran it in Chromium. The
entry was restored by SHA-256 after each run:

| plant | run error | read and close | attached | end state |
|---|---|---|---|---|
| connect throws for `spectrum-collection` (live), and its read fails | `planted connect failure` | closed | `renderAllocationReadFailures` | 9 instances, 8 rows, 0 open |
| throw in the hop-1024 body while its context runs | `planted hop body error (state=running)` | suspended, read 0, closed | none | 10 / 10, 0 open |
| throw between the collection's create and its `try` (backstop), and its read fails | `planted pre-try error` | closed by the backstop | `renderAllocationReadFailures` | 9 / 8, 0 open |
| `subscription.close()` throws the string `"planted cleanup string"` in hop-256's `finally`, body OK | `non-object error: planted cleanup string` | read 0, closed | none | 8 / 8, 0 open |
| live-bypass body throws, its read fails and its `browser.close()` fails | `planted body error live-bypass:authored-none` | closed | read failure and `sdkCleanupFailures: close failed` | 11 / 10, 0 open |
| live-bypass body throws the string `"planted body string"`, its read fails | `non-object error: planted body string` | closed | `renderAllocationReadFailures` | 11 / 10, 0 open |

In every case, the first error is the one that propagates, every instance is closed, and no
instance is left open. The one path the entry cannot cover is a `createEngine` that rejects after
an internal boot: the entry has no handle to it. This is in the SDK, and the Non-goals exclude it.

### The backstop cannot hide a missing row on success

`renderAllocations: sdkRenderAllocations.rows.slice()` and `renderAllocationInstances` are
evaluated in the return expression, after every body and before the `finally`. A row that the
backstop pushes later is not in the returned rows. Plant: skip `closeSdkEngine` for
`live-bypass:authored-desk-hi` (a forgotten close, success path). The backstop reads and closes the
instance, and the gate is red: `chromium: sdk-render-allocations: an SDK instance closed without
its render allocation read: 16 rows for 17 instances` (`plant/p4-skipclose.log`). On the success
path, a failed read or close in the backstop is thrown after all instances are closed, so the run
is red there too.

### No ack precedes a drop

A row is pushed only after a `result === 0` reply (`:137-141`). Each instance leaves `open` before
its read, so it gets at most one row. A refused or failed read (a rejected `suspend()` included)
is thrown, or it is attached to the first error. A row that the backstop pushes after the slice is
left out of the returned rows, but the length check then fails. No path records a row and then
loses a read failure, and no path loses a read while its row is still counted.

### #1480's read hold and `renderLoss` are intact

The continuous region (`:663-737`, `:813-816`) is unchanged. Only its `finally` changed, to
`finishSdkEngine(browser, () => shared?.close(), () => subscription?.close())`, with the same
cleanup order as before. Both of #1480's mutations are still red (below).

### m1: the suspend, and whether it can hang a run

- The suspend is reached. The event log shows `running -> suspended` before the read for
  `spectrum-collection` and `spectrum-continuous-hop-1024` in Chromium, Firefox and WebKit. Offline
  contexts show `closed` (finished) or `suspended` (never started), so no suspend is needed for
  them.
- No render follows the read. I built a planted artifact with one allocation in every
  `miso_engine_web_v1_render` call, so each instance's count is its count of rendered quanta. I
  read the count after the suspend, waited 250 ms and read it again. With the suspend, the count
  did not change in any browser (Chromium 132 -> 132 and 28 -> 28, Firefox 226 -> 226 and 30 -> 30,
  WebKit 172 -> 172 and 12 -> 12). Without the suspend, 180 to 200 quanta rendered after the read
  (Chromium 132 -> 314, Firefox 30 -> 226, WebKit 14 -> 208) (`plant/p9-cnt-*.log`). So D2's "after
  the instance's last render" is now true.
- Hang: `suspend()` has no timeout (NIT 1). In the 45 runs that suspended (18 clean, 15
  instrumented, 12 for #1480's mutations) it always resolved at once. A `suspend()` that never settles fails the run red at the CI job's
  10-minute timeout. It cannot make the run pass.

## Mutations redone

- **#1476** (instrumented `mutationProofs` that prints each result; `run.mjs` restored by SHA-256).
  With the gate: in all three browsers, `sdk-render-allocations` is red with `<b>:
  sdk-render-allocations: an SDK worklet instance allocated on its render thread:
  resident-observation=1`, and `sdk-render-allocations-missing` is red with `<b>:
  sdk-render-allocations: an SDK instance closed without its render allocation read: 16 rows for
  17 instances`. With `validateSdkRenderAllocations` disabled, both mutations escape in all three
  browsers (`plant/m1476-{on,off}-*.log`). So no other gate catches them.
- **#1480, mutation 1** (`nativeMissedWindows: 0n`, `skippedPublications: 0n` in `#notifySpectrum`,
  `sdk/src/core/observation-subscriptions.ts`, restored by SHA-256): red on 2 of 2 runs in each of
  Chromium, Firefox and WebKit, each with `false: gap` alone (`plant/m1480a-*.log`).
- **#1480, mutation 2** (`await readsHeld` and `await Promise.allSettled([...inFlightReads])`
  removed): Firefox red on 6 of 6 runs, each with `false: renderLoss` (status 3 with 4 to 6 drops,
  or status 2 with 0 drops) (`plant/m1480b-*.log`).

## Findings

**NIT 1. `await context.suspend()` (`sdk-response-entry.ts:134`) has no timeout.** The same file
bounds `resume()` with a 10 s timer and a diagnostic (`:905-919`, `:1185-1199`). A `suspend()`
that never settles would hold `page.evaluate` until the job's 10-minute timeout, and the
diagnostic would be lost. The read and the close after it have no timeout either
(`#request` in the host has no timeout), so this adds an unbounded await of the same class. It
was never slow in any run. A bound like resume's would keep the diagnostic.

**NIT 2. `finishSdkEngine` can throw a cleanup error over a body error when the instance has
already left `open`** (`:170-187`). `bodyFailed` is then `false`, because `sdkEngineFailed` could
not record the body's error. No current caller reaches this: the three bodies that use
`finishSdkEngine` never close their instance in the middle of the body. A future caller that
closes mid-body would lose the body's error.

## Test value

No new test. The gate and its two mutations are unchanged and still red, as the spec says. The
redone mutations show that no other gate catches them.

## Gates run (export of `45b87c94a`, CI invocations)

- Artifacts: `bash scripts/build-web-audioworklet.sh --named-twin
  target/ci/qualification-named-twin target/ci/qualification-artifacts`, which gives module
  `7c6ee7357eaa...`, the same as the implementer's. `npm ci --no-audit --no-fund --prefer-offline`
  in `sdk/`, `npm ci` in `qualification/`. A private PulseAudio null sink per run, as
  `qualification.yml` sets it up.
- `npm run qualify -- --artifacts <a> --sdk-root <tree>/sdk --browser <b> --check-matrix
  --self-test-mutations`. Every log says "sdk bundle: the source at .../sdk/src (CI's mode)", and
  `sdk/dist` did not exist before or after any run.
  - Chromium 151.0.7922.34: pass (1 run, plus 2 runs with the instrumented `run.mjs`).
  - Firefox 153.0: pass on 10 of 10 runs (15-18 s each), plus 2 runs with the instrumented
    `run.mjs`. That is 12 of 12.
  - WebKit 26.5: pass (1 run, plus 2 runs with the instrumented `run.mjs`).
  - Each run prints all 17 SDK rows `=0` and the ten raw rows `=0`.
- `bash scripts/check-workspace-policy.sh`: ok.
- An ad-hoc `tsc --noEmit` of the entry (CI does not typecheck it), base `ea2def177` against head:
  31 against 26 errors. The changes are as the record states (6 TS2322 at `createContext` take the
  place of 6 of the 7 TS2769, and the `destination` TS2339 errors go from 10 to 5).
- SDK package checks: not rerun, because no SDK, engine or worklet source changed.
