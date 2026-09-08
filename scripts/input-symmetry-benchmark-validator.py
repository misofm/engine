#!/usr/bin/env python3
"""Strict, untimed validator for the two Issue #600 measurement records."""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import math
import pathlib
import tempfile
from typing import Any


HEX40 = set("0123456789abcdef")
HEX64 = HEX40
EXPECTED = {
    "schema_version", "issue", "record", "round", "sample_rate_hz", "quantum_frames",
    "lane_width", "track_count", "records_per_block", "target_pair_db", "smoothing_samples",
    "warmup_blocks", "measured_blocks", "attempted_records", "accepted_records",
    "rendered_blocks", "render_errors", "output_words", "output_sha256", "owner_digests",
    "owner_attempted_records", "owner_accepted_records", "owner_rendered_blocks", "elapsed_ns",
    "nanoseconds_per_block", "backend", "source_commit", "source_tree", "binary_sha256",
    "fixture_sha256", "fixture_id", "argv", "cwd", "rust_version", "compiler",
    "target_triple", "build_flags", "cpu", "os", "metadata_missing", "descriptive_only",
}
IDENTITY = ("source_commit", "source_tree", "binary_sha256", "fixture_sha256")
MACHINE = ("cpu", "os")


def integer(value: Any) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def hex_value(value: Any, length: int) -> bool:
    return isinstance(value, str) and len(value) == length and set(value) <= HEX40


def reject_nonfinite(value: Any) -> None:
    if isinstance(value, float) and not math.isfinite(value):
        raise ValueError("nonfinite number")
    if isinstance(value, dict):
        for item in value.values():
            reject_nonfinite(item)
    elif isinstance(value, list):
        for item in value:
            reject_nonfinite(item)


def load_expected(path: pathlib.Path | None) -> dict[str, Any] | None:
    if path is None:
        return None
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError("identity seal is not an object")
    required = {"candidate_commit", "candidate_tree", "binary_sha256", "fixture_sha256", "source_sha256", "validator_sha256"}
    if set(value) < required:
        raise ValueError("identity seal is incomplete")
    return value


