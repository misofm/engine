#!/usr/bin/env bash
# Sole exactly-once Issue-109 descriptive benchmark entrypoint.
set -euo pipefail

script_directory=${BASH_SOURCE[0]%/*}
[[ "$script_directory" != "${BASH_SOURCE[0]}" ]] || script_directory=.
root="$(cd "$script_directory/.." && pwd)"
artifact_directory="$root/target/issue109"
repair_seal="$artifact_directory/metadata-repair.seal.json"
preflight_seal="$artifact_directory/builtins-benchmark.preflight.json"
binary="$artifact_directory/miso_engine_builtins_bench"
raw="$artifact_directory/builtins-benchmark.raw.jsonl"
accepted="$artifact_directory/builtins-benchmark.jsonl"
stderr_log="$artifact_directory/builtins-benchmark.validator.stderr"
prelaunch_disposition="$artifact_directory/builtins-benchmark.prelaunch.disposition.json"
disposition="$artifact_directory/builtins-benchmark.disposition.json"
record_validator="$script_directory/builtins-benchmark-record-validator.jq"
aggregate_validator="$script_directory/builtins-benchmark-validator.jq"

for tool in mkdir mktemp mv rm stat; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'Issue-109 runner bootstrap tool unavailable: %s\n' "$tool" >&2
        exit 1
    }
done
mkdir -p "$artifact_directory"
[[ ! -L "$artifact_directory" ]] || { printf 'Issue-109 artifact directory is a symlink\n' >&2; exit 1; }
for terminal in "$prelaunch_disposition" "$disposition"; do
    [[ ! -e "$terminal" && ! -L "$terminal" ]] || {
        printf 'refusing consumed Issue-109 runner authority: %s\n' "$terminal" >&2
        exit 1
    }
done

scratch="$(mktemp -d "$artifact_directory/.builtins-benchmark-109.XXXXXX")"
candidate_commit= candidate_tree= binary_sha256= preflight_sha256= repair_sha256=
metadata_projection_sha256=
launch_attempted=0 workload_started=0 timed_started=0 warmup_passes=0 measured_rounds=0
workload_status=1 failure_reason=prelaunch_failure completed=0

hash_file() { sha256sum "$1" | awk '{print $1}'; }
json_string_or_null() { [[ -z "$1" ]] && printf null || printf '"%s"' "$1"; }
artifact_identity() {
    local path=$1
    if command -v sha256sum >/dev/null 2>&1 && command -v wc >/dev/null 2>&1 &&
       [[ -f "$path" && ! -L "$path" ]]; then
        printf '"%s" %s' "$(hash_file "$path")" "$(wc -c <"$path" | tr -d ' ')"
    else
        printf 'null 0'
    fi
}
refresh_progress() {
    workload_started=0 timed_started=0 warmup_passes=0 measured_rounds=0
    if [[ -f "$stderr_log" && ! -L "$stderr_log" ]] && command -v awk >/dev/null 2>&1; then
        workload_started="$(awk '$0 == "MISO_BUILTINS_BENCH_PHASE workload_started" {n++} END {print n + 0}' "$stderr_log")"
        timed_started="$(awk '$0 == "MISO_BUILTINS_BENCH_PHASE timed_started" {n++} END {print n + 0}' "$stderr_log")"
        warmup_passes="$(awk '$0 == "MISO_BUILTINS_BENCH_PHASE warmup_complete" {n++} END {print n + 0}' "$stderr_log")"
        measured_rounds="$(awk '$0 == "MISO_BUILTINS_BENCH_PHASE round_1_complete" || $0 == "MISO_BUILTINS_BENCH_PHASE round_2_complete" {n++} END {print n + 0}' "$stderr_log")"
    fi
}
publish_disposition() {
    local destination=$1 kind=$2 status=$3 reason=$4 exit_status=$5
    local raw_id accepted_id stderr_id raw_sha raw_bytes accepted_sha accepted_bytes stderr_sha stderr_bytes
    raw_id="$(artifact_identity "$raw")"; read -r raw_sha raw_bytes <<<"$raw_id"
    accepted_id="$(artifact_identity "$accepted")"; read -r accepted_sha accepted_bytes <<<"$accepted_id"
    stderr_id="$(artifact_identity "$stderr_log")"; read -r stderr_sha stderr_bytes <<<"$stderr_id"
    local temporary="$scratch/disposition.json"
    printf '{"schema_version":1,"issue":109,"kind":"%s","status":"%s","reason":"%s","preflight_invocations":1,"runner_invocations":1,"workload_invocations":%s,"timed_benchmark_invocations":%s,"warmup_passes":%s,"measured_rounds_completed":%s,"candidate_commit":%s,"candidate_tree":%s,"binary_sha256":%s,"repair_seal_sha256":%s,"preflight_sha256":%s,"metadata_projection_sha256":%s,"workload_exit_status":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s,"stderr_sha256":%s,"stderr_bytes":%s}\n' \
        "$kind" "$status" "$reason" "$workload_started" "$timed_started" "$warmup_passes" \
        "$measured_rounds" "$(json_string_or_null "$candidate_commit")" \
        "$(json_string_or_null "$candidate_tree")" "$(json_string_or_null "$binary_sha256")" \
        "$(json_string_or_null "$repair_sha256")" "$(json_string_or_null "$preflight_sha256")" \
        "$(json_string_or_null "$metadata_projection_sha256")" "$exit_status" \
        "$raw_sha" "$raw_bytes" "$accepted_sha" "$accepted_bytes" "$stderr_sha" "$stderr_bytes" \
        >"$temporary"
    [[ ! -e "$destination" && ! -L "$destination" ]] || return 1
    mv -n "$temporary" "$destination"
    [[ ! -e "$temporary" && -f "$destination" && ! -L "$destination" &&
       "$(stat -c %h "$destination")" == 1 ]]
}
on_exit() {
    local status=$?
    trap - EXIT INT TERM
    if [[ "$completed" == 0 ]]; then
        set +e
        refresh_progress
        if [[ "$launch_attempted" == 1 ]]; then
            publish_disposition "$disposition" builtins_benchmark_disposition FAIL \
                "$failure_reason" "$workload_status"
        else
            publish_disposition "$prelaunch_disposition" \
                builtins_benchmark_prelaunch_disposition FAIL "$failure_reason" 1
        fi
    fi
    rm -rf "$scratch"
    exit "$status"
}
on_signal() { failure_reason=workload_interrupted; workload_status=130; exit 130; }
trap on_exit EXIT
trap on_signal INT TERM

[[ $# == 0 ]] || { failure_reason=invalid_arguments; printf 'usage: %s\n' "$0" >&2; exit 2; }
for tool in awk cmp cp getconf git jq rustc sha256sum sort tr uname wc; do
    command -v "$tool" >/dev/null 2>&1 || {
        failure_reason=missing_tool
        printf 'Issue-109 required tool unavailable: %s\n' "$tool" >&2
        exit 1
    }
done
for path in "$raw" "$accepted" "$stderr_log"; do
    [[ ! -e "$path" && ! -L "$path" ]] || { failure_reason=existing_output; exit 1; }
done

require_one_link() {
    [[ -f "$1" && ! -L "$1" && "$(stat -c %h "$1")" == 1 ]]
}
verify_issue072() {
    local name bytes digest path
    while read -r name bytes digest; do
        path="$root/target/issue72/$name"
        require_one_link "$path" && [[ "$(wc -c <"$path" | tr -d ' ')" == "$bytes" ]] &&
            [[ "$(hash_file "$path")" == "$digest" ]] || return 1
    done <<'EOF'
nonbenchmark.seal.json 2109 7c38b068ae16055df3cfe6b817943f5fbb1a639d85597560e223d631bc37885d
miso_engine_builtins_bench 3200296 a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912
builtins-benchmark.preflight.json 1525 f4e624b88eddbea5eb09928b544d13093d9a68be278f8afb6b70076fc8dce6bf
builtins-benchmark.raw.jsonl 40136 c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a
builtins-benchmark.jsonl 40136 c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a
builtins-benchmark.validator.stderr 211 7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396
builtins-benchmark.disposition.json 1252 b650449d6a8944f4b00fcd833e5f775c9601a9aeb580864624a4b2c978a0698e
EOF
    [[ ! -e "$root/target/issue72/builtins-benchmark.prelaunch.disposition.json" &&
       ! -L "$root/target/issue72/builtins-benchmark.prelaunch.disposition.json" ]]
}

for path in "$repair_seal" "$preflight_seal" "$binary" "$record_validator" "$aggregate_validator" \
    "$root/Cargo.lock" "$root/tools/miso-engine-builtins-bench/Cargo.toml" \
    "$root/tools/miso-engine-builtins-bench/src/main.rs" "$root/fixtures/builtins/v1/MANIFEST.tsv" \
    "$root/fixtures/builtins/v1/pcm/graph-taps.f32le" \
    "$root/fixtures/builtins/v1/meters/graph-taps.jsonl" \
    "$root/.github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md" \
    "$script_directory/preflight-builtins-benchmark-109.sh" \
    "$script_directory/test-builtins-benchmark-109.sh" \
    "$script_directory/check-builtins-benchmark-109.sh" \
    "$script_directory/test-builtins-benchmark-109-policy.sh"; do
    require_one_link "$path" || { failure_reason=missing_authority; exit 1; }
done
[[ -x "$binary" ]] || { failure_reason=missing_authority; exit 1; }
verify_issue072 || { failure_reason=inherited_evidence_mismatch; exit 1; }
[[ "$(stat -c %i "$root/target/issue72/builtins-benchmark.raw.jsonl")" != \
   "$(stat -c %i "$root/target/issue72/builtins-benchmark.jsonl")" ]] || {
    failure_reason=inherited_evidence_mismatch; exit 1;
}

candidate_branch="$(git -C "$root" branch --show-current)"
candidate_commit="$(git -C "$root" rev-parse --verify HEAD)"
candidate_tree="$(git -C "$root" rev-parse 'HEAD^{tree}')"
[[ "$candidate_branch" == codex/batch-benchmark-109 &&
   -z "$(git -C "$root" status --porcelain=v1 --untracked-files=normal)" ]] || {
    failure_reason=dirty_candidate
    exit 1
}

binary_sha256="$(hash_file "$binary")"
repair_sha256="$(hash_file "$repair_seal")"
preflight_sha256="$(hash_file "$preflight_seal")"
lock_sha="$(hash_file "$root/Cargo.lock")"
manifest_sha="$(hash_file "$root/fixtures/builtins/v1/MANIFEST.tsv")"
pcm_sha="$(hash_file "$root/fixtures/builtins/v1/pcm/graph-taps.f32le")"
meter_sha="$(hash_file "$root/fixtures/builtins/v1/meters/graph-taps.jsonl")"
tool_manifest_sha="$(hash_file "$root/tools/miso-engine-builtins-bench/Cargo.toml")"
tool_source_sha="$(hash_file "$root/tools/miso-engine-builtins-bench/src/main.rs")"
record_sha="$(hash_file "$record_validator")"
aggregate_sha="$(hash_file "$aggregate_validator")"
runner_sha="$(hash_file "$script_directory/run-builtins-benchmark-109.sh")"
preflight_script_sha="$(hash_file "$script_directory/preflight-builtins-benchmark-109.sh")"
lifecycle_sha="$(hash_file "$script_directory/test-builtins-benchmark-109.sh")"
checker_sha="$(hash_file "$script_directory/check-builtins-benchmark-109.sh")"
mutation_sha="$(hash_file "$script_directory/test-builtins-benchmark-109-policy.sh")"
issue068_source_sha=0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19
issue068_found=0
while IFS= read -r line; do [[ "$line" != *"$issue068_source_sha"* ]] || issue068_found=1; done \
    <"$root/.github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md"
[[ "$issue068_found" == 1 &&
   "$lock_sha" == 4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a &&
   "$manifest_sha" == bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff &&
   "$pcm_sha" == 508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19 &&
   "$meter_sha" == 958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f &&
   "$tool_manifest_sha" == f361c26b6a59c984a9fc60484748b5a2fd0bd0c35079e83ee72d3932f118cf97 &&
   "$tool_source_sha" == b520e3d14bd4fa2985d18f273e515261a53b4ea69ac1a2a38aba9bc77bf6e7fe &&
   "$record_sha" == c3db1d9574360bdab0d9ac335615787446e5537439d6accdded4fdd0a4479467 &&
   "$aggregate_sha" == 6085e740f15d7902fca4443d761cfb8e29df7168ba12f632c7946db56a3e1b63 ]] || {
    failure_reason=frozen_authority_mismatch; exit 1;
}

jq -e --arg branch "$candidate_branch" --arg commit "$candidate_commit" --arg tree "$candidate_tree" \
    --arg lock "$lock_sha" --arg manifest "$manifest_sha" --arg pcm "$pcm_sha" --arg meter "$meter_sha" \
    --arg tm "$tool_manifest_sha" --arg ts "$tool_source_sha" --arg rv "$record_sha" --arg av "$aggregate_sha" \
    --arg runner "$runner_sha" --arg preflight "$preflight_script_sha" --arg lifecycle "$lifecycle_sha" \
    --arg checker "$checker_sha" --arg mutation "$mutation_sha" \
    --arg issue068 "$issue068_source_sha" \
    'type=="object" and keys==["accepted_issue068_source_sha256","aggregate_validator_sha256","branch","candidate_commit","candidate_tree","cargo_lock_sha256","checker_sha256","fixture_manifest_sha256","graph_meter_sha256","graph_pcm_sha256","issue","issue072_artifacts","kind","lifecycle_sha256","metadata_regressions","mutation_sha256","preflight_invocations","preflight_sha256","record_validator_sha256","runner_invocations","runner_sha256","schema_version","timed_benchmark_invocations","tool_manifest_sha256","tool_source_sha256","workload_invocations"] and
     .schema_version==1 and .issue==109 and .kind=="builtins_benchmark_metadata_repair" and
     .branch==$branch and .candidate_commit==$commit and .candidate_tree==$tree and
     .cargo_lock_sha256==$lock and .fixture_manifest_sha256==$manifest and
     .graph_pcm_sha256==$pcm and .graph_meter_sha256==$meter and .tool_manifest_sha256==$tm and
     .tool_source_sha256==$ts and .record_validator_sha256==$rv and .aggregate_validator_sha256==$av and
     .runner_sha256==$runner and .preflight_sha256==$preflight and .lifecycle_sha256==$lifecycle and
     .checker_sha256==$checker and .mutation_sha256==$mutation and
     .accepted_issue068_source_sha256==$issue068 and .metadata_regressions==1 and
     .issue072_artifacts=={
       "nonbenchmark.seal.json":"7c38b068ae16055df3cfe6b817943f5fbb1a639d85597560e223d631bc37885d",
       "miso_engine_builtins_bench":"a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912",
       "builtins-benchmark.preflight.json":"f4e624b88eddbea5eb09928b544d13093d9a68be278f8afb6b70076fc8dce6bf",
       "builtins-benchmark.raw.jsonl":"c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a",
       "builtins-benchmark.jsonl":"c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a",
       "builtins-benchmark.validator.stderr":"7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396",
       "builtins-benchmark.disposition.json":"b650449d6a8944f4b00fcd833e5f775c9601a9aeb580864624a4b2c978a0698e",
       "builtins-benchmark.prelaunch.disposition.json":null} and
     .preflight_invocations==0 and .runner_invocations==0 and .workload_invocations==0 and
     .timed_benchmark_invocations==0' "$repair_seal" >/dev/null || {
    failure_reason=repair_seal_mismatch
    exit 1
}

jq -e --arg commit "$candidate_commit" --arg tree "$candidate_tree" --arg binary "$binary_sha256" \
    --arg repair "$repair_sha256" --arg lock "$lock_sha" --arg manifest "$manifest_sha" \
    --arg pcm "$pcm_sha" --arg meter "$meter_sha" --arg tm "$tool_manifest_sha" --arg ts "$tool_source_sha" \
    --arg rv "$record_sha" --arg av "$aggregate_sha" --arg runner "$runner_sha" \
    --arg preflight "$preflight_script_sha" --arg lifecycle "$lifecycle_sha" \
    --arg checker "$checker_sha" --arg mutation "$mutation_sha" \
    --arg issue068 "$issue068_source_sha" \
    'type=="object" and keys==["accepted_issue068_source_sha256","aggregate_validator_sha256","binary_sha256","candidate_commit","candidate_tree","cargo_lock_sha256","checker_sha256","fixture_manifest_sha256","graph_meter_sha256","graph_pcm_sha256","issue","kind","lifecycle_sha256","measured_rounds","mutation_sha256","preflight_invocations","preflight_sha256","record_validator_sha256","records_required","repair_seal_sha256","runner_invocations","runner_sha256","schema_version","timed_benchmark_invocations","tool_manifest_sha256","tool_source_sha256","warmup_passes","workload_invocations"] and
     .schema_version==1 and .issue==109 and .kind=="builtins_benchmark_preflight" and
     .candidate_commit==$commit and .candidate_tree==$tree and .binary_sha256==$binary and
     .repair_seal_sha256==$repair and .cargo_lock_sha256==$lock and .fixture_manifest_sha256==$manifest and
     .graph_pcm_sha256==$pcm and .graph_meter_sha256==$meter and .tool_manifest_sha256==$tm and
     .tool_source_sha256==$ts and .record_validator_sha256==$rv and .aggregate_validator_sha256==$av and
     .runner_sha256==$runner and .preflight_sha256==$preflight and .lifecycle_sha256==$lifecycle and
     .checker_sha256==$checker and .mutation_sha256==$mutation and
     .accepted_issue068_source_sha256==$issue068 and .records_required==20 and
     .warmup_passes==1 and .measured_rounds==2 and .preflight_invocations==1 and
     .runner_invocations==0 and .workload_invocations==0 and .timed_benchmark_invocations==0' \
    "$preflight_seal" >/dev/null || { failure_reason=preflight_seal_mismatch; exit 1; }

metadata_names=(CPU_MODEL CPU_ARCHITECTURE LOGICAL_CORE_COUNT PHYSICAL_CORE_COUNT OS KERNEL
    GOVERNOR_OR_POWER_MODE RUST_VERSION LLVM_VERSION TARGET_TRIPLE TARGET_FEATURES PROFILE
    OPT_LEVEL LTO CODEGEN_UNITS BACKGROUND_LOAD_NOTE)
for name in "${metadata_names[@]}"; do unset "MISO_ENGINE_BENCH_$name"; done

usable_text() {
    local value=$1 lower
    [[ "$value" =~ [^[:space:]] && ! "$value" =~ [[:cntrl:]] ]] || return 1
    [[ "$value" != [[:space:]]* && "$value" != *[[:space:]] ]] || return 1
    lower=${value,,}
    [[ "$lower" != unknown && "$lower" != default ]]
}
positive_u32() {
    [[ "$1" =~ ^[0-9]+$ && ${#1} -le 10 ]] && (( 10#$1 > 0 && 10#$1 <= 4294967295 ))
}
required_text() { usable_text "$1" || { failure_reason="invalid_$2"; exit 1; }; }

cpu_architecture="$(uname -m 2>/dev/null)" || { failure_reason=missing_cpu_architecture; exit 1; }
required_text "$cpu_architecture" cpu_architecture
os_value="$(uname -s 2>/dev/null)" || { failure_reason=missing_os; exit 1; }
required_text "$os_value" os
kernel="$(uname -r 2>/dev/null)" || { failure_reason=missing_kernel; exit 1; }
required_text "$kernel" kernel
logical_core_count="$(getconf _NPROCESSORS_ONLN 2>/dev/null)" || { failure_reason=missing_logical_core_count; exit 1; }
positive_u32 "$logical_core_count" || { failure_reason=invalid_logical_core_count; exit 1; }
rust_version="$(rustc -V 2>/dev/null)" || { failure_reason=missing_rust_version; exit 1; }
required_text "$rust_version" rust_version
rust_verbose="$scratch/rustc-vV"
rustc -vV >"$rust_verbose" 2>/dev/null || { failure_reason=missing_rust_verbose; exit 1; }
llvm_version="$(awk -F': ' '$1=="LLVM version" {if (++n>1) exit 3; value=$2} END {if(n!=1 || value=="") exit 3; print value}' "$rust_verbose")" || {
    failure_reason=invalid_llvm_version; exit 1;
}
target_triple="$(awk -F': ' '$1=="host" {if (++n>1) exit 3; value=$2} END {if(n!=1 || value=="") exit 3; print value}' "$rust_verbose")" || {
    failure_reason=invalid_target_triple; exit 1;
}
required_text "$llvm_version" llvm_version
required_text "$target_triple" target_triple
cfg="$scratch/target-cfg"
rustc --print cfg --target "$target_triple" >"$cfg" 2>/dev/null || { failure_reason=missing_target_features; exit 1; }
features="$scratch/features"
awk '/^target_feature=/ {if ($0 !~ /^target_feature="[A-Za-z0-9_.+-]+"$/) exit 3; value=$0; sub(/^target_feature="/,"",value); sub(/"$/,"",value); if(seen[value]++) exit 3; print value} END {if(length(seen)==0) exit 3}' "$cfg" >"$features" || {
    failure_reason=invalid_target_features; exit 1;
}
target_features="$(LC_ALL=C sort "$features" | awk 'BEGIN{first=1} {if(!first) printf ","; printf "%s",$0; first=0} END{printf "\n"}')"
required_text "$target_features" target_features

cpu_model= physical_core_count= governor_or_power_mode= background_load_note=
set +e
cpu_model="$(awk '/^[[:space:]]*model name[[:space:]]*:/ {sub(/^[^:]*:[[:space:]]*/,""); print; found=1; exit} END {if(!found) exit 2}' /proc/cpuinfo 2>/dev/null)"; source_status=$?
set -e
if [[ $source_status == 0 ]]; then usable_text "$cpu_model" || { failure_reason=invalid_cpu_model; exit 1; }
elif [[ $source_status != 2 ]]; then failure_reason=invalid_cpu_model; exit 1; else cpu_model=; fi

