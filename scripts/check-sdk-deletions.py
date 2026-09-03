#!/usr/bin/env python3
"""Deletion gate for the pre-boot-v1 SDK's retired spellings (issue #243 S1, eval 9).

Boot v1's whole claim is that the guess machine was **deleted rather than relocated**. That is a
claim about absence, and absence is the one property a test suite structurally cannot demonstrate:
every eval in `sdk/test/` exercises code that exists, so a `sessionHeader` regex that survived in a
corner nothing imports, or a second copy of the 29-field configuration writer kept "for the browser
path", would leave the suite green. Issue #207's N-13(d) is the standing evidence that this is the
failure mode to fear rather than a hypothetical one: it counted five copies of the configuration
table, one of which wrote a 192-byte structure's offsets into a 64-byte buffer and produced garbage
in silence, and every one of those copies was reachable, compiled and untested. So the sentenced
spellings get a grep gate, which is the only kind of check that can fail on something not being
there.

# Comments are exempt, and deliberately so

Every rule below runs over the SDK's source with its comments blanked out. That is not a loophole,
it is the point: `sdk/src/core/boundary.ts` opens by naming `sessionHeader`, `validationFallback`,
`SessionPlan` and `PrepareLimits` in prose, because a reader who does not know what was deleted
cannot tell a simplification from an omission. A gate that could not tell that paragraph from a
live declaration would force the SDK to forget its own history in order to pass. So the ban is on
the spelling appearing as *code*, and the comment stripper (which blanks comments in place, so
reported line numbers stay true) is what draws the line.

# The rules that had to discriminate rather than match

Three of the sentenced spellings survive somewhere legitimate, and a flat string ban on any of them
would be a gate that has to be weakened the first time it fires:

* **`sampleRateHz`.** The SESSION-level rate is the whole point of Session V1 -- one document, one
  rate -- and `session().sampleRateHz`, `SessionShape.sampleRateHz` and
  `defaultSourceRingFrames(sampleRateHz, ...)` are all live surface. What #241/A2 deleted is the
  *per-source* rate. So the rule is stated where a per-source field would have to live: no
  `SourceSpec` or `SourceShape` interface body may declare it, no emitted `sources` row may carry
  `sample_rate_hz`, and no expression may spell `source.sampleRateHz` or `spec.sampleRateHz`.
* **`startFrame` / `start_sample`.** `submitSource({ startFrame })` is the absolute frame a staged
  PCM block begins at -- a call argument, not a document field -- and `start_sample` is an
  automation segment's u64 bound, which the schema still carries. Both are banned only in the same
  three per-source positions as the rate.
* **`reprepareRequired`.** It is a live result-code *name* in the frozen ladder, and
  `sdk/src/browser/policy.ts` correctly raises it when an `AudioContext`'s quantum disagrees with
  the document. What died with the two-phase lifecycle is its use as an error *phase*, so the rule
  is a vocabulary check on the phase position: every `phase: "..."`, `error.phase === "..."` and
  `assert(x.phase, "...")` must name one of the six phases the generated layout publishes, which
  refuses `compile` and `reprepareRequired` alike without touching the result-code name.

# Byte offsets

`PrepareLimits`, `DEFAULT_LIMITS` and the literal `192` are the surface of the deeper rule, which
is worth enforcing on its own terms: **no SDK file outside `src/generated/` may contain a numeric
byte offset for an ABI structure.** Every offset is resolved by field name through `ABI_LAYOUT`, so
a field that moves moves its accessor with it and a field that is renamed fails at the lookup
rather than at the wrong address. The rule is applied to the files that actually touch the ABI --
those naming `ABI_LAYOUT` or `WebAssembly` -- because a `DataView` over a freshly allocated
four-byte buffer, which is how `sdk/src/core/decimal.ts` and the canonical writer's float speller
punt a float to its bits, addresses no structure and offset zero there means nothing.

# The positive half

A gate that only forbids passes trivially on an empty directory. So the replacements are asserted
too: `sdk/src/core/abi.ts` must export `BootOptions`, `writeBootOptions` and
`defaultSourceRingFrames` and must resolve its fields by name; the generated layout must publish
the six error phases and the four-call staging sequence; `sdk/src/core/errors.ts` must derive
`ErrorPhase` from that document and name all six in its prose; and every export in the staging
sequence must actually be called somewhere in the SDK.

`--self-test` injects each banned spelling into an in-memory copy of a real SDK file and requires
the gate to catch every one, then requires the untouched tree to pass, so the validator is proved
to discriminate before it is trusted.
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

GENERATED_ABI = "sdk/src/generated/abi.ts"
CORE_ABI = "sdk/src/core/abi.ts"
CORE_ERRORS = "sdk/src/core/errors.ts"
CORE_ASSET = "sdk/src/core/asset.ts"
CORE_SESSION = "sdk/src/core/session.ts"
CORE_TYPES = "sdk/src/core/types.ts"
CORE_BOUNDARY = "sdk/src/core/boundary.ts"

ERROR_PHASES = ["asset", "boot", "source", "render", "output", "lifecycle"]

# The identifiers #243 S1 sentenced outright: there is no position in which any of them is
# legitimate, so the rule is the plain one.
DELETED_IDENTIFIERS = {
    "sessionHeader": "the raw-TOML header regex, which could not see a quoted key",
    "validationFallback": "the silent 48 kHz / 128 frame fallback",
    "writeConfig": "the 29-field configuration writer",
    "PrepareLimits": "the 192-byte-era limits structure",
    "SessionLimits": "the document-side limits surface #241 deleted",
    "DEFAULT_LIMITS": "the invented default limits table",
    "pcmRingFrames": "the ring field whose 1024 default was not a multiple of a 127-frame quantum",
    "pcm_ring_frames": "the wire spelling of the same field",
    "SessionPlan": "the plan fabricated solely to invent a ring",
    "prepareRejected": "the two-phase lifecycle's refusal name",
}

# `limits` as a key, a member or a document root key. Spelled as three patterns rather than as a
# bare word because the word itself appears inside `PrepareLimits` and in ordinary English.
LIMITS_PATTERNS = [
    (re.compile(r"\blimits\s*\??\s*:"), "a `limits` key"),
    (re.compile(r"\.\s*limits\b"), "a `.limits` member access"),
    (re.compile(r"""(["'])limits\1"""), "a quoted `limits` key"),
]

