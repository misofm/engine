#!/usr/bin/env bash
# Synthetic Issue-431 current validator tests only. This script never launches the benchmark process.
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
command -v jq >/dev/null || { printf 'jq is required for benchmark validator tests\n' >&2; exit 1; }

hash64="6a1633442678cfdecb2872deacd053e727c47f0bc94039a84b4e950949e195d0"
binary64="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
output64="cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
manifest64="b244da45d88d670951205098b7516af20387a141eccb3bf60edb61e8ba57a919"
commit40="0123456789abcdef0123456789abcdef01234567"

record="$(jq -cn --arg hash "$hash64" --arg binary "$binary64" --arg output "$output64" --arg manifest "$manifest64" --arg commit "$commit40" '
  {
    schema_version:1, issue:35, workload_kind:"full_chain_filters",
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
    'include "builtins-current-benchmark-record-validator"; builtins_benchmark_record_valid' >/dev/null
}
aggregate_valid() {
  jq -e -L "$script_directory" -f "$script_directory/builtins-current-benchmark-validator.jq" >/dev/null
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
  .input_fixture_sha256="a1dec8525c20505a9b440e6cf93fa6ffa1144896c889fa3abd94f76224f3e210" |
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
  include "builtins-current-benchmark-record-validator";
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


# Exact validator deltas: seven record hashes and one aggregate include line only.
delta_scratch=$(mktemp -d)
trap 'rm -rf -- "$delta_scratch"' EXIT
sed \
  -e 's/ddb4b201dcd4cc00ad445013c9a1b29d9d5f6071f018e649748963c74af4c55b/b244da45d88d670951205098b7516af20387a141eccb3bf60edb61e8ba57a919/' \
  -e 's/4e5e2c9fc8e2c2400b816715273879f3635f2374133e5775ade18dabee1f6ad9/6a1633442678cfdecb2872deacd053e727c47f0bc94039a84b4e950949e195d0/' \
  -e 's/cc4f23f6579cc255a1282797de2b78c93951f947c7b0ab72fa2ca713780f8a1e/ac9e825b5051a161ca731b04bd9b9b825bad6484c3a3f911551051e316224fa0/' \
  -e 's/65232ba5a59f54a22762a6ebc82620be6332f9d583c0e61fe4c5d82ede23e7ac/15dfc8b6d918d01a5d6e46417e37a10023d31a85391e8fb2371af0cdc055dd95/' \
  -e 's/9bc765fb84d94dd31f83137e2aa091fd09a28a8dab8fbe1d18a0b4a9a60c85a7/962bc24d4104cb5a30e3a5aa158a5ca1075cae01f08433d2c7cbe8c1271cd99a/' \
  -e 's/0c2130e5f3563e011cc7251a4a42d27b2a84f5871a81facae49be0a5c1cf21ff/a1dec8525c20505a9b440e6cf93fa6ffa1144896c889fa3abd94f76224f3e210/' \
  -e 's/5ca5e3b6e0080b66c53f0a12753e3681ea1caf6571ff3747e2303ac8cf0779a6/880faace46cfa2e9f454d625e54206aa752a9947292057a6b58f64224ea13f30/' \
  "$script_directory/builtins-benchmark-record-validator.jq" >"$delta_scratch/record.expected"
cmp -s "$delta_scratch/record.expected" "$script_directory/builtins-current-benchmark-record-validator.jq" ||
  { printf 'current record validator exceeds seven allowed hash deltas\n' >&2; exit 1; }
sed 's/include "builtins-benchmark-record-validator";/include "builtins-current-benchmark-record-validator";/' \
  "$script_directory/builtins-benchmark-validator.jq" >"$delta_scratch/aggregate.expected"
cmp -s "$delta_scratch/aggregate.expected" "$script_directory/builtins-current-benchmark-validator.jq" ||
  { printf 'current aggregate validator exceeds include delta\n' >&2; exit 1; }

lifecycle_scratch="$delta_scratch/lifecycle"
template="$lifecycle_scratch/template"
mkdir -p "$template/scripts" "$template/bin" "$template/tools/bench/src" \
  "$template/fixtures/builtins/v1" "$template/fixtures/session/v1" "$template/.cargo"
cp "$script_directory/preflight-builtins-current-benchmark.sh" \
  "$script_directory/run-builtins-current-benchmark.sh" \
  "$script_directory/test-builtins-current-benchmark.sh" \
  "$script_directory/builtins-current-benchmark-record-validator.jq" \
  "$script_directory/builtins-current-benchmark-validator.jq" \
  "$script_directory/check-bench-preconditions.sh" "$template/scripts/"
cp "$repository_root/tools/bench/src/builtins.rs" "$template/tools/bench/src/"
cp "$repository_root/Cargo.lock" "$template/"
cp "$repository_root/.cargo/config.toml" "$template/.cargo/"
cp -a "$repository_root/fixtures/builtins/v1/." "$template/fixtures/builtins/v1/"
cp "$repository_root/fixtures/session/v1/canonical.json" "$template/fixtures/session/v1/"
printf '[workspace]\nmembers=[]\n[profile.release]\nopt-level=3\nlto="fat"\ncodegen-units=1\n' >"$template/Cargo.toml"
printf '%s\n' "$records" | jq -c '.[]' >"$template/synthetic-records.jsonl"

cat >"$template/bin/git" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in -C) shift 2 ;; esac
case "${1:-}" in
  rev-parse) [[ "${2:-}" == 'HEAD^{tree}' ]] && printf '%s\n' aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa || printf '%s\n' 0123456789abcdef0123456789abcdef01234567 ;;
  status) exit 0 ;;
  *) exit 90 ;;
