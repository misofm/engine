# Compute every effective fader, mute and send gate with one function

Slice S1b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design point C7).
Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

One pure function in the `controls` crate computes every derived value of a session: each strip
lane's effective fader and mute (own value, VCA offsets and mutes, and the solo term) and each
route's follow gate. Preparation receives its output as an argument instead of composing the VCA
itself, and an allocation-free form lets a host call it where allocation is refused. Rendered
output does not change.

## Context

- **Today's composition.** `SessionModel::effective_strip_faders` (`crates/session/src/vca.rs:100-120`)
  returns one `EffectiveStripFader` (`:38-46`: `db`, `mute`, `vca_mute`, each `[_; 2]`) per strip in
  `strips()` order. It sums offsets with `vca_effective_db` (`:23-34`: `f64` sum, own value first,
  then each offset in ascending VCA-ID order, clamp to `[-144, 24]`, one rounding; no offset returns
  the own value bit for bit) over `vca_reach()` (`:52-94`). It has no solo term and no route term.
- **Its callers.**
  - Builtins preparation bakes it into every strip's fader:
    `prepare_session_builtins_with_live_controls_and_policy`
    (`crates/builtins-compiler/src/lib.rs:4507`), call at `:4624`. Public entries:
    `prepare_session_builtins` (`:4389`, 73 calls, mostly tests and tools),
    `prepare_session_builtins_with_live_controls` (`:4413`),
    `prepare_session_builtins_between_render_calls` (`:4436`),
    `prepare_selected_session_builtins_with_live_controls` (`:4455`) and
    `prepare_selected_session_builtins_between_render_calls` (`:4477`) (29 calls together; the
    production caller is host-core).
  - The graph compiler reads the effective strip mutes for `follows_mute` routes
    (`crates/graph-compiler/src/compile.rs:312-322`). It already reads other control-side data from
    `PreparedBuiltinsSession` (`builtins.tails()`, `:140`).
  - host-core's successor join (`join_strips`, `crates/host-core/src/prepare.rs:2793-2799`, and
    `effective_fader`, `:2761-2774`).
  - The browser boot seeds its solo state's VCA mute from it (`hosts/host-web/src/lib.rs:7181-7191`).
  - Tests: `crates/session/tests/vca_composition.rs`, `crates/host-core/tests/vca_live.rs`,
    `crates/graph-compiler/tests/vca_follow.rs`, and builtins-compiler unit tests near
    `crates/builtins-compiler/src/lib.rs:13338`, `:13390`, `:15185`.
- **The live composition, kept separately today.** host-core's `LiveVcaState::effective_db`
  (`crates/host-core/src/vca.rs:295-305`), `LiveControlSoloState::effective_mute`
  (`crates/host-core/src/solo.rs:259-264`: `user_mute || vca_mute || (any_solo && !solo_safe &&
  !solo)`; tracks soloable, every submix solo-safe, `:38-43`) and `LiveRouteMuteFollow::delta`
  (`crates/host-core/src/live_route_state.rs:242`). #1548 moves the browser onto this slice's
  function.
- **The follow gate is per source lane.** A following route zeroes the matrix column of each muted
  source lane (`graph::gated_route_coefficients`, `crates/graph/src/lib.rs:790`), so the gate has one
  value per source lane.
