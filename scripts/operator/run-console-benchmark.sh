#!/usr/bin/env bash
# The native console benchmark's sole timing entrypoint. Do not invoke its binary directly: the
# runner is what supplies the round marker and the host metadata, and a direct invocation produces
# a record whose provenance is a guess.
#
# Usage: scripts/operator/run-console-benchmark.sh --step NAME
#
# A person runs it; no workflow does, which is why it lives in `scripts/operator/` (moved from
# `scripts/` by #1027).
#
# `--step NAME` is the per-issue arm of a sequential optimisation batch: one record per merged
# issue, written to `artifacts/steps/NAME`, so each issue's motion is read against the step before
# it rather than against one paired baseline. NAME is lowercase kebab-case, at most 64 characters.
# Any other invocation, including none, is a usage error (exit 2).
#
# Before anything is timed the run refuses (exit 1), in this order: when a record already exists
# in that directory (a record is never overwritten), when the host is not x86_64 with AVX2, when
# the tree is not clean and committed, and when the console fixture check fails. It then builds
# `bench` in release and refuses again when an admissibility precondition below is unmet. An
# admitted run takes one untimed warmup and exactly two measured rounds, validates the 62 records
# with `console-benchmark-validator.jq`, and promotes them to the accepted file beside a
# disposition.
# `scripts/operator/preflight-console-benchmark.sh --step NAME` checks everything that can fail
# without launching the workload; run it first.
#
# #1025 retired the 48 historical one-shot arms (`--phase2` ... `--plumbing-floor-baseline`) and
# the no-argument default (`artifacts/issue149`): each had its record and could only refuse. Their
# invocations and histories are in the runner as it stood before the retirement:
# https://github.com/misofm/engine/blob/d3349b72dd9e48674d087d14722368b23c4bfc1b/scripts/run-console-benchmark.sh
#
# # Admissibility (#144 item 13, #163 phase 0a)
#
# Everything from `check-bench-preconditions.sh` down to the warmup is a *precondition*, not a
# note. The runner refuses a measurement it cannot control and names which control it lacked. The
# escape hatch `MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1` exists for machines where control is
# genuinely impossible, and it does not make the run look controlled: every record it writes
# carries `measurement_control: "uncontrolled"` and the validator refuses to let that record claim
# otherwise.
set -euo pipefail
if [[ "$#" != 2 || "$1" != --step ]]; then
    printf 'usage: %s --step NAME\n' "$0" >&2
    exit 2
fi
[[ "$2" =~ ^[a-z0-9][a-z0-9-]{0,63}$ ]] || { printf 'invalid --step name: %s\n' "$2" >&2; exit 2; }
step_directory="steps/$2"
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$root"
# shellcheck source=scripts/check-bench-preconditions.sh
source "$root/scripts/check-bench-preconditions.sh"

artifact_dir="$root/artifacts/$step_directory"
raw="$artifact_dir/console-benchmark.raw.jsonl"
accepted="$artifact_dir/console-benchmark.accepted.jsonl"
stderr_log="$artifact_dir/console-benchmark.stderr.log"
disposition="$artifact_dir/console-benchmark.disposition.json"
# #184: the perf-counter evidence behind the records' cycle columns. One file per launch that was
# counted, kept beside the records for the same reason the stderr log is kept -- a derived column
# whose instrument left no trace is a claim, not a measurement.
core_clock_log="$artifact_dir/console-benchmark.core-clock.csv"
for path in "$raw" "$accepted" "$stderr_log" "$disposition" "$core_clock_log"; do
    [[ ! -e "$path" ]] || { printf 'refusing to overwrite console artifact: %s\n' "$path" >&2; exit 1; }
done
[[ "$(uname -m)" == "x86_64" ]] || { printf 'the console benchmark requires x86_64\n' >&2; exit 1; }
grep -qm1 -w avx2 /proc/cpuinfo || { printf 'the console benchmark requires AVX2\n' >&2; exit 1; }
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'the console benchmark requires a clean committed candidate\n' >&2
    exit 1
}

