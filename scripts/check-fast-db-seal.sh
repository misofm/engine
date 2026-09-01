#!/usr/bin/env bash
# The fast dB tier's container (issue #144 item 5, "Condition H"; #149 workplan item 2).
#
# `math::fast_db` is deliberately less accurate than the exact `exp2_lane`/`log2_lane`
# tier: about twice the error, for about a fifth of the polynomial. That trade is correct on a
# dynamics detector, whose result is a level reading multiplied into audio, and wrong on anything
# whose result becomes a pinned coefficient word -- a filter design, a route gain, a parameter
# mapping. Nothing in the type system distinguishes those two uses, and the cheaper function is
# the one a future author will reach for.
#
# So the tier is sealed. The fast vocabulary is admitted only inside the module that defines it,
# the gate that proves its bound, and an enumerated registry of *named crossings*. A new call site
# fails this gate until someone adds it to the registry, which is the point: crossing the seal
# should be an act, not an accident.
#
# The registry is checked in both directions (issue #144 item 4, "seal exclusions that cannot
# rot"): an unregistered call fails, and a registered crossing that no longer exists *also* fails,
# so the roster cannot quietly describe a tree that has moved on.
set -euo pipefail

root=$(cd "${1:-$(dirname "${BASH_SOURCE[0]}")/..}" && pwd)
cd "$root"

fail() {
    printf 'fast dB seal failure: %s\n' "$1" >&2
    exit 1
}

# The module that defines the tier, and the gate that proves its error bound. Both may use the
# vocabulary freely; neither is a crossing.
sealed_module=crates/math/src/fast_db.rs
sealed_gate=crates/math/tests/f1_fast_db_bounds.rs
# The owning crate's root, which declares the module. It may name it and nothing else: a call
# inside `math` would put the fast tier behind an exact-tier spelling, which is the
# one confusion this seal cannot detect anywhere else.
sealed_owner=crates/math/src/lib.rs

# The named crossings: source file, exact number of fast-tier calls, and the crossing identifiers
# that must mark them. Every row is a reviewed decision that this call site is a dynamics gain
# path. Adding a row is the deliberate act the seal exists to require.
crossing_registry() {
    cat <<'EOF'
crates/compressor/src/kernel.rs 2 X1 X2
crates/gate-expander/src/kernel.rs 2 X3 X4
crates/multiband-compressor/src/lib.rs 2 X5 X6
EOF
}

# Test files permitted to name the tier in order to *restate its law*, with the exact number of
# calls each may make. A restatement is not a crossing: it puts no fast arithmetic on a render
# path. It is enumerated anyway, and checked in both directions like the crossing registry, so a
# test cannot become the route by which the vocabulary spreads.
restatement_registry() {
    cat <<'EOF'
crates/gate-expander/tests/contract.rs 1
EOF
}

# The number of crossings the container claims. "Exactly N named crossings" is only a meaningful
# statement if N is written down and checked, so it is.
expected_crossing_count=6

# A call of the fast tier. The `(` is what separates a call from the `use` line that imports it.
#
# Counted against *comment-stripped* source: the crossing markers below quote the function names
# in prose, and a seal that counts a comment as a call is a seal that can be fooled by writing
# one. Stripping from `//` to end of line can also blank a `//` inside a string literal, which
# would only ever hide a non-call, never admit a real one.
# The optional turbofish is not decoration: without it `fast_level_db::<f32>(x)` would not match,
# and the call count -- the thing that makes "exactly N crossings" checkable -- could be evaded by
# spelling the type parameter explicitly.
call_pattern='\bfast_(level_db|gain_from_db)\s*(::<[^>]*>)?\s*\('
# The same call pattern in POSIX ERE, for `awk`, which has no `\s` and no PCRE escapes. The `(`
# is written as a bracket expression so it needs no escaping at all.
awk_call_pattern='fast_(level_db|gain_from_db)([[:space:]]*::<[^>]*>)?[[:space:]]*[(]'
without_comments() { sed 's://.*::' "$1"; }
# Any mention of the tier at all, including imports and module paths.
vocabulary_pattern='\bfast_db\b|\bfast_(level_db|gain_from_db|exp2|log2)\b'

[[ -f "$sealed_module" ]] || fail "the sealed module $sealed_module is missing"
[[ -f "$sealed_gate" ]] || fail "the bound gate $sealed_gate is missing"

# The owner declares the module and does not use it.
owner_uses=$(rg -n -e "$vocabulary_pattern" "$sealed_owner" | rg -v 'pub mod fast_db;' || true)
[[ -z "$owner_uses" ]] ||
    fail "$sealed_owner may declare the fast dB module and nothing else, found: $owner_uses"

# ---------------------------------------------------------------------------------------------
# 1. The tier's primitives stay private to the sealed module.
#
# Only the two decibel conversions are exported. If `fast_exp2`/`fast_log2` were public, a caller
# could recombine them with its own constants and the seal would be policing a spelling rather
# than a conversion.
# ---------------------------------------------------------------------------------------------
if rg -qn 'pub(\s*\([^)]*\))?\s+fn\s+fast_(exp2|log2)\b' "$sealed_module"; then
    fail 'fast_exp2/fast_log2 must stay private to the sealed module'
fi
for required in 'pub fn fast_level_db' 'pub fn fast_gain_from_db'; do
    rg -qn -- "$required" "$sealed_module" || fail "$sealed_module no longer defines '$required'"
