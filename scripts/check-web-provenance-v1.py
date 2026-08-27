#!/usr/bin/env python3
"""Independent schema and sibling-byte gate for web artifact provenance (issue #207 P4)."""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import pathlib
import re
import sys
import tempfile

SCHEMA = "miso.web.provenance.v1"
SOURCE_REPOSITORY = "misofm/engine-v2"
BUILT_WITH = "scripts/build-sdk.sh"
ABI_VERSION = 0x0001_0000
BACKEND = "simd128"
OUTPUT_NAME = "miso-engine-v2.provenance.json"
ASSET_NAMES = (
    "miso-engine-v2-abi-layout.json",
    "miso-engine-v2-audio-worklet-host.d.ts",
    "miso-engine-v2-audio-worklet-host.js",
    "miso-engine-v2-audio-worklet.js",
    "miso-engine-v2-audio-worklet.simd128.wasm",
    "miso-engine-v2-parameter-metadata.json",
)
REVISION = re.compile(r"[0-9a-f]{40}\Z")
DIGEST = re.compile(r"[0-9a-f]{64}\Z")


class Invalid(Exception):
    """One provenance rule was violated."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


def asset_record(path: pathlib.Path) -> dict[str, object]:
    require(path.is_file() and not path.is_symlink(), f"required regular artifact: {path.name}")
    data = path.read_bytes()
    return {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}


def validate(document: object, directory: pathlib.Path) -> None:
    require(directory.is_dir() and not directory.is_symlink(), "artifact directory")
    require(isinstance(document, dict), "document object")
    require(
        set(document) == {
            "schema", "sourceRepository", "sourceRevision", "sourceSubject", "builtWith",
            "abiVersion", "backend", "assets",
        },
        "top-level keys",
    )
    require(document["schema"] == SCHEMA, "schema")
    require(document["sourceRepository"] == SOURCE_REPOSITORY, "source repository")
    require(isinstance(document["sourceRevision"], str) and REVISION.fullmatch(document["sourceRevision"]), "source revision")
    require(
        isinstance(document["sourceSubject"], str)
        and document["sourceSubject"]
        and "\n" not in document["sourceSubject"]
        and "\r" not in document["sourceSubject"],
        "source subject",
    )
    require(document["builtWith"] == BUILT_WITH, "build script")
    require(document["abiVersion"] == ABI_VERSION, "ABI version")
    require(document["backend"] == BACKEND, "backend")

    assets = document["assets"]
    require(isinstance(assets, dict), "assets object")
    require(OUTPUT_NAME not in assets, "provenance recursion")
    require(list(assets) == sorted(assets), "assets sorted")
    require(set(assets) == set(ASSET_NAMES), "asset membership")
    for name in ASSET_NAMES:
        record = assets[name]
        require(isinstance(record, dict) and set(record) == {"bytes", "sha256"}, f"asset record keys: {name}")
        require(isinstance(record["bytes"], int) and record["bytes"] >= 0, f"asset bytes: {name}")
        require(isinstance(record["sha256"], str) and DIGEST.fullmatch(record["sha256"]), f"asset digest syntax: {name}")
        require(record == asset_record(directory / name), f"asset digest or bytes: {name}")


def valid_document(directory: pathlib.Path) -> dict[str, object]:
    return {
        "schema": SCHEMA,
        "sourceRepository": SOURCE_REPOSITORY,
        "sourceRevision": "a" * 40,
        "sourceSubject": "Phase-0 provenance self-test",
        "builtWith": BUILT_WITH,
        "abiVersion": ABI_VERSION,
        "backend": BACKEND,
        "assets": {name: asset_record(directory / name) for name in ASSET_NAMES},
    }


def self_test() -> int:
    with tempfile.TemporaryDirectory() as raw_directory:
        directory = pathlib.Path(raw_directory)
        for index, name in enumerate(ASSET_NAMES):
            (directory / name).write_bytes(f"asset-{index}".encode("ascii"))
        document = valid_document(directory)
        validate(document, directory)
        mutations: list[tuple[str, object, bool]] = [
            ("top-level key", lambda d, p: d.update(extra=True), False),
            ("schema", lambda d, p: d.update(schema="miso.web.provenance.v2"), False),
            ("repository", lambda d, p: d.update(sourceRepository="other/repository"), False),
            ("bad revision", lambda d, p: d.update(sourceRevision="A" * 40), False),
            ("empty subject", lambda d, p: d.update(sourceSubject=""), False),
            ("multiline subject", lambda d, p: d.update(sourceSubject="line one\nline two"), False),
            ("build script", lambda d, p: d.update(builtWith="scripts/other.sh"), False),
            ("ABI version", lambda d, p: d.update(abiVersion=1), False),
            ("backend", lambda d, p: d.update(backend="scalar"), False),
            ("missing asset", lambda d, p: d["assets"].pop(ASSET_NAMES[0]), False),
            ("extra asset", lambda d, p: d["assets"].update(extra={"bytes": 0, "sha256": "0" * 64}), False),
            ("recursion attempt", lambda d, p: d["assets"].update({OUTPUT_NAME: {"bytes": 0, "sha256": "0" * 64}}), False),
            ("assets unsorted", lambda d, p: d.update(assets=dict(reversed(list(d["assets"].items())))), False),
            ("asset bytes", lambda d, p: d["assets"][ASSET_NAMES[0]].update(bytes=99), False),
            ("flipped hash", lambda d, p: d["assets"][ASSET_NAMES[0]].update(sha256="0" * 64), False),
            ("uppercase hash", lambda d, p: d["assets"][ASSET_NAMES[0]].update(sha256="A" * 64), False),
            ("asset fields", lambda d, p: d["assets"][ASSET_NAMES[0]].update(extra=True), False),
            ("changed artifact", lambda d, p: (p / ASSET_NAMES[0]).write_bytes(b"changed"), True),
        ]
        failures = 0
        for name, mutate, changes_artifact in mutations:
            candidate = copy.deepcopy(document)
            if changes_artifact:
                (directory / ASSET_NAMES[0]).write_bytes(b"asset-0")
            mutate(candidate, directory)
            try:
                validate(candidate, directory)
            except Invalid:
                continue
            except Exception:  # A malformed red mutation still proves it was not accepted.
                continue
            print(f"self-test FAILED: mutation escaped -- {name}", file=sys.stderr)
            failures += 1
        if failures == 0:
            print("web provenance schema self-test passed")
        return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("artifact_directory", nargs="?", type=pathlib.Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if args.artifact_directory is None:
        parser.error("an artifact directory is required")
    provenance = args.artifact_directory / OUTPUT_NAME
    try:
        require(provenance.is_file() and not provenance.is_symlink(), "provenance file")
        validate(json.loads(provenance.read_text(encoding="utf-8")), args.artifact_directory)
    except (Invalid, json.JSONDecodeError) as error:
        print(f"FAIL web provenance: {error}", file=sys.stderr)
        return 1
    print(f"web provenance: ok ({provenance})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
