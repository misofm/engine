# Parametric EQ: bind declines before validating its members

Found by #1051's randomized differential (`crates/conformance/src/randomized.rs`, "#1051 defect 2", seed 0).

## Problem

The EQ's bank bind returns "decline" (fall back to per-node rendering) before it validates the bank's members, so an invalid member can be reported as a decline instead of a refusal. The bind contract has three outcomes (bind, decline, refuse), and validation must decide refusal before the decline test runs.

## Smallest closable slice

Validate every member first, then decide bind or decline. Un-ignore the reproducer.

## Gates

1. The reproducer passes; an invalid member is refused with its typed diagnostic whether or not the bank would otherwise decline.
2. A planted reordering (decline first) turns it red.
3. Valid sessions bind or decline exactly as before; console digests unchanged.

## Decision and evidence record (attempt 1, 2026-10-23)

- **Shape of the defect.** `bind_bank` (`crates/parametric-eq/src/lib.rs`) returned `Ok(None)` at a
  width this build does not execute before it looked at any member, and returned `Ok(None)` at the
  first heterogeneous member before it validated the members after it. Its member loop also ran only
  `expected_prepared_metadata`, not the EQ's own initial-section checks (`physical_targets`), which
  `prepare` runs.
- **The foreign width is a refusal, not a legitimate decline, when a member is invalid.** The
  contract on `NativeEffectFactory::bind_homogeneous_bank` lists the foreign width as an `Ok(None)`
  case, and also says every implementation validates every member before it decides the shape, so
  an absent capability can never hide a malformed member. A malformed member is a malformed request
  at every width: `prepare` would refuse it on the scalar fallback the decline sends it to. So the
  decline stays legal only for a cohort whose members all validate.
- **Fix.** `validate_member` runs `prepare`'s checks in `prepare`'s order (contract metadata, then
  `physical_targets` for both channels), so the bank refuses with `prepare`'s code. `bind_bank`
  validates every member, then declines on a foreign width or a heterogeneous cohort. Valid cohorts
  bind or decline as before; `prepare_width` and every render path are unchanged.
- **Other effects.** The compressor, delay, gate, multiband, soft clip, shaper and limiter binds
  validate every member before they decline (read, and their differentials run with no narrowing).
  Only the EQ had the defect.
- **Harness.** `Known::BindDeclinesBeforeValidating` and its allowance in
  `crates/conformance/src/randomized.rs` are removed; `Known` is empty. The EQ's per-PR differential
  runs with `known: &[]`, and its ignored full-strength twin is deleted (it is now the same test).
- **Red on the pre-fix code.** With the allowance removed and the old `bind_bank`:
  `cargo test -p parametric-eq --test randomized` fails at seed 4, `Four` (the width an AVX2 build
  does not execute): "bind declined a cohort with a member prepare refuses
  (effect.parameter.initial)". Green after the fix.
- **Mutants.** (1) The width decline moved back ahead of validation: the differential is red
  (seed 4). (2) `return Ok(None)` at the first heterogeneous member: the differential stays green
  (its illegal probe never adds a heterogeneous member), so `a_padded_request_binds_after_every_lane_is_validated`
  (`crates/parametric-eq/tests/bank.rs`) now also asserts the refusal with lane 1 foreign, and is red
  on that mutant.
