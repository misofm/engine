# Record the swap block's cost on the 64-track console

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-17; plan risk 5).
Slice 16 of *Swap a rebuilt plan without an audio gap* (#1269). Tooling and evidence only: no engine
change. Code anchors verified on `main` at `6fb211594`. It may run beside stream A's current
feature slice.

## Product outcome

The owner, stream C and the weekly performance pass get one measured answer on the 64-track
app-shape console at native 8-lane (AVX2), the primary benchmark target, and **the single
derivation** of every warm-successor constant. It measures, against an ordinary render call:
- the call that applies a carrying swap (move mode);
- the call that copies state into a successor (copy mode, *Carry plan state by copy as well as by
  move*, #1322);
- a successor rendering real, non-silent material on a second thread, and the ordinary render calls
  beside it.

D3 below is the only place where `K_MAX_BLOCKS`, `CATCH_UP_DEADLINE_SAMPLES`, `P_MAX_SAMPLES`, the
ring headroom, `COPY_BYTES_MAX` and `CATCH_UP_SLICE_BLOCKS` are defined. *Fall back from a missed
catch-up deadline: bounded render-thread pre-roll, then the transition* (#1358) D1-D2, *Snapshot a
running plan into a returned successor at a block* (#1354) D3 and *Run the C ABI catch-up from
miso_engine_v1_service and report its outcome* (#1360) D2 freeze the values from this record and
*Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331) D7 by
D3's formulas; they restate no formula. The record also says whether a whole-bank move (#1269,
Deferred) earns a brief.

## Context

- The native console benchmark is `tools/bench` (`src/console.rs`; subjects at
  `tools/bench/src/main.rs:15`) over the rows of `tools/console-workload/src/lib.rs`.
  - It runs only through `scripts/operator/run-console-benchmark.sh --step NAME`: one untimed
    warmup, two measured rounds, and records under `artifacts/steps/<NAME>/`.
  - `scripts/operator/preflight-console-benchmark.sh` checks everything that can fail without
    timing.
  - `tools/console-workload` links no host crate. It builds plans with the graph compiler
    directly, so it cannot prepare a successor.
- Successor preparation and the carry live in host-core (#1272, and #1276-#1284). The 64-track
  app-shape fixture is `fixtures/session/v1/console-sixty-four-track-app.json` (48 kHz, quantum
  128).
- **Silence is cheap.** Silent lanes take the silent fast path (#1085's
  `InputSignal::{Tone, Silence, OddTracksSilent}`, `tools/console-workload/src/lib.rs:482`). A
  copied successor's carried sources stay vacant until adoption (#1322 D2(c)), so a copied
  successor rendered as it is would time silence. The tone is `source_block(track, false)`
  (`:2046`).
- The default source ring is `ceil(100 ms · fs / quantum) + 2` quanta
  (`default_source_ring_frames`, `crates/host-core/src/prepare.rs:65-77`;
  `SOURCE_STALL_TOLERANCE_MS`, `:57`). A ring peek gates release (#1320), so a catch-up that lags
  render holds ring frames for as long as it lags.
- #1355 D5 publishes once the successor's clock reaches `render_clock() + 2 · quantum`; its prime
  phase first reads `P` source frames into the source-claim lines (#1355 D3).
- AGENTS.md: benchmarks are descriptive. Freeze the workload and the validator before timing. One
  invocation, one warmup, two measured rounds, no tuning and no retry. A runner defect moves to a
  tooling issue.

## Decisions frozen for this slice

- **D1. Subject.** Every track reads the tone through host-core source rings that the bench fills
  before each block, outside the timed call. Rate 96 kHz (the highest launch rate, where a block's
  cost is the largest share of its quantum) and quantum 128 (Web Audio's quantum).
  - Session A is the 64-track fixture at 96 kHz. Session B is A plus one muted track whose ID sorts
    first, on an existing source, so every bank shifts a lane: the worst carry shape for lane
    copies. Session C is A plus a true-peak limiter insert on the output, the latency-growth shape.
  - All three are prepared through host-core as the C ABI prepares them, with live lanes.
- **D2. Measurement.** One run, three phases, each timing one render call per observation:
  1. **Move:** alternate A → B → A …, with one carrying swap every 32 blocks.
  2. **Copy:** every 32 blocks, copy the running plan into a prepared successor with #1322's
     `copy_predecessor_state`, and time that block's render call plus the copy call, and the copy
     call alone.
  3. **Successor:** a second thread renders C forward as fast as it can through #1321's
     `OffThreadPlanRenderer` (FP environment pinned), fed the same tone through C's own source
     rings, which that thread fills. Meanwhile the render thread renders A's ordinary blocks.
     Record the second thread's per-block times and blocks per second, and the render thread's
     ordinary block times.

  Successors are prepared off the timed path. Every timed block's output peak is recorded
  (untimed). The record reports every distribution (p50, p90, p99, max), the bytes per copy
  (`carry_program_copy_bytes`), the count of silent successor blocks, and the host and build facts
  the console records already carry.
- **D3. The derivation (the only one).** Notation: `q = 128`, `D = q / fs` at 96 kHz,
  `ceil_q(x)` rounds `x` samples up to whole quanta. Per measured row:
  - `c` = the larger of the render thread's ordinary-block p99 in phase 3 and the second thread's
    successor-block p99 (native), or `D / h` for the slowest engine's #1331 D7 headroom `h`
    (browser);
  - `r` = successor render samples per render-clock sample under contention: the second thread's
    blocks per second times `q / fs` (native), or `h` (browser). `r_min` is the smaller row.

  The constants, each the tighter over the native and browser rows:
  1. `K_MAX_BLOCKS = max(0, floor(D / c) − 1)`: the largest `k` whose `k` pre-rolled successor
     blocks plus the adoption block fit one quantum, `(k + 1) · c <= D`.
  2. `P_MAX_SAMPLES(fs) = 4 · ceil_q(L_max(fs))`, where `L_max(fs)` is the largest declared latency
     of any launch native effect at `fs` over its quality modes (the true-peak limiter's
     `Fs/100 + 6`, `crates/true-peak-limiter/src/lib.rs:242`, unless the record finds a larger
     one). That is four of the largest growths between host-declared discontinuities.
  3. `T_need(fs) = ceil_q((2 · q + P_MAX_SAMPLES(fs)) / (r_min − 1))`: render samples a catch-up
     needs to reach #1355 D5's publication point, counting the prime phase as if it rendered
     `P_MAX_SAMPLES` (an over-count, so the bound is safe).
     `CATCH_UP_DEADLINE_SAMPLES(fs) = ceil_q(fs · SOURCE_STALL_TOLERANCE_MS / 1000) + 2 · T_need(fs)`:
     one stall tolerance for the host's service cadence (the same allowance the ring gives a
     producer thread), plus twice the measured need. If `r_min <= 1`, the record states that the
     catch-up cannot lead on that row and no deadline is derived; #1358 then stops for a root
     ruling.
  4. **Ring headroom.** `default_source_ring_frames` grows by
     `CATCH_UP_DEADLINE_SAMPLES(fs) + P_MAX_SAMPLES(fs)`: a catch-up that lags render until its
     deadline, at `ΣP = P_MAX_SAMPLES`, still leaves the producer the full stall tolerance. The
     record reports the ring bytes of session A before and after, at each launch rate.
  5. `COPY_BYTES_MAX = floor((D − ordinary_p99) / copy_call_p99 · copy_bytes)`, from phase 2: the
     largest copy whose time, scaled linearly from the measured copy (its fixed cost charged to its
     bytes, so the bound is safe), fits one quantum beside an ordinary block.
  6. `CATCH_UP_SLICE_BLOCKS = max(1, floor(D / c))`: the most successor blocks one service call
     renders while staying under one quantum.

  Values at 44.1, 48 and 88.2 kHz come from the same formulas with the 96 kHz ratios `c / D` and
  `r`, which are the worst case. Quanta other than 128 are outside this measurement, and the
  record says so.
- **D4. Home.** A new `bench swap` subcommand in `tools/bench` (add `host-core` to its
  dependencies), with its own validator, and a `--swap` mode of the operator runner, or another
  home the bench policy accepts. `bash scripts/check-bench-policy.sh` must pass either way.
- **D5. Order.**
  - First commit: the subcommand, the validator, a short untimed self-test and the preflight, with
    no timed run.
  - Second commit: one operator run (`run-console-benchmark.sh` with the swap mode and
    `--step swap-carry-base`), its record, and a short report. The report states the per-lane
    overhead of the swap block and of the copy block, and every D3 value at each launch rate.
  - No tuning between them.

## Deliverables

1. D1-D5.
2. `artifacts/steps/swap-carry-base/` with the record and the report.
3. A comment on #1269, #1287, #1354, #1358 and #1360 with the move and copy block p50 and p99
   against the ordinary p50 and p99, and D3's values.

## Authorized paths

- `tools/bench/` (the new subcommand and the `Cargo.toml` dependency)
- `scripts/operator/run-console-benchmark.sh`, `scripts/operator/preflight-console-benchmark.sh`
- new `scripts/*swap*` validator files, `scripts/check-bench-policy.sh` (only if the policy must
  learn the new subcommand), `scripts/test-bench-policy.sh`
- `artifacts/steps/swap-carry-base/`

## Non-goals

- No optimisation of the carry. A large number opens a weekly-pass issue; it is not chased here.
- No browser number (#1331), and no AArch64 timing (that waits for funding).
- No change to any engine constant: #1354, #1358 and #1360 write D3's values into code.

## Objective gates

1. The preflight passes. The self-test renders A and B through one move swap and one copy, and C
   for 8 blocks on a second thread, without timing; every C block has a nonzero output peak.
2. The validator refuses:
   - a record with a missing distribution or a missing D3 value;
   - a swap or copy count different from the frozen one;
   - a run with no carrying swap or no carrying copy (`carry != Carried`);
   - a run with any silent successor block, or an input other than the tone.
3. Exactly one timed invocation, recorded with its warmup and its two rounds.
4. Commands:
   - `cargo test --locked -p bench`
   - `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh`
   - the preflight of the home D4 chose (for the console runner,
     `bash scripts/operator/preflight-console-benchmark.sh` with the swap mode's flag and
     `--step swap-carry-base`)
   - `bash scripts/check-workspace-policy.sh`, `cargo fmt --all -- --check`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 2: a run whose successors silently stopped carrying (every swap or copy cold) would time the
  wrong thing; so would a successor rendering silence on the fast path (vacant sources). A record
  without D3's values would leave #1354, #1358 and #1360 without their bounds. The validator turns
  red on each.

## Dependencies

- *Carry strip delay lines and live send ramps across a plan swap* (#1284)
- *Carry plan state by copy as well as by move* (#1322)
- *Render a successor plan off the render thread with a pinned floating-point environment* (#1321)
- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331)

#1284 makes every state family carry in both modes, so the measured carry is the real one.
Phase 3 renders on #1321's real off-thread renderer. D3's browser row takes the slowest engine's
headroom `h` from #1331 D7.
