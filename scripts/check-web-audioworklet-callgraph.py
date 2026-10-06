#!/usr/bin/env python3
"""Static call-graph and opcode gates over the shipped browser AudioWorklet artifact.

Issue #106 evals E1, E2 and E5. The input is `wasm-objdump -d NAMED_TWIN` on stdin: the gate reads
the emitted code of the build that ships, not an rlib, a debug build or the Rust source, because
"the render path never frees" is a property of the emitted code and nothing else can witness it.
It finds functions by name, and the shipped module carries none (issue #1109), so it reads the
build's named twin, which `strip-wasm-names.py check` proves is the shipped module plus its `name`
section and nothing else: `check-web-audioworklet.sh` runs that check before this gate.

Modes
-----
`--callgraph EXPORT [--trap-owner SUBSTRING ...]`
    Walk the direct-call closure of `EXPORT` and fail if any member's name matches `FORBIDDEN`
    (allocator, deallocator or drop glue). Then list the closure members that contain an
    `unreachable` instruction and fail unless that set equals `TRAP_ALLOW_LIST` plus any
    `--trap-owner` substrings the caller named.

    A C allocator name (any name containing `free`, `alloc`, `memalign` or `sbrk`) fails anywhere
    in an *unmangled* member name and never in a mangled Rust name (issues #1234 and #1417). A
    Rust v0 (`_R`) name is a Rust item. A `_ZN` prefix is Itanium C++ mangling, which Rust's legacy
    scheme and C++ namespaced names share: the rule assumes the worklet links no C++, so a `_ZN`
    name in it is a Rust legacy item, and a C++ dependency would need this rule revisited. An
    out-of-line accessor whose mangled name ends in `4free` is an ordinary method named `free`,
    not the allocator. The Rust allocator's own symbols on `wasm32-unknown-unknown` are matched by
    the other alternatives (`dlmalloc`, `dealloc`, `__rust_alloc`, `__rust_realloc`) or through
    their dlmalloc callees: `__rdl_alloc`, `__rdl_alloc_zeroed` and `__rdl_realloc` match no
    alternative themselves and fail because each calls dlmalloc's `malloc` or `memalign`. Any
    other name is C or a `#[no_mangle]` symbol, and a C allocator is not always spelled with one
    of the four bare names: `dlfree`, `__libc_malloc`, `mi_free` and `je_malloc` are prefixed,
    and `posix_memalign`, `aligned_alloc`, `memalign`, jemalloc's `sdallocx` and `rallocx`,
    mimalloc's `mi_zalloc` and snmalloc's `sn_rust_alloc` contain none of them. A C-backed
    `#[global_allocator]` whose `__rust_*` shim LTO inlined would reach such a name with no other
    alternative matching; on an over-aligned path it can call `posix_memalign` with no `malloc`
    beside it. So any unmangled name that contains `free`, `alloc`, `memalign` or `sbrk` fails.
    This fails safe: an unmangled engine function whose name matches fails too, and the fix is to
    rename it, not to relax the rule. This checker never sees an import: an import has no body,
    so `wasm-objdump -d` prints no function header for it, and `check-web-audioworklet.sh`
    refuses a module with any import before this gate runs. `scripts/test-web-audioworklet.sh`
    runs `--self-test`, which pins these cases.

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

`--callgraph EXPORT --render-thread`
    Issue #1333 D4: three more rules over the same direct-call closure, for the exports the
    AudioWorklet calls on its render thread. They keep decision 15's rule ("the worklet never
    allocates or frees after boot") visible once the module is built with atomics (#1332).

    1. **Thread-local destructor registration.** A member whose name carries std's registration
       path -- `thread_local` + `destructors` + `register`, or `thread_local` + `guard` +
       `enable` -- fails. With atomics std registers a destructor for a thread local that needs
       drop, lazily and by allocating; on today's non-atomic module no such path exists.
    2. **Atomic wait.** A member containing `memory.atomic.wait32` or `memory.atomic.wait64`
       fails: render never blocks. Today's module carries no atomic opcode at all
       (`check-web-audioworklet.sh` bans them outright), so this rule is in place before #1332.
    3. **Pinned `call_indirect` sites.** The direct-call closure cannot see what an indirect call
       reaches -- the whole plan executor sits behind `Box<dyn PreparedPlanExecutor>` -- and
       resolving indirect calls by signature is not sound (it flags drop glue the render path
       never runs), so the runtime render-locked allocation count is the proof through them.
       What this rule holds is the *set* of indirect sites: each closure member that contains
       `call_indirect`, by its name without the mangling hash, with its count, must equal
       `INDIRECT_SITES[EXPORT]`. A new, removed or changed site fails and prints the difference,
       so new dynamic dispatch on the render path is reviewed, with its reason, in this table.

`--kernel-shape --kernel-pattern REGEX --kernel-min K`
    Assert the artifact still computes in the vector family, and in four lanes only. Four rules,
    none of which is a raw op-count minimum:

    1. **Roster presence.** Each of the ten named kernels in `KERNEL_ROSTER` -- the
       `process_bank`/`process_section`/`process_block` bodies of the shipped effect library and
       the live route's indexed-ramp mix (#1220); the eleventh, the collapsed limiter's, is held
       by the forwarding rule below -- must match exactly
       one arithmetic-carrying function. A kernel that vanished, that was renamed, or that
       de-vectorised so completely it stopped carrying `f32x4` arithmetic at all fails here.
    2. **Per-kernel scalar budget (the shape gate).** Each roster kernel's scalar
       `f32.{mul,add,sub,div}` count must satisfy `scalar <= max(ceiling * vector, SCALAR_SLACK)`,
       where `ceiling` is that kernel's roster entry. The rule is a *shape*: it is scale free in
       the vector count, so halving a kernel's op count leaves it exactly as compliant as it was,
       while moving arithmetic out of the vector family and into the scalar family fails it.
    3. **Kernel count.** At least `K` functions matching `REGEX` carry `f32x4.{mul,add,sub,div}`
       arithmetic, each using strictly more vector than scalar `f32` arithmetic. `K` is a
       **ratchet**: when a wave adds kernels, raise it. It never drops.
    4. **No eight lanes (issue #1110).** No function name in the module may spell an eight-lane
       instantiation (`EIGHT_LANE`). The browser runs four lanes only, and since #1110 the wasm
       build has no eight-lane type, so a name that carries one means eight lanes came back to
       `wasm32` -- code no browser can execute, which is what #1110 removed.

    The collapsed true-peak limiter is a forwarding entry plus a distinct arithmetic kernel. The
    roster therefore requires exactly one `PreparedTruePeakLimiterBank<f32x4>::process_bank_mono`
    entry, exactly one arithmetic-bearing `LimiterCore<f32x4>::process_block_mono` kernel, and a
    direct call between them. The entry must carry no counted vector or scalar arithmetic; the
    kernel receives the same dominance, scalar-budget and slack checks as every other row.

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
    Synthetic disassembly cases (a)-(h) below, each the red mutation of one rule.

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
    r"^(?!_R|_ZN).*(?:free|alloc|memalign|sbrk)"
    r"|dealloc|dlmalloc|drop_glue|drop_in_place|drop_slow|unlink_chunk"
    r"|insert_large_chunk|memory_grow|__rust_alloc|__rust_realloc"
)
# The one non-entry trap owner that may remain, with the reason it is unreachable in production:
# `PreparedRenderPlan::render_inner` inlines `PlanarBufferMut::plane_mut`, whose `&mut
# self.storage[start..end]` is a checked slice index, in the executor-less silence branch. Every
# web plan carries an executor (both graph binding paths end in `prepare_with_executor`), so the
# branch is dead. It is core-owned (#84) and is not fixed from this job.
TRAP_ALLOW_LIST = frozenset({"18PreparedRenderPlan12render_inner"})

# Issue #1333 D4.1: std's thread-local destructor registration path, as substrings that must all
# appear in one member name. `std::sys::thread_local::destructors::list::register` registers a
# destructor; `std::sys::thread_local::guard::*::enable` arms the per-thread run of them.
TLS_DESTRUCTOR_REGISTRATION: tuple[tuple[str, ...], ...] = (
    ("thread_local", "destructors", "register"),
    ("thread_local", "guard", "enable"),
)
# Issue #1333 D4.2: the two blocking waits a render-thread closure may never contain.
ATOMIC_WAIT = frozenset({"memory.atomic.wait32", "memory.atomic.wait64"})
# The mangling hash, stripped so a pinned site survives a rebuild: the legacy scheme's trailing
# `17h<16 hex>E`, and the v0 scheme's crate disambiguator `Cs<base62>_`.
LEGACY_HASH = re.compile(r"17h[0-9a-f]{16}E$")
V0_CRATE_HASH = re.compile(r"Cs[0-9A-Za-z]*_")

# Issue #1333 D4.3: the pinned `call_indirect` sites of each render-thread export's direct-call
# closure, by member name without its mangling hash, with the count and the reason for each.
# Measured on the named twin of the artifact this table was set with; a change to any entry is a
# review of new dynamic dispatch on the render thread, never a re-pin without its reason.
INDIRECT_SITES: dict[str, dict[str, tuple[int, str]]] = {
    "miso_engine_web_v1_render": {
        "_RNvMs2_NtNtCs_6engine8realtime4planNtB5_18PreparedRenderPlan6render": (
            1,
            "`executor.invalidate_observers()` on a failed block, through the plan's "
            "`Box<dyn PreparedPlanExecutor>`",
        ),
        "_RNvMs2_NtNtCs_6engine8realtime4planNtB5_18PreparedRenderPlan12render_inner": (
            1,
            "`executor.render(...)`: the whole plan executor, through the plan's "
            "`Box<dyn PreparedPlanExecutor>`; the runtime render-locked count covers it",
        ),
    },
    "miso_engine_web_v1_meter_poll": {},
    "miso_engine_web_v1_command_submit": {
        "_RNvMNtCs_9host_core16live_route_stateNtB2_14LiveRouteState6follow": (
            2,
            "the `effective_mute: &dyn Fn(usize, usize) -> bool` predicate, once per lane, when a "
            "route follows its source strip's mute",
        ),
        "_RNvNtCs_9host_core16live_route_state14followed_lanes": (
            2,
            "the same `effective_mute: &dyn Fn` predicate, once per lane",
        ),
        "_RNvMs0_NtCs_15effect_compiler7controlNtB5_18EffectControlOwner4edit": (
            1,
            "`factory.descriptor()` on the owner's `Arc<dyn NativeEffectFactory>`",
        ),
        "_RNvMs2_NtCs_15effect_compiler7prepareNtB5_21EffectControlProducer25publish_candidate_targets": (
            4,
            "the owner's `Arc<dyn NativeEffectFactory>`: `target_preparation()` and the "
            "capability's target-count and validation calls, inlined from `publish`",
        ),
        "_RNvMs2_NtCs_15effect_compiler7prepareNtB5_21EffectControlProducer27preflight_candidate_targets": (
            4,
            "the same factory and target-preparation capability calls, inlined from "
            "`preflight_publication`",
        ),
    },
}

VECTOR = re.compile(r"^(v128|i8x16|i16x8|i32x4|i64x2|f32x4|f64x2)\.")
SIMD_ARITH = re.compile(r"^f32x4\.(mul|add|sub)$")
KERNEL_VECTOR = re.compile(r"^f32x4\.(mul|add|sub|div)$")
KERNEL_SCALAR = re.compile(r"^f32\.(mul|add|sub|div)$")

# An eight-lane instantiation, as a function name spells it (issue #1110). The `wide` eight-lane
# types -- `f32x8` and the `f64x8`, `u32x8` and `i32x8` it widens and masks to -- are what every
# eight-lane kernel, bank or stage is generic over, so their mangled names carry them
# (`4wide6f32x8_5f32x8`); `simd8` and `transpose_tile_8` are the lane crate's and the bank tile's
# own spellings. The module at `7d030945` carried 29 such functions, every one naming `f32x8`.
# A const-generic `8` (`Kj8_`) is not listed: that module's 19 all name `f32x8` too, and an
# eight-element array is not an eight-lane kernel.
EIGHT_LANE = re.compile(r"(?:f32|f64|u32|i32)x8|(?i:simd8)|transpose_tile_8")

# The smallest scalar budget any kernel gets, in instructions.
#
# Eight of the eleven roster kernels -- the ten `KERNEL_ROSTER` rows and the collapsed limiter's
# kernel -- currently emit *zero* scalar `f32` arithmetic, so a ceiling
# expressed purely as a multiple of `vector` would be exactly zero for them: a single scalar
# coefficient load introduced by an ordinary refactor would fail the gate. Eight instructions is
# well under the ~4x explosion de-vectorisation produces even in the smallest roster kernel
# (the route ramp mix, 21 vector operations -> ~84 scalar), so the slack cannot hide a
# scalarisation.
SCALAR_SLACK = 8

# The named arithmetic kernel bodies of the shipped effect library, with each one's scalar budget.
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
#   (multiband-compressor f32x8 was 2560 / 20; #1110 removed eight lanes from the browser build)
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
#   route-mix-ramp       f32x4     22 /  0   ratio 0        (#1220; row added by its verdict NIT-1)
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
# ## What mono-collapse M2 did to this roster, and why the limiter has a forwarding rule
#
# The collapse gave the compressor, the true-peak limiter and the parametric EQ a **second** block
# body each: a one-plane variant a bank chain runs when every lane of its cohort is
# collapse-eligible. All three survive monomorphisation as their own symbols, so
# `compressor.*4wide6f32x4` went from one match to two and the roster failed exactly as
# it is designed to -- "two matches is a failure" is not a nuisance here, it is the rule noticing
# that the artifact grew a kernel.
#
# The fix is to name the new kernels, not to loosen the patterns. The compressor and EQ rows below
# pin their arithmetic bodies directly: the v0 mangling carries a length prefix (`12process_bank`
# against `17process_bank_mono`, `13process_block` against `18process_block_mono`), so a dual and a
# collapsed pattern cannot drift onto each other. The limiter's collapsed entry is different: LLVM
# outlines its arithmetic into `LimiterCore::process_block_mono`, so its entry/kernel identity and
# direct edge are checked by `check_limiter_forwarding` below. Both forms receive the same shape
# rule, which is the point: a one-plane body that de-vectorised would be a collapse that made the
# browser slower while still rendering the right bits, and no digest gate in the tree could see it.
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
    # #1220 verdict NIT-1: the live route's indexed-ramp mix. Its sub-vector frames are outlined
    # (`route_mix_ramp_tail`, `route_mix_settled_tail`); an inlined settled tail unrolls as 18
    # scalar operations beside its 21 vector ones (22 before the #1220 amendment built each chunk's
    # frame index from a `u32` counter), which the generic "vector > scalar" rule admits and this
    # row's budget (max(0.10 x 21, slack 8) = 8) refuses.
    (
        "route-mix-ramp f32x4",
        r"lane7kernels20route_mix_ramp_block.*4wide6f32x4",
        0.10,
    ),
)

LIMITER_MONO_ENTRY_PATTERN = (
    r"true_peak_limiter.*27PreparedTruePeakLimiterBank.*4wide6f32x4"
    r".*17process_bank_mono"
)
LIMITER_MONO_KERNEL_PATTERN = (
    r"true_peak_limiter.*11LimiterCore.*4wide6f32x4"
    r".*18process_block_mono"
)
LIMITER_MONO_LABEL = "true-peak-limiter f32x4 collapsed"
LIMITER_MONO_CEILING = 0.10


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
                    "name section required: func[%s] has no <name>; a module without names "
                    "blinds this gate, and the shipped one has none (#1109), so give it the "
                    "build's named twin" % header.group(2)
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
    # An exact name wins: `miso_engine_web_v1_render` is a prefix of
    # `miso_engine_web_v1_render_allocation_count` (issue #1333), and an export's name-section
    # entry is its exact symbol.
    roots = [function for function in functions.values() if function.name == export]
    if not roots:
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


def unhashed(name: str) -> str:
    """`name` without its mangling hash, so a pinned site survives an unrelated rebuild."""
    return V0_CRATE_HASH.sub("Cs_", LEGACY_HASH.sub("E", name))


def indirect_sites(members: list[Function]) -> dict[str, int]:
    """Each member that contains `call_indirect`, by unhashed name, with its total count."""
    sites: dict[str, int] = {}
    for function in members:
        count = function.opcodes.count("call_indirect")
        if count:
            name = unhashed(function.name)
            sites[name] = sites.get(name, 0) + count
    return sites


def check_render_thread(
    functions: dict[int, Function],
    export: str,
    pinned: dict[str, dict[str, tuple[int, str]]] | None = None,
) -> int:
    """Issue #1333 D4: destructor registration, atomic waits and pinned indirect sites."""
    table = INDIRECT_SITES if pinned is None else pinned
    members = closure(functions, export)
    failures = 0
    registering = sorted(
        function.name
        for function in members
        if any(all(part in function.name for part in parts) for parts in TLS_DESTRUCTOR_REGISTRATION)
    )
    if registering:
        failures += 1
        print(
            f"FAIL {export}: the render-thread closure registers a thread-local destructor:",
            file=sys.stderr,
        )
        for name in registering:
            print(f"  {name}", file=sys.stderr)
    waiting = sorted(function.name for function in members if ATOMIC_WAIT.intersection(function.opcodes))
    if waiting:
        failures += 1
        print(f"FAIL {export}: the render-thread closure contains an atomic wait:", file=sys.stderr)
        for name in waiting:
            print(f"  {name}", file=sys.stderr)
    if export not in table:
        failures += 1
        print(f"FAIL {export}: no pinned call_indirect table for this export", file=sys.stderr)
        return failures
    expected = {name: count for name, (count, _reason) in table[export].items()}
    actual = indirect_sites(members)
    if actual != expected:
        failures += 1
        print(
            f"FAIL {export}: the render-thread call_indirect sites differ from INDIRECT_SITES:",
            file=sys.stderr,
        )
        for name in sorted(set(expected) | set(actual)):
            if expected.get(name) != actual.get(name):
                print(
                    f"  {name}: pinned {expected.get(name, 0)}, found {actual.get(name, 0)}",
                    file=sys.stderr,
                )
    print(
        f"{export}: render-thread indirect_sites={sum(actual.values())} "
        f"members={len(actual)} destructor_registration={len(registering)} "
        f"atomic_waits={len(waiting)}"
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


def check_limiter_forwarding(functions: dict[int, Function]) -> int:
    """Check the collapsed limiter's zero-arithmetic entry and direct arithmetic kernel edge."""
    failures = 0
    entry_re = re.compile(LIMITER_MONO_ENTRY_PATTERN)
    entries = [function for function in functions.values() if entry_re.search(function.name)]
    if len(entries) != 1:
        failures += 1
        print(
            f"FAIL roster {LIMITER_MONO_LABEL} entry: {len(entries)} functions match "
            f"{LIMITER_MONO_ENTRY_PATTERN!r} (expected exactly one)",
            file=sys.stderr,
        )

    kernels = kernel_arithmetic(functions, LIMITER_MONO_KERNEL_PATTERN)
    if len(kernels) != 1:
        failures += 1
        print(
            f"FAIL roster {LIMITER_MONO_LABEL} kernel: {len(kernels)} arithmetic-carrying "
            f"functions match {LIMITER_MONO_KERNEL_PATTERN!r} (expected exactly one)",
            file=sys.stderr,
        )

    if len(entries) != 1 or len(kernels) != 1:
        return failures

    entry = entries[0]
    kernel, vector, scalar = kernels[0]
    if kernel.index not in entry.calls:
        failures += 1
        print(
            f"FAIL roster {LIMITER_MONO_LABEL}: entry {entry.name} does not directly call "
            f"selected kernel {kernel.name}",
            file=sys.stderr,
        )

    entry_vector = sum(1 for opcode in entry.opcodes if KERNEL_VECTOR.match(opcode))
    entry_scalar = sum(1 for opcode in entry.opcodes if KERNEL_SCALAR.match(opcode))
    if entry_vector or entry_scalar:
        failures += 1
        print(
            f"FAIL roster {LIMITER_MONO_LABEL} entry {entry.name}: forwarding entry carries "
            f"counted arithmetic vector={entry_vector} scalar={entry_scalar}; expected 0/0",
            file=sys.stderr,
        )

    if vector <= scalar:
        failures += 1
        print(
            f"FAIL kernel {kernel.name}: vector={vector} scalar={scalar} "
            "(a vector instantiation must use strictly more vector than scalar arithmetic)",
            file=sys.stderr,
        )
    budget = max(LIMITER_MONO_CEILING * vector, float(SCALAR_SLACK))
    verdict = "ok"
    if scalar > budget:
        failures += 1
        verdict = "FAIL"
        print(
            f"FAIL roster {LIMITER_MONO_LABEL}: vector={vector} scalar={scalar} "
            f"budget={budget:.1f} (ceiling {LIMITER_MONO_CEILING:g} x vector, slack "
            f"{SCALAR_SLACK}). The selected kernel moved arithmetic out of the vector family.",
            file=sys.stderr,
        )
    print(
        f"  roster {verdict:4s} entry=0/0 direct={'yes' if kernel.index in entry.calls else 'no'} "
        f"vector={vector} scalar={scalar} budget={budget:.1f} "
        f"ceiling={LIMITER_MONO_CEILING:g} {LIMITER_MONO_LABEL}"
    )
    return failures


def check_no_eight_lanes(functions: dict[int, Function]) -> int:
    """Rule 4: no function name spells an eight-lane instantiation (issue #1110)."""
    eight = sorted(
        function.name for function in functions.values() if EIGHT_LANE.search(function.name)
    )
    if eight:
        print(
            f"FAIL eight lanes: {len(eight)} functions name an eight-lane instantiation. The "
            "browser runs four lanes only and the wasm build has none (#1110); this is code no "
            "browser can execute.",
            file=sys.stderr,
        )
        for name in eight:
            print(f"  {name}", file=sys.stderr)
        return 1
    print(f"eight lanes: none among {len(functions)} functions")
    return 0


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


SELF_TEST_LIMITER_ENTRY = (
    "true_peak_limiter27PreparedTruePeakLimiterBank4wide6f32x417process_bank_mono"
)
SELF_TEST_LIMITER_KERNEL = "true_peak_limiter11LimiterCore4wide6f32x418process_block_mono"


def synthetic_limiter(
    *,
    entry_name: str = SELF_TEST_LIMITER_ENTRY,
    kernel_name: str = SELF_TEST_LIMITER_KERNEL,
    direct: bool = True,
    entry_vector: int = 0,
    entry_scalar: int = 0,
    kernel_vector: int = 25,
    kernel_scalar: int = 0,
) -> str:
    """Build an entry/kernel pair for the independent forwarding-rule controls."""
    lines = [f"000010 func[1] <{entry_name}>:"]
    if direct:
        lines.append(f" 000011: 10 02                      | call 2 <{kernel_name}>")
    for _ in range(entry_vector):
        lines.append(" 000012: fd e6 01                   | f32x4.mul")
    for _ in range(entry_scalar):
        lines.append(" 000013: 94                         | f32.mul")
    lines.append(" 000014: 0b                         | end")
    lines.append(f"000020 func[2] <{kernel_name}>:")
    for _ in range(kernel_vector):
        lines.append(" 000021: fd e6 01                   | f32x4.mul")
    for _ in range(kernel_scalar):
        lines.append(" 000022: 94                         | f32.mul")
    lines.append(" 000023: 0b                         | end")
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

    # (a1) issues #1234 and #1417: a C allocator name fails anywhere in an unmangled name and
    # never in a mangled Rust name.
    def reaching(name: str) -> dict[int, Function]:
        return parse(freeing.replace("<_ZN8dlmalloc4free17h0E>", f"<{name}>"))

    # An out-of-line Rust accessor that is merely *named* like a C allocator is not the
    # allocator, for each of the four names (v0 mangling) and for legacy mangling.
    for length, c_name in ((4, "free"), (6, "malloc"), (6, "calloc"), (7, "realloc")):
        expect(
            f"(a1) out-of-line v0 accessor named {c_name} passes",
            check_callgraph(
                reaching(f"_RNvMs_NtCs0_5graphNtB4_25GraphRouteControlProducer{length}{c_name}"),
                "miso_engine_web_v1_render",
            )
            == 0,
        )
    expect(
        "(a1) out-of-line legacy accessor named free passes",
        check_callgraph(
            reaching("_ZN5graph25GraphRouteControlProducer4free17h0123456789abcdefE"),
            "miso_engine_web_v1_render",
        )
        == 0,
    )
    # A member named exactly like a C allocator entry point fails, each of the four.
    for c_name in ("free", "malloc", "calloc", "realloc"):
        expect(
            f"(a1) bare C allocator {c_name}",
            check_callgraph(reaching(c_name), "miso_engine_web_v1_render") == 1,
        )
    # A prefixed C or third-party allocator spelling fails: a C allocator is not always bare.
    for c_name in (
        "dlfree",
        "__libc_free",
        "mi_free",
        "mi_malloc_aligned",
        "je_malloc",
        "tlsf_free",
        "__libc_calloc",
        "je_realloc",
    ):
        expect(
            f"(a1) prefixed C allocator {c_name}",
            check_callgraph(reaching(c_name), "miso_engine_web_v1_render") == 1,
        )
    # A C allocator entry point that contains none of the four bare names fails (Amendment 1 of
    # #1417): aligned and sized entry points, and allocators whose names end in `alloc`.
    for c_name in (
        "posix_memalign",
        "aligned_alloc",
        "memalign",
        "sdallocx",
        "rallocx",
        "mi_zalloc",
        "sn_rust_alloc",
        "sbrk",
    ):
        expect(
            f"(a1) C allocator entry point {c_name}",
            check_callgraph(reaching(c_name), "miso_engine_web_v1_render") == 1,
        )
    # An unmangled name that contains a C allocator name is refused, wherever the name sits: it
    # may be a C allocator (`free_list`, `malloc_usable`), and an unmangled engine function so
    # named is renamed rather than admitted.
    for c_name in ("free", "malloc", "calloc", "realloc"):
        expect(
            f"(a1) unmangled {c_name}_count fails: an unmangled name containing a C allocator name",
            check_callgraph(reaching(f"{c_name}_count"), "miso_engine_web_v1_render") == 1,
        )
    # The Rust allocator's symbols still fail, each through its own alternative: the mangled
    # `__rust_realloc` shim is exempt from the C alternative, so only `__rust_realloc` matches it.
    for rust_name in ("_ZN8dlmalloc4free17h0E", "_RNvCs0_7___rustc14___rust_realloc"):
        expect(
            f"(a1) Rust allocator {rust_name}",
            check_callgraph(reaching(rust_name), "miso_engine_web_v1_render") == 1,
        )

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

    # (c6) the collapsed limiter is an exact entry -> arithmetic-kernel pair. Each mutation below
    # is independent so a broad name match or an indirect edge cannot accidentally satisfy the rule.
    expect(
        "(c6) forwarding entry and kernel pass",
        check_limiter_forwarding(parse(synthetic_limiter())) == 0,
    )
    expect(
        "(c6a) forwarding entry absent",
        check_limiter_forwarding(parse(synthetic_limiter(entry_name="other_forwarding"))) == 1,
    )
    ambiguous_entry = synthetic_limiter() + (
        f"000030 func[3] <{SELF_TEST_LIMITER_ENTRY}>:\n"
        " 000031: 10 02                      | call 2 <kernel>\n"
        " 000032: 0b                         | end\n"
    )
    expect(
        "(c6b) forwarding entry ambiguous",
        check_limiter_forwarding(parse(ambiguous_entry)) == 1,
    )
    expect(
        "(c6c) arithmetic kernel absent",
        check_limiter_forwarding(parse(synthetic_limiter(kernel_name="other_kernel"))) == 1,
    )
    ambiguous_kernel = synthetic_limiter() + synthetic_kernel(
        SELF_TEST_LIMITER_KERNEL, vector=25, scalar=0, index=3
    )
    expect(
        "(c6d) arithmetic kernel ambiguous",
        check_limiter_forwarding(parse(ambiguous_kernel)) == 1,
    )
    expect(
        "(c6e) forwarding entry must directly call kernel",
        check_limiter_forwarding(parse(synthetic_limiter(direct=False))) == 1,
    )
    expect(
        "(c6f) forwarding entry scalar arithmetic",
        check_limiter_forwarding(parse(synthetic_limiter(entry_scalar=1))) == 1,
    )
    expect(
        "(c6g) selected kernel scalar budget",
        check_limiter_forwarding(parse(synthetic_limiter(kernel_scalar=SCALAR_SLACK + 1))) == 1,
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

    # (g) issue #1110: a function name that spells an eight-lane instantiation fails, whichever of
    # the spellings it uses, while a crate hash that merely contains `x8` -- the base module had
    # three -- does not.
    expect("(g) no eight lanes in the valid shape", check_no_eight_lanes(parse(VALID_SHAPE)) == 0)
    for label, name in (
        ("f32x8", "_RNvMs3_NtCs0_4rack5stageINtB2_5StageNtNtCs1_4wide6f32x8_5f32x8E7process"),
        ("u32x8", "_RNvNtCs0_4lane4mask8from_u32NtNtCs1_4wide6u32x8_5u32x8"),
        ("simd8", "_RNvNtNtCs0_4lane5simd84load"),
        ("transpose_tile_8", "_RNvNtCs0_15effect_contract16transpose_tile_8"),
    ):
        expect(
            f"(g) eight-lane name {label}",
            check_no_eight_lanes(parse(VALID_SHAPE + synthetic_kernel(name, 4, 0, index=2))) == 1,
        )
    hash_only = VALID_SHAPE + synthetic_kernel(
        "_RNvMs3_NtCs5yx8Jh2iHKX_9once_cell4race8once_box4init", 0, 0, index=2
    )
    expect(
        "(g) a hash containing x8 is not eight lanes", check_no_eight_lanes(parse(hash_only)) == 0
    )

    # (h) issue #1333 D4: the render-thread rules. A synthetic pinned table for `VALID_SHAPE`'s
    # render export, with one indirect site in `render_next`.
    def render_thread(text: str, table: dict[str, dict[str, tuple[int, str]]]) -> int:
        return check_render_thread(parse(text), "miso_engine_web_v1_render", table)

    indirect_line = " 000025: 11 00 00                   | call_indirect 0 0\n"
    dispatching = VALID_SHAPE.replace(
        " 000024: 0b                         | end", indirect_line + " 000024: 0b                         | end"
    )
    pinned_one = {"miso_engine_web_v1_render": {"render_next": (1, "self-test executor dispatch")}}
    pinned_none: dict[str, dict[str, tuple[int, str]]] = {"miso_engine_web_v1_render": {}}
    expect("(h) pinned indirect site passes", render_thread(dispatching, pinned_one) == 0)
    expect("(h) no indirect site against an empty pin passes", render_thread(VALID_SHAPE, pinned_none) == 0)
    # (h1) a new `call_indirect` in a render-thread closure fails.
    expect("(h1) new indirect site", render_thread(dispatching, pinned_none) == 1)
    # (h1a) a removed pinned site fails: the table must say what the closure does.
    expect("(h1a) removed indirect site", render_thread(VALID_SHAPE, pinned_one) == 1)
    # (h1b) a changed count fails.
    doubled = dispatching.replace(indirect_line, indirect_line + indirect_line)
    expect("(h1b) changed indirect count", render_thread(doubled, pinned_one) == 1)
    # (h1c) an export with no pinned table fails rather than passing unchecked.
    expect("(h1c) unpinned export", render_thread(dispatching, {}) == 1)
    # (h1d) the mangling hash is not part of the pinned name, in either scheme.
    for hashed, pinned_name in (
        ("_ZN4plan6render17h0123456789abcdefE", "_ZN4plan6renderE"),
        ("_RNvCs5yx8Jh2iHKX_4plan6render", "_RNvCs_4plan6render"),
    ):
        renamed = dispatching.replace("<render_next>", f"<{hashed}>").replace(
            "call 1 <render_next>", f"call 1 <{hashed}>"
        )
        expect(
            f"(h1d) hash-free pin of {hashed}",
            render_thread(renamed, {"miso_engine_web_v1_render": {pinned_name: (1, "x")}}) == 0,
        )
    # (h2) a member that registers a thread-local destructor fails, by either std path.
    for registrar in (
        "_ZN3std3sys12thread_local11destructors4list8register17h0123456789abcdefE",
        "_ZN3std3sys12thread_local5guard3key6enable17h0123456789abcdefE",
    ):
        registering = VALID_SHAPE.replace(
            "000020 func[1] <render_next>:",
            " 000013: 10 02                      | call 2 <x>\n000020 func[1] <render_next>:",
        ) + f"000030 func[2] <{registrar}>:\n 000031: 0b                         | end\n"
        expect(f"(h2) destructor registration {registrar}", render_thread(registering, pinned_none) == 1)
    # A thread-local accessor that registers nothing is not a registration.
    accessor = VALID_SHAPE.replace(
        "000020 func[1] <render_next>:",
        " 000013: 10 02                      | call 2 <x>\n000020 func[1] <render_next>:",
    ) + "000030 func[2] <_ZN3std6thread5local17LocalKey$LT$T$GT$4with17h0123456789abcdefE>:\n"
    accessor += " 000031: 0b                         | end\n"
    expect("(h2) a thread-local accessor passes", render_thread(accessor, pinned_none) == 0)
    # (h3) a member containing an atomic wait fails, either width; a notify is not a wait.
    for opcode, verdict in (
        ("memory.atomic.wait32 2 0", 1),
        ("memory.atomic.wait64 3 0", 1),
        ("memory.atomic.notify 2 0", 0),
    ):
        waiting = VALID_SHAPE.replace(
            " 000024: 0b                         | end",
            f" 000025: fe 01 02 00                | {opcode}\n 000024: 0b                         | end",
        )
        expect(f"(h3) {opcode.split()[0]}", render_thread(waiting, pinned_none) == verdict)

    # (h4) an export whose name prefixes another export's resolves to the exact one:
    # `miso_engine_web_v1_render` prefixes `miso_engine_web_v1_render_allocation_count`.
    prefixed = VALID_SHAPE + (
        "000040 func[3] <miso_engine_web_v1_render_allocation_count>:\n"
        " 000041: 0b                         | end\n"
    )
    try:
        resolved = check_callgraph(parse(prefixed), "miso_engine_web_v1_render")
    except SystemExit:
        resolved = -1
    expect("(h4) exact export name wins over a prefix match", resolved == 0)

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
    parser.add_argument("--render-thread", action="store_true")
    parser.add_argument("--kernel-shape", action="store_true")
    parser.add_argument("--kernel-pattern")
    parser.add_argument("--kernel-min", type=int)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        return self_test()
    if args.render_thread and args.callgraph is None:
        parser.error("--render-thread requires --callgraph")
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
        if args.render_thread:
            failures += check_render_thread(functions, args.callgraph)
    if args.kernel_shape:
        if args.kernel_pattern is None or args.kernel_min is None:
            parser.error("--kernel-shape requires --kernel-pattern and --kernel-min")
        failures += check_kernel_shape(functions, args.kernel_pattern, args.kernel_min)
        failures += check_limiter_forwarding(functions)
        failures += check_no_eight_lanes(functions)
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
