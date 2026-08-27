#!/usr/bin/env python3
"""Schema gate for the SDK-facing browser ABI layout (issue #207 E0a).

The Rust emitter derives every offset from public ``repr(C)`` structures. This independent Python
gate holds the shipped JSON to the frozen V1 byte contract. ``--self-test`` mutates each declared
field and constant row, proving that the gate rejects the exact drift a generated SDK must not
silently absorb.
"""

from __future__ import annotations

import argparse
import copy
import json
import pathlib
import sys

SCHEMA = "miso.web.abi-layout.v1"
ABI_VERSION = 0x0001_0000


def fields(*rows: tuple[str, int, str]) -> list[dict[str, object]]:
    return [{"name": name, "offset": offset, "type": ty} for name, offset, ty in rows]


STRUCTURES = {
    "prepareConfig": (192, fields(
        ("structSize", 0, "u32"), ("abiVersion", 4, "u32"),
        ("sampleRateHz", 8, "u32"), ("quantumFrames", 12, "u32"),
        ("sessionTomlBytes", 16, "u32"), ("diagnosticBytes", 20, "u32"),
        ("sourceIdBytes", 24, "u32"), ("maximumSourceChannels", 28, "u32"),
        ("sourceRingFrames", 32, "u32"), ("maximumAutomationSpansPerBlock", 36, "u32"),
        ("maximumTracks", 40, "u64"), ("maximumSources", 48, "u64"),
        ("maximumRoutes", 56, "u64"), ("maximumEffects", 64, "u64"),
        ("maximumGraphSessionPlusPlanBytes", 72, "u64"),
        ("maximumSourceTotalBytes", 80, "u64"),
        ("maximumSourceOverheadBytes", 88, "u64"),
        ("maximumEffectStateBytes", 96, "u64"),
        ("maximumEffectScratchBytes", 104, "u64"),
        ("maximumBuiltinRetainedBytes", 112, "u64"),
        ("maximumHostRetainedBytes", 120, "u64"),
        ("maximumNamedAllocationBytes", 128, "u64"),
        ("maximumMeterStreams", 136, "u64"), ("maximumMeterItems", 144, "u64"),
        ("maximumMeterBytes", 152, "u64"),
        ("consoleCommandQueueRecords", 160, "u64"),
        ("consoleMeterBlocks", 168, "u64"),
        ("consoleObservationTaps", 176, "u64"),
        ("consoleMasterTrackPlusOne", 184, "u64"),
    )),
    "status": (80, fields(
        ("structSize", 0, "u32"), ("abiVersion", 4, "u32"), ("state", 8, "u32"),
        ("lastResult", 12, "u32"), ("backend", 16, "u32"),
        ("sampleRateHz", 20, "u32"), ("quantumFrames", 24, "u32"),
        ("reserved0", 28, "u32"), ("nextAbsoluteSample", 32, "u64"),
        ("renderedQuanta", 40, "u64"), ("reserved", 48, "u64[4]"),
    )),
    "resourceReport": (224, fields(
        ("structSize", 0, "u32"), ("abiVersion", 4, "u32"),
        ("sampleRateHz", 8, "u32"), ("quantumFrames", 12, "u32"),
        ("backend", 16, "u32"), ("reserved0", 20, "u32[3]"),
        ("configBytes", 32, "u64"), ("statusBytes", 40, "u64"),
        ("sessionTomlBytes", 48, "u64"), ("diagnosticBytes", 56, "u64"),
        ("sourceIdBytes", 64, "u64"), ("sourcePcmStagingBytes", 72, "u64"),
        ("outputPcmBytes", 80, "u64"), ("bridgeMetadataBytes", 88, "u64"),
        ("bridgeRetainedBytes", 96, "u64"), ("largestBridgeAllocationBytes", 104, "u64"),
        ("sourceTotalBytes", 112, "u64"), ("sourceOverheadBytes", 120, "u64"),
        ("effectScalarStateBytes", 128, "u64"), ("effectScalarScratchBytes", 136, "u64"),
        ("builtinRetainedBytes", 144, "u64"), ("graphSessionPlusPlanBytes", 152, "u64"),
        ("graphIncrementalPlanBytes", 160, "u64"), ("graphMetadataBytes", 168, "u64"),
        ("graphDelayBytes", 176, "u64"), ("largestNamedAllocationBytes", 184, "u64"),
        ("observationRetainedBytes", 192, "u64"), ("reserved", 200, "u64[3]"),
    )),
    "meterHeader": (64, fields(
        ("structSize", 0, "u32"), ("abiVersion", 4, "u32"), ("trackCount", 8, "u32"),
        ("windows", 12, "u32"), ("firstSample", 16, "u64"), ("endSample", 24, "u64"),
        ("sequence", 32, "u64"), ("masterTrackPlusOne", 40, "u32"),
        ("masterGrPresent", 44, "u32"), ("reserved", 48, "u64[2]"),
    )),
    "commandReport": (48, fields(
        ("structSize", 0, "u32"), ("abiVersion", 4, "u32"), ("result", 8, "u32"),
        ("reason", 12, "u32"), ("rejectedIndex", 16, "u32"), ("admitted", 20, "u32"),
        ("appliedAtSample", 24, "u64"), ("reserved", 32, "u64[2]"),
    )),
}

