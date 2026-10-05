# Duck-swap the strips a latency growth restarts, and fall back to the transition when no catch-up can finish

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8 fallbacks, D15-9, D15-17).
Slice of *Pre-roll a successor whose latency grows* (#1287), W7 (the transition) and W10. Split
from *Fall back from a missed catch-up deadline: bounded render-thread pre-roll, then the
transition* (#1358), which depends on it. Code anchors verified on `main` at `6fb211594`; every
type named below that is not on `main` is added by the dependency cited beside it.

## Product outcome

The most common latency growth, a latent insert added to an audible strip, swaps in with every
unchanged path bit-exact and the edited strip ducked out and faded back in. When no warm
successor can be prepared, or a catch-up cannot finish, the edit still completes: the strips
whose arrival grows are duck-swapped, and the watermark reports `TRANSITION_FALLBACK`.

## Context

- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324): the strips of
  `PreparedHost::restarted_strips()` (D1), a whole pre-fader restart (D2), the arm and fire at the
  adoption block (D3), the ramped mutes and one `S` through #1325 D7's `plan_strip_transition` in
  `crates/host-core/src/transition.rs` (D4). Its D6 reports a planned duck-swap as `EXACT`.
- *Fade in a strip that a swap adds during playback* (#1288): the arm and its fire table (D2-D3),
  the delay `D` (D4).
- *Snapshot a running plan into a returned successor at a block* (#1354): D1
  `PlanAdoption::CopyAndReturn { not_before }`; D3 `warm_lead`, the only computation of `Δ` per
  surviving node, and `WarmUnavailable`.
- *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355): D7's other
  `WarmUnavailable` reasons, D8's per-epoch outcome word (`PlanPublisher::set_outcome`), D10's
  re-preparation inside `service` (donor rings, no budget refusal, a counted retry).
- *Give a plan a source-read clock that leads its render clock* (#1396): D2, an ordinary successor
  inherits its predecessor's offset; D4, `PlanPublisher::render_clock()`.
- *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311) D6 and
  *Let the control thread withdraw an unadopted candidate plan* (#1343) D5: a published candidate
  is taken back only with `PlanPublisher::withdraw()`, which yields `Withdrawn`, `Returned { reason }`,
  `Taken` (adopted) or `Nothing`.
- *Hold live edits during a catch-up and apply them at the adoption sample* (#1356) D4: the hold is
  dropped only on a path that prepares the committed model again.
- `LiveRamps::for_session(model).mute_samples` (`crates/host-core/src/live_delta.rs:39`, `:52`) is the
  session mute ramp (#1288 D5). `crates/host-core/src/transition.rs` is created by #1325 D7.

## Decisions frozen for this slice

- **D1. A warm edit that restarts strips.** When `warm_lead` is `Some` and the warm successor's
  `restarted_strips()` is not empty:
  1. the warm successor applies #1324 D2-D3 to those strips: whole pre-fader restart, armed;
  2. in the same submit, the control plane writes their ramped mutes to the running plan, as #1324
     D4 does;
  3. it publishes `CopyAndReturn { not_before }`, with `not_before` = the `render_clock()` read
     before those writes, plus one quantum (the block render may be inside), plus the session mute
     ramp, rounded up to a multiple of the quantum. So the duck has settled by B, and the copy
     carries silent strips;
  4. the catch-up and adoption run as #1355 states. The armed strips fire at `S + D` (#1288 D3).

  Every unchanged path stays exact, downstream nodes included, because the restarted strips are
  silent in both plans from B on. It is a planned duck-swap: the epoch's word stays `EXACT`
  (#1324 D6).
- **D2. The transition fallback.** One host-core entry point,
  `CatchUp::fall_back_to_transition(&mut self, ..)`, plus the submit-time branch below. It runs:
  - at submit, when warm preparation returns `WarmUnavailable` (any reason);
  - from `service`, when the catch-up cannot finish. #1358 D4 calls it for a returned pre-roll;
    this slice exposes it and tests it directly.

  Steps:
  1. If a catch-up candidate is published, control takes it with `withdraw()`. `Withdrawn` or
     `Returned { .. }` gives it back whole; `Taken` means render adopted it, and the fallback ends
     with nothing to do. The catch-up's successor and peeks are then dropped on the control thread
     (each peek's drop ends it, #1320 D3), and the hold is dropped (#1356 D4).
  2. The committed model is prepared again as an ordinary successor: #1285 floors, no lead, the
     predecessor's source-read offset (#1396 D2). Inside `service` this follows #1355 D10. At
     submit it is ordinary preparation, whose failure returns the error before commit.
  3. The strips whose arrival at any surviving node grows over the predecessor (the same
     comparison as #1354 D3's `Δ`, kept per strip) join #1324's duck set. A new host-core
     function `grown_strips(predecessor, successor)` returns them, sorted by ID.
  4. It is published as #1324 D4 publishes, with one `S`. Before publication,
     `set_outcome(epoch, TRANSITION_FALLBACK)` (#1355 D8), so the adoption advances the watermark
     with that flag and adds the covered revisions to `transition_fallback_count` (#1314 D5).
- **D3. Path.** The response path of a transition edit is `rebuild` (one of the three values
  `live`, `model_only`, `rebuild`).
- **D4. Acked-batch question.** Every path builds from the committed model, which holds every
  held edit; the carry's retargets (#1277 D5) ramp carried strips to it. A withdrawn or returned
  candidate is dropped only after its replacement is prepared from that model. A refusal at
  submit returns before commit. An ack can never precede a drop.

## Deliverables

1. D1 in the control plane's warm submit path and `crates/host-core/src/prepare.rs`.
2. D2 in `crates/host-core/src/catch_up.rs`, `grown_strips` in
   `crates/host-core/src/transition.rs`, and the submit-time branch in the control plane.
3. A `test-support` hook that makes the next re-preparation inside `service` refuse; it marks the
   refusal as injected so #1355 D10's debug assertion does not fire for it.
4. Gates in `crates/host-core/tests/warm_successor.rs` and the control-plane unit tests.

## Authorized paths

- `crates/host-core/src/catch_up.rs`, `crates/host-core/src/prepare.rs`,
  `crates/host-core/src/transition.rs`
- `crates/control-plane/src/` (package `control-plane`, created by #1309)
- `crates/host-core/tests/warm_successor.rs`

## Non-goals

- The deadline, the pre-roll, its return and the constants (#1358). Stop (#1359). C ABI and
  browser wiring (#1360, #1361).
- No change to #1324's duck itself (its D2-D5) or to #1288's fade.

## Objective gates

1. **Latent insert on an audible strip.** Predecessor A is #1355 gate 1's two tracks, both
   audible; quantum 128, 48 kHz. The edit adds a true-peak limiter insert to track 1, which grows
   the output's arrival, so `warm_lead` is `Some` and track 1 is restarted. The reference run makes
   no structural edit and mutes track 1 with the same ramped live mute at the same block. The
   swapped run's output equals the reference bit for bit in every block before `S + D`, through
   the copy, the catch-up and the adoption. From `S + D` track 1 fades in. The advance reports
   `EXACT`.
2. **Copy after the duck.** In gate 1, B is at or after the sample where the duck has settled:
   track 1's carried fader is `+0.0` at B.
3. **Transition at submit.** With `p_max` one quantum below `ΣP + P`, the edit returns OK with path
   `rebuild`. Every strip `grown_strips` names is ducked, no output step exceeds the duck ramp
   (#1324's step bound), and the advance reports `TRANSITION_FALLBACK` with
   `transition_fallback_count` up by the revisions it covers.
4. **Transition from service.** On a running catch-up, a direct `fall_back_to_transition` call
   with the injected refusal counts `catch_up_reprepare_refusals`, keeps the revision pending and
   the displaced candidate held, and completes the revision on the next `service` call with
   `TRANSITION_FALLBACK`.
5. **Adopted before the fallback.** If render adopted the catch-up candidate before
   `fall_back_to_transition` runs (`withdraw()` yields `Taken`), the call publishes nothing and
   the revision completes `EXACT`.
6. Commands:
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p control-plane --features control-plane/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a warm edit that copies before the duck settles carries the fading strip into the bus
  and moves the unedited path's bits; one that fires the arm at S instead of `S + D` differs
  early. Red.
- Gate 2: a `not_before` computed without the in-flight quantum copies one block early. Red.
- Gate 3: a bound that refuses the edit, a transition that reports `EXACT`, or a duck set without
  the grown strips steps the output. Red.
- Gate 4: a refused re-preparation that drops the revision completes it as nothing. Red.
- Gate 5: a fallback that ignores `Taken` publishes a second swap over an adopted plan. Red.

## Dependencies

- *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355).
- *Hold live edits during a catch-up and apply them at the adoption sample* (#1356).
- *Give a plan a source-read clock that leads its render clock* (#1396).
- *Snapshot a running plan into a returned successor at a block* (#1354).
- *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311).
- *Duck-swap a strip whose state cannot continue across a plan swap* (#1324).
- *Fade in a strip that a swap adds during playback* (#1288).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
