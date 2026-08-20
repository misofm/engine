#!/usr/bin/env bash
set -euo pipefail
cd "${1:-.}"
root="fixtures/effects/runtime-v1"
manifest="$root/MANIFEST.tsv"
[[ -f "$manifest" ]] || { printf 'missing runtime fixture manifest\n' >&2; exit 1; }
[[ "$(head -n 1 "$manifest")" == $'path\tlength\tsha256' ]] || { printf 'invalid runtime fixture manifest header\n' >&2; exit 1; }
previous=""
listed="$(mktemp)"
actual="$(mktemp)"
trap 'rm -f -- "$listed" "$actual"' EXIT
while IFS=$'\t' read -r path length hash; do
    [[ "$path" == path ]] && continue
    [[ "$path" > "$previous" ]] || { printf 'runtime fixture manifest is not strictly sorted\n' >&2; exit 1; }
    previous="$path"
    file="$root/$path"
    [[ -f "$file" ]] || { printf 'missing runtime fixture: %s\n' "$path" >&2; exit 1; }
    [[ "$(wc -c <"$file" | tr -d ' ')" == "$length" ]] || { printf 'runtime fixture length mismatch: %s\n' "$path" >&2; exit 1; }
    [[ "$hash" =~ ^[0-9a-f]{64}$ ]] || { printf 'runtime fixture hash format: %s\n' "$path" >&2; exit 1; }
    [[ "$(sha256sum "$file" | awk '{print $1}')" == "$hash" ]] || { printf 'runtime fixture hash mismatch: %s\n' "$path" >&2; exit 1; }
    printf '%s\n' "$path" >>"$listed"
done <"$manifest"
find "$root" -type f ! -name MANIFEST.tsv -printf '%P\n' | sort >"$actual"
cmp -s "$listed" "$actual" || { printf 'runtime fixture missing/unlisted file\n' >&2; exit 1; }
printf 'effect runtime fixtures: ok (%s files)\n' "$(wc -l <"$actual" | tr -d ' ')"
