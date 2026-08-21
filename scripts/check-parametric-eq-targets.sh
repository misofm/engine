#!/usr/bin/env bash
# Prove Issue-042's frozen endpoint-conditioned delta kernels and target surface.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

fail() {
    printf 'parametric-EQ target failure: %s\n' "$1" >&2
    exit 1
}

command -v objdump >/dev/null || fail 'objdump unavailable'
command -v wasm-objdump >/dev/null || fail 'wasm-objdump unavailable'

scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
packages=(-p miso-engine-core -p miso-engine-parametric-eq)

RUSTFLAGS='-C target-feature=-avx2,-fma' cargo check --quiet --locked --release "${packages[@]}"
for target in aarch64-linux-android aarch64-apple-ios; do
    CARGO_TARGET_DIR="$scratch/check-$target" \
        cargo check --quiet --locked --release --target "$target" "${packages[@]}"
done
for feature in scalar simd128; do
    if [[ "$feature" == scalar ]]; then
        flags='-C target-feature=-simd128'
    else
        flags='-C target-feature=+simd128'
    fi
    CARGO_TARGET_DIR="$scratch/check-wasm-$feature" RUSTFLAGS="$flags" \
        cargo check --quiet --locked --release --target wasm32-unknown-unknown "${packages[@]}"
done

native_object() {
    local name=$1
    local features=$2
    local target="$scratch/$name"
    RUSTFLAGS="-C codegen-units=1 -C target-feature=$features" \
        CARGO_TARGET_DIR="$target" \
        cargo rustc --quiet --locked -p miso-engine-core --release --lib -- --emit=obj
    find "$target/release/deps" -maxdepth 1 -name 'miso_engine_core-*.o' -print -quit
}

native_symbol() {
    local object=$1
    local symbol=$2
    objdump -Cd "$object" | awk -v symbol="$symbol" '
        $0 ~ "<miso_engine_core::arch::" && $0 ~ symbol ">:" { emit = 1 }
        emit && /^Disassembly of section/ { emit = 0 }
        emit { print }
    '
}

baseline_object=$(native_object native-baseline '-avx2,-fma')
scalar=$(native_symbol "$baseline_object" 'scalar::process_delta_scalar')
[[ -n "$scalar" ]] || fail 'missing named scalar delta symbol'
if printf '%s\n' "$scalar" | rg -q '%[yz]mm|\bv[a-z0-9]*ps\b|\bv?fm(add|sub)|\bvfnmadd'; then
    fail 'baseline delta scalar symbol contains packed AVX or FMA'
fi

avx2_object=$(native_object native-avx2 '+avx2,-fma')
avx2=$(native_symbol "$avx2_object" 'x86::process_delta_x86_avx2_inner')
[[ -n "$avx2" ]] || fail 'missing named AVX2 delta symbol'
for instruction in vmulps vaddps vsubps vdivps; do
    printf '%s\n' "$avx2" | rg -q "\\b$instruction\\b" ||
        fail "AVX2 delta kernel missing $instruction"
done
printf '%s\n' "$avx2" | rg -q '%ymm' || fail 'AVX2 delta kernel is not eight-lane'
if printf '%s\n' "$avx2" | rg -q '\bv?fm(add|sub)|\bvfnmadd'; then
    fail 'non-FMA AVX2 delta kernel contains a fused instruction'
fi

fma_object=$(native_object native-avx2-fma '+avx2,+fma')
fma=$(native_symbol "$fma_object" 'x86::process_delta_x86_avx2_fma_inner')
[[ -n "$fma" ]] || fail 'missing named AVX2+FMA delta symbol'
for instruction in vmulps vaddps vsubps vdivps; do
    printf '%s\n' "$fma" | rg -q "\\b$instruction\\b" ||
        fail "AVX2+FMA delta kernel missing $instruction"
done
if printf '%s\n' "$fma" | rg -q '\bv?fm(add|sub)|\bvfnmadd'; then
    fail 'AVX2+FMA delta kernel contains a contraction; V1 permits zero'
fi

arm_target="$scratch/aarch64-neon"
RUSTFLAGS='-C codegen-units=1 -C target-feature=+neon' CARGO_TARGET_DIR="$arm_target" \
    cargo rustc --quiet --locked -p miso-engine-core --target aarch64-linux-android \
    --release --lib -- --emit=asm
arm_assembly=$(find "$arm_target/aarch64-linux-android/release/deps" -maxdepth 1 \
    -name 'miso_engine_core-*.s' -print -quit)
neon=$(awk '
    /process_delta_aarch64_neon_inner:$/ { emit = 1 }
    emit && /^\.Lfunc_end/ { exit }
    emit { print }
' "$arm_assembly")
[[ -n "$neon" ]] || fail 'missing named AArch64 delta symbol'
for instruction in fmul fadd fsub fdiv; do
    printf '%s\n' "$neon" | rg -q "^[[:space:]]*$instruction[[:space:]]+v[0-9]+\\.4s" ||
        fail "NEON delta kernel missing four-lane $instruction"
done
if printf '%s\n' "$neon" | rg -q '^[[:space:]]*fml[as]'; then
    fail 'NEON delta kernel contains a fused instruction'
fi

wasm_object() {
    local name=$1
    local features=$2
    local target="$scratch/$name"
    RUSTFLAGS="-C codegen-units=1 -C target-feature=$features" CARGO_TARGET_DIR="$target" \
        cargo rustc --quiet --locked -p miso-engine-core --target wasm32-unknown-unknown \
        --release --lib -- --emit=obj
    find "$target/wasm32-unknown-unknown/release/deps" -maxdepth 1 \
        -name 'miso_engine_core-*.o' -print -quit
}

wasm_scalar=$(wasm_object wasm-scalar '-simd128')
wasm-objdump -d "$wasm_scalar" >"$scratch/wasm-scalar.txt"
if rg -q 'f32x4\.|v128\.|relaxed' "$scratch/wasm-scalar.txt"; then
    fail 'scalar Wasm delta object contains SIMD or relaxed-SIMD opcodes'
fi

wasm_simd=$(wasm_object wasm-simd '+simd128')
wasm-objdump -d "$wasm_simd" >"$scratch/wasm-simd.txt"
rg -q 'process_delta_wasm_simd128_inner' "$scratch/wasm-simd.txt" ||
    fail 'missing named Wasm simd128 delta symbol'
for instruction in mul add sub div; do
    rg -q "f32x4\\.$instruction" "$scratch/wasm-simd.txt" ||
        fail "Wasm SIMD delta kernel missing f32x4.$instruction"
done
if rg -q 'relaxed' "$scratch/wasm-simd.txt"; then
    fail 'Wasm SIMD delta object contains relaxed-SIMD opcode'
fi

printf 'parametric-EQ targets: PASS (scalar; AVX2; AVX2+FMA=0; NEON; wasm scalar/simd128)\n'
