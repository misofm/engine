#!/usr/bin/env bash
# Prove Issue-008's named architecture kernels emit only their frozen instruction families.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

# Issue-083 D12 added `[profile.release] lto = "fat"` to the workspace. Under fat LTO, rustc emits
# LLVM bitcode into the `.o` files this script disassembles, and `objdump` reports "file format not
# recognized". Turn link-time optimisation off for these single-crate probes only.
#
# This does not weaken the check. The probe compiles `-p miso-engine-core --lib` on its own and
# disassembles one function to see which instructions the target selected; it never links, and LTO
# is a link-time transform across crates that plays no part in that selection. `lto = false` is the
# setting these gates were written against, before the workspace had a release profile at all.
export CARGO_PROFILE_RELEASE_LTO=false

fail() {
    printf 'rack instruction failure: %s\n' "$1" >&2
    exit 1
}

command -v objdump >/dev/null || fail 'objdump unavailable'
command -v wasm-objdump >/dev/null || fail 'wasm-objdump unavailable'

scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT

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
scalar_disassembly=$(native_symbol "$baseline_object" 'scalar::process_tpt_scalar')
[[ -n "$scalar_disassembly" ]] || fail 'missing named scalar kernel symbol'
if printf '%s\n' "$scalar_disassembly" | rg -q '%[yz]mm|\bv[a-z0-9]*ps\b|\bv?fm(add|sub)|\bvfnmadd'; then
    fail 'baseline scalar symbol contains packed AVX or FMA'
fi

avx2_object=$(native_object native-avx2 '+avx2,-fma')
avx2_disassembly=$(native_symbol "$avx2_object" 'x86::process_tpt_x86_avx2_inner')
[[ -n "$avx2_disassembly" ]] || fail 'missing named AVX2 kernel symbol'
for instruction in vmulps vaddps vsubps; do
    printf '%s\n' "$avx2_disassembly" | rg -q "\\b$instruction\\b" ||
        fail "AVX2 kernel missing $instruction"
done
printf '%s\n' "$avx2_disassembly" | rg -q '%ymm' || fail 'AVX2 kernel is not eight-lane'
if printf '%s\n' "$avx2_disassembly" | rg -q '\bv?fm(add|sub)|\bvfnmadd'; then
    fail 'non-FMA AVX2 kernel contains a fused instruction'
fi

fma_object=$(native_object native-avx2-fma '+avx2,+fma')
fma_disassembly=$(native_symbol "$fma_object" 'x86::process_tpt_x86_avx2_fma_inner')
[[ -n "$fma_disassembly" ]] || fail 'missing named AVX2+FMA kernel symbol'
fma_sites=$(printf '%s\n' "$fma_disassembly" | rg -c '\b(vfmsub|vfmadd|vfnmadd)[0-9]*ps\b' || true)
[[ "$fma_sites" == 3 ]] || fail "AVX2+FMA kernel has $fma_sites contraction sites, expected 3"
for instruction in vfmsub vfmadd vfnmadd; do
    printf '%s\n' "$fma_disassembly" | rg -q "\\b${instruction}[0-9]*ps\\b" ||
        fail "AVX2+FMA kernel missing $instruction contraction"
done

arm_target="$scratch/aarch64-neon"
RUSTFLAGS='-C codegen-units=1 -C target-feature=+neon' \
    CARGO_TARGET_DIR="$arm_target" \
    cargo rustc --quiet --locked -p miso-engine-core --target aarch64-linux-android \
    --release --lib -- --emit=asm
arm_assembly_file=$(find "$arm_target/aarch64-linux-android/release/deps" -maxdepth 1 \
    -name 'miso_engine_core-*.s' -print -quit)
arm_disassembly=$(awk '
    /process_tpt_aarch64_neon_inner:$/ { emit = 1 }
    emit && /^\.Lfunc_end/ { exit }
    emit { print }
' "$arm_assembly_file")
[[ -n "$arm_disassembly" ]] || fail 'missing named AArch64 NEON kernel symbol'
for instruction in fmul fadd fsub; do
    printf '%s\n' "$arm_disassembly" | rg -q "^[[:space:]]*$instruction[[:space:]]+v[0-9]+\\.4s" ||
        fail "NEON kernel missing four-lane $instruction"
done
if printf '%s\n' "$arm_disassembly" | rg -q '^[[:space:]]*fml[as]'; then
    fail 'base NEON kernel contains a fused instruction'
fi

wasm_object() {
    local name=$1
    local features=$2
    local target="$scratch/$name"
    RUSTFLAGS="-C codegen-units=1 -C target-feature=$features" \
        CARGO_TARGET_DIR="$target" \
        cargo rustc --quiet --locked -p miso-engine-core --target wasm32-unknown-unknown \
        --release --lib -- --emit=obj
    find "$target/wasm32-unknown-unknown/release/deps" -maxdepth 1 \
        -name 'miso_engine_core-*.o' -print -quit
}

wasm_scalar_object=$(wasm_object wasm-scalar '-simd128')
wasm-objdump -d "$wasm_scalar_object" >"$scratch/wasm-scalar.txt"
if rg -q 'f32x4\.|v128\.|relaxed' "$scratch/wasm-scalar.txt"; then
    fail 'scalar Wasm object contains SIMD or relaxed-SIMD opcodes'
fi

wasm_simd_object=$(wasm_object wasm-simd '+simd128')
wasm-objdump -d "$wasm_simd_object" >"$scratch/wasm-simd.txt"
rg -q 'process_tpt_wasm_simd128_inner' "$scratch/wasm-simd.txt" ||
    fail 'missing named Wasm simd128 kernel symbol'
for instruction in mul add sub; do
    rg -q "f32x4\\.$instruction" "$scratch/wasm-simd.txt" ||
        fail "Wasm SIMD kernel missing f32x4.$instruction"
done
if rg -q 'relaxed' "$scratch/wasm-simd.txt"; then
    fail 'Wasm SIMD object contains relaxed-SIMD opcode'
fi

printf 'rack instructions: PASS (scalar; AVX2; AVX2+FMA=3; NEON; wasm scalar/simd128)\n'
