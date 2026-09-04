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

What survives, because the *sequential* executor depends on it: the plan-owned disjoint audio
arena in `crates/engine/src/realtime/disjoint.rs` and its lease API. Invariants I1 and
I2 are still proved at bind by `ArenaLeaseSetBuilder::finish`, and they are still what makes node
semantics have exactly one implementation of *where the audio is*. I3 and I4 described a
multi-wave issue discipline and a coordinator that could decline to own a parcel; with one
executor and one lease over the whole coloured arena, both are trivially satisfied rather than
enforced.

## Unsafe-code ownership

The workspace denies unsafe code. If a later approved issue needs a narrow exception, it is limited
to `crates/engine/src/realtime/spsc.rs` for the issue-003 SPSC slot protocol or
`crates/capi/src/ffi` for ABI boundaries. (`crates/engine/src/arch` was a
third such owner until #84 phase A deleted it: the per-target kernels moved to
`crates/lane`, and the exemption was removed from
`scripts/check-realtime-policy.sh` in the same change.) Issue 083 adds
`crates/lane/src/softfma.rs`, the first file of the lane crate that carries unsafe: the
wasm `simd128` promote/demote intrinsics of the software FMA, and the `x86` MXCSR read/write that
gate G6 uses to prove hardware flush-to-zero is inert under the D7 flush law (`_mm_getcsr`/
`_mm_setcsr` are used rather than the inline assembly their deprecation note recommends). No `Lane`
value or vector type escapes the crate as unsafe.

Issue 146 adds the second: `crates/lane/src/fpenv.rs`, the canonical floating-point
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
in registers, and `crates/lane/tests/fp_env.rs` proves that limit rather than assuming
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
outside push, pop, and render. Issue 100 added `crates/engine/src/realtime/disjoint.rs`, the plan-owned disjoint audio
arena, which the sequential executor still renders through. Its `unsafe impl Sync` and its raw
slice construction are justified by invariants stated in the module documentation and proved once
at bind by `ArenaLeaseSetBuilder::finish`: **I1** every buffer is writable by at most one lease for
the life of the plan (buffers are never recycled) and **I2** a lease reads only buffers produced
strictly earlier or by itself. I3 and I4 constrained the removed multi-wave scheduler and are now
trivially satisfied: there is one executor holding one lease. `crates/graph` remains
entirely free of unsafe code. A second test-only exception is
`tools/audit/src/realtime.rs`, whose audited global allocator forwards unchanged
layouts to `System` and terminates without unwinding if allocation/free is attempted in render.
Loom `=0.7.2` is MIT licensed and test/model-only; it is not a production, Wasm, or
render-reachable dependency. Issue 005 additionally permits only
`tools/audit/src/protocol.rs` to locally allow unsafe code for its audit-only
global allocator. That allocator forwards original pointer/layout contracts unchanged to `System`
and counts only allocations while the audit thread is armed; its prepared corpus, queues, and
output/scratch buffers exist before arming. It is not linked into a production crate and does not
change protocol allocation behavior.

Issue 005 also permits `tools/bench/src/protocol.rs` to locally allow unsafe
code for its comparison-only allocation counter. It forwards original allocator contracts to
`System` and records requested allocation count/bytes only while a native host-harness interval is
armed. The preallocated BTLV output, decode scratch, and official FlatBuffers builder are prepared
before that interval. `flatbuffers = 25.12.19` is an Apache-2.0, tool-only dependency with no
engine, protocol, browser-host, or render-reachable target impact.

The source-policy checker currently accepts unsafe syntax in exactly four source files:
`crates/engine/src/realtime/spsc.rs`,
`tools/audit/src/realtime.rs`, and
`tools/audit/src/protocol.rs`, and
`tools/bench/src/protocol.rs`. The latter two are the only Issue-005
audit/benchmark exceptions; no sibling source file in either tool is permitted to use unsafe code.

That sentence has fallen behind `scripts/check-realtime-policy.sh`, whose exemption list has grown
with each approved issue and is the authority; the script, not this paragraph, is what CI runs.
Reconciling the two belongs to the #104 evidence triage. Two categories have been added since:
the C-ABI boundary files (`crates/capi/src/ffi.rs`,
`crates/effect-package/src/ffi.rs`, `hosts/host-web/src/ffi.rs` and their
tests), and **test-only counting global allocators** — `builtins-compiler`,
`effect-package`, from audit #92 `transient-shaper`, and from issue #240
`hosts/host-web/tests/boot_transient_budget.rs`. The last category is
`unsafe impl GlobalAlloc` that forwards every request to `System` unchanged and adds only audit
counters, in a `tests/` file that no production target links. The earlier fixtures prove render
paths allocate nothing; #240's thread-local fixture measures the parse/model-build high-water mark
against its pinned conservative multiplier.

`scripts/check-realtime-policy.sh` extracts explicitly marked render-reachable regions and rejects
allocation/growth, locks, waits, I/O, logging, networking, process/thread APIs, and async surfaces.
Its mutation suite proves allocation, lock, log, and unsafe-scope violations are rejected, including
unsafe code adjacent to but outside the exact protocol-audit `main.rs` allowlist. Runtime mutation
probes separately prove the realtime allocator/deallocator and forbidden-operation hooks are armed.

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
read-modify-write retry. Browser launch uses `LocalRing` on its single render agent; the inspected
baseline Wasm object contains no atomic opcode and makes no cross-agent shared-memory claim.

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
(`libfoo.so`, `libfoo.a`). Three packages are in that shape:

| package | crate-types |
|---|---|
| `crates/capi` | `rlib`, `staticlib`, `cdylib` |
| `crates/effect-package` | `rlib`, `cdylib` |
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
  `CapacityUnarmed` — while the old assertion subtracted the `NoConsole` baseline and so charged
  observation for the cost of a console *existing*. It is rebaselined to assert the claim that is
  actually load-bearing: `armed <= unarmed_with_console * 1.10 + 50 µs`, i.e. arming is not
  measurably slower than an attached-but-unarmed console.
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
| `hosts/host-native` `main` | diagnostic on stderr, `ExitCode::FAILURE` |
| `hosts/host-mobile` `mobile_target_smoke` | `Err(HostAttestation)` |
| `crates/capi` `miso_engine_v1_engine_create` | `MISO_ENGINE_V1_UNSUPPORTED` (7) |

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
exports into the browser artifact. It deliberately does not depend on `protocol`: the
control protocol is a host-specific transport, and a host that does not speak it does not pay for
it. `scripts/check-host-core-policy.sh` enforces all of this, with mutation coverage in
`scripts/test-host-core-policy.sh`.

## Issue 011 runtime boundary and issue 029 package hashing dependency

`effect-contract` is render-reachable and depends only on `engine`; it has no
parser, hashing, package, filesystem, network, logging, or synchronization dependency.
`effect-package` is control-plane-only and uses `sha2 = 0.11.0` with default features
disabled for deterministic SHA-256 package/artifact/state identity. `sha2` is dual MIT/Apache-2.0,
pure Rust for these targets, and failure yields typed package/state rejection before any prepared
processor can be published. The package crate and hashing evidence are provisional issue-029 work,
not issue-011 acceptance evidence. Issue 011's new `effect-compiler` is control-plane
only and depends on core, session and the render-reachable contract; neither compiler nor package
crate is reachable from process. The resolved `sha2` feature tree and archive-size delta must be
re-reviewed by issue 029; no package claim applies to the render dependency graph.

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
   every package under `crates/`, `hosts/`, and `sidecars/` (a sidecar ships, so its production
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
