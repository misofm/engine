# EQ: refuse elision on a restored subnormal integrator in the stationary cascade (class-A gap)

## Problem

The stationary EQ cascade elides dead (identity) sections on admitted blocks. Its leg (c) admits a live section whose integrators are finite and not `-0.0` (`section_state_is_finite_without_negative_zero`, `crates/parametric-eq/src/lib.rs`, used by `cascade_sections` and `cascade_sections_mono`). That admits a **restored subnormal** integrator, and the `-0.0` induction behind elision assumes kernel-written states (the kernel never writes a subnormal; it flushes below `FLUSH_EPS`).

Found by the #1005 implementer and reproduced on batch code `a1fcab3d`: one live high shelf at `m0 = m2 = 0.5` (gain `-6.0206` dB as an `f32`), restored `ic1 = ic2 = -2^-149`, input `-2^-149`, every other section dead. The elided cascade renders `0x80000000` (`-0.0`); the full cascade renders `0x00000000` (`+0.0`). The live section emits `-0.0` on its first frame, and an elided identity after it passes the `-0.0` where the executed identity would write `+0.0`.

It needs a restore payload carrying a subnormal integrator, so it is not reachable from rendering alone; it is inaudible (a signed zero), but it breaks the engine's class-A guarantee that elision renders the full cascade's bits.

## Smallest closable slice

Close it in one of the two ways the finding names, whichever is simpler and cheaper at render:

1. tighten the stationary leg (c) to #1005's `section_state_is_flush_shaped` (every integrator of a kept section is `+0.0` or finite with `|x| >= FLUSH_EPS`), so a restored subnormal refuses the block, the refused block flushes the words, and the next block engages; or
2. make `restore_track` refuse (or flush to `+0.0`) a non-zero integrator below `FLUSH_EPS`, if restore semantics allow it (check the effect contract's restore rules first; a restore must not silently change state it accepted before without a documented reason).

Coordinate with #998 (which caches leg (b) and is in review) and #1005 (which added `section_state_is_flush_shaped` for the ramping lists): land after whichever of them merges first, and reuse #1005's helper.

## Objective gates

1. A test pinned on the batch head reproduces the gap (elided `-0.0` against full `+0.0`) and passes after the fix, at `f32`, `Simd4` and `Simd8`, dev and release.
2. The #977/#979 differentials and the #1005 differential stay bit-identical; every `WORKLOADS` digest is unchanged.
3. Mutation: reverting to the old leg (c) turns gate 1 red.
4. Timing (under the timing lock, built first outside it): the shipped `host_web.wasm` one-band, two-band and builtins isolates, and native `Simd8`/`Simd4` `eq_only`, no slower than the batch head; `scripts/run-wasm-gates.sh` (spill gate) green, and the V8 listing scan over all EQ loops, masked included, shows no new carried slot.
