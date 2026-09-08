# Issue #570 final-source qualification evidence

Source is frozen at pushed HEAD `f347ec06b58b004749633ba602b599f4011aa7c1` in
`/home/bl/misofm/engine-cp4-borrowed-sorted-bind`. No production, test, spec, or policy file was
modified during this qualification pass. Root preserves the evidence under this directory.

Each gate has separate command, context, stdout, stderr, and numeric-exit payloads. Context records
UTC, cwd, HEAD, worktree status, and Git/SHA-256 hashes for `crates/graph/src/lib.rs` and the #570
spec immediately before the command. Raw stdout/stderr are losslessly gzip-compressed with
reproducible gzip headers.

| Gate | Result | Evidence |
| --- | --- | --- |
| Focused debug test | PASS, 1 passed / 0 failed / 60 filtered | [command](focused-debug.command.txt.gz), [context](focused-debug.context.txt), [stdout](focused-debug.stdout.gz), [stderr](focused-debug.stderr.gz), [exit](focused-debug.exit.txt) |
| Focused release test | PASS, 1 passed / 0 failed / 60 filtered | [command](focused-release.command.txt.gz), [context](focused-release.context.txt), [stdout](focused-release.stdout.gz), [stderr](focused-release.stderr.gz), [exit](focused-release.exit.txt) |
| Full debug graph library | PASS, 61 passed / 0 failed / 0 ignored | [command](full-debug.command.txt.gz), [context](full-debug.context.txt), [stdout](full-debug.stdout.gz), [stderr](full-debug.stderr.gz), [exit](full-debug.exit.txt) |
| Full release graph library | PASS, 61 passed / 0 failed / 0 ignored | [command](full-release.command.txt.gz), [context](full-release.context.txt), [stdout](full-release.stdout.gz), [stderr](full-release.stderr.gz), [exit](full-release.exit.txt) |
| Strict graph Clippy | PASS, exit 0 | [command](clippy.command.txt.gz), [context](clippy.context.txt), [stdout](clippy.stdout.gz), [stderr](clippy.stderr.gz), [exit](clippy.exit.txt) |
| `cargo fmt --all --check` | PASS, exit 0 | [command](fmt.command.txt.gz), [context](fmt.context.txt), [stdout](fmt.stdout.gz), [stderr](fmt.stderr.gz), [exit](fmt.exit.txt) |
| `git diff --check c8951bfe...HEAD` | PASS, exit 0 | [command](diff-check.command.txt.gz), [context](diff-check.context.txt), [stdout](diff-check.stdout.gz), [stderr](diff-check.stderr.gz), [exit](diff-check.exit.txt) |
| Graph policy | PASS, exit 0 | [command](graph-policy.command.txt.gz), [context](graph-policy.context.txt), [stdout](graph-policy.stdout.gz), [stderr](graph-policy.stderr.gz), [exit](graph-policy.exit.txt) |
| Bind-block source census | PASS, exit 0 | [command](source-census-corrected.command.txt.gz), [context](source-census-corrected.context.txt), [stdout](source-census-corrected.stdout.gz), [stderr](source-census-corrected.stderr.gz), [exit](source-census-corrected.exit.txt) |

The first census parser used over-escaped regular expressions and is retained under [source-census.*](source-census.command.txt) as non-credit diagnostic evidence. The corrected literal census below is authoritative. It reports zero `BTreeSet`, `GraphNodeId::clone`, and `.clone()` tokens in the
`bind_optional_source_set` block; five capacity-sized vectors; five sorts; four deduplications; one
builtin-member binary search; one observer adjacent-value check; and borrowed node references for
supplied nodes, source claims, and observers.

The prior authentic formatting failure and bounded checker evidence are linked from
[attempt 1](../issue570-attempt1/README.md) and [attempt 2](../issue570-attempt2/README.md).
An initial wrapper mistake invoked `clippy` without `cargo` and exited 127 before any product gate;
its raw non-credit diagnostic is retained under [clippy/cargo.exit.txt](clippy/cargo.exit.txt) with
its split output and context. The exact requested Clippy command then passed as recorded above.

`sha256sums.txt` covers every retained payload except itself. No artifact build, benchmark, commit,
push, or GitHub operation was performed.
