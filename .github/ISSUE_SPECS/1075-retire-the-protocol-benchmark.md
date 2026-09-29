# Retire the protocol benchmark (R9: only real host paths are benchmarked)

Owner ruling R9 (`docs/rulings/engine-footprint-2026-09-28.md`): only real host paths are benchmarked, namely the native console `--step` rows and the V8 rows on the shipped artifact. The protocol benchmark is neither. #1062's verifier also found its runner dead: `scripts/run-protocol-benchmark.sh` needs a scalar wasm artifact that #1062 retired, and nothing has built its inputs since before this sprint; CI only runs `scripts/test-protocol-benchmark.sh` against bad arguments.

## Smallest closable slice

Remove the `bench protocol` subject (`tools/bench/src/protocol.rs`, about 1,660 lines), its FlatBuffers comparison schema (`tools/bench/protocol_benchmark.fbs`) and the `flatbuffers` dependency it alone needs, `scripts/run-protocol-benchmark.sh`, `scripts/test-protocol-benchmark.sh`, the two `protocol-benchmark*-validator.jq` files, and the CI step that runs the test script. Update `check-bench-policy.sh`'s `timed_subjects` ratchet honestly, the bench unsafe-owner list if `tools/bench/src/protocol.rs` is on it, and #1027's reachability lint fixtures that name these scripts.

Keep: `crates/protocol` (the C ABI's command path), its fuzz targets and `run-protocol-fuzz.sh`, `check-protocol-wasm-parity.sh`, the protocol allocation audit, and `check-protocol-control-policy.sh`.

## Gates

1. `cargo check --workspace --all-targets --all-features`; clippy and fmt; `bench` tests; console digests unchanged.
2. `check-bench-policy.sh` and its suite, the reachability lint and its suite, the routing tests, every policy script.
3. `cargo tree` shows `flatbuffers` gone from the workspace, or names what still needs it.
