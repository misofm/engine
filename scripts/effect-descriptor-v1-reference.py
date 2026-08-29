#!/usr/bin/env python3
"""Independent stdlib-only V1 effect-descriptor encoder, verifier, and identity check."""

from __future__ import annotations

import hashlib
import json
import math
import pathlib
import struct
import sys
import unicodedata

MAGIC = b"MISOEFD1"
HEADER = 96
PARAMETER = 80
PORT = 24
QUALITY = 64
CHOICE = 16
OBSERVATION = 32
LIMIT = 1 << 20
UNAVAILABLE = 0xFFFFFFFF
DOMAIN = b"miso.engine.effect-descriptor.identity.v1\0"
LAUNCH_RATES = {44100, 48000, 88200, 96000}
EXTENDED_RATES = {176400, 192000, 352800, 384000}


class WireError(Exception):
    def __init__(self, code: int, offset: int = UNAVAILABLE, index: int = UNAVAILABLE):
        super().__init__((code, offset, index, 0))
        self.diagnostic = (code, offset, index, 0)


def fail(condition: bool, code: int, offset: int, index: int = UNAVAILABLE) -> None:
    if condition:
        raise WireError(code, offset, index)


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


def float_value(bits: int) -> float:
    return struct.unpack("<f", struct.pack("<I", bits))[0]


def canonical_float(bits: int) -> bool:
    return bits != 0x80000000 and math.isfinite(float_value(bits))


def valid_id(value: str) -> bool:
    raw = value.encode("utf-8")
    return (
        1 <= len(raw) <= 127
        and 0x61 <= raw[0] <= 0x7A
        and all(
            0x61 <= byte <= 0x7A
            or 0x30 <= byte <= 0x39
            or byte in b"._-"
            for byte in raw[1:]
        )
    )


def valid_text(value: str) -> bool:
    return bool(value) and len(value.encode("utf-8")) <= 255 and all(
        unicodedata.category(character) != "Cc" for character in value
    )


def bits(source: dict, name: str) -> int | None:
    value = source[name]
    return None if value is None else int(value, 16)


