# Document host exhaustion test seam and make telemetry fixture deterministic

**Status:** Sol brief approved by root; matching GitHub issue misofm/engine#409. This is a candid merge-blocker correction for PR #398 / qualification run `33935625430`; it is not issue #393 attempt 4. Issue #393 already exhausted its three attempts and received Astra PASS at `bed7634c`. Its host product result remains closed.

## Smallest closable slice

Make the two required CI checks deterministic and truthful without changing browser-host behavior:

1. Document the existing private safe-integer test selector `MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST` in the enforced environment vocabulary. It was introduced in reviewed #393 commit `beeb8557a`, is set only by `scripts/test-web-audioworklet.sh`, and selects the transformed-host exhaustion branch in `scripts/test-web-audioworklet.mjs`. No variable is renamed and no second selector is added.
2. Replace the telemetry fixture's use of the Node process's real `performance.now()` with a test-local deterministic clock installed only while the telemetry processors are constructed and exercised. Keep exact assertions for zero misses and add a positive case with exactly one injected over-budget block.

The second failure predates #393: `scripts/test-web-audioworklet.mjs:1502` and its `deadlineMisses === 0` assertion are from #137 baseline `1f8d3f0df`. Node 22.23.2 observed one scheduler-sensitive miss. Record that as a real-clock test-fixture defect, not a Node product regression established by evidence.

## Exact correction

### Vocabulary

Add one row beside the existing host test-module selector in `docs/ENGINE_ENV_VOCABULARY.md`:

`MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST` — hermetic host allocator test selector; when `1`, the existing test runs the transformed private counter at `MAX_SAFE_INTEGER - 1`, proves the final safe ID once, then proves repeatable local exhaustion with no post, wrap or reuse.

The existing bidirectional checker and its self-test already discriminate undocumented and unused rows. Do not exempt a path, weaken `check-env-vocabulary.sh`, rename the environment variable, add an alias, or edit the #393 implementation/spec merely to hide the failure.

### Deterministic telemetry clock

In `scripts/test-web-audioworklet.mjs`, keep the existing worklet source and `makeProcessor()` path. Add a small local clock fixture that returns monotonic start/end samples for each render block. Temporarily replace `globalThis.performance` (preserving/restoring its original value in `finally`) before constructing the processor, so the existing `renderClock()` probe and `telemetryMessage.resolutionMs` use the same injected clock naturally. Do not assign `processor.clock` after construction or patch the imported worklet.

Run two exact 128-block windows through the normal `process()` path:

- a no-miss window whose per-block elapsed value is a fixed positive duration safely below the 64-frame/48-kHz budget; require one telemetry frame and `deadlineMisses === 0`;
- a fresh processor/window with that same duration except for exactly one elapsed value above budget; require one telemetry frame and `deadlineMisses === 1`.

Retain the existing assertions for block/window count, sequence, budget range, positive reported resolution, `belowResolution`, finite/nonnegative CPU fields, lease release, and no clock reads/messages after release. The injected clock should count reads so the fixture also proves two reads per leased rendered block and no reads after release. Restore the real global clock even if an assertion fails.

The exact one-miss case is the red discriminator: a correction that clamps, ignores or loosens deadline misses must fail. Do not replace `=== 0` with a range, retry the test, increase the budget, skip on Node 22, sleep, mock render output, or change telemetry/product arithmetic.

## Exact allowed paths

- `.github/ISSUE_SPECS/409-document-host-exhaustion-test-seam-and-make-telemetry-fixture-deterministic.md` — new stateless tooling successor and evidence
- `docs/ENGINE_ENV_VOCABULARY.md`
- `scripts/test-web-audioworklet.mjs`

No other tracked path is allowed. In particular, do not edit `hosts/host-web/**`, SDK source/declarations, `scripts/test-web-audioworklet.sh`, either environment-vocabulary checker/test, workflows, generated assets, Rust/Wasm/ABI files, issue #393, or issue #405. If the existing shell wrapper cannot pass after only these corrections, stop and amend this successor rather than broadening it during implementation.

