#!/usr/bin/env bash
# Build and run the bounded native C11 descriptor-inspection ABI smoke.
set -euo pipefail

workspace_root="$(cd "$(dirname "$0")/.." && pwd)"
target_directory="${CARGO_TARGET_DIR:-$workspace_root/target}"
scratch_directory="$(mktemp -d)"
trap 'rm -rf -- "$scratch_directory"' EXIT

cargo build --locked --manifest-path "$workspace_root/Cargo.toml" \
    -p miso-engine-effect-package --features c-abi --lib

case "$(uname -s)" in
    Darwin)
        library_file="$target_directory/debug/libmiso_engine_effect_package.dylib"
        dynamic_symbols=(nm -gU)
        ;;
    Linux)
        library_file="$target_directory/debug/libmiso_engine_effect_package.so"
        dynamic_symbols=(nm -D --defined-only)
        ;;
    *)
        printf 'effect descriptor C smoke failure: unsupported native host\n' >&2
        exit 1
        ;;
esac

[[ -f "$library_file" ]] || {
    printf 'effect descriptor C smoke failure: missing library %s\n' "$library_file" >&2
    exit 1
}

exported_symbols="$({
    "${dynamic_symbols[@]}" "$library_file" |
        awk '{print $NF}' |
        LC_ALL=C sort -u |
        sed -n '/^miso_engine_/p'
} || true)"
[[ "$exported_symbols" == "miso_engine_effect_descriptor_v1_inspect" ]] || {
    printf 'effect descriptor C smoke failure: unexpected descriptor symbols\n' >&2
    printf '%s\n' "$exported_symbols" >&2
    exit 1
}

cc -std=c11 -pedantic -Wall -Wextra -Werror \
    -I"$workspace_root/crates/miso-engine-effect-package/include" \
    "$workspace_root/crates/miso-engine-effect-package/tests/c/descriptor_smoke.c" \
    -L"$(dirname "$library_file")" -lmiso_engine_effect_package \
    -Wl,-rpath,"$(dirname "$library_file")" \
    -o "$scratch_directory/descriptor-smoke"

"$scratch_directory/descriptor-smoke" \
    "$workspace_root/fixtures/effect-descriptor/v1/comprehensive-a.wire.hex"
