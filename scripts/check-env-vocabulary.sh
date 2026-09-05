#!/usr/bin/env bash
set -euo pipefail
root="$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)"
cd "$root"
fail() { printf 'env vocabulary failure: %s\n' "$1" >&2; exit 1; }
vocabulary=docs/ENGINE_ENV_VOCABULARY.md
[[ -f "$vocabulary" && ! -L "$vocabulary" ]] || fail "missing vocabulary: $vocabulary"
[[ -d tools ]] || fail 'missing required source root: tools'
[[ -d scripts ]] || fail 'missing required source root: scripts'
tmp="$(mktemp -d)"; trap 'rm -rf -- "$tmp"' EXIT
paths0="$tmp/paths0"; classification="$tmp/git-classification"
if git rev-parse --is-inside-work-tree >"$classification" 2>&1; then
    [[ "$(<"$classification")" == true ]] || fail 'Git classification returned unexpected output'
    git ls-files -z --cached --others --exclude-standard >"$paths0" 2>"$tmp/git-listing-error" || {
        tr '\0' '\n' <"$paths0" >&2 || true
        cat "$tmp/git-listing-error" >&2; fail 'Git file listing failed'
    }
else
    rc=$?; classification_output="$(<"$classification")"
    if [[ "$rc" == 128 && "$classification_output" == *'not a git repository'* ]]; then
        find . -type f -not -path './.git/*' -not -path './target/*' -print0 >"$paths0" 2>"$tmp/find-error" || {
            cat "$tmp/find-error" >&2; fail 'fallback file traversal failed'
        }
    else
        printf '%s\n' "$classification_output" >&2
        fail "Git classification failed (status $rc)"
    fi
fi
paths="$tmp/paths"; normalised="$tmp/normalised"
filtered_once="$tmp/filtered-once"; filtered="$tmp/filtered"
tr '\0' '\n' <"$paths0" >"$paths" || fail 'path NUL conversion failed'
sed 's|^\./||' "$paths" >"$normalised" || fail 'path normalization failed'
if grep -v -x -F -e "$vocabulary" "$normalised" >"$filtered_once"; then :; else
    rc=$?; [[ "$rc" == 1 ]] || fail "vocabulary path exclusion failed (grep status $rc)"
    : >"$filtered_once"
fi
if grep -v '^\.github/ISSUE_SPECS/' "$filtered_once" >"$filtered"; then :; else
    rc=$?; [[ "$rc" == 1 ]] || fail "issue-spec path exclusion failed (grep status $rc)"
    : >"$filtered"
fi
stray="$tmp/stray"; : >"$stray"
while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    output=''; if output="$(grep -hoE 'MISO_[A-Z0-9_]+' -- "$path" 2>&1)"; then rc=0; else rc=$?; fi
    case "$rc" in
        0) printf '%s\n' "$output" >>"$stray" ;;
        1) ;;
        *) printf '%s\n' "$output" >&2; fail "source scan failed for $path (grep status $rc)" ;;
    esac
done <"$filtered"
sort -u "$stray" >"$tmp/stray-sorted" || fail 'stray-name sort failed'
if grep -v '^MISO_ENGINE_' "$tmp/stray-sorted" >"$tmp/stray-names"; then
    cat "$tmp/stray-names" >&2; fail 'identifier outside the MISO_ENGINE_ prefix'
else
    rc=$?; [[ "$rc" == 1 ]] || fail "stray-name prefix filter failed (grep status $rc)"
fi
used_raw="$tmp/used-raw"; used_filtered="$tmp/used-filtered"; used="$tmp/used"
if grep -rhoE 'MISO_ENGINE_[A-Z0-9_]+' tools scripts >"$used_raw" 2>"$tmp/used-error"; then :; else
    rc=$?; cat "$used_raw" >&2; cat "$tmp/used-error" >&2
    [[ "$rc" == 1 ]] || fail "tools/scripts source scan failed (grep status $rc)"
    fail 'no environment names used under tools/ or scripts/'
fi
if grep -v '_$' "$used_raw" >"$used_filtered"; then :; else
    rc=$?; [[ "$rc" == 1 ]] || fail "used-name fragment filter failed (grep status $rc)"
    fail 'no complete environment names used under tools/ or scripts/'
fi
sort -u "$used_filtered" >"$used" || fail 'used-name sort failed'
[[ -s "$used" ]] || fail 'no complete environment names used under tools/ or scripts/'
documented_rows="$tmp/documented-rows"; documented_trimmed="$tmp/documented-trimmed"
documented="$tmp/documented"
if grep -oE '^\| `MISO_ENGINE_[A-Z0-9_]+`' "$vocabulary" >"$documented_rows" 2>"$tmp/vocabulary-error"; then :; else
    rc=$?; cat "$documented_rows" >&2; cat "$tmp/vocabulary-error" >&2
    [[ "$rc" == 1 ]] || fail "vocabulary scan failed (grep status $rc)"
    fail 'no documented environment names'
fi
tr -d '|` ' <"$documented_rows" >"$documented_trimmed" || fail 'vocabulary delimiter removal failed'
sort -u "$documented_trimmed" >"$documented" || fail 'documented-name sort failed'
[[ -s "$documented" ]] || fail 'no documented environment names'
comm -23 "$used" "$documented" >"$tmp/undocumented" || fail 'undocumented-name comparison failed'
if [[ -s "$tmp/undocumented" ]]; then
    cat "$tmp/undocumented" >&2
    fail "name used under tools/ or scripts/ but absent from $vocabulary"
fi
comm -13 "$used" "$documented" >"$tmp/unused" || fail 'unused-name comparison failed'
if [[ -s "$tmp/unused" ]]; then
    cat "$tmp/unused" >&2
    fail "name documented in $vocabulary but unused under tools/ or scripts/"
fi
wc -l <"$documented" >"$tmp/count" || fail 'documented-name count failed'
tr -d ' ' <"$tmp/count" >"$tmp/count-trimmed" || fail 'documented-name count formatting failed'
printf 'env vocabulary: ok (%s names, one MISO_ENGINE_ prefix)\n' "$(<"$tmp/count-trimmed")"
