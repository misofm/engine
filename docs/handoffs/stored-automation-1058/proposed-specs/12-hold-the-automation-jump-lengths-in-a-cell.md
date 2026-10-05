# Hold the automation jump lengths in a plan cell

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A3 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A host that changes a playing session's `control_smoothing` hears the new ramp lengths on the next
automation jump, with no plan rebuild. On a session with stored automation the edit's response
reports `live`; on a session without it, `model_only`, as today's rule for that edit says. A jump
already in flight keeps the length it started with.

## Context

- **Jump lengths** (README A3 and A1.4): a jump of a fader or trim cell ramps over the
  session's `fader_ms`, of a mute or polarity cell over `mute_ms`, of a pan or matrix cell over
  `pan_ms`, each converted to samples by `SessionModel::control_smoothing_samples()` (*Session
  `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*, #1054 D2-D3).
  HPF, LPF and EQ targets use their fixed 64-sample ramp; other effect parameters their descriptor's
  `smoothing_samples`. A session `explicit` 0 makes a jump a step.
- **No such field at HEAD.** `SessionModel` has no `control_smoothing` yet, and
  `LiveRamps::for_session` returns 0 for every row (`crates/host-core/src/live_delta.rs:33-58`).
  #1054 adds both. This slice is written against #1054.
- **The edit.** *Edit control_smoothing by a session transaction, model-only* (#1365) adds opcode
  `0x0008` and masks `control_smoothing` in the classifier (#1365 D4), so its delta is empty and its
  path `model_only` (#1313 D2). README amendment row #1365: with stored automation the path is
  `live`.
- **The cell primitive.** #1312 D1: a triple-buffered latest-target cell with one writer (control)
  and one reader (render); a write that completes before a block's drain is applied in that block,
  and render never skips the newest completed write.
- **The block entry.** The graph's render begins each block with the source set's `begin_block`
  (`crates/graph/src/lib.rs:3254-3256`).
- **Draft 07 D2** gives each cell program a jump-length key, never a copied `J`: a builtin row's key
  names a word of this slice's cell (`Fader`, `Mute`, `Pan`); filter and effect cells keep a fixed
  length. This slice lands before draft 09a, the first program that reads the cell.

## Decisions frozen for this slice

- **D1. One plan-level cell.** Every host-core plan owns one #1312 latest-target cell of three
  words: `mute`, `fader`, `pan`, in samples, the value of `control_smoothing_samples()`.
  Preparation writes it on both hosts. A builtin cell program reads the word its row names (fader
  and trim: `fader`; mute and polarity: `mute`; pan and matrix: `pan`; draft 07 D2's key). Filter,
  EQ and effect cells keep their fixed length. The cell's bytes are fixed and do not depend on the
  session.
- **D2. One reader.** The cell has one reader, so render reads it once per block: the graph reads it
  at block entry, beside the source set's `begin_block`, and holds the three values for that block.
  Draft 09b passes them, with the node-time mapping, in the block context it gives every strip
  stage. A jump reads its row's value when it starts. A ramp in flight keeps its length. No stage
  reads the cell itself.
- **D3. The edit path.** In `classify_live_delta`:
  - when `control_smoothing_samples()` differs between `current` and `next`, and `next` has at least
    one automation entry on a row whose jumps read a word of D1 (builtin rows 1, 2, 5, 6, 7-10 and
    12), the delta carries one `jump_lengths: Option<[u32; 3]>` value. `LiveDelta::is_empty` reads
    it, so the path is `live`. The test reads the session only, not which rows render yet: the cell
    is in every plan, so the write always lands, and a row whose rendering slice is still to come
    reads the new value from its first jump;
  - otherwise nothing changes: the delta stays empty and the path `model_only` (#1365 D4);
  - equal sample counts give no value, even when the milliseconds differ (never a redundant write).
  The commit writes the cell after every fallible check. A write to the newest candidate follows
  #1053 D7, as every live write does.
- **D4. Swaps.** A successor is prepared from the committed model, which holds the edit, so its cell
  starts with the new lengths. Nothing carries.
- **D5. The browser.** Preparation fills the cell. The browser has no `control_smoothing` edit until
  *Admit browser live edits in the Worker through the committed model* (#1382) (#1365 non-goal), so
  no browser path writes it.
- **D6. #1365 and D3.** D1, D2, D4 and D5 do not need #1365; draft 09a needs them. D3 is the
  `live` classification of #1365's `control_smoothing` edit, so D3 alone depends on #1365. This slice
  and #1365 may merge in either order, under one rule: whichever lands second wires the edit to the
  cell (D3 and gate 3). The README's #1365 amendment row records the same rule.
- **D7. The acked-batch question: can an ack ever precede a drop? No.** The cell write cannot fail,
  every check precedes it, and a replaced value is in the committed model.

## Deliverables

1. D1 and D2: the cell, preparation's seed and the graph's block-entry read. The strip stages' reads
   come with their rendering slices (draft 09b for the fader; drafts 13a, 14a and 15 for their
   words).
2. D3 in the classifier and the commit path, under D6's rule.

## Authorized paths

- `crates/host-core/src/prepare.rs` (the seed only), `crates/host-core/src/live_delta.rs`,
  `crates/host-core/tests/live_delta.rs`
- `crates/graph/src/lib.rs` (the block-entry read and its `test-support` reader only; stream A's
  file, sequenced by root)
- `crates/control-plane/src/` (the commit path), `crates/capi/src/runtime/live_tests.rs`
- Tests that pin a plan byte row the cell moves (re-pin each with the reason "the jump-length cell,
  draft 12 D1", never in bulk)

## Non-goals

- The `control_smoothing` grammar, defaults and rounding (#1054). The edit's opcode (#1365). A
  per-edit ramp length (#1394): stored automation has no per-edit length.
- Changing a ramp already in flight.

## Hazards

- **One reader only.** A second reader of a #1312 cell breaks its triple buffer. The reader half is
  one owned value, moved into the graph at preparation and not `Clone`, so a second reader cannot be
  written.
- **A transaction that changes `control_smoothing` and a rendered automated row together** is a
  rebuild once draft 10a classifies that row (its D1); the successor is prepared with the new
  lengths (D4), so D3's write is not needed there.

## Objective gates

1. **Block-entry read** (`crates/graph` test, new, through a `test-support` reader of the block's
   held values). A fresh plan holds the prepared lengths at its first block. A write of new lengths
   before a block entry is held from that block on, not before; two writes before one block give
   the last. The audible effect on a jump is draft 09b's gate 2.
2. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). With a fader entry, and with a mute
   entry: a `control_smoothing` change gives a live delta with `jump_lengths` and no other record.
   Without stored automation, or with only an effect entry: an empty delta. A change of `fader_ms`
   from 10.0 to 10.001 at 48 kHz (both 480 samples): an empty delta.
3. **C ABI** (`crates/capi/src/runtime/live_tests.rs`, new; landed by whichever of this slice and
   #1365 merges second, D6). A playing session with a fader entry takes a `SetControlSmoothing`
   edit: the response path is `live`, the provider epoch is unchanged, and the plan's cell holds the
   new lengths from the next block (gate 1's reader). The same edit on a session without automation
   reports `model_only`.
4. **Realtime.** The render block that reads a written cell allocates and frees nothing
   (`bench_support::alloc` thread counters, statics warmed), and
   `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` reports all
   violation counts 0.
5. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support --test live_delta`,
     `cargo test --locked -p builtins-compiler --features test-support`,
     `cargo test --locked -p graph --features test-support`, `cargo test --locked -p capi`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-graph-policy.sh`,
     `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
6. **No rendered bit moves** for a session with no stored automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence), and the browser legs of the `browser` job pass with
   unchanged digests.

## Test value

- Gate 1: red if the graph reads the cell only at preparation, a block late, or reads an older
  write.
- Gate 2: red if the edit is `live` with no entry that reads a word (a needless write), `model_only`
  with one (the new lengths never reach render), or if equal sample counts still write.
- Gate 3: red if the commit does not write the cell, or writes it to a plan that is not the newest.
- Gate 4: red if the block-entry read or the hand-off to the stages allocates on render.

## Dependencies

- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054).
- *Hold live values in latest-target cells on both hosts* (#1312).
- For D3, the edit path, only: *Edit control_smoothing by a session transaction, model-only*
  (#1365), in either merge order under D6's rule. #1365 brings the edit's `model_only` path from
  *Report each transaction's edit path in its response* (#1313).
- Batch: R1. Draft 09a builds on it; no dependency on draft 10a.
