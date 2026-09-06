cd /tmp/engine-443-baseline-660fce8
PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/engine-443-sol-exact-baseline-release cargo test --locked --release -p graph-compiler scalar_dispatch_compiles_without_banks_on_any_host
