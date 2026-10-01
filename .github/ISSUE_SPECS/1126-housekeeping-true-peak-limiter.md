# Housekeeping: true-peak-limiter

## Authorized scope and smallest closable slice

Review the complete `crates/true-peak-limiter` Cargo package and its tests. Deliver practical, behavior-preserving cleanup in this package, or an evidenced no-change verdict when existing choices earn their cost. Cover all five user requests: repeated code/LoC; load-bearing test purposes; avoidable copies; applicable micro SIMD; and data structures that cap efficiency. The user requests two GPT-6.1 Sol xhigh agents concurrently; this overrides the default Terra implementation model for this workflow. Root Sol owns briefing, adversarial review, exact-path checkpoints, integration and remote synchronization.

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
- Stop after a coherent compiling, focused-green tranche; root commits exact owned paths before further edits. Keep edits within `crates/true-peak-limiter` and this numbered spec. Independent crates use separate worktrees. Root pushes promptly and updates this GitHub issue with evidence; close only after PASS and evidence upstream, verifying remote state. Merge through the required qualification check and remove clean completed worktrees.

## Sol brief approval and decision record

Approved 2026-10-01 by root Sol within the user's crate-by-crate housekeeping request. Attempt 1 is authorized with the frozen scope/gates above. Owner questions are collected for the final report; routine reversible cleanup proceeds autonomously. Scope is this package's housekeeping, not completion of pre-existing feature/bug issues.

## Attempt evidence

### Attempt 1 scope decision

Worker A read the full package and related open #988/#989/#991/#992/#1102/#559 effect scopes; those remain separate. Root approves replacing the verbatim pre-#990 block oracle with a current per-frame driver using ordered detector/link/ramp/channel primitives, preserving all current LinkedPair generators, payload/report/state comparisons and engagement censuses. Independent f64 DSP law and public scalar/bank/padding/mono/automation gates remain. Retire linked/segments historical digest-only scenarios only with their retarget/reset/body cases mapped to stronger current owners; seedless keeps every delayed lane-local fault, line-fill, signed-zero/subnormal/limiting witness, schedule and body while removing SHA/pin work. G5 stays the sole cross-target digest owner. Reuse the existing warmed thread-scoped audited allocator, preserving zero scopes and positive allocate/free control; no new allocator test. Borrow small test PCM/offsets and replace a corpus membership tree with sort/dedup where order is immaterial. Correct stale current-layout/persistence/FMA/mono-ramp documentation without altering product arithmetic or APIs.

Scope amendment: remove the now-unused sha2 dev dependency and only the true-peak-limiter package dependency row in Cargo.lock; no other dependency/lock changes. No new generator, timing, harness, kernel consolidation or algorithm. Stop focused-green for root's exact checkpoint before more product edits. Baseline log-derived tests: 62 active PASS, one existing ignore. Final five-axis and test-purpose evidence follows the checkpoint.

### Attempt 1 implementation and audit — worker A, GPT-6.1 Sol xhigh

Read every original package file: manifest, both source files, all eleven Rust integration files and MUTATIONS.md. Product checkpoint `a6da0cad` integrated/upstream as `73e0701a`; product edits paused. Script-derived checkpoint metrics: Rust 181 added / 1,215 removed = **−1,034 LoC**; complete package 188 added / 1,216 removed = **−1,028**. Two active tests retired; no ignore removed and no test added.

