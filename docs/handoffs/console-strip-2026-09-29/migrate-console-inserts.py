#!/usr/bin/env python3
"""One-off mechanical migration of Session V1 documents to decision 12's console/inserts shape (#1093).

A rack is *uniform* when every track carries an identical declaration sequence in it: the same
effect IDs, native identities, qualities and link modes, no sidechain, and every effect on the
console eligibility list. The slot is the effect's ID, and each track's entry keeps its own
``bypass`` and ``params``. Chain order is never changed:

* a uniform ``simd1`` becomes ``console.pre_insert`` and a uniform ``simd2`` becomes
  ``console.post_insert``;
* a uniform ``dynamic`` in a document of two or more tracks (the app's shape: EQ -> compressor on
  every track with per-track bypass) is appended to ``pre_insert`` when ``simd1`` is console or
  empty, or else prepended to ``post_insert`` when ``simd2`` is console or empty. A single track
  shows no strip shared across tracks, and the two console-versus-insert placement witnesses
  (``compressor-dynamic-bank-observation.json`` and ``console-sixty-four-track.json``) keep their
  ``dynamic`` as ``inserts``;
* every other rack folds into ``inserts`` in chain order. A folded ID collision is refused, never
  renamed, and so is a slot ID repeated across the console.

Send taps take their new spellings (codes unchanged), and automation targets move to
``console`` or ``inserts`` with their rack. Number spellings, key order and layout are
preserved byte for byte outside the migrated fields: a document that was canonical stays
canonical, which the Rust writer then proves.

Usage: migrate-console-inserts.py FILE... (rewrites in place; ``--check`` reports without writing)

This is #1093's one-off, kept here as its PR attachment (the console batch opens no per-slice pull
request) and for the SDK and app handoffs (#1097). No gate or operator path runs it; the checked-in
documents it produced are proved by the Rust parser, the canonical writer and the class-A evidence
in the #1093 spec.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

ELIGIBLE = {
    "miso.parametric-eq",
    "miso.compressor",
    "miso.gate-expander",
    "miso.soft-clip",
    "miso.transient-shaper",
    "miso.true-peak-limiter",
}
TAPS = {
    "input": "input",
    "post_input_builtins": "post_input",
    "post_simd1": "insert_send",
    "post_dynamic": "insert_return",
    "post_simd2_pre_fader": "pre_fader",
    "post_fader": "post_fader",
    "post_matrix": "post_pan",
}


class Raw(str):
    """A JSON number token kept exactly as spelled."""


def load(text: str):
    return json.loads(text, parse_float=Raw, parse_int=Raw)


def scalar(value) -> str:
    if isinstance(value, Raw):
        return str(value)
    if value is True:
        return "true"
    if value is False:
        return "false"
    if value is None:
        return "null"
    return json.dumps(value, ensure_ascii=False)


def dump_indented(value, level: int = 0) -> str:
    pad = "  "
    if isinstance(value, dict):
        if not value:
            return "{}"
        items = [f"{pad * (level + 1)}{json.dumps(k, ensure_ascii=False)}: {dump_indented(v, level + 1)}"
                 for k, v in value.items()]
        return "{\n" + ",\n".join(items) + "\n" + pad * level + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        items = [pad * (level + 1) + dump_indented(v, level + 1) for v in value]
        return "[\n" + ",\n".join(items) + "\n" + pad * level + "]"
    return scalar(value)


def dump_line(value) -> str:
    if isinstance(value, dict):
        return "{" + ", ".join(f"{json.dumps(k, ensure_ascii=False)}: {dump_line(v)}"
                               for k, v in value.items()) + "}"
    if isinstance(value, list):
        return "[" + ", ".join(dump_line(v) for v in value) + "]"
    return scalar(value)


def layout_of(text: str):
    """Return the serializer that reproduces this document's bytes, or refuse."""
    document = load(text)
    for dump, suffix in ((dump_indented, "\n"), (dump_indented, ""), (dump_line, "\n"), (dump_line, "")):
        if dump(document) + suffix == text:
            return lambda value, dump=dump, suffix=suffix: dump(value) + suffix
    raise SystemExit("document layout is not reproducible; refusing to rewrite it")


def signature(effect: dict):
    identity = effect["identity"]
    return (effect["id"], identity.get("kind"), identity.get("effect_id"), effect["quality"],
            effect["link_mode"])


