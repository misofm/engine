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

build_target=$(mktemp -d)
cleanup() {
  rm -rf -- "$build_target"
}
trap cleanup EXIT

(
  cd "$repo_root"
  CARGO_TARGET_DIR="$build_target" RUSTFLAGS='-C strip=debuginfo' \
    cargo build --locked --release --target wasm32-unknown-unknown \
      -p miso-engine-flac-decoder
)

artifact="$build_target/wasm32-unknown-unknown/release/miso_engine_flac_decoder.wasm"
observed=$(sha256sum "$artifact" | awk '{print $1}')
pin_file="$repo_root/hosts/miso-engine-flac-decoder/decoder-artifact.sha256"
expected=$(tr -d '\n' <"$pin_file")
loader="$repo_root/hosts/miso-engine-flac-decoder/miso-engine-flac-decoder.js"

if [[ "${MISO_FLAC_DECODER_REPIN:-0}" == 1 ]]; then
  printf '%s\n' "$observed"
  exit 0
fi
[[ "$observed" == "$expected" ]] || {
  printf 'FLAC decoder artifact pin mismatch: expected=%s observed=%s\n' \
    "$expected" "$observed" >&2
  exit 1
}
grep -qF "\"$expected\"" "$loader" || {
  echo "FLAC decoder loader pin differs from decoder-artifact.sha256" >&2
  exit 1
}

cp --update=none "$artifact" "$output_dir/miso-engine-flac-decoder.wasm"
cp --update=none "$loader" "$output_dir/miso-engine-flac-decoder.js"
cp --update=none \
  "$repo_root/hosts/miso-engine-flac-decoder/miso-engine-flac-decoder.d.ts" "$output_dir/"
cp --update=none "$pin_file" "$output_dir/"
