# Check the shipped AudioWorklet fingerprint at release; report changed/unchanged on every PR

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md` draft 15, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body.** **Owner ruling (2026-09-28, decision 5):** every PR still builds the shipped module, runs every artifact gate against the freshly built bytes, and reports whether the artifact changed; only release (publish) PRs must match a recorded fingerprint and carry a fresh three-browser qualification. Per the verification, the at-least-16 filed 'shipped artifact' gates must keep testing the exact freshly built bytes, and the per-PR report must be visible (for example a job summary).


Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §4.3 and §12 R3). Base `a9414c0c`. Paths starting
`../` are relative to the audit's handoff folder. **Needs owner
ruling R3.** The app repository's provenance file consumes the pin, so its purpose is release
integrity, and that is the owner's call.

## Problem

Every PR's `artifact` job refuses to build when the shipped module's sha256 differs from
`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`
(`scripts/build-web-audioworklet.sh:95-99`). Any code change moves the digest, even a doc comment,
because panic line numbers are embedded: `52d59c7c` had to re-pin after a change with no rendered
difference.

Three more records follow the pin:
- `hosts/host-web/qualification/results.json`'s `wasmSha256` and `candidateCommit`, which
  `--check-matrix` requires to match (`hosts/host-web/qualification/run.mjs:123-124`);
- the generated `BROWSER_DEPLOYMENT_MATRIX.md`;
- a literal copy, `REAL_WASM_SHA256`, in `scripts/test-web-audioworklet.mjs:153`. It is read only
  under `--real-wasm-receiver`, which no workflow or script runs, yet it moved in 11 commits.

From 2026-09-01 to 09-28 (`../data/ci-red-jobs.tsv`):
- **41 red jobs**: 20 on the sha256 pin and 21 on browser lineage;
- **98 distinct pin values**;
- **50-125 pin-only commits since 2026-08-28**, depending on whether specs and docs count;
- **no pin red exposed an unintended change**;
- in the one real kernel defect of the window (#920), the pin had *already* been updated to the
  defective artifact. The vector-kernel ratchet caught it, not the pin.

What the pin *does* prove: the build is reproducible across checkouts, and `npm-publish.yml`
publishes exactly the reviewed bytes (`npm-publish.yml:69-76`, `:160`).

## Outcome (if R3 is "publish-time")

- **PR runs** build the artifact, record its digest in the job summary, and upload it. They do not
  compare it to a committed pin.
- **Reproducibility becomes a same-run check.** A second job builds the artifact again, in parallel,
  under a different `CARGO_HOME` and checkout path, and `artifact-gates` requires identical bytes.
  This proves checkout-independence on every PR with no commit. It does not prove agreement with a
  build made on another machine or toolchain image; the committed pin, checked at publish, still
  covers that.
- **`npm-publish.yml` keeps its pin assertions unchanged.** The committed pin file is updated in the
  release PR only.
- **The downstream "Verify the downloaded artifact against its source pin" steps**
  (`qualification.yml:162`, `:203`, `:258`) compare against the digest the `artifact` job publishes
  as a job output, instead of the committed file.
- **The browser jobs keep their digest-parity gates** (native digest = browser digest). They drop the
  `results.json` lineage check. `BROWSER_DEPLOYMENT_MATRIX.md` is regenerated when Playwright or the
  browsers are bumped, not per artifact.
- **`REAL_WASM_SHA256` and the `--real-wasm-receiver` mode are deleted,** or the mode is wired into
  `artifact-gates`, reading the pin file.

## Scope

Authorized paths:
- `scripts/build-web-audioworklet.sh`;
- `.github/workflows/qualification.yml` (`artifact`, `sdk`, `artifact-gates`, `browser`);
- `hosts/host-web/qualification/run.mjs` (the lineage gate);
- `scripts/test-web-audioworklet.mjs`;
- `docs/` for the release procedure;
- this issue's spec.

`npm-publish.yml` is unchanged.

## Gates

1. **Reproducibility is still proven, and better.** The two-build check fails when a scratch branch
   embeds the absolute checkout path into the module, for example via `file!()` in a panic message
   with path remapping disabled. The failure appears in the PR that introduces it, with no pin to
   update.
2. **Publish integrity is unchanged.** `npm-publish.yml`'s `EXPECTED_WORKLET_SHA256` assertion still
   fails a publish whose built bytes differ from the pin file. Dry-run on a scratch branch.
3. **The browser parity is unchanged.** A one-bit PCM change in the browser render still fails each
   browser leg's native-digest gate.
4. **Historical catches kept.** Re-apply #920's scalarised `route_reduce<f32x4>` shape (revert
   `f7524db7`'s fix, or apply `graph/tests/MUTATIONS.md` 926-7). `artifact-gates`' vector ratchet
   still goes red.
5. **Churn.** Over the following 30 days, pin-only commits drop to release PRs only.

## Saving and risk

- **Saving:** about 41 red jobs a month and 50-125 pin-only commits. CI time rises by one parallel
  artifact build of about 100 s of runner time, off the critical path if it runs as its own job.
- **Risk:** a non-reproducible build is found in the same run instead of by pin drift, which is an
  improvement. The app's provenance file still receives the pin at release.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Adopt R3 only with a replacement for what the per-PR pin enforces (finding F3).**
   - **What the pin enforces today.** At least 16 filed specs (#1017-#1041) carry a
     "Shipped artifact" gate, "unchanged" or "re-pin with the reason". The pin makes those gates
     mechanical.
   - **Its other role.** It is also the only link between the bytes `qualification.yml` gates and
     the bytes `npm-publish.yml` publishes. `npm-publish.yml` runs none of the callgraph ratchet, the
     V8 spill gate, the atomics check or the browser legs.
   - **Replacement 1.** Every PR prints `ARTIFACT CHANGED|UNCHANGED` against the merge base's digest
     in the `artifact` job summary. Issue gates cite that line.
   - **Replacement 2.** The committed-pin comparison still runs on any PR that edits the pin file,
     which is the release PR. So the published bytes are the bytes that passed the gates.
2. **Scope additions:**
   - `scripts/check-ci-path-routing.py` pins the step name "Verify the downloaded artifact against
     its source pin" and its order before the V8 spill gate (`ARTIFACT_PIN_STEP`, `:334-353`);
   - `scripts/test-ci-path-routing.py`;
   - `scripts/test-sdk-artifact-builder-output-contract.sh`;
   - the V8 benchmark's `module_matches_pin` provenance (`scripts/web-mixing-automation-benchmark.mjs`,
     `run-web-mixing-automation-benchmark.sh:92`). Footprint R9 keeps that benchmark as the
     benchmark.
3. **New gate.** In a scratch branch, a change that alters the shipped bytes while claiming
   "no artifact change" prints `CHANGED` in its run. A release-PR pin that differs from the built
   bytes fails that PR, not only `npm-publish`.
4. **Confirmed.** 98 distinct pin values and 97 pin commits on `main` since 2026-09-01.

## Attempt 1 evidence

Terra, 2026-09-28, branch `codex/1061-artifact-fingerprint-at-release` from `codex/batch-slim-2`
(`3da5cae7`). Commits `2e548470` and `01ba58aa`. Excluding this spec: 19 files, +821/-950 lines
against the base, no Rust file. Of the +821, 386 are the new `scripts/web-audioworklet-identity.py`
(about half of it its self-test) and 79 are `docs/RELEASE.md`; of the -950, 818 are the deleted
`--real-wasm-receiver` mode. `npm-publish.yml` is unchanged.

**Changed.**
- **`scripts/build-web-audioworklet.sh`** prints `AudioWorklet module <sha256>` in every mode and
  compares it with the committed pin only under the new `--check-pin` (the release check, refused
  with nothing written). The default and `--module-only` modes never read the pin. The
  `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN` hook is retired (its vocabulary row goes, and
  `test-env-vocabulary.sh`'s documented-name count moves 97 -> 96).
- **`qualification.yml`.** `artifact` builds once, exports the module's digest as the job output
  `sha256`, and writes it to its summary. `sdk`, `artifact-gates` and each browser leg verify their
  download against `needs.artifact.outputs.sha256` ("Verify the downloaded artifact against the
  artifact job's digest"), before anything reads it. A new job, **`artifact-identity`** (after
  `artifact`, on its routes, parallel to the readers):
  1. rebuilds the same commit with `--module-only` from another checkout path
     (`$RUNNER_TEMP/twin`, a `git worktree`) and another `CARGO_HOME`;
  2. builds the base's module with the base's own build script;
  3. runs `web-audioworklet-identity.py --self-test`, then `report`, into its job summary.
  The verdict needs it and expects `success` wherever `artifact` runs. On the evidence route the
  verdict's summary says `ARTIFACT UNCHANGED: not built; ...` (no build reads those paths:
  `include_str!`/`include_bytes!` in `crates/`, `hosts/`, `tools/` name no documentation, and
  there is no `build.rs`).
- **`scripts/web-audioworklet-identity.py`** holds the rules and their self-test (five release
  mutations plus changed, unchanged, no base, not reproducible, the base rules and the CLI exit
  codes; eleven hand mutations of the script each turn its self-test red).
  - *Base.* A pull request run checks out GitHub's merge commit, so the base is its first parent:
    the tip of `main` it merges onto. Amendment 1 says "merge base"; a literal `git merge-base`
    would also count what `main` did to the module after the branch was cut, and report CHANGED
    for a PR that changed nothing. A `main` push compares with `github.event.before`. A
    `workflow_dispatch` run has no base and says `ARTIFACT BASE UNAVAILABLE`.
  - *Report.* The headline is `ARTIFACT CHANGED: <base digest> at base <commit> -> <digest> at
    <commit>` or `ARTIFACT UNCHANGED: <digest> at <commit> is the base <commit>'s module`.
  - *Reproducible.* The twin must equal the `artifact` job's digest, or the job fails
    (`NOT REPRODUCIBLE`).
  - *Release change.* A change that edits the pin file, or `npm-publish.yml`'s `PACKAGE_VERSION`
    or `EXPECTED_WORKLET_SHA256`, must have: pin == built digest; `EXPECTED_WORKLET_SHA256` == pin;
    `results.json` `wasmSha256` == built digest with a canonical `candidateCommit`. The last is the
    lineage gate moved out of `run.mjs`: a release must carry a fresh three-browser record of its
    bytes. The version trigger closes the gap where a publish PR bumps the version but forgets to
    re-pin. The same rules run on the merge push to `main`.
