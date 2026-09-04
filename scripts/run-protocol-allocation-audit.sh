#!/usr/bin/env bash
# Run the deterministic issue-005 caller-buffer allocation audit. This records no timing data.
# Usage: run-protocol-allocation-audit.sh [path/to/audit]
set -euo pipefail

workspace_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# S3: resolve a relative binary path against the caller's cwd BEFORE `cd "$workspace_root"` below.
binary="${1:-}"
if [[ -n "$binary" ]]; then
    case "$binary" in
        /*) : ;;
        *) binary="$(realpath -m -- "$binary")" ;;
    esac
    # S1/S2: an explicit path must be an existing executable file, never a directory or a missing
    # path -- and being explicit-but-missing is a hard error; only the defaulted path may trigger
    # a build.
    [[ -f "$binary" && -x "$binary" ]] || {
        printf 'run-protocol-allocation-audit.sh: explicit binary path must be an existing executable file: %s\n' \
            "$binary" >&2
        exit 1
    }
else
    binary="$workspace_root/target/release/audit"
fi

cd "$workspace_root"
if [[ ! -x "$binary" ]]; then
  cargo build --locked --release --manifest-path "$workspace_root/Cargo.toml" \
    -p audit --bin audit
fi

# B1: assert on the binary's own success line instead of trusting a bare exit 0, so a wrong or
# stale binary (e.g. `/bin/true`) cannot pass vacuously.
output="$("$binary" protocol)"
printf '%s\n' "$output"
grep -qF \
  'protocol caller-buffer audit: ok (command, success, non-OK, event, reliable/meter/counter egress, 64 edits, 10,000 automation records in 40 batches; diagnostic egress is control-plane typed storage)' \
  <<<"$output"
