#!/usr/bin/env python3
"""V8 register-allocation gate over the EQ's stationary cascade loops in the shipped browser module.

Issues #1000 and #1009. The input is the shipped AudioWorklet module's named twin. The pinned
Node's V8 compiles the parametric EQ's two `f32x4` bank bodies with TurboFan; the gate finds the
stationary cascade's innermost loops in that machine code and fails when one carries a value from
one iteration to the next through a stack slot. It times nothing.

It reads the code that ships, from one build. The shipped module carries no `name` section (issue
#1109) and the gate finds the two functions by name, so it reads the build's named twin, which
`strip-wasm-names.py check` proves is the shipped module plus that section and nothing else: the
same code section, so the same code for V8 to compile. CI runs it in `artifact-gates` on the
downloaded twin, after that job has verified it against the `artifact` job's digest and checked it
against the shipped module. Locally `scripts/run-wasm-gates.sh` runs it on the named twin
`scripts/build-web-audioworklet.sh --module-only --named-twin` writes: the delivery build's own
cargo line. Neither holds the module to the release pin (#1061).

Why it exists
-------------
#977 attempt 1 made the standing one-band browser EQ about 20 % slower. TurboFan kept one integrator
of the select-free depth-one tail loop in a frame slot, stored mid-iteration and reloaded at the top
of the next, so the recurrence ran through store-to-load forwarding. The roster, the call-graph
rules, the digests and the in-crate timings all stayed green. Attempt 2 restored the tail's
surrounding code (see "The tail keeps #976's rule" on `interleave` in
`crates/parametric-eq/src/lib.rs`), and its verifier showed that a one-token edit to the tail
condition brings the carried slot back. The allocation is a property of V8's compilation of the
shipped bytes, so the gate reads that and nothing else.

What it proves, and what it does not
------------------------------------
For the pinned Node/V8 on an x86-64-v3 Linux host, in TurboFan's code for each held loop below, no
stack slot written in the loop carries a value from one iteration to a later one. Two rules find
such a slot, and either one fails the loop:

* **Live across the back edge**: the slot is read on a path from the loop's header before it is
  written (the #977 attempt-1 tail: reloaded at the top, stored mid-iteration).
* **On a recurrence**: a value loaded from the slot reaches a store to the same slot along a path,
  through registers and slots, that crosses the loop's header. This is the same mechanism wherever
  V8 lays out the store and the reload, including a reload on the back-edge path itself, which
  leaves nothing live across the header in memory (the masked mono tail does exactly that).

A loop-invariant spill that is only ever reloaded is neither, and is allowed. So is slot reuse
inside one iteration: V8 packs spill ranges that do not overlap into one slot, and a value that is
fresh every iteration may be spilled, reloaded and turned into another value spilled to the same
slot. No path from the load to the store crosses the header, so nothing is carried (issue #1009).

It proves nothing about speed beyond that one mechanism, and it measures no time. It is a proxy
for the browser twice over: Node's V8 is not a given Chrome's, and eager TurboFan without Liftoff's
feedback is not the tier-up a page gets. Red means "this build brings back the #977 mechanism in
the reference V8"; green does not mean "the browser EQ is as fast as before".

The loops
---------
The functions are found by symbol in the named twin's name section,
`PreparedParametricEq<f32x4, _>::process_bank` (dual) and `::process_bank_mono` (collapsed), each
exactly once. A loop inside a function has no symbol, so it is found by what it computes, never by
where it sits in the listing:

* **SVF steps** `k`: one `svf_step` plus its output mix is 7 `vmulps`, 9 `vaddps` and 2 `vsubps`
  (the flush's `vandps`/`vcmpps`/`vandnps` are not counted), so a loop running `k` steps per
  iteration contains exactly `7k`/`9k`/`2k`. A ramped section's six coefficient increments break
  the shape.
* **Streams** `s`: vector stores to non-stack memory per iteration. The last section of each
  stream writes its frame back, so there is one per stream.
* **Select-free**: no `vpor`/`vorps`/blend. The dry-mask kernels' bitselect needs one; the flush
  does not.

* **After the pairs** (the tails only): `interleave` runs the depth-2 passes and then the depth-1
  tail, so the tail is reachable from a pair loop. The ramp path's per-section `svf_block` loop
  computes exactly a mono tail's arithmetic (one section, one stream); today only V8's unrolling of
  it by three gives it another shape. It is not reachable from the stationary passes, so this
  tells the two apart with or without unrolling (issue #1009).

| function | loop | streams | steps | |
|---|---|---:|---:|---|
| dual | depth-1 tail (`svf_cascade_interleaved`, no dry lane), after the pair | 2 | 2 | held |
| mono | depth-2 pair (`svf_cascade_skewed`, admitted plan) | 1 | 2 | held |
| mono | depth-1 tail, after the pair | 1 | 1 | held |
| dual | depth-2 pair | 2 | 4 | reported |

A held row must match exactly one select-free innermost loop of its function. A row that matches
none, or more than one, fails closed and prints the loops that were found: nothing is checked, so
nothing passes. The masked kernels (a refused or all-live plan's pairs, a tail with a dry lane) are
out of scope: the masked depth-one tails carried a slot before #977.

**Why the dual pair is reported, not held.** It carries ten values across its back edge (eight
integrators and two skew carries) beside 24 loop-invariant coefficients, in sixteen vector
registers. At #977-#979 TurboFan routes ten stack slots through its recurrences, among them `ic1`
and `ic2` of one chain live across the back edge (issue #1000 evidence); #977's scan reported the
loop clean because it is entered in the middle. In a loop that starved, the count moves with any
allocation change and says nothing about time, so the gate prints it and does not hold it.

Loops are natural loops of the listing's control-flow graph (a back edge is a jump to a block that
dominates its source). Blocks that make a call are left out of a loop's body: inside these kernels
the only calls are the out-of-line stack guard and trap stubs, which spill live registers around
the call and do not run per iteration. A block with real work in it would take its arithmetic
along, and the loop would stop matching its row.

Determinism
-----------
Checked before anything else: the exact Node and V8 versions, the platform and architecture, and
the host CPU features V8 12.4 selects instructions by (all within x86-64-v3). The V8 flags are
`V8_FLAGS`, and the child runs without `NODE_OPTIONS`. The listing must come from TurboFan with a
non-empty protected-instruction table (trap-handler bounds checks, as a browser uses on x64).
Nothing in the listing is pinned: no offset, register or instruction count. Every verdict line
names the CPU model, since a CI runner's codegen is observed only there.

Re-pinning Node (`PINNED_NODE`, `PINNED_V8`) is a change to the reference V8, not a chore: build
the #1000 red arms (#977 attempt 1, and the one-token tail edit) and the current head, run the gate
on the new V8, and record what each gives. Keep the rule whatever they show; if a red arm turns
green, say so, rather than loosening a row to match.

Usage
-----
`check-web-audioworklet-v8-spill.py NAMED_TWIN.wasm` runs the gate.
`check-web-audioworklet-v8-spill.py --check-toolchain` checks the pins alone (a runner's preflight).
`check-web-audioworklet-v8-spill.py --self-test` runs synthetic listings through the analysis.
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
from collections import Counter
from dataclasses import dataclass, field

PINNED_NODE = "v22.23.2"
PINNED_V8 = "12.4.254.21-node.56"
PINNED_PLATFORM = ("linux", "x64")
# TurboFan for every function, compiled when the module is created. The one function asked for is
# printed; nothing is instantiated or run.
V8_FLAGS = ("--no-liftoff", "--no-wasm-lazy-compilation")
# V8 12.4's x64 `CpuFeatures::ProbeImpl` selects instructions by these, as `/proc/cpuinfo` spells
# them: SSE3, SSSE3, SSE4.1, SSE4.2, SAHF, AVX, AVX2, FMA3, BMI1, BMI2, LZCNT, POPCNT.
REQUIRED_CPU_FLAGS = (
    "pni", "ssse3", "sse4_1", "sse4_2", "lahf_lm", "avx", "avx2", "fma", "bmi1", "bmi2", "abm",
    "popcnt",
)
COMPILE_SCRIPT = "new WebAssembly.Module(require('node:fs').readFileSync(process.argv[1]));"

FUNCTIONS = (
    ("dual", r"13parametric_eq.*20PreparedParametricEq.*4wide6f32x4.*12process_bank"),
    ("mono", r"13parametric_eq.*20PreparedParametricEq.*4wide6f32x4.*17process_bank_mono"),
)


@dataclass(frozen=True)
class Row:
    function: str
    label: str
    streams: int
    steps: int
    held: bool = True
    after: str | None = None  # the label of a row whose loops this loop must be reachable from


PAIR = "depth-2 pair, select-free"
LOOPS = (
    Row("dual", "depth-1 tail, select-free", streams=2, steps=2, after=PAIR),
    Row("dual", PAIR, streams=2, steps=4, held=False),
    Row("mono", PAIR, streams=1, steps=2),
    Row("mono", "depth-1 tail, select-free", streams=1, steps=1, after=PAIR),
)
STEP_SHAPE = {"vmulps": 7, "vaddps": 9, "vsubps": 2}
SELECT_OPS = frozenset({"vpor", "vorps", "vpblendvb", "vblendvps", "vpternlogd", "vpternlogq"})
VECTOR_MOVES = frozenset({"vmovdqu", "vmovups", "vmovaps", "vmovapd"})

LINE = re.compile(r"^0x[0-9a-f]+\s+([0-9a-f]+)\s+[0-9a-f]+\s+(.*?)\s*$")
TABLE_ENTRY = re.compile(r"^jump table entry (\d+)$")
DIRECT_TARGET = re.compile(r"<\+0x([0-9a-f]+)>")
ABSOLUTE_TARGET = re.compile(r"0x[0-9a-f]+\s+(?=<\+0x|\()")
MEMORY = re.compile(r"\[([^\]]+)\]")
STACK_SLOT = re.compile(r"^(rbp|rsp)([+-]0x[0-9a-f]+)?$")
TABLE_ADDRESS = re.compile(r"^leaq r10,\[rip\+0x([0-9a-f]+)\]$")
PURE_STORES = re.compile(r"^(v?mov|v?pextr|v?extractps)")
COMPARES = re.compile(r"^(cmp|test|v?u?comis|v?ptest|bt)")
NO_VALUE_OPS = ("j", "call", "ret", "push", "pop", "nop")
ZEROING = re.compile(r"^(xor|sub|vxorp|vpxor|vpsub|vsubp)")
REGISTER = re.compile(r"^([xy]mm\d+|r\d+|r[a-z]{2})$")


class GateError(Exception):
    """A precondition failed: the verdict would not be the pinned toolchain's."""