- **`hosts/host-web/qualification/run.mjs`** drops `validateLineage` and its two mutation proofs.
  `--check-matrix` still holds every browser row, the Playwright version and the generated matrix
  to `results.json`.
- **`scripts/test-web-audioworklet.mjs`** loses the `--real-wasm-receiver` mode, its
  `REAL_WASM_SHA256` copy of the pin and its now-unused imports (818 lines). No workflow or script
  ran it, and the three browser legs run the real module's lifecycle in real browsers. Wiring it into
  `artifact-gates` would have meant reviving an unrun issue harness, not keeping a gate.
- **V8 benchmark** (`run-web-mixing-automation-benchmark.sh prepare`). It builds through
  `build-web-audioworklet.sh --module-only` instead of its own copy of the recipe; the copy existed
  only because the delivery build used to refuse an unpinned module. `module_matches_pin` and
  `pinned_sha256` keep their schema and now mean "is the module the last release shipped". `prepare`
  and `preflight` ran on this branch: `host_web.wasm f7bd75ca... (release pin f7bd75ca...: the
  released module)`, preflight exit 0. `test-console-benchmark.sh` passes.
- **Policy.** `check-ci-path-routing.py`: `ARTIFACT_PIN_STEP` becomes `ARTIFACT_DIGEST_STEP`, still
  required before the V8 spill gate, plus two new rules.
  - `check_qualification_artifact_digest`: `artifact` exports `sha256`, and each reader needs
    `[route, artifact]` and verifies after it downloads and before any gate script reads the module.
  - `check_qualification_artifact_identity`: the identity job runs on exactly `artifact`'s
    routes, needs `[route, artifact]`, and keeps its twin build, self-test and report, in that
    order.
  `test-ci-path-routing.py` adds 13 mutations, each red: the output dropped; each reader's verify
  step deleted; the browser verify reverted to the pin file; each of the four identity lines;
  the twin sharing `CARGO_HOME`; the identity job narrowed to `full`; its `needs` without
  `artifact`; its expectation-table row dropped.
- **Release procedure:** `docs/RELEASE.md` (linked from `docs/README.md`,
  `hosts/host-web/DEPLOYMENT.md`, the build script's header, and the report's `RELEASE:` line). It states the six steps of a release PR (build and read the digest; re-pin; record a
  fresh three-browser qualification with `--record-matrix --candidate-commit`; set
  `npm-publish.yml`'s five version literals and `EXPECTED_WORKLET_SHA256`, and
  `test-npm-publish-modes.py`, `sdk/package*.json`, `sdk/README.md`; open the PR and read its
  `RELEASE:` lines; publish from `main`). It also says what runs on every other change.

**Records that stop being committed per PR:** the pin
`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`; `results.json`'s `wasmSha256`
and `candidateCommit`; the generated `BROWSER_DEPLOYMENT_MATRIX.md` (all three now change only in a
release PR, and the matrix also when Playwright or a browser floor changes); and `REAL_WASM_SHA256`,
which is deleted.

**Scope notes.** Beyond the authorized paths, this attempt touched only comments and counts that the
change made false: `scripts/run-wasm-gates.sh` (2 comments),
`scripts/check-web-audioworklet-v8-spill.py` (docstring), `scripts/test-env-vocabulary.sh` (the
count), `hosts/host-web/MUTATIONS.md` (the two lineage rows now name their new home). **Merge with
#1017** is mechanical: the verdict `needs:` line and its mutation in `test-ci-path-routing.py` gain
both sets of jobs; `test-env-vocabulary.sh`'s count is #1017's 70 minus one; the header comment's
job count ("thirteen leaf jobs") is left for that merge. #1048's G5 pairing and #1027's
reachability lint are untouched and green.

### Scenario runs

`qualification.yml`'s own `run:` blocks were executed locally, exactly as written, by a scratch
harness that supplies `RUNNER_TEMP`, `GITHUB_OUTPUT`, `GITHUB_STEP_SUMMARY` and the `${{ }}`
values, in an independent scratch clone (a different path from the worktree). Each scenario ran the
`artifact` job's build step, then every step of `artifact-identity`.

| scenario | commit (clone) | event, base | report | job |
|---|---|---|---|---|
| U: this change | `2e548470` | push, `3da5cae7` | `ARTIFACT UNCHANGED: f7bd75ca...`; reproducible; release not checked | pass |
| C: a "doc-only" note in `parametric-eq/src/lib.rs` (moves panic line numbers, as #1015's did) | merge `06500af2` | pull_request, first parent `2e548470` | `ARTIFACT CHANGED: f7bd75ca... -> d9bd9567...` | pass (reported) |
| N1: `black_box(env!("CARGO_MANIFEST_DIR"))` in an export | `1746a786` | push | CHANGED; **FAIL NOT REPRODUCIBLE** (`fb50b6ce...` vs twin `3959add3...`) | **red** |
| N2: build script drops the `CARGO_HOME` remap (the pre-#300 defect) | `c4abb318` | push | CHANGED; **FAIL NOT REPRODUCIBLE** (`0f406109...` vs `acf173cb...`) | **red** |
| no base | `2e548470` | push, unknown `before` | `ARTIFACT BASE UNAVAILABLE`; reproducible | pass |
| Rbad: pin set to 0.4.3's `e18acf9c...` | `8da2cdff` | push | RELEASE; **FAIL the pin is e18acf9c..., not the built f7bd75ca...** | **red** |
| Rgood: 0.4.4, `EXPECTED_WORKLET_SHA256` = pin = built | `f24ab3d6` | push | RELEASE; all four lines pass | pass |
| Rstale: C's bytes re-pinned in pin and `npm-publish.yml`, `results.json` not re-recorded | `cba8bfdd` | push | RELEASE; **FAIL artifact-lineage** | **red** |

**The per-PR report** is produced for an unchanged module (U) and a changed one (C, through the
pull-request merge path), and for a run without a base.

**Spec gates.**
1. *Reproducibility.* N1 (absolute path via `env!`) and N2 (remap dropped) fail in the change that
   introduces them, with no pin to update; U and C reproduce.
2. *Publish integrity.* `npm-publish.yml`'s two pin steps ("Assert the worklet sha256 pin..." and
   "Build ... prove its Linux pin"), run from each tree's own workflow with the same harness:
   - (a) this branch: step 1 refuses, `EXPECTED_WORKLET_SHA256 (e18acf9c...) has drifted from ...
     (f7bd75ca...)`. That was already so on the base: the pin moved per batch, `npm-publish.yml`
     per release;
   - (b) Rgood: both steps pass, `AudioWorklet module f7bd75ca...`;
   - (c) Rgood's release identity over C's tree: step 1 passes and the build step refuses
     `d9bd9567...` (exit 1).
   A mismatched pin still fails the publish.
3. *Browser parity.* Planted module `a48acd71...`: `hosts/host-web/src/lib.rs::render_next` flips
   the low bit of the first rendered sample. It is red in every leg at `native-corpus-digest:
   browser PCM differs from the native corpus pin` (chromium 151, firefox 153, webkit 26.5). It is
   also red in `direct-oracle.mjs` (`native and simd128 must render this session to identical
   bits`), which `check-browser-expected-resources.py --artifacts` runs.
4. *Historical catch.* #920's `route_reduce`/`route_tail` no longer exist in the source (the route
   kernels were rewritten since), so neither `f7524db7` (a re-pin commit, not a fix) nor mutation
   926-7 applies at this base. The equivalent plant scalarises a roster `f32x4` kernel, the
   soft-clip `Channel<f32x4>::process` mix/output arithmetic, lane by lane through `black_box`. A
   plain scalar loop was re-vectorised by LLVM and stayed green: `vector=29 scalar=0`. On the
   freshly built module (`e992d4c6...`), `check-web-audioworklet.sh` goes red with the rule that
   caught #920: `FAIL kernel ...soft_clip...4wide6f32x4...process: vector=20 scalar=20` and
   `FAIL roster soft-clip f32x4`. Clean: `vector=25 scalar=0`. Before this change the artifact
   job would have refused that module at the pin, and the ratchet would have run only after a
   re-pin.
