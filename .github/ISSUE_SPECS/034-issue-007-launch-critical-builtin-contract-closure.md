# 034 Issue-007 launch-critical builtin contract closure

## Outcome

Close the launch-critical public-contract and preparation-containment defects left after the
three-attempt stop of issue 007, without changing its accepted scalar DSP recurrence or running a
timed benchmark.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy,
benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated
`PreparedRenderPlan`: graph, schedule and capacities are immutable while its DSP state is mutated
only through exclusive render ownership. Render performs no allocation/free, locks, file/network
I/O, logging, syscalls, feature detection, panic/unwind, structural plan mutation or
data-dependent unbounded work. Displaced plans enter a bounded retirement queue and are reclaimed
off render; a full retirement queue defers a swap. There is no compiled track or meter limit.
Audio is planar `f32`; L/R parameters and state are independent unless an explicit link mode or
smoothed 2x2 matrix says otherwise. Launch session/render rates are exactly 44,100, 48,000, 88,200
and 96,000 Hz, with no implicit SRC. Output is PCM.

Issue 007 stopped after its third failed attempt. Its conditioned incremental all-`f32` TPT
HPF/LPF operation graph, scalar builtin sections, seven meter-tap semantics and focused response
thresholds are accepted only as a reusable DSP/runtime slice. They are not machine qualification.
This issue owns only four unresolved launch-critical contracts: truthful parameter descriptors,
sealed-only builtin graph attachment, exact checked retained-payload accounting, and the complete
builtin compiler mutation gate. Issue **Issue-007 builtin qualification tooling, audits, and
benchmark** depends on this issue and owns all remaining fixtures, million-render audits, target
qualification and the sole eventual timed benchmark.

This corrective workflow has at most **two total implementation attempts**: Terra attempt 1,
then one bounded Sol correction/review attempt. A second failure stops and requires a new
rescope/rebrief; no gate may be weakened. Timed benchmark invocation count is **0**. This issue
must not execute or authorize a timed benchmark.

## Scope

- Replace ambiguous builtin parameter metadata with stable scope, mapping, conditional-domain,
  update-rate, smoothing and reset contracts while preserving numeric IDs 1–10.
- Make the prepared-builtin compiler artifact and its graph-lowering provenance unforgeable by
  ordinary production callers; close every generic internal-binding bypass.
- Compute and preflight the exact engine-owned retained payload for processors, meters, queues,
  seals and bindings using checked layout arithmetic before payload allocation.
- Add deterministic 10,000-case builtin-compiler mutation coverage over valid and invalid
  parameters, meters, targets, blocks, times and all resource caps.

## Required public interfaces/contracts

`BuiltinParameterDescriptorV1` or its versioned replacement exposes explicit stable enums rather
than `per_lane: bool`, an undifferentiated decibel unit or an infinite numeric filter maximum:

- IDs 1 polarity, 2 trim, 3 HPF, 4 LPF, 5 fader and 6 mute have scope `PerLane`; IDs 7–10
  `matrix_ll/lr/rl/rr` have scope `MatrixShared`.
- Mappings are `Boolean`, `DecibelAmplitude`, `Hertz` and `Linear`. Trim/fader decibels map to
  amplitude gain, not power; their retained domain is `[-144, 24] dB`. Matrix coefficients are
  finite linear values in `[-1, 1]`. Boolean values have exact false/true encodings.
- HPF/LPF domain is conditional on the prepared sample rate: exact `0 Hz` disables the section;
  otherwise the value is finite and `10 Hz <= f < sample_rate/2`. The descriptor represents this
  conditional upper bound directly and never publishes `f32::INFINITY` as the maximum.
- Polarity, trim, HPF, LPF, fader and mute are `PreparedOnly` and restore the prepared value on
  reset. Matrix coefficients are `BlockTarget`, use exact linear-N-update smoothing, and reset
  current state to the retained target. Defaults remain the existing identity values.

`PreparedBuiltinsSession` remains opaque and consuming. Its immutable seal contains canonical
session-TOML SHA-256, rate, quantum, sorted exact track IDs, three `(track, stage)` processor
identities per track, exact recomputable `(track, BuiltinTail)` values, sorted exact meter request
tuples, observer and consumer identities, and the checked resource report. The only production
path that marks graph nodes as internal builtin bindings must consume a compiler-produced sealed
artifact after recomputing and comparing every field. No public API accepting arbitrary
processor/observer vectors may grant equivalent internal-builtin provenance. An external-crate
compile-fail contract proves that callers cannot construct, mutate, clone back, forge or bypass
the artifact. Normal issue-006 external graph bindings remain supported and cannot masquerade as
the compiler-owned builtin path.

