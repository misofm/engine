#!/usr/bin/env bash
# Issue-149 console fixture integrity.
#
# The fixture is the workload. A benchmark whose fixture drifted is measuring something other than
# what its records claim, which is the "measuring a fiction" failure the bench discipline exists to
# catch -- so the shape the subject depends on is asserted here rather than assumed.
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s [fixture]\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
fixture=${1:-"$root/fixtures/session/v1/console-sixty-four-track.toml"}
export LC_ALL=C

fail() { printf 'console fixture failure: %s\n' "$1" >&2; exit 1; }

[[ -f "$fixture" && ! -L "$fixture" ]] || fail "missing fixture: $fixture"

# Exactly sixty-four tracks: the whole point of the fixture is that it is eight full eight-lane
# banks with no scalar tail. Sixty-three or sixty-five would silently reintroduce the ragged shape.
tracks=$(grep -c '^\[\[tracks\]\]$' "$fixture" || true)
[[ "$tracks" == 64 ]] || fail "expected 64 tracks, found $tracks"
[[ "$((tracks % 8))" == 0 ]] || fail "track count is not a whole number of eight-lane banks"

# Every track carries the full strip and its own route.
for pattern in 'miso.parametric-eq' 'miso.compressor'; do
    count=$(grep -c "$pattern" "$fixture" || true)
    [[ "$count" == 64 ]] || fail "expected 64 occurrences of $pattern, found $count"
done
routes=$(grep -c 'tap = "post_matrix" }, destination' "$fixture" || true)
[[ "$routes" == 64 ]] || fail "expected 64 post-matrix routes, found $routes"

# The launch rate and quantum the records pin.
grep -Fqx 'sample_rate_hz = 48000' "$fixture" || fail 'sample rate is not the launch rate'
grep -Fqx 'quantum_frames = 128' "$fixture" || fail 'quantum is not the launch quantum'
grep -Fqx 'session_id = "console-sixty-four-track"' "$fixture" || fail 'session id changed'
grep -Fqx 'automation = []' "$fixture" || fail 'the standing fixture must carry no automation'

# No two strips may share a coefficient set: identical tracks would let the measurement collapse
# work a real console cannot collapse, and the per-track number would be a fiction.
distinct=$(grep -o 'trim_db = [-0-9.]*' "$fixture" | sort -u | wc -l)
[[ "$distinct" -ge 8 ]] || fail "tracks share trim coefficients ($distinct distinct values)"

printf 'console fixture: ok (64 tracks, 8 full banks, EQ + compressor strip, %s distinct trims)\n' "$distinct"
