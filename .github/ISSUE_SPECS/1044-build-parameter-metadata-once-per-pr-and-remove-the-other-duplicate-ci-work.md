# Build parameter-metadata once per PR, and remove the other duplicate CI work

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 items 3 and 13). Base `a9414c0c`. Paths
starting `../` are relative to the audit's handoff folder. No ruling needed.

## Problem

Several required jobs repeat work another job already did at the same commit. Figures are from PR
#1016's logs and `../data/script-gates-verify.md` §2 and §8.

| duplicate | where | cost |
|---|---|---:|
| native fat-LTO `cargo run --release -p parameter-metadata` | `artifact` (writes the metadata into the artifact), then `artifact-gates` (`scripts/check-web-audioworklet.sh:492`, `--check`) and `sdk` (`sdk/codegen/assets.mjs:44`, twice via `scripts/check-sdk-generated.sh` and `scripts/sdk-package.sh:43`) | 61.6 + **75.0 + 72.3 s** |
| `node scripts/test-web-audioworklet.mjs` | `lint` (`qualification.yml:305-306`), and three runs again in `artifact-gates` via `test-web-audioworklet.sh` | 0.6 s plus the lint job's Node setup |
| effect-runtime policy and its mutation suite | `lint`, and again inside `scripts/check-effect-contract.sh:30-31` in `audit-native` | 4.9 s |
| "Math M3 digests on an FMA-enabled build" | `qualification.yml:540-541` sets `RUSTFLAGS=+fma`, but `.cargo/config.toml` already builds every x86-64 target with `+avx2,+fma`, so the main release leg is already FMA-enabled. The override only drops AVX2 | 9 s plus a separate target dir |

The `--check` in `artifact-gates` compares the generator's output with the same generator's output
at the same commit. It can only go red if the generator is non-deterministic.

## Outcome

- **`artifact-gates`:** `check-web-audioworklet.sh` skips `parameter-metadata --check` when it is
  given the downloaded, pin-verified CI artifact directory. Local no-argument use keeps the check.
- **`sdk`:** `check-sdk-generated.sh` compares `sdk/assets/miso-engine-v1-{parameter-metadata,abi-layout}.json`
  byte for byte against the downloaded artifact's copies instead of re-running cargo. The job drops
  the Rust toolchain and cache if nothing else in it needs them. `sdk-package.sh` stops calling
  `check-sdk-generated.sh` a second time.
- **`lint`:** drops its copy of `test-web-audioworklet.mjs`.
- **`check-effect-contract.sh`:** drops its re-run of the effect-runtime policy.
- **`test-release`:** drops the M3 "FMA" step.

## Scope

Authorized paths:
- `scripts/check-web-audioworklet.sh`;
- `scripts/check-sdk-generated.sh`, `scripts/sdk-package.sh`, `sdk/codegen/assets.mjs`;
- `scripts/check-effect-contract.sh`;
- `.github/workflows/qualification.yml`;
- this issue's spec.

## Gates

1. **The SDK drift claim still discriminates:**
   - a one-byte hand edit to `sdk/assets/miso-engine-v1-parameter-metadata.json` fails the `sdk` job;
   - a Rust change that alters the generated metadata without regenerating `sdk/assets` fails the
     `sdk` job, because the artifact copy differs.
2. **M3 still runs with FMA.** The test-release log shows `target_feature="fma"` in
   `rustc --print cfg` for the leg that runs `m3_determinism`, and the M3 pins still pass.
3. **Nothing else lost.** Every step removed here names the step that still runs the same check.
   The effect-runtime policy still fails lint on a seeded dependency addition.
4. **Cost.** Measured on a full-route PR:
   - `artifact-gates` about −70 s;
   - `sdk` about −70 s plus toolchain setup;
   - `test-release` −9 s.

## Saving and risk

- **Saving:** about 150 s of runner time per full PR.
- **Not included:** the capi release rebuild in `check-capi-abi.sh` (36 s). It recompiles 23
  workspace crates because `-p capi` alone resolves different features than the audit build.
  `rust-cache` does not keep workspace crates, so a separate target dir would not help. Setting
  `SKIP_BUILD=1` would point the ABI check at an audit-feature build. No cheap fix was found.
