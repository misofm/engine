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

## Issue #143 — the bank slot's observation surface

| # | mutation | file | test | result |
|---|---|---|---|---|
| 143-E5-rack | build the per-lane sample scratch unconditionally in `ConsoleEffectBankStage::new` | `rack/src/lib.rs` | `console_bank::an_unobserved_bank_slot_reports_no_observation_state_at_all` | RED — an unobserved slot stops being structurally distinguishable from an observed one, and `is_observed` stops meaning anything |

## Issue #218 — the chain's fold epilogue

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Driver: one mutation at a time,
`cargo test -p miso-engine-rack`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 218-R1 | a folded lane's plane is written *as well as* handed over (the `else` becomes an unconditional block), so the fold is a second destination rather than a replacement | `rack/src/lib.rs` `scatter_tiled` | `the_fold_epilogue_is_absent_by_default_and_replaces_the_lane_write_when_armed` | RED (`lane 0: a folded lane's own plane must not be written`) |
| 218-R2 | the tiled scatter's lane loop runs `(0..W).rev()`, so the epilogue visits lanes in descending order | `rack/src/lib.rs` `scatter_tiled` | the same test | RED (`the epilogue visits lanes in order`) |

Row 218-R2 is why the lane order is asserted here and not only end to end: a scatter-accumulate
into a destination several lanes share associates in the order the epilogue visits them, and the
graph runtime's `route_fold` proves that order matches a frozen D9 reduction. If the chain quietly
reordered its own lanes, that proof would be about the wrong sequence.

## Mono-collapse M2 — the collapsed execution

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Driver: one mutation at a time, tree restored
(and `touch`ed, so cargo cannot serve a stale artifact) before the next row.

The collapse renders the bits a dual run renders — that is the whole claim — so **no digest can see
whether it fired, and no digest can see it firing wrongly on a lane whose two channels differ only
in state.** Every row here is therefore either an output comparison against an independently
computed expectation or a counter, and never a comparison of the collapsed path against itself.

| # | mutation | file | test | result |
|---|---|---|---|---|
| M2-1 | `BankChain::new` initialises `collapse_source: true`, so a chain nobody performed the structural join for collapses anyway | `rack/src/lib.rs` | `an_unarmed_chain_never_collapses` (and `an_ineligible_armed_chain_renders_the_dual_bits`) | RED — 2 failed. The `SOURCE` term is keyed by track id and a chain sees anonymous lanes; without the join a two-source track's right channel becomes the duplicated left one. This gates the **default** only: a plan that *is* joined overwrites the field on every chain, so the same edit is green end to end. The join itself is cut in `miso-engine-graph`'s M2-G1 |
| M2-2 | the dispatch drops `self.all_lanes_symmetric()` | `rack/src/lib.rs` `run` | `an_ineligible_armed_chain_renders_the_dual_bits` | RED — the ineligible chain's right plane comes back scaled by the *left* gain |
| M2-3 | the seam-side matrix is folded to `(ll + lr) * l` / `(rl + rr) * l` — the "obvious" saving once `l == r` | `rack/src/lib.rs` (test mock `Matrix`) | `a_collapsed_seam_keeps_the_matrixs_operation_order` | RED — `+0.0` against `-0.0` on every `-0.0` frame, on both the collapsed and the dual arm, which is why the expectation is written out rather than taken from a second chain |
| M2-4 | the seam duplication is removed (`right[..len].copy_from_slice(&left[..len])` becomes a no-op) | `rack/src/lib.rs` `run` | `a_collapsed_seam_keeps_the_matrixs_operation_order` | RED — the seam-side slot reads the plane the collapsed gather never wrote |
| M2-5 | the drain is moved back inside `process`, so the dispatch reads the witness *before* this block's records are admitted | `rack/src/lib.rs` `run` | `miso-engine-console-workload` `chain_shape::a_live_one_channel_retarget_disengages_on_the_block_it_lands` | RED — a `ParameterChannel::Left` retarget takes effect on a block that still ran collapsed, so the right channel receives a retarget addressed to the left one |
| M2-6 | a collapsed slot's observation publish drops `sample.right = sample.left`, so the right-channel tap carries the state word frozen at the engage | `rack/src/lib.rs` `process_inner` | `miso-engine-console-workload` `chain_shape::a_collapsed_cohorts_right_channel_taps_read_what_a_dual_runs_do` | RED — and every digest assertion in that file stays green, which is exactly why the tap values are compared rather than counted |
| M2-7 | the collapsed slot's dry capture takes `block.right` -- the ungathered resident scratch -- instead of `block.left` | `rack/src/lib.rs` `process_inner` | `miso-engine-console-workload` `chain_shape::a_bypass_engaged_after_a_collapsed_run_renders_the_dual_bits` | RED — the limiter arm diverges on blocks 32-35 and the compressor arm on 35-43, while the zero-latency EQ arm stays green. This was a **shipped defect**, found in verification, not a hypothetical: see the note below |
| M2-8 | the dispatch's `collapse_prefix > 0` becomes `>= 0` | `rack/src/lib.rs` `run` | `an_armed_seam_side_only_chain_renders_the_dual_bits` (and `only_a_seam_suffix_over_a_collapsible_prefix_can_collapse`) | RED — 2 failed. A fader-or-matrix-only chain reports every lane symmetric on every session, so nothing but the empty prefix refuses it, and the seam would publish the duplicated left plane as its right output |
| M2-9 | `collapse_prefix_of` drops its `lanes_agree` clause | `rack/src/lib.rs` | `a_partial_agreement_slot_declines_the_collapse` | RED |

### M2-7 was a shipped defect

The row above is the one mutation in this file that was not invented to test a gate: it *was* the
code, and adversarial verification found it. `BypassShunt::capture` does two separable things --
stage `dry_*`, which no reader carries across a block boundary, and exchange that staging through a
delay **line**, which persists for the slot's declared latency. The collapsed path's comment argued
that the seam overwrites the right plane after the slot, which is true of the plane and false of the
line: the line is written *before* the slot runs and read `latency` samples later, on a block the
seam has long since passed. Engaging a live bypass after a collapsed run then restored ungathered
resident scratch into the bypassed lane's right channel.

Two things about the shape of the failure are worth keeping. It needed a **sequence** -- collapse,
then a live `Bypass(true)`, then the disengage that record itself causes -- so no single-mode test
could reach it; and it was **proportional to declared latency**, so it failed at the limiter and the
compressor and not at the zero-latency EQ, which is what named the cause rather than blaming an
effect. The fix captures the left plane twice on the collapsed path, so the line receives the words
a never-collapsed run puts there on *every* block rather than being repaired at a boundary, and the
collapsed path is left reading no right plane at all.

### Counting

Nine rows in this section, and the M2 work as a whole carries **31 ledger rows across six crates for
30 distinct mutations**: rack 9, compressor 8, true-peak-limiter 8, parametric-eq 2, builtins 1,
graph 3. Rows outnumber mutations by exactly one: the drain/dispatch ordering edit is listed twice,
once from each side of the seam it crosses (M2-5 here, M2-G3 in `miso-engine-graph`). Every row was
applied, run, observed red and reverted; the counts are stated so a reader who tallies them and gets
a different number knows which of the two they have counted.

## Mono-collapse M3 — the re-engage rule

Same driver and same host as M2: one mutation at a time, applied to the working tree, the named
test run, the failure observed, the tree restored (and `touch`ed) before the next row.

M2 shipped the disengage as a one-way latch, so **no M2 test rendered a re-engaged block**. The
rows below are the gates on the transition that latch was standing in for, and they split into two
groups that have to fail differently: a rule that is too strict costs the collapse (a counter falls)
and a rule that is too loose is wrong audio (a digest moves). A ledger with only one kind of row
would not be checking a rule at all.

| # | mutation | file | test | result |
|---|---|---|---|---|
| M3-1 | the dispatch drops `&& self.collapse_channels_agree`, which is M2's latch simply deleted | `rack/src/lib.rs` `run` | `mono_reengage::re_equal_words_after_a_desymmetrised_episode_do_not_re_engage`, `mono_reengage::an_earned_agreement_proof_re_engages_a_chain_the_witness_could_not` | RED — 2 failed, both on the **output** comparison against a never-collapsed oracle and neither on a counter: the chain re-engages onto a right channel four blocks behind its left |
| M3-2 | `disengage_collapse` no longer sets `collapse_channels_agree = true` | `rack/src/lib.rs` | (whole `mono_reengage` suite, whole `chain_shape` suite) | **GREEN, deliberately.** Reaching that line requires `self.collapsed`, and a chain only collapses while the flag holds, so the assignment is redundant today. It is kept as the statement of *why* agreement survives a window in which the right channel was frozen, and a `debug_assert!` next to it fails the moment the redundancy stops holding. Recorded rather than dropped because a reader who cuts the line and sees green deserves to find the reason written down |
| M3-3 | `ChannelSymmetryWitnessV1::AGREEING` becomes `ALL`, folding `UNBYPASSED` back into the invariant | `effect-contract/src/symmetry.rs` | `mono_reengage::a_bypass_episode_re_engages_because_it_never_moved_the_channels_apart`; `miso-engine-console-workload` `chain_shape::a_lifted_bypass_re_engages_the_collapse_and_renders_the_dual_bits` | RED — 2 failed, both on the **counter**: the bypassed cohort is retired for the rest of the plan instead of for the four bypassed blocks. Both digests stay green, which is the point of asserting the count as well — a too-strict rule is invisible to every digest in the tree |
| M3-4 | the agreement proof is never asked (`BankStage::channels_agree` is short-circuited to `false` in the dispatch) | `rack/src/lib.rs` `run` | `mono_reengage::an_earned_agreement_proof_re_engages_a_chain_the_witness_could_not` (and `mono_reengage::re_equal_words_after_a_desymmetrised_episode_do_not_re_engage` on its query count) | RED — 2 failed. The second row is what proves the recovery window is entered at all on the session that must *not* recover, so the first is not passing because the query was skipped |
| M3-B1 | `InputStage::process_mono` drops `.saturating_add(self.members_sum(report.sanitized[1]))` from `sanitized_input` | `builtins/src/lib.rs` | `miso-engine-builtins` `mono_collapse::the_collapsed_body_publishes_the_dual_bodys_report` | RED — the collapsed body reports half the sanitised samples a dual body reports, and the audio is untouched. Ledgered in `miso-engine-builtins/tests/MUTATIONS.md` as well; it is repeated here because it is the accounting half of `BankStage::process_mono`'s contract |

### What the two red tests are, and why one of them is a session and not a stage

`mono_reengage::re_equal_words_after_a_desymmetrised_episode_do_not_re_engage` is the flaw this
milestone exists to exclude, stated at the chain: a window rendered dual with the `DESIGNED` term down separates the two
channels' recursive state, the words then agree again, every term of the witness holds, and
collapsing would publish the left channel's state as the right channel's. The test asserts the
decline **and** asserts that the states really do disagree at that boundary, so it cannot pass
because the episode stopped separating anything.

`re_equal_designed_words_after_a_one_channel_retarget_never_re_engage` in
`miso-engine-console-workload` is the same session in the vocabulary the engine actually has — a
`ParameterChannel::Left` retarget followed by a `ParameterChannel::Both` one carrying the same
value — and it is **green under M3-1**, which is stated in its own doc comment rather than hidden.
The `LIVE` term has no restoring arm, so today's live-record vocabulary already declines that
session forever, and the invariant is the second of two mechanisms. That is worth having and it is
not the rule: `miso_engine_effect_contract::symmetry`'s module header names two seams (builtins
liveness, session automation spans) whose arrival could make a term restorable, and the invariant is
what will still be standing when one of them lands.

### Counting

Five rows in this section for five distinct mutations, four of them red and one deliberately green.
Rows M3-1, M3-3 and M3-4 are `rack`'s and `effect-contract`'s; M3-B1 is `builtins`' and is
double-listed there for the same reason M2-5 is double-listed in `miso-engine-graph`.
