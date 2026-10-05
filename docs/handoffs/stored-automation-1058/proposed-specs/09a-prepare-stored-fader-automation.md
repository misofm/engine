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
renders exactly as before. Moving curves, seeks and latency in render are draft 09b, in the same
batch and push.

This slice lands after *Admit browser live edits in the Worker through the committed model*
(#1382). From then on every browser live edit goes through the shared commit, so the browser and
the C ABI meet one rule for a live edit of an automated lane (draft 10), and no host-only
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
- **What this slice builds on.** Drafts 01 and 02 (one entry per lane, the hold rule, the fader's
  unit and domain), 07 (the builder, its byte report and the jump-length key), 08 (`SetGain`), 12
  (the plan cell that holds the jump lengths, seeded from #1054's
  `control_smoothing_samples()`). #1285 D2 records each node's floored input arrival `a(n)`. #1312
  gives the latest-target cell (`crates/engine/src/realtime/latest_cell.rs`, #1312 D1); its cells hold two words (fader,
  mute) or five (matrix) (#1312 D3), and this slice's offsets cell holds three (D1).
- **Root's decision F16** (README, "Findings for the coordinator"): the VCA offsets add first,
  then the member (D6), under the standing summation-order ruling as root reads it.

## Decisions frozen for this slice

- **D1. Cells.** Host-core preparation compiles one cell per automated fader lane (target rack
  `builtins`, parameter 5; `both` gives two cells) with draft 07's builder:
  - `a(n)` is the floored input arrival of the strip's `PostFader` node (#1285 D2);
  - grid period 64, grid ramp 64, completion `End`; jump-length key `Word(Fader)` (draft 07 D2):
    each jump reads the `fader` word of draft 12's plan cell for its block, so the program stores
    no jump length; no `no_restart`;
  - **the offsets cell** (layout 4 of README finding F16). Preparation builds
    draft 11's offsets cell for every automated fader lane, in every session, with or without
    VCAs: a #1312 latest-target cell of three words, the lane's offset sum `S` as one `f64` (two
    `u32` words) and one `u32` ramp word that draft 11 writes (the length of the jump a VCA edit
    asks for; seeded with the session's fader length, #1054). `S` is
    `vca_offsets_sum(reach offsets)` (D6) for the lane's channel, and `+0.0` when no VCA reaches the
    lane. A value `v` becomes the gain `checked_fader_gain(vca_compose_db(v, S))` (D6), with `S`
    from the cell's newest slot. No static copy of `S` is kept in the program, so one source holds
    it; draft 11 adds only the live VCA edit that rewrites the cell. Because the cell exists on
    every automated fader lane, a VCA that a later edit adds or attaches only rewrites it.
  - **Why the cell keeps the bits.** With a reach, D6 makes the static path compute
    `vca_compose_db(member, S)` with the same `S`, so the bits are equal by construction. With no
    reach, the static path returns `v` unchanged, and render computes `clamp(f64(v) + (+0.0))`. In
    `f64`, `v + (+0.0) == v` for every `v` except `-0.0`, which becomes `+0.0`; `v` lies in
    `[-144, 24]` (draft 02), so the clamp changes nothing. `db_gain(+0.0)` and `db_gain(-0.0)` are
    both exactly 1 (`math::pow(10, ±0) = 1`), so the gain has the static bits in every case. Draft
    07's "change only" compares the curve value, before the offsets, and draft 11 D3 compares the
    composed gain, never the dB sum, so a sign of zero never causes a retarget.
  - **Where the event state lives.** Each cell's event state (draft 07's `CellState`) lives in the
    fader bank stage beside the lane's ramp state, keyed by the cell's stable address (strip, row
    5, channel). So the stage's carry program (*Carry fader, mute and pan ramps across a plan
    swap*, #1277) moves it with the ramp state at a swap, and no second carry path exists. Draft 10
    relies on that carry.
  The program is installed on the strip's fader bank processor (or the scalar track's), as
  immutable plan data.
