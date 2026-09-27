# Software-pipeline the EQ's depth-2 cascade passes


EQ optimisation, slice 3 (research 2026-09-27, base `6ca203f8`). Evidence:
`docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md`, sections 3 and 5 (EQ-3). The timed prototype is
`docs/handoffs/effects-2026-09-27/eq-variant-skewed-depth-2.patch` (a local kernel in
`parametric-eq`); this issue puts the kernel in `crates/lane`, where the SVF lives.

## Product outcome

`svf_cascade_interleaved_impl` (`crates/lane/src/kernels.rs:259`) runs, per frame, section 0 then
section 1 of each stream, and section 1 consumes section 0's output of the same frame. Each
frame's work is therefore a dependency chain two sections long, roughly twice the 20-cycle
recurrence on the bench host (Zen 3), and the out-of-order window does not overlap enough of it
across frames: a depth-2 pass runs 10.3 cycles per section-frame at `Simd8` against about 9.0 for
the best arrangement measured.

Skew the pass: in iteration `i`, section `k` of every stream processes frame `i - k`, taking its
input from section `k - 1`'s output of the previous iteration (held in a register). The sections of
one iteration are then independent. Kernel replicas, per two-section bank-block:

| target | today, select-free | skewed, select-free | today, masked | skewed, masked |
|---|---:|---:|---:|---:|
| native `Simd8` | 5,301 | 4,616 (-13 %) | 5,883 | 4,828 (-18 %) |
| native `Simd4` | 4,953 | 4,815 (-3 %) | 5,691 | 5,059 (-11 %) |
| wasm `simd128`, V8 | 6,124 | 5,021 (-18 %) | 7,062 | 5,963 (-16 %) |

A clean browser build of the prototype moved today's padded fixture's EQ isolate from 43.2 to 36.9
us per 64-track block, and a clean native build by -1.3 us. After EQ-1 the standing fixtures have no
depth-2 pass; the saving lands on sessions with two or more live sections per bank.

## Lessons carried from #944

Dev and release; NaN words compared as "both NaN, or equal bits"; scenario test pinned on base
first; mutations recorded red in `crates/lane/tests/MUTATIONS.md`.

## Invariants

- **Class A.** Each `(stream, section)` chain runs `svf_step` and the output mix
  `m2.fma(v2, m1.fma(v1, m0.mul(x)))` on exactly the inputs, in exactly the order, it does today;
  with a dry mask, the same `L::select(mask, x, wet)` at the same section boundary. Only the
  interleaving of independent chains changes. State words leave the kernel identical.
- A frame is read before any section writes it back (the last section writes frame `i - (D - 1)`
  in iteration `i`, after section 0 has read frame `i`).
- `frames < D` falls back to today's kernel. No allocation, no `unsafe`, `#[inline(always)]`.
- `svf_cascade_interleaved` and `svf_cascade_interleaved_with_dry_masks` are not edited: they stay
  the oracle.

## Interface contract

1. `crates/lane/src/kernels.rs`, beside `svf_cascade_interleaved_with_dry_masks`:

   ```rust
   /// [`svf_cascade_interleaved`] scheduled as a software pipeline: in iteration `i`, section `k`
   /// of every stream runs frame `i - k`. Same chains, same order per chain, same bits.
   #[inline(always)]
   pub fn svf_cascade_skewed<L: Lane, const S: usize, const D: usize>(
       io: [&mut [f32]; S], frames: usize,
       c: &[[SvfCoef<L>; D]; S], s: &mut [[SvfState<L>; D]; S],
   )
   /// The same with [`svf_cascade_interleaved_with_dry_masks`]'s per-section dry selection.
   #[inline(always)]
   pub fn svf_cascade_skewed_with_dry_masks<L: Lane, const S: usize, const D: usize>(
       io: [&mut [f32]; S], frames: usize,
       c: &[[SvfCoef<L>; D]; S], s: &mut [[SvfState<L>; D]; S],
       dry_masks: &[[L::Mask; D]; S],
   )
   ```

   Body: a prologue (iterations `0..D-1`), a branch-free steady state (iterations `D-1..frames`,
   sections visited from `D-1` down to `0` so each consumes the previous iteration's carry before it
   is overwritten), an epilogue (iterations `frames..frames+D-1`). The prototype's body is in the
   patch named above.
