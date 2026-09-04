#!/usr/bin/env bash
# Run the deterministic issue-005 caller-buffer allocation audit. This records no timing data.
# Usage: run-protocol-allocation-audit.sh [path/to/audit]
set -euo pipefail

workspace_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$workspace_root"
binary="${1:-$workspace_root/target/release/audit}"
if [[ ! -x "$binary" ]]; then
  cargo build --locked --release --manifest-path "$workspace_root/Cargo.toml" \
    -p audit --bin audit
fi
"$binary" protocol
