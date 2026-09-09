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

## Attempt 1 authorization

Astra LOW returned **SCOPE PASS** at exact clean pushed feature
`60e45a40c967ea870211fc63dd2412ef74b8229c`, current main
`7d16d9c9752c9ac2d31e69008fe075df86ce3c26`, and synchronized tracker
`536385715043376b440b54b59d9aff9bb3440465`. GitHub #671 matches the
numbered spec; #670/#671 are the two path-disjoint active slots. The marker and
EOF wrapper are unique, all 13 tests admit the exact deindent, and no prohibited
path/include/file/line construct exists.

Sole Luna HIGH executor `issue671_luna_impl` may persist the required clean-source
preflight, original file, names, and fresh-path observations; perform only the
exact two-source-path module extraction; append this spec; and execute the eight
frozen gates once in order with complete streams, statuses, and a verified self-
excluding manifest. Stop on the first failure. Mechanical formatting may not
relax the byte-transform gate. Root alone edits trackers and commits. No source
behavior, API, arithmetic, allocation, scheduling, artifact, AudioWorklet, pin,
or retained-state cleanup is authorized.

## Luna HIGH attempt 1 result

- Preflight was clean at `8c9df9b774830bcc25af12800eec2b539bb63041`; original `program.rs` and the ordered 13-test inventory were persisted under `/tmp/issue671-graph-program-tests-evidence` after confirming the path was absent and non-symlink.
- Gate 1, `python3 /tmp/issue671-graph-program-tests-evidence/exact-transform.py`: status `1` (the persisted proof path contained generated output rather than executable script text, producing a Python `SyntaxError`).
- Gates 2–8 were not run because the frozen sequence stops on the first failure. No same-attempt correction or rerun was performed.

## Astra LOW attempt-1 verdict and attempt-2 brief

Astra LOW returned **ATTEMPT-1 FAIL / SOURCE ASSESSMENT PASS** at exact clean
pushed head `711f04c6f2c16cd1121925a1f7663e9e2b0a0e19`, unchanged main
`7d16d9c9752c9ac2d31e69008fe075df86ce3c26`, and reconciled tracker
`b54c7bc55e966ffcd13f4d4faf7cc4e07fbc7a0a`. The retained original hash,
marker, ordered 13-test inventory, exact deindent transform, failed script bytes,
SyntaxError status, lack of gates 2-8, and self-excluding manifest reproduce.
Attempt 1 is consumed and supplies no gate credit. Its freshness observation is
persisted testimony because the present filesystem cannot reconstruct prior
absence.

Attempt 2 is qualification-only. Preserve the two source files byte for byte at:

- `program.rs` SHA-256
  `2f34607637b4785715d5aa31a787a9f94d6a63ec5ecd9ce2727799406a269b6c`;
- `program/tests.rs` SHA-256
  `64431ae1066d96232ded4200bd82b11f72b1b1dff6995999bda571ec00f552de`.

Preserve every attempt-1 evidence byte. Use only fresh absent non-symlink
`/tmp/issue671-graph-program-tests-attempt2-evidence`. Persist the absence checks
before creation. Before any gate, write the following literal script as
`exact-transform.py`; do not create it by redirecting the output of an execution.
Read it back, persist its SHA-256 and byte count, and require those bytes to remain
unchanged through the attempt:

```python
from hashlib import sha256
from pathlib import Path
import re

base = Path("/tmp/issue671-graph-program-tests-evidence/program.rs.base").read_bytes()
program = Path("crates/graph/src/program.rs").read_bytes()
tests = Path("crates/graph/src/program/tests.rs").read_bytes()
marker = b"#[cfg(test)]\nmod tests {\n"

def digest(value: bytes) -> str:
    return sha256(value).hexdigest()

assert digest(base) == "5d2575c6c4348e13481e33d71d53e797a2d79df628ca2044d4703ffc6885440f"
assert digest(program) == "2f34607637b4785715d5aa31a787a9f94d6a63ec5ecd9ce2727799406a269b6c"
assert digest(tests) == "64431ae1066d96232ded4200bd82b11f72b1b1dff6995999bda571ec00f552de"
assert base.count(marker) == 1
offset = base.index(marker)
body = base[offset + len(marker):]
assert body.endswith(b"}\n")
body = body[:-2]
lines = body.splitlines(keepends=True)
assert all((not line.strip()) or line.startswith(b"    ") for line in lines)
expected_tests = b"".join(line[4:] if line.strip() else line for line in lines)
assert program == base[:offset] + b"#[cfg(test)]\nmod tests;\n"
assert tests == expected_tests
name_pattern = re.compile(rb"(?m)^\s*#\[test\]\n\s*fn ([a-zA-Z0-9_]+)\(\) \{")
old_names = name_pattern.findall(body)
new_names = name_pattern.findall(tests)
assert old_names == new_names
assert len(old_names) == 13
for forbidden in (b"include_str!", b"include_bytes!", b"file!", b"line!", b"#[path"):
    assert forbidden not in program
    assert forbidden not in tests
print("PASS exact-transform")
print(f"marker_byte_offset={offset}")
print(f"test_count={len(old_names)}")
```

The source hashes must be checked before and after every command. Run all eight
original gates fresh, once and in order, carrying no attempt-1 or review credit.
Persist the literal script, its frozen identity, commands, separate streams,
statuses, source identities, and a verified self-excluding manifest under the
attempt-2 path. Any failed precondition, script error, source drift, or nonzero
gate stops without correction or retry. Only this spec may change in attempt 2;
root checkpoints before further work. A clean pushed amendment and fresh Astra
LOW scope PASS are required before sole Luna HIGH `issue671_luna_impl` resumes.
