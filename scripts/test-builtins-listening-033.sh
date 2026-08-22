#!/usr/bin/env bash
# Hermetic lifecycle proof. No real renderer, source render, player, or human evidence is invoked.
set -euo pipefail
script_directory="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
root="$(cd "$script_directory/.." && pwd)"
issue33_state() {
    if [[ ! -e "$root/target/issue33" && ! -L "$root/target/issue33" ]]; then
        printf 'ABSENT\n'
        return
    fi
    find "$root/target/issue33" -mindepth 0 -printf '%P\t%y\t%s\t%n\n' | sort
    find "$root/target/issue33" -type f -print0 | sort -z | xargs -0 -r sha256sum
}
real_issue33_before="$(issue33_state)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
template="$scratch/template"
mkdir -p "$template"/{bin,scripts,target/issue110,target/issue33/inbox,tools/miso-engine-builtins-fixture/src,crates/miso-engine-builtins/src,crates/miso-engine-builtins-compiler/src,fixtures/builtins/v1,fixtures/conformance/v1,dsp-research/listening/issue033}
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
cp "$root/scripts/check-builtins-listening.sh" "$root/scripts/check-builtins-listening-033.py" \
    "$root/scripts/check-builtins-listening-033.sh" "$root/scripts/test-builtins-listening-033.sh" \
    "$root/scripts/test-builtins-listening-033-policy.sh" "$root/scripts/preflight-builtins-listening-033.sh" \
    "$root/scripts/prepare-builtins-listening-033.sh" "$template/scripts/"
cp -a "$root/target/issue110/." "$template/target/issue110/"
printf fake >"$template/target/issue33/inbox/source.mepcm"
printf '{}\n' >"$template/target/issue33/inbox/provenance.json"
printf '42\n' >"$template/target/issue33/inbox/seed.txt"
printf '{}\n' >"$template/target/issue33/preparation.seal.json"
chmod 0700 "$template/target/issue33/inbox"
chmod 0400 "$template/target/issue33/inbox/source.mepcm"
chmod 0600 "$template/target/issue33/inbox/provenance.json" "$template/target/issue33/inbox/seed.txt"
chmod 0444 "$template/target/issue33/preparation.seal.json"

real_bash="$(command -v bash)"
real_python="$(command -v python3)"
cat >"$template/bin/git" <<'EOF'
#!/bin/bash
case "$*" in
  *'branch --show-current'*) printf 'codex/listening-033\n' ;;
  *"rev-parse --verify HEAD"*|*"rev-parse HEAD"*) printf '%s\n' "$MISO_TEST_CANDIDATE" ;;
  *"HEAD^{tree}"*) printf '%s\n' "$MISO_TEST_CANDIDATE" ;;
  *status*)
    if [[ "${MISO_TEST_MODE:-}" == dirty ||
          ( "${MISO_TEST_MODE:-}" == pre_render_drift && -e "$MISO_TEST_CASE_ROOT/status-seen" ) ||
          ( "${MISO_TEST_MODE:-}" == post_render_drift && -e "$MISO_TEST_LAUNCH_LOG" ) ]]; then
      printf ' M synthetic\n'
    fi
    : >"$MISO_TEST_CASE_ROOT/status-seen" ;;
  *) exit 91 ;;
