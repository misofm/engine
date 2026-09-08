# Issue 539 integrated-head gates

This bounded record covers the no-commit integration of `origin/main` at
`9e113be98cf31c1eaf4297b0a031518244b71c33` into the accepted pushed source
head `979f79ccaa17c70e97df626724d02cf4289ea702`. The merge completed
automatically with no conflicts and remains uncommitted for the root agent.

The accepted production and test paths are unchanged from attempt 2
(`82a42b5221a0a40d165bbb04b33f113ad5329b01`). Their SHA-256 values are in
`source-identity.txt`. `gate-results.tsv` records every retained command
status. The corpus JSONL, host-web module hashes, and lowering summary are
copied here; the large raw host-web disassemblies remain at the temporary
paths recorded in `host-web-lowering.txt`.

Commands run:

```text
cargo test --locked -p true-peak-limiter
cargo test --locked -p true-peak-limiter --test mono_collapse
cargo test --locked --release -p true-peak-limiter --test mono_collapse
cargo test --locked --release -p true-peak-limiter --lib stationary_dispatch_matches_runtime_oracle_and_observes_selected_body
cargo test --locked --release -p true-peak-limiter --test allocation
cargo test --locked -p true-peak-limiter --test determinism
cargo clippy --locked -p true-peak-limiter --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
bash scripts/check-realtime-policy.sh
bash scripts/check-lane-policy.sh
bash scripts/check-workspace-policy.sh
bash scripts/check-env-vocabulary.sh
bash scripts/run-wasm-gates.sh /tmp/issue539-wasm-gates
env CARGO_TARGET_DIR=/tmp/issue539-web-scalar RUSTFLAGS='-C target-feature=-simd128' cargo build --locked --release --target wasm32-unknown-unknown -p host-web
env CARGO_TARGET_DIR=/tmp/issue539-web-simd128 RUSTFLAGS='-C target-feature=+simd128' cargo build --locked --release --target wasm32-unknown-unknown -p host-web
wasm-objdump -x/-d on each host-web module, with scalar SIMD-opcode absence and SIMD128 opcode-presence checks
bash scripts/check-wasm-realtime-atomics.sh
bash scripts/check-capi-abi.sh
git diff --cached --check
git diff --check
```

All listed commands returned zero. The full limiter suite and the Wasm gate
reported the existing 139-case/349-comparison corpus with zero mismatches.
No benchmark, timing/capture, AudioWorklet builder, browser qualification, pin
change, commit, or push was performed.
