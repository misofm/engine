# Bank the full meter pass: seeded `f64` energy, counts and held peak

Research on `252622b6`, 2026-09-27. This is slice S3 of #943, and it replaces the
remaining scope of #884: counts, held peak and energy. **Owner ruling** (2026-09-26): "We shouldn't
leave scalar arithmetic where vector arithmetic is possible." Banked ALL-metric meters are wanted,
the `f64` energy sum included, and #943 ruling R4 is settled in favour of `f64` lanes.

- **Hard dependencies:** #943 (P1), and #949.
- **Class A.** No rendered bit moves, and every published meter word stays bit-identical. The energy
  sum keeps its exact per-lane, sample-serial order of additions; only the lanes run in parallel.
- **The research** is under "Research findings" at the end. The lane draft cites it.

## Amendments (adversarial verification, 2026-09-27; these override any conflicting text below)

The verification confirmed the design's exactness: widening is exact over all 2^32 bit patterns at
`Simd4` and `Simd8` natively and under V8 simd128; `add` and `mul` match scalar `f64` over an edge
pool and a million random pairs; no FMA contraction appears (no `vfmadd` in the linked fat-LTO
kernels; no `contract`, `fmuladd` or `fma` in the IR); and the differential saw 0 mismatches over
13.6 M `Simd8`, 6.8 M `Simd4` and 1.67 M `f32` snapshots, plus 27 M with two meters on one track.
Saving reproduced in process: `Simd8` 3.33-3.37 to 0.69-0.71 us per bank per block, `Simd4`
1.67-1.75 to 0.49, about 21 us per 64-track block; the row that moves is `console_meters`.
Evidence: `docs/handoffs/meters-2026-09-26/F64-VERIFY.md`.

1. **Item 7 contradicts M3's mixed control.** Every final member receives `meter: Some`, so a
   `SAMPLE_PEAK`-only meter would commit through `observe_input_banked` and never through #943's
   merge. Call `observe_input_banked` only when `block.meter.is_some() && accepts_banked_meter()`;
   otherwise make #943's call with `block.sample_peak`.
2. **Seed pollution.** A `SAMPLE_PEAK` meter answers `banked_seed` with its unused energy (`0.0`),
   and graph takes the first answer, so with `[SAMPLE_PEAK, ALL]` on one track the ALL meter
   committed only 128 of 512 blocks (exact, but the saving silently lost). `banked_seed` returns
   `None` unless `ENERGY_RMS` is selected. Add an M2 row for that pair expecting 8 x 64 ALL commits.
3. **#714.** The design respects #714's rule against observer state precomputed by another party:
   the seed is a `&self` read of each meter's own state, the kernel is a pure function of that seed
   and the block, and each meter re-checks the seed after its own preamble and commits its own lane,
   in binding order (`crates/graph/src/lib.rs:2141-2148`, `runtime.rs:3602`). #714's own spec
   (lines 250-253) lists across-track vector meter work as a successor gated on an owner ruling,
   which now exists. State in the brief that the owner's ruling supersedes #714 line 167 ("no new
   f64 lanes") while keeping its "no reassociation, per-lane sample order" requirement. Add a
   failure-boundary gate: when an observer fails mid-bank, the later meters never commit and their
   snapshots equal the declined arm.
4. **Pin the production lowering.** M5's census of the production pass must be a gate, not a
   record: callgraph rule 3 counts only `f32`, and graph is reached through `call_indirect`, so no
   required gate inspects graph code. Authorize `scripts/run-wasm-gates.sh`, and add a probe that
   calls the real `meter_block::<Simd4>` to `check_f64_lane_lowering`.
5. **Small gaps.** Skip the kernel when no lane returned a seed (a period shorter than the block
   would otherwise pay for a pass that commits nothing). In `#[inline(never)] bank_meter_pass`, take
   `frames` from the slice length with `.get(..)` rather than indexing `words[..frames * W]`, which
   keeps a panic path (measured panic-free and still vector). The `f32` counts are exact only up to
   2^24 frames per block: decline the pass above that. Tools crates name the `f64` types through
   `<Simd4 as Widen>::F64`, since the lane policy forbids `wide::` under `tools/`.

## Rulings (coordinator, 2026-09-27)

1. **#884 is closed as superseded** by this issue once it is filed.
2. **Meters with peak hold or decay are not left scalar for good.** Under the owner's rule, a held
   peak with hold or decay is a per-lane, sample-serial state machine that can run across lanes like
   the energy sum. It is out of this slice only to keep the slice closable, and it is filed as its
   own successor issue, #951, rather than accepted as a permanent scalar path.
3. **The seeded design and #714's rule** ("no observer state precomputed by another party") is a
   question for the adversarial verification to settle before implementation. The design seeds from
   each meter's own state and each meter accepts the result only on a bit-exact seed match; the
   verification must say whether that is the meter computing its own state through a shared kernel
   (acceptable) or another party computing it (not).
4. **The `f32`-to-`f64` widening** may come from the compiler's lowering, pinned by the wasm opcode
   check, rather than by loosening the lane policy.
5. **#883** (the scalar hold-0 shortcut) is closed as superseded when this issue lands; until then
   it stays open.

## Product outcome

After P1, a `SAMPLE_PEAK` meter merges one banked block partial. Every other meter still walks its
lane of the bank's resident block with the scalar loop, once per lane per block. That loop carries
the `f64` energy chain, the held-peak state machine and two counters.

The ALL-metric console meters cost about 31 µs per 64-track block today (#943 F2; recorded
`console_meters` delta +30.9 / +31.4 µs). Those are the native host-core console default, the
`console_meters` benchmark row and the web spectrum boot.

This slice runs one `Lane`-generic pass per bank per block. The pass computes every lane's:

- peak and counts, as order-free partials;
- energy, seeded from each meter's own running sum and accumulated sample by sample in `f64`
  lanes.

Each meter then commits its own lane after checking that the seed is bit for bit its own state.

Measured per 8-lane bank, in-process: 3.57-3.60 µs becomes 0.77-0.79 µs (R4). No in-situ number
exists yet, and none may be quoted as this issue's result.

## Root evidence

- **The scalar loops.** `observe_input` (`crates/builtins/src/lib.rs:4574`) splits a block at window
  boundaries. `observe_segment` (`:4810`, used for ALL) and `observe_selected_segment` (`:4858`)
  then run per lane and per sample:
  - `energy += f64::from(s) * f64::from(s)`;
  - the held three-way branch;
  - `clipped` and `sanitized`.
- **The dispatch.** `MeterObserver::observe_resident` (`crates/builtins-compiler/src/lib.rs:4690`) is
  called once per lane per block from graph's bank arm (`crates/graph/src/runtime.rs:3046-3090`).
