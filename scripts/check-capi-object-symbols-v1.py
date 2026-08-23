#!/usr/bin/env python3
"""Classify nm definitions separately from imports and enforce exact C ABI V1 exports."""

from __future__ import annotations

import pathlib
import sys


PREFIX = "miso_engine_v2_"
UNDEFINED_TYPES = {"U", "u", "w", "v"}


def symbols(path: pathlib.Path) -> tuple[set[str], set[str]]:
    defined: set[str] = set()
    imported: set[str] = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        fields = line.split()
        if len(fields) < 2 or not fields[-1].startswith(PREFIX):
            continue
        kind = fields[-2]
        if kind in UNDEFINED_TYPES:
            imported.add(fields[-1])
        else:
            defined.add(fields[-1])
    return defined, imported


def main() -> int:
    if len(sys.argv) != 4:
        return 2
    expected = set(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8").splitlines())
    if len(expected) != 14:
        return 1
    for dump_name in sys.argv[2:]:
        defined, imported = symbols(pathlib.Path(dump_name))
        if defined != expected or imported:
            return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
