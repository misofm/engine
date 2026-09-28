<!-- Produced for the 2026-09-28 test-value audit (base a9414c0c) by a helper agent, then spot-checked by the auditor. Paths such as `ci/`, `tva/` or `method/` refer to the auditor's scratch directory, which was deleted after the audit; CI run and job IDs are enough to re-fetch the logs with `gh api`. -->

# Near-duplicate families: DSP and graph group (19 crates, 1,206 tests, HEAD a9414c0c)

A family is 4 or more tests that prove **one** claim at different parameter points: width twins,
partition or rate sweeps, per-entry-point copies, or per-effect copies of one generic claim. There
are **49 families with 305 members** (25% of the group). Every member carries its `family_id` in
`classes.tsv`.

The "distinct branches?" column is judged from the crate's `tests/MUTATIONS.md`. If a recorded
mutation turns some members red and leaves others green, the members are distinct. Where no record
exists, the column says so and the judgement comes from the code the parameter selects. Simd4 and
Simd8 members are separate monomorphisations of one generic body. They count as distinct only when
a record separates them.

## How to read the verdicts

- **Collapse** (no member is distinct): keep the 1-3 representatives, delete the rest.
- **Trim**: some members are distinct. Keep the distinct members and the representatives.
- **Keep**: every member has its own recorded red, or the family is a bug reproducer set. These
  families cost CPU or maintenance, but no member is a copy.

## The families worth acting on

These families have members that no recorded mutation distinguishes. The cut volume is the number
of members that can go with no claim lost.

| id | crate | members | claim | why it collapses | keep | cut |
|---|---|---:|---|---|---|---:|
| MA-01 | math | 8 | fast dB tier within its gate over a named crossing domain (`f1_fast_db_bounds.rs:1019-1115`) | only two distinct computations exist. X1=X3=X5=X7 make the byte-identical call `crossing_worst_db(1.0e-8, 16.0, true)` (f1:1020/1048/1076/1103). X2=X6 make the identical call `(-124, 24, false)` (f1:1034/1089). X4 `(-96,0)` and X8 `(-18,18)` are subsets of X2's domain. Verified | X1 :1019, X2 :1033 | 6 |
| MA-02 | math | 4 | full-domain proof of the fast tier's bound (ignored; run with `--ignored` on math-closure PRs, qualification.yml:549, and nightly) | f1:1127 (X7) runs the same sweep, function and gate as f1:463. f1:1142 (X8) sweeps a strict subset of f1:432's domains | :432, :463 | 2 |
| CO-02 | compressor | 7 | a kernel scenario renders a SHA-256 pinned on the pre-slice base (`kernel.rs:3352-3796`, `ramping_prefix_scenario.rs:294`) | every recorded red on a scenario (981-M1..M4, 982-M1..M3, 983-M1..M4, 984-M1/M2, 985-M2/M3, 995-M1..M6, 1006-M1..M12; compressor MUTATIONS.md:202-335) is also red on gate 1 (the settled-body grid or differentials, CO-01) | :3796, `ramping_prefix_scenario.rs:294` | 5 |
| CF-01 | conformance | 5 | the `.mepcm` parser rejects every corruption with a typed error | `tests/fixtures.rs:22` repeats `fixture_corruption.rs:35-43` on a real fixture. `src/lib.rs:52`'s flip loop repeats `:82`'s exhaustive loop. `src/lib.rs:71` is a subset of `:20` | `fixture_corruption.rs:20`, `:82`, plus the -0.0/NaN preservation asserts of `lib.rs:52` | 2-3 |
| ER-01 | effect-runtime | 5 | block partition moves no ramp-driven output or state | `tests/partition.rs:127/132/137` drive a test-local mock (partition.rs:28-72). Its only production cross-block state is `LinearRamp`, which `tests/ramp.rs:98/:132` prove at the same {1,7,64,128,512} × 3 widths | `ramp.rs:98`, `:132` | 3 |
| BI-01 | builtins | 6 | the prepared TPT HPF/LPF cascade matches the f64 RBJ oracle (`response.rs:134-469`) | all six measure one kernel by six methods, and no MUTATIONS row separates them. MUTATIONS.md:25-37 records that a 0.1% mix error passes every response gate and is caught only by the bit gates `stage.rs:87/:141` | `response.rs:297`, `:469`, `:134` | 3 (the two heaviest, `:356` and `:422`, trim to launch rates at minimum) |
| BI-02 | builtins + builtins-compiler | 6 | the cutoff maximum is admitted and its f32 successor refused | every entry point delegates to `validate_builtin_filter_cutoff` (builtins/src/lib.rs:318, :343, :3217). Only the compiler diagnostic mapping (builtins-compiler lib.rs:12384) is a separate branch | `contract.rs:327`, builtins-compiler `lib.rs:12384` | 3 |
| EQ-03 | parametric-eq | 8 | the stationary/interleaved schedule renders the per-section schedule's bits | seven members are distinct by record (979-M2 at :4176/:4500; 977-M1m only at :4600). `:4551` makes calls byte-identical to three of `:4176`'s (verified: `compare::<L,W>(label, 0, false)` at each width) | :4176, :4562, :4600 | 1 |
| GC-01 | graph-compiler | 8 | each launch effect compiles, binds banks with scalar tails, carries latency/tail/PDC, and one-byte-below caps refuse transactionally (`lib.rs:5681-14021`) | the effect half is distinct: each member reaches a different factory. The ~200-line cap/estimate half is shared. 964-12 turned five members red together and 964-13 turned four red together. No record separates one member's cap arm from another's | all 8 keep their effect half. Only :12561 (limiter) and :13319 (soft-clip) keep the cap/estimate arms | 0 tests, ~6 × 200 lines |
| TL-02 | true-peak-limiter | 4 | a frozen render hashes to its pinned digest (`linked.rs:289`, `seedless.rs:444`, `segments.rs:356`, `determinism.rs:33`) | the 990/1013/1014 reds on linked/segments/seedless always co-occur with gate 1/2/randomized or E1b/E2/E5. round2-11 is red only on the D90 pin, and G5 renders that same pin | `determinism.rs:33` (vacuity half), `linked.rs:289` | 2 |
| ER-02 | effect-runtime | 6 | lane-generic functions are width-independent | the members test distinct functions, but `partition.rs:144` repeats the width claim on the test-local mock | `lane_identity.rs:29`, `envelope.rs:398` | 1 |
| RK-01 | rack | 11 | mono collapse engages and re-engages only when armed, eligible and agreeing | ten members each have their own rack MUTATIONS row (M2-1..9, M3-*). `lib.rs:7017` has no row and is a strict subset of `mono_reengage.rs:263` | all except :7017 | 1 |
| RC-01 | rack-compiler | 5 | the cohort planner's invariants hold on seeded corpora | `:1116` has no row, and `plan_invariants_hold` already runs as a `debug_assert!` inside the planner (rack-compiler lib.rs:358) | :572, :958, :1010, :1070 | 1 |
| LA-01 | lane | 5 | an elided input chain renders the unelided chain's bits, state and report | `:694/:698/:702` are one generic `mixed_reference_cases::<L>` at f32/Simd4/Simd8, and no MUTATIONS row separates them | `:287`, `:702`, `:707` | 2 (or fold the width twins into one test that loops widths) |
| GE-02 | gate-expander | 6 | #738 causal contract: output at n depends on the current detector sample only | `contract.rs:408` asserts only finiteness and zero trips, though its name promises the current detector word. `kernel.rs:259` runs with `coef.bypass = 1.0`, so it asserts identity, not causality | `contract.rs:251`, `:440`, `:518` | 2 (or rewrite them to assert the claim) |

