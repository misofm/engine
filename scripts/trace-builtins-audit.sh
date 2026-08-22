#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
binary="$workspace_dir/target/release/miso_engine_builtins_audit"
trace_root="$workspace_dir/target/issue7/strace"
validator="$workspace_dir/scripts/validate-realtime-trace.sh"

[[ "$#" -eq 0 ]] || {
  printf 'trace-builtins-audit.sh accepts no arguments\n' >&2
  exit 2
}

cargo build --quiet --locked --release --manifest-path "$workspace_dir/Cargo.toml" \
  -p miso-engine-builtins-audit
command -v strace >/dev/null 2>&1 || {
  printf 'strace is required for the builtins realtime syscall gate\n' >&2
  exit 1
}
mkdir -p "$trace_root"
trace_prefix="$trace_root/trace"
find "$trace_root" -maxdepth 1 -type f -name 'trace.*' -delete
strace -ff -qq -ttt -o "$trace_prefix" "$binary" >"$trace_root/audit.json"
"$validator" "$trace_root" MISO_ISSUE069_DIRECT_RT_BEGIN MISO_ISSUE069_DIRECT_RT_END 7 \
  >"$trace_root/validator.json"
jq -e '
  .kind == "issue069_direct_realtime_audit" and
  .calls == 1000000 and
  .sample_rate_hz == 48000 and
  .quantum_frames == 128 and
  .schedule_blocks == 6 and
  .stable_left_address == true and .stable_right_address == true and
  .allocations == 0 and .deallocations == 0 and .locks == 0 and
  .feature_detection == 0 and .logs == 0 and .file_io == 0 and
  .network_io == 0 and .syscalls == 0 and .panic_unwinds == 0 and
  .total_violations == 0
' "$trace_root/audit.json" >/dev/null
raw_hash=$(for file in "$trace_root"/trace.*; do sha256sum "$file" | cut -d' ' -f1; done | sha256sum | cut -d' ' -f1)
validator_hash=$(sha256sum "$trace_root/validator.json" | cut -d' ' -f1)
printf 'builtins all-TID realtime trace: PASS (raw=%s validator=%s)\n' \
  "$raw_hash" "$validator_hash"
