#!/usr/bin/env python3
"""Issue-111 successor authority validator; stdlib only and import-safe."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import math
import os
import stat
import struct
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PREDECESSOR = ROOT / "scripts/check-builtins-listening-033.py"
PREDECESSOR_SHA256 = "6654089b2a9cd466da531ed929dbe77c0005875d8bbfc8eb9a7d74c80a76fccc"
class Invalid(ValueError):
    pass


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def canonical(value: object) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n").encode()


def predecessor():
    if sha256(PREDECESSOR) != PREDECESSOR_SHA256:
        raise Invalid("frozen Issue-033 validator identity")
    specification = importlib.util.spec_from_file_location("issue033_frozen", PREDECESSOR)
    if specification is None or specification.loader is None:
        raise Invalid("frozen Issue-033 validator import")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


def require_regular(path: Path, mode: int | None = None) -> None:
    try:
        metadata = path.lstat()
    except OSError as error:
        raise Invalid(f"missing regular file: {path.name}") from error
    if not stat.S_ISREG(metadata.st_mode) or metadata.st_nlink != 1:
        raise Invalid(f"regular one-link file: {path.name}")
    if mode is not None and stat.S_IMODE(metadata.st_mode) != mode:
        raise Invalid(f"file mode: {path.name}")


def packet_authority(packet: Path):
    old = predecessor()
    if packet.is_symlink() or not packet.is_dir():
        raise Invalid("packet directory")
    preparation_path = packet / "public/preparation.json"
    key_path = packet / "private/assignment-key.json"
    require_regular(preparation_path, 0o444)
    require_regular(key_path, 0o600)
    preparation = old.load_canonical(preparation_path)
    old.validate_preparation(preparation)
    key_digest = sha256(key_path)
    if preparation["assignment_key_sha256"] != key_digest:
        raise Invalid("preparation assignment-key commitment")
    if preparation["packet_member_sha256"]["private/assignment-key.json"] != key_digest:
        raise Invalid("packet-member assignment-key commitment")
    old.validate_packet(packet)
    return old, preparation_path, key_path


def validate_packet(packet: Path) -> None:
    packet_authority(packet)


def validate_linked(
    packet: Path,
    responses_path: Path,
    reveal_path: Path,
    qualification_path: Path,
) -> None:
    old, preparation_path, key_path = packet_authority(packet)
    for path in (responses_path, reveal_path, qualification_path):
        require_regular(path, 0o600)
    qualification = old.load_canonical(qualification_path)
    old.validate_linked_qualification(
        qualification,
        preparation_path,
        responses_path,
        reveal_path,
        key_path,
    )


def re_hex(value: object, length: int) -> bool:
    return (
        type(value) is str
        and len(value) == length
        and all(character in "0123456789abcdef" for character in value)
    )


def write_mode(path: Path, content: bytes, mode: int) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(content)
    path.chmod(mode)


def wave(sample_bits: tuple[int, int]) -> tuple[bytes, float, float]:
    pair = struct.pack("<II", *sample_bits)
    body = pair * 480_000
    header = (
        b"RIFF"
        + (3_840_036).to_bytes(4, "little")
        + b"WAVEfmt "
        + (16).to_bytes(4, "little")
        + (3).to_bytes(2, "little")
        + (2).to_bytes(2, "little")
        + (48_000).to_bytes(4, "little")
        + (384_000).to_bytes(4, "little")
        + (8).to_bytes(2, "little")
        + (32).to_bytes(2, "little")
        + b"data"
        + (3_840_000).to_bytes(4, "little")
    )
    left, right = (struct.unpack("<f", bits.to_bytes(4, "little"))[0] for bits in sample_bits)
    energy = 0.0
    peak = 0.0
    for sample in (left, right):
        for _ in range(480_000):
            energy += sample * sample
            peak = max(peak, abs(sample))
    return header + body, math.sqrt(energy / 960_000), peak


def build_fixture(root: Path):
    old = predecessor()
    packet = root / "packet"
    public = packet / "public"
    private = packet / "private"
    public.mkdir(parents=True)
    private.mkdir()
    public.chmod(0o755)
    private.chmod(0o700)
    tokens = [f"{index:032x}.wav" for index in range(1, 5)]
    roles = old.ROLES
    patterns = [
        (0x3DCCCCCD, 0x3DCCCCCD),
        (0xBDCCCCCD, 0x3DCCCCCD),
        (0x3DCCCCCD, 0xBDCCCCCD),
        (0xBDCCCCCD, 0xBDCCCCCD),
    ]
    stimuli = []
    for token, pattern in zip(tokens, patterns):
        content, rms, peak = wave(pattern)
        path = public / token
        write_mode(path, content, 0o444)
        stimuli.append(
            {
                "bytes": len(content),
                "frames": 480_000,
                "peak": peak,
                "rms": rms,
                "sha256": sha256(path),
                "token": token,
            }
        )
    key = {
        "filter_x_candidate": [True] * 10 + [False] * 10,
        "matrix_candidate_first": [True] * 10 + [False] * 10,
        "schema_version": 1,
        "seed": "42",
        "token_roles": dict(zip(tokens, roles)),
    }
    key_path = private / "assignment-key.json"
    write_mode(key_path, canonical(key), 0o600)
    provenance = {
        "conversion_command": "documented conversion",
        "conversion_tool": "format-only-tool",
        "conversion_version": "1",
        "license_or_permission": "format-only permission",
        "permission_confirmed": True,
        "redistribution_status": "private-no-redistribution",
        "retention_reference": "private-artifact-reference",
        "rights_holder": "format-only-holder",
        "schema_version": 1,
        "source_sha256": "1" * 64,
    }
    provenance_path = private / "source-provenance.json"
    write_mode(provenance_path, canonical(provenance), 0o600)
    schedule = {
        "filter_x_candidate": key["filter_x_candidate"],
        "matrix_candidate_first": key["matrix_candidate_first"],
        "schema_version": 1,
    }
    manifest = {
        "assignment_key_sha256": sha256(key_path),
        "probe_render_sha256": sorted(hashlib.sha256(role.encode()).hexdigest() for role in roles),
        "schedule_sha256": hashlib.sha256(canonical(schedule)).hexdigest(),
        "schema_version": 1,
        "source_provenance_sha256": sha256(provenance_path),
        "source_sha256": "1" * 64,
        "stimuli": sorted(stimuli, key=lambda value: value["token"]),
    }
    write_mode(public / "render-manifest.json", canonical(manifest), 0o444)
    copied = {
        "FACILITATOR.md": b"format-only facilitator\n",
        "filter-preregistration.md": b"format-only filter preregistration\n",
        "matrix-preregistration.md": b"format-only matrix preregistration\n",
        "preparation.schema.json": b"{}\n",
        "qualification.schema.json": b"{}\n",
        "response-form.jsonl": b"",
        "response.schema.json": b"{}\n",
        "reveal.schema.json": b"{}\n",
    }
    for name, content in copied.items():
        write_mode(public / name, content, 0o444)
    preparation = old.base_preparation()
    preparation.update(
        {
            "assignment_key_sha256": sha256(key_path),
            "probe_render_sha256": manifest["probe_render_sha256"],
            "schedule_sha256": manifest["schedule_sha256"],
            "source_provenance_sha256": manifest["source_provenance_sha256"],
            "source_sha256": manifest["source_sha256"],
            "stimuli": manifest["stimuli"],
        }
    )
    preparation["packet_members"] = old.expected_packet_members(preparation)
    preparation["packet_member_sha256"] = {
        name: sha256(packet / name)
        for name in preparation["packet_members"]
        if name != "public/preparation.json"
    }
    preparation_path = public / "preparation.json"
    write_mode(preparation_path, canonical(preparation), 0o444)
    rows = []
    for procedure in old.PROCEDURES:
        for trial in range(1, 21):
            rows.append(
                {
                    "answer": "A" if trial % 2 else "B",
                    "attempt": 1,
                    "confidence": 50,
                    "logical_trial": trial,
                    "observation": "",
                    "procedure": procedure,
                    "reason": None,
                    "schema_version": 1,
                    "sequence": len(rows) + 1,
                    "valid": True,
                }
            )
    responses_path = root / "responses.jsonl"
    write_mode(responses_path, b"".join(canonical(row) for row in rows), 0o600)
    reveal = {
        "assignment_key_sha256": sha256(key_path),
        "conditions": {
            "calibration_level_method": "format-only method",
            "conflicts": "none declared",
            "driver_mode": "format-only mode",
            "environmental_notes": "format-only notes",
            "playback_hardware": "format-only hardware",
            "room_or_headphone": "format-only room",
            "transducer": "format-only transducer",
        },
        "record_id": "issue007-listening-reveal-v1",
        "response_sha256": sha256(responses_path),
        "reveal_utc": "2026-08-22T12:00:00Z",
        "schema_version": 1,
        "signoffs": {
            "facilitator": "format-person-f",
            "listener": "format-person-l",
            "reveal_verifier": "format-person-r",
        },
        "token_roles": key["token_roles"],
        "trials": [
            {
                "assignment": (
                    "x-candidate"
                    if procedure == old.PROCEDURES[0] and trial <= 10
                    else "x-comparator"
                    if procedure == old.PROCEDURES[0]
                    else "candidate-first"
                    if trial <= 10
                    else "comparator-first"
                ),
                "logical_trial": trial,
                "procedure": procedure,
            }
            for procedure in old.PROCEDURES
            for trial in range(1, 21)
        ],
    }
    reveal_path = root / "reveal.json"
    write_mode(reveal_path, canonical(reveal), 0o600)
    numerator, denominator, p = old.exact_p(10)
    low, high = old.wilson(10)
    qualification = {
        "adverse_observations": [],
        "authorities": {
            "preparation": sha256(preparation_path),
            "responses": sha256(responses_path),
            "reveal": sha256(reveal_path),
        },
        "conclusion": "Format-only linked record.",
        "corrective_links": [],
        "counts": {
            "abx_correct": 10,
            "filter_valid_responses": 20,
            "human_listening_sessions": 2,
            "matrix_candidate_preferred": 10,
            "matrix_valid_responses": 20,
            "total_attempts": 40,
        },
        "deviations": [],
        "disposition": "PASS",
        "record_id": "issue007-listening-qualification-v1",
        "schema_version": 1,
        "statistics": {
            "abx_p_denominator": denominator,
            "abx_p_numerator": numerator,
            "abx_p_two_sided": format(p, ".17g"),
            "abx_wilson_high": format(high, ".17g"),
            "abx_wilson_low": format(low, ".17g"),
            "matrix_wilson_high": format(high, ".17g"),
            "matrix_wilson_low": format(low, ".17g"),
        },
    }
    qualification_path = root / "qualification.json"
    write_mode(qualification_path, canonical(qualification), 0o600)
    return packet, responses_path, reveal_path, qualification_path


def expect_invalid(action, context: str) -> None:
    try:
        action()
    except (Invalid, ValueError, OSError):
        return
    raise AssertionError(f"mutation accepted: {context}")


def self_test() -> None:
    old = predecessor()
    old.self_test()
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        packet, responses, reveal, qualification = build_fixture(root)
        validate_linked(packet, responses, reveal, qualification)
        preparation_path = packet / "public/preparation.json"
        key_path = packet / "private/assignment-key.json"
        original_preparation = preparation_path.read_bytes()
        original_key = key_path.read_bytes()
        original_reveal = reveal.read_bytes()
        original_responses = responses.read_bytes()
        original_qualification = qualification.read_bytes()

        def mutate_preparation(field: str) -> None:
            value = json.loads(original_preparation)
            if field == "assignment":
                value["assignment_key_sha256"] = "0" * 64
            else:
                value["packet_member_sha256"]["private/assignment-key.json"] = "0" * 64
            preparation_path.write_bytes(canonical(value))

        mutate_preparation("assignment")
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "preparation assignment")
        preparation_path.write_bytes(original_preparation)
        mutate_preparation("member")
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "packet key member")
        preparation_path.write_bytes(original_preparation)

        alternate = json.loads(original_key)
        alternate["seed"] = "43"
        key_path.write_bytes(canonical(alternate))
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "alternate same-shaped key")
        key_path.write_bytes(original_key)

        moved = packet / "private/alternate-key.json"
        key_path.rename(moved)
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "wrong key location")
        moved.rename(key_path)
        backup = packet / "private/key-backup"
        key_path.rename(backup)
        key_path.mkdir()
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "key directory")
        key_path.rmdir()
        backup.rename(key_path)
        key_path.rename(backup)
        key_path.symlink_to(backup.name)
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "key symlink")
        key_path.unlink()
        backup.rename(key_path)
        backup.write_bytes(original_key)
        key_path.unlink()
        os.link(backup, key_path)
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "key hardlink")
        key_path.unlink()
        backup.rename(key_path)
        key_path.chmod(0o644)
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "key mode")
        key_path.chmod(0o600)

        packet_input = packet / "public/FACILITATOR.md"
        packet_input.write_bytes(packet_input.read_bytes() + b"drift")
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "packet drift")
        packet_input.write_bytes(b"format-only facilitator\n")

        changed_reveal = json.loads(original_reveal)
        changed_reveal["assignment_key_sha256"] = "0" * 64
        reveal.write_bytes(canonical(changed_reveal))
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "reveal key")
        reveal.write_bytes(original_reveal)
        changed_rows = original_responses.replace(b'"answer":"A"', b'"answer":"B"', 1)
        responses.write_bytes(changed_rows)
        expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), "response drift")
        responses.write_bytes(original_responses)

        for context, mutation in (
            ("authority", lambda value: value["authorities"].__setitem__("preparation", "0" * 64)),
            ("count", lambda value: value["counts"].__setitem__("abx_correct", 9)),
            ("statistic", lambda value: value["statistics"].__setitem__("abx_p_numerator", 0)),
        ):
            value = json.loads(original_qualification)
            mutation(value)
            qualification.write_bytes(canonical(value))
            expect_invalid(lambda: validate_linked(packet, responses, reveal, qualification), context)
        qualification.write_bytes(original_qualification)
    print("Issue-111 linked authority self-test: PASS (14 rejection classes; format-only data)")


def main(arguments: list[str]) -> int:
    try:
        if arguments == ["--self-test"]:
            self_test()
        elif len(arguments) == 2 and arguments[0] == "--packet":
            validate_packet(Path(arguments[1]))
        elif len(arguments) == 5 and arguments[0] == "--linked":
            validate_linked(*(Path(argument) for argument in arguments[1:]))
        else:
            raise Invalid("usage: check-builtins-listening-111.py --self-test|--packet PACKET|--linked PACKET RESPONSES REVEAL QUALIFICATION")
    except (Invalid, OSError, ValueError) as error:
        print(f"Issue-111 validation failure: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
