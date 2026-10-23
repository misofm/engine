# Make console effects reach exact rest after real signal

Found by #1107 attempt 2 (2026-10-23, D7 "Coverage and reach limits").

## Problem

#1107 skips a console bank only when every lane reports `lanes_at_rest` (#1745, #1746, #1781). Several console effects reach that state slowly or never after they have processed signal, so the skip rarely engages on a real session:

- **Compressor:** a lane that has reduced gain did not report rest again within 300 blocks; with thresholds at or below about -18 dB it did not within 200,000 samples. Suspected cause: `gain_reduction_db` settles at `-0.0` or stalls short of `+0.0` (not yet diagnosed).
- **Transient shaper:** did not reach rest within 200,000 samples.
- **Gate/expander:** about 50,000 samples at its defaults.
- **Soft clip:** stops the chain's mono collapse as a console slot and the route fold as an insert.

## Smallest closable slice

1. **Diagnose first (record per effect before any change):** for each effect, trace why its state does not reach the exact fixed point after a tone then silence (which word stalls, at what value, and why: `-0.0`, a subnormal flushed to a nonzero floor, a smoother that converges geometrically and never lands, a counter).
2. **Fix only where the effect's own arithmetic allows an exact landing without changing rendered bits before rest** (class A), for example snapping a word to `+0.0` when its flush rule already makes it inaudible, as the effect's flush policy states. Any change that moves rendered bits is class B and goes to root first.
3. Record each effect's measured time to rest (tone, then silence) before and after.

## Authorized paths

- `crates/compressor/`, `crates/transient-shaper/`, `crates/gate-expander/`, `crates/soft-clip/` (source and tests)
- this spec

## Objective gates

1. Per effect: a test that renders a tone then silence and asserts the lane reports rest within a stated bound (the bound recorded with its derivation).
2. Class A: the effect's existing differential and digest tests unchanged, unless root rules a class B change.
3. fmt, clippy, the AArch64 test check, cargo doc -D warnings, `scripts/check-web-audioworklet.sh`.

## Test value

Gate 1 turns red when an effect's state stops landing on its exact fixed point after real signal, which no current test checks (the rest probes start from planted or near-rest states).

## Dependencies

#1107 merged.