def validate_source(source: dict) -> None:
    fail(source["contract_major"] != 1, 13, 20)
    fail(source["state_layout_version"] == 0, 13, 24)
    fail(not valid_id(source["effect_id"]), 13, 32)
    fail(not valid_text(source["display_name"]), 13, 32)
    link_bits = source["supported_link_mode_bits"]
    fail(link_bits & ~7 != 0 or link_bits & 1 == 0, 7, 28)
    prior_id = 0
    seen_ids: set[int] = set()
    for index, parameter in enumerate(source["parameters"]):
        record = HEADER + index * PARAMETER
        identifier = parameter["id"]
        fail(identifier == 0 or identifier in seen_ids, 13, record, index)
        fail(identifier <= prior_id, 13, record, index)
        seen_ids.add(identifier)
        prior_id = identifier
        fail(parameter["unit"] not in range(1, 7), 7, record + 4, index)
        fail(parameter["domain"] not in range(1, 4), 7, record + 8, index)
        fail(parameter["mapping"] not in range(1, 5), 7, record + 12, index)
        fail(parameter["automation_rate"] not in range(1, 4), 7, record + 16, index)
        fail(parameter["channel_policy"] not in range(1, 3), 7, record + 20, index)
        fail(parameter["smoothing"] not in range(1, 4), 7, record + 24, index)
        fail(not valid_text(parameter["display_name"]), 13, record + 4, index)
        fail(not valid_text(parameter["display_unit"]), 13, record + 4, index)
        default = bits(parameter, "default_bits")
        minimum = bits(parameter, "minimum_bits")
        maximum = bits(parameter, "maximum_bits")
        fail(not canonical_float(default), 12, record + 44, index)
        automatable = parameter["automatable"]
        smoothing = parameter["smoothing"]
        fail(parameter["automation_rate"] == 3 and (automatable or smoothing != 1), 13, record + 4, index)
        fail(parameter["automation_rate"] != 3 and not automatable, 13, record + 4, index)
        fail((smoothing == 1) != (parameter["smoothing_samples"] == 0), 13, record + 4, index)
        domain = parameter["domain"]
        mapping = parameter["mapping"]
        choices = parameter["enum_choices"]
        if domain == 1:
            fail(minimum is None or maximum is None or choices, 13, record + 4, index)
            fail(not canonical_float(minimum) or not canonical_float(maximum), 12, record + 36, index)
            low, high, default_value = map(float_value, (minimum, maximum, default))
            fail(not low < high or not low <= default_value <= high, 13, record + 4, index)
            fail(mapping not in (1, 2, 3) or (mapping == 2 and low <= 0.0), 13, record + 4, index)
        elif domain == 2:
            fail(minimum is not None or maximum is not None or choices or mapping != 4, 13, record + 4, index)
            fail(default not in (0, 0x3F800000), 13, record + 4, index)
        else:
            fail(minimum is not None or maximum is not None or mapping != 4 or len(choices) < 2, 13, record + 4, index)
            values = [int(choice["value_bits"], 16) for choice in choices]
            labels = [choice["label"] for choice in choices]
            fail(any(not canonical_float(value) for value in values), 12, record + 4, index)
            fail(any(float_value(a) >= float_value(b) for a, b in zip(values, values[1:])), 13, record + 4, index)
            fail(any(not valid_text(label) for label in labels) or len(set(labels)) != len(labels), 13, record + 4, index)
            fail(default not in values, 13, record + 4, index)

    port_offset = HEADER + len(source["parameters"]) * PARAMETER
    canonical_ports = sorted(source["ports"], key=lambda port: (port["role"], port["id"].encode()))
    fail(len(canonical_ports) not in (2, 3), 13, port_offset)
    seen_ports: set[str] = set()
    roles = []
    for index, port in enumerate(canonical_ports):
        record = port_offset + index * PORT
        fail(not valid_id(port["id"]) or port["id"] in seen_ports, 13, record, index)
        seen_ports.add(port["id"])
        fail(port["role"] not in (1, 2, 3) or port["layout"] != 1, 7, record + 8, index)
        fail(type(port["required"]) is not bool, 7, record + 12, index)
        roles.append((port["id"], port["role"], port["required"]))
    fail(("main-in", 1, True) not in roles or ("main-out", 2, True) not in roles, 13, port_offset)
    sidechains = [role for role in roles if role[1] == 3]
    fail(len(sidechains) > 1 or any(role[0] in ("main-in", "main-out") for role in sidechains), 13, port_offset)

    qualities = source["qualities"]
    quality_offset = port_offset + len(canonical_ports) * PORT
    fail(any((row["quality"], row["sample_rate"]) >= (next_row["quality"], next_row["sample_rate"])
             for row, next_row in zip(qualities, qualities[1:])), 13, quality_offset)
    by_quality: dict[int, set[int]] = {}
    for index, row in enumerate(qualities):
        record = quality_offset + index * QUALITY
        fail(row["quality"] not in (1, 2, 3), 7, record, index)
        fail(row["sample_rate"] not in LAUNCH_RATES | EXTENDED_RATES, 13, record + 4, index)
        fail(row["tail_kind"] not in (1, 2), 7, record + 16, index)
        fail(row["tail_kind"] == 2 and row["tail_samples"] != 0, 7, record + 24, index)
        fail(row["left_state_bytes"] != row["right_state_bytes"], 13, record + 36, index)
        by_quality.setdefault(row["quality"], set()).add(row["sample_rate"])
    fail(2 not in by_quality or any(not LAUNCH_RATES <= rates for rates in by_quality.values()), 13, quality_offset)

    # Issue #143: the declared observation menu. `get` with a default keeps every pre-#143 source
    # document valid unchanged, which is the same statement as "a zero-tap descriptor's bytes do
    # not move".
    observations = source.get("observations", [])
    choice_count = sum(len(parameter["enum_choices"]) for parameter in source["parameters"])
    observation_offset = quality_offset + len(qualities) * QUALITY + choice_count * CHOICE
    per_lane_state = all(row["left_state_bytes"] > 0 for row in qualities)
    prior_observation = 0
    seen_observations: set[int] = set()
    for index, observation in enumerate(observations):
        record = observation_offset + index * OBSERVATION
        identifier = observation["id"]
        fail(identifier == 0 or identifier in seen_observations, 13, record, index)
        fail(identifier <= prior_observation, 13, record, index)
        seen_observations.add(identifier)
        prior_observation = identifier
        fail(observation["kind"] not in (1,), 7, record + 4, index)
        fail(observation["unit"] not in range(1, 7), 7, record + 5, index)
        fail(observation["cost"] not in range(1, 3), 7, record + 6, index)
        fail(observation["cadence"] not in range(1, 3), 7, record + 7, index)
        fail(observation["fold"] not in range(1, 3), 7, record + 8, index)
        fail(observation["channels"] not in range(1, 3), 7, record + 9, index)
        fail(not valid_text(observation["display_name"]), 13, record + 4, index)
        fail(not valid_text(observation["display_unit"]), 13, record + 4, index)
        minimum = int(observation["minimum_bits"], 16)
        maximum = int(observation["maximum_bits"], 16)
        fail(not canonical_float(minimum) or not canonical_float(maximum), 12, record + 12, index)
        fail(float_value(minimum) >= float_value(maximum), 13, record + 4, index)
        fail(observation["cost"] == 2 and observation["cadence"] == 1, 13, record + 4, index)
        fail(observation["channels"] == 2 and not per_lane_state, 13, record + 4, index)


