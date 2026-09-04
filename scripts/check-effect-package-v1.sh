#!/usr/bin/env bash
# Check the independent canonical package corpus and native/scalar-Wasm package surface.
#
# WP-3 (#359): the cold scratch-dir `cargo test -p effect-package` this script used to run is gone
# -- it is an exact subset of the workspace debug test run. The five precondition re-runs
# (check/test-workspace-policy, check/test-realtime-policy, check-effect-runtime-policy) are gone
# too: each is its own separate CI step, and re-running them here bought nothing beyond ordering. The
# wasm `--emit=obj` build now uses a shared `target/ci/effect-v1` dir (honouring CARGO_TARGET_DIR as
# a base) instead of a private mktemp dir, so scripts/check-effect-descriptor-v1.sh's identical
# build (same RUSTFLAGS, same crate, same features) is incremental when it runs after this one.
#
# The `cargo check --manifest-path fuzz/Cargo.toml --bins` step stays: fuzz.yml runs the identical
# command whenever effect-package's dependency closure changes, but fuzz.yml is not a required
# status check (design doc #359 6.8/6.1), so this is the only required-check assertion that the
# fuzz binaries still build against the current effect-package surface.
set -euo pipefail

workspace_root="$(cd "$(dirname "$0")/.." && pwd)"
scratch_directory="$(mktemp -d)"
trap 'rm -rf -- "$scratch_directory"' EXIT

for command in cargo python3 rg wasm-objdump; do
    command -v "$command" >/dev/null 2>&1 || {
        printf 'effect package V1 check failure: missing %s\n' "$command" >&2
        exit 1
    }
done

PYTHONDONTWRITEBYTECODE=1 python3 -I -B \
    "$workspace_root/scripts/effect-package-v1-reference.py"

CARGO_TARGET_DIR="$scratch_directory/fuzz-target" \
    cargo check --quiet --locked --manifest-path "$workspace_root/fuzz/Cargo.toml" --bins

wasm_target="${CARGO_TARGET_DIR:-$workspace_root/target}/ci/effect-v1"
RUSTFLAGS='-C target-feature=-simd128' CARGO_TARGET_DIR="$wasm_target" \
    cargo rustc --quiet --locked --manifest-path "$workspace_root/Cargo.toml" \
        -p effect-package --features c-abi --lib --release --target wasm32-unknown-unknown -- \
        --emit=obj
wasm_module="$wasm_target/wasm32-unknown-unknown/release/effect_package.wasm"
[[ -f "$wasm_module" ]] || {
    printf 'effect package V1 check failure: missing Wasm module\n' >&2
    exit 1
}
wasm_object="$(find "$wasm_target/wasm32-unknown-unknown/release/deps" -maxdepth 1 \
    -type f -name 'effect_package.o' -print)"
[[ -n "$wasm_object" && "$wasm_object" != *$'\n'* ]] || {
    printf 'effect package V1 check failure: expected one exact Wasm object\n' >&2
    exit 1
}

metadata="$scratch_directory/wasm-metadata.txt"
disassembly="$scratch_directory/wasm-disassembly.txt"
object_metadata="$scratch_directory/wasm-object-metadata.txt"
object_disassembly="$scratch_directory/wasm-object-disassembly.txt"
wasm-objdump -x "$wasm_module" >"$metadata"
wasm-objdump -d "$wasm_module" >"$disassembly"
wasm-objdump -x "$wasm_object" >"$object_metadata"
wasm-objdump -d "$wasm_object" >"$object_disassembly"
export_count="$(rg -c -- '-> "miso_engine_effect_descriptor_v1_inspect"' "$metadata" || true)"
[[ "$export_count" == 1 ]] || {
    printf 'effect package V1 check failure: descriptor inspect export count %s\n' \
        "$export_count" >&2
    exit 1
}
# Issue #143 added exactly one additive export: the observation projection.
observation_export_count="$(rg -c -- \
    '-> "miso_engine_effect_descriptor_v1_inspect_observations"' "$metadata" || true)"
[[ "$observation_export_count" == 1 ]] || {
    printf 'effect package V1 check failure: observation inspect export count %s\n' \
        "$observation_export_count" >&2
    exit 1
}
if rg -- '-> "miso_engine_' "$metadata" | \
    rg -v -- '-> "miso_engine_effect_descriptor_v1_inspect"' | \
    rg -v -- '-> "miso_engine_effect_descriptor_v1_inspect_observations"'; then
    printf 'effect package V1 check failure: unexpected miso_engine Wasm export\n' >&2
    exit 1
fi
if rg -n '\b(v128|f32x4|i8x16|i16x8|i32x4|i64x2|f64x2)\b' \
    "$disassembly" "$object_disassembly"; then
    printf 'effect package V1 check failure: SIMD opcode in scalar output\n' >&2
    exit 1
fi
for symbol in effect_package_required_size encode_effect_package \
    verify_effect_package select_effect_package_artifact effect_package_cid; do
    rg -q "$symbol" "$object_metadata" || {
        printf 'effect package V1 check failure: %s absent from Wasm object\n' "$symbol" >&2
        exit 1
    }
done

native_source="$scratch_directory/package-native.rs"
cid_source="$scratch_directory/package-cid.rs"
awk '/^#\[cfg\(test\)\]/{exit} {print}' \
    "$workspace_root/crates/effect-package/src/package.rs" >"$native_source"
awk '/^#\[cfg\(test\)\]/{exit} {print}' \
    "$workspace_root/crates/effect-package/src/cid.rs" >"$cid_source"
if rg -n 'Vec[<:]|String[<:]|vec!\[|\.to_vec\(|\.collect\(|\bunsafe\b|\.sort\(|\.sort_by\(|\.sort_by_key\(|\.sort_by_cached_key\(' \
    "$native_source" "$cid_source"; then
    printf 'effect package V1 check failure: allocation/unsafe package-native surface\n' >&2
    exit 1
fi
if rg -n 'effect_package|effect-package' \
    "$workspace_root/crates/engine/src/realtime"; then
    printf 'effect package V1 check failure: package is render reachable\n' >&2
    exit 1
fi

if find "$workspace_root" \( -type d -name __pycache__ -o -type f -name '*.pyc' \) | rg .; then
    printf 'effect package V1 check failure: generated Python artifact\n' >&2
    exit 1
fi

printf 'effect package V1 corpus/native/scalar-Wasm check: ok\n'
