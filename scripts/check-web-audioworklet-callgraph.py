#!/usr/bin/env python3
"""Static call-graph and opcode gates over the shipped browser AudioWorklet artifact.

Issue #106 evals E1, E2 and E5. The input is `wasm-objdump -d ARTIFACT` on stdin: the gate reads
the binary that ships, not an rlib, a debug build or the Rust source, because "the render path
never frees" is a property of the emitted code and nothing else can witness it.

Modes
-----
`--callgraph EXPORT [--trap-owner SUBSTRING ...]`
    Walk the direct-call closure of `EXPORT` and fail if any member's name matches `FORBIDDEN`
    (allocator, deallocator or drop glue). Then list the closure members that contain an
    `unreachable` instruction and fail unless that set equals `TRAP_ALLOW_LIST` plus any
    `--trap-owner` substrings the caller named.

    `--allocation-only` runs the forbidden-name half alone. Use it for an export that runs on the
    control path (`port.onmessage`), where the engine's rule is "never allocate on the render
    thread" and a checked index inside a pure-math helper is not an allocation. It is refused
    together with `--trap-owner`, so a caller states one intent or the other.

    `--trap-owner` never relaxes the allocation half of the gate, which is the half that matters:
    an allocator, a deallocator or drop glue in the closure is a failure no matter what. It exists
    because issue #137 put two more exports on the render thread -- `miso_engine_web_v1_meter_poll`
    and `miso_engine_web_v1_command_submit` -- whose bodies inline the bounded SPSC endpoints'
    own checked slot indexing. Naming the export's own symbol keeps the strong statement "nothing
    this export *calls* may trap" while admitting the one checked index the queue primitive emits.

`--simd-floor N --kernel-pattern REGEX --kernel-min K`
    Assert the artifact still contains the vector kernels: at least `N` `f32x4.{mul,add,sub}`
    instructions in total, and at least `K` functions matching `REGEX` that carry
    `f32x4.{mul,add,sub,div}` arithmetic, each of which must use strictly more vector than scalar
    `f32` arithmetic. `N` and `K` are **ratchets**: when a wave moves kernels and the counts rise,
    raise them. A count below the floor is a regression to report, never a floor to lower.

    The per-kernel rule is "vector dominates", not "no scalar at all". At `ae02d2a` the kernels were
    hand-written `core::arch::wasm32` bodies with literally zero scalar `f32` arithmetic; after
    wave 2 they are `Lane`-over-`wide` generic bodies instantiated at `wide::f32x4`/`f32x8`, and a
    real instantiation legitimately keeps a handful of scalar coefficient and tail operations.
    Asserting the old absolute rule would assert something that is no longer true; strict
    domination is the strongest statement the current code actually supports, and it still fails
    loudly if a kernel silently de-vectorises.

`--self-test`
    Synthetic disassembly cases (a)-(f) below, each the red mutation of one rule.

Why the traversal stops at the panic entry functions
----------------------------------------------------
`PANIC_ENTRY` names are kept as closure *members* -- a trap owner is still reported -- but their
callees are not followed. Below `panic_fmt` lies the std abort runtime (`panic_with_hook`, the
hook, the backtrace formatter, `__rust_abort`), which formats and frees on its way to `abort`.
Following it makes every artifact reach `dlmalloc::free` through `drop_glue<Option<Vec<u8>>>` and
the gate would say nothing about the render path. A non-panicking render never executes any of it.
"""

from __future__ import annotations

import argparse
import re
import sys

HEADER = re.compile(r"^([0-9a-f]+) func\[(\d+)\](?: <(.+)>)?:")
CALL = re.compile(r"\|\s*call (\d+)")
INSN = re.compile(r"^\s*[0-9a-f]+: [0-9a-f ]+\|\s*(\S+)")

PANIC_ENTRY = re.compile(
    r"core9panicking|slice_index_fail|panic_bounds_check|expect_failed|unwrap_failed"
    r"|panic_const|panic_fmt"
)
FORBIDDEN = re.compile(
    r"dealloc|dlmalloc|free|malloc|drop_glue|drop_in_place|drop_slow|unlink_chunk"
    r"|insert_large_chunk|memory_grow|__rust_alloc"
)
# The one non-entry trap owner that may remain, with the reason it is unreachable in production:
# `PreparedRenderPlan::render_inner` inlines `PlanarBufferMut::plane_mut`, whose `&mut
# self.storage[start..end]` is a checked slice index, in the executor-less silence branch. Every
# web plan carries an executor (both graph binding paths end in `prepare_with_executor`), so the
# branch is dead. It is core-owned (#84) and is not fixed from this job.
TRAP_ALLOW_LIST = frozenset({"18PreparedRenderPlan12render_inner"})

