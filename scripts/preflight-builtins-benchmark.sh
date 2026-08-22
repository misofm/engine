#!/usr/bin/env bash
# Exactly-once Issue-072 preflight. Builds and seals without executing the benchmark.
set -euo pipefail

[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
artifact_directory="$repository_root/target/issue72"
nonbenchmark_seal="$artifact_directory/nonbenchmark.seal.json"
sealed_binary="$artifact_directory/miso_engine_builtins_bench"
seal="$artifact_directory/builtins-benchmark.preflight.json"
raw_output="$artifact_directory/builtins-benchmark.raw.jsonl"
accepted_output="$artifact_directory/builtins-benchmark.jsonl"
stderr_output="$artifact_directory/builtins-benchmark.validator.stderr"
prelaunch_disposition="$artifact_directory/builtins-benchmark.prelaunch.disposition.json"
disposition="$artifact_directory/builtins-benchmark.disposition.json"
runner="$script_directory/run-builtins-benchmark.sh"
record_validator="$script_directory/builtins-benchmark-record-validator.jq"
aggregate_validator="$script_directory/builtins-benchmark-validator.jq"
lifecycle="$script_directory/test-builtins-benchmark.sh"

fail() { printf 'Issue-072 builtin benchmark preflight failure: %s\n' "$1" >&2; exit 1; }
hash_file() { sha256sum "$1" | awk '{print $1}'; }
require_hash() {
    local path=$1 expected=$2
    [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 ]] ||
        fail "required one-link regular file is unavailable: $path"
    [[ "$(hash_file "$path")" == "$expected" ]] || fail "hash mismatch for $path"
}
for tool in awk bash cargo chmod cp dirname git jq mkdir mktemp mv rg rm sha256sum stat wc; do
    command -v "$tool" >/dev/null 2>&1 || fail "required tool is unavailable: $tool"
done
cd "$repository_root"

candidate_branch="$(git branch --show-current)"
candidate_commit="$(git rev-parse --verify HEAD)"
candidate_tree="$(git rev-parse 'HEAD^{tree}')"
[[ "$candidate_branch" == codex/batch-benchmark-072 ]] || fail 'candidate branch changed'
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || fail 'candidate is not clean'
mkdir -p "$artifact_directory"
[[ ! -L "$artifact_directory" ]] || fail 'artifact directory is a symlink'
for path in "$sealed_binary" "$seal" "$raw_output" "$accepted_output" "$stderr_output" \
    "$prelaunch_disposition" "$disposition"; do
    [[ ! -e "$path" && ! -L "$path" ]] || fail "refusing existing Issue-072 artifact: $path"
done

lock_sha256=4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a
manifest_sha256=bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff
graph_pcm_sha256=508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19
graph_meter_sha256=958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f
issue068_source_sha256=0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19
record_validator_sha256=c3db1d9574360bdab0d9ac335615787446e5537439d6accdded4fdd0a4479467
aggregate_validator_sha256=6085e740f15d7902fca4443d761cfb8e29df7168ba12f632c7946db56a3e1b63

require_hash Cargo.lock "$lock_sha256"
require_hash fixtures/builtins/v1/MANIFEST.tsv "$manifest_sha256"
require_hash fixtures/builtins/v1/pcm/graph-taps.f32le "$graph_pcm_sha256"
require_hash fixtures/builtins/v1/meters/graph-taps.jsonl "$graph_meter_sha256"
require_hash "$record_validator" "$record_validator_sha256"
require_hash "$aggregate_validator" "$aggregate_validator_sha256"
rg -Fq "$issue068_source_sha256" \
    .github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md ||
    fail 'accepted Issue-068 source identity is unavailable'
while read -r name bytes digest; do
    path="target/issue35/$name"
    require_hash "$path" "$digest"
    [[ "$(wc -c <"$path")" == "$bytes" ]] || fail "Issue-058 artifact size: $name"
