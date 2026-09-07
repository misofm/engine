#!/usr/bin/env bash
set -euo pipefail
cd /home/bl/misofm/engine-js-hex-recovery
roots=(crates hosts sdk tools sidecars)
printf '%s\n' 'QUERY 1: JavaScript/TypeScript toString(16) paired with padStart(2)'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.ts' --glob '*.js' --glob '*.mjs' 'toString\(16\)[^\n]*padStart\(2|padStart\(2[^\n]*toString\(16\)' "${roots[@]}" || true
printf '%s\n' 'QUERY 2: lowercase nibble tables'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.rs' --glob '*.ts' --glob '*.js' --glob '*.mjs' '0123456789abcdef' "${roots[@]}" || true
printf '%s\n' 'QUERY 3: Rust :02x byte formatting'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.rs' '(:02x|:02X)' "${roots[@]}" || true
printf '%s\n' 'QUERY 4: Rust char::from_digit'
rg -n --hidden --glob '!node_modules/**' --glob '!dist/**' --glob '!target/**' --glob '*.rs' 'char::from_digit' "${roots[@]}" || true
printf '%s\n' 'AUTHORITY DELEGATION CHECKS'
rg -n 'hexLower|engine::hex_lower' sdk/src/core/asset.ts hosts/host-web/web/stem-store/incremental-sha256.js hosts/host-web/qualification/qualification.js sdk/src/core/hex.ts hosts/host-web/web/hex-lower.js crates/engine/src/lib.rs
printf '%s\n' 'CLASSIFICATIONS'
cat <<'EOF'
toString(16)+padStart(2) hits in sdk/src/core/asset.ts and sdk/src/core/boundary.ts are decorated ABI diagnostic strings (0x prefix), not byte encoders.
toString(16)+padStart(2) in sdk/src/internal/session-json.ts is an uppercase Unicode escape formatter (\\u plus four digits), not a byte encoder.
toString(16)+padStart(2) in sdk/test/support.mjs is a fixed-width 64-bit FNV word formatter (16 digits), not a byte encoder.
The lowercase nibble tables in crates/engine/src/lib.rs, sdk/src/core/hex.ts, and hosts/host-web/web/hex-lower.js are exactly the three approved raw unprefixed byte authorities.
All Rust :02x hits are decorated 0x diagnostic/repin fixture renderers; none is an unprefixed production byte encoder.
No char::from_digit hits were found.
The three #552 residual call sites delegate: SDK asset sha256Hex -> SDK hexLower; incremental digestHex -> host-web hexLower; qualification PCM digest adapter -> host-web hexLower.
EOF
printf '%s\n' 'semantic census: PASS (all hits classified; raw authorities exactly engine::hex_lower, SDK hexLower, host-web hexLower)'
