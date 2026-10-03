# Red-mutation log — audit #103 wave 4 (shared host facade, render path, C boundary)

Every gate this job lands was proven non-vacuous by applying the mutation below, running the named
command, observing the failure recorded here, and reverting. A gate with no red mutation is not
evidence. Two mutations recorded here went **green** on the first attempt and forced a change: they
are kept (M-04a, M-16a) because a mutation that stays green is a fact about the gate, not a
footnote.

This log covers the whole wave-4 job, not only this crate: the job spans
`host-core` (new), `graph`, `engine` and `capi`, and
one log is easier to check than four. Numbering is the plan's step order, so gaps (M-09, M-14,
M-15) mark work that was implemented, proven, and then dropped when #98 merged -- see W4-6.
M-27..M-31 close a gap the verifier found after the first pass, and are recorded here in full,
including the mutation that was green before them.

Delivery host: x86_64 with AVX2+FMA (`x86-64-v3`), rustc 1.97.1.

---

## W4-1 — a binding can acknowledge a node without a processor

### M-01 — the executor ignores supplied processors
* Mutation: in `runtime::RuntimeParts::node_kind`, replace
  `else if let Some(Some(processor)) = self.bindings.remove(node) { NodeKind::Bound(processor) }`
  with `else if self.bindings.remove(node).is_some() { NodeKind::Identity }`.
* Command: `cargo test -p graph identity_binding`
* Red: `identity_binding_acknowledges_without_a_processor` — `left: [0, 0]` vs
  `right: [1048576000, 3204448256]` (the bound `Constant` processor never ran).

*Recorded green attempt:* the first version of this gate compared identity-bound output against
processor-bound output with no processor anywhere in the plan, and every mutation left it green
because both arms rendered silence. The gate now binds a value-writing processor to the *input*
node, so a mutation that ignores supplied processors shows up as `[0, 0]`.

## W4-2 — the shared host-preparation facade

### M-02 — collapse the source-error string table
* Mutation: replace the whole body of `SourceControlError::diagnostic` with `"source.rejected"`.
* Command: `cargo test -p host-core --test prepare`
* Red: `source_control_errors_are_typed` — `left: "source.rejected"` vs
  `right: "source.region.outside"`.
* **Not sufficient on its own.** This gate only ever asserted one arm's string, so it caught a
  whole-table collapse and missed a single-arm one. The verifier proved that; M-27..M-31 below are
  the gate that actually pins the table.

### M-03 — drop the end-of-region symmetry rule
* Mutation: delete the `submission.end_of_region != (end == source.region_end)` check in
  `SourceControlSet::submit`.
* Command: `cargo test -p host-core --test prepare`
* Red: `source_control_errors_are_typed` — `left: Chunk(NonContiguous { .. })` vs
  `right: EndOfRegionMismatch` (the ring caught it later, with the wrong name).

### M-04 — the layout mirror reads the wrong type
* Mutation: in `control_table_bytes`, `Layout::array::<ControlSource>` → `Layout::array::<u64>`.
* Command: `cargo test -p host-core --test prepare`
* Red: `retained_bytes_projection_matches_the_live_set` — `left: Some(238)` vs `right: Some(22)`.

*M-04a, recorded green:* the first version of `SourceControlSet::retained_bytes` called
`control_table_bytes` itself, so mutating the mirror moved both sides of the assertion and the test
stayed green — a vacuous gate. `retained_bytes` now measures the live boxes with `size_of_val`, and
the two sides are independent witnesses. M-04 is the same mutation against the fixed gate.

### M-05 — `Exact` shape ignores the quantum
* Mutation: in `HostPrepareCaps::validate_shape`, drop the `compiled.quantum().0 != quantum_frames`
  disjunct.
* Command: `cargo test -p host-core --test prepare`
* Red: `shape_policy_pins_rate_and_quantum` — a session with quantum 128 is accepted under a policy
  demanding 64.

### M-06 — the per-source channel cap is ignored
* Mutation: delete the `caps.maximum_source_channels` check in `prepare_host_runtime`.
* Command: `cargo test -p host-core --test prepare`
* Red: `source_channel_cap_is_optional` — a stereo source is admitted under `Some(1)`.

