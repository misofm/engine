#!/usr/bin/env bash
set -euo pipefail

workspace_root="${1:-.}"
cd "$workspace_root"

fail() {
    printf 'scheduler policy failure: %s\n' "$1" >&2
    exit 1
}

scheduler_manifest="crates/miso-engine-native-scheduler/Cargo.toml"
graph_manifest="crates/miso-engine-graph/Cargo.toml"
[[ -f "$scheduler_manifest" && -f "$graph_manifest" ]] || fail 'missing scheduler/graph manifests'

scheduler_dependencies=$(awk '
    /^\[dependencies\]$/ { in_dependencies = 1; next }
    /^\[/ { in_dependencies = 0 }
    in_dependencies && /^[a-zA-Z0-9_-]+[.]workspace/ { print $1 }
' "$scheduler_manifest" | sort)
[[ "$scheduler_dependencies" == 'miso-engine-core.workspace' ]] ||
    fail 'native scheduler must depend only on core'

rg -q '^\[target[.]'"'"'cfg[(]not[(]target_arch = "wasm32"[)][)]'"'"'[.]dependencies\]$' "$graph_manifest" ||
    fail 'graph scheduler dependency must be excluded from Wasm'
rg -q '^miso-engine-native-scheduler[.]workspace = true$' "$graph_manifest" ||
    fail 'graph native dependency-wave seam is missing'

reverse_dependencies=$(rg -l '^miso-engine-native-scheduler([.]workspace)?[[:space:]]*=' crates/*/Cargo.toml | sort)
[[ "$reverse_dependencies" == "$graph_manifest" ]] ||
    fail 'only the render graph may depend on the native scheduler'

if rg -n 'unsafe[[:space:]]+(impl|fn)|unsafe[[:space:]]*\{' \
    crates/miso-engine-native-scheduler crates/miso-engine-graph --glob '*.rs'; then
    fail 'scheduler ownership split must not add unsafe code'
fi

marked=$(mktemp)
trap 'rm -f -- "$marked"' EXIT
for source in crates/miso-engine-native-scheduler/src/lib.rs crates/miso-engine-graph/src/lib.rs; do
    begins=$(rg -c 'REALTIME_POLICY_BEGIN' "$source" || true)
    ends=$(rg -c 'REALTIME_POLICY_END' "$source" || true)
    [[ "$begins" == "$ends" && "$begins" -ge 1 ]] || fail "$source has unmatched realtime markers"
    awk '
        /REALTIME_POLICY_BEGIN/ { inside = 1; next }
        /REALTIME_POLICY_END/ { inside = 0; next }
        inside { print FILENAME ":" FNR ":" $0 }
    ' "$source" >>"$marked"
done

if rg -n \
    'Vec::|vec!|Box::|String::|[.]to_vec[(]|[.]collect[(]|Arc::clone|Rc::clone|drop[(]|Mutex|RwLock|Condvar|mpsc|sync_channel|thread::|sleep[(]|yield_now|park[(]|unpark[(]|available_parallelism|target_capabilities|is_[a-z0-9_]*_feature_detected|Instant|SystemTime|std::fs|std::net|println!|eprintln!|format!|log::|tracing::|panic!|unreachable!' \
    "$marked"; then
    fail 'render-reachable scheduler code contains a forbidden operation surface'
fi

if rg -n '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' \
    crates/miso-engine-native-scheduler crates/miso-engine-graph; then
    fail 'compiled track ceiling is forbidden'
fi

printf 'scheduler policy: PASS\n'
