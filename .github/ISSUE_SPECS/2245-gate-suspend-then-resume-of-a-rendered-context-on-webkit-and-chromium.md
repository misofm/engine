# Gate suspend-then-resume of a rendered context on WebKit and Chromium

Filed by root on 2026-11-04 from the verify of #1531's closure. #1531 closes with this issue as its successor.

## Problem
#1531's diagnosis found that a rendered AudioContext, once suspended and resumed, can stall on WebKit: the clock does not advance and nothing renders. It stalled in 19 of 60 runs with the engine and in 0 of 510 trivial runs, so the cause may be in the engine package. The studio console boots on exactly this path: it suspends a context that has rendered (`studio-v2 src/studio/engine-client.ts:382`) and resumes it on a click (`:654`). No product gate covers it; #1535 covers observation unsubscribe with one resume per context.

## Scope
- A browser qualification case on Chromium and WebKit, both instance modes: boot, render a short first segment (6 quanta or fewer, as the console does), suspend, resume, and require that `currentTime` and `renderedQuanta` advance and that audio renders, repeated enough times to catch a 1-in-3 stall.
- If the case shows the stall, find the cause (engine package or browser) and fix it in the engine if it is ours; if it is the browser, record the evidence and a work-around the SDK applies.

## Gate
The case passes 20 consecutive runs per leg with no rerun, and a mutation that skips the resume path turns it red.

## Attempt 1 record (2026-11-04)

Tree: `codex/issue-2245-resume-gate` from `d4956341c`. Stack: Linux (Ubuntu 24.04, 32 CPUs), Playwright 1.62.1, WebKit 26.5 (`webkit-2336`, WPE MiniBrowser), Chromium 151.0.7922.34, a private PulseAudio null sink at 48 kHz as in CI. Artifacts built from this tree with `scripts/build-web-audioworklet.sh`; no Rust changed.

