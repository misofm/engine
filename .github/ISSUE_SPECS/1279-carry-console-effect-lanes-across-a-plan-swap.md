# Carry console effect lanes across a plan swap

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-7, D15-8).
Slice 10 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

A console slot keeps its exact state through a plan swap when the transaction changed none of its
prepared values. This holds on every strip, also when the strip's lane moves to another bank. The
carry moves the state at the swap block, and the predecessor never renders again. The state covers
EQ filter memories, compressor, gate and transient-shaper envelopes, soft-clip filters and limiter
look-ahead lines. Adding a track to a playing session with a console EQ, compressor and limiter is
bit-continuous for every other track.
Mono-source chains keep collapsing after the swap.

## Context

- Console groups always bank, padded (`pads`, `crates/graph-compiler/src/banks.rs:76`). The
  eligible effects are `CONSOLE_ELIGIBLE_EFFECTS` (`crates/effect-compiler/src/prepare.rs:223-230`):
  EQ, compressor, gate/expander, soft-clip, transient shaper and true-peak limiter.
- A console slot without live controls is an `EffectBankStage` (`crates/rack/src/lib.rs:750`, impl
  `:800`) over a `Box<dyn PreparedNativeEffectBank>`. With live controls it is #1280's
  `LiveControlEffectBankStage`.
