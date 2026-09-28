# Delete dead fixtures and uncompiled research sources, and fix the stale target matrix

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 3, rows A7 and
A8. No ruling is needed. Nothing reads or compiles any of these files. Git keeps them.

## Context

- **`fixtures/capi-qualification/v1/`: 13 files, 30,007 bytes.**
  - It is a ledger of C-ABI gates. `GATES.tsv` and `AUTHORITIES.sha256` name
    `scripts/check-native-pcm-runner-v1.sh` and
    `scripts/check-native-pcm-runner-portability-v1.sh`, which no longer exist.
  - No code, script or workflow reads the folder.
  - Only `docs/C_ABI_V1_QUALIFICATION.md`, `docs/ENGINE_ENV_VOCABULARY.md`,
    `docs/derivations/243-sdk-boot.md` and inventories mention it.
  - It is dead whatever the C-ABI ruling (`R2-…`) decides.
- **`dsp-research/archive/issue-0{31,42,44,45}/*.rs`: 4 files, 5,134 lines.**
  - These are research sources. No workspace or fuzz manifest includes them.
  - `scripts/check-dsp-research.sh` reads only the Markdown corpus. Confirm by running it.
  - Because they are not Markdown, editing them routes CI to `full`.
- **`docs/TARGET_MATRIX.md`'s table is stale.**
  - It still claims a "native baseline scalar", runtime AVX2 detection, and separate baseline and
    `simd128` wasm artifacts.
  - Master plan #83 D4 (pinned x86-64-v3, no runtime dispatch) and W4-D1 (one `simd128` artifact)
    superseded all three.
  - Its #378 register (native AArch64 "unsupported, no claim") is current and stays.

## Smallest closable slice

1. Delete `fixtures/capi-qualification/`, and remove or re-point the three doc mentions listed
   above.
