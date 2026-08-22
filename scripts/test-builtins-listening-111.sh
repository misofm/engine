#!/usr/bin/env bash
# Hermetic Issue-111 lifecycle; no real build, renderer, source read, or playback.
set -euo pipefail
script_directory="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
root="$(cd "$script_directory/.." && pwd)"
issue111_state() {
    if [[ ! -e "$root/target/issue111" && ! -L "$root/target/issue111" ]]; then printf 'ABSENT\n'; return; fi
    find "$root/target/issue111" -mindepth 0 -printf '%P\t%y\t%s\t%n\n' | sort
    find "$root/target/issue111" -type f -print0 | sort -z | xargs -0 -r sha256sum
}
real_before="$(issue111_state)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
template="$scratch/template"
mkdir -p "$template"/{bin,scripts,target/issue110,target/issue111/inbox} \
    "$template/tools/miso-engine-builtins-fixture/src" \
    "$template/crates/miso-engine-builtins/src" \
    "$template/crates/miso-engine-builtins-compiler/src" \
    "$template/fixtures/builtins/v1" "$template/fixtures/conformance/v1" \
    "$template/dsp-research/listening/issue033"
cp "$root/Cargo.lock" "$template/"
cp "$root/crates/miso-engine-builtins/src/lib.rs" "$template/crates/miso-engine-builtins/src/"
cp "$root/crates/miso-engine-builtins-compiler/src/lib.rs" "$template/crates/miso-engine-builtins-compiler/src/"
cp "$root/tools/miso-engine-builtins-fixture/Cargo.toml" "$template/tools/miso-engine-builtins-fixture/"
cp "$root/tools/miso-engine-builtins-fixture/src/listening_main.rs" "$template/tools/miso-engine-builtins-fixture/src/"
cp "$root/fixtures/builtins/v1/MANIFEST.tsv" "$template/fixtures/builtins/v1/"
cp "$root/fixtures/conformance/v1/prng-noise-048000-dual-mono.mepcm" "$template/fixtures/conformance/v1/"
cp "$root/dsp-research/listening/issue007-filter-abx-preregistration.md" \
    "$root/dsp-research/listening/issue007-matrix-ramp-preregistration.md" \
    "$root/dsp-research/listening/TEMPLATE.md" "$template/dsp-research/listening/"
cp -a "$root/dsp-research/listening/issue033/." "$template/dsp-research/listening/issue033/"
cp "$root/scripts/check-builtins-listening.sh" "$root/scripts/"*builtins-listening-033* \
    "$root/scripts/"*builtins-listening-111* "$template/scripts/"
cp -a "$root/target/issue110/." "$template/target/issue110/"
printf fake >"$template/target/issue111/inbox/source.mepcm"
printf '{}\n' >"$template/target/issue111/inbox/provenance.json"
printf '42\n' >"$template/target/issue111/inbox/seed.txt"
printf '{}\n' >"$template/target/issue111/preparation.seal.json"
chmod 0700 "$template/target/issue111/inbox"
chmod 0400 "$template/target/issue111/inbox/source.mepcm"
chmod 0600 "$template/target/issue111/inbox/provenance.json" "$template/target/issue111/inbox/seed.txt"
chmod 0444 "$template/target/issue111/preparation.seal.json"
real_bash="$(command -v bash)"
real_python="$(command -v python3)"

cat >"$template/bin/git" <<'EOF'
#!/bin/bash
case "$*" in
  *'branch --show-current'*) printf 'codex/listening-111\n' ;;
  *'rev-parse --verify HEAD'*|*'rev-parse HEAD'*) printf '%s\n' "$MISO_TEST_CANDIDATE" ;;
  *'HEAD^{tree}'*) printf '%s\n' "$MISO_TEST_CANDIDATE" ;;
  *status*)
    if [[ "${MISO_TEST_MODE:-}" == dirty ||
          ( "${MISO_TEST_MODE:-}" == authority_drift && -e "$MISO_TEST_CASE_ROOT/status-seen" ) ||
          ( "${MISO_TEST_MODE:-}" == post_render_drift && -e "$MISO_TEST_LAUNCH_LOG" ) ]]; then
      printf ' M format-only\n'
    fi
    : >"$MISO_TEST_CASE_ROOT/status-seen" ;;
  *) exit 91 ;;
esac
EOF
cat >"$template/bin/bash" <<'EOF'
#!/bin/bash
case "${1:-}" in
  scripts/test-builtins-listening-111-policy.sh|scripts/test-builtins-listening-111.sh) exit 0 ;;
  *) exec "$MISO_TEST_REAL_BASH" "$@" ;;
