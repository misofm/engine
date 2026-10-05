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
  for `P + q` frames queued past the consumer beyond that tolerance. *Prepare a warm successor whose
  carried nodes lead the predecessor by P* (#1354) D1 adds `stall_ring_frames(fs, q)` with that
  body: the fixed baseline its D2 measures a ring's headroom against.
- The rule is published and pinned outside host-core. The ABI layout document's `sourceRing`
  carries its two inputs (`tools/parameter-metadata/src/abi_layout.rs:108-115`, `:2266-2270`), and
  these pin the rule or its values: `tools/parameter-metadata/tests/abi_layout.rs:262-290`,
  `scripts/check-abi-layout-v1.py:591-595` and its self-test mutations (`:653-656`),
  `scripts/fixtures/abi-layout-v1-self-test.json:666`, the generated
  `sdk/assets/miso-engine-v1-abi-layout.json:666` and `sdk/src/generated/abi.ts:2423` (kept equal
  by `scripts/check-sdk-generated.sh`), the SDK's `defaultSourceRingFrames`
  (`sdk/src/core/abi.ts:206-212`), `hosts/host-web/src/tests.rs:1508-1535`,
  `crates/host-core/tests/prepare.rs:618-633` and `crates/capi/src/runtime/tests.rs:625-629`.
- `hosts/host-web/qualification/run.mjs:211` and `qualification.js:6-8` pin 5,120, but the stall
  test passes that ring explicitly (`bootOptions(DEFAULT_RING_FRAMES)`, `qualification.js:497-501`)
  and tests the stall body, which stays 5,120. `hosts/host-web/qualification/rebuild-cost.mjs:66-74`
  and `scripts/web-mixing-automation-benchmark.mjs:106-110` compute the stall body from the two
  inputs and pass it explicitly. None of them changes. Only the recorded label does: the
  qualification record writes `defaultRingFrames: DEFAULT_RING_FRAMES` (`qualification.js:715`,
  `run.mjs:926` pins `5120`) and the matrix prose calls it "the default source ring"
  (`generate-matrix.mjs:24`). After D2 that ring is the stall ring, not the default, so the label
  is renamed `stallRingFrames` and the prose says "stall source ring" (deliverable 5).
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
- **D2. Ring headroom.** `default_source_ring_frames(fs, q)` becomes
  `stall_ring_frames(fs, q) + P_MAX_SAMPLES(fs) + q`: #1354 D1's `stall_ring_frames`, today's
  default body, plus #1286 D3 item 1's `P_MAX_SAMPLES`, plus one quantum. That is the most
  frames readiness asks to be queued (`(k + 1) * q` with `k * q <= P_MAX`), on top of the stall
  tolerance the ring already gives the producer. Nothing is held for a deadline, so the deadline adds no term.
  - At quantum 128 the default becomes 6,912, 7,296, 12,800 and 14,080 frames at 44.1, 48, 88.2
    and 96 kHz (from 4,736, 5,120, 9,088 and 9,856).
  - The published rule gains the inputs of the new term. `sourceRing` keeps `stallToleranceMs` and
    `reserveQuanta` (the stall body, unchanged) and adds `primeGrowths` (4) and
    `primeLatencySamples`, `L_max(fs)` per launch rate (#1286 D3 item 1). The SDK's
    `defaultSourceRingFrames` applies the whole rule. The stall-body derivations listed in the
    context stay as they are.
  - Every pin listed in the context is updated to the new rule, and the generated SDK files are
    regenerated. The resource report and its exact assertions are updated.
- **D3. The deadline step.** A new host-core function
  `check_prime_deadline(publisher, pending: &PrimedCandidate) -> DeadlineStep`. The control plane
  runs it once per control call while it holds a #1403 record (#1360 D2 for the C ABI, #1361 for
  the browser).
  - If `pending.epoch` is not the newest epoch the control plane has published, the record is
    stale: the step drops it and does nothing else. #1403 D4 makes this unreachable; the check
    keeps a stale record from ever withdrawing another candidate.
  - If `render_clock() < not_before + PRIME_DEADLINE_SAMPLES`, it returns `Pending` and does
    nothing. A render clock that does not move never passes it.
  - Otherwise it calls `withdraw()` (#1343 D5):
    - `Taken`: render adopted the candidate exactly. The step drops the record and returns
      `Adopted`, and the revision completes `EXACT` through the watermark.
    - `Nothing`: no candidate is pending. The step drops the record and does nothing else.
    - `Withdrawn(c)` with `c`'s epoch equal to `pending.epoch`: the step drops the record and hands
      `c` to `fall_back_to_transition` (#1397 D2) as its donor. That re-prepares the committed
      model, publishes the transition, and counts `TRANSITION_FALLBACK`.
    - `Withdrawn(c)` with another epoch: the step republishes `c` unchanged, as #1310 D4
      republishes (same kind, revision and words), drops the record, and fires a debug assertion.
  - If #1397 D2's re-preparation was refused, the donor is kept, and the next call's step retries
    `fall_back_to_transition` with it.
  - One step does at most one withdrawal and one re-preparation, and never waits on render.
- **D4. Acked-batch question.** The deadline step drops nothing. A `Taken` candidate is adopted
  with every edit in its cells. A withdrawn one is kept as the donor until its replacement is
  prepared from the committed model, which holds every acked edit (#1397 D4). While render waits
  for readiness, the predecessor keeps playing every queued frame. An ack can never precede a
  drop.
- **D5. `P_MAX_SAMPLES`.** This slice writes `P_MAX_SAMPLES(fs)`, a `const fn` in
  `crates/host-core/src/warm.rs`, with #1286 D3 item 1's value; its comment names the record
  row. D1, D2 and the control plane's `WarmConfig` (#1360 D1) read it.

## Deliverables

1. D1, D3 and D5 in `crates/host-core/src/warm.rs`.
2. D2 in `crates/host-core/src/prepare.rs`, with the source-report assertions updated, the
   published rule in `tools/parameter-metadata`, and every pin in D2's list.
3. One control-plane method in `crates/control-plane/src/` that runs D3 for #1403's record. The
   hosts' service steps call it (#1360 D2, #1361).
4. Gates in `crates/host-core/tests/warm_successor.rs` and the control-plane unit tests.
5. The qualification record's ring label renamed `defaultRingFrames` -> `stallRingFrames` in
   `hosts/host-web/qualification/qualification.js:715`, `run.mjs:926` and the prose of
   `generate-matrix.mjs:24`; its value stays 5,120 (the stall body at 48 kHz, q = 128).

## Authorized paths

- `crates/host-core/src/warm.rs`, `crates/host-core/src/prepare.rs`
- `crates/control-plane/src/`
- `crates/host-core/tests/warm_successor.rs`, `crates/host-core/tests/prepare.rs`, and the
  source-report assertions in host-core and capi tests (`crates/capi/src/runtime/tests.rs`)
- `tools/parameter-metadata/src/abi_layout.rs`, `tools/parameter-metadata/tests/abi_layout.rs`
- `scripts/check-abi-layout-v1.py`, `scripts/fixtures/abi-layout-v1-self-test.json`
- `sdk/assets/miso-engine-v1-abi-layout.json`, `sdk/src/generated/abi.ts` (regenerated only),
  `sdk/src/core/abi.ts` (`defaultSourceRingFrames` only)
- `hosts/host-web/src/tests.rs` (`default_ring_covers_stall_tolerance` only)
- `hosts/host-web/qualification/qualification.js`, `run.mjs`, `generate-matrix.mjs` (the ring
  label of deliverable 5 only)

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
5. **A superseded candidate's deadline is not used.** W is published `Primed` with frames
   withheld. A structural edit supersedes it with a `Next` candidate B (#1310), and render does not
   adopt B. Control calls run past W's `not_before + PRIME_DEADLINE_SAMPLES`: they publish
   nothing, B stays `Full`, and `transition_fallback_count` is unchanged. Render then adopts B at
   its next block. Repeat with W adopted by render and B published after it: the same.
6. **The rule is published.** The parameter-metadata test checks the published rule against
   `default_source_ring_frames` at every launch rate and its spread of quanta, and the layout
   check accepts the new `sourceRing` and rejects a document without `primeGrowths`.
7. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `cargo test --locked -p capi`, `cargo test --locked -p parameter-metadata`,
     `cargo test --locked -p host-web`
   - `python3 scripts/check-abi-layout-v1.py --self-test`, `bash scripts/check-sdk-generated.sh`
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
- Gate 5: an unkeyed step withdraws B with W's deadline, reports it `TRANSITION_FALLBACK` and
  loses its adoption. Red.
- Gate 6: an SDK that derives the old ring sizes its producer one prime short of the engine's
  ring. Red.

## Dependencies

- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355).
- *Classify a latency-growth edit and publish its warm successor from the control plane* (#1403):
  the `PrimedCandidate` record.
- *Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm
  successor cannot adopt* (#1397): the transition the deadline takes.
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354): `P_MAX`.
- *Give a plan a source-read clock that leads its render clock* (#1396): `render_clock()`.
- *Let the control thread withdraw an unadopted candidate plan* (#1343), D5: `withdraw()`.
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310): the D4 republish D3 uses.
- *Record the swap block's cost on the 64-track console* (#1286): `P_MAX_SAMPLES`.
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