- **S1's derived rows** (`effective_fader_db`, `effective_mute`, `effective_gate`) read a private
  helper (#1546 D7). This slice replaces the helper.

## Decisions frozen for this slice

- **D1. Types (in `controls`).**
  - `MonitorOverlay`: the host-local monitor values. In this slice it holds only the soloed strips,
    as a sorted set of `StableId`. `MonitorOverlay::default()` solos nothing. A soloed ID that names
    a submix or no strip has no effect (a submix is solo-safe). #1559 gives it a writer.
  - `EffectiveStrip { fader_db: [f32; 2], mute: [bool; 2], vca_mute: [bool; 2] }`.
  - `EffectiveRoute { gate: [bool; 2] }` (index = source lane).
  - `Effective { strips: Box<[EffectiveStrip]>, routes: Box<[EffectiveRoute]> }`, strips in
    `strips()` order, routes in `model.routes` order. `Effective::with_shape(strips, routes)`
    allocates; nothing else on `Effective` allocates.
- **D2. The input view.** A trait `EffectiveInputs` that reads without allocating:
  `strip_count()`, `own_fader(strip) -> ([f32; 2], [bool; 2])`, `soloed(strip) -> bool`,
  `solo_safe(strip) -> bool`, `reach(strip) -> &[usize]` (VCA indices in ascending VCA-ID order),
  `vca(vca) -> ([f32; 2], [bool; 2])`, `route_count()`, and
  `route(route) -> EffectiveRouteInput { mute: bool, follows_mute: bool, source_strip: usize }`.
  `ModelInputs::new(&SessionModel, &MonitorOverlay)` implements it for a model; it allocates its
  reach table once (from `SessionModel::vca_reach`, which stays in `session`).
- **D3. The functions.**
  - `controls::effective_into(&impl EffectiveInputs, &mut Effective) -> Result<(), EffectiveShapeError>`:
    never allocates; refuses an `Effective` whose shape differs from the inputs, before any write.
    Per strip and lane: `fader_db = vca_effective_db(own, reach offsets)` (the session function, so
    the bits equal today's); `vca_mute = any reach VCA mute`; `mute = own || vca_mute || solo_term`,
    where `solo_term = any_solo && !solo_safe(strip) && !soloed(strip)` and `any_solo` counts only
    soloed strips that are not solo-safe. Per route and source lane:
    `gate = mute || (follows_mute && strips[source_strip].mute[lane])`.
  - `effective_into` is a loop over three public, allocation-free building blocks, which are the
    only places the rules are written: `controls::any_solo(&impl EffectiveInputs) -> bool`,
    `controls::effective_strip(&impl EffectiveInputs, strip, any_solo) -> EffectiveStrip` and
    `controls::effective_gate(&impl EffectiveInputs, route, source: &EffectiveStrip) -> [bool; 2]`.
    A host that updates one strip at a time (#1548) calls the building blocks; it never spells a
    rule again.
  - `controls::effective(&SessionModel, &MonitorOverlay) -> Effective`: builds `ModelInputs`, then
    calls `effective_into`. One code path; no second composition.
- **D4. Preparation receives it.** `prepare_session_builtins_with_live_controls`,
  `prepare_session_builtins_between_render_calls` and the two `prepare_selected_*` entries gain an
  argument `effective: &controls::Effective`; the private `_and_policy` function bakes
  `effective.strips` where it called `effective_strip_faders`. `prepare_session_builtins` computes
  `controls::effective(model, &MonitorOverlay::default())` itself and delegates (its 73 callers do
  not change). `PreparedBuiltinsSession` keeps the effective strip mutes it was prepared with, as
  control-side data next to its tails, and the graph compiler reads them there instead of calling
  the model. host-core computes the value once per preparation, with
  `MonitorOverlay::default()` in this slice, and passes it. `builtins-compiler` and
  `graph-compiler` gain a dependency on `controls` (no cycle: `controls` never depends on them).
- **D5. Deleted.** `SessionModel::effective_strip_faders` and `EffectiveStripFader` are deleted.
  `vca_effective_db` and `vca_reach` stay in `session` (one arithmetic, one reach). host-core's
  `join_strips` uses `controls::effective` for both models. host-web's boot seed reads
  `controls::effective(model, &MonitorOverlay::default()).strips[i].vca_mute` (named exception in
  stream H's file, see Dependencies).
- **D6. Derived rows.** `controls::list` takes `&MonitorOverlay` and fills the derived rows from
  `effective`. `effective_gate` is a per-lane row, one per source lane (#1546 D5).
- **D7. No behaviour change.** With no solo, every baked fader, mute and follow gate is the value
  that `effective_strip_faders` gave. No resource report moves.

## Deliverables

1. D1-D3 in `crates/controls` (a new module, for example `effective.rs`).
2. D4's argument and the stored mutes; the graph compiler's read.
3. D5's deletions and call-site changes; D6.
4. The tests below; the session composition tests move from `crates/session/tests/vca_composition.rs`
   to `crates/controls/tests/` and call `controls::effective`.

## Authorized paths

- `crates/controls/**`.
- `crates/session/src/{vca.rs,lib.rs,model.rs}` (the deletion, its re-export and two doc links),
  `crates/session/tests/vca_composition.rs` (moved).
- `crates/builtins-compiler/Cargo.toml`, `crates/builtins-compiler/src/lib.rs` (the four entries, the
  `_and_policy` call site, the stored mutes and their accessor, and the unit tests that called the
  deleted function).
- `crates/graph-compiler/Cargo.toml`, `crates/graph-compiler/src/{compile.rs,lib.rs}`,
  `crates/graph-compiler/tests/vca_follow.rs`, and the call sites of the four entries in
  `crates/graph-compiler/**` and `tools/console-workload/src/lib.rs`.
- `crates/host-core/src/prepare.rs` (the builtins calls and `join_strips`),
  `crates/host-core/tests/vca_live.rs`, `crates/host-core/src/{solo.rs,vca.rs}` (doc lines only).
- `hosts/host-web/src/lib.rs`: the boot seed at `:7181-7191` only (named exception).
- `Cargo.lock`.

## Non-goals

- No change to `LiveVcaState`, `LiveControlSoloState` or `LiveRouteMuteFollow` (#1548 uses this
  slice's function from the browser).
- No solo writer and no solo on the C ABI (#1559). No classifier change (#1558 adds step 3).
- No change to the VCA grammar, reach or clamp.

## Objective gates

Commands: `cargo test --locked -p controls`, `cargo test --locked -p session`,
`cargo test --locked -p builtins-compiler --features test-support`,
`cargo test --locked -p graph-compiler`, `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p host-web --lib`, `cargo test --locked -p capi`.

1. **Composition.** The moved session tests pass against `controls::effective` unchanged in their
   expected values (nested VCAs, several VCAs on one strip, the clamp, a VCA mute on one lane).
2. **Solo term.** On two tracks, a submix and a send from track 2 into the submix with
   `follows_mute`: soloing track 1 mutes track 2's two lanes, leaves track 1 and the submix
   unmuted, and sets the send's gate on both source lanes; soloing the submix ID changes nothing;
   a user mute stays muted under solo; a VCA mute mutes a soloed track.
3. **Follow gate per lane.** A source strip muted on its left lane only gives `gate = [true, false]`
   for a following route and `[false, false]` for a route without `follows_mute`.
4. **No allocation.** `effective_into` on a prebuilt `ModelInputs` and `Effective` for a session of
   64 tracks, 4 submixes, 8 nested VCAs and 8 sends performs 0 allocations, measured with
   `bench_support::alloc`'s thread-scoped counters after one warm call. A wrong-shaped `Effective`
   is refused before any write.
5. **Nothing moved (PR evidence, not a committed test).** `./target/release/audit capi` reports the
   same `pcm_digest` on this branch and its merge base, and every resource test passes with no
   re-pin.

## Test value

- Gate 1 turns red if the move changes the VCA sum order, the clamp or the mute OR.
- Gate 2 turns red if the solo term mutes a solo-safe submix, ignores a user or VCA mute, or lets a
  submix ID count as a solo.
- Gate 3 turns red if the gate is computed per route instead of per source lane, which would mute
  both columns of a send whose source mutes one lane.
- Gate 4 turns red if the function allocates, which would make it unusable in the browser's
  render-locked admission (#1548).

## Dependencies

- #1546 (the `controls` crate and the derived rows).
- `hosts/host-web/src/lib.rs` is stream H's file (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`,
  row 101). This slice edits only the boot seed; root places it in that row's order.

## Supersedes and shrinks

- None.
