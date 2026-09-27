# Bank the sample-peak meter pass at the resident final lane

Banked meters, slice P1. Researched on `14f2917b` (2026-09-26). It replaces the peak half of
open #884 with a measured, bounded slice; #881, #882 and #883 stay separate issues. The coordinator
rulings are under "Rulings".

## Amendments (adversarial verification, 2026-09-26; these override any conflicting text below)

The verification confirmed F1 (meters never change cohorts), F4 (the partial-peak merge is
bit-identical: exhaustive over all 2^32 bit patterns at every width natively and on wasm, and 14.3
million meter snapshots bit-identical across periods 1 to 4,096, seven metric sets and four hold and
decay settings), I4/I5 placement and opt-in, the layout, and the saving (paired, one runtime:
-16.7 to -17.4 us on the 64-track browser block; the browser period of 12 x 128 frames takes the
fast path every block). Evidence: `docs/handoffs/meters-2026-09-26/METERS-VERIFY.md`, with the
verification's prototype patch and wasm checker beside it.

1. **Kernel spelling (lane policy).** `scripts/check-lane-policy.sh` refuses a bare `.max(` in
   `crates/lane/src` without a marker. Write the kernel's step as `peak = L::max(c, peak)`, the
   crate's own form (`scalar.rs:171`, `wide_impl.rs:328`); the expected-green mutation becomes
   `L::max(peak, c)`. Loop with `words[..frames * L::WIDTH].chunks_exact(L::WIDTH)` rather than
   `&words[f * W..]`, which keeps a bounds-check branch on every frame.
2. **Wasm gate (G5).** LLVM lowers the two compares to an integer range test (`v128.and`,
   `i32x4.sub`, `i32x4.lt_u`), so `f32x4.ge` never appears, and inlined the pass is lost inside
   `GraphExecutor::render`. Put the width dispatch in an `#[inline(never)] fn bank_sample_peak` in
   `crates/graph`, and record the opcodes it actually emits, with no scalar `f32` compare or max.
   Also add the kernel to `tools/wasm-gate-corpus` so its wasm result is executed and compared with
   native; that crate's paths are authorized for this.
3. **Reach (F3, Mission).** A plan bound with an observation activation sends every observer,
   permanent ones included, through `observe_active_unit` (`crates/graph/src/lib.rs:2483`,
   `:2538-2556`), where the pass does not run; and the web spectrum boot binds ALL-metric meters
   (`crates/host-core/src/prepare.rs:544-584`, `:1087-1095`). P1 therefore reaches the default web
   boot only. Add a G3 control: a mixed permanent-plus-controlled plan runs zero passes and stays
   bit-identical.
4. **Small gaps.** Use `checked_sub` for `members.len() - lanes` (interface item 4). Hazard 7 also
   covers `crates/host-core/src/observation_demand.rs:946-1030`, which pin the probe counter. The new
   `sample_peak` field's doc says it is the peak of the **sanitized** magnitude.

## Mission

AGENTS.md: meters "may observe any boundary without changing signal flow". They should not change
the execution shape either, and they should cost no more than the arithmetic they report. Two
things happen today:

- A meter at an intermediate strip boundary splits its cohort's fused chain. It never changes the
  cohort itself.
- Every meter re-reads its lane of the bank's resident block with a scalar, sample-serial loop,
  once per lane. This costs 15-24 % of the 64-track console block, even at the post-matrix tap,
  where nothing splits.

The first slice (P1) handles the meter the browser product binds, `SAMPLE_PEAK` at `PostMatrix`.
It replaces the eight per-lane loops with one `Lane`-generic pass per bank per block, and each meter
merges its own lane's result into its own window. Class A: rendered PCM and every published meter
word stay unchanged. Keeping chains whole at intermediate taps (S2) and banking the other
statistics (S3) are named successors.

## Research findings

### F1. A meter does not split a cohort. It splits a chain, and only at an intermediate boundary

- **Cohort membership never reads observers.** `bind_rack_banks_indexed`
  (`crates/graph-compiler/src/banks.rs:67`) and the builtin bank planner group lanes by program
  signature. `a_pre_fader_meter_splits_its_cohorts_chain_and_reads_the_limiter`
  (`crates/graph-compiler/src/lib.rs:8520`) asserts two things: `slots == STRIP_SLOTS_PER_COHORT *
  cohorts` ("the meter changes no bank's membership", `:8544-8548`), and `chains == cohorts + 1`
  (`:8561-8565`). In every metered arm measured below, the plan keeps 48 bank slots and only the
  chain count moves (8 to 16). Eight-lane banking is never lost, so no per-track fallback happens.
- **The split is a chain-fusion refusal.** `chains_into` (`crates/graph/src/runtime.rs:7273`)
  declines the `earlier -> later` merge in two cases: an observer is bound to the producing op's
  node (`:7299`), or an observer is bound to a `program::Tap` alias on that op (`:7301-7307`). The
  doc at `:7217-7272` ("The perf cliff this last clause buys", `:7258`) gives the reason: a merged
  chain leaves `earlier`'s member buffers holding the chain's input.
- **Root cause: the boundary must be materialized. The meter itself is not the problem.**
  - Observers run only after a unit completes (executor `crates/graph/src/lib.rs:2540` and
    `:2556`; `observe_unit`, `runtime.rs:3023`; `observe_active_unit`, `:2943`).
  - At that point they can read only a planar arena buffer (`observe`, `:3602`) or the chain's
    resident final lane (`BankChain::final_output_lane`, `crates/rack/src/lib.rs:2157`).
  - An intermediate slot's output exists only inside `BankChain::run_with_input` (rack `:2234`;
    slot loop `:2353`), and the next slot overwrites it.
  - Any `GraphNodeObserverBinding` on such a node therefore forces the split. Spectrum capture is
    one. A send taken from the alias does the same through the sole-readership clause.
- **Which boundaries split:**
  - `PostSimd1`, `PostDynamic` and `PostSimd2PreFader`, the elided aliases.
  - `PostFader`, because the matrix slot follows it in the same chain.
  - `PostInputBuiltins`, when the builtin stage is not its chain's last slot.
