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

## Issue 100 worker idle policy and the single wake

Issue 100 makes the native dependency-wave workers park between blocks instead of burning a core
each, and creates exactly one documented exception to the "no syscalls" rule above.

* The coordinator (the audio callback thread) may issue **at most one** `std::thread::unpark` per
  rendered block, from `NativeSchedulerV1::wake_root`, and only when worker 0 is observed parked.
  On Linux/glibc that is one `futex(FUTEX_WAKE)`. It is the only syscall any render-reachable
  coordinator code is permitted to make; everything else in the prohibitions above still holds.
* Auxiliary workers wake their own binary-tree children (`2i+1`, `2i+2`), so one coordinator wake
  reaches every issued worker. A worker may `park` and `unpark` (futex wait/wake) and nothing
  else: it allocates nothing, takes no lock, and performs no I/O or logging.
* A worker never parks while its block is open -- that is what makes "at most one coordinator wake
  per rendered block" structural rather than statistical: `wake_root` can only ever find a parked
  worker on a block's first wave. (A runaway guard of many quanta still bounds the spin if a
  coordinator never closes its block.) After the block closes the worker spins a short linger
  budget and then parks; the count restarts at each transition. A host
  rendering at the real block cadence leaves nearly a whole quantum between blocks, so the worker
  parks and the next block costs exactly one coordinator wake; an offline or saturated render
  re-opens the next block within microseconds, so the worker never parks and the render thread
  makes no syscall at all. `scripts/trace-scheduler-audit.sh` gates both: in steady (unpaced) mode
  the coordinator's armed interval contains **at most one** `futex` (the pool has been parked since
  preparation, so the first block wakes it and nothing after that does), and in paced mode at most
  one `futex` per block, never more than the executor's own `coordinator_wakes` counter. Worker
  idle CPU is measured from `/proc/<tid>/stat` and gated at 5 % between blocks.
* The wake protocol is a Dekker pair: the worker stores `parked = true`, fences `SeqCst`, then
  re-checks its command queue; the coordinator publishes the command, fences `SeqCst`, then reads
  `parked`. Removing or weakening either fence is a lost-wake bug.
* `AGENTS.md` records the same exception in one sentence next to the render prohibitions.

Bounded recovery replaces the old unbounded completion spin: `recover_issued` spins for a
calibrated budget -- one render quantum, never less than `MINIMUM_RECOVERY_NS` -- and then declares
the worker dead *for that block*. The dead worker's parcel stays *trapped* (it is never touched
again until the worker returns it at a later block boundary), every edge sourced from it is muted
to the arena's always-zero silence buffer, and the remaining units of the block render on the
coordinator. The audio callback therefore has a bounded worst case and never wedges.

The #100 plan proposed half a quantum and a permanent death. Both were widened, in the safe
direction, by what parking makes true: the first wave of a block also pays the operating system's
wake latency, which the quantum does not bound, and a worker that answers late is demonstrably
alive, so a false deadline miss must cost one degraded block rather than the life of the lease.
`NativeSchedulerConfigV1::with_recovery_deadline_ns` lets a qualification harness that measures
determinism rather than deadlines opt out of the derived budget entirely.

## Unsafe-code ownership

The workspace denies unsafe code. If a later approved issue needs a narrow exception, it is limited
to `crates/miso-engine-core/src/realtime/spsc.rs` for the issue-003 SPSC slot protocol or
`crates/miso-engine-capi/src/ffi` for ABI boundaries. (`crates/miso-engine-core/src/arch` was a
third such owner until #84 phase A deleted it: the per-target kernels moved to
`crates/miso-engine-lane`, and the exemption was removed from
`scripts/check-realtime-policy.sh` in the same change.) Issue 083 adds
`crates/miso-engine-lane/src/softfma.rs`, the first file of the lane crate that carries unsafe: the
wasm `simd128` promote/demote intrinsics of the software FMA, and the `x86` MXCSR read/write that
gate G6 uses to prove hardware flush-to-zero is inert under the D7 flush law (`_mm_getcsr`/
`_mm_setcsr` are used rather than the inline assembly their deprecation note recommends). No `Lane`
value or vector type escapes the crate as unsafe.

