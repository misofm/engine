#!/usr/bin/env bash
# Usage: trace-builtins-graph-audit.sh [path/to/audit]
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
trace_root="$workspace_dir/target/issue7/graph-strace"
validator="$workspace_dir/scripts/validate-realtime-trace.sh"
[[ "$#" -le 1 ]] || {
  printf 'usage: trace-builtins-graph-audit.sh [path/to/audit]\n' >&2
  exit 2
}

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
        printf 'trace-builtins-graph-audit.sh: explicit binary path must be an existing executable file: %s\n' \
            "$binary" >&2
        exit 1
    }
else
    binary="$workspace_dir/target/release/audit"
fi

if [[ ! -x "$binary" ]]; then
  cargo build --quiet --locked --release --manifest-path "$workspace_dir/Cargo.toml" \
    -p audit --bin audit
fi
command -v strace >/dev/null 2>&1 || {
  printf 'strace is required for the issue-007 graph lifecycle syscall gate\n' >&2
  exit 1
}
mkdir -p "$trace_root"
trace_prefix="$trace_root/trace"
find "$trace_root" -maxdepth 1 -type f -name 'trace.*' -delete
strace -ff -qq -ttt -s 200 -o "$trace_prefix" "$binary" builtins-graph >"$trace_root/audit.json"
"$validator" "$trace_root" MISO_ENGINE_BUILTINS_GRAPH_RT_BEGIN MISO_ENGINE_BUILTINS_GRAPH_RT_END 4 \
  >"$trace_root/validator.json"
jq -e '
  .schema_version == 1 and .trace_files >= 2 and .intervals == 4 and .violations == 0
' "$trace_root/validator.json" >/dev/null
jq -e '
  .kind == "issue069_graph_realtime_lifecycle_audit" and
  .renders == 1000000 and .quantum_frames == 128 and .observers == 7 and
  .render_count_by_plan == {"A":1,"B":999999,"C":0} and
  .swaps_applied == 1 and .swaps_deferred == 999998 and
  .prior_plan_renders_on_deferred == 999998 and
  .pdc_samples == 9 and .distinct_taps == 7 and
  .retirement_owner_destroyed == 1 and .control_owner_destroyed == 2 and
  .render_owner_destroyed == 0 and
  .stable_left_address == true and .stable_right_address == true and
  .allocations == 0 and .deallocations == 0 and .locks == 0 and
  .feature_detection == 0 and .logs == 0 and .file_io == 0 and
  .network_io == 0 and .syscalls == 0 and .panic_unwinds == 0 and
  .total_violations == 0
' "$trace_root/audit.json" >/dev/null
# #1060: the record's hash is a live report, not a pin. The jq check above holds every claim the
# record makes (the render/swap/retirement transcript, PDC and taps, and zero allocations, locks
# and syscalls on render), and the audit compares the rendered PCM and meters with the accepted
# fixtures byte for byte before it prints. What the hash added was its labels and constants: it was
# re-pinned five to eight times in September 2026 and never caught a defect.
audit_hash=$(sha256sum "$trace_root/audit.json" | cut -d' ' -f1)
raw_hash=$(for file in "$trace_root"/trace.*; do sha256sum "$file" | cut -d' ' -f1; done | sha256sum | cut -d' ' -f1)
validator_hash=$(sha256sum "$trace_root/validator.json" | cut -d' ' -f1)
printf 'issue-070 graph all-TID trace: PASS (audit=%s raw=%s validator=%s)\n' \
  "$audit_hash" "$raw_hash" "$validator_hash"
