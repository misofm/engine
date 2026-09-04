#!/usr/bin/env python3
"""Reject retired delivery-codec Cargo identities from the live workspace."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import tomllib
from pathlib import Path


RETIRED = frozenset({"flac-decoder", "stem-publisher", "catalog-migrate", "flacenc", "symphonia"})
RETIRED_DIRECTORIES = {
    Path("sidecars/flac-decoder"): "flac-decoder",
    Path("tools/stem-publisher"): "stem-publisher",
    Path("tools/catalog-migrate"): "catalog-migrate",
}
LOCK_PACKAGE = re.compile(r'^name = "([A-Za-z0-9_-]+)"$', re.MULTILINE)


def fail(message: str) -> None:
    raise SystemExit(f"delivery-codec boundary: {message}")


def manifest_identities(value: object) -> set[str]:
    if isinstance(value, dict):
        found = {key for key in value if key in RETIRED}
        package = value.get("package")
        if isinstance(package, str) and package in RETIRED:
            found.add(package)
        for child in value.values():
            found |= manifest_identities(child)
        return found
    if isinstance(value, list):
        return set().union(*(manifest_identities(child) for child in value))
    if isinstance(value, str):
        return {value, Path(value).name} & RETIRED
    return set()


def check_text_files(root: Path) -> None:
    for relative, identity in RETIRED_DIRECTORIES.items():
        if (root / relative).exists():
            fail(f"retired delivery directory remains: {relative} ({identity})")
    for manifest in sorted(root.rglob("Cargo.toml")):
        bad = manifest_identities(tomllib.loads(manifest.read_text(encoding="utf-8")))
        if bad:
            fail(f"retired Cargo identity in {manifest.relative_to(root)}: {', '.join(sorted(bad))}")
    lock = root / "Cargo.lock"
    if not lock.is_file():
        fail("Cargo.lock is missing")
    bad = set(LOCK_PACKAGE.findall(lock.read_text(encoding="utf-8"))) & RETIRED
    if bad:
        fail(f"retired Cargo identity in Cargo.lock: {', '.join(sorted(bad))}")


def check_locked_metadata(root: Path) -> None:
    result = subprocess.run(
        ["cargo", "metadata", "--locked", "--format-version=1"],
        cwd=root,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        fail(f"locked cargo metadata failed: {result.stderr.strip()}")
    metadata = json.loads(result.stdout)
    names = {package.get("name") for package in metadata.get("packages", [])}
    bad = names & RETIRED
    if bad:
        fail(f"retired Cargo identity in locked metadata: {', '.join(sorted(bad))}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parent.parent)
    args = parser.parse_args()
    root = args.root.resolve()
    check_text_files(root)
    check_locked_metadata(root)
    print("delivery-codec Cargo boundary: ok")


if __name__ == "__main__":
    main()
