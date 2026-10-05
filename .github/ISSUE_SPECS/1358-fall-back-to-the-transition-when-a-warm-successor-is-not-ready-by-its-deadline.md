# Fall back to the transition when a warm successor is not ready by its deadline

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-9, D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287): the deadline
and its step. The ring headroom and its pins were split to *Grow the default source ring by the
warm-prime headroom* (#1406) in round 6. Code anchors verified on `main` at `6fb211594`.

## Product outcome

A latency-growing edit always completes, and the host always learns how.
- A warm successor that render has not adopted within `prime_deadline_samples` of render is
  withdrawn by the next control call and replaced by the transition: `TRANSITION_FALLBACK`. That
  happens when a source is never fed far enough ahead, a producer stalls past its tolerance, or
  seeks keep landing in the prime window.
- If render adopts it first, the edit completes `EXACT`, even after the deadline.
- A paused host never falls back. Its edit stays pending until render resumes.

## Context

- *Grow the default source ring by the warm-prime headroom* (#1406) D1-D2: `p_max_samples(fs, q)`
  and a default ring of `stall_ring_frames(fs, q) + p_max_samples(fs, q) + q`, so a host that
  keeps its rings full holds `P_MAX + q` frames past each consumer through any producer stall up
  to the tolerance.
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355): render adopts
  a `Primed` candidate at the first block at or after `not_before` where every carried ring passes
  *Let a source consumer check and replay its next blocks for a prime* (#1320) D2.
- *Classify a latency-growth edit and publish its warm successor from the control plane* (#1403):
  `publish_primed` returns a `PrimedCandidate { epoch, not_before, lead_blocks }` record, which the
  control plane keeps while that candidate is pending and drops on adoption, supersession or a
  declared stop (D3, D4).
- *Give a plan a source-read clock that leads its render clock* (#1396) D4 adds
  `PlanPublisher::render_clock()`, which does not move while render is paused.
- *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
  successor cannot adopt* (#1397) D2: `fall_back_to_transition`, which takes the withdrawn
  candidate as its donor.
- `P_MAX` is the bound on `ΣP + P` that warm preparation checks
  (`WarmUnavailable::LeadBound`, *Prepare a warm successor whose carried nodes lead the
  predecessor by P*, #1354), with the value of #1406 D1's `p_max_samples(fs, q)` (*Record the swap
  block's cost on the 64-track console*, #1286 D3).
- D15-8 once listed "a host that renders nothing" as a fallback trigger. D15-17 supersedes that:
  the deadline is counted in render samples.

## Decisions frozen for this slice

- **D1. Deadline.** `prime_deadline_samples(fs, q)`, a `const fn` in
  `crates/host-core/src/warm.rs`:
  `T(fs, q) + p_max_samples(fs, q) + q`, where `T(fs, q) = ceil_q(fs * SOURCE_STALL_TOLERANCE_MS /
  1000)` is one stall tolerance in whole quanta. It is counted in render samples from
  `not_before`, which is never earlier than the render clock read at publication.
  - Why this value. With #1406 D2's headroom, a host that keeps its rings full holds at least
    `P_MAX + q` frames past each consumer through any producer stall up to the tolerance. So
    queued frames alone never delay readiness for such a host. The other thing that delays
    readiness is a held seek or command inside the prime window. A held seek is anchored below
    `S + O + P + q`, so it applies within `P + q <= P_MAX + q` render samples. After it applies,
    the host gets one stall tolerance to queue the new generation `P + q` frames ahead.
  - A candidate not adopted by then belongs to a host that does not keep `P + q` frames queued, or
    that keeps seeking inside the window. No exact mechanism can serve that host, because it needs
    those future frames. It gets the transition.
  - No other spec restates this formula.
- **D2. Ring headroom.** Moved to *Grow the default source ring by the warm-prime headroom* (#1406)
  D2 in round 6, with every pin of the ring rule.
- **D3. The deadline step.** A new host-core function
  `check_prime_deadline(publisher, pending: &PrimedCandidate) -> DeadlineStep`. The control plane
  runs it once per control call while it holds a #1403 record (#1360 D2 for the C ABI, #1361 for
  the browser).
  - If `pending.epoch` is not the newest epoch the control plane has published, the record is
    stale: the step drops it and does nothing else. #1403 D4 makes this unreachable; the check
    keeps a stale record from ever withdrawing another candidate.
  - If `render_clock() < not_before + prime_deadline_samples`, it returns `Pending` and does
    nothing. A render clock that does not move never passes it.
  - Otherwise it calls `withdraw()` (#1343 D5):
    - `Taken`: render adopted the candidate exactly. The step drops the record and returns
      `Adopted`, and the revision completes `EXACT` through the watermark.
    - `Nothing`: no candidate is pending. The step drops the record and does nothing else.
    - `Withdrawn(c)` with `c`'s epoch equal to `pending.epoch`: the step hands `c` to
      `fall_back_to_transition` (#1397 D2) as its donor. When that succeeds, it has re-prepared the
      committed model, published the transition and counted `TRANSITION_FALLBACK`, and the step
      drops the record.
    - If #1397 D2's re-preparation is refused (its step 3), the step republishes the donor `c` as
      #1310 D4 republishes: the same kind (`Primed`, with its `not_before` and `lead_blocks`), its
      own retirement credit, and its revision, `superseded` and `outcome` words. Republishing
      cannot fail: the control thread is the only publisher and the cell is `Empty`. Its rings'
      `prime_required` flags are as `publish_primed` wrote them, since nothing was published in
      between. The step then
      sets the record again, unchanged (same epoch, `not_before` and `lead_blocks`). So no donor is
      ever held outside the mailbox. Until the next control call, `c` is an ordinary pending
      candidate: render may still adopt it (the next step sees `Taken` and the revision completes
      `EXACT`), a structural edit supersedes it by #1310 and a declared stop by #1323 D2, each of
      which drops the record. Otherwise the next call's step is past the deadline at once and
      withdraws it again.
    - `Withdrawn(c)` with another epoch: the step republishes `c` unchanged, as #1310 D4
      republishes (same kind, revision and words), drops the record, and fires a debug assertion.
  - One step does at most one withdrawal, one re-preparation and one republication, and never
    waits on render.
- **D4. Acked-batch question.** The deadline step drops nothing. A `Taken` candidate is adopted
  with every edit in its cells. A withdrawn one is kept as the donor until its replacement is
  prepared from the committed model, which holds every acked edit (#1397 D4); on a refusal it goes
  back into the mailbox whole, with every ring and acked chunk an added source holds, and any later
  path takes it as it takes a pending candidate (#1310 D8, #1323). While render waits
  for readiness, the predecessor keeps playing every queued frame. An ack can never precede a
  drop.
- **D5. `p_max_samples`.** Moved to #1406 D1 in round 6. D1 reads it.

## Deliverables

1. D1 and D3 in `crates/host-core/src/warm.rs`.
2. One control-plane method in `crates/control-plane/src/` that runs D3 for #1403's record. The
   hosts' service steps call it (#1360 D2, #1361).
3. Gates in `crates/host-core/tests/warm_successor.rs` and the control-plane unit tests.

## Authorized paths

- `crates/host-core/src/warm.rs`
- `crates/control-plane/src/`
- `crates/host-core/tests/warm_successor.rs`

## Non-goals

- The transition itself (#1397). C ABI and browser wiring and their docs (#1360, #1361).
- The ring headroom and its pins (#1406). No tuning of `p_max_samples` (#1286).

## Objective gates

1. **Never fed.** In #1355 gate 1's setup, the added growth's carried sources are fed no frame
   past what A needs for its next block. Render runs on. The control call at render clock
   `not_before + prime_deadline_samples - q` publishes nothing. The first control call at or after
   `not_before + prime_deadline_samples` withdraws the candidate and publishes the transition.
   Its adoption reports `TRANSITION_FALLBACK`, and `transition_fallback_count` grows by 1. Until
   that adoption, the output equals A continued.
2. **Render adopts first.** In gate 1's setup, the deadline passes with frames withheld. Then the
   frames are queued and render runs one block, which adopts the candidate, before the next
   control call. That call's `withdraw()` returns `Taken`. It publishes nothing, the revision
   completes `EXACT`, the output equals A continued, and `transition_fallback_count` is unchanged.
3. **Paused host.** With no render for 10 times the deadline in wall time, 1,000 control calls
   publish nothing and fall back nothing. After render resumes with frames queued, the edit
   completes `EXACT`.
4. **A superseded candidate's deadline is not used.** The session's mute ramp is `N = 24,000`
   samples (500 ms, within #1054's bound). W is published `Primed` with frames withheld. Before
   W's deadline, a structural edit B removes W's added track again and removes track 2, which
   supersedes W by #1310. B grows nothing over the running plan, so it is a two-phase removal
   (*Remove a strip in two phases: ramp out, then a scheduled swap*, #1325) published
   `NoEarlierThan(S)` with `S = ceil_q(p + q + N + C)`, past W's `not_before +
   prime_deadline_samples` (7,040 at 48 kHz and quantum 128). Control calls run past W's deadline
   and before `S`: they publish nothing, B stays `Full`, and `transition_fallback_count` is
   unchanged. Render adopts B at `S`. Repeat with W adopted by render and the same B submitted
   after that adoption and before W's deadline: the same. Then, with frames still withheld, B's
   supersession is refused instead (one D4 cap of *Size the C ABI's plan capacities and resource
   admission for a superseding candidate* (#1398) set one below the value B's admission needs):
   #1310 D4 republishes W, and #1403 D4 keeps its record. The first control call at or after W's
   `not_before + prime_deadline_samples` withdraws W and publishes the transition, whose adoption
   reports `TRANSITION_FALLBACK`.
5. **Refused re-preparation.** Gate 1's setup with #1397's injected refusal (its deliverable 3).
   The step at the deadline withdraws W, the re-preparation is refused, and W is back in the
   mailbox `Full` with its words; the record names W's epoch, `transition_reprepare_refusals` is
   1 and the revision is pending. Then, in three runs:
   - the next control call withdraws W again and publishes the transition, and its adoption
     reports `TRANSITION_FALLBACK`;
   - instead, W's frames are queued and render runs one block first: render adopts W, the next
     call's `withdraw()` returns `Taken`, and the revision completes `EXACT`;
   - instead, a structural edit supersedes W by #1310 before the next call: its adoption reports
     `EXACT | SUPERSEDED`, no transition is published, and every chunk submitted for a source W
     added is played.
6. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a deadline that never fires leaves the edit pending forever; one counted from
  publication instead of `not_before`, or one quantum short, fires at the earlier call. Red.
- Gate 2: a step that falls back without withdrawing, or that ignores `Taken`, publishes a second
  swap over an adopted plan. Red.
- Gate 3: a deadline counted in wall time falls back on a paused host. Red.
- Gate 4: an unkeyed step, or a record kept across the supersession, withdraws B with W's
  deadline, reports it `TRANSITION_FALLBACK` and loses its `S`; a record dropped on the refused
  supersession leaves W pending forever, since the step runs only while a record exists. Red.
- Gate 5: a refusal that keeps the donor outside the mailbox and drops the record leaves the
  revision pending forever, since the step runs only while a record exists; a superseding edit
  then never takes the donor, and the added source's acked chunks are lost. A republish that resets
  the words reports the wrong outcome. Red.

## Dependencies

- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355).
- *Classify a latency-growth edit and publish its warm successor from the control plane* (#1403):
  the `PrimedCandidate` record.
- *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
  successor cannot adopt* (#1397): the transition the deadline takes.
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354): `P_MAX`.
- *Give a plan a source-read clock that leads its render clock* (#1396): `render_clock()`.
- *Let the control thread withdraw an unadopted candidate plan* (#1343), D5: `withdraw()`.
- *Size the C ABI's plan capacities and resource admission for a superseding candidate* (#1398):
  the cap that refuses gate 4's supersession.
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310): the D4 republish D3 uses.
- *Reset latency floors at a host-declared discontinuity* (#1323): the stop that takes a
  republished donor.
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325): gate 4's `S`.
- *Grow the default source ring by the warm-prime headroom* (#1406): `p_max_samples` and the
  ring headroom D1 relies on.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
