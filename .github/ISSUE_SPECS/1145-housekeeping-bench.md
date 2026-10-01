# Housekeeping: bench

## Authorized scope and smallest closable slice

Review the complete `tools/bench` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `tools/bench` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — complete package audit and bounded cleanup

Worker B (GPT-6.1 Sol xhigh) read all four Rust files (3,853 pre-change lines), the manifest and their test/support paths before implementation. Related current boundaries were checked in #1039, #1075, #881, #938, owner ruling R9 and the effect-floor accounting/exit ruling. The official native `console --step` and `effect-contract` workflows remain; retired Wasmtime/protocol subjects were not revived. Root approved the bounded proposal, committed its three product paths as `753fb5c7`, and integrated/pushed them through `5dc263ee`. Product edits stopped at that checkpoint.

Five-axis findings:

- **Repetition/LoC:** four measurement families now use fixed outer arrays and move their samples/digests into records. Existing arm order, warmups, timed loops, snapshots, record fields, floor formulas and validators are unchanged. The CLI passes its existing `OsString` iterator directly to `Command::args`. Net Rust change: **−17 lines** (21 added, 38 removed); no new helper, test or harness.
- **Test value:** all 13 test functions remain. The sole assertion deletion is the second identical positive jq floor comparison in `rust_and_jq_floor_tables_have_exact_key_value_parity`; the first positive and all missing-key, extra-key and wrong-value counterexamples survive. Those mutations affect only a child jq expression, so there was no restored state for the repeated positive to qualify. Complete retained-family purposes appear below.
- **Copies/ownership:** the four fixed-cardinality measurement families remove ten duration-vector clones, ten digest-string clones and twelve temporary outer vectors. Inner observation storage and owned record results remain. Prepared targets, independent arm state and frozen input restoration still earn their copies: a render must start from the same input rather than recursively process the previous output. CLI argument order and command ownership are preserved.
- **Micro SIMD/generated code:** the edits concern argument forwarding and untimed measurement-result ownership; no sample arithmetic or lane kernel changed. Build-only native AVX2/FMA assembly inspection confirmed an existing, unchanged `HoistArm::render` by-value target copy: a 672-byte `memcpy` per selected lane, in a function reserving 936 stack bytes. This is a source/code-generation finding, not a measured cost or speed claim. Root owns a separate bounded tooling/optimization successor; removing it from this timed subject was not layered into housekeeping. No new SIMD algorithm is justified for the changed paths.
- **Data structures:** fixed arm counts now use arrays; variable facility configurations and observation vectors remain appropriate. Ordered session rows, percentile inputs and independently owned mutable arms preserve record matching and deterministic comparisons. No new hash index or abstraction is needed. Four direct manifest edges (`builtins-compiler`, `graph`, `graph-compiler`, `sha2`) have no direct Rust consumer in this package; they remain unchanged because pruning requires the lockfile and the independent conformance-boundary policy union outside this slice. This is a non-runtime configuration follow-up, not a claim that the dependency graph was cleaned.

Retained floor test purposes (current numeric/record contracts, not historical waveform pins):

- `rust_and_jq_floor_tables_have_exact_key_value_parity`: independent Rust/jq tables agree on every key, value, basis, control and width; its three negative cases discriminate incomplete or wrong floor tables.
- `the_sixty_four_track_block_has_the_lane_sample_count_the_rulings_quote`: the actual track × frame × dual-mono count used as the floor divisor is correct for full and partial cohorts.
- `every_derived_row_names_its_ruling_and_composes_a_positive_floor`: every derived runtime row has provenance and a positive floor, with only the specified rows underived.
- `a_control_row_is_always_cheaper_than_the_row_it_isolates`: a declared control exists and leaves a positive isolated difference.
- `the_identity_inventory_is_the_floor_of_the_table_and_the_identity_pair_shares_it`: mandatory identity arithmetic bounds every row and the dispatch/gain-pan comparison uses compatible inventory and formulas.
- `the_driver_fed_gain_pan_row_is_costed_at_the_identity_inventory_and_isolates_nothing`: input feeding does not invent arithmetic or falsely identify an effect/control isolation.
- `the_mono_rows_carry_the_standing_strips_floor`: the three mono layouts retain the standing unhalved inventory pending an independently authorized recount.
- `the_current_effect_recount_keeps_fractional_link_work_and_composes_the_strip`: fractional stereo-link work, EQ depth and strip/control composition remain numerically consistent.

Retained console test purposes (the short internal timed renders qualify records and current behavior; their durations are not performance evidence):