def encode(source: dict) -> bytes:
    validate_source(source)
    parameters = source["parameters"]
    ports = sorted(source["ports"], key=lambda port: (port["role"], port["id"].encode()))
    qualities = source["qualities"]
    choices = [choice for parameter in parameters for choice in parameter["enum_choices"]]
    strings = [source["effect_id"], source["display_name"]]
    for parameter in parameters:
        strings.extend((parameter["display_name"], parameter["display_unit"]))
        strings.extend(choice["label"] for choice in parameter["enum_choices"])
    strings.extend(port["id"] for port in ports)
    observations = source.get("observations", [])
    for observation in observations:
        strings.extend((observation["display_name"], observation["display_unit"]))
    string_bytes = sum(len(value.encode()) for value in strings)
    parameter_offset = HEADER
    port_offset = parameter_offset + len(parameters) * PARAMETER
    quality_offset = port_offset + len(ports) * PORT
    choice_offset = quality_offset + len(qualities) * QUALITY
    observation_offset = choice_offset + len(choices) * CHOICE
    string_offset = observation_offset + len(observations) * OBSERVATION
    total = string_offset + string_bytes
    fail(total > LIMIT, 2, 16)
    output = bytearray(total)
    output[:8] = MAGIC
    put16(output, 8, 1)
    put16(output, 10, HEADER)
    put32(output, 16, total)
    put16(output, 20, source["contract_major"])
    put16(output, 22, source["contract_minor"])
    put32(output, 24, source["state_layout_version"])
    put32(output, 28, source["supported_link_mode_bits"])
    for offset, value in ((48, len(parameters)), (52, parameter_offset), (56, len(ports)),
                          (60, port_offset), (64, len(qualities)), (68, quality_offset),
                          (72, len(choices)), (76, choice_offset), (80, string_bytes),
                          (84, string_offset), (88, len(observations)),
                          # Zero when no tap is declared: header bytes 88..96 then stay the eight
                          # zeros a pre-#143 reader demands, so the identity does not move.
                          (92, observation_offset if observations else 0)):
        put32(output, offset, value)
    cursor = string_offset

    def text(value: str) -> tuple[int, int]:
        nonlocal cursor
        raw = value.encode()
        result = (cursor, len(raw))
        output[cursor:cursor + len(raw)] = raw
        cursor += len(raw)
        return result

    for field, value in ((32, source["effect_id"]), (40, source["display_name"])):
        offset, length = text(value)
        put32(output, field, offset)
        put32(output, field + 4, length)
    choice_index = 0
    for index, parameter in enumerate(parameters):
        record = parameter_offset + index * PARAMETER
        scalar_fields = (parameter["id"], parameter["unit"], parameter["domain"],
                         parameter["mapping"], parameter["automation_rate"],
                         parameter["channel_policy"], parameter["smoothing"],
                         parameter["smoothing_samples"])
        for field, value in enumerate(scalar_fields):
            put32(output, record + field * 4, value)
        minimum, maximum = bits(parameter, "minimum_bits"), bits(parameter, "maximum_bits")
        flags = int(parameter["readable"]) | int(parameter["automatable"]) << 1
        flags |= int(minimum is not None) << 2 | int(maximum is not None) << 3
        put32(output, record + 32, flags)
        put32(output, record + 36, minimum or 0)
        put32(output, record + 40, maximum or 0)
        put32(output, record + 44, bits(parameter, "default_bits"))
        put32(output, record + 48, choice_index)
        put32(output, record + 52, len(parameter["enum_choices"]))
        for field, value in ((56, parameter["display_name"]), (64, parameter["display_unit"])):
            offset, length = text(value)
            put32(output, record + field, offset)
            put32(output, record + field + 4, length)
        for choice in parameter["enum_choices"]:
            choice_record = choice_offset + choice_index * CHOICE
            put32(output, choice_record, int(choice["value_bits"], 16))
            offset, length = text(choice["label"])
            put32(output, choice_record + 4, offset)
            put32(output, choice_record + 8, length)
            choice_index += 1
    for index, port in enumerate(ports):
        record = port_offset + index * PORT
        offset, length = text(port["id"])
        for field, value in ((0, offset), (4, length), (8, port["role"]),
                             (12, int(port["required"])), (16, port["layout"])):
            put32(output, record + field, value)
    for index, observation in enumerate(observations):
        record = observation_offset + index * OBSERVATION
        put32(output, record, observation["id"])
        name_offset, name_length = text(observation["display_name"])
        unit_offset, unit_length = text(observation["display_unit"])
        output[record + 4] = observation["kind"]
        output[record + 5] = observation["unit"]
        output[record + 6] = observation["cost"]
        output[record + 7] = observation["cadence"]
        output[record + 8] = observation["fold"]
        output[record + 9] = observation["channels"]
        output[record + 10] = name_length
        output[record + 11] = unit_length
        put32(output, record + 12, int(observation["minimum_bits"], 16))
        put32(output, record + 16, int(observation["maximum_bits"], 16))
        put32(output, record + 20, name_offset)
        put32(output, record + 24, unit_offset)
    for index, quality in enumerate(qualities):
        record = quality_offset + index * QUALITY
        for field, value in ((0, quality["quality"]), (4, quality["sample_rate"]),
                             (16, quality["tail_kind"]), (32, quality["common_state_bytes"]),
                             (36, quality["left_state_bytes"]), (40, quality["right_state_bytes"])):
            put32(output, record + field, value)
        for field, value in ((8, quality["latency_samples"]), (24, quality["tail_samples"]),
                             (48, quality["scratch_fixed_bytes"]),
                             (56, quality["scratch_bytes_per_frame"])):
            put64(output, record + field, value)
    assert cursor == total
    return bytes(output)


