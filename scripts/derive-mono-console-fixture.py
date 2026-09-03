#!/usr/bin/env python3
"""Derive the collapse-eligible mono 64-track JSON fixture.

Both channels read source channel zero, the right builtin lane copies the left,
and right effect parameters take their left siblings' values. Fader and pan
asymmetry and the limiter link deliberately remain.
"""
import copy
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STANDING = ROOT / "fixtures/session/v1/console-sixty-four-track-intended.json"
TRACKS = 64


def canonicalise(document: dict) -> str:
    with tempfile.TemporaryDirectory() as directory:
        draft = Path(directory) / "draft.json"
        draft.write_text(json.dumps(document, ensure_ascii=False))
        result = subprocess.run(
            ["cargo", "run", "-q", "-p", "session-validator", "--", "validate", "--canonical", str(draft)],
            cwd=ROOT, capture_output=True, text=True, check=False,
        )
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        raise SystemExit(f"session validator refused the derived draft (exit {result.returncode})")
    return result.stdout


def main() -> int:
    document = json.loads(STANDING.read_text())
    document["session_id"] = "console-sixty-four-track-mono"
    tracks = document["tracks"]
    assert len(tracks) == TRACKS
    seam_faders = sum(t["fader"]["left_db"] != t["fader"]["right_db"] for t in tracks)
    seam_pans = sum(t["pan"]["left"] != t["pan"]["right"] for t in tracks)
    pairs = 0
    for track in tracks:
        track["right_source_channel"] = 0
        track["builtins"]["right"] = copy.deepcopy(track["builtins"]["left"])
        for rack_name in ("simd1", "dynamic", "simd2"):
            for effect in track[rack_name]["effects"]:
                left = {(p["parameter_id"], p["unit"]): p for p in effect["params"] if p["channel"] == "left"}
                for parameter in effect["params"]:
                    if parameter["channel"] == "right":
                        sibling = left.get((parameter["parameter_id"], parameter["unit"]))
                        if sibling is not None:
                            parameter["value"] = sibling["value"]
                            pairs += 1
    assert pairs == 2 * TRACKS, f"expected {2 * TRACKS} left/right pairs, saw {pairs}"
    assert seam_faders == 49 and seam_pans == 50
    assert all(t["right_source_channel"] == 0 for t in tracks)
    assert all(t["builtins"]["left"] == t["builtins"]["right"] for t in tracks)
    assert sum(e["link_mode"] == "maximum" for t in tracks for e in t["simd2"]["effects"]) == TRACKS
    sys.stdout.write(canonicalise(document))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
