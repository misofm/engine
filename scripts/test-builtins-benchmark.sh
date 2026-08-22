#!/usr/bin/env bash
# Synthetic Issue-035 validator tests only. This script never launches the benchmark process.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
command -v jq >/dev/null || { printf 'jq is required for benchmark validator tests\n' >&2; exit 1; }

hash64="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
binary64="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
output64="cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
manifest64="dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
commit40="0123456789abcdef0123456789abcdef01234567"

record="$(jq -cn --arg hash "$hash64" --arg binary "$binary64" --arg output "$output64" --arg manifest "$manifest64" --arg commit "$commit40" '
  {
    schema_version:2, issue:35, workload_kind:"full_chain_filters",
    workload_id:"issue035.full_chain_filters.48000hz.q128", sample_rate_hz:48000,
    quantum_frames:128, round:1, render_scope:"render", warmup_batches:64,
    measured_batches:512, operations_per_batch:8, total_operations:4096,
    frames_per_operation:128, tracks:1, meter_observers:0, meter_queue_capacity:null,
    retained_payload_bytes:0, percentile_method:"nearest_rank", units:"ns_per_operation",
    min_ns:1, p50_ns:2, p95_ns:3, p99_ns:4, p99_9_ns:5, max_ns:6,
    descriptive_only:true, candidate_commit:$commit, binary_sha256:$binary,
    fixture_manifest_id:"fixtures/builtins/v1/MANIFEST.tsv", fixture_manifest_sha256:$manifest,
    input_fixture_id:"fixtures/builtins/v1/benchmark/full_chain_filters-48000.toml",
    input_fixture_sha256:$hash, output_sha256:$output,
    render_errors:0, render_allocations:0, render_deallocations:0, render_locks:0,
    render_logs:0, render_file_io:0, render_network_io:0, render_syscalls:0,
    render_feature_detection:0, render_panic_unwind:0, render_total_forbidden_operations:0,
    cpu_model:null, cpu_architecture:null, logical_core_count:null, physical_core_count:null,
    os:null, kernel:null, governor_or_power_mode:null, rust_version:null, llvm_version:null,
    target_triple:null, target_features:null, profile:null, opt_level:null, lto:null,
    codegen_units:null, background_load_note:null,
    missing_metadata:[
      "background_load_note","codegen_units","cpu_architecture","cpu_model",
      "governor_or_power_mode","kernel","llvm_version","logical_core_count","lto",
      "opt_level","os","physical_core_count","profile","rust_version","target_features",
      "target_triple"
    ]
  }'
)"

record_valid() {
  jq -e -L "$script_directory" \
    'include "builtins-benchmark-record-validator"; builtins_benchmark_record_valid' >/dev/null
}
aggregate_valid() {
  jq -e -L "$script_directory" -f "$script_directory/builtins-benchmark-validator.jq" >/dev/null
}
reject_record_mutation() {
  local mutation=$1
  if jq "$mutation" <<<"$record" | record_valid; then
    printf 'record validator accepted mutation: %s\n' "$mutation" >&2
    exit 1
  fi
}
reject_aggregate_mutation() {
  local mutation=$1
  if jq "$mutation" <<<"$records" | aggregate_valid; then
    printf 'aggregate validator accepted mutation: %s\n' "$mutation" >&2
    exit 1
  fi
}

record_valid <<<"$record"
while IFS= read -r field; do
  if jq --arg field "$field" 'del(.[$field])' <<<"$record" | record_valid; then
    printf 'record validator accepted missing field: %s\n' "$field" >&2
    exit 1
  fi
  if jq --arg field "$field" '.[$field] = []' <<<"$record" | record_valid; then
    printf 'record validator accepted wrong field type: %s\n' "$field" >&2
    exit 1
  fi
done < <(jq -r 'keys[]' <<<"$record")
if jq '.unexpected = 0' <<<"$record" | record_valid; then
  printf 'record validator accepted an extra key\n' >&2
  exit 1
fi

