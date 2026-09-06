#!/usr/bin/env bash
# Sole controlled Issue-431 current full-chain invocation. Never rebuild, retry, or resume.
set -euo pipefail

script_directory=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repository_root=$(cd "$script_directory/.." && pwd)
artifact_directory="$repository_root/artifacts/issue431-full-chain"
prepared_directory="$repository_root/target/issue431-prepared"
binary="$prepared_directory/bench"
seal="$artifact_directory/builtins-benchmark.preflight.json"
manifest_evidence="$artifact_directory/builtins-benchmark.manifest.json"
readme="$artifact_directory/README.md"
raw_output="$artifact_directory/builtins-benchmark.raw.jsonl"
accepted_output="$artifact_directory/builtins-benchmark.jsonl"
stderr_output="$artifact_directory/builtins-benchmark.stderr"
disposition="$artifact_directory/builtins-benchmark.disposition.json"
runner="$script_directory/run-builtins-current-benchmark.sh"
preflight="$script_directory/preflight-builtins-current-benchmark.sh"
record_validator="$script_directory/builtins-current-benchmark-record-validator.jq"
aggregate_validator="$script_directory/builtins-current-benchmark-validator.jq"
lifecycle="$script_directory/test-builtins-current-benchmark.sh"
source_file="$repository_root/tools/bench/src/builtins.rs"
lock_file="$repository_root/Cargo.lock"
workspace_manifest="$repository_root/Cargo.toml"
config_file="$repository_root/.cargo/config.toml"
preconditions="$script_directory/check-bench-preconditions.sh"
fixture_manifest="$repository_root/fixtures/builtins/v1/MANIFEST.tsv"

for tool in awk cargo cat cmp cp date env git jq kill mkdir mktemp mv rustc sha256sum sleep stat taskset wc; do
    command -v "$tool" >/dev/null 2>&1 || { printf 'Issue-431 runner bootstrap tool unavailable: %s\n' "$tool" >&2; exit 1; }
done
for directory in "$repository_root/artifacts" "$repository_root/target" \
    "$artifact_directory" "$prepared_directory"; do
    [[ -d "$directory" && ! -L "$directory" ]] ||
        { printf 'Issue-431 fixed namespace is not physical: %s\n' "$directory" >&2; exit 1; }
done
[[ -d "$artifact_directory" && ! -L "$artifact_directory" ]] ||
    { printf 'Issue-431 artifact namespace is not a physical directory\n' >&2; exit 1; }
[[ ! -e "$disposition" && ! -L "$disposition" ]] ||
    { printf 'refusing consumed Issue-431 invocation\n' >&2; exit 1; }
scratch=$(mktemp -d "$artifact_directory/.run.XXXXXX")
completed=0
reservation_owned=0
publication_attempted=0
launched=0
child_pid=
child_status=null
validation_status=not_run
reason=prelaunch_failure
candidate_commit=
candidate_tree=
binary_sha256=
workload_started=0
warmup_complete=0
timed_started=0
round_1_complete=0
round_2_complete=0

