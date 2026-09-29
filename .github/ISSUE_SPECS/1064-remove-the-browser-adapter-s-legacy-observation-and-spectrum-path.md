# Remove the browser adapter's unadopted protected observation path

**Owner ruling (2026-09-29, decision 9 in `docs/rulings/engine-footprint-2026-09-29.md`):** keep the ordinary ("legacy") observation path, which production uses, and remove the protected path, which only tests use. This supersedes the original direction below, whose premise step 1 disproved. #824 closes as not planned.


Owner ruling (2026-09-28, `docs/rulings/engine-footprint-2026-09-28.md`, decision 8): remove it, provided nothing still uses it. `hosts/host-web` keeps a "legacy" and a "protected" observation and spectrum path (`hosts/host-web/src/ffi.rs:646-690`, `protected_observation_prepared` in `lib.rs`), which doubles the refusal tests. Source: the test-value audit, ruling R8, and its verification. This is browser-adapter code, not core.

## Smallest closable slice

1. **Prove non-use first.** Show that the SDK (`sdk/src/core/boundary.ts` and every caller) and the shipped web entry points (`hosts/host-web/web/`) reach only the protected path: trace every `miso_engine_web_v1_*observ*` / spectrum export the SDK calls, and the mode each boot path prepares. **If anything reaches the legacy path, stop and report instead of deleting.**
2. Remove the legacy path, its dispatch and alias handling, and the tests that exist only for it; keep the protected path's refusal tests.

## Objective gates

- The SDK tests, browser qualification (all three browsers) and the host-web tests pass.
- The shipped module shrinks or is explained; the AudioWorklet callgraph and realtime gates pass.
- Test lists before and after show only legacy-only tests removed.

## Attempt 1 evidence

Terra, attempt 1, 2026-09-28, branch `codex/1064-remove-legacy-observation-path` from `92ef396f`.

**Verdict of step 1: STOP. The legacy path is the only observation and spectrum path production
reaches, so nothing was deleted.** The brief's premise is inverted: the SDK, the shipped
AudioWorklet and the app all boot a `Legacy` host, and the `Protected` path has no production
caller.

Trace (line numbers at `92ef396f`):

- **Boot mode.** `miso_engine_web_v1_boot` and `miso_engine_web_v1_boot_with_spectrum_hop`
  select `StagedBootMode::Legacy` (`hosts/host-web/src/ffi.rs:5314-5323`), which boots through
  `boot_with_spectrum_config` and stores `PreparedObservationStorage::Legacy(spectrum_capture)`
  (`hosts/host-web/src/lib.rs:8191`), with `Legacy(None)` for a boot without a spectrum request.
  Only `miso_engine_web_v1_boot_with_observation_demand{,_and_spectrum_hop}`
  (`ffi.rs:5298-5311`) create a `Protected` host.
- **Headless SDK.** `stage()` calls `miso_engine_web_v1_boot` or, with `spectrumHopFrames`,
  `miso_engine_web_v1_boot_with_spectrum_hop` (`sdk/src/core/boundary.ts:1401-1407`, `:90`), after
  staging the ordinary `spectrum`/`spectrumCollection` request (`:1386-1390`). The protected boot
  name appears only in the optional-export set (`:91-97`) and is never called.
- **Shipped AudioWorklet.** `initialize` calls the same two legacy boots
  (`hosts/host-web/web/miso-engine-v1-audio-worklet.js:284-324`) after staging the ordinary
  spectrum request or collection (`:306-315`). The browser SDK reaches it through the shipped host,
  which posts `miso.spectrum.v1` arm/read/select/streamSelect/cancel and stream messages
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-host.js:1424-1517`); the worklet forwards
  them to `spectrum_arm/read/cancel/select/stream_select/stream_start/stream_stop/stream_read`
  (`worklet.js:1194-1327`), and also calls `meter_lease` (`:1591`) and `observation_read`
  (`:1024`).
- **Alias dispatch.** On those legacy hosts `refuse_protected_observation_alias` returns
  `Ok(None)` and `stream_host_dispatch` returns `Ok(false)` (`ffi.rs:652-690`), so every one of
  the aliases above (`ffi.rs:3157, 3299, 3319, 3363, 3924, 4137, 4252, 5553, 5746`) runs its
  legacy body. The ordinary meter lease and resident observation read are live product features
  on this path; the protected profile refuses console meters and resident taps outright
  (`validate_protected_options`, `lib.rs:1950`; the unsupported guards).
- **App.** `misofm/app` at `0757a849` (`src/lib/mixer/engine/open-session.ts:292-309`) opens every
  session with `spectrumCollection`, `policy.spectrumHopFrames` and console `observationTaps`:
  the ordinary collection boot through `boot_with_spectrum_hop`, plus resident taps. This matches
  #835's frozen source facts.
- **Protected callers.** Only tests: `sdk/test/boot-evals.mjs:304,329` and the host-web Rust unit
  tests (at least 69 named `protected_*`). #828 published the protected ABI and left browser/SDK/app
  adoption to a later issue; #835 set a conditional coexistence policy; #824 (protected one-shot)
  is still open.

**What the owner needs to rule on.** R8 ("remove the legacy path if the SDK and app call only the
protected path") cannot apply, because they call only the legacy path. The live question is the
reverse: remove the unadopted protected path (the #825/#828 exports, `PreparedObservationStorage::Protected`,
`observation_ingress.rs`, the protected dispatch and its tests), or keep it for the adoption that
#835 and #824 describe. That is a product decision outside this brief. Under the brief's step 1 it
is recorded here, not acted on.

Gates: no code, test, SDK or artifact changed, so the implementation gates were not run and the
AudioWorklet module is unchanged (its size and hash are those pinned at `570a79f0`). This
checkpoint changes only this spec.
