#!/usr/bin/env bash
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"
# #104 phase A: `miso-engine-audit` gained a second `[[bin]]` (the duration audit), so the
# bare `cargo run -p` became ambiguous and this gate could not run at all.
cargo run -p miso-engine-audit --bin miso_engine_audit -- source
