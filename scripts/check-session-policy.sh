#!/usr/bin/env bash
# Dependency-direction and preflight-order gate for canonical Session V1 JSON (issue #338).
#
# #1050 retired this gate's two TOML rules -- no live `*.toml` session outside a historical
# allowlist, and no retired TOML spelling anywhere in the tree -- with the allowlist itself. The
# format they guarded was deleted in #338; the dependency ban below (no `toml`/`serde` in the
# session crate) is what would have to break for it to come back.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
GATE_FAILURE_PREFIX='session policy'
source "$root/scripts/lib/gate.sh"
fail() { printf 'session policy: %s\n' "$1" >&2; exit 1; }
manifest=crates/session/Cargo.toml
source_dir=crates/session/src

forbid() {
    local description=$1 pattern=$2 path=$3 message=$4 output
    if ! output="$(gate_scan_collect "$description" "$pattern" '' "$path")"; then fail "$description search failed"; fi
    [[ -z "$output" ]] || fail "$message"
}
require() {
    local description=$1 pattern=$2 path=$3 message=$4 output
    if ! output="$(gate_scan_collect "$description" "$pattern" '' "$path")"; then fail "$description search failed"; fi
    [[ -n "$output" ]] || fail "$message"
}
forbid 'engine reverse dependency' '^session\.workspace = true$' crates/engine/Cargo.toml 'engine must not depend on session'
require 'session engine workspace dependency' '^engine\.workspace = true$' "$manifest" 'session must depend on engine'
require 'session json-syntax pin' '^json-syntax = \{ version = "=0\.12\.5", default-features = false \}$' "$manifest" 'session must exact-pin json-syntax 0.12.5 without default features'
forbid 'session TOML/serde dependency' '^[[:space:]]*(toml|serde)[[:space:]]*=' "$manifest" 'session runtime parser baggage returned'
forbid 'session publication API' 'use engine::.*(PreparedRenderPlan|PlanPublisher)|PlanPublisher<' "$source_dir" 'session may not import plan publication APIs'
forbid 'session estimate allocation vocabulary' 'format!|\.to_owned\(|\.to_string\(|String::with_capacity|Vec::with_capacity|\.collect::' "$source_dir/estimate.rs" 'successful resource preflight may not allocate temporary diagnostics or collections'

compile_source="$source_dir/compile.rs"
anchor() {
    local description=$1 pattern=$2 output first line
    if ! output="$(gate_scan_collect "$description" "$pattern" '' "$compile_source")"; then fail "$description search failed"; fi
    [[ -n "$output" ]] || fail "$description anchor missing"
    first=${output%%$'\n'*}; line=${first%%:*}
    [[ "$line" =~ ^[1-9][0-9]*$ ]] || fail "$description anchor line is not a positive decimal"
    printf '%s' "$line"
}
estimate_line=$(anchor estimate 'let estimate = estimate_session\(session\)')
caps_line=$(anchor caps 'check_caps\(session, estimate, caps\)')
validate_line=$(anchor validate 'validate_session\(session\)')
canonical_line=$(anchor canonical 'let canonical_json = write_canonical\(session\)')
clone_line=$(anchor clone 'let mut normalized = session\.clone\(\)')
(( estimate_line < caps_line && caps_line < validate_line && validate_line < canonical_line && canonical_line < clone_line )) || fail 'resource preflight/cap ordering changed'
printf 'session policy: ok\n'
