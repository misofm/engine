# Render stored VCA offset and mute automation

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), root's ruling on finding F20 (2026-10-14, README "Root's rulings (2026-10-14)"),
under decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, C3, C6 and C7). A draft for
root to file. Written by the controls coordinator on 2026-10-14. Code anchors verified on
`2e6343a99`. Batch R2, after #1651-#1653.

## Product outcome

A producer automates the "vocals" VCA: an `offset_db` ride and `mute` steps. Every member lane
plays the ride and the steps on every host, in the member's own node time, so a member behind a
latent insert hears the VCA move with its own audio. Sends that follow a member's mute follow the
automated VCA mute at their tap. A hand move of an automated VCA offset writes a stored dB offset,
and a hand VCA mute ORs with the automated VCA mute (D5). A VCA automation edit is a carried
rebuild. A session with no VCA automation renders the bits it renders today.

## Context

- **The rows.** A VCA has two rows, `offset_db` (1) and `mute` (2), both per lane, family
  `StripLevel`; `offset_db` has the fader domain and the ramp row `VcaOffset`, `mute` is a toggle
  with the ramp row `VcaMute` (`crates/controls/src/vca.rs:13-72`). A VCA has no render cell: a
  change reaches render through its members' effective fader and mute (#1558). `automatable` is
  computed (`crates/controls/src/descriptor.rs:124-132`), so both rows are targets today, and
  `fixtures/session/v1/builtins-automation.json` holds the entry `vca-ride` (target owner `vca`,
  `riders`). Nothing renders it: the classifier masks the whole table
  (`crates/host-core/src/live_delta.rs:359-361`).
- **The fader composition.** #1646 D6 (README F16, root's decision): a member's gain is
  `checked_fader_gain(vca_compose_db(member, S))`, `S` the `f64` sum of the reaching offsets in
  ascending VCA ID, offsets first. #1646 D1 gives each automated fader lane an offsets cell (`S`
  as one `f64`, one ramp word); #1649 writes it on every VCA edit; #1650 puts the lane's hand
  offset first in `S`.
- **The mute composition.** #1651 (amended): the stage mute of an automated mute lane is
  `curve || terms`, `terms = static_mute || reaching VCA mutes || solo term` in one cell word.
  #1652 (amended) gives a following send of an automated source lane a seven-word route cell
  and evaluates the source curve in the tap's node time.
