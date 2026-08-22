#!/usr/bin/env bash
# Hermetic Issue-110 metadata discovery and lifecycle proof; never launches the real benchmark.
set -euo pipefail
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$script_directory/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
template="$scratch/template"
mkdir -p "$template"/{scripts,bin,target/issue110,target/issue72,target/issue109,fixtures/builtins/v1/{pcm,meters},tools/miso-engine-builtins-bench/src}
mkdir -p "$template/.github/ISSUE_SPECS"
cp "$root/Cargo.lock" "$template/"
cp "$root/tools/miso-engine-builtins-bench/Cargo.toml" "$template/tools/miso-engine-builtins-bench/"
cp "$root/tools/miso-engine-builtins-bench/src/main.rs" "$template/tools/miso-engine-builtins-bench/src/"
cp "$root/fixtures/builtins/v1/MANIFEST.tsv" "$template/fixtures/builtins/v1/"
cp "$root/fixtures/builtins/v1/pcm/graph-taps.f32le" "$template/fixtures/builtins/v1/pcm/"
cp "$root/fixtures/builtins/v1/meters/graph-taps.jsonl" "$template/fixtures/builtins/v1/meters/"
cp "$root/.github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md" \
    "$template/.github/ISSUE_SPECS/"
cp "$root/scripts/run-builtins-benchmark-110.sh" "$root/scripts/preflight-builtins-benchmark-110.sh" \
    "$root/scripts/test-builtins-benchmark-110.sh" "$root/scripts/check-builtins-benchmark-110.sh" \
    "$root/scripts/test-builtins-benchmark-110-policy.sh" "$root/scripts/builtins-benchmark-record-validator.jq" \
    "$root/scripts/builtins-benchmark-validator.jq" "$template/scripts/"
cp "$root/scripts/run-builtins-benchmark-109.sh" "$root/scripts/preflight-builtins-benchmark-109.sh" \
    "$root/scripts/test-builtins-benchmark-109.sh" "$root/scripts/check-builtins-benchmark-109.sh" \
    "$root/scripts/test-builtins-benchmark-109-policy.sh" "$template/scripts/"
cp -a "$root/target/issue72/." "$template/target/issue72/"
cp "$root/target/issue109/metadata-repair.seal.json" "$template/target/issue109/"
cp "$root/target/issue72/builtins-benchmark.raw.jsonl" "$template/base.jsonl"

real_awk="$(command -v awk)"
real_bash="$(command -v bash)"
cat >"$template/bin/git" <<'EOF'
#!/bin/bash
case "$*" in
  *'branch --show-current'*) printf '%s\n' codex/batch-benchmark-110 ;;
  *'rev-parse'*) printf '%s\n' "$MISO_TEST_CANDIDATE" ;;
  *'status'*)
    if [[ "${MISO_TEST_METADATA_MODE:-}" == dirty ||
          ( "${MISO_TEST_METADATA_MODE:-}" == preflight_drift && -e "$MISO_TEST_CASE_ROOT/cargo-built" ) ]]; then
      printf ' M synthetic\n'
    fi ;;
  *) exit 91 ;;
esac
EOF
cat >"$template/bin/bash" <<'EOF'
#!/bin/bash
case "${1:-}" in
  scripts/check-builtins-benchmark-110.sh|scripts/test-builtins-benchmark-110-policy.sh|scripts/test-builtins-benchmark-110.sh)
    exit 0 ;;
  *) exec "$MISO_TEST_REAL_BASH" "$@" ;;
esac
EOF
cat >"$template/bin/cargo" <<'EOF'
#!/bin/bash
printf '%s\n' "$*" >>"$MISO_TEST_CASE_ROOT/cargo.log"
if [[ " $* " == *' build '* ]]; then
  mkdir -p "$CARGO_TARGET_DIR/release"
  printf '#!/bin/bash\nexit 91\n' >"$CARGO_TARGET_DIR/release/miso_engine_builtins_bench"
  chmod 755 "$CARGO_TARGET_DIR/release/miso_engine_builtins_bench"
  : >"$MISO_TEST_CASE_ROOT/cargo-built"