| User axis | Finding and bounded disposition |
|---|---|
| Repetition/LoC | Retired the copied pre-#990 uniform block body and three historical SHA folding/pin paths. Current `reference_block` walks frames independently of block bodies, detector chunks, uniform gathers, segment walks and mirrored linked writes. Ordered detector/link/ramp/channel primitives and lane hot state remain the current law. Production dual/mono and steady/completion specializations stay; further consolidation belongs to #559 FX5 and would need generated-code evidence. |
| Test value | Retained all 46 lib tests and 14 active integration tests with purposes below. Linked/segments historical hashes had stronger current owners; seedless keeps its explicit timing/fault/domain witnesses. Removed only the partition test's repeated 512-frame arm: the reference already renders precisely that partition. No generator, scenario count, schedule, engagement or completion/wrap census changed. |
| Copies | Cohort input PCM becomes scalar-owned PCM after the bank is packed; restore continuation consumes its last input copies, and immutable corruption-test right payload is borrowed. Mono idle offsets are borrowed. Padding extraction/zero checks use strided iterators; two channel references use a stack array. Required independent mutable render buffers, snapshots, atomic two-channel restore parsing and full disengage state copy stay. |
| SIMD | Existing detector, reduction, uniform minimum/box and boundary scans already use `Lane`; scalar, W4 and W8 arithmetic/order remain verbatim. Mixed-window addressing retains per-lane fallback. No additional SIMD lowering or speedup is claimed. #988/#989/#991/#992, #1102 phase recovery and #559 FX4/FX5 stay separate. |
| Data structures | Corpus distinct-word counting sorts/deduplicates the already-owned words rather than allocating a BTreeSet; order is unused after finiteness checking. Fixed AoSoA ring capacities, chunk scratch and bounded windows remain. Replaced the custom unsafe allocator/TLS wrapper with the existing installed audited allocator and warmed current-thread deltas; all measured scopes and both positive allocate/free assertions stay, and realloc is still an allocation event that fails zero. |

Corrected current-layout-1, in-memory payload, unfused `Lane::fma`, mono ramp/body and recovery-owner descriptions. G5 inputs, pins, numerical expressions, public API, features, state codec, latency/tail and preparation behavior did not change. Existing #1018 Apple lowering and #1073 public recovery-count defects remain tracked; no new owner decision or safety/product change was introduced.

### Retained load-bearing families

Lib family counts total **46**. Names identify current owners; paired/width cases are homogeneous families.

| Current lib owners | Count | Plausible defect defended |
|---|---:|---|
| `descriptor_metadata_and_exact_resource_rows_are_frozen`; `bank_binding_validates_before_fallback_and_retains_exact_width_bytes`; `a_padded_request_binds_after_every_lane_is_validated`; `a_padded_lane_has_no_state_payload`; `automation_routed_to_a_padded_lane_is_not_charged`; `a_padded_bank_of_uniform_members_takes_the_uniform_body` | 6 | Wrong contract metadata/layout, malformed requests hidden by fallback, unvalidated first/late/padded lane, addressing a nonexistent member, charged padding or wrongly seeded padding moving body selection. |
| `phase_outputs_match_the_frozen_scalar_order`; `bs1770_annex2_conformance_is_unchanged`; `seedless_peaks_match_the_seeded_order_at_every_width`; `detector_chunk_matches_the_current_stream_{scalar,w4,w8}` | 6 | FIR table/order or tap error, signed-zero/NaN peak divergence, offset/full/short-tail slice error, incomplete twelve-tap history or writes beyond the active prefix. Seeded dot products remain an explicit mathematical order, not an old block body. |
| `fixed_latency_guarded_ceiling_and_bypass_bits_hold`; `the_gain_ramp_falls_gradually_and_arrives_at_the_requirement`; `silence_restores_exact_identity_including_signed_zero` | 3 | Wrong N+6 latency/bypass zero bits, abrupt or late attack ramp, release word failing to flush, or reciprocal averaging losing exact identity at Wb=97. |
| `the_stationary_hoist_reads_what_advancing_would_have_produced`; `an_open_window_is_never_stationary`; `the_lane_ramp_reproduces_the_scalar_ramp_bit_for_bit` | 3 | Hoisting an open one-ULP move, changing rest state, or wrong lane ramp update/snap/scatter. |
| `lane_identity_holds_across_widths`; `a_uniform_cohort_renders_exactly_the_per_lane_path`; `a_mixed_lookahead_cohort_falls_back_bit_identically`; `the_two_channels_of_a_uniform_cohort_keep_their_own_phases`; `a_restore_that_desyncs_the_phase_falls_back` | 5 | Width-specific PCM/state writes, absent vector suffix pass, shape-only/phase-only admission, crossed channel phase writeback or a single restored lane corrupting its peers. |
| `the_segment_walk_visits_the_slots_a_frame_at_a_time_walk_visits` | 1 | Wrong independent-modulo slot sequence or empty/out-of-range segment at window/ring/rate boundaries. |
| `stationary_dispatch_matches_runtime_oracle_and_observes_selected_body` | 1 | Wrong stationary/ramping specialization on populated PCM/state across scalar and all native dual/mono uniform/per-lane routes, or accessing the stale mono right plane. |
| `a_settled_silent_limiter_renders_exactly_the_never_fast_path`; `a_limiter_still_releasing_through_the_silence_is_never_frozen`; `a_negative_zero_input_block_is_not_treated_as_silence`; both `automation_withdraws_the_claim*`; `a_stale_detector_history_refuses_the_claim`; `a_de_zipper_window_open_across_a_block_boundary_refuses_the_claim`; `a_restore_withdraws_the_silence_claim` | 8 | Incorrect silence admission/phase/cursor advance, frozen release/history/open short-quantum ramp, lost negative zero, skipped automated block or dropped restored delay-line contents. PCM and complete state compare against forced processing with engagement non-vacuity. |
| `partition_invariance_holds_over_block_sizes` | 1 | Block-boundary dependence at real partitions 1/7/64/128 versus 512, including payload state. |
| `a_nonfinite_block_is_zeroed_reset_and_counted`; `a_failed_lane_is_recovered_and_reported_alone`; `a_padded_lane_stays_at_rest_through_the_lookahead`; `a_lane_reset_is_the_whole_reset_at_one_lanes_stride` | 4 | Missed D7 recovery, whole-bank collateral reset, charging failed padding, moved surviving cursors, padded nonzero/nonfinite state or incomplete/crossed lane reset writes. |
| `state_round_trips_and_rejects_corruption`; `both_resets_return_the_runtime_state_to_a_silent_lane` | 2 | Incorrect codec/continuation, accepted corrupt cursor/phase/grid/ramp/version/length or partial peer mutation; full/keep-parameter reset loses its distinct semantics. |
| `automation_retargets_linear_coefficients_and_counts_invalid_spans` | 1 | Wrong linear coefficient target/update count, accepting Both/preparation-only lookahead spans, or modifying the other channel. |
| `the_linked_body_matches_the_current_frame_law`; `randomized_scenarios_match_the_current_frame_law`; `the_stationary_walk_matches_the_current_frame_law`; `a_collapsed_block_unlinks_the_pair_without_desymmetrize`; `the_linked_body_engages_exactly_where_the_record_allows` | 5 | Mirrored gain ring/state write loss, wrong two-pass target/frame position, bad linked record transitions or stationary stream wrap/completion boundaries—even when two optimized width/partition arms would share the error. Every block retains PCM, reports, payloads, complete state and internal recovery/claim comparisons. Existing hostile-domain/event generators and nonzero coverage censuses stay. |

