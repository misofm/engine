#!/usr/bin/env bash
# Synthetic Issue-035 validator tests only. This script never launches the benchmark process.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
command -v jq >/dev/null || { printf 'jq is required for benchmark validator tests\n' >&2; exit 1; }

hash64="4e5e2c9fc8e2c2400b816715273879f3635f2374133e5775ade18dabee1f6ad9"
binary64="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
output64="cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
manifest64="bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff"
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
  '.fixture_manifest_sha256 = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"' \
  '.input_fixture_id = "fixtures/builtins/v1/MANIFEST.tsv"' \
  '.input_fixture_sha256 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"' \
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
  .input_fixture_sha256="0c2130e5f3563e011cc7251a4a42d27b2a84f5871a81facae49be0a5c1cf21ff" |
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

records="$(jq -cn -L "$script_directory" --argjson base "$record" '
  include "builtins-benchmark-record-validator";
  ["full_chain_filters","identity_chain","matrix_ramp","meter_success_full","prepare_256_tracks"] as $kinds |
  [48000,96000] as $rates | [1,2] as $rounds |
  [$kinds[] as $kind | $rates[] as $rate | $rounds[] as $round |
    $base |
    .workload_kind=$kind |
    .workload_id=("issue035." + $kind + "." + ($rate | tostring) + "hz.q128") |
    .sample_rate_hz=$rate | .round=$round |
    .input_fixture_id=("fixtures/builtins/v1/benchmark/" + $kind + "-" + ($rate | tostring) + ".toml") |
    .input_fixture_sha256=({workload_kind:$kind,sample_rate_hz:$rate} | frozen_input_sha256) |
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

lifecycle_runner="$script_directory/run-builtins-benchmark.sh"
lifecycle_scratch="$(mktemp -d)"
trap 'rm -rf -- "$lifecycle_scratch"' EXIT
lifecycle_template="$lifecycle_scratch/template"
mkdir -p "$lifecycle_template/scripts" "$lifecycle_template/bin" "$lifecycle_template/target/issue35"
cp "$lifecycle_runner" "$script_directory/builtins-benchmark-record-validator.jq" \
  "$script_directory/builtins-benchmark-validator.jq" "$lifecycle_template/scripts/"
printf '%s\n' "$records" | jq -c '.[]' >"$lifecycle_template/records.jsonl"
cat >"$lifecycle_template/bin/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$*" in
  *rev-parse*) printf '%s\n' "$MISO_TEST_CANDIDATE" ;;
  *status*) exit 0 ;;
  *) exit 91 ;;
esac
EOF
cat >"$lifecycle_template/target/issue35/miso_engine_builtins_bench" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
printf 'sealed-binary\n' >>"$MISO_TEST_LAUNCH_LOG"
case "${MISO_TEST_MODE:?}" in
  success) cat "$MISO_TEST_RECORDS" ;;
  workload_failure) printf '{"partial":"workload"}\n'; exit 73 ;;
  round_one_failure) jq -c 'select(.round == 1)' "$MISO_TEST_RECORDS"; exit 73 ;;
  interrupted_partial) printf '{"partial":"interrupted"}\n'; kill -TERM "$BASHPID" ;;
  validator_failure) cat "$MISO_TEST_RECORDS"; printf '{}\n' ;;
  *) exit 91 ;;
esac
EOF
chmod 755 "$lifecycle_template/bin/git" "$lifecycle_template/target/issue35/miso_engine_builtins_bench"

