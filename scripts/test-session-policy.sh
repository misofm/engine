#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
make_base() {
  local d=$1
  mkdir -p "$d/scripts/lib" "$d/crates/engine" "$d/crates/session/src" "$d/foreign"
  cp "$root/scripts/check-session-policy.sh" "$d/scripts/"; cp "$root/scripts/lib/gate.sh" "$d/scripts/lib/"
  printf '[package]\nname = "engine"\n' >"$d/crates/engine/Cargo.toml"
  printf '[package]\nname = "session"\n[dependencies]\nengine.workspace = true\njson-syntax = { version = "=0.12.5", default-features = false }\n' >"$d/crates/session/Cargo.toml"
  printf 'fn clean() {}\n' >"$d/crates/session/src/lib.rs"; printf 'fn clean() {}\n' >"$d/crates/session/src/estimate.rs"
  cat >"$d/crates/session/src/compile.rs" <<'EOF'
fn compile(session: (), caps: ()) {
 let estimate = estimate_session(session);
 check_caps(session, estimate, caps);
 validate_session(session);
 let canonical_json = write_canonical(session);
 let mut normalized = session.clone();
 let _ = (canonical_json, normalized);
}
EOF
}
check() { local d=$1; shift || :; set +e; CHECK_OUTPUT=$(cd "$d/foreign" && "$@" bash "$d/scripts/check-session-policy.sh" 2>&1); CHECK_STATUS=$?; set -e; return "$CHECK_STATUS"; }
red() { local label=$1 d=$2 want=${3:-session\ policy:}; if (($# >= 3)); then shift 3; else shift 2; fi; if check "$d" "$@"; then echo "unexpected pass: $label" >&2; return 97; fi; [[ "$CHECK_OUTPUT" == *"$want"* ]] || { printf 'wrong failure %s expected %s\n%s\n' "$label" "$want" "$CHECK_OUTPUT" >&2; return 98; }; }
base="$tmp/base"; make_base "$base"; check "$base"
for kind in engine session toml serde publication estimate; do d="$tmp/$kind"; cp -a "$base" "$d"; case "$kind" in
  engine) printf 'session.workspace = true\n' >>"$d/crates/engine/Cargo.toml";;
  session) printf 'engine.workspace = true EXTRA\n' >"$d/crates/session/Cargo.toml";;
  toml) printf 'toml = "1"\n' >>"$d/crates/session/Cargo.toml";;
  serde) printf 'serde = "1"\n' >>"$d/crates/session/Cargo.toml";;
  publication) printf 'use engine::PlanPublisher;\n' >>"$d/crates/session/src/lib.rs";;
  estimate) printf 'String::with_capacity(1);\n' >>"$d/crates/session/src/estimate.rs";;
 esac; red "$kind" "$d"; done
for f in crates/engine/Cargo.toml crates/session/Cargo.toml crates/session/src/estimate.rs crates/session/src/compile.rs; do d="$tmp/missing-${f//\//-}"; cp -a "$base" "$d"; rm "$d/$f"; red "missing $f" "$d"; done
for f in 'let estimate = estimate_session(session);' 'check_caps(session, estimate, caps);' 'validate_session(session);' 'let canonical_json = write_canonical(session);' 'let mut normalized = session.clone();'; do d="$tmp/anchor-$RANDOM"; cp -a "$base" "$d"; sed -i "s|$f||" "$d/crates/session/src/compile.rs"; red anchor "$d"; done
d="$tmp/order"; cp -a "$base" "$d"; sed -i '0,/check_caps(session, estimate, caps);/{s//let canonical_json = write_canonical(session);\n check_caps(session, estimate, caps);/}' "$d/crates/session/src/compile.rs"; red order "$d"
d="$tmp/duplicate"; cp -a "$base" "$d"; cat >>"$d/crates/session/src/compile.rs" <<'EOF'
 let estimate = estimate_session(session);
 check_caps(session, estimate, caps);
 validate_session(session);
 let canonical_json = write_canonical(session);
 let mut normalized = session.clone();
EOF
check "$d"
make_rg_shim() { local d=$1 token=$2; mkdir -p "$d/bin"; cat >"$d/bin/rg" <<EOF
#!/usr/bin/env bash
if [[ "\$*" == *"$token"* ]]; then printf '1:partial\n'; exit 9; fi
exec /usr/bin/rg "\$@"
EOF
chmod +x "$d/bin/rg"; }
for token in estimate_session check_caps validate_session canonical_json 'session\.clone'; do d="$tmp/shim-$RANDOM"; cp -a "$base" "$d"; make_rg_shim "$d" "$token"; if PATH="$d/bin:$PATH" check "$d"; then echo "shim unexpectedly passed: $token" >&2; exit 1; fi; done

