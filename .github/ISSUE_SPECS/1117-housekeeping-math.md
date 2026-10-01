# Housekeeping: math

## Authorized scope and smallest closable slice

Review the complete `crates/math` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/math` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1: audit and consolidation

Implementer: worker B, GPT-6.1 Sol xhigh, 2026-10-01. Read the complete package: public wrappers, corpus, lane/fast polynomials, the entire vendored/helper module tree, six integration test files, manifest and provenance; reviewed related #1019, #1112 and #1008 scopes. The product source and M3/corpus generation/pins are unchanged.

Removed the permanent pre-E1 old-body comparison and moved its useful exhaustive exp2 scalar/vector qualification alongside log2 in M2. F1's eight crossing labels now share the four distinct domain sweeps, retaining the same input sets, independent log10/pow definition oracle and bounds. Deleted the identical X7 exhaustive replay; its full 257,176,458-input premise moved into the surviving level exhaustive gate. M2 compares one vector result buffer at a time and streams the positive-normal filter. Each M3 gate reuses one output buffer; domain cardinality sorts it in place rather than copying its words into a HashSet. `run_case` assigns every output slot via `out.iter_mut()`/`*slot = eval(...)`, so reused or sorted storage cannot leak previous cases. All three independent M3 assertions and pin-mode refusal remain.

Actual attempt-1 checks: scalar default tests 12 passed/2 ignored; release lane tests 26 passed/13 ignored; all-target/all-feature Clippy with warnings denied; formatting, lane policy and workspace policy. All exited 0. All-target/all-feature compile checks passed for Wasm simd128, AArch64 iOS and Android. Temporary bounded calls exercised the factored exhaustive helper at 13 aligned ranges around zero/subnormals, fold/normal/clamp edges, infinities/NaNs and the end of the u32 space, for exp2/log2 at scalar and both x86 vector widths; the temporary test was removed and the source restored exactly. Logs: `/tmp/issue1117-math-{default,lane-release,clippy,helper-boundaries}.log` and `/tmp/issue1117-math-check-<target>.log`.

Root checkpoint: `56d1204e`, pushed through delivery branch checkpoint `efc0f31d`.

### Sol adversarial verdict: attempt 1 — REQUEST CHANGES

Root Sol found two remaining tests without current unique coverage: `f1_lower_degree_refits_exhaustive` only remeasures rejected test-local candidates; `f1_exhaustive_x8_transient_shaper_applied_gain` repeats a subset of the retained gain exhaustive sweep. The user's authorization to remove unnecessary tests covers both. A bounded second attempt was authorized to delete their unused machinery while retaining the current-polynomial, independent-definition and composed error/domain gates. No algorithm or product change was requested.

### Attempt 2: bounded correction and final audit

Removed the rejected degree-candidate constants/functions/test; their research evidence remains in #880 F-3 and git history, and package references now say so. Removed X8's [-18,-0]/[+0,18] exhaustive repeat: both ranges are strict subsets of the retained [-160,-0]/[+0,24] sweeps, using the same measurement and error ceiling. The retained gain sweeps now require all 1,126,170,625 negative and 1,103,101,953 positive inputs, preserving the removed subset's complete-coverage premise.

| Requested axis | Finding and disposition |
| --- | --- |
| Repetition / LoC | Consolidated equivalent test domains and width sweeps; deleted historical old-body and rejected-candidate gates. Package total versus `72893fca`: 326 fewer lines (322 Rust, 4 manifest). Production wrappers and vendored routines retain explicit precision/operation differences and provenance; a generic abstraction would add complexity or obscure frozen arithmetic. |
| Test value | The families below retain independent numeric, boundary and width coverage. No new behavior scenario or permanent old-output comparison was added. Tests fall from 47 to 37; all 26 default lane tests remain meaningful and 11 long qualifications remain ignored by default. |
| Copies | Removed M2's positive-normal collection and simultaneous per-width result ownership; reused M3 buffers and removed the cardinality HashSet copy. Coefficient/scratch arrays and required oracle outputs remain owned where their lifetimes require it. |
| Micro SIMD | Existing Lane polynomials already execute in vectors. Scratch release probes of exact exp2/log2 at widths 4/8 emit packed xmm/ymm multiply/add, floor and division; no fused multiply-add appears. Scalar libm's branchy reductions remain its independent deterministic layer. No additional SIMD or speed claim. Assembly: `/tmp/issue1117-math-simd-probe.s`. |
| Data structures | Product coefficients/tables and bounded stack scratch already fit their fixed numerical tasks; no dynamic collection caps track counts. Test cardinality now sorts/counts its existing buffer without a second collection. No timing or benchmark result is claimed. |

Retained load-bearing families, including every existing test family:

- Scalar exact anchors/dB conversions, platform-libm accuracy and huge-trig/extreme-scaling cases catch wrong public mapping, coefficients, reductions and underflow/overflow handling; exp2's `i0_wrap_test` specifically catches the signed table-index wrap regression. Independent integer-root tests retain f64 raw/subnormal/midpoint cases and f32 exact/exhaustive qualification; floor's raw-pattern differential retains signed-zero/exponent handling.
- M1 anchors/clamps, sampled and exhaustive oracle/monotonicity sweeps and measured-worst neighborhoods defend lane accuracy and all arithmetic/reduction margins independently of vector agreement.
- M2 directed exp2/fast-gain edges, million-input digests/width comparisons, generator branch coverage, and both ignored all-pattern qualifications defend composed width identity. The moved exp2 exhaustive gate catches vector-only rounding/reduction disagreements at rare negative fractional, exponent/clamp or NaN/subnormal patterns that directed and random corpora cannot cover completely; log2 retains the equivalent full-pattern contract.
- M3 digests catch changed output words; its NaN check catches invalid determinism inputs; its distinct-result floor catches a corpus dominated by saturation. Buffer reuse/sorting preserves each assertion and does not alter the corpus or its pins.
- F1 identity/clamps, sampled/exhaustive current-polynomial bounds and exact-tier ratio retain actual fast-tier contracts, including adjacent-step limits. The consolidated definition test catches conversion/scaling errors through independent log10/pow algorithms over every previously distinct domain; renamed duplicate caller labels never inspected those caller implementations. Strengthened exhaustive input counts catch incomplete sweep coverage that the old one-billion minima admitted.
- F1 shaper domain, out-of-domain rails and composed gain/oracle error families retain the bounded-domain premise and composed-error behavior beyond bare conversion error. Rejected candidates and subset sweeps supplied no additional current-production claim.

Final actual checks (all exit 0): `cargo test --locked -p math --features lane` and its `--release` counterpart, each 26 passed/11 ignored; `cargo clippy --locked -p math --all-targets --all-features -- -D warnings`; `cargo fmt -p math --check`; `git diff --check`. Logs: `/tmp/issue1117-attempt2-math-{lane-debug,lane-release,clippy,fmt}.log`. The unchanged scalar-only layer and target-dependent paths were checked in attempt 1 above; attempt 2 only deletes test-local candidates/subset qualification and tightens counts. No 2^32/full-domain workload or timed benchmark was rerun. This host performed AArch64 compile checks only; runtime NEON qualification and the existing #1019 discrepancy remain outside this housekeeping issue. No owner decision remains pending. Final Sol verdict and root checkpoint/remote synchronization are pending.

### Root adversarial review — attempt 2

Root adversarial verdict: PASS. Reviewed both attempt diffs and confirmed that rejected refits call only their own coefficients, both removed shaper ranges are strict subsets with the same measurement/ceiling, all distinct definition domains survive, and the corpus writer overwrites every reused output slot. M3's in-place cardinality operation preserves its threshold and is separate from digest/NaN traversal. Product functions, operation order, parameters and all corpus pins are unchanged. Integrated lane/math/wasm-gates tests passed through G5/G6; the final F1 correction was verified by focused debug/release tests and clippy. Long exhaustive gates were compiled and retained, with the factored identity helper exercised in bounded scratch ranges rather than claiming a complete replay.

Test value: the moved exhaustive exp2 test rejects rare width-dependent composition/rounding errors outside the directed/random corpus; its corresponding log2 test retains all-pattern log2 parity. M2 width buffers still compare every previous input/width, and branch coverage still rejects vacuous generator populations. Each M3 test retains a separate defect: changed output bits, forbidden NaNs, or saturation-dominated coverage. The consolidated F1 definition test rejects conversion/scaling defects through a different log10/pow oracle over every distinct prior domain; retained exhaustive gain/level tests preserve complete input coverage and strengthen count admission. No old-implementation oracle or rejected candidate is a permanent test. No new product scenario, digest pin or timing claim is introduced, and no owner question remains.
