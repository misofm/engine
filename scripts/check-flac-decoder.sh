#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
[[ "$#" == 0 ]] || { echo "usage: $0" >&2; exit 2; }
artifacts=$(mktemp -d)
cleanup() {
  rm -rf -- "$artifacts"
}
trap cleanup EXIT

bash "$repo_root/scripts/build-flac-decoder.sh" "$artifacts"
node "$repo_root/scripts/check-flac-decoder.mjs" "$artifacts"

if rg -n '\.decodeAudioData[[:space:]]*\(' \
  "$repo_root/crates" "$repo_root/hosts" "$repo_root/tools" "$repo_root/sidecars"; then
  echo "decodeAudioData call site is forbidden in the engine delivery pipeline" >&2
  exit 1
fi
echo "FLAC delivery decodeAudioData gate passed"