umask 077
mkdir -p "$artifact_dir"
set -o noclobber
: >"$stderr_log"

failed=1
failure_reason=unexpected_failure
workload_process_launches=0
warmup_launches=0
measured_rounds_completed=0
candidate_commit=
candidate_commit_sha256=
binary_sha256=
# Declared before the exit trap can fire so a failure *inside* the precondition block still writes
# a disposition that says what the run knew about its own admissibility at the point it failed.
measurement_control=unevaluated
cpu_affinity=unevaluated

artifact_identity() {
    local path=$1
    if [[ -e "$path" ]]; then
        printf '"%s" %s' "$(sha256sum "$path" | awk '{print $1}')" "$(wc -c <"$path")"
    else
        printf 'null 0'
    fi
}
write_disposition() {
    local status=$1 reason=$2 raw_identity accepted_identity stderr_identity
    raw_identity=$(artifact_identity "$raw")
    accepted_identity=$(artifact_identity "$accepted")
    stderr_identity=$(artifact_identity "$stderr_log")
    local raw_sha raw_bytes accepted_sha accepted_bytes stderr_sha stderr_bytes
    read -r raw_sha raw_bytes <<<"$raw_identity"
    read -r accepted_sha accepted_bytes <<<"$accepted_identity"
    read -r stderr_sha stderr_bytes <<<"$stderr_identity"
    local candidate_json=null candidate_sha_json=null binary_sha_json=null
    [[ -n "$candidate_commit" ]] && candidate_json="\"$candidate_commit\""
    [[ -n "$candidate_commit_sha256" ]] && candidate_sha_json="\"$candidate_commit_sha256\""
    [[ -n "$binary_sha256" ]] && binary_sha_json="\"$binary_sha256\""
    printf '{"schema_version":1,"issue":149,"status":"%s","reason":"%s","runner_invocations":1,"workload_process_launches":%s,"warmup_launches":%s,"measured_rounds_completed":%s,"measurement_control":"%s","cpu_affinity":"%s","candidate_commit":%s,"candidate_commit_sha256":%s,"binary_sha256":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s,"stderr_sha256":%s,"stderr_bytes":%s}\n' \
        "$status" "$reason" "$workload_process_launches" "$warmup_launches" \
        "$measured_rounds_completed" "$measurement_control" "$cpu_affinity" \
        "$candidate_json" "$candidate_sha_json" \
        "$binary_sha_json" "$raw_sha" "$raw_bytes" "$accepted_sha" "$accepted_bytes" \
        "$stderr_sha" "$stderr_bytes" >"$disposition"
}
on_exit() {
    local status=$?
    trap - EXIT
    if [[ "$failed" == 1 && ! -e "$disposition" ]]; then
        set +e
        write_disposition FAIL "$failure_reason"
    fi
    exit "$status"
}
on_signal() {
    failure_reason=interrupted
    exit 130
}
trap on_exit EXIT
trap on_signal INT TERM

failure_reason=fixture_failed
bash scripts/check-console-benchmark-fixture.sh >>"$stderr_log" 2>&1 || exit 1

failure_reason=candidate_identity_failed
candidate_commit=$(git rev-parse --verify HEAD 2>>"$stderr_log")
candidate_commit_sha256=$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')

# Freeze the release profile so the recorded build metadata describes the binary actually run.
export CARGO_PROFILE_RELEASE_OPT_LEVEL=3
export CARGO_PROFILE_RELEASE_LTO=false
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
failure_reason=build_failed
cargo build --locked --release --quiet -p bench 2>>"$stderr_log"
binary="$root/target/release/bench"
[[ -x "$binary" ]] || { failure_reason=missing_binary; exit 1; }
failure_reason=binary_identity_failed
binary_sha256=$(sha256sum "$binary" | awk '{print $1}')

