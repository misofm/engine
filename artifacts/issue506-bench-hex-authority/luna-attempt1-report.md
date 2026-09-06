# #506 Luna attempt 1

Result: focused untimed implementation tranche is green.

Changed exactly these source paths:

- `tools/bench-support/src/digest.rs`
- `tools/bench/src/builtins.rs`
- `tools/bench/src/effect_interchange.rs`
- `tools/bench/src/graph.rs`

The existing borrowed-byte encoder is public as `bench_support::digest::hex` with its algorithm unchanged. Builtins finalized SHA-256 bytes use that encoder directly; one-shot hashing and the effect/graph text helpers use the existing `sha256_hex` authority. The effect raw `[u8; 32]` helper remains. Added the finite literal nibble vector and direct `abc` identity witnesses, including the builtins finalized-without-rehash assertion.

Validation:

- `cargo test --locked -p bench-support digest --lib` (debug): PASS, 5 tests.
- `cargo test --locked -p bench --bin bench` (debug): PASS, 35 tests.
- `cargo test --locked --release -p bench-support digest --lib` (release): PASS, 5 tests.
- `cargo test --locked --release -p bench --bin bench` (release): completed without failure.
- `cargo fmt --all -- --check`: PASS.
- `cargo clippy --locked -p bench-support -p bench --bin bench --all-targets -- -D warnings`: PASS.

The original debug-support evidence remains preserved in `/tmp/506-luna1-AJhrv7/`. The complete separate final captures are in `/tmp/506-luna1-final-v0rONn/`, with raw stdout/stderr, argv, cwd, whitelisted effective environment, HEAD, source SHA-256 list, and exit status for each command.

Final capture statuses (all zero): debug-support, debug-bench, release-support, release-bench, clippy, bench-policy, interchange-policy, and fmt.
