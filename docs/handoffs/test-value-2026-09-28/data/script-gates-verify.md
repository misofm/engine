<!-- Produced for the 2026-09-28 test-value audit (base a9414c0c) by a helper agent, then spot-checked by the auditor. Paths such as `ci/`, `tva/` or `method/` refer to the auditor's scratch directory, which was deleted after the audit; CI run and job IDs are enough to re-fetch the logs with `gh api`. -->

> **Auditor's corrections (2026-09-28), which supersede this file where they conflict.**
> 1. §2's "a cached capi-only target dir is the honest fix" does not hold. `rust-cache` does not
>    keep workspace crates, so every run recompiles them anyway. No cheap fix was found for the
>    36 s rebuild.
> 2. §7's list of "every other target" arms is too wide:
>    - `lane/src/wide_impl.rs:291-298` and `:315-322` are the portable `max`/`min` that AArch64
>      (NEON) also compiles;
>    - `hosts/host-web/src/lib.rs:7260-7265` is the native branch every x86 build takes;
>    - `soft-clip/src/lib.rs:903-911` is a width predicate.
>
>    Only `lane/src/backend.rs:60-68`, `graph/src/runtime.rs:328-334` and
>    `target-smoke/src/lib.rs:75-85` are compiled by scalar Wasm alone. See issue 05.
> 3. The unrun one-shot files are 17, not 18 (two are under `scripts/fixtures/`): 8 + 1 + 17 = 26.


# Verification of fork-scripts.md headlines (script gates in CI)

Tree: `.claude/worktrees/agent-af4b42f777572dd67` at `a9414c0c` (= origin/main). CI figures come from PR #1016's
qualification run 36382065641: step timings from `ci/jobs-pr.json`, sub-step splits from log timestamps in
`ci/log-*.txt`. The "40 runs" are `ci/failed-steps.json`. `tva/failed-jobs.json` covers 132 failed jobs in 73
failed runs (2026-09-04..26), a superset used only where marked "73 runs". No cargo was run. Local timings
are in `timings.tsv`. Only `check-env-vocabulary.sh` (first row) ran at a low load (3.5 -> 7.0). Every other
local run overlapped the cargo-mutants runs, at a 1-minute load of 50-64 on 32 cores. Their wall times are
2-6x the CI figures and are secondary. User+sys CPU seconds were recorded for a second pass (rows `*.cpu`).

The analysis scripts are in `method/`. The timing wrappers there are `timeit.sh` and `cpu-batch.sh`.
`countrun.sh` runs a command with logging wrappers for grep/jq/rg, which is how spawns were counted.
The `onepass-env.sh` row in timings.tsv refers to `method/onepass-env.sh`.

Verdict key: VERIFIED / CORRECTED / UNVERIFIABLE.

---

## 1. Time in scripts: lint job, three script steps — VERIFIED (timings); CORRECTED (what dominates)

**Step timings are right.** The lint job ran 294 s (main push: 403 s). The three steps took:

| step | PR run | main push |
|---|---:|---:|
| env vocabulary | 71 s | 92 s |
| benchmark validators | 47 s | 74 s |
| conformance boundaries | 30 s | 38 s |
| **total** | **148 s** | **204 s** |

**What dominates is wrong in all three.** In each step the gate itself is cheap. The cost is the gate's own
mutation/self-test suite. Sub-step times below come from log timestamps (`ci/log-lint.txt`).

| step | sub-step | CI s | local CPU s (user+sys, under load ~60) | what dominates |
|---|---|---:|---:|---|
| env vocabulary (71) | `check-env-vocabulary.sh` | 15.5 (05:30:20.8 -> 05:30:36.2, log-lint.txt:552-562) | 40.7; wall 27.8 at load 3.5 | One `grep` per listed file (check-env-vocabulary.sh:44-52). There are 12,128 spawns, 10,685 of them (88 %) over `artifacts/` evidence records. A one-pass `git grep` equivalent takes **0.70 s** wall under the same load (`method/onepass-env.sh`, timings.tsv) against 102.5 s |
| | `test-env-vocabulary.sh` | **53.7** | 135.5 | The fault-injection matrix: log 05:30:44.7 -> 05:31:28.3, **43.6 s** of it. The logging wrapper counted **22,929 per-file grep spawns**, i.e. 39 full scans of the 583-file docs/scripts/tools copy. 22 of those scans run through the suite's own bash tool-wrappers (test-env-vocabulary.sh:171-224: 15 stages x 2 modes), which doubles the process count. The suite also pins the vocabulary row count `134` as an expected payload (:222-223), so adding one env var breaks the self-test. It went red that way once (33966949736: "dropped partial output ... 99") |
| benchmark validators (47) | `check-bench-preconditions.sh` | 0.05 | 0.14 | – |
| | `test-console-benchmark.sh` | **32.4** | 77.5 | **4,187 jq processes**, counted with a logging wrapper; fork-scripts said "~130 jq validations". 1,743 runs of `console-benchmark-record-validator.jq`, 58 of the aggregate validator and 187 of `web-mixing-automation-validator.jq`. The other ~2,200 jq calls build each mutated record |
| | `test-rack-benchmark.sh` | 4.0 | 10.5 | fake cargo/git lifecycle cases |
| | `test-builtins-current-benchmark.sh` | 10.3 | 28.8 | fake cargo/git lifecycle cases |
| | `test-wasm-kernel-timing.sh` | 0.3 | 0.5 | – |
| conformance (30) | `check-conformance-boundaries.sh` | 0.35 | 0.97 | – |
| | `test-conformance-boundaries.sh` | **29.3** | 80.3 | The suite runs itself three times. Pass 1 takes ~10.5 s: 74 checker invocations on a synthetic tree, 27 of them `make_shim` tool faults (test-conformance-boundaries.sh:113-128). Then the two counter-mutant blocks re-run the **whole suite** recursively with `MUTANT_RUN=1` (:141-165): 8.8 s and 9.9 s. Two thirds of the step re-proves the suite's own fail-closed plumbing |