- **Which boundaries do not:**
  - `Input` is not a bank member. It is planar and never splits.
  - `PostMatrix` is the chain's final lane, served by the resident view
    (`served_by_the_resident_lane`, `runtime.rs:6642`; #885, #886). It keeps the chain and the route
    fold (`console_facilities_do_not_change_the_chain_shape_or_the_bits`,
    `tools/console-workload/tests/chain_shape.rs:120`).
- **No shipped host meters an intermediate boundary:**
  - The web host pins `MeterTap::PostMatrix` (`hosts/host-web/src/lib.rs:7726-7738`, `:8300`).
  - The controlled policy refuses any other tap (`crates/builtins-compiler/src/lib.rs:3388-3412`).
  - The C ABI exposes no tap.
  - Only the Rust host-core API reaches one: `HostConsoleRequest::meter_tap` and
    `HostMeterRequest::tap` (`crates/host-core/src/prepare.rs:291`, `:340`).
- **The one shipped route to a split is not a meter.**
  - The browser spectrum FFI accepts `TrackPostInputBuiltins` (`hosts/host-web/src/ffi.rs:911`).
  - `has_observer` (`runtime.rs:6538`) counts every *prepared* observer binding, active or not.
  - So a spectrum target that is prepared but idle splits its cohort's chain for the whole life
    of the plan.
- **What a split costs.** The earlier half still scatters every lane (AoSoA to planar transpose).
  The later half then takes the earlier's resident block through a whole-block copy
  (`run_with_resident_input`, `runtime.rs:2931`, into `acquire_resident_input`, rack `:2447`). A
  further unit dispatch and `begin_block` follow. That is two block-sized copies per split cohort
  per block.

### F2. What metering costs today (measured; method under "Evidence")

**64-track console, per-block p50 and paired median delta against the unmetered arm.** Workload:
`SixtyFourTrackConsole`, 128-frame blocks, 48 kHz. Every arm puts one meter on each of the 64
tracks, with a 4-block period and hold and decay off. Arms were alternated block by block. "Web
shape" means `prepare_selected_session_builtins_between_render_calls`, which is what host-core
calls for the browser (`prepare.rs:1156`). "Concurrent" means `..._with_console`.

| arm (tap, metrics, delivery) | `[chains, slots]` | p50 µs | Δ µs vs unmetered |
|---|---|---:|---:|
| unmetered, concurrent (two runs) | [8, 48] | 133.1 / 132.6 | 0 (second unmetered arm: +0.02) |
| `PostMatrix` ALL, concurrent (the `console_meters` arm) | [8, 48] | 164.9 / 163.8 | **+31.3 / +31.1** |
| `PostMatrix` SAMPLE_PEAK, concurrent | [8, 48] | 153.6 / 153.8 | +20.1 / +21.0 |
| `Input` SAMPLE_PEAK, concurrent (planar, no split) | [8, 48] | 150.1 | +17.6 |
| `PostInputBuiltins` SAMPLE_PEAK, concurrent | [16, 48] | 156.0 | +23.2 |
| `PostFader` / `PostSimd2PreFader` SAMPLE_PEAK, concurrent | [16, 48] | 158.3 / 158.2 | +24.7 / +24.5 |
| `PostFader` ALL, concurrent | [16, 48] | 170.1 / 169.7 | +36.4 / +36.8 |
| unmetered, web shape | [8, 48] | 130.9 | 0 |
| `PostMatrix` SAMPLE_PEAK, web shape (**the product**) | [8, 48] | 151.3 | **+20.5** (four runs: 20.1 to 22.6) |
| `Input` SAMPLE_PEAK, web shape | [8, 48] | 147.6 | +16.7 |
| `PostInputBuiltins` / `PostDynamic` SAMPLE_PEAK, web shape | [16, 48] | 153.0 / 153.1 | +22.2 / +22.2 |
| `PostSimd2PreFader` / `PostFader` SAMPLE_PEAK, web shape | [16, 48] | 155.4 / 158.8 | +24.6 / +28.1 |

- **Every arm renders the same output.** Each arm's output digest equals the unmetered digest.
  Folds are 64 in every arm and redirects 0. Split arms double the transpose count (34,048 to
  68,096 over 4,256 blocks).
- **The split's own price.** Take a split arm and subtract the `Input` arm. Both use scalar peak
  meters over planar words in the same run, so the difference is the split alone:
  - `PostInputBuiltins` and `PostDynamic`: +5.5 µs per block for 8 split cohorts.
  - `PostSimd2PreFader`: +7.9 µs.
  - `PostFader`: +11.3 µs.
  - That is about 0.7-1.4 µs per split cohort per block.
- **The meter arithmetic costs 1.5-4 times the split.** It is 17-21 µs for peak meters and 31 µs
  for ALL, against 5.5-11.3 µs for splitting all 8 cohorts.
- **Cross-check against the recorded benchmark.** The recorded `console_meters` record agrees:
  `artifacts/plumbing-floor-baseline/console-benchmark.accepted.jsonl`, rounds 1-2, meters_off
  138.4 µs, delta +30.9 / +31.4 µs, 64 folds. #203 measured the same pass at +17.6-17.9 µs on the
  older 84 µs block.

**Per-bank microbenchmark.** Scope: one 8-lane bank, one 128-frame resident AoSoA block, tone
input, 200,000 blocks, three rounds.

| path | ns per bank per block |
|---|---:|
| today: 8 × `observe_input`, `SAMPLE_PEAK`, strided resident view | 1,975-2,164 |
| today: 8 × `observe_input`, `ALL` | 3,626-3,802 |
| lane pass, `Simd8`, both planes, one accumulator per plane | 159-166 |
| lane pass, `Simd8`, both planes interleaved, two accumulators each | 145-146 |
| lane pass, `Simd4`, one 4-lane bank, both planes | 154 |
| lane pass + 8 × merge-and-window bookkeeping (P1's shape) | 375-401 (5.3x faster) |
| ALL prototype: seeded `f64` energy, peak, held, counts (AVX2 intrinsics, throwaway) | 623-628 (5.9x faster) |

- **The scalar loop is latency-bound.** At about 3.7 cycles per sample it is the loop-carried
  `max` (and the `f64` add chain for ALL), not memory.
- **After P1, about 29 ns per lane remains.** That is the per-lane merge-and-bookkeeping share of
  the fast-path row (375-401 ns less 146 ns, over 8 lanes). On top of it sits graph's per-observer
  dispatch.

**In-situ prototype of P1** (throwaway, wired exactly as the interface contract below, paired
against the same plan with the pass off):

| arm | scalar Δ | banked Δ | saving | readings |
|---|---:|---:|---:|---|
| web shape, `SAMPLE_PEAK` `PostMatrix`, `Simd8`, 6,000 blocks | +22.6 µs | +5.7 µs | **-16.8 µs** | 96,000 snapshots bit-identical |
| same, 4,000 blocks | +21.0 µs | +5.5 µs | -15.5 µs | 64,000 bit-identical |
| concurrent, `SAMPLE_PEAK` `PostMatrix`, `Simd8` | +22.9 µs | +8.6 µs | -14.3 µs | 64,000 bit-identical |
| web shape, `Simd4` dispatch (the wasm/NEON bank width, measured natively) | +20.9 µs | +6.6 µs | -14.3 µs | 64,000 bit-identical |
| concurrent, `ALL` meters, pass **not** opt-in | +33.0 µs | +35.3 µs | **+2.3 µs worse** | 64,000 bit-identical |

- **Reading the table.** P1 removes about 74 % of the product's meter cost: 11 % of the metered
  64-track block, or 13 % of the unmetered one. PCM digests were equal in every run.
- **Why the last row matters.** ALL meters cannot use a peak partial, so an ungated pass is pure
  waste. The pass must therefore be opt-in (invariant I5).
- **The concurrent row's scalar and banked deltas both carry about 3.9 µs** of baseline
  difference: that run's first arm used the faster web-shape plan. The saving is a difference
  between the two, so this offset cancels.

### F3. The metering contract (no slice here changes it)

- **Boundaries.** `MeterTap` (`crates/builtins/src/lib.rs:4294`) maps to `TrackStage` nodes
  (`stage`, `crates/builtins-compiler/src/lib.rs:4763`).
- **Statistics, per window and per channel** (`MeterLaneSnapshot`, `:4408`; selected by
  `MeterMetricSet`, `:4308`):
  - `sample_peak` (`f32`): the D8 select-form maximum of the sanitized magnitude.
  - `energy` (`f64`): the running sum of `x*x` in sample order. `rms = sqrt(energy / frames)` is
    computed at emit.
  - `held_peak`: a hold-frames plus dB-per-second decay state machine.
  - `clipped_samples` (`|x| >= 1.0`) and `sanitized_samples`.
- **Sanitization is `normal_or_zero`** (`:4988`): a finite, non-subnormal sample is kept and
  anything else becomes `+0.0`. This is the meter's rule, not the D7 input rule `NONFINITE_LIMIT
  = 1e30` in `crates/lane/src/kernels/builtins.rs`. Render runs with FTZ and DAZ clear
  (`crates/lane/src/fpenv.rs`), and both rules classify subnormals the same way under either mode.
- **Windows.** Each meter has its own `period_frames`. `observe_input` (`:4574`) splits a block at
  window boundaries. `emit` (`:4735`) publishes one fixed-size `MeterSnapshot` (`:4417`) carrying
  the sequence number, start and end sample, generation, and interval and cumulative counters.
  Discontinuities (`:4728`) and `restart_observation` (`:4711`) reset the window.
- **Reporting.** Reporting is telemetry that may be dropped, with a counter:
  - Each meter has one bounded SPSC `Producer<MeterSnapshot>`, allocated at prepare
    (`prepare_selected`, `:4501`).
  - A full queue drops the snapshot and counts it in `cumulative_dropped_snapshots` (`:4765`).
  - Reads happen off render. host-core `try_read_meter`
    (`crates/host-core/src/observation_demand.rs:1273`) coalesces to the newest snapshot of the
    applied generation (`read_newest_meter_generation`, `:1767`). The web host polls between
    render calls.
- **Shipped configurations:**
  - Browser: `SAMPLE_PEAK` at `PostMatrix`, period = `console_meter_blocks` × quantum (default
    12 × 128), hold 0, decay off (`hosts/host-web/src/lib.rs:7726-7738`, `:8285-8300`). The
    default boot path binds these meters permanently (`prepare_selected_session_builtins_between_render_calls`).
  - Controlled policy: `SAMPLE_PEAK` at `PostMatrix` only, period a multiple of the quantum, no
    ballistics (`builtins-compiler:3388-3412`).
  - Native console default: ALL at `meter_tap` (default `PostMatrix`), hold 0, decay off
    (`prepare.rs:1094`, `:1097-1120`).

### F4. The in-bank pass, and why it is bit-identical (reduction order)

- **Where it hooks in.** At the post-matrix tap it hooks into the unit's existing post-execution
  observation point, `Runtime::observe_unit`'s bank arm (`runtime.rs:3046-3090`). It does not hook
  into `BankChain::run`. After `run` returns, the chain's resident block is still intact: the fold
  mixes a transposed copy in staging, and the scatter reads it. That block is what
  `final_output_lane` hands every post-matrix meter today. So P1 needs no change to how the chain
  runs (at most one read-only rack accessor) and no kernel epilogue.
- **What P1 computes.** For each lane and channel it takes a partial peak over the block, seeded
  with `+0.0`. Each meter then merges its partial into its window's running peak with the same
  select form.
- **Today's order.** The window peak `P` runs `P <- sel(a_i > P, a_i, P)` over the block's
  sanitized magnitudes `a_1..a_k`, in sample order.
- **P1's order.** `Q <- sel(a_i > Q, a_i, Q)` from `Q = +0.0`, then `P <- sel(Q > P, Q, P)`.
- **This is a reassociation, and it is exact.** Every `a_i` and every reachable `P` lies in the
  domain `{+0.0} ∪ [f32::MIN_POSITIVE, f32::MAX]`: sanitization maps NaN, ±inf, subnormals and
  `-0.0` to `+0.0`, and the peak starts at `+0.0` and is cleared to `+0.0` (`clear_interval`). On
  that domain there is no NaN and no `-0.0`, so equal values have equal bits. D8
  `sel(x > y, x, y)` therefore returns the maximum's unique bit pattern. It is commutative and
  associative there, with `+0.0` as identity, so both orders yield the maximum of `{P} ∪ {a_i}`.
- **Class A, no re-baseline.** Measured: 96,000 in-situ snapshots were bit-identical. The builtins
  probe also held on hostile data (NaN, ±inf, ±0, subnormals, MIN_POSITIVE, ±1.0, `2^-34..2^5`)
  at periods 512, 300, 128 and 64. The two-accumulator variant was bitwise equal to the
  one-accumulator one.
- **NaN under the lane contract.** Ordered compares are false for NaN, so the validity mask
  excludes it before `max`. The accumulator never holds NaN, so D8's asymmetric NaN rule
  (`max(NaN, x) = x`) is never exercised.
- **Energy is not order-free.** `f64` sums round, so a block partial is not the running sum. The
  only exact banked form is S3's seeded, per-lane, sample-serial accumulation. Even that keeps the
  scalar loop's exact order of additions, and `f64(x) * f64(x)` is exact (48 significant bits), so
  contraction cannot move a bit. A reordered form (pairwise or block partials) would be class B and
  would move published `energy` and `rms` bits. It is not proposed, and see R4.
- **The rest of ALL is order-free with hold 0 and decay off.** Held peak with `>=` is exact, and
  so are the clipped and sanitized counts: exact integer sums, `f32`-exact below `2^24` per
  block, then merged into `u64`. The ALL prototype was bit-identical on every field.

### F5. Allocation, realtime, control plane

- **Meter slots are already preallocated.** The per-meter `MeterAccumulator` and its SPSC queue
  are allocated at prepare.
- **P1 retains no new byte:**
  - Its per-block result is a stack `[[f32; 8]; 2]`.
  - Its bind-time flag sits in `UnitIdentity`'s existing padding (`runtime.rs:2235`, next to
    `observed`, `resident_input` and `source_lanes`; pinned by
    `rt9_identity_metadata_has_no_retained_or_peak_layout_delta`, `:8159`).
- **The pass is pure.** It loads the resident block and writes the stack array, with no
  allocation, lock, syscall or branch per sample.
- **The control-plane read is unchanged.** The same snapshots reach the same queues, which are
  read off render and coalesced to the newest (F3).

### F6. wasm and callgraph rule 3

- **Rule 3** (`scripts/check-web-audioworklet-callgraph.py:43-45`, `:353-359`; invoked at
  `scripts/check-web-audioworklet.sh:428` with `--kernel-pattern '4wide6f32x[48]'
  --kernel-min 11`) counts only `f32x4.{mul,add,sub,div}` against `f32.{mul,add,sub,div}`.
- **The peak pass carries none of those.** It is `abs`, `ge`, `lt`, `and`, `bitselect` and D8
  `max`, which lowers to operand-swapped `f32x4.pmax` on wasm.
- **So rule 3 can neither fail on it nor vouch for it.** A function with zero counted vector
  arithmetic is skipped (`kernel_arithmetic`, `:327-341`). P1 therefore keeps rule 3 and
  `--kernel-min 11` unchanged, and records an explicit opcode census of the pass instead (gate
  G5).
- **The same blindness applies to S3's `f64x2` arithmetic.** That issue must carry its own check.

### F7. Relationship to other issues

- **#884 (open), "Bank-wide lane meter kernel for peak and count metrics".** It names the right
  idea, but as written it cannot be delivered inside its authorized paths:
  - It asks `builtins-compiler` to "call `observe_resident` once per bank instead of once per
    lane". The per-lane dispatch belongs to `graph` (`runtime.rs:3075-3088`, `:3222`), which #884
    does not authorize.
  - Its kernel updates meter state in place (`state: &mut [LaneMeterWords; ...]`). That state
    lives in eight separate `MeterAccumulator`s behind eight `dyn` observers, and no API reaches
    it from one place.
  - It leaves the `f64` energy loop scalar. For the ALL meters it targets, a per-lane,
    per-sample loop therefore stays. Measured: scalar ALL costs 3.6-3.8 µs per bank against
    2.0-2.2 µs for peak alone.
  - This draft's P1 is the peak part, done with partials and merges and no shared state. #884's
    counts and held metrics belong with energy in S3.
- **#881 (open)** adds `sixty_four_track_console_metered` (`SAMPLE_PEAK` at `PostMatrix` through
  the web entry point). That is the row P1 can move.
- **#882 (open)** rebinds the web meters as controlled observers. That moves them onto
  `observe_active_unit`, which P1 does not bank. S1 must land with or before #882.
- **#883 (open)** is the scalar hold-0 shortcut for ALL meters. It is independent.
- **#714 (closed)** delivered the resident view. Its constraints apply here and are kept:
  - no metering as an executing DSP stage;
  - no observer state precomputed by another party;
  - execute-then-observe order and first-error short-circuit preserved;
  - both final planes read, never left-for-right.

## Class statement

- **P1 is class A.** Rendered PCM is untouched, since this is observation only. Every
  `MeterSnapshot` field is bit-identical to the scalar path by F4's exactness argument. The
  execution shape is unchanged: `[chains, slots]`, transposes, folds and redirects. No fixture or
  digest is re-baselined.
- **S1 and S2 are class A.** S2 hands observers the same words the split shape scattered.
- **S3 is class A only in its seeded, sample-serial `f64` form.** A reordered energy sum would be
  class B: faster, perhaps, but it would change reported `energy` and `rms` values, which are part
  of the observable contract, and it would need an owner ruling and a meter-fixture re-baseline.
  Nothing here proposes it.

## Rulings (coordinator, 2026-09-26)

- **R1. Scope: P1 first.** The shipped cost is the meter arithmetic, not a lost bank (F1, F2). This
  issue is P1 only.
- **R2. #884.** Its peak half is superseded by this issue. #884 stays open for the counts, held
  peak and energy work, to be rescoped as S3 once R4 is decided.
- **R3. S2 (chains kept whole at intermediate taps) is deferred.** No shipped host meters an
  intermediate boundary. The prepared-but-idle spectrum target at `TrackPostInputBuiltins` is
  recorded as its trigger for when S2 is briefed.
- **R4. `f64` lanes in `crates/lane` are an owner decision** that belongs to S3, not to this slice.
- **R5. S1 (controlled observers) changes a probe contract** and must land with or before #882. It
  is filed as its own issue when #882 is scheduled.
- **Benchmark row.** #881 adds `sixty_four_track_console_metered` (`SAMPLE_PEAK` at `PostMatrix`
  through the web entry point), the row P1 can move. #881 lands first.

Side finding, not in scope: `tools/bench` links `graph` with `test-support` as a normal dependency
(`tools/bench/Cargo.toml:31`). #935 removes it.

## Smallest closable slice (P1): banked sample peak at the resident final lane, permanent observers

### Invariants

- **I1. PCM.** No rendered bit moves.
- **I2. Meter words.** Every published `MeterSnapshot` field is bit-identical to the scalar path,
  for every metric set, period, activation and discontinuity pattern. Compare `sample_peak` by
  `to_bits()`.
- **I3. Shape.** `[chains, slots]`, transposes, folds and redirects are identical with the pass on
  and off.
- **I4. Order.** The pass runs after the unit executed successfully and before the unit's first
  observer. It only reads the resident block. Observer order, the first-error short-circuit and
  `write_resident_lane` for planar decliners (#885) are unchanged.
- **I5. Opt-in.** A unit runs the pass only if it was bound with at least one final-slot observer
  that accepts it. ALL-metric meters, spectrum capture and custom observers never trigger it.
- **I6. Realtime and layering.** No allocation, lock or syscall, and no retained byte. No new
  `unsafe` in `graph`. `graph`'s dependency list stays `effect-contract, engine, lane, rack`
  (`scripts/check-graph-policy.sh`). Only `crates/lane` names `wide`.
- **I7. Dual-mono.**
  - Left comes from the left plane and right from the right plane, never left-for-right.
  - The block read is the actual final planes, after the collapse seam's dual suffix (#714).
  - Inactive lanes are never meter input.

### Interface contract

1. **Lane kernel.** Add `lane::kernels::builtins::meter_sample_peak_block<L: Lane>(words: &[f32],
   frames: usize, peak: L) -> L`.
   - Mark it `#[inline(always)]`, with `debug_assert!(words.len() >= frames * L::WIDTH)`, and
     keep it branch-free per frame.
   - Frozen order, for each frame `f` in `0..frames`: `a = L::load(&words[f * W..]).abs()`; `c =
     L::select(L::mask_and(a.ge(L::splat(f32::MIN_POSITIVE)), a.lt(L::splat(f32::INFINITY))), a,
     L::zero())`; `peak = c.max(peak)`.
   - `Lane::max` is D8 `select(c > peak, c, peak)`: the scalar meter's `if absolute > peak {
     absolute } else { peak }` (`crates/builtins/src/lib.rs:4873`).
   - The doc lists this order and says the validity test is the meter's `normal_or_zero`,
     deliberately not `NONFINITE_LIMIT`.
2. **Resident block field.** `graph::GraphResidentObservationBlock` (`crates/graph/src/lib.rs:2114`)
   gains `pub sample_peak: Option<[f32; 2]>`. The value is `[left, right]`, this lane's kernel
   result seeded with `+0.0` over this block. It is `Some` only when this unit's pass ran this
   block.
3. **Observer opt-in.** `graph::GraphRuntimeObserver` (`lib.rs:2121`) gains `fn
   accepts_sample_peak(&self) -> bool { false }`. It is read at bind only, in
   `Runtime::new_with_observation_activation` (`runtime.rs:2577`, beside `row.observed =
   unit.has_observers()` at `:2613`). Render never calls it.
4. **Bind-time flag.** `UnitIdentity` (`runtime.rs:2235`) gains `sample_peak: bool`, placed in the
   existing padding. It is true iff the unit is a `RuntimeUnit::Bank` and some final-slot member
   (`members[members.len() - lanes..]`) holds an observer whose `accepts_sample_peak()` is true.
5. **The pass.** It goes in `Runtime::observe_unit`'s bank arm (`runtime.rs:3046-3090`).
   - **When.** It runs iff `identity[index].sample_peak`, the existing `eligible` predicate
     (`:3061-3071`), and the test switch not declining.
   - **Once per unit.** It runs before the member loop, over the chain's whole final resident
     block: two calls, one per plane, each seeded with `L::zero()`.
   - **Width.** Use the chain's width: `BankWidth::Four` gives `lane::Simd4` and `Eight` gives
     `lane::Simd8`, as the route fold instantiates `fold_resident_tiles` at `:2074-2076`.
   - **Result storage.** Store the result in a stack `[[f32; 8]; 2]`.
   - **Hand-off.** Each final-slot member at lane `l` passes `Some([left[l], right[l]])` to
     `observe` (`:3602`), which puts it in the block it builds (`:3630`). Every other member
     passes `None`, and so does the controlled path's `observe_one` (`:3704`).
   - **Block access.** Take the whole-block words from a new `BankChain::final_output_block(frames)
     -> Option<(&[f32], &[f32])>` beside `final_output_lane` (`crates/rack/src/lib.rs:2157`), with
     the same shape checks.
6. **Meter fast path.** `builtins::MeterAccumulator` (`crates/builtins/src/lib.rs:4442`) gains
   `pub fn observe_input_with_block_peak(&mut self, input: MeterInput<'_>, first_sample: u64,
   block_peak: Option<[f32; 2]>) -> Result<(), MeterObservationError>`.
   - `observe_input` (`:4574`) becomes a call with `None`.
   - The existing preamble is unchanged: the overflow check, the discontinuity and `start`.
   - The fast path applies iff `block_peak == Some(p)`, `self.metrics == MeterMetricSet::SAMPLE_PEAK`
     exactly, `len > 0`, and `len <= (period - self.frames) as usize`. It sets `left.peak = if
     p[0] > left.peak { p[0] } else { left.peak }` (likewise right), adds `len` to `frames`, calls
     `emit()` if `frames == period`, and returns.
   - Otherwise the existing segment loop runs and `block_peak` is ignored.
   - Also add `pub const fn metrics(&self) -> MeterMetricSet`.
7. **Meter observer.** `builtins_compiler::MeterObserver` (`crates/builtins-compiler/src/lib.rs:4673`):
   - `accepts_sample_peak` returns `self.0.metrics() == MeterMetricSet::SAMPLE_PEAK`.
   - `observe_resident` (`:4690`) passes `block.sample_peak` to `observe_input_with_block_peak`.
8. **Test support.**
   - `graph`, under `#[cfg(any(test, feature = "test-support"))]`: add
     `test_only_set_bank_sample_peak_declined(bool)`, which skips the pass and is the oracle arm,
     and `test_only_bank_sample_peak_passes() -> u64` with a reset.
   - `builtins`: add a counter of fast-path merges, under the same `cfg`.

### Authorized paths

- `crates/lane/src/kernels/builtins.rs`, plus the new test `crates/lane/tests/meter_peak.rs`.
- `crates/builtins/src/lib.rs` (meter accumulator only) and `crates/builtins/tests/meter.rs`.
- `crates/rack/src/lib.rs`, for the one accessor only.
- `crates/graph/src/lib.rs` and `crates/graph/src/runtime.rs`, and their tests.
- `crates/builtins-compiler/src/lib.rs` (`MeterObserver` only).
- `crates/graph-compiler/src/lib.rs` (tests only).
- `crates/{lane,builtins,graph}/tests/MUTATIONS.md`.
- This spec.

### Non-goals

- **Out of scope in this slice:**
  - The controlled path (S1).
  - Intermediate taps and `chains_into` (S2).
  - ALL, `COUNTS` and `HELD_PEAK` metrics (S3).
  - Window boundaries inside a block, which fall back (S4).
  - Any change to windows, snapshot shape, queues, host APIs, the SDK, `console_meters` or the
    #143 observation-ordering contract.
- **Fusing the pass into the fader/matrix kernel.** The whole pass measures 146-166 ns per 8-lane
  bank, about 1.3 µs per 64-track block. A fused epilogue could save less than that, and it would
  couple meters into `fader_matrix_block` and the collapse seam.

### Hazards (what the implementer will hit)

1. **A source-scan test pins the dispatch.**
   `resident_meter_entry_has_one_final_output_dispatch_and_admission_control` (`runtime.rs:7832`)
   pins exactly one `.final_output_lane(` call in production and the literal `observe(...)`
   argument lists (`:7859-7873`), with mutation rows after them. Update its terms for the new
   argument and accessor, and add a row that forces the pass on (`sample_peak && eligible` to
   `true`). Do not loosen it.
2. **The validity test must be the meter's.** Use `MIN_POSITIVE` and `INFINITY`, never
   `NONFINITE_LIMIT` (F3). Reusing the D7 constant admits subnormals and rejects `1e30..inf`
   differently, and gate G1 goes red.
3. **A block that crosses its window must take the scalar path.** That happens with a period that
   is not a multiple of the block, a period shorter than the block, or variable block frames.
   Merging the partial there would put post-boundary samples in the old window.
4. **The fast path needs exactly `SAMPLE_PEAK`.** Any other selection still needs the samples.
5. **The opt-in is not optional.** Measured: an ungated pass costs +2.3 µs per 64-track block on
   ALL meters (F2).
6. **Partial banks are eligible.** Their `population < width` (`*active == (lane < population)`).
   The pass covers all W scratch lanes, including inactive ones. Consume `peaks[l]` only for
   final members, which exist only for active lanes.
7. **The controlled-path probe tests must stay green.** `builtins::test_only_peak_samples` counts
   scalar-path visits. The tests that assert it are controlled-path tests
   (`crates/builtins-compiler/src/lib.rs:11550-11557`;
   `crates/host-core/tests/observation_demand.rs:173-887`). P1 does not bank the controlled path
   (`observe_one` only passes `None`), so they stay green; if one goes red, P1 has leaked into S1.
8. **`GraphResidentObservationBlock` has public fields.** Its only constructors are
   `runtime.rs:3630` and `:3704`, and test observers only read it.
9. **`MeterSnapshot`'s derived `PartialEq` compares floats with `==`.** Compare `sample_peak` by
   bits explicitly.
10. **The console test helper only takes ALL meters.** `compile_console_model_with_builtins`
    (`crates/graph-compiler/src/lib.rs:9474`) takes `MeterRequest`. Add a sibling taking
    `SelectedMeterRequest`, through `prepare_selected_session_builtins_with_console` and
    `prepare_selected_session_builtins_between_render_calls`.
11. **wasm.** The pass is invisible to callgraph rule 3 (F6). Record its opcode census; do not
    change the ratchet.

### Objective gates

- **G1. Lane kernel identity** (new `crates/lane/tests/meter_peak.rs`).
  - Compare `meter_sample_peak_block::<Simd4>` and `<Simd8>`, lane by lane and bit for bit,
    against two references: `::<f32>` over the de-interleaved lane, and an independent oracle in
    the test (`if x.is_finite() && !x.is_subnormal() { x } else { 0.0 }`, then `abs`, then `if a
    > p { a } else { p }`).
  - Run 64 blocks with the accumulator carried across them, at frames 1, 2, 3, 127, 128 and 129.
  - Inputs: NaN, ±inf, ±0.0, the smallest and largest subnormal (both signs), ±`MIN_POSITIVE`,
    ±1.0, ±`f32::MAX`, and magnitudes `2^-34..2^5`.
  - A lane fed only invalid values ends at `0x0000_0000`.
- **G2. Meter fast path** (`crates/builtins/tests/meter.rs`, new test
  `a_block_peak_merge_publishes_the_scalar_meters_snapshots`).
  - Setup: 8 meters per arm, one per lane of a strided 8-lane block; periods 512, 300, 128 and 64;
    hostile and tone input; 64 blocks.
  - Arm A calls `observe_input`. Arm B calls `observe_input_with_block_peak` with G1's peaks.
    Every snapshot must be equal, `sample_peak` by bits.
  - The same equality must hold with each of these added: a skipped block (discontinuity),
    `restart_observation` mid-window, and a zero-frame block.
  - With `ALL` or `SAMPLE_PEAK | COUNTS` and `Some(peaks)`, arm B declines: it stays equal to arm
    A and the merge counter does not move.
  - Merge counter: exactly `8 * 64` at period 512, `0` at period 64, strictly between at 300.
- **G3. End to end** (`crates/graph-compiler/src/lib.rs`, beside
  `a_meter_on_the_matrix_keeps_the_route_fold_and_still_meters`, `:8799`).
  - Setup: the 64-track intended fixture with a `SAMPLE_PEAK` `PostMatrix` meter on every track.
    Run both deliveries (concurrent and between-render-calls) and two periods (512 and 300), 24
    blocks each.
  - Each configuration runs twice: once with the pass on, and once with
    `test_only_set_bank_sample_peak_declined(true)`.
  - Required: PCM bit-identical; every meter frame equal (`sample_peak` by bits); frames non-empty
    and carrying signal; at least one window where left and right peaks differ.
  - Required shape: `[chains, slots]`, transposes, `folds == 64` and redirects identical.
  - Pass counter: `BLOCKS * cohorts` with the pass on, `0` with it declined.
  - Control: the same fixture with ALL meters (`MeterRequest`) runs `0` passes.
- **G4. Realtime.** In `graph-compiler`, beside the existing audit test at `lib.rs:5116`:
  - Render the peak-metered 64-track plan for 1,000 blocks under `audit::warm_up();
    audit::reset();`.
  - Require `audit::snapshot().total() == 0` and passes `== 1_000 * cohorts`.
  - `rt9_identity_metadata_has_no_retained_or_peak_layout_delta` must stay green.
- **G5. wasm.**
  - `bash scripts/build-web-audioworklet.sh`, then `bash scripts/check-web-audioworklet.sh` passes
    with `--kernel-min 11` unchanged.
  - The evidence records the opcode census of the function holding the pass in the built
    artifact (`wasm-objdump -d`): `f32x4.abs`, `f32x4.ge`, `f32x4.lt`, `v128.and`,
    `v128.bitselect` and `f32x4.pmax` against `f32.abs`, `f32.max`, `f32.gt`.
  - The artifact pin and browser qualification are repinned once at the batch boundary.
- **G6. Suites and policy.**
  - `cargo fmt --all --check`, and `cargo clippy --locked --workspace --all-targets
    --all-features -- -D warnings`.
  - `cargo test` for:
    - `-p lane`, `-p builtins`, and `-p rack` if touched;
    - `-p graph`, both with and without `--features test-support`;
    - `-p graph-compiler`, `-p builtins-compiler --features test-support`, and
      `-p host-core --all-features`;
    - `-p host-web` and `-p console-workload`.
  - Policy scripts: `scripts/check-lane-policy.sh`, `check-unfused-seal.sh`,
    `check-builtins-policy.sh`, `check-builtins-fixtures.sh`, `check-graph-policy.sh`,
    `check-graph-determinism.sh`, `check-realtime-policy.sh`, `check-workspace-policy.sh`.

### Red mutations (each applied alone; record in the crate's MUTATIONS.md)

| # | mutation | must go red |
|---|---|---|
| L-1 | `a.ge(MIN_POSITIVE)` becomes `a.ge(zero)` (admits subnormals) | G1, G2 |
| L-2 | drop the `a.lt(INFINITY)` term | G1, G2 |
| B-1 | the fast-path window test becomes `true` | G2 (periods 300 and 64) |
| B-2 | fast path when `metrics.contains(SAMPLE_PEAK)` | G2 (the ALL arm) |
| B-3 | fast path skips `emit()` at `frames == period` | G2 |
| G-1 | the `sample_peak` identity flag is forced `true` | G3's ALL-arm pass counter; PCM and frames stay green, so the counter is the only witness |
| G-2 | final lane index shifted (`l ^ 1`) | G3 frames |
| G-3 | the right-plane pass reads the left plane | G3 frames (needs the L-differs-from-R window) |

Expected green, and recorded as the domain argument witnessed: `c.max(peak)` becomes `peak.max(c)`.
The two agree bitwise on the sanitized domain.

### Console benchmark rows

- **Can move:** #881's `sixty_four_track_console_metered` (`SAMPLE_PEAK` at `PostMatrix`, web entry
  point).
