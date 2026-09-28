# Dead-code and scope audit

Date: 2026-09-28. Tree: `main` at `a9414c0c` (branch `dead-code-audit`). Read-only research: no
product code, test, script or workflow was changed, nothing was pushed, and no GitHub issue was
created or edited. The issue drafts are in `issues/` beside this file.

> **Read `VERIFY-DEAD-CODE.md` first.** An adversarial verification (2026-09-28) corrects this
> audit in place. The owner's later scope correction puts native iOS and Android playback in scope,
> which reverses the R1, R2 and R3 recommendations below and moves draft 02 to "needs a ruling".
> The failing test of section 5.2 is an intended #916 change, not a regression. Every draft now
> ends with an "Amendments" section, and a new draft `00b` repairs the operator scripts.

## 1. The verdict, in plain words

**The Rust code itself is clean. Very little of it is dead in the strict sense.** Across about
411,000 lines of Rust, the compiler proves only 129 items (about 900 lines) unused by anything. One
more block of about 8,200 lines runs only from its own tests.

**Most of the weight is elsewhere, in four places:**

1. **Finished one-off measurements.** There are benchmark runners, validators and test suites for
   issues that closed long ago. Each runner refuses to overwrite its output folder, and every folder
   already exists, so none of them can run again. CI still tests several of them on every pull
   request.
2. **Evidence files.** `artifacts/` holds 62 MB in 10,685 files: 84% of the repository's files and
   61% of its bytes. No test or CI job reads any of it.
3. **Whole features for products you have not ruled in.** These are the C ABI and native
   embedding, the binary control protocol, native WAV decoding, and third-party effect packages.
   The browser product uses none of them. Together they are about 83,000 lines of Rust, about 20%
   of the codebase, and only you can decide whether they go.
4. **Two things that are broken rather than dead.** Six of the seven operator scripts have been
   unable to run since 2026-09-01. Tests behind four `test-support` features never run in CI, and
   one of them fails on `main` today.

| bucket | Rust lines | other lines | files / bytes | ruling needed? |
|---|---:|---:|---|---|
| A. Truly dead code (section 3) | about 9,500 | a few | — | no |
| B. Used-up one-off benchmark and measurement machinery (section 4) | about 9,000 | about 10,700 | about 80 files | no |
| C. Repository weight: history that git already keeps (section 6) | 5,134 (uncompiled `dsp-research/archive/*.rs`) | — | about 76 MB, about 11,300 files; of that, 10.2 MB / 509 files are closed issue specs | only for the closed issue specs (policy) |
| D. Wider-scope features (section 7) | about 83,000 | about 8,000 | — | **yes, yours** |
| Stays | about 310,000 | — | — | — |

Removing A and B changes no rendered bit. It can change the AudioWorklet artifact's hash, but only
because deleting lines shifts the panic line numbers compiled into the module. Every issue draft
says how to prove that is the only change.

Removing D also leaves the browser untouched. Most D items are not compiled into the shipped
module at all (`capi`, `protocol`, the native decode workers, which are `cfg(not(wasm32))`).
A few are compiled in but never execute there:

- the whole-plan scalar path;
- the extended-rate descriptor predicate;
- `effect-package`, which a scratch build showed adds no code to the module.

D is a product decision, not a correctness one.

## 2. How this was measured

Everything below was produced by commands run on `a9414c0c`; scratch copies and build directories
were deleted afterwards.

- **Dependency graph.** `cargo metadata` and
  `cargo tree -p host-web --target wasm32-unknown-unknown -e normal` give the shipped module's
  crate closure. It is 25 workspace crates. `protocol` and `capi` are not among them. `source` and
  `effect-package` are, through `host-core` and `effect-compiler`.
- **Cross-references.** `rust-analyzer scip` (all features) produced an index of 27,415 workspace
  definitions and 799,437 occurrences. Each reference was classified as product, test, tool or
  other host.
- **Compiler proof of dead items.** Every zero-reference candidate was demoted from `pub` to
  private in a scratch copy, and rustc's `dead_code` lint judged it. Two builds were run, both with
  all features: `cargo check --workspace --lib --bins --examples` (non-test) and
  `cargo check --workspace --tests` (test), plus a `wasm32` simd128 check of the browser closure.
  An item counts as dead only if rustc reported it unused in **both** builds and no other crate
  failed to compile.
- **Unused dependencies.** `cargo machete` found one (`rack-compiler` → `engine`), and a build
  without it proved it. `cargo udeps` found only `engine`'s `loom` dev-dependency, a false positive:
  it is used under `cfg(loom)` in CI.
- **Scripts.** A reachability graph was built from workflow `run:` lines through every script that
  names another (comment lines ignored). The first pass matched file names only, so it missed jq's
  extension-less `include "name"`. The verifier caught this, and the graph was re-run with jq
  includes counted.
- **Tests never run.** `cargo tree -e features` with CI's exact `test-debug-a` feature list showed
  which `test-support` features CI turns on. Then
  `cargo test -p host-web -p host-core -p effect-compiler -p parametric-eq --features <their test-support>`
  was run once.
- **Artifact.** The module was built with `scripts/build-web-audioworklet.sh --module-only` and
  compared with the pin. See section 9 for a caveat.
- **Per-area facts.** Six parallel research passes covered native embedding, the protocol, sources
  and sample rates, effect packages, remnants of excluded modes, and repository weight. Each pass
  proved removals by compiling a scratch copy. Their numbers are quoted with the command behind them.

## 3. Truly dead code (no ruling needed)

