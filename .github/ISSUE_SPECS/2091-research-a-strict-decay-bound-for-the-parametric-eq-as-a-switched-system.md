# Research a strict-decay bound for the parametric EQ as a switched system

Filed by root under the owner ruling of 2026-10-25 (the #1372 question). Research only; no engine change.

## Problem
#1372 D2 asks for a common quadratic Lyapunov function `Q` (`A^T Q A <= q^2 Q`, `q < 1`) over every designed SVF step and every ramp step of the parametric EQ, at each rate. At 44.1 and 48 kHz none exists: the bell family spans more than 90 degrees of rotation, and every design has `||A||_2 = 1`. So the EQ keeps an `Infinite` tail, #1372 stays blocked, and #1378 (retire the Infinite tail) waits.

## Question
Is there a certified strict-decay bound for the EQ as a switched linear system (designed steps plus 64-sample ramps between any two designed points), for example a piecewise-quadratic or polyhedral Lyapunov function, a dwell-time bound that production already guarantees, or a joint spectral radius bound below 1? If yes, give `q`, its premise and the domain extreme. If no, show why.

## Deliverable
A derivation note in `docs/derivations/` with the method, its primary citations (switched-system stability literature), the bound or the proof that none exists under production's premises, and a recommendation for #1372: a bound that #1372 can implement, or a contract change for the owner.

## Gates
- Every claimed bound checked numerically at each launch rate (44.1, 48, 88.2, 96 kHz) over the whole parameter domain, with the grid and the Lipschitz margin stated.
- No rendered bit moves; no committed code beyond the note.

Blocks: #1372, and through it #1378.
