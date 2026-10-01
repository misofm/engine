#!/usr/bin/env python3
"""Ship the browser module without its `name` section, and prove the pair are twins (issue #1109).

`scripts/build-web-audioworklet.sh` compiles the AudioWorklet module once. rustc's output, the
**named twin**, carries the WebAssembly `name` custom section: the function, local and global names
a browser's devtools print in a stack trace (`host_web::...` instead of `wasm-function[N]`). It is
about 380 KB of the module, and nothing that runs in the browser reads it: the host, the worklet
and the SDK never read `Error.stack`, a wasm frame name or a custom section, and errors cross the
boundary as typed codes. So the **shipped** module is the named twin with that one section removed.
Every other section -- the code, the data, the exports, and the `producers` and `target_features`
custom sections -- is copied byte for byte, in order, size encoding included.

The named twin stays a non-shipped debug artifact (`docs/RELEASE.md`), and the gates that find
functions by name read it: the call-graph gate, the V8 spill gate and the scalar-oracle-absent
gate. `check` is what lets their verdicts stand for the shipped bytes.

Modes
-----
`strip NAMED SHIPPED`
    Write SHIPPED, which must not exist, as NAMED minus its `name` section. NAMED must carry
    exactly one: a module with none is refused, because the gates that read the named twin would
    otherwise be handed a nameless module.
`check NAMED SHIPPED`
    Pass only when SHIPPED is exactly NAMED minus its one `name` section: the same sections in the
    same order, each byte-identical (id, size encoding and payload). So every non-custom section --
    type, function, table, memory, global, export, element, code, data -- and every other custom
    section is the same in both.
`sections MODULE`
    Print each section's id, name (for a custom section) and size in bytes, header included.
`--self-test`
    Hermetic mutations of the reader, the stripper and the check; builds nothing.

What is refused
---------------
Every mode parses the whole module first. A module is refused when it does not start with the
WebAssembly magic and version 1, when a section size is a malformed LEB128 (truncated or longer
than five bytes) or runs past the end of the module, when a section id is unknown, when a custom
section's name runs past its section or is not UTF-8, and when the non-custom sections are out of
the order the core specification fixes or one appears twice. Custom sections may appear anywhere.
The tool depends on nothing outside the Python standard library.
"""
from __future__ import annotations

import argparse
import hashlib
import pathlib
import sys

MAGIC = b"\0asm\x01\x00\x00\x00"
NAME = "name"

# The core specification's section order (WebAssembly 2.0 section 5.5.2, with the exception-
# handling proposal's tag section between memory and global). Custom sections (id 0) are free.
ORDER = {1: 1, 2: 2, 3: 3, 4: 4, 5: 5, 13: 6, 6: 7, 7: 8, 8: 9, 9: 10, 12: 11, 10: 12, 11: 13}
LABEL = {
    0: "custom", 1: "type", 2: "import", 3: "function", 4: "table", 5: "memory", 6: "global",
    7: "export", 8: "start", 9: "element", 10: "code", 11: "data", 12: "datacount", 13: "tag",
}


class Invalid(RuntimeError):
    """The module is malformed, or the pair are not twins."""


class Section:
    __slots__ = ("id", "name", "raw")

    def __init__(self, section_id: int, name: str | None, raw: bytes) -> None:
        self.id = section_id
        self.name = name  # the custom section's name; None for every other section
        self.raw = raw  # id byte, size LEB128 as encoded, payload

    def label(self) -> str:
        return f"custom {self.name!r}" if self.id == 0 else LABEL[self.id]


def leb128_u32(data: bytes, offset: int, end: int) -> tuple[int, int]:
    """Decode an unsigned LEB128 of at most five bytes (a `u32`), ending before `end`. A value over
    32 bits in five bytes needs no rule of its own: no section of it fits in a module read here."""
    result = 0
    for index in range(5):
        if offset >= end:
            raise Invalid("truncated LEB128 value")
        byte = data[offset]
        offset += 1
        result |= (byte & 0x7F) << (7 * index)
        if byte < 0x80:
            return result, offset
    raise Invalid("LEB128 value longer than five bytes")


