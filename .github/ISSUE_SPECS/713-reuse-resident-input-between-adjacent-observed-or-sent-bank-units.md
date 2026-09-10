# Reuse resident input between adjacent observed or sent bank units

GitHub: https://github.com/misofm/engine/issues/713

Parent: #349 (RT-9). Coordination: #559/#560. Base main:
`b1f9128f3e06532afdfc16aad661c4b2deb5dea1`. Tracker:
`8fae33b6554642d7afd0e66c2ac463a75c0beb3a`.

## Problem

RT-9 remains open because an observer or send tap can preserve separate adjacent
bank units while the successor reacquires the predecessor's output through a
whole-block gather. The bounded child removes only that successor gather when a
resident-input proof is complete. It does not merge the owners, bypass the
predecessor scatter, or deliver the full selective-tee finding.

The historical cross-rack chain merge in PR #208 deliberately retained declines
for extra readers, sends, leased-stage observers, and lane misalignment. The
#203 metering issue and PR #521 metering change did not deliver a resident-input
tee. Those retained boundaries and their attribution remain unchanged.

## Smallest partial slice

For immediately adjacent compatible bank units, reuse the predecessor's retained
AoSoA output as the successor's input and remove exactly one successor gather
transpose. Keep the two `BankChain` owners separate and preserve all unit
boundaries, predecessor scatter, predecessor observation, successor observation,
sends, and normal scratch ownership:

- the predecessor executes and scatters its final output as before;
- the successor copies the identical retained AoSoA words into its existing
  scratch at its normal input-acquisition boundary;
- the successor then executes with the same input it would have obtained from the
  old gather path;
- no pointer or lifetime alias crosses the owners, and no unsafe alias is
  introduced; and
- the full selective tee remains a separately scoped successor.

This child may establish only a bounded RT-9 partial outcome. It must not claim
that all observer/send declines are eliminated or that the original RT-9 finding
is fully delivered.

The physical work removed is one successor gather transpose; a whole-block AoSoA
copy remains. This is not a timing, cycle, or full-selective-tee claim.

## Frozen eligibility and declines

Graph bind/runtime privately proves the resident-input candidate for immediately
adjacent emitted bank units. Its proof covers backend, lane/dataflow identity,
bank width, render quantum, populated-lane set, lane order, the exact successor
first-slot predecessor output, one undelayed main input, and the absence of
reduction, sidechain, or staging on that input. Because predecessor scatter
remains, the separate resident predicate may admit extra readers, planar sends,
direct observers, and observed aliases. Each such reader/send/observer keeps its
original planar path and execution order. Admission still declines folded or
transformed predecessor output, nonadjacency, backend/width/quantum/lane
mismatch, changed B input identity, delay/reduction/sidechain/staging, or any
relationship that would change execution order.

The existing `chains_into` extra-reader, direct-observer, and alias-observer
declines remain unchanged and are irrelevant to this separate resident
admission. Do not weaken its reader, send, sidechain, session-output, or
alias-observer proofs and do not turn this child into a general chain executor
rewrite. Rack does not reject nonadjacency or current-block identity and does
not accept a caller-trusted mode; graph owns those proofs.

## Safe resident run API and word semantics

Because `BankChain` scratch is private, authorize one narrow safe public rack
entry for graph's adjacent prepared-unit path. Freeze the exact preferred
signature as:

```rust
pub fn run_with_resident_input<M: BankMembers + ?Sized>(
    &mut self,
    predecessor: &BankChain,
    members: &mut M,
    frames: u32,
    first_sample: u64,
) -> Result<(), RenderError>;
```

The existing public `run` keeps its signature and delegates with
`predecessor: None` to one private `run_with_input(..., predecessor:
Option<&BankChain>)`; `run_with_resident_input` delegates to that same private
method with `Some(predecessor)`. Both entries therefore share one executor, and
the unchanged `run` retains the old acquisition behavior for every ordinary
case.

At the start of the private invocation, rack validates only local stored source
compatibility before successor drains: equal width, quantum, active mask,
destination bounds, and no unsupported predecessor epilogue. A local decline
selects ordinary gather inside that same invocation. It does not return a public
mode/error choice and it never calls `run` again. All local compatibility checks
complete before any destination write.

The method exposes no mutable scratch and retains no references. It is the sole
production rack entry used by graph for the graph-owned adjacent proof; no
execution token or caller-trusted compatibility flag is introduced. The private
mode is derived only after this invocation's drains from its collapse decision.
The successor then runs `begin_block`, witness, collapse, and agreement in their
existing order; at the existing acquisition boundary it copies resident words
or performs the ordinary gather, followed by unchanged stages, seam, scatter,
and epilogue.

