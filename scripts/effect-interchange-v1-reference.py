#!/usr/bin/env python3
"""Import-safe Issue 081 aggregator for the three independent V1 references."""

from __future__ import annotations

import argparse
import contextlib
import hashlib
import importlib.util
import io
import json
import pathlib
import sys


ROOT = pathlib.Path(__file__).resolve().parent.parent
MANIFESTS = {
    "descriptor": ROOT / "fixtures/effect-descriptor/v1/MANIFEST.sha256",
    "package": ROOT / "fixtures/effect-package/v1/MANIFEST.sha256",
    "state": ROOT / "fixtures/effect-state/v1/MANIFEST.sha256",
}


def _load(name: str, filename: str):
    path = ROOT / "scripts" / filename
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot import {filename}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


def _require_exact_files(directory: pathlib.Path, expected: dict[str, object]) -> None:
    actual = sorted(path.name for path in directory.iterdir() if path.is_file())
    if actual != sorted(expected):
        raise AssertionError(f"fixture membership mismatch: {directory.relative_to(ROOT)}")


def run_references() -> None:
    descriptor = _load("miso_effect_descriptor_reference_v1", "effect-descriptor-v1-reference.py")
    package = _load("miso_effect_package_reference_v1", "effect-package-v1-reference.py")
    state = _load("miso_effect_state_reference_v1", "effect-state-v1-reference.py")
    descriptor_names = {
        "MANIFEST.sha256": None,
        "comprehensive-a.identity.hex": None,
        "comprehensive-a.json": None,
        "comprehensive-a.wire.hex": None,
        "comprehensive-b.identity.hex": None,
        "comprehensive-b.json": None,
        "comprehensive-b.wire.hex": None,
        "comprehensive-c.identity.hex": None,
        "comprehensive-c.json": None,
        "comprehensive-c.wire.hex": None,
    }
    _require_exact_files(ROOT / "fixtures/effect-descriptor/v1", descriptor_names)
    with contextlib.redirect_stdout(io.StringIO()):
        descriptor.check(ROOT)
    package_expected = package.expected_files()
    _require_exact_files(package.FIXTURES, package_expected)
    for name, expected in package_expected.items():
        if (package.FIXTURES / name).read_text(encoding="utf-8") != expected:
            raise AssertionError(f"package reference mismatch: {name}")
    state_expected = state.expected_files()
    _require_exact_files(state.FIXTURES, state_expected)
    for name, expected in state_expected.items():
        if (state.FIXTURES / name).read_bytes() != expected:
            raise AssertionError(f"state reference mismatch: {name}")


def manifest_hashes() -> tuple[dict[str, str], str]:
    raw = {name: path.read_bytes() for name, path in MANIFESTS.items()}
    hashes = {name: hashlib.sha256(value).hexdigest() for name, value in raw.items()}
    combined = hashlib.sha256()
    for name in ("descriptor", "package", "state"):
        value = raw[name]
        combined.update(len(value).to_bytes(8, "little"))
        combined.update(value)
    return hashes, combined.hexdigest()


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(allow_abbrev=False)
    parser.add_argument("--process-index", type=int, required=True)
    args = parser.parse_args(argv)
    if not 0 <= args.process_index < 100:
        parser.error("--process-index must be in 0..99")
    run_references()
    hashes, combined = manifest_hashes()
    record = {
        "combined_sha256": combined,
        "descriptor_manifest_sha256": hashes["descriptor"],
        "issue": 81,
        "package_manifest_sha256": hashes["package"],
        "process_index": args.process_index,
        "schema_version": 1,
        "state_manifest_sha256": hashes["state"],
    }
    print(json.dumps(record, sort_keys=True, separators=(",", ":")))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
