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

## Decision record

### D1 Diagnosis (attempt 1, 2026-10-23)

Method: one bank, every lane playing a 0.5 tone, then exact `+0.0`; the samples until
`lanes_at_rest` covers the lane, with the stalling word traced. 48 kHz unless stated.

| Effect | Word that holds rest back | Why | Time to rest |
|---|---|---|---|
| Compressor | `gain_reduction_db` (dB) | release `flush(fma(c, 0 - g, g))` (`kernel.rs:401`, `effect-runtime/src/envelope.rs:173`) is an exact geometric decay with no stall: no `-0.0`, no fixed point above the flush floor. It lands on `+0.0` only at `FLUSH_EPS = 1e-20` dB (`lane/src/lib.rs:177`). `ln(7.84 / 1e-20) * 4800 = 230,900` | defaults (-18 dB, 100 ms): 230,928; -40 dB: 236,352; 5 ms release: 11,424 |
| Transient shaper | the slow follower `slow` (linear) | release `flush(c * e + k * 0)` (`envelope.rs:160`), retention of 100 ms: geometric down to `1e-20`. The output is the dry word once both followers are below `FLOOR = 1e-8` (`lib.rs:229`): 84,480 samples | 217,088; 96 kHz: 433,408 |
| Gate/expander | `gain_db` | release `flush(fma(c, t - g, g))` toward `t = -range` (`kernel.rs:369`) stops at its own float fixed point, where `c (t - g)` is below half an ulp of `g` (-79.98 dB for -80). `channel_at_rest` (`kernel.rs:195`) accepts that word. Hold, then about `ln(2 c range / ulp) / c` | defaults: 44,544 (100 ms hold + release); 96 kHz: 82,304; 0 dB range: 4,864 |
| Soft clip | the `e` history | no recursion: `X` and dry clear in 32 pushes, the last non-zero `e` is pushed on silent frame 29 and leaves 32 pushes later | 62 (bound 62) |

No effect has a stall bug. Each rest time is its own law: the compressor and the transient shaper
reach `+0.0` only at the D7 flush floor, about 400 dB under any audible level, and the gate stops
at the fixed point of its rounding.

### D2 Class A: no fix

A class-A landing needs a state that leaves every later output bit unchanged for every later input.
Nothing earlier than the existing flush meets that:

- Compressor: `fast_gain_from_db(g)` is exactly `1.0` only for `g == +0.0` (measured: the first
  unity-gain sample is the rest sample). Every non-zero word still scales signal by `1 - ulp` or
  less. A snap moves the attack path's bits on the next signal, and `gain_mix`'s dry-identity
  select (`smoothed == 0`) changes how a `-0.0` input comes out at `mix < 1`.
- Transient shaper: below `FLOOR` the output does not depend on the followers during silence. But
  on the next signal `c * e + k * u` rounds differently for `e = 1e-9` and `e = 0`, so a snap at
  `FLOOR` moves bits.
- Gate: the trajectory to the fixed point is bit-defined. Landing sooner changes it.

So this attempt changes no source. The tests and the existing differentials and digests are
unchanged by construction.

**Class B, for root (not implemented):**

- B1 compressor: snap `gain_reduction_db` to `+0.0` below a per-effect floor (for example where
  `fast_gain_from_db` is within one ulp of `1.0`).
- B2 transient shaper: snap a follower to `+0.0` below `FLOOR`. Rest would then come at about
  84,480 samples (measured, see D1) instead of 217,088.

Both move the low bits of output after the next signal, the digests included.

### D3 Soft clip: collapse and fold

- Collapse: the soft clip does not implement the mono trio. `supports_mono_collapse` keeps the
  default `false` (`effect-contract/src/lib.rs:2414`), so a chain with it never collapses. The
  gate/expander and the transient shaper do not implement it either: only parametric-eq, the
  compressor and the true-peak limiter do. Writing the trio would be class A and inside the
  crate. But it is a new feature: `process_bank_mono`, `desymmetrize_channels` and
  `lane_channel_symmetry`, plus a new kernel in the wasm roster. That is outside a rest fix, so
  it is a successor.
- Fold: not measured, and outside the crate. The soft clip has a 31-sample latency
  (`lib.rs:237`). `foldable_lane` refuses a route whose input carries a PDC delay
  (`graph/src/runtime.rs:9910`), which is the likely reason for a mixed session. This needs a
  graph-side check.

### D4 Gate 1 tests and their bounds

Each bound is derived in the test's doc comment. `u = 2^-24`. Rest is read at block ends, so each
bound is rounded up to one.

- Compressor: `a_compressing_lane_lands_at_rest_within_its_release_bound` (new). The bound is
  `ln(|g| / FLUSH_EPS) / (-ln(1 - c(1 - u)) - u) + 1`. Measured 230,928 against 230,990. At the
  bound to the block for thresholds -18 to -80, at W4 and W8.
- Transient shaper: `a_lane_that_followed_a_tone_lands_at_rest_within_its_release_bound` (new).
  The bound is `ln(e / FLUSH_EPS) / (-ln c - u) + 1` per follower. Measured 217,088 against
  217,095 to 217,109.
- Gate/expander: folded into `the_silent_fast_path_renders_the_kernel_bits`. The bound per lane is
  `hold + 1 + 24 ln 2 / -ln(1 - c(1 - u)) + 3`: the distance to the stall point is below `2^24`
  grid units. This replaces `< RELOAD_BLOCK` (19,200 samples) as the per-lane limit. Measured
  2,736 to 6,912 against 4,032 to 11,184.
- Soft clip: folded into `the_silent_fast_path_renders_what_the_kernel_renders`. The bound per
  lane is `max(DRY_DELAY - 1 + 32, ramp end)`, which is tight to the block.

Before and after are the same, because no source changed.
