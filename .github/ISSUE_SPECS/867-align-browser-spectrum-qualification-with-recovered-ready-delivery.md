# Align browser spectrum qualification with recovered ready delivery

## Product outcome

Make the real-browser continuous-spectrum qualification assert the post-#863 contract: a synchronously recovered native gap publishes one ready/available callback that preserves the loss counters and matches the owned result identity. Keep every existing DSP, frequency, magnitude, meter, PCM, underrun, hop, smoothing, lifecycle, and browser requirement unchanged.

## Trigger and root evidence

Engine release PR #866 runs the full browser matrix and fails Chromium, Firefox, and WebKit at `sdk-spectrum-continuous`. Fresh Astra xhigh reproduction against the accepted seven-file artifact proves the only failing conjunct is `continuous.statuses.includes("gap")`. Issue #863 intentionally stopped publishing the intermediate gap before its bounded recovery read, because that callback consumed cadence and suppressed the recovered ready publication in the app.

All three browsers now produce one ready/available callback with truthful loss (`nativeMissedWindows > 0`, `skippedPublications > 0`), result identity matching the notification, H256, window `[0, 2048)`, bin 32 at 750 Hz, correct FFT magnitude/response/meter/PCM, and no underrun. Substituting only the pre-#863 SDK source restores `[gap, ready]` while moving the loss off the ready callback. The #863 SDK-route CI skipped browsers; #866 release routing exposed this stale assertion.

## Smallest closable slice

Authorized paths are only:

- this issue spec;
- `hosts/host-web/qualification/sdk-response-entry.ts`;
- `hosts/host-web/qualification/run.mjs`.

In the fixture result, expose the recovered callback's status, availability, native-loss count, skipped/coalesced-publication count, notification identity, and owned-result identity. Update the obsolete pump comment/break so it waits for the recovered available publication rather than a visible gap. In `run.mjs`, replace the mandatory observed-gap callback with a dedicated recovery-delivery gate requiring ready/available status, positive truthful loss evidence, and exact notification/result identity. Add narrow mutation checks that prove the gate rejects erased loss, unavailable/non-ready recovery, and mismatched identity.

Do not change Engine/SDK runtime, native host, Wasm/artifacts, DSP, spectrum window/hop/smoothing, package/release identities, CI routing, timings, browser skips, numerical tolerances, or unrelated qualification behavior. Do not assert the scheduling-specific exact loss value 16; require positive truthful loss and identity preservation.

## Objective gates

1. Chromium, Firefox, and WebKit pass the existing browser qualification with matrix and self-test mutations against the accepted artifact.
2. Red mutations independently fail for erased loss, unavailable/non-ready recovery, and mismatched publication identity.
3. Every existing 750 Hz, FFT magnitude, response, meter, PCM, span, source-underrun, H256, and lifecycle assertion remains byte-for-byte unchanged.
4. Existing SDK type, static Worklet, real-Wasm receiver, and hermetic gates pass proportionally.
5. Exact three-path audit and `git diff --check` pass.
6. Fresh adversarial review verifies the change tests #863's intended public contract rather than weakening qualification.

## Delivery

This issue unblocks #865/#866. Merge this bounded qualification correction to `main`, require exact-main qualification green, then rebase or refresh the release branch from that synchronized main without altering the reviewed 0.4.3 candidate payload. Record PR/run/merge evidence here and close only after GitHub synchronization.

## Starting evidence

- Accepted runtime source: `c78e1fbed267b8e1382a59beaf266eac015335bc`.
- Failed full-route run: 35180028907, same obsolete predicate in Chromium, Firefox, and WebKit.
- Current three-browser raw results: `/tmp/issue866-browser-current-Yklt0Q/`.
- Pre-#863 Chromium control: `/tmp/issue866-browser-pre863-qcTQZY/chromium-continuous-result.json`.
- No runtime/artifact correction is authorized or required.
