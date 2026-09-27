# Red-mutation log — audit #99 (`graph-compiler`, plan lowering and cohorts)

Every gate this job lands was proven non-vacuous by applying the mutation below, running the named
command, observing the failure recorded here, and reverting. A gate with no red mutation is not
evidence; a mutation that stays green is recorded as such and the gate is strengthened until it
does not (see M-05, which was green on first attempt and forced a new fixture).

Delivery host: x86_64 with AVX2+FMA (`x86-64-v3`), rustc 1.97.1.

---

## F4 — deterministic route gain

### M-01 — restore the platform `powf`
* Mutation: in `route_transform`, replace `math::db_to_gain_f32(gain_db)` with
  `10_f64.powf(f64::from(gain_db) / 20.0) as f32`.
* Command: `cargo test -p graph-compiler --lib route_transform_uses`
* Red: `route_transform_uses_the_canonical_db_to_gain_conversion` —
  `assertion left == right failed / left: [1038469653] / right: [1038469654]`
  (`0x3de5_ca15` vs the canonical `0x3de5_ca16` at -19 dB).

### M-02 — perturb the conversion argument
* Mutation: in `tests/route_gain.rs`, call `db_to_gain_f32(db * (1.0 + 1e-6))`.
* Command: `cargo test -p graph-compiler --test route_gain`
* Red: `route_gain_matches_f64_oracle_within_two_ulp` —
  `db_to_gain_f32 deviates 43 ulp from the f64 oracle at 23.75 dB, inside the +/-24 dB mixing range`.

### M-03 — re-add a platform transcendental to the crate
* Mutation: same source edit as M-01.
* Command (historical): `bash scripts/check-math-policy.sh .`, retired in favour of
  `clippy.toml`'s `disallowed-methods` -- `cargo clippy -p graph-compiler` now catches the same
  mutation as a compile error, `use of a disallowed method`.
* Red: exit 1 —
  `math policy failure: platform transcendental calls outside crates/math; call math instead (D6)`.

---

## F6 — dispatch is a compile input

### M-04 — compile reads the host CPU again
* Mutation: `let rack_cohorts = rack_cohort_report(&effects, KernelDispatch::select(engine::target_capabilities()));`
* Command: `cargo test -p graph-compiler --lib scalar_dispatch`
* Red: `scalar_dispatch_compiles_without_banks_on_any_host` —
  `assertion left == right failed / left: 1 / right: 0` (`prepared_bank_count()` under the scalar
  dispatch).

---

## F2 — the lowering pass (`graph::program`)

### M-05 — count taps as readers  *(green on first attempt; gate strengthened)*
* Mutation: in `lower`, delete the `if elided[destination] { continue; }` guard in the reader
  count, so an edge into an elided stage counts as a consumption of its producer's buffer.
* Command: `cargo test -p graph -- program::`
* **First attempt: GREEN.** The effect-free chain fixture roots every alias chain at the
  bank-eligible builtin stage, whose buffer is never consumed in place for an unrelated reason, so
  miscounting taps there changes nothing. Recorded rather than accepted: a fixture with a
  *dynamic-rack* effect (not bank-eligible) was added,
  `taps_are_not_readers_so_an_alias_chain_still_folds_into_its_producer`.
* Red, after: `an alias chain with one real reader must fold into its producer's buffer`.

