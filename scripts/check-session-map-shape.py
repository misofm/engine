#!/usr/bin/env python3
"""One session-map shape, proved across every file that spells it (issue #207).

Before issue #207 the browser ABI exposed track discovery and nothing at all about sources, so a
headless consumer compiling raw session TOML could not learn which sources a session declares, how
many channels each carries, or which frames it is waiting for -- it could not drive the render loop
it had just compiled. The introspection queries close that, and closing it means the same shape is
now written out in five places:

* `hosts/miso-engine-host-web/src/ffi.rs` -- the exports. This is the authority, and it is where
  each field's *width* is decided: a `u32` export becomes a JavaScript `number`, a `u64` export
  becomes a `bigint`, and the `.d.ts` has to say so or a typed consumer compiles against a lie.
* `scripts/check-web-audioworklet.sh` -- the shipped module's frozen export set. An export that
  exists in the crate and not in that list is an export nothing proves is shipped.
* `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js` -- the worklet, which calls the
  exports once at construction and posts the map.
* `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js` -- the main-realm host,
  whose acknowledgement validator fails the WHOLE host on a field set it does not expect. This is
  the #151 failure shape exactly: a map that grew a field the validator did not know about would
  not degrade, it would take the session down with a sticky 255.
* `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` -- the declaration an SDK
  generates against. It is the *generated-against* surface, so a drift here is a drift in
  everything downstream of it.

The drift class this gate makes red: a source field added, removed, renamed or retyped in one of
those five without the other four. `--self-test` mutates in-memory copies and requires each
mutation to be caught, so the gate is proved to discriminate before it is trusted.

Extending it: the introspection family is derived from the Rust FFI by *signature* -- a handle and
at most an index -- but the name pattern is `source_`, because that is the only family the map
carries today. When the output-bus and route lists arrive on this same mechanism (#210 phase 4),
the family regexes below take a second prefix and the field/type derivation applies unchanged; the
rules do not need rewriting, only pointing at one more list.

Run with no arguments to check the tree, or `--artifacts DIR` to additionally hold a built artifact
directory's shipped JS/`.d.ts` to the same rules.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent

RUST_FFI = pathlib.Path("hosts/miso-engine-host-web/src/ffi.rs")
EXPORT_GATE = pathlib.Path("scripts/check-web-audioworklet.sh")
WORKLET_JS = pathlib.Path("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet.js")
HOST_JS = pathlib.Path("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js")
HOST_DTS = pathlib.Path("hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts")

SOURCES = (RUST_FFI, EXPORT_GATE, WORKLET_JS, HOST_JS, HOST_DTS)

# The one hand-written link in this gate: the `id` field is not read as a scalar, it is copied
# through the source-ID staging buffer and decoded byte by byte, so its export returns a byte
# length rather than the value. Every other field's TypeScript type is derived, not written.
ID_FIELD = "id"
ID_EXPORT = "miso_engine_web_v1_source_id"
WIDTH_TO_TS = {"u32": "number", "u64": "bigint"}


class Invalid(Exception):
    """One rule this gate enforces was broken."""


def require(condition: object, message: str) -> None:
    if not condition:
        raise Invalid(message)


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
    at = text.find(anchor)
    require(at >= 0, f"anchor not found: {anchor!r}")
    start = text.index(opening, at + len(anchor) - 1)
    return balanced(text, start, opening, closing)


def object_keys(block: str) -> list[str]:
    """Top-level keys of one object-literal block, in source order."""
    keys: list[str] = []
    depth = 0
    for line in block.splitlines():
        stripped = line.strip()
        if depth == 1 and not stripped.startswith("//"):
            match = re.match(r"([A-Za-z_][A-Za-z0-9_]*)\s*:", stripped)
            if match:
                keys.append(match.group(1))
        depth += line.count("{") + line.count("[") + line.count("(")
        depth -= line.count("}") + line.count("]") + line.count(")")
    return keys


# --- the five spellings --------------------------------------------------------------------------


def rust_introspection_exports(text: str) -> dict[str, str]:
    """`export name -> return width`, for the source queries only.

    The PCM feed path (`source_submit`, `source_seek`) shares the `source_` prefix and is
    deliberately excluded by *signature*, not by name: an introspection query takes the handle and
    at most an index, which is what makes it answerable without touching the render plane.
    """
    found: dict[str, str] = {}
    for name, params, width in re.findall(
        r'pub extern "C" fn (miso_engine_web_v1_source_[a-z_]+)\(([^)]*)\) -> (u32|u64)', text
    ):
        signature = [part.strip() for part in params.split(",") if part.strip()]
        if signature in (["handle: u32"], ["handle: u32", "index: u32"]):
            found[name] = width
    require(found, "no source-introspection exports found in the Rust FFI")
    require(
        "miso_engine_web_v1_source_count" in found,
        "the source-introspection family has no count export, which is its bounds authority",
    )
    return found


def gate_export_list(text: str) -> set[str]:
    """The continuation-joined names in the gate's `expected_exports` list."""
    at = text.find("expected_exports=$(printf")
    require(at >= 0, "the shipped-export gate no longer declares expected_exports")
    end = text.index("| sort)", at)
    listed = re.findall(r"\bmiso_engine_web_v1_[a-z_]+\b", text[at:end])
    require(listed, "the shipped-export gate list is empty")
    return set(listed)


