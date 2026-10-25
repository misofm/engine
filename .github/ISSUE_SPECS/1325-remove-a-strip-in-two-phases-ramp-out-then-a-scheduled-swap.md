# Remove a strip in two phases: ramp out, then a scheduled swap

Stream D of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-9).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

When a structural edit removes a track or a submix while audio plays, the strip fades out over the
session mute ramp and only then leaves the plan. Today its contribution stops dead at the swap
block (#1269 P4), which is a click. Its sends fade with it, the ones that tap before its fader
included. The removed strip's source keeps playing until it is gone, so the fade is made of real
audio, not of underrun zeros. A transaction that restores the strip during the fade brings it back
with a fade-in; it never leaves it muted, and the PCM the host already gave the strip's source
keeps playing. This slice also builds the shared "ramp on the
predecessor, then a scheduled swap" step that the duck-swap (#1324) reuses.

## Context

- Today a structural transaction prepares a successor and publishes it for the next block:
  `SessionState::command`'s structural arm (`crates/control-plane/src/control.rs:1043` onward,
  `prepare_runtime` with a `SuccessorBase`). #1309 moves this into a portable control-plane crate.
- A pending candidate makes the next structural edit BACKPRESSURE
  (`crates/control-plane/src/control.rs:1095`); #1310 replaces that with compare-and-swap supersession.
- Source calls address the newest committed session: `newest_providers` (`control.rs:1628`), used by
  `submit` (`:1641`), `seek` (`:1655`) and `seek_at` (`:1671`). A source the transaction removed is
  refused at once as `source.id.unknown`, and its accepted PCM is discarded with its plan
  (`crates/capi/include/miso_engine_v1.h:85-94`).
- The default source ring hides 100 ms (`default_source_ring_frames`,
  `crates/host-core/src/prepare.rs:65`), but a session mute ramp may be up to 1000 ms (#1054 bounds),
  so the ramp needs PCM submitted after the commit.
- Every C ABI plan has each strip's live fader/mute lane (`C_ABI_LIVE_LANES`,
  `crates/control-plane/src/compile.rs:25`, attached in `prepare_runtime`, `:578-606`). The browser
  gets them from #1326.
- Render publishes the start of its next block after every block: `SharedPlanState::render_sample`
  (`crates/control-plane/src/plan.rs:13`, stored at `:263`).
- Live mute ramps: `LiveRamps::for_session(..).mute_samples` (`crates/host-core/src/live_delta.rs:52`),
  0 until #1054.
- A route's source names a strip and a tap (`RouteSource`, `crates/session/src/model.rs:804-819`).
  Five taps precede the fader: `input`, `post_input`, `insert_send`, `insert_return`, `pre_fader`
  (`SendTap`, `:856-871`). A fader mute does not reach a send from those taps.
- A route mixes its input after its compensation delay (`RouteTiming { compensation_delay, .. }`,
  `crates/graph/src/lib.rs:337-340`), so a route's line still holds up to that many samples of
  audio after its source settles.
- A successor is prepared against `SuccessorBase { inventory, committed }`
  (`crates/host-core/src/prepare.rs:641-647`). Across a withdrawn candidate, #1310 D2-D3 build it
  from the running plan's kept model and carry from that plan only the rows the candidate carried.

## Decisions frozen for this slice

- **D1. Which strips.** Every strip (track or submix) whose ID is in the displaced plan's committed
  model and absent from the transaction's model, except a strip already in the running plan's duck
  overlay (D6). The same exception holds for every route D2 would write.
