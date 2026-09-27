# Keep the mono pool a whole number of cohorts

Draft, not yet a GitHub issue. Class A (regroups lanes; no lane's arithmetic changes). Base
`6ca203f8`. Depends on: "Arm the mono collapse only on chains that gather the track input".

## Problem

The cohort planner pools collapse-eligible tracks apart from the rest (`CohortPoolClass`,
`crates/rack-compiler/src/lib.rs:48-102`; derived once in
`crates/graph-compiler/src/compile.rs:433-444`). Effect banks bind only **full** groups (#96 F7),
so each pool whose size is not a multiple of the lane width strands its tail: those tracks' effects
render per node, in scalar code. `a_single_odd_track_strands_both_pools_remainders`
(`crates/graph-compiler/src/lib.rs:8307-8331`) documents it.

Measured on the dogfood first-listen session (81 tracks, 18 dual mono folded to `(0, 0)`, every
track given the standing EQ + compressor + limiter strip; `DUAL-MONO.md` §4.3-4.4): today's
pooling makes the mix **33% slower at 8 lanes** (169 to 225 µs) and 1% slower at 4 lanes, almost
all of it stranding (about 65 µs at 8 lanes). A prototype that keeps the mono pool a whole number
of cohorts makes the same session **5.3% faster at 8 lanes and 7.6% faster at 4 lanes**, with
digests unchanged.

## Outcome

Within each group of tracks that would share a cohort program **with at least one effect slot**,
keep `floor(n_mono / W) * W` mono-class tracks (the first in normalized track order) in the mono
pool and move the rest to the stereo pool, where they render dual. Groups whose program is builtins
only are left alone: builtin banks pad a partial cohort, so the mono remainder still collapses
there (measured: on the builtins-only dogfood mix at 4 lanes, demoting costs 0.7 µs).

## Read first

* `crates/builtins-compiler/src/lib.rs:3996-4060`: `SessionPoolClasses` (`from_session`,
  `conjoin`, `class_of`). Both planners read this one object; the demotion must happen in it,
  before either planner runs, or the two planners' lane sets diverge and every chain merge declines
  silently (`CohortPoolClass::of_prepare_witness` doc).
* `crates/graph-compiler/src/compile.rs:424-455` and `bind_rack_banks_indexed`
  (`crates/graph-compiler/src/banks.rs`) for how candidates are grouped (level, rack, program).
* The prototype (global rather than per group, so only valid for a one-program session):
  `research_demote_mono_remainder` in `docs/handoffs/dual-mono-2026-09-27/dual-mono-prototypes.patch`.

## Authorized paths

`crates/builtins-compiler/src/lib.rs` (`SessionPoolClasses`), `crates/graph-compiler/src/compile.rs`,
`crates/graph-compiler/src/lib.rs` tests, this issue's spec.

## Design notes

* Demote by clearing the class only (for example `SOURCE` in the pool map); never touch the
  structural join in `host-core` (`prepare.rs:1641-1646`). A chain holding a demoted track together
  with stereo tracks is not armed; a chain of only demoted tracks is armed and collapses correctly.
* The group key must be what makes two tracks share a cohort in both planners (dependency level and
  the ordered effect program of `simd1`, `dynamic`, `simd2`).

## Gates

* New test: the 64-track mono fixture (`fixtures/session/v1/console-sixty-four-track-mono.json`)
  cloned to 81 tracks, with `right_source_channel = 1` on every track except positions
  0, 4, 10, 18, 19, 20, 22, 35, 36, 38, 40, 42, 43, 44, 53, 58, 60, 66 (the dogfood layout). At the
  host width: `prepared_bank_count()` equals the all-stereo version's (30 at 8 lanes, 60 at 4);
  rendered digest equals today's pooling's; `bank_collapse_counters()[1]` is 2 at 8 lanes (4 at 4).
* Update `a_single_odd_track_strands_both_pools_remainders`: with 63 mono and 1 stereo track the
  pools become 56 and 8, so nothing strands and the bank count equals the unsplit session's. Keep
  its digest assertion.
* Coordinate with #969 ("Pin the input-section symmetry witness in the pool-class grouping"): its
  gate counts stranded banks (21 against 24) on exactly this one-odd-track shape, which this issue
  removes. Land #969 first and re-point its assertion at the class map, or fold it in here.
* Unchanged: `cargo test -p console-workload --test chain_shape`, `-p graph-compiler`,
  `-p host-core --test symmetry_witness`.
* `cargo fmt --check`, clippy `-D warnings` on the touched crates.

## Non-goals

Partial effect banks (#96 F7), the route-fold conflict (separate ruling), detection.
