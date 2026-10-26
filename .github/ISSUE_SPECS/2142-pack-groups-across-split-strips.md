# Pack groups across a split strip: one permutation per strip, over every unit that renders it

Draft slice 5a (#2142) of the banking design (`docs/BANKING_DESIGN.md`). GitHub number to be assigned.
Files before #2135 (pattern packing), which depends on it. Found by #2132's review: a design gap,
not an implementation defect.

Owner direction (2026-10-26): "I want the ideal implementation that maximizes banking without
sacrificing correctness or flexibility. The user should be able to eventually do whatever they
want to do on the frontend in terms of creating tracks/auxes/sends/etc and the engine should
figure out how to use banking to maximize performance."

## Class

A: a move stays a lane permutation; it now applies to every unit of a strip at once, so every
positional relation between those units is preserved by construction and no lane's words change.

## Problem

- C1's two `post_fader` sends split every track strip at the fader
  (`crates/graph-compiler/src/lib.rs`, the post-fader meter/send split that
  `a_meter_leased_at_post_fader_splits_the_chain_and_still_meters` pins at `:11050`): part one is
  the fused 6-slot chain, part two a 1-slot matrix bank.
- Part two reads part one by **position**, twice: the resident pairing
  (`UnitIdentity::resident_input`, `acquire_resident_input` copies the predecessor's AoSoA block
  lane for lane, `crates/rack/src/lib.rs:4400`) and the collapse-proof producer row
  (`input_producer_table`, `crates/graph/src/runtime.rs:11426`), whose read asks the producer
  **unit's** `BankChain::planes_identical()` (`runtime.rs:5272`).
- `pack_member_eligible` (`runtime.rs:10023-10028`) therefore drops both parts (resident input on
  either side, a producer row, a referenced unit), so no C1 track bank is in a pack group and
  #1981 gives the console nothing. The gate rows in #1981 used sessions without sends.
- Admitting part one alone is unsound: a swap carries the member ops (`runtime.rs:10372`), so a
  planar gather stays right, but the resident block and the producer row are positional. After a
  swap, part two's lane `i` would copy another strip's resident block (audio bits move), and its
  row would ask the wrong unit's `planes_identical`, which can collapse two channels that differ
  (bits move).

## The rule

> **A strip is packed as one strip.** The unit of packing is the **lane chain**: the ordered
> bank units that render the same `lanes` strips at the same positions (`identity.lane_tracks`
> equal position for position), linked by resident pairing or by a bank producer row. A pack
> group is a run of two or more **peer lane chains** (equal length, and unit `j` of each a peer
> of unit `j` of the other by today's `pack_peers` rule). One `swap_bank_lanes(a, b)` trades the
> strips at positions `a` and `b` in **every** unit of both chains, with every refusal checked in
> every unit before any write. Resident pairing and bank producer rows are then invariant: lane `i`
> of unit `j+1` still reads lane `i` of unit `j`, and a consumer unit's lanes still have their
> producers in the one producer unit the row names.

Why this and not mapping the row through the lane map: the row can be translated (one load per
block), but the resident block cannot; it is a contiguous positional copy, and translating it
would turn the resident path into a gather, which is the round trip the resident path exists to
remove. One permutation keeps both relations correct with no new read on the render path, and
the per-block pack step already pays per strip, not per unit (the cycle walk moves a strip; the
swap cost grows by the extra units' state, which the budget `B_total` already bounds per move).

Generality, for any graph:

- **Any cut that splits a strip** (a send at any tap, a leased stage meter, a sidechain tap out of
  a stage, a PDC staging point) produces adjacent or near-adjacent bank units of the same strips;
  all are links of one lane chain by the two relations above. Nothing in the rule names the
  fader.
- **A per-node op inside a strip** (aux A's delay between its pre and post segments, a
  sidechained insert, a third-party effect later) ends the lane chain on both sides: the op reads
  and writes by buffer, which the member-op swap carries, so each side is its own lane chain and
  may pack with its own peers. Its own producer row (#1899) names a bank unit; that row becomes a
  **home position** resolved through `LaneMap::at` at its one read per block (the one
  translation this slice adds, on a per-node path, never on a bank's), so a mono track with a
  per-node insert keeps its collapse proof. A bank consumer whose lane set differs from its
  producer's (a subset of strips) ends the chain and both stay ungrouped: today's behaviour,
  correct, and reported in #2132's `ungrouped_units` count.
- **`referenced`** is kept for references from outside a lane chain and dropped inside one.
- The deferred home-order fold (#1782a), observation rows, eligibility rows and fade arms
  translate per unit as today; the carry restores the identity across every unit of a chain
  (`restore_home_lanes` walks chains, not units).

## Smallest closable slice

- `LaneMap::discover`: build lane chains (a pass over adjacent units and producer rows), then pack
  groups over peer lane chains; `PackGroup` gains `units_per_chain` and a per-chain unit list;
  `group_of` maps every unit of a chain to the group.
- `Runtime::swap_bank_lanes`: the chain swap, member ops, fold entries, `source_lanes`,
  observation rows and the stale/hold bits (#2129) for every unit of both lane chains, refusals
  first across all of them; `restore_home_lanes` and the carry over chains.
- Per-node op producer rows resolved through the map at their read.
- `pack_member_eligible` rewritten as `lane_chain_eligible` (resident input and bank producer
  rows allowed inside a chain; a trailing active mask, no aux lane, fold all-or-none, at most
  eight lanes per unit as today).
- #1981's packer (`pack_group`) unchanged in rule; it addresses positions, which now move whole
  strips.

## Gates (one test per claim, red first on a named mutant)

1. `crates/graph-compiler/tests/silent_bank_skip.rs`, `a_split_strip_packs_as_one_strip`: the
   C1-16 fixture (#2132; two post-fader sends on every track): every track's two units are in one
   pack group (`pack_groups()` reports 2 units per chain and `ceil(16/W)` chains per group), and on
   the `Sparse` state the track group's bank-slot runs reach the per-key-class ideal
   `sum over slots of ceil(A/W)` (8 of 16 tracks silent: 1 bank at W=8, 2 at W=4) after
   `PACK_IDLE_BLOCKS` blocks. Red on today's eligibility (no group; BSR stays at the unpacked
   count). Test value: red when a send keeps a console strip out of packing.
2. Same file, `a_packed_split_strip_renders_the_bits_of_an_unpacked_one`: a packed plan against
   its unpacked twin over a seeded silence schedule, with a **collapse-armed mono-pool bank in
   which one strip's two channels differ at render** (its right-channel trim moved live, so its
   chain's `planes_identical` is false while its bank mates' are true); every host word,
   post-matrix word and the master equal by `to_bits`; the matrix bank's collapse count on that
   strip is zero in both. Red on the mutant that swaps part one only (the resident copy reads
   another strip: host words differ at the first packed block) and on the mutant that leaves the
   producer row unit-keyed while swapping both (it passes; that is the point: one permutation
   needs no row translation; the test's third mutant translates nothing and swaps only the
   consumer, which collapses the differing strip and moves its bits). Test value: red when a pack
   moves a split strip's bits or collapses channels that differ.
3. `crates/graph` unit test: a mono track with a per-node delay insert whose pre bank is packed:
   its post bank's collapse decision follows the moved strip (the op row resolves through the
   map), and a track whose consumer bank has fewer lanes than its producer stays ungrouped. Red on
   an untranslated op row. Test value: red when a per-node insert loses or misreads its collapse
   proof under packing.
4. Allocation: #1981's carried-pair gate on the C1-16 fixture; zero across the chain swaps and
   the carry.
5. CPU, descriptive: #2132's `Sparse` and `AllActive` C1-16 rows, native and V8, before and after;
   `AllActive` within 2 %.

## Paths

`crates/graph/src/runtime.rs` (`LaneMap::discover`, `PackGroup`, `lane_chain_eligible`,
`swap_bank_lanes`, `restore_home_lanes`, the op-row read), `crates/graph/src/lib.rs`
(test-support counters), `crates/graph-compiler/tests/silent_bank_skip.rs`, `docs/BANKING_DESIGN.md`
(section 1.1: the pack group is over lane chains; section 3.3 the move), this spec.

## Dependencies

#1981 and #2131 merged; #2132 for the C1-16 fixture and the counters (its rows are the gate's
instrument; if #2132 has not landed, gate 1 builds the fixture locally and the counter is
`test_only_bank_silent_skips`). Blocks #2135. Independent of #2129, #2133, #2134.
