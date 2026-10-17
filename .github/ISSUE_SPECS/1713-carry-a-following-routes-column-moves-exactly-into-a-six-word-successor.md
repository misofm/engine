# Carry a following route's column moves exactly into a six-word successor

Filed by root from batch 18 (#1652 U2a fold 2, MAJOR-A; root, 2026-10-17: "exact form not now",
phase 2). Stateless: everything needed is below.

## Problem

A send into a submix whose source strip has a mute entry is a *following route*: a seven-word
route cell and render state with one ramp per source column (#1652 D1, root ruling R18-5). A
rebuild that removes the strip's last mute entry makes that send a six-word route again
(#1347's cell, one ramp for all four coefficients). The carry by route ID (#1652 D3) then moves
two column ramps into one ramp, and render can re-time a move that is in flight at the swap:

- (a) `finish_six_word_carry` (`crates/graph/src/runtime.rs`): when the join's record, composed
  with the curve the predecessor heard, differs from it, the record's open columns ramp over the
  session mute length `J`, not over the record's own length `R`;
- (b) `ColumnRamps::one_ramp` (same file): two column moves in different states fold into one
  ramp from where the route is to the composite target over the longer remaining length, so the
  shorter move is stretched.

Today (#1652 U2a fold 2) such a rebuild completes as `CandidateOutcome::TransitionFallback`: host
core's send join (`resolve_route_carry`, `crates/host-core/src/prepare.rs`) sets
`PlanStateInventory::carries_a_following_route_into_a_six_word_route` when a move's predecessor
row follows and its successor row is six-word, and the control plane's rebuild
(`crates/control-plane/src/control.rs`) marks the candidate. The flag is conservative: it is set
for every such rebuild, also when no column moves at the swap.

## Exact form

Keep a seven-word successor while the predecessor follows: the successor's route stays a
following route whose source has no automated lane (`automated == [false; 2]`, so the gate is
`terms` alone, and every column keeps its own ramp), until the next rebuild, which prepares it
six-word. Costs and limits:

- Validation refuses a following source with no automated lanes today
  (`GraphFollowRouteSource::new` refuses "no automated lane"); this issue allows it for a carried
  route only.
- About 470 B per such route (the seven-word owner, cell, binding and producer against the
  six-word ones), charged by `size_of`, until the next rebuild.
- #1653's live commit writes the seven-word record to such a route.

## Gates

1. A rebuild that removes the mute entry while one send column is mid live move: the move ends at
   its own length, bit for bit the never-swapped twin's column (both hosts, the C ABI and the
   browser at the native width; host-core at `Simd8` under avx2, `Simd4` and the scalar oracle).
2. Two column moves of different lengths at the swap each end at their own length.
3. The same rebuild's watermark reports `OUTCOME_EXACT`.
4. The rebuild after that one prepares the route six-word and carries it (seven -> six with no
   automated lane) with no re-timing.
5. Realtime: the swap block allocates and frees nothing.

## Deletes

The conservative flag: `PlanStateInventory::carries_a_following_route_into_a_six_word_route`, its
field and its join line, the control plane's `follow_into_six_word` term, and the gates
`a_rebuild_that_drops_a_following_route_to_six_words_reports_a_fallback` (capi),
`a_rebuild_that_drops_a_following_route_to_six_words_reports_the_fallback_alike` (host-web) and
`the_join_flags_a_following_route_carried_into_a_six_word_one` (host-core), replaced by gate 3.
