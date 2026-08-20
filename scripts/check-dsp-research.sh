#!/usr/bin/env bash
# Structural check for the bounded, citation-backed issue-002 corpus.
set -euo pipefail
cd "${1:-.}"

required=(filters dynamics loudness oversampling true-peak delay nonlinear-antialiasing multirate-crossovers simd-numerics console-daw-architecture)
headings=("Scope and engineering question" "Adopted V2 decisions" "Definitions and assumptions" "Algorithm and equations" "Coefficients and update rules" "Numerical and stability limits" "Latency and tail" "Units, mappings, automation and smoothing" "Denormal, signed-zero and NaN policy" "Fixtures" "Objective tests and tolerances" "Benchmark plan" "Listening protocol or evidence" "Primary and official sources" "Rejected alternatives and tradeoffs" "Known gaps and follow-up")
for name in "${required[@]}"; do
    file="dsp-research/$name.md"; [[ -s "$file" ]] || { printf 'missing research note: %s\n' "$file" >&2; exit 1; }
    for heading in "${headings[@]}"; do
        rg -q "^## $heading$" "$file" || { printf 'missing heading %s in %s\n' "$heading" "$file" >&2; exit 1; }
        awk -v heading="## $heading" '
            $0 == heading { found = 1; next }
            found && /^## / { exit }
            found && NF { content = 1; exit }
            END { exit !content }
        ' "$file" || { printf 'empty heading %s in %s\n' "$heading" "$file" >&2; exit 1; }
    done
    source_count="$(awk '
        /^## Primary and official sources$/ { found = 1; next }
        found && /^## / { exit }
        found { print }
    ' "$file" | rg -o '\[[A-Z0-9][A-Z0-9-]+\]' | sort -u | wc -l | tr -d ' ')"
    [[ "$source_count" -ge 2 ]] || { printf 'fewer than two sources in %s\n' "$file" >&2; exit 1; }
    while IFS= read -r key; do
        rg -Fq -- "- \`[$key]\`" dsp-research/BIBLIOGRAPHY.md || {
            printf 'unresolved bibliography key %s in %s\n' "$key" "$file" >&2
            exit 1
        }
    done < <(rg -o '\[[A-Z0-9][A-Z0-9-]+\]' "$file" | tr -d '[]' | sort -u)
done
for file in dsp-research/{README,CITATION_POLICY,BIBLIOGRAPHY,NOTE_TEMPLATE}.md dsp-research/listening/{TEMPLATE,FORMAT_EXAMPLE}.md; do [[ -s "$file" ]] || { printf 'missing research artifact: %s\n' "$file" >&2; exit 1; }; done
rg -q 'DiGiCo' dsp-research/console-daw-architecture.md && rg -q 'SSL' dsp-research/console-daw-architecture.md && rg -q 'Lawo' dsp-research/console-daw-architecture.md && rg -q 'Avid' dsp-research/console-daw-architecture.md && rg -q 'Logic' dsp-research/console-daw-architecture.md
for heading in "${headings[@]}"; do
    rg -q "^## $heading$" dsp-research/NOTE_TEMPLATE.md || {
        printf 'note template missing required heading: %s\n' "$heading" >&2
        exit 1
    }
done
rg -q 'Evidence kind: synthetic format example' dsp-research/listening/FORMAT_EXAMPLE.md
rg -q 'Sound-quality claim: none' dsp-research/listening/FORMAT_EXAMPLE.md
rg -q 'SYNTHETIC-NOT-A-HUMAN' dsp-research/listening/FORMAT_EXAMPLE.md
printf 'dsp research corpus: ok\n'
