#!/usr/bin/env python3
"""Strict stdlib validator for Issue 081's eight benchmark JSONL records."""

from __future__ import annotations

import argparse
import copy
import json
import math
import pathlib
import re
import tempfile


WORKLOADS = (
    "descriptor_verify_identity_a",
    "package_verify_cid_select_a",
    "state_verify_reencode_current",
    "migration_two_step_bank_restore",
)
OBSERVATIONS = 256
MACHINE_FIELDS = (
    "background_load",
    "cpu_model",
    "governor",
    "kernel",
    "logical_cores",
    "os",
    "physical_cores",
    "power_mode",
)
HEX40 = re.compile(r"[0-9a-f]{40}\Z")
HEX64 = re.compile(r"[0-9a-f]{64}\Z")
ADDRESS = re.compile(r"0x[0-9a-fA-F]+")
KEYS = {
    "schema_version", "issue", "workload_id", "round", "observation_count", "unit",
    "candidate_commit", "candidate_tree", "binary_sha256", "tool_manifest_sha256", "tool_source_sha256",
    "fixture_manifest_sha256", "output_sha256", "rust_version", "llvm_version",
    "target_triple", "profile", "cpu_model", "logical_cores", "physical_cores", "os",
    "kernel", "power_mode", "governor", "background_load", "timer_method",
    "percentile_method", "total_ns", "min_ns_per_operation", "p50_ns_per_operation",
    "p95_ns_per_operation", "p99_ns_per_operation", "p99_9_ns_per_operation",
    "max_ns_per_operation", "descriptive_only", "metadata_incomplete", "missing_metadata",
}


def load_expected(path: pathlib.Path) -> dict[str, object]:
    value = json.loads(path.read_text(encoding="utf-8"))
    required = {
        "candidate_commit", "candidate_tree", "binary_sha256", "tool_manifest_sha256", "tool_source_sha256",
        "fixture_manifest_sha256", "output_sha256",
    }
    if not isinstance(value, dict) or set(value) != required:
        raise ValueError("expected identity file is incomplete")
    outputs = value["output_sha256"]
    if (
        not isinstance(outputs, dict)
        or set(outputs) != set(WORKLOADS)
        or any(not isinstance(digest, str) or not HEX64.fullmatch(digest) for digest in outputs.values())
    ):
        raise ValueError("expected output identity map")
    return value


