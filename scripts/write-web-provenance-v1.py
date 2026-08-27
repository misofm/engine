#!/usr/bin/env python3
"""Write deterministic provenance for the six Phase-0 web artifacts (issue #207 P4)."""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import re
import sys

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


def fail(message: str) -> None:
    print(f"provenance writer: {message}", file=sys.stderr)
    raise SystemExit(2)


def asset_record(path: pathlib.Path) -> dict[str, object]:
    if not path.is_file() or path.is_symlink():
        fail(f"required regular artifact missing: {path.name}")
    data = path.read_bytes()
    return {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}


def write(output_dir: pathlib.Path, source_revision: str, source_subject: str) -> pathlib.Path:
    if not output_dir.is_dir() or output_dir.is_symlink():
        fail("output must be an existing non-symlink directory")
    if not REVISION.fullmatch(source_revision):
        fail("source revision must be lowercase 40-hex")
    if not source_subject or "\n" in source_subject or "\r" in source_subject:
        fail("source subject must be one nonempty line")
    output = output_dir / OUTPUT_NAME
    if output.exists() or output.is_symlink():
        fail("refusing to overwrite provenance")
    assets = {name: asset_record(output_dir / name) for name in ASSET_NAMES}
    document = {
        "abiVersion": ABI_VERSION,
        "assets": assets,
        "backend": BACKEND,
        "builtWith": BUILT_WITH,
        "schema": SCHEMA,
        "sourceRepository": SOURCE_REPOSITORY,
        "sourceRevision": source_revision,
        "sourceSubject": source_subject,
    }
    output.write_text(
        json.dumps(document, ensure_ascii=True, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    return output


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("output_directory", type=pathlib.Path)
    parser.add_argument("--source-revision", required=True)
    parser.add_argument("--source-subject", required=True)
    args = parser.parse_args()
    output = write(args.output_directory, args.source_revision, args.source_subject)
    print(f"wrote {output}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