def parse(data: bytes) -> list[Section]:
    if data[:8] != MAGIC:
        raise Invalid("not a WebAssembly version 1 module")
    sections: list[Section] = []
    rank = 0
    offset = 8
    while offset < len(data):
        start = offset
        section_id = data[offset]
        if section_id != 0 and section_id not in ORDER:
            raise Invalid(f"unknown section id {section_id} at offset {start}")
        size, payload = leb128_u32(data, offset + 1, len(data))
        end = payload + size
        if end > len(data):
            raise Invalid(f"{LABEL[section_id]} section at offset {start} runs past the end of "
                          f"the module ({size} bytes declared, {len(data) - payload} left)")
        name = None
        if section_id == 0:
            length, cursor = leb128_u32(data, payload, end)
            if cursor + length > end:
                raise Invalid(f"custom section name at offset {start} runs past its section")
            try:
                name = data[cursor:cursor + length].decode("utf-8")
            except UnicodeDecodeError:
                raise Invalid(f"custom section name at offset {start} is not UTF-8") from None
        else:
            if ORDER[section_id] <= rank:
                raise Invalid(f"{LABEL[section_id]} section at offset {start} is out of order or "
                              "repeated")
            rank = ORDER[section_id]
        sections.append(Section(section_id, name, data[start:end]))
        offset = end
    return sections


def name_sections(sections: list[Section]) -> list[int]:
    return [index for index, section in enumerate(sections)
            if section.id == 0 and section.name == NAME]


def strip(data: bytes) -> bytes:
    """`data` minus its one `name` section, every other byte in place."""
    sections = parse(data)
    names = name_sections(sections)
    if len(names) != 1:
        raise Invalid(f"the named twin must carry exactly one `name` section, found {len(names)}: "
                      "the gates that find functions by name would read a nameless module")
    return MAGIC + b"".join(section.raw for index, section in enumerate(sections)
                            if index != names[0])


def check(named: bytes, shipped: bytes) -> str:
    """A one-line verdict when `shipped` is `named` minus its `name` section; `Invalid` if not."""
    named_sections = parse(named)
    shipped_sections = parse(shipped)
    names = name_sections(named_sections)
    if len(names) != 1:
        raise Invalid(f"the named twin carries {len(names)} `name` sections, not one")
    expected = [section for index, section in enumerate(named_sections) if index != names[0]]
    if [section.raw for section in expected] != [section.raw for section in shipped_sections]:
        position = next((index for index, (want, got) in enumerate(zip(expected, shipped_sections))
                         if want.raw != got.raw), min(len(expected), len(shipped_sections)))
        want = expected[position].label() if position < len(expected) else "nothing"
        got = shipped_sections[position].label() if position < len(shipped_sections) else "nothing"
        raise Invalid(f"section {position} differs: the named twin has {want} there (its `name` "
                      f"section aside), the shipped module {got}, and they are not the same bytes")
    removed = len(named_sections[names[0]].raw)
    kept = [section for section in shipped_sections if section.id == 0]
    return (f"shipped module is the named twin minus its `name` section: named {len(named)} B, "
            f"shipped {len(shipped)} B, `name` section {removed} B; "
            f"{len(shipped_sections) - len(kept)} non-custom and {len(kept)} other custom "
            "sections byte-identical")


def section_table(data: bytes) -> list[str]:
    sections = parse(data)
    lines = [f"{'id':>2}  {'section':<26} {'bytes':>9}"]
    lines += [f"{section.id:>2}  {section.label():<26} {len(section.raw):>9}" for section in sections]
    lines.append(f"{'':>2}  {'module (8-byte header)':<26} {len(data):>9}")
    return lines


# ---- self-test ----------------------------------------------------------------------------------

def uleb(value: int) -> bytes:
    out = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        out.append(byte | (0x80 if value else 0))
        if not value:
            return bytes(out)


def section(section_id: int, payload: bytes) -> bytes:
    return bytes([section_id]) + uleb(len(payload)) + payload


def custom(name: str, body: bytes) -> bytes:
    return section(0, uleb(len(name)) + name.encode() + body)


