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

## Evidence and decision record (attempt 1, 2026-10-26)

Branch `codex/batch-b13-splitpack` from origin/main `16f548a92`.

### What was built

- **Lane chains** (`crates/graph/src/runtime.rs`, `LaneMap::discover`). A bank unit's predecessor
  is the unit before it when it takes resident input, or the one bank of its producer row when
  `lane_tracks` are equal. A chain is a unit with no predecessor and its successors. A bank whose
  row is not such a link, a resident unit whose row names another unit, and a unit named by two
  successors are in no group, and neither is any bank such a row names. `pack_member_eligible` is
  now `lane_chain_eligible` (the old structural terms). `chain_links_hold` checks that row `j+1`
  gathers lane `l` from row `j`'s final lane `l` and that only the last row folds. `chain_peers`
  is `pack_peers` row by row. A group is the longest prefix of a run of consecutive peer chains
  for which `group_layout_holds`.
- **`PackGroup`** gains `chains`, `rows` and `units_base`; `LaneMap` gains `unit_slot`,
  `chain_units` and `producer_lane`. A position names a chain and a lane in every row.
  `local`, `unit_lane` and `current` translate through rows.
- **`swap_bank_lanes`** checks the lanes of every row, then every row's chains
  (`BankChain::lane_swap_check`, now `pub` in `crates/rack`), and only then trades each row
  (`swap_unit_lanes`: chain state, member ops, fold entries, source bits, observation rows), then
  records the move once. `restore_home_lanes`, the packer and `fold_displaced_group` walk chains.
- **Per-node op rows.** `LaneMap::bind_producer_lanes` records the home lane of each op whose one
  producer is a grouped bank. `execute` asks `producer_unit`, the unit that renders the strip now.
- **Packer facts.** Only a first-row unit records idle facts. A later row's input silence is stated
  only when its predecessor skipped, so its fact would hide every strip whose bank mates run. This
  changes cost, never bits.
- **`test_only_pack_groups`** returns `TestOnlyPackGroup { width, chains }`.

### Decisions taken without a ruling (for review)

1. **The layout term (not in the spec).** A move changes when a strip's member ops run. It is exact
   only when every buffer the group touches holds one value over the group's span. So the group's
   written buffers must be distinct, no first-row gather source may be among them, the master
   must not be among them, and any plain unit inside the span must sit between two whole rows.
   The term is needed. On C1 with its sends at `post_input` (W=8), the second chain's part one
   writes into the buffers the first chain gathered from. Without the term the group forms and the
   packed plan's host words differ at block 20 (mutant M5 below). The same term also guards
   #1981's one-row groups. No existing fixture hit it, but nothing in #1981 ruled it out.
2. **The spec's gate 2 rationale is wrong for C1 as built. The rule stands.** C1's split is
   post-fader, so part two is the matrix alone (collapse prefix 0), and the link is a producer
   row, not resident. Swapping part one only would not move bits there: the buffer carries the
   strip, and part two's row cannot collapse anything. Gate 2 therefore taps the sends at
   `insert_send`, where part two (drive, fader, matrix) has a collapse prefix and is
   `armed_on_input`. No resident chain forms with two or more chains in the shapes tried: resident
   pairing needs adjacency, and the row layout of peer chains breaks it.
3. **Gate 3 lives in `crates/graph-compiler/tests/silent_bank_skip.rs`, not in a `crates/graph`
   unit test.** It needs a compiled plan with a per-node delay. The "consumer with fewer lanes"
   case runs through the same branch as a bank row that names ops: a bank row that is not a link
   leaves the bank ungrouped. The test asserts that branch.
4. **C1's gate needs about 348 blocks of silence to rest from its fresh state** (its release
   from the open state). Packing on `Sparse` therefore starts at about block 363 at W=8. The gates
   render 400 blocks or more (`C1_PACKED`). This is an effect property and outside this slice.
