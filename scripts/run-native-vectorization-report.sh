#!/usr/bin/env bash
# Issue #144 item 3. Build the certification chain from source and write the non-blocking report.
#
# Three builds, all from this tree, all under the shipped release profile:
#   1. tools/miso-engine-vectorization-probes for the host x86-64-v3 backend, emitting fresh LLVM IR
#      and a fresh object next to each other;
#   2. the same crate for AArch64, guarded on the cross target's standard library being installed;
#   3. the real release products -- the C ABI cdylib and the browser artifact's native twin.
#
# Only `lto` differs from the shipped profile, and only for the probe builds: `lto = "fat"` makes
# cargo emit LLVM bitcode rather than machine code for an intermediate rlib, and there is nothing to
# disassemble in bitcode. The kernels are `#[inline(always)]` generics instantiated inside the probe
# crate itself, so cross-crate LTO does not participate in their code generation; the products,
# which are what item 2 binds to, are built with the profile exactly as it ships. The report records
# the difference rather than hiding it.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "${1:-$script_directory/..}" && pwd)"
cd "$repository_root"

output_directory="${2:-$repository_root/target/ci/native-vectorization}"
report="$output_directory/report.json"
# The manifest holds only facts that do not move between two identical runs -- toolchain and tool
# versions, the effective profile and ISA pin -- so that `receipt.chain_sha256` is a stable identity
# for the whole chain. Cargo's own output goes to a separate log beside it: it carries wall-clock
# build durations, and hashing it would make every run's chain digest unique for no evidence.
manifest="$output_directory/build-manifest.txt"
build_log="$output_directory/build.log"
mkdir -p "$output_directory"

resolve_tool() {
    local preferred="$1"
    local fallback="$2"
    if command -v "$preferred" >/dev/null 2>&1; then
        command -v "$preferred"
    elif command -v "$fallback" >/dev/null 2>&1; then
        command -v "$fallback"
    else
        printf 'native vectorization report requires %s or %s\n' "$preferred" "$fallback" >&2
        exit 2
    fi
}
objdump="$(resolve_tool llvm-objdump objdump)"
nm="$(resolve_tool llvm-nm nm)"

probe_package="miso-engine-vectorization-probes"
probe_crate="miso_engine_vectorization_probes"

single() {
    local pattern="$1"
    local label="$2"
    local matches
    matches="$(find "$(dirname "$pattern")" -maxdepth 1 -type f -name "$(basename "$pattern")" 2>/dev/null | sort)"
    if [[ "$(printf '%s\n' "$matches" | sed '/^$/d' | wc -l)" -ne 1 ]]; then
        printf 'native vectorization report expected exactly one %s, found:\n%s\n' \
            "$label" "$matches" >&2
        exit 2
    fi
    printf '%s\n' "$matches"
}

# `--emit=llvm-ir,obj` replaces the rlib emit, so nothing is linked and no cross linker is needed.
build_probes() {
    local label="$1"
    local target="$2"
    local directory="$repository_root/target/ci/native-vectorization/probes-$label"
    rm -rf "$directory"
    if [[ -z "$target" ]]; then
        CARGO_PROFILE_RELEASE_LTO=false CARGO_TARGET_DIR="$directory" \
            cargo rustc --locked --release -p "$probe_package" --lib -- --emit=llvm-ir,obj \
            >>"$build_log" 2>&1
        printf '%s/release/deps\n' "$directory"
    else
        CARGO_PROFILE_RELEASE_LTO=false CARGO_TARGET_DIR="$directory" \
            cargo rustc --locked --release --target "$target" -p "$probe_package" --lib -- \
            --emit=llvm-ir,obj >>"$build_log" 2>&1
        printf '%s/%s/release/deps\n' "$directory" "$target"
    fi
}

{
    printf 'issue=144-item-3\n'
    printf 'probe_profile=release with lto=false (see the header of this script)\n'
    printf 'product_profile=release as shipped\n'
    rustc --version --verbose
    cargo --version
    "$objdump" --version | head -n 2
    "$nm" --version | head -n 2
    printf 'cargo_config=%s\n' "$(sed -n '1,20p' .cargo/config.toml | tr '\n' ' ')"
} >"$manifest"
: >"$build_log"

host_deps="$(build_probes host '')"
host_ir="$(single "$host_deps/$probe_crate-*.ll" 'host probe LLVM IR')"
host_object="$(single "$host_deps/$probe_crate-*.o" 'host probe object')"

arguments=(--backend "x86_64-avx2=$host_ir,$host_object")

# Issue #144 item 3, AArch64 coverage behind a target-availability guard. NEON is baseline on
# AArch64, so the standard library for any AArch64 target certifies the same four-lane backend; the
# Android target is the one the release matrix already installs. An absent target standard library
# is reported as an explicit skip with its reason, never as a pass.
aarch64_target="${MISO_ENGINE_VECTORIZATION_AARCH64_TARGET:-aarch64-linux-android}"
if rustc --print target-libdir --target "$aarch64_target" >/dev/null 2>&1 \
    && [[ -d "$(rustc --print target-libdir --target "$aarch64_target" 2>/dev/null)" ]]; then
    aarch64_deps="$(build_probes aarch64 "$aarch64_target")"
    aarch64_ir="$(single "$aarch64_deps/$probe_crate-*.ll" 'AArch64 probe LLVM IR')"
    aarch64_object="$(single "$aarch64_deps/$probe_crate-*.o" 'AArch64 probe object')"
    arguments+=(--backend "aarch64-neon=$aarch64_ir,$aarch64_object")
else
    arguments+=(--skip-backend \
        "aarch64-neon=the $aarch64_target standard library is not installed on this host, so no AArch64 object could be built; this backend is unproven in this run")
fi

# The shipped products, built with the release profile exactly as it ships.
products_directory="$repository_root/target/ci/native-vectorization/products"
CARGO_TARGET_DIR="$products_directory" cargo build --locked --release \
    -p miso-engine-capi -p miso-engine-host-web >>"$build_log" 2>&1
capi="$products_directory/release/libmiso_engine_capi.so"
web="$products_directory/release/libmiso_engine_host_web.so"
for artifact in "$capi" "$web"; do
    [[ -f "$artifact" ]] || {
        printf 'native vectorization report: missing shipped artifact %s\n' "$artifact" >&2
        exit 2
    }
done
arguments+=(--product "capi-cdylib=$capi" --product "web-native-twin=$web")

arguments+=(--receipt-input Cargo.lock --receipt-input rust-toolchain.toml \
    --receipt-input .cargo/config.toml \
    --receipt-input tools/miso-engine-vectorization-probes/src/lib.rs \
    --receipt-input "$manifest")

set +e
cargo run --locked --release -p miso-engine-audit --bin miso_engine_audit -- vectorization \
    --nm "$nm" --objdump "$objdump" "${arguments[@]}" >"$report"
status=$?
set -e

printf 'native vectorization report: %s (status %s)\n' "$report" "$status"
exit "$status"
