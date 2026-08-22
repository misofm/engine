#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)
if command -v node >/dev/null; then
  exec node "$repo_root/scripts/test-web-audioworklet.mjs"
fi
if command -v bun >/dev/null; then
  exec bun "$repo_root/scripts/test-web-audioworklet.mjs"
fi
echo "Node.js-compatible runtime required" >&2
exit 2
