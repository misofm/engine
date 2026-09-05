#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
source "$root/scripts/lib/gate.sh"
tmp=$(mktemp -d); trap 'rm -rf -- "$tmp"' EXIT
assert_fail() { local label=$1; shift; local out; if out=$("$@" 2>&1); then echo "graph fixture unexpectedly passed: $label" >&2; exit 1; else :; fi; [[ $out == *"$label"* ]] || { echo "wrong graph diagnostic: $out" >&2; exit 1; }; }
bash "$root/scripts/check-graph-policy.sh" >/dev/null
assert_fail 'missing graph manifests' bash "$root/scripts/check-graph-policy.sh" "$tmp"
cat >"$tmp/find" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' plausible.rs
exit 7
EOF
chmod +x "$tmp/find"
if out=$(PATH="$tmp:$PATH" bash -c 'source "$1"; GATE_FAILURE_PREFIX="graph policy failure"; gate_find_collect discovery "$2"' _ "$root/scripts/lib/gate.sh" "$tmp" 2>&1); then echo 'find partial-output unexpectedly passed' >&2; exit 1; fi
[[ $out == *'plausible.rs'* && $out == *'discovery traversal errored'* ]] || { echo "find error evidence missing: $out" >&2; exit 1; }
manifest="$tmp/deps.toml"
printf '[dependencies]\nengine.workspace=true\nlane.workspace = true\n' >"$manifest"
[[ "$(gate_toml_dependencies "$manifest")" == $'engine.workspace=true\nlane' ]] || { echo 'graph default parser quirk changed' >&2; exit 1; }
echo 'graph policy fixtures: ok'
