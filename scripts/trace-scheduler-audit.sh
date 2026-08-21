#!/usr/bin/env bash
# Trace the q128 scheduler audit across every spawned thread without launching a benchmark.
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
binary="$workspace_dir/target/release/miso_engine_scheduler_audit"
trace_root="$workspace_dir/target/issue039/scheduler-audit-strace"
trace_prefix="$trace_root/trace"

fail() {
    printf 'issue-039 scheduler syscall trace failure: %s\n' "$1" >&2
    exit 1
}

[[ "$#" == 0 ]] || fail 'this audit accepts no arguments'
command -v strace >/dev/null 2>&1 || fail 'strace is required'
command -v jq >/dev/null 2>&1 || fail 'jq is required'
command -v sha256sum >/dev/null 2>&1 || fail 'sha256sum is required'
[[ ! -e "$trace_root" ]] || fail "refusing to overwrite preserved trace directory: $trace_root"

cargo build --quiet --offline --locked --release --manifest-path "$workspace_dir/Cargo.toml" \
    -p miso-engine-scheduler-audit
mkdir -p "$trace_root"
strace -ff -ttt -qq -o "$trace_prefix" "$binary" >"$trace_root/audit.json"

mapfile -t trace_files < <(find "$trace_root" -maxdepth 1 -type f -name 'trace.*' | LC_ALL=C sort)
[[ "${#trace_files[@]}" == 8 ]] || {
    printf 'expected coordinator, six prepared workers, and retirement thread; found %s trace files\n' \
        "${#trace_files[@]}" >&2
    exit 1
}

marker_count() {
    (rg --no-filename -o --fixed-strings "$1" "${trace_files[@]}" || true) |
        wc -l | tr -d '[:space:]'
}

for marker in \
    MISO_039_PHASE_PREPARED \
    MISO_039_PHASE_ARMED \
    MISO_039_PHASE_DISARMED \
    MISO_039_PHASE_RETIRED; do
    [[ "$(marker_count "$marker")" == 1 ]] || fail "expected exactly one $marker marker"
done

mapfile -t prepared_marker_files < <(rg -l --fixed-strings MISO_039_PHASE_PREPARED "${trace_files[@]}")
[[ "${#prepared_marker_files[@]}" == 1 ]] || fail 'prepared marker does not identify one coordinator trace'
coordinator_trace="${prepared_marker_files[0]}"
rg -q --fixed-strings MISO_039_PHASE_ARMED "$coordinator_trace" || fail 'armed marker moved threads'
rg -q --fixed-strings MISO_039_PHASE_DISARMED "$coordinator_trace" || fail 'disarmed marker moved threads'

marker_line() {
    rg -n --fixed-strings "$1" "$coordinator_trace" | cut -d: -f1
}

prepared_line=$(marker_line MISO_039_PHASE_PREPARED)
armed_line=$(marker_line MISO_039_PHASE_ARMED)
disarmed_line=$(marker_line MISO_039_PHASE_DISARMED)
[[ "$prepared_line" -lt "$armed_line" && "$armed_line" -lt "$disarmed_line" ]] ||
    fail 'phase marker order is not prepared/armed/disarmed'

marker_timestamp() {
    awk -v marker="$1" '
        index($0, marker) {
            for (field = 1; field <= NF; field += 1) {
                if ($field ~ /^[0-9]+[.][0-9]+$/) {
                    print $field;
                    exit;
                }
            }
        }
    ' "$2"
}

armed_timestamp=$(marker_timestamp MISO_039_PHASE_ARMED "$coordinator_trace")
disarmed_timestamp=$(marker_timestamp MISO_039_PHASE_DISARMED "$coordinator_trace")
[[ -n "$armed_timestamp" && -n "$disarmed_timestamp" ]] || fail 'strace timestamps missing from phase markers'
awk -v armed="$armed_timestamp" -v disarmed="$disarmed_timestamp" \
    'BEGIN { exit !(armed < disarmed) }' || fail 'armed interval has no positive timestamp ordering'

mapfile -t prepared_worker_tids < <(
    sed -n "1,$((prepared_line - 1))p" "$coordinator_trace" |
        awk '$NF ~ /^[0-9]+$/ && ($0 ~ /clone[(]/ || $0 ~ /clone3[(]/ || $0 ~ /clone resumed>/) { print $NF }'
)
[[ "${#prepared_worker_tids[@]}" == 6 ]] ||
    fail "expected six prepared worker TIDs before the prepared marker, found ${#prepared_worker_tids[@]}"
