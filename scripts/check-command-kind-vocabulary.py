#!/usr/bin/env python3
"""One command-KIND vocabulary, proved across every file that spells it (issues #137, #143, #210).

The sibling gate `check-command-reason-vocabulary.py` proves the *reason* vocabulary across seven
spellings. The *kind* vocabulary had no such proof, and by the time this gate was written it had
already drifted -- silently, in the shipped artifact:

* `hosts/miso-engine-host-web/src/lib.rs` -- the `COMMAND_*` constants. This is the authority.
* the same file's `CommandRecord::decode` whitelist -- the `matches!` arm that decides what the
  48-byte wire actually accepts. A constant the whitelist does not name is a kind no caller can
  send; a whitelist entry the constants do not name cannot exist.
* `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js` -- the `COMMAND_KINDS` set
  `validCommand` gates every submitted record through.
* `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` -- the
  `MisoCommandKind` enum a typed consumer compiles against.
* `tools/miso-engine-parameter-metadata/src/lib.rs` -- the `commandKinds` rows of the shipped JSON,
  which is where an app reads the vocabulary from.
* `scripts/check-parameter-metadata-v1.py` -- the schema gate's deliberately independent list.
* the shipped `miso-engine-v2-parameter-metadata.json` itself, under `--artifacts`.

**The drift this gate was written to close**: the Rust constants, the decode whitelist, the host JS
set and the `.d.ts` enum all carried eight kinds, while the metadata generator's table and the
schema gate's list stopped at six -- `observeSubscribe` (7) and `observeUnsubscribe` (8) were
absent from the one file an app reads its vocabulary from -- and the schema gate pinned kind values
to the literal `range(1, 7)`, so the missing two could not be noticed there either. Six-versus-eight
for two releases, with no gate on it.

**The observation-kinds ruling this gate enforces.** Kinds 7 and 8 join `commandKinds`: the
vocabulary is complete at whatever the Rust constants say it is -- eight when this gate was
written, nine since issue #210 phase 1 added `solo` (9). They are not, however, DSP kinds, so every
row carries `plane`:

* `applied` keeps its issue #140 meaning -- the ABI applies this kind rather than declaring and
  refusing it. That is true of all eight (`admit_commands` binds or unbinds the tap and
  acknowledges `none`), so `applied: false` would have been a *false* marker: it would say the ABI
  refuses a kind it does not refuse.
* `plane` is the marker that distinguishes them: `"render"` for the kinds that move state the
  render thread reads, `"observation"` for the two that move an entry in the `miso.observe.v1`
  subscription map and change nothing rendered. `solo` is a render kind: it carries no strip
  parameter of its own, but what it composes to -- the fader section's mute -- is state the render
  thread reads.

So the #140 invariant survives verbatim -- every declared kind is applied -- and the schema gate
additionally pins *which* plane each kind is applied on, in both directions.

Every source must yield the same ordered `value -> camelCase name` mapping, contiguous from `1`,
and the two planes must partition it identically wherever a source spells the split.

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
HOST_DTS = pathlib.Path("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts")
METADATA_GENERATOR = pathlib.Path("tools/miso-engine-parameter-metadata/src/lib.rs")
SCHEMA_GATE = pathlib.Path("scripts/check-parameter-metadata-v1.py")

SOURCES = (RUST_CONSTANTS, HOST_JS, HOST_DTS, METADATA_GENERATOR, SCHEMA_GATE)

METADATA_DOCUMENT = "miso-engine-v2-parameter-metadata.json"

# `pub const COMMAND_*` in the host is three families, not one: the kinds, the reasons (the sibling
# gate's business) and one size constant. Naming the non-kinds here rather than pattern-matching
# the kinds keeps the scan fail-closed -- a new `COMMAND_*` constant that is neither a reason nor
# `RECORD_BYTES` is read as a kind and has to agree with every other spelling or go red.
NON_KIND_COMMAND_CONSTANTS = frozenset({"RECORD_BYTES"})
REASON_PREFIX = "REASON_"

PLANE_RENDER = "render"
PLANE_OBSERVATION = "observation"


class Invalid(Exception):
    """One rule this gate enforces was broken."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


