#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
checker="$repo_root/scripts/check-wasm-realtime-atomics.sh"
test_root="$(mktemp -d)"; trap 'rm -rf -- "$test_root"' EXIT
bin="$test_root/bin"; mkdir -p "$bin"
export REAL_AR="$(command -v ar)" REAL_FIND="$(command -v find)" REAL_RG="$(command -v rg)" REAL_SORT="$(command -v sort)"

cat >"$bin/rustc" <<'EOF'
#!/usr/bin/env bash
[[ "$*" == '--print cfg --target wasm32-unknown-unknown -C target-feature=-simd128' ]] || { printf 'RUSTC_ARGUMENT_SENTINEL\n' >&2; exit 90; }
case "${FAKE_CASE:-}" in
  cfg-empty-error) printf 'CFG_EMPTY_SENTINEL\n' >&2; exit 12 ;;
  cfg-error) printf 'target_has_atomic="ptr"\n'; printf 'CFG_SENTINEL\n' >&2; exit 13 ;;
  cfg-missing) exit 0 ;;
  cfg-feature) printf 'target_has_atomic="ptr"\ntarget_feature="atomics"\n'; exit 0 ;;
esac
printf 'target_has_atomic="ptr"\n'
EOF
cat >"$bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
[[ "$CARGO_PROFILE_RELEASE_LTO" == false && "$RUSTFLAGS" == '-C target-feature=-simd128' ]] || { printf 'CARGO_ENV_SENTINEL\n' >&2; exit 91; }
[[ "$*" == 'build --locked --release --target wasm32-unknown-unknown -p engine -p source -p target-smoke' ]] || { printf 'CARGO_ARGUMENT_SENTINEL\n' >&2; exit 92; }
deps="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/deps"; mkdir -p "$deps"
if [[ "${FAKE_CASE:-}" == cargo-fail ]]; then printf 'CARGO_SENTINEL\n' >&2; touch "$deps/libengine-partial.rlib"; exit 42; fi
case "${FAKE_CASE:-}" in missing-engine) families=(source target_smoke);; missing-source) families=(engine target_smoke);; missing-target) families=(engine source);; *) families=(engine source target_smoke);; esac
for family in "${families[@]}"; do
  object="$deps/$family-main.o"; text='observe clean'
  case "${FAKE_CASE:-}" in source-fallback|source-error|no-observation) text=clean;; late-observation-error) [[ "$family" != engine ]] || text='observe clean';; atomic-opcode) [[ "$family" != engine ]] || text='atomic.get observe';; empty-decode) [[ "$family" != engine ]] || text='';; esac
  printf '%s' "$text" >"$object"
  "$REAL_AR" rcs "$deps/lib$family-hash.rlib" "$object"
done
EOF
cat >"$bin/ar" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${FAKE_CASE:-}:$1:$2" in
  ar-list-empty:t*:*libengine-*) printf 'LIST_EMPTY_SENTINEL\n' >&2; exit 17 ;;
  ar-list-partial:t*:*libengine-*) "$REAL_AR" t "$2"; printf 'LIST_PARTIAL_SENTINEL\n' >&2; exit 17 ;;
  duplicate-object:t*:*libengine-*) "$REAL_AR" t "$2"; "$REAL_AR" t "$2" ;;
  list-omits-member:t*:*libengine-*) : ;;
  ar-extract-empty:x*:*libengine-*) printf 'EXTRACT_EMPTY_SENTINEL\n' >&2; exit 18 ;;
  ar-extract-partial:x*:*libengine-*) "$REAL_AR" x "$2"; printf 'EXTRACT_PARTIAL_SENTINEL\n' >&2; exit 18 ;;
  missing-extracted:x*:*libengine-*) exit 0 ;;
  extracted-extra:x*:*libengine-*) "$REAL_AR" x "$2"; printf extra >extra.o ;;
  *) exec "$REAL_AR" "$@" ;;
esac
EOF
cat >"$bin/wasm-objdump" <<'EOF'
#!/usr/bin/env bash
case "${FAKE_CASE:-}" in
  decoder-empty-error) printf 'DECODE_EMPTY_SENTINEL\n' >&2; exit 19 ;;
  decoder-partial-error) printf 'clean partial\n'; printf 'DECODE_PARTIAL_SENTINEL\n' >&2; exit 19 ;;
