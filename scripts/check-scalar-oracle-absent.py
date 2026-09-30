#!/usr/bin/env python3
"""Shipped artifacts contain no whole-plan scalar path (issue #1059).

Owner ruling 2026-09-28, decision 3 (`docs/rulings/engine-footprint-2026-09-28.md`): the
whole-plan scalar renderer -- a plan compiled at `lane::Backend::Scalar`, every track rendered one
at a time and unbanked -- stays as the test-only correctness reference for vector banking. It is
compiled only for tests and the `test-support` features. This gate reads a *built* artifact and
fails if any of the scalar path's own functions is in it:

`--wasm MODULE`
    The AudioWorklet module (`miso-engine-v1-audio-worklet.simd128.wasm`). Function names are read
    from the module's `name` custom section, parsed here, so no external tool is needed.
`--native LIBRARY`
    A release `capi` library (`libcapi.so`, `.a` or `.dylib`), read with `nm --no-demangle`.
`--self-test`
    Hermetic mutations of both readers and of the matcher; builds nothing.

What is matched
---------------
Rust's symbol manglings (v0 `_R...` and legacy `_ZN...`) both spell an identifier as its decimal
byte length followed by the identifier, so `19ScalarPairProcessor` is exactly that type and never
`LiveControlScalarPairProcessor`, and `19into_graph_artifact` is never the banked
`30into_graph_artifact_with_banks`. A roster entry matches a symbol that contains its
length-prefixed spelling where the length is not itself the tail of a longer number.

`FORBIDDEN` is the scalar path: the per-node strip owners and their pair factories in
`builtins-compiler`, the bankless lowering, and the graph's scalar pairing passes and hooks. Each
exists only under `cfg(any(test, feature = "test-support"))`, so a shipped build that contains one
has compiled the scalar path back in -- a normal dependency that enables a `test-support`
feature, or a gate that was dropped.

`REQUIRED` are the banked twins every shipped plan renders through. They are the positive
control: a stripped artifact, a module without a `name` section or a reader that silently parsed
nothing would otherwise pass by finding nothing at all.

What is not matched
-------------------
The per-effect one-lane leg (`W = 1`, `FrameLane`) is live production code at every width and is
not the whole-plan scalar path. The executor's split-pair slot and table stay in every build
because every plan's resource estimate charges their layout; with no factory compiled in they are
never filled, and they own no function of their own to find.
"""
from __future__ import annotations

import argparse
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

# (identifier, owning crate, what it is). Every entry is compiled only for tests and
# `test-support` (issue #1059).
FORBIDDEN: tuple[tuple[str, str, str], ...] = (
    ("InputProcessor", "builtins_compiler", "per-node input strip owner"),
    ("FaderProcessor", "builtins_compiler", "per-node fader strip owner"),
    ("MatrixProcessor", "builtins_compiler", "per-node matrix strip owner"),
    ("LiveControlInputProcessor", "builtins_compiler", "per-node live input strip owner"),
    ("LiveControlFaderProcessor", "builtins_compiler", "per-node live fader strip owner"),
    ("LiveControlMatrixProcessor", "builtins_compiler", "per-node live matrix strip owner"),
    ("ScalarPairProcessor", "builtins_compiler", "serialized scalar fader/matrix pair"),
    ("ScalarSplitPairProcessor", "builtins_compiler", "split scalar fader/matrix owner"),
    ("make_scalar_pair", "builtins_compiler", "scalar pair factory"),
    ("make_scalar_split_pair", "builtins_compiler", "split scalar pair factory"),
    ("strip_bindings", "builtins_compiler", "per-node strip lowering"),
    ("into_graph_artifact", "builtins_compiler", "bankless whole-plan lowering"),
    ("select_scalar_pairs", "graph", "scalar fader/matrix pairing passes"),
    ("scalar_pair_is_in_place", "graph", "scalar pairing admission"),
    ("scalar_split_interval_is_clear", "graph", "split scalar pairing admission"),
    ("scalar_pair_factory", "graph", "processor hook offering a scalar pair"),
    ("scalar_split_pair_factory", "graph", "processor hook offering a split scalar pair"),
)

# The banked strip path every shipped plan takes; each must be present (positive control).
REQUIRED: tuple[tuple[str, str, str], ...] = (
    ("into_graph_artifact_with_banks", "builtins_compiler", "banked lowering"),
    ("BuiltinBankProcessor", "builtins_compiler", "input strip bank"),
    ("FaderBankProcessor", "builtins_compiler", "fader strip bank"),
    ("MatrixBankProcessor", "builtins_compiler", "matrix strip bank"),
)


