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

Attempt 1 is blocked on the frozen oracle gate and owner direction for R2. No tolerance was widened.
The math F1 default run passed 14 active tests, including X7/X8 subsampled domains and the eight-
crossing seal; math and transient-shaper package clippy, `cargo fmt --all -- --check`, and
`git diff --check` passed. The full ignored math sweep and wasm gates remain outstanding. The full
transient-shaper package run fails only at the recorded 96-sample oracle row; its impulse, step,
decay, width identity, bank, allocation, boundary and contract tests passed. MQ-1 null comparison
and post-change timing remain outstanding. Timing is held until #902's corrected runner is
integrated and root clears the shared CPU.
