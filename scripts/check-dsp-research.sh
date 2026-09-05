#!/usr/bin/env bash
# Structural check for the bounded, citation-backed issue-002 corpus.
set -euo pipefail
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$(cd "${1:-.}" && pwd)"
cd "$root"
source "$script_dir/lib/gate.sh"
GATE_FAILURE_PREFIX='dsp research corpus failure'
fail() { printf 'dsp research corpus failure: %s\n' "$1" >&2; exit 1; }
literal_required() {
    local description="$1" pattern="$2" file="$3" output rc
    if output="$(rg -n -F -- "$pattern" "$file" 2>&1)"; then rc=0; else rc=$?; fi
    [[ "$rc" == 0 ]] || { printf '%s\n' "$output" >&2; fail "$description search failed (rg exit $rc)"; }
}

required=(filters dynamics loudness oversampling true-peak delay nonlinear-antialiasing multirate-crossovers simd-numerics console-daw-architecture)
headings=("Scope and engineering question" "Adopted decisions" "Definitions and assumptions" "Algorithm and equations" "Coefficients and update rules" "Numerical and stability limits" "Latency and tail" "Units, mappings, automation and smoothing" "Denormal, signed-zero and NaN policy" "Fixtures" "Objective tests and tolerances" "Benchmark plan" "Listening protocol or evidence" "Primary and official sources" "Rejected alternatives and tradeoffs" "Known gaps and follow-up")
for name in "${required[@]}"; do
    file="dsp-research/$name.md"; [[ -s "$file" ]] || { printf 'missing research note: %s\n' "$file" >&2; exit 1; }
    for heading in "${headings[@]}"; do
        gate_scan_required "heading $heading in $file" "^## $heading$" '' "$file" >/dev/null || exit $?
        if ! awk -v heading="## $heading" '
            $0 == heading { found = 1; next }
            found && /^## / { exit }
            found && NF { content = 1; exit }
            END { exit !content }
        ' "$file"; then fail "empty heading $heading in $file"; fi
    done
    primary="$(awk '
        /^## Primary and official sources$/ { found = 1; next }
        found && /^## / { exit }
        found { print }
    ' "$file")" || fail "Primary section extraction failed in $file"
    if primary_keys="$(printf '%s\n' "$primary" | rg -o '\[[A-Z0-9][A-Z0-9-]+\]' | tr -d '[]' | sort -u)"; then :; else
        rc=$?; [[ "$rc" == 1 ]] || fail "Primary key extraction failed in $file"; primary_keys=''
    fi
    source_count="$(printf '%s\n' "$primary_keys" | awk 'NF { n++ } END { print n + 0 }')"
    [[ "$source_count" -ge 2 ]] || { printf 'fewer than two sources in %s\n' "$file" >&2; exit 1; }
    if all_keys="$(rg -o '\[[A-Z0-9][A-Z0-9-]+\]' "$file" | tr -d '[]' | sort -u)"; then :; else
        rc=$?; [[ "$rc" == 1 ]] || fail "whole-note key extraction failed in $file"; all_keys=''
    fi
    while IFS= read -r key; do
        [[ -n "$key" ]] || continue
        literal_required "bibliography key $key" "- \`[$key]\`" dsp-research/BIBLIOGRAPHY.md || {
            printf 'unresolved bibliography key %s in %s\n' "$key" "$file" >&2
            exit 1
        }
    done <<<"$all_keys"
done
for file in dsp-research/{README,CITATION_POLICY,BIBLIOGRAPHY,NOTE_TEMPLATE}.md dsp-research/listening/{TEMPLATE,FORMAT_EXAMPLE}.md; do [[ -s "$file" ]] || { printf 'missing research artifact: %s\n' "$file" >&2; exit 1; }; done
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
