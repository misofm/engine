# Correct the section_is_identity comment about a -0.0 integrator

## Product outcome

The doc comment of `section_is_identity` (`crates/lane/src/kernels/builtins.rs`, about lines 1623-1640 at 82f987296)
states correctly where a `-0.0` integrator's sign can and cannot be observed, so no later change relies on
a false "observable divergence" claim.

## Context

- The comment says: "identity coefficients over a `-0.0` integrator emit `-0.0`, which the elided form
  would have washed to `+0.0`. That is an observable divergence, and `-0.0 == 0.0` would have admitted
  it."
- #1268's evidence (`docs/handoffs/decision-15-2026-10-05/verdicts/stream-dfg/stream-dfg-spec-check2.md`,
  probes A-E) shows the comment's premise is false as worded:
  - a `-0.0` integrator pair changes no output bit on either section (HPF or LPF), for input `±0.0`;
  - on the LPF, a *negative non-zero* stale integrator pair over an exact `-0.0` input can change the
    sign of an exact zero at the strip output (probe A: bank `0x80000000`, elided twin `0x0`);
  - on the HPF, no stale state changes an output bit, the sign of an exact zero included (probes B, B2,
    E: 180 combinations).
- So the bitwise state-word check that the comment justifies may still be right, but for a different
  reason than the one it gives. Section 0 is the HPF and section 1 the LPF (`for section in 0..2`).

## Decisions

- D1. Reword the comment to the evidence above: name the stale state that can be observed (a negative
  non-zero LPF integrator pair over an exact `-0.0` input, as the sign of an exact zero), say that a
  `-0.0` integrator is not observable on either section, and give the true reason for the bitwise
  state-word check.
- D2. Dropped (root, 2026-10-24). Since #1061 the committed AudioWorklet sha256 is a release
  fingerprint checked only under `scripts/build-web-audioworklet.sh --check-pin`; the CI job "shipped
  AudioWorklet artifact identity" reports `ARTIFACT CHANGED`/`UNCHANGED` and fails only for a
  non-reproducible build or a release change. A moved line may read `CHANGED`: a report, not a
  failure. The comment is written at the length the truth needs (+11 lines).
- D2a. The doc of `every_lane_is` (just above) repeated the false claim; it is corrected too (root).
- D3. No code change. If D1 finds that the bitwise check is not needed, report it for a separate issue
  rather than changing the check here.

## Authorized paths

`crates/lane/src/kernels/builtins.rs` (the doc comments of `every_lane_is` and `section_is_identity`
only), this spec.

## Gates

- G1. `cargo doc --locked -p lane --no-deps` and `cargo fmt --check -p lane` pass.
- G2. The review checks the reworded claim against the evidence above (no committed prose test).

## Evidence

- Re-derived from `svf_step_when` (`crates/lane/src/kernels.rs`) and the section mix
  `fma(m2, v2, fma(m1, v1, m0 * v0))`: a `(-0.0, -0.0)` integrator pair gives `v1 = +0.0`,
  `v2 = +-0.0` and output `v0 + 0.0` (no bit moves); the full form writes `ic1' = +0.0`, the elided form
  keeps `-0.0`, so only the state words diverge. A `(a, b)`, `a, b < 0` pair gives `(v0 + -0.0) + -0.0`,
  `-0.0` for an exact `-0.0` input (probe A). So D3 holds: the bitwise check stays.
