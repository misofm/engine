# K2 follow-ups verdict (Sol): `4517a4077` + `0006b77f0` (base `89ba1b233`), branch `codex/batch-submix-k2`

**FAIL for the push. The follow-up commits themselves pass review.**

- The two follow-up commits are correct. Every ledger item is applied or justifiably skipped,
  every new or strengthened test bites, and the specs and GitHub match.
- **K2 is not ready to push.** One BLOCKER from #1210 (attempt 1, `66b2c7dd7`) makes CI's
  required `browser` job fail on all three browsers. The fix is seven lines in one test-harness file;
  I proved it below. After the fix lands, K2 is ready to push. No other finding blocks.
- The worktree was left clean at `0006b77f0`. Every mutation was restored, and `git status` is
  empty. Nothing was committed or pushed, and nothing was edited on GitHub.

## BLOCKER-1: the browser qualification legs fail on HEAD (chromium, firefox, webkit)

- **Symptom.** `npm run qualify -- --artifacts <A> --sdk-root <HEAD sdk> --browser <b>
  --check-matrix --self-test-mutations` exits 1 in every browser, in CI's source-bundle mode (a
  HEAD copy without `sdk/dist`):
  - chromium: `TypeError: shape.submixes is not iterable` at `strips` <- `observationMap` <-
    `runResidentObservationQualification` <- `runSdkObservationQualification`;
  - firefox: `shape.submixes is undefined`;
  - webkit: `Spread syntax requires ...iterable not be null or undefined`.
- **Cause.**
  - #1210 D5 made the browser engine's observation path read
    `strips = () => [...shape.tracks, ...shape.submixes]` (`sdk/src/browser/engine.ts:597`).
  - `SessionShape.submixes` is required (`sdk/src/core/boundary.ts:237`). But the seven injected
    `scratchBoot` stubs in `hosts/host-web/qualification/sdk-response-entry.ts` (lines 248, 365,
    420, 1077, 1349, 1521 and 1746 at HEAD) return `tracks` and no `submixes`.
  - esbuild bundles that file without type-checking, and `check-sdk-types.sh` does not cover it,
    so nothing local catches it.
  - The #1210 record accepted the lazy `strips()` precisely to tolerate stub shapes in
    `live-response-evals.mjs`. The browser harness's stubs were missed, and no per-slice verifier
    ran the browser legs.
  - The production path is not affected: `scratchBootWithWorker` passes `WasmBoundary.shape()`
    through by structured clone, and that includes `submixes`.
- **Impact.** The qualification route is `full`, so the verdict expects `browser` = success. The
  K2 PR and the `main` push would both fail `qualification`.
- **Fix (proved).** Add `submixes: [],` after `tracks: [...]` in each of the seven `scratchBoot`
  stubs. With exactly that edit applied to a HEAD copy:
  - chromium 151.0.7922.34: all qualification gates passed;
  - firefox 153.0: all qualification gates passed;
  - webkit 26.5: all qualification gates passed.
- **Record it.** #1210's authorized paths do not include the file. #1209's paths do, but only
  "if a field set they check must follow D4". So the root should record the fix as a #1210
  amendment, or as a K2 follow-up. Test value: the three browser legs themselves, which are red
  on its revert (shown above).
- **Optional hardening** (not required to push): give the harness a type check, or have
  `createEngine` refuse a scratch shape without a `submixes` array with a typed `MisoUsageError`.
  Either would turn a deep `TypeError` into an early, named failure. `scratchBoot` is a public
  `createEngine` option, and the app's test fake (`engineFakeScratchBoot(shape)` in
  `misofm/app`) is typed `SessionShape`, so TypeScript will flag it on the bump. See NIT-4.

## Ledger items