for mutation in \
  '.issue = 7' \
  '.workload_id = "issue035.identity_chain.48000hz.q128"' \
  '.sample_rate_hz = 44100' \
  '.round = 3' \
  '.total_operations = 512' \
  '.frames_per_operation = null' \
  '.meter_queue_capacity = 1' \
  '.p50_ns = 0' \
  '.descriptive_only = false' \
  '.candidate_commit = "bad"' \
  '.binary_sha256 = "bad"' \
  '.fixture_manifest_id = "fixtures/builtins/v1/benchmark/full_chain_filters-48000.toml"' \
  '.input_fixture_id = "fixtures/builtins/v1/MANIFEST.tsv"' \
  '.output_sha256 = "bad"' \
  '.render_errors = 1' \
  '.render_feature_detection = 1' \
  '.render_total_forbidden_operations = 1' \
  '.cpu_model = "unknown"' \
  '.logical_core_count = "1"' \
  '.missing_metadata |= .[1:]'; do
  reject_record_mutation "$mutation"
done

preparation="$(jq '
  .workload_kind="prepare_256_tracks" |
  .workload_id="issue035.prepare_256_tracks.48000hz.q128" |
  .render_scope="not_applicable_preparation" |
  .warmup_batches=16 | .measured_batches=128 | .operations_per_batch=1 |
  .total_operations=128 | .frames_per_operation=null |
  .tracks=256 | .meter_observers=56 | .meter_queue_capacity=4 |
  .input_fixture_id="fixtures/builtins/v1/benchmark/prepare_256_tracks-48000.toml" |
  .render_errors="not_applicable" | .render_allocations="not_applicable" |
  .render_deallocations="not_applicable" | .render_locks="not_applicable" |
  .render_logs="not_applicable" | .render_file_io="not_applicable" |
  .render_network_io="not_applicable" | .render_syscalls="not_applicable" |
  .render_feature_detection="not_applicable" | .render_panic_unwind="not_applicable" |
  .render_total_forbidden_operations="not_applicable"
' <<<"$record")"
record_valid <<<"$preparation"
if jq '.meter_observers=14' <<<"$preparation" | record_valid; then
  printf 'record validator accepted invalid preparation shape\n' >&2
  exit 1
fi

records="$(jq -cn --argjson base "$record" '
  ["full_chain_filters","identity_chain","matrix_ramp","meter_success_full","prepare_256_tracks"] as $kinds |
  [48000,96000] as $rates | [1,2] as $rounds |
  [$kinds[] as $kind | $rates[] as $rate | $rounds[] as $round |
    $base |
    .workload_kind=$kind |
    .workload_id=("issue035." + $kind + "." + ($rate | tostring) + "hz.q128") |
    .sample_rate_hz=$rate | .round=$round |
    .input_fixture_id=("fixtures/builtins/v1/benchmark/" + $kind + "-" + ($rate | tostring) + ".toml") |
    if $kind == "prepare_256_tracks" then
      .render_scope="not_applicable_preparation" |
      .warmup_batches=16 | .measured_batches=128 | .operations_per_batch=1 |
      .total_operations=128 | .frames_per_operation=null | .tracks=256 |
      .meter_observers=56 | .meter_queue_capacity=4 |
      .render_errors="not_applicable" | .render_allocations="not_applicable" |
      .render_deallocations="not_applicable" | .render_locks="not_applicable" |
      .render_logs="not_applicable" | .render_file_io="not_applicable" |
      .render_network_io="not_applicable" | .render_syscalls="not_applicable" |
      .render_feature_detection="not_applicable" | .render_panic_unwind="not_applicable" |
      .render_total_forbidden_operations="not_applicable"
    elif $kind == "meter_success_full" then
      .meter_observers=14 | .meter_queue_capacity=1
    else
      .meter_observers=0 | .meter_queue_capacity=null
    end
  ]'
)"
aggregate_valid <<<"$records"
for mutation in \
  'del(.[0])' \
  '.[20] = .[0]' \
  '.[1].round = 1' \
  '.[1].candidate_commit = "1111111111111111111111111111111111111111"' \
  '.[1].binary_sha256 = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"' \
  '.[1].fixture_manifest_sha256 = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"' \
  '.[1].input_fixture_sha256 = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"' \
  '.[1].output_sha256 = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"'; do
  reject_aggregate_mutation "$mutation"
done

printf 'builtins benchmark validators: PASS (runner/workload/timing invocations: 0/0/0)\n'
