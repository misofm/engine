# Test-usefulness audit — rack, builtins, graph, host-core, host-web, hosts, tools/*

Method: every listed file read in full (assertion bodies, loop bounds, corpus sizes); no cargo. Cost classes are estimates from bounds; "unverified" marks the ones I could not corroborate. CI facts that drive verdicts: `ci.yml:70` runs `cargo test --workspace --all-targets` in **debug** (where every test below is paid for); `ci.yml:231` reruns `-p builtins` in release, `ci.yml:481` reruns `-p wasm-gates` in release; `scripts/check-builtins-fixtures.sh` + `test-builtins-fixtures.sh` run at `ci.yml:248-249`.

---

## tools/audit (29 tests, 0 ignored) — the heavy binary, read directly

### What the fixture_builtins tests actually do

Two cost centres in `tools/audit/src/fixture_builtins.rs`:

**A. `generated()` (line 442) — the full authoring render.** Called via `complete_files()` (5602) by **five** tests. Each call:
- `responses()` (797): 1,630 rows (`RESPONSE_ROW_COUNT`, line 42) = 4 rates × {2 sections × 6 cutoffs × ≤7 probes + cascade probes} × 5 quanta. Each row runs `measure_response` (844): a `rate`-frame impulse render **plus** `sustained_metrics` (938) of 0.75·`rate` frames through `BuiltinChain`, plus a `rate`-length DFT (920) and two sin/cos fit passes. ≈ 1,630 × 1.75 × 69k ≈ **200M chain frames + ~115M complex DFT iterations** per call.
- `resources()` (1360): the 3×3 grid `tracks ∈ [1, 4, 65_537] × meters ∈ [0,1,7]` — **three `compile_session` + `prepare_session_builtins` runs at 65,537 tracks** per call (1364-1398, via `fixture_session_tracks`). builtins-compiler's `scale.rs` (one such compile + two prepares) measures 22-42 s, so this alone is ~30-60 s per `generated()`.
- `cases()`, `diagnostics()`, `meters()`, `pcm_cases()` (8-frame renders), `graph_tap_fixtures()`: cheap.

**B. `check_fixture_root()` (1788) — the read-only checker.** Cheap stages (manifest, sha256, `verify_cases`, CSV coverage count) come first; then `verify_reference_oracle` → `verify_response_oracle_tolerances` (4453) runs `independent_response_measurement` (4627) per *invariant coordinate* (rate, section, cutoff, probe — quantum dropped, 4253): 1,630/5 = **326 coordinates × (rate impulse + 0.75·rate sustained) through `ReferenceRetainedTptF32` + 326 DFTs ≈ 40M reference samples** per pass. Everything after it (`verify_functional_fixture_completeness`, meters, diagnostics, resources, benchmark inputs) is cheap. So a check that fails *before* the oracle is milliseconds; one that fails *after* it (or passes) costs one oracle pass, call it **O** (est. 5-10 s debug, unverified).

| test (line) | renders `generated()` | oracle passes O | why |
|---|---|---|---|
| `issue064_checked_corpus_is_read_only_complete…` 5196 | 0 | 1 | `--check` on a copy of the checked-in corpus |
| `issue064_checked_corpus_rejects_exactly_twenty_four_corruptions` 5278 | 0 | **4** | 24 copies of the 1 MB corpus; Delete/UnlistedAdd/StaleByte (18 cases) all die in `verify_manifest_bytes`; Toml/Csv semantic holes die at coverage counts; only F32Le/Meter/Diagnostics/Resources holes reach the oracle |
| `issue067_graph_pdc_and_dependent_identity_mutations_are_rejected` 5339 | **1** | **4** | baseline pass + pcm-word/tap-field/dependent-hash mutations all sit after the oracle |
| `issue061_response_tuples_decimals_and_partitions…` 5398 | 0 | 0 | in-memory |
| `issue061_unsuffixed_ramp_and_reset_script…` 5423 | 0 (3× `pcm_cases()`, 8 frames) | 0 | cheap |
| `check_rejects_all_twenty_four_format_mutations` 5463 | **1** | **4** | same 6×4 matrix as issue064 but on the production render; delete/alter/add die at manifest; f32le/meter/diag/resources coverage holes reach the oracle |
| `check_rejects_owned_jsonl_tuple_mutations` 5521 | **1** | **6** | all six mutations are post-oracle |
| `owned_jsonl_parsers_reject_…` 5554 | 0 | 0 | pure parsers |
| `check_rejects_manifest_grammar` 5579 | **1** | 0 | all three die in `parse_manifest`/`verify_manifest_bytes` — the render is wasted |
| `check_rejects_benchmark_identity_parameter_and_pcm_hash_mutations` 5614 | **1** | **3** | benchmark checks are the last stage |
| **total** | **5 renders** (≈15 compiles/prepares at 65,537 tracks + ≈1 G chain frames) | **22 O** | consistent with the measured 233-470 s |

**Cheapest restructuring that keeps every discriminating claim:**
1. Stop calling `generated()` in mutation tests. The checker is a pure function of a directory; use `copied_checked_in_fixture_root` (5862, 1 MB copy) for `check_rejects_all_twenty_four_format_mutations`, `check_rejects_owned_jsonl_tuple_mutations`, `check_rejects_manifest_grammar`, `check_rejects_benchmark_identity…`. (4 renders → 0.)
2. Keep **one** production render, memoised in a `OnceLock`, for the single claim that needs it: "the production-rendered corpus passes the independent checker" (`issue067` baseline, 5343). That is also the only place `resources()` needs its 65,537-track row rendered in tests.
3. Post-oracle mutations should call the stage function directly (`verify_meter_corpus(root)`, `verify_diagnostics(root)`, `verify_resources(root)`, `verify_benchmark_inputs(root,&manifest)`, `verify_functional_fixture_completeness(...)`) — each is milliseconds — and keep **one** end-to-end post-oracle mutation through `check_fixture_root` (the `ResourcesJsonl` semantic hole, the last stage) to prove nothing short-circuits. 22 O → 3 O (read-only pass, issue067 baseline, one pipeline-order mutation).
4. Merge `check_rejects_all_twenty_four_format_mutations` into `issue064…twenty_four_corruptions` (identical 6×4 matrix; the only difference is which corpus) and trim the manifest-stage cases from 18 to 3 (one class × delete/alter/add — the error strings at 5988-5996 are class-independent). Keep all 6 semantic holes (genuinely different code paths).
5. Drop the manifest-sha256 pin at 5218 (re-pinned three times already per its own comment; the corpus is integrity-checked per file by `check-builtins-fixtures.sh` and semantically by `--check`). Keep the 50-file count only if wanted.

Estimated debug wall after restructuring: **~25-40 s** (one render ≈ 20-30 s dominated by the 65,537-track rows + 3 O), from 233-470 s.

Note on `scripts/test-builtins-fixtures.sh`: its `coverage` mutation edits `cases.toml` without refreshing `MANIFEST.tsv`, so it is caught by the sha256 stage, not the coverage stage — name promises more than it tests. Its five mutations are all also in the Rust tests; the script gate is the release-path duplicate (it runs `cargo run --bin audit` in **debug** at `ci.yml:248`, one full O per run).

