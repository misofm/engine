#!/usr/bin/env bash
# Sole exactly-once entrypoint for the issue #163 phase 0b wasm kernel timing arm.
#
# # What this measures, and what it does not
#
# Full 0b is the console benchmark running under a wasm runtime. It is not deliverable without
# porting the bench tool: `console.rs` and every compiler it drives are `cfg(not(wasm32))`
# dependencies, `wasm32-unknown-unknown` has no clock, and the guest reads no environment. The
# exact list of what full 0b needs is in `docs/rulings/wasm-kernel-timing-interim.md`.
#
# This runner takes the smallest honest wasm measurement available today, through the harness gate
# G5 already owns: the frozen lane kernels, built for `wasm32-unknown-unknown` with `+simd128`,
# executed under the pinned wasmtime, timed on the host clock, against the same kernels run
# natively in the same process minutes apart. Its records are a **separate family**: every one of
# them carries `comparable_with_console_records: false`, because a `wasm-simd128` number and a
# native console number differ in target, in width and in whether a multiply-add is one
# instruction or fifty-four.
#
# The admissibility preconditions are the console runner's (#144 item 13, #163 phase 0a), sourced
# from the same file, so a timing number and a console number were taken under the same rules.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
# shellcheck source=scripts/check-bench-preconditions.sh
source "$root/scripts/check-bench-preconditions.sh"

artifact_dir="$root/artifacts/issue163-phase0"
raw="$artifact_dir/wasm-kernel-timing.raw.jsonl"
accepted="$artifact_dir/wasm-kernel-timing.accepted.jsonl"
stderr_log="$artifact_dir/wasm-kernel-timing.stderr.log"
disposition="$artifact_dir/wasm-kernel-timing.disposition.json"
for path in "$raw" "$accepted" "$stderr_log" "$disposition"; do
    [[ ! -e "$path" ]] || { printf 'refusing to overwrite phase-0b artifact: %s\n' "$path" >&2; exit 1; }
done
[[ "$(uname -m)" == "x86_64" ]] || { printf 'the native leg requires x86_64\n' >&2; exit 1; }
grep -qm1 -w avx2 /proc/cpuinfo || { printf 'the native Simd8 leg requires AVX2\n' >&2; exit 1; }
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'a one-shot measurement requires a clean committed candidate\n' >&2
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
measurement_control=unevaluated
cpu_affinity=unevaluated