def fixture(names: bool = True) -> bytes:
    """Two exported functions, memory, data, and the three custom sections rustc emits."""
    body = b"\x00\x41\x07\x0b"  # no locals, i32.const 7, end
    module = MAGIC
    module += section(1, b"\x01\x60\x00\x01\x7f")  # type: () -> i32
    module += section(3, b"\x02\x00\x00")  # function: two of type 0
    module += MEMORY + EXPORT
    module += section(10, b"\x02" + (uleb(len(body)) + body) * 2)
    module += section(11, b"\x01\x00\x41\x00\x0b\x03abc")  # data: active at 0, "abc"
    if names:
        function_names = b"\x02" + b"\x00\x05first" + b"\x01\x06second"
        module += custom(NAME, b"\x01" + uleb(len(function_names)) + function_names)
    return module + PRODUCERS + TARGET_FEATURES


MEMORY = section(5, b"\x01\x00\x01")  # memory: min 1 page
EXPORT = section(7, b"\x02\x01a\x00\x00\x01b\x00\x01")  # export: functions a and b
PRODUCERS = custom("producers", b"\x01\x08language\x01\x04Rust\x00")
TARGET_FEATURES = custom("target_features", b"\x01\x2b\x07simd128")


def self_test() -> int:
    failures = 0

    def expect(label: str, condition: bool) -> None:
        nonlocal failures
        if not condition:
            failures += 1
            print(f"self-test FAILED: {label}", file=sys.stderr)

    def refused(label: str, action) -> None:
        try:
            action()
        except Invalid:
            return
        expect(label, False)

    def flip(module: bytes, offset: int) -> bytes:
        return module[:offset] + bytes([module[offset] ^ 0x01]) + module[offset + 1:]

    named = fixture()
    nameless = fixture(names=False)
    shipped = strip(named)
    sections = parse(shipped)

    # (a) A module with a name section: exactly that section goes, and the check accepts the pair.
    expect("(a1) strip removes the name section and nothing else", shipped == nameless)
    expect("(a2) the check accepts the stripped twin", check(named, shipped).startswith("shipped"))
    expect("(a3) the fixture's sections are read in order",
           [s.label() for s in sections] == ["type", "function", "memory", "export", "code",
                                             "data", "custom 'producers'",
                                             "custom 'target_features'"])

    # (b) A module without a name section is refused, never passed through as a named twin.
    refused("(b1) strip refuses a module without a name section", lambda: strip(nameless))
    refused("(b2) the check refuses a nameless named twin", lambda: check(nameless, nameless))
    refused("(b3) the check refuses a shipped module that kept its names",
            lambda: check(named, named))
    refused("(b4) strip refuses two name sections", lambda: strip(named + custom(NAME, b"")))

    # (c) The check goes red on a one-byte change in each place a stripper could go wrong.
    code, data = sections[4], sections[5]
    immediate = shipped.index(code.raw) + len(code.raw) - 2  # the last body's `i32.const 7`
    planted = flip(shipped, immediate)
    expect("(c0) the planted byte changes the code section and only it",
           [a.raw == b.raw for a, b in zip(parse(planted), sections)]
           == [True, True, True, True, False, True, True, True])
    refused("(c1) a one-byte change to the code section fails the check",
            lambda: check(named, planted))
    refused("(c2) a one-byte change to the data section fails the check",
            lambda: check(named, flip(shipped, shipped.index(data.raw) + len(data.raw) - 1)))
    refused("(c3) another custom section dropped fails the check",
            lambda: check(named, shipped.replace(PRODUCERS, b"")))
    refused("(c4) a section added fails the check",
            lambda: check(named, shipped + custom("extra", b"")))
    size = len(PRODUCERS) - 2
    padded = shipped.replace(PRODUCERS, bytes([0, 0x80 | size, 0]) + PRODUCERS[2:])
    expect("(c5) the padded fixture re-encodes one size and parses",
           len(padded) == len(shipped) + 1 and len(parse(padded)) == len(sections))
    refused("(c5) a size re-encoded in more bytes fails the check",
            lambda: check(named, padded))
    swapped = shipped.replace(PRODUCERS + TARGET_FEATURES, TARGET_FEATURES + PRODUCERS)
    expect("(c6) the swapped fixture keeps every section", len(parse(swapped)) == len(sections))
    refused("(c6) two custom sections swapped fail the check", lambda: check(named, swapped))

    # (d) A malformed section length is refused, by every mode.
    shifted = flip(named, named.index(MEMORY) + 1)  # memory's size 3 -> 2: every later id moves
    refused("(d1) a section size past the end of the module is refused",
            lambda: parse(MAGIC + section(1, b"\x01\x60\x00\x01\x7f")[:-1]))
    refused("(d2) a size one short of its payload is refused", lambda: parse(shifted))
    refused("(d3) a truncated size LEB128 is refused", lambda: parse(MAGIC + b"\x01\x80"))
    refused("(d4) a six-byte size LEB128 is refused",
            lambda: parse(MAGIC + b"\x01\x80\x80\x80\x80\x80\x00"))
    refused("(d5) a custom section name past its section is refused",
            lambda: parse(MAGIC + section(0, b"\x09name")))
    refused("(d6) a custom section name that is not UTF-8 is refused",
            lambda: parse(MAGIC + section(0, b"\x01\xff")))
    refused("(d7) a module that is not WebAssembly 1 is refused",
            lambda: parse(b"\0asm\x02\x00\x00\x00"))
    refused("(d8) an unknown section id is refused", lambda: parse(MAGIC + section(14, b"")))
    refused("(d9) strip refuses a malformed named twin", lambda: strip(shifted))
    refused("(d10) the check refuses a malformed shipped module",
            lambda: check(named, flip(shipped, shipped.index(MEMORY) + 1)))

    # (e) The section order: a non-custom section out of order, or repeated, is refused.
    reordered = named.replace(MEMORY + EXPORT, EXPORT + MEMORY)
    expect("(e0) the reordered fixture moved two sections", reordered != named)
    refused("(e1) export before memory is refused", lambda: parse(reordered))
    refused("(e2) a repeated memory section is refused",
            lambda: parse(named.replace(MEMORY, MEMORY + MEMORY)))
    tagged = MAGIC + MEMORY + section(13, b"\x00") + section(6, b"\x00")
    expect("(e3) the tag section sits between memory and global", len(parse(tagged)) == 3)
    refused("(e4) a tag section after global is refused",
            lambda: parse(MAGIC + section(6, b"\x00") + section(13, b"\x00")))
    refused("(e5) a data count section after code is refused",
            lambda: parse(MAGIC + section(10, b"\x00") + section(12, b"\x00")))
    expect("(e6) custom sections may sit anywhere",
           len(parse(MAGIC + custom("a", b"") + section(1, b"\x00") + custom("b", b"")
                     + section(10, b"\x00") + custom("c", b""))) == 5)

    if failures:
        return 1
    print("strip-wasm-names self-test: every mutation caught")
    return 0