- **The reach.** Nested VCAs reach members through `session::vca`; the host caps the reach pairs
  (`maximum_vca_reach_pairs`; the browser's `MAXIMUM_VCA_REACH_PAIRS`).
- **The shape rule.** Amended #1645: a toggle row takes only `step`.

## Decisions frozen for this slice

- **D1. Programs.** Each automated VCA lane compiles once, with #1637's builder: `offset_db`
  with grid period and ramp 64, completion `End`, jump row `VcaOffset`; `mute` with jumps only,
  row `VcaMute`. The segment table is plan-owned once per VCA lane. Each member lane the VCA reaches
  holds its own cursor and event state for it and evaluates it in the member's fader node time
  `a(fader)` (#1647), because a VCA has no audio of its own.
- **D2. Offset composition.** A member lane's fader stage runs events when its own fader is
  automated or when an automated VCA offset reaches it ("event lanes"). Events are the union of its
  own curve's events and each reaching automated VCA's events. At each event:
  `S_t = S + c_1(τ) + … + c_m(τ)` in `f64`, the `c_k` the reaching automated VCAs' curve values in
  ascending VCA ID, then `gain = checked_fader_gain(vca_compose_db(own, S_t))`. `S` (the cell) holds
  the lane's own hand offset, then the static offsets of the reaching VCAs that are not automated,
  then the hand offsets of the reaching automated VCAs, each group in ascending VCA ID. `own` is
  the member's curve value when its fader is automated, else the cell's `own_db` word (D3).
  - A session with no VCA automation has no `c_k` and no `own_db` word: the composition is
    #1646's and #1649's, and no bit moves.
  - A flat VCA curve at `v` gives the bits of a static VCA offset `v` whenever F16's exactness
    condition holds (each nonzero term at least `k·2^-20` dB, so every partial sum is exact); the
    order "static terms, then automated terms" can differ from ascending-ID order only in the
    degenerate cases F16 lists. Root records it as a class B order under the standing
    summation-order ruling, with F16.
- **D3. The cell of a member that is not automated.** An event lane whose own fader is not
  automated gets #1646's offsets cell plus one word, `own_db` (the member's static fader dB,
  `f32`): four words, 48 bytes in #1312's three slots. A hand fader move on that lane is a live
  value, as today, but the classifier writes `own_db` through the cell (#1649 D4's write, ramp
  `LiveRamps::resolve(Fader, the edit's ramp)`) instead of a `FaderDb` record, and render jumps at
  the next block entry only if the gain bits change (#1649 D3).
- **D4. Mute composition.** A member lane reached by an automated VCA mute uses #1651's
  `terms` word whether or not its own mute is automated. Its stage mute is
  `own_curve || terms || v_1(τ) || … || v_m(τ)`, the `v_k` the reaching automated VCA mute curves
  in the member's node time, with `own_curve` false when the own mute is not automated. `terms`
  holds the static mute, the static mutes of every reaching VCA (automated or not: D5) and the solo
  term. Events are the curves' discontinuities; only a change emits #1638's `SetMute`, over the
  `VcaMute` jump row for a VCA curve event. A following send of such a member uses #1652's
  seven-word cell, and its follow term ORs the same `v_k` evaluated in the tap's node time
  `τ_tap`, so the send closes on the timeline sample where the member mutes (README A6).
- **D5. Hand edits** (OQ1 = C with root's ruling on level controls).
  - `SET_CONTROL` on an automated VCA `offset_db` writes the entry's stored offset (#1650's
    field; `takes_offset` gains VCA `offset_db`, per lane), and the classifier rewrites `S` of every
    member lane the VCA reaches (#1649 D4's write). This reads root's ruling "send gain is a
    level control, so it takes an offset" as covering the VCA offset, a level control too; root
    confirms at filing. If root reads it otherwise, the edit stores the fallback and commits
    `model_only` (#1648 D3), and gate 4's offset case expects that.
  - `SET_CONTROL` on an automated VCA `mute` ORs: the VCA's static mute stays live and is written
    into `terms` of each reached member lane (#1651's terms write).
  - Edits of a VCA that is not automated, and reach changes that involve only such VCAs, stay
    #1649's live `S` write.
  - A change of which member lanes an automated VCA reaches (a membership or nesting edit, adding or
    removing an automated VCA) needs new event state for the new pairs: a carried rebuild, for the
    new-memory reason (decision 14 rule 1). It narrows D15-6's live set for automated VCAs only;
    root sees it at filing.
- **D6. Rendering rows.** VCA rows 1 and 2 join #1648 D1's `automation_row_renders`. A VCA
  automation edit is a carried rebuild; each member pair's event state carries by `(VCA ID, row,
  member strip, lane)`, as #1648 D2 carries a fader cell.
- **D7. Memory.** Per automated VCA lane one program (`32 + 32·n` bytes, A4's header and
  segments); per (member lane, automated VCA lane) pair one event state (48 bytes); per event lane
  whose own fader is not automated 12 bytes more (D3's word in three slots). Every term is bounded
  by the session and the host's reach-pair cap; none reads the song length.
- **D8. Work.** A member lane takes at most `(1 + m)(2⌈q/64⌉ + 3)` fader events per block, `m` the
  automated VCA offset lanes that reach it, and `m` more `f64` additions per event.
- **D9. The acked-batch question: can an ack ever precede a drop? No.** Curve events come from the
  plan; every check of a transaction precedes the first cell write; a cell write cannot fail.

## Deliverables

1. D1-D4 in host-core preparation (programs, member pairs, cells and seeds), the fader stage's
   events and mute terms, and the route op's follow term.
2. D5-D6 in `crates/controls` (`takes_offset`, the rendering rows) and the classifier.
3. `docs/SESSION_SCHEMA_V1.md`: VCA rows render; the composition order of D2.

## Authorized paths

- `crates/host-core/src/{prepare.rs,live_delta.rs}`, `crates/host-core/tests/{vca.rs,live_delta.rs}`
- `crates/builtins-compiler/src/lib.rs` (the fader stage's VCA events, the `own_db` word and the
  mute terms only)
- `crates/graph/src/{lib.rs,runtime.rs}` (the route op's follow term only)
- `crates/controls/src/{automation.rs,vca.rs}` and their tests
- `crates/capi/src/runtime/live_tests.rs`, `hosts/host-web/tests/control_parity.rs`,
  `crates/control-plane/tests/vca_automation_realtime.rs` (#1649's binary, extended)
- `docs/SESSION_SCHEMA_V1.md`

## Non-goals

- A VCA with an audio path; VCA solo.
- Any change to a session without VCA automation (D2).
- Sends' own rows (#1654).

## Hazards

- **Class B order.** D2's order equals ascending-ID order except in F16's degenerate cases; root
  records it with F16. Gates use values on the 0.1 dB lattice.
- **Hot files.** `crates/host-core/src/prepare.rs` and `crates/builtins-compiler/src/lib.rs` are
  shared with #1651-#1654; root sequences the merge.

## Objective gates

Commands: `cargo test --locked -p host-core --features test-support`, `cargo test --locked -p
builtins-compiler --features test-support`, `cargo test --locked -p controls`, `cargo test --locked
-p graph --features test-support`, `cargo test --locked -p capi`, `cargo test --locked -p host-web
--features test-support --test control_parity`, `cargo test --locked -p control-plane --features
test-support`, `./target/release/realtime_policy`, `bash scripts/check-cross-targets.sh` (README
F19), clippy `-D warnings`, `cargo fmt --all -- --check`.

1. **Flat equals static** (C ABI and browser). A flat VCA offset ride at `-6` dB and a flat VCA mute
   at 0 render, bit for bit, the session with those static values: for a member with its own fader
   ride and for a member with none.
2. **Member node time.** A linear VCA ride reaches two members, one behind the true-peak limiter.
   Each member's gain at each grid completion equals `checked_fader_gain(vca_compose_db(own, S +
   c(τ)))` computed on the test thread, and the latent member's events fall `L` samples later in
   render time.
3. **Mute.** A VCA mute step at a sample that is not a grid sample mutes each member at its node
   time over the `VcaMute` ramp; a following send of a member closes at its tap (#1652's rule);
   after the ramps the output equals a static-mute plan.
4. **Hand edits.** On an automated VCA: `SET_CONTROL offset_db = +3` gives `live`, stores the
   offset and, after the jump, equals a fresh plan with that offset; a hand mute ORs (closes an
   open curve, leaves a closed curve closed). A hand fader move on a member with no own automation
   gives `live` and one `own_db` write, no `FaderDb`. A membership edit of the automated VCA gives
   `rebuild` and completes `exact`; the same edit of a VCA that is not automated gives `live`.
5. **Realtime.** The events, cell reads and writes allocate and free nothing on the render thread
   (`bench_support::alloc` thread counters).
6. **No rendered bit moves** for a session without VCA automation: `audit capi` gives the same
   `pcm_digest` at base and head (PR evidence); the browser legs pass with unchanged digests.

## Test value

- Gate 1 turns red if the VCA curve composes with other arithmetic than the static path.
- Gate 2 turns red if VCA events use the render clock or the VCA's own (non-existent) arrival
  instead of each member's.
- Gate 3 turns red if a following send of a member keeps passing audio under an automated VCA mute.
- Gate 4 turns red if a hand VCA move writes the fallback, if a plain member's hand move fights the
  VCA curve with a `FaderDb` record, or if an automated VCA's reach change commits live with no
  event state for the new pair.

## Dependencies

- #1651, #1652 and #1653 (the terms word, the seven-word route cell, the follow evaluation).
- #1649 and #1650 (the offsets cell, its write and the offset field), #1648 D1
  (`automation_row_renders`), #1639 (jump rows), amended #1645 (the shape rule), #1647
  (member node time).
- Batch R2, after #1653. A complete outcome alone; not in a must-land group.

## Ruling (root, 2026-10-14)

- **Hand move.** A hand move of an automated VCA `offset_db` writes an offset, the same rule as
  fader, trim and send gain (a level control).
- **Summation order.** Static terms are summed first, then the automated VCA curves: a class B
  change under the standing summation-order ruling, recorded with F16.
- **Membership of an automated VCA.** A change to which members an automated VCA reaches is a
  carried rebuild. Decision 14, rule 3 entry, for automated VCAs only: the reach change needs new
  memory (an `own_db` cell word per newly reached lane and the member's curve tables), so it cannot
  be a live update. Non-automated VCAs keep D15-6's live set.
