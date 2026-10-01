# Housekeeping: console-workload

## Authorized scope and smallest closable slice

Review the complete `tools/console-workload` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `tools/console-workload` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — implementation and complete review

Implementer: GPT-6.1 Sol xhigh, worker A, on `bfdb4cc0`. Read all 10 package files: manifest,
`src/lib.rs`, `src/mixing_automation.rs`, all six integration files and the example (9,835 Rust
lines before this pass). Read current #881/#938, retired #885/#1004 and applicable R9/floor
evidence. #938's live producer, the meter delivery distinction, route association, paired-span
admission and outstanding DSP optimizations remain separate. No dependency/lock/API change.
Product checkpoints: `8794232e` (five paths) and `e6b3ad96` (known-size template destination).

**Five axes.** Repetition: reuse the already computed structural witness; consolidate the EQ
width twins; retire historical assertions and the strict placement subset. Copying: borrow kept
slot IDs and injected PCM; move the synthetic track template into its owner while preallocating
the known destination size; consume the scalar differential's owned PCM and the strip digest
vector. Track/route clones that create repeated independent tracks, bank/dual PCM, captured state
and aligned final source storage retain their ownership purpose. Data structures: the at-most-three
slot-ID membership scan stays ordered and linear; preparation's structural `BTreeSet`, normalized
claim sorting and test models' `BTreeMap` keep their deterministic join/replay purposes. Per-write
test tuple-key copies and small example/report buffers were left alone to avoid redesigning rigs.
Micro SIMD: this package delegates numeric render kernels to lane/builtins/effects. Its own source
callback is copying; the current release artifact emits two length guards and two 512-byte
`memcpy` calls. Played planes remain borrowed into the bank gather. No packed arithmetic is added
here, and no instruction-count/timing saving is claimed. Offline sine generation is unchanged.
Test value and deletion survivors are below; public workload identities, authored PCM words,
LCGs/seeds, control values, all retained schedules and arithmetic expressions stay fixed. Docs now
describe current collapse/pooling/padding/app placement, R9 and hash-based comparisons accurately.

**Retained behavioral families (64 active native tests).**

- `lib.rs` (14): driver facts/feed uniqueness, bound-versus-played output and in-place counts,
  omitted input dispatch, actual executor-table accounting; direct claim/copy/lend/quantum and
  short-buffer refusal with independent track-position/channel expectations; source/output
  alignment; meter facts/emission and real snapshots, sample-peak mask, cadence, spans, drop and
  discontinuity counts, unchanged memberships but 48 versus 40 delivery stages; strip facts/order,
  first-N track bodies/routes, app slots and every-third bypass, malformed/partial bypass censuses,
  exact odd-track zeros and mixed activity, and all five strip rows' audible/distinct/RT scopes.
- `automation.rs` (10): threshold restatement versus movement on every block, stable channel
  selection and absence refusal; eight resolved controls/held bases, alternating values and
  restatement admissions, no-control refusal, all preflight nonvacuity arms, and EQ `Both` target
  lowering versus two one-channel transactions and their collapse consequences.
- `chain_shape.rs` (22): intended/legacy/ragged chains and transposes, facility equivalence and
  armed observations, per-row fold counts/order, half-mono pooling plus structural/runtime join,
  vacuous seam classification, mono/dual pairing and unit census, identity-pair activity, folded
  versus forcibly declined reduction; engagement, stop/start/repeated transitions, live asymmetric
  retarget, bypass lift/engage and re-equalized targets, and right-channel taps against forced dual.
  Each transition owner checks the particular history transfer or refusal its schedule reaches.
- `eq_ramping_scenario.rs` (2): EQ-only Left/Right targets and mono `Both` targets, every supported
  vector width, 64-block pre-roll and 128-block settled/eight-of-64/all-64 rides; activity contrasts,
  current width equality and mono engagement. Independent current LIST/full-section differentials
  in parametric-EQ still own numeric section elision and ramp/state behavior.
- `paired_spans.rs` (13): real EQ/compressor/limiter/chained banks against forced dual by every PCM
  word, report and full lane state; three seeded rides, hostile words, last-wins/near/split/lone
  pairs, target FIFO interleavings, resets/restores, exact queue refusal and missing halves; every
  mixed-drain permutation against a final-value model; collapse-capability census; all registered
  effects/link modes' symmetric validity over domain edges/NaNs/infinities/signed zero; exact
  staging-window capacity refusal; per-block console engagement/restatement/asymmetric retirement;
  128-block mixed ride, all isolated-effect activity controls and supported-width equality.
- `placement.rs` (2): limiter activity and current merged/split equality for each of 64 blocks.
  `gain_pan_profile.rs` (1 active, 2 unchanged ignored helpers): real probe phase/entry counters,
  nesting, routes and output parity over 16 blocks. Ignored descriptive timing/digest-print helpers
  were neither run nor rewritten. The example still emits eight controls and two document facts.

