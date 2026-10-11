# Correct the section_is_identity comment about a -0.0 integrator

## Product outcome

The doc comment of `section_is_identity` (`crates/lane/src/kernels/builtins.rs`, about lines 1583-1585)
states correctly where a `-0.0` integrator's sign can and cannot be observed, so no later change relies on
a false "observable divergence" claim.

## Context

- The comment says: "identity coefficients over a `-0.0` integrator emit `-0.0`, which the elided form
  would have washed to `+0.0`. That is an observable divergence, and `-0.0 == 0.0` would have admitted
  it."
- #1268's attempt-1 evidence (branch `codex/d15-batch-dfg`, verdict
  `docs/handoffs/decision-15-2026-10-05/verdicts/stream-dfg/1268-attempt1.md` after that batch merges)
  shows that at the strip output a stale integrator never changes an output bit, the sign of an exact zero
  included: the next stage's `v + 0.0` washes `-0.0` to `+0.0`.
- So the claim may be true at the section's own output words and false at the strip output. The comment
  does not say which. The bitwise state-word check it justifies may still be right for other reasons
  (the state words themselves are compared bitwise).

## Decisions

- D1. First establish, with a short test or a reading of the kernel, at which boundary (section output,
  state words, strip output) the `-0.0` is observable. Then reword the comment to name that boundary, and
  keep the reason for the bitwise state-word check accurate.
- D2. Keep the file's line count unchanged: line numbers in browser-compiled sources reach the shipped
  AudioWorklet module, so a changed count moves its sha256.
- D3. No code change. If D1 finds that the bitwise check is not needed, report it for a separate issue
  rather than changing the check here.

## Authorized paths

`crates/lane/src/kernels/builtins.rs` (that doc comment only), this spec.

## Gates

- G1. `cargo doc --locked -p lane --no-deps` and `cargo fmt --check -p lane` pass; the file's line count
  is unchanged; the browser module's sha256 is unchanged (record both hashes).
- G2. The review checks the reworded claim against the evidence in D1 (no committed prose test).
