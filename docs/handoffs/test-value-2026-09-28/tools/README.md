# Audit tools

These are the scripts behind the 2026-09-28 test-value audit, kept so that the issue gates can be
re-run. None of them is a CI gate. Run them from a checkout, with a scratch directory outside the
repository.

## Mutation equivalence (issue gate 1)

```bash
export TVA_DIR=$HOME/tva            # scratch; cargo-mutants 27.x on PATH
tools/run-mutants.sh compressor 5 10
tools/run-mutants.sh graph-compiler 5 10
tools/run-mutants.sh host-core 5 10 --features control-provider,test-support --shard 0/4 --sharding round-robin
tools/run-mutants.sh parametric-eq 6 12 --shard 0/4 --sharding round-robin
# the three positional arguments are: package, parallel jobs, jobserver tasks
python3 tools/parse_mutants.py "$TVA_DIR/mut/out-compressor" compressor
python3 tools/analyze_matrix.py compressor "$TVA_DIR/mut/out-compressor/matrix.json" --exclude-file src/corpus.rs
python3 tools/dominance.py compressor "$TVA_DIR/mut/out-compressor/matrix.json" --exclude-file src/corpus.rs
```

- Run once on the base commit and once on the change, with identical arguments. The gate holds
  when the set of `CaughtMutant` keys in `matrix.json` does not shrink.
- `OUTNAME=<dir>` keeps two runs apart.
- `--iterate` with `--cargo-test-arg=--test --cargo-test-arg=<binary>` re-tests only the survivors
  against one extra binary. This is how the audit showed that the 65,537-track scale tests catch
  nothing new.

## Historical bugs (issue gate 2)

`revert.py <tree> <bug> apply|restore` re-injects one recorded bug (966, 970, 994 or 1015) into a
**scratch copy** of the tree, never the checkout itself. For example:

```bash
git archive HEAD | tar -x -C "$TVA_DIR/tree"
python3 tools/revert.py "$TVA_DIR/tree" 970 apply
(cd "$TVA_DIR/tree" && CARGO_TARGET_DIR="$TVA_DIR/target" cargo test --locked <CI shard args> --no-fail-fast)
python3 tools/revert.py "$TVA_DIR/tree" 970 restore
```

The CI shard arguments are the `test-debug-a` and `test-debug-b` command lines in
`.github/workflows/qualification.yml`. Collect the `test … FAILED` lines.

What each bug turned red at `a9414c0c`:
- **#966:** the 9 tests in `bank_levels.rs`.
- **#970:** the 7 `collapse_arming.rs` reproducers, plus the `bank_levels.rs` randomized probe.
- **#994:** the 7 knee reproducers, plus the compressor's 3 `randomized_differential_*` tests.
- **#1015:** the 3 `stationary_subnormal` tests.

`revert.py` edits code by exact text match, so if a later refactor moves the lines it fails loudly
rather than silently injecting nothing. The recorded reverts are in each crate's `MUTATIONS.md`.

## Script reachability (issue 10's gate)

`reach.py <repo root> strict <tracked-files.txt> <out.json>` lists the files under `scripts/` that
no workflow or `package.json` script reaches, following non-comment mentions transitively. Make the
file list with `git ls-files > tracked-files.txt`. The audit found 26 such files at `a9414c0c`.