VECTOR = re.compile(r"^(v128|i8x16|i16x8|i32x4|i64x2|f32x4|f64x2)\.")
SIMD_ARITH = re.compile(r"^f32x4\.(mul|add|sub)$")
KERNEL_VECTOR = re.compile(r"^f32x4\.(mul|add|sub|div)$")
KERNEL_SCALAR = re.compile(r"^f32\.(mul|add|sub|div)$")


class Function:
    __slots__ = ("index", "name", "calls", "opcodes")

    def __init__(self, index: int, name: str) -> None:
        self.index = index
        self.name = name
        self.calls: list[int] = []
        self.opcodes: list[str] = []


def parse(text: str) -> dict[int, Function]:
    """Parse `wasm-objdump -d` output into an index -> function map."""
    functions: dict[int, Function] = {}
    current: Function | None = None
    for line in text.splitlines():
        header = HEADER.match(line)
        if header is not None:
            name = header.group(3)
            if name is None:
                raise SystemExit(
                    "name section required: func[%s] has no <name>; a stripped artifact "
                    "blinds this gate, so `strip` must keep the name section" % header.group(2)
                )
            current = Function(int(header.group(2)), name)
            functions[current.index] = current
            continue
        if current is None:
            continue
        call = CALL.search(line)
        if call is not None:
            current.calls.append(int(call.group(1)))
        instruction = INSN.match(line)
        if instruction is not None:
            current.opcodes.append(instruction.group(1))
    if not functions:
        raise SystemExit("no functions found; is this `wasm-objdump -d` output?")
    return functions


def closure(functions: dict[int, Function], export: str) -> list[Function]:
    roots = [function for function in functions.values() if export in function.name]
    if not roots:
        raise SystemExit(f"export not found in the disassembly: {export}")
    if len(roots) != 1:
        raise SystemExit(f"ambiguous export name {export}: {[r.name for r in roots]}")
    seen: dict[int, Function] = {}
    pending = [roots[0]]
    while pending:
        function = pending.pop()
        if function.index in seen:
            continue
        seen[function.index] = function
        if PANIC_ENTRY.search(function.name):
            continue  # a member, but the abort runtime below it is never executed
        for index in function.calls:
            callee = functions.get(index)
            if callee is not None and callee.index not in seen:
                pending.append(callee)
    return sorted(seen.values(), key=lambda function: function.index)


def check_callgraph(
    functions: dict[int, Function],
    export: str,
    trap_owners: tuple[str, ...] = (),
    allocation_only: bool = False,
) -> int:
    allowed_owners = frozenset(TRAP_ALLOW_LIST | set(trap_owners))
    members = closure(functions, export)
    failures = 0
    forbidden = [function.name for function in members if FORBIDDEN.search(function.name)]
    if forbidden:
        failures += 1
        print(
            f"FAIL {export}: the render closure reaches allocation or drop glue:", file=sys.stderr
        )
        for name in sorted(forbidden):
            print(f"  {name}", file=sys.stderr)
    trap_owners = [
        function
        for function in members
        if "unreachable" in function.opcodes and not PANIC_ENTRY.search(function.name)
    ]
    unexpected = [
        function.name
        for function in trap_owners
        if not any(allowed in function.name for allowed in allowed_owners)
    ]
    if unexpected and not allocation_only:
        failures += 1
        print(f"FAIL {export}: unexpected trap owner in the render closure:", file=sys.stderr)
        for name in sorted(unexpected):
            print(f"  {name}", file=sys.stderr)
    traps = sum(function.opcodes.count("unreachable") for function in members)
    entries = sorted({function.name for function in members if PANIC_ENTRY.search(function.name)})
    print(
        f"{export}: closure={len(members)} traps={traps} "
        f"trap_owners={sorted(function.name for function in trap_owners)} entries={entries}"
    )
    return failures