# Diagnostic-complete attempt-2 evidence.
rgshim() { local d=$1 token=$2 out=$3 rc=${4:-9}; mkdir -p "$d/bin"; cat >"$d/bin/rg" <<EOF
#!/bin/bash
if [[ "\$*" == *'$token'* ]]; then printf '%s\n' '$out'; exit $rc; fi
exec /usr/bin/rg "\$@"
EOF
chmod +x "$d/bin/rg"; }
# Distinct JSON pin and exact-line negatives.
for x in 'json-pin|json-syntax = { version = "=0.12.4", default-features = false }|session must exact-pin json-syntax' 'engine-extra|engine.workspace = true EXTRA|session must depend on engine' 'json-extra|json-syntax = { version = "=0.12.5", default-features = false } EXTRA|session must exact-pin json-syntax'; do IFS='|' read -r n row msg <<<"$x"; d="$tmp/$n"; cp -a "$base" "$d"; case $n in engine-extra) sed -i 's/^engine.workspace.*/'"$row"'/' "$d/crates/session/Cargo.toml";; *) sed -i 's/^json-syntax.*/'"$row"'/' "$d/crates/session/Cargo.toml";; esac; red "$n" "$d" "$msg"; done
# All six direct scans fail at the selected producer and expose its status.
tokens=('session\.workspace' 'engine\.workspace' 'json-syntax = ' 'toml|serde' 'PreparedRenderPlan|PlanPublisher' 'String::with_capacity')
descs=('engine reverse dependency' 'session engine workspace dependency' 'session json-syntax pin' 'session TOML/serde dependency' 'session publication API' 'session estimate allocation vocabulary')
outs=('1:session.workspace = true' '4:engine.workspace = true' '5:json-syntax = { version = "=0.12.5", default-features = false }' '4:toml = "1"' '1:use engine::PlanPublisher;' '1:String::with_capacity(1);')
for i in "${!tokens[@]}"; do d="$tmp/direct-$i"; cp -a "$base" "$d"; rgshim "$d" "${tokens[i]}" "${outs[i]}"; red direct "$d" "${descs[i]} scan errored (rg exit 9)" env PATH="$d/bin:$PATH"; done
# Plausible correct first rows, malformed/zero rows, and early clone distinguish first-match order.
an=(estimate caps validate canonical clone); pats=('estimate_session' 'check_caps' 'validate_session' 'write_canonical' 'session\.clone'); rows=('2: let estimate = estimate_session(session);' '3: check_caps(session, estimate, caps);' '4: validate_session(session);' '5: let canonical_json = write_canonical(session);' '6: let mut normalized = session.clone();')
for i in "${!an[@]}"; do d="$tmp/ae-$i"; cp -a "$base" "$d"; rgshim "$d" "${pats[i]}" "${rows[i]}"; red anchor "$d" "${an[i]} scan errored (rg exit 9)" env PATH="$d/bin:$PATH"; done
for row in 'x: let estimate = estimate_session(session);' '0: let estimate = estimate_session(session);'; do d="$tmp/al-$RANDOM"; cp -a "$base" "$d"; rgshim "$d" 'estimate_session' "$row" 0; red line "$d" 'estimate anchor line is not a positive decimal' env PATH="$d/bin:$PATH"; done
d="$tmp/early-clone"; cp -a "$base" "$d"; sed -i '2i let mut normalized = session.clone();' "$d/crates/session/src/compile.rs"; red early-clone "$d" 'resource preflight/cap ordering changed'
# Actual counter-mutants: same red assertion returns 97 only when faulty checker accepts.
counter() { local label=$1 d=$2 diagnostic=$3; shift 3; if red "$label" "$d" "$diagnostic" "$@" >/dev/null 2>&1; then rc=0; else rc=$?; fi; [[ $rc == 97 ]] || { printf 'counter unrelated failure %s assertion_status=%s\n' "$label" "$rc" >&2; exit 1; }; printf 'counter-mutant rejected: %s assertion_status=%s (unexpected-success assertion)\n' "$label" "$rc"; }
d="$tmp/ma"; cp -a "$base" "$d"; sed -i '/gate_scan_collect()/,/^}/s/return "\$rc"/printf '"'"'%s'"'"' "\$output"; return 0/' "$d/scripts/lib/gate.sh"; rgshim "$d" 'estimate_session' "${rows[0]}"; counter anchor "$d" 'estimate scan errored (rg exit 9)' env PATH="$d/bin:$PATH"
printf 'session policy fixture, diagnostic, and counter-mutant checks: PASS\n'