write_disposition() {
    local status=$1 reason=$2 raw_sha=null raw_bytes=0 accepted_sha=null accepted_bytes=0
    [[ -e "$raw" ]] && { raw_sha="\"$(sha256sum "$raw" | awk '{print $1}')\""; raw_bytes=$(wc -c <"$raw"); }
    [[ -e "$accepted" ]] && { accepted_sha="\"$(sha256sum "$accepted" | awk '{print $1}')\""; accepted_bytes=$(wc -c <"$accepted"); }
    local commit_json=null
    [[ -n "$candidate_commit" ]] && commit_json="\"$candidate_commit\""
    printf '{"schema_version":1,"issue":163,"phase":"0b","status":"%s","reason":"%s","runner_invocations":1,"workload_process_launches":%s,"warmup_launches":%s,"measured_rounds_completed":%s,"measurement_control":"%s","cpu_affinity":"%s","candidate_commit":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s}\n' \
        "$status" "$reason" "$workload_process_launches" "$warmup_launches" \
        "$measured_rounds_completed" "$measurement_control" "$cpu_affinity" "$commit_json" \
        "$raw_sha" "$raw_bytes" "$accepted_sha" "$accepted_bytes" >"$disposition"
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
trap on_exit EXIT
trap 'failure_reason=interrupted; exit 130' INT TERM

failure_reason=candidate_identity_failed
candidate_commit=$(git rev-parse --verify HEAD 2>>"$stderr_log")

failure_reason=build_failed
guest_target=target/ci/issue163-phase0-guest
CARGO_TARGET_DIR="$guest_target" RUSTFLAGS="-C target-feature=+simd128" \
    cargo build --locked --release --quiet --target wasm32-unknown-unknown \
    -p miso-engine-wasm-gate-guest 2>>"$stderr_log"
cargo build --locked --release --quiet -p miso-engine-wasm-gates 2>>"$stderr_log"
guest="$guest_target/wasm32-unknown-unknown/release/miso_engine_wasm_gate_guest.wasm"
host="$root/target/release/miso_engine_wasm_gates"
[[ -f "$guest" && -x "$host" ]] || { failure_reason=missing_artifact; exit 1; }

# ---------------------------------------------------------------------------------------------
# The same admissibility preconditions the console runner applies (#163 phase 0a).
# ---------------------------------------------------------------------------------------------
failure_reason=precondition_failed
declare -a refusals=()
declare -a affinity=()
cpu_affinity=uncontrolled
note_affinity='affinity none'
note_sibling='smt not-checked'

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

binary_mtime_before=$(stat -c %Y "$host" 2>/dev/null) || { failure_reason=binary_mtime_unreadable; exit 1; }
binary_age=$(( $(date +%s) - binary_mtime_before ))
cooldown_waited=0
if (( binary_age < MISO_ENGINE_BENCH_COOLDOWN_SECONDS )); then
    cooldown_waited=$(( MISO_ENGINE_BENCH_COOLDOWN_SECONDS - binary_age ))
    sleep "$cooldown_waited"
fi
binary_mtime_after=$(stat -c %Y "$host" 2>/dev/null) || { failure_reason=binary_mtime_unreadable; exit 1; }
[[ "$binary_mtime_before" == "$binary_mtime_after" ]] || {
    failure_reason=binary_rebuilt_during_cooldown
    exit 1
}

loadavg_text=$(< /proc/loadavg)
loadavg_one=$(bench_loadavg_one_minute "$loadavg_text") || loadavg_one=
if [[ -n "$loadavg_one" ]] &&
    bench_within_ceiling "$loadavg_one" "$MISO_ENGINE_BENCH_LOADAVG_CEILING"; then
    :
else
    refusals+=(loadavg_above_ceiling)
fi

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
elif [[ "${MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED:-}" == 1 ]]; then
    measurement_control=uncontrolled
else
    failure_reason="precondition_${refusals[0]}"
    printf 'refusing an uncontrolled measurement: %s\n' "${refusals[*]}" >&2
    printf 'loadavg %s; %s; %s\n' "$loadavg_text" "$note_affinity" "$note_sibling" >&2
    printf 'set MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 to record an uncontrolled run instead.\n' >&2
    exit 1
fi

# The four legs, in emission order. Native and wasm at both widths, so the *target* half of the
# comparison is isolated from the *width* half: native-simd4 against wasm-simd128 at simd4 is one
# target difference at one width, and native-simd8 is the production native backend for context.
run_round() {
    local round=$1
    workload_process_launches=$((workload_process_launches + 1))
    "${affinity[@]}" "$host" --native-timing simd4 --round "$round"
    "${affinity[@]}" "$host" --native-timing simd8 --round "$round"
    "${affinity[@]}" "$host" "$guest" --wasm-timing simd4 --expect-backend simd4 --round "$round"
    "${affinity[@]}" "$host" "$guest" --wasm-timing simd8 --expect-backend simd4 --round "$round"
}

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
[[ "$(wc -l <"$raw")" == 8 ]] || exit 1
failure_reason=validation_failed
jq -s -e -f scripts/wasm-kernel-timing-validator.jq "$raw" >/dev/null || exit 1
failure_reason=accepted_promotion_failed
: >"$accepted"
cp -- "$raw" "$accepted"
cmp -s -- "$raw" "$accepted" || exit 1
write_disposition PASS complete
failed=0
trap - EXIT INT TERM
printf '%s\n' "$accepted"
