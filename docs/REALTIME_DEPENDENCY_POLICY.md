# Realtime dependency policy

The issue-001 packages intentionally have no external dependencies. This is a bootstrap boundary,
not evidence that future DSP needs no dependencies. Any proposed dependency must be reviewed at
the issue that introduces it, with its realtime reachability, allocation behavior, platform support,
license, binary-size impact, and failure modes documented.

## Render-reachable prohibitions

Code reachable from `PreparedRenderPlan::render` may not allocate or free, use mutexes, read-write
locks, condition variables, blocking channels, async executors, filesystem or network APIs,
logging or tracing, dynamic loading, a general-purpose Wasm runtime, or syscalls. It may not
perform structural plan mutation or data-dependent unbounded work. Loading, parsing, I/O,
decoding, plan compilation, plugin validation, retirement, and telemetry aggregation stay on
control or worker threads.

The policy applies transitively. A dependency that hides one of these operations behind a benign
API is still forbidden in the render-reachable graph.

## Render is syscall-free (the single wake is gone)

Issue 100 created exactly one documented exception to the "no syscalls" rule above: the audio
callback could issue at most one `std::thread::unpark` per rendered block to wake a parked native
dependency-wave worker. **That exception no longer exists.** The native scheduler, its worker pool
and the graph crate's dependency-wave executor were removed as production-unreachable -- nothing
outside the graph crate's own tests ever engaged them -- so render-reachable code now makes no
syscall at all, unconditionally. `AGENTS.md` records the same, without an exception clause.

What the removal retires with it: the Dekker wake protocol and its `SeqCst` fence pair, the
binary-tree child wake, the linger-and-park budget, the bounded `recover_issued` deadline with its
trapped parcels and muted edges, and the strace/`/proc` gate that measured all of it. None of these
can be reintroduced by accident; a future multicore render must re-argue this exception from
scratch, and the burden is the same one #100 carried.

The sequential executor retains the plan-owned audio arena in
`crates/engine/src/realtime/disjoint.rs`. Under #1154, one non-cloneable `DisjointArena` owns its
ordinary `Box<[f32]>`; the shared multi-lease builder, wave/access tables and `Arc` are retired.
Shared reads borrow the owner, and every mutable access requires its exclusive borrow, so Rust
borrowing establishes access order. The constructor validates checked word/byte products and the
platform allocation limit before allocating. Every access validates planes, buffer IDs, shape,
silence-write protection and output/input disjointness in release before references are formed.
These are bounded acquisition checks, not per-sample validation loops. Buffer zero remains the
immutable `+0.0` silence slot, and valid render paths stay zero-copy.

## Unsafe-code ownership

The workspace denies unsafe code (`unsafe_code = "deny"`, `unsafe_op_in_unsafe_fn = "deny"` and
`clippy::undocumented_unsafe_blocks = "deny"`, root `Cargo.toml:90`, `:91`, `:102`). An exception
is one named file, never a crate or a pattern. The introducing issue must use a local, minimal lint
allowance; state the invariant next to the operation; include a `SAFETY` explanation; add tests;
and obtain explicit review. Unsafe code must not leak through a public API. A `pub unsafe fn` whose
`# Safety` section states the caller's contract is not a leak: the compiler makes every caller take
on that contract in an `unsafe` block of its own (#1494's control-word writers below).

The authority is the unsafe allowlist of `tools/realtime-policy` (`UNSAFE_ALLOWLIST` in
`src/policy.rs`), the list that CI runs; this section lists it, one entry per path, and adds
nothing to it. `scripts/check-bench-policy.sh` holds the matching exact set of
`#![allow(unsafe_code)]` files under `tools/`; `scripts/check-lane-policy.sh`, through
`scripts/policies/lane-source.toml`, confines raw architecture intrinsics to `softfma.rs` and
`fpenv.rs`; and `scripts/check-builtins-policy.sh` repeats `allocation_tracker.rs`'s entry. A file
that is not listed here needs a new decision, not an edit to a pattern.

### Realtime primitives

- `crates/engine/src/realtime/spsc.rs` (issue 003): the bounded single-producer/single-consumer
  ring. It owns fixed `UnsafeCell<MaybeUninit<T>>` storage, and its `SAFETY` comments rely on one
  producer, one consumer, release publication after writes, acquire before reads, and shared `Arc`
  storage that outlives both non-cloneable endpoints. `Arc` creation and destruction stay outside
  push, pop, and render. Its slot lookup (`Ring::slot`, #2041) skips the bounds check: a ring
  cursor advances only through `wrap_increment(cursor, slots_len)` and so stays below
  `slots_len == slots.len()`, which the lookup `debug_assert`s. The plan mailbox's cell index stays
  checked: the optimiser already proves it below 2.
- `crates/engine/src/realtime/disjoint.rs` (issue 100): the plan-owned disjoint audio arena, which
  the sequential executor still renders through. #1154 removed its `UnsafeCell`, manual
  `Send`/`Sync` implementations and foreign leases. `read_stereo` and `write_stereo` use safe
  slices; the combined fixed-array borrows and the `write_read_stereo` borrow (`:317-345`) form
  raw slices (`:208`, `:303`, `:334`). Their pointers derive from the complete uniquely owned
  allocation after release bounds, shape and disjointness checks, and every returned lifetime is
  tied to the exclusive owner borrow. Shared inputs may repeat; mutable outputs cannot overlap them
  or one another. No caller-side unsafe obligation remains.

`crates/graph` is entirely free of unsafe code. (`crates/engine/src/arch` was a third owner until
#84 phase A deleted it: the per-target kernels moved to `crates/lane`, and the exemption was
removed from the realtime policy gate (then `scripts/check-realtime-policy.sh`) in the same change.)

### Lane intrinsics

- `crates/lane/src/softfma.rs` (issue 083): the `x86_64` MXCSR helpers `read_mxcsr` and
  `write_mxcsr`, whose two unsafe blocks call `_mm_getcsr` and `_mm_setcsr`. They use these
  deprecated intrinsics rather than the inline assembly their deprecation note recommends. Their
  only non-test caller is `fpenv.rs` (`:145`, `:195`), at every native `x86_64` render entry; the
  direct test callers are `crates/lane/tests/fp_env.rs` and `crates/capi/src/runtime/tests.rs`,
  and gate G6 (`crates/lane/tests/g6_ftz_inert.rs`) reaches the control word through
  `lane::fpenv`. Since #1494 `write_mxcsr` is an `unsafe fn`, so its soundness no longer rests on
  what its callers happen to pass. Its `# Safety` section is the contract: the word sets no
  reserved bit (16-31), which would raise #GP; only `CANONICAL_MXCSR` (0x1F80, the architectural
  default), or a word that differs from it only in the status flags, is inside the floating-point
  environment Rust assumes (the `core::arch` documentation of `_mm_setcsr` says Rust assumes the
  default exception masks, rounding and DAZ), and a word handed back can be outside it, because a
  host can run with FTZ or DAZ set; the caller lets no compiled floating-point code run under a
  word outside it except the code it deliberately measures, and a hand-back meets this by returning
  the thread to the word its caller already ran under, with no floating-point code after it in the
  writer; and unless the write installs a word inside the environment or is a hand-back, the caller
  restores the previous word, itself or through `CanonicalFpEnv`'s `Drop`. Its `SAFETY` comment
  states only what the body relies on: SSE, which every `x86_64` host has, and that contract. A
  `compile_fail` doctest with a passing twin checks, on each `x86_64` doctest run, that safe code
  cannot call it; `read_mxcsr` stays safe. `write_mxcsr` itself does not restore the previous word.
  Until #163 phase 2 (`477dc15ee`) the file also held the software FMA and its wasm `simd128`
  promote/demote intrinsics; that phase retired the emulation, and the file kept its name because
  the policy files name its path. No `Lane` value or vector type escapes the crate as unsafe.
- `crates/lane/src/fpenv.rs` (issue 146): the canonical floating-point environment that every
  native render entry pins. Since #1494 its three `write_fp_control_word` variants (`x86_64`,
  AArch64, and the no-op on targets without a control word) are `unsafe fn` with the
  `write_mxcsr` contract (on AArch64: no `RES0` bit of FPCR). Each variant carries a
  `compile_fail` doctest with a passing twin: the `x86_64` pair runs in every `x86_64` doctest run
  and the AArch64 pair in CI's `aarch64-debug` doctest leg, and each checks there that safe code
  cannot call the writer; the no-op variant's pair is documentation only, because no doctest run
  compiles it. `CanonicalFpEnv` is the only safe way to change the word: `enter` (`:477`)
  writes the canonical word, and `Drop` (`:502`) writes the word `enter` read from this thread; each
  call's `SAFETY` comment says why that word meets the contract, and `enter`'s says why the guard
  stays sound when safe code passes it to `mem::forget`. `read_fp_control_word` stays safe.