## Families to keep whole

The members are distinct by record, or the family is a bug reproducer set. Several are expensive.
The lever there is sharing setup, such as one compiled 64-track baseline, not deleting tests.

- **Bug reproducer sets (must stay):**
  - GC-05: graph-compiler `bank_levels.rs:881-960`, #966.
  - ER-03: effect-runtime `dynamics.rs:283/345/385/426`, #994.
  - CO-03: compressor `knee_overflow.rs:138-241`, #994.
  - See `bug-reproducers.md`.
- **Distinct by record:**
  - Parametric-eq:
    - EQ-01: 9 randomized differentials. 1005-M5d is red only at W8; 1007-M2 only at Simd4/Simd8.
    - EQ-02: 5 base-bit pins, each the named gate of #976-#979/#999.
    - EQ-04: 7 elision legs.
  - True-peak-limiter:
    - TL-01: 8 silent-fixed-point terms. 182-9, 182-13, 182-14 and 182-16 are each red in exactly one member.
    - TL-03: 4 pre-#990 kernel differentials. 990-M14/M16 are red only in :9162.
    - TL-04: 4 uniform-cohort differentials. 182-5, 182-2 and round2-12/13 are each red in one member.
  - Compressor:
    - CO-01: 6 settled-body differentials. 1006-M1 is red only at Simd4/Simd8.
    - CO-07: 4 mono-collapse tests. M2-C3..C5 are red only at :276; 985-M4 only at :509.
  - Graph and graph-compiler:
    - GR-05: 7 D9 reduction tests. :12989 is the only catcher of 916-4.
    - GR-01: 5 banked source fixtures. 957-2c, 957-3 and 957-4 are each red in one member.
    - GR-03: 6 rt9 tests.
    - GC-02: 12 strip clauses.
    - GC-03: 10 mono-pool rules.
    - GC-06: 7 meter passes, one row each.
  - Builtins and builtins-compiler:
    - BI-03: 5 select-free matrix tests. M2/M4/M5 are red only on counters; M1/M3 only on pins.
    - BI-04: 5 partition tests, one per stage kernel.
    - BI-05: 4 class-A OFF tests. P3-M2 is red only at :156.
    - BC-01: 12 serialized-pair decline predicates.
    - BC-02: 9 error-order sites.
    - BC-03: 6 bind-refusal codes.
    - BC-04: 9 allocation shapes.
  - Rack and rack-compiler:
    - RK-02: 5 constructors. The one merge is :5928 into :5889.
    - RK-03: 8 transpose paths. M1 is red only at :5960; M7 only at :6264.
  - Gate-expander:
    - GE-01: 4 identity arms. Each repeats its arm at 4 rates, and one rate would do.
    - GE-03: 4 initial-record refusals.
