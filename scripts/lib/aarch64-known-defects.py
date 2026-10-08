#!/usr/bin/env python3
"""The known AArch64 defects of issue #1017, as expected failures by name, and their judges.

Each row names its owning issue. Two scripts consume this file, so the rows live in one place:

* `scripts/run-aarch64-tests.sh`, the `aarch64-debug` and `aarch64-release` legs, reads the test
  rows. A row names one test and the reason it fails: a string its panic message must contain,
  normally the AArch64 digest or the assertion text that the defect produces. The leg skips each
  row's test by exact name in its main run and then runs it alone. The run must fail as that one
  test and for that reason. A pass, a different panic, or the same test failing for another reason
  fails the leg. So a fixed defect cannot leave a stale row, and a new regression cannot hide
  behind an old one.
* `scripts/check-cross-targets.sh`, the `cross-target` job, reads the iOS memset ceilings. The row
  for each product crate carries its count of `bl _memset_pattern16` in the library an iPhone app
  links: `capi`'s `aarch64-apple-ios` release staticlib after fat LTO (#1018, #1472), with each
  call charged to a crate by the assembly's DWARF inline records (`count-memset`). A crate at zero
  with a row fails ("fixed: delete the row"). A crate above its ceiling fails ("rose"). A crate
  with calls and no row fails, and so do calls charged to a crate outside the product closure and
  a row for a crate that is not a product crate. A crate below its ceiling passes and is reported,
  so a partial fix shows as progress and the defect reads fixed only when every row is gone.

Subcommands:

    rows <debug|release>                       issue|package|selector|name, one per line
    judge-skips <debug|release> <list-output>  each row names exactly one test in the leg
    judge-test <debug|release> <name> <log> <exit-status>
    count-memset <assembly> <product-crates-file> <counts-file>
    judge-memset <counts-file> <product-crates-file>
    --self-test                                hermetic mutations of every judge
"""
from __future__ import annotations

import ast
import bisect
import pathlib
import re
import sys

# issue, package, selector (`lib` or `test:<integration test>`), exact test name, reason.
TEST_ROWS: dict[str, list[tuple[str, str, str, str, str]]] = {
    # #1065 (AArch64 NaN encodings) left this leg: class-A identity reads every NaN as one value
    # (owner decision 10), so its two pins fold NaN words through `dsp_reference::class_a` and
    # pass on both CPUs.
    "debug": [],
    "release": [
        # LANE-3 (#1019): in release the D8 `select(a > b, a, b)` folds into `fmaxnm`/`fminnm`
        # inside `exp2_lane`, scalar and vector alike (`log2_lane` passes since #1451's
        # splat change). Those instructions answer
        # differently on NaN and signed-zero inputs. The same tests pass in the debug leg.
        ("1019", "math", "test:m2_lane_identity", "m2_exp2_lane_identity",
         "exp2_lane: scalar digest 69c5e8f5e4639e1438b6f38bce75a7267904f84501a3d89c2704146272c4942a"
         " does not match the pin"),
    ],
}

# Product crate -> (owning issue, ceiling): `bl _memset_pattern16` calls charged to the crate in
# `capi`'s post-LTO `aarch64-apple-ios` release staticlib assembly, as
# `scripts/check-cross-targets.sh` emits it and `count_memset` charges them. LLVM's loop-idiom
# pass rewrites a loop that stores one constant `f32` pattern into
# `llvm.experimental.memset.pattern`, which only Darwin lowers to this libc call.
#
# History, on the per-crate pre-link rlib assembly that the ratchet read until #1472: #1017
# attempt 2 counted 3,494 calls on Rust 1.97.1; #1112, #1328, its amendment A9 and its follow-up
# lowered that to 2,122 by removing eight-lane AArch64 code and by carrying constants as words.
# #1451 found the cause of nearly all of them: `wide`'s `splat` is `transmute([elem; N])`, rustc
# lowers that array repeat to a store loop, and every `Lane::splat` in a kernel became such a loop;
# `lane` now builds its splats as array literals (`crates/lane/src/wide_impl.rs`): 2,122 -> 16.
# #1456 removed the `true-peak-limiter`'s 6 (`clear_runtime` and `ChannelState::new`): 16 -> 10.
#
# #1472 re-based the rows on the shipped library, where fat LTO keeps 5 calls, all in preparation
# code reached from `builtins-compiler`'s `into_graph_artifact_with_banks` and from `builtins`'
# constructors: `builtins` 4 (`BuiltinChain::new`, `FaderMuteRampBuiltins::new`,
# `BuiltinInputBank::new`, `BuiltinFaderBank::new`; the last two are inlined into
# `builtins-compiler`) and `lane` 1 (`kernels::builtins::lanes_below`'s flag fill, inlined into
# `builtins-compiler`). The pre-link rows of `host-core` (4, the spectrum arrays) and `soft-clip`
# (1, the test corpus) were code that LTO removes: no iPhone app links it.
IOS_MEMSET_CEILINGS: dict[str, tuple[str, int]] = {
    "builtins": ("1018", 4),
    "lane": ("1018", 1),
}


