# Luna1 #495 execution inventory

All commands ran in `/home/bl/misofm/engine-cp4-bind-coverage`, from source base `88d56f9e78496a261a5b7d22837afb3352401c8a` plus the uncommitted `crates/graph/src/lib.rs` tranche. The logs are immutable; their command/CWD/source preambles were printed to the tool output rather than piped into the files.

- `/tmp/495-luna1-focused-debug.log`: `PATH=/home/bl/.cargo/bin:$PATH cargo test --locked -p graph --lib tests::binding_coverage_preserves_validation_and_ownership --exact`; status 1, CLI syntax error (`--exact` belonged after `--`).
- `/tmp/495-luna1-focused-release.log`: release equivalent; status 1, same CLI syntax error.
- `/tmp/495-luna1-focused-debug-corrected.log`: `PATH=/home/bl/.cargo/bin:$PATH cargo test --locked -p graph --lib tests::binding_coverage_preserves_validation_and_ownership -- --exact`; status 0, 1 passed. Before final empty-node fixture correction.
- `/tmp/495-luna1-focused-release-corrected.log`: corrected release equivalent; status 0, 1 passed. Before final empty-node fixture correction.
- `/tmp/495-luna1-focused-debug-final.log`: corrected debug command; status 0, 1 passed. Exact final source.
- `/tmp/495-luna1-focused-release-final.log`: corrected release command; status 0, 1 passed. Exact final source.
- `/tmp/495-luna1-graph-debug.log`: `PATH=/home/bl/.cargo/bin:$PATH cargo test --locked -p graph --lib`; status 0, 57 passed. Before final empty-node fixture correction.
- `/tmp/495-luna1-graph-release.log`: release equivalent; status 0, 57 passed. Before final empty-node fixture correction.
- `/tmp/495-luna1-clippy.log`: `PATH=/home/bl/.cargo/bin:$PATH cargo clippy --locked -p graph --all-targets --all-features -- -D warnings`; status 0, existing unrelated `clippy.toml` invalid-path warnings. Before final empty-node fixture correction.
- `/tmp/495-luna1-clippy-strict.log`: `RUSTFLAGS='-D warnings' PATH=/home/bl/.cargo/bin:$PATH cargo clippy --locked -p graph --all-targets --all-features -- -D warnings`; status 101 because overwriting workspace flags removed required AVX2/FMA.
- `/tmp/495-luna1-clippy-strict-corrected.log`: `RUSTFLAGS='-C target-feature=+avx2,+fma -D warnings' PATH=/home/bl/.cargo/bin:$PATH cargo clippy --locked -p graph --all-targets --all-features -- -D warnings`; status 0, same existing unrelated warnings. Exact final source.
- `/tmp/495-luna1-fmt.log`: `PATH=/home/bl/.cargo/bin:$PATH cargo fmt --all --check`; status 0. Before final fixture correction.
- `/tmp/495-luna1-diff-check.log`: `git diff --check`; status 0. Before final fixture correction.
- `/tmp/495-luna1-graph-policy.log`: `bash scripts/check-graph-policy.sh`; status 0. Before final fixture correction.

The final correction only changed the new test's empty node/source fixture; production logic did not change after the earlier full suites. No command metadata was retroactively added to the immutable logs. No source changes or reruns were made for this inventory.