COMMAND_RECORD = fields(
    ("kind", 0, "u8"), ("rack", 1, "u8"), ("channel", 2, "u8"),
    ("reserved0", 3, "u8"), ("trackIndex", 4, "u32"), ("effectIndex", 8, "u32"),
    ("parameterId", 12, "u32"), ("smoothingSamples", 16, "u32"),
    ("reserved1", 20, "u32"), ("values", 24, "f32[4]"), ("reserved2", 40, "u8[8]"),
)


def named(rows: list[tuple[int, str]]) -> list[dict[str, object]]:
    return [{"value": value, "name": name} for value, name in rows]


CONSTANTS = {
    "resultCodes": named([
        (0, "ok"), (1, "invalidArgument"), (2, "abiMismatch"), (3, "wrongState"),
        (4, "bufferTooSmall"), (5, "prepareRejected"), (6, "backpressure"),
        (7, "unsupported"), (8, "renderRejected"), (9, "reprepareRequired"),
        (255, "internal"),
    ]),
    "states": named([(0, "config"), (1, "prepared"), (2, "ready"), (3, "failed"), (4, "disposed")]),
    "backends": named([(0, "scalar"), (1, "simd128")]),
    "bufferKinds": named([
        (1, "sessionToml"), (2, "sourceId"), (3, "sourcePcm"), (4, "diagnostic"),
        (5, "outputPcm"), (6, "command"), (7, "meterFrame"),
    ]),
    "wireCommandKinds": named([
        (1, "pan"), (2, "matrix"), (3, "faderDb"), (4, "mute"),
        (5, "effectParam"), (6, "effectBypass"), (7, "observeSubscribe"),
        (8, "observeUnsubscribe"),
    ]),
    "commandReasons": named([
        (0, "none"), (1, "malformed"), (2, "unknownTrack"), (3, "unknownRack"),
        (4, "unknownEffect"), (5, "unknownParameter"), (6, "domain"),
        (7, "unsupportedKind"), (8, "backpressure"), (9, "wrongState"),
        (10, "unknownTap"), (11, "observationUnbound"),
    ]),
    "maximumCommandRecords": 256,
}