## Gates and evidence

1. `bash scripts/check-env-vocabulary.sh` passes and reports the incremented documented-name count.
2. `bash scripts/test-env-vocabulary.sh` passes its existing undocumented-name, unused-row and deleted-row red cases unchanged.
3. On Node 22.23.2, `node scripts/test-web-audioworklet.mjs` passes once and records the deterministic `0`-miss and injected `1`-miss windows. One invocation is evidence; no retry loop or repeated-until-green run.
4. `bash scripts/test-web-audioworklet.sh` passes, including the existing transformed safe-integer host and unchecked-increment red mutation. The selector remains private to the harness.
5. Run the proportional qualification lint/test route that failed in run `33935625430`, then `git diff --check` and an exact-path audit. The diff must contain zero host/product/worklet/generated changes.

The issue evidence must say plainly that the real-clock zero-miss test was scheduler-sensitive and that the deterministic one-miss case preserves the behavioral assertion. It must not claim a Node 22 engine bug or new host qualification.

## Delivery and review

Root creates and synchronizes this separately numbered tooling issue before implementation. Luna implements one coherent correction checkpoint; a dedicated Astra review verifies the exact diff, runs the zero/one-miss discriminator and the unchanged safe-integer mutation, and confirms no product file changed. Because this successor is independent from closed #393, its attempt count starts at one. Merge PR #398 only after the required qualification context is green and the successor evidence is upstream; do not reopen or amend #393 as a disguised fourth attempt.

## Decision record

- 2026-09-05: Root approved this independent tooling correction in isolated /private/tmp/miso-dx-ci, branch codex/dx-ci-fixtures. It can run alongside SDK #405 without shared edits or broad workspace gates. Luna implements; dedicated Astra reviews. No product scope is added. Root checkpoints each coherent tranche before more edits.
- 2026-09-05 attempt 1 evidence: `docs/ENGINE_ENV_VOCABULARY.md` now documents the existing private `MISO_ENGINE_WEB_HOST_MAX_SAFE_TEST` selector, and `scripts/test-web-audioworklet.mjs` runs two fresh 128-block telemetry windows through `process()`: a fixed positive 0-miss window and a same-duration window with exactly one injected over-budget block. The local clock fixture proves 256 reads (two per leased block) and no reads or messages after release, while restoring the original `globalThis.performance` descriptor in `finally`.
- Focused gates pass on Node `v22.23.2`: `/private/tmp/node-v22.23.2-darwin-arm64/bin/node scripts/test-web-audioworklet.mjs`; `bash scripts/check-env-vocabulary.sh` reports 99 documented names; `PATH=/opt/homebrew/opt/gnu-sed/libexec/gnubin:$PATH bash scripts/test-env-vocabulary.sh` passes unchanged mutation coverage; and the full `PATH=/opt/homebrew/opt/gnu-sed/libexec/gnubin:$PATH bash scripts/test-web-audioworklet.sh` passes, including the safe-integer boundary and unchecked-increment red mutation.
- The prior real-clock zero-miss assertion was scheduler-sensitive; the deterministic one-miss case keeps the behavioral `deadlineMisses === 1` discriminator. This evidence does not claim a Node 22 engine bug or new host qualification. `git diff --check` passes and the exact diff contains only the issue spec, environment vocabulary, and test harness paths; no host/product/worklet/generated file changed.

## Dedicated Astra attempt 1 verdict — PASS (2026-09-05)

Astra independently reviewed `04bbf4e5` and verified exact three-path scope, focused Node22.23.2 suite, 99-name vocabulary and unchanged mutation suite, and full unchanged browser wrapper. Force-zero, count-every-block, and read-after-release mutations each fail. Original performance property descriptors restore on normal completion and callback exceptions. No product source changed. The full review is attached to PR #398. This tooling result fixes the demonstrated CI causes; combined PR qualification still requires its own green run and separate PCM review.
