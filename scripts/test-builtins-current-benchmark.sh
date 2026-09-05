#!/usr/bin/env bash
set -euo pipefail
[[ $# -eq 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
command -v jq >/dev/null
for script in preflight-builtins-current-benchmark.sh run-builtins-current-benchmark.sh; do
  bash -n "$(dirname "${BASH_SOURCE[0]}")/$script"
done
printf 'current benchmark fake-only lifecycle checks require an isolated scratch harness\n'