if command -v lscpu >/dev/null 2>&1; then
    set +e
    physical_core_count="$(lscpu -p=CORE,SOCKET 2>/dev/null | awk -F, '!/^#/ && NF {if($0 !~ /^[0-9]+,[0-9]+$/) bad=1; else seen[$0]=1} END {if(bad) exit 3; if(length(seen)==0) exit 2; print length(seen)}')"; source_status=$?
    set -e
    if [[ $source_status == 0 ]]; then positive_u32 "$physical_core_count" || { failure_reason=invalid_physical_core_count; exit 1; }
    elif [[ $source_status != 2 ]]; then failure_reason=invalid_physical_core_count; exit 1; else physical_core_count=; fi
fi

set +e
governor_or_power_mode="$(awk 'NF {print; found=1; exit} END {if(!found) exit 2}' /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor 2>/dev/null)"; source_status=$?
set -e
if [[ $source_status == 0 ]]; then usable_text "$governor_or_power_mode" || { failure_reason=invalid_governor_or_power_mode; exit 1; }
elif [[ $source_status != 2 ]]; then failure_reason=invalid_governor_or_power_mode; exit 1; else governor_or_power_mode=; fi

set +e
background_load_note="$(awk 'NR==1 {if(NF<3 || $1 !~ /^[0-9]+([.][0-9]+)?$/ || $2 !~ /^[0-9]+([.][0-9]+)?$/ || $3 !~ /^[0-9]+([.][0-9]+)?$/) {bad=1; exit} print $1 "," $2 "," $3 ";not-controlled"; found=1; exit} END {if(bad) exit 3; if(!found) exit 2}' /proc/loadavg 2>/dev/null)"; source_status=$?
set -e
if [[ $source_status == 0 ]]; then usable_text "$background_load_note" || { failure_reason=invalid_background_load_note; exit 1; }
elif [[ $source_status != 2 ]]; then failure_reason=invalid_background_load_note; exit 1; else background_load_note=; fi

