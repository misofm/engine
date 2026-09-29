# Group bank cohorts by co-silence (optional, class B)

Draft, slice S7 of the silence architecture issue (A0). **Class B** (reorders the master sum); the
owner has ruled class B acceptable when measurably faster, and this slice still needs its own go.
Evidence: `docs/handoffs/silence-2026-09-27/DESIGN.md` sections 3, 4.6 and 6.

## Product outcome

A bank's lanes run in lockstep, so one live track keeps its seven or three bank-mates running. In the
dogfood session's real bank order (sorted track ids, which are content hashes), 50.2 % of eight-lane
bank-blocks are entirely silent although 90.9 % of track-blocks are. Grouping tracks into cohorts by
co-silence raises that to 77.8 % (74.4 % when optimised for a 1 s settle allowance, holding 69.1 % at
1 s against today's 36.2 %); at four lanes 69.9 → 83.9 %. Measured over the whole song, 64 tracks,
native eight lanes: the console strip 98.1 µs → 78.7 µs mean from grouping alone (the effects'
bank-wide claims engage more), 67.0 µs with S4; builtins-only 24.5 → 12.7 µs with S4.

The user's visual track order is unchanged; only bank membership and the master accumulation order
follow cohorts.

## Design

* **Activity maps at import.** When a source is imported or edited, a worker computes one bit per
  128-frame block (digital silence or absent) per source channel pair, off the render thread (the
  decoder already touches every word; S1 computes the same fact per played block). Stored with the
  source's prepared resources; bounded by duration (6 KB per 142 s stem).
* **The planner.** The cohort planner keys on `(level, id, program)`
  (`crates/graph-compiler/src/banks.rs`; `bank_membership_is_independent_of_entry_order`,
  `crates/graph-compiler/src/lib.rs:4126`). With an activity hint, within each `(level, program)`
  class it orders members by a deterministic co-silence heuristic (greedy seed by most-silent, grow by
  the largest intersection of silent blocks, then bounded pairwise-swap hill climbing; the handoff's
  `stem-silence-banking.rs.txt` is the reference) and chunks into banks. Without a hint, today's
  order: every existing session and row keeps its bits.
* **The master order.** `route_fold` declines when chain order and reduction order disagree
  (`route_ids_ordered_against_the_cohorts_decline_the_route_fold`, `:10249`). A hinted plan
  reorders the master accumulation to cohort order so the fold stays admitted. This is the class-B
  change: the sum's association differs, so output bits differ from today's order by rounding.
* **Edits.** An edit that changes a source's activity map schedules an off-thread recompile; the plan
  swaps at a block boundary as every structural change does.

## Non-goals

No change to the user's visual track order, to sessions without an activity hint, or to the silence
skip itself. No regrouping across effect-program signatures.

## Authorized paths

`crates/graph-compiler/src/{banks.rs,lib.rs}`, `crates/graph/src/program.rs` (fold order only),
`crates/host-core/src/prepare.rs` (passing the hint), `crates/source/` (the import-time map),
tests, this spec.

## Objective gates

1. Determinism: the same session and activity maps produce the same plan bytes on every run and target.
2. No hint, no change: every existing digest and graph-resource fixture is unchanged.
3. Class-B record: for a hinted plan, the output differs from the unhinted plan only by summation
   order (an independent `f64` reference of the master agrees with both within the documented bound),
   and the fold is admitted (count). Per the class-B ruling ([#944's spec](https://github.com/misofm/engine/blob/80c4119b9e6814cb450e87568243d6df9b6be7bc/.github/ISSUE_SPECS/944-drop-the-per-frame-identity-select-from-settled-2x2-matrices-that-have-no-identity-lane.md), "wanted
   when it measurably improves performance, with digests re-baselined and the new arithmetic
   pinned"): the hinted rows' digests are pinned, the re-baseline is recorded, and the new
   accumulation order is pinned by a test.
4. The sparse rows' hinted twins are measured once at the batch boundary and recorded against the
   unhinted rows.

## Console benchmark rows

Hinted twins of the S2 rows (new kinds). Existing rows unchanged.

## Dependencies

S4 (the latch it feeds), S2 (the rows).

## Standing rules for the implementer

As S4, except that bit identity is replaced by gate 3 for hinted plans only.