failure_reason=metadata_failed
cpu_model=$(awk -F: '/model name/ {gsub(/^ +/, "", $2); print $2; exit}' /proc/cpuinfo)
rust_version=$(rustc -V)
llvm_version=$(rustc -vV | awk -F: '/LLVM version/ {gsub(/^ +/, "", $2); print $2}')
target_triple=$(rustc -vV | awk -F: '/host/ {gsub(/^ +/, "", $2); print $2}')
target_features="runtime-avx2$(grep -qm1 ' fma ' /proc/cpuinfo && printf '%s' ',fma' || true);baseline"
governor_or_power_mode=unknown
if [[ -r /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]]; then
    governor_or_power_mode=$(< /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)
fi

# ---------------------------------------------------------------------------------------------
# Preconditions. Each one refuses with a named reason, or is recorded as waived.
# ---------------------------------------------------------------------------------------------
failure_reason=precondition_failed
allow_uncontrolled=0
[[ "${MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED:-}" == 1 ]] && allow_uncontrolled=1
declare -a refusals=()
declare -a affinity=()
cpu_affinity=uncontrolled
note_affinity='affinity none'
note_sibling='smt not-checked'

# 1. Single-core affinity. Two tenants alternating on one core is the single largest source of
#    per-block variance on a loaded host, and it is the one the process itself can eliminate.
if command -v taskset >/dev/null 2>&1 && [[ -r /sys/devices/system/cpu/online ]] &&
    bench_cpu=$(bench_highest_cpu "$(< /sys/devices/system/cpu/online)") &&
    taskset -c "$bench_cpu" true >/dev/null 2>&1; then
    affinity=(taskset -c "$bench_cpu")
    cpu_affinity="$bench_cpu"
    note_affinity="affinity cpu $bench_cpu"
else
    refusals+=(affinity_unavailable)
    bench_cpu=
fi

# 2. Binary-mtime cooldown. A release build saturates every core; the package is hot and the
#    governor is ramped for tens of seconds afterwards. Wait out the remainder of the cooldown
#    before anything is timed, then refuse if the binary moved underneath us while we waited --
#    a second build racing this one would make the recorded sha256 describe a different program
#    than the one measured.
binary_mtime_before=$(stat -c %Y "$binary" 2>/dev/null) || {
    failure_reason=binary_mtime_unreadable
    exit 1
}
binary_age=$(( $(date +%s) - binary_mtime_before ))
cooldown_waited=0
if (( binary_age < MISO_ENGINE_BENCH_COOLDOWN_SECONDS )); then
    cooldown_waited=$(( MISO_ENGINE_BENCH_COOLDOWN_SECONDS - binary_age ))
    sleep "$cooldown_waited"
fi
binary_mtime_after=$(stat -c %Y "$binary" 2>/dev/null) || {
    failure_reason=binary_mtime_unreadable
    exit 1
}
[[ "$binary_mtime_before" == "$binary_mtime_after" ]] || {
    failure_reason=binary_rebuilt_during_cooldown
    exit 1
}

# 3. Load-average ceiling, read *after* the cooldown so it describes the machine that is about to
#    be measured rather than the machine that just finished compiling.
loadavg_text=$(< /proc/loadavg)
loadavg_one=$(bench_loadavg_one_minute "$loadavg_text") || loadavg_one=
if [[ -n "$loadavg_one" ]] &&
    bench_within_ceiling "$loadavg_one" "$MISO_ENGINE_BENCH_LOADAVG_CEILING"; then
    :
else
    refusals+=(loadavg_above_ceiling)
fi

# 4. SMT sibling quiet. Cheap where the topology is exported and skipped, not refused, where it is
#    not: a container that hides `/sys/devices/system/cpu/*/topology` is not evidence of a busy
#    sibling, and inventing a refusal from missing information is its own dishonesty. What the
#    check cannot establish it says it could not establish.
if [[ -n "$bench_cpu" ]]; then
    sibling_path="/sys/devices/system/cpu/cpu$bench_cpu/topology/thread_siblings_list"
    if [[ -r "$sibling_path" ]]; then
        siblings=$(bench_other_siblings "$bench_cpu" "$(< "$sibling_path")") || siblings=
        if [[ -z "$siblings" ]]; then
            note_sibling='smt none'
        else
            stat_before=$(< /proc/stat)
            sleep "$MISO_ENGINE_BENCH_SIBLING_SAMPLE_SECONDS"
            stat_after=$(< /proc/stat)
            note_sibling="smt siblings $siblings"
            for sibling in $siblings; do
                busy=$(bench_cpu_busy_percent "$stat_before" "$stat_after" "$sibling") || busy=
                if [[ -z "$busy" ]]; then
                    note_sibling="$note_sibling cpu$sibling=unreadable"
                    continue
                fi
                note_sibling="$note_sibling cpu$sibling=$busy%"
                bench_within_ceiling "$busy" "$MISO_ENGINE_BENCH_SIBLING_BUSY_CEILING" ||
                    refusals+=(smt_sibling_busy)
            done
        fi
    else
        note_sibling='smt topology-unavailable'
    fi
