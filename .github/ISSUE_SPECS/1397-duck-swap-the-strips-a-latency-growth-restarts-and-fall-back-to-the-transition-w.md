# Duck-swap the strips a latency growth restarts, and fall back to the transition when a warm successor cannot adopt

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 (round-5 amendment), D15-9, D15-17).
Slice of *Grow latency during playback by adopting a primed warm successor* (#1287): W10 and the
transition fallback. *Fall back to the transition when a warm successor is not ready by its
deadline* (#1358) depends on it. Code anchors verified on `main` at `6fb211594`; every type named
below that is not on `main` is added by the dependency cited beside it.

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
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355): `Primed`
  publication (D3), render's readiness check (D4) and the per-epoch outcome word
  `PlanPublisher::set_outcome` (D7).
- *Give a plan a source-read clock that leads its render clock* (#1396): D2, an ordinary
  successor inherits its predecessor's offset; D4, `PlanPublisher::render_clock()`.
- *Let the control thread withdraw an unadopted candidate plan* (#1343) D5: `withdraw()` yields
  `Withdrawn`, `Taken` (adopted) or `Nothing`.
- `LiveRamps::for_session(model).mute_samples` (`crates/host-core/src/live_delta.rs:39`, `:52`) is
  the session mute ramp `N`. `crates/host-core/src/transition.rs` is created by #1325 D7.

## Decisions frozen for this slice

- **D1. A warm edit that restarts strips (W10).** When `warm_lead` is `Some` and the warm
  successor's `restarted_strips()` is not empty, the control plane runs #1325 D7's shared step
  with #1324 D4's duck set, with one change at the end:
  1. The warm successor applies #1324 D2-D3 to those strips: a whole pre-fader restart, armed.
  2. In the same transaction, the control plane writes their ramped mutes to the running plan
     (#1325 D2) and reads `p`.
  3. It publishes through `publish_primed` (#1355 D3) with `not_before` = #1325 D3's `S`, instead
     of `NoEarlierThan(S)`. So by `not_before` every restarted strip and every route line out of it
     is exact `+0.0`, which is the isolation the lemma needs.
  4. Render adopts at the first ready block at or after `not_before` (#1355 D4). The armed strips
     fire at the first block at or after that block plus `D` (#1288 D3).

  Every carried path stays exact, downstream nodes included. It is a planned duck-swap: the
  epoch's word stays `EXACT` (#1324 D6).
- **D2. The transition fallback.** One host-core entry point, `fall_back_to_transition(..)` in
  `crates/host-core/src/warm.rs`, takes an optional donor. It runs:
  - at submit, when warm preparation returns `WarmUnavailable` (any reason), with no donor;
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
     fires. The next deadline step retries. The revision stays pending and never completes as
     nothing.
  4. The duck set is `grown_strips(predecessor, successor)`, a new function in
     `crates/host-core/src/transition.rs`, sorted by ID, joined with #1324's restarted strips. It
     returns every strip whose content timing moves: a strip with a node whose arrival grows over
     the predecessor, or a strip with an outgoing edge (route, send, or its path into the output)
     whose compensation line changes length. A line that changes length restarts with a gap, so
     its strip must be ducked. When the output's arrival grows, every strip that reaches the
     output is in the set.
  5. It is published as #1324 D4 publishes, with one `S`. Before publication,
     `set_outcome(epoch, TRANSITION_FALLBACK)` (#1355 D7) is written. The adoption then advances the
     watermark with that flag, and `transition_fallback_count` grows by the covered revisions
     (#1314 D5).
- **D3. Path.** The response path of a transition edit is `rebuild`.
- **D4. Acked-batch question.** Every path builds from the committed model, which holds every
  acked edit. A donor is dropped only after its replacement is prepared from that model, and it is
  kept whole on a refusal. A refusal at submit returns before the commit. An ack can never precede
  a drop.

## Deliverables

1. D1 in the control plane's growth path and `crates/host-core/src/prepare.rs`.
2. D2 in `crates/host-core/src/warm.rs`, `grown_strips` in `crates/host-core/src/transition.rs`,
   and the submit-time branch in the control plane.
3. A `test-support` hook that makes the next re-preparation refuse. It marks the refusal as
   injected, so D2's debug assertion does not fire for it.
4. Gates in `crates/host-core/tests/warm_successor.rs` and the control-plane unit tests.

## Authorized paths

- `crates/host-core/src/warm.rs`, `crates/host-core/src/prepare.rs`,
  `crates/host-core/src/transition.rs`
- `crates/control-plane/src/` (package `control-plane`, created by #1309)
- `crates/host-core/tests/warm_successor.rs`

## Non-goals

- The deadline, its step and ring headroom (#1358). C ABI and browser wiring (#1360, #1361).
- No change to #1324's duck itself (its D2-D5) or to #1288's fade.

## Objective gates

1. **Latent insert on an audible strip.** A is #1355 gate 1's two tracks, both audible; quantum
   128, 48 kHz, every source queued at least `P + q` frames ahead. The edit adds a true-peak limiter
   insert to track 1. That grows the output's arrival, so `warm_lead` is `Some`, and track 1 is
   restarted. The reference makes no structural edit and mutes track 1 with the same ramped live
   mute at the same block. The swapped output equals the reference bit for bit in every block
   before the fire block (adoption plus `D`), through the duck, the readiness wait and the
   adoption. From the fire block track 1 fades in, and the advance reports `EXACT`. Run at all four
   launch rates and both bank widths.
2. **Adoption waits for the duck.** In gate 1, with sources ready from the start, render adopts at
   exactly the first block at or after #1325 D3's `S`. Track 1's fader and every route line out of
   it hold only `+0.0` in A's state at that block.
3. **Transition at submit.** With the warm configuration's `P_MAX` one quantum below `ΣP + P`,
   the edit returns OK with path `rebuild`. Every strip `grown_strips` names is ducked, and it
   names track 2 too, since the output's arrival grows. No output step exceeds the duck ramp
   (#1324's step bound). The advance reports `TRANSITION_FALLBACK`, and
   `transition_fallback_count` grows by the revisions it covers.
4. **Transition with a donor.** A `Primed` candidate is withdrawn and passed as the donor, with the
   injected refusal. `transition_reprepare_refusals` counts 1, the revision stays pending and the
   donor is held. The next call completes the revision with `TRANSITION_FALLBACK`. With an edit
   that also adds a source `c`, fed from frame 0 while the candidate was pending, the transition
   holds the donor's `c` ring and plays every chunk submitted since frame 0.
5. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: an adoption before the duck has settled carries the fading strip into the bus and moves
  the carried path's bits; a fire at S instead of S plus `D` differs early. Red.
- Gate 2: a `not_before` without the in-flight quantum or the route lines' `C` adopts one block
  early. Red.
- Gate 3: a bound that refuses the edit, a transition that reports `EXACT`, or a duck set without
  track 2 steps the output. Red.
- Gate 4: a refused re-preparation that drops the revision completes it as nothing; a fallback
  that drops the donor before preparing loses `c`'s acked chunks. Red.

## Dependencies

- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355).
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354).
- *Give a plan a source-read clock that leads its render clock* (#1396).
- *Adopt a successor plan no earlier than a scheduled sample* (#1311).
- *Let the control thread withdraw an unadopted candidate plan* (#1343), D5.
- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324).
- *Remove a strip in two phases: ramp out, then a scheduled swap* (#1325), D2, D3 and D7.
- *Fade in a strip that a swap adds during playback* (#1288).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Prepare a successor across a withdrawn candidate plan* (#1344), D3 and D5.
- *Size the C ABI's plan capacities and resource admission for a superseding candidate* (#1398),
  D4.
