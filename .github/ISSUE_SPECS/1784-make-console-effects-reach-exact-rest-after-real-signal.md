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

Method: one bank, every lane playing a 0.5 tone, then exact `+0.0`. Count the samples until
`lanes_at_rest` covers the lane, and trace the word that holds it back. 48 kHz unless stated.
These are the base (pre-D5) laws.

| Effect | Word that holds rest back | Why | Time to rest (base) |
|---|---|---|---|
| Compressor | `gain_reduction_db` (dB) | The release `fma(c, 0 - g, g)` (`kernel.rs`, `ballistic`; `effect-runtime/src/envelope.rs:173`) is an exact geometric decay. It does not stall: no `-0.0`, no fixed point above the floor. It landed on `+0.0` only at `FLUSH_EPS = 1e-20` dB (`lane/src/lib.rs:177`): `ln(7.84 / 1e-20) * 4800 = 230,930` | defaults (-18 dB, 100 ms): 230,928 with the committed test's tone, 230,976 with the diagnostic driver's; -40 dB: 236,352; 5 ms release: 11,424 |
| Transient shaper | the slow follower `slow` (linear) | The release `c * e + k * 0` (`envelope.rs:160`) at the 100 ms retention is geometric down to `1e-20`. While the input is silent, the output is the dry word once both followers are below `FLOOR = 1e-8` (`lib.rs`): from 84,480 samples | 217,088; 96 kHz: 433,408 |
| Gate/expander | `gain_db` | The release `flush(fma(c, t - g, g))` toward `t = -range` (`kernel.rs:369`) stops at its own float fixed point, where `c (t - g)` is below half an ulp of `g`: -79.98 dB for a -80 dB range. `channel_at_rest` (`kernel.rs:195`) accepts that word. Time: the hold, then about `ln(2 c range / ulp) / c` | defaults: 44,544 (100 ms hold, then the release); 96 kHz: 82,304; 0 dB range: 4,864 |
| Soft clip | the `e` history | No recursion. `X` and dry clear in 32 pushes. The last non-zero `e` is pushed on silent frame 29 and leaves 32 pushes later | 62 (bound 62) |

No effect has a stall bug. Each rest time is the effect's own law. The compressor and the transient
shaper reached `+0.0` only at the D7 flush floor, about 400 dB under any audible level. The gate
stops at the fixed point of its rounding.

**Limit that remains after D5:** the compressor's rest time scales with the user's release time.
At 48 kHz, a 1 s release took about 48 s to rest before B1 and takes about 16 s after it. A 5 s
release takes minutes. A long release therefore keeps a bank from skipping for that long.

### D2 Class A: no fix (attempt 1)

A class-A landing needs a state that leaves every later output bit unchanged, for every later
input. No state other than the existing flush has that property.

- **Compressor:** among the words the engine holds (`+0.0`, or `|g|` at or above `FLUSH_EPS`; a
  subnormal word is never held, because the flush and now the snap clear it),
  `fast_gain_from_db(g)` is exactly `1.0` only for `g == +0.0`. For every
  `0 < |g| <= 5.38e-7` dB (`0x3510_7e90`) it is the plateau value `0x3f7f_fffe`, which is
  `1 - 2` ulps. The plateau is contiguous: an exhaustive release-build sweep found no return to
  it up to `1e-3` dB. Every non-zero word therefore still scales the signal, and a snap moves bits.
- **Transient shaper:** below `FLOOR` the output does not depend on the followers while the input
  is silent. On the next signal, `c * e + k * u` rounds differently for `e = 1e-9` than for
  `e = 0`.
- **Gate:** the trajectory to the fixed point is defined to the bit. Landing sooner changes it.

Attempt 1 changed no source and gave root two class-B options. The owner ruled them on
2026-10-23: "B1 + B2 at 1e-12" (D5).

### D3 Soft clip: collapse and fold

- **Collapse:** the soft clip does not implement the mono-collapse trio. `supports_mono_collapse`
  keeps the default `false` (`effect-contract/src/lib.rs:2414`), so a chain that holds it never
  collapses. The gate/expander and the transient shaper do not implement the trio either. Only
  parametric-eq, the compressor and the true-peak limiter do. The trio (`process_bank_mono`,
  `desymmetrize_channels`, `lane_channel_symmetry`, plus a new wasm roster kernel) is a future
  feature. This issue records it and does not file it.