The copy is mode-aware and overwrites exactly the words the successor's old
acquisition would overwrite:

- partial gathers write only active-lane words;
- mono gathers write only the left plane and leave right scratch untouched; and
- inactive lanes and untouched right scratch remain unchanged, with no
  unobservability waiver for the discriminator path.

The allocation/equivalence fixtures must poison inactive lanes and right scratch
to discriminate an overbroad copy. No public mutable scratch API, retained
borrow, unsafe alias, or public mode/error type is authorized.

## Structural resident-call gate

A captured `rg`/source validator must find exactly one non-test production call
to `run_with_resident_input`, located in graph's adjacency- and
freshness-enforcing execution path. The method definition, its private wrapper,
and test calls are accounted for separately. A mutation that adds or moves a
second production call, or bypasses graph admission, must fail this same source
gate. Do not add a policy or script path for this check.

## Current-block freshness and placement

Graph reaches B only after A executes successfully and A's observation completes
in the same invocation. Safe disjoint borrowing (for example, a split unit
slice) gives A immutable access and B mutable access without aliasing owners.
Rack validates its local stored shape before B drains. Then B's
`begin_block`/witness/collapse/agreement decisions occur once in their original
order before resident acquisition. A producer process or observation failure
prevents B from executing; no stale resident data is reused and `run` is never
called a second time on decline or error.

Preserve `BankChain::transposes` semantics. Add only private or test-only gather
evidence that distinguishes the resident acquisition from the old planar gather;
do not publish a new runtime counter or reinterpret an existing counter as a
timing result.

## Behavioral and realtime contract

Preserve the exact sequence
`execute(A) -> observe(A) -> execute(B) -> observe(B)`.

- An observer failure in A prevents every B drain/state transition exactly as it
  does on the old path.
- A command enqueued by A's observer reaches B at the same block boundary and in
  the same order.
- Begin/process errors, reports, counters, queued records, and state are exact
  against an old scalar reference and the prior acquisition path.
- Sends retain planar source, gain, matrix, PDC, fanout, and reduction semantics,
  including nonunity crossfeed and delayed compensation paths.
- Independent collapse and right-channel state remain independent.
- Render performs no allocation, free, lock, syscall, I/O, logging, unbounded
  work, unsafe aliasing, or structural mutation.

## Exact ownership and paths

The authorized implementation and evidence paths are only:

- `crates/rack/src/lib.rs`;
- `crates/graph/src/runtime.rs`;
- `crates/graph/src/lib.rs`;
- `crates/graph-compiler/src/lib.rs`, only for named meter/send/width/collapse
  integration evidence;
- new `crates/graph/tests/rt9_resident_bank_input_alloc.rs`; and
- this numbered specification.

No `program.rs`, manifest, policy, host, artifact, pin, workflow, benchmark,
timing, or unrelated test path is authorized. Lane B alone owns AudioWorklet
qualification and pins. Preserve PR #208's retained declines, #203/#521's
non-tee history, and all existing source, evidence, artifact, and worktree
history.

## Preparation and allocation gates

Before implementation, freeze a safe borrow/API design that does not move
`BankChain` owners or rewrite the general chain executor. Calculate retained,
preparation-peak, and largest-allocation coverage for native and Wasm layouts.
If the resident metadata or acquisition peak is outside the named accounting
paths, stop and split an accounting prerequisite; do not invent a cap, waive a
bound, or continue into unrelated resource work.

The allocation fixture must exercise an actual `PreparedRenderPlan` with a live
detector and prove zero render allocation/free. Preparation allocation accounting
is separate and must identify ownership and release. It must not be converted
into a timing, cycle, or memory-improvement claim.

## Finite evidence gates

Compare the old scalar/reference acquisition with W4 and W8 full and partial
units at frames `1`, `width - 1`, `width`, `width + 1`, and the configured
quantum. The evidence must include asymmetric values, signed zero, nonfinite
values, stateful stages, and multiblock behavior, with bitwise PCM,
observations, reports, counters, and state equality.

Exercise direct observers, alias observers, multiple observers, observer failure,
producer-process failure, observation failure, successor-begin failure, and
successor-process failure with ordered traces and subsequent-state checks.
Exercise nonunity crossfeed sends, delayed PDC, an extra consumer, fanout and
reduction. Cover independent collapse modes and recovery, scalar and incompatible
controls, nonadjacent controls, and the forced old acquisition path. Direct,
alias, multiple-observer, and planar-send fixtures are positive resident cases
when the separate predicate holds; incompatible forms remain controls.

One mutation must restore the old planar gather and fail the physical resident-
acquisition assertion while semantic PCM/reference checks remain meaningful. Keep
the mutation private and preserve its original failure and restored result.