class Refused(RuntimeError):
    pass


Row = tuple[str, str, str, str, str]


def row_for(rows: list[Row], name: str) -> Row:
    rows = [row for row in rows if row[3] == name]
    if len(rows) != 1:
        raise Refused(f"{name}: not exactly one row")
    return rows[0]


def judge_skips(rows: list[Row], listing: str) -> None:
    """`listing` is the leg's `cargo test ... -- --list` output. `--exact --skip <name>` applies
    to every test binary in the run, so each row's name must name exactly one test there."""
    for issue, package, _, name, _ in rows:
        count = sum(1 for line in listing.splitlines() if line == f"{name}: test")
        if count != 1:
            raise Refused(f"expected failure {package} {name} (#{issue}) names {count} tests in "
                          "the leg; a row must name exactly one")


def panic_block(log: str, name: str) -> list[str]:
    """The lines of `name`'s panic message: from its `thread '<name>' ... panicked at` line up to
    the next blank or `note:` line."""
    lines = log.splitlines()
    starts = [index for index, line in enumerate(lines)
              if line.startswith(f"thread '{name}'") and " panicked at " in line]
    if len(starts) != 1:
        raise Refused(f"{name}: expected one panic from the test, found {len(starts)}")
    block = []
    for line in lines[starts[0]:]:
        if block and (not line.strip() or line.startswith("note:")):
            break
        block.append(line)
    return block


def judge_test(rows: list[Row], name: str, log: str, status: int) -> str:
    issue, package, _, _, reason = row_for(rows, name)
    label = f"expected failure {package} {name} (#{issue})"
    if status == 0:
        raise Refused(f"{label} now passes: delete its row from scripts/lib/aarch64-known-defects.py")
    if (f"test {name} ... FAILED" not in log.splitlines()
            or len(re.findall(r"^test result: FAILED\. 0 passed; 1 failed;", log, re.MULTILINE)) != 1):
        raise Refused(f"{label} did not fail as that one test")
    block = panic_block(log, name)
    if not any(reason in line for line in block):
        raise Refused(f"{label} failed for another reason: its panic does not contain "
                      f"{reason!r}:\n" + "\n".join(block))
    return f"expected failure (#{issue}): {package} {name}"


def judge_memset(counts: dict[str, int], products: list[str],
                 ceilings: dict[str, tuple[str, int]]) -> list[str]:
    report: list[str] = []
    errors: list[str] = []
    for crate in sorted(set(ceilings) - set(products)):
        errors.append(f"{crate}: has a row but is not a product crate; delete its row")
    for crate in sorted(set(products) - set(counts)):
        errors.append(f"{crate}: no count was measured")
    for crate in sorted(counts):
        count = counts[crate]
        row = ceilings.get(crate)
        if row is None:
            if count and crate not in products:
                errors.append(f"{crate}: {count} memset_pattern16 calls charged to a crate "
                              "outside the product closure: no product crate's code holds them")
            elif count:
                errors.append(f"{crate}: {count} memset_pattern16 calls and no row: a new libc "
                              "call in the iOS build (#1018)")
            continue
        issue, ceiling = row
        if count == 0:
            errors.append(f"{crate}: expected failure ios-asm-memset-pattern16 (#{issue}) now "
                          "passes: delete its row from scripts/lib/aarch64-known-defects.py")
        elif count > ceiling:
            errors.append(f"{crate}: memset_pattern16 calls rose from {ceiling} to {count} "
                          f"(#{issue})")
        else:
            note = "" if count == ceiling else f", down from {ceiling}: lower its row"
            report.append(f"ios-asm-memset-pattern16: expected failure (#{issue}): {crate} "
                          f"{count} calls{note}")
    if errors:
        raise Refused("ios-asm-memset-pattern16:\n  " + "\n  ".join(errors))
    return report


def parse_counts(text: str) -> dict[str, int]:
    counts: dict[str, int] = {}
    for line in text.splitlines():
        crate, count = line.split()
        if crate in counts:
            raise Refused(f"{crate}: counted twice")
        counts[crate] = int(count)
    return counts


# --- the post-LTO iOS `capi` assembly (#1472) ----------------------------------------------------
# The counted artifact is `capi`'s `aarch64-apple-ios` release staticlib assembly: after fat LTO it
# is one module, every function an iPhone app links, after cross-crate inlining. A call sits in the
# function it was inlined into, so its function label can name another crate than the code that
# wrote the fill. The release profile's line tables (`debug = 1`) keep DWARF inline records in the
# assembly's `__debug_info` section: every inlined call has a `DW_TAG_inlined_subroutine` with the
# code ranges it covers and an abstract origin, whose enclosing `DW_TAG_namespace` chain starts with
# its crate. The count reads those records: a call is attributed to the innermost function holding
# it that belongs to a product crate (an inlined `core`, `alloc` or `std` frame such as
# `<[T]>::fill` is skipped, so the fill is charged to the crate that called it). A call with no
# product frame is charged to its function label's crate, which the judge then refuses as not a
# product crate. Every reference to `_memset_pattern16` in the file is read, and any that is not a
# `bl` line in `__TEXT,__text` (a tail call `b _memset_pattern16`, an address load) is refused, so
# a call cannot leave the count by changing its form (#1472 follow-up).