def worklet_called_exports(text: str) -> set[str]:
    """Every source query the worklet's construction path calls.

    The PCM feed path lives in `receiveSource`/`receiveSeek`, not here, so a `source_` name found in
    this body is an introspection query by construction.
    """
    body = block_after(text, "\n  bindConsole(init) ", "{", "}")
    called = set(
        re.findall(r"this\.exports\.(miso_engine_web_v1_source_[a-z_]+)\(", body)
    )
    require(called, "the worklet's construction path calls no source query at all")
    return called


def worklet_source_reads(text: str) -> dict[str, str]:
    """`field name -> export name`, as the worklet actually wires them together."""
    body = block_after(text, "\n  bindConsole(init) ", "{", "}")
    reads = {
        field: export
        for field, export in re.findall(
            r"const ([A-Za-z][A-Za-z0-9]*) = "
            r"this\.exports\.(miso_engine_web_v1_source_[a-z_]+)\(this\.handle, index\);",
            body,
        )
        # The ID query returns a byte length into staging, not the value, so its local name is the
        # length rather than a record field; the field it feeds is bound below.
        if export != ID_EXPORT
    }
    require(reads, "the worklet reads no per-source scalar query")
    require(
        f"this.exports.{ID_EXPORT}(this.handle, index)" in body,
        f"the worklet does not copy source IDs through {ID_EXPORT}",
    )
    require(ID_FIELD not in reads, f"{ID_FIELD!r} is copied through staging, not read as a scalar")
    reads[ID_FIELD] = ID_EXPORT
    return reads


def worklet_pushed_fields(text: str) -> list[str]:
    body = block_after(text, "\n  bindConsole(init) ", "{", "}")
    match = re.search(r"this\.sources\.push\(\{([^}]*)\}\);", body)
    require(match, "the worklet does not push a per-source record with shorthand fields")
    fields = [part.strip() for part in match.group(1).split(",") if part.strip()]
    require(
        all(re.fullmatch(r"[A-Za-z][A-Za-z0-9]*", field) for field in fields),
        f"the pushed source record is not plain shorthand fields: {fields}",
    )
    return fields


def worklet_map_literal(text: str) -> str:
    at = text.index('tag: "miso.sessionmap.v1",')
    return balanced(text, text.rindex("{", 0, at), "{", "}")


def worklet_posted_source_fields(literal: str) -> list[str]:
    inner = block_after(literal, "sources: this.sources.map(", "{", "}")
    return object_keys(inner)


def host_js_map_fields(text: str) -> list[str]:
    """The field set the acknowledgement validator demands of a `sessionMap` response."""
    marker = 'pending.response === "sessionMap"\n'
    at = text.find(marker)
    require(at >= 0, "the host no longer branches its expected fields on the sessionMap response")
    block = balanced(text, text.index("[", at + len(marker)), "[", "]")
    return re.findall(r'"([A-Za-z][A-Za-z0-9]*)"', block)