5. *Churn.* Measurable only over the next 30 days. Nothing in a non-release PR writes the pin,
   `results.json` or the matrix any more.

Amendment 3: C is the scratch-branch change that alters the bytes while claiming no artifact change,
and it prints `CHANGED`. Rbad is a release-PR pin that differs from the built bytes, and it fails
that PR's run.

**The shipped-artifact gates of the filed specs.** #1021 g4, #1022 g3, #1023 g4, #1024 g3, #1025
g4, #1026 g3, #1027 g3, #1028 g3, #1030 g3, #1032 g3, #1033 g3, #1034 g3, #1035 g3, #1036 g3,
#1037 g3, #1038 g3, #1039 g3, #1040 g3, #1041 g2 and #1056 g3: twenty gates, each "unchanged",
"byte-identical", "unaffected", or "build base and change on one machine". Each now cites its PR's
`artifact-identity` line, which is exactly "build base and change on one machine" done by CI.
- **The line reads the freshly built bytes.** It compares the digest the `artifact` job built and
  every reader verified, never a committed value. C's plant turns it from UNCHANGED to CHANGED, so
  an "unchanged" claim over those bytes is refuted in its own run; U keeps it UNCHANGED.
  #1040-style documentation-only PRs get the evidence-route line.
- **The gates that read the module read the fresh bytes.** A module with one flipped byte
  (`81cba87c...`) is refused by the verify step of each of `sdk`, `artifact-gates` and the browser
  leg (`the artifact job built f7bd75ca..., got 81cba87c...`), before any gate reads it. Content
  plants in the built module turn the gates red: gate 3 (three browsers and the direct oracle),
  gate 4 (the kernel ratchet), N1/N2 (reproducibility).

### Gates run

- `actionlint` is not installed on this host. `qualification.yml` was checked with a PyYAML parse
  (job graph as intended), plus `check-ci-path-routing.py` and `test-ci-path-routing.py`, which
  pass. `npm-publish.yml` is unchanged; `test-npm-publish-modes.py` passes.
- `check-script-reachability.py` (156 files reached) and `test-script-reachability.py` (18 cases):
  pass.
- `check-web-audioworklet.sh` on the freshly built set: pass (15 kernels, every roster row ok).
  `test-web-audioworklet.sh`: pass.
- `test-sdk-artifact-builder-output-contract.sh`: pass. It now also proves that the default and
  `--module-only` modes write an unpinned module and print its digest, and that `--check-pin`
  refuses it, writes nothing, and accepts the pinned one.
- On the fresh set: `check-web-audioworklet-v8-spill.py`, `check-browser-expected-resources.py
  --artifacts` (26 red mutations), `check-sdk-generated.sh`, `check-sdk-headless.sh` (284 tests)
  and `sdk-package.sh check`: pass. `check-env-vocabulary.sh` (96 names) and
  `test-env-vocabulary.sh`: pass.
- Three-browser qualification in the PR mode that remains (`--check-matrix
  --self-test-mutations`, one leg per browser, each behind a private 48 kHz PulseAudio null sink as
  in CI): chromium 151.0.7922.34, firefox 153.0 and webkit 26.5 pass every gate on `f7bd75ca...`.
  The sink's socket must sit on a path under 108 bytes; from this host's scratch path, firefox's
  context stayed suspended until the socket moved.
- Every `scripts/check-*.sh` with no arguments: all 41 pass. Three
  (`check-effect-interchange-qualification.sh`, `-targets.sh`, `check-cross-targets.sh`) first
  failed on `sdk/dist/assets/*.wasm`, which this attempt's own `sdk-package.sh check` had left in
  the worktree; they pass once it was removed. Every `check-*.py` that takes no arguments: all
  pass. `check-abi-layout-v1.py`, `check-builtins-listening-033.py`/`-111.py`,
  `check-parameter-metadata-v1.py`, `check-web-audioworklet-callgraph.py` and
  `check-web-audioworklet-v8-spill.py` require arguments and ran where their callers pass them.
- `scripts/run-wasm-gates.sh` (all legs, 431 s): pass, `wasm gates: ok (native + wasm scalar + wasm
  simd128 + V8 EQ loops)`. Its V8 leg built the module through `--module-only` (`f7bd75ca...`).
- `cargo fmt --all --check`: pass. No Rust changed, so clippy was not rerun.

## Sol verdict, attempt 1

**FAIL.** One defect: the per-PR report is false for a toolchain change. Release integrity, the
fresh-bytes gates, reproducibility and every gate on the merge hold. The fix is one step.

Reviewer: Sol, 2026-09-28, on `77048ab7`, merged into a scratch detached checkout of
`codex/batch-slim-2` at `8b3c0794` (merge `7625b8c6`). The workflow's own `run:` blocks were run by a
scratch harness that supplies `RUNNER_TEMP`, `GITHUB_OUTPUT`, `GITHUB_STEP_SUMMARY` and the
`${{ }}` values, in a clone at a different path from the merge.

### Findings, by severity

1. **HIGH: the base module is built with the PR's toolchain, so a toolchain change reports
   `ARTIFACT UNCHANGED`.** `qualification.yml` sets `RUSTUP_TOOLCHAIN` for the whole workflow, and
   the "Build the base's module" step inherits it. rustup's environment override beats the base
   worktree's `rust-toolchain.toml`. So the base's source is rebuilt with the toolchain the PR
   chose, not the one `main` shipped with.
   - Reproduced: a scratch PR on `7625b8c6` changes 1.97.1 to 1.98.1 in `qualification.yml` and
     `rust-toolchain.toml`, with no source change.
     - `artifact` built `749ffb71...`, and the twin agreed.
     - The base was rebuilt with 1.98.1 and gave `749ffb71...`. Its real module, as `main`'s CI
       builds it, is `e34073a2...`.
     - The job passed, and its summary said `ARTIFACT UNCHANGED: 749ffb71... at 4b4188f8... is the
       base 7625b8c6...'s module`. That is false.
   - A toolchain bump changes the shipped bytes with no source diff, so the report is the only
     place that change can show. The per-PR pin used to catch it.
   - Fix: build the base with the base's own toolchain. Read it from the base's `qualification.yml`
     `RUSTUP_TOOLCHAIN` or its `rust-toolchain.toml`, install it with the `wasm32` target, and set
     it for that step. Then pin that step in `check-ci-path-routing.py` and plant this scenario.
2. **MEDIUM: the policy lint does not pin the base wiring, so the release check can disappear with
   CI green.** `IDENTITY_LINES` pins the twin build, the self-test and the report, but not the base
   step or the `base_args` line. On a copy of the merge, each of these mutants passes
   `check-ci-path-routing.py`:
   - the `[[ -z "$BASE" ]] || base_args=(...)` line deleted;
   - `--event "$EVENT"` replaced by `--event none`;
   - `BEFORE:` emptied;
   - `fetch-depth: 0` dropped;
   - a twin symlinked to the workspace.

   With any of the first four, a `main` push, or every run, says `ARTIFACT BASE UNAVAILABLE` and
   skips the release checks. `npm-publish.yml` still refuses mismatched bytes, but the
   fresh-qualification requirement would be gone. Pin the base step, its env and the `base_args`
   line.
3. **LOW: with no base, the report makes a false statement.** It says "this change does not edit
   the pin or `npm-publish.yml`'s release identity" even when the change does. A `workflow_dispatch`
   run over a release commit shows it. The line should say that the release was not checked because
   there is no base.
