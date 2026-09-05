#!/usr/bin/env bash
# Hermetic contract tests for check-wasm-realtime-atomics.sh.  No Cargo or timing is used here.
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
checker="$repo_root/scripts/check-wasm-realtime-atomics.sh"
test_root="$(mktemp -d)"
trap 'rm -rf -- "$test_root"' EXIT
tool_bin="$test_root/bin"
mkdir -p "$tool_bin"
real_ar="$(command -v ar)"; real_find="$(command -v find)"; real_rg="$(command -v rg)"
export REAL_AR="$real_ar" REAL_FIND="$real_find" REAL_RG="$real_rg"

cat >"$tool_bin/rustc" <<'EOF'
#!/usr/bin/env bash
if [[ "${FAKE_CASE:-}" == cfg-error ]]; then printf 'target_has_atomic="ptr"\n'; exit 13; fi
if [[ "${FAKE_CASE:-}" == cfg-missing ]]; then exit 0; fi
if [[ "${FAKE_CASE:-}" == cfg-feature ]]; then printf 'target_has_atomic="ptr"\ntarget_feature="atomics"\n'; exit 0; fi
printf '%s\n' 'target_has_atomic="ptr"'
EOF
cat >"$tool_bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${FAKE_CASE:-}" == cargo-fail ]]; then
  mkdir -p "${CARGO_TARGET_DIR}/wasm32-unknown-unknown/release/deps"
  exit 42
fi
deps="${CARGO_TARGET_DIR}/wasm32-unknown-unknown/release/deps"
mkdir -p "$deps"
case "${FAKE_CASE:-}" in
  missing-source) families=(engine target_smoke) ;;
  missing-engine) families=(source target_smoke) ;;
  missing-target) families=(engine source) ;;
  empty-objects) families=(engine source target_smoke) ;;
  *) families=(engine source target_smoke) ;;
esac
for family in "${families[@]}"; do
  object="$family-main.o"
  archive_family="${family//_/-}"
  if [[ "${FAKE_CASE:-}" == duplicate-members && "$family" == engine ]]; then
    printf 'duplicate\n' >"$deps/lib$family-hash.rlib.members"
    "$REAL_AR" rcs "$deps/lib$archive_family-hash.rlib" "$deps/lib$family-hash.rlib.members"
    continue
  fi
  if [[ "${FAKE_CASE:-}" != empty-objects ]]; then
    object_text="${FAKE_OBJECT_TEXT:-observe clean}"
    [[ "${FAKE_CASE:-}" == no-observation-fail || "${FAKE_CASE:-}" == source-search-error || "${FAKE_CASE:-}" == obs-search-error ]] && object_text=clean
    [[ "${FAKE_CASE:-}" == atomic-opcode ]] && object_text='atomic.get'
    printf '%s\n' "$object_text" >"$deps/$object"
    "$REAL_AR" rcs "$deps/lib$archive_family-hash.rlib" "$deps/$object"
  else
    printf 'metadata\n' >"$deps/$family.txt"
    "$REAL_AR" rcs "$deps/lib$archive_family-hash.rlib" "$deps/$family.txt"
  fi
