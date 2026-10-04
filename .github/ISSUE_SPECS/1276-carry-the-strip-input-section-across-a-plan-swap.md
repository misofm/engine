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

### Attempt 1 verdict (FAIL): summary

One MAJOR, four MINORs, four NITs (`/home/bl/misofm/submix-verdicts/1276-attempt1.md`).

- **MAJOR-1.** The carry ANDed the predecessor chain's `collapse_channels_agree` into the successor,
  but rack maintains that flag only on a chain that can collapse; an unarmed chain (stereo-source
  neighbour, asymmetric input delay) keeps `true` from bind. A successor could then collapse a
  carried lane whose channels disagree and move output bits, reachable on the C ABI by a
  structural transaction that makes an asymmetric delay symmetric.
- **MINOR-1.** D1's comparator was defended for `hpf_hz` only; trim, lpf, polarity, the
  control-kind and strip-kind clauses had no test.
- **MINOR-2.** `drain_for_carry` discarded a drain error, and the drain stopped at the first
  refused record, leaving that bank's later records in a queue that never renders again.
- **MINOR-3.** The flag's documentation did not mention the carry.
- **MINOR-4.** The browser module grew by 44.5 KB, about 30 KB of it prepare-time sort and
  `BTreeMap` instantiations in the join.
- **NITs.** A weakened `debug_assert`, unmarked render-thread helpers, untested cache refreshes
  in `import_lane`, and a forward note on routing admission to the newest plan (NIT-4, no action
  in this slice).

### Attempt 2 (implementer, on `55690373d`)

**Changes.**

