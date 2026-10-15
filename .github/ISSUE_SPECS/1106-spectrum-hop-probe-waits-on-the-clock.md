# Bound the browser spectrum hop probe by its deadline, not a poll count

## Problem

The SDK-driven browser qualification's configured spectrum hop probe
(`hosts/host-web/qualification/sdk-response-entry.ts`, `runConfiguredSpectrumHopQualification`)
waits for two published windows after it resumes the `AudioContext`:

```ts
const deadline = performance.now() + 10_000;
for (let attempt = 0; attempt < 512 && publications.length < 2; attempt += 1) {
  await subscription.pump();
  if (performance.now() >= deadline) break;
}
```

`pump()` can return almost at once, so the loop can use up its 512 attempts long before the
10-second deadline. If the audio device starts slowly, the second 1,024-sample window (about
21 ms at 48 kHz) has not been published yet. The probe then fails with
`H1024 browser probe published 1 windows`.

Evidence:
- It failed once in Chromium on PR #1105 (run 36719596271, job 109902172521).
- It failed again in Chromium on PR #1238 (run 37115329466, attempt 1, job 111181516316, head
  `8c6268967`), and passed when the failed jobs were re-run. The same attempt failed Firefox's
  `sdk-spectrum-continuous` gate, which is *Name the failed predicate and bound the waits of the
  browser continuous-spectrum gate by a deadline* (#1248).
- It passed when the failed job was re-run, and in every local three-browser run.
- It failed again in Chromium on PR #1625 (controls batch 9, head `097b9151e`) after passing on that PR's first run and in two local three-browser runs on the same code; batch 9 changed nothing the probe reaches (the qualification page's spectrum code and the spectrum service are unchanged by it).
- The probe's code has been unchanged since `756ed813`.

## Smallest closable slice

Bound the wait by the deadline only, and yield to the event loop between pumps (for example
`await new Promise((r) => setTimeout(r, 5))`), so the probe cannot finish before the audio clock
has advanced. Keep the 10-second deadline, the two-window requirement and the typed failure.
Apply the same fix to any sibling probe in the qualification entry points that bounds a
wait by an attempt count.

Authorized paths: `hosts/host-web/qualification/sdk-response-entry.ts`, any sibling entry in
`hosts/host-web/qualification/`, and this spec.

## Objective gates

1. The browser qualification with `--check-matrix --self-test-mutations` passes in Chromium,
   Firefox and WebKit.
2. A planted slow start (for example a 500 ms delay before the first pump) still passes, while the
   same delay with the old attempt bound fails. Record both as PR evidence.
3. A probe that never publishes still fails with its typed message within the deadline.

## Decision record: hop probe wait (batch 8)

Shipped. `runConfiguredSpectrumHopQualification` waits through `waitForCount`: it polls `pump()`
and then sleeps 5 ms, and stops at a 10 s `performance.now()` deadline. There is no poll count.
The failure still names what it saw: the windows seen with their captured samples, the elapsed
time, the poll count, the notification count and the context state.
The only sibling bounded by an attempt count was `submitBlocks` in the collection probe (200
retries of 5 ms on source backpressure). It now stops at the probe's 10 s readiness budget.

Departures from the text, and why:
- The wait starts at the click, not when `resume()` resolves. Firefox resolves `resume()` only
  when its audio has started, so a wait that began after the resolve could not observe a slow start.
- The wait ends on two windows one hop (1,024 samples) apart, not on the first two windows. A
  late start leaves the first reads behind the audio and the capture ring drops windows (WebKit
  published windows 7,168 samples apart under the planted delay). The hop is the distance between
  two consecutive publications, so a hop bug still times out and fails; the row reports that pair.
- The slow start is planted by holding the native `startSpectrumStream` for 750 ms after the
  click. The context is running and every `pump()` returns at once with nothing published. A
  `resume()` outside the click handler is not possible (the `resume-clicks` gate), and a worklet
  that stalls the audio thread did not work: it blocks `pump()` too, so the old loop survived it.

Gates (`run.mjs`, row `sdk-spectrum-hop-wait`): the first publication is at least 650 ms after the
click; the wait ended before its deadline with a bounded poll count; a wait that can never be
satisfied (250 ms deadline, nothing published) ends at its own deadline and polls no faster than
its timer. Self-test mutations: `sdk-spectrum-hop-wait-fast-start`, `-early`, `-spin`,
`-never-unbounded`.

Measured (first publication / second publication after the click, polls): Chromium 813 / 836 ms,
162 polls; Firefox 836 / 889 ms, 150 polls; WebKit 1289 / 1344 ms, 165 polls. The never-publishing
wait ended at 254 ms (deadline 250 ms) with 35 to 50 polls.

Red evidence: under the same 750 ms plant, the old 512-poll loop (restored temporarily after the
resume, then removed) fails in Chromium (`published 0 windows` after 512 polls in 13 ms) and WebKit
(512 polls in 10 ms). It passes in Firefox, whose `resume()` resolves only after the delay. With a
3 s plant and a 1 s deadline the new probe fails in bounded time with its typed message (`published
0 windows in 1001 ms (deadline 1000 ms, 197 polls ...)`).
