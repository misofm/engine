#!/usr/bin/env python3
"""Independent stdlib-only effect-state V1 encoder, verifier, and fixture checker."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import pathlib
import struct
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
FIXTURES = ROOT / "fixtures/effect-state/v1"
HEADER = 224
INITIAL = 16
UNAVAILABLE_INDEX = 0xFFFFFFFF
UNAVAILABLE_OFFSET = 0xFFFFFFFFFFFFFFFF
MAGIC = b"MISOEFST"
DIGEST_DOMAIN = b"miso.engine.effect-state.current-layout.v1\0"


def descriptor_reference():
    path = ROOT / "scripts/effect-descriptor-v1-reference.py"
    spec = importlib.util.spec_from_file_location("effect_descriptor_v1_reference", path)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load descriptor reference")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


DESCRIPTOR_REFERENCE = descriptor_reference()


class StateError(Exception):
    def __init__(
        self,
        code: int,
        detail: int = 0,
        item_index: int = UNAVAILABLE_INDEX,
        byte_offset: int = UNAVAILABLE_OFFSET,
        required_bytes: int = 0,
    ):
        self.diagnostic = (code, detail, item_index, byte_offset, required_bytes)
        super().__init__(self.diagnostic)


def descriptor_source() -> dict:
    parameter_common = {
        "display_unit": "linear",
        "unit": 5,
        "domain": 1,
        "mapping": 1,
        "automation_rate": 2,
        "smoothing": 1,
        "smoothing_samples": 0,
        "readable": True,
        "automatable": True,
        "enum_choices": [],
    }
    return {
        "name": "effect-state-v1",
        "effect_id": "test.state",
        "display_name": "State test",
        "contract_major": 1,
        "contract_minor": 7,
        "state_layout_version": 3,
        "supported_link_mode_bits": 7,
        "parameters": [
            {
                **parameter_common,
                "id": 1,
                "display_name": "Shared",
                "channel_policy": 1,
                "minimum_bits": "bf800000",
                "maximum_bits": "3f800000",
                "default_bits": "00000000",
            },
            {
                **parameter_common,
                "id": 2,
                "display_name": "Per lane",
                "channel_policy": 2,
                "minimum_bits": "c0000000",
                "maximum_bits": "40000000",
                "default_bits": "00000000",
            },
        ],
        "ports": [
            {"id": "main-in", "role": 1, "required": True, "layout": 1},
            {"id": "main-out", "role": 2, "required": True, "layout": 1},
            {"id": "detector", "role": 3, "required": False, "layout": 1},
        ],
        "qualities": [
            {
                "quality": 2,
                "sample_rate": rate,
                "latency_samples": 9,
                "tail_kind": 1,
                "tail_samples": 17,
                "common_state_bytes": 3,
                "left_state_bytes": 5,
                "right_state_bytes": 5,
                "scratch_fixed_bytes": 11,
                "scratch_bytes_per_frame": 2,
            }
            for rate in (44100, 48000, 88200, 96000)
        ],
    }


def source_definition() -> dict:
    return {
        "schema": "miso.effect-state.source.v1",
        "descriptor": descriptor_source(),
        "replay": {
            "sample_rate": 48000,
            "quantum": 8,
            "quality": 2,
            "bypass": True,
            "link_mode": 2,
            "sidechain_kind": 2,
            "sidechain_id": "detector",
            "sidechain_required": False,
            "initial_values": [
                {"parameter_index": 0, "channel": 3, "value_bits": "3e800000"},
                {"parameter_index": 1, "channel": 1, "value_bits": "bf000000"},
                {"parameter_index": 1, "channel": 2, "value_bits": "3fc00000"},
            ],
            "maximum_total_state_bytes": 64,
            "maximum_scratch_bytes": 128,
            "maximum_automation_spans_per_block": 23,
        },
        "payload": {
            "common_hex": "636f6d",
            "left_hex": "6c65667421",
            "right_hex": "7269676874",
        },
    }


def u16(data: bytes | bytearray, offset: int) -> int:
    return struct.unpack_from("<H", data, offset)[0]


def u32(data: bytes | bytearray, offset: int) -> int:
    return struct.unpack_from("<I", data, offset)[0]


def u64(data: bytes | bytearray, offset: int) -> int:
    return struct.unpack_from("<Q", data, offset)[0]


def put16(data: bytearray, offset: int, value: int) -> None:
    struct.pack_into("<H", data, offset, value)


def put32(data: bytearray, offset: int, value: int) -> None:
    struct.pack_into("<I", data, offset, value)


def put64(data: bytearray, offset: int, value: int) -> None:
    struct.pack_into("<Q", data, offset, value)


def float_from_bits(bits: int) -> float:
    return struct.unpack("<f", struct.pack("<I", bits))[0]


def state_digest(data: bytes | bytearray) -> bytes:
    hasher = hashlib.sha256()
    hasher.update(DIGEST_DOMAIN)
    hasher.update(struct.pack("<Q", len(data)))
    hasher.update(data[:56])
    hasher.update(bytes(32))
    hasher.update(data[88:])
    return hasher.digest()


def encode(source: dict) -> tuple[bytes, bytes, bytes]:
    descriptor = source["descriptor"]
    descriptor_wire = DESCRIPTOR_REFERENCE.encode(descriptor)
    DESCRIPTOR_REFERENCE.verify(descriptor_wire)
    identity = DESCRIPTOR_REFERENCE.identity(descriptor_wire)
    replay = source["replay"]
    quality = next(
        row
        for row in descriptor["qualities"]
        if row["quality"] == replay["quality"] and row["sample_rate"] == replay["sample_rate"]
    )
    effect_id = descriptor["effect_id"].encode("ascii")
    sidechain_id = replay["sidechain_id"].encode("ascii")
    initial_values = replay["initial_values"]
    common = bytes.fromhex(source["payload"]["common_hex"])
    left = bytes.fromhex(source["payload"]["left_hex"])
    right = bytes.fromhex(source["payload"]["right_hex"])
    assert len(common) == quality["common_state_bytes"]
    assert len(left) == quality["left_state_bytes"]
    assert len(right) == quality["right_state_bytes"]
    string_end = HEADER + len(effect_id) + len(sidechain_id)
    initial_start = (string_end + 7) & ~7
    initial_bytes = len(initial_values) * INITIAL
    payload_start = initial_start + initial_bytes
    payload_bytes = len(common) + len(left) + len(right)
    total = payload_start + payload_bytes
    data = bytearray(total)
    data[:8] = MAGIC
    put16(data, 8, 1)
    put16(data, 10, HEADER)
    put64(data, 16, total)
    data[24:56] = identity
    put16(data, 88, descriptor["contract_major"])
    put16(data, 90, descriptor["contract_minor"])
    put32(data, 92, descriptor["state_layout_version"])
    put32(data, 96, replay["sample_rate"])
    put32(data, 100, replay["quantum"])
    put32(data, 104, replay["quality"])
    put32(data, 108, int(replay["bypass"]))
    put32(data, 112, replay["link_mode"])
    put32(data, 116, replay["sidechain_kind"])
    put32(data, 120, int(replay["sidechain_required"]))
    put32(data, 124, len(effect_id))
    put32(data, 128, len(sidechain_id))
    put32(data, 132, len(initial_values))
    put64(data, 136, quality["latency_samples"])
    put32(data, 144, quality["tail_kind"])
    put64(data, 152, quality["tail_samples"])
    put32(data, 160, len(common))
    put32(data, 164, len(left))
    put32(data, 168, len(right))
    scratch = quality["scratch_fixed_bytes"] + quality["scratch_bytes_per_frame"] * replay["quantum"]
    put64(data, 176, scratch)
    put32(data, 184, replay["maximum_automation_spans_per_block"])
    put32(data, 188, initial_bytes)
    put64(data, 192, replay["maximum_total_state_bytes"])
    put64(data, 200, replay["maximum_scratch_bytes"])
    put32(data, 208, replay["maximum_automation_spans_per_block"])
    put64(data, 216, payload_bytes)
    data[HEADER : HEADER + len(effect_id)] = effect_id
    data[HEADER + len(effect_id) : string_end] = sidechain_id
    for index, value in enumerate(initial_values):
        record = initial_start + index * INITIAL
        put32(data, record, value["parameter_index"])
        put32(data, record + 4, value["channel"])
        put32(data, record + 8, int(value["value_bits"], 16))
    data[payload_start : payload_start + len(common)] = common
    data[payload_start + len(common) : payload_start + len(common) + len(left)] = left
    data[payload_start + len(common) + len(left) :] = right
    data[56:88] = state_digest(data)
    return descriptor_wire, identity, bytes(data)


def valid_id(data: bytes) -> bool:
    return (
        1 <= len(data) <= 127
        and 0x61 <= data[0] <= 0x7A
        and all(
            0x61 <= byte <= 0x7A or 0x30 <= byte <= 0x39 or byte in b"._-"
            for byte in data[1:]
        )
    )


def verify(data: bytes, identity: bytes) -> dict:
    if len(data) < HEADER:
        raise StateError(3, byte_offset=len(data))
    if data[:8] != MAGIC:
        offset = next((index for index, pair in enumerate(zip(data[:8], MAGIC)) if pair[0] != pair[1]), 0)
        raise StateError(3, byte_offset=offset)
    if u16(data, 8) != 1:
        raise StateError(3, byte_offset=8)
    if u16(data, 10) != HEADER:
        raise StateError(3, byte_offset=10)
    if u32(data, 92) == 0:
        raise StateError(3, byte_offset=92)
    for offset, size in ((12, 4), (148, 4), (172, 4), (212, 4)):
        for index, byte in enumerate(data[offset : offset + size]):
            if byte:
                raise StateError(5, byte_offset=offset + index)
    if u64(data, 16) != len(data):
        raise StateError(4, byte_offset=16)
    effect_len = u32(data, 124)
    sidechain_len = u32(data, 128)
    count = u32(data, 132)
    payload = u32(data, 160) + u32(data, 164) + u32(data, 168)
    if u32(data, 188) != count * INITIAL:
        raise StateError(4, byte_offset=188)
    if u64(data, 216) != payload:
        raise StateError(4, byte_offset=216)
    effect_end = HEADER + effect_len
    sidechain_end = effect_end + sidechain_len
    initial_start = (sidechain_end + 7) & ~7
    initial_end = initial_start + count * INITIAL
    total = initial_end + payload
    if total != len(data):
        raise StateError(4, byte_offset=16)
    for index, byte in enumerate(data[sidechain_end:initial_start]):
        if byte:
            raise StateError(4, byte_offset=sidechain_end + index)
    for index in range(count):
        record = initial_start + index * INITIAL
        if u32(data, record + 12):
            raise StateError(5, item_index=index, byte_offset=record + 12)
    if u32(data, 104) not in (1, 2, 3):
        raise StateError(6, byte_offset=104)
    if u32(data, 108) not in (0, 1):
        raise StateError(6, byte_offset=108)
    if u32(data, 112) not in (1, 2, 3):
        raise StateError(6, byte_offset=112)
    sidechain_kind = u32(data, 116)
    required = u32(data, 120)
    if sidechain_kind > 2:
        raise StateError(6, byte_offset=116)
    if required not in (0, 1):
        raise StateError(6, byte_offset=120)
    if (sidechain_kind == 0 and (sidechain_len or required)) or (
        sidechain_kind == 1 and (not sidechain_len or required)
    ) or (sidechain_kind == 2 and not sidechain_len):
        raise StateError(6, byte_offset=116)
    tail_kind = u32(data, 144)
    if tail_kind not in (1, 2):
        raise StateError(6, byte_offset=144)
    if tail_kind == 2 and u64(data, 152):
        raise StateError(6, byte_offset=152)
    if not valid_id(data[HEADER:effect_end]):
        raise StateError(8, byte_offset=124)
    if sidechain_kind and not valid_id(data[effect_end:sidechain_end]):
        raise StateError(8, byte_offset=128)
    prior = None
    for index in range(count):
        record = initial_start + index * INITIAL
        parameter = u32(data, record)
        channel = u32(data, record + 4)
        if channel not in (1, 2, 3):
            raise StateError(6, item_index=index, byte_offset=record + 4)
        key = (parameter, channel)
        if prior is not None and prior >= key:
            raise StateError(7, item_index=index, byte_offset=record)
        prior = key
        bits = u32(data, record + 8)
        if bits == 0x80000000 or not math.isfinite(float_from_bits(bits)):
            raise StateError(12, item_index=index, byte_offset=record + 8)
    if state_digest(data) != data[56:88]:
        raise StateError(10, byte_offset=56)
    if data[24:56] != identity:
        raise StateError(9, detail=3 << 16)
    return {
        "effect_id": data[HEADER:effect_end].decode("ascii"),
        "initial_start": initial_start,
        "initial_count": count,
        "payload_start": initial_end,
        "total": total,
    }


def refreshed(data: bytearray) -> bytes:
    data[56:88] = state_digest(data)
    return bytes(data)


def mutation_diagnostics(state: bytes, identity: bytes) -> list[tuple[str, tuple[int, int, int, int, int]]]:
    view = verify(state, identity)
    initial = view["initial_start"]
    cases: list[tuple[str, bytes]] = []
    cases.append(("truncated-header", state[: HEADER - 1]))
    mutated = bytearray(state); mutated[0] ^= 1; cases.append(("magic", bytes(mutated)))
    mutated = bytearray(state); mutated[12] = 1; cases.append(("reserved-flags", bytes(mutated)))
    mutated = bytearray(state); put64(mutated, 16, len(state) + 1); cases.append(("total-length", bytes(mutated)))
    mutated = bytearray(state); put32(mutated, initial + 12, 1); cases.append(("initial-reserved", bytes(mutated)))
    mutated = bytearray(state); put32(mutated, 104, 99); cases.append(("quality-enum", bytes(mutated)))
    mutated = bytearray(state); mutated[HEADER] = ord("T"); cases.append(("effect-text", bytes(mutated)))
    mutated = bytearray(state); put32(mutated, initial + INITIAL + 4, 3); cases.append(("initial-order", bytes(mutated)))
    mutated = bytearray(state); mutated[56] ^= 1; cases.append(("digest", bytes(mutated)))
    mutated = bytearray(state); mutated[24] ^= 1; cases.append(("descriptor-identity", refreshed(mutated)))
    diagnostics = []
    for name, candidate in cases:
        try:
            verify(candidate, identity)
        except StateError as error:
            diagnostics.append((name, error.diagnostic))
        else:
            raise AssertionError(f"mutation accepted: {name}")
    return diagnostics


def wrapped_hex(data: bytes) -> str:
    encoded = data.hex()
    return "\n".join(encoded[index : index + 64] for index in range(0, len(encoded), 64)) + "\n"


def expected_files() -> dict[str, bytes]:
    source = source_definition()
    descriptor_wire, identity, state = encode(source)
    view = verify(state, identity)
    diagnostics = mutation_diagnostics(state, identity)
    source_bytes = (json.dumps(source, indent=2, sort_keys=True) + "\n").encode("utf-8")
    expected = {
        "schema": "miso.effect-state.expected.v1",
        "descriptor_identity_hex": identity.hex(),
        "descriptor_wire_bytes": len(descriptor_wire),
        "descriptor_wire_sha256": hashlib.sha256(descriptor_wire).hexdigest(),
        "header_bytes": HEADER,
        "initial_record_bytes": INITIAL,
        "initial_start": view["initial_start"],
        "initial_count": view["initial_count"],
        "payload_start": view["payload_start"],
        "state_bytes": len(state),
        "state_digest_hex": state[56:88].hex(),
        "state_sha256": hashlib.sha256(state).hexdigest(),
    }
    diagnostic_text = "name\tcode\tdetail\titem_index\tbyte_offset\trequired_bytes\n" + "".join(
        f"{name}\t{diagnostic[0]}\t{diagnostic[1]}\t{diagnostic[2]}\t{diagnostic[3]}\t{diagnostic[4]}\n"
        for name, diagnostic in diagnostics
    )
    files = {
        "canonical.source.json": source_bytes,
        "canonical.descriptor.wire.hex": wrapped_hex(descriptor_wire).encode("ascii"),
        "canonical.descriptor.identity.hex": (identity.hex() + "\n").encode("ascii"),
        "canonical.state.bin": state,
        "canonical.state.hex": wrapped_hex(state).encode("ascii"),
        "canonical.state.digest.hex": (state[56:88].hex() + "\n").encode("ascii"),
        "canonical.expected.json": (json.dumps(expected, indent=2, sort_keys=True) + "\n").encode("utf-8"),
        "canonical.diagnostics.tsv": diagnostic_text.encode("ascii"),
    }
    manifest_rows = [
        f"{hashlib.sha256(content).hexdigest()}  fixtures/effect-state/v1/{name}"
        for name, content in sorted(files.items())
    ]
    files["MANIFEST.sha256"] = ("\n".join(manifest_rows) + "\n").encode("ascii")
    return files


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    arguments = parser.parse_args()
    files = expected_files()
    if arguments.write:
        FIXTURES.mkdir(parents=True, exist_ok=True)
        for name, content in files.items():
            (FIXTURES / name).write_bytes(content)
    else:
        actual_names = sorted(path.name for path in FIXTURES.iterdir() if path.is_file())
        if actual_names != sorted(files):
            raise SystemExit("effect state V1 fixture membership mismatch")
        for name, expected in files.items():
            path = FIXTURES / name
            if path.read_bytes() != expected:
                raise SystemExit(f"effect state V1 reference mismatch: {path}")
    print("effect state V1 independent reference: ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
