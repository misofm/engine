PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/engine-460-sol-target cargo clippy --locked -p protocol --all-targets -- -D warnings
