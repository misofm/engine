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

Admit resident input only when all of these are proven at preparation time for
immediately adjacent emitted bank units:

- backend, bank width, render quantum, populated-lane set, and lane order match;
- every successor first-slot input is exactly the predecessor's final output;
- the successor has one undelayed main input and no reduction, sidechain, or
  staging path for that input; and
- the retained words are the same planar/AoSoA values that the old successor
  gather would have acquired.

Decline on any mismatch, folded or transformed output, nonadjacency, or identity
that cannot be proved from the lowered program and prepared metadata. The
existing `chains_into` extra-reader, direct-observer, and alias-observer declines
remain unchanged. Do not weaken its reader, send, sidechain, session-output, or
alias-observer proofs and do not turn this child into a general chain executor
rewrite.

## Safe resident-copy API and word semantics

Because `BankChain` scratch is private, authorize one narrow safe rack method (or
equivalent trait hook) for graph's adjacent prepared-unit path. Freeze the
preferred shape as:

```text
BankChain::copy_resident_input_into(
    &self,
    successor: &mut BankChain,
    frames: usize,
    mode: ResidentInputMode,
) -> Result<(), ResidentInputError>
```

The method copies from the predecessor's current resident output into the
successor's existing scratch. It exposes no mutable scratch, retains no
references, and is callable only for the graph's adjacent prepared units. Its
typed error/fallback must reject at least width, frame-count, mode, active-lane,
nonadjacency, and unavailable/current-block mismatches. The unchanged `run`
method remains the old acquisition path and remains valid for every decline.

The copy is mode-aware and overwrites exactly the words the successor's old
acquisition would overwrite:

- partial gathers write only active lanes;
- mono gathers leave right-channel scratch untouched; and
- inactive lanes and untouched right scratch remain unchanged, or a separate
  proof establishes that those words are unobservable.

The allocation/equivalence fixtures must poison inactive lanes and right scratch
to discriminate an overbroad copy. No public mutable scratch API, retained
borrow, unsafe alias, or caller-trusted compatibility flag is authorized.

## Current-block freshness and placement

Resident input is valid only after predecessor execution succeeds and its
predecessor observation completes. Successor `begin_block` and collapse
decisions occur in their original order before resident acquisition. Graph uses
safe disjoint borrowing, such as a split unit slice, and never aliases the two
owners. A producer process or observation failure prevents B from executing; it
falls back by not executing B and never by reusing stale resident data.

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
controls, nonadjacent controls, and the forced old acquisition path.

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
- The safe resident-copy method exposes no scratch or retained references,
  returns a typed/explicit fallback for every frozen mismatch, and leaves the
  unchanged `run` path available.
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
this correction consumes no implementation attempt. No RT-9 full-delivery or
performance credit is claimed.
