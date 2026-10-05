# Edit a pan or matrix group cell under stored automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1.5, A2 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2, after draft 14a *Render stored pan and
matrix automation*.

## Product outcome

On a strip whose pan or matrix is partly automated, a live edit of a cell the session does not
automate (the other pan position, or a matrix coefficient with no curve) is a live update on the C
ABI: it moves that cell, never overwrites the automated one, and needs no rebuild. The browser
refusal of pan and matrix commands on an automated group is draft 14a's and stays until #1382.

## Context

- **The group** is draft 14a D2: at each event render builds the stage's target from the curves of
  the automated cells and the group cell for the others. Draft 14a D3 makes a static edit of a
  non-automated cell of an automated group a carried rebuild, because the plan has no live slot for
  it yet (decision 14 rule 2), and refuses the browser's pan and matrix commands on such a group.
- **The cell.** #1312 D3: one matrix cell of five words (four coefficients, ramp), written by
  `TrackControlProducer::producer` (`crates/builtins-compiler/src/lib.rs:254-270`). README amendment
  row #1312 D3: for an automated group the matrix cell holds the pan positions or the coefficients
  the session does not automate.
- **The classifier** emits one matrix record when a lowered coefficient's bits change
  (`crates/host-core/src/live_delta.rs:308-313`), with the post-commit model's `smoothing_samples`
  through `lower_matrix_or_pan` (`crates/builtins-compiler/src/lib.rs:4925-4940`). *Session
  `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes* (#1054) D5
  makes the record's ramp the model's window, else the session pan length.

## Decisions frozen for this slice

- **D1. The group-cell write.** For a strip whose matrix group has an automated cell, a static edit
  of a cell the session does not automate writes the group cell: the two pan positions, or the four
  coefficients (the automated ones carry no meaning there), with the ramp word from #1054 D5's rule.
  The group cell is the live slot that decision 14 rule 2 asks for. The classifier emits it instead
  of a matrix record, and the path is `live`. It replaces draft 14a D3's rebuild for this case. An edit that leaves the group cell's bits unchanged gives nothing.
- **D2. Render.** At the next block entry after the cell changes, render builds the group's target
  from the curves and the new cell values (draft 14a D2) and retargets over the cell's ramp word, at
  the block's first sample, only if the target's bits change. Grid events inside that ramp are held
  (README "Held events").
- **D3. The acked-batch question: can an ack ever precede a drop? No.** The cell write follows
  every check and cannot fail.

## Deliverables

1. D1 in the classifier and the commit path.
2. D2 in the matrix and fused processors.

## Authorized paths

- `crates/host-core/src/live_delta.rs`, `crates/host-core/tests/live_delta.rs`
- `crates/builtins-compiler/src/lib.rs` (the group-cell read)
- `crates/control-plane/src/` (the group-cell write), `crates/capi/src/runtime/live_tests.rs`

## Non-goals

- Rendering pan and matrix automation and the browser refusal (draft 14a). Options B and C of OQ1.
  A browser transaction path (#1382).

## Hazards

- **A redundant retarget moves bits** (README "Change only"). D1 writes only changed bits; D2
  compares the whole target.

## Objective gates

1. **Group cell** (C ABI, `crates/capi/src/runtime/live_tests.rs`, new). Left position automated; a
   transaction moves the static `right` position: path `live`, the same provider epoch; the left
   columns (`ll`, `rl`) keep their curve values at every completion sample; after the ramp, the
   output equals a plan prepared with the new `right` and the same automation.
2. **Matrix** (same file, new). `ll` automated; a transaction moves the static `rr`: path `live`;
   `ll` stays on its curve; after the ramp, the output equals a plan prepared with the new `rr`.
3. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). The edit of gate 1 gives one
   group-cell write and no matrix record; an edit that leaves the cell's bits unchanged gives
   nothing.
4. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support --test live_delta`,
     `cargo test --locked -p builtins-compiler --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-web --features host-web/test-support`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` (all
     violation counts 0)
   - `bash scripts/check-web-audioworklet.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
5. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if a live edit overwrites the automated columns, is lost, or still rebuilds.
- Gate 2: red if the matrix form reads the wrong coefficient from the cell.
- Gate 3: red if the classifier still emits a matrix record for an automated group, or writes an
  unchanged cell.

## Dependencies

Batch R2. Direct dependencies:

- Draft 14a *Render stored pan and matrix automation*.

Drafts 10a and 10b, *Hold live values in latest-target cells on both hosts* (#1312, amended at D3)
and *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
(#1054) arrive through draft 14a.
