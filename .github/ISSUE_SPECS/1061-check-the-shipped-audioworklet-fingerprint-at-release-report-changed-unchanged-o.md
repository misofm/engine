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
