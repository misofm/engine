cd /tmp/engine-443-baseline-lib-check
PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/engine-443-baseline-lib-release-target cargo test --locked --release -p graph-compiler --lib scalar_dispatch_compiles_without_banks_on_any_host
