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

# The artifact is content-addressed, so every path rustc embeds in it must be a
# function of the SOURCE and nothing else. It is not by default: dependency
# sources live under CARGO_HOME, whose absolute path differs between a
# developer's machine and CI (`/root/.cargo` vs `/home/runner/.cargo`), and
# rustc bakes those paths into panic locations. That made the digest a function
# of WHERE cargo's registry sits — the pin could only ever match on a machine
# whose CARGO_HOME matched whoever generated it, and CI's browser-qualification
# job had been red on exactly that difference, skipping every browser gate
# behind it.
#
# Remapping both roots to fixed labels makes the digest reproducible anywhere.
# Verified: with these flags the digest is identical under CARGO_HOME=/root/.cargo
# and CARGO_HOME=/home/runner/.cargo, which previously produced two different ones.
cargo_home=${CARGO_HOME:-$HOME/.cargo}
remap="--remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$repo_root=/repo"
(
  cd "$repo_root"
  CARGO_TARGET_DIR="$build_target" RUSTFLAGS="-C strip=debuginfo $remap" \
    cargo build --locked --release --target wasm32-unknown-unknown \
      -p miso-engine-flac-decoder
)

artifact="$build_target/wasm32-unknown-unknown/release/miso_engine_flac_decoder.wasm"
observed=$(sha256sum "$artifact" | awk '{print $1}')
pin_file="$repo_root/sidecars/flac-decoder/decoder-artifact.sha256"
expected=$(tr -d '\n' <"$pin_file")
loader="$repo_root/sidecars/flac-decoder/miso-engine-flac-decoder.js"

if [[ "${MISO_ENGINE_FLAC_DECODER_REPIN:-0}" == 1 ]]; then
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
  "$repo_root/sidecars/flac-decoder/miso-engine-flac-decoder.d.ts" "$output_dir/"
cp --update=none "$pin_file" "$output_dir/"