fi
EOF
cat >"$template/bin/uname" <<'EOF'
#!/bin/bash
case "${MISO_TEST_METADATA_MODE:-complete}:$1" in
  drift_authority:-m) printf '\n' >>"$MISO_TEST_DRIFT_BINARY"; printf 'x86_64\n' ;;
  missing_arch:-m|missing_os:-s|missing_kernel:-r) exit 1 ;;
  empty_arch:-m) printf '\n' ;;
  bad_arch:-m) printf 'unknown\n' ;;
  bad_os:-s) printf 'default\n' ;;
  bad_kernel:-r) printf 'bad\rkernel\n' ;;
  *:-m) printf 'x86_64\n' ;;
  *:-s) printf 'TestOS\n' ;;
  *:-r) printf '6.9-test\n' ;;
  *) exit 2 ;;
esac
EOF
cat >"$template/bin/getconf" <<'EOF'
#!/bin/bash
[[ "$1" == _NPROCESSORS_ONLN ]] || exit 2
case "${MISO_TEST_METADATA_MODE:-complete}" in
  missing_logical) exit 1 ;;
  bad_logical) printf '0\n' ;;
  huge_logical) printf '4294967296\n' ;;
  *) printf '8\n' ;;
esac
EOF
cat >"$template/bin/rustc" <<'EOF'
#!/bin/bash
mode=${MISO_TEST_METADATA_MODE:-complete}
case "$1" in
  -V)
    [[ "$mode" != missing_rust ]] || exit 1
    [[ "$mode" != bad_rust ]] || { printf 'unknown\n'; exit; }
    printf 'rustc 1.80.0 (synthetic)\n' ;;
  -vV)
    printf 'rustc 1.80.0 (synthetic)\n'
    [[ "$mode" == missing_host ]] || printf 'host: x86_64-unknown-linux-gnu\n'
    [[ "$mode" == missing_llvm ]] || printf 'LLVM version: 18.1.0\n' ;;
  --print)
    [[ "$2" == cfg && "$3" == --target && "$4" == x86_64-unknown-linux-gnu ]] || exit 2
    if [[ "$mode" == missing_features ]]; then
      printf 'target_os="linux"\n'
    elif [[ "$mode" == duplicate_features ]]; then
      printf 'target_feature="sse2"\ntarget_feature="sse2"\n'
    elif [[ "$mode" == malformed_features ]]; then
      printf 'target_feature=sse2\n'
    else
      printf 'target_feature="sse2"\ntarget_feature="fxsr"\n'
    fi ;;
  *) exit 2 ;;
esac
EOF
cat >"$template/bin/lscpu" <<'EOF'
#!/bin/bash
case "${MISO_TEST_METADATA_MODE:-complete}" in
  missing_physical) exit 1 ;;
  malformed_physical) printf '# comment\n0,0\nbad\n' ;;
  *) printf '# comment\n0,0\n1,0\n0,1\n1,1\n' ;;
esac
EOF
cat >"$template/bin/awk" <<'EOF'
#!/bin/bash
mode=${MISO_TEST_METADATA_MODE:-complete}
case "$*" in
  */proc/cpuinfo*)
    case "$mode" in
      missing_cpu_model) exit 2 ;;
      sentinel_cpu_model) printf 'unknown\n' ;;
      control_cpu_model) printf 'Test\rCPU\n' ;;
      *) printf 'Synthetic CPU 9000\n' ;;
    esac ;;
  */sys/devices/system/cpu/cpu0/cpufreq/scaling_governor*)
    case "$mode" in
      missing_governor) exit 2 ;;
      sentinel_governor) printf 'default\n' ;;
      *) printf 'performance\n' ;;
    esac ;;
  */proc/loadavg*)
    case "$mode" in
      missing_load) exit 2 ;;
      malformed_load) exit 3 ;;
      *) printf '0.10,0.20,0.30;not-controlled\n' ;;
    esac ;;
  *) exec "$MISO_TEST_REAL_AWK" "$@" ;;