done

# ---------------------------------------------------------------------------------------------
# 2. No unregistered file mentions the fast vocabulary.
# ---------------------------------------------------------------------------------------------
registered_files=$( { crossing_registry; restatement_registry; } | awk '{ print $1 }' | sort)

mentions=$(rg -l -e "$vocabulary_pattern" crates hosts tools sidecars --glob '*.rs' 2>/dev/null | sort || true)
while IFS= read -r file; do
    [[ -n "$file" ]] || continue
    [[ "$file" == "$sealed_module" || "$file" == "$sealed_gate" || "$file" == "$sealed_owner" ]] &&
        continue
    if ! printf '%s\n' "$registered_files" | grep -qxF "$file"; then
        fail "unregistered use of the fast dB vocabulary in $file -- add a named crossing or use the exact tier"
    fi
done <<<"$mentions"

# ---------------------------------------------------------------------------------------------
# 3. Every registered crossing still exists, with exactly the registered call count, and every
#    call carries its marker.
#
# The marker must sit on the call or within the six lines above it, so the justification travels
# with the code rather than living in this script.
# ---------------------------------------------------------------------------------------------
seen_ids=""
while read -r file count ids; do
    [[ -n "$file" ]] || continue
    [[ -f "$file" ]] || fail "registered crossing file $file does not exist -- the registry has rotted"

    actual=$(without_comments "$file" | rg -c -e "$call_pattern" 2>/dev/null || printf '0')
    [[ "$actual" == "$count" ]] ||
        fail "$file has $actual fast-tier calls, the registry says $count"

    for id in $ids; do
        marks=$(rg -c -e "FAST-DB-CROSSING $id\b" "$file" 2>/dev/null || printf '0')
        [[ "$marks" == "1" ]] ||
            fail "$file carries $marks markers for crossing $id, expected exactly 1"
        case " $seen_ids " in
            *" $id "*) fail "crossing identifier $id is used more than once" ;;
        esac
        seen_ids="$seen_ids $id"
    done

    # Marker proximity: every call needs a marker on it or within the six preceding lines.
    unmarked=$(awk -v pattern="$awk_call_pattern" '
        { source = $0; sub(/\/\/.*/, "", source) }
        { for (i = 6; i >= 1; i--) window[i + 1] = window[i]; window[1] = $0 }
        source ~ pattern {
            marked = 0
            for (i = 1; i <= 7; i++) if (window[i] ~ /FAST-DB-CROSSING/) marked = 1
            if (!marked) print FILENAME ":" FNR
        }
    ' "$file")
    [[ -z "$unmarked" ]] ||
        fail "fast-tier call without a FAST-DB-CROSSING marker within six lines: $unmarked"
done < <(crossing_registry)

# ---------------------------------------------------------------------------------------------
# 3b. Every restatement site still exists, with exactly the registered call count and its marker.
# ---------------------------------------------------------------------------------------------
while read -r file count; do
    [[ -n "$file" ]] || continue
    [[ -f "$file" ]] ||
        fail "registered restatement file $file does not exist -- the registry has rotted"

    actual=$(without_comments "$file" | rg -c -e "$call_pattern" 2>/dev/null || printf '0')
    [[ "$actual" == "$count" ]] ||
        fail "$file has $actual fast-tier calls, the restatement registry says $count"

    marks=$(rg -c -e 'FAST-DB-RESTATEMENT' "$file" 2>/dev/null || printf '0')
    [[ "$marks" == "1" ]] ||
        fail "$file carries $marks FAST-DB-RESTATEMENT markers, expected exactly 1"
done < <(restatement_registry)

# A restatement site must not carry a crossing marker: the two registries are disjoint, and a file
# in both would let a render-path call hide behind a test-shaped exemption.
while read -r file _; do
    [[ -n "$file" ]] || continue
    [[ -f "$file" ]] || continue
    if rg -qn -e 'FAST-DB-CROSSING' "$file"; then
        fail "$file is a restatement site and must not carry a FAST-DB-CROSSING marker"
    fi
done < <(restatement_registry)

# ---------------------------------------------------------------------------------------------
# 4. N is what the container says it is.
# ---------------------------------------------------------------------------------------------
declared=$(crossing_registry | awk '{ total += $2 } END { print total }')
[[ "$declared" == "$expected_crossing_count" ]] ||
    fail "the registry declares $declared crossings, the container claims $expected_crossing_count"

# The `rg` is wrapped in its own `|| true`: without it, `rg` exiting 2 on a missing search root
# (e.g. a hermetic test fixture with no sidecars/) would make the whole `markers=$(rg | awk)`
# pipeline exit non-zero under `pipefail`, which -- unlike an `if rg; then` guard -- aborts the
# script outright under `set -e` on an otherwise-clean tree.
markers=$({
    rg -c -e 'FAST-DB-CROSSING' crates hosts tools sidecars --glob '*.rs' 2>/dev/null || true
} | awk -F: '{ total += $2 } END { print total + 0 }')
[[ "$markers" == "$expected_crossing_count" ]] ||
    fail "found $markers FAST-DB-CROSSING markers in the tree, expected $expected_crossing_count"

printf 'fast dB seal: ok (%s named crossings)\n' "$expected_crossing_count"
