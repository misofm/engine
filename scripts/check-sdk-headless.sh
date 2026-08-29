#!/usr/bin/env bash
# Issue #243: the SDK's headless evals, against the shipped module.
#
# With no argument the gate builds the artifact it checks, matching
# `scripts/check-web-audioworklet.sh`; CI passes a directory it already built.
#
# The tests run under Node's native type stripping, so there is no build step and no `node_modules`
# on this path: `sdk/src/**/*.ts` is imported directly. That is deliberate. A gate that needed an
# `npm install` would need the network, and a gate that needed the network could not be a sweep row.
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)

command -v node >/dev/null || { echo "node is required" >&2; exit 2; }

if (($# == 0)); then
  self_built_artifacts=$(mktemp -d)
  trap 'rm -rf -- "$self_built_artifacts"' EXIT
  bash "$repo_root/scripts/build-web-audioworklet.sh" "$self_built_artifacts" >&2
  set -- "$self_built_artifacts"
fi

if (($# != 1)); then
  echo "usage: $0 [ARTIFACT_DIRECTORY]" >&2
  exit 2
fi

artifact_dir=$1
[[ -d "$artifact_dir" && ! -L "$artifact_dir" ]] || {
  echo "artifact directory must be a non-symlink directory" >&2
  exit 2
}
[[ -f "$artifact_dir/miso-engine-v2-audio-worklet.simd128.wasm" ]] || {
  echo "artifact directory has no wasm module" >&2
  exit 2
}

cd "$repo_root/sdk"
# The glob names the eval suites explicitly. Passing the directory would also run support.mjs,
# which carries fixtures rather than tests, and Node reports a bare directory argument as a
# failing test of its own.
MISO_ENGINE_SDK_ARTIFACTS="$artifact_dir" node --test 'test/*-evals.mjs'