esac
EOF
cat >"$template/bin/bash" <<'EOF'
#!/bin/bash
case "${1:-}" in
  scripts/test-builtins-listening-033-policy.sh|scripts/test-builtins-listening-033.sh) exit 0 ;;
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
if [[ "$*" == *'--assemble'* ]]; then
  output=${!#}; printf '{}\n' >"$output"; exit 0
fi
if [[ "${MISO_TEST_MODE:-}" == binary_drift && "$*" == *'--seal preflight'* &&
      ! -e "$MISO_TEST_CASE_ROOT/binary-drifted" ]]; then
  printf mutation >>"$MISO_TEST_BINARY"
  : >"$MISO_TEST_CASE_ROOT/binary-drifted"
  exit 0
fi
if [[ "${MISO_TEST_MODE:-}" == post_preflight_schema_drift && "$*" == *'--seal preflight'* ]]; then exit 80; fi
if [[ "${MISO_TEST_MODE:-}" == post_render_schema_drift && "$*" == *'--seal preflight'* &&
      -e "$MISO_TEST_LAUNCH_LOG" ]]; then exit 81; fi
if [[ "${MISO_TEST_MODE:-}" == packet_drift && "$*" == *'--packet'* ]]; then
  printf mutation >>"$MISO_TEST_CASE_ROOT/target/issue33/preparation.partial/public/response.schema.json"
  exit 82
fi
if [[ "${MISO_TEST_MODE:-}" == invalid_provenance &&
      ( "$*" == *'--provenance'* || "$*" == *'--source'* ) ]]; then exit 79; fi
if [[ "$*" == *check-builtins-listening-033.py* ]]; then exit 0; fi
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
chmod 0700 "$output/private"
printf '{}\n' >"$output/public/render-manifest.json"
printf '{}\n' >"$output/private/assignment-key.json"
printf '{}\n' >"$output/private/source-provenance.json"
chmod 0600 "$output/private/"*
for token in 00000000000000000000000000000001 00000000000000000000000000000002 00000000000000000000000000000003 00000000000000000000000000000004; do
  printf RIFF >"$output/public/$token.wav"
done
if [[ "${MISO_TEST_MODE:-}" == renderer_failure ]]; then printf partial >"$output/public/partial"; exit 73; fi
if [[ "${MISO_TEST_MODE:-}" == post_render_schema_drift ]]; then
  printf mutation >>"$MISO_TEST_CASE_ROOT/dsp-research/listening/issue033/response.schema.json"
fi
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
    artifact="$case_root/target/issue33"
}
run_preflight() {
    MISO_TEST_MODE=${1:-complete} MISO_TEST_CANDIDATE="$candidate" MISO_TEST_CASE_ROOT="$case_root" \
      MISO_TEST_REAL_BASH="$real_bash" MISO_TEST_REAL_PYTHON="$real_python" \
      MISO_TEST_LAUNCH_LOG="$launch_log" MISO_TEST_BINARY="$artifact/miso_engine_builtins_fixture_listening" PATH="$case_root/bin:$PATH" \
      "$real_bash" "$case_root/scripts/preflight-builtins-listening-033.sh" "${@:2}"
}
run_preflight_without_cargo() {
    local hidden="$case_root/bin/cargo.hidden"
    mv "$case_root/bin/cargo" "$hidden"
    MISO_TEST_MODE=complete MISO_TEST_CANDIDATE="$candidate" MISO_TEST_CASE_ROOT="$case_root" \
      MISO_TEST_REAL_BASH="$real_bash" MISO_TEST_REAL_PYTHON="$real_python" \
      MISO_TEST_LAUNCH_LOG="$launch_log" MISO_TEST_BINARY="$artifact/miso_engine_builtins_fixture_listening" \
      PATH="$case_root/bin:/usr/bin:/bin" \
      "$real_bash" "$case_root/scripts/preflight-builtins-listening-033.sh"
}
run_prepare() {
    MISO_TEST_MODE=${1:-complete} MISO_TEST_CANDIDATE="$candidate" MISO_TEST_CASE_ROOT="$case_root" \
      MISO_TEST_REAL_BASH="$real_bash" MISO_TEST_REAL_PYTHON="$real_python" \
      MISO_TEST_LAUNCH_LOG="$launch_log" MISO_TEST_BINARY="$artifact/miso_engine_builtins_fixture_listening" PATH="$case_root/bin:$PATH" \
      "$real_bash" "$case_root/scripts/prepare-builtins-listening-033.sh" "${@:2}"
}
assert_preflight_consumed() {
    "$real_python" - "$artifact/preparation.prelaunch.disposition.json" <<'PY'
import json,sys
v=json.load(open(sys.argv[1])); assert v["preflight_invocations"]==1 and v["preparation_invocations"]==0
for k in ("audio_playback_invocations","human_listening_sessions","human_trial_attempts","valid_human_responses","reveal_invocations","completed_listening_records"): assert v[k]==0
PY
}

new_case preflight-success
run_preflight >/dev/null
[[ -x "$artifact/miso_engine_builtins_fixture_listening" && -f "$artifact/preflight.seal.json" &&
   ! -e "$launch_log" && "$(wc -l <"$case_root/cargo.log")" == 3 ]]
binary_hash="$(sha256sum "$artifact/miso_engine_builtins_fixture_listening")"
seal_hash="$(sha256sum "$artifact/preflight.seal.json")"
if run_preflight >/dev/null 2>&1; then exit 1; fi
[[ "$(sha256sum "$artifact/miso_engine_builtins_fixture_listening")" == "$binary_hash" &&
   "$(sha256sum "$artifact/preflight.seal.json")" == "$seal_hash" && ! -e "$launch_log" ]]