- **Must not move:**
  - `console_meters` (ALL meters): no pass may run (I5), and its fold and redirect counters stay
    as they are.
  - Every `console_session` row and `console_observation`.
- **Timing.** The paired console benchmark runs once at the batch boundary. If #881 has not merged
  before this batch, no row can show the change, and the evidence is G3/G4 plus the research
  numbers above, which must not be quoted as the issue's result.

## Named successors (stateless, each its own issue)

- **S1. Controlled observers.** Path: `observe_active_unit` (`runtime.rs:2943`) to
  `observe_active_entry` (`:3222`) to `observe_one` (`:3680`).
  - Compute the pass lazily, at the unit's first active final-lane entry whose observer accepts
    it, and reuse it for that unit's remaining entries.
  - Probe contract change (R5): the tests pinning scalar lane visits (builtins-compiler
    `:11550-11557`; host-core `tests/observation_demand.rs:173-887`) become "one pass per bank
    per block with at least one selected accepting lane; zero scalar visits".
  - Must land with or before #882. Class A.
- **S2. In-chain observation at intermediate slot boundaries (keeps the chain).**
  - Mechanism: `BankChain` gets a bind-time per-slot tap mask. `run_with_input` (rack `:2234`)
    calls a new `BankMembers::observe_slot(k, view)` (default no-op) after slot `k` (`:2353`).
    Graph's `ArenaMembers` dispatches slot `k`'s member observers with resident lane views, and
    P1's pass where accepted.
  - `chains_into` (`:7299-7307`) and `observed` (`:6615`) excuse observers that declare, at bind,
    that they are served resident. Meters (`builtins-compiler:4690`) and spectrum capture
    (`crates/host-core/src/spectrum.rs:2909`) both accept resident views today.
  - Order and failure semantics equal the split shape: unit A's observers already ran between slot
    `k` and slot `k + 1`.
  - Removes, per split cohort per block, the scatter and the whole-block resident copy
    (rack `:2447`). Measured bound: 5.5-11.3 µs per 64-track block when all 8 cohorts split.
  - Hazards: a tap inside a collapsed prefix, where the right plane is invalid until the seam, so
    decline collapse or offer after the seam; the source-scan pins; and the activation cursor must
    not dispatch a slot-`k` entry twice.
  - Ruling R3.
