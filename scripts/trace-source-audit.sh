#!/usr/bin/env bash
# Usage: trace-source-audit.sh [path/to/audit]
set -euo pipefail

workspace_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$workspace_root"
binary="${1:-$workspace_root/target/release/audit}"
# The shipped profile is release, so this gate builds and runs the release binary rather than a
# debug `cargo run` (WP-3, #359); `--bin audit` stays explicit in case a second `[[bin]]` is ever
# added to the `audit` crate again.
if [[ ! -x "$binary" ]]; then
  cargo build --locked --release --manifest-path "$workspace_root/Cargo.toml" \
    -p audit --bin audit
fi
"$binary" source
