# EQ: fold the 4.4 boundary scan into the cascade's final store on admitted stationary blocks

## Product outcome

After the stationary EQ cascade renders an admitted block, `render` re-reads both output planes in `check_block` to apply the 4.4 block-limit rule (`|y| < BLOCK_LIMIT`, otherwise zero the block and reset). The cascade's last pass already holds every output word in a register when it stores it. EQ-6 in `docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md` measured folding that check into the final store at about -294 cycles per bank under V8 (about -1.3 us per 64-track block) and -125 natively at `Simd8` (about -0.3 us). The depth-1 kernel is latency-bound, so the extra compare and `and` per vector ride free there.

## Smallest closable slice

For an admitted stationary block only, the last pass (depth-2 pair or depth-1 tail) accumulates the `abs(y) < BLOCK_LIMIT` verdict per stream as it stores, and `render` uses that verdict instead of calling `check_block`. A zero-section plan, a refused block and every ramped block keep `check_block` unchanged. This needs a verdict-returning variant of the lane kernel in `crates/lane`. Class A: the verdict, the zeroing and the reset must be the same on every block.

**Wasm hazard.** #977 showed that V8's register allocation of the one-band depth-1 tail depends on the code around the loop: attempt 1 spilled an integrator and ran +21 % slower in the shipped artifact while every in-crate timing looked fine. This change touches that loop. Measure through the shipped `host_web.wasm` render export and inspect the V8 listing for a carried stack slot (the #977 verifier's harness: `v8loops.py`), not only a private wasm guest.

## Objective gates

1. **Verdict equality.** A test asserts the folded verdict equals `check_block` on the same output planes for every admitted stationary block, including outputs exactly at, just below and just above `BLOCK_LIMIT`, `+-inf` and NaN produced from admitted input where reachable, at `f32`, `Simd4` and `Simd8`, dev and release. Compare NaN words as "both NaN" where the compiler may commute.
2. **Bit-identity.** Rendered words, integrators and the reset behaviour equal the batch head's on the #977/#979 differentials and a block-limit scenario pinned on the batch head. Every `WORKLOADS` digest unchanged.
3. **Mutations** (each alone, red): the folded verdict ignores the second stream; uses `<=` instead of `<`; is used on a refused block.
4. **Timing, no regression** (under the timing lock): shipped `host_web.wasm` one-band, two-band and builtins isolates, and native `Simd8`/`Simd4` `eq_only` isolates, none slower than the batch head; the V8 listing of the one-band tail carries no stack slot.
5. Clippy, fmt, `scripts/check-web-audioworklet-callgraph.py` (rule 3 in particular) and the wasm gates pass. Render stays allocation-free.