`BuiltinResourceReportV1` names **engine-owned retained payload bytes**, not RSS or allocator
metadata. `retained_payload_bytes`, `maximum_single_allocation_bytes` and an allocation-count/
layout breakdown cover exactly the payload retained by the prepared artifact. All arithmetic and
all `usize`/`u64`/`isize` conversions are checked; arithmetic failure returns
`builtin.resource.arithmetic_overflow`.

## Exact resource and preparation contract

Phase 1 validates parameter domains, counts, layouts and every cap without allocating any
issue-owned processor, meter queue or artifact payload. Count exactly:

- each concrete input, fader and matrix processor box payload;
- processor, observer, consumer, tail and seal vectors at actual element layouts/capacities;
- every retained stable-ID/string payload;
- every meter observer box plus producer and consumer endpoint payload;
- every SPSC logical header and exact `logical_capacity + 1` slot payload through one checked
  helper shared by preflight and queue construction; and
- all alignment padding imposed by the engine-owned `Layout` formulas.

Do not count the transient unsplit `BuiltinChain`, allocator headers, page rounding, unrelated
session/effect artifacts, or a concrete box payload twice through its trait-object pointer. Phase
2 performs only the reported allocations and remains transactional on allocation/queue failure.
A test-only tracking allocator records every phase-2 requested layout. The resource grid is the
Cartesian product of tracks `{1,4,65537}` and meter-set counts `{0,1,7}` at logical queue capacity
four, subject only to explicitly configured resources; it compares total bytes, largest request,
allocation count and every layout/count class. One-byte-below each independently applicable total
and largest-allocation cap rejects in phase 1 with zero issue-owned payload allocations. Retain a
65,537-track zero-meter success case and a configured-resource rejection case.

## Opaque artifact corruption and compiler mutation requirements

A compiler-owned, test-only constructor independently corrupts exactly these eight seal
categories: (1) session/rate/quantum identity, (2) tracks, (3) processors, (4) tails, (5) meter
requests, (6) observers, (7) consumers and (8) resources. Each corruption returns its exact
sorted typed diagnostic before either prepared input is consumed. The seam is absent from
production artifacts. Tests also cover unknown track, duplicate handle/request, missing/extra
processor, changed tail, changed observer node, changed consumer metadata and all identity
mismatches.

One deterministic seeded test executes exactly 10,000 builtin **compiler** cases spanning:

- valid and invalid per-lane gains/booleans/filter cutoffs/orders and every matrix coefficient;
- all seven meter taps, valid/duplicate/unknown handles and tracks, periods, hold/decay,
  reset-generation and queue-capacity boundaries;
- finite and nonfinite matrix targets plus smoothing counts `0,1,2,127,128,u32::MAX`;
- empty/mismatched blocks, quanta `1,127,128,255,1024`, discontinuous and overflowing sample
  times; and
- exact/equal/one-byte-below total and largest-allocation caps plus arithmetic-overflow cases.

Every case records complete success or exact sorted diagnostics. It must never partially consume
inputs, panic, time out, introduce a compiled ceiling or allocate payload beyond an accepted
report.

## Deliverables

- versioned parameter descriptor API and exhaustive descriptor/domain tests;
- sealed-only builtin graph-lowering API with external compile-fail proof;
- all eight compiler-owned corruption categories and transactional diagnostic tests;
- checked shared SPSC/layout accounting, two-phase preflight and full tracking-allocator grid;
- exact 10,000-case compiler mutation suite; and
- a checksummed nonbenchmark evidence record naming commands, target/toolchain and results.

## Explicit non-goals

Changing the accepted TPT coefficients, operation order, response thresholds, matrix/pan law,
meter math, session schema, issue-006 graph topology/PDC, issue-008 SIMD kernels, fixture corpus,
million-render audit, performance threshold, human listening or any timed benchmark.

## Dependencies by exact issue title

- Dual-mono builtins and metering
- DSP research corpus and conformance harness
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral

## Sol implementation brief (2026-08-21)

