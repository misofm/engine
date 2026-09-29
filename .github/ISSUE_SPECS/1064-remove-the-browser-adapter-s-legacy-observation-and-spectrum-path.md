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

## Attempt 2 evidence

Terra, attempt 2, 2026-09-29, branch `codex/1064-remove-protected-observation` from `b695758b`,
under owner decision 9. Commits `fdd4fec1` (the removal) and `760a577e` (the fixture's
linear-memory pins).

**Non-use, re-proved on this base.** No shipped caller reaches the protected surface:
`hosts/host-web/web/` names none of the 18 protected exports; the SDK names only
`..._boot_with_observation_demand_and_spectrum_hop`, in its optional-export set, and never calls
it; `misofm/app` at `7effbd6` names none of them (it boots through `boot`/`boot_with_spectrum_hop`
with `spectrumCollection`). The only callers were `sdk/test/boot-evals.mjs`,
`sdk/test/browser-evals.mjs`, the host-web unit tests and the layout gates.

**Removed.** The 18 protected exports: `boot_with_observation_demand{,_and_spectrum_hop}` and
`observation_{preparation,demand,admission,status,capture_identity}_*` plus
`observation_application_{take,ptr,bytes,capacity}` (each of the latter works only on a protected
host, so keeping it would have left a vestigial refusal). Also removed: their eight wire records and
nine vocabularies (including `observationProfiles`, so `legacyUnprotected` goes), `ObservationSideRecords`,
`PreparedObservationStorage` (the ready host is back to the pre-#825
`spectrum_capture: Option<PreparedSpectrumCapture>`), `observation_ingress.rs`,
`validate_protected_options`, `protected_preparation_facts`, the endpoint staging, the protected
arm of every alias and host method, the render-boundary ingress hook, the raw-observation batch
gate in `submit_commands`, `SpectrumStaging::stream_history_exhausted` (the compiler proved it
unused), and the host-web `graph` dev-dependency. `ff5956c2` added that dependency for the
protected fixtures, and nothing else uses it. Net **−15,593 lines** in 18 files (histogram diff
+276/−15,869). host-web alone is −11,677: `ffi.rs` −6,629, `lib.rs` −2,163, `tests.rs` −2,068, and
`observation_ingress.rs` −817.

**Ordinary path unchanged.** Every alias (`spectrum_{read,select,stream_select,stream_start,
stream_read,stream_stop,cancel}`, `meter_lease`, `observation_read`) now has one body, the
legacy one. The handle half of the old dispatch stays as `verify_live_host`: handle 0 or a
mismatched handle returns `INVALID_ARGUMENT`, and a failed exclusive borrow returns `INTERNAL`,
before any staging is touched. That is the code and ordering the ordinary path returned before, so
no result changes. It is a handle check, not a protected refusal. Two changes are unobservable
from single-threaded Wasm: boot and dispose no longer borrow the observation staging, which is the
pre-#825 behaviour (the worklet still forces that staging at init), and the unreachable
publication-failure diagnostic is renamed `web.observation.staging` → `web.host.publication`.

**Generated surfaces.** Regenerated with `parameter-metadata --print-abi-layout`, `npm run assets`
and `npm run codegen`: `sdk/assets/miso-engine-v1-abi-layout.json` and
`scripts/fixtures/abi-layout-v1-self-test.json` lose 196 lines each, and `sdk/src/generated/abi.ts`
loses 897. The **SDK surface** change: `ABI_LAYOUT.exports` goes from 134 to 116 names; 8 structures
and 9 constant groups leave `ABI_LAYOUT`, which narrows `AbiStructureName` and
`AbiConstantName`; no named export is removed; the optional-export set goes from 3 to 2.
`check-web-audioworklet.sh`'s frozen export list drops the same 18 names.
`check-abi-layout-v1.py` drops the protected rules. Its mutations go from 27 to 19, and the
boolean-value mutation is retargeted to `observationChannels`. `sdk/README.md` drops its
protected-ABI paragraph.

**Module and resources** (`--module-only`, both built by `build-web-audioworklet.sh`):

| | base `b695758b` | this branch |
|---|---|---|
| simd128 module bytes | 3,415,176 | 3,322,821 (−92,355, −2.70 %) |
| functions | 8,081 | 7,787 |
| sha256 | `9aca423b…` | `5afb47e3…` |
| render closure (functions / traps) | 8 / 5 | 8 / 5 |
| `command_submit` closure | 38 | 36 |
| `bridgeMetadataBytes` / `bridgeRetainedBytes` | 1,149,255 / 1,169,764 | 1,146,259 / 1,166,768 (−2,996) |
| fixture linear memory at boot | 1,376,256 | 1,310,720 (−1 page) |

The three `memoryBytes` pins in `hosts/host-web/tests/browser-v1/expected.json` return to the
values `31393181` raised them from. The three PCM digests are unchanged. The release pin and
`qualification/results.json` are not touched (#1061).

**Test lists** (`cargo test --workspace --all-targets --all-features -- --list`): 2,212 → 2,114.
host-web goes from 210 to 123; 69 of its removed tests are named `protected_*`. `tests/abi_layout.rs`
goes from 17 to 6; the 11 removed tests pinned the protected records and vocabularies. The diff
has 102 removals and 4 additions. The additions are kept ordinary-path tests, moved out of the
deleted checkpoint modules into `ffi::observation_alias_tests`:
- `observation_staging_retention_adds_actual_refcell_once` and
  `invalid_alias_handle_is_terminal_before_any_staging_work` moved unchanged;
- `protected_alias_host_borrow_refusal_is_terminal_before_any_staging_work` became
  `alias_host_borrow_refusal_...` and now runs on an ordinary host;
- `legacy_stream_dispatch_refusals_...` became `stream_guard_refusals_...`, without the protected
  capture-identity marker.

So 98 tests were removed; every one existed only for the protected path or its records. The
SDK loses one test, `boot-evals`' protected boot. The #852 hop-boot evals lose their protected
option. `browser-evals`' BigInt offset test is retargeted to kept u64 fields.

**host-core.** Nothing removed. Its observation-demand API is public and exercised by its own
tests, so the compiler proves nothing unused: workspace clippy `-D warnings` is clean.

**Gates, all green:**
- `cargo fmt --check`; `cargo check` and `cargo clippy --workspace --all-targets --all-features -D warnings`; `cargo doc -D warnings` for host-web and parameter-metadata.
- The wasm32 `+simd128` check and clippy of host-web.
- Tests for host-web (121 unit + 2 integration), host-core, parameter-metadata and session-validator.
- `build-web-audioworklet.sh`; `check-web-audioworklet.sh`, including the render, `meter_poll` and `command_submit` callgraphs and kernel shape 15 ≥ 11; `test-web-audioworklet.sh`.
- The V8 spill gate and its self-test.
- `check-browser-expected-resources.py --artifacts` (every row within its #1060 ceiling) and `--self-test`.
- The SDK: `check-sdk-generated.sh`, `check-sdk-deletions.py` and `--self-test`, `check-sdk-types.sh`, `check-sdk-headless.sh` (284/284 evals), and `sdk-package.sh check`.
- Three-browser qualification in CI mode (`--check-matrix --self-test-mutations`, private PulseAudio sink): Chromium 151, Firefox 153 and WebKit 26.5 all pass.
- `check-console-fixtures.sh`.
- The CI routing check and its tests.
- 58 policy and self-test scripts, run with `python3 -B` for Python: the set of the lint, docs-gates and gate-self-tests jobs, plus `check-release-shape.py`, `check-scalar-oracle-absent.py --wasm` and `web-audioworklet-identity.py --self-test`.