@dataclass
class Instruction:
    offset: int
    op: str
    operands: list[str]
    text: str


@dataclass
class Block:
    start: int
    instructions: list[Instruction] = field(default_factory=list)
    successors: list[int] = field(default_factory=list)


@dataclass
class Loop:
    instructions: list[Instruction]
    blocks: int
    reaches: frozenset[int]  # every block reachable from the loop's blocks
    members: frozenset[int]
    ops: Counter
    steps: int | None
    streams: int
    select_free: bool
    carried: list[str]


# -------------------------------------------------------------------------------------------------
# Listing
# -------------------------------------------------------------------------------------------------


def split_operands(text: str) -> list[str]:
    parts, depth, current = [], 0, ""
    for char in text:
        depth += (char == "[") - (char == "]")
        if char == "," and depth == 0:
            parts.append(current.strip())
            current = ""
        else:
            current += char
    if current.strip():
        parts.append(current.strip())
    return parts


def parse_listing(listing: str) -> tuple[list[Instruction], dict[int, int]]:
    """Instructions and jump-table words (offset -> target) of one `--print-wasm-code` body."""
    lines = listing.splitlines()
    start = next((i for i, line in enumerate(lines) if line.startswith("Instructions (size")), None)
    if start is None:
        raise GateError("no `Instructions` section in the V8 listing")
    instructions: list[Instruction] = []
    tables: dict[int, int] = {}
    for line in lines[start + 1 :]:
        if not line.strip():
            break
        match = LINE.match(line)
        if match is None:
            continue
        offset, text = int(match[1], 16), match[2].removeprefix("REX.W ")
        text = ABSOLUTE_TARGET.sub("", text)  # this run's code address; `<+0x...>` stays
        entry = TABLE_ENTRY.match(text)
        if entry is not None:
            tables[offset] = int(entry[1])
            continue
        op, _, operands = text.partition(" ")
        instructions.append(Instruction(offset, op, split_operands(operands), text))
    if not instructions:
        raise GateError("the V8 listing has no instructions")
    return instructions, tables


