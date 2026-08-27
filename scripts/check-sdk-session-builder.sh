#!/usr/bin/env bash
# E3-E5 Session V1 builder corpus: build the validator once, then invoke that binary directly.
set -euo pipefail

if (($# > 1)) || { (($# == 1)) && [[ ${1:-} != "--self-test" ]]; }; then
  echo "usage: $0 [--self-test]" >&2
  exit 2
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
# Keep the explicit target under the ignored workspace target/ directory so the built binary is
# reachable after Cargo exits. The caller serializes this whole gate; nesting the serializer here
# would deadlock when scripts/sweep.sh is itself running under the shared CPU lock.
target_dir=$(mktemp -d "$repo_root/target/miso-sdk-session-validator.XXXXXX")
trap 'rm -rf -- "$target_dir"' EXIT

cd "$repo_root"
bash -n "$0"
env CARGO_TARGET_DIR="$target_dir" cargo build --locked -p miso-engine-session-validator
validator="$target_dir/debug/miso_engine_session_validator"
[[ -x $validator ]] || { echo "validator build produced no binary" >&2; exit 1; }
node sdk/test/session-corpus.mjs "$validator" "$@"
