# Make the browser check, ramp and compose live edits through the core

Slice S7c of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, review edit 7).
Code anchors verified on `main` `df585e503`. Feature sprint mode (owner, 2026-10-13): one
implementer, normal tests, a batch review; no per-attempt verifier or mutation runs.

## Product outcome

The browser's live-edit admission stops spelling control rules of its own. It checks every strip,
route, VCA and solo value with the core's validator, resolves every absent ramp with the core's
resolver, and takes every effective fader, mute and follow gate from the core's effective-value
function. So browser behaviour equals the core's before the browser joins the core (#1562). The
browser's command kinds and wire stay as they are.

## Context

- **Admission.** `admit_commands_staged` (`hosts/host-web/src/lib.rs:5051`) runs inside the
  render-locked `miso_engine_web_v1_command_submit` (`hosts/host-web/src/ffi.rs:4015-4016`), so nothing
  it calls may allocate. `scripts/check-web-audioworklet-callgraph.py` enforces that on the shipped
  artifact (no allocator or drop glue in the export's closure; traps only from the allow-list).
- **Domain literals.** The fader `[-144, 24]` at `lib.rs:4485`, trim at `:4524`, VCA offset at
  `:4604`; the boolean checks `values[0] != 0.0 && values[0] != 1.0` for mute (`:4501`), polarity
  (`:4541`), route mute (`:4577`), VCA mute (`:4609`), solo (`:4629`) and effect bypass (`:4698`).
  The boolean check accepts `-0.0`; `BuiltinParameterDomain::BooleanExact`
  (`crates/builtins/src/lib.rs:312-314`) and `controls::check` refuse it.
- **Ramp resolution.** `CommandRecord::with_session_ramp` (`lib.rs:4385-4406`) maps each kind to a
  `LiveRampRow` and calls `SessionRamps::resolve` (`:1597-1603`), which reads `LiveRamps::for_row`
  for the sentinel `SESSION_DEFAULT_RAMP`; pan and matrix use `resolve_strip_pan` (`:1608`), the
  strip's own window first.
- **Composition.** A fader command on a VCA-reached strip stages
  `ready.vcas.effective_db(track, lane)` (`lib.rs:5166-5167`; host-core
  `LiveVcaState::effective_db`, `crates/host-core/src/vca.rs:295-305`). A mute command stages
  `ready.solo.effective_mute` (`lib.rs:5246-5247`; `LiveControlSoloState::effective_mute`,
  `crates/host-core/src/solo.rs:259-264`), after refreshing the solo state's copy of the VCA mute
  (`set_vca_mute`, `:311`; `LiveVcaState::vca_mute`, `vca.rs:309-317`). The VCA fader pass and the
  solo/VCA-mute coalescing pass follow the loop; the follow pass
  (`lib.rs:5727-5728`, `:5789`) feeds `LiveRouteMuteFollow::delta`
  (`crates/host-core/src/live_route_state.rs:242`) with the same closure.
- **The core functions.** `controls::check` (#1546 D7), `controls::LiveRamps::resolve` (moved by #1546)
  and the effective building blocks `controls::any_solo`, `controls::effective_strip` and
  `controls::effective_gate` over an `EffectiveInputs` view (#1547 D2, D3). All are allocation-free.
- **Effect commands already use one authority.** `into_effect_records` checks values with
  `effect_contract::parameter_value_valid` (`crates/effect-contract/src/lib.rs:883`), and input
  filters with builtins' `validate_input_filter_pair` through `host_core::apply_input_filter_edit`.
  This slice leaves them.

## Decisions frozen for this slice

- **D1. Descriptors at boot.** Boot (off the render lock, where it already allocates) resolves one
  `ControlDescriptor` per row kind the browser admits: builtin `fader_db`, `mute`, `trim_db`,
  `polarity_invert`, `pan`, `matrix_ll`..`matrix_rr`; strip own `solo`; route own `gain_db`, `mute`,
  `matrix_ll`..`matrix_rr`; VCA own `offset_db`, `mute`. It stores them in `ReadyOwnership`. To
  resolve them without a model walk, `controls` gains
  `pub fn row_descriptor(owner: OwnerKind, section: SectionKind, control: u32, sample_rate: u32) ->
  Option<ControlDescriptor>` for the non-effect sections (`OwnerKind` and `SectionKind` are the
  field-free kinds of `Owner` and `Section`; #1551 gives them wire codes). A route row resolves with
  the class of a route into a submix, the only routes the browser admits.
- **D2. One check.** Each arm calls `controls::check(descriptor, lane, value, None)` in place of its
  literal: lane from the command's `channel` (`0` left, `1` right, `2` both; `255` is both for the
  shared rows and solo); pan checks `values[0]` on `Left` and `values[1]` on `Right`; a matrix checks
  the four rows. `control.domain` maps to `COMMAND_REASON_DOMAIN`; `control.lane` and
  `control.ramp.fixed` map to `COMMAND_REASON_MALFORMED` (the browser's reason for a wrong channel
  or a stray word today). The shape checks of the 48-byte record (rack byte, zero words, index
  words) stay: they are wire, not control rules. The lowering calls `builtins::pan_matrix` and
  `Matrix2x2::checked` stay, after the check.
- **D3. One behaviour change.** A boolean value of `-0.0` (mute, polarity, route mute, VCA mute,
  solo) is refused with `COMMAND_REASON_DOMAIN`, as the session grammar and the C ABI refuse it.
  Effect bypass keeps its own check in this slice (effect rows arrive in #1549; #1565 deletes the
  kind).
- **D4. One ramp rule.** `SessionRamps::resolve` becomes
  `self.live.resolve(row, (word != SESSION_DEFAULT_RAMP).then_some(word))`. `resolve_strip_pan`
  keeps the strip's window first and then calls the same function. No ramp length changes.
- **D5. One composition.** host-web gains one temporary type, `WebEffectiveView<'a>` (an
  `EffectiveInputs` impl over `&LiveVcaState`, `&LiveControlSoloState` and `&LiveRouteState`),
  deleted by #1565. Every effective value admission stages comes from it:
  - a fader command on a reached strip stages `controls::effective_strip(..).fader_db[lane]` (for a
    strip no VCA reaches, the effective value is the own value bit for bit, so it stages the
    command's value as today);
  - a mute command, the coalescing pass and the follow pass stage
    `controls::effective_strip(..).mute[lane]`, and the follow pass takes each route's gate from
    `controls::effective_gate`;
  - `any_solo` is computed once per pass with `controls::any_solo`.
  host-core's composition methods are deleted: `LiveVcaState::effective_db`,
  `LiveVcaState::vca_mute`, `LiveControlSoloState::effective_mute`, the solo state's VCA-mute copy
  (`set_vca_mute` and `StripMuteSeed::vca_mute`) and `followed_lanes`. The mirrors keep their
  values, shadows, emitted mirrors and deltas, and gain only the read accessors the view needs
  (each lane's own fader dB of every strip, each strip's reach slice, each VCA's values, each live
  route's `mute`, `follows_mute` and source strip). `LiveRouteMuteFollow::delta` takes the gates.
- **D6. Records do not change.** For every batch the existing host-web tests send, the staged
  records (lanes, values, ramps, wire indices, order) are the same as before. The passes, the
  coalescing rule and the never-a-redundant-record rule stay as they are.
- **D7. Realtime.** No function on the admission path allocates, frees or adds a trap owner. A
  `controls` function that admission calls uses checked access (`get`), never a panicking index.
- **D8. Acked-batch question.** The checks run where the literals ran, before the room pass and the
  stamp; a refused batch rolls back every mirror as today. No ack can precede a drop.

## Deliverables

1. `controls::row_descriptor`, `OwnerKind`, `SectionKind` (D1).
2. host-web: the boot table, D2's checks, D4, the view and D5's call sites.
3. host-core: D5's deletions and accessors; its tests migrated.
4. The tests below.

## Authorized paths

- `crates/controls/**`.
- `crates/host-core/src/{vca.rs,solo.rs,live_route_state.rs,lib.rs}`,
  `crates/host-core/tests/vca_live.rs`, `crates/host-core/Cargo.toml`.
- `hosts/host-web/src/lib.rs` (`CommandRecord`'s value checks and `with_session_ramp`,
  `SessionRamps`, `admit_commands_staged`'s composition sites, `ReadyOwnership`'s new table, the
  boot seed), `hosts/host-web/src/tests.rs`, `hosts/host-web/Cargo.toml` (the `controls`
  dependency), `Cargo.lock`.

## Non-goals

- No change to the 48-byte record, the command kinds, reason codes (except D3's case), the SDK or
  the worklet script.
- No change to effect, observation or input-filter admission.
- No C ABI change, and no move of admission into the Worker (#1562).

## Objective gates

Commands: `cargo test --locked -p controls`, `cargo test --locked -p host-core --all-features`,
`cargo test --locked -p host-web --lib`, `cargo test --locked -p host-web --features
host-web/test-support --tests`; the browser artifact gates
`bash scripts/build-web-audioworklet.sh --named-twin <twin-dir> <artifact-dir>`, then
`bash scripts/check-web-audioworklet.sh <artifact-dir> <twin>` (the call-graph gate) and
`bash scripts/test-web-audioworklet.sh`.

1. **Records unchanged.** Every existing host-web admission test passes with no expected value
   changed (VCA, solo, follow, mute-split, route and ramp-resolution tests included).
2. **Domain from the row.** A host-web test sends, for fader, trim, VCA offset and route gain,
   each range end (accepted) and the next `f32` beyond it (refused `DOMAIN`), and for each boolean
   kind `0.5` and `-0.0` (refused `DOMAIN`). A test in `controls` changes nothing; the point is that
   the browser's answers equal `controls::check`'s for the same row and value: the test asserts
   both for every case.
3. **Composition from the core.** A host-web test with two nested VCAs, a solo and a following send
   checks, after each batch, that every staged fader and mute value and every follow record's
   source lanes equal `controls::effective(model', &overlay)` of the model the batch would commit.
4. **Realtime.** The call-graph gate passes with no new trap owner, and the render-locked
   allocation count stays exactly zero in the existing host-web allocation tests.

## Test value

- Gate 1 turns red if moving the composition changes any staged record, ramp or wire index.
- Gate 2 turns red if the browser and the core disagree on a value's domain or on `-0.0`.
- Gate 3 turns red if the browser composes VCA, solo or follow differently from the function
  preparation and the C ABI use.
- Gate 4 turns red if a core function on the admission path allocates or adds a panic.

## Dependencies

- #1546 (`controls`, `check`, `LiveRamps`) and #1547 (`effective_strip`, `effective_gate`,
  `any_solo`, `EffectiveInputs`).
- **Question for root.** `hosts/host-web/src/lib.rs` is stream H's file
  (`docs/handoffs/decision-15-2026-10-05/STREAMS.md`, row 101), and this slice rewrites part of
  `admit_commands_staged`, which stream H's #1332 rewires and #1562 then moves. Root must place this
  slice in row 101's order (before #1332 is the smaller rebase) and name it as an exception.
- The VCA rows are per lane (#1546 D5), as the browser's VCA kinds 16 and 17 are (`channel` 0, 1,
  2; `lib.rs:917-939`), so D2 changes no VCA acceptance.

## Supersedes and shrinks

- *Route the builtins fader domain checks through checked_fader_gain* (#1303): its non-goal that
  host-web keeps its literal is overridden here (see `amendments-a.md`).
