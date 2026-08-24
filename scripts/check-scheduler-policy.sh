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
scheduler_sources=(
    crates/miso-engine-native-scheduler/src/lib.rs
    crates/miso-engine-native-scheduler/src/platform/native.rs
    crates/miso-engine-native-scheduler/src/platform/browser.rs
)
for source in "${scheduler_sources[@]}" crates/miso-engine-graph/src/lib.rs \
    crates/miso-engine-graph/src/runtime.rs; do
    [[ -f "$source" ]] || fail "missing marked source $source"
    begins=$(rg -c 'REALTIME_POLICY_BEGIN' "$source" || true)
    ends=$(rg -c 'REALTIME_POLICY_END' "$source" || true)
    [[ "$begins" == "$ends" && "$begins" -ge 1 ]] || fail "$source has unmatched realtime markers"
    # Records the enclosing function name with every marked line, so the issue-100 wake rules can
    # be checked by owner rather than by count alone.
    awk '
        {
            if (match($0, /fn [a-z_0-9]+/)) {
                owner = substr($0, RSTART + 3, RLENGTH - 3)
            }
        }
        /REALTIME_POLICY_BEGIN/ { inside = 1; next }
        /REALTIME_POLICY_END/ { inside = 0; next }
        inside { print FILENAME ":" FNR ":" owner ":" $0 }
    ' "$source" >>"$marked"
done

# Issue 100: `park`/`unpark` are the one permitted render-thread syscall pair and are checked by
# owner below, so they leave this list. Everything else stays forbidden.
if rg -n \
    'Vec::|vec!|Box::|String::|[.]to_vec[(]|[.]collect[(]|Arc::clone|Rc::clone|drop[(]|Mutex|RwLock|Condvar|mpsc|sync_channel|thread::spawn|Builder::new|sleep[(]|yield_now|available_parallelism|target_capabilities|is_[a-z0-9_]*_feature_detected|Instant|SystemTime|std::fs|std::net|println!|eprintln!|format!|log::|tracing::|panic!|unreachable!' \
    "$marked"; then
    fail 'render-reachable scheduler code contains a forbidden operation surface'
fi

# Issue 100 wake protocol: the coordinator issues at most one `unpark` per block, from
# `wake_root`; auxiliary workers wake their own tree children from `wake_children`; nothing else
# in a marked region may unpark, and only `worker_loop` may park.
unpark_owners=$(rg -N '[.]unpark[(][)]' "$marked" | awk -F: '{ print $3 }' | sort | uniq -c |
    awk '{ print $2 "=" $1 }' | sort | tr '\n' ' ')
[[ "$unpark_owners" == 'wake_children=1 wake_root=1 ' ]] ||
    fail "marked regions must unpark exactly once from wake_root and once from wake_children (saw: ${unpark_owners:-none})"

park_owners=$(rg -N 'thread::park[(][)]' crates/miso-engine-native-scheduler/src --glob '*.rs' -l |
    sort | tr '\n' ' ')
[[ "$park_owners" == 'crates/miso-engine-native-scheduler/src/platform/native.rs ' ]] ||
    fail "thread::park belongs to the native worker loop alone (saw: ${park_owners:-none})"
park_context=$(awk '
    { if (match($0, /fn [a-z_0-9]+/)) { owner = substr($0, RSTART + 3, RLENGTH - 3) } }
    /thread::park[(][)]/ { print owner }
' crates/miso-engine-native-scheduler/src/platform/native.rs | sort -u | tr '\n' ' ')
[[ "$park_context" == 'worker_loop ' ]] ||
    fail "thread::park belongs to worker_loop alone (saw: ${park_context:-none})"

# Issue 100: bounded fault injection is a dev-only feature. It may be declared by the scheduler's
# own [features] table and requested only under a dev-dependencies heading, so it can never reach
# a host, C-ABI or Wasm artifact.
fault_violations=$(awk '
    /^\[/ {
        section = $0
        dev = (section ~ /dev-dependencies\]$/)
        features = (section == "[features]")
        next
    }
    /fault-injection/ {
        if (!dev && !(features && FILENAME ~ /native-scheduler/)) {
            print FILENAME ":" FNR ":" $0
        }
    }
' crates/*/Cargo.toml tools/*/Cargo.toml hosts/*/Cargo.toml)
[[ -z "$fault_violations" ]] || {
    printf '%s\n' "$fault_violations" >&2
    fail 'the fault-injection feature may only be requested from dev-dependencies'
}

if rg -n '\b(MAX_TRACKS|MAX_TRACK_COUNT|DEFAULT_MAX_TRACKS|TRACK_LIMIT)\b' \
    crates/miso-engine-native-scheduler crates/miso-engine-graph; then
    fail 'compiled track ceiling is forbidden'
fi

printf 'scheduler policy: PASS\n'
