#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
binary="$workspace_dir/target/debug/miso_engine_builtins_audit"
cargo build --quiet --locked --manifest-path "$workspace_dir/Cargo.toml" \
  -p miso-engine-builtins-audit
for operation in allocation deallocation lock log file-io network-io syscall; do
  if ("$binary" --probe "$operation") >/dev/null 2>&1; then
    printf 'builtins realtime probe escaped: %s\n' "$operation" >&2
    exit 1
  fi
done
printf 'builtins realtime mutation probes: ok\n'
