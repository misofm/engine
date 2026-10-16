# Prepare stored fader automation and render it flat

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.1, A1.4, A1.5) and A4, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

Host-core preparation, on the C ABI and in the browser, compiles each automated fader lane of the
session's `automation` table into a cell program on the strip's fader bank, charges its bytes to
the graph plan rows, and starts the lane at its prepared value: the curve at the fader's node time
at render sample 0. A flat curve at `v` therefore renders the bits of a static fader at `v`, on
both hosts and through any VCA composition. A session with no stored automation prepares and
renders exactly as before. Moving curves, seeks and latency in render are #1647, in the same
batch and push.

This slice lands after *Admit browser live edits in the Worker through the committed model*
(#1382). From then on every browser live edit goes through the shared commit, so the browser and
the C ABI meet one rule for a live edit of an automated lane (#1648), and no host-only
admission path remains that could write a live record onto an automated lane.

## Context

- **The table renders nothing today.** `docs/SESSION_SCHEMA_V1.md:220-225`; the builtin target
  vocabulary (`crates/session/src/validate.rs:812-853`). The fader row is builtin parameter 5,
  `PerLane` (`:830-831`).
- **Where the fader is prepared.** Builtins lowering bakes each strip's VCA-effective fader, once
  (`crates/builtins-compiler/src/lib.rs:3519-3521`), from `effective_strip_faders`
  (`crates/session/src/vca.rs:96-120`): `vca_effective_db(own, offsets)` sums the member's value
  and each reaching VCA's offset in `f64`, in ascending VCA-ID order, clamps and rounds once
  (`:19-34`). The gain is `checked_fader_gain` (`crates/builtins/src/lib.rs:4292-4297`), which calls
  `db_gain`, `10^(dB/20)` through `math::pow` in `f64`, rounded once (`:5356-5364`).
- **The fader bank.** `FaderBankProcessor` (`crates/builtins-compiler/src/lib.rs:715-723`); the
  per-node scalar fader is `FaderMuteRampBuiltins` (`crates/builtins/src/lib.rs:4163-4233`). The
  fader node is `TrackStage::PostFader` (`crates/graph/src/lib.rs:257-265`).
- **The fader is seam-side.** The mono-collapse seam sits immediately before the fader: a collapsed
  track duplicates its one plane into the fader, whose two channels are free to differ, so no fader
  word gates the collapse (`SEAM_SIDE_WITNESS`, `crates/builtins-compiler/src/lib.rs:391-398`;
  `:756-771`). The collapse predicate is `track_mono_source` (`:3967-3970`), and an asymmetric
  input delay declines it through the structural witness (`:4025-4032`).
- **Graph plan rows.** Builtin bank storage joins `builtin_bank_bytes`,
  `graph_incremental_plan_bytes` and `graph_session_plus_plan_bytes` through
  `GraphResourceEstimate::checked_add_builtin_banks` (`crates/graph/src/lib.rs:654-687`); host-core
  admits the total against `maximum_graph_session_plus_plan_bytes`
  (`crates/host-core/src/prepare.rs:1609-1627`), and the C ABI's replacement peak adds both plans'
  `graph_session_plus_plan_bytes` (`crates/capi/src/runtime/compile.rs:434-457`). The graph
  compiler's own estimate (`crates/graph-compiler/src/estimate.rs:28-160`) sees no builtin bank
  and no automation.
- **The floating-point environment.** Native render entries pin it with `CanonicalFpEnv`
  (`crates/lane/src/fpenv.rs:287-290`; the wasm form `:369-372` changes nothing), entered at
  `crates/host-core/src/render_session.rs:111`; preparation runs in the host's environment (README
  finding F11).
- **What this slice builds on.** #1626 and #1645 (one entry per lane, the hold rule, the fader's
  unit and domain), #1637 (the builder, its byte report and the jump-length key), #1638 (`SetGain`), #1639
  (the plan cell that holds the jump lengths, seeded from #1054's
  `control_smoothing_samples()`). #1285 D2 records each node's floored input arrival `a(n)`. #1312
  gives the latest-target cell (`crates/engine/src/realtime/latest_cell.rs`, #1312 D1); its cells hold two words (fader,
  mute) or five (matrix) (#1312 D3), and this slice's offsets cell holds three (D1).
- **Root's decision F16** (README, "Findings for the coordinator"): the VCA offsets add first,
  then the member (D6), under the standing summation-order ruling as root reads it.

## Decisions frozen for this slice

- **D1. Cells.** Host-core preparation compiles one cell per automated fader lane (target rack
  `builtins`, parameter 5; `both` gives two cells) with #1637's builder:
  - `a(n)` is the floored input arrival of the strip's `PostFader` node (#1285 D2);
  - grid period 64, grid ramp 64, completion `End`; jump length `JumpLength::Block` (#1637 D2,
    as shipped): the program stores no jump length; at render the caller resolves the fader
    row's jump length for the block with `LiveRamps::for_row` from #1639's plan cell and passes
    it to `CellEvents::new` as one `u32` (#1637's decision record); no `no_restart`;
  - **the offsets cell** (layout 4 of README finding F16). Preparation builds
    #1649's offsets cell for every automated fader lane, in every session, with or without
    VCAs: a #1312 latest-target cell of three words, the lane's offset sum `S` as one `f64` (two
    `u32` words) and one `u32` ramp word that #1649 writes (the length of the jump a VCA edit
    asks for; seeded with the session's fader length, #1054). `S` is
    `vca_offsets_sum(reach offsets)` (D6) for the lane's channel, and `+0.0` when no VCA reaches the
    lane. A value `v` becomes the gain `checked_fader_gain(vca_compose_db(v, S))` (D6), with `S`
    from the cell's newest slot. No static copy of `S` is kept in the program, so one source holds
    it; #1649 adds only the live VCA edit that rewrites the cell. Because the cell exists on
    every automated fader lane, a VCA that a later edit adds or attaches only rewrites it.
  - **Why the cell keeps the bits.** With a reach, D6 makes the static path compute
    `vca_compose_db(member, S)` with the same `S`, so the bits are equal by construction. With no
    reach, the static path returns `v` unchanged, and render computes `clamp(f64(v) + (+0.0))`. In
    `f64`, `v + (+0.0) == v` for every `v` except `-0.0`, which becomes `+0.0`; `v` lies in
    `[-144, 24]` (#1645), so the clamp changes nothing. `db_gain(+0.0)` and `db_gain(-0.0)` are
    both exactly 1 (`math::pow(10, ±0) = 1`), so the gain has the static bits in every case.
    #1637's "change only" compares the curve value, before the offsets, and #1649 D3 compares the
    composed gain, never the dB sum, so a sign of zero never causes a retarget.
  - **Where the event state lives.** Each cell's event state (#1637's `CellState`) lives in the
    fader bank stage beside the lane's ramp state, keyed by the cell's stable address (strip, row
    5, channel). So the stage's carry program (*Carry fader, mute and pan ramps across a plan
    swap*, #1277) moves it with the ramp state at a swap, and no second carry path exists. #1648
    relies on that carry.
  The program is installed on the strip's fader bank processor (or the scalar track's), as
  immutable plan data.