def camel_from_screaming(name: str) -> str:
    """`OBSERVE_SUBSCRIBE` -> `observeSubscribe`."""
    head, *rest = name.lower().split("_")
    return head + "".join(word.capitalize() for word in rest)


def camel_from_pascal(name: str) -> str:
    """`ObserveSubscribe` -> `observeSubscribe`."""
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


# --- the seven spellings ------------------------------------------------------------------------


def rust_constants(text: str) -> list[tuple[int, str]]:
    """The authority: `pub const COMMAND_<KIND>: u32 = <value>;`, reasons and sizes excluded."""
    rows = [
        (int(value), camel_from_screaming(name))
        for name, value in re.findall(
            r"^pub const COMMAND_([A-Z0-9_]+): u32 = (\d+);", text, re.MULTILINE
        )
        if not name.startswith(REASON_PREFIX) and name not in NON_KIND_COMMAND_CONSTANTS
    ]
    require(rows, "no COMMAND_* kind constants found in the Rust host")
    return sorted(rows)


def rust_decode_whitelist(text: str) -> list[str]:
    """The `matches!` arm in `CommandRecord::decode` -- what the 48-byte wire actually accepts."""
    block = block_after(text, "let kind = u32::from(bytes[0]);", "(", ")")
    names = [
        camel_from_screaming(name)
        for name in re.findall(r"COMMAND_([A-Z0-9_]+)", block)
        if not name.startswith(REASON_PREFIX)
    ]
    require(names, "the decode whitelist names no COMMAND_* kind")
    return names


def host_js_set(text: str) -> list[int]:
    """The host JS gates `validCommand` on this set; it spells values, not names."""
    block = block_after(text, "const COMMAND_KINDS = new Set(", "[", "]")
    values = [int(value) for value in re.findall(r"\d+", block)]
    require(values, "the host JS COMMAND_KINDS set is empty")
    return sorted(values)


def dts_enum_members(text: str) -> list[tuple[int, str, str]]:
    """`(value, camelCase name, the doc comment that precedes it)` for the `.d.ts` enum."""
    block = block_after(text, "export const enum MisoCommandKind ", "{", "}")
    members: list[tuple[int, str, str]] = []
    doc: list[str] = []
    for line in block.splitlines():
        stripped = line.strip()
        if stripped.startswith("///"):
            doc.append(stripped)
            continue
        match = re.match(r"^([A-Z][A-Za-z0-9]*) = (\d+),$", stripped)
        if match:
            members.append((int(match.group(2)), camel_from_pascal(match.group(1)), "\n".join(doc)))
            doc = []
        elif stripped:
            doc = []
    require(members, "the .d.ts MisoCommandKind enum is empty")
    return sorted(members)


def host_dts_enum(text: str) -> list[tuple[int, str]]:
    return [(value, name) for value, name, _ in dts_enum_members(text)]


def dts_observation_plane(text: str) -> list[str]:
    """The enum members whose doc marks them as applying on the `miso.observe.v1` plane."""
    return [name for _, name, doc in dts_enum_members(text) if "`miso.observe.v1`" in doc]


def generator_planes(text: str) -> dict[str, str]:
    """`PLANE_RENDER` -> `"render"`, read from the generator's own constants."""
    planes = dict(re.findall(r'^pub const (PLANE_[A-Z]+): &str = "([a-z]+)";', text, re.MULTILINE))
    require(planes, "the metadata generator declares no PLANE_* constants")
    return planes


