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
- **C ABI.** No input lane today (`crates/capi/src/runtime/compile.rs:15-21`); #1261 attaches one
  and #1262 adds filter targets, both written to these cells.

## Decisions frozen for this slice

- **D1. Cells per strip**, on #1312's primitive: trim left and right (value bits, ramp), polarity
  left and right (value, ramp), and one filter cell per `(section, channel)` (section 0 and 1;
  the pair and the six coefficients, eight words). A `Both` record writes both channel cells. One
  dirty word per strip.
- **D2. Canonical drain order:** trim, then polarity, then filter targets (section 0 before 1),
  left before right. When both channel cells of a kind are dirty with equal words, render applies
  one `Both` call.
- **D3. The witness folds from final contents.** For each kind drained, equal left and right
  words are admitted as `SymmetryEvent::Preserve`, any other pair or a lone channel as
  `Desymmetrize`, through the same `admit` hook, so a record kind still cannot reach render state
  without declaring its witness effect.
- **D4. A filter target is a level.** A later target for the same lane and section replaces an
  undrained one; the ramp starts from the coefficients render holds (#1262 D2).
- **D5. The carry drain** applies every dirty cell of the predecessor before the input lanes
  carry, as it drains the queue today; a refused apply is counted as now.
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

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312).
- Followed by *Apply value-only input trim and polarity edits to the running C ABI plan* (#1261)
  and *Apply value-only input HPF and LPF edits to the running C ABI plan through prepared targets*
  (#1262), which write to these cells.
