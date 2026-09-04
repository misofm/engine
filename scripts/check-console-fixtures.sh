#!/usr/bin/env bash
# Regenerate the derived JSON sessions and prove their semantic witness shapes.
# Usage: check-console-fixtures.sh [path/to/session-validator]
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s [path/to/session-validator]\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$root"
export LC_ALL=C
tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT

validator_args=()
if [[ -n "${1:-}" ]]; then
    [[ -x "$1" ]] || { printf 'missing session-validator binary: %s\n' "$1" >&2; exit 1; }
    validator_args=(--validator "$1")
fi

python3 -I -B scripts/derive-intended-console-fixture.py "${validator_args[@]}" >"$tmp_dir/intended.json"
cmp "$tmp_dir/intended.json" fixtures/session/v1/console-sixty-four-track-intended.json
python3 -I -B scripts/derive-mono-console-fixture.py "${validator_args[@]}" >"$tmp_dir/mono.json"
cmp "$tmp_dir/mono.json" fixtures/session/v1/console-sixty-four-track-mono.json

python3 -I -B - <<'PY'
import json
from pathlib import Path

root = Path("fixtures/session/v1")
intended = json.loads((root / "console-sixty-four-track-intended.json").read_text())
mono = json.loads((root / "console-sixty-four-track-mono.json").read_text())
assert intended["session_id"] == "console-sixty-four-track-intended"
assert mono["session_id"] == "console-sixty-four-track-mono"
assert intended["sample_rate_hz"] == mono["sample_rate_hz"] == 48000
assert intended["quantum_frames"] == mono["quantum_frames"] == 128
assert intended["sources"] == mono["sources"]
assert len(intended["tracks"]) == len(mono["tracks"]) == 64
for track in intended["tracks"]:
    ids = [e["identity"]["effect_id"] for e in track["simd1"]["effects"]]
    assert ids == ["miso.parametric-eq", "miso.compressor"]
    assert track["dynamic"]["effects"] == []
    limiter = track["simd2"]["effects"]
    assert len(limiter) == 1 and limiter[0]["identity"]["effect_id"] == "miso.true-peak-limiter"
    assert limiter[0]["link_mode"] == "maximum"
assert len({t["simd2"]["effects"][0]["params"][0]["value"] for t in intended["tracks"]}) == 64
for track in mono["tracks"]:
    assert track["left_source_channel"] == track["right_source_channel"] == 0
    assert track["builtins"]["left"] == track["builtins"]["right"]
    for rack in ("simd1", "dynamic", "simd2"):
        for effect in track[rack]["effects"]:
            values = {(p["parameter_id"], p["channel"], p["unit"]): p["value"] for p in effect["params"]}
            for (parameter_id, channel, unit), value in values.items():
                if channel == "left":
                    assert values[(parameter_id, "right", unit)] == value
assert sum(t["fader"]["left_db"] != t["fader"]["right_db"] for t in mono["tracks"]) == 49
assert sum(t["pan"]["left"] != t["pan"]["right"] for t in mono["tracks"]) == 50
PY

printf 'console session fixtures: ok (canonical regeneration and 64-track intended/mono witnesses)\n'
