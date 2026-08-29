#!/usr/bin/env bash
# The mono console fixture is exactly what its generator produces.
#
# `fixtures/session/v1/console-sixty-four-track-mono.toml` is derived, not authored: it is the
# standing intended-placement fixture with three edits, all upstream of the fader/matrix seam --
# both channels read source channel 0, `builtins.right` copies `builtins.left`, and every
# `channel = "right"` effect parameter takes its `channel = "left"` sibling's value. Every track is
# therefore collapse-eligible by the two structural terms of the channel-symmetry witness
# (`crates/miso-engine-effect-contract/src/symmetry.rs`), which is the fixture's whole purpose.
#
# A hand-edit would break that silently. The `sixty_four_track_console_mono` /
# `sixty_four_track_console_mono_dual` row-pair would keep running, keep validating and keep
# asserting a digest equality that had stopped meaning "a collapse-eligible session renders the
# same bits with the collapse taken as without it". So the pin is not a set of properties the file
# should have; it is the file, regenerated and compared byte for byte.
#
# This mirrors `check-intended-console-fixture.sh` exactly, for the same reason and in the same
# order: the byte compare first, then the structural facts the subject and the records depend on,
# which are redundant with it by construction and kept so that a deliberate generator change says
# *which* property it moved.
set -euo pipefail
[[ "$#" == 0 ]] || { printf 'usage: %s\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
export LC_ALL=C

fixture=fixtures/session/v1/console-sixty-four-track-mono.toml
standing=fixtures/session/v1/console-sixty-four-track-intended.toml
generator=scripts/derive-mono-console-fixture.py

fail() { printf 'mono console fixture failure: %s\n' "$1" >&2; exit 1; }

[[ -f "$fixture" && ! -L "$fixture" ]] || fail "missing fixture: $fixture"
[[ -f "$generator" && ! -L "$generator" ]] || fail "missing generator: $generator"

# 1. The fixture is its generator's output. This also re-proves, on every run, that the derivation
#    still passes all four stages of the real session pipeline: the generator takes its canonical
#    spelling from `miso-engine-session-validator --canonical`, which writes nothing when a stage
#    fails.
regenerated=$(mktemp)
trap 'rm -f "$regenerated"' EXIT
python3 -I -B "$generator" >"$regenerated" || fail 'the generator refused to derive the fixture'
cmp -s "$regenerated" "$fixture" || {
    diff -u "$fixture" "$regenerated" | head -40 >&2
    fail 'the committed fixture is not what the generator produces (regenerate, do not hand-edit)'
}

# Every count below is taken over the *track* lines only. The header comment quotes the very
# strings this file greps for -- `right_source_channel = 1`, `link_mode = "maximum"` -- and an
# unscoped grep would count the prose, which is the kind of check that passes for the wrong reason.
tracks_only=$(grep '^  { id = "ch[0-9]*", source_id = ' "$fixture")
# Extended regular expressions throughout: several of the patterns below are literal
# `{`/`[`, which a basic-regexp `grep -o` refuses outright.
count() { printf '%s\n' "$tracks_only" | grep -oE "$1" | wc -l; }

tracks=$(printf '%s\n' "$tracks_only" | wc -l)
[[ "$tracks" == 64 ]] || fail "expected 64 tracks, found $tracks"
[[ "$((tracks % 8))" == 0 ]] || fail 'track count is not a whole number of eight-lane banks'

# 2. The SOURCE term: both channels of every track read source channel 0.
mono_sources=$(count 'left_source_channel = 0, right_source_channel = 0')
[[ "$mono_sources" == 64 ]] || fail "expected 64 mono source mappings, found $mono_sources"
[[ "$(count 'right_source_channel = 1')" == 0 ]] || fail 'a stereo source mapping survived'
# ...and the source still declares two channels, which is what lets the half-mono bench row put
# `right_source_channel = 1` back on its odd tracks in code.
grep -Fq 'channels = 2, bit_depth = "32f"' "$fixture" ||
    fail 'the source must still declare two float channels'

# 3. The DESIGNED term, both halves.
symmetric_builtins=$(printf '%s\n' "$tracks_only" |
    grep -oE 'builtins = \{ left = \{ [^{}]* \}, right = \{ [^{}]* \} \}' |
    sed -E 's/^builtins = \{ left = \{ (.*) \}, right = \{ (.*) \} \}$/\1|\2/' |
    awk -F'|' '$1 == $2' | wc -l)
[[ "$symmetric_builtins" == 64 ]] || fail "expected 64 symmetric builtin pairs, found $symmetric_builtins"
asymmetric_params=$(printf '%s\n' "$tracks_only" |
    grep -oE '\{ parameter_id = [0-9]+, channel = "left", unit = "[a-z_]+", value = -?[0-9.]+ \}, \{ parameter_id = [0-9]+, channel = "right", unit = "[a-z_]+", value = -?[0-9.]+ \}' |
    sed -E 's/.*channel = "left"[^=]*= "[a-z_]+", value = (-?[0-9.]+) \}.*value = (-?[0-9.]+) \}$/\1|\2/' |
    awk -F'|' '$1 != $2' | wc -l)
[[ "$asymmetric_params" == 0 ]] || fail "$asymmetric_params left/right parameter pairs still differ"
# The pairs must survive *as pairs*. Collapsing them into one `channel = "both"` entry would make
# the fixture prove that the witness reads a declaration shape rather than a designed word.
left_entries=$(count 'channel = "left"')
right_entries=$(count 'channel = "right"')
[[ "$left_entries" == 128 && "$right_entries" == 128 ]] ||
    fail "expected 128 left and 128 right parameter entries, found $left_entries and $right_entries"

# 4. What the fixture deliberately keeps: the seam, and the limiter's stereo link. These are not
#    incidental leftovers -- a mono fixture without them cannot distinguish a correct collapse from
#    one that wrongly gates on seam-side words, so their absence is a defect and is named as one.
seam_faders=$(printf '%s\n' "$tracks_only" |
    grep -oE 'fader = \{ left_db = -?[0-9.]+, right_db = -?[0-9.]+,' |
    sed -E 's/^fader = \{ left_db = (-?[0-9.]+), right_db = (-?[0-9.]+),$/\1|\2/' |
    awk -F'|' '$1 != $2' | wc -l)
[[ "$seam_faders" == 49 ]] || fail "expected the standing fixture's 49 asymmetric faders, found $seam_faders"
seam_pans=$(printf '%s\n' "$tracks_only" |
    grep -oE 'pan = \{ left = -?[0-9.]+, right = -?[0-9.]+,' |
    sed -E 's/^pan = \{ left = (-?[0-9.]+), right = (-?[0-9.]+),$/\1|\2/' |
    awk -F'|' '$1 != $2' | wc -l)
[[ "$seam_pans" == 50 ]] || fail "expected the standing fixture's 50 asymmetric pans, found $seam_pans"
linked=$(count 'effect_id = "miso\.true-peak-limiter" \}, quality = "normal", bypass = false, link_mode = "maximum"')
[[ "$linked" == 64 ]] || fail "expected 64 stereo-linked limiters, found $linked"

# 5. The strip itself is the standing fixture's, unchanged: same racks, same slot order, same
#    effects. Only what is listed above may differ between the two files.
for pattern in 'miso\.parametric-eq' 'miso\.compressor' 'miso\.true-peak-limiter'; do
    found=$(count "$pattern")
    [[ "$found" == 64 ]] || fail "expected 64 occurrences of ${pattern//\\/}, found $found"
done
[[ "$(count 'dynamic = \{ effects = \[\] \}')" == 64 ]] || fail 'expected 64 empty dynamic racks'
[[ "$(count 'id = "eq".*id = "comp"')" == 64 ]] || fail 'expected EQ before compressor on all 64 simd1 chains'
[[ "$(count 'simd2 = \{ effects = \[\{ id = "limiter"')" == 64 ]] || fail 'expected the limiter on simd2 of all 64 tracks'

# 6. The launch rate and quantum the records pin, the session's own identity, and the input the
#    mono row shares with the standing row -- the source content identity is byte-identical, which
#    is half of what makes the two fixtures' numbers comparable.
grep -Fqx 'sample_rate_hz = 48000' "$fixture" || fail 'sample rate is not the launch rate'
grep -Fqx 'quantum_frames = 128' "$fixture" || fail 'quantum is not the launch quantum'
grep -Fqx 'session_id = "console-sixty-four-track-mono"' "$fixture" || fail 'session id changed'
grep -Fq 'content = "sha256:aa28dfa39be77bff07309fb8d60983556232291660332ecb949dc3082a971f75"' "$fixture" ||
    fail 'the mono session must be fed the standing fixture source'
cmp -s <(grep -F 'sources = [' -A1 "$fixture") <(grep -F 'sources = [' -A1 "$standing") ||
    fail 'the source declaration drifted from the standing fixture'
[[ "$(sed -n '/^automation = \[$/,/^\]$/p' "$fixture" | wc -l)" == 2 ]] ||
    fail 'the mono fixture must carry no automation'

# 7. No two strips may share a coefficient set. Symmetrising a track's two channels must not have
#    made two *tracks* alike -- that would let the measurement collapse work a real console cannot.
distinct_trims=$(printf '%s\n' "$tracks_only" | grep -o 'trim_db = [-0-9.]*' | sort -u | wc -l)
[[ "$distinct_trims" -ge 8 ]] || fail "tracks share trim coefficients ($distinct_trims distinct values)"
distinct_strips=$(printf '%s\n' "$tracks_only" | sed -E 's/^  \{ id = "ch[0-9]+", source_id = "[^"]*", //' | sort -u | wc -l)
[[ "$distinct_strips" == 64 ]] || fail "expected 64 distinct strips, found $distinct_strips"

printf 'mono console fixture: ok (regenerates byte-identically; 64 mono-source tracks, symmetric upstream, %s seam faders / %s seam pans kept, 64 linked limiters)\n' \
    "$seam_faders" "$seam_pans"
