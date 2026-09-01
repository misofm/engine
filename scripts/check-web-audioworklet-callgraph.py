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

`--kernel-shape --kernel-pattern REGEX --kernel-min K`
    Assert the artifact still computes in the vector family. Three rules, none of which is a raw
    op-count minimum:

    1. **Roster presence.** Each of the eight named kernels in `KERNEL_ROSTER` -- the
       `process_bank`/`process_section`/`process_block` bodies of the shipped effect library --
       must match exactly one arithmetic-carrying function. A kernel that vanished, that was
       renamed, or that de-vectorised so completely it stopped carrying `f32x4` arithmetic at all
       fails here.
    2. **Per-kernel scalar budget (the shape gate).** Each roster kernel's scalar
       `f32.{mul,add,sub,div}` count must satisfy `scalar <= max(ceiling * vector, SCALAR_SLACK)`,
       where `ceiling` is that kernel's roster entry. The rule is a *shape*: it is scale free in
       the vector count, so halving a kernel's op count leaves it exactly as compliant as it was,
       while moving arithmetic out of the vector family and into the scalar family fails it.
    3. **Kernel count.** At least `K` functions matching `REGEX` carry `f32x4.{mul,add,sub,div}`
       arithmetic, each using strictly more vector than scalar `f32` arithmetic. `K` is a
       **ratchet**: when a wave adds kernels, raise it. It never drops.

    ### Why this replaced the raw `--simd-floor N` total (issue #163 phase 0e)

    The old gate asserted "at least N `f32x4.{mul,add,sub}` instructions in the whole module", most
    recently `N = 3450`. That proxy conflates two different events. Scalarising a kernel -- the
    failure the gate exists to catch -- lowers the count. Reducing a polynomial's degree, refitting
    a minimax approximation, or hoisting an invariant out of an inner loop *also* lowers the count,
    and those are the exact optimisations the floor pass exists to perform. Under a raw total the
    second is indistinguishable from the first, so the gate red-lights work it should wave through
    and the only available response is to lower the floor -- which the floor's own comment forbids.

    The property actually wanted is not "many vector instructions" but "each kernel still does its
    arithmetic in the vector family". Rules 1 and 2 state exactly that and nothing else. Rule 2 is
    the discriminating one: de-vectorising a `Lane`-over-`wide` body at `f32x4` turns one vector
    operation into four scalar ones, so a scalarised kernel's scalar-to-vector ratio does not drift
    -- it inverts. A degree reduction moves `vector` down with `scalar` fixed, which the ceiling
    absorbs by construction because it is expressed as a multiple of `vector`.

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

# The smallest scalar budget any kernel gets, in instructions.
#
# Four of the eight roster kernels currently emit *zero* scalar `f32` arithmetic, so a ceiling
# expressed purely as a multiple of `vector` would be exactly zero for them: a single scalar
# coefficient load introduced by an ordinary refactor would fail the gate. Eight instructions is
# well under the ~4x explosion de-vectorisation produces even in the smallest roster kernel
# (soft-clip, 25 vector operations -> ~100 scalar), so the slack cannot hide a scalarisation.
SCALAR_SLACK = 8

