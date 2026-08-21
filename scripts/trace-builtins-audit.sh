#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
binary="$workspace_dir/target/release/miso_engine_builtins_audit"
blocks="${1:-1000000}"
trace_root="$workspace_dir/target/issue7/strace"

cargo build --quiet --locked --release --manifest-path "$workspace_dir/Cargo.toml" \
  -p miso-engine-builtins-audit
command -v strace >/dev/null 2>&1 || {
  printf 'strace is required for the builtins realtime syscall gate\n' >&2
  exit 1
}
mkdir -p "$trace_root"
trace_prefix="$trace_root/trace"
find "$trace_root" -maxdepth 1 -type f -name 'trace.*' -delete
strace -ff -qq -o "$trace_prefix" "$binary" --blocks "$blocks" >"$trace_root/audit.json"

marker_file=""
while IFS= read -r candidate; do
  if rg -q 'MISO_BUILTINS_RT_BEGIN' "$candidate" && rg -q 'MISO_BUILTINS_RT_END' "$candidate"; then
    [[ -z "$marker_file" ]] || { printf 'multiple traced builtins render threads\n' >&2; exit 1; }
    marker_file="$candidate"
  fi
done < <(find "$trace_root" -maxdepth 1 -type f -name 'trace.*' | sort)
[[ -n "$marker_file" ]] || { printf 'missing builtins render markers\n' >&2; exit 1; }
unexpected=$(awk '
  /MISO_BUILTINS_RT_BEGIN/ { inside = 1; next }
  /MISO_BUILTINS_RT_END/ { inside = 0; found_end = 1; next }
  inside { print }
  END { if (!found_end) exit 2 }
' "$marker_file" || true)
[[ -z "$unexpected" ]] || { printf 'unexpected builtins render syscall(s):\n%s\n' "$unexpected" >&2; exit 1; }
jq -e --argjson blocks "$blocks" '
  .kind == "builtins_realtime_audit" and
  .blocks == $blocks and
  .quantum_frames == 128 and
  .observers == 7 and
  .queue_success_windows == 7 and
  .queue_full_windows == (($blocks - 1) * 7) and
  .total_violations == 0
' "$trace_root/audit.json" >/dev/null
printf 'builtins realtime syscall trace: PASS (%s blocks)\n' "$blocks"