Issue 146 adds the second: `crates/miso-engine-lane/src/fpenv.rs`, the canonical floating-point
environment that every native render entry pins. It is the one place in the workspace that **is**
reachable from a render path, deliberately -- pinning the environment is the render entry's first
act and unpinning it is its last -- and it is three register accesses and two empty assembly blocks,
with no memory operand, no call and no branch. It carries unsafe for two reasons, and both are
inline assembly -- the workspace's only inline assembly, and the reason the softfma paragraph above
says "rather than the inline assembly their deprecation note recommends" and this one does not.

First, AArch64's `mrs`/`msr FPCR` pair, because the standard library exposes no stable FPCR
intrinsic (Arm Architecture Reference Manual for A-profile, `FPCR`, Floating-point Control
Register). On `x86` there is no counterpart: `fpenv.rs` calls `softfma.rs`'s already-approved MXCSR
helpers. The blocks write only a value previously read from the same thread or the architectural
default, and affect no other thread.

Second, on both, an empty `asm!` block as the guard's scheduling barrier. Installing a control word
is a side effect the optimizer does not model -- `_mm_setcsr` lowers to an intrinsic declared as
touching only its own argument's memory -- so without a barrier nothing stops a computation being
scheduled outside the region it was meant to run in. The empty block is deliberately **not**
`nomem`: being a memory clobber is its entire purpose, and it emits no instructions. It anchors
every memory-dependent computation, which is every render; it does not anchor a value held entirely
in registers, and `crates/miso-engine-lane/tests/fp_env.rs` proves that limit rather than assuming
it, with a register-only product that a release build really does schedule outside the guard until
the test anchors it itself.

The exemption is the file, not the crate:
`scripts/check-realtime-policy.sh` and `scripts/check-lane-policy.sh` both name `fpenv.rs`
explicitly and both have a mutation test proving a third lane file does not inherit it. The introducing issue must use a local,
minimal lint allowance; state the invariant next to the operation; include a `SAFETY` explanation;
add tests; and obtain explicit review. Unsafe code must not leak through a public API. The SPSC
exception owns fixed `UnsafeCell<MaybeUninit<T>>` storage and its local `SAFETY` assertions
require one producer, one consumer, release publication after writes, acquire before reads, and
shared `Arc` storage outliving both non-cloneable endpoints. `Arc` creation/destruction stays
outside push, pop, and render. Issue 100 adds `crates/miso-engine-core/src/realtime/disjoint.rs`, the plan-owned disjoint audio
arena that lets each consuming parcel read its producers' buffers in place on the worker that
needs them instead of having the coordinator copy every inter-parcel edge. Its `unsafe impl Sync`
and its raw slice construction are justified by four invariants stated in the module
documentation and proved once at bind by `ArenaLeaseSetBuilder::finish`: **I1** every buffer is
writable by at most one lease for the life of the plan (buffers are never recycled), **I2** a
lease reads only buffers produced by a strictly earlier wave or by itself, **I3** the scheduler
runs one wave at a time with an SPSC release/acquire edge between waves, and **I4** a parcel the
coordinator does not own is never read (its buffers are muted to the always-zero silence buffer).
Soundness therefore does not depend on any worker being on time: a late worker can only write its
own unique slots, which nobody reads. `crates/miso-engine-native-scheduler` and
`crates/miso-engine-graph` remain entirely free of unsafe code, which
`scripts/check-scheduler-policy.sh` enforces. A second test-only exception is
`tools/miso-engine-audit/src/realtime.rs`, whose audited global allocator forwards unchanged
layouts to `System` and terminates without unwinding if allocation/free is attempted in render.
Loom `=0.7.2` is MIT licensed and test/model-only; it is not a production, Wasm, or
render-reachable dependency. Issue 005 additionally permits only
`tools/miso-engine-audit/src/protocol.rs` to locally allow unsafe code for its audit-only
global allocator. That allocator forwards original pointer/layout contracts unchanged to `System`
and counts only allocations while the audit thread is armed; its prepared corpus, queues, and
output/scratch buffers exist before arming. It is not linked into a production crate and does not
change protocol allocation behavior.

Issue 005 also permits `tools/miso-engine-bench/src/protocol.rs` to locally allow unsafe
code for its comparison-only allocation counter. It forwards original allocator contracts to
`System` and records requested allocation count/bytes only while a native host-harness interval is
armed. The preallocated BTLV output, decode scratch, and official FlatBuffers builder are prepared
before that interval. `flatbuffers = 25.12.19` is an Apache-2.0, tool-only dependency with no
engine, protocol, browser-host, or render-reachable target impact.

