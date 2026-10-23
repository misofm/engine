# Swap two strips' builtin stage state inside a bank without allocating

Split from #1747 (verifier finding 4, 2026-10-23). Needed by #1107.

## Problem

#1747 adds a render-time lane swap for effect bank stages, the live control stage and the dry shunt line (`BankStage::swap_lanes`, `BankChain::swap_lanes`). Production strip chains also hold `BuiltinStage` slots (`crates/graph/src/runtime.rs` near 5760 and 7900-7935). These keep the default `can_swap_lanes = false`, so `BankChain::swap_lanes` refuses every chain that the graph builds. That fails safe, but #1107 cannot pack a production console until builtin stages swap too.

The builtin stages carry per-lane state: the input section (polarity, trim, HPF, LPF and their ramps), the fader and mute ramps, the matrix ramps, timed operations, and their live control cells.

## Smallest closable slice

- Implement `can_swap_lanes` / `swap_lanes` for each builtin bank stage that a production strip chain holds: input section, fader, matrix. Include the fused fader+matrix stage only if it still exists when this starts (#1754's open owner question).
- Swap every per-lane field: state, coefficients, ramps (current, target, step, remaining), pending timed operations, live control cells and their dirty bits. Record a field class table in this spec, as #1747 D2 does.
- Allocate nothing. Decide every refusal before the first write.

## Authorized paths

- `crates/builtins/src/lib.rs`, `crates/builtins-compiler/src/lib.rs`, `crates/graph/src/runtime.rs` (only the builtin stage's `BankStage` impl)
- the tests of those crates
- this spec

## Objective gates

1. One randomized chain test on a graph-built strip chain: render N blocks, swap two strips (in one bank and across banks, mid-ramp, with a pending timed operation and different live values per lane), render M more with the inputs swapped too; every strip's output is bit-identical to the no-swap twin. Simd8 and, where built, Simd4.
2. The swap allocates nothing (`bench_support::alloc`, folded into the gate 1 test).
3. `cargo fmt --all --check`, the workspace clippy line, `tools/realtime-policy`.

## Test value

Gate 1 turns red when a builtin swap leaves one per-lane field behind, such as a ramp target or a pending timed operation. No test can see this today, because builtin stages refuse every swap.

## Dependencies

- #1747 (the swap contract).
- #1754 owner decision on the fused fader+matrix stage.
- Blocks #1107.

## Decision record

### D1 Design (attempt 1, 2026-10-23)

- **One source of truth: the plan-swap carry payload.** Each builtin bank (`BuiltinInputBank`,
  `BuiltinFaderBank`, `BuiltinMatrixBank`) swaps two lanes by exporting both lanes' carry payload
  (`InputLaneState`, `FaderLaneState`, `MatrixLaneState`) and importing them crosswise. The payload
  is already the complete, carry-tested record of a lane's running state, and `import_lane`
  re-derives every cache those words feed, so no second per-field list can drift. Chosen over a
  direct field swap for that reason (as #1747 D1 chose the effect payload). The payloads are
  `Copy` values on the stack: no scratch (`lane_swap_scratch_bytes` is 0) and no allocation.
- **Prepared per-lane endpoint.** The payload leaves the input stage's `filter_initial` (what a
  `FullToPrepared` reset returns to) with the stage, because a successor plan keeps its own. A
  swap moves it with the strip (class D below). The fader and matrix stages hold no prepared
  per-lane endpoint: their reset snaps to the live target.
- **Processor level** (`builtins-compiler`): the bank swap, then the lane's live cell reader
  (`controls[lane]`: cells, dirty bits, carry marks, superseded counter), the input bank's live
  symmetry terms (`live[lane]`), and the fader bank's stored automation (`automation[lane]`). The
  strip's live controls and automation move with the strip.
- **Graph trait** (root-approved edit of `crates/graph/src/lib.rs`):
  `GraphPreparedBuiltinBankProcessor::{can_swap_builtin_lanes, swap_builtin_lanes}`, both
  declining by default. `BuiltinStage` (`runtime.rs`) forwards `rack::BankStage::{as_any,
  can_swap_lanes, swap_lanes, lane_swap_scratch_bytes}`; a peer must be a `BuiltinStage`, and the
  bank decides that it is of its own concrete type (downcast) and width.
- **All or nothing.** Every refusal (lane not populated, other type, other width, a reader or
  automation slot out of range, automation that cannot land) is decided before the first write;
  after the check nothing can fail (debug-asserted).
- **Collapse.** The rack's `BankChain::swap_lanes` takes the disengage boundary before any slot
  swaps (#1747), so the input stage is never exported collapsed (debug-asserted). The gate-1 test
  covers a collapsed chain with real stages (#1747 limit R7).
- **Stated limit (automation slots).** A fader bank none of whose members is automated holds no
  automation slots (`automation` is empty, #1646 D1), so a lane carrying stored automation cannot
  move into it: that swap refuses (`ChainLaneSwapRefusal::Slot`). Note for #1107: packing moves an
  automated strip only into a fader bank that holds slots. Lifting it would give every fader bank
  of an automated plan its slots and its event-driver path; not done here.
- **Test hooks** (root-approved, `test-support` only): `graph::test_only_builtin_stage` and
  `graph::test_only_builtin_processor_mut`. The gate-1 chain is built from the real `BuiltinStage`
  adapter and the real bank processors (lowered by the real `lower_strip_banks`), not from a
  graph-compiled plan, because no plan exposes its chains yet. #1107 owes one plan-level swap test.

### D2 Per-lane field classes

Classes as #1747 D2: **P** payload; **D** prepared endpoint exchanged beside the payload; **S**
processor-level exchange; **R** re-derived on import from a moved field; **U** bank-uniform (reason
given); **B** bank-wide, not per lane; **X** per-block scratch; **A** padding (padded lanes never
swap).

| Bank | Field | Class |
|---|---|---|
| input stage | `coef.trim`, `coef.section` (HPF/LPF in use), `filter_target`, `filter_step`, `filter_remaining`, `ramp.{current,target,step,remaining}`, `remaining`, `state.section` integrators, `state.silence` | P |
| input stage | `filter_initial` (prepared endpoint of a full reset) | D |
| input stage | `symmetry` bit, `plan`, `ramping`, `filter_ramping` | R (`import_lane`) |
| input stage | `coef.silence` | U (one rate per bank) |
| input stage | `collapsed` | B (cleared by the chain's disengage before the swap) |
| input stage | `lifetime_recovered` | B (counter) |
| input stage | `members`, `active` | A |
| fader stage | `ramp[c].{current,target,step}`, `remaining`, `fader_gain`, `muted`, `mute_ramp` | P |
| fader stage | `ramp[c].mute` mask, `ramp[c].remaining` kernel word | R (`sync_mute`, countdown clamp) |
| matrix stage | `ramp.{current,target,step}`, `smoothing_samples`, `remaining` | P |
| matrix stage | `coef` with its `identity` mask, `ramp.remaining` kernel word | R (`sync_settled`, countdown clamp) |
| all banks | `backend`, `width`, `members` | U / A |
| input processor | `controls[lane]` (`InputReader`: cells, dirty reader, carry marks, superseded) | S |
| input processor | `live[lane]` (live symmetry terms) | S |
| fader processor | `controls[lane]` (`FaderReader`) | S |
| fader processor | `automation[lane]`: fader and mute cells (event cursors, held-ramp obligations, offsets cells, reposition), `arrival`, `fresh` | S |
| fader processor | `automation[lane].{span, fader_jump, mute_jump, vca_jumps}` | X (set by `set_block_time` before every block) |
| fader processor | `automation[lane].program` | U (one program per plan) |
| matrix processor | `controls[lane]` (`MatrixReader`) | S |
| every processor | `process_calls`, `frames_processed` | B (counters) |
| `BuiltinStage` | (holds only the processor) | - |
