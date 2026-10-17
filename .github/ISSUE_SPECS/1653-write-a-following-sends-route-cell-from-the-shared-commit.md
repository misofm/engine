# Write a following send's route cell from the shared commit

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A6 and A10, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R2. **It lands in the same push as #1651
*Render stored mute automation on the strip* and #1652 *Follow an automated mute on following sends
in render*.**

## Product outcome

On a send that follows a strip whose mute the session automates, every live edit that changes the
send's gate reaches it, on both hosts, through the shared commit: a route gain, matrix or mute edit,
a live `follows_mute` toggle, and a change of the source strip's static mute, VCA mute or solo. Each
writes the route's seven-word cell (#1652 D1) and gives no folded follow record, so render
composes the gate with the curve at the next event, and nothing a live edit writes can contradict
the curve. Solo and VCA mutes compose with the curve for the send as for the strip.

## Context

- **The C ABI's route records.** *Deliver value-only send edits to the running C ABI plan* (#1225)
  D3 emits route records from `route_target(model, route)`; *Let C ABI sends follow their source
  strip's mute live* (#1226) D2-D3 makes `follows_mute` live and builds follow records from each
  strip's effective mute; #1226 D7 adds `LiveRouteState::set_follows_mute`. #1226 D6 leaves the
  follow of an automated mute to #1058.
