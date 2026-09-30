#!/usr/bin/env python3
"""Derive the app-shape 64-track JSON fixture (issue #1085, console shape since #1093).

The app (misofm/app) compiles EQ -> compressor onto every track and marks the
tracks a user has not selected ``bypass``. This fixture is that shape on the
standing console. Under decision 12 the EQ and the compressor are the session's
two ``pre_insert`` slots, exactly as the standing fixture declares them, and
each track keeps its own entry for each; the ``post_insert`` limiter slot and
every track's limiter entry go; and both entries of every track whose index is
2 mod 3 (21 of 64, about a third) are bypassed. Nothing else changes. Every
track's ``inserts`` stays empty. The Rust session writer remains the only
canonical-format authority.
"""
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
STANDING = ROOT / "fixtures/session/v1/console-sixty-four-track-intended.json"
TRACKS = 64
BYPASSED = [index for index in range(TRACKS) if index % 3 == 2]


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
            raise SystemExit("usage: derive-app-console-fixture.py [--validator <path>]")
        validator = args[1]
    document = json.loads(STANDING.read_text())
    document["session_id"] = "console-sixty-four-track-app"
    console = document["console"]
    ids = [slot["identity"]["effect_id"] for slot in console["pre_insert"]]
    assert ids == ["miso.parametric-eq", "miso.compressor"], ids
    kept = [slot["slot"] for slot in console["pre_insert"]]
    dropped = [slot["slot"] for slot in console["post_insert"]]
    assert [slot["identity"]["effect_id"] for slot in console["post_insert"]] == [
        "miso.true-peak-limiter"
    ]
    console["post_insert"] = []
    tracks = document["tracks"]
    assert len(tracks) == TRACKS
    for index, track in enumerate(tracks):
        assert [entry["slot"] for entry in track["console"]] == kept + dropped
        assert track["inserts"]["effects"] == []
        assert all(not entry["bypass"] for entry in track["console"])
        track["console"] = track["console"][: len(kept)]
        if index % 3 == 2:
            for entry in track["console"]:
                entry["bypass"] = True
    bypassed = [i for i, t in enumerate(tracks) if all(e["bypass"] for e in t["console"])]
    assert bypassed == BYPASSED and len(bypassed) == 21
    assert not any(e["bypass"] for i, t in enumerate(tracks) if i not in BYPASSED
                   for e in t["console"])
    sys.stdout.write(canonicalise(document, validator))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
