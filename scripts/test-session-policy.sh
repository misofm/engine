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
check() { (cd "$1/foreign" && bash "$1/scripts/check-session-policy.sh") >/dev/null 2>&1; }
red() { local label=$1 d=$2; if check "$d"; then echo "unexpected pass: $label" >&2; exit 1; fi; }
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
printf 'session policy fixture and selective producer checks: PASS\n'
