# Issue 880 MA-3 evidence

MA-3 landed in checkpoint `6f662fee` (`math: simplify exp2 lane reduction without changing
bits`). The release math suite passed with `cargo test --locked --release -p math --features lane`.
The full M1 exhaustive gate passed with
`taskset -c 0-3 cargo test --locked --release -p math --features lane --test m1_exhaustive -- --ignored --test-threads=1`
(both exp2 and log2 sweeps, 68.57 s). The MA-3 identity sweep passed with
`taskset -c 0-3 cargo test --locked --release -p math --features lane --test e1_identity -- --ignored --nocapture --test-threads=1`:
all 4,294,967,296 input patterns, Scalar/Simd4/Simd8, zero mismatches (9.53 s). `git diff --check`
passed before checkpointing.

For wasm codegen, a temporary example instantiated the pre-E1 compare/select body and the current
`math::exp2_lane::<Simd4>` in separately retained functions. It was built with
`CARGO_TARGET_DIR=/tmp/ma3-wasm-target RUSTFLAGS='-C target-feature=+simd128' cargo build --locked --release --target wasm32-unknown-unknown -p math --features lane --example ma3_codegen`,
then inspected with `wasm2wat --generate-names`. The old function contained two
`v128.bitselect` and one `f32x4.gt`; the E1 function contained none. The temporary probe and target
were not committed. The regular math suite also passed with the M1 subsample, M2 identity, and M3
digest controls enabled.
