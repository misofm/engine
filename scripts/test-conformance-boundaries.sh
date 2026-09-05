#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
source "$root/scripts/lib/gate.sh"
tmp=$(mktemp -d); trap 'rm -rf -- "$tmp"' EXIT
bash "$root/scripts/check-conformance-boundaries.sh" >/dev/null
manifest="$tmp/deps.toml"
cat >"$manifest" <<'EOF'
[dependencies]
zeta.workspace=true
alpha = "1"
[dev-dependencies]
ignored = "1"
[target.'cfg(unix)'.dependencies]
target_only.workspace = true
[features]
feature_only = []
EOF
[[ "$(gate_toml_dependencies "$manifest" plain-target)" == $'alpha\ntarget_only\nzeta' ]] || { echo 'plain-target dependency mode wrong' >&2; exit 1; }
cat >"$tmp/find" <<'EOF'
#!/usr/bin/env bash
printf '%s\n' partial/Cargo.toml
exit 9
EOF
chmod +x "$tmp/find"
if out=$(PATH="$tmp:$PATH" bash -c 'source "$1"; GATE_FAILURE_PREFIX="conformance boundary failure"; gate_find_collect manifests "$2"' _ "$root/scripts/lib/gate.sh" "$tmp" 2>&1); then echo 'conformance find unexpectedly passed' >&2; exit 1; fi
[[ $out == *'partial/Cargo.toml'* && $out == *'manifests traversal errored'* ]] || { echo "conformance find evidence missing: $out" >&2; exit 1; }
echo 'conformance boundary fixtures: ok'
