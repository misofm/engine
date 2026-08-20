#!/usr/bin/env bash
# Run the deterministic issue-005 caller-buffer allocation audit. This records no timing data.
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"
cargo run --locked --release -p miso-engine-protocol-audit
