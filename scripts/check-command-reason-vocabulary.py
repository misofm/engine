#!/usr/bin/env python3
"""One command-reason vocabulary, proved across every file that spells it (issues #143, #151).

`COMMAND_REASON_*` is written out seven times in this repository, in six languages' worth of
syntax, and issue #151's field defect is exactly what that costs when they drift:

* `hosts/miso-engine-host-web/src/lib.rs` -- the constants. This is the authority.
* `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js` -- the acknowledgement
  validator's table. Before this gate it wrote its bound as the literal `<= 9`, so reasons 10 and
  11 -- the only two the observation path ever returns -- read as malformed acknowledgements and
  failed the *whole host* with a sticky 255. One refused subscription cost the session.
* `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js` -- the render-thread worklet,
  which names only the subset of reasons it refuses a request with itself.
* `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` -- the `MisoCommandReasonV1`
  enum a typed consumer compiles against.
* `tools/miso-engine-parameter-metadata/src/lib.rs` -- the `commandReasons` rows of the shipped
  JSON, which is where an app reads the vocabulary from.
* `scripts/check-parameter-metadata-v1.py` -- the schema gate's deliberately independent list.
* `sdk/src/generated/catalog.ts` -- the SDK's generated TypeScript transcription of the shipped
  metadata. This is the seventh spelling: it is checked against the Rust authority rather than
  trusted merely because its generator also reads the metadata artifact.

Adding a reason to the Rust constants without the other six is the drift class. This gate makes
that red: every source must yield the same ordered `value -> camelCase name` mapping, contiguous
from `0`.

Issue #151's other half is structural rather than numeric, and is checked here too: the shipped
`.d.ts` must declare `observe()` and the `miso.observe.v1` subscription-map types, with field sets
that match the shipped JS implementation exactly -- not the issue's sketch of it.

`--self-test` mutates in-memory copies of every source and requires each mutation to be caught, so
the gate is proved to discriminate before it is trusted.
"""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent

RUST_CONSTANTS = pathlib.Path("hosts/miso-engine-host-web/src/lib.rs")
HOST_JS = pathlib.Path("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js")
WORKLET_JS = pathlib.Path("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js")
HOST_DTS = pathlib.Path("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts")
METADATA_GENERATOR = pathlib.Path("tools/miso-engine-parameter-metadata/src/lib.rs")
SCHEMA_GATE = pathlib.Path("scripts/check-parameter-metadata-v1.py")
SDK_CATALOG = pathlib.Path("sdk/src/generated/catalog.ts")

SOURCES = (
    RUST_CONSTANTS,
    HOST_JS,
    WORKLET_JS,
    HOST_DTS,
    METADATA_GENERATOR,
    SCHEMA_GATE,
    SDK_CATALOG,
)

METADATA_DOCUMENT = "miso-engine-v2-parameter-metadata.json"


