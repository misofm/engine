#!/usr/bin/env bash
# Mutation tests for the frozen C ABI checker. No engine session or render workload is executed.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")" && pwd)"
workspace_root="$(cd "$script_directory/.." && pwd)"
checker="$script_directory/check-capi-abi.sh"
scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

cargo build --locked -p capi --manifest-path "$workspace_root/Cargo.toml" >/dev/null

host_triple="$(rustc -vV | sed -n 's/^host: //p')"
case "$host_triple" in
    *-linux-*) library="$workspace_root/target/debug/libcapi.so" ;;
    *-apple-*) library="$workspace_root/target/debug/libcapi.dylib" ;;
    *) printf 'unsupported pinned native host: %s\n' "$host_triple" >&2; exit 1 ;;
esac

expect_failure() {
    local name="$1"
    shift
    if "$@" >/dev/null 2>&1; then
        printf 'C ABI mutation unexpectedly passed: %s\n' "$name" >&2
        exit 1
    fi
}

common_env=(env MISO_ENGINE_CAPI_SKIP_BUILD=1 MISO_ENGINE_CAPI_LIBRARY="$library")
expect_failure missing-c-compiler \
    "${common_env[@]}" CC="$scratch_root/no-such-cc" bash "$checker" "$workspace_root"

abi_header="$scratch_root/abi-version.h"
cp "$workspace_root/crates/capi/include/miso_engine_v2.h" "$abi_header"
sed -i 's/UINT32_C(0x00010000)/UINT32_C(0x00010001)/' "$abi_header"
expect_failure header-constant-drift \
    "${common_env[@]}" MISO_ENGINE_CAPI_HEADER="$abi_header" bash "$checker" "$workspace_root"

layout_header="$scratch_root/layout.h"
cp "$workspace_root/crates/capi/include/miso_engine_v2.h" "$layout_header"
sed -i '0,/uint64_t reserved\[4\];/s//uint64_t reserved[3];/' "$layout_header"
expect_failure header-layout-drift \
    "${common_env[@]}" MISO_ENGINE_CAPI_HEADER="$layout_header" bash "$checker" "$workspace_root"

signature_header="$scratch_root/signature.h"
cp "$workspace_root/crates/capi/include/miso_engine_v2.h" "$signature_header"
sed -i 's/miso_engine_v2_abi_version(void)/miso_engine_v2_abi_version(uint32_t reserved)/' \
    "$signature_header"
expect_failure header-signature-drift \
    "${common_env[@]}" MISO_ENGINE_CAPI_HEADER="$signature_header" bash "$checker" "$workspace_root"

real_nm="$(command -v "${NM:-nm}")"
nm_add="$scratch_root/nm-add"
printf '%s\n' '#!/usr/bin/env bash' \
    '"${REAL_NM}" "$@"' \
    'printf "%s\n" "00000000 T miso_engine_v2_added"' >"$nm_add"
chmod +x "$nm_add"
expect_failure symbol-addition \
    "${common_env[@]}" NM="$nm_add" REAL_NM="$real_nm" bash "$checker" "$workspace_root"

nm_remove="$scratch_root/nm-remove"
printf '%s\n' '#!/usr/bin/env bash' \
    '"${REAL_NM}" "$@" | sed "/miso_engine_v2_abi_version/d"' >"$nm_remove"
chmod +x "$nm_remove"
expect_failure symbol-removal \
    "${common_env[@]}" NM="$nm_remove" REAL_NM="$real_nm" bash "$checker" "$workspace_root"

bad_library="$scratch_root/not-a-library.so"
printf '%s\n' 'not a native library' >"$bad_library"
expect_failure link-failure \
    env MISO_ENGINE_CAPI_SKIP_BUILD=1 MISO_ENGINE_CAPI_LIBRARY="$bad_library" \
    bash "$checker" "$workspace_root"

printf 'C ABI mutation tests: ok\n'