- **D2. Prepared value.** After the graph compile and before publication, each automated lane is
  set exactly (#1638 `SetGain`, off the render thread) to the curve at node time `-a(n)`,
  composed as in D1. By the hold rule the curve at any `t <= first.start_sample` is the first
  `start_value`, and every `start_sample >= 0`, so `curve(-a(n)) == curve(0)` always: the prepared
  value is the entry's first `start_value`, whatever `a(n)` is (README A1.4, "New lanes"). A lane
  whose curve is flat at `v` is therefore set to the bits a static `v` prepares. Every curve
  evaluation and conversion of preparation runs inside `CanonicalFpEnv`, as render's do (README
  A1.1). A conversion that fails is unreachable (#1645
  bounds the curve to the fader domain and `vca_compose_db` clamps, D6) and refuses preparation with
  `host.automation.fader`.
- **D3. Mono collapse is kept.** The fader is seam-side (Context), so a fader automation that drives
  one lane, or both lanes with different curves, keeps the track's collapse. This refines the
  README's mono-collapse rule (A1.5), which holds for rows upstream of the seam: trim, polarity,
  HPF, LPF and strip effects (#1658, #1659-#1660, #1663-#1664). Nothing is added to
  `track_mono_source`.
- **D4. Bytes.** Each fader program's bytes (builder report, #1637 D5) and each offsets cell's
  bytes (three slots of three 4-byte words, 36 bytes, plus the cell's fixed words) are charged as builtin
  bank payload through `checked_add_builtin_banks`, so they reach `builtin_bank_bytes`,
  `graph_incremental_plan_bytes` and `graph_session_plus_plan_bytes`, the caller's
  `maximum_graph_session_plus_plan_bytes` and the C ABI's replacement peak. This row is more exact
  than `crates/graph-compiler/src/estimate.rs`, which never sees the program. No ABI row is added.
- **D5. The acked-batch question.** Stored automation has no queue: the program is plan data.
  Nothing is acknowledged and later dropped.
- **D6. The composition order (README F16, root's decision).** `crates/session/src/vca.rs` gains two functions
  and `vca_effective_db` is written from them, so every caller (`effective_strip_faders`, #1247's
  classifier row, host-core's live VCA state, builtins lowering) takes the new order:
  - `vca_offsets_sum(offsets_db) -> Option<f64>`: `None` for no offset; otherwise the `f64` sum in
    the order given, starting from the first offset;
  - `vca_compose_db(member_db: f32, offsets_sum: f64) -> f32`:
    `(f64(member_db) + offsets_sum).clamp(-144, 24) as f32`;
  - `vca_effective_db(member, offsets)`: `member` bit for bit when `vca_offsets_sum` is `None`,
    otherwise `vca_compose_db(member, sum)`.

  `effective_strip_faders` still passes the reach in ascending VCA-ID order. Three test oracles
  compute the member-first order today and are rewritten in this change to sum the offsets first,
  then add the member (`vca_offsets_sum` and `vca_compose_db`, or the same arithmetic written out):
  - `reference_db` (`crates/host-core/tests/vca.rs:268-279`), with its rustdoc;
  - the `lane` closure of `a_browser_vca_renders_as_its_effective_faders_under_solo_and_mute`
    (`hosts/host-web/src/tests.rs:13358-13371`);
  - `vca_reference_effective` (`hosts/host-web/src/tests.rs:13697-13715`), with its rustdoc. Its
    clamp flag compares the clamped value with the new sum.

  The sentence at
  `docs/SESSION_SCHEMA_V1.md:80` becomes "summed in `f64` (the reaching VCAs' offsets first, in
  ascending VCA ID, then its own value added to that sum)". This is a class B change to #1242's
  order that root ruled under the standing summation-order ruling (README F16); it moves the bits of a static session only when
  a nonzero member or offset is smaller than `k·2^-20` dB, `k` the term count (README F16).

## Deliverables

1. D1, D2 and D4 in `crates/host-core/src/prepare.rs` and `crates/builtins-compiler/src/lib.rs` (the
   program and the event state on the fader bank processor and the scalar track, the offsets cells
   and their seed, the prepared value, the bank-payload charge).
2. The three-word cell of D1 in `crates/engine/src/realtime/latest_cell.rs`, with its loom case,
   only if #1312's module has no three-word form (its two- and five-word cells, #1312 D3, show the
   form it uses).
3. D6 in `crates/session/src/vca.rs` and `crates/session/src/lib.rs`, its gate rewritten in
   `crates/session/tests/vca_composition.rs`, the three oracles of D6 rewritten in
   `crates/host-core/tests/vca.rs` and `hosts/host-web/src/tests.rs`, and the sentence in
   `docs/SESSION_SCHEMA_V1.md:80`.
4. Tests in `crates/host-core/tests/stored_fader_automation.rs` (new),
   `crates/capi/src/runtime/tests.rs` and `hosts/host-web/src/tests.rs` (one test).

## Authorized paths

- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/stored_fader_automation.rs` (new)
- `crates/builtins-compiler/src/lib.rs` (program and event-state installation on the fader bank
  processor and the scalar track; the offsets cell's read; the program charge)
- `crates/capi/src/runtime/tests.rs`, `hosts/host-web/src/tests.rs` (one new test, and the two
  oracles of D6 at `:13358-13371` and `:13697-13715`)
- `crates/host-core/tests/vca.rs` (`reference_db`, `:268-279`, only)
- `crates/automation/src/events.rs` (one method that seeds a `CellState`'s target, and its test,
  only if D2's event state needs a target at preparation: `CellState::new` leaves it empty and no
  setter exists; #1648's batch 13b amendment authorizes the same method, and whichever lands first
  adds it)
- `crates/engine/src/realtime/latest_cell.rs` (a three-word cell form and its loom case, only if
  #1312's module has none)
- `crates/session/src/vca.rs` (D6), `crates/session/src/lib.rs` (the `pub use` at `:31` only),
  `crates/session/tests/vca_composition.rs` (gate 1's rewrite, the order cases and the reference),
  `docs/SESSION_SCHEMA_V1.md` (the composition sentence at `:80` only)
- Tests that pin a builtin bank or graph plan row that D4 moves for a session with automation (none
  exists today; a session without automation moves no row)
- `crates/host-core/Cargo.toml` (a normal `automation` dependency: preparation runs #1637's
  builder), `Cargo.lock`

## Non-goals

- Events in render: the grid, jumps, held events, seeks, the timed chain call (#1647).
- The documents that say the fader row renders (#1647).
- Classifying fader automation edits, the rule for a live edit of an automated lane, and carrying
  event state across a swap (#1648).
- The live VCA move that rewrites the offsets cell (#1649). The plan cell of jump lengths and
  its edit path (#1639).
- Mute, pan, matrix, input and effect rows (#1651 to #1667).

## Hazards

- **Batch R1, one push with #1647, #1648 and #1649** (README "Must-land-together groups"). Alone,
  this slice starts a moving curve at its prepared value and holds it there; #1647 renders the
  motion, #1648 stops a live fader edit from fighting the curve, and #1649 makes a VCA ride
  heard on an automated member. The four merge in one push, so `main` never holds a partial state.
- **Host-core is stream A's file** (`crates/host-core/src/prepare.rs`); root orders the merge after
  #1285 and #1312.
- **Bit-identity with VCAs.** D6's one rule serves the static path and the cell, so `S` in the
  cell is the static path's own sum; #1649's live edit computes it with the same function, so
  it keeps the same property.
- **A pinned order changes** (D6). The amendment to #1242's documented order, its pinned test and
  the three oracles of D6 land in this slice, together, so `main` never holds a test that pins or
  computes the old order.

## Objective gates

1. **Flat equals static** (`crates/capi/src/runtime/tests.rs`, new; `hosts/host-web/src/tests.rs`,
   new). A session with a flat fader automation at `v = -7.25` dB (one `linear` segment from `v`
   to `v`) on one track renders, bit for bit, the session with static `fader_db` `-7.25` and no
   automation, for 64 blocks, on the C ABI and on the browser host. The same with a `both` entry on
   a track one VCA reaches with an offset of `+3` dB, and on a track three VCAs reach, with a flat
   value and offsets for which the two orders of README F16 give different gain bits. Different
   `f64` sums are not enough: they can round to the same `f32` dB value. An example: the flat value
   `f32::from_bits(0x405a_f905)` (about 3.42145 dB) and the offsets, in ascending VCA-ID order,
   `f32::from_bits(0xc1af_fc6c)` (about -21.99825 dB), `24.0` and `f32::from_bits(0x2624_8cd2)`
   (about 5.7e-16 dB). The member-first order gives the dB bits `0x40ad_8ad3`, the offsets-first
   order `0x40ad_8ad2`. The test first asserts that `checked_fader_gain` of the two orders' dB
   values differs in bits, so a render that keeps the other order than the static path cannot
   pass. The same in a session with two VCAs, for a track that neither reaches and whose flat value
   is `-0.0` dB (the cell's `S = +0.0`).
2. **Bytes.** The same session with and without one automated fader lane: `builtin_bank_bytes`,
   `graph_incremental_plan_bytes` and `graph_session_plus_plan_bytes` each differ by the builder's
   byte report for that program, and a `maximum_graph_session_plus_plan_bytes` one byte below the
   session's row refuses it with `host.graph.resource.limit`.
3. **Mono collapse kept.** A mono track (source channels `0, 0`) with a flat `left`-only fader
   automation at a value its right lane does not have keeps its collapse, and its output equals the
   dual render of a twin whose two source channels carry the same PCM (`0, 1`).
4. **No change without automation.** Every existing render test passes with unchanged digests;
   `./target/release/audit capi` shows the same `pcm_digest` as the base (PR evidence; the audit
   session has no automation) and 0 violations.
5. **The three-word cell**, only if Deliverable 2 adds a form (`crates/engine/src/realtime/latest_cell.rs`,
   new loom case beside #1312's `spsc_loom_cells_*`). A three-word cell never shows a slot that
   mixes two writes and never skips the newest completed write under a racing writer.
6. **The order** (`crates/session/tests/vca_composition.rs`, gate 1 of #1242 rewritten in place).
   The reference sums the offsets first, then adds the member. The crafted cases become
   `vca_effective_db(24.0, [-24.0, 1e-30]) == 0.0` (today `1e-30`, line 81),
   `vca_effective_db(1e-30, [24.0, -24.0]) == 1e-30` (today `0.0`) and
   `vca_effective_db(0.0, [24.0, -24.0, 1e-30]) == 1e-30` (offsets in the order given); the
   no-offset and clamp cases stay. Its rustdoc (`:46-51`) states the new order.
7. **Commands:**
   - `cargo test --locked -p session`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p builtins-compiler --features test-support`
   - `cargo test --locked -p capi`
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `CARGO_TARGET_DIR=target/ci/loom RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test
     --locked --release -p engine --lib spsc_loom`
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`
   - `bash scripts/run-aarch64-tests.sh debug` (the `aarch64-debug` job)
   - `bash scripts/check-host-core-policy.sh`, `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if render composes the offsets in another order than the static path, if the
  `+0.0` of a lane no VCA reaches changes the gain bits, or if either host converts dB differently.
- Gate 2 turns red if the program's or the offsets cell's bytes escape the caller's graph cap or the
  replacement peak.
- Gate 3 turns red if fader automation declines the collapse, which costs a plane per track for no
  audible change.
- Gate 5 turns red if a new three-word cell form can tear or skip.
- Gate 6 turns red if the static composition keeps the member-first order or sums the offsets in
  another order than given; the rewritten line 81 replaces the case that pinned today's order. The
  three rewritten oracles of D6 are not new tests: they keep their gates and now state the order
  that the engine computes.

## Dependencies

- #1626 *Validate stored automation lanes in the session crate and state the hold rule*.
- #1645 *Validate builtin automation targets against their rows at preparation*.
- #1637 *Compile stored automation into per-cell events in node time*.
- #1638 *Apply timed operations inside a block on the strip fader stage*.
- #1639 *Hold the automation jump lengths in a plan cell*: the lengths from which a caller resolves
  the fader row's jump length with `LiveRamps::for_row` (#1637 as shipped, departure 1 of its
  decision record; no program holds a jump-length key).
- *Session controlSmoothing: configurable ramp lengths for live mute, fader and pan changes*
  (#1054): the fader jump length.
- *Keep every node's latency from dropping during playback* (#1285): `a(n)`.
- *Extract the C ABI control plane into a portable crate both hosts call* (#1309): the C ABI
  preparation path both hosts share.
- *Hold live values in latest-target cells on both hosts* (#1312): the cell module this slice adds
  a constructor to, and the merge order of the fader bank processor.
- *Admit browser live edits in the Worker through the committed model* (#1382): every browser live
  edit goes through the shared commit before any host renders stored automation. #1382 brings the
  shared classifier rows of #1225, #1226, #1247, #1261, #1262 and #1390 and the cells of #1345,
  #1346 and #1347, which the later drafts build on.
- Batch: R1, in one push with #1647 *Render moving stored fader automation, seeks and
  latency*, #1648 *Classify fader automation edits as carried rebuilds* and #1649 *Compose VCA offsets
  with stored fader automation*.


## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **The composition moved.** `effective_strip_faders` is gone; `controls::effective` (decision 17
  C7, `crates/controls/src/effective.rs`) composes every effective fader, through
  `session::vca_effective_db` and `session::VcaSum`, which still add the member first
  (`crates/session/src/vca.rs:20-60`). D6 changes `VcaSum` and `vca_effective_db` and adds
  `vca_offsets_sum` and `vca_compose_db` there; every caller follows.
- **The pinned order test moved** to `crates/controls/tests/composition.rs:82-83` (and its
  `reference_db`, `:37`); the browser oracles of D6 were deleted with the browser's own path
  (#1565). The oracles to rewrite are `crates/controls/tests/composition.rs` and
  `crates/host-core/tests/vca.rs:271`. Authorized paths change to these files.
- **The hand offset joins `S`** (OQ1 = C, #1650 D6): `S = offset + Σ reaching VCA offsets`.
  This slice seeds `S` with the stored offset; #1650 adds the field and the write.
- **Live lanes on every plan** (C6): every strip already has its `FaderCells`
  (`crates/builtins-compiler/src/lib.rs:298`); the offsets cell is an extra cell for automated lanes
  only.
- **Moved files.** `crates/capi/src/runtime/compile.rs` is now `crates/control-plane/src/compile.rs`.
  #1312, #1054 and #1309 have landed; #1285 is open.
- **The browser.** #1382 is closed; its role is #1562. #1562a (the browser commits through
  `SessionState`) has landed; #1562b (the browser's rebuild path) is a dependency of batch R1,
  because every fader automation edit is a rebuild after #1648.
- **Must-land group** is #1646, #1647, #1648, #1649 and #1650.

## Amendment (controls coordinator, batch 17 spec check, 2026-10-16)

Checked against `31fa33fee`.

**Anchors moved.**

- Inert-table text: `docs/SESSION_SCHEMA_V1.md:286-291` (was `:220-225`). The builtin target list
  left `validate.rs` (#1566); the fader row is `BUILTIN_PARAMETER_DESCRIPTORS` row `id: 5`
  (`crates/builtins/src/lib.rs:488`, `:596-598`), and the row rule is
  `controls::automation_diagnostics` (`crates/controls/src/automation.rs:89`).
- Composition: `session::vca_effective_db` and `VcaSum` (`crates/session/src/vca.rs:26-67`,
  member first); the one caller that composes is `controls::effective_strip`
  (`crates/controls/src/effective.rs:311-334`); preparation takes `controls::effective` output
  (`crates/host-core/src/prepare.rs:2756`). `pub use` is `crates/session/src/lib.rs:38` (not `:31`).
  The schema sentence of D6 is `docs/SESSION_SCHEMA_V1.md:83-84` (not `:80`).
- Order test: `crates/controls/tests/composition.rs:82-83`, its `reference_db` `:37-45`, its rustdoc
  `:47-52`. Host oracle: `crates/host-core/tests/vca.rs:270-279`. The two browser oracles of D6 are
  gone (#1565).
- `checked_fader_gain` `crates/builtins/src/lib.rs:5298`, `db_gain` `:6430`.
- `FaderBankProcessor` `crates/builtins-compiler/src/lib.rs:1519-1700`; fused
  `FaderMatrixBankProcessor` `:1831` (`process` `:2334`); `FaderCells` `:298`, `FaderLane` `:266`;
  `SEAM_SIDE_WITNESS` `:1141`; `track_mono_source` `:5184`; bank bytes
  `graph_builtin_bank_resource` `:3593` and `strip_processor_bytes` `:2480`.
- `TrackStage::PostFader` `crates/graph/src/lib.rs:258-266`; `checked_add_builtin_banks` `:666-698`.
  Host admission `host.graph.resource.limit` `crates/host-core/src/prepare.rs:2939`. Replacement
  admission `crates/control-plane/src/compile.rs:824` onward.
- `CanonicalFpEnv` `crates/lane/src/fpenv.rs:432` (wasm form `:527`); render enters it at
  `crates/host-core/src/render_session.rs:121`.

**Landed deps and what they give.**

- #1637: `automation::{ProgramBuilder, CellDescription, CellState, CellEvents, NodeSpan,
  JumpLength::Block}`. `ProgramBuilder::bytes` is the byte report. No program field holds `a(n)`.
  `CellState::new` has no target and no setter exists.
- #1638: `BuiltinFaderBank::apply_fader_operation(lane, channels, FaderOperation::SetGain)`
  (`crates/builtins/src/lib.rs:4591`), and the same on `FaderMuteRampBuiltins` (`:5198`).
- #1639: `GraphJumpLengths` and its reader (`crates/graph/src/lib.rs:1234-1430`), charged by
  `graph::jump_length_retained_bytes()` in `prepare.rs`.
- #1312: `LatestCell<const N>` is generic. A three-word cell already exists in production: the
  jump-length cell is `LatestCell<3>`. Deliverable 2 and gate 5 fall away; no file under
  `crates/engine/src/realtime/` is edited.
- #1285: floored arrivals are `GraphCompileReport::input_arrivals`, known only after
  `compile_with_builtins` (`prepare.rs:2810`), which runs after the builtins are prepared.
- #1562a/b, #1309: both hosts prepare through host-core's one backend (the C ABI through
  `control-plane`; the browser calls four `prepare_host_runtime_*` forms, `hosts/host-web/src/lib.rs:5575-5655`).
- #1650 has not landed: no hand-offset field exists yet.

**Departures from the filed text.**

1. **`a(n)` is not this slice's.** The builder has no `a(n)` field, and D2 proves the prepared value
   is the first `start_value` for every `a(n)`. So this slice needs no `a(n)`. It moves to #1647
   (ruling Q1 below).
2. **The scalar track is the test-only oracle.** Shipped builds bank every strip; the per-node
   `LiveControlFaderProcessor` exists only under `test-support` (#1059, `lib.rs:3704-3723`). See
   ruling Q2.
3. **Cell bytes.** `LatestCell<3>` is not 36 bytes: each slot also holds a `u64` key and a `u64`
   sequence. Charge `size_of::<LatestCell<3>>()` (96 bytes, measured on x86-64) plus its
   writer, reader and dirty share, as measured, never a literal. The README F16 figure is wrong in
   the same way; it is still a fixed size per lane, so F16's decision stands.
4. **Charge every retained byte.** D4 also charges each cell's `CellState` (32 bytes) and the per-lane
   automation slots on the fader processor, not only the program and the offsets cell.
5. **Seed, not write.** A #1312 cell cannot be written without a gate stamp. Seed `S` as #1639 does:
   the render reader holds the prepared words until its first applied write. This is not a "static
   copy in the program"; the cell is still the one source after the first write.
6. **Thick core (decision 17 C1, C7).** `S` per lane is a derived value: `controls::effective`
   returns it (a new `EffectiveStrip` field, from `vca_offsets_sum`), and preparation reads it from
   the `Effective` argument; no other crate computes it. The fader row's cell parameters (grid 64,
   grid ramp 64, `End`, `JumpLength::Block`, `SeekMode::Exact`) and the segment conversion
   (`stored_segment`, `crates/controls/src/automation.rs:282`, now private) are written once in
   `controls`, so #1651-#1667 reuse them. Host-core builds the `Program` and passes it to the
   builtins preparation with `&effective`.
7. **Every builtins preparation form installs it.** The four forms in `prepare.rs:2757-2797`
   (selected meters, between render calls, live controls) all install the program, so the result
   does not depend on which form a host calls.
8. **Event state lives in `FaderBankProcessor`** (builtins-compiler), per lane and channel, beside
   the cell readers. #1648 extends `FaderBankProcessor::adopt_lane` (`lib.rs:1557`) to move it;
   `crates/builtins` is not edited.
9. **The seed method lands here.** A fresh `CellState` has no target, so #1647's first grid event
   would always emit. This slice adds the one seeding method to `crates/automation/src/events.rs`
   and seeds each cell with its prepared curve value; #1648 reuses it.
10. **D2's mechanism.** Setting the lane's prepared fader parameter to `vca_compose_db(first value, S)`
    through the existing `checked_fader_gain` path gives the same bits as a `SetGain`; either is D2.
11. **Strips, not tracks.** Submix strips carry the same fader bank; D1-D4 apply to every strip.

**Ruling questions.**

- **Q1. How `a(n)` reaches the fader stage** (decided in #1647; repeated there). Options:
  (A) at the end of `compile_with_builtins` the graph compiler hands each builtin bank its members'
  floored input arrivals through a new defaulted method on `GraphPreparedBuiltinBankProcessor`;
  (B) host-core installs them after the compile through a new artifact method that downcasts the
  fader banks; (C) the graph runtime keeps each lane's arrival and passes a node-time sample per
  lane. **Recommendation: A** (one core rule, no host step, the value sits beside the lane's
  program).
- **Q2. The test-only scalar oracle.** Options: (A) bank only; a `Backend::Scalar` preparation of a
  session with stored fader automation is refused with a typed diagnostic; (B) install the program
  on the oracle too, with one event driver shared by both processors, as #1638 gave both its
  operations. **Recommendation: B**: it keeps the oracle a full reference for "a placement change
  moves no bit" (about 1 h more).

**Authorized paths, as they now must read.**

- `crates/session/src/vca.rs`, `crates/session/src/lib.rs` (`:38` only),
  `docs/SESSION_SCHEMA_V1.md` (`:83-84` only)
- `crates/controls/src/effective.rs` (`S` per lane, `VcaSum` order), `crates/controls/src/automation.rs`
  and `crates/controls/src/lib.rs` (the fader row's cell descriptions), `crates/controls/tests/composition.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/vca.rs` (`reference_db` only),
  `crates/host-core/tests/stored_fader_automation.rs` (new), `crates/host-core/Cargo.toml`
  (`automation`), `crates/builtins-compiler/Cargo.toml` (`automation`), `Cargo.lock`
- `crates/builtins-compiler/src/lib.rs` (program, event state, offsets cell and its writer handle,
  seed, prepared value, bank bytes; the oracle per Q2)
- `crates/automation/src/events.rs` (the seed method and its test)
- `crates/capi/src/runtime/tests.rs`, `hosts/host-web/src/tests.rs` (gate 1)
- No path under `crates/engine/src/realtime/`. `crates/engine/src/realtime/latest_cell.rs` is
  removed from the list.

**Gates' test files.** Gate 1: `crates/capi/src/runtime/tests.rs`, `hosts/host-web/src/tests.rs`.
Gates 2-3: `crates/host-core/tests/stored_fader_automation.rs`. Gate 5: dropped (no new cell form).
Gate 6: `crates/controls/tests/composition.rs`. Gate 7 adds `cargo test --locked -p controls` and
`-p automation`, and drops the loom command.

**Size.** About 9 implementer hours. Split into the smallest closable slices, in this order:
**#1646a** D6 only (session, controls effective and its test, host-core oracle, schema sentence;
about 2 h, closable alone: class B, ruled) and **#1646b** the preparation (about 7 h). Both stay in
batch R1.


## Decision record: #1646a (batch 17)

D6 only. Shipped:

- `crates/session/src/vca.rs`: `vca_offsets_sum(offsets) -> Option<f64>` (first offset starts the
  sum, no `+0.0` seed) and `vca_compose_db(member, sum) -> f32`
  (`(f64(member) + sum).clamp(-144, 24) as f32`); `vca_effective_db` is written from them (member
  bit for bit with no offset). `pub use` in `crates/session/src/lib.rs`.
- Departure in shape, not in arithmetic: `VcaSum` no longer takes the member. `VcaSum::new()` is an
  empty offsets accumulator with `add`, `sum() -> Option<f64>` and `compose(member) -> f32`
  (replacing `finish`), because the member now joins after the offsets. Its only caller is
  `controls::effective_strip` (`crates/controls/src/effective.rs`), which now builds
  `[VcaSum::new(); 2]` and calls `compose(own_db[lane])`. No per-lane `S` field (that is #1646b).
- `docs/SESSION_SCHEMA_V1.md:83-84` states the new order.
- Gate 6 is `vca_effective_db_sums_in_f64_in_order_and_clamps_once`
  (`crates/controls/tests/composition.rs`), rewritten in place: reference sums offsets first, also
  checks `vca_offsets_sum` + `vca_compose_db` against it, crafted cases
  `(24, [-24, 1e-30]) == 0.0`, `(1e-30, [24, -24]) == 1e-30`, `(0, [24, -24, 1e-30]) == 1e-30`
  (plus `(0, [1e-30, 24, -24]) == 0.0` for the offset order and a `-0.0` lone-offset case for the
  no-seed rule).
- Host oracle `reference_db` in `crates/host-core/tests/vca.rs` rewritten to the new order. The
  other callers (`crates/capi/src/runtime/live_tests.rs:7040,7082`) call `vca_effective_db`
  itself, so they follow without edits. The two browser oracles of D6 no longer exist (#1565).
- Measured: `./target/release/audit capi` pcm_digest `bffc21e857b5e134` (unchanged from base),
  0 violations. No pinned render or digest moved; `-p session`, `-p controls`,
  `-p host-core --features host-core/test-support`, `-p capi`, `-p host-web --features
  host-web/test-support` all pass; clippy (`--all-features`) and no-feature `cargo check` clean.

## Rulings (root, batch 17, 2026-10-16)

- **Q1 (R17-2): A.** `a(n)` belongs to #1647. At the end of `compile_with_builtins` the graph
  compiler gives each builtin bank its members' floored input arrivals through a new defaulted
  processor method. No downcast in host-core. This slice needs no `a(n)` (departure 1).
- **Q2 (R17-4): B.** The program is installed on the test-only scalar oracle too, through the one
  shared event driver. The scalar backend is not refused.

## Decision record: #1646b (batch 17)

The preparation (D1-D5). Shipped:

- **Thick core (`controls`).** `EffectiveStrip::offsets_sum` is `S` per lane
  (`vca_offsets_sum` of the reach, `+0.0` with none), written by `effective_strip` beside
  `fader_db`; #1649 departure 3 only reads it. `crates/controls/src/automation.rs` gains
  `CellRule` and `FADER_CELL_RULE` (grid 64, grid ramp 64, `End`, `JumpLength::Block`, restartable,
  `SeekMode::Exact`), `stored_segments` (the one segment conversion, now also read by the order
  rule), `automation_row_renders` (today the fader row; #1648's classifier reuses it) and
  `fader_automation(model, strip) -> [Option<&Automation>; 2]`.
- **Seed (`automation`).** `CellState::seed_target(value)` sets the target as a completed retarget
  (clears `held`, keeps the cursor and the ramp); #1648 reuses it. Test
  `events::tests::a_seeded_target_suppresses_the_first_retarget_to_the_same_bits`.
- **Preparation (`builtins-compiler`).** The shared private body of every preparation form calls
  `prepare_fader_automation` after the preflight and before phase two: one cell per automated
  fader lane of every strip (tracks and submixes; `both` gives two), in strip then channel order,
  built once into one `Arc<Program>`. Each lane's prepared value is its first `start_value` (D2,
  the hold rule) composed as `vca_compose_db(value, S)`; the composition and its
  `checked_fader_gain` run inside `CanonicalFpEnv`. That dB is the lane's prepared fader parameter
  (departure 10), so the existing `checked_fader_gain` path bakes the static bits.
- **Install.** At lowering each fader bank with an automated member gets one
  `Option<LaneFaderAutomation>` slot per bank lane (`FaderBankProcessor::automation`, empty and
  unallocated otherwise); a slot holds the program handle and per channel an `AutomatedFaderCell`:
  cell index, `CellState` seeded with the prepared value, its own offsets cell (`Arc` of a
  `LatestCell<3>` and its dirty word, created unwritten) and the held `S` and ramp word (the
  session's fader length, `LiveRampRow::VcaOffset`). The test-only scalar oracle (R17-4) holds the
  same `LaneFaderAutomation`, boxed, on both its fader owners (`FaderProcessor`,
  `LiveControlFaderProcessor`); the scalar backend is not refused. The fused fader-matrix bank
  holds the fader bank, so it carries the slots.
- **Bytes (D4).** `strip_bank_resource` (the one sum the preflight `graph_builtin_bank_resource`
  and the lowering both take) adds `fader_automation_bank_resource`: the program once
  (`size_of::<SharedAllocation<Program>>()` plus `ProgramBuilder::bytes`), `width` slots per bank
  with an automated member and one offsets cell (`size_of::<SharedAllocation<OffsetsLane>>()`)
  per automated channel, as builtin bank payload, so they reach `builtin_bank_bytes`,
  `graph_incremental_plan_bytes`, `graph_session_plus_plan_bytes`, the host's cap and the C ABI's
  replacement peak. The oracle's are charged in `graph_scalar_owner_resource`. The sizes are
  `builtins_compiler::fader_automation_sizes()`, measured, never literals.

**Measured (x86-64, AVX2, eight lanes):** program overhead 64 bytes, offsets cell 120, lane slot
120 (`Option<LaneFaderAutomation>`: program handle and two channels with their 32-byte
`CellState`). One left-only lane with one segment: program report 68, charge
`68 + 64 + 8 × 120 + 120 = 1,212` bytes (`builtin_bank_bytes` 8,472 -> 9,684 on the bank
fixture). Each further automated channel in the same bank adds 120 plus its cells' program bytes.
A session without fader automation carries the empty slot array: `size_of::<FaderBankProcessor>()`
grows by 16 bytes per fader bank, a fixed structural change inside every ceiling (no re-pin).

*Re-measured at the #1647 U2 fold (batch 17), same host:* #1647a-c, #1648b and #1649 grew the slot
(240 bytes at `275ff76e8`) and the U2 fold removed #1647b's two unread render counters: lane slot
224, program overhead 64, offsets cell 120. The one left-only lane's charge is
`68 + 64 + 8 × 224 + 120 = 2,044` bytes (`builtin_bank_bytes` 8,472 -> 10,516 on the bank
fixture); each further automated channel in the same bank adds 120 plus its cells' program bytes.

**Departures.**

1. **The program is built in the builtins preparation, not in host-core** (departure 6 said
   host-core builds it and passes it). The shared body of all five public forms builds it from
   the session and the `Effective` it already takes, so every form installs it with no new
   argument, no caller can omit it (departure 7's aim), the oracle and the graph compiler's
   preflight read the same plan, and no public signature changes. The row rule, the segment
   conversion and `S` stay written once in `controls`; host-core has no edit.
2. **`host.automation.fader` is raised by that body**, so it reaches a host as a builtin
   rejection, at `$.automation[id=<id>]`. It also covers a builder refusal: a session may hold a
   segment end at or past `2^53` samples, which the session validator accepts and
   `ProgramBuilder` refuses (`BuildError::Segment`), so that refusal is reachable, not only the
   unreachable conversion of D2.
   *Batch 17 follow-up:* the session validator now refuses a segment `start_sample` or
   `end_sample` at or past `automation::TIME_LIMIT` (`2^53`) with `numeric.out_of_schema_range`, so a
   validated session no longer reaches this builder refusal; it stays as a defensive check.
   Gate: `the_validator_and_the_program_builder_agree_on_the_segment_time_bound`
   (`crates/session/tests/diagnostic_parity.rs`).
3. **No offsets-cell writer here.** #1649 departure 4 adds the writer to `FaderCells`; the cell is
   created unwritten and its holder keeps the prepared words.
4. **Parts with no reader until the same push** carry `#[expect(dead_code, reason = ...)]`, which
   turns into a warning (and so must go) as soon as the successor reads them: the program, the
   seeded `CellState`, the held `S` and ramp word (render: #1647), the offsets cell (write and
   dirty read: #1649), the slot carry in `adopt_lane` (#1648). Today the prepared value alone
   renders: a flat curve exactly, a moving curve held at its first value.
5. **The oracle's plain `FaderProcessor`** (a strip with no live-control request) holds the
   automation but wraps the unramped fader, which takes no timed operation; #1647 binds an
   automated oracle strip on the ramped fader.
6. `a(n)` is #1647's (R17-2); nothing here reads it.

**Gates.**

- Gate 1: `a_flat_fader_automation_renders_the_static_fader_bits_on_the_c_abi`
  (`crates/capi/src/runtime/tests.rs`) and
  `a_flat_fader_automation_renders_the_static_fader_bits_in_the_browser`
  (`hosts/host-web/src/tests.rs`, live-controlled boot): no VCA, one `+3` dB VCA, the crafted three
  VCAs (asserting the two orders give dB bits `0x40ad_8ad3` and `0x40ad_8ad2` and different gain
  bits) and `-0.0` dB on a lane neither of two VCAs reaches, 64 blocks each, and each automated
  session differs from itself without the automation.
- Gate 2: `a_fader_automation_is_charged_to_the_graph_plan_rows_and_the_cap`
  (`crates/host-core/tests/stored_fader_automation.rs`): the three rows differ by exactly the
  charge above; the admitted total prepares and one byte below refuses with
  `host.graph.resource.limit`.
- Gate 3: `a_left_only_fader_automation_keeps_a_mono_tracks_collapse` (same file): the collapse
  counters equal the session without automation, and the output equals the twin's dual render.
- Gate 4: `./target/release/audit capi` `pcm_digest` `bffc21e857b5e134` (unchanged), 0
  violations, 0 allocations; every existing render test of `-p session`, `-p controls`,
  `-p automation`, `-p host-core`, `-p builtins-compiler`, `-p capi`, `-p host-web` passes with
  no digest moved. One layout test moved mechanically:
  `scalar_owner_resource_is_independent_two_boxes_plus_two_pointer_outer` now counts the oracle
  fader owner's new `automation` field. Gate 6 landed with #1646a. `S` in
  `vca_reach_counts_a_diamond_once_and_includes_nested_parents_in_id_order`
  (`crates/controls/tests/composition.rs`) now also pins `offsets_sum`, `+0.0` by bits.

## U1 fold (batch 17)

Folds the U1 review of #1646b.

- **D2, canonical environment (MAJOR 1).** The builtins preparation body
  (`prepare_session_builtins_with_live_controls_and_policy`) holds `lane::CanonicalFpEnv` for its
  whole run, and both lowerings (`into_graph_artifact_with_banks`, the oracle's
  `into_graph_artifact`) hold their own, so every strip's fader gain and input coefficients,
  automated or static, are baked in the canonical environment. host-core's
  `prepare_host_runtime_with_live_controls_policy_and_spectrum` computes `controls::effective` (the static VCA composition) inside
  the guard too, so a flat automated lane and a static fader compose and convert alike by
  construction. An automated lane's composed dB is computed once in `prepare_fader_automation`,
  stored on its `PreparedFaderCell` and read by `FaderAutomationPlan::fader_db`; the discarded
  `black_box` conversion is gone. Its domain check stays as a defensive typed refusal.
  Gate: `canonical_preparation::a_hostile_caller_environment_prepares_the_canonical_faders`
  (`crates/host-core/tests/stored_fader_automation.rs`, x86-64 and AArch64): a caller word with
  FTZ, DAZ and round-toward-zero prepares the plan a canonical thread prepares, bit for bit over 16
  blocks; the fixture asserts its own conversions and one VCA composition round differently under
  that word. Red, checked once, with any one of the three guards removed (body, bank lowering,
  host-core effective).
- **`host.automation.fader` is defensive (MINOR 3, follow-up of departure 2).** A validated
  session reaches none of its causes: the validator refuses an empty segment list, a `start_value`
  outside the fader domain (`vca_compose_db` then clamps), and every segment the builder refuses.
  Only `TooLarge` and `Allocation` (at `$.automation`) can reach a validated session. The code
  comments say so; no test reaches the per-entry refusal, because no validated session can. The
  host diagnostics are not listed in `docs/SESSION_SCHEMA_V1.md`, so no doc row is added.
- **Scalar oracle charge (MINOR 1, R17-4).** Gate:
  `tests::the_scalar_owner_row_charges_one_automated_fader_lane` (`crates/builtins-compiler`):
  at `Backend::Scalar` the owner row grows by `size_of::<LaneFaderAutomation>()` plus one offsets
  cell plus the program (overhead and report), once.
- **Gate 2 cross-check (MINOR 2).** `a_fader_automation_is_charged_to_the_graph_plan_rows_and_the_cap`
  now also measures the heap with `bench_support::alloc`'s thread-scoped counters: the automated
  prepared host holds exactly the charge (1,212 bytes on x86-64 AVX2) more than the plain one.
- **Departure (NIT 1).** `FaderBankProcessor::automation` is a `Box<[Option<LaneFaderAutomation>]>`
  on every fader bank: `size_of` 16 bytes (x86-64), so `builtin_bank_bytes` grows by 16 per fader
  bank for every session, with or without automation. Kept: an empty boxed slice does not allocate,
  and one owner shape for every bank keeps the carry (#1648) and the render path (#1647) without a
  second type.
- `./target/release/audit capi`: `pcm_digest` `bffc21e857b5e134` (unchanged), 0 violations, 0
  allocations.
