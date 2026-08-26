#!/usr/bin/env bash
# Sole exactly-once entrypoint for the issue #163 phase 2 step 1 wasm console arm.
#
# # What this records, and why it is its own family
#
# The owner's ruling on decision 1 (GO on the unfused multiply-add contract) attached a condition:
# the win must be confirmed **at console level**, not at kernel level. Step 1 is the instrument
# that makes such a confirmation possible -- the console workloads running under the target the
# product ships on, with a baseline taken before any contract change.
#
# `docs/rulings/wasm-kernel-timing-interim.md` listed exactly what a full console arm needed. All
# three of its blockers are answered here rather than worked around:
#
#   1. *`console.rs` and the four compilers are `cfg(not(wasm32))`.* That was bench-tool-level.
#      Every compiler builds for `wasm32-unknown-unknown` unchanged; the subject moved into
#      `miso-engine-console-workload`, which the native legs and the guest both link. No crate
#      changed, and the native console record's nine output digests are byte-identical across the
#      move.
#   2. *The guest has no clock.* It does not get one. The host times around an exported call that
#      renders exactly one block, and every record carries the host-to-guest crossing cost it does
#      not subtract.
#   3. *The guest reads no environment.* It still does not. The round marker and the eleven host
#      metadata names belong to this runner and reach the record through the host process.
#
# The records are a **separate family**. Every one of them carries
# `comparable_with_console_records: false`, because a `wasm-simd128` number and a native console
# number differ in target, in lane width, and in whether one multiply-add is one instruction or
# fifty-four. Every one also carries `browser_field_measurement: false`: wasmtime's Cranelift
# compiles ahead of time and does not tier, deoptimise or recompile on feedback the way a browser
# JIT does, on hardware that is not a phone. This is the determinism-pinned reference. The browser
# numbers remain the owner's field pass.
#
# # Admissibility (#144 item 13, #163 phase 0a)
#
# The console runner's preconditions, sourced from the same file, so a wasm console number and a
# native console number were taken under the same rules. A run that cannot be controlled refuses
# and names the control it lacked.
#
# # The two arms
#
# The default arm writes `artifacts/issue163-phase2-wasm-baseline`: the browser baseline taken
# *before* the phase-2 contract change, which is what the owner's ruling asked for first so that
# the change had a premise to be judged against.
#
# `--issue183` writes `artifacts/issue183`: the paired W4/W8 arm of issue #183 step 2. It builds
# the guest twice -- once as every other arm does, once with `--cfg miso_wasm_simd8`, which is the
# single build-time switch that moves `Backend::current()` on wasm32 from `Simd4` to `Simd8` -- and
# hands both modules to one host process, which renders them inside the same observation. The
# record grows a `wasm_simd128_w8` leg and a third ratio, `wasm_simd128_w8 / wasm_simd128`; that
# ratio against the 1.8x null threshold is the whole decision. The four-lane module is byte-
# identical to the one every other arm builds: no default build reads the cfg.
#
# `--after` writes `artifacts/issue163-phase2`: the same nine rows on the unfused tree. The two are
# separate sealed directories rather than one re-run for the reason every one-shot in this repo is
# -- a consumed measurement describes the tree that produced it -- and for one more that is
# specific to phase 2: the contract change moves every `output_sha256`, so the two records cannot
# be reconciled row by row on digests, only on timings. The digest columns are expected to differ
# and `docs/rulings/unfused-multiply-add-audit.md` is the evidence that the new ones are intended.
#
# `--round2-lane` and `--round2-lane-baseline` are the wasm half of round 2's lane lowerings. On
# this target `Lane::select` is unchanged -- `wide` emits the identical `v128.bitselect` for both
# the old call and the new one -- so the wasm interest is entirely `Lane::max`/`Lane::min`, which
# become `f32x4.pmax`/`f32x4.pmin` with their operands swapped. Class A on both counts: every
# `output_sha256` of the two arms must match, row for row and leg for leg.
set -euo pipefail
arm=baseline
if [[ "$#" == 1 ]]; then
    case "$1" in
        --after) arm=after ;;
        --issue175) arm=issue175 ;;
        --issue182) arm=issue182 ;;
        --issue-loop-eq-r1) arm=issue-loop-eq-r1 ;;
        --compressor-round1) arm=compressor-round1 ;;
        --compressor-round1-baseline) arm=compressor-round1-baseline ;;
        --round1-composed) arm=round1-composed ;;
        --round2-lane) arm=round2-lane ;;
        --round2-lane-baseline) arm=round2-lane-baseline ;;
        --round2-eqrack) arm=round2-eqrack ;;
        --round2-eqrack-baseline) arm=round2-eqrack-baseline ;;
        --round2-comp) arm=round2-comp ;;
        --round2-comp-baseline) arm=round2-comp-baseline ;;
        --round2-lim) arm=round2-lim ;;
        --round2-lim-baseline) arm=round2-lim-baseline ;;
        --round2-composed) arm=round2-composed ;;
        *) printf 'usage: %s [--after|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline|--round1-composed|--issue183|--round2-lane|--round2-lane-baseline|--round2-eqrack|--round2-eqrack-baseline|--round2-comp|--round2-comp-baseline|--round2-lim|--round2-lim-baseline|--round2-composed]
