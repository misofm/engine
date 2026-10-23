# Report lane rest and advance clocks on every production bank stage

Split from #1107 (D5 item 1, root approval 2026-10-23). A building block, the same kind as #1745 and #1746.

## Problem

Only `PreparedNativeEffectBank` has `lanes_at_rest` and `advance_clocks` (#1745, #1746, #1747). The rack `BankStage` trait has neither, and the builtin input, fader and matrix banks have no rest predicate. Every production console chain holds those stages, so no chain can report a lane at rest, and #1107's whole-bank skip cannot start.

## Smallest closable slice

- `BankStage::{lanes_at_rest, advance_clocks}`, declining by default, on:
  - `EffectBankStage`: forwards to the effect;
  - `LiveControlEffectBankStage`: the effect's mask minus every lane with a staged span, a staged prepared target, a bypass, a running crossfade or an armed observation tap after this block's drain; its clock advance also feeds the shunt line a `+0.0` block through `feed_line`;
  - `BuiltinStage`: forwards through `GraphPreparedBuiltinBankProcessor`.
- Builtin input, fader and matrix banks: a state-only predicate under #1107 D3 (one `+0.0` block leaves every state word bit-equal and writes `+0.0`):
  - input: every integrator at `+0.0` bits, no trim or filter ramp in flight; a lane whose signed trim or polarity can turn `+0.0` into `-0.0` declines unless the kernel proves otherwise;
  - fader: no gain or mute ramp in flight;
  - matrix: not ramping, and no output row whose two coefficients both carry the sign bit;
  - processor level: no automation span this block.
  - Clocks: the input's silence counter (`lane::silence_advance`); the fader and matrix have none.
- `BankChain::lanes_at_rest()`: the AND over the slots; a slot that is an identity on a lane counts as at rest on it.

## Authorized paths

- `crates/rack/src/lib.rs`, `crates/effect-contract/src/live.rs` (the live stage's predicate and shunt feed)
- `crates/builtins/`, `crates/builtins-compiler/src/lib.rs`, `crates/graph/src/lib.rs` (the two declining trait methods), `crates/graph/src/runtime.rs` (`BuiltinStage` forwarding)
- their tests; this spec

## Objective gates

1. One rest-probe test per new predicate, in the crate that owns it (the #1745 driver form): it renders a `+0.0` block and fails when a reported lane is not an exact fixed point (state words and output).
2. Each `advance_clocks` equals a rendered `+0.0` block on its clocks.
3. Allocation-free; fmt, the workspace clippy line, `tools/realtime-policy`, the AArch64 test check, `cargo doc -D warnings`.

## Test value

Gate 1 turns red when a predicate claims rest while a word would still change; gate 2 when a skipped lane's clock drifts from a rendered one.

## Dependencies

#1745, #1746, #1747, #1763, #1765 (merged). Blocks #1107.

## Evidence (attempt 1, 2026-10-23)

**Mechanism.**
- `rack::BankStage` gains `lanes_at_rest()` and `advance_clocks(lanes_mask, block)`. Both decline
  by default. The skipped `+0.0` block is passed in, so a stage that stages input for a later
  block can feed those words.
- `EffectBankStage` forwards both to the effect.
- `LiveControlEffectBankStage` takes the effect's mask and clears every lane that, after the
  drain, has any of: a staged span, a staged prepared target, a dry read (bypassed or
  crossfading), or an armed tap. The #1763 quiet skip keeps an empty partition, so it is covered.
  Its `advance_clocks` forwards the effect clocks and feeds the shunt line through `feed_line`.
- `BankChain::lanes_at_rest` is the AND over the slots that have an active lane. A slot active on
  some lanes answers its inactive (identity) lanes from its own state.
- The builtin banks implement the predicate as follows:
  - **Input:** every integrator at `+0.0` bits, and no trim or filter ramp in the bank. Its clock
    is the silence counter (`silence_advance`, masked).
  - **Fader:** no gain or mute ramp in flight, and the gain is muted or has no sign bit.
  - **Matrix:** not ramping, and no output row whose two coefficients both carry the sign bit.
- The graph trait `GraphPreparedBuiltinBankProcessor` gains `builtin_lanes_at_rest` and
  `advance_builtin_clocks`, both declining by default. `BuiltinStage` forwards them. The fader
  processor clears every automated lane.

**Signed-zero finding (input).** A polarity-inverted lane does *not* decline. With zero
integrators, `+0.0 * negative trim = -0.0` leaves the state at `+0.0`, and the last step of every
section chain turns the output into `+0.0`. The derivation is in the `InputStage::lanes_at_rest`
doc. The rest probe draws inverted lanes.

**Stated limits.**
- The whole input bank declines while any of its lanes has a trim or filter ramp in flight. The
  ramp bodies rewrite every lane's countdown residue and the elision plan.
- An automated fader lane declines outright, whatever its curve. Its timed path advances event
  cursors and held-ramp obligations, and no clock advance reproduces that.

**Gates.** The worker ran these; root relayed them, and I reran fmt, clippy and doc on the
committed tree.
1. Rest-probe tests, one per predicate:
   - `an_input_lane_called_at_rest_stays_there_on_a_silent_block`,
     `a_fader_lane_called_at_rest_stays_there_on_a_silent_block` and
     `a_matrix_lane_called_at_rest_stays_there_on_a_silent_block`, in
     `crates/builtins/src/rest_tests.rs`, using `conformance::rest_probe` with scheduled retargets,
     sign-bit gains, negative matrix rows and inverted polarity;
   - `the_live_stage_rests_only_lanes_with_no_work_and_skips_exactly`
     (`crates/rack/tests/lane_rest.rs`);
   - `a_chain_rests_a_lane_every_running_slot_rests` (rack lib);
   - one assertion in builtins-compiler's existing fader-automation test.
2. Clock advance:
   - `input_advance_clocks_leaves_the_stage_where_a_rendered_zero_block_does`: across the arming
     edge, at saturation, with short blocks, and the next signal block renders the same bits;
   - the live stage's line feed, checked by a step bypass after a skipped block, inside the
     live-stage test.
3. Checks: fmt, clippy on the four crates with `--all-features`, and the rack, builtins,
   builtins-compiler and graph tests with test-support (404 passed, 0 failed). Also the AArch64
   `cargo check --tests`, `cargo doc -D warnings` and `tools/realtime-policy` (191 regions).
   The worker's first doc run failed on a public doc link to the private `LaneWalk`. That was
   fixed, and the rerun is clean.

**Mutants.** All 15 were red (worker-reported):

| Area | Term dropped |
|---|---|
| Input | the ramp term; the silence advance |
| Input | `-0.0` integrator counted as rest (a broken term, not a dropped one) |
| Fader | the sign term; the ramp term |
| Matrix | the sign-row term; the ramp term |
| Live stage | the span term; the target term; the dry term; the tap term; the line feed; the effect-clock forward |
| Fader processor | the automation term |
| Chain | inactive slots counted |

**Test value.**
- Each builtin rest probe turns red when its predicate claims rest while a state word, a ramp word
  or the output sign would still change on a `+0.0` block. No test can see this today, because no
  builtin bank has a predicate.
- The input clock test turns red when a skipped block's silence counter drifts from a rendered
  one.
- The live-stage test turns red in either of two cases. One is a lane with a span, a target, a
  dry read or a tap reported at rest. The other is a skipped block that leaves the shunt line
  other than a rendered one leaves it, which a later bypass edge would expose.
- The chain test turns red when the AND counts a slot that never runs, or drops a running slot's
  identity lanes.
- The fader-processor assertion turns red when an automated lane is reported at rest.