- `the_meters_record_carries_each_arms_fold_and_redirect_counters`: actual prepared-plan fold/scatter facts are emitted once for the correct arm.
- `the_mixing_automation_row_prints_its_controls_and_the_validator_pins_them`: all eight controls resolve, each automated effect moves bits, EQ Both lowering retains collapse, restatement preserves PCM and warmed allocator/RT counters stay zero; malformed control/nonmoving/collapse records are refused.
- `the_driver_fed_gain_pan_row_prints_its_feed_and_the_validator_pins_it`: current driver and bound-feed arms produce equal PCM with zero RT violations, while missing, wrong or shortened feed records are refused.
- `the_metered_console_row_prints_its_meters_and_the_validator_pins_them`: current sample-peak/post-matrix windows drain uniformly without loss, preserve PCM and report their own fold/scatter facts; grafted, missing or wrong group/window records are refused.
- `the_console_strip_rows_print_their_facts_and_the_validator_pins_them`: every existing full/partial/app-bypass/sparse-input case reports its actual layout and input facts with zero RT violations; incorrect layout, track count, input, bypass or required-field records are refused.

There are no `effect_contract.rs` unit functions. Its existing CLI gate runs the production-factory conformance harness with the actual ModeCount allocation negative control; the warmed audit exercises extreme finite input and measures zero allocation/deallocation and forbidden-operation counters. Current same-run PCM digests and record fields remain validators of arm equality/nonvacuity, not newly pinned output history. No case, generator, floor, allocator control or workload was removed.

Actual qualification (all PASS; logs in `/tmp/engine-housekeeping-b-1145/`; builds used `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-b`):

- `cargo test --locked -p bench`: **13 passed**, `debug-tests.log`; `cargo test --locked --release -p bench`: **13 passed**, `release-tests.log`. Default features are empty; no duplicate feature test matrix was needed.
- `cargo clippy --locked -p bench --all-targets --all-features -- -D warnings`: `clippy.log`; `cargo fmt --all --check`: `fmt.log`; `git diff --check`: PASS.
- `cargo check --locked -p bench --target aarch64-apple-darwin`: `aarch64-darwin-check.log`. This native-only tool is checked on x86 and AArch64; #1039/#1075/R9 removed its Wasm CLI boundary. No unsupported Wasm/mobile CLI or target runtime claim is made.
- `cargo build --locked --release -p bench`: `release-build.log`. Build-only `cargo rustc --locked -p bench --release --bin bench -- --emit=asm`: `codegen-base-build.log`; the unchanged Hoist function excerpt is saved in `hoist-render.s.txt`. The filename does not denote a base/head equivalence test.
- Existing `check-bench-policy.sh`, `test-bench-policy.sh`, `test-console-benchmark.sh`: `bench-policy.log`, `bench-policy-mutations.log`, `console-validator-mutations.log`. The validator/runner mutation suite reports **real runner/workload/timing invocations 0/0/0**; browser runner checks used its existing untimed stub.
- Existing `check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-conformance-boundaries.sh`, `check-console-benchmark-fixture.sh`: `workspace-policy.log`, `realtime-policy.log`, `conformance-boundaries.log`, `console-fixture.log`.
- Fresh shipping-profile binary: `check-effect-contract.sh <release/bench>` passed eight production factories (`effect-contract-conformance.log`); `bench console --preflight` resolved all eight controls and current collapse/PCM claims (`console-preflight.log`); `bench effect-contract --audit 8 --trace-markers` measured zero allocations, deallocations and forbidden-operation counters (`effect-contract-audit.log`). This marker/counter check is not a new syscall trace proof.
- Unknown subject and unknown effect-contract mode both refused with exit **2**, `cli-refusals.log`. No descriptive CLI benchmark, additional timing experiment, browser matrix, Wasmtime run or benchmark percentage was launched/reported. Saved logs contain no compiler warnings/errors.

Completion evidence is ready for root's single attempt-1 adversarial verdict. No architecture owner question arose; the Hoist copy and unused dependency edges are explicit separate follow-ups. Required delivery qualification and GitHub synchronization remain root-owned.

## Root adversarial verdict: PASS — attempt 1

Root Sol inspected all three changed Rust diffs and the complete package/family record. Each fixed array uses the same declared arm order and cardinality as the old outer vector, with unchanged inner capacity, warmup/timed loop, digest update, snapshot and record assertions. Moving samples/digests preserves independent owned results and removes the ten copies of each kind. Command argument iteration preserves order and command-owned argument lifetimes. The retired second jq positive had no intervening persistent state; the first positive and every missing/extra/value counterexample remain.

Root read actual release/lint/native-target, policy/mutation, untimed preflight, production-factory conformance, warmed allocator audit and typed CLI-refusal logs. Short internally timed unit qualification is accurately distinguished from a descriptive benchmark, with no elapsed value or improvement claim. Current SIMD/DSP subjects are unchanged. The independently confirmed Hoist target materialization is now separately briefed in #1171 and is not silently included here; unused direct dependency edges remain a candid configuration follow-up.

Rewritten-test purpose: `rust_and_jq_floor_tables_have_exact_key_value_parity` catches a wrong, missing or extra independent floor-table key/value/control/width, preserving all original distinguishing cases and one unchanged positive. No new/replaced test function is introduced; all thirteen retained functions have specific numeric, record, allocator or mutation purposes.

All five requested axes are complete within this frozen slice. Native-only scope, absent mobile/Wasm runtime evidence, current output formats and algorithm/floor boundaries are preserved. No architecture owner question or measured speedup is claimed.
