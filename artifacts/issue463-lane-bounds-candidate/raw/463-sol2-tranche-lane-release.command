cwd=/home/bl/misofm/engine-lane2-plan
PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/463-sol2-tranche-target cargo test --locked --release -p lane --test g2_kernel_identity lane2_kernels_preserve_original_words_and_reject_short_inputs_before_writing -- --exact