The source-policy checker currently accepts unsafe syntax in exactly four source files:
`crates/miso-engine-core/src/realtime/spsc.rs`,
`tools/miso-engine-audit/src/realtime.rs`, and
`tools/miso-engine-audit/src/protocol.rs`, and
`tools/miso-engine-bench/src/protocol.rs`. The latter two are the only Issue-005
audit/benchmark exceptions; no sibling source file in either tool is permitted to use unsafe code.

That sentence has fallen behind `scripts/check-realtime-policy.sh`, whose exemption list has grown
with each approved issue and is the authority; the script, not this paragraph, is what CI runs.
Reconciling the two belongs to the #104 evidence triage. Two categories have been added since:
the C-ABI boundary files (`crates/miso-engine-capi/src/ffi.rs`,
`crates/miso-engine-effect-package/src/ffi.rs`, `hosts/miso-engine-host-web/src/ffi.rs` and their
tests), and **test-only counting global allocators** — `miso-engine-builtins-compiler`,
`miso-engine-effect-package` and, from audit #92, `miso-engine-transient-shaper`. The last category
is `unsafe impl GlobalAlloc` that forwards every request to `System` unchanged and adds two relaxed
atomic counters, in a `tests/` file that no production target links; it exists to *prove* the render
path allocates nothing, which is the policy this document states.

`scripts/check-realtime-policy.sh` extracts explicitly marked render-reachable regions and rejects
allocation/growth, locks, waits, I/O, logging, networking, process/thread APIs, and async surfaces.
Its mutation suite proves allocation, lock, log, and unsafe-scope violations are rejected, including
unsafe code adjacent to but outside the exact protocol-audit `main.rs` allowlist. Runtime mutation
probes separately prove the realtime allocator/deallocator and forbidden-operation hooks are armed.

## CPU and Wasm policy

CPU ISA selection is not a Cargo feature. Issue 083 (master plan D4) replaces the earlier runtime
capability model on x86: native `x86_64` builds are pinned to `x86-64-v3` by the workspace
`.cargo/config.toml` (`-C target-feature=+avx2,+fma`), `crates/miso-engine-lane` refuses to compile
without both features, and every host and C-ABI entry attests the CPU once at boot through
`miso_engine_lane::attest_host`, refusing to start rather than falling back silently. That pin is
the only approved global ISA configuration and `scripts/check-workspace-policy.sh` admits exactly
it; `-C target-cpu`, a global `[build]` rustflags table and any other feature set stay forbidden.
NEON is baseline on AArch64. Browser Wasm baseline and `simd128` are separate artifacts; relaxed
SIMD is forbidden and correctness cannot depend on it (`scripts/check-lane-policy.sh`). Fusion
exists only where `Lane::fma` is written (D3): hardware FMA on x86 and NEON, and an exact software
FMA on wasm that gate G3 proves bit-identical to the hardware instruction. Intrinsics live only in
`crates/miso-engine-lane`; the session's semantics stay target-independent, and cross-backend and
cross-target equality is `to_bits` identity, not a tolerance (D5).

Issue 003 concurrent queues use only pointer-width atomic loads/stores. Rust guarantees every
available standard atomic type is lock-free, and the supported native/mobile targets expose
`target_has_atomic="ptr"`. Counters are endpoint-local plain integers, so render performs no atomic
read-modify-write retry. Browser launch uses `LocalRing` on its single render agent; the inspected
baseline Wasm object contains no atomic opcode and makes no cross-agent shared-memory claim.

## Panic behaviour by profile

Issue 083 (master plan D12) gives the workspace one release profile: `lto = "fat"`,
`codegen-units = 1`, `panic = "abort"`, `debug = 1`, with `[profile.bench]` inheriting it so a
benchmark measures the shipped code. Fat LTO and a single codegen unit are what let a consumer's
instantiation of the `#[inline(always)]` generic kernel bodies in `crates/miso-engine-lane` collapse
into the intended straight-line loop; `debug = 1` keeps line tables so a profile or a core dump
names a kernel, and costs build time and artifact size, never speed.

`panic = "abort"` is a deliberate, user-visible change and is recorded here rather than hidden:

- A release build has **no unwinding**. `std::panic::catch_unwind` still compiles and still returns
  `Ok` on the normal path, but it can no longer contain a panic: the process aborts instead. The
  affected boundaries are `crates/miso-engine-capi/src/ffi.rs` (`catch_result`, `catch_destroy`,
  which map a contained panic to `RESULT_INTERNAL`), `hosts/miso-engine-host-web/src/ffi.rs`, the
  `catch_unwind` probes inside `crates/miso-engine-conformance`, and the `panic_unwinds` counter in
  `tools/miso-engine-bench`. In a release artifact each of those is a diagnostic that no longer
  fires; none of them is load-bearing for a call that does not panic, so behaviour on a passing host
  is unchanged.
- Embedders must read this as: **the C ABI does not convert a panic into `RESULT_INTERNAL` in a
  release build of `libmiso_engine_capi`.** A panic is an engine defect, and unwinding across a C or
  Wasm frame is undefined by either ABI, so aborting is the honest contract. `RESULT_INTERNAL` stays
  in the ABI for the internal failures that are returned, not thrown.
- The browser artifacts built by `scripts/build-web-audioworklet.sh` are `--release` builds and
  therefore inherit `panic = "abort"`: a panic inside the AudioWorklet traps the module instead of
  returning `RESULT_INTERNAL` to the worklet shim.
- Cargo ignores the `panic` setting when it builds a test or benchmark harness, so
  `cargo test --release` (including the Loom race model and every gate that runs in release) still
  unwinds and `#[should_panic]` still works. That was verified by inspecting the `rustc`
  command lines: a release binary is compiled with `-C panic=abort`, a release test harness is not.

## Issue 083 boot attestation and the cross-target gate runtime

**Boot attestation.** Master plan D4 removes runtime SIMD dispatch: the instruction set is chosen
at compile time and there is no scalar fallback to fall back *to*. Every entry point that can start
an engine therefore calls `miso_engine_lane::attest_host` once, on the control plane, before any
render state exists, and refuses to start on an error:

| entry point | on failure |
|---|---|
| `hosts/miso-engine-host-native` `main` | diagnostic on stderr, `ExitCode::FAILURE` |
| `hosts/miso-engine-host-mobile` `mobile_target_smoke` | `Err(HostAttestation)` |
| `crates/miso-engine-capi` `miso_engine_v2_engine_create` | `MISO_ENGINE_V2_UNSUPPORTED` (7) |

The C header previously said `MISO_ENGINE_V2_UNSUPPORTED` was reserved and never returned; it is
now returned by that one entry point and the header says so. An embedder that receives it must not
retry: the library and the CPU do not match. On every supported host the attestation succeeds and
nothing about these calls changes. `hosts/miso-engine-host-web` is `wasm32`, where the pinned
instruction set is a whole-artifact build flag rather than a CPU property, so the attestation is a
compile-time no-operation there and no call is added.

**The gate runtime.** `tools/miso-engine-wasm-gates` depends on `wasmtime = "=47.0.3"`
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

`miso-engine-session` is not render-reachable. It depends one way on `miso-engine-core` only for
the checked `SampleRateHz` and `QuantumFrames` value carriers; core does not depend on session.
Parsing, canonicalization, validation, model cloning, sorting, indexes, and all failure allocation
remain on the control plane.

The direct parser dependencies are `serde = 1.0.228` with `derive`, and the Cargo requirement
`toml = 0.9.9`, which resolves in `Cargo.lock` to package version
`0.9.9+spec-1.0.0`. The latter suffix is package version metadata, not a Cargo feature. TOML default
features are disabled; only `parse` and `serde` are enabled. In particular `display` is excluded:
canonical output is produced by the audited schema-specific writer, never by a dependency display
implementation. Both direct packages are dual MIT/Apache-2.0 licensed, support the workspace's
native/mobile/browser targets through Rust `std`, and expose no runtime behavior to render.

Parser allocation and diagnostic formatting are expected control-plane behavior. Malformed input
returns typed diagnostics; arithmetic and configured-cap preflight runs before the canonical string,
model clone, normalized indexes, or downstream plan work. TOML 1.0 string and duplicate-key
fixtures, the strict unknown-key matrix, target compilation, and parser/compiler fuzz targets are
the compatibility and failure-mode evidence for this dependency choice.