MEMSET_CALL = "\tbl\t_memset_pattern16"
MEMSET_REFERENCE = re.compile(r"(?<![\w$.])_memset_pattern16(?![\w$.])")
DWARF_SECTIONS = ("__debug_abbrev", "__debug_info", "__debug_str", "__debug_ranges")
DIRECTIVE_SIZES = {".byte": 1, ".short": 2, ".long": 4, ".quad": 8}
TAG_COMPILE_UNIT, TAG_INLINED = 0x11, 0x1D
TAG_SUBPROGRAM, TAG_NAMESPACE = 0x2E, 0x39
AT_NAME, AT_LOW_PC, AT_HIGH_PC, AT_ABSTRACT_ORIGIN = 0x03, 0x11, 0x12, 0x31
AT_SPECIFICATION, AT_RANGES, AT_LINKAGE_NAME = 0x47, 0x55, 0x6E
KEPT_ATTRIBUTES = {AT_NAME, AT_LOW_PC, AT_HIGH_PC, AT_ABSTRACT_ORIGIN, AT_SPECIFICATION, AT_RANGES,
                   AT_LINKAGE_NAME}
FIXED_FORMS = {0x01: 8, 0x05: 2, 0x06: 4, 0x07: 8, 0x0B: 1, 0x0C: 1, 0x0E: 4, 0x10: 4, 0x11: 1,
               0x12: 2, 0x13: 4, 0x14: 8, 0x17: 4, 0x20: 8}
CU_RELATIVE_REFERENCES = {0x11, 0x12, 0x13, 0x14, 0x15}


class DwarfSection:
    """One DWARF section of the assembly as bytes. A `.long` or `.quad` whose operand is a symbol
    keeps its size in the bytes and is recorded by offset, so a reader of that size gets the
    symbol back."""

    def __init__(self) -> None:
        self.data = bytearray()
        self.symbols: dict[int, tuple[int, str]] = {}
        self.labels: dict[str, int] = {}

    def take(self, offset: int, size: int) -> int | str:
        if offset in self.symbols:
            symbol_size, symbol = self.symbols[offset]
            if symbol_size != size:
                raise Refused(f"DWARF: {symbol} is {symbol_size} bytes, read as {size}")
            return symbol
        if offset + size > len(self.data):
            raise Refused("DWARF: read past the end of a section")
        return int.from_bytes(self.data[offset:offset + size], "little")

    def leb(self, offset: int, signed: bool = False) -> tuple[int, int]:
        value = shift = 0
        while True:
            if offset >= len(self.data):
                raise Refused("DWARF: LEB128 runs past the end of a section")
            byte = self.data[offset]
            offset += 1
            value |= (byte & 0x7F) << shift
            shift += 7
            if not byte & 0x80:
                if signed and byte & 0x40:
                    value -= 1 << shift
                return value, offset


