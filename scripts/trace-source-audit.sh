#!/usr/bin/env bash
# Usage: trace-source-audit.sh [path/to/audit]
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
        printf 'trace-source-audit.sh: explicit binary path must be an existing executable file: %s\n' \
            "$binary" >&2
        exit 1
    }
else
    binary="$workspace_root/target/release/audit"
fi

cd "$workspace_root"
# The shipped profile is release, so this gate builds and runs the release binary rather than a
# debug `cargo run` (WP-3, #359); `--bin audit` stays explicit in case a second `[[bin]]` is ever
# added to the `audit` crate again.
if [[ ! -x "$binary" ]]; then
  cargo build --locked --release --manifest-path "$workspace_root/Cargo.toml" \
    -p audit --bin audit
fi

# B1: assert on the binary's own output record instead of trusting a bare exit 0, so a wrong or
# stale binary (e.g. `/bin/true`) cannot pass vacuously.
output="$("$binary" source)"
printf '%s\n' "$output"
jq -e '
  .schema_version == 1 and
  .kind == "issue010_source_realtime_audit" and
  .blocks == 100000 and
  .quantum_frames == 128 and
  .native_worker_hold_release == true and
  .total_violations == 0
' <<<"$output" >/dev/null
