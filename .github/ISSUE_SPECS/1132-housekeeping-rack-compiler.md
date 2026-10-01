# Housekeeping: rack-compiler

## Authorized scope and smallest closable slice

Review the complete `crates/rack-compiler` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/rack-compiler` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 — checkpoint

Worker B reviewed the complete package (`Cargo.toml` and its sole source/test file). Root approved the package brief. Related partial-insert banking (#888/#889), partial gather/scatter (#887), and live silent-lane packing (#1107) remain separate scopes.

- **Repetition/LoC:** delete `WorkingGroup` and its staging vector; materialize each final group directly in the same level/rack/class/cohort/chunk traversal. Delete the pre-#96 `legacy_full_banks` implementation oracle and historical mixed-cohort comparison branch. All 13 current behavioral tests remain.
- **Copies:** borrow the selected leader until each output clones its required owned program; borrowed IDs replace debug-validator ID clones and its per-group temporary vector. Test member assertions borrow slices; seeded candidate programs use dense-ID borrowed references, homogeneity compares iterators, and ascending/scalar checks use adjacent windows.
- **Test value:** retain current level/key partition, complete candidate conservation, padding/mask, duplicate rejection, greedy repeated-key subsequence, longest/full-first placement, ascending-bank regression, class/witness, exhaustive-pooling, homogeneity, shuffle determinism and comparator gates. Full family purposes and deletion survivor mapping follow below.
- **Micro SIMD:** this control-plane package compares opaque program keys, sorts IDs and builds masks; it performs no sample arithmetic. No additional ISA implementation or arithmetic change is justified.
- **Data structures:** eliminate the intermediate group vector and the test-only ordered maps for dense generated IDs. The production level `BTreeMap` still supplies ascending level order; output programs/IDs/masks remain owned, and public APIs, leader selection, chunk ordering and width handling are unchanged.

Focused gate: `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-b cargo test --locked -p rack-compiler` — PASS (13 unit tests, zero doc-tests). Product edits paused for root exact-path checkpoint. Release/clippy/fmt/policies, supported target builds and proportional downstream evidence pending; no timing or rendered-bit claim is made.

### Attempt 1 — completed local evidence

Root checkpointed the exact two owned paths as `401e3738`, integrated/pushed through `73b67238`. Source changes are +55/−129 Rust lines (net −74); dependencies and public APIs are unchanged. No additional product tranche followed the checkpoint.

**Retained test purposes (all 13):**

- `single_slot_programs_partition_by_level_and_key`: seeded mixed-level/key pools at both available widths conserve every ID exactly once, retain trailing padding and mask agreement, and never place a candidate in another dependency level or key. The other mixed-program structural test does not independently compare candidate and group levels.
- `empty_programs_and_connected_sidechains_never_bank`: empty/connected programs stay scalar while an unconnected sidechain can fill a bank.
- `subsequence_uses_program_equality_not_occurrence`: repeated keys match greedily by equality; insufficient repeats, longer programs and another rack refuse. This is the directed subsequence gate; `rack` has no duplicate test.
- `longest_program_leads_and_full_programs_fill_first`: short programs with smaller IDs cannot displace full-program members; exact chunk membership, identity slots and padded masks preserve the two-stage ordering contract.
- `a_subsequence_program_still_emits_ascending_banks`: issue #206's 64-member legal-session repro remains strictly ascending and complete at both widths without moving its short-program member into the first bank.
- Pool-class partition and prepare-witness tests: classes cannot mix lanes or reduce uniform-session bank occupancy; only SOURCE/DESIGNED classify at preparation, while live/bypass/restore terms cannot repartition cohorts.
- `pooling_is_exhaustive_so_no_member_is_stranded`: seeded repeated-key programs cannot leave an earlier free lane that a later member could legally occupy.
- `every_slot_cohort_is_homogeneous`: an independent reading of active leader keys reproduces each candidate's complete ordered key sequence; a padding lane cannot execute a slot.
- `output_is_input_order_invariant`: seeded permutations leave the complete plan identical.
- `duplicate_ids_are_rejected_across_levels`: duplicate IDs refuse within and across levels before emitting a plan.
- `invariants_hold_on_seeded_corpus`: mixed empty/nonempty programs across levels/racks preserve dimensions, ascending/padding layout, unique IDs, candidate rack and exact full subsequence masks; scalar IDs remain sorted. The explicit invariant call and slice equality execute in release too.
- `program_comparison_is_a_total_order`: the public comparator honors equality, key order and rack precedence for callers reproducing planner order.

**Deletion/survivor mapping:** the old `legacy_full_banks` body and pre-#96 comparison branch are removed; the same single-slot test retains current conservation, level/key partition and padding claims. Directed longest/full-first placement and ascending-bank repro, exhaustive pooling, and permutation invariance retain current chunk/ordering claims. No entire behavioral test is removed. Replaced sorted-copy assertions retain the same adjacent-order predicates; dense borrowed test references retain the original seeds, generated bits/order and full ordered-key comparisons. No historical output pin or new test is added.

Root identified that the homogeneity iterator's `zip` would truncate an overlong activity mask. The surviving release gate is `invariants_hold_on_seeded_corpus`: for every active lane it compares the **entire** activity slice with `subsequence_mask`'s leader-sized result, rejecting an extra flag independently of the production debug assertion. Its explicit invariant assertion additionally rejects overlong padding masks in release; the directed full-first test asserts exact active and padded three-flag masks. This limitation of the individual homogeneity assertion is recorded rather than presented as standalone dimension coverage.

**Generated code/data structures:** inspected the native optimized `plan_bank_groups::<u32, EffectProgramKey>` and member-order comparator monomorphization from the release unit-test binary (`objdump`; temporary `/tmp/issue1132-planner.asm` and `/tmp/issue1132-member-order.asm`). LLVM already emits AVX2 integer `vpmovzxbq`/`vpaddq` reductions for sufficiently long boolean masks and scalar byte-add tails; no hand-written SIMD is justified for opaque key comparisons and control-plane sorting. Materialization staging and cloned debug/test data are removed, while the ascending-level `BTreeMap`, total leader scan and stable member sorts retain their deterministic roles. Repeated comparator mask scans and cohort searches remain; no missed budget or measurement justifies a new algorithm. No timing claim is made.

**Actual checks:** all Cargo commands used `CARGO_TARGET_DIR=/home/bl/misofm/engine/target/housekeeping-b`.

- `cargo test --locked -p rack-compiler` and the same command with `--release`: PASS, 13 tests each plus zero doc-tests.
- `cargo clippy --locked -p rack-compiler --all-targets --all-features -- -D warnings`: exit 0. Existing workspace `clippy.toml` unresolved fast-dB path warnings were emitted by dependencies/package; no new Rust lint failure.
- `cargo test --locked -p graph-compiler --test compile_shapes --test bank_levels --test bypass_cohorts`: PASS, 3 + 9 + 6 tests. These cover mixed depth/sidechain preparation, bank alignment and randomized scalar/SIMD render comparisons, and bypass/nonfinite/negative-zero behavior.
- `cargo fmt --all --check`, `git diff --check`; `bash scripts/check-rack-policy.sh`, `check-graph-determinism.sh` (100 fresh-process comparisons), `check-lane-policy.sh`, `check-realtime-policy.sh`, `check-workspace-policy.sh`: PASS.
- `cargo check --locked -p rack-compiler --lib --all-features --target wasm32-unknown-unknown` with `RUSTFLAGS='-C target-feature=+simd128'`, and the same lib check for `aarch64-apple-ios` and `aarch64-linux-android`: PASS. These are supported production compile checks; no browser/mobile execution or device ABI qualification is claimed.

No benchmark, new owner question, DSP algorithm, resource-accounting policy, live queue/owner, latency/PDC or feature change. Related insert identity-slot and live packing outcomes remain deferred to their existing issues. Root adversarial verdict and remote issue closure pending.

### Root adversarial verdict: PASS — attempt 1, 2026-10-01

Root reviewed the full source diff and the leader's borrow lifetime, cohort/chunk traversal, ownership, deterministic ordering and release mask survivor. Each output retains its required single owned program clone; the staging vector and cohort-level clone are unnecessary. Materialization changes allocation placement on the control plane, with no render arithmetic or realtime contract change. The rewritten iterator checks retain their current purposes and the release exact-slice gate covers the recorded zip limitation. The package and downstream/target evidence is proportional; unresolved existing clippy configuration warnings and absence of measured speedup are disclosed.

Unique-defect verdicts for rewritten tests: `single_slot_programs_partition_by_level_and_key` reaches mixed dependency-level/key cases and rejects assigning a candidate to the wrong level, which the general structural corpus does not compare; `longest_program_leads_and_full_programs_fill_first` rejects sacrificing full-program occupancy to smaller short-program IDs; `a_subsequence_program_still_emits_ascending_banks` preserves the 64-member ordering regression; `pooling_is_exhaustive_so_no_member_is_stranded` reaches repeated-key pools and rejects leaving a compatible later member behind a free lane; `every_slot_cohort_is_homogeneous` independently reconstructs each ordered active-key sequence across its seeded generator; `invariants_hold_on_seeded_corpus` reaches mixed empty programs, racks and levels and rejects malformed full masks and scalar order. The latter two randomized tests are judged by these generators' reachable properties, rather than claims of unique sampled catches. No new test, old implementation oracle or arbitrary pin is introduced.