4. **LOW, docs:** `docs/RELEASE.md`, `hosts/host-web/DEPLOYMENT.md` and the report's "pinned release
   module" wording say that the pin, `results.json` and the matrix describe the last release.
   - They do not. On the batch, `origin/main` and `8b3c0794`, the pin is `f7bd75ca...`, the last
     per-PR pin. 0.4.3's `EXPECTED_WORKLET_SHA256` is `e18acf9c...`. So `npm-publish.yml`'s pin
     assertion already refuses on `main`; that is pre-existing and the same on base.
   - Say so until the first release PR under #1061.
   - Step 3 should also note the PulseAudio socket's 108-byte path limit, which the attempt
     itself hit.
5. **LOW, cosmetic:** the header comment says "thirteen leaf jobs"; with `artifact-identity` there
   are 14.
   - With #1017 merged there are 16, and #1017's header says "fifteen".
   - The evidence says the env count would go from 70 to 69 on a #1017 merge. That is stale: on
     this batch the count auto-merges to 67, which is correct.

### Merges

- **#1061 into `8b3c0794`** has two textual conflicts:
  - `docs/README.md`: #1037 removed the three effect-package rows; keep only the `RELEASE.md` row.
  - `scripts/test-env-vocabulary.sh`: the count is 68 on the batch, and 67 after #1061's
    retirement. `check-env-vocabulary.sh` reports 67 and `test-env-vocabulary.sh` passes.
- **No semantic conflict with the batch.** The batch's pin equals `origin/main`'s (`f7bd75ca...`,
  after the slim-1 batch merged as `5379e46c`), so the batch's PR to `main` is not a release change.
  - Scenario S1 ran it: a synthetic merge of the batch into `5379e46c`, as a `pull_request` run.
  - This is also the first run: the base module was built by the pre-#1061 script's
    `--module-only`.
  - The job passed with `ARTIFACT CHANGED: f7bd75ca... -> e34073a2...`, which is correct, and the
    twin reproduced.
- **#1017 (`f3644ff6`) on top** conflicts only in two places, both mechanical unions:
  - the verdict's `needs:` list in `qualification.yml`;
  - `test-ci-path-routing.py`: the `needs` mutation, plus both new mutation blocks.

  After resolving them, `check-ci-path-routing.py` and `test-ci-path-routing.py` pass, and
  `check-env-vocabulary.sh` (67) and `test-env-vocabulary.sh` pass. #1017's branch was not
  edited.

### The questions

1. **Release integrity holds.**
   - **What `npm-publish.yml` accepts.** It is `workflow_dispatch` only, from `main`, with
     `expected_sha` in `origin/main`. Its checks:
     - `qualify` refuses unless `EXPECTED_WORKLET_SHA256` is the pin file;
     - it rebuilds, and refuses bytes other than `EXPECTED_WORKLET_SHA256`;
     - `publish` sends only that run's archive.
   - **Why the published bytes are reviewed bytes.** A release PR edits the pin or
     `EXPECTED_WORKLET_SHA256`. The pin path routes `full`, and `artifact-identity` holds the
     change to pin == built == `EXPECTED_WORKLET_SHA256` and `results.json` == built, on the PR and
     on the merge push. That same run's `artifact-gates` and browser legs read those bytes.
   - **A change that moved the artifact but not the pin cannot publish.** It is stopped at
     `qualify`'s build step. Re-pinning it is a release change, which needs a fresh
     `results.json`.
   - **Re-run.**
     - R1: the version is bumped and `EXPECTED_WORKLET_SHA256` is set to the stale pin while the
       tree builds `e34073a2...`. `artifact-identity` fails on the pin and on lineage. In
       `npm-publish.yml` the pin assertion passes, and "Build ... prove its Linux pin" exits 1.
     - R2: the whole release identity for `e34073a2...`. Every check passes, and both
       `npm-publish.yml` steps pass.
2. **Every artifact gate reads the freshly built bytes.** `artifact` builds once and exports the
   digest. Each reader checks its download against that digest before any gate script runs:
   - `sdk`;
   - `artifact-gates`: `check-web-audioworklet.sh`, `check-browser-expected-resources.py
     --artifacts` and the V8 spill gate;
   - the three browser legs.

   The filed "shipped artifact" gates cite `artifact-identity`, which compares that same digest
   with a fresh build of the base: #1021 g4, #1022 g3, #1023 g4, #1024 g3, #1025 g4, #1026 g3,
   #1027 g3, #1028 g3, #1030 g3, #1032 g3, #1033 g3, #1034 g3, #1035 g3, #1036 g3, #1037 g3,
   #1038 g3, #1040 g3, #1041 g2 and #1056 g3. None of them changes the toolchain, so finding 1
   does not reach them.

   Plants re-run here:
   - **A one-bit PCM flip** of the first rendered sample in `render_next` (`758e3dc3...`): the
     chromium leg is red at `native-corpus-digest: browser PCM differs from the native corpus pin`.
   - **Soft-clip's mix and output arithmetic, scalarised lane by lane**, with `black_box` on every
     intermediate (`12fb3b06...`): `check-web-audioworklet.sh` is red at `vector=20 scalar=20` in
     both the kernel rule and the roster rule. A weaker version, with `black_box` on the inputs
     only, moved the digest, but LLVM re-vectorised it (`vector=25 scalar=0`, by disassembly too).
     Its green was correct.
   - **A flipped download byte**: the verify step of `sdk`, `artifact-gates` and `browser` each
     exit 1. An empty `BUILT` also exits 1.
3. **The report is visible and its base is right.**
   - **Visible.** `artifact` writes the digest to its summary, `artifact-identity` tees the report
     into its summary even when it fails, and the verdict writes the evidence-route line.
   - **Correct outside the toolchain case.**
     - S1 is CHANGED, correctly.
     - U, a comment-only PR, is `ARTIFACT UNCHANGED: e34073a2...`, correctly.
     - No evidence-route path is read by a build: no `include_str!` or `include_bytes!` names one,
       and there is no `build.rs`.
   - **The base.** It is the first parent of GitHub's merge commit for `pull_request`, and
     `event.before` for a `main` push. `merge_group` is not a trigger. `workflow_dispatch` says
     `ARTIFACT BASE UNAVAILABLE` and stays green.
   - **The first run** builds the base from source with the base's own script, so it needs no base
     artifact. S1 shows no spurious failure.
4. **Reproducibility.**
   - The twin is a `git worktree` at `$RUNNER_TEMP/twin` with a fresh
     `CARGO_HOME=$RUNNER_TEMP/twin-cargo-home`. In S1, U and TC it downloaded the crates again and
     reproduced from another path.
   - Plant: `black_box(env!("CARGO_MANIFEST_DIR").as_ptr())` in an export. `artifact` built
     `f9a2ed16...` and the twin `2d946f52...`, so the job fails with `NOT REPRODUCIBLE`.
5. **Wiring.**
   - `artifact-identity` is in the verdict's `needs` and its expectation table (`artifact_expected`).
     It is gated by the router's `route`, with no `paths:` filter.
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - `check-script-reachability.py` (134 files) and `test-script-reachability.py` (18 cases) pass.
   - actionlint 1.7.7 passes on `qualification.yml` and `npm-publish.yml`. With shellcheck 0.10.0,
     the same four pre-existing warnings appear on base and merge, and none in the new blocks.
   - Vacuous-pass gaps: finding 2.
6. **`docs/RELEASE.md`** gives a correct pin, qualification record and matrix.
   - Its step 4 literal list matches the repository: five in `npm-publish.yml`; `VERSION`,
     `release_pin` and its mutation in `test-npm-publish-modes.py`; `sdk/package*.json` and
     `sdk/README.md`.
   - Gaps: finding 4.
7. **Gates on the merge: all pass, so there is no base comparison to make.**
   - `check-web-audioworklet.sh` and `test-web-audioworklet.sh`.
   - `check-browser-expected-resources.py --artifacts`: 26 red mutations.
   - The V8 spill gate: its self-test and the module.
   - `test-sdk-artifact-builder-output-contract.sh`, `test-npm-publish-modes.py` and
     `cargo fmt --check`.
   - The identity self-test.
   - All 35 `scripts/check-*.sh`.
   - Every argument-free `check-*.py` (10). Six take arguments and ran through their callers.
   - `run-wasm-gates.sh`, 320 s. Its V8 leg built `e34073a2...`.
   - The three-browser PR mode (`--check-matrix --self-test-mutations`) on `e34073a2...`: chromium
     151.0.7922.34, firefox 153.0 and webkit 26.5.

## Attempt 2 evidence

