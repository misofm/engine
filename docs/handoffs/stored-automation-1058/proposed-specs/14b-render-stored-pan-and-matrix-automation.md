# Render stored pan and matrix automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.4, A1.5, A2 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2.

## Product outcome

A producer's stored pan moves (row 12) and matrix moves (rows 7-10) play on every host. Pan stays
equal-power: at each ramp's completion sample the strip's 2x2 equals `pan_matrix` of the curve
values exactly, and it is linear between. A jump lands on its exact sample. A pan or matrix
automation edit is a carried rebuild, and a static edit of an automated pan position or coefficient
is `model_only`. On a strip whose pan or matrix is partly automated, a live edit of a cell the
session does not automate (the other pan position, or a matrix coefficient with no curve) is a live
group-cell write on both hosts: it moves that cell and never overwrites the automated one.

## Context

- **Pan.** `pan_matrix(left, right)` designs the equal-power 2x2 in `f64` through `math::cos` and
  `math::sin`, one cast per word (`crates/builtins/src/lib.rs:4363-4378`); `left` pans the left
  input (`ll`, `rl`), `right` the right input (`lr`, `rr`). `lower_matrix_or_pan` is the one
  lowering (`crates/builtins-compiler/src/lib.rs:4925-4940`). The model is `MatrixOrPan`
  (`crates/session/src/model.rs:655-680`).
- **The in-block operations** on the matrix stage and the fused path's rule are draft 14a's.
- **The seam.** The matrix is seam-side: "the 2x2 matrix **is** the seam"
  (`crates/builtins-compiler/src/lib.rs:817-829`; `SEAM_SIDE_WITNESS`, `:391-398`). A collapsed track
  duplicates its plane into the matrix stage, whose two channels' words are free to differ.
