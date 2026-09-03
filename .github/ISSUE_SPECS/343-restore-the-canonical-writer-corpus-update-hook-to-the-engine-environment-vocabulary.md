# Restore the canonical-writer corpus update hook to the engine environment vocabulary

## Objective

Restore the repository-wide environment-name contract for the crate-local canonical-writer corpus
update hook. Rename its two source spellings to the sole `MISO_ENGINE_` prefix, document that exact
crate-local hook in the authoritative vocabulary, and prove the existing vocabulary checker rejects
the retired spelling. This is a bounded CI-integration successor; it does not change canonical JSON,
the corpus bytes, Session V1 behavior, or issue #342's accepted product timing gate.

## Baseline and cause

The exact baseline is branch checkpoint `f11ad49e` on PR #339. In remote CI run `33711749622`,
`cargo test --locked --workspace --all-targets` passed, including issue #342's named strict `<1s`
maximum-document boot gate. The later command

```text
bash scripts/check-env-vocabulary.sh
```

failed because `crates/session/src/canonical.rs:361` reads
`MISO_UPDATE_CANONICAL_WRITER_CORPUS` and line 367 prints the same spelling in its regeneration
instruction. Rule 1 requires every tracked `MISO_*` identifier outside issue history and the
vocabulary's retired-name prose to continue `MISO_ENGINE_`. The hook is also absent from
`docs/ENGINE_ENV_VOCABULARY.md`'s crate-local vocabulary.

The checker correctly discriminated the defect; do not weaken or exempt it. Issue #338 has reached
its hard three-attempt stop, and #342's product gate passed at this candidate. Folding this rename
into either issue would be a disguised retry. The independently useful successor is one correctly
named and documented corpus-update hook.

## Smallest closable slice

Rename both live spellings, and only those spellings, to
`MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS`:

- the `std::env::var_os` lookup; and
- the test failure's operator/regeneration instruction.

Add that exact name to the crate-local hook description in `docs/ENGINE_ENV_VOCABULARY.md` and state
that it rewrites the Rust-authored canonical-writer corpus before the test compares the regenerated
bytes with the checked fixture. It remains crate-local, so it does not become a tool/script table
row: the checker's bidirectional table rule intentionally applies only under `tools/` and
`scripts/`.

### Allowed paths

- `.github/ISSUE_SPECS/343-restore-the-canonical-writer-corpus-update-hook-to-the-engine-environment-vocabulary.md`
- `crates/session/src/canonical.rs`
- `docs/ENGINE_ENV_VOCABULARY.md`

No other tracked path may change.

### Forbidden scope

- changing the canonical-writer implementation, field ordering, float/string encoding, corpus
  schema, generated cases, checked corpus bytes, fixture path or comparison behavior;
- retaining an alias or fallback for `MISO_UPDATE_CANONICAL_WRITER_CORPUS`, accepting multiple
  names for the same fact, or exempting `crates/session/src/canonical.rs` from the vocabulary scan;
- renaming the hook to the `MISO_ENGINE_REPIN_*_CORPUS` family: those existing hooks suppress a
  digest comparison and print scalar pins, while this hook deliberately writes a complete
  Rust-authored JSON corpus before exact comparison;
- changing `scripts/check-env-vocabulary.sh`, `scripts/test-env-vocabulary.sh`, CI workflows,
  environment-prefix rules, tool/script table semantics, dependencies or lockfiles;
- changing issue #338 or #342 product code, tests, evidence, thresholds, profiles or timing policy;
  and
- rerunning the #338 descriptive benchmark, a local full-workspace/timed #342 gate, live browser
  qualification, package qualification or unrelated target matrices.

## Objective gates

1. `crates/session/src/canonical.rs` contains exactly two
   `MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS` spellings: the environment lookup and its matching
   operator instruction. No live tracked path outside issue-history prose contains
   `MISO_UPDATE_CANONICAL_WRITER_CORPUS`, and no compatibility alias exists.
2. `docs/ENGINE_ENV_VOCABULARY.md` names
   `MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS` once in its crate-local hook vocabulary and
   accurately distinguishes its write-and-compare behavior from the existing
   `MISO_ENGINE_REPIN_*_CORPUS` digest-printing family. It is not added to a tool/script table.
3. `bash scripts/check-env-vocabulary.sh` passes on the candidate and still scans crate sources.
   As the exact adversarial mutation, restore either live source occurrence to
   `MISO_UPDATE_CANONICAL_WRITER_CORPUS` in a scratch copy, run the unchanged checker against that
   copy, observe `identifier outside the MISO_ENGINE_ prefix`, and discard the scratch copy. The
   captured run-`33711749622` failure is the baseline negative proof; no new checker is required.
