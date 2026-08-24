#!/usr/bin/env bash
# Carry-forward validation/promotion only. It intentionally contains no benchmark workload launch.
#
# #104 phase A / #83 wave-4 decision W4-D2. This script used to pin `expected_bytes=10364` and the
# sha256 of one historical raw output, so it could only ever promote the single Issue-006 run that
# produced those bytes and was permanently unusable afterwards. Those two pins are retired (they
# are recorded in `.github/ISSUE_SPECS/006-*.md`); what promotes an artifact now is the property
# that actually matters and holds for every run: six records, LF terminated, accepted by the
# aggregate validator, and byte-identical to the raw file it came from, published no-clobber.
set -euo pipefail

[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
validator="$script_directory/graph-benchmark-validator.jq"
artifact_directory="$repository_root/target/issue6"
raw_output="$artifact_directory/graph-compiler-benchmark.raw.jsonl"
accepted_output="$artifact_directory/graph-compiler-benchmark.jsonl"

report_identity() {
    if [[ -f "$raw_output" && ! -L "$raw_output" ]]; then
        printf 'raw artifact: bytes=%s sha256=%s path=%s\n' \
            "$(wc -c <"$raw_output" | tr -d ' ')" \
            "$(sha256sum "$raw_output" | awk '{print $1}')" "$raw_output" >&2
    fi
}

fail() {
    printf 'issue-006 graph benchmark promotion rejected: %s\n' "$1" >&2
    report_identity
    exit 1
}

command -v jq >/dev/null || fail 'jq is required'
[[ -f "$validator" ]] || fail 'validator is missing'
[[ -f "$raw_output" && ! -L "$raw_output" ]] || fail 'raw source is missing or not regular'
[[ ! -e "$accepted_output" && ! -L "$accepted_output" ]] || fail 'accepted destination already exists'
[[ "$(awk 'END { print NR }' "$raw_output")" == 6 ]] || fail 'raw record count differs'
[[ "$(tail -c 1 "$raw_output" | od -An -t x1 | tr -d '[:space:]')" == 0a ]] || fail 'raw is not LF terminated'
jq -s -e -L "$script_directory" -f "$validator" "$raw_output" >/dev/null || fail 'aggregate validator rejected raw'

temporary="$(mktemp "$artifact_directory/.graph-compiler-benchmark-promotion.XXXXXX")"
cleanup() { rm -f -- "$temporary"; }
trap cleanup EXIT INT TERM
cp -- "$raw_output" "$temporary"
[[ "$(awk 'END { print NR }' "$temporary")" == 6 ]] || fail 'temporary record count differs'
[[ "$(tail -c 1 "$temporary" | od -An -t x1 | tr -d '[:space:]')" == 0a ]] || fail 'temporary is not LF terminated'
jq -s -e -L "$script_directory" -f "$validator" "$temporary" >/dev/null || fail 'aggregate validator rejected temporary'
cmp -s -- "$raw_output" "$temporary" || fail 'temporary differs from raw'
[[ ! -e "$accepted_output" && ! -L "$accepted_output" ]] || fail 'accepted destination appeared'
mv -n -- "$temporary" "$accepted_output"
[[ ! -e "$temporary" && -f "$accepted_output" && ! -L "$accepted_output" ]] || fail 'no-clobber publication failed'
cmp -s -- "$raw_output" "$accepted_output" || fail 'accepted bytes differ from raw'
trap - EXIT INT TERM
printf '%s\n' "$accepted_output"