lifecycle_case=0
new_lifecycle_case() {
  lifecycle_case=$((lifecycle_case + 1))
  case_root="$lifecycle_scratch/case-$lifecycle_case-$1"
  mkdir "$case_root"
  cp -a "$lifecycle_template/." "$case_root/"
  launch_log="$case_root/launch.log"
  seal="$case_root/target/issue35/builtins-benchmark.preflight.json"
  raw="$case_root/target/issue35/builtins-benchmark.raw.jsonl"
  accepted="$case_root/target/issue35/builtins-benchmark.jsonl"
  stderr_log="$case_root/target/issue35/builtins-benchmark.validator.stderr"
  disposition="$case_root/target/issue35/builtins-benchmark.disposition.json"
  candidate="$commit40"
  runner_sha="$(sha256sum "$case_root/scripts/run-builtins-benchmark.sh" | awk '{print $1}')"
  record_sha="$(sha256sum "$case_root/scripts/builtins-benchmark-record-validator.jq" | awk '{print $1}')"
  aggregate_sha="$(sha256sum "$case_root/scripts/builtins-benchmark-validator.jq" | awk '{print $1}')"
  binary_sha="$(sha256sum "$case_root/target/issue35/miso_engine_builtins_bench" | awk '{print $1}')"
  jq -cn --arg candidate "$candidate" --arg binary "$binary_sha" --arg runner "$runner_sha" \
    --arg record "$record_sha" --arg aggregate "$aggregate_sha" \
    '{schema_version:2,issue:58,kind:"builtins_benchmark_preflight",
      candidate_commit:$candidate,binary_sha256:$binary,runner_sha256:$runner,
      record_validator_sha256:$record,aggregate_validator_sha256:$aggregate,
      runner_invocations:0,workload_invocations:0,timed_benchmark_invocations:0}' >"$seal"
}
run_lifecycle_runner() {
  local mode=$1
  shift
  MISO_TEST_MODE="$mode" MISO_TEST_CANDIDATE="$candidate" \
    MISO_TEST_LAUNCH_LOG="$launch_log" MISO_TEST_RECORDS="$case_root/records.jsonl" \
    PATH="$case_root/bin:$PATH" bash "$case_root/scripts/run-builtins-benchmark.sh" "$@"
}
expect_no_scratch_launch() {
  [[ ! -e "$launch_log" ]]
}
expect_no_accepted() {
  [[ ! -e "$accepted" && ! -L "$accepted" ]]
}

new_lifecycle_case argument
if run_lifecycle_runner success --retry >/dev/null 2>&1; then
  printf 'sealed runner accepted an argument\n' >&2
  exit 1
fi
expect_no_scratch_launch

new_lifecycle_case missing-tool
missing_tool_bin="$case_root/missing-tool-bin"
mkdir "$missing_tool_bin"
ln -s "$(command -v bash)" "$missing_tool_bin/bash"
ln -s "$(command -v dirname)" "$missing_tool_bin/dirname"
set +e
PATH="$missing_tool_bin" "$missing_tool_bin/bash" \
  "$case_root/scripts/run-builtins-benchmark.sh" >"$case_root/result" 2>&1
status=$?
set -e
[[ "$status" == 1 ]]
grep -Fq 'required tool is unavailable: jq' "$case_root/result"
expect_no_scratch_launch

new_lifecycle_case success
if ! published="$(run_lifecycle_runner success 2>"$case_root/result")"; then
  cat "$case_root/result" >&2
  [[ ! -f "$disposition" ]] || cat "$disposition" >&2
  [[ ! -f "$stderr_log" ]] || cat "$stderr_log" >&2
  exit 1
fi
[[ "$published" == "$accepted" ]] || {
  printf 'unexpected sealed-runner stdout: %s\n' "$published" >&2
  exit 1
}
cmp -s "$raw" "$accepted"
[[ "$(wc -l <"$launch_log")" == 1 ]]
jq -e '.status == "PASS" and .reason == "complete" and
       .runner_invocations == 1 and .workload_invocations == 1 and
       .warmup_passes == 1 and .measured_rounds_completed == 2 and
       .timed_benchmark_invocations == 1 and .raw_sha256 == .accepted_sha256 and
       .raw_bytes == .accepted_bytes and .workload_exit_status == 0' "$disposition" >/dev/null

new_lifecycle_case workload-failure
set +e
run_lifecycle_runner workload_failure >"$case_root/result" 2>&1
status=$?
set -e
[[ "$status" == 73 ]]
grep -Fqx '{"partial":"workload"}' "$raw"
expect_no_accepted
[[ "$(wc -l <"$launch_log")" == 1 ]]
jq -e '.status == "FAIL" and .reason == "workload_failed" and .workload_exit_status == 73 and
       .workload_invocations == 1 and .timed_benchmark_invocations == 1 and
       .warmup_passes == 0 and .measured_rounds_completed == 0 and
       .raw_sha256 != null and .accepted_sha256 == null' "$disposition" >/dev/null