class Invalid(RuntimeError):
    pass


def pattern(identifier: str) -> re.Pattern[str]:
    """The length-prefixed spelling of `identifier` in a mangled Rust symbol."""
    return re.compile(rf"(?<![0-9]){len(identifier)}_?{re.escape(identifier)}")


def leb128(data: bytes, offset: int) -> tuple[int, int]:
    result = 0
    shift = 0
    while True:
        if offset >= len(data):
            raise Invalid("truncated LEB128 value")
        byte = data[offset]
        offset += 1
        result |= (byte & 0x7F) << shift
        if byte < 0x80:
            return result, offset
        shift += 7
        if shift > 35:
            raise Invalid("LEB128 value longer than a u32")


def wasm_function_names(data: bytes) -> list[str]:
    """Every function name in the module's `name` section (subsection 1)."""
    if data[:4] != b"\0asm" or len(data) < 8:
        raise Invalid("not a WebAssembly module")
    names: list[str] = []
    found_section = False
    offset = 8
    while offset < len(data):
        section_id = data[offset]
        size, offset = leb128(data, offset + 1)
        end = offset + size
        if end > len(data):
            raise Invalid("section runs past the end of the module")
        if section_id == 0:
            name_length, cursor = leb128(data, offset)
            section_name = data[cursor:cursor + name_length]
            cursor += name_length
            if section_name == b"name":
                found_section = True
                while cursor < end:
                    subsection = data[cursor]
                    sub_size, cursor = leb128(data, cursor + 1)
                    sub_end = cursor + sub_size
                    if subsection == 1:
                        count, cursor = leb128(data, cursor)
                        for _ in range(count):
                            _, cursor = leb128(data, cursor)
                            length, cursor = leb128(data, cursor)
                            names.append(data[cursor:cursor + length].decode("utf-8"))
                            cursor += length
                    cursor = sub_end
        offset = end
    if not found_section:
        raise Invalid("the module has no `name` section, so its functions cannot be checked")
    return names


def native_symbols(path: pathlib.Path) -> list[str]:
    tool = shutil.which("nm")
    if tool is None:
        raise Invalid("nm is required to read a native library")
    result = subprocess.run(
        [tool, "--no-demangle", str(path)], capture_output=True, text=True, check=False
    )
    if result.returncode != 0:
        raise Invalid(f"nm failed on {path}: {result.stderr.strip()}")
    symbols = []
    for line in result.stdout.splitlines():
        fields = line.split()
        if len(fields) >= 2 and not line.endswith(":"):
            symbols.append(fields[-1])
    return symbols


def verdict(label: str, symbols: list[str]) -> list[str]:
    """Failure lines for `symbols`; empty when the artifact passes."""
    failures = []
    if not symbols:
        return [f"{label}: no symbol names were read, so the scalar path cannot be ruled out"]
    for identifier, owner, what in FORBIDDEN:
        compiled = pattern(identifier)
        hits = [symbol for symbol in symbols if compiled.search(symbol)]
        if hits:
            failures.append(
                f"{label}: {len(hits)} symbol(s) of the whole-plan scalar path, {owner}::{identifier} "
                f"({what}), e.g. {hits[0]}"
            )
    for identifier, owner, what in REQUIRED:
        compiled = pattern(identifier)
        if not any(compiled.search(symbol) for symbol in symbols):
            failures.append(
                f"{label}: positive control {owner}::{identifier} ({what}) is missing; the "
                "artifact is stripped or this reader found nothing, so its silence proves nothing"
            )
    return failures


def check(label: str, symbols: list[str]) -> int:
    failures = verdict(label, symbols)
    for line in failures:
        print(f"FAIL {line}", file=sys.stderr)
    if failures:
        return 1
    print(
        f"scalar oracle absent from {label}: {len(symbols)} symbols, none of {len(FORBIDDEN)} "
        f"scalar-path identifiers, all {len(REQUIRED)} banked controls present"
    )
    return 0


# ---- self-test ----------------------------------------------------------------------------------

