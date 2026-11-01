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
`a_rebuild_that_drops_a_following_route_to_six_words_reports_a_fallback` (capi; deleted by TA5),
`a_rebuild_that_drops_a_following_route_to_six_words_reports_the_fallback_alike` (host-web) and
`the_join_flags_a_following_route_carried_into_a_six_word_one` (host-core), replaced by gate 3.

## Note (controls coordinator, batch 19, 2026-10-17)

With #1654, a route with only route-row automation (rows 1-6, no source mute entry) also uses the
seven-word cell. Its seven-to-six carry (its last route entry removed) is flagged by the existing
conservative flag (`resolve_route_carry` reads `GraphLiveRoute::following`); no new code. This issue's
exact form must also cover that case.

## Decision record (attempt 1, 2026-11-01)

- **Root Q1: form B, not the exact form above.** A six-word route holds its column ramps
  (`ColumnRamps`, two `IndexedRamp`s and two positions inline in the route's box, so a carry is a
  bounded copy and render grows nothing). The seven-to-six carry adopts the predecessor's ramps
  verbatim, and `finish_six_word_carry` applies the join's record per column (a column retargets
  only when its bits change, over the record's own length `R`) and reopens only a column the
  heard curve zeroed, over `J`. `ColumnRamps::one_ramp` is deleted. Form B has no sticky
  seven-word successor, so gate 4's hole (a `terms` change in flight at the next rebuild that
  folds again) cannot arise, and it needs no plan-versus-model split and no relaxed
  `GraphFollowRouteSource::new`.
- A six-word route's own changes (a record, a fade-in or fade-out) restart both columns on one ramp
  from where they are, exactly as before. `route_mix_column_ramps_block` mixes one shared ramp at
  one position bit for bit as `route_mix_ramp_block`, so no six-word route outside a seven-to-six
  carry moves a bit.
- **Root Q4:** `apply_pending_heard` keeps its zeroed-column composition (MAJOR-3).
- **Owner ruling Q3 (2026-10-31): the edge-case bit move is accepted.** A send that changes from
  the seven-word to the six-word shape while its columns ramp in different moves (case (b)), or
  whose join record meets a zeroed column (case (a)), now renders bit for bit like the
  never-rebuilt twin and reports `OUTCOME_EXACT`. Before, one column's move was re-timed at the
  swap and the rebuild reported `TRANSITION_FALLBACK`.
- The conservative flag goes with #1718's (one PR closes both): the inventory flag, its join
  line, the control plane's `follow_into_six_word` term, and the doc text on the C header, the
  ABI constant, `docs/C_ABI_V1_QUALIFICATION.md` and the browser `.d.ts` (and its SDK copy).
- Every live route now mixes by `route_mix_column_ramps_block`, so the one-ramp kernel left the
  browser worklet; its roster and render-kernel rows move to the column kernel (48 vector / 0
  scalar on the named twin).

## Evidence (attempt 1)

- Graph tests, each red on `7d89eecc5`:
  `a_six_word_successor_keeps_each_columns_move_in_flight` (case (b): columns at positions 40 and
  10 of the mute length; the swap block equals the never-rebuilt twin's; red: `one_ramp` folds
  them) and `a_six_word_successors_record_composes_with_the_heard_curve_then_reopens` (case (a),
  rewritten: the open column keeps `R`, the zeroed one reopens over `J`, equal to the twin whose
  curve turns off at the swap; red: the reopening restarted the open column over `J`).
- Host gate (gate 1, 2, 3 on both hosts): `a_rebuild_that_drops_a_following_route_to_six_words_is_exact_alike`
  (host-web, both hosts) removes `t1`'s separate left and right mute entries while the columns
  ramp at positions 324 and 124. Every block equals the never-rebuilt run and the watermark reads
  `OUTCOME_EXACT`. Red on `7d89eecc5` at the swap block's bits. It replaces
  `a_rebuild_that_drops_a_following_route_to_six_words_reports_the_fallback_alike`;
  `the_join_flags_a_following_route_carried_into_a_six_word_one` (host-core) and #1594's
  `a_configure_that_supersedes_a_fallback_edit_reports_the_fallback` and
  `a_configure_that_supersedes_a_monitor_candidate_reports_no_fallback` (control-plane; they held
  the outcome terms a superseding monitor rebuild inherited, which are gone) are deleted.
- Gate 4 needs no test of its own under form B: the successor is six-word at once.
- Gate 5 (one-time, scratch): a control-plane rig removes `eq0`'s mute entry at block 8, inside
  the mute ramp; all 24 blocks, the swap block included, allocate and free nothing, and the send
  is open at block 14. `realtime_policy` passes.
- No bit moved elsewhere (one-time, scratch): the 34 digests over `fixtures/session/v1` (300
  blocks, plain and live; three fixtures refuse alike) are identical at `7d89eecc5` and at head.
- The resource ceilings hold: `LiveRoute`'s ramp state grows by 56 bytes (one more `IndexedRamp` and position),
  inside every pinned ceiling; no re-pin was needed.