def check_listing_header(listing: str, symbol: str, index: int) -> None:
    header = listing.split("Instructions (size", 1)[0]
    for line in (f"name: {symbol}", f"index: {index}", "kind: wasm function", "compiler: TurboFan"):
        if f"\n{line}\n" not in f"\n{header}":
            raise GateError(f"the V8 listing header lacks {line!r}")
    protected = listing.split("Protected instructions:", 1)[1:]
    if not protected or not re.search(r"^\s+[0-9a-f]+\s*$", protected[0].split("\n\n")[0], re.M):
        raise GateError(
            "the listing has no protected instructions: V8 compiled explicit bounds checks "
            "(no trap handler), which is not the code a browser runs on x64"
        )


# -------------------------------------------------------------------------------------------------
# Control flow, dominators, natural loops
# -------------------------------------------------------------------------------------------------


def build_blocks(instructions: list[Instruction], tables: dict[int, int]) -> dict[int, Block]:
    offsets = [instruction.offset for instruction in instructions]
    known = set(offsets)

    # `leaq r10,[rip+d]` then `jmp [r10+reg*8]`: the jump's table starts d bytes past the lea. It
    # runs over consecutive eight-byte entries up to the next jump's table.
    table_begins: dict[int, int] = {}
    for i, instruction in enumerate(instructions):
        if instruction.op != "jmp" or not instruction.operands[0].startswith("[r10+"):
            continue
        for back in range(i - 1, max(i - 8, -1), -1):
            address = TABLE_ADDRESS.match(instructions[back].text)
            if address is not None:
                table_begins[i] = instructions[back + 1].offset + int(address[1], 16)
                break
        else:
            raise GateError(f"indirect jump at {instruction.offset:#x} without a table address")
    begins = sorted(set(table_begins.values()))

    def table_targets(i: int) -> list[int]:
        begin = table_begins[i]
        end = next((other for other in begins if other > begin), None)
        targets, entry = [], begin
        while entry in tables and (end is None or entry < end):
            targets.append(tables[entry])
            entry += 8
        if not targets:
            raise GateError(f"indirect jump at {instructions[i].offset:#x} has an empty table")
        return targets

    branches: dict[int, list[int]] = {}
    leaders = {offsets[0]}
    for i, instruction in enumerate(instructions):
        following = offsets[i + 1] if i + 1 < len(offsets) else None
        if instruction.op.startswith("j"):
            direct = DIRECT_TARGET.search(" ".join(instruction.operands))
            if direct is not None:
                target = int(direct[1], 16)
                targets = [target] if instruction.op == "jmp" else [target, following]
            elif instruction.op == "jmp" and instruction.operands[0].startswith("[r10+"):
                targets = table_targets(i)
            else:
                targets = []  # leaves the function
        elif instruction.op.startswith("ret") or instruction.op in ("ud2", "int3", "hlt"):
            targets = []
        else:
            continue
        branches[instruction.offset] = list(dict.fromkeys(t for t in targets if t in known))
        leaders.update(branches[instruction.offset])
        if following is not None:
            leaders.add(following)
    blocks: dict[int, Block] = {}
    current: Block | None = None
    for instruction in instructions:
        if instruction.offset in leaders:
            if current is not None and current.instructions[-1].offset not in branches:
                current.successors = [instruction.offset]  # falls through
            current = blocks[instruction.offset] = Block(instruction.offset)
        assert current is not None
        current.instructions.append(instruction)
        if instruction.offset in branches:
            current.successors = branches[instruction.offset]
    return blocks