**READY FOR TERRA ATTEMPT 1.** The tracked authoritative brief is
`.github/ISSUE_SPECS/BRIEFS/034-issue-007-launch-critical-builtin-contract-closure.md`. It freezes
the two-attempt budget, exact descriptor table, private wrapper-owned graph bind path, all eight
corruption categories, allocation-layout grid and 10,000-case compiler mutation coverage. It
prohibits every timed workload; invocation count is 0.

## Hazards/decisions

Rust has no general cross-crate friend visibility. The approved solution must remove authority
from public generic attachment, not merely rename it, add a convention, use a forgeable feature
flag, or validate values derived from the same attacker-controlled vectors. Resource exactness is
defined at engine-owned Rust payload/layout boundaries so it is portable and testable; it is not
an RSS claim. Test-only corruption support cannot become a production mutation API.

Issue 008 is blocked on this issue, because its four/eight-lane builtin adapters consume the
stable per-lane parameter/preparation and graph/resource contract. Issue 008 is **not** blocked on
issue 035: scalar expected-output tooling and the descriptive scalar benchmark do not define SIMD
semantics. Issues 022–024 and 026 remain blocked on issue 035's corrected machine candidate.

## Acceptance gates with objective measurements

All descriptor IDs, scopes, mappings, conditional domains, defaults, update rates, smoothing and
reset semantics pass exhaustive table tests at all four launch rates. External compile-fail tests
prove opacity and absence of a generic internal-builtin attachment bypass. All eight corruption
categories and every listed mismatch reject transactionally with exact diagnostics. The full
resource grid and each one-byte-below boundary agree exactly with independently tracked phase-2
layouts/counts, including 65,537 tracks, and static/policy checks find no unchecked conversion,
saturation or substitute maximum on the accounting path. The deterministic 10,000-case compiler
mutation suite passes. Locked focused/workspace tests, warning-denied all-target Clippy and
rustdoc, formatting, graph/realtime/builtin/workspace policies and their mutations pass. A
zero-launch check reports `workload_launches=0`; no timed workload or accepted benchmark artifact
is created.

## Target matrix

Native scalar build/test plus compile checks for `aarch64-linux-android`, `aarch64-apple-ios` and
`wasm32-unknown-unknown` with `-simd128` and `+simd128`. These are contract/portability checks, not
the complete target qualification owned by issue 035.

## Terra attempt 1 evidence (2026-08-21)

Implementation checkpoint pending Sol review. `BuiltinParameterDescriptorV1` now uses explicit
scope, mapping and rate-aware domain enums; the filter contract is represented as
`DisabledOrRateBoundedHertz`, with no infinite public maximum. `PreparedGraphBuiltinsArtifact`
now privately owns the compiler-produced processor/observer parts and exposes only consuming
sealed binding of disjoint external nodes; the public generic
`PreparedGraphPlan::attach_internal_bindings` capability is removed. `BuiltinResourceReportV1`
aliases the single exact retained-payload report type and resource-cap conversion is checked.

The deterministic compiler preparation matrix uses seed `0x000000034007c10`, executes exactly
10,000 cases, observes all seven taps and eight frozen mutation classes, and checks a frozen
diagnostic transcript hash `565235985001749527`. It performs no render benchmark.

Passing commands: `cargo fmt --check`; `cargo test -p miso-engine-builtins --lib`; `cargo test
-p miso-engine-builtins-compiler --features test-support`; `cargo test -p
miso-engine-graph-compiler --lib`; `cargo test -p miso-engine-graph-compiler --doc`; `cargo check
--workspace`; `cargo test --workspace --locked`; `RUSTFLAGS='-Dwarnings' cargo clippy --workspace
--all-targets -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`; `bash
scripts/test-builtins-policy.sh`; `bash scripts/check-builtins-policy.sh`; `bash
scripts/check-graph-policy.sh`; `bash scripts/check-realtime-policy.sh`; `bash
scripts/check-workspace-policy.sh`; `bash scripts/check-graph-determinism.sh`; and `bash
scripts/check-builtins-targets.sh`.

Candidate source SHA-256: builtins `59e5ebf08d39a4699dec90b6cf1fa23fbae6ada57320d6ad2f771562dd327f11`;
builtin compiler `e8eb45863654f326adff9fb5fa6912649c8566425b10a14843092ff4d7ad0e47`;
graph `acf7cb6f9f932a05ecba7447f399c265f02897cbe18a5cf9b785d4b801e4e957`;
graph compiler `4263f592b241089b689564e207a16456c8f674f8c5b4d0acc1fa69c8a8ff9da4`.
`timed_benchmark_invocations=0`; no benchmark artifact was created.

