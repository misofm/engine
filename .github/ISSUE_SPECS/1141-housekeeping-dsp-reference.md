# Housekeeping: dsp-reference

## Authorized scope and smallest closable slice

Review the complete `crates/dsp-reference` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/dsp-reference` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — worker B, 2026-10-01

Read all 19 source files, the integration test and manifest (21 package files), plus current related scopes #349/#559/#560, #234/#291, #763/#774, #988/#991/#992, #1019, #1069 and #1072. No dependency, generator, public API, numeric domain or DSP equation changed. Root approved the bounded proposal and checkpointed the eight Rust paths as `8efe5296`, integrated/pushed at `2ad35ddb`; subsequent work is evidence only.

**Five-axis findings.**

- **Repeated code/LoC:** one private iterator-based state-space loop now serves the existing slice filter and unit impulse. The equations, initialization, update order, output length/capacity and returned ownership are unchanged, including an empty impulse. Consolidated gate curve setup and removed the redundant RBJ smoke test, repeated identity/comparator assertions and unused PI silencer. Rust change: 46 additions, 106 deletions, net **−60 lines**. One test function is retired; another is saved by retaining both gate tests' complete cases in one function.
- **Test value:** retained independent mathematical, numeric boundary, generator and comparator owners below. The cast TPT adapter test now states its current matrix/tuple contract and checks both complex components at the nondegenerate cutoff across the same 40 rate/cutoff/output combinations. Its other three response probes add no distinct adapter claim; generic transfer correctness remains owned by independent SVF/RBJ and downstream response gates.
- **Copies:** removed the allocated impulse input (the owned output remains), LR4's collected 256-probe list and the spectrum test's collected 33 magnitudes. Lazy traversal preserves every value, order and peak tie rule. Owned planar outputs, gate traces, recursive delay/limiter rings and caller-visible model state remain necessary. SVF's sorted/deduplicated probes still require storage.
- **Micro SIMD:** inspected every recurrence, DFT, FIR, window minimum and sum. This is an offline oracle with no production kernel dependency. Its explicit scalar reductions and alternative state-space/RBJ realizations provide independence; vector reductions, FIR symmetry or production deque algorithms would change that purpose or arithmetic order. No SIMD/kernel change, generated-code speed claim or timing measurement is made.
- **Data structures:** contiguous fixed histories, checked planar storage and finite ring/output vectors fit the offline contracts. The brute-force limiter minimum/box sum deliberately differs from the streamed production implementation. Sorted design probes preserve their distinct design-frequency sample. Retained immutable-parameter delay/limiter state-law scaffolding and per-sample closed-form coefficients; no modeled DSP-law/retarget redesign or coefficient scheduling change was attempted.

**Deletion/consolidation survivors and rewritten test purposes.**

- Retired `parametric_eq::all_families_design_stably_and_have_finite_analytic_response`: `svf_transfer_matches_rbj_cookbook` designs all six RBJ families throughout the full grid, construction itself requires strict Jury stability, and every legal analytic response is compared with a different realization. The removed six-case finite-response smoke adds no independent rejection or response claim.
- Folded `gate_expander::hand_computed_curve_point_is_range_clamped` into `curve_is_identity_at_ratio_one_and_range_limited_when_closed`: all three original level/ratio/range/expected rows remain. This owner catches ratio-one attenuation or a wrong explicit range cap; the transition tests exercise different hold/hysteresis behavior.
- Removed only lib's impulse identity-copy assertion: `independent_identity_oracle_round_trips_noise` keeps complete two-channel nonzero data and independent ownership/shape coverage. The renamed lib DFT owner retains phase/sign, empty and 4097-frame refusal assertions.
- Removed only randomized's direct `same_word` assertions: `class_a::every_nan_folds_to_one_word_and_nothing_else_moves` and `a_nan_is_never_the_same_value_as_a_number` own payload/signed-zero boundaries; the renamed `first_difference_folds_nans_and_distinguishes_signed_zeros` retains both plane-level assertions and catches comparator/first-index wiring mistakes.
- Renamed the EQ identity test to the behavior it actually asserts: input passthrough including negative zero. No history-assertion claim is made.
- `cast_adapter_matches_low_and_high_pass_state_space_definitions` retains the explicit independent nine-field matrix definition and bit comparisons of both returned components. A wrong mix/sign, duplicated/swapped component or wrong forwarded frequency turns it red; downstream tests construct generic state spaces directly and do not exercise this public adapter.
- LR4 and spectrum iterator rewrites preserve their owning defects: a non-allpass/crossover law, and a wrong impulse magnitude or exact-bin sine peak. All four rates/four crossovers/256 probes and all 16 impulse bins/33 sine bins remain. No new test or corpus is introduced.

**Complete surviving behavioral families (29 unit + 3 integration tests).**

| Family | Current load-bearing purposes |
| --- | --- |
| lib DFT/valid frequency/floor (2) | Delayed-delta complex phase/sign; empty/oversized input refusal; accepted arbitrary-frequency evaluation; silence's finite floor. |
| integration reference API (3) | Complete asymmetric noise round-trip; signal/spectrum nonfinite/domain refusal; analytic impulse magnitudes and exact-bin sine peak. |
| class-A fold/word stream (3) | Positive/negative quiet/signalling NaNs share one class; finite/infinite/zero/subnormal words retain identity; NaN-versus-number and signed-zero distinction; little-endian stream folding. |
| randomized generators/comparator (4) | Seed separation/repeatability; legal bounded draws reaching both edges; plane first-difference/NaN/signed-zero semantics; short/maximum quantum bounds. Draw algorithms, seed namespace, hostile words, profiles and environment handling are unchanged. |
| linear ramp (2) | Once-computed step and exact final assignment, including settled updates; immediate zero-window target. |
| EQ identity (1) | Exact passthrough of negative zero and a subsequent nonzero input. |
| SVF/RBJ (3) | Independent transfer derivations on all 1,584 rows with conditioning/null/high-frequency gates; 4,096-sample recurrence versus distinct state-space realization per row; zero-gain bell/shelf identity. |
| cast TPT adapter (1) | Cast coefficient low/high-pass matrix definition plus correctly forwarded real/imaginary response. |
| LR4 (3) | Analytic allpass/half-amplitude crossing; four-section impulse against cascaded state spaces; invalid crossover/rate refusal. |
| gate/expander (6) | Ratio-one/range law; exact hold expiry; inclusive opening threshold; in-band hold reload; current-sample dry causality; nonfinite sample refusal. |
| true-peak limiter (4) | Actual first nonzero output at declared latency with guarded gain; minimum/rounded/full lookahead windows; short-record impulse estimator without artificial edge padding; bad-lookahead refusal. |

Files without local tests provide independently transcribed biquad, compressor, shaper, clipping and delay equations for downstream numerical owners; block/processor/signal/spectrum behavior is exercised through the package's public API families. These references keep separate topologies/equations and intentionally do not import production DSP or share its generators.

**Actual checks and limits.** All Cargo commands used `--locked` and `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-b`; logs are under `/tmp/engine-housekeeping-b-1141/`.

- Baseline `cargo test -p dsp-reference`: 31 unit + 3 integration PASS (`baseline-debug.log`). Checkpoint debug and final release package tests: 29 unit + 3 integration PASS each, 0 doctests/ignores (`checkpoint-debug.log`, `release.log`). This includes existing E2 and both LR4 numerical owners.
- Targeted downstream gates PASS, one test each: builtins `--features test-support --test response cast_tpt_state_space_matches_independent_rbj_transfer_at_compatibility_rates`; multiband-compressor `--test lr4_two_section_mapping_f64 two_section_bands_match_the_four_section_reference`; parametric-eq `--test analytic svf_words_match_the_independent_oracle_on_the_complete_grid` (`builtins-cast-response.log`, `multiband-lr4-reference.log`, `eq-rbj-grid.log`). No downstream caller invokes the changed private filter/impulse traversal; these additionally qualify the independent current reference roles.
- `cargo clippy -p dsp-reference --all-targets --all-features -- -D warnings` PASS (`clippy.log`); this package has only an empty default feature. `cargo fmt -p dsp-reference -- --check` and `git diff --check` PASS.
- Existing workspace, lane and realtime policy scripts PASS (`workspace-policy.log`, `lane-policy.log`, `realtime-policy.log`); the nonexecutable workspace/realtime scripts were invoked with `bash`.
- `cargo check -p dsp-reference --lib --target wasm32-unknown-unknown` with target-specific `-C target-feature=+simd128`, `aarch64-apple-ios`, and `aarch64-linux-android` PASS (`wasm-simd128.log`, `ios.log`, `android.log`). Native x86 tests used the repository's AVX2/FMA flags. Foreign checks establish compilation, not execution, mobile linking or cross-target numeric identity. No bare-Wasm replay, benchmark, listening or added matrix is claimed.

**Deferred finding:** root independently recorded #1167 (`1167-bound-offline-limiter-construction.md`, upstream `9a0644af`). The offline limiter accepts low/huge positive rates before incompatible window/unchecked capacity arithmetic. This is source-derived evidence only; no runtime reproducer, fix, accepted-domain extension or owner API question is claimed here. Existing related open scopes remain separate. Attempt 1 awaits root's single adversarial verdict.


## Root adversarial verdict: PASS — attempt 1

Root Sol read all eight changed Rust diffs, the complete five-axis/family record and actual gate logs. The private iterator helper performs precisely the former ordered state-space expressions with the same zero initialization, one impulse followed by positive zeros, output capacity/length and empty behavior. No production DSP code, oracle generator/grid or independent mathematical realization is shared or changed. The table consolidation keeps all three gate cases; the maximum search traverses the same33 bins with the same tie rule. The cast adapter retains its independently expressed low/high-pass matrix and a nondegenerate component check across all40 cases, so a wrong tuple mapping still fails. Supported foreign targets are compile-only evidence; #1167 is a separate source-level refusal finding.

Rewritten-test purposes:

- `curve_is_identity_at_ratio_one_and_range_limited_when_closed` catches ratio-one attenuation and an incorrect explicit range cap through all three original hand-derived cases.
- `direct_dft_oracles_are_bounded_and_correct` catches delayed-delta phase/sign or empty/oversized DFT refusal errors; the stronger two-channel noise owner covers identity copying.
- `identity_section_returns_input_including_negative_zero` catches the reference EQ identity path changing signed zero or a subsequent nonzero value.
- `first_difference_folds_nans_and_distinguishes_signed_zeros` catches plane-comparator/first-index wiring errors while shared class-A owners cover payload classification.
- `cast_adapter_matches_low_and_high_pass_state_space_definitions` catches an incorrect cast mix/matrix or duplicated/swapped/incorrectly forwarded complex response, a public adapter no other test exercises.
- `lr4_sum_is_allpass_and_crossing_is_half` catches crossover topology/half-amplitude errors through the unchanged four-rate/four-crossover256-probe grid after its temporary list is removed.
- `delayed_delta_and_exact_bin_sine_have_known_spectra` catches incorrect impulse magnitudes or an exact-bin peak through every original16/33-bin probe after its temporary magnitudes are removed.

One redundant RBJ smoke function was retired and one gate function consolidated; independent SVF/RBJ, E2/LR4 impulse, numeric boundary, generator and API owners remain. No new test, benchmark or sound-quality/engine timing claim was added. This bounded housekeeping slice is complete with no owner decision needed.
