# Hermetically qualify the browser harness boot contract

## Status and scope authority

Astra medium scope PASS, revalidated against merged main fe9fc8d4e29ca1d6bca3e101b68b43b05f8078b9 after #739 delivery. Current GitHub #288 has no comments and no recorded failed implementation attempts. This is attempt1 of at most5: Astra medium scopes and verifies; Luna xhigh implements; root owns exact-path commits, pushes, GitHub synchronization, CI and cleanup. Latest owner routing overrides older model names.

The smallest closable outcome is a fast Node test proving the browser qualification harness's actual boot call arguments pass the current shipped host/worklet argument guards. A stale caller spelling must fail this test before artifact build/download or browser installation is needed. No rendered-audio or browser-support claim is made by these mocks.

## Current finding

The historical issue names a deleted `scripts/sweep.sh`; do not revive it. `hosts/host-web/qualification/session-identities.mjs` already imports qualification.js and validates source identities, but does not execute its boot calls. Raw-Wasm `scripts/check-web-boot-budget.mjs` validates memory sizing, not the JS caller's strict object contract.

Actual caller sites in qualification.js are `renderCorpusSegment`, `typedUnsupportedAttestation`, `runConsoleQualification`, `runObservationRun`, and `runStallQualification`. All invoke their supplied createHost with the shared bootOptions helper. `diagnoseReady` separately builds AudioWorkletNode processorOptions. Current production guards are `createMisoAudioWorkletHost` in the host file and `initialize` in the worklet class. Both reject stale/missing/extra fields. Tests must execute these existing guards, not regex-extract arrays and compare copied field lists.

Existing `scripts/test-web-audioworklet.mjs` already provides fake host fetch/compile/context/node/ready-port behavior and a registered real worklet class with fake WebAssembly exports. Reuse these bounded fixtures locally. No generic new mocking framework, parser dependency, production validation export or ABI change is needed.

## Frozen implementation plan

1. Make the six existing qualification functions directly callable by the hermetic test through a small qualification-only named export or frozen export object. These must reference the actual existing functions; do not create parallel option constructors or change the functions' browser behavior. Existing runQualification continues using those same functions.

2. Extend the existing hermetic mjs suite to invoke all five createHost paths with a minimal nonempty Uint8Array document and the existing fake OfflineAudioContext shape. Inject a wrapper that forwards the argument unchanged to the REAL `createMisoAudioWorkletHost`, observes successful ready/backend, then disposes/stops through a distinct test sentinel before the later render/stall/source loops. For typedUnsupportedAttestation, account explicitly for its catch: prove the wrapper was reached and the real guard accepted, rather than treating any caught exception/false return as success. Require exactly the expected caller witnesses, nonzero documents, correct context quantum/rate and accepted boot variants (plain, console, observation). No busyWait, network, browser, Cargo, timing, or real Wasm required.

3. Execute diagnoseReady to its actual node construction. Route its captured processorOptions through the REAL registered worklet class/initialize using the existing fake Wasm export fixture; prove the initializer passes its strict fields and produces ready rather than invalid-argument/internal failure. Fake context/addModule/fetch/compile/node/port may be reused or minimally extended. Bound all promises: malformed input cannot hang the test; restore every patched global in finally. This qualifies the direct diagnostic boot shape only.

4. Add narrow qualification-source mutations using existing shell mutation conventions and a test-module override via CLI argument (preferred over a new environment vocabulary): historical outer `document`->`sessionDocument` at a real caller; missing or renamed shared inner boot key; and diagnostic `options`->`limits`. Include an extra-key negative against a real caller if not already demonstrated by the shared guard suite. Each mutant must actually change its unique intended source site, load the mutated qualification module, and cause the SAME hermetic check to exit nonzero with the expected caller/invalid-contract diagnostic. A missing import/path, unchanged mutation, sentinel mistake or unrelated parse error is not an accepted RED. Use temporary files outside repository source, preserving qualification.js's relative hex helper import via an explicitly resolved module URL/copy; clean them in traps. Do not weaken existing host/worklet mutation coverage.

