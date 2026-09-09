# Extract graph program unit tests into their module file

GitHub: https://github.com/misofm/engine/issues/671

Parent: #559 (lane A, RT17). Coordination: #560. Base:
`7d16d9c9752c9ac2d31e69008fe075df86ce3c26`.

## Problem

`crates/graph/src/program.rs` combines 852 lines of production implementation
with an inline `#[cfg(test)] mod tests` that runs from line 853 to the 2,641-line
file's end. The inline module currently contains 13 tests. RT17 calls for concrete
module splits; this issue owns only the existing `program::tests` extraction, not
the rest of RT17 or any runtime optimization.

## Smallest closable slice

Replace the inline module declaration and body with:

```rust
#[cfg(test)]
mod tests;
```

Move the existing module body into `crates/graph/src/program/tests.rs`, removing
only the four-space indentation imposed by the former inline module. Preserve the
module path `graph::program::tests`, private `super::*` access, imports, helpers,
test names, test count, assertions, literals, and order. Formatting may make only
the mechanical layout changes required by `cargo fmt`.

This closes only the `program.rs` test-module extraction slice. The separate
large inline tests in `graph/src/lib.rs`, the combined `builtins/src/lib.rs`, and
the rest of RT17 remain open.

## Exact ownership

This issue owns only:

- `.github/ISSUE_SPECS/671-extract-graph-program-unit-tests.md`;
- `crates/graph/src/program.rs`;
- `crates/graph/src/program/tests.rs`;
- concise #559/#560 coordination rows written serially by root.

It owns no production behavior change, dependency, public API, allocation,
arithmetic, scheduling, policy, workflow, lockfile, benchmark, artifact,
AudioWorklet action, or pin. It must not touch any retained failed worktree,
branch, history, or temporary evidence from #643/#647/#649/#651/#656/#660/#668,
or lane-B #669/#670 paths. Never inspect a legacy engine.

## Frozen behavior and proof

The base production prefix through the closing `compile_program` brace must remain
byte-identical. The only bytes removed from `program.rs` are the inline test
module wrapper and body; the only replacement is the external test-module
declaration. The new file must equal the old inline body after removing exactly
one four-space prefix from every nonblank body line. The old module's final brace
is not copied.

Before editing, record exact HEAD/upstream/main, clean tracked and untracked
status, the SHA-256 of `program.rs`, the marker byte offset, 13 test names, and a
fresh absent non-symlink `/tmp/issue671-graph-program-tests-evidence` path. Persist
the base file there before changing source. No generated payload enters Git.

After editing, prove the exact transform with one script that:

1. reads the persisted base file;
2. locates the unique final `#[cfg(test)]\nmod tests {\n` marker;
3. requires the old file to end in the module's closing `}\n`;
4. requires current `program.rs` to equal the byte-identical base prefix plus
   `#[cfg(test)]\nmod tests;\n`;
5. constructs the expected new file by removing exactly four leading spaces from
   each nonblank old body line and requires byte identity with `tests.rs`;
6. requires the same ordered 13 `#[test]` function names in the old and new body;
7. rejects `include_str!`, `include_bytes!`, `file!`, `line!`, `#[path`, or a new
   non-test module declaration in either changed source path.

Any mismatch stops the attempt. Do not repair or rerun a failed gate in the same
attempt.

## Objective gates

After Astra LOW scope PASS, one Luna HIGH executor performs one mechanical source
tranche and runs these gates once in order, stopping at the first nonzero status:

1. the exact-transform script above;
2. `cargo test --locked -p graph --lib`;
3. `cargo test --locked --release -p graph --lib`;
4. `cargo clippy --locked -p graph --all-targets -- -D warnings`;
5. `cargo fmt --all -- --check`;
6. `git diff --check`;
7. `bash scripts/check-workspace-policy.sh`;
8. a final exact-path census and clean-before-edit provenance check.

Persist literal commands, separate streams, numeric statuses, source identities,
and a self-excluding manifest in the fresh temporary evidence directory. Commit
only the three owned repository paths. The root agent checkpoints immediately;
corrections need an explicit next-attempt scope. Three failed attempts hard-stop
without a disguised fourth retry.

Acceptance requires the exact transform, all 13 tests at the preserved module
path, debug and release package passes, strict Clippy, formatting, whitespace,
workspace policy, and an exact-path diff. No benchmark, timing, instruction-count,
resource-saving, or performance claim is authorized.

## Review and delivery

Astra LOW reviews scope before implementation and independently reproduces the
transform, test identity, statuses, source unchanged outside the test module, and
manifest after the checkpoint. Root then records **no artifact applicability**:
the changed implementation bytes are all under `cfg(test)` and the production
prefix is byte-identical, so ordinary native and Wasm artifacts receive no changed
input. Lane B alone owns AudioWorklet qualification and pins.

Delivery requires exact-head/current-main PR-readiness review, required PR
qualification, a fresh guarded merge review, ordered-parent merge verification,
post-main qualification, GitHub/tracker synchronization, and removal only of the
clean delivered worktree. A failed or stopped worktree and its evidence remain
preserved.
