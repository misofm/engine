#!/usr/bin/env bash
# Phase 2 Node/Bun headless SDK E6-E10a over the real Wasm and native C-ABI/WAV oracle.
set -euo pipefail

if (($# > 1)) || { (($# == 1)) && [[ ${1:-} != "--self-test" ]]; }; then
  echo "usage: $0 [--self-test]" >&2
  exit 2
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
target_dir=$(mktemp -d "$repo_root/target/miso-sdk-headless.XXXXXX")
trap 'rm -rf -- "$target_dir"' EXIT
cd "$repo_root"

command -v node >/dev/null || { echo "headless SDK gate requires Node" >&2; exit 1; }
command -v bun >/dev/null || { echo "headless SDK gate requires Bun" >&2; exit 1; }

env CARGO_TARGET_DIR="$target_dir/cargo" cargo build --locked -p miso-engine-native-pcm-runner
native_runner="$target_dir/cargo/debug/miso-engine-native-pcm-runner"
[[ -x $native_runner ]] || { echo "native runner build produced no binary" >&2; exit 1; }

sdk_dist="$target_dir/sdk"
sdk/node_modules/.bin/tsc --project sdk/tsconfig.json --noEmit false --rootDir sdk/src --outDir "$sdk_dist"
node sdk/test/headless-evals.mjs "$native_runner" "$sdk_dist" "$@"
bun sdk/test/headless-evals.mjs "$native_runner" "$sdk_dist" "$@"

echo "SDK headless Node/Bun check passed"
