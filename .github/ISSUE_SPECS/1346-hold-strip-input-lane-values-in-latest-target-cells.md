# Hold strip input-lane values in latest-target cells

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A strip's live input trim, polarity and HPF/LPF filter targets are held in latest-target cells
instead of the input lane's bounded queue, through shared code on both hosts. Any number of input
edits between two render calls is accepted; render applies the last committed value of each cell
at the next block, keeps the channel-symmetry witness exact, and #1312's counter records every
replaced value. The C ABI's input edits (#1261, #1262) are built on these cells.

## Context

- **Records.** `TrackInputRecord::{TrimDb, PolarityInvert, PreparedFilter}`
  (`crates/builtins-compiler/src/lib.rs:175-211`). A filter target is
  `PreparedInputFilterTarget { lanes, section, pair: [f32; 2], coefficients: [f32; 6] }`
  (`crates/builtins/src/filter_control.rs:20-26`); one final pair yields two section targets
  (`PreparedInputFilterPair`, `:30-33`).
- **Ring.** One bounded SPSC per strip (`lib.rs:3590`), `TrackControlProducer::input`
  (`:266-269`), charged at `:3796`.
- **Render.** `BuiltinBankProcessor::drain_controls` (`:451-499`) pops every record available at
  entry, admits each into the lane's channel-symmetry witness (`:479-481`) and applies it
  (`set_trim_db`, `set_polarity_invert`, `apply_prepared_filter`). The input section is
  upstream of the seam (`SeamSide::UpstreamOfSeam`, `:213-216`), so the witness gates the mono
  collapse. At a plan swap the predecessor drains once more for the carry (`drain_for_carry`,
  `:590-594`).
- **Browser admission.** The input band (`queue_available`, `hosts/host-web/src/lib.rs:1771-1782`;
  `push`, `:1816-1820`) and the per-strip filter shadows (`input_filter_shadows`, `:1562`).
- **C ABI.** No input lane today (`crates/control-plane/src/compile.rs:19-28`); #1261 attaches one
  and #1262 adds filter targets, both written to these cells.

## Decisions frozen for this slice

- **D1. Cells per strip**, on #1312's primitive: trim left and right (value bits, ramp), polarity
  left and right (value, ramp), and one filter cell per `(section, channel)` (section 0 and 1;
  the pair and the six coefficients, eight words). A `Both` record writes both channel cells. One
  dirty word per strip.
- **D2. Canonical drain order:** trim, then polarity, then filter targets (section 0 before 1),
  left before right, every block, under the block's live snapshot (#1312 D12, Amendment 1). When
  both channel cells of a kind are `Applied` in one block with equal words, render applies one
  `Both` call.
- **D3. The witness folds from final contents.** For each kind drained, equal left and right
  words are admitted as `SymmetryEvent::Preserve`, any other pair or a lone channel as
  `Desymmetrize`, through the same `admit` hook, so a record kind still cannot reach render state
  without declaring its witness effect.