5. **Deleted `console-workload`'s `the_key_class_ideal_packs_across_pack_groups`.** C1's track banks
   are now one group, so the per-group ideal equals the key-class ideal on every standing row (all
   16 checked). The test can no longer turn red on its own mutant.
6. `bank_slot_runs` rows still merge consecutive units of a group. Their `last - first + 1` counts
   the group's units only when no other effect bank sits between them. That holds on C1 and on
   every fixture tried. On a chain-major layout with effect slots on several rows it would
   over-count, which affects test and bench counters only.

### Gates (red on a named mutant, then green)

| gate | test | mutant | red |
|---|---|---|---|
| 1 | `a_split_strip_packs_as_one_strip` | M1: no chain link (today's eligibility) | `the track banks are in a pack group` (no group) |
| 2 | `a_packed_split_strip_renders_the_bits_of_an_unpacked_one` | M2: trade row 0 only | block 404 host word 160 differs (1.5477573e-2 vs 1.5477574e-2) |
| 2 | same | M3: trade the last row only | the packer homes the strips (displaced 0 at the trim); gate 1 also red (BSR stays 8) |
| 2 | same | producer row left unit-keyed, both rows traded | green, as the spec predicts |
| 3 | `a_per_node_insert_reads_the_moved_strip` | M4: op row untranslated | block 404 host word 206 differs |
| 1a | `a_recycled_buffer_keeps_its_strips_apart` | M5: drop "inputs are never written" | block 20 host word 20 differs |
| 4 | `a_split_strip_move_allocates_nothing` | M6: a `Vec` per row in `swap_bank_lanes` | block 10 allocated |

All gates pass at W=8 on this host. The tests assert only width-independent facts (`ceil(16/W)`
chains, the ideal `slots x ceil(8/W)`), but the four-lane native leg was not run here. In gate 1,
C1-16 `Sparse` BSR per block for the track group is 8 before packing and 4 (the per-key-class
ideal) from block 363.

### Bit identity against origin/main (one-time, not committed)

All 16 `BANK_REPORT_WORKLOADS` rows (C1 at 16/32/64 in Load, Typical, AllActive, Sparse and Toggle,
plus the irregular graph) were rendered for 600 blocks (past packing) through
`console_workload::SessionRuntime`. The FNV-1a of every host word is identical on main
`16f548a92` and on this branch, on all 16 rows.

### W=4 pins (`tools/wasm-gates`)

`scripts/run-wasm-gates.sh` reported `pack_groups` 0 -> 1 on all four rows. On this branch the
track banks are one group of lane chains at W=4 too. Within `W4_COUNTER_BLOCKS = 256` no strip
packs: the gate's rest takes about 348 blocks. So every other counter is unchanged. Re-pinned
from the wasm leg's printed rows. The four-lane native leg (arm64) was not run here.

### CPU (descriptive; `bench console --bank-report`, one invocation each, one warmup, two rounds, `taskset -c 31`)

Raw output is in `artifacts/steps/bank-design-5a/`. p50 us per block, round 2, main -> branch:

| row | BSR/block | p50 r2 |
|---|---|---|
| c1 16 all active | 16.66 -> 16.66 | 96.31 -> 97.20 (+0.9 %) |
| c1 16 sparse | 16.66 -> 13.60 | 96.86 -> 78.88 (-18.6 %) |
| c1 32 all active | 24.66 -> 24.66 | 143.46 -> 145.21 (+1.2 %) |
| c1 32 sparse | 24.66 -> 18.54 | 144.72 -> 109.88 (-24.1 %) |
| c1 64 all active | 40.66 -> 40.66 | 243.76 -> 242.91 (-0.3 %) |
| c1 64 sparse | 40.66 -> 28.43 | 245.14 -> 170.84 (-30.3 %) |
| c1 16/32/64 load | 0 -> 0 | 23.61/35.34/60.51 -> 24.86/36.92/62.64 (+5.3/+4.5/+3.5 %) |

`AllActive` is within 2 % on all three sizes. The Sparse BSR is averaged over 1000 timed blocks
that start at block 128, so about 235 of them run before packing. The load rows are 3.5-5.3 %
slower. That is one run, not investigated, and not a gate. The likely cause is that their track
units are now grouped, so each block adds a pass-by of the packer and a fact record. V8: not
measured (the V8 runner has no C1 document, as #2143 recorded).

### Gates run

- `cargo test -p graph -p rack -p graph-compiler`: pass, including `silent_bank_skip.rs` and
  `bypass_skip_warmup.rs`.
- The workspace debug test step with qualification.yml's exact excludes and features: exit 0.
- `cargo test --locked --release -p bench -p console-workload -p wasm-gates`: pass.
- Lint job: `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features -D warnings`,
  `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --no-deps`, and every policy script and
  self-test of the job (`check-lane-policy.sh` included): ok. `realtime_policy`: ok (283 regions).
- `scripts/run-wasm-gates.sh`: ok after the re-pin.
- AudioWorklet: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`,
  `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
  `strip-wasm-names.py check`, `test-web-audioworklet.sh`: ok.
- Not run: the arm64 native legs and the browser legs (Playwright).

## Evidence and decision record (attempt 2, 2026-10-26)

Attempt 1's verdict was FAIL on one MAJOR: the load state costs more. Root accepted the layout
term. Attempt 2 makes these changes:

- **MAJOR, the fact path.** Each unit's chain and row are stored at bind (`unit_chain`,
  `unit_row`), so `local` no longer divides. Only a first-row unit computes the packer facts:
  `lane_input_silence`, the rest query and `record_facts` all run behind `LaneMap::fact_group`.
  A later row's facts were discarded before, and now they are never computed.
  `bind_silence_readers` wants only first-row inputs. `record_facts` computes the chain once per
  call and reads `home` by `chain * W + lane`.
- **MINOR 1, `bank_slot_runs`.** A row is now one `(group, chain row)`, merged only across units
  that are adjacent among the effect banks. Gate 2's fixture taps at `insert_send`, so row 1 has
  the drive. A new assertion in
  `a_packed_split_strip_renders_the_bits_of_an_unpacked_one` checks that the sum of
  `effect_slots x units` over the rows equals the compile's bound banks. Red on attempt 1's
  `bank_slot_runs`: `left: 127, right: 21`. Green now.
- **MINOR 2, `widen_block_end_units`.** It now widens only within the listed unit's row: a move
  carries members between units of one row only. The change is exact and moves no bit.

### CPU (descriptive; one invocation each of main `16f548a92` and this branch, one warmup, two rounds, `taskset -c 31`)

p50 us per block, round 1 / round 2. Raw output is in
`artifacts/steps/bank-design-5a/bank-report.attempt2.*.jsonl`.

| row | main | branch | change |
|---|---|---|---|
| c1 16 load | 23.52 / 23.57 | 23.77 / 24.30 | +1.0 % / +3.1 % |
| c1 32 load | 34.93 / 35.04 | 36.11 / 36.06 | +3.4 % / +2.9 % |
| c1 64 load | 60.87 / 61.05 | 60.85 / 61.18 | -0.0 % / +0.2 % |
| c1 16 all active | 96.45 / 97.27 | 94.97 / 94.70 | -1.5 % / -2.6 % |
| c1 32 all active | 146.10 / 145.19 | 142.02 / 143.12 | -2.8 % / -1.4 % |
| c1 64 all active | 245.76 / 246.98 | 244.07 / 238.78 | -0.7 % / -3.3 % |
| c1 16 / 32 / 64 sparse | 98.23 / 147.45 / 247.30 (r2) | 77.22 / 107.44 / 167.66 (r2) | -21.4 / -27.1 / -32.2 % |

As measured, with no retry. The 16- and 32-track load rows are still above 2 % in at least one
round. What a first-row unit still pays per block in the load state: `lane_input_silence` over
its lanes, and `record_facts`, which resets each lane's idle count to zero. Main pays neither on
C1, because no C1 track unit is grouped there. That is the same per-unit cost #1981 already
accepted for its groups. No further reduction was tried in this attempt.

### Gates run (attempt 2)

- Lint job: fmt, clippy `-D warnings`, doc `-D warnings`, every policy script and self-test, and
  `realtime_policy`: ok.
- Workspace debug test step (exact excludes and features): exit 0, 2647 passed.
- `cargo test --release -p bench -p console-workload -p wasm-gates`: 84 passed.
- `scripts/run-wasm-gates.sh`: ok, and no pin moved in this attempt.
- AudioWorklet build and checks, and `test-web-audioworklet.sh`: ok.
- `silent_bank_skip.rs`: 11 of 11 pass.

## Evidence and decision record (attempt 3, 2026-10-26)

**Lever: the fact path writes nothing when the facts cannot change.** `LaneMap::unit_idle` (one
`bool` per unit) records whether any of a first-row unit's strips has an idle count above zero
after its last `record_facts`. When the block has no quiet lane and that flag is clear,
`record_facts` returns at once. Every count is already zero and stays zero, nothing crosses
`PACK_IDLE_BLOCKS` (so `dirty` misses no change), and the unit adds nothing to the packable count.
The group's first unit still resets that count to zero. The write-nothing path is exact.
`lane_input_silence` still runs: the input facts are what decide whether a lane is quiet, and no
cheaper exact condition was found for skipping it.

**Red row.** `a_strip_that_plays_again_is_not_packable`: C1-16 on `Sparse` packs by `C1_PACKED`,
then every strip plays for 24 blocks, and the group must be home (`displaced == 0`). Mutant: take
the write-nothing path on every block with no quiet lane, ignoring the flag. Red:
`every strip plays again, so the packer took the group home, left: 8`. Green on the real code.

### CPU (descriptive; one invocation each of main `16f548a92` and this branch, one warmup, two rounds, `taskset -c 31`)

Raw output is in `artifacts/steps/bank-design-5a/bank-report.attempt3.*.jsonl`. p50 us per
block, round 1 / round 2:

| row | main | branch | change |
|---|---|---|---|
| c1 16 load | 23.86 / 24.40 | 24.31 / 23.92 | +1.9 % / -2.0 % |
| c1 32 load | 35.057 / 35.146 | 35.768 / 35.467 | **+2.03 %** / +0.9 % |
| c1 64 load | 60.11 / 60.66 | 57.82 / 60.69 | -3.8 % / +0.0 % |
| c1 16 all active | 97.08 / 98.42 | 97.92 / 98.05 | +0.9 % / -0.4 % |
| c1 32 all active | 147.26 / 146.85 | 145.17 / 145.84 | -1.4 % / -0.7 % |
| c1 64 all active | 247.19 / 247.34 | 242.95 / 246.41 | -1.7 % / -0.4 % |
| c1 16 / 32 / 64 sparse | 98.77 / 146.39 / 245.33 (r1) | 80.33 / 110.79 / 172.79 (r1) | -18.7 / -24.3 / -29.6 % |

As measured, with no retry. One round of one row is above 2 %: the 32-track load, round 1, at
+2.03 %, which is 0.71 us per block. Every other load round is within 2 %. Per the brief, tuning
stops here and the trade-off goes to the owner.

### Gates run (attempt 3)

All ok:
- Lint job: fmt, clippy, doc, every policy script and self-test, and `realtime_policy`.
- Workspace debug step: 2648 passed.
- Release tests: 84 passed.
- `run-wasm-gates.sh`: no pin moved.
- AudioWorklet build and checks, and `test-web-audioworklet.sh`.
