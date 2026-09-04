#!/usr/bin/env bash
# Usage: trace-builtins-audit.sh [path/to/audit]
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
trace_root="$workspace_dir/target/issue7/strace"
validator="$workspace_dir/scripts/validate-realtime-trace.sh"

[[ "$#" -le 1 ]] || {
  printf 'usage: trace-builtins-audit.sh [path/to/audit]\n' >&2
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
        printf 'trace-builtins-audit.sh: explicit binary path must be an existing executable file: %s\n' \
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
  printf 'strace is required for the builtins realtime syscall gate\n' >&2
  exit 1
}
mkdir -p "$trace_root"
trace_prefix="$trace_root/trace"
find "$trace_root" -maxdepth 1 -type f -name 'trace.*' -delete
strace -ff -qq -ttt -s 200 -o "$trace_prefix" "$binary" builtins >"$trace_root/audit.json"
"$validator" "$trace_root" MISO_ENGINE_BUILTINS_RT_BEGIN MISO_ENGINE_BUILTINS_RT_END 7 \
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
