#!/usr/bin/env python3
"""Schema gate for the shipped browser ABI layout (issue #243).

The generator and this validator are deliberately two implementations. The generator takes its
offsets from Rust `offset_of!` and its vocabularies from the frozen `RESULT_*`/`STATE_*`/`BUFFER_*`
constants; this walks the emitted JSON in Python and knows only what the schema promises. That
split is the whole point of the file: the boot ABI's bytes used to be hand-written on the
JavaScript side -- issue #207's N-13(d) counted five copies of the configuration table, one of
which wrote a 192-byte struct's offsets into a 64-byte buffer and produced garbage in silence --
and a document nothing validates is just a sixth copy.

# What this gate checks that `offset_of!` cannot

`offset_of!` guarantees an emitted offset is *some* field's offset. It cannot catch a **renamed**
row, a **dropped** row, a row whose declared width disagrees with the next row's offset, or a
structure whose rows no longer tile its `bytes`. Those are exactly the drifts a consumer feels, so
they are the rules here:

* every structure's rows are strictly ascending, start at 0, never overlap, and **tile the
  structure exactly** -- the sum of the widths equals `bytes`, so a field cannot be added, dropped
  or silently resized without the total moving;
* the `bootOptions` block is 64 bytes and names exactly the eleven boot words, with the two
  `require_*` words and the four `console*` words present under those exact spellings, because the
  SDK's scratch/worklet equality rule (adopted ruling finding 3) is written in terms of them;
* `resultCodes` is the eleven-value frozen ladder with no duplicate value and no duplicate name;
* `bootResultAliases` is exactly three rows, every one of which **re-uses** a value that
  `resultCodes` already names under a *different* name -- that is what makes it an alias table
  rather than a renumbering (adopted ruling finding 2);
* `stagingSequence` is the four boot exports in call order, so the retired "3-call boot" shorthand
  cannot come back;
* `errorPhases` is the six-phase vocabulary, and does not contain the dead two-phase `compile`
  spelling;
* the `sourceRing` rule carries both of its inputs, since a consumer that has only one of them
  cannot derive the ring at all.

`--self-test` runs every rule against a valid document and against its own red mutation, so the
validator is proved to discriminate before it is trusted.
"""

from __future__ import annotations

import argparse
import copy
import json
import pathlib
import sys

SCHEMA = "miso.web.abi-layout.v1"
ABI_VERSION = 0x0002_0000

# Spelled out here rather than imported, on purpose: this file is the second implementation.
# `hosts/miso-engine-host-web/src/tests.rs` is what proves the Rust constants themselves.
RESULT_CODES = [
    (0, "ok"),
    (1, "invalidArgument"),
    (2, "abiMismatch"),
    (3, "wrongState"),
    (4, "bufferTooSmall"),
    (5, "refusedBudget"),
    (6, "backpressure"),
    (7, "unsupported"),
    (8, "renderRejected"),
    (9, "reprepareRequired"),
    (255, "internal"),
]
BOOT_RESULT_ALIASES = [(1, "refusedDocument"), (2, "refusedOptions"), (3, "refusedLifecycle")]
STATES = [(2, "ready"), (3, "failed"), (4, "disposed")]
BACKENDS = [(0, "scalar"), (1, "simd128")]
BUFFER_KINDS = [
    (2, "sourceId"),
    (3, "sourcePcm"),
    (4, "diagnostic"),
    (5, "outputPcm"),
    (6, "command"),
    (7, "meterFrame"),
]
COMMAND_KINDS = [
    "pan", "matrix", "faderDb", "mute", "effectParam", "effectBypass",
    "observeSubscribe", "observeUnsubscribe", "solo", "trimDb", "polarityInvert",
]
COMMAND_REASONS = [
    "none", "malformed", "unknownTrack", "unknownRack", "unknownEffect", "unknownParameter",
    "domain", "unsupportedKind", "backpressure", "wrongState", "unknownTap", "observationUnbound",
]

STAGING_SEQUENCE = [
    "miso_engine_web_v1_abi_version",
    "miso_engine_web_v1_boot_options_ptr",
    "miso_engine_web_v1_document_ptr",
    "miso_engine_web_v1_boot",
]
ERROR_PHASES = ["asset", "boot", "source", "render", "output", "lifecycle"]
# The dead two-phase lifecycle's vocabulary. Named so its return is a failure rather than a
# silently accepted extra row (issue #243 S2(b)).
RETIRED_PHASES = {"compile", "config", "prepared", "prepareRejected"}