Terra, 2026-09-28, same branch.
- `665ae26e` merges the batch head `codex/batch-slim-2` (`c69736c1`: #1037, #1063, #1035, #1034,
  #1036, #1017).
- `20ffabf0` fixes the verdict's findings: 6 files, +317/-61.
- Against the batch head, excluding specs, the branch is now 19 files, +1079/-952, with no Rust
  file.

**Merge.** Four conflicts, each mechanical:
- the verdict's `needs:` line carries `artifact-identity` and #1017's two AArch64 jobs;
- `test-ci-path-routing.py` gets the same union in its `needs` mutation and keeps both new mutation
  blocks;
- `docs/README.md` keeps only the `RELEASE.md` row after #1037's removals;
- `test-env-vocabulary.sh` gets 67, the batch's 68 less the retired REPIN hook.
  `check-env-vocabulary.sh` reports 67 names. Attempt 1's "70 -> 69" prediction was stale, as Sol
  noted.

The header comment now reads "sixteen leaf jobs (#1017 added the two AArch64 jobs, #1061
`artifact-identity`)" (finding 5); the workflow parses to 16 leaves.

**Finding 1 (HIGH), the base toolchain.**
- *The step.* The base step, renamed "Build the base's module with the base's own build script and
  toolchain", now runs `web-audioworklet-identity.py toolchain --commit "$base"`. It then installs
  that toolchain with the `wasm32` target and builds the base with `RUSTUP_TOOLCHAIN="$toolchain"`.
- *Where the toolchain comes from.* `toolchain` reads the base commit's own
  `.github/workflows/qualification.yml` workflow-level `RUSTUP_TOOLCHAIN`: the value the base's
  `artifact` job built with, which overrides `rust-toolchain.toml` in CI. Failing that, it reads the
  base's `rust-toolchain.toml` channel. A job-level `RUSTUP_TOOLCHAIN` never counts. A missing or
  malformed pin fails the step instead of guessing.
- *The report* names both toolchains, and flags "(a toolchain change)" when they differ.
- *Plant, run through both jobs' own `run:` blocks* (the attempt 1 harness, in a scratch clone at
  another path):
  - A push commit `5067d9b3` changes every `1.97.1` in `qualification.yml` and
    `rust-toolchain.toml` to `1.98.1`, with no source change, over base `20ffabf0`.
  - `artifact` built `b7dfd909...`, and the twin agreed.
  - The base step read `toolchain=1.97.1` and built `6c952a2c...`, the base's real module.
  - The report says `ARTIFACT CHANGED: 6c952a2c... at base 20ffabf0... -> b7dfd909... at
    5067d9b3...`, then "Toolchains: this commit built with Rust `1.98.1`, the base with its own
    pinned `1.97.1` (a toolchain change)". The job passes.
- *Control.* The same base built with the PR's `1.98.1`, attempt 1's behaviour, gives `b7dfd909...`
  exactly, which would have printed `ARTIFACT UNCHANGED`.
- *Unchanged run.* A push of `20ffabf0` over `665ae26e` is `ARTIFACT UNCHANGED: 6c952a2c...`, with
  both toolchains `1.97.1`.
- *Committed tests.*
  - The identity self-test adds a toolchain-only change. The base's toolchain is read as `1.97.1`
    while `HEAD` pins `1.98.1`, and the report is CHANGED with the toolchain line. It also covers the
    `rust-toolchain.toml` fallback, a job-level value that must not count, a missing pin and a
    malformed pin, and the CLI.
  - `test-ci-path-routing.py` goes red when the base build drops `RUSTUP_TOOLCHAIN="$toolchain"`,
    reads `toolchain --commit HEAD`, or feeds `BASE_TOOLCHAIN` from `env.RUSTUP_TOOLCHAIN`.
  - Nine hand mutations of the new identity rules each turn the self-test red:
    - the toolchain read from `HEAD`;
    - the channel read from `HEAD`;
    - `rust-toolchain.toml` winning over the workflow;
    - any-level `RUSTUP_TOOLCHAIN` counting;
    - a missing pin guessed;
    - a malformed name accepted;
    - the no-base wording reverted;
    - the toolchain flag dropped;
    - the base toolchain made optional.

**Finding 2 (MEDIUM), the base wiring.**
- *The rule.* `check_qualification_artifact_identity` now splits the job into steps, ignoring
  comment and blank lines, and requires exactly five of them:
  1. the checkout, with `fetch-depth: 0`;
  2. the toolchain install (free, so a toolchain bump edits only the workflow);
  3. `IDENTITY_TWIN_STEP`, exactly;
  4. `IDENTITY_BASE_STEP`, exactly;
  5. `IDENTITY_REPORT_STEP`, exactly.
- *Mutants.* `test-ci-path-routing.py` adds these, each red:
  - Sol's five: the `base_args` line deleted; `--event none`; `BEFORE: ""`; the identity job's
    `fetch-depth: 0` dropped; the twin replaced by `ln -s "$GITHUB_WORKSPACE" "$RUNNER_TEMP/twin"`.
  - The three toolchain mutants above.
  - Each pinned step deleted.
  - A step inserted before the report.
- *Result.* `check-ci-path-routing.py` and `test-ci-path-routing.py` pass on the merge.

**Finding 3 (LOW), the no-base wording.**
- With no base, the report now says "Release fingerprint not checked: with no base, this run cannot
  tell whether the change edits the pin or `npm-publish.yml`'s release identity".
- It never says "does not edit". The self-test holds this over a release commit, and a
  `report` call with no base on a scratch re-pin commit (`3c27d044`) prints it.

**Finding 4 (LOW), the docs.**
- *`docs/RELEASE.md` "Between releases"* now says what the pin is until the first release PR
  under #1061:
  - it is `f7bd75ca...`, the last per-change re-pin, from slim-1 (`570a79f0`), and `results.json`
    and the matrix record that module;
  - release 0.4.3 shipped `e18acf9c...`;
  - so `npm-publish.yml`'s pin assertion refuses on `main` today, which predates #1061.
- *Step 2* says a release PR sets the pin to the digest of the release commit's own tree, built
  with the toolchain its workflow pins, and puts the same digest in `EXPECTED_WORKLET_SHA256`.
- *Step 3* notes the PulseAudio socket's 108-byte path limit.
- *Elsewhere.* `hosts/host-web/DEPLOYMENT.md` says the same, and the report says "the committed
  pin" / "the pinned module", not "the pinned release module".

**Gates rerun on the merged branch.**
- **actionlint.** Not available here: not on PATH, no Go toolchain, and the Docker socket is
  refused. A PyYAML parse of `qualification.yml` passes; Sol ran actionlint 1.7.7 on attempt 1's
  workflow.
- **The workflow's own contract.**
  - `check-ci-path-routing.py` and `test-ci-path-routing.py`: pass.
  - `check-script-reachability.py` (137 files) and `test-script-reachability.py` (18 cases): pass.
- **On the freshly built module `6c952a2c...`.**
  - `check-web-audioworklet.sh` passes, with 11 roster rows ok; `test-web-audioworklet.sh` passes.
  - `test-sdk-artifact-builder-output-contract.sh` passes.
  - The V8 spill gate passes on the module, and its self-test passes (19 cases); the call-graph
    self-test passes.
- **Every Python policy script under `python3 -B`.**
  - The argument-free `check-*.py` all pass: `check-browser-expected-resources.py` (26
    mutations), `-ci-path-routing`, `-command-kind-vocabulary`, `-command-reason-vocabulary`,
    `-release-shape`, `-script-reachability`, `-sdk-deletions`, `-session-map-shape`,
    `-step-vocabulary`, `-test-support-ci`.
  - Every `test-*.py` passes: `test-ci-path-routing`, `test-npm-publish-modes`,
    `test-script-reachability`, `test-test-support-ci`.
  - The six that need arguments (`check-abi-layout-v1.py`, `check-builtins-listening-033.py`,
    `-111.py`, `check-parameter-metadata-v1.py`, the call-graph and V8 spill gates) exit with
    their usage message. They ran through their callers or with their arguments above.
- **The rest.** `web-audioworklet-identity.py --self-test` passes; `check-env-vocabulary.sh` (67)
  and `test-env-vocabulary.sh` pass; `cargo fmt --all --check` passes.

## Sol verdict, attempt 2

**FAIL.** Attempt 1's toolchain case is fixed, but its root cause is not. The base module is still
rebuilt in the PR's build environment. So a workflow-level cargo variable, which the checker lets
through, reports `ARTIFACT UNCHANGED` while the module CI built has moved. Everything else holds.

Reviewer: Sol, 2026-09-28, on `d0e55daf`.
- Scenarios are synthetic `pull_request` merge commits onto `d0e55daf`. They run through the
  workflow's own `run:` blocks, with workflow-, job- and step-level env, in a clone at another path.
- Merges were checked against `codex/batch-slim-2` at `6709552c`, which carries the slim-2 re-pin.

### Findings, by severity