# The named kernel bodies of the shipped effect library, with each one's scalar budget.
#
# Each row is `(label, name pattern, scalar-to-vector ceiling)`. The pattern must match exactly one
# arithmetic-carrying kernel in the artifact; two matches or none is a failure, because a roster
# that silently stopped naming a kernel is a gate that silently stopped checking one.
#
# ## Derivation of the ceilings (issue #163 phase 0e; re-derived at mono-collapse M2)
#
# Measured with this analyser on `miso-engine-v1-audio-worklet.simd128.wasm` built from this tree
# (vector = `f32x4.{mul,add,sub,div}`, scalar = `f32.{mul,add,sub,div}`):
#
#   multiband-compressor f32x8   2560 / 20   ratio 0.0078
#   multiband-compressor f32x4   1280 / 20   ratio 0.0156
#   transient-shaper     f32x4    786 / 72   ratio 0.0916
#   true-peak-limiter    f32x4    448 /  0   ratio 0        (dual)
#   compressor           f32x4    267 /  0   ratio 0        (dual)
#   true-peak-limiter    f32x4    224 /  0   ratio 0        (collapsed)
#   gate-expander        f32x4    180 /  8   ratio 0.0444
#   parametric-eq        f32x4    168 /  0   ratio 0        (dual)
#   compressor           f32x4    138 /  0   ratio 0        (collapsed)
#   parametric-eq        f32x4     84 /  0   ratio 0        (collapsed)
#   soft-clip            f32x4     25 /  0   ratio 0
#
# Each ceiling is **four times the measured ratio, floored at 0.10**. Four times, because that is
# the factor by which a *partial* de-vectorisation would have to stay below to escape: converting
# one `f32x4` operation to scalar at width four costs one vector operation and buys four scalar
# ones, so even a single scalarised inner statement moves the ratio by far more than 4x its
# starting value in every kernel here. Floored at 0.10, because a ratio of zero admits no budget
# at all and the zero-scalar kernels would then fail on a stray coefficient move.
#
# The counts above are deliberately **not** asserted. They are the derivation's input, not the
# gate: a kernel is free to halve them, and the previous `--simd-floor 3450` total is exactly the
# assertion this note replaces. What is asserted is that no kernel's arithmetic migrates out of
# the vector family.
#
# ## What mono-collapse M2 did to this roster, and why the answer is three more rows
#
# The collapse gave the compressor, the true-peak limiter and the parametric EQ a **second** block
# body each: a one-plane variant a bank chain runs when every lane of its cohort is
# collapse-eligible. All three survive monomorphisation as their own symbols, so
# `compressor.*4wide6f32x4` went from one match to two and the roster failed exactly as
# it is designed to -- "two matches is a failure" is not a nuisance here, it is the rule noticing
# that the artifact grew a kernel.
#
# The fix is to name the new kernels, not to loosen the patterns. Each row below pins a specific
# body: the v0 mangling carries a length prefix (`12process_bank` against `17process_bank_mono`,
# `13process_block` against `18process_block_mono`), so a dual and a collapsed pattern cannot drift
# onto each other. The collapsed bodies are held to the same shape rule as the dual ones, which is
# the point: a one-plane body that de-vectorised would be a collapse that made the browser *slower*
# while still rendering the right bits, and no digest gate in the tree could see it.
#
# **The separation is itself a requirement, and it was measured.** M2 first wrote the two bodies as
# one function behind a `mono: bool`, and the shipped *dual* path got slower -- the
# `sixty_four_track_eq_only` console row, which never collapses, moved 28% against its sealed
# number. The bodies are now split by a const generic, and this table is where that shows: the EQ
# reads 168 + 84 where the merged form read one symbol at 252. Two matches per effect is therefore
# the healthy state, and a future change that merges them back would show up here as a row that
# vanished, not as a row that grew.
#
# Each collapsed body carries about **half** its dual sibling's vector arithmetic (138 against 267,
# 224 against 448, 84 against 168) and zero scalar arithmetic, which is what a correct one-plane
# variant looks like from here.
KERNEL_ROSTER: tuple[tuple[str, str, float], ...] = (
    ("multiband-compressor f32x8", r"multiband_compressor.*4wide6f32x8", 0.10),
    ("multiband-compressor f32x4", r"multiband_compressor.*4wide6f32x4", 0.10),
    ("transient-shaper f32x4", r"transient_shaper.*4wide6f32x4", 0.38),
    ("gate-expander f32x4", r"gate_expander.*4wide6f32x4", 0.19),
    ("compressor f32x4 dual", r"compressor6kernel13process_block.*4wide6f32x4", 0.10),
    (
        "compressor f32x4 collapsed",
        r"compressor6kernel18process_block_mono.*4wide6f32x4",
        0.10,
    ),
    (
        "true-peak-limiter f32x4 dual",
        r"true_peak_limiter.*11LimiterCore.*4wide6f32x4.*13process_block",
        0.10,
    ),
    (
        "true-peak-limiter f32x4 collapsed",
        r"true_peak_limiter.*27PreparedTruePeakLimiterBank.*4wide6f32x4"
        r".*17process_bank_mono",
        0.10,
    ),
    (
        "parametric-eq f32x4 dual",
        r"parametric_eq.*4wide6f32x4.*12process_bank",
        0.10,
    ),
    (
        "parametric-eq f32x4 collapsed",
        r"parametric_eq.*4wide6f32x4.*17process_bank_mono",
        0.10,
    ),
    ("soft-clip f32x4", r"soft_clip.*4wide6f32x4", 0.10),
)


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


