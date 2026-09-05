#!/usr/bin/env bash
set -euo pipefail

workspace_root="${1:-.}"
script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$script_directory/lib/gate.sh"
cd "$workspace_root"
GATE_FAILURE_PREFIX='graph policy failure'

fail() {
  printf 'graph policy failure: %s\n' "$1" >&2
  exit 1
}

graph_manifest="crates/graph/Cargo.toml"
compiler_manifest="crates/graph-compiler/Cargo.toml"
[[ -f "$graph_manifest" && -f "$compiler_manifest" ]] || fail 'missing graph manifests'

graph_dependencies_raw="$(awk '
  /^\[dependencies\]$/ { in_dependencies = 1; next }
  /^\[/ { in_dependencies = 0 }
  in_dependencies && /^[a-zA-Z0-9_-]+[.]workspace/ { print $1 }
' "$graph_manifest")" || { rc=$?; fail "graph dependency extraction failed (awk status $rc)"; }
production_graph_dependencies="$(gate_sort_lines 'graph dependency extraction' "$graph_dependencies_raw")" || exit $?
[[ "$production_graph_dependencies" == $'effect-contract.workspace\nengine.workspace\nlane.workspace\nrack.workspace' ]] ||
  fail 'render graph dependency boundary changed'

gate_scan_required 'control-plane compiler SHA-256 dependency' '^sha2[.]workspace = true$' '' "$compiler_manifest" >/dev/null || exit $?
gate_scan_forbidden 'control-plane dependency leaked into render graph' 'sha2|\b(session|effect_compiler)::' '' \
  crates/graph/src crates/graph/Cargo.toml || exit $?
graph_sources_raw="$(gate_find_collect 'graph source discovery' crates/graph/src crates/graph-compiler/src -name '*.rs' -type f)" || exit $?
[[ -n "$graph_sources_raw" ]] || fail 'graph source discovery returned no Rust files'
graph_sources="$(gate_sort_lines 'graph source discovery' "$graph_sources_raw")" || exit $?
production_sources=$(mktemp)
trap 'rm -f -- "$production_sources"' EXIT
while IFS= read -r source; do
  [[ -n "$source" ]] || continue
  stripped="$(sed '/^#\[cfg(test)\]/,$d' "$source")" || fail "graph source read failed: $source"
  printf '%s\n' "$stripped" >>"$production_sources"
done <<<"$graph_sources"
publication_matches="$(gate_scan_collect 'graph publication predicate' \
  'PlanPublisher|plan_exchange|std::fs|std::net|std::thread|std::sync::(Mutex|RwLock|Condvar|mpsc)|log::|tracing::' '' \
  "$production_sources")" || exit $?
if [[ -n "$publication_matches" ]]; then
  printf '%s\n' "$publication_matches" >&2
  fail 'publication, I/O, threading, synchronization, or logging leaked into graph path'
fi
# The MAX_TRACKS ban lives once, in scripts/check-workspace-policy.sh (P12): it scans the whole
# {crates,hosts,tools,sidecars} tree, graph/graph-compiler included, rather than one of five
# copies of the same regex over five different root lists.
# Production code only: a `#[cfg(test)]` module may implement the seam to exercise it (issue 100
# tests the block-boundary hand-over inside `engine`), but nothing that ships may.
all_sources_raw="$(gate_find_collect 'workspace Rust discovery' crates -name '*.rs' -type f)" || exit $?
[[ -n "$all_sources_raw" ]] || fail 'workspace Rust discovery returned no Rust files'
all_sources="$(gate_sort_lines 'workspace Rust discovery' "$all_sources_raw")" || exit $?
implementations_raw=''
while IFS= read -r source; do
    # No pipeline here: `rg -q` exits on its first match, and under `pipefail` sed's SIGPIPE
    # would make the whole condition read as false.
    stripped=$(sed '/^#\[cfg(test)\]/,$d' "$source") || fail "workspace source read failed: $source"
    executor_matches="$(gate_scan_text_collect "prepared-plan executor predicate for $source" \
      'impl PreparedPlanExecutor for' "$stripped")" || exit $?
    [[ -z "$executor_matches" ]] || implementations_raw+="$source"$'\n'
done <<<"$all_sources"
implementations_raw="${implementations_raw%$'\n'}"
implementations="$(gate_sort_lines 'prepared-plan executor owner aggregation' "$implementations_raw")" || exit $?
[[ "$implementations" == 'crates/graph/src/lib.rs' ]] ||
  fail 'production prepared-plan executor must remain graph-owned'

printf 'graph policy: PASS\n'
