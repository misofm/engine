#!/usr/bin/env bash
# Enforce master plan #83 decision D6: production code calls the engine's own deterministic math,
# never the platform's.
#
# `f32`/`f64` transcendental methods are not specified to agree between targets, toolchains or
# optimisation levels, so every one of them on a render path is a hole in the cross-target
# bit-identity claim (D5). `crates/miso-engine-math` owns the vendored implementations; this guard
# keeps everything else pointed at it. `sqrt` stays legal: IEEE 754 specifies it exactly.
#
# The allowlist below is the pre-migration state, dated and owned. It shrinks to empty during wave
# 2, when each effect crate moves to `miso-engine-math`; it must never grow. Counts are a ratchet:
# exceeding one fails, and an entry that reaches zero must be deleted in the same change.
set -euo pipefail

# Files that still call the platform libm, with the maximum number of call sites permitted and the
# issue that removes them. Enumerated 2026-08-23 by running this script's own pattern.
#
# Issue #91 removed two lines: `miso-engine-soft-clip` moved to `miso_engine_math::db_to_gain_f32`
# and its tests out of `src/`, and `miso-engine-core/src/arch/mod.rs`'s single site was inside the
# soft-clip kernel test that #91 deleted (the kernel's last consumer had moved).
#
#   path                                                  max  owner
math_policy_allowlist() {
    cat <<'ALLOWLIST'
crates/miso-engine-builtins/src/lib.rs                     38  85
crates/miso-engine-parametric-eq/src/lib.rs                22  87
crates/miso-engine-compressor/src/lib.rs                    4  88
crates/miso-engine-gate-expander/src/lib.rs                 6  89
crates/miso-engine-true-peak-limiter/src/lib.rs             3  90
crates/miso-engine-effect-contract/src/lib.rs               4  95
crates/miso-engine-graph/src/lib.rs                         2  98
crates/miso-engine-graph-compiler/src/lib.rs                1  99
crates/miso-engine-conformance/src/compare.rs               1 105
ALLOWLIST
}

if [[ "${1:-}" == "--print-allowlist" ]]; then
    math_policy_allowlist
    exit 0
fi

workspace_root="${1:-.}"
cd "$workspace_root"

fail() {
    printf 'math policy failure: %s\n' "$1" >&2
    exit 1
}

# Method-call form only: `x.exp()`, `x.powf(y)`, ... A free function named `exp` is the engine's
# own, and `miso_engine_math::exp(x)` must keep working.
pattern='\.(exp|exp2|ln|log2|log10|powf|powi|sin|cos|tan|atan|atan2|sinh|cosh|tanh|exp_m1|ln_1p)\('

# Scanned by design; `tools/` is not (fixture generators use the platform libm deliberately, and
# #104 decides their fate).
roots=()
for candidate in crates hosts; do
    [[ -d "$candidate" ]] && roots+=("$candidate")
done
[[ "${#roots[@]}" -gt 0 ]] || fail "neither crates/ nor hosts/ exists at $workspace_root"

# Structural exemptions, as opposed to migration debt:
#   miso-engine-math          owns the implementations
#   miso-engine-dsp-reference is the independent f64 oracle; using the platform libm is the point
#   tests/ examples/ src/bin/ are not production code and legitimately compare against the platform
structural_exempt='^crates/miso-engine-math/|^crates/miso-engine-dsp-reference/|/tests/|/examples/|/src/bin/'

hits="$({ rg -n "$pattern" "${roots[@]}" --glob '*.rs' || true; } | rg -v "$structural_exempt" || true)"

allowlisted_files="$(math_policy_allowlist | awk '{ print $1 }')"

# 1. Any hit in a file that is not on the allowlist is a violation.
unexpected=""
while IFS= read -r line; do
    [[ -n "$line" ]] || continue
    file="${line%%:*}"
    if ! printf '%s\n' "$allowlisted_files" | grep -qxF "$file"; then
        unexpected+="$line"$'\n'
    fi
done <<<"$hits"

if [[ -n "$unexpected" ]]; then
    printf '%s' "$unexpected" >&2
    fail "platform transcendental calls outside crates/miso-engine-math; call miso_engine_math instead (D6)"
fi

# 2. Allowlisted files may not gain call sites, and an entry that reaches zero must be removed.
while read -r file limit owner; do
    [[ -n "$file" ]] || continue
    if [[ ! -f "$file" ]]; then
        fail "allowlist entry $file no longer exists; delete the line (issue #$owner)"
    fi
    count="$({ rg -c "$pattern" "$file" || true; })"
    count="${count:-0}"
    if [[ "$count" -gt "$limit" ]]; then
        fail "$file has $count platform transcendental calls, allowlist pins at most $limit (issue #$owner)"
    fi
    if [[ "$count" -eq 0 ]]; then
        fail "$file no longer calls the platform libm; delete its allowlist line (issue #$owner is done)"
    fi
done < <(math_policy_allowlist)

remaining="$(math_policy_allowlist | awk '{ total += $2 } END { print total + 0 }')"
printf 'math policy: ok (%s allowlisted call sites remain, target 0 after wave 2)\n' "$remaining"