class Invalid(Exception):
    """One rule this gate enforces was broken."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


def camel_from_screaming(name: str) -> str:
    """`UNKNOWN_TAP` -> `unknownTap`."""
    head, *rest = name.lower().split("_")
    return head + "".join(word.capitalize() for word in rest)


def camel_from_pascal(name: str) -> str:
    """`UnknownTap` -> `unknownTap`."""
    return name[:1].lower() + name[1:]


def balanced(text: str, start: int, opening: str, closing: str) -> str:
    """Return `text[start:]` up to and including the bracket that closes the one at `start`."""
    require(text[start] == opening, f"expected {opening!r} at offset {start}")
    depth = 0
    for index in range(start, len(text)):
        if text[index] == opening:
            depth += 1
        elif text[index] == closing:
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    raise Invalid(f"unbalanced {opening!r} from offset {start}")


def block_after(text: str, anchor: str, opening: str, closing: str) -> str:
    """The bracketed block that follows `anchor`."""
    at = text.find(anchor)
    require(at >= 0, f"anchor not found: {anchor!r}")
    start = text.index(opening, at + len(anchor) - 1)
    return balanced(text, start, opening, closing)


# --- the seven spellings -----------------------------------------------------------------------


def rust_constants(text: str) -> list[tuple[int, str]]:
    rows = [
        (int(value), camel_from_screaming(name))
        for name, value in re.findall(
            r"^pub const COMMAND_REASON_([A-Z0-9_]+): u32 = (\d+);", text, re.MULTILINE
        )
    ]
    require(rows, "no COMMAND_REASON_* constants found in the Rust host")
    return sorted(rows)


def host_js_table(text: str) -> list[tuple[int, str]]:
    block = block_after(text, "const COMMAND_REASONS = Object.freeze(", "[", "]")
    names = re.findall(r'"([A-Za-z][A-Za-z0-9]*)"', block)
    require(names, "the host JS COMMAND_REASONS table is empty")
    return list(enumerate(names))


def host_dts_enum(text: str) -> list[tuple[int, str]]:
    block = block_after(text, "export const enum MisoCommandReasonV1 ", "{", "}")
    rows = [
        (int(value), camel_from_pascal(name))
        for name, value in re.findall(r"^\s*([A-Z][A-Za-z0-9]*) = (\d+),", block, re.MULTILINE)
    ]
    require(rows, "the .d.ts MisoCommandReasonV1 enum is empty")
    return sorted(rows)


def metadata_generator_table(text: str) -> list[tuple[int, str]]:
    block = block_after(text, "let reasons = ", "[", "]")
    names = [
        camel_from_screaming(constant)
        for constant, _ in re.findall(
            r'\(COMMAND_REASON_([A-Z0-9_]+), "([A-Za-z][A-Za-z0-9]*)"\)', block
        )
    ]
    emitted = re.findall(r'\(COMMAND_REASON_[A-Z0-9_]+, "([A-Za-z][A-Za-z0-9]*)"\)', block)
    require(names, "the metadata generator's reasons table is empty")
    require(
        names == emitted,
        "a metadata generator row emits a name that is not its own constant's name: "
        f"{[pair for pair in zip(names, emitted) if pair[0] != pair[1]]}",
    )
    return list(enumerate(names))


def schema_gate_list(text: str) -> list[tuple[int, str]]:
    block = block_after(text, "\nCOMMAND_REASONS = ", "[", "]")
    names = re.findall(r'"([A-Za-z][A-Za-z0-9]*)"', block)
    require(names, "the schema gate's COMMAND_REASONS list is empty")
    return list(enumerate(names))


def worklet_js_constants(text: str) -> list[tuple[int, str]]:
    """The worklet names only the reasons it produces itself, so this is a subset, not the table."""
    return sorted(
        (int(value), camel_from_screaming(name))
        for name, value in re.findall(
            r"^const COMMAND_REASON_([A-Z0-9_]+) = (\d+);", text, re.MULTILINE
        )
    )


def metadata_document_rows(document: dict) -> list[tuple[int, str]]:
    rows = document["commandReasons"]
    return sorted((int(row["value"]), row["name"]) for row in rows)


def sdk_catalog_table(text: str) -> list[tuple[int, str]]:
    """Read the generated catalog's JSON literal without executing TypeScript."""
    start = "export const CATALOG = deepFreeze(\n"
    end = " as const,\n);"
    first = text.find(start)
    require(first >= 0, "the generated SDK CATALOG declaration is absent")
    first += len(start)
    last = text.find(end, first)
    require(last >= 0, "the generated SDK CATALOG declaration has no closing marker")
    try:
        document = json.loads(text[first:last])
    except json.JSONDecodeError as error:
        raise Invalid(f"the generated SDK CATALOG JSON literal is invalid: {error}") from error
    return metadata_document_rows(document)


# --- the #151 structural half -------------------------------------------------------------------


def js_string_array(text: str, anchor: str) -> list[str]:
    block = block_after(text, anchor, "[", "]")
    return re.findall(r'"([A-Za-z][A-Za-z0-9]*)"', block)


