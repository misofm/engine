#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
cd "$workspace_dir"
packages=(-p builtins -p builtins-compiler)

# Master plan #83 D4 (verifier decision W2-D1): these crates reach `lane`, which
# refuses to compile on x86 without AVX2+FMA -- that guard is the point, not a regression, and it
# is never weakened to keep a script green. The native leg therefore builds the pinned
# `x86-64-v3` target instead of the retired scalar one. Scalar semantics stay proven three ways:
# the `WIDTH = 1` `Lane` instantiation runs in every test on this build, `scripts/run-wasm-gates.sh`
# executes a genuinely SIMD-less `wasm32` target, and the scalar oracle is the identity baseline of
# every lane gate.
RUSTFLAGS='-C target-feature=+avx2,+fma' cargo check --locked --release "${packages[@]}"
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