- MAJOR-1 / MINOR-3 (`crates/rack`, `crates/graph`): `BankChain::carried_channel_agreement()`
  is `can_collapse() && collapse_channels_agree`, and the carry reads it instead of the raw
  flag. The `Coming back` clause list gains a fifth clause (the carry's AND and its premise);
  `collapse_channels_agree()` now says the flag is maintained only while `can_collapse` holds
  and points outside readers at the new accessor.
- MINOR-2 (`crates/builtins-compiler`, `crates/graph`): `drain_controls(carry)` applies every
  record present at entry; the block drain still stops at a refused apply and returns it, the
  carry drain counts it and goes on. `GraphPreparedBuiltinBankProcessor::drain_for_carry` now
  returns the refused count (default 0); the runtime `debug_assert_eq!`s it to zero. No
  allocation, no `RenderError` built on the carry path.
- MINOR-4 (`crates/host-core/src/prepare.rs`): both `BTreeMap`s of the join are gone. The
  committed strip is found by binary search in its own segment of the normalized committed
  model (`committed_input_section`), which also makes the strip-kind clause structural (a track
  is searched among tracks, a submix among submixes; `same_strip_kind` removed). The successor's
  own rows are sorted once (the inventory needs that sort anyway) and the join binary-searches
  them. `SuccessorBase::committed` documents that it is a normalized model (every caller passes
  `normalized_model()`); a search that misses only leaves a strip at rest.
- NIT-1: `debug_assert_eq!(moves.len(), carried_inputs.len())` and the retained-bytes assertion
  back to `debug_assert_eq!`. NIT-2: `REALTIME_POLICY` regions around `builtin_processor_mut` /
  `input_lane_mut`, `InputStage` and `BuiltinInputBank` `export_lane` / `import_lane`, and rack's
  `slot_stage_mut` .. `inherit_channel_agreement` (58 -> 62 regions). NIT-3: a comment in
  `import_lane` says both cache refreshes are defence and why no test sees them.

**Tests** (`crates/host-core/tests/successor_swap.rs`, `crates/builtins-compiler` unit test).

- `a_diverged_lane_keeps_its_chain_dual_after_the_swap_at_{eight,four}_lanes`: three arms per
  width. (1) Stereo neighbour: `eq6` (R trim 6 dB below L) pools beside stereo `eq5` in an unarmed
  chain; a `Both` record two blocks before the swap equalises the trims; B commits them and adds
  a muted track. (2) Armed: the same without the stereo neighbour, so `eq6`'s chain cleared its
  flag. Both compare the collapsing and the forced-dual successor bit for bit with the reference
  (A plus the muted track, same record, same block). (3) Delay, no live queue: `eq6` has R delay
  37 in A, B makes it symmetric; the collapsing successor equals the forced-dual one bit for bit.
- `a_strip_whose_input_section_changed_starts_at_rest` replaces
  `a_strip_whose_filter_changed_starts_at_rest`: the same at-rest oracle for an `hpf_hz`, an
  `lpf_hz`, a `trim_db` and a `polarity_invert` edit.
- `a_changed_control_kind_carries_no_input_section`: plain -> live and live -> plain successors
  move only the source (8 bytes); the same kind on both sides moves nine lanes.
- `a_track_replaced_by_a_submix_of_its_name_does_not_carry`: B replaces track `eq0` by a submix
  `eq0` with the same input section, fed by `eq1`; only the source and `eq1` move.
- `tests::the_carry_drain_applies_every_record_past_a_refused_one` (builtins-compiler): lane 0
  holds a NaN trim then a valid one, lane 1 a valid one; the carry drain returns 1, applies both
  valid trims and empties both queues.

**Mutations** (each applied, run red, reverted; logs in the worker scratchpad `mut1276/`):

| Mutation | Red test(s) |
| --- | --- |
| MA1, carry reads `collapse_channels_agree()` (attempt 1) | diverged-lane, both widths: stereo-neighbour arm at block 6; with that arm skipped, the delay arm red; the armed arm stays green (its flag is maintained), as it should |
| MA2, `inherit_channel_agreement` ignores `agree` | diverged-lane, both widths; with the stereo arm skipped, the armed arm red at block 6 |
| Mh / Ml / Mt / Mp, drop the hpf / lpf / trim / polarity comparison | `a_strip_whose_input_section_changed_starts_at_rest`, each |
| Md, `row.live == live_input` -> `true` | `a_changed_control_kind_carries_no_input_section` (152 bytes, not 8) |
| Mk, a submix also found among the committed tracks | `a_track_replaced_by_a_submix_of_its_name_does_not_carry` (40 bytes, not 24) |
| Mdrain, the carry drain stops at a refused apply | `the_carry_drain_applies_every_record_past_a_refused_one` |

**Gates** (in `/home/bl/misofm/wt-swap`, all exit 0):

- `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
  -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- `check-workspace-policy.sh`, `test-workspace-policy.sh`, `check-realtime-policy.sh` (62 marked
  regions in 16 files), `test-realtime-policy.sh`, `check-capi-abi.sh` (shared and static).
- Focused `cargo test --locked -p builtins -p builtins-compiler -p graph -p rack -p host-core
  --features builtins-compiler/test-support,graph/test-support,host-core/test-support`: 581
  passed, 0 failed (`successor_swap` 28/28, avx2 build so the `Simd8` rows ran);
  `rt11_swap_carry_alloc` green; `cargo test -p capi` and `-p console-workload`: 117 passed, 0
  failed (static digests unmoved).
- `cargo build --locked --release -p audit -p capi`; `audit capi`: 0 allocations, 0
  deallocations, 0 syscalls, 0 violations, `pcm_digest` `c60671f6593fa603`.
- `trace-builtins-audit.sh`, `trace-builtins-graph-audit.sh`: PASS; `check-builtins-fixtures.sh`
  (50 files), `test-builtins-fixtures.sh`: ok.
- `check-cross-targets.sh`: PASS, only the known #1018 iOS `memset_pattern16` expected failures.
- Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`
  (graphSessionPlusPlanBytes 29794 of 35648, sourceTotalBytes 3358 of 3648),
  `check-scalar-oracle-absent.py`, `test-web-audioworklet.sh`: green.
- **ARTIFACT CHANGED**: the shipped module is
  `e0fef6988d081ed4f8fd763562f25b771f8615f189e96452eb00c347b9235e63` (2,895,964 B). Against
  the attempt-1 tree (`f41388939`, 2,914,664 B per the verdict; `55690373d` changes only capi
  files, so its module is the same) that is -18,700 B; against the pre-#1276 base `0297efa8c`
  (2,870,096 B) the slice now costs +25,868 B instead of +44,568 B. Not re-pinned
  (`docs/RELEASE.md`).

**The `audit capi` digest.** `807b48547` moved `pcm_digest` from `7281b6c931e05dcc` to
`c60671f6593fa603`; attempt 2 leaves it at `c60671f6593fa603`. The audit renders the nine-track
fixture, whose input sections run a 20 Hz high-pass and a 20 kHz low-pass, and applies one
structural transaction (a muted track) after its first render call. Before #1276 the successor's
input filters restarted at rest on the swap block; since #1276 every unchanged section carries its
integrators, so the PCM from the swap block on is the gap-free continuation, and the digest of it
differs. Nothing live pins this digest.

**Open points.** NIT-4 stands as a forward note for the first slice that gives a host input live
edits across a published successor. The carried agreement from an unarmed chain is now `false`,
which costs at most one agreement proof on the successor (a lane that does agree proves it at
once, since input banks answer `channels_agree`).