def verify(data: bytes, maximum: int = LIMIT) -> tuple[int, ...]:
    fail(maximum == 0 or len(data) > maximum or len(data) > 0xFFFFFFFF, 2, 16)
    fail(len(data) < HEADER, 4, len(data))
    if data[:8] != MAGIC:
        offset = next((index for index, (actual, expected) in enumerate(zip(data[:8], MAGIC)) if actual != expected), 0)
        raise WireError(4, offset)
    fail(u16(data, 8) != 1, 4, 8)
    fail(u16(data, 10) != HEADER, 4, 10)
    fail(u32(data, 16) != len(data), 5, 16)
    counts = (u32(data, 48), u32(data, 56), u32(data, 64), u32(data, 72), u32(data, 88))
    def add_at(left: int, right: int, offset: int) -> int:
        fail(left + right > 0xFFFFFFFF, 14, offset)
        return left + right

    def mul_at(left: int, right: int, offset: int) -> int:
        fail(left * right > 0xFFFFFFFF, 14, offset)
        return left * right

    parameter_offset = HEADER
    port_offset = add_at(parameter_offset, mul_at(counts[0], PARAMETER, 48), 48)
    quality_offset = add_at(port_offset, mul_at(counts[1], PORT, 56), 56)
    choice_offset = add_at(quality_offset, mul_at(counts[2], QUALITY, 64), 64)
    observation_offset = add_at(choice_offset, mul_at(counts[3], CHOICE, 72), 72)
    string_offset = add_at(observation_offset, mul_at(counts[4], OBSERVATION, 88), 88)
    fail(add_at(string_offset, u32(data, 80), 80) != len(data), 5, 80)
    fail(u32(data, 12) != 0, 6, 12)
    if counts[4] == 0:
        for offset in range(88, 96):
            fail(data[offset] != 0, 6, offset)
    for index in range(counts[0]):
        record = parameter_offset + index * PARAMETER
        flags = u32(data, record + 32)
        fail(flags & ~15 != 0, 8, record + 32, index)
        fail(flags & 4 == 0 and u32(data, record + 36) != 0, 8, record + 36, index)
        fail(flags & 8 == 0 and u32(data, record + 40) != 0, 8, record + 40, index)
        for field in (72, 76):
            fail(u32(data, record + field) != 0, 6, record + field, index)
    for index in range(counts[1]):
        fail(u32(data, port_offset + index * PORT + 20) != 0, 6, port_offset + index * PORT + 20, index)
    for index in range(counts[2]):
        record = quality_offset + index * QUALITY
        for field in (20, 44):
            fail(u32(data, record + field) != 0, 6, record + field, index)
    for index in range(counts[3]):
        fail(u32(data, choice_offset + index * CHOICE + 12) != 0, 6, choice_offset + index * CHOICE + 12, index)
    for index in range(counts[4]):
        record = observation_offset + index * OBSERVATION
        fail(u32(data, record + 28) != 0, 6, record + 28, index)
    cursor = string_offset

    def take(field: int, index: int = UNAVAILABLE) -> bytes:
        nonlocal cursor
        offset, length = u32(data, field), u32(data, field + 4)
        fail(offset != cursor, 10, field, index)
        end = offset + length
        fail(end > len(data), 5, field, index)
        cursor = end
        return data[offset:end]

    effect_id, display_name = take(32), take(40)
    for field, expected in ((52, parameter_offset), (60, port_offset), (68, quality_offset),
                            (76, choice_offset), (84, string_offset),
                            (92, observation_offset if counts[4] else 0)):
        fail(u32(data, field) != expected, 10, field)
    choice_cursor = 0
    prior_parameter = None
    for index in range(counts[0]):
        record = parameter_offset + index * PARAMETER
        identifier = u32(data, record)
        fail(prior_parameter is not None and identifier <= prior_parameter, 9, record, index)
        prior_parameter = identifier
        fail(u32(data, record + 48) != choice_cursor, 10, record + 48, index)
        choice_count = u32(data, record + 52)
        fail(choice_cursor + choice_count > counts[3], 5, record + 52, index)
        take(record + 56, index)
        take(record + 64, index)
        prior_choice = None
        for choice_index in range(choice_cursor, choice_cursor + choice_count):
            choice_record = choice_offset + choice_index * CHOICE
            value = float_value(u32(data, choice_record))
            fail(prior_choice is not None and math.isfinite(value) and value <= prior_choice, 9, choice_record, choice_index)
            if math.isfinite(value):
                prior_choice = value
            take(choice_record + 4, choice_index)
        choice_cursor += choice_count
    fail(choice_cursor != counts[3], 10, 72)
    prior_port = None
    port_ids = []
    for index in range(counts[1]):
        record = port_offset + index * PORT
        identifier = take(record, index)
        key = (u32(data, record + 8), identifier)
        fail(prior_port is not None and prior_port >= key, 9, record + 8, index)
        prior_port = key
        port_ids.append(identifier)
    prior_observation = None
    observation_texts = []
    for index in range(counts[4]):
        record = observation_offset + index * OBSERVATION
        identifier = u32(data, record)
        fail(prior_observation is not None and identifier <= prior_observation, 9, record, index)
        prior_observation = identifier
        for field, length_field in ((20, 10), (24, 11)):
            offset, length = u32(data, record + field), data[record + length_field]
            fail(offset != cursor, 10, record + field, index)
            fail(offset + length > len(data), 5, record + field, index)
            cursor = offset + length
            observation_texts.append((data[offset:offset + length], record + field, index))
    prior_quality = None
    for index in range(counts[2]):
        record = quality_offset + index * QUALITY
        key = (u32(data, record), u32(data, record + 4))
        fail(prior_quality is not None and prior_quality >= key, 9, record, index)
        prior_quality = key
    fail(cursor != len(data), 10, 80)
    fail(u32(data, 28) & ~7 != 0 or u32(data, 28) & 1 == 0, 7, 28)
    enum_fields = ((4, range(1, 7)), (8, range(1, 4)), (12, range(1, 5)),
                   (16, range(1, 4)), (20, range(1, 3)), (24, range(1, 4)))
    for index in range(counts[0]):
        record = parameter_offset + index * PARAMETER
        for field, accepted in enum_fields:
            fail(u32(data, record + field) not in accepted, 7, record + field, index)
    for index in range(counts[1]):
        record = port_offset + index * PORT
        fail(u32(data, record + 8) not in (1, 2, 3), 7, record + 8, index)
        fail(u32(data, record + 12) not in (0, 1), 7, record + 12, index)
        fail(u32(data, record + 16) != 1, 7, record + 16, index)
    for index in range(counts[2]):
        record = quality_offset + index * QUALITY
        fail(u32(data, record) not in (1, 2, 3), 7, record, index)
        tail = u32(data, record + 16)
        fail(tail not in (1, 2), 7, record + 16, index)
        fail(tail == 2 and u64(data, record + 24) != 0, 7, record + 24, index)
    for index in range(counts[4]):
        record = observation_offset + index * OBSERVATION
        for field, accepted in ((4, (1,)), (5, range(1, 7)), (6, range(1, 3)),
                                (7, range(1, 3)), (8, range(1, 3)), (9, range(1, 3))):
            fail(data[record + field] not in accepted, 7, record + field, index)
    texts = [(effect_id, 32, UNAVAILABLE), (display_name, 40, UNAVAILABLE)]
    for index in range(counts[0]):
        record = parameter_offset + index * PARAMETER
        for field in (56, 64):
            texts.append((data[u32(data, record + field):u32(data, record + field) + u32(data, record + field + 4)], record + field, index))
    for raw, field, index in texts:
        try:
            value = raw.decode("utf-8")
        except UnicodeDecodeError as error:
            raise WireError(11, field, index) from error
        fail(not valid_text(value), 11, field, index)
    fail(not valid_id(effect_id.decode()), 11, 32)
    for index, raw in enumerate(port_ids):
        try:
            value = raw.decode("utf-8")
        except UnicodeDecodeError as error:
            raise WireError(11, port_offset + index * PORT, index) from error
        fail(not valid_id(value), 11, port_offset + index * PORT, index)
    choice_texts = []
    for index in range(counts[3]):
        record = choice_offset + index * CHOICE
        choice_texts.append((data[u32(data, record + 4):u32(data, record + 4) + u32(data, record + 8)], record + 4, index))
    for raw, field, index in choice_texts:
        try:
            value = raw.decode("utf-8")
        except UnicodeDecodeError as error:
            raise WireError(11, field, index) from error
        fail(not valid_text(value), 11, field, index)
    for raw, field, index in observation_texts:
        try:
            value = raw.decode("utf-8")
        except UnicodeDecodeError as error:
            raise WireError(11, field, index) from error
        fail(not valid_text(value), 11, field, index)
    for index in range(counts[0]):
        record = parameter_offset + index * PARAMETER
        flags = u32(data, record + 32)
        for field, present in ((36, flags & 4), (40, flags & 8), (44, True)):
            fail(bool(present) and not canonical_float(u32(data, record + field)), 12, record + field, index)
    for index in range(counts[3]):
        record = choice_offset + index * CHOICE
        fail(not canonical_float(u32(data, record)), 12, record, index)
    for index in range(counts[4]):
        record = observation_offset + index * OBSERVATION
        for field in (12, 16):
            fail(not canonical_float(u32(data, record + field)), 12, record + field, index)
    def decoded_text(field: int) -> str:
        offset, length = u32(data, field), u32(data, field + 4)
        return data[offset:offset + length].decode("utf-8")

    parameters = []
    for index in range(counts[0]):
        record = parameter_offset + index * PARAMETER
        flags = u32(data, record + 32)
        choices = []
        start, count = u32(data, record + 48), u32(data, record + 52)
        for choice_index in range(start, start + count):
            choice_record = choice_offset + choice_index * CHOICE
            choices.append({
                "value_bits": f"{u32(data, choice_record):08x}",
                "label": decoded_text(choice_record + 4),
            })
        parameters.append({
            "id": u32(data, record),
            "display_name": decoded_text(record + 56),
            "display_unit": decoded_text(record + 64),
            "unit": u32(data, record + 4),
            "domain": u32(data, record + 8),
            "mapping": u32(data, record + 12),
            "automation_rate": u32(data, record + 16),
            "channel_policy": u32(data, record + 20),
            "smoothing": u32(data, record + 24),
            "smoothing_samples": u32(data, record + 28),
            "readable": bool(flags & 1),
            "automatable": bool(flags & 2),
            "minimum_bits": f"{u32(data, record + 36):08x}" if flags & 4 else None,
            "maximum_bits": f"{u32(data, record + 40):08x}" if flags & 8 else None,
            "default_bits": f"{u32(data, record + 44):08x}",
            "enum_choices": choices,
        })
    ports = []
    for index in range(counts[1]):
        record = port_offset + index * PORT
        ports.append({
            "id": decoded_text(record),
            "role": u32(data, record + 8),
            "required": bool(u32(data, record + 12)),
            "layout": u32(data, record + 16),
        })
    qualities = []
    for index in range(counts[2]):
        record = quality_offset + index * QUALITY
        qualities.append({
            "quality": u32(data, record),
            "sample_rate": u32(data, record + 4),
            "latency_samples": u64(data, record + 8),
            "tail_kind": u32(data, record + 16),
            "tail_samples": u64(data, record + 24),
            "common_state_bytes": u32(data, record + 32),
            "left_state_bytes": u32(data, record + 36),
            "right_state_bytes": u32(data, record + 40),
            "scratch_fixed_bytes": u64(data, record + 48),
            "scratch_bytes_per_frame": u64(data, record + 56),
        })
    observations = []
    for index in range(counts[4]):
        record = observation_offset + index * OBSERVATION
        name_offset, name_length = u32(data, record + 20), data[record + 10]
        unit_offset, unit_length = u32(data, record + 24), data[record + 11]
        observations.append({
            "id": u32(data, record),
            "display_name": data[name_offset:name_offset + name_length].decode("utf-8"),
            "display_unit": data[unit_offset:unit_offset + unit_length].decode("utf-8"),
            "kind": data[record + 4],
            "unit": data[record + 5],
            "cost": data[record + 6],
            "cadence": data[record + 7],
            "fold": data[record + 8],
            "channels": data[record + 9],
            "minimum_bits": f"{u32(data, record + 12):08x}",
            "maximum_bits": f"{u32(data, record + 16):08x}",
        })
    decoded = {
        "effect_id": effect_id.decode("utf-8"),
        "display_name": display_name.decode("utf-8"),
        "contract_major": u16(data, 20),
        "contract_minor": u16(data, 22),
        "state_layout_version": u32(data, 24),
        "supported_link_mode_bits": u32(data, 28),
        "parameters": parameters,
        "ports": ports,
        "qualities": qualities,
        "observations": observations,
    }
    validate_source(decoded)
    if encode(decoded) != data:
        raise AssertionError("verified descriptor does not decode/re-encode byte-identically")
    return (*counts, u32(data, 24), u32(data, 28))


