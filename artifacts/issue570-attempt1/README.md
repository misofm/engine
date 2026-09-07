# Issue #570 attempt 1 qualification evidence

Source was frozen at `64f3365f259250483470fd63ce904a13a895b967` in
`/home/bl/misofm/engine-cp4-borrowed-sorted-bind`. This evidence covers the one-file
implementation tranche in `crates/graph/src/lib.rs`; no production or test file was edited by the
qualification owner.

Each gate has separate losslessly gzipped command, context, stdout, stderr, and exit payloads.
Context captures UTC, cwd, HEAD, status, `crates/graph/src/lib.rs` SHA-256, and `Cargo.lock`
SHA-256 immediately before the command.

The five named focused tests passed exactly `1 passed, 0 failed` in both debug and release. Full
graph library tests passed `60 passed, 0 failed` in both debug and release. Strict graph clippy
passed with exit 0 (existing clippy configuration emitted warnings about unreachable allow-list
entries). The formatting gate was an authentic finite failure: `cargo fmt --all --check` exited 1
because rustfmt would join the wrapped `builtin_bank_members` declaration at
`crates/graph/src/lib.rs:1230`. No retry or source formatting edit was made. The subsequent diff
and graph-policy gates were not run because the evidence sequence stops on a gate failure.

`sha256sums.txt` covers every retained payload except itself.
