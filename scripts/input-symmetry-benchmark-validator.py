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
    "owner_attempted_records", "owner_accepted_records", "owner_rendered_blocks", "output_witness_sha256",
    "traffic_witness_sha256", "elapsed_ns",
    "nanoseconds_per_block", "backend", "source_commit", "source_tree", "binary_sha256",
    "fixture_sha256", "fixture_id", "argv", "cwd", "rust_version", "compiler",
    "target_triple", "build_flags", "cpu", "os", "metadata_missing", "descriptive_only",
}
IDENTITY = ("source_commit", "source_tree", "binary_sha256", "fixture_sha256")
MACHINE = ("cpu", "os")
SEAL_KEYS = {
    "schema_version", "issue", "kind", "status", "candidate_commit", "candidate_tree", "binary_sha256",
    "fixture_sha256", "source_sha256", "validator_sha256", "runner_sha256", "preflight_sha256",
    "cargo_lock_sha256", "target_triple", "rust_version", "compiler", "build_flags", "cwd", "argv",
    "warmup_blocks", "measured_blocks", "records_required", "preflight_invocations", "runner_invocations",
    "workload_invocations", "timed_benchmark_invocations",
}


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
    def no_duplicates(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        value: dict[str, Any] = {}
        for key, item in pairs:
            if key in value:
                raise ValueError("duplicate JSON key")
            value[key] = item
        return value
    value = json.loads(path.read_text(encoding="utf-8"), object_pairs_hook=no_duplicates)
    if not isinstance(value, dict):
        raise ValueError("identity seal is not an object")
    if set(value) != SEAL_KEYS:
        raise ValueError("identity seal is incomplete")
    if value.get("schema_version") != 1 or value.get("issue") != 600 or value.get("kind") != "input_symmetry_benchmark_preflight" or value.get("status") != "READY":
        raise ValueError("identity seal header")
    for key in ("candidate_commit", "candidate_tree"):
        if not hex_value(value[key], 40):
            raise ValueError(f"identity seal digest: {key}")
    for key in ("binary_sha256", "fixture_sha256", "source_sha256", "validator_sha256", "runner_sha256", "preflight_sha256", "cargo_lock_sha256"):
        if not hex_value(value[key], 64):
            raise ValueError(f"identity seal digest: {key}")
    for key in ("target_triple", "rust_version", "compiler", "build_flags", "cwd", "argv"):
        if type(value[key]) is not str or not value[key]:
            raise ValueError(f"identity seal metadata: {key}")
    frozen = {
        "warmup_blocks": 512, "measured_blocks": 4096, "records_required": 2,
        "preflight_invocations": 1, "runner_invocations": 0,
        "workload_invocations": 0, "timed_benchmark_invocations": 0,
    }
    for key, expected in frozen.items():
        if type(value[key]) is not int or value[key] != expected:
            raise ValueError(f"identity seal field: {key}")
    return value


def validate_record(value: Any, expected: dict[str, Any] | None) -> dict[str, Any]:
    if not isinstance(value, dict) or set(value) != EXPECTED:
        raise ValueError("record key set")
    reject_nonfinite(value)
    if (type(value["schema_version"]) is not int or type(value["issue"]) is not int or
            value["schema_version"] != 1 or value["issue"] != 600 or value["record"] != "input_symmetry"):
        raise ValueError("schema or issue")
    if type(value["round"]) is not int or value["round"] not in (1, 2):
        raise ValueError("round")
    frozen = {
        "sample_rate_hz": 48_000, "quantum_frames": 128, "lane_width": 8,
        "track_count": 8, "records_per_block": 8, "smoothing_samples": 256,
        "warmup_blocks": 512, "measured_blocks": 4096,
    }
    for key, expected_value in frozen.items():
        if type(value[key]) is not int or value[key] != expected_value:
            raise ValueError(f"frozen field: {key}")
    if (not isinstance(value["target_pair_db"], list) or len(value["target_pair_db"]) != 2 or
            any(type(item) is not float for item in value["target_pair_db"]) or
            value["target_pair_db"] != [-6.0, -12.0]):
        raise ValueError("target pair")
    for key in ("attempted_records", "accepted_records", "rendered_blocks", "render_errors", "output_words", "elapsed_ns", "nanoseconds_per_block"):
        if type(value[key]) is not int or value[key] < 0:
            raise ValueError(f"counter: {key}")
    if value["attempted_records"] != 4096 * 8 or value["accepted_records"] != value["attempted_records"]:
        raise ValueError("record traffic count")
    if value["rendered_blocks"] != 4096 or value["render_errors"] != 0:
        raise ValueError("render count")
    if value["output_words"] != 4096 * 128 * 2:
        raise ValueError("output word count")
    if value["elapsed_ns"] <= 0 or value["nanoseconds_per_block"] <= 0:
        raise ValueError("timing")
    if value["nanoseconds_per_block"] != value["elapsed_ns"] // 4096:
        raise ValueError("inconsistent nanoseconds per block")
    for key in ("output_sha256", *IDENTITY):
        if not hex_value(value[key], 64 if key.endswith("sha256") else 40):
            raise ValueError(f"digest: {key}")
    if (not isinstance(value["owner_digests"], list) or len(value["owner_digests"]) != 2 or
            value["owner_digests"] != [value["output_sha256"]] * 2):
        raise ValueError("independent owner digests")
    for key, expected_value in (("owner_attempted_records", [32768, 32768]), ("owner_accepted_records", [32768, 32768]), ("owner_rendered_blocks", [4096, 4096])):
        if (not isinstance(value[key], list) or len(value[key]) != 2 or
                any(type(item) is not int for item in value[key]) or value[key] != expected_value):
            raise ValueError(f"independent owner counters: {key}")
    for key in ("backend", "source_commit", "source_tree", "binary_sha256", "fixture_sha256", "fixture_id", "argv", "cwd", "rust_version", "compiler", "target_triple", "build_flags", "cpu", "os"):
        if not isinstance(value[key], str) or not value[key]:
            raise ValueError(f"metadata: {key}")
    if value["fixture_id"] != "fixtures/session/v1/parametric-eq-bank-console.json":
        raise ValueError("fixture identity")
    if (not isinstance(value["metadata_missing"], list) or
            value["metadata_missing"] != sorted(set(value["metadata_missing"])) or
            any(item not in MACHINE for item in value["metadata_missing"])):
        raise ValueError("metadata missing list")
    if any(not isinstance(item, str) for item in value["metadata_missing"]):
        raise ValueError("metadata missing item")
    if value["descriptive_only"] is not True:
        raise ValueError("descriptive-only flag")
    if any((key in value["metadata_missing"]) != (value[key] == "unknown") for key in MACHINE):
        raise ValueError("dishonest missing metadata")
    expected_output_witness = hashlib.sha256(
        f"issue600-output-v1|{value['owner_digests'][0]}|{value['owner_digests'][1]}|{value['round']}".encode()
    ).hexdigest()
    expected_traffic_witness = hashlib.sha256(
        f"issue600-traffic-v1|{value['round']}|48000|128|8|8|256|512|4096|{value['attempted_records']}|{value['accepted_records']}|{value['rendered_blocks']}|{value['render_errors']}|{value['output_words']}".encode()
    ).hexdigest()
    if (value["output_witness_sha256"] != expected_output_witness or
            value["traffic_witness_sha256"] != expected_traffic_witness):
        raise ValueError("independent frozen witness")
    if expected is not None:
        pairs = {
            "source_commit": expected["candidate_commit"], "source_tree": expected["candidate_tree"],
            "binary_sha256": expected["binary_sha256"], "fixture_sha256": expected["fixture_sha256"],
        }
        for key, expected_value in pairs.items():
            if value[key] != expected_value:
                raise ValueError(f"identity mismatch: {key}")
        for key in ("target_triple", "rust_version", "compiler", "build_flags", "cwd", "argv"):
            if value[key] != expected[key]:
                raise ValueError(f"metadata mismatch: {key}")
    return value


def validate(records_path: pathlib.Path, expected_path: pathlib.Path | None = None) -> None:
    expected = load_expected(expected_path)
    lines = records_path.read_text(encoding="utf-8").splitlines()
    if len(lines) != 2 or any(not line for line in lines):
        raise ValueError("exactly two records required")
    def no_duplicates(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        value: dict[str, Any] = {}
        for key, item in pairs:
            if key in value:
                raise ValueError("duplicate JSON key")
            value[key] = item
        return value
    records = [validate_record(json.loads(line, object_pairs_hook=no_duplicates), expected) for line in lines]
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
        "runner_sha256": "0" * 64, "preflight_sha256": "1" * 64,
        "cargo_lock_sha256": "2" * 64, "target_triple": "x86_64-unknown-linux-gnu",
        "rust_version": "rustc synthetic", "compiler": "rustc synthetic",
        "build_flags": "avx2,fma", "cwd": "/synthetic", "argv": "bench input-symmetry",
        "warmup_blocks": 512, "measured_blocks": 4096, "records_required": 2,
        "preflight_invocations": 1, "runner_invocations": 0, "workload_invocations": 0,
        "timed_benchmark_invocations": 0,
    }
    base = {
        "schema_version": 1, "issue": 600, "record": "input_symmetry", "sample_rate_hz": 48_000,
        "quantum_frames": 128, "lane_width": 8, "track_count": 8, "records_per_block": 8,
        "target_pair_db": [-6.0, -12.0], "smoothing_samples": 256, "warmup_blocks": 512,
        "measured_blocks": 4096, "attempted_records": 32768, "accepted_records": 32768,
        "rendered_blocks": 4096, "render_errors": 0, "output_words": 1_048_576, "output_sha256": "1" * 64,
        "owner_digests": ["1" * 64, "1" * 64], "owner_attempted_records": [32768, 32768],
        "owner_accepted_records": [32768, 32768], "owner_rendered_blocks": [4096, 4096],
        "output_witness_sha256": "", "traffic_witness_sha256": "",
        "elapsed_ns": 409600, "nanoseconds_per_block": 100, "backend": "Simd8",
        "source_commit": identity["candidate_commit"], "source_tree": identity["candidate_tree"],
        "binary_sha256": identity["binary_sha256"], "fixture_sha256": identity["fixture_sha256"],
        "fixture_id": "fixtures/session/v1/parametric-eq-bank-console.json", "argv": "bench input-symmetry",
        "cwd": "/synthetic", "rust_version": "rustc synthetic", "compiler": "rustc synthetic",
        "target_triple": "x86_64-unknown-linux-gnu", "build_flags": "avx2,fma", "cpu": "unknown", "os": "linux",
        "metadata_missing": ["cpu"], "descriptive_only": True,
    }
    base["output_witness_sha256"] = hashlib.sha256(b"issue600-output-v1|" + b"1" * 64 + b"|" + b"1" * 64 + b"|1").hexdigest()
    base["traffic_witness_sha256"] = hashlib.sha256(b"issue600-traffic-v1|1|48000|128|8|8|256|512|4096|32768|32768|4096|0|1048576").hexdigest()
    with tempfile.TemporaryDirectory() as directory:
        root = pathlib.Path(directory)
        expected_path = root / "expected.json"
        records_path = root / "records.jsonl"
        expected_path.write_text(json.dumps({"schema_version": 1, "issue": 600, "kind": "input_symmetry_benchmark_preflight", "status": "READY", **identity}), encoding="utf-8")
        records = []
        for round_ in (1, 2):
            record = dict(base, round=round_)
            record["output_witness_sha256"] = hashlib.sha256(
                f"issue600-output-v1|{'1' * 64}|{'1' * 64}|{round_}".encode()
            ).hexdigest()
            record["traffic_witness_sha256"] = hashlib.sha256(
                f"issue600-traffic-v1|{round_}|48000|128|8|8|256|512|4096|32768|32768|4096|0|1048576".encode()
            ).hexdigest()
            records.append(record)

        def write(values: list[dict[str, Any]]) -> None:
            records_path.write_text("".join(json.dumps(item, sort_keys=True, allow_nan=True) + "\n" for item in values), encoding="utf-8")

        write(records)
        validate(records_path, expected_path)
        duplicate = json.dumps(records[0], sort_keys=True)[:-1] + ',"round":1}\n'
        records_path.write_text(duplicate + json.dumps(records[1], sort_keys=True) + "\n", encoding="utf-8")
        try:
            validate(records_path, expected_path)
        except ValueError:
            pass
        else:
            raise AssertionError("duplicate JSON key escaped")
        write(records)
        incomplete = json.loads(expected_path.read_text(encoding="utf-8"))
        incomplete.pop("validator_sha256")
        expected_path.write_text(json.dumps(incomplete), encoding="utf-8")
        try:
            validate(records_path, expected_path)
        except ValueError:
            pass
        else:
            raise AssertionError("incomplete seal escaped")
        expected_path.write_text(json.dumps({"schema_version": 1, "issue": 600, "kind": "input_symmetry_benchmark_preflight", "status": "READY", **identity}), encoding="utf-8")
        mutations: list[list[dict[str, Any]]] = []
        value = copy.deepcopy(records); value.pop(); mutations.append(value)
        value = copy.deepcopy(records); value.reverse(); mutations.append(value)
        value = copy.deepcopy(records); value[0]["accepted_records"] = 0; mutations.append(value)
        value = copy.deepcopy(records); value[0]["output_sha256"] = "f" * 64; mutations.append(value)
        value = copy.deepcopy(records); value[0]["accepted_records"] = True; mutations.append(value)
        value = copy.deepcopy(records); value[0]["target_pair_db"] = [-6, -12.0]; mutations.append(value)
        value = copy.deepcopy(records); value[0]["source_tree"] = "0" * 40; mutations.append(value)
        value = copy.deepcopy(records); value[0]["cpu"] = "arbitrary-host-label"; mutations.append(value)
        value = copy.deepcopy(records); value[0]["mutation"] = True; mutations.append(value)
        value = copy.deepcopy(records); value[0]["elapsed_ns"] = float("nan"); mutations.append(value)
        for mutation in mutations:
            write(mutation)
            try:
                validate(records_path, expected_path)
            except (ValueError, json.JSONDecodeError):
                continue
            raise AssertionError("validator self-test mutation escaped")
    print("input-symmetry validator self-test passed (12 mutations caught)")


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