### M-06 — drop the dedicated-buffer rule
* Mutation: `const fn is_dedicated(_node: &GraphNodeId) -> bool { false }`.
* Command: `cargo test -p graph -- program::`
* Red: `chain_of_seven_stages_lowers_to_six_ops_three_taps_and_two_buffers` —
  `left: 1 / right: 2` (the arena collapses onto a bank member's storage).

### M-07 — never consume a buffer in place
* Mutation: `let in_place = false && single && !dedicated && { ... };`
* Command: `cargo test -p graph -- program::`
* Red: two tests. `chain_of_seven_stages_...` — `left: 3 / right: 2`; and
  `taps_are_not_readers_...` — `an alias chain with one real reader must fold into its producer's
  buffer`.

### M-08 — free a buffer one op too early
* Mutation: in the colouring sweep, `for buffer in expire[op_index].drain(..)` instead of
  `expire[op_index - 1]`.
* Command: `cargo test -p graph -- program::`
* Red: `delayed_edge_gets_staging_buffer_and_blocks_in_place` — the PDC staging buffer collides
  with a buffer whose last read is the current op.

### M-09 — return dedicated buffers to the free list  *(green on first attempt; gate strengthened)*
* Mutation: drop the `if !lifetimes[buffer].dedicated` guard, so a bank-eligible node's buffer
  re-enters the arena when its last *reader* has run.
* Command: `cargo test -p graph -- program::`
* **First attempt: GREEN.** The symbolic interpreter evaluates one op at a time, and a bank is
  not one op: nothing in a per-op dataflow comparison can see that a bank keeps every member's
  output live from the first gather to the last scatter. Recorded rather than accepted.
* First strengthening attempt was itself **wrong** and failed on unmutated code: asserting that a
  dedicated buffer is never shared with any other op forbids *inheriting* storage a dead buffer
  used earlier, which is legal and is what keeps the arena small. The invariant lowering actually
  owes is forward-only: once a bank-eligible node has written its buffer, no later op may write
  it or stage PDC into it. That is what the property test now asserts.
* Red, after: `graph 0: op 9 writes buffer BufferRef(5), held by a bank-eligible node since op 7`.

### M-10 — allow in-place onto a bank member's buffer
* Mutation: `reads_of[owner] == 1` without `&& !lifetimes[buffer].dedicated`.
* Command: `cargo test -p graph -- program::`
* Red: two tests — `chain_of_seven_stages_...` and `taps_are_not_readers_...`.

---

## F1 — the wave-0 level-order property (gated here, not assumed)

`#122`/`#123` landed the fix; the property test the plan specified for it did not land with it, so
it lands here. `direct-route` cannot see this bug: it is a chain with one node per level.

### M-11 — emit levels in Kahn pop order
* Mutation: in `topo`, replace the per-level `nodes.sort()` with a no-op.
* Command: `cargo test -p graph-compiler --lib random_dags`
* Red: `random_dags_have_strictly_ascending_levels_and_level_major_schedule` —
  `graph 0: level 1 is not strictly ascending` (which is exactly what
  graph binding rejects with `graph.scheduler.layout`).

### M-12 — level = max(predecessor) instead of max(predecessor) + 1
* Mutation: `.map_or(0, |value| value)` in `topo`'s level computation.
* Command: `cargo test -p graph-compiler --lib random_dags`
* Red: `graph 0: level of Submix { submix_id: StableGraphId("n03") }` — the in-test longest-path
  recomputation disagrees.

---

## F5 — evidence and allocation off the compile path

### M-13 — wrong token length in `node_text_len`
* Mutation: `"route".len()` instead of `"route:".len()` in the `Route` arm.
* Command: `cargo test -p graph-compiler --lib node_text_len`
* Red: `node_text_len disagrees for Route { route_id: StableGraphId("bbb") }`.

### M-14 — drop a separator in the `Effect` arm
* Mutation: remove one `+ 1` between `rack_token` and the effect id.
* Command: `cargo test -p graph-compiler --lib node_text_len`
* Red: `node_text_len disagrees for Effect(EffectNodeId { .., rack: Simd1, .. })`.

---

## F3 — one cohort former, over whole rack chains

### M-15 — build chain slots from `entries` order instead of session order
* Mutation: sort each track's declared rack effects by effect id before reading their program keys.
* Command: `cargo test -p graph-compiler --lib -- multi_slot chains_of_different bank_membership_is_independent`
* Red: all three chain tests. The fixtures name slot 0 `chain1` and slot 1 `chain0` deliberately,
  so session order and `EffectPreparedSession::entries` order (sorted by effect id) disagree --
  the exact trap #96's crate doc calls out for #99. Without that naming the mutation is invisible.

### M-16 — bind a slot even when some lane skips it
* Mutation: drop the `group.active_slots.iter().all(|lane| lane[slot])` guard.
* Command: as above.
* Red: `chains_of_different_depths_share_a_cohort_through_identity_slots` — slot 1 binds with half
  its lanes inactive, which the effect contract cannot express until #95 adds the per-lane mask.

### M-17 — bucket every chain at level 0 instead of its first slot's level
* Mutation: `candidates_by_level.entry(0)`.
* Command: `cargo test -p graph-compiler --lib`
* Red: `mixed_twelve_track_plan_binds_renders_full_banks_and_scalar_tails_without_graph_changes` --
  the level-uniformity assertion, which now checks slot `k` of a chain sits at `level + k`.

### M-18 — the compile path stops lowering
* Mutation: make `PreparedGraphPlan::new` store `None` instead of the lowered program.
* Command: `cargo test -p graph-compiler --lib compiled_plans_always_lower`
* Red: `direct route: compiled plan must lower`.

### M-19 — the arena bound is claimed against the wrong baseline  *(a mistake I made, recorded)*
* Not a mutation: the first version of `compiled_plans_always_lower_to_a_smaller_executable_program`
  asserted `program.buffers <= buffer_assignments.max() + 1` and **failed on unmutated code**
  (`reverse submixes: arena 4 exceeds the 3 coloured outputs`). The colouring counts node outputs
  only; `GraphExecutor` additionally allocates one contribution buffer per edge and re-buffers
  every bank member, which is what `audio_buffer_samples` already said
  (`colored_outputs + logical_edges`). The program legitimately keeps a dedicated buffer where the
  colouring shared one and the executor un-shared it again at bind time. The assertion now compares
  against the executor's real model, and the reason is written at the assertion.

## Issue #143 P3 — the binding

Every row applied to the working tree, the named binary run, the result recorded, the tree
restored. Host: `x86_64` (AMD Ryzen 7 9700X, Zen 5), `-C target-feature=+avx2,+fma`, debug.

The tests live in `host-core/tests/effect_observation.rs`, which is the only place the
whole seam exists: a real session, compiled by the graph compiler, with a real homogeneous bank and
a real per-node scalar instance.

| # | eval | mutation | file | result |
|---|---|---|---|---|
| 143-E1 | digest identity per tap | make the peak fold perturb the value by `1e-30` on the first block of a window — i.e. let the observation touch anything the block computes | `effect-contract/src/live.rs` | RED — `lane 0 (threshold 0) published its own reduction`, `228737632` vs `0`; 2 of 5 fail |
| 143-E5 | zero binding, zero cost | attach lanes whenever the descriptor declares a tap regardless of the request | `host-core/src/prepare.rs` | RED — `a_session_that_asked_for_no_observation_holds_none`. **The output stayed identical**, which is the point: only the structural walk catches it |
| 143-E2 | bank-lane correctness | the bank publishes `samples[0]` into every lane | `rack/src/lib.rs` | RED — 3 of 5 fail, including the bit-exact comparison against an independently prepared scalar compressor at each lane's own threshold |
| 143-E3-bank | window exactness | publish **before** `process_bank` | `rack/src/lib.rs` | RED |
| 143-E3-scalar | window exactness | publish **before** `process` in `execute_op`'s `ConsoleEffect` arm (the #137-E1 mirror) | `graph/src/runtime.rs` | RED — `window 2 published its own blocks, not the previous block's state`, `1088069417` vs `1090923272` |
| 143-E13 | plan replacement | a freshly built lane starts `armed: true`, so a subscription would survive a replacement | `effect-contract/src/live.rs` | RED — `the replacement plan carries capacity and no subscription`, `[8, 8, 8]` vs `[8, 8, 0]` |

### E7 — the cost classes, measured

`observation_cost_classes_are_what_they_claim` (deterministic) and
`observation_cost_classes_are_separated_from_a_computed_scan_in_release` (`--ignored`, release
only) are the deterministic and descriptive halves of this claim, as two separate test functions.

The deterministic half counts reads through the same `wants` gate the runtime uses: **0** reads
over 4 096 blocks with capacity but nothing armed, exactly **4 096** with one tap armed, and back
to zero the moment it is disarmed.

The release-only half renders a real eight-compressor plan for 256 blocks in each of the four legs.
The table below is a debug-profile capture kept for the general shape; the test's actual gate is
release-only: `armed <= unarmed_with_console * 1.10 + 50 µs`, i.e. arming eight taps is not
measurably slower than an attached-but-unarmed console (debug profile, `x86_64` Zen 5, one shared
machine — evidence, not a pin):

| leg | 256 blocks | per block |
|---|---|---|
| no console | 253.74 ms | 991.2 us |
| console, no capacity | 252.36 ms | 985.8 us |
| capacity, unarmed | 252.11 ms | 984.8 us |
| every tap armed | 252.60 ms | 986.7 us |
| **synthetic computed scan** (negative control) | 14.33 ms | **56.0 us** |

The four observation legs span 0.6%, which is inside the run-to-run noise of the machine; the
negative control is ~90x the *entire* spread, which is what makes the comparison meaningful rather
than merely quiet.

| # | mutation | file | result |
|---|---|---|---|
| 143-E7-a | a tap declared `Resident` but implemented as a per-sample scan (the eval's named case, applied to the compressor's bank read) | `compressor/src/lib.rs` | RED — the armed row separates and the "far more than a copy out of state" bound fires |
| 143-E7-b | `ObservationLane::wants` returns true for any declared tap, armed or not | `effect-contract/src/live.rs` | RED — `an unarmed tap's state is never read`: 4 096 reads where 0 was required |

### Two mutations that had to be sharpened before they went red

* **E3 on the scalar path first escaped.** The test read a *settled* window: once the reduction
  stops moving, folding blocks `n-1..n+2` and `n..n+3` give the same peak, so publishing one block
  early was invisible. The test now reads four consecutive windows with a threshold retarget in the
  middle, so two of them sit on a moving envelope. Recorded because a gate that only discriminates
  on a moving signal is a fact about the gate, not a detail.
* **The scalar publish site was not wired at all** until this test existed: `stage` was still being
  handed `None` in `graph::runtime`, so a subscription on a per-node effect armed nothing. The
  banked fixture could not have found it, which is why the single-track dynamic-rack fixture exists.

---

## Issue #964 — the tests ported to `compile_with_builtins`

A representative sample of the ported tests, each shown to still catch its target after the port.
Every row was applied to the working tree, the named command run, the result recorded and the tree
restored (`git checkout`). Host: `x86_64` (`x86-64-v3`, eight-lane banks), rustc 1.97.1, debug,
on `fdb411fa`. Two rows carry a **control**: the same mutation against the naive port, without the
fixture strengthening #964 added. Both controls stay green, which is why the strengthening exists.

| # | ported test (what it must still catch) | mutation | file | command | result |
|---|---|---|---|---|---|
| 964-1 | `direct_graph_report_exposes_zero_output_latency_and_tail_without_identity_change` (a filter-free input section keeps the output tail finite) | `BuiltinTail::FiniteZero => TailSamples::Infinite` in `compile_with_builtins` | `graph-compiler/src/compile.rs` | `cargo test -p graph-compiler --lib direct_graph_report_exposes_zero_output_latency` | RED — `left: Infinite`, `right: Finite(0)` |
| 964-2 | `issue122_reverse_route_ids_emit_sorted_levels_and_bind` (SHA `14d73…` pins the with-builtins plan) | as 964-1 | as 964-1 | `... --lib issue122` | RED — `sorted production identity: "canonical identity"` |
| 964-3 | `the_zero_delay_plan_digest_is_the_current_semantic_plan` (the re-pinned digest covers the builtin tails) | `BuiltinTail::Infinite => TailSamples::Finite(0)` | as 964-1 | `cargo test -p graph-compiler --test track_delay` | RED — the digest falls back to exactly the builtins-less `eb3ca776…cb18e0ea10`, against the pinned `957e97ca…ce7640e42`: the eighteen tail tokens are the whole difference |
| 964-4 | `scalar_dispatch_compiles_without_banks_on_any_host` (the scalar dispatch forms no bank, builtin banks included) | attach the builtin banks at `Backend::current()` instead of the request's dispatch | as 964-1 (`into_graph_artifact_with_banks` call) | `... --lib scalar_dispatch` | RED — `plain.prepared_builtin_bank_count()`, `6` vs `0`. The assertion is new in #964; the effect-bank count the test already had stays green here |
| 964-5 | `runtime_bank_slot_reservation_is_published_and_capped_transactionally` (one slot reservation over effect and builtin banks, against a prior observed at `Backend::Scalar`) | the reservation counts the effect banks only (`.checked_add(0)` for the builtin count) | as 964-1 | `... --lib runtime_bank_slot_reservation` | RED — `published estimate carries slots`, `graph_metadata_bytes` 42,683 vs 43,091 |
| 964-6 | `effect_control_resource_uses_independent_queue_and_owner_arithmetic` (owner deltas on the report and the whole-plan estimate) | charge half the target-staging bytes | `graph-compiler/src/estimate.rs` | `... --lib effect_control_resource_uses` | RED — `bank-one-target: report graph owner delta`, 344,952 vs 459,640 |
| 964-7 | `accepted_session_compiles_binds_and_renders_direct_route` (the host binds exactly the input and the output) | a builtin bank member counts as an external node (`\|\| bank_nodes.contains(node)` for `&& !bank_nodes.contains(node)`) | `builtins-compiler/src/lib.rs` (`external_binding_nodes`) | `... --lib accepted_session_compiles` | RED — the external set lists `PostInputBuiltins`, `PostFader` and `PostMatrix` |
| 964-8 | `route_transform_bits_participate_in_semantic_hash` | the canonical `route-transform` row writes `1.0` for the gain | `graph-compiler/src/canonical.rs` | `... --lib route_transform_bits_participate` | RED — both plans hash to `0b8ee1b2…`. **Control:** the same mutation with the changed model left on the fixture's own input filters stays GREEN: their infinite tails alone make the two hashes differ |
| 964-9 | `dynamic_rack_compressors_bank_and_render_bit_identically_to_the_per_node_path` (bank against per node, lane by lane) | the per-node effect exchanges its two output planes after processing | `graph/src/runtime.rs` (`NodeKind::Effect`) | `... --lib dynamic_rack_compressors_bank_and_render` | RED — block 0 differs, `1068755679` vs `3201302908`. **Control:** the same mutation with `accepted_compressor_graph_fixture` on the fixture's own `pan: {left: 1, right: 1}` stays GREEN: both lanes are panned hard right and sum into one output, so a crossed lane renders the same bits |
| 964-10 | `add_a_track_keeps_existing_track_bits_and_one_transpose_per_chain`, `console_sixty_four_track_fixture_banks_its_dynamic_compressor_bit_identically` (the chain re-pins) | the runtime never merges two bank units into one chain (`&& false` on `chains_into`) | `graph/src/runtime.rs` (`cohort_runs`) | `... --lib add_a_track`; `... --lib console_sixty_four_track_fixture` | RED — `8 tracks: [post-input, EQ] and [fader, matrix] per cohort`, 4 vs 2; `the EQ and the dynamic compressor fuse into one chain per cohort`, 40 vs 8. The pin add-a-track replaced, `chains == slots`, holds under this mutation (4 == 4) |
| 964-11 | `compiles_65_537_tracks_or_rejects_only_a_configured_resource` (only a configured cap can refuse the session) | a compiled track ceiling: `graph.track.limit` above 65,536 tracks | as 964-1 | `cargo test -p graph-compiler --test scale compiles_65_537` | RED — the constrained compile's diagnostics are no longer all `graph.resource.limit` |

### Attempt 2 (after Sol's attempt-1 verdict)

Same method and host, on the attempt-2 tree. Sol's M-b showed that attempt 1 had weakened five
launch-effect cap tests: they took their one-byte-below plan cap from the report's estimate, which
is taken before the builtin banks attach, while the compile caps the whole-plan figure. So each cap
sat 64,043 bytes (the builtin attachment) below the real limit, and M-b stayed GREEN on the ported
tests while the builtins-less originals went RED. The five now read `graph_resource_estimate()`.

| # | ported test (what it must still catch) | mutation | file | command | result |
|---|---|---|---|---|---|
| 964-12 (Sol's M-b) | the limiter, multiband, soft-clip, transient-shaper and delay `launch_*` tests (the one-byte-below cap sits exactly one byte below the figure the compile caps) | the bank-slot reservation is left out of the capped estimate only (`capped_estimate.checked_add_bank_slot_owners(Default::default())`) | `graph-compiler/src/compile.rs` | `cargo test -p graph-compiler --lib launch_` | RED, all five: `one-byte-below {limiter,multiband,soft-clip,transient} graph cap must reject before publication`, `one-byte-below delay graph cap must reject`. Attempt 1's versions stayed GREEN under the same mutation (Sol) |
| 964-13 | the limiter, multiband, soft-clip and transient-shaper `launch_*` tests (the bank-against-scalar deltas they pin on the report hold on the whole-plan estimate: `assert_builtin_attachment_matches`) | the builtin attachment charges the effect banks' metadata to `incremental_plan_bytes` a second time (in `PreparedGraphPlan::with_builtin_banks`) | `graph/src/lib.rs` | `cargo test -p graph-compiler --lib launch_` | RED, all four: `the builtin attachment adds the same bytes to both arms`, e.g. multiband `64802` against `64042` |
| 964-3 (recheck) | `track_delay::the_zero_delay_plan_digest_is_the_current_semantic_plan`, now at `Backend::current()` | as 964-3 | as 964-3 | `cargo test -p graph-compiler --test track_delay` | RED, the digest falls back to `eb3ca776…` |
| S5 (recheck) | `track_delay::the_estimate_charges_each_lane_its_own_ring` at `Backend::current()` | the left ring is charged 2 bytes per sample | `graph-compiler/src/compile.rs` | as above | RED, `delay_bytes for 1/0`, 2 vs 4 |

---

## Issue #966 — no effect bank spans dependency levels

Every row was applied to the working tree, the named command run, the result recorded and the tree
restored. Host: `x86_64` (`x86-64-v3`, eight-lane banks), rustc 1.97.1, debug,
`CARGO_INCREMENTAL=0`, on the #966 branch cut from `584dbc16`. The command for every row is
`cargo test -p graph-compiler --test bank_levels`. The unmutated tree passes 9 of 9.

| # | mutation | file | result |
|---|---|---|---|
| 966-M1 | delete the level check (the base tree's binder) | `graph-compiler/src/banks.rs` | RED, 9 of 9. Every reproducer and guard refuses with `graph.scheduler.layout` at `Simd8`; the probe refuses 29 lines over 23 seeds (16 at `Simd4`, 13 at `Simd8`) |
| 966-M2 | over-strict check: leave a slot unbound whenever any lane skipped an earlier slot (`rank != slot`), instead of whenever the members' levels differ | as M1 | RED, 2 of 9, and only the two over-reach guards: `lanes_that_skip_the_same_slot_still_bank_it` (19 banks, not 20) and `a_slot_after_a_misaligned_one_realigns_and_still_banks` (8, not 9). Nothing else can see a pure banking-count loss, which is why both guards exist |
| 966-M7 | `continue` becomes `break` in the check, so every slot after a group's first misaligned slot is dropped, including slots where the lanes realign | as M1 | RED, 1 of 9: `a_slot_after_a_misaligned_one_realigns_and_still_banks` (8 banks, not 9). The #966 verification found M7 green against the prototype's four tests (`docs/handoffs/bug-966-2026-09-27/VERIFY-966.md`, finding 2): in every other reproducer the slots after the misaligned one are misaligned too |
| 966-M3 | compare only the first and the last member's level | as M1 | RED, 3 of 9: `the_console_less_a_middle_lanes_eq_binds_at_every_width` (`ch60` is lane 4 of 8), the seed-412 shape and the probe, each with `graph.scheduler.layout`. The first-lane and last-lane gate-1 variants stay green, which is why the middle-lane variant exists |
| 966-M8 | skip lane 1 in the comparison (`.skip(2)`) | as M1 | RED, 1 of 9: only the probe (seeds 31 and 58). No reproducer puts its ragged lane at lane 1 |
| 966-M9 | the test's own source feeds a mono-mapped track's right side from the next source channel, so the collapse's premise is false | `graph-compiler/tests/bank_levels.rs` (`TrackSource`) | RED, 2 of 9, armed legs only: `the_mono_console_less_one_eq_binds_and_collapses_at_every_width` (`Simd4`, armed) and the probe (every moved line is `Armed`). The armed comparisons can see a wrong collapse; every unarmed comparison stays green |
