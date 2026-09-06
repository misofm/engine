cd /home/bl/misofm/engine-443-plan
PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/engine-443-sol-unified-release cargo test --locked --release --workspace --all-features live_scalar_owner_bytes_are_published_and_capped_before_binding