def kernel_arithmetic(
    functions: dict[int, Function], pattern: str
) -> list[tuple[Function, int, int]]:
    """Every function matching `pattern` that carries vector arithmetic, with its op counts."""
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
    return kernels


def check_kernel_shape(
    functions: dict[int, Function],
    pattern: str,
    minimum: int,
    roster: tuple[tuple[str, str, float], ...] = KERNEL_ROSTER,
) -> int:
    """The per-kernel shape gate that replaced the raw `--simd-floor` total (#163 phase 0e)."""
    failures = 0
    kernels = kernel_arithmetic(functions, pattern)
    for function, vector, scalar in kernels:
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

    total = sum(
        1
        for function in functions.values()
        for opcode in function.opcodes
        if SIMD_ARITH.match(opcode)
    )
    print(f"kernel shape: f32x4_arith={total} kernels={len(kernels)} pattern={pattern!r}")

    for label, kernel_pattern, ceiling in roster:
        entry_re = re.compile(kernel_pattern)
        matched = [row for row in kernels if entry_re.search(row[0].name)]
        if len(matched) != 1:
            failures += 1
            print(
                f"FAIL roster {label}: {len(matched)} arithmetic-carrying kernels match "
                f"{kernel_pattern!r} (expected exactly one). A kernel that vanished, was renamed, "
                "or stopped carrying f32x4 arithmetic is a de-vectorisation, not a roster edit.",
                file=sys.stderr,
            )
            continue
        function, vector, scalar = matched[0]
        budget = max(ceiling * vector, float(SCALAR_SLACK))
        verdict = "ok"
        if scalar > budget:
            failures += 1
            verdict = "FAIL"
            print(
                f"FAIL roster {label}: vector={vector} scalar={scalar} "
                f"budget={budget:.1f} (ceiling {ceiling:g} x vector, slack {SCALAR_SLACK}). "
                "This kernel moved arithmetic out of the vector family. The budget is scale free "
                "in the vector count, so a genuine op-count reduction never trips it and lowering "
                "the ceiling is never the fix.",
                file=sys.stderr,
            )
        print(
            f"  roster {verdict:4s} vector={vector} scalar={scalar} budget={budget:.1f} "
            f"ceiling={ceiling:g} {label}"
        )
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

# The synthetic roster the self-test drives, matching `VALID_SHAPE`'s one kernel.
SELF_TEST_ROSTER: tuple[tuple[str, str, float], ...] = (("render-next", "render_next", 0.10),)


