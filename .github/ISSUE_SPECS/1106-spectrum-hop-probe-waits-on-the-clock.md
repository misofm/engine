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
