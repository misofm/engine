#!/usr/bin/env bash
# Check the independent V1 descriptor corpus plus native and scalar-Wasm inspection surfaces.
set -euo pipefail

workspace_root="$(cd "$(dirname "$0")/.." && pwd)"
scratch_directory="$(mktemp -d)"
trap 'rm -rf -- "$scratch_directory"' EXIT

for command in cargo python3 wasm-objdump; do
    command -v "$command" >/dev/null 2>&1 || {
        printf 'effect descriptor V1 check failure: missing %s\n' "$command" >&2
        exit 1
    }
done

PYTHONDONTWRITEBYTECODE=1 python3 -I -B \
    "$workspace_root/scripts/effect-descriptor-v1-reference.py"

CARGO_TARGET_DIR="$scratch_directory/native-target" \
    bash "$workspace_root/scripts/test-effect-descriptor-capi.sh"

wasm_target="$scratch_directory/wasm-target"
RUSTFLAGS='-C target-feature=-simd128' CARGO_TARGET_DIR="$wasm_target" \
    cargo rustc --quiet --locked --manifest-path "$workspace_root/Cargo.toml" \
        -p miso-engine-effect-package --features c-abi --lib --release --target wasm32-unknown-unknown -- \
        --emit=obj
wasm_module="$wasm_target/wasm32-unknown-unknown/release/miso_engine_effect_package.wasm"
[[ -f "$wasm_module" ]] || {
    printf 'effect descriptor V1 check failure: missing Wasm module\n' >&2
    exit 1
}
wasm_object="$(find "$wasm_target/wasm32-unknown-unknown/release/deps" -maxdepth 1 \
    -type f -name 'miso_engine_effect_package.o' -print)"
[[ -n "$wasm_object" && "$wasm_object" != *$'\n'* ]] || {
    printf 'effect descriptor V1 check failure: expected one exact Wasm object\n' >&2
    exit 1
}

metadata="$scratch_directory/wasm-metadata.txt"
disassembly="$scratch_directory/wasm-disassembly.txt"
wasm-objdump -x "$wasm_module" >"$metadata"
wasm-objdump -d "$wasm_module" >"$disassembly"
wasm-objdump -x "$wasm_object" >"$scratch_directory/wasm-object-metadata.txt"
wasm-objdump -d "$wasm_object" >"$scratch_directory/wasm-object-disassembly.txt"
export_count="$(rg -c -- '-> "miso_engine_effect_descriptor_v1_inspect"' "$metadata" || true)"
[[ "$export_count" == 1 ]] || {
    printf 'effect descriptor V1 check failure: inspect export count %s\n' "$export_count" >&2
    exit 1
}
# Issue #143 added exactly one additive export: the observation projection. The frozen inspect
# signature and its record layouts are untouched, so the export set grows by one name and no more.
observation_export_count="$(rg -c -- \
    '-> "miso_engine_effect_descriptor_v1_inspect_observations"' "$metadata" || true)"
[[ "$observation_export_count" == 1 ]] || {
    printf 'effect descriptor V1 check failure: observation export count %s\n' \
        "$observation_export_count" >&2
    exit 1
}
if rg -- '-> "miso_engine_' "$metadata" |
    rg -v -- '-> "miso_engine_effect_descriptor_v1_inspect"' |
    rg -v -- '-> "miso_engine_effect_descriptor_v1_inspect_observations"'; then
    printf 'effect descriptor V1 check failure: unexpected miso_engine Wasm export\n' >&2
    exit 1
fi
if rg -n '\b(v128|f32x4|i8x16|i16x8|i32x4|i64x2|f64x2)\b' "$disassembly"; then
    printf 'effect descriptor V1 check failure: SIMD opcode in scalar module\n' >&2
    exit 1
fi
rg -q 'miso_engine_effect_descriptor_v1_inspect' \
    "$scratch_directory/wasm-object-metadata.txt" || {
    printf 'effect descriptor V1 check failure: inspect symbol absent from Wasm object\n' >&2
    exit 1
}
rg -q 'miso_engine_effect_descriptor_v1_inspect_observations' \
    "$scratch_directory/wasm-object-metadata.txt" || {
    printf 'effect descriptor V1 check failure: observation symbol absent from Wasm object\n' >&2
    exit 1
}
if rg -n '\b(v128|f32x4|i8x16|i16x8|i32x4|i64x2|f64x2)\b' \
    "$scratch_directory/wasm-object-disassembly.txt"; then
    printf 'effect descriptor V1 check failure: SIMD opcode in scalar object\n' >&2
    exit 1
fi

if find "$workspace_root" -type d -name __pycache__ -o -type f -name '*.pyc' | rg .; then
    printf 'effect descriptor V1 check failure: generated Python artifact\n' >&2
    exit 1
fi

printf 'effect descriptor V1 corpus/native/Wasm check: ok\n'
