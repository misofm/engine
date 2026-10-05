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
  `SessionState::command`'s structural arm (`crates/capi/src/runtime/control.rs:907` onward,
  `prepare_runtime` with a `SuccessorBase`). #1309 moves this into a portable control-plane crate.
- A pending candidate makes the next structural edit BACKPRESSURE
  (`crates/capi/src/runtime/control.rs:959`); #1310 replaces that with compare-and-swap supersession.
- Source calls address the newest committed session: `newest_providers` (`control.rs:1488`), used by
  `submit` (`:1501`), `seek` (`:1514`) and `seek_at` (`:1530`). A source the transaction removed is
  refused at once as `source.id.unknown`, and its accepted PCM is discarded with its plan
  (`crates/capi/include/miso_engine_v1.h:85-94`).
- The default source ring hides 100 ms (`default_source_ring_frames`,
  `crates/host-core/src/prepare.rs:65`), but a session mute ramp may be up to 1000 ms (#1054 bounds),
  so the ramp needs PCM submitted after the commit.
- Every C ABI plan has each strip's live fader/mute lane (`C_ABI_LIVE_LANES`,
  `crates/capi/src/runtime/compile.rs:18`, attached in `prepare_runtime`, `:572-600`). The browser
  gets them from #1326.
- Render publishes the start of its next block after every block: `SharedPlanState::render_sample`
  (`crates/capi/src/runtime/plan.rs:10`, stored at `:236`).
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
    `crates/capi/src/runtime/control.rs:1019`; #1310 D2). #1310 D5 step 5,
    `P0.sources.adopt_persisting(&mut A.sources)`, returns it to the running plan's set before the
    withdrawn candidate is dropped (step 6), so D4's routing finds it there.
- **D5. Reporting.** The response path is `rebuild` (#1313). The revision completes when render
  adopts at `S`: the watermark (#1314) reports first sample `S` with `EXACT` (or `SUPERSEDED` when
  #1310 displaces it) and counts it in `exact_count`. A planned D15-9 transition is the designed
  result of this edit, not a fallback: it never sets `PREROLL_FALLBACK` or `TRANSITION_FALLBACK`
  nor their counters, which belong only to the catch-up fallback (#1358).
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
  fade in by #1288.
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
   nothing but its lanes, which the arming then drives. From `S + N` on, B's output is the
   unducked reference's.
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
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-realtime-policy.sh`,
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
- *Adopt a successor plan no earlier than a scheduled sample, with a return queue* (#1311).
- *Hold live values in latest-target cells on both hosts* (#1312).
- *Report each transaction's edit path in its response* (#1313).
- *Publish an applied-revision watermark and complete edits asynchronously* (#1314).
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054).
- *Fade in a strip that a swap adds during playback* (#1288).
- *Ramp a route that a plan swap adds to or removes from a surviving strip* (#1363).
- *Give every route whose tap precedes its strip's fader a live lane on every plan* (#1391).
- *Hold route-lane values in latest-target cells* (#1347).