`fpenv.rs` is reachable from a render path deliberately -- pinning the environment is the render
entry's first act and unpinning it is its last. The guard reads the control word, writes the
canonical word and runs a barrier on entry (`CanonicalFpEnv::enter`), and runs a barrier and
writes the saved word on exit (`Drop`). It allocates nothing, takes no lock and makes no syscall.
On AArch64 the read and the writes are `mrs`/`msr` register accesses with no memory operand. On
`x86_64` they are the `softfma.rs` helpers, and x86 has no register form of these instructions:
`_mm_getcsr`/`_mm_setcsr` lower to `STMXCSR m32`/`LDMXCSR m32` through a stack slot (the
workspace's `+avx2,+fma` build emits the VEX forms, `vstmxcsr -4(%rsp)` and `vldmxcsr -4(%rsp)`).
`fpenv.rs` carries unsafe inline assembly for two reasons -- the workspace's only inline
assembly, and the reason the softfma entry above says "rather than the inline assembly their
deprecation note recommends" and this one does not. Its other unsafe sites are the raw writer's
calls (#1494, above).

First, AArch64's `mrs`/`msr FPCR` pair (`:213`, `:271`), because the standard library exposes no
stable FPCR intrinsic (Arm Architecture Reference Manual for A-profile, `FPCR`, Floating-point
Control Register). On `x86_64` there is no counterpart: `fpenv.rs` calls `softfma.rs`'s
already-approved MXCSR helpers. On the render path the blocks write only a value previously read
from the same thread or `CANONICAL_FPCR`, and they affect no other thread.

Second, on both, an empty `asm!` block as the guard's scheduling barrier (`scheduling_barrier`,
`:456-463`). Installing a control word is a side effect the optimizer does not model --
`_mm_setcsr` lowers to an intrinsic declared as touching only its own argument's memory -- so
without a barrier nothing stops a computation being scheduled outside the region it was meant to
run in. The empty block is deliberately **not** `nomem`: being a memory clobber is its entire
purpose, and it emits no instructions. It anchors every memory-dependent computation, which is
every render; it does not anchor a value held entirely in registers, and
`crates/lane/tests/fp_env.rs` proves that limit rather than assuming it, with a register-only
product that a release build really does schedule outside the guard until the test anchors it
itself.

The exemption is the file, not the crate: `tools/realtime-policy` and
`scripts/check-lane-policy.sh` (through `scripts/policies/lane-source.toml`) both name `fpenv.rs`
explicitly, and both have a mutation test that proves a third lane file does not inherit it
(`cargo test -p realtime-policy`, `scripts/test-lane-policy.sh`, `fpenv_extra.rs`).

### Control-word test writers

Each file below calls the `unsafe fn` control-word writers (#1494) to install a word a host might
arrive with -- FTZ, DAZ, a directed rounding mode, sticky status flags or FPCR's `FZ` and `DN` --
and measure the engine under it: through the guard, and with no guard as a control
(`g6_ftz_inert.rs` measures only the D7 flush arms, with no guard). The calls are the file's only
unsafe code. Each call sits in an `unsafe` block whose `SAFETY` comment names the word, says it sets
no reserved (or `RES0`) bit, and names the guard or write-back that restores the previous word.

- `crates/lane/tests/fp_env.rs` (#146, #1017): the guard's own tests. It calls `write_mxcsr` on
  `x86_64` and `write_fp_control_word` on AArch64, and restores through its `Restore` guard.
- `crates/lane/tests/g6_ftz_inert.rs` (#83, gate G6): sets the flush bits around one pass of its
  arms and writes the saved word back.
- `crates/capi/src/runtime/tests.rs` (#146): the `#[cfg(test)]` capi runtime tests. Only its
  `x86_64` `fp_environment` module carries `#[allow(unsafe_code)]`; it calls `write_mxcsr` and
  restores through `Restore` and explicit write-backs.
- `crates/host-core/tests/fp_environment.rs` (#146, #1017): the host facade's arms. It restores
  through `Restore`.
- `tools/wasm-gates/tests/g6_full_corpus_ftz.rs` (#144, #146): gate G6 over the full corpus. It
  restores through `WordGuard`'s `Drop`, and `scripts/check-bench-policy.sh` names it in its
  `tools/` set.

### C-ABI and browser-ABI boundaries

- `crates/capi/src/ffi.rs` (issue 022): the raw C-pointer ownership boundary of the exported
  `miso_engine_v1_*` entries. Each unsafe operation carries a `SAFETY` comment that names what it
  relies on: the caller's pointer or handle contract, the size or capacity check made before a
  write, the live-kind check on the shared `HandleHeader` prefix before a concrete handle is
  borrowed, the disjointness of raw field projections, or the `Box::into_raw`/`Box::from_raw`
  ownership transfer between create and destroy.
- `hosts/host-web/src/ffi.rs` (issue 024): the raw Wasm32 handle, pointer and slice boundary of the
  `miso_engine_web_v1_*` exports. Its non-test unsafe sites are the staging slices of
  `miso_engine_web_v1_source_submit` (`:825`, `:832`, `:835`), `miso_engine_web_v1_source_seek`
  (`:879`) and `with_staged_source_id` (`:904`). Each carries a
  `SAFETY` comment that names the bounds check, `repr(C)` layout or staging liveness it relies
  on; the module carries no `catch_unwind`, because the target and the release profile abort on
  panic. The file
  has a second, test-only unsafe owner: the `#[cfg(test)]` module `allocation_counter`
  (`:2102`) registers `CountingAllocator` as the unit-test binary's `#[global_allocator]` (added by
  `708036a76`). It forwards every operation unchanged to `System` and counts successful operations
  in thread-local counters, only while armed; its `SAFETY` comments state that forwarding.
- `hosts/blake3-web/src/lib.rs` (#2111): the raw Wasm32 hasher-handle boundary of the browser's
  stem-identity BLAKE3 module (`miso_engine_blake3_v1_*`). Its unsafe sites are the
  `#[unsafe(no_mangle)]` exports and the handle borrows of `input`, `update` and `finalize` and
  the `Box::from_raw` of `free`; each carries a `SAFETY` comment naming the caller's live-handle
  contract or the `Box::into_raw` it reverses. No export takes a data pointer: the hasher owns its
  staging buffer and a length beyond it is refused. Control-side only; it never runs on an audio
  thread.
- `crates/capi/tests/plan_swap_race.rs` (#1042, #1273): races control calls against
  plan-swapping render calls, both through the exported C entries. Its unsafe is the C calls
  themselves, each with the handle and storage contract stated beside it; the allocator it runs
  under is `bench_support::alloc`'s.
- `crates/capi/tests/response_get.rs` (#1573): sends `RESPONSE_GET` through the exported
  `miso_engine_v1_submit_command` and compares the curves with the core's own. Its unsafe is the
  C calls themselves (engine create and destroy, compile, command submit, session and plan
  destroy), each with the handle and storage contract stated beside it; it installs no allocator
  and calls no render entry.
- `crates/capi/tests/response_strip.rs` (#1578): sends a strip `RESPONSE_GET` through the
  exported `miso_engine_v1_submit_command` and reads its `RESPONSE_READY` through
  `miso_engine_v1_dequeue_event`, and compares the curves with the core's own. Its unsafe is the
  C calls themselves (engine create and destroy, compile, command submit, render, service, event
  dequeue, session and plan destroy), each with the handle and storage contract stated beside it;
  it installs no allocator.
- `crates/capi/tests/session_replace.rs` (#1386): sends `SESSION_REPLACE` through the exported
  `miso_engine_v1_submit_document_command` and the equal transaction through
  `miso_engine_v1_submit_command` on a twin, and compares replies, events and rendered blocks.
  Its unsafe is the C calls themselves (engine create and destroy, compile, both command submits,
  source submit, render, event dequeue, session and plan destroy), each with the handle and
  storage contract stated beside it; the allocator it reads is `bench_support::alloc`'s.
- `crates/capi/tests/empty_boot.rs` (#1682): the exported-C evidence that a session with no source
  and no track boots, renders silence and grows by a structural transaction, with zero allocator
  calls in the render entry. Its unsafe is the C calls themselves (engine create and destroy,
  compile, command submit, service, event dequeue, source seek and submit, plan resource and
  watermark queries, render, session and plan destroy), each with the handle and storage contract
  stated beside it; the allocator it runs under is `bench_support::alloc`'s.
- `crates/capi/tests/strip_transitions.rs` (#1325): the exported-C evidence that a removed strip
  ramps out on the running plan and its successor is adopted at the scheduled sample, with zero
  allocator calls in the render entry from the commit through the adoption. Its unsafe is the C
  calls themselves (engine create and destroy, compile, command submit, event dequeue, last error,
  source submit, plan resource and watermark queries, render, session and plan destroy), each with
  the handle and storage contract stated beside it; the allocator it runs under is
  `bench_support::alloc`'s.
- `crates/capi/tests/resource_lifecycle.rs` (issue 119): the exported-C retained-allocation and
  disposal evidence. It calls the C entries, including the render entry, and owns
  `LifecycleAllocator`, a `GlobalAlloc` that delegates the original pointer and layout unchanged
  to `System` and adds thread-local counters that are enabled only around its isolated test
  thread.
- `crates/capi/tests/input_design_bounds.rs` (#1556, #1602): the exported-C evidence that a plan
  bounds no input design. It compiles, submits structural transactions, renders (so a pending plan swaps in) and drains the
  event lanes through the C entries. Its unsafe is the C calls themselves, render included, each
  with the handle and storage contract stated beside it; it installs no allocator.
- `hosts/host-web/tests/control_parity.rs` (#1560, which absorbed #1364's
  `control_smoothing_parity.rs`): renders one session and one script of control edits through the
  browser host and through the exported C entries, and compares the outcomes and the bits. Its unsafe
  is the C calls themselves (engine and plan create and destroy, compile, command submit, planar
  source submit, render, event dequeue), each with the handle and storage contract stated beside
  it; it installs no allocator.
- `tools/audit/src/capi.rs` (issue 022): the non-timed audit plan for the exported C render entry.
  Its unsafe is the C calls, each with the handle and storage contract stated beside it; only the
  render call is inside the audited scope.

### The browser render-locked allocator

- `hosts/host-web/src/render_lock.rs` (#1333, decision 15 D15-10): the `#[global_allocator]` of the
  shipped browser module, and the only allowlisted global allocator linked into a shipped artifact.
  `RenderLockedAllocator` forwards `alloc`, `alloc_zeroed`, `realloc` and `dealloc` to the inner
  allocator with their arguments unchanged. Its only additions are one thread-local flag read per
  call and, inside a render-locked window, one relaxed increment of the process-wide counter; it
  never alters a pointer, layout, size or lifetime. The registration is
  `cfg(all(target_family = "wasm", not(test)))` (`:117-120`), so native test binaries and tools
  keep their own allocator. `render_locked` itself is safe code.

It is the runtime proof behind D15-10: the static call-graph gate cannot see behind the plan
executor's `call_indirect` (`Box<dyn PreparedPlanExecutor>`), so the allocator counts each call
made inside a render-locked window, and browser qualification reads
`miso_engine_web_v1_render_allocation_count` and asserts zero
(`hosts/host-web/qualification/run.mjs:266-270`). The count covers exactly the exports whose
bodies run inside `render_locked`. Two sets of exports matter here, and both are named below.

1. **The exports either worklet calls on its AudioWorklet thread after boot.** The engine worklet
   (`hosts/host-web/web/miso-engine-v1-audio-worklet.js`, `process()` and its port handlers)
   calls the exports that the table `tools/realtime-policy/render-thread-exports.json` lists.
   The table is the one list of set 1. Each entry names an export without its
   `miso_engine_web_v1_` prefix, the worklet handlers that call it after boot, and whether its
   body is one `render_locked` call (`wrapped`). It has four `wrapped: false` entries, `dispose`,
   `render_allocation_count` and, since #1381a, `service` and `control_allocation_count`, and
   since #1562a the five control-frame exports and since #1570b `control_event`, each with its reason. `tools/realtime-policy` proves that every
   `wrapped` entry's body is that call. The harnesses `scripts/test-web-audioworklet.mjs` and
   `sdk/test/browser-pcm-evals.mjs` fail when a worklet calls an export that the table does not
   list. This document does not repeat the names, so that they cannot drift from the table.
   The table lists the PCM-feed worklet's calls too (worklet `feed`); their sites are below.
   The SDK's PCM-feed worklet
   (`sdk/src/browser-assets/miso-engine-v1-pcm-feed-worklet.js`) calls `source_submit` (`:386`)
   and `source_seek` (`:445`) at the top of its `process()` (`:306-328`) before it renders, and
   `source_seek` again from its attach node's `prepare-seek` handler (`:539`).
   Since batch 7 review B1 the two exports reach the sources through
   `SessionState::submit_unserviced` and `seek_unserviced`, which run no service step: the step
   takes the report lock, drops retired plans and stages counter snapshots, so it belongs to the
   message handler alone (`render_locked_source.rs` measures both exports after a render with
   counter telemetry configured: zero allocations).
2. **The exports whose bodies run inside `render_locked`.** Every export of set 1 except
   `dispose`, `render_allocation_count`, `service` and `control_allocation_count`, and also five staging accessors that the engine
   worklet calls only during boot: `observation_id_ptr`, `observation_id_capacity`,
   `observation_selection_ptr`, `observation_selection_bytes` and `observation_selection_capacity`.

Since #1488 the two sets are equal for the worklets' post-boot calls, apart from the named
exceptions, as the `ffi` module header (`hosts/host-web/src/ffi.rs:14-39`) and
`render_lock.rs:5-11` state: `dispose`, which is teardown and frees the host by design (it is also
the boot-failure path), `render_allocation_count`, the reader of the count, and, since #1381a,
`service` and `control_allocation_count`. The service step is the single-instance mode's control
work (decision 15 D15-10): the worklet's message handler runs it after every control message, never
in `process()`, and its allocations and frees (it frees every plan and attachment render has
displaced) count in the control counter `CONTROL_ALLOCATIONS` of `render_lock.rs`, which
`control_allocation_count` reads and the host status reports as `singleModeControlAllocations`.
Since #1562a the generic command-frame entry (`control_request_ptr`, `control_command`,
`control_response_ptr`, `control_response_bytes` and `control_revision`) is the same kind of
export: `control_command` runs `SessionState::command` in the message handler, which allocates and
frees, so its allocations are control allocations too. Since #1570b the event entry `control_event`
is the same kind: the worklet's message handler drains the reliable lane and then the lossy lane
through it, after every message and service step, and posts each frame to the page. The
render-locked count stays exactly zero. The browser has no spectrum export: spectrum is the
session's `SPECTRUM_*` frames through `control_command` (#1597).

### Test-only counting allocators

Each is an `unsafe impl GlobalAlloc` that forwards every request to `System` unchanged and adds only
audit counters, in a file or module that no production target links: the three below are `tests/`
files, and `hosts/host-web/src/ffi.rs`'s `#[cfg(test)]` `CountingAllocator` (above) is the one in a
`src/` file.

- `crates/builtins-compiler/tests/allocation_tracker.rs` (issue 007): `TrackingAllocator` records
  the builtin compiler's phase-two allocations, on the armed thread only, so they can be compared
  with the phase-one layout report. The file also makes three direct `std::alloc` calls
  (`alloc`, `realloc`, `dealloc`, `:802-809`) in its positive realloc liveness control.
- `crates/session/tests/allocation_budget.rs` (#107): `CountingAllocator` adds one relaxed counter
  for the 4,096-track control-pipeline allocation-call budgets.
- `hosts/host-web/tests/boot_transient_budget.rs` (#240): `PeakAllocator` keeps thread-local
  counters that measure the parse and model-build high-water mark against the pinned
  `PARSE_TRANSIENT_MULTIPLIER`.

Four further entries on the allowlist carry no unsafe code today:
`crates/soft-clip/tests/allocation.rs` (#91), `crates/transient-shaper/tests/allocation.rs` (#92),
`crates/true-peak-limiter/tests/allocation.rs` (#90) and
`crates/multiband-compressor/tests/no_alloc_render.rs` (issue 018, re-landed under audit #94).
Each once owned a counting allocator; each now reads `bench_support::alloc`'s thread-scoped
counters instead (the wrappers were removed by `568ad4087`, #1046's `cb4898437`, `a6da0cade` and
`39c4651b1` respectively). Their entries approve nothing, and nothing here approves new unsafe code
in them; #1438's Amendment removes them from the allowlist.

### Tool-only allocators and guests

- `tools/bench-support/src/alloc.rs` (#104): the one audited global allocator for every tool. Every
  method forwards the caller's unchanged pointer and layout to `System`. It records an allocation
  made while a render scope is armed and then, by default, terminates without unwinding, because
  unwinding out of a `GlobalAlloc` method is not permitted; a probe may set count-and-continue
  mode (`Mode::Count`).
- `tools/wasm-gate-guest/src/lib.rs` (#83, gate G5): the `cdylib` guest of the cross-target digest
  harness. Exporting a function from a `cdylib` needs `#[unsafe(no_mangle)]` under edition 2024,
  and there is no safe spelling of it. Its exports take and return `u32`, carry no pointer and no
  import, and it holds no `unsafe` block, `fn` or `impl`. Its entry in this gate therefore approves
  nothing, like the four stale entries above: the gate's pattern never matches
  `#[unsafe(no_mangle)]`, and #1438's Amendment removes the entry. The approval that does bind is
  `scripts/check-bench-policy.sh`'s exact set of `#![allow(unsafe_code)]` files under `tools/`.

### Render-path reachability

Approved unsafe code that runs on a render thread, per entry:

- `spsc.rs`: yes. Its push, pop and peek unsafe sites and the unchecked `Ring::slot`
  lookup lie inside the gate's marked render regions; render drains event and
  control rings and exchanges plans through them.
- `disjoint.rs`: yes. Its raw-slice borrows lie inside its marked region (`:106-347`), and the
  sequential executor forms its combined multi-buffer and stereo borrows through them.
- `fpenv.rs`: yes, at every native render entry, on both `x86_64` and AArch64. On wasm its guard is
  a zero-sized value with no code.
- `softfma.rs`: yes, on `x86_64` only, through `fpenv.rs`.
- The control-word test writers: no. They are test code, and each writes the word on its test
  thread between render calls, never inside one.
- `crates/capi/src/ffi.rs`: yes. `miso_engine_v1_render_f32_planar` (`:807`) pins the environment
  and then dereferences the plan handle and the output descriptor through unsafe blocks before it
  renders.
- `hosts/host-web/src/ffi.rs`: yes, on the AudioWorklet thread, and every such site runs inside a
  render-locked window. The body of `miso_engine_web_v1_render` holds no unsafe block. The staging
  slices of `source_submit` and `source_seek` run in the PCM-feed worklet's `process()` before
  each render and in the engine worklet's source and seek handlers. The five session-less `response_frame_*` exports
  (#1574) are control work of the response-preview instance of the same kind: they stage one
  `RESPONSE_GET` frame and answer one frame through `host_core::serve_response_frame`, and no
  worklet calls them. Its
  `CountingAllocator` is test-only.
- `render_lock.rs`: its allocator methods run on every allocator call the browser module makes. They
  count a call exactly when it is made inside a render-locked window, at boot or after it; the count
  runs from instantiation and is never reset. Calls outside every window are not counted: the boot
  exports themselves, the unwrapped boot accessors, `dispose`, `render_allocation_count`, `service`
  and `control_allocation_count` (the control counter counts every call outside a window, #1381a). The
  worklet calls render-locked exports during boot too: the eleven staging accessors of set 2.
  Those calls are counted, and today they make no allocator call: the stagings they touch are
  allocated outside any window, by boot's `reserve_stagings` (`hosts/host-web/src/ffi.rs`).
  Qualification reads the count of each instance before its
  `dispose` and asserts it is zero, so a boot-time allocation inside a window fails it as a
  render-time one does.
- Every other entry is test or tool code. `plan_swap_race.rs`, `resource_lifecycle.rs`,
  `input_design_bounds.rs`, `empty_boot.rs`, `control_parity.rs` and `tools/audit/src/capi.rs` call the C
  render entry from threads of their own. Their own unsafe is
  the caller side of the C contract, apart from `resource_lifecycle.rs`'s `LifecycleAllocator`,
  which runs on the same test thread as its render calls.

### Retired exceptions

The source-policy checker's first allowlist (`68ff477e4`) accepted unsafe syntax in exactly five
source files, although its failure message said "four":
`crates/miso-engine-core/src/realtime/spsc.rs` (now `crates/engine/src/realtime/spsc.rs`), the
`main.rs` of the realtime audit, the protocol audit and the protocol benchmark (whose files later
became `tools/audit/src/realtime.rs`, `tools/audit/src/protocol.rs` and
`tools/bench/src/protocol.rs`), and the native-effect contract bench's `main.rs`. Issue 005 had
permitted the protocol audit and the protocol benchmark; the realtime audit carried a test-only
realtime audit allocator, and the effect contract bench carried its own audited allocator. #104
phase B (`d9a66a952`) replaced the audit tools' and the effect contract bench's allocators with
`tools/bench-support/src/alloc.rs` and removed their entries, so `tools/audit/src/realtime.rs` and
`tools/audit/src/protocol.rs` contain no unsafe code and are off the allowlist (#136 later folded
the effect contract bench into the collapsed tool packages). #1075 retired the
BTLV-versus-FlatBuffers benchmark under owner ruling R9 (only real host paths are benchmarked):
`tools/bench/src/protocol.rs`, its unsafe exemption and `tools/bench`'s `flatbuffers` dependency are
gone. #1033 removed `tools/native-pcm-runner` and its exemption.

Loom `=0.7.2` is MIT licensed and test/model-only; it is not a production, Wasm, or
render-reachable dependency.

`tools/realtime-policy` extracts explicitly marked render-reachable regions and rejects
allocation/growth, locks, waits, I/O, logging, networking, process/thread APIs, and async surfaces.
Its test suite (`cargo test -p realtime-policy`) proves allocation, lock, log, and
unsafe-scope violations are rejected, including unsafe code in a file beside an allowlisted one
(`tools/audit/src/other.rs` beside `capi.rs`) and at the paths of retired exceptions. Runtime
mutation probes (`tools/audit/src/realtime.rs`, `--probe`) separately prove the realtime
allocator/deallocator and forbidden-operation hooks are armed.

## CPU and Wasm policy

CPU ISA selection is not a Cargo feature. Issue 083 (master plan D4) replaces the earlier runtime
capability model on x86: native `x86_64` builds are pinned to `x86-64-v3` by the workspace
`.cargo/config.toml` (`-C target-feature=+avx2,+fma`), `crates/lane` refuses to compile
without both features, and every host and C-ABI entry attests the CPU once at boot through
`lane::attest_host`, refusing to start rather than falling back silently. That pin is
the only approved global ISA configuration and `scripts/check-workspace-policy.sh` admits exactly
it; `-C target-cpu`, a global `[build]` rustflags table and any other feature set stay forbidden.
NEON is baseline on AArch64. Browser Wasm baseline and `simd128` are separate artifacts; relaxed
SIMD is forbidden and correctness cannot depend on it (`scripts/check-lane-policy.sh`). Fusion
exists only where `Lane::fma` is written (D3): hardware FMA on x86 and NEON, and an exact software
FMA on wasm that gate G3 proves bit-identical to the hardware instruction. Intrinsics live only in
`crates/lane`; the session's semantics stay target-independent, and cross-backend and
cross-target equality is `to_bits` identity, not a tolerance (D5).

Issue 003 concurrent queues use only pointer-width atomic loads/stores. Rust guarantees every
available standard atomic type is lock-free, and the supported native/mobile targets expose
`target_has_atomic="ptr"`. Counters are endpoint-local plain integers, so render performs no atomic
read-modify-write retry. Browser launch uses `LocalRing` on its single render agent. Since #1380
the shipped `simd128` module is built with atomics and imports one shared memory, `env.memory`,
with one instance per memory; `scripts/check-web-audioworklet.sh` (which took over the scalar
object inspection #1062 retired) holds that shape, atomics are allowed, and the call-graph gate
keeps every render-thread closure at `atomic_waits=0`. The module makes no cross-agent
shared-memory claim yet: two instances on one memory are #1332's.

The revision gate and the latest-target cells (#1432) give render one `fetch_or` per plan per
block and one `swap` per pending dirty word; on AArch64 without LSE (ARMv8.0) a read-modify-write
compiles to a load-linked or store-conditional loop, the same property the plan mailbox's claim
compare-and-swap already has.

## Panic behaviour by profile

Issue 083 (master plan D12) gives the workspace one release profile: `lto = "fat"`,
`codegen-units = 1`, `panic = "abort"`, `debug = 1`, with `[profile.bench]` inheriting it so a
benchmark measures the shipped code. Fat LTO and a single codegen unit are what let a consumer's
instantiation of the `#[inline(always)]` generic kernel bodies in `crates/lane` collapse
into the intended straight-line loop; `debug = 1` keeps line tables so a profile or a core dump
names a kernel, and costs build time and artifact size, never speed.

`panic = "abort"` is a deliberate, user-visible change and is recorded here rather than hidden:

- A release build has **no unwinding**. `std::panic::catch_unwind` still compiles and still returns
  `Ok` on the normal path, but it can no longer contain a panic: the process aborts instead. The
  affected boundaries are `crates/capi/src/ffi.rs` (`catch_result`, `catch_destroy`,
  which map a contained panic to `RESULT_INTERNAL`), `hosts/host-web/src/ffi.rs`, the
  `catch_unwind` probes inside `crates/conformance`, and the `panic_unwinds` counter in
  `tools/bench`. In a release artifact each of those is a diagnostic that no longer
  fires; none of them is load-bearing for a call that does not panic, so behaviour on a passing host
  is unchanged.
- `rebuild_superseding` in `crates/control-plane/src/control.rs` (#2010) runs a rebuild under
  `catch_unwind`, republishes a withdrawn candidate the rebuild did not take, then resumes the
  panic. It runs on the control thread only, never in render. Under `panic = "abort"` nothing
  unwinds, so it changes nothing in a release build.
- Embedders must read this as: **the C ABI does not convert a panic into `RESULT_INTERNAL` in a
  release build of `libcapi`.** A panic is an engine defect, and unwinding across a C or
  Wasm frame is undefined by either ABI, so aborting is the honest contract. `RESULT_INTERNAL` stays
  in the ABI for the internal failures that are returned, not thrown.
- The browser artifacts built by `scripts/build-web-audioworklet.sh` are `--release` builds and
  therefore inherit `panic = "abort"`: a panic inside the AudioWorklet traps the module instead of
  returning `RESULT_INTERNAL` to the worklet shim.
- Cargo ignores the `panic` setting when it builds a test or benchmark harness, so
  `cargo test --release` (including the Loom race model and every gate that runs in release) still
  unwinds and `#[should_panic]` still works. That was verified by inspecting the `rustc`
  command lines: a release binary is compiled with `-C panic=abort`, a release test harness is not.

### What `panic = "abort"` costs a workspace-wide release build

The previous bullet is the whole story for a *per-package* release invocation. It is not the whole
story for `cargo test --release --workspace --all-targets`, and the difference is a build failure
rather than a behaviour change.

Because Cargo ignores `panic` for test harnesses but honours it for lib and bin units, one
workspace-wide invocation builds **both** panic variants of every crate: the abort variant for the
shipped units, the unwind variant for the harnesses and everything they depend on. Cargo normally
keeps the two apart by hashing the variant into the output filename. It cannot do that for a lib
unit that also carries a `cdylib` or `staticlib` crate-type, because those emit un-hashed filenames
(`libfoo.so`, `libfoo.a`). Two native packages are in that shape (`effect-package` was a third
until #1037 removed it):

| package | crate-types |
|---|---|
| `crates/capi` | `rlib`, `staticlib`, `cdylib` |
| `hosts/host-web` | `rlib`, `cdylib` |

The two variants write to the same paths, the second clobbers the first, and a downstream unit
links whichever landed last and hits a metadata mismatch. Which unit reports it depends on build
scheduling, so the *error face* is nondeterministic — it usually surfaces as a misleading
`error[E0463]: can't find crate for ...` — while the *failure* is deterministic: a clean
`cargo test --locked --release --workspace --all-targets` does not build.

**The supported invocation is `scripts/run-release-workspace-tests.sh`**, which sets
`CARGO_PROFILE_RELEASE_PANIC=unwind` for that one run. Forcing a single panic variant means nothing
is built twice and nothing is clobbered.

`nightly.yml`'s `release-link-proof` job runs the script with `--no-run`, so the workspace-wide
release build cannot silently rot again. That gates exactly what rotted: the clobber is a build
failure that happens while compiling the test targets, so building them is enough to catch it, and
doing so is deterministic.

That job overrides three codegen settings at job level (issue #1511 D2): `lto = "thin"`,
`codegen-units = 16` and `debug = 0`. The clobber is a property of the profile name `release`, its
panic setting (which makes Cargo build two variants) and the `cdylib`/`staticlib` crate-types (which
give un-hashed filenames); none of the three settings changes the variant count or a filename, and
`thin` is still a Cargo-level LTO setting, so the unit graph keeps its shape. The
`cfg(debug_assertions)` residual depends on `debug-assertions`, which is not overridden. Shipped
artifacts and every per-package `-p` release leg keep D12's full `[profile.release]`; the one loss
is that the nightly 100x randomized differentials run under thin, not fat, LTO.

Design #359 §12 stage 4 moved this off the per-PR path: the old `release-build.yml` ran the script
unconditionally on every push to `main`, and on a pull request only when the diff touched a Cargo
manifest, `rust-toolchain.toml`, the script, or the workflow itself. That workflow is retired.
`qualification.yml`'s `release-shape` job is gated the same way (`route == 'full' &&
release_inputs == 'true'`, i.e. a manifest-touching diff) and still runs on every such PR and
`main` push, but it only runs `check-release-shape.py`'s metadata policy and a metadata-only,
check-only `cargo check --release --workspace --all-targets` under the unwind override -- a check
cannot reproduce a link-time clobber, so it is not a substitute for this link proof. The clobber
is a property of crate-types and `[profile.release]`, so a manifest is the only thing that can
introduce it; the residual risk a source-only pull request carries is a `cfg(debug_assertions)`
item that compiles in debug and not in release, now bounded by nightly's cadence (worst case ~24h,
03:17 UTC) rather than caught on the same `main` push. `gh workflow run nightly.yml --ref <branch>`
forces the full nightly suite, including this proof, on a branch outside the schedule.

Running those tests in CI is the intended end state and is not done yet. Repairing the build made
two release-only failures reachable for the first time, and both predated this work — no CI leg had
ever run these tests in release, and the workspace release build did not compile, so nothing could
have run them. Both are now resolved (issue #359 WP-2/WP-5b):

- `observation_cost_classes_are_separated_from_a_computed_scan_in_release` (#143, in
  `crates/host-core/tests/effect_observation.rs`, `--ignored`, release only) failed
  deterministically. Arming eight taps costs nothing measurable — `AllArmed` sits at or below
  `CapacityUnarmed` — while the old assertion subtracted the `NoLiveControls` baseline and so
  charged observation for the cost of live controls *existing*. It is rebaselined to assert the
  claim that is actually load-bearing: `armed <= unarmed_with_live_controls * 1.10 + 50 µs`, i.e. arming is not
  measurably slower than attached-but-unarmed live controls.
- `a_million_windows_are_read_whole_and_in_order` (#160) (#143, in
  `crates/engine/tests/observation_transport.rs`) failed intermittently: three of five
  full-workspace release runs, against 20 of 20 passing standalone release reruns. It was
  timing-dependent rather than a plain red. WP-2 removed the one scheduler-dependent assertion
  (the livelock bound `absent <= reads * 10 + 1000`); the remaining assertions (`torn == 0`,
  `regressions == 0`, `newest == WINDOWS`) are absolute and do not depend on scheduling.

The nightly step (`release-link-proof` in `nightly.yml`) still passes `--no-run`: it continues to
gate the build-clobber failure it exists for, and nothing in CI runs the release-mode test suite
yet. Turning that step into the full `cargo test` invocation is a follow-up — the script already
runs the tests by default.

What the override does **not** touch:

- **Shipped artifacts.** The script builds none. `scripts/build-web-audioworklet.sh` and every
  release build of a shipped artifact still get `panic = "abort"` exactly as before; the artifact
  digests are unchanged by the script's existence.
- **Per-package release invocations.** The many `cargo test --locked --release -p <pkg>` gate legs
  select one package's targets, so they never put two panic variants of a clobbering lib unit in
  the same run. They are unaffected and are deliberately left un-overridden — they keep measuring
  and testing D12's shipped codegen.

**Deferred, owner decision.** The structural fix is to move `panic = "abort"` off
`[profile.release]` onto a separate `dist` profile, leaving `release` unwinding. That would remove
the dual-variant build entirely, but `[profile.bench]` inherits `release`, so it would also change
D12's "a benchmark measures the shipped code" intent — benchmarks would stop measuring the panic
strategy the artifact ships. That trade is not made here.

## Issue 083 boot attestation and the cross-target gate runtime

**Boot attestation.** Master plan D4 removes runtime SIMD dispatch: the instruction set is chosen
at compile time and there is no scalar fallback to fall back *to*. Every entry point that can start
an engine therefore calls `lane::attest_host` once, on the control plane, before any
render state exists, and refuses to start on an error:

| entry point | on failure |
|---|---|
| `crates/capi` `miso_engine_v1_engine_create` | `MISO_ENGINE_V1_UNSUPPORTED` (7) |

The native and mobile bootstrap shells (`hosts/host-native`, `hosts/host-mobile`) also called it
until #1032 removed them: nothing linked them, and a native iOS or Android app embeds the engine
through the C ABI, so `engine_create` is the one native entry point.

The C header previously said `MISO_ENGINE_V1_UNSUPPORTED` was reserved and never returned; it is
now returned by that one entry point and the header says so. An embedder that receives it must not
retry: the library and the CPU do not match. On every supported host the attestation succeeds and
nothing about these calls changes. `hosts/host-web` is `wasm32`, where the pinned
instruction set is a whole-artifact build flag rather than a CPU property, so the attestation is a
compile-time no-operation there and no call is added.

**The gate runtime.** `tools/wasm-gates` depends on `wasmtime = "=47.0.3"`
(Apache-2.0 WITH LLVM-exception, `default-features = false`, features `runtime`, `cranelift`,
`std`). It is dev/tooling: it links into no shipped artifact, and no engine crate, host or fixture
may reach a WebAssembly runtime — AGENTS.md is explicit that a render callback never invokes one.
The version is pinned exactly because a runtime upgrade may change which post-MVP proposals
validate, and rejecting a module that uses one is part of what the gate does: the runner sets
`wasm_relaxed_simd(false)`, so a build that emits a relaxed instruction fails validation instead of
returning a digest. That distinction is load-bearing and was measured — wasmtime 47 lowers
`f32x4.relaxed_madd` to a hardware `vfmadd` on an x86 host, so the relaxed instruction *agreed*
with the exact software FMA there. Agreement on one runtime's lowering choice is not a property of
the program, which is why D3 forbids the opcode rather than testing its result.

## Issue 004 control-plane parser dependencies

`session` is not render-reachable. It depends one way on `engine` only for
the checked `SampleRateHz` and `QuantumFrames` value carriers; core does not depend on session.
Parsing, canonicalization, validation, model cloning, sorting, indexes, and all failure allocation
remain on the control plane.

The direct parser dependency is exact-pinned `json-syntax = 0.12.5` with default features disabled.
The session crate has no runtime `serde`, `serde_json`, or TOML dependency. A bounded preflight
rejects excess depth and decoded duplicate keys before the dependency constructs a value tree;
the typed walk then consumes `json-syntax` values and byte spans. Canonical output is produced by
the audited schema-specific writer, never by a dependency display implementation.

**Issue #381 ruling, 2026-09-04:** `session` depended on `jstrict 0.14.0` from #338 through #380,
a single-owner performance fork of `json-syntax` created 2026-07-25 ("Semantics are unchanged: same
strictness, same `Value`, same `CodeMap`" per its own README). The audit behind #377 (Part 2,
finding 4) measured `jstrict` at 296 total crates.io downloads against `json-syntax`'s 3.45M (701k
in the last 90 days) and counted 26 `unsafe` sites across the fork's 10.3k lines. The owner ruled
the engine will not ship a parser at that popularity/unsafe-footprint ratio when an API-identical
upstream exists, and #381 replaced the dependency with upstream `json-syntax 0.12.5` (MIT OR
Apache-2.0, last released 2024-07-03) without any change to `parse.rs`'s typed walk, `CodeMap`
byte spans, or the exact-`f32` lexeme path -- `crates/session/src/parse.rs`'s `Value::Number(_) =>
match parse_f32_token(&self.source[span.0..span.1])` already read the original source slice by
`CodeMap` span rather than any dependency-owned number buffer, on both the fork and upstream. Do
not reintroduce `jstrict`, `json-escape-simd`, or `memchr` into the session dependency graph: the
fork's SIMD scanning and SIMD string-escape printing were the only load-bearing performance
additions, and canonical printing was never their consumer -- `session` writes its own audited
canonical form and never calls a dependency's printer.

Parser allocation and diagnostic formatting are expected control-plane behavior. Malformed input
returns typed diagnostics; arithmetic and configured-cap preflight runs before the canonical string,
model clone, normalized indexes, or downstream plan work. JSON string and duplicate-key
fixtures, the strict unknown-key matrix, target compilation, and parser/compiler fuzz targets are
the compatibility and failure-mode evidence for this dependency choice. `json-syntax`'s object
indexing goes through `hashbrown` 0.12's default hasher, `ahash` 0.7's `RandomState`, which on its
first construction in a process lazily allocates 88 bytes of process-lifetime heap and seeds from
OS entropy per process -- harmless to every shipped cap (they are explicit row sums), but relevant
to any exact allocator oracle that arms across the first object parse in a process (see
`crates/capi/tests/resource_lifecycle.rs`).

The earlier issue-004 cross-target archive sizes are historical measurements for the retired
parser stack and are not projected onto the JSON implementation. Current size and allocation
evidence is recorded by issue #338; it remains descriptive rather than a render-plane allowance.

## Audit #103 shared host preparation

`host-core` is control-plane only, like `session`: it parses, compiles,
allocates the prepared plan and the source rings, and is never reachable from render. It contains
no `unsafe` code and exports no C symbol -- it is a plain `rlib`, because a `cdylib` re-exports
every `no_mangle` symbol it links, and a facade carrying them would push the C ABI's fifteen
exports into the browser artifact. Its default graph deliberately does not depend on `protocol`:
the issue #369 `control-provider` adapter is an optional, non-default feature enabled only by
`capi`, so the browser and every host that does not speak the protocol pay for neither its code nor
its dependencies. `scripts/check-host-core-policy.sh` enforces this exact edge and rejects a
mandatory or default-enabled protocol dependency, with mutation coverage in
`scripts/test-host-core-policy.sh`.

## Issue 011 runtime boundary

`effect-contract` is render-reachable and depends only on `engine`; it has no
parser, hashing, package, filesystem, network, logging, or synchronization dependency.
Issue 011's `effect-compiler` is control-plane only and depends on core, session and the
render-reachable contract; it is not reachable from process. Issue 029's `effect-package` crate,
the control-plane `sha2` package/artifact/state hashing it carried, and its state migration were
removed by #1037 (owner rulings R6a and R6b); nothing in the shipped closure hashes packages.

## Audit #84 phase D / #105 phase 2: the render-audit instrumentation never ships

`engine`'s `realtime-audit` feature compiles the thread-local depth guard that
`in_render_scope` arms and that the counting allocators report to. It is evidence machinery, and it
must not reach a shippable artifact. Three independent statements enforce that, and each one is
necessary because the other two do not imply it:

1. **Manifests.** Only `tools/*` binaries and `[dev-dependencies]` may enable the feature.
   `conformance` *forwards* it (`[features] realtime-audit =
   ["engine/realtime-audit"]`) instead of hard-enabling it, so a regular dependent never
   receives the instrumentation unless it asks. `scripts/check-realtime-audit-leak.sh` checks both
   the manifest sections and the resolved graph (`cargo tree -e features,no-dev --target all`) of
   every package under `crates/` and `hosts/` (each ships, so its production
   graph must resolve without the feature too).
2. **Invocations.** Cargo unifies features across the packages selected by *one* invocation, so a
   clean per-package graph does not make a multi-package build clean. CI builds host artifacts in
   an invocation that lists no evidence crate; the evidence crates keep their cross-target compile
   coverage in a separate step, and `scripts/check-artifact-evidence-leak.sh` gates both halves.
   This is what makes the artifact independent of rule 1 continuing to hold: with the feature
   temporarily restored under conformance's `[dependencies]`, the pre-#105 combined wasm list
   resolves the feature and the host-only list does not.
3. **Consumers.** A test binary that runs `conformance::run_effect_conformance` must
   arm the scope *and* install the workspace's one audited `GlobalAlloc`
   (`bench_support::alloc`, #104 phase B) in count-and-continue mode. The harness
   proves both before it judges any effect and reports `harness.audit_unarmed` or
   `harness.allocator_not_installed` rather than a vacuous pass, so the allocation gate cannot
   silently become decorative. `scripts/check-bench-policy.sh` allows that dev edge from `crates/`
   and nothing else.

`process.allocation` is therefore a real measurement (a global allocator, on the consumer's side).
`process.lock`, `process.log`, `process.io` and `process.feature_detection` are hook reports: an
effect that calls a raw `Mutex::lock` or `println!` is caught by the syscall trace
(`scripts/trace-effect-contract-audit.sh`), not by the harness.