done <<'EOF'
miso_engine_builtins_bench 3191104 242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944
builtins-benchmark.preflight.json 2211 85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d
builtins-benchmark.raw.jsonl 0 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
builtins-benchmark.validator.stderr 0 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
builtins-benchmark.disposition.json 974 e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce
EOF
for name in builtins-benchmark.jsonl builtins-benchmark.prelaunch.disposition.json; do
    [[ ! -e "target/issue35/$name" && ! -L "target/issue35/$name" ]] ||
        fail "forbidden Issue-058 artifact appeared: $name"
done
[[ -f "$nonbenchmark_seal" && ! -L "$nonbenchmark_seal" &&
   "$(stat -c %h "$nonbenchmark_seal")" == 1 ]] || fail 'nonbenchmark seal unavailable'

tool_source_sha256="$(hash_file tools/miso-engine-builtins-bench/src/main.rs)"
runner_sha256="$(hash_file "$runner")"
preflight_script_sha256="$(hash_file "$script_directory/preflight-builtins-benchmark.sh")"
lifecycle_sha256="$(hash_file "$lifecycle")"
nonbenchmark_sha256="$(hash_file "$nonbenchmark_seal")"
jq -e \
    --arg branch "$candidate_branch" --arg commit "$candidate_commit" --arg tree "$candidate_tree" \
    --arg lock "$lock_sha256" --arg source "$tool_source_sha256" \
    --arg runner "$runner_sha256" --arg preflight "$preflight_script_sha256" \
    --arg lifecycle "$lifecycle_sha256" --arg record "$record_validator_sha256" \
    --arg aggregate "$aggregate_validator_sha256" --arg manifest "$manifest_sha256" \
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
       "miso_engine_builtins_bench":"242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944",
       "builtins-benchmark.preflight.json":"85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d",
       "builtins-benchmark.raw.jsonl":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
       "builtins-benchmark.validator.stderr":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
       "builtins-benchmark.disposition.json":"e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce",
       "builtins-benchmark.jsonl":null,
       "builtins-benchmark.prelaunch.disposition.json":null} and
     .focused_regressions == 1 and .preflight_invocations == 0 and .runner_invocations == 0 and
     .workload_invocations == 0 and .timed_benchmark_invocations == 0' \
    "$nonbenchmark_seal" >/dev/null || fail 'nonbenchmark seal mismatch'

scratch="$(mktemp -d "${TMPDIR:-/tmp}/miso-engine-issue072-preflight.XXXXXX")"
trap 'rm -rf -- "$scratch"' EXIT
# Proportional nonexecuting gates. The benchmark binary is built but never launched.
bash scripts/check-builtins-fixtures.sh
bash "$lifecycle"
cargo test --locked -p miso-engine-builtins-bench
cargo test --locked -p miso-engine-builtins-compiler --features test-support \
    phase_two_allocator_layouts_match_the_checked_resource_report
cargo clippy --locked -p miso-engine-builtins-bench --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked -p miso-engine-builtins-bench --no-deps
for policy in workspace realtime builtins graph rack; do bash "scripts/check-${policy}-policy.sh" .; done