def identity(data: bytes) -> bytes:
    verify(data)
    return hashlib.sha256(DOMAIN + struct.pack("<Q", len(data)) + data).digest()


def mutation_matrix(data: bytes) -> None:
    parameter_offset, port_offset, quality_offset = u32(data, 52), u32(data, 60), u32(data, 68)
    choice_offset = u32(data, 76)
    cases = [
        ("header", 0, b"X", (4, 0, UNAVAILABLE, 0)),
        ("version", 8, struct.pack("<H", 2), (4, 8, UNAVAILABLE, 0)),
        ("header-size", 10, struct.pack("<H", HEADER + 8), (4, 10, UNAVAILABLE, 0)),
        ("length", 16, struct.pack("<I", len(data) - 1), (5, 16, UNAVAILABLE, 0)),
        ("reserved", 12, b"\x01", (6, 12, UNAVAILABLE, 0)),
        ("header-reserved-tail", 95, b"\x01", (6, 95, UNAVAILABLE, 0)),
        ("flags", parameter_offset + 32, struct.pack("<I", 16), (8, parameter_offset + 32, 0, 0)),
        ("absent-minimum-bits", parameter_offset + 3 * PARAMETER + 36,
         struct.pack("<I", 1), (8, parameter_offset + 3 * PARAMETER + 36, 3, 0)),
        ("parameter-reserved", parameter_offset + 72, struct.pack("<I", 1),
         (6, parameter_offset + 72, 0, 0)),
        ("port-reserved", port_offset + 20, struct.pack("<I", 1),
         (6, port_offset + 20, 0, 0)),
        ("quality-reserved", quality_offset + 20, struct.pack("<I", 1),
         (6, quality_offset + 20, 0, 0)),
        ("choice-reserved", choice_offset + 12, struct.pack("<I", 1),
         (6, choice_offset + 12, 0, 0)),
        ("offset", 52, struct.pack("<I", HEADER + 4), (10, 52, UNAVAILABLE, 0)),
        ("parameter-order", parameter_offset + PARAMETER, struct.pack("<I", 1), (9, parameter_offset + PARAMETER, 1, 0)),
        ("quality-order", quality_offset + QUALITY + 4, struct.pack("<I", 44100),
         (9, quality_offset + QUALITY, 1, 0)),
        ("choice-order", choice_offset + CHOICE, struct.pack("<I", 0xc0000000),
         (9, choice_offset + CHOICE, 1, 0)),
        ("link-missing", 28, struct.pack("<I", 2), (7, 28, UNAVAILABLE, 0)),
        ("link-unknown", 28, struct.pack("<I", 9), (7, 28, UNAVAILABLE, 0)),
        ("unit", parameter_offset + 4, struct.pack("<I", 0), (7, parameter_offset + 4, 0, 0)),
        ("domain", parameter_offset + 8, struct.pack("<I", 0), (7, parameter_offset + 8, 0, 0)),
        ("mapping", parameter_offset + 12, struct.pack("<I", 0), (7, parameter_offset + 12, 0, 0)),
        ("automation", parameter_offset + 16, struct.pack("<I", 0), (7, parameter_offset + 16, 0, 0)),
        ("channel", parameter_offset + 20, struct.pack("<I", 0), (7, parameter_offset + 20, 0, 0)),
        ("smoothing", parameter_offset + 24, struct.pack("<I", 0), (7, parameter_offset + 24, 0, 0)),
        ("port-role", port_offset + 8, struct.pack("<I", 0), (7, port_offset + 8, 0, 0)),
        ("port-bool", port_offset + 12, struct.pack("<I", 2), (7, port_offset + 12, 0, 0)),
        ("port-layout", port_offset + 16, struct.pack("<I", 0), (7, port_offset + 16, 0, 0)),
        ("quality", quality_offset, struct.pack("<I", 0), (7, quality_offset, 0, 0)),
        ("tail", quality_offset + 16, struct.pack("<I", 0), (7, quality_offset + 16, 0, 0)),
        ("infinite-tail-samples", quality_offset + 8 * QUALITY + 24,
         struct.pack("<Q", 1), (7, quality_offset + 8 * QUALITY + 24, 8, 0)),
        ("float", parameter_offset + 44, struct.pack("<I", 0x80000000), (12, parameter_offset + 44, 0, 0)),
        ("float-nan", parameter_offset + 44, struct.pack("<I", 0x7fc00000),
         (12, parameter_offset + 44, 0, 0)),
        ("choice-float-nan", choice_offset, struct.pack("<I", 0x7fc00000),
         (12, choice_offset, 0, 0)),
        ("parameter-semantic", parameter_offset + 44, struct.pack("<I", 0x7f7fffff),
         (13, parameter_offset + 4, 0, 0)),
        ("port-semantic", port_offset + 12, struct.pack("<I", 0),
         (13, port_offset, UNAVAILABLE, 0)),
        ("quality-semantic", quality_offset + 40, struct.pack("<I", 17),
         (13, quality_offset + 36, 0, 0)),
        ("semantic", 20, struct.pack("<H", 2), (13, 20, UNAVAILABLE, 0)),
    ]
    for field in (48, 56, 64, 72, 80):
        cases.append((f"overflow-{field}", field, struct.pack("<I", 0xFFFFFFFF),
                      (14, field, UNAVAILABLE, 0)))
    display = u32(data, 40)
    cases.append(("text", display, b"\x0a", (11, 40, UNAVAILABLE, 0)))
    cases.append(("text-invalid-utf8", display, b"\xff", (11, 40, UNAVAILABLE, 0)))
    effect_id = u32(data, 32)
    cases.append(("string-alias", 40, struct.pack("<I", effect_id), (10, 40, UNAVAILABLE, 0)))
    cases.append(("string-gap", 40, struct.pack("<I", u32(data, 40) + 1),
                  (10, 40, UNAVAILABLE, 0)))
    cases.append(("effect-id-first", effect_id, b"F", (11, 32, UNAVAILABLE, 0)))
    cases.append(("effect-id-rest", effect_id + 7, b"/", (11, 32, UNAVAILABLE, 0)))
    port_id = u32(data, port_offset)
    cases.append(("port-id-first", port_id, b"M", (11, port_offset, 0, 0)))
    cases.append(("port-id-rest", port_id + 4, b"/", (11, port_offset, 0, 0)))
    for name, offset, replacement, expected in cases:
        mutated = bytearray(data)
        mutated[offset:offset + len(replacement)] = replacement
        try:
            verify(bytes(mutated))
        except WireError as error:
            if error.diagnostic != expected:
                raise AssertionError(f"{name}: {error.diagnostic} != {expected}") from error
        else:
            raise AssertionError(f"{name}: mutation accepted")

    port_order = bytearray(data)
    put32(port_order, port_offset + PORT + 8, 1)
    second_port_text = u32(data, port_offset + PORT)
    port_order[second_port_text] = ord("a")
    try:
        verify(bytes(port_order))
    except WireError as error:
        assert error.diagnostic == (9, port_offset + PORT + 8, 1, 0)
    else:
        raise AssertionError("port-order: mutation accepted")

    for link_bits in list(range(256)) + [1 << bit for bit in range(8, 32)] + [
        (1 << bit) | 1 for bit in range(8, 32)
    ]:
        if link_bits in (1, 3, 5, 7):
            continue
        mutated = bytearray(data)
        put32(mutated, 28, link_bits)
        try:
            verify(bytes(mutated))
        except WireError as error:
            assert error.diagnostic == (7, 28, UNAVAILABLE, 0)
        else:
            raise AssertionError(f"link-bits-{link_bits}: mutation accepted")

    for name, mutated, expected in (
        ("truncated", data[:-1], (5, 16, UNAVAILABLE, 0)),
        ("trailing", data + b"\x00", (5, 16, UNAVAILABLE, 0)),
    ):
        try:
            verify(mutated)
        except WireError as error:
            assert error.diagnostic == expected
        else:
            raise AssertionError(f"{name}: mutation accepted")

    flags_before_reserved = bytearray(data)
    put32(flags_before_reserved, parameter_offset + 32, 16)
    put32(flags_before_reserved, parameter_offset + 72, 1)
    try:
        verify(bytes(flags_before_reserved))
    except WireError as error:
        assert error.diagnostic == (8, parameter_offset + 32, 0, 0)
    else:
        raise AssertionError("flags-before-reserved: mutation accepted")

    header_text_before_table = bytearray(data)
    put32(header_text_before_table, 32, u32(data, 32) + 1)
    put32(header_text_before_table, 52, HEADER + 4)
    try:
        verify(bytes(header_text_before_table))
    except WireError as error:
        assert error.diagnostic == (10, 32, UNAVAILABLE, 0)
    else:
        raise AssertionError("header-text-before-table: mutation accepted")

    port_before_choice = bytearray(data)
    port_text = u32(data, port_offset)
    port_before_choice[port_text] = ord("A")
    choice_offset = u32(data, 76)
    choice_text = u32(data, choice_offset + 4)
    port_before_choice[choice_text] = ord("\n")
    try:
        verify(bytes(port_before_choice))
    except WireError as error:
        assert error.diagnostic == (11, port_offset, 0, 0)
    else:
        raise AssertionError("port-before-choice-text: mutation accepted")


