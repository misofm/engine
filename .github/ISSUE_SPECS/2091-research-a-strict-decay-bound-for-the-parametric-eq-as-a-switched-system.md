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

## Evidence and decision record (2026-11-04)

- **Derivation:** `docs/derivations/2091-eq-switched-system-strict-decay.md`. Research only: no
  code is committed and no rendered bit moves. The check scripts are its appendices, as text.
- **Answer: no strict-decay bound exists under production's premises.** A disabled band executes
  the exact identity (`A = I`), so the joint spectral radius of the executable steps is 1 and no
  norm, piecewise-quadratic or polyhedral function, or dwell-time result gives `q < 1` (Theorem
  B1). For every quantum below 64 frames, touching a disabled band's row at every block restarts
  its disable ramp and drives the design weight to 0. Below `delta_div` = 0.00544, 0.00597, 0.01142
  and 0.01247 (44.1, 48, 88.2, 96 kHz) the standard `f32` rounding model admits growth, so no
  peak-relative bound can be certified (Theorem B2). A quantum of at least 64 frames is not a
  production guarantee; even with it, the common-norm certificate closes only at 44.1 and 48 kHz.
- **Correction to the Problem section.** Without the identity, a common quadratic norm exists at
  every rate: `L = [[1, 0.0093375], [0, 0.9998114]]`, `q_D` = 0.9999934300, 0.9999939638,
  0.9999967150 and 0.9999969819, from an exact corner theorem plus the `f32` word box. The domain
  extreme is a tie at 10 Hz: the bell at +24 dB, Q 18, and the low shelf at +24 dB, slope 0.1. The
  rotation per step does not matter, because the bilinear map keeps a continuous-time Lyapunov
  inequality for every `g`. The #1372 note's evidence missed the shelves (`g = w / sqrt(A)`).
- **Recommendation for #1372 (owner ruling, class B; revised in attempt 2, below):** on a ramp
  toward the identity, hold the recursion words at design weight `1/16` while the mix words finish
  today's ramp. Certified per-step contraction at that weight: `1 - 3.50e-7`, `1 - 3.17e-7`,
  `1 - 1.45e-7`, `1 - 1.28e-7` with no walk loss, and `1 - 1.04e-7`, `1 - 9.05e-8`, `1 - 2.18e-8`,
  `1 - 1.52e-8` with a walk loss of 0.6. Fallback: `1/8`.
- **Further findings for #1372:** the multiband compressor's absolute walk box adds 0.93 to 2.02
  times the EQ's margin, so #1372 needs a relative bounded-excess lemma (measured walk loss at most
  1.2e-3); a tail in the common norm is about 1.51 times the brute-force tail at the extreme; a
  disabled band's held state blocks D3a rest, and zeroing it at the identity changes no output
  value (only the sign of a zero output for a `-0.0` input).
- **Gates:** every claimed bound checked at 44.1, 48, 88.2 and 96 kHz. The corner theorem gives the
  exact maximum over the continuous domain, so the off-grid margin is only the `f32` word box (at
  most 3e-10 at the extreme). Regression checks: the 780,000-point gate-2 grid, 100,000 random
  designs and 100,000 random `f32` walks per rate, all at or below `q_D`.

### Attempt 2 (verifier FAIL on attempt 1: two MAJORs in the snap recommendation)

- **MAJOR 1 (bit-move scope of the end snap).** Confirmed. The snap ends the ramp at frame 61, so a
  re-enable on frames 61 to 63 meets `remaining = 0` and identity words, and P5 zeroes the state.
  All later output of the band changes: up to 18x the input amplitude, still 16.5x to 17.2x after
  3,000 frames. The snap is rejected.
- **MAJOR 2 (audible cost).** Over 99,372 lanes per rate at all four rates (`f64`, unit complex
  exponential), the snap adds a step of up to 1.08x the input amplitude, about 3x today's largest
  per-frame change, and up to 9.4 dB above today's largest per-frame gain change.
- **Recommended design: the hold** (note §6.3). The recursion words stop at weight 1/16; the mix
  words finish today's 64-frame ramp; all six take the identity at frame 64, as today. A retarget
  toward the identity keeps held words held. A disable within 3 frames of an enable from rest lets
  the recursion words rise to 1/16 first. It keeps the certificate, P5's timing is unchanged, and it
  adds no step: during a disable its output differs from today's by at most 0.036x the input
  amplitude, and its largest per-frame change is at most 1.064x today's (0.054 dB excess).
- **Exact bit-move scope of the hold** (note §6.5): frames on which today's recursion weight is
  below 1/16 on a ramp toward the identity (frames 61 to 63 of a disable from a design; at most
  0.19x the input under a restarted disable); a disable within 3 frames of an enable from rest (at
  most 0.155x); and all later output of a band re-enabled while either is in flight (at most 2.80x
  after a normal disable, never more than today's own difference when the re-enable lands one to
  three frames earlier; about 13x after a long restarted disable).
- **Listening check:** not needed for the hold's normal cases; on every lane its differences are no
  larger than today's own sensitivity to a one-to-three-frame shift of the same edit. The restarted
  case is a behaviour choice (which held state a re-enable revives), not a step, and a listening
  check would not decide it. No claim of absolute inaudibility is made. The restarted case comes from
  the defect filed as #2247 and goes away when it is fixed.
- **MINORs folded:** "in the standard rounding model" in the answer; the `-0.0` sign effect of
  zeroing a held state; walks checked at every frame with a seed per rate; the corner tie explained
  as the norm optimum. New finding §7.4: a disabled band's parameter change restarts its disable
  ramp (P4), so under automation the disable never completes.

### Attempt 2 verdict

- PASS (hold). Three MINORs folded: the frame-64 drop is the hold's largest added step and equals
  `|D63|` (0.036x at 44.1 kHz); the 1.064 ratio comes from the +24 dB, Q 18 bell at 20 kHz, not
  from the shelf; the 13x restarted case is the #2247 defect, not a class-B question. The #2247 spec
  is added on this branch.
