# Render stored pan and matrix automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.4, A1.5 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2.

## Product outcome

A producer's stored pan moves (row 12) and matrix moves (rows 7-10) play on every host. Pan stays
equal-power: at each ramp's completion sample the strip's 2x2 equals `pan_matrix` of the curve
values exactly, and it is linear between. A jump lands on its exact sample. A pan or matrix
automation edit is a carried rebuild, and a static edit of an automated pan position or coefficient
is `model_only`. On the C ABI, a static edit of a cell the session does not automate, in an
automated group, is a carried rebuild. The browser refuses a pan or matrix command on an automated
group until #1382. Live edits of the group's other cells are draft 14b's.

## Context

- **The matrix stage.** `MatrixStage` holds the ramp words and a per-lane countdown
  (`crates/builtins/src/lib.rs:2884-2894`). `set_target_over` retargets one bank lane, one division
  per coefficient (`:3008-3049`). The kernel `matrix2x2_ramp_block` advances each lane by its own
  additions, so a lane's ramp is partition-invariant (`crates/lane/src/kernels/builtins.rs:425-470`).
- **Pan.** `pan_matrix(left, right)` designs the equal-power 2x2 in `f64` through `math::cos` and
  `math::sin`, one cast per word (`crates/builtins/src/lib.rs:4363-4378`); `left` pans the left
  input (`ll`, `rl`), `right` the right input (`lr`, `rr`). `lower_matrix_or_pan` is the one
  lowering (`crates/builtins-compiler/src/lib.rs:4925-4940`). The model is `MatrixOrPan`
  (`crates/session/src/model.rs:655-680`).
- **Bank processors.** `MatrixBankProcessor` (`crates/builtins-compiler/src/lib.rs:778`, impl
  `:787`) drains its lanes at block entry (`drain_matrix_controls`, `:1108`). The fused
  `FaderMatrixBankProcessor` (`:831`, impl `:1172-1261`) runs fader and matrix in one traversal only
  through `try_process_settled_with_matrix`, which returns false while any ramp runs
  (`crates/builtins/src/lib.rs:3926-3935`); draft 08 D5 makes it decline a block with any
  operation. Test-support counters record which path ran.
- **The seam.** The matrix is seam-side: "the 2x2 matrix **is** the seam"
  (`crates/builtins-compiler/src/lib.rs:817-829`; `SEAM_SIDE_WITNESS`, `:391-398`). A collapsed track
  duplicates its plane into the matrix stage, whose two channels' words are free to differ.
- **The classifier** emits one matrix record when a lowered coefficient's bits change
  (`crates/host-core/src/live_delta.rs:308-313`).
- **The cell.** #1312 D3: one matrix cell of five words (four coefficients, ramp). README amendment
  row #1312 D3: for an automated group the matrix cell holds the pan positions or the coefficients
  the session does not automate (the group cell).
- **Browser kinds.** `COMMAND_PAN = 1` retargets both pan positions; `COMMAND_MATRIX = 2` all four
  coefficients (`hosts/host-web/src/lib.rs:827`, `:829`).