- **S3. Banked ALL metrics (rescoped #884).**
  - One pass computes the order-free partials for peak, held peak (hold 0 and decay off, `>=`),
    and clipped and sanitized counts (`f32`-exact, merged into `u64`).
  - Energy is seeded, per lane and sample-serial, in `f64` (F4).
  - Needs R4. Prototype: 623-628 ns against 3,626-3,802 ns per 8-lane bank, bit-identical on
    every field.
  - Estimate, not measured in situ: most of the 31-34 µs ALL cost on the 64-track block.
- **S4. Window boundary inside a block.** Split the partial at a per-lane boundary with one
  frame-index mask instead of falling back. No shipped host produces such a block: the controlled
  policy requires `period % quantum == 0`, and the web period is blocks × quantum.

## Dependencies

- None hard.
- #881 should merge before this batch, so the batch-boundary run has a row that can move.
- #882 must not land before S1.
- #883 is independent.
- #884: see R2.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit and no published meter word: the same arithmetic
  up to the exact reassociation F4 proves, and fewer passes, loads and branches. Every gate that
  says "bit-identical" is a hard stop, not a tolerance.
- The owner's copy rule: this slice adds no block-sized copy (the pass is a read). It must not
  introduce a planar meter scratch.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh`
  is mandatory). `crates/graph` stays free of `unsafe`. Only `crates/lane` names `wide` or
  intrinsics.
- Run `cargo fmt --all --check`, the workspace clippy and the focused tests before every
  checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`.
- Do not quote a projected saving. The paired console benchmark runs once at the batch boundary.
  The AudioWorklet artifact pin and browser qualification are repinned once there too.

## Evidence: how F2 and F4 were measured

All harnesses were throwaway. They were run in the research worktree and reverted; none of this is
committed.

- **Host and build.** AMD EPYC 7313P (Zen 3, 16C/32T), `x86-64-v3` pinned by
  `.cargo/config.toml`, `--release`.
  - Runs were not CPU-pinned: the sandbox refused `taskset`.
  - Load average was 1.4-6.9 from concurrent agents. Alternation per block protects the paired
    deltas.
  - `graph` was built with `test-support`, as `tools/bench` builds it.
- **Console arms.**
  - A probe hook in `console-workload`'s `SessionRuntime::build_full` prepared builtins with
    `prepare_selected_session_builtins_with_console` or `..._between_render_calls`. It requested
    `SAMPLE_PEAK` or ALL at one tap on all 64 tracks, with period `4 * 128`, queue depth 8, hold 0
    and decay 0.
  - Each arm was warmed for 256 blocks, then all arms rendered in turn every round (#104 paired
    alternation), 4,000-6,000 rounds. Wall time per block used `Instant`.
  - Meters were drained outside the clock. Digests were asserted equal across arms, and meter
    snapshots equal between each banked/scalar pair.
- **P1 prototype.** P1's interface was wired exactly as above: the pass in `observe_unit`, the
  block field, and the meter fast path. A thread-local switch turned the pass off for the scalar
  arm, and the ALL row ran it without the opt-in.
- **Microbenchmark and bit-identity probes.** These were throwaway `builtins` integration tests:
  - 8 `MeterAccumulator`s over one strided 8-lane block.
  - The peak kernel written over `lane::Simd8` and `lane::Simd4`.
  - An AVX2-intrinsic ALL prototype (seeded `f64` energy, peak, held, counts), checked field by
    field against 8 scalar ALL meters over 64 blocks (16 windows) on hostile and tone input.
- **Recorded benchmark cross-check.** `artifacts/plumbing-floor-baseline/console-benchmark.accepted.jsonl`,
  record `console_meters`.

## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27. Branch `codex/943-banked-sample-peak`, base `925f47d6` (#881's
metered-row branch tip plus this brief). Code commit `1975fc44`; the follow-up commit carries this
record, the mutation rows and G2's invalid-only lanes (deviation 2). Host AMD EPYC 7313P (Zen 3), rustc 1.97.1, `.cargo/config.toml` pin
`+avx2,+fma`, `CARGO_INCREMENTAL=0`, the worktree's own `target/`. No timed console benchmark was
run and no saving is quoted as the issue's result; the A/B below is a throwaway, in-process,
descriptive run.

