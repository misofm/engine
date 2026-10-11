FAIL

# #1277 attempt 2 -- adversarial slice verdict

Range: `8c0deee2c..9994001ad` on `codex/d15-stream-a2` (7 commits). I reviewed the commit from an export at
`/tmp/claude-1002/v1277/tree` (git archive). I did not build, edit or commit anything in `/home/bl/misofm/wt-d15-a2`.

## Findings

### BLOCKER 1 -- the required `lint` job fails: `cargo doc` has broken intra-doc links

- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` (qualification.yml, lint job, step
  "Documentation") fails:
  `error: unresolved link to checked_fader_gain` and `unresolved link to lower_matrix_or_pan` at
  `crates/host-core/src/live_delta.rs:148`, then `error: could not document host-core`.
- Cause: 8e557130f moved the per-owner record functions to `prepare.rs` and removed the `use builtins::{..,
  checked_fader_gain}` and `use builtins_compiler::{.., lower_matrix_or_pan}` imports from `live_delta.rs`. The
  classifier doc (step 5) still links both names.
- Effect: the `qualification` required check is red, so the batch cannot merge. The attempt record lists fmt and
  clippy but not `cargo doc`.
- Fix: path-qualify the two links (`builtins::checked_fader_gain`, `builtins_compiler::lower_matrix_or_pan`).

### MAJOR 2 -- D3 is not followed: a followed mute is treated as live, so the strip carries and is not in the restart set

- D3 (frozen): "A value of this family is *live* when the classifier would emit a record for its change on a plan that
  attaches this owner's control kind. Every other difference is *prepared*."
- `classify_live_delta` refuses a mute change on a track that a `follows_mute` route in `next` sends from
  (`LiveRebuild::FollowedMute`, `live_delta.rs` step 5, `follows_mute_from`).
- `join_strips` (`crates/host-core/src/prepare.rs:2493`, rule at `:2516`) does not check this:
  `live_family = live && !vca_in_play && is Track`. Its doc (`:2486-2492`) lists only "track, no VCA, values in
  domain".
- Result: the mute change is `FamilyJoin::Live`, the stage carries, a `Mute` retarget is written, and the strip is
  not in `restarted_strips()`. By D3, D4 and D6 the fader/mute stage restarts and the strip is listed.
- Verified by a probe test in a scratch copy: the session is the gate-1 nine-track session, plus a submix `bus`, plus
  a `follows_mute: true` send from `eq0`.
  - B adds the muted track and mutes `eq0`.
  - Result: `classify_live_delta` returned `FollowedMute`, and the successor's `restarted_strips()` was `[]`.
    `[eq0]` was expected.
- Impact:
  - Today `LiveRamps` is a step (`mute_samples: 0`), so the audio of carry plus retarget equals a restart.
  - But the D6 restart set that #1324 consumes is wrong.
  - Once #1054 gives live mutes a non-zero ramp, the carried stage ramps the mute while the following route,
    prepared fresh, steps. That is the inconsistency #1053 G2's `FollowedMute` refusal exists to prevent: a shortcut
    that a later issue would have to repair.
- No test covers it.
- Fix: in `join_strips`, a mute change on a strip that a `follows_mute` route in `model` sends from makes the fader
  family `Prepared`. Move the `follows_mute_from` predicate beside `fader_records` (live_delta.rs is
  `control-provider`-gated) so classifier and join share it. Add a gate.
  - If root prefers the carry here, D3 needs an amendment first. The implementation must not decide it silently.

### MINOR 3 -- realtime-policy floors not raised (implementer item 7)

- This slice adds 11 marked regions: 100 on the base, 111 at `9994001ad`, in 27 files.
- The awk floors stay at 27 files and 96 regions (`scripts/check-realtime-policy.sh:79-80`). The gate's own comment
  says "Raising a floor is part of the change that adds a marker".
- STREAMS' standing exception (STREAMS.md:49-60) lets any slice edit the two awk floor lines and the self-test pad
  while #1446 is not on `main`. `tools/realtime-policy` does not exist in this tree.
- So this was a permitted step that was left undone. The gate passes, but deleting up to 15 regions would now go
  unseen.
- Raise both floors to 111/27 with the self-test pad, and name the edit in the spec.

### MINOR 4 -- `crates/engine/src/realtime/mod.rs` edited outside the authorized paths (implementer item 6)

- Amendment 3 authorizes only `plan.rs` in the engine ("No other engine change"). The 53 added lines are two tests of
  D11 in the existing `watermark` test module, beside the `take_revision_gate` tests. `plan.rs` has no test module.
- The tests are valuable: both go red when `stamp_held_gate` stamps `revision + 1`. The risk is low, and no hot-file
  row names `mod.rs`.
- Root should record the named exception (B's crate) in the spec and STREAMS rather than have the tests moved.

### NIT 5 -- a pan smoothing-window-only change carries the predecessor's window

- `matrix_record` ignores the window, so a window-only change is `Same`. `MatrixLaneState` imports the predecessor's
  `smoothing_samples` over the successor's prepared one.
- This is inert: bank retargets always pass their own window (`set_target_smoothed`), and the window-less
  `set_target` is per-track only.
- It is also the same as the live path, whose classifier masks `matrix_or_pan` and emits no record. A one-line note in
  the D7 doc would make it explicit.

### NIT 6 -- `a_preparation_without_a_retarget_stamps_nothing` is vacuous in release

It relies on `GatePin::stamp`'s debug assertion. CI runs host-core tests in debug only, so this is acceptable.

## Rulings on the implementer's reported deviations

1. **Record functions and `LiveRamps` in `prepare.rs`: acceptable.**
   - D5's substance holds: one per-owner function, which the classifier calls, with nothing duplicated.
   - `live_delta.rs` is `control-provider`-gated, but preparation needs the functions on every build. `live_delta.rs`
     only loses the moved code and calls it; `LiveRamps` is re-exported.
   - This move caused BLOCKER 1.
2. **Restart set in `PlanStateInventory`: acceptable.** It is charged through `inventory.retained_bytes()`, which
   `prepared_capi_resources` charges (`compile.rs:366`).
3. **Strip-move table checked after bind: acceptable.**
   - It is checked at its exact length against the same `maximum_graph_session_plus_plan_bytes` cap.
   - The measured `carry_program_retained_bytes`, which includes the strips (`debug_assert_eq!` ties it), feeds
     `validate_replacement_peak` (`compile.rs:710`).
   - A refusal drops the plan.
4. **Six re-pinned `carry_program_retained_bytes` expectations: acceptable.**
   - Each adds `strip_moves(n)` with its reason.
   - The number of stage moves is the claim, and the pattern is #1276's.
5. **`host_core::prepare::test_only_..._between_render_calls_{on,successor_on}`: acceptable.** They are behind
   `test-support` and `doc(hidden)`, in A's file, with no `lib.rs` edit.
6. **`engine/src/realtime/mod.rs` tests: MINOR 4.**
7. **Realtime floors: MINOR 3.** The standing exception applies, and leaving the floors unraised is a gap.

## Other checks (all hold)

- **D5 ordering and the acked-batch question.**
  - The retarget writes are preparation's last step (`prepare.rs:2157-2173`), after every fallible preparation step.
    A failure inside the loop (`host.control.order`) drops the plan.
  - In the control plane, every refusal after preparation drops the candidate, its epoch, its handle (with the open
    stamp) and its cells.
  - The handle's `stamp(revision)` is the last fallible step before `prepared.commit`. `publish(r)` precedes
    `reservation.commit()`.
  - The ack (`RESULT_OK`) follows the commit. **An ack cannot precede a drop.** Gate 8 confirms that a refused
    candidate is not reused.
- **D8.** `apply_pending` = `drain(LiveSnapshot::ALL)`, once per predecessor bank (moves sorted by predecessor
  location, then owner). It runs before any lane of that bank is exported. Fader and matrix are seam-side, so the
  input carry's `disengage_for_carry` is not needed.
- **D10 and D11.**
  - `SuccessorBase.revision` exists, with the entry check `host.successor.revision` (`PrepareRejection::Session`).
  - `stamp_held_gate` has the specified signature and the `PlanRevisionGate` doc change.
  - `control.rs` changes only the one `r` read with its range check (`CommandError::Internal`) before
    `prepare_runtime`, the `revision` field and the stamp's reuse of that value. This is within D10's scope.
  - `compile.rs` is untouched.
- **Gate 8 shape.** The gate meets Amendment 3's stronger shape: refused retargeting rebuild, then the same
  structural change without X's fader. The twin is bit-identical in all 8 blocks, and a third rig proves the
  retarget is audible.
- **Fused form (gate 3).** Every gate-2/4/5 case runs Split and Fused, and a cross-form pair restarts (D9).
- **Amendment 3 (`3e0449ae5`).** It applies root's r1277 ruling faithfully: the D5 stamp sentence, D10, D11, gate 8
  and gate 7's `cargo test -p capi`, the authorized paths, the non-goal, the hot-file rows (`plan.rs`, control-plane
  with B3, A's capi named exceptions) and #1323's constructor amendment with the revision-0 open question.
- **Person-role wording.** None added. "producer" appears only as the existing SPSC type and field names.

## Test value (one sentence per new test)

- builtins `a_fader_lane_imported_mid_fade_renders_what_the_exported_lane_renders`: catches an import that drops the
  countdown, ramp words, mute flag or remembered gain. It is the only test that catches the `muted` and `fader_gain`
  words.
- builtins `a_matrix_lane_imported_mid_ramp_renders_what_the_exported_lane_renders`: catches a matrix import that
  drops the countdown, the ramp target or the settled-flag resync.
- engine `a_held_gate_stamps_on_the_pin_the_handle_inherits`: catches `stamp_held_gate` stamping on anything other
  than the plan's own pin at the given revision, or stamping after hand-out.
- engine `the_handle_inherits_the_open_stamp_of_a_held_gate`: catches an open stamp that does not move with the pin
  into the handle.
- `strip_state_survives_a_swap_at_{eight,four}_lanes`: catches stages keyed by lane rather than strip ID, a stage left
  out of the program, or added or carried strips listed as restarted.
- `a_strip_behind_an_effect_still_carries_at_eight_lanes`: catches a chain walk that stops at the first non-builtin
  slot.
- `a_ramp_in_flight_finishes_after_the_swap_at_{four,eight}_lanes`: catches a lost `remaining`, an export before
  `apply_pending`, or a fused stage that does not carry.
- `records_in_the_pending_successor_apply_at_its_first_block_at_{four,eight}_lanes`: catches successor cells applied
  before the import, which then overwrites them.
- `a_live_difference_retargets_the_carried_stage_at_{four,eight}_lanes`: catches a live difference treated as
  prepared, a skipped retarget, or a retarget applied before the import.
- `a_prepared_change_restarts_the_stage_at_{four,eight}_lanes`: catches a VCA change treated as live, or a restart set
  that is not exactly `[X]`.
- `the_strip_carry_allocates_and_frees_nothing`: catches an allocation or free in the swap block's strip carry.
- `a_cross_form_pair_does_not_carry`: catches a split and fused pair that is paired.
- `a_changed_control_kind_carries_no_strip_stage`: catches a carry between plans with different control kinds.
- `a_successor_revision_its_gate_cannot_stamp_is_refused`: catches a removed D10 entry check.
- `a_preparation_without_a_retarget_stamps_nothing`: catches a preparation that stamps even when it writes no retarget.
- capi `a_refused_rebuild_with_a_carry_retarget_is_never_reused`: catches a refused candidate's retarget, cell or
  stamp that is kept and re-written into the next same-revision successor.

## Mutations re-done (scratch copy, each reverted)

| Mutation | Red |
| --- | --- |
| Fader `import_lane` zeroes `remaining` | `a_ramp_in_flight_*` (4, 8); builtins `a_fader_lane_imported_mid_fade_*` |
| `carry_strip_lanes` skips `drain_for_carry` | `a_ramp_in_flight_*`, `a_live_difference_*` |
| Retargets never written | `a_live_difference_*`; capi gate 8 (vacuity guard) and the existing `a_live_edit_while_a_candidate_is_pending_reaches_the_candidate` and `a_live_mute_survives_a_later_rebuild` |
| `live_family` ignores VCA | `a_prepared_change_restarts_*` only |
| Fused bank reports no strip lanes | `a_ramp_in_flight_*`, `a_live_difference_*`, `a_prepared_change_*` |
| `stamp_held_gate` stamps `revision + 1` | both engine tests, `a_live_difference_*`, `the_strip_carry_allocates_*`, capi gate 8 |
| D10 entry check removed | `a_successor_revision_its_gate_cannot_stamp_is_refused` only |
| Successor cells drained before the import | `a_live_difference_*`, `records_in_the_pending_successor_*` |
| Refused retargets kept per (predecessor plan, revision) and re-written into the next successor | capi gate 8 only (bit-identity assertion, `live_tests.rs:1305`) |

A first gate-8 mutation keyed only by revision across rigs on one thread also turned the existing
`a_live_mute_survives_a_later_rebuild` red. The per-plan-keyed version, which is the defect the gate names, is caught
by gate 8 alone.

## Gates run (export, `CARGO_TARGET_DIR` under `/tmp/claude-1002/v1277/target`, toolchain 1.97.1, x86-64-v3)

- **lint job, every step:**
  - Failed: Documentation (BLOCKER 1).
  - Passed: fetch, fmt, clippy `--all-features -D warnings`, workspace policy and mutations, session policy, env
    vocabulary, bench policy, test-support CI policy and mutations, scalar-oracle self-test, script reachability
    (passes once the export is a git repo), console fixture, builtins fixture-manifest mutations, bench
    preconditions, host-core policy, protocol policy, realtime policy (111 regions in 27 files), audit-leak,
    artifact-evidence-leak, trace validator, lane policy, unfused seal, rack/builtins/graph policy, effect-runtime
    policy and fixtures, conformance boundaries, PEQ render contract, release-shape self-test, npm publish modes,
    stem store, stem identity, the two sub-v3 refusals, AVX2+FMA probe.
- **test-release job, every step: pass.** lane/math/wasm-gates release, `filter_liveness`, `tail_contract`,
  `ramp_endpoint` (8 crates), `designer_total --ignored`, M3 FMA cfg, loom leg (`spsc_loom`). The router gives
  `math_closure=false`, so M1 and F1 do not run.
- **Gate 7: pass.**
  - `cargo test -p builtins -p builtins-compiler -p graph -p host-core` (test-support features): 619 passed.
  - `cargo test -p capi`: 103 passed.
  - `cargo test -p console-workload`.
  - Release `audit` with `trace-builtins-audit.sh` and `trace-builtins-graph-audit.sh`: PASS.
  - `check-cross-targets.sh`: PASS.
  - Also passing: `cargo test -p engine --lib`, `cargo test -p control-plane --features test-support`.
- **Worklet chain: pass.** `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`
  (private TMPDIR, empty afterwards).
- Not run: AArch64 (CI only, per the rules).
