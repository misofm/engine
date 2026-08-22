#!/usr/bin/env bash
set -euo pipefail

if (($# != 2)); then
  echo "usage: $0 SEALED_INPUT NEW_EVIDENCE_JSON" >&2
  exit 2
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
seal=$1
output=$2
checksum="$output.sha256"
if [[ -e "$output" || -L "$output" || -e "$checksum" || -L "$checksum" ]]; then
  echo "refusing to overwrite browser evidence or checksum" >&2
  exit 2
fi
if [[ ! -f "$seal" || -L "$seal" ]]; then
  echo "sealed input must be an existing non-symlink file" >&2
  exit 2
fi
output_parent=$(dirname "$output")
if [[ ! -d "$output_parent" || -L "$output_parent" ]]; then
  echo "evidence parent must be an existing non-symlink directory" >&2
  exit 2
fi
if [[ -n "$(git -C "$repo_root" status --short)" ]]; then
  echo "browser correctness requires a clean committed candidate" >&2
  exit 2
fi

browser=${MISO_CHROMIUM_BINARY:-}
driver=${MISO_CHROMEDRIVER_BINARY:-}
if [[ -z "$browser" || ! -x "$browser" || -z "$driver" || ! -x "$driver" ]]; then
  echo "set MISO_CHROMIUM_BINARY and MISO_CHROMEDRIVER_BINARY to matched executables" >&2
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
  --seal-input "$seal" \
  --artifacts "$artifact_dir" \
  --output "$output" \
  --browser "$browser" \
  --driver "$driver"