| # | what | size | evidence | removal risk | draft |
|---|---|---|---|---|---|
| A1 | 129 items the compiler proves unused. Draft 01 deletes 104 of them in 18 crates: builtins (8), dsp-reference (23), engine (23), effect-contract (7), host-core (6), host-web (4 constants), rack (5), source (5), and others. The other 25 are in protocol, the endpoint files and host-mobile, and go with drafts 02, R1 and R3 | about 910 lines (about 720 in draft 01) | demotion + two-build rustc proof (section 2) | very low; artifact hash may move by panic line numbers only | 01 |
| A2 | `rack-compiler` depends on `engine` but never names it | 1 line | `cargo machete`; builds without it | none | 01 |
| A3 | 47 `#[allow(dead_code)]` attributes that suppress nothing (17 in `host-web/src/observation_ingress.rs`, 16 of the 17 in `host-web/src/lib.rs`, others) | 47 lines | stripping every non-test one (59 by the research pass's count, which excludes test modules and `tests/`) left 12 warnings, so these 47 hide nothing. The 12 that remain are layout mirrors or test-only members | very low | 01 |
| A4 | `host-core` `builtin_batch_endpoint.rs` and `scalar_point_endpoint.rs` with their tests: native control endpoints behind `control-provider`, called by nothing, not even the C ABI | **8,244 lines** (3,589 in the two source files, including their unit tests; 4,655 in their integration tests) | rg and SCIP: no caller outside the two files and their tests; the full workspace builds without them | low. They were steps toward open #140 (protocol automation delivery), so tell #140 | 02 |
| A5 | Leftovers of the removed multicore renderer: the auxiliary-worker hand-over (`take_handover`, `accept_handover`, `ExecutorHandover`, `copy_worker_audit_snapshots`, `dispatch_counters`) in `engine/src/realtime/plan.rs` and `plan_exchange.rs` | about 190 lines | only engine's own test executor implements hand-over; `graph` does not; nothing calls the wrappers | low | 03 |
| A6 | The unused external render input: `RenderIo::input`, `RenderEnvelope::input_channels`, `RenderError::InputShape`, and the `input` argument of `PreparedPlanExecutor::render` | about 130 call sites (95 `input: None`, 38 `input_channels: None`), mostly tests | the only executor (`graph/src/lib.rs:2678`) ignores `_input`; every host passes `input: None`; the C ABI render entry has no input planes | low; compiled into the artifact (one shape check per render), so the pin moves | 03 |
| A7 | `fixtures/capi-qualification/v1`: a stale ledger naming two scripts that no longer exist | 13 files, 30 KB | nothing reads it | none | 06 |
| A8 | `dsp-research/archive/issue-0{31,42,44,45}/*.rs`: research sources that no crate compiles | 5,134 lines | not in any workspace or fuzz manifest | none (git keeps them) | 06 |
| A9 | Two "thin wrapper" scripts kept "so any caller keeps working", with no callers: `check-parametric-eq-targets.sh` and `check-builtins-targets.sh` (the latter is mentioned only in comments and a scratch stub inside a test going in B3) | 2 files | reachability graph | none | 04b, 04c |

**Also found (hygiene, not removal):** 86 `test_only_*` functions and several counters
(`bank_route_folds`, `bank_scatter_redirects`, `force_mono_collapse_off`) exist only for tests and
benchmarks. They are "moves no bit" oracles that guard live claims, so they stay. Draft 01 only
converts the handful of `allow(dead_code)` members used solely by tests to `#[cfg(test)]`.

## 4. Used-up one-off benchmark and measurement machinery (no ruling needed)

**The pattern.** AGENTS.md makes each benchmark "run exactly one invocation" and refuse to
overwrite its record. So each issue grew its own runner, preflight, validator, fixture and mutation
test. Once the record existed, the runner could never run again, but CI kept testing it.

`scripts/operator/README.md` states: "Every script under `scripts/` is reachable from a GitHub
workflow." That is no longer true: 18 files under `scripts/`, outside `operator/`, are reached by
no workflow (jq `include`s counted). Seventeen are the dead one-shots below. The eighteenth is the
live `run-console-benchmark.sh`, which a person runs; by the README's own rule it belongs in
`scripts/operator/`.

| # | what | size | why it is dead | CI it costs now | draft |
|---|---|---|---|---|---|
| B1 | 48 historical arms of `scripts/run-console-benchmark.sh` (and their mirror in `operator/preflight-console-benchmark.sh`); only `--step NAME` can still run | about 320 lines, 271 of them header history | every arm's `artifacts/<arm>` exists, and the runner refuses to overwrite (checked all 48) | none directly; it is maintenance weight (for example #956 re-indexed arms that cannot run) | 04a |
| B2 | Rack #038 benchmark: runner, preflight, validators, `check-rack-benchmark-fixture.sh`, `fixtures/rack/issue038-v1`, `bench` `rack` subject | 1,905 lines (947 Rust) | `artifacts/issue038` exists | lint fixture check + `test-rack-benchmark.sh` on every PR | 04b |
| B3 | Builtins #072 and #431 benchmarks: two runners, preflights, test suites, four jq validators, `bench` `builtins` subject; plus the "manifest consumers" check that pins one fixture hash in 7 places | 4,935 lines (2,095 Rust) | #431's folder exists; #072 is a "sole exactly-once" run whose own script needs spec 068's text | lint `test-builtins-current-benchmark.sh` + manifest-consumer step; nightly `test-builtins-benchmark.sh` | 04b |
| B4 | Graph-compiler #006 benchmark: runner, promote script, tests, validators, fixture record, `bench` `graph` subject | 1,350 lines (847 Rust) | "sole future Issue-006 entry point"; it writes only to `target/issue6`, no record is committed, and it measures compile time, not a host render path | nightly `test-graph-benchmark.sh` | 04b |
| B5 | Issue #880 MQ-1/MQ-2 micro-benchmarks: runners, library, validators, fixtures, and the two ignored crate bench tests `transient-shaper/tests/bench.rs` and `compressor/tests/bench_ramp.rs` | 2,170 lines (761 Rust) | no workflow reaches them; #880 closed | test compile time only | 04c |
| B6 | #746 and #748 gate/multiband "active" benchmark: `run-gate-active-benchmark.py`, its test, `bench` `gate_active` and `multiband_active` | 3,225 lines (1,831 Rust) | no workflow reaches them | bench compile and tests in audit-native | 04c |
| B7 | #650 allocation-record validator (`check-prepared-effect-allocation-records.py`) and the `audit prepared-effect-allocations` subject | 890 lines (645 Rust) | the validator is reached by nothing; the subject is one-shot | audit unit tests | 04c |
| B8 | #600-#606 "input symmetry" capture: three scripts, validator, `bench` `input_symmetry` and `input_symmetry_capture`, `artifacts/issue-60{2,3,6}-*`. Despite the name this is the input-trim capture, **not** dual-mono content detection | 1,720 lines (823 Rust) | no workflow reaches them | bench compile | 04c |
| B9 | #163 phase-0 wasm kernel timing: `operator/run-wasm-kernel-timing.sh`, validator, test; `wasm-gates` `--native-timing`/`--wasm-timing` modes | 373 script lines + timing modes | its folder exists; the operator script cannot run (section 5) | lint `test-wasm-kernel-timing.sh` | 04c |
| B10 | Unreached or broken misc: `operator/probe-opfs-move-v1.cjs`, `operator/seal-web-audioworklet-browser-correctness.sh` (superseded by the CI browser job). (`protocol-benchmark-record-validator.jq` looked unreferenced but is pulled in by a jq `include` and runs in CI; it stays until R3) | about 290 lines | reachability graph; section 5 | none | 04c |
| B11 | #081/#108 effect-interchange **benchmark**: 7 benchmark scripts and validators, and the `bench` `effect_interchange` arm | 2,819 lines (1,092 Rust) | issues closed; benchmarks no host path. The interchange *qualification* scripts (1,107 lines) go with ruling R6 | part of lint's 20 s | 05 |

B1 to B11 total about 9,000 Rust lines and about 10,700 script, fixture and validator lines.

**Deliberately not in this list** (they need your call, section 7):

- the wasmtime console benchmark (`tools/wasm-console`, `tools/wasm-console-guest`, its operator
  runner);
- the nightly descriptive native benchmarks.

**Kept:** the live measurement paths are `run-console-benchmark.sh --step NAME` (native) and
`run-web-mixing-automation-benchmark.sh ... --step NAME` (V8, the shipped module).

## 5. Broken rather than dead

1. **Six of the seven operator shell scripts cannot run, and have not been able to since
   2026-09-01.**
   - They compute the repository root as `$(dirname "$0")/..`, which is `scripts/`, not the repo.
     So they look for `scripts/scripts/check-bench-preconditions.sh` and `scripts/artifacts/...`.
   - This dates from when they moved into `scripts/operator/` (commit `f0509c3f`, #319). Only
     `preflight-console-benchmark.sh` was later fixed to `../..`.
   - Affected: `run-wasm-console-benchmark.sh`, `preflight-wasm-console-benchmark.sh`,
     `run-wasm-kernel-timing.sh`, `preflight-rack-benchmark.sh`,
     `prepare-builtins-listening.sh`, `seal-web-audioworklet-browser-correctness.sh`.
   - The last wasm console record is from 2026-08-28. The runner was still being edited on
     2026-09-27 (#956).
   - Drafts 04 and R9 either delete or repair each one.
2. **Tests behind `test-support` features never run in CI, and one fails on `main`.**
   - CI's workspace test run turns on `test-support` only for `builtins-compiler`, `source` and
     `graph`. It never turns it on for `host-web`, `host-core`, `effect-compiler` or
     `parametric-eq` (`cargo tree -e features` with the `test-debug-a` flags).
   - Clippy compiles those tests under `--all-features`, but nothing runs them.
   - Running them once: 588 pass and **1 fails**.
     `host-web tests::acknowledged_pair_render_records_the_same_live_dispatch` (`tests.rs:3220`)
     expects `process_calls == 1` and gets 2.
   - Other live-claim tests in this never-run set:
     - `prepared_eq_owner_transaction_is_design_and_allocation_free_after_preparation`;
     - `host-core/tests/observation_demand.rs`'s `dormant_controlled_spectrum_does_no_capture_work_on_render`;
     - the parametric-eq "performance half" assertions.
   - **This must be fixed first** (draft 00). Otherwise no cleanup can honestly claim that "no
     test guarding a live claim is lost": some of those guards are already lost.

## 6. Repository weight

Deleting files does not shrink a clone: git keeps history, and the pack is 82 MiB. AGENTS.md
forbids rewriting others' history. The gain is a smaller working tree, less grep noise and less to
review. Any deletion under `artifacts/` routes CI to the **full** path, because
`scripts/ci-path-router.py:17` treats only `docs/` and `.github/ISSUE_SPECS/` as documentation. So
batch these deletions.

| what | size | read at test/CI time? | before deleting | ruling? | draft |
|---|---|---|---|---|---|
| `artifacts/`, 275 folders and one loose file | 62.4 MB, 10,685 files (84% of tracked files) | **nothing reads any of it.** 39 folders are *named* by the used-up runner arms (B1-B3, B8, B9); five doc comments mention three folders | re-point 28 links in 12 rulings and 23 links in 9 live docs, or keep the 17 ruling-cited folders (2.93 MB) | no (history) | 07 |
| `artifacts/steps/` | 3.98 MB, 68 files | written by the live `--step` arms; old records are history | keep the folder; decide how long step records live | light | 07 |
| `.github/ISSUE_SPECS/`, closed | 509 specs, 10.19 MB | spec 068 is read by the B3 runner and its nightly test; 5 allowlist lines | amend the AGENTS.md rule that compares this folder with `gh issue list --state all` | **yes (policy)** | R10 |
| `.github/ISSUE_SPECS/BRIEFS/` | 77 files, 622 KB | one allowlist line | none | light | 08 |
| `docs/handoffs/` (other than the unfiled `silence-2026-09-27` drafts and the builtins-less `SCOPE.md` cited by #956-#964) | about 2.7 MB of the folder's 3.1 MB, mostly `.patch` and raw timing files | 3 `MUTATIONS.md` links | re-point 3 links | no | 08 |
| `docs/issue880-*.md` (12 notes, 61 KB); `docs/audits/5xx-8xx-*.md` (44 per-attempt reviews and evidence notes, 109 KB, many for the endpoints of A4 and the capture of B8); `docs/research/legacy-v2old/` (13 legacy-engine research copies, 32 KB) | about 0.2 MB | 19 of the 23 live-doc links into `artifacts/` are in the issue880 notes | none | no | 08 |
| `dsp-research/archive/*.rs` | 5,134 lines | no | none | no | 06 |
| `fixtures/` | 2.17 MB, 160 files | all are read, except `capi-qualification/v1` (A7) and `rack/issue038-v1` (B2) | — | the native-WAV, C-ABI, extended-rate and effect-package sets follow rulings R2, R4, R5 and R6 | — |

**If `artifacts/` goes, measurement records need a stated new home.** The refuse-to-overwrite
convention and `scripts/operator/README.md` assume records live there. Options are CI uploads, a
summary table under `docs/`, or keeping only `artifacts/steps/`. Draft 07 proposes keeping
`artifacts/steps/` and deleting the rest.

## 7. Needs your ruling

AGENTS.md still names these areas. None of them is in the browser product. Each ruling below is
independent, except that R3 (protocol) needs R2 (C ABI) first.

### Size and cost at a glance

| ruling | area | Rust lines | CI cost today | browser depends on it? | recommendation |
|---|---|---:|---|---|---|
| R1 | native host shells (`host-native`, `host-mobile`), `target-smoke`, AArch64 `cfg` arms | about 150 + about 45 cfg sites | seconds | no | **remove** |
| R2 | C ABI (`capi`) + `native-pcm-runner` + `control-provider` + engine plan-replacement API | about 13,800 | about 1 job-minute per full run + compile share | no | **remove unless native embedding is on the roadmap** |
| R3 | binary control protocol (`protocol`), conformance protocol corpus, protocol fuzz/bench/audit; sidecar/WebSocket (design text only) | about 33,000 | about 1 minute per full run; 1.6 minutes per qualifying `fuzz.yml` run; about 12 minutes nightly | no | **remove together with R2** |
| R4 | native WAV/RF64 decode workers (`source/src/native_source.rs` and native hooks), `audit source`/`source-duration` | about 6,100 (+3,700 tied to R2: `native_wave.rs`, `audit fixture-source`, `stem-hasher`) | under 1 minute | no (all `cfg(not(wasm32))`) | **remove** |
| R5 | extended sample rates 176.4-384 kHz | about 50 | none | no (sessions already refuse them) | **remove** |
| R6 | third-party effect packages (CID, package, C header) and persisted effect-state migration (`effect-package`, `effect-compiler` state/migration, interchange qualification) | about 21,300 + about 3,400 script lines | about 1 minute per full run; 6 minutes nightly | no (a scratch build without it has the same functions, code bytes and data size; only symbol order moves) | **remove; reopen with #27/#28 when the product needs it** |
| R7 | the eight-lane wasm measurement build (`miso_wasm_simd8`) and the `--issue183` arms | about 200 | lint share | no | **remove** (it conflicts with the #183 ruling, so it is yours) |
| R8 | the whole-plan `Backend::Scalar` path (per-node builtin strips, scalar split pairs) + the scalar wasm CI build | about 1,500 production + 2,000-3,000 test | 1.2 minutes per full run | no (never runs at Simd4) | **remove, but replace its role as the "banking moves no bit" oracle first** |
| R9 | benchmark policy: the wasmtime console benchmark (broken since 2026-09-01) and the nightly descriptive native benchmarks | about 1,100 + scripts | nightly | no | **delete the wasmtime console; keep only the console `--step` and V8 rows** |
| R10 | closed issue specs kept locally | 509 files, 10.2 MB | none | no | **keep only open specs locally** |

### R1. Native host shells, `target-smoke`, AArch64 arms

- **What.** `hosts/host-native` (30 lines) and `hosts/host-mobile` (26) are stubs: they attest the
  CPU and print `target-smoke` values. Their audio callbacks are "deferred to issue 023".
  `host_mobile::mobile_target_smoke` has zero references. `crates/target-smoke` is 87 lines.
- **AArch64.** About 45 `cfg(target_arch = "aarch64")` arms exist:
  - lane `fpenv.rs` (27), `backend.rs`;
  - `soft-clip`, `graph/runtime.rs`, `audit/vectorization.rs`, two tests.
  - Nothing builds or tests AArch64. The toolchain has only the `wasm32` target, and no CI job
    covers it.
  - `docs/TARGET_MATRIX.md` already records your 2026-09-04 ruling (#378): native AArch64 is
    "unsupported, no claim".
- **CI.** The "Native host smoke" step (0 s). `target-smoke` appears only as a compile subject in
  the lint probes, the scalar wasm build and the SIMD128 probe; `-p engine -p lane` can replace it.
- **What removal breaks.** Nothing product-side. Edit the policy lists in
  `check-artifact-evidence-leak.sh` and `check-conformance-boundaries.sh`, and the docs.
- **Recommendation: remove** (draft R1). It is target-specific code for targets you have ruled
  unsupported.

### R2. The C ABI and native PCM runner

- **What.**
  - `crates/capi`: 9,802 lines, plus a 255-line header and C/C++ smoke tests.
  - `tools/native-pcm-runner`: 2,689 lines, plus `fixtures/native-pcm-runner/v1` (5 WAVs, 127 KB).
  - `audit capi`: 342 lines.
  - `host-core` `control_provider.rs`: 874 lines behind `control-provider`.
  - The engine plan-replacement API (`reserve_replacement`, `epoch`, `commit`,
    `render_contiguous`): about 86 lines, used only by capi.
  - Scripts: `check-capi-abi.sh` and three native-pcm-runner policy scripts, about 700 lines.
  - Docs: `C_ABI_V1_QUALIFICATION.md` and `NATIVE_PCM_REFERENCE_RUNNER_V1.md`.
- **Browser dependency: none.**
  - The shipped module exports `miso_engine_web_v1_*`, not the 14 frozen `miso_engine_v1_*`
    symbols.
  - `check-host-core-policy.sh` *forbids* host-web from enabling `control-provider`.
  - `sdk/assets/miso-engine-v1-abi-layout.json` describes the **wasm** ABI, not the C ABI, so it
    stays.
  - `native-pcm-runner` is the oracle for no browser gate. The browser oracles are
    `direct-oracle.mjs` and host-web's own native examples.
- **CI.**
  - "C ABI linkage…" is 45 s; the capi runtime audit is 1 s.
  - capi is one of four packages in the 99 s release build step.
  - The native-pcm-runner lint step is 9 s.
  - `check-release-shape.py` pins capi in the cdylib set.
- **What removal loses.**
  - The only public-entry native render audit (100,000 calls with zero allocations or syscalls).
    The browser has its own: the wasm call-graph and allocation-closure checks in
    `check-web-audioworklet.sh`.
  - The only user of transactional plan replacement at a block boundary. AGENTS.md describes that
    architecture, but today only capi exercises it.
  - Open #895 (native runner I/O) closes as descoped.
- **Recommendation: remove unless a native or cloud embedder is on the roadmap** (draft R2).

### R3. The binary control protocol (and sidecar/WebSocket)

- **What.**
  - `crates/protocol`: 29,957 lines (18,049 production, 11,094 in-source tests), 175 tests.
  - About 950 lines of `conformance` (protocol corpus and wasm golden runner).
  - `bench protocol` (1,667 lines, and the only reason for the `flatbuffers` dependency).
  - `audit protocol` (512 lines).
  - Four fuzz targets, 9 scripts (668 lines) and six `docs/CONTROL_*.md` files.
- **Browser dependency: none.**
  - host-web's command ABI (`miso.command.v1`) is its own, with its own vocabularies.
  - The "acked-batch" lesson in AGENTS.md (#139/#140 admission) is implemented in host-web
    (`admit_commands`, `hosts/host-web/src/lib.rs:6246`), not in `protocol`.
- **Coupling.** capi carries protocol frames (179 `protocol::` uses), so R3 requires R2.
- **Sidecar and WebSocket:** no code exists, only AGENTS.md text and
  `docs/CONTROL_PROVIDER_BOUNDARY.md`. Open #25 ("Optional binary WebSocket sidecar") would close
  as descoped.
- **CI.**
  - Golden parity 19 s, the protocol share of "Conformance boundaries" (38 s total), the probe 1 s.
  - The `fuzz.yml` protocol job: 1.6 minutes on every push touching seven crates.
  - About 12 of the roughly 25 nightly deep-fuzz minutes.
- **Compile proof.** With protocol, capi, native-pcm-runner and the protocol parts of
  conformance, bench and audit deleted:
  - the whole workspace builds (all targets, all features);
  - the wasm32 checks pass.
- **Recommendation: remove with R2** (draft R3). If you keep native embedding, keep the protocol
  too, but still do drafts 01 and 02.

### R4. Native WAV/RF64 decode workers

- **What.**
  - `crates/source/src/native_source.rs`: 5,015 lines, of which 3,225 are tests. These are the
    decode worker threads and session-level native prepare.
  - About 192 lines of native-only ring hooks.
  - `audit source` and `audit source-duration`: 855 lines.
  - `scripts/trace-source-audit.sh`.
  - The "Issue-544 source-duration" CI step and its inline validator.
- **Browser and C ABI dependency: none.**
  - All of it is `cfg(not(target_arch = "wasm32"))`.
  - `prepare_native_session_sources` has no caller outside `source`.
  - Even the C ABI takes host-decoded planar chunks, like the browser.
  - The browser uses only the shared ring (`PcmSourceRing`, `HostChunkProvider`,
    `prepare_graph_source_set`), which stays.
- **Tied to R2.** The WAV parser `native_wave.rs` (1,597 lines), `audit fixture-source`
  (1,181 lines, which reads `fixtures/sources/v1`) and `tools/stem-hasher` (883 lines) are used
  only by native-pcm-runner, the audits and stem-hasher.
  - No test couples stem-hasher to the browser stem store: `stem-store-hash-v1.mjs` uses its own
    vectors.
- **What removal changes.**
  - It contradicts the AGENTS.md sentence "native WAV/RF64 decode workers fill bounded SPSC PCM
    rings". Amend it.
  - Open #124 (decode pool) closes as descoped.
- **Recommendation: remove** (draft R4). Delete the parser, `fixture-source` and stem-hasher with
  R2, or move the parser into a tool crate if R2 keeps the runner.

### R5. Extended sample rates (176.4-384 kHz)

- **Who accepts them: no one that renders.**
  - `session::validate` refuses every non-launch rate
    (`crates/session/src/validate.rs:36-43`), so the browser and the C ABI already refuse them.
  - They survive only as descriptor metadata:
    - `EXTENDED_COMPATIBILITY_SAMPLE_RATES` and its predicate (`crates/engine/src/lib.rs:55-84`);
    - `effect-contract::validate_descriptor` (`:761`);
    - the effect-package descriptor decoder.
  - 4 of the 11 `fixtures/conformance/v1` files use them.
- **What removal costs.** About 50 lines, the two predicates tightened to launch-only, some
  descriptor tests flipped from accept to reject, and the conformance fixtures updated. No console
  digest moves.
- **Recommendation: remove** (draft R5). It is small and self-contained.

### R6. Third-party effects, effect packages and effect-state migration

- **What.**
  - `crates/effect-package`: 13,389 lines, plus a 392-line C header and a 917-line C smoke test.
  - `effect-compiler`'s `migration.rs` (1,197 lines), the `prepare.rs` state span (about 750
    lines) and six test files (about 6,000 lines).
  - About 3,400 script lines, and 25 fixtures:
    - about 2,300 for the package, descriptor and state checks;
    - 1,107 for the interchange qualification's reference processes.
  - Two fuzz targets and five `docs/EFFECT_*_V1.md` files.
- **Browser dependency: none.**
  - `effect-package` is in the closure only through `effect-compiler`'s state and migration paths,
    which only tests and the #081 benchmark call.
  - A scratch build with the crate removed produced a module with identical code and data sizes.
    The only differences were function order and one moved 2-byte string, so no rendered code
    changes and console digests cannot move.
  - The session grammar accepts `{"kind":"cid"}` effects. `effect-compiler` refuses them at
    compile time with `effect.third_party.unavailable_at_launch`.
- **CI.**
  - Lint: 20 s + 5 s.
  - Cross-target: 16 s + 5 s + a share of 37 s.
  - About 105 tests.
  - Nightly: 6 minutes of fuzzing, plus a release-budget test.
- **Recommendation: remove, in two rulings** (draft R6):
  - "third-party effects are out of scope until reopened", closing open #27 and #28;
  - "no persisted DSP state or migration without a product need".

  Keep the native effects, the dynamic rack, `EffectBankPreparation`, the descriptors and the
  parameter metadata. Removing the session `cid` identity is optional: it moves the refusal from
  compile time to parse time, a grammar change you would own.

### R7. The eight-lane wasm measurement build

- **What.** The `miso_wasm_simd8` cfg:
  - `lane/src/backend.rs:42-57`, `target-smoke`, and `Cargo.toml`'s `check-cfg`;
  - the wasm-console W8 leg and its validator branch, which CI lint tests
    (`test-wasm-console-benchmark.sh:214-265`);
  - the `--issue183` arms, which cannot run (their folder exists).
- **Related.** The wasm gate corpus still digests at Simd8 on wasm (`WIDTHS = 3`).
- **The conflict.**
  - Your #183 ruling (2026-08-27) kept this "for re-measurement".
  - Your 2026-09-28 answer ("no target-specific code") and the rule "modes production never needs
    are removed entirely" point the other way.
- **Recommendation: remove** (draft R7). A future re-measurement can re-add a cfg in one change.

### R8. The whole-plan `Backend::Scalar` path

- **What.** When a plan is compiled at `Backend::Scalar`, builtins become per-node strips and
  pairs become scalar split pairs.
  - About 620 lines in `builtins-compiler` `lib.rs:4086-4707`.
  - `strip_bindings` and the scalar-owner estimate.
  - Graph's scalar pair factories, split-pair layout mirrors and selection passes.
  - About 1,500 production lines in total, plus 2,000-3,000 test lines.
- **No shipped target produces Scalar.** x86 gives Simd8; wasm simd128 and AArch64 give Simd4.
  host-core's backend parameter is a `#[cfg(test)]` seam.
  - This is exactly the "back door" the #959 spec flagged for you.
- **Scalar wasm in CI.**
  - The wasm-guests job builds 18 packages with `-simd128` (0.6 min).
  - `check-wasm-realtime-atomics.sh` (0.6 min) inspects that build. It duplicates
    `check-web-audioworklet.sh:69`, which already checks the shipped module for atomics.
  - The W4-D1 note in `build-web-audioworklet.sh` kept "the scalar cargo check" on purpose, so
    this is yours to reverse.
- **The catch.** Scalar per-node rendering is the oracle for "banking never moves a bit". The
  largest user is `crates/graph-compiler/tests/bank_levels.rs`: ten tests render every session at
  Scalar, Simd4 and Simd8 and compare bits. graph-compiler's `src/lib.rs` and host-core's
  `limiter_linked_session.rs` do the same.
- **Recommendation: remove, but first replace the oracle** (draft R8). Compare Simd4 with Simd8,
  or the lane scalar kernel with `dsp-reference`.
- **Keep:** lane's one-lane `Scalar`/`FrameLane`. Every width uses it for scalar tails, and it is
  live production code.

### R9. Which benchmarks stay

- **The wasmtime console benchmark** (`tools/wasm-console`, `tools/wasm-console-guest`,
  `operator/run-wasm-console-benchmark.sh` and its preflight, validator and test: about 2,400
  lines).
  - It has not been able to run since 2026-09-01.
  - All 30 of its arms point at existing folders.
  - It measures Cranelift, not V8.
  - The real host path is measured by `run-web-mixing-automation-benchmark.sh` (V8, the shipped
    module) and by the ad-hoc V8 harnesses in recent handoffs.
- **The nightly descriptive benchmarks** (conformance, realtime, session, effect-contract,
  FP-environment) measure native kernels, not a host path, and gate nothing.
- **Also:** every benchmark links `bench-support`, which turns on `engine/realtime-audit` and
  installs an audited global allocator. So the console numbers are not taken on exactly the
  shipped build.
- **Recommendation.**
  - Delete the wasmtime console, and its W8 leg with R7.
  - Keep the two live `--step` paths: native console and V8 mixing automation.
  - Either promote a V8 multi-row console harness from the handoffs, or accept native-only rows
    for effects without a V8 row.
  - Drop or keep the nightly descriptive benchmarks as you prefer; they cost about 30 nightly
    minutes.

### R10. Closed issue specs

- **What.** 560 numbered specs (10.55 MB), of which 509 are closed on GitHub (10.19 MB).
  - Every spec has a GitHub issue.
  - 115 GitHub issues have no local spec.
- **The policy.** AGENTS.md says "Work only from a stateless issue body in `.github/ISSUE_SPECS/`"
  and "compare `.github/ISSUE_SPECS/` with `gh issue list --state all` at every issue boundary".
  Deleting closed specs would make that comparison report 509 missing entries.
- **Recommendation: keep only open specs locally** (draft R10). Amend the AGENTS.md rule to: "a
  closed issue's spec lives on GitHub and in git history". Remove the #072 benchmark (B3) first,
  because it reads spec 068.

### Also yours (small)

- **The `dependency_waves` session token.** `render_profile.mode: "dependency_waves"`
  (`docs/session-v1.schema.json:36`, `session::RenderMode::DependencyWaves`) is still parseable
  and always refused. Removing it is a Session V1 schema change, so it is your call. The SDK emits
  only `single_thread`.
- **Open issues the rulings would close:** #25 (WebSocket sidecar), #27 and #28 (third-party
  Wasm), #124 (native decode pool), #140 (protocol automation delivery), #895 (native runner I/O),
  and #972 (dual-mono stem report, which is content detection and already deferred by your
  dual-mono ruling).
  - #210 lists "monitor mixes". Confirm whether that is compatible with the stems-for-mixing
    ruling.

## 8. What the browser would lose if native builds went

**Native as a product** (R1-R4) and **native as a build target for tooling** are different
things. Removing the native products leaves the native build in place, and the browser loses
nothing. Removing the native *build* would cost the browser most of its safety net:

- **Tests.**
  - All roughly 1,550 Rust tests run natively on x86-64-v3.
  - The browser's own crates are tested natively at Simd8, plus forced Simd4 plans that mirror the
    shipped four-lane geometry.
- **Browser oracles.**
  - `check-browser-expected-resources.py` runs host-web's native example
    `browser_fixture_resources`.
  - `sdk/test/render-evals.mjs` runs `sdk_render_oracle`.
  - `check-web-boot-budget.mjs` runs `worst_boot_document`.
  - `run-wasm-gates.sh` compares native and wasm digests (G5).
- **Realtime proofs.** The allocation counters and strace syscall traces of `tools/audit` are
  native.
- **Performance.** The console benchmark (the scoreboard in every recent handoff) and the
  effect-floor accounting (`isolated_cycles_per_lane_sample`) are native.
- **Fuzzing and concurrency.** libFuzzer and loom are native-only.

**Recommendation: keep the native x86-64-v3 build and its Simd8 width as tooling.** It is an
architectural lane width, not target-specific code. Rule only on the native *products*.

## 9. Caveats found along the way

- **The artifact hash depends on whether `rust-src` is installed.**
  - `build-web-audioworklet.sh` remaps `CARGO_HOME` and the repo root, but not the rustup
    `rust-src` path. rustc embeds that path in std panic locations when the component is present.
  - While this audit ran, rust-analyzer installed `rust-src` into the 1.97.1 toolchain. The local
    build then gave `60cb75e1…` instead of the pinned `476e58ad…`.
  - After `rustup component remove rust-src` restored the toolchain, it matched the pin again.
  - So "artifact unchanged" gates must compare base and change built on one machine, or rely on
    the CI `artifact` job.
  - A one-line remap of `$(rustc --print sysroot)/lib/rustlib/src/rust` would fix the script. That
    is a separate issue, not drafted here.
- **Line shifts move the pin.** Deleting lines in any file compiled into the module shifts panic
  line numbers, so the pin can move even when no code changes. Every draft asks for a
  function-by-function `wasm-objdump -d` comparison to prove that (section 11).
- **Line counts are approximate.** The per-crate production/test splits come from a script that
  counts one extra line per file (a trailing newline), so they run about 0.1% high. Per-crate
  totals quoted in section 7 and the R drafts were re-measured with `wc -l`. No conclusion depends
  on the difference.
- **Cross-reference lists need a compile proof.** rust-analyzer can miss references made inside
  macro expansions. So every "used only by X" list here is a lead, and each draft re-proves it by
  compile. The dead items in section 3 were already proved by rustc.
- **Earlier ledger.** The test-usefulness ledger of 2026-09-04 (`docs/audits/test-usefulness-2026-09-04/`)
  is a per-test proposal that was never applied in full. It is complementary: it judges tests, and
  this audit judges code and scope.

## 10. Proposed order of work

1. **00: run the never-run `test-support` tests in CI and fix the failing one.** This is the
   prerequisite for every "no guard lost" claim.
2. **01: delete the proven-dead items.** Remove the no-op `allow(dead_code)` attributes and the
   unused dependency.
3. **02: delete the unused control-provider endpoints** (tell #140).
4. **03: delete the multicore hand-over and the unused render input.**
5. **04a, 04b, 04c and 05: retire the used-up benchmark machinery.** This restores the rule that
   every script under `scripts/` is CI-reachable. The operator scripts are either deleted here or
   left for R9.
6. **06: remove the dead fixtures and uncompiled research sources,** and rewrite the stale
   `TARGET_MATRIX.md` table.
7. **07 and 08: prune `artifacts/`, handoffs, `BRIEFS/` and per-attempt review notes.** Run them as
   one CI-conscious batch, since `artifacts/` deletions route to the full CI path.
8. **Your rulings, then their drafts:**
   - R2 then R3 (C ABI, then protocol);
   - R1, R4, R6, R5, R7, R8 (after its oracle replacement), R9 and R10.

   Amend AGENTS.md's mission, scope and "Interfaces and transports" text in the same change as the
   first ruling that contradicts it.

Steps 1-7 need no ruling. They cover about 18,500 Rust lines, about 11,000 script lines, and
about 60 MB of history. R10 would add another 10 MB.

Step 8 is your call. It covers about 83,000 Rust lines, and the product is unaffected either way.

## 11. Gate conventions every draft uses

- **Shipped artifact.** Run `bash scripts/build-web-audioworklet.sh --module-only EMPTY_DIR` on
  the base commit and on the change, on the same machine and toolchain. The script refuses a
  directory that is not empty (exit 2).
  - Compare the two modules. They must be byte-identical, or `wasm-objdump -d` (wabt) must show
    every changed function and explain why it changed.
  - Do not compare a local build against the committed pin: the pin depends on the toolchain's
    installed components (section 9). The CI `artifact` job is the authority for the pin.
- **Wasm gates.** `bash scripts/run-wasm-gates.sh` needs `wasm-objdump` (wabt) and the pinned
  Node version, unless it is run with `--without-v8-spill` as the wasm-guests CI job does.
- **Console digests.** Run
  `cargo test --locked --release -p console-workload --test gain_pan_profile -- --ignored --exact digests --nocapture`
  on base and change. The two outputs must be byte-identical.
- **Tests (no live claim lost).** Compare `cargo test … -- --list` on base and change, with CI's
  feature sets plus the `test-support` features draft 00 adds. Every test missing on the change
  side is listed in the evidence, either with the surviving test that holds its claim or with the
  reason its claim was removed.
- **Artifact unchanged by construction.** When a draft says no crate in the module's closure
  changes, show `git diff --stat -- crates hosts`. Alternatively, show that every changed crate is
  outside `cargo tree -p host-web --target wasm32-unknown-unknown -e normal`.

## Issue drafts

The drafts are in `issues/`. The ones with an `R` prefix wait for your ruling.

- `00-run-the-test-support-tests-ci-never-runs.md`
- `01-delete-the-items-the-compiler-proves-unused.md`
- `02-delete-the-unused-control-provider-endpoints.md`
- `03-delete-the-multicore-hand-over-and-the-unused-render-input.md`
- `04a-retire-the-used-up-console-benchmark-arms.md`
- `04b-retire-the-one-shot-rack-builtins-and-graph-benchmarks.md`
- `04c-retire-the-issue-specific-measurement-scripts.md`
- `05-retire-the-effect-interchange-benchmark.md`
- `06-delete-dead-fixtures-and-uncompiled-research-sources.md`
- `07-prune-artifacts-to-the-live-step-records.md`
- `08-prune-handoffs-briefs-and-per-attempt-review-notes.md`
- `R1-remove-the-native-host-shells-target-smoke-and-aarch64-arms.md`
- `R2-remove-the-c-abi-and-native-pcm-runner.md`
- `R3-remove-the-control-protocol.md`
- `R4-remove-native-wav-decode-workers.md`
- `R5-accept-only-launch-sample-rates-in-descriptors.md`
- `R6-remove-third-party-effect-packages-and-state-migration.md`
- `R7-remove-the-eight-lane-wasm-measurement-build.md`
- `R8-remove-the-whole-plan-scalar-backend.md`
- `R9-retire-the-wasmtime-console-and-native-descriptive-benchmarks.md`
- `R10-keep-only-open-issue-specs-locally.md`