def console_section(tracks: list, rack: str):
    """The slot declarations when `rack` is uniform and eligible on every track, else None."""
    if not tracks:
        return []
    first = [signature(e) for e in tracks[0][rack]["effects"]]
    for track in tracks:
        effects = track[rack]["effects"]
        if [signature(e) for e in effects] != first:
            return None
        for effect in effects:
            if effect["sidechain"] != {"kind": "none"}:
                return None
            if effect["identity"].get("kind") != "native":
                return None
            if effect["identity"].get("effect_id") not in ELIGIBLE:
                return None
    return [
        {
            "slot": effect["id"],
            "identity": effect["identity"],
            "quality": effect["quality"],
            "link_mode": effect["link_mode"],
        }
        for effect in tracks[0][rack]["effects"]
    ]


def rename_taps(value):
    if isinstance(value, dict):
        for key, item in value.items():
            if key == "tap" and isinstance(item, str):
                if item not in TAPS:
                    raise SystemExit(f"unknown tap token {item!r}")
                value[key] = TAPS[item]
            else:
                rename_taps(item)
    elif isinstance(value, list):
        for item in value:
            rename_taps(item)


# The console-versus-insert placement witnesses (#1093's named exceptions): a uniform `dynamic`
# stays an insert here so a placement change can be proved not to move a rendered bit.
PLACEMENT_WITNESSES = {"compressor-dynamic-bank-observation.json", "console-sixty-four-track.json"}


def migrate(document: dict, name: str = "") -> tuple[dict, str]:
    tracks = document["tracks"]
    uniform = {rack: console_section(tracks, rack) for rack in ("simd1", "dynamic", "simd2")}
    # Where each rack goes, in chain order: "pre", "post" or "inserts".
    place = {
        "simd1": "pre" if uniform["simd1"] is not None else "inserts",
        "simd2": "post" if uniform["simd2"] is not None else "inserts",
    }
    movable = (
        bool(uniform["dynamic"]) and len(tracks) >= 2 and name not in PLACEMENT_WITNESSES
    )
    if movable and place["simd1"] == "pre":
        place["dynamic"] = "pre"
    elif movable and place["simd2"] == "post":
        place["dynamic"] = "post"
    else:
        place["dynamic"] = "inserts"
    order = ("simd1", "dynamic", "simd2")
    pre = [slot for rack in order if place[rack] == "pre" for slot in uniform[rack]]
    post = [slot for rack in order if place[rack] == "post" for slot in uniform[rack]]
    slots = [slot["slot"] for slot in pre + post]
    if len(slots) != len(set(slots)):
        raise SystemExit(f"console slot IDs collide: {slots}")
    for track in tracks:
        inserts = [e for rack in order if place[rack] == "inserts" for e in track[rack]["effects"]]
        ids = [effect["id"] for effect in inserts]
        if len(ids) != len(set(ids)):
            raise SystemExit(f"track {track['id']}: folded insert IDs collide: {ids}")
        console = [
            {"slot": effect["id"], "bypass": effect["bypass"], "params": effect["params"]}
            for section in ("pre", "post")
            for rack in order
            if place[rack] == section
            for effect in track[rack]["effects"]
        ]
        rebuilt = {}
        for key, value in track.items():
            if key == "simd1":
                rebuilt["console"] = console
                rebuilt["inserts"] = {"effects": inserts}
            elif key in ("dynamic", "simd2"):
                continue
            else:
                rebuilt[key] = value
        track.clear()
        track.update(rebuilt)
    for automation in document["automation"]:
        target = automation["target"]
        rack = target["rack"]
        if rack in place:
            target["rack"] = "inserts" if place[rack] == "inserts" else "console"
    rename_taps(document["routes"])
    rename_taps(document["tracks"])
    rebuilt = {}
    for key, value in document.items():
        rebuilt[key] = value
        if key == "sources":
            rebuilt["console"] = {"pre_insert": pre, "post_insert": post}
    shape = " ".join(f"{rack}->{place[rack]}" for rack in order)
    shape += f" pre_insert={[s['slot'] for s in pre]} post_insert={[s['slot'] for s in post]}"
    return rebuilt, shape


def main(arguments: list[str]) -> int:
    check = arguments[:1] == ["--check"]
    paths = arguments[1:] if check else arguments
    if not paths:
        raise SystemExit(__doc__)
    for path in map(Path, paths):
        text = path.read_text()
        dump = layout_of(text)
        migrated, shape = migrate(load(text), path.name)
        print(f"{path}: {shape}")
        if not check:
            path.write_text(dump(migrated))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
