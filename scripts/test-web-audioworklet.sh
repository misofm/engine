#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
if command -v node >/dev/null; then
  node "$repo_root/scripts/test-web-audioworklet.mjs"
elif command -v bun >/dev/null; then
  bun "$repo_root/scripts/test-web-audioworklet.mjs"
else
  echo "Node.js-compatible runtime required" >&2
  exit 2
fi

worklet="$repo_root/hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js"
"$repo_root/scripts/check-web-audioworklet.sh" "--source-policy=$worklet"
"$repo_root/scripts/check-web-audioworklet.sh" --self-test-opcodes
mutation_dir=$(mktemp -d)
cleanup() {
  rm -rf -- "$mutation_dir"
}
trap cleanup EXIT
for mutation in \
  'new Array(1);' \
  'this.port.postMessage({});' \
  'BigInt(1);' \
  'this.exports.memory.grow(1);'
do
  mutated="$mutation_dir/worklet.js"
  awk -v mutation="$mutation" '
    { print }
    /silence\(outputs\) \{/ { print "    " mutation }
  ' "$worklet" >"$mutated"
  if "$repo_root/scripts/check-web-audioworklet.sh" "--source-policy=$mutated" >/dev/null 2>&1; then
    echo "transitive process-helper mutation escaped policy: $mutation" >&2
    exit 1
  fi
done
echo "web AudioWorklet transitive process-policy mutations passed"
