# Issue 539 CP8 integrated-head gates

This bounded record covers the current-main integration already active in the
shared worktree: source head `2fd796597693a78577c2e5ee931404068a1ef067`,
`origin/main` and `MERGE_HEAD` `773682433ef451b89e5359fa8f722e1016c64fb3`,
and merge-base `9e113be98cf31c1eaf4297b0a031518244b71c33`. The merge has no
conflicts and remains staged and uncommitted for root.

The three accepted limiter paths remain byte-identical to the authorized
attempt-2 source. Their SHA-256 values are in `source-identity.txt`.
`gate-results.tsv` records the command statuses. The Wasm corpus JSONL, host-web
module hashes, and lowering summary are retained here; large raw disassemblies
remain under `/tmp/issue539-cp8-integration`.

Commands run:

```text
CARGO_TARGET_DIR=/tmp/issue539-cp8-integration/target cargo test --locked -p true-peak-limiter
CARGO_TARGET_DIR=/tmp/issue539-cp8-integration/target cargo test --locked -p true-peak-limiter --test mono_collapse
CARGO_TARGET_DIR=/tmp/issue539-cp8-integration/target cargo test --locked --release -p true-peak-limiter --test mono_collapse
CARGO_TARGET_DIR=/tmp/issue539-cp8-integration/target cargo test --locked --release -p true-peak-limiter --lib stationary_dispatch_matches_runtime_oracle_and_observes_selected_body
CARGO_TARGET_DIR=/tmp/issue539-cp8-integration/target cargo test --locked --release -p true-peak-limiter --test allocation
CARGO_TARGET_DIR=/tmp/issue539-cp8-integration/target cargo test --locked -p true-peak-limiter --test determinism
CARGO_TARGET_DIR=/tmp/issue539-cp8-integration/target cargo clippy --locked -p true-peak-limiter --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
bash scripts/check-realtime-policy.sh
bash scripts/check-lane-policy.sh
bash scripts/check-workspace-policy.sh
bash scripts/check-env-vocabulary.sh
bash scripts/run-wasm-gates.sh /tmp/issue539-cp8-wasm-gates
env CARGO_TARGET_DIR=/tmp/issue539-cp8-web-scalar RUSTFLAGS='-C target-feature=-simd128' cargo build --locked --release --target wasm32-unknown-unknown -p host-web
env CARGO_TARGET_DIR=/tmp/issue539-cp8-web-simd128 RUSTFLAGS='-C target-feature=+simd128' cargo build --locked --release --target wasm32-unknown-unknown -p host-web
wasm-objdump -x/-d on each host-web module, with scalar SIMD-opcode absence and SIMD128 opcode-presence checks
bash scripts/check-wasm-realtime-atomics.sh
bash scripts/check-capi-abi.sh
```

Every listed gate returned zero. The full limiter suite passed; the Wasm gate
reported 139 cases and 349 comparisons on native, scalar, and SIMD128 legs with
zero mismatches. No benchmark, timing/capture, AudioWorklet builder, browser,
pin, consumer, commit, or push work was performed.