# Real symbols: from the shipped module and `libcapi.so` at c69736c1 (before #1059), where the
# scalar path was still compiled in, and, for the two pairing functions a release build inlines,
# from a debug `graph` rlib built with `test-support`. Issue #1095 renamed the three per-node live
# strip owners from `Console*Processor` to `LiveControl*Processor`; their five symbols below are
# those captures with the type's length-prefixed name re-spelled in place and nothing else moved.
REAL_FORBIDDEN = {
    "InputProcessor": "_RNvXsd_Cs57Yi9iMBrcW_17builtins_compilerNtB5_14InputProcessorNtCs9KyrgcuZWw8_5graph21GraphRuntimeProcessor7process",
    "FaderProcessor": "_RNvXse_Cs57Yi9iMBrcW_17builtins_compilerNtB5_14FaderProcessorNtCs9KyrgcuZWw8_5graph21GraphRuntimeProcessor7process",
    "MatrixProcessor": "_RNvXsf_Cs57Yi9iMBrcW_17builtins_compilerNtB5_15MatrixProcessorNtCs9KyrgcuZWw8_5graph21GraphRuntimeProcessor7process",
    "LiveControlInputProcessor": "_RNvXsg_Cs57Yi9iMBrcW_17builtins_compilerNtB5_25LiveControlInputProcessorNtCs9KyrgcuZWw8_5graph21GraphRuntimeProcessor7process",
    "LiveControlFaderProcessor": "_RNvMsk_Cs57Yi9iMBrcW_17builtins_compilerNtB5_25LiveControlFaderProcessor14drain_controls",
    "LiveControlMatrixProcessor": "_RNvMsi_Cs57Yi9iMBrcW_17builtins_compilerNtB5_26LiveControlMatrixProcessor14drain_controls",
    "ScalarPairProcessor": "_RNvXsl_Cs57Yi9iMBrcW_17builtins_compilerNtB5_19ScalarPairProcessorNtCs9KyrgcuZWw8_5graph21GraphRuntimeProcessor7process",
    "ScalarSplitPairProcessor": "_RNvXsm_CseApoyVW5ce9_17builtins_compilerNtB5_24ScalarSplitPairProcessorNtCs2vOlDtAeped_5graph30GraphRuntimeSplitPairProcessor11begin_fader",
    "make_scalar_pair": "_RNvCs57Yi9iMBrcW_17builtins_compiler16make_scalar_pair",
    "make_scalar_split_pair": "_RNvCs57Yi9iMBrcW_17builtins_compiler22make_scalar_split_pair",
    "strip_bindings": "_RNvMsa_CseApoyVW5ce9_17builtins_compilerNtB5_23PreparedBuiltinsSession14strip_bindings",
    "into_graph_artifact": "_RINvMsa_Cs57Yi9iMBrcW_17builtins_compilerNtB6_23PreparedBuiltinsSession19into_graph_artifactNtCseoBS7rBbMCz_14graph_compiler18GraphCompileReportEB1u_",
    "select_scalar_pairs": "_RNvNtCsd3M4iPsRW3L_5graph7runtime19select_scalar_pairs",
    "scalar_pair_is_in_place": "_RNvNtCs9KyrgcuZWw8_5graph7runtime23scalar_pair_is_in_place",
    "scalar_split_interval_is_clear": "_RNvNtCsd3M4iPsRW3L_5graph7runtime30scalar_split_interval_is_clear",
    "scalar_pair_factory": "_RNvXsj_Cs57Yi9iMBrcW_17builtins_compilerNtB5_25LiveControlFaderProcessorNtCs9KyrgcuZWw8_5graph21GraphRuntimeProcessor19scalar_pair_factory",
    "scalar_split_pair_factory": "_RNvXsj_Cs57Yi9iMBrcW_17builtins_compilerNtB5_25LiveControlFaderProcessorNtCs9KyrgcuZWw8_5graph21GraphRuntimeProcessor25scalar_split_pair_factory",
}
REAL_CONTROLS = [
    "_RINvMsa_Cs57Yi9iMBrcW_17builtins_compilerNtB6_23PreparedBuiltinsSession30into_graph_artifact_with_banksNtCseoBS7rBbMCz_14graph_compiler18GraphCompileReportEB1F_",
    "_RNvXs0_Cs57Yi9iMBrcW_17builtins_compilerNtB5_20BuiltinBankProcessorNtCs9KyrgcuZWw8_5graph33GraphPreparedBuiltinBankProcessor7process",
    "_RNvXs1_Cs57Yi9iMBrcW_17builtins_compilerNtB5_18FaderBankProcessorNtCs9KyrgcuZWw8_5graph33GraphPreparedBuiltinBankProcessor7process",
    "_RNvXs2_Cs57Yi9iMBrcW_17builtins_compilerNtB5_19MatrixBankProcessorNtCs9KyrgcuZWw8_5graph33GraphPreparedBuiltinBankProcessor7process",
]
# Near misses that must not match: the banked twins, a longer identifier ending in a roster name,
# a longer number ending in a roster length, and legacy mangling of an unrelated `InputProcessor`
# suffix.
NEAR_MISSES = [
    "_RNvXs3_Cs57Yi9iMBrcW_17builtins_compilerNtB5_24FaderMatrixBankProcessorNtCs9KyrgcuZWw8_5graph33GraphPreparedBuiltinBankProcessor7process",
    "_RNvCs0_4demo30LiveControlScalarPairProcessor",
    "_RNvCs0_4demo114InputProcessor",
    "_ZN4demo20HostInputProcessor4new17h0123456789abcdefE",
    "_RNvCs0_4demo23scalar_pair_factory_log",
]


