#!/usr/bin/env bash
# Sole zero-render Issue-033 preflight. It builds but never executes the renderer.
set -euo pipefail
script_directory="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
root="$(cd "$script_directory/.." && pwd)"
cd "$root"
artifact_directory=target/issue33
inbox="$artifact_directory/inbox"
preparation_seal="$artifact_directory/preparation.seal.json"
binary="$artifact_directory/miso_engine_builtins_fixture_listening"
preflight_seal="$artifact_directory/preflight.seal.json"
prelaunch="$artifact_directory/preparation.prelaunch.disposition.json"
disposition="$artifact_directory/preparation.disposition.json"
fail() { printf 'Issue-033 preflight failure: %s\n' "$1" >&2; exit 1; }
require_file() { [[ -f "$1" && ! -L "$1" && "$(stat -c %h "$1")" == 1 ]]; }
mode() { stat -c %a "$1"; }
[[ -d "$artifact_directory" && ! -L "$artifact_directory" ]] || fail 'artifact directory'
for terminal in "$prelaunch" "$disposition"; do
    [[ ! -e "$terminal" && ! -L "$terminal" ]] || fail 'consumed preparation authority'
done
completed=0
failure_reason=preflight_failure
scratch=
publish_prelaunch() {
    local temporary
    temporary="$(mktemp "$artifact_directory/.preparation.prelaunch.XXXXXX")"
    printf '{"audio_playback_invocations":0,"binary_sha256":null,"candidate_commit":null,"candidate_tree":null,"completed_listening_records":0,"human_listening_sessions":0,"human_trial_attempts":0,"issue":33,"kind":"issue033_listening_prelaunch_disposition","packet_preparation_sha256":null,"preflight_invocations":1,"preparation_invocations":0,"reason":"%s","reveal_invocations":0,"schema_version":1,"status":"FAIL","valid_human_responses":0}\n' \
        "$failure_reason" >"$temporary"
    chmod 0444 "$temporary"
    mv -n "$temporary" "$prelaunch"
}
on_exit() {
    local exit_status=$?
    trap - EXIT
    if [[ "$completed" == 0 ]]; then
        set +e
        publish_prelaunch
    fi
    [[ -z "$scratch" ]] || rm -rf "$scratch"
    exit "$exit_status"
}
trap on_exit EXIT
[[ $# == 0 ]] || { failure_reason=invalid_arguments; printf 'usage: %s\n' "$0" >&2; exit 2; }
for tool in awk bash cargo chmod cp find git mkdir mktemp mv python3 rm sha256sum sort stat; do
    command -v "$tool" >/dev/null 2>&1 || {
        failure_reason="tool_unavailable_$tool"
        printf 'Issue-033 preflight tool unavailable: %s\n' "$tool" >&2
        exit 1
    }
done

branch="$(git branch --show-current)"
commit="$(git rev-parse --verify HEAD)"
tree="$(git rev-parse 'HEAD^{tree}')"
[[ "$branch" == codex/listening-033 && -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] ||
    fail 'candidate identity/cleanliness'
[[ -d "$inbox" && ! -L "$inbox" ]] ||
    fail 'artifact/inbox directory'
[[ "$(find "$artifact_directory" -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)" == $'inbox\npreparation.seal.json' ]] ||
    fail 'preflight input membership'
[[ "$(find "$inbox" -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)" == $'provenance.json\nseed.txt\nsource.mepcm' ]] ||
    fail 'inbox membership'
[[ "$(mode "$inbox")" == 700 ]] || fail 'inbox mode'
for path in "$preparation_seal" "$inbox/source.mepcm" "$inbox/provenance.json" "$inbox/seed.txt"; do
    require_file "$path" || fail "regular one-link input: $path"
done
[[ "$(mode "$preparation_seal")" == 444 && "$(mode "$inbox/source.mepcm")" == 400 &&
   "$(mode "$inbox/provenance.json")" == 600 && "$(mode "$inbox/seed.txt")" == 600 ]] ||
    fail 'input modes'
[[ ! -e "$binary" && ! -L "$binary" && ! -e "$preflight_seal" && ! -L "$preflight_seal" ]] ||
    fail 'preflight output exists'

python3 -I -B scripts/check-builtins-listening-033.py --provenance "$inbox/provenance.json"
python3 -I -B scripts/check-builtins-listening-033.py --source "$inbox/source.mepcm" "$inbox/provenance.json"
python3 -I -B scripts/check-builtins-listening-033.py --seal preparation "$preparation_seal" "$root" "$commit" "$tree"
seed="$(<"$inbox/seed.txt")"
[[ "$seed" =~ ^(0|[1-9][0-9]*)$ && ${#seed} -le 20 ]] || fail 'seed format'
python3 - "$seed" <<'PY' || fail 'seed range'
import sys
value = int(sys.argv[1], 10)
raise SystemExit(0 if value <= 2**64 - 1 else 1)
PY

scratch="$(mktemp -d "${TMPDIR:-/tmp}/miso-listening-033-preflight.XXXXXX")"
bash scripts/check-builtins-listening-033.sh || { failure_reason=policy; exit 1; }
bash scripts/test-builtins-listening-033-policy.sh || { failure_reason=policy_mutation; exit 1; }
bash scripts/test-builtins-listening-033.sh || { failure_reason=lifecycle; exit 1; }
cargo test --locked -p miso-engine-builtins-fixture --bin miso_engine_builtins_fixture_listening || {
    failure_reason=renderer_tests; exit 1;
}
cargo check --locked -p miso-engine-builtins-fixture --all-targets || {
    failure_reason=compile_check; exit 1;
}
CARGO_TARGET_DIR="$scratch/build" cargo build --locked --release -p miso-engine-builtins-fixture \
    --bin miso_engine_builtins_fixture_listening || { failure_reason=release_build; exit 1; }
built="$scratch/build/release/miso_engine_builtins_fixture_listening"
[[ -x "$built" && ! -L "$built" ]] || fail 'built binary'
[[ "$commit" == "$(git rev-parse --verify HEAD)" && "$tree" == "$(git rev-parse 'HEAD^{tree}')" &&
   -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || fail 'candidate drift'
python3 -I -B scripts/check-builtins-listening-033.py --source "$inbox/source.mepcm" "$inbox/provenance.json"
python3 -I -B scripts/check-builtins-listening-033.py --seal preparation "$preparation_seal" "$root" "$commit" "$tree"

temporary_binary="$(mktemp "$artifact_directory/.miso_engine_builtins_fixture_listening.XXXXXX")"
cp "$built" "$temporary_binary"
chmod 0555 "$temporary_binary"
mv -n "$temporary_binary" "$binary"
[[ ! -e "$temporary_binary" && -x "$binary" && ! -L "$binary" && "$(stat -c %h "$binary")" == 1 ]] ||
    fail 'binary publication'
temporary_seal="$(mktemp "$artifact_directory/.preflight.seal.XXXXXX")"
rm "$temporary_seal"
python3 -I -B scripts/check-builtins-listening-033.py --write-seal preflight "$temporary_seal" "$root" "$commit" "$tree"
chmod 0444 "$temporary_seal"
mv -n "$temporary_seal" "$preflight_seal"
[[ ! -e "$temporary_seal" && -f "$preflight_seal" && ! -L "$preflight_seal" &&
   "$(stat -c %h "$preflight_seal")" == 1 ]] || fail 'preflight seal publication'
[[ "$(find "$artifact_directory" -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)" == \
   $'inbox\nmiso_engine_builtins_fixture_listening\npreflight.seal.json\npreparation.seal.json' ]] ||
    fail 'preparation-ready membership'
printf 'Issue-033 preflight: PASS counters=1/0/0/0/0/0/0\n'
completed=1
trap - EXIT
rm -rf "$scratch"