- The lane payload calls are render-safe and exact mid-ramp (#1278, on `main`):
  `snapshot_track_state_payload(&self, …)` and `restore_track_state_payload`
  (`crates/effect-contract/src/lib.rs:2070`, `:2079`). A collapsed bank must be desymmetrized before
  a snapshot (`desymmetrize_channels`, `:2182`). Two docs still say that no engine path snapshots a
  bound bank (`:2066`, `:2178`).
- `EffectBankStage` caches each lane's designed-word symmetry at bind (`designed`, `:755-771`), on
  the premise that restore never runs on a bound bank. Every witness term a stage caches for a lane
  must be refreshed after a restore, the RESTORED term included (`ChannelSymmetryWitness`,
  `crates/effect-contract/src/symmetry.rs:155`).
- The base, the live/prepared split and the restart set come from *Carry fader, mute and pan ramps
  across a plan swap* (#1277, D2-D6).

## Decisions frozen for this slice

- **D1. Key and rule** (every effect owner; #1280-#1282 reuse it). The owner key is `(strip ID,
  rack, effect ID)` (`EffectNodeId`); a console slot's effect ID is its slot ID. The owner's
  **prepared values** are:
  - its identity, quality and link mode (a console slot's come from the session slot);
  - every value the classifier refuses as `Prepared`, `PreparedBypass` or `Domain`;
  - for an owner with no live control lane, every parameter value and its bypass.

  The owner carries when no prepared value differs between the predecessor's base (#1277 D2) and
  the successor's model, and the inventory's prepared layout matches: prepared bypass, latency,
  `state_sizes` and `state_layout_version`. Otherwise it starts at rest, and its strip goes into
  the restart set (#1277 D6). Padding lanes never carry.

  A strip whose effect chain changed also goes into the restart set, even when its remaining owners
  carry (D15-9). A changed chain means an insert added, removed or reordered, or a session console
  slot added or removed, which changes every strip's chain. The chain's processing changes, so a
  carried neighbour does not make it continuous.
- **D2. Live differences.** An `EffectBankStage` has no live lane, so every difference on it is
  prepared (D1), and this slice emits no retarget. #1280 retargets live-controlled lanes.
- **D3. Move mode.** At the swap block, for each carried lane:
  1. desymmetrize a collapsed predecessor bank, once per bank;
  2. snapshot the predecessor lane into one scratch buffer;
  3. restore it into the successor lane.

  The successor preallocates the scratch at the largest carried `state_sizes.total()`.
- **D4. Desymmetrizing is bit-neutral.** It writes into the right channel the state a dual run
  would hold, not an approximation of it (`desymmetrize_channels`' doc,
  `crates/effect-contract/src/lib.rs:2173-2182`). Collapse saves cost and never changes bits. So
  the bank's output is the same after the call as before, and the snapshot of step 2 is exactly the
  payload a dual run would write. Every carry slice that snapshots a bank lane (#1280, #1281)
  relies on this rule.
- **D5. Witness refresh.** After a lane is restored, recompute every term the stage caches for it
  (the `designed` flag and any RESTORED term), and update the cache's rustdoc: restore now also runs
  on a bound bank, in the swap block only. Chains take #1276's AND rule for their
  agreement flag. Update the two "no engine path snapshots a bound bank" docs.
- **D6. Refusal.** A restore that refuses leaves the lane as prepared (at rest) and increments its
  console bank stage's saturating `u64` count `carry_refused`. The prepared graph plan sums these
  in `carry_refusal_count()`, read after render (named like `RealtimePlanOwner::carry_mismatch_count`,
  `crates/engine/src/realtime/plan_exchange.rs:372`). It never panics on the render thread. #1278
  makes a refusal of an own payload a defect, so gate 1 asserts the count is 0.

## Deliverables

1. D1-D6 in `crates/rack` and `crates/graph`, and the inventory rows and the join in
   `crates/host-core`.
2. Gap-free tests in `crates/host-core/tests/successor_swap.rs`.

## Authorized paths

- `crates/rack/src/lib.rs`
- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`,
  `crates/host-core/tests/support/successor.rs`
- `crates/effect-contract/src/lib.rs` (docs only)

## Non-goals

- No live-controlled bank lanes (#1280), and no bank to per-node flips (#1281).
- No whole-bank move (#1269, Deferred; it reopens only if #1286 shows that lane copies dominate).
- No transition for a restarted strip (#1324).

## Objective gates

1. **Gap-free acceptance, every console effect, both widths.** At `Backend::Simd8` and
   `Backend::Simd4`, session A is `parametric-eq-nine-track.json` with a console of all six
   `CONSOLE_ELIGIBLE_EFFECTS` in `pre_insert` and `post_insert`. Each effect does work on the test
   signal: EQ gain, the compressor and the limiter reducing gain, the gate opening and closing.
   Session B adds a muted track whose ID sorts first, so every lane shifts. The swapped run (swap
   after block 6) and a fresh B fed from frame 0 are bit-identical in every block.
   `carry_refusal_count()` is 0. With this slice's section of the program disabled, block 7 differs.
2. **Collapse kept.** On a mono-source session with the same console, the successor's chains
   collapse in the blocks after the swap (`bank_collapse_counters` grows), and the output stays
   bit-identical.
3. **Prepared change restarts.** B changes one strip's compressor threshold. On this owner it is
   prepared: an `EffectBankStage` has no live lane. That slot does not carry, its other slots do,
   and `restarted_strips()` is exactly that strip. A second case adds a compressor insert to one
   strip: its console slots carry, and the strip is in the restart set.
4. **Realtime.** The swap block of gate 1 makes zero allocations and frees with every console
   effect carried (`bench_support::alloc` thread counters and the engine render audit).
5. Commands:
   - `cargo test --locked -p rack -p graph -p host-core --features rack/test-support,graph/test-support,host-core/test-support`
   - `cargo build --locked --release -p audit && bash scripts/trace-graph-audit.sh target/release/audit`
   - `cargo test --locked -p console-workload`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`

## Test value

- Gate 1: a lane restored to its old lane index instead of its strip's new lane, or an effect whose
  restore misses a derived word, turns it red.
- Gate 2: a stale `designed` cache, or an agreement flag cleared for good, either collapses on
  asymmetric words (bits move) or never collapses again. One of gates 1-2 turns red.
- Gate 3: a rule that carries an owner whose prepared value changed keeps the old threshold. A
  restart set that misses effect owners fails the exact list. Either turns it red.
- Gate 4: a payload call that still allocates on any console effect turns it red.

## Dependencies

- *Carry fader, mute and pan ramps across a plan swap* (#1277).
- #1278 is on `main`.

## Attempt record

### Attempt 1

Commits on `codex/d15-stream-a2`, after the #1277 follow-ups `1080ccbc0`: the carry `f5f61af3b`, the
realtime floor and docs `8f897e6ab`, the layout and agreement tests `bc267c2a5` and `287510412`, the scratch charge `3e1f74c2c`, the capi oracle `7852f0a25`, and this record.

Premise check. A console slot without live controls is an `EffectBankStage`; one with a live
control lane, or a lane whose session bypass is lowered (#1087), is a `LiveControlEffectBankStage`
(`graph/src/runtime.rs` `stage_for`). The browser form that attaches fader and matrix lanes only
keeps `EffectBankStage`; a plan prepared with effect lanes (the C ABI, `HostLiveLanes::ALL`) has no
`EffectBankStage` console lane, so this slice carries nothing there and every strip with a console
slot is in `restarted_strips()` until #1280. The spec's premise holds.

Design.

- `rack`: `EffectBankStage` keeps a per-lane `ChannelSymmetryWitness` (the `DESIGNED` and `RESTORED`
  terms it decides) instead of the `designed` flags, its `EffectLaneLayout` (effect, state layout
  version, state sizes, prepared bypass, latency) and a saturating `carry_refused`.
  `carry_lane_from` snapshots the predecessor lane into the scratch, restores it, and recomputes the
  witness (`DESIGNED` from the bank, `RESTORED` from whether the payload's left and right sections
  are byte-equal). The cache's rustdoc says restore now runs on a bound bank, in the swap block.
- `graph`: `bank_effect_lanes` (rows), `GraphEffectMove`, `install_effect_carry` (validates, sorts by
  predecessor, preallocates the scratch at the largest carried payload), `carry_refusal_count`,
  `effect_carry_retained_bytes`, `EffectLaneLayout` re-export. At the swap block
  `can_carry_effect_lanes` checks every move before anything moves (all or nothing, as the other
  sections); `carry_effect_lanes` disengages each predecessor chain once (`disengage_for_carry`,
  which desymmetrizes every collapsed bank, D3 step 1 and D4), moves each lane, and ANDs the
  predecessor chain's agreement into the successor chain (#1276 D5).
- `host-core`: `EffectLaneInventoryRow` in `PlanStateInventory`; `join_chains` (model half: a console
  slot is `same` when its session slot, section, bypass and parameters are bit-equal; the chain is
  changed when a session slot is added, removed or moved, or an insert is added, removed, reordered
  or changes identity); `resolve_effect_carry` (lane half: row in both plans, `same`, equal layout);
  the restart set is the union with #1277's; the move table and scratch are charged against the
  graph cap, the scratch against the named-allocation cap.
- Owners are console slots only. An insert is a chain member (its addition restarts the strip), but
  its own lane, banked or per node, is #1281 and #1282; making it an owner here would list every
  strip with an insert as restarted.

Deviations from the spec text, each forced:

- Gate 1 and the other cases run at four lanes only on a build with `neon` or `simd128`: an effect
  bank binds only at the width its build executes (D4), so `Backend::Simd4` fails to prepare a
  console on x86-64-v3 (`console.slot.unbanked`). The four-lane cases compile for
  `aarch64-unknown-linux-gnu` (`cargo check --tests`) and run in CI only.
- Gate 2 uses the collapse-capable subset (EQ, compressor, limiter). The gate/expander, soft-clip
  and transient shaper declare no channel symmetry, so a chain holding one never collapses and
  "the same console" cannot satisfy the gate.
- `carry_refusal_count` is `graph::carry_refusal_count(&mut PreparedRenderPlan)`, read by whoever
  holds the plan: the active plan lives in `RealtimePlanOwner`, whose file is not authorized.
- Gate 1's "block 7 differs" mutation is red at block 6, the swap block (the swapped run's first).
- `crates/capi/tests/resource_lifecycle.rs` (not authorized) needed one line: the double-live
  oracle read the prospective epoch's inventory bytes off a fresh preparation, whose restart set is
  empty; the successor's inventory now lists the restarted strips (every strip, for that C ABI
  session). Root to approve or rule otherwise.
- `scripts/check-realtime-policy.sh` and `scripts/test-realtime-policy.sh`: floor 111 to 112
  regions (one region in `rack`), under STREAMS' standing exception.

Tests (each answers the test-value question):

- `console_effect_state_survives_a_swap_at_*` (gate 1): a lane restored to the wrong lane, an effect
  whose restore misses state, or a join that skips a slot; `every_console_effect_does_work_at_*`:
  a session whose effects leave the signal alone would pass gate 1 vacuously.
- `console_chains_keep_collapsing_across_a_swap_at_*` (gate 2, four cases: successor free or forced
  dual, input sections carried or restarted): a carry that does not disengage a collapsed predecessor
  (the dual successor reads a stale right channel), a successor that loses the agreement it inherits.
- `a_chain_that_could_not_collapse_hands_over_no_agreement_at_*`: the successor chain not taking the
  predecessor chain's agreement through the effect carry (the input carry is out of play).
- `a_prepared_change_to_one_slot_restarts_only_its_strip_at_*` (gate 3): a rule that carries a slot
  whose value changed (old threshold kept: exact at-rest oracle on a lone compressor), a restart set
  that misses or over-lists, a join that restarts the whole strip (one move fewer, exactly).
- `an_added_insert_restarts_its_strip_and_its_console_slots_carry_at_*` (gate 3 second case, with a
  transparent insert as exact oracle): a missing chain-change rule, or a console slot that does not
  carry beside a changed chain. `a_removed_console_slot_restarts_every_strip_at_*`: the same rule
  for a session slot.
- `the_effect_carry_allocates_and_frees_nothing_at_*` (gate 4): an allocation in the swap block.
- `a_console_slot_with_live_controls_restarts_its_strip_at_*`: a join that assumes a row exists for a
  `LiveControlEffectBankStage` slot.
- `prepare::tests::a_lane_of_another_prepared_layout_is_not_moved`: layouts are equal in every
  reachable session edit, so only the pure join can show a guard that trusts two rows.
- rack `effect_lane_carry_tests` (6): lane index, `DESIGNED` and `RESTORED` recomputation,
  refusal counted, saturating, left at rest, layout and scratch guards.

Mutations (defect applied: red; reverted: green):

| Defect | Red |
| --- | --- |
| no effect move written | gate 1 (block 6), gate 2, gate 3 (both), removed slot, insert, `a_strip_behind_an_effect_still_carries_at_eight_lanes` (#1277) |
| successor location = predecessor location | gate 1, 2, 3, 4, insert |
| carry despite a changed value | `a_prepared_change_to_one_slot_*` |
| chain change ignored | removed slot, insert |
| effect carry does not disengage the predecessor chain | `console_chains_keep_collapsing_*` (inputs-restart case) |
| successor chain does not inherit agreement | `a_chain_that_could_not_collapse_*` |
| allocation in `carry_effect_lanes` | render audit aborts the binary (gate 4) |
| missing row not a restart | `a_console_slot_with_live_controls_*` |
| layout guard dropped from the join | `a_lane_of_another_prepared_layout_is_not_moved` |
| witness not recomputed / `DESIGNED` stale / sections not compared | rack: `recomputes_the_designed_term`, `divergent_sections_clears_the_restored_term` |
| refusal not counted / wrapping add | rack: `a_refused_restore_*`, `lanes_that_do_not_fit_*` |
| predecessor lane index replaced by the successor's | rack: three tests |
| layout not checked in `can_carry_lane_from` | rack: `lanes_that_do_not_fit_*` |

Gates run (all green, x86-64-v3, AArch64 not run): the gate 5 commands (`cargo test -p rack -p graph
-p host-core` with the test-support features, 459 passed; `-p builtins -p builtins-compiler`;
`-p console-workload`), the CI workspace debug test command (1589 passed), `cargo test -p capi`,
release `audit` with `trace-graph-audit.sh`, `trace-builtins-audit.sh` and
`trace-builtins-graph-audit.sh`, `audit capi` (0 allocations, 0 syscalls), `check-realtime-policy.sh`
(112 regions, 27 files) and its self-test, `check-workspace-policy.sh`, rack, host-core, graph and
builtins policies, `check-capi-abi.sh`, `check-cross-targets.sh`, fmt, clippy `-D warnings`,
`RUSTDOCFLAGS='-D warnings' cargo doc`, the worklet chain (named-twin build,
`check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`,
`check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh` with a private TMPDIR, empty
afterwards, the V8 spill gate).

Open items:

- The non-zero path of `graph::carry_refusal_count` (the sum over stages) has no end-to-end test:
  an own payload is never refused, and no host seam injects a refusing bank. The stage count is
  covered in `rack`; gate 1 asserts 0.
- Insert lanes (banked or per node) restart until #1281 and #1282.
- On a C ABI plan every console slot restarts until #1280.