### tools/audit table

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tools/audit/src/fixture_builtins.rs | 10 | 0 | The builtins fixture corpus is checked read-only against independent oracles and every corruption class is rejected at its own stage | mutation-proof + digest-pin | **heavy** (5× `generated()`, 22 oracle passes; see above) | `scripts/test-builtins-fixtures.sh` (5 of the 24 mutations), `scripts/check-builtins-fixtures.sh` (per-file sha; runs `--check`); the two 24-matrix tests duplicate each other | TRIM+MERGE per steps 1-5: keep `issue064 read-only` (drop sha pin), merged 9-case corruption matrix, `issue067` (memoised render), both `issue061`, `owned_jsonl_parsers`, `check_rejects_manifest_grammar`/`owned_jsonl`/`benchmark` on the checked-in corpus via stage functions |
| tools/audit/src/builtins_fixture_check.rs | 2 | 0 | The audit-fixture checker is read-only and rejects a payload byte flip | behaviour | trivial (5 files, 28 KB) | none found | KEEP `issue069_checker_is_read_only…`; DELETE `issue069_author_is_not_reachable_from_audit_mains` (538) — `include_str!` scrape of sibling sources for `--write`/`write_scratch`; protection is the read-only tree-hash test plus module visibility |
| tools/audit/src/builtins_graph.rs | 3 | 0 | Two graph plans render identical PCM with success vs saturated meter queues; the retirement worker reclaims plan A on its own thread | behaviour | small (two `prepare_graph_plan` compiles of canonical.json, 3 blocks × 128 frames; spin-loop threads) | `tools/bench/src/builtins.rs:2009` asserts the same success/full drop-count claim on another plan | KEEP 724, 757; DELETE `issue070_retirement_worker_source_is_limited_to_nonblocking_primitives` (845) — `concat!`-obfuscated scrape of its own source; assert instead with a `clippy::disallowed_methods` entry (`clippy.toml` already carries the mechanism; `main.rs:1` `#![allow(clippy::disallowed_methods)]` is why it was scraped) |
| tools/audit/src/capi.rs | 3 | 0 | Evidence JSON names nine counters; prepare/destroy never enters render scope | behaviour + tautology | small (`PreparedAudit::prepare` builds engine via C ABI) | none | KEEP 283, 317; DELETE `audit_plan_is_fixed_non_timed…` (328) — `assert_eq!(CALLS, 100_000)` is a constant re-derived, the timer scrape is `include_str!` of itself |
| tools/audit/src/source.rs | 1 | 0 | (none) | tautology | trivial | none | DELETE — `ASSERTED_TRANSCRIPT` (463) is a test-local string literal; the test hashes it, compares to a pinned fnv and `println!`s. No production code is exercised |
| tools/audit/src/source_duration.rs | 1 | 0 | The bounded-WAVE capture accounts exactly 17 layout entries / 6,416 bytes | digest-pin (allocation) | small (writes one quantum WAVE to `env::temp_dir()`) | none | KEEP; TRIM the fnv pin over a `{:?}`-formatted string (325) — pin the 17 `(category,size,align,count)` rows instead; drop the `println!` |
| tools/audit/src/source_fixture.rs | 1 | 0 | Generated WAVE fixtures match `fixtures/sources/v1/manifest.sha256`, decode via an independent oracle, hit the exact diagnostic matrix, and 256 seek schedules match the model | property/oracle + digest-pin | small-medium: 256 schedules × ≤8 quanta of 4 frames (607-609) | **none found** — `fixture-source` subject is wired into no script/workflow; this test is the corpus's only gate | KEEP |
| tools/audit/src/fixture_builtins_listening.rs | 5 | 0 | Listening renders are strict-shape checked, level matched, blind-named, closed-permission, no-clobber | behaviour | medium: `FRAMES = 480_000` (16); `block_events…` renders 4 roles twice = 3.84 M frames; `output_is_closed…` once more | none (`scripts/operator/prepare-builtins-listening.sh` is the production caller) | KEEP; TRIM `FRAMES` for tests to 48_000 (`EVENT_FRAMES` at 20 and `SILENCE_FRAMES` 480 fit) — nothing asserted depends on 10 s |
| tools/audit/src/vectorization.rs | 3 | 0 | `certify` is red on missing family, scalar fallback, incomplete allowlist | behaviour | trivial (synthetic disassembly strings) | `scripts/test-native-vectorization-report.sh` is the objdump-level gate | KEEP |

Flags: `env::temp_dir()` + pid + nanos everywhere (fine, but 24 roots × 1 MB are copied in one test); no env-var reads in tests; `issue064` test string-scrapes its own file (5236-5273 `calls_authoring`) — acceptable as a reachability seal but brittle; `INTERNAL_SUBJECT` env dispatch in `main.rs` is production, not test.

**Summary tools/audit:** 29 tests; keep 20; merge/trim 5 (the fixture_builtins family, listening FRAMES, source_duration pin); delete 4 (`source.rs`, `capi` plan test, `builtins_fixture_check` author scrape, `builtins_graph` source scrape). **Estimated debug seconds saved: 200-430 s** (233-470 → ~25-40).

---

## crates/rack

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| crates/rack/src/lib.rs (mod tests L2163-3499) | 20 | 0 | AoSoA gather/scatter is a bit-exact permutation; fold/aux epilogues exact; mono collapse fires only when armed+eligible and renders the dual bits | mutation-proof (M1-M7, M2-1..9, 218-R1/R2) | trivial (max 512 frames × 8 lanes × 5 partitions, L2971) | `console-workload/tests/chain_shape.rs` re-proves collapse claims at 64 tracks; `mono_reengage.rs` subsumes one | KEEP 17; MERGE `arming_a_fold_refuses…` (L2646) + `arming_an_auxiliary_destination_refuses…` (L2685) into one; MERGE `inactive_lanes_are_never_gathered_or_scattered` (L3021) into `gather_scatter_round_trip_is_bit_exact` (L2717, already catches M7); DELETE `disengaging_copies_the_prefixs_state_once` (L3471) — strict subset of `mono_reengage::the_forced_off_window…`; TRIM the `size_of` line at L2922 |
| crates/rack/tests/mono_reengage.rs | 4 | 0 | Re-engagement only on a provable agreement; cycled run renders never-collapsed bits | property/oracle | trivial (≤40 blocks × 4 lanes × 8 frames) | chain_shape.rs L979/L1201/L1296 are 64-track twins of tests 1-3 | KEEP all 4 (the cheap survivors; cut the console-workload copies instead) |
| crates/rack/tests/console_bank.rs | 10 | 0 | Bank stage hands each lane its own spans, per-lane bypass at declared latency, no observation state unless asked | mutation-proof | trivial | host-core `effect_console.rs` (real bank), `effect_observation.rs` E5 | KEEP 9; DELETE `a_stage_with_no_controlled_lane_allocates_no_shunt` (L420) — asserts only `dropped_records()==0` on a fresh stage; the `shunt` field is private and unobservable |

Flag: `a_collapsed_seam_keeps_the_matrixs_operation_order` (L3267) half-tests its own mock `Matrix`; name overreaches. No env reads.

**Summary rack:** 34 tests; keep 30; merge 3; delete 2; nightly 0; seconds saved ≈ 0.

## crates/rack-compiler

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| crates/rack-compiler/src/lib.rs (L446-1193) | 13 | 0 | `plan_bank_groups` forms level-uniform, program-homogeneous, ascending banks; exhaustive pooling; class partition; input-order invariant | property/oracle (4 × 200-case corpora, L566/L980/L1034/L1113) | small (<1 s) | `plan_invariants_hold` is already a `debug_assert!` inside the planner (L353); graph-compiler `a_subsequence_program_track_binds…` is the render twin | KEEP 11; MERGE `every_slot_cohort_is_homogeneous` (L1034) + `invariants_hold_on_seeded_corpus` (L1131) into one corpus walk; rename `program_comparison_is_a_total_order` (L1174, three comparisons prove no total order) |

