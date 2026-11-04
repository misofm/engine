# Parametric EQ: a touched row restarts a disabled band's disable ramp, so it never completes at small quanta

Filed by root from the #2091 research and its verify (2026-11-04). A defect on main today, independent of #2091.

## Problem
A disabled parametric-EQ band's disable ramp restarts whenever a row of its section is touched: each touched row sends the identity target again (`crates/parametric-eq/src/control.rs:198-226`), and `apply_prepared_target` / `start_ramp` (`crates/parametric-eq/src/lib.rs:1565, 1606`) restart the ramp because nothing compares the new target with the one already in flight. With a render quantum below 64 frames and an automated (or repeatedly set) parameter on a disabled band, the 64-frame disable never completes, so P5 (re-enable from zero state, the #1337 D4 contract at `lib.rs:1602`, "whatever its history") never fires, and the band never reaches rest, which blocks rest detection and elision. Browser hosts (quantum 128) are not affected; C ABI and mobile hosts with small quanta are.

## Scope
Do not restart a ramp whose in-flight target equals the new target (the identity for a disabled band, and in general any equal target words); keep the remaining frames. Show which rendered bits move (only the restarted-disable path at quanta below 64) and confirm that every other path is bit-identical.

## Gates
- A test at quantum 16 with a parameter written every block on a disabled band: the disable completes at frame 64 and a later enable starts from zero state. Red on today's code.
- No bit moves at quantum 128 or on any `fixtures/session/v1` session (one-time comparison, PR evidence).
- Bits move on the restarted path at small quanta: this is the fix of a broken contract, recorded for the owner with the #2091 hold ruling.

