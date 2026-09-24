# Compiler scratch reuse and allocator experiment

## Scope approved for implementation

The user requested bounded tasks suitable for fresh GPT-6 Luna MAX agents, followed by benchmarks and the user's own fresh GPT-6 Astra XHIGH verification. This document and its linked issue bodies are the implementation handoff; no source implementation or official benchmark has run during scoping.

Starting synchronized source baseline: `e09302ad` (Engine SDK 0.4.3 delivery). The primary checkout had older commits; scoping uses an isolated worktree. Rebase the implementation branch onto synchronized main before starting and review any changes to the named code boundaries.

The first deployable capability is an explicit, reusable native Rust preparation workspace. It keeps eight fixed-length numeric vectors between graph compilations: topology order/degree/levels, and coloring consumer counts/last consumers/main-input counts/main-input sources/node buffers. Both old stateless entry points and new workspace entry points use the same algorithms. Maps, sets, nested adjacency/expiration lists, cycle traversal and owned plan outputs remain independently allocated. This deliberately measures a small opportunity before extending it.

The workspace is owned by the serialized control caller, never by a render plan. Its retention cap is explicit and its capacity is reported separately. Returned artifacts cannot borrow workspace storage; every returned error and success leaves it reusable. Retention is bounded after each call; this is not a new bound on total preparation peak or a fallible-allocation guarantee.

## Issue map

| Order | Task | Owner and bounded result | Dependencies |
|---|---|---|---|
| 1 | [#870](https://github.com/misofm/engine/issues/870) | Luna MAX: three-array topology workspace and additive graph compile APIs | None |
| 2 | [#871](https://github.com/misofm/engine/issues/871) | Luna MAX: five-array coloring reuse with unchanged liveness/order | 1 |
| 3 | [#872](https://github.com/misofm/engine/issues/872) | Luna MAX: ordinary host preparation accepts the caller's workspace | 2 |
| 4 | [#873](https://github.com/misofm/engine/issues/873) | Luna MAX: correct requested-live-byte accounting and isolated timing measurement | Independent |
| 5 | [#874](https://github.com/misofm/engine/issues/874) | Luna MAX: tooling-only pool and collection-control candidates using the same algorithm slices | 2 |
| 6 | [#875](https://github.com/misofm/engine/issues/875) | Luna MAX: frozen fixtures, lifecycle runner, schema, validator and zero-workload preflight | 3, 4, 5 |
| 7 | [#876](https://github.com/misofm/engine/issues/876) | Luna MAX executes one frozen capture; root prepares decision and user review packet | 6 |

Keep one active production implementation issue. Task 4 can proceed in a separate worktree alongside 1/2. Tasks 3 and 5 may proceed independently once 2 is checkpointed if their source/manifest ownership remains disjoint; root integrates and runs broad gates serially. Every focused-green tranche is committed by root before another is layered onto it. Each assignment is intended to fit within half a working day; split it before exceeding its explicit files or product contract. Maximum five implementation attempts per issue, with one root verdict per attempt.

## Allocator and benchmark decisions

Rust's std Allocator stabilization merged September 23 for 1.100; the engine remains on 1.97.1. The lockfile already contains bumpalo 3.20.3 and allocator-api2 0.2.21 as tooling transitive dependencies. Their stable compatibility API supports a bounded experiment without making them enabled production dependencies. It does not prove the performance of the future std implementation. See the [upstream stabilization](https://github.com/rust-lang/rust/pull/156882) and [bumpalo stable allocator support](https://docs.rs/bumpalo/3.20.3/bumpalo/#using-the-allocator-api-on-stable-rust).

Compare fresh std vectors, retained std vectors, allocator-api2 Global vectors, and reused Bump-backed allocator-api2 vectors. The Global arm distinguishes collection implementation cost from allocator policy. Pool storage is limited to the same eight numeric arrays; no custom unsafe allocator or arena-owned DSP objects.

Use existing 9/64-track native-effect sessions for complete host preparation and overlap, and the existing 256-track routing corpus for graph-only preparation. Its conformance-only effect factory cannot be used by the native host. Freeze 48 kHz, q128, the release profile, target, cap, operations and validator before measurement. Record cold workspace preparation, steady replacement, growth/shrink, late rejection, and active/candidate coexistence. Timings exclude fixture construction, evidence formatting and rendering, while including required scratch reset/cleanup.

Timing uses the single benchmark allocator registration configured to System directly, without event counters or audit hooks. It performs preparation only. A separate audited executable supplies correctness/render proofs and byte measurements, so fewer global allocations cannot win timing merely by avoiding instrumentation.

Metrics are preparation time, allocator events, cumulative requested bytes, live/peak requested heap bytes, workspace retained capacity, pool backing/occupancy and fresh-process RSS where supported. Pool backing is counted once. RSS is not a substitute for stage-specific live bytes, and two rounds do not establish statistical confidence or realtime track capacity. A null result completes the experiment.

No timed run occurs until the runner preflights arguments, schema, output persistence, overwrite refusal and real child exit propagation. Then use one supervisor invocation, one sequence warmup and exactly two measured rounds per frozen row/pass. Preserve all raw output on failure; no timing rerun or tuning.

## Review and adoption

Root reviews each bounded source tranche and checkpoints it. After all implementation and measurements, hand the user a clean source SHA/base, issue map, test results, raw records, validator command, limitations and adoption recommendation. The user initiates a fresh `gpt-6-astra` agent with `reasoning_effort=xhigh`, with no implementation-agent history. Final production merge waits for that verdict and required qualification. Scoping does not claim review PASS or implementation completion.

The first series makes reuse available to native Rust callers. Existing CAPI/browser owners are not automatically accelerated. CAPI's persistent `SessionState` is the next adoption candidate only if measurements justify it; that successor must account for retained scratch alongside active/candidate/queued resources. Browser ownership, artifact qualification and SDK release are separate bounded delivery issues when relevant. Do not create a broad allocator migration merely to use the new API.

GitHub issue bodies remain identical to the numbered specs. After implementation delivery, root synchronizes evidence and states and removes only clean completed worktrees with all work/evidence preserved. All seven implementation issues remain open after this planning checkpoint.

## Scoping evidence

Two read-only Luna MAX assignments inspected compiler/host ownership and benchmark accounting. Root incorporated their findings: a partial eight-array contract, supported native fixtures separate from conformance-only graph fixtures, no redundant plan-exchange qualification, isolated timing instrumentation, explicit warmup/round state, and requested-backing cap accounting. One compact original semantic manifest remains useful because the new stateless and retained entry points share an algorithm; existing independent oracles remain the principal correctness gates. This is scope review only, not the user's final Astra implementation review.