### M-07 — the named-allocation cap rejects its own reported row
* Mutation: `largest_engine_allocation_bytes.max(..) > caps.maximum_named_allocation_bytes` → `>=`.
* Command: `cargo test -p host-core --test prepare`
* Red: `every_byte_cap_admits_its_reported_row_and_rejects_one_byte_below` —
  `largest_named: the reported value must be admitted`.

## F6 — the typed source-diagnostic table (verifier gap, closed)

The verifier found the F6 table itself unpinned: rewriting one arm of
`SourceControlError::diagnostic` to `"engine.invalid_argument"` survived the whole host-core suite
*and* the whole capi suite, because every gate asserted the error **value** and never the string a
C host actually reads. That is the exact collapse F6 removed, so it is now pinned three ways in
`tests/source_diagnostics.rs` (string, classification, reverse map) plus a compile-time
exhaustiveness guard, and once more end to end through the C entry points in
`capi/src/ffi.rs::source_rejections_reach_the_c_host_as_their_own_diagnostic`.

### M-27 — the verifier's mutation: collapse one arm to a generic code
* Mutation: `Self::UnknownSource => "source.id.unknown"` -> `"engine.invalid_argument"`.
* Commands: `cargo test -p host-core` and
  `cargo test -p capi source_rejections`
* Red twice:
  * `every_source_rejection_reports_its_own_pinned_diagnostic` — `row 0 (UnknownSource) must
    report its recorded code`, `left: "engine.invalid_argument"` vs `right: "source.id.unknown"`.
  * `source_rejections_reach_the_c_host_as_their_own_diagnostic` — `no source carries this ID`,
    `engine.invalid_argument` vs `source.id.unknown` as read back through `last_error`.
  (Before this commit the same mutation was green in both suites.)

### M-28 — merge one arm into a neighbour's code
* Mutation: `Self::UnknownSource => "source.region.outside"`.
* Command: `cargo test -p host-core --test source_diagnostics`
* Red: `every_source_rejection_reports_its_own_pinned_diagnostic` —
  `left: "source.region.outside"` vs `right: "source.id.unknown"`.

### M-29 — split a documented multi-variant arm
* Mutation: give `Seek(GenerationZero)` its own `"source.seek.generation.zero"` instead of sharing
  `"source.generation.zero"` with `GenerationZero`.
* Command: `cargo test -p host-core --test source_diagnostics`
* Red: `every_source_rejection_reports_its_own_pinned_diagnostic` — `row 5 (Seek(GenerationZero))`,
  `left: "source.seek.generation.zero"` vs `right: "source.generation.zero"`.

### M-30 — a variant joins the backpressure class
* Mutation: add `Self::OutsideRegion` to the `is_backpressure` pattern.
* Command: `cargo test -p host-core --test source_diagnostics`
* Red twice: `every_source_rejection_reports_its_own_pinned_diagnostic` —
  `row 2 (OutsideRegion) backpressure classification`, `left: true` vs `right: false`; and
  `classification_partitions_the_table` — `["source.region.outside", "source.backpressure",
  "source.seek.backpressure"]` vs `["source.backpressure", "source.seek.backpressure"]`.

*First attempt recorded green:* the same mutation written against reformatted source text never
matched and appeared green. The applied edit is now read back before the run — the same lesson as
M-16a.

### M-31 — collapse an arm *and* update the table to match
* Mutation: `Self::UnknownSource => "source.region.outside"` in `source.rs`, plus the matching row
  edit in `TABLE` — the careless "fix the test" regression the string pin alone cannot catch.
* Command: `cargo test -p host-core --test source_diagnostics`
* Red: `only_the_documented_pairs_share_a_code` — `source.region.outside is reported by rows
  [0, 2], but exactly 1 variant(s) may report it`. The other two tests stay green, which is
  precisely why the reverse map is a separate pin.

## W4-4 — the plan owns the sample clock

`PlanarBufferMut::plane_pair_mut` was written for W4-6 and is not in the delivered branch: W4-6 did
not land (below), and an unused public primitive with a gate attached is worse than no primitive.