def dominators(blocks: dict[int, Block], entry: int) -> dict[int, int]:
    """Immediate dominators (Cooper, Harvey and Kennedy) of the blocks reachable from `entry`."""
    order: list[int] = []
    seen = {entry}
    stack = [(entry, iter(blocks[entry].successors))]
    while stack:
        node, children = stack[-1]
        child = next(children, None)
        if child is None:
            order.append(node)
            stack.pop()
        elif child not in seen:
            seen.add(child)
            stack.append((child, iter(blocks[child].successors)))
    rpo = order[::-1]
    number = {node: i for i, node in enumerate(rpo)}
    predecessors: dict[int, list[int]] = {node: [] for node in rpo}
    for node in rpo:
        for child in blocks[node].successors:
            predecessors[child].append(node)
    idom = {entry: entry}

    def intersect(a: int, b: int) -> int:
        while a != b:
            while number[a] > number[b]:
                a = idom[a]
            while number[b] > number[a]:
                b = idom[b]
        return a

    changed = True
    while changed:
        changed = False
        for node in rpo[1:]:
            done = [p for p in predecessors[node] if p in idom]
            new = done[0]
            for other in done[1:]:
                new = intersect(other, new)
            if idom.get(node) != new:
                idom[node] = new
                changed = True
    return idom


def natural_loops(blocks: dict[int, Block], entry: int) -> dict[int, set[int]]:
    """Header -> body of every natural loop, the bodies of a header's back edges merged."""
    idom = dominators(blocks, entry)

    def dominates(a: int, b: int) -> bool:
        while a != b:
            if idom[b] == b:
                return False
            b = idom[b]
        return True

    predecessors: dict[int, list[int]] = {node: [] for node in idom}
    for node in idom:
        for child in blocks[node].successors:
            predecessors[child].append(node)
    loops: dict[int, set[int]] = {}
    for node in idom:
        for header in blocks[node].successors:
            if dominates(header, node):
                body = loops.setdefault(header, {header})
                pending = [node]
                while pending:
                    member = pending.pop()
                    if member not in body:
                        body.add(member)
                        pending.extend(predecessors[member])
    return loops


# -------------------------------------------------------------------------------------------------
# Loop shape and the carried-slot rule
# -------------------------------------------------------------------------------------------------


def slot_accesses(instruction: Instruction) -> tuple[set[str], set[str]]:
    """(stack slots read, stack slots written) by one instruction."""
    if instruction.op.startswith(("lea", "nop")):
        return set(), set()
    reads, writes = set(), set()
    for position, operand in enumerate(instruction.operands):
        memory = MEMORY.search(operand)
        if memory is None or STACK_SLOT.match(memory[1]) is None:
            continue
        slot = f"[{memory[1]}]"
        if position > 0 or COMPARES.match(instruction.op):
            reads.add(slot)
            continue
        writes.add(slot)
        if PURE_STORES.match(instruction.op) is None:
            reads.add(slot)  # read-modify-write
    return reads, writes


def live_across(blocks: dict[int, Block], header: int, body: set[int]) -> set[str]:
    """Slots live into the header along the back edge: read before written on some path from the
    header, and written somewhere in the loop."""
    exposed: dict[int, set[str]] = {}
    stored: dict[int, set[str]] = {}
    for node in body:
        exposed[node], stored[node] = set(), set()
        for instruction in blocks[node].instructions:
            reads, writes = slot_accesses(instruction)
            exposed[node] |= reads - stored[node]
            stored[node] |= writes
    live_in: dict[int, set[str]] = {node: set() for node in body}
    changed = True
    while changed:
        changed = False
        for node in body:
            live_out = set().union(
                *(live_in[child] for child in blocks[node].successors if child in body)
            )
            new = exposed[node] | (live_out - stored[node])
            if new != live_in[node]:
                live_in[node] = new
                changed = True
    return live_in[header] & set().union(*stored.values())


def data_flow(instruction: Instruction) -> tuple[list[str], list[str]]:
    """(locations written, locations read) by one instruction's values: registers and stack slots.

    Address registers are not values, flags are ignored, and a load from non-stack memory reads
    nothing this analysis tracks. Unknown shapes read every operand, which can only add a path."""
    op, operands = instruction.op, instruction.operands

    def location(operand: str) -> str | None:
        memory = MEMORY.search(operand)
        if memory is not None:
            return f"[{memory[1]}]" if STACK_SLOT.match(memory[1]) else None
        return operand if REGISTER.match(operand) else None

    if not operands or COMPARES.match(op) or op.startswith(NO_VALUE_OPS):
        return [], []
    if op.startswith("lea"):
        address = MEMORY.search(operands[1]) if len(operands) > 1 else None
        registers = re.findall(r"[a-z][a-z0-9]*", address[1]) if address else []
        return [operands[0]], [r for r in registers if REGISTER.match(r) and r != "rip"]
    written = [loc for loc in [location(operands[0])] if loc is not None]
    if PURE_STORES.match(op) or (op.startswith("v") and len(operands) >= 3):
        sources = operands[1:]
    else:
        sources = operands  # two-operand arithmetic reads its destination
    read = [loc for loc in map(location, sources) if loc is not None]
    if ZEROING.match(op) and len(set(operands)) == 1:
        read = []  # `xor x,x`, `vpxor x,x,x`: no input
    return written, read