def observation_mutation_matrix(data: bytes) -> None:
    """Issue #143: every observation-section rule refuses, at its exact byte."""
    observation_offset = u32(data, 92)
    second = observation_offset + OBSERVATION
    cases = [
        ("observation-reserved", observation_offset + 28, struct.pack("<I", 1),
         (6, observation_offset + 28, 0, 0)),
        ("observation-order", second, struct.pack("<I", 1), (9, second, 1, 0)),
        ("observation-kind", observation_offset + 4, b"\x00", (7, observation_offset + 4, 0, 0)),
        ("observation-unit", observation_offset + 5, b"\x07", (7, observation_offset + 5, 0, 0)),
        ("observation-cost", observation_offset + 6, b"\x03", (7, observation_offset + 6, 0, 0)),
        ("observation-cadence", observation_offset + 7, b"\x03", (7, observation_offset + 7, 0, 0)),
        ("observation-fold", observation_offset + 8, b"\x00", (7, observation_offset + 8, 0, 0)),
        ("observation-channels", observation_offset + 9, b"\x00",
         (7, observation_offset + 9, 0, 0)),
        ("observation-float", observation_offset + 12, struct.pack("<I", 0x80000000),
         (12, observation_offset + 12, 0, 0)),
        ("observation-float-nan", observation_offset + 16, struct.pack("<I", 0x7fc00000),
         (12, observation_offset + 16, 0, 0)),
        ("observation-bounds", observation_offset + 16, struct.pack("<I", 0),
         (13, observation_offset + 4, 0, 0)),
        # A `Computed` tap may not claim per-block cadence: that would put an analysis pass on the
        # render thread, which is exactly what the cost split exists to prevent.
        ("observation-computed-per-block", second + 7, b"\x01", (13, second + 4, 1, 0)),
        ("observation-offset", 92, struct.pack("<I", observation_offset + 1),
         (10, 92, UNAVAILABLE, 0)),
        ("observation-count", 88, struct.pack("<I", 3), (5, 80, UNAVAILABLE, 0)),
    ]
    name_offset = u32(data, observation_offset + 20)
    cases.append(("observation-text", name_offset, b"\x0a",
                  (11, observation_offset + 20, 0, 0)))
    cases.append(("observation-string-gap", observation_offset + 20,
                  struct.pack("<I", name_offset + 1), (10, observation_offset + 20, 0, 0)))
    for name, offset, replacement, expected in cases:
        mutated = bytearray(data)
        mutated[offset:offset + len(replacement)] = replacement
        try:
            verify(bytes(mutated))
        except WireError as error:
            if error.diagnostic != expected:
                raise AssertionError(f"{name}: {error.diagnostic} != {expected}") from error
        else:
            raise AssertionError(f"{name}: mutation accepted")


