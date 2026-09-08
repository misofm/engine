env PATH=/home/bl/.cargo/bin:$PATH CARGO_TARGET_DIR=/tmp/issue621-attempt1-native-target cargo rustc --locked --release -p true-peak-limiter --lib -- --emit=asm,llvm-ir
