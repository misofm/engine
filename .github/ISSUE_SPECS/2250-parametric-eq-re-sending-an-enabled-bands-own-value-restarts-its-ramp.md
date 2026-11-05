# Parametric EQ: re-sending an enabled band's own value restarts its ramp, so it never ends at small quanta

Filed by root from the #2247 verify (2026-11-05). A defect on main, in the same class as #2247; the fix moves bits, so it needs an owner ruling before a build.

## Problem
An enabled parametric-EQ band whose row is written again with its own value on every block (automation that holds a value, or a host that re-sends unchanged values) restarts its 64-frame ramp each time (`Channel::start_ramp`, `crates/parametric-eq/src/lib.rs`). Below a 64-frame quantum the ramp never ends, so `no_ramp_in_flight` stays false: the stationary path, the hoist, rest and elision are blocked, and the whole bank stays on the ramped kernel. #2247 fixed only the disabled band (a restated identity); the general rule "keep an in-flight ramp whose target words equal the new target" was left out because it moves bits on this path (#2247 scenario B digests, at 32 and 16 frames).

## Scope
Keep an in-flight ramp whose target words equal the new target words, for any design. State the exact bit-move scope (only re-sent unchanged values at quanta below 64) and confirm that every other path is bit-identical. Measure the render cost saved on a 64-track session at quantum 32 with held automation.

## Gates
- A test at quantum 16 that re-sends an enabled band's value every block: the ramp ends at frame 64 and the band reaches the stationary path. Red on today's code.
- No bit moves at quantum 128 or on any `fixtures/session/v1` session (one-time comparison, PR evidence).
