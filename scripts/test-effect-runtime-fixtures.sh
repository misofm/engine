#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-.}" && pwd)"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
mkdir -p "$temp/fixtures/effects"
cp -R "$root/fixtures/effects/runtime-v1" "$temp/fixtures/effects/"
printf 'mutation\n' >>"$temp/fixtures/effects/runtime-v1/valid/automation.txt"
if bash "$root/scripts/check-effect-runtime-fixtures.sh" "$temp" >/dev/null 2>&1; then
    printf 'runtime fixture mutation escaped\n' >&2; exit 1
fi
printf 'effect runtime fixture mutations: ok\n'
