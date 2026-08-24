#!/usr/bin/env bash
# Trace the q128 scheduler audit across every spawned thread without launching a benchmark.
#
# Issue 100 gates two modes:
#   steady -- blocks rendered back to back, so the workers never exhaust their linger budget. The
#             coordinator's armed interval must contain at most ONE syscall: the pool has been
#             parked since preparation, so the first block wakes it and no block after that does.
#   paced  -- blocks rendered at the real 2.667 ms cadence, so the workers park between blocks.
#             The coordinator may then issue at most ONE `futex` wake per block (the single
#             documented render-thread syscall, docs/REALTIME_DEPENDENCY_POLICY.md) and nothing
#             else; auxiliary workers may only `futex` and must stay under 5 % CPU between blocks.
#
# Minimum host: 4 cores (three auxiliary workers plus the callback coordinator).
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
binary="$workspace_dir/target/release/miso_engine_scheduler_audit"
trace_root="${MISO_ENGINE_SCHEDULER_TRACE_ROOT:-$workspace_dir/target/issue039/scheduler-audit-strace}"
blocks=10000
workers=3

fail() {
    printf 'issue-100 scheduler syscall trace failure: %s\n' "$1" >&2
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

marker_timestamp() {
    awk -v marker="$1" '
        index($0, marker) {
            for (field = 1; field <= NF; field += 1) {
                if ($field ~ /^[0-9]+[.][0-9]+$/) { print $field; exit }
            }
        }
    ' "$2"
}

lines_in_interval() {
    awk -v armed="$2" -v disarmed="$3" '
        {
            timestamp = "";
            for (field = 1; field <= NF; field += 1) {
                if ($field ~ /^[0-9]+[.][0-9]+$/) { timestamp = $field; break }
            }
            if (timestamp != "" && timestamp > armed && timestamp < disarmed) { print }
        }
    ' "$1"
}

# $1 = mode name, $2 = 1 to pace
run_mode() {
    local mode="$1"
    local pace="$2"
    local root="$trace_root/$mode"
    local prefix="$root/trace"
    mkdir -p "$root"
    if [[ "$pace" == 1 ]]; then
        MISO_ENGINE_SCHEDULER_AUDIT_PACED=1 strace -ff -ttt -qq -s 200 -o "$prefix" "$binary" \
            >"$root/audit.json"
    else
        strace -ff -ttt -qq -s 200 -o "$prefix" "$binary" >"$root/audit.json"
    fi

    mapfile -t trace_files < <(find "$root" -maxdepth 1 -type f -name 'trace.*' | LC_ALL=C sort)
    [[ "${#trace_files[@]}" == 5 ]] || {
        printf '%s: expected coordinator, three workers and the retirement thread; found %s trace files\n' \
            "$mode" "${#trace_files[@]}" >&2
        exit 1
    }

    local marker
    for marker in MISO_ENGINE_SCHEDULER_PHASE_PREPARED MISO_ENGINE_SCHEDULER_PHASE_ARMED MISO_ENGINE_SCHEDULER_PHASE_DISARMED \
        MISO_ENGINE_SCHEDULER_PHASE_RETIRED; do
        local count
        count=$( (rg --no-filename -o --fixed-strings "$marker" "${trace_files[@]}" || true) |
            wc -l | tr -d '[:space:]')
        [[ "$count" == 1 ]] || fail "$mode: expected exactly one $marker marker"
    done

    mapfile -t prepared_marker_files < <(rg -l --fixed-strings MISO_ENGINE_SCHEDULER_PHASE_PREPARED "${trace_files[@]}")
    [[ "${#prepared_marker_files[@]}" == 1 ]] || fail "$mode: prepared marker spans threads"
    local coordinator_trace="${prepared_marker_files[0]}"
    rg -q --fixed-strings MISO_ENGINE_SCHEDULER_PHASE_ARMED "$coordinator_trace" ||
        fail "$mode: armed marker moved threads"
    rg -q --fixed-strings MISO_ENGINE_SCHEDULER_PHASE_DISARMED "$coordinator_trace" ||
        fail "$mode: disarmed marker moved threads"

    local armed disarmed
    armed=$(marker_timestamp MISO_ENGINE_SCHEDULER_PHASE_ARMED "$coordinator_trace")
    disarmed=$(marker_timestamp MISO_ENGINE_SCHEDULER_PHASE_DISARMED "$coordinator_trace")
    [[ -n "$armed" && -n "$disarmed" ]] || fail "$mode: strace timestamps missing from markers"
    awk -v a="$armed" -v d="$disarmed" 'BEGIN { exit !(a < d) }' ||
        fail "$mode: armed interval has no positive ordering"

    local prepared_line
    prepared_line=$(rg -n --fixed-strings MISO_ENGINE_SCHEDULER_PHASE_PREPARED "$coordinator_trace" | cut -d: -f1)
    mapfile -t worker_tids < <(
        sed -n "1,$((prepared_line - 1))p" "$coordinator_trace" |
            awk '$NF ~ /^[0-9]+$/ && ($0 ~ /clone[(]/ || $0 ~ /clone3[(]/ || $0 ~ /clone resumed>/) { print $NF }'
    )
    [[ "${#worker_tids[@]}" == "$workers" ]] ||
        fail "$mode: expected $workers prepared worker TIDs, found ${#worker_tids[@]}"

    # The coordinator's armed interval: nothing at all in steady mode, and only bounded futex
    # wakes in paced mode.
    local coordinator_lines coordinator_count
    coordinator_lines=$(lines_in_interval "$coordinator_trace" "$armed" "$disarmed")
    coordinator_count=$(printf '%s' "$coordinator_lines" | rg -c '' || true)
    [[ -n "$coordinator_lines" ]] || coordinator_count=0
    if [[ "$pace" == 0 ]]; then
        # At most the one wake that starts the pool, and it must be a futex.
        local steady_other
        steady_other=$(printf '%s\n' "$coordinator_lines" | rg -v '^[0-9.]+ futex\(' | rg -v '^$' || true)
        [[ -z "$steady_other" && "$coordinator_count" -le 1 ]] || {
            printf '%s: unexpected coordinator syscall(s) while armed (%s):\n%s\n' \
                "$mode" "$coordinator_count" "$coordinator_lines" >&2
            exit 1
        }
    else
        local non_futex
        non_futex=$(printf '%s\n' "$coordinator_lines" | rg -v '^[0-9.]+ futex\(' | rg -v '^$' || true)
        [[ -z "$non_futex" ]] || {
            printf '%s: the coordinator made a non-futex syscall while armed:\n%s\n' \
                "$mode" "$non_futex" >&2
            exit 1
        }
        [[ "$coordinator_count" -ge 1 && "$coordinator_count" -le "$blocks" ]] ||
            fail "$mode: coordinator issued $coordinator_count syscalls, expected 1..$blocks (at most one wake per rendered block)"
        local wakes
        wakes=$(jq -r '.coordinator_wakes' "$root/audit.json")
        [[ "$coordinator_count" -le "$wakes" ]] ||
            fail "$mode: $coordinator_count coordinator syscalls exceed $wakes counted wakes"
        [[ "$wakes" -le "$blocks" ]] ||
            fail "$mode: $wakes coordinator wakes exceed one per block"
    fi

    # Auxiliary workers: futex only, bounded by park plus two child wakes per block.
    local worker_counts=()
    local tid
    for tid in "${worker_tids[@]}"; do
        local worker_lines worker_count worker_other
        worker_lines=$(lines_in_interval "$prefix.$tid" "$armed" "$disarmed")
        worker_count=$(printf '%s' "$worker_lines" | rg -c '' || true)
        [[ -n "$worker_lines" ]] || worker_count=0
        worker_other=$(printf '%s\n' "$worker_lines" | rg -v '^[0-9.]+ futex\(' | rg -v '^$' || true)
        [[ -z "$worker_other" ]] || {
            printf '%s: worker %s made a non-futex syscall while armed:\n%s\n' \
                "$mode" "$tid" "$worker_other" >&2
            exit 1
        }
        [[ "$worker_count" -le $((3 * blocks)) ]] ||
            fail "$mode: worker $tid issued $worker_count syscalls, over three per block"
        if [[ "$pace" == 0 ]]; then
            # Only the initial wake: a park, and at most the two child wakes it propagates.
            [[ "$worker_count" -le 3 ]] ||
                fail "$mode: a steady-state worker re-parked (worker $tid: $worker_count)"
        fi
        worker_counts+=("$worker_count")
    done

    jq -e --argjson paced "$([[ "$pace" == 1 ]] && printf true || printf false)" '
        .schema_version == 3 and
        .kind == "native_scheduler_realtime_audit" and
        .fixture_id == "issue039-q128-production-v1" and
        .callbacks == 10000 and .sample_rate_hz == 48000 and .quantum_frames == 128 and
        .render_lanes == 4 and .worker_count == 3 and .paced == $paced and
        .plan_swaps == 1 and .pdc_samples > 0 and
        .observer_records == 20000 and .observer_hash > 0 and .output_address > 0 and
        .workers_lost == 0 and .dead_partitions_executed == 0 and
        .blocks_without_lease == 0 and
        .coordinator_forbidden_total == 0 and .worker_forbidden_totals == [0, 0, 0]
    ' "$root/audit.json" >/dev/null || fail "$mode: audit JSON did not validate"

    if [[ "$pace" == 1 ]]; then
        jq -e '[.worker_cpu_fraction[] | . <= 0.05] | all' "$root/audit.json" >/dev/null ||
            fail "$mode: a worker burned more than 5 % of a core between blocks"
    fi

    local worker_tid_json worker_count_json
    worker_tid_json=$(IFS=,; printf '%s' "${worker_tids[*]}")
    worker_count_json=$(IFS=,; printf '%s' "${worker_counts[*]}")
    jq -n \
        --arg mode "$mode" \
        --arg coordinator "$coordinator_trace" \
        --argjson paced "$([[ "$pace" == 1 ]] && printf true || printf false)" \
        --argjson worker_tids "[$worker_tid_json]" \
        --argjson armed_worker_syscalls "[$worker_count_json]" \
        --argjson armed_coordinator_syscalls "$coordinator_count" \
        '{schema_version: 3, kind: "issue100_scheduler_syscall_trace", mode: $mode,
          paced: $paced, coordinator_trace: $coordinator,
          worker_tids: $worker_tids,
          armed_coordinator_syscalls: $armed_coordinator_syscalls,
          armed_worker_syscalls: $armed_worker_syscalls,
          markers: {prepared: 1, armed: 1, disarmed: 1, retired: 1}}' >"$root/validator.json"

    sha256sum "${trace_files[@]}" >"$root/trace-manifest.sha256"
}