- **Fold:** this is by design and is not a soft-clip chain defect. The soft clip has 31 samples of
  latency (`lib.rs:237`), and PDC puts the compensation delay on the shorter paths. So the fold
  is lost on the *other* strips' routes: `foldable_lane` refuses a route whose input carries a
  delay (`graph/src/runtime.rs:9910`).

### D4 Gate 1 tests and their bounds

Each bound is derived in the doc comment of its test. `u = 2^-24`. Rest is read at block ends, so
each bound is rounded up to a block end.

- **Compressor,** `a_compressing_lane_lands_at_rest_within_its_release_bound` (new). Bound:
  `ln(|g| / REDUCTION_REST_DB) / (-ln(1 - c(1 - u)) - u) + 1`. Measured 79,200 samples at the
  -18 dB default against a bound of 79,185, which is 79,200 at the block end. Before B1 the bound
  used `FLUSH_EPS`, and the measurement was 230,928 against 230,990. It holds to the block for
  thresholds -18 to -80, at W4 and W8.
- **Transient shaper,** `a_lane_that_followed_a_tone_lands_at_rest_within_its_release_bound`
  (new). Bound, per follower: `ln(e / FOLLOWER_REST) / (-ln c - u) + 1`. Measured 128,640 against
  128,653 to 128,668. Before B2: 217,088 against 217,095 to 217,109.
- **Gate/expander:** folded into `the_silent_fast_path_renders_the_kernel_bits`. Bound, per lane:
  `hold + 1 + 24 ln 2 / -ln(1 - c(1 - u)) + 3`, because the distance to the stall point is below
  `2^24` grid units. This is a loose but valid ceiling, about 1.5x to 1.7x the measured time:
  2,736 to 6,912 samples against 4,032 to 11,184. It replaces `< RELOAD_BLOCK` (19,200) as the
  per-lane limit.
- **Soft clip:** folded into `the_silent_fast_path_renders_what_the_kernel_renders`. Bound, per
  lane: `max(DRY_DELAY - 1 + 32, ramp end)`, tight to the block.
- **B1 floor,** `the_reduction_rest_floor_is_the_fast_gain_plateau_edge` (new). It re-derives
  `REDUCTION_REST_DB` from the real `fast_gain_from_db`:
  - the edge is on the plateau;
  - the next `f32` is off it;
  - every 997th word from `f32::MIN_POSITIVE` to the edge is on it;
  - the plateau value is pinned at `0x3f7f_fffe`.

  So a change to `fast_db` cannot leave the floor stale.

### D5 Class B, owner ruling of 2026-10-23: B1 + B2 at 1e-12 (attempt 2)

**What changed:**

- **B1, compressor.** `REDUCTION_REST_DB = f32::from_bits(0x3510_7e90)`, about `5.38e-7` dB, the
  largest `|g|` with `fast_gain_from_db(g) == fast_gain_from_db(-1e-30)`. `snap_reduction`,
  `andnot(abs(g) <= REDUCTION_REST_DB, g)`, replaces `flush` in `ballistic`. That is the one
  place the word is written, and every path calls it. The snap has the flush's form, subsumes it,
  and adds no operation and no per-lane branch.
- **B2, transient shaper.** `FOLLOWER_REST = 1e-12`. The shared follower step
  `effect_runtime::envelope::ar_one_pole_step` now takes the rest floor as a parameter. Its snap
  `andnot(abs(x) < rest, x)` replaces the fixed D7 flush and is that flush bit for bit at
  `rest = FLUSH_EPS`, so it adds no operation. The transient shaper is its only production
  caller, and it passes `FOLLOWER_REST`. `effect-runtime`'s own tests pass `FLUSH_EPS`, so their
  claims (strict switch, two-product form, flush to `+0.0`, width independence) are unchanged.
  Root approved the path extension to `crates/effect-runtime/src/envelope.rs` (attempt 2). An
  intermediate crate-local copy of the step, with a drift test, was removed in favour of the
  parameter.

**Time to rest, at 48 kHz:**

| Effect | Before (base) | After |
|---|---|---|
| Compressor, defaults | 230,928 | 79,200 |
| Compressor, -40 dB threshold | 236,352 | 84,576 |
| Transient shaper | 217,088 | 128,640 |