- **Risk:** none identified. Each removed step duplicated a check that stays.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Measured.** Over 8 full-route runs the two duplicate `parameter-metadata` builds take a median
   65.8 s (`artifact-gates`) and 56.2 s (`sdk`), 122 s in all; PR #1016's run took 147 s. Set gate
   4's targets to about −55 s for `artifact-gates` and about −50 s for `sdk`, plus toolchain setup.
2. **The claims hold.**
   - The CI `--check` compares the generator with itself at the same commit.
   - The M3 "FMA" step's `RUSTFLAGS` replaces `.cargo/config.toml`'s rustflags. It builds `math`
     without `lane` on `+fma` alone, a configuration that ships nowhere, and the main leg already runs
     M3 with FMA on.
3. **Scope addition.** `scripts/check-ci-path-routing.py` pins the `sdk` job's command lines
   (`SDK_CLOSURE_LINES`, `:294-301`), including `bash scripts/check-sdk-generated.sh`. Add it and
   `scripts/test-ci-path-routing.py` to the authorized paths if any pinned line changes.
4. **Keep the local check.** Local no-argument `check-web-audioworklet.sh` keeps `--check`: it is the
   one check that catches a stale committed metadata file before a push.

## Attempt 1 evidence

Terra, 2026-09-29, branch `codex/1044-dedupe-ci-work` from `codex/batch-slim-4` (`93105e18`).
Implementation commit `d49e602d`: 8 files, +380/-53, no Rust file. Paths: the body's list plus
`check-ci-path-routing.py` and `test-ci-path-routing.py` (amendment 3). CI cannot run here; every
result below is a local run, and the savings are estimates from recent CI logs.

**Re-derived at `93105e18`.** All four duplicates still hold. #1017, #1043, #1048 and #1061 added
no new copy of any of them: `artifact-identity`'s twin is `--module-only` (no generator), and the
AArch64, `gate-self-tests` and `nightly.yml` jobs run none of these commands.

| duplicate | ran in | the one run left (job, step, routes) |
|---|---|---|
| fat-LTO `parameter-metadata` generator | `artifact` (`--write`); `artifact-gates` (`--check` in `check-web-audioworklet.sh`); `sdk` (`assets.mjs --check`, twice: `check-sdk-generated.sh`, then `sdk-package.sh`) | `artifact`, "Build the exact shipped artifact", sdk and full routes |
| `node scripts/test-web-audioworklet.mjs` | `lint`; `artifact-gates` (`test-web-audioworklet.sh:6`, plus two variant runs) | `artifact-gates`, "Hermetic browser host and worklet tests", full route, the same runner Node (both run before any `setup-node`) |
| effect-runtime policy, its mutation suite, and the fixture pair | `lint`; `audit-native` via `check-effect-contract.sh:30-33` | `lint`, "Effect runtime dependency-boundary and fixture policy and mutation tests", full route |
| M3 with FMA | `test-release` main leg (`.cargo/config.toml` `+avx2,+fma`); the "+fma only" step | `test-release`, "Lane and math gates ...", full route |

**Changed.**
- **Closure hand-off (new).** The `artifact` build step publishes `closure_sha256`: one sha256
  over `sha256sum` of every file in the directory, sorted by name in byte order. `sdk`,
  `artifact-gates` and each `browser` leg recompute it in their existing verify step, with the
  same line, before any script reads the download. So the metadata the consumers read is
  provably the generator's output from this run.
- **`artifact-gates`.** It now runs `check-web-audioworklet.sh --without-metadata-regeneration
  target/ci/qualification-artifacts`, which leaves out only `parameter-metadata --check` and says
  so in the log. The schema and vocabulary gates still read the documents. Any other form keeps
  the check: no argument (amendment 4), a directory alone, or the flag without a directory (exit 2).
- **`sdk`.**
  - `check-sdk-generated.sh DIR` runs `assets.mjs --check DIR`, which compares `sdk/assets/*.json`
    byte for byte (`Buffer.equals`) with the directory's copies and runs no cargo. With no
    argument it still runs the generator.
  - `sdk-package.sh` no longer calls `check-sdk-generated.sh`. Both workflows that package run it
    first: this job, and `npm-publish.yml`'s qualify step. Locally, `npm run build` no longer runs
    it; `npm run check:assets` does.