def validate_record(value: Any, expected: dict[str, Any] | None) -> dict[str, Any]:
    if not isinstance(value, dict) or set(value) != EXPECTED:
        raise ValueError("record key set")
    reject_nonfinite(value)
    if value["schema_version"] != 1 or value["issue"] != 600 or value["record"] != "input_symmetry":
        raise ValueError("schema or issue")
    if not integer(value["round"]) or value["round"] not in (1, 2):
        raise ValueError("round")
    frozen = {
        "sample_rate_hz": 48_000, "quantum_frames": 128, "lane_width": 8,
        "track_count": 8, "records_per_block": 8, "smoothing_samples": 256,
        "warmup_blocks": 512, "measured_blocks": 4096,
    }
    for key, expected_value in frozen.items():
        if value[key] != expected_value:
            raise ValueError(f"frozen field: {key}")
    if value["target_pair_db"] != [-6.0, -12.0]:
        raise ValueError("target pair")
    for key in ("attempted_records", "accepted_records", "rendered_blocks", "render_errors", "output_words", "elapsed_ns", "nanoseconds_per_block"):
        if not integer(value[key]) or value[key] < 0:
            raise ValueError(f"counter: {key}")
    if value["attempted_records"] != 4096 * 8 or value["accepted_records"] != value["attempted_records"]:
        raise ValueError("record traffic count")
    if value["rendered_blocks"] != 4096 or value["render_errors"] != 0:
        raise ValueError("render count")
    if value["output_words"] != 4096 * 128 * 2:
        raise ValueError("output word count")
    if value["elapsed_ns"] <= 0 or value["nanoseconds_per_block"] <= 0:
        raise ValueError("timing")
    if value["nanoseconds_per_block"] != max(1, value["elapsed_ns"] // 4096):
        raise ValueError("inconsistent nanoseconds per block")
    for key in ("output_sha256", *IDENTITY):
        if not hex_value(value[key], 64 if key.endswith("sha256") else 40):
            raise ValueError(f"digest: {key}")
    if not isinstance(value["owner_digests"], list) or value["owner_digests"] != [value["output_sha256"]] * 2:
        raise ValueError("independent owner digests")
    for key, expected_value in (("owner_attempted_records", [32768, 32768]), ("owner_accepted_records", [32768, 32768]), ("owner_rendered_blocks", [4096, 4096])):
        if value[key] != expected_value:
            raise ValueError(f"independent owner counters: {key}")
    for key in ("backend", "source_commit", "source_tree", "binary_sha256", "fixture_sha256", "fixture_id", "argv", "cwd", "rust_version", "compiler", "target_triple", "build_flags", "cpu", "os"):
        if not isinstance(value[key], str) or not value[key]:
            raise ValueError(f"metadata: {key}")
    if value["fixture_id"] != "fixtures/session/v1/parametric-eq-bank-console.json":
        raise ValueError("fixture identity")
    if not isinstance(value["metadata_missing"], list) or value["metadata_missing"] != sorted(set(value["metadata_missing"])):
        raise ValueError("metadata missing list")
    if any(not isinstance(item, str) for item in value["metadata_missing"]):
        raise ValueError("metadata missing item")
    if value["descriptive_only"] is not True:
        raise ValueError("descriptive-only flag")
    if expected is not None:
        pairs = {
            "source_commit": expected["candidate_commit"], "source_tree": expected["candidate_tree"],
            "binary_sha256": expected["binary_sha256"], "fixture_sha256": expected["fixture_sha256"],
        }
        for key, expected_value in pairs.items():
            if value[key] != expected_value:
                raise ValueError(f"identity mismatch: {key}")
    return value


def validate(records_path: pathlib.Path, expected_path: pathlib.Path | None = None) -> None:
    expected = load_expected(expected_path)
    lines = records_path.read_text(encoding="utf-8").splitlines()
    if len(lines) != 2 or any(not line for line in lines):
        raise ValueError("exactly two records required")
    records = [validate_record(json.loads(line), expected) for line in lines]
    if [record["round"] for record in records] != [1, 2]:
        raise ValueError("round completeness/order")
    for key in (*IDENTITY, "backend", "fixture_id", "argv", "cwd", "rust_version", "compiler", "target_triple", "build_flags", *MACHINE, "metadata_missing"):
        if records[0][key] != records[1][key]:
            raise ValueError(f"inconsistent shared field: {key}")


def self_test() -> None:
    identity = {
        "candidate_commit": "a" * 40, "candidate_tree": "b" * 40,
        "binary_sha256": "c" * 64, "fixture_sha256": "d" * 64,
        "source_sha256": "e" * 64, "validator_sha256": "f" * 64,
    }
    base = {
        "schema_version": 1, "issue": 600, "record": "input_symmetry", "sample_rate_hz": 48_000,
        "quantum_frames": 128, "lane_width": 8, "track_count": 8, "records_per_block": 8,
        "target_pair_db": [-6.0, -12.0], "smoothing_samples": 256, "warmup_blocks": 512,
        "measured_blocks": 4096, "attempted_records": 32768, "accepted_records": 32768,
        "rendered_blocks": 4096, "render_errors": 0, "output_words": 1_048_576, "output_sha256": "1" * 64,
        "owner_digests": ["1" * 64, "1" * 64], "owner_attempted_records": [32768, 32768],
        "owner_accepted_records": [32768, 32768], "owner_rendered_blocks": [4096, 4096],
        "elapsed_ns": 409600, "nanoseconds_per_block": 100, "backend": "Simd8",
        "source_commit": identity["candidate_commit"], "source_tree": identity["candidate_tree"],
        "binary_sha256": identity["binary_sha256"], "fixture_sha256": identity["fixture_sha256"],
        "fixture_id": "fixtures/session/v1/parametric-eq-bank-console.json", "argv": "bench input-symmetry",
        "cwd": "/synthetic", "rust_version": "rustc synthetic", "compiler": "rustc synthetic",
        "target_triple": "x86_64-unknown-linux-gnu", "build_flags": "avx2,fma", "cpu": "unknown", "os": "linux",
        "metadata_missing": [], "descriptive_only": True,
    }
    with tempfile.TemporaryDirectory() as directory:
        root = pathlib.Path(directory)
        expected_path = root / "expected.json"
        records_path = root / "records.jsonl"
        expected_path.write_text(json.dumps(identity), encoding="utf-8")
        records = [dict(base, round=round_) for round_ in (1, 2)]

        def write(values: list[dict[str, Any]]) -> None:
            records_path.write_text("".join(json.dumps(item, sort_keys=True, allow_nan=True) + "\n" for item in values), encoding="utf-8")

        write(records)
        validate(records_path, expected_path)
        mutations: list[list[dict[str, Any]]] = []
        value = copy.deepcopy(records); value.pop(); mutations.append(value)
        value = copy.deepcopy(records); value.reverse(); mutations.append(value)
        value = copy.deepcopy(records); value[0]["accepted_records"] = 0; mutations.append(value)
        value = copy.deepcopy(records); value[0]["output_sha256"] = "f" * 64; mutations.append(value)
        value = copy.deepcopy(records); value[0]["source_tree"] = "0" * 40; mutations.append(value)
        value = copy.deepcopy(records); value[0]["mutation"] = True; mutations.append(value)
        value = copy.deepcopy(records); value[0]["elapsed_ns"] = float("nan"); mutations.append(value)
        for mutation in mutations:
            write(mutation)
            try:
                validate(records_path, expected_path)
            except (ValueError, json.JSONDecodeError):
                continue
            raise AssertionError("validator self-test mutation escaped")
    print("input-symmetry validator self-test passed (7 mutations caught)")


def main() -> int:
    parser = argparse.ArgumentParser(allow_abbrev=False)
    parser.add_argument("records", nargs="?", type=pathlib.Path)
    parser.add_argument("expected", nargs="?", type=pathlib.Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        if args.records is not None or args.expected is not None:
            parser.error("--self-test takes no paths")
        self_test()
        return 0
    if args.records is None:
        parser.error("records path is required")
    validate(args.records, args.expected)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
