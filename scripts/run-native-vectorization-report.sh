#!/usr/bin/env bash
# Issue #144 item 3: build release-profile probe instantiations of the production lane kernels,
# disassemble their named bodies, and write the non-blocking certification report.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
output_directory="${1:-$repository_root/target/ci/native-vectorization}"
target_directory="${CARGO_TARGET_DIR:-$repository_root/target}"
report="$output_directory/report.json"

mkdir -p "$output_directory"
cd "$repository_root"

if command -v llvm-objdump >/dev/null 2>&1; then
    objdump="$(command -v llvm-objdump)"
elif command -v objdump >/dev/null 2>&1; then
    objdump="$(command -v objdump)"
else
    printf 'native vectorization report requires llvm-objdump or objdump\n' >&2
    exit 2
fi

CARGO_TARGET_DIR="$target_directory" cargo build --locked --release \
    -p miso-engine-audit --bin miso_engine_audit
binary="$target_directory/release/miso_engine_audit"

"$binary" vectorization \
    --artifact "$binary" \
    --allowlist tools/miso-engine-audit/vectorization-allowlist.tsv \
    --objdump "$objdump" | tee "$report"

printf 'native vectorization report: %s\n' "$report"
