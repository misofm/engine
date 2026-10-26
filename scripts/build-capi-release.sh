#!/usr/bin/env bash
# Build a C ABI library the way it ships: `capi` with the release profile's fat LTO (#1499).
#
# Usage:
#   build-capi-release.sh [--emit-asm FILE] <target> <crate-type> <out-dir>
#   build-capi-release.sh --check <crate-type> <artifact>
#   build-capi-release.sh --self-test
#
# <target> is a target triple, or `host` for the native host build (no `--target`, so the library
# lands at `target/release/`, which the audit-native shard reads). <crate-type> is `staticlib` or
# `cdylib`. The shipped mobile libraries are the `aarch64-apple-ios` and `aarch64-linux-android`
# staticlibs (docs/TARGET_MATRIX.md); the `host` cdylib (`libcapi.so`, or `libcapi.dylib` on an
# Apple host) and staticlib are the forms scripts/check-capi-abi.sh links, not shipped. A cdylib is
# built for the host only: a cross-target cdylib needs a target linker (the NDK, Xcode).
#
# Why `cargo rustc --crate-type`: cargo passes `-C lto` to a library only when every crate type it
# builds can be LTO'd. `crates/capi/Cargo.toml` lists `rlib` too (tools/audit and capi's own tests
# link it as a Rust library), and an rlib cannot be, so `cargo build -p capi` builds capi with no
# LTO at all. Asking for one crate type gets `-C lto=fat` under the unchanged release profile.
#
# The artifact is copied to <out-dir> (never left only at the shared `target/.../libcapi.*` path,
# which the next build of another form overwrites), and the copy is then judged by its own content:
#   staticlib: the archive's Rust objects are exactly one `capi` module plus `compiler_builtins`
#              (the C builtins objects are not Rust objects and are not counted). A non-LTO archive
#              holds one or more objects per crate (`lane-*`, `builtins-*`, `std-*`, ...); a
#              thin-LTO archive holds many `capi-*` objects, which the "not exactly one" error
#              catches.
#   cdylib, on an x86_64 Linux host: `miso_engine_v1_render_f32_planar` holds the floating-point
#              guard's STMXCSR and LDMXCSR inline and calls no `lane::softfma` helper; without LTO
#              they are out-of-line calls. On AArch64 the guard is inline in both forms, so there is
#              no cdylib marker: on any other host the cdylib is built by the same command and not
#              checked, and the same command's staticlib is.
# --emit-asm FILE also writes the same compilation's assembly to FILE (scripts/check-cross-targets.sh
# scans it), so a scan reads exactly the library that ships.
set -euo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/lib/gate.sh"

usage() {
    printf 'usage: build-capi-release.sh [--emit-asm FILE] <target|host> <staticlib|cdylib> <out-dir>\n' >&2
    printf '       build-capi-release.sh --check <staticlib|cdylib> <artifact>\n' >&2
    printf '       build-capi-release.sh --self-test\n' >&2
    exit 2
}

fail() {
    printf 'C ABI release build failure: %s\n' "$1" >&2
    exit 1
}