**Summary rack-compiler:** 13; keep 11; merge 2→1; delete 0; ≈0.2 s saved.

## crates/host-core

P = one `prepare_host_session` of the 8/9-track fixture (≈50-200 ms debug, unverified).

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| src/solo.rs | 7 | 0 | Solo = mute the complement; minimal restore records; transactional | behaviour | trivial | host-web solo tests build on it, don't repeat the algebra | KEEP 7 |
| tests/source_diagnostics.rs | 3 | 0 | 17 `SourceControlError` variants pin their own string/classification | digest-pin table + exhaustiveness | trivial | `prepare.rs:296-306` repeats 5 rows; `capi ffi.rs:1886` 2 rows | KEEP 3 |
| tests/prepare.rs | 12 | 0 | Prepare facade reports shape, enforces every byte cap at its own row, types source rejections, attaches a bounded console only when asked | behaviour + mutation-proof | small (`every_byte_cap…` 15 P; `session_validation…` 18 compiles) | see verdicts | KEEP 8; TRIM `session_validation_owns_the_launch_rate_set` (L385) 18→4 (rule is `session`'s, pinned by `session-validator/tests/validate.rs:169`); TRIM tail of `source_control_errors_are_typed` (L296-306 dup of source_diagnostics); TRIM `dense_refusal_diagnostics…` (L563) — keep count, drop `elapsed < 1s` (NIGHTLY); DELETE `default_ring_derivation_covers…` (L408) — `parameter-metadata/tests/abi_layout.rs:249-265` asserts the exact formula incl. the `9_906` pin; this is a lower bound |
| tests/effect_console.rs | 2 | 0 | A real cohort-planned bank hands lane l track l's spans only | behaviour | small (5 P) | rack `console_bank.rs` (mock); 140-15 caught only here | KEEP L189; MERGE L226 into it |
| tests/effect_observation.rs | 10 | 0 | Taps read without moving a bit; per-lane reductions equal scalar twins; windows tile; subscriptions die with the plan | property/oracle + timing | medium (E7 L864: 4×272 compressor-bank blocks + 3×4096 loop) | `effect-contract/tests/observation_lane.rs:153` owns `wants` | KEEP 8; TRIM `observation_retained_bytes…` (L809-815, drop `48/104/832` literals); `observation_cost_classes_are_what_they_claim` (L864): NIGHTLY the timing half, DELETE the deterministic half (exercises a transcribed copy of `publish_observations`, L910-919, so production mutations can't fail it); `println!` present |
| tests/fp_environment.rs | 3 | 0 | Render entry pins FTZ/DAZ off and restores MXCSR bit-exactly incl. error paths | property/oracle | small (3 P, 16 blocks) | `capi/src/runtime/tests.rs:1602` is the C-entry twin | KEEP 2; MERGE `a_started_session_hands_the_plan_back…` (L140) into L237. Hidden global: MXCSR (guarded by Drop) |
| tests/input_liveness_console.rs | 10 | 0 | Live trim/polarity reach exactly the addressed lane on the draining block; per-lane drain on the disengage block reaches one channel | property/oracle + mutation-proof | medium (`a_per_lane_record_drained…` L689: 24 P + 192 blocks; `a_command_moves…` 9 P) | 3-layer duplication with `builtins/tests/input_liveness_mono.rs` and chain_shape.rs (table in builtins section) | KEEP 8; DELETE `two_commands_are_not_interchangeable` (L389, caught by the per-track loop at L326); TRIM L689 12→3 combos ((Left trim ride, t0), (Left trim snap, t2), (Right polarity snap, t7)) |
| tests/symmetry_witness.rs | 11 | 0 | The witness declines exactly the track/lane a mapping, designed word, live one-channel write or bypass touches | behaviour | small | `track_delay.rs::eligible()`, `builtins-compiler/tests/input_drain.rs` | KEEP 10; MERGE `a_left_channel_command_declines_exactly_one_lane` (L504) into L541 (both name the same red mutation; L557's "only test that does" claim is false); L219 needs only `compile_host_session`, not 3 prepares |
| tests/track_delay.rs | 8 | 0 | `delay_samples=N` renders bit-identically to a pre-padded source; lanes independent; symmetric delay collapses | property/oracle | small-medium (L215: 8 P + 142 blocks, 90 of them for N=4800) | graph-compiler `tests/track_delay.rs` (plan level), builtins-compiler `track_delay_domain.rs` (domain) — clean layer partition; only overlap is the zero-delay claim | KEEP 6; DELETE `a_zero_delay_arm_never_lowers_a_delay_node` (L191 — `document.contains("\"delay_samples\":0")` scrapes JSON the helper itself wrote; survivor graph-compiler L189); TRIM L215 to N∈{1,128,200} unless a ring boundary above 200 exists (unverified) |

**Summary host-core:** 66 tests; keep 54; merge 4; trim 5; nightly 1; delete 4. **≈8-15 s saved** (E7 4-8 s, per-lane trim 2-4 s).

## crates/builtins

Runs twice per CI (debug workspace + release `ci.yml:231`).

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tests/contract.rs | 4 | 0 | 12-row descriptor table and rate-keyed cutoff domain frozen | digest-pin + behaviour | trivial | `check-parameter-metadata-v1.py:285-294`; cutoff max bits also in `response.rs:791`, `builtins-compiler/src/lib.rs:4549` | KEEP; MERGE the cutoff block of L211-224 into L313 |
| tests/determinism.rs | 2 | 0 | 8 corpus cases digest to scalar pins at f32/Simd4/Simd8; none vacuous | digest-pin | trivial (8 × 256 fr × 8 lanes × 3) | `wasm-gates g5_native_corpus.rs:21` replays these pins in release | KEEP (the cheap owner). Env: `MISO_ENGINE_REPIN_BUILTINS_CORPUS` (L35) |
| tests/fader_ramp.rs | 8 | 0 | Live fader/mute bit-identical to prepared path when uncommanded; mute is a ramp endpoint to exact +0.0 | mutation-proof | trivial | `stage.rs:1067` banked form | KEEP 8 |
| tests/input_liveness.rs | 13 | 0 | Class-A OFF, smoother-law identity, partition invariance, elided≡ramping, selector rules | property/oracle | trivial-small | host-core `input_liveness_console.rs:242`; `effect-runtime contract_ramp_identity.rs`; `lane input_chain_elision.rs` | KEEP; TRIM sweeps: L279 36→9 (windows {1,2,128} × 3 pairs), L499 15→6 |
| tests/input_liveness_mono.rs | 8 | 0 | Live trim on a collapsed bank: witness reads live words; symmetric rides keep the collapse; disengage copy restores integrators | mutation-proof | trivial | host-core + chain_shape twins | KEEP 7; MERGE `the_disengage_copy_still_restores_the_integrators` (L490) into `mono_collapse.rs:176` |
| tests/matrix.rs | 7 | 0 | 2×2 ramp is the D11 law vs `ReferenceLinearRamp`; retarget from value in flight; zero window snaps | property/oracle | trivial | audit `pcm/matrix-ramp-*` pins the fixture, not production | KEEP 6; DELETE `matrix_ramp_reaches_target` (L185, inside the L38 sweep) |
| tests/meter.rs | 7 | 0 | Windows tile; counters exact; segment law partition-invariant; early-out exact | behaviour + stress | small (L110: 10,000 prepares) | audit `meters/window-and-drop.jsonl` checked against an *independent* meter | KEEP 5; MERGE `meter_windows_are_exact` (L15 ⊂ L39); TRIM L107 10,000→1,000 (config lattice exhausted long before) |
| tests/mono_collapse.rs | 2 | 0 | Collapsed body publishes the dual body's report; desymmetrized bank renders never-collapsed bits | mutation-proof | trivial | chain_shape L903 (plane half) | KEEP both |
| tests/response.rs | 8 | 0 | Prepared TPT cascade matches the f64 RBJ oracle within 007/031/036 tolerances | property/oracle + stress | **heavy** (18-37 s): `one_second_impulse_dfts…` (L266) 60 rate-length renders + **300 rate-length DFTs ≈ 414M complex iterations** recomputed for quanta already asserted bit-identical (L301); `coherent_sustained_sines` (L332) ≈ 250M sample-ops; `representable_cutoff_domain…` (L790) **9.43 M `SvfSection::design` calls** | audit `responses()` is a second implementation of the same grid, but `--check` never re-renders production — **this file is the live gate** | KEEP claims; TRIM: DFT once per (rate,cutoff,kind) (−80%), quanta {1,1024} (partition invariance is bit-gated by `stage.rs:425`), drop **extended rates** (file itself calls them "informational … not a support claim", L34-35 → NIGHTLY), L790 stride 4,096 + endpoints, L82 10,000→1,000; then RELEASE-ONLY for L207/L266/L332 (release leg already exists) |
| tests/speed.rs | 1 | 1 | none — prints ns/frame | descriptive/no-assert | (4 × 20,000 × 128 if run) | tools/bench builtins benchmark | DELETE |
| tests/stage.rs | 12 | 0 | Input stage is the reference recurrence bit-for-bit; banks equal scalar at every width; signed-zero laws; elision decision | property/oracle + mutation-proof (M1-M15) | small (207k frames L144; 230k tan evals L133) | — | KEEP 10; MERGE `polarity_trim_fader_and_matrix_are_exact` (L591) into `signed_zero_and_mute_laws` (L522); DELETE `engine_tan_agrees_with_the_platform…` (L119) — a `math`-crate claim gated by m1/m3 in release; `assert_eq!(worst, 1)` (L136) couples to host libm |

**Summary builtins:** 71 tests (1 ignored); keep 60; merge 5; trim 6; release-only 3; delete 3. **≈20-35 s saved** in the debug leg.

## crates/builtins-compiler

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tests/allocation_tracker.rs | 1 | 0 | Phase-two allocator requests equal the phase-one report byte-for-byte; one-below caps reject before any phase-two allocation | allocation/realtime | **heavy** (18-41 s): tracks `[1,4,65_537]` × meters `[0,1,7]` (L161-163) → **6 full prepares + 9-18 cap-rejecting prepares at 65,537 tracks**, each through a mutex-locked tracker with a linear scan (lib.rs:956-985) | 65,537 row pinned by `fixtures/builtins/v1/resources.jsonl` (via audit) and `scale.rs`; per-dimension rejection by lib.rs L4478/L4508 | TRIM tracks to `[1,4]` (layout-class equality is count-invariant; 4 tracks exercise the linear Vecs) → <2 s. Flags: `#[global_allocator]` + `TEST_PHASE_TWO_*` statics live in every workspace test binary via `graph-compiler/Cargo.toml:28` |
| tests/scale.rs | 1 | 0 | No u16 track ceiling: 65,537 tracks prepare; one-below state cap rejects | scale/stress | **heavy** (22-42 s): compile + 2 prepares at 65,537 | same session compiled by `session/tests/scale_transaction.rs:22`, `graph-compiler/tests/scale.rs:28`; constrained arm duplicates lib.rs L4508 | DELETE the constrained re-prepare (L59-72); **65,537 = 2¹⁶+1 cannot be shrunk** for this claim → RELEASE-ONLY/NIGHTLY, optional 4,097-track debug smoke |
| tests/input_drain.rs | 5 | 0 | Input record is upstream of the seam; per-lane desymmetrizes; LIVE is a latch | behaviour | trivial | host-core L437/L724 | KEEP 4; DELETE `both_variants_answer_the_hook_identically…` (L162, implied by L57) |
| tests/track_delay_domain.rs | 2 | 0 | Descriptor domain == schema max; a delay moves no builtin byte | behaviour | small | `contract.rs:258` second spelling | KEEP |
| tests/builtin_automation_targets.rs | 3 | 0 | `BUILTIN_AUTOMATION_TARGETS` == BlockTarget rows of the ABI | behaviour | trivial | `contract.rs:96-131` | KEEP L27; MERGE L53 into it; DELETE L94 (re-asserts a string constant) |
| src/lib.rs (mod tests L3309-5411) | 16 | 0 | Preparation validates/seals/charges; banked lowering bit-identical to scalar; frozen 10k mutation matrix | behaviour + property + digest-pin | medium (L4610 10,000 × compile+prepare+render, unverified 5-15 s) | — | KEEP 13; MERGE `track_bits_do_not_depend_on_session_track_count` (L4352) into L4313; TRIM matrix 10,000→5,880 (coordinates cycle with lcm(49,4,5,6)=5,880, L4660-4679; re-pin transcript once). `Backend::current()` (L4301) makes the bit-identity harness compare scalar-to-scalar on scalar hosts |

**Summary builtins-compiler:** 28 tests; keep 22; merge 2; trim 3; release-only 1; delete 2. **≈45-85 s saved**.

## crates/graph

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| src/runtime.rs | 10 | 0 | Executor kernels (reduction order, fold, PDC ring, track delay, route 2×2, gather redirect) bit-exact vs independent oracles; every non-copy shape refused | property/oracle + mutation-proof | small (7 delays × 5 blocks × 4,096, L3325; 4,096-frame route oracle, L3392) | lib.rs `delay_is_exact…`/`reduction_is_left_to_right` are subsets | KEEP; MERGE `a_single_in_place_input_is_left_untouched` (L3309) into L3248 |
| src/program.rs | 11 | 0 | `lower()` preserves dataflow under aliasing, in-place folding, PDC staging, bank-window hoisting, cohort merging, bounded arena | property/oracle (symbolic interpreter) | medium: two 4,000-graph corpora lowered 2-3× (L2182, L2336; est. 8-20 s unverified) | none — sole home of the interpreter oracle | KEEP; TRIM both corpora 4,000→1,000, turn the 7 exact corpus-count pins into `>=` bounds |
| src/lib.rs | 24 | 0 | `bind` validates layout/bindings transactionally; bound plan renders PDC, aliases, banks, console commands, bypass bit-exactly | behaviour + mutation-proof | small (L4628 renders every launch rate × 5 quanta × 2) | `bin/graph_fixture.rs` summation report; audit `builtins_graph.rs:620-658` PDC rows | KEEP 17; MERGE size-of pair L1550+L1568; DELETE `delay_is_exact_and_lane_independent` (L2441, ⊂ runtime L3325), `reduction_is_left_to_right` (L2450, `[1,2,3]→6.0` holds in any order), `left_to_right_reduction_meets_analytic_bound…` (L2463, shuffle-then-sort tautology + bound weaker than runtime L3248); TRIM `fifty_random_dag…` (L3445: name says deterministic, nothing compared — add `render(seed)==render(seed)`, 50→20) and L4628 to rates {min,max} × quanta {1,127,1024} |

Flag: `tests/MUTATIONS.md` names retired tests (M5/M6/M8 `_in_both_executors`, 218-4 `the_folded_master_is_the_reductions_own_bits` exists only in console-workload).

**Summary graph:** 45 tests; keep 36; merge 3; trim 4; delete 3. **≈8-15 s saved**.

## crates/graph-compiler

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tests/route_gain.rs | 3 | 0 | `math::db_to_gain_f32` vs f64 `powf` oracle; −19 dB witness bits | property/oracle | trivial | tests `math`, not this crate | KEEP; MERGE `unity_route_gain_is_exactly_one` into the sweep; rename `…within_two_ulp` (asserts ≤10, L62-66) |
| tests/scale.rs | 1 | 0 | No hidden track cap above 65,536 in the graph compiler | scale/stress + count pin | **heavy** (22-42 s): 65,537-track compile (L37-57), `session.clone()` (L65), constrained compile that still runs Tarjan over 458,761 nodes (compile.rs:436-448), then full compile (L80-94) | `session/tests/scale_transaction.rs`, `builtins-compiler/tests/scale.rs`, `tools/bench/src/graph.rs:213 graph_validate_65537_tracks` (identical fixture, release); cap rejection asserted 7× in lib.rs | TRIM to 10,923 tracks (76,463 nodes / 65,540 edges — crosses 65,536 on both counts, ~6× cheaper), drop constrained arm; NIGHTLY/RELEASE-ONLY the 65,537 form (or rely on the bench) |
| tests/track_delay.rs | 8 | 0 | Zero delay lowers no node; delay moves no PDC row; 4 B/sample/lane charged under caps | behaviour + digest-pin | small-medium (~21 compiles of the 9-track EQ fixture) | host-core/builtins-compiler: disjoint layers | KEEP 4; DELETE `the_zero_delay_plan_digest_is_the_pre_feature_digest` (L214 — own doc says the pin misses the mutation; requirement retired); MERGE L242 as `assert_ne!` without literal; TRIM L266 pairs to {(1,1),(48000,0)}; MERGE L329/L349/L364 sharing 4 compiles |
| src/bin/rack_fixture.rs | 1 | 0 | Issue-008 rack corpus checker rejects corruption | regen-helper | trivial | `fixtures/rack/v1` read by nothing; `--check` runs in no script/CI | DELETE test + corpus (retired requirement; external consumer unverified) |
| src/bin/graph_fixture.rs | 1 | 0 | Graph fixture checker rejects corruption | regen-helper | trivial | `graph_fixture --check` runs nowhere in CI (`check-graph-determinism.sh` uses only the fingerprint) | KEEP only if CI adds `graph_fixture -- --check`; else DELETE with the checker |
| src/lib.rs (mod tests) | 60 | 0 | Compiled plans canonical/deterministic, lower correctly, bank cohorts/chains/folds bit-transparent, caps transactional | behaviour + property + digest-pin + scale | **heavy in aggregate** (est. 150-300 s debug, unverified): ~22 tests compile the 64-track console (64 EQ + 64 comp [+64 limiter]) 2-4× and render 12-28 blocks per arm | `check-graph-determinism.sh` (100 fresh processes) supersedes in-process 100× repeats; audit `builtins_graph.rs` is the production-path PDC/PCM audit | KEEP ~44; TRIM/MERGE ~15; RELEASE-ONLY 1 (below) |

src/lib.rs non-KEEP verdicts (line = `#[test]`):
- `mixed_twelve_track_plan…` L2456: the **`MISO_ENGINE_AUDIT_037`-gated 100,000-block render + hash** (L2852-3196) hides inside a normal test → move to `#[ignore]` RELEASE-ONLY beside `tools/audit/src/builtins_graph.rs`.
- `canonical_artifacts_are_complete_and_repeatable_100_times` L9940: 100→2 in-process repeats (script gate is stronger); MERGE `direct_graph_report…` L1928 into it.
- `issue122_reverse_route_ids…` L1693: pin sha once; drop `repeated`/`single_artifact`/`wave_artifact` (L1875-1924).
- `add_a_track_keeps_existing_track_bits…` L3221: BLOCKS 32→8. `launch_compressor…` L3680, `dynamic_rack_compressors…` L3931, `rack_placement…` L4008 (also drop 2 unused compiles), `console_sixty_four_track…` L4400: 16/12→10 blocks (latency 960 = 7.5 blocks). `launch_gate_expander…` L6550 and `launch_true_peak_limiter…` L6794: 16→6 blocks.
- `intended_placement_merges…` L4567: drop the third compile (count asserted at L6416). `a_single_odd_track_strands…` L5072: keep counts + odd-vs-scalar pair only. `class_pooling_forfeits…` L5146: 12→1 block (folds decided at bind). `the_intended_strip_folds_every_route…` L5613: drop the metered arm (= L5734 verbatim).
- `frozen_issue_037_seeded_builtin_bank_layouts…` L9061: 100→27 layouts, drop `repeat_artifact` and the transcript literal (every field asserted per layout; `pcm_hash` derived by the D9 oracle).
- `ten_thousand_graph_mutations…` L10086: 10,000→1,000. `node_text_len_matches…` L1214: 1,000→100 iters.
- Six `launch_*` tests repeat ~200 lines of cap-rejection/estimate boilerplate; keep the limiter (L6794) as the representative for caps+estimate, soft-clip (L7583) for finite tail, drop the arms from multiband (L7329-7377, L7549-7579), transient-shaper, delay (L8319 — also carries delay-crate state-size literals `768_168`/`7_682_040`).
- `a_banked_fader_command_lands_on_the_block…` L5880 (largest single test, 4 × 28-block 64-track renders, est. 15-25 s): KEEP; 28→24 is the only safe trim.

Flags: 19 console tests `return` silently on a scalar host (`let Some(width) … else { return }`); one loud guard (`scalar_dispatch_compiles_without_banks_on_any_host` L1958) is enough. Both bins write to `env::temp_dir()`.

**Summary graph-compiler:** 74 tests; keep ~48; merge ~10; trim ~14; release-only 1; nightly 1 (scale); delete 2-3. **≈60-100 s saved** (scale 20-30, lib.rs 40-70 unverified).

## hosts/host-web (Rust only)

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| src/tests.rs | 59 | 0 | The browser-host facade/FFI boots, budgets, streams sources, applies commands at the acknowledged sample, and renders bit-identically to the shipped wasm pins | behaviour + digest-pin (3) + timing (1) + tautology (1) | small per test; file est. 10-25 s (two 1 MiB parses L101-137, one 64-track console compiled twice L691-748, ~100 boots) | `check-abi-layout-v1.py` (ladders/sizes), `check-web-boot-budget.mjs` (projection refusal, asserted 3×), `direct-oracle.mjs:491-497` (wasm leg of the 3 digests, `ci.yml:439`); `check-wasm-realtime-atomics.sh`/`check-protocol-wasm-parity.sh` overlap nothing here | KEEP 47; MERGE 5; TRIM 5; DELETE 1; RELEASE-ONLY 1 |
| tests/boot_transient_budget.rs | 1 | 0 | `PARSE_TRANSIENT_MULTIPLIER` (17×) bounds the native parse+compile peak; one-byte-under refuses before parsing | allocation/realtime | medium (2-6 s: 1/64/192/192-padded ×4 docs, L162-172, + 1 MiB refusal L200) | `check-web-boot-budget.mjs` re-measures the 192×4 doc on wasm; refusal leg = `tests.rs:483` | TRIM: drop 1- and 64-track interior cases (L168-169). Hidden global: `#[global_allocator]` + thread-local `ARMED/LIVE/PEAK` (L18-27) |

src/tests.rs non-KEEP: MERGE #1 `maximum_document_dense_invalid_fixture…` (L139) into #8 (L518, same bytes, already asserts the 64 lines) and RELEASE-ONLY #8's `elapsed < 1s` (L550); MERGE #6 `malformed_config_and_atomic_compile_failure_are_sticky` (L466 — nothing sticky/atomic asserted) and #35 `observation_configuration_words_are_validated` (L2708) into #10 (L585); MERGE #29 `command_flood…` (L2130) into #27 (L2006, same red mutation); MERGE #47 `solo_records_are_shape_checked…` (L3743) into #30's table (L2176); DELETE #37 `the_observation_fields_account_for_the_moved_bridge_rows` (L2904 — literal arithmetic `3_753-3_641==112`, stale vs `expected.json:62-63`; survivor `check-browser-expected-resources.py --artifacts`); TRIM #13 (L692) 3→2 fixtures, #28 (L2096) loop→1 and rename (staging is `None`, "every live kind" untested), #42 `solo_is_bit_identically_mute…` (L3393) 5→3 sets, #52 (L4130) 12→4 boots, #53 (L4157) 14 boots→1 host; drop the boot inside #2 (L312-325, `>=` satisfied by construction).

**Summary host-web:** 60 tests; keep 48; merge/trim 10; release-only 1; delete 1. **≈3-6 s saved**.

## hosts/host-native, hosts/host-mobile
No tests (`src/main.rs` / `src/lib.rs` only; no `#[test]`, `#[cfg(test)]`, or `tests/`). Nothing to audit.

## tools/wasm-gates

Corpus: 135 cases / 337 digest comparisons (51 lane cases × 3 widths + delegated families); one native render ≈ 6-12 s debug. G5's native comparison runs **three** times per CI (debug L70, release L481, `run-wasm-gates.sh:31 --native`).

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tests/g5_native_corpus.rs | 7 | 0 | Corpus matches pins natively at every width; finite/non-vacuous/distinct; `lane_fma` unfused; delegated pins are the owners' | digest-pin + property | medium (9-20 s: `native_report` L22 + two 51×3 re-renders L50/L356) | `g5_native_digests_match_pins` == `wasm-gates --native` in the same job (which also checks `minmax_lowering_mismatches`); delegated families pinned natively in release by each owning crate (`builtins/tests/determinism.rs:34`, compressor, soft-clip, parametric-eq, gate-expander, limiter, transient-shaper, multiband, delay, math m3, effect-runtime — `ci.yml:161-231`); graph/rack/audit hold **no** digest pins | DELETE `g5_native_digests_match_pins` (L21); MERGE `g5_lane_corpus_is_finite` (L50) + `g5_no_case_is_vacuously_zero` (L356) (one render); KEEP L84, L107, L143, L177 (pin-table only, trivial) |
| tests/g6_full_corpus_ftz.rs | 3 | 0 | Caller FTZ+DAZ never reaches a guarded render; guard is an identity when FTZ is clear | property/oracle | **heavy** (29-61 s: 5 full renders L122/129/138/163/166) | already run in release `ci.yml:481` (its doc says release is the measured profile); `lane/tests/g6_ftz_inert.rs` covers the lane subset | RELEASE-ONLY (`ci.yml:70 --exclude wasm-gates` or `#[cfg_attr(debug_assertions, ignore)]`); MERGE the identity test (L159) into the main one (its `without` arm is the `canonical` report already computed; 5→4 renders) |

**Summary wasm-gates:** 9 tests; keep 4; merge 2; release-only 2; delete 1. **≈38-81 s saved from the debug leg** with zero lost claim.

## tools/parameter-metadata

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tests/round_trip.rs | 3 | 0 | Every published parameter/tap/sidechain port resolves through a real command ack/preparation; one-past does not | property/oracle | small (3-4 boots, ~60 submits) | `check-parameter-metadata-v1.py` validates shape only | KEEP 3 |
| tests/abi_layout.rs | 7 | 0 | The emitted ABI-layout document is the engine's own bytes | behaviour (1), property (1), digest-pin (2), tautology (1), **string-scrape (2)** | trivial | offsets duplicate `host-web/src/tests.rs:250-262` + `check-abi-layout-v1.py`; fixture currency vs `check-sdk-generated.sh` and `parameter-metadata -- --check` in `check-web-audioworklet.sh:360` (three copies each pinned separately) | KEEP offsets-decode and source-ring rule (drop `assert_eq!(78*127, 9_906)`); MERGE alias-table test into the schema test; TRIM literal offset rows from the schema test; DELETE `rendering_is_deterministic` (pure fn of consts); DELETE `the_published_export_set_is_the_frozen_artifact_set` — scrapes `scripts/check-web-audioworklet.sh`'s `expected_exports=$(printf …)` heredoc; **assert instead**: make `parameter_metadata::abi_layout::EXPORTS` (`src/abi_layout.rs:103`) the single source, have the script read `exports` from the shipped `miso-engine-v1-abi-layout.json` it already validates, and in Rust check `EXPORTS` against a `(name, fn-pointer)` table of every `#[unsafe(no_mangle)] extern "C"` in `host-web/src/ffi.rs` so an unpublished export fails at compile time; `the_checked_in_self_test_fixture_is_current` is brittle (`env!` + `../../scripts`) — better to have `check-abi-layout-v1.py --self-test` consume `cargo run -- --print-abi-layout` and drop the third copy |

**Summary parameter-metadata:** 10; keep 5; merge/trim 3; delete 2; ≈0 s (value is removing two cross-file scrapes).

## tools/session-validator

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tests/skill.rs | 1 | 0 | `.claude/skills/author-session/SKILL.md` names commands that exist | descriptive/string-scrape | trivial | none | REWRITE: the five literals (L23-29) are hard-coded in the test, so renaming `--canonical` in `src/lib.rs:346-366` leaves it green while the skill goes stale. Assert instead: extract fenced `session-validator -- …` lines from SKILL.md, feed the argv to `session_validator::run()` against `fixtures/session/v1/canonical.json`, require `SUCCESS`; derive flags from the lib's `USAGE` const |
| tests/validate.rs | 8 | 0 | Every valid fixture passes all four stages; each defect attributed to its stage; canonicalisation a fixed point and byte-equal to web-boot diagnostics | behaviour + property + digest-pin | small-medium (13 fixtures incl. three 300-358 KB consoles through builtins preparation, 1-4 s) | schema rows re-test `crates/session` diagnostics; no script runs the validator over all fixtures | KEEP 5 (L76, L116, L403, L429, L442); MERGE `the_duplicate_key_fixture_fails_the_grammar_stage` (L163) as a row in the mutation table; TRIM `each_mutation_is_attributed…` (L353) 21→~7 — one row per stage-1 code family + grammar + preparation, and **add the missing stage-2 (resource caps) row** the file's own claim (L39-42) promises; DELETE `the_report_is_deterministic` (L461) |

**Summary session-validator:** 9; keep 5; merge/trim 3; delete 1; <1 s.

## tools/console-workload

Unit = one 64-track full-strip 128-frame block in debug; `BLOCKS = 64` in all three files; each `SessionRuntime::build` parses a 288 KiB fixture and compiles session + builtins + three effect banks.

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tests/placement.rs | 3 | 0 | Moving the compressor to `simd1` moves no bit; the limiter does | digest-pin | medium (6 builds + 6×64 blocks) | bench in-run `console.rs:1033`, `console-benchmark-record-lib.jq:502`, chain_shape L212 | DELETE L44 (⊂ L81); KEEP L62 at 8 blocks, L81 at 16; fold file into chain_shape.rs (one fewer debug link) |
| tests/automation.rs | 4 | 0 | Restating a threshold moves no bit; moving it moves every block; automated track is `ch00` | digest-pin | medium (4 builds + 4×128 blocks) | bench `console.rs:1530` (recorded runs only) | MERGE L86+L118 into a three-arm loop, PREROLL 64→16; MERGE L170 into L152 |
| tests/chain_shape.rs | 21 | 0 | One fused bank chain per cohort, one transpose per chain per block, one folded route per track; mono collapse engages/disengages/re-engages without moving a bit | mutation-proof counts + digest-pin | **heavy** (35-62 s: ~128 builds, ≈6,500 block-units; the three 16-workload sweeps at 64 blocks — L696-728, L836-884, L1053-1066 — are ≈58% of render work) | Count claims exist nowhere else at console level (graph-compiler L6402 covers chain count without console facilities; jq only checks positivity); mono-pair digest equality asserted 4× (console.rs:1225, record-lib.jq:585, tests #9/#19); rack `mono_reengage.rs` isolates the same mechanisms cheaply | TRIM BLOCKS 64→16 file-wide; collapse the three WORKLOADS sweeps (#6 L353, #12 L836, #15 L1047) into one build-once sweep with stereo rows at 2 blocks (bind-time claims); #11 `the_folded_master…` (L797, 30 builds) → 4 representatives at 8 blocks (Ragged9, Console64, Console64Mono, Stretch128; drop HalfMono whose baseline folds 0); MERGE #3 into #2, #9 into #12; #7 reuse 3 `counted` builds; #17/#21 SWITCH 6-8; est. 35-62 → 12-20 s |

Flags: `[3,9]` (L326), `[8,48]` (L372), `(64,[64,129])`, `(32,[128,193])` are 8-lane constants via `Backend::current()` — fail on aarch64 despite the file's comment (L55-58); names over-promise: `every_standing_workload_folds_one_route_per_track` (two rows fold zero), `no_workload_transitions_unless…` (vacuous on 13/16 rows).

**Summary console-workload:** 28 tests; keep 18; merge/trim 9; delete 1. **≈30-45 s saved**.

## tools/bench

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| src/conformance.rs | 1 | 0 | percentile + escaper behave | tautology (re-tests bench-support) | trivial | bench-support stats.rs:39, json.rs:211 | DELETE |
| src/session.rs | 2 | 0 | 256-track issue-004 fixture shape; canonical bytes idempotent | behaviour | small | percentile → bench-support; sha half pins a **hand-rolled SHA-256** (L446-585) | KEEP 1; DELETE 1 after replacing the private SHA-256 with `bench_support::digest::sha256_hex` (the F4 "second copy" `check-bench-policy.sh` cannot see) |
| src/graph.rs | 2 | 0 | Canonical issue-006 fixture (256t/1024r) prepares and compiles | behaviour | small-medium (1-5 s unverified) | percentile → bench-support | KEEP `canonical_benchmark_fixture_prepares_and_compiles`; DELETE percentile |
| src/effect_interchange.rs | 1 | 0 | Two-step D1→D3 migration restores the pinned 283-byte envelope | digest-pin | small | `effect-interchange-benchmark-108-validator.py:41`; `effect-compiler/tests/migration_terminal.rs:691` | KEEP; drop the implied `assert_ne!` (L1082) and the `#[cfg(test)] const` (L32-34). **Flag**: `scripts/check-effect-interchange-benchmark-108.sh:37-50` string-scrapes this test's source for literal tokens — should run `cargo test -p bench exact_four_rate` instead |
| src/protocol.rs | 4 | 0 | Frozen 54-frame corpus checksum; BTLV/FlatBuffer round trip; JSONL record fields | behaviour + digest-pin | small (`corpus()` builds 10,000 records per test) | checksum also asserted by the binary (L1359) and `check-protocol-benchmark-wasm-parity.sh` | KEEP 4; DELETE the four `schema.contains(…)` lines (L1594-1598) grepping the `.fbs` — cannot catch `VT_*` offset drift (L900-912); rename `btlv_sources_encode_and_decode_without_schema_escapes` (only checks no panic) |
| src/rack.rs | 3 | 0 | Three issue-038 workloads prepare to exact shape without launching; input identity deterministic | behaviour | medium | `check-rack-benchmark-fixture.sh`; bench-support stats | KEEP L920 (**fails on aarch64**: asserts `Backend::current()==Simd8`, L125-133); TRIM L950 1000 observations→2 (7.2 M 4-byte `Sha256Sink::update`s; no layout independence asserted despite the name); DELETE L963 percentile |
| src/floor.rs | 7 | 0 | #184 floor table names a ruling per row; controls cheaper than isolates; plumbing row is the floor | behaviour/table-pin | trivial | `console-benchmark-validator.jq:38-59` (recorded runs) | KEEP 5; TRIM L433 (L441-443 re-derive `(22-4)/(8*3.7)`); DELETE `the_compressor_isolate_is_the_compressor_inventory` (L504, `(B+C)-B==C`) |
| src/builtins.rs | 9 | 0 | Issue-035 records have the exact 61-key shape; inputs match the checked manifest; render workloads arm only the product render; 256-track projection address-free | behaviour + mutation-proof | one medium, rest small | `builtins-benchmark-validator.jq` (recorded runs); `check-builtins-fixtures.sh` (files, not the Rust hash table L1538); audit `builtins_graph.rs:724` (same success/full claim) | KEEP 9; TRIM `all_render_workloads_arm_only_product_render…` (L1935): `RENDER_WARMUP_BATCHES`=64 × 8 ops × 16 arms = 8,192 warm renders before one asserted op — 2 batches suffice (uses process-global `audit::reset()/snapshot()`, order-sensitive if another test arms a scope); drop literal-vs-literal `ded3579e…` (L2001-2004) |

**Summary bench:** 29 tests; keep 22; trim 4; delete 6 (+ scrape lines). **≈6-18 s saved** (unverified).

## tools/bench-support

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| src/stats.rs | 6 | 0 | Nearest-rank percentile is HF type 1 with the frozen 1000-observation indices | behaviour | trivial | indices re-pinned 4× in tools/bench (session/conformance/graph/rack) — those go | KEEP 5; DELETE `the_numerator_denominator_form_matches_the_per_mille_form` (L65, `per_mille` is literally `nearest_rank(…,1000)`, L30-31) |
| src/timing.rs | 4 | 0 | `timed` panics if the body hashed through `Sha256Sink` (F1) | mutation-proof | trivial | none | KEEP L157; MERGE L138+L145+L163 into one |
| src/json.rs | 4 | 0 | Escaper is RFC 8259 §7 complete | behaviour | trivial | conformance.rs re-pin (deleted) | KEEP 3; MERGE L229 into L224 |
| src/digest.rs | 4 | 0 | `Sha256Sink` counts every update; `sha256_hex` does not | behaviour | trivial | none | KEEP 4 |
| src/alloc.rs | 4 | 0 | Audited allocator installed; counters saturate | behaviour | trivial | `assert_installed()` called by every bench main | KEEP 2; MERGE L295+L305 (same probe). Global `MODE` static (L158) — `default_mode_is_abort` flips if any test calls `set_mode(Count)` |
| src/metadata.rs | 1 | 0 | `gather()` memoized | descriptive | trivial | none | REWRITE: reads live `PATH` behind `if let` (L450, asserts nothing without PATH); memoization is `OnceLock`'s property. The untested F2 claim — `missing()` sorts, `record_fields()` emits `null` + `missing_metadata` (L388-438) — can be pinned by constructing `Metadata { values }` directly |

**Summary bench-support:** 23; keep 16; merge/delete 6; rewrite 1; ≈0 s.

## tools/native-pcm-runner

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| src/lib.rs (mod tests L1380-2557) | 19 | 0 | C-ABI runner renders the frozen five-fixture corpus bit-exactly, refuses every preflight fault before compiling, publishes through a no-clobber owned-inode state machine | digest-pin (1, with embedded regen helper) + 17 behaviour on mock/FS | small (only L1503 touches the real C ABI: 5 compiles of the 16.7 KB nine-track EQ session × 8 quanta) | digests triplicated: lib.rs:1562-1567, `fixtures/native-pcm-runner/v1/generate.py`, `MANIFEST.tsv`; `check-native-pcm-runner.sh` only checks generate.py↔MANIFEST and **string-scrapes lib.rs** for `if !adapter.partial_is_absent() \|\| !adapter.final_is_owned()` — textual shadow of test 17 | KEEP 9; MERGE L1747+L1780+L2064 into one preflight-ordering test, L1840+L1979 into L2099; TRIM L1503 5→3 fixtures (riff-44100, riff-96000, rf64-48000; read pins from MANIFEST.tsv), L1800 8→3 shapes, L2385 4→1 (`shape` is an opaque `&str`), L2476/L2511 drop `rename` rows; DELETE `injected_create_write_and_publish_failures_are_terminal` (L2041 — mock never touches disk, `!output.exists()` vacuous; L2184 covers it with real files) |

Flags: **env read in a `#[test]`**: `MISO_ENGINE_REPIN_NATIVE_PCM_RUNNER` (L1528) turns the render test into a regen helper that deliberately fails — outside the documented `MISO_ENGINE_REPIN_*_CORPUS` family (`docs/ENGINE_ENV_VOCABULARY.md:25`); should be an `#[ignore]` regen test. `#[cfg(test)]` fault branches inside production `write_block`/`finish`/`publish_held` (L638-651, L1002-1018, L1087-1160) mean the shipped `finish` is textually not the tested one. Unix-only calls without `#[cfg(unix)]`. Leaks `/tmp/miso-native-pcm-runner-*` on failure.

**Summary native-pcm-runner:** 19; keep 9; merge 6→2; trim 5; delete 1; ≈1-2 s.

## tools/stem-hasher

| path | tests | ign | claim | kind | cost | redundancy | verdict |
|---|---|---|---|---|---|---|---|
| tests/conformance.rs | 8 | 0 | Library, CLI (raw/wave) and the engine WAVE parser reproduce the six Python-generated identities in `fixtures/stem-identity/v1/VECTORS.tsv` | property/oracle | trivial (10-62 byte inputs) | `generate.py --check` invoked by no script/workflow; `check-stem-store-v1.mjs` tests the JS hasher on ad-hoc bytes — **gap: web and Rust hashers are not cross-pinned** | KEEP; MERGE the 6 per-vector tests (L48-76) into one loop over `parse_vectors` + `assert_eq!(vectors.len(), 6)`; KEEP L79 |
| src/lib.rs | 3 | 0 | CLI grammar closed; raw mode demands exact length; WAVE depth set {16,24,32f} | behaviour | trivial | positive parse overlaps conformance | KEEP 3 |

Flag: `parse_vectors` ignores `samples_by_frame` (field 4), the human-derivable oracle the README advertises — re-deriving `canonical_hex` from it would turn the pins into a real oracle. `write_canonical_file`'s `create_new` no-clobber (L591-614) is untested.

**Summary stem-hasher:** 11; keep 4; merge 7→1; ≈0 s.

## tools/wasm-gate-corpus, wasm-gate-guest, wasm-console-guest, wasm-console
No tests (confirmed by grep for `#[test]`/`#[cfg(test)]` and absence of `tests/`).

---

## Cross-cutting findings

**Where the debug minutes go (est. saved):** tools/audit 200-430 s; graph-compiler 60-100 s; builtins-compiler 45-85 s; wasm-gates 38-81 s; console-workload 30-45 s; builtins 20-35 s; graph 8-15 s; host-core 8-15 s; bench 6-18 s; host-web 3-6 s. Roughly **7-14 CPU-minutes** per workspace debug run, unverified in aggregate.

**Three structural moves cover most of it:** (1) tools/audit: render `generated()` once (memoised) instead of five times and call stage functions for post-oracle mutations — the 65,537-track `resources()` rows and the 1,630-row response grid are currently rendered 5× and independently re-measured 22×; (2) `ci.yml:70` `--exclude wasm-gates` (every G5/G6 claim already runs in release at L481 plus the script's native leg); (3) the two 65,537-track scale tests (graph-compiler, builtins-compiler) plus `allocation_tracker` → nightly/release, with `allocation_tracker` trimmed to `[1,4]` tracks and graph-compiler's to 10,923.

**Source/script string-scrapes to replace:** `tools/audit/src/builtins_graph.rs:845` and `capi.rs:328` (own source, `concat!`-split tokens → `clippy::disallowed_methods`, mechanism already in `clippy.toml`); `builtins_fixture_check.rs:538`; `parameter-metadata/tests/abi_layout.rs` export-set test (scrapes `check-web-audioworklet.sh`); `session-validator/tests/skill.rs` (`.claude/skills`); host-core `track_delay.rs:191` (JSON the helper wrote); the reverse direction — scripts scraping test source: `check-effect-interchange-benchmark-108.sh:37-50`, `check-native-pcm-runner.sh`.

**Env-var reads inside tests:** `builtins/tests/determinism.rs:35` (`MISO_ENGINE_REPIN_BUILTINS_CORPUS`, documented family), `native-pcm-runner/src/lib.rs:1528` (`MISO_ENGINE_REPIN_NATIVE_PCM_RUNNER`, undocumented name), `graph-compiler/src/lib.rs:2852` (`MISO_ENGINE_AUDIT_037` hides a 100k-block render inside a normal test), `bench-support/src/metadata.rs:450` (`PATH`).

**Hidden global state:** builtins-compiler `#[global_allocator]` + `TEST_PHASE_TWO_*` statics linked into every workspace test binary via `graph-compiler/Cargo.toml:28`; host-web `boot_transient_budget.rs` allocator; bench `audit::reset()/snapshot()`; bench-support `MODE`; MXCSR in host-core `fp_environment.rs` (guarded).

**Arch coupling:** `Backend::current()` is a per-arch constant — `tools/bench/src/rack.rs:126` and four `chain_shape.rs` pins fail on aarch64; 19 graph-compiler console tests and builtins-compiler L4313 pass silently/vacuously on scalar hosts.

**Tautologies/descriptive (DELETE):** `tools/audit/src/source.rs`, `builtins/tests/speed.rs`, `graph/src/lib.rs:2450/2463`, `bench/src/conformance.rs`, `bench/src/floor.rs:504`, `bench-support/src/stats.rs:65`, `host-web/src/tests.rs:2904`, `parameter-metadata rendering_is_deterministic`, `session-validator the_report_is_deterministic`, `builtin_automation_targets.rs:94`.

**Stale MUTATIONS.md references:** graph `tests/MUTATIONS.md` M5/M6/M8 and 218-4 name tests that no longer exist in that crate; wasm-gates MUTATIONS.md says 331 comparisons (now 337, unverified).