- **D4. A filter target is a level.** A later target for the same lane and section replaces an
  undrained one; the ramp starts from the coefficients render holds (#1262 D2).
- **D5. The carry drain** applies every dirty cell of the predecessor before the input lanes
  carry, as it drains the queue today; a refused apply is counted as now. It reads with
  `LiveSnapshot::ALL`, which on the quiescent predecessor equals the adoption block's snapshot
  (#1312 D6; Amendment 1).
- **D6. Producers and admission.** `TrackControlProducer::input` becomes a cell writer with
  infallible writes; the input ring and its rows go. The browser's input band leaves the room
  pass and `in_flight`; its filter shadows stay the control-side authority for the designed pair.
- **D7. The acked-batch question: can an ack ever precede a drop? No.** Every fallible check (the
  trim and polarity domain checks, the filter design) precedes the first cell write; writes
  cannot fail; a replaced value is in the committed model and counted; the swap drain applies
  whatever was written before it.

## Deliverables

1. Input cells in `builtins-compiler` (bank and test-only scalar processors), D2-D5.
2. Browser admission (D6) and the producer type the C ABI's #1261 writes to.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (after #1312; then stream D), `crates/builtins/src/` only
  if an apply signature must change.
- `crates/host-core/src/prepare.rs` (producer plumbing only; stream A's file, sequenced).
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs` (input band; stream H's files).

## Non-goals

- The C ABI input edits themselves (#1261, #1262). Effect and route lanes (#1345, #1347).
- Default ramp lengths (#1054).

## Objective gates

1. **Many edits, one block (new host-web test).** 30 `Both` trim edits on one strip, then 30
   `Both` HPF edits (section 0), each admitted; one render equals a twin that sent only the last
   of each; the counter grows by 116 in #1312 D2's per-cell unit (each `Both` edit writes two
   cells, so each kind adds `2 × 29 = 58`).
2. **Order (new builtins-compiler test).** One block with trim, polarity and both filter sections
   dirty renders as a twin that applies them as records in the order trim, polarity, filter.
3. **Witness (new builtins-compiler tests).** A `Both` trim on a mono stem keeps the bank
   collapsed; left and right trims written separately to equal values in one block keep it
   collapsed; unequal values clear `LIVE`. Mutation (PR evidence): admit a fixed `Desymmetrize`
   for every drained cell; the first two turn red.
4. **Carry (keep green):** the #1276 carry tests in `crates/host-core/tests/successor_swap.rs`
   pass with dirty input cells at the swap block.
5. **Superseded tests.** Browser tests that fill the input queue and expect
   `COMMAND_REASON_BACKPRESSURE` (find them beside `the_input_queue_is_a_destination_of_its_own`,
   `hosts/host-web/src/tests.rs:8359`) are deleted or rewritten on observation records; gate 1
   replaces them.
6. **Realtime and workspace.** `cargo build --locked --release -p audit -p capi &&
   target/release/audit capi` (all violation counts 0); `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-web-audioworklet.sh`; `cargo test --locked -p builtins-compiler --features
   test-support`; `cargo test --locked -p builtins --features test-support,lane/test-support`;
   `cargo test --locked -p host-web --features test-support`; `cargo test --locked -p host-core
   --features control-provider,test-support`; `cargo fmt --all -- --check`; `cargo clippy --locked
   --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: the input lane left a queue, or miscounted supersession.
- Gate 2: a drain order other than trim, polarity, filter, which moves bits when a polarity flip
  and a trim ride meet in one block.
- Gate 3: a witness folded per write instead of from final contents, retiring a mono collapse on
  a both-channel edit.

## Amendment 1 (root, 2026-10-05): revision-bounded cells

Root's binding requirement (2026-10-05) makes every live value of one revision take effect in one
block (#1432 Amendment 1, #1312 Amendment 2). D2 reads under the block's snapshot, and D5's carry
drain reads the quiescent predecessor with `LiveSnapshot::ALL`. D3's witness folds from what this
block applied.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312).
- **Deferred into this issue by #1504** (recorded in #1504's "Batch follow-ups", n3, and #1312's open
  item 2). `BuiltinStage` (`crates/graph/src/runtime.rs`) reports `live_lanes() == 0`, so #1504's
  slot-guard assertion in `BankChain::new` (`crates/rack/src/lib.rs`) is vacuous for builtin banks,
  the fader and matrix cell lanes included. This issue moves the last builtin lane (input) to cells,
  so it adds `live_lanes` to `GraphPreparedBuiltinBankProcessor` and its four implementations in
  `crates/builtins-compiler` (fader, matrix, input and the fused pair) and forwards it from
  `BuiltinStage`. It is safe until then because every builtin slot is always active.
- Followed by *Apply value-only input trim and polarity edits to the running C ABI plan* (#1261)
  and *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
  (#1262), which write to these cells.

## Attempt record

### Attempt 1 (implementer, on `codex/d15-stream-b3`, commits `62ad55156`, `864dbf13a` and the evidence commit that carries this record)

**What landed.**

- `builtins-compiler`: `InputCells` (the writer: eight cells, trim left and right, polarity left and
  right, then the filter cells of section 0 and section 1, left before right; cell index = dirty bit
  = canonical drain order) and `InputReader` replace `Producer`/`Consumer<TrackInputRecord>`.
  `InputCells::write(stamp, record)` is infallible on a value; a filter target naming a section other
  than 0 or 1 panics, and every caller has run `validate_prepared_input_filter_target` before its
  first write. `InputReader::drain(snapshot, witness, apply)` reads each pending cell under the
  block's snapshot, pairs the two channel cells of a kind into one `Both` record when both were
  applied with equal words (D2), admits each record through `ChannelSymmetryWitness::admit` before it
  is applied (D3: the witness folds from final contents), applies trim, polarity, then filter
  section 0, then section 1 (D2), applies every value read before it returns the first refusal, and
  adds the replaced count to the session counter once. `BuiltinBankProcessor::begin_block` and the
  test-only `LiveControlInputProcessor::process` drain through it; `drain_for_carry` reads with
  `LiveSnapshot::ALL` and counts a refused apply (D5).
- `TrackControlRequest::queue_capacity` is gone: it sized the input ring only, and no lane has a
  depth now. The seal's control rows lose the capacity (`(track, input_lane)`), and the resource plan
  charges one `Arc<InputLane>` allocation per request with `input_lane` in place of the ring rows.
- `GraphPreparedBuiltinBankProcessor::live_lanes` (default `0`), implemented by the input, fader,
  matrix and fused-pair banks (bit `n` is lane `n` holding a reader), forwarded by `BuiltinStage`
  (the Dependencies deferral from #1504).
- `host-core::prepare`: the control requests lose the depth; docs follow.
- `host-web`: `write_strip_cells` also writes the batch's `Input` records and the prepared builtin
  filter targets (a prepared owner marker's companion targets) under the batch's one stamp; the
  input band takes no room and no `in_flight` credit; the room pass only checks that the strip holds
  its input cells (`UNSUPPORTED_KIND` otherwise, as before). The slot layout is unchanged (the band
  stays an unaddressable queue slot range, as the matrix and fader bands did after #1312). The
  filter shadows stay the control-side authority for the designed pair (D6). A `#[cfg(test)]` field
  `ReadyOwnership::live_values_superseded` holds the session count for the tests; the status field
  that reports it is #1399's, which removes the `cfg`.

**Choices the spec left open, and files outside the authorized list (each forced).**

1. `crates/graph/src/{lib,runtime}.rs`: the `live_lanes` method and its forward, per the
   Dependencies section (the STREAMS row for `crates/graph` lists #1504's drain signatures only).
2. `crates/graph-compiler/src/lib.rs` (four test literals) and
   `crates/builtins-compiler/tests/allocation_tracker.rs`: the removed `queue_capacity` field.
3. `crates/control-plane/src/compile.rs`: one doc comment that still said each strip's input queue
   holds 16 records.
4. `crates/host-core/tests/{collapse_arming,input_liveness_live_controls,randomized,successor_swap}.rs`:
   they wrote input records with `try_push`; they now write cells through the plan's own gate
   handle.
5. Re-pins (each individually, reason in the file): `tools/audit/src/fixture_builtins.rs`
   `STRIP_PREPARATION_BYTES` 1088 to 1136 (+48 = `InputReader` 88 bytes in place of the 40-byte
   ring consumer, measured with `size_of`); `fixtures/builtins/v1/resources.jsonl` (+48 bytes per
   track in every processor payload count and in the largest allocation; allocation counts do not
   move; regenerated with `audit fixture-builtins --write`, only that file and `MANIFEST.tsv`
   differ); the joined manifest digest in `fixture_builtins.rs` and `ACCEPTED_MANIFEST_SHA256` in
   `builtins_graph.rs` (`0834666e` to `28f109c9`, the measured `sha256sum` of the manifest);
   `fixtures/graph/v1/direct-route.resources.json` `builtin_bank_bytes` 6663 to 7047 (+384 = 8 input
   lanes x 48; regenerated with `graph_fixture --write`, no other graph fixture moved).
6. Realtime-policy floors: the tree held 100 marked regions in 27 files against a floor of 96; this
   slice adds two regions (the bank's input drain, and the paired-channel and cell-read helpers
   with `InputReader::drain`)
   and raises the awk gate's floor to 102 regions, 27 files, and the self-test's pad with it (one pad
   file of 20 regions, thirteen of 3, on top of the 13 files and 43 regions).
7. `InputReader::drain` pairing makes equal words written to the two channels in one block a `Both`
   record, as D2 says, so `a_symmetric_ride_renders_the_same_bits_collapsed_or_not` can no longer
   split the ride into two per-lane records to decline the collapse (two equal records are now one
   `Both` record by design). Its dual arm declines the collapse with a left-only polarity write to
   the polarity the lane already has, which changes no arithmetic; the bits comparison is unchanged.
8. Gate 1's HPF edits are HPF-only edits (selector 3), which touch section 0 alone; a whole-pair
   edit touches both sections and so writes four cells per edit. With selector 3 each kind adds
   `2 x 29` and the total is 116, as the gate says.
9. Filter section order (section 0 before 1) is a determinism convention: the two sections hold
   independent state, so no bit depends on it and no test can tell it. A mutation that drains section
   1 first stays green and is recorded as such below.

**Tests moved or deleted.** Deleted: `a_block_drains_every_input_record_queued_at_its_entry`
(builtins-compiler `input_drain.rs`; it filled the 16-record ring), replaced by
`a_block_applies_the_last_of_many_input_edits` (banked and scalar arms, 40-edit walks, one
transaction per edit); `the_input_queue_is_a_destination_of_its_own` (host-web; two queues that
refuse each other's records), replaced by gate 1; the queue-room half of
`a_both_lane_trim_command_is_one_record_and_one_queue_slot` (now
`a_both_lane_trim_command_addresses_both_lanes`: the left-only/right-only/both bits half stays);
`the_carry_drain_applies_every_record_past_a_refused_one` (now
`the_input_drains_apply_every_value_past_a_refused_one`, which also holds the block drain).
Rewritten on a queue that is still one (an effect bypass queue): `idle_render_skips_admission_counter_clear_...`
(plus a line that an input trim takes no credit), `a_refused_solo_submission_leaves_the_live_controls_untouched`
(new `solo_host_with_insert`), `prepared_eq_owner_refusals_preserve_config_revision_indexes_and_queues`
(track 1's console EQ queue is the unrelated full queue),
`prepared_builtin_input_filter_owner_is_atomic_across_mixed_batches` (the full queue is the effect
queue; the refused batch writes no input cell and no fader cell). Adapted for the new types:
`builtin_bank_resource_charges_two_planes_and_actual_members` (the hand formula names
`Option<InputReader>`), the allocation tracker's input test and rings test (renamed `..cell..`),
`the_per_node_live_control_input_processor_drains_and_folds_its_witness`.

**New tests (the plausible defect each defends).**

| Test | Defect it turns red |
|---|---|
| builtins-compiler `input_cells_drain_in_the_canonical_order_whatever_the_write_order` (gate 2) | polarity drained before trim: the two windows differ, so the last call's window wins and the bits move |
| `the_input_witness_folds_from_the_final_cell_contents` (gate 3, five cases) | a witness folded per write or fixed: a `Both` write, or equal words written per channel, retiring the mono collapse; unequal words, a differing window or a lone channel keeping it |
| host-core `equal_words_written_per_channel_in_one_block_keep_the_collapse` (gate 3, banked plan) | the same, as the collapse census of a real mono plan and its bits |
| `many_input_edits_before_one_block_apply_the_last_and_count_each_replaced_value` | replaced writes uncounted or counted on one channel; the oldest value applied |
| `the_input_drain_reads_under_the_blocks_snapshot` | `begin_block` reading `ALL`: a value of a later revision applied early |
| `the_input_drains_apply_every_value_past_a_refused_one` | a drain that stops at the first refusal (carry and block drains) |
| `a_block_applies_the_last_of_many_input_edits` (integration, banked and scalar) | an input drain skipped or reading an old walk value |
| host-web `many_input_edits_before_one_render_apply_the_last_and_are_counted` (gate 1) | the 17th edit refused (a ring back in the band); counter 58 instead of 116; input records not written |
| graph `a_builtin_stage_forwards_its_banks_live_lanes` | `BuiltinStage` dropping `live_lanes`, which makes #1504's slot guard vacuous for builtin banks again |

**Gates (final tree unless noted).**

| Gate | Result |
|---|---|
| `cargo test --locked --workspace --no-fail-fast` | 344 test binaries, none failed (the first run's only failures were the audit resource grid and `graph_fixture`: item 5) |
| `cargo test --locked -p builtins --features test-support,lane/test-support` | 155 passed |
| `cargo test --locked -p builtins-compiler --features test-support` | 87 passed |
| `cargo test --locked -p host-web --features test-support` | 196 passed |
| `cargo test --locked -p host-core --features control-provider,test-support --no-fail-fast` | 299 passed, the #1276 carry tests (`successor_swap`) included (gate 4) |
| `cargo build --locked --release -p audit -p capi && target/release/audit capi` | allocations, deallocations, locks, syscalls 0; `total_violations` 0 |
| `check-realtime-policy.sh`, `test-realtime-policy.sh` | ok (102 marked regions in 27 files) |
| `check-rack-policy.sh`, `check-workspace-policy.sh`, `check-builtins-policy.sh`, `check-graph-policy.sh`, `check-host-core-policy.sh` | ok |
| `check-builtins-fixtures.sh . target/release/audit`, `check-console-fixtures.sh`, `check-graph-determinism.sh`, `cargo test --release -p audit --bin audit` | ok (50 files; 33 passed) |
| `trace-builtins-graph-audit.sh`, `trace-graph-audit.sh` | PASS |
| `check-cross-targets.sh` | PASS |
| `cargo fmt --all -- --check`; clippy `--workspace --all-targets --all-features -D warnings` | ok |
| Worklet chain: `build-web-audioworklet.sh --named-twin` (shipped module `ffec707c...`), `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts` (graphSessionPlusPlanBytes 30098 of 35648), `check-scalar-oracle-absent.py --wasm`, `check-web-audioworklet-v8-spill.py`, `test-web-audioworklet.sh` | pass |

**Mutation evidence.** Each applies one defect to a committed file, runs the named tests, reverts.

| Defect | Red |
|---|---|
| witness admitted as a fixed `Desymmetrize` per drained value | `the_input_witness_folds_from_the_final_cell_contents` (cases 1 and 2), `the_per_node_live_control_input_processor_...`; host-core `equal_words_written_per_channel_...`, `a_both_lane_record_drained_while_collapsed_...`, `a_symmetric_ride_renders_...`, `the_collapse_census_follows_the_commands` |
| witness admitted as a fixed `Preserve` | the same two builtins-compiler tests; host-core `a_symmetric_ride_renders_...`, `the_collapse_census_follows_the_commands` |
| equal channel words not paired into one `Both` record | `the_input_witness_folds_...`; the four host-core tests above |
| polarity drained before trim | `input_cells_drain_in_the_canonical_order_...` |
| filter section 1 drained before section 0 | none: the sections' state is independent (item 9) |
| `begin_block` reads `LiveSnapshot::ALL` | `the_input_drain_reads_under_the_blocks_snapshot` |
| `begin_block` does not drain the input cells | `a_block_applies_the_last_of_many_input_edits` |
| the replaced count not added / added for the left trim cell only | `many_input_edits_before_one_block_...`; host-web `many_input_edits_before_one_render_...` (the first) |
| the drain stops applying after the first refusal | `the_input_drains_apply_every_value_past_a_refused_one` |
| the carry drain applies nothing | `successor_swap`: `a_pending_trim_record_survives_the_swap_at_eight_lanes` and at four lanes, `a_one_channel_record_keeps_its_chain_dual_after_the_swap` |
| `BuiltinStage` does not forward `live_lanes` | `a_builtin_stage_forwards_its_banks_live_lanes` |
| browser cells written before the room pass | `prepared_builtin_input_filter_owner_is_atomic_across_mixed_batches`, `a_refused_browser_batch_writes_no_cell_and_leaves_the_gate_alone`, `a_refused_solo_submission_leaves_the_live_controls_untouched`, `a_refused_send_batch_pushes_nothing_and_keeps_the_mirror` |
| browser admission skips the input records' cell write | `a_trim_command_reaches_the_render_plane`, `many_input_edits_before_one_render_...` |

**Open items.** (a) The browser input band's queue slots stay as unaddressable slot numbers beside
the matrix and fader bands; a renumbering of the queue-slot layout is a separate change. (b) The
C ABI's input edits (#1261, #1262) write `TrackControlProducer::input` with `InputCells::write`
behind the stamp, as #1312's fader edits do. (c) Filter section order is unobservable (item 9).

### Follow-ups (verdict of attempt 1: m1, m2, m3, n1 to n5; m5 is root's)

**m1. The four browser backpressure tests now fill an insert queue with observation records.**
Gate 5 said observation records, and effect bypass becomes a cell in #1345 (D1), so a rewrite on
bypass records would be rewritten again. `COMMAND_OBSERVE_SUBSCRIBE` records stay a FIFO (#1345 D2),
so these four tests keep their queue. The parametric EQ declares no tap, so the hosts they use now
carry the observation fixture's compressor as the insert (`observed_compressor_insert`, tap 1) with
`live_control_observation_taps: 4`; `stage_effect_bypass` is deleted and `stage_observe_subscribe`
replaces it. #1345 D2 sizes the observation queue by tap count, not by `live_control_command_queue_records`,
so #1345 must re-size these four fills (depth and the `in_flight` numbers), but no longer has to change the
record kind.

| Test (hosts/host-web/src/tests.rs) | Defect it turns red | Mutation (applied to `hosts/host-web/src/lib.rs`, then reverted) |
|---|---|---|
| `idle_render_skips_admission_counter_clear_without_losing_queue_credit` | an insert-queue record that takes no `in_flight` credit, or an idle block that clears the dense counters | drop `ready.in_flight[slot] += 1` for a queue record: red; `if ready.has_in_flight_commands` made `if true`: red |
| `prepared_builtin_input_filter_owner_is_atomic_across_mixed_batches` | the browser writing a cell (the prepared input targets, the fader value) before the room pass that refuses the batch | `write_strip_cells(..)` called before the room pass: red (16 tests in all); room check off by one (`needed > available + 1`): red |
| `prepared_eq_owner_refusals_preserve_config_revision_indexes_and_queues` | a batch refused for an unrelated full queue (track 1's compressor insert) that still admits or publishes the EQ owner's targets | room check off by one: red |
| `a_refused_solo_submission_leaves_the_live_controls_untouched` | a refused batch that leaves the solo transaction applied | delete `ready.solo.rollback()`: red (with 4 other tests); room check off by one: red |

**m2. A test pins the four `live_lanes` implementations.** `every_builtin_bank_reports_exactly_the_lanes_that_hold_a_reader`
(builtins-compiler) builds the input, fader and matrix banks with readers on lanes 0 and 2 and the
fused pair with fader readers on lanes 0 and 2 and matrix readers on lanes 1 and 2, and asserts
`0b101`, `0b101`, `0b101` and the union `0b111`; banks without a reader report 0. Defect it turns
red that nothing else does: a builtin bank reporting 0 for a lane it drains, which makes
`BankChain::new`'s slot guard (`live_lanes() == 0 || ..`) vacuous for that bank. Mutations, one
each, all red at the named assertion and green when reverted: the input bank returns 0; the fader
bank returns 0; the matrix bank returns 0; the fused pair returns 0; the fused pair reports only
its fader stage (`5` instead of `7`); the input bank reports `0xff` (255 instead of 5).

**m3. `equal_words_written_per_channel_in_one_block_keep_the_collapse` is deleted.** I looked for a
defect only it catches. The one candidate was pairing any two applied channel words into a `Both`
record, whatever their values (`paired_channels` without `left == right`): it turns that test red
and no other host-core test, but it also turns `the_input_witness_folds_from_the_final_cell_contents`
red at crate level (builtins-compiler), so the defect is caught. Every other defect I tried (a fixed
`Desymmetrize`, a fixed `Preserve`, never pairing) is already caught by five or more host-core tests
and the builtins-compiler gate-3 test. The end-to-end collapse dispatch is held by
`a_both_lane_record_drained_while_collapsed_keeps_the_collapse` and
`a_symmetric_ride_renders_the_same_bits_collapsed_or_not`.

**Nits.**

- n1: the dead `let prepared = ..; let _ = prepared;` is removed from
  `many_input_edits_before_one_block_apply_the_last_and_count_each_replaced_value`.
- n2: the doc of `the_input_drains_apply_every_value_past_a_refused_one` names the mutation that turns it red
  (the reader's `drain` applies only while `first_error` is `Ok`: red in the block arm; the
  carry applies nothing: red in the carry arm) and says that the carry arm's count-and-continue
  replaced by a return stays green (re-run: green), since `drain_for_carry` reports a returned
  error as one refusal.
- n3: the `command_wanted` accumulation and its `u32` overflow `BACKPRESSURE` refusal now run only
  for an EQ owner marker; a builtin input owner's targets are cells and want no room.
- n4: `TrackControlProducer` is named in the `Arc` prose in place of "the producer".
- n5: `BuiltinBankProcessor::drain_controls` has a `debug_assert!(lane < MAXIMUM_BANK_LANES, ..)`
  before the `live.get_mut(lane)` lookup. The `get_mut` stays in the render path (a slice index puts a
  panic edge into the worklet call graph, which the shipped-module gate refuses); a debug build
  now refuses a lane that holds a reader and has no symmetry term, instead of skipping its drain.

**Follow-up gates (this tree).** `cargo fmt --all -- --check` and clippy `--workspace --all-targets
--all-features -D warnings`: ok. builtins-compiler `--features test-support` 66 passed (the new
`live_lanes` test included); host-core `--features control-provider,test-support` all binaries
green (`input_liveness_live_controls` 11 passed, one test fewer); host-web `--features test-support`
lib 191 passed, 2 ignored. `check-workspace-policy.sh`, `check-host-core-policy.sh`,
`check-builtins-policy.sh`, `check-graph-policy.sh`, `check-rack-policy.sh`,
`check-realtime-policy.sh` (102 regions in 27 files) and `test-realtime-policy.sh`: ok.
`check-cross-targets.sh`: PASS. Worklet chain on this tree: `build-web-audioworklet.sh
--named-twin` (shipped module `13e78585e4a5...`; it moved from `ffec707c...` because `host-web`'s admission
code changed, n3), `check-web-audioworklet.sh --without-metadata-regeneration`,
`check-browser-expected-resources.py --artifacts` (sourceTotalBytes 3358 of 3648),
`check-scalar-oracle-absent.py --wasm`, `check-web-audioworklet-v8-spill.py` and
`test-web-audioworklet.sh` (private TMPDIR left empty): pass. Not run: the Playwright `browser` legs
(no browsers on this box). Open: #1345 re-sizes the four observation fills (m1); STREAMS hot-file rows
(m5) are root's.

### Follow-up after attempt 1: the SDK eval of the removed input queue

**`sdk/test/writer-evals.mjs`: "a paired builtin filter edit stays whole when only one queue slot
remains" is deleted.** `scripts/check-sdk-headless.sh` has failed on it since #1346: it booted with
`commandQueueRecords: 2`, seeded one `hpfHz` edit "into one of two queue slots", and expected the
next `inputFilters` pair (a `both`-channel record) to be refused `backpressure` with
`admitted == 0` and `pending == 1`, then admitted after one render. The input filter band is cells
and no queue, so the second submit is admitted at once, which is the cell contract; the eval's
refusal and its "queue drain" no longer exist. The defect it defended, a pair acknowledged as one
half, cannot happen in a cell: both channel cells are written under the batch's one stamp after
every check, and a render applies the pair whole or not at all. That is held by host-web
`prepared_builtin_input_filter_owner_is_atomic_across_mixed_batches` (a refused batch writes
neither cell; it is also the only test that turns red when a write lands only the first
prepared-filter target of a pair). I looked for a defect only an SDK-level rewrite would catch (the SDK lowering a `both` filter record to one channel's cell) and
found none: the eval observed acknowledgements, not cells, and a rewrite on the cell path would
need a render and a bit comparison, which the host-web twin tests already make. The other SDK
evals that set `commandQueueRecords` (the bounded-queue writer evals run on EQ parameter records,
an effect lane) are unaffected. `compressorDocument` stays: the raw queue-cost probe still uses it.

### Follow-ups 2 (Sonnet 5.5, on `codex/d15-stream-b3`; #1347 attempt 2 verdict, section on `51d6ba7a9`)

**NIT: the note above named a test that does not hold the pair whole.** It named builtins-compiler
`the_input_witness_folds_from_the_final_cell_contents` as keeping a filter pair whole. That test
defends the `Both` fold (`paired_channels` collapsing equal words into one record); the verdict
measured it green when half the pair is dropped. The note now names only host-web
`prepared_builtin_input_filter_owner_is_atomic_across_mixed_batches` for the writer side: it is the
only test that turns red when the host writes just the first prepared-filter target of a pair (the
verdict's MP1), and it also turns red when cells are written before the room pass. The render side
(a render that never reads the right channel's filter cells, the verdict's MP2) is caught by six
other tests: builtins-compiler `input_cells_drain_in_the_canonical_order_whatever_the_write_order`,
`many_input_edits_before_one_block_...` and `the_per_node_live_control_input_processor_drains_and_folds_its_witness`;
host-web `many_input_edits_before_one_render_...` and the two `a_collapsed_chain_renders_the_forced_dual_bits_*` tests.
The m3 paragraph above is unchanged: it concerns `paired_channels` without `left == right`, which
the witness test does catch.

**STREAMS hot-file rows (root ratified).** `docs/handoffs/decision-15-2026-10-05/STREAMS.md` now
names #1346 in the rows for `crates/graph/src/{lib,runtime}.rs` (the `live_lanes` forward),
`crates/graph-compiler/src/*` (`lib.rs` test literals for the removed `queue_capacity`) and
`crates/host-core/src/prepare.rs`.

### Batch-verdict fixes (stream B batch 3; Sonnet 5.5, on `codex/d15-stream-b3`)

Verdict: `submix-verdicts/stream-b3-batch-verdict.md`, on `7c5b8099d`.

**M1: the intermediate commits that do not compile (root, 2026-10-12; process finding, no history
rewrite).** Commits `65c51ab41` (#1346 checkpoint 1) through `c0f825939` (the #1345 attempt-2 record),
25 commits, do not compile `crates/host-core/tests/successor_swap.rs` under
`cargo check --workspace --all-targets --all-features`: stream A2's `CellPlan` helper builds a
`LivePlan` literal, and `65c51ab41` adds `LivePlan::gate` (E0063 at the two literals). The rebase onto
A2 caused the gap. `cffc3006f` ("Reconcile batch 3 with stream A2") fixes it, and every commit after it
and every commit before `65c51ab41` compiles. At every commit every library and binary compiles.
`71b4ecdb8` (#1345 checkpoint 1) additionally fails nine test units, as its message says ("the
workspace tests are not green until then"). To bisect on this branch, use `git bisect --first-parent`
on the merged history, or skip `65c51ab41..c0f825939`. The batch's SHAs are cited by
specs and verdicts, so the branch is not rewritten.

**MINOR 1: `live_values_superseded` must not depend on bank packing (#1312 D2, the header's "a value
on a lane that a swap restarts is not counted").** The swap carry drained whole predecessor banks
(`drain_for_carry` over every lane), so a strip that restarts but shares a predecessor bank with a
strip that carries had its replaced writes counted, and bank membership depends on lane width. Now:
- `GraphPreparedBuiltinBankProcessor::drain_for_carry(lanes)` (input cells) and the new
  `drain_strip_for_carry(owner, lanes)` (fader, mute and matrix cells; a fused bank drains only
  `owner`'s half) read only the lanes in the `lanes` mask. `crates/graph/src/runtime.rs`'s
  `carry_input_lanes` and `carry_strip_lanes` build the mask from the moves of the bank, which are
  adjacent because the program is sorted by predecessor location. The block drain passes every lane.
  Cells are still read with `CellReader::read_counted`; render stays allocation-, lock- and
  syscall-free.
- Tests (`crates/host-core/tests/successor_swap.rs`, each at four and at eight lanes, the strip one in
  the split and the fused form):
  `a_restarting_lane_does_not_count_its_replaced_writes_at_{four,eight}_lanes` (B adds a VCA over
  `eq0`, so its fader stage restarts; three fader writes sit unread in `eq0`'s cells, and in
  `eq1`'s, which carries from the same bank, three fader and three pan writes) and
  `a_restarting_input_lane_does_not_count_its_replaced_writes_at_{four,eight}_lanes` (the committed
  model differs from B's in `eq0`'s trim, so its input section restarts; three trim writes sit unread
  in each of `eq0`'s and `eq1`'s cells). Each asserts the predecessor's count after the hand-over:
  0 with only the restarting strip written, the carried strip's own count (6 for the strip case, 4
  for the input case) alone and with the restarting strip written too.
  *Test value:* turns red if the swap counts a restarting lane's replaced writes (the count then
  changes with which lanes share a bank, so with the lane width); no other test reads the count
  across a swap.
  *Mutations (apply: red; revert: green):* `carried_input_lane_mask` returning every lane turns red the
  two input tests (left 4, right 0) and leaves the strip tests green; `carried_strip_lane_mask`
  returning every lane turns red the two strip tests (left 4, right 0) and leaves the input tests
  green.
- `drain_for_carry`, the builtins-compiler drain tests and `apply_pending_equals_the_block_drain` follow
  the new signatures.

**Gates on the final tree** (x86_64 Linux, toolchain 1.97.1, Node 22.23.2; each job with its own target
directory, as CI runs them). Every step of: `fmt`; `clippy --workspace --all-targets --all-features -D
warnings`; `cargo doc`; the workspace, session, env, bench, test-support, scalar-oracle, script-reach,
host-core, protocol-control, realtime (118 marked regions in 27 files, with its audit-leak and
evidence-leak checks), lane, unfused-seal, rack, builtins, graph and effect-runtime policies and
their self-tests; `test-debug-a` (133 suites, 1,616 passed, 0 failed, 20 ignored; doctests);
`test-debug-b` (163 suites, 916 passed, 26 ignored); the `test-release` steps including loom
(13 passed); `audit-native` (all 20 steps: `audit capi` 0 allocations, 0 syscalls; `check-capi-abi.sh`
and its self-test); `wasm-guests`; `check-cross-targets.sh`; `check-release-shape.py` and the unwind
check; the worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh` with
`check-abi-layout-v1.py`, `check-browser-expected-resources.py --artifacts`, scalar-oracle-absent,
`test-web-audioworklet.sh`, the V8 spill gate); the `sdk` job's checks (`check-sdk-generated`,
`check-sdk-deletions`, `check-sdk-types`, `check-sdk-headless`, `sdk-package.sh check`). All pass.
The browser legs, `npm run qualify -- --check-matrix --self-test-mutations` in SDK source-bundle mode
with `sdk/dist` deleted first: chromium 151.0.7922.34, firefox 153.0 and webkit 26.5 each print
"all qualification gates passed". The unsubscribe gate was also run 3 more times unloaded on all
three browsers, 14 times on webkit and 4 times on chromium and firefox with 24 busy loops on 32
cores: no failure after the `resumeUntil` retry. AArch64 and the artifact-identity report run only
in CI.


### Batch re-verify fixes (stream B batch 3; batch verdict 2; Sonnet 5.5, on `codex/d15-stream-b3`)

B2 (the WebKit unsubscribe gate) belongs to #1345 Amendment 5: see its "Batch re-verify fixes" record
for the change, the run table and the open WebKit failure under full load.

**MINOR 1:** `a_pending_trim_record_in_a_later_bank_survives_the_swap_at_{four,eight}_lanes`
(`crates/host-core/tests/successor_swap.rs`; `eq5` is lane 1 of the second predecessor bank at four
lanes, `eq8` lane 0 of it at eight; the strip at the same lane of the first bank, `eq1` or `eq0`,
is muted in every session and restarts at the swap: the committed model moves its trim and B keeps
the old one). *Test value:* red if the input drain's lane mask is built from the first bank's moves
instead of the drained bank's, which loses a later bank's pending trim write at the swap when the
first bank's mask lacks that lane and which the first-bank tests (`eq2`) cannot see. Without the
restarting strip every lane carries, bank 0's mask is full and the defect hides (batch verdict 3,
MINOR 1: the earlier form of these tests stayed green under it). *Mutations (apply: red; revert:
green; `runtime.rs` restored byte-for-byte, `cmp` clean):* (a) `moves.get(first)` -> `moves.get(0)`
in `carried_input_lane_mask`: the two tests red (70 other tests green); (b) `moves.get(first)` ->
`moves.get(0)` and `.skip(first)` -> `.skip(0)`, so every bank uses bank 0's mask: the two tests
red (70 green), and green (72 passed) with the tests as they were before this fix; reverted: 72
passed.

**NITs.** `lane_bit` (`crates/graph/src/runtime.rs`) carries a `debug_assert!` for a lane beyond the
mask width and keeps `checked_shl(..).unwrap_or(0)` in release, so the Wasm call graph gains no
panic edge. The fused-form doc of the strip test is corrected. `hosts/host-web/MUTATIONS.md` row 20
names the tests that now hold the deleted flood test's claim.
**Known gaps (recorded, not closed):** (1) the owner filter in `carried_strip_lane_mask` is defence
in depth: host-core always carries both owners of a fused bank or neither
(`host-core/src/prepare.rs`), so no test reaches it and removing it leaves every test green. (2) No
SDK eval sends fader edits through `LiveControlWriter` to a real engine; `faderEdit` is used only in
an engine-free coalescing test (`sdk/test/writer-evals.mjs`) since the deleted "playing" eval.
