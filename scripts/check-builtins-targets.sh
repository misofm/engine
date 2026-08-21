#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
cd "$workspace_dir"
packages=(-p miso-engine-builtins -p miso-engine-builtins-compiler)

RUSTFLAGS='-C target-feature=-avx2,-fma' cargo check --locked --release "${packages[@]}"
for target in aarch64-linux-android aarch64-apple-ios; do
  CARGO_TARGET_DIR="target/issue7/targets/$target" \
    cargo check --locked --release --target "$target" "${packages[@]}"
done
for feature in scalar simd128; do
  if [[ "$feature" == scalar ]]; then
    flags='-C target-feature=-simd128'
  else
    flags='-C target-feature=+simd128'
  fi
  CARGO_TARGET_DIR="target/issue7/targets/wasm-$feature" RUSTFLAGS="$flags" \
    cargo build --locked --release --target wasm32-unknown-unknown "${packages[@]}"
done
printf 'issue-007 scalar builtins target matrix: PASS\n'
