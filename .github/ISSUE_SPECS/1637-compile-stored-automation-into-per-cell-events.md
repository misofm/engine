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
it yet; #1645's order check (`value_at`) and #1646 and #1647 are its first users.

## Context

- **The table.** `Automation { id, target, segments }` with `AutomationSegment { shape,
  start_sample: u64, end_sample: u64, start_value: f32, end_value: f32, unit }` and the shapes
  `step`, `linear`, `exponential` (`crates/session/src/model.rs:874-955`). Session validation
  orders segments and refuses overlap (`crates/session/src/validate.rs:972-1000`); #1626 adds
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
  `s = r + ΣP - a(n)`, and its node time is `τ(r) = timeline(s)`. #1640's
  `timeline_block` gives one node block's mapping: a start value and at most one step (a seek).
  This crate cannot depend on `source` (D1), so it takes the mapping as its own plain type, and
  #1647 fills it from #1640's reader.
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
    `JumpWord { Fader, Mute, Pan }`. A builtin strip row stores which word of #1639's plan cell
    it reads (fader and trim: `Fader`; mute and polarity: `Mute`; pan and matrix: `Pan`). A filter
    target stores `Fixed(64)` and an effect parameter `Fixed(smoothing_samples)`. The program never
    stores a copied jump length `J` for a word key: `J` is read from the plan cell, so a
    `control_smoothing` edit reaches the next jump with no rebuild (#1639).
  - `CellState`: cursor, current target bits, the node time at which the ramp in flight on the
    lane completes, a held flag, and the last seek boundary. At most 48 bytes (a `const` assertion).
- **D3. `value_at(segments: &[Segment], t: i64) -> f32`.** A free function of the crate that takes
  only a segment table, never a `CellProgram`: render calls it with a program's segment slice, and
  #1645 calls it with a table it builds from one session entry (`Segment` per
  `AutomationSegment`, the same fields), so preparation's check and render read one law. The hold
  rule over the segment containing `t`. Inside a
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
  #1639's plan cell; `J` below is the key resolved against it (a `Word` key) or the `Fixed`
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
    `Jump` over `J` to `value_at(completion(τ, J))`. The step may be at offset 0: #1640's reader
    reports `step = (0, t)` when the seek's source-read block starts the node block, and the cell
    then emits its `Set` (or ramped `Jump`) at offset 0. So every seek that reaches a block yields
    its event, whatever its offset. The cursor is repositioned first by binary
    search over the segment table (`⌈log2(n + 1)⌉` compares). The same repositioning serves an
    adoption (`CellState::reposition(t)`).
  - **Change only.** A `Grid` or `Jump` whose value has the current target's bits emits nothing.
    `Set` always emits.
  - The cursor advances one segment per boundary crossed and never moves backwards except at a seek
    or a reposition.
  - Bound: at most `⌈q/G⌉ + 1` grid events, `⌈q/64⌉ + 1` jumps (#1626's spacing rule) and one
    seek: `2⌈q/64⌉ + 3` per cell and block.
- **D5. Builder.** `ProgramBuilder` takes neutral cell descriptions (segments and D2's parameters;
  no session type) and reports the exact bytes it will allocate (`bytes(&cells) -> u64`) before it
  allocates them with fallible reservations. It refuses, typed, a description that breaks D2's
  invariants (segments out of order or overlapping; a `G` that is not a multiple of 64 or that
  differs from the larger of 64 and `L`; a non-positive exponential value). Preparation (
  #1646) charges the report.
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
  block), `tools/wasm-gates/tests/g5_native_corpus.rs` (the layout check for the appended block),
  `tools/wasm-gates/tests/g6_full_corpus_ftz.rs` (its `expected_rows`, `:188-192`, counts only math
  and delay cases as width-independent, `tools/wasm-gate-corpus/src/lib.rs:527-529`; the new
  scalar block joins that count)
- `scripts/check-realtime-policy.sh` (the two floors only)

## Non-goals

- Reading a session, a descriptor or a plan; mapping targets to cells (#1646, #1647 and later).
- Any stage operation or render wiring (#1638, #1646 and #1647).
- Vector evaluation. Events are sparse control arithmetic; vector lane math diverges on AArch64
  release builds (LANE-3, `docs/TARGET_MATRIX.md:163-167`), so evaluation stays scalar `f64`
  (README A1.7). This is the reason the no-scalar rule asks for.

## Hazards

- **Corpus order is part of the pin.** The block goes on the end; no existing index moves. The G5
  layout test checks it.
- **Exponential near the edges.** `v1/v0` is positive by validation (#1645 and #1566's row rule,
  `controls::automation_diagnostics`); the builder still refuses a non-positive value so the evaluator has no NaN path.
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
     `bash scripts/check-cross-targets.sh` (README F19, the iOS memset rule)
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

- #1626 *Validate stored automation lanes in the session crate and state the hold rule*.
- Batch: R1, with #1645 *Validate builtin automation targets against their rows at
  preparation* and #1646 *Prepare stored fader automation and render it flat* and #1647 *Render
  moving stored fader automation, seeks and latency*, its first users. The key names the words of
  #1639's plan cell, but this crate reads no cell: the caller passes the words, so #1639 is
  not a dependency.

## Amendment (controls coordinator, 2026-10-14)

Checked against `12d4a4f9d`.

- **Jump lengths follow the ramp rows.** Decision 17 gives every row a `LiveRampRow`
  (`crates/effect-contract/src/control.rs:70-102`); polarity ramps over twice `mute_ms`
  (`PolarityInvert`), and sends and VCAs have rows of their own. Replace D2's `JumpWord { Fader,
  Mute, Pan }` with the row's `LiveRampRow`, resolved per block against #1639's three key
  lengths by `controls::LiveRamps::for_row` (`crates/controls/src/live_ramps.rs`), so no row's rule
  is written twice. `Fixed(n)` stays for filter, EQ and effect cells.
- **Realtime policy.** D7 and gate 7: `scripts/check-realtime-policy.sh` is deleted; raise
  `marked_files_floor` and `marked_regions_floor` in `tools/realtime-policy/src/policy.rs` and run
  `./target/release/realtime_policy`. Authorized paths change to that file.
- **The table.** `Automation.target` is a `ControlAddress` (#1566); the crate reads no session type,
  so nothing else changes.
- **OQ1 = C** adds no evaluator work: the caller adds the offset after `value_at` (#1650 D6,
  #1658).

## Decision record: #1637 (batch 13b)

**Shipped.** `crates/automation` (package, lib and directory `automation`; `#![no_std]` + `alloc`
for the builder; depends on `math` only): `value_at`, `ProgramBuilder` (neutral `CellDescription`,
typed `BuildError`, `bytes()` report), `CellProgram`, `CellState`, `NodeSpan`, `CellEvents`, and
`automation::corpus` (six scalar G5 cases and their pins). The G5 block is appended after the
builtins tail in `tools/wasm-gate-corpus`, scalar (not width dependent); `g5_native_corpus.rs`
checks its layout and `g6_full_corpus_ftz.rs` counts it as width independent. Realtime regions
mark `value_at` (`evaluate.rs`) and the event generator (`events.rs`).

**Departures from the filed text (each a code-truth or correctness call).**

1. **The jump-length key.** The amendment replaces `JumpWord` with `LiveRampRow` resolved by
   `controls::LiveRamps::for_row`. `effect-contract` is `std` and pulls `engine` and `lane`
   (the AVX2-pinned crate), and `controls` pulls `session`; either would break D1 ("`math` alone").
   `JumpLength` is `Block | Fixed(u32)`: a `Block` cell takes one resolved `u32` per block
   (`CellEvents::new(..., jump)`). The caller resolves the cell's row through `for_row` (which owns
   the polarity doubling and every other row rule), so no row's rule is written here and the program
   still stores no copy of `J`. `jump_lengths: [u32; 3]` became that single `jump: u32`.