| Current integration family | Active | Purpose |
|---|---:|---|
| allocation | 2 | Own-thread allocate/free positive control and exact zero during all original scalar/full-bank/mono scopes (100 blocks each) and padded dual/mono scopes (99 measured blocks), including automation, failures and original reset calls. |
| conformance | 1 | Independent public contract validation, fault/state and realtime audit gates against the actual factory. |
| determinism | 1 | Finite, non-vacuous delegated corpus; G5 remains sole digest owner. |
| gain_law | 2 | Independent f64 gain law and output true-peak ceiling across the original 288 configurations, plus direct numerical law comparison. |
| mono_collapse | 2 | Direct public collapsed/dual equality with ramping and stationary frames; complete disengage copy before/after resumed dual output, with populated history/rings and settled retargeted coefficients. |
| observation | 2 | Resident reduction equals snapshot state, and repeated reads between blocks do not perturb output. |
| padding | 3 | Every active count/public width under both links, native scalar twins, collapse, automation and silence; members independent of padding request contents; nonfinite input fails alone and recovers to its per-node continuation. |
| seedless | 1 | Original 96-block noise/signed-zero/subnormal schedule, both links and collapsed banks plus eight scalar tracks. Explicit limiting/domain non-vacuity and exact delayed failed-lane/line-fill zero blocks catch common timing errors that bank/scalar parity alone could share. |

`randomized.rs` retains the one ignored, real #1073 public recovery-report reproducer. MUTATIONS.md preserves historical observations with an explicit current-owner notice; no mutation run or new harness was added.

### Deleted/recast coverage map