class Assembly:
    """What the count needs from one assembly file: the code labels and the memset calls in
    `__TEXT,__text` by line, the function labels, the `Lset` assignments and the DWARF sections."""

    def __init__(self, lines) -> None:
        self.sections = {name: DwarfSection() for name in DWARF_SECTIONS}
        self.assignments: dict[str, tuple[str, str | None]] = {}
        self.code_labels: dict[str, int] = {}
        self.functions: list[tuple[int, str]] = []
        self.calls: list[int] = []
        current: DwarfSection | None = None
        in_text = False
        for number, line in enumerate(lines, 1):
            line = line.rstrip("\n")
            if MEMSET_REFERENCE.search(line) and not (in_text and line == MEMSET_CALL):
                raise Refused(f"line {number}: a reference to _memset_pattern16 that is not a "
                              f"`bl` call in __TEXT,__text: {line.strip()!r}")
            if line.startswith("\t.section\t"):
                segment, _, rest = line.split("\t")[2].partition(",")
                name = rest.split(",")[0]
                in_text = (segment, name) == ("__TEXT", "__text")
                current = self.sections.get(name) if segment == "__DWARF" else None
                if segment == "__DWARF" and current is None and name.startswith("__debug_") and \
                        name not in ("__debug_line", "__debug_aranges"):
                    raise Refused(f"line {number}: unexpected DWARF section {name}")
                continue
            if in_text:
                if line == MEMSET_CALL:
                    self.calls.append(number)
                elif line.endswith(":") and line[:1] not in ("\t", " ", ""):
                    self.code_labels[line[:-1]] = number
                    if not line.startswith("L"):
                        self.functions.append((number, line[:-1]))
                continue
            if " = " in line and line[:1] not in ("\t", " "):
                name, _, expression = line.partition(" = ")
                minuend, _, subtrahend = expression.partition("-")
                self.assignments[name] = (minuend, subtrahend or None)
                continue
            if current is None or not line:
                continue
            if line.endswith(":") and line[:1] not in ("\t", " "):
                current.labels[line[:-1]] = len(current.data)
                continue
            directive, _, operand = line.strip().partition("\t")
            if directive in DIRECTIVE_SIZES:
                size = DIRECTIVE_SIZES[directive]
                if re.fullmatch(r"-?[0-9]+", operand):
                    current.data += (int(operand) % (1 << (8 * size))).to_bytes(size, "little")
                elif size in (4, 8):
                    current.symbols[len(current.data)] = (size, operand)
                    current.data += bytes(size)
                else:
                    # Only an offset (`.long`) or an address (`.quad`) is a symbol. The LEB and
                    # children reads take raw bytes, so any other operand would be misread.
                    raise Refused(f"line {number}: {directive} {operand!r} is not a decimal number")
            elif directive in (".ascii", ".asciz"):
                current.data += ast.literal_eval("b" + operand)
                if directive == ".asciz":
                    current.data += b"\0"
            else:
                raise Refused(f"line {number}: unexpected {directive!r} in a DWARF section")

    def difference(self, symbol: int | str, section: DwarfSection) -> int:
        """A `.long LsetN` whose assignment is the difference of two labels of `section`."""
        if isinstance(symbol, int):
            return symbol
        minuend, subtrahend = self.assignments.get(symbol, (symbol, None))
        if minuend not in section.labels or (subtrahend and subtrahend not in section.labels):
            raise Refused(f"DWARF: {symbol} is not an offset in its section")
        return section.labels[minuend] - (section.labels[subtrahend] if subtrahend else 0)

    def line_of(self, label: str) -> int:
        if label not in self.code_labels:
            raise Refused(f"DWARF: {label} is not a code label")
        return self.code_labels[label]


def abbreviation_tables(section: DwarfSection) -> dict[int, dict[int, tuple]]:
    tables: dict[int, dict[int, tuple]] = {}
    offset = 0
    while offset < len(section.data):
        start, table = offset, {}
        while True:
            code, offset = section.leb(offset)
            if code == 0:
                break
            tag, offset = section.leb(offset)
            children = section.data[offset]
            offset += 1
            attributes = []
            while True:
                attribute, offset = section.leb(offset)
                form, offset = section.leb(offset)
                implicit = None
                if form == 0x21:
                    implicit, offset = section.leb(offset, signed=True)
                if attribute == 0 and form == 0:
                    break
                attributes.append((attribute, form, implicit))
            table[code] = (tag, bool(children), attributes)
        tables[start] = table
    return tables


def read_form(info: DwarfSection, offset: int, form: int, implicit, unit: int):
    """The attribute value at `offset` and the offset after it. References come back as
    absolute `__debug_info` offsets."""
    if form == 0x16:
        form, offset = info.leb(offset)
    if form in FIXED_FORMS:
        size = FIXED_FORMS[form]
        value = info.take(offset, size)
        offset += size
    elif form in (0x0F, 0x15):
        value, offset = info.leb(offset)
    elif form == 0x0D:
        value, offset = info.leb(offset, signed=True)
    elif form == 0x08:
        end = info.data.index(0, offset)
        value, offset = info.data[offset:end].decode("utf-8", "replace"), end + 1
    elif form in (0x09, 0x18):
        length, offset = info.leb(offset)
        value, offset = None, offset + length
    elif form in (0x0A, 0x03, 0x04):
        size = {0x0A: 1, 0x03: 2, 0x04: 4}[form]
        length = info.take(offset, size)
        value, offset = None, offset + size + int(length)
    elif form == 0x19:
        value = True
    elif form == 0x21:
        value = implicit
    else:
        raise Refused(f"DWARF: unsupported attribute form 0x{form:02x}")
    if form in CU_RELATIVE_REFERENCES:
        if not isinstance(value, int):
            raise Refused("DWARF: a symbolic DIE reference")
        value += unit
    return value, offset


def debug_string(assembly: Assembly, value) -> str | None:
    if value is None or isinstance(value, str):
        return value
    data = assembly.sections["__debug_str"].data
    if not 0 <= value < len(data):
        raise Refused(f"DWARF: string offset {value} is outside __debug_str")
    return data[value:data.index(0, value)].decode("utf-8", "replace")