- **D2. Prepared value.** After the graph compile and before publication, each automated lane is
  set exactly (draft 08 `SetGain`, off the render thread) to the curve at node time `-a(n)`,
  composed as in D1. By the hold rule the curve at any `t <= first.start_sample` is the first
  `start_value`, and every `start_sample >= 0`, so `curve(-a(n)) == curve(0)` always: the prepared
  value is the entry's first `start_value`, whatever `a(n)` is (README A1.4, "New lanes"). A lane
  whose curve is flat at `v` is therefore set to the bits a static `v` prepares. Every curve
  evaluation and conversion of preparation runs inside `CanonicalFpEnv`, as render's do (README
  A1.1). A conversion that fails is unreachable (draft 02
  bounds the curve to the fader domain and `vca_compose_db` clamps, D6) and refuses preparation with
  `host.automation.fader`.
- **D3. Mono collapse is kept.** The fader is seam-side (Context), so a fader automation that drives
  one lane, or both lanes with different curves, keeps the track's collapse. This refines the
  README's mono-collapse rule (A1.5), which holds for rows upstream of the seam: trim, polarity,
  HPF, LPF and strip effects (drafts 15, 16a-16b, 18a-18b). Nothing is added to
  `track_mono_source`.
- **D4. Bytes.** Each fader program's bytes (builder report, draft 07 D5) and each offsets cell's
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
- `crates/engine/src/realtime/latest_cell.rs` (a three-word cell form and its loom case, only if
  #1312's module has none)
- `crates/session/src/vca.rs` (D6), `crates/session/src/lib.rs` (the `pub use` at `:31` only),
  `crates/session/tests/vca_composition.rs` (gate 1's rewrite, the order cases and the reference),
  `docs/SESSION_SCHEMA_V1.md` (the composition sentence at `:80` only)
- Tests that pin a builtin bank or graph plan row that D4 moves for a session with automation (none
  exists today; a session without automation moves no row)
- `crates/host-core/Cargo.toml` (a normal `automation` dependency: preparation runs draft 07's
  builder), `Cargo.lock`

## Non-goals

- Events in render: the grid, jumps, held events, seeks, the timed chain call (draft 09b).
- The documents that say the fader row renders (draft 09b).
- Classifying fader automation edits, the rule for a live edit of an automated lane, and carrying
  event state across a swap (draft 10).
- The live VCA move that rewrites the offsets cell (draft 11). The plan cell of jump lengths and
  its edit path (draft 12).
- Mute, pan, matrix, input and effect rows (drafts 13a to 20).

## Hazards

- **Batch R1, one push with drafts 09b, 10 and 11** (README "Must-land-together groups"). Alone,
  this slice starts a moving curve at its prepared value and holds it there; draft 09b renders the
  motion, draft 10 stops a live fader edit from fighting the curve, and draft 11 makes a VCA ride
  heard on an automated member. The four merge in one push, so `main` never holds a partial state.
- **Host-core is stream A's file** (`crates/host-core/src/prepare.rs`); root orders the merge after
  #1285 and #1312.
- **Bit-identity with VCAs.** D6's one rule serves the static path and the cell, so `S` in the
  cell is the static path's own sum; draft 11's live edit computes it with the same function, so
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

- Draft 01 *Validate stored automation lanes in the session crate and state the hold rule*.
- Draft 02 *Validate builtin automation targets against their rows at preparation*.
- Draft 07 *Compile stored automation into per-cell events in node time*.
- Draft 08 *Apply timed operations inside a block on the strip fader stage*.
- Draft 12 *Hold the automation jump lengths in a plan cell*: the `fader` word the jump-length key
  reads.
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
- Batch: R1, in one push with drafts 09b *Render moving stored fader automation, seeks and
  latency*, 10 *Classify fader automation edits as carried rebuilds* and 11 *Compose VCA offsets
  with stored fader automation*.

