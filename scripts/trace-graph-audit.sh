#!/usr/bin/env bash
# Usage: trace-graph-audit.sh [path/to/audit] [blocks]
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
blocks="${2:-1000000}"
trace_root="$workspace_dir/target/issue6/strace"

# S3: this script never `cd`s, so a relative path already resolves against the caller's cwd; still
# normalize to absolute for consistency with the scripts in this family that do `cd`.
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
        printf 'trace-graph-audit.sh: explicit binary path must be an existing executable file: %s\n' \
            "$binary" >&2
        exit 1
    }
else
    binary="$workspace_dir/target/release/audit"
fi

if [[ ! -x "$binary" ]]; then
  cargo build --quiet --locked --release --manifest-path "$workspace_dir/Cargo.toml" \
    -p audit
fi
command -v strace >/dev/null 2>&1 || {
  printf 'strace is required for the graph realtime syscall gate\n' >&2
  exit 1
}
mkdir -p "$trace_root"
trace_prefix="$trace_root/trace"
find "$trace_root" -maxdepth 1 -type f -name 'trace.*' -delete

strace -ff -qq -s 200 -o "$trace_prefix" "$binary" graph --blocks "$blocks" \
  > "$trace_root/audit.json"

marker_file=""
while IFS= read -r candidate; do
  if rg -q 'MISO_ENGINE_GRAPH_RT_BEGIN' "$candidate" && rg -q 'MISO_ENGINE_GRAPH_RT_END' "$candidate"; then
    [[ -z "$marker_file" ]] || {
      printf 'multiple trace threads contain both graph markers\n' >&2
      exit 1
    }
    marker_file="$candidate"
  fi
done < <(find "$trace_root" -maxdepth 1 -type f -name 'trace.*' | sort)
[[ -n "$marker_file" ]] || {
  printf 'no trace thread contains both graph markers\n' >&2
  exit 1
}

unexpected=$({
  awk '
    /MISO_ENGINE_GRAPH_RT_BEGIN/ { inside = 1; next }
    /MISO_ENGINE_GRAPH_RT_END/ { inside = 0; found_end = 1; next }
    inside { print }
    END { if (!found_end) exit 2 }
  ' "$marker_file"
} || true)
[[ -z "$unexpected" ]] || {
  printf 'unexpected graph render syscall(s):\n%s\n' "$unexpected" >&2
  exit 1
}

jq -e --argjson blocks "$blocks" '
  .kind == "graph_realtime_audit" and
  .blocks == $blocks and
  .quantum_frames == 1 and
  .swaps_accepted == 2 and
  .swaps_deferred == 1 and
  .displaced_plans_destroyed_off_render == 2 and
  .total_violations == 0
' "$trace_root/audit.json" >/dev/null
printf 'graph realtime syscall trace: PASS (%s blocks)\n' "$blocks"
