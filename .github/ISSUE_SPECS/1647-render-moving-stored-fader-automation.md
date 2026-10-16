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