- **The lane mask and reason** are draft 10b D1-D2 (`COMMAND_REASON_AUTOMATED`).
- *Keep every trim, fader and matrix ramp inside its endpoints* (#1408) changes the ramp kernels'
  last-step law; it is a per-sample rule inside the kernel, so it does not change where events go.

## Decisions frozen for this slice

- **D1. In-block operations on the matrix stage**, on draft 08's model: per bank lane, at an offset,
  "retarget to a 2x2 over `n` samples" and "set a 2x2 exactly". Between operations the existing
  kernels run; operations at one offset apply in lane order. The stage is class A: a block with an
  operation at offset `o` equals the same block split at `o` with the operation at the second part's
  entry, bit for bit, at `Scalar`, `Simd4` and `Simd8`.
- **D2. The matrix stage is a target group** (A1.5).
  - **Pan** (row 12, per lane). At each event the stage's target is `pan_matrix(left, right)`, each
    position from its curve when automated, else from the group cell.
  - **Matrix** (rows 7-10, `both`). At each event the target takes each automated coefficient from
    its curve and every other coefficient from the group cell, then `Matrix2x2::checked`.
  - Preparation seeds the group cell with the session's static values for the cells it does not
    automate (#1312 D3 amended).
  - Grid period and ramp 64, completion `End` (`τ + 63`): the kernel assigns the target exactly on
    the ramp's last sample. Jumps ramp over the session pan length (draft 12's `pan` word). The
    strip's `smoothing_samples` is a live-record rule (#1054 D5) and is not read here.
  - A seek where it reaches the matrix node sets the target exactly.
  - Events of the cells of one group at one offset make one target, so the group retargets at most
    once per offset. Only a target whose bits change retargets.
- **D3. Edits.**
  - The classifier's row list (draft 10a D1) gains rows 7-10 and 12: an edit of their automation is
    a carried rebuild; cells carry by draft 10a's address.
  - A static edit of an automated pan position or coefficient gives no record (draft 10a D3).
  - A static edit of a cell the session does not automate, in a group that has an automated cell,
    returns `LiveRebuild::Automation` on the C ABI: it is a carried rebuild. Decision 14 rule 2
    makes a value live only when the plan already has a slot for it. The slot for this value is the
    group-cell write that draft 14b adds; in this slice the group cell holds only preparation's seed,
    and the matrix record would overwrite the automated columns. Draft 14b makes the edit a live
    group-cell write.
  - The browser refuses `COMMAND_PAN` and `COMMAND_MATRIX` on a strip whose matrix group has any
    automated cell, with `COMMAND_REASON_AUTOMATED`, admitting nothing from the batch: a 48-byte
    command sets every cell of the group, the automated ones included. The lane mask (draft 10b D1)
    gains `automated_matrix`. #1382 replaces this with the shared commit's group rule.
- **D4. Mono collapse.** The matrix stage is seam-side (Context), so pan and matrix automation,
  symmetric or not, never declines a track's collapse. (README A1.5's collapse rule applies to the
  input rows only; drafts 15 and 16a apply it.)
- **D5. The fused path** runs only on a block in which neither stage has an event or a ramp in
  flight; any event sends the block to the split fader and matrix path.
- **D6. The acked-batch question: can an ack ever precede a drop? No.** Curve events come from the
  plan; an edit of an automated group either gives no record or rebuilds; the browser refusal
  precedes any admission.

## Deliverables

1. D1 in `crates/builtins` and the matrix bank processors.
2. D2 and D5 in the matrix and fused processors, and preparation's group-cell seed.
3. D3 in the classifier.
4. D3's browser refusal and the lane mask in host-web.

## Authorized paths

- `crates/builtins/src/lib.rs` (the matrix stage), `crates/builtins/tests/matrix.rs`
- `crates/builtins-compiler/src/lib.rs` (the matrix and fused processors, and the lane mask)
- `crates/lane/src/kernels/builtins.rs` (a range entry only, if the split needs one)
- `crates/host-core/src/prepare.rs` (group cell seed), `crates/host-core/src/live_delta.rs`,
  `crates/host-core/tests/live_delta.rs`
