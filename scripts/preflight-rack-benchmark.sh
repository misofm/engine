#!/usr/bin/env bash
# Builds, seals and validates Issue-038 inputs without launching a timed workload.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
[[ -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    printf 'Issue-038 preflight requires a clean committed candidate\n' >&2
    exit 1
}
for path in \
    artifacts/issue038/rack-benchmark.raw.jsonl \
    artifacts/issue038/rack-benchmark.accepted.jsonl \
    artifacts/issue038/rack-benchmark.stderr.log \
    artifacts/issue038/rack-benchmark.disposition.json; do
    [[ ! -e "$path" ]] || { printf 'Issue-038 artifact already exists: %s\n' "$path" >&2; exit 1; }
done
bash scripts/test-rack-benchmark.sh
bash scripts/check-rack-benchmark-fixture.sh
[[ "$(uname -m)" == "x86_64" ]] || { printf 'Issue-038 qualification requires x86_64\n' >&2; exit 1; }
grep -qm1 -w avx2 /proc/cpuinfo || { printf 'Issue-038 qualification requires AVX2\n' >&2; exit 1; }
cargo test --locked --quiet -p miso-engine-rack-bench -- --test-threads=1
export CARGO_PROFILE_RELEASE_OPT_LEVEL=3
export CARGO_PROFILE_RELEASE_LTO=false
export CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16
cargo build --locked --release --quiet -p miso-engine-rack-bench
binary=target/release/miso_engine_rack_bench
[[ -x "$binary" ]] || { printf 'missing rack benchmark binary\n' >&2; exit 1; }
candidate_commit=$(git rev-parse --verify HEAD)
candidate_commit_sha256=$(printf '%s' "$candidate_commit" | sha256sum | awk '{print $1}')
printf '{"schema_version":2,"issue":38,"kind":"rack_benchmark_preflight","workload_launches":0,"warmup_rounds":1,"measured_rounds":2,"records_required":6,"candidate_commit":"%s","candidate_commit_sha256":"%s","binary_sha256":"%s","benchmark_source_sha256":"%s","runner_sha256":"%s","record_validator_sha256":"%s","aggregate_validator_sha256":"%s","validator_library_sha256":"%s","fixture_manifest_sha256":"%s"}\n' \
    "$candidate_commit" "$candidate_commit_sha256" \
    "$(sha256sum "$binary" | awk '{print $1}')" \
    "$(sha256sum tools/miso-engine-rack-bench/src/main.rs | awk '{print $1}')" \
    "$(sha256sum scripts/run-rack-benchmark.sh | awk '{print $1}')" \
    "$(sha256sum scripts/rack-benchmark-record-validator.jq | awk '{print $1}')" \
    "$(sha256sum scripts/rack-benchmark-validator.jq | awk '{print $1}')" \
    "$(sha256sum scripts/rack-benchmark-record-lib.jq | awk '{print $1}')" \
    "$(sha256sum fixtures/rack/issue038-v1/MANIFEST.tsv | awk '{print $1}')"
