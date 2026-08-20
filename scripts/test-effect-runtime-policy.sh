#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
cp -R "$root/crates" "$root/tools" "$root/docs" "$root/scripts" "$temp/"
printf '\nmiso-engine-effect-package.workspace = true\n' >>"$temp/crates/miso-engine-effect-compiler/Cargo.toml"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'effect runtime dependency mutation escaped\n' >&2; exit 1
fi
cp "$root/crates/miso-engine-effect-compiler/Cargo.toml" "$temp/crates/miso-engine-effect-compiler/Cargo.toml"
printf '\npub struct EffectProgramSignature(pub [u8; 32]);\n' >>"$temp/crates/miso-engine-effect-contract/src/lib.rs"
if bash "$temp/scripts/check-effect-runtime-policy.sh" "$temp" >/dev/null 2>&1; then
    printf 'effect runtime identity mutation escaped\n' >&2; exit 1
fi
printf 'effect runtime policy mutations: ok\n'
