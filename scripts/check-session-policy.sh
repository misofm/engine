#!/usr/bin/env bash
# Sole-format and dependency-direction gate for canonical Session V1 JSON (issue #338).
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
fail() { printf 'session policy: %s\n' "$1" >&2; exit 1; }

session_manifest=crates/session/Cargo.toml
session_source=crates/session/src
allowlist=scripts/session-policy-historical-allowlist.txt
[[ -f "$allowlist" ]] || fail "missing explicit historical allowlist"
! rg -q '^session\.workspace = true$' crates/engine/Cargo.toml || fail "engine must not depend on session"
rg -qx 'engine\.workspace = true' "$session_manifest" || fail "session must depend on engine"
rg -qx 'json-syntax = \{ version = "=0\.12\.5", default-features = false \}' "$session_manifest" || fail "session must exact-pin json-syntax 0.12.5 without default features"
! rg -n '^[[:space:]]*(toml|serde)[[:space:]]*=' "$session_manifest" || fail "session runtime parser baggage returned"
! rg -n 'use engine::.*(PreparedRenderPlan|PlanPublisher)|PlanPublisher<' "$session_source" || fail "session may not import plan publication APIs"
! rg -n 'format!|\.to_owned\(|\.to_string\(|String::with_capacity|Vec::with_capacity|\.collect::' "$session_source/estimate.rs" || fail "successful resource preflight may not allocate temporary diagnostics or collections"

compile_source="$session_source/compile.rs"
estimate_line="$(rg -n 'let estimate = estimate_session\(session\)' "$compile_source" | head -1 | cut -d: -f1)"
caps_line="$(rg -n 'check_caps\(session, estimate, caps\)' "$compile_source" | head -1 | cut -d: -f1)"
validate_line="$(rg -n 'validate_session\(session\)' "$compile_source" | head -1 | cut -d: -f1)"
canonical_line="$(rg -n 'let canonical_json = write_canonical\(session\)' "$compile_source" | head -1 | cut -d: -f1)"
clone_line="$(rg -n 'let mut normalized = session\.clone\(\)' "$compile_source" | head -1 | cut -d: -f1)"
(( estimate_line < caps_line && caps_line < validate_line && validate_line < canonical_line && canonical_line < clone_line )) || fail "resource preflight/cap ordering changed"

mapfile -t historical < <(sed -e '/^[[:space:]]*#/d' -e '/^[[:space:]]*$/d' "$allowlist")
is_historical() {
  local path=$1 entry
  for entry in "${historical[@]}"; do [[ "$path" == "$entry" ]] && return 0; done
  return 1
}

while IFS= read -r path; do
  is_historical "$path" && continue
  fail "live session TOML remains: $path"
done < <({
  find fixtures/session -type f -name '*.toml' 2>/dev/null
  find fixtures/native-pcm-runner -type f -name '*.toml' 2>/dev/null
  find hosts/host-web/qualification hosts/host-web/tests/browser-v1 -type f -name '*.toml' 2>/dev/null
  find sdk fuzz -type f \( -name '*.session.toml' -o -path '*/session_*/*.toml' \) 2>/dev/null
} | sort)

retired='SessionToml|parse_session_toml|canonical_session_toml|canonical_toml_chunk|maximum_toml_bytes|sessionTomlBytes|sessionToml|toToml\('
while IFS=: read -r path line rest; do
  [[ -z "$path" ]] && continue
  path="${path#./}"
  is_historical "$path" && continue
  fail "retired live spelling at $path:$line"
done < <(rg -n "$retired" . --glob '!target/**' --glob '!.git/**' --glob '!sdk/node_modules/**' --glob '!scripts/check-session-policy.sh' --glob '!scripts/check-sdk-deletions.py' || true)

printf 'session policy: ok\n'
