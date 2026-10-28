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
  `require_*` words and the three `liveControl*` words present under those exact spellings, because
  the SDK's scratch/worklet equality rule (adopted ruling finding 3) is written in terms of them.
  The word at offset 32 is `maximumControlFrameBytes` (issue #2191; it was the command-queue word
  until slice #1565A deleted it);
* `resultCodes` is the eleven-value frozen ladder with no duplicate value and no duplicate name;
* `bootResultAliases` is exactly three rows, every one of which **re-uses** a value that
  `resultCodes` already names under a *different* name -- that is what makes it an alias table
  rather than a renumbering (adopted ruling finding 2);
* `stagingSequence` is the four boot exports in call order, so the retired "3-call boot" shorthand
  cannot come back;
* `errorPhases` is the six-phase vocabulary, and does not contain the dead two-phase `compile`
  spelling;
* the `sourceRing` rule carries every one of its inputs (#1406 D3): the stall ring's two and the
  warm-successor prime's three -- `primeGrowths`, `primeReserveQuanta` and one
  `primeLatencySamples` row per launch rate -- since a consumer that lacks any of them cannot
  derive the ring the engine picks;
* `memory` names the shared linear memory's page counts (#1380 D1): `maximumPages` is the 512 MiB
  the owner fixed, `initialPages` is a positive count no larger, and the default boot budget fits
  the maximum, so a consumer creating `new WebAssembly.Memory({initial, maximum, shared: true})`
  from it can always boot the default.

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
ABI_VERSION = 0x0001_0000

# Spelled out here rather than imported, on purpose: this file is the second implementation.
# `hosts/host-web/src/tests.rs` is what proves the Rust constants themselves.
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
    (10, "compileRejected"),
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
    (7, "meterFrame"),
]
# Buffer kind 6 was the observation-command staging; #1589c deleted it with the command record,
# its kinds and its reasons. The value is retired and never reallocated.
# Decision 12 (#1096): the effect racks of the companion and configuration records. `inserts` keeps the code `dynamic` had and `console` is appended. The retired `simd1`
# (0) and `simd2` (2) codes are absent from the vocabulary and never reallocated.
RACKS = [(1, "inserts"), (3, "console"), (255, "notApplicable")]
RETIRED_RACKS = {0, 2}

STAGING_SEQUENCE = [
    "miso_engine_web_v1_abi_version",
    "miso_engine_web_v1_boot_options_ptr",
    "miso_engine_web_v1_document_ptr",
    "miso_engine_web_v1_boot",
]
# This is the generator's stable canonical lexical export order.  WebAssembly consumers use names
# rather than positions, but keeping the document order fixed makes generated diffs and review
# deterministic.
EXPORTS = [
    "miso_engine_web_v1_abi_version",
    "miso_engine_web_v1_boot",
    "miso_engine_web_v1_boot_command_ptr",
    "miso_engine_web_v1_boot_diagnostic_bytes",
    "miso_engine_web_v1_boot_diagnostic_ptr",
    "miso_engine_web_v1_boot_options_ptr",
    "miso_engine_web_v1_boot_result",
    "miso_engine_web_v1_buffer_capacity",
    "miso_engine_web_v1_buffer_ptr",
    "miso_engine_web_v1_control_allocation_count",
    "miso_engine_web_v1_control_command",
    "miso_engine_web_v1_control_event",
    "miso_engine_web_v1_control_frame_bytes",
    "miso_engine_web_v1_control_request_ptr",
    "miso_engine_web_v1_control_response_bytes",
    "miso_engine_web_v1_control_response_ptr",
    "miso_engine_web_v1_control_revision",
    "miso_engine_web_v1_declare_discontinuity",
    "miso_engine_web_v1_dispose",
    "miso_engine_web_v1_document_ptr",
    "miso_engine_web_v1_host_adopt",
    "miso_engine_web_v1_host_release",
    "miso_engine_web_v1_instance_bind",
    "miso_engine_web_v1_instance_reserve",
    "miso_engine_web_v1_meter_header_ptr",
    "miso_engine_web_v1_meter_lease",
    "miso_engine_web_v1_meter_poll",
    "miso_engine_web_v1_plan_latency_ptr",
    "miso_engine_web_v1_plan_latency_read",
    "miso_engine_web_v1_render",
    "miso_engine_web_v1_render_allocation_count",
    "miso_engine_web_v1_render_diagnostic_bytes",
    "miso_engine_web_v1_render_diagnostic_ptr",
    "miso_engine_web_v1_replace_document_ptr",
    "miso_engine_web_v1_resolved_source_ring_frames",
    "miso_engine_web_v1_resource_ptr",
    "miso_engine_web_v1_response_frame_close",
    "miso_engine_web_v1_response_frame_ptr",
    "miso_engine_web_v1_response_frame_reply_bytes",
    "miso_engine_web_v1_response_frame_reply_ptr",
    "miso_engine_web_v1_response_frame_submit",
    "miso_engine_web_v1_service",
    "miso_engine_web_v1_session_seek",
    "miso_engine_web_v1_session_seek_at",
    "miso_engine_web_v1_source_channels",
    "miso_engine_web_v1_source_count",
    "miso_engine_web_v1_source_fill",
    "miso_engine_web_v1_source_frames",
    "miso_engine_web_v1_source_id",
    "miso_engine_web_v1_source_seek",
    "miso_engine_web_v1_source_seek_at",
    "miso_engine_web_v1_source_seek_report",
    "miso_engine_web_v1_source_seek_report_ptr",
    "miso_engine_web_v1_source_submit",
    "miso_engine_web_v1_status_ptr",
    "miso_engine_web_v1_successor_header_ptr",
    "miso_engine_web_v1_watermark_ptr",
    "miso_engine_web_v1_watermark_read",
]
ERROR_PHASES = ["asset", "boot", "source", "render", "output", "lifecycle"]
# The dead two-phase lifecycle's vocabulary. Named so its return is a failure rather than a
# silently accepted extra row (issue #243 S2(b)).
RETIRED_PHASES = {"compile", "config", "prepared", "prepareRejected"}

STRUCTURES = {
    "bootOptions": 64,
    "status": 80,
    "resourceReport": 224,
    "meterHeader": 88,
    "planWatermark": 96,
    "planLatency": 64,
    "seekReport": 64,
    "instanceReservation": 16,
}
BOOT_OPTION_FIELDS = [
    "structSize", "abiVersion", "requireSampleRateHz", "requireQuantumFrames",
    "sourceRingFrames", "reserved0", "maximumMemoryBytes", "maximumControlFrameBytes",
    "liveControlMeterBlocks", "maximumDocumentBytes", "reserved2",
]
# The words the scratch and worklet boots must write identically (adopted ruling finding 3), and
# the two whose values are role-defined. Kept here so the SDK's equality eval has a schema-level
# statement of which words it masks.
POLICY_WORDS = [
    "sourceRingFrames", "maximumMemoryBytes",
    "liveControlMeterBlocks",
]
ROLE_DEFINED_WORDS = ["requireSampleRateHz", "requireQuantumFrames"]

# The applied-revision watermark record (#1381a D5): the C ABI's `miso_engine_v1_watermark`, field
# for field and in its order (#1314 D6), without a pre-roll counter.
PLAN_WATERMARK_FIELDS = [
    "structSize", "reserved0", "revision", "firstSample", "outcomeFlags", "exactCount",
    "transitionFallbackCount", "supersededCount", "reserved",
]
# The plan latency record (#1688 D2): the C ABI report's latency and tail words.
PLAN_LATENCY_FIELDS = [
    "structSize", "reserved0", "latencySamples", "tailKind", "tailSamples", "reserved",
]
# The source seek report record (#1293 D6): the C ABI seek report's layout.
SEEK_REPORT_FIELDS = [
    "structSize", "reserved0", "generation", "firstSample", "sourceFrame",
    "cumulativeUnderrunFrames", "cumulativeHeldFrames", "reserved",
]
# The instance reservation record (#1332 S6): four u32 words a page reads before it starts the
# second instance on the shared memory.
INSTANCE_RESERVATION_FIELDS = ["structSize", "tlsBase", "stackTop", "stackBottom"]
# The status block (#1349 D2): offset 28 `blockingRebuilds` (#1294b), offset 48
# `liveValuesSuperseded` (#1399), then the applied-revision watermark words at 56, 64 and 72, and
# no reserved word.
STATUS_FIELDS = [
    "structSize", "abiVersion", "state", "lastResult", "backend", "sampleRateHz",
    "quantumFrames", "blockingRebuilds", "nextAbsoluteSample", "renderedQuanta", "liveValuesSuperseded",
    "watermarkRevision", "watermarkFirstSample", "watermarkOutcomeFlags",
]
STATUS_WATERMARK_OFFSETS = {
    "liveValuesSuperseded": 48,
    "watermarkRevision": 56,
    "watermarkFirstSample": 64,
    "watermarkOutcomeFlags": 72,
}
# Issue #1590: the browser's own observation records (`observationSelection`,
# `observationResult`), their channel and status vocabularies and their sixteen exports are
# deleted; the observation edit and read are the core's frames. The exact structure, constant
# and export sets above refuse them should they return.

# Issue #1380 D1 (owner decision of 2026-10-27): the shared memory's maximum, 512 MiB in pages.
MAXIMUM_MEMORY_PAGES = 8192
PAGE_BYTES = 1 << 16

WIDTHS = {"u8": 1, "u32": 4, "u64": 8, "f64": 8}


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
    require(all(isinstance(row, dict) and set(row) == {"value", "name"}
                and type(row["value"]) is int for row in rows),
            f"constants.{group} rows are exactly value/name")
    actual = [(row["value"], row["name"]) for row in rows]
    require(actual == expected, f"constants.{group} is {actual}, expected {expected}")



def validate(document: object) -> None:
    require(isinstance(document, dict), "the document is a JSON object")
    assert isinstance(document, dict)
    require(set(document) == {"schema", "abiVersion", "stagingSequence", "errorPhases",
                              "exports", "structures", "constants", "memory"},
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
    require(isinstance(exports, list) and len(exports) == len(EXPORTS),
            f"exports names the {len(EXPORTS)} module functions, not {len(exports)}")
    require(exports == EXPORTS, "exports are the frozen module function sequence")
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

    status_rows = structures["status"]["fields"]
    require([row["name"] for row in status_rows] == STATUS_FIELDS,
            f"status names exactly {STATUS_FIELDS}")
    for row in status_rows:
        if row["name"] in STATUS_WATERMARK_OFFSETS:
            require(row["offset"] == STATUS_WATERMARK_OFFSETS[row["name"]]
                    and row["type"] == "u64",
                    f"status.{row['name']} is a u64 at {STATUS_WATERMARK_OFFSETS[row['name']]}")
    for name, expected_fields in (
        ("planWatermark", PLAN_WATERMARK_FIELDS),
        ("planLatency", PLAN_LATENCY_FIELDS),
        ("seekReport", SEEK_REPORT_FIELDS),
        ("instanceReservation", INSTANCE_RESERVATION_FIELDS),
    ):
        require([row["name"] for row in structures[name]["fields"]] == expected_fields,
                f"{name} names exactly {expected_fields}")
    constants = document["constants"]
    require(isinstance(constants, dict), "constants is an object")
    require(set(constants) == {
        "resultCodes", "bootResultAliases", "states", "backends", "bufferKinds",
        "parseTransientMultiplier", "maximumSourceChannels", "diagnosticBytes",
        "defaultMeterBlocks",
        "maximumResponsePoints", "defaultMaximumMemoryBytes", "sourceRing",
        "racks",
    }, f"constants keys are exact: {sorted(constants)}")

    check_named(document, "resultCodes", RESULT_CODES)
    check_named(document, "bootResultAliases", BOOT_RESULT_ALIASES)
    check_named(document, "states", STATES)
    check_named(document, "backends", BACKENDS)
    check_named(document, "bufferKinds", BUFFER_KINDS)
    check_named(document, "racks", RACKS)
    # Spelled out on purpose: a retired code must never be reallocated, whatever it is named.
    require(not RETIRED_RACKS & {row["value"] for row in constants["racks"]},
            "constants.racks reallocates a retired simd1/simd2 code")

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
        ("parseTransientMultiplier", 20),
        ("maximumSourceChannels", 2),
        ("diagnosticBytes", 1 << 14),
        ("defaultMeterBlocks", 12),
        ("maximumResponsePoints", 4096),
        ("defaultMaximumMemoryBytes", 512 << 20),
    ):
        require(constants[name] == expected,
                f"constants.{name} is {constants[name]}, expected {expected}")

    memory = document["memory"]
    require(isinstance(memory, dict) and set(memory) == {"initialPages", "maximumPages"},
            "memory carries exactly initialPages/maximumPages")
    require(all(type(memory[key]) is int for key in memory), "memory page counts are integers")
    require(memory["maximumPages"] == MAXIMUM_MEMORY_PAGES,
            f"memory.maximumPages is {memory['maximumPages']}, expected {MAXIMUM_MEMORY_PAGES}")
    require(0 < memory["initialPages"] <= memory["maximumPages"],
            "memory.initialPages is positive and no larger than maximumPages")
    require(constants["defaultMaximumMemoryBytes"] <= memory["maximumPages"] * PAGE_BYTES,
            "the default boot budget fits the shared memory's maximum")

    ring = constants["sourceRing"]
    require(isinstance(ring, dict) and set(ring) == {
        "stallToleranceMs", "reserveQuanta", "primeGrowths", "primeReserveQuanta",
        "primeLatencySamples",
    }, "sourceRing carries exactly its five inputs")
    require(ring["stallToleranceMs"] == 100, "the stall tolerance is 100 ms")
    require(ring["reserveQuanta"] == 2, "the ring reserves two quanta")
    require(ring["primeGrowths"] == 4, "the ring holds a prime for four growths")
    require(ring["primeReserveQuanta"] == 1, "the ring reserves one quantum past the prime")
    # `L_max(fs) = fs / 100 + 6`, the true-peak limiter's latency, at each launch rate in order.
    require(ring["primeLatencySamples"] == [
        {"sampleRateHz": 44100, "samples": 447},
        {"sampleRateHz": 48000, "samples": 486},
        {"sampleRateHz": 88200, "samples": 888},
        {"sampleRateHz": 96000, "samples": 966},
    ], "primeLatencySamples is L_max at each launch rate")


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
        document["structures"]["seekReport"]["fields"][3]["offset"] = 20

    def drop_export(document: dict) -> None:
        document["exports"].pop()

    def drop_service_export(document: dict) -> None:
        export = "miso_engine_web_v1_service"
        try:
            document["exports"].remove(export)
        except ValueError as error:
            raise AssertionError("the service export exists in the valid fixture") from error

    def drop_watermark_field(document: dict) -> None:
        document["structures"]["planWatermark"]["fields"].pop(2)

    def drop_plan_latency_export(document: dict) -> None:
        export = "miso_engine_web_v1_plan_latency_read"
        try:
            document["exports"].remove(export)
        except ValueError as error:
            raise AssertionError("the plan latency export exists in the valid fixture") from error

    def drop_plan_latency_field(document: dict) -> None:
        document["structures"]["planLatency"]["fields"].pop(2)

    def drop_seek_export(export: str):
        def drop(document: dict) -> None:
            try:
                document["exports"].remove(export)
            except ValueError as error:
                raise AssertionError(f"{export} exists in the valid fixture") from error
        return drop

    def swap_seek_report_names(document: dict) -> None:
        # Two u64 words trade names and keep their offsets: only the record's name rule sees it.
        fields = document["structures"]["seekReport"]["fields"]
        first, second = (next(row for row in fields if row["name"] == name)
                         for name in ("firstSample", "sourceFrame"))
        first["name"], second["name"] = second["name"], first["name"]

    def drop_status_watermark_word(document: dict) -> None:
        document["structures"]["status"]["fields"].pop()

    def unsorted_exports(document: dict) -> None:
        document["exports"].reverse()

    def three_call_boot(document: dict) -> None:
        document["stagingSequence"].remove("miso_engine_web_v1_boot_options_ptr")

    def retired_phase(document: dict) -> None:
        document["errorPhases"] = ["asset", "compile", "source", "render", "output", "lifecycle"]

    def command_record_returns(document: dict) -> None:
        document["commandRecord"] = {"bytes": 272, "endianness": "little", "fields": []}

    def command_kinds_return(document: dict) -> None:
        document["constants"]["wireCommandKinds"] = [{"value": 7, "name": "observeSubscribe"}]

    def command_buffer_returns(document: dict) -> None:
        document["constants"]["bufferKinds"].insert(4, {"value": 6, "name": "command"})

    def retired_queue_word_returns(document: dict) -> None:
        document["structures"]["bootOptions"]["fields"][7]["name"] = (
            "liveControlCommandQueueRecords"
        )

    def retired_export_returns(document: dict) -> None:
        document["exports"].append("miso_engine_web_v1_prepared_command_submit")

    def drop_ring_input(document: dict) -> None:
        del document["constants"]["sourceRing"]["reserveQuanta"]

    def wrong_ring_reserve(document: dict) -> None:
        document["constants"]["sourceRing"]["reserveQuanta"] = 1

    def drop_ring_key(key: str):
        def drop(document: dict) -> None:
            del document["constants"]["sourceRing"][key]
        return drop

    def private_prime_reserve(document: dict) -> None:
        document["constants"]["sourceRing"]["primeReserveQuanta"] = 0

    def wrong_prime_latency(document: dict) -> None:
        document["constants"]["sourceRing"]["primeLatencySamples"][3]["samples"] = 960

    def prepare_config_returns(document: dict) -> None:
        document["structures"]["prepareConfig"] = {"bytes": 192, "fields": []}

    def stale_abi_version(document: dict) -> None:
        document["abiVersion"] = 0

    def duplicate_result_name(document: dict) -> None:
        document["constants"]["resultCodes"][5]["name"] = "prepareRejected"

    def reallocate_retired_rack(document: dict) -> None:
        for row in document["constants"]["racks"]:
            if row["name"] == "console":
                row["value"] = 0
                return
        raise AssertionError("the console rack row exists in the valid fixture")

    def boolean_named_value(document: dict) -> None:
        rows = document["constants"]["racks"]
        for row in rows:
            if row["name"] == "inserts":
                row["value"] = True
                return
        raise AssertionError("the inserts rack row exists in the valid fixture")

    def memory_maximum_moves(document: dict) -> None:
        document["memory"]["maximumPages"] = MAXIMUM_MEMORY_PAGES + 1

    def memory_initial_exceeds_maximum(document: dict) -> None:
        document["memory"]["initialPages"] = document["memory"]["maximumPages"] + 1

    mutations = [
        ("the memory page counts are dropped", lambda d: d.pop("memory")),
        ("the shared memory's maximum moves", memory_maximum_moves),
        ("the initial memory exceeds the maximum", memory_initial_exceeds_maximum),
        ("an alias row is dropped", drop_alias),
        ("an alias repeats its base name", alias_repeats_base),
        ("an alias invents a value", alias_invents_value),
        ("a boot option is renamed", rename_field),
        ("a boot option is dropped", drop_field),
        ("a status word is widened", widen_field),
        ("a structure gains a hole", hole_in_layout),
        ("an export is dropped", drop_export),
        ("the service export is dropped", drop_service_export),
        ("the watermark record omits a field", drop_watermark_field),
        ("the plan latency record omits a field", drop_plan_latency_field),
        ("the plan latency read export is dropped", drop_plan_latency_export),
        ("the seek report record swaps two field names", swap_seek_report_names),
        ("the anchored seek export is dropped",
         drop_seek_export("miso_engine_web_v1_source_seek_at")),
        ("the session seek export is dropped",
         drop_seek_export("miso_engine_web_v1_session_seek")),
        ("the anchored session seek export is dropped",
         drop_seek_export("miso_engine_web_v1_session_seek_at")),
        ("the status omits a watermark word", drop_status_watermark_word),
        ("the export set is unsorted", unsorted_exports),
        ("the staging sequence drops back to three calls", three_call_boot),
        ("a retired lifecycle phase returns", retired_phase),
        # Issue #1589c: the observation command record, its kinds, its staging and its export.
        ("the deleted command record returns", command_record_returns),
        ("the deleted command-kind table returns", command_kinds_return),
        ("the deleted command-staging buffer kind returns", command_buffer_returns),
        ("the deleted command-submit export returns",
         lambda d: d["exports"].insert(7, "miso_engine_web_v1_command_submit")),
        ("the retired command-queue boot word returns", retired_queue_word_returns),
        ("a retired prepared-command export returns", retired_export_returns),
        # Issue #1590 gate 4: the browser's own observation records, vocabularies and exports.
        ("the deleted observation selection record returns",
         lambda d: d["structures"].__setitem__("observationSelection", d["structures"]["seekReport"])),
        ("the deleted observation status vocabulary returns",
         lambda d: d["constants"].__setitem__("observationStatuses", [{"value": 1, "name": "pending"}])),
        ("the deleted observation read export returns",
         lambda d: d["exports"].insert(19, "miso_engine_web_v1_observation_read")),
        ("the retired observation-taps boot word returns",
         lambda d: d["structures"]["bootOptions"]["fields"][9].__setitem__("name", "liveControlObservationTaps")),
        ("a source-ring input is dropped", drop_ring_input),
        ("the source-ring reserve is wrong", wrong_ring_reserve),
        ("the prime growth count is dropped", drop_ring_key("primeGrowths")),
        ("the prime reserve quantum is dropped", drop_ring_key("primeReserveQuanta")),
        ("the prime reserve quantum is kept private", private_prime_reserve),
        ("a launch rate's prime latency is wrong", wrong_prime_latency),
        ("the 192-byte prepare config returns", prepare_config_returns),
        ("the ABI version goes stale", stale_abi_version),
        ("a retired result name returns", duplicate_result_name),
        ("a named numeric value becomes boolean", boolean_named_value),
        ("a retired rack code is reallocated to console", reallocate_retired_rack),
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
