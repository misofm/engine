# Cost-weighted mono/stereo remainders for console cohorts

Draft slice 7 (#2137) of the banking design. Amends #1098 amendment 10 (the retired #971 demotion for
console slots) with a stated objective. Class A: a mono-symmetric strip rendered dual-mono emits
the same words on both planes as its collapsed render (the collapse's own premise).

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Problem

Mono and stereo strips never share a bank, so a mixed session pads two remainders per key class
and window (audit section 2, "Mixed mono/stereo"): at W=8 with 10 mono and 6 stereo tracks, three
banks instead of two. The planner never weighs it for console strips (`banks.rs:479-504`).

## Smallest closable slice

- After the sweep of slice 6b (#2136), per `(rack, program, placed)`: if the mono pool's last group is
  partial with `m` members and the stereo pool's last group is partial with `s` members and
  `m + s <= W`, the mono members join the stereo group (as stereo-class lanes) and the mono
  group is dropped. Objective: planes x banks; a merged stereo bank costs 1.0 against 0.6 + 1.0.
  The rule never touches a full group and never moves a strip whose builtin stage would then
  pool differently: the class map is updated for the moved strips before the builtin planner
  runs (the two-planner obligation, `CohortPoolClass::of_prepare_witness`'s doc), as #971 did.
- Insert cohorts keep #971's rule as it stands.

## Gates (one test per claim, red first)

1. `crates/rack-compiler` unit test: 10 mono + 6 stereo at W=8 forms two banks (8 mono, 2 mono +
   6 stereo); 8 + 8 forms two homogeneous banks; 2 mono + 14 stereo forms two banks. Red on a
   merge of a full mono group. Test value: red when the planner pads two remainders that fit one
   bank.
2. `crates/graph-compiler` half-mono console fixture: the builtin planner and the rack planner
   agree on every moved strip's class (the existing two-planner agreement test, one new row).
   Red on a class map updated after the builtin planner ran.
3. Bit identity on the half-mono fixture before and after (PR evidence).

## Paths

`crates/rack-compiler/src/lib.rs`, `crates/graph-compiler/src/banks.rs` (`stranded_mono_tracks`
rewritten as the remainder rule), its tests, this spec.

## Dependencies

Slice 6b (#2136), and #2131 (`reset_lane` and the per-lane un-bypass law): moving a strip between pools
is a regrouping, class A only under that law.
