#!/usr/bin/env bash
# The unfused multiply-add contract's container (issue #163 phase 2).
#
# Since phase 2 the numeric contract has no fused multiply-add anywhere: `Lane::fma` is
# `(a * b) + c` with two roundings, identically on every backend, and the exact software emulation
# that used to stand behind it on wasm is deleted. `docs/rulings/unfused-multiply-add-audit.md`
# records the nineteen sites and why none of them keeps an exact path.
#
# That makes this seal the inverse shape of `check-fast-db-seal.sh`. The fast dB tier admits
# exactly six named crossings; this one admits *none*. The property being sealed is an **absence**,
# and an absence is the easiest thing in a codebase to lose by accident:
#
#   * `f32::mul_add` is one method call away and reads like an optimisation;
#   * `wide`'s `mul_add` is fused on x86 and NEON and unfused elsewhere, so a single forwarded call
#     would silently make the contract per-backend -- the exact split the lane crate exists to
#     prevent, and the one no single-target test can see;
#   * wasm relaxed SIMD's `f32x4_relaxed_madd` is *permitted to fuse or not at the engine's
#     discretion*, so it would make the contract nondeterministic across browsers.
#
# None of those would fail a build, and only the last would even be visible in a diff as an
# obviously numeric change. So the vocabulary is refused mechanically, in both directions: an
# unregistered call fails, and a registered exemption that no longer exists *also* fails, so the
# roster cannot quietly describe a tree that has moved on.
set -euo pipefail

root=$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)
cd "$root"

fail() {
    printf 'unfused seal failure: %s\n' "$1" >&2
    exit 1
}

# The two dispatch points. These define the contract and must state it in the unfused form.
dispatch_wide=crates/miso-engine-lane/src/wide_impl.rs
dispatch_scalar=crates/miso-engine-lane/src/scalar.rs

# The exemption registry: source file, and the exact number of fused calls it may make.
#
# There are exactly two, and both exist for the same reason: to compute the contract that was
# retired, so that the contract that replaced it can be measured *against* something.
#
#   * the audit that justified the change -- without its fused arm the bounds in
#     `docs/rulings/unfused-multiply-add-audit.md` could never be reproduced, because the fused arm
#     no longer exists anywhere else in the tree;
#   * gate G5's native leg -- its `lane_fma` case asserts that `Lane::fma` equals a written-out
#     multiply and add, and that assertion would pass vacuously the moment the corpus operands
#     stopped separating the two forms. The fused reference is what keeps it honest. It lives in
#     the native gate rather than the corpus crate because the corpus is compiled into the wasm
#     guest, where `mul_add` is unfused and a "fused" reference would silently become a second copy
#     of the unfused one.
#
# Both are evidence code: neither is reachable from a render path, and neither is `Lane::fma`.
#
# Adding a row here is the deliberate act the seal exists to require. A new row means someone has
# decided a fused multiply-add belongs in the workspace again, which is an owner-ruling-sized
# decision (the last one moved every pinned bit in the tree).
exemption_registry() {
    cat <<'EOF'
tools/miso-engine-audit/src/unfused_fma.rs 7
tools/miso-engine-wasm-gates/tests/g5_native_corpus.rs 1
EOF
}

# The number of fused calls the container claims exist in the whole workspace. "No fused
# multiply-add anywhere" is only a meaningful statement if the residue is written down and checked.
expected_fused_call_count=8