Cargo check. None of the three steps invokes cargo:
- `test-rack-benchmark.sh` and `test-builtins-current-benchmark.sh` put fake `cargo`/`git`/`rustc` first on PATH
  and assert the fakes were selected (test-rack-benchmark.sh:487-494; test-builtins-current-benchmark.sh:360-366).
- The rack runner's argument refusals exit at run-rack-benchmark.sh:4, before its `cargo build` at :89.
- The rack fake `rustc` execs the real `rustc` only for `-vV`.

Local runs (secondary, `timings.tsv`):
- **Low-load wall:** env check 27.8 s.
- **Under load 50-64, wall:** env test 240 s / 316 s; console 138 s / 156 s; conformance test 171 s / 192 s.
- **Under load 50-64, CPU:**
  - env check 40.7 s;
  - env test 135.5 s;
  - bench step total 117.4 s;
  - conformance step total 81.3 s.

Corrected per-gate verdicts:
- **env vocabulary.** The claim "one grep per tracked file" is right for the check, but the check is only
  15.5 s of the 71 s. A single `git grep` pass removes ~15 s. Dropping the fault-injection matrix, which tests
  whether `tr`/`sed`/`sort`/`comm`/`wc` failures propagate, removes ~44 s. The self-test's literal `134` is a pin.
- **conformance.** Removing the two recursive counter-mutant re-runs saves ~19 s. Cutting the tool-failure
  shims saves most of the remaining ~10 s. The check itself is 0.35 s.
- **benchmark validators.** The console record-validator suite (4,187 jq processes) is 32 of the 47 s. All
  five suites validate descriptive benchmark records or runner lifecycles, not engine behaviour, which supports
  fork-scripts' MOVE verdict.

## 2. C ABI self-test "rebuilds capi 8 times" — CORRECTED

qualification.yml:722-725 runs `bash scripts/check-capi-abi.sh` and then `bash scripts/check-capi-abi.sh --self-test`,
and never sets `MISO_ENGINE_CAPI_SKIP_BUILD`. How the two calls build:
- **The main call** builds at check-capi-abi.sh:134-136 (`cargo build --locked --release -p capi`).
- **The self-test** builds once at :28. It then runs the checker **9 times**: one baseline and 8 `expect_failure`
  mutations. Every run passes `MISO_ENGINE_CAPI_SKIP_BUILD=1` (`common_env`, :46, :51-102, and explicitly at :95).
  So the self-test never rebuilds inside its loop.

The log (`ci/log-audit.txt:956-993`, step 05:35:43.2 -> 05:36:26.8 = 43.6 s) shows **one** real rebuild:

| span | what | seconds |
|---|---|---:|
| 05:35:43.3 -> 05:36:19.6 | `Compiling engine ... capi`, `Finished ... in 36.35s` | **36.4** |
| -> 05:36:25.7 | C11/C++17 consumers, link, run: `C ABI check: ok` | 6.1 |
| -> 05:36:26.8 | self-test, including a no-op cargo (`Finished ... in 0.07s`) and the 9 checker runs | 1.2 |

Two lines in the log say "Finished" and one set of 23 "Compiling" lines appears. **Rebuild = 36.4 s of 43.6 s.**

Why it rebuilds: the script's own header explains it (check-capi-abi.sh:4-13). The shard's four-package build
(`cargo build -p audit -p bench -p capi -p session-validator`, log line 323) unifies features: tools/audit and
tools/bench enable `engine/realtime-audit` and graph/protocol/source `test-support`
(tools/audit/Cargo.toml:17-32, tools/bench/Cargo.toml:27). `-p capi` alone therefore resolves a different set and
recompiles 23 crates.