assert_preflight_consumed

for mutation in arguments missing-input invalid-provenance extra-inbox bad-mode hardlink dirty issue110-drift; do
    new_case "preflight-$mutation"
    case "$mutation" in
      arguments) args=(--bad) ;;
      missing-input) rm "$artifact/inbox/source.mepcm"; args=() ;;
      invalid-provenance) args=() ;;
      extra-inbox) printf extra >"$artifact/inbox/extra"; args=() ;;
      bad-mode) chmod 0644 "$artifact/inbox/seed.txt"; args=() ;;
      hardlink) rm "$artifact/inbox/seed.txt"; ln "$artifact/inbox/provenance.json" "$artifact/inbox/seed.txt"; args=() ;;
      dirty) args=();;
      issue110-drift) printf mutation >>"$case_root/target/issue110/completion.seal.json"; args=() ;;
    esac
    mode=complete
    [[ "$mutation" == dirty ]] && mode=dirty
    [[ "$mutation" == invalid-provenance ]] && mode=invalid_provenance
    if run_preflight "$mode" "${args[@]}" >/dev/null 2>&1; then printf 'preflight mutation passed: %s\n' "$mutation" >&2; exit 1; fi
    [[ ! -e "$launch_log" && ! -e "$artifact/miso_engine_builtins_fixture_listening" && ! -e "$artifact/preflight.seal.json" ]]
    assert_preflight_consumed
done

new_case preflight-missing-tool
if run_preflight_without_cargo >/dev/null 2>&1; then exit 1; fi
[[ ! -e "$launch_log" && ! -e "$artifact/miso_engine_builtins_fixture_listening" &&
   ! -e "$artifact/preflight.seal.json" ]]
assert_preflight_consumed

for artifact_name in binary seal; do
  for shape in regular symlink hardlink; do
    new_case "preflight-no-clobber-$artifact_name-$shape"
    case "$artifact_name" in
      binary) protected="$artifact/miso_engine_builtins_fixture_listening";;
      seal) protected="$artifact/preflight.seal.json";;
    esac
    printf protected >"$case_root/protected-base"
    case "$shape" in
      regular) printf protected >"$protected";;
      symlink) ln -s "$case_root/protected-base" "$protected";;
      hardlink) ln "$case_root/protected-base" "$protected";;
    esac
    before="$(stat -c '%F:%h:%s:%i' "$protected")"
    if run_preflight >/dev/null 2>&1; then exit 1; fi
    [[ ! -e "$launch_log" && ! -e "$case_root/cargo.log" &&
       "$(stat -c '%F:%h:%s:%i' "$protected")" == "$before" ]]
    assert_preflight_consumed
  done
done

new_case prepare-success
run_preflight >/dev/null
result="$(run_prepare)"
[[ "$result" == target/issue33/packet/public/preparation.json && "$(wc -l <"$launch_log")" == 1 &&
   -d "$artifact/packet" && ! -e "$artifact/preparation.partial" ]]
python3 - "$artifact/preparation.disposition.json" <<'PY'
import json, sys
value=json.load(open(sys.argv[1], encoding="utf-8"))
assert value["status"]=="PASS" and value["reason"]=="complete"
assert value["preflight_invocations"]==1 and value["preparation_invocations"]==1
for key in ("audio_playback_invocations","human_listening_sessions","human_trial_attempts","valid_human_responses","reveal_invocations","completed_listening_records"):
    assert value[key]==0
PY
disposition_hash="$(sha256sum "$artifact/preparation.disposition.json")"
if run_prepare >/dev/null 2>&1; then exit 1; fi
[[ "$(sha256sum "$artifact/preparation.disposition.json")" == "$disposition_hash" && "$(wc -l <"$launch_log")" == 1 ]]

