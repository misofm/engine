#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
make_base() {
  local d=$1
  mkdir -p "$d/scripts/lib" "$d/crates/engine" "$d/crates/session/src" "$d/fixtures/session" "$d/fixtures/native-pcm-runner" "$d/hosts/host-web/qualification" "$d/hosts/host-web/tests/browser-v1" "$d/sdk" "$d/fuzz" "$d/foreign"
  cp "$root/scripts/check-session-policy.sh" "$d/scripts/"; cp "$root/scripts/lib/gate.sh" "$d/scripts/lib/"
  printf '# comments only\n' >"$d/scripts/session-policy-historical-allowlist.txt"
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
d="$tmp/toml-find"; cp -a "$base" "$d"; printf x >"$d/fixtures/session/a.toml"; red find "$d"
d="$tmp/retired"; cp -a "$base" "$d"; printf 'Session''Toml\n' >"$d/retired.txt"; red retired "$d"
for root_path in fixtures/session fixtures/native-pcm-runner hosts/host-web/qualification hosts/host-web/tests/browser-v1 sdk fuzz; do d="$tmp/root-$RANDOM"; cp -a "$base" "$d"; rm -rf "$d/$root_path"; red "missing root $root_path" "$d"; done
allow="$tmp/allow"; cp -a "$base" "$allow"; printf 'fixtures/session/a.toml\n' >"$allow/scripts/session-policy-historical-allowlist.txt"; printf x >"$allow/fixtures/session/a.toml"; check "$allow"
make_find_shim() { local d=$1 token=$2; mkdir -p "$d/bin"; cat >"$d/bin/find" <<EOF
#!/usr/bin/env bash
if [[ "\$*" == *"$token"* ]]; then printf 'fixtures/session/partial.toml\n'; exit 9; fi
exec /usr/bin/find "\$@"
EOF
chmod +x "$d/bin/find"; }
for token in fixtures/session fixtures/native-pcm-runner hosts/host-web/qualification sdk; do d="$tmp/find-shim-$RANDOM"; cp -a "$base" "$d"; make_find_shim "$d" "$token"; if PATH="$d/bin:$PATH" check "$d"; then echo "find shim unexpectedly passed: $token" >&2; exit 1; fi; done
make_sed_shim() { local d=$1; mkdir -p "$d/bin"; cat >"$d/bin/sed" <<'EOF'
#!/usr/bin/env bash
printf 'fixtures/session/allowlisted.toml\n'; exit 9
EOF
chmod +x "$d/bin/sed"; }
d="$tmp/sed-shim"; cp -a "$base" "$d"; make_sed_shim "$d"; if PATH="$d/bin:$PATH" check "$d"; then echo 'sed shim unexpectedly passed' >&2; exit 1; fi
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
findshim() { local d=$1 first=$2 out=$3; mkdir -p "$d/bin"; cat >"$d/bin/find" <<EOF
#!/bin/bash
if [[ "\${1-}" == '$first' ]]; then [[ -z '$out' ]] || printf '%s\n' '$out'; exit 9; fi
exec /usr/bin/find "\$@"
EOF
chmod +x "$d/bin/find"; }
sedshim() { local d=$1 out=$2; mkdir -p "$d/bin"; cat >"$d/bin/sed" <<EOF
#!/bin/bash
[[ "\${*: -1}" != scripts/session-policy-historical-allowlist.txt ]] && exec /usr/bin/sed "\$@"
[[ -z '$out' ]] || printf '%s\n' '$out'; exit 9
EOF
chmod +x "$d/bin/sed"; }
sortshim() { mkdir -p "$1/bin"; printf '#!/bin/bash\nprintf "%%s\\n" fixtures/session/a.toml fixtures/session/b.toml; exit 9\n' >"$1/bin/sort"; chmod +x "$1/bin/sort"; }
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
# Allowlist missing, error-only, and useful partial error.
d="$tmp/no-allow"; cp -a "$base" "$d"; rm "$d/scripts/session-policy-historical-allowlist.txt"; red no-allow "$d" 'missing explicit historical allowlist'
for out in '' fixtures/session/allowed.toml; do d="$tmp/se-$RANDOM"; cp -a "$base" "$d"; sedshim "$d" "$out"; red sed "$d" 'historical allowlist read failed (sed status 9)' env PATH="$d/bin:$PATH"; done
# Four distinct find calls: error-only and an actually allowlisted partial result.
for first in fixtures/session fixtures/native-pcm-runner hosts/host-web/qualification sdk; do for out in '' fixtures/session/allowed.toml; do d="$tmp/fe-$RANDOM"; cp -a "$base" "$d"; printf 'fixtures/session/allowed.toml\n' >"$d/scripts/session-policy-historical-allowlist.txt"; findshim "$d" "$first" "$out"; red find "$d" 'traversal errored (find status 9)' env PATH="$d/bin:$PATH"; done; done
for path in sdk/a.session.toml fuzz/session_case/a.toml; do d="$tmp/shape-$RANDOM"; cp -a "$base" "$d"; mkdir -p "$d/$(dirname "$path")"; : >"$d/$path"; red shape "$d" "live session TOML remains: $path"; done
d="$tmp/sorted"; cp -a "$base" "$d"; : >"$d/fixtures/session/z.toml"; : >"$d/fixtures/session/a.toml"; red sorted "$d" 'live session TOML remains: fixtures/session/a.toml'
d="$tmp/sorterr"; cp -a "$base" "$d"; printf 'fixtures/session/a.toml\nfixtures/session/b.toml\n' >"$d/scripts/session-policy-historical-allowlist.txt"; sortshim "$d"; red sort "$d" 'session TOML discovery sort errored (sort status 9)' env PATH="$d/bin:$PATH"
# Retired exact allowlist, exclusions, error-only and useful allowlisted partial.
word='Session''Toml'; d="$tmp/ra"; cp -a "$base" "$d"; printf 'old.txt\n' >"$d/scripts/session-policy-historical-allowlist.txt"; printf '%s\n' "$word" >"$d/old.txt"; check "$d"
d="$tmp/rex"; cp -a "$base" "$d"; printf '%s\n' "$word" >"$d/scripts/check-sdk-deletions.py"; check "$d"
for out in '' "./old.txt:1:$word"; do d="$tmp/re-$RANDOM"; cp -a "$base" "$d"; printf 'old.txt\n' >"$d/scripts/session-policy-historical-allowlist.txt"; rgshim "$d" "$word|" "$out"; red retired "$d" 'retired session spelling search errored (rg exit 9)' env PATH="$d/bin:$PATH"; done
# Actual counter-mutants: same red assertion returns 97 only when faulty checker accepts.
counter() { local label=$1 d=$2 diagnostic=$3; shift 3; if red "$label" "$d" "$diagnostic" "$@" >/dev/null 2>&1; then rc=0; else rc=$?; fi; [[ $rc == 97 ]] || { printf 'counter unrelated failure %s assertion_status=%s\n' "$label" "$rc" >&2; exit 1; }; printf 'counter-mutant rejected: %s assertion_status=%s (unexpected-success assertion)\n' "$label" "$rc"; }
d="$tmp/ma"; cp -a "$base" "$d"; sed -i '/gate_scan_collect()/,/^}/s/return "\$rc"/printf '"'"'%s'"'"' "\$output"; return 0/' "$d/scripts/lib/gate.sh"; rgshim "$d" 'estimate_session' "${rows[0]}"; counter anchor "$d" 'estimate scan errored (rg exit 9)' env PATH="$d/bin:$PATH"
d="$tmp/mf"; cp -a "$base" "$d"; sed -i '/gate_find_collect()/,/^}/s/return "\$rc"/printf '"'"'%s'"'"' "\$output"; return 0/' "$d/scripts/lib/gate.sh"; printf 'fixtures/session/allowed.toml\n' >"$d/scripts/session-policy-historical-allowlist.txt"; findshim "$d" fixtures/session fixtures/session/allowed.toml; counter find "$d" 'traversal errored (find status 9)' env PATH="$d/bin:$PATH"
d="$tmp/ms"; cp -a "$base" "$d"; sed -i '/gate_sort_lines()/,/^}/s/return "\$rc"/printf '"'"'%s'"'"' "\$output"; return 0/' "$d/scripts/lib/gate.sh"; printf 'fixtures/session/a.toml\nfixtures/session/b.toml\n' >"$d/scripts/session-policy-historical-allowlist.txt"; sortshim "$d"; counter sort "$d" 'session TOML discovery sort errored (sort status 9)' env PATH="$d/bin:$PATH"
d="$tmp/md"; cp -a "$base" "$d"; sed -i 's/^\[\[ "\$allowlist_rc" == 0.*/: # ignore/' "$d/scripts/check-session-policy.sh"; sedshim "$d" ''; counter allowlist "$d" 'historical allowlist read failed (sed status 9)' env PATH="$d/bin:$PATH"
d="$tmp/mr"; cp -a "$base" "$d"; sed -i 's/^\[\[ "\$retired_rc" == 0.*/: # ignore/' "$d/scripts/check-session-policy.sh"; rgshim "$d" "$word|" ''; counter retired "$d" 'retired session spelling search errored (rg exit 9)' env PATH="$d/bin:$PATH"
printf 'session policy fixture, diagnostic, and counter-mutant checks: PASS\n'
