#!/usr/bin/env bash
# Builds, seals and validates Issue-038 inputs without launching a timed workload.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
bash scripts/test-rack-benchmark.sh
[[ "$(uname -m)" == "x86_64" ]] || { printf 'Issue-038 qualification requires x86_64\n' >&2; exit 1; }
grep -qm1 -w avx2 /proc/cpuinfo || { printf 'Issue-038 qualification requires AVX2\n' >&2; exit 1; }
cargo build --locked --release --quiet -p miso-engine-rack-bench
binary=target/release/miso_engine_rack_bench
[[ -x "$binary" ]] || { printf 'missing rack benchmark binary\n' >&2; exit 1; }
[[ "$(sha256sum fixtures/rack/issue038-v1/workloads.toml | awk '{print $1}')" == "1f67ed9960e5a6728f02442b65af70704957d5f6056865d8b44555637273188d" ]] || { printf 'frozen workload fixture hash mismatch\n' >&2; exit 1; }
printf '{"schema_version":2,"issue":38,"kind":"rack_benchmark_preflight","workload_launches":0,"warmup_rounds":1,"measured_rounds":2,"records_required":6,"candidate_commit":"%s","binary_sha256":"%s","fixture_manifest_sha256":"%s"}\n' "$(git rev-parse HEAD)" "$(sha256sum "$binary" | awk '{print $1}')" "$(sha256sum fixtures/rack/issue038-v1/MANIFEST.tsv | awk '{print $1}')"
