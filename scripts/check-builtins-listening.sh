#!/usr/bin/env bash
set -euo pipefail

cd "${1:-.}"
records=(
  dsp-research/listening/issue007-filter-abx-preregistration.md
  dsp-research/listening/issue007-matrix-ramp-preregistration.md
)
for record in "${records[@]}"; do
  [[ -f "$record" ]] || { printf 'missing issue-007 listening preregistration: %s\n' "$record" >&2; exit 1; }
  rg -Fqx -- '- Evidence kind: real listening' "$record"
  rg -Fqx -- '- Status: preregistered' "$record"
  rg -Fqx -- '- Sound-quality claim: none' "$record"
  rg -Fq 'No human trial has been run.' "$record"
  ! rg -Fq '| 1 |' "$record" || { printf 'preregistration contains fabricated trial row: %s\n' "$record" >&2; exit 1; }
done
printf 'issue-007 listening preregistrations: ok (human evidence pending)\n'
