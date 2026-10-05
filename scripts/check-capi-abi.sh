#!/usr/bin/env bash
# Check the frozen Issue-022 C header, exact exported symbols, and native consumer linkage.
#
# S4 (#359): the default path below still does a full `cargo build --locked --release -p capi`, so
# a bare invocation stays self-contained. But that build resolves a different feature/package set
# than the audit-native CI shard's single `cargo build --locked --release -p audit -p bench
# -p session-validator -p capi`, so running both back to back in the same shard's `target/`
# alternately invalidates each other's fingerprints and rebuilds every time. The shard-friendly
# path is first-class, not a workaround: it builds that four-package release set exactly once,
# then calls this script with `MISO_ENGINE_CAPI_SKIP_BUILD=1` (an existing hook, no new env name)
# so this script skips its own build and links straight against the release artifacts the shard
# already produced (`target/release/libcapi.so`/`.a`, overridable via `MISO_ENGINE_CAPI_LIBRARY`
# / `MISO_ENGINE_CAPI_STATIC_LIBRARY` exactly as before).
set -euo pipefail


# ---------------------------------------------------------------------------------------------
# --self-test: mutation tests for this checker (issue #104-shape: folded in from the former
# test-capi-abi.sh so the gate and its own mutation proof cannot drift apart or be wired in
# separately). Adds a positive control the original file never had -- the unmutated checker
# must pass before any mutation is asked to fail it.
capi_abi_self_test() {
    local script_directory workspace_root scratch_root host_triple library
    script_directory="$(cd "$(dirname "$0")" && pwd)"
    workspace_root="$(cd "$script_directory/.." && pwd)"
    scratch_root="$(mktemp -d)"
    trap 'rm -rf -- "$scratch_root"' RETURN
    cargo build --locked --release -p capi --manifest-path "$workspace_root/Cargo.toml" >/dev/null

    host_triple="$(rustc -vV | sed -n 's/^host: //p')"
    case "$host_triple" in
        *-linux-*) library="$workspace_root/target/release/libcapi.so" ;;
        *-apple-*) library="$workspace_root/target/release/libcapi.dylib" ;;
        *) printf 'unsupported pinned native host: %s\n' "$host_triple" >&2; exit 1 ;;
    esac

    expect_failure() {
        local name="$1"
        shift
        if "$@" >/dev/null 2>&1; then
            printf 'C ABI mutation unexpectedly passed: %s\n' "$name" >&2
            return 1
        fi
    }

    common_env=(env MISO_ENGINE_CAPI_SKIP_BUILD=1 MISO_ENGINE_CAPI_LIBRARY="$library")

        # Positive control: the unmutated checker must pass against the real header and library
        # before any mutation is asked to fail it. Without this, a checker "hardened" into
        # refusing everything would pass every expect_failure case below.
        "${common_env[@]}" bash "$0" "$workspace_root" >/dev/null ||
            { printf 'C ABI mutation self-test FAILED: baseline (unmutated) run did not pass\n' >&2; return 1; }

    expect_failure missing-c-compiler \
        "${common_env[@]}" CC="$scratch_root/no-such-cc" bash "$0" "$workspace_root"

    # Header legs (#1232). The fixtures `#include "miso_engine_v1.h"` and the checker adds only
    # `-I"$(dirname "$header")"`, so a staged header must be named `miso_engine_v1.h` in its own
    # directory or every run stops at "No such file or directory" before any assertion runs.
    original_header="$workspace_root/crates/capi/include/miso_engine_v1.h"
    stage_header() {
        mkdir -p "$scratch_root/$1"
        cp "$original_header" "$scratch_root/$1/miso_engine_v1.h"
        printf '%s\n' "$scratch_root/$1/miso_engine_v1.h"
    }
    expect_header_failure() {
        local name="$1" staged="$2" log="$scratch_root/$1.stderr" cmp_status=0
        if [[ ! -f "$staged" ]]; then
            printf 'C ABI mutation self-test FAILED: %s staged header is missing: %s\n' \
                "$name" "$staged" >&2
            return 1
        fi
        cmp -s "$original_header" "$staged" || cmp_status=$?
        case "$cmp_status" in
            1) ;;
            0)
                printf 'C ABI mutation self-test FAILED: %s did not change the staged header\n' \
                    "$name" >&2
                return 1
                ;;
            *)
                printf 'C ABI mutation self-test FAILED: %s could not compare the staged header (cmp status %s)\n' \
                    "$name" "$cmp_status" >&2
                return 1
                ;;
        esac
        # The mutated header must still compile alone, so the leg's red has to come from the
        # fixtures' pins, not from a mutation that broke the header's syntax.
        if ! "${CC:-cc}" -std=c11 -Wall -Wextra -Werror -pedantic -fsyntax-only -x c "$staged" \
            2>"$log"; then
            printf 'C ABI mutation self-test FAILED: %s staged header does not compile alone:\n' \
                "$name" >&2
            cat "$log" >&2
            return 1
        fi
        # LC_ALL=C keeps the compiler's missing-file text in English; GCC says `No such file or
        # directory` and clang says `'miso_engine_v1.h' file not found`.
        if "${common_env[@]}" LC_ALL=C MISO_ENGINE_CAPI_HEADER="$staged" bash "$0" \
            "$workspace_root" >/dev/null 2>"$log"; then
            printf 'C ABI mutation unexpectedly passed: %s\n' "$name" >&2
            return 1
        fi
        if grep -qE "No such file or directory|'miso_engine_v1\.h' file not found" "$log"; then
            printf 'C ABI mutation self-test FAILED: %s failed on a missing file, not on drift:\n' \
                "$name" >&2
            cat "$log" >&2
            return 1
        fi
    }

    # Header positive control: an unmodified copy, staged exactly as the legs stage theirs, must
    # pass. Without it a staging defect would make every header leg red for the wrong reason.
    control_header="$(stage_header header-staging-control)"
    "${common_env[@]}" MISO_ENGINE_CAPI_HEADER="$control_header" bash "$0" "$workspace_root" \
        >/dev/null ||
        { printf 'C ABI mutation self-test FAILED: header staging control (unmodified staged header) did not pass\n' >&2; return 1; }

    abi_header="$(stage_header abi-version)"
    sed -i 's/UINT32_C(0x00010000)/UINT32_C(0x00010001)/' "$abi_header"
    expect_header_failure header-constant-drift "$abi_header"

    # The first `uint64_t reserved[4];` is miso_engine_v1_engine_config's tail (header at #1232).
    layout_header="$(stage_header layout)"
    sed -i '0,/uint64_t reserved\[4\];/s//uint64_t reserved[3];/' "$layout_header"
    expect_header_failure header-layout-drift "$layout_header"

    signature_header="$(stage_header signature)"
    sed -i 's/miso_engine_v1_abi_version(void)/miso_engine_v1_abi_version(uint32_t reserved)/' \
        "$signature_header"
    expect_header_failure header-signature-drift "$signature_header"

    real_nm="$(command -v "${NM:-nm}")"
    nm_add="$scratch_root/nm-add"
    printf '%s\n' '#!/usr/bin/env bash' \
        '"${REAL_NM}" "$@"' \
        'printf "%s\n" "00000000 T miso_engine_v1_added"' >"$nm_add"
    chmod +x "$nm_add"
    expect_failure symbol-addition \
        "${common_env[@]}" NM="$nm_add" REAL_NM="$real_nm" bash "$0" "$workspace_root"

    nm_remove="$scratch_root/nm-remove"
    printf '%s\n' '#!/usr/bin/env bash' \
        '"${REAL_NM}" "$@" | sed "/miso_engine_v1_abi_version/d"' >"$nm_remove"
    chmod +x "$nm_remove"
    expect_failure symbol-removal \
        "${common_env[@]}" NM="$nm_remove" REAL_NM="$real_nm" bash "$0" "$workspace_root"

    # Issue #1275: a library whose miso_engine_v1_source_seek_at is only an undefined reference.
    # The wrapper models one: a defined-only listing (`--defined-only`, or `-gU`/`-U` on Apple)
    # omits the symbol, and any other listing shows it as `U`. The checker refuses this library
    # only while it lists defined symbols alone and holds the frozen set exact.
    nm_undefined="$scratch_root/nm-undefined"
    printf '%s\n' '#!/usr/bin/env bash' \
        'if [[ " $* " == *" --defined-only "* || " $* " == *" -gU "* || " $* " == *" -U "* ]]; then' \
        '    "${REAL_NM}" "$@" | sed "/miso_engine_v1_source_seek_at$/d"' \
        'else' \
        '    "${REAL_NM}" "$@" | sed "s/^.* \([A-Za-z]\) miso_engine_v1_source_seek_at$/                 U miso_engine_v1_source_seek_at/"' \
        'fi' >"$nm_undefined"
    chmod +x "$nm_undefined"
    expect_failure seek-at-undefined-reference \
        "${common_env[@]}" NM="$nm_undefined" REAL_NM="$real_nm" bash "$0" "$workspace_root"

    bad_library="$scratch_root/not-a-library.so"
    printf '%s\n' 'not a native library' >"$bad_library"
    expect_failure link-failure \
        env MISO_ENGINE_CAPI_SKIP_BUILD=1 MISO_ENGINE_CAPI_LIBRARY="$bad_library" \
        bash "$0" "$workspace_root"

    bad_static_library="$scratch_root/not-a-library.a"
    printf '%s\n' 'not a static archive' >"$bad_static_library"
    expect_failure static-link-failure \
        "${common_env[@]}" MISO_ENGINE_CAPI_STATIC_LIBRARY="$bad_static_library" \
        bash "$0" "$workspace_root"

    printf 'C ABI mutation tests: ok\n'
}