### M-08 — `render_contiguous` drops the continuity compare
* Mutation: delete the `absolute_sample != self.next_absolute_sample` guard.
* Command: `cargo test -p engine render_contiguous`
* Red: `render_contiguous_rejects_stale_and_accepts_next` —
  `left: Ok(RenderReport { plan_id: 7, next_absolute_sample: 4, frames: 4 })` vs
  `right: Err(TimeDiscontinuity { expected: 4 })`.

### M-10 — the plan never advances its own clock
* Mutation: delete `self.next_absolute_sample = next;` from `render_inner`.
* Command: `cargo test -p engine render_contiguous`
* Red: `render_contiguous_rejects_stale_and_accepts_next` — `left: 0` vs `right: 4`.

## W4-5 — one validation pass at the C boundary

### M-11 — the platform bound on `sample_capacity` is dropped
* Mutation: in `miso_engine_v1_render_f32_planar`, `Ok(value) if value <= isize::MAX as usize / 4`
  → `Ok(value)`.
* Command: `cargo test -p capi render_rejections`
* Red: `render_rejections_name_their_single_check` — the `from_raw_parts_mut` debug precondition
  aborts the test process at `ffi.rs:779`. (This is `assert_unsafe_precondition!` →
  `panic_nounwind`; it is a crash, not a catchable panic, which is why the check must precede the
  slice.)

### M-12 — render rejections collapse onto one code again
* Mutation: map `RenderError::OutputShape` and `RenderError::TimeDiscontinuity` both to
  `plan_error::PLAN_REJECTED`.
* Command: `cargo test -p capi render_rejections`
* Red: `render_rejections_name_their_single_check` — `render.plan.rejected` vs
  `render.output.shape`.

### M-13 — the stereo ABI rule is dropped
* Mutation: delete `|| output.channels != 2` from the descriptor check.
* Command: `cargo test -p capi render_rejections`
* Red: `render_rejections_name_their_single_check` — a one-channel descriptor returns
  `RESULT_OK` (`left: 0` vs `right: 1`).

## W4-6 — direct output binding: not landed

The plan's second half of F4 -- the sequential executor reducing its output node straight into the
caller's planes instead of copying its arena buffer out -- was implemented and proven (three red
mutations) against the executor as it stood at branch point. #98 then merged and replaced that
executor with a driver over a lowered op program and a flat coloured arena indexed by buffer id.
Redirecting the output colour to caller storage there means threading an external destination
through `runtime::execute_op` for every node kind and both executors, which is a change to #98's
layer, not this job's. The work is reported as a §10 fallback on issue #103, with the mechanism a
successor needs, rather than carried as a mutation log for code that is not on the branch.

What *is* gated is that nothing else moved a rendered bit:

### M-16 — the class-A digest oracle reacts to a rendered bit
* Mutation: in `runtime::reduce_plane`, `sum2_block::<FrameLane>(output, a, b)` ->
  `sum2_block::<FrameLane>(output, a, a)`.
* Command: `cargo run --release -p audit -- capi`
* Red: `pcm_digest d412e33bcc063aef` instead of `774f0722585e918b`.

*M-16a, recorded green:* two weaker mutations left the digest unchanged and are recorded because
they bound what this oracle proves. Swapping `first` and `second` in the pairwise seed is exactly
commutative, so it cannot move a bit. Reversing the `rest` accumulation order *is* an
associativity change, but the audit fixture's nine tracks produce bit-identical contributions for
every input after the second, so the reversed order sums the same values. The audit digest is
therefore a strong oracle for "this refactor moved no rendered bit" and a weak one for reduction
order specifically; `scripts/check-graph-determinism.sh` and the graph fixture corpus are the order
oracles, and both are green and unchanged.

## W4-8 — the policy script itself

