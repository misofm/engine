cwd=/home/bl/misofm/engine-lane2-plan
source_head=47d9ca678566fe786121d31a074cb02371dd04d1
PATH=/home/bl/.cargo/bin:$PATH RUSTFLAGS='-C target-feature=-simd128' CARGO_TARGET_DIR=/tmp/engine-463-wasm-scalar cargo check --locked --target wasm32-unknown-unknown -p target-smoke -p protocol -p host-web -p builtins-compiler