- **Plausibly distinct, unrecorded:**
  - MB-01: multiband `split.rs:281-405`. 8 schedule shapes against the test-only `FORCE_RAMPING` arm. No MUTATIONS rows exist for split.rs.
  - ER-04: effect-runtime hoist situations. There is no #144 row.
  - BI-07: builtins symmetry-mask tests. There are no rows, and a `debug_assert_eq!` already cross-checks the walk.
  - CO-04, CO-05, CO-06: compressor partition, identity and silent-path families. Each is partly distinct; the notes are in the table below.

## Cross-crate patterns (no single family id)

- **Per-crate corpus pin compares:** 10 tests. `tools/wasm-gates/tests/g5_native_digests_match_pins`
  re-renders the same pins at all three widths in the release job.
  - builtins `determinism.rs:34`
  - effect-runtime `determinism.rs:39`
  - parametric-eq `determinism.rs:50`
  - compressor `cross_target.rs:60`
  - true-peak-limiter `determinism.rs:33`
  - multiband `cross_target_digest.rs:27`
  - gate-expander `determinism.rs:29`
  - delay `determinism.rs:27`
  - soft-clip `determinism.rs:36`
  - transient-shaper `cross_target.rs:69`

  G5 re-renders these corpora through the owning crate's `run_case` and pin constants
  (`tools/wasm-gate-corpus/src/lib.rs:1059-1070`). math `m3_determinism.rs:142` is the exception:
  its `+fma` CI leg is unique. The paired finiteness and non-vacuity tests are not in G5 and stay.
- **`shared_hex_adapter_matches_literal_bytes`:** 4 copies, in compressor `cross_target.rs:31`,
  true-peak-limiter `determinism.rs:81`, parametric-eq `determinism.rs:27` and transient-shaper
  `cross_target.rs:105`. All four test `bench_support::digest::hex`, which
  `tools/bench-support/src/digest.rs:89` owns.
- **`a_resident_read_is_repeatable_to_the_bit`:** 2 copies, compressor `observation.rs:125` and
  limiter `observation.rs:148`. Both are true by the `&self` signature.
- **Kernel benchmarks inside test binaries:** 12 rows, all flagged `bench`, `out_of_scope`. None
  times a host path.

---

## Full family tables by crate group

The tables below are the per-part detail: members with line numbers, the claim, the evidence for
or against distinct branches, and the representatives.

### graph, graph-compiler