def read_hex(path: pathlib.Path) -> bytes:
    return bytes.fromhex("".join(path.read_text(encoding="ascii").split()))


def check(root: pathlib.Path) -> None:
    fixture = root / "fixtures/effect-descriptor/v1"
    names = ("comprehensive-a", "comprehensive-b", "comprehensive-c")
    manifest_rows = []
    for name in names:
        source_path = fixture / f"{name}.json"
        wire_path = fixture / f"{name}.wire.hex"
        identity_path = fixture / f"{name}.identity.hex"
        source = json.loads(source_path.read_text(encoding="utf-8"))
        wire = encode(source)
        verify(wire)
        expected_wire = read_hex(wire_path)
        expected_identity = read_hex(identity_path)
        assert wire == expected_wire, f"{name}: wire mismatch"
        assert identity(wire) == expected_identity, f"{name}: identity mismatch"
        for path in (source_path, wire_path, identity_path):
            digest = hashlib.sha256(path.read_bytes()).hexdigest()
            relative = path.relative_to(root).as_posix()
            manifest_rows.append((relative, f"{digest}  {relative}"))
    # Issue #143 E10: the tap-bearing total is the zero-tap total plus exactly the observation
    # section and its strings. `comprehensive-c` is `comprehensive-a` with the menu added and
    # nothing else changed (the id and display name are the same byte lengths), so the difference
    # is the formula and not an approximation of it.
    zero_tap = read_hex(fixture / "comprehensive-a.wire.hex")
    tap_bearing = read_hex(fixture / "comprehensive-c.wire.hex")
    source_c = json.loads((fixture / "comprehensive-c.json").read_text(encoding="utf-8"))
    expected_delta = sum(
        OBSERVATION + len(row["display_name"].encode()) + len(row["display_unit"].encode())
        for row in source_c["observations"]
    )
    assert len(tap_bearing) - len(zero_tap) == expected_delta, "observation section formula"
    assert u32(zero_tap, 88) == 0 and u32(zero_tap, 92) == 0, "zero-tap header stays reserved-zero"
    assert any(byte != 0 for byte in tap_bearing[88:96]), "tap-bearing header is nonzero"
    mutation_matrix(read_hex(fixture / "comprehensive-a.wire.hex"))
    observation_mutation_matrix(tap_bearing)
    expected_manifest = "\n".join(row for _, row in sorted(manifest_rows)) + "\n"
    actual_manifest = (fixture / "MANIFEST.sha256").read_text(encoding="ascii")
    assert actual_manifest == expected_manifest, "manifest mismatch"
    print("effect descriptor V1 independent reference: ok")


def emit(root: pathlib.Path) -> None:
    fixture = root / "fixtures/effect-descriptor/v1"
    for name in ("comprehensive-a", "comprehensive-b", "comprehensive-c"):
        source = json.loads((fixture / f"{name}.json").read_text(encoding="utf-8"))
        wire = encode(source)
        verify(wire)
        print(f"[{name}.wire.hex]\n{wire.hex()}\n[{name}.identity.hex]\n{identity(wire).hex()}")


def main() -> None:
    root = pathlib.Path(__file__).resolve().parent.parent
    if sys.argv[1:] == ["--emit"]:
        emit(root)
    elif not sys.argv[1:]:
        check(root)
    else:
        raise SystemExit("usage: effect-descriptor-v1-reference.py [--emit]")


if __name__ == "__main__":
    main()
