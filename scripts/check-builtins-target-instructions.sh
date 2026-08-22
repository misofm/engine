#!/usr/bin/env bash
# Candidate-bound Issue-068 target and named-TPT-instruction qualification.
set -euo pipefail

root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

fail() {
    printf 'issue068 target qualification failure: %s\n' "$1" >&2
    exit 1
}

for command in cargo rustc objdump wasm-objdump sha256sum; do
    command -v "$command" >/dev/null || fail "$command unavailable"
done

closure=(
    -p miso-engine-core
    -p miso-engine-builtins
    -p miso-engine-builtins-compiler
    -p miso-engine-graph
    -p miso-engine-graph-compiler
)
scratch=$(mktemp -d "${TMPDIR:-/tmp}/miso-engine-issue068.XXXXXX")
trap 'rm -rf -- "$scratch"' EXIT

hash_file() { sha256sum "$1" | awk '{print $1}'; }

require_hash() {
    local actual
    actual=$(hash_file "$1")
    [[ "$actual" == "$2" ]] || fail "hash mismatch for $1: $actual"
}

source_manifest() {
    {
        printf '%s\n' Cargo.toml Cargo.lock \
            scripts/check-builtins-targets.sh \
            scripts/check-rack-instructions.sh \
            scripts/check-builtins-target-instructions.sh
        [[ ! -d .cargo ]] || find .cargo -type f -print
        find crates/miso-engine-core crates/miso-engine-builtins \
            crates/miso-engine-builtins-compiler crates/miso-engine-graph \
            crates/miso-engine-graph-compiler -type f \
            \( -name Cargo.toml -o -name '*.rs' \) -print
    } | LC_ALL=C sort -u | while IFS= read -r path; do
        [[ -f "$path" ]] || fail "source-manifest entry is not a regular file: $path"
        printf '%s\t%s\t%s\n' "$path" "$(wc -c <"$path")" "$(hash_file "$path")"
    done
}

seal_candidate() {
    git diff --quiet || fail 'candidate has unstaged changes'
    git diff --cached --quiet || fail 'candidate has staged changes'
    git rev-parse --verify HEAD
}

candidate_before=$(seal_candidate)
source_manifest >"$scratch/source-before.tsv"
source_before=$(hash_file "$scratch/source-before.tsv")
lock_before=$(hash_file Cargo.lock)
require_hash fixtures/builtins/v1/MANIFEST.tsv bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff
require_hash fixtures/builtins/v1/pcm/graph-taps.f32le 508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19
require_hash fixtures/builtins/v1/meters/graph-taps.jsonl 958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f
printf 'issue068 candidate=%s source_manifest=%s cargo_lock=%s scratch=%s\n' \
    "$candidate_before" "$source_before" "$lock_before" "$(basename "$scratch")"
printf 'issue068 tool rustc=%s\n' "$(rustc --version)"
printf 'issue068 tool cargo=%s\n' "$(cargo --version)"
printf 'issue068 tool objdump=%s\n' "$(objdump --version | head -n1)"
printf 'issue068 tool wasm-objdump=%s\n' "$(wasm-objdump --version | head -n1)"

CARGO_TARGET_DIR="$scratch/runtime-selection-tests" cargo test --locked -p miso-engine-rack \
    tests::issue068_all_capability_tuples_select_exact_backend_and_width -- --exact --test-threads=1
CARGO_TARGET_DIR="$scratch/preparation-tests" cargo test --locked -p miso-engine-core \
    arch::tests::preparation_is_the_only_safe_architecture_gate -- --exact --test-threads=1
CARGO_TARGET_DIR="$scratch/four-rate-tests" cargo test --locked -p miso-engine-builtins \
    tests::issue068_launch_rates_match_scalar_for_nonfused_and_fma_tpt_banks \
    -- --exact --test-threads=1 --nocapture

build_closure() {
    local name target features
    name=$1
    target=$2
    features=$3
    local -a target_args=()
    [[ -z "$target" ]] || target_args=(--target "$target")
    RUSTFLAGS="-C target-feature=$features" CARGO_TARGET_DIR="$scratch/$name" \
        cargo check --locked --release "${closure[@]}" "${target_args[@]}"
    printf 'issue068 build leg=%s target=%s features=%s result=PASS compile-or-object-only\n' \
        "$name" "${target:-native}" "$features"
}