# A call of a fused multiply-add, in any of its spellings. The `(` is what separates a call from a
# `use` line or a doc mention.
#
# Counted against *comment-stripped* source: this contract is discussed at length in prose all over
# the workspace -- `lib.rs` explains why the fusion is gone, `lane_math.rs` records the precedent,
# this file names every spelling -- and a seal that counts a comment as a call is a seal that can
# be tripped by writing documentation. Stripping `//` to end of line can also blank a `//` inside a
# string literal, which would only ever hide a non-call, never admit a real one.
#
# The optional turbofish is not decoration: without it `mul_add::<f32>(x, y, z)` would not match,
# and the count that makes "exactly N" checkable could be evaded by spelling the type explicitly.
call_pattern='\b(mul_add|mul_neg_add|mul_sub|mul_neg_sub|fmaf|fma_f32_via_f64|fma_f32x4_soft|fma_f32x8_soft|f32x4_relaxed_madd|f32x4_relaxed_nmadd|f64x2_relaxed_madd|_mm256_fmadd_ps|_mm256_fmsub_ps|_mm256_fnmadd_ps|_mm_fmadd_ps|_mm_fmadd_ss|vfmaq_f32|vfmsq_f32)\s*(::<[^>]*>)?\s*\('
# The same pattern in POSIX ERE for `awk`, which has no `\s` and no PCRE escapes.
awk_call_pattern='(mul_add|mul_neg_add|mul_sub|mul_neg_sub|fmaf|fma_f32_via_f64|fma_f32x4_soft|fma_f32x8_soft|f32x4_relaxed_madd|f32x4_relaxed_nmadd|f64x2_relaxed_madd|_mm256_fmadd_ps|_mm256_fmsub_ps|_mm256_fnmadd_ps|_mm_fmadd_ps|_mm_fmadd_ss|vfmaq_f32|vfmsq_f32)([[:space:]]*::<[^>]*>)?[[:space:]]*[(]'
without_comments() { sed 's://.*::' "$1"; }
# Occurrences, not matching lines. `rg -c` counts lines, and the audit's SVF output mix puts two
# fused calls on one line -- counting lines would let a second call hide behind the first.
# The `|| true` is load-bearing under `set -o pipefail`: `rg` exits 1 when it matches nothing,
# which is the *normal* case for almost every file in the tree, and without it the seal would
# exit silently -- passing only because every file it happened to look at contained a match.
count_calls() {
    without_comments "$1" | { rg -o -e "$call_pattern" 2>/dev/null || true; } | wc -l | tr -d ' '
}

[[ -f "$dispatch_wide" ]] || fail "the vector dispatch point $dispatch_wide is missing"
[[ -f "$dispatch_scalar" ]] || fail "the scalar dispatch point $dispatch_scalar is missing"

# ---------------------------------------------------------------------------------------------
# 1. Both dispatch points state the contract in the unfused form.
#
# A positive check, not just the absence of `mul_add`: deleting the body would pass a pure absence
# test. `(self * b) + c` is the literal contract, and the parentheses are part of it -- they are
# what stop a future reader from "simplifying" the expression into an order the backends could
# reassociate.
# ---------------------------------------------------------------------------------------------
for dispatch in "$dispatch_wide" "$dispatch_scalar"; do
    without_comments "$dispatch" | rg -q -F '(self * b) + c' ||
        fail "$dispatch no longer states 'Lane::fma' as the unfused '(self * b) + c'"
done

# ---------------------------------------------------------------------------------------------
# 2. The contract is not split by target.
#
# The failure this catches is the tempting one: fuse where the hardware has an instruction, unfuse
# where it does not. That reintroduces a per-backend numeric contract, which no single-target test
# can detect -- only the wasm gate legs would, and only after a re-pin had already hidden it. So
# the fma body is required to contain no conditional compilation at all.
# ---------------------------------------------------------------------------------------------
for dispatch in "$dispatch_wide" "$dispatch_scalar"; do
    body=$(without_comments "$dispatch" |
        awk '/fn fma\(self, b: Self, c: Self\) -> Self \{/ { capture = 1 }
             capture { print }
             capture && /^ *\}$/ { capture = 0 }')
    [[ -n "$body" ]] || fail "$dispatch has no recognisable 'Lane::fma' body"
    if printf '%s\n' "$body" | rg -q 'cfg\s*\(|cfg!|target_arch|target_feature'; then
        fail "$dispatch conditions 'Lane::fma' on the target -- the contract must not be per-backend"
    fi
done

# ---------------------------------------------------------------------------------------------
# 3. No unregistered file calls a fused multiply-add.
# ---------------------------------------------------------------------------------------------
registered_files=$(exemption_registry | awk '{ print $1 }' | sort)

