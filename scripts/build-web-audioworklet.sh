#!/usr/bin/env bash
set -euo pipefail

if (($# != 1)); then
  echo "usage: $0 EMPTY_OUTPUT_DIRECTORY" >&2
  exit 2
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
output_dir=$1
if [[ ! -d "$output_dir" || -L "$output_dir" ]]; then
  echo "output must be an existing non-symlink directory" >&2
  exit 2
fi
if [[ -n "$(find "$output_dir" -mindepth 1 -maxdepth 1 -print -quit)" ]]; then
  echo "output directory must be empty; refusing overwrite" >&2
  exit 2
fi

scalar_target=$(mktemp -d)
simd_target=$(mktemp -d)
cleanup() {
  rm -rf -- "$scalar_target" "$simd_target"
}
trap cleanup EXIT

(
  cd "$repo_root"
  CARGO_TARGET_DIR="$scalar_target" RUSTFLAGS="-C target-feature=-simd128" \
    cargo build --locked --release --target wasm32-unknown-unknown -p miso-engine-host-web
  CARGO_TARGET_DIR="$simd_target" RUSTFLAGS="-C target-feature=+simd128" \
    cargo build --locked --release --target wasm32-unknown-unknown -p miso-engine-host-web
)

cp --update=none "$scalar_target/wasm32-unknown-unknown/release/miso_engine_host_web.wasm" \
  "$output_dir/miso-engine-v2-audio-worklet.scalar.wasm"
cp --update=none "$simd_target/wasm32-unknown-unknown/release/miso_engine_host_web.wasm" \
  "$output_dir/miso-engine-v2-audio-worklet.simd128.wasm"
cp --update=none "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js" "$output_dir/"
cp --update=none "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js" "$output_dir/"
cp --update=none "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts" "$output_dir/"
