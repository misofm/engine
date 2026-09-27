# Run every pass of an admitted stationary EQ plan select-free


EQ optimisation, slice 2 (research 2026-09-27, base `6ca203f8`). Evidence:
`docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md`, sections 1, 2 and 5 (EQ-2). A tested prototype
of EQ-1 plus this change is `docs/handoffs/effects-2026-09-27/eq-variant-no-padding-select-free.patch`.

## Product outcome

Every stationary pass runs `svf_cascade_interleaved_with_dry_masks` (`crates/parametric-eq/src/lib.rs:1780`
in `interleave`, and its mono twin), and builds its masks per pass per block through `dry_mask`
(`:1136`), which extracts each lane's six coefficient words through the stack. A general band is
never dry, so for any session with two live general bands every select is a no-op. They still cost
10 % of the pass natively (`vblendvps`) and 13 % in the browser, where V8 rebuilds each mask inside
the frame loop (`vbroadcastss`, three `vinsertps`, `vcmpps`, then a three-instruction bitselect: 32
instructions per frame for four no-op selects).

On an **admitted** block (the list `cascade_sections` returned is shorter than six), every select is
a no-op, including a dedicated cut's dry lanes. Run those passes select-free and build no mask. A
refused block (all six sections) keeps today's masked kernel.

## Why this is exact

Admission (leg (a), `block_admits_elision`, `:997`) means both planes are finite, within
`BLOCK_LIMIT`, and free of `-0.0`. A dry lane is a dedicated cut holding the exact identity words
(`c1 = a2 = a3 = m1 = m2 = +0.0`, `m0 = 1.0`) with no ramp. For any finite state its step gives
`v1 = ic1 + (+-0)` and `v2 = ic2 + (+-0)`, and its wet output is
`(+0 * v2) + ((+0 * v1) + 1.0 * x)`. For a finite `x != -0.0` that is `x`, bit for bit: a nonzero
`x` absorbs the signed zeros, and `x = +0.0` gives `+0.0` for either sign of the zero terms. The
state update never reads the mask. So `select(dry, x, wet)` returns `wet`'s bits on every lane.

The prototype passed `cargo test -p parametric-eq` in dev and release (mixed cuts, signed-zero
refusals, disabled-cut oracles, mono collapse, partition invariance); the only failures were EQ-1's
kept-count pins.

## Lessons carried from #944

Dev and release; NaN words compared as "both NaN, or equal bits"; the scenario test written and
pinned on the base first; every mutation recorded red in `crates/parametric-eq/tests/MUTATIONS.md`.

## Invariants

- **Class A.** Every rendered word and every integrator word is unchanged.
- A refused or all-live block (the full six-section list) keeps the masked kernel: its input may
  carry `-0.0` or a non-finite word, and there the select is not a no-op.
- The non-stationary path (`process_block`/`process_section`) is untouched.
- Allocation-free; no new field; kernels stay `#[inline(always)]` (wasm `KERNEL_ROSTER` rule 1).

## Interface contract

1. `interleave` and `interleave_mono`: `let admitted = length < EQ_SECTION_COUNT;`. When
   `admitted`, every pair calls `svf_cascade_interleaved::<L, S, 2>` and the depth-1 section (EQ-1)
   calls `svf_cascade_interleaved::<L, S, 1>`; no `dry_mask` is evaluated. Otherwise today's code.
2. `cascade_sections`/`cascade_sections_mono` are unchanged: "shorter than six" is exactly
   "the gate admitted and at least one section was elided".

## Smallest closable slice

Authorized paths: `crates/parametric-eq/src/lib.rs` (`interleave`, `interleave_mono`, their docs,
one `#[cfg(test)]` counter of masked passes); `crates/parametric-eq/tests/bank.rs` (gate 1);
`crates/parametric-eq/tests/MUTATIONS.md`; the ruling's EQ inventory paragraph on selects
(`docs/rulings/effect-floor-accounting.md`, "EQ inventory"); this spec.

Steps: (1) gate 1 on the base, digest pinned; (2) contract 1; (3) gates.

## Non-goals

The skewed kernel (EQ-3), the refused path's masks, `dry_mask` itself (it stays for refused
blocks and ramps), any `crates/lane` change.

## Objective gates

1. **Scenario (public API, pinned on base).** `admitted_blocks_render_the_base_bits_without_selects`
   in `tests/bank.rs`, scalar and `native_bank()`: two live general bands (a depth-2 pass), plus the
   HPF live on half the lanes (dry lanes on the others) after an enable-then-disable of those
   lanes' HPF through prepared targets, so the dry lanes hold a frozen non-zero state. 32 blocks of
   hostile input (subnormals, `+0.0`, `2^-24..2^25`), some blocks with `-0.0` or a non-finite word
   (refused). SHA-256 of every output word and every lane snapshot, pinned on base.
2. **Counter.** The `#[cfg(test)]` masked-pass counter reads 0 across gate 1's admitted blocks and
   is non-zero on its refused blocks.
3. `cargo test -p parametric-eq`, dev and release; `chain_shape` and the before/after digests of
   every `WORKLOADS` row, as in EQ-1 gate 3.
4. **Mutations**, each alone, red:
   - M1: `admitted = true` always: gate 1 (a refused block with `-0.0` on a dry lane renders `+0.0`).
   - M2: `admitted = false` always: gate 2.
   - M3: skip the depth-1 arm's `admitted` branch only: gate 2.
5. **Browser artifact.** As EQ-1 gate 5. Record V8's loop for the admitted pass (`node --no-liftoff
   --print-wasm-code`): no `vinsertps`/`vcmpps eq` mask rebuild in it.
6. Toolchain and policy scripts as EQ-1 gate 7.

## Console benchmark rows

The standing fixtures have one live band, so after EQ-1 their pass is already select-free: expect no
row to move more than noise. The saving shows on sessions with two or more live bands (replica:
native W8 5,878 -> 5,301 cycles per two-section pass, V8 7,062 -> 6,125). A multi-band row is a
separate tooling issue.

## Dependencies

EQ-1 (the depth-1 arm this issue makes select-free on admitted blocks).

## Standing rules for the implementer

As EQ-1.

## What the implementer will hit

- **"Admitted" is a list length, not a flag.** A six-entry list is either refused or all-live, and
  neither scanned the input for `-0.0`; both must keep the masks.
- **`dry_mask` looks cheap in the source and is not**: per dedicated section it extracts `W x 6`
  lane words through a stack array, and on this path it runs per pass per block. Do not call it on
  the admitted path at all.
- **The mono body** (`interleave_mono`) takes the same rule; the collapsed test suite covers it.

