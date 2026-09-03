# Restore the independently locked fuzz workspace after dependency changes

## Objective

Restore reproducible locked builds and the existing bounded per-PR fuzz qualification for the
standalone `fuzz/` Cargo workspace after a production dependency of one of its path dependencies
changes. This is a tooling and dependency-closure repair only; it does not revise Session V1 or
continue implementation of issue #338.

## Baseline and cause

The exact baseline is checkpoint `2cf8aa84` on PR #339, based on `origin/main` `51468d5d`. Issue
#338 replaced the session parser and exact-pinned `jstrict = 0.14.0` in the root workspace lockfile,
but `fuzz/Cargo.toml` is an independent workspace with its own `fuzz/Cargo.lock`. That lockfile was
not refreshed. Both jobs in `.github/workflows/fuzz.yml` therefore fail before any target executes:

```text
cargo check --locked --manifest-path fuzz/Cargo.toml --bins
```

Cargo reports that the lockfile needs to be updated. This invalidates #338's terminal qualification
claim, but it is not permission for a fourth #338 implementation attempt. The independently useful
successor outcome is that the separately locked fuzz workspace builds and its already-frozen
bounded jobs execute again.

## Smallest closable slice

Regenerate `fuzz/Cargo.lock` through the pinned Cargo toolchain from the unchanged
`fuzz/Cargo.toml`, then execute the existing locked all-bin check and the two existing bounded fuzz
jobs. Do not hand-edit the lockfile and do not introduce another checker: the existing `--locked`
CI preflight already discriminated the defect exactly.

### Allowed paths

- `.github/ISSUE_SPECS/340-restore-the-independently-locked-fuzz-workspace-after-dependency-changes.md`
- `fuzz/Cargo.lock`

No other tracked path may change.

### Forbidden scope

- changes to `fuzz/Cargo.toml`, fuzz targets, corpora, run counts, seeds, time budgets, sanitizer
  options, toolchain pins, workflows, or `scripts/run-protocol-fuzz.sh`;
- changes to #338's parser, canonical writer, schema, SDK, protocol, fixtures, policies, package or
  generated/browser/Wasm artifacts;
- dependency upgrades or removals beyond the resolver-produced closure required by the current
  manifests;
- new lockfile checksum pins, lock-digest prose gates, CI routing, benchmark infrastructure, or
  broad workspace qualification; and
- rerunning the #338 descriptive benchmark or live browser qualification.

## Objective gates

1. `fuzz/Cargo.lock` is the normal Cargo-generated resolution of the existing manifest graph and
   contains `session`'s exact `jstrict = 0.14.0` dependency and only its resolver-required closure.
2. `cargo +1.97.1 check --locked --manifest-path fuzz/Cargo.toml --bins` succeeds, proving all eight
   declared fuzz binaries build without a lock update.
3. The existing per-PR session parser and compiler fuzz commands each complete exactly 10,000 runs
   with their frozen seeds and `fixtures/session/v1/canonical.json` input:

   ```text
   RUSTFLAGS='-C target-feature=+avx2,+fma' cargo +nightly-2026-08-20 fuzz run session_parse target/ci/session-fuzz/parse -- -runs=10000 -seed=557074001 -seed_inputs=fixtures/session/v1/canonical.json
   RUSTFLAGS='-C target-feature=+avx2,+fma' cargo +nightly-2026-08-20 fuzz run session_compile target/ci/session-fuzz/compile -- -runs=10000 -seed=557074002 -seed_inputs=fixtures/session/v1/canonical.json
   ```

4. `bash scripts/run-protocol-fuzz.sh` completes its existing four fixed-count protocol targets and
   produces its valid bounded evidence, proving the second failed CI job passes the same preflight
   and reaches execution.
5. `bash scripts/check-session-policy.sh`, `bash scripts/check-workspace-policy.sh`, and
   `git diff --check` pass; the checkpoint has no uncommitted files beyond its declared tranche.
6. The pull request's `bounded session parser and compiler fuzzing` and `bounded issue-005 protocol
   decoder fuzzing` jobs pass after the single coherent update.
7. No benchmark or browser qualification is run, and no claim about parser performance or new fuzz
   coverage is made.

## Review and delivery

This issue gets one implementation attempt and one fresh Sol-high adversarial review. HOLD rather
than expanding or weakening the gates if the unchanged manifests require anything beyond a normal
lock refresh or an existing fuzz command exposes a distinct defect.

Keep the work on `codex/batch-338-canonical-json` and deliver it in PR #339 as a distinct
`fix(#340)` checkpoint. A new branch from `main` cannot resolve the dependency graph introduced by
#338, while a separate issue and commit prevent this repair from becoming a disguised fourth #338
attempt. In CI-conscious mode, commit the exact two-path tranche locally, run all proportional
gates once, then include it in one coherent PR update. Do not force-push or manufacture CI commits.

Before implementation, create the matching GitHub issue with this exact title, verify it receives
number 340, synchronize its body with this file, and commit the brief checkpoint. After Sol PASS and
remote green evidence, synchronize and close #340. #338 may cite #340 as resolution of its terminal
fuzz-delivery blocker but must not claim a fourth attempt or PASS from this issue.

## Evidence

Sol-high briefing classified the failure from PR #339's two fuzz jobs as an independent-workspace
lock integration omission. The existing locked preflight is sufficient and correctly failed before
execution; generic lockfile policy machinery would add ceremony without discriminating a new claim.

## Implementation and Sol-high review evidence

Checkpoint `022c50e3` changes only `fuzz/Cargo.lock`. Cargo generated the new resolution and then
restored the unrelated floating `cpufeatures` package to its prior `0.3.0` version through
`cargo +1.97.1 update --manifest-path fuzz/Cargo.toml -p cpufeatures --precise 0.3.0`; comparison
with parent `6322d222` shows no version change among packages already present in the old lock. The
diff removes the obsolete TOML/Serde/proc-macro parser closure and adds exact `jstrict 0.14.0` plus
only its resolver-required closure.

`cargo +1.97.1 check --locked --manifest-path fuzz/Cargo.toml --bins` passes all eight declared
fuzz binaries. With CI's pinned `cargo-fuzz 0.13.2` and `nightly-2026-08-20`, `session_parse` at
seed `557074001` and `session_compile` at seed `557074002` each reported `#10000 DONE` and
`Done 10000 runs` against `fixtures/session/v1/canonical.json`. The unchanged
`scripts/run-protocol-fuzz.sh` passed all four fixed 10,000-run protocol campaigns; its evidence
records 40,000 executions with no crash. Session policy, workspace policy and `git diff --check`
all pass. No benchmark or live-browser qualification was run.

Fresh Sol-high adversarial review returned PASS for `022c50e3`: exact-path scope, lock graph,
pre-existing package versions, all eight locked bins, supplied fixed-count evidence and protocol
evidence were independently inspected. Remote closure still requires both existing fuzz jobs to
pass on the pushed candidate; until then this is local accepted evidence only.
