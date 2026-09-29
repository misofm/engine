#!/usr/bin/env python3
"""No SDK file outside `src/generated/` addresses an ABI structure by a numeric byte offset.

Issue #243 S1 made the SDK resolve every ABI field **by name** through `ABI_LAYOUT`, the layout
generated from the engine's own `offset_of!` output, so a field that moves moves its accessor with
it and a field that is renamed fails at the lookup rather than at the wrong address. Issue #207's
N-13(d) is the standing evidence for why that is worth a gate: it counted five hand-kept copies of
the configuration table, one of which wrote a 192-byte structure's offsets into a 64-byte buffer
and produced garbage in silence, and every one of those copies was reachable, compiled and
untested. A test suite exercises the code that exists; only a scan can fail on an offset that
should not be written down at all.

The rule is applied to the files that actually touch the ABI -- those naming `ABI_LAYOUT` or
`WebAssembly` -- because a `DataView` over a freshly allocated four-byte buffer, which is how
`sdk/src/core/decimal.ts` and the canonical writer's float speller punt a float to its bits,
addresses no structure, and offset zero there means nothing. Comments are blanked in place before
the scan (line numbers stay true), so prose may describe an offset without tripping the gate.
The positive half requires `sdk/src/core/abi.ts` to keep reading its structures out of the
generated layout and resolving fields by name, so the gate cannot pass on a tree that stopped
resolving offsets at all.

History. This gate began as #243 S1's deletion gate for the pre-boot-v1 SDK's retired spellings
(`sessionHeader`, `PrepareLimits`, a `limits` key, the `"config"`/`"prepared"` states, per-source
rates, retired error phases, the TOML session surface, the literal `192`). #1050 retired those
bans: they caught only exact re-adds of deleted code, and in 73 CI runs they fired four times, each
time on new, legitimate code (`"prepared"` in `prepared-control.js` and `pcm-feed`, and a
`limits:` key in `live-response.ts` and `response.ts`). The numeric-offset rule is the one that
guards a live defect class, and it is what remains. The file keeps its name because the CI router
and its policy checker route on it.

`--self-test` injects each violation into an in-memory copy of a real SDK file and requires the
gate to catch it, and requires a commented-out offset and the untouched tree to pass.
"""

from __future__ import annotations

import argparse
import copy
import pathlib
import re
import sys

SDK = "sdk"
# Generated, vendored, or shipped-asset trees. `src/generated/` is emitted from the engine's own
# `offset_of!` output and is the one place a numeric offset belongs; the other two are not ours.
SKIPPED_PREFIXES = ("sdk/assets/", "sdk/dist/", "sdk/node_modules/", "sdk/src/generated/")
SCANNED_SUFFIXES = (".ts", ".mjs", ".js")

CORE_ABI = "sdk/src/core/abi.ts"
CORE_BOUNDARY = "sdk/src/core/boundary.ts"

# A `DataView` accessor whose byte offset is written as a number rather than resolved by name.
NUMERIC_OFFSET = re.compile(
    r"\.\s*(?:get|set)(?:Big)?(?:Uint8|Uint16|Uint32|Uint64|Int8|Int16|Int32|Int64|Float32|Float64)"
    r"\s*\(\s*(?:0[xX][0-9a-fA-F]+|[0-9]+)"
)
# A hand-written offset constant, which is how the fifth configuration copy carried its table.
OFFSET_CONSTANT = re.compile(
    r"\b[A-Za-z_][A-Za-z0-9_]*(?:Offset|OFFSET|offset)\s*(?::\s*number\s*)?=\s*"
    r"(?:0[xX][0-9a-fA-F]+|[0-9]+)"
)

# Files that address the engine's memory or read its layout. The offset rule applies to these.
ABI_TOUCHING = re.compile(r"\bABI_LAYOUT\b|\bWebAssembly\b")

# Identifier characters, for deciding whether a `/` opens a regex literal or divides.
_WORD = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_$"
_REGEX_KEYWORDS = {
    "return", "typeof", "case", "in", "of", "new", "delete", "void", "instanceof",
    "yield", "await", "do", "else", "throw",
}


