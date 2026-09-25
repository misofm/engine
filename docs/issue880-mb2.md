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

## Original oracle gate and accepted R2 amendment

The complete existing 96-sample scalar oracle row was measured before and after the change. Attempt
1 tested the original strict `max_abs_error < 2.0e-5` gate:

| source | max absolute error | dBFS | sample | input | production | f64 oracle |
|---|---:|---:|---:|---:|---:|---:|
| baseline `a5cb5d8e` | `1.668930054e-5` | `-95.551231` | 22 | `0.6999729276` | `5.285184383` | `5.285167694` |
| MB-2 candidate | `2.098083496e-5` | `-93.563545` | 19 | `0.6828526855` | `5.327214718` | `5.327193737` |

The row is 48 kHz, 96 samples of `max(sin(i * 0.071) * 0.7, -0.7)`, attack `0.75`, sustain
`-0.5`, mix `1.0`, dual mono. The candidate exceeds the frozen gate by `9.8083496e-7` (4.90%).
The measurements used a temporary diagnostic that traversed all 96 samples, recorded the maximum,
then applied the original unchanged threshold; that diagnostic is not a tolerance change.

Under the owner's explicit delegation, root accepted an amended R2 bound of strict `< 2.5e-5`
absolute sample error **only** in `scalar_matches_the_independent_f64_oracle`. The measured
candidate maximum is `2.098083496e-5`, leaving `4.01916504e-6` headroom. Root's engineering
judgment is that this residual is unlikely audible in normal playback; this is not a universal
inaudibility claim and there is no completed blinded listening test. ITU
[BS.1116-3](https://www.itu.int/rec/R-REC-BS.1116-3-201502-I) informs listening-test methodology,
not this numeric threshold. The `0.01` dB impulse/step/decay limits and every F1/M1 gate remain
unchanged.

MA-5's `1.287460e-5` absolute gain error and `1.654115e-5` dB fast-versus-exact gain delta came
from exhaustive ratios at the four `attack/sustain ∈ {-1, +1}` corners. They do not by themselves
bound the row's interior values `attack=0.75`, `sustain=-0.5`. The initial attempt exceeded the
original 96-sample absolute limit; with the accepted row-specific amendment, the full
transient-shaper package passes, including its unchanged `0.01` dB impulse, step and decay rows.

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

The R2 amendment resolves the only observed threshold failure; no other tolerance was changed.
`cargo test --locked --release -p transient-shaper` passed all active package tests. The math F1
default run passed 14 active tests, including X7/X8 subsampled domains and the eight-crossing seal;
all eight ignored full-domain math sweeps also passed in 73.33 seconds with four workers maximum.
Math and transient-shaper package clippy, `cargo fmt --all -- --check`, and `git diff --check`
passed. No blinded listening test was performed.

## Integrated MQ-1 timing evidence

After #902's corrected runner was integrated, root completed the combined #880 gates: native and
focused tests, workspace clippy and policy checks, all eight F1 sweeps, and native/scalar/SIMD
Wasm gates passed. The corrected MQ-1 self-test also passed. The untimed programme PCM null
comparison remains the one recorded above.

One MQ-1 timed invocation completed successfully on the integrated candidate
`5ce58d6eb0bdbe0cc02e5e7a01d3a9aaca5eb0fc`, with MB-2 effect source
`e00d4c2dbef1af166bedeb28fdc103d62a4de269` and revision `MB-2 (R2 fast dB tier)`. The frozen
four-second, 48 kHz, 128-frame, eight-track dual-mono programme used attack `0.75`, sustain `-0.5`,
mix `1.0`, and the same seeds and fixture hash recorded above. It ran one warmup and two measured
rounds per arm:

| Arm | Round 1 | Round 2 |
|---|---:|---:|
| Simd8 bank | 4.276925 ns/lane-sample | 4.305019 ns/lane-sample |
| Scalar | 30.301938 ns/lane-sample | 30.328344 ns/lane-sample |

The validated record and exact captured output are preserved in [the MQ-1 MB-2 record](../artifacts/issue880/mq1-mb2/mq1-mb2.json)
and [raw log](../artifacts/issue880/mq1-mb2/mq1-mb2.json.raw.log). The record identifies the
benchmark source, runner, fixture, candidate/source commits, and host (AMD EPYC 7313P, x86_64,
Linux; rustc 1.97.1, LLVM 22.1.6). The runner preflight passed and the timed test passed; no timing
retry was made. These are descriptive timings, not a release budget or isolated proof of MB-2's
causal performance effect. The earlier E1 timing observations remain preserved in
`docs/issue880-mq1.md`; the old run had a postprocessing failure, so this is not presented as a
validated matched before/after benchmark.