# Judge a staticlib by its member names. `ar t` reads both GNU and BSD (Apple) archives.
check_staticlib() {
    local artifact="$1" members rust crates
    members="$(ar t "$artifact")" || fail "cannot list the archive members of $artifact"
    [[ -n "$members" ]] || fail "$artifact has no archive members"
    # A rustc object member is `<crate>-<16 hex>.<crate>.<hash>-cgu.<n>.rcgu.o`.
    rust="$(gate_scan_text_collect 'archive member' '^[A-Za-z0-9_]+-[0-9a-f]{16}\..*\.rcgu\.o$' \
        "$members")" || fail "could not scan the archive members of $artifact"
    rust="$(sed -E 's/^[0-9]+://' <<<"$rust")"
    crates="$(sed -E 's/-[0-9a-f]{16}\..*$//' <<<"$rust" | sort | uniq -c | sed -E 's/^ +//')"
    local count crate capi_count=0 others=()
    while read -r count crate; do
        [[ -n "$crate" ]] || continue
        case "$crate" in
            capi) capi_count="$count" ;;
            compiler_builtins) ;;
            *) others+=("$crate") ;;
        esac
    done <<<"$crates"
    if ((${#others[@]})); then
        fail "$artifact is not the fat-LTO build: it holds per-crate Rust objects (${others[*]:0:6}${others[6]:+ ...}; ${#others[@]} crates besides capi and compiler_builtins)"
    fi
    ((capi_count == 1)) ||
        fail "$artifact is not the fat-LTO build: it holds $capi_count capi objects, not exactly one"
}

# Judge an x86_64 Linux cdylib by the render entry's disassembly. Other hosts have no marker.
check_cdylib() {
    local artifact="$1" body host
    host="$(rustc -vV | sed -n 's/^host: //p')"
    if [[ "$host" != x86_64-*-linux-* ]]; then
        printf 'no cdylib LTO check on %s; the same command'"'"'s staticlib is checked\n' "$host" >&2
        return 0
    fi
    require_tools objdump
    body="$(objdump -d -C --no-show-raw-insn --disassemble=miso_engine_v1_render_f32_planar \
        "$artifact")" || fail "objdump could not disassemble $artifact"
    rg -q '<miso_engine_v1_render_f32_planar>:' <<<"$body" ||
        fail "$artifact has no miso_engine_v1_render_f32_planar to read"
    # A named helper call is refused, but objdump rarely names it: the non-LTO cdylib calls
    # through unnamed GOT slots. The real red case is the missing inline MXCSR below.
    if rg -q 'call.*softfma' <<<"$body"; then
        fail "$artifact is not the fat-LTO build: miso_engine_v1_render_f32_planar calls a lane::softfma helper"
    fi
    rg -q 'v?stmxcsr' <<<"$body" && rg -q 'v?ldmxcsr' <<<"$body" ||
        fail "$artifact is not the fat-LTO build: the render entry has no inline STMXCSR/LDMXCSR"
}

check_artifact() {
    local crate_type="$1" artifact="$2"
    [[ -f "$artifact" ]] || fail "missing artifact: $artifact"
    case "$crate_type" in
        staticlib) require_tools ar; check_staticlib "$artifact" ;;
        cdylib) require_tools rustc; check_cdylib "$artifact" ;;
        *) usage ;;
    esac
    printf 'C ABI release check: ok (%s %s)\n' "$crate_type" "$artifact"
}