- **`lint`** drops its `test-web-audioworklet.mjs` step.
- **`check-effect-contract.sh`** drops all four effect-runtime scripts. The fixture pair was a
  duplicate too, since `lint` runs it on the same route.
- **`test-release`** drops the "+fma only" M3 step. In its place is
  `cargo rustc --locked --release -p math --lib -- --print cfg | grep -x 'target_feature="fma"'`,
  which prints the cfg cargo gives that leg's `math` (gate 2):
  - `math` has no dependency without `lane`, so it builds nothing. It takes 0.2-0.4 s and emits
    nothing, so it recompiles every time; a warm target dir cannot turn it into a silent skip.
  - An `RUSTFLAGS='-C target-feature=+avx2'` override turns it red.
- **`check-ci-path-routing.py`** pins these rules:
  - the closure output and build lines (including the delivery-mode `build-web-audioworklet.sh`);
  - the closure env, digest and compare lines in each reader's verify step;
  - `--without-metadata-regeneration` only in a closure-verified reader, only for
    `target/ci/qualification-artifacts`, and in no other workflow;
  - the SDK drift line with the directory;
  - each owner above, unconditional on the full route;
  - a release `cargo test -p math` in `test-release` that does not deselect `m3_determinism` and
    has no `RUSTFLAGS=` in front of it.
- **`test-ci-path-routing.py`** adds 34 mutants, run in-process. Each is refused for its own reason
  (I printed each rejection message and checked it).

**Deviation: the `sdk` job keeps its Rust toolchain and cache.** The body drops them "if nothing
else in it needs them", and something does. `check-sdk-headless.sh`'s evals run native debug
oracles:
- `cargo run -p parameter-metadata --bin parameter_metadata_lattice_oracle`;
- `cargo run -p host-web --example sdk_render_oracle`, three times.

With cargo off `PATH`, 4 of its 285 evals fail. The workflow's old comment blamed
`check-sdk-generated.sh` for this need; the comment now names the evals. The sdk saving is
therefore the generator only.

**Gates (local).**
1. **The SDK drift claim discriminates.** I ran the real delivery build
   (`build-web-audioworklet.sh`, 3 min 15 s) and then the `sdk` job's step line by line, with
   every cargo call logged:
   - `check-sdk-generated.sh target/ci/qualification-artifacts` passes in 0 s with **no** cargo
     call. `sdk/assets` is byte-identical to the closure's two documents.
   - The whole step passes: deletions, types, headless 285/285, and the package tarball (11/11).
     The only cargo calls are the headless oracles.
   - A one-byte edit to `sdk/assets/miso-engine-v1-parameter-metadata.json` is red: "differs from
     .../miso-engine-v1-parameter-metadata.json". The same holds for the ABI layout.
   - A Rust change, `"pan"` -> `"panorama"` in `tools/parameter-metadata/src/lib.rs`, then the
     artifact job's `--write` into a fresh directory with `sdk/assets` not refreshed, is red.
2. **M3 runs with FMA.** The cfg line prints `target_feature="fma"`. `cargo test --locked
   --release -p math --features lane --test m3_determinism` on the config's flags passes 5/5,
   `m3_corpus_digests_match_pins` included.
3. **Nothing else lost.** The table above names each owner, and the checker pins it.
   - The effect-runtime policy fails on a seeded `serde = "1"` in `crates/effect-contract`'s
     dependencies ("effect-contract dependency boundary changed", exit 1), on a scratch copy.
   - `check-effect-contract.sh`, with a stub bench printing the conformance record, passes. It
     calls no effect-runtime script (`bash -x` trace).
   - `test-web-audioworklet.sh`, the mjs owner, passes.
   - `check-web-audioworklet.sh` over the built closure:
     - with the flag, it passes and makes one cargo call (`host-web --example
       worst_boot_document`);
     - without the flag, it also runs `parameter-metadata --release -- --check`.
   - **Hand-off.** A tar round trip of the closure verifies with the workflow's own lines. A
     one-byte change to the downloaded metadata, or an extra file, is red: "downloaded artifact
     closure mismatch".
4. **Cost (estimated, CI not run).** From the medians of 8 full-route runs (36382065641 ...
   36507292157, 4 PR and 4 main-push), job step timings and log-timestamp brackets:

| job | removed | median | range | target (amendment 1) |
|---|---|---:|---:|---:|
| `artifact-gates` | `parameter-metadata --check` | −74.4 s | 45.8-78.2 | ≈ −55 s |
| `sdk` | first generator run, plus the warm second (0.3 s) | −73.8 s | 62.6-79.3 | ≈ −50 s, plus toolchain (kept, see deviation) |
| `test-release` | "+fma only" M3 step (9 s), less the cfg line (≈0.3 s) | −8.7 s | 6-10 | −9 s |
| `audit-native` | four effect-runtime scripts | −4.1 s | 2.7-5.1 | — |
| `lint` | mjs step | −0.5 s | 0.4-1.1 | — |

   - **Added cost:** the closure digest, one sha256 pass over about 2 MB in each of the 6 jobs
     that compute it. That is well under 0.1 s each.
   - **Total:** about **161 s (2.7 runner-minutes) per full-route PR**, and about 74 s per
     sdk-route PR.
   - **Wall time:** unchanged. The longest jobs (`audit-native` 444 s and `test-release` 436 s
     median) do not shrink by much. The `artifact` -> `artifact-gates` chain drops from about
     391 s to about 317 s.

**Other gates run.**
- `check-ci-path-routing.py` and `test-ci-path-routing.py` pass (102 s).
- actionlint 1.7.7 is clean, on this workflow and on the base's.
- The script-reachability check and its mutation tests pass.
- All 55 hermetic commands of `lint`, `docs-gates` and `gate-self-tests` pass (`python3 -B`).
- `check-sdk-deletions.py`, `check-web-audioworklet.sh --self-test-opcodes`, the V8 spill
  self-test and `test-sdk-artifact-builder-output-contract.sh` pass.
- `bash -n` and `node --check` pass on the changed scripts.

**Not done.** `npm-publish.yml` still runs the cargo form of `check-sdk-generated.sh`, before
`sdk-package.sh build "$worklet"`. Passing `"$worklet"` there would drop one more fat-LTO run per
publish, but that file is outside this issue's paths.

## Sol verdict, attempt 1

