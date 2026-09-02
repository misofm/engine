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

capture_physical_directory() {
  CDPATH='' cd -P -- "$1" && printf '%sx' "$PWD"
}

script_directory=${BASH_SOURCE[0]%/*}
if [[ "$script_directory" == "${BASH_SOURCE[0]}" ]]; then
  script_directory=.
fi
if ! repo_root_with_sentinel=$(capture_physical_directory "$script_directory/.."); then
  echo "repository root cannot be resolved" >&2
  exit 2
fi
repo_root=${repo_root_with_sentinel%x}

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
artifact_link_probe=$artifact_dir
while :; do
  case "$artifact_link_probe" in
    /)
      break
      ;;
    */)
      artifact_link_probe=${artifact_link_probe%/}
      ;;
    */.)
      artifact_link_probe=${artifact_link_probe%/.}
      ;;
    *)
      break
      ;;
  esac
done
[[ -d "$artifact_dir" && ! -L "$artifact_link_probe" ]] || {
  echo "artifact directory must be a non-symlink directory" >&2
  exit 2
}
# Command substitution strips every trailing newline, which is legal pathname data. Append one
# known non-newline byte to the physical directory, then remove exactly that byte after capture.
# POSIX paths cannot contain NUL, but every other byte Bash can carry remains untouched.
if ! artifact_dir_with_sentinel=$(capture_physical_directory "$artifact_dir"); then
  echo "artifact directory cannot be resolved" >&2
  exit 2
fi
artifact_dir=${artifact_dir_with_sentinel%x}
[[ -f "$artifact_dir/miso-engine-v1-audio-worklet.simd128.wasm" ]] || {
  echo "artifact directory has no wasm module" >&2
  exit 2
}

cd "$repo_root/sdk"
# The glob names the eval suites explicitly. Passing the directory would also run support.mjs,
# which carries fixtures rather than tests, and Node reports a bare directory argument as a
# failing test of its own.
MISO_ENGINE_SDK_ARTIFACTS="$artifact_dir" node --test 'test/*-evals.mjs'