| id | members | claim | distinct branches? | representatives |
|---|---|---|---|---|
| GC-01 | 8: lib.rs:5681 EQ, :6503 compressor, :12338 gate, :12561 limiter, :12957 multiband, :13319 soft-clip, :13683 transient-shaper, :14021 delay | each launch effect compiles in a 9/10-track session, binds banks at the host width with scalar tails, carries its latency/tail/PDC, bank PCM == scalar PCM, and one-byte-below plan caps refuse transactionally | Partly. Each member reaches a different effect factory (bank bind, latency 0/31/486, finite vs infinite tail), which is real. The ~200-line cap/estimate half is duplicated: MUTATIONS 964-12 (graph-compiler/tests/MUTATIONS.md, attempt 2) turned the limiter, multiband, soft-clip, transient-shaper and delay tests red **together**, and 964-13 turned four of them red together; no recorded mutation reds one cap arm and not the others | :12561 (caps + estimate + latency 486), :13319 (finite tail 29, PDC 31), :5681 (bank-vs-scalar across blocks) |
| GC-05 | 6: bank_levels.rs:881, :896, :908, :920, :933, :960 | #966 reproducers: a console whose effect bank would span dependency levels binds at every width, leaves exactly the misaligned slot unbound, and renders the scalar bits | Partly. 966-M1 and 966-M10 red all six; 966-M3 (first/last member only) reds :896 and :920 but leaves :881 and :908 green; 966-M9 reds :933's armed leg only. No recorded mutation separates :881 (lane 0) from :908 (last lane) | :896 (middle lane, only M3 catcher among lane variants), :933 (mono, armed collapse), :920 (seed-412 soft-clip shape). All six are bug reproducers; keep, but :881/:908 could share one compile |
| GR-05 | 7: runtime.rs:9026, :9171, :9271, :9294, :10502, :10611, :12989 | the block reduction is D9 left-to-right, bit for bit (vs scalar reference, frozen pre-change kernels, host-plane variant) | Mostly yes. :10502 is the D9 owner (M1/M2/M15 red); :12989 is the only catcher of 916-4 (host-plane `reduce_plane_into`); :9271 is the absolute -0.0 property; :9171 guards the #898 group hoisting with random fan-ins. :10611 is a disclosed equivalent-mutant (perf property, MUTATIONS "Disclosed equivalent mutants"); :9026 and :9294 compare against the same frozen `old_reduce_plane` (runtime.rs:8872) that :10502 also uses | :10502, :12989, :9171 |
| GC-02 (cost family, not near-duplicates) | 12: lib.rs:8043, :8149, :9588, :9698, :9839, :9930, :10000, :10069, :10118, :11303, :11612, :12203 | on the 64-track intended strip, one clause of chain-merge / scatter-redirect / route-fold is taken or declined exactly where its session shape says, and PCM (and meter frames) equal the scalar or declined oracle | Yes: each names a distinct clause and red mutation in its doc; 218-3 reds :11303, :8043, :9839 only. Listed because each compiles and renders the 64-track console at least twice (bank + scalar oracle), which is where graph-compiler's debug time goes | none to cut; share one compiled 64-track baseline across the family |
| GC-03 (cost family) | 10: lib.rs:8329, :8550, :8713, :8762, :8820, :8878, :8992, :9040, :9269, :9354 | #971/#1001/#1002 mono-pool re-plan rule: a stranded mono remainder moves to the stereo pool only when the move binds strictly more banks, and a failed re-plan leaves nothing behind | Yes: each is a separate clause or Sol-review counterexample (doc comments cite #1001, #1002, "Sol's #971 attempt-2 session") | none to cut |
| GC-06 (cost family) | 7: lib.rs:5358, :5431, :10243, :10553, :10715, :10918, :11058 | #943/#950 banked meter passes publish the declined arm's frames, run once per cohort, and are realtime-clean | Yes: graph MUTATIONS #943 rows G-1..G-3/S-7/O-1/O-2 and #950 rows G-1..G-5/A-5/K-1 each red a different member | :10243, :10553, :5431 |
| GR-01 (cost family) | 5: runtime.rs:14158, :14488, :14512, :14541, :14645 | banked source fixture shapes render the pre-change executor's bits (pinned digests) with the right dispatch/mode tables | Yes: 957-2c reds only :14512, 957-3 only :14541, 957-4/5b only :14645 (graph MUTATIONS #957) | all |
| GR-03 | 6: rt9_resident_bank_input_alloc.rs:779, :901, :1081, :1188, :1251, :1335 | resident bank input / controlled observation sets behave as the old acquisition and fixed-bank baseline | Yes, separate clauses (order, fallback, failure, stall); :779 is itself a 960-config sweep | :779 (could trim frames {1, W-1, W, W+1, 17} to {1, W+1, 17}) |

Not a family (3 members, below the >=4 bar) but near-copies: lib.rs:6725 / :6849 / :6970
`mixed_causal_{compressor,multiband,gate}_and_fixed_latency_limiter_keep_parallel_pdc_aligned`
differ only in which zero-latency causal effect runs in parallel with the 486-sample limiter.


### builtins, builtins-compiler, rack, rack-compiler

