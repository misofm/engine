# Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm successor cannot adopt

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-9, D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287): the duck of
the strips a warm growth restarts, the transition fallback and its outcome word (moved from #1355
D7 in round 5). *Fall back to the transition when a warm successor is not ready by its deadline*
(#1358) depends on it. Code anchors verified on `main` at `6fb211594`; every type named below that
is not on `main` is added by the dependency cited beside it.

## Product outcome

The most common latency growth, a latent insert added to an audible strip, swaps in with every
carried path bit-exact and the edited strip ducked out and faded back in. When no warm successor
can be prepared, or one is not ready by its deadline, the edit still completes: the strips whose
timing moves are duck-swapped, and the watermark reports `TRANSITION_FALLBACK`.

## Context

- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324): the strips of
  `PreparedHost::restarted_strips()` (D1), a whole pre-fader restart (D2), the arm (D3), and the
  duck through #1325 D7's `plan_strip_transition` (D4). Its D6 reports a planned duck-swap as
  `EXACT`.
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325): the ramped mutes (D2)
  and the scheduled sample `S = ceil_q(p + q + N + C)` (D3), after which every ducked strip and
  every route line out of it is exact `+0.0`.
- *Fade in a strip that a swap adds during playback* (#1288): the fire table (D3) and the delay
  `D` (D4). A strip fires at the first block at or after `S + D`.
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354): `warm_lead`
  over the carried nodes, `WarmUnavailable { Misaligned, LeadBound, PrimeBudget }`, and the
  restart iteration that puts a strip in `restarted_strips()` when alignment or isolation needs
  it.
- *Classify a latency-growth edit and publish its warm successor from the control plane* (#1403):
  the classification match this slice extends (D2), `publish_primed` (D3) and the
  `PrimedCandidate` record (D4).
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355): render's
  readiness check (D4).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314) D1 and D5: the
  per-candidate `outcome` word, carried by `UnadoptedCandidate`, and how an advance reports it.
