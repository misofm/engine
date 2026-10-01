# Housekeeping: lane

## Authorized scope and smallest closable slice

Review the complete `crates/lane` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/lane` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — worker A Sol, 2026-10-01

Read the manifest, all thirteen production Rust files and every integration-test/support file,
including the in-source dispatch tests and historical mutation evidence. Related open specs read:
#1112 (width availability), #1018 (Apple stored-splat calls), #890 (ramping output fusion) and
#883 (held-peak specialization). Those separate outcomes are not implemented here.

**Five review axes and decisions.**

| Axis | Finding and action |
|---|---|
| Repetition/LoC | `exp2_int`'s identical max-then-min clamp now lives in the `Lane` default. Scalar and vector implementations retain their own in-range bit construction; signatures, overrides, operand order and rounding are unchanged. Existing vector bodies are already one macro, and SVF recurrence/output policies already have a single home. Leave the specialized input and matrix bodies intact: they encode established loop schedules, mono ownership and absence of identity selects. |
| Test value | Remove the redundant G1 max/min lowering subpool test, preserving all its exact inputs in the larger directed differential. Its `0xFFC00001` NaN row was absent despite the former subset comment, so that row moves into shared `EDGES`. Retain scalar signed-zero/NaN witnesses: they can reject a shared scalar/vector semantic mistake that a differential alone cannot. Remove P1's assertion of a literal corpus-width constant and the sanitise test's lifetime equality between two totals both incremented from the same oracle count. Per-block kernel-vs-oracle assertions and non-vacuity remain. |
| Copies | G2 now borrows each lane-signal group rather than deep-cloning it, and extracts bits directly instead of materializing an intermediate deinterleaved float vector. Delete the unused deinterleave helper. Keep independent mutable oracle/DUT buffers and register-local DSP state copies: they serve differential independence and bounded state load/store. |
| Micro SIMD | Scalar/four/eight-lane optimized `exp2_int` probes inspected before/after. W4/W8 assembly is identical and still uses packed max/min/add/shift. Scalar code complements its upper-bound compare and swaps the blend arms, with the same selected value and instruction count. No new SIMD or speed claim: vector kernels, sample-serial lane-parallel state and exact f64 widening already earn their shape. Reassociation across FIR taps or recursive samples would change the numeric contract. |
| Data structures | Replace halfband differential's scalar prefix replay for every frame with one forward traversal per lane: O(F²·W) oracle work becomes O(F·W), with all 512 frames, both vector widths and every lane/wrap still asserted. Scalar history/calculations remain separate from the vector driver. Remove its unused scalar run/length check. Production uses fixed arrays and slices; the halfband's mirrored history deliberately pays two stores to keep the window contiguous without gathers/modulus. No map/tree/queue change is justified. |

Implementation checkpoint: `690f0fd7`. Package changes: **27 added, 107 removed, net −80 lines**
across nine files (production net −9, tests/support net −71); this evidence text is additional.
No new test, dependency, API identity, DSP algorithm, parameter, state layout or benchmark was added.

**Retained load-bearing tests.** Each row is an explicitly named homogeneous family; distinct
numeric, fault, boundary, state and target witnesses remain separate.

| Existing family | Plausible defect it rejects |
|---|---|
| G1 directed/random operations | Wrong SIMD operand order, payload/sign transfer, comparison/mask or bit-field lowering across heterogeneous lanes; the directed pool now strictly contains the deleted min/max pool. |
| G1 signed-zero/NaN/FMA witnesses | A common scalar/vector change to D8 ties/unordered operands or fused arithmetic that would defeat differential agreement. |
| G1 mask-any, exp2 integer-range, frexp reconstruction | Ignoring the last set lane, wrong exact exponent/clamp, or inconsistent significand/exponent fields. |
| G2 width/kernel identity, idle ramp, subnormal state | Lane/channel leakage, an idle coefficient update, or failure to flush recursive state on silence. G2's borrowed inputs/direct extraction preserve every previous signal, seed, width and comparison. |
| G2 ordered accumulation and lane2 bounds/categories | Contributor reordering, loss of initial signed-zero copy, admission of invalid contributor count/shape, writing before short-input rejection, or corrupt scalar tails/long-input prefix behavior. |
| G2 independent two-tap SVF and interleaved cascade | Swapped taps/coefficient slots, missing incoming state, or incorrect section/state ownership; both output and integrators are judged. |
| G2 skewed and bounded cascades | Wrong carry/prologue/epilogue addressing, guard overwrite, ignored stream or wrong strict boundary verdict, including dry-selected stored words. |
| G4 and G6 | Inclusive flush threshold, cross-lane clearing, or hardware FTZ dependence in flushed state; G6's unflushed control proves the hardware mode really changes arithmetic. |
| P1 partition family | Dropped state or incrementally advanced gain/coefficient continuation at block boundaries; the deleted literal-width assertion guarded no behavior. |
| Halfband table/history/width identity | Incorrect halfband structure or age at any ring position, and width-specific history/address/arithmetic divergence for every frame/lane across wrap. The rewritten oracle keeps all comparisons; independent FIR-design/full-graph evidence remains in soft-clip. |
| FP environment and compile-fail guard traits | Incorrect canonical word/status masking, lost caller state on normal or unwind exit, FTZ exposure, or moving/sharing a thread-bound guard. |
| f64 widening/add/mul/square families | Incorrect exponent/subnormal conversion, lane permutation, rounding or square exactness; independent integer oracles, hand-known oracle values, tie witnesses, directed/sparse and optional exhaustive cases remain. |
| Input elision patterns, poison/zero words, frozen mixed bodies and trim ramps | Incorrect identity admission or add position, elided-state writes, mono right-channel writes, missed adverse output report, or ramp endpoint/continuation drift. |
| In-source mixed/identity-ramp dispatch tests | Moving immutable shape selection into the frame loop or dropping the identity-ramp specialization while output equivalence still passes. |
| Masked SVF family | Dry selection at the wrong section boundary, stopped dry-state evolution, lost signed-zero input into downstream sections or changed ramp/partition state. |
| Fader/matrix families | Mute implemented as signed-zero multiplication, use of overwritten left input, wrong matrix coefficient/order, select-free admission with an identity lane, overflow/NaN mishandling or non-neutral padding. |
| Meter peak/full families | Wrong normal-or-zero/clip boundaries, cross-lane seed leakage, wrong published counts or reassociated binary64 energy; independent serial oracles, invalid-only lanes, boundary bit sweeps and mathematical/rounding witnesses remain. |
| Sanitise counter families | Misclassification exactly at ±1e30/NaN, a per-block rather than per-sample count, or broadcasting one lane's count; all subsets, three kernel/elision paths and 64 carried blocks remain. |

**Actual checks.** All Cargo commands used
`CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-a` and `--locked`.

- Baseline: `cargo test -p lane --all-features`: 67 executable tests plus 2 compile-fail doctests;
  release G1: 9 passed.
- Head: the same package command: 66 executable tests plus 2 compile-fail doctests passed; G1
  reran after migrating the exact NaN corpus row: 8 passed. Final release G1/G2/halfband/P1/
  sanitise-counter with `--all-features`: 23 passed.
- `cargo test -p math --features lane`: 33 passed, 14 pre-existing exhaustive cases ignored;
  release `--test m2_lane_identity`: 4 passed, 1 pre-existing exhaustive case ignored. Existing
  directed callers, lane parity and math corpus pins passed without repinning.
- `cargo clippy -p lane --all-targets --all-features -- -D warnings`; package formatting and
  `git diff --check`: passed.
- `cargo check -p lane --all-targets --all-features --target aarch64-apple-ios`, the Android
  arm64 twin, and wasm32 with `RUSTFLAGS='-C target-feature=+simd128'`: passed, including tests.
  These are compile checks; no mobile target runtime or Wasm execution was claimed.
- `scripts/check-lane-policy.sh`, `scripts/check-unfused-seal.sh` and
  `scripts/check-realtime-policy.sh`: passed.
- One-time optimized probe source/assembly was kept outside the worktree at
  `/tmp/lane-housekeeping-1116/`. A whole-file byte comparison intentionally did not pass because
  of the scalar complementary compare/blend; direct inspection found identical W4/W8 bodies and
  the scalar equivalent described above. This is PR evidence, not a permanent byte pin.

**Deferred decisions and owner questions.** None required to close this slice. The #1018
stored-splat compiler defect and #890 ramp-fusion outcome remain with their existing issues;
this cleanup does not claim to fix them. No timed benchmark or listening result is claimed.
Root Sol's adversarial verdict and upstream/GitHub synchronization remain pending.

### Root adversarial review — attempt 1

Root adversarial verdict: PASS. Inspected every package diff, including the formerly missing negative NaN input, the unchanged clamp ordering, borrowed G2 inputs, independent forward scalar halfband history, and removal of oracle-to-itself assertions. The shared default retains implementation-specific bit construction and introduces no allocation, target dispatch or structural render work. Integrated lane/math/wasm-gates all-feature tests passed, including unchanged G5 pins and G6 canonical-FP replay. Worker target results are compile checks, with native target execution left to the existing qualification jobs.

Test value: the rewritten halfband test rejects a width-specific history/address/arithmetic divergence at any frame/lane across ring wrap that the table and scalar age tests cannot catch. G2 still rejects kernel composition/lane differences and now borrows immutable signals; P1 still rejects partition-dependent continuation; sanitise-counter still rejects classification/count errors per block and path. The migrated G1 NaN row preserves every deleted lowering input, while scalar D8 witnesses protect against a shared incorrect oracle. No new permanent test, digest or byte gate was added. All five housekeeping axes have concrete findings; no owner decision is required.