def code_intervals(assembly: Assembly, attributes: dict, base: str | None) -> list[tuple[int, int]]:
    """The `[first line, end line)` intervals of the code a DIE covers."""
    ranges = assembly.sections["__debug_ranges"]
    if AT_RANGES in attributes:
        offset = assembly.difference(attributes[AT_RANGES], ranges)
        intervals = []
        while True:
            start, end = ranges.take(offset, 8), ranges.take(offset + 8, 8)
            offset += 16
            if start == 0 and end == 0:
                return intervals
            labels = []
            for entry in (start, end):
                if isinstance(entry, int):
                    raise Refused("DWARF: a numeric range entry")
                minuend, subtrahend = assembly.assignments.get(entry, (entry, None))
                if subtrahend is not None and subtrahend != base:
                    raise Refused(f"DWARF: range entry {entry} is not relative to its unit's base")
                labels.append(minuend)
            intervals.append((assembly.line_of(labels[0]), assembly.line_of(labels[1])))
    if AT_LOW_PC in attributes and AT_HIGH_PC in attributes:
        low, high = attributes[AT_LOW_PC], attributes[AT_HIGH_PC]
        end, start = assembly.assignments.get(high, (None, None)) if isinstance(high, str) \
            else (None, None)
        if not isinstance(low, str) or start != low:
            raise Refused(f"DWARF: high_pc {high!r} is not an end label minus low_pc {low!r}")
        return [(assembly.line_of(low), assembly.line_of(end))]
    return []


class Frame:
    def __init__(self, offset: int, tag: int, depth: int) -> None:
        self.offset, self.tag, self.depth = offset, tag, depth


def memset_frames(assembly: Assembly) -> tuple[dict, dict, dict[int, list[Frame]]]:
    """Walk every DIE: keep the attributes and parents the attribution needs, and for each call
    the subprograms and inlined subroutines whose code holds it."""
    info = assembly.sections["__debug_info"]
    tables = abbreviation_tables(assembly.sections["__debug_abbrev"])
    entries: dict[int, tuple[int, dict]] = {}
    parents: dict[int, int | None] = {}
    frames: dict[int, list[Frame]] = {call: [] for call in assembly.calls}
    offset = 0
    while offset < len(info.data):
        unit = offset
        length = assembly.difference(info.take(offset, 4), info)
        if info.take(offset + 4, 2) != 4:
            raise Refused(f"DWARF: unit at {unit} is not DWARF 4")
        table = tables.get(assembly.difference(info.take(offset + 6, 4),
                                               assembly.sections["__debug_abbrev"]))
        if table is None or info.take(offset + 10, 1) != 8:
            raise Refused(f"DWARF: unit at {unit} has no abbreviation table or 8-byte addresses")
        end, offset, stack, base = unit + 4 + length, offset + 11, [], None
        while offset < end:
            die = offset
            code, offset = info.leb(offset)
            if code == 0:
                stack.pop()
                continue
            if code not in table:
                raise Refused(f"DWARF: unknown abbreviation {code} at {die}")
            tag, children, specs = table[code]
            attributes = {}
            for attribute, form, implicit in specs:
                value, offset = read_form(info, offset, form, implicit, unit)
                if attribute in KEPT_ATTRIBUTES:
                    attributes[attribute] = value
            parents[die] = stack[-1] if stack else None
            if tag == TAG_COMPILE_UNIT:
                low = attributes.get(AT_LOW_PC)
                base = low if isinstance(low, str) else None
            if tag in (TAG_SUBPROGRAM, TAG_INLINED, TAG_NAMESPACE):
                entries[die] = (tag, attributes)
            if tag in (TAG_SUBPROGRAM, TAG_INLINED) and assembly.calls:
                for first, last in code_intervals(assembly, attributes, base):
                    for call in assembly.calls:
                        if first <= call < last:
                            frames[call].append(Frame(die, tag, len(stack)))
            if children:
                stack.append(die)
        if offset != end or stack:
            raise Refused(f"DWARF: unit at {unit} does not end where its length says")
    return entries, parents, frames


def declaration(entries: dict, die: int) -> int:
    for _ in range(16):
        _, attributes = entries.get(die, (None, {}))
        origin = attributes.get(AT_ABSTRACT_ORIGIN, attributes.get(AT_SPECIFICATION))
        if origin is None:
            return die
        die = origin
    raise Refused(f"DWARF: the origin chain of {die} does not end")


def crate_of(entries: dict, parents: dict, die: int) -> str | None:
    crate, node = None, parents.get(declaration(entries, die))
    while node is not None:
        tag, attributes = entries.get(node, (None, {}))
        if tag == TAG_NAMESPACE:
            crate = attributes.get(AT_NAME)
        node = parents.get(node)
    return crate