for mutation in arguments missing-input invalid-provenance extra-member input-mode input-hardlink dirty issue110-drift post-preflight-schema-drift pre-render-drift binary-drift; do
    new_case "prepare-$mutation"
    run_preflight >/dev/null
    rm -f "$case_root/status-seen"
    case "$mutation" in
      arguments) args=(--bad) ;;
      missing-input) rm "$artifact/inbox/source.mepcm"; args=() ;;
      invalid-provenance) args=() ;;
      extra-member) printf extra >"$artifact/extra"; args=() ;;
      input-mode) chmod 0644 "$artifact/inbox/seed.txt"; args=() ;;
      input-hardlink) rm "$artifact/inbox/seed.txt"; ln "$artifact/inbox/provenance.json" "$artifact/inbox/seed.txt"; args=() ;;
      dirty) args=() ;;
      issue110-drift) printf mutation >>"$case_root/target/issue110/completion.seal.json"; args=() ;;
      post-preflight-schema-drift) printf mutation >>"$case_root/dsp-research/listening/issue033/response.schema.json"; args=() ;;
      pre-render-drift|binary-drift) args=() ;;
    esac
    mode=complete
    case "$mutation" in
      dirty) mode=dirty;;
      invalid-provenance) mode=invalid_provenance;;
      post-preflight-schema-drift) mode=post_preflight_schema_drift;;
      pre-render-drift) mode=pre_render_drift;;
      binary-drift) mode=binary_drift;;
    esac
    if run_prepare "$mode" "${args[@]}" >/dev/null 2>&1; then printf 'prepare mutation passed: %s\n' "$mutation" >&2; exit 1; fi
    [[ ! -e "$launch_log" && -f "$artifact/preparation.prelaunch.disposition.json" && ! -e "$artifact/packet" ]]
    python3 - "$artifact/preparation.prelaunch.disposition.json" <<'PY'
import json,sys
v=json.load(open(sys.argv[1])); assert v["preparation_invocations"]==0
for k in ("audio_playback_invocations","human_listening_sessions","human_trial_attempts","valid_human_responses","reveal_invocations","completed_listening_records"): assert v[k]==0
PY
done

new_case post-render-drift
run_preflight >/dev/null
rm -f "$case_root/status-seen"
if run_prepare post_render_drift >/dev/null 2>&1; then exit 1; fi
[[ "$(wc -l <"$launch_log")" == 1 && -d "$artifact/preparation.partial" &&
   -f "$artifact/preparation.disposition.json" && ! -e "$artifact/packet" ]]

for mutation in post-render-schema-drift packet-drift; do
    new_case "$mutation"
    run_preflight >/dev/null
    mode=${mutation//-/_}
    if run_prepare "$mode" >/dev/null 2>&1; then exit 1; fi
    [[ "$(wc -l <"$launch_log")" == 1 && -d "$artifact/preparation.partial" &&
       -f "$artifact/preparation.disposition.json" && ! -e "$artifact/packet" ]]
done

for artifact_name in partial packet stderr prelaunch disposition; do
  for shape in regular symlink hardlink; do
    new_case "no-clobber-$artifact_name-$shape"
    run_preflight >/dev/null
    case "$artifact_name" in
      partial) protected="$artifact/preparation.partial";;
      packet) protected="$artifact/packet";;
      stderr) protected="$artifact/preparation.stderr";;
      prelaunch) protected="$artifact/preparation.prelaunch.disposition.json";;
      disposition) protected="$artifact/preparation.disposition.json";;
    esac
    printf protected >"$case_root/protected-base"
    case "$shape" in
      regular) printf protected >"$protected";;
      symlink) ln -s "$case_root/protected-base" "$protected";;
      hardlink) ln "$case_root/protected-base" "$protected";;
    esac
    before="$(stat -c '%F:%h:%s:%i' "$protected")"
    if run_prepare >/dev/null 2>&1; then exit 1; fi
    [[ ! -e "$launch_log" && "$(stat -c '%F:%h:%s:%i' "$protected")" == "$before" ]]
  done
done

new_case renderer-failure
run_preflight >/dev/null
if run_prepare renderer_failure >/dev/null 2>&1; then exit 1; fi
[[ "$(wc -l <"$launch_log")" == 1 && -d "$artifact/preparation.partial" &&
   -f "$artifact/preparation.partial/public/partial" && -f "$artifact/preparation.stderr" &&
   -f "$artifact/preparation.disposition.json" && ! -e "$artifact/packet" ]]
python3 - "$artifact/preparation.disposition.json" <<'PY'
import json,sys
v=json.load(open(sys.argv[1])); assert v["status"]=="FAIL" and v["preparation_invocations"]==1
PY

[[ "$(issue33_state)" == "$real_issue33_before" ]]
printf 'Issue-033 hermetic lifecycle: PASS (real preflight/render/playback/session/trial/response/reveal/result=0/0/0/0/0/0/0/0)\n'