' "$0" >&2; exit 2 ;;
    esac
elif [[ "$#" != 0 ]]; then
    printf 'usage: %s [--after|--issue175|--issue182|--issue-loop-eq-r1|--compressor-round1|--compressor-round1-baseline|--round1-composed|--issue183|--round2-lane|--round2-lane-baseline|--round2-eqrack|--round2-eqrack-baseline|--round2-comp|--round2-comp-baseline|--round2-lim|--round2-lim-baseline|--round2-composed]
' "$0" >&2
    exit 2
fi
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
# shellcheck source=scripts/check-bench-preconditions.sh
source "$root/scripts/check-bench-preconditions.sh"

if [[ "$arm" == after ]]; then
    artifact_dir="$root/artifacts/issue163-phase2"
elif [[ "$arm" == issue175 ]]; then
    artifact_dir="$root/artifacts/issue175"
elif [[ "$arm" == issue182 ]]; then
    artifact_dir="$root/artifacts/issue182"
elif [[ "$arm" == issue-loop-eq-r1 ]]; then
    artifact_dir="$root/artifacts/issue-loop-eq-r1"
elif [[ "$arm" == compressor-round1 ]]; then
    artifact_dir="$root/artifacts/compressor-round1"
elif [[ "$arm" == compressor-round1-baseline ]]; then
    artifact_dir="$root/artifacts/compressor-round1-baseline"
elif [[ "$arm" == round1-composed ]]; then
    artifact_dir="$root/artifacts/round1-composed"
elif [[ "$arm" == issue183 ]]; then
    artifact_dir="$root/artifacts/issue183"
elif [[ "$arm" == round2-lane ]]; then
    artifact_dir="$root/artifacts/round2-lane"
elif [[ "$arm" == round2-lane-baseline ]]; then
    artifact_dir="$root/artifacts/round2-lane-baseline"
elif [[ "$arm" == round2-eqrack ]]; then
    artifact_dir="$root/artifacts/round2-eqrack"
elif [[ "$arm" == round2-eqrack-baseline ]]; then
    artifact_dir="$root/artifacts/round2-eqrack-baseline"
elif [[ "$arm" == round2-comp ]]; then
    artifact_dir="$root/artifacts/round2-comp"
elif [[ "$arm" == round2-comp-baseline ]]; then
    artifact_dir="$root/artifacts/round2-comp-baseline"
elif [[ "$arm" == round2-lim ]]; then
    artifact_dir="$root/artifacts/round2-lim"
elif [[ "$arm" == round2-lim-baseline ]]; then
    artifact_dir="$root/artifacts/round2-lim-baseline"
elif [[ "$arm" == round2-composed ]]; then
    artifact_dir="$root/artifacts/round2-composed"
else
    artifact_dir="$root/artifacts/issue163-phase2-wasm-baseline"
fi
raw="$artifact_dir/wasm-console-benchmark.raw.jsonl"
accepted="$artifact_dir/wasm-console-benchmark.accepted.jsonl"
stderr_log="$artifact_dir/wasm-console-benchmark.stderr.log"
disposition="$artifact_dir/wasm-console-benchmark.disposition.json"
for path in "$raw" "$accepted" "$stderr_log" "$disposition"; do
    [[ ! -e "$path" ]] || { printf 'refusing to overwrite phase-2 wasm artifact: %s\n' "$path" >&2; exit 1; }
done
# The native legs are the comparison's denominators. `Simd8` is the backend every recorded console
# number was taken at, and the host process refuses to run at anything else.
[[ "$(uname -m)" == "x86_64" ]] || { printf 'the native legs require x86_64\n' >&2; exit 1; }
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
candidate_commit_sha256=
binary_sha256=
guest_sha256=
guest_simd8_sha256=
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
    local commit_json=null commit_sha_json=null binary_json=null guest_json=null
    local guest_simd8_json=null
    [[ -n "$candidate_commit" ]] && commit_json="\"$candidate_commit\""
    [[ -n "$candidate_commit_sha256" ]] && commit_sha_json="\"$candidate_commit_sha256\""
    [[ -n "$binary_sha256" ]] && binary_json="\"$binary_sha256\""
    [[ -n "$guest_sha256" ]] && guest_json="\"$guest_sha256\""
    [[ -n "$guest_simd8_sha256" ]] && guest_simd8_json="\"$guest_simd8_sha256\""
    printf '{"schema_version":1,"issue":163,"phase":"2-step1","status":"%s","reason":"%s","runner_invocations":1,"workload_process_launches":%s,"warmup_launches":%s,"measured_rounds_completed":%s,"measurement_control":"%s","cpu_affinity":"%s","candidate_commit":%s,"candidate_commit_sha256":%s,"binary_sha256":%s,"guest_module_sha256":%s,"guest_simd8_module_sha256":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s,"stderr_sha256":%s,"stderr_bytes":%s}\n' \
        "$status" "$reason" "$workload_process_launches" "$warmup_launches" \
        "$measured_rounds_completed" "$measurement_control" "$cpu_affinity" \
        "$commit_json" "$commit_sha_json" "$binary_json" "$guest_json" "$guest_simd8_json" \
        "$raw_sha" "$raw_bytes" "$accepted_sha" "$accepted_bytes" \
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
trap on_exit EXIT
trap 'failure_reason=interrupted; exit 130' INT TERM

