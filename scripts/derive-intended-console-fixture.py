#!/usr/bin/env python3
"""Derive the intended-placement 64-track JSON fixture.

Move each exact compressor object after the EQ, empty ``dynamic``, and add the
per-track true-peak limiter on ``simd2``. The Rust session writer remains the
only canonical-format authority.
"""
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STANDING = ROOT / "fixtures/session/v1/console-sixty-four-track.json"
TRACKS = 64


def limiter(index: int) -> dict:
    return {
        "id": "limiter",
        "identity": {"kind": "native", "effect_id": "miso.true-peak-limiter"},
        "quality": "normal", "bypass": False, "link_mode": "maximum",
        "params": [
            {"parameter_id": 1, "channel": "both", "unit": "db", "value": -0.5 - index / 32},
            {"parameter_id": 2, "channel": "both", "unit": "milliseconds", "value": 60.0 + index * 1.25},
            {"parameter_id": 3, "channel": "both", "unit": "milliseconds", "value": 5.0},
        ],
        "sidechain": {"kind": "none"},
    }


def canonicalise(document: dict, validator: str | None) -> str:
    with tempfile.TemporaryDirectory() as directory:
        draft = Path(directory) / "draft.json"
        draft.write_text(json.dumps(document, ensure_ascii=False))
        if validator:
            command = [validator, "validate", "--canonical", str(draft)]
        else:
            command = [
                "cargo", "run", "-q", "-p", "session-validator", "--",
                "validate", "--canonical", str(draft),
            ]
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, check=False)
    if result.returncode != 0:
        sys.stderr.write(result.stderr)
        raise SystemExit(f"session validator refused the derived draft (exit {result.returncode})")
    return result.stdout


def main() -> int:
    validator = None
    args = sys.argv[1:]
    if args:
        if args[0] != "--validator" or len(args) != 2:
            raise SystemExit("usage: derive-intended-console-fixture.py [--validator <path>]")
        validator = args[1]
    document = json.loads(STANDING.read_text())
    document["session_id"] = "console-sixty-four-track-intended"
    tracks = document["tracks"]
    assert len(tracks) == TRACKS
    for index, track in enumerate(tracks):
        simd1 = track["simd1"]["effects"]
        dynamic = track["dynamic"]["effects"]
        assert len(simd1) == 1 and simd1[0]["identity"]["effect_id"] == "miso.parametric-eq"
        assert len(dynamic) == 1 and dynamic[0]["identity"]["effect_id"] == "miso.compressor"
        simd1.append(dynamic.pop())
        track["simd2"]["effects"] = [limiter(index)]
    sys.stdout.write(canonicalise(document, validator))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
