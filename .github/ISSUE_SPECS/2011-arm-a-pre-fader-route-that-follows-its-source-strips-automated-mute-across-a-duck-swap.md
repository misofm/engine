# Arm a pre-fader route that follows its source strip's automated mute across a duck-swap

Filed by root from slice 1324a (backlog sprint, 2026-10-25).

## Defect
At a duck-swap (#1324), a restarted strip's pre-fader routes are armed so they fade in after `S`. A pre-fader
route that follows its source strip's mute (`follows_mute`) while that mute is automated cannot be armed:
arming it needs graph work that 1324a does not have. Such a route restarts at its prepared value at `S`,
which is an audible step.

## Product outcome
A following pre-fader route of a restarted strip fades in after `S` like every other armed route, with its
followed mute applied, and no step.

## Smallest closable slice
The graph support to arm a following route whose source mute is automated, the preparation that arms it at a
duck-swap, and one gate in the 1324a gate-1 shape (bits against the reference runs, zero allocations).

## Paths
`crates/graph/src/{lib,runtime}.rs`, `crates/host-core/src/prepare.rs`, `crates/host-core/tests/`, this spec.