# Hermetic: synthesized archives and a tiny assembled library; builds no Rust.
self_test() {
    require_tools ar cc
    local scratch self
    self="${BASH_SOURCE[0]}"
    scratch="$(mktemp -d)"
    trap 'rm -rf -- "$scratch"' RETURN
    expect() {
        local name="$1" want="$2" message="$3" output rc
        shift 3
        if output="$("$@" 2>&1)"; then rc=0; else rc=$?; fi
        if [[ "$want" == pass ]]; then
            ((rc == 0)) || { printf 'self-test %s: expected pass, got exit %s: %s\n' "$name" "$rc" "$output" >&2; return 1; }
        else
            ((rc == 1)) || { printf 'self-test %s: expected exit 1, got exit %s: %s\n' "$name" "$rc" "$output" >&2; return 1; }
            [[ "$output" == *"$message"* ]] ||
                { printf 'self-test %s: wrong diagnostic: %s\n' "$name" "$output" >&2; return 1; }
        fi
    }
    archive() {
        local name="$1" member
        shift
        mkdir -p "$scratch/$name"
        for member in "$@"; do : >"$scratch/$name/$member"; done
        (cd "$scratch/$name" && ar rc "../$name.a" "$@")
    }
    local capi='capi-312169de264e87fd.capi.d328571eeb638189-cgu.0.rcgu.o'
    local cb1='compiler_builtins-0123456789abcdef.compiler_builtins.aaaa-cgu.000.rcgu.o'
    local cb2='compiler_builtins-0123456789abcdef.compiler_builtins.aaaa-cgu.001.rcgu.o'
    local lane='lane-78bef5ce2c463721.lane.bbbb-cgu.0.rcgu.o'
    archive lto "$capi" "$cb1" "$cb2" 43aac466c8bb2fc4-lse_cas1_relax.o
    archive nolto "$capi" "$cb1" "$lane" 43aac466c8bb2fc4-lse_cas1_relax.o
    archive twocapi "$capi" 'capi-312169de264e87fd.capi.d328571eeb638189-cgu.1.rcgu.o' "$cb1"
    archive nocapi "$cb1" 43aac466c8bb2fc4-lse_cas1_relax.o
    expect staticlib-lto pass '' bash "$self" --check staticlib "$scratch/lto.a"
    expect staticlib-per-crate fail 'per-crate Rust objects (lane' \
        bash "$self" --check staticlib "$scratch/nolto.a"
    expect staticlib-two-capi fail '2 capi objects' bash "$self" --check staticlib "$scratch/twocapi.a"
    expect staticlib-no-capi fail '0 capi objects' bash "$self" --check staticlib "$scratch/nocapi.a"
    expect staticlib-not-archive fail 'cannot list' bash "$self" --check staticlib "$self"

    if [[ "$(rustc -vV | sed -n 's/^host: //p')" == x86_64-*-linux-* ]]; then
        printf '%s\n' '.text' '.globl miso_engine_v1_render_f32_planar' \
            'miso_engine_v1_render_f32_planar:' 'vstmxcsr -4(%rsp)' 'vldmxcsr -4(%rsp)' 'ret' \
            >"$scratch/inline.s"
        printf '%s\n' '.text' '.globl miso_engine_v1_render_f32_planar' \
            '.globl _ZN4lane7softfma10read_mxcsr17h0123456789abcdefE' \
            '_ZN4lane7softfma10read_mxcsr17h0123456789abcdefE:' 'vstmxcsr -4(%rsp)' 'ret' \
            'miso_engine_v1_render_f32_planar:' 'call _ZN4lane7softfma10read_mxcsr17h0123456789abcdefE' \
            'vldmxcsr -4(%rsp)' 'ret' >"$scratch/helper.s"
        printf '%s\n' '.text' '.globl other_entry' 'other_entry:' 'ret' >"$scratch/other.s"
        require_tools objdump
        cc -shared -nostdlib -o "$scratch/inline.so" "$scratch/inline.s"
        cc -shared -nostdlib -o "$scratch/other.so" "$scratch/other.s"
        cc -shared -nostdlib -o "$scratch/helper.so" "$scratch/helper.s"
        expect cdylib-inline pass '' bash "$self" --check cdylib "$scratch/inline.so"
        expect cdylib-helper-call fail 'calls a lane::softfma helper' \
            bash "$self" --check cdylib "$scratch/helper.so"
        expect cdylib-no-entry fail 'no miso_engine_v1_render_f32_planar' \
            bash "$self" --check cdylib "$scratch/other.so"
    else
        printf 'self-test note: not an x86_64 Linux host, cdylib cases skipped\n' >&2
    fi

    # Preflight refusals, before any cargo run.
    : >"$scratch/existing.s"
    mkdir -p "$scratch/out" && : >"$scratch/out/libcapi.a"
    expect preflight-crate-type fail 'unsupported crate type' \
        bash "$self" host rlib "$scratch/fresh"
    expect preflight-android-cdylib fail 'cdylib is built only for the host' \
        bash "$self" aarch64-linux-android cdylib "$scratch/fresh"
    expect preflight-overwrite fail 'refusing to overwrite' \
        bash "$self" host staticlib "$scratch/out"
    expect preflight-asm-overwrite fail 'refusing to overwrite' \
        bash "$self" --emit-asm "$scratch/existing.s" host staticlib "$scratch/fresh"
    expect preflight-unknown-target fail 'unknown target' \
        bash "$self" no-such-triple staticlib "$scratch/fresh"
    # The build runs cargo without the caller's flag and release-profile variables, and keeps the
    # others (#2079). A stub cargo records the environment it gets, then fails.
    mkdir -p "$scratch/bin"
    printf '%s\n' '#!/usr/bin/env bash' "env >'$scratch/cargo-env'" 'exit 1' >"$scratch/bin/cargo"
    chmod +x "$scratch/bin/cargo"
    expect build-flag-env fail 'cargo rustc failed for host staticlib' \
        env PATH="$scratch/bin:$PATH" RUSTFLAGS='-C opt-level=1' \
        CARGO_ENCODED_RUSTFLAGS='-Copt-level=1' CARGO_BUILD_RUSTFLAGS='-C opt-level=1' \
        CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS='-C opt-level=1' \
        CARGO_PROFILE_RELEASE_OPT_LEVEL=1 CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=cc \
        CARGO_TARGET_DIR="$scratch/target" bash "$self" host staticlib "$scratch/stub-out"
    local leaked kept
    leaked="$(grep -E '^(RUSTFLAGS|CARGO_ENCODED_RUSTFLAGS|CARGO_BUILD_RUSTFLAGS|CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS|CARGO_PROFILE_RELEASE_OPT_LEVEL)=' \
        "$scratch/cargo-env" || true)"
    [[ -z "$leaked" ]] || { printf 'self-test build-flag-env: cargo got %s\n' "$leaked" >&2; return 1; }
    kept="$(grep -cE '^(CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=cc|CARGO_TARGET_DIR='"$scratch"'/target)$' \
        "$scratch/cargo-env" || true)"
    ((kept == 2)) || { printf 'self-test build-flag-env: cargo lost CARGO_TARGET_DIR or a linker variable\n' >&2; return 1; }
    [[ ! -e "$scratch/fresh" ]] || { printf 'self-test: a refused preflight created its out-dir\n' >&2; return 1; }
    printf 'C ABI release build self-test: ok\n'
}