def count_memset(assembly: Assembly, products: list[str]) -> tuple[dict[str, int], list[str]]:
    """Per-crate `bl _memset_pattern16` counts of one post-LTO assembly, with one report line per
    call: its line, its function label and that label's crate, and the crate it is charged to."""
    entries, parents, frames = memset_frames(assembly)
    names = {product.replace("-", "_"): product for product in products}
    product_set = set(products)
    counts = {product: 0 for product in products}
    report = []
    for call in assembly.calls:
        chain = sorted(frames[call], key=lambda frame: -frame.depth)
        concrete = [frame for frame in chain if frame.tag == TAG_SUBPROGRAM]
        if len(concrete) != 1:
            raise Refused(f"line {call}: the memset call lies in {len(concrete)} described "
                          "function bodies, not one")
        crates = [crate_of(entries, parents, frame.offset) for frame in chain]
        if any(crate is None for crate in crates):
            raise Refused(f"line {call}: a function holding the memset call names no crate")
        crates = [names.get(debug_string(assembly, crate), debug_string(assembly, crate))
                  for crate in crates]
        label_crate = crates[-1]
        charged = next((crate for crate in crates if crate in product_set), label_crate)
        via = next(frame for frame, crate in zip(chain, crates) if crate == charged)
        _, attributes = entries[declaration(entries, via.offset)]
        name = debug_string(assembly, attributes.get(AT_LINKAGE_NAME, attributes.get(AT_NAME)))
        counts[charged] = counts.get(charged, 0) + 1
        index = bisect.bisect_left(assembly.functions, (call, ""))
        label = assembly.functions[index - 1][1] if index else "?"
        report.append(f"line {call}: {charged} (in {name}); function label {label} is "
                      f"{label_crate}")
    return counts, report


def refuses(action) -> bool:
    try:
        action()
    except Refused:
        return True
    return False


def synthetic_assembly(nested_module: bool = False, leading_unit: bool = False) -> str:
    """A post-LTO-shaped assembly with one function, `crate_a::caller`, that inlines
    `crate_b::inner`, which inlines `core::fill`. Its memset calls lie in `fill` (line 6), in
    `inner` only (line 8) and in `caller` only (line 10). The DIE offsets in the comments are the
    unit-relative offsets the references use. `nested_module` declares `inner` in a module of
    `crate_b` named `crate_a` (as `lane::kernels::builtins` is named like the crate `builtins`);
    `leading_unit` puts an empty unit first, so the unit's references are not absolute offsets."""
    strings = ["crate_a", "_Rcaller", "caller", "crate_b", "_Rinner", "inner", "core", "_Rfill",
               "fill"]
    at = {text: sum(len(earlier) + 1 for earlier in strings[:index])
          for index, text in enumerate(strings)}
    abbreviations = [  # code, tag, children, (attribute, form) pairs
        (1, TAG_COMPILE_UNIT, 1, [(AT_LOW_PC, 0x01), (AT_HIGH_PC, 0x06)]),
        (2, TAG_NAMESPACE, 1, [(AT_NAME, 0x0E)]),
        (3, TAG_SUBPROGRAM, 0, [(AT_LINKAGE_NAME, 0x0E), (AT_NAME, 0x0E), (0x3F, 0x19)]),
        (4, TAG_SUBPROGRAM, 1, [(AT_LOW_PC, 0x01), (AT_HIGH_PC, 0x06), (AT_ABSTRACT_ORIGIN, 0x13)]),
        (5, TAG_INLINED, 1, [(AT_ABSTRACT_ORIGIN, 0x13), (AT_RANGES, 0x17)]),
        (6, TAG_INLINED, 0, [(AT_ABSTRACT_ORIGIN, 0x13), (AT_LOW_PC, 0x01), (AT_HIGH_PC, 0x06)]),
    ]
    abbrev = []
    for code, tag, children, attributes in abbreviations:
        abbrev += [code, tag, children] + [value for pair in attributes for value in pair] + [0, 0]
    namespace = lambda crate: [".byte\t2", f".long\t{at[crate]}"]  # noqa: E731
    declared = lambda linkage, name: [".byte\t3", f".long\t{at[linkage]}",  # noqa: E731
                                      f".long\t{at[name]}"]
    module = namespace("crate_a") + declared("_Rinner", "inner") + [".byte\t0"] \
        if nested_module else declared("_Rinner", "inner")  # 89 (module), 94 (inner) when nested
    inner, fill = (94, 110) if nested_module else (89, 104)
    unit = ["Lset0 = Ldebug_info_end0-Ldebug_info_start0", ".long\tLset0",
            "Ldebug_info_start0:", ".short\t4", ".long\tLset1", ".byte\t8",
            ".byte\t1", ".quad\tLfunc_begin0", ".long\tLset2"]  # 11: the unit
    info = (unit
            + namespace("crate_a") + declared("_Rcaller", "caller")  # 24, 29
            + [".byte\t4", ".quad\tLfunc_begin0", ".long\tLset3", ".long\t29"]  # 38: caller
            + [".byte\t5", f".long\t{inner}", ".long\tLset4"]  # 55: inner, inlined
            + [".byte\t6", f".long\t{fill}", ".quad\tLtmp0", ".long\tLset5"]  # 64: fill, inlined
            + [".byte\t0", ".byte\t0", ".byte\t0"]
            + namespace("crate_b") + module + [".byte\t0"]  # 84
            + namespace("core") + declared("_Rfill", "fill") + [".byte\t0"]  # 99 (105 nested)
            + [".byte\t0", "Ldebug_info_end0:"])
    if leading_unit:  # 25 bytes: a unit whose compile unit has no children
        info = ["Lset8 = Ldebug_info_end8-Ldebug_info_start8", ".long\tLset8",
                "Ldebug_info_start8:", *unit[3:], ".byte\t0", "Ldebug_info_end8:", *info]
    lines = [
        ".section\t__TEXT,__text,regular,pure_instructions",
        "_caller:", "Lfunc_begin0:", "\tnop", "Ltmp0:", MEMSET_CALL.strip(), "Ltmp1:",
        MEMSET_CALL.strip(), "Ltmp2:", MEMSET_CALL.strip(), "Lfunc_end0:",
        ".section\t__DWARF,__debug_abbrev,regular,debug", "Lsection_abbrev:",
        *(f".byte\t{value}" for value in abbrev), ".byte\t0",
        ".section\t__DWARF,__debug_info,regular,debug", "Lsection_info:",
        *info,
        "Lset1 = Lsection_abbrev-Lsection_abbrev", "Lset2 = Lfunc_end0-Lfunc_begin0",
        "Lset3 = Lfunc_end0-Lfunc_begin0", "Lset4 = Ldebug_ranges0-Ldebug_range",
        "Lset5 = Ltmp1-Ltmp0",
        ".section\t__DWARF,__debug_ranges,regular,debug", "Ldebug_range:", "Ldebug_ranges0:",
        "Lset6 = Ltmp0-Lfunc_begin0", ".quad\tLset6", "Lset7 = Ltmp2-Lfunc_begin0",
        ".quad\tLset7", ".quad\t0", ".quad\t0",
        ".section\t__DWARF,__debug_str,regular,debug", "Linfo_string:",
        *(f'.asciz\t"{text}"' for text in strings),
    ]
    return "\n".join(line if line.endswith(":") or " = " in line else "\t" + line
                     for line in lines) + "\n"


