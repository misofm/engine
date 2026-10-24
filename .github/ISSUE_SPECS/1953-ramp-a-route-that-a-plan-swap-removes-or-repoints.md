# Ramp a route that a plan swap removes from or re-points on a surviving strip

Filed by root as #1363's slice 2 (backlog sprint, 2026-10-24). #1363 slice 1 covers routes that fade in.

## Smallest closable slice

1. D4 of #1363's spec: a fading route kept alive across the swap with its line aliased, firing its ramp to silence at the replacement block.
2. D5: retire the faded route by the applied-revision watermark, never on the render thread.
3. #1363's gates 2, 4, 5 and 7 (removed route fades out; re-pointed route fades out at the old target and in at the new; retirement allocates nothing on render; the watermark retires it exactly once).

## Order

After #1363 slice 1 and after #1325 (remove a strip in two phases), so that one design also covers a removed strip's sends.

## Paths

`crates/graph/src/{lib.rs,runtime.rs}`, `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`, the C header comments, this spec.