hash_file() {
    local output status
    if output=$(sha256sum "$1"); then status=0; else status=$?; fi
    ((status == 0)) || return "$status"
    printf '%s' "${output%% *}"
}
artifact_json() {
    local path=$1
    if [[ -f "$path" && ! -L "$path" ]]; then
        jq -n --arg sha "$(hash_file "$path")" --argjson bytes "$(wc -c <"$path")" '{sha256:$sha,bytes:$bytes}'
    else
        printf 'null'
    fi
}
refresh_phases() {
    workload_started=0; warmup_complete=0; timed_started=0; round_1_complete=0; round_2_complete=0
    [[ ! -f "$stderr_output" ]] || {
        workload_started=$(awk '$0=="MISO_ENGINE_BENCH_PHASE workload_started"{n++}END{print n+0}' "$stderr_output")
        warmup_complete=$(awk '$0=="MISO_ENGINE_BENCH_PHASE warmup_complete"{n++}END{print n+0}' "$stderr_output")
        timed_started=$(awk '$0=="MISO_ENGINE_BENCH_PHASE timed_started"{n++}END{print n+0}' "$stderr_output")
        round_1_complete=$(awk '$0=="MISO_ENGINE_BENCH_PHASE round_1_complete"{n++}END{print n+0}' "$stderr_output")
        round_2_complete=$(awk '$0=="MISO_ENGINE_BENCH_PHASE round_2_complete"{n++}END{print n+0}' "$stderr_output")
    }
}
publish_disposition() {
    local status=$1 final_reason=$2 runner_status=$3
    local temporary="$scratch/disposition"
    refresh_phases
    if jq -n -S --arg status "$status" --arg reason "$final_reason" \
      --arg commit "$candidate_commit" --arg tree "$candidate_tree" --arg binary "$binary_sha256" \
      --argjson child_status "$child_status" --arg validation_status "$validation_status" \
      --argjson runner_status "$runner_status" \
      --argjson launched "$launched" \
      --argjson workload "$workload_started" --argjson warmup "$warmup_complete" \
      --argjson timed "$timed_started" --argjson round1 "$round_1_complete" \
      --argjson round2 "$round_2_complete" --argjson raw "$(artifact_json "$raw_output")" \
      --argjson accepted "$(artifact_json "$accepted_output")" \
      --argjson stderr "$(artifact_json "$stderr_output")" \
      '{schema_version:1,issue:431,kind:"builtins_current_benchmark_disposition",
        status:$status,reason:$reason,preflight_invocations:1,runner_invocations:1,
        workload_invocations:$launched,timed_benchmark_invocations:$timed,
        workload_started:$workload,warmup_complete:$warmup,timed_started:$timed,
        round_1_complete:$round1,round_2_complete:$round2,child_status:$child_status,
        validation_status:$validation_status,runner_status:$runner_status,
        candidate_commit:(if $commit=="" then null else $commit end),
        candidate_tree:(if $tree=="" then null else $tree end),
        binary_sha256:(if $binary=="" then null else $binary end),
        raw:$raw,accepted:$accepted,stderr:$stderr}' >"$temporary"; then :; else
        return 1
    fi
    if mv -f -- "$temporary" "$disposition"; then :; else return 1; fi
}
stop_and_reap_child() {
    local index wait_status
    [[ -n "$child_pid" ]] || return 0
    if kill -0 "$child_pid" 2>/dev/null; then
        kill -TERM "$child_pid" 2>/dev/null || :
        for ((index=0; index<50; index++)); do
            kill -0 "$child_pid" 2>/dev/null || break
            sleep 0.02
        done
        if kill -0 "$child_pid" 2>/dev/null; then kill -KILL "$child_pid" 2>/dev/null || :; fi
    fi
    if wait "$child_pid"; then wait_status=0; else wait_status=$?; fi
    child_status=$wait_status
    child_pid=
}
on_exit() {
    local status=$?
    trap - EXIT INT TERM
    stop_and_reap_child
    if [[ "$reservation_owned" == 1 && "$completed" == 0 && "$publication_attempted" == 0 ]]; then
        set +e
        publication_attempted=1
        publish_disposition FAIL "$reason" "$status" || :
    fi
    rm -rf -- "$scratch"
    exit "$status"
}
on_signal() {
    local signal=$1 exit_status=$2
    reason="workload_interrupted_${signal}"
    stop_and_reap_child
    exit "$exit_status"
}
trap on_exit EXIT
trap 'on_signal INT 130' INT
trap 'on_signal TERM 143' TERM

# Persist the sole-invocation reservation before any refusal or possible workload launch.
umask 077
if (set -o noclobber; jq -n -S \
    '{schema_version:1,issue:431,kind:"builtins_current_benchmark_disposition",
      status:"RUNNING",reason:"reserved",preflight_invocations:1,runner_invocations:1,
      workload_invocations:0,timed_benchmark_invocations:0}' >"$disposition"); then
    reservation_owned=1
else
    trap - EXIT INT TERM
    rm -rf -- "$scratch"
    printf 'Issue-431 invocation reservation was not acquired\n' >&2
    exit 1
fi

[[ "$#" == 0 ]] || { reason=invalid_arguments; printf 'usage: %s\n' "$0" >&2; exit 2; }
for override in RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_BUILD_RUSTFLAGS CARGO_BUILD_TARGET \
    CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS \
    RUSTC RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER \
    MISO_ENGINE_BENCH_CANDIDATE_COMMIT MISO_ENGINE_BENCH_BINARY_SHA256 \
    MISO_ENGINE_BENCH_RUST_VERSION MISO_ENGINE_BENCH_LLVM_VERSION \
    MISO_ENGINE_BENCH_TARGET_TRIPLE MISO_ENGINE_BENCH_TARGET_FEATURES \
    MISO_ENGINE_BENCH_PROFILE MISO_ENGINE_BENCH_OPT_LEVEL MISO_ENGINE_BENCH_LTO \
    MISO_ENGINE_BENCH_CODEGEN_UNITS MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE \
    MISO_ENGINE_BENCH_CPU_MODEL MISO_ENGINE_BENCH_CPU_ARCHITECTURE \
    MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT \
    MISO_ENGINE_BENCH_OS MISO_ENGINE_BENCH_KERNEL \
    MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE; do
    [[ ! -v "$override" ]] || { reason=environment_override_forbidden; exit 1; }