- *Give a plan a source-read clock that leads its render clock* (#1396): D2, an ordinary
  successor inherits its predecessor's offset; D4, `PlanPublisher::render_clock()`.
- *Let the control thread withdraw an unadopted candidate plan* (#1343) D5: `withdraw()` yields
  `Withdrawn`, `Taken` (adopted) or `Nothing`.
- `LiveRamps::for_session(model).mute_samples` (`crates/host-core/src/live_delta.rs:39`, `:52`) is
  the session mute ramp `N`. `crates/host-core/src/transition.rs` is created by #1325 D7.

## Decisions frozen for this slice

- **D1. A warm edit that ducks strips.** This is the arm of #1403 D2's match for `Warm` with
  `lead_samples > 0` and a non-empty duck set: #1324 D4's set, the strips the edit removes plus the
  warm successor's `restarted_strips()`. A growth that only removes a strip ("replace a stem":
  remove X, add a track with a limiter) takes this arm too, so X ramps out (#1325) instead of
  stopping dead at the adoption block (#1269 P4). Until this slice, #1403 D2 routes this arm, the
  `lead_samples == 0` arm and `Unavailable` to today's path. Only `test-support` sessions hold a
  `WarmConfig` before #1360 and #1361, which depend on this
  slice, so no product session reaches these arms before they exist. That is merge order, not a
  gap. The control
  plane runs #1325 D7's shared step with #1324 D4's duck set, with one change at the end:
  1. The warm successor applies #1324 D2-D3 to its restarted strips: a whole pre-fader restart,
     armed. A strip in `restart_whole` (#1354 D2 step 6) is restarted in every owner as #1324 D2
     states for such a strip, and ducked and armed the same way. A removed strip is not in W.
  2. In the same transaction, the control plane writes the ramped mutes of the whole duck set to
     the running plan (#1325 D2) and reads `p`.
  3. It publishes through `publish_primed` (#1403 D3), admitted with `AdmissionPeak::WithReprepare`
     as #1403 D2 admits every warm candidate, with `not_before` = #1325 D3's `S` instead of
     `NoEarlierThan(S)`. `S` uses #1324 D4's `C`, the one every duck-swap uses: every route line
     out of a ducked strip (removed or restarted); every `EffectSidechain` line from a restarted
     strip's `post_fader` or `post_pan` tap into a carried node (#1354 D2 step 2 keeps that
     consumer carried); and, for a restarted submix whose `input` tap feeds a carried node under
     #1354 D2 step 2's exemption, that sidechain line plus the longest line into the stage. So by
     `not_before` every ducked strip and every line out of it holds only exact `+0.0`, and a line
     that W shortens drops only `+0.0`. That is the isolation the lemma needs.
  4. Render adopts at the first ready block at or after `not_before` (#1355 D4). The armed strips
     fire at the first block at or after that block plus `D` (#1288 D3).

  Every carried path stays exact, downstream nodes included. It is a planned duck-swap: the
  epoch's word stays `EXACT` (#1324 D6).

  In the arm for `Warm` with `lead_samples == 0` (the restarts confine the growth to R, so
  `Δ = 0` over C), there is no lead and no prime. The successor is prepared with that
  `WarmLead` (#1354 D4), so its `restart_whole` strips join `restarted_strips()`, and it is
  published as #1324 D4 publishes an ordinary duck-swap (`NoEarlierThan(S)`, not `Primed`), with
  `AdmissionPeak::Single`, and with `S` counted with #1324 D4's `C`, as every duck-swap is: a
  `restart_whole` strip whose arrival grows can shorten a `post_fader` sidechain line into a
  carried consumer. It completes `EXACT`; it is not a transition fallback.
- **D2. The transition fallback.** One host-core entry point, `fall_back_to_transition(..)` in
  `crates/host-core/src/warm.rs`, takes an optional donor. It runs:
  - at submit, in #1403 D2's arm for `Unavailable` (any reason), with no donor;
  - from the deadline step (#1358 D3), after that step has withdrawn the `Primed` candidate
    (`Withdrawal::Withdrawn`). The withdrawn candidate is the donor. A `Taken` withdrawal means
    render adopted it exactly, and the caller never calls this function.

  Steps:
  1. The committed model is prepared again as an ordinary successor: #1285 floors, no lead, and
     the predecessor's source-read offset (#1396 D2).
     - With a donor, the donor's added rings are donated (*Prepare a successor across a withdrawn
       candidate plan*, #1344 D3). They pass #1344 D5's unconsumed check, because render never
       begins a candidate's consumers before it claims it (#1355 D2).
     - The cross-plan admission is not run again. The warm submit admitted this re-preparation
       (`AdmissionPeak::WithReprepare`, *Size the C ABI's plan capacities and resource admission
       for a superseding candidate*, #1398 D4). The model and base are the ones submit prepared,
       and the floors are no higher, so the plan's resource row is no larger than the donor's.
     - At submit it is ordinary preparation, and a failure returns the error before the commit.
  2. Only after that preparation succeeds is the donation applied. Then the donor is dropped on the
     control thread. Every live edit written to its cells is also in the committed model, and the
     carry's retargets (#1277 D5) ramp the carried strips to it.
  3. If the re-preparation is refused anyway, that is a defect. The donor is kept whole,
     `transition_reprepare_refusals` (a saturating session counter) rises, and a debug assertion
     fires. The function returns the donor to its caller, the deadline step, which republishes it
     and sets its record again (#1358 D3), so it is never held outside the mailbox. The next
     deadline step retries. The revision stays pending and never completes as nothing.
  4. The duck set is `grown_strips(predecessor, successor)`, a new function in
     `crates/host-core/src/transition.rs`, sorted by ID, joined with #1324's restarted strips. It
     returns every strip whose content timing moves: a strip with a node whose arrival grows over
     the predecessor, or a strip with an outgoing edge (route, send, or its path into the output)
     whose compensation line changes length. A line that changes length restarts with a gap, so
     its strip must be ducked. When the output's arrival grows, every strip that reaches the
     output is in the set.
  5. It is published as #1324 D4 publishes, with one `S`. Before publication it writes
     `PlanReplacementReservation::set_outcome(TRANSITION_FALLBACK)` (D5). The adoption then
     advances the watermark with that flag, and `transition_fallback_count` grows by the covered
     revisions (#1314 D5).
- **D3. Path.** The response path of a transition edit is `rebuild`.
- **D4. Acked-batch question.** Every path builds from the committed model, which holds every
  acked edit. A donor is dropped only after its replacement is prepared from that model, and it is
  kept whole on a refusal. A refusal at submit returns before the commit. An ack can never precede
  a drop.
- **D5. Outcome word (moved from #1355 D7).** This slice is the only writer of an `outcome` other
  than `EXACT` (#1314 D1). The transition writes `TRANSITION_FALLBACK` on its reservation before
  publication. The word travels with the candidate: `UnadoptedCandidate` carries it, so a
  transition withdrawn by #1310 and republished on a refusal (#1310 D4) still reports
  `TRANSITION_FALLBACK`. A warm adoption, a planned duck-swap (D1) and a declared stop leave it
  `EXACT`.

## Deliverables

1. D1 in the control plane's growth path (the arms of #1403 D2's match) and
   `crates/host-core/src/prepare.rs`.
2. D2 in `crates/host-core/src/warm.rs`, `grown_strips` in `crates/host-core/src/transition.rs`,
   and the submit-time branch in the control plane. D5's write in the transition's publication.
3. A `test-support` hook that makes the next re-preparation refuse. It marks the refusal as
   injected, so D2's debug assertion does not fire for it.
4. Gates in `crates/host-core/tests/warm_successor.rs` and the control-plane unit tests.
5. The rewrite of #1403 gate 2's control-plane test, in the same change as D1. That test pins
   today's path for gate 1's growth plus the removal of track 2 (`NoEarlierThan(S)`, no record).
   D1 routes that edit to its arm, so the test now asserts that it is published `Primed` with
   `not_before = S` and that the record names its epoch, and keeps its check that no block before
   `S` differs from A continued with track 2 live-muted by the same ramped mute. The old
   assertions are deleted, not left beside the new ones.

## Authorized paths

- `crates/host-core/src/warm.rs`, `crates/host-core/src/prepare.rs`,
  `crates/host-core/src/transition.rs`
- `crates/control-plane/src/` (package `control-plane`, created by #1309)
- `crates/host-core/tests/warm_successor.rs`

## Non-goals

- The deadline and its step (#1358), and the ring headroom (#1406). C ABI and browser wiring (#1360,
  #1361).
- No change to #1324's duck itself (its D2-D5) or to #1288's fade.

## Objective gates

Every source ring is set explicitly, as in #1355's gates, to `stall_ring_frames(fs, q) +
p_max_samples(fs, q) + q` (7,296 frames at 48 kHz and quantum 128). The fixture's own 4,096 frames
(`crates/host-core/tests/support/successor.rs:33`) are below the stall body, so with them every
warm edit would be `LeadBound`.

1. **Latent insert on an audible strip.** A is #1355 gate 1's two tracks, both audible; quantum
   128, 48 kHz, every source queued at least `P + q` frames ahead. The edit adds a true-peak limiter
   insert to track 1. That grows the output's arrival, so `warm_lead` is `Some`, and track 1 is
   restarted. The reference makes no structural edit and mutes track 1 with the same ramped live
   mute at the same block.
   - Every block before the fire block (adoption plus `D`) equals the reference bit for bit,
     through the duck, the readiness wait and the adoption.
   - From the fire block on, for 64 blocks, the output equals the sum of two runs. The first is
     this swap with track 2 muted in the model, which leaves track 1's restarted chain alone on the
     output bus. The second is the reference, which leaves track 2 alone. The bus adds exactly two
     route terms and one of them is `+0.0` in each run, so the sum is bit-exact. Track 1 fades in
     from the fire block, and the advance reports `EXACT`.
   - Run at all four launch rates and both bank widths.
2. **Adoption waits for the duck.** In gate 1, with sources ready from the start, render adopts at
   exactly the first block at or after #1325 D3's `S`. Track 1's fader and every route line out of
   it hold only `+0.0` in A's state at that block. Repeat with a compressor on track 2 whose routed
   sidechain reads track 1's `post_fader` tap: `S` counts that line, adoption waits for it, and it
   holds only `+0.0` at the adoption block.
3. **Transition at submit.** With the warm configuration's `P_MAX` one quantum below `ΣP + P`,
   the edit returns OK with path `rebuild`. Every strip `grown_strips` names is ducked, and it
   names track 2 too, since the output's arrival grows. Up to the transition's `S`, every block
   equals a reference that makes no structural edit and live-mutes tracks 1 and 2 with the same
   ramped mute at the same block. From `S`, every block equals a fresh plan of the successor
   session (no lead, the predecessor's source-read offset) with tracks 1 and 2 muted and
   live-unmuted with `N` at their fire block, fed the same PCM from the frames the consumers stood
   at (#1324 gate 1's form). The advance reports `TRANSITION_FALLBACK`, and
   `transition_fallback_count` grows by the revisions it covers.
4. **Transition with a donor.** A `Primed` candidate is withdrawn and passed as the donor, with the
   injected refusal. `transition_reprepare_refusals` counts 1, the revision stays pending, and the
   donor is republished `Full` with its words and its record set again (#1358 D3). The next deadline
   step withdraws it again and completes the revision with `TRANSITION_FALLBACK`. With an edit that
   also adds a source `c`, fed from frame 0 while the candidate was pending, the transition
   holds the donor's `c` ring and plays every chunk submitted since frame 0. A structural edit
   refused while that transition is pending (#1310 D4 republishes it) leaves its word: its
   adoption still reports `TRANSITION_FALLBACK`.
5. **A warm growth that also removes a strip (moved from #1403 gate 2).** A renders sources `s`
   (track 1) and `u` (track 2), with a session mute ramp `N`. W1 is #1403 gate 1's growth,
   published `Primed` with frames withheld, so every ring is flagged. A structural edit then
   removes track 1 and adds a second muted limiter track, which supersedes W1 by #1310 with a
   warm successor W2 whose duck set is track 1, so it takes D1.
   - At W2's publication, `s`'s ring reads `prime_required() == false`, `u`'s reads `true`, and
     `not_before` is #1325 D3's `S`.
   - `u` alone is queued `P + q` frames ahead; `s` is fed exactly the frames A renders up to `S`
     and none past them. W2 is adopted at the first block at or after `S`, never before.
   - Up to that block the output equals A continued with track 1 live-muted by the same ramped
     mute at the same block, fed the same frames; from it on, for 64 blocks, it equals that
     reference continued.
6. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: an adoption before the duck has settled carries the fading strip into the bus and moves
  the carried path's bits; a fire at S instead of S plus `D` differs early; a carried path that
  drifts after the fire, or a restarted chain that starts with state, breaks the sum. Red.
- Gate 2: a `not_before` without the in-flight quantum, the route lines' `C` or the sidechain
  line's adopts early, and the shortened line drops nonzero samples. Red.
- Gate 3: a bound that refuses the edit, a transition that reports `EXACT`, or a duck set without
  track 2 breaks the references. Red.
- Gate 4: a refused re-preparation that drops the revision completes it as nothing; a refusal
  that holds the donor outside the mailbox leaves no record, so no step retries; a fallback that
  drops the donor before preparing loses `c`'s acked chunks; a republish that resets the word
  reports `EXACT`. Red.
- Gate 5: a warm growth routed on `restarted_strips()` alone publishes W2 at the render clock and
  cuts track 1 dead at the adoption block; a `publish_primed` that leaves the withdrawn
  candidate's flags alone keeps `s` flagged, so W2 waits for frames the host never sends and is
  not adopted at the first block at or after `S`. Red.

## Dependencies

- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355).
- *Classify a latency-growth edit and publish its warm successor from the control plane* (#1403).
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354).
- *Give a plan a source-read clock that leads its render clock* (#1396).
- *Adopt a successor plan no earlier than a scheduled sample* (#1311).
- *Let the control thread withdraw an unadopted candidate plan* (#1343), D5.
- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324).
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325), D2, D3 and D7.
- *Fade in a strip that a swap adds during playback* (#1288).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314): the outcome
  word D5 writes.
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310): the republish D5 relies on.
- *Prepare a successor across a withdrawn candidate plan* (#1344), D3 and D5.
- *Size the C ABI's plan capacities and resource admission for a superseding candidate* (#1398),
  D4.
