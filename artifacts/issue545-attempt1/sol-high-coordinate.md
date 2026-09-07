Attempt 1 is green and paused for root checkpoint. No Sol final review was launched.

Common evidence:

- CWD: `/home/bl/misofm/engine-tooling-orphan-cleanup`
- HEAD before/after: `f1d56f63c1f6a0a69652542b81e5079b0d0da63f`
- No commit, push, staging, GitHub action, or unrelated edit.

Deleted exactly:

- `crates/graph-compiler/src/bin/rack_fixture.rs`
- `fixtures/rack/v1/MANIFEST.tsv`
- `fixtures/rack/v1/cases.toml`
- `fixtures/rack/v1/input.f32le`
- `fixtures/rack/v1/scalar-expected.f32le`

Results:

- `cargo metadata --no-deps --format-version 1`: status 0 before and after.
  - Workspace members: 45 → 45.
  - Removed targets: `["rack_fixture"]`.
  - Added targets: `[]`.
- Focused live-consumer scan: status 1, empty stdout/stderr; no genuine current product consumer found.
- Historical references remain untouched.
- Protected hashes remained identical:
  - `MANIFEST.tsv`: `2d6b8c4b11bb00a17185d7777300194bf53ab30d86cf581a55886f07c5273985`
  - `workloads.toml`: `1f67ed9960e5a6728f02442b65af70704957d5f6056865d8b44555637273188d`
  - checker: `483fd69c805b0d3c37c1bc3ce93420313808e1d9aeada3a8180abcfe50bf4d70`
- `CARGO_TARGET_DIR=/tmp/issue545-target cargo test --locked -p graph-compiler`: status 0; 83 tests passed.
- Direct `scripts/check-workspace-policy.sh`: status 126 because the file lacks its executable bit.
- `bash scripts/check-workspace-policy.sh`: status 0; `workspace policy: ok`.
- `cargo fmt --all -- --check`: status 0.
- `git diff --check`: status 0.
- In-worktree `target/`: absent.
- `fixtures/rack/v1/`: absent.

Final diff:

```text
crates/graph-compiler/src/bin/rack_fixture.rs | 216 --------------------------
fixtures/rack/v1/MANIFEST.tsv                 |   4 -
fixtures/rack/v1/cases.toml                   |  14 --
fixtures/rack/v1/input.f32le                  | Bin 12288 -> 0 bytes
fixtures/rack/v1/scalar-expected.f32le        | Bin 1024 -> 0 bytes
5 files changed, 234 deletions(-)
```

Raw stdout/stderr and metadata are preserved under `/tmp/issue545-*`, including:

- `/tmp/issue545-before-metadata.json`
- `/tmp/issue545-after-metadata.json`
- `/tmp/issue545-graph-test.stdout`
- `/tmp/issue545-graph-test.stderr`
- `/tmp/issue545-policy-bash.stdout`
- `/tmp/issue545-policy-bash.stderr`
- `/tmp/issue545-fmt.stdout`
- `/tmp/issue545-fmt.stderr`

Two evidence-command corrections were recorded: unsupported `jq --argfile` was replaced with `--slurpfile`, and a substring false positive matching `track_fixture` was replaced by a token-boundary scan. Both corrected checks passed. The checkpoint is ready for root ownership; #546/#547 and CP20/#543 were not touched.