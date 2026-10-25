#!/usr/bin/env python3
"""Tie the shipped browser module to the compiler it names (issue #2045).

`scripts/build-web-audioworklet.sh` builds the AudioWorklet module on one pinned dated nightly
(`rust-toolchain.toml`'s `[browser-artifact] channel`, issue #1334). rustc writes its own version
into the module's `producers` custom section (the tool-conventions `processed-by` field), and the
shipped module keeps that section byte for byte (`strip-wasm-names.py`). This check reads it and
compares it with what that channel's compiler says it is, so a build that ran on another rustc --
a floating toolchain, a `RUSTUP_TOOLCHAIN` the build did not set, another install at the same
name -- fails here, and not only when its digest happens to move.

Modes
-----
`MODULE --toolchain CHANNEL`
    Pass only when MODULE carries exactly one `producers` section whose `processed-by` field
    names `rustc` exactly once, with exactly the version `rustup run CHANNEL rustc --version`
    prints after its `rustc ` prefix (release, commit hash and date: the whole string, never the
    channel name alone).
`--self-test`
    Hermetic: synthesised modules, red on another rustc version and on a missing or malformed
    `producers` section; builds nothing and runs no rustc.

The section walker is `strip-wasm-names.py`'s, loaded by path: one WebAssembly parser in the
repository, with its refusals (bad magic, malformed sizes, unknown or misordered sections). The
tool depends on nothing outside the Python standard library.
"""
from __future__ import annotations

import argparse
import importlib.util
import pathlib
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
_spec = importlib.util.spec_from_file_location("strip_wasm_names", HERE / "strip-wasm-names.py")
assert _spec is not None and _spec.loader is not None
wasm = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(wasm)
Invalid = wasm.Invalid

PRODUCERS = "producers"
PROCESSED_BY = "processed-by"
RUSTC = "rustc"
RUSTC_PREFIX = "rustc "


def name_at(data: bytes, offset: int, end: int) -> tuple[str, int]:
    length, cursor = wasm.leb128_u32(data, offset, end)
    if cursor + length > end:
        raise Invalid("a `producers` string runs past its section")
    try:
        return data[cursor:cursor + length].decode("utf-8"), cursor + length
    except UnicodeDecodeError:
        raise Invalid("a `producers` string is not UTF-8") from None


def producers(module: bytes) -> dict[str, list[tuple[str, str]]]:
    """The module's one `producers` section, as field name -> [(tool, version)]."""
    found = [section for section in wasm.parse(module)
             if section.id == 0 and section.name == PRODUCERS]
    if len(found) != 1:
        raise Invalid(f"the module must carry exactly one `producers` section, found {len(found)}: "
                      "nothing would tie its bytes to a compiler")
    raw = found[0].raw
    _, payload = wasm.leb128_u32(raw, 1, len(raw))
    _, offset = name_at(raw, payload, len(raw))  # the section's own name
    end = len(raw)
    count, offset = wasm.leb128_u32(raw, offset, end)
    fields: dict[str, list[tuple[str, str]]] = {}
    for _ in range(count):
        field, offset = name_at(raw, offset, end)
        if field in fields:
            raise Invalid(f"the `producers` field {field!r} appears twice")
        values, offset = wasm.leb128_u32(raw, offset, end)
        entries = []
        for _ in range(values):
            tool, offset = name_at(raw, offset, end)
            version, offset = name_at(raw, offset, end)
            entries.append((tool, version))
        fields[field] = entries
    if offset != end:
        raise Invalid(f"the `producers` section has {end - offset} trailing bytes")
    return fields


def rustc_version(module: bytes) -> str:
    """The version the module's `processed-by` field gives `rustc`."""
    versions = [version for tool, version in producers(module).get(PROCESSED_BY, [])
                if tool == RUSTC]
    if len(versions) != 1:
        raise Invalid(f"the `producers` section's `{PROCESSED_BY}` field must name `{RUSTC}` "
                      f"exactly once, found {len(versions)}")
    return versions[0]


def check(module: bytes, version_line: str) -> str:
    """A one-line verdict when `module` names the compiler `version_line` describes."""
    line = version_line.strip()
    if not line.startswith(RUSTC_PREFIX) or not line[len(RUSTC_PREFIX):]:
        raise Invalid(f"not a `rustc --version` line: {version_line!r}")
    expected = line[len(RUSTC_PREFIX):]
    named = rustc_version(module)
    if named != expected:
        raise Invalid(f"the module was processed by rustc {named!r}, the named toolchain is rustc "
                      f"{expected!r}")
    return f"module processed by rustc {named}, the named toolchain's compiler"