def host_js_source_fields(text: str) -> list[str]:
    block = block_after(text, "const SESSION_SOURCE_FIELDS = ", "[", "]")
    return re.findall(r'"([A-Za-z][A-Za-z0-9]*)"', block)


def dts_interface(text: str, name: str) -> dict[str, str]:
    """`field -> declared type text`, for one exported interface."""
    block = block_after(text, f"export interface {name} ", "{", "}")
    rows = re.findall(
        r"^\s*(?:readonly\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([^;]+);", block, re.MULTILINE
    )
    require(rows, f"the .d.ts interface {name} declares no fields")
    return dict(rows)


# --- the rules -----------------------------------------------------------------------------------


def check(texts: dict[pathlib.Path, str]) -> None:
    exports = rust_introspection_exports(texts[RUST_FFI])

    shipped = gate_export_list(texts[EXPORT_GATE])
    missing = sorted(set(exports) - shipped)
    require(
        not missing,
        "source-introspection exports exist in the crate but are not in the shipped-export gate "
        f"list, so nothing proves they ship: {missing}",
    )

    called = worklet_called_exports(texts[WORKLET_JS])
    require(
        called == set(exports),
        "the worklet does not call exactly the source-introspection family: "
        f"calls {sorted(called - set(exports))} that are not introspection queries, and misses "
        f"{sorted(set(exports) - called)}",
    )
    reads = worklet_source_reads(texts[WORKLET_JS])

    pushed = worklet_pushed_fields(texts[WORKLET_JS])
    require(
        set(pushed) == set(reads),
        f"the worklet reads {sorted(reads)} but records {sorted(pushed)}",
    )

    literal = worklet_map_literal(texts[WORKLET_JS])
    posted_map = object_keys(literal)
    require(
        "sources" in posted_map,
        "the posted session map carries no source list -- issue #207's whole point",
    )
    posted_source = worklet_posted_source_fields(literal)
    require(
        posted_source == pushed,
        f"the posted source rows are {posted_source}, the recorded rows are {pushed}",
    )

    host_map = host_js_map_fields(texts[HOST_JS])
    require(
        sorted(host_map) == sorted(posted_map),
        "the host's session-map acknowledgement validator expects "
        f"{sorted(host_map)} but the worklet posts {sorted(posted_map)} -- a mismatch fails the "
        "whole host with a sticky 255, which is the issue #151 failure shape",
    )
    host_source = host_js_source_fields(texts[HOST_JS])
    require(
        sorted(host_source) == sorted(pushed),
        f"the host validates source fields {sorted(host_source)}, the worklet posts "
        f"{sorted(pushed)}",
    )

    declared_map = dts_interface(texts[HOST_DTS], "MisoSessionMapV1")
    require(
        sorted(declared_map) == sorted(posted_map),
        f"the .d.ts declares session-map fields {sorted(declared_map)}, the worklet posts "
        f"{sorted(posted_map)}",
    )
    element = declared_map["sources"].strip()
    require(
        element.endswith("[]"),
        f"MisoSessionMapV1.sources is not an array type: {element!r}",
    )
    element_name = element[:-2].strip()
    declared_source = dts_interface(texts[HOST_DTS], element_name)
    require(
        sorted(declared_source) == sorted(pushed),
        f"the .d.ts {element_name} declares {sorted(declared_source)}, the worklet posts "
        f"{sorted(pushed)}",
    )

    # The width half. A `u64` export read into a field the `.d.ts` calls a `number` is the drift a
    # consumer discovers at runtime, when a `bigint` refuses to compare with a `Number`.
    for field, export in sorted(reads.items()):
        expected = "string" if field == ID_FIELD else WIDTH_TO_TS[exports[export]]
        actual = declared_source[field].strip()
        require(
            actual == expected,
            f"{element_name}.{field} is declared {actual!r} but {export} returns "
            f"{'a copied ASCII ID' if field == ID_FIELD else exports[export]}, which is "
            f"{expected!r} in JavaScript",
        )

    # Issue #215 removes all version scoping at this sprint's close. Nothing new may arrive already
    # carrying the suffix the sweep exists to delete.
    require(
        not element_name.endswith("V1"),
        f"{element_name} mints a new V1 name; issue #215 removes version scoping, so a type "
        "introduced now must not carry the suffix",
    )


