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

## Unsafe-code ownership

The workspace denies unsafe code. If a later approved issue needs a narrow exception, it is limited
to `crates/miso-engine-core/src/realtime/spsc.rs` for the issue-003 SPSC slot protocol,
`crates/miso-engine-core/src/arch` for auditable architecture intrinsics or
`crates/miso-engine-capi/src/ffi` for ABI boundaries. Issue 083 adds
`crates/miso-engine-lane/src/softfma.rs`, the one file of the lane crate that carries unsafe: the
wasm `simd128` promote/demote intrinsics of the software FMA, and the `x86` MXCSR read/write that
gate G6 uses to prove hardware flush-to-zero is inert under the D7 flush law (the workspace forbids
inline assembly, so the deprecated `_mm_getcsr`/`_mm_setcsr` intrinsics are used instead). Neither
is reachable from a render path, and no `Lane` value or vector type escapes the crate as unsafe. The introducing issue must use a local,
minimal lint allowance; state the invariant next to the operation; include a `SAFETY` explanation;
add tests; and obtain explicit review. Unsafe code must not leak through a public API. The SPSC
exception owns fixed `UnsafeCell<MaybeUninit<T>>` storage and its local `SAFETY` assertions
require one producer, one consumer, release publication after writes, acquire before reads, and
shared `Arc` storage outliving both non-cloneable endpoints. `Arc` creation/destruction stays
outside push, pop, and render. A second test-only exception is
`tools/miso-engine-realtime-audit/src/main.rs`, whose audited global allocator forwards unchanged
layouts to `System` and terminates without unwinding if allocation/free is attempted in render.
Loom `=0.7.2` is MIT licensed and test/model-only; it is not a production, Wasm, or
render-reachable dependency. Issue 005 additionally permits only
`tools/miso-engine-protocol-audit/src/main.rs` to locally allow unsafe code for its audit-only
global allocator. That allocator forwards original pointer/layout contracts unchanged to `System`
and counts only allocations while the audit thread is armed; its prepared corpus, queues, and
output/scratch buffers exist before arming. It is not linked into a production crate and does not
change protocol allocation behavior.

Issue 005 also permits `tools/miso-engine-protocol-bench/src/main.rs` to locally allow unsafe
code for its comparison-only allocation counter. It forwards original allocator contracts to
`System` and records requested allocation count/bytes only while a native host-harness interval is
armed. The preallocated BTLV output, decode scratch, and official FlatBuffers builder are prepared
before that interval. `flatbuffers = 25.12.19` is an Apache-2.0, tool-only dependency with no
engine, protocol, browser-host, or render-reachable target impact.

The source-policy checker currently accepts unsafe syntax in exactly four source files:
`crates/miso-engine-core/src/realtime/spsc.rs`,
`tools/miso-engine-realtime-audit/src/main.rs`, and
`tools/miso-engine-protocol-audit/src/main.rs`, and
`tools/miso-engine-protocol-bench/src/main.rs`. The latter two are the only Issue-005
audit/benchmark exceptions; no sibling source file in either tool is permitted to use unsafe code.

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