profile=release opt_level=3 lto=false codegen_units=16
expected="$scratch/metadata.json"
jq -cnS --arg cpu_model "$cpu_model" --arg arch "$cpu_architecture" \
    --arg logical "$logical_core_count" --arg physical "$physical_core_count" --arg os "$os_value" \
    --arg kernel "$kernel" --arg governor "$governor_or_power_mode" --arg rust "$rust_version" \
    --arg llvm "$llvm_version" --arg target "$target_triple" --arg features "$target_features" \
    --arg profile "$profile" --arg opt "$opt_level" --arg lto "$lto" --arg units "$codegen_units" \
    --arg load "$background_load_note" '
    {cpu_model:(if $cpu_model=="" then null else $cpu_model end),cpu_architecture:$arch,
     logical_core_count:($logical|tonumber),physical_core_count:(if $physical=="" then null else ($physical|tonumber) end),
     os:$os,kernel:$kernel,governor_or_power_mode:(if $governor=="" then null else $governor end),
     rust_version:$rust,llvm_version:$llvm,target_triple:$target,target_features:$features,
     profile:$profile,opt_level:$opt,lto:$lto,codegen_units:$units,
     background_load_note:(if $load=="" then null else $load end)} |
    . + {missing_metadata:([to_entries[] | select(.value==null) | .key] | sort)}' >"$expected"
