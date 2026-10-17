# Keep a live level move's own length across a Reposition swap

Filed by root from batch 17 (#1648's split, root ruling B17-R13; controls coordinator, 2026-10-16).
It follows the "split a blocking gate early" practice: #1648 closes on its passing gates, and the
in-flight length case moves here.

## Product outcome

A live level move (a VCA move over an automated fader, or a hand move stored as the ride's offset)
that is still ramping when a `Reposition` swap adopts (a rebuild whose only change on that lane is
the ride's table) ends when it was due, with its own length, for every `remaining` (the frames
left in its ramp at the swap), including `remaining < J` (`J`, the fader jump length, 960 in the
gate's session). The revision's watermark then reports `OUTCOME_EXACT`. This issue replaces the
conservative fallback flag of B17-R14 with the exact behaviour and deletes that flag path.

## Context

- Since #1648 attempt 3 (U3 fold 2), the adoption rule (`settle_adoption`,
  `crates/builtins-compiler/src/lib.rs`) holds a free stage's running ramp again, compares the
  curve where that ramp ends with the stage's target, and lands a moved target over
  `max(J, remaining)` (a one-sample landing piece, then the rest of the block over `J`).
- The defect (verifier, batch 17, at 0fac00ee9): when `remaining < J` the landing is stretched to
  `J`. In the gate's setup (a 4,000-sample move from block 4, `a_level_move_across_a_table_change_
  keeps_its_length` in `crates/capi/src/runtime/live_tests.rs`) a move heard 24, 26 and 30 blocks
  has 928, 672 and 160 samples left; the rebuilt rig differs from the uninterrupted engine by
  4.8e-4, 3.1e-3 and 2.1e-3 (both table changes, VCA and hand moves).
- Current behaviour (B17-R13 and B17-R14): the move is stretched to `J`. The control plane marks a
  rebuild candidate `CandidateOutcome::TransitionFallback` before it publishes it when the
  rebuild is a transaction (not a declared discontinuity), the successor's model has at least one
  stored fader automation entry (`controls::automation_row_renders`), and the session's level
  move window may still ramp. The window (`LevelMoveWindow` in
  `crates/control-plane/src/control.rs`) folds every live commit's fader `FaderDb` and offsets
  records with a nonzero ramp, and every rebuild commit that changes some kept strip's effective
  fader dB or `S` (`controls::effective`, old model against new) with the ramp
  `max(fader_samples, the longest explicit ramp of the transaction)`, into one bound: the newest
  revision and the longest ramp. "May still ramp" is `watermark.revision < revision ||
  render_clock < watermark.first_sample + ramp` (a watermark read that gives up counts as "may").
  The watermark and the render clock are read before the reservation.
  So the watermark reports `OUTCOME_TRANSITION_FALLBACK`. This is conservative: it is also set
  when no move is stretched (any transaction rebuild of a session with fader automation while any
  level move may still ramp; see the #1648 decision record "the conservative fallback flag").
- First thing to find out, before the design work (root's note): why the stage target moves
  during a heard move (0.398 at 12 blocks, 0.335 at 24, against 0.3018). This may be a defect of
  its own. Measured while landing B17-R13 with a temporary probe: for every heard move on the
  moving ride, the stage's `fader_gain` at the swap is not the composed curve at the ramp's end.
  So the adoption rule's comparison always differs, and every heard move lands at the swap (over
  `remaining`, or `J`). The 4.6e-6 at 12 blocks heard comes from that landing recomputing the
  stage's ramp step.
- A second Reposition inside a stretched landing (B17-R13 tranche 5): a flagged rebuild also folds
  `(its revision, J)` into the window, so a second table-only rebuild inside the first one's
  landing over `J` is flagged too (gate
  `a_second_reposition_inside_a_stretched_landing_reports_a_fallback`). D2 deletes this with the
  rest of the flag.
- Further over-report: `may_ramp` reads the watermark's newest `first_sample`, so any later
  revision restarts the window (a non-level edit, or the flagged rebuild itself). A stream of live
  edits keeps it open after one level move.
- A cell's own jump of `J` that is still running at a Reposition swap and is landed over `J` again
  is specified D2 behaviour, not a defect: it has no user length, and a fresh plan with the new
  table also jumps over `J`. The exception is a stretched live move's landing, which is covered by
  the tranche-5 fold today and by D1 after this issue.
- MINOR 4 (from B17-R13, not gated): the comparison point `at = end` in `settle_adoption` has no
  gate. It is observable with a heard move on the flat ride (`S` moves, the curve does not), then
  a table whose step falls inside the remaining ramp. G2 can use that setup.

## Decision to make at filing

- D1. The deferred jump. At a `Reposition` adoption, a free stage's live ramp is not retargeted:
  it runs to its end as the uninterrupted engine runs it. The cell makes the D2 comparison and, if
  the target moved, the `J` jump at the first free sample after the ramp (the ramp's end), not at
  the block's first sample. A small API in `crates/automation` gives the cell's deferred jump
  (the node time at which it is due and the comparison it makes there), so the driver needs no
  second landing path; the one-sample landing piece and `max(J, remaining)` are deleted.
- D2. The conservative fallback flag of B17-R14 is deleted: `LevelMoveWindow`, `fold_level_move`,
  `rebuild_level_ramp`, `rebuild_moves_a_level` and the `set_outcome` call in
  `crates/control-plane/src/control.rs`, with `ObservedReservation::set_outcome` if nothing else
  uses it. The text that B17-R14 appended is removed from `crates/capi/src/abi.rs`, the C header
  comment, `docs/C_ABI_V1_QUALIFICATION.md` and the two d.ts files. Its gates
  (`a_reposition_swap_while_a_level_move_ramps_reports_a_fallback` and its host-web parity test)
  are replaced by G1.
- D3. A step of the cell's own at the swap block's first sample keeps the B17-R13 rule: the curve
  is compared there and the landing is over `J` (gate
  `a_step_at_a_reposition_swap_jumps_over_the_fader_length`).

## Objective gates

- G1. VCA and hand moves of 4,000 samples, heard 1, 12, 24, 26 and 30 blocks before a table-only
  rebuild (the ride split at 10,000, and a change in its last 2,000 samples): every block after
  the swap is bit-equal to the uninterrupted engine's (or, where the table's own values differ,
  to a fresh plan prepared with the new table and given the same move), allocation-free, adopted,
  and the rebuild's watermark advance reports `OUTCOME_EXACT`.
- G2. The comparison point (setup: a heard move on the flat ride, then a table whose step falls
  inside the remaining ramp): a heard move whose stage target equals the curve at its ramp's end
  keeps its own length with less than `J` left (red if the curve is compared at the block's first
  sample).
- G3. The B17-R13 step gate and the #1648 gates stay green; realtime policy count unchanged or
  explained.

## Dependencies

- #1648 (closed on its gates), B17-R13 and B17-R14 (the conservative flag this issue removes).

## Amendment (controls coordinator, batch 18, root ruling 2026-10-17)

- **Added item: gate or delete the owed discharge on a free fader write.** In
  `crates/builtins-compiler/src/lib.rs` (about `:1689` on `ad03780ee`), a free live write releases the
  hold and sets `self.owed = false`. Deleting that line leaves every test green (batch 18 T1 gate
  power check, `scratchpad/c18t1/`). Prove the discharge observable and gate it, or prove it
  unobservable and delete it. This slice reworks the same owed and hold machinery at a Reposition,
  so the item belongs here, not in a phase-2 placeholder.

## Amendment (controls coordinator, batch 19 spec check, 2026-10-17)

Anchors are on `323433f6a`. #1648 is closed; batches 17 and 18 are merged. Every number below was
measured with temporary probes in a detached worktree at `323433f6a` (removed after use).

### A0. Diagnosis: the stage target does not move; the comparison reads the wrong field (a defect)

- The stage's ramp target stays at the composed curve at the ramp's end: 0.301812 at every
  `remaining` (3872, 2464, 928, 672, 160), VCA and hand moves, both tables.
- `fader_gain` is not the ramp target. Inside a live move's ramp the cell's grid events are held
  (`held: true`, `crates/automation/src/events.rs:391-409`). The driver gives them to the stage as
  `RememberGain` (`crates/builtins-compiler/src/lib.rs:2026-2043`), and `RememberGain` writes
  `fader_gain` with no retarget (`crates/builtins/src/lib.rs:3184`). So `fader_gain` follows the
  ride (0.466 heard 1, 0.398 heard 12, 0.335 heard 24, 0.325 heard 26, 0.307 heard 30).
- `bank_hold` puts `fader_gain` in `ChannelHold` (`lib.rs:4032-4041`, field `:1438`), and
  `settle_adoption` compares the curve with it (`lib.rs:2196-2198`). On a moving ride it always
  differs, so every heard move lands at the swap.
- Fix F0 (smallest): for a free stage (`!held`), `ChannelHold` carries the stage's ramp target
  (a new `BuiltinFaderBank` accessor over the kernel's `target_gain`, `builtins/src/lib.rs:3196`,
  beside `fader_gain` at `:4639-4649`). A held stage keeps `fader_gain` (the gain an unmute lands
  on; correct there). Measured with F0: all 40 cases of the gate setup (VCA and hand, both tables,
  heard 1, 12, 24, 26, 30) are bit-equal to the uninterrupted engine over 44 blocks, adopted and
  allocation-free. At HEAD: 4.6e-6 (heard 12), 4.8e-4, 3.1e-3, 2.1e-3 (heard 24, 26, 30).
- Rows: the fader stage only. VCA and hand moves both (one offsets-jump path, R17-1), and any
  free ramp the cell holds (its own `J` jump or grid ramp). Mute cells are not affected (no gain
  comparison). #1655's VCA cells drive the same stage and inherit F0.
- MINOR 4 (`at = end`): with F0, G1 is red if the curve is compared at the block's first sample
  (measured: the same 4.8e-4, 3.1e-3, 2.1e-3). G2 as filed cannot be red: on the flat ride
  `curve_toward` stops before the step, so `at = start` and `at = end` read the same value. G2 is
  replaced (A3).

### A1. The owed discharge: observable, so keep it and gate it (G4)

- The line is now `crates/builtins-compiler/src/lib.rs:2104` (`HeldRampObligation::block`,
  `:2080-2111`): a free stage at a block entry with no move due.
- `owed` can be true there only when the stage held the gain through the previous block and a
  live write freed it at this entry (an exact live unmute). Without the line, the stale `owed`
  survives the free blocks and makes a later hold's end `E` land a move over `J`
  (`lib.rs:2243`, `moving`) when nothing was remembered in that hold.
- Measured on the C ABI (quantum 128): ride linear 0 to -6 dB over `[0, 1280)`, flat -6 dB to
  2320, then linear. Hand mute (ramp 256) at block 4, exact hand unmute (ramp 0) at block 8, hand
  mute (256) at block 12, hand unmute (200) at block 16, so `E` = 2248 with no grid event in the
  second hold. Reference: the same rig with only the second hold. With the line, bit-equal from
  block 9. Without it, the first difference is block 17 (2.7e-4).

### A2. D1 against the code (amended)

- F0 first. An unmoved target then takes no landing: the cell seeds its target and holds the ramp
  again (`settle_adoption`, `lib.rs:2174-2202`, already does this when `!differs`).
- A moved target on a free stage with a ramp in flight (and no step at the block's first sample):
  no landing at the swap. The cell calls `CellState::defer_jump(at)` with `at` the ramp's first
  free sample (`node_time_at(span, remaining)`). The existing deferred path
  (`events.rs:456-490`) gives a `Jump` over the block's `J` there, change only.
- One rule change in `crates/automation` (measured as needed): on a restartable cell
  (`no_restart` false) grid events before a pending deferral are held. Without it the grid point
  32 samples before the move's end retargets to the new curve (a grid ramp that completes after
  the ramp in flight restarts, `events.rs:395-396`), the move is cut short, and change only then
  drops the jump. A `no_restart` cell is unchanged.
- The deferral is cleared by `reposition` and a seek landing (both already), and by a new
  `CellState::clear_deferred_jump()` that the fader cell calls where the ramp it waits for is
  replaced or ends early: the free path of `offsets_jump` (`lib.rs:1346-1352`, a newer live move
  keeps its own length), the R17-10 release (`:1920-1926`) and the `release_hold_by` sites
  (`:1576`, `:1592`). Reason for the last two: a held grid would freeze the lane after an early
  unmute (the #1651 U1 MAJOR-1 defect).
- Kept: the ramped landing at the block's first sample over `J` when no ramp is in flight, when
  the stage is held, for D3 (a step at the block's first sample), and for the `Compare` backstop.
- Deleted: `long_landing` and its one-sample landing arm (`lib.rs:1898-1919`, `:1966-2009`) and
  `length = length.max(remaining)` (`:2191`). `settle_adoption` no longer returns a length.
- Prototype measured (F0 + `defer_jump` + the held grid): G2's setup (A3) is bit-equal to its
  reference at heard 12 (2464 left) and heard 30 (160 left), VCA and hand. At HEAD: 9.4e-3 and
  1.5e-3 from the swap block. With the prototype on, the capi `automated` tests (24), host-core
  `stored_fader_automation`, `stored_mute_automation`, `successor_swap`, `session_seek` and the
  `automation` and `builtins-compiler` tests stay green.
- No new state: `CellState::deferred` exists (`events.rs:102`, 32 B unchanged).

### A3. Gates (amended)

- G1 (unmoved target). The existing gate (`live_tests.rs:9059-9147`) becomes bit-equal (not 1e-3)
  for Pending, SameTransaction and heard 1, 12, 24, 26, 30; both tables; VCA and hand. Add: two
  table-only rebuilds 5 blocks apart while a 500-sample move ramps (the tranche-5 setup). Every
  rebuild's watermark advance reports `OUTCOME_EXACT` and no transition fallback. Simd8 on the
  C ABI. Simd4 and Simd8 in `crates/host-core/tests/stored_fader_automation.rs` through a carry
  harness with live offsets writes (the pattern of `carry_run`,
  `crates/host-core/tests/stored_mute_automation.rs:1129`). The scalar oracle has no plan swap
  (`ChannelHold` there, `lib.rs:8234`). Both hosts: the parity test at
  `hosts/host-web/tests/control_parity.rs:2933-3012` becomes "a hand move in flight reports
  `EXACT` alike on both hosts, same bits".
- G2 (moved target, replaces the flat-ride G2). Base ride flat at -3 dB (`flat()`); move -6 dB
  over 4,000 samples from block 4; rebuild to a ride flat at -5 dB to 6,000, then -3 dB; heard 12
  and 30; VCA and hand. Reference: a fresh plan whose table is -3 dB before the move's first free
  sample (4,512 here; compute it from the rig), -5 dB to 6,000, then -3 dB, given the same move.
  Every block is bit-equal to the reference, allocation-free, adopted, `OUTCOME_EXACT`. Red at
  HEAD (above). Case (c): a second live move committed after the swap, before 4,512: bit-equal to
  a fresh plan with the new table given both moves, from the second move's last sample on (red if
  the deferral is not cleared). Simd8 and Simd4 in host-core as for G1; the C ABI leg; one
  both-host parity case.
- G3 as filed. Named: `a_step_at_a_reposition_swap_jumps_over_the_fader_length`
  (`live_tests.rs:9186-9289`) and the #1648 carry gates in `automated_carry`. Also the G5
  automation corpus digests (`crates/automation/src/corpus.rs:245`,
  `tools/wasm-gates/tests/g5_native_corpus.rs`) do not move. Realtime policy count unchanged or
  explained.
- G4 (owed discharge, A1). The A1 setup in `stored_fader_automation.rs` with `acted` on every form
  (`FORMS`, `:2092-2100`: Simd8, Simd4, the between-render forms and the scalar oracle), and on
  both hosts in `control_parity.rs` (same bits). Test value: red if the discharge line is
  deleted.

### A4. D2 exact deletions

- `crates/control-plane/src/control.rs`: the `level_moves` field (`:649-651`);
  `LevelMoveWindow`, its `may_ramp`, `fold_level_move`, `rebuild_level_ramp`,
  `rebuild_moves_a_level` (`:654-735`); the B17-R14 comment paragraph (`:2621-2629`, keep the
  #1652 U2a paragraph); `automates`, `in_flight`, `level_fallback`, `moved`, `landing` and the
  `rebuild_level` value (`:2640-2672`; the outcome becomes `follow_into_six_word` alone); the fold
  after commit (`:2866-2870`); the live commit's `level_ramp` and its fold (`:3519-3539`,
  `:3555-3557`). `crates/control-plane/src/compile.rs:1483` (`level_moves: None`).
- Keep `reservation.set_outcome` (`control.rs:2840`) and `ObservedReservation::set_outcome`
  (`crates/control-plane/src/plan.rs:199-206`): #1713's six-word fallback uses them. Fix the doc
  at `plan.rs:199-200` to name #1713 only.
- The B17-R14 clause: `crates/capi/src/abi.rs:96-97`, `crates/capi/include/miso_engine_v1.h:183-184`,
  `docs/C_ABI_V1_QUALIFICATION.md:153-154`,
  `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:782-784` and its generated copy
  `sdk/src/browser/shipped-host.d.ts:782-784` (`scripts/check-sdk-generated.sh`). Keep the #1713
  clause. Doc mentions of the window: `crates/controls/src/automation.rs:397-398`, `:414-415`.
- Tests deleted: `a_reposition_swap_while_a_level_move_ramps_reports_a_fallback`
  (`live_tests.rs:9297-9368`), `a_second_reposition_inside_a_stretched_landing_reports_a_fallback`
  (`:9370-9414`), and
  `a_mute_curve_rebuild_while_a_level_move_ramps_completes_exactly_on_both_hosts`
  (`control_parity.rs:3014-3114`, its window is gone). The `outcome` helper
  (`live_tests.rs:9290-9295`) stays for G1.

### A5. #1655 (same batch, same fader stage)

- #1711 lands first. It removes the window that #1655 would otherwise have to extend for VCA
  level moves.
- #1711 assumes: (1) the adoption compares the stage's ramp target (F0), which does not depend
  on how the gain is composed; (2) the curve side of the comparison is the lane's composed gain at
  `at`. Today that is `FaderEventDriver::gain(value, S)` (`lib.rs:2196`). #1655 replaces it with
  its one composition `checked_fader_gain(vca_compose_db(own(at), S + Σ c_k(at)))`; (3) one
  deferral per lane channel, so one `Jump` fires at the free sample, composed with every cell at
  that node time. #1655 names which cell holds it on an event lane with no own fader cell, and
  gives its VCA cells the same grid-hold and clear rules.

### A6. Mapping onto the batch 17-18 code

- Fader cells and the one event driver: `FaderEventDriver::new` (`lib.rs:1755`), adoption at
  `:1898-1919`. Offsets cells and hand moves (`takes_offset`, R17-1): one `offsets_jump` path
  (`:1285`). Jump lengths: `BlockJumps` (`:1404`). Node time (R17-3): `take_block_time`
  (`:1044`).
- The scalar oracle (R17-4) shares the driver; no swap reaches it. It is in G4 only.
- R17-7, the classifier rows and carry, `LatestCell<7>`, `route_mix_column_ramps_block`, the
  #1653 commit write and mute `curve || terms`: not touched. Mute adoption stays
  `AutomatedMuteCell::reposition`.
- `controls::fader_row` and `automation_row_renders`: D2 deletes the only control-plane reader of
  `fader_row` for the window (`control.rs:2651`).
- The conservative TransitionFallback: B17-R14's is deleted (D2). #1713's stays.

### A7. Authorized paths

- Render (realtime policy regions): `crates/builtins-compiler/src/lib.rs`,
  `crates/automation/src/events.rs`, `crates/builtins/src/lib.rs` (one accessor).
- Control and docs: `crates/control-plane/src/control.rs`, `crates/control-plane/src/plan.rs`,
  `crates/control-plane/src/compile.rs`, `crates/controls/src/automation.rs` (doc only),
  `crates/capi/src/abi.rs`, `crates/capi/include/miso_engine_v1.h`,
  `docs/C_ABI_V1_QUALIFICATION.md`, `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`,
  `sdk/src/browser/shipped-host.d.ts`.
- Tests: `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
  `crates/host-core/tests/stored_fader_automation.rs`,
  `crates/host-core/tests/support/successor.rs` (only if the carry harness needs a live-write
  hook).
- This spec.
- Nothing under `crates/engine/src/realtime/`.

## Rulings (root, batch 19, 2026-10-17)

- **R19-1.** Confirmed: for a moved target with `remaining >= J`, the old ramp runs to its end, then
  the cell jumps over `J`. Today the ramp is retargeted over `remaining`; D1 changes that. A gate holds
  it (red if the ramp is retargeted at the swap).
- **R19-2 (shared with #1655).** The deferred jump acts on one composed `CellState` per output. K1
  (`crates/automation/src/compose.rs`, after this issue) reuses the decisions this issue adds; events.rs
  exposes them `pub(crate)`, no copy.
- **Two PRs.** This issue ships in batch 19a with K1 and #1654.

## Decision record: F1 (batch 19)

- **F0 shipped.** `BuiltinFaderBank::ramp_target(lane, channel)` (`crates/builtins/src/lib.rs`, over
  the kernel's `target_gain`) is new. `ChannelHold::fader_gain` is renamed `stage_gain`. `bank_hold`
  fills it with the ramp target on a free stage and with `fader_gain` on a held stage. The
  scalar oracle still gives `None`. `settle_adoption` compares the curve with `stage_gain`. No
  other behaviour changes: `long_landing`, `length.max(remaining)` and the B17-R14 flag stay
  (D1 and D2 are later tranches).
- **G1, C ABI (Simd8).** `a_level_move_across_a_table_change_keeps_its_length`
  (`crates/capi/src/runtime/live_tests.rs`) now asserts bit-equality with the uninterrupted engine
  over 44 blocks after the swap, not 1e-3. Timings: Pending, SameTransaction, heard 1, 12, 24, 26
  and 30. VCA and hand moves, both tables: 28 cases, all bit-equal, allocation-free, adopted.
- **G1, host-core (Simd8 and Simd4).** New
  `a_level_move_in_flight_across_a_table_only_rebuild_keeps_its_bits`
  (`crates/host-core/tests/stored_fader_automation.rs`, `test-support`), with its own carry
  harness `level_move_run` (the `carry_run` pattern: a plan exchange, a live offsets write at
  block 4, a successor prepared on the committed model with the move). Heard 1, 12, 24, 26 and
  30, VCA and hand moves, both tables: every block bit-equal to the uninterrupted engine,
  allocation-free, adopted at the swap block, the strip carries. `successor.rs` is not touched.
- **Red without F0** (`bank_hold` back to `fader_gain`, run once, restored): the C ABI gate fails
  at "a VCA move, the ride split at 10,000, Heard(12)" (first difference block 25, sample 79); the
  host-core gate fails (first difference block 28).
- **Not asserted in F1: `OUTCOME_EXACT`.** The B17-R14 window still flags the heard 24, 26 and 30
  cases (`a_reposition_swap_while_a_level_move_ramps_reports_a_fallback` is unchanged and green).
  The outcome assertion of G1 belongs to the tranche that deletes the flag (D2).
- **Departure: the two-rebuild case of A3 is not in F1.** Two table-only rebuilds 5 blocks apart
  around a 500-sample move (heard 2) cannot be bit-equal to the uninterrupted engine: the second
  rebuild comes after the move's ramp ends, the stage is then free and settled on a grid ramp, and
  the split tables' values differ from the original ride in `f32` rounding, so the cell jumps over
  `J` (probe: 1.4e-4 to 1.7e-4 with no move at all; 7.7e-5 with only the later rebuild). Its
  reference must be a fresh plan with the new table (G1's "where the table's own values differ"
  clause), or its claim is the outcome alone. Moved to the D2 tranche, which owns the outcome
  assertion and deletes the tranche-5 fallback gate it replaces.
- Gates run: `cargo fmt --all`; clippy `-D warnings` `--all-targets --all-features` on builtins,
  builtins-compiler, host-core, capi; `cargo check --locked --all-targets` (no features) on the
  same; tests of builtins, builtins-compiler, automation (all features), host-core
  `stored_fader_automation` (31), `stored_mute_automation` (13), `successor_swap` (140),
  `session_seek`, `browser_successor`, capi lib (209), host-web `control_parity` (30): all green.

## Decision record: F2 (batch 19)

- **D1 shipped (A2).** `crates/automation/src/events.rs`: new `CellState::defer_jump(at)`,
  `clear_deferred_jump()` and `deferred_jump()` (small `pub` methods on the existing `deferred`
  field; no new state, 32 B). One rule change: a restartable cell holds its grid events while a
  deferral is pending (`time < deferred`); a `no_restart` cell is unchanged. The existing deferred
  path in `next_in_run` fires the jump (a `Jump` over the block's `J`, change only, or always with
  a held event outstanding).
- `crates/builtins-compiler/src/lib.rs`: `settle_adoption` returns `bool` (land at the block's
  first sample over `J`), not a length. On a `Reposition` of a free stage with a ramp in flight
  and no step of the cell's own at the block's first sample, it holds the ramp again, compares the
  curve at the ramp's end with the stage's ramp target, and, if it differs, calls
  `defer_jump(node_time_at(span, remaining))` (no landing at the swap; R19-1). The landing at the
  block's first sample over `J` stays for no ramp in flight, a held stage, D3 and `Compare`.
  Deleted: `long_landing`, its one-sample landing arm, `length.max(remaining)` and the `jump` field
  of `AdoptionBlock`. `clear_deferred_jump()` is called on `offsets_jump`'s free path, at the
  R17-10 release, and at the two `release_hold_by` sites (through a new `release_lane_hold`).
  R19-2: the decisions are the `CellState` methods above, for K1 to reuse; `compose.rs` is not
  written here.
- **The B17-R14 flag stays** (F4 deletes it), so F2 asserts no `OUTCOME_EXACT`: the G2 and R19-1
  cases are inside the level-move window and still report the transition fallback.
- **G2 (A3).** C ABI (Simd8): `a_moved_target_jumps_at_a_live_moves_first_free_sample`
  (`crates/capi/src/runtime/live_tests.rs`). Host-core (Simd8 and Simd4):
  `a_moved_target_jumps_over_the_fader_length_at_the_moves_first_free_sample`
  (`crates/host-core/tests/stored_fader_automation.rs`; `level_move_run` now takes a list of
  `(block, record)` moves and writes a later one to the successor's cells). Flat ride -3 dB, move
  -6 dB over 4,000 from block 4, heard 12 and 30, VCA and hand, rebuild to -5 dB to 6,000 then
  -3 dB; reference: a fresh plan with -3 dB to 4,512, -5 dB to 6,000, then -3 dB, same move. Every
  sample bit-equal, allocation-free, adopted, the strip carries. Case (c): a second move -9 dB over
  1,200 from block 35 (ends 5,679, after 4,512 + `J`): bit-equal to a fresh plan with the
  rebuild's table given both moves from 5,679 on; host-core also asserts the lane still moves on
  the move's last sample (the length).
- **R19-1 gate.** `a_moved_target_lets_a_live_move_longer_than_the_jump_end_when_it_was_due` (C
  ABI and host-core, same names): heard 1 and 12 (3,872 and 2,464 left, both >= `J`), bit-equal
  to the fresh plan with the jump at 4,512.
- **Both hosts.** `a_moved_target_defers_its_jump_alike_on_both_hosts`
  (`hosts/host-web/tests/control_parity.rs`): hand move heard 3 blocks, rebuild to -5 dB; same
  verdicts, replies, watermark and bits over 60 blocks at the four rates.
- **Unit tests** (`events.rs`): `a_deferred_jump_fires_at_its_time_and_holds_the_grid_before_it`,
  `a_cleared_deferral_fires_nothing_and_holds_no_grid`.
- **Mutations (each run once, restored).** Land at the swap instead of deferring (the old
  behaviour): R19-1 red (C ABI and host-core, VCA move, heard 1) and G2 red (heard 12). No grid
  hold while deferring: G2 and R19-1 red (host-core, heard 12 and heard 1). No clear in
  `offsets_jump`: G2 case (c) red (host-core length assertion, VCA move, heard 12). Compare at
  the block's first sample (`at = end` removed): G2 stays green (on a flat table both points read
  -5 dB, as A0 found for the filed G2); G1 host-core is red (heard 1, the ride split at 10,000).
  So `at = end` is held by G1, not by G2.
- **F1 note checked: no.** Two table-only rebuilds 5 blocks apart (riding ride, R1 split at
  10,000 to -6 dB, R2 to -9 dB) against a fresh plan prepared with R2's table: with the 500-sample
  move (heard 2), bit-equal up to R2, then max 7.7e-5 (block 15, 4 blocks after R2); with no move,
  max 1.7e-4 after R1 (block 9) and 1.4e-4 after R2. Cause: at the adoption the stage's grid-ramp
  target (taken from the old table's values) and the new table's curve differ in `f32` rounding,
  so the cell jumps over `J` (deferred to the grid ramp's end, or at the block's first sample),
  where a fresh plan never jumps. Not fixed (outside D1); the D2 tranche's two-rebuild case needs
  an outcome-only claim or a phase-2 decision on rounding-only differences.
- Gates run: `cargo fmt --all`; clippy `-D warnings --all-targets --all-features` on automation,
  builtins, builtins-compiler, host-core, capi (and host-web `--all-targets`); `cargo check --locked
  --all-targets` (no features) on the five; tests: automation, builtins-compiler (all features),
  host-core `stored_fader_automation` (33), `stored_mute_automation` (13), `successor_swap` (140),
  `session_seek`, `browser_successor`, capi lib (211), host-web `control_parity` (31): all green.
  Realtime policy: 168 regions (unchanged).

## Decision record: U1 fold 1 (batch 19)

- **Base.** The fold worktree is at `202724b74` (K1), which does not contain F3/F4 (`13abb0cc0`,
  `1872b54c2`, on `codex/controls-batch-19f` at `14027678f`); the two merge cleanly (merge-tree
  `00c6abf3d`, and this fold's diff applies to it). MAJOR-1's fix, its host-core gate and MINOR-2
  are folded here. The C ABI and both-host MAJOR-1 legs, MINOR-1 and the NIT edit files that F3/F4
  rewrote and are left for the merged base.
- **MAJOR-1 fixed (A2 clear list).** `crates/automation/src/events.rs`, `next_in_run`: a
  discontinuity jump on a restartable cell drops a pending deferral (the jump compares the curve
  with the target and replaces the ramp the deferral waits for; the stale deferral's grid hold
  froze the lane on the step's target until the deferral's time). A `no_restart` cell is
  unchanged. The A2 clear list is now: `reposition`, a seek landing, `clear_deferred_jump()` (the
  free path of `offsets_jump`, the R17-10 release, the `release_hold_by` sites) and a
  discontinuity jump of a restartable cell. `defer_jump`'s doc says so.
- **Effect on K1.** A value cursor's own discontinuity jump inside a column-jump deferral now drops
  that cursor's deferral (its grid moves again after its own jump, held by its own ramp by the
  cell rule) instead of holding its grid to the column ramp's end. The other cursors keep their
  deferrals. K = 1 still equals `CellEvents` (one rule); the K1 tests and the allocation test are
  green unchanged.
- **MAJOR-1 gate (host-core, Simd8 and Simd4).**
  `a_table_step_inside_a_deferred_moves_ramp_drops_the_deferral`
  (`crates/host-core/tests/stored_fader_automation.rs`): VCA and hand moves, heard 1 and 4, the
  rebuild `[-5 dB step 0..2,000; linear -8 to -14 dB 2,000..8,000; -14 dB]` against a fresh plan
  with -3 dB before 2,000: every sample bit-equal, allocation-free, adopted, carried. Red without
  the fix: first difference sample 2,944, max 2.9e-2 (all four cases).
- **MINOR-2 gate (host-core, Simd8 and Simd4).**
  `a_hand_mute_inside_a_pending_deferral_lands_the_lane_on_the_new_table_at_the_unmute`: G2
  heard 12 (swap 2,048, F 4,512), hand mute at 2,560, unmute at 3,072 or 5,120, ramps 200 and 0.
  `level_move_run_muted` adds live mutes to the carry harness. Inside: the rebuild's table is a
  linear -5 to -8 dB to 6,000 (a flat table hides a held grid), and from the mute's end on every
  sample is bit-equal to a fresh plan with that table. After F (G2's flat table): bit-equal to the
  deferred-jump reference everywhere. Mutation (once each): the R17-10 release without its clear
  is red (exact unmute); with all three builtins-compiler clears removed the ramped unmute stays
  green, because its owed landing at `E` is a seek landing that clears the deferral. The
  `release_lane_hold` sites are not reached by this setup.
- Gates run (own target dir: the shared one held another worktree's `graph` build): `cargo fmt
  --all`; clippy `-D warnings --all-targets --all-features` and no-feature `cargo check
  --all-targets` on automation and host-core; tests: automation (43, compose included),
  builtins-compiler (all features), host-core `stored_fader_automation` (35),
  `stored_mute_automation` (13), `successor_swap` (140), capi lib (211), host-web
  `control_parity` (31): green. Realtime policy: 171 regions (unchanged).