- **Why #884's design fails.** It updated meter state in place. That state lives in W separate
  `MeterAccumulator`s behind W `dyn` observers (#943 F7). R2 gives the protocol that works: the
  seed is read, the pass computes, and each meter commits its own lane.
- **The numerical facts** (R3):
  - energy is the only order-sensitive statistic;
  - `f64(x)·f64(x)` is exact for every `f32` `x`;
  - reassociating the sum moves `energy`/`rms` bits by an ulp. Measured: the "zero-seeded partial
    plus seed" mutation goes red.

## Smallest closable slice (S3): the full banked pass at the resident final lane, permanent observers

### Invariants

- **I1. PCM.** No rendered bit moves.
- **I2. Meter words.** Every published `MeterSnapshot` field is bit-identical to the scalar path.
  This holds for every metric set, period, hold/decay, discontinuity, restart and reset pattern.
  Compare `sample_peak`, `held_peak`, `energy` and `rms` by `to_bits()`.
- **I3. Shape.** `[chains, slots]`, transposes, folds and redirects are identical with the pass on
  and off.
- **I4. Order.** The pass runs after the unit executed successfully and before its first observer.
  - The seed query is a pure `&self` read.
  - Observers still run in binding order, with the first-error short-circuit.
  - An observer that fails leaves every later observer's state untouched, because the pass only
    read it.
- **I5. Opt-in.** A unit runs the pass only if a final-slot observer accepts it.
  - `SAMPLE_PEAK`-only meters stay on P1's cheaper pass.
  - Meters with hold or decay never accept.
  - Spectrum capture and custom observers never accept.
- **I6. Energy is never merged from a partial.** A meter commits exactly one energy: the seeded,
  sample-serial result whose seed equals its own post-preamble `energy` bit for bit. There is no
  pairwise, block-partial or tree sum anywhere; that form is class B (R3).
- **I7. Realtime and layering.**
  - No allocation, lock or syscall.
  - No retained byte beyond one `bool` in `UnitIdentity`'s padding.
  - No `unsafe` in `graph`, and `graph`'s dependency list is unchanged.
  - Only `crates/lane` names `wide`.
  - No block-sized copy and no planar meter scratch (the owner's copy rule): the pass is a read.
- **I8. Dual-mono.**
  - Left comes from the left plane and right from the right plane.
  - Inactive lanes are never meter input.
  - The planes read are the final planes, after the collapse seam's dual suffix (#714).

### Interface contract

These are deltas on #943's interface, as amended. P1's names are used as P1 lands them.

1. **Lane kernel** (`crates/lane/src/kernels/builtins.rs`, beside P1's `meter_sample_peak_block`):

   ```rust
   pub struct MeterBlock<L: Widen> { pub peak: L, pub clipped: L, pub sanitized: L, pub energy: L::F64 }
   pub fn meter_block<L: Widen>(words: &[f32], frames: usize, energy: L::F64) -> MeterBlock<L>
   ```

   - Mark it `#[inline(always)]`, with `debug_assert!(words.len() >= frames * L::WIDTH)`.
   - Loop `for frame in words[..frames * L::WIDTH].chunks_exact(L::WIDTH)`.
   - `peak`, `clipped` and `sanitized` start at `L::zero()`. `one = L::splat(1.0)`.
   - **Frozen order, per frame:**
     1. `a = L::load(frame).abs()`
     2. `c = L::select(L::mask_and(a.ge(L::splat(f32::MIN_POSITIVE)), a.lt(L::splat(f32::INFINITY))), a, L::zero())`.
        This is P1's sanitised magnitude, and the validity test is the meter's `normal_or_zero`.
     3. `peak = L::max(c, peak)`. This is P1's step, in the UFCS form the lane policy requires.
     4. `sanitized = sanitized.add(one.andnot(a.eq(c)))`. The magnitude was replaced exactly for NaN,
        `±inf` and a nonzero subnormal. `±0.0` and normals keep `a == c`.
     5. `clipped = clipped.add(one.andnot(L::mask_not(c.ge(one))))`, which counts `c >= 1.0`.
     6. `w = c.widen(); energy = energy.add(w.mul(w))`. This is the only order-sensitive step: seeded,
        per lane, in frame order.
   - The doc states four things:
     - `peak` equals `meter_sample_peak_block(words, frames, L::zero())` bit for bit;
     - the counts are exact integers in `f32`, because a block never exceeds `2^24` frames (as
       `sanitize_gain_block` states);
     - `w * w` is exact;
     - the additions are in the scalar loop's order, from the scalar loop's seed.
2. **Block field.** `graph::GraphResidentObservationBlock` gains `pub meter:
   Option<GraphBankedMeterLane>`, a new public `Clone, Copy, Debug, PartialEq` struct. Every field is
   `[left, right]`:
   - `sample_peak: [f32; 2]`: the seeded-`+0.0` block peak of the sanitised magnitude.
   - `clipped: [u32; 2]` and `sanitized: [u32; 2]`: this block's counts.
   - `energy_seed: [f64; 2]`: the seed this lane's pass started from.
   - `energy: [f64; 2]`: the seeded, sample-serial sum after this block.

   It is `Some` only for a final-slot member of a unit whose full pass ran this block.
3. **Observer trait.** `graph::GraphRuntimeObserver` gains two methods:
   - `fn accepts_banked_meter(&self) -> bool { false }`. It is read at bind only, beside P1's
     `accepts_sample_peak`.
   - `fn banked_meter_seed(&self, _first_sample: u64, _frames: u32) -> Option<[f64; 2]> { None }`.
     - It is called at render, before any observer of the unit runs.
     - It must be pure and bounded.
     - It returns the `[left, right]` energy the observer's own scalar loop would start this block
       from, or `None` to decline.
4. **Bind-time flag.** `UnitIdentity` (`runtime.rs:2235`) gains `banked_meter: bool`, in the existing
   padding. After P1 this is 30 of 32 bytes.
   - It is true iff the unit is a `RuntimeUnit::Bank` and some final-slot member holds an observer
     whose `accepts_banked_meter()` is true.
   - Compute it beside P1's flag, with the same `checked_sub`.
5. **The pass**, in `Runtime::observe_unit`'s bank arm, before the member loop:
   - **When.** It runs iff `identity[index].banked_meter`, P1's `eligible`, `frames` is `Some`, and
     the test switch does not decline. When it runs, P1's peak pass does **not** run for this unit.
   - **Seeds.** Start from `let mut seeds = [[0.0_f64; 8]; 2]`. For each final lane `l < population`,
     walk member `final_start + l`'s observers in binding order. Take the first
     `banked_meter_seed(first_sample, frames)` that returns `Some(s)`, set `seeds[0][l] = s[0]` and
     `seeds[1][l] = s[1]`, and stop.
   - **Width.** Width dispatch goes in `#[inline(never)] fn bank_meter_pass` in `graph`, beside P1's
     `bank_sample_peak`. `BankWidth::Four` gives `lane::Simd4` and `Eight` gives `lane::Simd8`.
   - **Kernel calls.** It calls `meter_block` once per plane, on P1's
     `BankChain::final_output_block(frames)` words, seeded with `LaneF64::load(&seeds[p][..W])`.
   - **Results.** It stores the results into stack arrays: `[[f32; 8]; 2]` for peak, clipped and
     sanitized, and `[[f64; 8]; 2]` for energy. Counts become `u32` with `as`, which is exact.
   - **Hand-off.**
     - A final member at lane `l` passes `meter = Some(..lane l..)`, with `energy_seed` from `seeds`.
     - It also passes P1's `sample_peak = Some([peak[0][l], peak[1][l]])` from this pass, so
       `SAMPLE_PEAK` meters on the same unit keep P1's merge.
     - Every other member, and the controlled path's `observe_one`, passes `meter: None`.
6. **Meter accumulator** (`crates/builtins/src/lib.rs`, meter accumulator only):
   - **The block type.** `pub struct MeterBankedBlock { pub sample_peak: [f32; 2], pub clipped:
     [u64; 2], pub sanitized: [u64; 2], pub energy_seed: [f64; 2], pub energy: [f64; 2] }`.
   - **`pub fn banked_eligible(&self) -> bool`.** It is `!metrics.contains(HELD_PEAK) ||
     (peak_hold_frames == 0 && peak_decay_db_per_second == 0.0)`.
   - **`pub fn banked_seed(&self, first_sample: u64, frames: usize) -> Option<[f64; 2]>`.** It is pure.
     - It returns `None` unless `banked_eligible() && frames > 0`.
     - Let `discontinuous = start.is_some_and(|s| first_sample != s.saturating_add(u64::from(self.frames)))`.
     - Then `(now, seed) = if discontinuous { (0, [0.0, 0.0]) } else { (self.frames,
       [left.energy, right.energy]) }`.
     - It returns `None` if `frames > (period - now) as usize`, and `Some(seed)` otherwise.
     - This mirrors `observe_input`'s preamble exactly: a discontinuity zeroes both `frames` and
       `energy`, and `start == None` already has both at zero.
   - **`pub fn observe_input_banked(&mut self, input: MeterInput<'_>, first_sample: u64, banked:
     Option<MeterBankedBlock>) -> Result<(), MeterObservationError>`.**
     - **Preamble.** `observe_input`'s preamble runs unchanged: the overflow check, the discontinuity,
       then `start`.
     - **When it commits.** All of these must hold: `banked == Some(b)`, `banked_eligible()`,
       `len > 0`, and `len <= (period - self.frames) as usize`. If `metrics.contains(ENERGY_RMS)`,
       also `b.energy_seed[0].to_bits() == self.left.energy.to_bits()`, and likewise for the right.
     - **What it commits,** per channel, left then right:
       - `SAMPLE_PEAK`: `peak = if q > peak { q } else { peak }`.
       - `ENERGY_RMS`: `energy = b.energy[ch]`. This replaces the value and never adds to it.
       - `COUNTS`: `clipped`/`sanitized` `saturating_add` the block's counts, then
         `cumulative_clipped`/`cumulative_sanitized` do the same.
       - `HELD_PEAK`: `held = if q >= held { q } else { held }`. `hold_remaining` is untouched: it is
         `0` whenever the hold is 0.
     - Then add `len` to `frames` and `emit()` at `frames == period`.
     - Otherwise today's code runs unchanged, `settled_silence` included. Put the commit before
       `settled_silence` is computed.
7. **Meter observer** (`crates/builtins-compiler/src/lib.rs`, `MeterObserver` only):
   - `accepts_banked_meter` is `self.0.banked_eligible() && self.0.metrics() !=
     MeterMetricSet::SAMPLE_PEAK`.
   - `banked_meter_seed` forwards to `banked_seed`.
   - `observe_resident` calls `observe_input_banked` with the mapped block when `block.meter` is
     `Some`, and otherwise makes P1's call.
8. **Test support** (`cfg(any(test, feature = "test-support"))`):
   - `graph`: `test_only_set_bank_meter_declined(bool)`, which is the oracle arm. P1's pass still runs
     under its own flag.
   - `graph`: `test_only_bank_meter_passes() -> u64`, with a reset.
   - `builtins`: a counter of banked commits, with a reset.

### Authorized paths

- `crates/lane/src/kernels/builtins.rs` and the new `crates/lane/tests/meter_block.rs`.
- `crates/builtins/src/lib.rs` (meter accumulator only) and `crates/builtins/tests/meter.rs`.
- `crates/graph/src/lib.rs` and `crates/graph/src/runtime.rs`, and their tests.
- `crates/builtins-compiler/src/lib.rs` (`MeterObserver` only).
- `crates/graph-compiler/src/lib.rs` (tests only).
- `tools/wasm-gate-corpus/src/lib.rs`, `tools/wasm-gate-guest/src/lib.rs` and
  `tools/wasm-gates/src/{lib,main}.rs`, for gate M5's count only.
- `crates/{lane,builtins,graph}/tests/MUTATIONS.md` and `tools/wasm-gates/MUTATIONS.md`.
- This spec.

## Non-goals

- **Held peak with hold > 0 or decay on.** It stays scalar (R3). No shipped host configures it:
  host-core pins hold 0 and decay 0 (`crates/host-core/src/prepare.rs:1116-1118`).
- **The controlled path** (#943 S1), **intermediate taps** (#943 S2), and **plans bound with an
  observation activation.** As in P1, those never reach `observe_unit`.
- **A window boundary inside a block** (#943 S4). That lane falls back to the scalar path.
- **Interleaving both planes in one frame loop.** It measured 573 ns against 649-653 ns for the pass
  alone at `Simd8`. It is a schedule-only follow-up for the weekly optimization pass.
- **A reordered energy sum** (pairwise, block partial or tree). It is class B (R3), and this issue
  refuses it (I6).
- **Any change to** windows, snapshot shape, queues, host APIs, the SDK, the AudioWorklet
  `--kernel-min` ratchet, or the #143 observation-ordering contract.

## Objective gates

- **M1. Kernel identity** (new `crates/lane/tests/meter_block.rs`).
  - Compare `meter_block::<Simd4>` and `<Simd8>`, lane by lane and every field by bits (counts as
    integers), against two references:
    - `::<f32>` over the de-interleaved lane;
    - an independent oracle written in the test: the scalar ALL loop. For each sample, `s = if
      x.is_finite() && !x.is_subnormal() { x } else { 0.0 }`, then `a = s.abs()`, the peak `if a >
      p`, `e += f64::from(s) * f64::from(s)`, `sanitized += !(x.is_finite() && !x.is_subnormal())`
      and `clipped += a >= 1.0`.
  - Run 64 blocks, with energy carried from each block into the next as its seed, at frames 1, 2, 3,
    127, 128 and 129.
  - Inputs: P1's G1 hostile pool, plus `1 ± ulp`, `±1e30`, `1e38` and `±f32::MAX`; tone input; and
    random positive seeds.
  - `peak` must equal `meter_sample_peak_block(.., L::zero())` bit for bit.
  - Every vector result passes through `black_box` before comparison (the lane draft's measured
    folding hazard).
- **M2. Accumulator differential** (`crates/builtins/tests/meter.rs`, new test
  `a_banked_block_commit_publishes_the_scalar_meters_snapshots`).
  - **Arms.** Arm A calls `observe_input`. Arm B calls `banked_seed`, then `meter_block` (`Simd8`
    with 8 meters, and `Simd4` with 4), then `observe_input_banked`. Each meter observes one lane of
    a strided block.
  - **Configurations:**
    - periods {1, 64, 127, 128, 129, 300, 512, 1536, 4096};
    - metric sets {ALL, E, C, H, P, P|E, P|C, E|H, C|H};
    - hold/decay {(0, 0), (7, 0), (0, 12), (100, 60)};
    - hostile and tone input;
    - a fixed stream of 48 blocks of 128 frames;
    - a random stream of 150 events: skips, restarts, both reset kinds, zero-frame blocks, and blocks
      of {1, 2, 3, 63, 64, 127, 128, 129, 256, 300, 511} frames.
  - **Equality.** Every snapshot must be equal, with every float field compared by bits.
  - **Commit counter:**
    - exactly `8 × 64` for ALL at period 512, over 64 blocks of 128 frames;
    - `0` at period 64;
    - `8 × 64` at period 1536;
    - strictly between 0 and 512 at period 300;
    - `0` on every stream that asks for `HELD_PEAK` with hold ≠ 0 or decay ≠ 0;
    - `0` with `banked: None`.
  - **Discontinuity.** In the stream "block, skip 7 frames, block", the second block commits: the
    counter rises by 8 for ALL at period 512.
  - **Safety net.** An `energy_seed` moved by one ulp gives no commit, and the snapshots still equal
    arm A.
- **M3. End to end** (`crates/graph-compiler/src/lib.rs`, beside P1's G3).
  - **Setup.** The 64-track intended fixture, with an ALL meter on every track at `PostMatrix`
    through `compile_console_model_with_builtins`. Run both deliveries and two periods (512 and 300),
    24 blocks each, once with the pass on and once with `test_only_set_bank_meter_declined(true)`.
  - **Required outputs.**
    - PCM bit-identical, and every meter frame equal by bits.
    - Frames are non-empty and carry signal.
    - At least one window where the left and right energies differ.
  - **Required shape.** `[chains, slots]`, transposes, `folds == 64` and redirects are identical.
  - **Counters.**
    - Full passes are `BLOCKS × cohorts` with the pass on, and `0` declined.
    - P1 passes are `0` in both arms.
    - Commits are above 0 with the pass on, and `0` declined.
  - **Controls:**
    - **Mixed.** Tracks alternate `SAMPLE_PEAK` and ALL, through P1's `SelectedMeterRequest`
      helper. Full passes are `BLOCKS × cohorts`, P1 passes are `0`, P1 fast merges are above 0, and
      the frames equal the declined arm's.
    - **P1's `SAMPLE_PEAK` fixture** runs `0` full passes.
    - **Ballistics.** ALL meters with `peak_hold_frames: 8` run `0` passes and stay bit-identical.
    - **Activation.** The ALL fixture plus one controlled observer runs `0` passes and stays
      bit-identical.
- **M4. Realtime.** Beside P1's G4:
  - Render the ALL-metered 64-track plan for 1,000 blocks under `audit::warm_up(); audit::reset();`.
  - Require `audit::snapshot().total() == 0` and passes `== 1_000 * cohorts`.
  - `rt9_identity_metadata_has_no_retained_or_peak_layout_delta` must stay green.
- **M5. wasm.**
  - **The artifact gate.** `bash scripts/build-web-audioworklet.sh`, then `bash
    scripts/check-web-audioworklet.sh` passes. `--kernel-min 11` and rule 1's trap owners are
    unchanged.
  - **The census.** Record the census of `bank_meter_pass` in the built artifact (`wasm-objdump
    -d`):
    - expected: `f64x2.promote_low_f32x4`, `f64x2.mul`, `f64x2.add`, `f32x4.abs`, `f32x4.add`, and
      `f32x4.pmax` or `lt` plus `bitselect`;
    - forbidden: any `f64.promote_f32`, `f64.mul`, `f64.add`, `f32.add`, `f32.gt` or `f32.abs`.
  - **The kernel differential.** `bash scripts/run-wasm-gates.sh` passes, and gains
    `meter_block_mismatches(width) -> u32`.
    - It has the same count form, guest export and host plumbing as the lane draft's
      `f64_lane_mismatches`.
    - It runs `meter_block` against M1's independent oracle over 16 carried blocks of hostile and
      tone input at every width.
    - It must be `0` on all three legs.
- **M6. Suites and policy.** Run P1's G6 list unchanged, plus `scripts/run-wasm-gates.sh` and
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.

### Red mutations (each applied alone; record in the crate's MUTATIONS.md)

| # | mutation | must go red |
|---|---|---|
| K-1 | the commit writes `seed + (energy of a zero-seeded pass)` instead of the seeded result | M2. Measured in research: 1-ulp `energy` differences on tone input. This is the class-B form. |
| K-2 | `banked_seed` ignores the discontinuity | M2 discontinuity counter. Snapshots stay green because the bit check declines, so the counter is the witness. |
| K-3 | the commit drops the seed-bit check | M2 safety net |
| K-4 | `banked_eligible` ignores decay | M2 (the (0, 12) and (100, 60) streams) |
| K-5 | the sanitized count uses `mask_not(normal)`, which also counts zeros | M1, M2 |
| K-6 | clipped uses `c.gt(one)` | M1, M2 (`±1.0` is in the pool) |
| G-1 | final lane index shifted (`l ^ 1`) | M3 frames |
| G-2 | the right-plane pass reads the left plane | M3 frames (needs the L-differs-from-R window) |
| G-3 | P1's pass also runs when the full pass ran | M3 P1 counter |
| G-4 | `accepts_banked_meter` also accepts `SAMPLE_PEAK`-only meters | M3 P1-fixture control |

Expected green, and recorded as the domain argument witnessed: the held merge with `>` instead of
`>=`; and the counts committed right channel first. Saturating adds commute below `u64::MAX`.

## Console benchmark rows

- **Can move:** `console_meters`, the meters-on arm: ALL at `PostMatrix` on 64 tracks, period 4 ×
  128, hold 0, decay 0 (`tools/console-workload/src/lib.rs:791-810`). Its fold and redirect counters
  must still equal the meters-off arm's (#914).
- **Must not move:**
  - #881's `sixty_four_track_console_metered`, a `SAMPLE_PEAK` row that P1 owns and where no full
    pass runs;
  - every `console_session` row;
  - `console_observation`.
- **Timing.** The paired console benchmark runs once, at the batch boundary. R4's numbers are
  research, not this issue's result.

## Dependencies

- **Hard:** #943 merged. It provides the hook, the `UnitIdentity` flag pattern, `sample_peak`,
  `final_output_block`, `metrics()`, the test switches and the source-scan pins.
- **Hard:** #949 merged. It provides `LaneF64`, `Widen`, and the wasm
  differential and lowering pin.
- #881 is not needed: the row that moves is `console_meters`.
- #882 is unaffected, since the controlled path stays unbanked.
- #883 is independent, but its reach shrinks to the fallback paths (R7).
- Supersedes #884. Close it as superseded when this issue is filed.

## Standing rules for the implementer

- Work only from this body and #943's interface. Read the cited functions first; do not survey the
  workspace.
- Class A means the change moves no rendered bit and no published meter word. Every
  "bit-identical", "equal by bits" or "0 mismatches" gate is a hard stop, not a tolerance.
- The energy is committed only from the seeded, sample-serial pass (I6). Never add a partial to a
  running sum.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh`).
  `crates/graph` stays free of `unsafe`. Only `crates/lane` names `wide`.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not quote a projected saving. The
  console benchmark, the AudioWorklet artifact pin and browser qualification run or repin once, at
  the batch boundary.

## What the implementer will hit

1. **P1's source-scan pins.** `resident_meter_entry_has_one_final_output_dispatch_and_admission_control`
   (`runtime.rs:7832`, as P1 left it) pins the dispatch and the literal `observe(...)` argument
   lists. Update its terms for the new argument and pass, and add a row that forces the full pass on.
   Do not loosen it.
2. **The seed query must mirror the preamble exactly.** The bit check makes a wrong seed safe, but
   only slow: every lane falls back and nothing goes red except the M2 counters. Keep both halves.
3. **Energy commits by replacement.** `energy = b.energy[ch]`. The research's K-1 mutation (`seed +
   partial`) moved `energy`/`rms` by an ulp at a window spanning several blocks.
4. **Counts are left then right.** They go into `cumulative_*` in that order, with `saturating_add`,
   as `observe_segment` does. They are `f32` in the kernel, `u32` in `graph` and `u64` in `builtins`.
5. **Vacuous comparisons.** A native test comparing a vector result with the scalar expression it was
   written from can be constant-folded (lane draft, Root evidence). `black_box` every vector result.
6. **Misleading assembly.** An rlib's assembly is pre-link and shows scalar `vcvtss2sd`. Census the
   AudioWorklet artifact (M5) or a linked release binary.
7. **wasm rule 3 is blind to `f64`.** It counts only `f32x4`/`f32` arithmetic, and its pattern
   `4wide6f32x[48]` does not match the non-generic `bank_meter_pass`, so the kernel count is
   unchanged. M5's census and the wasm-gates differential are the checks.
8. **Partial banks.** The pass covers all W lanes, inactive ones included. Seeds for inactive lanes
   stay `0.0`. Consume results only for final members, which exist only for active lanes.
9. **`MeterSnapshot`'s derived `PartialEq` compares floats with `==`.** Compare every float field by
   bits.
10. **The console helper.** `compile_console_model_with_builtins` (`graph-compiler/src/lib.rs:9474`)
    takes ALL `MeterRequest`s, which is exactly M3's fixture. P1 added the `SelectedMeterRequest`
    sibling for the mixed control.
11. **The builtins probe counters stay unchanged.** `test_only_peak_samples` and the `cfg(test)`
    `meter_work_probe` count scalar-path visits, and their tests call `observe_input` directly, so
    they stay green. If one goes red, the banked path leaked into a direct call.

## Research findings

Shared with #949. Throwaway prototypes were run in the research worktree
and reverted; nothing below is committed code. Host: AMD EPYC 7313P, `x86-64-v3`, rustc 1.97.1,
release profile (fat LTO), every timing pinned with `taskset` to one core, node 22.23.2 for wasm.

### R1. The smallest `f64` lane surface, and fusion

- **What the energy needs.** Per frame and lane: widen `f32` to `f64`, multiply, then add into a
  per-lane `f64` accumulator. Per block: load the seeds and store the sums.
  - That is five operations: `widen`, `mul`, `add`, `load` and `store`.
  - They sit in two traits, `LaneF64` and `Widen: Lane { type F64 }`. `Lane` stays untouched,
    because `builtins`' test wrapper `Observed<L>` implements it (`corpus.rs:605`).
- **The companion has the same lane count.** `f32` becomes `f64`, `Simd4` becomes `wide::f64x4`,
  and `Simd8` becomes `wide::f64x8`, with lane i matching lane i. `wide` splits these into exactly
  the requested shapes:
  - `Simd8` on AVX2: two `__m256d`;
  - `Simd4` on wasm and NEON: two `f64x2` (`v128` or `float64x2_t`);
  - `Simd4` on x86: one `__m256d`.
- **Exactness rules:**
  - **widen** is exact for every non-NaN input. A NaN input gives some NaN; the meter sanitises
    first, so no NaN is ever widened.
  - **mul and add** are IEEE binary64 round-to-nearest-even per lane, the same rounding as the
    scalar `*` and `+`.
  - **`w * w` is exact for `w = widen(x)`.** `x = ±M·2^E` with `M < 2^24`, so `M² < 2^48` fits in
    53 bits, and `2E ∈ [-298, 208]` stays inside the binary64 normal range. The energy's only
    rounding is therefore the add, once per sample, in frame order. A lane runs exactly the scalar
    loop's sequence of adds from the same seed, so every sum is bit-identical.
- **Measured lowering (fat LTO).**
  - x86, `Simd8`, per frame per plane: 2 `vcvtps2pd`, 1 `vextractf128`, 2 `vmulpd`, 2 `vaddpd`, no
    `vfmadd`.
  - x86, `Simd4`: 1 `vcvtps2pd`, 1 `vmulpd` and 1 `vaddpd` on `ymm`.
  - wasm `simd128`, `Simd4`: 2 `f64x2.promote_low_f32x4`, 2 `f64x2.mul` and 2 `f64x2.add`, with no
    scalar `f64`.
  - The widen spelling is `F64::new(self.to_array().map(f64::from))`. `wide` has no conversion, and
    `core::arch` is confined to `softfma.rs` and `fpenv.rs`, so the vector form depends on LLVM's SLP
    pass at fat LTO. The rlib pre-link assembly shows 8 scalar `vcvtss2sd`. The lane draft pins the
    wasm lowering in the required `run-wasm-gates.sh`.
- **FMA contraction under `+fma`.**
  - `wide`'s operators never fuse (`add_m256d`/`mul_m256d`, `f64x2_add`/`f64x2_mul`,
    `vaddq_f64`/`vmulq_f64`). Its `mul_add` does fuse under `avx`+`fma` and on NEON, and is never
    forwarded. That is the `f32` rule: `Lane::fma` is written `(self * b) + c`
    (`wide_impl.rs`, `scalar.rs`), and `check-unfused-seal.sh` refuses any `mul_add(` call outside
    its registered sites.
  - LLVM fuses a separate `fmul` and `fadd` only with `contract`/`fast` flags, `llvm.fmuladd` or
    `-fp-contract=fast`, and rustc emits none of those. The measured kernel has no `vfmadd*pd`.
  - Even a hypothetical `fma(w, w, e)` would round `w·w + e` once, which equals rounding
    `round(w·w) + e` because `w·w` is exact. The energy is doubly safe, but the vocabulary is still
    pinned unfused.
- **Seal vocabulary gap (minor).** `check-unfused-seal.sh`'s `call_pattern` names only the `f32`
  fused intrinsics (`_mm256_fmadd_ps`, `vfmaq_f32`). No gap opens, because `core::arch` is confined
  to two files and `mul_add(` is caught. Adding `_pd`/`_f64` spellings is a one-line follow-up.

### R2. The seeded protocol: per-meter windows, discontinuities, restarts and fallback

- **Why neither naive shape works.** The state lives in W `MeterAccumulator`s behind W `dyn`
  observers, each with its own window, period and discontinuity history, so graph cannot update it
  in place (#943 F7). A block partial cannot be merged for energy either (R3).
- **What works: read the seed, compute, let each meter commit.** Per bank unit and block, after the
  unit executes:
  1. For each final lane `l`, graph asks that member's first accepting observer for
     `banked_meter_seed(first_sample, frames)`. This is a pure read. The meter answers with the
     `[left, right]` energy its own scalar loop would start this block from:
     - `[0, 0]` after a discontinuity, which the preamble would reset;
     - `[0, 0]` for a fresh or restarted window, which is already zero;
     - its running `energy` otherwise;
     - or `None` when the block would cross its window boundary, or when a requested metric cannot
       be banked.
  2. `bank_meter_pass` runs `meter_block` over each final plane from those seeds. Declined lanes and
     inactive lanes run from `0.0`, and their results are unused.
  3. Each meter's own `observe_resident` runs its unchanged preamble (discontinuity, `start`). It
     commits only if the block fits its window **and** the seed it was given equals its post-preamble
     `energy` bit for bit. Otherwise it runs today's scalar segment loop.
- **The bit check makes the protocol self-validating.**
  - A wrong seed can never corrupt a meter; it can only fall back.
  - Two meters on one member are handled the same way: the second commits only if its energy bits
    equal the first's seed.
  - M2's counters are what keep the seed query honest.
- **Window boundaries inside a block** (period not a multiple of the block, or variable frames): the
  meter declines, and the scalar loop splits the block as today. No shipped host produces such a
  block (#943 S4): the web period is blocks × quantum, the controlled policy requires `period %
  quantum == 0`, and the console uses 4 × 128.
- **Restarts and resets** happen between blocks.
  - `restart_observation` comes only from `activation_changed`, and plans with an activation never
    reach `observe_unit`.
  - `reset` is a direct control-plane call; `MeterObserver` wires no reset at render.
  - The seed therefore always reads the post-restart state.
- **Fallback levels.**
  - The whole unit falls back when the bank is ineligible (P1's predicate), in an activation plan, on
    the controlled path, or when every observer declines.
  - A single lane falls back when its seed is `None`, the bit check fails, or the block crosses its
    window.
- **The #714 clause.** #714 forbids "precompute or publish another observer's state" and "batch
  observers across failure boundaries". The protocol computes a meter's next energy outside the
  meter, but only from that meter's own seed, handed only to that meter, and committed only by that
  meter after the bit check. The query is pure, so a failing earlier observer leaves later meters
  untouched. The coordinator should confirm this reading (R8).

### R3. Which statistics are order-free

| statistic | form | why |
|---|---|---|
| sample peak | partial and merge (P1) | Select-max is associative and commutative on the sanitised domain (#943 F4). |
| held peak, hold 0 and decay off | partial and merge, `held = q >= held ? q : held` with P1's `q` | The state machine collapses to `a >= held ? a : held`, and `hold_remaining` stays 0. On the sanitised domain equal values have equal bits, so this is the same max. `held` is never cleared per window, and it merges with the same `q`. |
| clipped and sanitized counts | partial and merge | Exact integers: `f32` below `2^24` per block, then `saturating_add` into `u64`, left then right, the scalar order. |
| energy (and `rms` at emit) | seeded and sample-serial (R2) | `f64` adds round. A partial `(p1 + … + pk)` then `+ E` is a different rounding sequence from `E + p1 + … + pk`. Measured in research: that mutation moved `energy` by 1 ulp at a window spanning several blocks. |
| held peak with hold > 0 or decay on | scalar fallback | It is a data-dependent three-way state machine with a `u32` counter and a multiply-and-flush. A lane-parallel form needs exact integer counter lanes, which `Lane` lacks and `f32` cannot hold above `2^24`, plus `f64` compare and select. No host configures it. |

- **Differential of the whole protocol** (prototype of interface items 1 and 6; both spellings
  of the sanitized count, `valid = normal | zero` and item 1's `a == c`, were run):
  - **Configurations:** 9 periods, 9 metric sets, 4 hold/decay settings, hostile and tone input,
    fixed and random event streams.
  - **Result:** 13,799,688 (`Simd8`), 6,887,064 (`Simd4`) and 1,730,785 (`f32`) snapshots were
    bit-identical on every field, with 313,144, 156,432 and 39,022 banked commits.
  - **Commit counts:** 512 at ALL/512, 0 at /64, 512 at /1536, and 296 at /300 (64 blocks, 8
    lanes).
  - **Liveness:** two mutations go red. Zero seeds, and seed plus zero-seeded partial.

### R4. Measured saving (in-process microbenchmark, direction only)

One bank, one 128-frame resident AoSoA block, tone input, ALL metrics, hold 0, decay 0. Three rounds
of 3,000 × 64 blocks, pinned to one core. The "scalar" column is today's `observe_input` per lane.

| configuration | scalar ns per bank per block | seeds + pass + commits | pass alone | ratio |
|---|---:|---:|---:|---:|
| `Simd8`, 8 meters, period 512 | 3,594-3,603 | 785-788 | 649-651 | 4.6x |
| `Simd8`, period 1536 | 3,571-3,572 | 768-774 | 649-653 | 4.6x |
| `Simd4`, 4 meters, period 512 | 1,800-1,801 | 494 | 476-479 | 3.6x |
| `Simd8`, both planes interleaved (follow-up) | 3,594-3,595 | 677-691 | 573 | 5.2x |
| earlier AVX2-intrinsics prototype (#943 F2) | 3,626-3,802 | 623-628 | — | 5.9x |

- **Estimate, not measured in situ.** On the 64-track `Simd8` console block (8 banks) this is about
  8 × (3.59 - 0.79) ≈ 22 µs of the ~31 µs ALL meter cost, roughly 13-14 % of the 164 µs metered block.
- **P1's precedent.** For P1, the per-bank microbenchmark predicted the in-situ saving within its
  spread (#943 F2: -16.8 µs).
- **The browser shape** (`Simd4`, 16 banks of 4) saves about 20 µs, native-equivalent.

### R5. The wasm story

- **The opcodes are base `simd128`.** `f64x2.add`, `f64x2.mul` and `f64x2.promote_low_f32x4` are
  base opcodes, not relaxed ones. Relaxed SIMD stays refused.
  - `wide`'s `f64x2` lowers to them directly.
  - The `Simd4` companion `f64x4` is two `f64x2`, and the dead `Simd8` arm is four.
- **Exactness under V8** (node 22, `simd128`, fat LTO):
  - widen: all 2^32 patterns at `Simd4` and `Simd8` against in-module `f64::from`, and all 2^32
    against a JS exact-conversion oracle, 0 mismatches;
  - carried `e + w·w` chains (20,000 × 256 steps, both widths): 0 mismatches.
- **Speed under V8.** For a 4-lane bank, 128 frames, both planes, the banked pass took 474-480 ns
  against 2,053-2,097 ns for a per-lane scalar loop of the same arithmetic without window
  bookkeeping: 4.3-4.4x. The census of that pass had four each of `f64x2.promote_low_f32x4`,
  `f64x2.mul` and `f64x2.add`, and zero scalar `f64.*`.
- **Callgraph rule 3** (`check-web-audioworklet-callgraph.py`) counts only `f32x4.{mul,add,sub,div}`
  against `f32.*`. It cannot see `f64`, and its pattern `4wide6f32x[48]` does not match a non-generic
  graph function. So neither draft touches rule 3 or `--kernel-min`. Instead:
  - **The lane draft** adds `f64_lane_mismatches` (an executed differential on all three legs) and
    `check_f64_lane_lowering` in the required `run-wasm-gates.sh`. The latter is a durable census of
    a `Simd4` probe on the simd128 guest, modelled on `check_detector_residency`; its shape was
    prototyped with a clean census.
  - **This draft** adds `meter_block_mismatches`, and records the artifact census of
    `bank_meter_pass` (M5).
- **NaN nondeterminism** (wasm `f64.promote_f32` and `f64x2` arithmetic on NaN) never matters. The
  meter sanitises before widening, and NaN is excluded from every comparison by rule 2 of the
  corpus.

### R6. Reach

- **Reached:** permanent meters in plans without an observation activation, whose metric set is
  bankable and not exactly `SAMPLE_PEAK`. That covers:
  - host-core's native console default (ALL at `meter_tap`; `prepare.rs:1087-1120`);
  - the `console_meters` benchmark arm;
  - the web spectrum boot, whose meters are ALL (#943 amendment 3) and permanent, since it passes
    `observation_demand: None` (`prepare.rs:847-890`).
- **Not reached:**
  - the controlled path, and plans with an activation (#943 S1, #882);
  - planar (non-bank) units;
  - meters with hold or decay (reachable only through the builtins-compiler `MeterRequest` API).
- The web default boot stays on P1.

### R7. Relationship to other issues

- **#884.** This issue delivers its counts and held-peak kernel and its energy, which #884 kept
  scalar. The in-place-state design is replaced by R2's protocol. Close #884 as superseded.
- **#943 (P1).** This issue extends P1's hook, identity-flag pattern, block field and accessor. On a
  unit where it runs, the full pass replaces P1's pass, and `SAMPLE_PEAK` meters there take their
  peak from it.
- **#883** (scalar hold-0 shortcut). After this issue, hold-0 meters on banked permanent units no
  longer run the scalar loop. #883 still helps the fallbacks: window-crossing blocks, controlled and
  planar observers.
- **#881 and #882.** Unaffected.
- **#714.** See R2 for how the protocol reads against its observer clause.

### R8. Decisions for the owner or coordinator

1. **#884.** Close it as superseded by these two drafts.
2. **Held peak with ballistics stays scalar.** No host exposes hold or decay. Accept the permanent
   fallback, or ask for a later brief of a lane-parallel state machine, which would need exact
   integer counter lanes in `crates/lane`.
3. **#714's clause.** Confirm that R2's read-seed, compute, owner-commits protocol satisfies "no
   precomputed or published observer state".
4. **The widen lowering relies on LLVM's SLP at fat LTO.** The recommendation is the portable
   spelling, pinned by the wasm census. The alternative is to let `lane-source.toml` admit
   conversion intrinsics (`_mm256_cvtps_pd`, `f64x2_promote_low_f32x4`, `vcvt_f64_f32`) in a new lane
   file. That is a policy change, and it is not needed today: the measured lowering is already
   vector on x86 and wasm.
5. **#883.** Keep it for the fallback paths, or close it.

## Evidence: how R2-R5 were measured

All harnesses were throwaway, run in the research worktree and reverted.

- **Lane prototype.** `LaneF64`/`Widen` exactly as the lane draft's item 1, plus `meter_block`
  (single-plane) and a two-plane variant.
- **Accumulator prototype.** `banked_seed` and `observe_input_banked` exactly as interface item 6,
  with a thread-local commit counter.
- **Differential.** A `builtins` integration test ran arm A (`observe_input`) against arm B (seed,
  pass, commit) over R3's configurations, comparing every snapshot field by bits.
  - Two mutations turned it red. The zero-seed mutation drifted `energy` at `metrics=15`,
    `period=64`. The reordered partial drifted 1 ulp at a window spanning several blocks.
- **Microbenchmark.** Inside the same test, with the mode flags read once through a `OnceLock`,
  because reading an environment variable inside the timed loop inflated the first run by about
  40 ns.
- **Codegen.** A release example binary was disassembled with `objdump -d -M intel`. For wasm, a
  standalone `cdylib` (fat LTO, `+simd128`) was disassembled with `wasm-objdump -d`, and a census was
  taken per function.
- **Native widen check.** All 2^32 patterns at `Simd4` and `Simd8` against the integer-construction
  oracle, with `black_box` on the vector result: 9.7 s, 0 mismatches. Without `black_box`, against
  `f64::from`, the sweep folded to 5 ms.

## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27, on branch `codex/950-banked-full-meters` from `e2b5b9c8` (the
optimisation batch with #943 and #949 merged). Host AMD EPYC 7313P, `x86-64-v3`, rustc 1.97.1,
node 22.23.2, wasmtime 47.0.3. `CARGO_INCREMENTAL=0`, the worktree's own `target/` (deleted at the
end). Nothing pushed; no timed benchmark; the AudioWorklet pin is not repinned.

Commits: `b0ee0f1e` (lane kernel, meter accumulator, M1, M2), `b4265212` (graph pass,
`MeterObserver`, M3, controls, failure boundary, M4, source scan), `232eb989` (wasm count, probe,
lowering pin, G6 assertion), `326607ce` (seeds written in place: two 128-byte copies removed),
`9696f74d` (M2's two-meter row isolated, failure-boundary commits pinned), `9073fffb` (rustdoc
links), `646bc0dd` (mutations), and this section.

### Design

- **Kernel** (`crates/lane/src/kernels/builtins.rs`): `MeterBlock<L: Widen>` and
  `meter_block<L: Widen>(words, frames, energy: L::F64)`, the frozen six-step order of interface
  item 1, `#[inline(always)]`, with the four facts in its doc. `peak` is steps 1-3 of
  `meter_sample_peak_block`, so it equals that kernel seeded with `+0.0`.
- **Accumulator** (`crates/builtins/src/lib.rs`, meter only): `MeterBankedBlock`,
  `banked_eligible`, `banked_seed` (pure; `None` unless eligible, **`ENERGY_RMS` selected**
  (amendment 2) and `frames > 0`; mirrors the preamble's discontinuity; `None` when the block would
  cross the window), and `observe_input_banked`. `observe_input`, `observe_input_with_block_peak`
  and `observe_input_banked` share one private body: preamble, then the banked commit
  (`commit_banked`, before `settled_silence`), then #943's merge, then the unchanged loop. The
  commit checks eligibility, `0 < len <= period - frames` and, with `ENERGY_RMS`, both seeds' bits;
  it then merges peak (`q > peak`) and held (`q >= held`), **replaces** the energy, and
  `saturating_add`s the counts into the window and then the lifetime counters, left then right
  (`commit_banked_lane`, a new realtime-marked region). Test support:
  `test_only_banked_meter_commits` / `test_only_reset_banked_meter_commits`.
- **Graph** (`crates/graph/src/{lib,runtime}.rs`): `GraphBankedMeterLane`,
  `GraphResidentObservationBlock::meter`, and `GraphRuntimeObserver::{accepts_banked_meter,
  banked_meter_seed}` with declining defaults. The bind-time flag lives in #943's
  `UnitObservation` byte, now five states (`Unobserved`, `Observed`, `ObservedWithPeak`,
  `ObservedWithMeter`, `ObservedWithPeakAndMeter`), derived through a shared
  `RuntimeUnit::final_slot_accepts` with #943's `checked_sub`; #943's `const` layout assertion
  holds (`cargo check --target wasm32-unknown-unknown -p graph` passes: still 20 bytes there). In
  `observe_unit`'s bank arm, when the flag, `eligible`, `frames` and the test switch allow:
  `bank_meter_seeds` fills an in-place `Option<[[f64; 8]; 2]>` with each final lane's first seed in
  binding order and returns whether any lane answered (amendment 5: no answer, no pass); then
  `final_output_block` and the `#[inline(never)]` `bank_meter_pass`, which dispatches the width,
  borrows exactly `frames * W` words with `get` (amendment 5), declines above `2^24` frames
  (amendment 5), and calls `meter_block` once per plane. When the full pass ran, #943's pass does
  not, and its peak becomes every final member's `sample_peak`; otherwise #943's pass runs
  unchanged. Each final member gets `meter = Some(lane l)` with counts as `u32` and `energy_seed`
  from the seeds; every other member, the `Op` arm and `observe_one` pass `None`. Test support:
  `test_only_set_bank_meter_declined` (resets the count) and `test_only_bank_meter_passes`.
- **Observer** (`crates/builtins-compiler/src/lib.rs`, `MeterObserver` only):
  `accepts_banked_meter = banked_eligible() && metrics() != SAMPLE_PEAK`; `banked_meter_seed`
  forwards to `banked_seed`; `observe_resident` calls `observe_input_banked` only when
  `block.meter.is_some() && accepts_banked_meter()` (amendment 1) and #943's call with
  `block.sample_peak` otherwise.
- **#714** (amendment 3). The owner's 2026-09-26 ruling supersedes #714 line 167's "no new `f64`
  lanes"; its "no energy reassociation, per-lane sample order" is kept (I6, and K-1 below goes red
  on it). The seed is a `&self` read of each meter's own state, the kernel a pure function of that
  seed and the resident block, and each meter re-checks the seed after its own preamble and
  commits its own lane in binding order behind the `?` short-circuit. The failure-boundary gate
  below witnesses I4.

### Deviations and decisions

1. **The flag is a `UnitObservation` state, not a `bool`** (interface item 4). #943 attempt 2 packed
   its flag into one byte because a fifth `bool` grew the row on wasm32; a sixth would too. Two
   states were added instead, and the `const` assertion is unchanged.
2. **The seeds are not stored in the pass's result.** The first build's census showed two
   `memory.copy` of 128 bytes per bank per block (the seeds returned by value, then copied into
   the result). `326607ce` writes them once into an `Option` in `observe_unit` that the pass and the
   hand-off read by reference, per the owner's copy rule; `bank_meter_pass` now has no
   `memory.copy`, and the caller has only the `+0.0` fill.
3. **Full passes are one per cohort per block that some window holds**, not `BLOCKS x cohorts` at
   every period (amendment 5 skips blocks where no lane answered). At period 300 every meter's
   window crosses on the same blocks, so M3 asserts `passes == cohorts x inside` and `commits == 64 x
   inside` with `inside` computed by a window simulation in the test (14 of 24 blocks); at 512 it is
   `BLOCKS x cohorts`.
4. **G-4's witness is the mixed control**, not the #943 fixture control: after amendment 2 a
   `SAMPLE_PEAK` meter answers no seed, so no pass runs on that fixture even if it accepts; the
   mixed control sees the peak meters commit through the banked path instead of merging.
5. **`observe()` has eight parameters**, under `#[expect(clippy::too_many_arguments)]` with a
   reason, as `observe_one` already does.
6. **The source scan** now counts `.final_output_block(` twice (one borrow in each of the two
   exclusive pass arms) and pins `bank_meter_pass(` and `bank_meter_seeds(` twice each, the new
   gate, seed walk and hand-off terms, the Op arm's two-line form, and `meter,` in the member call
   and in `observe`'s block. New control rows: the full pass forced on, #943's pass run beside it,
   the meter lane withheld and shifted, the seeds not read, and one plane passed twice.
7. **M2's two-meter row is its own test** (`a_peak_meter_bound_first_leaves_the_seed_to_the_all_meter_behind_it`),
   so amendment 2's pollution is reported by the row itself rather than by the sweep failing first.
   M2's sweep runs one thread per period (the counters are thread-local): 20 s in dev.
8. **The `2^24`-frame decline has no test.** A block that long needs 512 MiB of planes at four lanes.
   It is one comparison in `bank_meter_pass`, whose body is otherwise read off the census.
9. **The G6 assertion** is on the canonical arm only, per the issue comment: under DAZ a subnormal
   widen input reads as zero, which is the environment every render entry clears.
10. **The lowering probe also refuses scalar `f32.add`, `f32.gt` and `f32.abs`** and requires
    `f32x4.abs` and `f32x4.add`, so a scalarised sanitize or count is caught as well as a scalarised
    widen.
11. **Twin meters** (two energy-keeping meters on one member) are left as the verification found
    them: the second commits the first's seeded result when the seed bits are equal, which is
    value-exact. No shipped host binds two.

### Gates

| gate | command | result |
|---|---|---|
| M1 | `cargo test --locked -p lane --test meter_block` (dev and `--release`) | PASS, 3 tests: per input family (hostile, tone) `Simd8` 3,072, `Simd4` 1,536 and `f32` 384 lane-blocks bit-identical to the `f32` kernel and the independent oracle on peak, both counts (as integers) and energy, over 64 carried blocks at frames 1, 2, 3, 127, 128, 129 from random positive seeds; `peak` equal to `meter_sample_peak_block(.., L::zero())`; the count-boundary bit sweep; the zero-seeded-partial witness differs at least once |
| M2 | `cargo test --locked -p builtins --features test-support --test meter` (and without the feature: equality only) | PASS: 19,127,808 snapshots bit-identical on every field, 187,908 banked commits. Counters on 64 x 128 frames, 8 ALL meters: 512 at period 512, 0 at 64, 512 at 1536, strictly between at 300; 0 with any hold or decay on a held peak; 0 for every set without `ENERGY_RMS`; 0 withheld; 0 with the seed moved one ulp (snapshots still equal); `[8, 8, 16]` across "block, skip 7, block"; the `[SAMPLE_PEAK, ALL]` row commits 512 (`Simd8`) and 256 (`Simd4`) |
| M3 | `cargo test --locked -p graph-compiler --lib post_matrix_all_meters` | PASS, table below |
| M3 controls | `... --lib the_full_meter_pass_stays_off` | PASS: mixed (on) passes 192, #943 passes 0, merges 768, commits 768; (declined) 0, 192, 768, 0; frames equal. #943's `SAMPLE_PEAK` fixture: 0 full passes, 192 #943 passes, 1,536 merges. Ballistics (hold 8): 0 passes and 0 commits in both arms, frames equal, held peaks non-zero. Activation: 0 passes, 0 commits, the controlled row called 24 times, PCM and 384 frames equal to the declined web arm |
| failure boundary | `... --lib an_observer_failing_mid_bank` | PASS: an observer bound after `ch01`'s meter fails block 5 in both arms; commits per block `[64 x 5, 2, 64 x 10]`; `ch01`'s meter has no discontinuity, `ch02`'s has one; PCM of every good block and every frame equal to the declined arm's |
| M4 | `... --lib the_full_meter_pass_renders_without` | PASS: 1,000 blocks, `audit::snapshot().total() == 0`, full passes 8,000 (`1,000 x 8`), #943 passes 0, commits 64,000, windows 16,000 |
| rt9 / layout | `cargo test -p graph --features test-support`; `cargo check --locked --target wasm32-unknown-unknown -p graph` | PASS (`rt9_identity_metadata_has_no_retained_or_peak_layout_delta` green; the `const` assertion holds on wasm32) |
| M5 kernel differential | `bash scripts/run-wasm-gates.sh` | PASS: native, wasm scalar and wasm simd128 legs each `cases 142, comparisons 358, minmax_lowering_mismatches 0, f64_lane_mismatches 0, meter_block_mismatches 0, mismatches []` |
| M5 lowering pin (amendment 4) | same script, `check_f64_lane_lowering` | PASS: `miso_gate_f64_lane_probe` 2/2/2 `f64x2` promote/mul/add, no scalar `f64`; `miso_gate_meter_block_probe` (the real `meter_block::<Simd4>`) 2/2/2, no scalar `f64`, no scalar `f32.{add,gt,abs}`, `f32x4.abs` 1, `f32x4.add` 2 |
| M5 artifact | the build script's cargo line into `target/web-950`, `wasm-objdump -d`, the four callgraph checks | PASS: `--callgraph miso_engine_web_v1_render` (closure 8, traps 5, #943's owner); `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11` (15 kernels, unchanged); `--callgraph miso_engine_web_v1_meter_poll --trap-owner ...poll_meters`; `--callgraph miso_engine_web_v1_command_submit --allocation-only`; `--self-test`. `host_web.wasm` 3,313,930 bytes, sha256 `b005b58b...5efeaac2`; the pin `8934cdd9...` is not repinned |
| G6 FTZ | `cargo test --locked -p wasm-gates` | PASS, with the new assertions (`f64_lane_mismatches == 0`, `meter_block_mismatches == 0` on the canonical arm) |
| fmt | `cargo fmt --all --check` | PASS |
| clippy | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | PASS |
| doc | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| suites | `cargo test --locked -p ...` | PASS: lane 67 (2 ignored); builtins 116 and 116 with `test-support`; builtins-compiler `test-support` 79; graph 118 and 125 with `test-support`; graph-compiler 98; host-core `--all-features` 225; host-web 206; console-workload 40; capi 36; wasm-gates + wasm-gate-corpus 9 |
| policy | `scripts/check-{lane-policy,unfused-seal,builtins-policy,builtins-fixtures,graph-policy,graph-determinism,realtime-policy,workspace-policy}.sh` | PASS, all eight (realtime: 58 marked regions in 16 files, two new; determinism 100/100) |

`crates/graph`'s dependency list and every `Cargo.toml` are unchanged; no `unsafe` was added; only
`crates/lane` names `wide` (the tools name `<Simd4 as Widen>::F64`). The builtins scalar-path
probes (`test_only_peak_samples`, `meter_work_probe`) are green unchanged.

**M3, per configuration** (24 blocks, pass on / declined; PCM bit-identical, every frame equal on
every field by bits, shape tuple equal, frames carry signal, some window's L and R energies differ,
#943 passes 0 / 0 everywhere):

| dispatch, delivery, period | `[chains, slots]` | transposes | folds | redirects | frames | full passes | commits |
|---|---|---:|---:|---:|---:|---:|---:|
| Simd8, both deliveries, 512 | [8, 48] | 192 | 64 | 0 | 384 | 192 / 0 | 1,536 / 0 |
| Simd8, both deliveries, 300 | [8, 48] | 192 | 64 | 0 | 640 | 112 / 0 | 896 / 0 |
| Simd4, both deliveries, 512 | [32, 64] | 768 | 64 | 60 | 384 | 384 / 0 | 1,536 / 0 |
| Simd4, both deliveries, 300 | [32, 64] | 768 | 64 | 60 | 640 | 224 / 0 | 896 / 0 |
| `console_meters` path (`compile_console_model_with_builtins`), Simd8, 512 | [8, 48] | 192 | 64 | 0 | 384 | 192 / 0 | 1,536 / 0 |

Meter differential totals: M2 19,127,808 snapshots; M3's frames per configuration as in the table
(each control 384 per arm); M4 16,000 windows; the A/B below 104,192 snapshots per arm. Zero
mismatches anywhere.

### wasm census of `bank_meter_pass` (M5)

In the artifact above `bank_meter_pass` is `func[2254]`, with one call site (in
`GraphExecutor::render`, `observe_unit` inlined); `bank_meter_seeds` is inlined into that caller.
`bank_sample_peak` is `func[2256]`, unchanged.

| opcode | count | | opcode | count |
|---|---:|---|---|---:|
| `f64x2.promote_low_f32x4` | 12 | | `f32x4.pmax` | 2 |
| `f64x2.mul` | 12 | | `f32x4.lt` | 4 |
| `f64x2.add` | 12 | | `v128.bitselect` | 10 |
| `f32x4.abs` | 6 | | `f32x4.ge` | 6 |
| `f32x4.add` | 12 | | `f32x4.ne` | 6 |
| `i32x4.sub`, `i32x4.lt_u` | 6, 6 | | `i8x16.shuffle` | 6 |
| `loop` | 4 | | `memory.copy`, `call`, `unreachable` | 0 |
| forbidden `f64.promote_f32`, `f64.mul`, `f64.add` | 0 | | forbidden `f32.add`, `f32.gt`, `f32.abs` | 0 |

- The live `Simd4` arm has 4 each of the three `f64x2` opcodes; the `Simd8` arm, dead on wasm, has
  8 (two `v128` halves), as the verification predicted. `i8x16.shuffle` moves the high pair of each
  `f32x4` down for its second `promote_low`.
- The sanitize is #943's integer range test (`i32x4.sub`, `i32x4.lt_u`); the sanitized count lowers
  as `f32x4.ne` plus `and`; the clipped count as `f32x4.ge` plus `and`.
- Each of the four loops has one conditional branch, its back edge: branch free per frame. No call
  and no trap: the `get` borrow removed the prototype's `slice_index_fail`.
- The caller around the call site has one `memory.fill` of 128 bytes (the seeds' `+0.0`) and no
  `memory.copy` (the first build had two of 128 bytes; deviation 2).

### In-process A/B (throwaway, descriptive, deleted)

A temporary `tools/console-workload/tests/zz_ab_950.rs`: two `SessionRuntime`s of
`Workload::SixtyFourTrackConsole` with `PlanConfig { meters: true, .. }` (the `console_meters`
meters-on arm: ALL at `PostMatrix`, 4 x 128, hold 0, decay 0), one with the full pass and one with
`test_only_set_bank_meter_declined(true)`, alternated per block with the order swapped every block,
512 warm-up and 6,000 timed blocks, release, `taskset -c 29`, meters drained and PCM hashed outside
the clock. Load average 2.3-2.8 (other agents building).

| run | p50 pass on | p50 declined | paired median (declined - on) | p10 / p90 of the delta |
|---|---:|---:|---:|---:|
| 1 | 137.4 us | 160.3 us | 22.7 us | 17.4 / 28.2 us |
| 2 | 137.6 us | 159.9 us | 22.1 us | 16.8 / 27.7 us |

Both arms rendered the same PCM digest over 6,512 blocks and published the same 104,192 snapshots
(handles, windows, starts, peaks, energies, `rms`, counts and held peaks by bits); the pass ran
52,096 times (`6,512 x 8`) on and 0 declined. This is not the benchmark row and not the issue's
result.

### Red mutations

Each applied alone as an exact-text replacement and restored with `git checkout`; the tables with
the failing messages are in `crates/{lane,builtins,graph}/tests/MUTATIONS.md` and
`tools/wasm-gates/MUTATIONS.md`.

| # | mutation | red on |
|---|---|---|
| K-1 | the kernel's energy is a zero-seeded partial plus the seed | M1 (one ulp), M2, M3 (`meter 1 window 1`), the native count (686) |
| K-2 | `banked_seed` ignores the discontinuity | M2's skip counter (`[8, 8, 8]` against `[8, 8, 16]`); snapshots stay equal, as the brief says |
| K-3 | the commit drops the seed-bit check | M2's safety net (moved seed: window 0 differs) |
| K-4 | `banked_eligible` ignores the decay | M2 (`period 129, hold 0, decay 12`) |
| K-5 | the sanitized count also counts zeros | M1, M2, the native count (1,315) |
| K-6 | the clipped count uses `c > 1.0` | M1, M2, the count on all three legs (662); green on M3 (no post-matrix word is exactly 1.0) |
| A-2 | `banked_seed` answers without `ENERGY_RMS` | M2 (`no energy, no seed, no pass`) and the two-meter row alone (128 against 512) |
| A-5 | the pass runs when no lane answered | M3 at period 300 (192 against 112 passes); the source scan |
| G-1 | the hand-off reads lane `lane ^ 1` | M3 frames; the source scan |
| G-2 | the right plane's pass reads the left plane | M3 frames |
| G-3 | #943's pass also runs where the full pass ran | the mixed control (#943 passes 192 against 0) |
| G-4 | `accepts_banked_meter` accepts `SAMPLE_PEAK`-only meters | the mixed control (merges 0 against 768); green on the #943 fixture control (deviation 4) |
| G-5 | the member loop keeps observing after a failure | the failure boundary (8 commits in the failing block against 2) |
| M-W | the kernel widens through a `black_box`ed scalar loop | the lowering pin on `miso_gate_meter_block_probe` (`f64.promote_f32=4`); every count stays 0 and the #949 probe stays green |
| E-1 | held merge `>` for `>=` | green, as expected |
| E-2 | counts committed right channel first | green, as expected |

### For the verifier

- The `2^24` decline (deviation 8) is untested by construction.
- The #943 G3 control that says "ALL meters must not make a bank run the pass" still passes and is
  still true of #943's pass; on that fixture the full pass now runs.
- `sample_peak` and `meter` reach every observer of a final member, not only meters; both field
  docs say to ignore them unless accepted, and `MeterObserver` re-checks its acceptance at render.
- The failure-boundary gate relies on bank 0 (`ch00`..) being the first observed unit, which holds
  at both widths on the intended fixture.