def recurrent_slots(blocks: dict[int, Block], header: int, body: set[int]) -> set[str]:
    """Slots on a loop-carried dependence cycle: a value loaded from the slot reaches a store to
    the same slot along a path, through registers and slots, that crosses the loop's header. This
    is the #977 mechanism wherever V8 lays out the store and the reload, including a reload on the
    back-edge path itself, which leaves nothing live across the header in memory.

    A path that stays inside one iteration is slot reuse, not a carry: V8 packs spill ranges that
    do not overlap into one slot, so a value that is fresh every iteration can be spilled,
    reloaded and turned into another value spilled to the same slot. The taint therefore carries
    a bit, set when it flows along the back edge into the header, and only a crossed value
    stored to its own slot counts."""
    stored = {
        slot
        for node in body
        for instruction in blocks[node].instructions
        for slot in slot_accesses(instruction)[1]
    }
    found = set()
    for slot in stored:
        # location -> whether some path to it from a load of `slot` crossed the header.
        tainted: dict[int, dict[str, bool]] = {node: {} for node in body}
        changed = True
        while changed:
            changed = False
            for node in body:
                state = dict(tainted[node])
                for instruction in blocks[node].instructions:
                    written, read = data_flow(instruction)
                    sources = [state[location] for location in read if location in state]
                    if slot in read and slot not in state:
                        sources.append(False)  # a fresh load of the slot starts a path
                    crossed = any(sources)
                    if sources and crossed and slot in written:
                        found.add(slot)
                    for location in written:
                        if sources:
                            state[location] = crossed
                        else:
                            state.pop(location, None)
                for child in blocks[node].successors:
                    if child not in body:
                        continue
                    target = tainted[child]
                    for location, bit in state.items():
                        bit = bit or child == header  # along the back edge
                        if target.get(location) is None or (bit and not target[location]):
                            target[location] = bit
                            changed = True
    return found


def analyse(listing: str) -> list[Loop]:
    instructions, tables = parse_listing(listing)
    blocks = build_blocks(instructions, tables)
    loops = natural_loops(blocks, instructions[0].offset)
    result = []
    for header in sorted(loops):
        if any(other != header and other in loops[header] for other in loops):
            continue  # not innermost
        body = {
            node
            for node in loops[header]
            if not any(instruction.op == "call" for instruction in blocks[node].instructions)
        }
        if header not in body:
            continue
        code = [instruction for node in sorted(body) for instruction in blocks[node].instructions]
        ops = Counter(instruction.op for instruction in code)
        k = ops["vmulps"] // STEP_SHAPE["vmulps"]
        steps = k if k and all(ops[op] == k * n for op, n in STEP_SHAPE.items()) else None
        streams = sum(
            1
            for instruction in code
            if instruction.op in VECTOR_MOVES
            and (memory := MEMORY.search(instruction.operands[0])) is not None
            and STACK_SLOT.match(memory[1]) is None
        )
        select_free = not any(ops[op] for op in SELECT_OPS)
        carried = sorted(live_across(blocks, header, body) | recurrent_slots(blocks, header, body))
        reaches, pending = set(body), list(body)
        while pending:
            for child in blocks[pending.pop()].successors:
                if child not in reaches:
                    reaches.add(child)
                    pending.append(child)
        result.append(
            Loop(code, len(body), frozenset(reaches), frozenset(body), ops, steps, streams,
                 select_free, carried)
        )
    return result


def describe(loop: Loop) -> str:
    shape = f"{loop.steps} SVF steps" if loop.steps is not None else "no SVF-step shape"
    return (
        f"{len(loop.instructions)} instructions in {loop.blocks} blocks, {shape}, "
        f"{loop.streams} streams, {'select-free' if loop.select_free else 'masked'}, "
        f"vmulps={loop.ops['vmulps']} vaddps={loop.ops['vaddps']} vsubps={loop.ops['vsubps']}"
    )


def shaped(loops: list[Loop], row: Row, rows: tuple[Row, ...]) -> list[Loop]:
    """The select-free loops of a row's shape, restricted by its `after` row."""
    matched = [
        loop
        for loop in loops
        if loop.select_free and loop.steps == row.steps and loop.streams == row.streams
    ]
    if row.after is not None:
        (before,) = [r for r in rows if r.function == row.function and r.label == row.after]
        sources = shaped(loops, before, rows)
        matched = [
            loop
            for loop in matched
            if any(loop is not source and loop.members <= source.reaches for source in sources)
        ]
    return matched


