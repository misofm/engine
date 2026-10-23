# Swap two lanes' state inside a console bank without allocating

Split from *Skip silent lanes' work in console banks* (#1107), D2 item 2.

## Problem

#1107's mechanism (D1, "swap-to-pack") moves an active lane out of a bank that it would keep
running and into a silent lane's position in a bank that runs. That needs a render-time swap of
two lanes' full per-lane state in every slot of a console chain. None exists: `carry_lane_from`
(`crates/rack/src/lib.rs` near `:911` and `:1382`) restores a lane through a payload at a plan
swap, and it is not a render-time operation.

## Smallest closable slice

- Add `swap_lanes(a: (bank, lane), b: (bank, lane))` to the bank stage contract. It exchanges the
  two lanes' state, coefficients, ramps, control lanes and taps, and allocates nothing.
- Implement it for the six console-eligible effects, for `LiveControlEffectBankStage` (controls,
  taps) and for the rack `BypassShunt` line.
- Shared per-bank cursors (the limiter's `Cursors`, the soft clip's `pos`) are not per lane and
  do not swap; the slice records that every bank of one slot keeps equal cursors, so a swapped
  lane lands on the same cursor.
- Padded lanes never swap.
- Add an allocation-free `advance_clocks(lanes_mask, frames)` (or fold it into the swap entry) so a caller that skips a resting lane's block advances that lane's clocks exactly as a rendered zero block would: the limiter's per-bank cursors and per-lane van Herk `phase`, and the EQ's silence counters, which are payload-visible (`STATE_SILENCE_WORD`; #1745 D1). A test renders N zero blocks on one bank and skips N on a twin with `advance_clocks`; every state and payload word is equal after.

## Authorized paths

- `crates/rack/src/lib.rs`, `crates/effect-runtime/src/bank.rs`, `crates/effect-contract/src/live.rs`
- the six effect crates (parametric-eq, compressor, gate-expander, soft-clip, transient-shaper,
  true-peak-limiter): source and tests
- this spec

## Objective gates

1. A randomized test per effect and for a whole chain: render N blocks, swap two random lanes
   (in one bank and across banks), render M more blocks with the inputs swapped too; every
   strip's output is bit-identical to the run with no swap. Simd8 and, where built, Simd4 (#1694).
2. `swap_lanes` allocates nothing (`bench_support::alloc` thread counters after warm-up).
3. `cargo fmt --all --check`, the workspace clippy line, `tools/realtime-policy`.

## Test value

Gate 1 turns red when a swap leaves one state field behind (for example a ramp target or a tap),
which no current test can see because no swap exists.

## Dependencies

None. It can run beside the predicate slices.

## Decision record

### D1 Design (attempt 1, 2026-10-23; root rulings of the same day)

- **One source of truth for lane state.** `effect_contract::swap_bank_lanes` (`live.rs`) swaps
  two lanes (one bank, or two banks of one layout) through the existing state payload: snapshot
  both, rotate the prepared defaults, restore crosswise. The payload is already the complete,
  carry-tested record of a lane's running state, so no second per-field list can drift. Cost:
  each lane's state is read twice and written twice, plus two payloads of scratch (about 2x a
  native field swap; limiter at 96 kHz: 23,328 payload bytes, 46,656 + 24 scratch bytes). It is
  paid only when #1107 moves a lane. Scratch is caller-preallocated:
  `swap_bank_lanes_scratch_bytes(bank, sizes)`, the one size formula.
- **Prepared per-lane defaults** (not in the payload; read by `FullToDefaults`, D7 recovery,
  restore validation and target permission checks) move by
  `PreparedNativeEffectBank::{lane_default_bytes, exchange_lane_defaults}` (lib.rs, root-approved).
  `None` declines and the swap refuses (`LaneSwapRefusal::Defaults`). The plan-swap carry does
  not use them: a successor keeps its own prepared defaults.
- **All or nothing.** Every refusal (defaults size, scratch, lane index, snapshots) comes before
  the first lane write. Then: rotate defaults, restore first, restore second. A restore refusal
  there is a defect (own payloads, one layout, own defaults); it is still undone (rotation
  reversed, first lane restored from its own payload). Each step's precondition is a comment in
  `swap_bank_lanes`.
