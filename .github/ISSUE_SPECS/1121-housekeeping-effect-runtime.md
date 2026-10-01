# Housekeeping: effect-runtime

## Authorized scope and smallest closable slice

Review the complete `crates/effect-runtime` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/effect-runtime` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — GPT-6.1 Sol xhigh worker A, 2026-10-01

Read every production/test file, the manifest and mutation record (4,814 Rust lines at
`e24388e9`), plus related #889/#893/#894/#1069 and #559's DYN7/DYN16/FX15/FX16 boundaries.
Root product checkpoint `e659f516` changes seven package files: +92/-299, **net -207 LoC**.
Production changes only `src/bank.rs` (+1 net line); the practical reduction is in test scaffolding.
No manifest, dependency, lockfile, API, state format, numeric law or feature changed.

#### Five-axis findings

| Axis | Finding/decision |
|---|---|
| Repeated code | `check_block` and `nonfinite_lane_mask` now share their identical `#[inline(always)]` ordered-finite vector mask scan. Same limit splat, initial true mask, load/abs/strict comparison/AND sequence; wrappers retain their debug guards, reduction and lane attribution. Clean/failing channel dispatch, paired reset, saturating counter and positive-zero admission are unchanged. No kernel arithmetic moved. |
| Test value | Deleted the pre-hoist ramp body and old knee-design body. Stationary tests use direct constant, signed-zero and segment expectations; the exhaustive knee gate states the current finite-reciprocal law. Three obsolete/overlapping stationary cases and a redundant clean-channel call were removed as mapped below. The identical 512-frame partition comparison was removed; all true split runs remain. |
| Avoidable copies | Ramp block tests reuse one buffer per run instead of allocating each block, preserving the exact scalar sequence comparison and all lanes. Boundary identity tests reuse one fixed 64-word array for every dirty position and check the identical clean buffer once. Stationary checks no longer allocate old/new output Vecs or run both arms twice. Mutable channel buffers and codec outputs still need distinct storage; PCM snapshots and state handoff copies keep their ownership/transactionality. |
| Micro SIMD | The mask scan already operates through Lane at W1/W4/W8 and reduces only after the block. Followers/dynamics/hysteresis are existing lane-generic bodies; slice fills/copies and the tuned positive-zero bit fold remain. No new ISA path, dispatch, reassociation or speedup claim is justified. This refactor does not change the Wasm multiply/add, NEON, AVX2/FMA or scalar/tail arithmetic. |
| Data structures | Prepared coefficient/left/right Vecs provide direct slot indexing and fixed capacity; render never changes their lengths. Ramps/coefficient/state words are Copy values, enum choices are small borrowed static slices, and the payload codec writes caller-provided storage only after all checks. Corpus scratch is fixed stack storage; its test-only distinct-output HashSet has a useful coverage floor. No production collection/layout replacement earns its cost here. |

#### Complete retained behavioral-family assessment

