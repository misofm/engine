#!/usr/bin/env bash
# Issue-149 console benchmark preflight: everything that can fail without launching the workload.
#
# AGENTS.md requires benchmark infrastructure to preflight arguments, schema, output persistence,
# shell exit semantics and overwrite refusal *before* the timed workload runs, so that a runner
# defect cannot consume the one authorised measurement. Nothing here is timed.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"

fail() { printf 'console preflight failure: %s\n' "$1" >&2; exit 1; }

artifact_dir="$root/artifacts/issue149"
for name in console-benchmark.raw.jsonl console-benchmark.accepted.jsonl \
    console-benchmark.stderr.log console-benchmark.disposition.json; do
    path="$artifact_dir/$name"
    [[ ! -e "$path" && ! -L "$path" ]] || fail "issue-149 artifact already exists: $path"
done

for tool in awk cmp cp git jq sha256sum wc; do
    command -v "$tool" >/dev/null 2>&1 || fail "required tool is unavailable: $tool"
done

bash scripts/check-console-benchmark-fixture.sh >/dev/null || fail 'fixture check failed'
bash scripts/test-console-benchmark.sh >/dev/null || fail 'validator mutation suite failed'

cargo test --locked -p miso-engine-bench >/dev/null || fail 'bench crate tests failed'
cargo clippy --locked -p miso-engine-bench --all-targets --all-features -- -D warnings >/dev/null 2>&1 ||
    fail 'bench crate clippy failed'
cargo build --locked --release --quiet -p miso-engine-bench || fail 'release build failed'

binary="$root/target/release/miso_engine_bench"
[[ -x "$binary" ]] || fail 'release binary is missing'
# The subject refuses to run without its runner's round marker. Proving that here means a direct
# invocation cannot quietly produce an unprovenanced record later.
if MISO_ENGINE_BENCH_ROUND= "$binary" console >/dev/null 2>&1; then
    fail 'the console subject accepted an empty round marker'
fi
if "$binary" console extra-argument >/dev/null 2>&1; then
    fail 'the console subject accepted an argument'
fi

candidate_commit=$(git rev-parse --verify HEAD)
jq -n -S \
    --arg commit "$candidate_commit" \
    --arg commit_sha256 "$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')" \
    --arg binary_sha256 "$(sha256sum "$binary" | awk '{print $1}')" \
    --arg subject_sha256 "$(sha256sum tools/miso-engine-bench/src/console.rs | awk '{print $1}')" \
    --arg fixture_sha256 "$(sha256sum fixtures/session/v1/console-sixty-four-track.toml | awk '{print $1}')" \
    --arg runner_sha256 "$(sha256sum scripts/run-console-benchmark.sh | awk '{print $1}')" \
    --arg record_validator_sha256 "$(sha256sum scripts/console-benchmark-record-validator.jq | awk '{print $1}')" \
    --arg aggregate_validator_sha256 "$(sha256sum scripts/console-benchmark-validator.jq | awk '{print $1}')" \
    --arg library_sha256 "$(sha256sum scripts/console-benchmark-record-lib.jq | awk '{print $1}')" \
    '{schema_version: 1, issue: 149, kind: "console_benchmark_preflight",
      workload_launches: 0, warmup_rounds: 1, measured_rounds: 2, records_required: 12,
      candidate_commit: $commit, candidate_commit_sha256: $commit_sha256,
      binary_sha256: $binary_sha256, benchmark_source_sha256: $subject_sha256,
      fixture_sha256: $fixture_sha256, runner_sha256: $runner_sha256,
      record_validator_sha256: $record_validator_sha256,
      aggregate_validator_sha256: $aggregate_validator_sha256,
      validator_library_sha256: $library_sha256}'

printf 'console benchmark preflight: PASS (workload launches 0)\n' >&2