- **The classifier** emits one matrix record when a lowered coefficient's bits change
  (`crates/host-core/src/live_delta.rs:308-313`), with the post-commit model's `smoothing_samples`
  through `lower_matrix_or_pan`. *Session `controlSmoothing`: configurable ramp lengths for live
  mute, fader and pan changes* (#1054) D5 makes the record's ramp the model's window, else the
  session pan length.
- **The cell.** *Hold live values in latest-target cells on both hosts* (#1312) D3: one matrix cell
  of five words (four coefficients, ramp), written by `TrackControlProducer::producer`
  (`crates/builtins-compiler/src/lib.rs:254-270`).
- **Browser edits.** Today `COMMAND_PAN = 1` sets both pan positions and `COMMAND_MATRIX = 2` all
  four coefficients (`hosts/host-web/src/lib.rs:827`, `:829`). After *Admit browser live edits in
  the Worker through the committed model* (#1382) every browser live edit reaches the shared
  commit through the Worker's apply and meets the shared classifier, as a C ABI transaction does,
  whatever form the browser uses to send it.

## Decisions frozen for this slice

- **D1. The matrix stage is a target group** (A1.5).
  - **Pan** (row 12, per lane). At each event the stage's target is `pan_matrix(left, right)`, each
    position from its curve when automated, else from the group cell.
  - **Matrix** (rows 7-10, `both`). At each event the target takes each automated coefficient from
    its curve and every other coefficient from the group cell, then `Matrix2x2::checked`.
  - **The group cell.** For a strip whose matrix group has an automated cell, this slice gives
    #1312's matrix cell the meaning "group cell": the two pan positions, or the four coefficients
    (the automated ones carry no meaning there), and the ramp. Every other strip keeps #1312's
    meaning; #1312 is not amended. Preparation seeds it with the session's static values.
  - Grid period and ramp 64, completion `End` (`τ + 63`): the kernel assigns the target exactly on
    the ramp's last sample. Jumps ramp over the session pan length (draft 12's `pan` word). The
    strip's `smoothing_samples` is a live-record rule (#1054 D5) and is not read here.
  - A seek where it reaches the matrix node sets the target exactly (draft 14a's set operation).
  - Events of the cells of one group at one offset make one target, so the group retargets at most
    once per offset. Only a target whose bits change retargets.
  - At the next block entry after the group cell changes, render builds the group's target from the
    curves and the new cell values and retargets over the cell's ramp word, at the block's first
    sample, only if the target's bits change. Grid events inside that ramp are held (README "Held
    events").
- **D2. Edits, one rule on both hosts.**
  - The classifier's row list (draft 10 D1) gains rows 7-10 and 12: an edit of their automation is
    a carried rebuild; cells carry by draft 10's address.
  - A static edit of an automated pan position or coefficient gives no record (draft 10 D3).
  - A static edit of a cell the session does not automate, in a group that has an automated cell,
    writes the group cell (the two pan positions or the four coefficients, with the ramp word
    `LiveRamps::resolve(Matrix, the edit's own ramp)`, *Carry an optional per-edit ramp length on
    live session edits*, #1394 D5, which keeps #1054 D5's matrix precedence when the edit carries
    none) instead of a matrix record. The group cell is the live slot that decision 14
    rule 2 asks for, so the path is `live`. An edit that leaves the cell's bits unchanged gives
    nothing.
  - A browser live edit that sets every cell of the group (both pan positions, or all four
    coefficients) reaches the shared commit through the Worker's apply and meets the two rules
    above cell by cell: the automated cells' values commit
    as fallback values, and the other cells go to the group cell.
- **D3. Mono collapse.** The matrix stage is seam-side (Context), so pan and matrix automation,
  symmetric or not, never declines a track's collapse. (README A1.5's collapse rule applies to the
  input rows only; drafts 15 and 16b apply it.)
- **D4. The acked-batch question: can an ack ever precede a drop? No.** Curve events come from the
  plan; an edit of an automated group gives no record, a group-cell write after every check, or a
  rebuild; a cell write cannot fail.

## Deliverables

1. D1 in the matrix and fused processors, and preparation's group-cell seed.
2. D2 in the classifier and the commit path.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (the matrix and fused processors' events and group-cell
  read)
- `crates/host-core/src/prepare.rs` (group cell seed), `crates/host-core/src/live_delta.rs`,
  `crates/host-core/tests/live_delta.rs`
- `crates/control-plane/src/` (the group-cell write)
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/pan_automation_realtime.rs` (new)

## Non-goals

- The in-block operations (draft 14a).
- Send matrix automation (OQ2). Options B, C and D of OQ1. A pan law other than `pan_matrix`.

## Hazards

- **A redundant retarget moves bits** (README "Change only"). D1 compares the whole target, and D2
  writes only changed bits.

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
5. **Group cell, both hosts** (`crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`,
   new). Left position automated; a transaction (C ABI) and a browser live pan edit through the Worker's apply move the
   static `right` position: path `live`, the same provider epoch; the left columns (`ll`, `rl`) keep
   their curve values at every completion sample; after the ramp, the output equals a plan
   prepared with the new `right` and the same automation. The matrix form: `ll` automated, a
   transaction moves the static `rr`, `ll` stays on its curve.
6. **Edits** (`crates/host-core/tests/live_delta.rs`, new). A pan entry change gives
   `Err(LiveRebuild::Automation)`; a static change of the automated position gives no record; a
   static change of the other position gives one group-cell write and no matrix record; an edit
   that leaves the cell's bits unchanged gives nothing.
7. **Realtime.** `hosts/host-web/tests/pan_automation_realtime.rs` (new integration binary; links
   `bench_support::alloc`, calls `assert_installed()` first): `allocations == 0 && frees == 0`
   around every render call after warm-up; `cargo build --locked --release -p audit -p capi &&
   ./target/release/audit capi` reports all violation counts 0.
8. **Commands:**
   - `cargo test --locked -p builtins-compiler --features test-support`,
     `cargo test --locked -p host-core --features host-core/test-support --test live_delta`,
     `cargo test --locked -p control-plane --features test-support`,
     `cargo test --locked -p capi`, `cargo test --locked -p host-web --features host-web/test-support`
   - the workspace debug leg (`test-debug-a`) and the DSP leg (`test-debug-b`) in
     `.github/workflows/qualification.yml`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/check-builtins-policy.sh`,
     `bash scripts/check-realtime-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-workspace-policy.sh`, `bash scripts/check-cross-targets.sh`,
     `bash scripts/run-aarch64-tests.sh debug`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
9. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if preparation sets the static value instead of the curve, or an event fires on a flat
  curve.
- Gate 2: red if pan is interpolated in positions per sample, or a target is computed in `f32`, or
  the completion sample is off by one.
- Gate 3: red if the group's static cell is not seeded, so the other columns drift.
- Gate 4: red if a jump snaps to a grid sample or uses the fader or mute length.
- Gate 5: red if a live edit overwrites the automated columns, is lost, rebuilds, or differs
  between the hosts.
- Gate 6: red if the pan rows stay masked, a live record of the other cell overwrites the automated
  columns, or an unchanged cell is written.
- Gate 7: red if event handling allocates on render.

## Dependencies

Batch R2. Direct dependencies:

- Draft 14a *Retarget the matrix stage inside a block*.
- Draft 10 *Classify fader automation edits as carried rebuilds* (the row list, the predicate and
  the carry; it brings #1382 through draft 09a).
- Draft 12 *Hold the automation jump lengths in a plan cell* (the `pan` word).

Draft 07's events, the cells of *Hold live values in latest-target cells on both hosts* (#1312) and
*Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes* (#1054)
arrive through drafts 10 and 12.
