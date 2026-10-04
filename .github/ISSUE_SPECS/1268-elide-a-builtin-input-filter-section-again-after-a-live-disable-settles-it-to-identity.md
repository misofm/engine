# Elide a builtin input filter section again after a live disable settles it to identity

Performance follow-up found while planning *Apply value-only input HPF and LPF edits to the running
C ABI plan through prepared targets* (#1262), slice of #1053. It affects the browser today, which
already disables filters live. Anchors verified on `main` at `54b0a1bf8`.

## Problem

An input filter section that a live edit disables keeps costing work until the plan is rebuilt.

- A section is skipped (elided) only when its six coefficient words are the identity **and** both
  integrators are exactly `+0.0` (`section_is_identity`,
  `crates/lane/src/kernels/builtins.rs:1146-1163`). The state words are compared bitwise on
  purpose: identity coefficients over a `-0.0` integrator emit `-0.0`, which the elided form would
  wash to `+0.0`.
- After a live disable, the coefficient ramp reaches the identity, and the elision plan is
  recomputed when the ramp ends (`refresh_filter_plan`, `crates/builtins/src/lib.rs:1263-1279`).
  But the integrators keep their last values: with identity coefficients, `svf_step` does not
  drive them to zero (`crates/lane/src/kernels.rs:636-652`). So the section is never elided again
  for the life of the plan.
- A plan prepared with the filter off elides the section from the start. So the live plan also
  differs in sign-of-zero bits from a rebuild on exact `±0.0` input samples, as long as the stale
  state is negative.

The owner's standing rule is that work on silence and identity sections is skipped at every stage.

## Outcome

When a section's coefficient ramp settles to the identity, the section's integrator state is set
to `+0.0`, at that block boundary, on the render thread, without allocation, so the recomputed plan
elides it. The output is then bit-identical to a plan prepared with the filter off, from the
settling block on.

## Decisions to make first (record them in this spec)

- **D1. Verify the mechanism.** Write the failing test of gate 1 first, and confirm that the
  state stays non-zero, before changing code. If the state does decay to `+0.0`, close this issue
  with the evidence.
- **D2. Where to zero.** Choose between the ramp's final step and `refresh_filter_plan`, per lane
  and per section. Banking must not couple lanes' bits: zero only the settled lanes.
- **D3. Class.** Zeroing the state of an identity section changes no output sample except the sign
  of an exact zero, and that change moves toward the prepared plan's bits. Say so in the PR.

## Authorized paths

- `crates/builtins/src/lib.rs` (the input bank's filter ramp and plan refresh).
- `crates/lane/src/kernels/builtins.rs`, only if the plan function needs it.
- Their tests.
- This spec.

## Non-goals

- No change to the filter design, the 64-update ramp or the elision rule itself.

## Objective gates

1. **The fix.** A builtins test enables an HPF on one bank lane through a prepared target, renders,
   disables it through a prepared target, and renders past the ramp, with negative input samples
   and some exact `-0.0` samples. After the ramp:
   - the section is elided (the plan says so);
   - the output is bit-identical to a bank whose filter was never enabled, fed the same input after
     the ramp;
   - the other lanes' bits do not change.

   *Test value: it turns red if a disabled section keeps its stale state (no elision, a sign
   difference), or if the zeroing touches a lane that is still ramping.*
2. **Nothing else changes.**
   - `cargo test --locked --all-targets -p builtins --features builtins/test-support`
   - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   - `cargo test --locked --release -p audit -p bench -p console-workload`
   - the browser artifact gates of #1253's gate 5, and the module digest before and after.
   - `bash scripts/check-cross-targets.sh`; 4-lane (NEON) is CI-only.

## Dependencies

None.

## Standing rules for the implementer

- Work from this body. Change nothing outside the authorized paths.
- No allocation, lock or unbounded work on the render thread.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
