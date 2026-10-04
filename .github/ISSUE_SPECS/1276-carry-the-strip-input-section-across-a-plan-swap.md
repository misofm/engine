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

## Attempt record

### Attempt 1 (implementer, on `0297efa8c`; implementation commit `807b48547`)

**Implementation.**

- D2 (`crates/builtins`): `InputLaneState`, a fixed-size `Copy` record (both channels: live trim
  word, trim ramp `current/target/step/remaining` plus the authoritative countdown, and per section
  the coefficients in use, the target, the per-sample step, the countdown and both integrators).
  `BuiltinInputBank::export_lane` / `import_lane` refuse padding lanes. Import writes every word
  verbatim, then re-derives only caches: `ramping` and `filter_ramping` as `settle` and
  `settle_filter` publish them, the elision plan, and the lane's channel-symmetry bit. The stage's
  own `filter_initial` (prepared endpoint) stays.
- D3: `GraphPreparedBuiltinBankProcessor` gains `as_any_mut` (defaulted to `None`, so the
  test-only implementors outside this slice's paths compile unchanged), `carried_input_lanes`,
  `can_adopt_input_lane`, `adopt_input_lane` and `drain_for_carry`. `rack::BankStage::as_any_mut`
  and `BankChain::slot_stage_mut` lend a slot's stage; the graph's `BuiltinStage` answers `Some`.
- D4: `BuiltinBankProcessor::drain_for_carry` runs the same body as `begin_block`
  (`drain_controls`). Every input record kind (trim, polarity, prepared filter) is applied to lane
  state by the drain, none is only staged for `process`, so no consumer is inherited.
- D5: `BankChain::disengage_for_carry` takes the disengage boundary of a predecessor chain that
  rendered its last block collapsed; `BankChain::inherit_channel_agreement` ANDs the predecessor
  chain's flag into the successor chain that receives a lane. The successor starts dual
  (`collapsed == false` at bind).
- D6: `graph::GraphLaneLocation` `(unit, slot, lane)`, `GraphLaneMove`,
  `graph::builtin_input_lanes` (read from the bound runtime's input-filter response bindings,
  kept only where the slot is a carrying builtin bank and the lane is populated) and
  `graph::install_builtin_input_carry` (refuses no carry program, a non-carrying or padding
  successor lane, repeated lanes; sorts by predecessor location). At the swap block
  `GraphExecutor::adopt_predecessor` checks every input move against both runtimes first (all or
  nothing), then moves the sources, then per move: disengages each predecessor chain once, drains
  each predecessor bank once, copies the lane and its `LIVE` symmetry term, and ANDs the agreement
  flag. `install_carry_program` now accepts a program with no source moves on a plan without a
  source set, and `carry_program_retained_bytes` adds the input move table.
- D1 (`crates/host-core/src/prepare.rs`): the inventory records one crate-private row per banked
  strip input section (strip ID, lane location, live-queue flag); `input_section_count` is public.
  A successor carries a section when the strip is in both models, of the same kind (track or
  submix), its `polarity_invert`, `trim_db`, `hpf_hz` and `lpf_hz` are bit-equal per channel in the
  committed model and in the successor's normalized model, and both plans attach the same control
  kind. The carry program is now installed when any owner carries (sources or input sections), and
  its input table is charged before bind at its full length (one move per carried strip).
- Q1 default: a strip whose input values changed in the transaction starts at rest.
- No dependency on #1253: the drain is the processor's own `begin_block` body, run once at the
  carry.

**Gates** (`crates/host-core/tests/successor_swap.rs`, appended; two existing byte assertions in
`added_muted_track_keeps_playing` and `a_changed_source_gets_its_own_ring` now include the two
unchanged input sections the carry program also moves).

| Gate | Test | Result |
| --- | --- | --- |
| 1 gap-free, W8 and W4 | `filtered_strips_keep_their_state_across_a_swap_at_{eight,four}_lanes` | green: nine tracks, both console sections and inserts empty, per-track per-channel HPF/LPF, nonzero trims, one polarity-inverted channel; B adds a muted track sorting first (banks 8+1 -> 8+2, 4+4+1 -> 4+4+2); every block bit-identical to fresh B; 9 inventory rows; program bytes `8 + 9 * 16` |
| 2 pending trim record, W8 and W4 | `a_pending_trim_record_survives_the_swap_at_{eight,four}_lanes` | green: record pushed to A's queue after block 5, committed in B's model; output equals A + muted track fed the record at block 6; without the record block 6 differs |
| 2 HPF ramp in flight, W8 and W4 | `a_high_pass_ramp_in_flight_finishes_after_the_swap_at_{eight,four}_lanes` | green: 32-frame quantum, per-channel prepared HPF targets admitted before block 5, so the 64-sample ramp is half done at the swap |
| 3 collapse kept, W8 and W4 | `collapsed_strips_keep_collapsing_across_a_swap_at_{eight,four}_lanes` | green: all-mono session collapsed before the swap; successor collapses on every post-swap block exactly as the reference, with zero agreement proofs; with the successor forced dual the output is still bit-identical |
| 3 live term | `a_one_channel_record_keeps_its_chain_dual_after_the_swap` | green: a no-op left-only trim record pending at the swap retires exactly one chain from collapsing, in the swapped run as in the reference |
| 4 rule | `a_strip_whose_filter_changed_starts_at_rest` | green: eq0's HPF changed with a muted track added; post-swap blocks equal fresh B from rest on the same PCM; program bytes 8 (source only) |
| 5 realtime | `the_input_carry_allocates_and_frees_nothing` | green: hand-over (drain, disengage, copy) plus the first successor block: allocator `(0, 0, 0)`, render audit `(0, 0)`, both widths |
| D6/P11 | `a_bad_input_move_is_refused_whole` | green: install refuses no program, a padding lane, repeated lanes; an unresolvable predecessor lane gives `PredecessorMismatch` and the successor's first block equals a fresh plan's |

**Mutations** (each applied, run red, reverted, run green; all 24 tests green after):

- M1, skip the input moves in `adopt_predecessor` (the slice's section disabled): gate 1 red at
  block 6 (the first successor block, "block 7" one-based) at both widths, and gates 2 and 3 red.
- M2, import at the predecessor's lane index: gate 1 red at block 6, both widths; gates 2, 3 red.
- M3, `drain_for_carry` drains nothing: gate 2 trim red, both widths.
- M4, `inherit_channel_agreement` clears the flag: gate 3 red (agreement proofs 1, not 0). With
  only input banks in the prefix the proof recovers the collapse at once, so the bits cannot see
  it; with an effect in the prefix (which declines the proof) the chain would never collapse again.
- M5, skip the disengage in `disengage_for_carry`: gate 3 red (forced-dual arm, stale right
  integrators).
- M6, carry a section whose values changed: gate 4 red.
- M7, do not import the coefficient target, step and countdown: gate 2 ramp red, both widths.
- M8, a `Box` in `drain_for_carry`: gate 5 red, allocator `(2, 0, 2)`, audit `(2, 2)`; a `vec!` in
  `import_lane`: gate 5 red.
- M9, drop the copied `LIVE` term: the live-term gate red.
- M10, drop the all-or-nothing pre-check: D6/P11 red (`Carried`). M11, accept a padding lane at
  install: D6/P11 red.

**Commands** (in a clean detached worktree of `807b48547`, own target directory, since other
agents had uncommitted edits in the shared tree):

- Focused: `cargo test --locked -p builtins -p builtins-compiler -p graph -p rack -p host-core --features builtins-compiler/test-support,graph/test-support,host-core/test-support`: 61 result lines, 0 failed. `cargo test --locked -p graph --features graph/test-support,engine/realtime-audit --test rt11_swap_carry_alloc`: green. `cargo test --locked -p capi`: green.
- `cargo build --locked --release -p audit -p capi`; `audit capi`: 0 allocations, 0 deallocations, 0 syscalls; `trace-builtins-audit.sh`: PASS; `trace-builtins-graph-audit.sh`: PASS.
- `check-builtins-fixtures.sh . <audit>`: ok (50 files); `test-builtins-fixtures.sh`: ok.
- `cargo test --locked -p console-workload`: green (static digests unmoved).
- `check-workspace-policy.sh`, `test-workspace-policy.sh`, `check-realtime-policy.sh` (58 regions in 16 files), `test-realtime-policy.sh`, `check-capi-abi.sh`: ok.
- `check-cross-targets.sh`: PASS, with only the known #1018 iOS `memset_pattern16` expected failures.
- In the shared tree: `cargo fmt --all -- --check` clean for this slice's files (the only diffs
  were another agent's uncommitted `crates/source/src/lib.rs`), `cargo clippy --locked --workspace
  --all-targets --all-features -- -D warnings` and `RUSTDOCFLAGS='-D warnings' cargo doc --locked
  --workspace --no-deps`: green.
- Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts` and
  `test-web-audioworklet.sh`: green. Budgets: graphSessionPlusPlanBytes 29794 of 35648,
  sourceTotalBytes 3358 of 3648.
- **ARTIFACT CHANGED**: the shipped module is
  `222393360726dbd8e30b4fa333017df294cbcead781c1d6f89c05aff38efac65` (2914595 B), against
  `751122a9ecdd05e96c8d5055bbefbb56ca4c75948d9692ade32ed01fa3959743` (2870096 B) built from
  `0297efa8c`: +44499 B. Per `docs/RELEASE.md` nothing re-pins between releases.

**Open points.** The browser module grows by 44 KB for code it does not call yet (the browser
swaps synchronously but has no successor path, Q6); unmeasured where the bytes go. The agreement
flag's AND is defended by the proof counter, not by bits, because no carried effect can sit in a
gap-free prefix until slices 9-12.
