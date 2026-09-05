#!/usr/bin/env bash
# Sole-format and dependency-direction gate for canonical Session V1 JSON (issue #338).
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
GATE_FAILURE_PREFIX='session policy'
source "$root/scripts/lib/gate.sh"
fail() { printf 'session policy: %s\n' "$1" >&2; exit 1; }
manifest=crates/session/Cargo.toml
source_dir=crates/session/src
allowlist=scripts/session-policy-historical-allowlist.txt
[[ -f "$allowlist" ]] || fail 'missing explicit historical allowlist'

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

allowlist_text=; if allowlist_text="$(sed -e '/^[[:space:]]*#/d' -e '/^[[:space:]]*$/d' "$allowlist" 2>&1)"; then allowlist_rc=0; else allowlist_rc=$?; fi
[[ "$allowlist_rc" == 0 ]] || fail "historical allowlist read failed (sed status $allowlist_rc)"
mapfile -t historical < <(printf '%s\n' "$allowlist_text")
is_historical() { local path=$1 entry; for entry in "${historical[@]}"; do [[ "$path" == "$entry" ]] && return 0; done; return 1; }

find_text=
if ! value="$(gate_find_collect 'fixtures/session find' fixtures/session -type f -name '*.toml')"; then fail 'fixtures/session find failed'; fi; [[ -n "$value" ]] && find_text+="$value"$'\n'
if ! value="$(gate_find_collect 'fixtures/native-pcm-runner find' fixtures/native-pcm-runner -type f -name '*.toml')"; then fail 'fixtures/native-pcm-runner find failed'; fi; [[ -n "$value" ]] && find_text+="$value"$'\n'
if ! value="$(gate_find_collect 'host qualification find' hosts/host-web/qualification hosts/host-web/tests/browser-v1 -type f -name '*.toml')"; then fail 'host qualification find failed'; fi; [[ -n "$value" ]] && find_text+="$value"$'\n'
if ! value="$(gate_find_collect 'sdk/fuzz session find' sdk fuzz -type f \( -name '*.session.toml' -o -path '*/session_*/*.toml' \))"; then fail 'sdk/fuzz session find failed'; fi; [[ -n "$value" ]] && find_text+="$value"$'\n'
if ! sorted="$(gate_sort_lines 'session TOML discovery' "$find_text")"; then fail 'session TOML discovery sort failed'; fi
while IFS= read -r path; do [[ -z "$path" ]] && continue; is_historical "$path" && continue; fail "live session TOML remains: $path"; done <<< "$sorted"

retired='SessionToml|parse_session_toml|canonical_session_toml|canonical_toml_chunk|maximum_toml_bytes|sessionTomlBytes|sessionToml|toToml\('
if retired_output="$(rg -n "$retired" . --glob '!target/**' --glob '!.git/**' --glob '!sdk/node_modules/**' --glob '!scripts/check-session-policy.sh' --glob '!scripts/check-sdk-deletions.py' 2>&1)"; then retired_rc=0; else retired_rc=$?; fi
[[ "$retired_rc" == 0 || "$retired_rc" == 1 ]] || fail "retired session spelling search errored (rg exit $retired_rc)"
while IFS=: read -r path line rest; do [[ -z "$path" ]] && continue; path="${path#./}"; is_historical "$path" && continue; fail "retired live spelling at $path:$line"; done <<< "$retired_output"
printf 'session policy: ok\n'