| id | members | claim | distinct branches? (evidence) | representatives |
|---|---|---|---|---|
| BI-01 | 6: response.rs:134, :225, :297, :356, :422, :469 | The prepared TPT HPF/LPF cascade matches the f64 RBJ oracle, at launch rates plus the extended tier. | No. All six measure one kernel (`SvfSection::design` + `svf_step`) by different methods: impulse, magnitude sweep, state space, DFT, sustained sine and cascade. No MUTATIONS row separates them. MUTATIONS.md:25-37 records that M4 (0.1 % mix error) passes every response gate and is caught only by the bit gates stage.rs:87/:141 and the corpus. | :297 (state space built from all 7 words, cheapest and strongest), :469 (production-order cascade, launch rates), :134 (PCM impulse at 2e-5) |
| BI-02 | 6: contract.rs:327, filter_liveness.rs:169, response.rs:880, filter_response.rs:552, :706, builtins-compiler lib.rs:12384 | The cutoff maximum is admitted and its f32 successor refused at each launch rate. | Mostly no. Every entry point (the descriptor `domain.contains`, `BuiltinChain::new`, `prepare_input_filter_pair`, the response query) delegates to `validate_builtin_filter_cutoff` (builtins/src/lib.rs:318, :343, :3217). Only the compiler diagnostic mapping (:12384) is a separate branch. The max bit literals (0x46ac_42f7…) are repeated in 3 files. | contract.rs:327, builtins-compiler lib.rs:12384; keep response.rs:880 only for its -3 dB design claim, with the walk trimmed |
| BI-03 | 5: matrix.rs:376, :534, :659, src/tests.rs:873, :1039 | The select-free matrix and fused fader/matrix arms (#944, #954) are taken exactly when no lane is the identity, and they render the base bits. | Yes. MUTATIONS.md:226-236 and :287-299: M2, M4 and M5 turn only the counter witnesses (:873, :1039) red, and M1 and M3 turn only the SHA pins red. The pins are B (frozen digest of the pre-change base). | :873, :1039, plus one digest (:534) |
| BI-04 | 5: stage.rs:425, fader_ramp.rs:285, input_liveness.rs:333, filter_liveness.rs:223, meter.rs:243 | Block partition changes no output bit or state. | Yes. Each test drives a different stage kernel (input chain / matrix ramp M7, fader ramp, trim ramp P3-M4, filter ramp, meter segment M10). | all (each is its stage's only partition gate) |
| BI-05 | 4: fader_ramp.rs:51, input_liveness.rs:99, :156, :204 | A live stage with no command in flight is bit-identical to the prepared stage (class-A OFF). | Yes. 140-7 (fader), P3-M1 (:99) and P3-M2 (:156 only; "no digest can see this") | all |
| BI-07 | 5: src/tests.rs:114, :137, :248, :288, :417 | The cached post-ramp channel-symmetry mask equals the per-lane walk (`compute_lane_channel_symmetry`). | Partly. :248 pins a work count (78 extractions) and :288 a work witness. There is no MUTATIONS row. `lane_channel_symmetry` also carries a `debug_assert_eq!` against the walk (MUTATIONS.md:192-197). | :137, :417 |
| BC-01 | 12: lib.rs:7478, :7764, :7828, :7947, :9130, :9197, :9269, :9361, :9579, :9627, :9717, :9792 | A serialized scalar fader/matrix pair is selected or declined correctly, and it renders exactly the separate-owner twin. | Yes. Each test exercises a different decline predicate (extra reader, sidechain, send reader, post-fader meter, alias observer, staggered observer, nonadjacent, overlapping, output track; graph 916-16 for :7947). | all; the family's cost is maintenance (~1,300 lines), not CPU |
| BC-02 | 9: lib.rs:8072, :8239, :8444, :8779, :9038, :10185, :10431, :10554, :10621 | A failed or invalid render keeps the original owners' order, boundaries and queues. | Yes, distinct error sites (observer error, post-fader materialize, ramp retarget retry, matrix prefix, envelope). No MUTATIONS rows are recorded for these. | all, or keep 4 if forced: :8072, :8444, :10185, :10554 |
| BC-03 | 6: lib.rs:5347, :5372, :5467, :5495, :5540, :5584 | A source-set bind refusal returns every owned input for retry. | Yes, different refusal codes and paths. No rows are recorded. | :5372, :5584 |
| BC-04 | 9: allocation_tracker.rs:166…:1290 | The builtin graph paths allocate or free nothing on render, and preparation retains exactly the charged layouts. | Yes, distinct graph shapes (composite, queued, scalar, split, bank slot owners, phase-two report). | all; trim :1174's 65,537-track row |
| RK-01 | 11: rack lib.rs:6729, :6764, :6813, :6872, :6932, :6978, :7017, mono_reengage.rs:263, :338, :403, :489 | Mono collapse engages and re-engages only when armed, eligible and agreeing, and renders the never-collapsed bits. | Yes. One row per member: M2-1, M2-2, M2-3/4, M2-8 (two members), M2-9, M3-1/3/4/5 (rack MUTATIONS.md:170-236). Exception: :7017 has no row and is a strict subset of mono_reengage.rs:263. | all except :7017 |
| RK-02 | 5: rack lib.rs:3245, :5653, :5690, :5889, :5928 | Fold and auxiliary constructors and arming refuse invalid masks and shapes. | Yes, but narrowly: four distinct constructors. :5889 and :5928 are the same shape of test on `arm_fold` and `arm_aux`. | :5690, :5889 (merge :5928 into it) |
| RK-03 | 8: rack lib.rs:3412, :4684, :4708, :4760, :4789, :4840, :5960, :6264 | Gather and scatter are a bit-exact permutation on every transpose path. | Yes, by design each covers a different path: tiled full bank, per-lane partial (M1 at :5960), inactive lanes (M7 at :6264), direct views, staged fallback, RT9 resident. | :4840, :5960, :6264 |
| RC-01 | 5: rack-compiler lib.rs:572, :958, :1010, :1070, :1116 | The cohort planner's invariants hold on seeded random corpora. | Yes, except :1116. Rows: P1 (:572), P5 (:958), P9 (:1010), P6 (:1070). :1116 has no row, and `plan_invariants_hold` is already a `debug_assert!` inside the planner (rack-compiler lib.rs:358). | :572, :958, :1010, :1070 |


### parametric-eq, effect-runtime

| id | members | claim | distinct branches? | representatives |
|---|---|---|---|---|
| EQ-01 | 9: `src/lib.rs:7731,7736,7741` (#1005 list vs batch-head), `:7748,7753,7758` (#1007 select writes on the list vs `lane_set`), `:7764,7769,7774` (#1007 writes alone on the batch-head path) | Randomized contract-level differential: a ramping block renders the reference path's bits (outputs, reports, payloads, internal words). The bodies are one-line calls to `differential::<L,W>(…, paths)`, which asserts in `scenario` (40 seeded scenarios × 96 blocks per width and per dual/mono in debug, 300 in release). | Yes, by width. MUTATIONS.md 1005-M5d is red only at W8 in dev. 1007-M2 is red at Simd4 and Simd8 only, not scalar. 1005-M1 is red at all widths. The three path pairs are three distinct claims (#1005 list, #1007 on list, #1007 alone). | simd8 of each triple (`:7741`, `:7758`, `:7774`); add scalar `:7731` if one scalar leg is wanted. Width legs could merge into one test per claim. |
| EQ-02 | 5 in `tests/bank.rs`: `:1118` odd_live, `:1642` admitted_blocks…without_selects, `:1962` two_and_four_live, `:2327` a_cut_switched_off, `:2819` over_the_block_limit | "Optimisation X renders the pre-change base bits". Each leg (scalar, bank, bank-mono) is folded into one SHA-256 of seeded hostile input, pinned on the unmodified base. Legs are compared only to their own pin, not to each other. | Yes. Each is the named gate of a different issue (#976, #977, #978, #979, #999). Mutations: 976-M2 is red only via the odd_live digests. 979-M4 is red via the cliff digests. 999-M3 is red in all five. 977-M1/M4 are red via the select digests. | All five are mutation-distinct. They are B-class pins with no second owner (G5 does not replay them). |
| EQ-03 | 8 in `src/lib.rs` `interleave_identity`: `:4176,4390,4464,4500,4551,4562,4600,4624` | The stationary/interleaved six-section schedule renders the per-section (or four-section, production `svf_block`) schedule's bits. | Partly. 979-M2 is red at `:4176` and `:4500`. 977-M1m is red only at `:4600`. `:4551` is byte-for-byte a subset of `:4176`: the same `compare(case 0, seeded=false)` at the same three widths (lib.rs:4115 helper). | `:4176`, `:4562`, `:4600` |
| EQ-04 | 7 in `src/lib.rs` `elision`: `:4958,4989,5019,5055,5098,5282,5587` | The elision gate admits or refuses per leg: (a) input `-0.0`, non-finite or >1e30; (b) non-inert dead state; (c) live state. | Yes, per leg. 980-M1 is red at `:5019`. 977-M4 is red at `:5587`. 979-M1..M4 are red at `:5098`. `:4958` is non-vacuity. | `:5019`, `:5098`, `:5587` |
| ER-01 | 5: `tests/ramp.rs:98`, `:132`, `tests/partition.rs:127,132,137` | Block partition does not move ramp-driven output or state. | ramp.rs:98 and :132 exercise production `LinearRamp::advance_block` at partitions {1,7,64,128,512} × 3 widths (rows 1, 2 red). The partition.rs trio drives a test-local mock whose only production cross-block state is the same `LinearRamp`. The mock carries the envelope itself (partition.rs:47-60), so the trio adds no production branch. | `ramp.rs:98`, `ramp.rs:132` |
| ER-02 | 6: `tests/lane_identity.rs:29,54,85,104`, `tests/envelope.rs:398`, `tests/partition.rs:144` | Lane-generic functions are width-independent at W1/4/8. | Distinct functions, not parameter points. Row 13 (corpus loop width-dependent) is red only at `:29`. `partition.rs:144` repeats the width claim on the mock. `envelope.rs:398` is the only width proof for `ar_one_pole_step`. | `lane_identity.rs:29`, `envelope.rs:398` |
| ER-03 | 4: `tests/dynamics.rs:283,345,385,426` | #994: a knee whose `1/(2W)` overflows is a hard knee, and only those widths move. | Partly. `:283` is red under all three recorded mutations (994-R1, R2, R3). `:385` is red only under R2. `:345` and `:426` are red under R1 and R3, which `:283` also catches. | `:283` (+`:385`). All four are #994 reproducers and must stay (see below). |
| ER-04 | 5: `tests/stationary_hoist.rs:71,145,182,213,251` | A hoisted redundant retarget renders the hand-written unhoisted arm's bits. | Different situations (plain, subnormal, mid-ramp, block-boundary, mid-block split). There is no MUTATIONS row for #144 in this crate, so discrimination is unrecorded. | `:71`, `:182`, `:251` |


### compressor, true-peak-limiter, multiband-compressor

| id | members | claim | distinct branches? (evidence) | representatives |
|---|---|---|---|---|
| CO-01 | 6: compressor src/kernel.rs:2903, :2927, :2957, :3215, :3220, :3225 | the settled/two-pass kernel body renders exactly what the verbatim pre-#981 body (kept in `settled_body_tests::reference`) renders: output, recursive, coefficient and ramp words, masks | Partly. MUTATIONS.md:222 982-M2 reds the corpus-table grid and the simd4/simd8 differentials only; :293 995-M5 reds the all-wet grid and simd4/simd8 only; :326 1006-M1 reds only the simd4/simd8 differentials (plus scenarios) and "cannot fail at f32" (:343-346). No row names the f32 differential alone. Simd4/Simd8 are separate monomorphisations of one generic body. | :2903, :2927, :3225 |
| CO-02 | 7: compressor src/kernel.rs:3352, :3411, :3461, :3503, :3578, :3796; tests/ramping_prefix_scenario.rs:294 | a scenario render hashes to a SHA-256 pinned on the unmodified base before each slice landed | No. Every recorded red on a scenario (981-M1..M4 :202-205, 982-M1..M3 :221-223, 983-M1..M4 :236-239, 984-M1/M2 :240-241, 985-M2/M3 :257-258, 995-M1..M6 :289-294, 1006-M1..M12 :326-335) is also red in gate 1 (grid or differentials). The pins' residual value is catching a co-edit of `reference` and production. | :294 (bank contract, payload), :3796 |
| CO-03 | 5: compressor tests/knee_overflow.rs:138, :151, :185, :213, :241 | an overflowing knee width (1/(2W) not finite) renders exactly as the hard knee: no duck at the threshold | Yes: one entry point each (prepare, preservation, automation, restore, bank). MUTATIONS.md:358 994-C1 red 4 of 5; :359 994-C2 red only :138 and :241; :151 never red by design (:361). | :138, :213, :241 |
| CO-04 | 4: compressor tests/partition.rs:59, :111, :185; tests/ramps.rs:538 | PCM and state are identical at every block partition | Partly: 995-M2 (:290) reds only :185 (+gate 1); 981-M2/983-M2 (:203, :237) red :59 and :111; :538 has no recorded red and is a partition-64 case of :59. | :59, :111, :185 |
| CO-05 | 4: compressor tests/identity.rs:25, :40, :69, :114 | an identity configuration (bypass, mix 0, mix 1, unity gain stage) renders exact dry/wet bits | Partly: row 9 and row 3 (MUTATIONS.md:45, :39) are red in `identity` (mix/bypass selects); :114's term is equivalent (row 21 GREEN, :66); :25's 982-M1 is also red in 8 other tests (:221). | :40, :69 |
| CO-06 | 5: compressor tests/silent_fixed_point.rs:144, :167, :194, :214, :239 | the silent fast path renders exactly the never-fast-path bank | Partly: legs differ (settled, non-vacuity, releasing recursive word, boundary, -0.0 input), but MUTATIONS.md:20-23 records that the sign-mask mutation stays GREEN in :239 (EQ owns the gate); the pre-#737 per-test rows are superseded. | :144 (with :167 merged), :194 |
| CO-07 | 4: compressor tests/mono_collapse.rs:175, :276, :362, :509 | a collapsed bank renders the dual bank's bits (and disengage copies restore state) | Yes: M2-C3..C5 (:165-167) red only :276; 985-M4 (:259) only :509 (+gate 1); 985-M3 (:258) reds four mono tests. | :175, :276, :509 |
| MB-01 | 8: multiband src/split.rs:281, :289, :306, :323, :343, :373, :387, :405 | the ramping split renders the same bytes and fingerprint as `FORCE_RAMPING` (test-only const A/B arm, false on every production path, src/lib.rs:957) at f32/Simd4/Simd8 | Unverified: no MUTATIONS rows exist for split.rs. Each scenario aims at a different `plan_segment` shape (flat, mid-block arrival, boundary arrival, in-flight, overlapping, restatement, restatement mid-flight, partition), so plausibly distinct; all cheap (≤6 blocks). | :289, :323, :387 |
| TL-01 | 8: limiter src/lib.rs:6695, :6746, :6789, :6825, :6930, :6983, :7085, :7178 | the earned silent fixed point is admitted and withdrawn exactly when bit-identity allows | Yes: MUTATIONS 182-9 (stale history), 182-13 (de-zipper), 182-14 (restore), 182-16 (bank path) are each red in exactly one member ("and nothing else"). | :6695, :6930, :7085 |
| TL-02 | 4: limiter tests/linked.rs:289, tests/seedless.rs:444, tests/segments.rs:356, tests/determinism.rs:33 | a frozen render hashes to its pinned digest | Mostly no: 990/1014/1013 reds on linked/segments/seedless always co-occur with gate 1/2/randomized or E1b/E2/E5. Exception: round2-11 (alignment tap 0 for 6) is recorded red only on the D90 pin (determinism.rs:33), which G5 also renders. | :33 (vacuity half), :289 |
| TL-03 | 4: limiter src/lib.rs:8376, :8519, :8780, :9162 | the linked / stationary-walk body renders exactly the pre-#990 kernel kept in the test module | Yes: 990-M14 and 990-M16 red only in :9162 (gate 2); 1014-M5 reds :8519 but not :8376; 990-M11/M13/M15/M17 red in randomized + gate 2 only. | all 4 |
| TL-04 | 4: limiter src/lib.rs:6142, :6170, :6213, :6284 | the uniform-cohort vector path equals the per-lane path | Yes: 182-5 only :6142; 182-2 only :6284; round2-12/13 only :6213; 182-1/3/4 on :6170. | all 4 |

Not a family (3 members): limiter src/lib.rs:5443/:5448/:5453 `detector_chunk_active_window_matches_old_shape_{scalar,w4,w8}`, one generic body per width, no MUTATIONS rows; could be one test looping widths at no loss.
Cross-crate family for the parent: `shared_hex_adapter_matches_literal_bytes` exists in compressor tests/cross_target.rs:31, true-peak-limiter tests/determinism.rs:81, parametric-eq tests/determinism.rs and transient-shaper tests/cross_target.rs (4 copies testing `bench_support::digest::hex`, owned by tools/bench-support/src/digest.rs:89).


### lane, math, dsp-reference, conformance, gate-expander, soft-clip, transient-shaper, delay

| id | members | claim | distinct branches? | representatives |
|---|---|---|---|---|
| MA-01 | 8: math/tests/f1_fast_db_bounds.rs:1019, 1033, 1047, 1061, 1075, 1088, 1102, 1115 (X1..X8) | fast dB tier within its gate over a named crossing's domain | No. Every member calls `crossing_worst_db` (f1:969) with one of four argument sets: X1 = X3 = X5 = X7 = `(1e-8, 16, level)` byte-identical; X2 = X6 = `(-124, 24, gain)`; X4 `(-96, 0)` and X8 `(-18, 18)` are subsets of X2's domain at the same stride rule. Only two distinct computations exist. | X1 (:1019), X2 (:1033) |
| MA-02 | 4: f1:432 gain exhaustive, f1:463 level exhaustive, f1:1127 X7 exhaustive, f1:1142 X8 exhaustive | full-domain proof of the fast tier's bound (all `#[ignore]`, run by `--ignored` on math-closure PRs and nightly) | No for X7/X8. :1127 sweeps `level_domain()` with `level_error_db` against `LEVEL_MAX_DB`, identical to :463 (which also pins decreasing steps); :1142 sweeps [-18, 0] and [0, 18], strict subsets of :432's [-160, 0] and [0, 24]. | :432, :463 |
| LA-01 | 5: lane/tests/input_chain_elision.rs:287, 694, 698, 702, 707 | an elided input chain renders the unelided chain's bits, state and report | Partly. :287 exercises `input_chain_block_elided`; :694/:698/:702 are one generic `mixed_reference_cases::<L>` (mixed_chain_block and _mono) monomorphised at f32/Simd4/Simd8; :707 exercises the trim-ramp wrappers. The three width tests share one body; no MUTATIONS row separates them (the file has no MUTATIONS rows at all). | :287, :702, :707 |
| GE-01 | 4: gate-expander/tests/contract.rs:204, 221, 236; identity.rs:172 | an identity configuration (bypass, ratio 1, range 0) renders input bit-exactly | Yes. Bypass, the ratio-1 curve and the range-0 clamp are distinct code arms; :172 adds the -0.0 and state-advance half. Each of :204/:221/:236 repeats its arm at 4 rates, and the rate changes only coefficients. | :204, :221, :236 (trim each to one rate) |
| GE-02 | 6: gate-expander/tests/contract.rs:251, 298, 408, 440, 518; src/kernel.rs:259 | #738 causal contract: output at sample n depends on the current detector sample and not on later ones | Partly. Unconnected (:251, :298) vs connected sidechain (:408, :440, :518) are distinct paths. :408 asserts only finiteness and zero trips (its name promises the current detector word); :259 runs with `coef.bypass = 1.0`, so it asserts identity, not causality. The current MUTATIONS "causal contract summary" (MUTATIONS.md:269-279) names the file, not individual tests. | :251, :440, :518 |
| GE-03 | 4: gate-expander/tests/contract.rs:130, 153, 169, 185 | prepare/bind refuses malformed initial parameter records with a typed code | Yes. Extra ID, unknown bank member, lane interleave/non-normal value, and missing/misordered are separate validation checks. Keep; a merge would only reduce fn count. | all four, or :169 + :185 |
| CF-01 | 5: conformance/tests/fixture_corruption.rs:20, 82; tests/fixtures.rs:22; src/lib.rs:52, 71 | `.mepcm` parser rejects every corruption with a typed error and preserves payload bits | No for the copies. fixtures.rs:22 repeats fixture_corruption.rs:35-43 on a real fixture; lib.rs:52's flip loop repeats :82's exhaustive loop; lib.rs:71 is a subset of :20. lib.rs:52's -0.0/NaN preservation half is unique. | fixture_corruption.rs:20, :82 (+ lib.rs:52 bit-preservation asserts) |

Not families (each member proves a different kernel or law): lane fader_matrix.rs (5 distinct kernels), g2_kernel_identity.rs (each has its own recorded reds: 978-M*, 999-M*, rows 4/11/12), g1 per-op truth tables, dsp-reference gate_expander.rs (rows 1-3 each hit one test).

Cross-crate pattern (no family id, one per crate): corpus digest pins duplicated by `tools/wasm-gates` G5 `g5_native_digests_match_pins`: delay determinism.rs:27, gate-expander determinism.rs:29, soft-clip determinism.rs:36, transient-shaper cross_target.rs:69 (and math m3_determinism.rs:142, whose `+fma` CI leg is unique). The paired vacuity tests (delay :53, gate :68, soft-clip :62, transient cross_target:29) are not in G5 and should stay.