| Test/family | Plausible defect defended |
|---|---|
| Positive-zero unit tests: all lengths/chunk edges/seeded positions and two-word pairs | Lost head/remainder words, sign masking, or XOR/wrapping-add cancellation admits nonsilent words. The naive all-words bit predicate is independent of the optimized fold. |
| Bank finite/threshold/NaN and lane-mask tests | Wrong magnitude inequality, missed NaN, clean mask falsely nonzero, lost highest lane or incorrect attribution. Includes scalar, W4 and native W8. |
| Bank paired rejection/counting, right failure and per-channel finish | Missing zero writes/reset, clean reset, counter per sample, right-channel short circuit or conflating paired and single-channel recovery. |
| Bank identity/divergence/reset | Absent slot changes PCM, unstable recurrence escapes recovery, state survives rejection/manual reset or coefficients are undone by reset. |
| Contract ramp identity/current-law/None/OnePole endpoint | Control/render linear increments or trajectories disagree; both implementations drift from iterated constant addition/final assignment; None fails to snap or OnePole misses its declared final update. This is the current smoother gate home referenced by effect-contract's mutation record. |
| Ramp scalar step/snap/zero length/rest/constant segment | Event-time increment changes, final sample accumulates, immediate reset does not snap, or settled public segment/state words are wrong. |
| Ramp block sequence and state partition | Segment off by one, stale scalar progress, divergent vector lanes or block-dependent state; multiple ramp lengths and {1,7,64,128,512} block sizes remain. |
| Stationary redundant finite/subnormal and signed-zero/nonfinite/one-ULP boundaries | False negative stationary admission, subnormal flushing/exclusion, float-equality hoist swallowing -0.0/sign changes, nonfinite admission or tolerance swallowing a real movement. Negative-zero updates assert all 63 intermediate positive zeros and the exact final negative-zero snap. |
| Stationary mid-flight retarget and next-block settled segment | Hoist incorrectly requires rest, keeps an old target/step, or a redundant retarget after a closing block produces a ramping/wrong segment. Direct checks retain exact live value and whole settled state. |
| Envelope coefficient design/degenerate arguments and follower restatements/extremes/D8 | Wrong exponent/time/rate or complement, divergent coefficient, wrong rounding/product order, NaN swallowed or wrong signed-zero/endpoint result. Independent f64 coefficient and unfused rounding oracles remain. |
| Envelope hysteresis hold/countdown/lanes and AR complements/strict switch/two-product/flush/width | Off-by-one close, no hold, negative countdown, neighbor contamination, wrong rising equality, one-rounding deadband, nonzero decayed state or width-specific recurrence. |
| Dynamics paper/grid, knee edges/hard/nonpositive/identity/slope, dB conversions/floor/NaN | Wrong equation/domain branch, discontinuity, 0*inf, changed identity or expansion slope, wrong level conversion, silent infinity or NaN suppression. Independent paper equation in f64 remains. |
| Dynamics exhaustive finite-reciprocal words, overflow/narrowest knees and randomized curve | One-ULP knee boundary error, wrong exact coefficient words at widths not covered by directed curves, nonfinite narrow-knee output, scalar/vector discrepancy or paper error over the generated domain. All 8.4 million subnormal widths and one million sampled widths remain, plus the 100,000-case differential sweep. |
| Lane corpus/segments/boundary/attribution identity | Backend-dependent corpus stride/arithmetic, segment state, dirty-position omission or lane index. W1/W4/W8 comparisons remain. |
| Composed processor partitions/width identity | Follower/ramp history is reset or advanced differently at a real block boundary. Every output/state compares {1,7,64,128} splits to one 512-frame baseline at W1/W4/W8. |
| Corpus NaN-free and distinct-output floors | Generator introduces a wasm-canonicalized NaN or collapses useful domain coverage. D1 digests stay with the single cross-target G5 owner; no duplicate/historical pins were added or removed. |
| Params validation/kinds/zero/clamp, mapping/inverse/degenerate/defaults | Inclusive bounds or exact enum/boolean admission changes, nonfinite/negative-zero leaks, wrong mapping slope/endpoints/round trip, degenerate NaN or wrong default write count/clamp. |
| Payload sizes/endian/float bits/round trip and length/version/count/word-slice/transaction rejection | Wrong handoff header/section sizes or byte order, lost sample words, wrong typed rejection or partial writes after refusal. Exact byte lengths are the codec's format claim, not an incidental memory-budget pin. |

Deletion survivors: `a_redundant_change_arriving_mid_block_is_partition_invariant` only called
`next_value` per sample, without block partitioning; mid-flight retarget, next-block segment and
the genuine ramp/composition partition gates retain its defects. `the_rest_invariant_survives_the_hoist`
overlapped `ramp::the_rest_invariant_holds` and the new stationary helper's full settled-state checks.
`the_contract_smoother_hoists_linear_and_refuses_one_pole` only asserted a finite first OnePole
output, which passes a mistaken hoist; the retained contract endpoint/linear-law checks provide
its meaningful coverage. The removed unrelated `finish_channel(other)` call repeated the clean
path and could not establish independence from the failing buffer; clean/reset and dirty-mask
checks remain in the same test. The repeated partition 512 run compared the same call to itself.
Test count falls **89→86** (2 unit +84 integration); no independent numeric, fault or state gate was deleted.

