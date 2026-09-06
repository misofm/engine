PATH=/home/bl/.cargo/bin:$PATH cargo test --release --locked -p builtins-compiler --features test-support --lib tests::actual_scalar_graph_queues_fuse_and_fall_back_against_separate_owners -- --exact