def integer(value: object) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def validate_record(record: object, expected: dict[str, object]) -> dict[str, object]:
    if not isinstance(record, dict) or set(record) != KEYS:
        raise ValueError("record key set")
    if (
        not integer(record["schema_version"])
        or not integer(record["issue"])
        or record["schema_version"] != 1
        or record["issue"] != 81
    ):
        raise ValueError("schema or issue")
    if (
        record["workload_id"] not in WORKLOADS
        or not integer(record["round"])
        or record["round"] not in (1, 2)
    ):
        raise ValueError("workload or round")
    if (
        not integer(record["observation_count"])
        or record["observation_count"] != OBSERVATIONS
        or record["unit"] != "ns_per_operation"
    ):
        raise ValueError("observation contract")
    for key in (
        "candidate_commit", "candidate_tree", "binary_sha256", "tool_manifest_sha256", "tool_source_sha256",
        "fixture_manifest_sha256",
    ):
        if record[key] != expected[key]:
            raise ValueError(f"identity mismatch: {key}")
    if not isinstance(record["candidate_commit"], str) or not HEX40.fullmatch(record["candidate_commit"]):
        raise ValueError("candidate commit")
    if not isinstance(record["candidate_tree"], str) or not HEX40.fullmatch(record["candidate_tree"]):
        raise ValueError("candidate tree")
    for key in ("binary_sha256", "tool_manifest_sha256", "tool_source_sha256", "fixture_manifest_sha256", "output_sha256"):
        if not isinstance(record[key], str) or not HEX64.fullmatch(record[key]):
            raise ValueError(f"digest: {key}")
    expected_outputs = expected["output_sha256"]
    if record["output_sha256"] != expected_outputs[record["workload_id"]]:
        raise ValueError("output digest")
    for key in ("rust_version", "llvm_version", "target_triple", "profile"):
        if not isinstance(record[key], str) or not record[key]:
            raise ValueError(f"required metadata: {key}")
    for key in MACHINE_FIELDS:
        if not isinstance(record[key], str):
            raise ValueError(f"machine metadata type: {key}")
    missing = record["missing_metadata"]
    if (
        not isinstance(missing, list)
        or any(not isinstance(value, str) or value not in MACHINE_FIELDS for value in missing)
        or missing != sorted(set(missing))
    ):
        raise ValueError("missing metadata")
    honest_missing = sorted(key for key in MACHINE_FIELDS if not record[key])
    if (
        not isinstance(record["metadata_incomplete"], bool)
        or missing != honest_missing
        or record["metadata_incomplete"] is not bool(missing)
    ):
        raise ValueError("dishonest missing metadata")
    if record["timer_method"] != "std::time::Instant" or record["percentile_method"] != "nearest-rank":
        raise ValueError("timing method")
    timing_keys = (
        "total_ns", "min_ns_per_operation", "p50_ns_per_operation", "p95_ns_per_operation",
        "p99_ns_per_operation", "p99_9_ns_per_operation", "max_ns_per_operation",
    )
    if any(not integer(record[key]) or record[key] <= 0 for key in timing_keys):
        raise ValueError("nonpositive or noninteger timing")
    ordered = [record[key] for key in timing_keys[1:]]
    if ordered != sorted(ordered):
        raise ValueError("unordered timing")
    if record["total_ns"] < record["min_ns_per_operation"] * OBSERVATIONS:
        raise ValueError("impossible total")
    if record["total_ns"] > record["max_ns_per_operation"] * OBSERVATIONS:
        raise ValueError("impossible total")
    if record["descriptive_only"] is not True:
        raise ValueError("descriptive-only flag")
    for key, value in record.items():
        if isinstance(value, float) and not math.isfinite(value):
            raise ValueError("nonfinite value")
        if isinstance(value, str) and (value.startswith("/") or ADDRESS.search(value)):
            raise ValueError(f"address or absolute path: {key}")
    return record


def validate(records_path: pathlib.Path, expected_path: pathlib.Path) -> None:
    expected = load_expected(expected_path)
    lines = records_path.read_text(encoding="utf-8").splitlines()
    if len(lines) != 8 or any(not line for line in lines):
        raise ValueError("exactly eight records required")
    records = [validate_record(json.loads(line), expected) for line in lines]
    pairs = [(record["workload_id"], record["round"]) for record in records]
    required = [(workload, round_) for round_ in (1, 2) for workload in WORKLOADS]
    if pairs != required or len(set(pairs)) != 8:
        raise ValueError("record order, duplicate, or missing pair")
    shared = (
        "candidate_commit", "candidate_tree", "binary_sha256", "tool_manifest_sha256", "tool_source_sha256",
        "fixture_manifest_sha256", "rust_version", "llvm_version", "target_triple", "profile",
        *MACHINE_FIELDS, "metadata_incomplete", "missing_metadata",
    )
    for key in shared:
        if any(record[key] != records[0][key] for record in records[1:]):
            raise ValueError(f"inconsistent shared field: {key}")
    for workload in WORKLOADS:
        values = [record["output_sha256"] for record in records if record["workload_id"] == workload]
        if len(set(values)) != 1:
            raise ValueError("round output digest mismatch")