esac
EOF
cat >"$template/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
[[ "${1:-}" == build && "${2:-}" == --locked && "${3:-}" == --release && "${4:-}" == -p && "${5:-}" == bench ]] || exit 90
mkdir -p "$CARGO_TARGET_DIR/release"
cp synthetic-emitter.sh "$CARGO_TARGET_DIR/release/bench"
chmod 755 "$CARGO_TARGET_DIR/release/bench"
EOF
cat >"$template/bin/taskset" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
[[ "${1:-}" == -c ]] || exit 90
shift 2
exec "$@"
EOF
cat >"$template/bin/sleep" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF
cat >"$template/bin/cat" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
case "${1:-}" in
  /sys/devices/system/cpu/online) printf '0-1\n' ;;
  /sys/devices/system/cpu/cpu1/topology/thread_siblings_list) printf '1\n' ;;
  /proc/loadavg) [[ "$(<"$root/control-mode")" == refuse ]] && printf '0.51 0 0 1/1 1\n' || printf '0.01 0 0 1/1 1\n' ;;
  /proc/stat) printf 'cpu1 0 0 0 100 0 0 0 0 0 0\n' ;;
  *) exec "$(<"$root/real-cat")" "$@" ;;
esac
EOF
cat >"$template/synthetic-emitter.sh" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
printf 'launch\n' >>"$root/synthetic-launches"
mode=$(<"$root/synthetic-mode")
records="$root/synthetic-records.jsonl"
emit_records() {
  jq -c --arg commit "$MISO_ENGINE_BENCH_CANDIDATE_COMMIT" --arg binary "$MISO_ENGINE_BENCH_BINARY_SHA256" \
    '.candidate_commit=$commit|.binary_sha256=$binary' "$records"
}
printf 'MISO_ENGINE_BENCH_PHASE workload_started\n' >&2
[[ "$mode" != warmup_fail ]] || { printf '{"partial":"warmup"}\n'; exit 71; }
printf 'MISO_ENGINE_BENCH_PHASE warmup_complete\nMISO_ENGINE_BENCH_PHASE timed_started\n' >&2
[[ "$mode" != round1_fail ]] || { head -n 5 "$records"; exit 72; }
printf 'MISO_ENGINE_BENCH_PHASE round_1_complete\n' >&2
[[ "$mode" != round2_fail ]] || { head -n 15 "$records"; exit 73; }
printf 'MISO_ENGINE_BENCH_PHASE round_2_complete\n' >&2
case "$mode" in
  success) emit_records ;;
  bad_record) emit_records | jq -c 'if .workload_kind=="full_chain_filters" and .sample_rate_hz==48000 and .round==1 then .render_allocations=1 else . end' ;;
  bad_aggregate) emit_records | awk 'NR==20{print first;next} NR==1{first=$0} {print}' ;;
  bad_phase) printf 'MISO_ENGINE_BENCH_PHASE round_2_complete\n' >&2; emit_records ;;
  interrupt) kill -TERM "$PPID"; exit 143 ;;
  *) exit 90 ;;