### M-17 … M-26 — `scripts/test-host-core-policy.sh`
Ten mutations, each proven to fail `scripts/check-host-core-policy.sh`: capi calling
`compile_session`; a non-exempt host calling `prepare_session_builtins`; a non-exempt host calling
`into_bound_with_source_set`; capi defining `IdentityProcessor`; a host implementing
`GraphRuntimeProcessor`; capi carrying the `MISOCTL` wire literal; capi defining
`ReplayEntryRecord`; the facade depending on `protocol`; the facade exporting a
`no_mangle` C symbol; the facade becoming a `cdylib`. The suite also asserts the positive case,
including that the one pending-conversion host (`hosts/host-web`, issue #106) is
exempt.

## Issue #140 A — the banked-effect live-control seam

Applied to the working tree, the named test run, the failure observed, the mutation reverted, in
the same session. Host: `x86_64` (Simd8 bank width), debug profile.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-14 | `LiveControlEffectBankStage::process` packs every lane at `packed[..staged]` instead of at that lane's own running offset | `rack/src/lib.rs` | `effect_live_controls::*` | RED (`two_lanes_of_one_bank_take_two_different_commands`: each lane carries exactly the command addressed to it) |
| 140-15 | the bank builder never takes a member's control channel (`.filter(\|_\| false)` after `effect_controls.remove`), so a banked lane silently keeps the live-control-free stage | `graph/src/runtime.rs` | `effect_live_controls::a_banked_effect_applies_each_lanes_own_command_and_no_others` | RED (`the commanded lane moved`) |


---

## Issue #146 — the canonical floating-point environment at the render entries

The gates are `crates/host-core/tests/fp_environment.rs` (the started-session entry) and
`crates/capi/src/runtime/tests.rs::fp_environment` (the C ABI entry). Both render the
same subnormal fade tail three ways -- caller clear through the entry, caller FTZ+DAZ through the
entry, caller FTZ+DAZ *behind* the entry -- so the claim "the guard normalises" is asserted next to
the control arm that proves the caller's FTZ moves this fixture at all.

Every mutation below was applied, run, recorded and reverted on the delivery host
(x86-64-v3, AMD Ryzen 7 9700X, rustc 1.97.1).

### M-146-1 — the guard is removed from the host facade's render entry

* Mutation: delete `let _fp_env = CanonicalFpEnv::enter();` from
  `StartedRenderSession::render_planar` in `src/render_session.rs`.
* Command: `cargo test -p host-core --test fp_environment`
* Result: **RED**

  ```
  assertion `left == right` failed: a caller's FTZ+DAZ must not reach a guarded render:
    2048 of 4096 words moved
  test result: FAILED. 2 passed; 1 failed
  ```

  Half of the rendered words move, because with DAZ set the whole subnormal input block reads as
  zero.

### M-146-2 — the guard is removed from the C ABI render entry

* Mutation: delete `let _fp_env = CanonicalFpEnv::enter();` from
  `miso_engine_v1_render_f32_planar` in `crates/capi/src/ffi.rs`.
* Command: `cargo test -p capi --lib fp_environment`
* Result: **RED**, and red at the *earliest* possible point: the plan's first block fails the
  session-start re-attestation and the entry returns `RESULT_RENDER_REJECTED` (8) with
  `render.fp_environment.invalid`, before any audio is produced.

  ```
  assertion `left == right` failed
    left: 8
    right: 0
  test result: FAILED. 1 passed; 1 failed
  ```

### M-146-2b — the guard *and* the re-attestation are removed together

Recorded because M-146-2 short-circuits on the attestation and therefore does not, by itself, show
that the digest comparison discriminates.

* Mutation: M-146-2, plus replace the first-block attestation condition with `if false`.
* Command: `cargo test -p capi --lib fp_environment`
* Result: **RED** on the bytes:

  ```
  assertion `left == right` failed: a caller's FTZ+DAZ reached a render through the C ABI entry
  test result: FAILED. 1 passed; 1 failed
  ```

### M-146-3 — the restore is removed from `Drop for CanonicalFpEnv`

* Mutation: empty the body of `impl Drop for CanonicalFpEnv` in
  `crates/lane/src/fpenv.rs`.
* Commands and results: **RED** in all three crates --
  `-p lane --test fp_env` 4 of 8 failed,
  `-p capi --lib fp_environment` 2 of 2 failed
  (`a refused descriptor leaked MXCSR`),
  `-p host-core --test fp_environment` 3 of 3 failed.

  This is the E2 mutation: it is red on the error path (`a refused descriptor leaked MXCSR`) as
  well as the success path, which is the half a success-only gate would miss.

### M-146-4 — the started session becomes movable between threads

* Mutation: in `src/render_session.rs`, replace the `PhantomData<*const ()>` field of
  `StartedRenderSession` with `PhantomData<()>`. The same edit applied to `CanonicalFpEnv` in
  `crates/lane/src/fpenv.rs`.
* Commands: `cargo test -p host-core --doc`, `cargo test -p lane --doc`
* Result: **RED**, and the two halves are red for different reasons, which is the fact worth
  recording:

  ```
  crates/host-core/src/render_session.rs - StartedRenderSession (line 47) - compile fail ... FAILED
  crates/lane/src/fpenv.rs - CanonicalFpEnv (line 250) - compile fail ... FAILED
  crates/lane/src/fpenv.rs - CanonicalFpEnv (line 255) - compile fail ... FAILED
  ```

  `CanonicalFpEnv` loses **both** claims: the marker is the only thing making it `!Send` and
  `!Sync`. `StartedRenderSession` loses only the `Send` claim -- its `!Sync` survives, because
  `PreparedRenderPlan` is already `!Sync` and the session owns one. The marker is therefore
  load-bearing for exactly one of the session's two claims, and the doctest that proves the other
  is proving the plan's property rather than the session's. Both are asserted, and neither is
  assumed.

## Issue #210 phase 3 — the live input trim and polarity, end to end

Driver: `cargo test -p host-core --test input_liveness_live_controls`, one mutation at a
time, tree restored between rows. The mutated code lives in `builtins-compiler` and
`graph`; the rows are logged there as P3-M21 through P3-M26. What this file records is
why the *end-to-end* form is the one that catches them.

Three of the four drain defects -- a missed member queue, an off-by-one lane, a constant lane --
are **invisible to the mix**. The fixture's eight tracks are identical and sum into one output, so
silencing any one of them produces the same sum, and a command that landed on the wrong track
renders bit-identically to one that landed on the right one. The per-track post-matrix meter is
what makes the bank lane index observable at all, and it is why
`a_command_moves_exactly_the_addressed_track` reads meters rather than samples.

The fourth -- draining in `process` rather than `begin_block` -- is invisible to *any* single-block
observation: the record still reaches the right lane, one block late relative to the collapse
dispatch. It shows only in the census, and only because `BankChain::run` reads the witness between
the drain and the gather.

### The disengage-under-drain window

One more defect class this file gates, and the only one the phase actually shipped and had to fix:
a per-lane record drained on the block a collapsed chain disengages. `BankChain::run` drains every
slot's `begin_block` before it reads the collapse witness and runs `disengage_collapse` after, so
the boundary is reached with the two channels' trim-ramp records legitimately apart — and a stage
that copies the whole per-channel state there clones the just-drained record onto the channel the
live controls did not address.

It is unreachable from every other test in these suites, because they all push their commands
before block 0 and the chain has therefore never collapsed when the record arrives. The gate is
`a_per_lane_record_drained_on_the_disengaging_block_reaches_one_channel`, which renders first and
pushes second, and it is red under `P3-M17` (`crates/builtins/tests/MUTATIONS.md`).

The oracle it uses is worth reading before trusting it: the **stereo-mapped** fixture never
collapses (its two lanes read different source channels) while carrying identical content on both,
so it is a genuine never-collapsed run of the same audio rather than a second run of the same path.
`the_stereo_arm_is_a_never_collapsed_oracle` asserts both halves of that — the mono arm collapses,
the stereo arm does not, and uncommanded they agree.

## Issue #1211 — one mute owner and live-control producers per strip

Driver: `cargo test -p host-core --lib solo`, `cargo test -p host-core --test strip_controls
--test strip_handles` and `cargo test -p host-core --test randomized`, one mutation at a time, tree
restored between rows. Delivery host: x86_64 AVX2+FMA.

| Row | Mutation | Red |
| --- | --- | --- |
| 1211-M1 | delete the solo-safe guard in `LiveControlSoloState::set_solo` | `a_solo_safe_strip_cannot_be_soloed` (a bus accepts a solo) and `rollback_restores_every_strip_including_the_solo_safe_ones` (the bus solo is no longer refused) |
| 1211-M2 | drop `!self.solo_safe(strip)` from `effective_mute` | `a_solo_safe_strip_keeps_its_user_mute_through_every_solo_transition` (the bus is solo-muted) |
| 1211-M3 | `rollback` restores `user_mute` only for entries that are not solo-safe | `rollback_restores_every_strip_including_the_solo_safe_ones` |
| 1211-M4 | the guard bumps `solo_count` for a refused bus solo before returning | `a_solo_safe_strip_cannot_be_soloed` (`any_solo` counts a solo-safe entry) |
| 1211-M5 | build the control requests from `live_control_tracks` (the track prefix) again | `a_bus_fader_record_renders_as_the_session_with_that_fader` (three producers) and `handles_list_tracks_then_submixes_and_file_bus_effects` (`left: ["t0", "t1", "t2"]`) |
| 1211-M6 | sort `strip_controls` by ID instead of by `canonical_index` | `a_bus_fader_record_renders_as_the_session_with_that_fader` (the bus producer is no longer at strip index 3) |
| 1211-M7 | `live_records` draws over `handles.track_count` instead of `strip_controls.len()` | `randomized_consoles_render_the_same_bits_armed_dual_and_serialized` (`bus_live_records: 0`) |

`a_bus_fader_record_renders_as_the_session_with_that_fader` also goes red when its own push is
moved to `strip_controls[2]` (a track's lane): the fader reaches `t2` alone and the output
differs from the twin's from the commanded block on.

## Issue #1221 — live send records from host-core

Driver: `cargo test -p host-core --features host-core/test-support --test live_routes`, one
mutation at a time, tree restored between rows. Delivery host: x86_64 AVX2+FMA. 1221-M8 and
1221-M11 mutate `graph`; the others mutate `host-core`.

| Row | Mutation | Red |
| --- | --- | --- |
| 1221-M1 | `RouteControlProducer::record` passes `[false; 2]` to `route_coefficients` instead of `source_lane_muted` | `the_live_target_is_the_prepared_constant` (the follow-zeroed column carries `gain * lr`) |
| 1221-M2 | `record` computes its target itself, `10^(gain_db / 20) * coefficient` with the same gating | `the_live_target_is_the_prepared_constant` (one ulp off on every coefficient) |
| 1221-M3 | `route_controls` collected in reverse | `a_settled_live_edit_through_host_core_equals_a_fresh_plan` (index 1 drives `e-x`, not `c-b`) and `live_send_handles_are_in_canonical_route_order` |
| 1221-M4 | `set` builds its record with a gain of `0.0` instead of `gain_db` | `a_settled_live_edit_through_host_core_equals_a_fresh_plan` |
| 1221-M5 | `push` drops the record `try_push` hands back and returns `Ok` | `a_refused_send_record_pushes_nothing` (set `depth + 1` returns `Ok(())`) |
| 1221-M6 | `record` clamps a too-long `length` to the maximum instead of refusing it | `a_refused_send_record_pushes_nothing` (`Ok(record)` with length `1 << 22`, not `Err(Length)`) |
| 1221-M7 | `set` pushes a muted step record when `record` refuses, then returns the refusal | `a_refused_send_record_pushes_nothing` (`free()` 3, not 4: a refusal pushed) |
| 1221-M8 | graph's `attach_route_controls` also takes the routes into the output (`*-main`) | `live_controls_cost_the_standing_sessions_no_fold` (`bank_route_folds()` 0 with `Some(64)`, 64 with `None`) |
| 1221-M9 | host-core drops the `route_control_resources` total from `admitted_graph_and_model` | `live_send_lanes_are_charged_against_the_graph_cap` (one byte below the admitted total prepares) |
| 1221-M10 | host-core attaches lanes of depth 1 when no depth was requested | `live_send_lanes_are_charged_against_the_graph_cap` (the idle report charges 2 routes, 1323 bytes) |
| 1221-M11 | graph's live-route `drain` boxes each record it applies | `live_sends_render_without_allocating`: the process aborts (`SIGABRT`) on the first render-thread allocation, before the counter assertion |

Gate 4's base behaviour was replayed by removing the attach call entirely: the test stays green
(64 folds with `None` and with `Some(64)`), so the fallback did not apply.