class Invalid(Exception):
    """One rule this gate enforces was broken."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


def _skip_string(text: str, index: int, quote: str) -> int:
    index += 1
    while index < len(text):
        if text[index] == "\\":
            index += 2
            continue
        if text[index] == quote or text[index] == "\n":
            return index + 1
        index += 1
    return index


def _skip_template(text: str, index: int) -> int:
    index += 1
    while index < len(text):
        char = text[index]
        if char == "\\":
            index += 2
            continue
        if char == "`":
            return index + 1
        if char == "$" and text[index + 1: index + 2] == "{":
            depth = 1
            index += 2
            while index < len(text) and depth > 0:
                inner = text[index]
                if inner == "{":
                    depth += 1
                    index += 1
                elif inner == "}":
                    depth -= 1
                    index += 1
                elif inner in "\"'":
                    index = _skip_string(text, index, inner)
                elif inner == "`":
                    index = _skip_template(text, index)
                else:
                    index += 1
            continue
        index += 1
    return index


def _skip_regex(text: str, index: int) -> int | None:
    """End of a regex literal starting at `index`, or `None` if this `/` was a division."""
    cursor = index + 1
    in_class = False
    while cursor < len(text):
        char = text[cursor]
        if char == "\\":
            cursor += 2
            continue
        if char == "\n":
            return None
        if char == "[":
            in_class = True
        elif char == "]":
            in_class = False
        elif char == "/" and not in_class:
            return cursor + 1
        cursor += 1
    return None


def strip_comments(text: str) -> str:
    """Blank every comment in place, leaving strings, offsets and line numbers untouched.

    Blanking rather than deleting is what lets a violation still be reported at its true
    `file:line`.
    """
    out = list(text)
    index = 0
    length = len(text)
    previous = ""
    word = ""
    while index < length:
        char = text[index]
        following = text[index + 1] if index + 1 < length else ""
        if char == "/" and following == "/":
            while index < length and text[index] != "\n":
                out[index] = " "
                index += 1
            previous, word = "", ""
            continue
        if char == "/" and following == "*":
            while index < length and not (text[index] == "*" and text[index + 1: index + 2] == "/"):
                if text[index] != "\n":
                    out[index] = " "
                index += 1
            if index < length:
                out[index] = " "
                if index + 1 < length:
                    out[index + 1] = " "
                index += 2
            previous, word = "", ""
            continue
        if char in "\"'":
            index = _skip_string(text, index, char)
            previous, word = char, ""
            continue
        if char == "`":
            index = _skip_template(text, index)
            previous, word = "`", ""
            continue
        if char == "/":
            opens_regex = (
                previous == ""
                or word in _REGEX_KEYWORDS
                or previous not in _WORD + ")]\"'`"
            )
            end = _skip_regex(text, index) if opens_regex else None
            if end is not None:
                index = end
                previous, word = "/", ""
                continue
        if not char.isspace():
            word = word + char if char in _WORD else ""
            previous = char
        index += 1
    return "".join(out)


def line_of(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def load(root: pathlib.Path) -> dict[str, str]:
    """Every SDK file the offset rule applies to, keyed by repo-relative path."""
    files: dict[str, str] = {}
    for path in sorted((root / SDK).rglob("*")):
        if not path.is_file() or path.suffix not in SCANNED_SUFFIXES:
            continue
        relative = path.relative_to(root).as_posix()
        if relative.startswith(SKIPPED_PREFIXES):
            continue
        files[relative] = path.read_text(encoding="utf-8")
    return files


def scanned(files: dict[str, str]) -> dict[str, str]:
    """The comment-stripped text of every file under the rule."""
    return {path: strip_comments(text) for path, text in files.items()}


def find_all(code: dict[str, str], pattern: re.Pattern[str]) -> list[tuple[str, int, str]]:
    hits: list[tuple[str, int, str]] = []
    for path, text in sorted(code.items()):
        for match in pattern.finditer(text):
            hits.append((path, line_of(text, match.start()), match.group(0).strip()))
    return hits


def refuse(hits: list[tuple[str, int, str]], what: str) -> None:
    if hits:
        where = "; ".join(f"{path}:{line} ({snippet})" for path, line, snippet in hits[:4])
        raise Invalid(f"{what} is present in SDK code: {where}")


def check_numeric_offsets(code: dict[str, str]) -> None:
    abi_files = {path: text for path, text in code.items() if ABI_TOUCHING.search(text)}
    require(CORE_ABI in abi_files, f"{CORE_ABI} no longer reads the generated layout")
    refuse(find_all(abi_files, NUMERIC_OFFSET),
           "a numeric byte offset on a DataView accessor (offsets resolve by name through "
           "ABI_LAYOUT)")
    refuse(find_all(abi_files, OFFSET_CONSTANT), "a hand-written byte-offset constant")


def check_name_resolution(code: dict[str, str]) -> None:
    abi_code = code.get(CORE_ABI, "")
    require("ABI_LAYOUT.structures[structure].fields" in abi_code,
            f"{CORE_ABI} no longer reads its structures out of the generated layout")
    require("row.name === name" in abi_code,
            f"{CORE_ABI} no longer resolves fields by name")


def validate(files: dict[str, str]) -> None:
    code = scanned(files)
    require(code, "no SDK sources were found to check")
    check_numeric_offsets(code)
    check_name_resolution(code)


def self_test(root: pathlib.Path) -> int:
    sample = load(root)
    try:
        validate(sample)
    except Invalid as error:
        print(f"self-test FAILED: the real tree was rejected -- {error}", file=sys.stderr)
        return 1

    def append(path: str, snippet: str):
        def mutate(files: dict[str, str]) -> None:
            require(path in files, f"self-test anchor file {path} is gone")
            files[path] = files[path] + snippet
        return mutate

    def replace_once(path: str, before: str, after: str):
        def mutate(files: dict[str, str]) -> None:
            require(path in files and before in files[path],
                    f"self-test anchor `{before}` is gone from {path}")
            files[path] = files[path].replace(before, after, 1)
        return mutate

    mutations = [
        ("a DataView offset is written as a number",
         append(CORE_ABI,
                "\nfunction stale(view: DataView): number { return view.getUint32(24, true); }\n")),
        ("a hexadecimal DataView offset is written as a number",
         append(CORE_BOUNDARY,
                "\nfunction stale(view: DataView): void { view.setBigUint64(0x10, 0n, true); }\n")),
        ("a hand-written offset constant returns",
         append(CORE_BOUNDARY, "\nconst STATUS_STATE_OFFSET = 20;\n")),
        ("abi.ts stops reading its structures out of the generated layout",
         replace_once(CORE_ABI, "ABI_LAYOUT.structures[structure].fields", "STRUCTURES[structure]")),
        ("abi.ts stops resolving fields by name",
         replace_once(CORE_ABI, "row.name === name", "row.offset === 0")),
    ]

    failures = 0
    for name, mutate in mutations:
        broken = copy.deepcopy(sample)
        mutate(broken)
        try:
            validate(broken)
        except Invalid:
            continue
        print(f"self-test FAILED: mutation escaped -- {name}", file=sys.stderr)
        failures += 1

    # Prose may name an offset: a commented-out accessor is not code.
    commented = copy.deepcopy(sample)
    append(CORE_ABI, "\n// The pre-#243 writer said view.getUint32(24, true); it is gone.\n")(commented)
    try:
        validate(commented)
    except Invalid as error:
        print(f"self-test FAILED: a comment was reported as code -- {error}", file=sys.stderr)
        failures += 1

    if failures:
        return 1
    print(f"sdk offset gate self-test passed ({len(mutations)} mutations caught, "
          "1 comment admitted)")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=pathlib.Path,
                        default=pathlib.Path(__file__).resolve().parent.parent)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    root = args.root.resolve()
    if args.self_test:
        return self_test(root)
    files = load(root)
    try:
        validate(files)
    except Invalid as error:
        print(f"FAIL sdk offsets: {error}", file=sys.stderr)
        return 1
    print(f"sdk offsets: ok ({len(files)} files; no numeric ABI byte offset outside "
          "src/generated/)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