## Sol attempt 2 final verdict (2026-08-21) — FAIL

Attempt 1 did not satisfy the frozen contract. Its 10,000-case test cycled only eight
meter/resource cases and omitted session parameters, explicit meter handles, matrix targets,
smoothing boundaries, quanta, malformed blocks, sample-time behavior, complete cap boundaries and
arithmetic overflow. The phase-two allocator test compared only total/largest for one
`1 track x 7 meters` point, not allocation count or the ordered `(size, align)` multiset over the
frozen 3x3 grid. `PreparedBuiltinsSession::into_graph_parts` was also public and returned the
provenance-bearing processor/observer vectors that the compile-fail contract explicitly forbids
external callers from extracting.

The single bounded Sol correction removed that extraction API by moving graph attachment into an
opaque builtins-compiler-owned wrapper; added construction/mutation/extraction/clone/
back-conversion/generic-attachment compile-fail cases; made meter handles explicit and rejected
duplicate handles; expanded all eight corruption categories with missing/extra/changed and
identity subcases; made the retained report and independent allocator probe compare exact ordered
`(size, align, count)` classes; corrected the shared `Arc` SPSC header layout; added the full
`{1,4,65537} tracks x {0,1,7} meters` grid; and exercised equal/one-byte-below state, total,
meter, item and largest-allocation caps with zero phase-two allocations on rejection.

Those bounded corrections pass `cargo fmt --check`; `cargo test -p miso-engine-builtins --lib
descriptor`; `cargo test -p miso-engine-builtins-compiler --features test-support --test
allocation_tracker`; `cargo test -p miso-engine-graph-compiler --lib
each_forged_builtin_seal_tuple_is_rejected_before_graph_attachment`; `cargo test -p
miso-engine-graph-compiler --doc`; and `cargo check --workspace`.

The corrected composite compiler matrix then exposed a frozen contract contradiction. At 88,200
Hz, the LPF cutoff `f32::from_bits((44_100.0_f32).to_bits() - 1)` is finite, at least 10 Hz and
strictly below Nyquist, so `BuiltinParameterDomain::DisabledOrRateBoundedHertz` accepts it as the
required public domain mandates. `BuiltinChain::new`, however, returns
`BuiltinParameterError::FilterCoefficients`: the retained all-`f32` coefficient cast reaches the
strict Jury boundary near Nyquist. Exact red gate:

`cargo test -p miso-engine-builtins-compiler --lib deterministic_builtin_compiler_mutation_matrix_has_exactly_ten_thousand_cases -- --nocapture`

The failure is deterministic at `case=46, class=46` and reports
`builtin.filter.coefficients` at `$.tracks[id=vocal].builtins`. Narrowing the descriptor would
violate the frozen `0 or [10, Fs/2)` domain; accepting the value requires changing the retained
coefficient/DSP preparation, which this brief explicitly prohibits and names as a stop condition.
No gate was weakened. The two-attempt budget is exhausted, issue 034 remains open, and a new
rescope/rebrief is required before issue 008 or 035. `timed_benchmark_invocations=0`; no timed
workload or benchmark artifact was created.

## Post-stop rescope (2026-08-21)

**ISSUE 034 IS STOPPED AND DID NOT PASS.** Preserve the complete attempt and failure evidence
above. Checkpoint `9c57af8` remains reusable input for its bounded metadata, opacity, corruption
and resource-accounting corrections, but those corrections are not a PASS for this issue.
**Representable TPT cutoff domain and builtin contract acceptance** owns the single numerical
boundary defect and the final composite matrix/nonbenchmark acceptance. Issues 008 and 035 now
depend on that successor. `timed_benchmark_invocations=0` remains authoritative.

## Required evidence

Descriptor table dump; external compile-fail transcript; eight-category corruption results;
resource formulas, tracked layout/count grid and cap-boundary results; mutation seed/case count and
diagnostic summary; policy/target/workspace logs; candidate hashes; and explicit
`timed_benchmark_invocations=0`. Passing this issue means only **launch-critical builtin contract
closure**. It is not issue-007 machine qualification, audible-quality evidence or launch approval.