4. `bash scripts/test-env-vocabulary.sh` passes every existing prefix, undocumented-name,
   unused-row, missing-vocabulary and synonym mutation. No exemption, scan-root or rule changes are
   made to obtain green.
5. The ordinary focused corpus test passes without the hook:

   ```text
   cargo +1.97.1 test --locked -p session canonical_writer_corpus_is_rust_generated_and_current
   ```

   Then one invocation with
   `MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS=1` passes the same test and leaves the checked corpus
   byte-identical and the worktree clean. The retired name does not update the corpus.
6. `cargo +1.97.1 fmt --all -- --check`, `bash scripts/check-session-policy.sh`,
   `bash scripts/check-workspace-policy.sh` and `git diff --check` pass. The exact-path diff and
   status contain only the three allowed paths.
7. No local full-workspace test is run, because it would unnecessarily rerun #342's already-passed
   timed gate. No benchmark, live browser, package or target qualification is rerun, and no product
   performance or canonical-byte claim is changed.
8. After the single coherent PR #339 update, the unchanged remote `engine qualification` job passes
   both the workspace tests and `bash scripts/check-env-vocabulary.sh` at the same exact candidate.
   Its unavoidable execution of the already-green #342 test is delivery confirmation, not a retry
   or replacement for #342's accepted evidence. All required checks must be green before closure.

## Review and delivery

This issue gets one implementation attempt and one fresh Sol-high adversarial review. HOLD rather
than broadening into vocabulary-policy redesign, corpus regeneration or product changes if the
three-file correction exposes another independent defect.

Keep the work on `codex/batch-338-canonical-json` and deliver it in PR #339 as a distinct
`fix(#343)` checkpoint. In CI-conscious mode, commit the exact three-path tranche locally, run only
the proportional non-timing gates above, then include it in one coherent PR update. Do not request
an unchanged CI rerun, force-push or manufacture a CI commit.

Before implementation, create the matching GitHub issue with this exact title, verify it receives
number 343, synchronize its body with this file, and commit the brief checkpoint. After Sol PASS
and the accepted evidence commit is upstream, require the fresh remote candidate to pass the
unchanged environment vocabulary gate, synchronize the issue evidence, and close #343. Issue #338
may cite #343 as resolution of this terminal CI integration omission but cannot treat it as a
fourth #338 attempt; issue #342 remains independently accepted and unchanged.

## Brief evidence and decision record

Sol-high briefing inspected checkpoint `f11ad49e`, remote run `33711749622`, both live source
occurrences, the complete environment checker and mutation script, and the authoritative vocabulary.
It ruled that the exact `MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS` rename plus crate-local
documentation is the smallest correction. The existing checker already catches the defect exactly,
so adding another policy mechanism would add ceremony without discriminating a new claim.

## Implementation and Sol-high review evidence

Checkpoint `a8a3b09f` changes only `crates/session/src/canonical.rs` and
`docs/ENGINE_ENV_VOCABULARY.md`. The source lookup and operator instruction now use exactly
`MISO_ENGINE_UPDATE_CANONICAL_WRITER_CORPUS`; the authoritative vocabulary documents its
crate-local write-before-byte-compare behavior and distinguishes it from the digest-printing
`MISO_ENGINE_REPIN_*_CORPUS` family without adding a tool/script table row.

The unchanged vocabulary checker passed with 99 documented tool/script names and the sole
`MISO_ENGINE_` prefix. In a discarded scratch copy, restoring one retired live spelling made the
same checker exit 1 with `identifier outside the MISO_ENGINE_ prefix`; the complete existing
vocabulary mutation suite also passed. The focused corpus test passed both without the hook and in
one invocation with the new hook. The checked corpus remained byte-identical before and after,
with SHA-256 `5faaf8f3994c1d910115c0a1c5b3b4ea4b583f7e82e3a83ff90434f3c666639d`, and did
not dirty the worktree. Formatting, session/workspace policies and diff checks passed. No full
workspace test, benchmark, live-browser run, package qualification or target matrix was run
locally.

Fresh Sol-high adversarial review returned PASS at `a8a3b09f` with no blocking findings. It
independently confirmed the exact-path diff, occurrence counts, unchanged hook behavior and corpus
blob, truthful vocabulary placement, unchanged policy scripts, supplied negative mutation and
focused test evidence, and forbidden-scope compliance. Remote closure still requires the pushed
evidence candidate to pass the unchanged engine qualification and environment-vocabulary gate.