metadata_projection_sha256="$(hash_file "$expected")"

export MISO_ENGINE_BENCH_CPU_ARCHITECTURE="$cpu_architecture"
export MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT="$logical_core_count"
export MISO_ENGINE_BENCH_OS="$os_value" MISO_ENGINE_BENCH_KERNEL="$kernel"
export MISO_ENGINE_BENCH_RUST_VERSION="$rust_version" MISO_ENGINE_BENCH_LLVM_VERSION="$llvm_version"
export MISO_ENGINE_BENCH_TARGET_TRIPLE="$target_triple" MISO_ENGINE_BENCH_TARGET_FEATURES="$target_features"
export MISO_ENGINE_BENCH_PROFILE="$profile" MISO_ENGINE_BENCH_OPT_LEVEL="$opt_level"
export MISO_ENGINE_BENCH_LTO="$lto" MISO_ENGINE_BENCH_CODEGEN_UNITS="$codegen_units"
[[ -z "$cpu_model" ]] || export MISO_ENGINE_BENCH_CPU_MODEL="$cpu_model"
[[ -z "$physical_core_count" ]] || export MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT="$physical_core_count"
[[ -z "$governor_or_power_mode" ]] || export MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE="$governor_or_power_mode"
[[ -z "$background_load_note" ]] || export MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE="$background_load_note"

