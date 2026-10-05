# Catch up a returned successor and adopt it exactly at a scheduled sample

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 steps 3-4, D15-12, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287), W3, W4 and W6. Code anchors verified
on `main` at `6fb211594`.

## Product outcome

In host-core, a structural edit that grows a node's latency by up to `P` swaps in with no gap and
no jump.
- A warm successor returned at B (#1354) is rendered forward off the render thread through the
  ring peeks, then adopted exactly at a sample S.
- Every unchanged path continues bit for bit.
- The plan's source-read clock leads its render clock by the accumulated `ΣP`.

## Context

- The graph driver reads every source at the block's sample
  (`consumer.begin_block_at(first_sample)`, `crates/source/src/lib.rs:1721`). *Anchor every seek on
  the plan's source-read clock* (#1316) D1 renames that parameter `source_read_sample` and leaves
  this issue to make it differ from the render clock.
- `PreparedRenderPlan` has one clock, `next_absolute_sample` (`crates/engine/src/realtime/plan.rs:609`),
  passed to the executor as `RenderTime` (`:943`).
- Inputs from earlier slices:
  - #1321's `OffThreadPlanRenderer::render_slice`;
  - #1320's `PcmSourcePeek` reads (D5) and `advance_past_peek` (D8);
  - #1311's `ExactlyAt(S)`, else return (D3);
  - #1322 D3: after a copy, move-mode adoption runs only the source section;
  - #1327 D4: observers publish only windows the predecessor did not publish;
  - #1277 D5: in copy mode the retarget records stay on the prepared successor and are written at
    publication.
- *Reset latency floors at a host-declared discontinuity* (#1323) D2-D3 publishes a successor with
  no floors that carries only the sources.

## Decisions frozen for this slice

- **D1. Two clocks.**
  - `PreparedRenderPlan` gains `source_read_offset: u64`, 0 by default. `RenderTime` gains
    `source_read_sample = absolute_sample + source_read_offset`, and the graph driver passes it to
    `begin_block_at`.
  - A warm successor's offset is its predecessor's plus `lead_samples` (#1354 D3).
  - A #1323 discontinuity successor has offset 0. This resets `ΣP` at the declaration, and the
    floors reset with it.
- **D2. Peek-backed entries.**
  - At warm preparation, each carried source's successor entry takes its ring's peek
    (`take_peek`, #1320 D3) instead of staying vacant. During the catch-up the driver reads it as
    it reads a consumer: `played_planes` lends the peek's planes.
  - Before each block the catch-up checks that every peek holds the block's frames; `NotYet` ends
    the slice without rendering.
  - `Diverged` abandons the catch-up. The peeks are abandoned (#1320) and the successor is dropped
    on the control thread. The edit is then republished as a new warm successor from a new B, until
    #1358's deadline.
- **D3. Alignment.** The catch-up starts and fills exactly as #1287's recorded proof states. Per
  that proof:
  1. A prime phase reads source frames `[B, B + P)` through the peeks into each source-claim line,
     without advancing any node downstream of the lines.
  2. The successor then renders render samples `B ...` with sources read at
     `render + source_read_offset`.
  3. Its render clock equals the predecessor's throughout.
- **D4. Render-clock reader.** Render stores the active plan's next sample in an `AtomicU64` once
  per block (`Release`). `PlanPublisher::render_clock()` reads it. #1358's deadline uses the same
  word.
- **D5. Publication.** The catch-up type is `host_core::CatchUp`, which owns the
  `OffThreadPlanRenderer`.
  - `CatchUp::service(&mut self, max_blocks) -> CatchUpState` renders while the successor's clock
    is behind `render_clock() + 2 * quantum`.
  - Then it writes the held #1277 D5 retarget records into the successor's queues and cells, and
    publishes `ExactlyAt(S)` with `S` the successor's next sample.
  - A returned candidate goes back into the loop and is published again later.
- **D6. Adoption at S.** Render adopts by pointer swap. The move-mode source section moves each
  consumer into the successor, calls `advance_past_peek(lead_samples)`, and rebinds the entry from
  the peek to the consumer. Observers take the predecessor's producers under #1327 D4. A ring whose
  advance refuses fails the whole section and moves nothing; the candidate is returned, never
  adopted.
- **D7. Bound.** Preparation refuses a warm successor with `ΣP + P > p_max` (a typed error). Here
  `p_max` is a `CatchUp` configuration value; *Fall back from a missed catch-up deadline: bounded
  render-thread pre-roll, then the transition* (#1358) fixes it.
- **D8. Outcome.** The structural revision completes at S with `EXACT` (#1314 D5).
- **D9. Acked-batch question.** A returned or abandoned successor drops no committed content:
  the edit stays committed and is republished. `advance_past_peek` releases only frames the
  successor consumed. An ack can never precede a drop.

## Deliverables

1. D1 in `crates/engine/src/realtime/plan.rs`, and the driver change in `crates/source`.
2. D2 in `crates/source/src/lib.rs`.
3. D3, D5 and D7 in a new `crates/host-core/src/catch_up.rs` section, beside #1321's renderer.
4. D4 and D6 in `crates/engine/src/realtime/plan_exchange.rs` and the graph's source section.
5. The proof's alignment, with any correction, back in #1287's decision record.
6. Gates in `crates/host-core/tests/warm_successor.rs`.

## Authorized paths

- `crates/engine/src/realtime/plan.rs`, `plan_exchange.rs`, `mod.rs`
- `crates/source/src/lib.rs`, `crates/graph/src/lib.rs` (source section, claim-line prime)
- `crates/host-core/src/catch_up.rs`, `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs`
- `crates/host-core/tests/warm_successor.rs`

## Non-goals

- Held live edits (#1356), supersession (#1357), the deadline and fallbacks (#1358), and stop
  (#1359).
- Control-plane and C ABI wiring (#1360), and the browser (#1361).
- A successor that adds a source: its new ring has no peek and is read directly; gates use
  existing sources.

## Hazards

- **Size.** This is the largest stream C slice. If the prime phase (D3) needs a partial-render
  executor call, move it into its own slice before this one and report that.

## Objective gates

1. **Output growth, gap-free (the old gate 1).**
   - Predecessor A is the two-track fixture of `crates/host-core/tests/successor_swap.rs`.
     Successor B adds a muted track with a true-peak limiter insert and silent input.
   - With the copy after block 6 and the catch-up serviced from a second thread, block `j` of the
     swapped run equals block `j + k` of a fresh B. That fresh B is compiled with the same floors
     and offset 0 and fed the same PCM from frame 0. This holds for every `j`.
   - Run at all four launch rates and both bank widths.
2. **Submix growth (the old gate 2).** The same, with the limiter inside a submix whose arrival
   grows while the output's does not.
3. **Anchor (the old gate 4, per D15-12).** After adoption, a `seek_at` anchored at source-read
   sample `A` applies in the block whose `source_read_sample` is `A`. Its first frame reaches the
   output where the fresh reference of gate 1, given the same seek, puts it.
4. **Discontinuity resets ΣP.** After gate 1's adoption, a #1323 declaration yields a plan with
   offset 0 and no floors.
5. **Realtime.** The adoption block makes zero allocations and frees on the render thread
   (`bench_support::alloc`).
6. **Abandon.** A seek during the catch-up yields `Diverged`. The producer's admission depth returns
   to the configured value, and a republished successor reaches gate 1's equality.
7. Commands: those of #1354, plus `cargo test --locked -p source` and `cargo test --locked -p capi`.

## Test value

- Gate 1: an adoption that does not advance the consumers by `P`, or a prime that leaves `+0.0` in
  the lines, repeats or gaps `P` frames. Red.
- Gate 2: a lead sized from the output alone leaves the submix's paths skipping. Red.
- Gate 3: a seek compared on the render clock starts the stem `k` blocks early. Red.
- Gate 4: an offset that survives the declaration keeps reading ahead after a stop. Red.
- Gate 6: a divergence that keeps the gate armed starves the producer. Red.

## Dependencies

- *Snapshot a running plan into a returned successor at a block* (#1354).
- *Give the source ring a read-only peek cursor that gates release* (#1320).
- *Render a successor plan off the render thread with a pinned floating-point environment* (#1321).
- *Anchor every seek on the plan's source-read clock* (#1316).
- *Reset latency floors at a host-declared discontinuity* (#1323).
- *Carry fader, mute and pan ramps across a plan swap* (#1277), D5.
- *Carry meter, observation and spectrum state across a plan swap* (#1327), D4.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
