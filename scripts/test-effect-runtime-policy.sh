#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
cp -R "$root/crates" "$root/tools" "$root/docs" "$root/scripts" "$temp/"
mkdir "$temp/fuzz"
cp "$root/fuzz/Cargo.toml" "$root/fuzz/Cargo.lock" "$temp/fuzz/"
cp -R "$root/fuzz/fuzz_targets" "$temp/fuzz/"
compiler_manifest="$temp/crates/miso-engine-effect-compiler/Cargo.toml"
contract_manifest="$temp/crates/miso-engine-effect-contract/Cargo.toml"

bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null
bash "$temp/scripts/check-effect-state-migration-v1.sh" "$temp" >/dev/null

restore_compiler_manifest() {
    cp "$root/crates/miso-engine-effect-compiler/Cargo.toml" "$compiler_manifest"
}

expect_dependency_failure() {
    local mutation="$1"
    if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
        printf 'effect runtime dependency mutation escaped: %s\n' "$mutation" >&2
        exit 1
    fi
    restore_compiler_manifest
}

sed -i '/^\[dependencies\]$/a miso-engine-graph.workspace = true' "$compiler_manifest"
expect_dependency_failure arbitrary-extra

sed -i '/^miso-engine-parametric-eq[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-parametric-eq
sed -i 's/^miso-engine-parametric-eq[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-parametric-eq
sed -i '/^miso-engine-compressor[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-compressor
sed -i 's/^miso-engine-compressor[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-compressor
sed -i '/^miso-engine-gate-expander[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-gate-expander
sed -i 's/^miso-engine-gate-expander[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-gate-expander
sed -i '/^miso-engine-multiband-compressor[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-multiband-compressor
sed -i 's/^miso-engine-multiband-compressor[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-multiband-compressor
sed -i '/^miso-engine-true-peak-limiter[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-true-peak-limiter
sed -i 's/^miso-engine-true-peak-limiter[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-true-peak-limiter
sed -i '/^miso-engine-soft-clip[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-soft-clip
sed -i 's/^miso-engine-soft-clip[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-soft-clip
sed -i '/^miso-engine-transient-shaper[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-transient-shaper
sed -i 's/^miso-engine-transient-shaper[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-transient-shaper
sed -i '/^miso-engine-delay[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-delay
sed -i 's/^miso-engine-delay[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-delay

for reverse_dependency in miso-engine-core miso-engine-session; do
    reverse_manifest="$temp/crates/$reverse_dependency/Cargo.toml"
    printf '\nmiso-engine-effect-package.workspace = true\n' >>"$reverse_manifest"
    if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
        printf 'effect runtime package reverse-dependency mutation escaped: %s\n' \
            "$reverse_dependency" >&2
        exit 1
    fi
    cp "$root/crates/$reverse_dependency/Cargo.toml" "$reverse_manifest"
done

printf '\nuse miso_engine_effect_package as leaked_state_package;\n' \
    >>"$temp/fuzz/fuzz_targets/session_parse.rs"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'effect runtime package fuzz-target mutation escaped\n' >&2
    exit 1
fi
cp "$root/fuzz/fuzz_targets/session_parse.rs" "$temp/fuzz/fuzz_targets/session_parse.rs"

printf '\nuse miso_engine_effect_package as leaked_state_package;\n' \
    >>"$temp/tools/miso-engine-rack-bench/src/main.rs"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'effect runtime package unrelated-tool mutation escaped\n' >&2
    exit 1
fi
cp "$root/tools/miso-engine-rack-bench/src/main.rs" \
    "$temp/tools/miso-engine-rack-bench/src/main.rs"

printf '\npub fn effect_state_migration_render_leak() {}\n' \
    >>"$temp/crates/miso-engine-core/src/realtime/plan.rs"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'effect runtime migration render mutation escaped\n' >&2
    exit 1
fi
if bash "$temp/scripts/check-effect-state-migration-v1.sh" "$temp" >/dev/null 2>&1; then
    printf 'effect state migration render mutation escaped narrow checker\n' >&2
    exit 1
fi
cp "$root/crates/miso-engine-core/src/realtime/plan.rs" \
    "$temp/crates/miso-engine-core/src/realtime/plan.rs"

# Issue #95: the contract's dependency boundary is `miso-engine-core` plus `miso-engine-math` and
# nothing else. `miso-engine-math` is there because decision D6 forbids the platform libm, and it
# must stay; `miso-engine-lane` must never appear, because it would pin every control-plane
# consumer of the contract to an AVX2+FMA build.
restore_contract_manifest() {
    cp "$root/crates/miso-engine-effect-contract/Cargo.toml" "$contract_manifest"
}

expect_contract_dependency_failure() {
    local mutation="$1"
    if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
        printf 'effect contract dependency mutation escaped: %s\n' "$mutation" >&2
        exit 1
    fi
    restore_contract_manifest
}

sed -i '/^\[dependencies\]$/a miso-engine-lane.workspace = true' "$contract_manifest"
expect_contract_dependency_failure contract-gains-lane

sed -i '/^miso-engine-math[.]workspace = true$/d' "$contract_manifest"
expect_contract_dependency_failure contract-loses-math

printf '\npub struct EffectProgramSignature(pub [u8; 32]);\n' >>"$temp/crates/miso-engine-effect-contract/src/lib.rs"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'effect runtime identity mutation escaped\n' >&2
    exit 1
fi
printf 'effect runtime policy mutations: ok\n'
