# Test usefulness audit, 2026-09-04 — index

## What this is

A per-file ledger of every test in the `misofm/engine` workspace, produced for the CI redesign in issue #359. For each test file it records the product claim the file protects, the kind of test it is, its cost class with the loop bound that justifies the estimate, any other test or script gate asserting the same claim, and a verdict. Its purpose is to let the CI redesign remove work without removing protection, and to make each removal arguable item by item rather than in bulk.

Scope: 1,550 Rust tests across 40 crates and tool packages, plus 80 non-Rust test and gate rows covering the SDK's node evals, the host-web JavaScript tests, the browser qualification harness, the script self-test suites, the fuzz targets, and every fixture directory's consumers.

**Nothing in this ledger has been applied.** It is a proposal. Each deletion requires Sol's per-item acceptance before any test is removed.

## Method

Every file in scope was read in full, including assertion bodies and loop bounds. Nothing was modified and `cargo` was not run, so every cost figure is estimated from the cited bound and corpus size, with figures that could not be corroborated marked "unverified" inline. Verdicts were produced by five parallel audit streams over disjoint parts of the tree and then consolidated; where two streams disagreed about the same test, both positions are recorded rather than silently reconciled.

The audit also flagged, per file: hidden global state, environment-variable reads inside tests, names that promise more than the assertion delivers, architecture-coupled skips that pass green without asserting anything, tests that are dead in CI behind an unenabled feature, duplication of a script gate, stale `MUTATIONS.md` references, and orphaned fixture directories.

## Rubric, in one sentence

A test is useful only if a plausible current code change would make it fail and no cheaper or stronger surviving test would also catch that change. See [`05-rubric.md`](05-rubric.md) for the full rule, the standing verdicts (sweeps trimmed to boundary plus one interior representative; tautologies, print-only and documentary tests deleted; wall-clock in debug moved to nightly), and the verdict vocabulary.

## The ledgers

| file | scope |
|---|---|
| [`01-foundation.md`](01-foundation.md) | engine, lane, math, session, source, protocol, capi, target-smoke, dsp-reference, conformance, effect-contract |
| [`02-dsp-effects.md`](02-dsp-effects.md) | effect-runtime and the ten dependent crates: parametric-eq, compressor, multiband-compressor, delay, gate-expander, true-peak-limiter, soft-clip, transient-shaper, effect-compiler, effect-package, plus the cross-crate per-effect patterns |
| [`03-compilers-hosts-tools.md`](03-compilers-hosts-tools.md) | rack, rack-compiler, builtins, builtins-compiler, graph, graph-compiler, host-core, hosts/host-web, and every `tools/*` package |
| [`04-non-rust.md`](04-non-rust.md) | sdk/test, hosts/host-web JS tests and the stem-store gate, browser qualification, script self-tests, fuzz targets, fixture consumers |
| [`05-rubric.md`](05-rubric.md) | the rule every verdict was measured against |

## Totals

