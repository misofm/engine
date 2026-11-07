# Effect runtime: put the remaining ramp encodings and the effects' lane-vector ramp helpers on `LinearRamp`'s law

Successor of #2368 (GR-7's first slice), filed when the implicit-state-machine tracker #2258 closed. #2368 made `LinearRamp`'s settled invariant structural (private fields, `from_words`, `record_advance`) and found that the tracker's three listed divergences were not defects.

## Problem
There are still several encodings of one ramp law, each with its own settle and restore checks:
- `WordRamp` (the dynamic band);
- the builtins' `u32` countdown plus `f32` residue stages (fader, matrix, input);
- `IndexedRamp`;
- near-duplicate lane-vector helpers that gather, advance by the law and scatter: the compressor's `RampVec`, the transient shaper's `LinkRamp` and the limiter's `RampLanes`.

Each copy is a place where a restore can admit a settled ramp off its target, which is the defect class #2368 fixed in three effects.

## Contract
- One lane-vector helper in `effect-runtime`, next to `LinearRamp`, that gathers, advances by the law and scatters through `record_advance`. The compressor, transient shaper and limiter use it.
- `WordRamp`, the builtins' countdown stages and `IndexedRamp` either share `LinearRamp`'s validation (`from_words`) or document why their encoding cannot. They are SIMD lane-word encodings: they stay branch-free, with no enums in lane words.
- No rendered bit or state or payload word moves.

## Non-goals
- A stationary hoist for builtin gain retargets. It is cost-only; if it is wanted, it gets its own issue, and it must exclude mute moves, which would move signed-zero bits.

## Gates
- Mutation evidence that each removed duplicate's settle and restore check is still caught by one test in `effect-runtime`.
- No bit moved: per effect, rendered output and snapshot payloads are identical to base (PR evidence).
- Downstream: graph-compiler fixtures and host-core are green; struct-size changes regenerate `graph_fixture`.
- Split by encoding if the brief cannot reach a closable slice in half a day.

## Evidence / attempt record
(empty)