def metadata_generator_table(text: str) -> tuple[list[tuple[int, str]], list[str]]:
    """The generator's `commandKinds` rows, and the names it puts on the observation plane.

    Row position stands for value here, exactly as it does in the shipped document (the schema gate
    requires `commandKinds[i]["value"] == i + 1`). The values themselves are the Rust constants --
    the generator interpolates them rather than writing numbers -- so the JSON's own value column,
    checked under `--artifacts`, is what closes the loop.
    """
    block = block_after(text, "let kinds = ", "[", "]")
    # Whitespace-tolerant on purpose: rustfmt explodes a row whose contents exceed `fn_call_width`
    # onto five lines, and a gate that a reformat can silently disarm is not a gate.
    rows = re.findall(
        r"\(\s*COMMAND_([A-Z0-9_]+),\s*"
        r'"([A-Za-z][A-Za-z0-9]*)",\s*(true|false),\s*(PLANE_[A-Z]+),?\s*\)',
        block,
    )
    require(rows, "the metadata generator's commandKinds table is empty")
    planes = generator_planes(text)
    mismatched = [
        (camel_from_screaming(constant), emitted)
        for constant, emitted, _, _ in rows
        if camel_from_screaming(constant) != emitted
    ]
    require(
        not mismatched,
        f"a metadata generator row emits a name that is not its own constant's name: {mismatched}",
    )
    unapplied = [emitted for _, emitted, applied, _ in rows if applied != "true"]
    require(
        not unapplied,
        "issue #140: every declared kind is applied, so no generator row may say otherwise: "
        f"{unapplied}",
    )
    unknown = [plane for _, _, _, plane in rows if planes.get(plane) is None]
    require(not unknown, f"a generator row names a plane constant that is not declared: {unknown}")
    observation = [
        emitted for _, emitted, _, plane in rows if planes[plane] == PLANE_OBSERVATION
    ]
    render = [emitted for _, emitted, _, plane in rows if planes[plane] == PLANE_RENDER]
    require(
        len(observation) + len(render) == len(rows),
        "a generator row is on neither the render nor the observation plane",
    )
    return list(enumerate([emitted for _, emitted, _, _ in rows], start=1)), observation


def schema_gate_list(text: str) -> list[tuple[int, str]]:
    """The schema gate's independent list. Position stands for value, which is its own rule."""
    block = block_after(text, "\nCOMMAND_KINDS = ", "[", "]")
    names = re.findall(r'"([A-Za-z][A-Za-z0-9]*)"', block)
    require(names, "the schema gate's COMMAND_KINDS list is empty")
    return list(enumerate(names, start=1))


def schema_gate_observation_plane(text: str) -> list[str]:
    block = block_after(text, "\nOBSERVE_COMMAND_KINDS = ", "[", "]")
    names = re.findall(r'"([A-Za-z][A-Za-z0-9]*)"', block)
    require(names, "the schema gate's OBSERVE_COMMAND_KINDS list is empty")
    return names


def metadata_document_rows(document: dict) -> tuple[list[tuple[int, str]], list[str]]:
    rows = document["commandKinds"]
    kinds = sorted((int(row["value"]), row["name"]) for row in rows)
    observation = [row["name"] for row in rows if row["plane"] == PLANE_OBSERVATION]
    return kinds, observation


# --- the derived-bound rules ---------------------------------------------------------------------


def check_no_literal_bound(js: str, schema_gate: str) -> None:
    """Neither downstream spelling may write the vocabulary's size as a number again.

    The host JS asks the set; the schema gate derives its value range from the list's length. The
    literal `range(1, 7)` is exactly how the shipped document was allowed to stop two kinds short
    of the wire without any gate noticing.
    """
    require(
        "COMMAND_KINDS.has(command.kind)" in js,
        "validCommand no longer gates the record's kind on the COMMAND_KINDS set",
    )
    stray = re.findall(r"command\.kind\s*(?:<=|<|>=|>|===|!==)\s*\d+", js)
    require(not stray, f"a hand-written numeric bound on command.kind is back: {stray}")
    require(
        "list(range(1, len(COMMAND_KINDS) + 1))" in schema_gate,
        "the schema gate no longer derives its command-kind value range from COMMAND_KINDS",
    )
    literal = re.findall(r"list\(range\(1,\s*\d+\)\)", schema_gate)
    require(
        not literal,
        f"the schema gate wrote a literal command-kind bound again: {literal}",
    )


# --- the gate -------------------------------------------------------------------------------------


