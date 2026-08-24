#!/usr/bin/env bash
# Mutation tests proving every clause of the lane policy is enforced.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
policy_script="$script_directory/check-lane-policy.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

create_valid_fixture() {
    local root="$1"
    mkdir -p \
        "$root/crates/miso-engine-lane/src" \
        "$root/crates/miso-engine-lane/tests" \
        "$root/crates/miso-engine-core/src" \
        "$root/crates/miso-engine-compressor/src" \
        "$root/hosts/miso-engine-host-web/src" \
        "$root/tools/miso-engine-audit/src"

    printf '%s\n' \
        'pub use wide::f32x8 as Simd8;' \
        'pub fn flush(x: f32) -> f32 { x }' \
        >"$root/crates/miso-engine-lane/src/lib.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'use core::arch::x86_64::_mm_getcsr;' \
        'pub fn fma_f32_via_f64(a: f32, b: f32, c: f32) -> f32 { a * b + c }' \
        >"$root/crates/miso-engine-lane/src/softfma.rs"
    printf '%s\n' \
        'impl Lane for f32 {' \
        '    fn fma(self, b: Self, c: Self) -> Self {' \
        '        // LANE-OP-OK(mul_add): the oracle rounds once.' \
        '        f32::mul_add(self, b, c)' \
        '    }' \
        '}' \
        >"$root/crates/miso-engine-lane/src/scalar.rs"
    printf '%s\n' \
        'fn oracle(a: f32, b: f32, c: f32) -> f32 { f32::mul_add(a, b, c) }' \
        >"$root/crates/miso-engine-lane/tests/g3_softfma.rs"
    printf 'pub fn version() {}\n' >"$root/crates/miso-engine-core/src/lib.rs"
    printf 'pub fn process() {}\n' >"$root/crates/miso-engine-compressor/src/lib.rs"
    printf 'pub fn render() {}\n' >"$root/hosts/miso-engine-host-web/src/lib.rs"
    printf 'fn main() {}\n' >"$root/tools/miso-engine-audit/src/realtime.rs"

    printf '%s\n' \
        '[workspace.dependencies]' \
        'wide = { version = "=1.6.1", default-features = false }' \
        >"$root/Cargo.toml"
    printf '%s\n' \
        'version = 4' \
        '' \
        '[[package]]' \
        'name = "bytemuck"' \
        'version = "1.25.2"' \
        '' \
        '[[package]]' \
        'name = "safe_arch"' \
        'version = "1.2.0"' \
        'dependencies = [' \
        ' "bytemuck",' \
        ']' \
        '' \
        '[[package]]' \
        'name = "miso-engine-lane"' \
        'version = "0.1.0"' \
        'dependencies = [' \
        ' "wide",' \
        ']' \
        '' \
        '[[package]]' \
        'name = "wide"' \
        'version = "1.6.1"' \
        'dependencies = [' \
        ' "bytemuck",' \
        ' "safe_arch",' \
        ']' \
        >"$root/Cargo.lock"
}

# Each mutation is a shell fragment that edits the fixture rooted at `$root`.
expect_failure() {
    local fixture_name="$1"
    local fixture_root="$scratch_root/$fixture_name"
    shift
    create_valid_fixture "$fixture_root"
    local root="$fixture_root"
    eval "$@"

    if bash "$policy_script" "$fixture_root" >/dev/null 2>&1; then
        printf 'lane policy mutation unexpectedly passed: %s\n' "$fixture_name" >&2
        exit 1
    fi
}

valid_root="$scratch_root/valid root"
create_valid_fixture "$valid_root"
bash "$policy_script" "$valid_root" >/dev/null

expect_failure fusion-outside-lane \
    'printf "%s\n" "let y = a.mul_add(b, c);" >>"$root/crates/miso-engine-compressor/src/lib.rs"'
expect_failure wide-outside-lane \
    'printf "%s\n" "use wide::f32x4;" >>"$root/hosts/miso-engine-host-web/src/lib.rs"'
expect_failure arch-outside-softfma \
    'printf "%s\n" "use core::arch::x86_64::_mm256_add_ps;" >>"$root/tools/miso-engine-audit/src/realtime.rs"'
expect_failure arch-in-second-lane-file \
    'printf "%s\n" "use core::arch::x86_64::_mm256_add_ps;" >>"$root/crates/miso-engine-lane/src/scalar.rs"'
# #84 phase A: the legacy `core/arch` exemption is gone entirely, so an intrinsic there -- the
# very file the exemption used to name -- is now a failure like any other.
expect_failure deleted-core-arch-has-no-exemption \
    'mkdir -p "$root/crates/miso-engine-core/src/arch"; printf "%s\n" "use core::arch::x86_64::_mm256_fmadd_ps;" >"$root/crates/miso-engine-core/src/arch/x86.rs"'
expect_failure deleted-core-detection-has-no-exemption \
    'printf "%s\n" "pub fn detect() { let _ = is_x86_feature_detected!(\"avx2\"); }" >>"$root/crates/miso-engine-core/src/lib.rs"'
expect_failure relaxed-simd-anywhere \
    'printf "%s\n" "let y = f32x4_relaxed_madd(a, b, c);" >>"$root/crates/miso-engine-lane/src/lib.rs"'
expect_failure unmarked-wide-max \
    'printf "%s\n" "fn m(a: f32x8, b: f32x8) -> f32x8 { a.max(b) }" >>"$root/crates/miso-engine-lane/src/lib.rs"'
expect_failure unmarked-std-mul-add \
    'printf "%s\n" "fn f(a: f32) -> f32 { f32::mul_add(a, a, a) }" >>"$root/crates/miso-engine-lane/src/lib.rs"'
expect_failure new-runtime-detection \
    'printf "%s\n" "let _ = is_x86_feature_detected!(\"avx2\");" >>"$root/crates/miso-engine-compressor/src/lib.rs"'
expect_failure unpinned-wide-requirement \
    'sed -i "s/=1.6.1/^1.6/" "$root/Cargo.toml"'
expect_failure unpinned-wide-lock \
    'sed -i "s/version = \"1.6.1\"/version = \"1.7.0\"/" "$root/Cargo.lock"'
expect_failure foreign-lane-dependency \
    'sed -i "s/^ \"wide\",$/ \"wide\",\n \"rayon\",/" "$root/Cargo.lock"'
expect_failure foreign-wide-dependency \
    'sed -i "s/^ \"safe_arch\",$/ \"safe_arch\",\n \"serde\",/" "$root/Cargo.lock"'

printf 'lane policy mutation tests: ok\n'