done
while IFS= read -r override; do
    [[ "$override" == CARGO_PROFILE_* ]] && { reason=environment_override_forbidden; exit 1; }
done < <(compgen -e)
[[ -z "${MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED:-}" ]] ||
    { reason=uncontrolled_override_forbidden; printf 'Issue-431 refuses uncontrolled benchmark override\n' >&2; exit 1; }
for path in "$raw_output" "$accepted_output" "$stderr_output"; do
    [[ ! -e "$path" && ! -L "$path" ]] ||
        { reason=existing_output; printf 'refusing existing Issue-431 output: %s\n' "$path" >&2; exit 1; }
done
for path in "$binary" "$seal" "$manifest_evidence" "$readme" "$runner" "$preflight" "$record_validator" \
    "$aggregate_validator" "$lifecycle" "$source_file" "$lock_file" "$workspace_manifest" "$config_file" \
    "$preconditions" "$fixture_manifest"; do
    [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 ]] ||
        { reason=missing_sealed_input; exit 1; }
done
[[ -x "$binary" ]] || { reason=missing_sealed_input; exit 1; }

if candidate_commit=$(git -C "$repository_root" rev-parse --verify HEAD); then :; else reason=git_commit_failed; exit 1; fi
if candidate_tree=$(git -C "$repository_root" rev-parse 'HEAD^{tree}'); then :; else reason=git_tree_failed; exit 1; fi
if clean_output=$(git -C "$repository_root" status --porcelain=v1 --untracked-files=normal); then :; else reason=git_status_failed; exit 1; fi
# The durable Issue-431 namespace is expected after preflight; no other dirt is accepted.
filtered_clean=$(printf '%s\n' "$clean_output" | awk '$0 !~ /^\?\? artifacts\/issue431-full-chain\//')
[[ -z "$filtered_clean" ]] || { reason=dirty_candidate; exit 1; }

input_rows="$scratch/input-identities"
: >"$input_rows"
while IFS= read -r relative; do
    path="$repository_root/$relative"
    [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 ]] || { reason=input_identity_mismatch; exit 1; }
    printf '%s  %s\n' "$(hash_file "$path")" "$relative" >>"$input_rows"
