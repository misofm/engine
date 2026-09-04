#!/usr/bin/env bash
# Exact Issue 081 five-row native/compile/object target matrix. Do not use as a smoke test.
#
# The cargo/wasm-objdump matrix itself, and the qualification-policy call, now live in
# scripts/check-cross-targets.sh (one cached target dir per target triple, deduplicated against
# scripts/check-parametric-eq-targets.sh and scripts/check-builtins-targets.sh). This file keeps:
#   * `validate_wasm_exports`, sourced directly by
#     scripts/test-effect-interchange-target-export-parser.sh's synthetic regression -- it is the
#     one live implementation, not a copy;
#   * the tool/target preconditions and the literal target-triple and Wasm-feature-flag strings
#     scripts/check-effect-interchange-qualification.sh polices by grepping this file's source;
# and delegates the rest to scripts/check-cross-targets.sh, which runs the qualification check
# exactly once, at its own start (this file used to run it twice: once here, once at its own end).
set -euo pipefail

validate_wasm_exports() {
    local metadata=$1 context=$2 exports
    exports="$(awk '
        /^Export\[/ { in_exports = 1; next }
        in_exports && /^[A-Z][A-Za-z]+\[[0-9]+\]:/ { in_exports = 0 }
        in_exports && /^[[:space:]]*-[[:space:]]+func\[[0-9]+\].*-> "/ {
            name = $0
            sub(/^.*-> "/, "", name)
            sub(/".*$/, "", name)
            if (name ~ /^miso_engine_/) print name
        }
    ' "$metadata")"
    # Issue #143 added exactly one additive export, the observation projection; the frozen
    # `..._inspect` signature and its record layouts are untouched.
    local expected
    expected=$'miso_engine_effect_descriptor_v1_inspect\nmiso_engine_effect_descriptor_v1_inspect_observations'
    [[ "$(printf '%s\n' "$exports" | LC_ALL=C sort)" == "$expected" ]] || {
        printf 'effect interchange target matrix: unexpected Wasm export in %s\n%s\n' \
            "$context" "$exports" >&2
        return 1
    }
}

if [[ "${BASH_SOURCE[0]}" != "$0" ]]; then
    return 0
fi

[[ $# -eq 0 ]] || { printf 'usage: check-effect-interchange-targets.sh\n' >&2; exit 2; }
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
for tool in awk cargo rustc rustup wasm-objdump rg uname; do
    command -v "$tool" >/dev/null 2>&1 || {
        printf 'effect interchange target matrix: missing tool %s\n' "$tool" >&2
        exit 1
    }
done
[[ "$(uname -s)" == Linux ]] || {
    printf 'effect interchange target matrix: native row requires Linux\n' >&2
    exit 1
}
[[ "$(rustc -vV | sed -n 's/^host: //p')" == x86_64-unknown-linux-gnu ]] || {
    printf 'effect interchange target matrix: native row requires x86_64 Linux host\n' >&2
    exit 1
}
for target in x86_64-unknown-linux-gnu aarch64-linux-android aarch64-apple-ios wasm32-unknown-unknown; do
    rustup target list --installed | rg -qx "$target" || {
        printf 'effect interchange target matrix: required target unavailable: %s\n' "$target" >&2
        exit 1
    }
done

# scripts/check-effect-interchange-qualification.sh polices these two Wasm feature spellings by
# grepping this file's source; scripts/check-cross-targets.sh applies them to the actual per-mode
# android/ios/wasm builds now.
for mode in scalar simd; do
    if [[ "$mode" == scalar ]]; then
        feature=-simd128
    else
        feature=+simd128
    fi
done

bash "$root/scripts/check-cross-targets.sh"
printf 'effect interchange five-target matrix: ok\n'
