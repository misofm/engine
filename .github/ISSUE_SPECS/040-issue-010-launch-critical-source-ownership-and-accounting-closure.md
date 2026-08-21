# 040 Issue-010 launch-critical source ownership and accounting closure

## Outcome

Close the launch product contract for bounded source streaming by making native worker lifetime a
retired-plan ownership property, exposing the frozen source capacity and sanitation telemetry, and
charging every retained source allocation exactly once before publication.

## Context

Engine V2 is greenfield and must never inspect or inherit V1. The render thread exclusively owns a
preallocated `PreparedRenderPlan`; render performs zero allocation/free, locks, file/network I/O,
logging, syscalls, thread lifecycle, structural mutation or data-dependent unbounded work. Plans
swap only at block boundaries and are reclaimed off render. Audio is planar `f32`; source/engine
rate mismatch rejects without SRC. Launch rates are exactly 44,100, 48,000, 88,200 and 96,000 Hz.

Issue **JIT PCM streaming and host-supplied source rings** stopped at checkpoint `5dbe1cb` after
strict Sol review. Its source crate, bounded move-owned ring, RIFF/RF64 parser/decoder, host chunk
boundary, coordinator source-set fan-out, deterministic basic seek behavior, existing ring audit,
target policy and Wasm no-atomic evidence are accepted only as technical input. Issue 010 remains
FAIL. Its review found that workers are separated from plan retirement, capacity and native
sanitation telemetry are incomplete, exact accounting omits native control and retained
source-set/worker allocations, and several representative product cases are absent.

This rescope has exactly **two total attempts**: one Terra implementation/review attempt and, if
needed, one bounded Sol correction/review. A second failure stops. No benchmark, timing call,
performance threshold or qualification-matrix expansion is allowed;
`timed_benchmark_invocations=0` is invariant.

## Scope

- Make the graph-owned native source set retain the sole worker stop/join owners so successful bind
  moves them into `PreparedRenderPlan` and off-render plan reclamation stops, joins and destroys
  them. Return the same ownership transactionally on preparation/bind/cap failure.
- Keep non-render controller endpoints separate from join ownership. A dropped controller cannot
  detach a worker, and no worker/file/decoder may outlive its retired source set.
- Expose one exact immutable ring shape on both endpoints: channel count, quantum frames, frame
  capacity and transfer-block count. Propagate the native decoder's saturating sanitation count to
  bounded non-render telemetry; host-decoded chunks report zero source-decoder sanitations.
- Replace or account for native control/event storage with an exact prepared bound. Charge retained
  ring, worker-control, worker/controller, source-plane, entry, mapping, claim, driver and owned-ID
  allocations exactly once; keep OS thread stack, allocator headers, file page cache and RSS
  explicitly descriptive and outside exact engine-owned bytes.
- Recheck arithmetic, largest allocation, source queue/item caps, session compile caps and
  `limits.memory_bytes` transactionally before a publishable plan exists.
- Add only compact product tests: all four matching launch rates plus one extended mismatch, one
  delayed-old-chunk seek boundary, the existing one-source/three-track fan-out plus transactional
  claim rejection, lifecycle retirement, telemetry and exact-cap boundary cases.

## Required public interfaces/contracts

`PcmSourceProducer` and `PcmSourceConsumer` expose the same immutable `PcmSourceShape` (or an
equivalent typed value) containing `channel_count: u32`, `quantum_frames: QuantumFrames`,
`frame_capacity: u64` and `transfer_block_count: u64`. Existing cursor, generation, full/empty,
stale, underrun and EOF telemetry semantics remain unchanged and saturating.

Native preparation returns movable controller endpoints while the sealed source set owns the
uncloneable worker-retirement tokens. A successful graph bind moves those tokens into the plan's
executor ownership. A rejected bind returns the graph inputs and source set with every token still
usable. When the plan exchange retires and reclaims the plan off render, each native worker receives
one stop, is joined once, and destroys its reader/decoder/staging on that retirement owner. Render
never sends, waits, joins or destroys a worker.

The native sanitation telemetry is cumulative, saturating and counts every decoder replacement of
a non-finite, subnormal or non-representable sample with positive zero. It is transported only
through bounded prepared non-render state; it cannot add a lock, allocation, syscall or baseline
Wasm atomic to render. Host-supplied decoded `f32` is not reinterpreted as native decoder work and
therefore contributes zero to this counter.

The exact resource report enumerates every retained engine allocation by semantic category and
reports total bytes, largest allocation and fixed queue/item counts. PCM already charged by the
session remains a separate equal value and is never double charged. Checked cap failure returns all
ownership and starts or publishes no partial plan.

## Deliverables

- plan/source-set-owned native worker retirement and separate bounded controllers;
- exact ring-shape and native sanitation telemetry APIs;
- corrected exact retained source/session/graph accounting and cap validation;
- representative launch-rate, seek, fan-out/rejection, lifetime, telemetry and cap tests;
- focused/full policy, target and realtime evidence for the changed production boundary; and
- a concise evidence record with `timed_benchmark_invocations=0`.

## Explicit non-goals

Expanding the WAVE corpus or parser catalog; randomized race qualification; a 100,000-render real
worker-delay qualification run; sparse multi-hour allocation-layout/RSS tooling; benchmark work;
performance tuning; SRC; compressed formats; device/browser runtime adapters; graph topology/PDC
changes; session or control-wire changes; or a general streaming service.

## Dependencies by exact issue title

- JIT PCM streaming and host-supplied source rings
- Real-time memory, buffers, queues, and plan lifetime
- Versioned TOML schema and transactional session compiler
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral

