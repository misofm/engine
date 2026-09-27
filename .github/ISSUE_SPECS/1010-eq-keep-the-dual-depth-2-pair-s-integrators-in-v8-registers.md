# EQ: keep the dual depth-2 pair's integrators in V8 registers

Weekly-optimisation issue from the #1000 Sol verification (`.github/ISSUE_SPECS/1000-*.md`, draft follow-up). #1000's gate only reports this loop today; this issue would let it hold it.


## Problem

In the shipped `host_web.wasm`, V8 12.4's TurboFan compiles the admitted, select-free
dual depth-2 pair loop of `PreparedParametricEq<f32x4, _>::process_bank` (`svf_cascade_skewed`
with S=2, D=2) to 181 instructions. Ten stack slots lie on its recurrences, among them `ic1` and
`ic2` of stream 1, section 0, which go through memory from one iteration to the next. 27
instructions are that traffic.

- **Why V8 spills.** The loop carries 8 integrators and 2 skew carries beside 24 invariants, against
  15 allocatable XMM registers. V8 spills the loop phis rather than the invariants.
- **What does not help.** Splitting it into spill-free single-stream passes costs 16 % on the
  two-band isolate (30.5 to 35.3 us per 64 tracks). Re-reading the coefficients, sharing them, or
  dropping the skew does not remove the spills either.
- **What is at stake.** Store-to-load forwarding on the recurrence and the extra dispatch. An upper
  bound from the instruction count is about 15 % of the pair loop, at most about 3 us of the 30.5 us
  isolate. Nothing measured has realised any of it.

## Candidate

Take the two skew carries out of registers by routing section 0's output through the
block in place: section 0 writes frame `i`, and section 1 reads it back one iteration later. An
`f32` store and load is the identity, and a carry is not a recurrence, so memory costs it no
recurrence latency. Then check whether V8 keeps the eight integrators in registers.

## Objective gates

1. The render is bit-identical: the G2 kernel identity, the parametric-eq suite, and the 90 native
   and 30 wasm console digests.
2. #1000's gate, with finding 2 fixed, reports no slot on an integrator recurrence in the dual pair.
   The pair is then moved from reported to held in the same change.
3. The two-band isolate through the render export (`web.mjs`) improves by at least 1.0 us. This is
   the mean of 6 runs per arm in both orders under `timing.lock` and `taskset -c 31`. The one-band
   isolate and the builtins row must stay within noise.
4. Stop after one prototype if gate 2 or gate 3 fails. Record the listing and the numbers, and name
   the reason for the gap: V8 x64's 15 XMM registers at S=2, D=2 is class B. Do not chase it further.
   This is weekly-optimisation work, not launch-critical.

## Amendment (root, from the #1009 verification)

The candidate reshapes the dual depth-2 pair, so #1000's gate (as amended by #1009) will fail
closed on the held dual tail until its row definitions are updated for the new shape. Update the
gate's rows in the same change, and show the gate green on the new shape and still red on the
#977 attempt-1 build and the one-token edit.

Also carried from the #1009 verdict (LOW, not in this issue's scope): the CI routing check does not
notice a gate step that is commented out, suffixed `|| true`, or guarded `if: false`.