if [[ "${1:-}" == "--self-test" ]]; then
    capi_abi_self_test
    exit $?
fi

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

header="${MISO_ENGINE_CAPI_HEADER:-crates/capi/include/miso_engine_v1.h}"
c_fixture="${MISO_ENGINE_CAPI_C_FIXTURE:-crates/capi/tests/c/abi_smoke.c}"
cpp_fixture="${MISO_ENGINE_CAPI_CPP_FIXTURE:-crates/capi/tests/c/header_smoke.cpp}"
[[ -f "$header" ]] || fail "missing header: $header"
[[ -f "$c_fixture" ]] || fail "missing C11 fixture: $c_fixture"
[[ -f "$cpp_fixture" ]] || fail "missing C++17 fixture: $cpp_fixture"

if [[ "${MISO_ENGINE_CAPI_SKIP_BUILD:-0}" != 1 ]]; then
    cargo build --locked --release -p capi
fi

host_triple="$(rustc -vV | sed -n 's/^host: //p')"
case "$host_triple" in
    *-linux-*) default_library="target/release/libcapi.so" ;;
    *-apple-*) default_library="target/release/libcapi.dylib" ;;
    *) fail "unsupported pinned native host for ABI seal: $host_triple" ;;
esac
library="${MISO_ENGINE_CAPI_LIBRARY:-$default_library}"
[[ -f "$library" ]] || fail "missing native library: $library"
# The staticlib leg (issue #114's toolchain matrix named both static and shared linkage, but
# never automated either -- it only grepped its own now-deleted runner script's source for the
# words "static"/"shared"). `crates/capi`'s crate-type list includes `staticlib`, so this actually
# links and runs, rather than asserting a string appears in a script nobody ran.
#
# Release, not debug: release is the shipped profile, and the shared/static objects this script
# links against must be the ones a consumer actually gets (WP-3, #359).
static_library="${MISO_ENGINE_CAPI_STATIC_LIBRARY:-target/release/libcapi.a}"
[[ -f "$static_library" ]] || fail "missing native static library: $static_library"

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
        | rg '^miso_engine_v1_' | sort >"$actual_symbols"