def check_function(name: str, listing: str, rows: tuple[Row, ...] = LOOPS) -> int:
    """Hold one function's loops to its rows. Returns the number of failures."""
    loops = analyse(listing)
    failures = 0
    for row in rows:
        if row.function != name:
            continue
        label = f"{name} {row.label}"
        matched = shaped(loops, row, rows)
        if not row.held:
            for loop in matched or [None]:
                state = (
                    f"{describe(loop)}; {len(loop.carried)} carried slots"
                    if loop is not None
                    else "no loop of its shape"
                )
                print(f"info {label} (reported, not held): {state}")
            continue
        if len(matched) != 1:
            failures += 1
            where = f", reachable from the {row.after} loop," if row.after else ""
            print(
                f"FAIL {label}: {len(matched)} innermost loops with {row.streams} streams and "
                f"{row.steps} SVF steps{where} where exactly one was expected, so nothing was "
                "checked (fail closed). Innermost loops with SVF arithmetic:",
                file=sys.stderr,
            )
            for loop in loops:
                if loop.ops["vmulps"]:
                    mark = "*" if any(loop is other for other in matched) else " "
                    print(f"  {mark} {describe(loop)}", file=sys.stderr)
            continue
        (loop,) = matched
        if not loop.carried:
            print(f"ok   {label}: {describe(loop)}; no carried stack slot")
            continue
        failures += 1
        print(
            f"FAIL {label}: V8 carries {', '.join(loop.carried)} from one iteration to the "
            f"next ({describe(loop)}). Listing, carried slots marked:",
            file=sys.stderr,
        )
        for instruction in loop.instructions:
            touched = set().union(*slot_accesses(instruction)) & set(loop.carried)
            mark = "*" if touched else " "
            print(f"  {mark} {instruction.offset:6x}  {instruction.text}", file=sys.stderr)
    return failures


# -------------------------------------------------------------------------------------------------
# Toolchain, module and V8
# -------------------------------------------------------------------------------------------------


def child_environment() -> dict[str, str]:
    # No NODE_OPTIONS: nothing outside `V8_FLAGS` reaches V8.
    return {"PATH": os.environ.get("PATH", "/usr/bin:/bin")}


def check_toolchain(node: str) -> str:
    """Refuse any Node, V8, platform or host CPU but the pinned ones; return the CPU model."""
    script = "[process.version, process.versions.v8, process.platform, process.arch].join(' ')"
    probe = subprocess.run(
        [node, "-p", script],
        capture_output=True, text=True, env=child_environment(), check=False,
    )
    found = probe.stdout.split() if probe.returncode == 0 else []
    if found != [PINNED_NODE, PINNED_V8, *PINNED_PLATFORM]:
        raise GateError(
            f"Node {PINNED_NODE} (V8 {PINNED_V8}) on {'/'.join(PINNED_PLATFORM)} is required, "
            f"found {' '.join(found) or probe.stderr.strip() or 'nothing'} at {node}. Another V8 "
            "allocates registers differently, so its verdict is not this gate's."
        )
    try:
        with open("/proc/cpuinfo", encoding="ascii", errors="replace") as cpuinfo:
            fields = dict(
                (key.strip(), value.strip())
                for key, _, value in (line.partition(":") for line in cpuinfo)
                if key.strip() in ("flags", "model name")
            )
        flags = fields["flags"]
    except (OSError, KeyError):
        raise GateError("cannot read the host CPU flags from /proc/cpuinfo") from None
    missing = [flag for flag in REQUIRED_CPU_FLAGS if flag not in flags.split()]
    if missing:
        raise GateError(f"the host CPU lacks {', '.join(missing)}; V8 would select other code")
    # Printed on every verdict: the runner's codegen is observed only there (issue #1009).
    return fields.get("model name", "unknown model")


def leb128(data: bytes, position: int) -> tuple[int, int]:
    result = shift = 0
    while True:
        byte = data[position]
        position += 1
        result |= (byte & 0x7F) << shift
        shift += 7
        if not byte & 0x80:
            return result, position


def function_names(module: bytes) -> dict[int, str]:
    """The name section's function names (subsection 1)."""
    if module[:8] != b"\0asm\x01\0\0\0":
        raise GateError("not a WebAssembly module")
    names: dict[int, str] = {}
    position = 8
    while position < len(module):
        section = module[position]
        size, position = leb128(module, position + 1)
        end = position + size
        if section == 0:
            length, cursor = leb128(module, position)
            if module[cursor : cursor + length] == b"name":
                cursor += length
                while cursor < end:
                    kind = module[cursor]
                    sub_size, cursor = leb128(module, cursor + 1)
                    if kind == 1:
                        count, inner = leb128(module, cursor)
                        for _ in range(count):
                            index, inner = leb128(module, inner)
                            length, inner = leb128(module, inner)
                            names[index] = module[inner : inner + length].decode()
                            inner += length
                    cursor += sub_size
        position = end
    if not names:
        raise GateError("the module has no function names; the shipped module carries none "
                        "(#1109), so give the gate the build's named twin")
    return names


def v8_listing(node: str, artifact: str, index: int) -> str:
    run = subprocess.run(
        [node, *V8_FLAGS, f"--print-wasm-code-function-index={index}", "-e", COMPILE_SCRIPT,
         artifact],
        capture_output=True, text=True, env=child_environment(), check=False, timeout=300,
    )
    if run.returncode != 0:
        raise GateError(f"node failed to compile the module: {run.stderr.strip()[:2000]}")
    if run.stdout.count("--- WebAssembly code ---") != 1:
        raise GateError(f"expected exactly one printed function for index {index}")
    return run.stdout


