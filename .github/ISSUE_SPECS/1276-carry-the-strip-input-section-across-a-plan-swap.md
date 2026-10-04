# Carry the strip input section across a plan swap

Slice 7 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

A strip whose input section did not change keeps its exact state through a plan swap: the high-pass
and low-pass filter integrators, the trim and polarity ramps, and the section's live-symmetry terms.
A track that plays through an enabled high-pass filter no longer clicks when another track is added.
A live input record admitted before the swap is applied, not lost. A mono-source chain that was
collapsing keeps collapsing after the swap.

## Context

- The input section of every strip renders in a bank: `BuiltinBankProcessor`
  (`crates/builtins-compiler/src/lib.rs:418`, impl `:442`; struct doc `:389-416`) wraps a
  `BuiltinInputBank` (`crates/builtins/src/lib.rs:3390`; kernel state `InputStage` `:908`,
  `InputLane` `:778`, `SvfSection` `:684`) and one optional `Consumer<TrackInputRecord>` per lane. It
  drains them in `BankStage::begin_block`, applying records to lane state.
- The per-node builtin processors are a test-only oracle (`builtins-compiler/src/lib.rs:4012-4019`);
  a shipped build banks every strip and asserts it (`:2401-2414`). They get no carry.
- The graph sees a builtin bank as `GraphPreparedBuiltinBankProcessor`
  (`crates/graph/src/lib.rs:1284`; only `as_any(&self)` and `into_any`, `:1285-1286`) wrapped in
  `BuiltinStage` (`crates/graph/src/runtime.rs:4017`) inside a `BankChain`
  (`crates/rack/src/lib.rs:1858`), whose slots are the private `PreparedSlot { stage: Box<dyn
  BankStage>, .. }` (`:1469`).
- Mono collapse: a chain keeps `collapsed` and `collapse_channels_agree` flags
  (`rack/src/lib.rs:1858-1935`); `disengage_collapse` (`:2624-2647`) desymmetrizes the prefix stages
  and keeps agreement. Every launch effect declines `channels_agree`
  (`crates/effect-contract/src/lib.rs:2200-2216`), so a chain whose flag is cleared cannot prove
  agreement again in that plan.
- Lanes are ordered by strip ID; a track whose ID sorts first shifts every later lane. Padding lanes
  belong to no strip.
- The scaffold (inventory, `SuccessorBase`, join, the test harness and the `test-support` width
  entries) comes from *Prepare a successor plan whose unchanged sources keep playing* (#1272).

## Decisions frozen for this slice

- **D1. Key and rule.** Owner key `(strip ID, PostInputBuiltins)`. It carries when the strip's input
  section (`polarity_invert`, `trim_db`, `hpf_hz`, `lpf_hz` per lane) is bit-equal in the committed
  model before the transaction and in the successor's model, and both plans attach the same control
  kind (a live queue or none). Padding lanes never carry.
- **D2. Lane state.** One fixed-size, plain-data `InputLaneState` (no heap): filter state, the
  coefficients in use and any coefficient ramp in flight, trim and polarity ramp positions and
  targets, the live-symmetry term. `BuiltinInputBank` gains `export_lane(lane) -> InputLaneState` and
  `import_lane(lane, &InputLaneState)`. Import copies every word verbatim and re-derives only caches
  (settled flags), so the lane is self-consistent and bit-exact.
- **D3. Access.** Add `as_any_mut` to `GraphPreparedBuiltinBankProcessor`, and a rack accessor that
  lends a `BankChain` slot's stage mutably at bind and at the carry, so the carry can reach a
  predecessor and a successor processor of the same concrete type.
- **D4. Drain, then carry.** Before exporting any lane of a predecessor processor, drain its live
  queues once, exactly as its `begin_block` would (records present at entry). If a record kind is
  only staged for `process` rather than applied to lane state, carry it by inheriting the lane's
  consumer instead, as *Carry live-controlled effect lanes across a plan swap* does for
  effects, and say so in the PR.
- **D5. Mono collapse.** Before exporting, disengage a collapsed predecessor chain
  (`disengage_collapse`), so both channels of each lane are real. A successor chain that receives
  carried lanes starts dual, with `collapse_channels_agree` set to the AND of the source chains'
  flags (a fresh lane at rest agrees). Collapse is class A, so this moves no bit and keeps the saving.
- **D6. Carry program.** A builtin-input section of `(successor unit and lane, predecessor unit and
  lane)` pairs, built off the render thread from the two inventories and the models. No allocation
  on the render thread.

## Deliverables

1. D2 in `crates/builtins`; D3 in `crates/graph` and `crates/rack`; D4-D6 in `crates/graph`,
   `crates/rack`, `crates/builtins-compiler`; the inventory rows and join in `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/rack/src/lib.rs` (slot accessor and collapse handling)
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`

## Non-goals

- No fader, mute or pan state (slice 8), no effects (slices 9-12), no delay lines (slices 13-14).
- No per-node builtin carry (test-only processors).
- No meter state (umbrella P10).

## Objective gates

1. **Gap-free acceptance, both widths.** At `Backend::Simd8` and `Backend::Simd4` (the `test-support`
   width entries): session A is `parametric-eq-nine-track.json` with both console sections empty,
   high-pass and low-pass filters enabled at per-track, per-channel frequencies, and nonzero trims.
   Session B adds a muted track whose ID sorts first. The swapped run (swap after block 6) and a
   fresh B fed from frame 0 are bit-identical for every block. With this slice's section of the
   program disabled, block 7 differs.
2. **Live record at the swap.** Prepared with live controls: an input-trim record admitted to A
   between blocks 5 and 6 and committed in B's model (so the transaction did not change it, D1) gives
   output bit-identical to the reference A plus the muted track fed the same record at the same
   block. A second case swaps while a high-pass prepared target's coefficient ramp is in flight.
3. **Collapse kept.** A session whose tracks read a mono source: after the swap, the successor's
   chains still collapse (`bank_collapse_counters` grows in the blocks after the swap) and the output
   stays bit-identical.
4. **Rule.** A strip whose `hpf_hz` changed in the transaction is not carried; its stage starts at
   rest.
5. **Realtime.** The swap block makes zero allocations and frees (`bench_support::alloc` thread
   counters and the engine render audit).
6. Commands:
   - `cargo test --locked -p builtins -p builtins-compiler -p graph -p rack -p host-core --features builtins-compiler/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-builtins-audit.sh target/release/audit && bash scripts/trace-builtins-graph-audit.sh target/release/audit`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit` and `bash scripts/test-builtins-fixtures.sh`
   - `cargo test --locked -p console-workload` (static digests do not move)
   - the umbrella's inherited gates.

## Test value

- Gate 1: a filter restarted at rest, or a lane imported at its old lane index instead of its strip's
  new lane, turns it red.
- Gate 2: a carry that exports before draining loses the record with the retired plan; it turns red.
- Gate 3: a successor chain whose agreement flag is cleared never collapses again; it turns red.
- Gate 4: carrying a stage whose values changed keeps the old filter; it turns red.

## Dependencies

- *Prepare a successor plan whose unchanged sources keep playing* (#1272).
