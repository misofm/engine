ready to push

# Decision-15 stream A batch verdict (`43ba3f7c0`)

- Source: branch `codex/d15-stream-a` at `43ba3f7c0` (worktree `/home/bl/misofm/wt-d15-a`, not
  edited; still clean at `43ba3f7c0`), on `main` `0a1176b3b`. Three commits: `c288358b4` (#1300,
  attempt 1, PASS), merge `13302fceb` of `0a1176b3b`, follow-up `43ba3f7c0` (doc comments in
  `crates/soft-clip`, spec text, copied verdicts). Remote `main` was still `0a1176b3b` when I
  checked; the branch is not pushed.
- Method: `git clone --no-checkout /home/bl/misofm/wt-d15-a /tmp/claude-1002/vbatch-a/tree`,
  checkout `43ba3f7c0`. One cargo target, `CARGO_TARGET_DIR=/tmp/claude-1002/vbatch-a/target`,
  also reached through a `tree/target` symlink (in the clone's `.git/info/exclude`), so the
  workflow's literal paths (`./target/release/audit`, `target/ci/...`) resolve as in CI.
  `TMPDIR=/tmp/claude-1002/vbatch-a/tmp`, `RUSTUP_TOOLCHAIN=1.97.1`. Each step ran under
  `bash -e` (the Actions default for a step with no `shell:`); steps that need `pipefail` set it
  themselves, as in the workflow. `git status` in the clone was clean after every gate (only the
  ignored `node_modules` and `target`).
- Router: `check-ci-path-routing.py` and `test-ci-path-routing.py` pass. `ci-path-router.py
  --flags --event pull_request --base 0a1176b3b --head 43ba3f7c0` gives `route=full`,
  `math_closure=false`, `release_inputs=false`, `self_tests=[]`.
- Host: AMD EPYC 7313P, 32 threads, shared with other agents (load average 20-47). rustc 1.97.1,
  Node v22.23.2, Playwright 1.62.1 (chromium 151.0.7922.34, firefox 153.0, webkit 26.5).
- Result: 97 of 97 steps pass, in every runnable job, including the three jobs and steps that this
  route skips (`release-shape`, `gate-self-tests`, the M1/F1 sweeps). No blocker. No failure, so no
  comparison run on `main` was necessary.

## Routing (verdict job's expectation table, `qualification.yml:1091-1137`)

With `route=full`, `release_inputs=false`, `self_tests=[]`, on `pull_request`:

| job | expected on the PR | local result |
| --- | --- | --- |
| route | success | pass (policy, mutation tests, router) |
| docs-gates | success | pass |
| artifact, artifact-identity, sdk | success | pass |
| artifact-gates, browser (3 legs), lint, test-debug-a, test-debug-b, test-release, audit-native, wasm-guests, cross-target | success | pass |
| aarch64-debug, aarch64-release | success | not runnable here (see below) |
| release-shape | skipped (`release_inputs=false`) | ran anyway: pass |
| gate-self-tests | skipped (`self_tests=[]`) | ran anyway: all five suites pass |
| artifact-record | skipped on a PR; success on the merge push to `main` | not run (writes a commit status) |
| test-release M1/F1 steps | skipped (`math_closure=false`) | ran anyway: pass |

## Jobs that cannot run on this host

| job | reason |
| --- | --- |
| aarch64-debug | `runs-on: ubuntu-24.04-arm`; it needs native arm64 (NEON `Simd4`, FPCR pinning). This host is x86-64 with no arm64 user-mode emulator. The AArch64 cargo check, clippy and iOS/Android assembly scans did run, in `cross-target`. |
| aarch64-release | Same reason (`qualification.yml:951-981`). |
| artifact-record | Runs only on a push to `main` and posts the `audioworklet-sha256` commit status with a write token (`qualification.yml:207-223`). Running it from here would write to GitHub. |
| verdict | An aggregation of job results; I applied its table by hand above. |

## The shipped module and the pin

- This batch's module: `d4cf86ea0fe3523a560a4e2bc0eefd738435c5c7f977b0ae4b47d37cc02cb9fa`,
  2896157 B. Named twin `46abba7a66b1293b52b7295e2172df41e0f708a2c01ac006b1ea6aa65d357fd4`,
  closure `1062dc2dcd12c6851488019988eaf3cad2b0d50d81ad80a20e06c8d0660991e7`. A twin build
  from another checkout path and another `CARGO_HOME` (a second clone at a synthetic PR merge
  commit with parents `0a1176b3b`, `43ba3f7c0` and the tree of `43ba3f7c0`) gave the same bytes.
- `web-audioworklet-identity.py report` against the base's real record (read-only `gh api`):
  **ARTIFACT CHANGED: `09622a43...` -> `d4cf86ea...`**, base `0a1176b3b`, record from run
  37320574266, both with rustc 1.97.1. "Reproducible". "Release fingerprint not checked: this
  change does not edit the pin or `npm-publish.yml`'s release identity." Exit 0.
- No `qualification.yml` job compares the module with the committed pin
  (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`, `6c952a2c...`) on a
  non-release change:
  - `artifact` builds in delivery mode (`scripts/build-web-audioworklet.sh:33`); only
    `--check-pin` compares (`:122-129`), and no workflow step passes it.
  - `artifact-identity` holds the pin only for a release change, that is one that edits the pin
    file or `npm-publish.yml`'s `PACKAGE_VERSION`/`EXPECTED_WORKLET_SHA256`
    (`scripts/web-audioworklet-identity.py:214`, `:344`). This batch edits neither.
  - The browser legs' `--check-matrix` no longer checks `results.json`'s `wasmSha256` lineage
    (`hosts/host-web/qualification/run.mjs:907-911`, #1061).
  - `sdk-package.sh check` runs `test-sdk-artifact-builder-output-contract.sh`, which reads the
    pin, but only with a fake cargo, so the real module's bytes do not reach it (`:214-261`).
  - `suspended-host.mjs` and `run-web-mixing-automation-benchmark.sh` read the pin, but no
    workflow runs them.
- What it means for "ready to push": the unpinned digest is not a blocker. The PR's
  `artifact-identity` will print `ARTIFACT CHANGED 09622a43... -> d4cf86ea...` and pass. The pin
  is checked only by a release PR and by `npm-publish.yml`, which is `workflow_dispatch` only. That
  workflow already refuses on `main`, independent of this batch: pin `6c952a2c...` differs from
  `EXPECTED_WORKLET_SHA256` `e18acf9c...` (`docs/RELEASE.md`, "Between releases").

## Non-blocking note: the batch head's digest is not the one the evidence quotes

The caller's brief and `docs/handoffs/decision-15-2026-10-05/verdicts/stream-a/README.md:13` quote
`9275bcce...` (2896157 B). I rebuilt `13302fceb` and got exactly `9275bcce3596...`, so that number
is correct for the merge commit. But the batch head `43ba3f7c0` builds `d4cf86ea...`. The two
modules differ in 11 bytes, all in the data section, by +12 or +14 each. The follow-up adds 12
comment lines in `crates/soft-clip/src/lib.rs` before line 805 and 2 more after it. The deltas
match these line shifts exactly, so these bytes are almost certainly `core::panic::Location` line
numbers. I did not decode the data segment to confirm this. No code byte moves (all 11 offsets are
past the end of the code section). No rendered bit moves, which agrees with
the spec's "no code or rendered bit moves"
(`.github/ISSUE_SPECS/1300-let-soft-clip-restore-its-own-non-finite-history.md:360`). Any record
that cites "the batch's module" should cite `d4cf86ea...`, which is the digest the PR's CI will
print. This does not block the push. If root wants the README line to name the head's digest, it is
a one-line edit, and that edit would not move the module again (it is a `.md` file).

## Local deviations from CI (none changes a verdict)

- One shared target dir and `TMPDIR` instead of per-job runners and rust-cache. The
  upload/download-artifact steps became local paths, with the same sha256 and closure checks.
- `artifact-identity`'s twin is a second `git clone` at the synthetic merge commit, not
  `git worktree add` (the brief forbids `git worktree add`). Its status lookup used the local `gh`
  login (a read-only GET), not `GITHUB_TOKEN`.
- Browser legs: no `sudo`, so `npx playwright install-deps` and the apt install of pulseaudio did
  not run. Pulseaudio and the browser system libraries were already present. The Playwright
  revisions that 1.62.1 needs (chromium-1234, firefox-1538, webkit-2336) were already in
  `~/.cache/ms-playwright`, so `npx playwright install` downloaded nothing. `sdk/dist` was deleted
  before each leg, and each leg printed `sdk bundle: the source at .../sdk/src (CI's mode)`. The
  legs ran one after the other, with the pulseaudio null sink started exactly as in
  `qualification.yml:415-429`.
- Node v22.23.2 for every job (CI uses `22` for `sdk` and `browser`, and exactly 22.23.2 for the
  V8 spill gate).
- Lint's sub-v3 probe logs went to `$TMPDIR` rather than `/tmp/ci-*-probe.log`, so that they do not
  collide with other agents' files.

## Gate table

One row per workflow step. A step's several commands ran as one script. The duration is wall
seconds on the shared host. The route job (path-routing policy and its mutation tests, then the
router) also passed; it is not in this table.

| job | step | ran | result | time |
| --- | --- | --- | --- | --- |
| docs-gates | DSP research corpus and listening-evidence packet validators | ran | pass | 11s |
| artifact | Build the exact shipped artifact | ran | pass | 105s |
| artifact-identity | Make the PR merge-commit twin checkout (clone, not worktree) | ran | pass | 1s |
| artifact-identity | Rebuild the module from another checkout path and CARGO_HOME | ran | pass | 29s |
| artifact-identity | Report ARTIFACT CHANGED or UNCHANGED against the base's recorded digest, and hold a release change to its pin | ran | pass | 2s |
| sdk | Install SDK dependencies | ran | pass | 1s |
| sdk | Verify the downloaded artifact against the artifact job's digest | ran | pass | 0s |
| sdk | Qualify the SDK package against the shared artifact | ran | pass | 51s |
| artifact-gates | Verify the downloaded artifact against the artifact job's digest | ran | pass | 1s |
| artifact-gates | Verify the named twin against the artifact job's digest and the shipped module | ran | pass | 0s |
| artifact-gates | Browser AudioWorklet artifact gates and native parity | ran | pass | 62s |
| artifact-gates | Shipped AudioWorklet module contains no whole-plan scalar path | ran | pass | 0s |
| artifact-gates | Hermetic browser host and worklet tests | ran | pass | 4s |
| artifact-gates | V8 spill gate over the shipped module's EQ cascade loops (node 22.23.2) | ran | pass | 2s |
| browser-chromium | Verify the downloaded artifact against the artifact job's digest | ran | pass | 0s |
| browser-chromium | Install pinned browser (npm ci + playwright install; install-deps skipped: needs sudo, deps already present) | ran | pass | 2s |
| browser-chromium | chromium - attestation, AudioWorklet, native digest, and stall gates | ran | pass | 6s |
| browser-firefox | Verify the downloaded artifact against the artifact job's digest | ran | pass | 0s |
| browser-firefox | Install pinned browser (npm ci + playwright install; install-deps skipped: needs sudo, deps already present) | ran | pass | 1s |
| browser-firefox | firefox - attestation, AudioWorklet, native digest, and stall gates | ran | pass | 14s |
| browser-webkit | Verify the downloaded artifact against the artifact job's digest | ran | pass | 0s |
| browser-webkit | Install pinned browser (npm ci + playwright install; install-deps skipped: needs sudo, deps already present) | ran | pass | 1s |
| browser-webkit | webkit - attestation, AudioWorklet, native digest, and stall gates | ran | pass | 8s |
| lint | Hydrate locked Cargo dependencies | ran | pass | 0s |
| lint | Format | ran | pass | 4s |
| lint | Clippy | ran | pass | 43s |
| lint | Documentation | ran | pass | 31s |
| lint | Workspace policy | ran | pass | 1s |
| lint | Workspace policy mutation tests | ran | pass | 19s |
| lint | Session dependency and compiler boundary policy | ran | pass | 2s |
| lint | Environment and marker vocabulary policy | ran | pass | 0s |
| lint | Bench harness duplication policy and mutation tests | ran | pass | 9s |
| lint | test-support CI coverage policy and mutation tests | ran | pass | 3s |
| lint | Scalar-oracle-absent gate self-test | ran | pass | 0s |
| lint | Script reachability policy and mutation tests | ran | pass | 14s |
| lint | Console benchmark fixture integrity | ran | pass | 0s |
| lint | Standing builtins fixture manifest mutation tests | ran | pass | 2s |
| lint | Benchmark preconditions | ran | pass | 0s |
| lint | Shared host facade policy and mutation tests | ran | pass | 2s |
| lint | Protocol control typed-boundary policy and mutation tests | ran | pass | 1s |
| lint | Realtime source policy and mutation tests | ran | pass | 76s |
| lint | Realtime trace validator mutation tests | ran | pass | 0s |
| lint | Lane numeric-boundary policy and mutation tests | ran | pass | 21s |
| lint | Unfused multiply-add seal and self-test | ran | pass | 8s |
| lint | Rack, builtins and graph dependency-boundary policy and mutation tests | ran | pass | 5s |
| lint | Effect runtime dependency-boundary and fixture policy and mutation tests | ran | pass | 8s |
| lint | Conformance boundaries | ran | pass | 0s |
| lint | Parametric EQ render-contract seal | ran | pass | 0s |
| lint | Release shape policy self-test | ran | pass | 1s |
| lint | npm publish mode reachability and trust gates | ran | pass | 5s |
| lint | Stem store V1 gate | ran | pass | 2s |
| lint | Stem identity corpus drift check | ran | pass | 0s |
| lint | Sub-v3 builds are refused by the lane guard (scalar) | ran | pass | 3s |
| lint | Sub-v3 builds are refused by the lane guard (AVX2 without FMA) | ran | pass | 5s |
| lint | AVX2 with FMA compile probe and cfg assertion | ran | pass | 6s |
| test-debug-a | Compile builtins-compiler tests with default features | ran | pass | 8s |
| test-debug-a | Workspace debug tests (excluding DSP crates, audit, and wasm evidence tools) | ran | pass | 225s |
| test-debug-b | DSP crates debug tests (lane feature unification pinned explicitly) | ran | pass | 254s |
| test-debug-b | Conformance research-fixture completeness | ran | pass | 8s |
| test-release | Lane and math gates (G1-G4, G6, P1, M1-M3) and wasm-gates G5/G6 in release | ran | pass | 245s |
| test-release | M3 leg is built with FMA (cargo cfg for math) | ran | pass | 0s |
| test-release | Realtime race model (engine SPSC loom leg) | ran | pass | 45s |
| test-release | Math M1 exhaustive sweep (routed skip: math_closure=false; run anyway) | ran | pass | 45s |
| test-release | Fast dB tier gate F1 exhaustive (routed skip: math_closure=false; run anyway) | ran | pass | 52s |
| audit-native | Build release audit, bench, capi, and session-validator binaries | ran | pass | 97s |
| audit-native | Audit and console-workload unit tests in release | ran | pass | 143s |
| audit-native | Issue-544 C ABI runtime caller audit | ran | pass | 1s |
| audit-native | Validate Issue-544 runtime audit records | ran | pass | 0s |
| audit-native | Delay realtime allocation and syscall audit | ran | pass | 1s |
| audit-native | Compressor realtime allocation and syscall audit | ran | pass | 0s |
| audit-native | Parametric EQ realtime allocation and syscall audit | ran | pass | 0s |
| audit-native | Gate/expander realtime audit | ran | pass | 2s |
| audit-native | Builtins realtime audit (1,000,000 blocks, direct chain and graph) | ran | pass | 51s |
| audit-native | Graph realtime audit | ran | pass | 1s |
| audit-native | Protocol caller-buffer allocation audit | ran | pass | 0s |
| audit-native | Realtime audit hooks and syscall trace | ran | pass | 8s |
| audit-native | Builtins and builtins-graph audit probe mutation tests | ran | pass | 22s |
| audit-native | Native effect one-million-call allocation and syscall audit | ran | pass | 3s |
| audit-native | C ABI linkage, frozen symbol set, native consumer smoke test, and self-test | ran | pass | 38s |
| audit-native | Shipped C ABI library contains no whole-plan scalar path | ran | pass | 0s |
| audit-native | Graph fresh-process determinism (100 fresh processes) | ran | pass | 1s |
| audit-native | Builtins fixture corpus audit | ran | pass | 1s |
| audit-native | Standing console fixtures against the release session-validator | ran | pass | 1s |
| audit-native | Native effect runtime conformance bench | ran | pass | 0s |
| wasm-guests | Build the wasmtime runner | ran | pass | 201s |
| wasm-guests | SIMD128 compile probe (target-smoke and protocol) | ran | pass | 8s |
| wasm-guests | Evidence crates compile for Wasm (not an artifact) | ran | pass | 3s |
| wasm-guests | Issue-005 simd128 protocol golden parity | ran | pass | 17s |
| wasm-guests | WebAssembly cross-target digest gates | ran | pass | 22s |
| cross-target | Cross-target cargo checks (AArch64 iOS/Android product crates, parametric-eq, builtins, effect-compiler) | ran | pass | 350s |
| release-shape | Release shape policy | ran | pass | 0s |
| release-shape | Workspace release check under the unwind override | ran | pass | 46s |
| gate-self-tests | Environment and marker vocabulary mutation tests | ran | pass | 17s |
| gate-self-tests | Conformance boundary mutation tests | ran | pass | 67s |
| gate-self-tests | Console benchmark validator and runner mutation tests | ran | pass | 159s |
| gate-self-tests | SDK deletion gate self-test | ran | pass | 3s |
| gate-self-tests | DSP research gate mutation tests | ran | pass | 36s |

## Pre-existing failures that also show on `main`

None. No step failed, so no failure needed a comparison run on `main` `0a1176b3b`. The only
standing red I know of is not in `qualification.yml`: `npm-publish.yml`'s pin assertion refuses
on `main` (pin `6c952a2c...` and `EXPECTED_WORKLET_SHA256` `e18acf9c...` differ), which
`docs/RELEASE.md` records and which predates this batch.

## Logs

`/tmp/claude-1002/vbatch-a/logs/` (`<job>.log`, `results.tsv`, `identity-summary.md`,
`router.log`, `merged-13302fceb-module.log`), with the job scripts that ran the steps
(`/tmp/claude-1002/vbatch-a/job-*.sh`). I deleted the clone, the twin, the target dir and every
other build product. Nothing was committed or pushed.
