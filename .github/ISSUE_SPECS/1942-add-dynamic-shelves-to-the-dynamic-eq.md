# Add dynamic shelves to the dynamic EQ

Filed by root as #17's slice S4 (backlog sprint, 2026-10-24). **Owner-gated:** the launch shape is four bell bands; dynamic low and high shelves (a band `kind` parameter, id 13) wait for the owner's decision.

## Smallest closable slice (if the owner approves)

A per-band `kind` (bell, low shelf, high shelf) with the shelf's static design from `parametric_eq::design_svf`, the dynamic mix law for a shelf, the detector at the user's band, an f64 oracle row per kind, and the joint word retarget (#1928) across a kind change.

## Paths

`crates/dynamic-eq/**`, `crates/dsp-reference/src/dynamic_eq.rs`, this spec.