done
# This archive is deliberately outside the owned child and must never be inspected.
mkdir -p "${CARGO_TARGET_DIR}/stale/wasm32-unknown-unknown/release/deps"
printf 'atomic.stale\n' >"${CARGO_TARGET_DIR}/stale/stale.o"
"$REAL_AR" rcs "${CARGO_TARGET_DIR}/stale/wasm32-unknown-unknown/release/deps/libengine-stale.rlib" "${CARGO_TARGET_DIR}/stale/stale.o"
EOF
cat >"$tool_bin/ar" <<'EOF'
#!/usr/bin/env bash
if [[ "${FAKE_CASE:-}" == ar-list-fail && "$1" == t* ]]; then exit 17; fi
if [[ "${FAKE_CASE:-}" == ar-extract-fail && "$1" == x* ]]; then exit 18; fi
exec "$REAL_AR" "$@"
EOF
cat >"$tool_bin/wasm-objdump" <<'EOF'
#!/usr/bin/env bash
if [[ "${FAKE_CASE:-}" == decoder-fail ]]; then printf 'clean partial\n'; exit 19; fi
cat "${@: -1}"
EOF
cat >"$tool_bin/find" <<'EOF'
#!/usr/bin/env bash
"$REAL_FIND" "$@"
status=0
if [[ "${FAKE_CASE:-}" == archive-discovery-fail && "$*" == *-name\ libengine-* ]]; then status=23; fi
exit "$status"
EOF
cat >"$tool_bin/rg" <<'EOF'
#!/usr/bin/env bash
if [[ "${FAKE_CASE:-}" == source-search-error && "$*" == *ObservationSlot* ]]; then exit 24; fi
if [[ "${FAKE_CASE:-}" == no-observation-fail && "$*" == *ObservationSlot* ]]; then exit 1; fi
if [[ "${FAKE_CASE:-}" == obs-search-error && "$*" == *observe* ]]; then exit 25; fi
if [[ "${FAKE_CASE:-}" == opcode-scan-error && "$*" == *atomic* ]]; then exit 26; fi
exec "$REAL_RG" "$@"
EOF
chmod +x "$tool_bin"/*
export PATH="$tool_bin:$PATH"

run_case() {
  local name="$1" expected="$2"; shift 2
  local target="$test_root/target-$name" log="$test_root/$name.log"
  set +e
  FAKE_CASE="$name" "$checker" "$target" >"$log" 2>&1
  local status=$?
  set -e
  if [[ "$expected" == pass && "$status" != 0 ]]; then cat "$log" >&2; printf 'case %s unexpectedly failed\n' "$name" >&2; return 1; fi
  if [[ "$expected" == fail && "$status" == 0 ]]; then cat "$log" >&2; printf 'case %s unexpectedly passed\n' "$name" >&2; return 1; fi
}
run_case base pass
run_case no-observation pass
run_case cargo-fail fail
run_case missing-source fail
run_case missing-engine fail
run_case missing-target fail
run_case empty-objects fail
run_case decoder-fail fail
run_case ar-list-fail fail
run_case ar-extract-fail fail
run_case archive-discovery-fail fail
run_case cfg-error fail
run_case cfg-missing fail
run_case cfg-feature fail
run_case atomic-opcode fail
run_case opcode-scan-error fail
run_case obs-search-error fail
run_case source-search-error fail
run_case no-observation-fail fail

# Counter-mutant 1: restore the old conditional decoder/scan false-pass. The named decoder
# assertion must reject the mutant's unexpected success on a clean-looking partial decode.
mutant_decoder="$test_root/mutant-decoder.sh"
sed 's/if ! wasm-objdump -d "\$object" >"\$decoded" 2>"\$decoded_stderr"; then printf '\''wasm-objdump failed for object: %s\\n'\'' "\$object" >&2; exit 1; fi/wasm-objdump -d "\$object" >"\$decoded" 2>"\$decoded_stderr" || true/' "$checker" >"$mutant_decoder"
chmod +x "$mutant_decoder"
if FAKE_CASE=decoder-fail "$mutant_decoder" "$test_root/mutant-decoder-target" >/dev/null 2>&1; then
  printf 'decoder counter-mutant: focused assertion rejected unexpected success\n'
else
  printf 'decoder counter-mutant did not reproduce the historical false-pass\n' >&2; exit 1
fi

# Counter-mutant 2: swallow a partial complete-looking archive-discovery producer error.
mutant_find="$test_root/mutant-find.sh"
sed '/if ! find .*archive_list/c\    find "$deps" -maxdepth 1 -type f -name "lib${family//_/-}-*.rlib" -print0 >"$archive_list" || true' "$checker" >"$mutant_find"
chmod +x "$mutant_find"
if FAKE_CASE=archive-discovery-fail "$mutant_find" "$test_root/mutant-find-target" >/dev/null 2>&1; then
  printf 'producer counter-mutant: focused assertion rejected unexpected success\n'
else
  printf 'producer counter-mutant did not reproduce the historical false-pass\n' >&2; exit 1
fi

printf 'wasm realtime atomics hermetic suite: PASS (directed cases and 2 causal mutants)\n'