build_directory="$scratch/build"
CARGO_TARGET_DIR="$build_directory" cargo build --locked --release -p miso-engine-builtins-bench
built_binary="$build_directory/release/miso_engine_builtins_bench"
[[ -x "$built_binary" && ! -L "$built_binary" ]] || fail 'fresh release binary unavailable'
[[ "$(git rev-parse --verify HEAD)" == "$candidate_commit" &&
   "$(git rev-parse 'HEAD^{tree}')" == "$candidate_tree" &&
   -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || fail 'candidate drifted'
require_hash Cargo.lock "$lock_sha256"
require_hash "$nonbenchmark_seal" "$nonbenchmark_sha256"
require_hash tools/miso-engine-builtins-bench/src/main.rs "$tool_source_sha256"
require_hash "$runner" "$runner_sha256"
require_hash "$script_directory/preflight-builtins-benchmark.sh" "$preflight_script_sha256"
require_hash "$lifecycle" "$lifecycle_sha256"
require_hash "$record_validator" "$record_validator_sha256"
require_hash "$aggregate_validator" "$aggregate_validator_sha256"
require_hash fixtures/builtins/v1/MANIFEST.tsv "$manifest_sha256"
require_hash fixtures/builtins/v1/pcm/graph-taps.f32le "$graph_pcm_sha256"
require_hash fixtures/builtins/v1/meters/graph-taps.jsonl "$graph_meter_sha256"
while read -r name bytes digest; do
    path="target/issue35/$name"
    require_hash "$path" "$digest"
    [[ "$(wc -c <"$path")" == "$bytes" ]] || fail "Issue-058 artifact size after gates: $name"
done <<'EOF'
miso_engine_builtins_bench 3191104 242f6789ea994c4147205396bb10c10dbef85a48681160037680bb5b745b8944
builtins-benchmark.preflight.json 2211 85fcfcfb1c72e2dfd1128667c583dfc2aae74b5f183bb4d04dd8604fa07a195d
builtins-benchmark.raw.jsonl 0 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
builtins-benchmark.validator.stderr 0 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
builtins-benchmark.disposition.json 974 e722148752733cb16cbfa1534c7bc10d048cea31182ea58c8af4eb1627ee44ce
EOF
for name in builtins-benchmark.jsonl builtins-benchmark.prelaunch.disposition.json; do
    [[ ! -e "target/issue35/$name" && ! -L "target/issue35/$name" ]] ||
        fail "forbidden Issue-058 artifact appeared after gates: $name"
done

temporary_binary="$(mktemp "$artifact_directory/.miso_engine_builtins_bench.XXXXXX")"
cp -- "$built_binary" "$temporary_binary"
chmod 0755 "$temporary_binary"
mv -n -- "$temporary_binary" "$sealed_binary"
[[ ! -e "$temporary_binary" && -x "$sealed_binary" && ! -L "$sealed_binary" &&
   "$(stat -c %h "$sealed_binary")" == 1 ]] || fail 'binary publication failed'
binary_sha256="$(hash_file "$sealed_binary")"

temporary_seal="$(mktemp "$artifact_directory/.builtins-benchmark.preflight.XXXXXX")"
jq -n -S \
    --arg commit "$candidate_commit" --arg tree "$candidate_tree" --arg lock "$lock_sha256" \
    --arg source "$tool_source_sha256" --arg binary "$binary_sha256" \
    --arg runner "$runner_sha256" --arg preflight "$preflight_script_sha256" \
    --arg lifecycle "$lifecycle_sha256" --arg record "$record_validator_sha256" \
    --arg aggregate "$aggregate_validator_sha256" --arg nonbenchmark "$nonbenchmark_sha256" \
    --arg manifest "$manifest_sha256" --arg pcm "$graph_pcm_sha256" --arg meter "$graph_meter_sha256" \
    '{schema_version:2,issue:72,kind:"builtins_benchmark_preflight",
      candidate_commit:$commit,candidate_tree:$tree,cargo_lock_sha256:$lock,
      tool_source_sha256:$source,binary_sha256:$binary,runner_sha256:$runner,
      preflight_script_sha256:$preflight,lifecycle_sha256:$lifecycle,
      record_validator_sha256:$record,aggregate_validator_sha256:$aggregate,
      nonbenchmark_seal_sha256:$nonbenchmark,fixture_manifest_sha256:$manifest,
      graph_pcm_sha256:$pcm,graph_meter_sha256:$meter,records_required:20,
      warmup_passes:1,measured_rounds:2,preflight_invocations:1,runner_invocations:0,
      workload_invocations:0,timed_benchmark_invocations:0}' >"$temporary_seal"
mv -n -- "$temporary_seal" "$seal"
[[ ! -e "$temporary_seal" && -f "$seal" && ! -L "$seal" &&
   "$(stat -c %h "$seal")" == 1 ]] || fail 'preflight seal publication failed'
for path in "$raw_output" "$accepted_output" "$stderr_output" "$prelaunch_disposition" "$disposition"; do
    [[ ! -e "$path" && ! -L "$path" ]] || fail "runtime artifact appeared: $path"
done
printf 'Issue-072 builtin benchmark preflight: PASS (preflight/runner/workload/timed=1/0/0/0)\n'
