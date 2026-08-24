#!/usr/bin/env bash
set -euo pipefail

if (($# != 1)); then
  echo "usage: $0 NEW_SEAL_JSON" >&2
  exit 2
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
seal=$1
if [[ -e "$seal" || -L "$seal" ]]; then
  echo "refusing to overwrite browser seal" >&2
  exit 2
fi
seal_parent=$(dirname "$seal")
if [[ ! -d "$seal_parent" || -L "$seal_parent" ]]; then
  echo "seal parent must be an existing non-symlink directory" >&2
  exit 2
fi
if [[ -n "$(git -C "$repo_root" status --short)" ]]; then
  echo "browser seal requires a clean committed candidate" >&2
  exit 2
fi

browser=${MISO_ENGINE_CHROMIUM_BINARY:-}
driver=${MISO_ENGINE_CHROMEDRIVER_BINARY:-}
if [[ -z "$browser" || ! -x "$browser" || -z "$driver" || ! -x "$driver" ]]; then
  echo "set MISO_ENGINE_CHROMIUM_BINARY and MISO_ENGINE_CHROMEDRIVER_BINARY to matched executables" >&2
  exit 2
fi

artifact_dir=$(mktemp -d)
cleanup() {
  rm -rf -- "$artifact_dir"
}
trap cleanup EXIT
"$repo_root/scripts/build-web-audioworklet.sh" "$artifact_dir"
"$repo_root/scripts/check-web-audioworklet.sh" "$artifact_dir"
python3 -B "$repo_root/scripts/web-audioworklet-browser-correctness.py" \
  --check \
  --artifacts "$artifact_dir"
python3 -B "$repo_root/scripts/web-audioworklet-browser-correctness.py" \
  --seal \
  --artifacts "$artifact_dir" \
  --output "$seal" \
  --browser "$browser" \
  --driver "$driver"