class Invalid(Exception):
    """One ABI-schema invariant was broken."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


def valid_document() -> dict[str, object]:
    return {
        "schema": SCHEMA,
        "abiVersion": ABI_VERSION,
        "structures": {
            name: {"bytes": bytes_, "fields": copy.deepcopy(rows)}
            for name, (bytes_, rows) in STRUCTURES.items()
        },
        "commandRecord": {"bytes": 48, "endianness": "little", "fields": copy.deepcopy(COMMAND_RECORD)},
        "constants": copy.deepcopy(CONSTANTS),
    }


def validate(document: object) -> None:
    require(isinstance(document, dict), "document object")
    require(set(document) == {"schema", "abiVersion", "structures", "commandRecord", "constants"}, "top-level keys")
    require(document["schema"] == SCHEMA, "schema")
    require(document["abiVersion"] == ABI_VERSION, "ABI version")

    structures = document["structures"]
    require(isinstance(structures, dict) and set(structures) == set(STRUCTURES), "structure names")
    for name, (bytes_, expected_fields) in STRUCTURES.items():
        actual = structures[name]
        require(isinstance(actual, dict) and set(actual) == {"bytes", "fields"}, f"{name} keys")
        require(actual["bytes"] == bytes_, f"{name} bytes")
        require(actual["fields"] == expected_fields, f"{name} fields")

    command_record = document["commandRecord"]
    require(isinstance(command_record, dict) and set(command_record) == {"bytes", "endianness", "fields"}, "command record keys")
    require(command_record["bytes"] == 48, "command record bytes")
    require(command_record["endianness"] == "little", "command record endianness")
    require(command_record["fields"] == COMMAND_RECORD, "command record fields")

    constants = document["constants"]
    require(isinstance(constants, dict) and set(constants) == set(CONSTANTS), "constant groups")
    for name, expected in CONSTANTS.items():
        require(constants[name] == expected, f"constants {name}")


def self_test() -> int:
    mutations: list[tuple[str, object]] = [
        ("schema", lambda d: d.update(schema="miso.web.abi-layout.v2")),
        ("ABI version", lambda d: d.update(abiVersion=1)),
        ("top-level key", lambda d: d.update(extra=True)),
        ("structure name", lambda d: d["structures"].pop("status")),
        ("structure key", lambda d: d["structures"]["status"].update(extra=True)),
        ("command record bytes", lambda d: d["commandRecord"].update(bytes=47)),
        ("command record endianness", lambda d: d["commandRecord"].update(endianness="big")),
        ("command record key", lambda d: d["commandRecord"].update(extra=True)),
        ("constant group", lambda d: d["constants"].pop("states")),
    ]
    for structure, (bytes_, rows) in STRUCTURES.items():
        mutations.append((f"{structure} bytes", lambda d, s=structure, b=bytes_: d["structures"][s].update(bytes=b + 1)))
        for index, _ in enumerate(rows):
            for key, value in (("name", "wrongName"), ("offset", -1), ("type", "wrongType")):
                mutations.append((
                    f"{structure} field {index} {key}",
                    lambda d, s=structure, i=index, k=key, v=value: d["structures"][s]["fields"][i].update({k: v}),
                ))
    for index, _ in enumerate(COMMAND_RECORD):
        for key, value in (("name", "wrongName"), ("offset", -1), ("type", "wrongType")):
            mutations.append((
                f"command record field {index} {key}",
                lambda d, i=index, k=key, v=value: d["commandRecord"]["fields"][i].update({k: v}),
            ))
    for group, expected in CONSTANTS.items():
        if isinstance(expected, list):
            for index, _ in enumerate(expected):
                for key, value in (("value", -1), ("name", "wrongName")):
                    mutations.append((
                        f"{group} row {index} {key}",
                        lambda d, g=group, i=index, k=key, v=value: d["constants"][g][i].update({k: v}),
                    ))
        else:
            mutations.append((f"{group}", lambda d, g=group: d["constants"].update({g: -1})))

    failures = 0
    for name, mutate in mutations:
        document = valid_document()
        mutate(document)
        try:
            validate(document)
        except Invalid:
            continue
        except Exception:  # A mutation that crashes validation still proves it was not accepted.
            continue
        print(f"self-test FAILED: mutation escaped -- {name}", file=sys.stderr)
        failures += 1
    if failures == 0:
        print("ABI layout schema self-test passed")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("document", nargs="?", type=pathlib.Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if args.document is None:
        parser.error("a document path is required")
    try:
        validate(json.loads(args.document.read_text(encoding="utf-8")))
    except (Invalid, json.JSONDecodeError) as error:
        print(f"FAIL ABI layout: {error}", file=sys.stderr)
        return 1
    print(f"ABI layout schema: ok ({args.document})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