**Retirements/recasts.** The whole-run placement comparison is a strict subset of the surviving
per-block comparison and `the_two_placements_realise_the_same_chain_shape`. The old select-free
four-row SHA pin adds historical outputs; current owners are lane's
`select_free_matrix_matches_the_select_form_when_no_lane_is_identity` (all widths, hostile
families, frame counts and guards), builtins' `settled_matrix_takes_the_select_free_arm_only_without_an_identity_lane`
(full/partial/identity banks and ramp tails), `settled_identity_matrix_preserves_signed_zero`,
the D11/f64 matrix references and the folded/declined console reduction. Wrong matrix coefficients,
identity selection or tail dispatch remain observable there. Driver/meter/strip old pins retire
while their current parity, exact model/input, activity, RT and snapshot assertions stay. Four EQ
historical width functions become two workload families with all six scenarios per x86 workload;
the mixed-ride historical pin becomes a current width comparison with every isolated-effect run.
No fixture digest or G5 owner changes. Rust: 112 added, 294 removed, **−182**; four net active
functions removed, distinct from unchanged input/scenario counts.

**Rewritten-owner defects.** Injected driver: a borrowed table taking the wrong track prefix or
channel mapping now fails the independent expected words; Local is still exercised elsewhere.
EQ width families: a width-specific preparation/render or track-lookup divergence, ignored ride or
indistinguishable ride arms fails current equality/activity for the existing EQ-only and mono schedules. Mixed ride:
width divergence or an inactive effect ride fails current equality or that effect's isolated control.
Strip activity: an inaudible row, forbidden render operation, or sparse/strip row collapsed to one
of its current controls fails; exact derivation remains owned by the adjacent model/source tests.

**Allocator correction.** The initial inference that these binaries lacked an allocator was wrong
and is retracted. Actual executable symbols show bench-support's `GLOBAL_ALLOCATOR` and all four
`AuditedAllocator` methods; their provider reports alloc/zeroed/realloc/free into engine's thread
audit before System. Digest use links that provider. Scoped zero snapshots therefore include real
allocator events; explicit forbidden-operation controls prove hook nonvacuity, not a real allocator
positive probe by themselves. Other operation categories are explicit hooks plus policy coverage.

**Actual gates and limits.** Baseline debug: 68 active PASS, two ignored. Changed debug: 53 PASS
(14 lib + 22 shape + 2 EQ + 13 paired + 2 placement); destination completion: first-N and ragged
owners each PASS. Complete final release/all-features: **64 active PASS, two existing ignored,
zero doctests**. The unchanged automation/probe debug binaries were not rerun; final release covers
them and the final preallocation. Final all-targets/all-features strict clippy, fmt/diff: PASS,
no warnings. All-targets/all-features Wasm `+simd128`, iOS and Android AArch64 checks: PASS without
warnings; compile evidence only, no device/Wasm execution. Workspace/bench/realtime/lane/boundary
policies PASS (54 marked realtime regions). Canonical fixture regeneration/witness gate PASS;
the initial explicit call refused a missing validator, then the existing validator was built and
the gate resumed successfully. The untimed example emits valid JSON, eight controls, two documents,
bypass counts 0/21 and unchanged 64/64/64 pre-roll/preflight/smoothing. No timed CLI, new harness,
benchmark, fixture corpus, listening claim or additional sweep. No owner question is required.

Raw logs: `/tmp/housekeeping-a-console-workload-{baseline,focused,focused-shape,focused-clippy,template-lib,template-shape,release,final-clippy,wasm,ios,android,policies,validator-build,fixtures,controls,allocator-symbols,codegen}.log`;
example JSON: `/tmp/housekeeping-a-console-workload-controls.json`. Codegen uses the release
unit executable `console_workload-7596d3576611b83a` produced by the complete gate.

### Root adversarial verdict: PASS — attempt 1

Root Sol inspected every changed line in all five Rust paths, the final destination-capacity completion, structural-witness lifetime, frozen source construction/ownership, supported-width loops, surviving placement/matrix/reduction owners, full family map and actual logs/artifact excerpt. Reusing the immutable witness is valid; the borrowed table is copied only into the required aligned owned source; template ownership and track/route derivations remain unchanged. The final known-size destination avoids growing an empty vector. No render arithmetic, source generator, workload/report identity, admission, latency or feature behavior changed.

The historical matrix pins are superseded by the current independent select-form, D11/f64 matrix and dispatch/tail owners; placement equality retains each block and plan shape. Current width equality does not defend common-mode arithmetic errors by itself: the existing numeric and full-word/state differential owners provide that evidence. Single-width targets retain activity/schedule gates, while current cross-target corpus ownership remains independent. The allocator linkage correction is explicit and the actual symbols are present; hook controls are not misrepresented as installed-allocator positive probes.

Test-value verdict sentences: the Injected driver owner uniquely catches a wrong borrowed track prefix or channel mapping in that table path, with independent expected words and direct copy/lend/refusal calls. The EQ-only family catches whole-plan Left/Right ride lookup or width-specific preparation errors over the retained settled/eight/all schedules; the mono family separately catches Both-target lowering and collapse behavior over those schedules. The mixed-ride owner catches a missing effect ride or width divergence in the composed production queue schedule, with each effect's isolated activity control. The strip activity owner catches an inaudible or conflated generated row and forbidden render operations across all five ragged/app/sparse subjects; adjacent exact-model/source owners retain their distinct derivation claims. The scalar differential's consuming move changes storage ownership only and retains the existing independent comparisons. No new test or unsupported historical/private pin is introduced.

Root's independent frozen-snapshot integration (`e39299af`): `cargo test --locked -p bench -p console-workload` passed **77 active / two existing ignored / zero doctests**, exit zero and no warnings, in `/tmp/engine-housekeeping-bench-console-test.log`. Complete final package release and supported-target/lint/policy evidence is sufficient. PASS approves this bounded housekeeping issue; existing optimization and owner follow-ups remain separate.