[[ "$candidate_commit" == "$(git -C "$root" rev-parse --verify HEAD)" &&
   "$candidate_tree" == "$(git -C "$root" rev-parse 'HEAD^{tree}')" &&
   -z "$(git -C "$root" status --porcelain=v1 --untracked-files=normal)" &&
   "$binary_sha256" == "$(hash_file "$binary")" && "$repair_sha256" == "$(hash_file "$repair_seal")" &&
   "$preflight_sha256" == "$(hash_file "$preflight_seal")" && "$lock_sha" == "$(hash_file "$root/Cargo.lock")" &&
   "$tool_source_sha" == "$(hash_file "$root/tools/miso-engine-builtins-bench/src/main.rs")" &&
   "$runner_sha" == "$(hash_file "$script_directory/run-builtins-benchmark-109.sh")" &&
   "$preflight_script_sha" == "$(hash_file "$script_directory/preflight-builtins-benchmark-109.sh")" &&
   "$lifecycle_sha" == "$(hash_file "$script_directory/test-builtins-benchmark-109.sh")" &&
   "$checker_sha" == "$(hash_file "$script_directory/check-builtins-benchmark-109.sh")" &&
   "$mutation_sha" == "$(hash_file "$script_directory/test-builtins-benchmark-109-policy.sh")" ]] || {
    failure_reason=authority_drift; exit 1;
}
verify_issue072 || { failure_reason=inherited_evidence_mismatch; exit 1; }

