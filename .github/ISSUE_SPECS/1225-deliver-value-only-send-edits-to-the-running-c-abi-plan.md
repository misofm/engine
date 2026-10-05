# Deliver value-only send edits to the running C ABI plan

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-2, D15-6, D15-7).
Code anchors verified on `main` at `6fb211594`.

Slice 27 of *Submix strips and live aux sends* (#1196). Rewritten 2026-10-05 for decision 15. The
live commit path moves into `crates/control-plane` (#1309), live values become latest-target
cells (#1312, route lanes #1347), and every live value ramps (#1054). Decision 15 supersedes the old batch-C1 order, the
"a step until #1054" rule and the per-queue `BACKPRESSURE` gates (D15-1, D15-2, D15-11). The
design record cited as `DESIGN` is `docs/handoffs/submix-sends-2026-10-02/DESIGN.md`.

## Product outcome

A C ABI host, such as a fan's phone, can change a send's `gain_db`, `mute` or `channel_matrix` (a
route whose destination is a submix) on the running plan. The change never rebuilds the plan,
leaves a silent block, resets a source ring or steps hard. Track faders, mutes and pans are already
live (#1257). After this slice a send edit lands on the same plan, ramped, and is never lost, also
when the same transaction makes a structural edit that swaps the plan.

## Context

- **The classifier covers tracks only.** `host_core::classify_live_delta`
  (`crates/host-core/src/live_delta.rs:211`) masks each track's `fader` and `matrix_or_pan`
  (`:237-256`) and compares everything else by canonical JSON (`:257-263`). It never pairs
  submixes or routes, so any change to a submix or route value is `LiveRebuild::Structure`
  (#1053 guard G1, documented at `:155-157`). The VCA guard (`:216-218`) belongs to #1247. The
  follow-mute guard (`:276-280`, with `follows_mute_from` at `:543-549`) belongs to #1226.
- **#1309 moves capi's control plane** (`crates/capi/src/runtime/{control,compile,plan,error}.rs`,
  `commit_live` at `control.rs:1065`) into `crates/control-plane` (lib `control_plane`), with names
  unchanged. The classifier stays in `crates/host-core/src/live_delta.rs`, and capi's tests stay in
  capi. The anchors below name the locations at `6fb211594`; the implementer edits the moved code.
- **capi resolves track producers only.** `commit_live` takes `strips.controls[..track_count]`
  (`control.rs:1093-1094`) and searches it by strip ID.
- **capi attaches no route lane.** `C_ABI_LIVE_LANES` (`crates/capi/src/runtime/compile.rs:18-21`)
  is `FADER_AND_MATRIX` plus the effect lanes. `HostLiveLanes::routes`
  (`crates/host-core/src/prepare.rs:378`) is false, so `HostLiveControlHandles::route_controls`
  (`prepare.rs:465`) is empty on the C ABI. When the flag is set, host-core attaches one lane per
  route into a submix, in canonical route-ID order (`prepare.rs:1574-1590`). A route into the
  output never gets a lane. It keeps its prepared constants and its fold
  (`crates/graph/src/lib.rs:1687-1693`).
- **One coefficient authority.** `graph_compiler::route_coefficients`
  (`crates/graph-compiler/src/ids.rs:341`) lowers `(gain_db, matrix, mute, source_lane_muted)` to
  the four gated coefficients. Preparation calls it with `follow_zeroed` set from the strip's
  *effective* mute (`crates/graph-compiler/src/compile.rs:323-358`, through
  `SessionModel::effective_strip_faders`, `crates/session/src/vca.rs:100`).
  `RouteControlProducer::record` (`crates/host-core/src/route_controls.rs:86-108`) builds a live
  record from the same function. It also checks the ramp length against
  `ROUTE_RAMP_LENGTH_MAXIMUM`.
- **Route fields on the wire.** A route has `id`, `source`, `destination`, `channel_matrix`,
  `gain_db`, `mute` and `follows_mute` (`crates/session/src/visit.rs:114`), and no smoothing field.
  The opcodes are `SetRouteChannelMatrix` `0x0504`, `SetRouteGainDb` `0x0505` and `SetRouteMute`
  `0x0506` (`crates/protocol/src/model.rs:96-100`). `SetTrackFader` `0x020f` and
  `SetTrackMatrixOrPan` `0x0210` address a submix by its ID (#1204).
- **Tests that stay.** `a_submix_fader_change_is_structural`
  (`crates/host-core/tests/live_delta.rs:402`) and the G1 case ("a submix fader") of
  `deltas_outside_the_live_set_rebuild_and_a_domain_failure_pushes_nothing`
  (`crates/capi/src/runtime/live_tests.rs:1015`; its fixture `submix_session` is at `:969`) keep
  passing here; *Deliver value-only submix-strip fader, mute and pan edits to the running C ABI plan* (#1390) inverts them. The structural "route gain" case
  (`crates/host-core/tests/live_delta.rs:497`) edits a route into the output and stays.
- **Carry.** *Carry strip delay lines and live send ramps across a plan swap* (#1284) carries a
  live send's ramp, position and mute by route ID (its D2). A carried send keeps the predecessor's
  target. *Carry fader, mute and pan ramps across a plan swap* (#1277 D5) retargets carried strips
  with records extracted from the classifier; nothing yet does it for sends.

## Decisions frozen for this slice

- **D1. Classification.** Besides what is live today, a delta is live when it changes only the
  `gain_db`, `mute` and `channel_matrix` of routes whose destination is a submix. Routes are paired
  by route ID, and the route set must be equal.

  Everything else stays structural:
  - every submix field (#1390 makes `fader` and `matrix_or_pan` live);
  - the route set;
  - a route's source, tap, destination or `follows_mute` (`follows_mute` becomes live in #1226);
  - **any** field of a route into the output (decision 14 rule 3, optimisation reason O9), in this
    slice. Decision 15, D15-9, narrows O9: a route whose tap precedes its strip's fader gets a live
    lane, into the output too, through *Give every route whose tap precedes its strip's fader a live
    lane on every plan* (#1391), which grows this D1 set; `post_fader` and `post_pan` routes into
    the output stay structural;
  - a mute change on a follow source (guard G2, which #1226 removes);
  - any delta of a model that has a VCA (guard G3, which #1247 removes).
- **D2. No submix strip record.** `LiveDelta::strips` keeps holding track records only. #1390
  adds submix records after the tracks.
- **D3. Route records come from the two models, never from a mirror.**
  - One function, `route_target(model, route) -> (coefficients, silenced)`, computes a route's
    target exactly as preparation does: `route_coefficients(gain_db, matrix, mute, follow_zeroed)`.
    `follow_zeroed` is the source strip's `effective_strip_faders()` mute when `follows_mute` is
    set, and `[false; 2]` otherwise.
  - The classifier emits one route record for each live route whose target bits or `silenced`
    differ between `current` and `next`. The records go in canonical route-ID order, in a new
    `LiveDelta::routes` list. `LiveDelta::is_empty` (#1313 D2) gains the `routes` term, so a
    send-only edit reports `live`.
  - A domain refusal is `LiveRebuild::Domain`.
  - A gain edit on a muted route changes no target bit, so it emits nothing. The later unmute
    record carries the new gain.
- **D4. Ramps (D15-1). One record per route, one governing row.** A route cell holds one target
  and one ramp (#1347 D1), so a transaction that changes several of a route's fields emits one
  record, whose length is one row's `LiveRamps::resolve(row, edit_ramp)` (*Carry an optional
  per-edit ramp length on live session edits*, #1394 D5, D6):
  - **The gate changes** (`silenced`, or any `follow_zeroed` lane): the send-mute row governs, with
    the route's `RouteMute` entry (for a follow record in #1226, its source strip's `Mute` entry).
    A gain or matrix change in the same record rides that ramp: the route fades in to, or out
    from, the new target, as the unmute record of D3 does.
  - **Otherwise:** the longest of the resolved lengths of the rows whose field changed, the
    send-gain row with the `RouteGain` entry and the send-matrix row with the `RouteMatrix` entry.
    The longest wins so that no changed field moves faster than its own row asks.
  - With no edit ramp, a row resolves to its session length from `LiveRamps::for_session(next)`,
    the send lengths that *Session `controlSmoothing`: configurable ramp lengths for live mute,
    fader and pan changes* (#1054) defines. An explicit 0 is a step (rule R9), and a step occurs
    only when the governing row resolves to 0.
- **D5. Route lanes on the C ABI.** `C_ABI_LIVE_LANES` sets `routes: true`. The C ABI epoch keeps
  the `route_controls` handles beside its strip and effect producers, addressed by route ID. The
  lanes are the latest-target cells of *Hold live values in latest-target cells on both hosts*
  (#1312) and *Hold route-lane values in latest-target cells* (#1347). This slice adds no ring.
- **D6. Commit order (D15-2).**
  1. Classify the delta.
  2. Build every strip and route record, which runs each domain and length check.
  3. Run the live admission (`validate_live_peak`, `compile.rs:443`).
  4. Resolve every producer by ID in the newest epoch (#1053 D7).
  5. Run the protocol's commit predicate.
  6. Write the cells.
  7. Commit.

  Every fallible step runs before the first cell write. A cell write cannot fail and never returns
  `BACKPRESSURE`. When a value is written twice before one drain, the later write supersedes the
  earlier one, and #1312's `live_values_superseded` counts it.
- **D7. Resources.** The route cells and their IDs are charged where the C ABI charges its strip
  producers today (`capi_resources`, `compile.rs:140`). The graph rows grow by
  `graph::route_control_resources`. The `resource_lifecycle` oracles change by exactly those rows.
- **D8. Carry, then retarget (D15-7; formerly #1284 D3).** A transaction that makes a structural
  edit and also changes a carried send's live value sounds like "live edit, then structural edit".
  - The per-route record derivation of D3 is one function, `route_records(current, next, ramps)`,
    which `classify_live_delta` calls; nothing is duplicated (the pattern of #1277 D5).
  - For every send that #1284 carries and whose live values differ between the base and the
    successor's model, the successor entry points that return `HostLiveControlHandles` write that
    route's record into the successor's route cell before they return. The write is infallible
    (#1347), so it adds no failure to preparation.
  - At the swap block the carry copies the predecessor's ramp, then the successor drains its own
    cell. The record applies on the swap block's first sample, exactly where a live edit written to
    the predecessor just before the swap would apply.
  - Copy mode writes nothing at preparation. The records are kept on the prepared successor and
    written at publication, as #1277 D5 does for strips.

## Deliverables

1. D1-D3 in the classifier, with `LiveDelta::routes`.
2. D4-D6 in the control plane's live commit path, and D5 in the C ABI lane selection.
3. D7.
4. D8 in host-core's successor preparation.
5. In `docs/C_ABI_V1_QUALIFICATION.md`, update the "Rebuild" bullet (`:280-284`) and add a
   sentence after the paragraph at `:78-81`: sends into submixes are value-only. What still
   rebuilds: a submix strip value (until #1390), a route into the output (until #1391 for a tap
   before the fader; always for `post_fader` and `post_pan`),
   `follows_mute` and a follow-source mute (until #1226), and any session with a VCA (until #1247).

## Authorized paths

- `crates/host-core/src/live_delta.rs` (the classifier).
- `crates/control-plane/src/control.rs`: `commit_live` and its helpers (moved there by #1309).
- `crates/control-plane/src/compile.rs`: `capi_resources` (moved there by #1309).
- `C_ABI_LIVE_LANES` (D5), where it lives when this slice merges (*Prepare through an
  adapter-supplied preparer in the control plane crate*, #1400 D3): in
  `crates/control-plane/src/compile.rs` if this slice merges before #1400 (which then moves the
  edited constant to capi unchanged); beside `CapiPreparer` in `crates/capi/src/` if it merges
  after. Root rebases whichever lands second; neither order changes a value.
- `crates/host-core/src/prepare.rs`: route-lane attachment and D8's successor retarget only.
- These test files:
  - `crates/host-core/tests/live_delta.rs`;
  - `crates/capi/src/runtime/live_tests.rs`;
  - `crates/capi/tests/resource_lifecycle.rs`;
  - `crates/host-core/tests/successor_swap.rs` and `crates/host-core/tests/support/successor.rs`
    (gate 7).
- `docs/C_ABI_V1_QUALIFICATION.md`, and this spec.

## Non-goals

- No submix strip value (#1390). No `follows_mute` change and no
  follow-source mute (#1226). No VCA (#1247).
- No live route into the output (decision 14 O9). Routes into the output whose tap precedes the
  fader become live in *Give every route whose tap precedes its strip's fader a live lane on every
  plan* (#1391); `post_fader` and `post_pan` routes into the output stay structural (O9 as narrowed
  by decision 15, D15-9).
- No submix input section and no submix effect (#1267).
- No browser change. No new C symbol, opcode or field, and no struct layout change.

## Hazards

- **A live route never folds.** Turning on route lanes removes the fold for routes into submixes
  on every C ABI plan. If a committed digest moves, list it with this reason. Never re-pin digests
  in bulk.
- **Delayed sends.** A send with a compensation delay keeps running while muted (`DESIGN.md` P4).
  Its settled comparison point therefore adds that delay.
- **Carry.** #1284 owns copying the route lanes' ramp state across a plan swap. D8 only writes
  the retarget record into the successor's cell; it does not change the copy.
- **Addressing.** Strips are found by ID through `strips`, and routes by ID through
  `route_controls`. Never index into the model's vectors.

## Objective gates

1. **PCM through the C ABI** (new tests beside today's live PCM tests).
   - Fixture: `submix_session`, with a bus `b`, a send into `b`, and the bus's route into the
     output. Each lane carries a distinct, non-constant signal, and the send matrix is asymmetric.
   - Through `SESSION_TRANSACTION_APPLY`, apply `0x0505`, `0x0506` and `0x0504` on the send.
   - Each edit keeps the same plan: no new epoch, no pending candidate, no re-seek and no silent
     block.
   - The comparison point is `latency_samples`, plus the record's ramp, plus the send's
     compensation delay. From there the output is bit-identical to a plan compiled from the
     committed model and fed the same sources from sample 0.
   - Run with 1 and 10 tracks at 44.1, 48, 88.2 and 96 kHz. Run once more with a 486-sample
     compensation delay: a true-peak limiter insert on a sibling contributor, at 48 kHz.
2. **The boundary of liveness.** Each of these still rebuilds (a new epoch), and the plan then
   renders the committed model:
   - `0x0505` on a `post_fader` or `post_pan` route into the output (#1391 makes earlier taps
     live);
   - `0x0507` on a send;
   - `0x020f` muting a track that a `follows_mute` send reads;
   - `0x020f` on the bus `b` (a submix strip value, until #1390).
3. **No ack before a drop.**
   - A transaction whose second route value fails the domain check writes no cell. The model, the
     revision and the replay cache are unchanged.
   - Twenty consecutive send edits with no render between them all return `OK`. The next render
     converges to the last value, and `live_values_superseded` rises by exactly 19 for that route.
   - An exact replay writes nothing.
   - A live edit made while a candidate is pending lands in the candidate.
   - **Path of a send-only edit.** Each of the three send edits of gate 1 runs alone in its
     transaction on its own fresh boot of gate 1's fixture, whose send is unmuted (so `0x0506`
     mutes it, and `0x0504` and `0x0505` change an audible send; D1 emits a record for each). Each
     returns a response whose decoded path is `live` (#1313), never `model_only`. A `0x0505` on a
     send muted in the base emits no record (gate 4) and returns `model_only`.
4. **Classifier unit tests.**
   - A gain edit on a muted route emits no route record. Its unmute emits one record, which carries
     the new gain.
   - Governing row: one transaction that unmutes a route and changes its gain, with edit ramps
     `RouteMute` 480 and `RouteGain` 960, emits one record of length 480. One that changes gain
     (ramp 960) and matrix (ramp 240) emits one record of length 960; with no edit ramps it takes
     the longer of the session's send-gain and send-matrix lengths.
   - A matrix edit that lowers to the same gated target bits emits nothing.
5. **Realtime.** Extend
   `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`
   (`crates/capi/tests/resource_lifecycle.rs:2900`) with send edits, over 20 runs:
   - `allocations == 0` and `frees == 0` around every render call after warm-up;
   - zero `INTERNAL` results;
   - a final block bit-identical to a fresh plan of the final committed model.
6. **Unchanged behaviour and policy.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`: zero allocations, locks and syscalls.
   - `bash scripts/check-capi-abi.sh` and `bash scripts/check-capi-abi.sh --self-test`.
   - The workspace test command of `.github/workflows/qualification.yml` (the "Workspace debug
     tests" step), with `control-plane/test-support` added by #1309.
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `for x in host-core realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/check-cross-targets.sh`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job.
7. **Carry, then retarget** (in `crates/host-core/tests/successor_swap.rs`, on gate 1 of #1284's
   session: a live send from a track into a submix). B is A plus a muted track whose ID sorts
   first, and B also changes the carried send's gain.
   - Run 1 is the structural swap with D8's retarget.
   - Run 2 writes the same record to A just before the swap and prepares B from a base that holds
     it.
   - Every block of the two runs is bit-identical, in move mode and in copy mode (copy after
     block 6, then the adoption).
   - `cargo test --locked -p graph -p host-core --features graph/test-support,host-core/test-support`

## Test value

- Gate 1 turns red if a send edit is still structural, if a live route target differs from
  the prepared one, or if a record lands on the wrong strip or route.
- Gate 2 turns red if the classifier treats an output route, `follows_mute`, a follow-source mute
  or a submix strip value as live before its slice. The plan would then render something other than the committed model.
- Gate 3 turns red if a cell is written before a later check fails, if a superseded write is lost
  without being counted, or if a route cell back-pressures.
- Gate 3's path case turns red if `LiveDelta::is_empty` ignores `routes`, so a send-only edit is
  reported `model_only`.
- Gate 4 turns red if the route diff compares raw fields instead of gated targets. A redundant
  record restarts a settled ramp and moves a zero's sign (`crates/host-core/src/solo.rs:58-70`).
  Its governing-row case turns red if a record takes the wrong row's length (a gate change on the
  gain ramp, or the shorter of two changed rows).
- Gate 5 turns red if the route drain or the live commit allocates on the render thread, or if a
  send edit that races a swap reaches the retiring plan.
- Gate 7 turns red if a carried send keeps the predecessor's level after a transaction that also
  changed it (a missing retarget), or if the retarget applies a block late or early.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309)
- *Hold live values in latest-target cells on both hosts* (#1312)
- *Hold route-lane values in latest-target cells* (#1347)
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054), for the send lengths in D4
- *Carry an optional per-edit ramp length on live session edits* (#1394), for the per-edit ramp in
  D4
- *Carry strip delay lines and live send ramps across a plan swap* (#1284), for D8
- *Carry fader, mute and pan ramps across a plan swap* (#1277), whose D5 is D8's pattern
- *Report each transaction's edit path in its response* (#1313), for `LiveDelta::is_empty` and
  gate 3's path case
