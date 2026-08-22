#!/usr/bin/env bash
# Sole Issue-109 zero-workload preflight; builds but never executes the benchmark.
set -euo pipefail
[[ $# == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "$script_directory/.." && pwd)"
cd "$root"
artifact_directory="$root/target/issue109"
repair_seal="$artifact_directory/metadata-repair.seal.json"
binary="$artifact_directory/miso_engine_builtins_bench"
seal="$artifact_directory/builtins-benchmark.preflight.json"
runtime_paths=(
    "$artifact_directory/builtins-benchmark.raw.jsonl"
    "$artifact_directory/builtins-benchmark.jsonl"
    "$artifact_directory/builtins-benchmark.validator.stderr"
    "$artifact_directory/builtins-benchmark.prelaunch.disposition.json"
    "$artifact_directory/builtins-benchmark.disposition.json"
)
for tool in awk bash cargo cat chmod cp diff dirname find git grep jq ln mkdir mktemp mv rg rm sed \
    sha256sum stat tr wc; do
    command -v "$tool" >/dev/null 2>&1 || { printf 'Issue-109 preflight tool unavailable: %s\n' "$tool" >&2; exit 1; }
done
hash_file() { sha256sum "$1" | awk '{print $1}'; }
require_one_link() { [[ -f "$1" && ! -L "$1" && "$(stat -c %h "$1")" == 1 ]]; }
fail() { printf 'Issue-109 preflight failure: %s\n' "$1" >&2; exit 1; }
verify_issue072() {
    local name bytes digest path
    while read -r name bytes digest; do
        path="target/issue72/$name"
        require_one_link "$path" && [[ "$(wc -c <"$path")" == "$bytes" ]] &&
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
    [[ ! -e target/issue72/builtins-benchmark.prelaunch.disposition.json &&
       ! -L target/issue72/builtins-benchmark.prelaunch.disposition.json ]]
}

branch="$(git branch --show-current)"
commit="$(git rev-parse --verify HEAD)"
tree="$(git rev-parse 'HEAD^{tree}')"
[[ "$branch" == codex/batch-benchmark-109 ]] || fail 'candidate branch'
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || fail 'dirty candidate'
mkdir -p "$artifact_directory"
umask 077
[[ ! -L "$artifact_directory" ]] || fail 'artifact directory symlink'
require_one_link "$repair_seal" || fail 'repair seal unavailable'
for path in "$binary" "$seal" "${runtime_paths[@]}"; do
    [[ ! -e "$path" && ! -L "$path" ]] || fail "refusing existing artifact: $path"
done
verify_issue072 || fail 'Issue-072 evidence changed'
[[ "$(stat -c %i target/issue72/builtins-benchmark.raw.jsonl)" != \
   "$(stat -c %i target/issue72/builtins-benchmark.jsonl)" ]] || fail 'Issue-072 raw/accepted inode alias'

lock_sha="$(hash_file Cargo.lock)"
manifest_sha="$(hash_file fixtures/builtins/v1/MANIFEST.tsv)"
pcm_sha="$(hash_file fixtures/builtins/v1/pcm/graph-taps.f32le)"
meter_sha="$(hash_file fixtures/builtins/v1/meters/graph-taps.jsonl)"
tool_manifest_sha="$(hash_file tools/miso-engine-builtins-bench/Cargo.toml)"
tool_source_sha="$(hash_file tools/miso-engine-builtins-bench/src/main.rs)"
record_sha="$(hash_file scripts/builtins-benchmark-record-validator.jq)"
aggregate_sha="$(hash_file scripts/builtins-benchmark-validator.jq)"
runner_sha="$(hash_file scripts/run-builtins-benchmark-109.sh)"
preflight_sha="$(hash_file scripts/preflight-builtins-benchmark-109.sh)"
lifecycle_sha="$(hash_file scripts/test-builtins-benchmark-109.sh)"
checker_sha="$(hash_file scripts/check-builtins-benchmark-109.sh)"
mutation_sha="$(hash_file scripts/test-builtins-benchmark-109-policy.sh)"
repair_sha="$(hash_file "$repair_seal")"
issue068_source_sha=0c71b71d864fbdd01aa918c6825abea78c38f0486535bc914af92142a5080d19
rg -Fq "$issue068_source_sha" \
    .github/ISSUE_SPECS/068-builtin-native-aarch64-and-wasm-runtime-selection-and-instruction-qualification.md ||
    fail 'accepted Issue-068 source identity'

[[ "$lock_sha" == 4213efd775d1d1207fea805ccdc01392acb015ae36d1bf2eba783f938f19916a &&
   "$manifest_sha" == bfcc7bbe66ab4a643a3969048d9ad4660111874fcd4316c23645db1e7c1eafff &&
   "$pcm_sha" == 508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19 &&
   "$meter_sha" == 958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f &&
   "$tool_manifest_sha" == f361c26b6a59c984a9fc60484748b5a2fd0bd0c35079e83ee72d3932f118cf97 &&
   "$tool_source_sha" == b520e3d14bd4fa2985d18f273e515261a53b4ea69ac1a2a38aba9bc77bf6e7fe &&
   "$record_sha" == c3db1d9574360bdab0d9ac335615787446e5537439d6accdded4fdd0a4479467 &&
   "$aggregate_sha" == 6085e740f15d7902fca4443d761cfb8e29df7168ba12f632c7946db56a3e1b63 ]] ||
    fail 'frozen authority changed'

jq -e --arg branch "$branch" --arg commit "$commit" --arg tree "$tree" --arg lock "$lock_sha" \
    --arg manifest "$manifest_sha" --arg pcm "$pcm_sha" --arg meter "$meter_sha" \
    --arg tm "$tool_manifest_sha" --arg ts "$tool_source_sha" --arg rv "$record_sha" --arg av "$aggregate_sha" \
    --arg runner "$runner_sha" --arg preflight "$preflight_sha" --arg lifecycle "$lifecycle_sha" \
    --arg checker "$checker_sha" --arg mutation "$mutation_sha" --arg issue068 "$issue068_source_sha" \
    'type=="object" and keys==["accepted_issue068_source_sha256","aggregate_validator_sha256","branch","candidate_commit","candidate_tree","cargo_lock_sha256","checker_sha256","fixture_manifest_sha256","graph_meter_sha256","graph_pcm_sha256","issue","issue072_artifacts","kind","lifecycle_sha256","metadata_regressions","mutation_sha256","preflight_invocations","preflight_sha256","record_validator_sha256","runner_invocations","runner_sha256","schema_version","timed_benchmark_invocations","tool_manifest_sha256","tool_source_sha256","workload_invocations"] and
     .schema_version==1 and .issue==109 and .kind=="builtins_benchmark_metadata_repair" and
     .branch==$branch and .candidate_commit==$commit and .candidate_tree==$tree and
     .cargo_lock_sha256==$lock and .fixture_manifest_sha256==$manifest and .graph_pcm_sha256==$pcm and
     .graph_meter_sha256==$meter and .tool_manifest_sha256==$tm and .tool_source_sha256==$ts and
     .record_validator_sha256==$rv and .aggregate_validator_sha256==$av and .runner_sha256==$runner and
     .preflight_sha256==$preflight and .lifecycle_sha256==$lifecycle and .checker_sha256==$checker and
     .mutation_sha256==$mutation and .accepted_issue068_source_sha256==$issue068 and
     .metadata_regressions==1 and
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
     .timed_benchmark_invocations==0' "$repair_seal" >/dev/null || fail 'repair seal mismatch'

scratch="$(mktemp -d "${TMPDIR:-/tmp}/miso-engine-issue109-preflight.XXXXXX")"
trap 'rm -rf "$scratch"' EXIT
bash scripts/check-builtins-benchmark-109.sh
bash scripts/test-builtins-benchmark-109-policy.sh
bash scripts/test-builtins-benchmark-109.sh
cargo check --locked -p miso-engine-builtins-bench --all-targets
CARGO_TARGET_DIR="$scratch/build" cargo build --locked --release -p miso-engine-builtins-bench
built="$scratch/build/release/miso_engine_builtins_bench"
[[ -x "$built" && ! -L "$built" ]] || fail 'release binary unavailable'
[[ "$commit" == "$(git rev-parse --verify HEAD)" && "$tree" == "$(git rev-parse 'HEAD^{tree}')" &&
   -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || fail 'candidate drifted'
verify_issue072 || fail 'Issue-072 evidence changed after gates'
[[ "$(stat -c %i target/issue72/builtins-benchmark.raw.jsonl)" != \
   "$(stat -c %i target/issue72/builtins-benchmark.jsonl)" ]] || fail 'Issue-072 inode alias after gates'
[[ "$repair_sha" == "$(hash_file "$repair_seal")" ]] || fail 'repair seal changed'

temporary_binary="$(mktemp "$artifact_directory/.miso_engine_builtins_bench.XXXXXX")"
cp "$built" "$temporary_binary"
chmod 0755 "$temporary_binary"
mv -n "$temporary_binary" "$binary"
[[ ! -e "$temporary_binary" && -x "$binary" && ! -L "$binary" && "$(stat -c %h "$binary")" == 1 ]] ||
    fail 'binary publication'
binary_sha="$(hash_file "$binary")"
temporary_seal="$(mktemp "$artifact_directory/.builtins-benchmark.preflight.XXXXXX")"
jq -n -S --arg commit "$commit" --arg tree "$tree" --arg binary "$binary_sha" --arg repair "$repair_sha" \
    --arg lock "$lock_sha" --arg manifest "$manifest_sha" --arg pcm "$pcm_sha" --arg meter "$meter_sha" \
    --arg tm "$tool_manifest_sha" --arg ts "$tool_source_sha" --arg rv "$record_sha" --arg av "$aggregate_sha" \
    --arg runner "$runner_sha" --arg preflight "$preflight_sha" --arg lifecycle "$lifecycle_sha" \
    --arg checker "$checker_sha" --arg mutation "$mutation_sha" --arg issue068 "$issue068_source_sha" \
    '{schema_version:1,issue:109,kind:"builtins_benchmark_preflight",
      accepted_issue068_source_sha256:$issue068,candidate_commit:$commit,
      candidate_tree:$tree,binary_sha256:$binary,repair_seal_sha256:$repair,cargo_lock_sha256:$lock,
      fixture_manifest_sha256:$manifest,graph_pcm_sha256:$pcm,graph_meter_sha256:$meter,
      tool_manifest_sha256:$tm,tool_source_sha256:$ts,record_validator_sha256:$rv,
      aggregate_validator_sha256:$av,runner_sha256:$runner,preflight_sha256:$preflight,
      lifecycle_sha256:$lifecycle,checker_sha256:$checker,mutation_sha256:$mutation,
      records_required:20,warmup_passes:1,measured_rounds:2,preflight_invocations:1,
      runner_invocations:0,workload_invocations:0,timed_benchmark_invocations:0}' >"$temporary_seal"
mv -n "$temporary_seal" "$seal"
[[ ! -e "$temporary_seal" && -f "$seal" && ! -L "$seal" && "$(stat -c %h "$seal")" == 1 ]] ||
    fail 'preflight seal publication'
for path in "${runtime_paths[@]}"; do [[ ! -e "$path" && ! -L "$path" ]] || fail 'runtime artifact appeared'; done
printf 'Issue-109 preflight: PASS counters=1/0/0/0\n'
