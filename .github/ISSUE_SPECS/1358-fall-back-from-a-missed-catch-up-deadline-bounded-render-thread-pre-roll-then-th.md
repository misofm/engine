# Fall back from a missed catch-up deadline: bounded render-thread pre-roll, then the transition

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 fallbacks, D15-9, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287), W6-W9. Code anchors verified on `main`
at `6fb211594`.

## Product outcome

A latency-growing edit always completes, and the host always learns how.
- If the catch-up cannot reach its lead within the deadline (an overloaded device, a stalled
  producer), render finishes it with a bounded pre-roll in one callback: `PREROLL_FALLBACK`.
- Failing that, the strips whose arrival grows are duck-swapped: `TRANSITION_FALLBACK`.
- A paused host never falls back. Its edit stays pending until render resumes.

## Context

- #1355 adds the catch-up, the render-clock reader (D4), and the `p_max` configuration value (D7).
- `default_source_ring_frames` (`crates/host-core/src/prepare.rs:65-77`) is the stall tolerance
  (`SOURCE_STALL_TOLERANCE_MS = 100`, `:57`) plus two quanta. It has no room for a read-ahead.
- *Record the swap block's cost on the 64-track console* (#1286) measures one ordinary block and
  the swap block at AVX2. *Prove two Wasm instances on one shared memory in three browser engines
  and on iOS* (#1331) measures the browser side.
- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324) ducks the strips of
  `restarted_strips()` (D1), schedules `S` (D4), and reports `TRANSITION_FALLBACK` (D6).
- #1314 D5 has the `PREROLL_FALLBACK` and `TRANSITION_FALLBACK` bits and counters.
- D15-8 also lists "a host that renders nothing" as a fallback trigger. D15-17 supersedes that:
  the deadline is counted in render samples, so a paused host never falls back.

## Decisions frozen for this slice

- **D1. Constants.** These go in `crates/host-core/src/catch_up.rs`, each with a comment naming
  the record row it comes from. They are not tuned afterwards.
  - `K_MAX_BLOCKS`: the most successor blocks render may pre-roll in one callback. It is the
    largest `k` whose `k + 1` block cost stays inside one quantum's real-time budget on #1286's
    AVX2 row and on #1331's slowest browser row.
  - `CATCH_UP_DEADLINE_SAMPLES`: render samples from B after which the next service call falls
    back.
  - `P_MAX_SAMPLES`: the largest accumulated read-ahead `ΣP`.
- **D2. Ring headroom.** `default_source_ring_frames` adds `P_MAX_SAMPLES`, rounded up to a
  quantum. That keeps the stall tolerance after `ΣP` reaches `P_MAX_SAMPLES`. #1355's `p_max` for
  a session is the smaller of `P_MAX_SAMPLES` and the smallest carried ring's frames minus the
  stall tolerance minus two quanta. The resource report and its exact assertions are updated.
- **D3. Deadline.** At each service call, if `render_clock() - B > CATCH_UP_DEADLINE_SAMPLES`, the
  catch-up stops and falls back. A render clock that does not move never passes it.
- **D4. Pre-roll fallback.** The control plane publishes the successor with a new
  `PlanAdoption::PreRoll { max_blocks: K_MAX_BLOCKS }`. Render claims it at the next block, `h`.
  1. If the successor's clock is behind `h` by `d`, with `d / quantum <= max_blocks`, and every peek
     holds the frames for those blocks, render renders those blocks into the successor's
     preallocated discard planes, inside the render scope.
  2. It then adopts at `h` exactly as #1355 D6 does.
  3. Otherwise it returns the candidate (#1311 D4), and the control plane takes D5.

  The catch-up's prime phase, if not yet done, counts toward `d`.
- **D5. Transition fallback.**
  1. The catch-up is abandoned: peeks abandoned, hold dropped (#1356 D4).
  2. The committed model is prepared again as an ordinary successor: #1285 floors, no lead.
  3. The strips whose arrival at any node grows over the predecessor's are added to #1324's duck
     set.
  4. It is published as #1324 publishes, with one `S`.
- **D6. Render-only catch-up.** `CatchUpMode::RenderOnly`, for a host with no off-thread executor
  (a browser that is not isolated, #1361), skips D3's wait. It publishes `PreRoll` right after the
  copy returns.
- **D7. Outcome.** D4 completes the revision with `PREROLL_FALLBACK`, and D5 with
  `TRANSITION_FALLBACK`. Each counter rises by the revisions covered (#1314 D5).
- **D8. Acked-batch question.** Both fallbacks build from the committed model, and a returned
  pre-roll candidate is kept whole. An ack can never precede a drop.

## Deliverables

1. D1-D7, and the qualification-doc paragraph on the three outcomes in
   `docs/C_ABI_V1_QUALIFICATION.md`.
2. Gates in `crates/host-core/tests/warm_successor.rs` and the engine tests.

## Authorized paths

- `crates/host-core/src/catch_up.rs`, `crates/host-core/src/prepare.rs`
- `crates/engine/src/realtime/plan_exchange.rs`, `crates/control-plane/src/`
- `crates/host-core/tests/warm_successor.rs`, the source-report assertions in host-core and capi
  tests
- `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Stop (#1359). C ABI and browser wiring (#1360, #1361). No tuning of D1's values.

## Objective gates

1. **Pre-roll.** In #1355 gate 1's setup, the catch-up is never serviced and the render clock
   passes the deadline. The next service call publishes `PreRoll`. The output equals #1355 gate 1's
   reference, and the advance reports `PREROLL_FALLBACK`.
2. **Pre-roll bound.** With `d` one block beyond `K_MAX_BLOCKS`, render returns the candidate. The
   transition follows: the grown strips are ducked, the advance reports `TRANSITION_FALLBACK`, and
   the output at no sample steps by more than the duck ramp allows (#1324's gate).
3. **Paused host.** With no render for `10 * CATCH_UP_DEADLINE_SAMPLES` of wall time, service calls
   never fall back. After render resumes, the edit completes `EXACT`.
4. **Realtime.** A pre-roll of `K_MAX_BLOCKS` blocks makes zero allocations and frees on the render
   thread.
5. **Headroom.** With `ΣP = P_MAX_SAMPLES`, a producer stall of the stall tolerance does not
   underrun.
6. Commands: those of #1356, plus `cargo build --locked --release -p audit -p capi &&
   ./target/release/audit capi`.

## Test value

- Gate 1: a deadline counted in wall time, or a fallback that skips the pre-roll, misses the exact
  output or the flag. Red.
- Gate 2: a pre-roll that ignores its bound renders `k_max + 1` blocks in one callback. Red.
- Gate 3: a wall-clock deadline falls back on a paused host. Red.
- Gate 5: rings not grown by `P_MAX_SAMPLES` underrun inside the stall tolerance. Red.

## Dependencies

- *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355).
- *Hold live edits during a catch-up and apply them at the adoption sample* (#1356).
- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324).
- *Record the swap block's cost on the 64-track console* (#1286).
- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331).
- *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