1. **HIGH: the base is still built in the PR's environment, so a build-affecting variable gives a
   false `ARTIFACT UNCHANGED`.**
   - Only `RUSTUP_TOOLCHAIN` is taken from the base. Everything else the PR's workflow sets reaches
     the base build, because cargo reads it from the environment. That includes `CARGO_PROFILE_*`,
     `CARGO_ENCODED_RUSTFLAGS` (which overrides the build script's `RUSTFLAGS`), `GITHUB_ENV` and
     `GITHUB_PATH`.
   - **T5.** The PR adds one line to the workflow-level `env:`,
     `CARGO_PROFILE_RELEASE_OPT_LEVEL: s`. `check-ci-path-routing.py` stays green.
     - `artifact` built `6d9e8945...`, where the base's CI built `6c952a2c...`. The twin agreed.
     - The base was rebuilt under the same variable, gave `6d9e8945...`, and the job passed:
       `ARTIFACT UNCHANGED: 6d9e8945... is the base d0e55daf...'s module`. That is false.
     - From then on every artifact gate tests bytes that `npm-publish.yml`, which has no such
       variable, does not build. The next release PR fails only at `qualify`. The per-PR pin
       caught this at once.
   - **Job-level toolchain overrides.** A job-level `RUSTUP_TOOLCHAIN` on `artifact` is allowed
     (T2), but `toolchain` never reads it. Once one merges, every later PR rebuilds the base with
     the workflow-level value and reports a false `ARTIFACT CHANGED`. `toolchain --commit` over
     T2's merge prints `1.97.1`, while that commit's `artifact` job built with `1.98.1`.
   - **A deliberate route.** The toolchain install step is not pinned. One line in it,
     `echo "$(dirname "$(rustup +1.98.1 which cargo)")" >> "$GITHUB_PATH"`, keeps the checker
     green and builds the base with 1.98.1 despite `RUSTUP_TOOLCHAIN=1.97.1`. Measured: `cargo`
     and `rustc` resolve to 1.98.1 under that `PATH`.
   - **Fix: pin the build environment in the checker, not only the steps.**
     - The workflow-level `env:` allows exactly `CARGO_TERM_COLOR` and `RUSTUP_TOOLCHAIN`.
     - `artifact` and `artifact-identity` carry no job-level `env:` or `defaults:`.
     - The install step's body is the two `rustup` lines, and their version equals the
       workflow's `RUSTUP_TOOLCHAIN`.
     - Plant T5, a job-level override and a `GITHUB_PATH` or `GITHUB_ENV` write in the install
       step; each must go red.
2. **LOW: the release check does not tie `npm-publish.yml`'s `RUSTUP_TOOLCHAIN` to
   `qualification.yml`'s.**
   - After a toolchain bump in `qualification.yml` alone (T1), a release PR passes
     `artifact-identity`, then `qualify` builds with 1.97.1 and refuses.
   - That fails safe, but `docs/RELEASE.md` step 4 should name that literal, or `release_checks`
     should compare the two.
3. **LOW: the docs will be stale after the slim-2 re-pin.** `docs/RELEASE.md` ("The pin is
   `f7bd75ca...` ... `570a79f0`") and `hosts/host-web/DEPLOYMENT.md` name the slim-1 pin. The
   batch-2 boundary (`cbfaf9de`) re-pins to `6c952a2c...`, with `results.json` over `cbfaf9de`.
   Update the literals when #1061 merges into batch 3, or stop naming them.
4. **Pre-existing, not #1061: vacuous-pass classes the routing checker misses for every job.** Each
   of these is green on `c69736c1` for `sdk`, and green on this branch for `artifact-identity`:
   - a job-level `continue-on-error: true`;
   - `defaults.run.shell: bash {0}`, which masks the identity self-test's exit;
   - a verdict env hard-coded to `success`;
   - an expectation-table entry swapped.

   These belong in a follow-up issue.

### Attempt 1's findings

- **HIGH (toolchain): fixed for every toolchain route I tried.**
  - **T1**, workflow-level `RUSTUP_TOOLCHAIN` 1.97.1 to 1.98.1 only: the base read `1.97.1` from
    the base commit and built `6c952a2c...`; `artifact` and the twin built `b7dfd909...`. The report
    is `ARTIFACT CHANGED`, with "(a toolchain change)".
  - **T2**, job-level `RUSTUP_TOOLCHAIN: 1.98.1` on `artifact` and `artifact-identity` only: the
    same result, CHANGED.
  - **`rust-toolchain.toml` only**: the workflow `RUSTUP_TOOLCHAIN` beats the file (`rustc -V` in
    a 1.98.1 `rust-toolchain.toml` directory gives 1.97.1 under the env). So neither CI nor
    `npm-publish.yml` builds differently, and `UNCHANGED` is right.
  - **Components and targets**: these do not change codegen.
  - **The base toolchain comes from the base commit.** `pinned_toolchain` reads it through
    `git show BASE:`. The base step's `RUSTUP_TOOLCHAIN="$toolchain"` is pinned, and it beats job
    env and `GITHUB_ENV`. Only the `GITHUB_PATH` route in finding 1 gets past it.
- **MEDIUM (lint): fixed.** My mutants, run on the new checker:
  - `base_args` deleted: killed;
  - `--event none`: killed;
  - `fetch-depth` dropped: killed;
  - my new one, the identity job needing `[route, artifact, sdk]`: killed.

  Job-level `continue-on-error` and `defaults` survive; that is finding 4.
- **LOW (no-base wording, docs, header): fixed**, apart from finding 3.

### Merges

- **`665ae26e` (batch-2 head `c69736c1`).** Against `c69736c1`, `qualification.yml` differs only in
  #1061's hunks. These survive intact:
  - #1017's `aarch64-debug` and `aarch64-release` jobs;
  - their verdict `needs:`, env and `check` lines (`full_expected`);
  - `cross-target`'s mobile targets.

  The routing scripts lose only the old `ARTIFACT_PIN_STEP` and the old `needs:` mutation string.
- **`d0e55daf` into `6709552c` (the slim-2 re-pin).** One conflict, in
  `scripts/test-web-audioworklet.mjs`: `cbfaf9de` re-pinned `REAL_WASM_SHA256` inside the block
  #1061 deletes.
  - Taking #1061's side leaves the file identical to the branch's.
  - The pin and `results.json` merge cleanly and keep the batch's `6c952a2c...` over `cbfaf9de`.
  - The module built on the merge is `6c952a2c...`.

### Gates

On the re-pin merge, all of these pass:
- `check-web-audioworklet.sh` on the freshly built set, `test-web-audioworklet.sh`,
  `test-web-audioworklet.mjs`, `test-sdk-artifact-builder-output-contract.sh`, and the V8 spill
  gate on the module;
- `check-env-vocabulary.sh` (67);
- every `check-*.py` and `test-*.py` under `python3 -B`, including `check-ci-path-routing.py`,
  `test-ci-path-routing.py`, `check-script-reachability.py` (137), `test-script-reachability.py`,
  `check-browser-expected-resources.py` (26 mutations), `test-npm-publish-modes.py` and
  `test-test-support-ci.py`. The six scripts that need arguments exit with their usage;
- the identity self-test;
- actionlint 1.7.7 on `qualification.yml` and `npm-publish.yml`, on the branch and on the merge.
  With shellcheck 0.10.0 the warnings are the same four (SC2034 once, SC2251 three times) as on
  `c69736c1`, and none come from the #1061 blocks.

## Attempt 3 evidence

Terra, 2026-09-28, same branch.
- Commits: `c7331f34` (the redesign) and `646758ba` (the job-level `env` rationale and its mutant).
- Attempt 3: 7 files, +609/-384, most of it the rewritten `web-audioworklet-identity.py`.
- Against the batch head `c69736c1`, excluding specs, the branch is 19 files, +1304/-952, with no
  Rust file.

**The redesign (root's direction on Sol's attempt 2 HIGH).** A change's run no longer rebuilds its
base. Any base rebuilt in the change's own workflow is built in the environment the change sets:
its toolchain, a cargo variable such as T5's `CARGO_PROFILE_RELEASE_OPT_LEVEL`, a `GITHUB_PATH`
line. The comparison is now with what the base's own CI run built and recorded.
- **The record, on `main`.** The `artifact` job's build step also outputs the rustc release it built
  with (`rustc -vV`, the same environment as the build). On a push to `main` only, a last step
  (`web-audioworklet-identity.py record`) posts the commit status `audioworklet-sha256` on the
  pushed commit:
  - its description is `<sha256> rustc <release>`, and its target URL is the run;
  - job permissions are `contents: read, statuses: write`;
  - a commit status does not expire, and the context is not `qualification`, so it cannot satisfy
    the required check.
- **The compare, on every change.** `artifact-identity` is four steps: checkout (full history),
  toolchain install, the unchanged twin build, and one report step. That step runs the self-test,
  then `report`.
  - `report` finds the base: the merge commit's first parent for a PR, `--before` for a push.
  - It reads the base's combined status (`gh api repos/R/commits/BASE/status`, permissions
    `contents: read, statuses: read`).
  - It prints `ARTIFACT CHANGED`, `ARTIFACT UNCHANGED`, or `ARTIFACT CANNOT TELL` with the reason:
    no base; the base's run has not recorded yet (rerun the job); it never built the module; or it
    predates records.
  - A base with no record uses its nearest recorded first-parent ancestor, up to 20 back, when
    everything between them is in the router's evidence class, so documentation-only pushes do
    not break the chain. CANNOT TELL never fails the job.
  - A refused or malformed API response fails it (exit 2): a lookup that can never succeed is
    misconfiguration, not a missing record.
- **What was removed.** The base rebuild step, the `base` and `toolchain` subcommands, and the base
  toolchain logic.
- **The release side is unchanged.** Pin, `EXPECTED_WORKLET_SHA256`, and `results.json` lineage
  are still required to describe the built bytes, as before.

**LOWs.**
- **npm-publish's toolchain.** A release change must now have `npm-publish.yml`'s
  `RUSTUP_TOOLCHAIN` equal to the rustc release the `artifact` job built with. Sol's T1, a
  toolchain bump in `qualification.yml` alone, now fails the release PR instead of `qualify`.
  `docs/RELEASE.md` step 4 says so. Any other build-environment difference between the two
  workflows can only fail `qualify`, which rebuilds and refuses any bytes but
  `EXPECTED_WORKLET_SHA256`, so it never publishes unreviewed bytes.
- **Stale hashes.** `docs/RELEASE.md` and `hosts/host-web/DEPLOYMENT.md` no longer name a digest.
  Until the first release PR under #1061 they say the pin holds the last batch-boundary re-pin
  (`git log -1 -- <pin>` names it), not 0.4.3's module, which is `npm-publish.yml`'s
  `EXPECTED_WORKLET_SHA256`.

**What the checker pins now.** Only what the comparison rests on:
- the record step, exactly, including its push-to-`main` `if:`, as the `artifact` job's last step;
- the build step's `sha256=` and `rustc=` output lines, and both job outputs;
- both jobs' permission blocks;
- `artifact-identity`'s route (the same as `artifact`'s), its `needs`, full history, and its twin
  and report steps, exactly. The report step carries no `if:`;