def check_simd(
    functions: dict[int, Function], floor: int, pattern: str, minimum: int
) -> int:
    failures = 0
    total = sum(
        1
        for function in functions.values()
        for opcode in function.opcodes
        if SIMD_ARITH.match(opcode)
    )
    if total < floor:
        failures += 1
        print(
            f"FAIL simd floor: {total} f32x4.{{mul,add,sub}} < {floor}. This floor is a ratchet: "
            "a lower count is a regression to report, never a floor to lower.",
            file=sys.stderr,
        )
    kernel_re = re.compile(pattern)
    kernels = []
    for function in functions.values():
        if not kernel_re.search(function.name):
            continue
        vector = sum(1 for opcode in function.opcodes if KERNEL_VECTOR.match(opcode))
        if vector == 0:
            continue  # a vector-typed helper that does no arithmetic (drop glue, reset, stores)
        scalar = sum(1 for opcode in function.opcodes if KERNEL_SCALAR.match(opcode))
        kernels.append((function, vector, scalar))
        if vector <= scalar:
            failures += 1
            print(
                f"FAIL kernel {function.name}: vector={vector} scalar={scalar} "
                "(a vector instantiation must use strictly more vector than scalar arithmetic)",
                file=sys.stderr,
            )
    if len(kernels) < minimum:
        failures += 1
        print(
            f"FAIL kernel count: {len(kernels)} arithmetic-carrying functions match {pattern!r} "
            f"< {minimum}. If a wave moved the kernels, update the pattern and RAISE the floor.",
            file=sys.stderr,
        )
    print(f"simd: f32x4_arith={total} kernels={len(kernels)} pattern={pattern!r}")
    for function, vector, scalar in sorted(kernels, key=lambda row: -row[1]):
        print(f"  kernel vector={vector} scalar={scalar} {function.name}")
    return failures


VALID_SHAPE = """\
000010 func[0] <miso_engine_web_v1_render>:
 000011: 10 01                      | call 1 <render_next>
 000012: 0b                         | end
000020 func[1] <render_next>:
 000021: fd e6 01                   | f32x4.mul
 000022: fd e4 01                   | f32x4.add
 000023: fd e5 01                   | f32x4.sub
 000024: 0b                         | end
"""


