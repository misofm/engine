cd /home/bl/misofm/engine-443-plan
PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/engine-443-sol-graphcompiler-release cargo test --locked --release -p graph-compiler live_scalar_owner_bytes_are_published_and_capped_before_binding
