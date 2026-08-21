#!/usr/bin/env bash
# Builds and seals benchmark inputs without invoking the timing workload.
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
cd "$workspace_dir"
bash scripts/check-builtins-fixtures.sh
bash scripts/test-builtins-benchmark.sh
cargo test --locked -p miso-engine-builtins-compiler --features test-support phase_two_allocator_layouts_match_the_checked_resource_report
cargo build --locked --release -p miso-engine-builtins-bench
binary=target/release/miso_engine_builtins_bench
[[ -x "$binary" ]] || { printf 'missing builtins benchmark binary\n' >&2; exit 1; }
printf '{"schema_version":2,"issue":7,"kind":"builtins_benchmark_preflight","workload_launches":0,"records_required":20,"rates_hz":[48000,96000],"workloads":["full_chain_filters","identity_chain","matrix_ramp","meter_success_full","prepare_256_tracks"],"internal_rounds":2,"binary_sha256":"%s","input_manifest_sha256":"%s"}\n' \
  "$(sha256sum "$binary" | awk '{print $1}')" \
  "$(sha256sum fixtures/builtins/v1/MANIFEST.tsv | awk '{print $1}')"