failure_reason=fixture_failed
bash scripts/check-console-benchmark-fixture.sh >>"$stderr_log" 2>&1 || exit 1

failure_reason=candidate_identity_failed
candidate_commit=$(git rev-parse --verify HEAD 2>>"$stderr_log")
candidate_commit_sha256=$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')

# Both legs are built with the *same* frozen release settings, which is the console runner's
# freeze. Two reasons, and they pull the same way. The native leg has to be built the way the
# binary behind the phase-3 console record was built, or the ratio's denominator is not the number
# it is being read against. And the two legs have to be built alike as each other, or the ratio
# measures an optimisation-settings difference alongside the target difference it is for.
export CARGO_PROFILE_RELEASE_OPT_LEVEL=3
export CARGO_PROFILE_RELEASE_LTO=false
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16

failure_reason=guest_build_failed
guest_target=target/ci/issue163-phase2-guest
CARGO_TARGET_DIR="$guest_target" RUSTFLAGS="-C target-feature=+simd128" \
    cargo build --locked --release --quiet --target wasm32-unknown-unknown \
    -p miso-engine-wasm-console-guest 2>>"$stderr_log"
guest="$guest_target/wasm32-unknown-unknown/release/miso_engine_wasm_console_guest.wasm"
[[ -f "$guest" ]] || { failure_reason=missing_guest_module; exit 1; }
guest_sha256=$(sha256sum "$guest" | awk '{print $1}')

# The issue #183 paired arm needs a second guest: the same source, the same flags, plus the one
# build-time cfg that moves `Backend::current()` on wasm32 from `Simd4` to `Simd8`. It is built
# into its own target directory so the W4 module measured above is not disturbed, and the two are
# handed to the host together so both are timed inside one observation.
#
# The default W4 module is byte-identical whether or not this arm exists: nothing above reads the
# cfg, and `guest_sha256` is recorded before this block runs.
guests=("$guest")
if [[ "$arm" == issue183 ]]; then
    failure_reason=guest_simd8_build_failed
    guest_simd8_target=target/ci/issue183-guest-simd8
    CARGO_TARGET_DIR="$guest_simd8_target" \
        RUSTFLAGS="-C target-feature=+simd128 --cfg miso_wasm_simd8" \
        cargo build --locked --release --quiet --target wasm32-unknown-unknown \
        -p miso-engine-wasm-console-guest 2>>"$stderr_log"
    guest_simd8="$guest_simd8_target/wasm32-unknown-unknown/release/miso_engine_wasm_console_guest.wasm"
    [[ -f "$guest_simd8" ]] || { failure_reason=missing_guest_simd8_module; exit 1; }
    guest_simd8_sha256=$(sha256sum "$guest_simd8" | awk '{print $1}')
    [[ "$guest_simd8_sha256" != "$guest_sha256" ]] || {
        failure_reason=guest_simd8_module_identical
        printf 'the eight-lane guest hashed the same as the four-lane one: the width override did not take\n' >&2
        exit 1
    }
    guests+=("$guest_simd8")
fi

failure_reason=build_failed
cargo build --locked --release --quiet -p miso-engine-wasm-console 2>>"$stderr_log"
binary="$root/target/release/miso_engine_wasm_console"
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
    "${affinity[@]}" "$binary" "${guests[@]}"
}

# One untimed warmup, then exactly the two frozen measured rounds.
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
[[ "$(wc -l <"$raw")" == 22 ]] || exit 1
failure_reason=validation_failed
jq -s -e -f scripts/wasm-console-benchmark-validator.jq "$raw" >/dev/null || exit 1
failure_reason=accepted_promotion_failed
: >"$accepted"
cp -- "$raw" "$accepted"
cmp -s -- "$raw" "$accepted" || exit 1
write_disposition PASS complete
failed=0
trap - EXIT INT TERM
printf '%s\n' "$accepted"
