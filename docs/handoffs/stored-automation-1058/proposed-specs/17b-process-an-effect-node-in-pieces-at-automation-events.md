# Process an effect node in pieces at automation events

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.6), A5 and A7, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`. Batch R3.

## Product outcome

An effect node, per node or in a bank, whose stored automation has an event inside a block renders
that block in pieces. Each piece ends at the next event sample. At each piece start, every cell
with an event there stages one `Point` at that sample, the only span shape the contract accepts.
The pieces render the same bits as a render whose every event sample is a block start, which
draft 17a proves for every launch effect. A node with no event in the block renders exactly as
today: one call. This is the building block that lets draft 18a drive effect parameters at
sample-exact, quantum-independent times. It merges in the batch of its first user, draft 18a
(batch R3).

## Context

**The contract.**
- A `Block` parameter accepts only a `Point` at the call's `first_sample`
  (`crates/effect-contract/src/lib.rs:1496-1499`). A call may be shorter than the quantum
  (`EffectProcessBlock::new`, `:1299`; `validate_automation_block`, `:1473`).
- A bank block holds exactly `frames x lanes` words per plane (`EffectBankProcessBlock::new`,
  `:1359-1397`). The planes are AoSoA, so frames `[a, b)` of every lane are the contiguous words
  `[a·W, b·W)`: a piece needs no copy.
- Draft 17a proves that a `Point` at a mid-quantum call start renders the bits of 1-frame blocks,
  per node and in banks, for every launch effect.

**The per-node stage** (`crates/graph/src/runtime.rs`).
- `LiveControlEffect::new` gives a window of `automation_capacity` spans only when the lane has a
  live channel, and none otherwise (`:1206-1239`).
- `execute_op` renders a plain effect node with `&[]` in one call (`:3478-3509`). A live-control
  node drains its lane at block entry (`:3517-3525`), applies prepared targets (`:3539-3545`),
  captures the shunt, makes one `process` call, restores bypassed output and publishes
  observations after the block (`:3546-3598`).
- The binder picks the node kind: `NodeKind::Effect` when no control lane exists,
  `LiveControlEffect` otherwise (`:4636-4648`).

**The bank stage** (`crates/rack/src/lib.rs`).
- `EffectBankStage::process` makes one `process_bank` call with `&[]` (`:867-883`).
- `LiveControlEffectBankStage::new` sizes the staging and packed windows from
  `automation_capacity` only when a lane has a live channel (`:1014-1029`, `:1054-1070`).
- `drain` stages every lane at `begin_block` (`:1257-1284`). `process_inner` applies targets,
  captures the shunt, makes one `process_bank` (or `process_bank_mono`) call, then publishes
  observations (`:1294-1464`).
- The graph builds the live-control stage only when some lane has a control lane
  (`crates/graph/src/runtime.rs:4800-4848`).
- `scripts/check-rack-policy.sh:23` pins rack's dependencies to exactly `effect-contract` and
  `engine`.

**Silence.** A non-empty span slice clears an effect's silent claim
(`crates/compressor/src/lib.rs:1050-1053`; `crates/parametric-eq/src/lib.rs:3515-3517`), and a
ramp in flight blocks the skip (`crates/compressor/src/lib.rs:568`;
`crates/parametric-eq/src/lib.rs:2936-2939`).

## Decisions frozen for this slice

- **D1. Pieces.** For a block `[r0, r0 + q)` of a node or bank slot with a stored program, the
  stage asks each cell's event generator (draft 07) for the block's events.
  - Piece starts are offset 0 and every event offset. A piece ends at the next start or at `q`.
  - A bank's piece starts are the union over its lanes: at most `1 + lanes·(2⌈q/64⌉ + 3)` pieces,
    and at most `q`.
  - Piece 0's window holds the live spans drained at block entry, plus one `Point` for each cell
    with an event at offset 0. Piece `k > 0` holds one `Point` for each cell with an event at its
    start.
  - A `Point` has the cell's channel, `start_sample = end_sample = r0 + offset` and the event's
    target value. Each piece call passes `r0 + offset` as its `first_sample`.
  - One `process`, `process_bank` or `process_bank_mono` call per piece, over that piece's frames
    and the matching AoSoA words.
- **D2. Canonical order and no buffer.** Within a piece every span has one start sample, so the
  order is `(parameter_index, channel)`. Live spans and stored `Point`s are merged in place in the
  existing window. The stage pulls events lazily, in offset order, from per-cell cursors. It keeps
  no per-block event buffer and allocates nothing.
- **D3. What stays per block.** The live drain at `begin_block`, the prepared-target application
  before piece 0, the bypass decision, the shunt capture before piece 0 and its restore after the
  last piece, and the observation publish after the last piece (with the whole block's window) are
  unchanged.
- **D4. No silent skip on an evented piece.** A piece with a staged `Point` takes the effect's
  normal path, by the rules in Context. This slice adds no skip and removes none.
- **D5. One span per cell per piece.** An automated cell takes no live record (A10), so a piece
  stages each cell at most once. With #1306's capacity (draft 18a) the window always holds
  `live + stored` spans, and `Staged.dropped` stays 0. A live span that addresses a cell the
  stage automates is an admission defect: a `debug_assert`; in release the stored `Point` is
  staged, the live span is skipped and a saturating `shadowed_live_spans` counter on the stage
  counts it, read after render.
- **D6. Which stage types.**
  - A node or slot with a stored program uses `LiveControlEffect` or `LiveControlEffectBankStage`.
    Where no live controls are attached, its lane is `EffectControlLane::without_channel`, as a
    lowered bypass already is (`crates/effect-contract/src/live.rs:173`).
  - The constructors take the stored program as one more argument, `None` for a node with no
    stored automation.
  - A node or slot with no stored program keeps today's types, windows and calls byte for byte.
  - The rule "only a live channel has a window" (`crates/graph/src/runtime.rs:1206-1218`,
    `crates/rack/src/lib.rs:1014-1029`) becomes "a live channel or a stored program". The capacity
    itself is draft 18a's.
- **D7. Symmetry.**
  - A `both` entry on a `PerLane` parameter is two cells with one curve and one `a(n)`. They emit
    equal events at equal offsets, and the stage stages them in one piece as a twin pair
    `Left v`, `Right v`. That is `SymmetryEvent::Preserve`
    (`crates/effect-contract/src/symmetry.rs:135-141`), so collapse survives it.
  - A lane whose program has a one-channel cell, or two channel cells with different curves,
    reports the `LIVE` term cleared in `lane_symmetry` (`crates/rack/src/lib.rs:1187-1193`, and the
    per-node witness) for the plan's life. A collapsed slot renders each piece with
    `process_bank_mono`.
  - This is the render-side form of draft 18a D7's preparation rule: a live one-channel record or
    an asymmetric program clears `LIVE` at render, and draft 18a D7 declines the collapse at
    preparation for the same asymmetric program.
- **D8. Node time.** Node time reaches the stage as draft 07's `NodeSpan`. `graph` computes it for
  each lane with draft 04's reader, `timeline_block(s0 - a(n))`, from the block's source-read sample
  `s0` and the lane's arrival `a(n)`, as draft 09b D1 does for the fader stage, and passes it into
  the rack stage. The rack never reads draft 04's timeline history itself. This slice adds no
  second node-time path.
- **D9. The rack's dependency boundary.** `rack` gains the `automation` crate (draft 07), as A1.1
  names. `scripts/check-rack-policy.sh:23` and its fixture in `scripts/test-rack-policy.sh:14`
  change to the three crates, with the reason in a comment.

## Deliverables

1. D1-D8 in `crates/rack/src/lib.rs` (dual and mono bodies) and in the effect arms and node
   construction of `crates/graph/src/runtime.rs`.
2. D9's dependency and policy line.
3. The tests below.

## Authorized paths

- `crates/rack/src/lib.rs`, `crates/rack/Cargo.toml`, `crates/rack/tests/`.
- `crates/graph/src/runtime.rs` (effect node kinds, their construction and the effect arms of
  `execute_op` only), `crates/graph/Cargo.toml`, `crates/graph/tests/` (one new file).
- `scripts/check-rack-policy.sh`, `scripts/test-rack-policy.sh` (D9 only).

## Non-goals

- The effect-level proof (draft 17a).
- Compiling cells from the session, `a(n)`, window sizing (draft 18a); the seek step (draft 18b).
- The EQ's render-side design (draft 20). Classifier and carry (draft 19).
- Any change to an effect crate.
- A silence skip that advances ramp state: finding F10's successor.

## Hazards

- **Hot files.** `crates/graph/src/runtime.rs` and `crates/rack/src/lib.rs` are stream A's files
  (#1279, #1280 edit the same stages). Root sequences the merge.
- **#1345** replaces the lane's queue with cells and changes `stage`. This slice is written on the
  post-#1345 lane: D2's merge reads the window `stage` fills, whatever fills it.
- **The bank witness.** A cached witness that ignores D7's asymmetric program would collapse a
  lane whose channels differ: wrong audio, not lost speed. Gate 2 holds it.
- **Mid-block `first_sample`.** Every effect validates `span.start_sample == first_sample` of the
  call. A piece called with the block's `r0` counts every `Point` invalid.

## Objective gates

1. **The stages** (new tests in `crates/rack/tests/live_control_bank.rs` and in
   `crates/graph/src/runtime.rs`'s tests): a compressor slot at Simd4 and Simd8 and a per-node
   delay, each with a stored program (built with draft 07's builder from neutral cell
   descriptions) whose events fall at offsets 0, 1, 63, 64 and 127 of a 128-frame block, render the
   bits of the same instances driven by hand with the same `Point`s in 1-frame blocks. A slot with
   live spans and stored `Point`s in piece 0 stages them in canonical order. `Staged.dropped` and
   `shadowed_live_spans` are 0.
2. **Symmetry** (new test in `crates/rack/tests/mono_reengage.rs`): a mono-source track with a
   `both` program keeps collapsing and renders the dual run's bits; with a left-only program it
   never collapses.
3. **No events, no change.** A slot or node with a stored program whose block has no event makes
   exactly one process call (counted with a test-support call counter on the stage). A plan with no
   stored program keeps `EffectBankStage` and `NodeKind::Effect`.
4. **Allocation** (new `crates/graph/tests/rt12_effect_pieces_alloc.rs`): 1,000 blocks of a bank
   slot and a per-node effect, each with an event in every block, make 0 allocations and 0 frees,
   counted with `bench_support::alloc::current_thread_delta_since` after one warm block.
5. **No rendered bit moved (PR evidence).** `cargo build --locked --release -p audit -p capi`, then
   `target/release/audit capi` at base and head: all violation counts 0 and the same `pcm_digest`.
   The browser legs of `qualification.yml`'s `browser` job pass with unchanged digests.
6. **Policy and workspace.**
   - `bash scripts/check-rack-policy.sh && bash scripts/test-rack-policy.sh`
   - `for x in realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `cargo test --locked -p rack`, `cargo test --locked -p graph --features test-support`
   - `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 1 turns red if a piece call passes the block's `first_sample`, slices the AoSoA planes
  wrong, stages a `Point` in the wrong piece, or stages live and stored spans out of canonical
  order.
- Gate 2 turns red if an asymmetric stored program leaves the `LIVE` term set (wrong audio under
  collapse) or a `both` program clears it (lost collapse).
- Gate 3 turns red if an event-free block is split, or if a plan with no stored automation takes
  the new stage type.
- Gate 4 turns red if the piece loop allocates or keeps a per-block buffer.

## Dependencies

Batch R3. Direct dependencies:

- Draft 07 *Compile stored automation into per-cell events in node time*: the event generator and
  `NodeSpan`.
- Draft 17a *Prove every launch effect partition-invariant with Point spans*.
- Draft 04 *Give every plan a timeline clock that seeks and carries like a source*: the
  `timeline_block` reader that `graph` calls for D8.

This slice is written on the lane of *Hold effect parameter, bypass and EQ-target values in
latest-target cells* (#1345); D2's merge reads the window `stage` fills, whatever fills it. It
merges in batch R3 with its first user, draft 18a *Compile and bind stored effect parameter
automation*.