The issue-004 cross-target release build recorded the session crate's Wasm `rlib` at 1,232,588
bytes for scalar and 1,222,746 bytes for `simd128`; these are descriptive archive sizes, not linked
host-binary size claims or acceptance thresholds. Android and iOS `cargo check` metadata artifacts
were 302,440 and 302,436 bytes respectively. Dependency feature-tree evidence confirms TOML
`parse`/`serde` only and no `display` feature.

## Audit #103 shared host preparation

`miso-engine-host-core` is control-plane only, like `miso-engine-session`: it parses, compiles,
allocates the prepared plan and the source rings, and is never reachable from render. It contains
no `unsafe` code and exports no C symbol -- it is a plain `rlib`, because a `cdylib` re-exports
every `no_mangle` symbol it links, and a facade carrying them would push the C ABI's fifteen
exports into the browser artifact. It deliberately does not depend on `miso-engine-protocol`: the
control protocol is a host-specific transport, and a host that does not speak it does not pay for
it. `scripts/check-host-core-policy.sh` enforces all of this, with mutation coverage in
`scripts/test-host-core-policy.sh`.

## Issue 011 runtime boundary and issue 029 package hashing dependency

`miso-engine-effect-contract` is render-reachable and depends only on `miso-engine-core`; it has no
parser, hashing, package, filesystem, network, logging, or synchronization dependency.
`miso-engine-effect-package` is control-plane-only and uses `sha2 = 0.11.0` with default features
disabled for deterministic SHA-256 package/artifact/state identity. `sha2` is dual MIT/Apache-2.0,
pure Rust for these targets, and failure yields typed package/state rejection before any prepared
processor can be published. The package crate and hashing evidence are provisional issue-029 work,
not issue-011 acceptance evidence. Issue 011's new `miso-engine-effect-compiler` is control-plane
only and depends on core, session and the render-reachable contract; neither compiler nor package
crate is reachable from process. The resolved `sha2` feature tree and archive-size delta must be
re-reviewed by issue 029; no package claim applies to the render dependency graph.

## Audit #84 phase D / #105 phase 2: the render-audit instrumentation never ships

`miso-engine-core`'s `realtime-audit` feature compiles the thread-local depth guard that
`in_render_scope` arms and that the counting allocators report to. It is evidence machinery, and it
must not reach a shippable artifact. Three independent statements enforce that, and each one is
necessary because the other two do not imply it:

1. **Manifests.** Only `tools/*` binaries and `[dev-dependencies]` may enable the feature.
   `miso-engine-conformance` *forwards* it (`[features] realtime-audit =
   ["miso-engine-core/realtime-audit"]`) instead of hard-enabling it, so a regular dependent never
   receives the instrumentation unless it asks. `scripts/check-realtime-audit-leak.sh` checks both
   the manifest sections and the resolved graph (`cargo tree -e features,no-dev --target all`) of
   every package under `crates/` and `hosts/`.
2. **Invocations.** Cargo unifies features across the packages selected by *one* invocation, so a
   clean per-package graph does not make a multi-package build clean. CI builds host artifacts in
   an invocation that lists no evidence crate; the evidence crates keep their cross-target compile
   coverage in a separate step, and `scripts/check-artifact-evidence-leak.sh` gates both halves.
   This is what makes the artifact independent of rule 1 continuing to hold: with the feature
   temporarily restored under conformance's `[dependencies]`, the pre-#105 combined wasm list
   resolves the feature and the host-only list does not.
3. **Consumers.** A test binary that runs `miso_engine_conformance::run_effect_conformance` must
   arm the scope *and* install the workspace's one audited `GlobalAlloc`
   (`miso_engine_bench_support::alloc`, #104 phase B) in count-and-continue mode. The harness
   proves both before it judges any effect and reports `harness.audit_unarmed` or
   `harness.allocator_not_installed` rather than a vacuous pass, so the allocation gate cannot
   silently become decorative. `scripts/check-bench-policy.sh` allows that dev edge from `crates/`
   and nothing else.

`process.allocation` is therefore a real measurement (a global allocator, on the consumer's side).
`process.lock`, `process.log`, `process.io` and `process.feature_detection` are hook reports: an
effect that calls a raw `Mutex::lock` or `println!` is caught by the syscall trace
(`scripts/trace-effect-contract-audit.sh`), not by the harness.
