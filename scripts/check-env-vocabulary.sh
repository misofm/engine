#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"; cd "$root"
fail() { printf 'env vocabulary failure: %s\n' "$1" >&2; exit 1; }
vocabulary=docs/ENGINE_ENV_VOCABULARY.md
[[ -f "$vocabulary" && ! -L "$vocabulary" ]] || fail "missing vocabulary: $vocabulary"
tmp="$(mktemp -d)"; trap 'rm -rf -- "$tmp"' EXIT
paths0="$tmp/paths0"; paths="$tmp/paths"; filtered="$tmp/filtered"
if probe="$(git rev-parse --is-inside-work-tree 2>/dev/null)"; then
    [[ "$probe" == true ]] || fail 'Git classification returned unexpected output'
    git ls-files -z --cached --others --exclude-standard >"$paths0" || fail 'Git file listing failed'
else
    rc=$?; [[ "$rc" == 128 ]] || fail "Git classification failed (status $rc)"
    find . -type f -not -path './.git/*' -not -path './target/*' -print0 >"$paths0" || fail 'fallback file traversal failed'
fi
tr '\0' '\n' <"$paths0" | sed 's|^\./||' >"$paths" || fail 'path conversion failed'
if grep -v -x -F -e "$vocabulary" "$paths" | grep -v '^\.github/ISSUE_SPECS/' >"$filtered"; then :; else
    rc=$?; [[ "$rc" == 1 ]] || fail 'path exclusion failed'; : >"$filtered"
fi
stray="$tmp/stray"; : >"$stray"
while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    output=''; if output="$(grep -hoE 'MISO_[A-Z0-9_]+' -- "$path" 2>&1)"; then rc=0; else rc=$?; fi
    case "$rc" in
        0) printf '%s\n' "$output" >>"$stray";;
        1) ;;
        *) printf '%s\n' "$output" >&2; fail "source scan failed for $path (grep status $rc)";;
    esac
done <"$filtered"
stray_names="$(sort -u "$stray" | grep -v '^MISO_ENGINE_' || true)"
if [[ -n "$stray_names" ]]; then printf '%s\n' "$stray_names" >&2; fail 'identifier outside the MISO_ENGINE_ prefix'; fi
used_raw="$tmp/used-raw"; used="$tmp/used"; documented="$tmp/documented"
if grep -rhoE 'MISO_ENGINE_[A-Z0-9_]+' tools scripts >"$used_raw"; then :; else rc=$?; [[ "$rc" == 1 ]] || fail "tools/scripts source scan failed (grep status $rc)"; : >"$used_raw"; fi
if grep -v '_$' "$used_raw" | sort -u >"$used"; then :; else rc=$?; [[ "$rc" == 1 ]] || fail 'used-name filtering failed'; : >"$used"; fi
if grep -oE '^\| `MISO_ENGINE_[A-Z0-9_]+' "$vocabulary" | tr -d '|` ' | sort -u >"$documented"; then :; else rc=$?; [[ "$rc" == 1 ]] || fail 'vocabulary scan failed'; : >"$documented"; fi
undocumented="$(comm -23 "$used" "$documented")"; if [[ -n "$undocumented" ]]; then printf '%s\n' "$undocumented" >&2; fail "name used under tools/ or scripts/ but absent from $vocabulary"; fi
unused="$(comm -13 "$used" "$documented")"; if [[ -n "$unused" ]]; then printf '%s\n' "$unused" >&2; fail "name documented in $vocabulary but unused under tools/ or scripts/"; fi
printf 'env vocabulary: ok (%s names, one MISO_ENGINE_ prefix)\n' "$(wc -l <"$documented" | tr -d ' ')"