def main() -> int:
    parser = argparse.ArgumentParser(allow_abbrev=False)
    parser.add_argument("records", nargs="?", type=pathlib.Path)
    parser.add_argument("expected", nargs="?", type=pathlib.Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        if args.records is not None or args.expected is not None:
            parser.error("--self-test takes no paths")
        expected = {
            "candidate_commit": "1" * 40,
            "candidate_tree": "2" * 40,
            "binary_sha256": "3" * 64,
            "tool_manifest_sha256": "a" * 64,
            "tool_source_sha256": "4" * 64,
            "fixture_manifest_sha256": "5" * 64,
            "output_sha256": {
                workload: str(index + 6) * 64 for index, workload in enumerate(WORKLOADS)
            },
        }
        base = {
            "schema_version": 1, "issue": 81, "observation_count": 256,
            "unit": "ns_per_operation",
            **{key: value for key, value in expected.items() if key != "output_sha256"},
            "rust_version": "rustc synthetic", "llvm_version": "LLVM synthetic",
            "target_triple": "x86_64-unknown-linux-gnu", "profile": "release",
            "cpu_model": "", "logical_cores": "", "physical_cores": "", "os": "",
            "kernel": "", "power_mode": "", "governor": "", "background_load": "",
            "timer_method": "std::time::Instant", "percentile_method": "nearest-rank",
            "total_ns": 2560, "min_ns_per_operation": 10, "p50_ns_per_operation": 10,
            "p95_ns_per_operation": 10, "p99_ns_per_operation": 10,
            "p99_9_ns_per_operation": 10, "max_ns_per_operation": 10,
            "descriptive_only": True, "metadata_incomplete": True,
            "missing_metadata": list(MACHINE_FIELDS),
        }
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            expected_path = root / "expected.json"
            records_path = root / "records.jsonl"
            expected_path.write_text(json.dumps(expected), encoding="utf-8")
            records = [
                {
                    **base,
                    "workload_id": workload,
                    "round": round_,
                    "output_sha256": expected["output_sha256"][workload],
                }
                for round_ in (1, 2) for workload in WORKLOADS
            ]
            def write(values: list[dict[str, object]]) -> None:
                records_path.write_text(
                    "".join(json.dumps(value, sort_keys=True, allow_nan=True) + "\n" for value in values),
                    encoding="utf-8",
                )
            write(records)
            validate(records_path, expected_path)
            expected_mutations = []
            value = copy.deepcopy(expected); del value["output_sha256"]; expected_mutations.append(value)
            value = copy.deepcopy(expected); del value["output_sha256"][WORKLOADS[-1]]; expected_mutations.append(value)
            value = copy.deepcopy(expected); value["output_sha256"][WORKLOADS[0]] = "A" * 64; expected_mutations.append(value)
            value = copy.deepcopy(expected); value["output_sha256"]["extra"] = "f" * 64; expected_mutations.append(value)
            for mutation in expected_mutations:
                expected_path.write_text(json.dumps(mutation), encoding="utf-8")
                try:
                    validate(records_path, expected_path)
                except ValueError:
                    continue
                raise AssertionError("validator expected-identity mutation escaped")
            expected_path.write_text(json.dumps(expected), encoding="utf-8")
            mutations = []
            value = copy.deepcopy(records); del value[0]["unit"]; mutations.append(value)
            value = copy.deepcopy(records); value[0]["extra"] = True; mutations.append(value)
            mutations.append(copy.deepcopy(records[:-1]))
            value = copy.deepcopy(records); value[-1] = copy.deepcopy(value[0]); mutations.append(value)
            value = copy.deepcopy(records); value[0]["total_ns"] = 0; mutations.append(value)
            value = copy.deepcopy(records); value[0]["p50_ns_per_operation"] = 11; mutations.append(value)
            value = copy.deepcopy(records); value[0]["candidate_commit"] = "a" * 40; mutations.append(value)
            value = copy.deepcopy(records); value[4]["output_sha256"] = "f" * 64; mutations.append(value)
            value = copy.deepcopy(records)
            for record in value:
                record["output_sha256"] = "f" * 64
            mutations.append(value)
            value = copy.deepcopy(records); value[0]["missing_metadata"] = []; mutations.append(value)
            value = copy.deepcopy(records); value[0]["min_ns_per_operation"] = float("nan"); mutations.append(value)
            value = copy.deepcopy(records); value[0]["observation_count"] = 255; mutations.append(value)
            value = copy.deepcopy(records); value[0]["schema_version"] = True; mutations.append(value)
            value = copy.deepcopy(records); value[0]["issue"] = True; mutations.append(value)
            value = copy.deepcopy(records); value[0]["round"] = True; mutations.append(value)
            value = copy.deepcopy(records); value[0]["metadata_incomplete"] = 1; mutations.append(value)
            value = copy.deepcopy(records); value[0]["total_ns"] = 2561; mutations.append(value)
            for mutation in mutations:
                write(mutation)
                try:
                    validate(records_path, expected_path)
                except (ValueError, json.JSONDecodeError):
                    continue
                raise AssertionError("validator self-test mutation escaped")
        return 0
    if args.records is None or args.expected is None:
        parser.error("records and expected paths are required")
    validate(args.records, args.expected)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
