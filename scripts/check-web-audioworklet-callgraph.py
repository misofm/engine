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
`--callgraph EXPORT [--trap-owner SUBSTRING=SITES ...]`
    Walk the direct-call closure of `EXPORT` and fail if any member's name matches `FORBIDDEN`
    (allocator, deallocator or drop glue). Then list the closure members that contain an
    `unreachable` instruction and fail unless every one of them matches a `--trap-owner`
    substring the caller named; there is no built-in allow list (issue #2068). Every owner is
    pinned, `SUBSTRING=SITES`, and a bare `SUBSTRING` is refused (#2068): its matching members
    together must hold exactly `SITES` `unreachable` instructions, so a new checked index inlined
    into it is red rather than admitted with the old one (issue #1581).

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
    rename it, not to relax the rule. This checker never sees an imported function: an import has
    no body, so `wasm-objdump -d` prints no function header for it, and `check-web-audioworklet.sh`
    refuses a module with any import but its one shared `env.memory` (#1380) before this gate
    runs. `scripts/test-web-audioworklet.sh`
    runs `--self-test`, which pins these cases.

    `--trap-owner` never relaxes the allocation half of the gate, which is the half that matters:
    an allocator, a deallocator or drop glue in the closure is a failure no matter what. It exists
    because issue #137 put more exports on the render thread -- `miso_engine_web_v1_meter_poll`
    among them -- whose bodies inline the bounded SPSC endpoints' own checked slot indexing. Naming the export's own symbol keeps the strong statement "nothing
    this export *calls* may trap" while admitting the one checked index the queue primitive emits.

`--callgraph EXPORT --render-thread`
    Issue #1333 D4: three more rules over the same direct-call closure, for the exports the
    AudioWorklet calls on its render thread. They keep decision 15's rule ("the worklet never
    allocates or frees after boot") visible once the module is built with atomics (#1332).

    1. **Thread-local destructor registration.** A member whose name carries std's registration
       path -- `thread_local` + `destructors` + `register`, or `thread_local` + `guard` +
       `enable` -- fails. With atomics (the shipped build since #1380) std registers a destructor
       for a thread local that needs drop, lazily and by allocating.
    2. **Atomic wait.** A member containing `memory.atomic.wait32` or `memory.atomic.wait64`
       fails: render never blocks. Since #1380 the module is built with atomics on a shared
       memory, so std's lock, `Once` and the data-segment initialiser carry waits; this rule is what
       keeps them out of every render-thread closure.
    3. **Pinned `call_indirect` sites.** The direct-call closure cannot see what an indirect call
       reaches -- the whole plan executor sits behind `Box<dyn PreparedPlanExecutor>` -- and
       resolving indirect calls by signature is not sound (it flags drop glue the render path
       never runs), so the runtime render-locked allocation count is the proof through them.
       What this rule holds is the *set* of indirect sites: each closure member that contains
       `call_indirect`, by its name without the mangling hash, with its count, must equal
       `INDIRECT_SITES[EXPORT]`. A new, removed or changed site fails and prints the difference,
       so new dynamic dispatch on the render path is reviewed, with its reason, in this table.

`--kernel-shape --kernel-pattern REGEX --kernel-min K`
    Assert the artifact still computes in the vector family, and in four lanes only. Five rules,
    none of which is a raw op-count minimum:

    1. **Roster presence.** Each of the nineteen named kernels in `KERNEL_ROSTER` -- the
       `process_bank`/`process_section`/`process_block`/`run_block`/`render` bodies of the shipped
       effect library and the live route's indexed-ramp mix (#1220); the twentieth, the collapsed
       limiter's, is held
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
       **ratchet**: when a wave adds kernels, raise it. It never drops. The one exception is a
       control-thread function pinned in `CONTROL_THREAD_EXEMPT`, by its name without the mangling
       hash and with its reason: it is neither held to dominance nor counted. An entry that names
       no function in the module fails, so the table cannot outlive its function. The render
       export reaches its kernels through `call_indirect`, so the direct-call closure cannot tell
       a render kernel from control code; a reviewed table can (#877).
    4. **No eight lanes (issue #1110).** No function name in the module may spell an eight-lane
       instantiation (`EIGHT_LANE`). The browser runs four lanes only, and since #1110 the wasm
       build has no eight-lane type, so a name that carries one means eight lanes came back to
       `wasm32` -- code no browser can execute, which is what #1110 removed.
    5. **Pinned render kernels (issue #1889).** Rule 3 skips a function with no `f32x4`
       arithmetic as a helper, so a counted kernel that a compiler or source change scalarises in
       full would leave the count silently while `K` still holds. `RENDER_KERNELS` names every
       render kernel rule 3 counts, by its name without the mangling hash and with the stage it
       renders. Each entry must resolve to exactly one function over the whole module (pattern
       or not, so a rename away from the pattern cannot hide it) and that function must carry
       `f32x4` arithmetic; zero matches, more than one, or `vector == 0` fails. A counted kernel
       in neither this table nor `CONTROL_THREAD_EXEMPT` fails too. A vanished entry
       is answered by a reviewed table edit with evidence that its arithmetic moved into a kernel
       the table or roster still holds, never by deleting the row silently.

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
        # Issue #1381b: the block that adopted a plan hands the plan's tables over together with
        # what the swap block's attachment hook paired, which it reads from the active plan's
        # companions. #1332 S5 moved the same four sites, by inlining only: the host's
        # `render_next` now copies a render failure's diagnostic after the render half's call, so
        # `RenderHalf::render_next` and `RenderReady::adopt_shape` are no longer inlined into it;
        # the sites and their count are unchanged (4 before, 2 + 2 after).
        "_RNvMs2_Cs_8host_webNtB5_11RenderReady11adopt_shape": (
            2,
            "`PlanState::active_attachment_mut::<RenderCompanions>()` at the adoption block "
            "(#1381b): the active plan's `Box<dyn HostAttachment>` answers `as_any_mut` and the "
            "downcast asks its `type_id`, one `call_indirect` each, neither of which allocates or "
            "drops",
        ),
        "_RNvMs3_Cs_8host_webNtB5_10RenderHalf11render_next": (
            2,
            "the same pair in the meter fold that runs after the block while the protocol's meter "
            "table is on without the lease (#1588b, `poll_meter_delivery`, inlined)",
        ),
        # Issue #1381a D4: the browser renders through the plan owner, so the block boundary's
        # swap decision is in the closure. The plan's own `render` entry is not: the owner calls
        # `render_inner` and counts a failed block itself. The `Ms7` impl index moved from `Ms6`
        # when #1396 added `impl PlanClocks` above `impl RealtimePlanOwner`; the sites and their
        # count are unchanged (owner-approved pin edit, 2026-10-16).
        "_RNvMs7_NtNtCs_6engine8realtime13plan_exchangeNtB5_17RealtimePlanOwner11enter_block": (
            4,
            "`running.prime_ready(...)`, the running plan's readiness hook, asked for a `Primed` "
            "candidate only (#1311), and `carry_from`'s executor hand-over at an adopted swap "
            "(#1273), both through the plan's `Box<dyn PreparedPlanExecutor>`; and, at the same "
            "adopted swap, `carry_from`'s attachment hand-over (#1381 D3: "
            "`HostAttachment::adopt_predecessor(...)` through the plan's `Box<dyn HostAttachment>`, "
            "which pairs the host's readers with the observers the executors carried, #1327) and "
            "its read of the successor executor's `observer_carries()` record (#1327 D3); the "
            "runtime render-locked count covers them",
        ),
        "_RNvMs7_NtNtCs_6engine8realtime13plan_exchangeNtB5_17RealtimePlanOwner22copy_response_snapshot": (
            1,
            "#1577: the response capture copies the plan's snapshot through the executor trait "
            "object (`Box<dyn PreparedPlanExecutor>`), once per armed job; no allocation, no drop",
        ),
        "_RNvMs4_NtNtCs_6engine8realtime4planNtB5_18PreparedRenderPlan12render_inner": (
            2,
            "`executor.begin_live_block(snapshot)` (#1502 D2: hands the block's live snapshot to "
            "the executor) and `executor.render(...)` (the whole plan executor), both through the "
            "plan's `Box<dyn PreparedPlanExecutor>`; the runtime render-locked count covers them",
        ),
    },
    # Issue #1332 S6: the handoff exports dispatch nothing dynamically. Each one's closure is its
    # own body (the registry scan, the box exchange and the canary compare all inline), with no
    # allocator, free or drop glue: a refusal path forgets or redeposits what it holds.
    "miso_engine_web_v1_host_adopt": {},
    "miso_engine_web_v1_host_release": {},
    # The worklet's bind and render-diagnostic exports: loads, compares and one CAS.
    "miso_engine_web_v1_instance_bind": {},
    "miso_engine_web_v1_render_diagnostic_ptr": {},
    "miso_engine_web_v1_render_diagnostic_bytes": {},
    # Issue #1332 S7 (R-S7-meters): the worker-mode `process()` reads the current meter header's
    # address after `render()`; a borrow and a pointer, no dynamic dispatch.
    "miso_engine_web_v1_meter_header_ptr": {},
    "miso_engine_web_v1_meter_poll": {
        # #1332 S5: `RenderHalf::poll_meters` has a second caller (`RenderHost::poll_meters`), so
        # it is no longer inlined into the host's; the same two sites, by inlining only.
        "_RNvMs3_Cs_8host_webNtB5_10RenderHalf11poll_meters": (
            2,
            "`PlanState::active_attachment_mut::<RenderCompanions>()` (#1381a D3): the active "
            "plan's `Box<dyn HostAttachment>` answers `as_any_mut` and the downcast asks its "
            "`type_id`, one `call_indirect` each, neither of which allocates or drops",
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
# Sixteen of the twenty roster kernels -- the nineteen `KERNEL_ROSTER` rows and the collapsed
# limiter's kernel -- currently emit *zero* scalar `f32` arithmetic, so a ceiling
# expressed purely as a multiple of `vector` would be exactly zero for them: a single scalar
# coefficient load introduced by an ordinary refactor would fail the gate. Eight instructions is
# well under the ~4x explosion de-vectorisation produces even in the smallest roster kernel
# (the route ramp mix, 22 vector operations -> ~88 scalar), so the slack cannot hide a
# scalarisation.
SCALAR_SLACK = 8

# The named arithmetic kernel bodies of the shipped effect library, with each one's scalar budget.
#
# Each row is `(label, name pattern, scalar-to-vector ceiling)`. The pattern must match exactly one
# arithmetic-carrying kernel in the artifact; two matches or none is a failure, because a roster
# that silently stopped naming a kernel is a gate that silently stopped checking one.
#
# ## Derivation of the ceilings (issue #163 phase 0e; re-derived at mono-collapse M2, re-measured at #1970)
#
# Measured with this analyser on the named twin of `miso-engine-v1-audio-worklet.simd128.wasm`
# (`build-web-audioworklet.sh --named-twin`) built from `15f37828f`, issue #1970
# (vector = `f32x4.{mul,add,sub,div}`, scalar = `f32.{mul,add,sub,div}`; the count this table
# gave before #1970 is in brackets where it moved):
#
#   (multiband-compressor f32x8 was 2560 / 20; #1110 removed eight lanes from the browser build)
#   multiband-compressor f32x4   3464 /  0   ratio 0        (dual; 2527 / 0 before #1367a's mixed arm)
#   multiband-compressor f32x4   1862 /  0   ratio 0        (collapsed; 1389 / 0 before #1367a)
#   transient-shaper     f32x4    808 /  0   ratio 0        (dual; 786 / 72 at M2, codegen; [700 / 0])
#   transient-shaper     f32x4    407 /  0   ratio 0        (collapsed, #1873; [352 / 0])
#   true-peak-limiter    f32x4   1139 /  0   ratio 0        (dual; [448 / 0])
#   compressor           f32x4   1359 /  0   ratio 0        (dual; [267 / 0])
#   true-peak-limiter    f32x4    537 /  0   ratio 0        (collapsed; [224 / 0])
#   gate-expander        f32x4    265 /  0   ratio 0        (dual; 180 / 8 before #973, codegen; [162 / 0])
#   parametric-eq        f32x4   1352 /  1   ratio 0.0007   (dual; [168 / 0])
#   compressor           f32x4    733 /  0   ratio 0        (collapsed; [138 / 0])
#   gate-expander        f32x4    134 /  0   ratio 0        (collapsed, #973; [82 / 0])
#   parametric-eq        f32x4    748 /  1   ratio 0.0013   (collapsed; [84 / 0])
#   soft-clip            f32x4     55 /  0   ratio 0        ([25 / 0])
#   route-mix-ramp       f32x4     22 /  0   ratio 0        (#1220; row added by its verdict NIT-1)
#   dynamic-eq           f32x4   1014 /  0   ratio 0        (dual, #17 S2; 626 / 0 before #1969)
#   dynamic-eq           f32x4    511 /  0   ratio 0        (collapsed, #17 S2; 316 / 0 before #1969)
#   de-esser             f32x4    893 /  0   ratio 0        (dual, #1920; 561 / 0 before #1969)
#   de-esser             f32x4    450 /  0   ratio 0        (collapsed, #1920; 283 / 0 before #1969)
#   reverb               f32x4    168 / 25   ratio 0.149    (#2114, `render::<_, true>`; ceiling 0.25, below; 84 / 25 with 16 lines)
#   reverb               f32x4    168 / 25   ratio 0.149    (#2114, `render::<_, false>`; ceiling 0.25)
#
# The reverb rows were measured on the named twin built from the #2114 attempt-5 tree (32 lines,
# on `9c6365f46`).
#
# The dynamic-eq and de-esser rows were re-measured after #1969 (its detector link glide), on the
# named twin built from `e3e7cdab3`; the other rows are the #1970 build's.
#
# The parametric EQ's one scalar operation in each body is an `f32.sub` once per block, not per
# sample: `lane::silence_armable_holding` (issue #1328) computes `armed_after - frames`, and both
# operands are splats (`silence_window`'s `silence_frames(rate)` and the block's frame count), so
# LLVM subtracts the two scalars and splats the result. 4 x 0.0007 and 4 x 0.0013 are below the
# floor, so both rows keep the 0.10 ceiling (budgets 135.2 and 74.8 against 1).
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
# body each, #973 gave the gate/expander one, #1871 the multiband compressor one and #1873 the
# transient shaper one: a one-plane
# variant a bank chain runs when every lane of its cohort is collapse-eligible. All six survive
# monomorphisation as their own symbols, so `compressor.*4wide6f32x4` (and at #973
# `gate_expander.*4wide6f32x4`; the multiband row was already pinned to its `process_bank` by #893)
# went from one match to two and the roster failed exactly as
# it is designed to -- "two matches is a failure" is not a nuisance here, it is the rule noticing
# that the artifact grew a kernel.
#
# The fix is to name the new kernels, not to loosen the patterns. The compressor, EQ and gate rows
# below pin their arithmetic bodies directly: the v0 mangling carries a length prefix
# (`12process_bank` against `17process_bank_mono`, `13process_block` against
# `18process_block_mono`), so a dual and a collapsed pattern cannot drift onto each other. The
# limiter's collapsed entry is different: LLVM outlines its arithmetic into `LimiterCore::process_block_mono`, so its entry/kernel identity and
# direct edge are checked by `check_limiter_forwarding` below. Both forms receive the same shape
# rule, which is the point: a one-plane body that de-vectorised would be a collapse that made the
# browser slower while still rendering the right bits, and no digest gate in the tree could see it.
#
# **The separation is itself a requirement, and it was measured.** M2 first wrote the two bodies as
# one function behind a `mono: bool`, and the shipped *dual* path got slower -- the
# `sixty_four_track_eq_only` console row, which never collapses, moved 28% against its sealed
# number. The bodies are now split by a const generic, and this table is where that shows: the EQ
# read 168 + 84 at M2 where the merged form read one symbol at 252. Two matches per effect is therefore
# the healthy state, and a future change that merges them back would show up here as a row that
# vanished, not as a row that grew.
#
# Each collapsed body carries about **half** its dual sibling's vector arithmetic (733 against 1359,
# 537 against 1139, 748 against 1352, 134 against 265, 1862 against 3464, 407 against 808, 316
# against 626, 283 against 561) and no more scalar arithmetic than its sibling, which is what a
# correct one-plane variant looks like from here.
KERNEL_ROSTER: tuple[tuple[str, str, float], ...] = (
    # #893: named by its `process_bank` body, as the EQ rows are, because the bank's
    # `advance_clocks` now carries one vector add of its own (the silence counters' advance).
    (
        "multiband-compressor f32x4",
        r"multiband_compressor.*4wide6f32x4.*12process_bank",
        0.10,
    ),
    (
        "multiband-compressor f32x4 collapsed",
        r"multiband_compressor.*4wide6f32x4.*17process_bank_mono",
        0.10,
    ),
    # #1368b: the per-lane link grew the bank's constructor (`Shaper::<f32x4>::new`), which LLVM
    # outlines with the follower coefficients' four control-side `f32x4.sub`; it is not a render
    # kernel, and the generic shape rule still checks it. The body went 522 -> 700 vector
    # operations, 0 scalar: the mixed-link arm. #1873: the block body is `#[inline(always)]` into
    # the bank's `process_bank` and `process_bank_mono`, so the rows pin those (700 / 0 and
    # 352 / 0); the ceiling is the derivation's floor, 4 x 0 floored at 0.10 (it was 0.38, four
    # times the 786 / 72 body's ratio).
    (
        "transient-shaper f32x4 dual",
        r"transient_shaper.*27PreparedTransientShaperBank.*4wide6f32x4.*12process_bank",
        0.10,
    ),
    (
        "transient-shaper f32x4 collapsed",
        r"transient_shaper.*27PreparedTransientShaperBank.*4wide6f32x4.*17process_bank_mono",
        0.10,
    ),
    ("gate-expander f32x4 dual", r"gate_expander.*4wide6f32x4.*12process_bank", 0.10),
    (
        "gate-expander f32x4 collapsed",
        r"gate_expander.*4wide6f32x4.*17process_bank_mono",
        0.10,
    ),
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
    # scalar operations beside its 22 vector ones (re-measured after #1452's undo 2 advanced the
    # frame index by a vector add again; 21 between the #1220 amendment and that undo), which the
    # generic "vector > scalar" rule admits and this row's budget (max(0.10 x 22, slack 8) = 8)
    # refuses.
    (
        "route-mix-ramp f32x4",
        r"lane7kernels20route_mix_ramp_block.*4wide6f32x4",
        0.10,
    ),
    # #17 S2: the dynamic EQ's block bodies are `run_block::<MONO>`'s inner closures (the bank's
    # `process_bank` and `process_bank_mono` only forward): `KB` is the back-reference to the
    # `false` const, `Kb1_` the literal `true`. Measured 626 / 0 and 316 / 0 on the named twin.
    (
        "dynamic-eq f32x4 dual",
        r"dynamic_eq.*9DynamicEq.*4wide6f32x4.*9run_blockKB",
        0.10,
    ),
    (
        "dynamic-eq f32x4 collapsed",
        r"dynamic_eq.*9DynamicEq.*4wide6f32x4.*9run_blockKb1_",
        0.10,
    ),
    # #1920: the de-esser has the dynamic EQ's shape, `run_block::<MONO>`'s inner closures behind
    # forwarding bank entries. Measured 561 / 0 and 283 / 0 on the named twin.
    (
        "de-esser f32x4 dual",
        r"de_esser.*7DeEsser.*4wide6f32x4.*9run_blockKB",
        0.10,
    ),
    (
        "de-esser f32x4 collapsed",
        r"de_esser.*7DeEsser.*4wide6f32x4.*9run_blockKb1_",
        0.10,
    ),
    # #2114 S4: the reverb's per-block render, `PreparedReverb::render::<L, FULL>`, kept out of
    # line (`#[inline(never)]`) so its symbol carries the lane type; inlined into `process` it
    # carried none and this gate never saw it. `Kb1_` is `FULL = true` (every ring word is valid
    # history, the steady state), `Kb0_` the filling form; both are 168 vector / 25 scalar (32
    # lines; 84 / 25 with 16).
    #
    # The scalar 25 is by design, not a scalarisation: every one of the 32 lines and eight
    # diffusers has its own length, so the ring reads, clear-on-read writes and cursor wraps are
    # per-line scalar work (the delay has no four-lane kernel for the same reason), and the
    # pre-delay crossfade blend, the two output taps, the width M/S matrix and the mix are
    # per-sample scalar arithmetic on two words. The vector 168 is the loop (two diffuser vectors,
    # the size crossfade's tap blend over eight vectors, eight vectors of low-pass/high-pass, the
    # 5-stage Hadamard butterfly, the gain ramp and injection).
    #
    # The ceiling is **not** four times the ratio here, and why: the derivation's "four times"
    # assumes a near-zero baseline, where one scalarised statement multiplies the ratio. With 16
    # lines (84 / 25, ratio 0.298) the smallest loop-wide scalarisation -- one butterfly stage or
    # one filter statement across the four vectors, 4 vector operations become 16 scalar -- read
    # 41 / 80 = 0.513, and 0.50 sat between and refused it. With 32 lines that scalarisation is 8
    # vector operations becoming 32 scalar: 57 / 160 = 0.356, which 0.50 would admit. By the same
    # rule (a ceiling between the measured ratio and the smallest loop-wide scalarisation) the
    # ceiling is 0.25 (budget 42), between 0.149 and 0.356 (#2114 attempt 5; a tightening).
    # The scalar part is fixed per sample rather than proportional to the vector part, so a
    # genuine cut of more than about two fifths of the vector operations with the scalar part
    # unchanged trips this row: re-measure it then, with the reason, rather than raising the
    # ceiling to absorb a scalarisation.
    (
        "reverb f32x4 full",
        r"reverb6kernel.*14PreparedReverb6render.*4wide6f32x4_5f32x4Kb1_",
        0.25,
    ),
    (
        "reverb f32x4 filling",
        r"reverb6kernel.*14PreparedReverb6render.*4wide6f32x4_5f32x4Kb0_",
        0.25,
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
) -> int:
    failures = 0
    pinned: dict[str, int] = {}
    for owner in trap_owners:
        name, separator, sites = owner.partition("=")
        if not separator or not name or not sites.isdigit():
            failures += 1
            print(
                f"FAIL {export}: --trap-owner {owner!r} is not SUBSTRING=SITES; "
                "an owner is admitted only at a pinned site count (#2068)",
                file=sys.stderr,
            )
            continue
        pinned[name] = int(sites)
    allowed_owners = frozenset(pinned)
    members = closure(functions, export)
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
    if unexpected:
        failures += 1
        print(f"FAIL {export}: unexpected trap owner in the render closure:", file=sys.stderr)
        for name in sorted(unexpected):
            print(f"  {name}", file=sys.stderr)
    for name, sites in sorted(pinned.items()):
        found = sum(
            function.opcodes.count("unreachable")
            for function in trap_owners
            if name in function.name
        )
        if found != sites:
            failures += 1
            print(
                f"FAIL {export}: trap owner {name} holds {found} trap sites, pinned at {sites}",
                file=sys.stderr,
            )
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


BASE62_INDEX = re.compile(r"[0-9A-Za-z]*_")
CONST_DATA = re.compile(r"n?[0-9a-f]*_")
BASIC_TYPE = frozenset("abcdefhijlmnostuvxyz")


def kernel_key(name: str) -> str:
    """`unhashed(name)` without the indices that move when unrelated code moves (#1889).

    A v0 symbol's impl disambiguator (`Ms4_`, `Xs9_`) numbers the impl blocks of its crate, and a
    back-reference (`B5_`) is a byte offset into the symbol, so both change when an unrelated impl
    block is added. The walk copies each length-prefixed identifier and each const's hex data
    verbatim (so a tag-like byte inside them is never touched), drops the disambiguator after
    `M`/`X`, and writes every back-reference as `B_`. A name that is not v0, or that the walk
    cannot follow, is returned as `unhashed` gives it. The key is less specific than the name, so
    a table entry must still resolve to exactly one function; a collision fails and names both.
    """
    name = unhashed(name)
    if not name.startswith("_R"):
        return name
    out = ["_R"]
    i = 2
    try:
        while i < len(name):
            c = name[i]
            if c.isdigit() or (c == "u" and i + 1 < len(name) and name[i + 1].isdigit()):
                start = i
                if c == "u":
                    i += 1
                j = i
                while name[j].isdigit():
                    j += 1
                length = int(name[i:j])
                if name[j] == "_":
                    j += 1
                if j + length > len(name):
                    return name
                out.append(name[start : j + length])
                i = j + length
            elif c in "MX" and name[i + 1] == "s":
                index = BASE62_INDEX.match(name, i + 2)
                if index is None:
                    return name
                out.append(c)
                i = index.end()
            elif c == "B":
                index = BASE62_INDEX.match(name, i + 1)
                if index is None:
                    return name
                out.append("B_")
                i = index.end()
            elif c == "K" and name[i + 1] in BASIC_TYPE:
                data = CONST_DATA.match(name, i + 2)
                if data is None:
                    return name
                out.append(name[i : data.end()])
                i = data.end()
            else:
                out.append(c)
                i += 1
    except IndexError:
        return name
    return "".join(out)


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


# Issue #877: control-thread functions whose lane-generic name matches the kernel pattern, by
# name without the mangling hash (`unhashed`), with the reason each is not a render kernel. A new
# entry is a review that no render path calls the function; an entry that matches nothing fails.
CONTROL_THREAD_EXEMPT: dict[str, str] = {
    "_RNvMCs_13parametric_eqINtB_7ChannelNtNtCs_4wide6f32x4_5f32x4Kj4_E12decode_trackB_": (
        "control-thread state decode; LLVM 22 partly vectorizes its ramp check (#877); no render "
        "path calls it"
    ),
    "_RNvMCs_16transient_shaperINtB_6ShaperNtNtCs_4wide6f32x4_5f32x4Kj4_E3newB_": (
        "transient shaper constructor, called only from `prepare` and the bank factory (#1889); "
        "its four coefficient products run once per plan, on the control thread"
    ),
}


# Issue #1889: the render kernels the f32x4 build must keep, by name without the mangling hash
# (`unhashed`), with the stage each renders. `kernel_arithmetic` skips a function with no `f32x4`
# arithmetic as a helper, so a kernel that LLVM or a source change scalarises in full would leave
# the K count with nothing red; rule 5 holds each entry present (exactly one function) and
# vector-carrying. Measured on the 1.99.1 named twin of shipped module 8e0e4e79 (main after
# #1922 and #1925): these are all the kernels rule 3 counts. A vanished entry is answered by a reviewed edit, with evidence
# that its arithmetic moved into a kernel this table or the roster still holds, never by
# deleting the row silently (the INDIRECT_SITES policy). A counted kernel that is neither here
# nor in `CONTROL_THREAD_EXEMPT` fails, so a change that adds a kernel adds its row.
RENDER_KERNELS: dict[str, str] = {
    "_RINvNvNtCs_5graph7runtime15bank_meter_pass5planeNtNtCs_4wide6f32x4_5f32x4EB_":
        "resident bank meter plane: seeded energy and optional held-peak hold/decay (#951)",
    "_RNvMCs_8builtinsINtB_10InputStageNtNtCs_4wide6f32x4_5f32x4E7processB_":
        "builtins input section (polarity/trim/HPF/LPF), stereo body",
    "_RNvMCs_8builtinsINtB_10InputStageNtNtCs_4wide6f32x4_5f32x4E12process_monoB_":
        "builtins input section, mono-collapsed body",
    "_RNvMCs_8builtinsINtB_14FaderRampStageNtNtCs_4wide6f32x4_5f32x4E13process_planeB_":
        "builtins fader/mute ramp plane, the builtins instance",
    "_RNvMCs_8builtinsINtB_14FaderRampStageNtNtCs_4wide6f32x4_5f32x4E13process_planeCs_17builtins_compiler":
        "builtins fader/mute ramp plane, the builtins_compiler instance",
    "_RINvNtCs_4lane7kernels19silence_skip_settleNtNtCs_4wide6f32x4_5f32x4ECs_8builtins":
        "silence-skip settle step, the builtins instance",
    "_RINvNtCs_4lane7kernels19silence_skip_settleNtNtCs_4wide6f32x4_5f32x4ECs_13parametric_eq":
        "silence-skip settle step, the parametric_eq instance",
    "_RINvNtCs_4lane7kernels19silence_skip_settleNtNtCs_4wide6f32x4_5f32x4ECs_20multiband_compressor":
        "silence-skip settle step, the multiband_compressor instance",
    "_RINvNtCs_4lane7kernels20route_mix_ramp_blockNtNtCs_4wide6f32x4_5f32x4ECs_5graph":
        "live route mix, one ramped route (#1220)",
    "_RINvNtCs_4lane7kernels28route_mix_column_ramps_blockNtNtCs_4wide6f32x4_5f32x4ECs_5graph":
        "live route mix, the indexed-ramp column body",
    # #2129: the per-node crossfade's body, behind `BypassShunt::crossfade` and `crossfade_held`.
    "_RINvNtCs_15effect_contract4live16crossfade_planesNtNtCs_4wide6f32x4_5f32x4ECs_5graph":
        "live bypass crossfade of a per-node insert",
    "_RINvMNtCs_15effect_contract4liveNtB_11BypassShunt14crossfade_bankNtNtCs_4wide6f32x4_5f32x4ECs_4rack":
        "live bypass crossfade of a banked console slot or insert",
    "_RNvXCs_13parametric_eqINtB_20PreparedParametricEqNtNtCs_4wide6f32x4_5f32x4Kj4_ENtCs_15effect_contract24PreparedNativeEffectBank12process_bankB_":
        "parametric EQ bank, stereo body",
    "_RNvXCs_13parametric_eqINtB_20PreparedParametricEqNtNtCs_4wide6f32x4_5f32x4Kj4_ENtCs_15effect_contract24PreparedNativeEffectBank17process_bank_monoB_":
        "parametric EQ bank, mono-collapsed body",
    "_RNvXCs_13parametric_eqINtB_20PreparedParametricEqNtNtCs_4wide6f32x4_5f32x4Kj4_ENtCs_15effect_contract24PreparedNativeEffectBank14advance_clocksB_":
        "parametric EQ ramp clocks on a skipped silent block",
    "_RNvMCs_13parametric_eqINtB_7ChannelNtNtCs_4wide6f32x4_5f32x4Kj4_E10rest_lanesB_":
        "parametric EQ rest test of the silence skip",
    "_RINvNtCs_10compressor6kernel13process_blockNtNtCs_4wide6f32x4_5f32x4EB_":
        "compressor kernel, stereo body",
    "_RINvNtCs_10compressor6kernel18process_block_monoNtNtCs_4wide6f32x4_5f32x4EB_":
        "compressor kernel, mono-collapsed body",
    "_RINvNtCs_10compressor6kernel17settled_sidechainNtNtCs_4wide6f32x4_5f32x4EB_":
        "compressor settled-sidechain block",
    "_RNvXCs_13gate_expanderINtB_12PreparedGateNtNtCs_4wide6f32x4_5f32x4Kb0_ENtCs_15effect_contract24PreparedNativeEffectBank12process_bank":
        "gate/expander bank, stereo body",
    "_RNvXCs_13gate_expanderINtB_12PreparedGateNtNtCs_4wide6f32x4_5f32x4Kb0_ENtCs_15effect_contract24PreparedNativeEffectBank17process_bank_mono":
        "gate/expander bank, mono-collapsed body",
    "_RNvMCs_17true_peak_limiterINtB_11LimiterCoreNtNtCs_4wide6f32x4_5f32x4E13process_blockB_":
        "true-peak limiter core, stereo body",
    "_RNvMCs_17true_peak_limiterINtB_11LimiterCoreNtNtCs_4wide6f32x4_5f32x4E18process_block_monoB_":
        "true-peak limiter core, mono-collapsed body (also held by check_limiter_forwarding)",
    "_RNvXCs_20multiband_compressorINtB_31PreparedMultibandCompressorBankNtNtCs_4wide6f32x4_5f32x4Kj4_ENtCs_15effect_contract24PreparedNativeEffectBank12process_bankB_":
        "multiband compressor bank, stereo body",
    "_RNvXCs_20multiband_compressorINtB_31PreparedMultibandCompressorBankNtNtCs_4wide6f32x4_5f32x4Kj4_ENtCs_15effect_contract24PreparedNativeEffectBank17process_bank_monoB_":
        "multiband compressor bank, mono-collapsed body",
    "_RNvXCs_20multiband_compressorINtB_31PreparedMultibandCompressorBankNtNtCs_4wide6f32x4_5f32x4Kj4_ENtCs_15effect_contract24PreparedNativeEffectBank14advance_clocksB_":
        "multiband compressor ramp clocks on a skipped silent block",
    "_RNvMCs_9soft_clipINtB_7ChannelNtNtCs_4wide6f32x4_5f32x4E7processB_":
        "saturator/clipper channel body",
    "_RNvXCs_16transient_shaperINtB_27PreparedTransientShaperBankNtNtCs_4wide6f32x4_5f32x4Kj4_ENtCs_15effect_contract24PreparedNativeEffectBank12process_bankB_":
        "transient shaper bank, stereo body",
    "_RNvXCs_16transient_shaperINtB_27PreparedTransientShaperBankNtNtCs_4wide6f32x4_5f32x4Kj4_ENtCs_15effect_contract24PreparedNativeEffectBank17process_bank_monoB_":
        "transient shaper bank, mono-collapsed body",
    "_RNCINvMCs_8de_esserINtB_7DeEsserNtNtCs_4wide6f32x4_5f32x4Kb0_E9run_blockKB_E0B_":
        "de-esser block body, stereo (`run_block::<false>`'s inner closure)",
    "_RNCINvMCs_8de_esserINtB_7DeEsserNtNtCs_4wide6f32x4_5f32x4Kb0_E9run_blockKb1_E0B_":
        "de-esser block body, mono-collapsed (`run_block::<true>`'s inner closure)",
    "_RNvXCs_8de_esserINtB_7DeEsserNtNtCs_4wide6f32x4_5f32x4Kb0_ENtCs_15effect_contract24PreparedNativeEffectBank12process_bank":
        "de-esser bank, stereo entry",
    "_RNvXCs_8de_esserINtB_7DeEsserNtNtCs_4wide6f32x4_5f32x4Kb0_ENtCs_15effect_contract24PreparedNativeEffectBank17process_bank_mono":
        "de-esser bank, mono-collapsed entry",
    "_RNvXCs_8de_esserINtB_7DeEsserNtNtCs_4wide6f32x4_5f32x4Kb0_ENtCs_15effect_contract24PreparedNativeEffectBank14advance_clocks":
        "de-esser ramp clocks on a skipped silent block",
    "_RINvCs_10dynamic_eq14set_lane_wordsNtNtCs_4wide6f32x4_5f32x4EB_":
        "dynamic EQ per-lane word write: a live record's retarget in the block, a restore at the "
        "plan swap",
    "_RNCINvMCs_10dynamic_eqINtB_9DynamicEqNtNtCs_4wide6f32x4_5f32x4Kb0_E9run_blockKB_E0B_":
        "dynamic EQ block body, stereo (`run_block::<false>`'s inner closure)",
    "_RNCINvMCs_10dynamic_eqINtB_9DynamicEqNtNtCs_4wide6f32x4_5f32x4Kb0_E9run_blockKb1_E0B_":
        "dynamic EQ block body, mono-collapsed (`run_block::<true>`'s inner closure)",
    "_RNvXCs_10dynamic_eqINtB_9DynamicEqNtNtCs_4wide6f32x4_5f32x4Kb0_ENtCs_15effect_contract24PreparedNativeEffectBank12process_bank":
        "dynamic EQ bank, stereo entry",
    "_RNvXCs_10dynamic_eqINtB_9DynamicEqNtNtCs_4wide6f32x4_5f32x4Kb0_ENtCs_15effect_contract24PreparedNativeEffectBank17process_bank_mono":
        "dynamic EQ bank, mono-collapsed entry",
    "_RNvXCs_10dynamic_eqINtB_9DynamicEqNtNtCs_4wide6f32x4_5f32x4Kb0_ENtCs_15effect_contract24PreparedNativeEffectBank14advance_clocks":
        "dynamic EQ ramp clocks on a skipped silent block",
    "_RINvMNtCs_6reverb6kernelNtB_14PreparedReverb6renderNtNtCs_4wide6f32x4_5f32x4Kb1_EB_":
        "reverb block render, every ring word valid history (`render::<_, true>`, the steady state)",
    "_RINvMNtCs_6reverb6kernelNtB_14PreparedReverb6renderNtNtCs_4wide6f32x4_5f32x4Kb0_EB_":
        "reverb block render while the rings fill (`render::<_, false>`)",
}


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
    exempt: dict[str, str] | None = None,
    pinned: dict[str, str] | None = None,
) -> int:
    """The per-kernel shape gate that replaced the raw `--simd-floor` total (#163 phase 0e).

    `exempt` is a `CONTROL_THREAD_EXEMPT`-shaped table (#877); `None` exempts nothing.
    `pinned` is a `RENDER_KERNELS`-shaped table (#1889, rule 5); `None` pins nothing.
    """
    failures = 0
    for name, reason in (pinned or {}).items():
        matched = [function for function in functions.values() if kernel_key(function.name) == name]
        if len(matched) != 1:
            failures += 1
            print(
                f"FAIL render kernel {name}: {len(matched)} functions in the module carry this "
                f"key (expected exactly one; {reason}). A vanished or renamed kernel is a "
                "reviewed table edit with evidence of where its arithmetic went, never a silent "
                "row deletion; a collision needs a more specific key.",
                file=sys.stderr,
            )
            for function in matched:
                print(f"  {function.name}", file=sys.stderr)
            continue
        vector = sum(1 for opcode in matched[0].opcodes if KERNEL_VECTOR.match(opcode))
        scalar = sum(1 for opcode in matched[0].opcodes if KERNEL_SCALAR.match(opcode))
        if vector == 0:
            failures += 1
            print(
                f"FAIL render kernel {name}: vector=0 scalar={scalar} (a listed render kernel is "
                f"fully scalar; {reason})",
                file=sys.stderr,
            )
            continue
        print(f"  pinned ok   vector={vector} scalar={scalar} {reason}")
    exempt = exempt or {}
    for name, reason in exempt.items():
        matched = [function for function in functions.values() if kernel_key(function.name) == name]
        if len(matched) != 1:
            failures += 1
            print(
                f"FAIL kernel exemption {name}: {len(matched)} functions in the module carry this "
                f"key (expected exactly one). Delete a stale entry, or make a colliding key more "
                f"specific ({reason}).",
                file=sys.stderr,
            )
            for function in matched:
                print(f"  {function.name}", file=sys.stderr)
    kernels = []
    for function, vector, scalar in kernel_arithmetic(functions, pattern):
        if kernel_key(function.name) in exempt:
            print(f"  exempt control-thread vector={vector} scalar={scalar} {function.name}")
            continue
        if pinned is not None and kernel_key(function.name) not in pinned:
            failures += 1
            print(
                f"FAIL kernel {function.name}: vector={vector} scalar={scalar} is counted but "
                "is in neither RENDER_KERNELS nor CONTROL_THREAD_EXEMPT. Add its row with the "
                "stage it renders (or its control-thread reason).",
                file=sys.stderr,
            )
        kernels.append((function, vector, scalar))
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

    # (b) an `unreachable` in a function that is neither a panic entry nor a named trap owner fails.
    trapping = VALID_SHAPE.replace(
        " 000024: 0b                         | end",
        " 000024: 00                         | unreachable\n"
        " 000025: 0b                         | end",
    )
    expect("(b) trap owner", check_callgraph(parse(trapping), "miso_engine_web_v1_render") == 1)

    # (b1) issue #137: `--trap-owner` admits the named owner and nothing else, and it never
    # relaxes the allocation half of the gate. Issue #2068: a bare name, with no pinned site
    # count, admits nothing.
    expect(
        "(b1) trap-owner refuses a bare name",
        check_callgraph(parse(VALID_SHAPE), "miso_engine_web_v1_render", ("render_next",)) == 1,
    )
    expect(
        "(b1) trap-owner does not admit a different owner",
        check_callgraph(parse(trapping), "miso_engine_web_v1_render", ("some_other_name=0",)) == 1,
    )
    # (b1a) issue #1581: a pinned owner admits exactly its pinned number of trap sites.
    expect(
        "(b1a) pinned trap-owner admits its one site",
        check_callgraph(parse(trapping), "miso_engine_web_v1_render", ("render_next=1",)) == 0,
    )
    twice = trapping.replace(
        " 000024: 00                         | unreachable\n",
        " 000024: 00                         | unreachable\n"
        " 000024: 00                         | unreachable\n",
    )
    expect(
        "(b1a) pinned trap-owner refuses a second site",
        check_callgraph(parse(twice), "miso_engine_web_v1_render", ("render_next=1",)) == 1,
    )
    expect(
        "(b1) trap-owner never admits a free",
        check_callgraph(parse(freeing), "miso_engine_web_v1_render", ("render_next=0",)) == 1,
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

    # (d4) issue #877: a pinned control-thread function with vector <= scalar is exempt, and is
    # not counted toward K.
    decode = (
        "_RNvMs2_Csb6SUMLvW3Nv_13parametric_eqINtB5_7ChannelNtNtCsgmSSuZlKkZ2_4wide6f32x4_5f32x4"
        "Kj4_E12decode_trackB5_"
    )
    control = VALID_SHAPE + synthetic_kernel(decode, vector=1, scalar=4, index=2)
    pinned = {kernel_key(decode): "control-thread state decode"}
    expect(
        "(d4) exempted control-thread function passes",
        check_kernel_shape(parse(control), "render_next|decode_track", 1, SELF_TEST_ROSTER,
                           exempt=pinned) == 0,
    )
    expect(
        "(d4) an exempted function does not count toward K",
        check_kernel_shape(parse(control), "render_next|decode_track", 2, SELF_TEST_ROSTER,
                           exempt=pinned) == 1,
    )
    # (d5) the same scalar shape in a function the table does not name fails.
    expect(
        "(d5) non-exempt function with vector <= scalar fails",
        check_kernel_shape(parse(control), "render_next|decode_track", 1, SELF_TEST_ROSTER,
                           exempt={}) == 1,
    )
    # (d6) a stale entry, one that names no function in the module, fails on its own.
    expect(
        "(d6) stale exemption entry fails",
        check_kernel_shape(parse(VALID_SHAPE), "render_next", 1, SELF_TEST_ROSTER,
                           exempt={"no_such_function": "stale"}) == 1,
    )
    expect(
        "(d6) the shipped table's entries are hash-free",
        all(kernel_key(name) == name for name in CONTROL_THREAD_EXEMPT),
    )

    # (e1) issue #1889: a pinned render kernel that the build scalarises in full fails, though
    # `kernel_arithmetic` drops it as a helper and K still holds; with vector arithmetic it passes.
    route = (
        "_RINvNtCsaB1_4lane7kernels28route_mix_column_ramps_blockNtNtCsgmSSuZlKkZ2_4wide6f32x4_"
        "5f32x4ECsQ9_5graph"
    )
    kernel_table = {"render_next": "synthetic render", kernel_key(route): "live route mix"}
    for vector, verdict in ((0, 1), (1, 0)):
        module = VALID_SHAPE + synthetic_kernel(route, vector=vector, scalar=0, index=2)
        expect(
            f"(e1) pinned render kernel at vector={vector}",
            check_kernel_shape(parse(module), "render_next", 1, SELF_TEST_ROSTER,
                               pinned=kernel_table) == verdict,
        )
    # (e2) an entry that names no function in the module fails.
    expect(
        "(e2) stale render-kernel entry fails",
        check_kernel_shape(parse(VALID_SHAPE), "render_next", 1, SELF_TEST_ROSTER,
                           pinned={"render_next": "synthetic render",
                                   "no_such_kernel": "stale"}) == 1,
    )
    # (e4) a counted kernel that the table does not name fails, so the table cannot go stale.
    expect(
        "(e4) unpinned counted kernel fails",
        check_kernel_shape(parse(VALID_SHAPE), "render_next", 1, SELF_TEST_ROSTER,
                           pinned={}) == 1,
    )
    # (e5) a key survives an impl-index move (`Xs9_` -> `Xsa_`, and a back-reference offset
    # shift with it), and a key that two functions share after normalising fails.
    eq_bank = (
        "_RNvXs{}_Csb6SUMLvW3Nv_13parametric_eqINtB{}_20PreparedParametricEqNtNtCsgmSSuZlKkZ2_"
        "4wide6f32x4_5f32x4Kj4_ENtCs4yi8P0M01YE_15effect_contract24PreparedNativeEffectBank"
        "12process_bankB{}_"
    )
    before, after = eq_bank.format("9", "5", "5"), eq_bank.format("10", "6", "6")
    eq_table = {"render_next": "synthetic render", kernel_key(before): "parametric EQ bank"}
    moved = VALID_SHAPE + synthetic_kernel(after, vector=4, scalar=0, index=2)
    expect(
        "(e5) a pinned key matches its kernel after an impl-index move",
        check_kernel_shape(parse(moved), "render_next", 1, SELF_TEST_ROSTER,
                           pinned=eq_table) == 0,
    )
    both = moved + synthetic_kernel(before, vector=4, scalar=0, index=3)
    expect(
        "(e5) a key that two functions share fails",
        check_kernel_shape(parse(both), "render_next", 1, SELF_TEST_ROSTER,
                           pinned=eq_table) == 1,
    )
    # (e3) the shipped table survives a rebuild and never pins an exempted function.
    expect(
        "(e3) the shipped render-kernel table is hash-free and disjoint from the exemptions",
        all(kernel_key(name) == name for name in RENDER_KERNELS)
        and not set(RENDER_KERNELS) & set(CONTROL_THREAD_EXEMPT),
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
    parser.add_argument("--trap-owner", action="append", default=[], metavar="SUBSTRING=SITES")
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
        failures += check_callgraph(functions, args.callgraph, tuple(args.trap_owner))
        if args.render_thread:
            failures += check_render_thread(functions, args.callgraph)
    if args.kernel_shape:
        if args.kernel_pattern is None or args.kernel_min is None:
            parser.error("--kernel-shape requires --kernel-pattern and --kernel-min")
        failures += check_kernel_shape(
            functions,
            args.kernel_pattern,
            args.kernel_min,
            exempt=CONTROL_THREAD_EXEMPT,
            pinned=RENDER_KERNELS,
        )
        failures += check_limiter_forwarding(functions)
        failures += check_no_eight_lanes(functions)
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
