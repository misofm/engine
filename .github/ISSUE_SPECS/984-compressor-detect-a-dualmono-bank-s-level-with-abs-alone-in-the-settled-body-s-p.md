# Compressor: detect a DualMono bank's level with `abs` alone, in the settled body's pass 1


Compressor slice 4 of 5 (research 2026-09-27, base `6ca203f8`; builds on slices 1-3). Evidence:
`docs/handoffs/effects-2026-09-27/COMPRESSOR-DIAGNOSIS.md`. The prototype is mode 15 of
`compressor-diagnosis-prototypes.patch`. Neither patch may be committed.

## Product outcome

Tracks are dual-mono (AGENTS.md), and every compressor in the standing fixture is
`link_mode = dual_mono`. `link_frame` (`crates/compressor/src/kernel.rs:316-339`) nevertheless
computes, per frame:

* both magnitudes;
* their maximum;
* their average (two multiplies and an add);
* three selects on the link masks.

Nine vector operations produce `(|left|, |right|)`. This slice takes the `abs`-only arm when the
prepared link mode is DualMono, **inside the two-pass body's pass 1 only**.

Measured on the prototype (EPYC 7313P, pinned, in process), on top of slices 1-3:

* compressor-only minus builtins-only, 64 tracks:
  * native `Simd8`: **-9 %** (25.3 to 23.0 us per block);
  * wasm under V8: 0 to -4 % (61.6 to 59.3-61.5 us, within V8's run-to-run spread);
* kernel: 42.6 to 38.3 (`Simd8`), 41.9 to 38.4 (`Simd4`) and 50.3 to 48.4 (V8) cycles per
  channel-frame.

## Invariants

* **Class A.** Under DualMono, `link_frame` returns
  `select(linked = none, combined, magnitude) = magnitude` bit for bit. The arm computes `magnitude`
  directly, which is `abs` of the same load. No NaN relaxation is needed: `abs` is a sign-bit
  operation, and the selected value is the same word.
* The arm is keyed on `metadata.link_mode`, which is whole-instance and fixed at preparation. It
  is never keyed on data.
* **Pass 1 only.** The same arm in a one-pass loop regressed V8 by 18-40 % (prototype modes 3 and
  7: V8's register allocator spilled the recurrence differently). The two-pass body is the only
  place measured to be safe on both targets.
* `#[inline(always)]`. No new field. Allocation-free.

## Interface contract

In the two-pass body (slice 3), pass 1's detector becomes:

```rust
#[inline(always)]
fn settled_detect<L: Lane, const DM: bool>(ml: L, mr: L, inv: &Invariants<L>) -> (L, L) {
    if DM { (ml.abs(), mr.abs()) } else { link_frame(Detector::Main, 0, ml, mr, inv) }
}
```

`settled_main` sets `DM = matches!(link, LinkMode::DualMono)` once per block. That gives four
settled-body instantiations per width: `DM` times `WET`.

## Smallest closable slice

Authorized paths:

* `crates/compressor/src/kernel.rs`, including `settled_body_tests`;
* `crates/compressor/tests/MUTATIONS.md`;
* this spec.

Steps:

1. Confirm that slices 1-3's pinned digests exist.
2. Add the arm.
3. Gates, then the evidence record.

## Non-goals

* A Maximum or Average arm (`max` alone, or the average alone). Those are linked modes, not the
  product default. Record them as a follow-up if a linked fixture is ever benchmarked.
* The collapsed body (slice 5).

## Objective gates

1. **Old body is the oracle.** Slice 1's gate 1, now required to cover `DualMono`, `Maximum` and
   `Average` with the same hostile input, at `f32`, `Simd4` and `Simd8`, in dev and release.
   Compare by bits, except the wet arm's "both NaN" rule.
2. **Pinned scenarios.** The digests of slices 1-3 are unchanged. Slice 1's scenario already
   includes DualMono and Maximum.
3. **Existing gates stay green:** `cross_target` (cases `dual_mono_static`, `maximum_link`,
   `average_link` and `dual_mono_ramping`), `identity::average_link_is_two_products_and_an_add`,
   `partition::linked_sidechain_partitions_are_invariant`, `lane_identity`, `mono_collapse`.
4. **Console digests.** All 15 standing workloads, both dispatch widths: slice 1's gate 4.
5. **Mutations.** Apply each alone, record it red, and revert it:
   * M1: the arm is taken for every link mode. `cross_target` `maximum_link` and `average_link` go
     red, and so does gate 1.
   * M2: the arm returns `(|left|, |left|)`. Gates 1 and 2 and `cross_target` go red.
   * M3: the arm is never taken. There is no correctness gate; record it as performance-only. As
     in #944's M2, performance-only regressions are caught by the recorded codegen evidence (gate
     7), not by a test.
6. **Browser artifact, toolchain and policy.** As slice 1's gates 6 and 7. Record the artifact
   size, since there are now four settled instantiations per width.
7. **Codegen evidence** (recorded). Pass 1 of the DualMono instantiation has one `vandps` (abs)
   per channel and no `vblendvps` before the level floor. Quote it.

## Console benchmark rows

As slice 1. Native rows move; wasm rows move within their spread.

## Dependencies

Slice 3.

## Standing rules for the implementer

As slice 1.

## What the implementer will hit

* **V8 is fragile here.** Under V8, removing operations from a one-pass loop made it slower (mode 3:
  85.5 against 60.3 cycles per channel-frame). Keep the arm out of any loop that also holds the
  recurrence.
* **The mono body has its own link call.** `frames_loop_mono` evaluates `link_frame(main, main)`.
  Slice 5 owns that call; do not touch it here.