new_lifecycle_case round-one-failure
set +e
run_lifecycle_runner round_one_failure >"$case_root/result" 2>&1
status=$?
set -e
[[ "$status" == 73 ]]
[[ "$(wc -l <"$raw")" == 10 ]]
expect_no_accepted
jq -e '.status == "FAIL" and .reason == "workload_failed" and .workload_exit_status == 73 and
       .workload_invocations == 1 and .timed_benchmark_invocations == 1 and
       .warmup_passes == 1 and .measured_rounds_completed == 1' "$disposition" >/dev/null

new_lifecycle_case interrupted-partial
set +e
run_lifecycle_runner interrupted_partial >"$case_root/result" 2>&1
status=$?
set -e
[[ "$status" == 143 ]]
grep -Fqx '{"partial":"interrupted"}' "$raw"
expect_no_accepted
jq -e '.status == "FAIL" and .reason == "workload_interrupted" and .workload_exit_status == 143' \
  "$disposition" >/dev/null
jq -e '.warmup_passes == 0 and .measured_rounds_completed == 0 and
       .workload_invocations == 1 and .timed_benchmark_invocations == 1' \
  "$disposition" >/dev/null

new_lifecycle_case validator-failure
set +e
run_lifecycle_runner validator_failure >"$case_root/result" 2>&1
status=$?
set -e
[[ "$status" == 1 ]]
grep -Fqx '{}' "$raw"
expect_no_accepted
[[ -s "$stderr_log" ]]
jq -e '.status == "FAIL" and .reason == "validation_failed" and .workload_exit_status == 1 and
       .warmup_passes == 1 and .measured_rounds_completed == 2 and
       .workload_invocations == 1 and .timed_benchmark_invocations == 1' \
  "$disposition" >/dev/null

for artifact in raw accepted stderr disposition; do
  new_lifecycle_case "existing-$artifact"
  case "$artifact" in
    raw) protected="$raw" ;;
    accepted) protected="$accepted" ;;
    stderr) protected="$stderr_log" ;;
    disposition) protected="$disposition" ;;
  esac
  printf 'protected\n' >"$protected"
  if run_lifecycle_runner success >/dev/null 2>&1; then
    printf 'sealed runner accepted an existing %s artifact\n' "$artifact" >&2
    exit 1
  fi
  expect_no_scratch_launch
  [[ "$(<"$protected")" == protected ]]
done

new_lifecycle_case symlink-artifact
ln -s "$case_root/records.jsonl" "$raw"
if run_lifecycle_runner success >/dev/null 2>&1; then exit 1; fi
expect_no_scratch_launch

new_lifecycle_case hard-link-alias
ln "$case_root/records.jsonl" "$accepted"
if run_lifecycle_runner success >/dev/null 2>&1; then exit 1; fi
expect_no_scratch_launch

new_lifecycle_case seal-mismatch
jq '.candidate_commit = "1111111111111111111111111111111111111111"' "$seal" >"$case_root/seal-mutated"
mv "$case_root/seal-mutated" "$seal"
if run_lifecycle_runner success >/dev/null 2>&1; then exit 1; fi
expect_no_scratch_launch

preflight_template="$lifecycle_scratch/preflight-template"
mkdir -p "$preflight_template"/{scripts,bin,crates,tools/miso-engine-builtins-bench/src,fixtures/builtins/v1/{pcm,meters},.github/ISSUE_SPECS}
cp "$script_directory/preflight-builtins-benchmark.sh" "$preflight_template/scripts/"
for script in run-builtins-benchmark.sh builtins-benchmark-record-validator.jq \
  builtins-benchmark-validator.jq; do
  printf 'sealed-tool\n' >"$preflight_template/scripts/$script"
done
for script in test-builtins-benchmark.sh check-builtins-fixtures.sh \
  check-workspace-policy.sh check-realtime-policy.sh check-builtins-policy.sh \
  check-graph-policy.sh check-rack-policy.sh check-builtins-targets.sh \
  check-rack-instructions.sh check-builtins-target-instructions.sh; do
  printf '#!/usr/bin/env bash\nexit 0\n' >"$preflight_template/scripts/$script"