esac
EOF
chmod 755 "$template/bin/"* "$template/synthetic-emitter.sh"
command -v cat >"$template/real-cat"

new_case() {
  local name=$1
  case_root="$lifecycle_scratch/$name"
  cp -a "$template" "$case_root"
  printf 'success\n' >"$case_root/synthetic-mode"
  printf 'pass\n' >"$case_root/control-mode"
}
assert_fakes() {
  local tool selected
  for tool in cargo git taskset sleep cat; do
    selected=$(PATH="$case_root/bin:$PATH" command -v "$tool")
    [[ "$selected" == "$case_root/bin/$tool" ]] || { printf 'selected non-fake %s\n' "$tool" >&2; exit 1; }
  done
  [[ -x "$case_root/synthetic-emitter.sh" ]]
}
run_preflight() { assert_fakes; (cd / && PATH="$case_root/bin:$PATH" bash "$case_root/scripts/preflight-builtins-current-benchmark.sh" "$@"); }
run_benchmark() { assert_fakes; (cd / && PATH="$case_root/bin:$PATH" bash "$case_root/scripts/run-builtins-current-benchmark.sh" "$@"); }
disposition_reason() {
  jq -e --arg status "$1" --arg reason "$2" '.status==$status and .reason==$reason and .runner_invocations==1' \
    "$case_root/artifacts/issue431-full-chain/builtins-benchmark.disposition.json" >/dev/null
}

new_case preflight
if run_preflight extra >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$case_root/synthetic-launches" && ! -e "$case_root/artifacts/issue431-full-chain/builtins-benchmark.preflight.json" ]]
run_preflight >/dev/null
[[ ! -e "$case_root/synthetic-launches" ]]
jq -e '.status=="READY" and .issue==431 and .records_required==20 and
 .workload_invocations==0 and .timed_benchmark_invocations==0 and
 .profile=="release" and .lto=="fat" and .codegen_units==1' \
 "$case_root/artifacts/issue431-full-chain/builtins-benchmark.preflight.json" >/dev/null
for selected in cargo git; do [[ "$(PATH="$case_root/bin:$PATH" command -v "$selected")" == "$case_root/bin/$selected" ]]; done

new_case preflight-existing
mkdir -p "$case_root/artifacts/issue431-full-chain"
printf 'protected\n' >"$case_root/artifacts/issue431-full-chain/builtins-benchmark.preflight.json"
if run_preflight >/dev/null 2>&1; then exit 1; fi
[[ "$(<"$case_root/artifacts/issue431-full-chain/builtins-benchmark.preflight.json")" == protected && ! -e "$case_root/synthetic-launches" ]]

run_failure_case() {
  local name=$1 mode=$2 reason=$3
  new_case "$name"; run_preflight >/dev/null
  printf '%s\n' "$mode" >"$case_root/synthetic-mode"
  if run_benchmark >/dev/null 2>&1; then exit 1; fi
  disposition_reason FAIL "$reason" || {
    printf 'wrong lifecycle disposition for %s (expected %s)\n' "$name" "$reason" >&2
    jq . "$case_root/artifacts/issue431-full-chain/builtins-benchmark.disposition.json" >&2
    exit 1
  }
}

