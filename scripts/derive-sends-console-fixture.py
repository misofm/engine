#!/usr/bin/env python3
"""Derive the bus-and-send 64-track JSON fixture (issue #1227).

The native console benchmark's ``sixty_four_track_console_sends`` row renders a real bus-and-send
session: the intended fixture's 64 tracks feeding eight processed submix strips, two effect
returns, and two sends per track. The source, the rate, the quantum, the console declaration and
every track stay as the intended fixture writes them; only ``session_id``, ``submixes`` and
``routes`` change. Each of the ten submixes (``bus-0``..``bus-7``, ``fx-a``, ``fx-b``) has the
transparent input section of ``Submix::unity``, a deep copy of track ``ch00``'s three console
entries, one insert with its descriptor defaults (a ``maximum``-linked compressor on each bus, the
delay on ``fx-a``, the EQ on ``fx-b``), an unmuted 0 dB fader and the identity matrix. Track
``chNN``'s main route is re-pointed into ``bus-<NN div 8>``; every track gains a ``pre_fader``
send into ``fx-a`` at -12 dB and a ``post_fader`` send into ``fx-b`` at -18 dB; every submix
returns ``post_pan`` to ``main-out`` at 0 dB. No route is muted, every route into a submix follows
its source's mute (the SDK's default there) and every route into the output does not (the only
legal value). No VCA and no automation: a VCA is baked into the faders at preparation and adds no
render work. The Rust session writer remains the only canonical-format authority.
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
BUSES = 8
TRACKS_PER_BUS = TRACKS // BUSES
OUTPUT = "main-out"
IDENTITY = {"ll": 1.0, "lr": 0.0, "rl": 0.0, "rr": 1.0}


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


def insert(effect_id: str, name: str, link_mode: str) -> dict:
    return {
        "id": name,
        "identity": {"kind": "native", "effect_id": effect_id},
        "quality": "normal",
        "bypass": False,
        "link_mode": link_mode,
        "params": [],
        "sidechain": {"kind": "none"},
    }


def submix(name: str, console: list, effect: dict) -> dict:
    lane = {
        "polarity_invert": False,
        "trim_db": 0.0,
        "hpf_hz": 0.0,
        "lpf_hz": 0.0,
        "delay_samples": 0,
    }
    return {
        "id": name,
        "builtins": {"left": dict(lane), "right": dict(lane)},
        "console": copy.deepcopy(console),
        "inserts": {"effects": [effect]},
        "fader": {"left_db": 0.0, "right_db": 0.0, "left_mute": False, "right_mute": False},
        "matrix": dict(IDENTITY, smoothing_samples=0),
    }


def route(name: str, source: dict, destination: dict, gain_db: float) -> dict:
    into_submix = destination["kind"] == "submix_input"
    return {
        "id": name,
        "source": source,
        "destination": destination,
        "channel_matrix": dict(IDENTITY),
        "gain_db": gain_db,
        "mute": False,
        "follows_mute": into_submix,
    }


def main() -> int:
    validator = None
    args = sys.argv[1:]
    if args:
        if args[0] != "--validator" or len(args) != 2:
            raise SystemExit("usage: derive-sends-console-fixture.py [--validator <path>]")
        validator = args[1]
    document = json.loads(STANDING.read_text())

    # The parent's shape, as #1227's context states it.
    assert document["session_id"] == "console-sixty-four-track-intended"
    assert document["sample_rate_hz"] == 48000 and document["quantum_frames"] == 128
    assert len(document["sources"]) == 1
    console = document["console"]
    assert [s["identity"]["effect_id"] for s in console["pre_insert"]] == [
        "miso.parametric-eq", "miso.compressor"
    ]
    assert [s["identity"]["effect_id"] for s in console["post_insert"]] == [
        "miso.true-peak-limiter"
    ]
    slots = [s["slot"] for s in console["pre_insert"] + console["post_insert"]]
    assert slots == ["eq", "comp", "limiter"]
    tracks = document["tracks"]
    assert [t["id"] for t in tracks] == [f"ch{i:02d}" for i in range(TRACKS)]
    for track in tracks:
        assert [entry["slot"] for entry in track["console"]] == slots
        assert all(not entry["bypass"] for entry in track["console"])
        assert track["inserts"]["effects"] == []
    assert [o["id"] for o in document["outputs"]] == [OUTPUT]
    routes = document["routes"]
    assert [r["id"] for r in routes] == [f"ch{i:02d}-main" for i in range(TRACKS)]
    for index, main_route in enumerate(routes):
        assert main_route["source"] == {"kind": "track", "track_id": f"ch{index:02d}",
                                        "tap": "post_pan"}
        assert main_route["destination"] == {"kind": "output_input", "output_id": OUTPUT}
        assert main_route["mute"] is False and main_route["follows_mute"] is False
    assert document["submixes"] == [] and document["vcas"] == []
    assert document["automation"] == []

    document["session_id"] = "console-sixty-four-track-sends"
    strip = tracks[0]["console"]
    buses = [f"bus-{k}" for k in range(BUSES)]
    document["submixes"] = [
        submix(name, strip, insert("miso.compressor", "glue", "maximum")) for name in buses
    ] + [
        submix("fx-a", strip, insert("miso.delay", "echo", "dual_mono")),
        submix("fx-b", strip, insert("miso.parametric-eq", "tone", "dual_mono")),
    ]

    def into(submix_id: str) -> dict:
        return {"kind": "submix_input", "submix_id": submix_id}

    out = {"kind": "output_input", "output_id": OUTPUT}
    for index, main_route in enumerate(routes):
        main_route["destination"] = into(buses[index // TRACKS_PER_BUS])
        main_route["follows_mute"] = True
    for index in range(TRACKS):
        track_id = f"ch{index:02d}"
        routes.append(route(f"{track_id}-fx-a",
                            {"kind": "track", "track_id": track_id, "tap": "pre_fader"},
                            into("fx-a"), -12.0))
        routes.append(route(f"{track_id}-fx-b",
                            {"kind": "track", "track_id": track_id, "tap": "post_fader"},
                            into("fx-b"), -18.0))
    for name in buses + ["fx-a", "fx-b"]:
        routes.append(route(f"{name}-main",
                            {"kind": "submix", "submix_id": name, "tap": "post_pan"},
                            out, 0.0))
    assert len(routes) == TRACKS + 2 * TRACKS + BUSES + 2 == 202
    sys.stdout.write(canonicalise(document, validator))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