def self_test() -> int:
    failures = 0

    def expect(label: str, condition: bool) -> None:
        nonlocal failures
        if not condition:
            failures += 1
            print(f"self-test FAILED: {label}", file=sys.stderr)

    # (f) the valid shape passes both gates.
    functions = parse(VALID_SHAPE)
    expect("(f) valid shape callgraph", check_callgraph(functions, "miso_engine_web_v1_render") == 0)
    expect(
        "(f) valid shape simd",
        check_simd(functions, 3, "render_next", 1) == 0,
    )

    # (a) a free reachable from the render export fails.
    freeing = VALID_SHAPE.replace(
        "000020 func[1] <render_next>:",
        " 000013: 10 02                      | call 2 <x>\n"
        "000020 func[1] <render_next>:",
    ) + "000030 func[2] <_ZN8dlmalloc4free17h0E>:\n 000031: 0b                         | end\n"
    expect("(a) dlmalloc free", check_callgraph(parse(freeing), "miso_engine_web_v1_render") == 1)

    # (b) an `unreachable` in a function that is neither a panic entry nor allow-listed fails.
    trapping = VALID_SHAPE.replace(
        " 000024: 0b                         | end",
        " 000024: 00                         | unreachable\n"
        " 000025: 0b                         | end",
    )
    expect("(b) trap owner", check_callgraph(parse(trapping), "miso_engine_web_v1_render") == 1)

    # (b1) issue #137: `--trap-owner` admits the named owner and nothing else, and it never
    # relaxes the allocation half of the gate.
    expect(
        "(b1) trap-owner admits the named owner",
        check_callgraph(parse(trapping), "miso_engine_web_v1_render", ("render_next",)) == 0,
    )
    expect(
        "(b1) trap-owner does not admit a different owner",
        check_callgraph(parse(trapping), "miso_engine_web_v1_render", ("some_other_name",)) == 1,
    )
    expect(
        "(b1) trap-owner never admits a free",
        check_callgraph(parse(freeing), "miso_engine_web_v1_render", ("render_next",)) == 1,
    )

    # (b1b) `--allocation-only` drops the trap half and keeps the allocation half.
    expect(
        "(b1b) allocation-only ignores a trap owner",
        check_callgraph(parse(trapping), "miso_engine_web_v1_render", (), True) == 0,
    )
    expect(
        "(b1b) allocation-only still fails a free",
        check_callgraph(parse(freeing), "miso_engine_web_v1_render", (), True) == 1,
    )

    # (b2) the same `unreachable` inside the allow-listed core function passes.
    allowed = trapping.replace("<render_next>", "<_ZN18PreparedRenderPlan12render_innerE>").replace(
        "call 1 <render_next>", "call 1 <_ZN18PreparedRenderPlan12render_innerE>"
    )
    expect(
        "(b2) allow-listed trap owner",
        check_callgraph(parse(allowed), "miso_engine_web_v1_render") == 0,
    )

    # (b3) a panic entry is a member but never a reported trap owner, and its callees are not
    # followed -- otherwise the abort runtime's own free would fail every artifact.
    entry = (
        VALID_SHAPE.replace(
            " 000024: 0b                         | end",
            " 000024: 10 02                      | call 2 <slice_index_fail>\n"
            " 000025: 0b                         | end",
        )
        + "000030 func[2] <_ZN4core9panicking16slice_index_failE>:\n"
        " 000031: 00                         | unreachable\n"
        " 000032: 10 03                      | call 3 <free>\n"
        "000040 func[3] <_ZN8dlmalloc4freeE>:\n"
        " 000041: 0b                         | end\n"
    )
    expect("(b3) panic entry is a leaf", check_callgraph(parse(entry), "miso_engine_web_v1_render") == 0)

    # (c) one op below the simd floor fails.
    expect("(c) simd floor", check_simd(parse(VALID_SHAPE), 4, "render_next", 1) == 1)

    # (d) a kernel whose scalar arithmetic reaches its vector arithmetic fails (de-vectorisation).
    scalarized = VALID_SHAPE.replace(
        " 000024: 0b                         | end",
        " 000024: 94                         | f32.mul\n"
        " 000025: 94                         | f32.mul\n"
        " 000026: 94                         | f32.mul\n"
        " 000027: 0b                         | end",
    )
    expect("(d) scalar dominates kernel", check_simd(parse(scalarized), 3, "render_next", 1) == 1)

    # (d3) a vector-typed helper with no arithmetic is not counted as a kernel.
    helper = VALID_SHAPE + (
        "000030 func[2] <_RINv_soft_clip16write_lane_words4wide6f32x4E>:\n"
        " 000031: 0b                         | end\n"
    )
    expect(
        "(d3) arithmetic-free helper is not a kernel",
        check_simd(parse(helper), 3, "render_next|write_lane_words", 2) == 1,
    )

    # (d2) too few kernel functions fails.
    expect("(d2) kernel count", check_simd(parse(VALID_SHAPE), 3, "render_next", 2) == 1)

    # (e) a missing name section is refused rather than silently passing.
    try:
        parse("000010 func[0]:\n 000011: 0b                         | end\n")
    except SystemExit:
        pass
    else:
        failures += 1
        print("self-test FAILED: (e) missing name section", file=sys.stderr)

    if failures:
        return 1
    print("web AudioWorklet call-graph analyser self-test passed")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--callgraph", metavar="EXPORT")
    parser.add_argument("--trap-owner", action="append", default=[], metavar="SUBSTRING")
    parser.add_argument("--allocation-only", action="store_true")
    parser.add_argument("--simd-floor", type=int)
    parser.add_argument("--kernel-pattern")
    parser.add_argument("--kernel-min", type=int)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        return self_test()
    if args.callgraph is None and args.simd_floor is None:
        parser.error("one of --callgraph, --simd-floor or --self-test is required")

    functions = parse(sys.stdin.read())
    failures = 0
    if args.callgraph is not None:
        if args.allocation_only and args.trap_owner:
            parser.error("--allocation-only and --trap-owner state opposite intents")
        failures += check_callgraph(
            functions, args.callgraph, tuple(args.trap_owner), args.allocation_only
        )
    if args.simd_floor is not None:
        if args.kernel_pattern is None or args.kernel_min is None:
            parser.error("--simd-floor requires --kernel-pattern and --kernel-min")
        failures += check_simd(
            functions, args.simd_floor, args.kernel_pattern, args.kernel_min
        )
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
