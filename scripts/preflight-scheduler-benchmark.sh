#!/usr/bin/env bash
# Compile and seal the Issue-009 benchmark candidate without launching timed audio.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || { printf 'Issue-009 preflight requires a clean candidate\n' >&2; exit 1; }
for path in artifacts/issue009/scheduler-benchmark.raw.jsonl artifacts/issue009/scheduler-benchmark.accepted.jsonl artifacts/issue009/scheduler-benchmark.stderr.log artifacts/issue009/scheduler-benchmark.disposition.json; do
    [[ ! -e "$path" ]] || { printf 'Issue-009 artifact already exists: %s\n' "$path" >&2; exit 1; }
done
bash scripts/test-scheduler-benchmark.sh
rg -q 'into_bound_native' tools/miso-engine-bench/src/scheduler.rs
rg -q 'prepared_builtin_bank_count' tools/miso-engine-bench/src/scheduler.rs
cargo test --locked --quiet -p miso-engine-bench
cargo build --locked --release --quiet -p miso-engine-bench
binary=target/release/miso_engine_bench
[[ -x "$binary" ]] || { printf 'missing scheduler benchmark binary\n' >&2; exit 1; }
candidate_commit=$(git rev-parse --verify HEAD)
candidate_sha256=$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')
printf '{"schema_version":2,"issue":39,"kind":"scheduler_benchmark_preflight","workload_launches":0,"warmup_rounds":1,"measured_rounds":2,"records_required":6,"candidate_commit":"%s","candidate_sha256":"%s","binary_sha256":"%s","source_sha256":"%s","fixture_sha256":"%s","runner_sha256":"%s","validator_library_sha256":"%s","record_validator_sha256":"%s","aggregate_validator_sha256":"%s"}\n' \
    "$candidate_commit" "$candidate_sha256" "$(sha256sum "$binary" | awk '{print $1}')" \
    "$(sha256sum tools/miso-engine-bench/src/scheduler.rs | awk '{print $1}')" \
    "$(sha256sum fixtures/session/v1/canonical.toml | awk '{print $1}')" \
    "$(sha256sum scripts/run-scheduler-benchmark.sh | awk '{print $1}')" \
    "$(sha256sum scripts/scheduler-benchmark-record-lib.jq | awk '{print $1}')" \
    "$(sha256sum scripts/scheduler-benchmark-record-validator.jq | awk '{print $1}')" \
    "$(sha256sum scripts/scheduler-benchmark-validator.jq | awk '{print $1}')"