fi

if [[ "${#refusals[@]}" == 0 ]]; then
    measurement_control=controlled
    background_load_note="controlled; loadavg $loadavg_text; ceiling $MISO_ENGINE_BENCH_LOADAVG_CEILING; $note_affinity; $note_sibling; cooldown ${MISO_ENGINE_BENCH_COOLDOWN_SECONDS}s waited ${cooldown_waited}s"
elif [[ "$allow_uncontrolled" == 1 ]]; then
    measurement_control=uncontrolled
    background_load_note="uncontrolled; MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1; waived ${refusals[*]}; loadavg $loadavg_text; $note_affinity; $note_sibling; cooldown ${MISO_ENGINE_BENCH_COOLDOWN_SECONDS}s waited ${cooldown_waited}s"
else
    failure_reason="precondition_${refusals[0]}"
    printf 'refusing an uncontrolled measurement: %s\n' "${refusals[*]}" >&2
    printf 'loadavg %s; %s; %s\n' "$loadavg_text" "$note_affinity" "$note_sibling" >&2
    printf 'set MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 to record an uncontrolled run instead.\n' >&2
    exit 1
fi

# ---------------------------------------------------------------------------------------------
# The pinned core's clock (#184). A cycle column needs cycles, and wall time is not cycles.
# ---------------------------------------------------------------------------------------------
#
# `perf stat` counts `cycles` and `task-clock` over a launch, and their ratio is the clock the
# pinned core actually ran at while it was running this subject -- a hardware counter reading,
# taken under exactly the preconditions above, not a nameplate frequency and not `/proc/cpuinfo`.
# The warmup launch supplies it, because the subject has to have it in its environment *before* it
# builds a record and a `perf stat` result only exists after its workload has exited.
#
# The measured rounds are counted too, and their own ratio is checked against the exported figure.
# That is what makes using the warmup's number honest rather than convenient: if the core clocked
# differently while the numbers that are kept were being taken, the run refuses instead of
# publishing cycle columns derived from a clock that was not in force.
#
# A host with no usable counter is not a failure. It exports nothing, every record omits the whole
# column group, and the records validate exactly as the sealed ones under `artifacts/` do.
readonly MISO_ENGINE_BENCH_CORE_CLOCK_DRIFT_CEILING=0.03
core_clock_hz=
core_clock_source=
core_clock_available=0

# Cycles per second from one `perf stat -x,` CSV. The last complete pair in the file wins, so a
# file appended to by several launches reports the launch that wrote last. Empty when neither a
# `cycles` nor a `task-clock` row counted.
core_clock_from_csv() {
    awk -F, '
        $1 ~ /^[0-9]+([.][0-9]+)?$/ && $3 == "cycles" { cycles = $1 }
        $1 ~ /^[0-9]+([.][0-9]+)?$/ && $3 == "task-clock" { milliseconds = $1 }
        END { if (cycles > 0 && milliseconds > 0) printf "%.0f", cycles * 1000.0 / milliseconds }
    ' "$1"
}

# Refuse a run whose measured round did not clock like the warmup the records were told about.
core_clock_agrees() {
    local measured=$1
    awk -v exported="$core_clock_hz" -v measured="$measured" \
        -v ceiling="$MISO_ENGINE_BENCH_CORE_CLOCK_DRIFT_CEILING" \
        'BEGIN {
            drift = (measured - exported) / exported
            if (drift < 0) { drift = -drift }
            exit (drift <= ceiling) ? 0 : 1
        }'
}

