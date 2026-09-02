#!/usr/bin/env bash
# Issue #320: build or qualify the publishable @misofm/engine package.
set -euo pipefail

if (($# < 1 || $# > 2)) || [[ $1 != build && $1 != check ]]; then
  echo "usage: $0 build|check [ARTIFACT_DIRECTORY]" >&2
  exit 2
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
sdk_root="$repo_root/sdk"
mode=$1
artifact_dir=${2:-}
owned_artifacts=""
pack_dir=""
unpack_dir=""
cleanup() {
  [[ -z "$owned_artifacts" ]] || rm -rf -- "$owned_artifacts"
  [[ -z "$pack_dir" ]] || rm -rf -- "$pack_dir"
  [[ -z "$unpack_dir" ]] || rm -rf -- "$unpack_dir"
}
trap cleanup EXIT

[[ -x "$sdk_root/node_modules/.bin/tsc" ]] || {
  echo "sdk/node_modules is missing; run 'npm ci' in sdk/ first" >&2
  exit 2
}

if [[ -z "$artifact_dir" ]]; then
  owned_artifacts=$(mktemp -d)
  artifact_dir=$owned_artifacts
  bash "$repo_root/scripts/build-web-audioworklet.sh" "$artifact_dir"
fi
[[ -d "$artifact_dir" && ! -L "$artifact_dir" ]] || {
  echo "artifact directory must be a non-symlink directory" >&2
  exit 2
}

bash "$repo_root/scripts/check-sdk-generated.sh"
rm -rf -- "$sdk_root/dist"
"$sdk_root/node_modules/.bin/tsc" --project "$sdk_root/tsconfig.build.json"
node "$sdk_root/codegen/stage-package.mjs" "$artifact_dir"

if [[ $mode == build ]]; then
  echo "SDK package tree prepared at $sdk_root/dist"
  exit 0
fi

pack_dir=$(mktemp -d)
unpack_dir=$(mktemp -d)
(cd "$sdk_root" && npm_config_cache="$pack_dir/npm-cache" \
  npm pack --ignore-scripts --pack-destination "$pack_dir" >/dev/null)
archives=("$pack_dir"/*.tgz)
[[ ${#archives[@]} == 1 && -f ${archives[0]} ]] || {
  echo "npm pack did not produce exactly one tarball" >&2
  exit 1
}
tar -xzf "${archives[0]}" -C "$unpack_dir"
node "$sdk_root/test/package-tarball-smoke.mjs" "$unpack_dir/package"
echo "SDK publishable-tarball gate passed"
