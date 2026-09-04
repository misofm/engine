# Test usefulness ledger 4 — non-Rust tests, script self-tests, fuzz and fixtures

## Non-Rust test audit — /home/bl/misofm/engine

Method: read every file in scope; ran only hermetic node/python self-tests (timings measured on this machine). Nothing modified, no cargo. Forks covered sdk/test and hosts/host-web/tests + stem-store gate; I did qualification, scripts, fuzz, fixtures directly.

### Scope 1 — sdk/test/** (gate: `scripts/check-sdk-headless.sh` → `node --test 'test/*-evals.mjs'`; `scripts/sdk-package.sh check`; `scripts/check-sdk-types.sh` → tsc; sdk.yml + npm-publish.yml)

| path | #tests | invoked by | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| sdk/test/boot-evals.mjs | 16 | check-sdk-headless.sh | headless boot reads engine shape, typed refusals, validate() never recompiles, oversize refuses w/o growth | behaviour + timing (L189-217 `<1000ms`) | small (~12 wasm boots, 1 MiB doc) | oversize half dup of hosts/host-web/src/tests.rs:111-142 + tests/boot_transient_budget.rs | KEEP; NIGHTLY L189-217; DELETE tautology L322-323 |
| sdk/test/browser-evals.mjs | 16 | same | scratch/worklet boot option blocks agree; refusals precede AudioContext; close idempotent | behaviour | small | none found | KEEP |
| sdk/test/builder-evals.mjs | 38 | same | builder output plan-equal, canonical bytes match Rust corpus + fixtures/session-canonical, refusals name paths | behaviour + digest-pin + mutation-proof (L147) | medium (~10 boots) | L593 "engine refuses unknown port" is a Rust-side claim (crates/session, unverified line); corpus half intentionally twins crates/session/src/canonical.rs | KEEP; MERGE/drop L593. Env `MISO_ENGINE_SDK_SKIP_ASSET` (L36) set by nothing |
| sdk/test/writer-evals.mjs | 13 | same | ConsoleWriter queue capacity/coalescing/async race safety | behaviour + scale (L380 hundreds of records) | medium (~8 boots, render loops) | capacity constants also at Rust boundary (unverified) | KEEP |
| sdk/test/agent-evals.mjs | 9 | same | SDK lattice reproduces engine per-parameter digest (≥60 params) | property/oracle | **heavy: `cargo run -p parameter-metadata --bin parameter_metadata_lattice_oracle` inside node test (L42)** | tools/parameter-metadata/tests/round_trip.rs covers Rust side only | KEEP claim; TRIM: precompute oracle TSV once in the gate / check in as codegen fixture |
| sdk/test/render-evals.mjs | 4 | same | wasm render digest == native `sdk_render_oracle` for 3 configs | property/oracle | **heavy: `cargo run -p host-web --example sdk_render_oracle` ×3 (L31)** | native↔wasm parity already gated by run-wasm-gates.sh / check-protocol-wasm-parity.sh; only SDK marshalling is new | TRIM to one document with a fixture-pinned native digest; remove in-test cargo build |
| sdk/test/capability-evals.mjs | 4 | same | status/sessionMap/seek/meter-lease expose compiled addressing | behaviour | small | Rust twins hosts/host-web/src/tests.rs:751 (partial) | KEEP |
| sdk/test/console-evals.mjs | 4 | same | all 11 command kinds build+admit; unknown track refuses pre-transport; torn ack rejected | behaviour | small | kind vocabulary pinned by check-command-kind-vocabulary.py; builder semantics unique | KEEP |
| sdk/test/package-evals.mjs | 9 | same (0.10 s measured) | export-map shape, README imports resolve, barrel === deep imports | behaviour | trivial | export-map + serializer-absence duplicated in package-tarball-smoke.mjs:14-45 and barrel-surface.ts:135-141 | MERGE into tarball smoke; keep README + identity tests |
| sdk/test/headless-path-evals.mjs | 19 | same (0.54 s measured; self-referential — gate glob runs a test of the gate) | check-sdk-headless.sh preserves path bytes, ignores CDPATH, refuses symlinks, propagates exit codes | behaviour (bash under fake node) | small (~25 bash spawns) | none | TRIM: move out of `*-evals.mjs` glob to its own row; DELETE tautology L153-156 |
| sdk/test/enginectl-cli.mjs | 11 | sdk-package.sh:48 | built enginectl: help/version, stdin→canonical JSON, refusals, receipts | behaviour | medium (~14 spawns booting wasm) | tarball smoke L146-200 re-runs one request path | KEEP |
| sdk/test/package-tarball-smoke.mjs | 43 asserts | sdk-package.sh:65; npm-publish.yml:170,:209 | packed tarball export map, .d.ts, asset closure, strict-TS consumer, boot+render, tamper detection | behaviour + compile | medium (tsc + 3 boots) | runs 3× per release path; publish-mode re-smoke (:209) adds nothing beyond shasum check at :211-220 | KEEP; TRIM the :209 re-smoke (not per-PR) |
| sdk/test/support.mjs | 0 | imported | fixtures | descriptive | trivial | dead `quoteKeys` option never read (boot-evals L44/61/121, browser-evals L201) | KEEP; delete dead option |
| sdk/test/host-mirror.ts | 3 type pins | check-sdk-types.sh | SDK adapter returns exactly shipped `MisoWebBootOptions` | compile-only | trivial | none | KEEP |
| sdk/test/barrel-surface.ts | 35 pins | same | barrel exports declaration-identical to deep imports | compile-only | trivial | runtime half in package-evals | KEEP |
| sdk/test/console-types.ts | 7 `@ts-expect-error` | same | catalog-derived console API rejects bad bags at compile time | compile-only | trivial | none | KEEP |
| sdk/test/port-types.ts | 9 | same | sidechain portId typed from descriptor rows | compile-only | trivial | runtime half builder-evals L493-592 | KEEP |

