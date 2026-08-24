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
# stays in CI: `miso-engine-lane`'s wasm-scalar path is still gated, it just is not shipped.
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
# `MISO_WEB_STRIP=none`.
strip_flag="-C strip=${MISO_WEB_STRIP:-debuginfo}"

(
  cd "$repo_root"
  CARGO_TARGET_DIR="$simd_target" RUSTFLAGS="-C target-feature=+simd128 $strip_flag" \
    cargo build --locked --release --target wasm32-unknown-unknown -p miso-engine-host-web
)

cp --update=none "$simd_target/wasm32-unknown-unknown/release/miso_engine_host_web.wasm" \
  "$output_dir/miso-engine-v2-audio-worklet.simd128.wasm"
cp --update=none "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js" "$output_dir/"
cp --update=none "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js" "$output_dir/"
cp --update=none "$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts" "$output_dir/"
