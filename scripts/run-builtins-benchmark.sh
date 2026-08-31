#!/usr/bin/env bash
# Sole exactly-once Issue-072 descriptive invocation. Never rebuild, retry, or resume.
set -euo pipefail

script_directory=${BASH_SOURCE[0]%/*}
[[ "$script_directory" != "${BASH_SOURCE[0]}" ]] || script_directory=.
script_directory="$(cd "$script_directory" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
artifact_directory="$repository_root/target/issue72"
nonbenchmark_seal="$artifact_directory/nonbenchmark.seal.json"
seal="$artifact_directory/builtins-benchmark.preflight.json"
binary="$artifact_directory/bench"
raw_output="$artifact_directory/builtins-benchmark.raw.jsonl"
accepted_output="$artifact_directory/builtins-benchmark.jsonl"
stderr_output="$artifact_directory/builtins-benchmark.validator.stderr"
prelaunch_disposition="$artifact_directory/builtins-benchmark.prelaunch.disposition.json"
disposition="$artifact_directory/builtins-benchmark.disposition.json"
record_validator="$script_directory/builtins-benchmark-record-validator.jq"
aggregate_validator="$script_directory/builtins-benchmark-validator.jq"
lifecycle="$script_directory/test-builtins-benchmark.sh"
preflight_script="$script_directory/preflight-builtins-benchmark.sh"
tool_source="$repository_root/tools/bench/src/builtins.rs"
cargo_lock="$repository_root/Cargo.lock"
fixture_manifest="$repository_root/fixtures/builtins/v1/MANIFEST.tsv"
graph_pcm="$repository_root/fixtures/builtins/v1/pcm/graph-taps.f32le"
graph_meter="$repository_root/fixtures/builtins/v1/meters/graph-taps.jsonl"
issue068_spec="$repository_root/.github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md"

for tool in mkdir mktemp mv rm; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'Issue-072 runner bootstrap tool is unavailable: %s\n' "$tool" >&2
        exit 1
    }
done
mkdir -p "$artifact_directory"
[[ ! -L "$artifact_directory" ]] || { printf 'Issue-072 artifact directory is a symlink\n' >&2; exit 1; }
for terminal in "$prelaunch_disposition" "$disposition"; do
    [[ ! -e "$terminal" && ! -L "$terminal" ]] || {
        printf 'refusing consumed Issue-072 runner authority: %s\n' "$terminal" >&2
        exit 1
    }
done

scratch="$(mktemp -d "$artifact_directory/.builtins-benchmark-run.XXXXXX")"
candidate_commit= candidate_tree= binary_sha256= runner_sha256=
record_validator_sha256= aggregate_validator_sha256=
preflight_sha256= nonbenchmark_sha256=
workload_started=0 timed_started=0 warmup_passes=0 measured_rounds_completed=0
workload_status=1 failure_reason=prelaunch_failure launch_attempted=0 completed=0

hash_file() { sha256sum "$1" | awk '{print $1}'; }
json_identity() { [[ -z "$1" ]] && printf null || printf '"%s"' "$1"; }
verify_issue035() {
    local name bytes digest path
    while read -r name bytes digest; do
        path="$repository_root/target/issue35/$name"
        [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 &&
           "$(wc -c <"$path" | tr -d ' ')" == "$bytes" &&
           "$(hash_file "$path")" == "$digest" ]] || return 1
    done <<'EOF'
bench 3191104 242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944
builtins-benchmark.preflight.json 2211 85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d
builtins-benchmark.raw.jsonl 0 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
builtins-benchmark.validator.stderr 0 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
builtins-benchmark.disposition.json 974 e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce
EOF
    for name in builtins-benchmark.jsonl builtins-benchmark.prelaunch.disposition.json; do
        [[ ! -e "$repository_root/target/issue35/$name" &&
           ! -L "$repository_root/target/issue35/$name" ]] || return 1
    done
}
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
    workload_started=0 timed_started=0 warmup_passes=0 measured_rounds_completed=0
    if [[ -f "$stderr_output" && ! -L "$stderr_output" ]] && command -v awk >/dev/null 2>&1; then
        workload_started="$(awk '$0 == "MISO_ENGINE_BENCH_PHASE workload_started" {n++} END {print n + 0}' "$stderr_output")"
        timed_started="$(awk '$0 == "MISO_ENGINE_BENCH_PHASE timed_started" {n++} END {print n + 0}' "$stderr_output")"
        warmup_passes="$(awk '$0 == "MISO_ENGINE_BENCH_PHASE warmup_complete" {n++} END {print n + 0}' "$stderr_output")"
        measured_rounds_completed="$(awk '$0 == "MISO_ENGINE_BENCH_PHASE round_1_complete" || $0 == "MISO_ENGINE_BENCH_PHASE round_2_complete" {n++} END {print n + 0}' "$stderr_output")"
    fi
}
publish_disposition() {
    local destination=$1 kind=$2 status=$3 reason=$4 exit_status=$5
    local raw_identity accepted_identity stderr_identity
    local raw_sha raw_bytes accepted_sha accepted_bytes stderr_sha stderr_bytes
    raw_identity="$(artifact_identity "$raw_output")"
    accepted_identity="$(artifact_identity "$accepted_output")"
    stderr_identity="$(artifact_identity "$stderr_output")"
    read -r raw_sha raw_bytes <<<"$raw_identity"
    read -r accepted_sha accepted_bytes <<<"$accepted_identity"
    read -r stderr_sha stderr_bytes <<<"$stderr_identity"
    local tmp="$scratch/disposition.json"
    printf '{"schema_version":2,"issue":72,"kind":"%s","status":"%s","reason":"%s","preflight_invocations":1,"runner_invocations":1,"workload_invocations":%s,"warmup_passes":%s,"measured_rounds_completed":%s,"timed_benchmark_invocations":%s,"candidate_commit":%s,"candidate_tree":%s,"binary_sha256":%s,"runner_sha256":%s,"record_validator_sha256":%s,"aggregate_validator_sha256":%s,"nonbenchmark_seal_sha256":%s,"preflight_sha256":%s,"workload_exit_status":%s,"raw_sha256":%s,"raw_bytes":%s,"accepted_sha256":%s,"accepted_bytes":%s,"stderr_sha256":%s,"stderr_bytes":%s}\n' \
        "$kind" "$status" "$reason" "$workload_started" "$warmup_passes" \
        "$measured_rounds_completed" "$timed_started" \
        "$(json_identity "$candidate_commit")" "$(json_identity "$candidate_tree")" \
        "$(json_identity "$binary_sha256")" "$(json_identity "$runner_sha256")" \
        "$(json_identity "$record_validator_sha256")" \
        "$(json_identity "$aggregate_validator_sha256")" \
        "$(json_identity "$nonbenchmark_sha256")" "$(json_identity "$preflight_sha256")" \
        "$exit_status" "$raw_sha" "$raw_bytes" "$accepted_sha" "$accepted_bytes" \
        "$stderr_sha" "$stderr_bytes" >"$tmp"
    [[ ! -e "$destination" && ! -L "$destination" ]] || return 1
    mv -n -- "$tmp" "$destination"
    [[ ! -e "$tmp" && -f "$destination" && ! -L "$destination" &&
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
    rm -rf -- "$scratch"
    exit "$status"
}
on_signal() { failure_reason=workload_interrupted; workload_status=130; exit 130; }
trap on_exit EXIT
trap on_signal INT TERM

[[ "$#" == 0 ]] || {
    failure_reason=invalid_arguments
    printf 'usage: %s\n' "$0" >&2
    exit 2
}

for tool in awk cmp cp git jq sha256sum stat tr wc; do
    command -v "$tool" >/dev/null 2>&1 || {
        failure_reason=missing_tool
        printf 'required tool is unavailable: %s\n' "$tool" >&2
        exit 1
    }
done
for path in "$raw_output" "$accepted_output" "$stderr_output"; do
    [[ ! -e "$path" && ! -L "$path" ]] || {
        failure_reason=existing_output
        printf 'refusing to overwrite Issue-072 artifact: %s\n' "$path" >&2
        exit 1
    }
done
for path in "$nonbenchmark_seal" "$seal" "$binary" "$record_validator" "$aggregate_validator"; do
    [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 ]] || {
        failure_reason=missing_sealed_input
        exit 1
    }
done
[[ -x "$binary" ]] || { failure_reason=missing_sealed_input; exit 1; }
verify_issue035 || { failure_reason=inherited_evidence_mismatch; exit 1; }

for path in "$cargo_lock" "$tool_source" "$preflight_script" "$lifecycle" \
    "$fixture_manifest" "$graph_pcm" "$graph_meter" "$issue068_spec"; do
    [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 ]] || {
        failure_reason=missing_authority
        exit 1
    }
done

candidate_branch="$(git -C "$repository_root" branch --show-current)"
candidate_commit="$(git -C "$repository_root" rev-parse --verify HEAD)"
candidate_tree="$(git -C "$repository_root" rev-parse 'HEAD^{tree}')"
[[ -z "$(git -C "$repository_root" status --porcelain=v1 --untracked-files=normal)" ]] || {
    failure_reason=dirty_candidate
    exit 1
}
binary_sha256="$(hash_file "$binary")"
runner_sha256="$(hash_file "$script_directory/run-builtins-benchmark.sh")"
record_validator_sha256="$(hash_file "$record_validator")"
aggregate_validator_sha256="$(hash_file "$aggregate_validator")"
cargo_lock_sha256="$(hash_file "$cargo_lock")"
tool_source_sha256="$(hash_file "$tool_source")"
preflight_script_sha256="$(hash_file "$preflight_script")"
lifecycle_sha256="$(hash_file "$lifecycle")"
fixture_manifest_sha256="$(hash_file "$fixture_manifest")"
graph_pcm_sha256="$(hash_file "$graph_pcm")"
graph_meter_sha256="$(hash_file "$graph_meter")"
issue068_source_sha256=0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19
issue068_found=0
while IFS= read -r line; do
    [[ "$line" != *"$issue068_source_sha256"* ]] || issue068_found=1
done <"$issue068_spec"
[[ "$issue068_found" == 1 ]] || { failure_reason=accepted_source_mismatch; exit 1; }
preflight_sha256="$(hash_file "$seal")"
nonbenchmark_sha256="$(hash_file "$nonbenchmark_seal")"
jq -e \
    --arg branch "$candidate_branch" --arg commit "$candidate_commit" --arg tree "$candidate_tree" \
    --arg lock "$cargo_lock_sha256" --arg source "$tool_source_sha256" \
    --arg runner "$runner_sha256" --arg preflight "$preflight_script_sha256" \
    --arg lifecycle "$lifecycle_sha256" --arg record "$record_validator_sha256" \
    --arg aggregate "$aggregate_validator_sha256" --arg manifest "$fixture_manifest_sha256" \
    --arg pcm "$graph_pcm_sha256" --arg meter "$graph_meter_sha256" \
    --arg issue068 "$issue068_source_sha256" \
    'type == "object" and
     keys == ["accepted_issue068_source_sha256","aggregate_validator_sha256","branch","candidate_commit","candidate_tree","cargo_lock_sha256","fixture_manifest_sha256","focused_regressions","graph_meter_sha256","graph_pcm_sha256","issue","issue035_artifacts","kind","lifecycle_sha256","preflight_invocations","preflight_script_sha256","record_validator_sha256","runner_invocations","runner_sha256","schema_version","timed_benchmark_invocations","tool_source_sha256","workload_invocations"] and
     .schema_version == 2 and .issue == 72 and
     .kind == "builtins_benchmark_nonbenchmark" and .branch == $branch and
     .candidate_commit == $commit and .candidate_tree == $tree and .cargo_lock_sha256 == $lock and
     .tool_source_sha256 == $source and .runner_sha256 == $runner and
     .preflight_script_sha256 == $preflight and .lifecycle_sha256 == $lifecycle and
     .record_validator_sha256 == $record and .aggregate_validator_sha256 == $aggregate and
     .fixture_manifest_sha256 == $manifest and .graph_pcm_sha256 == $pcm and
     .graph_meter_sha256 == $meter and .accepted_issue068_source_sha256 == $issue068 and
     .issue035_artifacts == {
       "bench":"242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944",
       "builtins-benchmark.preflight.json":"85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d",
       "builtins-benchmark.raw.jsonl":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
       "builtins-benchmark.validator.stderr":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
       "builtins-benchmark.disposition.json":"e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce",
       "builtins-benchmark.jsonl":null,
       "builtins-benchmark.prelaunch.disposition.json":null} and
     .focused_regressions == 1 and .preflight_invocations == 0 and .runner_invocations == 0 and
     .workload_invocations == 0 and .timed_benchmark_invocations == 0' \
    "$nonbenchmark_seal" >/dev/null || { failure_reason=nonbenchmark_seal_mismatch; exit 1; }
jq -e \
    --arg commit "$candidate_commit" --arg tree "$candidate_tree" \
    --arg binary "$binary_sha256" --arg runner "$runner_sha256" \
    --arg record "$record_validator_sha256" --arg aggregate "$aggregate_validator_sha256" \
    --arg nonbenchmark "$nonbenchmark_sha256" --arg lock "$cargo_lock_sha256" \
    --arg source "$tool_source_sha256" --arg preflight "$preflight_script_sha256" \
    --arg lifecycle "$lifecycle_sha256" --arg manifest "$fixture_manifest_sha256" \
    --arg pcm "$graph_pcm_sha256" --arg meter "$graph_meter_sha256" \
    'type == "object" and
     keys == ["aggregate_validator_sha256","binary_sha256","candidate_commit","candidate_tree","cargo_lock_sha256","fixture_manifest_sha256","graph_meter_sha256","graph_pcm_sha256","issue","kind","lifecycle_sha256","measured_rounds","nonbenchmark_seal_sha256","preflight_invocations","preflight_script_sha256","record_validator_sha256","records_required","runner_invocations","runner_sha256","schema_version","timed_benchmark_invocations","tool_source_sha256","warmup_passes","workload_invocations"] and
     .schema_version == 2 and .issue == 72 and .kind == "builtins_benchmark_preflight" and
     .candidate_commit == $commit and .candidate_tree == $tree and
     .cargo_lock_sha256 == $lock and .tool_source_sha256 == $source and
     .binary_sha256 == $binary and .runner_sha256 == $runner and
     .preflight_script_sha256 == $preflight and .lifecycle_sha256 == $lifecycle and
     .record_validator_sha256 == $record and .aggregate_validator_sha256 == $aggregate and
     .fixture_manifest_sha256 == $manifest and .graph_pcm_sha256 == $pcm and
     .graph_meter_sha256 == $meter and
     .nonbenchmark_seal_sha256 == $nonbenchmark and .records_required == 20 and
     .warmup_passes == 1 and .measured_rounds == 2 and
     .preflight_invocations == 1 and .runner_invocations == 0 and
     .workload_invocations == 0 and .timed_benchmark_invocations == 0' \
    "$seal" >/dev/null || { failure_reason=preflight_seal_mismatch; exit 1; }

umask 077
set -o noclobber
exec {raw_fd}>"$raw_output"
exec {stderr_fd}>"$stderr_output"
[[ "$(stat -c %h "$raw_output")" == 1 && "$(stat -c %h "$stderr_output")" == 1 ]] || {
    failure_reason=artifact_link_count
    exit 1
}
failure_reason=workload_failed
launch_attempted=1
set +e
MISO_ENGINE_BENCH_CANDIDATE_COMMIT="$candidate_commit" \
MISO_ENGINE_BENCH_BINARY_SHA256="$binary_sha256" \
"$binary" builtins >&"$raw_fd" 2>&"$stderr_fd"
workload_status=$?
set -e
exec {raw_fd}>&-
exec {stderr_fd}>&-
refresh_progress
verify_issue035 || { failure_reason=inherited_evidence_mismatch; workload_status=1; exit 1; }
if [[ "$workload_status" != 0 ]]; then
    if (( workload_status >= 128 )); then failure_reason=workload_interrupted; fi
    exit "$workload_status"
fi
[[ "$workload_started" == 1 && "$timed_started" == 1 && "$warmup_passes" == 1 &&
   "$measured_rounds_completed" == 2 ]] || {
    failure_reason=phase_mismatch
    workload_status=1
    exit 1
}
failure_reason=validation_failed
if ! jq -s -e -L "$script_directory" -f "$aggregate_validator" "$raw_output" \
    >/dev/null 2>>"$stderr_output"; then
    printf 'Issue-072 aggregate validator rejected raw output\n' >>"$stderr_output"
    workload_status=1
    exit 1
fi

failure_reason=accepted_promotion_failed
temporary_accepted="$scratch/accepted.jsonl"
cp -- "$raw_output" "$temporary_accepted"
cmp -s -- "$raw_output" "$temporary_accepted"
[[ "$(stat -c %i "$raw_output")" != "$(stat -c %i "$temporary_accepted")" ]] || exit 1
[[ ! -e "$accepted_output" && ! -L "$accepted_output" ]] || exit 1
mv -n -- "$temporary_accepted" "$accepted_output"
[[ ! -e "$temporary_accepted" && -f "$accepted_output" && ! -L "$accepted_output" &&
   "$(stat -c %h "$accepted_output")" == 1 &&
   "$(stat -c %i "$raw_output")" != "$(stat -c %i "$accepted_output")" ]] || exit 1
cmp -s -- "$raw_output" "$accepted_output"

publish_disposition "$disposition" builtins_benchmark_disposition PASS complete 0
completed=1
trap - EXIT INT TERM
rm -rf -- "$scratch"
printf '%s\n' "$accepted_output"
