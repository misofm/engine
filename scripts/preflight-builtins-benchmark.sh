#!/usr/bin/env bash
# Seals the Issue-035 builtin benchmark candidate without executing the workload.
set -euo pipefail

[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
artifact_directory="$repository_root/target/issue35"
sealed_binary="$artifact_directory/miso_engine_builtins_bench"
seal="$artifact_directory/builtins-benchmark.preflight.json"
raw_output="$artifact_directory/builtins-benchmark.raw.jsonl"
accepted_output="$artifact_directory/builtins-benchmark.jsonl"
stderr_output="$artifact_directory/builtins-benchmark.validator.stderr"
disposition="$artifact_directory/builtins-benchmark.disposition.json"
runner="$script_directory/run-builtins-benchmark.sh"
record_validator="$script_directory/builtins-benchmark-record-validator.jq"
aggregate_validator="$script_directory/builtins-benchmark-validator.jq"
validator_test="$script_directory/test-builtins-benchmark.sh"
issue068_spec="$repository_root/.github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md"
issue070_spec="$repository_root/.github/ISSUE_SPECS/070-quiescent-builtin-graph-retirement-worker-trace-closure.md"

fail() {
    printf 'Issue-058 builtin benchmark preflight failure: %s\n' "$1" >&2
    exit 1
}
hash_file() {
    sha256sum "$1" | awk '{print $1}'
}
require_hash() {
    local path=$1 expected=$2 actual
    [[ -f "$path" && ! -L "$path" ]] || fail "required regular file is unavailable: $path"
    actual="$(hash_file "$path")"
    [[ "$actual" == "$expected" ]] || fail "hash mismatch for $path: $actual"
}
require_literal() {
    local path=$1 literal=$2
    rg -Fq -- "$literal" "$path" || fail "accepted evidence identity is absent from $path: $literal"
}
issue068_source_manifest() {
    {
        printf '%s\n' Cargo.toml Cargo.lock \
            scripts/check-builtins-targets.sh \
            scripts/check-rack-instructions.sh \
            scripts/check-builtins-target-instructions.sh
        [[ ! -d .cargo ]] || find .cargo -type f -print
        find crates/miso-engine-core crates/miso-engine-builtins \
            crates/miso-engine-builtins-compiler crates/miso-engine-graph \
            crates/miso-engine-graph-compiler -type f \
            \( -name Cargo.toml -o -name '*.rs' \) -print
    } | LC_ALL=C sort -u | while IFS= read -r path; do
        if [[ "$path" == Cargo.lock ]]; then
            printf '%s\t%s\t%s\n' "$path" \
                "$(git cat-file -s "$lock_base_commit:$path")" \
                "$(git show "$lock_base_commit:$path" | sha256sum | awk '{print $1}')"
        else
            [[ -f "$path" && ! -L "$path" ]] || fail "source-manifest entry is not a regular file: $path"
            printf '%s\t%s\t%s\n' "$path" "$(wc -c <"$path" | tr -d ' ')" "$(hash_file "$path")"
        fi
    done
}
candidate_source_manifest() {
    {
        printf '%s\n' Cargo.toml Cargo.lock \
            scripts/run-builtins-benchmark.sh \
            scripts/preflight-builtins-benchmark.sh \
            scripts/test-builtins-benchmark.sh \
            scripts/builtins-benchmark-record-validator.jq \
            scripts/builtins-benchmark-validator.jq
        [[ ! -d .cargo ]] || find .cargo -type f -print
        find crates -type f \( -name Cargo.toml -o -name '*.rs' \) -print
        find tools/miso-engine-builtins-bench -type f \
            \( -name Cargo.toml -o -name '*.rs' \) -print
    } | LC_ALL=C sort -u | while IFS= read -r path; do
        [[ -f "$path" && ! -L "$path" ]] || fail "candidate-source entry is not a regular file: $path"
        printf '%s\t%s\t%s\n' "$path" "$(wc -c <"$path" | tr -d ' ')" "$(hash_file "$path")"
    done
}

for tool in bash cargo git jq sha256sum wc cmp cp chmod find rg awk mktemp mv sort tr; do
    command -v "$tool" >/dev/null || fail "required tool is unavailable: $tool"
done
cd "$repository_root"
candidate_commit="$(git rev-parse --verify HEAD)"
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || fail 'candidate is not clean'

for path in "$sealed_binary" "$seal" "$raw_output" "$accepted_output" "$stderr_output" "$disposition"; do
    [[ ! -e "$path" && ! -L "$path" ]] || fail "refusing to overwrite Issue-035 artifact: $path"
done

manifest_sha256=bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff
graph_pcm_sha256=508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19
graph_meter_sha256=958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f
direct_audit_sha256=3581ebf058151a0a0014ff08adcdd7fcd6fe6ad51a5baf41538272d4bba6ce8e
graph_audit_sha256=54103c89b557a72da9c79cd00a636ea64933240a4dcb27c27647fb960b013db4
graph_trace_sha256=812e7c62cf8963fba1cb6f32615005ec8bd7df6b97f6c72a0c4960fadcf0d4c1
graph_trace_validator_sha256=1c98d033c0c5d156dea887a829cc683d460145c08856c705fdbde7ef8b4324c5
accepted_issue068_source_sha256=0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19
preimplementation_lock_sha256=96d0585ab8059905b256f87e7cadd717ae6e790aa140de3a4e7cc9db4791d424
lock_base_commit=265109f300f58e005ac7a68a56298d167c5ae809
lock_diff_sha256=5ebc70f8a35208d50ff4d9afd92602462180b345125263a0a4916aa3bb08940e

require_hash fixtures/builtins/v1/MANIFEST.tsv "$manifest_sha256"
require_hash fixtures/builtins/v1/pcm/graph-taps.f32le "$graph_pcm_sha256"
require_hash fixtures/builtins/v1/meters/graph-taps.jsonl "$graph_meter_sha256"
require_literal "$issue068_spec" "$accepted_issue068_source_sha256"
for identity in "$direct_audit_sha256" "$graph_audit_sha256" "$graph_trace_sha256" "$graph_trace_validator_sha256"; do
    require_literal "$issue070_spec" "$identity"
done
base_lock_sha256="$(git show "$lock_base_commit:Cargo.lock" | sha256sum | awk '{print $1}')"
[[ "$base_lock_sha256" == "$preimplementation_lock_sha256" ]] || fail 'preimplementation lock provenance changed'
actual_lock_diff_sha256="$(git diff --no-ext-diff --binary "$lock_base_commit" -- Cargo.lock | sha256sum | awk '{print $1}')"
[[ "$actual_lock_diff_sha256" == "$lock_diff_sha256" ]] || fail 'Cargo.lock changed beyond the frozen benchmark dependency stanza'
candidate_lock_sha256="$(hash_file Cargo.lock)"

scratch="$(mktemp -d "${TMPDIR:-/tmp}/miso-engine-issue058-preflight.XXXXXX")"
trap 'rm -rf -- "$scratch"' EXIT
issue068_source_manifest >"$scratch/issue068-source-manifest.tsv"
[[ "$(hash_file "$scratch/issue068-source-manifest.tsv")" == "$accepted_issue068_source_sha256" ]] ||
    fail 'accepted Issue-068 source manifest cannot be reconstructed from the frozen inputs'
candidate_source_manifest >"$scratch/source-before.tsv"
candidate_source_sha256="$(hash_file "$scratch/source-before.tsv")"
runner_sha256="$(hash_file "$runner")"
preflight_script_sha256="$(hash_file "$script_directory/preflight-builtins-benchmark.sh")"
record_validator_sha256="$(hash_file "$record_validator")"
aggregate_validator_sha256="$(hash_file "$aggregate_validator")"
validator_test_sha256="$(hash_file "$validator_test")"

# Proportional, nonexecuting qualification. None of these commands invokes benchmark main.
bash scripts/check-builtins-fixtures.sh
bash "$validator_test"
cargo test --locked -p miso-engine-builtins-bench
cargo test --locked -p miso-engine-builtins-compiler --features test-support \
    phase_two_allocator_layouts_match_the_checked_resource_report
cargo clippy --locked -p miso-engine-builtins-bench --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked -p miso-engine-builtins-bench --no-deps
for policy in workspace realtime builtins graph rack; do
    bash "scripts/check-${policy}-policy.sh" .
done

build_directory="$scratch/build"
CARGO_TARGET_DIR="$build_directory" cargo build --locked --release -p miso-engine-builtins-bench
built_binary="$build_directory/release/miso_engine_builtins_bench"
[[ -x "$built_binary" && ! -L "$built_binary" ]] || fail 'fresh release binary was not produced'

[[ "$(git rev-parse --verify HEAD)" == "$candidate_commit" ]] || fail 'candidate commit changed during preflight'
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || fail 'candidate became dirty during preflight'
candidate_source_manifest >"$scratch/source-after.tsv"
cmp -s "$scratch/source-before.tsv" "$scratch/source-after.tsv" || fail 'candidate source changed during preflight'
[[ "$(hash_file Cargo.lock)" == "$candidate_lock_sha256" ]] || fail 'Cargo.lock changed during preflight'
[[ "$(git diff --no-ext-diff --binary "$lock_base_commit" -- Cargo.lock | sha256sum | awk '{print $1}')" == "$lock_diff_sha256" ]] ||
    fail 'Cargo.lock transition changed during preflight'
issue068_source_manifest >"$scratch/issue068-source-after.tsv"
[[ "$(hash_file "$scratch/issue068-source-after.tsv")" == "$accepted_issue068_source_sha256" ]] ||
    fail 'accepted Issue-068 source identity changed during preflight'
require_hash fixtures/builtins/v1/MANIFEST.tsv "$manifest_sha256"
require_hash fixtures/builtins/v1/pcm/graph-taps.f32le "$graph_pcm_sha256"
require_hash fixtures/builtins/v1/meters/graph-taps.jsonl "$graph_meter_sha256"
require_hash "$runner" "$runner_sha256"
require_hash "$script_directory/preflight-builtins-benchmark.sh" "$preflight_script_sha256"
require_hash "$record_validator" "$record_validator_sha256"
require_hash "$aggregate_validator" "$aggregate_validator_sha256"
require_hash "$validator_test" "$validator_test_sha256"
for identity in "$direct_audit_sha256" "$graph_audit_sha256" "$graph_trace_sha256" "$graph_trace_validator_sha256"; do
    require_literal "$issue070_spec" "$identity"
done
for path in "$sealed_binary" "$seal" "$raw_output" "$accepted_output" "$stderr_output" "$disposition"; do
    [[ ! -e "$path" && ! -L "$path" ]] || fail "Issue-035 artifact appeared during preflight: $path"
done

mkdir -p "$artifact_directory"
temporary_binary="$(mktemp "$artifact_directory/.miso_engine_builtins_bench.XXXXXX")"
cp -- "$built_binary" "$temporary_binary"
chmod 0755 "$temporary_binary"
[[ ! -e "$sealed_binary" && ! -L "$sealed_binary" ]] || fail 'sealed binary appeared during preflight'
mv -n -- "$temporary_binary" "$sealed_binary"
[[ ! -e "$temporary_binary" && -x "$sealed_binary" && ! -L "$sealed_binary" ]] || fail 'binary publication was not atomic and no-clobber'

binary_sha256="$(hash_file "$sealed_binary")"
issue068_spec_sha256="$(hash_file "$issue068_spec")"
issue070_spec_sha256="$(hash_file "$issue070_spec")"
seal_payload_sha256="$(printf '%s\n' \
    "$candidate_commit" "$candidate_source_sha256" "$candidate_lock_sha256" \
    "$binary_sha256" "$runner_sha256" "$preflight_script_sha256" \
    "$record_validator_sha256" "$aggregate_validator_sha256" "$validator_test_sha256" \
    "$manifest_sha256" "$graph_pcm_sha256" "$graph_meter_sha256" \
    "$direct_audit_sha256" "$graph_audit_sha256" "$graph_trace_sha256" \
    "$graph_trace_validator_sha256" "$accepted_issue068_source_sha256" | sha256sum | awk '{print $1}')"

temporary_seal="$(mktemp "$artifact_directory/.builtins-benchmark.preflight.XXXXXX")"
jq -n -S \
    --arg candidate_commit "$candidate_commit" \
    --arg candidate_source_sha256 "$candidate_source_sha256" \
    --arg cargo_lock_sha256 "$candidate_lock_sha256" \
    --arg preimplementation_lock_sha256 "$preimplementation_lock_sha256" \
    --arg binary_sha256 "$binary_sha256" \
    --arg runner_sha256 "$runner_sha256" \
    --arg preflight_script_sha256 "$preflight_script_sha256" \
    --arg record_validator_sha256 "$record_validator_sha256" \
    --arg aggregate_validator_sha256 "$aggregate_validator_sha256" \
    --arg validator_test_sha256 "$validator_test_sha256" \
    --arg fixture_manifest_sha256 "$manifest_sha256" \
    --arg graph_pcm_sha256 "$graph_pcm_sha256" \
    --arg graph_meter_sha256 "$graph_meter_sha256" \
    --arg direct_audit_sha256 "$direct_audit_sha256" \
    --arg graph_audit_sha256 "$graph_audit_sha256" \
    --arg graph_trace_sha256 "$graph_trace_sha256" \
    --arg graph_trace_validator_sha256 "$graph_trace_validator_sha256" \
    --arg accepted_issue068_source_sha256 "$accepted_issue068_source_sha256" \
    --arg issue068_spec_sha256 "$issue068_spec_sha256" \
    --arg issue070_spec_sha256 "$issue070_spec_sha256" \
    --arg seal_payload_sha256 "$seal_payload_sha256" \
    '{schema_version:2, issue:58, kind:"builtins_benchmark_preflight",
      candidate_commit:$candidate_commit, candidate_source_sha256:$candidate_source_sha256,
      cargo_lock_sha256:$cargo_lock_sha256,
      preimplementation_lock_sha256:$preimplementation_lock_sha256,
      binary_sha256:$binary_sha256, runner_sha256:$runner_sha256,
      preflight_script_sha256:$preflight_script_sha256,
      record_validator_sha256:$record_validator_sha256,
      aggregate_validator_sha256:$aggregate_validator_sha256,
      validator_test_sha256:$validator_test_sha256,
      fixture_manifest_sha256:$fixture_manifest_sha256,
      graph_pcm_sha256:$graph_pcm_sha256, graph_meter_sha256:$graph_meter_sha256,
      direct_audit_sha256:$direct_audit_sha256, graph_audit_sha256:$graph_audit_sha256,
      graph_trace_sha256:$graph_trace_sha256,
      graph_trace_validator_sha256:$graph_trace_validator_sha256,
      accepted_issue068_source_sha256:$accepted_issue068_source_sha256,
      issue068_spec_sha256:$issue068_spec_sha256, issue070_spec_sha256:$issue070_spec_sha256,
      seal_payload_sha256:$seal_payload_sha256,
      records_required:20, warmup_passes:1, measured_rounds:2,
      runner_invocations:0, workload_invocations:0, timed_benchmark_invocations:0}' \
    >"$temporary_seal"
[[ -s "$temporary_seal" ]] || fail 'preflight seal is empty'
for path in "$seal" "$raw_output" "$accepted_output" "$stderr_output" "$disposition"; do
    [[ ! -e "$path" && ! -L "$path" ]] || fail "Issue-035 artifact appeared before seal publication: $path"
done
[[ ! -e "$seal" && ! -L "$seal" ]] || fail 'preflight seal appeared during publication'
mv -n -- "$temporary_seal" "$seal"
[[ ! -e "$temporary_seal" && -f "$seal" && ! -L "$seal" ]] || fail 'seal publication was not atomic and no-clobber'

printf 'Issue-058 builtin benchmark preflight: PASS (runner=0 workload=0 timed=0)\n'
printf 'candidate=%s binary_sha256=%s seal_sha256=%s\n' \
    "$candidate_commit" "$binary_sha256" "$(hash_file "$seal")"