### Design

- **Kernel** (`crates/lane/src/kernels/builtins.rs`): `meter_sample_peak_block<L>(words, frames,
  peak) -> L`, `#[inline(always)]`, amendment 1's spelling: `for frame in
  words[..frames * L::WIDTH].chunks_exact(L::WIDTH)`, `a = |load|`, `c = select(a >= MIN_POSITIVE &
  a < INFINITY, a, +0.0)`, `peak = L::max(c, peak)`. The doc lists the frozen order, says the test is
  the meter's `normal_or_zero` and not `NONFINITE_LIMIT`, and states the reassociation argument.
- **Rack**: `BankChain::final_output_block(frames) -> Option<(&[f32], &[f32])>`, beside
  `final_output_lane`, with its shape checks minus the per-lane activity test.
- **Graph** (`crates/graph/src/{lib,runtime}.rs`):
  - `GraphResidentObservationBlock::sample_peak: Option<[f32; 2]>`, documented as the peak of the
    **sanitized** magnitude (amendment 4), `Some` only when the unit's pass ran, and to be ignored
    by an observer that did not ask for it.
  - `GraphRuntimeObserver::accepts_sample_peak(&self) -> bool { false }`, read at bind only.
  - `UnitIdentity::sample_peak: bool` in the existing padding, set in
    `new_with_observation_activation` beside `observed` from `RuntimeUnit::accepts_sample_peak`,
    which takes the final slot as `members.len().checked_sub(lanes)` (amendment 4).
  - The pass, in `observe_unit`'s bank arm: `if sample_peak && eligible`, before the member loop,
    `chain.final_output_block(frames)` then `bank_sample_peak(chain.width(), left, right)`. Each
    final member at lane `l` passes `Some([left[l], right[l]])` (read with `get`, so it cannot
    panic) to `observe`, which puts it in the resident block; every other member, the `Op` arm and
    `observe_one` (the controlled path) pass `None`.
  - `#[inline(never)] fn bank_sample_peak(width, left, right) -> Option<[[f32; 8]; 2]>`
    (amendment 2): one kernel call per plane at `Simd4` or `Simd8`, each seeded `L::zero()`,
    stored into a stack `[[f32; 8]; 2]`. It derives `frames` from the slice length and returns
    `None` unless both planes are the same whole number of frames, so the kernel cannot slice past
    a plane.
  - Test support: `test_only_set_bank_sample_peak_declined(bool)` (the oracle arm; also resets the
    count) and `test_only_bank_sample_peak_passes()`.
