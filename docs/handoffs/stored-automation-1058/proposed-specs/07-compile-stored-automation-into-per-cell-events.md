# Compile stored automation into per-cell events in node time

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answers A1 (A1.1, A1.4, A1.7), A4 and A5, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

A new leaf crate, `automation`, turns an automated cell's segment table into the events render
applies, and nothing else. Given one block's node-time mapping, it yields the cell's events in
offset order: grid retargets on a fixed node-time grid, jumps at discontinuities, and an exact set
at a seek. Every value is computed in scalar `f64` through `crates/math` and rounded once to `f32`,
so the same table and the same mapping give the same events, bit for bit, on x86-64, AArch64 and
wasm, at any quantum, wherever a seek lands. It allocates nothing after its builder. No stage uses
it yet; drafts 09a and 09b are its first users.

## Context

- **The table.** `Automation { id, target, segments }` with `AutomationSegment { shape,
  start_sample: u64, end_sample: u64, start_value: f32, end_value: f32, unit }` and the shapes
  `step`, `linear`, `exponential` (`crates/session/src/model.rs:874-955`). Session validation
  orders segments and refuses overlap (`crates/session/src/validate.rs:972-1000`); draft 01 adds
  one entry per lane, the hold rule and discontinuities at least 64 samples apart.
- **The evaluation law** (README A1.4): `linear` `v0 + (v1 - v0)·x`, `exponential`
  `v0·(v1/v0)^x`, `step` `v0`, with `x = (t - t0)/(t1 - t0)`; these are the Web Audio ramp formulas.
  The hold rule holds the first `start_value` before the first segment and a segment's `end_value`
  after it and in a gap; a `step` holds `start_value` on `[start, end)`.
- **Deterministic math.** `crates/math` is vendored libm with no target-conditional path
  (`crates/math/src/lib.rs:1-30`; `pow` at `:105-109`), `no_std`, and `clippy.toml` bans platform
  transcendentals (`clippy.toml:1-40`). Rust does not contract scalar `f64` arithmetic.
- **Where ramps complete.** The fader kernel assigns the target exactly on the ramp's last frame
  (`crates/lane/src/kernels/builtins.rs:165-170`, `:184-202`), so a ramp of `L` samples started at
  sample `τ` equals the target at `τ + L - 1`. The EQ and input-filter coefficient ramps reach the
  target at `A + 64` (`docs/EFFECT_CONTRACT_V1.md:165-166`), one sample later.
- **Node time** (README A1.3): a node at render sample `r` reads source-read sample
  `s = r + ΣP - a(n)`, and its node time is `τ(r) = timeline(s)`. Draft 04a's
  `timeline_block` gives one node block's mapping: a start value and at most one step (a seek).
  This crate cannot depend on `source` (D1), so it takes the mapping as its own plain type, and
  draft 09b fills it from draft 04a's reader.
- **The cross-target corpus.** Gate G5 digests every case on x86-64, AArch64 and wasm and compares
  with one set of pins (`tools/wasm-gate-corpus/src/lib.rs:1-45`). Crates delegate scalar cases
  through their own `corpus` module and pins (`math::corpus`, `:55`, `:118`); new blocks are
  appended at the end (`:34-45`) and the case decoder fixes the order (`case_of`, `:444-511`); a
  scalar case is not width dependent (`is_width_dependent`, `:513-529`). The native owner is
  `tools/wasm-gates/tests/g5_native_corpus.rs`; the wasm leg is `scripts/run-wasm-gates.sh`; the
  AArch64 release job runs G5 unfiltered (`scripts/run-aarch64-tests.sh:12-17`).
- **Realtime marking.** Render-reachable code sits between `REALTIME_POLICY_BEGIN` and
  `REALTIME_POLICY_END`; `scripts/check-realtime-policy.sh` scans every marked file and holds floors
  of 25 files and 89 regions, which the change that adds a marker raises (`:66-74`); the forbidden
  forms inside a region include `.expect(` and `.unwrap(` (`:76-77`).
