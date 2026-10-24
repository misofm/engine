# Add a signed gain-change observation for effects that boost

Filed by root from the #17 S1 verification (backlog sprint, 2026-10-24). Blocks #17 S2 (registering the dynamic EQ).

## Finding

The dynamic EQ's upward mode boosts a band, but the only observation kind it can declare is `GainReductionDb` with the `PeakMagnitude` fold (`crates/dynamic-eq/src/lib.rs` near line 493). An upward band therefore reads as a reduction of the same size on any meter a UI subscribes to: a 6 dB boost shows as "6 dB GR".

## Smallest closable slice

1. Add a signed observation kind (for example `GainChangeDb`, the next free value) and a signed fold (the value of largest magnitude in the window, sign kept) to `crates/effect-contract`, `docs/EFFECT_OBSERVATION_V1.md` and the host fold on both hosts (decision 17: one core function).
2. The dynamic EQ declares the signed kind; the compressor, gate, de-esser and multiband keep `GainReductionDb` (no change, class A).
3. One test in effect-contract for the signed fold (red if the fold drops the sign or picks the wrong extreme), and one host parity row.

## Paths

`crates/effect-contract/src/{lib.rs,live.rs}`, `crates/effect-contract/tests/{observation_lane.rs,registry.rs}`,
`tools/parameter-metadata/src/lib.rs`, `scripts/check-parameter-metadata-v1.py`,
`scripts/fixtures/parameter-metadata-v1-self-test.json`, the generated `sdk/assets/miso-engine-v1-parameter-metadata.json`
and `sdk/src/generated/catalog.ts`, `sdk/src/browser/shipped-host.d.ts`, `hosts/host-web/src/{lib.rs,tests.rs}`,
`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`, `docs/EFFECT_OBSERVATION_V1.md`, this spec. The fold is in
effect-contract, not host-core, and the C ABI has no observation surface. The dynamic EQ's declaration moves to #17 S2.

## Decision record (attempt 1, 2026-10-24)

- Root: `ObservationKind::GainChangeDb = 2`, `ObservationFold::PeakSigned = 3`. `PeakSigned` keeps the value of
  largest `|x|` in the window with its sign; equal magnitude keeps the earlier value (strict `>`, as
  `PeakMagnitude`); the `0.0` window reset is beaten by any nonzero reading.
- Root: the fold arm is in `ObservationLane::accumulate` (`crates/effect-contract/src/live.rs`), the one function
  the per-node and bank paths call on both hosts. Paths narrowed from "host-core" accordingly (scope note).
- Root: additive to V1. No reserved value, no wire, protocol, C ABI or meter-frame change. The C ABI has no
  observation read surface, so nothing is exported there.
- Root: host-web keeps `observed_decibels` (its clamp honours a negative minimum) and adds
  `strip_reduction_decibels`, the per-strip frame-slot contribution: a `GainReductionDb` window as before
  (larger lane, then convert), a `GainChangeDb` window `max(0, -x)` of the lower lane. `trackGrDb`/`submixGrDb`
  stay non-negative per the `.d.ts`; the signed value is on the `readObservations` path.
- Review fold-in: `validate_descriptor` refuses a mismatched tap: `GainChangeDb` requires `PeakSigned` and `Db`, and
  `PeakSigned` is admitted only for `GainChangeDb`. The host's strip slot takes `0` from a non-finite signed lane (the
  clamp would otherwise turn `+inf` into a full-depth cut).
- Not in this PR: the dynamic EQ's declaration (its crate is in open PR #1919); it moves to #17 S2. Compressor,
  gate, de-esser, multiband and limiter are untouched (class A: the fold `match` gains an arm only).

## Evidence (attempt 1)

- `observation_lane::the_signed_peak_fold_keeps_the_sign_of_the_largest_magnitude` passes; red on three mutants
  of the arm (store `abs`, `>=` tie, signed `>` max).
- `host-web` `observation_unit_conversion_is_declared_and_clamped` gains the signed row; red when a boost adds to
  the strip slot.
- Vocabulary: exhaustive `match` in `parameter-metadata`, the py pin, regenerated SDK output,
  `check-sdk-generated.sh` (source and fresh named-twin artifact), `check-sdk-types.sh`,
  `check-web-audioworklet.sh`, `check-browser-expected-resources.py`: pass.