The fix is not "set SKIP_BUILD=1" alone:
- If the main call skipped its build, the self-test's hard-coded build at :28 would do the 36 s rebuild instead.
- `SKIP_BUILD=1` would point the ABI check at a feature-unified `libcapi.so` carrying the audit instrumentation.
  `check-artifact-evidence-leak.sh` exists to keep exactly that instrumentation out of shipped artifacts.

The honest options are:
- give capi its own cached `CARGO_TARGET_DIR` (the rebuild then becomes a cache hit on unchanged PRs), or
- accept the ~36 s as the price of checking the non-instrumented library.

The self-test costs 1.2 s. Moving it off the per-PR path saves ~1 s, not "most of 43 s".

## 3. Pins with no behaviour behind them — VERIFIED (all six), with one nuance each

1. **check-dsp-research.sh.** It pins:
   - 16 headings in each of 10 notes: `required=(filters dynamics ...)` and `headings=("Scope and engineering question" ... "Known gaps and follow-up")` (:21-43);
   - at least 2 bracketed primary-source keys (:45-73), and that every key resolves in BIBLIOGRAPHY.md (:91-94);
   - the literals `DiGiCo SSL Lawo Avid Logic` in console-daw-architecture.md (:100-102);
   - the template headings (:103-105);
   - `'Evidence kind: synthetic format example' 'Sound-quality claim: none' 'SYNTHETIC-NOT-A-HUMAN'` (:106-108).

   **Behaviour:** none. No build, test or runtime reads `dsp-research/*.md`; the only code mention is a path in a
   comment at crates/dsp-reference/src/svf.rs:442. Nuance: bibliography-key resolution is a real citation-integrity
   check for the docs. docs-gates already runs only on the evidence route or the full route (qualification.yml:70-90).
2. **check-builtins-listening.sh:11-15.** For both issue-007 preregistrations it requires:
   - the lines `- Evidence kind: real listening`, `- Status: preregistered`, `- Sound-quality claim: none`;
   - the sentence `No human trial has been run.`;
   - the absence of `| 1 |`.

   It then runs `--self-test` of two validators (986 + 476 lines) for human records that do not exist
   (dsp-research/listening holds none). It also checks canonical form of the issue033 schemas.
   check-builtins-listening-033.py:23-31 hard-codes seven sha256 of `issue110` artifacts that a record must repeat
   exactly (:391). **Behaviour:** none. Nuance: the gate asserts the *absence* of evidence, so it is designed to go
   red on the day a human trial is recorded.
3. **check-parametric-eq-render-contract.sh:53-64.** It requires the test tree to contain the literals `rows, 1_488`,
   `searches, 1_104`, `cases, 48`, `sequences, 48`, `RESPONSE_TOLERANCE_DB: f64 = 0.005` and
   `ONE_SECOND_DFT_TOLERANCE_DB: f64 = 0.05` (`rg -q` anywhere under crates/parametric-eq/tests).

   **Behaviour:** none. It checks that text exists, not that an assertion uses it; the tests assert the behaviour
   themselves. The other rules break down as follows:
   - **Retired names**, `PreparedDeltaBankKernelV1 DeltaBankKernelError KernelBackendV1 process_delta` (:25-28):
     0 occurrences in crates/hosts/tools/sdk.
   - **`mul_add`, `core::arch`, `std::arch`, `is_x86_feature_detected`** (:32-35): duplicated by
     scripts/policies/lane-source.toml rules `fusion`/`architecture`/`detection`, roots crates/hosts/tools.
   - **Platform transcendentals** (:43-44): duplicated by clippy.toml:47-80 `disallowed-methods`.
   - **Unique live rules:** `is_normal`/`is_subnormal`/`sanitize_sample`, i.e. no sanitising (D7, :29-31), and
     `.div(`/`Lane::div`, a performance rule (:49-50).
4. **check-step-vocabulary.py.** It bans `nudge` as a name: NAME_PATTERN :33-42 matches `"nudge"`, `nudge_x`,
   `NudgeX`, `.nudge`, `nudge:`/`=`/`(` across .rs/.py/.json/.js/.mjs/.ts/.sh/.toml/.md. The allow-list at :50-64
   includes prose in crates/gate-expander/tests/MUTATIONS.md and tests.

   **Behaviour:** none. The wire spelling is refused independently: parameter metadata has exact keys, and
   check-parameter-metadata-v1.py:733-735 carries the red mutation "the step slot keeps its retired nudge spelling".
5. **fixtures/effect-interchange/v1/ACCEPTED.sha256.** Two kinds of rows are not wire vectors:
   - Rows 25-27 are sha256 of **Python source**, `scripts/effect-{descriptor,package,state}-v1-reference.py`.
   - Rows 1, 11 and 16 are sha256 of three nested `MANIFEST.sha256` files. The checker's own header says the
     reference's `check()` recomputes those (check-effect-interchange-qualification.sh:26-28).

   The seal also fixes membership at exactly 27 rows (:107-109). **Behaviour:** the 21 wire-vector rows are real
   protocol pins; the six source and manifest rows have no behaviour. Nuance: pinning the oracle's source stops a
   silent weakening of the independent verifier, but any comment edit forces a reseal.
