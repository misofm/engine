# Render stored mute automation on the strip

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2. **It lands in one push with draft 13b *Let
following sends follow an automated mute*: 13a must not reach `main` without 13b**, or a send that
follows an automated mute leaks.

## Product outcome

A producer's stored mute automation (row 6) plays on every host. A strip lane mutes and unmutes at
the exact samples the session names, over the session mute ramp. Solo and VCA mutes still compose
with the automated mute. A mute automation edit is a carried rebuild; a static mute edit on an
automated lane is `model_only`; the browser refuses a live mute on an automated mute lane until
#1382. Sends that follow the strip's mute are draft 13b's.

## Context

- **The mute composition.** `user_mute || vca_mute || (any_solo && !solo_safe && !soloed)` is
  written once, `LiveControlSoloState::effective_mute` (`crates/host-core/src/solo.rs:255-263`).
- **The mute stage.** `FaderRampStage::set_mute` retargets the lane's gain to 0 or to the
  remembered `fader_gain` (`crates/builtins/src/lib.rs:2773-2794`); the kernel clears a muted lane
  from the frame its ramp settles on (`crates/lane/src/kernels/builtins.rs:159-202`). Draft 08 D1
  adds `SetMute { muted, samples }` at an offset and `RememberGain`.
- **Browser admission.** `COMMAND_MUTE = 4`, `COMMAND_SOLO = 9`, `COMMAND_VCA_MUTE = 17`
  (`hosts/host-web/src/lib.rs:833`, `:869`, `:939`). Solo and VCA mute compose `Mute` records for
  each changed lane (the coalescing pass before the follow pass, `:5215-5273`).
- **The cell.** #1312 D3 gives each mute lane a cell of two words (value bits, ramp). README
  amendment row #1312 D3: for an automated lane, the mute cell's value word holds the mute terms the
  session does not automate.
- **Seam.** The fader and mute stage is seam-side (`crates/builtins-compiler/src/lib.rs:758-771`), so
  mute automation never declines a mono collapse.
- **The classifier row list and the lane mask** are draft 10a D1 and draft 10b D1; draft 10b's
  `COMMAND_REASON_AUTOMATED` is the refusal reason.

## Decisions frozen for this slice

- **D1. Mute cells (id 6).** Only `step` with values 0 or 1 (draft 02 D1). A step curve is constant
  between its discontinuities, so a mute cell has no grid events, only jumps.
  - The stage mute of a strip lane is `curve || terms`, where `terms` is `vca_mute || solo_mute`,
    held in the strip's mute cell value word (#1312 D3 amended).
  - At each discontinuity at render sample `r`, and at the first sample of the block after the mute
    cell changes, render composes the stage mute. Only a change emits draft 08's `SetMute` at that
    offset, over the session mute length (draft 12's `mute` word) for a curve event and over the
    cell's ramp word for a cell change. The caller reports the mute ramp to the cell's fader events
    through draft 07's `hold_until`.
  - At a session seek where the seek reaches the fader node, the stage mute is set exactly (`n = 0`).
  - While a lane's mute ramp is in flight or the lane is muted, fader events only remember the gain
    (draft 08 D2).
- **D2. Edits.**
  - The classifier's row list (draft 10a D1) gains row 6: a mute automation edit is a carried
    rebuild (draft 10a D2 carries mute cells by the same address scheme). A static mute edit on an
    automated lane gives no record (draft 10a D3).
  - The browser's lane mask (draft 10b D1) gains `automated_mute`. Admission refuses
    `COMMAND_MUTE` whose channel covers an automated mute lane, with `COMMAND_REASON_AUTOMATED`,
    admitting nothing from the batch.
  - For an automated mute lane, the solo and VCA-mute passes write the strip mute cell's `terms`
    instead of staging a `Mute` record, and only when the terms' bits change.
- **D3. The C ABI has no solo**, and any VCA session rebuilds until #1247, so on the C ABI `terms`
  is always false and changes only by a rebuild. Nothing else differs between hosts.
- **D4. The acked-batch question: can an ack ever precede a drop? No.** Every check of a batch
  precedes the first cell write; writes cannot fail; render generates curve events from the plan.

## Deliverables

1. D1: mute events on the fader stage (both bank forms and the fused form).
2. D2 and D3: the classifier row, the lane mask, browser admission.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (the fader stage's mute events and the lane mask),
  `crates/builtins/src/lib.rs` (only if draft 08's mute operation needs a change)
- `crates/host-core/src/{prepare.rs,live_delta.rs,solo.rs}`, `crates/host-core/tests/live_delta.rs`
- `crates/capi/src/runtime/live_tests.rs`
- `hosts/host-web/src/lib.rs` (admission only), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/mute_automation_realtime.rs` (new)

## Non-goals

- Sends that follow the mute, route lanes and the route op (draft 13b).
- Live VCA edits on the C ABI (#1247). Options B and C of OQ1.

## Hazards

- **One push with 13b.** Alone, this slice renders an automated mute while a following send keeps
  its prepared gate; the send leaks. Root merges 13a and 13b together.
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
   `t` mutes; unsolo restores the curve's state. A VCA mute of `t` composes the same way.
4. **Edits** (`crates/host-core/tests/live_delta.rs`, new). A mute entry change gives
   `Err(LiveRebuild::Automation)`; a static mute change on an automated lane gives no record; on the
   other, non-automated lane it gives its `Mute` record. Browser: a batch with a kind 4 on an
   automated lane returns `RESULT_UNSUPPORTED` with `COMMAND_REASON_AUTOMATED` and admits nothing.
5. **Realtime.** `hosts/host-web/tests/mute_automation_realtime.rs` (new integration binary, links
   `bench_support::alloc`, calls `assert_installed()` first) runs gate 1's browser script:
   `allocations == 0 && frees == 0` around every render call after warm-up.
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` reports all
   violation counts 0.
6. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p builtins-compiler --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-web --features host-web/test-support`
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
- Gate 4: red if the mute row stays masked, a static edit fights the curve, or the browser admits a
  mute the next curve event would overwrite.
- Gate 5: red if the mute events or the cell read allocate on render.

## Dependencies

Batch R2. Direct dependencies:

- Draft 10a *Classify fader automation edits as carried rebuilds*.
- Draft 10b *Refuse browser live commands on automated fader lanes*.
- Draft 12 *Hold the automation jump lengths in a plan cell* (the mute ramp length is its `mute`
  word).

Draft 08's `SetMute` and the cells of *Hold live values in latest-target cells on both hosts*
(#1312) arrive through draft 10a. This draft amends #1312 D3 (README amendment row). Draft 13b
depends on this draft and lands in the same push; this draft must not reach `main` without it.
