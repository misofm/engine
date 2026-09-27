# Keep an EQ identity section with a frozen state elidable


EQ optimisation, slice 4 (research 2026-09-27, base `6ca203f8`). Evidence:
`docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md`, section 5 (EQ-4); the measurement is
`docs/handoffs/effects-2026-09-27/eq-disable-cliff-test.patch`.

## Product outcome

Elision leg (b) requires every elided identity section to hold exactly `+0.0` in both integrators
(`section_state_is_positive_zero`, `crates/parametric-eq/src/lib.rs:975`, used at `:1715-1716`
and in `cascade_sections_mono` at `:1530`). An identity section never moves its state: `c1 = a2 = a3 = +0.0`
make every increment a signed zero, so whatever state the section held when it became the identity
stays there. That happens every time a dedicated cut is switched on and then off through a prepared
target (#807): the ramp ends on the identity words with a non-zero state. From then on leg (b)
refuses the **whole bank** on every block, and the bank renders all six sections until a reset.

Measured with the patch above (`Simd8`, a band switched on at block 2 and off at block 5): the
shipped rule elides on **0 of the 9** blocks after the disable; the relaxed rule below elides on
**9 of 9**, and its output words and integrator words equal the full cascade's. An affected bank
costs about 14,600 more cycles per block natively (six sections instead of one; +3.9 us per 8
tracks) and about 18,200 more in the browser (+4.9 us per 4 tracks). A restored payload with a
non-zero state in a disabled section has the same effect.

## Why the relaxed leg is exact

Take an identity section (exact identity words, no ramp) whose every integrator word is `+0.0` or
finite with magnitude at least `FLUSH_EPS`, on an admitted block (leg (a): finite, within
`BLOCK_LIMIT`, no `-0.0`).

* `v3 = v0 - ic2` is finite (`|v0| <= 1e30` cannot push a finite `ic2` past `f32::MAX`).
* `d1 = (-0.0 * ic1) + (+0.0 * v3)` and `d2 = (+0.0 * v3) + (+0.0 * ic1)` are zeros of some sign.
* `ic1' = flush(ic1 + (d1 + d1))`: a non-zero `ic1` absorbs the zero and `flush` keeps it
  (`|ic1| >= FLUSH_EPS`); `ic1 = +0.0` gives `+0.0`. Likewise `ic2`. **The state is unchanged.**
* `y = (+0.0 * v2) + ((+0.0 * v1) + 1.0 * v0)`: `v0` for every finite `v0 != 0`, and `+0.0` for
  `v0 = +0.0` whatever the signs of the zero terms. **The output is the input.**
* A dedicated cut additionally selects `v0` through its dry mask, which gives the same bits.

So executing the section and eliding it leave the same output and the same state, and the section
passes no `-0.0` on, which is what legs (a) and (c) rely on downstream. A word with magnitude below
`FLUSH_EPS`, a `-0.0`, or a non-finite word is still refused: the executed recurrence would flush
or propagate it, and the elided one would not.

## Lessons carried from #944

Dev and release; NaN words compared as "both NaN, or equal bits"; scenario test pinned on base
first; mutations recorded red in `crates/parametric-eq/tests/MUTATIONS.md`.

## Invariants

- **Class A.** Every rendered word and every integrator word is unchanged on every block.
- Legs (a) and (c) are unchanged. Only leg (b)'s predicate widens.
- `a_tiny_restored_disabled_cut_state_refuses_elision_but_preserves_old_bands` (`lib.rs:3663`) stays
  green unchanged: a restored word below `FLUSH_EPS` still refuses.
- Allocation-free, no new field.

## Interface contract

1. Replace `section_state_is_positive_zero` with `section_state_is_inert<L: Lane>(section) -> bool`:
   true when every lane of `ic1` and `ic2` has bits `0`, or magnitude bits in
   `[FLUSH_EPS.to_bits(), 0x7f80_0000)`. Read the bits with `store_bits`, as the current helpers do.
2. `cascade_sections` and `cascade_sections_mono` call it for dead sections.
3. Amend the proof in `cascade_sections`' doc comment (`:1599-1682`): the induction starts from an
   inert state, not from `+0.0`, with the argument above.

## Smallest closable slice

Authorized paths: `crates/parametric-eq/src/lib.rs` (the helper, the two call sites, the doc, the
`elision` test module); `crates/parametric-eq/tests/bank.rs` (gate 1);
`crates/parametric-eq/tests/MUTATIONS.md`; this spec.

## Non-goals

Clearing a disabled section's state (that moves bits for `-0.0` input), caching the legs per block
(EQ-7), any kernel change.

## Objective gates

1. **Scenario (public API, pinned on base).** `a_cut_switched_off_keeps_the_bank_eliding` in
   `tests/bank.rs`: `native_bank()` and scalar; enable the HPF on every lane with a prepared target,
   render 4 blocks, disable it (a target with `enabled = 0`), render 12 more blocks of hostile input
   (subnormals, `+0.0`, `2^-24..2^25`; two blocks with `-0.0`). SHA-256 of outputs and snapshots,
   pinned on base.
2. **Engagement (in-crate).** Port the patch's `diag_disable_cliff` as a test: a general band and,
   separately, the HPF switched on and off on every lane of both channels; assert the stationary
   blocks after the ramp all elide (`kept < EQ_SECTION_COUNT`) and that the rendered words and
   integrators equal the per-section path's, at `f32`, `Simd4` and `Simd8`.
3. **Refusals (in-crate).** A dead section holding a `-0.0` word, a word of `1e-30`, or (through
   `restore_track`) the largest finite word: `-0.0` and `1e-30` refuse; the largest finite word is
   admitted and stays bit-exact against the per-section path.
4. `cargo test -p parametric-eq`, dev and release; `chain_shape` and every `WORKLOADS` digest
   unchanged (the standing fixtures never disable a section mid-run).
5. **Mutations**, each alone, red:
   - M1: back to the exact `+0.0` test: gate 2's engagement assertion.
   - M2: accept magnitudes below `FLUSH_EPS`: gate 3 (`1e-30`: integrators differ).
   - M3: accept `-0.0`: gate 3 (integrators differ: the executed path flushes it to `+0.0`).
6. Toolchain and policy scripts as EQ-1 gate 7.

## Console benchmark rows

No standing row switches a cut mid-run: expect no move. A row that toggles a cut is a separate
tooling issue.

## Dependencies

None. EQ-7 (caching the legs) lands after this one.

## Standing rules for the implementer

As EQ-1.

## What the implementer will hit

- **The identity flag is per section across all lanes of both channels.** The cliff needs the
  section dead on every lane; a cut left live on one lane keeps the section live and never reached
  leg (b). The gates switch it off on every lane.
- **The ramp ends inside `process_section`**, which snaps the words and refreshes `identity`; the
  first stationary block after it is where leg (b) is asked.
- **The old predicate has a test caller** (`lib.rs:3396`, which asserts a state is exactly `+0.0`).
  Keep that assertion's meaning: move the exact-zero helper into the test module rather than
  pointing the test at the widened predicate.