esac
cat "${@: -1}"
EOF
cat >"$bin/find" <<'EOF'
#!/usr/bin/env bash
args=" $* "
case "${FAKE_CASE:-}" in
  archive-find-empty-error) [[ "$args" == *'libengine-*.rlib'* ]] && { printf 'ARCHIVE_FIND_EMPTY_SENTINEL\n' >&2; exit 23; } ;;
  object-find-empty-error) [[ "$args" == *" -name *.o "* && "$args" == *'/engine '* ]] && { printf 'OBJECT_FIND_EMPTY_SENTINEL\n' >&2; exit 27; } ;;
esac
set +e
"$REAL_FIND" "$@"; status=$?
set -e
[[ "$status" == 0 ]] || exit "$status"
case "${FAKE_CASE:-}" in
  archive-find-partial-error) [[ "$args" == *'libengine-*.rlib'* ]] && { printf 'ARCHIVE_FIND_PARTIAL_SENTINEL\n' >&2; exit 23; } ;;
  object-find-partial-error) [[ "$args" == *" -name *.o "* && "$args" == *'/engine '* ]] && { printf 'OBJECT_FIND_PARTIAL_SENTINEL\n' >&2; exit 27; } ;;
esac
exit 0
EOF
cat >"$bin/sort" <<'EOF'
#!/usr/bin/env bash
case "${FAKE_CASE:-}" in
 archive-sort-empty-error) [[ "$*" == *engine.archives* ]] && { : >"$3"; printf 'ARCHIVE_SORT_EMPTY_SENTINEL\n' >&2; exit 26; };;
 archive-sort-error) [[ "$*" == *engine.archives* ]] && { printf 'ARCHIVE_SORT_SENTINEL\n' >&2; exit 28; };;
 member-sort-empty-error) [[ "$*" == *engine/members ]] && { printf 'MEMBER_SORT_EMPTY_SENTINEL\n' >&2; exit 36; };;
 member-sort-partial-error) [[ "$*" == *engine/members ]] && { "$REAL_SORT" "$@"; printf 'MEMBER_SORT_PARTIAL_SENTINEL\n' >&2; exit 37; };;
 object-sort-empty-error) [[ "$*" == *engine/objects* ]] && { : >"$3"; printf 'OBJECT_SORT_EMPTY_SENTINEL\n' >&2; exit 25; };;
 object-sort-error) [[ "$*" == *engine/objects* ]] && { printf 'OBJECT_SORT_SENTINEL\n' >&2; exit 29; };;
esac
exec "$REAL_SORT" "$@"
EOF
cat >"$bin/rg" <<'EOF'
#!/usr/bin/env bash
args=" $* "
case "${FAKE_CASE:-}" in
 pointer-search-error) [[ "$args" == *target_has_atomic* ]] && { printf 'POINTER_SEARCH_SENTINEL\n' >&2; exit 30; };;
 feature-search-error) [[ "$args" == *target_feature* ]] && { printf 'FEATURE_SEARCH_SENTINEL\n' >&2; exit 31; };;
 opcode-search-error) [[ "$args" == *" atomic\\. "* ]] && { printf 'OPCODE_SEARCH_SENTINEL\n' >&2; exit 32; };;
 observation-error) [[ "$args" == *" observe "* ]] && { printf 'OBS_SEARCH_SENTINEL\n' >&2; exit 33; };;
 late-observation-error) [[ "$args" == *" observe "* && "$args" == *source-main.o* ]] && { printf 'LATE_OBS_SENTINEL\n' >&2; exit 34; };;
 source-error) [[ "$args" == *ObservationSlot* ]] && { printf 'SOURCE_SEARCH_SENTINEL\n' >&2; exit 35; };;
 no-source-needed) [[ "$args" == *ObservationSlot* ]] && { printf 'SOURCE_WAS_CONSULTED\n' >&2; exit 36; };;
 no-observation) [[ "$args" == *ObservationSlot* ]] && exit 1;;
