#!/usr/bin/env bash
# Sole no-playback Issue-033 machine preparation wrapper.
set -euo pipefail
script_directory="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
root="$(cd "$script_directory/.." && pwd)"
cd "$root"
artifact_directory=target/issue33
inbox="$artifact_directory/inbox"
preparation_seal="$artifact_directory/preparation.seal.json"
binary="$artifact_directory/miso_engine_builtins_fixture_listening"
preflight_seal="$artifact_directory/preflight.seal.json"
partial="$artifact_directory/preparation.partial"
packet="$artifact_directory/packet"
stderr_log="$artifact_directory/preparation.stderr"
prelaunch="$artifact_directory/preparation.prelaunch.disposition.json"
disposition="$artifact_directory/preparation.disposition.json"
for tool in awk bash chmod cp find git mkdir mktemp mv python3 rm sha256sum sort stat; do
    command -v "$tool" >/dev/null 2>&1 || { printf 'Issue-033 preparation bootstrap tool unavailable: %s\n' "$tool" >&2; exit 1; }
done
mkdir -p "$artifact_directory"
[[ ! -L "$artifact_directory" ]] || { printf 'Issue-033 artifact directory symlink\n' >&2; exit 1; }
for terminal in "$prelaunch" "$disposition"; do
    [[ ! -e "$terminal" && ! -L "$terminal" ]] || {
        printf 'refusing consumed Issue-033 preparation authority: %s\n' "$terminal" >&2
        exit 1
    }
done
scratch="$(mktemp -d "${TMPDIR:-/tmp}/miso-listening-033-prepare.XXXXXX")"
launched=0
completed=0
status=1
reason=prelaunch_failure
commit= tree= binary_sha= packet_sha=null
hash_file() { sha256sum "$1" | awk '{print $1}'; }
publish_disposition() {
    local destination=$1 kind=$2 state=$3 why=$4
    local temporary="$scratch/disposition.json"
    python3 - "$kind" "$state" "$why" "$launched" "$commit" "$tree" "$binary_sha" "$packet_sha" "$temporary" <<'PY'
import json, sys
kind, status, reason, launched, commit, tree, binary_sha, packet_sha, output = sys.argv[1:]
is_launched = int(launched)
record = {
    "audio_playback_invocations": 0,
    "binary_sha256": binary_sha or None,
    "candidate_commit": commit or None,
    "candidate_tree": tree or None,
    "completed_listening_records": 0,
    "human_listening_sessions": 0,
    "human_trial_attempts": 0,
    "issue": 33,
    "kind": kind,
    "packet_preparation_sha256": None if packet_sha == "null" else packet_sha,
    "preflight_invocations": 1 if commit else 0,
    "preparation_invocations": is_launched,
    "reason": reason,
    "reveal_invocations": 0,
    "schema_version": 1,
    "status": status,
    "valid_human_responses": 0,
}
with open(output, "xb") as destination:
    destination.write((json.dumps(record, sort_keys=True, separators=(",", ":")) + "\n").encode())
PY
    chmod 0444 "$temporary"
    mv -n "$temporary" "$destination"
    [[ ! -e "$temporary" && -f "$destination" && ! -L "$destination" && "$(stat -c %h "$destination")" == 1 ]]
}
on_exit() {
    local exit_status=$?
    trap - EXIT INT TERM
    if [[ "$completed" == 0 ]]; then
        set +e
        if [[ "$launched" == 1 ]]; then
            publish_disposition "$disposition" issue033_listening_preparation_disposition FAIL "$reason"
        else
            publish_disposition "$prelaunch" issue033_listening_prelaunch_disposition FAIL "$reason"
        fi
    fi
    rm -rf "$scratch"
    exit "$exit_status"
}
trap on_exit EXIT
trap 'reason=interrupted; status=130; exit 130' INT TERM

