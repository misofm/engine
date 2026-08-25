#!/usr/bin/env bash
# Sole exactly-once Issue-149 console qualification timing entrypoint. Do not invoke its binary
# directly: the runner is what supplies the round marker and the host metadata, and a direct
# invocation produces a record whose provenance is a guess.
#
# One optional argument, `--phase2`, `--phase3` or `--issue163-phase0`, moves the artifact
# directory and changes nothing else. Phase 1's
# record is a consumed one-shot authority: it describes the tree that produced it, and the sealed
# fast dB tier (#144 item 5) deliberately moves the rendered bits it measured, so a phase-2 number
# belongs beside it rather than on top of it. Phase 3 gets its own directory for the opposite
# reason -- the multiband ramping split is class A and moves no rendered bit at all, so its console
# numbers are a *re-measurement* of the phase-2 tree and have to be readable as one rather than
# blended into it. `--issue163-phase0` writes to `artifacts/issue163-phase0` for the same reason
# again, and one further one: phase 0 changes what the subject *measures* -- five decomposition
# rows, a meters arm and an observation arm join the stream, and the run is admissible under
# preconditions the earlier records were never held to -- so its numbers are a new authority
# beside the issue-149 ones and not a continuation of them. Every directory keeps the same
# refusal-to-overwrite discipline, so none of them can be quietly re-run. `--issue163-phase3`
# writes to `artifacts/issue163-phase3` on the same terms as `--issue163-phase4` below: phase 3
# (bank interleave) is class A throughout, so every workload's output digest is the phase-1 digest
# to the bit, and the arm exists so that the interleaved tree's numbers describe the interleaved
# tree. `--issue163-phase4`
# writes to `artifacts/issue163-phase4` and is a *re-measurement* of the phase-1 subject: phase 4
# is class A throughout, so every workload's output digest is the phase-1 digest to the bit and
# the two records are directly comparable row by row. It gets its own directory anyway, for the
# reason every phase does -- a consumed one-shot describes the tree that produced it, and phase 4
# changes what the idle row costs without changing what any row computes.
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
phase_directory=issue149
if [[ "$#" == 1 ]]; then
    case "$1" in
        --phase2) phase_directory=issue149-phase2 ;;
        --phase3) phase_directory=issue149-phase3 ;;
        --issue163-phase0) phase_directory=issue163-phase0 ;;
        --issue163-phase1) phase_directory=issue163-phase1 ;;
        --issue163-phase3) phase_directory=issue163-phase3 ;;
        --issue163-phase4) phase_directory=issue163-phase4 ;;
        *) printf 'usage: %s [--phase2|--phase3|--issue163-phase0|--issue163-phase1|--issue163-phase3|--issue163-phase4]\n' "$0" >&2; exit 2 ;;
    esac
elif [[ "$#" != 0 ]]; then
    printf 'usage: %s [--phase2|--phase3|--issue163-phase0|--issue163-phase1|--issue163-phase3|--issue163-phase4]\n' "$0" >&2
    exit 2
fi
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
# shellcheck source=scripts/check-bench-preconditions.sh
source "$root/scripts/check-bench-preconditions.sh"

artifact_dir="$root/artifacts/$phase_directory"
raw="$artifact_dir/console-benchmark.raw.jsonl"
accepted="$artifact_dir/console-benchmark.accepted.jsonl"
stderr_log="$artifact_dir/console-benchmark.stderr.log"
disposition="$artifact_dir/console-benchmark.disposition.json"
for path in "$raw" "$accepted" "$stderr_log" "$disposition"; do
    [[ ! -e "$path" ]] || { printf 'refusing to overwrite console artifact: %s\n' "$path" >&2; exit 1; }
done
[[ "$(uname -m)" == "x86_64" ]] || { printf 'Issue-149 qualification requires x86_64\n' >&2; exit 1; }
grep -qm1 -w avx2 /proc/cpuinfo || { printf 'Issue-149 qualification requires AVX2\n' >&2; exit 1; }
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'Issue-149 qualification requires a clean committed candidate\n' >&2
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
cargo build --locked --release --quiet -p miso-engine-bench 2>>"$stderr_log"
binary="$root/target/release/miso_engine_bench"
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

run_round() {
    local round=$1
    workload_process_launches=$((workload_process_launches + 1))
    MISO_ENGINE_BENCH_ROUND="$round" \
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
    "${affinity[@]}" "$binary" console
}

# One untimed warmup, then exactly the two frozen measured rounds. Raw stdout is append-only after
# its exclusive creation; failures preserve every byte emitted by the failed process.
failure_reason=warmup_failed
run_round warmup >/dev/null 2>>"$stderr_log" || exit 1
warmup_launches=1
failure_reason=round_1_failed
run_round 1 >"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=1
failure_reason=round_2_failed
run_round 2 >>"$raw" 2>>"$stderr_log" || exit 1
measured_rounds_completed=2
failure_reason=record_count
[[ "$(wc -l <"$raw")" == 26 ]] || exit 1
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
