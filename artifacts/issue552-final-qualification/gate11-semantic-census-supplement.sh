#!/usr/bin/env bash
set -euo pipefail
cd /home/bl/misofm/engine-js-hex-recovery
roots=(crates hosts sdk tools sidecars)
printf '%s\n' 'SUPPLEMENT QUERY A: all toString(16) hits'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.ts' --glob '*.js' --glob '*.mjs' 'toString\(16\)' "${roots[@]}" || true
printf '%s\n' 'SUPPLEMENT QUERY B: all padStart calls on toString(16) lines'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.ts' --glob '*.js' --glob '*.mjs' 'toString\(16\)[^\n]*padStart\(' "${roots[@]}" || true
printf '%s\n' 'SUPPLEMENT QUERY C: lowercase nibble table candidates'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.rs' --glob '*.ts' --glob '*.js' --glob '*.mjs' '0123456789abcdef' "${roots[@]}" || true
printf '%s\n' 'SUPPLEMENT QUERY D: Rust byte formatting and char::from_digit'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.rs' '(:02x|:02X|char::from_digit)' "${roots[@]}" || true
printf '%s\n' 'SUPPLEMENT CLASSIFICATIONS'
printf '%s\n' 'asset.ts ABI toString(16).padStart(8) and boundary.ts ABI toString(16) are decorated diagnostics.'
printf '%s\n' 'session-json.ts uses uppercase \\u escapes; sdk/test/support.mjs formats a fixed-width 64-bit word.'
printf '%s\n' 'The three nibble tables are engine::hex_lower, SDK hexLower, and host-web hexLower; test literals are fixed expected values.'
printf '%s\n' 'Every Rust :02x/:02X hit has a 0x decoration and is a diagnostic or repin renderer; char::from_digit has no hits.'
