#!/usr/bin/env bash
# Structural check for the bounded, citation-backed issue-002 corpus.
set -euo pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "${1:-.}" && pwd)"
cd "$root"
source "$script_dir/lib/gate.sh"
GATE_FAILURE_PREFIX='dsp research corpus failure'
fail() { printf 'dsp research corpus failure: %s\n' "$1" >&2; exit 1; }
tmp="$(mktemp -d)"; trap 'rm -rf -- "$tmp"' EXIT

literal_required() {
    local description="$1" pattern="$2" file="$3" output rc
    if output="$(rg -n -F -- "$pattern" "$file" 2>&1)"; then rc=0; else rc=$?; fi
    [[ "$rc" == 0 ]] || {
        printf '%s\n' "$output" >&2
        fail "$description search failed (rg exit $rc)"
    }
}

required=(filters dynamics loudness oversampling true-peak delay nonlinear-antialiasing multirate-crossovers simd-numerics console-daw-architecture)
headings=("Scope and engineering question" "Adopted decisions" "Definitions and assumptions" "Algorithm and equations" "Coefficients and update rules" "Numerical and stability limits" "Latency and tail" "Units, mappings, automation and smoothing" "Denormal, signed-zero and NaN policy" "Fixtures" "Objective tests and tolerances" "Benchmark plan" "Listening protocol or evidence" "Primary and official sources" "Rejected alternatives and tradeoffs" "Known gaps and follow-up")
for name in "${required[@]}"; do
    file="dsp-research/$name.md"
    [[ -s "$file" ]] || { printf 'missing research note: %s\n' "$file" >&2; exit 1; }
    for heading in "${headings[@]}"; do
        gate_scan_required "heading $heading in $file" "^## $heading$" '' "$file" >/dev/null || exit $?
        section_error="$tmp/section-error"
        if awk -v heading="## $heading" '
            $0 == heading { found = 1; next }
            found && /^## / { exit }
            found && NF { content = 1; exit }
            END { exit !content }
        ' "$file" 2>"$section_error"; then
            :
        else
            rc=$?; cat "$section_error" >&2
            if [[ "$rc" == 1 ]]; then
                fail "empty heading $heading in $file"
            fi
            fail "section content check failed in $file (awk status $rc)"
        fi
    done

    primary="$tmp/$name-primary"
    primary_error="$tmp/$name-primary-error"
    awk '
        /^## Primary and official sources$/ { found = 1; next }
        found && /^## / { exit }
        found { print }
    ' "$file" >"$primary" 2>"$primary_error" || {
        rc=$?; cat "$primary" >&2; cat "$primary_error" >&2
        fail "Primary section extraction failed in $file (awk status $rc)"
    }
    primary_brackets="$tmp/$name-primary-brackets"
    if rg -o '\[[A-Z0-9][A-Z0-9-]+\]' "$primary" >"$primary_brackets" 2>"$tmp/$name-primary-rg-error"; then
        :
    else
        rc=$?; cat "$primary_brackets" >&2; cat "$tmp/$name-primary-rg-error" >&2
        [[ "$rc" == 1 ]] || fail "Primary key extraction failed in $file (rg exit $rc)"
        : >"$primary_brackets"
    fi
    if tr -d '[]' <"$primary_brackets" >"$tmp/$name-primary-keys" 2>"$tmp/$name-primary-tr-error"; then :; else
        rc=$?; cat "$tmp/$name-primary-keys" >&2; cat "$tmp/$name-primary-tr-error" >&2
        fail "Primary key delimiter conversion failed in $file (tr status $rc)"
    fi
    if sort -u "$tmp/$name-primary-keys" >"$tmp/$name-primary-unique" 2>"$tmp/$name-primary-sort-error"; then :; else
        rc=$?; cat "$tmp/$name-primary-unique" >&2; cat "$tmp/$name-primary-sort-error" >&2
        fail "Primary key sort failed in $file (sort status $rc)"
    fi
    source_count=0
    while IFS= read -r key; do [[ -n "$key" ]] && source_count=$((source_count + 1)); done <"$tmp/$name-primary-unique"
    [[ "$source_count" -ge 2 ]] || { printf 'fewer than two sources in %s\n' "$file" >&2; exit 1; }

    whole_brackets="$tmp/$name-whole-brackets"
    if rg -o '\[[A-Z0-9][A-Z0-9-]+\]' "$file" >"$whole_brackets" 2>"$tmp/$name-whole-rg-error"; then
        :
    else
        rc=$?; cat "$whole_brackets" >&2; cat "$tmp/$name-whole-rg-error" >&2
        [[ "$rc" == 1 ]] || fail "whole-note key extraction failed in $file (rg exit $rc)"
        : >"$whole_brackets"
    fi
    if tr -d '[]' <"$whole_brackets" >"$tmp/$name-whole-keys" 2>"$tmp/$name-whole-tr-error"; then :; else
        rc=$?; cat "$tmp/$name-whole-keys" >&2; cat "$tmp/$name-whole-tr-error" >&2
        fail "whole-note key delimiter conversion failed in $file (tr status $rc)"
    fi
    if sort -u "$tmp/$name-whole-keys" >"$tmp/$name-whole-unique" 2>"$tmp/$name-whole-sort-error"; then :; else
        rc=$?; cat "$tmp/$name-whole-unique" >&2; cat "$tmp/$name-whole-sort-error" >&2
        fail "whole-note key sort failed in $file (sort status $rc)"
    fi
    while IFS= read -r key; do
        [[ -n "$key" ]] || continue
        literal_required "bibliography key $key" "- \`[$key]\`" dsp-research/BIBLIOGRAPHY.md
    done <"$tmp/$name-whole-unique"
done

for file in dsp-research/{README,CITATION_POLICY,BIBLIOGRAPHY,NOTE_TEMPLATE}.md dsp-research/listening/{TEMPLATE,FORMAT_EXAMPLE}.md; do
    [[ -s "$file" ]] || { printf 'missing research artifact: %s\n' "$file" >&2; exit 1; }
done
for literal in DiGiCo SSL Lawo Avid Logic; do
    gate_scan_required "console/DAW literal $literal" "$literal" '' dsp-research/console-daw-architecture.md >/dev/null || exit $?
done
for heading in "${headings[@]}"; do
    gate_scan_required "note template heading $heading" "^## $heading$" '' dsp-research/NOTE_TEMPLATE.md >/dev/null || exit $?
done
for literal in 'Evidence kind: synthetic format example' 'Sound-quality claim: none' 'SYNTHETIC-NOT-A-HUMAN'; do
    gate_scan_required "listening literal $literal" "$literal" '' dsp-research/listening/FORMAT_EXAMPLE.md >/dev/null || exit $?
done
printf 'dsp research corpus: ok\n'