done <<'INPUTS'
fixtures/builtins/v1/MANIFEST.tsv
fixtures/builtins/v1/benchmark/full_chain_filters-48000.toml
fixtures/builtins/v1/benchmark/full_chain_filters-96000.toml
fixtures/builtins/v1/benchmark/identity_chain-48000.toml
fixtures/builtins/v1/benchmark/identity_chain-96000.toml
fixtures/builtins/v1/benchmark/matrix_ramp-48000.toml
fixtures/builtins/v1/benchmark/matrix_ramp-96000.toml
fixtures/builtins/v1/benchmark/meter_success_full-48000.toml
fixtures/builtins/v1/benchmark/meter_success_full-96000.toml
fixtures/builtins/v1/benchmark/prepare_256_tracks-48000.toml
fixtures/builtins/v1/benchmark/prepare_256_tracks-96000.toml
fixtures/builtins/v1/pcm/filters-asymmetric.f32le
fixtures/builtins/v1/pcm/identity-signed-zero.f32le
fixtures/builtins/v1/pcm/matrix-ramp-128.f32le
fixtures/builtins/v1/pcm/graph-taps.f32le
fixtures/session/v1/canonical.json
INPUTS
input_tree_sha256=$(hash_file "$input_rows")
binary_sha256=$(hash_file "$binary")
seal_sha256=$(hash_file "$seal")
manifest_evidence_sha256=$(hash_file "$manifest_evidence")
cargo_executable=$(command -v cargo)
rustc_executable=$(command -v rustc)
cargo_version=$(cargo -V) || { reason=build_provenance_mismatch; exit 1; }
rustc_verbose=$(rustc -vV) || { reason=build_provenance_mismatch; exit 1; }
rust_version=$(printf '%s\n' "$rustc_verbose" | awk 'NR==1 {print}')
target_triple=$(printf '%s\n' "$rustc_verbose" | awk '$1=="host:" {print $2}')
llvm_version=$(printf '%s\n' "$rustc_verbose" | awk '$1=="LLVM" && $2=="version:" {print $3}')
jq -e --arg commit "$candidate_commit" --arg tree "$candidate_tree" \
  --arg binary "$binary_sha256" --arg lock "$(hash_file "$lock_file")" \
  --arg workspace "$(hash_file "$workspace_manifest")" \
  --arg source "$(hash_file "$source_file")" --arg config "$(hash_file "$config_file")" \
  --arg preconditions "$(hash_file "$preconditions")" --arg runner "$(hash_file "$runner")" \
  --arg preflight "$(hash_file "$preflight")" --arg record "$(hash_file "$record_validator")" \
  --arg aggregate "$(hash_file "$aggregate_validator")" --arg lifecycle "$(hash_file "$lifecycle")" \
  --arg inputs "$input_tree_sha256" --arg manifest "$(hash_file "$fixture_manifest")" \
  --arg evidence "$manifest_evidence_sha256" --arg readme "$(hash_file "$readme")" \
  --arg cargo_version "$cargo_version" --arg rust_version "$rust_version" \
  --arg llvm_version "$llvm_version" --arg target_triple "$target_triple" \
  --arg cargo_executable "$cargo_executable" --arg rustc_executable "$rustc_executable" \
  --arg cargo_executable_sha "$(hash_file "$cargo_executable")" \
  --arg rustc_executable_sha "$(hash_file "$rustc_executable")" \
  'type=="object" and .schema_version==1 and .issue==431 and
   .kind=="builtins_current_benchmark_preflight" and .status=="READY" and
   .candidate_commit==$commit and .candidate_tree==$tree and .binary_sha256==$binary and
   .cargo_lock_sha256==$lock and .workspace_manifest_sha256==$workspace and
   .tool_source_sha256==$source and
   .cargo_config_sha256==$config and .preconditions_sha256==$preconditions and
   .runner_sha256==$runner and .preflight_script_sha256==$preflight and
   .record_validator_sha256==$record and .aggregate_validator_sha256==$aggregate and
   .lifecycle_sha256==$lifecycle and .input_tree_sha256==$inputs and
   .fixture_manifest_sha256==$manifest and .manifest_evidence_sha256==$evidence and
   .readme_sha256==$readme and
   .cargo_version==$cargo_version and .rust_version==$rust_version and
   .llvm_version==$llvm_version and .target_triple==$target_triple and
   .cargo_executable==$cargo_executable and .rustc_executable==$rustc_executable and
   .cargo_executable_sha256==$cargo_executable_sha and
   .rustc_executable_sha256==$rustc_executable_sha and
   .target_features=="+avx2,+fma" and .profile=="release" and .opt_level=="3" and
   .lto=="fat" and .codegen_units==1 and .panic=="abort" and .debug==1 and
   .debug_assertions==false and .overflow_checks==false and .incremental==false and
   .strip=="none" and .split_debuginfo=="unpacked" and .records_required==20 and
   .warmup_passes==1 and .measured_rounds==2 and .preflight_invocations==1 and
   .runner_invocations==0 and .workload_invocations==0 and .timed_benchmark_invocations==0' \
  "$seal" >/dev/null || { reason=preflight_seal_mismatch; exit 1; }

# Frozen controlled-host checks: affinity, 60-second binary cooldown, load <=0.50,
# and <=5% sibling activity over the 0.2-second sample.
source "$preconditions"
online=$(cat /sys/devices/system/cpu/online) || { reason=control_unavailable; exit 1; }
bench_cpu=$(bench_highest_cpu "$online") || { reason=control_unavailable; exit 1; }
taskset -c "$bench_cpu" true >/dev/null 2>&1 || { reason=affinity_unavailable; exit 1; }
now=$(date +%s)
mtime=$(stat -c %Y "$binary")
age=$((now - mtime))
if ((age < MISO_ENGINE_BENCH_COOLDOWN_SECONDS)); then
    sleep "$((MISO_ENGINE_BENCH_COOLDOWN_SECONDS - age))"
fi
[[ "$(hash_file "$binary")" == "$binary_sha256" ]] || { reason=binary_drift; exit 1; }
loadavg=$(cat /proc/loadavg) || { reason=control_unavailable; exit 1; }
load_one=$(bench_loadavg_one_minute "$loadavg") || { reason=control_unavailable; exit 1; }
bench_within_ceiling "$load_one" "$MISO_ENGINE_BENCH_LOADAVG_CEILING" ||
    { reason=loadavg_above_ceiling; exit 1; }
siblings_text=$(cat "/sys/devices/system/cpu/cpu$bench_cpu/topology/thread_siblings_list") ||
    { reason=control_unavailable; exit 1; }