done
printf '[workspace]\n' >"$preflight_template/Cargo.toml"
printf 'candidate-lock\n' >"$preflight_template/Cargo.lock"
printf '[package]\nname="bench"\n' >"$preflight_template/tools/miso-engine-builtins-bench/Cargo.toml"
printf 'fn main() {}\n' >"$preflight_template/tools/miso-engine-builtins-bench/src/main.rs"
for crate in miso-engine-core miso-engine-builtins miso-engine-builtins-compiler \
  miso-engine-graph miso-engine-graph-compiler; do
  mkdir -p "$preflight_template/crates/$crate/src"
  printf '[package]\nname="%s"\n' "$crate" >"$preflight_template/crates/$crate/Cargo.toml"
  printf 'pub fn marker() {}\n' >"$preflight_template/crates/$crate/src/lib.rs"
done
printf 'manifest\n' >"$preflight_template/fixtures/builtins/v1/MANIFEST.tsv"
printf 'pcm\n' >"$preflight_template/fixtures/builtins/v1/pcm/graph-taps.f32le"
printf 'meter\n' >"$preflight_template/fixtures/builtins/v1/meters/graph-taps.jsonl"
cat >"$preflight_template/.github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md" <<'EOF'
0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19
EOF
cat >"$preflight_template/.github/ISSUE_SPECS/070-quiescent-builtin-graph-retirement-worker-trace-closure.md" <<'EOF'
3581ebf058151a0a0014ff08adcdd7fcd6fe6ad51a5baf41538272d4bba6ce8e
54103c89b557a72da9c79cd00a636ea64933240a4dcb27c27647fb960b013db4
812e7c62cf8963fba1cb6f32615005ec8bd7df6b97f6c72a0c4960fadcf0d4c1
1c98d033c0c5d156dea887a829cc683d460145c08856c705fdbde7ef8b4324c5
EOF

real_sha256sum="$(command -v sha256sum)"
cat >"$preflight_template/bin/git" <<'EOF'
#!/usr/bin/env bash
case "$1" in
  rev-parse) printf '%s\n' '0123456789abcdef0123456789abcdef01234567' ;;
  status) ;;
  show) printf 'base-lock\n' ;;
  cat-file) printf '10\n' ;;
  diff) printf 'lock-diff\n' ;;
  *) exit 91 ;;
esac
EOF
cat >"$preflight_template/bin/sha256sum" <<'EOF'
#!/usr/bin/env bash
if [[ "$#" == 0 ]]; then
  input="$(cat)"
  case "$input" in
    base-lock) hash=96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424 ;;
    lock-diff) hash=5ebc70f8a35208d50ff4d9afd92602462180b345125263a0a4916aa3bb08940e ;;
    *) hash="$(printf '%s' "$input" | "$MISO_TEST_REAL_SHA256SUM" | awk '{print $1}')" ;;
  esac
  printf '%s  -\n' "$hash"
  exit
fi
case "$1" in
  *fixtures/builtins/v1/MANIFEST.tsv)
    hash=bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff ;;
  *fixtures/builtins/v1/pcm/graph-taps.f32le)
    hash=508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19 ;;
  *fixtures/builtins/v1/meters/graph-taps.jsonl)
    hash=958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f ;;
  */issue068-source-manifest.tsv|*/issue068-source-after.tsv)
    hash=0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19 ;;
  *) exec "$MISO_TEST_REAL_SHA256SUM" "$@" ;;
esac
printf '%s  %s\n' "$hash" "$1"
EOF
cat >"$preflight_template/bin/cargo" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' "$*" >>"$MISO_TEST_PREFLIGHT_CARGO_LOG"
if [[ " $* " == *' build '* ]]; then
  mkdir -p "$CARGO_TARGET_DIR/release"
  cat >"$CARGO_TARGET_DIR/release/miso_engine_builtins_bench" <<'INNER'
