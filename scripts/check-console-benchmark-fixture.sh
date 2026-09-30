#!/usr/bin/env bash
# Issue-149 console JSON fixture integrity.
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s [fixture]\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
fixture=${1:-"$root/fixtures/session/v1/console-sixty-four-track.json"}
[[ -f "$fixture" && ! -L "$fixture" ]] || { printf 'console fixture failure: missing fixture: %s\n' "$fixture" >&2; exit 1; }

python3 -I -B - "$fixture" <<'PY'
import json
import sys
from pathlib import Path

document = json.loads(Path(sys.argv[1]).read_text())
tracks = document["tracks"]
assert len(tracks) == 64 and len(tracks) % 8 == 0
assert document["sample_rate_hz"] == 48000
assert document["quantum_frames"] == 128
assert document["session_id"] == "console-sixty-four-track"
assert document["automation"] == []
assert len(document["routes"]) == 64
assert all(route["source"]["tap"] == "post_pan" for route in document["routes"])
# Decision 12: the console's slots run on every track; a track's inserts are its own.
slots = {slot["slot"]: slot["identity"].get("effect_id")
         for section in ("pre_insert", "post_insert") for slot in document["console"][section]}
for track in tracks:
    ids = {slots[entry["slot"]] for entry in track["console"]}
    ids |= {effect["identity"].get("effect_id") for effect in track["inserts"]["effects"]}
    assert {"miso.parametric-eq", "miso.compressor"} <= ids
distinct = len({track["builtins"]["left"]["trim_db"] for track in tracks})
assert distinct >= 8
print(f"console fixture: ok (64 tracks, 8 full banks, EQ + compressor strip, {distinct} distinct trims)")
PY
