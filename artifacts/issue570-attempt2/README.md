# Issue #570 attempt 2 bounded correction evidence

Source is frozen at pushed `5879b6892c15405f304bd099e550448280606d14` in
`/home/bl/misofm/engine-cp4-borrowed-sorted-bind`. The only source delta after the attempt 1
checkpoint `64f3365f` is the exact rustfmt-requested wrap of the `builtin_bank_members` declaration
in `crates/graph/src/lib.rs`; no production or test path was otherwise edited.

The five bind-local transient trees remain absent: the source census records zero `BTreeSet`
mentions and zero `.clone()` calls inside `bind_optional_source_set`, with five
`Vec::with_capacity` validation families. The duplicate-binding focused test passed once in both
debug and release. `cargo fmt --all --check`, `git diff --check
c8951bfe23164086ca1ce34b456ab5d600fd4a13...HEAD`, and `bash scripts/check-graph-policy.sh` all
passed.

Attempt 1 evidence, carried forward from `artifacts/issue570-attempt1`, recorded full graph
library suites at exactly 60 passed / 0 failed in both debug and release and strict graph Clippy at
exit 0. Its only failed gate was the pre-correction formatting check. The later ad hoc exact-diff
assertion also exited 1 because its checker incorrectly expected the removed split line to equal the
joined line; this was a checker-command mistake, not a product gate. Direct root inspection then
verified the one expected formatting hunk, followed by the green bounded tests here.

Each current gate has separate command, context, stdout, stderr, and numeric exit payloads. Context
captures UTC, cwd, HEAD, status, the graph source SHA-256, and `Cargo.lock` SHA-256 before the gate.
Raw payloads are losslessly gzip-compressed with deterministic gzip headers. `sha256sums.txt`
covers every retained payload except itself.

No broader gates, artifact work, commit, push, or GitHub operation was performed in this attempt.