No `test(`/`assert` under sdk/src or sdk/codegen.

### Scope 2a — hosts/host-web/tests/** (all three stem .mjs invoked only via `scripts/check-stem-store-v1.mjs`, ci.yml:267; hermetic, measured hash 0.26 s / core 0.37 s / pump 0.19 s)

| path | #tests | invoked by | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tests/stem-store-hash-v1.mjs | 3 KAT × 7 chunkings + 4 scenarios | check-stem-store-v1.mjs:21 | incremental SHA-256 matches KATs at every chunk boundary; range resume/stall | property/oracle | small (1 MB ×7) | none (JS-only hasher) | KEEP |
| tests/stem-store-core-v1.mjs | 17 scenarios, 118 asserts | :21 | OPFS store dedupe, two-tab locking, self-heal, atomic promote, LRU, abort, lying-declaration refusal | behaviour; **timing L911-912 (`<5000`/`<2000` ms, measured 69/33 ms)**, L325-330 (`<100 ms` abort vs 300 ms race, flaky-risk) | small | self-test ledger proves it catches 20 regressions; unique | KEEP; NIGHTLY L911-913; L325-330 unverified flake |
| tests/stem-pump-v1.mjs | 12 scenarios, 55 asserts | :21 | worker PCM pump ring transcript, seeks, hard-stop, gesture gate, idle timer cancel | behaviour; L599-604 `Date.now()-started<1000` **tautology** (until(...,1000) at L601 already throws); real sleeps L531/561/609 | small | none | KEEP; DELETE L599-604 |
| tests/stem-store-fakes.mjs | 0 | imported | fakes | support | trivial | — | KEEP |
| tests/browser-v1/direct-oracle.mjs | 3 legs, 67 asserts + deepEqual vs expected.json L574 | web-audioworklet-browser-correctness.py --check (ci:436) **and again** in check-browser-expected-resources.py --artifacts print mode (ci:439) | raw simd128 wasm renders bit-identical PCM to native pin for 3 sessions; memory never grows | digest-pin + behaviour | small-medium (4 instantiations; needs built artifact) | native halves at hosts/host-web/src/tests.rs:1116/1242/2748 (intended two-leg parity); **executed twice per CI job** | KEEP; MERGE the two invocations |
| tests/browser-v1/browser-correctness.js | 0 asserts (5 throws) | **not in any workflow** (operator seal scripts only) | real-browser render equals expected.json | behaviour (browser) | heavy when run; 0/PR | qualification run.mjs native-corpus gate ×3 browsers overlaps | KEEP as operator tool; flag unwired |
| tests/browser-v1/*.json | fixtures | direct-oracle, tests.rs include_str, py, qualification/server.mjs | — | — | trivial | — | KEEP |

**scripts/check-stem-store-v1.mjs --self-test** (ci.yml:267; gate alone 0.90 s, with self-test 4.71 s → ledger ≈3.8 s, ~165 ms/mutation): 22 mutations (L135-330) + 1 in-memory regex tripwire (L379-383). Each mutation `cp(hosts/host-web → tmp, recursive)` (L334-337, 54 files incl. `qualification/` — drags a local playwright `node_modules` if installed), edits one file, runs one full test file, requires non-zero exit; 16/22 pin an `expectedFailure` substring.

| mutations | verdict |
|---|---|
| #1-#19, #21, #22 (content-key dir, drop Web Lock, skip staging verify, index-before-promote, gate interactive, trust warm filename, match one contention error, disable decoder deadline, omit AbortSignal, retain predecessor pin, missing index=empty, survivor evictable, zero shortfall, direct fallback write, ladder #15-17, drop verify-on-open, fill staging before refuse, pump drives by default, stop leaves idle sleep) | KEEP; pin `expectedFailure` on #2-#7 (L147-197, currently any-red) |
| #7 (L188) | MERGE candidate: static check L65-72 already forbids error-name matching |
| #20 (L296-303) "drop every open-time byte check" | DELETE: same expected string as #17, 4th arm covered by #18 |
| tripwire L379 | tautology-adjacent (regex vs inline string); fold or keep, trivial |
| copy mechanism L334-337 | TRIM: copy only `web/stem-store/` + `tests/` (17 files) or write mutated module to temp path; ~1-2 s saved |

### Scope 2b — hosts/host-web/qualification/** (browser-qualification.yml: 3-job matrix `npm run qualify -- --artifacts … --browser X --check-matrix --self-test-mutations`, plus `npm run matrix -- --check`)

| path / gate | #cases | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|
| run.mjs `validate()` L69-171 (attestation, boot, native-corpus-digest, control-path ×4, observation ×7, stall ×2) | 17 gates | browser result carries simd128 attestation, bit-exact corpus/console/observation/stall output | behaviour (browser) | heavy: browser launch, 1 page, ~4 renders; playwright install per job | corpus digest overlaps direct-oracle.mjs (Node) but the browser leg is the point | KEEP (×3 browsers is the claim) |
| run.mjs `mutationProofs` L173-197 (12 named mutations, in-memory `structuredClone`) | 12 | each gate goes red on its named mutation | mutation-proof | trivial (in-process) | **browser-independent, identical in all 3 jobs** | TRIM: run once (e.g. in `artifact` job or chromium only) |
| run.mjs `artifactSetProofs` L210-262 (6 removals + 1 stray + 6 substitutions + 1 dir-named-artifact = 14 fs mutations, each `cp -r` of the 6-file dir) | 14 | server refuses anything but the exact six-file set | mutation-proof | small (14 dir copies of ~2 MB) | **same claim as scripts/check-web-audioworklet.sh:130-142** (exact six frozen outputs) in ci.yml; and ×3 jobs | TRIM: run once; or DELETE substitution loop (13 rows prove one `Set` comparison) |
| run.mjs `validateLineage` + `lineageMutationProofs` L47-64 | 2 + 2 | results.json commit is 40-hex and wasmSha256 == artifact | digest-pin + mutation-proof | trivial | ×3 jobs | TRIM to once |
| run.mjs `validateCheckedRow` L290-305 | 1 + 1 red | this run's browser floor/outcome equals results.json row | digest-pin (browser version string!) | trivial | — | KEEP but note: fails on every Playwright bump; correct by design, unverified whether that is wanted per PR |
| run.mjs L319-322 playwright-version + generated-document | 2 | results.json playwright == package; BROWSER_DEPLOYMENT_MATRIX.md regenerated | digest-pin | trivial | **identical to `npm run matrix -- --check`** (browser-qualification.yml:105) and ×3 jobs | MERGE: keep only the workflow step |
| session-identities.mjs `checkSessionIdentities` (run first, L308) | 3 docs × (1 derive + 4 red mutations) | each session doc's `content` = sha256 of PCM actually fed | property/oracle + mutation-proof | trivial | ×3 jobs; hash also implemented in tools/stem-hasher (Rust) — different question | KEEP; run once |
| generate-matrix.mjs `--check` | 1 | md == render(results.json) | digest-pin | trivial | dup of run.mjs L321 | KEEP this one, drop the run.mjs copy |
| qualification.js | ~20 throws; busy-wait stall L218-222 (by design, ≥100 ms); 8×4 ms polls L341,449 | in-browser harness; not a test file | behaviour | heavy | — | KEEP |
| global-probe.js, server.mjs | 0 | helpers | — | — | — | KEEP |
| results.json, *-session.json | fixtures | consumed by run.mjs / qualification.js / session-identities | — | — | — | KEEP |

Per-PR cost: 3 × (playwright install + artifact download + launch); the trimmable Node-only proofs are ~1-3 s/job (unverified), the browser gates themselves are the cost and are KEEP.

### Scope 3 — scripts self-tests (all measured hermetic; `bash scripts/test-web-audioworklet.sh` from ci.yml:444 also runs external mutations and 3 `cargo run -p parameter-metadata`)

| path | #cases | invoked by | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| scripts/test-web-audioworklet.mjs | 216 asserts (2 top-level suites L1417-1418; no `test(`) | test-web-audioworklet.sh; twice more under host/worklet mutation env (.sh L~230-250) | host + processor JS behaviour with fake exports, typed errors, UTF-8 parity, console/observation | behaviour | trivial (0.037 s) | none | KEEP |
| check-web-audioworklet-callgraph.py --self-test | ~25 `expect` (L423-662) | test-web-audioworklet.sh; check-web-audioworklet.sh:294-318 uses it as gate | analyser flags dlmalloc free/trap/panic in render closure; roster de-vectorisation caught; ambiguous/missing roster red | mutation-proof (in-memory) | trivial (0.021 s) | none | KEEP |
| check-abi-layout-v1.py --self-test | 17 mutations (L347-364, in-memory over scripts/fixtures/abi-layout-v1-self-test.json) | check-web-audioworklet.sh:373 gate; self-test invoked where? (unverified — not seen in ci.yml/test-web-audioworklet.sh; tools/parameter-metadata/tests/abi_layout.rs:389 pins the fixture) | schema gate rejects dropped alias/field/export, holes, stale ABI version | mutation-proof | trivial (0.027 s) | none | KEEP; wire `--self-test` into ci if it truly is not (unverified) |
| check-command-kind-vocabulary.py --self-test | 32 in-memory mutations | test-web-audioworklet.sh | seven spellings of command kinds agree | mutation-proof | trivial (0.052 s) | **test-web-audioworklet.sh kind_dir block (COMMAND_SOLO_MODE bump; `Set([1..6])`) re-does self-test mutations #1 and #6 via tmp-dir file copies** | KEEP self-test; DELETE the .sh kind_dir block |
| check-command-reason-vocabulary.py --self-test | 18 in-memory mutations | test-web-audioworklet.sh | six spellings of command reasons agree; host derives bound from table | mutation-proof | trivial (0.035 s) | **.sh vocabulary_dir block (COMMAND_REASON_FUTURE_TAP) = self-test #1**; .sh reason-cap block (`<= 9`) = self-test #4 *but* runs product tests against mutated host — distinct detector, keep that one | KEEP self-test; DELETE .sh vocabulary_dir block |
| check-parameter-metadata-v1.py --self-test | ~33 in-memory mutations (L494-805, over scripts/fixtures/parameter-metadata-v1-self-test.json) | test-web-audioworklet.sh; gate at check-web-audioworklet.sh:367 | metadata schema/lattice/plane/reason rows are consistent | mutation-proof | trivial (0.082 s) | .sh `liveUpdatable` hand-edit block duplicates self-test L99/106/116; the Rust `--check` refusal half is covered by tools/parameter-metadata/tests/round_trip.rs:184 (hand-edit red, unverified exact overlap) | KEEP self-test; TRIM .sh block: 3× `cargo run -p parameter-metadata` (medium, native build) for a claim already unit-tested |
| check-session-map-shape.py --self-test | 15 in-memory mutations | test-web-audioworklet.sh | export gate / worklet / host JS / .d.ts / Rust ffi agree on session-map row shape | mutation-proof | trivial (0.030 s) | **.sh map_dir block (`frames: bigint→number`; drop "sources") = self-test mutations L53 and L29-49** | KEEP self-test; DELETE .sh map_dir block |
| check-browser-expected-resources.py --self-test | 26 mutations (L600-700; subprocess-free, in-memory) | ci.yml:442 | expected.json pins target-independent rows, digests, bootOptions words, class partition | mutation-proof | trivial (0.028 s) | none | KEEP |
| check-web-boot-budget.mjs | 2 blocks, 12 asserts (not a self-test) | check-web-audioworklet.sh:401 | wasm high-water growth ≤ 17×1 MiB + 1 page; pin-1 refuses with result 5 and no growth | behaviour/property | **medium-heavy: `cargo run -p host-web --example worst_boot_document` (native host-web build) + 2 wasm instantiations** | native peak twin hosts/host-web/tests/boot_transient_budget.rs:161 (System allocator, not wasm — different allocator so not redundant); refusal half dup of sdk boot-evals L128-187 + host-web/src/tests.rs:111-142 | KEEP growth assertion; TRIM: have the example emit the doc once into the artifact dir at build time to drop the cargo run |
| web-audioworklet-browser-correctness.py --self-test-webdriver-responses (+ 1 sed mutation in .sh) | unverified count | test-web-audioworklet.sh | WebDriver null-response handling | mutation-proof | trivial (0.56 s) | none | KEEP |
| check-web-audioworklet.sh --self-test-opcodes / --source-policy + 4 process-helper + 4 console + 1 clock mutations (.sh) | 9 sed/awk mutations | test-web-audioworklet.sh | source policy catches allocation/postMessage/BigInt/grow/clock-read in process() | mutation-proof | trivial (awk/sed) | none | KEEP |

### Scope 4 — fuzz/

| target | claim | kind | per-PR (fuzz.yml, path-filtered) | nightly | native redundancy | verdict |
|---|---|---|---|---|---|---|
| session_parse.rs | parse→canonical→reparse is a fixed point | property | 10 000 runs, seed 557074001, seeded from fixtures/session/v1/canonical.json | 180 s | crates/session/tests/fuzz_smoke.rs (4 096 deterministic bit-flips, same invariant) + canonical_schema.rs:33-82 | NIGHTLY only (per-PR bounded run adds nothing: fuzz.yml header says libFuzzer finishes "under a second"; the 19.9 job-min/25 runs is toolchain + sanitizer build) |
| session_compile.rs | compile on accepted parse: schema_version==1 and canonical JSON reparses | property | 10 000 runs, seed 557074002 | 180 s | fuzz_smoke.rs also calls compile_session over the same mutations (no post-assert, weaker) | NIGHTLY only |
| protocol_command / _response / _event / _session_transaction (.rs + protocol_support.rs `assert_stable`) | same bytes decode to the same class twice; >4096 bytes → LimitExceeded | property (weak: idempotence + one limit) | run-protocol-fuzz.sh: 4 × 10 000 runs, fixed seeds, corpus 1 hex seed each; emits "cumulative 100000" evidence JSON | 180 s each | crates/protocol/tests/mutation_million.rs (1 000 000 deterministic mutations over complete_schema_corpus, all typed dispatch) — strictly stronger and in `cargo test` | NIGHTLY only; the evidence JSON is descriptive |
| effect_package.rs | verify_effect_package never panics | crash-only | not per-PR (cargo check only) | 180 s | effect-package/tests/effect_interchange_mutation.rs:316-324 deterministic campaign | NIGHTLY (as is) |
| effect_state.rs | verify_effect_state against a fixed bound descriptor never panics | crash-only | not per-PR | 180 s | state_vectors.rs:1788 representative mutations | NIGHTLY (as is) |
| corpus/ (4 hex seeds, README, complete-schema-manifest.md) | seeds + documented hash | descriptive | — | consumed by nightly + run-protocol-fuzz.sh | — | KEEP |
| Cargo.toml `cargo check --bins` (both per-PR jobs) | all 8 targets compile | compile-only | ~build cost | also nightly | — | KEEP only if a PR leg survives; otherwise DELETE with the jobs |

Per-PR verdict: 6-of-8 bounded run → DELETE from fuzz.yml (keep `workflow_dispatch`), surviving protection = nightly 180 s/target + fuzz_smoke.rs + mutation_million.rs. Saving ≈ 48 job-s per run on the filtered closure (fuzz.yml's own figure: 19.9 job-min / 25 runs), plus the `cargo-fuzz`/nightly-toolchain install.

### Scope 5 — fixtures/** consumers

| directory | files | consumed by | status |
|---|---|---|---|
| fixtures/builtins/v1/{benchmark,meters,pcm} | 45 | scripts/check-builtins-fixtures.sh + test-builtins-fixtures.sh (ci:248), run/preflight/test-builtins-benchmark.sh, tools/audit, tools/bench | consumed |
| fixtures/builtins/v1/reference/filter-response.csv | 1 (404 K) | tools/audit/src/fixture_builtins.rs | consumed |
| fixtures/capi-qualification/v1 | 13 | **nothing** — check-/run-capi-qualification-v1.sh deleted in f0509c3f (#319); only docs/C_ABI_V1_QUALIFICATION.md cites it; check-capi-abi.sh uses crates/capi/tests/c/abi_smoke.c instead | **orphan → DELETE or move under docs/** |
| fixtures/conformance/v1 | 11 | crates/conformance/tests/fixtures.rs, examples, scripts/operator/prepare-builtins-listening.sh | consumed |
| fixtures/effect-descriptor/v1, effect-package/v1, effect-state/v1 | 10/5/9 | crates/effect-package tests, scripts/check-effect-package-v1.sh, check-effect-descriptor-v1.sh, *-reference.py, tools/bench | consumed |
| fixtures/effect-interchange/v1 | 1 | check-effect-interchange-qualification.sh, test-effect-interchange-policy.sh (ci:218), bench scripts | consumed |
| fixtures/effects/runtime-v1 (valid, invalid, MANIFEST) | 5 | check-effect-runtime-fixtures.sh / test-effect-runtime-fixtures.sh — sha-manifest integrity only; `invalid/*` are never parsed by any test | consumed (integrity only) |
| fixtures/effects/v1 (MANIFEST.tsv, golden/minimal-source.txt, invalid/not-a-package.txt) | 3 | **nothing** (last touched 68ff477e "checkpoint engine v2 foundations") | **orphan → DELETE** |
| fixtures/graph/v1 | 7 | crates/graph-compiler/src/bin/graph_fixture.rs, run by scripts/check-graph-determinism.sh (ci:351) and test-graph-benchmark.sh | consumed (whether verify-mode diff against checked-in files runs in CI: unverified) |
| fixtures/native-pcm-runner/v1 | 12 | check-native-pcm-runner.sh (ci:313), test-native-pcm-runner-v1-policy.sh, tools/native-pcm-runner | consumed |
| fixtures/rack/issue038-v1 | 2 | check-rack-benchmark-fixture.sh (ci:122), test-rack-benchmark.sh, tools/bench | consumed |
| fixtures/rack/v1 | 4 | only crates/graph-compiler/src/bin/rack_fixture.rs, which no script/workflow runs | **bin-only, no CI consumer → verify or DELETE** |
| fixtures/session/v1 (13 docs) | 13 (1.1 M) | fuzz/nightly seeds, crates/session, builtins-compiler, graph-compiler, capi, host-core, hosts/host-web tests, sdk builder-evals, console fixture scripts, tools/session-validator | consumed (every file has ≥1 consumer) |
| fixtures/session-canonical/v1 | 1 | crates/session/src/canonical.rs, sdk/test/builder-evals.mjs | consumed |
| fixtures/sources/v1 | 2 | tools/audit/src/source_fixture.rs (trace-source-audit.sh ci:242) | consumed |
| fixtures/stem-identity/v1 | 12 | tools/stem-hasher/tests/conformance.rs, tools/native-pcm-runner | consumed |
| scripts/fixtures/*.json (5) | 5 | abi-layout/parameter-metadata self-tests, benchmark validator tests | consumed |

### Summary

Verdict totals (rows): KEEP 49 · TRIM 12 · MERGE 5 · NIGHTLY 4 (boot-evals L189; stem-store-core L911-913; per-PR session fuzz ×2; per-PR protocol fuzz ×4 counted as one) · DELETE 10 (fuzz per-PR jobs; .sh kind_dir/vocabulary_dir/map_dir blocks; stem-store self-test #20; tautologies at boot-evals L322, headless-path L153, stem-pump L599; fixtures/capi-qualification, fixtures/effects/v1; fixtures/rack/v1 pending verification).

Estimated per-PR wall-clock saved (unverified where noted):
- Drop in-test `cargo run` from sdk render-evals/agent-evals: 60-180 s cold on the sdk job (unverified).
- Drop `cargo run -p host-web --example worst_boot_document` (boot budget) and 3× `cargo run -p parameter-metadata` from test-web-audioworklet.sh: 30-120 s on the browser-probes job (unverified; depends on shared build cache).
- Remove per-PR fuzz jobs: ≈48 job-s per filtered run (fuzz.yml's figure) + toolchain install.
- Run browser-independent qualification proofs once instead of ×3: ~2-6 s across the matrix.
- Stem-store ledger copy trim + #20: ~1.5 s. direct-oracle single run: ~1 s. Duplicate .sh mutations: <1 s.
- Total: roughly 2-6 min of CI job time per PR, dominated by the four hidden native cargo builds inside node/bash tests; the hermetic JS/python self-tests themselves cost <6 s combined and are not the problem.

Notable unwired items: hosts/host-web/tests/browser-v1/browser-correctness.js (no workflow); check-abi-layout-v1.py --self-test (no ci invocation seen, unverified); check-sdk-headless.sh header claims "no build step" but two evals compile Rust.