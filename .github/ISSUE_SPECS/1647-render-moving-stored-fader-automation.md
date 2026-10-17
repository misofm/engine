# Render moving stored fader automation, seeks and latency

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.3-A1.5, A1.7) and A5, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A producer's fader ride is heard on every platform. On the C ABI and in the browser, each automated
fader lane prepared by #1646 follows its curve in node time: it reaches the curve's value at
each ramp's completion sample on a fixed 64-sample grid, takes each discontinuity at its exact
sample over the session's fader ramp length, lines up with the audio it acts on through every
latent insert, and is set exactly where a session seek reaches the fader. The output does not
depend on the quantum or on where a seek landed. A session with no stored automation renders
exactly as before. The fader row leaves the classifier's mask in #1648, in the same batch.

## Context

- **Where the fader renders.** `FaderBankProcessor` (`crates/builtins-compiler/src/lib.rs:715-723`)
  drains its lanes' live records, then processes the bank (`:738-752`); the fused fader-and-matrix
  processor does the same and fuses only when nothing ramps (`:1198-1219`). Graph runs a strip's
  builtin chain with the block's first sample (`crates/graph/src/runtime.rs:3046-3048`) through
  `GraphPreparedBuiltinBankProcessor::process` (`crates/graph/src/lib.rs:1347-1353`). The graph
  executor calls the source set at block entry (`crates/graph/src/lib.rs:3254-3256`), where #1641
  advances the timeline.
- **What #1646 leaves.** Each automated lane has a cell program (#1637) with its `a(n)`, its
  grid and no jump length (#1637 as shipped, departure 1 of its decision record: a caller resolves
  the fader row's length with `LiveRamps::for_row` from the three lengths #1639's plan cell holds),
  an offsets cell (the lane's VCA offset sum `S`, `+0.0` when no VCA reaches it; #1646 D1), and
  its event state in the fader bank stage. It starts at its prepared value, the curve at node time
  `-a(n)`, which is the entry's first `start_value`.
- **What #1639 leaves.** The graph reads the plan's jump-length cell once at block entry and holds
  its three words for the block; no stage reads them yet.
- **The documents.** `docs/SESSION_SCHEMA_V1.md:220-225` says the table renders nothing; the
  comments at `crates/session/src/validate.rs:812-822` and `:842-853` say the same;
  `docs/REALTIME_MEMORY.md:13` gives #1058 the rendering.
- **What this slice builds on.** #1640 and #1641 (the timeline and its reader), #1642 and #1643 (the session
  seek on each host), #1637 (events), #1638 (operations inside a block, the held-gain rule).

## Decisions frozen for this slice