def synthetic_kernel(name: str, vector: int, scalar: int, index: int = 0) -> str:
    """One disassembled function body with exactly `vector` and `scalar` arithmetic operations."""
    lines = [f"{index:06x} func[{index}] <{name}>:"]
    for _ in range(vector):
        lines.append(" 000001: fd e6 01                   | f32x4.mul")
    for _ in range(scalar):
        lines.append(" 000002: 94                         | f32.mul")
    lines.append(" 000003: 0b                         | end")
    return "\n".join(lines) + "\n"


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
        "(f) valid shape kernel shape",
        check_kernel_shape(functions, "render_next", 1, SELF_TEST_ROSTER) == 0,
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

    # (c) the #163 phase 0e shape gate. Its whole reason for existing is that (c1) and (c2) below
    # have to land on opposite verdicts, which the raw `--simd-floor` total it replaced could not
    # do: both of them lower the module's vector instruction count.
    #
    # (c1) a roster kernel that de-vectorised -- vector operations traded for scalar ones -- fails
    # its scalar budget. The shape here still satisfies the old "vector strictly dominates scalar"
    # rule (100 > 40), so this case is precisely what the roster budget adds: a kernel can lose a
    # third of its arithmetic to the scalar family while still "dominating", and that is a
    # scalarisation.
    partly_scalarised = parse(synthetic_kernel("render_next", vector=100, scalar=40))
    expect(
        "(c1) partly de-vectorised roster kernel",
        check_kernel_shape(
            partly_scalarised, "render_next", 1, (("partly", "render_next", 0.10),)
        )
        == 1,
    )
    expect(
        "(c1) and the roster is what caught it",
        check_kernel_shape(partly_scalarised, "render_next", 1, ()) == 0,
    )
    # (c2) the same kernel with its vector op count *halved* and its scalar count untouched -- a
    # polynomial-degree reduction, a minimax refit, a hoisted loop invariant -- passes. The budget
    # is a multiple of `vector`, so it scales down with the kernel instead of red-lighting it.
    expect(
        "(c2) halved vector count still passes",
        check_kernel_shape(
            parse(synthetic_kernel("render_next", vector=381, scalar=72)),
            "render_next",
            1,
            (("transient-shaper", "render_next", 0.38),),
        )
        == 0,
    )
    # (c3) a roster entry that names no kernel in the artifact fails: a kernel that vanished or was
    # renamed must be a red gate, never a silently skipped row.
    expect(
        "(c3) roster entry matching nothing",
        check_kernel_shape(parse(VALID_SHAPE), "render_next", 1, (("absent", "no_such", 0.10),))
        == 1,
    )
    # (c4) an ambiguous roster entry fails too, because the gate would otherwise check whichever
    # of the two matches it happened to pick.
    expect(
        "(c4) ambiguous roster entry",
        check_kernel_shape(
            parse(synthetic_kernel("render_next_a", vector=40, scalar=0, index=0)
                  + synthetic_kernel("render_next_b", vector=40, scalar=0, index=1)),
            "render_next",
            1,
            (("ambiguous", "render_next", 0.10),),
        )
        == 1,
    )
    # (c5) the SCALAR_SLACK floor is a floor, not a hole: a zero-scalar kernel may acquire a
    # handful of scalar coefficient moves, and may not acquire a scalarised inner loop.
    expect(
        "(c5) slack admits a handful of scalar ops",
        check_kernel_shape(
            parse(synthetic_kernel("render_next", vector=25, scalar=SCALAR_SLACK)),
            "render_next",
            1,
            (("soft-clip", "render_next", 0.10),),
        )
        == 0,
    )
    expect(
        "(c5) slack does not admit one more",
        check_kernel_shape(
            parse(synthetic_kernel("render_next", vector=25, scalar=SCALAR_SLACK + 1)),
            "render_next",
            1,
            (("soft-clip", "render_next", 0.10),),
        )
        == 1,
    )

    # (d) a kernel whose scalar arithmetic reaches its vector arithmetic fails (de-vectorisation).
    scalarized = VALID_SHAPE.replace(
        " 000024: 0b                         | end",
        " 000024: 94                         | f32.mul\n"
        " 000025: 94                         | f32.mul\n"
        " 000026: 94                         | f32.mul\n"
        " 000027: 0b                         | end",
    )
    expect(
        "(d) scalar dominates kernel",
        check_kernel_shape(parse(scalarized), "render_next", 1, SELF_TEST_ROSTER) == 1,
    )

    # (d3) a vector-typed helper with no arithmetic is not counted as a kernel.
    helper = VALID_SHAPE + (
        "000030 func[2] <_RINv_soft_clip16write_lane_words4wide6f32x4E>:\n"
        " 000031: 0b                         | end\n"
    )
    expect(
        "(d3) arithmetic-free helper is not a kernel",
        check_kernel_shape(
            parse(helper), "render_next|write_lane_words", 2, SELF_TEST_ROSTER
        )
        == 1,
    )

    # (d2) too few kernel functions fails.
    expect(
        "(d2) kernel count",
        check_kernel_shape(parse(VALID_SHAPE), "render_next", 2, SELF_TEST_ROSTER) == 1,
    )

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
    parser.add_argument("--kernel-shape", action="store_true")
    parser.add_argument("--kernel-pattern")
    parser.add_argument("--kernel-min", type=int)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        return self_test()
    if args.callgraph is None and not args.kernel_shape:
        parser.error("one of --callgraph, --kernel-shape or --self-test is required")

    functions = parse(sys.stdin.read())
    failures = 0
    if args.callgraph is not None:
        if args.allocation_only and args.trap_owner:
            parser.error("--allocation-only and --trap-owner state opposite intents")
        failures += check_callgraph(
            functions, args.callgraph, tuple(args.trap_owner), args.allocation_only
        )
    if args.kernel_shape:
        if args.kernel_pattern is None or args.kernel_min is None:
            parser.error("--kernel-shape requires --kernel-pattern and --kernel-min")
        failures += check_kernel_shape(functions, args.kernel_pattern, args.kernel_min)
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