- **Builtins**: `MeterAccumulator::observe_input_with_block_peak(input, first_sample, block_peak)`;
  `observe_input` calls it with `None`. The fast path is the brief's, after the unchanged preamble:
  `Some(p)`, `metrics == SAMPLE_PEAK`, `len > 0`, `len <= period - frames`; merge each channel with
  `if p > peak { p } else { peak }`, add `len`, `emit()` at `frames == period`. Plus `metrics()` and
  the test-support merge counter `test_only_block_peak_merges()` / `test_only_reset_block_peak_merges()`.
- **Builtins-compiler**: `MeterObserver::accepts_sample_peak` is `metrics() == SAMPLE_PEAK`, and
  `observe_resident` passes `block.sample_peak`.
- No new `unsafe`, no `wide` outside `crates/lane`, `graph`'s dependencies unchanged, no retained
  byte (`rt9_identity_metadata_has_no_retained_or_peak_layout_delta` green), no block-sized copy.

### Deviations and decisions

1. **The wasm-gate corpus case is a lane case, not a tail case.** `meter_sample_peak_block/hostile`
   is appended to the lane-case block, after the element-wise cases (case 55): `LANE_DIGESTS` gains
   one entry at its end and every existing lane pin keeps its index. It is not at the global tail
   because `tools/wasm-gates/tests/g5_native_corpus.rs`, outside this issue's paths, pins the tail
   layout (`compressor_base + COMPRESSOR_CASE_COUNT + 2 == CASE_COUNT`). The delegated blocks after
   case 55 move up one global index; their pins are indexed in their own crates, and every
   `wasm-gates` test is green unchanged. The corpus doc says so. The case: per lane, 1,024 words
   from a seeded generator (every fourth window invalid-only: NaN payloads, infinities, zeros,
   subnormals; the others mixing those with `MIN_POSITIVE`, `f32::MAX`, `1e30` and moderate
   values), blocks of 1, 2, 3, 5, 16, 31, 64, 127, 128 and 129 frames, windows of three blocks;
   every frame's value is the running window peak through one kernel call over the block's prefix.
   Its pin `c108dcbd...e3e646` was taken from width 0 (the scalar `Lane` oracle) and equals widths 1
   and 2.
