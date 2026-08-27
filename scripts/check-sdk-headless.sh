#!/usr/bin/env bash
# Phase 2 Node/Bun headless SDK E6-E10a over the real Wasm and native C-ABI/WAV oracle.
set -euo pipefail

self_test=()
run_bun=false
for argument in "$@"; do
  case "$argument" in
    --self-test)
      ((${#self_test[@]} == 0)) || { echo "duplicate --self-test" >&2; exit 2; }
      self_test=(--self-test)
      ;;
    --bun)
      [[ $run_bun == false ]] || { echo "duplicate --bun" >&2; exit 2; }
      run_bun=true
      ;;
    *)
      echo "usage: $0 [--self-test] [--bun]" >&2
      exit 2
      ;;
  esac
done

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
target_dir=$(mktemp -d "$repo_root/target/miso-sdk-headless.XXXXXX")
trap 'rm -rf -- "$target_dir"' EXIT
cd "$repo_root"

command -v node >/dev/null || { echo "headless SDK gate requires Node" >&2; exit 1; }
if [[ $run_bun == true ]]; then
  command -v bun >/dev/null || { echo "headless SDK Bun eval requires Bun" >&2; exit 1; }
fi

env CARGO_TARGET_DIR="$target_dir/cargo" cargo build --locked -p miso-engine-native-pcm-runner
native_runner="$target_dir/cargo/debug/miso-engine-native-pcm-runner"
[[ -x $native_runner ]] || { echo "native runner build produced no binary" >&2; exit 1; }

sdk_dist="$target_dir/sdk"
sdk/node_modules/.bin/tsc --project sdk/tsconfig.json --noEmit false --rootDir sdk/src --outDir "$sdk_dist"
node sdk/test/headless-evals.mjs "$native_runner" "$sdk_dist" "${self_test[@]}"
if [[ $run_bun == true ]]; then
  bun sdk/test/headless-evals.mjs "$native_runner" "$sdk_dist" "${self_test[@]}"
fi

runtime_label=Node
[[ $run_bun == false ]] || runtime_label=Node/Bun
echo "SDK headless $runtime_label check passed"