umask 077
set -o noclobber
exec {raw_fd}>"$raw"
exec {stderr_fd}>"$stderr_log"
[[ "$(stat -c %h "$raw")" == 1 && "$(stat -c %h "$stderr_log")" == 1 ]] || {
    failure_reason=artifact_link_count; exit 1;
}
failure_reason=workload_failed launch_attempted=1
set +e
MISO_ENGINE_BUILTINS_BENCH_CANDIDATE_COMMIT="$candidate_commit" \
MISO_ENGINE_BUILTINS_BENCH_BINARY_SHA256="$binary_sha256" \
"$binary" >&"$raw_fd" 2>&"$stderr_fd"
workload_status=$?
set -e
exec {raw_fd}>&-; exec {stderr_fd}>&-
refresh_progress
verify_issue072 || { failure_reason=inherited_evidence_mismatch; workload_status=1; exit 1; }
[[ "$workload_status" == 0 ]] || exit "$workload_status"
[[ "$workload_started" == 1 && "$timed_started" == 1 && "$warmup_passes" == 1 &&
   "$measured_rounds" == 2 ]] || { failure_reason=phase_mismatch; workload_status=1; exit 1; }
failure_reason=validation_failed
jq -s -e -L "$script_directory" -f "$aggregate_validator" "$raw" >/dev/null 2>>"$stderr_log" || {
    workload_status=1; exit 1;
}
jq -s -e --slurpfile expected "$expected" --arg commit "$candidate_commit" --arg binary "$binary_sha256" '
  length==20 and all(.[];
    .candidate_commit==$commit and .binary_sha256==$binary and
    {cpu_model,cpu_architecture,logical_core_count,physical_core_count,os,kernel,
     governor_or_power_mode,rust_version,llvm_version,target_triple,target_features,
     profile,opt_level,lto,codegen_units,background_load_note,missing_metadata}==$expected[0])' \
    "$raw" >/dev/null || { workload_status=1; exit 1; }

failure_reason=accepted_promotion_failed
temporary_accepted="$scratch/accepted.jsonl"
cp "$raw" "$temporary_accepted"
cmp -s "$raw" "$temporary_accepted"
[[ "$(stat -c %i "$raw")" != "$(stat -c %i "$temporary_accepted")" ]] || exit 1
[[ ! -e "$accepted" && ! -L "$accepted" ]] || exit 1
mv -n "$temporary_accepted" "$accepted"
[[ ! -e "$temporary_accepted" && -f "$accepted" && ! -L "$accepted" &&
   "$(stat -c %h "$accepted")" == 1 && "$(stat -c %i "$raw")" != "$(stat -c %i "$accepted")" ]] || exit 1
cmp -s "$raw" "$accepted"
publish_disposition "$disposition" builtins_benchmark_disposition PASS complete 0
completed=1
trap - EXIT INT TERM
rm -rf "$scratch"
printf '%s\n' "$accepted"
