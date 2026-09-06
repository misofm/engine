PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/engine-479-sol-target cargo clippy --locked -p builtins-compiler --all-targets --features test-support -- -D warnings