#!/usr/bin/env bash
printf 'launched\n' >>"$MISO_TEST_PREFLIGHT_LAUNCH_LOG"
INNER
  chmod 755 "$CARGO_TARGET_DIR/release/miso_engine_builtins_bench"
  if [[ "${MISO_TEST_PREFLIGHT_DRIFT:-0}" == 1 ]]; then
    printf 'drift\n' >>"$MISO_TEST_PREFLIGHT_ROOT/Cargo.toml"
  fi
fi
EOF
chmod 755 "$preflight_template/bin/"{git,sha256sum,cargo}

for tool in bash jq wc cmp cp chmod find rg awk mktemp mv sort tr mkdir rm dirname cat; do
  ln -s "$(command -v "$tool")" "$preflight_template/bin/$tool"
done
run_scratch_preflight() {
  local mode=$1 root="$lifecycle_scratch/preflight-$1"
  cp -a "$preflight_template" "$root"
  : >"$root/cargo.log"
  MISO_TEST_REAL_SHA256SUM="$real_sha256sum" \
  MISO_TEST_PREFLIGHT_CARGO_LOG="$root/cargo.log" \
  MISO_TEST_PREFLIGHT_LAUNCH_LOG="$root/launch.log" \
  MISO_TEST_PREFLIGHT_ROOT="$root" \
  MISO_TEST_PREFLIGHT_DRIFT="${2:-0}" \
  PATH="$root/bin" "$root/bin/bash" "$root/scripts/preflight-builtins-benchmark.sh"
}

run_scratch_preflight success >"$lifecycle_scratch/preflight-success.log"
[[ ! -e "$lifecycle_scratch/preflight-success/launch.log" ]]
[[ "$(grep -c '^build ' "$lifecycle_scratch/preflight-success/cargo.log")" == 1 ]]
jq -e '.runner_invocations == 0 and .workload_invocations == 0 and
       .timed_benchmark_invocations == 0 and .records_required == 20 and
       .warmup_passes == 1 and .measured_rounds == 2' \
  "$lifecycle_scratch/preflight-success/target/issue35/builtins-benchmark.preflight.json" >/dev/null

set +e
run_scratch_preflight drift 1 >"$lifecycle_scratch/preflight-drift.log" 2>&1
status=$?
set -e
[[ "$status" == 1 ]]
grep -Fq 'candidate source changed during preflight' "$lifecycle_scratch/preflight-drift.log"
[[ ! -e "$lifecycle_scratch/preflight-drift/target/issue35/miso_engine_builtins_bench" ]]
[[ ! -e "$lifecycle_scratch/preflight-drift/target/issue35/builtins-benchmark.preflight.json" ]]
[[ ! -e "$lifecycle_scratch/preflight-drift/launch.log" ]]

preflight_existing="$lifecycle_scratch/preflight-existing"
cp -a "$preflight_template" "$preflight_existing"
mkdir -p "$preflight_existing/target/issue35"
printf 'protected\n' >"$preflight_existing/target/issue35/builtins-benchmark.raw.jsonl"
: >"$preflight_existing/cargo.log"
set +e
MISO_TEST_REAL_SHA256SUM="$real_sha256sum" \
MISO_TEST_PREFLIGHT_CARGO_LOG="$preflight_existing/cargo.log" \
MISO_TEST_PREFLIGHT_LAUNCH_LOG="$preflight_existing/launch.log" \
MISO_TEST_PREFLIGHT_ROOT="$preflight_existing" \
PATH="$preflight_existing/bin" "$preflight_existing/bin/bash" \
  "$preflight_existing/scripts/preflight-builtins-benchmark.sh" \
  >"$preflight_existing/result" 2>&1
status=$?
set -e
[[ "$status" == 1 ]]
grep -Fq 'refusing to overwrite Issue-035 artifact' "$preflight_existing/result"
[[ "$(<"$preflight_existing/target/issue35/builtins-benchmark.raw.jsonl")" == protected ]]
[[ ! -s "$preflight_existing/cargo.log" ]]
[[ ! -e "$preflight_existing/launch.log" ]]

printf 'builtins benchmark validators/runner/preflight lifecycle: PASS (real runner/workload/timing invocations: 0/0/0; scratch stubs only)\n'