| Item | Status |
|---|---|
| Retire 1199-1205 specs | Done per `.github/ISSUE_SPECS/README.md`: closed by PR #1231, and retired in K2, the batch after. `ISSUE-MAP.md` links each one at `b6b1bdf4` (all seven paths exist there, and `b6b1bdf4` is upstream via `258e1008c`). No live file links a removed path. |
| K1 NITs (schema line; APP-SDK automation gap) | Done. The schema line is now 116 columns (NIT-2). The APP-SDK section is accurate, and #1196:221 records the gap. |
| 1206 MINOR-1, NIT-1/2/3 | Done. The record sentence is added, and the qualification doc carries both the tightening and the probe note. I verified the probe claim: before #1206, `ffi.rs:340` returns `RESULT_INVALID_ARGUMENT` for a nonzero reserved word. The `prepare_caps` doc and the header comment are fixed (comment only; `check-capi-abi.sh` is green). |
| 1206 MINOR-2 -> #1232 | Filed. See the #1232 section. |
| 1207 MINOR-1 | Done (part 1, the buses differ in shape). **M9 reproduced**: `for submix in model.submixes.iter().rev()` in `compile_ready` turns the gate red; the boot refuses at `tests.rs:5517`, which also takes gate 5 red. |
| 1207 MINOR-2 | Folded into `bus_meters_render_and_poll_without_allocating` (#1209 gate 5). I confirmed that its measured closure is `render_next()` + `poll_meters()`, with a tap armed and a window published. |
| 1207 NITs | Done. `prepare.rs:944` now says `strips`; the effect-compiler, `effect_controls` and `ffi.rs` docs say strip. |
| 1208 MINOR 1, NIT 1/3 | Done: the D4 consequence line, and the docs at `:294`/`:990`. NIT 2 (the browser docs) was #1213's job, and one site is left (NIT-1 below). |
| 1209 MINOR-1/2, NIT 2 | Done. **S1, S2 and W0 reproduced red** (below). |
| 1209 NIT-3 skipped | Justified: the verdict called it optional, and it would add a refusal with no present defect. |
| 1210 MINOR-1, NIT-4 | Done. **M5 reproduced red.** |
| 1210 NIT-1..3 skipped | Justified: the verdict offered them as optional refactors, off the render path. |
| 1211 MINOR-1, NIT-1 | Landed in #1213. `solo.rs:164,282` has `strip_count()`/`strip_delta()`. |
| 1211 NIT-2..4 skipped | Justified. NIT-2 asked for "no action unless root wants", NIT-3 was "accept", and NIT-4 is a later candidate. |
| 1212 NIT-1/2/3 | Done. The self-test has 20 mutations, matching the MUTATIONS row, and the constant pin lists reason 12. |
| 1213 attempt 2 MINOR-A, NIT-A/B/C | Done. **MINOR-A reproduced**: bus -6 dB against reference -12 dB goes red ("block 2 sample 1: bus -0.0816 vs track -0.0815"). The module moved to `f08c5433...` by panic-location bytes only (see the identity section below). |
| 1214 MINOR-1 (A1), MINOR-2, NIT-3/4 | Done (see below). **MA, M3 and the type mutation reproduced red.** |
| 1214 NIT-1/2 skipped | Justified: both are type-level or cosmetic, and the D1 runtime separation is intact. |

## `LiveControlEdits.strip(id)` and bus-tap subscriptions

- **Correct.**
  - `strip(id)` resolves a track first, else a submix, and throws `MisoUsageError` naming both
    lists. Its type is `StripEdits`, so it has no `solo`.
  - The resolution is unambiguous. `session/src/validate.rs:76-92` inserts track and submix IDs
    into one map and refuses duplicates, so a strip ID is unique across both kinds.
  - The owner's `#edit` is the only managed caller.
  - The three test stubs were renamed `track` -> `strip` to match.
- **Tested.**
  - The new headless eval subscribes to `bus`/`bus-comp`, reaches `ready`, and returns to
    `unarmed` on close.
  - Mutation MA (restore `edit.track`) fails exactly that eval.
  - Making `strip()` return `TrackEdits` fails `tsc` with an unused `@ts-expect-error` at
    `live-controls-types.ts:151`.
- **Documented.** `sdk/README.md` names `edit.submix`/`edit.strip`. `APP-LIVE.md` says the
  managed owner arms through `edit.strip(id)`. The record is #1214 amendment A1.
- **API surface: acceptable.**
  - It is one additive method on an already public class, and it returns the already exported
    abstract `StripEdits`. Nothing is renamed or removed.
  - `sdk-package.sh check` and the `sdk-deletions` gate pass.
  - It closes the #1214 MINOR-1 hole, where `subscribeObservations` refused every bus tap.

## New and strengthened tests: each bites (mutations applied in a HEAD copy or restored in place)

| Mutation | Result |
|---|---|
| MA: owner `edit.strip` -> `edit.track` | red: "a managed subscription to a bus tap ..." |
| M3: browser map `[...remoteMap.submixes].reverse()` | red: "a browser engine's live controls address a submix by its strip index" |
| S1: SDK master pair from bus-a's slot | red: "a frame with submixes keeps the track shape ..." |
| S2: SDK `trackGrDb` from `2T + 2` | red: the same test |
| M5: worklet `submixIds.unshift` | red: "issue #1210: the enumerated submix order" (`zz-bus` first) |
| W0: worklet refuses `submixCount === 0` | red: the `S = 0` processor block posts `miso.error.v1` |
| M9: host-web submix tables reversed | red: gate 1 and gate 5 (`tests.rs:5517`) |
| MINOR-A: bus EQ -6 dB, reference -12 dB | red at block 2, sample 1 |
| `strip()` typed `TrackEdits` | `tsc` red (TS2578) |

The `id_staging_bytes >=` edit is a deliberate weakening to a ceiling, as AGENTS requires, and it
is not a new test.

## #1232 (new tooling issue)

- The GitHub title equals the H1, and the body is byte-equal to
  `.github/ISSUE_SPECS/1232-make-the-c-abi-checkers-header-mutation-legs-reach-the-compiler.md`.
  It is OPEN, has no labels (like its peers), and is in neither the umbrella nor `ISSUE-MAP.md`.
  It is a standalone issue.
- **Self-contained and accurate against the script:**
  - the leg names and scratch names;
  - `-I"$(dirname "$header")"`;
  - both fixtures `#include "miso_engine_v1.h"`;
  - five non-header legs;
  - the first `reserved[4]` is `engine_config` (`miso_engine_v1.h:104`).
- **Deliverables and gates.** They are testable. The positive staging control, the
  "No such file" stderr refusal and the byte-identical-copy refusal each have a red-on-revert
  gate.
- Missing a Dependencies section is normal for the standalone specs (#948, #1173, #1167).

## Spec and GitHub sync

- The title and body of #1196, #1206-#1214 and #1232 equal the local files exactly.
- #1197-#1205 are CLOSED.
- `.github/ISSUE_SPECS/` equals `gh issue list --state open` (115 issues).
- The 26 verdict and scratch copies in `docs/handoffs/submix-sends-2026-10-02/verdicts/` are
  byte-equal to `/home/bl/misofm/submix-verdicts/`.

## Shipped module and identity

- **Digest.** The module is `f08c5433ca6e31c253ebb121fbab8d36f763ac28bae9f6410258bbc727ba00de`
  (2 695 834 B; named twin `9b75059e...`). This matches the #1213 follow-up record.
- **Reproducible.** I rebuilt it from a `git archive` copy at another path with a fresh
  `CARGO_HOME` and got the same bytes.
- **Report.** `web-audioworklet-identity.py report --event push --before 258e1008c` says
  `ARTIFACT CHANGED f780c82b... -> f08c5433...`, "Reproducible" and "Release fingerprint not
  checked", and exits 0. Its self-test passes.
- **Pins.** No release pin or `npm-publish.yml` identity changed.

## Local gate set on HEAD `0006b77f0` (x86_64 AVX2+FMA, rustc 1.97.1, Node 22.23.2)

The router gives `route=full`, `math_closure=false`, `release_inputs=false` and
`self_tests=["sdk-deletions"]`. Every step was rc 0 except the browser legs (BLOCKER-1).

- **`route`:** `check-ci-path-routing.py` and `test-ci-path-routing.py`.
- **`docs-gates`:** the DSP research and listening gates.
- **`artifact`:** the build with the named twin.
- **`sdk`:**
  - generated, deletions and types;
  - headless: 347/347;
  - package check: 15/15.
- **`artifact-gates`:**
  - strip-names self-test and check;
  - `check-web-audioworklet.sh --without-metadata-regeneration`;
  - `check-browser-expected-resources.py --artifacts` (32 red mutations);
  - wasm scalar-absent;
  - `test-web-audioworklet.sh`;
  - V8 spill self-test and gate.
- **`gate-self-tests`:** `check-sdk-deletions.py --self-test`.
- **`lint`:**
  - fmt;
  - clippy `--all-features -D warnings`;
  - doc `-D warnings`;
  - every policy check/test pair: workspace, session, env vocabulary, bench, test-support CI,
    script reachability, host-core, protocol-control, realtime plus audit-leak plus
    artifact-evidence-leak, trace validator, lane, unfused seal, rack/builtins/graph and
    effect-runtime with its fixtures;
  - scalar-oracle self-test, console benchmark fixture, builtins fixture mutations and bench
    preconditions;
  - conformance boundaries, the PEQ seal and the release-shape self-test;
  - npm publish modes, stem store and stem identity;
  - the three x86 lane-guard probes.
- **`test-debug-a`:** the exact CI command: 103 binaries, 1174 passed, 0 failed.
- **`test-debug-b`:** 145 binaries, 787 passed, 0 failed, plus `conformance_fixtures --check`.
- **`test-release`:**
  - lane, math and wasm-gates: 28 binaries, 101 passed;
  - the M3 FMA cfg;
  - loom `spsc_loom`.
  - M1 and F1 were skipped, because `math_closure=false`.
- **`audit-native`:**
  - the release build;
  - release audit, bench and console-workload tests: 110 passed;
  - `audit capi` plus the CI validator: 0 allocations, 0 syscalls, 0 violations;
  - the delay, compressor, PEQ and gate audits;
  - the builtins, builtins-graph and graph traces, and the protocol allocation audit;
  - the realtime probes and the 1M-block trace, plus the builtins probes;
  - the effect-contract trace;
  - `check-capi-abi.sh` and `--self-test`;
  - `libcapi.so` scalar-absent;
  - graph determinism 100/100, plus `graph_fixture --check`;
  - builtins fixtures (50 files);
  - console fixtures, including the 64-track witnesses;
  - effect contract.
- **`wasm-guests`:**
  - the simd128 compile probe and the evidence crates;
  - protocol wasm parity;
  - `run-wasm-gates.sh --without-v8-spill --without-native`.
- **`cross-target`:** `check-cross-targets.sh` PASS.
- **`browser`:** **FAIL** on HEAD in all three browsers (BLOCKER-1). All three pass once the
  harness is fixed.

**Not run:**
- `aarch64-debug` and `aarch64-release`: this is an x86 host, so they run in CI at the push.
- The release unwind check: `release_inputs=false`, so the route does not run it.
- nightly and fuzz.

**Disk.** Free space stayed at or above 20 GB. The only growth was in pre-existing cargo caches
(`target/ci/cross-target` grew 4.1 -> 6.6 GB). I removed every directory I created: the probe,
loom and wasm-simd target dirs, the pulse socket dirs, the twin and HEAD copies, and `CARGO_HOME`.

## NITs (optional; none blocks)

1. **Leftover "track" wording on the master surface.**
   - `hosts/host-web/src/lib.rs:5652-5654` still says "a master designation must be a plausible
     track index ... refuses what cannot be a track index at all". This is the site #1208 NIT 2
     named.
   - `lib.rs:3510` says "no track was designated or the designated track published nothing".
   - `sdk/README.md:265` says "stable track/rack/effect-slot/tap IDs".
   - Fix them at the next `lib.rs` touch. Rewrapping moves panic-location bytes only, which is an
     honest ARTIFACT CHANGED.
2. **`docs/SESSION_SCHEMA_V1.md:179` is still 116 columns.** The reflow moved the overflow
   rather than removing it. K1 NIT-1 asked for about 100.
3. **#1214 amendment A1 lists only `observation-subscriptions.ts`.** The follow-up also edited
   the `track:` -> `strip:` stubs in `observation-subscription-evals.mjs`,
   `response-subscription-evals.mjs` and `spectrum-evals.mjs`. Those edits are mechanical. The
   A1 bullet also has a line of about 150 columns.
4. **`APP-LIVE.md` (or `APP-SDK.md`) could say that `SessionShape` gained a required `submixes`.**
   An app fake `scratchBoot` must return it (`[]` for no bus). TypeScript flags its absence. A
   JavaScript fake instead makes `observationMap()` throw, which is the BLOCKER-1 symptom.

## What blocks the push, and the order to unblock

1. Land BLOCKER-1's seven `submixes: [],` lines in
   `hosts/host-web/qualification/sdk-response-entry.ts`, and record them (#1210 amendment or a K2
   follow-up).
2. Re-run the three browser legs. In my run they pass with exactly that edit. Only
   `qualification/run.mjs` reads that file (it bundles it); no other gate does.
3. K2 is then ready to push. Deliver it as K1 was, through a PR merged into `main`.
   - The branch sits on `b6b1bdf4`, not on `main`'s `258e1008c`. The trees are identical, and
     `git merge-tree` is clean.
   - `main` is still at `258e1008c`.
