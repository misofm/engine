# Gate the stale-state clear of a live-disabled LPF at the strip output

## Product outcome

A test fails if a live-disabled input LPF keeps a stale negative integrator state that changes the sign of
an exact zero at the strip output. Today no output-level test covers this case.

## Context

- #1268 (batch `codex/d15-batch-dfg`) proved that the clear of a live-disabled input filter's state is
  correct, with a regression test. Its gate 1 disables only the HPF (section 0).
- The #1268 spec check (`docs/handoffs/decision-15-2026-10-05/verdicts/stream-dfg/stream-dfg-spec-check2.md`,
  probe A) shows that the LPF (section 1) is the one case where the clear changes an output bit: a negative
  non-zero stale LPF integrator pair, with an exact `-0.0` input, gives `-0.0` (`0x80000000`) at the
  strip output, where the elided twin gives `+0.0`. On the HPF, no stale state changes an output bit.
- So a regression that skipped the clear for the LPF only would pass every current output-level test.

## Decisions

- D1. Add one output-level case beside #1268's gate-1 test: drive a strip's live LPF into a negative
  non-zero integrator state, live-disable the LPF, feed exact `-0.0` input, and assert the output words
  bitwise against the elided twin (`+0.0`).
- D2. Test only; no production change. If the test is red on `main`, stop and report: that is a live
  defect for a separate issue.

## Authorized paths

The test file that holds #1268's gate-1 test (named in #1268's decision record), this spec.

## Gates

- G1. The new case passes on `main`. Mutation (PR evidence): skip the state clear for section 1 only on
  disable; the new case turns red and #1268's existing gate stays green. Test value: a disable path that
  keeps the LPF's stale state, which changes the sign of an exact zero at the strip output and which no
  current output-level test sees.
