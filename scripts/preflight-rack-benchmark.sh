#!/usr/bin/env bash
# Validates Issue-008 benchmark readiness without launching a timed workload.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
cd "$workspace_dir"
bash scripts/test-rack-benchmark.sh
cargo build --locked --release -p miso-engine-rack-bench
binary=target/release/miso_engine_rack_bench
[[ -x "$binary" ]] || { printf 'missing rack benchmark binary\n' >&2; exit 1; }
printf '{"schema_version":1,"issue":8,"kind":"rack_benchmark_preflight","workload_launches":0,"warmup_rounds":1,"measured_rounds":2,"records_required":6,"binary_sha256":"%s"}\n' "$(sha256sum "$binary" | awk '{print $1}')"
