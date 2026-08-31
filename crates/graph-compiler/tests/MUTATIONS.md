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
* Command: `bash scripts/check-math-policy.sh .`
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

`observation_cost_classes_are_what_they_claim` has a deterministic half and a descriptive one.

The deterministic half counts reads through the same `wants` gate the runtime uses: **0** reads
over 4 096 blocks with capacity but nothing armed, exactly **4 096** with one tap armed, and back
to zero the moment it is disarmed.

The descriptive half renders a real eight-compressor plan for 256 blocks in each of the four legs
(debug profile, `x86_64` Zen 5, one shared machine — evidence, not a pin):

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