def validate(texts: dict[pathlib.Path, str], document: dict | None = None) -> None:
    generator_rows, generator_observation = metadata_generator_table(texts[METADATA_GENERATOR])
    spellings = {
        "rust host constants": rust_constants(texts[RUST_CONSTANTS]),
        ".d.ts MisoCommandKind": host_dts_enum(texts[HOST_DTS]),
        "metadata generator rows": generator_rows,
        "schema gate list": schema_gate_list(texts[SCHEMA_GATE]),
    }
    planes = {
        ".d.ts MisoCommandKind": dts_observation_plane(texts[HOST_DTS]),
        "metadata generator rows": generator_observation,
        "schema gate list": schema_gate_observation_plane(texts[SCHEMA_GATE]),
    }
    if document is not None:
        document_rows, document_observation = metadata_document_rows(document)
        spellings["shipped metadata JSON"] = document_rows
        planes["shipped metadata JSON"] = document_observation

    authority = spellings["rust host constants"]
    require(
        [value for value, _ in authority] == list(range(1, len(authority) + 1)),
        f"the Rust kind constants are not contiguous from 1: {authority}",
    )
    for name, rows in spellings.items():
        require(
            rows == authority,
            f"{name} disagrees with the Rust host constants:\n"
            f"  authority: {authority}\n"
            f"  {name}: {rows}",
        )

    # The wire's own answer. A constant nothing decodes is a kind that cannot be sent; a decoded
    # kind no constant names cannot be spelled by any of the five tables above.
    whitelist = rust_decode_whitelist(texts[RUST_CONSTANTS])
    require(
        sorted(whitelist) == sorted(name for _, name in authority),
        "the decode whitelist and the Rust kind constants are not the same set:\n"
        f"  constants: {[name for _, name in authority]}\n"
        f"  whitelist: {whitelist}",
    )

    # The host JS spells values, not names -- it is a shape validator, not a vocabulary. Values are
    # the whole of what it has to agree about, and the #210 drift class ("a kind exists on the wire
    # and not in the set") is exactly a value disagreement.
    js_values = host_js_set(texts[HOST_JS])
    require(
        js_values == [value for value, _ in authority],
        "the host JS COMMAND_KINDS set disagrees with the Rust host constants:\n"
        f"  authority: {[value for value, _ in authority]}\n"
        f"  host JS: {js_values}",
    )

    # The observation-plane split, wherever a spelling states it. Exact in both directions: a DSP
    # kind marked observation would claim it renders nothing, and an observation kind marked render
    # would join the applied-DSP set the issue #140 rule stands on.
    observation = planes["schema gate list"]
    require(observation, "the schema gate names no observation-plane kinds")
    names = [name for _, name in authority]
    require(
        all(name in names for name in observation),
        f"the observation plane names a kind the vocabulary does not: {observation}",
    )
    for name, rows in planes.items():
        require(
            rows == observation,
            f"{name} disagrees about which kinds apply on the observation plane:\n"
            f"  schema gate: {observation}\n"
            f"  {name}: {rows}",
        )

    check_no_literal_bound(texts[HOST_JS], texts[SCHEMA_GATE])


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
        # The brief's named red mutation: a kind added to the Rust authority alone. It names the
        # next unclaimed value, so this stays the "added and not threaded" shape rather than a
        # duplicate of a kind that already ships -- issue #210 phase 1 spent kind 9 on `solo`, and
        # phase 3 spent 10 and 11 on `trimDb` and `polarityInvert`, so the next free value is 12.
        (
            "a Rust kind is added without the other spellings",
            mutate(
                RUST_CONSTANTS,
                "pub const COMMAND_POLARITY_INVERT: u32 = 11;",
                "pub const COMMAND_POLARITY_INVERT: u32 = 11;\n"
                "pub const COMMAND_SOLO_MODE: u32 = 12;",
            ),
        ),
        (
            "a Rust kind is renumbered out of the contiguous run",
            mutate(
                RUST_CONSTANTS,
                "pub const COMMAND_MUTE: u32 = 4;",
                "pub const COMMAND_MUTE: u32 = 9;",
            ),
        ),
        (
            "a Rust kind is renamed",
            mutate(
                RUST_CONSTANTS,
                "pub const COMMAND_FADER_DB: u32 = 3;",
                "pub const COMMAND_FADER_GAIN: u32 = 3;",
            ),
        ),
        (
            "the decode whitelist drops a kind the constants declare",
            mutate(RUST_CONSTANTS, "                | COMMAND_OBSERVE_UNSUBSCRIBE\n", ""),
        ),
        (
            "the decode whitelist admits something that is not a kind",
            mutate(
                RUST_CONSTANTS,
                "                | COMMAND_OBSERVE_UNSUBSCRIBE\n",
                "                | COMMAND_OBSERVE_UNSUBSCRIBE\n"
                "                | COMMAND_RECORD_BYTES\n",
            ),
        ),
        # The #210 drift class itself: present in Rust, missing from the JS set.
        (
            "the host JS set stops at effectBypass",
            mutate(
                HOST_JS,
                "const COMMAND_KINDS = new Set([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);",
                "const COMMAND_KINDS = new Set([1, 2, 3, 4, 5, 6]);",
            ),
        ),
        (
            "the host JS set stops one kind short of the wire",
            mutate(
                HOST_JS,
                "const COMMAND_KINDS = new Set([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);",
                "const COMMAND_KINDS = new Set([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);",
            ),
        ),
        (
            "the host JS set gains a kind the wire does not decode",
            mutate(
                HOST_JS,
                "const COMMAND_KINDS = new Set([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);",
                "const COMMAND_KINDS = new Set([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);",
            ),
        ),
        (
            "the host JS writes the kind bound as a literal",
            mutate(
                HOST_JS,
                "COMMAND_KINDS.has(command.kind)",
                "command.kind >= 1 && command.kind <= 11",
            ),
        ),
        (
            "the host JS stops asking the set at all",
            mutate(HOST_JS, "COMMAND_KINDS.has(command.kind)", "validU32(command.kind)"),
        ),
        (
            "the .d.ts enum drops ObserveUnsubscribe",
            mutate(HOST_DTS, "  ObserveUnsubscribe = 8,\n", ""),
        ),
        (
            "the .d.ts enum renames a kind",
            mutate(HOST_DTS, "  ObserveSubscribe = 7,", "  ObserveArm = 7,"),
        ),
        (
            "the .d.ts stops marking an observation kind as observation-plane",
            mutate(
                HOST_DTS,
                "  /// Applied on the `miso.observe.v1` plane: it clears an entry from the "
                "subscription map. The\n  /// metadata JSON reports it as "
                '`"plane": "observation"`.\n',
                "",
            ),
        ),
        (
            "the .d.ts marks a DSP kind as observation-plane",
            mutate(
                HOST_DTS,
                "  /// Set a lane mute, as a fader endpoint over the same window. "
                "Applied (issue #140 B).",
                "  /// Set a lane mute. Applied on the `miso.observe.v1` plane.",
            ),
        ),
        (
            "the metadata generator drops the observeUnsubscribe row",
            mutate(
                METADATA_GENERATOR,
                "        (\n"
                "            COMMAND_OBSERVE_UNSUBSCRIBE,\n"
                '            "observeUnsubscribe",\n'
                "            true,\n"
                "            PLANE_OBSERVATION,\n"
                "        ),\n",
                "",
            ),
        ),
        (
            "a metadata generator row emits the wrong name for its constant",
            mutate(
                METADATA_GENERATOR,
                '(COMMAND_FADER_DB, "faderDb", true, PLANE_RENDER)',
                '(COMMAND_FADER_DB, "faderGain", true, PLANE_RENDER)',
            ),
        ),
        (
            "a metadata generator row puts an observation kind on the render plane",
            mutate(
                METADATA_GENERATOR,
                '            "observeSubscribe",\n            true,\n'
                "            PLANE_OBSERVATION,\n",
                '            "observeSubscribe",\n            true,\n'
                "            PLANE_RENDER,\n",
            ),
        ),
        (
            "a metadata generator row declares a kind unapplied",
            mutate(
                METADATA_GENERATOR,
                '            "observeSubscribe",\n            true,\n',
                '            "observeSubscribe",\n            false,\n',
            ),
        ),
        (
            "a generator plane constant is respelled",
            mutate(
                METADATA_GENERATOR,
                'pub const PLANE_OBSERVATION: &str = "observation";',
                'pub const PLANE_OBSERVATION: &str = "observe";',
            ),
        ),
        (
            "the schema gate's list stops at effectBypass",
            mutate(
                SCHEMA_GATE,
                '    "observeSubscribe", "observeUnsubscribe", "solo", "trimDb", "polarityInvert",'
                "\n",
                "",
            ),
        ),
        (
            "the schema gate's list drops the render kind added last",
            mutate(
                SCHEMA_GATE,
                '"observeSubscribe", "observeUnsubscribe", "solo", "trimDb", "polarityInvert",',
                '"observeSubscribe", "observeUnsubscribe", "solo", "trimDb",',
            ),
        ),
        (
            "the .d.ts enum drops the render kind added last",
            mutate(HOST_DTS, "  PolarityInvert = 11,\n", ""),
        ),
        (
            "the .d.ts enum drops the first of the two phase-3 kinds",
            mutate(HOST_DTS, "  TrimDb = 10,\n", ""),
        ),
        (
            "the metadata generator puts solo on the observation plane",
            mutate(
                METADATA_GENERATOR,
                '(COMMAND_SOLO, "solo", true, PLANE_RENDER)',
                '(COMMAND_SOLO, "solo", true, PLANE_OBSERVATION)',
            ),
        ),
        (
            "the metadata generator puts trimDb on the observation plane",
            mutate(
                METADATA_GENERATOR,
                '(COMMAND_TRIM_DB, "trimDb", true, PLANE_RENDER)',
                '(COMMAND_TRIM_DB, "trimDb", true, PLANE_OBSERVATION)',
            ),
        ),
        (
            "the metadata generator emits the wrong name for the polarity constant",
            mutate(
                METADATA_GENERATOR,
                '            "polarityInvert",\n',
                '            "polarity",\n',
            ),
        ),
        (
            "the decode whitelist drops solo",
            mutate(RUST_CONSTANTS, "                | COMMAND_SOLO\n", ""),
        ),
        (
            "the decode whitelist drops trimDb",
            mutate(RUST_CONSTANTS, "                | COMMAND_TRIM_DB\n", ""),
        ),
        (
            "the decode whitelist drops polarityInvert",
            mutate(RUST_CONSTANTS, "                | COMMAND_POLARITY_INVERT\n", ""),
        ),
        (
            "the .d.ts marks the live polarity kind as observation-plane",
            mutate(
                HOST_DTS,
                "  /// Set or clear a lane's input polarity inversion. Applied (issue 210 "
                "phase 3).",
                "  /// Set or clear a lane's input polarity inversion. Applied on the "
                "`miso.observe.v1` plane.",
            ),
        ),
        (
            "the schema gate reinstates the literal range(1, 7)",
            mutate(
                SCHEMA_GATE,
                "list(range(1, len(COMMAND_KINDS) + 1))",
                "list(range(1, 7))",
            ),
        ),
        (
            "the schema gate's observation list disagrees",
            mutate(
                SCHEMA_GATE,
                'OBSERVE_COMMAND_KINDS = ["observeSubscribe", "observeUnsubscribe"]',
                'OBSERVE_COMMAND_KINDS = ["observeSubscribe"]',
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
        print(f"command-kind vocabulary self-test passed ({len(mutations)} red mutations)")
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
        for source in (HOST_JS, HOST_DTS):
            texts[source] = (args.artifacts / source.name).read_text()
        document = json.loads((args.artifacts / METADATA_DOCUMENT).read_text())
    try:
        validate(texts, document)
    except Invalid as error:
        print(f"FAIL command kind vocabulary: {error}", file=sys.stderr)
        return 1
    print("command-kind vocabulary agrees across every spelling")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
