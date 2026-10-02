#!/usr/bin/env python3
"""One-off mechanical migration of saved Session V1 documents to K1's submix strips (#1205).

Batch K1 (owner decision 13) gave every submix a full strip and retired the ``submix_output``
source. A document saved before it is refused twice: a bare ``{"id": ..}`` submix (its
``builtins``, ``console``, ``inserts``, ``fader`` and ``pan``/``matrix`` are now required), and a
``submix_output`` route or sidechain source (``schema.invalid_enum``). This rewrites both:

* every bare submix gains the transparent strip, the engine's ``Submix::unity``: the identity input
  section on both lanes, one ``{"slot", "bypass": true, "params": []}`` entry per declared console
  slot in slot order (``pre_insert``, then ``post_insert``), no inserts, a 0 dB unmuted fader and
  the identity ``matrix`` with zero smoothing. Its sound is unchanged apart from a ``-0.0`` sample
  becoming ``+0.0`` at the identity input section; the latency of the session's latent console
  slots is paid on the bus as on every strip;
* every ``submix_output`` source, on a route or on a routed sidechain, becomes
  ``{"kind": "submix", "submix_id": .., "tap": "post_pan"}``, which is the same node.

A submix that already carries some strip keys but not all is refused, never completed. Number
spellings, key order and layout are preserved byte for byte outside the migrated fields: a
document that was canonical stays canonical, which ``session_validator validate --canonical``
then proves.

Usage: migrate-submix-strips.py FILE... (rewrites in place; ``--check`` reports without writing)

This is #1205's one-off, on the ``migrate-console-inserts.py`` precedent, kept for the app handoff
(``APP-SDK.md`` beside it). No gate or operator path runs it.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

STRIP_KEYS = ("builtins", "console", "inserts", "fader")


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


def transparent_strip(submix_id, console: dict) -> dict:
    """`Submix::unity`, spelled as the canonical writer spells it."""
    lane = {
        "polarity_invert": False,
        "trim_db": Raw("0.0"),
        "hpf_hz": Raw("0.0"),
        "lpf_hz": Raw("0.0"),
        "delay_samples": Raw("0"),
    }
    slots = [slot["slot"] for section in ("pre_insert", "post_insert") for slot in console[section]]
    return {
        "id": submix_id,
        "builtins": {"left": dict(lane), "right": dict(lane)},
        "console": [{"slot": slot, "bypass": True, "params": []} for slot in slots],
        "inserts": {"effects": []},
        "fader": {
            "left_db": Raw("0.0"),
            "right_db": Raw("0.0"),
            "left_mute": False,
            "right_mute": False,
        },
        "matrix": {
            "ll": Raw("1.0"),
            "lr": Raw("0.0"),
            "rl": Raw("0.0"),
            "rr": Raw("1.0"),
            "smoothing_samples": Raw("0"),
        },
    }


def retag_sources(value, renamed: list) -> None:
    """Rewrite every `submix_output` source in place, keeping each record's key position."""
    if isinstance(value, dict):
        if value.get("kind") == "submix_output":
            if set(value) != {"kind", "submix_id"}:
                raise SystemExit(f"unexpected submix_output source keys: {sorted(value)}")
            submix_id = value["submix_id"]
            value.clear()
            value.update({"kind": "submix", "submix_id": submix_id, "tap": "post_pan"})
            renamed.append(submix_id)
            return
        for item in value.values():
            retag_sources(item, renamed)
    elif isinstance(value, list):
        for item in value:
            retag_sources(item, renamed)


def migrate(document: dict) -> tuple[dict, str]:
    console = document.get("console", {"pre_insert": [], "post_insert": []})
    stripped = []
    for index, submix in enumerate(document["submixes"]):
        keys = set(submix)
        if keys == {"id"}:
            document["submixes"][index] = transparent_strip(submix["id"], console)
            stripped.append(submix["id"])
        elif not keys >= {"id", *STRIP_KEYS} or not ({"pan", "matrix"} & keys):
            raise SystemExit(f"submix {submix.get('id')!r} carries a partial strip: {sorted(keys)}")
    renamed: list = []
    retag_sources(document["routes"], renamed)
    retag_sources(document["tracks"], renamed)
    retag_sources(document["submixes"], renamed)
    return document, f"transparent strips {stripped}; submix_output -> submix post_pan {renamed}"


def main(arguments: list[str]) -> int:
    check = arguments[:1] == ["--check"]
    paths = arguments[1:] if check else arguments
    if not paths:
        raise SystemExit(__doc__)
    for path in map(Path, paths):
        text = path.read_text()
        dump = layout_of(text)
        migrated, shape = migrate(load(text))
        print(f"{path}: {shape}")
        if not check:
            path.write_text(dump(migrated))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
