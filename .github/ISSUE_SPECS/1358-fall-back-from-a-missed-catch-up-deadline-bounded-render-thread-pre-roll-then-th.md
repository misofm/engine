# Fall back from a missed catch-up deadline: bounded render-thread pre-roll, then the transition

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 fallbacks, D15-9, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287), W6 (the bound), W7 (the pre-roll), W8 and W9. Code anchors verified on `main`
at `6fb211594`.

## Product outcome

A latency-growing edit always completes, and the host always learns how.
- If the catch-up cannot reach its lead within the deadline (an overloaded device, a stalled
  producer, a divergence that keeps recurring), render finishes it with a bounded pre-roll in one
  callback: `PREROLL_FALLBACK`.
- Failing that, the strips whose arrival grows are duck-swapped: `TRANSITION_FALLBACK`, through
  *Duck-swap the strips a latency growth restarts, and fall back to the transition when no
  catch-up can finish* (#1397).
- A paused host never falls back. Its edit stays pending until render resumes.

## Context

- #1355 adds the catch-up, the live-lane drain deferral (D4), the per-epoch outcome word (D8) and
  the `p_max` configuration value (D7). *Give a plan a source-read clock that leads its render
  clock* (#1396) D4 adds `PlanPublisher::render_clock()`.
- `default_source_ring_frames` (`crates/host-core/src/prepare.rs:65-77`) is the stall tolerance
  (`SOURCE_STALL_TOLERANCE_MS = 100`, `:57`) plus two quanta. It has no room for a read-ahead, so
  #1320 D6's hold cap is 0 on a default ring.
- *Record the swap block's cost on the 64-track console* (#1286) D3 owns the single derivation of
  `k_max`, the deadline and the ring headroom behind `P_max`, from a timed successor rendering real
  material. *Prove two Wasm instances on one shared memory in three browser engines and on iOS*
  (#1331) supplies the browser rows.
- #1314 D5 has the `PREROLL_FALLBACK` and `TRANSITION_FALLBACK` bits and counters.
- D15-8 also listed "a host that renders nothing" as a fallback trigger. D15-17 supersedes that:
  the deadline is counted in render samples, so a paused host never falls back.
- #1320 D3: a peek may be used on the render thread, with the plan that holds it; its reads are
  loads and lent slices only.

## Decisions frozen for this slice

- **D1. Constants.** These go in `crates/host-core/src/catch_up.rs`, with the values #1286 D3
  derives; each comment names #1286 D3's item and record row. This spec restates no formula, and
  the values are not tuned afterwards.
  - `K_MAX_BLOCKS`: #1286 D3 item 1, the most successor blocks render may pre-roll in one callback.
  - `P_MAX_SAMPLES(fs)`: #1286 D3 item 2, the largest accumulated read-ahead `ΣP`.
  - `CATCH_UP_DEADLINE_SAMPLES(fs)`: #1286 D3 item 3, in render samples from B.
- **D2. Ring headroom.** `default_source_ring_frames` grows by #1286 D3 item 4's headroom,
  `CATCH_UP_DEADLINE_SAMPLES(fs) + P_MAX_SAMPLES(fs)`: a lagging peek holds frames until the
  deadline, so `P_MAX_SAMPLES` alone would not keep the producer's stall tolerance. #1320 D6's hold
  cap is that headroom in blocks. #1355's default `p_max` for a session is the smaller of
  `P_MAX_SAMPLES(fs)` and the smallest carried ring's headroom above the ungrown default minus
  `CATCH_UP_DEADLINE_SAMPLES(fs)`. The resource report and its exact assertions are updated.
- **D3. Deadline.** At each service call, if `render_clock() - B > CATCH_UP_DEADLINE_SAMPLES(fs)`, the
  catch-up stops and falls back (D4). A render clock that does not move never passes it.
- **D4. Pre-roll fallback.**
  1. Before publication, the control plane writes the #1277 D5 retarget records, then the held
     live edits (#1356 D3), then the epoch's outcome word `PREROLL_FALLBACK` (#1355 D8). Only then
     does it publish the successor with a new `PlanAdoption::PreRoll { max_blocks: K_MAX_BLOCKS }`.
     The hold is not dropped: it now lives in the successor.
  2. Render claims it at the next block, `h`. If the successor's clock is behind `h` by `d`, with
     `d / quantum <= max_blocks`, and every peek holds the frames for those blocks, render renders
     those blocks into the successor's preallocated discard planes, inside the render scope,
     reading the peeks on the render thread (#1320 D3). The live lanes stay closed (#1355 D4).
  3. It then adopts at `h` exactly as #1355 D6 does, which opens the live lanes before block `h`,
     so the held edits apply at `h`.
  4. Otherwise render marks the cell `Returned` with `ReturnReason::PreRollBound` (#1311
     D3 defines it), and the candidate stays there whole. The next service call takes it
     with `withdraw()` (`Withdrawal::Returned`, #1311 D6) and takes the transition (#1397 D2,
     `CatchUp::fall_back_to_transition`): only that path, which prepares the committed model
     again, drops the hold (#1356 D4).

  The catch-up's prime phase, if not yet done, counts toward `d`.
- **D5. Render-only catch-up.** `CatchUpMode::RenderOnly`, for a host with no off-thread executor
  (a browser that is not isolated, #1361), skips D3's wait. It publishes `PreRoll` under D4 right
  after the copy returns.
- **D6. Outcome.** A pre-roll adoption completes its revisions with `PREROLL_FALLBACK` through the
  epoch's word, and the counter rises by the revisions covered (#1314 D5).
- **D7. Acked-batch question.** The pre-roll candidate carries every held edit before it is
  published (D4.1), so its adoption drops none. A returned pre-roll candidate is kept whole until
  the transition is prepared from the committed model, which holds every edit. An ack can never
  precede a drop.

## Deliverables

1. D1-D6, and the qualification-doc paragraph on the three outcomes in
   `docs/C_ABI_V1_QUALIFICATION.md`.
2. Gates in `crates/host-core/tests/warm_successor.rs` and the engine tests.

## Authorized paths

- `crates/host-core/src/catch_up.rs`, `crates/host-core/src/prepare.rs`
- `crates/engine/src/realtime/plan_exchange.rs`, `crates/engine/src/realtime/plan.rs` (the
  pre-roll blocks), `crates/control-plane/src/`
- `crates/host-core/tests/warm_successor.rs`, the source-report assertions in host-core and capi
  tests
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Stop (#1359). C ABI and browser wiring (#1360, #1361). No tuning of D1's values.
- The transition fallback and the duck of restarted strips (#1397).

## Objective gates

1. **Pre-roll.** In #1355 gate 1's setup, the catch-up is never serviced and the render clock
   passes the deadline. The next service call publishes `PreRoll`. The output equals #1355 gate 1's
   reference, and the advance reports `PREROLL_FALLBACK`.
2. **Pre-roll bound.** With `d` one block beyond `K_MAX_BLOCKS`, render returns the candidate. The
   transition follows: the grown strips are ducked, the advance reports `TRANSITION_FALLBACK`, and
   the output at no sample steps by more than the duck ramp allows (#1324's gate).
3. **Pre-roll carries the hold.** In gate 1's setup, with a fader change and an EQ gain committed
   during the window, the pre-roll adoption applies both at `h`: from `h` the output equals a run in
   which the same values are written to the successor just before block `h`. In gate 2's setup
   the same edits reach the output through the transition's retargets; the model, revision and
   watermark show them applied.
4. **Paused host.** With no render for `10 * CATCH_UP_DEADLINE_SAMPLES(fs)` of wall time, service calls
   never fall back. After render resumes, the edit completes `EXACT`.
5. **Realtime.** A pre-roll of `K_MAX_BLOCKS` blocks makes zero allocations and frees on the render
   thread.
6. **Headroom.** With `ΣP = P_MAX_SAMPLES(fs)` and a peek that lags render until the deadline, a
   producer stall of the stall tolerance does not underrun.
7. Commands: those of #1356, plus `cargo build --locked --release -p audit -p capi &&
   ./target/release/audit capi`.

## Test value

- Gate 1: a deadline counted in wall time, or a fallback that skips the pre-roll, misses the exact
  output or the flag. Red.
- Gate 2: a pre-roll that ignores its bound renders `k_max + 1` blocks in one callback. Red.
- Gate 3: a pre-roll published without the retargets and the hold, with the hold then dropped as
  for a fallback, loses an acknowledged edit; one that opens the live lanes before its pre-rolled
  blocks applies them early. Red.
- Gate 4: a wall-clock deadline falls back on a paused host. Red.
- Gate 6: rings grown by `P_MAX_SAMPLES` alone, without the deadline, underrun inside the stall
  tolerance while the peek lags. Red.

## Dependencies

- *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355).
- *Hold live edits during a catch-up and apply them at the adoption sample* (#1356).
- *Duck-swap the strips a latency growth restarts, and fall back to the transition when no
  catch-up can finish* (#1397, split from this issue): the transition a returned pre-roll takes.
- *Give a plan a source-read clock that leads its render clock* (#1396): `render_clock()`.
- *Record the swap block's cost on the 64-track console* (#1286).
- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331).
- *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
