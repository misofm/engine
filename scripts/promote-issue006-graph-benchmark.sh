#!/usr/bin/env bash
# Carry-forward validation/promotion only. It intentionally contains no benchmark workload launch.
set -euo pipefail

[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repository_root="$(cd "$script_directory/.." && pwd)"
validator="$script_directory/graph-benchmark-validator.jq"
artifact_directory="$repository_root/target/issue6"
raw_output="$artifact_directory/graph-compiler-benchmark.raw.jsonl"
accepted_output="$artifact_directory/graph-compiler-benchmark.jsonl"
expected_bytes=10364
expected_sha256=c03f1bc0399f0b9dea3a5c94c13a468512d2fcb2a2805c450c83110b56d623b5

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
[[ "$(wc -c <"$raw_output" | tr -d ' ')" == "$expected_bytes" ]] || fail 'raw byte count differs'
[[ "$(sha256sum "$raw_output" | awk '{print $1}')" == "$expected_sha256" ]] || fail 'raw hash differs'
[[ "$(awk 'END { print NR }' "$raw_output")" == 6 ]] || fail 'raw record count differs'
[[ "$(tail -c 1 "$raw_output" | od -An -t x1 | tr -d '[:space:]')" == 0a ]] || fail 'raw is not LF terminated'
jq -s -e -L "$script_directory" -f "$validator" "$raw_output" >/dev/null || fail 'aggregate validator rejected raw'

temporary="$(mktemp "$artifact_directory/.graph-compiler-benchmark-promotion.XXXXXX")"
cleanup() { rm -f -- "$temporary"; }
trap cleanup EXIT INT TERM
cp -- "$raw_output" "$temporary"
[[ "$(wc -c <"$temporary" | tr -d ' ')" == "$expected_bytes" ]] || fail 'temporary byte count differs'
[[ "$(sha256sum "$temporary" | awk '{print $1}')" == "$expected_sha256" ]] || fail 'temporary hash differs'
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