Reuse the existing graph/rack corpus and fixtures. Do not manufacture a timing
workload, retry a measurement, or claim an improvement from a source shape.

## Proportional verification and delivery

After Sol scope approval, Astra XHIGH owns the low-level/render implementation
because this child changes the audio execution path. Astra LOW performs every
scope, source, evidence, exact-head, and post-main verification assignment.
The implementation follows the repository's attempt, adversarial-review,
checkpoint, required-CI, guarded-merge, GitHub-synchronization, and clean
delivered-worktree requirements.

Run focused graph/rack gates first, then the affected crate debug and
release-unwind suites, strict Clippy, allocation gate, policy and mutation
controls, unfused/realtime checks, native x86-64-v3, Wasm scalar and Wasm
simd128 builds, and exact diff/resource checks. No timing or benchmark gate is
authorized. Required PR CI, guarded merge-parent review, post-main qualification,
and issue synchronization remain mandatory for any later delivery.

## Acceptance gates

- Only eligible adjacent compatible units use resident input, with a provable
  identical word copy into existing successor scratch.
- The safe resident execution entry exposes no scratch or retained references;
  every local mismatch selects ordinary gather within the same invocation and
  leaves the unchanged `run` path available, while execution failures return
  the existing `Result<(), RenderError>`.
- Partial and mono copy masks preserve inactive lanes and untouched right
  scratch, with poisoned discriminators covering accidental writes.
- Current-block freshness, predecessor observation ordering, successor begin and
  collapse placement, and safe disjoint borrowing prevent stale data and owner
  aliasing.
- Separate owners, predecessor scatter, observers, sends, unit boundaries,
  PDC, reductions, collapse, state, reports, errors, and command boundaries are
  unchanged.
- Existing `chains_into` declines remain intact, including extra readers,
  direct observers, alias observers, sends, sidechains, and nonadjacent cases.
- The separate resident predicate admits positive extra-reader, planar-send,
  direct-observer, and observed-alias cases only when the frozen identity and
  order proof holds; their original planar paths remain observable.
- The source validator finds exactly one non-test production call to
  `run_with_resident_input`, and its admission-bypass/second-call mutation fails
  the same gate.
- Old/reference and resident paths pass the finite scalar/W4/W8 behavioral and
  failure matrix, including the forced old-acquisition control and physical
  mutation failure.
- The actual prepared-plan allocation gate proves zero render allocation/free
  with a live detector; no timing, cycle, memory-saving, or improvement claim is
  made.
- The feature diff contains only the exact authorized paths, and this issue's
  title/body match the numbered local specification.
- The resulting status is a bounded RT-9 partial child. Full selective teeing
  remains a separately numbered successor.

## Initial scope record

At main `b1f9128f3e06532afdfc16aad661c4b2deb5dea1`, live #349 still records RT-9
as an open medium class-A row at `crates/graph/src/runtime.rs:2686-2731`.
Synchronized tracker head is `8fae33b6554642d7afd0e66c2ac463a75c0beb3a`.
Lane A has completed the RT-8 reconciliation in #711; #705 remains lane B's
active AudioWorklet qualification and pin responsibility. This child is path
disjoint from lane B and consumes one lane-A implementation slot after Astra LOW
scope review.

No product source, test, artifact, pin, timing, or benchmark change is made by
this opening checkpoint. The opening scope review failed before implementation;
the copy-only API correction also failed before implementation, and this is the
current correction. No implementation attempt was consumed by any of those
scope reviews. No RT-9 full-delivery or performance credit is claimed.

## Scope correction history

- Astra LOW returned **SCOPE FAIL** for opening checkpoint
  `211fb5ee142287ca029683c1f98e84da552c39eb`; no implementation ran and no
  attempt was consumed.
- Astra LOW returned **SCOPE FAIL** for the copy-only API correction
  `ee1fab399cde253e87303426e90c31858a3725f8`; no implementation ran and no
  attempt was consumed.
- Astra LOW returned **SCOPE FAIL** for correction
  `f55b1e4e32e5e294d7cb1a16ff0786336496cfc2`; the extra-reader/send/observer
  eligibility, the structural single-caller gate, and the prior failure history
  were not yet frozen precisely enough. No implementation ran and no attempt
  was consumed.
- Astra LOW returned **implementation-scope PASS** at clean
  `1bd33f9bb2243d9aa89378f39f06d340aa7209d7`. Astra XHIGH alone is authorized
  to implement, only after the borrow-safety, layout/retained-peak, and
  accounting preflight, and only within the frozen paths and gates above.

Zero implementation attempts are consumed. This PASS authorizes neither full
RT-9 delivery nor timing, artifact, AudioWorklet-pin, or performance authority.
