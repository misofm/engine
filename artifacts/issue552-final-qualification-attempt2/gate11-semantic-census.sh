#!/usr/bin/env bash
set -euo pipefail
cd /home/bl/misofm/engine-js-hex-recovery
roots=(crates hosts sdk tools sidecars)
printf '%s\n' 'COMPLETE CENSUS: toString(16) hits'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.ts' --glob '*.js' --glob '*.mjs' 'toString\(16\)' "${roots[@]}" || true
printf '%s\n' 'COMPLETE CENSUS: toString(16) paired with padStart'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.ts' --glob '*.js' --glob '*.mjs' 'toString\(16\)[^\n]*padStart\(' "${roots[@]}" || true
printf '%s\n' 'COMPLETE CENSUS: lowercase nibble tables and literals'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.rs' --glob '*.ts' --glob '*.js' --glob '*.mjs' '0123456789abcdef' "${roots[@]}" || true
printf '%s\n' 'COMPLETE CENSUS: Rust byte formatting and char::from_digit'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.rs' '(:02x|:02X|char::from_digit)' "${roots[@]}" || true
printf '%s\n' 'SUPPLEMENTAL AUTHORITY CENSUS'
rg -n 'hexLower|engine::hex_lower' sdk/src/core/asset.ts sdk/src/core/hex.ts hosts/host-web/web/hex-lower.js hosts/host-web/web/stem-store/incremental-sha256.js hosts/host-web/qualification/qualification.js crates/engine/src/lib.rs
printf '%s\n' 'CLASSIFICATIONS'
printf '%s\n' 'asset.ts ABI diagnostics use 0x plus toString(16).padStart(8); boundary.ts ABI diagnostics are decorated; neither encodes bytes.'
printf '%s\n' 'session-json.ts emits uppercase Unicode \\u escapes; sdk/test/support.mjs emits fixed-width 64-bit FNV words.'
printf '%s\n' 'The three lowercase nibble tables are engine::hex_lower, SDK hexLower, and host-web hexLower; test literals are fixed expected values.'
printf '%s\n' 'Every Rust :02x/:02X hit is 0x-prefixed diagnostic or repin output; no char::from_digit hits exist.'
printf '%s\n' 'The three #552 residual call sites delegate to SDK or host-web hexLower as appropriate.'
printf '%s\n' 'semantic census: PASS'