### The case
- `suspend-resume` row in `hosts/host-web/qualification/qualification.js` (`runSuspendResumeQualification`), gated in `run.mjs` (gate `suspend-resume`). It runs after the observation-unsubscribe run, so after the SDK phase when `--sdk-root` is given (as in CI).
- 10 cycles per leg run. Each cycle: a fresh 48 kHz `AudioContext`, suspended before boot; a raw host on `unsubscribe-session.json` with a meter lease (the console holds one) and 40 prefilled blocks of the +-0.5 square wave; the output goes through an `AnalyserNode` (the capture, 2048 frames) to the destination. A trusted click resumes it; the page suspends at the first status read that reports a rendered quantum; a second trusted click resumes it. Under the 10 s stall bound, the page then needs `renderedQuanta` to move 24 quanta past the suspended value and `currentTime` to move. The capture is read after that, so all 2048 samples come after the resume.
- Gate checks per cycle: `context`, `first-segment` (suspend after a rendered quantum), `suspended`, `resume-state`, `resume-clock`, `resume-render` (24 quanta, `lastResult` 0), `capture` (peak above 0.01; measured about 0.07, silence is 0); plus `count` (10 cycles). A stall throws as the named `stall timeout at suspend-resume cycle N again` with its stall signature.
- The first-segment length is reported, not gated: Chromium runs its first callbacks in a burst of 5 (sometimes 9 or 14) quanta before the page sees one, so the page cannot set the length. The report line gives the count of cycles at 6 quanta or fewer (7 of 10 on Chromium; 4 to 11 on WebKit).
- Self-test: the record mutation `suspend-resume-skipped` (the last cycle's record after the resume equals the suspended record, capture 0) is caught by the `suspend-resume` gate. `suspendResumeFailureProofs` turns each named check red alone. Both run under `--self-test-mutations`.
- One render-allocation row per cycle (`suspend-resume-N`, all 0); `RAW_INSTANCES` is 29; the click plan declares 20 more clicks.

### Result: the stall reproduces on WebKit, both legs
- 20 consecutive runs per leg, no rerun (the CI command plus `--check-matrix`):
  - Chromium single: 20 of 20 pass. Chromium worker: 20 of 20 pass.
  - WebKit single: 0 of 20 pass. WebKit worker: 0 of 20 pass.
- All 40 WebKit failures are a full-signature stall (root's signature (i) to (iv)) at the second resume: at cycle 0 in 30 runs, cycle 1 in 5, cycle 2 in 4, cycle 4 in 1. No other failure.

### Diagnosis: the cause is WebKit, with a no-engine repro
A diagnostic page (scratch, not committed; 10 cycles per run, fresh browser per run, the same click plan) compared the engine package with a trivial processor. The trivial processor writes the square wave, posts one message per quantum and answers pings; it can busy-wait a set time inside `process()`.

| Cell (WebKit single unless named) | stalls / cycles |
|---|---|
| engine, meters on | 7/10 |
| engine, meters off | 2/10 |
| engine, meters on, first segment at least 9 quanta (suspend at 10 to 11) | 1/10 |
| engine, meters on, 300 ms gap between suspend and resume | 8/10 |
| trivial, ping only (first segment 7 to 8) | 0/10 |
| trivial, one post per quantum (first segment 5 to 6) | 0/20 |
| trivial, posts, `process()` busy 1 ms per call | 8/10 |
| trivial, posts, `process()` busy 2 ms per call | 10/10 |
| trivial, posts, `process()` busy 10 ms for the first 3 calls only | 10/10 |
| trivial, posts, busy 2 ms, first segment 8 to 11 quanta | 8/10 |
| Chromium: trivial busy 2 ms / engine meters on | 0/10 / 0/10 |

- A processor with no engine code stalls the same way as soon as `process()` takes real time (1 ms per call, under the 2.67 ms quantum budget, is enough). The engine's `process()` does real DSP work, so it takes that time. This explains #1531's 0 of 510 trivial runs: that processor did no work.
- Neither the first-segment length nor the gap before the resume removes the stall. Chromium does not stall.
- The fault is WebKit's (GStreamer audio destination on Linux): after the second `resume()` the context reports `running`, the worklet answers messages, and the render stops (0 to 2 quanta after the resume).
- No engine, SDK or host fault was found, so no engine fix is made.

### Work-around: none small and safe
- Reconnect the node to the destination after a stalled resume: 9/10 still stalled.
- A second `resume()` outside a click on the stalled, "running" context: it stays pending (5/5 timed out).
- `suspend()` then `resume()` on the stalled context, with or without a new click: the WebKit page hangs (the whole run was killed by its timeout), as #1531's runs 494, 042 and 079 did.
- Re-creating the `AudioContext` and the engine after a stall is not small: it needs a new boot, the feed and the session state.
- So, per the brief, no SDK work-around is added, and this attempt stops here.

### Checks
- `--check-matrix --self-test-mutations`: Chromium single and worker pass (22 s per leg); WebKit single and worker fail at `browser-execution` with the stall above. The mutation proofs pass on Chromium; on WebKit they are not reached.
- SDK: `npm ci`, `check-sdk-generated.sh`, `check-sdk-deletions.py`, `check-sdk-types.sh`, `check-sdk-headless.sh`, `sdk-package.sh check`: pass (`sdk/dist` deleted after).
- Hermetic browser host and worklet tests (`scripts/test-web-audioworklet.sh`): pass.
- Web build (`build-web-audioworklet.sh --named-twin`) and `check-web-audioworklet.sh --without-metadata-regeneration`: pass.
- `bash scripts/check-workspace-policy.sh`: pass.

### Open, for root
- The gate cannot pass on WebKit. Merging the row as is turns both WebKit legs of `qualification.yml` red.
- Root decides: an owner ruling on a Linux WebKit exemption (the full-signature rule, now with a no-engine repro, which opens #1531's condition (2)), a CI audio change, an upstream report (needs owner consent), or an app-level recovery (re-create the context) in a new issue.
- The studio console's boot path (suspend after the feed attaches, resume on a click) is exposed to this stall on Linux WebKit. Real Safari on macOS uses a different audio destination and was not measured.

## Owner ruling (2026-11-05) and its application

- **Cause (recorded):** Linux WebKit's audio output (WebKit 26.5, GStreamer, the PulseAudio null sink) stalls after the second resume under any processor that does about 1 ms of work per `process()` call. The engine package is not the cause. The no-engine repro below is the evidence.
- **Ruling:** Chromium gates the `suspend-resume` row. WebKit reports it, "reported, not gated", like the other WebKit rows (`pcm-feed-suspended.mjs`'s live-seek rows). The owner checks real Safari on a Mac separately.
- **Applied (`1e9818360`):**
  - A stall after the second resume is now recorded in the cycle (`stall`, with its stall signature) instead of thrown, and the cycle still disposes its host and closes its context. A new `stall` check joins `suspendResumeFailures`.
  - `run.mjs`: `SUSPEND_RESUME_GATED` is `chromium`. On WebKit, only the setup checks (`SUSPEND_RESUME_SETUP_CHECKS`: `result`, `count`, `context`, `first-segment`, `suspended`) are gated. The report line names the stalled cycles and the full-signature count, and prints each stalled cycle's signature.
  - The mutation `suspend-resume-skipped` is proved on Chromium only (`onBrowser("chromium", ...)`). The proof output states that the WebKit row is not gated.
- **Counts:** Chromium single 20/20 and Chromium worker 20/20 pass, with no rerun (attempt 1 record). With the ruling applied, one `--check-matrix --self-test-mutations` run per leg passes on all four legs:
  - Chromium single and worker: 0 of 10 cycles stalled.
  - WebKit single: 7 of 10 cycles stalled, all with the full signature (leg 124 s).
  - WebKit worker: 4 of 10 cycles stalled, all with the full signature (leg 81 s).
- **Checks after the ruling:** SDK checks, `check-web-audioworklet.sh` with the named twin, the hermetic web tests and `check-workspace-policy.sh` all pass.

### The no-engine repro (recorded as text; the repository has no committed qualification diagnostics directory)
The page: fresh browser per run, 10 cycles per run, under the private PulseAudio null sink. Each cycle does the following:
1. Create a fresh 48 kHz `AudioContext`, call `suspend()`, wait for "suspended", `addModule` the processor below, then `new AudioWorkletNode(context, "diag-trivial", { numberOfInputs: 0, numberOfOutputs: 1, outputChannelCount: [2], processorOptions: { busyMs } })` connected to the destination.
2. A trusted click calls `resume()`. Suspend at the first port message (the first rendered quantum), and wait for "suspended".
3. A second trusted click calls `resume()`. The cycle passes when the processor's call count moves 24 past its suspended value and `currentTime` moves, within 5 s.

```js
class Trivial extends AudioWorkletProcessor {
  constructor(options) {
    super();
    this.calls = 0;
    this.busyMs = options.processorOptions.busyMs;
    this.port.onmessage = (e) => this.port.postMessage({ ping: e.data.ping, currentFrame, calls: this.calls });
  }
  process(_inputs, outputs) {
    this.calls += 1;
    const t = Date.now();
    while (Date.now() - t < this.busyMs) {} // the work
    for (const channel of outputs[0]) for (let i = 0; i < channel.length; i += 1) channel[i] = ((i >> 4) % 2 === 0) ? 0.5 : -0.5;
    this.port.postMessage({ meter: this.calls });
    return true;
  }
}
registerProcessor("diag-trivial", Trivial);
```

- `busyMs: 0` (the control): WebKit 0 of 30 cycles stalled.
- `busyMs: 1`: WebKit 8 of 10. `busyMs: 2`: WebKit 10 of 10. Chromium with `busyMs: 2`: 0 of 10.
- A later run re-checks whether WebKit still stalls by running this page with `busyMs: 1` and `busyMs: 0`.

### Open
- The owner checks real Safari on a Mac. macOS uses a different audio output, and this tree did not measure it.
- If Safari also stalls, the studio console's boot path (`studio-v2 src/studio/engine-client.ts:382` and `:654`) needs an app-level recovery in a new issue.
