#!/usr/bin/env bash
# Check the Phase 1 generated SDK data surface without building a package or Wasm artifact.
set -euo pipefail

if (($# > 1)) || { (($# == 1)) && [[ ${1:-} != "--self-test" ]]; }; then
  echo "usage: $0 [--self-test]" >&2
  exit 2
fi

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
cd "$repo_root"

run() {
  node sdk/codegen/generate.mjs --check
  node sdk/test/typecheck.mjs
  node sdk/test/generated-parity.mjs
  python3 -I -B scripts/check-parameter-metadata-v1.py \
    sdk/assets/miso-engine-v2-parameter-metadata.json
  python3 -I -B scripts/check-command-reason-vocabulary.py
}

if (($# == 1)); then
  bash -n "$0"
  run
  node sdk/test/typecheck.mjs --self-test
  node sdk/test/generated-parity.mjs --self-test
  python3 -I -B scripts/check-command-reason-vocabulary.py --self-test
  echo "SDK generated-data self-test passed"
else
  run
  echo "SDK generated-data check passed"
fi
