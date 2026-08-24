#!/usr/bin/env bash
set -euo pipefail

# The manifest is byte-sorted (C collation). `[[ a > b ]]` and `sort` both follow the caller's
# collation, so a UTF-8 locale reordered `pcm/matrix-ramp-1.f32le` against
# `pcm/matrix-ramp-127.f32le` and this gate failed on an unchanged tree (#104 phase A).
export LC_ALL=C

cd "${1:-.}"
root="fixtures/builtins/v1"
manifest="$root/MANIFEST.tsv"
workspace_root="$(cd "$(dirname "$0")/.." && pwd)"
[[ -f "$manifest" ]] || { printf 'missing builtins fixture manifest\n' >&2; exit 1; }
[[ "$(head -n 1 "$manifest")" == $'path\tlength\tsha256' ]] || {
    printf 'invalid builtins fixture manifest header\n' >&2; exit 1;
}
previous=""
listed="$(mktemp)"
actual="$(mktemp)"
trap 'rm -f -- "$listed" "$actual"' EXIT
while IFS=$'\t' read -r path length hash; do
    [[ "$path" == path ]] && continue
    [[ "$path" > "$previous" ]] || { printf 'builtins fixture manifest is not strictly sorted\n' >&2; exit 1; }
    previous="$path"
    file="$root/$path"
    [[ -f "$file" ]] || { printf 'missing builtins fixture: %s\n' "$path" >&2; exit 1; }
    [[ "$length" =~ ^[0-9]+$ ]] || { printf 'invalid builtins fixture length: %s\n' "$path" >&2; exit 1; }
    [[ "$(wc -c <"$file" | tr -d ' ')" == "$length" ]] || { printf 'builtins fixture length mismatch: %s\n' "$path" >&2; exit 1; }
    [[ "$hash" =~ ^[0-9a-f]{64}$ ]] || { printf 'invalid builtins fixture hash: %s\n' "$path" >&2; exit 1; }
    [[ "$(sha256sum "$file" | awk '{print $1}')" == "$hash" ]] || { printf 'builtins fixture hash mismatch: %s\n' "$path" >&2; exit 1; }
    printf '%s\n' "$path" >>"$listed"
done <"$manifest"
find "$root" -type f ! -name MANIFEST.tsv -printf '%P\n' | sort >"$actual"
cmp -s "$listed" "$actual" || { printf 'builtins fixture missing/unlisted file\n' >&2; exit 1; }
if [[ -d "$workspace_root/tools/miso-engine-audit" ]]; then
    cargo run --quiet --bin miso_engine_audit \
        --manifest-path "$workspace_root/tools/miso-engine-audit/Cargo.toml" \
        -- fixture-builtins --check "$(pwd)/$root" || {
        printf 'builtins fixture expected-output check failed\n' >&2
        exit 1
    }
fi
printf 'builtins fixtures: ok (%s files)\n' "$(wc -l <"$actual" | tr -d ' ')"
