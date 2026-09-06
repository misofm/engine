PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/492-sol2-target cargo clippy --locked -p host-core --all-targets --all-features -- -D warnings