def wasm_module(names: list[str], name_section: bool = True) -> bytes:
    """A minimal module: a type, `len(names)` empty functions and (optionally) a `name` section."""

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

    count = len(names)
    body = b"\x02\x00\x0b"  # size 2: no locals, `end`
    module = b"\0asm\x01\x00\x00\x00"
    module += section(1, b"\x01\x60\x00\x00")
    module += section(3, uleb(count) + b"\x00" * count)
    module += section(10, uleb(count) + body * count)
    if name_section:
        entries = b"".join(
            uleb(index) + uleb(len(name.encode())) + name.encode() for index, name in enumerate(names)
        )
        function_names = uleb(count) + entries
        payload = uleb(4) + b"name" + b"\x01" + uleb(len(function_names)) + function_names
        module += section(0, payload)
    return module


def self_test() -> int:
    failures = 0

    def expect(name: str, condition: bool) -> None:
        nonlocal failures
        if not condition:
            failures += 1
            print(f"self-test FAILED: {name}", file=sys.stderr)

    clean = REAL_CONTROLS + NEAR_MISSES
    expect("(a) the banked controls and the near misses pass", verdict("clean", clean) == [])
    expect("(b) every roster entry has a real example", set(REAL_FORBIDDEN) == {i for i, _, _ in FORBIDDEN})
    for identifier, symbol in REAL_FORBIDDEN.items():
        lines = verdict("mutant", clean + [symbol])
        expect(
            f"(c) a real {identifier} symbol fails, and only its own entry",
            len(lines) >= 1 and all("scalar path" in line for line in lines)
            and any(f"::{identifier} (" in line for line in lines),
        )
    for identifier, _, _ in REQUIRED:
        stripped = [symbol for symbol in clean if not pattern(identifier).search(symbol)]
        expect(f"(d) a missing control {identifier} fails", verdict("stripped", stripped) != [])
    expect("(e) no symbols at all fails", verdict("empty", []) != [])

    # The wasm reader, on real bytes.
    expect("(f) the wasm reader reads every name", wasm_function_names(wasm_module(clean)) == clean)
    try:
        wasm_function_names(wasm_module(clean, name_section=False))
        expect("(g) a module without a name section is refused", False)
    except Invalid:
        pass
    try:
        wasm_function_names(b"\x7fELF" + b"\0" * 16)
        expect("(h) a non-wasm file is refused", False)
    except Invalid:
        pass
    mutant = wasm_module(clean + [REAL_FORBIDDEN["ScalarSplitPairProcessor"]])
    expect("(i) a scalar owner in a module fails end to end", verdict("wasm", wasm_function_names(mutant)) != [])

    # The native reader, when `nm` and a C compiler are present (CI's audit-native has both).
    compiler = shutil.which("cc")
    if compiler and shutil.which("nm"):
        with tempfile.TemporaryDirectory() as scratch:
            source = pathlib.Path(scratch) / "probe.c"
            obj = pathlib.Path(scratch) / "probe.o"
            symbols = clean + [REAL_FORBIDDEN["strip_bindings"]]
            source.write_text(
                "".join(f"void {symbol}(void) {{}}\n" for symbol in symbols), encoding="utf-8"
            )
            subprocess.run([compiler, "-c", str(source), "-o", str(obj)], check=True)
            read = native_symbols(obj)
            expect("(j) the native reader reads every symbol", set(symbols) <= set(read))
            expect("(k) a scalar owner in an object fails end to end", verdict("native", read) != [])
    else:
        print("self-test note: no cc/nm, native reader cases (j)-(k) skipped", file=sys.stderr)

    if failures:
        return 1
    print("scalar-oracle-absent self-test: all mutations caught")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--wasm", type=pathlib.Path, help="the AudioWorklet module")
    mode.add_argument("--native", type=pathlib.Path, help="a release capi library")
    mode.add_argument("--self-test", action="store_true")
    arguments = parser.parse_args()
    try:
        if arguments.self_test:
            return self_test()
        if arguments.wasm is not None:
            return check(str(arguments.wasm), wasm_function_names(arguments.wasm.read_bytes()))
        return check(str(arguments.native), native_symbols(arguments.native))
    except (Invalid, OSError) as error:
        print(f"FAIL {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