def read_sources(root: pathlib.Path) -> dict[pathlib.Path, str]:
    return {source: (root / source).read_text(encoding="utf-8") for source in SOURCES}


def check_artifacts(directory: pathlib.Path, texts: dict[pathlib.Path, str]) -> None:
    """The shipped JS/`.d.ts` must be the tree's, so a stale artifact fails here too."""
    for source in (WORKLET_JS, HOST_JS, HOST_DTS):
        shipped = directory / source.name
        require(shipped.is_file(), f"the artifact directory has no {source.name}")
        require(
            shipped.read_text(encoding="utf-8") == texts[source],
            f"the shipped {source.name} is not the tree's {source}",
        )


def self_test() -> None:
    texts = read_sources(REPO)
    check(texts)

    def mutate(source: pathlib.Path, before: str, after: str) -> dict[pathlib.Path, str]:
        require(
            texts[source].count(before) >= 1,
            f"self-test mutation matched nothing in {source}: {before!r}",
        )
        copy = dict(texts)
        copy[source] = texts[source].replace(before, after, 1)
        require(copy[source] != texts[source], f"self-test mutation was a no-op in {source}")
        return copy

    mutations = [
        (
            "an export is added to the crate without reaching the shipped-export gate",
            mutate(
                EXPORT_GATE,
                "  miso_engine_web_v1_source_start_frame \\\n",
                "",
            ),
        ),
        (
            "the worklet stops reading one query",
            mutate(
                WORKLET_JS,
                "      const startFrame = this.exports."
                "miso_engine_web_v1_source_start_frame(this.handle, index);\n",
                "      const startFrame = 0n;\n",
            ),
        ),
        (
            "the worklet records a field it does not post",
            mutate(
                WORKLET_JS,
                "this.sources.push({ id, channels, sampleRateHz, startFrame, frames });",
                "this.sources.push({ id, channels, sampleRateHz, startFrame });",
            ),
        ),
        (
            "the posted row drops a field",
            mutate(WORKLET_JS, "          frames: source.frames,\n", ""),
        ),
        (
            "the host's acknowledgement validator does not expect the source list",
            mutate(
                HOST_JS,
                '["tag", "requestId", "result", "tracks", "sources", "metersAttached"]',
                '["tag", "requestId", "result", "tracks", "metersAttached"]',
            ),
        ),
        (
            "the host validates a stale per-source field set",
            mutate(
                HOST_JS,
                'const SESSION_SOURCE_FIELDS = ["id", "channels", "sampleRateHz", '
                '"startFrame", "frames"];',
                'const SESSION_SOURCE_FIELDS = ["id", "channels", "startFrame", "frames"];',
            ),
        ),
        (
            "the .d.ts session map does not declare the source list",
            mutate(
                HOST_DTS,
                "  readonly sources: MisoSessionSource[];\n",
                "",
            ),
        ),
        (
            "the .d.ts narrows a u64 region to a JavaScript number",
            mutate(HOST_DTS, "  readonly frames: bigint;", "  readonly frames: number;"),
        ),
        (
            "the .d.ts widens the source ID to something that is not a string",
            mutate(HOST_DTS, "  readonly id: string;", "  readonly id: unknown;"),
        ),
        (
            "a new type arrives already carrying the V1 suffix issue #215 deletes",
            mutate(HOST_DTS, "MisoSessionSource", "MisoSessionSourceV1"),
        ),
    ]
    for what, mutated in mutations:
        try:
            check(mutated)
        except Invalid:
            continue
        raise Invalid(f"mutation escaped the session-map shape gate: {what}")
    print(f"session-map shape self-test passed ({len(mutations)} mutations caught)")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--artifacts", type=pathlib.Path)
    args = parser.parse_args(argv)
    try:
        if args.self_test:
            self_test()
            return 0
        texts = read_sources(REPO)
        check(texts)
        if args.artifacts is not None:
            check_artifacts(args.artifacts.resolve(), texts)
        print("session-map shape is one shape across the Rust FFI, the gate list, the JS and "
              "the .d.ts")
        return 0
    except (Invalid, OSError, ValueError) as failure:
        print(f"session-map shape gate failed: {failure}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