STRUCTURES = {
    "bootOptions": 64,
    "status": 80,
    "resourceReport": 224,
    "meterHeader": 64,
    "commandReport": 48,
}
BOOT_OPTION_FIELDS = [
    "structSize", "abiVersion", "requireSampleRateHz", "requireQuantumFrames",
    "sourceRingFrames", "reserved0", "maximumMemoryBytes", "consoleCommandQueueRecords",
    "consoleMeterBlocks", "consoleObservationTaps", "consoleMasterTrackPlusOne",
]
# The words the scratch and worklet boots must write identically (adopted ruling finding 3), and
# the two whose values are role-defined. Kept here so the SDK's equality eval has a schema-level
# statement of which words it masks.
POLICY_WORDS = [
    "sourceRingFrames", "maximumMemoryBytes", "consoleCommandQueueRecords",
    "consoleMeterBlocks", "consoleObservationTaps", "consoleMasterTrackPlusOne",
]
ROLE_DEFINED_WORDS = ["requireSampleRateHz", "requireQuantumFrames"]

COMMAND_RECORD_FIELDS = [
    "kind", "rack", "channel", "reserved0", "trackIndex", "effectIndex", "parameterId",
    "smoothingSamples", "reserved1", "values", "reserved2",
]

WIDTHS = {"u8": 1, "u32": 4, "u64": 8}