- **Rack.** `BankStage::{can_swap_lanes, swap_lanes, lane_swap_scratch_bytes, as_any}` (declining
  defaults), implemented by `EffectBankStage` and `LiveControlEffectBankStage`;
  `BankChain::swap_lanes` checks every slot first (active lanes only, so a padded lane never
  swaps; equal slot count, per-slot activity, fold and aux bits; each slot `can_swap_lanes`),
  disengages a collapsed chain, ANDs the two chains' channel agreement, then swaps slot by slot.
  A slot that is an identity on both lanes is skipped (it holds neither strip's state). The
  resident output block is stale for the two lanes until the next run.
- **`advance_clocks(lanes_mask, frames)`** (lib.rs, default no-op; a caller skips a lane only when
  `lanes_at_rest` reports it). Clocks: EQ per-lane `silence` (and the bank's
  `silent_fixed_point` on a whole-bank mask); limiter per-bank `Cursors` and per-lane `phase`
  (and its three bank flags as a rendered zero block leaves them); soft clip per-bank history
  `pos`; gate bank-wide `ramp_frames_left` (block-split schedule only). Compressor and transient
  shaper: none. Bounded by lanes, never by `frames`.
- **Shared per-bank cursors never swap.** All banks of one slot keep equal cursors (limiter
  `Cursors`, soft clip `pos`, shunt line cursor): asserted in the limiter and soft clip clock
  tests and in the limiter gate-1 test. Payloads and shunt lines are walked relative to each
  bank's own cursor, so a swap is exact either way.

### D2 Per-lane field classes

Classes: **P** payload; **D** defaults exchange; **S** stage-level swap; **R** re-derived on
restore from a moved field; **U** bank-uniform (one program key per slot; reason given); **B**
bank-wide, not per lane; **X** per-block scratch; **A** padding mask (padded lanes never swap).

| Bank | Field | Class |
|---|---|---|
| parametric-eq | section `ic1/ic2`, `coef`, `step`, `remaining`, `silence`, numeric band targets, HPF/LPF enables | P |
| parametric-eq | `initial[lane]` (general band enable/kind and values), `initial_words[lane]` | D |
| parametric-eq | general-band enable/kind in `targets`, section `target`, `identity`, `dry` | R (`decode_track`, `commit_track`, `refresh_identity`) |
| parametric-eq | `sample_rate`, `bypass` | U |
| parametric-eq | `silent_fixed_point` | B (withdrawn by restore) |
| parametric-eq | rest planes / `active` | X / A |
| compressor | `gain_reduction_db`, `ramps`, `rate_ramps` (both channels) | P |
| compressor | `defaults[lane]` (both channels) | D |
| compressor | `words` (designed coefficients) | R (`commit_channel`) |
| compressor | `sample_rate`, `bypass`, `link_mode`, `automation_capacity`, `state_sizes`, `silent_bypass` | U (`same_program` to bind) |
| compressor | `silent_fixed_point` / `active` | B / A |
| gate-expander | `gain_db`, `hysteresis.open/hold`, `timing`, `ramps` | P |
| gate-expander | `defaults[lane]` | D |
| gate-expander | `coef.attack/release/hold_samples` | R (`rederive_lane`) |
| gate-expander | `coef.bypass/link_max/link_avg`, `sample_rate`, `automation_capacity`, `bank_width` | U (splatted from metadata) |
| gate-expander | `ramp_frames_left` / `active` | B (restore raises it; a clock) / A |
| soft-clip | `drive/output/mix` state, `ramps`, `x/e/dry` history ages read by the kernel | P (relative to `pos`) |
| soft-clip | `left_defaults/right_defaults[lane]` | D |
| soft-clip | `ramps[..].current` | R (same word as the state value) |
| soft-clip | older history rows | never read before overwritten; restore zeroes them |
| soft-clip | `bypass`, `automation_capacity`, `width`, `quantum` | U |
| soft-clip | `pos` / `nonfinite` / `active` | B (clock) / X / A |
| transient-shaper | `env.fast/slow`, `ramps` | P |
| transient-shaper | `Ramps.defaults[lane]` | D |
| transient-shaper | `coefficients`, `bypass`, `link_mode`, `automation_capacity`, `state_sizes`, `width`, `quantum` | U (rate row, program key) |
| transient-shaper | `nonfinite` / `active` | X / A |
| true-peak-limiter | `history`, `main_ring`, `required_ring`, `box_ring`, `reduction`, `prefix`, `box_sum`, `phase`, `limit`/`release` ramps, `lookahead_ms` | P (rings relative to the cursors) |
| true-peak-limiter | `left_defaults/right_defaults[lane]` | D |
| true-peak-limiter | `lane` (`LaneShape`), `gain_linked` | R (`commit_lane`, `gain_state_agrees`) |
| true-peak-limiter | `width`, `sample_rate`, `automation_capacity`, `quantum`, `shape`, `coefficients`, `silent_bypass` | U |
| true-peak-limiter | `cursors`, `silent_fixed_point` / `report` / `active` | B / X / A |
| `EffectBankStage` | effect state | P + D (`swap_bank_lanes`) |
| `EffectBankStage` | `witness[lane]` | S |
| `EffectBankStage` | `bypassed`, `width`, `quantum`, `offsets`, `layout`, `carry_refused` | U / B |
| `LiveControlEffectBankStage` | effect state | P + D |
| `LiveControlEffectBankStage` | `lanes[lane]` (cells, applied bypass and crossfade, symmetry terms, carried writes), observe-queue `Consumer`, `observations[lane]` taps, `designed[lane]`, `unrestored` bit | S |
| `LiveControlEffectBankStage` | dry shunt latency-line words | S (`BypassShunt::swap_line_lanes`, each from its own cursor) |
| `LiveControlEffectBankStage` | `offsets`, `staging`, `packed`, `staged_spans`, `crossfade`, `samples`, shunt `dry_*` | X |
| `LiveControlEffectBankStage` | `width`, `quantum`, `layout`, shunt cursor, `unbound`, `carry_refused` | U / B |

### D3 Evidence (attempt 1)

- Gate 1: one randomized gate-1 test per effect (Simd4 and Simd8; every console factory builds a
  Simd4 bank on this AVX2 host through its crate-internal binder) and one whole-chain test
  (`rack/tests/lane_swap.rs`), swapping mid-ramp within one bank and across two, with per-lane
  live values; outputs `to_bits`-equal to never-swapped twins, payloads equal, and after a
  `FullToDefaults` reset still equal. The EQ case swaps lanes whose band kinds differ.
- Gate 2: every swap call is measured with `bench_support::alloc` thread counters after warm-up:
  0 allocations.
- Clock tests: EQ, limiter, soft clip and gate render N zero blocks against a twin that calls
  `advance_clocks`; every state and payload word is equal after each block.
- Not covered: the `designed`/`unrestored` bits and the agreement AND reach only the collapse
  witness, and the rack mock does not collapse.