def pinned_node() -> tuple[str, str]:
    node = shutil.which("node")
    if node is None:
        raise GateError(f"node is not on PATH; the gate needs Node {PINNED_NODE}")
    return node, check_toolchain(node)


def run_gate(artifact: str, node: str) -> int:
    with open(artifact, "rb") as module:
        names = function_names(module.read())
    failures = 0
    for name, pattern in FUNCTIONS:
        matches = [(index, symbol) for index, symbol in names.items() if re.search(pattern, symbol)]
        if len(matches) != 1:
            failures += 1
            print(f"FAIL {name}: {len(matches)} functions match {pattern!r} (expected exactly one)",
                  file=sys.stderr)
            continue
        index, symbol = matches[0]
        listing = v8_listing(node, artifact, index)
        check_listing_header(listing, symbol, index)
        failures += check_function(name, listing)
    return failures


# -------------------------------------------------------------------------------------------------
# Self-test
# -------------------------------------------------------------------------------------------------


def assemble(code: list[str]) -> str:
    """A V8-format listing of `code`. A line `NAME:` labels the next instruction, and `<+NAME>`
    in a jump targets it."""
    labels, rows = {}, []
    for text in code:
        if text.endswith(":"):
            labels[text[:-1]] = 4 * len(rows)
        else:
            rows.append(text)
    out = ["--- WebAssembly code ---", "Instructions (size = 0)"]
    for i, text in enumerate(rows):
        text = re.sub(r"<\+([A-Z]\w*)>", lambda m: f"0xdead  <+{labels[m[1]]:#x}>", text)
        out.append(f"0x{4 * i:012x}  {4 * i:4x}  90909090             {text}")
    return "\n".join(out) + "\n\n"


def synthetic(body: list[str], outline: list[str] = ()) -> str:
    """An entry, a loop closing over `body`, a return, then `outline` (out-of-line code)."""
    return assemble(
        ["movl rax,0x40", "BODY:", *body, "subl rax,0x1", "jnz <+BODY>", "retl", "OUTLINE:",
         *outline]
    )


def svf_step(stream: str) -> list[str]:
    """One step's counted arithmetic (7 mul, 9 add, 2 sub) and its frame store to `stream`."""
    return [
        f"vmovdqu xmm0,[rbx+{stream}*1]",
        *["vmulps xmm1,xmm1,xmm2"] * 7,
        *["vaddps xmm1,xmm1,xmm2"] * 9,
        *["vsubps xmm1,xmm1,xmm2"] * 2,
        f"vmovdqu [rbx+{stream}*1],xmm1",
    ]