candidates=$(rg -l -e "$call_pattern" crates hosts tools sidecars --glob '*.rs' 2>/dev/null | sort || true)
while IFS= read -r file; do
    [[ -n "$file" ]] || continue
    # Re-test against comment-stripped source: `rg -l` above matched prose too.
    calls=$(count_calls "$file")
    # `[[ ... ]] && continue` would be a `set -e` trap here: when the test is false the list
    # returns non-zero and the shell exits silently, which is exactly the failure this seal must
    # never have (an exit code with no message reads as a refusal nobody can act on).
    if [[ "$calls" == "0" ]]; then
        continue
    fi
    if ! printf '%s\n' "$registered_files" | grep -qxF "$file"; then
        fail "fused multiply-add in $file -- the contract is unfused everywhere (#163 phase 2); \
see docs/rulings/unfused-multiply-add-audit.md"
    fi
done <<<"$candidates"

# ---------------------------------------------------------------------------------------------
# 4. Every registered exemption still exists, with exactly the registered call count, and every
#    call carries its marker.
#
# The marker must sit on the call or within the six lines above it, so the justification travels
# with the code rather than living in this script.
# ---------------------------------------------------------------------------------------------
while read -r file count; do
    [[ -n "$file" ]] || continue
    [[ -f "$file" ]] ||
        fail "registered exemption file $file does not exist -- the registry has rotted"

    actual=$(count_calls "$file")
    [[ "$actual" == "$count" ]] ||
        fail "$file has $actual fused calls, the registry says $count"

    unmarked=$(awk -v pattern="$awk_call_pattern" '
        { source = $0; sub(/\/\/.*/, "", source) }
        { for (i = 6; i >= 1; i--) window[i + 1] = window[i]; window[1] = $0 }
        source ~ pattern {
            marked = 0
            for (i = 1; i <= 7; i++) if (window[i] ~ /UNFUSED-SEAL-EXEMPT/) marked = 1
            if (!marked) print FILENAME ":" FNR
        }
    ' "$file")
    [[ -z "$unmarked" ]] ||
        fail "fused call without an UNFUSED-SEAL-EXEMPT marker within six lines: $unmarked"
done < <(exemption_registry)

# ---------------------------------------------------------------------------------------------
# 5. N is what the container says it is, counted two independent ways.
#
# The registry sum and a tree-wide recount must agree. One number is what the roster claims; the
# other is what the tree contains. A seal that only ever consults its own roster is checking its
# arithmetic, not the codebase.
# ---------------------------------------------------------------------------------------------
declared=$(exemption_registry | awk '{ total += $2 } END { print total + 0 }')
[[ "$declared" == "$expected_fused_call_count" ]] ||
    fail "the registry declares $declared fused calls, the container claims $expected_fused_call_count"

counted=0
while IFS= read -r file; do
    [[ -n "$file" ]] || continue
    calls=$(count_calls "$file")
    counted=$((counted + calls))
done < <(rg -l -e "$call_pattern" crates hosts tools sidecars --glob '*.rs' 2>/dev/null || true)
[[ "$counted" == "$expected_fused_call_count" ]] ||
    fail "found $counted fused calls in the tree, expected $expected_fused_call_count"

# ---------------------------------------------------------------------------------------------
# 6. The retired emulation stays retired.
#
# `softfma.rs` survives because it houses the MXCSR helpers gate G6 needs, and it kept its name
# because three policy files name that path. Keeping the name means the file could quietly regrow
# the thing it was named for, so the definition is refused explicitly rather than left to rule 3.
# ---------------------------------------------------------------------------------------------
if rg -qn 'fn\s+fma_f32_via_f64\b|fn\s+fma_f32x[48]_soft\b' crates/miso-engine-lane/src/softfma.rs; then
    fail 'the software FMA is retired (#163 phase 2) -- restoring it needs a ruling, not a commit'
fi

printf 'unfused seal: ok (no fused multiply-add on any path; %s registered audit calls)\n' \
    "$expected_fused_call_count"