# Issue #338 retires every format-bound SDK spelling rather than retaining aliases.
RETIRED_SESSION_FORMAT = re.compile(r"\b(?:sessionTomlBytes|sessionToml)\b|\btoToml\s*\(")

# The two retired engine states. Banned as bare string literals: the SDK has no configuration file
# and no prepare step, so these exact tokens have no other reason to be quoted here.
RETIRED_STATES = re.compile(r"""(["'])(config|prepared)\1""")

# The phase position, in the three shapes it takes: a literal in an error init, a comparison, and
# an assertion's expected value.
PHASE_USES = [
    re.compile(r"""\bphase\s*:\s*(["'])([A-Za-z_][A-Za-z0-9_]*)\1"""),
    re.compile(r"""\bphase\s*[=!]==?\s*(["'])([A-Za-z_][A-Za-z0-9_]*)\1"""),
    re.compile(r"""\.\s*phase\s*,\s*(["'])([A-Za-z_][A-Za-z0-9_]*)\1"""),
]

# The per-source spellings #241/A2 deleted, in the three positions where a per-source field can
# actually appear. `startSample` is absent on purpose: it is an automation segment's bound.
PER_SOURCE_FIELDS = ["sampleRateHz", "startFrame", "start_frame", "start_sample", "sample_rate_hz"]
PER_SOURCE_INTERFACES = [
    (CORE_TYPES, "SourceSpec"),
    (CORE_BOUNDARY, "SourceShape"),
]
PER_SOURCE_ACCESS = re.compile(
    r"\b(?:source|spec|sourceSpec|sourceShape)\s*\.\s*(sampleRateHz|startFrame)\b"
)

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
# The retired prepare-config structure's size. It has no other meaning in an SDK that reads every
# structure's width out of the generated layout.
RETIRED_STRUCT_SIZE = re.compile(r"(?<![\w.])192(?![\w.])")

# Files that address the engine's memory or read its layout. The offset rules apply to these.
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
    `file:line`, and keeping string literals is what lets the phase-vocabulary and retired-state
    rules look at the quoted words that carry the actual meaning.
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
    """Every SDK file the deletion rules apply to, keyed by repo-relative path."""
    files: dict[str, str] = {}
    for path in sorted((root / SDK).rglob("*")):
        if not path.is_file() or path.suffix not in SCANNED_SUFFIXES:
            continue
        relative = path.relative_to(root).as_posix()
        if relative.startswith(SKIPPED_PREFIXES):
            continue
        files[relative] = path.read_text(encoding="utf-8")
    # The generated layout is exempt from every ban but is the authority the positive half reads.
    generated = root / GENERATED_ABI
    if generated.is_file():
        files[GENERATED_ABI] = generated.read_text(encoding="utf-8")
    return files