run_mode steady 0
run_mode paced 1

jq -n \
    --arg steady_trace "$(sha256sum "$trace_root/steady/trace-manifest.sha256" | awk '{ print $1 }')" \
    --arg steady_audit "$(sha256sum "$trace_root/steady/audit.json" | awk '{ print $1 }')" \
    --arg steady_validator "$(sha256sum "$trace_root/steady/validator.json" | awk '{ print $1 }')" \
    --arg paced_trace "$(sha256sum "$trace_root/paced/trace-manifest.sha256" | awk '{ print $1 }')" \
    --arg paced_audit "$(sha256sum "$trace_root/paced/audit.json" | awk '{ print $1 }')" \
    --arg paced_validator "$(sha256sum "$trace_root/paced/validator.json" | awk '{ print $1 }')" \
    --argjson steady_validator_json "$(cat "$trace_root/steady/validator.json")" \
    --argjson paced_validator_json "$(cat "$trace_root/paced/validator.json")" \
    '{schema_version: 3, kind: "issue100_scheduler_audit_trace_evidence",
      steady: {trace_manifest_sha256: $steady_trace, audit_json_sha256: $steady_audit,
               validator_json_sha256: $steady_validator, validator: $steady_validator_json},
      paced: {trace_manifest_sha256: $paced_trace, audit_json_sha256: $paced_audit,
              validator_json_sha256: $paced_validator, validator: $paced_validator_json}}' \
    >"$trace_root/evidence.json"

printf 'issue-100 q128 all-thread scheduler syscall trace: PASS (%s)\n' "$trace_root"
