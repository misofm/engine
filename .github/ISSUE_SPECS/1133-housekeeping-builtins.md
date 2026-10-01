# Housekeeping: builtins

## Authorized scope and smallest closable slice

Review the complete `crates/builtins` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/builtins` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — worker B, GPT-6.1 Sol xhigh

Read every package source/test file and `tests/MUTATIONS.md`; related open scopes #1072
(bank identity negative zero), #890 (ramping strip fusion), #969 (input symmetry grouping) and
#1054/#1055 (smoothing policy/research) remain separate. Product checkpoint `2f9b9ad1` is integrated
upstream as `5b611eea`. Root verdict: REQUEST CHANGES. The same-flags generated-code comparison
showed two additional W8 memcpy calls and scalar materialization from the word-pair factoring;
the W4 reduction cannot justify those copies without a measured product reason.

Five-axis findings:

- Repetition/LoC: current and target input records now share borrowed section readback; both retain
  current trim and the original mix-first/coefficient read order. The lattice adapter passes its
  already-derived fields directly, removing an otherwise unused `ParameterDescriptor`. The candidate
  six-word filter-pair helper retains c1/a2/a3/m0/m1/m2 and target-before-step order; its generated
  code has the tradeoff recorded below. Independent per-lane symmetry definitions stay explicit.
- Test value: remove two weak/subset tests and one identical-program comparison; replace three
  historical output/state digests with two current owning differential families. No new test file,
  fixture, corpus pin, or historical implementation is retained. Mapping and all families follow.
- Copies: borrowed readback avoids duplicating its body without copying the bank section arrays.
  Remove duplicate identity-bank buffers. Preparation ownership copies, mono integrator disengage,
  per-block ramp mirrors, target snapshots and atomic response preflight remain necessary.
- Micro SIMD: DSP loops already use generic lane kernels and separate scalar/4/8 widths. Native
  emitted sample paths contain XMM/YMM `vmulps`/`vaddps`, masks and stores; the existing `Lane::fma`
  deliberately uses two roundings under #163, so no fused instruction or arithmetic is introduced.
  Sequential f64 meter energy, held-peak state machines and recursive state retain their order.
  The factored symmetry path is assessed separately below; no speed claim or timed benchmark.
- Data structures: determinism now sorts/deduplicates its already-owned output vector by bits,
  preserving finite/nonempty and >64-distinct-word assertions while eliminating its `BTreeSet`.
  Only this test-owned output is reordered; corpus generation/order/pins are untouched. Production
  fixed arrays, eight-bit active/symmetry masks and bounded meter SPSC ownership earn their cost.

Deletion/rewrite survivors:

- `post_ramp_symmetry_mask_matches_lane_oracle`/helper are a strict subset of
  `post_ramp_symmetry_handles_differing_words_countdowns_and_padding`: it starts with the same fresh
  default scalar/full banks and adds partial banks, every word/countdown perturbation and signed zero.
- `polarity_trim_fader_and_matrix_are_exact`'s one-sample tolerance smoke is covered more strongly by
  `signed_zero_and_mute_laws`, independent scalar recurrence, and
  `full_public_chain_matches_the_three_section_reference` for gains, polarity, channels and order.
- The identity-elision test's alleged reference used `build(0)` on both sides with identical injected
  state. Delete that comparison/buffers; retain all real/disabled/padded construction, seeded -0/+0,
  invalidation and reset predicates. These direct laws, scalar recurrence and scalar/bank differential
  tests remain independent of that identical-program comparison.
- Retire `settled_matrix_shapes_render_the_base_bits` and `WidthDigests`. Nonidentity arithmetic has
  lane's select/general matrix differential; exact signed zero has
  `settled_identity_matrix_preserves_signed_zero`; full/partial/padded and mid-block dispatch have
  `settled_matrix_takes_the_select_free_arm_only_without_an_identity_lane`; D11 and partition gates
  remain. Its 200-sample retarget-to-identity and target/countdown snap now belong to the bank family.
- Rewrite the bank historical digest as `fused_fader_matrix_shapes_match_the_separate_stages`:
  identical hostile words and commands versus current separate fader then matrix banks, comparing
  every PCM and retained lane word including padding. Preserve 1/W-1/W members, both widths, mixed
  mutes/gains, instant identity/back, 200-sample pan/identity and 100-sample fader ramps. Each block's
  admission is checked against authoritative countdowns, replacing aggregate workload count pins.
  Unique defect: a fused bank changes an identity lane's negative zero or padded/retargeted PCM/state
  while dispatch counters and the existing finite-input separate-stage bank differential stay green.
- Rewrite the scalar historical digest as `scalar_fused_fader_matrix_matches_the_separate_stages`:
  current live-fader then matrix reference, same hostile patterns, identity/pan/mute and instant/
  200-sample transitions, complete retained words and direct settled admission. Unique defect: the
  live scalar fused call changes signed-zero/mute PCM or retained ramp words that dispatch witnesses
  cannot inspect. Its duplicated historical full-chain arm is owned by the retained current
  `full_public_chain_matches_the_three_section_reference` and
  `eligibility_sequence_uses_whole_call_fallback_then_fuses_the_next_call`.

All surviving test families earn coverage for these reachable defects:

| Family | Load-bearing purposes |
| --- | --- |
| Inline meter selection (2) | Every metric subset publishes selected fields; omitted energy/count/held/sqrt work; invalid bits refuse before queue allocation. |
| Inline input symmetry/control (7) | Independent word/countdown/signed-zero/padding mask; one extraction per word; helper absent while settled; asymmetric filter steps preserved; filter prefix stops at its own countdown in mono/dual; only addressed predicate refreshed; coefficient design occurs before runtime application. |
| Inline whole chain/dispatch (4) | Current three-section PCM/report/state reference; mid-ramp/retarget/reset whole-call eligibility; settled matrix tail dispatch; all four fused call sites respect identity/padding/ramp admission. |
| Corpus unit gates (2) | Independent identity/filter equations and positive controls; real identity/mixed dispatch plus sanitation at each width. Single-owner G5 corpus remains unchanged. |
| Contract (4) | Stable descriptor IDs/domains/mappings/default/update/reset; domain and launch-rate refusals; exact representable cutoff maxima/successors. |
| Determinism (1) | Each case finite, nonempty and >64 distinct bit patterns; cross-target pins have their separate G5 owner. |
| Identity/RT (1) | Independent x+0 PCM law, signed zero, audit-clean render and positive allocation/free control at 48/96 kHz. |
| Fader ramps (8) | Prepared gain/mute law; zero-window addressed assignment; D11 monotone/exact endpoint; mute/unmute positive zero; one-channel mute; remembered muted gain; finite live domains; cross-block partition. |
| Filter liveness (9) | Pair/rate/order validation; target visible before applied endpoint; scalar disabled trim finite; symmetric mono/dual; exact prepared identity words; cutoff successor; lane-local target/padding refusal; endpoint/partition/reset; in-flight infinite tail. |
| Filter response (11) | Independent section/cascade response at every rate; optional buffers/excluded fields; settled PCM impulse; malformed shape/grid/budget atomic refusal; valid/refused zero allocation; configuration/rate errors and maxima; seeded state/ramp untouched; common adapter parity, resource/ownership/refusal and trait-object allocation. |
| Live input (13) | Prepared current/target words; settled ramp untouched; bank elision; independent smoother D11; partition; each elision/polarity shape; positive-zero integrators; polarity zero crossing and trim independence; selectors/domains/member bounds; scalar/bank live PCM. |
| Live input mono (8) | Asymmetric/symmetric admission; never-collapsed PCM; complete ramp-record agreement and mirror; re-equalization witness; drain-before-disengage retarget; integrator copy positive control. |
| Matrix (9) | Independent D11 lengths/retarget/drift; identity signed zero; zero window/reset; two-update rendered gain; window adoption/domain; current bank and scalar fused families above. |
| Meter (12) | Exact windows; discontinuity/reset/drop and lane mismatch; bounded hostile random configurations; segment law; positive-zero peak; silence advancement and nonzero held decay; resident/planar metric/stride parity and atomic shape/time errors; block-peak merge/fallback; banked seeded energy/moved-left/right/withheld guards; first-binding seed ownership. |
| Mono collapse (2) | Dual-equivalent recovery/sanitation/lifetime reports and never-collapsed PCM after disengage. |
| Randomized (1 active, 1 ignored) | Hostile scalar/bank/mono/chunk/current-state differentials, partial/full lanes and live/reset interleavings; original narrowly scoped #1072 allowance and ignored strict reproducer preserved. |
| Stage (11) | Independent design/tan/cast recurrence; every width/member padding; lane-local fault recovery; partitions and live events; signed-zero/mute/sanitation; bank admission; exact elision/invalidation/reset; scripted bank/per-track fader-matrix differential; settled bank positive-zero mute. |

Actual checks, rustc 1.97.1, target directory `target/housekeeping-b`:

- `cargo fmt -p builtins`, subsequent `--check`, and `git diff --check`: PASS.
- `cargo test --locked -p builtins --all-features --lib --test matrix --test stage --test determinism`:
  compiling/focused checkpoint PASS (15+9+11+1 tests).
- Full `cargo test --locked -p builtins`, `--all-features`, and `--all-features --release`: PASS,
  each 116 active, 1 originally ignored #1072 reproducer; default and instrumented gates both exercised.
- `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`: exit 0; only the
  pre-existing unreachable `math::fast_db` paths in `clippy.toml` warn, duplicated by Cargo targets.
- Existing builtins, realtime (54 regions/15 files), and workspace policy scripts: PASS.
- Locked production checks: Wasm with `RUSTFLAGS='-C target-feature=+simd128'`,
  `aarch64-apple-ios`, and `aarch64-linux-android`: PASS. These are compile checks; target execution
  and required browser/ARM qualification remain delivery CI responsibilities, without bare-Wasm replay.
- Downstream: builtins-compiler `--features test-support --test allocation_tracker allocate` PASS
  (5 actual queued/composite zero-allocation/free gates); its current composite live differential
  PASS (1); effect-compiler `--test parameter_lattice` PASS (9); host-core
  `--test input_liveness_live_controls` PASS (11).

Generated-code finding: `cargo rustc --locked --release -p builtins --lib -- --emit=asm`, plus one
same-command base/head comparison with only the four base production source files copied to `/tmp`
(identical optimization/ISA/dependencies). W4 filter symmetry frame changes 1192→760 bytes and
memcpy calls 1→0; W8 frame 3488→2560 but calls 4→6; scalar gains a 16-byte frame and more emitted
instructions. The helper is inlined, yet the two-array chain adds W8 copies and scalar materialization.
This is a concrete reason to reject that factoring, not a measured runtime claim. Root was notified;
no product edits followed checkpoint without a verdict.

Checkpoint LoC: Rust +173/-504, net -331; mutation-history clarifications +13, total net -318.
No dependency, fixture, numeric tolerance, API or DSP algorithm change. No benchmark/listening claim.
No new owner question; #1072/#890 and smoothing decisions retain their existing issues.

### Attempt 2 — authorized narrow revision

Restore only the explicit post-ramp/filter-symmetry word-pair lists and remove their helper. This
keeps every original field/comparison/early-exit order and preserves the approved readback, lattice,
determinism and test reductions. The one-time comparison above is evidence of why the abstraction
was rejected; no permanent historical code or code-generation pin is added. Prior full package,
supported-target and downstream results remain applicable; revision gates/results follow below.

- One-time source comparison confirms both complete restored symmetry bodies equal the pre-attempt
  source, with the rejected helper absent; there is no new arithmetic or extraction path to qualify.
- `cargo fmt -p builtins --check`, `git diff --check`, and locked all-feature library tests: PASS
  (15, including independent masks, extraction counts, settled exclusions, asymmetric filter steps,
  dual/mono prefix bounds and actual dispatch witnesses).
- Strict all-target/all-feature package Clippy: exit 0, same pre-existing `clippy.toml` path warnings.
- Final package delta versus the pre-attempt base: Rust +148/-428, net -280; history notes +13,
  total net -267. No additional package, target or timed workload was run for this exact restoration.
  Root final verdict and remote issue closure are pending.

### Root adversarial review — attempt 2

Root adversarial verdict: PASS for the frozen housekeeping scope. Root reviewed the complete implementation/test diff and narrow revision: symmetry bodies exactly restore the pre-attempt operation/extraction order, avoiding the newly demonstrated W8/scalar materialization; borrowed current/target readback and direct lattice fields preserve their order and values. The failed factoring remains candid evidence, with no timing claim. All existing package/target/downstream gates plus focused revision tests and strict Clippy support this bounded result.

Test value: the rewritten bank fused-matrix family detects hostile signed-zero or padded/retargeted PCM/state changes that finite-input separate-stage comparisons and dispatch counters cannot inspect, including the preserved 200-sample identity endpoint. The scalar family detects the same faults at the live scalar fused call site, comparing every retained ramp word and direct eligibility. Determinism still rejects nonfinite/empty/insufficiently diverse case output while replacing its set with owned-vector sorting. Identity-elision still rejects wrong negative-zero, padded-lane, seeded-state or reset predicates; its deleted comparison ran the same construction twice. Removed subset symmetry and one-sample stage smoke have the named stronger current owners. No new fixture pin, historical implementation, DSP expression or feature change.
