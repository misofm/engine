# Fall back to the transition when a warm successor is not ready by its deadline

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-9, D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287): the deadline
and the ring headroom. Code anchors verified on `main` at `6fb211594`.

## Product outcome

A latency-growing edit always completes, and the host always learns how.
- A warm successor that render has not adopted within `PRIME_DEADLINE_SAMPLES` of render is
  withdrawn by the next control call and replaced by the transition: `TRANSITION_FALLBACK`. That
  happens when a source is never fed far enough ahead, a producer stalls past its tolerance, or
  seeks keep landing in the prime window.
- If render adopts it first, the edit completes `EXACT`, even after the deadline.
- A paused host never falls back. Its edit stays pending until render resumes.
- Default source rings have room for the prime, so a host that keeps its rings full is always
  ready.

## Context

- `default_source_ring_frames` (`crates/host-core/src/prepare.rs:65-77`) is the stall tolerance
  (`SOURCE_STALL_TOLERANCE_MS = 100`, `:57`) rounded up to quanta, plus two quanta. It has no room
  for `P + q` frames queued past the consumer beyond that tolerance.
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355): render adopts
  a `Primed` candidate at the first block at or after `not_before` where every carried ring passes
  *Let a source consumer check and replay its next blocks for a prime* (#1320) D2. Its
  `publish_primed` returns a `PrimedCandidate { epoch, not_before, lead_blocks }` record (D3).
- *Give a plan a source-read clock that leads its render clock* (#1396) D4 adds
  `PlanPublisher::render_clock()`, which does not move while render is paused.
- *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
  successor cannot adopt* (#1397) D2: `fall_back_to_transition`, which takes the withdrawn
  candidate as its donor.
- `P_MAX` is the bound on `ΣP + P` that warm preparation checks
  (`WarmUnavailable::LeadBound`, *Prepare a warm successor whose carried nodes lead the
  predecessor by P*, #1354), with the value of *Record the swap block's cost on the 64-track
  console* (#1286) D3's `P_MAX_SAMPLES(fs)`.
- D15-8 once listed "a host that renders nothing" as a fallback trigger. D15-17 supersedes that:
  the deadline is counted in render samples.

## Decisions frozen for this slice

- **D1. Deadline.** `PRIME_DEADLINE_SAMPLES(fs, q)`, a `const fn` in
  `crates/host-core/src/warm.rs`:
  `T(fs, q) + P_MAX_SAMPLES(fs) + q`, where `T(fs, q) = ceil_q(fs * SOURCE_STALL_TOLERANCE_MS /
  1000)` is one stall tolerance in whole quanta. It is counted in render samples from
  `not_before`, which is never earlier than the render clock read at publication.
  - Why this value. With D2's headroom, a host that keeps its rings full holds at least
    `P_MAX + q` frames past each consumer through any producer stall up to the tolerance. So
    queued frames alone never delay readiness for such a host. The other thing that delays
    readiness is a held seek or command inside the prime window. A held seek is anchored below
    `S + O + P + q`, so it applies within `P + q <= P_MAX + q` render samples. After it applies,
    the host gets one stall tolerance to queue the new generation `P + q` frames ahead.
  - A candidate not adopted by then belongs to a host that does not keep `P + q` frames queued, or
    that keeps seeking inside the window. No exact mechanism can serve that host, because it needs
    those future frames. It gets the transition.
  - No other spec restates this formula.
- **D2. Ring headroom.** `default_source_ring_frames` grows by `P_MAX_SAMPLES(fs) + q`. That is
  the most frames readiness asks to be queued (`(k + 1) * q` with `k * q <= P_MAX`), on top of the
  stall tolerance the ring already gives the producer. Nothing is held for a deadline, so the
  deadline adds no term. The resource report and its exact assertions are updated.
- **D3. The deadline step.** A new host-core function
  `check_prime_deadline(publisher, pending: &PrimedCandidate) -> DeadlineStep`. The control plane
  runs it once per control call while a `Primed` candidate is pending (#1360 D2 for the C ABI,
  #1361 for the browser).
  - If `render_clock() < not_before + PRIME_DEADLINE_SAMPLES`, it returns `Pending` and does
    nothing. A render clock that does not move never passes it.
  - Otherwise it calls `withdraw()` (#1343 D5).
    - `Taken` means render adopted the candidate exactly. The step returns `Adopted`, and the
      revision completes `EXACT` through the watermark.
    - `Withdrawn` hands the candidate to `fall_back_to_transition` (#1397 D2) as its donor. That
      re-prepares the committed model, publishes the transition, and counts `TRANSITION_FALLBACK`.
  - If #1397 D2's re-preparation was refused, the donor is kept, and the next call's step retries
    `fall_back_to_transition` with it.
  - One step does at most one withdrawal and one re-preparation, and never waits on render.
- **D4. Acked-batch question.** The deadline step drops nothing. A `Taken` candidate is adopted
  with every edit in its cells. A withdrawn one is kept as the donor until its replacement is
  prepared from the committed model, which holds every acked edit (#1397 D4). While render waits
  for readiness, the predecessor keeps playing every queued frame. An ack can never precede a
  drop.

## Deliverables

1. D1 and D3 in `crates/host-core/src/warm.rs`.
2. D2 in `crates/host-core/src/prepare.rs`, with the source-report assertions updated.
3. One control-plane method in `crates/control-plane/src/` that runs D3 for the pending
   `PrimedCandidate`. The hosts' service steps call it (#1360 D2, #1361).
4. Gates in `crates/host-core/tests/warm_successor.rs` and the control-plane unit tests.

## Authorized paths

- `crates/host-core/src/warm.rs`, `crates/host-core/src/prepare.rs`
- `crates/control-plane/src/`
- `crates/host-core/tests/warm_successor.rs`, and the source-report assertions in host-core and
  capi tests

## Non-goals

- The transition itself (#1397). C ABI and browser wiring and their docs (#1360, #1361).
- No tuning of `P_MAX_SAMPLES` (#1286).

## Objective gates

1. **Never fed.** In #1355 gate 1's setup, the added growth's carried sources are fed no frame
   past what A needs for its next block. Render runs on. The control call at render clock
   `not_before + PRIME_DEADLINE_SAMPLES - q` publishes nothing. The first control call at or after
   `not_before + PRIME_DEADLINE_SAMPLES` withdraws the candidate and publishes the transition.
   Its adoption reports `TRANSITION_FALLBACK`, and `transition_fallback_count` grows by 1. Until
   that adoption, the output equals A continued.
2. **Render adopts first.** In gate 1's setup, the deadline passes with frames withheld. Then the
   frames are queued and render runs one block, which adopts the candidate, before the next
   control call. That call's `withdraw()` returns `Taken`. It publishes nothing, the revision
   completes `EXACT`, the output equals A continued, and `transition_fallback_count` is unchanged.
3. **Paused host.** With no render for 10 times the deadline in wall time, 1,000 control calls
   publish nothing and fall back nothing. After render resumes with frames queued, the edit
   completes `EXACT`.
4. **Headroom.** With `ΣP + P = P_MAX_SAMPLES(fs)` on a default ring, a producer that keeps the
   ring full and then stalls for exactly the stall tolerance leaves readiness `true` at every
   block of the stall, and the predecessor never underruns. Check at all four launch rates.
5. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `cargo test --locked -p capi`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a deadline that never fires leaves the edit pending forever; one counted from
  publication instead of `not_before`, or one quantum short, fires at the earlier call. Red.
- Gate 2: a step that falls back without withdrawing, or that ignores `Taken`, publishes a second
  swap over an adopted plan. Red.
- Gate 3: a deadline counted in wall time falls back on a paused host. Red.
- Gate 4: rings grown by `P_MAX` without the extra quantum, or not grown, fail readiness inside
  the stall tolerance. Red.

## Dependencies

- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355).
- *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
  successor cannot adopt* (#1397): the transition the deadline takes.
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354): `P_MAX`.
- *Give a plan a source-read clock that leads its render clock* (#1396): `render_clock()`.
- *Let the control thread withdraw an unadopted candidate plan* (#1343), D5: `withdraw()`.
- *Record the swap block's cost on the 64-track console* (#1286): `P_MAX_SAMPLES`.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
