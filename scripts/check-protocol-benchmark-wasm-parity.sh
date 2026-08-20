#!/usr/bin/env bash
# Build and execute only the corpus checksum assertion; this is not a timing benchmark.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
readonly target="wasm32-unknown-unknown"
readonly binary="miso_engine_protocol_bench"

cd "$repository_root"
command -v wasm-interp >/dev/null 2>&1 || { printf 'wasm-interp is required for protocol benchmark Wasm parity\n' >&2; exit 1; }

run_variant() {
    local name="$1"
    local feature="$2"
    local target_directory="target/ci/issue005-protocol-bench-wasm-$name"
    local artifact="$target_directory/$target/release/$binary.wasm"
    CARGO_TARGET_DIR="$target_directory" RUSTFLAGS="-C target-feature=$feature -C link-arg=--export=main" \
        cargo build --locked --release --target "$target" -p miso-engine-protocol-bench --bin "$binary"
    wasm-objdump -x "$artifact" | rg -- '-> "main"'
    wasm-interp --run-all-exports "$artifact"
}

run_variant scalar -simd128
run_variant simd128 +simd128
printf 'issue-005 protocol benchmark Wasm corpus parity: ok (scalar + simd128; no timings)\n'
