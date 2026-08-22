#!/usr/bin/env bash
set -euo pipefail

root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
mkdir -p "$temp/fixtures/builtins"
cp -R "$root/fixtures/builtins/v1" "$temp/fixtures/builtins/"
for mutation in content manifest unlisted missing coverage; do
    copy="$temp/$mutation-root"
    mkdir -p "$copy"
    cp -R "$temp/fixtures" "$copy/fixtures"
    case "$mutation" in
        content) printf 'mutation\n' >>"$copy/fixtures/builtins/v1/cases.toml" ;;
        manifest) printf 'broken\n' >>"$copy/fixtures/builtins/v1/MANIFEST.tsv" ;;
        unlisted) printf 'unlisted\n' >"$copy/fixtures/builtins/v1/unlisted.txt" ;;
        missing) rm "$copy/fixtures/builtins/v1/pcm/identity-signed-zero.f32le" ;;
        coverage) sed -i '/id = "response-cascade-44100-1-fixed-0"/,+8d' "$copy/fixtures/builtins/v1/cases.toml" ;;
    esac
    if bash "$root/scripts/check-builtins-fixtures.sh" "$copy" >/dev/null 2>&1; then
        printf 'builtins fixture corruption escaped: %s\n' "$mutation" >&2
        exit 1
    fi
done
printf 'builtins fixture mutations: ok\n'
