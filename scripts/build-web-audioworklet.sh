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

simd_target=$(mktemp -d)
cleanup() {
  rm -rf -- "$simd_target"
}
trap cleanup EXIT

# Owner decision W4-D1 (#83, 2026-08-24): the app's browser floor guarantees `simd128`, so exactly
# one artifact ships. The scalar worklet build and the dual-artifact selection in `host.js` are
# gone; `host.js` probes `simd128` at init and fails with a typed `miso.unsupported.v1` error when
# the probe fails -- the browser twin of D4's native boot attestation. The scalar *cargo check*
# stays in CI: `lane`'s wasm-scalar path is still gated, it just is not shipped.
#
# The browser artifact is the one place the workspace's `debug = 1` (issue 083 D12) is pure cost.
# It exists so a native profile or core dump names a kernel; a downloaded AudioWorklet module pays
# for the DWARF on every page load and cannot use it in production. Measured on this repository:
# 2,153,061 bytes before D12, 16,661,225 with `debug = 1`, 1,940,863 with the debug
# information stripped -- fat LTO alone makes the module *smaller* than it was, and the whole of
# the growth is DWARF.
#
# Stripped here, in the delivery script, and deliberately not in `[profile.release]`: the native
# artifacts keep their line tables. To build a debuggable browser module, override this with
# `MISO_ENGINE_WEB_STRIP=none`.
strip_flag="-C strip=${MISO_ENGINE_WEB_STRIP:-debuginfo}"

# The artifact is content-addressed, so every path rustc embeds in it must be a
# function of the SOURCE and nothing else. It is not by default: dependency
# sources live under CARGO_HOME, whose absolute path differs between a
# developer's machine and CI (`/root/.cargo` vs `/home/runner/.cargo`), and
# rustc bakes those paths into panic locations. That made the digest a function
# of WHERE cargo's registry sits, exactly as it did for the FLAC decoder artifact
# (see scripts/build-flac-decoder.sh and #300) before its own remap fix.
#
# Remapping both roots to fixed labels makes the digest reproducible anywhere.
# Verified: with these flags the digest is identical under CARGO_HOME=/root/.cargo
# and CARGO_HOME=/home/runner/.cargo, which previously produced two different ones,
# and also under a third repo-path/CARGO_HOME combination.
cargo_home=${CARGO_HOME:-$HOME/.cargo}
remap="--remap-path-prefix=$cargo_home=/cargo --remap-path-prefix=$repo_root=/repo"

(
  cd "$repo_root"
  CARGO_TARGET_DIR="$simd_target" RUSTFLAGS="-C target-feature=+simd128 $strip_flag $remap" \
    cargo build --locked --release --target wasm32-unknown-unknown -p host-web
)

artifact="$simd_target/wasm32-unknown-unknown/release/host_web.wasm"
observed=$(sha256sum "$artifact" | awk '{print $1}')
pin_file="$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256"
expected=$(tr -d '\n' <"$pin_file")

if [[ "${MISO_ENGINE_WEB_AUDIOWORKLET_REPIN:-0}" == 1 ]]; then
  printf '%s\n' "$observed"
  exit 0
fi
[[ "$observed" == "$expected" ]] || {
  printf 'AudioWorklet artifact pin mismatch: expected=%s observed=%s\n' \
    "$expected" "$observed" >&2
  exit 1
}

cp --update=none "$artifact" "$output_dir/miso-engine-v1-audio-worklet.simd128.wasm"
cp --update=none "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet.js" "$output_dir/"
cp --update=none "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.js" "$output_dir/"
cp --update=none "$repo_root/hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts" "$output_dir/"

# Issue #137 D4: the parameter metadata ships beside the module, so the app never introspects the
# Wasm for names, units, ranges, defaults or enumerations. The effect list is read from
# `launch_native_effect_registry()`, so an effect cannot be in the engine and missing here.
(
  cd "$repo_root"
  cargo run --locked --release -q -p parameter-metadata -- --write "$output_dir"
) >/dev/null