else
    "$nm_tool" -D --defined-only "$library_path" | awk '{print $NF}' \
        | rg '^miso_engine_v1_' | sort >"$actual_symbols"
fi
printf '%s\n' \
    miso_engine_v1_abi_version \
    miso_engine_v1_compile_session \
    miso_engine_v1_dequeue_event \
    miso_engine_v1_engine_create \
    miso_engine_v1_engine_destroy \
    miso_engine_v1_last_error \
    miso_engine_v1_plan_destroy \
    miso_engine_v1_plan_resources \
    miso_engine_v1_query_capabilities \
    miso_engine_v1_render_f32_planar \
    miso_engine_v1_session_destroy \
    miso_engine_v1_source_seek \
    miso_engine_v1_source_seek_at \
    miso_engine_v1_source_submit_planar_f32 \
    miso_engine_v1_submit_command \
    | sort >"$expected_symbols"
diff -u "$expected_symbols" "$actual_symbols" \
    || fail "exported symbol set differs from frozen ABI V1"

"$scratch_root/abi-smoke"

# Static linkage: the same C11 fixture, against the .a instead of the .so. Rust staticlibs pull in
# libc's own pthread/dl/m on this pinned toolchain; linked explicitly rather than relying on a
# default that can vary by distro.
static_library_path="$(cd "$(dirname "$static_library")" && pwd)/$(basename "$static_library")"
if [[ "$host_triple" == *-apple-* ]]; then
    "$cc_tool" -std=c11 -Wall -Wextra -Werror -pedantic \
        -I"$include_directory" "$c_fixture" "$static_library_path" \
        -o "$scratch_root/abi-smoke-static"
else
    "$cc_tool" -std=c11 -Wall -Wextra -Werror -pedantic \
        -I"$include_directory" "$c_fixture" "$static_library_path" \
        -lpthread -ldl -lm -o "$scratch_root/abi-smoke-static"
fi
"$scratch_root/abi-smoke-static"

printf 'C ABI check: ok (%s, shared and static linkage)\n' "$host_triple"
