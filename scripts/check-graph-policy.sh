#!/usr/bin/env bash
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"

fail() {
  printf 'graph policy failure: %s\n' "$1" >&2
  exit 1
}

graph_manifest="crates/miso-engine-graph/Cargo.toml"
compiler_manifest="crates/miso-engine-graph-compiler/Cargo.toml"
[[ -f "$graph_manifest" && -f "$compiler_manifest" ]] || fail 'missing graph manifests'

production_graph_dependencies=$(awk '
  /^\[dependencies\]$/ { in_dependencies = 1; next }
  /^\[/ { in_dependencies = 0 }
  in_dependencies && /^[a-zA-Z0-9_-]+[.]workspace/ { print $1 }
' "$graph_manifest" | sort)
[[ "$production_graph_dependencies" == $'miso-engine-core.workspace\nmiso-engine-effect-contract.workspace' ]] ||
  fail 'render graph dependency boundary changed'

rg -q '^sha2[.]workspace = true$' "$compiler_manifest" ||
  fail 'control-plane compiler must own SHA-256 dependency'
if rg -n 'sha2|miso_engine_session|miso_engine_effect_compiler' \
  crates/miso-engine-graph/src crates/miso-engine-graph/Cargo.toml; then
  fail 'control-plane dependency leaked into render graph'
fi
if rg -n 'PlanPublisher|plan_exchange|std::fs|std::net|std::thread|std::sync|log::|tracing::' \
  crates/miso-engine-graph/src crates/miso-engine-graph-compiler/src; then
  fail 'publication, I/O, threading, synchronization, or logging leaked into graph path'
fi
if rg -n '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' \
  crates/miso-engine-graph crates/miso-engine-graph-compiler; then
  fail 'compiled track ceiling is forbidden'
fi
implementations=$(rg -l 'impl PreparedPlanExecutor for' crates --glob '*.rs' | sort)
[[ "$implementations" == 'crates/miso-engine-graph/src/lib.rs' ]] ||
  fail 'production prepared-plan executor must remain graph-owned'

printf 'graph policy: PASS\n'