- **D2. Phase 1, the ramp.** After the transaction's fallible steps and before it is acknowledged,
  the control plane writes, with `ramp = N = LiveRamps::for_session(next model).mute_samples`:
  - for each removed strip, a mute of both channels into the displaced plan's strip lane (its
    latest-target cell, #1312, which cannot refuse a write);
  - for each route out of a removed strip whose tap precedes the fader, a mute into that route's
    lane in the displaced plan (a #1347 cell; *Give every route whose tap precedes its strip's
    fader a live lane on every plan* (#1391) puts one on every such route, on both hosts, whatever
    its destination).

  Then it reads `p = render_sample`. Routes from the post-fader and post-pan taps pass the fader
  and need no write.
- **D3. Phase 2, the scheduled swap.** The successor is published to adopt no earlier than
  `S = ceil_q(p + q + N + C)` (#1311), where `q` is the quantum, `ceil_q` rounds up to a multiple
  of it, and `C` is the largest `compensation_delay`, in the displaced plan, of a route out of any
  strip this transaction ducks: a removed strip, or a duck-swapped one (#1324 D4) (0 if none).
  When the transaction also restarts strips, #1324 D4 extends this `C` with the sidechain lines
  that a restarted strip's taps feed into carried nodes; every duck-swap uses that `C`.
  Proof: the writes precede the read of `p`, so render drains them no later than the block that
  starts at `p + q`; every ramp ends by `p + q + N`, and every route line
  has emptied its last nonzero frame by `p + q + N + C <= S`. Between then and `S` the strip and
  its routes contribute exact `+0.0`. A paused host keeps `S` valid: render resumes at `p`.
- **D4. The source retires with phase 2, unless restored.** Until the successor is adopted,
  `submit`, `seek` and `seek_at` for a source absent from the newest committed session but present
  in the plan that still renders go to that plan's producer. From the adoption on they are refused with
  `source.id.unknown`, as today. Its ring and producer retire with the displaced plan through the
  existing retirement path; PCM queued past `S` is discarded with it (documented, as today). The host
  learns the adoption from the watermark (#1314) and stops feeding then. The producer stays in the
  running plan's source control set, which the control plane keeps in that plan's epoch; the duck
  overlay (D6) lists the source. A transaction that restores the source in phase 1 takes over its
  producer and ring instead (D6).
  - **Under supersession the producer may sit in the withdrawn candidate.** When the removing
    transaction supersedes a candidate that kept the source, that candidate's commit already moved
    the running plan's producer into its own set (`adopt_persisting`,
    `crates/control-plane/src/control.rs:1155`; #1310 D2). #1310 D5 step 5,
    `P0.sources.adopt_persisting(&mut A.sources)`, returns it to the running plan's set before the
    withdrawn candidate is dropped (step 6), so D4's routing finds it there.
- **D5. Reporting.** The response path is `rebuild` (#1313). The revision completes when render
  adopts at `S`: the watermark (#1314) reports first sample `S` with `EXACT` (or `SUPERSEDED` when
  #1310 displaces it) and counts it in `exact_count`. A planned D15-9 transition is the designed
  result of this edit, not a fallback: it never sets `TRANSITION_FALLBACK` nor its counter, which
  belong only to the fallback of a warm successor that cannot adopt (*Duck-swap the strips a
  latency growth restarts, and fall back to the transition when a warm successor cannot adopt*,
  #1397).
- **D6. Supersession and restore: the duck overlay.** The phase-1 writes are records pushed to the
  displaced plan, so they are part of its base (D15-7, P1.4).
  - The control plane keeps them in the displaced plan's epoch as a **duck overlay**: the strips and
    routes written, the removed sources still feeding that plan (D4), and `S`. It lives as long as
    the epoch and is dropped when the epoch retires.
  - **No second write.** A newer transaction during phase 1 writes a ramped mute only for strips and
    routes not already in the overlay; those join it. A strip or route already in it is skipped:
    it is ramping to, or settled at, `+0.0`, and a second write would reach `retarget`
    (`crates/builtins/src/lib.rs:2731-2750`), which resets `remaining` to `N` and recomputes the
    step from the part-ducked level, so the ramp would end after the `S` already scheduled. The
    overlay's `S` becomes the maximum of its own and the `S` D3 gives the new writes; with no new
    write it stays unchanged.
  - Every later preparation against that epoch uses `SuccessorBase::committed` = the epoch's kept
    model (#1310 D3) with the overlay applied: each overlay strip's channels muted and each overlay
    route muted. That is exactly what the plan's cells hold.
  - A structural edit during phase 1 withdraws the candidate (#1310 D1). In the withdrawn case the
    newer candidate inherits `S` (its not-before is the maximum of both). Every overlay strip it
    keeps is restarted, because #1310 D2 carries from the running plan only rows the withdrawn
    candidate carried, and that candidate had none of a removed strip. So the newer candidate arms
    it (#1288 D2, through #1288 D1's strip-set argument) on the channels its model leaves unmuted,
    and #1363 D1 (c) arms its routes that tap before the fader. A restore therefore brings the
    strip back with the session fade-in from `S`, after the duck; it never stays muted, and it
    never steps from a part-ducked level.
  - **A restored source keeps its ring.** A source in the overlay that the newer candidate keeps
    with the same declaration and ring configuration as in the running plan carries from the
    running plan like a persisting source: the candidate's base inventory is `A.carried_base`
    (#1310 D2) plus the running plan's source row for it, so preparation makes it vacant and its
    carry program moves the running plan's consumer (ring, generation, read position, held seek)
    in at adoption (`adopt_sources`, `crates/source/src/lib.rs:1851`). Before the commit the
    control plane checks that the overlay's producer is present in the epoch's set; after the
    commit it moves it into the candidate's set (`SourceControlSet::adopt_persisting`,
    `crates/host-core/src/source.rs:286`) before publication, beside #1310 D5 step 2's producer
    moves.
    Every submit acked for it in phase 1 is in that ring, so no ack precedes a drop. A restored
    source with a different declaration or ring configuration is a new source with a fresh ring;
    the old one retires at adoption as D4 says. If a later transaction in the same phase 1 removes
    the source again, #1310 D5 step 5 moves its producer from the withdrawn candidate's set back to
    the epoch's set (it never left the overlay), so D4's routing holds.
  - In the taken case (render already adopted the older candidate) the base is that plan, the
    overlay is gone with the retired epoch, and a restored strip is an added strip (#1288).
- **D7. Shared step.** The order in the control plane is: validate and classify; prepare the
  successor (fallible; any arming happens inside preparation, #1288, #1324); then the pure
  host-core function
  `plan_strip_transition(base: &SuccessorBase, overlay: &DuckOverlay, next: &SessionModel, successor: &PreparedHost) -> StripTransition`
  (an empty `DuckOverlay` outside phase 1); then the publication reservation and D6's producer
  check; commit; D6's producer moves; the D2 writes; read `p`; publish with `NoEarlierThan(S)`.
  `StripTransition` holds the strips and routes to duck (the overlay's excluded, D6), `N`, `C`
  and the overlay entry. It reads the prepared successor because a duck set can depend on what
  preparation restarted: *Duck-swap a strip whose state cannot continue across a plan swap* (#1324 D4) adds
  `successor.restarted_strips()` to the removed strips here. It returns no arm set; arming is a
  preparation step. One transaction that both removes and adds strips uses one `S`; added strips
  fade in by #1288. A latency growth whose warm successor is prepared and whose duck set is not
  empty is published `Primed` with `not_before = S` instead of `NoEarlierThan(S)` (*Duck-swap the
  strips a latency growth restarts, and fall back to the transition when a warm successor cannot
  adopt*, #1397 D1); its duck set, ramps and `S` are this step's.
- **D8. Realtime and the acked-batch question.** Render work is the existing mute ramp and one
  not-before comparison at block entry (#1311). Every fallible step (preparation, publication and
  retirement credit, D15-17) runs before the cell writes and the commit, and a cell write cannot
    fail, so no ack precedes a drop. The restored source's producer check runs before the commit and
  its move after it cannot fail (D6).

## Deliverables

1. `crates/host-core/src/transition.rs` with D7 and `DuckOverlay`, exported from `crates/host-core/src/lib.rs`; in
   `crates/host-core/src/prepare.rs`, the overlay strips passed to #1288's arm entry point and the
   overlay's restored source rows added to the base inventory (D6).
2. The control plane's structural arm, the duck overlay and source routing (D2-D6), in the files
   #1309 creates.
3. The header paragraph (`miso_engine_v1.h:85-94`) and `docs/C_ABI_V1_QUALIFICATION.md` state the
   two phases, the feeding duty until the adoption, and `S`.
4. Tests in a new `crates/capi/tests/strip_transitions.rs` (its own binary, so `bench_support`'s
   allocator serves it, as `plan_swap_race.rs` does).

## Authorized paths

- `crates/host-core/src/transition.rs` (new), `crates/host-core/src/lib.rs`,
  `crates/host-core/src/prepare.rs` (D6's arm call and restored source rows only)
- the control-plane crate's structural transaction and source-routing files that #1309 creates
  (stream B owns them; root sequences the merge)
- `crates/capi/include/miso_engine_v1.h` (comments only; #1317 edits the same header)
- `crates/capi/tests/strip_transitions.rs` (new), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- Edited strips (#1324) and added strips (#1288).
- The browser path: the browser runs the same control-plane crate once *Run the browser control
  plane in a Worker and keep the AudioWorklet render-only* (#1332) lands, on the lanes #1326 adds.
- No crossfade between plans. A true crossfade with ghost strips is deferred by D15-9; it reopens
  on a measured, audible dip in a listening test.

## Objective gates

All through the exported C entries, quantum 128, 48 kHz, a session `controlSmoothing.muteMs` that
gives `N = 2000`, a source ring of 1024 frames, single-threaded (each command is submitted between
two render calls, so the ramp starts at `p`).

1. **Fade, then removal.** Tracks A and B play distinct, never-zero sources. A has a true-peak
   limiter insert, so B's route into the output carries a compensation delay `C > 0`. B also has a
   `pre_fader` send into submix R. Between blocks `k` and `k + 1` a transaction removes B. The
   reference run instead applies a value-only transaction that mutes B and mutes the send at the
   same point. Every block of the two runs is bit-identical until block `S/q + 8`, and the
   watermark covers the removal's revision with first sample `S` and
   `MISO_ENGINE_V1_OUTCOME_EXACT` only (`transition_fallback_count` unchanged), never earlier.
2. **The source feeds phase 1.** In gate 1, B's source is fed one block ahead only (the ring cannot
   hold the ramp). Every submit for it before the adoption returns OK and its frames are the ones
   the ramp plays (gate 1's bit-identity holds). The first submit after the adoption returns
   `MISO_ENGINE_V1_INVALID_ARGUMENT` with `source.id.unknown`.
3. **Supersession inherits S.** During phase 1 a second transaction adds track C: it returns OK
   (no BACKPRESSURE), and gate 1's blocks up to `S` are unchanged bit for bit. B is in the overlay,
   so the second transaction writes no mute for B or its send: the adoption is at exactly gate 1's
   `S` (the watermark's first sample), not at a later `S` computed from the second commit.
4. **A restore during phase 1 fades back in.** In gate 1, at block `k + 4` (mid-ramp) a second
   transaction restores B exactly as it was. It returns OK. The reference is gate 1's reference
   run (B's fader and its `pre_fader` send muted by a value-only transaction at the same point),
   continued: right after the block that ends at `S`, a second value-only transaction unmutes both,
   B's fader and the send, with ramp `N`, so both ramps start at `S`. Every block of the restore run
   equals that reference bit for bit, before `S` and after it. A fresh plan of the restored session
   is not a valid reference: muting its fader does not reach the `pre_fader` send, and it runs B's
   pre-fader state from frame 0. In both runs B carries no insert, its input filters are off and
   the session's console sections are empty, so the restart of B's strip at `S` (D6) restarts
   nothing but its lanes, which the arming then drives.
   (b) The same with B's source fed one block ahead only, as in gate 2. Every submit for it before
   and after the restore returns OK, none is refused, and from `S` on B plays the frames submitted
   for those blocks (the bit-identity above holds with the same source frames).
5. **A source removed across a supersession keeps feeding phase 1.** P0 runs gate 1's session.
   Without rendering, T1 adds a muted track C (its candidate keeps B's source, so B's producer
   moves into T1's set), then T2 removes B. T2 returns OK and supersedes T1. Feed B's source one
   block ahead only, as in gate 2. Every submit for it before the adoption returns OK, and every
   block up to `S` equals gate 1's reference bit for bit (C is muted). The first submit after the
   adoption returns `MISO_ENGINE_V1_INVALID_ARGUMENT` with `source.id.unknown`.
6. **Realtime.** On the render thread, every block from `k + 1` through the adoption block makes
   zero allocations and frees (`bench_support::alloc` thread-scoped counters, statics warmed).
7. Commands:
   - `cargo test --locked -p capi --test strip_transitions`
   - `cargo test --locked -p host-core -p capi --features host-core/test-support`
   - `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` (as in
     qualification.yml's `audit-native` job)
   - `bash scripts/check-host-core-policy.sh`, `cargo run --locked --release -q -p realtime-policy`,
     `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1: a removal that still stops at the swap block, an adoption before the ramp ends (an `S`
  computed from `p` without the `+ q`), an `S` without `C` (the delayed route's last frames cut at
  `S`), or a pre-fader send left unducked (R's input steps at `S`), turns it red.
- Gate 2: a source that retires with the commit makes the ramp play underrun zeros and refuses the
  phase-1 submits; it turns red.
- Gate 3: a superseding candidate that drops the inherited `S` swaps mid-ramp; a second write to
  an overlay strip restarts B's ramp at the second commit and moves `S` later. Either turns it red.
- Gate 4: a base that omits the overlay (the restored strip carries its ducked fader and stays
  muted), a restore that does not inherit `S` (B restarts from a part-ducked level, a step), or one
  that does not arm B (it enters at full gain) turns it red. Gate 4(b): a restored source given a
  fresh ring (its acked phase-1 PCM dropped, B plays underrun zeros from `S`) or a producer left in
  the retiring plan's set (submits refused or lost) turns it red.
- Gate 5: a supersession that drops the withdrawn candidate with the removed source's producer
  (#1310 D5 step 5 missing) leaves the running plan's entry vacant: the phase-1 submits are
  refused and the ramp plays underrun zeros. Gate 2 does not supersede, so it cannot see it.
- Gate 6: a schedule check or cell drain that allocates on the render thread turns it red.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Supersede an unadopted candidate plan by compare-and-swap* (#1310).
- *Adopt a successor plan no earlier than a scheduled sample* (#1311).
- *Hold live values in latest-target cells on both hosts* (#1312).
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054).
- *Fade in a strip that a swap adds during playback* (#1288).
- *Ramp a route that a plan swap adds to or removes from a surviving strip* (#1363).
- *Give every route whose tap precedes its strip's fader a live lane on every plan* (#1391).
- *Hold route-lane values in latest-target cells* (#1347).

## Amendment (controls coordinator, batch 15b fold, 2026-10-16)

- #1698 (batch 15b) removed `PreparedHost::carried_base`. The text above that reads the candidate's
  base inventory as `A.carried_base` (*A restored source keeps its ring*) now reads: B's base across
  a withdrawn candidate is the running plan P0's own inventory
  (`PlanStateInventory::across_withdrawn`), with its source rows filtered to those that A carried
  vacant. The restored source's carry program, the producer check and the adoption are unchanged.

## Decision record, slice 1325a (2026-10-24)

Slice 1325a implements D1-D5, D7, D8 and the overlay *record* of D6; slice 1325b does D6's restore
and gate 4. Checked against `origin/main` `068742aba`.

- **The 1325b boundary.** A transaction during phase 1 whose model holds a strip or a source of
  the running plan's duck overlay (a restore) is refused as `CommandError::Backpressure` before
  anything is prepared, so the caller republishes the withdrawn candidate. Without the base overlay
  of 1325b the restored strip would carry its ducked fader and stay muted. Once the watermark
  reports the removal, the same edit is an added strip (#1288). 1325b removes the refusal, the
  retry in `crates/capi/tests/plan_swap_race.rs` (`Control::restore_retries`) that depends on it,
  and the two restore assertions in `strip_transitions.rs`'s gate-1 test (refused mid-ramp,
  committed after the adoption).
- **D2 records come from the classifier.** `plan_strip_transition` classifies the running plan's
  base model with the overlay applied against the same with the new mutes
  (`classify_live_delta_with_monitor`, ramps `LiveRamps::for_session(next)`, no edit ramps, the
  base's monitor overlay on both sides). The records are what a value-only transaction making the
  same mutes writes, so gate 1's reference is exact, a strip or route already in the overlay gets
  no record (no second write), and a following route gets its seven-word record. A route out of a
  ducked strip is muted only if its tap precedes the fader: muting a `post_fader`/`post_pan` route
  into the output is rebuild-only (`RebuildOnly(Optimisation)`), and the fader mute reaches it.
- **D7 signature.** `plan_strip_transition(base: &SuccessorBase, overlay: &DuckOverlay, next:
  &SessionModel) -> Result<StripTransition, StripTransitionError>`. The `successor: &PreparedHost`
  argument is left out until #1324 adds `restarted_strips()` to the duck set. `S` is not a field:
  `p` is read after the writes, so `StripTransition::not_before(p, q)` returns it, and the overlay
  keeps it (`DuckOverlay::raise_not_before`, the maximum of its own and the new one). An error is an
  invariant fault (`Internal`), before anything is reserved.
- **Adoption and `p` (R1).** The reservation is taken with the classification's adoption as
  before; after the commit, the producer moves, the D2 writes and the `p` read,
  `PlanReplacementReservation::set_adoption(NoEarlierThan(S))` replaces it. Because the reservation
  borrows the publisher, `p` is read through a new `PlanReplacementReservation::render_clock`
  (the same `Acquire` load as `PlanPublisher::render_clock`).
- **Stamp (R2).** The writes go to the running plan's cells under
  `stamp_monitor(last_published)` on its gate, checked as the last fallible step before the commit
  and taken again (same key) after it, then `publish_key`. The gate never publishes `r`.
- **Which rebuilds inherit `S`.** A transaction and a monitor rebuild that supersede a candidate in
  phase 1 inherit the overlay's `S` (`NoEarlierThan`). A declared discontinuity ducks nothing and
  stays `Next` (#1323 D2: it need not continue the running plan's output); the overlay stays in the
  running epoch's kept base, so a transaction that later supersedes the discontinuity successor
  inherits `S` again.
- **D4 routing (R5).** `submit`, `seek` and `seek_at` (and their unserviced forms) look the source
  up in the kept holder's overlay first (one binary search, no allocation, no lock) and go to that
  epoch's producers on a hit, unless render has already adopted a later epoch (one `Acquire` load
  of the active epoch render publishes): then the overlay is over and the call falls through to
  the newest set, so the browser's unserviced feed after the adoption is refused as
  `source.id.unknown` before any service step (attempt 2). `session_seek` and `seek_report` are
  unchanged.
- **The browser's plain seek (attempt 2).** `hosts/host-web/src/lib.rs` `seek_source` prepared the
  consumer at the committed session's source index. Through phase 1 the plan that renders still
  has the removed source and its own indices, so the index is now the rendering plan's
  (`SessionState::rendering_source_index`, from that epoch's inventory). A source that plan does
  not render has no consumer to prepare and is answered by the control plane's routing.
- **The overlay's charge (attempt 2).** `DuckOverlay::retained_ceiling` bounds an overlay by its
  plan's strips, routes and sources (three `Box<str>` vectors at count capacity and the ID text);
  `merged`/`sorted_ids` shrink to length after dedup; the shrink is defensive (`merged`'s inputs
  are disjoint and `joined` builds a new vector, so no mutant of it is red). Each epoch's capi row carries `KEPT_BASES`
  times that ceiling at its own model (`MonitorRows::duck`), an epoch row like the monitor
  overlay's: an overlay names only the plan it ducks, and a superseding commit never changes that
  plan. The second copy is the replacement a superseding rebuild builds while the stored one
  lives. The longest vector joins the largest-allocation check.
- **`C`.** The rows of `PlanStateInventory::delay_lines` whose edge is a route's `RouteSource` or
  `RouteDestination` edge; a route's two lines add. A route has both when a successor floors its
  route node above its source's arrival: re-pointing a route to an earlier tap keeps the node at
  its old arrival (#1285), and the unit test builds exactly that. Root approved a read-only accessor
  `PlanStateInventory::delay_lines` in `prepare.rs` for this (the field was private).
- **Paths outside the scope note, root-approved.** `crates/host-core/src/prepare.rs` (the
  accessor, nothing else). Seven test files whose tests encoded the old contract (a removal adopted
  at the next block, a removed source refused and discarded at the commit):
  `crates/capi/src/runtime/tests.rs` (helpers `feed_and_render_until_direct`/`_c`; five tests render
  until the watermark reports the removal, and the D4 checks move after the adoption with both
  sides asserted), `crates/capi/src/runtime/supersede_tests.rs` (renders until the adoption),
  `crates/capi/tests/empty_boot.rs` (silence after the adoption), `crates/capi/tests/plan_swap_race.rs`
  (the race accepts BACKPRESSURE only on an add of `a-race` while its removal is committed and the
  watermark, read before the submit, has not reported it; it retries once the watermark has; the
  race ends only once the last revision is adopted, and the withdrawal count adds the retries),
  `crates/control-plane/tests/spectrum_monitor.rs` (the promotion bound), `hosts/host-web/src/tests.rs`
  (the discard count: none before `S`, two blocks after it) and `hosts/host-web/tests/control_parity.rs`
  (render until the adoption). Both hosts change: the browser runs the same control-plane rebuild.
  Attempt 2, root-approved: `hosts/host-web/src/lib.rs` (`seek_source` only).
  Also root-approved: `tools/realtime-policy/src/policy.rs` and `docs/REALTIME_DEPENDENCY_POLICY.md`
  put the new capi test binary on the unsafe-ownership list (its unsafe is the C calls), as every
  other capi test binary is.

### Gates and test value (1325a)

- `crates/capi/tests/strip_transitions.rs::a_removed_strip_ramps_out_then_leaves_at_s` (gates 1, 2,
  6): red if a removal still cuts at the swap, if `S` lacks `+ q` or `C`, if a pre-fader send is
  left unducked, if the watermark reports the removal before `S` or with another outcome, if the
  removed source is retired at the commit (refused phase-1 submits), or if a block from the commit
  through the adoption allocates on the render thread.
- `...::a_supersession_in_phase_one_keeps_s_and_the_feed` (gates 3, 5): red if a superseding
  candidate drops the inherited `S`, if a second transaction writes a second mute for an overlay
  strip, or if the producer of a source removed across a withdrawn candidate is not returned to the
  running epoch (#1310 D5 step 5).
- `host_core::transition::tests::a_transition_ducks_the_removed_strips_outside_the_overlay`: red if
  the duck set includes an overlay strip, if `C` is not the route's compensation line or not the
  sum of a route's two lines, if a route whose tap follows the fader is muted, or if the ceiling
  misses a term (a full overlay, every strip, route and source of the plan, retains exactly it).
- The restore assertions in gate 1's test: red if a phase-1 restore commits (1325b boundary).
- `capi/tests/resource_lifecycle.rs` capi epoch oracle: red if the duck row leaves the charge.
- `host-web/tests/control_parity.rs::a_removed_source_seeks_and_feeds_until_its_adoption_alike_on_both_hosts`:
  red if the browser prepares a seek at the committed session's index, or if its unserviced feed
  routes a removed source to the retiring plan after the adoption.
- Mutants, each recorded red (committed tree, restored with `git checkout` and `touch`): adoption
  left `Next`; `S` without `+ q`; `S` without `C`; no route records; source routing off; no `S`
  inheritance; no overlay exclusion and no overlay applied (second write); #1310 D5 step 5 removed;
  an allocation in `PlanAdoption::admits` (the test process aborts); and, on the unit test, no
  overlay exclusion, `C` = 0, and post-fader routes muted. Attempt 2: no restore refusal (gate 1
  red), `C` as the max of a route's lines (972 expected, 486 got), the ceiling without the
  source ID text (the full overlay's bytes differ), no duck row (resource oracle red by 834 bytes), no adoption check in the source routing and the
  committed-index browser seek (parity test red on each).

### Stated limits (1325a)

- A removed strip that is already silent (muted long before) still waits for `S`: the control plane
  cannot know that its route lines are empty.
- A submit that runs concurrently with the render call that adopts the successor is treated as
  coming before the adoption: it is accepted, and its PCM is discarded with P0.
- The `StripTransition` built inside one rebuild is a transient control-thread value, as the
  classifier's own working sets are, and is not charged.

## Decision record, slice 1325b (2026-10-25)

Slice 1325b does D6's restore and gate 4. Checked against `origin/main` `de42aac5a` (#1363 slice 1
and 1325a merged).

- **The base overlay.** `SuccessorBase` gains `duck: &DuckOverlay` (neutral: empty). The
  preparation lays it over `committed` once, at its start (`DuckOverlay::apply_to` on a
  control-thread copy that drops with the call), beside the discontinuity substitution: every join
  reads `committed`, so the effective values the carry starts from are what the running plan's
  cells hold, by the same function `plan_strip_transition` uses for the phase-1 records. A
  transient copy, not a stored model: nothing the restore adds outlives the call, so no charge
  changes (the overlay only loses restored sources).
- **The restored strip carries; it is not armed (D6 corrected against today's code).** D6 says an
  overlay strip the newer candidate keeps is restarted and so armed (#1288) with its pre-fader
  routes (#1363 D1 (c)). Since #1698, the base across a withdrawn candidate is P0's own inventory,
  with only its source rows filtered, so the restored strip, its routes and its compensation lines
  carry from P0 as a direct P0 -> B swap carries them. With the overlay in the base, its fader
  mute and its pre-fader send's mute differ between the base (muted) and the model (unmuted), so
  the carry writes them as retargets at the session ramp `N` from the adoption at the inherited
  `S`, where the cells have settled at `+0.0`: the strip ramps back in from its ducked level, which
  is root's stated outcome, with no restart and no arm. An arm on a carried strip would fight the
  carried lane, so none is added. Gate 4's reference (a value-only unmute at `S`) is matched bit
  for bit; its "no insert, filters off" conditions are kept but no longer needed.
- **The restored source.** `PlanStateInventory::across_withdrawn_in_phase_one` (P0's inventory
  across A, plus the source rows the overlay lists; `across_withdrawn` is it with an empty
  overlay) gives B P0's row for a removed source, so a restore with the same declaration and ring
  configuration carries P0's ring at adoption; any other declaration starts fresh. After the
  commit, beside #1310 D5 step 2, `adopt_persisting` moves P0's producer into B's set: every other
  entry B holds vacant took its producer from A's set first, so only restored ones move. The move
  cannot fail; the producer is in P0's set by construction (the overlay lists the source only
  while P0 holds it, and D5 step 5 returns it), so no separate pre-commit check was added; a debug
  assertion bounds the moves by the restored count.
- **A restored source leaves the overlay (amends D6's "it never left the overlay").**
  `StripTransition::restored_sources` lists the overlay sources the next model keeps, and
  `DuckOverlay::joined` drops them, so D4's routing sends their calls to the newest committed
  session (where the producer now is). Restored strips and routes stay: P0's cells still hold their
  ducks, so a later transaction writes them no second ramp and its base keeps composing them. If a
  later transaction in the same phase 1 removes the source again, it is a removed source of P0's
  base not in the overlay, so it rejoins it, and #1310 D5 step 5 returns its producer to P0.
- **Deleted (the 1325a interim).** The restore refusal in `rebuild`; the two restore assertions in
  `strip_transitions.rs`'s gate-1 test; `plan_swap_race.rs`'s `removal`/`restore_retries`/`retried`
  fields, the retry wait, the `in_phase_one` arm and the withdrawal term (the race's
  `race_over` wait for the last adoption stays: removals still adopt at `S`). With the retry gone,
  the paced race (two blocks between admissions) met an add during its removal's phase 1, which now
  supersedes it, and its `superseded == 0` check went red; the paced mode now also waits for the
  watermark to report the last admission before the next edit, which is what "each candidate
  adopted before the next" meant before removals adopted at `S`. The back-to-back race restores
  `a-race` in phase 1 on every other edit and settles to a fresh plan of the final model.
- **Paths.** `crates/host-core/src/prepare.rs` (the base field, its composition, the phase-one
  inventory), `transition.rs`, `crates/control-plane/src/control.rs`, the capi tests, the header
  paragraph (comments only) and `docs/C_ABI_V1_QUALIFICATION.md`.

### Gates and test value (1325b)

- `crates/capi/tests/strip_transitions.rs::a_restore_in_phase_one_fades_back_in_from_s` (gates
  4 (a) and (b) in one run: gate 1's feed is already one block ahead): red if the successor's base
  omits the overlay (the restored strip keeps its ducked fader and stays muted after the adoption),
  if the restore does not inherit `S` (adopted mid-ramp, ramping up from a part-ducked level), if
  the restored source gets a fresh ring, if its producer stays in the running plan's set, or if it
  stays in the overlay (its submits are refused after the restore).
- Mutants, each recorded red on the committed tree (restored with `git checkout` and `touch`):
  no overlay composition (block 31, the first after `S`, differs: `-7.65e-18` against
  `1.81e-4`); overlay routes not composed (the same block differs); no `S` inheritance (adopted at
  1408, not 3584; gate 3's test is red too); fresh ring (P0's row dropped) and no producer move
  and restored source kept in the overlay (each: the first submit after the restore is refused).

- PR evidence (the verifier's probe, not a committed test: one test per claim): a remove, restore,
  remove of `b` and `src-b` in one phase 1 matches gate 1's reference bit for bit, `src-b` is
  accepted until the adoption and refused after it, and the adoption is at `S = 3584`.
- Merge with #1354a (`origin/main` after PR #1979): `SuccessorBase` holds both `warm` and `duck`
  with both neutral values, and the duck substitution runs after the warm-lead check, which must
  precede the preparation's first allocation (the ducked copy allocates).

### Stated limits (1325b)

- A restore declared with another source declaration or ring configuration starts its source
  fresh at generation 1, frame 0, as an added source does; P0's ring drains until `S` and retires
  with P0.
