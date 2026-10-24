# Retarget an SVF's ramp words jointly in every shipped effect

Filed by root from the #1372a verification (backlog sprint, 2026-10-24). **May be class B: an owner ruling is needed for any shipped effect whose rendered bits move.**

## Finding

A state-variable filter's three design words (c1, a2, a3) ramp as separate `LinearRamp`s. If a retarget restarts only the words whose target bits changed, a word that is mid-ramp keeps its old schedule while the others restart, and the loaded word triple leaves the chord between two designs. Such off-chord triples are not contractive: the #1372a verifier measured a spectral radius of 1.36 at c1 and a3 at 1 kHz with a2 at 12 kHz (k = 1/8, 48 kHz), and 1.008 at a geometric-restart point. In the de-esser this was reachable (4452 of 8000 one-ulp frequency nudges at Q 8 / 48 kHz keep a2's bits); the de-esser is being fixed before registration (PR #1922). The parametric EQ's ramp-path tolerance (`crates/effect-runtime/src/svf.rs`, `RAMP_PATH_NORM_TOLERANCE`, and #1859's ramp-path grid) assumes lockstep ramps.

## Smallest closable slice

1. For each shipped effect with SVF words or other multi-word filter designs (parametric EQ, the builtins input HPF/LPF, the multiband crossovers, the dynamic EQ), state by reading and one probe whether a retarget can restart some words and not others.
2. Where it can, the fix is a joint retarget (restart every word of the design from its current value whenever any target word changes). For a shipped effect this moves bits on retarget paths: bring the per-effect evidence to the owner before the change (class B).
3. Where it cannot, record why, and add one assertion to the effect's existing ramp-path test that would catch a future per-word skip.

## Paths

Per effect, decided by the probe; this spec.