2. Delete `dsp-research/archive/` whole: four `.rs` files and the `README.md` that indexes them.
   That README says the files "will not build unmodified" and that `check-dsp-research.sh` "does
   not scan this directory". Re-point the comment in `crates/dsp-reference/src/svf.rs` that names
   the archive to the issue specs (#031, #042, #044, #045), which keep the history.
3. Rewrite `docs/TARGET_MATRIX.md`'s table to match today:
   - native x86-64-v3 (AVX2+FMA pinned, `Simd8`) for tooling;
   - one `simd128` AudioWorklet artifact (`Simd4`);
   - everything else unsupported.

   Keep the #378 register.

## Objective gates

1. **Build.** All three pass, and none of them references the deleted files:
   - `cargo check --locked --workspace --all-targets --all-features`;
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`;
   - `cargo check --locked --manifest-path fuzz/Cargo.toml --bins`.
2. **Research and docs gates:** `bash scripts/check-dsp-research.sh`,
   `bash scripts/test-dsp-research.sh` and `bash scripts/check-builtins-listening.sh` pass.
3. **Console digests and artifact:** unaffected. The only crate edit is a comment in
   `dsp-reference`, which is a dev-dependency and not in the shipped module's closure. Show
   `git diff --stat` touching only `fixtures/`, `dsp-research/`, `docs/` and that comment.
4. **CI routing:** `check-ci-path-routing.py` and `test-ci-path-routing.py` pass. The change
   routes to `full`, because `fixtures/` and non-Markdown `dsp-research/` files are not evidence
   paths.
5. **No live claim lost:** no test or check reads the deleted files. Show a
   `rg 'capi-qualification|archive/issue-0' crates hosts tools scripts sdk fuzz .github/workflows`
   that finds nothing.

## Dependencies

None.

## Standing rules for the implementer

- Commit on `codex/<issue>-delete-dead-fixtures`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F3.

1. **Step 3's target table contradicts the corrected scope.** "Everything else unsupported" would
   write native AArch64 out of the matrix just as the owner ruled mobile playback (iOS, Android) in
   scope. Rewrite the table as: native x86-64-v3 (AVX2+FMA pinned, `Simd8`) for tooling and tests;
   one `simd128` AudioWorklet artifact (`Simd4`); native AArch64 iOS and Android (`Simd4`, NEON)
   as **product targets pending the R1 ruling**, with the #378 register kept and marked as the list
   of defects to clear. Do not state a support level the owner has not ruled; if the ruling is not
   in yet, leave the two AArch64 rows as they are and change only the stale x86 and wasm rows.
2. **The "When native AArch64 is revived" recipe** (`TARGET_MATRIX.md:52-68`) names
   `-p target-smoke -p host-mobile`. Keep it until R1 decides those crates, and add
   `-p host-core` to it, since `capi` wraps host-core.
3. `fixtures/capi-qualification/v1` stays dead under the corrected scope: it names two scripts that
   no longer exist, and the C ABI's live gates are `scripts/check-capi-abi.sh`, the capi tests and
   `audit capi`.
4. **An open issue cites the fixture.** The open #26 spec (`.github/ISSUE_SPECS/026-…md:88-92`)
   names `fixtures/capi-qualification/v1` as the superseded C-ABI result whose re-run #26 owns.
   The fixture is still unread by any code, so deleting it is safe, but step 1 must re-point #26's
   spec text to a commit permalink in the same change (and #26 is now mobile-relevant, since it
   qualifies the native C ABI and runner target matrix).

## Attempt 1 evidence

Terra, on `codex/1029-dead-fixtures-research-sources` from the batch-2 head `52016391` (PR #1067:
#1017, #1035, #1037, #1063, #1034, #1036). Implementation commit `a1a96d3f`.

### Recount on this base

Nothing in scope had moved since the audit. #1035, #1037 and #1017 left both folders untouched, and
#1033 (in review) deletes `fixtures/sources/v1` and `fixtures/native-pcm-runner`, not this one.

| deleted | files | lines | bytes |
|---|---:|---:|---:|
| `fixtures/capi-qualification/v1/` | 13 | 478 | 30,007 |
| `dsp-research/archive/issue-0{31,42,44,45}/*.rs` | 4 | 5,134 | 168,431 |
| `dsp-research/archive/README.md` | 1 | 23 | 1,459 |
| **total** | **18** | **5,635** | **199,897** |

`git diff --stat 52016391 HEAD`: 22 files, +31 −5,655. The only paths outside `fixtures/` and
`dsp-research/` are three docs, #26's spec and one comment:

| file | +/− | change |
|---|---|---|
| `crates/dsp-reference/src/svf.rs` | +4 −2 | the comment inside `#[cfg(test)] mod tests` points to issues #031/#042/#044/#045 and to git (`dsp-research/archive/` at `5379e46c`) |
| `docs/C_ABI_V1_QUALIFICATION.md` | +6 −3 | the two fixture mentions are now past tense, with a permalink to the folder at `5379e46c` (origin/main, where it is byte-identical), and the live C-ABI gates are named |
| `.github/ISSUE_SPECS/026-…md` | +7 −4 | amendment 4: the same permalink. It records that #319 (`f0509c3f`) deleted the runner and checkers and that #1029 deleted the ledger. #26's obligation sentence is unchanged |
| `docs/TARGET_MATRIX.md` | +14 −11 | see below |

`dsp-reference` is a dev-dependency (`cargo tree -i dsp-reference -e normal`: only `audit`,
`conformance` and `bench`), and it is absent from `host-web`'s wasm closure and from `capi`'s. The
edited comment is also inside a test module.

### Target matrix

Changed only what is stale after #1017, #1041 and the rulings:

- The three x86 rows (scalar baseline; AVX2 "entered after runtime detection"; FMA "independently
  detected") are now one row: native x86-64 for tooling and tests, not a shipped product target.
  It is pinned x86-64-v3 (#83 D4, `.cargo/config.toml`) with `Simd8` as a compile-time constant,
  refused without AVX2 and FMA at compile time (`lane`) and at boot (`lane::attest_host()`). Its CI
  evidence is the `lint` job's three probes, which exist as described in `qualification.yml`
  (`-avx2,-fma` and `+avx2,-fma` must fail with `requires x86-64-v3`, and `+avx2,+fma` compiles).
- The wasm row said "baseline and `+simd128` are distinct artifacts" and cited "two release
  artifact directories". It now says there is one `simd128` AudioWorklet artifact (W4-D1), built by
  `build-web-audioworklet.sh`, with `Simd4`, multiply plus add, and no relaxed SIMD or FMA
  (`docs/audits/issue-triage-2026-09-14.md` cites that fact). It also names the typed
  `miso.unsupported.v1` refusal and the `artifact`/`artifact-gates`/`browser` jobs. It does not
  mention the sha256 pin, because #1061 changes where the pin is checked.
- Other fixes:
  - The intro line ("Issue 001 establishes … not … browser runtime") and the "CI evidence in issue
    001" column header.
  - "Probe flags are evidence that separate artifacts compile". The probes now prove the sub-v3
    refusal.
  - "Browser execution … deferred to the platform adapter issues". The browser job qualifies it
    now.
  - The reproducible checks add the artifact build line and label the scalar build as the CI
    exception. Both cargo lines are kept verbatim, including `-p target-smoke`, which the R1 shells
    still use.
- Not touched:
  - the AArch64 and Refused rows (#1017, #1041);
  - "64-bit only" and the scalar-wasm exception;
  - the dispatch contract, including the `KernelBackendV1` tombstone that
    `de-versioning-inventory.md:108` requires;
  - "Native AArch64";
  - "Render threading";
  - the "Known AArch64 defects (#1017)" register.
- Amendment 1 is satisfied by #1017's rows, which follow the owner's ruling. Amendment 2 is moot:
  #1017 (`401fc362`) already replaced the "When native AArch64 is revived" recipe, and no
  `-p host-mobile` recipe remains.

### Kept, and why

- `docs/ENGINE_ENV_VOCABULARY.md:129` and `docs/derivations/243-sdk-boot.md:42,173` match
  `capi-qualification` only because they name the checker scripts #319 deleted
  (`check-capi-qualification-v1.sh`, `check-capi-qualification-evidence-v1.py`). They do not name
  the fixture, and both stay true: one is a live env var's history, the other a dated script count.
  #1033 and #1061 also edit the vocabulary file.
- The dated rulings (`prefix-strip-inventory.md`, `de-versioning-inventory.md`), the 2026-09-04
  audit, the closed specs (045, 097, 104) and #1033's in-review spec record past states. They are
  left for #1040 (R10) and git.

### Gates (all on `a1a96d3f`)

| gate | result |
|---|---|
| `cargo check --locked --workspace --all-targets --all-features` | pass, 0 warnings |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `cargo fmt --all -- --check` | pass |
| `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web` | pass |
| `cargo check --locked --manifest-path fuzz/Cargo.toml --bins` | pass |
| `bash scripts/check-cross-targets.sh`: aarch64 iOS and Android check and clippy, iOS memset ceilings (#1018, unchanged counts), wasm scalar/simd128, armv7 refusal | PASS |
| docs-gates job: `check-dsp-research.sh`, `test-dsp-research.sh`, `check-builtins-listening.sh` | ok |
| `check-dsp-research.sh` on a base-tree copy with `dsp-research/archive` made unreadable (mode 000) | ok, so it never reads the archive |
| `check-ci-path-routing.py`, `test-ci-path-routing.py` | pass; `ci-path-router.py --base 52016391 --head a1a96d3f` gives `route=full` (the `.rs` archive files, `fixtures/` and `svf.rs` route full) |
| script reachability (`check-` and `test-script-reachability.py`) | ok (136 reached, 8 operator exempt; 18 mutations) |
| every lint-job script (54 policy, fixture and self-test commands, Python with `python3 -B`), plus the docs gates, routing and three vocabulary/shape checks: 62 commands | 62/62 exit 0 |
| spec gate 5: `rg 'capi-qualification\|archive/issue-0' crates hosts tools scripts sdk fuzz .github/workflows` | no match (exit 1) |
| tests of every crate that reads `fixtures/` (debug, `--all-features`: builtins, builtins-compiler, capi, compressor, conformance, effect-compiler, graph, graph-compiler, host-core, protocol, session, host-web, native-pcm-runner, parameter-metadata, session-validator) | 1,281 passed, 0 failed, 8 ignored |
| release `cargo test -p audit -p bench -p console-workload` | 130 passed, 0 failed, 2 ignored |
| `conformance_fixtures --check`; `check-builtins-fixtures.sh` and `check-console-fixtures.sh` against release binaries | pass |
| `check-capi-abi.sh` and `--self-test`; `audit capi` | ok (shared and static linkage); 100,000 calls, every violation counter 0 |
| console digests: `gain_pan_profile digests` on a base-tree build and on the change | 17 rows, byte-identical |

The `check-capi-abi.sh` self-test hardcodes `<workspace>/target/release`. It was run through a
temporary `target` symlink to the scratch build, which was then removed.

### `cargo test -- --list`

`cargo test --locked --workspace --all-features -- --list` on `52016391` and on `a1a96d3f`:
2,273 tests in 287 test binaries and doc-test groups on both. The list output is byte-identical,
and the binary names are identical once hashes are stripped. No test was added or removed, as
expected: nothing compiled or read the deleted files.

### Parallel branches

`git merge-tree` of this branch with #1033, #1039, #1049 and #1059 is clean. #1059 edits
`TARGET_MATRIX.md` in the dispatch section, in different hunks from this change. #1061 conflicts
only in `scripts/test-web-audioworklet.mjs`, and it conflicts there with the base `52016391` too,
so this change is not the cause.

## Sol verdict, attempt 1

**PASS.**

Sol verified the change on a scratch merge of `0e73621a` into the batch head `codex/batch-slim-3`
(`a509b681`: main, #1031, #1030, #1033 and #1061). The merge is clean, with no textual conflict.
#1033's header note and #1030's `df8cebb3` re-points in `docs/C_ABI_V1_QUALIFICATION.md` compose
with this change's paragraph, and the result reads coherently. `docs/TARGET_MATRIX.md` is untouched
by batch 3. `ci-path-router.py` routes the merge `full`.

### Checks

1. **Nothing live was deleted.** The recount matches: 18 files, 5,635 lines and 199,897 bytes.
   - A `git grep` of the merged tree finds no mention of `capi-qualification`, `dsp-research/archive`,
     `archive/issue-0`, `issue-0NN/`, or any of the 13 fixture basenames (for example
     `EXPECTED_SYMBOLS` and `runtime_consumer`). The search covered code, scripts, workflows,
     manifests, `.cargo`, and fuzz. The only hits are dated docs, closed specs, #1033's spec,
     `docs/handoffs`, and the re-pointed prose.
   - `check-capi-abi.sh` derives its expected symbols from `crates/capi/include/miso_engine_v1.h` and
     compiles `crates/capi/tests/c/abi_smoke.c`, not the fixture.
   - No manifest or workspace `exclude` names `dsp-research`.
   - `check-dsp-research.sh` reads only the named Markdown files.
2. **Citations resolve.** `git cat-file -e 5379e46c:<path>` succeeds for three paths: the folder, all
   13 fixture files, and `dsp-research/archive` with its four `.rs` files.
   - `5379e46c` is an ancestor of `origin/main`. The GitHub contents API also serves both folders at
     that commit (13 entries, and the README with four issue folders).
   - `parametric_eq_recurrence_proof.rs:17-21` at `5379e46c` holds exactly the RATES…SLOPES
     constants.
   - `f0509c3f` (#319) deleted `run-capi-qualification-v1.sh` and both Python checkers, as #26 now
     says.
3. **`TARGET_MATRIX.md` is accurate.** Each changed statement holds against the merged code and CI:
   - `.cargo/config.toml` pins `+avx2,+fma` for `x86_64`.
   - `lane`'s `compile_error!` reads "requires x86-64-v3".
   - `Backend::current()` is a `const fn` that returns `Simd8`/`Simd4`.
   - `attest_host()` is called from `capi`, `host-mobile` and `host-native`.
   - The `lint` job's three probes are at `qualification.yml:523-537`.
   - `lint`, `test-debug-a`, `test-debug-b`, `test-release` and `audit-native` run on `ubuntu-24.04`.
   - The one `miso-engine-v1-audio-worklet.simd128.wasm` is built once by `artifact`, and is gated
     by `artifact-gates` (`check-web-audioworklet.sh`) and by `browser` (chromium, firefox, webkit).
   - The host's typed refusal is `miso.unsupported.v1`, with capability `simd128`.
   - `build-web-audioworklet.sh` requires an existing, empty output directory.

   The following sections are byte-identical to `52016391`: the ARM64 and Refused rows, "64-bit only"
   (#1041) through the dispatch contract, and everything from "Native AArch64 (#1017)" to the end
   (#1017's register). The cross-target run's memset counts equal the register's ten ceilings.
   Amendment 2 is moot, as claimed: `401fc362` removed the "revived" recipe.
4. **Gates.** Every gate below was run on the merge, and every one passed.
   - `cargo check --workspace --all-targets --all-features` passed with 0 warnings. Clippy
     `-D warnings`, fmt, and `cargo doc` with `-D warnings` also passed.
   - `host-web` `+simd128` check and the fuzz `--bins` check passed.
   - `check-cross-targets.sh` passed: aarch64 iOS and Android check and clippy, the iOS memset
     ceilings, and the armv7 refusal.
   - The docs-gates trio, and routing check and test, passed.
   - Script reachability check and test passed: 134 scripts reached and 8 exempt, with 18 mutation
     cases.
   - All 52 of the `lint` job's hermetic commands passed, with Python run as `python3 -B`. Also
     passed: `check-release-shape.py`, `check-sdk-deletions.py` with its self-test,
     `web-audioworklet-identity.py --self-test`, `test-wasm-realtime-atomics.sh`, and
     `test-web-audioworklet.sh`.
   - Debug `--all-features` tests of the 14 fixture-reading crates: 1,271 passed, 0 failed, 8
     ignored.
   - Release tests of `audit`, `bench` and `console-workload`: 128 passed, 0 failed, 2 ignored.
   - The console digests are 17 rows, byte-identical to a separate build of `a509b681`.
   - `audit capi`: 100,000 calls, 0 violations.
   - `check-capi-abi.sh` and its `--self-test` passed.
   - The builtins, console and conformance fixture checks passed, as did `check-effect-contract.sh`
     and graph determinism (100/100).
   - The shipped delivery closure is byte-identical on the merge and on `a509b681`, with module
     `01dd58be…`. `check-web-audioworklet.sh` and `check-browser-expected-resources.py --artifacts`
     pass on it.
   - `build-web-audioworklet.sh --check-pin` fails the same way on base. The committed pin is
     `6c952a2c…` from slim-2. This is not a per-PR gate under #1061; the pin is refreshed at the
     release boundary.

### Findings, by severity (none blocks)

1. **Low: stale citations for #1040.** The svf.rs comment keeps its earlier pointer to
   `.github/ISSUE_SPECS/045-*.md`, and #26 points to the closed 114 spec for its drifted paths. Once
   #1040 (R10) prunes closed specs locally, both dangle, so #1040 must re-point them. Closed specs 002,
   031, 042, 044 and 045 still name `dsp-research/archive/…`, and 083, 097, 104 and 114 name the
   capi ledger. These are dated records of closed issues, which R10 owns.
2. **Low: #26 is partly obsolete after #1033.** Its "one re-run of that matrix" obligation and line
   31 ("Native PCM reference runner and C ABI qualification") include runner rows that #1033 made
   impossible. #1029 correctly left the obligation unchanged. #26 needs a rebrief, which is not this
   issue's scope.
3. **Information: evidence counts moved on the batch base.** The attempt 1 evidence counted
   1,281 and 130 tests, including `native-pcm-runner`. On the batch 3 merge, #1033 has removed that
   crate, and the counts are 1,271 and 128. No failure is involved.