class Invalid(Exception):
    """One rule this gate enforces was broken."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


def width(kind: str) -> int:
    """Byte width of a declared field type, including the `name[count]` array spellings."""
    if kind.endswith("]"):
        base, _, count = kind[:-1].partition("[")
        require(count.isdigit() and int(count) > 0, f"array count is a positive integer: {kind}")
        return width(base) * int(count)
    require(kind in WIDTHS or kind == "f32", f"unknown field type {kind}")
    return 4 if kind == "f32" else WIDTHS[kind]


def check_fields(name: str, fields: object, total: int) -> None:
    require(isinstance(fields, list) and fields, f"{name} carries a non-empty field list")
    offset = 0
    seen: set[str] = set()
    for row in fields:
        require(isinstance(row, dict), f"{name} field row is an object")
        require(set(row) == {"name", "offset", "type"}, f"{name} field row keys are exact: {row}")
        require(isinstance(row["name"], str) and row["name"], f"{name} field has a name")
        require(row["name"] not in seen, f"{name} repeats field {row['name']}")
        seen.add(row["name"])
        require(isinstance(row["offset"], int) and not isinstance(row["offset"], bool),
                f"{name}.{row['name']} offset is an integer")
        require(row["offset"] == offset,
                f"{name}.{row['name']} starts at {row['offset']}, but the previous rows tile "
                f"through {offset}: the layout has a hole or an overlap")
        offset += width(row["type"])
    require(offset == total,
            f"{name} rows tile {offset} bytes but the structure declares {total}")


def check_named(document: dict, group: str, expected: list[tuple[int, str]]) -> None:
    rows = document["constants"].get(group)
    require(isinstance(rows, list), f"constants.{group} is a list")
    require(all(isinstance(row, dict) and set(row) == {"value", "name"} for row in rows),
            f"constants.{group} rows are exactly value/name")
    actual = [(row["value"], row["name"]) for row in rows]
    require(actual == expected, f"constants.{group} is {actual}, expected {expected}")


def check_positional(document: dict, group: str, names: list[str]) -> None:
    """A vocabulary whose row position stands for its value, contiguous from 1."""
    rows = document["constants"].get(group)
    require(isinstance(rows, list), f"constants.{group} is a list")
    actual = [(row["value"], row["name"]) for row in rows]
    require(actual == list(enumerate(names, start=1) if group == "wireCommandKinds"
                           else enumerate(names)),
            f"constants.{group} is not the contiguous vocabulary {names}")


def validate(document: object) -> None:
    require(isinstance(document, dict), "the document is a JSON object")
    assert isinstance(document, dict)
    require(set(document) == {"schema", "abiVersion", "stagingSequence", "errorPhases",
                              "exports", "structures", "commandRecord", "constants"},
            f"top-level keys are exact: {sorted(document)}")
    require(document["schema"] == SCHEMA, f"schema is {SCHEMA}")
    require(document["abiVersion"] == ABI_VERSION,
            f"abiVersion is 0x{ABI_VERSION:08x}, not {document['abiVersion']}")

    require(document["stagingSequence"] == STAGING_SEQUENCE,
            "stagingSequence is the four boot exports in call order")
    phases = document["errorPhases"]
    require(phases == ERROR_PHASES, f"errorPhases is {ERROR_PHASES}")
    require(not RETIRED_PHASES.intersection(phases),
            "errorPhases carries a retired two-phase-lifecycle spelling")

    exports = document["exports"]
    require(isinstance(exports, list) and len(exports) == 25,
            f"exports names the 25 module functions, not {len(exports)}")
    require(exports == sorted(exports), "exports is sorted")
    require(len(set(exports)) == len(exports), "exports has no duplicate")
    require("memory" not in exports, "memory is linear memory, not an exported call")
    require(all(name.startswith("miso_engine_web_v1_") for name in exports),
            "every export carries the frozen prefix")
    for step in STAGING_SEQUENCE:
        require(step in exports, f"the staging sequence's {step} is an export")

    structures = document["structures"]
    require(isinstance(structures, dict), "structures is an object")
    require(set(structures) == set(STRUCTURES),
            f"structures names exactly {sorted(STRUCTURES)}")
    for name, expected_bytes in STRUCTURES.items():
        entry = structures[name]
        require(isinstance(entry, dict) and set(entry) == {"bytes", "fields"},
                f"structures.{name} keys are exactly bytes/fields")
        require(entry["bytes"] == expected_bytes,
                f"structures.{name} is {entry['bytes']} bytes, expected {expected_bytes}")
        check_fields(name, entry["fields"], expected_bytes)

    boot = [row["name"] for row in structures["bootOptions"]["fields"]]
    require(boot == BOOT_OPTION_FIELDS, f"bootOptions names exactly {BOOT_OPTION_FIELDS}")
    for word in POLICY_WORDS + ROLE_DEFINED_WORDS:
        require(word in boot, f"bootOptions names the boot word {word}")

    record = document["commandRecord"]
    require(isinstance(record, dict) and set(record) == {"bytes", "endianness", "fields"},
            "commandRecord keys are exactly bytes/endianness/fields")
    require(record["endianness"] == "little", "the command record is little-endian")
    require(record["bytes"] == 48, f"the command record is 48 bytes, not {record['bytes']}")
    check_fields("commandRecord", record["fields"], 48)
    require([row["name"] for row in record["fields"]] == COMMAND_RECORD_FIELDS,
            f"commandRecord names exactly {COMMAND_RECORD_FIELDS}")

    constants = document["constants"]
    require(isinstance(constants, dict), "constants is an object")
    require(set(constants) == {
        "resultCodes", "bootResultAliases", "states", "backends", "bufferKinds",
        "wireCommandKinds", "commandReasons", "maximumCommandRecords", "maximumDocumentBytes",
        "diagnosticBytes", "defaultCommandQueueRecords", "defaultMeterBlocks",
        "maximumObservationTaps", "defaultMaximumMemoryBytes", "sourceRing",
    }, f"constants keys are exact: {sorted(constants)}")

    check_named(document, "resultCodes", RESULT_CODES)
    check_named(document, "bootResultAliases", BOOT_RESULT_ALIASES)
    check_named(document, "states", STATES)
    check_named(document, "backends", BACKENDS)
    check_named(document, "bufferKinds", BUFFER_KINDS)
    check_positional(document, "wireCommandKinds", COMMAND_KINDS)
    check_positional(document, "commandReasons", COMMAND_REASONS)

    # The alias table is an alias table: every row re-uses a value `resultCodes` already names,
    # under a different name. A row naming a value `resultCodes` does not carry would be a
    # renumbering; a row repeating the base name would be a pointless duplicate.
    base = {row["value"]: row["name"] for row in constants["resultCodes"]}
    aliases = constants["bootResultAliases"]
    require(len(aliases) == 3, f"the alias table is exactly three rows, not {len(aliases)}")
    for row in aliases:
        require(row["value"] in base,
                f"alias {row['name']} names value {row['value']}, which resultCodes does not")
        require(row["name"] != base[row["value"]],
                f"alias {row['name']} repeats its own base name; that is not an alias")
    require(len({row["name"] for row in aliases}) == 3, "alias names are distinct")

    for name, expected in (
        ("maximumCommandRecords", 256),
        ("maximumDocumentBytes", 1 << 20),
        ("diagnosticBytes", 1 << 14),
        ("defaultCommandQueueRecords", 64),
        ("defaultMeterBlocks", 12),
        ("maximumObservationTaps", 16),
        ("defaultMaximumMemoryBytes", 512 << 20),
    ):
        require(constants[name] == expected,
                f"constants.{name} is {constants[name]}, expected {expected}")

    ring = constants["sourceRing"]
    require(isinstance(ring, dict) and set(ring) == {"stallToleranceMs", "reserveQuanta"},
            "sourceRing carries exactly its two inputs")
    require(ring["stallToleranceMs"] == 100, "the stall tolerance is 100 ms")
    require(ring["reserveQuanta"] == 2, "the ring reserves two quanta")


def self_test() -> int:
    here = pathlib.Path(__file__).resolve().parent
    sample = json.loads((here / "fixtures/abi-layout-v1-self-test.json").read_text())
    try:
        validate(sample)
    except Invalid as error:
        print(f"self-test FAILED: the valid sample was rejected -- {error}", file=sys.stderr)
        return 1

    def drop_alias(document: dict) -> None:
        document["constants"]["bootResultAliases"].pop()

    def alias_repeats_base(document: dict) -> None:
        document["constants"]["bootResultAliases"][1]["name"] = "abiMismatch"

    def alias_invents_value(document: dict) -> None:
        document["constants"]["bootResultAliases"][0]["value"] = 42

    def rename_field(document: dict) -> None:
        document["structures"]["bootOptions"]["fields"][2]["name"] = "sampleRateHz"

    def drop_field(document: dict) -> None:
        document["structures"]["bootOptions"]["fields"].pop()

    def widen_field(document: dict) -> None:
        document["structures"]["status"]["fields"][0]["type"] = "u64"

    def hole_in_layout(document: dict) -> None:
        document["structures"]["commandReport"]["fields"][3]["offset"] = 16

    def drop_export(document: dict) -> None:
        document["exports"].pop()

    def unsorted_exports(document: dict) -> None:
        document["exports"].reverse()

    def three_call_boot(document: dict) -> None:
        document["stagingSequence"].remove("miso_engine_web_v1_boot_options_ptr")

    def retired_phase(document: dict) -> None:
        document["errorPhases"] = ["asset", "compile", "source", "render", "output", "lifecycle"]

    def move_record_field(document: dict) -> None:
        document["commandRecord"]["fields"][6]["offset"] = 16

    def drop_ring_input(document: dict) -> None:
        del document["constants"]["sourceRing"]["reserveQuanta"]

    def wrong_ring_reserve(document: dict) -> None:
        document["constants"]["sourceRing"]["reserveQuanta"] = 1

    def prepare_config_returns(document: dict) -> None:
        document["structures"]["prepareConfig"] = {"bytes": 192, "fields": []}

    def stale_abi_version(document: dict) -> None:
        document["abiVersion"] = 0x0001_0000

    def duplicate_result_name(document: dict) -> None:
        document["constants"]["resultCodes"][5]["name"] = "prepareRejected"

    mutations = [
        ("an alias row is dropped", drop_alias),
        ("an alias repeats its base name", alias_repeats_base),
        ("an alias invents a value", alias_invents_value),
        ("a boot option is renamed", rename_field),
        ("a boot option is dropped", drop_field),
        ("a status word is widened", widen_field),
        ("a structure gains a hole", hole_in_layout),
        ("an export is dropped", drop_export),
        ("the export set is unsorted", unsorted_exports),
        ("the staging sequence drops back to three calls", three_call_boot),
        ("a retired lifecycle phase returns", retired_phase),
        ("a command-record field moves", move_record_field),
        ("a source-ring input is dropped", drop_ring_input),
        ("the source-ring reserve is wrong", wrong_ring_reserve),
        ("the 192-byte prepare config returns", prepare_config_returns),
        ("the ABI version goes stale", stale_abi_version),
        ("a retired result name returns", duplicate_result_name),
    ]
    for name, mutate in mutations:
        broken = copy.deepcopy(sample)
        mutate(broken)
        try:
            validate(broken)
        except Invalid:
            continue
        print(f"self-test FAILED: mutation escaped -- {name}", file=sys.stderr)
        return 1

    print(f"abi layout schema self-test passed ({len(mutations)} mutations caught)")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("document", nargs="?", type=pathlib.Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if args.document is None:
        parser.error("a document path or --self-test is required")
    try:
        validate(json.loads(args.document.read_text()))
    except Invalid as error:
        print(f"{args.document}: {error}", file=sys.stderr)
        return 1
    except json.JSONDecodeError as error:
        print(f"{args.document}: not valid JSON: {error}", file=sys.stderr)
        return 1
    print(f"{args.document} satisfies {SCHEMA}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
