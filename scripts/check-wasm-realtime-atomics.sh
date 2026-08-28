#!/usr/bin/env bash
# Build and inspect the browser-local queue monomorphization; it must require no atomic opcode.
set -euo pipefail

target_directory="${1:-target/ci/wasm-realtime-local}"
command -v wasm-objdump >/dev/null 2>&1 || {
    printf 'wasm-objdump is required for the Wasm realtime atomic inspection\n' >&2
    exit 1
}

cfg="$(rustc --print cfg --target wasm32-unknown-unknown)"
printf '%s\n' "$cfg" | rg -q '^target_has_atomic="ptr"$' || {
    printf 'wasm target does not advertise pointer-width atomic support\n' >&2
    exit 1
}
if printf '%s\n' "$cfg" | rg -q '^target_feature="atomics"$'; then
    printf 'browser-local fallback artifact unexpectedly enables Wasm atomics\n' >&2
    exit 1
fi

CARGO_TARGET_DIR="$target_directory" RUSTFLAGS='-C target-feature=-simd128' \
    cargo build --locked --release --target wasm32-unknown-unknown \
    -p miso-engine-core -p miso-engine-source -p miso-engine-target-smoke

scratch="$(mktemp -d)"
trap 'rm -rf -- "$scratch"' EXIT
for archive in \
    "$target_directory/wasm32-unknown-unknown/release/deps/"libmiso_engine_core-*.rlib \
    "$target_directory/wasm32-unknown-unknown/release/deps/"libmiso_engine_source-*.rlib \
    "$target_directory/wasm32-unknown-unknown/release/deps/"libmiso_engine_target_smoke-*.rlib; do
    [[ -f "$archive" ]] || continue
    archive="$(realpath "$archive")"
    archive_directory="$scratch/$(basename "$archive")"
    mkdir -p "$archive_directory"
    (
        cd "$archive_directory"
        ar x "$archive"
    )
done

# Issue #143 D4/R8: the conflating observation cell is a browser-local realtime primitive, so it
# is named here rather than left to the glob. A refactor that moves it out of the inspected crate
# fails this check instead of silently losing its coverage.
if ! find "$scratch" -type f -name '*.o' -print0 |
    xargs -0 -r rg -l --binary 'observe' >/dev/null 2>&1 &&
    ! rg -q 'ObservationSlot' crates/miso-engine-core/src/realtime/observe.rs; then
    printf 'the observation transport is not in the inspected browser-local set\n' >&2
    exit 1
fi

object_count=0
while IFS= read -r object; do
    object_count=$((object_count + 1))
    if wasm-objdump -d "$object" | rg -n 'atomic\.'; then
        printf 'browser-local fallback contains an atomic opcode: %s\n' "$object" >&2
        exit 1
    fi
done < <(find "$scratch" -type f -name '*.o' | sort)

[[ "$object_count" -gt 0 ]] || {
    printf 'no Wasm objects were available for atomic inspection\n' >&2
    exit 1
}

printf 'wasm realtime atomics: ok (%s objects, local fallback)\n' "$object_count"