def self_test() -> None:
    """Hermetic mutations of every judge, over synthetic rows: the real tables may shrink to
    nothing as defects are fixed, and the judges must still be proved."""
    reason = 'left: "' + "ab" * 32 + '"'
    name = "kernel::tests::scenario_is_pinned"
    rows: list[Row] = [("9999", "crate-a", "lib", name, reason),
                       ("9999", "crate-b", "test:t", "other_test", "assertion text")]
    ok_log = (f"running 1 test\ntest {name} ... FAILED\n\nfailures:\n\n---- {name} stdout ----\n\n"
              f"thread '{name}' (7) panicked at crates/x.rs:1:9:\nassertion `left == right` failed\n"
              f"  {reason}\n right: \"pin\"\nnote: run with `RUST_BACKTRACE=1`\n\n"
              f"test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out\n")
    assert judge_test(rows, name, ok_log, 101).startswith("expected failure (#9999)")
    mutations = {
        "passes": (ok_log, 0),
        "planted panic": (ok_log.replace(reason, "PLANTED: an unrelated regression"), 101),
        "another digest": (ok_log.replace(reason, reason[:-3] + '00"'), 101),
        "reason outside the panic": (ok_log.replace(f"  {reason}\n", "").replace(
            "running 1 test\n", f"running 1 test\n{reason}\n"), 101),
        "two failures": (ok_log.replace("0 passed; 1 failed", "0 passed; 2 failed"), 101),
        "not the test": (ok_log.replace(f"test {name} ... FAILED", f"test {name}_x ... FAILED"),
                         101),
    }
    for label, (log, status) in mutations.items():
        assert refuses(lambda: judge_test(rows, name, log, status)), label
    assert refuses(lambda: judge_test(rows, "not_a_row", ok_log, 101))

    listing = f"{name}: test\nother_test: test\nunrelated: test"
    judge_skips(rows, listing)
    assert refuses(lambda: judge_skips(rows, listing.replace(f"{name}: test\n", "")))
    assert refuses(lambda: judge_skips(rows, listing + f"\n{name}: test"))

    ceilings = {"crate-a": ("9999", 10), "crate-b": ("9999", 3)}
    products = ["crate-a", "crate-b", "crate-c"]
    counts = {"crate-a": 10, "crate-b": 3, "crate-c": 0}
    assert len(judge_memset(counts, products, ceilings)) == 2
    assert any("down from 10" in line
               for line in judge_memset(dict(counts, **{"crate-a": 4}), products, ceilings))
    for label, mutated, crates in (
        ("a row reaches zero", dict(counts, **{"crate-b": 0}), products),
        ("a count rises", dict(counts, **{"crate-a": 11}), products),
        ("a new crate has calls", dict(counts, **{"crate-c": 1}), products),
        ("a row for a crate that left the closure", counts, ["crate-a", "crate-c"]),
        ("a product crate was not measured", {k: v for k, v in counts.items() if k != "crate-c"},
         products),
    ):
        assert refuses(lambda: judge_memset(mutated, crates, ceilings)), label
    assert refuses(lambda: parse_counts("lane 0\nlane 0\n"))
    assert refuses(lambda: judge_memset(dict(counts, core=1), products, ceilings)), \
        "a call charged to a crate outside the product closure"

    # The post-LTO count. An inlined product crate's call is charged to it, not to the function
    # label's crate (D2); an inlined `core` frame is skipped; with no product frame the label's
    # crate is charged, and the judge refuses it.
    text = synthetic_assembly()
    count = lambda text, crates: count_memset(Assembly(text.splitlines()), crates)[0]  # noqa: E731
    assert count(text, ["crate-a", "crate-b"]) == {"crate-a": 1, "crate-b": 2}
    assert count(text, ["crate-a"]) == {"crate-a": 3}
    assert count(text, ["other"]) == {"other": 0, "crate_a": 3}
    # A frame's crate is its outermost namespace: a module named like another crate does not move
    # the charge (`lane::kernels::builtins` is not the crate `builtins`).
    assert count(synthetic_assembly(nested_module=True), ["crate-a", "crate-b"]) == \
        {"crate-a": 1, "crate-b": 2}, "a module named like another crate"
    # DIE references are relative to their unit, which is not the first one in the real file.
    assert count(synthetic_assembly(leading_unit=True), ["crate-a", "crate-b"]) == \
        {"crate-a": 1, "crate-b": 2}, "a unit that does not start the section"
    outside = text.replace("Lfunc_end0:\n", f"Lfunc_end0:\n_other:\n{MEMSET_CALL}\n")
    tail_call = f"{MEMSET_CALL}\nLfunc_end0:"
    for label, mutated in (
        ("a call outside every described function", outside),
        # A sibling call has no `bl`: without the refusal it would leave the count silently.
        ("a tail call", text.replace(tail_call, "\tb\t_memset_pattern16\nLfunc_end0:")),
        # The unit's closing null removed: its length still matches its labels, but the DIE tree
        # is left open.
        ("a unit whose DIE tree does not close", text.replace(
            "\t.byte\t0\nLdebug_info_end0:", "Ldebug_info_end0:")),
        # DW_AT_external's attribute code in hex: zero-filled it reads as attribute 0, which the
        # count ignores, so only the operand check can refuse it.
        ("a .byte operand that is not a decimal number", text.replace(
            "\t.byte\t63\n", "\t.byte\t0x3f\n")),
        # DW_AT_external's DW_FORM_flag_present (0x19, no bytes) read as form 0x02, which DWARF 4
        # does not define: the DIEs stay aligned, so only the form check can refuse it.
        ("an attribute form the reader does not know", text.replace(
            "\t.byte\t63\n\t.byte\t25\n", "\t.byte\t63\n\t.byte\t2\n")),
        ("a high_pc that is not an end label minus low_pc", text.replace(
            "Lset5 = Ltmp1-Ltmp0", "Lset5 = Ltmp1-Ltmp2")),
    ):
        assert mutated != text, label
        assert refuses(lambda: count(mutated, ["crate-a", "crate-b"])), label
    print("aarch64 known-defect judges: self-test passed")