2. `crates/parametric-eq/src/lib.rs`: `interleave` and `interleave_mono` call the skewed kernels for
   their depth-2 passes (masked or select-free per EQ-2). Depth-1 passes are unchanged.

## Smallest closable slice

Authorized paths: `crates/lane/src/kernels.rs` (the two kernels and their docs);
`crates/lane/tests/g2_kernel_identity.rs` (gate 1); `crates/lane/tests/MUTATIONS.md`;
`crates/parametric-eq/src/lib.rs` (`interleave`, `interleave_mono`); `crates/parametric-eq/tests/bank.rs`
(gate 2); this spec.

## Non-goals

A different depth (`SVF_CASCADE_DEPTH` stays 2), cross-bank fusion, op-by-op interleaving (measured:
no gain over skewing), any change to the ramped kernels or to the multiband crossover's `svf_step`
use.

## Objective gates

1. **Kernel identity (lane).** `g2_skewed_cascade_equals_the_interleaved_cascade`: for `L` in `f32`,
   `Simd4`, `Simd8`, `S` in `{1, 2}`, `D` in `{1, 2, 3}`, masked and select-free, frame counts
   `{1, 2, 3, 128, 1024}`, over `ALL_SIGNALS` plus a hostile family (both signed zeros, subnormals,
   a NaN payload, an infinity), 40 consecutive blocks with carried state: output words and final
   state words equal to `svf_cascade_interleaved[_with_dry_masks]` ("both NaN, or equal bits"), guard
   words untouched, under `CanonicalFpEnv::enter()`. Dev and release.
2. **Scenario (EQ, pinned on base).** `two_and_four_live_sections_render_the_base_bits` in
   `tests/bank.rs`: 2, 4 and 6 live sections, scalar and `native_bank()`, 32 hostile blocks,
   SHA-256 of outputs and snapshots pinned on base. `cargo test -p parametric-eq` green.
3. **Rows.** `chain_shape` green and every `WORKLOADS` digest unchanged (as EQ-1 gate 3).
4. **Mutations**, each alone, red:
   - M1: visit the sections `0..D` ascending in the steady state (the carry is overwritten before
     use): gate 1.
   - M2: store section `D - 1`'s output to frame `i` instead of `i - (D - 1)`: gate 1.
   - M3: drop the epilogue: gate 1 (the last `D - 1` frames).
5. **Browser artifact.** As EQ-1 gate 5 (rule 1 must still find one arithmetic-carrying EQ
   `process_bank`), **plus a clean-build wasm timing**: build the console guest at base and with the
   change, run both in one Node process on `sixty_four_track_eq_only` with the EQ-1 fixture and a
   two-band variant, and attach the numbers. A build that also carries other kernel variants is not
   evidence: the diagnosis's flag build outlined `render` and measured the prototype +13 us slower,
   while the clean build measured it 6.3 us faster.
6. Toolchain and policy scripts as EQ-1 gate 7; `bash scripts/check-lane-policy.sh`.

## Console benchmark rows

After EQ-1, no standing row has a depth-2 pass: expect no move. A multi-band row is a separate
tooling issue.

## Dependencies

EQ-1 and EQ-2 first (they decide which passes are depth 2 and which are select-free).

## Standing rules for the implementer

As EQ-1. Only `crates/lane` may name `wide`; the new kernels use the `Lane` trait alone.

## What the implementer will hit

- **V8 is the risk.** Sixteen `xmm` registers hold 2 x 2 sections' state, the carries and some
  coefficients; the rest are reloaded. The clean prototype still won 16-18 % there, but check the
  emitted loop (`node --no-liftoff --print-wasm-code`) for new spills.
- **`Simd4` natively gains little select-free (-3 %)**; that is not a reason to specialise by width.
- **The multiband crossover** calls `svf_step` directly and is not part of this change.

