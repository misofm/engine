#!/usr/bin/env bash
# Execute the same BTLV golden assertions in scalar and simd128 Wasm test binaries.
# The temporary `main` export is a wasm-interp entry point only; the protocol crate exports no C ABI.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
readonly TARGET="wasm32-unknown-unknown"
readonly BINARY="miso_engine_protocol_wasm_golden"

cd "$repository_root"
command -v wasm-interp >/dev/null 2>&1 || {
    printf 'wasm-interp is required for issue-005 Wasm parity\n' >&2
    exit 1
}

run_variant() {
    local name="$1"
    local feature="$2"
    local target_directory="target/ci/issue005-wasm-$name"
    local artifact="$target_directory/$TARGET/release/$BINARY.wasm"

    CARGO_TARGET_DIR="$target_directory" \
        RUSTFLAGS="-C target-feature=$feature -C link-arg=--export=main" \
        cargo build --locked --release --target "$TARGET" \
            -p miso-engine-protocol --bin "$BINARY"
  wasm-objdump -x "$artifact" | rg -- '-> "main"'
    wasm-interp --run-all-exports "$artifact"
}

run_variant scalar -simd128
run_variant simd128 +simd128
printf 'issue-005 Wasm golden parity: ok (scalar + simd128)\n'
