#!/usr/bin/env bash
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"

fail() {
  printf 'graph policy failure: %s\n' "$1" >&2
  exit 1
}

graph_manifest="crates/graph/Cargo.toml"
compiler_manifest="crates/graph-compiler/Cargo.toml"
[[ -f "$graph_manifest" && -f "$compiler_manifest" ]] || fail 'missing graph manifests'

production_graph_dependencies=$(awk '
  /^\[dependencies\]$/ { in_dependencies = 1; next }
  /^\[/ { in_dependencies = 0 }
  in_dependencies && /^[a-zA-Z0-9_-]+[.]workspace/ { print $1 }
' "$graph_manifest" | sort)
[[ "$production_graph_dependencies" == $'effect-contract.workspace\nengine.workspace\nlane.workspace\nrack.workspace' ]] ||
  fail 'render graph dependency boundary changed'

rg -q '^sha2[.]workspace = true$' "$compiler_manifest" ||
  fail 'control-plane compiler must own SHA-256 dependency'
if rg -n 'sha2|\b(session|effect_compiler)::' \
  crates/graph/src crates/graph/Cargo.toml; then
  fail 'control-plane dependency leaked into render graph'
fi
production_sources=$(mktemp)
trap 'rm -f -- "$production_sources"' EXIT
while IFS= read -r source; do
  sed '/^#\[cfg(test)\]/,$d' "$source" >>"$production_sources"
done < <(find crates/graph/src crates/graph-compiler/src -name '*.rs' -type f | sort)
if rg -n 'PlanPublisher|plan_exchange|std::fs|std::net|std::thread|std::sync::(Mutex|RwLock|Condvar|mpsc)|log::|tracing::' \
  "$production_sources"; then
  fail 'publication, I/O, threading, synchronization, or logging leaked into graph path'
fi
if rg -n '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' \
  crates/graph crates/graph-compiler; then
  fail 'compiled track ceiling is forbidden'
fi
# Production code only: a `#[cfg(test)]` module may implement the seam to exercise it (issue 100
# tests the block-boundary hand-over inside `engine`), but nothing that ships may.
implementations=$(
  while IFS= read -r source; do
    # No pipeline here: `rg -q` exits on its first match, and under `pipefail` sed's SIGPIPE
    # would make the whole condition read as false.
    stripped=$(sed '/^#\[cfg(test)\]/,$d' "$source")
    if rg -q 'impl PreparedPlanExecutor for' <<<"$stripped"; then
      printf '%s\n' "$source"
    fi
  done < <(find crates -name '*.rs' -type f | sort) | sort
)
[[ "$implementations" == 'crates/graph/src/lib.rs' ]] ||
  fail 'production prepared-plan executor must remain graph-owned'

printf 'graph policy: PASS\n'