- **Crate naming.** The package, the lib and the directory are `automation`
  (AGENTS.md; `scripts/check-workspace-policy.sh:158-161`).

## Decisions frozen for this slice

- **D1. The crate.** `crates/automation`, package and lib `automation`, `#![no_std]` with `alloc`
  for the builder only, dependent on `math` alone. A workspace member and a workspace dependency.
- **D2. Types.**
  - `Shape { Step, Linear, Exponential }`.
  - `Segment { start: u64, end: u64, start_value: f32, end_value: f32, shape: Shape }`, 32 bytes
    (a `const` assertion).
  - `Completion { End, After }`: a ramp of `n` samples started at node time `τ` completes at
    `τ + n - 1` (`End`, the fader, matrix, trim and effect ramps) or `τ + n` (`After`, the EQ and
    input-filter coefficient ramps). For `n = 0` both complete at `τ`.
  - `CellProgram`: its segment slice, grid period `G`, grid ramp `L`, a jump-length key,
    `Completion`, and `no_restart` (a target whose ramp cannot restart while one runs: the delay
    time). `G` is 64, or the cell's ramp length `L` when that is larger (128 for the delay time); it
    is always a multiple of 64.
  - `JumpLength { Word(JumpWord), Fixed(u32) }`, the jump-length key, with
    `JumpWord { Fader, Mute, Pan }`. A builtin strip row stores which word of draft 12's plan cell
    it reads (fader and trim: `Fader`; mute and polarity: `Mute`; pan and matrix: `Pan`). A filter
    target stores `Fixed(64)` and an effect parameter `Fixed(smoothing_samples)`. The program never
    stores a copied jump length `J` for a word key: `J` is read from the plan cell, so a
    `control_smoothing` edit reaches the next jump with no rebuild (draft 12).
  - `CellState`: cursor, current target bits, the node time at which the ramp in flight on the
    lane completes, a held flag, and the last seek boundary. At most 48 bytes (a `const` assertion).
- **D3. `value_at(t: i64) -> f32`.** The hold rule over the segment containing `t`. Inside a
  `linear` or `exponential` segment, `x = (t - t0) as f64 / (t1 - t0) as f64` (both differences are
  exact below `2^53`); `linear` is `v0 + (v1 - v0)·x` in `f64`; `exponential` is
  `v0·math::pow(v1/v0, x)` in `f64`. One rounding to `f32` at the end. No lane math, no platform
  math.