6. **`REAL_WASM_SHA256`** (test-web-audioworklet.mjs:153, `"476e58ad…fbf0"`). It is compared at :240-246 only
   inside `--real-wasm-receiver` mode (:17, :3936-3937). No workflow, script or operator script passes that flag
   (grep over scripts/.github/hosts/sdk; mentions exist only in issue specs). Yet the literal changed in
   **11 commits** since 2026-08-28, including c854692a and 52d59c7c. **Behaviour:** none in CI.
   - The CI invocations of the .mjs are:
     - qualification.yml:306 (lint);
     - test-web-audioworklet.sh:6, :22, :25, :134, :372 and :380 (artifact-gates).
   - None of them passes `--real-wasm-receiver`.

## 4. Gate rules guarding removed features — VERIFIED (with file:line)

| retired feature | rule | notes |
|---|---|---|
| FLAC / delivery-codec stack | check-workspace-policy.sh:162-163 (package names `flac-decoder\|stem-publisher\|catalog-migrate\|flacenc\|symphonia`), :218-231 (stub directories), :237-272 (Cargo identity scan of every manifest and Cargo.lock); 17 matching lines in test-workspace-policy.sh | Enforces AGENTS.md "delivery codecs live outside", but only for five exact names; a new codec under another name passes |
| retired `miso-engine-` prefix | check-workspace-policy.sh:156-158 | AGENTS.md mandates the unprefixed scheme, so this one is live policy, not dead |
| session TOML | check-session-policy.sh:46-58 (no live `*.toml` sessions outside the 12-line scripts/session-policy-historical-allowlist.txt), :60-63 (`SessionToml\|parse_session_toml\|canonical_session_toml\|canonical_toml_chunk\|maximum_toml_bytes\|sessionTomlBytes\|sessionToml\|toToml\(`); test-session-policy.sh:47, :111-121; check-sdk-deletions.py:120-121, :409 | the TOML format was retired (#338) |
| software FMA | check-unfused-seal.sh:723-732 (rule 6, `fn fma_f32_via_f64`/`fma_f32x[48]_soft` must not reappear in crates/lane/src/softfma.rs); self-test cases :239-282, :337-341, :406, :450-451; clippy.toml:36-37 records it | softfma retired in #163 phase 2 |
| delta-bank EQ kernel | check-parametric-eq-render-contract.sh:25-28 | 0 live occurrences |
| pre-boot-v1 SDK | check-sdk-deletions.py:99-110 (DELETED_IDENTIFIERS), :113-118 (`limits` key/member), :120-125 (TOML format, `"config"`/`"prepared"` state words), :156-158 (literal `192`), refusals :388-446; 37-mutation self-test `self_test` :495-623 (20 s in CI) | **4 reds in 73 runs** were these bans firing on new code: `"prepared"` in sdk/src/assets/prepared-control.js (34861401024) and pcm-feed (33960625717), and a `limits:` key in live-response.ts / response.ts (34749172858, 34737581447). Each forced a rename, and none revived retired code. The numeric-offset half (:439-446) is the live rule |
| raw-WebDriver browser harness | scripts/web-audioworklet-browser-correctness.py (752), run-web-audioworklet-browser-correctness.sh (53), operator/seal-web-audioworklet-browser-correctness.sh (47); CI still runs its `--self-test-webdriver-responses` plus a sed mutation of it at test-web-audioworklet.sh:146-157 | qualification.yml:207-212 says `check-browser-expected-resources.py` subsumed it |
| input-symmetry (RT5) capture #602/#603/#606 | scripts/{run,preflight}-input-symmetry-capture.sh, input-symmetry-capture-validator.py, test-input-symmetry-capture.sh (897 lines, never run); 20 env-vocabulary rows (docs/ENGINE_ENV_VOCABULARY.md:41-87: 9 `MISO_ENGINE_606_*` and 11 `MISO_ENGINE_CAPTURE_*`) that the env gate keeps live; tools/bench/src/input_symmetry.rs (597) and input_symmetry_capture.rs (226) still compile into `bench` | GitHub #606 closed 2026-09-08. Calling it "superseded by the dual-mono ruling" is the earlier helper's reading, **UNVERIFIED**: #602 was a one-shot RT5 input-trim render-cost capture |
| builtins-less console rows (#956) | test-console-benchmark.sh:531-536; test-wasm-console-benchmark.sh:164-170 | owner ruling: the builtins-less path was removed |

## 5. Stale pins among the 40 recent reds — VERIFIED (">= 10"); CORRECTED upward

Every failed job log for the 40 runs was fetched (`tva/faillogs`, all 40 runs covered). The classifier is
`method/pinreds.py`. It matches only non-echo error lines: CI logs echo the workflow's `run:` text, which contains
the error strings, and the earlier helper's approach would have false-matched them.

| pin | error text | runs |
|---|---|---:|
| artifact sha256 | `AudioWorklet artifact pin mismatch: expected=… observed=…` (build-web-audioworklet.sh:95-99) | **8** |
| Issue-544 dict | `AssertionError: …: layout_total_bytes != 5896` (qualification.yml:671) | **2** |
| browser matrix lineage | `Error: matrix: artifact-lineage: checked wasmSha256 differs from the artifact under qualification` (run.mjs:117-121), all three browsers each time | **5** |

- **Artifact sha256 runs:** 36232835088, 36122303914, 36121392285, 36084071070, 35046415805, 34921544375,
  34727193810, 34448821511.
- **Issue-544 dict runs:** 36171020566, 36170326170.
- **Browser matrix lineage runs:** 36131242221, 35122195213, 35028872238, 35027155140, 34921938092.

So the brief's three kinds account for **15 of the 40 runs** (no overlap). "At least 10" holds; the figure is 15.

Two more pins of the same construction show up in the same logs (outside this headline's scope, listed for the
other helper):
- **expected.json resource rows** (`FAIL browser expected resources: expected.json's resource rows are stale …`):
  5 runs. Two of them are new: 34922385988 and 34875764291.
- **`graph audit record hash differs`** (scripts/trace-builtins-graph-audit.sh:65-69): 4 runs, 34923128333,
  34877814180, 34758939108 and 34566763819. The four form a chain of successive pins. The sha256 covers an audit
  record whose only variable fields are three compile-time constants printed by tools/audit/src/builtins_graph.rs:48-53
  plus counters that jq already asserts at :47-60. There were 6 repin commits since 2026-08-28. This **corrects
  fork-scripts**, which lists the trace audits as having no pins.

With those, 21 of 40 runs contain at least one pin-refresh red. **12 of 40 runs were red only because of pins**:
- artifact-only: 5;
- lineage-only: 2;
- lineage plus resource rows: 2;
- graph-hash-only: 3.

A further 4 runs failed in crates/capi/tests/resource_lifecycle.rs; their cause is not classified here.

## 6. Unrun scripts — CORRECTED: 26 files, 6,119 lines (not 27 / 6,167)

Derivation: `method/reach.py`.
- **Seeds:** all four workflows and the two package.json `scripts` tables.
- **Edges:** a non-comment mention of a file's basename, with brace lists expanded, followed transitively
  through reached files, including sdk/host-web node code.
- **jq modules:** also matched by `include "stem"`.

Fork-scripts' extra file is `scripts/protocol-benchmark-record-validator.jq` (48 lines). It **is** reached:
qualification.yml:742 -> test-protocol-benchmark.sh:22 `include "protocol-benchmark-record-validator"`. A basename
search misses the jq module include.

A "broad" pass that also treats every .rs/.ts/.mjs outside scripts/ as a carrier reached three more files, but
only through an `#[ignore = "... run once through scripts/run-issue880-mq2-benchmark.sh"]` string. That is not an
invocation, so the strict result stands.

The 26 files:
- **Operator tools by design (scripts/operator/, README.md there says "not gates"):** 8 files, **1,479 lines**.
  - preflight-console-benchmark.sh 177
  - preflight-rack-benchmark.sh 39
  - preflight-wasm-console-benchmark.sh 269
  - prepare-builtins-listening.sh 87
  - probe-opfs-move-v1.cjs 157
  - run-wasm-console-benchmark.sh 496
  - run-wasm-kernel-timing.sh 207
  - seal-web-audioworklet-browser-correctness.sh 47
- **Live operator runner outside operator/:** scripts/run-console-benchmark.sh, **682 lines**. Last changed
  2026-09-27 (d3349b72). It violates the operator README's own rule.
- **One-shot issue tooling (18 files, 3,958 lines):**
  - issue 880 MQ1/MQ2, 9 files, 1,409 lines: run-issue880-mq1-benchmark.sh 355, run-issue880-mq2-benchmark.sh 248,
    issue880-mq1-benchmark-lib.sh 104, issue880-mq{1,2}-record-validator.jq 57+55,
    fixtures/issue880-mq{1,2}-record.json 46+288, test-issue880-mq1-benchmark.sh 212,
    test-issue880-mq2-benchmark.sh 44;
  - issue 606 input symmetry, 4 files, 897 lines;
  - issue 746: run-gate-active-benchmark.py 931, test-gate-active-benchmark.py 463;
  - issue 650: check-prepared-effect-allocation-records.py 245;
  - check-parametric-eq-targets.sh 13, a compatibility shim.

Material finding: scripts/operator/README.md states "**Every script under `scripts/` is reachable from a GitHub
workflow.** That rule is mechanically checkable". 18 files at `scripts/` top level break it, and no gate enforces
it (grep for such a check finds only the README).

## 7. Scalar wasm — VERIFIED; the shipped artifact is already checked for atomics

- **Only simd128 ships.** build-web-audioworklet.sh:36-40 records owner decision W4-D1 (one artifact), and its
  only build is `+simd128` (:67-71). The host goes further (miso-engine-v1-audio-worklet-host.js:12-16, :61-64):
  it probes simd128 and rejects a module whose backend row is not `simd128`, transactionally. A scalar build
  cannot even boot in the shipped host.
- **The scalar legs:**
  - The wasm-guests "Scalar Wasm build" builds 18 packages with `-C target-feature=-simd128` (qualification.yml:766-769).
    **38 s**. No later step consumes its output; the atomics checker builds its own tree.
  - "Browser-local realtime path has no atomic opcodes" (:772-775): **35 s**. test-wasm-realtime-atomics.sh takes
    13.3 s. check-wasm-realtime-atomics.sh takes 22.0 s and does a fresh scalar non-LTO build of
    engine/source/target-smoke (:37-40). Its messages still say "browser-local fallback artifact" (:33, :88).
  - check-protocol-wasm-parity.sh:201 `run_variant scalar -simd128`: ~9.2 s of the 19 s step.
  - run-wasm-gates.sh:201 `run_guest scalar -simd128 scalar`: ~34.8 s of the 63 s step
    (log-wasm.txt 05:33:21.7 -> 05:33:56.5).
  - check-cross-targets.sh:65-114 (cross-target job) also has a scalar mode: parametric-eq check, builtins build,
    effect-package/compiler/conformance check, cdylib plus a "no SIMD opcode" assertion. Its share of the 43 s
    step is not separable from the quiet log.
  - Evidence crates `-simd128` check: ~1-2 s.
  - Total: **≈117 s** of runner time plus the scalar half of the 43 s cross-target step. That is runner time,
    not critical path (audit-native, 463 s, is).
- **Atomics on the shipped artifact are already checked elsewhere.** check-web-audioworklet.sh:309-341 runs over
  the downloaded, pin-verified simd128 module and checks:
  - exact exports;
  - no imports (:323-326);
  - no shared memory (:327-335);
  - `check_no_atomic_opcodes` on the full disassembly (:336-340), self-tested at :82-109.

  The scalar check is a pre-LTO object scan of a configuration that never ships. Without `+atomics`, wasm32
  lowers atomics to plain loads and stores anyway, which the scalar script's own cfg check (:29-33) already
  guarantees.
- **cfg code.** There is no literal `cfg(not(target_feature = "simd128"))` in crates/. There **are**
  "every other target" arms, `cfg(not(any(x86, x86_64, aarch64, all(wasm32, simd128))))`, that only a scalar
  wasm (or an unsupported arch) compiles:
  - crates/lane/src/backend.rs:59-67 (`Backend::Scalar`);
  - lane/src/wide_impl.rs:290-297 and :314-321 (portable max/min);
  - graph/src/runtime.rs:328-335 (`FrameLane = f32`);
  - soft-clip/src/lib.rs:903-911 (width_is_native);
  - target-smoke/src/lib.rs:74-83;
  - hosts/host-web/src/lib.rs:7260-7265 (`BACKEND_SCALAR`).

  Native aarch64 is "unsupported, see #378" (check-cross-targets.sh:124), so the scalar-wasm legs are the only
  CI coverage of these arms. They are candidates under "modes production never needs" together with the arms.
  Keeping the legs without the arms, or the arms without the legs, is inconsistent. This needs an owner ruling.

## 8. Duplicate runs — VERIFIED (all five); costs attached

| duplicate | sites | CI cost |
|---|---|---|
| parameter-metadata release (fat-LTO) build, three jobs | artifact: build-web-audioworklet.sh:108-113 (`--write`); artifact-gates: check-web-audioworklet.sh:490-496 (`--check`); sdk: check-sdk-generated.sh:26 -> sdk/codegen/assets.mjs:42-47 (`--print`, `--print-abi-layout`) | artifact **61.6 s** (log-artifact.txt: wasm Finished 05:30:00.1 -> upload 05:31:01.7); artifact-gates **75.0 s** (05:32:21.3 -> 05:33:36.3); sdk **72.3 s** (05:31:36.0 -> 05:32:48.3). Plus debug-profile runs at test-web-audioworklet.sh:219-227 (~12 s). qualification.yml:143-148 already records the sdk half as a known follow-up |
| `node scripts/test-web-audioworklet.mjs` | lint qualification.yml:306; artifact-gates test-web-audioworklet.sh:6 (exact same no-argument call) | 0.55 s |
| check-sdk-generated.sh twice in `sdk` | qualification.yml:172; sdk-package.sh:43 | 2nd call 0.5 s (warm) |
| effect-runtime policy, fixtures and both mutation suites | lint qualification.yml:405-408; audit-native via check-effect-contract.sh:30-33 | 4.9 s in audit-native (log-audit.txt 05:36:37.7 -> 05:36:42.5) |
| interchange qualification | lint qualification.yml:414; cross-target check-cross-targets.sh:47 | ~0.2 s (the check alone; the 25-case mutation suite is not re-run) |

## 9. Artifact pin forces re-pin commits — VERIFIED; count depends on definition

- **The per-PR build refuses on a stale pin.** qualification.yml:111-117 runs `build-web-audioworklet.sh` with no
  repin variable, and :95-99 exits 1 on `observed != expected`. `sdk`, `artifact-gates` and `browser` all
  `needs: artifact`.
- **A comment-only change forces a repin.** 9465b329 changed parametric-eq doc comments and added in-file tests
  ("No rendered code changes"). The follow-up 52d59c7c had to repin; its message says: "four doc-comment lines in
  non-test code shift the module's panic-location line numbers, so the bytes change (16 bytes, same size)".
- **Commits since 2026-08-28** (2,957 non-merge; `method/pinonly.py` over `git log --name-only`):

| definition | commits |
|---|---:|
| strict list given (artifact `*.sha256`, results.json, expected.json, BROWSER_DEPLOYMENT_MATRIX.md, resource_lifecycle.rs, `docs/specs/MUTATIONS.md`) | **50** (1.7 %) |
| + any `MUTATIONS.md` + scripts/test-web-audioworklet.mjs (the orphan `REAL_WASM_SHA256` carrier; c854692a and 52d59c7c are pin + .mjs only) | **56** (1.9 %) |
| + `.github/ISSUE_SPECS/**` and `docs/**` (fork-web's definition; it reported 131) | **125** (4.2 %) |

Notes on the table:
- `docs/specs/MUTATIONS.md` has never existed (`git log --all` is empty); the brief presumably meant the
  per-crate MUTATIONS.md files.
- Of the 50 strict commits, one (8f30c0c2, "Clarify issue470 double-live accounting") touches only
  resource_lifecycle.rs and may be more than a repin.
- Per pin file, in the same window:
  - artifact sha256: 105 commits;
  - results.json: 95;
  - matrix doc: 95;
  - expected.json: 24;
  - resource_lifecycle.rs: 19;
  - any of these: 194.

## 10. Challenge to the "Must stay" list

fork-scripts.md has no section titled "Must stay". I used its KEEP verdicts (and fork-web's closing "Keep" list).

History sources:
- **Reds:** the 73 failed runs in `tva/failed-jobs.json`, 2026-09-04..26. "caught" means a non-pin, non-flake red
  whose error text names a real rule hit.
- **Code comments** where they document a past defect.

| KEEP gate | defect class it catches | history | challenge / corrected verdict |
|---|---|---|---|
| check-ci-path-routing.py + test (19 s) | a code PR routed around the full job set; a required context pending forever | 1 red: the self-test's "mutation anchor absent" after a workflow edit (33961500605), i.e. a text anchor, not a routing defect | KEEP the check; route the self-test to router or workflow changes |
| fmt / clippy / doc | hygiene; D6 via disallowed-methods | rustdoc 4, clippy 2, fmt 1 reds | KEEP |
| check-workspace-policy.sh | sysroot-name shadowing (`core`), MAX_TRACKS, ISA pin, naming | 0 reds | KEEP; drop the five-name codec bans (they catch only exact re-adds) |
| check-bench-policy.sh | a second allocator/escaper/percentile (the #104 history in its header) | 1 red was the self-test's "wrong diagnostic" (33962425273), not a duplicate | the check could move with the benchmark suites; keep off the required path |
| check-host-core-policy.sh | a second host pipeline; facade depending on control protocol | **caught 1**: "host facade must not depend on the control protocol" (33878884725) | KEEP |
| check-protocol-control-policy.sh | raw-byte escape hatch | 0 | KEEP (cheap) |
| check-realtime-policy.sh (+ leak checks) | unsafe/alloc tokens outside approved files; instrumentation feature in shipped artifacts | **caught 1**: "unsafe code exists outside the issue-approved ownership/audit files" (33969628807); 1 flake on main: `cargo tree` could not download `wasi` (34250520726); the leak gate's header documents a real prior leak (#105 C2) | KEEP; the leak half needs network (cargo tree), a flake source |
| check-lane-policy.sh | fusion or `wide`/arch outside lane, i.e. cross-target bit divergence | **caught 1**: "fused multiply-add and the SIMD vocabulary belong to crates/lane" (34753099504) | KEEP (strongest static gate by history) |
| check-unfused-seal.sh | `Lane::fma` fused or split per target | 0 | KEEP minus rule 6 (softfma) |
| check-effect-runtime-fixtures / policy | fixture corpus integrity; dependency lists; definition caps | 1 red: `normalize_zero has 3 definitions, the manifest pins at most 2` (34737581447), a count cap | KEEP fixtures; the count caps are pins |
| rack/builtins/graph policy (fork: MERGE) | dependency lists; `unsafe`; executor ownership | 4 reds in the 40: **2 false positives** where the word "unsafe" in a comment tripped check-builtins-policy.sh:22-24 (34875764291 `crates/builtins/src/lib.rs:974`, 34729987518 `tests/filter_response.rs:404`), and 2 on one branch for "production prepared-plan executor must remain graph-owned" (check-graph-policy.sh:73-74) | the `unsafe` substring scan duplicates `[workspace.lints.rust] unsafe_code = "deny"` (Cargo.toml:99-100, all 29 crates opt in) and fires on prose; replace it |
| check-sdk-generated / types / headless / package | codegen drift; SDK API | SDK step red 6x in 73: **4 were check-sdk-deletions retired-word hits on new code** (see §4), 1 headless eval failure (35093282391), 1 unclassified (33932900368) | KEEP types/headless/package; generated check to `cmp`; deletions keep only the numeric-offset rule |
| check-web-audioworklet.sh + children | render/meter/command closure allocation-free; atomics/exports; kernel vector ratchet; cross-language vocabulary | **caught**: kernel roster ratchet `route_reduce … vector=560 scalar=1680` (36171020566, 36170326170); command-reason vocabulary drift `observe() no longer accepts exactly requestId/subscriptions` (33932900368) | KEEP; drop the tautological `parameter-metadata --check` in CI (75 s) |
| check-browser-expected-resources.py (fork: SHRINK) | native vs simd128 digest; exact resource bytes | 5 reds in the 40, all stale rows | agree: keep the digest half |
| test-web-audioworklet.sh / .mjs | JS host behaviour | 2 reds on one branch (`1 !== 0`, 33936364417/33935625430) | KEEP once; drop the lint copy |
| audits + trace scripts (fork: "no pins") | zero alloc/lock/syscall over 1e5-1e6 blocks | all **4** "Builtins realtime audit" reds were `graph audit record hash differs`, a stale pin (trace-builtins-graph-audit.sh:65-69) | **CORRECTED**: KEEP the audits, drop `expected_audit_hash` |
| audit capi / source-duration + Issue-544 validator | realtime; memory independent of stem duration | 2 reds, both the 6472 dict | KEEP the claims; drop `layout_entries`/`layout_total_bytes` |
| check-capi-abi.sh | frozen C header, symbol set, C/C++ consumers | 0 reds | KEEP; see §2 for the build cost |
| check-graph-determinism.sh | HashMap order nondeterminism | 0 | KEEP |
| check-builtins-fixtures.sh / check-console-fixtures.sh | DSP bit identity; fixture regeneration | 0 (the lint-side manifest-consumer *copy* red once: "stale or missing builtins manifest consumer", 34922385988) | KEEP; cut the consumer-copy check |
| check-effect-contract.sh | every production effect factory runs the conformance harness | **caught 1**: "missing crates/effect-compiler/tests/conformance.rs … every production NativeEffectFactory must run the shared harness" (34861401024) | KEEP; drop its re-run of the lint scripts |
| check-native-pcm-runner.sh (fork: SHRINK) | runner bypassing product API | 2 reds on one branch: "forbidden product bypass dependency engine" (34110112332, 34108626421) | the reachability/dependency half earns its keep |
| env vocabulary (fork: CUT-CANDIDATE) | doc sync of MISO_ENGINE_* names | **9 reds in 73 runs**: 5 undocumented name, 3 foreign prefix, 1 self-test row-count pin (fork said "once in 40") | still doc sync only; the prefix rule is the only part tied to code (a runner and a binary agreeing on a name) |
| test-npm-publish-modes.py, check-release-shape.py, check-stem-store-v1.mjs, x86 probes, fuzz, math sweeps | release safety; ISA; product | 0 reds | KEEP (no history either way) |

Gates the history shows catching real rule violations:
- lane policy;
- realtime policy;
- host-core policy;
- effect-contract completeness;
- the kernel ratchet;
- cross-language vocabulary;
- native-pcm-runner dependency.

Gates whose recorded reds are pins, self-test plumbing or false positives:
- builtins `unsafe` substring;
- sdk-deletions retired words;
- bench-policy self-test;
- path-routing self-test anchor;
- env vocabulary row count;
- graph-audit hash;
- expected.json rows;
- the Issue-544 dict.

Gates with no recorded red either way: capi ABI, graph determinism, unfused seal, workspace policy, release
shape, npm modes. Their value rests on the claim, not on history.
