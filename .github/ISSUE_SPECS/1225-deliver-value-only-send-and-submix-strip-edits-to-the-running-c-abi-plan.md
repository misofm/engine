# Deliver value-only send and submix-strip edits to the running C ABI plan

Stream F of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-1, D15-2, D15-6).
Code anchors verified on `main` at `6fb211594`.

Slice 27 of *Submix strips and live aux sends* (#1196). Rewritten 2026-10-05 for decision 15. The
live commit path moves into `crates/control-plane` (#1309), live values become latest-target
cells (#1312, route lanes #1347), and every live value ramps (#1054). Decision 15 supersedes the old batch-C1 order, the
"a step until #1054" rule and the per-queue `BACKPRESSURE` gates (D15-1, D15-2, D15-11). The
design record cited as `DESIGN` is `docs/handoffs/submix-sends-2026-10-02/DESIGN.md`.

## Product outcome

A C ABI host, such as a fan's phone, can change any of these on the running plan:

- a send's `gain_db`, `mute` or `channel_matrix` (a route whose destination is a submix);
- a bus's (submix strip's) fader dB, lane mute, or pan/matrix.

None of these changes rebuilds the plan, leaves a silent block, resets a source ring or steps
hard. Track faders, mutes and pans are already live (#1257). After this slice a bus or send edit
lands on the same plan, ramped, and is never lost.

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
- **Superseded tests.** These tests assert today's G1 behaviour:
  - `a_submix_fader_change_is_structural` (`crates/host-core/tests/live_delta.rs:402`);
  - the G1 case ("a submix fader") of
    `deltas_outside_the_live_set_rebuild_and_a_domain_failure_pushes_nothing`
    (`crates/capi/src/runtime/live_tests.rs:1015`; its fixture `submix_session` is at `:969`).

## Decisions frozen for this slice

- **D1. Classification.** Besides what is live today, a delta is live when it changes only these:
  - a **submix** strip's `fader` (dB and mutes) and `matrix_or_pan`. Submixes are paired by ID,
    exactly as tracks are, and the submix set must be equal, pairwise in normalized order;
  - the `gain_db`, `mute` and `channel_matrix` of a route whose destination is a submix. Routes are
    paired by route ID, and the route set must be equal.

  Everything else stays structural:
  - the route set;
  - a route's source, tap, destination or `follows_mute` (`follows_mute` becomes live in #1226);
  - **any** field of a route into the output (decision 14 rule 3, optimisation reason O9);
  - every other submix field;
  - a mute change on a follow source (guard G2, which #1226 removes);
  - any delta of a model that has a VCA (guard G3, which #1247 removes).
- **D2. Strip records for submixes.** A submix uses exactly the track rules of today's classifier:
  - one `FaderDb` per changed lane, and one `Mute` per changed lane;
  - one matrix record when a lowered bit changes;
  - domain checks through `checked_fader_gain` and `lower_matrix_or_pan`.

  `LiveDelta::strips` then holds the tracks first and then the submixes, each in canonical ID order,
  which matches `HostLiveControlHandles::strips`.
- **D3. Route records come from the two models, never from a mirror.**
  - One function, `route_target(model, route) -> (coefficients, silenced)`, computes a route's
    target exactly as preparation does: `route_coefficients(gain_db, matrix, mute, follow_zeroed)`.
    `follow_zeroed` is the source strip's `effective_strip_faders()` mute when `follows_mute` is
    set, and `[false; 2]` otherwise.
  - The classifier emits one route record for each live route whose target bits or `silenced`
    differ between `current` and `next`. The records go in canonical route-ID order, in a new
    `LiveDelta::routes` list.
  - A domain refusal is `LiveRebuild::Domain`.
  - A gain edit on a muted route changes no target bit, so it emits nothing. The later unmute
    record carries the new gain.
- **D4. Ramps (D15-1).**
  - Strip records take their lengths as track records do.
  - A route record whose gate changes (`silenced`, or any `follow_zeroed` lane) uses the session's
    mute length. Any other route record uses the send-level length.
  - Both lengths come from `LiveRamps::for_session(next)`, with the send lengths that *Session
    `controlSmoothing`* (#1054) defines. A live route record is a step only when the session sets
    a length of 0.
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

## Deliverables

1. D1-D3 in the classifier, with `LiveDelta::routes`.
2. D4-D6 in the control plane's live commit path, and D5 in the C ABI lane selection.
3. D7.
4. In `docs/C_ABI_V1_QUALIFICATION.md`, replace the paragraph at `:78-81` and the "Rebuild" bullet
   (`:280-284`). The new text says that submix strip values and sends into submixes are
   value-only. It also says what still rebuilds: a route into the output, `follows_mute` (until
   #1226), a follow-source mute (until #1226) and any session with a VCA (until #1247).
5. The superseded tests in Context, inverted in the same PR: a submix fader edit is live.

## Authorized paths

- `crates/host-core/src/live_delta.rs` (the classifier).
- `crates/control-plane/src/control.rs`: `commit_live` and its helpers (moved there by #1309).
- `crates/control-plane/src/compile.rs`: `C_ABI_LIVE_LANES` and `capi_resources` (moved there by
  #1309).
- `crates/host-core/src/prepare.rs`: route-lane attachment only.
- These test files:
  - `crates/host-core/tests/live_delta.rs`;
  - `crates/capi/src/runtime/live_tests.rs`;
  - `crates/capi/tests/resource_lifecycle.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`, and this spec.

## Non-goals

- No `follows_mute` change and no follow-source mute (#1226). No VCA (#1247).
- No live route into the output (decision 14 O9).
- No submix input section and no submix effect (#1267).
- No browser change. No new C symbol, opcode or field, and no struct layout change.

## Hazards

- **A live route never folds.** Turning on route lanes removes the fold for routes into submixes
  on every C ABI plan. If a committed digest moves, list it with this reason. Never re-pin digests
  in bulk.
- **Delayed sends.** A send with a compensation delay keeps running while muted (`DESIGN.md` P4).
  Its settled comparison point therefore adds that delay.
- **Carry.** Stream A's carry program owns the route lanes' ramp state across a plan swap. This
  slice does not change it.
- **Addressing.** Strips are found by ID through `strips`, and routes by ID through
  `route_controls`. Never index into the model's vectors.

## Objective gates

1. **PCM through the C ABI** (new tests beside today's live PCM tests).
   - Fixture: `submix_session`, with a bus `b`, a send into `b`, and the bus's route into the
     output. Each lane carries a distinct, non-constant signal, and the send matrix is asymmetric.
   - Through `SESSION_TRANSACTION_APPLY`, apply `0x0505`, `0x0506` and `0x0504` on the send, and
     `0x020f` and `0x0210` on `b`.
   - Each edit keeps the same plan: no new epoch, no pending candidate, no re-seek and no silent
     block.
   - The comparison point is `latency_samples`, plus the record's ramp, plus the send's
     compensation delay. From there the output is bit-identical to a plan compiled from the
     committed model and fed the same sources from sample 0.
   - Run with 1 and 10 tracks at 44.1, 48, 88.2 and 96 kHz. Run once more with a 486-sample
     compensation delay: a true-peak limiter insert on a sibling contributor, at 48 kHz.
2. **The boundary of liveness.** Each of these still rebuilds (a new epoch), and the plan then
   renders the committed model:
   - `0x0505` on a route into the output;
   - `0x0507` on a send;
   - `0x020f` muting a track that a `follows_mute` send reads.
3. **No ack before a drop.**
   - A transaction whose second route value fails the domain check writes no cell. The model, the
     revision and the replay cache are unchanged.
   - Twenty consecutive send edits with no render between them all return `OK`. The next render
     converges to the last value, and `live_values_superseded` rises by exactly 19 for that route.
   - An exact replay writes nothing.
   - A live edit made while a candidate is pending lands in the candidate.
4. **Classifier unit tests.**
   - A gain edit on a muted route emits no route record. Its unmute emits one record, which carries
     the new gain.
   - A submix fader edit emits one strip record, addressed by the submix ID.
5. **Realtime.** Extend
   `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`
   (`crates/capi/tests/resource_lifecycle.rs:2900`) with send and bus edits, over 20 runs:
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

## Test value

- Gate 1 turns red if a send or bus edit is still structural, if a live route target differs from
  the prepared one, or if a record lands on the wrong strip or route.
- Gate 2 turns red if the classifier treats an output route, `follows_mute` or a follow-source mute
  as live before its slice. The plan would then render something other than the committed model.
- Gate 3 turns red if a cell is written before a later check fails, if a superseded write is lost
  without being counted, or if a route cell back-pressures.
- Gate 4 turns red if the route diff compares raw fields instead of gated targets. A redundant
  record restarts a settled ramp and moves a zero's sign (`crates/host-core/src/solo.rs:58-70`).
- Gate 5 turns red if the route drain or the live commit allocates on the render thread, or if a
  send edit that races a swap reaches the retiring plan.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309)
- *Hold live values in latest-target cells on both hosts* (#1312)
- *Hold route-lane values in latest-target cells* (#1347)
- *Session `controlSmoothing`: configurable ramp lengths for live mute, fader and pan changes*
  (#1054), for the send lengths in D4
