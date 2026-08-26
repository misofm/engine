#!/usr/bin/env bash
# Issue #175: the standing console fixture is exactly what its generator produces.
#
# `fixtures/session/v1/console-sixty-four-track-intended.toml` is derived, not authored. It is the
# retired 64-track fixture with the compressor moved verbatim onto the end of the `simd1` chain and
# a true-peak limiter added to `simd2`, spelled canonically by the session validator. That
# derivation is the entire basis for reading the two fixtures as a controlled pair: every EQ and
# compressor coefficient is byte-identical between them, so the only arithmetic difference is the
# limiter and the only structural difference is chain shape.
#
# A hand-edit to the committed file would break that silently -- the benchmark would still run, the
# validators would still pass, and the chain-shape row-pair would quietly be comparing two
# different sessions. So the pin here is not a set of properties the file should have. It is the
# file, regenerated and compared byte for byte.
set -euo pipefail
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
export LC_ALL=C

fixture=fixtures/session/v1/console-sixty-four-track-intended.toml
generator=scripts/derive-intended-console-fixture.py

fail() { printf 'intended console fixture failure: %s\n' "$1" >&2; exit 1; }

[[ -f "$fixture" && ! -L "$fixture" ]] || fail "missing fixture: $fixture"
[[ -f "$generator" && ! -L "$generator" ]] || fail "missing generator: $generator"

# 1. The fixture is its generator's output. This also re-proves, on every run, that the derivation
#    still passes all four stages of the real session pipeline: the generator takes its canonical
#    spelling from `miso-engine-session-validator --canonical`, which writes nothing when a stage
#    fails.
regenerated=$(mktemp)
trap 'rm -f "$regenerated"' EXIT
python3 "$generator" >"$regenerated" || fail 'the generator refused to derive the fixture'
cmp -s "$regenerated" "$fixture" || {
    diff -u "$fixture" "$regenerated" | head -40 >&2
    fail 'the committed fixture is not what the generator produces (regenerate, do not hand-edit)'
}

# 2. The structural facts the subject and the records depend on. Redundant with the byte compare
#    above by construction, and kept anyway: if the generator itself is ever changed, these say
#    which property the change broke instead of only reporting that some bytes moved.
tracks=$(grep -c 'id = "ch[0-9][0-9]", source_id' "$fixture" || true)
[[ "$tracks" == 64 ]] || fail "expected 64 tracks, found $tracks"
[[ "$((tracks % 8))" == 0 ]] || fail 'track count is not a whole number of eight-lane banks'

for pattern in 'miso.parametric-eq' 'miso.compressor' 'miso.true-peak-limiter'; do
    count=$(grep -o "$pattern" "$fixture" | wc -l)
    [[ "$count" == 64 ]] || fail "expected 64 occurrences of $pattern, found $count"
done

# The intended layout itself: a two-slot `simd1` chain, an empty `dynamic` rack, a one-slot
# `simd2` chain. This is the shape the whole issue is about, so it is asserted per track rather
# than sampled.
empty_dynamic=$(grep -o 'dynamic = { effects = \[\] }' "$fixture" | wc -l)
[[ "$empty_dynamic" == 64 ]] || fail "expected 64 empty dynamic racks, found $empty_dynamic"
limiter_in_simd2=$(grep -o 'simd2 = { effects = \[{ id = "limiter"' "$fixture" | wc -l)
[[ "$limiter_in_simd2" == 64 ]] || fail "expected the limiter on simd2 of all 64 tracks, found $limiter_in_simd2"
# The compressor must be the *second* slot of `simd1`: strip order is
# `simd1 -> dynamic -> simd2`, so EQ-then-compressor is what preserves the retired fixture's
# traversal order and therefore its rendered bits.
eq_then_comp=$(grep -o 'id = "eq".*id = "comp"' "$fixture" | wc -l)
[[ "$eq_then_comp" == 64 ]] || fail "expected EQ before compressor on all 64 simd1 chains, found $eq_then_comp"

# The launch rate and quantum the records pin, and the session's own identity.
grep -Fqx 'sample_rate_hz = 48000' "$fixture" || fail 'sample rate is not the launch rate'
grep -Fqx 'quantum_frames = 128' "$fixture" || fail 'quantum is not the launch quantum'
grep -Fqx 'session_id = "console-sixty-four-track-intended"' "$fixture" || fail 'session id changed'
# The canonical spelling of an empty array is a two-line `[` / `]`, not `[]`.
[[ "$(sed -n '/^automation = \[$/,/^\]$/p' "$fixture" | wc -l)" == 2 ]] ||
    fail 'the standing fixture must carry no automation'

# The limiter is stereo-linked on every track. A true-peak limiter is one of the few effects whose
# link mode is not inert, and homogeneous banking needs one link mode across the cohort.
linked=$(grep -o 'effect_id = "miso.true-peak-limiter" }, quality = "normal", bypass = false, link_mode = "maximum"' "$fixture" | wc -l)
[[ "$linked" == 64 ]] || fail "expected 64 stereo-linked limiters, found $linked"

# No two strips may share a coefficient set.
distinct_trims=$(grep -o 'trim_db = [-0-9.]*' "$fixture" | sort -u | wc -l)
[[ "$distinct_trims" -ge 8 ]] || fail "tracks share trim coefficients ($distinct_trims distinct values)"
# Scoped to the limiter's own declaration: the compressor's threshold is also `parameter_id = 1`
# in decibels, so an unscoped match would count both effects and pass on 64 identical ceilings.
distinct_ceilings=$(grep -o 'id = "limiter".\{0,220\}' "$fixture" |
    grep -o 'parameter_id = 1, channel = "both", unit = "db", value = -[0-9.]*' | sort -u | wc -l)
[[ "$distinct_ceilings" -ge 64 ]] || fail "limiters share ceilings ($distinct_ceilings distinct values)"

printf 'intended console fixture: ok (regenerates byte-identically; 64 tracks, simd1 eq+comp, simd2 limiter, %s distinct ceilings)\n' "$distinct_ceilings"
