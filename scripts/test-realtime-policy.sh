#!/usr/bin/env bash
# Mutation tests proving the marked realtime policy and unsafe allowlist are enforced.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
policy_script="$script_directory/check-realtime-policy.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

create_fixture() {
    local root="$1"
    mkdir -p "$root/crates/miso-engine-core/src/realtime" \
        "$root/crates/miso-engine-core/src/arch" \
        "$root/crates/miso-engine-capi/src" \
        "$root/crates/miso-engine-effect-package/src" \
        "$root/crates/miso-engine-effect-package/tests" \
        "$root/hosts/miso-engine-host-web/src" \
        "$root/tools/miso-engine-capi-audit/src" \
        "$root/tools/miso-engine-realtime-audit/src" \
        "$root/tools/miso-engine-protocol-audit/src" \
        "$root/tools/miso-engine-rack-bench/src"
    printf '%s\n' \
        '// REALTIME_POLICY_BEGIN' \
        'fn render() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn queue() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn buffer() {}' \
        '// REALTIME_POLICY_END' \
        '// REALTIME_POLICY_BEGIN' \
        'fn exchange() {}' \
        '// REALTIME_POLICY_END' \
        >"$root/crates/miso-engine-core/src/realtime/mod.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for Allowed {}' \
        'struct Allowed;' \
        >"$root/crates/miso-engine-core/src/realtime/spsc.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn architecture_kernel() {}' \
        >"$root/crates/miso-engine-core/src/arch/x86.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn capi_boundary() {}' \
        >"$root/crates/miso-engine-capi/src/ffi.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn descriptor_capi_boundary() {}' \
        >"$root/crates/miso-engine-effect-package/src/ffi.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for PackageAllocationAudit {}' \
        'struct PackageAllocationAudit;' \
        >"$root/crates/miso-engine-effect-package/tests/package_allocation.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn web_boundary() {}' \
        >"$root/hosts/miso-engine-host-web/src/ffi.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for CapiAudit {}' \
        'struct CapiAudit;' \
        >"$root/tools/miso-engine-capi-audit/src/main.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for Audit {}' \
        'struct Audit;' \
        >"$root/tools/miso-engine-realtime-audit/src/main.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for ProtocolAudit {}' \
        'struct ProtocolAudit;' \
        >"$root/tools/miso-engine-protocol-audit/src/main.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for RackBenchmarkAudit {}' \
        'struct RackBenchmarkAudit;' \
        >"$root/tools/miso-engine-rack-bench/src/main.rs"
}

expect_failure() {
    local name="$1"
    local root="$scratch_root/$name"
    local mutation="$2"
    create_fixture "$root"
    eval "$mutation"
    if bash "$policy_script" "$root" >/dev/null 2>&1; then
        printf 'realtime policy mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
}

valid="$scratch_root/valid"
create_fixture "$valid"
bash "$policy_script" "$valid" >/dev/null

expect_failure allocation \
    'sed -i "s/fn render() {}/fn render() { let _ = Vec::new(); }/" "$root/crates/miso-engine-core/src/realtime/mod.rs"'
expect_failure lock \
    'sed -i "s/fn queue() {}/fn queue() { let _ = Mutex::new(0); }/" "$root/crates/miso-engine-core/src/realtime/mod.rs"'
expect_failure log \
    'sed -i "s/fn buffer() {}/fn buffer() { println!(\"bad\"); }/" "$root/crates/miso-engine-core/src/realtime/mod.rs"'
expect_failure unsafe-scope \
    'printf "%s\n" "unsafe fn bad() {}" >>"$root/crates/miso-engine-core/src/realtime/mod.rs"'
expect_failure unsafe-outside-exact-allowlist \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/miso-engine-protocol-audit/src/other.rs"'
expect_failure unsafe-outside-capi-audit-main \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/miso-engine-capi-audit/src/other.rs"'
expect_failure unsafe-outside-architecture-allowlist \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/miso-engine-core/src/arch/other.rs"'
expect_failure unsafe-outside-rack-benchmark-main \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/miso-engine-rack-bench/src/other.rs"'
expect_failure unsafe-outside-capi-ffi \
    'printf "%s\n" "pub unsafe extern \"C\" fn bad() {}" >"$root/crates/miso-engine-capi/src/lib.rs"'
expect_failure unsafe-in-second-capi-ffi-path \
    'mkdir -p "$root/crates/miso-engine-capi/src/ffi"; printf "%s\n" "unsafe fn bad() {}" >"$root/crates/miso-engine-capi/src/ffi/other.rs"'
expect_failure unsafe-outside-effect-package-ffi \
    'printf "%s\n" "pub unsafe extern \"C\" fn bad() {}" >"$root/crates/miso-engine-effect-package/src/lib.rs"'
expect_failure unsafe-in-second-effect-package-ffi-path \
    'mkdir -p "$root/crates/miso-engine-effect-package/src/ffi"; printf "%s\n" "unsafe fn bad() {}" >"$root/crates/miso-engine-effect-package/src/ffi/other.rs"'
expect_failure unsafe-outside-package-allocation-audit \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/miso-engine-effect-package/tests/other.rs"'
expect_failure unsafe-outside-web-ffi \
    'printf "%s\n" "pub unsafe extern \"C\" fn bad() {}" >"$root/hosts/miso-engine-host-web/src/lib.rs"'
expect_failure unsafe-in-second-web-ffi-path \
    'mkdir -p "$root/hosts/miso-engine-host-web/src/ffi"; printf "%s\n" "unsafe fn bad() {}" >"$root/hosts/miso-engine-host-web/src/ffi/other.rs"'

printf 'realtime policy mutation tests: ok\n'
