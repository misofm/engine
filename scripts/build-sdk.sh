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

# This wrapper is the release-facing boundary. The lower-level web-artifact builder intentionally
# remains usable by dirty-tree developer and hermetic test flows.
if [[ -n "$(git -C "$repo_root" status --porcelain=v1 --untracked-files=normal)" ]]; then
  echo "SDK build requires a clean committed candidate" >&2
  exit 2
fi
source_revision=$(git -C "$repo_root" rev-parse --verify HEAD)
source_subject=$(git -C "$repo_root" show -s --format=%s HEAD)
if [[ ! "$source_revision" =~ ^[0-9a-f]{40}$ || -z "$source_subject" || "$source_subject" == *$'\n'* || "$source_subject" == *$'\r'* ]]; then
  echo "SDK build could not capture a valid source revision and subject" >&2
  exit 2
fi

"$repo_root/scripts/build-web-audioworklet.sh" "$output_dir"
python3 -I -B "$repo_root/scripts/write-web-provenance-v1.py" "$output_dir" \
  --source-revision "$source_revision" --source-subject "$source_subject"
python3 -I -B "$repo_root/scripts/check-web-provenance-v1.py" "$output_dir"