- neither job may carry a job-level `continue-on-error`, `defaults` (`shell: bash {0}` masks a
  failed self-test) or `env` (a job `PATH` could put a forged `gh` first).

The build environment is deliberately not pinned. The checker stays green on T5, and T5 now reads
CHANGED, as the dry run below shows.

`test-ci-path-routing.py` passes with these mutants, each red:
- *Record side:* the record step deleted; its `if:` made `pull_request`; `statuses: write` dropped;
  the `sha256=` line and the `rustc=` line each dropped; the `rustc` output dropped; a job-level
  `env: PATH` on `artifact`.
- *Compare side:*
  - the report and twin steps each deleted;
  - the report skipped by `if: github.event_name == 'push'`;
  - `--event none`; `BEFORE: ""`; `GH_TOKEN` dropped; the self-test dropped;
  - `statuses: read` dropped;
  - a job-level `continue-on-error: true`, `defaults: run: shell: bash {0}` and `env` on
    `artifact-identity`;
  - a step inserted before the report; full history dropped;
  - the twin symlinked to the workspace; the twin sharing `CARGO_HOME`.

I checked the failure message of eight of them by hand.

**Tests.**
- **The compare script's self-test.** A scratch repository and a fake `gh` on `PATH` that answers
  from canned combined-status JSON and logs its arguments. It covers:
  - a matching record (UNCHANGED), a different one (CHANGED), and a toolchain-only change
    (CHANGED, flagged);
  - a missing record, and a malformed one (CANNOT TELL, not failing);
  - a refused lookup (exit 2, `HTTP 403` passed through);
  - the documentation-only walk-back, and a source change between base and record (CANNOT TELL);
  - push and manual-run bases;
  - release checks: the npm-publish toolchain, the pin, `EXPECTED_WORKLET_SHA256`, lineage and the
    candidate commit, and the no-base wording;
  - `record` posting exactly `POST repos/R/statuses/SHA` with the context, description and
    target URL;
  - `report` making exactly one lookup of the base.

  Fifteen hand mutations of the script each turn the self-test red: the compare ignoring the
  record; any ancestor counting; no walk-back; any context counting; a refused lookup swallowed;
  the record's context or commit changed; the npm toolchain unchecked; the toolchain flag dropped;
  the base taken from the PR head; the no-base wording reverted; the twin, pin or lineage
  unchecked; and a malformed record accepted.
- **Read-only against the real API** (`gh api repos/misofm/engine/commits/5379e46c.../status`,
  `origin/main`). The response has `statuses: []` (keys `commit_url, repository, sha, state,
  statuses, total_count, url`), and `report --event push --before 5379e46c...` prints
  `ARTIFACT CANNOT TELL: ... 5379e46c... has no audioworklet-sha256 status ...`, exit 0. Nothing
  was written to GitHub.
- **A dry run of the fetch against fake API responses.** That real response, with one status added
  in GitHub's documented shape, is served by a fake `gh`. A matching record reads
  `ARTIFACT UNCHANGED ... is the digest base 5379e46c...'s own CI run recorded`, and a different one
  reads `ARTIFACT CHANGED: 0000... -> a6446aa5...`. Each makes one lookup of the base.
- **End to end through the workflow's own `run:` blocks**, with a fake `gh` that stores POSTs and
  answers GETs, in a scratch clone:
  - `main`'s push at `c7331f34`: the build step output `sha256=6c952a2c... rustc=1.97.1`, and the
    record step posted exactly that status.
  - Sol's T5 on top (`CARGO_PROFILE_RELEASE_OPT_LEVEL: s` at the workflow level;
    `check-ci-path-routing.py` green): `artifact` built `6d9e8945...`, and the twin reproduced it.
  - The report read `ARTIFACT CHANGED: 6c952a2c... -> 6d9e8945... at 73965948..., against the
    digest base c7331f34...'s own CI run recorded`. Attempt 2 printed UNCHANGED on the same case.
- **actionlint 1.7.7** (the official release binary, checksum-verified, in scratch): all workflows
  pass with shellcheck off. With shellcheck 0.10.0 on `qualification.yml` and `npm-publish.yml`,
  the five findings are the same as on `c69736c1` (SC2034 at 347 and in `npm-publish.yml`, SC2251
  at 514 twice and at 520), none in #1061's blocks.
- **Other gates.** `check-ci-path-routing.py`, `test-ci-path-routing.py`,
  `check-script-reachability.py` (137), `test-script-reachability.py` (18),
  `test-npm-publish-modes.py` and `test-sdk-artifact-builder-output-contract.sh` pass. No Rust
  changed.

