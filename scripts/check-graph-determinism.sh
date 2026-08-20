#!/usr/bin/env bash
set -euo pipefail

workspace_dir=$(cd "$(dirname "$0")/.." && pwd)
binary="$workspace_dir/target/debug/miso_engine_graph_fixture"
evidence_dir="$workspace_dir/target/issue6"
evidence_file="$evidence_dir/fresh-process-determinism.json"

cargo build --quiet --locked --manifest-path "$workspace_dir/Cargo.toml" \
  -p miso-engine-graph-compiler --bin miso_engine_graph_fixture

baseline=$($binary)
for process_index in $(seq 2 100); do
  candidate=$($binary)
  if [[ "$candidate" != "$baseline" ]]; then
    printf 'graph determinism mismatch in fresh process %s\n' "$process_index" >&2
    exit 1
  fi
done

mkdir -p "$evidence_dir"
printf '%s\n' "$baseline" > "$evidence_file"
printf 'graph fresh-process determinism: PASS (100/100)\n'