def toolchain_version(channel: str) -> str:
    try:
        completed = subprocess.run(["rustup", "run", channel, "rustc", "--version"],
                                   capture_output=True, text=True, check=False)
    except OSError as error:
        raise Invalid(f"cannot run rustup: {error.strerror}") from None
    if completed.returncode != 0:
        raise Invalid(f"`rustup run {channel} rustc --version` failed: {completed.stderr.strip()}")
    return completed.stdout


# ---- self-test ----------------------------------------------------------------------------------

def string(text: str) -> bytes:
    data = text.encode()
    return wasm.uleb(len(data)) + data


def producers_body(fields: list[tuple[str, list[tuple[str, str]]]]) -> bytes:
    body = wasm.uleb(len(fields))
    for field, entries in fields:
        body += string(field) + wasm.uleb(len(entries))
        body += b"".join(string(tool) + string(version) for tool, version in entries)
    return body


VERSION = "1.100.0-nightly (f7d782a3b 2026-08-19)"
LINE = f"rustc {VERSION}\n"


def module_with(body: bytes | None) -> bytes:
    """The strip-wasm-names fixture, shipped (no `name` section), with `body` as its producers."""
    shipped = wasm.fixture(names=False)
    if body is None:
        return shipped.replace(wasm.PRODUCERS, b"")
    return shipped.replace(wasm.PRODUCERS, wasm.custom(PRODUCERS, body))


def rustc_producers(*entries: tuple[str, str]) -> bytes:
    return producers_body([("language", [("Rust", "")]), (PROCESSED_BY, list(entries))])


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

    good = module_with(rustc_producers((RUSTC, VERSION)))
    # (a) The module rustc wrote names the compiler: green.
    expect("(a1) the named compiler's module passes", check(good, LINE).startswith("module"))
    expect("(a2) the version is read whole", rustc_version(good) == VERSION)
    expect("(a3) other processed-by tools are ignored",
           rustc_version(module_with(rustc_producers(("clang", "19"), (RUSTC, VERSION))))
           == VERSION)

    # (b) Another rustc: red, however small the difference.
    for label, other in (("release", "1.99.1 (f7d782a3b 2026-08-19)"),
                         ("commit hash", "1.100.0-nightly (f7d782a3c 2026-08-19)"),
                         ("date", "1.100.0-nightly (f7d782a3b 2026-08-20)")):
        refused(f"(b) another rustc {label} fails",
                lambda other=other: check(module_with(rustc_producers((RUSTC, other))), LINE))
    refused("(b) a version line that is not rustc's fails", lambda: check(good, VERSION))
    refused("(b) an empty version line fails", lambda: check(good, ""))

    # (c) No producers section, or one that names no compiler: red.
    refused("(c1) a module without a producers section fails", lambda: check(module_with(None), LINE))
    refused("(c2) two producers sections fail",
            lambda: check(good + wasm.custom(PRODUCERS, rustc_producers((RUSTC, VERSION))), LINE))
    refused("(c3) no processed-by field fails",
            lambda: check(module_with(producers_body([("language", [("Rust", "")])])), LINE))
    refused("(c4) processed-by without rustc fails",
            lambda: check(module_with(rustc_producers(("clang", VERSION))), LINE))
    refused("(c5) rustc named twice fails",
            lambda: check(module_with(rustc_producers((RUSTC, VERSION), (RUSTC, VERSION))), LINE))
    refused("(c6) a repeated field fails",
            lambda: check(module_with(producers_body([(PROCESSED_BY, [(RUSTC, VERSION)])] * 2)),
                          LINE))

    # (d) A malformed producers section: red, never read as a partial answer.
    body = rustc_producers((RUSTC, VERSION))
    refused("(d1) trailing bytes fail", lambda: check(module_with(body + b"\x00"), LINE))
    refused("(d2) a truncated section fails", lambda: check(module_with(body[:-1]), LINE))
    refused("(d3) a string that is not UTF-8 fails",
            lambda: check(module_with(body.replace(b"rustc", b"rust\xff")), LINE))
    refused("(d4) a malformed module fails", lambda: check(good[:-1], LINE))

    if failures:
        return 1
    print("check-wasm-producers self-test: every mutation caught")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    parser.add_argument("module", nargs="?", type=pathlib.Path)
    parser.add_argument("--toolchain", help="the rustup channel the module must name")
    parser.add_argument("--self-test", action="store_true")
    arguments = parser.parse_args()
    if arguments.self_test:
        if arguments.module or arguments.toolchain:
            parser.error("--self-test takes no other argument")
        return self_test()
    if not arguments.module or not arguments.toolchain:
        parser.error("give MODULE --toolchain CHANNEL, or --self-test")
    try:
        try:
            module = arguments.module.read_bytes()
        except OSError as error:
            raise Invalid(f"cannot read {arguments.module}: {error.strerror}") from None
        print(check(module, toolchain_version(arguments.toolchain)))
        return 0
    except Invalid as error:
        print(f"FAIL {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
