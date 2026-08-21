#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
cp -R "$root/crates" "$root/tools" "$root/docs" "$root/scripts" "$temp/"
compiler_manifest="$temp/crates/miso-engine-effect-compiler/Cargo.toml"

bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null

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

printf '\nmiso-engine-effect-package.workspace = true\n' >>"$temp/crates/miso-engine-effect-compiler/Cargo.toml"
expect_dependency_failure arbitrary-extra

sed -i '/^miso-engine-parametric-eq[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-parametric-eq
sed -i 's/^miso-engine-parametric-eq[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-parametric-eq
sed -i '/^miso-engine-compressor[.]workspace = true$/d' "$compiler_manifest"
expect_dependency_failure missing-compressor
sed -i 's/^miso-engine-compressor[.]workspace = true$/miso-engine-effect-package.workspace = true/' "$compiler_manifest"
expect_dependency_failure substituted-compressor

printf '\npub struct EffectProgramSignature(pub [u8; 32]);\n' >>"$temp/crates/miso-engine-effect-contract/src/lib.rs"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'effect runtime identity mutation escaped\n' >&2
    exit 1
fi
printf 'effect runtime policy mutations: ok\n'
