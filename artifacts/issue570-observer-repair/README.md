# Issue #570 observer-repair exact-head evidence

This evidence is for pushed HEAD `6913563c661d9faaffd6c9b25d57fe04a8d37179` in
`/home/bl/misofm/engine-cp4-borrowed-sorted-bind`. The current tracked tree was clean before the
gates. Each gate has separate command, context, stdout, stderr, and numeric-exit payloads;
contexts record UTC, cwd, HEAD, status, and Git/SHA-256 hashes for the graph source and #570 spec
immediately before execution. Raw stdout/stderr are losslessly gzip-compressed.

| Exact-head gate | Result | Evidence |
| --- | --- | --- |
| Focused debug `borrowed_sorted_bind_validation_preserves_set_semantics` | PASS, 1 passed / 0 failed | [command](focused-debug.command.txt.gz), [context](focused-debug.context.txt), [stdout](focused-debug.stdout.gz), [stderr](focused-debug.stderr.gz), [exit](focused-debug.exit.txt) |
| Focused release `borrowed_sorted_bind_validation_preserves_set_semantics` | PASS, 1 passed / 0 failed | [command](focused-release.command.txt.gz), [context](focused-release.context.txt), [stdout](focused-release.stdout.gz), [stderr](focused-release.stderr.gz), [exit](focused-release.exit.txt) |
| `cargo fmt --all --check` | PASS, exit 0 | [command](fmt.command.txt.gz), [context](fmt.context.txt), [stdout](fmt.stdout.gz), [stderr](fmt.stderr.gz), [exit](fmt.exit.txt) |
| `git diff --check c8951bfe23164086ca1ce34b456ab5d600fd4a13...HEAD` | PASS, exit 0 | [command](diff-check.command.txt.gz), [context](diff-check.context.txt), [stdout](diff-check.stdout.gz), [stderr](diff-check.stderr.gz), [exit](diff-check.exit.txt) |

The graph production source bytes are identical to the source captured at `f347ec06b58b004749633ba602b599f4011aa7c1` in
[final-source evidence](../issue570-final-source/README.md): that source had Git hash
`84b79468208432ce10b91e87203a2217a535cbc5` and SHA-256
`67fca95df25a4dcf5519e59a152336a9df4aa8c96fa76e329b50e7a642267b4d`. The only later source delta
is the ten-line test repair/rebind assertion in `crates/graph/src/lib.rs`; the production bind
implementation is unchanged.

The final-source evidence carries forward full debug/release graph suites at 61/61, strict Clippy
exit 0, and graph-policy exit 0. Those are historical results at `f347ec06`; they are carried
forward for this repair and are not relabeled as exact-head reruns. The exact-head gates above are
the focused tests, format check, and diff check executed at `6913563c`.

The observer fixture retains both returned-owner length assertions, removes exactly one returned
caller observer, and successfully rebinds the returned plan and bindings. No artifact build,
benchmark, commit, push, or GitHub operation was performed. `sha256sums.txt` covers every retained
payload except itself.
