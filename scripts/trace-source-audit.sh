#!/usr/bin/env bash
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"
cargo run -p miso-engine-source-audit
