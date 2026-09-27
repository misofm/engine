# Skip silent contributors in the master and bus sums

Draft, slice S10 of the silence architecture issue (A0). Class A. Evidence:
`docs/handoffs/silence-2026-09-27/DESIGN.md` sections 4.2 and 4.8; the skip theorem, its
independent verification (2,000,000 hostile cases, 0 mismatches) and its mutation list are #940's
Research findings (2) and `docs/handoffs/plumbing-floor-2026-09-26/SILENCE-MASKS-VERIFY.md`.

## Product outcome

In today's rows every track folds into the master inside its bank chain, and S4 applies the skip
rule there. Contributors that reach the Output op or a bus reduction unfolded (ragged tails, bus
returns, sends into buses) are still read and added when silent. This slice adds a **silence table**
and uses it in the reductions:

* **The table:** one byte per arena buffer (allocated at bind, sized by the plan's buffer count),
  stamped with a block counter; a producer that knows its output is silent this block (a latched chain
  for its unfolded outputs, a source claim with `played_silent`) records it; a stale stamp reads as
  "not known".
* **The reductions** (`route_reduce`, `reduce_many_into`, `crates/graph/src/runtime.rs:601`, and the
  bus reductions): an input whose fact is set is not loaded or added; when its silent route mix would
  be `+0.0` on a plane, that plane gets one `x + (+0.0)` after the chain; if no input is live, the
  plane is filled with `-0.0` iff every skipped mix is `-0.0`, else `+0.0` (#940's steps 6 and 7).
  Live inputs keep their order; the first live input stores.

## Authorized paths

`crates/graph/src/runtime.rs`, `crates/graph/src/lib.rs` (a test-support declined knob), tests,
`crates/graph/tests/MUTATIONS.md`, this spec.

## Objective gates

1. #940's G1 corpus (fan-in 2-64, quantum 128 and 100, hostile data and coefficients, unplayed sets)
   against an independent scalar oracle and against the declined runtime; the fix-up must be
   exercised.
2. #940's red mutations 1-5.
3. S4's corpus unchanged; a bus session (tracks → bus → Output) with sparse activity bit-identical to
   declined.
4. Realtime, rule 3 (the silent-mix arithmetic and fix-up tails are non-generic `#[inline(never)]`).

## Console benchmark rows

None of the standing rows reduces unfolded inputs. A bus variant of the sparse row is the evidence.

## Dependencies

S4.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A: every gate that says "bit-identical" or "unchanged" is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run the timed runner; do not quote a projected saving.
