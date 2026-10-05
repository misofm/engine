# Let C ABI sends follow their source strip's mute live

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-2, D15-6, D15-13 E3).
Code anchors verified on `main` at `6fb211594`.

Slice 28 of *Submix strips and live aux sends* (#1196). Rewritten 2026-10-05 for decision 15.
Decision 14's follow-up F3 (`docs/rulings/live-update-versus-rebuild-2026-10-04.md`, "F3, rule 2")
asked whether `follows_mute` itself should be live. Decision 15 answers yes (D15-6, D15-13 E3), on
both hosts. This issue is the C ABI half, and *Make a send's follows_mute live in the browser*
(#1342) is the browser half. Decision 15 supersedes the old capi-owned `LiveRouteState` mirror and
the per-queue `BACKPRESSURE` gates (D15-2).

## Product outcome

On a C ABI host, both of these are value-only, with no plan rebuild:

- muting or unmuting a track or a bus whose mute a send follows;
- turning a send's `follows_mute` on or off.

The strip's mute and every `follows_mute` send from it move together, over the same mute ramp, in
one all-or-nothing commit.

## Context

- **After *Deliver value-only send edits to the running C ABI plan* (#1225):**
  - the classifier (`crates/host-core/src/live_delta.rs`) pairs tracks and routes into submixes,
    and pairs submixes after *Deliver value-only submix-strip fader, mute and pan edits to the
    running C ABI plan* (#1390), which makes a bus's own mute live;
  - it emits route records from `route_target(model, route)`, which computes `follow_zeroed` from
    the source strip's `effective_strip_faders()` mute (#1225 D3), exactly as preparation does
    (`crates/graph-compiler/src/compile.rs:323-358`);
  - every live value is a latest-target cell (#1312; route lanes #1347).
- **What is still guarded.** Two rules keep a follow edit structural:
  - **Guard G2.** At `6fb211594` it is `LiveRebuild::FollowedMute`
    (`crates/host-core/src/live_delta.rs:133-135`), raised at `:276-280` through
    `follows_mute_from` (`:543-549`). A lane-mute change on a strip that a `follows_mute` route in
    `next` reads is structural.
  - **#1225 D1.** It keeps `follows_mute` itself structural.
- **Why the guard can go.** The follow composition is a function of the post-commit model. A
  following send's gated coefficients change only through `follow_zeroed`, which the existing
  route ramp carries (decision 14, `follows_mute` row of the edit table, and F3). The route record
  that #1225 D3 builds from `next` therefore already holds the right target. The C ABI has no
  solo, so a strip's effective mute there is its `effective_strip_faders()` mute: its own mute, or
  the mute of any VCA that reaches it.
- **The browser's mirror is not used here.** `LiveRouteState` holds `follows_mute` "Fixed for the
  plan" (`crates/host-core/src/live_route_state.rs:45-46`), and `LiveRouteMuteFollow::delta`
  (`:242-257`) composes the browser's solo state. The C ABI diffs models instead (#1225 D3) and
  never reads the mirror. The browser half (#1342) needs one mirror setter, which D7 adds here.
- **A follow is a send's property.** Validation refuses `follows_mute: true` on a route into the
  output (#1218), so every follow record targets a live route.
- **Superseded tests.** These tests assert today's G2 behaviour:
  - `a_followed_mute_change_needs_a_rebuild` (`crates/host-core/tests/live_delta.rs:412`);
  - the G2 case ("a followed mute") of
    `deltas_outside_the_live_set_rebuild_and_a_domain_failure_pushes_nothing`
    (`crates/capi/src/runtime/live_tests.rs:1015`);
  - #1225's gate-2 cases for `0x0507` and for a follow-source mute;
  - #1390's G2 cases for a bus source: its gate-3 case (a bus mute that a `follows_mute` send
    reads) and its gate-4 `FollowedMute` unit test (#1390 D4 extends G2 to submix sources).

## Decisions frozen for this slice

- **D1. Remove guard G2.** Delete `LiveRebuild::FollowedMute` and `follows_mute_from`. A lane-mute
  change on any strip is classified as today's mute rules say, whether or not a send follows it.
- **D2. `follows_mute` is live.** On a route into a submix, the classifier masks `follows_mute`
  out of the structural comparison, as it masks `gain_db`, `mute` and `channel_matrix`.
- **D3. One composition.** The classifier adds no new record type. After D1 and D2, #1225 D3's
  route diff already yields every follow record:
  - a source strip's mute change moves `follow_zeroed` for each route that follows it;
  - a `follows_mute` toggle moves `follow_zeroed` whenever the source has a muted lane.

  The classifier yields one route record per changed target and none for an unchanged one.
- **D4. One commit.** Strip mute records and route records, follow records included, are built and
  checked together. Then they are written to their cells and committed, in #1225 D6's order. A
  domain refusal on any record writes no cell.
- **D5. Ramps (D15-1).** A follow record is a gate change, so it uses the session's mute length,
  the same length as the strip mute record it follows (#1225 D4), including a per-edit length the
  strip's mute edit carries: its `Mute` entry, read through `LiveRamps::resolve` (*Carry an
  optional per-edit ramp length on live session edits*, #1394 D6). The strip and its sends
  therefore ramp together.
- **D6. Stored automation.** No host renders stored mute automation yet. When *Research: render
  stored session automation in the engine, identically on every platform* (#1058) designs mute
  automation, a following send must follow the automated mute through this same `route_target`
  composition. This issue adds nothing for it.
- **D7. The shared mirror setter.** Add `LiveRouteState::set_follows_mute(route, follows_mute) ->
  bool` beside `set_mute` (`crates/host-core/src/live_route_state.rs:181`): it goes through
  `update`, so the shadow and `rollback` cover it, and it returns `false`, changing nothing, for
  an unknown index. When it turns the flag off it also sets `source_lane_muted` to `[false; 2]`,
  so the mirror keeps the invariant of `LiveRoute::source_lane_muted` (`:51-53`). Replace "Fixed
  for the plan" in the field's doc (`:45`) with a pointer to the setter. The C ABI does not call
  it; *Make a send's follows_mute live in the browser* (#1342) does. A host-core unit test covers
  set, rollback and the unknown index.

## Deliverables

- D1-D5 in the classifier and in `crates/control-plane`'s live commit path; D7 in host-core.
- `docs/C_ABI_V1_QUALIFICATION.md`: a mute on a strip that sends follow is value-only, its sends
  follow in the same commit, and `follows_mute` is value-only.
- The superseded tests in Context, deleted or inverted in the same PR.
- In `.github/ISSUE_SPECS/1053-*.md`, if that spec is still in the directory: mark guard G2's
  bullet as superseded by this issue.

## Authorized paths

- `crates/host-core/src/live_delta.rs` (the classifier).
- `crates/control-plane/src/control.rs`: the live commit path (moved there by #1309).
- `crates/host-core/tests/live_delta.rs`, `crates/capi/src/runtime/live_tests.rs` and
  `crates/capi/tests/resource_lifecycle.rs`.
- `crates/host-core/src/live_route_state.rs`: D7's setter, the field doc and its unit test only.
- `docs/C_ABI_V1_QUALIFICATION.md`.
- `.github/ISSUE_SPECS/1053-*.md`: guard G2's bullet only.
- This spec.

## Non-goals

- No browser change (#1342). No change to `LiveRouteMuteFollow`, and none to `LiveRouteState`
  beyond D7's setter.
- No solo on the C ABI. No follow on a route into the output, which validation refuses.
- No VCA work: #1247 removes guard G3. This slice reads the effective mute through
  `effective_strip_faders()`, which is correct once G3 goes.
- No new C symbol, opcode or field, and no struct layout change.

## Hazards

- **A redundant record moves bits.** A strip mute that changes no follower's `follow_zeroed` must
  emit no route record. Neither must a toggle of `follows_mute` while the source is unmuted.
- **One transaction, both edits.** When one transaction mutes a source and edits one of its sends,
  the route record must carry the edited gain and the new `follow_zeroed` together. D3 gets this
  right only because it reads `next` alone.
- **Delayed sends.** A follow-muted delayed send stays active and mixes zeros (`DESIGN.md` P4,
  `docs/handoffs/submix-sends-2026-10-02/DESIGN.md`). Its settled comparison point includes its
  compensation delay.

## Objective gates

1. **PCM through the C ABI.**
   - Fixture: track `t` sends `pre_fader` with `follows_mute: true` into bus `b`, and `b` routes
     to the output. `t` and a second track feed distinct, non-constant signals on each lane, and
     the send's matrix is asymmetric.
   - Through `SESSION_TRANSACTION_APPLY`: `0x020f` mutes `t`'s left lane, then both lanes, then
     unmutes it. Then `0x0507` turns `follows_mute` off while `t` is muted, and back on.
   - Each edit keeps the same plan: no new epoch, no re-seek and no silent block.
   - The comparison point is `latency_samples`, plus the mute ramp, plus the send's compensation
     delay. From there the output is bit-identical to a plan compiled from the committed model.
   - Run with 1 and 10 tracks at 44.1, 48, 88.2 and 96 kHz.
2. **No redundant record** (classifier unit tests).
   - Muting a track that no route follows yields no route record.
   - Re-muting a muted source yields none.
   - Toggling `follows_mute` on an unmuted source yields none.
   - Flipping only the right lane yields one record, with only the right column zeroed.
3. **All or nothing.** In a transaction that mutes `t` and sets an out-of-domain gain on its send,
   the gain fails the domain check. No cell is written, and the model, revision and replay cache
   are unchanged.
4. **Realtime.** The race test
   `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`
   (`crates/capi/tests/resource_lifecycle.rs:2900`) also races follow-source mutes and `0x0507`
   toggles, over 20 runs:
   - `allocations == 0` and `frees == 0` around every render call after warm-up;
   - zero `INTERNAL` results;
   - a final block bit-identical to a fresh plan of the final committed model.
5. **Unchanged behaviour and policy.** The same commands as gate 6 of #1225:
   - `audit capi`;
   - `check-capi-abi.sh` and its self-test;
   - the workspace test step;
   - fmt and clippy;
   - the host-core, realtime and workspace policy scripts;
   - `check-cross-targets.sh`;
   - `run-aarch64-tests.sh debug`.

## Test value

- Gate 1 turns red if the follow records are not written with the strip mute, zero the wrong
  column or carry a stale gain, or if a follow-source mute or a `follows_mute` toggle is still
  structural.
- Gate 2 turns red if the composition re-emits unchanged targets. That re-enters the ramp kernel
  and moves a settled lane's zero sign.
- Gate 3 turns red if the strip mute is written while a record in its transaction is refused.
- Gate 4 turns red if the follow composition allocates on render, or if a racing mute lands in the
  retiring plan.
- D7's unit test turns red if the setter bypasses the shadow (a refused browser transaction would
  keep the flipped flag) or leaves stale `source_lane_muted` lanes after turning the flag off.

## Dependencies

- *Deliver value-only send edits to the running C ABI plan* (#1225)
- *Deliver value-only submix-strip fader, mute and pan edits to the running C ABI plan* (#1390),
  for a bus whose mute a send follows
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309)
- *Hold live values in latest-target cells on both hosts* (#1312)
- *Hold route-lane values in latest-target cells* (#1347)
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054)
- *Carry an optional per-edit ramp length on live session edits* (#1394), for D5
