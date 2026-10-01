# Housekeeping: soft-clip

## Authorized scope and smallest closable slice

Review the complete `crates/soft-clip` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

## Frozen boundaries

Existing public APIs, feature behavior, canonical/wire identities, arithmetic order and per-lane rendered bits, latency/tail, link modes, smoothing and NaN/denormal rules remain contractual. Zero render allocations/frees, locks, I/O or syscalls; no runtime native ISA dispatch. Retain scalar tails, 4-lane Wasm/NEON and x86 AVX2/FMA. Preserve all feature and target configurations. Do not expand another open issue or change DSP algorithms. Read current related issue bodies before touching their subject; discoveries that require architecture or product decisions become bounded follow-ups and final owner questions.

## Implementation and test-value decisions

Prefer deleting or consolidating repetition to adding abstractions that increase total complexity. Assess every existing test or a clearly named homogeneous family by its plausible unique defect; remove trivial, redundant or obsolete cases only after identifying the surviving behavioral gate. Keep independent numeric/oracle, fault, allocation, queue, boundary and target tests. Add/rewrite tests only for a concrete uncovered defect, and state which plausible defect no existing test catches. No prose/source-grep tests, new bit-digest pins or exact resource-byte pins. Copies needed for ownership, snapshots or atomic admission stay unless the same semantics are proved with less work. Data structure changes must preserve deterministic order and bounded realtime work. Inspect applicable hot loops and generated code before claiming additional SIMD; recursive/stateful dependencies alone do not justify changing arithmetic.

## Objective gates and evidence