5. Add an early `node scripts/test-web-audioworklet.mjs` step to qualification.yml's existing lint/hermetic job, before long Rust gates where convenient. The existing artifact-gates shell wrapper remains and runs the new source mutations with its existing broader mutations; no new job or required context. The direct Node run is intentionally fast and artifact-independent, so this caller mismatch reports without waiting for artifacts. All changed script/host/workflow paths already route full under the fail-safe router, so no router policy change is needed. Verify that rather than introducing path filters.

## Allowed paths

- `.github/ISSUE_SPECS/288-qualification-harness-boot-contract.md` (root installs/synchronizes this existing issue's scope and evidence).
- `hosts/host-web/qualification/qualification.js`: qualification-only test export of the real functions, and only necessary explanatory comments.
- `scripts/test-web-audioworklet.mjs`: bounded reuse/extension of current mocks and caller/initializer witnesses; narrow optional qualification-module CLI override for mutation testing.
- `scripts/test-web-audioworklet.sh`: qualification-source negative mutations and safe temporary cleanup, preserving all current checks.
- `.github/workflows/qualification.yml`: one early hermetic Node step in the existing lint job.

No shipped host/worklet source changes, artifact pins, generated SDK assets, dependencies/lockfiles, broad CI movement, source-scanner repairs, field-list parsing framework, sweep resurrection or rendering changes. If existing mocks cannot cover the boundary without materially expanding the harness, stop and report a smaller concrete seam before adding a second framework.

## Objective gates and checkpoint

- Positive execution covers all five real host caller functions and direct diagnoseReady init, with counted successful real-guard witnesses; all async paths terminate and no browser/artifact/network fetch occurs outside mocks.
- Distinct plain/console/observation options reach the real guard intact, and diagnoseReady produces ready from the real worklet initializer with mocked exports.
- Historical outer rename, shared boot-key change and direct diagnostic rename all fail for their intended contract reason. Extra/missing fields remain strict. Existing main-realm/worklet assertions and mutations remain green.
- `node scripts/test-web-audioworklet.mjs`.
- `bash scripts/test-web-audioworklet.sh` (existing full wrapper; no artifact argument or timed workload).
- `python3 -B scripts/check-ci-path-routing.py` and `python3 -B scripts/test-ci-path-routing.py`; use the existing router name-status seam to confirm qualification.js, both modified scripts and qualification.yml select full if not covered by these tests. No router edit required.
- `git diff --check`; scoped JS syntax checks if not already proven by execution. No Cargo/audio/browser/benchmark matrix needed locally for unchanged production code.

As soon as bounded positive/negative implementation gates pass, Luna pauses for root's exact-path checkpoint; do not layer another issue before root commit/upstream audit. Astra medium supplies one coherent adversarial verdict, checking mutation validity and actual production guard execution rather than copied schema assertions. Root then preserves concise evidence upstream, obtains required exact PR/main qualification, synchronizes/closes existing #288 and cleans the delivered worktree.

## Boundary and limitations

This closes an actual cheap-preflight gap rather than administratively closing stale wording. It does not claim that mocked boot proves real Wasm readiness, PCM quality, browser support, source identity or memory bounds; existing artifact/browser gates retain those claims. The old npm-installed source-scan caveat belongs elsewhere and is not needed for this slice.

## Merged-base revalidation

Root delivered #739 through PR749 at actual main fe9fc8d4e29ca1d6bca3e101b68b43b05f8078b9. Read-only comparison of all four proposed implementation paths against scoped PR head b5769d5a is empty. Scope PASS therefore applies unchanged to that merged main. Root still waits for its required main qualification and completes #739 issue synchronization before activating #288. No tests or implementation were performed in this scope turn.


## Activation — 2026-09-11

Root verified #739 CLOSED after PR749, exact PR/main qualification PASS,
remote evidence synchronization and clean delivered-worktree removal.
Issue-boundary reconciliation found no missing remote numbered issue.
The existing issue288 is retitled to match this stateless scope; no new
issue number is allocated. Implement one bounded tranche on dedicated
branch codex/qualification-harness-boot-288 with checkpoint pushes.
