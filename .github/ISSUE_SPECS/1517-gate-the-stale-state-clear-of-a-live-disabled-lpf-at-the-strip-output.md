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

- D1. Parametrize #1268's `settled_disable_scenario` (`crates/builtins/tests/filter_liveness.rs`) by
  section; its one `#[test]` loops over both. No new test function, file or helper (scope note, root).
  The LPF leg uses a 1 kHz LPF (root). A live-disabled identity LPF holds its integrators frozen, and
  an elided or identity HPF in front adds `+0.0` and hides the input sign, so the LPF leg also enables
  a real 120 Hz HPF on lane 1 in both banks at the disable (zero state, so it passes `-0.0` through and
  its integrators stay `+0.0`). The frozen LPF pair is both negative after 7, 10 and 13 history blocks.
- D2. Test only; no production change. If the test is red on `main`, stop and report: that is a live
  defect for a separate issue.

## Authorized paths

The test file that holds #1268's gate-1 test (named in #1268's decision record), this spec.

## Gates

- G1. The scenario's LPF leg passes on `main`. Mutation (PR evidence): skip the state clear for section 1 only on
  disable; the LPF leg turns red and #1268's existing gate stays green. Test value: a disable path that
  keeps the LPF's stale state, which changes the sign of an exact zero at the strip output and which no
  current output-level test sees.

## Evidence (attempt 1)

- The LPF leg passes on `main` (no live defect).
- Mutant: the clear wrapped in `if section == 0` (dual and mono bodies, `crates/lane/src/kernels/builtins.rs`).
  The HPF leg (run first) stays green. The LPF leg fails on `lane 1 integrators are +0.0` (left words
  `[0, 0, 975151958, 966925478, ..]`). With the state and plan assertions turned off, it fails on the
  output words: `left, lane 1, frame 0, section 1, dual body, neighbour false, 7 history blocks, block 3`,
  `0x80000000` against `0x0`. Without the HPF in front, that output comparison stayed green.