2. **G2's hostile family has invalid-only lanes.** Mutation L-1 (admit subnormals) was green on the
   first G2, because every hostile lane also carried normal words. Lanes 1 and 5 now carry only
   words the meter sanitizes to `+0.0`, their published peaks are asserted `+0.0`, and L-1 is red.
3. **The source scan pins the member call as one multi-line constant.** With the added argument
   rustfmt lays `observe(member, ..., peak)?` out vertically, so the test matches `MEMBER_CALL`
   exactly rather than one line. It still pins everything it pinned, and adds: one
   `.final_output_block(`, `bank_sample_peak(` exactly twice (definition and call), the gate
   `let peaks = if sample_peak && eligible {`, the block borrow and the pass call, the per-lane
   hand-off, and `sample_peak,` in `observe`'s block. New control rows: the pass forced on (the
   brief's), the peak withheld, the lane index shifted, the planes swapped, and the block borrow
   replaced.
4. **G4 uses the product period** (12 x 128 frames) and also pins the merge count and the window
   count.

### Gates

| gate | command | result |
|---|---|---|
| G1 | `cargo test -p lane --test meter_peak` | PASS, 4 tests: `Simd8` 3,072 and `Simd4` 1,536 lane-blocks bit-identical with the `f32` kernel and the meter's serial loop, and the seeded-zero partial merged with the select form equal to both; the invalid-only lane ends at `0x00000000`; the bit sweep (every exponent, both signs, boundary mantissas); the domain witness |
| G2 | `cargo test -p builtins --features test-support --test meter` (and without the feature: equality only) | PASS: 56,208 snapshots bit-identical on every field; merges 512 at periods 512 and 128, 296 at 300, 0 at 64, on hostile and tone input; `ALL` and `SAMPLE_PEAK \| COUNTS` merge 0 |
| G3 | `cargo test -p graph-compiler --lib post_matrix_peak_meters` | PASS, table below |
| G4 | `cargo test -p graph-compiler --lib the_banked_sample_peak_pass` | PASS: 1,000 blocks, `audit::snapshot().total() == 0`, passes 8,000 (`1,000 x 8` cohorts), merges 64,000, 5,312 windows (`64 x 83`) |
| G5 corpus | `bash scripts/run-wasm-gates.sh` | PASS: native, wasm scalar and wasm `simd128` legs each `cases 142, comparisons 358, mismatches [], minmax_lowering_mismatches 0` |
| G5 census | see below | recorded |
| G5 callgraph | the build script's cargo line into `target/web-943`, `wasm-objdump -d` piped into `check-web-audioworklet-callgraph.py` | PASS: `--callgraph miso_engine_web_v1_render`; `--kernel-shape --kernel-pattern '4wide6f32x[48]' --kernel-min 11` (unchanged); `--callgraph miso_engine_web_v1_meter_poll --trap-owner ...poll_meters`; `--callgraph miso_engine_web_v1_command_submit --allocation-only`; `--self-test` |
| G6 fmt | `cargo fmt --all --check` | PASS |
| G6 clippy | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | PASS |
| G6 doc | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | PASS |
| G6 suites | `cargo test --locked -p ...` | PASS: lane 56 (2 ignored); builtins 114 and 114 with `test-support`; rack 54; builtins-compiler `test-support` 79; graph 118 and 125 with `test-support`; graph-compiler 94; host-core `--all-features` 225; host-web 206; console-workload 39; capi 36; wasm-gate-corpus + wasm-gates 9 |
| G6 policy | `scripts/check-{lane-policy,unfused-seal,builtins-policy,builtins-fixtures,graph-policy,graph-determinism,realtime-policy,workspace-policy}.sh` | PASS, all eight (realtime: 56 marked regions in 16 files; determinism 100/100) |

