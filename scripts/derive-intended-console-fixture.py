#!/usr/bin/env python3
"""Derive the intended-placement 64-track JSON fixture.

The standing fixture runs the EQ as the ``console.pre_insert`` slot ``eq`` and
the compressor as each track's insert. Move the compressor into
``pre_insert`` after the EQ (one slot declaration, each track's exact knobs in
its console entry), empty the inserts, and add the true-peak limiter as the
``console.post_insert`` slot ``limiter``. The Rust session writer remains the
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


LIMITER_SLOT = {
    "slot": "limiter",
    "identity": {"kind": "native", "effect_id": "miso.true-peak-limiter"},
    "quality": "normal", "link_mode": "maximum",
}


def limiter(index: int) -> dict:
    return {
        "slot": "limiter", "bypass": False,
        "params": [
            {"parameter_id": 1, "channel": "both", "unit": "db", "value": -0.5 - index / 32},
            {"parameter_id": 2, "channel": "both", "unit": "milliseconds", "value": 60.0 + index * 1.25},
            {"parameter_id": 3, "channel": "both", "unit": "milliseconds", "value": 5.0},
        ],
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
    console = document["console"]
    assert [slot["identity"]["effect_id"] for slot in console["pre_insert"]] == ["miso.parametric-eq"]
    assert console["post_insert"] == []
    tracks = document["tracks"]
    assert len(tracks) == TRACKS
    compressor = None
    for index, track in enumerate(tracks):
        inserts = track["inserts"]["effects"]
        assert [entry["slot"] for entry in track["console"]] == ["eq"]
        assert len(inserts) == 1 and inserts[0]["identity"]["effect_id"] == "miso.compressor"
        insert = inserts.pop()
        assert insert["sidechain"] == {"kind": "none"}
        slot = {key: insert[key] for key in ("identity", "quality", "link_mode")}
        slot = {"slot": insert["id"], **slot}
        assert compressor in (None, slot), "every track's compressor is one console slot"
        compressor = slot
        track["console"].append(
            {"slot": insert["id"], "bypass": insert["bypass"], "params": insert["params"]}
        )
        track["console"].append(limiter(index))
    console["pre_insert"].append(compressor)
    console["post_insert"] = [LIMITER_SLOT]
    sys.stdout.write(canonicalise(document, validator))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
