#!/usr/bin/env bash
# Mutation tests proving the marked realtime policy and unsafe allowlist are enforced.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
policy_script="$script_directory/check-realtime-policy.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

create_fixture() {
    local root="$1"
    mkdir -p "$root/crates/engine/src/realtime" \
        "$root/crates/lane/src" \
        "$root/crates/capi/src" \
        "$root/crates/capi/tests" \
        "$root/crates/effect-compiler/tests" \
        "$root/crates/effect-package/src" \
        "$root/crates/effect-package/tests" \
        "$root/crates/session/tests" \
        "$root/hosts/host-web/src" \
        "$root/hosts/host-web/tests" \
        "$root/tools/bench-support/src" \
        "$root/tools/audit/src" \
        "$root/tools/native-pcm-runner/src" \
        "$root/tools/bench/src"
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
        >"$root/crates/engine/src/realtime/mod.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for Allowed {}' \
        'struct Allowed;' \
        >"$root/crates/engine/src/realtime/spsc.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Sync for DisjointArena {}' \
        'struct DisjointArena;' \
        >"$root/crates/engine/src/realtime/disjoint.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn read_mxcsr() {}' \
        >"$root/crates/lane/src/softfma.rs"
    # Issue #146: the AArch64 FPCR pair of the canonical render-entry environment.
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn write_fpcr() {}' \
        >"$root/crates/lane/src/fpenv.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn capi_boundary() {}' \
        >"$root/crates/capi/src/ffi.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for LifecycleAllocator {}' \
        'struct LifecycleAllocator;' \
        >"$root/crates/capi/tests/resource_lifecycle.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn descriptor_capi_boundary() {}' \
        >"$root/crates/effect-package/src/ffi.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for PackageAllocationAudit {}' \
        'struct PackageAllocationAudit;' \
        >"$root/crates/effect-package/tests/package_allocation.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for MigrationAllocationAudit {}' \
        'struct MigrationAllocationAudit;' \
        >"$root/crates/effect-compiler/tests/migration_terminal.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for CountingAllocator {}' \
        'struct CountingAllocator;' \
        >"$root/crates/session/tests/allocation_budget.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn web_boundary() {}' \
        >"$root/hosts/host-web/src/ffi.rs"
    # Issue #240: the exact peak fixture's forwarding allocator is a measured audit boundary.
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for PeakAllocator {}' \
        'struct PeakAllocator;' \
        >"$root/hosts/host-web/tests/boot_transient_budget.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl Send for CapiAudit {}' \
        'struct CapiAudit;' \
        >"$root/tools/audit/src/capi.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn frozen_c_abi_adapter() {}' \
        >"$root/tools/native-pcm-runner/src/lib.rs"
    # #104 phase B: the fourteen audited `GlobalAlloc` copies became one. `bench-support/src/alloc.rs`
    # is the only file under `tools/` that owns the allocator wrapper, and eleven tool paths left
    # this list because they no longer contain `unsafe` at all.
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe impl GlobalAlloc for AuditedAllocator {}' \
        'struct AuditedAllocator;' \
        >"$root/tools/bench-support/src/alloc.rs"
    printf '%s\n' \
        '#![allow(unsafe_code)]' \
        'unsafe fn follow() {}' \
        >"$root/tools/bench/src/protocol.rs"
    printf '%s\n' \
        'fn measure() {}' \
        >"$root/tools/bench/src/rack.rs"
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
    'sed -i "s/fn render() {}/fn render() { let _ = Vec::new(); }/" "$root/crates/engine/src/realtime/mod.rs"'
expect_failure lock \
    'sed -i "s/fn queue() {}/fn queue() { let _ = Mutex::new(0); }/" "$root/crates/engine/src/realtime/mod.rs"'
expect_failure log \
    'sed -i "s/fn buffer() {}/fn buffer() { println!(\"bad\"); }/" "$root/crates/engine/src/realtime/mod.rs"'
# #84 phase B (F12): a panic path is a realtime violation like an allocation is. `LocalRing`'s
# `.take().expect("prepared local ring slot")` was the only hit inside a marked region; it is gone,
# and the regex now keeps it gone.
expect_failure panic-path-expect \
    'sed -i "s/fn exchange() {}/fn exchange() { None::<u8>.expect(\"x\"); }/" "$root/crates/engine/src/realtime/mod.rs"'
expect_failure panic-path-macro \
    'sed -i "s/fn exchange() {}/fn exchange() { unreachable!(); }/" "$root/crates/engine/src/realtime/mod.rs"'
expect_failure unsafe-scope \
    'printf "%s\n" "unsafe fn bad() {}" >>"$root/crates/engine/src/realtime/mod.rs"'
expect_failure unsafe-outside-exact-allowlist \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/bench/src/other.rs"'
expect_failure unsafe-outside-capi-audit-main \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/audit/src/other.rs"'
expect_failure unsafe-outside-native-pcm-runner-lib \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/native-pcm-runner/src/other.rs"'
# #84 phase A deleted `crates/engine/src/arch/`; its unsafe exemption went with it, so
# unsafe code re-appearing under that path is now rejected like any other unlisted file.
expect_failure unsafe-in-deleted-core-arch \
    'mkdir -p "$root/crates/engine/src/arch"; printf "%s\n" "unsafe fn bad() {}" >"$root/crates/engine/src/arch/x86.rs"'
expect_failure unsafe-outside-rack-benchmark-main \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/tools/bench/src/other.rs"'
expect_failure unsafe-outside-capi-ffi \
    'printf "%s\n" "pub unsafe extern \"C\" fn bad() {}" >"$root/crates/capi/src/lib.rs"'
expect_failure unsafe-in-second-capi-ffi-path \
    'mkdir -p "$root/crates/capi/src/ffi"; printf "%s\n" "unsafe fn bad() {}" >"$root/crates/capi/src/ffi/other.rs"'
expect_failure unsafe-outside-capi-lifecycle-audit \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/capi/tests/other.rs"'
expect_failure unsafe-outside-effect-package-ffi \
    'printf "%s\n" "pub unsafe extern \"C\" fn bad() {}" >"$root/crates/effect-package/src/lib.rs"'
expect_failure unsafe-in-second-effect-package-ffi-path \
    'mkdir -p "$root/crates/effect-package/src/ffi"; printf "%s\n" "unsafe fn bad() {}" >"$root/crates/effect-package/src/ffi/other.rs"'
expect_failure unsafe-outside-package-allocation-audit \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/effect-package/tests/other.rs"'
expect_failure unsafe-outside-migration-allocation-audit \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/effect-compiler/tests/other.rs"'
expect_failure unsafe-outside-session-allocation-budget \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/session/tests/other.rs"'
expect_failure unsafe-outside-web-ffi \
    'printf "%s\n" "pub unsafe extern \"C\" fn bad() {}" >"$root/hosts/host-web/src/lib.rs"'
expect_failure unsafe-in-second-web-ffi-path \
    'mkdir -p "$root/hosts/host-web/src/ffi"; printf "%s\n" "unsafe fn bad() {}" >"$root/hosts/host-web/src/ffi/other.rs"'
expect_failure unsafe-outside-web-peak-audit \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/hosts/host-web/tests/other.rs"'
expect_failure unsafe-outside-disjoint-arena \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/engine/src/realtime/disjoint_extra.rs"'
expect_failure unsafe-outside-lane-softfma \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/lane/src/kernels.rs"'
expect_failure unsafe-outside-lane-fpenv \
    'printf "%s\n" "unsafe fn bad() {}" >"$root/crates/lane/src/fpenv_extra.rs"'

printf 'realtime policy mutation tests: ok\n'