Rewritten-test value: direct stationary expectations catch mistaken hoisting/exclusion and
unfinished retarget/segment state at the named boundaries; explicit knee words catch a wrong
finite coefficient at any enumerated width even when curve tolerances and directed points pass.
Buffer reuse keeps the existing dirty-position, every-frame/lane and block/scalar witnesses.

#### Actual checks and limits

All Cargo commands used `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-a`.

- Base locked package all-feature test: 89 passed. Intended final tree: **86 passed** in debug
  and release; zero doctests. The package declares no features. Existing tests execute W1/W4/W8
  lane/current-law/numeric/partition parity in this native AVX2/FMA build.
- Locked package `clippy --all-targets --all-features -- -D warnings`: exit 0, no warnings.
- `cargo fmt --all --check`, `git diff --check`: passed. Effect-runtime, realtime (54 marked
  regions/15 files) and realtime-audit-leak policies passed via `bash`; initial direct policy
  invocation lacked executable permission, so no file mode was changed.
- Locked all-target/all-feature compile checks passed for Wasm with
  `RUSTFLAGS='-C target-feature=+simd128'`, AArch64 iOS and Android. No target binary execution,
  new generated-code/performance evidence, wider downstream matrix or timed benchmark is claimed.
- No dedicated allocation-counter test exists in this package. The only production edit shares
  existing load/compare/mask operations and changes no storage/ownership; source review and policy
  gates support unchanged allocation freedom without claiming a new dynamic measurement.
- One generated patch initially reached the primary checkout by a relative path; its exact diff
  was restored, moved to the absolute owned worktree path, and the full focused gate rerun before
  checkpoint. Root verified the primary clean; final gates refer to the intended checkpoint.

Deferred: #559's cross-crate recurrence/ramp/PRNG consolidations and #889/#893/#894/#1069 remain
outside this slice. Existing coefficient documentation says every nonfinite time is instantaneous,
but the current +infinity path yields retention 1/rate 0 (freeze); finite parameter-domain callers
exclude it. This documentation/edge-contract discrepancy was reported to root without changing
behavior or executing an additional probe. No owner API/algorithm decision is needed for this
housekeeping slice. Worker paused product edits at the focused-green checkpoint; root verdict and
GitHub synchronization remain pending.

### Root adversarial review — attempt 1

Root adversarial verdict: REQUEST CHANGES. The shared mask body preserves strict threshold/NaN comparisons, accumulation order, empty-block behavior, debug guards and lane attribution; retained numeric/width/partition/fault gates pass. The main historical test scaffolding is correctly removed, and direct stationary and finite-reciprocal expectations preserve useful contracts. Two bounded cleanup items remain: the subnormal test's `(value + 0.0)` assertion attempts to check a global FP environment through a constant-foldable expression, rather than defending the runtime's hoist behavior; remove it while retaining all actual predicate/state/output checks. Also correct the discovered envelope documentation to describe the unchanged NaN/nonpositive, zero-rate and positive-infinity/overflow behavior. Neither correction requires a product decision or changes DSP semantics.

Attempt 2 is authorized only for that assertion deletion, accurate coefficient documentation and concise evidence. Run the stationary family, formatting/diff checks and package clippy; reuse the completed product/target/RT gates because production expressions are unchanged. Test value: retained stationary cases reject losing exact subnormal words, incorrectly arming a redundant target, swallowing a signed-zero or one-ULP movement, or leaving a stale target/step after mid-flight and block-boundary retargets. The current coefficient-law gate still rejects a wrong finite reciprocal or one-ULP overflow boundary over its existing dense domain. No new test or benchmark is needed.