def js_object_keys(text: str, anchor: str) -> list[str]:
    """Top-level keys of the object literal that follows `anchor`."""
    block = block_after(text, anchor, "{", "}")
    keys: list[str] = []
    depth = 0
    for line in block.splitlines():
        stripped = line.strip()
        if depth == 1:
            match = re.match(r"([A-Za-z_][A-Za-z0-9_]*)\s*:", stripped)
            if match:
                keys.append(match.group(1))
        depth += line.count("{") + line.count("[") + line.count("(")
        depth -= line.count("}") + line.count("]") + line.count(")")
    return keys


def dts_interface_fields(text: str, name: str) -> list[str]:
    block = block_after(text, f"export interface {name} ", "{", "}")
    return re.findall(
        r"^\s*(?:readonly\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*[?]?\s*:", block, re.MULTILINE
    )


def check_observe_typing(js: str, dts: str) -> None:
    """Issue #151: the `.d.ts` declares `observe()` and matches the SHIPPED implementation."""
    host_interface = block_after(dts, "export interface MisoAudioWorkletHost ", "{", "}")
    require(
        re.search(r"^\s*observe\(", host_interface, re.MULTILINE),
        "MisoAudioWorkletHost declares no observe() -- issue #151's defect",
    )
    require(
        re.search(
            r"observe\(request: MisoObservationRequestV1\): Promise<MisoObservationAckV1>;",
            host_interface,
        ),
        "observe() is declared with a signature other than "
        "(MisoObservationRequestV1) => Promise<MisoObservationAckV1>",
    )

    # The request shape is whatever `validSubscription` actually accepts, exactly.
    implemented = js_string_array(js, "const SUBSCRIPTION_FIELDS = ")
    declared = dts_interface_fields(dts, "MisoObservationSubscriptionV1")
    require(
        declared == implemented,
        "MisoObservationSubscriptionV1 does not match the shipped SUBSCRIPTION_FIELDS: "
        f"declared {declared}, implemented {implemented}",
    )
    require(
        dts_interface_fields(dts, "MisoObservationRequestV1") == ["requestId", "subscriptions"],
        "MisoObservationRequestV1 does not match observe()'s accepted request fields",
    )
    body = block_after(js, "async observe(request) ", "{", "}")
    request_fields = js_string_array(body, "hasExactFields(request, ")
    require(
        request_fields == ["requestId", "subscriptions"],
        f"observe() no longer accepts exactly requestId/subscriptions: {request_fields}",
    )

    # The response shapes are whatever `observe()` actually builds, exactly.
    binding = js_object_keys(body, "this.#observations.set(key, ")
    require(
        dts_interface_fields(dts, "MisoObservationBindingV1") == binding,
        "MisoObservationBindingV1 does not match the binding observe() stores: "
        f"declared {dts_interface_fields(dts, 'MisoObservationBindingV1')}, implemented {binding}",
    )
    ack = js_object_keys(body, "return Object.freeze(")
    require(
        dts_interface_fields(dts, "MisoObservationAckV1") == ack,
        "MisoObservationAckV1 does not match the acknowledgement observe() returns: "
        f"declared {dts_interface_fields(dts, 'MisoObservationAckV1')}, implemented {ack}",
    )


def check_no_literal_bound(js: str) -> None:
    """The acknowledgement bound is derived from the table, never written as a literal again."""
    require(
        "validCommandReason(message.reason)" in js,
        "the command acknowledgement no longer validates its reason through validCommandReason()",
    )
    stray = re.findall(r"message\.reason\s*(?:<=|<|>=|>|===|!==)\s*\d+", js)
    require(
        not stray,
        f"a hand-written numeric bound on message.reason is back: {stray}",
    )
    derived = re.search(
        r"function validCommandReason\(reason\) \{\s*return validU32\(reason\)"
        r" && reason < COMMAND_REASONS\.length;",
        js,
    )
    require(derived, "validCommandReason() no longer derives its bound from COMMAND_REASONS")


# --- the gate -----------------------------------------------------------------------------------