**Only the first real CI runs can verify:**
1. **The post.** A `main` push's `GITHUB_TOKEN`, with job-level `statuses: write`, posts the status
   (the first push after #1061 merges).
2. **The lookup.** `statuses: read` is enough for the combined-status GET in a PR run, fork PRs
   included.
3. **The runner.** `gh` behaves on the runner as it does here.
4. **Timing.** A PR opened while its base's run is still before `artifact` reads CANNOT TELL until
   rerun.

Also expected: the first PR based on a pre-#1061 `main` commit, such as batch 3's PR to `main`,
reads CANNOT TELL, because no record exists yet. The #1061 merge push writes the first record.

**Residual, stated plainly.**
- A deliberate edit of the unpinned toolchain install step, for example a `GITHUB_PATH` line with a
  forged `gh`, can still forge the lookup, as any hand-written change to a PR-controlled workflow
  can. The pins guard against losing the comparison by accident, and review guards against
  forgery.
- Sol's attempt 2 finding 4, the job-level masking classes for every other job, remains the
  follow-up issue he proposed. This attempt closes it only for the two #1061 jobs.

## Sol verdict, attempt 3

**PASS.** The redesign closes attempts 1 and 2 by design. A change is now compared with the digest
its base's own `main` run recorded, not with a base rebuilt in the change's environment. Every
environment plant now reports the truth. Release integrity is unchanged and gains a toolchain check.
What remains can only make the informational report lie through a deliberate, reviewable edit. None
of it reaches release integrity.

Reviewer: Sol, 2026-09-28, on `d9054cac`.
- The workflow's own `run:` blocks ran through a scratch harness: workflow-, job- and step-level
  env; `if:` evaluated; a stateful fake `gh` that stores posted statuses and serves the combined
  status as the latest status per context. It ran in a clone at another path.
- A `push` of `d9054cac` to `main` ran `artifact`, which built `6c952a2c...` and posted
  `6c952a2c... rustc 1.97.1` on `d9054cac`.
- Each scenario is a synthetic `pull_request` merge onto it, run through `artifact` and
  `artifact-identity`.

### Findings, by severity

1. **LOW: a same-repository change can forge its base's record, and the forgery moves only the
   report.**
   - **How.** `artifact` holds `statuses: write` on every event, not only the `main` push that uses
     it. `actions/checkout` persists that token in `.git/config`. So any of these can post
     `audioworklet-sha256` onto the base sha:
     - an extra step before the pinned record step;
     - a build step that reads the persisted token;
     - PR code run by the build, such as a proc macro or `cargo run -p parameter-metadata`;
     - a second workflow with `statuses: write`.

     The checker stays green for the first, second and fourth. The fake API showed the effect:
     after a forged post of the PR's own digest onto `d9054cac`, T5 reads `ARTIFACT UNCHANGED`.
     A later genuine post restores CHANGED, because the latest status per context wins. A forged
     `failure` status forces `ARTIFACT CANNOT TELL`.
   - **Who can do it.** Fork PRs get a read-only token, so they cannot. Same-repository authors
     already have status write through their own access.
   - **What it cannot do.** `main`'s required check is pinned to the Actions app
     (`checks: [{app_id: 15368, context: qualification}]`), so a forged status cannot satisfy it.
     The release checks read only git and the `artifact` job's outputs, never the record. And
     `npm-publish.yml` rebuilds and refuses bytes other than `EXPECTED_WORKLET_SHA256`.
   - **Hardening, cheap.** Move the record into its own push-only job that runs no PR code, with
     `persist-credentials: false`. Then no PR-event job of the required workflow holds a write
     token, which was true before #1061.
2. **LOW: the non-`success` record branch is untested.** Deleting `row.get("state") != "success"`
   from `recorded` leaves the self-test green: every record fixture is `success`. Add a `failure`
   record fixture. Three other hand mutations of mine each turn the self-test red:
   - every diff treated as documentation;
   - a failed lookup swallowed;
   - the result forced to UNCHANGED.
3. **LOW, operational.**
   - **Sequencing.** #1061 must reach `main` in a push that does not also move the pin. The planned
     order meets this: slim-2 merges first, under the old rule, with its re-pin; #1061 then goes
     into batch 3. My real-API read of the batch-2 merge against today's `origin/main` goes red,
     because relative to that `main` it is a release change and `EXPECTED_WORKLET_SHA256` is
     still 0.4.3's. From #1061 on, a batch-boundary re-pin is a release change, so the boundary
     habit of `570a79f0` and `cbfaf9de` must stop.
   - **Timing.** A newer `main` push replaces a pending one in the same concurrency group, and a
     replaced run never posts its record. PRs on such a base read CANNOT TELL, unless it is
     documentation-only from a recorded ancestor.
   - **No retry.** A transient API failure in the record step turns `main`'s `artifact` job red.

### The questions

1. **The environment findings are closed by design.** Each scenario, run through the workflow's own
   steps against `d9054cac`'s recorded `6c952a2c...`:
   - **T5**, workflow-level `CARGO_PROFILE_RELEASE_OPT_LEVEL: s` (checker green):
     `ARTIFACT CHANGED: 6c952a2c... -> 6d9e8945...`.
   - **T1**, workflow-level `RUSTUP_TOOLCHAIN` 1.98.1: `ARTIFACT CHANGED ... -> b7dfd909...`,
     "(a toolchain change)".
   - **T2**, a job-level `RUSTUP_TOOLCHAIN` on both jobs: the checker is now red ("no job-level
     ... env"), and the report is truthful anyway, the same CHANGED line.
   - **U**, comment-only: `ARTIFACT UNCHANGED`.
   - **Can a change alter the record it is compared against?** Only by forging, which is finding
     1; reading a different context means editing the script, a reviewable diff.
   - **Can CANNOT TELL be forced?** Only by a forged `failure` status. It never skips the release
     checks, which key on the git base, not on the record. A refused lookup exits 2, not CANNOT
     TELL.
   - **Relevance.** None of these touches release integrity; they affect only the report.
2. **Permissions and events.**
   - **Scoping.** `statuses: write` is on `artifact` only, and the record step is pinned to
     `push` on `main`. Each job's permissions are pinned exactly. Finding 1 has the scoping
     caveat.
   - **Fork PRs work.** The repository is public, and the combined status endpoint answers
     unauthenticated: a real read-only lookup of `5379e46c` returns `pending`, with 0 statuses.
     So the fork read-only token suffices. Through the script, that lookup reads
     `ARTIFACT CANNOT TELL ... has no audioworklet-sha256 status`.
   - **Other events.** `merge_group` is not a trigger, and `workflow_dispatch` has no base, so it
     reads CANNOT TELL.
   - **Several statuses.** The combined status is the latest per context, as demonstrated, and a
     re-run re-posts the same reproducible bytes. `main` commits carry no other statuses today;
     Actions uses check runs.
3. **Release integrity is unchanged.**
   - `npm-publish.yml` is byte-identical to `main`'s.
   - Toolchain plant, a release identity for `6c952a2c...` with `npm-publish.yml`
     `RUSTUP_TOOLCHAIN: "1.98.1"` while CI built with 1.97.1: exit 1, with `**FAIL**
     ... RUSTUP_TOOLCHAIN is 1.98.1, but the artifact job built with rustc 1.97.1`.
   - The same release with `"1.97.1"` passes all five lines.
4. **Gates, on `d9054cac` merged into `codex/batch-slim-2` at `6709552c`.**
   - **The merge.** One conflict, `scripts/test-web-audioworklet.mjs`; #1061's version is taken.
     The merge then differs from the branch only in the pin, `results.json` and the matrix, all
     `6c952a2c...` over `cbfaf9de`, with no build input.
   - **Workflow and routing:**
     - `check-ci-path-routing.py` and `test-ci-path-routing.py`: pass;
     - `check-script-reachability.py` (137) and `test-script-reachability.py`: pass;
     - actionlint 1.7.7 on both workflows: clean. With shellcheck 0.10.0 the same four
       pre-existing warnings appear.
   - **On the recorded module `6c952a2c...`:** `check-web-audioworklet.sh`,
     `test-web-audioworklet.sh`, `test-web-audioworklet.mjs` and
     `test-sdk-artifact-builder-output-contract.sh` pass.
   - **Script and policy checks:**
     - the identity self-test passes;
     - `check-env-vocabulary.sh` passes (67);
     - every `check-*.py` and `test-*.py` passes under `python3 -B`. The six that need arguments
       exit with their usage.

## Attempt 3 follow-up (verdict LOWs 1 and 2)

- **Record forgery.**
  - *The new job.* The status post moved out of `artifact` into a new job, `artifact-record`:
    `needs: [route, artifact]`, with
    `if: github.event_name == 'push' && github.ref == 'refs/heads/main' && (route sdk or full)`
    and `permissions: statuses: write`. It has one inline step: validate the digest and rustc
    release, then `gh api -X POST .../statuses/$GITHUB_SHA`. It checks out nothing and runs no
    repository code. `artifact` no longer declares any permission, so it inherits
    `contents: read`.
  - *The verdict* needs the job and expects `success` only for a push to `main` on a route that
    builds the module, otherwise `skipped`. The header now counts seventeen leaf jobs.
  - *The script's `record` subcommand is gone.*
  - *The checker.* `check_qualification_artifact_record` pins the job's `needs`/`if:`, its
    permissions and its one step exactly, bans job-level `continue-on-error`/`defaults`/`env`, and
    refuses a write permission (`<scope>: write` or `write-all`) on any other job.
  - *New mutants in `test-ci-path-routing.py`*, each red:
    - `statuses: write` granted to `artifact`, to `artifact-identity`, and `write-all` to
      `artifact-identity`;
    - the record job reachable by pull requests, or by any push;
    - its permission dropped, or a checkout added;
    - its expectation-table row dropped.

    Spot-checked by hand: a `contents: write` on `lint` is red too.
  - *Dry run.* The step's `run:` block, run with a fake `gh`, posts the exact arguments and refuses
    a malformed digest or rustc release.
- **Non-`success` records.** `recorded` now reports a record in any other state as "in state
  `<state>`, not `success`, so it is not trusted" (CANNOT TELL). The self-test serves `failure`,
  `error` and `pending` records whose digest equals the built one, and requires CANNOT TELL. Deleting
  the state check, or the malformed-description check, turns the self-test red.
- **Gates.**
  - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
  - `check-script-reachability.py` (137) and `test-script-reachability.py` (18) pass.
  - The identity self-test passes.
  - actionlint 1.7.7 passes on all workflows. With shellcheck 0.10.0, `qualification.yml` shows
    only its four pre-existing findings (shifted by nine lines), none in `artifact-record`.
