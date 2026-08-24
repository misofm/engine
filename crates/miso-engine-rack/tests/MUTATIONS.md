# Red-mutation record for the #96 bank-chain and cohort-planner gates

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Every row below was applied to the working tree, the named test was run, the failure output
was recorded, and the mutation was reverted in the same session. Nothing in this file is a claim
about code that was not run.

Host: `x86_64` (16 cores), workspace `.cargo/config.toml` pin `-C target-feature=+avx2,+fma`,
debug profile. Sweep driver: one mutation applied at a time, `cargo test -p <pkg> <test> --
--exact tests::<test>`, tree restored before the next row.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p miso-engine-rack -p miso-engine-rack-compiler -p miso-engine-graph-compiler
# and revert
```

## `miso-engine-rack` — the bank chain

| # | mutation | file | test | result | first failure line |
|---|---|---|---|---|---|
| M1 | `scatter_lane` reads the wrong lane: `*sample = chunk[lane]` becomes `chunk[lane ^ 1]` | `rack/src/lib.rs` | `gather_scatter_round_trip_is_bit_exact` | RED | ``assertion `left == right` failed: left lane=0 frame=0`` |
| M2 | `gather_lane` chunks by frames instead of lanes: `chunks_exact_mut(lanes)` becomes `chunks_exact_mut(frames as usize)` | `rack/src/lib.rs` | `stage_sees_frame_major_layout` | RED | ``assertion `left == right` failed: lane=0 frame=1`` |
| M3 | `BankChain::run` drops the identity-slot guard: `if slot.active_lanes.iter().any(..)` becomes `if true` | `rack/src/lib.rs` | `identity_everywhere_slot_is_not_executed` | RED | ``assertion `left == right` failed: both live slots ran once per block; the identity-everywhere slot never ran`` |
| M3b | the transpose counter moves inside the slot loop (master plan §4.5: it is *per chain*, not per slot) | `rack/src/lib.rs` | `identity_everywhere_slot_is_not_executed` | RED | ``assertion `left == right` failed`` (2 live slots ⇒ 10 counted round-trips for 5 blocks) |
| M4 | `BankChain::new` stops enforcing "a slot may only be active on an active lane" | `rack/src/lib.rs` | `chain_new_rejects_mask_shape_and_lane_implication` | RED | ``assertion `left == right` failed: a slot may not be active on a lane the chain never gathers`` |
| M5 | `AoSoaScratch::new` re-adds a plane: `vec![0.0; length]` becomes `vec![0.0; length * 2]` for the left plane | `rack/src/lib.rs` | `scratch_allocates_exactly_two_planes` | RED | ``assertion `left == right` failed`` |
| M6 | `gather_lane` drops the first frame of every block: `.zip(left)` becomes `.zip(&left[1..])` | `rack/src/lib.rs` | `chain_run_is_partition_invariant` | RED | ``assertion `left == right` failed: partition=1 lane=0 frame=0`` |
| M7 | `run` scatters from inactive lanes too: `if self.active[lane]` becomes `if true` on the scatter loop | `rack/src/lib.rs` | `inactive_lanes_are_never_gathered_or_scattered` | RED | ``assertion `left == right` failed`` |

## `miso-engine-rack-compiler` — the single cohort planner

| # | mutation | file | test | result | first failure line |
|---|---|---|---|---|---|
| P1 | lane order within a group is reversed: `order_members`'s tie-break becomes `b.id.cmp(&a.id)` | `rack-compiler/src/lib.rs` | `single_slot_programs_reproduce_exact_equal_chunking` | RED | ``assertion `left == right` failed: case=0`` |
| P2 | the `is_bankable` filter is dropped: `if candidate.program.is_bankable()` becomes `if true` | `rack-compiler/src/lib.rs` | `empty_programs_and_connected_sidechains_never_bank` | RED | ``assertion `left == right` failed`` |
| P3 | `subsequence_mask` stops advancing the cursor, so one leader slot can satisfy two program slots | `rack/src/lib.rs` | `subsequence_uses_program_equality_not_occurrence` | RED | ``assertion `left == right` failed`` |
| P4 | full-program tracks stop filling banks first: `b.active_count().cmp(&a.active_count())` becomes `a.active_count().cmp(&a.active_count())` | `rack-compiler/src/lib.rs` | `longest_program_leads_and_full_programs_fill_first` | RED | ``assertion `left == right` failed: full-program tracks fill the first bank even though their ids are larger`` |
| P5 | cohort pooling stops being exhaustive: subsequence matching is replaced by program equality | `rack-compiler/src/lib.rs` | `pooling_is_exhaustive_so_no_member_is_stranded` | RED | `case=0: id 4 could have filled a free lane in group 0` |
| P6 | `order_members` loses its id tie-break, so equal-`active_count` members keep pool order | `rack-compiler/src/lib.rs` | `output_is_input_order_invariant` | RED | ``assertion `left == right` failed`` |
| P9 | the leader's mask is reused for every member: `subsequence_mask(..)` becomes `subsequence_mask(..).map(|_| vec![true; leader.slots.len()])` | `rack-compiler/src/lib.rs` | `every_slot_cohort_is_homogeneous` | RED | ``assertion `left == right` failed: case=0 lane=0`` |
| **P10** | **the planner's lane order is reversed (P1's mutation), observed from `miso-engine-builtins-compiler`** | `rack-compiler/src/lib.rs` | `builtin_bank_layout_regroups_by_dependency_wave_and_scalar_falls_back` (#86 phase A's own test) | RED | `assertion failed: groups.iter().all(|members| ...` |
| P7 | duplicate ids are accepted: the `windows(2).any(..)` guard becomes `if false` | `rack-compiler/src/lib.rs` | `duplicate_ids_are_rejected_across_levels` | RED | `assertion failed: plan_invariants_hold(&plan, lanes)` |

## `miso-engine-graph-compiler` — the bound plan and the §4.5 law

| # | mutation | file | test | result | first failure line |
|---|---|---|---|---|---|
| G3 | padded groups are bound too: the `if !group.is_full()` guard in `bind_rack_banks` becomes `if false` | `graph-compiler/src/lib.rs` | `add_a_track_keeps_existing_track_bits_and_one_transpose_per_chain` | RED | `full group` (the nine-track session panics binding a lane-short bank) |
| G5 | the transpose counter stops counting: `saturating_add(1)` becomes `saturating_add(0)` | `rack/src/lib.rs` | `add_a_track_keeps_existing_track_bits_and_one_transpose_per_chain` | RED | ``assertion `left == right` failed: one planar/AoSoA round-trip per chain per block`` |

## Recorded equivalent mutants, with the arithmetic

Neither of these is quietly dropped; both are stated so a later job can re-test them once the
premise changes.

* **A lane permutation applied to *both* sessions of the cohort-boundary test (G3).** Mutating
  `scatter_lane` to `chunk[lane ^ 1]` permutes the eight-track bank identically in the eight-track
  and nine-track sessions, so the *comparison* stays equal. The property "lane `i` of the block is
  member `i`" is owned by M1 (bit-exact round trip) and P1 (lane order), not by G3, whose subject is
  the cohort *boundary*. Under master-plan D5 (bank ≡ scalar to the bit) a pure membership change
  is also unobservable in G3 by construction; that is what makes it a boundary test rather than a
  membership test.
* **Moving the transpose counter into the slot loop, observed at the graph level (G5).** Every chain
  #96 builds has exactly one slot, so per-slot and per-chain counting agree there. M3b makes the
  same mutation observable at the unit level with a three-slot chain, which is why the row above is
  RED rather than recorded here. #99's multi-slot chains make it observable at the graph level too.

## Dead code found by a surviving mutant, and removed rather than left untested

Two pieces of the planner survived their mutations because nothing could reach them:

* **The remainder-placement pass** (plan §6 step 4). Its mutation was green because the pass never
  moves a member: within one cohort every group but the last is full, and a member of a later
  cohort is by construction *not* a subsequence of an earlier cohort's leader — otherwise the
  exhaustive pooling in step 2 would already have placed it there. The pass is deleted and replaced
  by the proof in the crate doc plus `pooling_is_exhaustive_so_no_member_is_stranded` (P5), which
  gates the argument on a seeded corpus.
* **The pool canonicalising sort.** Its mutation was green because leader selection is a total
  `max_by` over unique ids, `order_members` fixes every group's lane order, and `scalar` is sorted
  on the way out. It is deleted; `output_is_input_order_invariant` (P6) is the gate on that claim,
  and P6 is red under the mutation that actually reintroduces order dependence.

## P10 is the load-bearing row for the #86-A reconciliation

#86 phase A landed padding for the post-input builtin banks against its own `chunks(W)` loop while
this branch was unmerged. Two padding implementations is the defect this workstream exists to kill,
so `planned_builtin_bank_members` now delegates to `plan_bank_groups`. **P10 is the proof that the
delegation is real rather than decorative:** mutating the lane-order rule inside
`miso-engine-rack-compiler` turns #86-A's own builtins layout test red. A surviving second copy in
`miso-engine-builtins-compiler` would have kept it green.

Byte-identity of the groups is proven by #86-A's evals staying green **untouched**:
`banked_tracks_are_bit_identical_to_their_scalar_tails` (E2),
`track_bits_do_not_depend_on_session_track_count` (E3),
`builtin_bank_resource_charges_two_planes_and_actual_members` (E7),
`builtin_bank_layout_regroups_by_dependency_wave_and_scalar_falls_back`,
`phase_two_allocator_layouts_match_the_checked_resource_report`, and the frozen seeded 100-layout
transcript in `miso-engine-graph-compiler`.

## Alignment with the #95 bank-binding semantic

`plan_bank_groups` is the place the effect contract's `bind_homogeneous_bank` doc names as the
gate on cohort homogeneity ("`graph-compiler` groups candidates by `metadata.program_key()` before
it ever calls this method"). `every_slot_cohort_is_homogeneous` (P9) is that gate, over a seeded
corpus: for every group and every lane, the leader keys the lane is active on are exactly the
lane's own program, in order.

| # | mutation | file | test | result | first failure line |
|---|---|---|---|---|---|
| P9 | the leader's mask is reused for every member: `subsequence_mask(&leader_program)` becomes `Some(vec![true; leader_program.slots.len()].into_boxed_slice())` | `rack-compiler/src/lib.rs` | `every_slot_cohort_is_homogeneous` | RED | ``assertion `left == right` failed: case=0 lane=0`` |

`bind_rack_banks` matches the frozen outcome table exactly: `Err(code)` becomes a transactional
graph-compile diagnostic (`id_ordered_bank_plan_rejects_transactionally_and_returned_ownership_is_reusable`,
`fixture.bank.bind_failure`), `Ok(None)` marks the group unbound and its members render on the
per-node scalar path (`launch_parametric_eq_fixture_retains_banks_and_matches_scalar_across_blocks`
asserts every planned group is reported unbound and all nine members appear in `scalar_in`).

## Issue #140 — the automation-span feed, the live fader, and GR observation

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one mutation at a time,
`cargo test -p <pkg> <test>`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-3 | `ConsoleEffectBankStage::process` packs every lane at `packed[..staged]` instead of at that lane's own running offset, so a later lane's spans overwrite an earlier lane's while the offsets still partition the array | `rack/src/lib.rs` | `console_bank::each_lane_receives_only_its_own_commands` | RED (`lane 0 got its own command`) |
| 140-4 | the per-lane bypass restore walks the AoSoA block with `index += 1` instead of `index += lane_count`, so a bypassed lane's dry samples land in every lane | `rack/src/lib.rs` | `console_bank::bypass_is_per_lane_and_preserves_the_declared_latency` | RED (`lane 1 keeps the wet, gained signal`) |