def validate(texts: dict[pathlib.Path, str], document: dict | None = None) -> None:
    spellings = {
        "rust host constants": rust_constants(texts[RUST_CONSTANTS]),
        "host JS table": host_js_table(texts[HOST_JS]),
        ".d.ts MisoCommandReasonV1": host_dts_enum(texts[HOST_DTS]),
        "metadata generator rows": metadata_generator_table(texts[METADATA_GENERATOR]),
        "schema gate list": schema_gate_list(texts[SCHEMA_GATE]),
        "generated SDK catalog": sdk_catalog_table(texts[SDK_CATALOG]),
    }
    if document is not None:
        spellings["shipped metadata JSON"] = metadata_document_rows(document)

    authority = spellings["rust host constants"]
    require(
        [value for value, _ in authority] == list(range(len(authority))),
        f"the Rust reason constants are not contiguous from 0: {authority}",
    )
    for name, rows in spellings.items():
        require(
            rows == authority,
            f"{name} disagrees with the Rust host constants:\n"
            f"  authority: {authority}\n"
            f"  {name}: {rows}",
        )

    # The worklet refuses a request or two on its own and names the reason locally. It spells only
    # the subset it produces, so it is checked as a subset -- but a value or a name that disagrees
    # with the authority is the same drift, in the one file that is on the render thread.
    worklet = worklet_js_constants(texts[WORKLET_JS])
    require(worklet, "the worklet JS names no COMMAND_REASON_* constant")
    stray = [row for row in worklet if row not in authority]
    require(
        not stray,
        f"the worklet JS names reasons the Rust host constants do not: {stray}",
    )

    check_no_literal_bound(texts[HOST_JS])
    check_observe_typing(texts[HOST_JS], texts[HOST_DTS])


def read_sources(root: pathlib.Path) -> dict[pathlib.Path, str]:
    return {source: (root / source).read_text() for source in SOURCES}