case "${1:-}" in
    --self-test) [[ $# -eq 1 ]] || usage; self_test; exit $? ;;
    --check) [[ $# -eq 3 ]] || usage; check_artifact "$2" "$3"; exit 0 ;;
esac

emit_asm=
if [[ "${1:-}" == --emit-asm ]]; then
    [[ $# -ge 2 ]] || usage
    emit_asm="$2"
    shift 2
fi
[[ $# -eq 3 ]] || usage
target="$1" crate_type="$2" out_dir="$3"

# --- preflight: every refusal happens before cargo runs --------------------------------------------
require_tools cargo rustc python3
host_triple="$(rustc -vV | sed -n 's/^host: //p')"
case "$crate_type" in
    staticlib) artifact_name=libcapi.a; require_tools ar ;;
    cdylib)
        [[ "$target" == host ]] ||
            fail "cdylib is built only for the host (the check-capi-abi.sh test form); a $target cdylib needs its target linker"
        case "$host_triple" in
            *-apple-*) artifact_name=libcapi.dylib ;;
            *-linux-*) artifact_name=libcapi.so ;;
            *) fail "unsupported host for a cdylib: $host_triple" ;;
        esac
        [[ "$host_triple" != x86_64-*-linux-* ]] || require_tools objdump
        ;;
    *) fail "unsupported crate type: $crate_type (staticlib or cdylib)" ;;
esac
target_args=()
if [[ "$target" != host ]]; then
    rustc --print target-list | rg -qxF -- "$target" || fail "unknown target: $target"
    target_args=(--target "$target")
fi
[[ ! -e "$out_dir/$artifact_name" ]] || fail "refusing to overwrite $out_dir/$artifact_name"
emit_args=()
if [[ -n "$emit_asm" ]]; then
    [[ ! -e "$emit_asm" ]] || fail "refusing to overwrite $emit_asm"
    [[ -d "$(dirname "$emit_asm")" ]] || fail "no directory for $emit_asm"
    emit_args=(-- --emit "link,asm=$emit_asm")
fi
mkdir -p "$out_dir"

# --- build ------------------------------------------------------------------------------------------
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
messages="$(mktemp)"
trap 'rm -f -- "$messages"' EXIT
# The library's flags and profile are the repository's, never the caller's shell (#2079, the stated
# limit of #2067). Cargo takes CARGO_ENCODED_RUSTFLAGS over RUSTFLAGS over the joined
# `target.<triple>`/`target.<cfg>` rustflags (CARGO_TARGET_<TRIPLE>_RUSTFLAGS) over build.rustflags
# (CARGO_BUILD_RUSTFLAGS), so any of them would replace or extend .cargo/config.toml's x86-64
# `+avx2,+fma`, and CARGO_PROFILE_RELEASE_* would override the manifest's release profile. They are
# unset next to the cargo command because they are part of the build. AArch64 needs no flags: NEON is in
# the aarch64-apple-ios and aarch64-linux-android baselines, and lane keys on it.
unset CARGO_ENCODED_RUSTFLAGS RUSTFLAGS CARGO_BUILD_RUSTFLAGS
for flag_var in ${!CARGO_TARGET_@} ${!CARGO_PROFILE_RELEASE_@}; do
    case "$flag_var" in
        CARGO_TARGET_*_RUSTFLAGS | CARGO_PROFILE_RELEASE_*) unset "$flag_var" ;;
    esac
done
cargo rustc --locked --release --manifest-path "$root/Cargo.toml" -p capi --lib \
    --crate-type "$crate_type" ${target_args[@]+"${target_args[@]}"} --message-format json-render-diagnostics \
    ${emit_args[@]+"${emit_args[@]}"} >"$messages" || fail "cargo rustc failed for $target $crate_type"
built="$(python3 -B - "$messages" "$artifact_name" <<'PY'
import json, sys
path, name = sys.argv[1:]
found = []
for line in open(path, encoding="utf-8"):
    line = line.strip()
    if not line.startswith("{"):
        continue
    message = json.loads(line)
    if message.get("reason") == "compiler-artifact" and message["target"]["name"] == "capi":
        found += [f for f in message.get("filenames", []) if f.rsplit("/", 1)[-1] == name]
if len(found) != 1:
    sys.exit(f"expected exactly one {name} from cargo, got {found}")
print(found[0])
PY
)" || fail "cargo reported no single $artifact_name artifact"
[[ -z "$emit_asm" || -s "$emit_asm" ]] || fail "no assembly written to $emit_asm"
cp -- "$built" "$out_dir/$artifact_name"
check_artifact "$crate_type" "$out_dir/$artifact_name"
