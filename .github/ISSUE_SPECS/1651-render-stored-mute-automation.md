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
