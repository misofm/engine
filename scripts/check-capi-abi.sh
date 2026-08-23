#!/usr/bin/env bash
# Check the frozen Issue-022 C header, exact exported symbols, and native consumer linkage.
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"

fail() {
    printf 'C ABI check failure: %s\n' "$1" >&2
    exit 1
}

cc_tool="${CC:-cc}"
cxx_tool="${CXX:-c++}"
nm_tool="${NM:-nm}"
command -v "$cc_tool" >/dev/null 2>&1 || fail "missing C compiler: $cc_tool"
command -v "$cxx_tool" >/dev/null 2>&1 || fail "missing C++ compiler: $cxx_tool"
command -v "$nm_tool" >/dev/null 2>&1 || fail "missing symbol tool: $nm_tool"

header="${MISO_CAPI_HEADER:-crates/miso-engine-capi/include/miso_engine_v2.h}"
c_fixture="${MISO_CAPI_C_FIXTURE:-crates/miso-engine-capi/tests/c/abi_smoke.c}"
cpp_fixture="${MISO_CAPI_CPP_FIXTURE:-crates/miso-engine-capi/tests/c/header_smoke.cpp}"
[[ -f "$header" ]] || fail "missing header: $header"
[[ -f "$c_fixture" ]] || fail "missing C11 fixture: $c_fixture"
[[ -f "$cpp_fixture" ]] || fail "missing C++17 fixture: $cpp_fixture"

if [[ "${MISO_CAPI_SKIP_BUILD:-0}" != 1 ]]; then
    cargo build --locked -p miso-engine-capi
fi

host_triple="$(rustc -vV | sed -n 's/^host: //p')"
case "$host_triple" in
    *-linux-*) default_library="target/debug/libmiso_engine_capi.so" ;;
    *-apple-*) default_library="target/debug/libmiso_engine_capi.dylib" ;;
    *) fail "unsupported pinned native host for ABI seal: $host_triple" ;;
esac
library="${MISO_CAPI_LIBRARY:-$default_library}"
[[ -f "$library" ]] || fail "missing native library: $library"

scratch_root="$(mktemp -d)"
trap 'rm -rf -- "$scratch_root"' EXIT

include_directory="$(cd "$(dirname "$header")" && pwd)"
library_path="$(cd "$(dirname "$library")" && pwd)/$(basename "$library")"

"$cxx_tool" -std=c++17 -Wall -Wextra -Werror -pedantic \
    -I"$include_directory" -c "$cpp_fixture" -o "$scratch_root/header-smoke.o"
"$cc_tool" -std=c11 -Wall -Wextra -Werror -pedantic \
    -I"$include_directory" "$c_fixture" "$library_path" \
    -Wl,-rpath,"$(dirname "$library_path")" -o "$scratch_root/abi-smoke"

actual_symbols="$scratch_root/actual-symbols.txt"
expected_symbols="$scratch_root/expected-symbols.txt"
if [[ "$host_triple" == *-apple-* ]]; then
    "$nm_tool" -gU "$library_path" | awk '{print $NF}' | sed 's/^_//' \
        | rg '^miso_engine_v2_' | sort >"$actual_symbols"
else
    "$nm_tool" -D --defined-only "$library_path" | awk '{print $NF}' \
        | rg '^miso_engine_v2_' | sort >"$actual_symbols"
fi
printf '%s\n' \
    miso_engine_v2_abi_version \
    miso_engine_v2_compile_session \
    miso_engine_v2_dequeue_event \
    miso_engine_v2_engine_create \
    miso_engine_v2_engine_destroy \
    miso_engine_v2_last_error \
    miso_engine_v2_plan_destroy \
    miso_engine_v2_plan_resources \
    miso_engine_v2_query_capabilities \
    miso_engine_v2_render_f32_planar \
    miso_engine_v2_session_destroy \
    miso_engine_v2_source_seek \
    miso_engine_v2_source_submit_planar_f32 \
    miso_engine_v2_submit_command \
    | sort >"$expected_symbols"
diff -u "$expected_symbols" "$actual_symbols" \
    || fail "exported symbol set differs from frozen ABI V1"

"$scratch_root/abi-smoke"
printf 'C ABI check: ok (%s)\n' "$host_triple"
