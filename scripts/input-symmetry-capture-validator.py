#!/usr/bin/env python3
"""Strict, side-effect free validator for the #603 seal and two capture records."""
from __future__ import annotations

import json
import pathlib
import sys

DIGEST = "75eeffae6a0116867d6d6fbe589834df53f3dce87ffd09092f22e5968bc28f87"
RECORD_KEYS = {
    "schema_version", "issue", "kind", "round", "fixture_id", "fixture_sha256",
    "sample_rate_hz", "quantum_frames", "lane_width", "track_count", "records_per_block",
    "target_pair_db", "smoothing_samples", "preparation_blocks_per_owner",
    "measured_blocks_per_owner", "owners", "render_denominator", "attempted_records",
    "accepted_records", "successful_renders", "render_errors", "output_words", "nonzero_samples",
    "owner_digests", "elapsed_ns", "nanoseconds_per_plan_render", "source_commit", "source_tree",
    "binary_sha256", "source_sha256", "argv", "cwd", "compiler", "target_triple",
    "effective_build_flags", "backend", "cpu_model", "os", "governor_or_power_mode",
    "rust_version", "llvm_version", "target_features", "profile", "background_load_note",
    "measurement_control", "cpu_affinity", "candidate_commit", "missing_metadata", "descriptive_only",
}
SEAL_KEYS = {
    "schema_version", "issue", "kind", "status", "candidate_commit", "candidate_tree",
    "binary_sha256", "fixture_sha256", "source_sha256", "timed_source_sha256", "untimed_source_sha256",
    "dispatcher_sha256", "validator_sha256", "runner_sha256", "preflight_sha256", "cargo_lock_sha256",
    "argv", "cwd", "compiler", "target_triple", "effective_build_flags", "sample_rate_hz",
    "quantum_frames", "lane_width", "track_count", "records_per_block", "preparation_blocks_per_owner",
    "measured_blocks_per_owner", "expected_records", "expected_attempted_records", "expected_renders_per_round", "expected_rounds",
    "expected_timed_render_calls", "expected_workload_processes",
    "target_pair_db", "smoothing_samples", "owners",
}
U64_MAX = (1 << 64) - 1


