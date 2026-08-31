#!/usr/bin/env bash
# Mutation test proving `check-flac-decoder.sh` actually scans the sidecar it names in its own
# subject (the FLAC decoder moved to `sidecars/flac-decoder` in this issue). Unlike the other
# `test-*-policy.sh` harnesses, `check-flac-decoder.sh` takes no workspace-root argument -- it
# always resolves paths from its own location -- so this test mutates the real tree in place,
# under a trap that restores it unconditionally, rather than building an isolated fixture.
set -euo pipefail

script_directory="$(cd "$(dirname "$0")/.." && pwd)"
policy_script="$script_directory/scripts/check-flac-decoder.sh"
loader="$script_directory/sidecars/flac-decoder/miso-engine-flac-decoder.js"

[[ -f "$loader" ]] || {
    printf 'test-flac-decoder: missing sidecar loader: %s\n' "$loader" >&2
    exit 1
}

backup="$(mktemp)"
cp -- "$loader" "$backup"
cleanup() {
    cp -- "$backup" "$loader"
    rm -f -- "$backup"
}
trap cleanup EXIT

# The mutation must not break module import (the artifact-correctness check runs before the
# decodeAudioData scan and would fail for an unrelated reason if the call site were reachable at
# import time), so it is planted inside a never-called exported function.
printf '\nexport function __miso_flac_decoder_mutation_probe(x) { return x.decodeAudioData(x); }\n' \
    >>"$loader"

if bash "$policy_script" >/dev/null 2>&1; then
    printf 'flac decoder gate mutation unexpectedly passed: decodeAudioData in sidecars/ escaped\n' >&2
    exit 1
fi

cp -- "$backup" "$loader"

if ! bash "$policy_script" >/dev/null 2>&1; then
    printf 'flac decoder gate fails on the unmutated tree\n' >&2
    exit 1
fi

printf 'flac decoder gate mutation tests: ok\n'