esac
EOF
cat >"$template/bin/python3" <<'EOF'
#!/bin/bash
if [[ "${1:-}" == - ]]; then exec "$MISO_TEST_REAL_PYTHON" "$@"; fi
if [[ "$*" == *'--write-seal'* ]]; then
  for ((index=1; index<=$#; index++)); do
    if [[ "${!index}" == --write-seal ]]; then output_index=$((index + 2)); printf '{}\n' >"${!output_index}"; exit 0; fi
  done
fi
if [[ "$*" == *'--assemble'* ]]; then output=${!#}; printf '{}\n' >"$output"; exit 0; fi
if [[ "${MISO_TEST_MODE:-}" == seal_drift && "$*" == *'--seal preflight'* ]]; then exit 81; fi
if [[ "$*" == *check-builtins-listening-033.py* || "$*" == *check-builtins-listening-111.py* ]]; then exit 0; fi
exec "$MISO_TEST_REAL_PYTHON" "$@"
EOF
cat >"$template/bin/cargo" <<'EOF'
#!/bin/bash
printf '%s\n' "$*" >>"$MISO_TEST_CASE_ROOT/cargo.log"
if [[ " $* " == *' build '* ]]; then
  mkdir -p "$CARGO_TARGET_DIR/release"
  cat >"$CARGO_TARGET_DIR/release/miso_engine_builtins_fixture_listening" <<'BIN'
#!/bin/bash
printf 'render\n' >>"$MISO_TEST_LAUNCH_LOG"
output=${!#}
mkdir "$output/public" "$output/private"
chmod 0755 "$output/public"; chmod 0700 "$output/private"
printf '{}\n' >"$output/public/render-manifest.json"
printf '{}\n' >"$output/private/assignment-key.json"
printf '{}\n' >"$output/private/source-provenance.json"
chmod 0600 "$output/private/"*
for token in 00000000000000000000000000000001 00000000000000000000000000000002 00000000000000000000000000000003 00000000000000000000000000000004; do
  printf RIFF >"$output/public/$token.wav"
done
if [[ "${MISO_TEST_MODE:-}" == renderer_failure ]]; then printf partial >"$output/public/partial"; exit 73; fi
BIN
  chmod 0555 "$CARGO_TARGET_DIR/release/miso_engine_builtins_fixture_listening"
fi
EOF
chmod 0755 "$template/bin/"*

case_number=0
new_case() {
    case_number=$((case_number + 1))
    case_root="$scratch/case-$case_number-$1"
    cp -a "$template" "$case_root"
    candidate=0123456789abcdef0123456789abcdef01234567
    launch_log="$case_root/launch.log"
    artifact="$case_root/target/issue111"
}
run_preflight() {
    MISO_TEST_MODE=${1:-complete} MISO_TEST_CANDIDATE="$candidate" MISO_TEST_CASE_ROOT="$case_root" \
      MISO_TEST_REAL_BASH="$real_bash" MISO_TEST_REAL_PYTHON="$real_python" \
      MISO_TEST_LAUNCH_LOG="$launch_log" PATH="$case_root/bin:$PATH" \
      "$real_bash" "$case_root/scripts/preflight-builtins-listening-111.sh" "${@:2}"
}
run_prepare() {
    MISO_TEST_MODE=${1:-complete} MISO_TEST_CANDIDATE="$candidate" MISO_TEST_CASE_ROOT="$case_root" \
      MISO_TEST_REAL_BASH="$real_bash" MISO_TEST_REAL_PYTHON="$real_python" \
      MISO_TEST_LAUNCH_LOG="$launch_log" PATH="$case_root/bin:$PATH" \
      "$real_bash" "$case_root/scripts/prepare-builtins-listening-111.sh" "${@:2}"
}
assert_zero() {
    "$real_python" - "$1" "$2" <<'PY'
import json,sys
value=json.load(open(sys.argv[1])); assert value["issue"]==111
assert value["preparation_invocations"]==int(sys.argv[2])
for key in ("audio_playback_invocations","human_listening_sessions","human_trial_attempts","valid_human_responses","reveal_invocations","completed_listening_records"): assert value[key]==0
PY
}

new_case preflight-success
run_preflight >/dev/null
[[ -x "$artifact/miso_engine_builtins_fixture_listening" && -f "$artifact/preflight.seal.json" &&
   ! -e "$launch_log" && "$(wc -l <"$case_root/cargo.log")" == 3 ]]
binary_hash="$(sha256sum "$artifact/miso_engine_builtins_fixture_listening")"
if run_preflight >/dev/null 2>&1; then exit 1; fi
[[ "$(sha256sum "$artifact/miso_engine_builtins_fixture_listening")" == "$binary_hash" && ! -e "$launch_log" ]]
assert_zero "$artifact/preparation.prelaunch.disposition.json" 0

for mutation in arguments missing-input input-mode input-hardlink dirty issue110-drift output-exists; do
    new_case "preflight-$mutation"
    args=(); mode=complete
    case "$mutation" in
      arguments) args=(--bad) ;;
      missing-input) rm "$artifact/inbox/source.mepcm" ;;
      input-mode) chmod 0644 "$artifact/inbox/seed.txt" ;;
      input-hardlink) rm "$artifact/inbox/seed.txt"; ln "$artifact/inbox/provenance.json" "$artifact/inbox/seed.txt" ;;
      dirty) mode=dirty ;;
      issue110-drift) printf drift >>"$case_root/target/issue110/completion.seal.json" ;;
      output-exists) printf protected >"$artifact/preflight.seal.json" ;;
    esac
    if run_preflight "$mode" "${args[@]}" >/dev/null 2>&1; then exit 1; fi
    [[ ! -e "$launch_log" && ! -e "$artifact/miso_engine_builtins_fixture_listening" &&
       ! -e "$case_root/cargo.log" ]]
    assert_zero "$artifact/preparation.prelaunch.disposition.json" 0
