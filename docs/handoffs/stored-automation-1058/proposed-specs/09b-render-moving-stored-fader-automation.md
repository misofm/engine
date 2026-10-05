# Render moving stored fader automation, seeks and latency

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.3-A1.5, A1.7) and A5, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A producer's fader ride is heard on every platform. On the C ABI and in the browser, each automated
fader lane prepared by draft 09a follows its curve in node time: it reaches the curve's value at
each ramp's completion sample on a fixed 64-sample grid, takes each discontinuity at its exact
sample over the session's fader ramp length, lines up with the audio it acts on through every
latent insert, and is set exactly where a session seek reaches the fader. The output does not
depend on the quantum or on where a seek landed. A session with no stored automation renders
exactly as before. The fader row leaves the classifier's mask in draft 10a, in the same batch.

## Context

- **Where the fader renders.** `FaderBankProcessor` (`crates/builtins-compiler/src/lib.rs:715-723`)
  drains its lanes' live records, then processes the bank (`:738-752`); the fused fader-and-matrix
  processor does the same and fuses only when nothing ramps (`:1198-1219`). Graph runs a strip's
  builtin chain with the block's first sample (`crates/graph/src/runtime.rs:3046-3048`) through
  `GraphPreparedBuiltinBankProcessor::process` (`crates/graph/src/lib.rs:1347-1353`). The graph
  executor calls the source set at block entry (`crates/graph/src/lib.rs:3254-3256`), where draft 04
  advances the timeline.
- **What draft 09a leaves.** Each automated lane has a cell program (draft 07) with its `a(n)`, its
  grid and its jump-length key (the `fader` word of draft 12's plan cell), an offsets cell when a
  VCA reaches it, and its event state in the fader bank stage. It starts at its prepared value, the
  curve at node time `-a(n)`, which is the entry's first `start_value`.
- **What draft 12 leaves.** The graph reads the plan's jump-length cell once at block entry and holds
  its three words for the block; no stage reads them yet.
- **The documents.** `docs/SESSION_SCHEMA_V1.md:220-225` says the table renders nothing; the
  comments at `crates/session/src/validate.rs:812-822` and `:842-853` say the same;
  `docs/REALTIME_MEMORY.md:13` gives #1058 the rendering.
- **What this slice builds on.** Draft 04 (the timeline and its reader), 05 and 06a (the session
  seek on each host), 07 (events), 08 (operations inside a block, the held-gain rule).

## Decisions frozen for this slice

- **D1. Render.** Per block, after the live drain at block entry, the fader bank processor:
  1. reads, for each automated lane, its node-time mapping: graph passes the block's source-read
     sample and a timeline reader (draft 04) to the strip chain through a new defaulted method on
     `GraphPreparedBuiltinBankProcessor` (`process_timed`, which forwards to `process` by default),
     and the processor reads `timeline_block(s0 - a(n))` into draft 07's `NodeSpan`. The same call
     passes the block's jump lengths, the three words draft 12's read gives the graph for that
     block, which each cell's events resolve its `Word(Fader)` key against (draft 07 D4);
  2. tells each cell the end of a ramp in flight it did not start (a live mute ramp) through
     `hold_until`, from the stage's `remaining` (draft 08 D2);
  3. pulls each lane's events (draft 07, `SeekMode::Exact`) and merges them by offset across lanes
     with a fixed per-lane cursor (no buffer);
  4. converts each event value: `Grid` and `Jump` become `RetargetGain`, a held event
     `RememberGain`, `Set` `SetGain` (draft 08 D1), each gain composed as draft 09a D1;
  5. runs `process_with` (draft 08 D3). The fused fader-and-matrix path is taken only when no
     fader or matrix lane moves in the block.
  A gain the conversion refuses is unreachable (draft 02 bounds the curve to the fader domain and
  `vca_effective_db` clamps); it keeps the current target, adds one to a render counter and fails a
  `debug_assert`.
- **D2. First blocks.** A fresh plan starts every automated lane at its prepared value (draft 09a
  D2), so its first block emits no `Set`. A lane a successor adds or restarts takes a `Set` at its
  first block when the curve at that block's node time differs from its prepared bits (README A1.4,
  "New lanes"). Carrying a lane's event state across a swap is draft 10a's.
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

## Non-goals

- Preparation, prepared values, bytes and the mono collapse (draft 09a).
- Classifying fader automation edits and carrying event state across a swap (draft 10a); refusing
  browser live commands on automated lanes (draft 10b).
- The live VCA move that rewrites the offsets cell (draft 11). The `control_smoothing` edit path
  that writes the jump-length cell (draft 12).
- Mute, pan, matrix, input and effect rows (drafts 13a to 20). The live path's held-gain rule (#1054
  D10).
- A benchmark (drafts 24a and 24b).

## Hazards

- **Batch R1 only.** Until draft 10a is in, a rebuild for another reason sets each automated lane
  exactly at adoption (D2), and until drafts 10a and 10b are in, a live fader edit on an automated
  lane retargets it until the next event. They merge in the same batch, so `main` never holds this
  state.
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
   write of 960 to draft 12's cell at a block entry while the first jump ramps: that jump completes
   over 480 and the second ramps over 960, bit-identical to the stage driven directly with draft
   08's operations at those offsets and lengths.
3. **Latency.** Two tracks on one source, the first with a latent insert before the fader (a
   true-peak limiter, latency `λ`), each with a fader step at timeline 4,800: the step's event is at
   render sample `4,800 + a(n)` on each track's fader (a test-support event probe), so the two
   events are `λ` apart, and in the output both steps meet the source frame 4,800 they act on.
4. **Session seek.** After a session seek to `T` that reaches the fader at render sample `r`, the
   lane equals `checked_fader_gain(curve(T))` exactly from `r`, with no ramp: through draft 04's
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

- Draft 09a *Prepare stored fader automation and render it flat*, in the same push. It brings
  drafts 07, 08 and 12 and #1312 (the events, the timed operations, the jump-length cell and the
  live input D1 drains first).
- Draft 04 *Give every plan a timeline clock that seeks and carries like a source*.
- Draft 05 *Seek the timeline and every source in one C ABI call* and draft 06a *Seek the timeline
  and every source from the browser module export and the headless SDK* (the session seek on each
  host).
- Batch: R1, in one push with draft 09a, and in the same batch as draft 10a *Classify fader
  automation edits as carried rebuilds* and draft 10b *Refuse browser live commands on automated
  fader lanes*.