2. **A grid or jump ramp never aims past the next discontinuity.** D4 says a grid event aims at
   `value_at(completion(τ, L))`. Taken literally, a grid event up to 63 samples before an off-grid
   step aims at the post-step value, the step's own `Jump` then has the same target and change-only
   drops it, so the step lands on the grid's ramp and not at its exact offset (gate 2 cannot hold).
   The aim time is `min(completion, d - 1)` for the first discontinuity `d` in `(τ, completion]`;
   the `Jump` at `d` retargets from there. The same clamp applies to a `Jump` and a ramped seek
   landing. Test `jump_at_off_grid_discontinuity_at_its_exact_offset` is red without it.
3. **The discontinuity list is built and stored.** The cursor indexes a per-cell list of
   discontinuity times (value just before the time differs from the value at it, bit for bit; built
   by the builder, charged in `bytes`), not the raw segment boundaries. Scanning raw boundaries for
   "the next discontinuity" is unbounded when continuous segments are dense (one-sample segments);
   the list makes every lookup `O(1)` and the reposition one binary search.
4. **`no_restart` also holds grid events.** A jump off the grid starts a ramp that a later grid
   event (not completing before it) would restart; for a `no_restart` cell every grid event inside
   a ramp in flight is `held`. The jump itself is deferred to `completion + 1` (`End`) or
   `completion` (`After`), carried in `CellState` across blocks.