done

new_case prepare-success
run_preflight >/dev/null
run_prepare >/dev/null
[[ "$(wc -l <"$launch_log")" == 1 && -d "$artifact/packet" && ! -e "$artifact/preparation.partial" ]]
assert_zero "$artifact/preparation.disposition.json" 1
disposition_hash="$(sha256sum "$artifact/preparation.disposition.json")"
if run_prepare >/dev/null 2>&1; then exit 1; fi
[[ "$(sha256sum "$artifact/preparation.disposition.json")" == "$disposition_hash" && "$(wc -l <"$launch_log")" == 1 ]]

for mutation in arguments extra-member input-mode input-hardlink dirty issue110-drift seal-drift authority-drift; do
    new_case "prepare-$mutation"
    run_preflight >/dev/null
    rm -f "$case_root/status-seen"
    args=(); mode=complete
    case "$mutation" in
      arguments) args=(--bad) ;;
      extra-member) printf extra >"$artifact/extra" ;;
      input-mode) chmod 0644 "$artifact/inbox/seed.txt" ;;
      input-hardlink) rm "$artifact/inbox/seed.txt"; ln "$artifact/inbox/provenance.json" "$artifact/inbox/seed.txt" ;;
      dirty) mode=dirty ;;
      issue110-drift) printf drift >>"$case_root/target/issue110/completion.seal.json" ;;
      seal-drift) mode=seal_drift ;;
      authority-drift) mode=authority_drift ;;
    esac
    if run_prepare "$mode" "${args[@]}" >/dev/null 2>&1; then exit 1; fi
    [[ ! -e "$launch_log" && ! -e "$artifact/packet" ]]
    assert_zero "$artifact/preparation.prelaunch.disposition.json" 0
done

new_case prepare-no-clobber
run_preflight >/dev/null
printf protected >"$artifact/packet"
protected_hash="$(sha256sum "$artifact/packet")"
if run_prepare >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$launch_log" && "$(sha256sum "$artifact/packet")" == "$protected_hash" ]]
assert_zero "$artifact/preparation.prelaunch.disposition.json" 0

for shape in regular symlink hardlink; do
    new_case "prepare-stderr-$shape"
    run_preflight >/dev/null
    printf protected >"$case_root/protected-stderr"
    protected="$artifact/preparation.stderr"
    case "$shape" in
      regular) printf sentinel >"$protected" ;;
      symlink) ln -s "$case_root/protected-stderr" "$protected" ;;
      hardlink) ln "$case_root/protected-stderr" "$protected" ;;
    esac
    before="$(stat -c '%F:%h:%s:%i' "$protected")"
    sentinel_hash="$(sha256sum "$protected")"
    if run_prepare >/dev/null 2>&1; then exit 1; fi
    [[ ! -e "$launch_log" && "$(stat -c '%F:%h:%s:%i' "$protected")" == "$before" &&
       "$(sha256sum "$protected")" == "$sentinel_hash" ]]
    assert_zero "$artifact/preparation.prelaunch.disposition.json" 0
done

for mutation in renderer-failure post-render-drift; do
    new_case "$mutation"
    run_preflight >/dev/null
    rm -f "$case_root/status-seen"
    mode=${mutation//-/_}
    if run_prepare "$mode" >/dev/null 2>&1; then exit 1; fi
    [[ "$(wc -l <"$launch_log")" == 1 && -d "$artifact/preparation.partial" &&
       -f "$artifact/preparation.disposition.json" && ! -e "$artifact/packet" ]]
    assert_zero "$artifact/preparation.disposition.json" 1
done

[[ "$(issue111_state)" == "$real_before" ]]
[[ ! -e "$root/target/issue33" && ! -L "$root/target/issue33" ]]
printf 'Issue-111 hermetic lifecycle: PASS (real preflight/render/playback/session/trial/response/reveal/result=0/0/0/0/0/0/0/0)\n'
