# Render stored mute automation on the strip

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5, A2 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2. **It lands in the same push as #1652
*Follow an automated mute on following sends in render* and #1653 *Write a following send's route
cell from the shared commit*: #1651 must not reach `main` without them**, or a send that follows an
automated mute leaks.

## Product outcome

A producer's stored mute automation (row 6) plays on every host. A strip lane mutes and unmutes at
the exact samples the session names, over the session mute ramp. Solo (the browser's host
overlay) and VCA mutes still compose with the automated mute. A mute automation edit is a carried
rebuild; a static mute edit on an automated lane, and a browser live mute command on it, commit as
`model_only` on both hosts through the one shared rule of #1648. Sends that follow the strip's
mute are #1652 and #1653.

## Context

- **The mute composition.** `user_mute || vca_mute || (any_solo && !solo_safe && !soloed)` is
  written once, `LiveControlSoloState::effective_mute` (`crates/host-core/src/solo.rs:255-263`).
  *Admit browser live edits in the Worker through the committed model* (#1382) D3 composes the solo
  term in the shared commit from the browser's host overlay; the C ABI passes no overlay. *Deliver
  value-only VCA edits to the running C ABI plan* (#1247) D2 emits strip `Mute` records from the
  effective mute.
- **The mute stage.** `FaderRampStage::set_mute` retargets the lane's gain to 0 or to the
  remembered `fader_gain` (`crates/builtins/src/lib.rs:2773-2794`); the kernel clears a muted lane
  from the frame its ramp settles on (`crates/lane/src/kernels/builtins.rs:159-202`). #1638 D1
  adds `SetMute { muted, samples }` at an offset and `RememberGain`.
- **The cell.** *Hold live values in latest-target cells on both hosts* (#1312) D3 gives each mute
  lane a cell of two words (value bits, ramp).
- **Seam.** The fader and mute stage is seam-side (`crates/builtins-compiler/src/lib.rs:758-771`), so
  mute automation never declines a mono collapse.
- **The classifier row list and the predicate** are #1648 D1 and D3.

## Decisions frozen for this slice

- **D1. Mute cells (id 6).** Only `step` with values 0 or 1 (#1645 D1). A step curve is constant
  between its discontinuities, so a mute cell has no grid events, only jumps.
  - The stage mute of a strip lane is `curve || terms`, where `terms` is `vca_mute || solo_mute`.
  - **The terms word.** For an automated mute lane, this slice gives #1312's mute cell value word
    the meaning `terms` (the mute terms the session does not automate); for every other lane the
    word keeps #1312's meaning. Preparation seeds it. #1312 is not amended: this slice adds the
    meaning for a lane kind that exists only from this slice on.
  - At each discontinuity at render sample `r`, and at the first sample of the block after the mute
    cell changes, render composes the stage mute. Only a change emits #1638's `SetMute` at that
    offset, over the session mute length (#1639's `mute` word) for a curve event and over the
    cell's ramp word for a cell change. The caller reports the mute ramp to the cell's fader events
    through #1637's `hold_until`.
  - At a session seek where the seek reaches the fader node, the stage mute is set exactly (`n = 0`).
  - While a lane's mute ramp is in flight or the lane is muted, fader events only remember the gain
    (#1638 D2).
- **D2. Edits, one rule on both hosts.**
  - The classifier's row list (#1648 D1) gains row 6: a mute automation edit is a carried
    rebuild (#1648 D2 carries mute cells by the same address scheme). A static mute edit on an
    automated lane gives no record (#1648 D3); a browser live mute edit on it reaches that rule
    through the Worker's apply (#1382) and replies `model_only`. This turns red #1648's gate 1
    case "A mute (row 6) entry change gives `Ok` with no records"
    (`crates/host-core/tests/live_delta.rs`); this slice rewrites that case in place to expect
    `Err(LiveRebuild::Automation)` (AGENTS.md: a change that supersedes a test rewrites it in the
    same PR), and gate 4 below holds the new rule.
  - For an automated mute lane, the shared commit's mute composition (#1247's VCA mute, #1382 D3's
    solo overlay) writes the strip mute cell's `terms` instead of emitting a `Mute` record, and only
    when the terms' bits change. On the C ABI the overlay is absent, so `terms` is the VCA mute;
    the code is the same on both hosts.
- **D3. The acked-batch question: can an ack ever precede a drop? No.** Every check of a
  transaction (and of #1382's overlay) precedes the first cell write; writes cannot fail; render
  generates curve events from the plan.

## Deliverables

1. D1: mute events on the fader stage (both bank forms and the fused form), and the terms word's
   seed.
2. D2: the classifier row and the shared commit's terms write.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (the fader stage's mute events and the terms read),
  `crates/builtins/src/lib.rs` (only if #1638's mute operation needs a change)
- `crates/host-core/src/{prepare.rs,live_delta.rs,solo.rs}`, `crates/host-core/tests/live_delta.rs`
- `crates/control-plane/src/` (the overlay composition's terms write)
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`,
  `crates/control-plane/tests/mute_automation_realtime.rs` (new)
- `crates/control-plane/Cargo.toml` (a `bench-support` dev-dependency for that binary, only if an
  earlier slice has not added it), `Cargo.lock`

## Non-goals

- Sends that follow the mute, route lanes and the route op (#1652 and #1653).
- Options B, C and D of OQ1.

## Hazards

- **One push with #1652 and #1653.** Alone, this slice renders an automated mute while a following
  send keeps its prepared gate; the send leaks. Root merges the three together.
- **A redundant retarget moves bits** (README "Change only"; `crates/host-core/src/solo.rs:58-67`).
  D1 and D2 compare before they write or retarget.

## Objective gates

1. **Exact samples** (both hosts: `crates/capi/src/runtime/live_tests.rs` and
   `hosts/host-web/src/tests.rs`, new). Track `t`, with no following send, has a mute entry on
   `both` with steps 0, 1, 0, at samples that are not grid samples. Each mute ramp starts at its step
   sample; from the end of each ramp the output is bit-identical to a plan prepared with the same
   static mute, fed the same PCM. At quanta 128 and 100.
2. **Mute and fader compose** (`crates/builtins-compiler` test, new). A fader ride and a mute step on
   one lane: fader events inside the mute only remember the gain; after the unmute ramp the lane is
   on the ride, bit-identical to the stage driven directly with #1638's operations.
3. **Composition** (browser, new). Solo on another track mutes `t` by solo while its curve is 0;
   `t` mutes; unsolo restores the curve's state. A VCA mute of `t` composes the same way, on both
   hosts.
4. **Edits** (`crates/host-core/tests/live_delta.rs`, new). A mute entry change gives
   `Err(LiveRebuild::Automation)`; a static mute change on an automated lane gives no record; on the
   other, non-automated lane it gives its `Mute` record; a VCA mute of `t` gives one terms write and
   no `Mute` record for the automated lane. Browser: a live mute edit through the Worker's apply on the automated lane replies
   `model_only` and moves no bit.
5. **Realtime.** `crates/control-plane/tests/mute_automation_realtime.rs` (new integration binary in control-plane, the crate that
   holds the shared commit, `control_plane::SessionState`, #1309 D1-D9. A host-core binary cannot
   reach the commit without a dev-dependency cycle, since control-plane depends on host-core.
   `scripts/check-bench-policy.sh:257-280` allows a `bench-support` dev-dependency only in a
   `crates/` manifest, so no host-web binary can link it. The binary links `bench_support::alloc`
   and calls `assert_installed()` first). It drives the script through control-plane's shared
   commit and host-core's render session, the code the browser Worker (#1382) and the C ABI both
   run. For gate 1's script:
   `allocations == 0 && frees == 0` around every render call after warm-up.
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` reports all
   violation counts 0.
6. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-web --features host-web/test-support`,
     `cargo test --locked -p control-plane --features test-support`
   - the workspace debug leg (`test-debug-a`) and the DSP leg (`test-debug-b`) in
     `.github/workflows/qualification.yml`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/test-web-audioworklet.sh`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule),
     `bash scripts/run-aarch64-tests.sh debug`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
7. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if a mute step snaps to the grid, uses another row's length, or ramps where the static
  plan does not.
- Gate 2: red if a fader event restarts the mute ramp, which `set_fader_gain` does today.
- Gate 3: red if solo or a VCA mute overwrites the curve instead of composing with it.
- Gate 4: red if the mute row stays masked, a static edit or a live command fights the curve, or a
  VCA mute emits a record the next curve event would contradict.
- Gate 5: red if the mute events or the cell read allocate on render.

## Dependencies

Batch R2. Direct dependencies:

- #1648 *Classify fader automation edits as carried rebuilds* (the row list, the predicate and
  the carry; it brings #1646 and, through it, #1382 and #1247).
- #1639 *Hold the automation jump lengths in a plan cell* (the mute ramp length is its `mute`
  word).

#1638's `SetMute` and the cells of *Hold live values in latest-target cells on both hosts*
(#1312) arrive through #1648. #1652 and #1653 depend on this draft and land in the same push.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **OQ1 = C: a hand mute ORs.** The stage mute of an automated lane is `curve || terms`, and
  `terms` now includes the lane's own static mute: `static_mute || any reaching VCA mute || solo
  term`, the effective mute of `controls::effective` (C7) without the curve. So a `SET_CONTROL mute`
  on an automated lane is a live `terms` write, path `live`, not `model_only`. D2's "A static mute
  edit on an automated lane gives no record (#1648 D3)" and the browser sentence are replaced by
  this rule; gate 4 tests a hand mute that closes an open curve, and a hand unmute that leaves a
  closed curve closed. When the entry is removed, the lane's mute is the static mute, as before.
- **Solo is on both hosts** (C8, #1559): "On the C ABI the overlay is absent" is no longer true; the
  code is the same on both hosts.
- **Moved files.** `crates/host-core/src/solo.rs` is deleted; the composition is
  `controls::effective` (`crates/controls/src/effective.rs`). Authorized paths change to
  `crates/controls/src/effective.rs` (read) and `crates/host-core/src/live_delta.rs`.
- **The jump length** is `LiveRamps::for_row(Mute)` (#1637, amended).

## Amendment (controls coordinator, batch 10, 2026-10-14)

- **Automated VCA mutes** (root's F20 ruling) compose with this slice's `curve || terms` in
  #1655: the stage mute of a member lane gains `|| vca_curve(τ)` for each reaching VCA whose mute is
  automated. This slice's `terms` word keeps the static VCA mutes.

## Amendment (controls coordinator, batch 13b, root ruling)

- **A `SetMute` that changes nothing does nothing** (constraint). Today #1638's `SetMute { muted,
  samples }` reaches `mute_channel` (`crates/builtins/src/lib.rs`, through `apply_operation`; the
  live `set_mute` calls the same function), which always writes `muted`, retargets the lane to `0.0`
  (muted) or to the remembered `fader_gain` (unmuted) over `samples`, and sets `mute_ramp` when
  `samples` is not 0, whether or not the lane's mute state changed. So a `SetMute` that repeats the
  lane's state restarts a fader ramp in flight over the mute length (its shape changes), and its
  `mute_ramp` makes `gain_held` true for that window, so the fader events inside it are only
  remembered. The rule: a `SetMute` whose `muted` equals the lane's and channel's current mute state
  leaves the stage as it was (no retarget, `mute_ramp` and the ramp's remaining frames unchanged).
  D1's "only a change emits" still holds; this rule makes the stage itself safe against a repeated
  `SetMute`, whatever composed it. The change
  is in `crates/builtins/src/lib.rs`, which this slice's authorized paths already allow; its test
  goes in `crates/builtins/tests/fader_ramp.rs` (authorized).
- **Gate 2, new case.** On the stage driven directly with #1638's operations: (a) an unmuted lane
  with a fader `RetargetGain` in flight takes `SetMute { muted: false, samples: 240 }` at an offset
  inside the ramp, then a `RetargetGain` 10 samples later; (b) a lane in a mute ramp toward `0.0`
  takes `SetMute { muted: true, samples: 240 }` inside it. Each block is bit-identical to a twin
  that never received the repeated `SetMute`, and in (a) `gain_held` stays false. Red if a repeated
  `SetMute` retargets the lane or starts a mute ramp. Gate 7 holds that the change moves no bit of a
  session with no stored automation.

## Amendment (controls coordinator, batch 18 spec check, 2026-10-17)

Checked against `ad03780ee`. #1637, #1638, #1639, #1312, #1648 and #1650 have landed (batch 17,
PR #1712).

- **Anchors.** The mute composition is `controls::effective_strip`
  (`crates/controls/src/effective.rs:326-366`); `EffectiveStrip::mute` (`:198`) is
  `own || vca_mute || solo_term`. The stage: `set_mute` and `mute_channel`
  (`crates/builtins/src/lib.rs:3129-3155`), `apply_operation` (`:3167-3178`), `FaderOperation`
  (`:4947-4958`); kernels `crates/lane/src/kernels/builtins.rs:187` and `:273`. The cells:
  `FaderLane` and `FaderCells` (`crates/builtins-compiler/src/lib.rs:255-427`); the drain
  `drain_fader_controls` (`:3940`). Seam: `FaderBankProcessor::seam_side` (`:3433`). Classifier:
  `controls::automation_row_renders` (`crates/controls/src/automation.rs:383-387`),
  `rendered_automation_changed` (`:472-489`), step 3 (`crates/host-core/src/live_delta.rs:415-418`).
- **Not landed.** The batch 13b rule has not landed: `mute_channel` still writes and retargets on
  every call. It stays this slice's deliverable.
- **D1 on batch 17's code.**
  - Mute cells join the plan's one program (`prepare_fader_automation`, `FaderAutomationPlan`),
    after the fader cells, so a fader-only session builds the same program. A new
    `controls::MUTE_CELL_RULE` sits beside `FADER_CELL_RULE`: completion `End`,
    `JumpLength::Block` (the caller resolves `LiveRamps::for_row(Mute)` from the block's jump
    lengths, as `block_fader_jump` does), seek `Exact`.
  - The lane's slot (`LaneFaderAutomation`) gains the mute channels: cell index, `CellState`,
    `terms`, ramp word, adoption. Same `a(n)` (R17-2), same span from `set_block_time` (R17-3).
  - The prepared stage mute of an automated lane is `first start_value != 0 || terms`
    (`prepared_fader`, `:6777`). Preparation seeds `terms` with `EffectiveStrip::mute`.
  - **The terms word.** Under OQ1 = C, `terms` equals `EffectiveStrip::mute`. So the `Mute` record
    the shared commit already emits (only on change, with its ramp) is the terms write, and the
    cell's value word carries it unchanged. No new record, no new cell, no edit under
    `crates/engine/src/realtime/`. The drain (the bank's and the scalar oracle's `drain_controls`)
    gives a mute value of an automated channel to the slot: the slot stores `terms` and the ramp
    word, composes `target || terms` with the cell's current target (the curve state the stage
    has heard), and applies `SetMute` at the block boundary over the ramp word only when it
    differs (the `read_offsets` pattern). The block's curve events then compose with the new
    `terms`.
  - Curve events go through the one `FaderEventDriver` (one `FaderOperationSource` per block; the
    sink takes one gain and one mute operation per lane, channel and offset, gain first). A mute
    cursor pushes `SetMute` over the block's mute jump length only when `curve || terms` changes.
  - **Fix.** "The caller reports the mute ramp ... through #1637's `hold_until`" cannot hold:
    `CellEvents` borrows the state (#1647 constraint 2) and D1.2 is deleted (B17-R11). Instead, at
    each mute event at offset `o`, the driver ends that channel's fader piece at `o` and starts the
    next piece at `o + 1` with the hold the `SetMute` gives (muted: held to the block's end; an
    unmute over `M`: `E = o + M`), through the shipped D1.6 obligation and cut at `E`. A gain
    operation at `o` keeps the old hold (gain before mute). Bounded by the block's mute events; no
    buffer.
  - **Seeks.** A seek `Set` composes like any event. With the 13b rule, a `Set` that does not
    change the state leaves the stage as it is, so a ramp toward that state finishes (as #1647
    constraint 5 for a held fader). Root confirms this reading (report ruling 1).
  - **Scalar oracle (R17-4).** Same slot and driver. A strip with mute automation and no fader
    automation binds `AutomatedFaderProcessor` (`:7497`) too.
  - **CanonicalFpEnv (R17-7).** The mute path has no float arithmetic (values 0 and 1, then
    booleans), and preparation already runs inside the guard. No new guard.
  - **For #1652.** This slice makes the program `Arc` and each strip's mute cell indices readable
    by host-core preparation, so #1652's route lanes read the same table.
- **D2 on batch 17's code.**
  - Row 6 joins `automation_row_renders`; `rendered_automation_changed` then makes a mute entry
    change `Err(LiveRebuild::Automation)`.
  - **Fix.** The fader-only readers of that predicate test the fader row: `fader_automation` and
    `automated_cell` (`automation.rs:393-448`), `model_fader_offsets` (`effective.rs:488`),
    `mask_fader_offsets` (`mask.rs:137`) and B17-R14's `automates`
    (`crates/control-plane/src/control.rs:2613-2616`). Without it, a mute entry gives its lane a
    hand offset (`S`, `fader_offset`), a fader edit becomes an offsets write, and a mute-only
    rebuild is flagged `TransitionFallback`.
  - **The second bullet** is replaced by the terms word above: a hand mute (OQ1 = C), a VCA mute
    and solo give the lane its `Mute` record, as today; render ORs it with the curve. The commit
    code does not change.
  - **Carry (#1648 D2).** `continued_cells` bits 2 and 3 are the mute lanes (a new
    `controls::continued_mute_lanes`, set in host-core's `StripJoin`, `prepare.rs:5338`); graph
    passes the `u8` unchanged (a doc edit only). `LaneFaderAutomation::adopt` copies a mute
    channel's state, `terms` and ramp by address; a changed table or a moved `a(n)` repositions;
    the first block emits `SetMute` over the mute jump length only on a change. A lost mute cell:
    the join emits a `Mute` record of `next`'s effective mute (as `join_level_records`, `:4660`,
    releases a fader lane); the 13b rule makes it a no-op when the stage agrees.
  - **TransitionFallback and watermark.** Nothing new: a mute adoption never stretches a live
    move, so `automates` stays fader-only. A terms write is a live record; a mute automation edit
    is a rebuild whose jump lands at the adoption block.
  - **Superseded tests, rewritten in place:** `crates/host-core/tests/live_delta.rs:1411-1451`
    (#1648 gate 1's three mute asserts now expect `Err(LiveRebuild::Automation)`) and the mute
    branch of `a_fader_automation_edit_rebuilds_alike_on_both_hosts`
    (`hosts/host-web/tests/control_parity.rs:2846-2857`, `ModelOnly` becomes `Rebuild`).
  - Root rules whether this slice sets the control listing's `automated` flag on an automated
    mute row (#1650 D4; report ruling 2). Until then the flag stays fader-only.
- **Gates.**
  - Gate 1: both hosts in one binary, `hosts/host-web/tests/control_parity.rs` (batch 17's form),
    at quanta 128, 100 and 37, plus a session seek into a muted span (muted exactly from the seek
    sample). Widths: the same script in `crates/host-core/tests/stored_mute_automation.rs` (new)
    at `Simd8` (with `avx2`) and `Simd4`, split and fused, live and between render calls, and the
    scalar oracle: the same bits.
  - Gate 2 runs in `crates/host-core/tests/stored_mute_automation.rs`, against the stage driven
    directly with #1638's operations, with a new case: fader events inside a curve mute, and the
    move at `E` after a curve unmute, at quanta 128, 100, 64 and 37, the same bits. Red if the
    fader piece is not cut at the mute event. The 13b case stays in
    `crates/builtins/tests/fader_ramp.rs`.
  - Gate 3 runs in `control_parity.rs`, both hosts.
  - Gate 4: "one terms write and no `Mute` record" now reads "one `Mute` record (the terms
    write)". New cases: a mute entry on `t` gives `t` no hand offset and keeps `S`; a `SET_CONTROL
    fader_db` on `t`'s unautomated fader gives a `FaderDb` record, not an offsets write. Red if a
    fader-only reader sees the mute entry.
  - Gate 8 (new, carry; `stored_mute_automation.rs`): a changed curve, an added entry and a removed
    entry each rebuild and carry `t`'s stage; the lane jumps at the adoption block only when
    `curve || terms` changes; after removal it holds the static mute; the swap block allocates and
    frees nothing. A mute-only session's rebuild with a live fader move in flight completes
    `Exact`. Red if a mute cell restarts, misses or repeats the adoption jump, or a lost cell keeps
    the curve's state.
  - Gate 6: `scripts/check-realtime-policy.sh` is deleted; run `cargo build --locked --release -p
    realtime-policy && ./target/release/realtime_policy`. Add `cargo test --locked -p controls`
    and `cargo test --locked -p builtins`. `crates/control-plane/Cargo.toml` already has
    `bench-support` (#1649).
- **Authorized paths (this list replaces the earlier ones).** Render paths are marked (R). None is
  under `crates/engine/src/realtime/`.
  - (R) `crates/builtins/src/lib.rs` (the 13b rule in `mute_channel` only),
    `crates/builtins/tests/fader_ramp.rs`
  - (R) `crates/builtins-compiler/src/lib.rs` (mute cells, slot, seed, drain branch, driver, scalar
    oracle, adopt, export for #1652)
  - `crates/controls/src/{automation.rs,effective.rs,mask.rs,lib.rs}`,
    `crates/controls/tests/automation.rs`
  - `crates/control-plane/src/control.rs` (B17-R14's `automates` only)
  - `crates/host-core/src/prepare.rs` (the strip join only), `crates/host-core/src/live_delta.rs`
    (docs only), `crates/graph/src/lib.rs` (`GraphStripMove::continued_cells` doc only)
  - `crates/host-core/tests/{live_delta.rs,stored_mute_automation.rs}`,
    `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
    `hosts/host-web/src/tests.rs`, `crates/control-plane/tests/mute_automation_realtime.rs` (new)
  - `docs/SESSION_SCHEMA_V1.md` (`:290-305`, the mute row renders), `docs/REALTIME_MEMORY.md`
    (`:13`)
  - Removed: `crates/host-core/src/solo.rs` (deleted), `crates/control-plane/Cargo.toml`,
    `Cargo.lock` (no new dependency).

## Decision record: T1 (batch 18)

- **Shipped.** The batch 13b rule: `FaderRampStage::mute_channel` (`crates/builtins/src/lib.rs`)
  returns at once when `muted` equals the channel's current mute state. No retarget, and
  `mute_ramp` and the ramp's remaining frames stay as they were. One compare on a `bool` array
  element; no allocation, no new panic path. Every route reaches it: `apply_operation`'s `SetMute`
  and the live `set_mute`, on the Simd8 and Simd4 banks and the scalar track.
- **Gate (13b Gate 2, new case).** `crates/builtins/tests/fader_ramp.rs`,
  `a_repeated_mute_leaves_the_stage_as_it_was`, on every subject (Simd4, Simd8 with `avx2`,
  scalar), at quanta 128 and 100: (a) an unmuted lane with `RetargetGain` in flight takes
  `SetMute { false, 240 }` at offset 20 and a `RetargetGain` at offset 30; (b) a lane in a 300-sample
  mute ramp takes `SetMute { true, 240 }` at offset 20; (c) the live `set_mute` repeats the state
  between blocks. Each block, both planes, is bit-identical to a twin without the repeat; the held
  flag agrees, and in (a) `gain_held` stays false. Red on the base `mute_channel` (case (a), Simd4,
  q=128, block 0: the repeat restarts the fader ramp over 240 samples); green with the change.
- **Superseded tests, rewritten in place** (each used a repeated zero-window unmute to end an
  unmute ramp, the behaviour this rule removes):
  - `crates/builtins/tests/fader_ramp.rs::the_held_flag_follows_the_mute_ramp`: mutes the lane
    first, then unmutes over 40, so the unmute is a real change.
  - `crates/host-core/tests/stored_fader_automation.rs`:
    `a_live_write_that_frees_a_held_lane_releases_its_hold`,
    `a_live_write_that_frees_a_lane_with_nothing_owed_releases_its_hold` and
    `an_offsets_jump_at_the_entry_that_frees_the_lane_keeps_its_ramp` (ramping form) free the lane
    with a live `FaderDb` write (`live_fader`, `Act::fader`) instead. The R17-10 release depends on
    the stage's held state at the block entry, not on the record kind, so the gates keep their
    claims. This edit is outside the spec's authorized list (a test file of batch 17's slice); it is
    mechanical to the new rule.
- **Measured.** `audit capi` `pcm_digest` `bffc21e857b5e134` (base: the same), `allocations` 0.
- **Departures.** None from the rule's text.

## Decision record: T2 (batch 18)

- **Shipped (no rendered bit moves).** A new `controls::fader_row(target)` (strip, builtins, row
  5) is the fader-only predicate; `automation_row_renders` is today `fader_row(target)` and T3a
  adds row 6 to it alone. The fader-only readers now test `fader_row`.
- **Reader classification** (every reader of `automation_row_renders` at `0e078aad8`):
  - Fader-only, now `fader_row`: `fader_automation` (`automation.rs`; also feeds
    `continued_fader_lanes`, `ModelView::fader_automation`, `OwnerEdits::take_fader_automation`
    and builtins-compiler's per-strip cells), `model_fader_offsets` (`effective.rs`; feeds
    `EffectiveStrip::fader_offset`/`offsets_sum` and `mask_values`' automated lanes),
    `mask_fader_offsets` (`mask.rs`), and B17-R14's `automates` (`control-plane/src/control.rs`).
  - Any rendered row, unchanged: `rendered_automation_changed` (a mute entry change must rebuild,
    D2), and `automated_cell` (keyed by the address's own row: it already compares
    `target.control == address.control`, so `automated_fader_lanes`, its one caller, asks at
    `fader_db` and a mute entry never matches; at a mute address it will say where T3a's mute cell
    exists, which is the doc's claim).
  - Any rendered row, outside T2's files: `prepare_fader_automation`'s early return
    (`builtins-compiler/src/lib.rs:2095`). It is right for T3a (the program then also builds mute
    cells); until T3a it is the same predicate.
- **R18-4 (root, accepted).** B17-R14's window stays fader-only because a mute adoption never
  stretches a live move: a mute cell's adoption jumps over the mute jump length and has no level
  ramp in flight to land over `J`, so a successor that automates only mute rows completes `Exact`.
- **Gate.** `crates/controls/tests/automation.rs::a_mute_entry_reaches_no_fader_only_reader`: a
  `both` mute entry before a `left` fader entry on `vocal`; the fader cells are the fader entry's
  left lane only, `automated_fader_lanes == [true, false]`, the right lane has no hand offset and
  the same `S` bits as the session without the mute entry, and `mask_values` masks the right
  lane's static dB. Row 6 cannot render before T3a, so the test is green on the base too; its
  predicate asserts (`fader_row` refuses a mute target, every fader target renders) hold the split
  now. Probe (not committed): with row 6 added to `automation_row_renders`, the test stays green
  with the split and turns red with `fader_automation` (the mute entry takes the left cell) or
  `model_fader_offsets` (`[Some(0.0), Some(0.0)]`) reverted to the old predicate.
  `mask_fader_offsets` reverted is not observable: a validated mute entry holds `+0.0` offsets,
  so copying one moves nothing. The control-plane `automates` change has no T2 test (no mute row
  renders yet); T3a's gate 8 (a mute-only rebuild with a fader move in flight completes `Exact`)
  holds it.
- **Ran.** `cargo test --locked -p controls --all-features`, `-p control-plane --features
  test-support`, `-p host-core --features host-core/test-support`, `-p capi`: all pass. fmt,
  clippy (`controls`, `control-plane`, all targets and features) and the no-feature check pass.
- **Departures.** `automated_cell` is not changed (above). None else.