esac
exec "$REAL_RG" "$@"
EOF
chmod +x "$bin"/*
export PATH="$bin:$PATH"

assert_case() {
  local checker_path="$1" name="$2" expected="$3" diagnostic="${4:-}"
  local target="$test_root/target-$name" log="$test_root/$name.log"
  mkdir -p "$target/parent-cache"
  printf keep >"$target/parent-cache/stale-sentinel"
  printf 'not an archive: atomic.stale' >"$target/parent-cache/libengine-stale.rlib"
  cp "$target/parent-cache/libengine-stale.rlib" "$target/stale.before"
  set +e; FAKE_CASE="$name" "$checker_path" "$target" >"$log" 2>&1; local status=$?; set -e
  [[ -f "$target/parent-cache/stale-sentinel" ]] || { printf 'case %s deleted caller parent cache\n' "$name" >&2; return 96; }
  cmp -s "$target/stale.before" "$target/parent-cache/libengine-stale.rlib" || { printf 'case %s changed caller stale archive\n' "$name" >&2; return 96; }
  if [[ "$expected" == pass ]]; then
    [[ "$status" == 0 ]] || { cat "$log" >&2; printf 'case %s unexpectedly failed (%s)\n' "$name" "$status" >&2; return 96; }
    printf 'case %s: PASS (status 0)\n' "$name"; return 0
  fi
  if [[ "$status" == 0 ]]; then printf 'case %s unexpected-success\n' "$name" >&2; return 97; fi
  local required
  IFS=';' read -r -a required <<<"$diagnostic"
  for required in "${required[@]}"; do
    "$REAL_RG" -F "$required" "$log" >/dev/null || { cat "$log" >&2; printf 'case %s failed at wrong site; missing %s\n' "$name" "$required" >&2; return 96; }
  done
  printf 'case %s: PASS (checker status %s, targeted diagnostic)\n' "$name" "$status"
}

assert_case "$checker" base pass
assert_case "$checker" source-fallback pass
assert_case "$checker" no-source-needed pass
assert_path_case() {
  local name="$1" mode="$2" target
  local cwd="$test_root/path-$name" log="$test_root/$name.log"
  mkdir -p "$cwd"
  if [[ "$mode" == relative ]]; then target='target/ci/wasm-scalar'; else target='target/ci/wasm-realtime-local'; fi
  mkdir -p "$cwd/$target/parent-cache"
  printf 'not an archive: atomic.stale' >"$cwd/$target/parent-cache/libengine-stale.rlib"
  cp "$cwd/$target/parent-cache/libengine-stale.rlib" "$cwd/stale.before"
  set +e
  if [[ "$mode" == relative ]]; then (cd "$cwd" && FAKE_CASE=base "$checker" "$target") >"$log" 2>&1
  else (cd "$cwd" && FAKE_CASE=base "$checker") >"$log" 2>&1
  fi
  local status=$?; set -e
  [[ "$status" == 0 ]] || { cat "$log" >&2; printf 'case %s unexpectedly failed (%s)\n' "$name" "$status" >&2; return 96; }
  cmp -s "$cwd/stale.before" "$cwd/$target/parent-cache/libengine-stale.rlib" || { printf 'case %s changed caller stale archive\n' "$name" >&2; return 96; }
  printf 'case %s: PASS (status 0, real ar extraction)\n' "$name"
}
assert_path_case relative-ci-target relative
assert_path_case omitted-default default
cases=(
 'cfg-empty-error|rustc cfg production failed (status 12);CFG_EMPTY_SENTINEL'
 'cfg-error|rustc cfg production failed (status 13);CFG_SENTINEL' 'cfg-missing|does not advertise pointer-width atomic' 'cfg-feature|unexpectedly enables Wasm atomics'
 'pointer-search-error|cfg atomic-support search failed (status 30);POINTER_SEARCH_SENTINEL' 'feature-search-error|cfg atomics-feature search failed (status 31);FEATURE_SEARCH_SENTINEL'
 'cargo-fail|cargo build failed (status 42);CARGO_SENTINEL' 'missing-engine|engine archive population is incomplete' 'missing-source|source archive population is incomplete' 'missing-target|target_smoke archive population is incomplete'
 'archive-find-empty-error|engine archive discovery failed (status 23);ARCHIVE_FIND_EMPTY_SENTINEL' 'archive-find-partial-error|engine archive discovery failed (status 23);ARCHIVE_FIND_PARTIAL_SENTINEL'
 'archive-sort-empty-error|engine archive sort failed (status 26);ARCHIVE_SORT_EMPTY_SENTINEL' 'archive-sort-error|engine archive sort failed (status 28);ARCHIVE_SORT_SENTINEL'
 'member-sort-empty-error|engine archive member sort failed (status 36);MEMBER_SORT_EMPTY_SENTINEL' 'member-sort-partial-error|engine archive member sort failed (status 37);MEMBER_SORT_PARTIAL_SENTINEL'
 'ar-list-empty|engine archive member listing failed (status 17);LIST_EMPTY_SENTINEL' 'ar-list-partial|engine archive member listing failed (status 17);LIST_PARTIAL_SENTINEL' 'duplicate-object|duplicate object member'
 'list-omits-member|archive has no object members' 'ar-extract-empty|engine archive extraction failed (status 18);EXTRACT_EMPTY_SENTINEL' 'ar-extract-partial|engine archive extraction failed (status 18);EXTRACT_PARTIAL_SENTINEL'
 'missing-extracted|object reconciliation failed' 'extracted-extra|object reconciliation failed' 'object-find-empty-error|engine object discovery failed (status 27);OBJECT_FIND_EMPTY_SENTINEL'
 'object-find-partial-error|engine object discovery failed (status 27);OBJECT_FIND_PARTIAL_SENTINEL'
 'object-sort-empty-error|engine object sort failed (status 25);OBJECT_SORT_EMPTY_SENTINEL' 'object-sort-error|engine object sort failed (status 29);OBJECT_SORT_SENTINEL'
 'decoder-empty-error|wasm-objdump failed (status 19);DECODE_EMPTY_SENTINEL' 'decoder-partial-error|wasm-objdump failed (status 19);DECODE_PARTIAL_SENTINEL' 'atomic-opcode|contains an atomic opcode'
 'opcode-search-error|opcode scan failed (status 32);OPCODE_SEARCH_SENTINEL' 'observation-error|observation object search failed (status 33);OBS_SEARCH_SENTINEL'
 'late-observation-error|observation object search failed (status 34);LATE_OBS_SENTINEL' 'source-error|source ObservationSlot search failed (status 35);SOURCE_SEARCH_SENTINEL'
 'no-observation|absent from objects and source fallback'
)
for row in "${cases[@]}"; do
  IFS='|' read -r name diagnostic <<<"$row"; assert_case "$checker" "$name" fail "$diagnostic"
done
assert_case "$checker" empty-decode pass

make_mutant() {
  local name="$1" old="$2" new="$3" count
  local output="$test_root/$name.sh"
  count="$(awk -v text="$old" 'index($0,text){n++} END{print n+0}' "$checker")"
  [[ "$count" == 1 ]] || { printf '%s mutant edit matched %s sites, expected 1\n' "$name" "$count" >&2; exit 1; }
  awk -v old="$old" -v new="$new" '{ if (index($0, old)) print new; else print }' "$checker" >"$output"; chmod +x "$output"
  diff -u "$checker" "$output" >"$test_root/$name.diff" || [[ "$?" == 1 ]]
  printf '%s exact diff:\n' "$name"; cat "$test_root/$name.diff"
  MUTANT_PATH="$output"
}
make_mutant decoder-mutant '[[ "$status" == 0 ]] || failed wasm-objdump "$status" "$object" "$decoded" "$scratch/$n.decode.err"' '[[ "$status" == 0 ]] || true'
decoder_mutant="$MUTANT_PATH"
if assert_case "$decoder_mutant" decoder-partial-error fail 'wasm-objdump failed (status 19)'; then mutant_status=0; else mutant_status=$?; fi
[[ "$mutant_status" == 97 ]] || { printf 'decoder mutant assertion status %s, expected 97\n' "$mutant_status" >&2; exit 1; }
printf 'decoder mutant same assertion status: 97 (unexpected-success)\n'
make_mutant producer-mutant '[[ "$status" == 0 ]] || failed "$family archive discovery" "$status" "$deps" "$list" "$scratch/$family.find.err"' '[[ "$status" == 0 ]] || true'
producer_mutant="$MUTANT_PATH"
if assert_case "$producer_mutant" archive-find-partial-error fail 'engine archive discovery failed (status 23)'; then mutant_status=0; else mutant_status=$?; fi
[[ "$mutant_status" == 97 ]] || { printf 'producer mutant assertion status %s, expected 97\n' "$mutant_status" >&2; exit 1; }
printf 'producer mutant same assertion status: 97 (unexpected-success)\n'
printf 'wasm realtime atomics hermetic suite: PASS (directed cases and 2 causal mutants)\n'