def self_test() -> int:
    texts = read_sources(REPO)
    validate(texts)

    def mutate(source: pathlib.Path, old: str, new: str):
        def apply(state: dict[pathlib.Path, str]) -> None:
            require(old in state[source], f"self-test mutation matched nothing: {old!r}")
            state[source] = state[source].replace(old, new, 1)

        return apply

    mutations: list[tuple[str, object]] = [
        # The brief's named red mutation: a reason is added to the Rust authority alone.
        (
            "a Rust reason is bumped without the other six spellings",
            mutate(
                RUST_CONSTANTS,
                "pub const COMMAND_REASON_OBSERVATION_UNBOUND: u32 = 11;",
                "pub const COMMAND_REASON_OBSERVATION_UNBOUND: u32 = 11;\n"
                "pub const COMMAND_REASON_FUTURE_TAP: u32 = 12;",
            ),
        ),
        (
            "a Rust reason is renumbered out of the contiguous run",
            mutate(
                RUST_CONSTANTS,
                "pub const COMMAND_REASON_UNKNOWN_TAP: u32 = 10;",
                "pub const COMMAND_REASON_UNKNOWN_TAP: u32 = 12;",
            ),
        ),
        # The shipped #151 defect itself, in each of the five downstream spellings.
        (
            "the host JS table stops at wrongState",
            mutate(
                HOST_JS,
                '  "unknownTap",\n  "observationUnbound",\n',
                "",
            ),
        ),
        (
            "the host JS reinstates the literal <= 9 bound",
            mutate(
                HOST_JS,
                "validCommandReason(message.reason)",
                "validU32(message.reason) && message.reason <= 9",
            ),
        ),
        (
            "the host JS bound stops deriving from the table",
            mutate(
                HOST_JS,
                "reason < COMMAND_REASONS.length;",
                "reason <= 11;",
            ),
        ),
        (
            "the .d.ts enum drops ObservationUnbound",
            mutate(HOST_DTS, "  ObservationUnbound = 11,\n", ""),
        ),
        (
            "the .d.ts enum renames a reason",
            mutate(HOST_DTS, "  UnknownTap = 10,", "  UnknownObservation = 10,"),
        ),
        (
            "the metadata generator drops a reason row",
            mutate(
                METADATA_GENERATOR,
                '        (COMMAND_REASON_OBSERVATION_UNBOUND, "observationUnbound"),\n',
                "",
            ),
        ),
        (
            "a metadata generator row emits the wrong name for its constant",
            mutate(
                METADATA_GENERATOR,
                '(COMMAND_REASON_UNKNOWN_TAP, "unknownTap")',
                '(COMMAND_REASON_UNKNOWN_TAP, "unknownObservation")',
            ),
        ),
        (
            "the worklet JS renumbers the reason it produces itself",
            mutate(
                WORKLET_JS,
                "const COMMAND_REASON_UNSUPPORTED_KIND = 7;",
                "const COMMAND_REASON_UNSUPPORTED_KIND = 12;",
            ),
        ),
        (
            "the worklet JS renames the reason it produces itself",
            mutate(
                WORKLET_JS,
                "const COMMAND_REASON_UNSUPPORTED_KIND = 7;",
                "const COMMAND_REASON_UNSUPPORTED_TAP = 7;",
            ),
        ),
        (
            "the schema gate's list stops at wrongState",
            mutate(
                SCHEMA_GATE,
                '"wrongState", "unknownTap", "observationUnbound",',
                '"wrongState",',
            ),
        ),
        (
            "the generated SDK catalog renames a reason",
            mutate(
                SDK_CATALOG,
                '"observationUnbound"',
                '"observationDetached"',
            ),
        ),
        # Issue #151's structural half.
        (
            "the .d.ts stops declaring observe()",
            mutate(
                HOST_DTS,
                "  observe(request: MisoObservationRequestV1): Promise<MisoObservationAckV1>;\n",
                "",
            ),
        ),
        (
            "the declared subscription drops a field the implementation requires",
            mutate(HOST_DTS, "  windowBlocks: number;\n  /// `true` arms", "  /// `true` arms"),
        ),
        (
            "the declared subscription adds a field the implementation refuses",
            mutate(
                HOST_DTS,
                "  armed: boolean;\n}",
                "  armed: boolean;\n  channel?: number;\n}",
            ),
        ),
        (
            "the declared binding drops frameSlot",
            mutate(HOST_DTS, "  readonly frameSlot: number;\n", ""),
        ),
        (
            "the declared acknowledgement drops reason",
            mutate(HOST_DTS, "  readonly reason: MisoCommandReasonV1;\n  /// Every armed tap", "  /// Every armed tap"),
        ),
        (
            "the implementation gains a binding field the .d.ts does not declare",
            mutate(
                HOST_JS,
                "            frameSlot: subscription.trackIndex,",
                "            frameSlot: subscription.trackIndex,\n            armed: true,",
            ),
        ),
    ]

    failures = 0
    for name, apply in mutations:
        state = dict(texts)
        try:
            apply(state)
        except Invalid as error:
            print(f"self-test FAILED: {error}", file=sys.stderr)
            failures += 1
            continue
        try:
            validate(state)
        except Invalid:
            continue
        except Exception:  # noqa: BLE001 - a mutation that crashes the parse still discriminates
            continue
        print(f"self-test FAILED: mutation escaped -- {name}", file=sys.stderr)
        failures += 1
    if failures == 0:
        print(f"command-reason vocabulary self-test passed ({len(mutations)} red mutations)")
    return failures


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--artifacts",
        type=pathlib.Path,
        help="a built artifact directory; its shipped JS, .d.ts and metadata JSON are checked "
        "instead of the working-tree web files",
    )
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()

    texts = read_sources(REPO)
    document = None
    if args.artifacts is not None:
        for source in (HOST_JS, WORKLET_JS, HOST_DTS):
            texts[source] = (args.artifacts / source.name).read_text()
        document = json.loads((args.artifacts / METADATA_DOCUMENT).read_text())
    try:
        validate(texts, document)
    except Invalid as error:
        print(f"FAIL command reason vocabulary: {error}", file=sys.stderr)
        return 1
    print("command-reason vocabulary agrees across every spelling")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
