# Red-mutation log — audit #99 (`miso-engine-graph-compiler`, plan lowering and cohorts)

Every gate this job lands was proven non-vacuous by applying the mutation below, running the named
command, observing the failure recorded here, and reverting. A gate with no red mutation is not
evidence; a mutation that stays green is recorded as such and the gate is strengthened until it
does not (see M-05, which was green on first attempt and forced a new fixture).

Delivery host: x86_64 with AVX2+FMA (`x86-64-v3`), rustc 1.97.1.

---

## F4 — deterministic route gain

### M-01 — restore the platform `powf`
* Mutation: in `route_transform`, replace `miso_engine_math::db_to_gain_f32(gain_db)` with
  `10_f64.powf(f64::from(gain_db) / 20.0) as f32`.
* Command: `cargo test -p miso-engine-graph-compiler --lib route_transform_uses`
* Red: `route_transform_uses_the_canonical_db_to_gain_conversion` —
  `assertion left == right failed / left: [1038469653] / right: [1038469654]`
  (`0x3de5_ca15` vs the canonical `0x3de5_ca16` at -19 dB).

### M-02 — perturb the conversion argument
* Mutation: in `tests/route_gain.rs`, call `db_to_gain_f32(db * (1.0 + 1e-6))`.
* Command: `cargo test -p miso-engine-graph-compiler --test route_gain`
* Red: `route_gain_matches_f64_oracle_within_two_ulp` —
  `db_to_gain_f32 deviates 43 ulp from the f64 oracle at 23.75 dB, inside the +/-24 dB mixing range`.

### M-03 — re-add a platform transcendental to the crate
* Mutation: same source edit as M-01.
* Command: `bash scripts/check-math-policy.sh .`
* Red: exit 1 —
  `math policy failure: platform transcendental calls outside crates/miso-engine-math; call miso_engine_math instead (D6)`.

---

## F6 — dispatch is a compile input

### M-04 — compile reads the host CPU again
* Mutation: `let rack_cohorts = rack_cohort_report(&effects, KernelDispatch::select(miso_engine_core::target_capabilities()));`
* Command: `cargo test -p miso-engine-graph-compiler --lib scalar_dispatch`
* Red: `scalar_dispatch_compiles_without_banks_on_any_host` —
  `assertion left == right failed / left: 1 / right: 0` (`prepared_bank_count()` under the scalar
  dispatch).

---

## F2 — the lowering pass (`miso_engine_graph::program`)

### M-05 — count taps as readers  *(green on first attempt; gate strengthened)*
* Mutation: in `lower`, delete the `if elided[destination] { continue; }` guard in the reader
  count, so an edge into an elided stage counts as a consumption of its producer's buffer.
* Command: `cargo test -p miso-engine-graph -- program::`
* **First attempt: GREEN.** The effect-free chain fixture roots every alias chain at the
  bank-eligible builtin stage, whose buffer is never consumed in place for an unrelated reason, so
  miscounting taps there changes nothing. Recorded rather than accepted: a fixture with a
  *dynamic-rack* effect (not bank-eligible) was added,
  `taps_are_not_readers_so_an_alias_chain_still_folds_into_its_producer`.
* Red, after: `an alias chain with one real reader must fold into its producer's buffer`.

### M-06 — drop the dedicated-buffer rule
* Mutation: `const fn is_dedicated(_node: &GraphNodeId) -> bool { false }`.
* Command: `cargo test -p miso-engine-graph -- program::`
* Red: `chain_of_seven_stages_lowers_to_six_ops_three_taps_and_two_buffers` —
  `left: 1 / right: 2` (the arena collapses onto a bank member's storage).

### M-07 — never consume a buffer in place
* Mutation: `let in_place = false && single && !dedicated && { ... };`
* Command: `cargo test -p miso-engine-graph -- program::`
* Red: two tests. `chain_of_seven_stages_...` — `left: 3 / right: 2`; and
  `taps_are_not_readers_...` — `an alias chain with one real reader must fold into its producer's
  buffer`.

### M-08 — free a buffer one op too early
* Mutation: in the colouring sweep, `for buffer in expire[op_index].drain(..)` instead of
  `expire[op_index - 1]`.
* Command: `cargo test -p miso-engine-graph -- program::`
* Red: `delayed_edge_gets_staging_buffer_and_blocks_in_place` — the PDC staging buffer collides
  with a buffer whose last read is the current op.

### M-09 — return dedicated buffers to the free list  *(green on first attempt; gate strengthened)*
* Mutation: drop the `if !lifetimes[buffer].dedicated` guard, so a bank-eligible node's buffer
  re-enters the arena when its last *reader* has run.
* Command: `cargo test -p miso-engine-graph -- program::`
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
* Command: `cargo test -p miso-engine-graph -- program::`
* Red: two tests — `chain_of_seven_stages_...` and `taps_are_not_readers_...`.

---

## F1 — the wave-0 level-order property (gated here, not assumed)

`#122`/`#123` landed the fix; the property test the plan specified for it did not land with it, so
it lands here. `direct-route` cannot see this bug: it is a chain with one node per level.

### M-11 — emit levels in Kahn pop order
* Mutation: in `topo`, replace the per-level `nodes.sort()` with a no-op.
* Command: `cargo test -p miso-engine-graph-compiler --lib random_dags`
* Red: `random_dags_have_strictly_ascending_levels_and_level_major_schedule` —
  `graph 0: level 1 is not strictly ascending` (which is exactly what
  `NativeGraphBlueprint::prepare` rejects with `graph.scheduler.layout`).

### M-12 — level = max(predecessor) instead of max(predecessor) + 1
* Mutation: `.map_or(0, |value| value)` in `topo`'s level computation.
* Command: `cargo test -p miso-engine-graph-compiler --lib random_dags`
* Red: `graph 0: level of Submix { submix_id: StableGraphId("n03") }` — the in-test longest-path
  recomputation disagrees.