5. **Held bookkeeping.** `CellState::held` means "a held value is outstanding at the stage": the
   next retarget or held event emits even when its value equals the target, once, so the stage's
   remembered value is refreshed (otherwise a curve that leaves the target and returns inside a
   long ramp leaves a stale value). `CellState` is 32 bytes (limit 48). The spec's "last seek
   boundary" field has no user and is not stored.
6. **`NodeSpan::new` is fallible** (`StepOutsideBlock` when the step offset is not below `frames`),
   so a seek cannot be silently dropped in render. A fresh `CellState` has no target, so its first
   retarget always emits; `reposition(&program, t)` places the cursor only (target kept, in-flight
   ramp and deferral dropped).
7. **Builder limits** beyond D5: `start < end`, every time below `2^53` (the evaluator's exactness
   bound), finite values, at least one segment per cell. `value_at` on an empty table returns `0.0`
   and never panics.
8. **Realtime floors.** `scripts/check-realtime-policy.sh` is gone; `marked_files_floor` and
   `marked_regions_floor` in `tools/realtime-policy/src/policy.rs` were 28 and 129 but the tree
   held 29 and 141 before this change (the floors had fallen behind). They are now 31 and 143, the
   tree's counts with this crate's two files and two regions.

**Observation for successors.** A seek `Set` in `Exact` mode clears the in-flight ramp in the
state, so the stage must treat `Set` as an immediate assignment. `CellState::hold_until` takes the completion node time in the cell's own `Completion`
convention.

**Gates and tests.**

| Gate | Test (`cargo test --locked -p automation`) | Result |
|---|---|---|
| 1 Evaluation | `evaluate`: `hold_rule_before_inside_after_and_between`, `step_end_value_applies_only_from_its_end`, `linear_at_zero_half_and_one`, `exponential_is_scalar_f64_through_math_pow` (bit equal to the `f64` formula, differs from the `f32` one), `empty_table_is_zero` | pass |
| 2 Event rules | `events`: `grid_events_sit_on_node_time_multiples_of_64` (quantum 100, start -37), `jump_at_off_grid_discontinuity_at_its_exact_offset`, `end_and_after_completions`, `change_only`, `held_inside_a_reported_ramp_and_the_grid_resumes`, `a_held_value_back_at_the_target_still_emits_once`, `no_restart_jump_waits_for_the_ramp_to_end`, `set_at_a_seek_step_for_exact_and_a_ramped_jump_for_ramp` (offsets 0 and 37), `set_always_emits`, `a_block_keyed_jump_ramps_over_the_blocks_length`, `a_fixed_key_ignores_the_blocks_length`, `a_step_outside_the_block_is_refused` | pass |
| 3 Bound | `events_per_block_are_bounded` (a discontinuity every 64 samples, every segment moving, seeks mid-block, q in {64, 100, 128, 1024}, at most `2⌈q/64⌉ + 3`) | pass |
| 4 Invariance | `quantum_invariance` (128 vs 100, both modes), `seek_invariance` (one timeline sample reached at three render positions after three histories, both modes) | pass |
| 5 No allocation | `allocation::ten_thousand_blocks_allocate_nothing` (0 allocations, 0 frees, seeks every 97th block); `build::bytes_report_equals_the_live_bytes` (`requested - released == bytes()`, 3 reservations, 0 reallocations); `build::refusals_are_typed`, `build::discontinuities_are_the_steps_of_the_hold_rule` | pass |
| 6 G5 | `cargo test --release -p wasm-gates --test g5_native_corpus --features math/lane` (7 tests incl. `g5_delegated_cases_use_the_owning_crates_pins` for the block, `g5_native_digests_match_pins`); wasm leg `scripts/run-wasm-gates.sh --without-v8-spill --without-native`; `automation::corpus` self-check `corpus::cases_produce_events_and_match_their_pins` | native pass; wasm leg pass (`wasm gates: ok`, no mismatch) |
| 7 Policy | `./target/release/realtime_policy` (143 regions in 31 files), `scripts/check-workspace-policy.sh` | ok |

**AArch64** cannot run on this host. The pins are scalar `f64` through `math::pow` (no fused
operations, no lane type), generated on x86-64, and must match on the `aarch64-release` job.

**Measured.** `Segment` 32 bytes, `CellState` 32 bytes (assertion at most 48). The six corpus cases
yield 20+ events each (case 5, `no_restart`, 42).