def main(argv: list[str]) -> int:
    try:
        if argv == ["--self-test"]:
            self_test()
        elif len(argv) == 2 and argv[0] == "rows" and argv[1] in TEST_ROWS:
            for issue, package, selector, name, _ in TEST_ROWS[argv[1]]:
                print(f"{issue}|{package}|{selector}|{name}")
        elif len(argv) == 3 and argv[0] == "judge-skips" and argv[1] in TEST_ROWS:
            judge_skips(TEST_ROWS[argv[1]],
                        pathlib.Path(argv[2]).read_text(encoding="utf-8", errors="replace"))
        elif len(argv) == 5 and argv[0] == "judge-test" and argv[1] in TEST_ROWS:
            log = pathlib.Path(argv[3]).read_text(encoding="utf-8", errors="replace")
            print(judge_test(TEST_ROWS[argv[1]], argv[2], log, int(argv[4])))
        elif len(argv) == 4 and argv[0] == "count-memset":
            products = pathlib.Path(argv[2]).read_text(encoding="utf-8").split()
            with open(argv[1], encoding="utf-8") as assembly:
                counts, report = count_memset(Assembly(assembly), products)
            pathlib.Path(argv[3]).write_text(
                "".join(f"{crate} {count}\n" for crate, count in sorted(counts.items())),
                encoding="utf-8")
            print("\n".join(f"ios-asm-memset-pattern16: {line}" for line in report))
        elif len(argv) == 3 and argv[0] == "judge-memset":
            counts = parse_counts(pathlib.Path(argv[1]).read_text(encoding="utf-8"))
            products = pathlib.Path(argv[2]).read_text(encoding="utf-8").split()
            print("\n".join(judge_memset(counts, products, IOS_MEMSET_CEILINGS)))
        else:
            print(__doc__, file=sys.stderr)
            return 2
    except Refused as error:
        print(f"aarch64 known defects: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