| crate / scope | tests | keep | merge/trim | nightly or release-only | delete | est. debug s saved |
|---|---|---|---|---|---|---|
| engine | 37 | 33 | 4 | 0 | 2 | 3–5 |
| lane | 34 | 24 | 8 | 2 | 3 | 12–18 |
| math | 42 | 27 | 12 | 8 | 1 | 20–35 |
| session | 52 | 30 | 12 | 1 | 9 | 40–75 |
| source | 58 | 40 | 10 | 1 | 7 | 2–30 |
| protocol | 132 | 99 | 21 | 1 | 11 | 60–170 |
| capi | 32 (+2 C) | 22 | 9 | 0 | 1 | 8–15 |
| target-smoke | 2 | 0 | 0 | 0 | 2 | 0 |
| dsp-reference | 25 | 17 | 7 | 0 | 1 | <1 |
| conformance | 23 | 13 | 8 | 0 | 1 | <1 |
| effect-contract | 40 | 36 | 4 | 0 | 0 | <1 |
| effect-runtime | 85 | 73 | 9 (+2 move) | 0 | 3 | 1.5–2.5 |
| parametric-eq | 59 | 46 | 10 | 1 | 3 | ~30 |
| compressor | 63 | 42 | 15 | 0 | — (stall.rs plus the named tests) | 4–7 |
| multiband-compressor | 39 | 29 | 6 | 0 | 4 | 30–80 |
| delay | 16 | 13 | 2 | 0 | 1 | 5–15 |
| gate-expander | 27 | 20 | 5 | 0 | 2 | 2–5 |
| true-peak-limiter | 39 | 34 | 3 | 0 | 2 | 5–10 |
| soft-clip | 27 | 19 | 6 | 0 | 2 | 3–8 |
| transient-shaper | 22 | 17 | 3 | 0 | 2 | <1 |
| effect-compiler | 50 | 34 | 11 | 0 | 5 | 10–30 |
| effect-package | 88 (+1 C) | 63 | 14 | 2 | 9 | ~0.5 |
| tools/audit | 29 | 20 | 5 | 0 | 4 | 200–430 |
| rack | 34 | 30 | 3 | 0 | 2 | ≈0 |
| rack-compiler | 13 | 11 | 2 | 0 | 0 | ≈0.2 |
| host-core | 66 | 54 | 9 | 1 | 4 | 8–15 |
| builtins | 71 | 60 | 11 | 3 (release-only) | 3 | 20–35 |
| builtins-compiler | 28 | 22 | 5 | 1 (release-only) | 2 | 45–85 |
| graph | 45 | 36 | 7 | 0 | 3 | 8–15 |
| graph-compiler | 74 | ~48 | ~24 | 2 (1 release-only, 1 nightly) | 2–3 | 60–100 |
| hosts/host-web (Rust) | 60 | 48 | 10 | 1 (release-only) | 1 | 3–6 |
| tools/wasm-gates | 9 | 4 | 2 | 2 (release-only) | 1 | 38–81 |
| tools/parameter-metadata | 10 | 5 | 3 | 0 | 2 | ≈0 |
| tools/session-validator | 9 | 5 | 3 | 0 | 1 | <1 |
| tools/console-workload | 28 | 18 | 9 | 0 | 1 | 30–45 |
| tools/bench | 29 | 22 | 4 | 0 | 6 | 6–18 |
| tools/bench-support | 23 | 16 | 6 merge-or-delete (report does not split them) + 1 rewrite | 0 | — | ≈0 |
| tools/native-pcm-runner | 19 | 9 | 11 | 0 | 1 | 1–2 |
| tools/stem-hasher | 11 | 4 | 7 | 0 | 0 | ≈0 |
| hosts/host-native, hosts/host-mobile | 0 | — | — | — | — | 0 |
| non-Rust (sdk, host-web JS, qualification, script self-tests, fuzz, fixtures) | 80 rows (not test counts) | 49 | 17 (12 trim, 5 merge) | 4 | 10 | ≈120–360 CI job s |
| **total (Rust)** | **1,550** | **~1,140** | **~300 (+5 move)** | **27** | **~106** | **≈660–1,370** |

Reading the total row: the compressor delete count is not stated numerically in its ledger (arithmetic implies 6, not asserted here); `tools/bench-support` reports "merge/delete 6" without splitting the two; `graph-compiler` reports "delete 2–3" and approximate keep and merge counts. The Rust total therefore carries roughly six units of slack, all in the delete column. Non-Rust rows are gate rows rather than individual assertions, and its figure is CI job wall-clock rather than workspace debug seconds, so it is not added into the Rust total.

## Relationship to the #359 CI design

This ledger assumes the design already posted on #359: `tools/audit`, `console-workload` and `wasm-gates` tests move to a release shard, the ten per-crate release re-runs are deleted, and five named wall-clock assertions move to nightly. Two consequences follow. First, the DSP ledger states several savings as "doubled by the release leg"; once the ten re-runs are gone that doubling no longer applies, and the totals above are single-leg. Second, moving `tools/audit` to release does not by itself remove its cost: the restructuring in `03-compilers-hosts-tools.md` (render `generated()` once, call stage functions for post-oracle mutations) is what takes it from 233–470 s to 25–40 s.

The audit found at least four wall-clock assertions beyond the design's list of five: `source/src/native_source.rs:2076`, `sdk/test/boot-evals.mjs:189-217`, `hosts/host-web/tests/stem-store-core-v1.mjs:911-912` and `:325-330`.

## Known conflicts between the ledgers

These are recorded, not resolved. They must be settled before the affected tests are touched.

- **Who owns the corpus digest pins.** `01-foundation.md` says `math/tests/m3_determinism.rs:142` is the owning gate and G5's native math half is the duplicate. `02-dsp-effects.md` says G5 owns every delegated family and the nine per-crate pin tests should go. `03-compilers-hosts-tools.md` says the opposite, that each owning crate pins natively in release and `g5_native_digests_match_pins` should go. Whichever direction is chosen, the per-family finiteness and non-vacuity checks must be preserved, because G5's vacuity checks exclude delegated families.
- **`protocol/tests/controller_response_api.rs`.** `01-foundation.md` says delete; the #359 design says keep as a zero-cost type-level claim.
- **The 65,537-track scale tests.** The design keeps all four in `test-debug-a`; `03-compilers-hosts-tools.md` moves two to release or nightly and trims graph-compiler's to 10,923 tracks; `01-foundation.md` trims session's to about 4,097.
- **`effect-package/tests/effect_interchange_mutation.rs:318`.** The design says delete (ignored, no assertion); `02-dsp-effects.md` says keep as nightly while agreeing it asserts nothing today.