active_worker_tids=("${prepared_worker_tids[@]:3:3}")

for worker_tid in "${active_worker_tids[@]}"; do
    [[ -f "$trace_prefix.$worker_tid" ]] || fail "missing active worker trace for TID $worker_tid"
done

unexpected_coordinator=$(awk -v armed_line="$armed_line" -v disarmed_line="$disarmed_line" \
    'NR > armed_line && NR < disarmed_line { print }' "$coordinator_trace")
[[ -z "$unexpected_coordinator" ]] || {
    printf 'unexpected coordinator syscall(s) while armed:\n%s\n' "$unexpected_coordinator" >&2
    exit 1
}

syscalls_in_interval() {
    awk -v armed="$armed_timestamp" -v disarmed="$disarmed_timestamp" '
        {
            timestamp = "";
            for (field = 1; field <= NF; field += 1) {
                if ($field ~ /^[0-9]+[.][0-9]+$/) {
                    timestamp = $field;
                    break;
                }
            }
            if (timestamp != "" && timestamp > armed && timestamp < disarmed) {
                print;
            }
        }
    ' "$1"
}

for worker_tid in "${active_worker_tids[@]}"; do
    worker_syscalls=$(syscalls_in_interval "$trace_prefix.$worker_tid")
    [[ -z "$worker_syscalls" ]] || {
        printf 'unexpected active-worker syscall(s) while armed (TID %s):\n%s\n' \
            "$worker_tid" "$worker_syscalls" >&2
        exit 1
    }
done

mapfile -t retired_marker_files < <(rg -l --fixed-strings MISO_039_PHASE_RETIRED "${trace_files[@]}")
[[ "${#retired_marker_files[@]}" == 1 ]] || fail 'retired marker does not identify one retirement trace'
retired_timestamp=$(marker_timestamp MISO_039_PHASE_RETIRED "${retired_marker_files[0]}")
[[ -n "$retired_timestamp" ]] || fail 'retired marker lacks a trace timestamp'
awk -v disarmed="$disarmed_timestamp" -v retired="$retired_timestamp" \
    'BEGIN { exit !(disarmed < retired) }' || fail 'retirement marker was not emitted after disarm'

jq -e '
    .schema_version == 2 and
    .kind == "native_scheduler_realtime_audit" and
    .fixture_id == "issue039-q128-production-v1" and
    .callbacks == 10000 and .sample_rate_hz == 48000 and .quantum_frames == 128 and
    .render_lanes == 4 and .worker_count == 3 and .plan_swaps == 1 and .pdc_samples > 0 and
    .observer_records == 20000 and .observer_hash > 0 and .output_address > 0 and
    .coordinator_forbidden_total == 0 and .worker_forbidden_totals == [0, 0, 0]
' "$trace_root/audit.json" >/dev/null

worker_tid_json=$(IFS=,; printf '%s' "${active_worker_tids[*]}")
printf \
    '{"schema_version":1,"kind":"issue039_scheduler_syscall_trace","coordinator_trace":"%s","active_worker_tids":[%s],"armed_coordinator_syscalls":0,"armed_worker_syscalls":[0,0,0],"markers":{"prepared":1,"armed":1,"disarmed":1,"retired":1}}\n' \
    "$coordinator_trace" "$worker_tid_json" >"$trace_root/validator.json"

sha256sum "${trace_files[@]}" >"$trace_root/trace-manifest.sha256"
trace_sha256=$(sha256sum "$trace_root/trace-manifest.sha256" | awk '{ print $1 }')
audit_sha256=$(sha256sum "$trace_root/audit.json" | awk '{ print $1 }')
validator_sha256=$(sha256sum "$trace_root/validator.json" | awk '{ print $1 }')
jq -n \
    --arg trace_sha256 "$trace_sha256" \
    --arg audit_sha256 "$audit_sha256" \
    --arg validator_sha256 "$validator_sha256" \
    --argjson trace_file_count "${#trace_files[@]}" \
    --argjson active_worker_tids "[$worker_tid_json]" \
    '{schema_version: 1, kind: "issue039_scheduler_audit_trace_evidence",
      trace_manifest_sha256: $trace_sha256, audit_json_sha256: $audit_sha256,
      validator_json_sha256: $validator_sha256, trace_file_count: $trace_file_count,
      active_worker_tids: $active_worker_tids}' >"$trace_root/evidence.json"

printf 'issue-039 q128 all-thread scheduler syscall trace: PASS (%s)\n' "$trace_root"