- **D4. Events of one block, pulled.** `CellEvents::new(program, &mut state, mapping, mode,
  jump_lengths)` returns an iterator of `Event { offset: u32, value: f32, ramp: u32,
  kind: Grid | Jump | Set, held: bool }` in increasing offset, with no buffer. `mapping` is a
  `NodeSpan { start: i64, step: Option<(u32, i64)>, frames: u32 }`: the node time at offset 0, the offset and node time of
  a seek step if the block has one, and the block length. `mode` is `SeekMode::Exact` (builtin
  cells) or `SeekMode::Ramp` (effect cells). Rules, at most one event per offset, priority
  `Set > Jump > Grid`. `jump_lengths: [u32; 3]` is the block's `fader`, `mute` and `pan` words of
  draft 12's plan cell; `J` below is the key resolved against it (a `Word` key) or the `Fixed`
  length:
  - **Grid.** At each offset whose node time `τ` is a multiple of `G`, a retarget over `L` to
    `value_at(completion(τ, L))`.
  - **Jump.** At each offset whose node time is a discontinuity of the table, a retarget over `J`
    to `value_at(completion(τ, J))`. A `no_restart` cell whose ramp is in flight takes the jump at
    the ramp's last sample plus one instead.
  - **Held.** A grid event whose completion is before the end of the ramp in flight on the lane is
    emitted with `held = true`: the stage remembers the value and does not retarget, and the cell's
    current target does not change. The caller reports ramps it starts itself (a mute ramp, a live
    record's ramp) through `CellState::hold_until(node_time)`.
  - **Set.** At a seek step, `Exact` emits `Set` (no ramp) to `value_at(τ)` and `Ramp` emits a
    `Jump` over `J` to `value_at(completion(τ, J))`. The step may be at offset 0: draft 04a's reader
    reports `step = (0, t)` when the seek's source-read block starts the node block, and the cell
    then emits its `Set` (or ramped `Jump`) at offset 0. So every seek that reaches a block yields
    its event, whatever its offset. The cursor is repositioned first by binary
    search over the segment table (`⌈log2(n + 1)⌉` compares). The same repositioning serves an
    adoption (`CellState::reposition(t)`).
  - **Change only.** A `Grid` or `Jump` whose value has the current target's bits emits nothing.
    `Set` always emits.
  - The cursor advances one segment per boundary crossed and never moves backwards except at a seek
    or a reposition.
  - Bound: at most `⌈q/G⌉ + 1` grid events, `⌈q/64⌉ + 1` jumps (draft 01's spacing rule) and one
    seek: `2⌈q/64⌉ + 3` per cell and block.
- **D5. Builder.** `ProgramBuilder` takes neutral cell descriptions (segments and D2's parameters;
  no session type) and reports the exact bytes it will allocate (`bytes(&cells) -> u64`) before it
  allocates them with fallible reservations. It refuses, typed, a description that breaks D2's
  invariants (segments out of order or overlapping; a `G` that is not a multiple of 64 or that
  differs from the larger of 64 and `L`; a non-positive exponential value). Preparation (draft
  09a) charges the report.
- **D6. G5 corpus.** `automation::corpus` holds a fixed curve set: a linear dB fade over 48,000
  samples, an exponential 20 Hz to 20 kHz sweep, steps, gaps, one entry with a seek, and a cell
  with `no_restart`. Each case runs D4 over a fixed sequence of node-time mappings and digests every
  event's offset, kind, ramp and value bits. The case block is appended at the end of
  `tools/wasm-gate-corpus`'s case list, scalar (not width dependent), with its pins in the
  `automation` crate, and runs on the x86-64, wasm and AArch64 legs.
- **D7. Realtime marking.** Event generation and `value_at` sit inside realtime policy regions;
  the floors in `scripts/check-realtime-policy.sh` rise by the file and regions this adds. The
  builder is outside them.
- **D8. The acked-batch question.** The crate has no queue. It reads a table and writes a fixed
  state. Nothing can be acknowledged and dropped.

## Deliverables

1. The crate (`Cargo.toml`, `src/lib.rs`, `src/corpus.rs`) and the workspace entries.
2. Unit tests and the allocation test in `crates/automation/tests/`.
3. D6's delegation in `tools/wasm-gate-corpus/src/lib.rs` and its manifest, and the G5 test's
   layout check.
4. D7's floor change.

## Authorized paths

- `crates/automation/` (new)
- `Cargo.toml` (workspace member and dependency), `Cargo.lock`
- `tools/wasm-gate-corpus/src/lib.rs`, `tools/wasm-gate-corpus/Cargo.toml` (one appended case
  block), `tools/wasm-gates/tests/g5_native_corpus.rs` (the layout check for the appended block)
- `scripts/check-realtime-policy.sh` (the two floors only)

## Non-goals

- Reading a session, a descriptor or a plan; mapping targets to cells (drafts 09a, 09b and later).
- Any stage operation or render wiring (drafts 08, 09a and 09b).
- Vector evaluation. Events are sparse control arithmetic; vector lane math diverges on AArch64
  release builds (LANE-3, `docs/TARGET_MATRIX.md:163-167`), so evaluation stays scalar `f64`
  (README A1.7). This is the reason the no-scalar rule asks for.

## Hazards

- **Corpus order is part of the pin.** The block goes on the end; no existing index moves. The G5
  layout test checks it.
- **Exponential near the edges.** `v1/v0` is positive by validation (draft 03a for effects, draft 02
  for builtins); the builder still refuses a non-positive value so the evaluator has no NaN path.
- **The grid in node time, not render time.** A grid on render samples would make the event values
  depend on where a seek landed when `q` is not a multiple of 64 (the README's blocker B1). The
  quantum-100 gate defends it.

## Objective gates

1. **Evaluation** (`crates/automation/tests/evaluate.rs`, new). The hold rule before, inside, after
   and between segments; a step's end; `linear` at `x = 0`, `0.5`, `1`; `exponential` equal, bit for
   bit, to `(v0 as f64 * math::pow(v1 as f64 / v0 as f64, x)) as f32` with `x` computed as D3 says,
   and different from the same formula in `f32` on at least one fixed input.
2. **Event rules** (`crates/automation/tests/events.rs`, new). A grid event at every node-time
   multiple of 64 and none elsewhere; a jump at an off-grid discontinuity at its exact offset; `End`
   and `After` completions; change only (a flat curve emits nothing after its first event); a held
   event inside a reported ramp, and the grid resuming after it; a `no_restart` jump deferred to the
   end of its ramp; `Set` at a seek step for `Exact` and a ramped `Jump` for `Ramp`, at a mid-block
  offset and at offset 0; a `Word` key's jump ramps over the word passed for the block, so two
  blocks with different words give two jump lengths.
3. **Bound.** An adversarial table (a discontinuity every 64 samples, every segment moving) and a
   seek in the block give at most `2⌈q/64⌉ + 3` events per block at `q` in {64, 100, 128, 1024}.
4. **Quantum and seek invariance.** The same table rendered through mappings of quantum 128 and
   quantum 100, and with one seek to the same timeline sample landing at two different render
   samples, gives the same sequence of `(node time, kind, ramp, value bits)` events from the first
   grid sample after the seek on.
5. **No allocation.** Generating events for 10,000 blocks allocates and frees nothing
   (`bench_support::alloc` thread counters after warming). The builder's `bytes` report equals the
   bytes its build leaves live (`requested_bytes - released_bytes` over the build).
6. **G5.** The appended cases' digests match their pins natively and on the wasm and AArch64 legs.
7. **Policy.** `bash scripts/check-realtime-policy.sh` and `bash scripts/check-workspace-policy.sh`
   accept the crate.
8. **Commands:**
   - `cargo test --locked -p automation`
   - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
   - `cargo build --locked --release -p wasm-gates && bash scripts/run-wasm-gates.sh --without-v8-spill --without-native`
   - `bash scripts/run-aarch64-tests.sh release` (the `aarch64-release` job)
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`

## Test value

- Gate 1 turns red if a segment is evaluated in `f32`, with a platform `powf`, or with the hold rule
  on the wrong side of a boundary.
- Gate 2 turns red if the grid follows render samples, a jump snaps to the grid, a completion is
  off by one, a redundant retarget is emitted, a held event retargets the stage, a seek at offset 0
  emits nothing, or a jump uses a stored length instead of the block's word.
- Gate 3 turns red if a pathological table makes the per-block work grow past the bound A1.7
  states.
- Gate 4 turns red if event values depend on the quantum or on the render sample a seek landed at
  (blocker B1).
- Gate 5 turns red if event generation allocates, or if the builder under-reports what preparation
  charges.
- Gate 6 turns red if any target computes a different event bit, the only cross-target owner of
  this claim.

## Dependencies

- Draft 01 *Validate stored automation lanes in the session crate and state the hold rule*.
- Batch: R1, with drafts 09a *Prepare stored fader automation and render it flat* and 09b *Render
  moving stored fader automation, seeks and latency*, its first users. The key names the words of
  draft 12's plan cell, but this crate reads no cell: the caller passes the words, so draft 12 is
  not a dependency.