- `crates/capi/src/runtime/live_tests.rs`
- `hosts/host-web/src/lib.rs` (admission only), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/pan_automation_realtime.rs` (new)

## Non-goals

- The live group-cell edit (draft 14b).
- Send matrix automation (OQ2). Options B and C of OQ1. A pan law other than `pan_matrix`.

## Hazards

- **A redundant retarget moves bits** (README "Change only"). D2 compares the whole target.
- **`set_target_over` adopts the window as the lane's own** (`:3008-3012`); no production path reads
  that window after this slice, but a test that calls `set_target` after an event sees it.

## Objective gates

1. **Flat equals static** (C ABI and browser, new). A flat pan entry at `(-0.25, 0.75)` renders the
   bits of a plan prepared with that static pan; the same for a flat matrix entry.
2. **Equal power at completion** (`crates/builtins-compiler` test, new). A linear pan ride: at every
   grid completion sample the stage's four coefficient words equal `pan_matrix` of the curves'
   values at that sample, bit for bit.
3. **One automated position** (C ABI, `crates/capi/src/runtime/live_tests.rs`, new). Left position
   automated, right static: the right columns (`lr`, `rr`) equal `pan_matrix`'s for the static value
   at every completion sample.
4. **Jump** (`crates/builtins-compiler` test, new). A `step` pan segment at a sample that is not a
   grid sample starts its ramp exactly there; with `control_smoothing` explicit `pan_ms` 0 it is an
   exact step.
5. **Partition identity** (`crates/builtins/tests/matrix.rs`, new). D1's identity at `o` in
   `{1, 63, 64, 127}` (`q = 128`) and in a block of 100, at `Scalar`, `Simd4` and `Simd8`.
6. **Fused path** (`crates/builtins-compiler` test, new). On the fused form, the fused counter
   advances on a block with no event and no ramp, and the fallback counter on every block with one.
7. **Edits** (`crates/host-core/tests/live_delta.rs`, new). A pan entry change gives
   `Err(LiveRebuild::Automation)`; a static change of the automated position gives no record; a
   static change of the other position gives `Automation`.
8. **Browser refusal** (`hosts/host-web/src/tests.rs`, new). `COMMAND_PAN` on a strip with an
   automated position returns `RESULT_UNSUPPORTED` with `COMMAND_REASON_AUTOMATED` and admits nothing
   from its batch; `COMMAND_PAN` on a strip with no pan automation is admitted.
9. **Realtime.** `hosts/host-web/tests/pan_automation_realtime.rs` (new integration binary; links
   `bench_support::alloc`, calls `assert_installed()` first): `allocations == 0 && frees == 0`
   around every render call after warm-up; `cargo build --locked --release -p audit -p capi &&
   ./target/release/audit capi` reports all violation counts 0.
10. **Commands:**
    - `cargo test --locked -p builtins --features test-support,lane/test-support`,
      `cargo test --locked -p builtins-compiler --features test-support`,
      `cargo test --locked -p host-core --features host-core/test-support --test live_delta`,
      `cargo test --locked -p capi`, `cargo test --locked -p host-web --features host-web/test-support`
    - the workspace debug leg (`test-debug-a`) and the DSP leg (`test-debug-b`) in
      `.github/workflows/qualification.yml`
    - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
    - `bash scripts/check-web-audioworklet.sh`, `bash scripts/check-builtins-policy.sh`,
      `bash scripts/check-realtime-policy.sh`, `bash scripts/check-unfused-seal.sh`,
      `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`,
      `bash scripts/run-aarch64-tests.sh debug`
    - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
      `cargo fmt --all -- --check`
11. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
    `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests; the
    G5 corpus (`cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`)
    passes unchanged.

## Test value

- Gate 1: red if preparation sets the static value instead of the curve, or an event fires on a flat
  curve.
- Gate 2: red if pan is interpolated in positions per sample, or a target is computed in `f32`, or
  the completion sample is off by one.
- Gate 3: red if the group's static cell is not seeded, so the other columns drift.
- Gate 4: red if a jump snaps to a grid sample or uses the fader or mute length.
- Gate 5: red if the split re-derives a step or restarts a lane's ramp.
- Gate 6: red if the fused path skips an event, or if the split path runs on silent, settled blocks.
- Gate 7: red if the pan rows stay masked, or a live record of the other cell overwrites the
  automated columns.
- Gate 8: red if the browser admits a command that the next curve event would overwrite, or refuses
  a strip with no automation.
- Gate 9: red if event handling allocates on render.

## Dependencies

Batch R2. Direct dependencies:

- Draft 10a *Classify fader automation edits as carried rebuilds*.
- Draft 10b *Refuse browser live commands on automated fader lanes* (the mask and the reason).
- Draft 12 *Hold the automation jump lengths in a plan cell* (the `pan` word).

Draft 07's events, draft 08's model (D1 follows it), the cells of *Hold live values in
latest-target cells on both hosts* (#1312) and *Session `controlSmoothing`: configurable ramp
lengths for live mute, fader and pan changes* (#1054) arrive through drafts 10a and 12. This draft
amends #1312 D3 (README amendment row). Draft 14b depends on this draft; this draft is complete
alone.