def read(path: pathlib.Path) -> bytes:
    try:
        return path.read_bytes()
    except OSError as error:
        raise Invalid(f"cannot read {path}: {error.strerror}") from None


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    commands = parser.add_subparsers(dest="command")
    for name in ("strip", "check"):
        command = commands.add_parser(name)
        command.add_argument("named", type=pathlib.Path)
        command.add_argument("shipped", type=pathlib.Path)
    commands.add_parser("sections").add_argument("module", type=pathlib.Path)
    parser.add_argument("--self-test", action="store_true")
    arguments = parser.parse_args()
    if arguments.self_test == (arguments.command is not None):
        parser.error("give one of strip, check, sections or --self-test")
    try:
        if arguments.self_test:
            return self_test()
        if arguments.command == "sections":
            print("\n".join(section_table(read(arguments.module))))
            return 0
        named = read(arguments.named)
        if arguments.command == "check":
            print(check(named, read(arguments.shipped)))
            return 0
        shipped = strip(named)
        try:
            with open(arguments.shipped, "xb") as output:
                output.write(shipped)
        except OSError as error:
            raise Invalid(f"cannot write {arguments.shipped}: {error.strerror}") from None
        print(f"named twin {digest(named)} ({len(named)} B); shipped module {digest(shipped)} "
              f"({len(shipped)} B), {len(named) - len(shipped)} B of `name` section removed")
        return 0
    except Invalid as error:
        print(f"FAIL {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
