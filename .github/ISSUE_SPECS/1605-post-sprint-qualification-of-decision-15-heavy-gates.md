# Post-sprint qualification of decision 15's heavy gates

Successor of #1530, #1532 and #1535 (root, 2026-10-13, Q8). Stream H. Issue number: assigned at
filing (written here as #1605). Not feature sprint work: it runs in the testing phase after the
sprint (owner, 2026-10-13).

## Product outcome

The three heavy measurements that the feature sprint moved out of their slices are run once, on
one head, with their bars unchanged. Each one either passes or opens a named defect. The slices
kept their CI rows; this issue holds only the measurements that CI cannot run or cannot afford.

## Context

- #1535 gate 8 (loaded paired A/B against `main`, Fisher one-sided p, stop on a full stall
  signature) needs about 260 browser runs and a loaded host. Method:
  `/home/bl/misofm/submix-verdicts/b3-acceptance/scripts/`.
- #1530 gate 6 (soak): 5 consecutive full-leg passes per browser and 10 Firefox passes under 24
  CPU hogs (the #1522 method).
- #1532 gate 7: source mutations M1-M9 (PR evidence, never committed tests).
- Not here: #1331's iOS leg (#1331b) is an owner-scheduled human step in #1331 itself (root,
  2026-10-13).

## Decisions

- D1. Every bar is the one in the source spec, unchanged. No bar is relaxed to pass.
- D2. One head: the `main` commit after the decision 17 browser chain (#1565, #1571) has landed.
  #1535's and #1530's measurements run on the then-current gates. A gate the chain changed (#1589
  reworks the observation hooks) is measured as it stands on that head.
- D3. Benchmark rule: one invocation, one warmup, two measured rounds, no retry, where a source
  spec measures time.

## Objective gates

1. #1535 gate 8, as written there.
2. #1530 gate 6, as written there.
3. #1532 gate 7: each mutation turns its named test red, recorded in this spec.

## Test value

- Gate 1 turns red if unsubscribe fails under load at a rate `main` does not show.
- Gate 2 turns red if the observation wait flakes under repetition or load.
- Gate 3 turns red if a close-path bound in #1532 is untested.

## Dependencies

- #1530, #1532, #1535 (their CI rows on `main`); #1565 and #1571 (the head of D2).
