# Issue #880 MB-2 attempt 1 evidence

## R2 implementation evidence

The approved X7 detector-contrast and X8 applied-gain calls use `math::fast_db` in the shaper
`frame`. Both retain the existing clamps and the `shape == 0` dry select. The X7/X8 crossing tests
are active in the default F1 run; their separate full-domain tests are ignored by default and run
with the exhaustive F1 command. The seal now requires exactly eight named crossings.

All three `CROSS_TARGET_DIGESTS` rows moved. They were regenerated from the scalar `Lane` path
using `MISO_ENGINE_REPIN_TRANSIENT_SHAPER_CORPUS=1`; the cross-target test separately passed the
scalar/Simd4/Simd8 width identity check:

| case | old digest | new scalar digest |
|---|---|---|
| dual mono | `38a3b6f5e1155ccf3331c4d45be798dae05625e90ee815ef1bec9cb280e0c2f4` | `7359b2825c97a9bfc68269c17a55683567d3b2112ddf9c9d9c40647028f5c6f5` |
| maximum link | `ef30e6868ab30fc310ab2724376c305b8a37c437757c3130601f9139bd0bb9a4` | `cd131473b7ddc02f12a48344da7849294c3dd0944b55a42e93be95631c2e07d1` |
| average link | `5b43b7abe28bef85860b3a18aa0cf5ffab214e89c3a300f2b6a896f790ca1751` | `edbf3cb956228b38db2378b9e6d6356a7bc6e409801d644268e48bb97afc0c9f` |

## Frozen oracle row failure

The complete existing 96-sample scalar oracle row was measured before and after the change. The
unchanged gate is strict `max_abs_error < 2.0e-5`:

| source | max absolute error | dBFS | sample | input | production | f64 oracle |
|---|---:|---:|---:|---:|---:|---:|
| baseline `a5cb5d8e` | `1.668930054e-5` | `-95.551231` | 22 | `0.6999729276` | `5.285184383` | `5.285167694` |
| MB-2 candidate | `2.098083496e-5` | `-93.563545` | 19 | `0.6828526855` | `5.327214718` | `5.327193737` |

The row is 48 kHz, 96 samples of `max(sin(i * 0.071) * 0.7, -0.7)`, attack `0.75`, sustain
`-0.5`, mix `1.0`, dual mono. The candidate exceeds the frozen gate by `9.8083496e-7` (4.90%).
The measurements used a temporary diagnostic that traversed all 96 samples, recorded the maximum,
then applied the original unchanged threshold; that diagnostic is not a tolerance change.

MA-5's `1.287460e-5` absolute gain error and `1.654115e-5` dB fast-versus-exact gain delta came
from exhaustive ratios at the four `attack/sustain ∈ {-1, +1}` corners. They do not by themselves
bound the row's interior values `attack=0.75`, `sustain=-0.5`. The existing `0.01` dB impulse,
step and decay oracle rows passed on the candidate; only the 96-sample absolute row failed in the
focused transient-shaper test.

## MQ-1 same-programme PCM null

An untimed temporary dump test rendered the MQ-1 eight-track Simd8 bank on both baseline
`a5cb5d8e` and candidate `e6989987`. Each run used the same `tests/bench.rs` fixture generator
(SHA-256 `710e2115e4d46534e22cd4aa3917342f7ecede87c3be6eff7d0812ca41a4c886`), the same temporary
dump harness (SHA-256 `ec178b32077afeccb972e99176f07bf9fd09518a3441318d9eade6c063bb7d9a`), release
profile, 48 kHz, 128-frame blocks and one uninterrupted 192,000-frame pass. Both harness copies
reported fixture SHA-256
`85645b77376e39c43934476b5e5c5c37f96ae6abff7cc66b2b3d28a827275b6f`. The dump contained 3,072,000
f32 samples (left then right, with eight interleaved tracks in each channel); raw PCM was removed
after comparison.

Across all channel/track/frame samples, maximum absolute residual was `7.629394531e-6`, or
`-102.350199 dBFS` relative to full scale 1.0. Maximum relative output-magnitude delta was
`0.000013073 dB`, computed as `max(abs(20*log10(abs(candidate)/abs(baseline))))` where both
magnitudes are nonzero. If both magnitudes are zero the sample is skipped; if exactly one is zero
the relative delta is defined as infinity. This run had zero joint-zero samples and zero
one-sided-zero samples. The largest absolute residual occurred at right channel, frame 822, track
0 (`baseline=-5.969945431`, `candidate=-5.969953060`); the largest relative delta occurred at left
channel, frame 999, track 2 (`baseline=-0.5148412585`, `candidate=-0.5148420334`). The renderer
reported Simd8 on x86_64 Linux with rustc 1.97.1 / LLVM 22.1.6. No timed measurement was taken.

Attempt 1 is blocked on the frozen oracle gate and owner direction for R2. No tolerance was widened.
The math F1 default run passed 14 active tests, including X7/X8 subsampled domains and the eight-
crossing seal; math and transient-shaper package clippy, `cargo fmt --all -- --check`, and
`git diff --check` passed. The full ignored math sweep and wasm gates remain outstanding. The full
transient-shaper package run fails only at the recorded 96-sample oracle row; its impulse, step,
decay, width identity, bank, allocation, boundary and contract tests passed. The MQ-1 PCM null
comparison is complete as recorded above. The post-change timed measurement remains held until
#902's corrected runner is integrated and root clears the shared CPU.
