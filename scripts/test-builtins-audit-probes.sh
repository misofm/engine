#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
binary="$workspace_dir/target/debug/audit"
cargo build --quiet --locked --manifest-path "$workspace_dir/Cargo.toml" \
  -p audit
for operation in allocation deallocation lock feature-detection log file-io network-io syscall panic-unwind; do
  if ("$binary" builtins --probe "$operation") >/dev/null 2>&1; then
    printf 'builtins realtime probe escaped: %s\n' "$operation" >&2
    exit 1
  fi
done
printf 'builtins realtime mutation probes: ok\n'
