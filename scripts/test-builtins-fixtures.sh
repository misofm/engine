#!/usr/bin/env bash
set -euo pipefail

root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
mkdir -p "$temp/fixtures/builtins"
cp -R "$root/fixtures/builtins/v1" "$temp/fixtures/builtins/"
printf 'mutation\n' >>"$temp/fixtures/builtins/v1/filter-response-cases.csv"
if bash "$root/scripts/check-builtins-fixtures.sh" "$temp" >/dev/null 2>&1; then
    printf 'builtins fixture mutation escaped\n' >&2
    exit 1
fi
printf 'builtins fixture mutations: ok\n'