The controlled-path probe tests (hazard 7, including `host-core/tests/observation_demand.rs`) are
green unchanged: P1 did not leak into S1.

**G3, per configuration** (24 blocks, pass on / declined):

| delivery, period | `[chains, slots]` | transposes | folds | redirects | frames | passes | merges |
|---|---|---:|---:|---:|---:|---:|---:|
| concurrent, 512 | [8, 48] | 192 | 64 | 0 | 384 | 192 / 0 | 1,536 / 0 |
| concurrent, 300 | [8, 48] | 192 | 64 | 0 | 640 | 192 / 0 | 896 / 0 |
| between render calls, 512 | [8, 48] | 192 | 64 | 0 | 384 | 192 / 0 | 1,536 / 0 |
| between render calls, 300 | [8, 48] | 192 | 64 | 0 | 640 | 192 / 0 | 896 / 0 |

In every configuration the PCM is bit-identical, every frame is equal on every field by bits, the
shape tuple is identical, the frames carry signal, and some window's left and right peaks differ.
Control 1: ALL-metric `MeterRequest` meters run 0 passes and 0 merges (folds 64). Control 2
(amendment 3): the web-shape plan bound through `into_bound_with_observation_activation` with one
active controlled observer on `ch00` `PostMatrix` runs 0 passes and 0 merges, the controlled row is
called on all 24 blocks, and the PCM and all 384 frames equal the declined arm's.

### wasm opcode census (G5, amendment 2)

Artifact: `scripts/build-web-audioworklet.sh`'s cargo line (`+simd128`, `-C strip=debuginfo`, both
remaps), `host_web.wasm` 3,308,878 bytes, sha256 `890705fa...9e54a9b2` (the pin `8934cdd9...` is
not repinned; that happens once at the batch boundary). `bank_sample_peak` is `func[2252]`, with
one call site, in `GraphExecutor::render` (`observe_unit` inlined), and makes no calls itself.

| opcode | count | | opcode | count |
|---|---:|---|---|---:|
| `f32x4.abs` | 6 | | `f32x4.pmax` | 2 |
| `v128.and` | 6 | | `f32x4.lt` | 4 |
| `i32x4.sub` | 6 | | `f32.*` (any scalar `f32` op) | 0 |
| `i32x4.lt_u` | 6 | | `f32x4.ge` | 0 |
| `v128.bitselect` | 10 | | `loop` | 4 |

- The validity test lowers to the exact integer range test `(bits & 0x7fffffff) - 0x00800000 <u
  0x7f000000`, as the verification found, so `f32x4.ge` never appears.
- The live `Simd4` arm (the browser's bank width) is two loops of `abs, and, sub, lt_u, bitselect,
  pmax`. The `Simd8` arm, dead on wasm, is two loops over two `v128` halves whose `max` lowers to
  `f32x4.lt` plus `bitselect`, the same D8 select form.
- Each of the four loops has exactly one `br_if`, its back edge: the per-frame body is branch free.
- Zero scalar `f32` compares, maxima or arithmetic in the function.

### In-process A/B on the metered row (throwaway, descriptive)

A temporary `console-workload` test, deleted afterwards: two `SessionRuntime`s of
`Workload::SixtyFourTrackConsoleMetered`, one with the pass and one with it declined, alternated per
block with the order swapped every block, 512 warm-up blocks then 6,000 timed, release, meters
drained outside the clock. Load average 1.4-1.9.

| run | p50 pass on | p50 declined | paired median (declined - on) | p10 / p90 of the delta |
|---|---:|---:|---:|---:|
| 1 | 133.6 us | 147.9 us | 14.3 us | 9.1 / 19.8 us |
| 2 | 132.1 us | 147.0 us | 14.8 us | 9.7 / 20.0 us |

Both arms rendered the same PCM digest over the 6,512 blocks, published the same 208,128 meter words
(peaks by bits, window numbers and bounds, handles), and the pass ran 52,096 times (`6,512 x 8`) on,
0 declined. This is not the benchmark row and is not the issue's result.

### Red mutations

Nine rows, each applied alone and restored: the graph rows to `1975fc44`, the lane and builtins rows
to `1975fc44` plus this commit's final G2 (deviation 2). Recorded in
`crates/{lane,builtins,graph}/tests/MUTATIONS.md`.

| # | mutation | red on |
|---|---|---|
| L-1 | `a.ge(MIN_POSITIVE)` becomes `a.ge(zero)` | G1 (3 of 4), G2, corpus case 55 |
| L-2 | drop `a.lt(INFINITY)` | G1 (3 of 4), G2, corpus case 55 |
| B-1 | window test becomes `true` | G2 (period 300: publication count) |
| B-2 | fast path on `metrics.contains(SAMPLE_PEAK)` | G2 (the ALL arm's words) |
| B-3 | fast path skips `emit()` | G2 (period 512: publication count) |
| G-1 | identity flag forced `true` | G3's ALL-control pass counter only (`192` against `0`) |
| G-2 | final lane index `lane ^ 1` | G3 frames; the source scan |
| G-3 | right-plane pass reads the left plane | G3 frames (right peak) |
| expected green | `L::max(c, peak)` becomes `L::max(peak, c)` | green on G1, G2 and the corpus, as the domain argument predicts |

### For the verifier

- The corpus placement (deviation 1) is the one choice outside the brief's letter; it keeps every
  file outside `tools/wasm-gate-corpus` untouched.
- `sample_peak: Some` reaches every observer on a final member, not only meters; the field doc says
  to ignore it unless asked for, and `MeterObserver` re-checks `metrics == SAMPLE_PEAK`.
- The pass runs for a unit whenever one final-slot observer accepts, including blocks whose windows
  then take the sample loop (a window boundary inside the block, S4). It never runs in a plan bound
  with an activation (control 2).