- **The browser.** *Make a send's follows_mute live in the browser* (#1342) D1-D2: the browser
  receives the edit only as a transaction through the Worker's committed model, and its follow
  records come from the shared commit. *Admit browser live edits in the Worker through the committed
  model* (#1382) D3 composes the solo overlay and the route mirror in the shared commit.
- **The mirror.** `LiveRouteMuteFollow::delta` (`crates/host-core/src/live_route_state.rs:232-257`)
  yields only changed routes; the mirror holds `follows_mute` and `source_lane_muted` (`:46-53`).
- **The cell.** #1652 D1: for a route into a submix whose source strip has an automated mute
  lane, the cell holds the open transform, a flags word (route `mute`, `follows_mute`, `terms[2]`,
  `automated[2]`) and the ramp.

## Decisions frozen for this slice

- **D1. Route edits write the cell.** For a route of #1652 D1, the shared classifier emits one
  route-cell write instead of the route records that #1225 and #1226 emit for that route (#1226 adds no record type
  of its own, #1226 D3). It carries the
  open transform from `route_values` (`crates/graph-compiler/src/ids.rs:309-322`), the route
  `mute`, `follows_mute` and `terms`, and the ramp from #1226 D5's rule. It writes only when a word's
  bits change. The path is `live`.
- **D2. Terms.** `terms[lane]` is, for an automated source lane, `vca_mute || solo_mute`; for a lane
  that is not automated, its whole effective mute (#1247 D2's effective mute, with #1382 D3's solo
  term). A change of any of them writes the cell. On the C ABI the overlay is absent, so the solo
  term is false; the code is the same on both hosts.
- **D3. `follows_mute`.** A toggle writes the cell and, through #1226 D7's setter, the mirror, in the
  same shadow as #1382 D3 requires, so a later solo change composes with the new flag.
- **D4. No second path.** #1225, #1226, #1342 and #1347 are not amended: they land before this slice,
  and this slice adds the branch for routes whose source mute is automated to the code they built.
- **D5. The acked-batch question: can an ack ever precede a drop? No.** Every check of a
  transaction and of the overlay precedes the first cell write; writes cannot fail; render
  generates curve events from the plan.

## Deliverables

1. D1-D3 in the shared classifier and the commit path, with tests.

## Authorized paths

- `crates/host-core/src/{live_delta.rs,live_route_state.rs}`,
  `crates/host-core/tests/{live_delta.rs,live_routes.rs}`
- `crates/control-plane/src/` (the route-cell write and the overlay composition)
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/src/tests.rs`

## Non-goals

- The route lane, its layout and the route op (#1652). Send automation (OQ2).

## Hazards

- **One push with #1651 and #1652.** Root merges the three together.
- **A redundant write moves bits** (README "Change only"). D1 writes only changed words.

## Objective gates

1. **`follows_mute` toggle** (both hosts, new). On a playing engine with `t` automated-muted, a
   transaction (C ABI) and a browser edit turn `follows_mute` off, then on: each is `live`; after
   the route ramp the output equals a fresh plan of the committed model.
2. **Composition** (browser, new). Solo on another track mutes `t` by solo while its curve is 0; the
   send follows; unsolo restores the curve's state. A VCA mute of `t` composes the same way on both
   hosts. A route into the output never follows.
3. **Edits** (`crates/host-core/tests/live_delta.rs`, new). A `follows_mute` toggle on a #1652
   route gives one route-cell write and no follow record; a static mute change on the source's
   non-automated lane gives that lane's `Mute` record and one route-cell write; a route gain edit
   gives one route-cell write with the new transform; an edit that leaves every word unchanged gives
   nothing.
4. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support`,
     `cargo test --locked -p control-plane --features test-support`, `cargo test --locked -p capi`,
     `cargo test --locked -p host-web --features host-web/test-support`
   - the workspace debug leg (`test-debug-a`) in `.github/workflows/qualification.yml`
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
5. **No rendered bit moves** for a session with no stored automation: `cargo build --locked
   --release -p audit -p capi && ./target/release/audit capi` gives the same `pcm_digest` at base
   and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1: red if `follows_mute` is still read from the prepared route only, so a live toggle is lost
  on an automated route.
- Gate 2: red if solo or a VCA mute overwrites the curve instead of composing with it for the send.
- Gate 3: red if an edit still emits a folded record that the curve's next event would contradict,
  or writes an unchanged cell.

## Dependencies

Batch R2. Direct dependencies:

- #1652 *Follow an automated mute on following sends in render* (same push).
- *Make a send's follows_mute live in the browser* (#1342): the browser's `follows_mute` edit. It
  brings #1225, #1226 and #1382.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **The mirror is gone.** `LiveRouteMuteFollow` and `crates/host-core/src/live_route_state.rs` are
  deleted; the send gate is `controls::effective`'s (C7), and route records come from
  `route_records` (`crates/host-core/src/live_delta.rs:1416`) and `RouteControlProducer`
  (`crates/host-core/src/route_controls.rs:201`). D1-D3 apply to that code.
- **`terms`** for an automated source lane is `static_mute || VCA mute || solo term` (OQ1 = C);
  solo exists on both hosts (C8).
- **#1342** is superseded by #1562 (its gate 1). #1225, #1226, #1347 have landed.
- Authorized paths: `crates/host-core/src/{live_delta.rs,route_controls.rs}`,
  `crates/host-core/tests/live_delta.rs`, `crates/control-plane/src/`,
  `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`.

## Amendment (controls coordinator, batch 18 spec check, 2026-10-17)

Checked against `ad03780ee`. The filed text above stays; where it disagrees, this section wins.

**Anchors now.**
- The one record function of a send is `route_record_between`,
  `crates/host-core/src/prepare.rs:4154-4193`. The classifier (`route_records`,
  `crates/host-core/src/live_delta.rs:1467`) and the swap join (`join_routes`, `prepare.rs:5473`)
  both call it. So the branch is in `prepare.rs`, which this slice now needs.
- The classifier lowers a route only when its rows changed or its effective gate changed
  (`live_delta.rs:1019`). The gate is `controls::effective_gate`
  (`crates/controls/src/effective.rs:381`).
- `route_record` and `RouteControlProducer`: `crates/host-core/src/route_controls.rs:146`, `:201`.
- The commit resolves route producers at `crates/control-plane/src/control.rs:3226-3242` and
  writes them at `:3419-3421`, in the same stamp as the strip and jump-length writes.
- The in-place classifier (`classify_live_staged`, `live_delta.rs:565`) must equal the dense one
  (#1613; `crates/host-core/tests/in_place_commit.rs`, R17-1).

**How the slice maps onto the landed code.**
- **D1.** For a route that the next plan gives a seven-word cell (#1652 D1, from the `after`
  model), `route_record_between` returns a seven-word record: the open transform from
  `route_values` (#1652 makes it public), the flags word of #1652's amendment, and the ramp from
  the existing length rule. `LiveRouteRecord` carries either a folded record or a seven-word one.
  The join gets the same record with no second code.
- **Trigger (fix).** For a seven-word route, the classifier also lowers it when its source lane's
  effective mute (`terms`) changes and the gate does not (for example while `follows_mute` is
  off). So the cell always holds the committed words, and "writes only when a word's bits change"
  is exact.
- **D2.** After OQ1 = C, `terms[lane]` is `controls::effective`'s source-lane mute, with the
  overlay, for every lane (it holds no curve). Solo is on both hosts (C8).
- **D3.** The mirror and its setter are gone. `follows_mute` is an input of `controls::effective`
  and the committed model is the shadow. A toggle writes the cell only.
- **D5 (the acked-batch question), made exact.** Before the first write, the commit checks that
  each seven-word record goes to a seven-word producer, and that its `automated` bits equal the
  producer's plan. Any mismatch is `CommandError::Internal`, before any write, so no ack precedes a
  drop.
- **Watermark.** The path is `live` (`monitor` for solo), with the existing rule. The strip's
  terms write (#1651) and the route's cell write of one transaction share one stamp and one block
  snapshot, so they apply in the same block. No `TransitionFallback` is set.
- **FP (R17-7).** `route_values` runs inside the guards that R17-7 put at the commit entry and the
  preparation entry. No new guard.

**Gates (fixes).**
- Gate 2 runs on both hosts (solo is on the C ABI too): `hosts/host-web/tests/control_parity.rs`.
- Gate 1 also runs in host-core at `Backend::Simd8` (`cfg(target_feature = "avx2")`) and
  `Backend::Simd4`, with the `Scalar` oracle: the classifier's records written through the
  producers (`crates/host-core/tests/live_routes.rs`).
- Gate 3, rewritten for OQ1 = C: a `follows_mute` toggle gives one seven-word record and no folded
  record; a hand mute on the automated lane gives #1651's terms write and one seven-word record; a
  hand mute on the other lane gives that lane's `Mute` record and one seven-word record; a route
  gain edit gives one seven-word record with the new transform; a terms change while
  `follows_mute` is off gives one seven-word record; an edit that leaves every word unchanged
  gives nothing. The same cases pass through the in-place path: add a session with a mute entry
  and a following send to `crates/host-core/tests/support/edit_sessions.rs`.

**Authorized paths after the check.**
- `crates/host-core/src/prepare.rs` (`route_record_between`'s branch only; after #1652),
  `crates/host-core/src/live_delta.rs`, `crates/host-core/src/route_controls.rs`,
  `crates/host-core/src/lib.rs` (re-exports only).
- `crates/host-core/tests/live_delta.rs`, `crates/host-core/tests/live_routes.rs`,
  `crates/host-core/tests/support/edit_sessions.rs`.
- `crates/control-plane/src/control.rs` (the route write and its pre-write check).
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
  `hosts/host-web/src/tests.rs`.
- No render code and no path under `crates/engine/src/realtime/`.

## Decision record: #1653 (batch 18)

Replaces #1652c's stopgap (`RebuildOnly(RuledLiveNotBuilt { issue: 1653 })` in `route_records`);
no placeholder remains. No render code, nothing under `crates/engine/src/realtime/`.

- **D1 (classifier, `crates/host-core/src/live_delta.rs`).** `route_records` lowers a route the
  next plan gives the seven-word cell through `follow_route_record_between` (terms from the diff,
  `automated` from the structure's `controls::mute_automation`), any other route through
  `route_record_between`. `LiveRouteRecord::record` is now `SendRecord` (`Folded` | `Following`),
  public and re-exported (`host_core::SendRecord`, with `folded()` / `following()`); the swap
  join's `SendRecord` is the same type. The path is `live`.
- **Trigger (fix).** `LevelDiff` gains `terms(before, after)`: `EffectiveDiff::strip_mutes` (dense)
  and the held source strip's `EffectiveStrip::mute` (`LocalDiff`, fast path). `lower_routes`
  lowers a following route also when its terms change with the gate unchanged
  (`follows_mute` off); `LocalDiff::new` holds such a route when its source's mute moved, whatever
  its `follows_mute`. A record is written only when a word's bits change
  (`follow_route_record_between`'s rule, #1652c).
- **D3.** A `follows_mute` toggle is a route row change and writes the cell only (no mirror).
- **D5 (control plane, `crates/control-plane/src/{compile.rs,control.rs}`).** `StripLanes` keeps
  `follow_routes` (the handles' `follow_route_controls`, no longer dropped by `..`). Step 2a
  resolves a `Folded` record among the six-word producers and a `Following` one with
  `host_core::follow_route_producer_position`, and refuses with `CommandError::Internal`, before
  any write, a record with no producer of its kind or whose `automated` bits differ from the
  producer's plan. The writes go under the transaction's one stamp, with the strip's terms
  (`Mute`) records, so they apply in one block. No `TransitionFallback`.
- **Departure (mechanical, outside the listed paths).** `crates/graph/src/lib.rs`:
  `GraphFollowRouteControlProducer` keeps the bound source's `flags.automated` (one
  `[bool; 2]` field set in `follow_route_control_pair`) behind `automated()`, which D5's check
  reads; the producer is charged by `size_of`, so its charge follows. `TestTransactionSnapshot`
  (test observer) gains `follow_route_cells`. `crates/host-core/tests/in_place_commit.rs` gains the
  differential case (the `Pair` harness lives there).
- **`crates/host-core/src/prepare.rs` functions touched.** `SendRecord` (public, accessors, doc),
  `follows_automated_mute` (now `following_route_lanes(..).is_some()`), new
  `following_route_lanes` (the predicate and the `automated` bits in one place). The join and
  `route_record_between` are unchanged.
- **Gates (all pass).**
  - Gate 1: `crates/capi/src/runtime/live_tests.rs`
    `a_follows_mute_toggle_on_a_following_route_is_live`; `hosts/host-web/tests/control_parity.rs`
    `a_follows_mute_toggle_on_a_following_route_is_live_on_both_hosts`;
    `crates/host-core/tests/live_routes.rs`
    `a_follows_mute_toggle_on_a_following_route_settles_on_a_fresh_plan` (Simd8 on AVX2, Simd4,
    Scalar oracle; the classifier's records written through the seven-word producers; every width
    equals the oracle bit for bit).
  - Gate 2: `hosts/host-web/tests/control_parity.rs`
    `solo_and_a_vca_mute_compose_with_the_curve_for_a_following_send_on_both_hosts` (solo path
    `monitor`, VCA mute `live`; a pre-fader route into the output still plays under the mute).
  - Gate 3: `crates/host-core/tests/live_delta.rs`
    `a_following_routes_edits_write_its_seven_word_cell_only` (every case also through
    `classify_live_staged`, equal to the dense delta); in-place path:
    `crates/host-core/tests/in_place_commit.rs`
    `a_following_routes_edits_commit_as_the_full_path_does` over
    `edit_sessions::following_session` / `following_script` (48 and 96 kHz).
  - Superseded rebuild gate `an_edit_of_a_following_route_rebuilds` replaced in place by
    `an_edit_of_a_following_route_is_live` (send gain and hand mute: `live`, the committed
    document's fresh bits after the ramps).
- **Red checks (temporary mutations, reverted).** Folded record still emitted for a following
  route -> gate 3, host-core gate 1, both capi gates and both parity gates red. Terms trigger
  missing in both classifiers -> gate 3 red ("one seven-word record", 0). Terms trigger missing in
  the fast path only -> the in-place differential red. The commit skips the seven-word write ->
  both capi gates and both parity gates red. The pre-write `automated` check removed -> no test
  turns red: every record and every producer of a live commit derive `automated` from the same
  committed structure (automation is structure, so a value commit cannot change it, and the
  newest epoch is prepared from that structure), so the refusal is unreachable through the public
  paths; it stays as D5's defence.

## Decision record: batch 18 cleanup

Test and doc only (U2b review PASS; U2a fold 2 verification PASS). No render, control or
preparation logic changed.

- **MINOR-1 (the seeded `mute` bit).** `crates/host-core/tests/route_mute.rs`
  `a_following_send_leaks_nothing_of_a_stored_mute` gains the case `model(128, PreFader, false,
  true)` with `routes[1].mute = true` and a `mute_entry(0.0, &[(3_000, 1.0)])`, `Live::None`, at
  Simd8, Simd4 and the scalar oracle: the send plane is exact `+0.0` while the strip is heard.
  Red with `mute: false` in the seeded `RouteFollowFlags` (`prepare.rs`): "muted route Simd8: a
  muted following send leaks".
- **MINOR-2 (the join's `before` terms).** New `route_mute.rs`
  `a_rebuild_that_hand_mutes_the_source_ramps_the_send_over_the_edits_length` (all three widths;
  the strip's reach of `+0.0` asserted on the bank widths only, as the scalar oracle restarts its
  strip). A rebuild from an open `t` with a held-open mute entry to a hand-muted `t`, with the
  transaction's `(Strip t, Mute)` edit ramp at 200 (not `M` = 480): the send reaches exact `+0.0`
  at the swap block plus 199 or 200. Red with `[m[1], m[1]]` for the join's `terms`
  (`join_routes`): the send reaches `+0.0` at 479. The helper `follow::live_swap_ramped` takes the
  `EditRamps`; `live_swap` forwards with none.
- **NIT-1 (capi_retained +96 B).** Read from the code, not re-measured: `StripLanes` gained
  `follow_routes: Box<[GraphFollowRouteControlProducer]>`, a 16-byte pointer and length held
  inline in every `ProviderEpoch`. The control row charges the layout of `PENDING_PROVIDERS` (2) +
  `RETIRED_PROVIDERS` (3) parked epochs, and the running epoch is inline in the charged
  `SessionState`: six epochs x 16 B = 96 B (168,650 on `main`, 168,746 at head). An empty box
  allocates nothing. `crates/capi/tests/resource_lifecycle.rs` explains no row by number, so its
  doc is unchanged.
- **NIT-2 (realtime gate).** `crates/control-plane/tests/follow_automation_realtime.rs`
  `a_following_send_of_a_stored_mute_renders_allocation_free` now commits the send's
  `follows_mute` off at block 9 and on at block 10 (`SetControl` on the route's row 7, each
  asserted `EditPath::Live`), inside the muted span, with every render block still allocation-
  and free-free; the silence check moved from block 13 to block 18 so the toggles' ramps end
  first. Red with the "on" write dropped (the send stays open: "the send follows the curve's
  mute").