- `linked::the_linked_scenario_renders_the_pinned_base_words`: blocks 30 (equal ceiling retarget), 60 (one-sided release) and 90 (full reset) survive exactly in `linked_engagement_witness`'s gate-3 arm. The full linked matrix and randomized current-frame-law differential preserve output/report/payload/state behavior, bypass/both links and transitions; public bank automation/padding/mono and observation families preserve boundary calls.
- `segments::the_segments_scenario_renders_the_pinned_base_words`: its symmetric/asymmetric Maximum/DualMono 5 ms /9.9 ms windows, checked-vs-stream runs and retarget/reset cases survive the stationary scenario's original seven window boundaries/four channel-link patterns, point retargets/resets and full per-block differentials. Nonzero census gates retain completion-first/last/chunk/wrap, right cuts, 0/1/2/3/31 steady runs, newest/expiring/no-wrap streams, checked/refused streams, ramping fused blocks and phase-0/last restores. Independent slot-modulo and crossed-channel-phase owners remain.
- Seedless SHA/payload/report folding and historical pins were removed; every explicit witness, generator word, schedule, body and scalar-track case stays. Full differential/codec/observation owners above retain report/state assertions.
- The detector active-window helper was renamed for its current stream law, with its implementation, sentinel tail and shifted-window positive control unchanged. No coverage was removed.

### Actual gates and limits

- Baseline locked all-features package: **62 PASS /1 ignored**. Current focused frame-law gates: **3 PASS**. Current full locked debug and release all-features package: **60 PASS /1 ignored** each, including all W1/W4/W8 driver/phase/ramp cases and unchanged release scenario shares (1,000 per width). No warnings.
- Strict `cargo clippy --locked -p true-peak-limiter --all-targets --all-features -- -D warnings`, package fmt check, diff check, realtime policy (54 marked regions/15 files), lane policy and workspace policy: **PASS**.
- Sole-owner `cargo test --locked -p wasm-gates --test g5_native_corpus --release g5_native_digests_match_pins -- --exact`: **1 PASS**, all native delegated-corpus pins unchanged. No warnings.
- Raw logs: `/tmp/housekeeping-a-true-peak-limiter-baseline.log` and `/tmp/housekeeping-a-limiter-{focused,debug,release,clippy,g5}.log`. Builds used target `housekeeping-a`; no benchmark, new sweep, bare-Wasm replay or fixture pin ran.
- Root accepts no repeated supported-target/codegen matrix: executable product expressions and G5 inputs/pins are unchanged; native tests instantiate W1/W4/W8 and existing qualification owns supported Wasm simd128/iOS/Android builds. No new target/RT-performance claim is made.

All authorized gates are complete. Worker pauses for root's one adversarial attempt-1 verdict; no further product edits or optional probes.

## Root adversarial verdict: PASS — attempt 1

Root reviewed all manifest/lock/source/test/doc hunks, the new per-frame driver, allocator scopes and historical-scenario survivors. Executable product arithmetic, state layout and G5 words are unchanged; the lock removal is confined to the approved limiter sha2 row. The driver independently addresses per-lane state/cursors and advances both channel ramps without optimized block/chunk/segment/gather/mirror machinery. Independent f64 numerical and current public contract/padding/mono owners remain. Exactly two obsolete hash-only tests and the partition test's already-rendered reference arm retire; the real ignored #1073 fault reproducer stays.

Rewritten-test purposes: the linked/current-frame differential catches omitted mirrored ring/state writes that two linked optimized widths can share; its randomized generator reaches ragged/asymmetric rates/windows, hostile words, reset/restore/automation and collapse transitions beyond the fixed matrix; the stationary generator catches wrap/completion/stream-selection errors and demands nonzero censuses for those branches. The seedless explicit witness catches a common delayed-recovery/line-fill timing error, loss of signed zero or subnormal outputs and collateral failed-lane zeroing that optimized/scalar bit parity can share. The existing audited allocation test retains positive allocate/free controls and exact-zero measurements through all prior scalar/bank/mono/padded/failure/reset scopes; realloc still fails the allocation gate. Other ownership/iterator/naming changes preserve every original input and assertion, with purposes recorded above. No test is added, no historic implementation is newly pinned and no gate is loosened.

Root integrated locked all-feature graph and true-peak-limiter packages: 154 test/doc cases PASS, one existing ignore, no failure (log /tmp/engine-housekeeping-graph-limiter-tests.log). Full package debug/release, strict lint/policies and sole-owner native G5 PASS. No speed percentage, new SIMD lowering, target hardware or listening claim. PASS covers this housekeeping; unchanged required qualification controls main integration.
