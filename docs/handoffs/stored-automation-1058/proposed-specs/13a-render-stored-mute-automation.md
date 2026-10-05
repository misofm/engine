# Render stored mute automation on the strip

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5, A2 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2. **It lands in the same push as drafts 13b
*Follow an automated mute on following sends in render* and 13c *Write a following send's route
cell from the shared commit*: 13a must not reach `main` without them**, or a send that follows an
automated mute leaks.

## Product outcome

A producer's stored mute automation (row 6) plays on every host. A strip lane mutes and unmutes at
the exact samples the session names, over the session mute ramp. Solo (the browser's host
overlay) and VCA mutes still compose with the automated mute. A mute automation edit is a carried
rebuild; a static mute edit on an automated lane, and a browser live mute command on it, commit as
`model_only` on both hosts through the one shared rule of draft 10. Sends that follow the strip's
mute are drafts 13b and 13c.

## Context

- **The mute composition.** `user_mute || vca_mute || (any_solo && !solo_safe && !soloed)` is
  written once, `LiveControlSoloState::effective_mute` (`crates/host-core/src/solo.rs:255-263`).
  *Admit browser live edits in the Worker through the committed model* (#1382) D3 composes the solo
  term in the shared commit from the browser's host overlay; the C ABI passes no overlay. *Deliver
  value-only VCA edits to the running C ABI plan* (#1247) D2 emits strip `Mute` records from the
  effective mute.
- **The mute stage.** `FaderRampStage::set_mute` retargets the lane's gain to 0 or to the
  remembered `fader_gain` (`crates/builtins/src/lib.rs:2773-2794`); the kernel clears a muted lane
  from the frame its ramp settles on (`crates/lane/src/kernels/builtins.rs:159-202`). Draft 08 D1
  adds `SetMute { muted, samples }` at an offset and `RememberGain`.
- **The cell.** *Hold live values in latest-target cells on both hosts* (#1312) D3 gives each mute
  lane a cell of two words (value bits, ramp).
- **Seam.** The fader and mute stage is seam-side (`crates/builtins-compiler/src/lib.rs:758-771`), so
  mute automation never declines a mono collapse.
- **The classifier row list and the predicate** are draft 10 D1 and D3.

## Decisions frozen for this slice

- **D1. Mute cells (id 6).** Only `step` with values 0 or 1 (draft 02 D1). A step curve is constant
  between its discontinuities, so a mute cell has no grid events, only jumps.
  - The stage mute of a strip lane is `curve || terms`, where `terms` is `vca_mute || solo_mute`.
  - **The terms word.** For an automated mute lane, this slice gives #1312's mute cell value word
    the meaning `terms` (the mute terms the session does not automate); for every other lane the
    word keeps #1312's meaning. Preparation seeds it. #1312 is not amended: this slice adds the
    meaning for a lane kind that exists only from this slice on.
  - At each discontinuity at render sample `r`, and at the first sample of the block after the mute
    cell changes, render composes the stage mute. Only a change emits draft 08's `SetMute` at that
    offset, over the session mute length (draft 12's `mute` word) for a curve event and over the
    cell's ramp word for a cell change. The caller reports the mute ramp to the cell's fader events
    through draft 07's `hold_until`.
  - At a session seek where the seek reaches the fader node, the stage mute is set exactly (`n = 0`).
  - While a lane's mute ramp is in flight or the lane is muted, fader events only remember the gain
    (draft 08 D2).
- **D2. Edits, one rule on both hosts.**
  - The classifier's row list (draft 10 D1) gains row 6: a mute automation edit is a carried
    rebuild (draft 10 D2 carries mute cells by the same address scheme). A static mute edit on an
    automated lane gives no record (draft 10 D3); a browser live mute edit on it reaches that rule
    through the Worker's apply (#1382) and replies `model_only`. This turns red draft 10's gate 1
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
  `crates/builtins/src/lib.rs` (only if draft 08's mute operation needs a change)
- `crates/host-core/src/{prepare.rs,live_delta.rs,solo.rs}`, `crates/host-core/tests/live_delta.rs`
- `crates/control-plane/src/` (the overlay composition's terms write)
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`,
  `crates/host-core/tests/mute_automation_realtime.rs` (new)

## Non-goals

- Sends that follow the mute, route lanes and the route op (drafts 13b and 13c).
- Options B, C and D of OQ1.

## Hazards

- **One push with 13b and 13c.** Alone, this slice renders an automated mute while a following
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
   on the ride, bit-identical to the stage driven directly with draft 08's operations.
3. **Composition** (browser, new). Solo on another track mutes `t` by solo while its curve is 0;
   `t` mutes; unsolo restores the curve's state. A VCA mute of `t` composes the same way, on both
   hosts.
4. **Edits** (`crates/host-core/tests/live_delta.rs`, new). A mute entry change gives
   `Err(LiveRebuild::Automation)`; a static mute change on an automated lane gives no record; on the
   other, non-automated lane it gives its `Mute` record; a VCA mute of `t` gives one terms write and
   no `Mute` record for the automated lane. Browser: a live mute edit through the Worker's apply on the automated lane replies
   `model_only` and moves no bit.
5. **Realtime.** `crates/host-core/tests/mute_automation_realtime.rs` (new integration binary in host-core, which already has the bench-support dev-dependency,
   `crates/host-core/Cargo.toml:37`; `scripts/check-bench-policy.sh:257-280` bans that edge in any
   `hosts/` manifest, so no host-web binary can link it; it links `bench_support::alloc` and calls
   `assert_installed()` first). It drives the script through host-core's shared commit and render
   session, the code the browser Worker and the C ABI both run. For gate 1's script:
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
     `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`,
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

- Draft 10 *Classify fader automation edits as carried rebuilds* (the row list, the predicate and
  the carry; it brings draft 09a and, through it, #1382 and #1247).
- Draft 12 *Hold the automation jump lengths in a plan cell* (the mute ramp length is its `mute`
  word).

Draft 08's `SetMute` and the cells of *Hold live values in latest-target cells on both hosts*
(#1312) arrive through draft 10. Drafts 13b and 13c depend on this draft and land in the same push.