- Read all production and test files in this package; record concise findings for each of the five requests, concrete changed/deferred locations, and load-bearing test families with retained coverage for deletions.
- Run focused locked package tests and affected feature configurations; use existing downstream/RT/differential gates proportional to the changed contract. Check formatting and package clippy with warnings denied. Relevant Wasm and AArch64 compile checks are required for changed product code; record limitations candidly.
- Changes to DSP arithmetic or hot state need existing independent numeric and scalar/SIMD gates plus one-time base/head evidence when needed; no permanent comparison against the old implementation. Existing research remains the algorithm authority; no new algorithm or listening claim is authorized.
- Benchmarks are optional and descriptive. Any timed measurement freezes its workload/validator, passes zero-workload preflight, and runs exactly one invocation with one warmup/two measured rounds. No timing optimization loop, performance percentage or unsupported sound-quality claim.
- Root conducts one adversarial verdict per coherent attempt, at most five total attempts. Every new/rewritten test gets its unique-defect sentence in that verdict. No-change audits require the same five-axis review, not manufactured edits.
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/soft-clip` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — implementation and evidence

Worker A (GPT-6.1 Sol xhigh) read all 19 package files before editing: manifest, kernel/driver,
padded-bank tests, corpus/pins, all integration tests, support and historical mutation notes.
Reviewed current #559 FX4/6/7/8/12–17 and LANE12, #560 dispositions, #1018, #1071, #1073 and
the frozen algorithm briefs 019/053. Root approved the bounded plan before implementation.
Product checkpoint `568ad408` is upstream through `d50b55fc`; no further product edits followed.

#### Five requests

- **Repetition/LoC:** `prepare_bank` validates and seeds its first member once, then enumerates
  from member 1; every later refusal still precedes incompatible-program/unsupported-width
  fallback. Removed the local unsafe allocator/armed TLS implementation in favor of the already
  installed shared audited allocator. Corrected stale benchmark, layout/migration, driver-owner,
  ramp-observable and boundary-report prose; kernel expressions are unchanged. Script-derived
  Rust delta: **71 added / 422 removed = −351**. Package including prose: 77/423 = −346.
- **Test value:** two active subset cases retired; no new test, seed, schedule, harness or pin.
  Complete current purposes and deletion survivors are below. Both real ignored reproducers stay.
- **Copies:** eliminated the corpus's intermediate lane readback array and the nonvacuity test's
  second `Vec<f32>`; borrowed the truncated payload slice. Distinct mutable oracle/input planes,
  donor payloads and transactional decode temporaries retain independent ownership. The 104-word
  channel payload, mirrored rings, lane extraction and kernel-current/ramp synchronization stay.
- **Micro SIMD:** inspected the generic kernel/halfband implementation and one current release
  `--emit=asm` artifact. `Channel<f32x8>::process` has contiguous `vmovups`, packed `vmulps` then
  `vaddps` in both FIR loops, explicit center insertion and packed `vdivps` in the cubic;
  `Channel<f32>::process` retains scalar `vmulss`/`vaddss`. No `vfmadd`/`vfnmadd` occurs in that
  artifact. Mirrored vector history stores implement the existing contiguous ring contract.
  No new SIMD or base instruction-count/timing claim; stationary splitting, constant splats,
  smaller dry storage and center-loop optimization remain in their existing issue scopes.
- **Data structures:** nonvacuity cardinality now sorts/deduplicates the owned `u32` result vector
  instead of allocating a `BTreeSet`, preserving exact word distinctions. Production storage is
  bounded boxed state/history with fixed masks; changing ring layout or capacities is outside scope.

#### Complete retained test purposes (30 active / 2 ignored)

- **`padding_tests` (5):** `every_padded_bank_renders_its_members_per_node_bits` catches inactive
  automation/coupling or failure to bind over every partial count, both widths, three unchanged
  seeded scenarios and drawn launch rates; `padded_banks_render_the_fixtures_per_node` compares
  the seven PCM fixtures with lane-distinct values/feed over the unchanged mixed block schedule;
  `active_lanes_do_not_depend_on_the_clone_source` catches parameter/request-index leakage from
  padding donors, including domain extremes. Every render compares all PCM words/reports, final
  active state and per-block idle padded state. `a_planted_nonfinite_state_recovers_and_is_charged_to_its_lane_alone`
  checks active and padded faults with an in-flight ramp, two-channel zeroing, history clear,
  target snap, counters, scalar continuation/state and unpoisoned bank-mates;
  `a_tripping_lane_leaves_every_bank_mates_bits` checks finite-gain overflow, infinity and NaN in
  full/partial cohorts over repeated faults. Native width binds publicly, other width internally.
- **`contract` (8):** descriptor/codec dimensions/resource refusal and independent f64 FIR design;
  f64 whole-chain error bound; the single frozen fixed-2x versus naive-cubic alias claim;
  impulse peak/support/tail; identity/bypass delay and signed zero; malformed/mixed/native bank
  binding; point order/channel/time/domain refusal; padded binding plus malformed active and
  cloned members. These retain the current algorithm, numeric, latency and admission owners.
- **`polyphase_identity` (1):** independent brief-derived zero-stuffed 63-tap scalar realization,
  ascending literal tap order and branching cubic; all ten unchanged 100,000-sample scenarios
  and odd-sized blocks remain. Catches reassociation, reciprocal division or wrong odd-phase age.
  This is a current mathematical realization, not a copied obsolete production block body.
- **`partition_invariance` (1):** every PCM bit and complete state of the 4,096-frame stream at
  partitions 1, 7, 64, 128, 512 and one shot, with sample-zero points and nonvacuity; catches
  cursor restart or block-local ramp arithmetic. Only the second identical partition-1 run went.
- **`ramp_law` (3):** every frame's current/target/step/countdown through final snap; descending
  ramp at block sizes 1/5/63/64/65/128; retarget from the actual mid-ramp current. Independent
  `LinearRamp::next_value` remains the current law and catches driver snap/step/restart defects.
- **`state_roundtrip` (4):** live-ramp scalar continuation and payload header; restored age order
  at two bank cursor positions plus sibling isolation; version/header/length/domain/step/countdown/
  three-history rejection and two-channel atomic refusal; full/default and discontinuity reset
  words. Removed only the unrelated assertion of a fixture array's compile-time length.
- **`boundary_check` (2):** warmed scalar NaN failure zeroes both outputs and continues as a fresh
  default instance, with full state parity; finite `1e35` identity-path failure proves magnitude
  checking. Existing frame-count reporting is preserved and explicitly documented as #1073 debt.
- **`allocation` (3):** scalar 1,000-block processing, both resets and preallocated snapshot/
  restore; native bank 1,000-block processing and discontinuity reset; bounded preparation/drop
  positive control. Shared warmed **current-thread** deltas require zero allocations and frees;
  reallocations count as allocations. Scalar points still name sample zero after the warm block,
  so they are rejected, and the bank loop has no spans: these scopes alone do not prove accepted
  automation allocation behavior. The conformance/randomized armed calls below own that behavior.
  The recast positive control catches inert per-thread allocation or free counters that would
  make zero-event claims vacuous, retains the existing allocation ceiling, and removes a duplicate
  preparation plus exact deterministic allocation-count comparison.
- **`determinism` (1):** unchanged six-case scalar corpus must be finite and have >16 distinct
  words; catches a degenerate corpus generator even if pins were regenerated. Removed the
  compile-time-derived length assertion; the corpus's own checked output shape remains.
- **`conformance` (1):** existing public-factory descriptor/preparation/process/state/control
  conformance and nonvacuous armed realtime allocation/free/lock/syscall/log hooks.
- **`randomized` (1 active, 2 ignored):** unchanged 8 seeds × 24 blocks, public full-bank versus
  scalar words/reports/state, whole/chunked processing, accepted points, resets/restores, hostile
  words and eligibility probes with coverage assertions. Only the existing #1071 refusal is
  narrowed. Its un-narrowed reproducer and fixed-input #1073 report reproducer remain ignored.

**Deletion survivors:** `the_hosts_bank_width_matches_the_scalar_instantiation` only ran the
native full bank with fixed 128-frame blocks; each ramp finished inside its starting block.
Variable-block padded differentials at both widths retain unequal parameters/automation and
segmentation PCM/report/state parity, while `randomized` owns full banks. A lane using another
lane's step/countdown is still red. `a_bank_block_fails_and_recovers_the_failing_lane_alone` was a
native-width default-state subset: planted-state recovery at both widths retains both-channel
zeroing, clearing/snapping, scalar continuation, exact reports/state and independent unaffected
bank-mates; the tripping test also includes full banks and infinity. A whole-bank reset/charge or
surviving bad history remains red. The old mutation log is labeled historical and points to these
current owners without inventing a new mutation run. Sole cross-target digest ownership stays G5.

#### Actual gates and limits

- Baseline locked all-feature package debug: **32 PASS / 2 ignored**. Focused candidate debug
  (`--lib` plus allocation/boundary/contract/determinism/partition/ramp/state/randomized):
  **28 PASS / 2 ignored**. Full candidate locked all-feature release: **30 PASS / 2 ignored**,
  including the unchanged polyphase and conformance bodies absent from focused debug.
- Strict package all-target/all-feature clippy `-D warnings`, package fmt, diff whitespace,
  workspace/lane/realtime policies: **PASS**. Two non-executable policy scripts were invoked with
  `bash` after their direct invocation returned permission denied; script contents did not change.
- Existing release `wasm-gates` **`g5_native_digests_match_pins`: 1 PASS** at its sole owner,
  checking the unchanged corpus/pins at scalar/W4/W8. One release soft-clip `--emit=asm` build PASS.
- Locked package `--all-targets --all-features` checks: **Wasm `+simd128`, iOS AArch64, Android
  AArch64 PASS**, with no compiler warning/error. Compile checks do not link or execute on those
  targets; current x86 tests run W4 internally and W8 natively. No bare-Wasm replay, listening,
  benchmark, wider target matrix or timing claim.
- Raw logs: `/tmp/housekeeping-a-soft-clip-{baseline,focused,release,clippy,g5,asm,wasm,ios,android}.log`
  and the matching `{workspace,lane,realtime}-policy.log` files. Inspected current assembly:
  `/home/bl/misofm/engine/target/housekeeping-a/release/deps/soft_clip-1f8811805db3ecbe.s` (W8 FIR lines 4251/4254 and
  4381/4384; center 4398; scalar driver 5700 onward).

**Deferred decisions:** #1071 subnormal self-snapshot refusal, #1073 frames-versus-blocks reports,
#1018 Apple stored-splat libc lowering, and #559 stationary/ring/flush-law work are unchanged.
No new owner question, API/algorithm/feature/dependency change, or resource/pin rewrite was found
necessary for this housekeeping slice.

### Root adversarial review — attempt 1

Root adversarial verdict: PASS. Reviewed the complete twelve-path checkpoint, full audit and retained family purposes, and the surviving padded/recovery implementations. Skipping the first member's repeated preparation preserves validation and refusal order, including later malformed members before fallback. Corpus readback and owned-word sort/dedup preserve every emitted bit and exact distinct-word count. The removed lane/recovery cases are subsets of current variable-block, full-bank and hostile recovery owners; their fixtures and independent references remain. The mathematical 63-tap realization and sole G5 corpus pins remain because they defend current algorithm and target contracts. Shared warmed thread counters strengthen existing realtime claims to include frees, and the preparation/drop control proves the measurements are nonvacuous. Native release and final affected debug gates, canonical digest, lint/policies and supported compilation pass with the explicitly recorded target and automation limits. Current codegen supports the SIMD assessment; no new arithmetic or speed claim is justified.

Test value: the rewritten scalar realtime gate rejects allocations or frees during both resets and preallocated snapshot/restore, calls outside the existing process-only allocation claims, while preserving its documented rejected-point limitation. The bank realtime gate rejects allocator activity during its native process/discontinuity scope. `preparation_has_bounded_allocations_and_the_counter_observes_frees` rejects an inert thread allocation/free counter that would falsely green both zero-event gates while retaining the current preparation ceiling. `the_corpus_is_not_vacuous` still rejects nonfinite or degenerate generated output without a second sample vector/tree; the canonical digest gate alone can be green after a mistaken pin refresh. The partition gate still rejects cursor restart or block-local ramp arithmetic against the retained sample-by-sample reference; only a self-comparison rerun was deleted. The renamed scalar boundary gate keeps its actual two-channel clear/report/state/continuation defect coverage, and the truncated-state refusal keeps transactional validation of both channels through borrowed bytes. No new test family or redundant comparison is introduced.