def pairs(pairs_list):
    result = {}
    for key, value in pairs_list:
        if key in result:
            raise ValueError(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def read_json(path: pathlib.Path):
    with path.open(encoding="utf-8") as handle:
        return json.load(handle, object_pairs_hook=pairs, parse_constant=lambda value: (_ for _ in ()).throw(ValueError(value)))


def exact_keys(obj, expected, label):
    if not isinstance(obj, dict) or set(obj) != expected:
        raise ValueError(f"{label}: exact key set required")


def integer(value, label, minimum=0):
    if isinstance(value, bool) or not isinstance(value, int) or value < minimum or value > U64_MAX:
        raise ValueError(f"{label}: strict nonnegative integer required")
    return value


def string(value, label):
    if not isinstance(value, str) or not value:
        raise ValueError(f"{label}: nonempty string required")
    return value


def digest(value, label):
    value = string(value, label)
    if len(value) != 64 or any(char not in "0123456789abcdef" for char in value):
        raise ValueError(f"{label}: lowercase sha256 required")


def git_identity(value, label):
    value = string(value, label)
    if len(value) != 40 or any(char not in "0123456789abcdef" for char in value):
        raise ValueError(f"{label}: lowercase git identity required")


def validate_record(record, expected_round, seal):
    exact_keys(record, RECORD_KEYS, "record")
    if integer(record["schema_version"], "schema_version") != 1 or integer(record["issue"], "issue") != 603 or record["kind"] != "input_symmetry_capture":
        raise ValueError("record identity mismatch")
    if integer(record["round"], "round", 1) != expected_round:
        raise ValueError("round order mismatch")
    if record["fixture_id"] != "fixtures/session/v1/parametric-eq-bank-console.json":
        raise ValueError("fixture identity mismatch")
    for key in ("fixture_sha256", "binary_sha256", "source_sha256"):
        digest(record[key], key)
    for key in ("source_commit", "source_tree"):
        git_identity(record[key], key)
    for key, expected in (("sample_rate_hz", 48000), ("quantum_frames", 128), ("lane_width", 8),
                          ("track_count", 8), ("records_per_block", 8), ("smoothing_samples", 256),
                          ("preparation_blocks_per_owner", 512), ("measured_blocks_per_owner", 4096),
                          ("owners", 2), ("render_denominator", 8192), ("attempted_records", 65536),
                          ("accepted_records", 65536), ("successful_renders", 8192), ("render_errors", 0),
                          ("output_words", 2097152)):
        if integer(record[key], key) != expected:
            raise ValueError(f"{key}: unexpected value")
    pair = record["target_pair_db"]
    if (not isinstance(pair, list) or len(pair) != 2
            or any(isinstance(value, bool) or not isinstance(value, float) for value in pair)
            or pair != seal["target_pair_db"] or record["backend"] != "Simd8"):
        raise ValueError("workload identity mismatch")
    if not isinstance(record["owner_digests"], list) or record["owner_digests"] != [DIGEST, DIGEST]:
        raise ValueError("reviewed owner digest mismatch")
    for key in ("elapsed_ns", "nanoseconds_per_plan_render"):
        integer(record[key], key, 1)
    if record["elapsed_ns"] // 8192 != record["nanoseconds_per_plan_render"]:
        raise ValueError("timing denominator mismatch")
    for key in ("argv", "cwd", "compiler", "target_triple", "effective_build_flags", "os"):
        string(record[key], key)
    if record["effective_build_flags"] != "-C target-feature=+avx2,+fma -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort -C debug=1":
        raise ValueError("effective release settings mismatch")
    for key in ("cpu_model", "governor_or_power_mode", "rust_version", "llvm_version", "target_features",
                "profile", "background_load_note", "measurement_control", "cpu_affinity", "candidate_commit"):
        if record[key] is not None and not isinstance(record[key], str):
            raise ValueError(f"{key}: string or null required")
    if (not isinstance(record["missing_metadata"], list)
            or any(not isinstance(item, str) for item in record["missing_metadata"])
            or sorted(record["missing_metadata"]) != record["missing_metadata"]
            or len(set(record["missing_metadata"])) != len(record["missing_metadata"])):
        raise ValueError("missing metadata must be sorted")
    allowed_missing = {"cpu_model", "governor_or_power_mode", "rust_version", "llvm_version", "target_triple",
                       "target_features", "profile", "background_load_note", "measurement_control", "cpu_affinity", "candidate_commit"}
    if any(not isinstance(item, str) or item not in allowed_missing for item in record["missing_metadata"]):
        raise ValueError("unknown missing metadata")
    metadata_keys = ("cpu_model", "governor_or_power_mode", "rust_version", "llvm_version", "target_triple",
                     "target_features", "profile", "background_load_note", "measurement_control", "cpu_affinity", "candidate_commit")
    if {key for key in metadata_keys if record[key] is None} != set(record["missing_metadata"]):
        raise ValueError("missing metadata does not match null fields")
    nonzero = integer(record["nonzero_samples"], "nonzero_samples", 1)
    if record["descriptive_only"] is not True or nonzero > record["output_words"]:
        raise ValueError("nonzero descriptive PCM required")
    for record_key, seal_key in (("source_commit", "candidate_commit"), ("source_tree", "candidate_tree"),
                                 ("binary_sha256", "binary_sha256"), ("fixture_sha256", "fixture_sha256"),
                                 ("source_sha256", "source_sha256"), ("argv", "argv"), ("cwd", "cwd"),
                                 ("compiler", "compiler"), ("effective_build_flags", "effective_build_flags"),
                                 ("target_triple", "target_triple")):
        if record[record_key] != seal[seal_key]:
            raise ValueError(f"record/seal identity mismatch: {record_key}")
    for record_key, seal_key in (("target_triple", "target_triple"), ("candidate_commit", "candidate_commit"),
                                 ("rust_version", "compiler")):
        if record[record_key] != seal[seal_key]:
            raise ValueError(f"record/seal metadata mismatch: {record_key}")
    for record_key, seal_key in (("sample_rate_hz", "sample_rate_hz"), ("quantum_frames", "quantum_frames"),
                                 ("lane_width", "lane_width"), ("track_count", "track_count"),
                                 ("records_per_block", "records_per_block"),
                                 ("preparation_blocks_per_owner", "preparation_blocks_per_owner"),
                                 ("measured_blocks_per_owner", "measured_blocks_per_owner")):
        if record[record_key] != seal[seal_key]:
            raise ValueError(f"record/seal workload mismatch: {record_key}")
    if record["smoothing_samples"] != seal["smoothing_samples"] or record["owners"] != seal["owners"]:
        raise ValueError("record/seal workload mismatch: smoothing or owners")


def validate_seal(seal):
    exact_keys(seal, SEAL_KEYS, "seal")
    if (integer(seal["schema_version"], "schema_version"), integer(seal["issue"], "issue"), seal["kind"], seal["status"]) != (1, 603, "input_symmetry_capture_seal", "READY"):
        raise ValueError("seal identity/status mismatch")
    for key in ("binary_sha256", "fixture_sha256", "source_sha256", "timed_source_sha256", "untimed_source_sha256", "dispatcher_sha256",
                "validator_sha256", "runner_sha256", "preflight_sha256", "cargo_lock_sha256"):
        digest(seal[key], key)
    git_identity(seal["candidate_commit"], "candidate_commit")
    git_identity(seal["candidate_tree"], "candidate_tree")
    for key in ("argv", "cwd", "compiler", "target_triple", "effective_build_flags"):
        string(seal[key], key)
    if seal["effective_build_flags"] != "-C target-feature=+avx2,+fma -C opt-level=3 -C lto=fat -C codegen-units=1 -C panic=abort -C debug=1":
        raise ValueError("seal effective release settings mismatch")
    for key, expected in (("sample_rate_hz", 48000), ("quantum_frames", 128), ("lane_width", 8), ("track_count", 8),
                          ("records_per_block", 8), ("preparation_blocks_per_owner", 512), ("measured_blocks_per_owner", 4096),
                          ("expected_records", 2), ("expected_attempted_records", 65536),
                          ("expected_renders_per_round", 8192), ("expected_rounds", 2),
                          ("expected_timed_render_calls", 16384), ("expected_workload_processes", 1)):
        if integer(seal[key], f"seal {key}") != expected:
            raise ValueError(f"seal {key}: unexpected value")
    pair = seal["target_pair_db"]
    if (not isinstance(pair, list) or len(pair) != 2
            or any(isinstance(value, bool) or not isinstance(value, float) for value in pair)
            or pair != [-6.0, -12.0]):
        raise ValueError("seal target pair mismatch")
    if integer(seal["smoothing_samples"], "seal smoothing_samples") != 256:
        raise ValueError("seal smoothing mismatch")
    if integer(seal["owners"], "seal owners") != 2:
        raise ValueError("seal owners mismatch")


def validate(seal_path, records_path):
    seal = read_json(pathlib.Path(seal_path))
    validate_seal(seal)
    lines = pathlib.Path(records_path).read_text(encoding="utf-8").splitlines()
    if len(lines) != 2 or any(not line.strip() for line in lines):
        raise ValueError("exactly two nonblank JSON lines required")
    for index, line in enumerate(lines, 1):
        validate_record(json.loads(line, object_pairs_hook=pairs, parse_constant=lambda value: (_ for _ in ()).throw(ValueError(value))), index, seal)
    return True


def main(argv):
    if len(argv) == 3 and argv[1] == "--seal-only":
        try:
            validate_seal(read_json(pathlib.Path(argv[2])))
        except (OSError, ValueError, json.JSONDecodeError) as error:
            print(f"FAIL: {error}", file=sys.stderr)
            return 1
        print("PASS: #603 seal")
        return 0
    if len(argv) != 3:
        print("usage: input-symmetry-capture-validator.py SEAL RECORDS | --seal-only SEAL", file=sys.stderr)
        return 2
    try:
        validate(argv[1], argv[2])
    except (OSError, ValueError, json.JSONDecodeError) as error:
        print(f"FAIL: {error}", file=sys.stderr)
        return 1
    print("PASS: #603 seal and two capture records")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
