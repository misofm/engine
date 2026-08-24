# 003 Real-time memory, buffers, queues, and plan lifetime

## Outcome

Establish the memory ownership model that makes the render plane allocation-free and safe during transactional reconfiguration.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy, benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated `PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O, logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and 96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only. Source/engine mismatches have no implicit SRC. Output is PCM.

This issue is independently implementable only after its exact dependencies are complete. Its change must follow the Sol-approved brief → Terra attempt 1 with evidence → Sol adversarial review workflow; Sol may make at most two further revisions, then the work must be rescoped/rebriefed rather than weakening gates.

## Scope

Implement preallocated planar buffers, bounded SPSC queues/rings, parameter/event storage, structurally immutable render programs, exclusively owned preallocated mutable DSP state, and block-boundary plan publication/off-thread reclamation.

## Required public interfaces/contracts

`PreparedRenderPlan::render(&mut self, RenderIo, RenderTime)` has exclusive state access and no allocation-capable parameters; `PlanPublisher` transfers only fully validated plans; the render thread returns displaced plans through a bounded retirement queue and defers a swap if that queue is full; SPSC APIs expose capacity, generation, typed full/empty results, overflow/underrun counters and never block.

## Deliverables

Allocation-audit hooks, buffer arena, queue implementations, plan epoch/lifetime design, failure policy, and realtime coding lint/test.

## Explicit non-goals

Graph compilation, decoder implementation, host callbacks, or arbitrary MPMC queues in the render path.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix

## Hazards/decisions

No allocations/frees, locks, I/O, network, logging, syscalls, or unbounded calls are permitted during render. Apple render guidance: https://developer.apple.com/documentation/audiotoolbox/auaudiounit/renderblock.

## Acceptance gates with objective measurements

Instrumented one-million-block render, including accepted and deferred plan-swap attempts, makes exactly 0 alloc/free, lock, log, file/network I/O or unexpected syscall calls; deliberately panicking allocator/deallocator/lock/log hooks are armed inside the render scope and native syscall tracing covers the reference test; concurrent plan swaps preserve exactly one complete plan per block; replaced plans are observed being destroyed only on a control/retirement thread; ring full/empty/wraparound tests never block and return typed results while incrementing counters.

## Target matrix

Native and Wasm core required. Any atomic used by render queues/publication must be proven lock-free on that target; targets without the required lock-free width use a single-owner or host-mediated fallback. Browser single render thread is supported.

## Required evidence

Allocator/lock audit output, race-model tests, queue wraparound/property tests, and plan-swap trace.

## Attempt 1 evidence / decision record (Terra, 2026-08-20)

- Implemented the initial std-only realtime buffer, parameter, native SPSC, local-ring, prepared
  plan, bounded publication, and reference audit-binary foundations. No V1 or legacy source was
  inspected.
- SPSC uses `capacity + 1` fixed `MaybeUninit` slots, owner-local counters, producer
  acquire/write/release and consumer acquire/read/release. The unsafe exception is confined to
  `realtime/spsc.rs` and documented in the realtime dependency policy.
- Loom is pinned to `=0.7.2` as an MIT test-only dev dependency. It is not available to normal
  production/Wasm builds; model tests still need adding under `cfg(loom)`.
- Initial smoke: `cargo check -p miso-engine-core --locked`; `cargo check -p
  miso-engine-realtime-audit`; and `cargo run -q -p miso-engine-realtime-audit --release --
  --blocks 1000000 --benchmark-rounds2` reported `blocks=1000000 swaps=2 deferred=1 ns_per_block=8
  audit_violations=0`. This is descriptive, uncalibrated smoke evidence, not an allocation-hook
  proof or performance gate.
- Deferred for Sol review: allocator/forbidden-operation hooks and mutation probes, strace
  parser, strict one-million-block evidence, Loom model tests, concurrency/drop tests, Wasm
  atomics inspection, and complete public API documentation. These gates remain unsatisfied and
  must not be weakened.

## Sol adversarial review and correction attempt 2 (2026-08-20)

Sol rejected Terra attempt 1. In addition to its candid missing gates, the public SPSC producer
held a raw pointer into consumer-owned storage, so safe code could drop the consumer and then use
the producer. Correction attempt 2 replaced that unsound lifetime convention with one fixed
`Arc<Ring<T>>` shared by two non-cloneable, deliberately `!Sync` endpoints. Arc clone/drop occurs
only at endpoint construction/destruction; push/pop/render never touch its reference count. The
final Arc owner drops any initialized move-only items, and endpoint-drop-order tests prove queued
items are destroyed exactly once.

`RealtimePlanOwner::render` now performs the one boundary poll/reservation/swap and the complete
plan render under a single audit scope. A full retirement queue retains the pending candidate and
renders the unchanged active plan. Publication backpressure returns plan ownership and consumes no
epoch. Concurrent publication tests assert a complete `(epoch, plan_id)` tuple for every block;
drop observers prove all 64 displaced plans in the stress fixture are destroyed on the dedicated
retirement thread.

The production default has a direct, inlined audit seam. The standalone audit tool enables the
semantic-neutral `realtime-audit` feature and installs the only test allocator wrapper. Allocator
violations terminate without unwinding because Rust forbids unwinding from `GlobalAlloc`; the
ordinary forbidden-operation hooks panic. Mutation probes proved allocation, deallocation, lock,
log, file-I/O, network-I/O and syscall categories all terminate while armed. Source-policy
mutation tests separately proved marked render code rejects allocation, locks, logging, and unsafe
outside the two approved files.

Local evidence passed:

- formatting; workspace/realtime/conformance/research policies and their mutation tests;
- locked workspace all-target/all-feature check, Clippy with warnings denied, all-target tests,
  and rustdoc with warnings denied;
- the Loom 0.7.2 release/acquire publication model; the native one-million-item SPSC FIFO stress;
  capacity-one/local and non-power-of-two/native wraparound; ownership-return and drop-order tests;
- pure-Rust `aarch64-linux-android` and `aarch64-apple-ios` core/target-smoke checks. Rust reports
  lock-free-available pointer atomics for native, Android ARM64 and iOS ARM64; render queues use
  only pointer-width atomic loads/stores and owner-local non-atomic counters;
- the baseline browser-local Wasm build and inspection of four object files, with no atomic opcode.
  Browser publication is host-mediated on one render agent and makes no cross-agent shared-memory
  claim;
- a traced 1,000,000-block release audit with exactly two accepted swaps, one forced deferred
  swap, fixed output storage, and zero allocation, deallocation, lock, log, file/network I/O,
  explicit syscall-hook or total violations. In the main-thread strace the begin/end marker writes
  were adjacent, with no syscall between them.

The prior Terra timing line was not valid benchmark JSON. The single authorized corrected
invocation ran two internal one-million-block rounds on an AMD Ryzen 7 9700X under the `powersave`
governor and Rust 1.97.1/LLVM 22.1.6. It reported 15.174314 and 15.220712 ns/block. This is rough,
descriptive evidence only; there was no threshold, retry, or optimization loop. The JSONL evidence
is `target/issue3/realtime-benchmark.jsonl`; syscall/allocation evidence is
`target/realtime-strace/audit.json`.

The checked-in CI now carries all repeatable gates. GitHub-hosted execution, iOS linking/device
execution, Android NDK linking/device execution, and browser runtime execution are not claimed in
this non-Git local environment and remain owned by their platform issues. No V1, legacy, old
repository, Git, or GitHub state was inspected.

## 2026-08-24 amendment (#84 phase B)

Three refinements to the SPSC contract, none of which changes an `Ordering`, the
reserve-before-swap protocol, or the `slot_count = capacity + 1` rule (power-of-two rounding was
considered and rejected: it would change this issue's capacity contract and buy nothing over a
compare the branch predictor gets right `capacity` times out of `capacity + 1`).

1. The producer and consumer cursors are each `#[repr(align(64))]`-padded and sit after the
   read-mostly header, so neither endpoint's `Release` store invalidates the line the other reads.
   The ring header grows from 72 to 256 bytes at align 64 on a 64-bit target; the oracle is
   `core::alloc::Layout`, pinned by `ring_header_is_arc_counts_plus_three_cache_lines`.
2. Each endpoint caches the peer cursor and reloads the shared line only on apparent full/empty.
   The monotonicity argument is in `docs/REALTIME_MEMORY.md` and on the two `cached_*` fields.
3. Cursor wrap is a compare rather than `%`, and `LocalRing` stores `MaybeUninit<T>` rather than
   `Option<T>` so the browser ring has no per-slot discriminant and no `expect` panic path inside
   a marked realtime region.

`scripts/check-realtime-policy.sh` now rejects `.expect(`, `.unwrap(`, `panic!(`, `unreachable!(`,
`todo!(` and `unimplemented!(` inside a marked region, with `panic-path-expect` and
`panic-path-macro` mutations proving it.

Fixture re-pin (master plan §8, class "implementation bits", oracle `core::alloc::Layout`): the
builtins resource fixture moves by exactly +200 bytes per meter stream -- +184 for the queue
header, +8 for `Consumer::cached_producer`, +8 for `Producer::cached_consumer` -- with every
diagnostic in the 10,000-case mutation matrix unchanged.