core_clock_probe=$(mktemp)
if command -v perf >/dev/null 2>&1 &&
    perf stat -x, -e cycles,task-clock -o "$core_clock_probe" -- true >/dev/null 2>&1 &&
    [[ -n "$(core_clock_from_csv "$core_clock_probe")" ]]; then
    core_clock_available=1
fi
rm -f "$core_clock_probe"

run_round() {
    local round=$1 counted=${2:-}
    local -a counter=()
    if [[ -n "$counted" ]]; then
        counter=(perf stat -x, -e cycles,task-clock --append -o "$core_clock_log" --)
    fi
    workload_process_launches=$((workload_process_launches + 1))
    MISO_ENGINE_BENCH_ROUND="$round" \
    MISO_ENGINE_BENCH_CORE_CLOCK_HZ="$core_clock_hz" \
    MISO_ENGINE_BENCH_CORE_CLOCK_SOURCE="$core_clock_source" \
    MISO_ENGINE_BENCH_CANDIDATE_COMMIT="$candidate_commit" \
    MISO_ENGINE_BENCH_CPU_MODEL="$cpu_model" \
    MISO_ENGINE_BENCH_RUST_VERSION="$rust_version" \
    MISO_ENGINE_BENCH_LLVM_VERSION="$llvm_version" \
    MISO_ENGINE_BENCH_TARGET_TRIPLE="$target_triple" \
    MISO_ENGINE_BENCH_TARGET_FEATURES="$target_features" \
    MISO_ENGINE_BENCH_PROFILE=release \
    MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE="$background_load_note" \
    MISO_ENGINE_BENCH_MEASUREMENT_CONTROL="$measurement_control" \
    MISO_ENGINE_BENCH_CPU_AFFINITY="$cpu_affinity" \
    MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE="$governor_or_power_mode" \
    "${counter[@]}" "${affinity[@]}" "$binary" console
}

# One untimed warmup, then exactly the two frozen measured rounds. Raw stdout is append-only after
# its exclusive creation; failures preserve every byte emitted by the failed process.
failure_reason=warmup_failed
counted=
if (( core_clock_available == 1 )); then
    counted=counted
fi
run_round warmup "$counted" >/dev/null 2>>"$stderr_log" || exit 1
warmup_launches=1
if [[ -n "$counted" ]]; then
    failure_reason=core_clock_unreadable
    core_clock_hz=$(core_clock_from_csv "$core_clock_log")
    [[ -n "$core_clock_hz" ]] || exit 1
    core_clock_source="perf stat cycles/task-clock over the warmup launch, cpu $cpu_affinity"
fi
failure_reason=round_1_failed
run_round 1 "$counted" >"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=1
if [[ -n "$counted" ]]; then
    failure_reason=precondition_core_clock_drift
    core_clock_agrees "$(core_clock_from_csv "$core_clock_log")" || {
        printf 'refusing cycle columns taken under a clock that moved: exported %s Hz\n' \
            "$core_clock_hz" >&2
        exit 1
    }
fi
failure_reason=round_2_failed
run_round 2 "$counted" >>"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=2
if [[ -n "$counted" ]]; then
    failure_reason=precondition_core_clock_drift
    core_clock_agrees "$(core_clock_from_csv "$core_clock_log")" || {
        printf 'refusing cycle columns taken under a clock that moved: exported %s Hz\n' \
            "$core_clock_hz" >&2
        exit 1
    }
fi
failure_reason=record_count
[[ "$(wc -l <"$raw")" == 62 ]] || exit 1
failure_reason=validation_failed
jq -s -e -L scripts -f scripts/console-benchmark-validator.jq "$raw" >/dev/null || exit 1
failure_reason=accepted_promotion_failed
: >"$accepted"
cp -- "$raw" "$accepted"
cmp -s -- "$raw" "$accepted" || exit 1
write_disposition PASS complete
failed=0
trap - EXIT INT TERM
printf '%s\n' "$accepted"
