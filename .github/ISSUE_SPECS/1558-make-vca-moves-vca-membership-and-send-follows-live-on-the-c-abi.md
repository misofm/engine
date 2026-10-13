# Make VCA moves, VCA membership and send follows live on the C ABI

Slice S7a of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C6
step 3, C7 and C11). Code anchors verified on `main` `df585e503`. Feature sprint mode (owner,
2026-10-13): one implementer, normal tests, a batch review; no per-attempt verifier or mutation
runs.

## Product outcome

A C ABI host (an iOS or Android app) rides a VCA, mutes it, adds or removes a VCA, changes a VCA's
members, mutes a strip that a send follows, or turns a send's `follows_mute` on or off, and the
running plan takes the change with no rebuild. Every one of these changes reaches render only
through the effective values of `controls::effective` (#1547), which preparation also uses, so the
live path and a fresh plan cannot compose them differently. This is the C ABI half of what the
browser already has; the browser joins the same code in #1562.

## Context

- **The two guards.** `classify_live_delta` refuses every delta while either model has a VCA:
  `LiveRebuild::Vca` (`crates/host-core/src/live_delta.rs:98`), raised at `:221-223`. It refuses a
  lane-mute change on a strip that a `follows_mute` route reads: `LiveRebuild::FollowedMute`
  (`:113`), raised at `:287-289` through `followed_mute_changes`
  (`crates/host-core/src/prepare.rs:2609`). #1554 rewrites the classifier to ruling C6 steps 1, 2
  and 4; it keeps these two refusals for the VCA rows and the follow inputs, because both are
  `RebuildOnly(RuledLiveNotBuilt { issue: EFFECTIVE_SLICE })` rows in #1546 (D5).
- **The successor join has the same guard.** `join_strips` (`prepare.rs:2791`) sets
  `vca_in_play` (`:2797`) and makes every strip family prepared, not carried, while a VCA is in
  either model (`:2815`). It also asks `followed_mute_changes`.
- **One effective function.** #1547 adds `controls::effective(&SessionModel, &MonitorOverlay) ->
  Effective` and its allocation-free form `controls::effective_into`. `Effective.strips[s]` holds
  `fader_db`, `mute` and `vca_mute` per lane; `Effective.routes[r].gate` holds the follow gate per
  source lane. Builtins preparation and the graph compiler receive it as an argument (#1547 D4).
  `SessionModel::effective_strip_faders` is deleted.
- **What render needs.** A VCA has no render cell. A VCA move reaches render as the members'
  `FaderDb` and `Mute` records on their strip cells (family strip level). A follow change reaches
  render as the route's folded coefficients (family route, `RouteControlProducer`,
  `crates/host-core/src/route_controls.rs:159`), whose gate zeroes the column of each muted source
  lane (`crates/graph-compiler/src/compile.rs:312-358`). #1557 lowers route gain, mute and matrix
  into that record.
- **Count caps.** Preparation refuses `vca_count > caps.maximum_vcas` with `host.resource.count`
  (`prepare.rs:1418-1432`). The live path never runs that check today, because no live delta adds
  a VCA.
- **The model holds a VCA per lane.** `Vca.fader` is a `DualMonoFader`
  (`crates/session/src/model.rs:835-842`): a left and a right offset and a left and a right mute.
  The browser addresses VCA lanes (`COMMAND_VCA_FADER_DB`, `COMMAND_VCA_MUTE`: channel `0`, `1` or
  `2`, `hosts/host-web/src/lib.rs:927-949`). The coordinator's ruling of 2026-10-13 makes the
  two VCA rows per-lane (#1546 D5 as updated).
- **The C ABI keeps no VCA mirror.** The module comment of `crates/host-core/src/vca.rs:59-61`
  says the C ABI "keeps no live VCA state until #1247". This slice keeps it so: the classifier
  diffs committed models.
- **Decision 15 already rules all of this live** (D15-6, D15-13 E3; decision 14 F3 and F9). The
  specs of *Let C ABI sends follow their source strip's mute live* (#1226) and *Deliver value-only
  VCA edits to the running C ABI plan* (#1247) hold the hazards and the measurement rules that
  this slice keeps; it replaces their per-kind code with the effective function.
- **Tests that assert today's guards:** `a_followed_mute_change_needs_a_rebuild`
  (`crates/host-core/tests/live_delta.rs:415`), `any_vca_needs_a_rebuild` (`:463`), and the VCA
  and followed-mute cases of `deltas_outside_the_live_set_rebuild_and_a_domain_failure_writes_nothing`
  (`crates/capi/src/runtime/live_tests.rs:1639`). (The specs of #1226 and #1247 name this test
  by an older name, `..._pushes_nothing`, at `:1015`.)

## Decisions frozen for this slice

- **D1. Classifier step 3 (ruling C6).**
  - Step 1's mask also covers every input of the effective function: the whole `vcas` list (the
    set, the IDs, the offsets and mutes, the members) and every route's `follows_mute`. A delta
    that changes only these and control values is not a plan rebuild.
  - Step 3: the classifier computes `controls::effective(current, overlay)` and
    `controls::effective(next, overlay)` (the overlay is `MonitorOverlay::default()` until #1559).
    Each strip lane whose effective `fader_db` bits or effective `mute` changed gives a strip-level
    record; each route whose effective `gate` changed, or whose own gain or matrix changed, gives
    one route record.
  - **Records come from effective values only.** The strip-level family lowers `FaderDb` with the
    effective dB and `Mute` with the effective mute. It never reads a raw `left_db`, `right_db`,
    `left_mute` or `right_mute`. Step 2 therefore lowers no strip-level record directly: an own
    `fader_db` or `mute` edit reaches render through step 3. With no VCA and no solo,
    `vca_effective_db` returns the own value bit for bit, so a VCA-free session emits exactly what
    it emits before this slice.
  - The route family takes its gate from `Effective.routes[r].gate`. If #1557 already does, this
    changes nothing there.
  - The domain check of a strip-level record is `builtins::checked_fader_gain` on the effective
    dB, the value the plan bakes.
- **D2. VCA rows are per-lane** (coordinator ruling, 2026-10-13). The two VCA own rows,
  `offset_db` (1) and `mute` (2), are `LaneShape::PerLane`: listed `left` and `right`; `both` sets
  both lanes. Each lane's offset sums into the same lane of every member, as the model and the
  browser already do. If #1546 shipped them shared, this slice changes them.
- **D3. Rows flip to live.** VCA `offset_db` and `mute` and route `follows_mute` (on a route into a
  submix) become `ControlClass::Live`. Route `follows_mute` on a route into the output stays
  `active: false`. The constant `EFFECTIVE_SLICE` is deleted, because no row names it.
- **D4. The guards go.** Delete `LiveRebuild::Vca`, `LiveRebuild::FollowedMute` and
  `followed_mute_changes`. In `join_strips`, delete `vca_in_play`: both models' strip values come
  from `controls::effective`, and a strip family carries and retargets by the same effective diff
  as the classifier. A strip whose effective values do not change is `Same`.
- **D5. Membership and the VCA set are live, and change the row set.** `UpsertVca` (`0x0700`),
  `RemoveVca` (`0x0701`) and a member change are live when nothing else structural changes. Adding
  or removing a VCA adds or removes its rows, so the commit advances `structure_epoch` (ruling C4)
  and rebuilds the control catalog of #1552 in the live commit, on the control thread. This
  amends #1552 D2's sentence "a live commit never changes the address sequence": from this slice
  on, a live VCA add or remove does, and the epoch rule of #1552 D2 (advance when the address
  sequence differs) applies to it unchanged. A member change, an offset or a mute changes no row
  and keeps `structure_epoch`.
- **D6. Count caps on the live path.** Move the count check of `prepare.rs:1418-1432` into one
  function, `host_core::check_counts(model: &SessionModel, caps: &HostPrepareCaps) -> Result<(),
  PrepareDiagnostics>`, that preparation and the control
  plane's `live_admission` (`crates/control-plane/src/control.rs:1636`) both call. A live delta
  whose `next` exceeds a cap is refused with `host.resource.count` and changes nothing.
- **D7. Ramps (D15-1, ruling C6 step 4).** Every record takes `LiveRamps::resolve(row, edit_ramp)`
  of `next`. The row and the edit ramp are, for each changed lane:
  - effective `fader_db` of strip `s`: the strip's own `fader_db` edit on that lane (row `Fader`)
    if the transaction has one; else the first edited VCA in `s`'s reach, in ascending VCA-ID
    order (row `VcaOffset`, that edit's ramp); else row `VcaOffset` with no edit ramp (a membership
    change or a VCA add or remove);
  - effective `mute` of strip `s`: the own `mute` edit (row `Mute`); else the first edited reaching
    VCA's `mute` edit (row `VcaMute`); else row `VcaMute` with no edit ramp;
  - a route record whose only change is its gate: the route's own `mute` or `follows_mute` edit
    (rows `SendMute`, `SendFollowsMute`) if the transaction has one; else the length that the
    source strip's `Mute` record of this transaction took (#1226 D5), so a strip and its following
    sends ramp together (this refines #1557 D3, which gives every gate change the `SendMute` row: a
    gate change that no route edit caused now takes its source's mute length); a route record
    whose gain or matrix also changed takes #1557 D3's rule.
- **D8. One commit (the acked-batch question).** Strip records and route records of one
  transaction are built and checked together, before the first cell write, in #1554's commit
  order. A refusal writes no cell and changes no model, revision or catalog. Every value is a
  latest-target cell, so nothing is refused for room, and no ack can precede a drop.
- **D9. No mirror on the C ABI.** The control plane keeps no `LiveVcaState` and no route mirror
  for this slice. Replace the clause at `crates/host-core/src/vca.rs:59-61` with: the C ABI
  composes VCAs and follows by diffing committed models through `controls::effective`.
- **D10. Cost.** Each classification computes the effective values twice:
  `O(strips x reach + routes)` on the control thread, bounded by `maximum_tracks`,
  `maximum_submixes` and `maximum_vcas`. Nothing runs on render.

## Deliverables

1. D1 and D4 in the classifier (`crates/host-core/src/live_delta.rs`) and `join_strips`.
2. D2 and D3 in the `controls` row tables.
3. D5 and D6 in `crates/control-plane/src/control.rs` (`live_admission`, the live commit's catalog
   step) and `prepare.rs` (the shared count function).
4. D9's comment.
5. `docs/C_ABI_V1_QUALIFICATION.md`: VCA offset, mute, membership, VCA add and remove, a mute
   that sends follow, and `follows_mute` are value-only; a live edit over `maximum_vcas` is
   refused like a compile.
6. The superseded tests in Context, inverted or deleted in the same PR.

## Authorized paths

- `crates/host-core/src/{live_delta.rs,prepare.rs,vca.rs,lib.rs}` (`vca.rs`: the module comment
  only).
- `crates/controls/**`.
- `crates/control-plane/src/{control.rs,compile.rs}`.
- `crates/host-core/tests/live_delta.rs`, `crates/host-core/tests/vca_caps.rs`,
  `crates/capi/src/runtime/live_tests.rs`, `crates/capi/tests/resource_lifecycle.rs`.
- `docs/C_ABI_V1_QUALIFICATION.md`.

## Non-goals

- No browser change. The browser's own VCA, follow and solo composition stays until #1565; #1548
  already makes it call the same building blocks.
- No solo on the C ABI (#1559).
- No change to `vca_effective_db`, `vca_reach`, the clamp, or the VCA grammar.
- No stored automation (#1566).
- No new C symbol and no struct layout change.

## Objective gates

Commands: `cargo test --locked -p controls -p host-core --all-features`,
`cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
`cargo build --locked --release -p audit -p capi && target/release/audit capi`.

1. **PCM through the C ABI.** Fixture: tracks `t1`, `t2`; submix `b` routed to the output;
   `t1` sends `pre_fader` into `b` with `follows_mute: true` and an asymmetric send matrix; VCA
   `outer` holds VCA `inner`, `inner` holds `t1`. Each source feeds a distinct, non-constant signal
   on each lane. Through `SESSION_TRANSACTION_APPLY` with `SET_CONTROL` (or the named structural
   op), in order: `inner` `offset_db` both lanes; `outer` `mute` left lane; `t1` own `fader_db`;
   `t1` own `mute` left, then both, then unmute; `follows_mute` off while `t1` is muted, then on;
   `UpsertVca` adding `t2` to `inner`; `UpsertVca` adding a new VCA over `b`; `RemoveVca` of
   `outer`. Each edit returns path `live`, advances the revision by 1 and keeps the plan epoch.
   From `latency_samples` plus the ramp plus the send's compensation delay, every block is
   bit-identical to a plan prepared from the committed model. Run with 1 and 10 extra tracks at
   44.1, 48, 88.2 and 96 kHz.
2. **Effective, not raw** (classifier unit test). `t1` is muted by a VCA and has a following send.
   Toggling `t1`'s own mute while the VCA mute holds yields no strip record and no route record.
3. **No redundant record** (classifier unit tests). Muting a track that no route follows yields no
   route record. Re-muting a muted source yields none. Toggling `follows_mute` on an unmuted source
   yields none. Muting only the right lane yields one route record whose right column only is
   zeroed. A VCA move that leaves every member clamped at +24 dB yields no record. An exact replay
   writes nothing.
4. **Count cap.** With `maximum_vcas` equal to the current count, an `UpsertVca` that adds a VCA
   returns `COMPILE_REJECTED` with `host.resource.count`; model, revision and cells are unchanged.
5. **Ramps.** A `SET_CONTROL` on `inner` `offset_db` with `ramp_samples` 480 moves `t1`'s fader over
   480 samples; with the field absent it moves over `fader_ms`; a transaction that sets `t1`'s own
   `fader_db` with ramp 0 and `inner`'s offset with ramp 480 steps `t1` (own edit first, D7). A
   follow-gate change from `t1`'s mute with ramp 96 ramps the send over 96 samples.
6. **Row set.** After the `UpsertVca` that adds a VCA, `CONTROL_LIST_GET` with the old
   `structure_epoch` is refused `control.list.stale`, and a fresh listing has the new VCA's four
   rows (two per-lane rows, D2) as `live` and automatable. After an offset edit, the old
   `structure_epoch` still pages.
7. **Realtime.** `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free`
   (`crates/capi/tests/resource_lifecycle.rs:3364`) also races VCA offset, mute and membership edits
   and `follows_mute` toggles over 20 runs: `allocations == 0 && frees == 0` around every render
   call after warm-up, zero `INTERNAL` results, and a final block bit-identical to a fresh plan of
   the final committed model.

## Test value

- Gate 1 turns red if a VCA, membership, follow or followed-mute edit still rebuilds, or if the
  live composition differs from preparation's (sum order, clamp, mute OR, follow column).
- Gate 2 turns red if records diff raw mutes instead of effective ones.
- Gate 3 turns red if the composition re-emits an unchanged target, which re-enters the ramp
  kernel and moves a settled lane's zero sign.
- Gate 4 turns red if a live edit commits a model that its own rebuild would refuse.
- Gate 5 turns red if a record takes the wrong row's length, drops a per-edit length, or ramps a
  follow independently of its source's mute.
- Gate 6 turns red if a live VCA add keeps stale handles or lists the VCA rows with a stale class.
- Gate 7 turns red if the effective path allocates on render or lands a racing edit in a retiring
  plan.

## Dependencies

- #1547 (`controls::effective`, `MonitorOverlay`).
- #1554 (the registry classifier and its families).
- #1557 (the route family's lowering).
- #1552 (the control catalog and `structure_epoch`) and #1555 (the core's live lanes, so every plan
  with a live route row has route lanes).
- *Hold route-lane values in latest-target cells* (#1347) and *Hold live values in latest-target
  cells on both hosts* (#1312), both on `main`.

## Supersedes and shrinks

- Supersedes *Let C ABI sends follow their source strip's mute live* (#1226).
- Supersedes *Deliver value-only VCA edits to the running C ABI plan* (#1247).
- Shrinks *Deliver value-only fader, mute and pan transactions to the running C ABI plan through
  the live console lanes* (#1053): guards G2 and G3 are lifted by this slice.

## Decision record (implementation)

- **One composition of the follow gate (batch-1 review carry-forward).** The graph compiler no
  longer spells `follows_mute ? strip_mutes[source] : [false; 2]`. `controls::effective` computes
  each route's follow part (`EffectiveRoute::follow`, from the new building block
  `effective_follow`) beside its gate (`effective_gate` is the route's mute OR that follow).
  Builtins preparation, which already receives the `Effective`, keeps `follow` in
  `PreparedBuiltinsSession::effective_route_follow` (charged in the builtin resource estimate), and
  `graph-compiler` reads it for `RouteGate::follow_zeroed`; `effective_strip_mutes` is deleted. The
  live classifier reads `Effective.routes[r].gate` of both models. The follow part is kept apart
  from the gate only so that the sealed graph text of a muted route (one `route-mute` row) does not
  move (#1216 D5).
- **`EffectiveDiff` (`crates/host-core/src/prepare.rs`) is the one place a strip's level records and
  a gate record's length are decided.** The classifier and `join_strips` both call it, so a
  successor's carry retarget and a live record cannot differ. `fader_records` takes the effective
  values and per-lane lengths (`LevelLengths`); two lanes merge into one `Both` record only when
  their value and their length agree.
- **Mask and diff.** `mask_values` copies the whole `vcas` list. `changed_controls` compares the VCA
  rows apart: with equal VCA addresses their changes are listed (last), with a different VCA set it
  lists none and does not return `None`; the classifier turns any VCA row, strip-level row or route
  row change, or a different VCA list, into "compute the effective values of both models".
- **D6.** `host_core::check_counts` is the count check of preparation; `live_admission` calls it
  with `prepare_caps(limits)` and maps the diagnostic to `COMPILE_REJECTED`.
- **Successor path (known residual, #1284).** `join_strips` now carries and retargets a strip whose
  followed mute changes, as D4 says. The sends of the successor are prepared from its model, so they
  are gated at the swap block without the source's ramp. The live path ramps a send's gate with its
  source's mute (D7); the successor path cannot until a send's ramp is carried across a swap.
- **Catalog growth (batch 3 review fold, replaces the first reading).** A live VCA add (or a
  remove and add with a longer ID) replaces the catalog by a larger one (four rows and a header per
  VCA, and the ID in each). The first reading charged only the catalog the plans were prepared
  with, and said the growth was bounded by `maximum_vcas`; that bounds the row count, not the
  bytes, and no ceiling saw them. Now `live_admission` builds the committed model's catalog before
  admission and `validate_live_peak` takes a `LiveCatalogAdmission`: the new catalog's retained bytes
  less the current catalog's (saturating at zero) join the capi term, and the new catalog's largest
  single allocation joins the largest-allocation term. A refusal is the compile diagnostic a
  rebuild gives (`ResourceFault::Limit`), before any write. After the commit the newest epoch's
  retained rows (`CapiResources` and the report row `plan_resources` reads) follow the committed
  catalog, so the next admission and the report see its bytes; an older pending-swap epoch keeps the
  catalog it was prepared with until its plan is gone, and the largest-allocation figure only
  grows. The peak therefore holds the committed catalog twice (the retained row and the
  prepared-protocol stand-in), as the first reading's did for the old one.
- **A route record with a gain or matrix change and a source-caused gate change takes the
  source's `Mute` length (batch 3 review fold).** D7's last clause names #1557 D3's rule for such a
  record, which would give the `SendGain` or `SendMatrix` row's length. The code does the other
  thing: once the gate changes the gate's length governs the whole record (`route_records`), and a
  gate change that no route edit caused takes the source's `Mute` length. Decision: keep the code.
  A send that follows its source's mute must ramp with it; a second, independent length on the same
  record would make the cell ramp one way for its gate and another for its gain, and a record
  has one length. The gain moves with the gate's ramp in that block. One test pins it
  (`a_gain_change_with_a_source_caused_gate_change_takes_the_sources_mute_length`).
- **Tests.** Inverted: `a_followed_mute_change_needs_a_rebuild`, `any_vca_needs_a_rebuild`,
  `a_followed_submix_mute_change_needs_a_rebuild` (host-core classifier);
  `deltas_outside_the_live_set_rebuild_...` (capi); the four followed-mute / VCA restart tests of
  `successor_swap.rs`, which used a VCA or a followed mute as their "prepared difference" (no valid
  session has a prepared fader family left, so the restart-count test became a carried-lane count
  test; the restarting-input-lane test remains).