esac
EOF
cat >"$template/target/issue110/miso_engine_builtins_bench" <<'EOF'
#!/bin/bash
set -euo pipefail
printf 'launch\n' >>"$MISO_TEST_LAUNCH_LOG"
phase() { printf 'MISO_BUILTINS_BENCH_PHASE %s\n' "$1" >&2; }
phase workload_started
[[ "${MISO_TEST_BINARY_MODE:-success}" != workload_failure ]] || { printf '{"partial":true}\n'; exit 73; }
phase warmup_complete
phase timed_started
jq -c --arg commit "$MISO_ENGINE_BUILTINS_BENCH_CANDIDATE_COMMIT" \
  --arg binary "$MISO_ENGINE_BUILTINS_BENCH_BINARY_SHA256" '
  .candidate_commit=$commit | .binary_sha256=$binary |
  .cpu_model=(env.MISO_ENGINE_BENCH_CPU_MODEL // null) |
  .cpu_architecture=(env.MISO_ENGINE_BENCH_CPU_ARCHITECTURE // null) |
  .logical_core_count=((env.MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT // null) | if .==null then null else tonumber end) |
  .physical_core_count=((env.MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT // null) | if .==null then null else tonumber end) |
  .os=(env.MISO_ENGINE_BENCH_OS // null) | .kernel=(env.MISO_ENGINE_BENCH_KERNEL // null) |
  .governor_or_power_mode=(env.MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE // null) |
  .rust_version=(env.MISO_ENGINE_BENCH_RUST_VERSION // null) |
  .llvm_version=(env.MISO_ENGINE_BENCH_LLVM_VERSION // null) |
  .target_triple=(env.MISO_ENGINE_BENCH_TARGET_TRIPLE // null) |
  .target_features=(env.MISO_ENGINE_BENCH_TARGET_FEATURES // null) |
  .profile=(env.MISO_ENGINE_BENCH_PROFILE // null) | .opt_level=(env.MISO_ENGINE_BENCH_OPT_LEVEL // null) |
  .lto=(env.MISO_ENGINE_BENCH_LTO // null) | .codegen_units=(env.MISO_ENGINE_BENCH_CODEGEN_UNITS // null) |
  .background_load_note=(env.MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE // null) |
  .missing_metadata=(["background_load_note","codegen_units","cpu_architecture","cpu_model",
    "governor_or_power_mode","kernel","llvm_version","logical_core_count","lto","opt_level","os",
    "physical_core_count","profile","rust_version","target_features","target_triple"] as $names |
    [$names[] as $name | select(.[$name]==null) | $name])' "$MISO_TEST_BASE_RECORDS" >"$MISO_TEST_GENERATED"
case "${MISO_TEST_BINARY_MODE:-success}" in
  dishonest_null) jq -c '.cpu_model=null | .missing_metadata += ["cpu_model"] | .missing_metadata|=sort|unique' "$MISO_TEST_GENERATED" ;;
  dishonest_missing) jq -c '.missing_metadata=["cpu_model"]' "$MISO_TEST_GENERATED" ;;
  mixed) jq -c 'if input_line_number==2 then .os="OtherOS" else . end' "$MISO_TEST_GENERATED" ;;
  *) cat "$MISO_TEST_GENERATED" ;;
esac
phase round_1_complete
[[ "${MISO_TEST_BINARY_MODE:-success}" != bad_phases ]] || exit 0
phase round_2_complete
EOF
chmod 755 "$template/bin/"* "$template/target/issue110/miso_engine_builtins_bench"

case_number=0
new_case() {
    case_number=$((case_number + 1))
    case_root="$scratch/case-$case_number-$1"
    cp -a "$template" "$case_root"
    candidate=0123456789abcdef0123456789abcdef01234567
    launch_log="$case_root/launch.log"
    generated="$case_root/generated.jsonl"
    completion="$case_root/target/issue110/completion.seal.json"
    seal="$case_root/target/issue110/builtins-benchmark.preflight.json"
    binary="$case_root/target/issue110/miso_engine_builtins_bench"
    raw="$case_root/target/issue110/builtins-benchmark.raw.jsonl"
    accepted="$case_root/target/issue110/builtins-benchmark.jsonl"
    stderr_log="$case_root/target/issue110/builtins-benchmark.validator.stderr"
    prelaunch="$case_root/target/issue110/builtins-benchmark.prelaunch.disposition.json"
    disposition="$case_root/target/issue110/builtins-benchmark.disposition.json"
    write_seals
}
digest() { sha256sum "$1" | awk '{print $1}'; }
write_seals() {
    lock=$(digest "$case_root/Cargo.lock"); manifest=$(digest "$case_root/fixtures/builtins/v1/MANIFEST.tsv")
    pcm=$(digest "$case_root/fixtures/builtins/v1/pcm/graph-taps.f32le")
    meter=$(digest "$case_root/fixtures/builtins/v1/meters/graph-taps.jsonl")
    tm=$(digest "$case_root/tools/miso-engine-builtins-bench/Cargo.toml")
    ts=$(digest "$case_root/tools/miso-engine-builtins-bench/src/main.rs")
    rv=$(digest "$case_root/scripts/builtins-benchmark-record-validator.jq")
    av=$(digest "$case_root/scripts/builtins-benchmark-validator.jq")
    runner=$(digest "$case_root/scripts/run-builtins-benchmark-110.sh")
    preflight=$(digest "$case_root/scripts/preflight-builtins-benchmark-110.sh")
    lifecycle=$(digest "$case_root/scripts/test-builtins-benchmark-110.sh")
    checker=$(digest "$case_root/scripts/check-builtins-benchmark-110.sh")
    mutation=$(digest "$case_root/scripts/test-builtins-benchmark-110-policy.sh")
    jq -n -S --arg candidate "$candidate" --arg lock "$lock" --arg manifest "$manifest" --arg pcm "$pcm" \
      --arg meter "$meter" --arg tm "$tm" --arg ts "$ts" --arg rv "$rv" --arg av "$av" \
      --arg runner "$runner" --arg preflight "$preflight" --arg lifecycle "$lifecycle" \
      --arg checker "$checker" --arg mutation "$mutation" '
      {schema_version:1,issue:110,kind:"builtins_benchmark_completion",
       accepted_issue068_source_sha256:"0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19",
       branch:"codex/batch-benchmark-110",
       candidate_commit:$candidate,candidate_tree:$candidate,cargo_lock_sha256:$lock,
       fixture_manifest_sha256:$manifest,graph_pcm_sha256:$pcm,graph_meter_sha256:$meter,
       tool_manifest_sha256:$tm,tool_source_sha256:$ts,record_validator_sha256:$rv,
       aggregate_validator_sha256:$av,runner_sha256:$runner,preflight_sha256:$preflight,
       lifecycle_sha256:$lifecycle,checker_sha256:$checker,mutation_sha256:$mutation,
       metadata_regressions:1,issue072_artifacts:{
         "nonbenchmark.seal.json":"7c38b068ae16055df3cfe6b817943f5fbb1a639d85597560e223d631bc37885d",
         "miso_engine_builtins_bench":"a7bafc459b69fb8bdfd7d9195e4ff8d1febf8602a57540498cb579d04a486912",
         "builtins-benchmark.preflight.json":"f4e624b88eddbea5eb09928b544d13093d9a68be278f8afb6b70076fc8dce6bf",
         "builtins-benchmark.raw.jsonl":"c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a",
         "builtins-benchmark.jsonl":"c44433bc5391bafa8463b0cfabcb78cfc80882015ff808c591d40ae5a508819a",
         "builtins-benchmark.validator.stderr":"7935bf62063c0e9d2bfaac91d02db6f448dbf0636fbf16d3c49660738f55b396",
         "builtins-benchmark.disposition.json":"b650449d6a8944f4b00fcd833e5f775c9601a9aeb580864624a4b2c978a0698e",
         "builtins-benchmark.prelaunch.disposition.json":null},issue109_artifacts:{
         "metadata-repair.seal.json":"1e8cec4904d8987ddca581e5b23870629d734127ad3f9e010f6a5c2d178b69c6",
         "miso_engine_builtins_bench":null,"builtins-benchmark.preflight.json":null,
         "builtins-benchmark.raw.jsonl":null,"builtins-benchmark.jsonl":null,
         "builtins-benchmark.validator.stderr":null,
         "builtins-benchmark.prelaunch.disposition.json":null,
         "builtins-benchmark.disposition.json":null},preflight_invocations:0,runner_invocations:0,
       workload_invocations:0,timed_benchmark_invocations:0}' >"$completion"
    completion_sha=$(digest "$completion"); binary_sha=$(digest "$binary")
    jq -n -S --arg candidate "$candidate" --arg binary "$binary_sha" --arg completion "$completion_sha" \
      --arg lock "$lock" --arg manifest "$manifest" --arg pcm "$pcm" --arg meter "$meter" \
      --arg tm "$tm" --arg ts "$ts" --arg rv "$rv" --arg av "$av" --arg runner "$runner" \
      --arg preflight "$preflight" --arg lifecycle "$lifecycle" --arg checker "$checker" --arg mutation "$mutation" '
      {schema_version:1,issue:110,kind:"builtins_benchmark_preflight",
       accepted_issue068_source_sha256:"0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19",
       candidate_commit:$candidate,
       candidate_tree:$candidate,binary_sha256:$binary,completion_seal_sha256:$completion,cargo_lock_sha256:$lock,
       fixture_manifest_sha256:$manifest,graph_pcm_sha256:$pcm,graph_meter_sha256:$meter,
       tool_manifest_sha256:$tm,tool_source_sha256:$ts,record_validator_sha256:$rv,
       aggregate_validator_sha256:$av,runner_sha256:$runner,preflight_sha256:$preflight,
       lifecycle_sha256:$lifecycle,checker_sha256:$checker,mutation_sha256:$mutation,
       records_required:20,warmup_passes:1,measured_rounds:2,preflight_invocations:1,
       runner_invocations:0,workload_invocations:0,timed_benchmark_invocations:0}' >"$seal"
}
run_case() {
    local metadata_mode=$1
    shift
    local binary_mode=${1:-success}
    [[ $# == 0 ]] || shift
    MISO_TEST_METADATA_MODE="$metadata_mode" MISO_TEST_BINARY_MODE="$binary_mode" \
    MISO_TEST_CANDIDATE="$candidate" MISO_TEST_REAL_AWK="$real_awk" MISO_TEST_REAL_BASH="$real_bash" \
    MISO_TEST_DRIFT_BINARY="$binary" \
    MISO_TEST_LAUNCH_LOG="$launch_log" MISO_TEST_BASE_RECORDS="$case_root/base.jsonl" \
    MISO_TEST_GENERATED="$generated" \
    MISO_ENGINE_BENCH_CPU_MODEL=ambient-spoof MISO_ENGINE_BENCH_CPU_ARCHITECTURE=ambient \
    MISO_ENGINE_BENCH_LOGICAL_CORE_COUNT=999 MISO_ENGINE_BENCH_PHYSICAL_CORE_COUNT=999 \
    MISO_ENGINE_BENCH_OS=ambient MISO_ENGINE_BENCH_KERNEL=ambient \
    MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE=ambient MISO_ENGINE_BENCH_RUST_VERSION=ambient \
    MISO_ENGINE_BENCH_LLVM_VERSION=ambient MISO_ENGINE_BENCH_TARGET_TRIPLE=ambient \
    MISO_ENGINE_BENCH_TARGET_FEATURES=ambient MISO_ENGINE_BENCH_PROFILE=ambient \
    MISO_ENGINE_BENCH_OPT_LEVEL=ambient MISO_ENGINE_BENCH_LTO=ambient \
    MISO_ENGINE_BENCH_CODEGEN_UNITS=ambient MISO_ENGINE_BENCH_BACKGROUND_LOAD_NOTE=ambient \
    PATH="$case_root/bin:$PATH" bash "$case_root/scripts/run-builtins-benchmark-110.sh" "$@"
}
expect_prelaunch() {
    [[ ! -e "$launch_log" && ! -e "$raw" && ! -e "$stderr_log" && ! -e "$accepted" ]]
    jq -e --arg reason "$1" '.issue==110 and .reason==$reason and .runner_invocations==1 and
      .workload_invocations==0 and .timed_benchmark_invocations==0' "$prelaunch" >/dev/null
}

new_case complete
published="$(run_case complete)"
[[ "$published" == "$accepted" && "$(wc -l <"$raw")" == 20 ]]
jq -s -e 'all(.[];.cpu_model=="Synthetic CPU 9000" and .cpu_architecture=="x86_64" and
  .logical_core_count==8 and .physical_core_count==4 and .os=="TestOS" and .kernel=="6.9-test" and
  .governor_or_power_mode=="performance" and .rust_version=="rustc 1.80.0 (synthetic)" and
  .llvm_version=="18.1.0" and .target_triple=="x86_64-unknown-linux-gnu" and
  .target_features=="fxsr,sse2" and .profile=="release" and .opt_level=="3" and .lto=="false" and
  .codegen_units=="16" and .background_load_note=="0.10,0.20,0.30;not-controlled" and
  .missing_metadata==[])' "$raw" >/dev/null
cmp -s "$raw" "$accepted"
[[ "$(stat -c %i "$raw")" != "$(stat -c %i "$accepted")" ]]
jq -e --arg raw "$(digest "$raw")" --arg stderr "$(digest "$stderr_log")" \
  '.raw_sha256==$raw and .accepted_sha256==$raw and .stderr_sha256==$stderr' "$disposition" >/dev/null
accepted_sha=$(digest "$accepted"); printf 'mutate raw\n' >>"$raw"; [[ "$(digest "$accepted")" == "$accepted_sha" ]]
jq -e '.status=="PASS" and .reason=="complete" and .workload_invocations==1 and
  .timed_benchmark_invocations==1 and .warmup_passes==1 and .measured_rounds_completed==2 and
  (.metadata_projection_sha256|test("^[0-9a-f]{64}$"))' "$disposition" >/dev/null
[[ "$(wc -l <"$launch_log")" == 1 ]]
disposition_sha=$(digest "$disposition")
if run_case complete >/dev/null 2>&1; then exit 1; fi
[[ "$(digest "$disposition")" == "$disposition_sha" && "$(wc -l <"$launch_log")" == 1 ]]
grep '^MISO_BUILTINS_BENCH_PHASE ' "$stderr_log" >"$case_root/phases"
diff -u - "$case_root/phases" <<'EOF'
MISO_BUILTINS_BENCH_PHASE workload_started
MISO_BUILTINS_BENCH_PHASE warmup_complete
MISO_BUILTINS_BENCH_PHASE timed_started
MISO_BUILTINS_BENCH_PHASE round_1_complete
MISO_BUILTINS_BENCH_PHASE round_2_complete
EOF

for mode_field in missing_cpu_model:cpu_model missing_physical:physical_core_count \
    missing_governor:governor_or_power_mode missing_load:background_load_note; do
    mode=${mode_field%%:*}; field=${mode_field#*:}; new_case "$mode"
    run_case "$mode" >/dev/null
    jq -s -e --arg field "$field" 'all(.[]; .[$field]==null and (.missing_metadata==[$field]))' "$raw" >/dev/null
done

for mode in missing_arch missing_os missing_kernel empty_arch bad_arch bad_os bad_kernel \
    missing_logical bad_logical huge_logical missing_rust bad_rust missing_host missing_llvm \
    missing_features duplicate_features malformed_features sentinel_cpu_model control_cpu_model \
    malformed_physical sentinel_governor malformed_load; do
    new_case "$mode"
    if run_case "$mode" >/dev/null 2>&1; then printf 'metadata failure accepted: %s\n' "$mode" >&2; exit 1; fi
    [[ ! -e "$launch_log" ]]
    jq -e '.runner_invocations==1 and .workload_invocations==0 and .timed_benchmark_invocations==0' \
      "$prelaunch" >/dev/null
done

for binary_mode in dishonest_null dishonest_missing mixed workload_failure bad_phases; do
    new_case "$binary_mode"
    set +e; run_case complete "$binary_mode" >/dev/null 2>&1; status=$?; set -e
    [[ $status != 0 && "$(wc -l <"$launch_log")" == 1 && ! -e "$accepted" ]]
    jq -e '.status=="FAIL" and .workload_invocations==1' "$disposition" >/dev/null
    if [[ "$binary_mode" == workload_failure ]]; then grep -Fqx '{"partial":true}' "$raw"; fi
done

new_case arguments
if run_case complete success --bad >/dev/null 2>&1; then exit 1; fi
expect_prelaunch invalid_arguments
prelaunch_sha=$(digest "$prelaunch")
if run_case complete >/dev/null 2>&1; then exit 1; fi
[[ "$(digest "$prelaunch")" == "$prelaunch_sha" ]]

new_case missing-tool
missing_bin="$case_root/missing-bin"
mkdir "$missing_bin"
for tool in bash mkdir mktemp mv rm stat; do ln -s "$(command -v "$tool")" "$missing_bin/$tool"; done
set +e
MISO_TEST_METADATA_MODE=complete MISO_TEST_CANDIDATE="$candidate" \
  PATH="$missing_bin" "$missing_bin/bash" "$case_root/scripts/run-builtins-benchmark-110.sh" \
  >"$case_root/result" 2>&1
status=$?
set -e
[[ $status == 1 && ! -e "$launch_log" ]]
jq -e '.reason=="missing_tool" and .runner_invocations==1 and .workload_invocations==0 and
  .timed_benchmark_invocations==0' "$prelaunch" >/dev/null

new_case dirty
if run_case dirty >/dev/null 2>&1; then exit 1; fi
expect_prelaunch dirty_candidate

new_case authority-drift
if run_case drift_authority >/dev/null 2>&1; then exit 1; fi
expect_prelaunch authority_drift

new_case seal
jq '.cargo_lock_sha256="eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"' "$seal" >"$case_root/mutated"
mv "$case_root/mutated" "$seal"
if run_case complete >/dev/null 2>&1; then exit 1; fi
expect_prelaunch preflight_seal_mismatch

bad=eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
for authority in cargo_lock_sha256 fixture_manifest_sha256 graph_pcm_sha256 graph_meter_sha256 \
  tool_manifest_sha256 tool_source_sha256 record_validator_sha256 aggregate_validator_sha256 \
  runner_sha256 preflight_sha256 lifecycle_sha256 checker_sha256 mutation_sha256 \
  accepted_issue068_source_sha256; do
  new_case "direct-$authority"
  jq --arg key "$authority" --arg bad "$bad" '.[$key]=$bad' "$seal" >"$case_root/mutated"
  mv "$case_root/mutated" "$seal"
  if run_case complete >/dev/null 2>&1; then printf 'direct authority accepted: %s\n' "$authority" >&2; exit 1; fi
  expect_prelaunch preflight_seal_mismatch
done

for authority in cargo_lock_sha256 fixture_manifest_sha256 graph_pcm_sha256 graph_meter_sha256 \
  tool_manifest_sha256 tool_source_sha256 record_validator_sha256 aggregate_validator_sha256 \
  runner_sha256 preflight_sha256 lifecycle_sha256 checker_sha256 mutation_sha256 \
  accepted_issue068_source_sha256; do
  new_case "tandem-$authority"
  jq --arg key "$authority" --arg bad "$bad" '.[$key]=$bad' "$completion" >"$case_root/mutated"
  mv "$case_root/mutated" "$completion"
  replacement=$(digest "$completion")
  jq --arg key "$authority" --arg bad "$bad" --arg completion "$replacement" \
    '.[$key]=$bad | .completion_seal_sha256=$completion' "$seal" >"$case_root/mutated"
  mv "$case_root/mutated" "$seal"
  if run_case complete >/dev/null 2>&1; then printf 'tandem authority accepted: %s\n' "$authority" >&2; exit 1; fi
  expect_prelaunch completion_seal_mismatch
done

new_case tandem-issue072
jq '.issue072_artifacts["builtins-benchmark.raw.jsonl"]="eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"' \
  "$completion" >"$case_root/mutated"
mv "$case_root/mutated" "$completion"
replacement=$(digest "$completion")
jq --arg completion "$replacement" '.completion_seal_sha256=$completion' "$seal" >"$case_root/mutated"
mv "$case_root/mutated" "$seal"
if run_case complete >/dev/null 2>&1; then exit 1; fi
expect_prelaunch completion_seal_mismatch

new_case tandem-issue109
jq '.issue109_artifacts["metadata-repair.seal.json"]="eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"' \
  "$completion" >"$case_root/mutated"
mv "$case_root/mutated" "$completion"
replacement=$(digest "$completion")
jq --arg completion "$replacement" '.completion_seal_sha256=$completion' "$seal" >"$case_root/mutated"
mv "$case_root/mutated" "$seal"
if run_case complete >/dev/null 2>&1; then exit 1; fi
expect_prelaunch completion_seal_mismatch

new_case inherited-issue109
printf mutation >>"$case_root/target/issue109/metadata-repair.seal.json"
if run_case complete >/dev/null 2>&1; then exit 1; fi
expect_prelaunch inherited_evidence_mismatch

new_case forbidden-issue109
printf forbidden >"$case_root/target/issue109/builtins-benchmark.raw.jsonl"
if run_case complete >/dev/null 2>&1; then exit 1; fi
expect_prelaunch inherited_evidence_mismatch

new_case extra-issue109
printf unexpected >"$case_root/target/issue109/unexpected"
if run_case complete >/dev/null 2>&1; then exit 1; fi
expect_prelaunch inherited_evidence_mismatch

for artifact in raw accepted stderr prelaunch disposition; do
  for kind in regular symlink hardlink; do
    new_case "existing-$artifact-$kind"
    case "$artifact" in raw) protected=$raw;; accepted) protected=$accepted;; stderr) protected=$stderr_log;;
      prelaunch) protected=$prelaunch;; disposition) protected=$disposition;; esac
    case "$kind" in regular) printf protected >"$protected";; symlink) ln -s "$case_root/base.jsonl" "$protected";;
      hardlink) ln "$case_root/base.jsonl" "$protected";; esac
    before=$(stat -c '%F:%h:%s' "$protected")
    if run_case complete >/dev/null 2>&1; then exit 1; fi
    [[ ! -e "$launch_log" && "$(stat -c '%F:%h:%s' "$protected")" == "$before" ]]
  done
done

run_preflight() {
  MISO_TEST_METADATA_MODE=${1:-complete} MISO_TEST_CANDIDATE="$candidate" \
    MISO_TEST_CASE_ROOT="$case_root" MISO_TEST_REAL_BASH="$real_bash" MISO_TEST_REAL_AWK="$real_awk" \
    PATH="$case_root/bin:$PATH" "$real_bash" "$case_root/scripts/preflight-builtins-benchmark-110.sh" "${@:2}"
}
prepare_preflight_case() {
  new_case "$1"
  rm "$binary" "$seal"
}

prepare_preflight_case preflight-success
run_preflight >/dev/null
[[ -x "$binary" && -f "$seal" && ! -L "$binary" && ! -L "$seal" &&
   "$(stat -c %h "$binary")" == 1 && "$(stat -c %h "$seal")" == 1 ]]
jq -e '.issue==110 and .preflight_invocations==1 and .runner_invocations==0 and
  .workload_invocations==0 and .timed_benchmark_invocations==0 and .records_required==20' "$seal" >/dev/null
[[ "$(wc -l <"$case_root/cargo.log")" == 2 && ! -e "$launch_log" ]]
seal_sha=$(digest "$seal"); binary_sha=$(digest "$binary")
if run_preflight >/dev/null 2>&1; then exit 1; fi
[[ "$(digest "$seal")" == "$seal_sha" && "$(digest "$binary")" == "$binary_sha" && ! -e "$launch_log" ]]

prepare_preflight_case preflight-arguments
if run_preflight complete --bad >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$binary" && ! -e "$seal" && ! -e "$case_root/cargo.log" ]]

prepare_preflight_case preflight-drift
if run_preflight preflight_drift >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$binary" && ! -e "$seal" && ! -e "$launch_log" ]]

prepare_preflight_case preflight-completion-mismatch
jq '.cargo_lock_sha256="eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"' \
  "$completion" >"$case_root/mutated"; mv "$case_root/mutated" "$completion"
if run_preflight >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$binary" && ! -e "$seal" && ! -e "$case_root/cargo.log" ]]

prepare_preflight_case preflight-issue109-seal
printf mutation >>"$case_root/target/issue109/metadata-repair.seal.json"
if run_preflight >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$binary" && ! -e "$seal" && ! -e "$case_root/cargo.log" ]]

prepare_preflight_case preflight-issue109-forbidden
printf forbidden >"$case_root/target/issue109/builtins-benchmark.raw.jsonl"
if run_preflight >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$binary" && ! -e "$seal" && ! -e "$case_root/cargo.log" ]]

prepare_preflight_case preflight-issue109-extra
printf unexpected >"$case_root/target/issue109/unexpected"
if run_preflight >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$binary" && ! -e "$seal" && ! -e "$case_root/cargo.log" ]]

for artifact in binary seal raw accepted stderr prelaunch disposition; do
  for kind in regular symlink hardlink; do
    prepare_preflight_case "preflight-existing-$artifact-$kind"
    case "$artifact" in binary) protected=$binary;; seal) protected=$seal;; raw) protected=$raw;;
      accepted) protected=$accepted;; stderr) protected=$stderr_log;; prelaunch) protected=$prelaunch;;
      disposition) protected=$disposition;; esac
    case "$kind" in regular) printf protected >"$protected";; symlink) ln -s "$case_root/base.jsonl" "$protected";;
      hardlink) ln "$case_root/base.jsonl" "$protected";; esac
    before=$(stat -c '%F:%h:%s' "$protected")
    if run_preflight >/dev/null 2>&1; then exit 1; fi
    [[ "$(stat -c '%F:%h:%s' "$protected")" == "$before" && ! -e "$launch_log" ]]
  done
done
printf 'Issue-110 metadata lifecycle: PASS (real preflight/runner/workload/timed=0/0/0/0)\n'
