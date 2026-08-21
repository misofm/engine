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
rg -q 'into_bound_native' tools/miso-engine-scheduler-bench/src/main.rs
rg -q 'prepared_builtin_bank_count' tools/miso-engine-scheduler-bench/src/main.rs
cargo test --locked --quiet -p miso-engine-scheduler-bench
cargo build --locked --release --quiet -p miso-engine-scheduler-bench
binary=target/release/miso_engine_scheduler_bench
[[ -x "$binary" ]] || { printf 'missing scheduler benchmark binary\n' >&2; exit 1; }
printf '{"schema_version":1,"issue":9,"kind":"scheduler_benchmark_preflight","workload_launches":0,"warmup_rounds":1,"measured_rounds":2,"records_required":6,"candidate":"%s","binary_sha256":"%s","source_sha256":"%s","runner_sha256":"%s","record_validator_sha256":"%s","aggregate_validator_sha256":"%s"}\n' \
    "$(git rev-parse --verify HEAD)" "$(sha256sum "$binary" | awk '{print $1}')" \
    "$(sha256sum tools/miso-engine-scheduler-bench/src/main.rs | awk '{print $1}')" \
    "$(sha256sum scripts/run-scheduler-benchmark.sh | awk '{print $1}')" \
    "$(sha256sum scripts/scheduler-benchmark-record-lib.jq scripts/scheduler-benchmark-record-validator.jq | sha256sum | awk '{print $1}')" \
    "$(sha256sum scripts/scheduler-benchmark-validator.jq | awk '{print $1}')"