**PASS.** Sol, 2026-09-29. Verified on a scratch merge of `4946b0d1` into `codex/batch-slim-4`
(`124968b7`: main plus #1060, #1062 and #1050). Every result below is a local run; CI was not run.

**Merge into the batch.** There is one textual conflict. It is in `qualification.yml`, in the
`sdk` job's verify step: #1050 reworded the comment line after it, and #1044 added two closure lines
before it. **Root applies:** keep #1044's `closure=...` and `[[ "$CLOSURE" ... ]]` lines, followed
by #1050's comment `# check-sdk-deletions.py's --self-test (design §6.6, S7) runs in gate-self-tests`
without "37-mutation". `check-web-audioworklet.sh` auto-merges: #1062's comment and #1044's flag
touch different hunks. There is no semantic conflict:
- #1062 and #1050 change neither routing script;
- no #1044 pin names a step or script that #1062 or #1050 removed;
- every gate below passes on the merge.

**Findings, by severity.**

1. **Low: the closure digest misses an extra file with whitespace in its name.**
   - The cause: `xargs` splits the name, `sha256sum` fails on both halves, and without `pipefail`
     the digest covers the other files only.
   - A planted `a b.json` in the download leaves all three readers green.
   - No reader reads such a file. `artifact-gates` refuses anything but "the exact seven frozen
     outputs" (`check-web-audioworklet.sh`). `sdk` refuses any file-set change in
     `stage-package.mjs`.
   - Every byte of every built file is covered.
   - Fix, for a successor or this batch: `find . -type f -print0 | LC_ALL=C sort -z | xargs -0
     sha256sum --` under `set -o pipefail`, on both sides.
2. **Low: the checker pins only the literal flag and the literal verify lines.**
   The checker accepts three deliberate evasions:
   - the flag passed through a shell variable, even in `lint` over another directory;
   - the flag passed by a script, since scripts are not scanned;
   - a step that rewrites the download after the verify step. This limit already applied to the
     module under #1061.

   An accidental edit is still refused. The plants: the verify step given `if: false` or
   `continue-on-error: true` in each reader, and the build closure pointed at another directory.
3. **Info: the M3 replacement loses no behavioural claim on a shipping target.**
   The plants flip one bit of `math::exp2`'s result. The flip sits in `lib.rs`, outside
   `src/vendored`, so only the digest half of M3 can see it.
   - **FMA-conditional flip:** red under the retired step (`RUSTFLAGS=+fma`, no `lane`) and red
     under the surviving main-leg command.
   - **No-AVX2-conditional flip:** red under the retired step and green on the x86 main leg.
     The retired step's only unique coverage was an x86 build without AVX2, which ships nowhere.
     Two jobs on the same full route build without AVX2 and replay the same pins:
     - `aarch64-release` runs `-p lane -p math --features math/lane` in release, and M3 is not
       among its known-defect rows;
     - `wasm-guests` replays `M3_DIGESTS` in G5.
   - **The cfg line** prints `target_feature="fma"` on the config's flags and goes red under
     `RUSTFLAGS='-C target-feature=+avx2'`.
4. **Info: a local-only loss.** `npm run build`, and a local `npm pack` through `prepack`, no longer
   run `check-sdk-generated.sh`. The body authorizes this and the evidence discloses it. Both CI
   packagers run the gate first: `sdk` against the closure, and `npm-publish.yml` in its cargo form.
5. **Info: gate 4 is estimated, not measured.** Confirm the savings on the batch's first full-route
   CI run.

**Checked (on the merge).**
- **Routes.** Each removed step and its owner run on identical routes:
  - `lint`, `artifact-gates`, `audit-native` and `test-release` are all `route == 'full'`;
  - `artifact` and `sdk` are `sdk || full`;
  - no removed step lived in `gate-self-tests`, and no owner depends on `self_tests`.

  The owners are:
  - the mjs suite: `artifact-gates` (`test-web-audioworklet.sh`, line 6, on the runner's own
    Node);
  - the four effect-runtime scripts: `lint`. A seeded `serde = "1"` in `effect-contract` turns
    `check-effect-runtime-policy.sh` red, while `check-effect-contract.sh` now stays green;
  - M3 with FMA: `test-release`'s main leg;
  - the metadata generator: `artifact`.
- **Hand-off.** I ran the workflow's own build and verify steps under `bash -e`, with a tar round
  trip standing in for upload and download.
  - The closure digest is computed in `artifact`'s build step, over the directory it uploads.
  - The results for each of the three readers:
    - a flipped byte in any one of the 7 built files is red;
    - an extra plainly named file, a missing file, a renamed file, and the metadata and ABI-layout
      contents swapped are all red.
  - In each reader the verify step precedes every script. The checker refuses the step made
    conditional or non-fatal.
  - `artifact-identity` and `artifact-record` read only `sha256` and `rustc`, which are unchanged.
    Their checker pins pass.
- **SDK drift (gate 1).** `check-sdk-generated.sh DIR` makes zero cargo calls. The following are
  red:
  - a one-byte edit to `sdk/assets`'s parameter metadata;
  - a one-byte edit to its ABI layout;
  - a `"pan"` -> `"panorama"` generator change written to a fresh directory without refreshing
    `sdk/assets`;
  - an artifact directory missing a document.

  A symlinked directory and two arguments each exit 2.
- **The skip flag** is used once, and it gates only the `parameter-metadata --check` block. The
  flag alone, the flag twice, the flag after the directory, the flag with a second directory, and
  the flag with `--self-test-opcodes` or `--source-policy=` each exit 2.
- **Gates.**
  - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass (1 min 42 s).
  - actionlint 1.7.7 is clean on all four workflows.
  - All 52 hermetic commands of `docs-gates`, `lint` and `gate-self-tests` pass under
    `python3 -B`, including `check-script-reachability.py` and its tests.
  - `check-effect-contract.sh` passes with a real release `bench`. Its `bash -x` trace shows no
    effect-runtime script.
  - `check-web-audioworklet.sh` passes over the built closure in both forms:
    - with the flag, it makes one cargo call (`worst_boot_document`);
    - without the flag, it also runs `parameter-metadata --check`.
  - `test-web-audioworklet.sh` passes.
  - The `sdk` job's step passes: headless 285/285 and tarball 11/11. Its only cargo calls are the 4
    native oracles, which bears out the kept toolchain.
  - The local `sdk-package.sh check`, with no directory, passes.