exact_object() {
    local directory target deps
    local -a objects
    directory=$1
    target=$2
    deps="$directory/release/deps"
    [[ -z "$target" ]] || deps="$directory/$target/release/deps"
    mapfile -t objects < <(find "$deps" -maxdepth 1 -type f -name 'miso_engine_core-*.o' -print | LC_ALL=C sort)
    [[ ${#objects[@]} == 1 ]] || fail "expected exactly one current core object in $deps, found ${#objects[@]}"
    printf '%s\n' "${objects[0]}"
}

native_object() {
    local name features target_dir
    name=$1
    features=$2
    target_dir="$scratch/$name"
    RUSTFLAGS="-C codegen-units=1 -C target-feature=$features" CARGO_TARGET_DIR="$target_dir" \
        cargo rustc --quiet --locked -p miso-engine-core --release --lib -- --emit=obj
    exact_object "$target_dir" ''
}

extract_native_symbol() {
    local object suffix output count
    object=$1
    suffix=$2
    output=$3
    objdump -Cd "$object" >"$output.all"
    count=$(rg -c "<miso_engine_core::arch::.*${suffix}.*>:" "$output.all" || true)
    [[ "$count" == 1 ]] || fail "expected exactly one $suffix symbol, found $count"
    awk -v suffix="$suffix" '
        $0 ~ "<miso_engine_core::arch::" && $0 ~ suffix ">:" { emit = 1 }
        emit && /^Disassembly of section/ { exit }
        emit { print }
    ' "$output.all" >"$output"
    [[ -s "$output" ]] || fail "empty $suffix symbol body"
}

scalar_object=$(native_object native-scalar-object '-avx2,-fma')
extract_native_symbol "$scalar_object" 'scalar::process_tpt_scalar' "$scratch/scalar.symbol"
if rg -q '%[yz]mm|\bv[a-z0-9]*ps\b|\bv?fm(add|sub)|\bvfnmadd' "$scratch/scalar.symbol"; then
    fail 'scalar TPT symbol contains packed AVX or FMA'
fi
printf 'issue068 object leg=native-scalar object_sha256=%s symbol_sha256=%s result=PASS\n' \
    "$(hash_file "$scalar_object")" "$(hash_file "$scratch/scalar.symbol")"

avx2_object=$(native_object native-avx2-object '+avx2,-fma')
extract_native_symbol "$avx2_object" 'x86::process_tpt_x86_avx2_inner' "$scratch/avx2.symbol"
for instruction in vmulps vaddps vsubps; do
    rg -q "\\b$instruction\\b" "$scratch/avx2.symbol" || fail "AVX2 TPT symbol lacks $instruction"
done
rg -q '%ymm' "$scratch/avx2.symbol" || fail 'AVX2 TPT symbol is not eight-lane'
if rg -q '\bv?fm(add|sub)|\bvfnmadd' "$scratch/avx2.symbol"; then
    fail 'AVX2 non-FMA TPT symbol contains a fused operation'
fi
printf 'issue068 object leg=native-avx2 object_sha256=%s symbol_sha256=%s result=PASS\n' \
    "$(hash_file "$avx2_object")" "$(hash_file "$scratch/avx2.symbol")"

fma_object=$(native_object native-avx2-fma-object '+avx2,+fma')
extract_native_symbol "$fma_object" 'x86::process_tpt_x86_avx2_fma_inner' "$scratch/fma.symbol"
fma_sites=$(rg -c '\b(vfmsub|vfmadd|vfnmadd)[0-9]*ps\b' "$scratch/fma.symbol" || true)
[[ "$fma_sites" == 3 ]] || fail "AVX2+FMA TPT symbol has $fma_sites fused sites, expected 3"
for instruction in vfmsub vfmadd vfnmadd; do
    rg -q "\\b${instruction}[0-9]*ps\\b" "$scratch/fma.symbol" || fail "AVX2+FMA TPT symbol lacks $instruction"
done
printf 'issue068 object leg=native-avx2-fma object_sha256=%s symbol_sha256=%s result=PASS\n' \
    "$(hash_file "$fma_object")" "$(hash_file "$scratch/fma.symbol")"

build_closure native-scalar '' '-avx2,-fma'
build_closure aarch64-android aarch64-linux-android '+neon'
build_closure aarch64-ios aarch64-apple-ios '+neon'
build_closure wasm-scalar wasm32-unknown-unknown '-simd128'
build_closure wasm-simd128 wasm32-unknown-unknown '+simd128'

neon_target="$scratch/aarch64-neon-object"
RUSTFLAGS='-C codegen-units=1 -C target-feature=+neon' CARGO_TARGET_DIR="$neon_target" \
    cargo rustc --quiet --locked -p miso-engine-core --target aarch64-linux-android --release --lib -- --emit=asm
mapfile -t neon_assembly < <(find "$neon_target/aarch64-linux-android/release/deps" -maxdepth 1 -type f -name 'miso_engine_core-*.s' -print | LC_ALL=C sort)
[[ ${#neon_assembly[@]} == 1 ]] || fail "expected exactly one AArch64 assembly file, found ${#neon_assembly[@]}"
awk '/process_tpt_aarch64_neon_inner:$/ { emit = 1 } emit && /^\.Lfunc_end/ { exit } emit { print }' \
    "${neon_assembly[0]}" >"$scratch/neon.symbol"
[[ -s "$scratch/neon.symbol" ]] || fail 'empty AArch64 NEON TPT symbol body'
for instruction in fmul fadd fsub; do
    rg -q "^[[:space:]]*$instruction[[:space:]]+v[0-9]+\\.4s" "$scratch/neon.symbol" || fail "AArch64 NEON TPT symbol lacks four-lane $instruction"
done
if rg -q '^[[:space:]]*fml[as]' "$scratch/neon.symbol"; then
    fail 'AArch64 NEON TPT symbol contains a fused operation'
fi
printf 'issue068 object leg=aarch64-neon object_sha256=%s symbol_sha256=%s result=PASS compile-or-object-only\n' \
    "$(hash_file "${neon_assembly[0]}")" "$(hash_file "$scratch/neon.symbol")"

wasm_object() {
    local name features target_dir
    name=$1
    features=$2
    target_dir="$scratch/$name"
    RUSTFLAGS="-C codegen-units=1 -C target-feature=$features" CARGO_TARGET_DIR="$target_dir" \
        cargo rustc --quiet --locked -p miso-engine-core --target wasm32-unknown-unknown --release --lib -- --emit=obj
    exact_object "$target_dir" wasm32-unknown-unknown
}

wasm_scalar_object=$(wasm_object wasm-scalar-object '-simd128')
wasm-objdump -d "$wasm_scalar_object" >"$scratch/wasm-scalar.symbol"
if rg -q 'f32x4\.|v128\.|relaxed' "$scratch/wasm-scalar.symbol"; then
    fail 'scalar Wasm object contains SIMD or relaxed-SIMD opcodes'
fi
printf 'issue068 object leg=wasm-scalar object_sha256=%s symbol_sha256=%s result=PASS compile-or-object-only\n' \
    "$(hash_file "$wasm_scalar_object")" "$(hash_file "$scratch/wasm-scalar.symbol")"

wasm_simd_object=$(wasm_object wasm-simd128-object '+simd128')
wasm-objdump -d "$wasm_simd_object" >"$scratch/wasm-simd128.all"
wasm_simd_count=$(rg -c 'process_tpt_wasm_simd128_inner' "$scratch/wasm-simd128.all" || true)
[[ "$wasm_simd_count" == 1 ]] || fail "expected exactly one Wasm SIMD TPT symbol, found $wasm_simd_count"
awk '/process_tpt_wasm_simd128_inner/ { emit = 1 } emit && /^ *func\[/ && !/process_tpt_wasm_simd128_inner/ { exit } emit { print }' \
    "$scratch/wasm-simd128.all" >"$scratch/wasm-simd128.symbol"
[[ -s "$scratch/wasm-simd128.symbol" ]] || fail 'empty Wasm SIMD TPT symbol body'
for instruction in mul add sub; do
    rg -q "f32x4\\.$instruction" "$scratch/wasm-simd128.symbol" || fail "Wasm SIMD TPT symbol lacks f32x4.$instruction"
done
if rg -q 'relaxed' "$scratch/wasm-simd128.symbol"; then
    fail 'Wasm SIMD TPT symbol contains relaxed-SIMD opcode'
fi
printf 'issue068 object leg=wasm-simd128 object_sha256=%s symbol_sha256=%s result=PASS compile-or-object-only\n' \
    "$(hash_file "$wasm_simd_object")" "$(hash_file "$scratch/wasm-simd128.symbol")"

candidate_after=$(seal_candidate)
source_manifest >"$scratch/source-after.tsv"
source_after=$(hash_file "$scratch/source-after.tsv")
lock_after=$(hash_file Cargo.lock)
[[ "$candidate_after" == "$candidate_before" ]] || fail 'candidate commit changed during qualification'
[[ "$source_after" == "$source_before" ]] || fail 'source manifest changed during qualification'
[[ "$lock_after" == "$lock_before" ]] || fail 'Cargo.lock changed during qualification'
require_hash fixtures/builtins/v1/MANIFEST.tsv bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff
require_hash fixtures/builtins/v1/pcm/graph-taps.f32le 508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19
require_hash fixtures/builtins/v1/meters/graph-taps.jsonl 958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f
printf 'issue068 target/instruction qualification: PASS (cross-target compile-or-object-only)\n'