siblings=$(bench_other_siblings "$bench_cpu" "$siblings_text") || { reason=control_unavailable; exit 1; }
if [[ -n "$siblings" ]]; then
    stat_before=$(cat /proc/stat) || { reason=control_unavailable; exit 1; }
    sleep "$MISO_ENGINE_BENCH_SIBLING_SAMPLE_SECONDS"
    stat_after=$(cat /proc/stat) || { reason=control_unavailable; exit 1; }
    for sibling in $siblings; do
        busy=$(bench_cpu_busy_percent "$stat_before" "$stat_after" "$sibling") ||
            { reason=control_unavailable; exit 1; }
        bench_within_ceiling "$busy" "$MISO_ENGINE_BENCH_SIBLING_BUSY_CEILING" ||
            { reason=smt_sibling_busy; exit 1; }
    done
fi

exec {raw_fd}>"$raw_output"
exec {stderr_fd}>"$stderr_output"
[[ "$(stat -c %h "$raw_output")" == 1 && "$(stat -c %h "$stderr_output")" == 1 ]] ||
    { reason=artifact_link_count; exit 1; }
reason=workload_failed
launched=1
set +e
MISO_ENGINE_BENCH_CANDIDATE_COMMIT="$candidate_commit" \
MISO_ENGINE_BENCH_BINARY_SHA256="$binary_sha256" \
MISO_ENGINE_BENCH_RUST_VERSION="$rust_version" \
MISO_ENGINE_BENCH_LLVM_VERSION="$llvm_version" \
MISO_ENGINE_BENCH_TARGET_TRIPLE="$target_triple" \
MISO_ENGINE_BENCH_TARGET_FEATURES='+avx2,+fma' \
MISO_ENGINE_BENCH_PROFILE=release MISO_ENGINE_BENCH_OPT_LEVEL=3 \
MISO_ENGINE_BENCH_LTO=fat MISO_ENGINE_BENCH_CODEGEN_UNITS=1 \
MISO_ENGINE_BENCH_PANIC=abort MISO_ENGINE_BENCH_DEBUG=1 \
MISO_ENGINE_BENCH_DEBUG_ASSERTIONS=false MISO_ENGINE_BENCH_OVERFLOW_CHECKS=false \
MISO_ENGINE_BENCH_INCREMENTAL=false MISO_ENGINE_BENCH_STRIP=none \
MISO_ENGINE_BENCH_SPLIT_DEBUG_INFO=unpacked \
MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE="controlled; loadavg $loadavg; affinity cpu$bench_cpu; sibling ceiling 5%; cooldown 60s" \
taskset -c "$bench_cpu" "$binary" builtins >&"$raw_fd" 2>&"$stderr_fd" &
child_pid=$!
if wait "$child_pid"; then child_status=0; else child_status=$?; fi
child_pid=
set -e
exec {raw_fd}>&-
exec {stderr_fd}>&-
refresh_phases
if ((child_status != 0)); then
    ((child_status < 128)) || reason=workload_interrupted
    exit "$child_status"
fi
[[ "$workload_started" == 1 && "$warmup_complete" == 1 && "$timed_started" == 1 &&
   "$round_1_complete" == 1 && "$round_2_complete" == 1 ]] ||
    { reason=phase_mismatch; validation_status=phase_failed; exit 1; }

reason=record_validation_failed
jq -s -e -L "$script_directory" 'include "builtins-current-benchmark-record-validator";
 all(.[]; builtins_benchmark_record_valid)' "$raw_output" >/dev/null 2>>"$stderr_output" ||
    { validation_status=record_failed; exit 1; }
reason=aggregate_validation_failed
jq -s -e -L "$script_directory" -f "$aggregate_validator" "$raw_output" >/dev/null 2>>"$stderr_output" ||
    { validation_status=aggregate_failed; exit 1; }
validation_status=passed

reason=promotion_failed
temporary_accepted="$scratch/accepted.jsonl"
cp -- "$raw_output" "$temporary_accepted"
cmp -s -- "$raw_output" "$temporary_accepted" || exit 1
[[ ! -e "$accepted_output" && ! -L "$accepted_output" ]] || exit 1
mv -n -- "$temporary_accepted" "$accepted_output"
[[ ! -e "$temporary_accepted" && -f "$accepted_output" && ! -L "$accepted_output" &&
   "$(stat -c %h "$accepted_output")" == 1 ]] || exit 1
cmp -s -- "$raw_output" "$accepted_output" || exit 1

publication_attempted=1
publish_disposition PASS complete 0 || { printf 'Issue-431 final disposition publication failed\n' >&2; exit 1; }
completed=1
trap - EXIT INT TERM
rm -rf -- "$scratch"
printf '%s\n' "$accepted_output"
