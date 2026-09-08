env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-web-scalar RUSTFLAGS='-C target-feature=-simd128' cargo build --locked --release --target wasm32-unknown-unknown -p host-web
