#!/usr/bin/env bash
# Sole one-shot Issue 081 independent-reference process matrix entrypoint.
set -euo pipefail

[[ $# -eq 0 ]] || { printf 'usage: run-effect-interchange-reference-processes.sh\n' >&2; exit 2; }
script_directory=${0%/*}
[[ "$script_directory" != "$0" ]] || script_directory=.
root="$(cd "$script_directory/.." && pwd)"
manifest="$root/fixtures/effect-interchange/v1/ACCEPTED.sha256"
output_dir="$root/target/issue081/reference-processes"
raw="$output_dir/raw.jsonl"
status="$output_dir/status.tsv"
accepted="$output_dir/accepted.jsonl"

for tool in python3 sha256sum mktemp sed wc ln seq tr cat cp rm mkdir; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'effect interchange reference runner: missing tool %s\n' "$tool" >&2
        exit 1
    }
done
[[ -f "$manifest" && ! -L "$manifest" ]] || {
    printf 'effect interchange reference runner: missing baseline manifest\n' >&2
    exit 1
}
mkdir -p "$output_dir"
[[ ! -L "$output_dir" ]] || {
    printf 'effect interchange reference runner: output directory is a symlink\n' >&2
    exit 1
}
for path in "$raw" "$status" "$accepted"; do
    [[ ! -e "$path" && ! -L "$path" ]] || {
        printf 'effect interchange reference runner: refusing existing output %s\n' "$path" >&2
        exit 1
    }
done

(
    cd "$root"
    sha256sum --check --strict "$manifest" >/dev/null
) || { printf 'effect interchange reference runner: baseline mismatch before launch\n' >&2; exit 1; }

scratch="$(mktemp -d "$output_dir/.run.XXXXXX")"
trap 'rm -rf -- "$scratch"' EXIT
( set -o noclobber; : >"$raw"; : >"$status" ) || {
    printf 'effect interchange reference runner: output creation race\n' >&2
    exit 1
}

child_failure=0
for index in $(seq 0 99); do
    child="$scratch/$index.out"
    set +e
    PYTHONDONTWRITEBYTECODE=1 python3 -I -B \
        "$root/scripts/effect-interchange-v1-reference.py" --process-index "$index" \
        >"$child"
    child_status=$?
    set -e
    cat "$child" >>"$raw"
    printf '%s\t%s\t%s\n' "$index" "$child_status" "$(wc -l <"$child" | tr -d ' ')" \
        >>"$status"
    if [[ $child_status -ne 0 ]]; then
        child_failure=1
    fi
done

(
    cd "$root"
    sha256sum --check --strict "$manifest" >/dev/null
) || { printf 'effect interchange reference runner: baseline mismatch after launch\n' >&2; exit 1; }
[[ $child_failure -eq 0 ]] || {
    printf 'effect interchange reference runner: one or more children failed\n' >&2
    exit 1
}
[[ "$(wc -l <"$raw" | tr -d ' ')" == 100 ]] || {
    printf 'effect interchange reference runner: expected exactly 100 records\n' >&2
    exit 1
}

first_hashes=''
for index in $(seq 0 99); do
    line="$(sed -n "$((index + 1))p" "$raw")"
    fields="$(printf '%s\n' "$line" | sed -n \
        's/^{"combined_sha256":"\([0-9a-f]\{64\}\)","descriptor_manifest_sha256":"\([0-9a-f]\{64\}\)","issue":81,"package_manifest_sha256":"\([0-9a-f]\{64\}\)","process_index":\([0-9]\{1,2\}\),"schema_version":1,"state_manifest_sha256":"\([0-9a-f]\{64\}\)"}$/\1 \2 \3 \4 \5/p')"
    [[ -n "$fields" ]] || {
        printf 'effect interchange reference runner: malformed record %s\n' "$index" >&2
        exit 1
    }
    read -r combined descriptor package actual_index state <<<"$fields"
    [[ "$actual_index" == "$index" ]] || {
        printf 'effect interchange reference runner: wrong or duplicate process index %s\n' "$index" >&2
        exit 1
    }
    hashes="$combined $descriptor $package $state"
    if [[ -z "$first_hashes" ]]; then
        first_hashes="$hashes"
    elif [[ "$hashes" != "$first_hashes" ]]; then
        printf 'effect interchange reference runner: child manifest hashes differ\n' >&2
        exit 1
    fi
done

publication="$scratch/accepted.jsonl"
cp "$raw" "$publication"
ln "$publication" "$accepted" || {
    printf 'effect interchange reference runner: accepted publication race\n' >&2
    exit 1
}
rm "$publication"
printf 'effect interchange reference processes: ok children=100\n'