- **D1. Render.** Per block, after the live drain at block entry, the fader bank processor:
  1. reads, for each automated lane, its node-time mapping: graph passes the block's source-read
     sample and a timeline reader (#1640) to the strip chain through a new defaulted method on
     `GraphPreparedBuiltinBankProcessor` (`process_timed`, which forwards to `process` by default),
     and the processor reads `timeline_block(s0 - a(n))` into #1637's `NodeSpan`. The same call
     passes the block's jump lengths, the three words #1639's read gives the graph for that
     block; for each cell the processor resolves the fader row's jump length from them with
     `LiveRamps::for_row` and passes it to `CellEvents::new` as one `u32` (#1637 D4, as shipped:
     #1637 stores no jump key and writes no row rule);
  2. tells each cell the end of a ramp in flight it did not start (a live mute ramp) through
     `hold_until`, from the stage's `remaining` (#1638 D2);
  3. pulls each lane's events (#1637, `SeekMode::Exact`) and merges them by offset across lanes
     with a fixed per-lane cursor (no buffer);
  4. converts each event value: `Grid` and `Jump` become `RetargetGain`, a held event
     `RememberGain`, `Set` `SetGain` (#1638 D1), each gain composed as #1646 D1;
  5. runs `process_with` (#1638 D3). The fused fader-and-matrix path is taken only when no
     fader or matrix lane moves in the block.
  A gain the conversion refuses is unreachable (#1645 bounds the curve to the fader domain and
  `vca_compose_db` clamps, #1646 D6); it keeps the current target, adds one to a render counter and fails a
  `debug_assert`.
  *(Superseded by the U2 fold, MINOR-7: the counters were deleted; the condition is unreachable, keeps the stage's target and fails its `debug_assert`.)*
- **D2. First blocks.** A fresh plan starts every automated lane at its prepared value (#1646
  D2), so its first block emits no `Set`. A lane a successor adds or restarts takes a `Set` at its
  first block when the curve at that block's node time differs from its prepared bits (README A1.4,
  "New lanes"). Carrying a lane's event state across a swap is #1648's.
- **D3. Seeks.** A session seek reaches the fader where its node time steps; the cell emits `Set`
  there and the lane takes the curve's value exactly, with no ramp (README A1.4).
- **D4. Docs.** `docs/SESSION_SCHEMA_V1.md:220-225`: the fader row (5) renders; every other row is
  valid and inert until its slice. The comments at `crates/session/src/validate.rs:812-822` and
  `:842-853` say the same. `docs/REALTIME_MEMORY.md:13`: stored fader automation is a compiled table
  in the plan (#1058), with no queue.
- **D5. The acked-batch question.** Stored automation has no queue: render generates its events
  from the plan. Nothing is acknowledged and later dropped.

## Deliverables

1. D1-D3 in `crates/builtins-compiler/src/lib.rs`, and the timed call in `crates/graph/src/lib.rs`
   and `crates/graph/src/runtime.rs` (the strip chain call only).
2. D4's documents.
3. Tests in `crates/host-core/tests/stored_fader_automation.rs`.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (the fader bank processor, the fused processor and the
  scalar track's render path)
- `crates/graph/src/lib.rs` (`process_timed` on `GraphPreparedBuiltinBankProcessor` and the
  timeline reader the chain passes), `crates/graph/src/runtime.rs` (the strip chain call)
- `crates/host-core/tests/stored_fader_automation.rs`, `crates/capi/src/runtime/tests.rs`
  (gate 4 on the C ABI)
- `docs/SESSION_SCHEMA_V1.md`, `docs/REALTIME_MEMORY.md`, `crates/session/src/validate.rs`
  (comments only)
- `crates/source/src/lib.rs` (the source driver's `timeline_block` reader only: #1641 D8 adds
  only `timeline_at` and leaves the render-side block reader to this slice)

## Non-goals

- Preparation, prepared values, bytes and the mono collapse (#1646).
- Classifying fader automation edits, the rule for a live edit of an automated lane, and carrying
  event state across a swap (#1648).
- The live VCA move that rewrites the offsets cell (#1649). The `control_smoothing` edit path
  that writes the jump-length cell (#1639).
- Mute, pan, matrix, input and effect rows (#1651 to #1667). The live path's held-gain rule (#1054
  D10).
- A benchmark (#1671 and #1672).

## Hazards

- **One push with #1646, #1648 and #1649** (README "Must-land-together groups"). Until #1648 is
  in, a rebuild for another reason sets each automated lane exactly at adoption (D2), and a live
  fader edit on an automated lane, on either host, retargets it until the next event. Until
  #1649 is in, an acknowledged VCA ride on an automated member is not heard. The four merge in one
  push, so `main` never holds these states.
- **The chain call is shared.** `process_timed` defaults to `process`, so every other builtin bank
  is untouched; only the fader bank processor and the scalar track override it.
- **No silence skip.** A lane with an event in the block takes the ramping path; a flat curve emits
  no event and changes nothing (README A1.7, finding F10).

## Objective gates

1. **Completion samples** (`crates/host-core/tests/stored_fader_automation.rs`, new cases). A
   linear dB fade from `-40` to `0` dB over 48,000 samples on a DC source of `1.0`,
   `control_smoothing` explicit 0: at each completion sample `64k + 63` the output equals
   `checked_fader_gain(curve(64k + 63))` exactly, and between them it is the ramp kernel's
   interpolation.
2. **Jumps at exact samples.** A `step` from `0` to `-6` dB at sample 1,000 (not a grid sample):
   with `control_smoothing` explicit 0 the output changes exactly at sample 1,000; with the default
   table the ramp starts at sample 1,000 and ends at `1,000 + fader length - 1`. **The next jump,
   not the one in flight:** steps at timeline 1,000 and 3,000, fader length 480; a test-support
   write of 960 to #1639's cell at a block entry while the first jump ramps: that jump completes
   over 480 and the second ramps over 960, bit-identical to the stage driven directly with
   #1638's operations at those offsets and lengths.
3. **Latency.** Two tracks on one source, the first with a latent insert before the fader (a
   true-peak limiter, latency `λ`), each with a fader step at timeline 4,800: the step's event is at
   render sample `4,800 + a(n)` on each track's fader (a test-support event probe), so the two
   events are `λ` apart, and in the output both steps meet the source frame 4,800 they act on.
4. **Session seek.** After a session seek to `T` that reaches the fader at render sample `r`, the
   lane equals `checked_fader_gain(curve(T))` exactly from `r`, with no ramp: through #1641's
   producer in host-core, and through `miso_engine_v1_session_seek` in
   `crates/capi/src/runtime/tests.rs`.
5. **Quantum invariance.** The same session at quantum 128 and at quantum 100 gives the same fader
   output bits, sample for sample, from sample 0, with and without a session seek.
6. **No allocation, no change without automation.** 1,000 blocks with events on eight lanes allocate
   and free nothing (`bench_support::alloc` thread counters after warming). Every existing render
   test passes with unchanged digests; `./target/release/audit capi` shows the same `pcm_digest` as
   the base (PR evidence; the audit session has no automation) and 0 violations.
7. **Commands:**
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `cargo test --locked -p builtins-compiler --features test-support`
   - `cargo test --locked -p graph --features graph/test-support`
   - `cargo test --locked -p capi`
   - `cargo test --locked -p host-web --features host-web/test-support`
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
     `./target/release/audit capi`
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-web-audioworklet.sh --without-metadata-regeneration target/ci/qualification-artifacts target/ci/qualification-named-twin/miso-engine-v1-audio-worklet.simd128.named.wasm`
   - `bash scripts/run-aarch64-tests.sh debug` (the `aarch64-debug` job)
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule: no new
     `memset_pattern16` call; fix one in code, never by a ceiling)
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if the completion is off by one or the grid follows render samples.
- Gate 2 turns red if a jump snaps to the grid or ignores `control_smoothing`, if a written length
  cuts a jump in flight, or if the jump length is read only at preparation or one block late.
- Gate 3 turns red if node time ignores `a(n)`: automation would lead its audio by the latency.
- Gate 4 turns red if a seek ramps the lane or reaches the fader at the wrong sample, on either
  path.
- Gate 5 turns red if any value depends on the quantum (README blocker B1).
- Gate 6 turns red if render allocates for events, or if a session without automation moves a bit.

## Dependencies

- #1646 *Prepare stored fader automation and render it flat*, in the same push. It brings
  #1637, #1638 and #1639 and #1312 (the events, the timed operations, the jump-length cell and the
  live input D1 drains first).
- #1641 *Give every plan a timeline that carries like a source*.
- #1642 *Seek the timeline and every source in one C ABI call* and #1643 *Seek the timeline
  and every source from the browser module export and the headless SDK* (the session seek on each
  host).
- Batch: R1, in one push with #1646, #1648 *Classify fader automation edits as carried
  rebuilds* and #1649 *Compose VCA offsets with stored fader automation*.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **Comment anchors gone.** The `crates/session/src/validate.rs` comments of D4 (`:812-822`,
  `:842-853`) left with `BUILTIN_AUTOMATION_TARGETS` (#1566); the inert sentence is now in
  `docs/SESSION_SCHEMA_V1.md` near `:278-279` and in `validate_automation_target`'s rustdoc
  (`validate.rs:868-870`). D4 rewrites those.
- **Composition.** Each event's gain is #1646 D1's with `S` that includes the hand offset (#1650).
- **Must-land group** is #1646, #1647, #1648, #1649 and #1650.

## Amendment (controls coordinator, batch 13b, from #1638's reviews)

Checked against `d5b9410df`. This is a requirement for this slice's brief, with the constraints
three #1638 reviews measured; the mechanism is this slice's design, reviewed with it.

- **The gap (a requirement, D1.6).** #1638 D2 holds a lane's gain during an unmute ramp as well as
  a mute ramp: while the stage reports `gain_held`, a timed `RetargetGain` or `SetGain` only
  remembers the gain. #1637 commits a `Jump`, a seek `Set` and a grid event that is not held as the
  cell's target with `held = false`, and on a curve that is flat afterwards it emits nothing more.
  So any gain event the stage only remembers inside a held ramp leaves the lane at the pre-unmute
  gain with no end. The requirement: when a held ramp ends with the gain free, the lane moves to the
  automation value the cell then holds (composed as D1.4, with `S` and the #1650 hand offset), and
  the result does not depend on the quantum (gate 5). Every remembered gain event counts, not only
  `Jump` and `Set`: in every hold of length `R` the one grid point in its last 64 samples is not
  held by #1637 (its own completion reaches the hold's end) and commits.
- **Constraints the design must meet (measured with the real `crates/automation`).**
  1. D1.2 reports the hold with `hold_until` once, at the block entry whose live drain starts or
     restarts the held ramp, and again after an adoption that carries it (`CellState::reposition`
     drops it). A refresh at later block entries overwrote a cell `Jump`'s own ramp end and made
     `target()` at the ramp end -0.0616 dB at quantum 128 against -0.1955 dB at 100.
  2. The cell is stopped exactly at the ramp end `E`: the lane's `NodeSpan` ends at `E` and a second
     span starts at `E + 1`, with any seek step kept in the piece that holds it; the per-lane merge
     (D1.3) never pulls an event past `E` before the move at `E`. `CellEvents` borrows the
     `CellState`, so `hold_until` cannot be called inside a span. A move built at the block end or
     the next block entry instead gave -0.4273 dB at 128 against -0.4535 dB at 100.
  3. The move at `E` holds the grid for its own ramp (`hold_until(completion_of(End, E, J))`), as
     after the cell's own `Jump`; otherwise a moving curve's grid events cut it into 64-sample
     retargets.
  4. A ramp that ends muted keeps the obligation for the later unmute ramp's end. A live fader write
     that clears the hold inside it discharges the obligation at that block entry.
  5. D3's "exactly, with no ramp" does not hold for a seek `Set` inside a held ramp: the stage only
     remembers it, and it lands by the move at `E`.
  6. The stage's `fader_operations_stopped` counter (#1638) is summed into this slice's report.
  7. `E` is the first sample at which the stage reports `gain_held` false for the lane. The move at
     `E` replaces any gain operation the cell gives at `E`: #1638's sink takes one gain operation per
     lane, channel and offset and refuses a second (`FaderOperationError::Duplicate`).
- **Gate 8 (new).** At quantum 128, 100, 64 and 37, the same bits, and the lane never settles at the
  pre-unmute gain, in these cases, with `J` the fader row's jump length:
  - (a) a `Jump`, and (b) a seek `Set`, inside an unmute ramp on a curve flat after it: from
    `E + J - 1` (from `E` when `control_smoothing` is explicit 0) the lane equals the gain composed
    from the cell's `target()` exactly, so it settles at that value one fader jump length after `E`;
  - (c) a `Jump` whose own ramp ends after `E`, with a block entry between the `Jump` and `E` at one
    quantum only: samples `[E, E + J - 1]` equal the stage driven directly with one `RetargetGain`
    at `E` over `J` (no grid event cuts the move's ramp);
  - (d) a curve that goes flat inside the last 64 samples of the unmute ramp (no `Jump` or `Set`,
    only grid events);
  - (e) grid events that are not held lying between `E` and the next block entry at one quantum only
    (a `Jump` whose ramp ends before `E`, or a seek `Set` on a moving curve): samples
    `[E, E + J - 1]` equal the stage driven directly with one `RetargetGain` at `E` over `J`.

  Test value: red if a remembered event is never moved to at `E`, if the move follows a block entry
  instead of `E`, if the lane does not equal the composed `target()` from `E + J - 1` (or from `E`
  with `control_smoothing` 0), if D1.2 refreshes `hold_until` inside the hold, or if a grid event
  cuts the move's ramp in `[E, E + J - 1]`.
- **#1648** carries the obligation across a swap (see its amendment).

## Amendment (controls coordinator, batch 15b fold, 2026-10-16)

- #1285 landed first (batch 15b). `a(n)` above must therefore be the floored input arrival
  (`GraphCompileReport::input_arrivals`, kept in `PlanStateInventory::input_floors()`), not the
  arrival the graph compiler computes without floors.

## Amendment (controls coordinator, batch 17 spec check, 2026-10-16)

Checked against `31fa33fee`.

**Anchors moved.**

- `FaderBankProcessor` `crates/builtins-compiler/src/lib.rs:1519`; its `begin_block` (the drain)
  and `process` `:1630-1660` (`process` calls `bank.process`, not `process_with`). Fused
  `FaderMatrixBankProcessor` `:1831`, `process` `:2334` (fuses through
  `try_process_settled_with_matrix`).
- The chain call is in `rack`, not graph: `Runtime::execute` (`crates/graph/src/runtime.rs:3188`)
  calls `BankChain::run` / `run_with_resident_input` (`:3310-3318`); `BankChain::run` (`crates/rack/src/lib.rs`
  about `:3230-3245`) calls each slot's `begin_block` then `process(BankBlock)`; the adapter
  `BuiltinStage` (`runtime.rs:4325-4349`) forwards to `GraphPreparedBuiltinBankProcessor::process`
  (`crates/graph/src/lib.rs:1737`) and `begin_block` (`:1782`).
- The source set's block entry and the read clock: `crates/graph/src/lib.rs:4983-4996`
  (`source_read_sample = absolute_sample + source_read_offset`, #1396). The jump lengths are read at
  `:4965-4971` into a local and are not passed on.
- `#1640`'s reader is `TimelineConsumer::timeline_block(first_source_read_sample: u64, frames) ->
  TimelineSpan` (`crates/source/src/lib.rs:2791`); the driver trait has only `timeline_at`
  (`crates/graph/src/lib.rs:2672`; impl `crates/source/src/lib.rs:3361`).
- D4: the inert text is `docs/SESSION_SCHEMA_V1.md:286-291`; `validate.rs` no longer says the table
  is inert (`:868-870` now says "structure only"), so `crates/session/src/validate.rs` leaves the
  list. `docs/REALTIME_MEMORY.md:13` is unchanged.

**Landed deps and what they give.**

- #1640/#1641: every host-core plan has a timeline (`prepare.rs:2949-2990`), history
  `K = ⌈A_max/q⌉ + 1`. #1637 shipped node time as `timeline(r + ΣP - a(n))`.
- #1639: `GraphJumpLengths` held per block in the executor; `LiveRamps::for_row`
  (`crates/controls/src/live_ramps.rs:44`).
- #1638: `BuiltinFaderBank::process_with`, `FaderOperationSink`, `gain_held`, `ramp_remaining`,
  `BuiltinProcessReport::fader_operations_stopped` (`crates/builtins/src/lib.rs:160`, `:4564-4625`).
- #1642/#1643, B16-R9: `SessionState::session_seek` (`crates/control-plane/src/control.rs:4119`) is the
  one core function behind `miso_engine_v1_session_seek[_at]` and
  `miso_engine_web_v1_session_seek[_at]`. A plain seek declares a discontinuity first; when the
  newest plan has a raised floor or `ΣP > 0`, that publishes a successor whose owners start at
  rest, so its automated lanes start fresh (D2's "restarts" case). An anchored seek declares
  nothing.
- #1285: `a(n)` is the floored arrival, known only after `compile_with_builtins`.

**Departures from the filed text.**

1. **`process_timed` cannot be a defaulted forward through the chain** without editing `rack`
   (`BankStage`/`BankBlock` carry no context). Ruling Q2.
2. **`a(n)` has no path to the fader stage** (the program holds none; #1646 now leaves it here).
   Ruling Q1.
3. **Negative node samples.** At `r + ΣP < a(n)` the node block starts before source-read sample 0,
   which `timeline_block(u64, ..)` cannot take. The driver-trait method and the source reader take a
   signed first sample and extrapolate back as `timeline_at` does (README A1.4, "New lanes":
   node time `-a(n)` at render 0). This widens the `crates/source/src/lib.rs` path to the reader.
4. **A restart's first-block `Set`** (D2) has no seek step to come from: the processor builds that
   block's `NodeSpan` with a step at offset 0 (`NodeSpan::new(t, Some((0, t)), q)`), so
   `SeekMode::Exact` emits the `Set`.
5. **The scalar track is the test-only oracle** (#1646 Q2). With #1646 Q2 = B, its render path gets
   the same event driver; with A, nothing here.
6. **Counters.** D1's refusal counter and the stage's `fader_operations_stopped` are
   `FaderBankProcessor` fields read off render (a test-support read and its qualification counters);
   name them in the brief. Each condition is unreachable and `debug_assert`s.
7. **Jump lengths.** Build `LiveRamps` from the three held words through one `controls` constructor
   (the link length equals the fader length), so the render path writes no row rule.
8. **Parity (decision 17 C1).** Gate 4 has no browser leg; both hosts now have a session seek behind
   one core function, so add it (`hosts/host-web/src/tests.rs`, through
   `miso_engine_web_v1_session_seek`). Gate 4 also covers a plan with a raised floor, so the plain
   seek's discontinuity successor is tested (D2 restart path).

**Ruling questions.**

- **Q1. How `a(n)` reaches the fader stage.** (A) at the end of `compile_with_builtins` the graph
  compiler hands each builtin bank its members' floored input arrivals through a new defaulted
  method on `GraphPreparedBuiltinBankProcessor`; (B) host-core installs them after the compile
  through a new artifact method that downcasts the fader banks; (C) the graph runtime keeps each
  lane's arrival and passes a node-time sample per lane. **Recommendation: A** (one rule in the core,
  no host step; adds `crates/graph-compiler/src/compile.rs`).
- **Q2. How the block's node-time context reaches the processor.** (A) `rack`'s `BankBlock` (or a
  new `BankStage` method) carries the source-read sample, the timeline view and the jump lengths
  (adds `crates/rack/src/lib.rs`); (B) graph's runtime sets the context on each builtin slot
  through `builtin_processor_mut` (`runtime.rs:4389`) with a new defaulted
  `GraphPreparedBuiltinBankProcessor` method before `chain.run`, and `process` reads it; `rack` is
  not edited. **Recommendation: B** (no change to the generic chain contract; the context is
  graph's own).

**Authorized paths, as they now must read.**

- `crates/builtins-compiler/src/lib.rs` (fader bank, fused processor, the oracle per #1646 Q2)
- `crates/graph/src/lib.rs` (the context method, the driver trait's signed `timeline_block`, the
  arrivals method of Q1), `crates/graph/src/runtime.rs` (the context set before `chain.run`; the
  `GraphSourcePlanes` view), `crates/graph-compiler/src/compile.rs` (Q1 = A only)
- `crates/source/src/lib.rs` (`TimelineConsumer::timeline_block` signed form and the driver's
  `timeline_block`, with their tests)
- `crates/controls/src/live_ramps.rs` (the constructor of departure 7)
- `crates/host-core/tests/stored_fader_automation.rs`, `crates/capi/src/runtime/tests.rs`,
  `hosts/host-web/src/tests.rs` (gate 4's browser leg)
- `docs/SESSION_SCHEMA_V1.md` (`:286-291`), `docs/REALTIME_MEMORY.md` (`:13`)
- No path under `crates/engine/src/realtime/`. Every edited render function sits in a
  `REALTIME_POLICY` region and `scripts/check-realtime-policy.sh` must stay green.

**Gates' test files.** Gates 1-3, 5, 6, 8: `crates/host-core/tests/stored_fader_automation.rs`.
Gate 4: the same file, `crates/capi/src/runtime/tests.rs` and `hosts/host-web/src/tests.rs`.
Departure 3: a unit test beside the reader in `crates/source/src/lib.rs`. Gate 7 adds
`cargo test --locked -p source`, `-p controls`, `-p control-plane` and (Q1 = A) `-p graph-compiler`.

**Size.** About 16 implementer hours: too big for one slice. Split, in this order, all in batch R1:
**#1647a** the plumbing (Q1, Q2, the signed reader; a test-support probe shows each lane's
`NodeSpan`, `a(n)` and jump length per block; no audible change; about 4 h); **#1647b** events on
the fader (D1.1-D1.5, D2, D3, D4; gates 1-6; about 7 h); **#1647c** the held-ramp obligation of
the batch 13b amendment (gate 8; about 5 h).

## Rulings (root, batch 17, 2026-10-16)

These rulings override the filed text and every amendment above where they differ. They answer the
ruling questions Q1 and Q2 of the batch 17 amendment and add one requirement for the reader.

### R17-2 (Q1): option A

At the end of `compile_with_builtins` the graph compiler gives each builtin bank its members'
floored input arrivals `a(n)` through a new defaulted method on the prepared builtin bank processor
trait (`GraphPreparedBuiltinBankProcessor`).

- No downcast in host-core. Host-core installs nothing after the compile.
- `a(n)` is the floored arrival (`GraphCompileReport::input_arrivals`, #1285, batch 15b fold).
- `crates/graph-compiler/src/compile.rs` is an authorized path of this issue now, not "Q1 = A
  only". Gate 7 runs `cargo test --locked -p graph-compiler` always.
- This work is #1647's (#1647a, the plumbing). #1646's departure 1 already moves `a(n)` here. Its
  slice list must name it as #1647's too. The coordinator makes that edit in #1646, not this
  ruling.

### R17-3 (Q2): option B

The graph sets the per-block node-time context on each builtin slot through `builtin_processor_mut`
(`crates/graph/src/runtime.rs`), with a new defaulted method on the same trait, before
`chain.run`. `process` reads it. `rack` is not edited.

- The set must be allocation-free and O(slots) per block.
- It is covered by the realtime-policy gate and the allocation gate (gate 6). The brief for
  #1647a runs gate 6's counter on the context-set path, because #1647b is not yet in when #1647a
  lands.
- `crates/rack/src/lib.rs` is not an authorized path.

### R17-4: the test-only scalar oracle

This issue mentions the oracle (departure 5). #1646's Q2 is ruled option B. The scalar oracle gets
the same event driver as the bank processors, and its render path installs the program through the
one shared event driver. The scalar backend is not refused. So the oracle line of the authorized
paths ("the oracle per #1646 Q2") reads: the oracle gets the event driver. The coordinator records
the matching ruling in #1646.

### R17-5: the signed timeline reader is accepted

The signed form of `TimelineConsumer::timeline_block` and of the driver's `timeline_block`
(departure 3) is accepted. It must not overflow.

- Every conversion between the unsigned source-read sample and the signed first sample is checked
  or saturating. The same holds for `r + ΣP - a(n)` and for the extrapolation back from sample 0.
- A unit test beside the reader in `crates/source/src/lib.rs` runs at the `u64` and `i64` limits:
  a first sample above `i64::MAX`, at `i64::MAX`, at `0` and at `i64::MIN`, and a block that crosses
  each of them. It checks that nothing panics and nothing wraps.
- This is part of #1647a.

## Decision record: #1647a (batch 17)

The plumbing (R17-2, R17-3, R17-5, departure 7). No audible change: nothing reads the new values
in render yet except the test-support probe.

**Shipped.**

- **`a(n)` (R17-2).** `GraphPreparedBuiltinBankProcessor::set_input_arrivals(&[u64])` (defaulted,
  lane order). `PreparedGraphPlan::install_builtin_input_arrivals` looks each bank member up in
  `GraphCompileReport::input_arrivals` (absent = 0) and calls it; `compile_with_builtins` calls it
  once through `PreparedBuiltinsGraphArtifact::install_builtin_input_arrivals` (a narrow,
  doc-hidden forward; the artifact's graph stays read-only). No downcast, no host-core step. The
  fader bank keeps `a(n)` in each automated lane's `LaneFaderAutomation`; the fused fader-matrix
  bank forwards to its fader stage.
- **Block context (R17-3).** `GraphBlockTime<'a>` (graph): the block's source-read sample
  `r + ΣP` (`Option`, `None` only on `u64` overflow, which a plan with a source set already refuses
  as `TimeOverflow`), the block length, the held `GraphJumpLengths` (#1639) and a borrow of the
  source-set driver; `node_span(a)` reads `timeline_block(r + ΣP - a, frames)`.
  `GraphPreparedBuiltinBankProcessor::set_block_time(&GraphBlockTime)` (defaulted). The executor
  builds the context once per block after the source set's block entry and passes it to
  `Runtime::execute`, which calls `set_block_time` on every builtin slot of a bank unit
  (`builtin_processor_mut`, a `slot_stage_mut` walk) before `chain.run` or
  `run_with_resident_input`. No `rack` edit. The fader bank, when it has automated lanes, stores per
  automated lane the block's `TimelineSpan` and the fader row's jump length; a bank without
  automation returns at once.
- **Signed reader (R17-5).** `TimelineConsumer::timeline_block(first: i128, frames)` and the
  driver trait's `timeline_block(i128, u32) -> Option<TimelineSpan>` (source driver implements
  it). `i128` holds every `u64 - u64` exactly, so `r + ΣP - a(n)` needs no conversion; `resolve`
  works in `i128` with checked differences, `offset_timeline` saturates in `i128` then to the
  `i64` range. `step_at` keeps `u64` (steps only fall at entry boundaries, which are `u64`), so
  every existing `u64` result is unchanged. `TimelineSpan`/`TimelineStep` now live in `graph` and
  `source` re-exports them (one type across the driver trait).
- **Jump length (departure 7).** `controls::LiveRamps::from_lengths(fader, mute, pan)` (link =
  fader); `for_session` now goes through it. The fader bank resolves its row with
  `for_row(LiveRampRow::Fader)` once per block.
- **`expect(dead_code)`.** Removed from `FaderBankProcessor::automation` (now read by
  `set_block_time` and `set_input_arrivals`). The ones on `LaneFaderAutomation`,
  `AutomatedFaderCell` and `OffsetsLane` stay (their program/state/offsets readers are #1647b and
  #1649).

**Departure.** The per-block span and jump length are stored per automated lane inside
`LaneFaderAutomation`, not as a fixed per-bank array: a per-bank array grew every fader bank by
320 bytes and moved the graph fixture `direct-route.resources.json` (a session with no
automation). Now only automated sessions pay: `lane_slot_bytes` 120 -> 168 (x86-64), measured by
`fader_automation_sizes()` and charged by #1646's existing sum; no fixture moved.

**Gates.**

- Probe and allocation (gate 6's counter on the context-set path):
  `each_automated_fader_lane_takes_its_arrival_and_node_time_per_block`
  (`crates/host-core/tests/stored_fader_automation.rs`): two automated lanes, one behind a 5 ms
  true-peak limiter (`a(n)` = 486 from the plan inventory), `fader_ms` 20 (960 samples) apart from
  `mute_ms`/`pan_ms`; for 12 blocks each lane holds its `a(n)`, span `start = r - a(n)` (`-486` at
  block 0) with no step, and jump 960; blocks 2-11 allocate, reallocate and free 0. Red on the
  revert of the compile-end install (`[0, 0]` against `[0, 486]`), checked once.
- R17-5: `tests::timeline::the_signed_reader_extrapolates_back_and_saturates_at_the_limits`
  (`crates/source/src/lib.rs`): first samples `i128::MIN`, below/at `i64::MIN`, `-300`, `-40`
  (crossing 0, no step, values continuous), 0, `i64::MAX - 40`, `i64::MAX`, above it, `u64::MAX`,
  `i128::MAX`, and a history at the `u64` end of the clock with blocks crossing it.
- `cargo test --locked -p source -p graph -p graph-compiler -p builtins-compiler -p controls
  --features builtins-compiler/test-support,graph/test-support`: pass. `-p host-core --all-features
  --test stored_fader_automation`: 3 pass. `-p capi`: 198 pass.
- `realtime-policy`: ok, 155 regions in 33 files (base 153: `GraphBlockTime`'s impl and the fader
  bank's `set_block_time`).
- `./target/release/audit capi`: `pcm_digest` `bffc21e857b5e134` (unchanged), 0 violations, 0
  allocations.
- fmt; clippy `-D warnings` (touched crates, all targets, all features); no-feature
  `cargo check --all-targets` on the touched crates; `cargo check --workspace --all-targets
  --all-features`.

**Notes for #1647b.**

- Read `LaneFaderAutomation::{span, fader_jump}` in `process` (set before `begin_block` of the same
  block); build `NodeSpan` from `span` and add the restart `Set` step of departure 4 there.
  `span.start + i` must be checked: it saturates at the `i64` edges.
- The scalar oracle (R17-4) gets neither `a(n)` nor the block context: its fader owners are bound
  node processors run by `execute_op`, not bank slots. #1647b needs a matching path there (the
  bound-node processor trait and the graph-compiler install, or a per-node arrival at binding).
- A plan with no source set or no jump-length cell gives `span`/`fader_jump` `None`; every
  host-core plan has both.

## Decision record: #1647b (batch 17)

The events on the fader (D1.1-D1.5, D2, D3, D4; gates 1-6). The held-ramp obligation (gate 8)
is #1647c.

**Shipped.**

- **One event driver (D1.3-D1.5, R17-4).** `FaderEventDriver` (`crates/builtins-compiler`): per
  block, one fixed cursor per automated channel (at most 16, two per bank lane) holding its
  `CellEvents` (`SeekMode::Exact` from `controls::FADER_CELL_RULE`, the block's fader jump length)
  and the next event; it is the stage's `FaderOperationSource`, so the merge by offset needs no
  buffer. Each event's gain is `checked_fader_gain(vca_compose_db(value, S))` with the held `S`
  (#1646 D1): `Grid`/`Jump` -> `RetargetGain` over the event's ramp, a held event ->
  `RememberGain`, `Set` -> `SetGain`. A refused conversion or sink push keeps the target, adds one
  to the lane's `refused_events` and fails a `debug_assert` (unreachable). `finish` consumes any
  event a stopped stage did not pull, so the cell's state still advances.
  *(Superseded by the U2 fold, MINOR-7: the counters were deleted; the condition is unreachable, keeps the stage's target and fails its `debug_assert`.)*
- **Fader bank.** `process` renders through `BuiltinFaderBank::process_with(driver)` when the bank
  has automation, else the old `process` (a session without automation runs the same code). The
  fused fader-matrix bank fuses through `try_process_settled_with_matrix_and_operations`, so it
  fuses only when no fader event falls in the block (the matrix half declines a ramping matrix);
  its fallback is `process_with` then the matrix.
- **D1.1.** Each lane builds its `NodeSpan` from the span #1647a stores (`start`, the timeline step
  as `(offset, timeline)`); a span whose `start + frames` (or step `timeline + frames`) leaves the
  `i64` range is not rendered that block (the lane holds; only at the saturated clock edges).
  No span or no jump length (a plan without a timeline or jump-length cell): the lane holds.
- **D1.2.** The drain marks each lane and channel a mute record with a nonzero window started a
  held ramp on (`drain_fader_controls_marking`); right after the drain, at that block entry only,
  the cell gets `hold_until(node time of the ramp's last sample)` from the stage's
  `ramp_remaining` (batch 13b constraint 1). The oracle's drain does the same.
- **D2, departure 4.** `LaneFaderAutomation::fresh`: a lane's first block, with no timeline step,
  gets the step `(0, start)` when `value_at(start)` differs from the seeded target, so a fresh plan
  emits no `Set` and an added or restarted lane is set to its curve. When the first block has a
  timeline step, that step lands instead (the samples before it precede the seek on the node).
- **D3.** A seek is the span's step: `SeekMode::Exact` lands a `Set` at its offset.
- **Scalar oracle (R17-4).** `GraphRuntimeProcessor` gains defaulted `set_input_arrival(u64)` and
  `set_block_time(&GraphBlockTime)`, `GraphRuntimeSplitPairProcessor` a defaulted
  `set_block_time`. `execute_op` takes the block's `GraphBlockTime` and calls `set_block_time` on a
  bound node (and on a split pair's fader half) right before it processes.
  `PreparedBuiltinsGraphArtifact::install_builtin_input_arrivals` also gives each builtin binding
  its node's `a(n)` through `GraphNodeBinding::set_input_arrival`, with the one lookup
  `graph::floored_input_arrival` the bank install now uses too (the smaller core-consistent path:
  the same install, no per-node arrival at binding, no `GraphBindingBlock` field). An automated
  strip with no live control binds `AutomatedFaderProcessor` (the ramped fader, #1646b departure
  5); `FaderProcessor` is the plain unramped owner again. Both fader owners render through
  `render_oracle_fader` (the same driver at lane 0). The scalar pair never fuses an automated
  fader and the split pair never defers one; both forward the block time.
- **Probe (gate 3).** `builtins_compiler::test_only_take_fader_events()`: each event the driver
  gave a stage, with its render sample (fixed array, thread-local, test-support).
- **D4.** `docs/SESSION_SCHEMA_V1.md` (the fader row renders, every other row inert),
  `.claude/skills/author-session/SKILL.md` (same), `docs/REALTIME_MEMORY.md:13` (a compiled table in
  the plan, no queue).

**Departures.**

1. **Counters live in the lane slot**, not as `FaderBankProcessor` fields (departure 6):
   `LaneFaderAutomation::{refused_events, operations_stopped}` (a bank's stop counts on each of its
   automated lanes). A bank field would grow `size_of::<FaderBankProcessor>()` for every session
   and move `fixtures/graph/v1/direct-route.resources.json` (#1647a's finding); the slot is paid
   only by automated sessions. They are read off render; no qualification row is added.
   *(Superseded by the U2 fold, MINOR-7: the counters were deleted; the condition is unreachable, keeps the stage's target and fails its `debug_assert`.)*
2. **The oracle's automated fader never fuses or defers** (the bank fuses when no event falls in
   the block). Both are exact; the oracle has no fused-with-operations form, and the parity gate
   below proves the bits equal.
3. **Gate 4's restart case.** On the C ABI the raised-floor plain seek's discontinuity successor
   lands the seek through the span's step in its first block (its timeline history carries the
   seek), so the D2 restart `Set` is not what sets that lane; the restart path is gated by a
   successor that adds an automated lane mid-curve (below).
4. `S` excludes the #1650 hand offset until #1650 folds it into `offsets_sum`; render already
   composes with that field.

**Gates** (x86-64 AVX2 host).

- Gate 1: `a_fader_fade_reaches_the_curve_at_each_completion_sample` (completion samples exact;
  every sample equals the stage driven directly with the grid's retargets).
- Gate 2: `a_fader_step_jumps_at_its_exact_sample` (smoothing 0 and the default table),
  `a_written_jump_length_reaches_the_next_jump_not_the_one_in_flight` (480 then 960, bit-identical
  to the stage driven directly).
- Gate 3: `a_fader_step_meets_its_source_frame_through_a_latent_insert` (events at `4,800` and
  `4,800 + λ`; output equals the 0 dB session before and the -6 dB session from the step's output
  sample). Red on the revert of `a(n)` in `take_block_time` (checked once).
- Gate 4: `a_session_seek_sets_the_fader_lane_exactly` (host-core, one `Set` per channel at `r`,
  exact), `a_session_seek_sets_an_automated_fader_exactly_on_the_c_abi` (running plan and raised
  floor), `a_session_seek_sets_an_automated_fader_exactly_in_the_browser`
  (`miso_engine_web_v1_session_seek`).
- D2: `a_lane_a_successor_adds_starts_at_its_curve` (Simd8, Simd4; red with the restart `Set`
  disabled, checked once).
- Gate 5: `the_fader_events_do_not_depend_on_the_quantum` (128 against 100, with and without a seek).
- Gate 6: `rendering_fader_events_allocates_nothing` (1,000 blocks, events on eight lanes, 0/0/0);
  `./target/release/audit capi` `pcm_digest` `bffc21e857b5e134` (unchanged), 0 violations, 0
  allocations.
- R17-4: `the_scalar_oracle_renders_the_banks_fader_events` (Scalar against Simd4: no live control,
  live controls, between render calls; each with a live mute and unmute ramp on an automated lane;
  red with the oracle's events disabled, checked once).
- Commands: `cargo test --locked -p automation -p graph -p builtins-compiler --features
  builtins-compiler/test-support,graph/test-support`; `-p host-core --features
  host-core/test-support` (all); `-p capi`; `-p host-web --features host-web/test-support`
  (including `control_parity`): pass. `realtime-policy`: ok, 159 regions in 33 files (155 + the
  lane-automation impl, the driver, the bank helpers and the oracle render). fmt; clippy `-D
  warnings` (touched crates, all targets, all features); no-feature `cargo check --all-targets`.
- Bytes: `lane_slot_bytes` 168 -> 192 (x86-64: `fresh` and the two counters), measured by
  `fader_automation_sizes()` and charged by #1646's sum; no fixture moved.

**`expect(dead_code)` left.** `OffsetsLane` (the cell and its dirty word) and
`AutomatedFaderCell::{offsets, offsets_ramp}`: #1649 writes the cell, adds its render read and
uses the ramp word. Removed: `LaneFaderAutomation`, `AutomatedFaderCell` (cell, state, `S`), the
oracle `FaderProcessor` field and `LiveControlFaderProcessor::automation`.

**Notes for #1647c.** The move at `E` needs a lane's events in two pieces (`[.., E]` and
`[E + 1, ..]`): a cursor holds one `CellEvents`, which borrows the `CellState`, so the second piece
needs the first iterator dropped; give `FaderCellCursor` a pending second span and rebuild its
`CellEvents` when the first ends (or return the state from `CellEvents`). D1.2's report is
`report_held_ramp` at the drain; the adoption re-report belongs with #1648's carry, which must also
clear `fresh`.

## Decision record: #1647c (batch 17)

The held-ramp obligation (D1.6, batch 13b amendment, gate 8).

**Shipped.**

- **The obligation.** `AutomatedFaderCell::obligation` (`HeldRampObligation { owed, due_at_entry }`,
  one per automated channel). `owed` is set when the driver pushes a gain event at an offset where
  the stage holds the gain (any kind: `Jump`, seek `Set`, a committed or held grid event).
- **`E` (constraint 7).** `FaderEventDriver::new` takes the stage's state of each channel after the
  block's drain (`ChannelHold { held, muted, remaining }`, `bank_hold` on the bank, the ramped
  fader's accessors on the oracle). Held, not muted, `remaining < frames`: `E = remaining`. A ramp
  that ends on the block's last sample sets `due_at_entry`, so `E` is the next block's offset 0.
  Muted (settled or ramping to `0.0`): held all block, the obligation kept (constraint 4). Free at
  a block entry with no move due: a live write cleared the hold there, so `owed` is discharged
  (constraint 4).
- **The cut (constraint 2).** A channel with `E` in the block gets the span `[0, E]` (the seek
  step kept in the piece that holds it) and a pending `HoldEnd` with the span `[E + 1, frames)`.
  The cursor reports `E` as its next offset once the first piece is spent, so no event past `E` is
  pulled before the move. At `E`, any event the cell gives at `E` is consumed; with `owed` it is
  replaced (constraint 7) by `RetargetGain` to `checked_fader_gain(vca_compose_db(target(), S))`
  over the block's fader jump length `J`, the cell gets `hold_until(completion_of(End, node(E),
  J))` (constraint 3), and the second piece's `CellEvents` is built on the same state. Without
  `owed` the span is still cut (the events are the same; #1637 events do not depend on span
  boundaries) and the event at `E` is pushed as usual.
- **`CellEvents::into_state`** (`crates/automation/src/events.rs`): returns the borrowed state, so
  the cursor can report the move's ramp and rebuild its events (the #1647b note's second option).
- **`BuiltinFaderBank::is_muted(lane, channel)`** (`crates/builtins/src/lib.rs`): the bank had
  `gain_held` and `ramp_remaining` but nothing to tell a ramp toward unmuted from one toward muted.
  Mirrors `FaderMuteRampBuiltins::is_muted`.
- **Scalar oracle.** `render_oracle_fader` passes its ramped fader's state; it runs the same driver.
- No new `debug_assert`: a refused move uses the existing refusal closure and counter
  (`refused_events`). The stage's `fader_operations_stopped` stays summed per lane (constraint 6,
  #1647b). `finish` still advances a cell through a cut the stage did not reach (no move).
  *(Superseded by the U2 fold, MINOR-7: the counters were deleted; the condition is unreachable, keeps the stage's target and fails its `debug_assert`.)*

**Departures.**

1. **Gate 8 (e) includes one grid retarget.** On a curve that moves after `E`, #1637 commits the
   one grid point in the last 64 samples of any ramp it holds the grid for (its own completion
   reaches the ramp's end), as after the cell's own `Jump`; constraint 3 asks for exactly that
   hold. So `[E, E + J - 1]` cannot equal one `RetargetGain` alone on a moving curve. The test
   compares `[E, E + J - 1]` with the stage driven directly with the move at `E` over `J` and that
   one grid retarget (`64` samples, curve at its completion); every other grid point in the ramp
   is held. On a flat curve ((c)) the comparison is one `RetargetGain`, as written.
2. **Out-of-path edits**, both one accessor: `crates/automation/src/events.rs`
   (`CellEvents::into_state`) and `crates/builtins/src/lib.rs` (`BuiltinFaderBank::is_muted`).
3. **The move is made only for an owed channel.** A hold with nothing remembered ends with no
   move: the unmute ramp already aimed at the remembered gain, and the fused fader-matrix path
   stays available.

**Gates** (x86-64 AVX2 host). Each gate 8 case runs at quantum 128, 100, 64 and 37 (same bits)
and on the scalar oracle at 128 (same bits). The track is prepared muted; a live unmute of 1,000
samples is drained at render sample 0, so `E = 1,000` (a block entry at quantum 100 only: the
due-at-entry path).

- (a), (b): `a_remembered_jump_or_seek_set_moves_the_lane_at_the_unmute_ramps_end` (a step at
  timeline 400; a session seek to 6,000 drained with the unmute; `J` = 480 and explicit 0). Exact
  composed target from `E + J - 1` (from `E` at 0), the move bit-identical to the stage driven
  directly.
- (c): `a_jump_ramping_past_the_unmute_ramps_end_is_moved_to_at_its_end` (a step at 990, ramp to
  1,469; a block entry between the step and `E` at quantum 37 only).
- (d): `a_curve_flat_inside_the_unmute_ramps_last_grid_point_moves_the_lane` (a fade to `-12` dB
  ending at timeline 960).
- (e): `grid_events_after_the_unmute_ramps_end_do_not_cut_the_move` (a step at 300 whose ramp ends
  at 779, then a fade; a seek to 2,000 on a fade).
- Red on revert, checked once each: no move (all four tests red), no `hold_until` for the move
  ((e) red), no `due_at_entry` (all four red, quantum 100), no stage state on the oracle (all four
  red at the oracle leg).
- Commands: `cargo test --locked -p automation -p graph -p builtins -p builtins-compiler --features
  builtins-compiler/test-support,graph/test-support`; `-p host-core --all-features --test
  stored_fader_automation` (17 pass); `-p capi`; `-p host-web --features host-web/test-support`:
  pass. fmt; clippy `-D warnings` (automation, builtins, builtins-compiler, host-core; all targets,
  all features); no-feature `cargo check --all-targets`: clean.
- `realtime-policy`: 159 regions in 33 files (unchanged). The run fails on the known `unsafe` FP
  control-word writes in `crates/host-core/tests/stored_fader_automation.rs` (the U1 fold's test;
  fixed in the W lane), the same on the base; with those two lines removed in a local probe it is
  `ok`.
- `./target/release/audit capi`: `pcm_digest` `bffc21e857b5e134` (unchanged), 0 violations, 0
  allocations.
- Bytes: `lane_slot_bytes` 192 -> 208 (x86-64: the two obligation flags per channel), measured by
  `fader_automation_sizes()` and charged by #1646's sum; no fixture moved.

**Notes for #1648b.** Carry `HeldRampObligation` with the cell state across a swap, re-report a
carried held ramp with `hold_until` at adoption (`CellState::reposition` drops it) and clear
`fresh` on a carried lane. The successor's first block reads the stage state itself, so `E` needs
no carry; `due_at_entry` must carry with `owed` (a ramp ending on the predecessor's last block). A
release cell takes the same obligation and the same move.

## Amendment (U2 fold, batch 17, from root's rulings R17-8, R17-9 and R17-10)

These amend the batch 13b amendment and gate 8 above; the filed text stays as it was filed.

- **The move (R17-9).** Where the batch 13b amendment says "the lane moves to the automation value
  the cell then holds" and gate 8 (a)/(b) say "the gain composed from the cell's `target()`", read:
  at `E` the cell lands as a seek in `SeekMode::Ramp` does, one `Jump` over the block's fader jump
  length `J` aimed at the curve at the ramp's completion `E + J - 1` (never past the next
  discontinuity, which jumps of its own), composed as D1.4, so the lane arrives on the curve at
  `E + J - 1`. On a curve flat after `E` this is the cell's target, so (a)-(d) keep their bits.
  The landing commits that target and holds the grid for its own ramp (constraint 3).
- **Constraint 7 (R17-8).** "The move at `E` replaces any gain operation the cell gives at `E`"
  holds for a `Jump` and a grid event. A seek `Set` that lands exactly at `E` is pushed instead:
  the gain is free from `E`, so it sets the lane there exactly (D3), and it discharges the
  obligation. Constraint 5 ("a seek `Set` inside a held ramp lands by the move") is for a seek
  before `E`.
- **Constraint 4 (R17-10).** A live write that frees, at a block entry, a gain the stage held
  through the end of the previous block discharges an owed obligation and, in the same block,
  releases the cell's hold, whether or not a move was owed (U2 fold 2), so the grid moves the lane
  from that block's first sample. A hold that ends by itself needs no release: the cell's hold of
  it ends at the same sample.
- **Gate 8 (e).** "Samples `[E, E + J - 1]` equal the stage driven directly with one `RetargetGain`
  at `E` over `J`" reads: with one `RetargetGain` at `E` over `J` to the curve at `E + J - 1`, and
  the one grid retarget #1637 commits in that ramp's last 64 samples (the grid point at
  `⌊(E + J - 1)/64⌋·64`, over 64 samples, to the curve at its completion); every other grid point
  in the ramp is held (#1647c departure 1, confirmed by the U2 review).
- **Gate 8's test value.** "Red if D1.2 refreshes `hold_until` inside the hold" is withdrawn: under
  R17-9 the landing at `E` rewrites the whole cell state, and before `E` the stage only remembers,
  so neither a refresh nor a missing D1.2 report reaches the output (U2 fold, finding MAJOR-1).
  D1.2 itself is removed (U2 fold 2).

## U2 fold (batch 17)

The U2 review of #1647 (at `5e695ecb6`) and root's rulings R17-8, R17-9 and R17-10, folded on
`codex/controls-batch-17k` over `275ff76e8`. Uncommitted for the coordinator. All tests in
`crates/host-core/tests/stored_fader_automation.rs` unless named otherwise. Each "red" was checked
once against the mutation the review found green (or the stated revert), then the code restored.

- **MAJOR-1 (constraint 1, D1.2): not gateable after R17-9; no test added.** A case with a `Jump`
  at 950 inside the unmute ramp, a fade after it and a quantum-64 block entry on grid point 960
  (4 quanta and the oracle) stayed green under both mutations (D1.2 removed; `hold_until`
  refreshed at every held block entry). Reason: before `E` the stage only remembers every gain
  operation, a refresh only moves the cell's state, and the R17-9 landing at `E` rewrites all of
  that state (cursor, target, held flag, ramp end). Without a move nothing was remembered, so the
  hold changes nothing, except when a live write frees the lane early (U2 fold 2: R17-10 then
  releases the hold at the entry of the block whose write freed a gain the stage held through the
  previous block, owed or not). The test was deleted (it defended nothing); D1.2 was kept here and
  is removed in U2 fold 2. Gate 8's test-value clause is withdrawn above.
- **MAJOR-2 (constraint 7).** `the_move_at_the_unmute_ramps_end_replaces_the_cells_event_there`: a
  step exactly at `E` (to `-15` dB at 1,000 after a remembered fade), and a grid point at `E`
  (an unmute of 1,024 on a fade); 4 quanta and the oracle. Red (the test process aborts on the
  sink refusal's `debug_assert` inside render) with the cell's event pushed beside the move.
- **MAJOR-3 (D1.4, `S`).** `a_vca_offset_composes_with_every_fader_event`: gate 1's fade on a
  member of a VCA at `-6` dB, every completion sample `checked_fader_gain(vca_compose_db(curve,
  S))` and every sample the stage driven directly with the composed retargets. Red with the event
  gain composed without `S`. A live `S` change stays covered by
  `the_scalar_oracle_renders_the_banks_fader_events` (#1649's writes), not added here.
- **MAJOR-4 (seeks inside a node block).**
  `a_session_seek_sets_a_latent_fader_lane_exactly_where_it_reaches_the_node` (gate 4 behind a
  true-peak limiter, quanta 128 and 100: one `Set` per channel at `3,200 + a(n)`, the unseeked
  bits before it, the static plan's bits from it); red with the seek step moved to the block's
  offset 0. `a_seek_step_after_the_unmute_ramps_end_in_its_block_still_lands` (gate 8 with a
  latent insert, quanta 100 and 37 with the oracle: the seek's `Set` after `E` in `E`'s block);
  red with `hold_end` dropping a step after `E`.
- **MAJOR-5 / R17-10 (render fix).** `HeldRampObligation::block` returns whether a free block
  entry discharged an owed obligation (U2 fold 2: whether a write freed a gain held through the
  previous block, owed or not); `FaderEventDriver::new` then reports a hold that ended
  before the block (`hold_until(node(0) - 1)`), which releases the cell's hold. One driver, so
  the bank, the fused bank and the scalar oracle all take it.
  `a_live_write_that_frees_a_held_lane_releases_its_hold` (a 4,000-sample unmute ended by a
  window-0 unmute at 3,200 on a `-30 -> 0` dB fade: every completion sample from 3,263 is the
  curve's gain; quanta 128, 100, 64 and the oracle). Red on the code before the fix.
- **#1648b MAJOR (required gate).** Already fixed by #1648b's comparison with the stage's gain.
  `a_ride_gained_on_a_carried_static_fader_jumps_from_the_stages_gain` (predecessor static at
  `-20` dB; the carried successor adds a ride flat at `0` dB; Simd8 and Simd4: the swap block
  ramps, and every block after the jump equals the successor rendered fresh). Red with the
  adoption compared with the cell's seed.
- **R17-8 (render fix).** `apply` pushes a seek `Set` that lands exactly at `E` and discharges the
  obligation; the cut then makes no move. `a_seek_set_exactly_at_the_unmute_ramps_end_sets_the_lane`
  (behind a latent insert, the seek and the unmute's end both at `3,200 + a(n)`, mid-block at
  quanta 128, 100 and 64, and the oracle: one `Set` per channel at `E` in the probe, the static
  plan's bits from `E`). Red without the rule.
- **R17-9 (render change).** `FaderCellCursor::cut` lands the cell at `E` through `CellEvents` on
  a one-sample span whose seek step is at its offset 0, in `SeekMode::Ramp`: #1637's own landing
  (`target_at(E, J)`, the curve at `E + J - 1`, capped before the next discontinuity; commit and
  ramp end). Allocation-free and bounded: one event, one binary search of the discontinuities and
  one curve read. Its reach is the fader jump `J`, the reach a `Jump` already has; the fader has
  no filter order rule, so `ORDER_REACH` (the input filters' 128 samples) is not touched.
  Moving curves: `grid_events_after_the_unmute_ramps_end_do_not_cut_the_move` (updated: the
  target is now `curve(E + J - 1)`; red on the pre-R17-9 move, which aimed at `curve(1,023)`), and
  the moving cases of the MAJOR-2, MINOR-11 and MAJOR-4 tests. Flat curves keep their bits: gate
  8 (a)-(d) pass unchanged.
- **MINOR-7 (counters): deleted.** `LaneFaderAutomation::{refused_events, operations_stopped}`,
  the driver's per-lane `refused` array (and its lane >= 8 drop) and `settle_fader_events` are
  gone. Each counted condition is unreachable and fails a `debug_assert` first (in a debug test
  build the process aborts before a count could be read), so no test can drive a count above 0;
  nothing read them. A refused event keeps the stage's target and fails its `debug_assert`; a
  stage that stops pulling fails its own. This withdraws D1's "adds one to a render counter",
  departure 6 of the batch 17 amendment, constraint 6 of the batch 13b amendment and #1647b's
  departure 1.
- **MINOR-9.** `a_written_jump_length_reaches_the_next_jump_not_the_one_in_flight` also writes at
  2,944, the entry of the block holding the second step. Red with the length read one block
  late.
- **MINOR-10.** `the_scalar_oracle_renders_the_banks_fader_events`: track 0 sits behind a
  true-peak limiter and every ride ends by 6,000, and the bank prepared between render calls
  must both fuse (the fused path with operations) and fall back. Red with the oracle's
  `set_input_arrival` ignored.
- **MINOR-11.** `a_ramp_that_ends_muted_keeps_the_obligation_and_a_hold_with_nothing_remembered_does_not_move`
  (constraint 4: a 300-sample mute at 0, the later unmute's end lands; departure 3: a hold with
  nothing remembered renders the free lane from `E`); red with a muted entry discharging, and red
  with a move at every hold end. `a_left_only_unmute_moves_the_left_lane_alone` (dual mono); red
  with the left channel's hold read for both.
- **MINOR-12.** `docs/SESSION_SCHEMA_V1.md` names the held-ramp exception to "set exactly where a
  seek reaches the fader". Gate 8 (c) and (d) no longer claim "red if a grid event cuts the move"
  (their curves are flat after `E`; (e) gates it). The C ABI gate 4 doc says the raised-floor leg
  lands the seek through the successor's step (#1647b departure 3).
- **NITs.** `block` cuts at a due entry only when a move is owed (`due && owed`), so an entry
  with nothing owed may fuse. The probe records the move (a `Jump` at `E`). A const assert holds
  every `BankWidth::ALL` width within `MAXIMUM_BANK_LANES` (and so `MAXIMUM_FADER_CELLS`). #1646's
  "Measured" paragraph is re-measured in a note under it. Gate 8 (e)'s text is amended above.

**Measured (x86-64, AVX2).** `lane_slot_bytes` 240 -> 224 (the two counters), by
`fader_automation_sizes()`; the one-lane charge is `68 + 64 + 8 × 224 + 120 = 2,044` bytes. No
fixture moved.

**Departures.** MAJOR-1 has no gate (above). MINOR-7 deletes the counters rather than reading
them. Out-of-path edits: `.github/ISSUE_SPECS/1646-prepare-stored-fader-automation.md` (the
re-measured note). No edit under `crates/engine/src/realtime/`, `crates/automation` or
`crates/builtins`.

**Checks (x86-64 AVX2).** `cargo test --locked -p automation -p graph -p builtins -p
builtins-compiler --features builtins-compiler/test-support,graph/test-support`: 388 pass.
`-p host-core --all-features`: 555 pass (`stored_fader_automation` 26; 23 without features).
`-p capi`: 206 pass. `-p host-web --features host-web/test-support --lib --test
control_parity`: 173 pass. `-p control-plane --all-features`: 24 pass. `realtime-policy`: ok, 160
regions in 33 files (unchanged). fmt; clippy `-D warnings` (builtins-compiler, host-core, capi;
all targets, all features); no-feature `cargo check --all-targets` on the same: clean.
`./target/release/audit capi`: `pcm_digest` `bffc21e857b5e134` (unchanged), 0 violations, 0
allocations.

## U2 fold 2 (batch 17)

The U2 fold's review (MAJOR-A, MINOR-B, a NIT) and root's ruling on D1.2, folded on
`codex/controls-batch-17k` over `6fcd46e01`. Uncommitted for the coordinator.

- **MAJOR-A (render fix: the held-through release).** R17-10 released the cell's hold only when a
  move was owed, so a live write that freed the lane with nothing owed left the cell holding (D1.2's
  report of the ended ramp): later grid events became `RememberGain` on a free stage and the lane
  stayed stale. `HeldRampObligation` now records `held_through` (the stage held the gain through the
  last sample of the block just rendered); `block` returns a release at any free block entry after
  such a block with no move due, owed or not, and the driver releases the cell's hold there
  (`hold_until(node(0) - 1)`). A hold that ended by itself (inside the previous block, or on its last
  sample: `due`) needs no release. The other places that copy the stage's hold into the cell (the
  held branch of `offsets_jump`, the adoption reposition) take the same release: both copy a stage
  ramp, which either ends inside the block (the cell's hold ends with it) or holds through the
  block's end (and so is released at the next free entry). One driver: the bank, the fused bank and
  the scalar oracle all take it. Gate
  `a_live_write_that_frees_a_lane_with_nothing_owed_releases_its_hold` (prepared muted; curve flat at
  `-30` dB over `[0, 2_000]`, then a fade to `0` dB at 10,000; a live unmute of 4,000 at 0 and of
  window 0 at 1,024: every completion sample from 1,087 is the curve's gain; quanta 128 and 64 and
  the oracle). ~~Red on `6fcd46e01` (completion sample 2,047 off the curve), checked once.~~
  Corrected in U2 fold 3: this claim does not hold for the gate as committed. With D1.2 removed
  nothing gave the cell a hold of the ended ramp in that case, so the gate stayed green under the
  mutation "release only when owed" (the fold-2 verifier's finding); it was replaced (U2 fold 3,
  F2).
- **D1.2: D1.2 was observable (P3) and produced a worse output; removed by root decision
  B17-R11.** With D1.2 an unchanged jump inside the hold becomes an owed move at `E`, and the lane
  departs from the never-muted reference; without it the lane equals the never-muted session from
  `E` (gate `an_unchanged_jump_inside_the_hold_owes_no_move`, U2 fold 3, F3). The fold-2 argument
  that D1.2 was unobservable is withdrawn. Removed with it:
  `LaneFaderAutomation::report_held_ramp`, `report_held_ramps`, the drain's held marks
  (`drain_fader_controls_marking`, folded back into `drain_fader_controls`), the scalar oracle's
  marks and the then unused `selector_channels`. This withdraws constraint 1's report (batch 13b
  amendment) and the "D1.2 is kept" of the U2 fold above.
- **MINOR-B.** Each unreachable `debug_assert` condition in the driver carries a one-line proof and
  its release action (a hold or a skip; no panic; no NaN reaches the stage): `read_offsets`'s
  offsets-jump refusal, `cut`'s span, the driver's missing program cell, its span / slot / `hold_end`
  refusal, `apply`'s push and moving cut, and the three `process_with` stop sites.
- **NIT.** D1's render counter, the #1647b driver bullet, #1647b departure 1 and the #1647c
  "No new `debug_assert`" bullet now carry a superseded note (the counters were deleted by the U2
  fold's MINOR-7).

**Measured (x86-64, AVX2).** `lane_slot_bytes` 224 (unchanged: the new flag fits the padding). No
fixture moved; `pcm_digest` `bffc21e857b5e134` unchanged.

**Checks (x86-64 AVX2).** `cargo test --locked -p automation -p graph -p builtins -p
builtins-compiler --features builtins-compiler/test-support,graph/test-support`: 388 pass.
`-p host-core --all-features`: 556 pass (`stored_fader_automation` 27; 24 without features).
`-p capi`: 206 pass. `-p host-web --features host-web/test-support --lib --test control_parity`:
173 pass. `realtime-policy`: ok, 160 regions in 33 files (unchanged). fmt; clippy `-D warnings`
(builtins-compiler, host-core; all targets, all features); no-feature `cargo check --all-targets`:
clean. `./target/release/audit capi`: `pcm_digest` `bffc21e857b5e134`, 0 violations, 0
allocations. No edit under `crates/engine/src/realtime/`, `crates/automation` or `crates/builtins`.

**Attempt count (AGENTS.md five-attempt rule).** #1647 is at attempt 3 of 5: the implementation
(#1647a-c), the U2 fold (59ca8e3b4) and U2 fold 2 (this section). A failed verification of fold 2
goes to root before attempt 4.

## U2 fold 3 (batch 17): attempt 4 of 5

The fold-2 verification's findings (F1-F3) and root decision B17-R11, folded on
`codex/controls-batch-17k` over `2d08d9142`. Root-approved scope only. Uncommitted for the
coordinator.

- **F1 (render fix: a regression from fold 2).** The held-through release (R17-10) overwrote a
  hold that `offsets_jump` set at the same block entry: a live unmute of window 0 and an `S`
  write at one entry cut the offsets jump's ramp at the next grid point. The free branch of
  `AutomatedFaderCell::offsets_jump` now clears `obligation.held_through` after the stage takes
  the operation; the jump's hold is newer than the ramp the write ended. A successor's carried
  record goes through the same `offsets_jump` call at its first block (`read_offsets`), with the
  carried obligation, so it takes the same treatment. Gate
  `an_offsets_jump_at_the_entry_that_frees_the_lane_keeps_its_ramp` (the verifier's P2):
  prepared muted, flat `-30` dB to 1,100 then a fade; unmute of window 0 and `S = -6` dB with a
  480-sample ramp at 1,024, with no unmute in progress and with a 4,000-sample unmute from 0;
  from 1,024 the lane equals the never-muted twin that takes the same write; quanta 128, 64, 37;
  bank at eight and four lanes, the fused path at both widths, and the scalar oracle. Red on
  `86271e096`'s logic (the one line reverted), checked once.
- **F2 (void gate replaced).** `a_live_write_that_frees_a_lane_with_nothing_owed_releases_its_hold`
  is now the verifier's P1b at quantum 37: an unmute of 4,000 at 0, an `S` write of `-6` dB (ramp
  200) at 962 under it (the held branch: the cell holds to 3,999), and an unmute of window 0 at
  999; no grid point lies between them, so nothing is owed. Every completion sample from 1,087 is
  the composed curve; both lanes, and a left-only form whose right lane stays muted; all five
  render forms. Red under the mutation "release only when owed", checked once.
- **F3 (B17-R11).** Gate `an_unchanged_jump_inside_the_hold_owes_no_move` (the verifier's P3):
  jump length 16; curve flat `-20` dB to 1,024, linear `-21` to `-13` dB over `[1_024, 1_144]`,
  flat `-13` after; prepared muted; unmute of window 1,060 at 0. From `E = 1_060` both lanes equal
  the never-muted twin; quanta 128, 64, 37; all five forms. Red with D1.2 restored (the hunks of
  the verifier's `lib.rs.d12`, ported to this tree), checked once. The U2 fold 2 record above now
  states the D1.2 decision and corrects the replaced gate's "Red on `6fcd46e01`" line.

The verifier's other probes (P1, P4, P5) are not committed: P1b covers P1's case, and P4 and P5
pass with no mutation named that they alone catch.

**Checks (x86-64 AVX2).** `cargo test --locked -p builtins-compiler --features test-support`:
86 pass. `-p host-core --all-features`: 561 pass (`stored_fader_automation` 30). `-p
control-plane --all-features`: 25 pass. `-p capi`: 220 pass, 1 ignored. `-p host-web --features
host-web/test-support --lib --test control_parity`: 175 pass. `realtime-policy`: ok, 160 regions
in 33 files. fmt; clippy `-D warnings` (builtins-compiler, host-core, capi; all targets, all
features); no-feature `cargo check --all-targets` of the same: clean. `./target/release/audit
capi`: `pcm_digest` `bffc21e857b5e134`, 0 violations, 0 allocations, 0 syscalls. No fixture or
byte moved. No edit under `crates/engine/src/realtime/`.

**Attempt count (AGENTS.md five-attempt rule).** #1647 is at attempt 4 of 5: the implementation
(#1647a-c), the U2 fold, U2 fold 2 and U2 fold 3 (this section). A failed verification of this
attempt leaves one attempt.