[[ $# == 0 ]] || { reason=invalid_arguments; exit 2; }
required_members=$'inbox\nmiso_engine_builtins_fixture_listening\npreflight.seal.json\npreparation.seal.json'
[[ "$(find "$artifact_directory" -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)" == "$required_members" ]] || {
    reason=artifact_membership; exit 1;
}
[[ -d "$inbox" && ! -L "$inbox" && "$(stat -c %a "$inbox")" == 700 ]] || { reason=inbox_shape; exit 1; }
[[ "$(find "$inbox" -mindepth 1 -maxdepth 1 -printf '%f\n' | sort)" == $'provenance.json\nseed.txt\nsource.mepcm' ]] || {
    reason=inbox_membership; exit 1;
}
for path in "$preparation_seal" "$binary" "$preflight_seal" "$inbox/source.mepcm" \
    "$inbox/provenance.json" "$inbox/seed.txt"; do
    [[ -f "$path" && ! -L "$path" && "$(stat -c %h "$path")" == 1 ]] || { reason=authority_shape; exit 1; }
done
[[ -x "$binary" && "$(stat -c %a "$preparation_seal")" == 444 &&
   "$(stat -c %a "$preflight_seal")" == 444 && "$(stat -c %a "$inbox/source.mepcm")" == 400 &&
   "$(stat -c %a "$inbox/provenance.json")" == 600 && "$(stat -c %a "$inbox/seed.txt")" == 600 ]] || {
    reason=authority_mode; exit 1;
}
[[ ! -e "$partial" && ! -L "$partial" && ! -e "$packet" && ! -L "$packet" &&
   ! -e "$stderr_log" && ! -L "$stderr_log" ]] || { reason=existing_output; exit 1; }

branch="$(git branch --show-current)"
commit="$(git rev-parse --verify HEAD)"
tree="$(git rev-parse 'HEAD^{tree}')"
[[ "$branch" == codex/listening-033 && -z "$(git status --porcelain=v1 --untracked-files=normal)" ]] || {
    reason=dirty_candidate; exit 1;
}
binary_sha="$(hash_file "$binary")"
python3 -I -B scripts/check-builtins-listening-033.py --source "$inbox/source.mepcm" "$inbox/provenance.json" || {
    reason=source_or_provenance; exit 1;
}
python3 -I -B scripts/check-builtins-listening-033.py --seal preparation "$preparation_seal" "$root" "$commit" "$tree" || {
    reason=preparation_seal; exit 1;
}
python3 -I -B scripts/check-builtins-listening-033.py --seal preflight "$preflight_seal" "$root" "$commit" "$tree" || {
    reason=preflight_seal; exit 1;
}
bash scripts/check-builtins-listening-033.sh >/dev/null || { reason=policy; exit 1; }

# Final authority check immediately before the first persistent render output is created.
[[ "$commit" == "$(git rev-parse --verify HEAD)" && "$tree" == "$(git rev-parse 'HEAD^{tree}')" &&
   -z "$(git status --porcelain=v1 --untracked-files=normal)" && "$binary_sha" == "$(hash_file "$binary")" ]] || {
    reason=authority_drift; exit 1;
}
python3 -I -B scripts/check-builtins-listening-033.py --source "$inbox/source.mepcm" "$inbox/provenance.json" || {
    reason=authority_drift; exit 1;
}
python3 -I -B scripts/check-builtins-listening-033.py --seal preflight "$preflight_seal" "$root" "$commit" "$tree" || {
    reason=authority_drift; exit 1;
}

umask 077
mkdir "$partial"
exec {stderr_fd}>"$stderr_log"
[[ "$(stat -c %h "$stderr_log")" == 1 ]] || { reason=output_shape; exit 1; }
reason=renderer_failed
launched=1
set +e
"$binary" --render "$inbox/source.mepcm" \
    fixtures/conformance/v1/prng-noise-048000-dual-mono.mepcm \
    "$inbox/provenance.json" "$inbox/seed.txt" "$partial" 2>&"$stderr_fd"
status=$?
set -e
exec {stderr_fd}>&-
[[ "$status" == 0 ]] || exit "$status"
reason=packet_validation_failed
python3 -I -B scripts/check-builtins-listening-033.py --renderer-output "$partial"
[[ "$commit" == "$(git rev-parse --verify HEAD)" && "$tree" == "$(git rev-parse 'HEAD^{tree}')" &&
   -z "$(git status --porcelain=v1 --untracked-files=normal)" && "$binary_sha" == "$(hash_file "$binary")" ]] || {
    reason=post_render_authority_drift; exit 1;
}
python3 -I -B scripts/check-builtins-listening-033.py --seal preflight "$preflight_seal" "$root" "$commit" "$tree" || {
    reason=post_render_authority_drift; exit 1;
}
cp dsp-research/listening/issue033/FACILITATOR.md "$partial/public/FACILITATOR.md"
cp dsp-research/listening/issue007-filter-abx-preregistration.md "$partial/public/filter-preregistration.md"
cp dsp-research/listening/issue007-matrix-ramp-preregistration.md "$partial/public/matrix-preregistration.md"
cp dsp-research/listening/issue033/preparation.schema.json "$partial/public/preparation.schema.json"
cp dsp-research/listening/issue033/response.schema.json "$partial/public/response.schema.json"
cp dsp-research/listening/issue033/reveal.schema.json "$partial/public/reveal.schema.json"
cp dsp-research/listening/issue033/qualification.schema.json "$partial/public/qualification.schema.json"
cp dsp-research/listening/issue033/response-form.jsonl "$partial/public/response-form.jsonl"
chmod 0444 "$partial/public/"*
python3 -I -B scripts/check-builtins-listening-033.py --seal preflight "$preflight_seal" "$root" "$commit" "$tree" || {
    reason=packet_input_drift; exit 1;
}
python3 -I -B scripts/check-builtins-listening-033.py --assemble \
    "$partial/public/render-manifest.json" "$commit" "$tree" "$partial/public/preparation.json"
chmod 0444 "$partial/public/preparation.json"
python3 -I -B scripts/check-builtins-listening-033.py --packet "$partial"
packet_sha="$(hash_file "$partial/public/preparation.json")"
mv -T -n "$partial" "$packet"
[[ ! -e "$partial" && -d "$packet" && ! -L "$packet" ]] || { reason=packet_publication; exit 1; }
publish_disposition "$disposition" issue033_listening_preparation_disposition PASS complete
completed=1
trap - EXIT INT TERM
rm -rf "$scratch"
printf '%s\n' "$packet/public/preparation.json"