new_case bad-args
run_preflight >/dev/null
if run_benchmark extra >/dev/null 2>&1; then exit 1; fi
disposition_reason FAIL invalid_arguments
[[ ! -e "$case_root/synthetic-launches" ]]

for output_name in builtins-benchmark.raw.jsonl builtins-benchmark.jsonl builtins-benchmark.stderr; do
  new_case "existing-$output_name"; run_preflight >/dev/null
  printf 'protected\n' >"$case_root/artifacts/issue431-full-chain/$output_name"
  if run_benchmark >/dev/null 2>&1; then exit 1; fi
  disposition_reason FAIL existing_output
  [[ "$(<"$case_root/artifacts/issue431-full-chain/$output_name")" == protected && ! -e "$case_root/synthetic-launches" ]]
done

new_case symlink-output
run_preflight >/dev/null
ln -s "$case_root/protected" "$case_root/artifacts/issue431-full-chain/builtins-benchmark.raw.jsonl"
if run_benchmark >/dev/null 2>&1; then exit 1; fi
disposition_reason FAIL existing_output
[[ ! -e "$case_root/synthetic-launches" ]]

for drift in binary source validator; do
  new_case "drift-$drift"; run_preflight >/dev/null
  case "$drift" in
    binary) printf 'drift\n' >>"$case_root/target/issue431-prepared/bench" ;;
    source) printf '// drift\n' >>"$case_root/tools/bench/src/builtins.rs" ;;
    validator) printf '# drift\n' >>"$case_root/scripts/builtins-current-benchmark-record-validator.jq" ;;
  esac
  if run_benchmark >/dev/null 2>&1; then exit 1; fi
  disposition_reason FAIL preflight_seal_mismatch
  [[ ! -e "$case_root/synthetic-launches" ]]
done

new_case controlled-refusal
run_preflight >/dev/null
printf 'refuse\n' >"$case_root/control-mode"
if run_benchmark >/dev/null 2>&1; then exit 1; fi
disposition_reason FAIL loadavg_above_ceiling
[[ ! -e "$case_root/synthetic-launches" ]]

new_case uncontrolled
run_preflight >/dev/null
if MISO_ENGINE_BENCH_ALLOW_UNCONTROLLED=1 run_benchmark >/dev/null 2>&1; then exit 1; fi
disposition_reason FAIL uncontrolled_override_forbidden
[[ ! -e "$case_root/synthetic-launches" ]]

run_failure_case warmup warmup_fail workload_failed
[[ "$(wc -l <"$case_root/synthetic-launches")" == 1 ]]
grep -Fqx '{"partial":"warmup"}' "$case_root/artifacts/issue431-full-chain/builtins-benchmark.raw.jsonl"
run_failure_case round1 round1_fail workload_failed
run_failure_case round2 round2_fail workload_failed
run_failure_case bad-record bad_record record_validation_failed
run_failure_case bad-aggregate bad_aggregate aggregate_validation_failed
run_failure_case bad-phase bad_phase phase_mismatch
run_failure_case interruption interrupt workload_interrupted

new_case success
run_preflight >/dev/null
accepted=$(run_benchmark)
raw="$case_root/artifacts/issue431-full-chain/builtins-benchmark.raw.jsonl"
[[ "$accepted" == "$case_root/artifacts/issue431-full-chain/builtins-benchmark.jsonl" ]]
cmp -s "$raw" "$accepted"
[[ "$(wc -l <"$raw")" == 20 && "$(wc -l <"$case_root/synthetic-launches")" == 1 ]]
disposition_reason PASS complete
jq -s -e -L "$case_root/scripts" -f "$case_root/scripts/builtins-current-benchmark-validator.jq" "$raw" >/dev/null
if run_benchmark >/dev/null 2>&1; then exit 1; fi
[[ "$(wc -l <"$case_root/synthetic-launches")" == 1 ]]

printf 'current builtins benchmark validators/lifecycle: PASS (real workload launches: 0)\n'
