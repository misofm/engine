#!/usr/bin/env bash
# Exact Issue 081 five-row native/compile/object target matrix. Do not use as a smoke test.
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: check-effect-interchange-targets.sh\n' >&2; exit 2; }
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
bash scripts/check-effect-interchange-qualification.sh . >/dev/null
packages=(
    -p miso-engine-effect-package
    -p miso-engine-effect-compiler
    -p miso-engine-conformance
)
for tool in cargo rustc rustup wasm-objdump rg uname; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'effect interchange target matrix: missing tool %s\n' "$tool" >&2
        exit 1
    }
done
[[ "$(uname -s)" == Linux ]] || {
    printf 'effect interchange target matrix: native row requires Linux\n' >&2
    exit 1
}
[[ "$(rustc -vV | sed -n 's/^host: //p')" == x86_64-unknown-linux-gnu ]] || {
    printf 'effect interchange target matrix: native row requires x86_64 Linux host\n' >&2
    exit 1
}
for target in x86_64-unknown-linux-gnu aarch64-linux-android aarch64-apple-ios wasm32-unknown-unknown; do
    rustup target list --installed | rg -qx "$target" || {
        printf 'effect interchange target matrix: required target unavailable: %s\n' "$target" >&2
        exit 1
    }
done

cargo test --locked "${packages[@]}" --lib --tests
bash scripts/test-effect-descriptor-capi.sh

for target in aarch64-linux-android aarch64-apple-ios; do
    cargo check --locked --all-targets --target "$target" "${packages[@]}"
done

scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT
for mode in scalar simd; do
    if [[ "$mode" == scalar ]]; then
        feature=-simd128
    else
        feature=+simd128
    fi
    target_dir="$scratch/$mode"
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="-C target-feature=$feature" \
        cargo check --locked --all-targets --target wasm32-unknown-unknown "${packages[@]}"
    CARGO_TARGET_DIR="$target_dir" RUSTFLAGS="-C target-feature=$feature" \
        cargo rustc --locked -p miso-engine-effect-package --target wasm32-unknown-unknown \
        --lib -- --crate-type=cdylib
    wasm="$(find "$target_dir/wasm32-unknown-unknown/debug" -maxdepth 1 -name '*.wasm' -type f -print -quit)"
    [[ -n "$wasm" ]] || { printf 'effect interchange target matrix: missing Wasm object\n' >&2; exit 1; }
    exports="$(wasm-objdump -x "$wasm" | sed -n 's/.*<\(miso_engine_[^>]*\)>.*/\1/p' | LC_ALL=C sort -u)"
    [[ "$exports" == miso_engine_effect_descriptor_v1_inspect ]] || {
        printf 'effect interchange target matrix: unexpected Wasm export in %s\n%s\n' \
            "$mode" "$exports" >&2
        exit 1
    }
    if [[ "$mode" == scalar ]] && wasm-objdump -d "$wasm" | rg -q \
        'v128|f32x4|f64x2|i8x16|i16x8|i32x4|i64x2'; then
        printf 'effect interchange target matrix: SIMD opcode in scalar object\n' >&2
        exit 1
    fi
done
bash scripts/check-effect-interchange-qualification.sh . >/dev/null
printf 'effect interchange five-target matrix: ok\n'