**When bits move.** Only after a snap, on the next non-silent input. Above the snap, every word is
bit-identical to the shared step.

**How much: ceilings.** Each value below is the largest change seen over every corner that two
harnesses measured: mine and the final verifier's. Both were scratchpad harnesses, and neither is
committed. Each harness rendered the same input on this branch and on the merged base
(`de91d45c5`), compared the output word by word, and took the largest change in ulps and in dB.
Release build, 48 kHz, one scalar instance, 128-frame blocks, DualMono. My harness:

- **Compressor:** default parameters, with mix and makeup at (1, 0 dB), (0.5, 0 dB), (1, +6 dB)
  and (0.5, -3 dB). A 0.5 sine for 4,800 samples, then 120,000 samples of `+0.0`. This silence
  ends after the snap and before the base rest. Then 9,600 samples of an onset sine at 1e-6,
  1e-4, 1e-2, 0.1 and 0.5, with a `-0.0` every 97th sample.
- **Transient shaper:** the amount corners (attack, sustain, mix) = (+1, -1, 1), (-1, +1, 1) and
  (+1, +1, 0.5). A 0.5 sine for 4,800 samples, then 129,000 samples of `+0.0`, which ends just
  after the snap, where the base followers are at their largest. A second run uses 160,000
  samples. Then 9,600 samples of an onset sine at 1e-7 to 0.5, stepping between full level and
  0.1 of it every 2,000 samples.

The verifier's harness reached larger values in some cells. The ceilings are:

- **Compressor:** at most 2 ulps, `1.4e-6` dB, at every corner and every onset level, including
  0.1 and 0.5 and both makeup cases. At mix 0.5, a `-0.0` input now leaves as `-0.0` (the dry
  identity) where the base wrote `+0.0`.
- **Transient shaper, onset just after the snap:**

| Onset level | Ceiling |
|---|---|
| 1e-7 (-140 dBFS) | 1,282 ulps, `7.0e-4` dB |
| 1e-6 (-120 dBFS) | 408 ulps, `2.25e-4` dB |
| 1e-5 (-100 dBFS) | `2.5e-5` dB |
| 1e-4 (-80 dBFS) | 8 ulps, `4.8e-6` dB |
| 1e-3 (-60 dBFS) | `3.4e-6` dB |
| 0.5 | 0 |

- **Transient shaper, onset after 160,000 silent samples:** up to 8 ulps (`4.6e-6` dB) at
  -140 dBFS, and 2 to 6 ulps at -120 and -100 dBFS.

**Correction to the expectation given with the ruling.** The expectation was at most `3.1e-5` dB
on onsets below -40 dBFS. That does not hold right after the snap: at -120 dBFS the ceiling is
`2.25e-4` dB, and at -140 dBFS it is `7.0e-4` dB. From -100 dBFS up, the ceiling is `2.5e-5` dB.

**Committed digests and fixtures:** none moved, so nothing was re-baselined. The four crates' cross-target,
conformance, oracle and digest tests pass unchanged, and so does the workspace suite (the gates in
the hand-back). Their corpora have no silence long enough to reach a snap.

**Tests changed for the new floors:**

- Compressor `staggered_rest_case` (`padding_tests.rs`): its releases were 5 to 8 ms, which now
  rest so early that the assert "lanes rest on different blocks well inside the silence" failed
  (281 of 320 admissions). They are now 15 to 24 ms, and the bank rests after 115 to 138 blocks.
- Transient `TONE_LEVEL` (gate 2): it was `1e-15`, which no longer wakes a follower above the snap
  (the fast path took every silent block). It is now `1e-8`.
- Transient `plant` words: `1e-19` and `3e-18` are now `3e-12` and `1e-10`. Each is still a
  word between the snap and the two attack wake points.
- The `boundary.rs` and `MUTATIONS.md` text now names the step's rest-floor snap.

**`scripts/check-web-audioworklet.sh`** now runs, because kernel source changed: it passes. The
roster rows are compressor dual (vector 949, scalar 0), compressor collapsed (524, 0) and
transient-shaper (522, 0).

**Recorded note, not fixed:** a state restore accepts a reduction word in
`(0, REDUCTION_REST_DB]` dB, a word the engine never holds after B1 (`state.rs`,
`validate_channel`, accepts any normal word in `[-100, 0]`). Such a lane is not at rest, and its
next rendered sample snaps the word to `+0.0`.
