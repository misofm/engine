# Correct the section_is_identity comment about a -0.0 integrator

## Product outcome

The doc comment of `section_is_identity` (`crates/lane/src/kernels/builtins.rs`, about lines 1583-1585)
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
- D2. Keep the file's line count unchanged: line numbers in browser-compiled sources reach the shipped
  AudioWorklet module, so a changed count moves its sha256.
- D3. No code change. If D1 finds that the bitwise check is not needed, report it for a separate issue
  rather than changing the check here.

## Authorized paths

`crates/lane/src/kernels/builtins.rs` (that doc comment only), this spec.

## Gates

- G1. `cargo doc --locked -p lane --no-deps` and `cargo fmt --check -p lane` pass; the file's line count
  is unchanged; the browser module's sha256 is unchanged (record both hashes).
- G2. The review checks the reworded claim against the evidence above (no committed prose test).