def self_test() -> int:
    tail = svf_step("rax") + svf_step("rsi")
    carried = ["vmovups xmm0,[rbp-0xc0]", *tail, "vmovups [rbp-0xc0],xmm5"]
    # A rotated loop: entered in the middle, so the back edge's store comes first in the listing.
    rotated = synthetic(
        ["vmovups [rbp-0xc0],xmm5", "vmovups xmm0,[rbp-0xc0]", *tail, "vmovups [rbp-0xc0],xmm5"]
    ).replace("movl rax,0x40", "jmp 0xdead  <+0x8>")
    guard = synthetic(
        ["vmovups xmm0,[rbp-0x90]", "cmpq rsp,[r13-0x60]", "jna <+OUTLINE>", *tail],
        ["vmovups [rbp-0x90],xmm0", "call 0xbeef  (jump table)", "vmovups xmm0,[rbp-0x90]",
         "jmp <+BODY>"],
    )
    # (label, listing, (steps, streams, carried, select-free) of its one loop)
    cases = (
        # An invariant reload, and a slot written before it is read within the iteration.
        ("clean", synthetic(["vmovups xmm2,[rbp-0x40]", "vmovups [rbp-0x50],xmm3", *tail,
                             "vmovups xmm4,[rbp-0x50]"]), (2, 2, [], True)),
        # The #977 shape: an integrator reloaded at the top and stored mid-iteration.
        ("carried", synthetic(carried), (2, 2, ["[rbp-0xc0]"], True)),
        ("rotated", rotated, (2, 2, ["[rbp-0xc0]"], True)),
        # The masked mono tail's shape: the new integrator is spilled mid-iteration and reloaded
        # on the back-edge path, so nothing is live across the header in memory.
        ("back-edge reload", synthetic(["vandps xmm11,xmm10,xmm0", "vminps xmm13,xmm11,xmm0",
                                        "vmovups [rbp-0xa0],xmm13", *tail,
                                        "vmovups xmm0,[rbp-0xa0]"]), (2, 2, ["[rbp-0xa0]"], True)),
        # Slot reuse within an iteration (the #1000 verdict's finding 2): A is fresh each
        # iteration, is spilled and reloaded, and B = f(A) is spilled to the same slot. No path
        # from a load of the slot to a store to it crosses the header.
        ("slot reuse", synthetic(["vmovdqu xmm3,[rbx+rdx*1]", "vandps xmm3,xmm3,xmm2",
                                  "vmovups [rbp-0x40],xmm3", *tail, "vmovups xmm4,[rbp-0x40]",
                                  "vandps xmm5,xmm4,xmm2", "vmovups [rbp-0x40],xmm5",
                                  "vmovups xmm6,[rbp-0x40]", "vandps xmm7,xmm6,xmm2"]),
         (2, 2, [], True)),
        # The same slot, but the value stored is a fresh zero: no recurrence.
        ("zero idiom", synthetic(["vpxor xmm13,xmm13,xmm13", "vmovups [rbp-0xa0],xmm13", *tail,
                                  "vmovups xmm13,[rbp-0xa0]"]), (2, 2, [], True)),
        # A read-modify-write, a slot read as an arithmetic operand, and a compare that only reads.
        ("rmw", synthetic([*tail, "addl [rbp-0x18],0x1"]), (2, 2, ["[rbp-0x18]"], True)),
        ("compare", synthetic(["cmpl [rbp-0x18],r8", *tail]), (2, 2, [], True)),
        ("operand", synthetic(["vandps xmm1,xmm1,[rbp-0x70]", *svf_step("rax"),
                               "vmovups [rbp-0x70],xmm1"]), (1, 1, ["[rbp-0x70]"], True)),
        # A lea names a slot without reading it.
        ("lea", synthetic(["leaq r8,[rbp-0x20]", *tail, "vmovups [rbp-0x20],xmm1"]),
         (2, 2, [], True)),
        # The dry-mask kernels' bitselect.
        ("masked", synthetic([*tail, "vpand xmm1,xmm1,xmm3", "vpandn xmm3,xmm3,xmm4",
                              "vpor xmm1,xmm1,xmm3"]), (2, 2, [], False)),
        # A ramped section's six coefficient increments.
        ("ramped", synthetic(svf_step("rax") + ["vaddps xmm5,xmm5,xmm6"] * 6), (None, 1, [], True)),
        # An out-of-line stack guard that spills around its call.
        ("guard", guard, (2, 2, [], True)),
    )
    failures = 0
    for label, listing, expected in cases:
        got = [(l.steps, l.streams, l.carried, l.select_free) for l in analyse(listing)]
        if got != [expected]:
            failures += 1
            print(f"self-test FAIL {label}: got {got}, want [{expected}]", file=sys.stderr)
    # Verdicts: carried fails, a reported row never fails, no loop fails closed, clean passes,
    # then the two `after` layouts below.
    verdicts = []
    with open(os.devnull, "w") as sink:
        stdout, stderr = sys.stdout, sys.stderr
        sys.stdout = sys.stderr = sink
        try:
            for listing, held in ((carried, True), (carried, False), (svf_step("rax"), True),
                                  (tail, True)):
                rows = (Row("t", "tail", streams=2, steps=2, held=held),)
                verdicts.append(check_function("t", synthetic(listing), rows))
            # `after`: of two loops of the tail's shape, the one the pair reaches is the tail; a
            # ramp-path loop of the same shape carrying a slot is not held. When the pair reaches
            # both, the row is ambiguous and fails closed.
            ramp = ["vmovups xmm0,[rbp-0x30]", *svf_step("rdx"), "vmovups [rbp-0x30],xmm1"]
            layout = ["movl rax,0x40", "cmpl rdi,0x0", "jz <+RAMP>", "PAIR:", *tail,
                      "subl rax,0x1", "jnz <+PAIR>", "TAIL:", *svf_step("rsi"), "subl rcx,0x1",
                      "jnz <+TAIL>", "retl", "RAMP:", *ramp, "subl rdx,0x1", "jnz <+RAMP>", "retl"]
            rows = (Row("t", "pair", streams=2, steps=2, held=False),
                    Row("t", "tail", streams=1, steps=1, after="pair"))
            verdicts.append(check_function("t", assemble(layout), rows))
            layout[layout.index("jnz <+TAIL>") + 1] = "jmp <+RAMP>"
            verdicts.append(check_function("t", assemble(layout), rows))
        finally:
            sys.stdout, sys.stderr = stdout, stderr
    if verdicts != [1, 0, 1, 0, 0, 1]:
        failures += 1
        print(f"self-test FAIL verdicts: {verdicts}, want [1, 0, 1, 0, 0, 1]", file=sys.stderr)
    if failures:
        return 1
    print(f"V8 spill gate self-test: {len(cases) + len(verdicts)} cases ok")
    return 0


def main() -> int:
    sys.stdout.reconfigure(line_buffering=True)  # keep `ok` and `FAIL` lines in order in a log
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n", 1)[0])
    parser.add_argument("artifact", nargs="?", help="the shipped module's named twin")
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument(
        "--check-toolchain", action="store_true", help="check the pinned Node, V8 and host only"
    )
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if args.check_toolchain == (args.artifact is not None):
        parser.error("give an artifact or --check-toolchain")
    try:
        node, cpu = pinned_node()
        where = f"Node {PINNED_NODE}, V8 {PINNED_V8}, {' '.join(V8_FLAGS)}, CPU {cpu}"
        if args.check_toolchain:
            print(f"V8 spill gate: toolchain ok ({where})")
            return 0
        failures = run_gate(args.artifact, node)
    except GateError as error:
        print(f"V8 spill gate: {error}", file=sys.stderr)
        return 2
    if failures:
        print(f"V8 spill gate: {failures} failure(s) ({where})", file=sys.stderr)
        return 1
    print(f"V8 spill gate: ok ({where})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
