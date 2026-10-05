# Hold live values in latest-target cells on both hosts

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2, D15-17).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A strip's live fader, mute and pan or matrix values are held in latest-target cells instead of
bounded FIFO queues, on the C ABI and in the browser through the same shared code. A host can
send any number of fader, mute or pan edits between two render calls, a paused host included,
and none is refused for room. Render applies the last committed value of each cell at the next
block, and an exact `live_values_superseded` counter reports how many committed values a later
one replaced before render read them. The ack bytes do not change.

This is the first slice of D15-2: the strip fader/mute and matrix/pan lanes. The remaining
cell slices are *Hold effect parameter, bypass and EQ-target values in latest-target cells*
(#1345), *Hold strip input-lane values in latest-target cells* (#1346) and *Hold route-lane
values in latest-target cells* (#1347); they reuse this slice's primitive and counter.

## Context

- **Records and rings.** `TrackControlRecord` (matrix, `crates/builtins-compiler/src/lib.rs:105`)
  and `TrackFaderRecord` (fader or mute with a `BuiltinLaneSelector`, `:129`) ride bounded SPSC
  rings created per strip (`:3583-3587`), charged at `:3777-3790`, and held by
  `TrackControlProducer::{producer, fader}` (`:254-270`).
- **Render drains.** `drain_fader_controls` (`:1064`) and `drain_matrix_controls` (`:1108`) pop
  every record available at block entry and apply each in order (`set_fader_db`, `set_mute`,
  `set_target_smoothed`, `crates/builtins/src/lib.rs:3869`, `:3891`, `:4056`). The test-only
  scalar processors do the same (`LiveControlMatrixProcessor` `:4316`, `LiveControlFaderProcessor`
  `:4377`). Both stages are seam-side, so no channel-symmetry witness reads them (`:758-771`,
  `:816-823`).
- **No record carries a sample time.** Every record popped at one block entry lasts zero samples
  except the last per address, so a FIFO and a latest-value cell differ only when two records for
  one address meet in one drain: FIFO snaps to the earlier value, then ramps from it.
- **C ABI admission.** `commit_live` checks room (`crates/capi/src/runtime/control.rs:1189-1198`,
  `LiveBackpressure`, `control.live.backpressure`) and pushes after every fallible check
  (`:1288-1303`); the live lane depth is 16 (`crates/capi/src/runtime/compile.rs:12-13`). Push
  order is `FaderDb` then `Mute`, left before right (`crates/host-core/src/live_delta.rs:62-77`).
  The header promises 16 pending values and the backpressure string
  (`crates/capi/include/miso_engine_v1.h:50-55`).
- **Browser admission.** The matrix and fader bands are queues with a free-room pass and
  `in_flight` accounting (`hosts/host-web/src/lib.rs:1548-1559`, `queue_available` `:1762-1772`,
  `push` `:1800-1815`, room check `:5568-5580`); solo and VCA mutes compose into fader records.
- **Counters.** `CounterId` ends at `ValidationFailures = 15`
  (`crates/protocol/src/message_wire.rs:626-642`). The browser status has four required-zero
  expansion words (`WebStatus::reserved`, `hosts/host-web/src/lib.rs:1300-1301`).

## Decisions frozen for this slice

- **D1. The cell primitive** lives in `crates/engine/src/realtime/` as a new module. A cell is a
  fixed number of `AtomicU32` words (the target and its ramp, one unit) plus one `AtomicU32`
  sequence number. The control thread is its only writer: odd sequence, release fence, word
  stores, release store of the next even sequence, then `fetch_or` of the cell's bit into its
  stage's dirty word. Render reads: acquire load of the sequence, word loads, acquire fence,
  relaxed reload. An odd or changed sequence is a torn read: render sets the bit again and skips
  the cell this block; it never spins.
- **D2. Exactly once, counted exactly.** Render keeps, per cell, the last sequence it applied.
  It skips a cell whose sequence equals it (a value already applied). When it applies sequence `s`
  after `p`, `(s - p) / 2 - 1` committed values were replaced unread: it adds that to
  `live_values_superseded`. The counter is one `Arc<AtomicU64>` per session, passed to every plan
  the session prepares; render is its only writer (load, saturating add, store) and the control
  thread reads it.
- **D3. Cells per strip.** Fader stage: four cells (fader left, fader right, mute left, mute
  right), each two words (value bits, ramp), and one dirty word. Matrix stage: one cell of five
  words (four coefficients, ramp) and one dirty word. A `Both` write writes both channel cells.
- **D4. Canonical drain order.** Fader before mute, left before right. When both channel cells of
  a kind are dirty with equal words, render applies one `Both` call, as today's single record does.
- **D5. Producers.** `TrackControlProducer::{producer, fader}` become cell writers with
  infallible writes; the input lane stays a queue. The rings at `:3583-3587` for these two lanes
  and their resource rows are replaced by the cells' rows, computed from the types.
- **D6. Validate before any write (D15-2 condition 3).** `commit_live`'s order stays: every
  fallible check, then the cell writes, then the protocol commit. The fader and matrix room terms
  of step 4 go; nothing else moves. The browser writes cells only after its whole batch passed
  every check.
- **D7. Counters.** `CounterId::LiveValuesSuperseded = 16` in the frozen registry; the control
  plane sets it on its provider at every control call, as it does the telemetry counters
  (`control.rs:850-853`). The browser exposes it as `WebStatus::live_values_superseded`, which
  takes `reserved[0]`; the layout does not change.
- **D8. Ack meaning (unchanged bytes).** Header text: a committed live value reaches render no
  later than the first block whose render call begins after the submit returns; a later value for
  the same lane committed before that block replaces it, and `LIVE_VALUES_SUPERSEDED` counts it.
  The 16-value room and the backpressure string are removed for fader, mute and pan or matrix;
  effect lanes keep theirs.
- **D9. FIFO stays FIFO.** Effect records (including `Observe`), input records, route records,
  automation and structural edits are untouched (D15-2).
- **D10. The acked-batch question: can an ack ever precede a drop? No.** Every fallible check
  precedes the first cell write (D6), and writes cannot fail. A value replaced before render read it
  is not dropped: the committed model holds the newer value, render applies it, and the counter
  records the replacement (D2). A value written to a plan that is swapped out before it drains is
  in the committed model its successor is prepared from (#1053 D7, D15-17).

## Deliverables

1. The cell module and its loom model.
2. Fader and matrix lanes on cells in `builtins-compiler` (bank and test-only scalar processors).
3. The C ABI and browser admission changes, the counter on both hosts, header and docs text.

## Authorized paths

- `crates/engine/src/realtime/` (new cell module, `mod.rs` exports).
- `crates/builtins-compiler/src/lib.rs` (after stream A's edits there; then stream D).
- `crates/host-core/src/live_delta.rs`, `crates/host-core/src/control_provider.rs` (D7 setter),
  `crates/host-core/src/prepare.rs` (the producer and counter plumbing only; stream A's file,
  sequenced by the coordinator).
- `crates/control-plane/src/`, `crates/capi/` (header prose and tests), `crates/protocol/src/`
  (D7).
- `hosts/host-web/src/lib.rs` (matrix and fader bands, `WebStatus`), `hosts/host-web/src/tests.rs`,
  `hosts/host-web/tests/` (gate 3), `hosts/host-web/Cargo.toml` (that dev-dependency), and the SDK
  files `bash scripts/check-sdk-generated.sh` regenerates for `WebStatus`. These are
  stream H's; if #1332 has landed, the browser admission is the control plane's and host-web needs
  only the status field.
- `docs/C_ABI_V1_QUALIFICATION.md`, `docs/CONTROL_PROTOCOL_REGISTRY.md` (counter 16).

## Non-goals

- Effect, input and route lanes (#1345, #1346, #1347). Ramp defaults (#1054).
- Per-transaction atomicity across cells: as today, one transaction's values may land one block
  apart.

## Hazards

- **Digests move only where two records met in one drain.** Re-pin each such fixture with its
  reason ("step then ramp" now ramps from the running value); never in bulk.
- **Fader and mute share one gain ramp** (`set_mute` is "a retarget of the same gain"). D4's order
  keeps a mute ramp from being cut short by a fader move in the same block.

## Objective gates

1. **Many edits, one block (new capi test).** Paused host: 40 fader edits on one track, each
   `RESULT_OK`; one render. The output equals a twin that made only the last edit;
   `COUNTERS_GET` reports `LIVE_VALUES_SUPERSEDED` 39. *Red if a lane is still a queue (refusal
   at 17) or the gap arithmetic is off by one.*
2. **No write before the predicate (new capi test).** Arm `TestStructuralFaultPhase::BeforeLivePush`
   on a fader-and-pan edit: every cell's sequence and the output are unchanged. Mutation (PR
   evidence): move the cell writes above step 5; this test turns red.
3. **Both hosts agree (new test in `hosts/host-web/tests/`, `control-plane` as a dev-dependency).**
   One session, one script with two fader edits and two pan edits between two blocks, delivered
   through the control plane's `SESSION_TRANSACTION_APPLY` and through host-web's command
   admission: bit-identical PCM and equal counters.
4. **Torn reads never spin (loom, new `spsc_loom_cells_*` tests).** A writer racing a reader: the
   reader never observes a mixed cell, applies each sequence at most once, and the counter equals
   writes minus applies minus the one pending. Command: `CARGO_TARGET_DIR=target/ci/loom
   RUSTFLAGS='--cfg loom --check-cfg=cfg(loom)' cargo test --locked --release -p engine --lib
   spsc_loom`.
5. **Superseded tests.** Delete `a_full_live_lane_refuses_before_anything_changes` and
   `a_live_transaction_with_one_full_lane_pushes_to_no_lane`
   (`crates/capi/src/runtime/live_tests.rs:609`, `:692`); gate 1 replaces them. Adapt
   `a_replayed_live_edit_pushes_nothing` (`:726`) and
   `a_fault_before_the_live_push_leaves_every_queue_and_the_model_alone` (`:1092`) to cell
   sequences. Rewrite `command_flood_is_typed_backpressure_and_leaves_the_render_untouched` and
   `overfilling_a_bus_queue_is_typed_backpressure_with_no_push`
   (`hosts/host-web/src/tests.rs:4775`, `:10187`) on observation records
   (`COMMAND_OBSERVE_SUBSCRIBE`), which stay a FIFO after #1345, or delete one that would
   duplicate an existing observation test.
6. **Realtime.** `cargo build --locked --release -p audit -p capi && target/release/audit capi`
   (all violation counts 0); `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-web-audioworklet.sh` (its call-graph gate); the browser legs of
   `qualification.yml`'s `browser` job.
7. **Workspace.** `cargo test --locked -p capi`; `cargo test --locked -p builtins-compiler
   --features test-support`; `cargo test --locked -p host-web --features test-support`;
   `cargo test --locked -p host-core --features control-provider,test-support`;
   `cargo test --locked -p protocol --features test-support`; `bash scripts/check-capi-abi.sh`;
   `bash scripts/check-sdk-generated.sh`; `bash scripts/check-workspace-policy.sh`;
   `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets --all-features
   -- -D warnings`.

## Test value

- Gate 1: a lane left a queue, or a miscounted supersession.
- Gate 2: a cell write moved before a fallible check, which with cells destroys an acked value.
- Gate 3: the two hosts draining differently (order, `Both` folding), which breaks #1054's
  cross-host bit-identity for two edits in one block.
- Gate 4: a seqlock ordering bug (torn apply, double apply, spin); judged by the interleavings
  loom reaches.

## Dependencies

- *Extract the C ABI control plane into a portable crate both hosts call* (#1309).
- *Carry fader, mute and pan ramps across a plan swap* (#1277): stream A edits
  `crates/builtins-compiler/src/lib.rs` before this slice.
- Followed by #1345 (after *Carry live-controlled effect lanes across a plan swap*, #1280), #1346
  and #1347, each on this primitive and counter.
