#!/usr/bin/env bash
# Regenerate the derived JSON sessions and prove their semantic witness shapes.
# Usage: check-console-fixtures.sh [path/to/session-validator]
set -euo pipefail
[[ "$#" -le 1 ]] || { printf 'usage: %s [path/to/session-validator]\n' "$0" >&2; exit 2; }
root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)

# S3: resolve a relative validator path against the caller's cwd BEFORE `cd "$root"` below.
validator="${1:-}"
if [[ -n "$validator" ]]; then
    case "$validator" in
        /*) : ;;
        *) validator="$(realpath -m -- "$validator")" ;;
    esac
    # S1/S2: an explicit path must be an existing executable file, never a directory or a missing
    # path -- and being explicit-but-missing is a hard error here, never a build trigger.
    [[ -f "$validator" && -x "$validator" ]] || {
        printf 'missing session-validator binary: %s\n' "$validator" >&2
        exit 1
    }
fi

cd "$root"
export LC_ALL=C
tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT

validator_args=()
if [[ -n "$validator" ]]; then
    validator_args=(--validator "$validator")
fi

python3 -I -B scripts/derive-intended-console-fixture.py "${validator_args[@]}" >"$tmp_dir/intended.json"
cmp "$tmp_dir/intended.json" fixtures/session/v1/console-sixty-four-track-intended.json
python3 -I -B scripts/derive-mono-console-fixture.py "${validator_args[@]}" >"$tmp_dir/mono.json"
cmp "$tmp_dir/mono.json" fixtures/session/v1/console-sixty-four-track-mono.json
python3 -I -B scripts/derive-app-console-fixture.py "${validator_args[@]}" >"$tmp_dir/app.json"
cmp "$tmp_dir/app.json" fixtures/session/v1/console-sixty-four-track-app.json
python3 -I -B scripts/derive-sends-console-fixture.py "${validator_args[@]}" >"$tmp_dir/sends.json"
cmp "$tmp_dir/sends.json" fixtures/session/v1/console-sixty-four-track-sends.json

python3 -I -B - <<'PY'
import json
from pathlib import Path

root = Path("fixtures/session/v1")
intended = json.loads((root / "console-sixty-four-track-intended.json").read_text())
mono = json.loads((root / "console-sixty-four-track-mono.json").read_text())
app = json.loads((root / "console-sixty-four-track-app.json").read_text())
sends = json.loads((root / "console-sixty-four-track-sends.json").read_text())
assert intended["session_id"] == "console-sixty-four-track-intended"
assert mono["session_id"] == "console-sixty-four-track-mono"
assert intended["sample_rate_hz"] == mono["sample_rate_hz"] == 48000
assert intended["quantum_frames"] == mono["quantum_frames"] == 128
assert intended["sources"] == mono["sources"]
assert len(intended["tracks"]) == len(mono["tracks"]) == 64
# Decision 12: EQ -> compressor are the session's `pre_insert` slots and the limiter its
# `post_insert` slot (with the `maximum` link); every track carries all three, in slot order, and
# no inserts.
for document in (intended, mono):
    console = document["console"]
    ids = [s["identity"]["effect_id"] for s in console["pre_insert"]]
    assert ids == ["miso.parametric-eq", "miso.compressor"]
    limiter = console["post_insert"]
    assert len(limiter) == 1 and limiter[0]["identity"]["effect_id"] == "miso.true-peak-limiter"
    assert limiter[0]["link_mode"] == "maximum"
    slots = [s["slot"] for s in console["pre_insert"] + console["post_insert"]]
    for track in document["tracks"]:
        assert [entry["slot"] for entry in track["console"]] == slots
        assert track["inserts"]["effects"] == []
assert len({t["console"][2]["params"][0]["value"] for t in intended["tracks"]}) == 64
for track in mono["tracks"]:
    assert track["left_source_channel"] == track["right_source_channel"] == 0
    assert track["builtins"]["left"] == track["builtins"]["right"]
    for entry in track["console"]:
        values = {(p["parameter_id"], p["channel"], p["unit"]): p["value"] for p in entry["params"]}
        for (parameter_id, channel, unit), value in values.items():
            if channel == "left":
                assert values[(parameter_id, "right", unit)] == value
assert sum(t["fader"]["left_db"] != t["fader"]["right_db"] for t in mono["tracks"]) == 49
assert sum(t["pan"]["left"] != t["pan"]["right"] for t in mono["tracks"]) == 50
# #1085: the app shape, in decision 12's console shape since #1093. The standing EQ and compressor
# slots are the session's `pre_insert`, with no `post_insert` limiter; each track's two entries are
# its standing entries, both bypassed on exactly the tracks whose index is 2 mod 3; no inserts.
assert app["session_id"] == "console-sixty-four-track-app"
outer = lambda d: {k: v for k, v in d.items() if k not in ("session_id", "console", "tracks")}
assert outer(app) == outer(intended) and len(app["tracks"]) == 64
assert app["console"] == {"pre_insert": intended["console"]["pre_insert"], "post_insert": []}
for index, (track, standing) in enumerate(zip(app["tracks"], intended["tracks"])):
    assert track["inserts"]["effects"] == []
    assert [dict(e, bypass=False) for e in track["console"]] == standing["console"][:2]
    assert all(e["bypass"] == (index % 3 == 2) for e in track["console"])
    rest = lambda t: {k: v for k, v in t.items() if k != "console"}
    assert rest(track) == rest(standing)
assert sum(e["bypass"] for t in app["tracks"] for e in t["console"]) == 2 * 21
# #1227: the bus-and-send shape. The intended fixture's rate, quantum, source, console and tracks;
# ten submix strips, each carrying every console slot in slot order (none bypassed) and exactly
# one insert; 64 main routes into the eight buses, a pre-fader send into `fx-a` and a post-fader
# send into `fx-b` per track, and ten returns into `main-out`; no muted route, `follows_mute`
# exactly on the routes into a submix, no VCA and no automation.
assert sends["session_id"] == "console-sixty-four-track-sends"
for key in ("sample_rate_hz", "quantum_frames", "sources", "console", "tracks"):
    assert sends[key] == intended[key], key
buses = [f"bus-{k}" for k in range(8)]
assert [s["id"] for s in sends["submixes"]] == buses + ["fx-a", "fx-b"]
console_slots = [s["slot"] for s in intended["console"]["pre_insert"] + intended["console"]["post_insert"]]
expected_insert = dict({b: ("miso.compressor", "maximum") for b in buses},
                       **{"fx-a": ("miso.delay", "dual_mono"),
                          "fx-b": ("miso.parametric-eq", "dual_mono")})
for strip in sends["submixes"]:
    assert [e["slot"] for e in strip["console"]] == console_slots
    assert not any(e["bypass"] for e in strip["console"])
    effects = strip["inserts"]["effects"]
    assert len(effects) == 1 and not effects[0]["bypass"]
    assert (effects[0]["identity"]["effect_id"], effects[0]["link_mode"]) == expected_insert[strip["id"]]
    assert not strip["fader"]["left_mute"] and not strip["fader"]["right_mute"]
routes = sends["routes"]
assert len(routes) == 202
def kinds(tap, destination):
    return sorted((r["source"].get("track_id") or r["source"]["submix_id"],
                   r["destination"].get("submix_id") or r["destination"]["output_id"])
                  for r in routes
                  if r["source"]["tap"] == tap and (r["destination"].get("submix_id")
                                                    or r["destination"]["output_id"]) in destination)
tracks = [f"ch{i:02d}" for i in range(64)]
assert kinds("post_pan", buses) == [(t, f"bus-{i // 8}") for i, t in enumerate(tracks)]
assert kinds("pre_fader", ["fx-a"]) == [(t, "fx-a") for t in tracks]
assert kinds("post_fader", ["fx-b"]) == [(t, "fx-b") for t in tracks]
assert kinds("post_pan", ["main-out"]) == sorted((s, "main-out") for s in buses + ["fx-a", "fx-b"])
assert all(r["source"]["kind"] == "track" for r in routes if r["destination"]["kind"] == "submix_input")
assert all(r["source"]["kind"] == "submix" for r in routes if r["destination"]["kind"] == "output_input")
assert not any(r["mute"] for r in routes)
assert [r["follows_mute"] for r in routes] == [r["destination"]["kind"] == "submix_input" for r in routes]
assert sum(r["follows_mute"] for r in routes) == 192
assert sends["vcas"] == [] and sends["automation"] == []
PY

printf 'console session fixtures: ok (canonical regeneration and 64-track intended/mono/app/sends witnesses)\n'