The stopped Issue-010 dependency means only checkpoint `5dbe1cb` and the explicitly preserved
technical input above; it does not imply an Issue-010 PASS.

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1 after the matching remote issue exists and matches this body.** The
tracked brief is
`.github/ISSUE_SPECS/BRIEFS/040-issue-010-launch-critical-source-ownership-and-accounting-closure.md`.
It freezes the ownership transfer, accounting boundary, representative gates, two-attempt maximum
and zero-benchmark rule.

## Hazards/decisions

Do not make worker handles clone-owning, join in render, attach workers beside rather than inside
plan-retired ownership, invent exact bytes for `std`/OS internals, retain duration-sized PCM, add a
second source ring per track, weaken transactional return, or treat a report formula as evidence
that omitted storage does not exist. Graph remains independent of the source crate; the source-set
driver is the additive ownership seam.

## Acceptance gates with objective measurements

1. A native session source set binds into sequential and native-fallback plans, then one
   block-boundary replacement proves the old source worker stops/joins exactly once only when the
   retired plan is reclaimed off render. Full retirement defers the swap. Bind/cap failure returns
   every graph binding, source controller and unconsumed worker-retirement owner.
2. Producer and consumer shapes agree exactly for one-quantum and wrapped multi-quantum rings.
   Native F32/F64 sanitation fixtures expose the exact cumulative replacement count through bounded
   non-render telemetry; host submission reports zero native-decoder sanitations.
3. An enumerated allocation table covers every retained ring, native control/event, worker/
   controller, source-plane, entry, mapping, claim, driver and ID allocation. Exact cap accepts at
   the reported byte count and rejects one byte below, with checked overflow and no PCM double
   charge. Source duration is absent from every retained formula.
4. Matching native preparation succeeds at 44,100/48,000/88,200/96,000 Hz; one launch session with
   an extended-rate source rejects with `source.rate.mismatch` and no source set. One deterministic
   delayed-old-chunk seek renders no old sample after the boundary. The existing four-channel
   one-ring/three-track mapping remains exact in sequential/native fallback, while missing, extra,
   duplicate and ordinary-binding-overlap claims reject and return ownership.
5. Focused locked tests and the existing source render audit pass with zero forbidden render
   operations. Full locked workspace check/test, format, warning-denied Clippy/rustdoc, workspace/
   realtime/source/graph policies, Linux native worker tests, Android/iOS ARM64 and Wasm scalar/
   simd128 compile checks pass. Wasm still links no native file worker or atomic opcode.
6. No Issue-040 benchmark command or artifact exists; timed benchmark invocation count is exactly
   zero.

## Target matrix

Linux/cloud owns native resolver/worker execution. Android/iOS compile the host source and changed
ownership surface without device-runtime claims. Browser Wasm compiles only the local host ring,
scalar and simd128, with no filesystem worker, thread or baseline atomic requirement.

## Required evidence

Candidate/source hashes; public API diff; lifecycle/drop/join/retirement transcript; exact shape and
sanitation counters; enumerated retained-allocation report and exact/one-below cap results;
representative rate/seek/fan-out/rejection transcripts; realtime/workspace/policy/target results;
explicit `timed_benchmark_invocations=0`; and Terra plus final Sol PASS/FAIL verdicts.

## Terra attempt 1 evidence (2026-08-21)

**Status: FAIL — hold for Sol review/correction.** The focused source checkpoint is green, but the
required full locked workspace test gate is not: `miso-engine-parametric-eq` fails
`production_coefficients_and_analytic_response_match_the_independent_f64_oracle` at `LowShelf`,
44,100 Hz, 10 Hz, probe 0 (`-23.457245778509655` versus
`-23.999999996345114`). This is outside Issue 040's source/graph boundary and was not modified.

- Public ownership surface: `prepare_native_source` now returns opaque `PreparedNativeSource`;
  its only graph conversion transfers the consumer and crate-private uncloneable join owner into
  `SourceGraphSource`. A `compile_fail` doctest proves `NativeSourceWorker` is not public.
  Source-set/retired-plan lifecycle tests prove the worker remains alive until off-render source
  set or retired-plan destruction; controllers receive `Stopped` afterward.
- Source reports enumerate ring, fixed decoder/staging, worker command/event queues, shared
  telemetry payload, graph source entries/mappings/claims/driver/planes/stable-ID payloads, and
  controller records. PCM remains separately session charged. The focused retained-layout grid
  covers 1, 4 and 65,537 count arithmetic without duration-sized PCM; exact combined cap accepts
  and one-byte-short rejects before publication.
- Product tests pass for producer/consumer frozen shapes, native sanitation and host zero
  sanitation, all four matching launch rates, 192 kHz mismatch rejection, delayed-old-generation
  seek discard, four-channel/three-track sequential plus native-fallback fan-out, and transactional
  missing/extra/duplicate/ordinary-overlap source claim rejection.
- PASS: `cargo test -p miso-engine-source` (24 unit tests plus one API compile-fail doctest),
  focused fan-out/claim test, source Clippy with warnings denied, locked workspace check, workspace
  Clippy with warnings denied, warning-denied workspace rustdoc, workspace/realtime/graph policy
  checks and mutation tests, Wasm no-atomic check, and source+graph checks for Android ARM64, iOS
  ARM64, Wasm scalar and Wasm `+simd128`.
- FAIL: `cargo test --workspace --locked` only for the parametric-EQ oracle failure above. The
  existing source render audit was not re-run in this checkpoint to avoid Issue 041's expanded
  audit matrix; no benchmark command or timing command was run.

`timed_benchmark_invocations=0`.