def scanned(files: dict[str, str]) -> dict[str, str]:
    """The comment-stripped text of every file under the ban, generated tree excluded."""
    return {
        path: strip_comments(text)
        for path, text in files.items()
        if not path.startswith(SKIPPED_PREFIXES)
    }


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


def interface_body(text: str, name: str) -> str | None:
    """The brace-balanced body of `interface <name> { ... }`, or `None` if it is not declared."""
    match = re.search(rf"\binterface\s+{re.escape(name)}\s*\{{", text)
    if match is None:
        return None
    depth = 0
    start = match.end() - 1
    for index in range(start, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[start + 1:index]
    return None


def emitted_source_row(session: str) -> str | None:
    """The record literal `normalize()` builds for each `sources` row of the emitted document."""
    match = re.search(r"\bstate\s*\.\s*sources\s*\.?\s*\n?\s*\.\s*map\s*\(", session)
    if match is None:
        return None
    depth = 0
    start = match.end() - 1
    for index in range(start, len(session)):
        if session[index] == "(":
            depth += 1
        elif session[index] == ")":
            depth -= 1
            if depth == 0:
                return session[start + 1:index]
    return None


def json_string_array(text: str, key: str) -> list[str]:
    match = re.search(rf'"{re.escape(key)}"\s*:\s*\[(.*?)\]', text, re.DOTALL)
    require(match is not None, f"the generated layout does not publish {key}")
    assert match is not None
    return re.findall(r'"([^"]*)"', match.group(1))


def check_deleted_identifiers(code: dict[str, str]) -> None:
    for name, what in DELETED_IDENTIFIERS.items():
        refuse(find_all(code, re.compile(rf"\b{re.escape(name)}\b")), f"`{name}` -- {what}")


def check_limits(code: dict[str, str], files: dict[str, str]) -> None:
    for pattern, what in LIMITS_PATTERNS:
        refuse(find_all(code, pattern), what)
    # The root key list is the canonical document's own table of contents, so a `limits` root key
    # would have to appear in it. Checked separately because it is the one place the word would be
    # a plain quoted string rather than a key or a member.
    root_keys = re.search(r"\bROOT_KEYS\s*=\s*\[(.*?)\]", code.get(CORE_SESSION, ""), re.DOTALL)
    require(root_keys is not None, f"{CORE_SESSION} no longer declares ROOT_KEYS")
    assert root_keys is not None
    require("limits" not in re.findall(r'"([^"]*)"', root_keys.group(1)),
            "the emitted session document's root keys carry `limits` again")


def check_lifecycle_vocabulary(code: dict[str, str]) -> None:
    refuse(find_all(code, RETIRED_SESSION_FORMAT), "a retired TOML session surface")
    refuse(find_all(code, RETIRED_STATES), "a retired two-phase lifecycle state word")
    for pattern in PHASE_USES:
        for path, text in sorted(code.items()):
            for match in pattern.finditer(text):
                phase = match.group(2)
                require(phase in ERROR_PHASES,
                        f"{path}:{line_of(text, match.start())} names `{phase}` in the error-phase "
                        f"position; the six phases are {ERROR_PHASES}")


def check_per_source_fields(code: dict[str, str]) -> None:
    for path, name in PER_SOURCE_INTERFACES:
        body = interface_body(code.get(path, ""), name)
        require(body is not None, f"{path} no longer declares the {name} interface")
        assert body is not None
        for field in PER_SOURCE_FIELDS:
            require(re.search(rf"\b{re.escape(field)}\s*\??\s*:", body) is None,
                    f"{name} in {path} declares the deleted per-source field `{field}` (#241/A2)")

    row = emitted_source_row(code.get(CORE_SESSION, ""))
    require(row is not None, f"{CORE_SESSION} no longer normalizes its sources through state.sources")
    assert row is not None
    for field in PER_SOURCE_FIELDS:
        require(re.search(rf"\b{re.escape(field)}\s*:", row) is None,
                f"the emitted `sources` row in {CORE_SESSION} carries the deleted key `{field}`")

    refuse(find_all(code, PER_SOURCE_ACCESS), "a per-source rate or start-frame member access")


def check_numeric_offsets(code: dict[str, str]) -> None:
    abi_files = {path: text for path, text in code.items() if ABI_TOUCHING.search(text)}
    refuse(find_all(abi_files, NUMERIC_OFFSET),
           "a numeric byte offset on a DataView accessor (offsets resolve by name through "
           "ABI_LAYOUT)")
    refuse(find_all(abi_files, OFFSET_CONSTANT), "a hand-written byte-offset constant")
    refuse(find_all(abi_files, RETIRED_STRUCT_SIZE),
           "the retired 192-byte structure size as a literal")


def check_replacements(files: dict[str, str], code: dict[str, str]) -> None:
    abi = files.get(CORE_ABI, "")
    for declaration in ("export interface BootOptions",
                       "export function writeBootOptions(",
                       "export function defaultSourceRingFrames("):
        require(declaration in abi, f"{CORE_ABI} no longer carries `{declaration}`")
    abi_code = code.get(CORE_ABI, "")
    require("ABI_LAYOUT.structures[structure].fields" in abi_code,
            f"{CORE_ABI} no longer reads its structures out of the generated layout")
    require("row.name === name" in abi_code,
            f"{CORE_ABI} no longer resolves fields by name")

    generated = files.get(GENERATED_ABI, "")
    require(generated, f"{GENERATED_ABI} is missing; the SDK has no layout to resolve against")
    require(json_string_array(generated, "errorPhases") == ERROR_PHASES,
            f"the generated layout's errorPhases is not {ERROR_PHASES}")
    require('export type ErrorPhase = AbiLayout["errorPhases"][number];' in generated,
            f"{GENERATED_ABI} no longer derives ErrorPhase from the published phases")

    errors = files.get(CORE_ERRORS, "")
    require("ErrorPhase" in errors and '"../generated/abi.ts"' in errors,
            f"{CORE_ERRORS} no longer takes its phase vocabulary from the generated layout")
    for phase in ERROR_PHASES:
        require(f"`{phase}`" in errors, f"{CORE_ERRORS} no longer names the `{phase}` phase")

    staging = json_string_array(generated, "stagingSequence")
    require(len(staging) == 4,
            f"the generated staging sequence is {len(staging)} calls, not the four #243 S5 pinned")
    require("ABI_LAYOUT.stagingSequence" in code.get(CORE_ASSET, ""),
            f"{CORE_ASSET} no longer reads the staging sequence out of the generated layout")
    for step in staging:
        require(any(step in text for text in code.values()),
                f"no SDK call site calls the staging sequence's {step}")


def validate(files: dict[str, str]) -> None:
    code = scanned(files)
    require(code, "no SDK sources were found to check")
    check_deleted_identifiers(code)
    check_limits(code, files)
    check_lifecycle_vocabulary(code)
    check_per_source_fields(code)
    check_numeric_offsets(code)
    check_replacements(files, code)


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

    def insert_after(path: str, anchor: str, snippet: str):
        def mutate(files: dict[str, str]) -> None:
            require(path in files and anchor in files[path],
                    f"self-test anchor `{anchor}` is gone from {path}")
            files[path] = files[path].replace(anchor, anchor + snippet, 1)
        return mutate

    def replace(path: str, before: str, after: str, count: int = -1):
        def mutate(files: dict[str, str]) -> None:
            require(path in files and before in files[path],
                    f"self-test anchor `{before}` is gone from {path}")
            files[path] = files[path].replace(before, after, count)
        return mutate

    def replace_once(path: str, before: str, after: str):
        return replace(path, before, after, 1)

    mutations = [
        ("the raw-TOML header regex returns",
         append(CORE_BOUNDARY, '\nconst sessionHeader = /^\\[session\\]/m;\n')),
        ("the silent rate fallback returns",
         append(CORE_BOUNDARY, "\nconst validationFallback = { rate: 48000, quantum: 128 };\n")),
        ("the 29-field config writer returns",
         append(CORE_BOUNDARY, "\nfunction writeConfig(): void {}\n")),
        ("the 192-byte limits structure returns",
         append(CORE_ABI, "\nexport interface PrepareLimits { readonly tracks: number }\n")),
        ("the document-side limits structure returns",
         append(CORE_SESSION, "\ninterface SessionLimits { readonly tracks: number }\n")),
        ("the invented default limits table returns",
         append(CORE_ABI, "\nconst DEFAULT_LIMITS = { tracks: 64 };\n")),
        ("the pcmRingFrames trap returns",
         append(CORE_ABI, "\nconst staging = { pcmRingFrames: 1024 };\n")),
        ("the wire spelling of the ring field returns",
         append(CORE_SESSION, '\nconst key = { pcm_ring_frames: 1024 };\n')),
        ("the fabricated plan returns",
         append(CORE_BOUNDARY, "\ninterface SessionPlan { readonly rate: number }\n")),
        ("a limits key returns",
         append(CORE_SESSION, "\nconst plan = { limits: { tracks: 64 } };\n")),
        ("a limits member access returns",
         append(CORE_BOUNDARY, "\nconst depth = arguments.limits;\n")),
        ("a limits root key returns",
         insert_after(CORE_SESSION, "const ROOT_KEYS = [\n", '  "limits",\n')),
        ("SourceSpec regains a per-source rate",
         insert_after(CORE_TYPES, "export interface SourceSpec {",
                      "\n  readonly sampleRateHz: number;")),
        ("SourceShape regains a per-source start frame",
         insert_after(CORE_BOUNDARY, "export interface SourceShape {",
                      "\n  readonly startFrame: bigint;")),
        ("the emitted source row regains its rate",
         insert_after(CORE_SESSION, "      content: spec.content,",
                      "\n      sample_rate_hz: 48000,")),
        ("the emitted source row regains a start frame",
         insert_after(CORE_SESSION, "      content: spec.content,",
                      "\n      start_frame: 0,")),
        ("a per-source rate is read off a spec",
         append(CORE_SESSION, "\nconst rate = spec.sampleRateHz;\n")),
        ("a per-source start frame is read off a source",
         append(CORE_SESSION, "\nconst begin = source.startFrame;\n")),
        ("a DataView offset is written as a number",
         append(CORE_ABI,
                "\nfunction stale(view: DataView): number { return view.getUint32(24, true); }\n")),
        ("a hand-written offset constant returns",
         append(CORE_BOUNDARY, "\nconst STATUS_STATE_OFFSET = 20;\n")),
        ("the 192-byte structure size returns as a literal",
         append(CORE_ABI, "\nconst prepareConfigBytes = 192;\n")),
        ("the two-phase refusal name returns",
         append(CORE_ERRORS, '\nconst dead: string = "prepareRejected";\n')),
        ("compile returns as an error phase",
         append(CORE_BOUNDARY, '\nconst dead = { phase: "compile", code: "internal" };\n')),
        ("reprepareRequired is used as an error phase",
         append(CORE_BOUNDARY, '\nconst dead = { phase: "reprepareRequired" };\n')),
        ("a phase comparison names a retired phase",
         append(CORE_BOUNDARY, '\nconst dead = error.phase === "compile";\n')),
        ("an assertion expects a retired phase",
         append("sdk/test/boot-evals.mjs", '\nassert.equal(error.phase, "compile");\n')),
        ("the config state returns",
         append(CORE_BOUNDARY, '\nconst state: string = "config";\n')),
        ("the prepared state returns",
         append(CORE_BOUNDARY, '\nconst state: string = "prepared";\n')),
        ("the retired session buffer kind returns",
         append(CORE_BOUNDARY, '\nconst kind = "sessionToml";\n')),
        ("the retired canonical writer returns",
         append(CORE_SESSION, '\nfunction toToml(): string { return ""; }\n')),
        ("the boot options writer stops being exported",
         replace_once(CORE_ABI, "export function writeBootOptions(",
                      "function writeBootOptions(")),
        ("the ring derivation is deleted",
         replace_once(CORE_ABI, "export function defaultSourceRingFrames(",
                      "function retiredRingRule(")),
        ("abi.ts stops resolving fields by name",
         replace_once(CORE_ABI, "row.name === name", "row.offset === 0")),
        ("errors.ts stops naming a phase",
         replace_once(CORE_ERRORS, "`lifecycle`", "`compile`")),
        ("the generated layout loses a phase",
         replace_once(GENERATED_ABI, '"lifecycle"', '"compile"')),
        ("the staging sequence drops back to three calls",
         replace_once(GENERATED_ABI, '    "miso_engine_web_v1_boot_options_ptr",\n', "")),
        ("asset.ts stops reading the staging sequence",
         replace(CORE_ASSET, "ABI_LAYOUT.stagingSequence[0]",
                 '"miso_engine_web_v1_abi_version"')),
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
    if failures:
        return 1
    print(f"sdk deletion gate self-test passed ({len(mutations)} mutations caught)")
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
        print(f"FAIL sdk deletions: {error}", file=sys.stderr)
        return 1
    scanned_count = len([path for path in files if not path.startswith(SKIPPED_PREFIXES)])
    print(f"sdk deletions: ok ({scanned_count} files carry none of the retired spellings)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
