# Record the swap block's cost on the 64-track console

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-17; D15-8 (round-5 amendment); plan risk 5).
Slice 16 of *Swap a rebuilt plan without an audio gap* (#1269). Tooling and evidence only: no engine
change. Code anchors verified on `main` at `6fb211594`. It may run beside stream A's current
feature slice.

## Product outcome

The owner, stream C and the weekly performance pass get one measured answer on the 64-track
app-shape console at native 8-lane (AVX2), the primary benchmark target, and **the single
derivation** of every warm-successor constant. It measures, against an ordinary render call:
- the call that applies a carrying swap (move mode);
- the call that adopts a primed warm successor (*Adopt a warm successor with a raw-frame prime at
  the first ready block*, #1355): the first growth, and the growth that brings `ΣP` to `P_MAX`.

D3 below is the only place where `p_max_samples`, the ring headroom and `PRIME_BYTES_MAX` are
defined. *Grow the default source ring by the warm-prime headroom* (#1406) writes `p_max_samples`
and the ring headroom into code, and *Check the warm-successor deadline in
miso_engine_v1_service and report its outcome* (#1360) writes `PRIME_BYTES_MAX` and passes both
bounds to *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354)'s
`WarmConfig`. None restates a formula. `p_max_samples` and the ring headroom are formulas
that need no timed run; only `PRIME_BYTES_MAX` needs this record. The record also says whether a
whole-bank move (#1269, Deferred) earns a brief.

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
  `InputSignal::{Tone, Silence, OddTracksSilent}`, `tools/console-workload/src/lib.rs:482`), so a
  run on silence would time the fast path. The tone is `source_block(track, false)` (`:2046`).
- The default source ring is `ceil(100 ms · fs / quantum) + 2` quanta
  (`default_source_ring_frames`, `crates/host-core/src/prepare.rs:65-77`;
  `SOURCE_STALL_TOLERANCE_MS`, `:57`). *Prepare a warm successor whose carried nodes lead the
  predecessor by P* (#1354) D1 names that body `stall_ring_frames(fs, q)`, and #1406 D2 makes the
  default `stall_ring_frames(fs, q) + p_max_samples(fs, q) + q`. A host sets the ring itself with
  `HostPrepareCaps::source_ring_frames` (`crates/host-core/src/prepare.rs:106`).
- **Rounding slack carries.** `Δ` is taken against the floored arrivals `a(n)` of the previous
  growth, so `ΣP` after `j` growths is `ceil_q` of track 1's natural arrival, not the sum of each
  growth's `ceil_q`. With the true-peak limiter alone (`L = 966` at 96 kHz), four limiters arrive at
  3,864 and the growths are 1,024, 1,024, 896 and 1,024: `ΣP = 3,968`, short of
  `p_max_samples = 4,096` (at 44.1 kHz the same chain gives 1,792 against 2,048). The soft clipper
  (`miso.soft-clip`) declares 31 samples at every rate (`crates/soft-clip/src/lib.rs:178`), and
  its latency counts in PDC. A track's `delay_samples` does not
  (`crates/graph-compiler/src/pdc.rs:12-17`), so it cannot pad an arrival.
- **Prime adoption** (D15-8 (round-5 amendment)). A warm successor is published `Primed`. Render
  adopts it at the first block at or after `not_before` whose readiness check passes, and in that
  block each carried source consumer replays its next `k = P / q` blocks (`prime_block_at`, #1320)
  and render fills the claim lines from the predecessor's pending frames and the prime (#1287 L2,
  L3). #1354 D3's `prime_bytes()` is the exact byte count of that work. Nothing is rendered ahead
  and nothing is copied.
- AGENTS.md: benchmarks are descriptive. Freeze the workload and the validator before timing. One
  invocation, one warmup, two measured rounds, no tuning and no retry. A runner defect moves to a
  tooling issue.

## Decisions frozen for this slice

- **D1. Subject.** Every track reads the tone through host-core source rings that the bench fills
  before each block, outside the timed call, at least `P + q` frames ahead of the read position so
  that a prime is always ready. Rate 96 kHz (the highest launch rate, where a block's cost is the
  largest share of its quantum, and where `P_MAX` is largest) and quantum 128 (Web Audio's
  quantum).
  - Session A is the 64-track fixture at 96 kHz. Session B is A plus one muted track whose ID sorts
    first, on an existing source, so every bank shifts a lane: the worst carry shape for lane
    copies.
  - **The growth chain (the one construction; #1355 gate 6 uses it at every rate).** Let
    `L = L_max(fs)` (D3 item 1; the true-peak limiter's `fs/100 + 6`,
    `crates/true-peak-limiter/src/lib.rs:236-242`, unless the record finds a larger one),
    `K = ceil_q(L)`, and `u = L + 31 · c(fs)`, where `c(fs)` soft clippers (31 samples at every
    rate, `crates/soft-clip/src/lib.rs:178`) pad one limiter. Growth `j` (1 to 4) adds one limiter
    and `c(fs)` soft clippers in series on track 1, so track 1 arrives at `j · u`. Every growth is
    then a warm growth of `P = K` and `ΣP` after growth `j` is `j · K` exactly when
    `ceil_q(j · u) = j · K` for `j = 1..4`, that is when `K − q/4 < u <= K`. `c(fs)` is the
    smallest count that meets it:

    | fs | `L` | `K` | `c` | `u` | `j · u` (j = 1..4) | `ceil_q` | `ΣP` after 4 = `p_max_samples` |
    |---|---|---|---|---|---|---|---|
    | 44.1 kHz | 447 | 512 | 2 | 509 | 509, 1,018, 1,527, 2,036 | 512, 1,024, 1,536, 2,048 | 2,048 |
    | 48 kHz | 486 | 512 | 0 | 486 | 486, 972, 1,458, 1,944 | 512, 1,024, 1,536, 2,048 | 2,048 |
    | 88.2 kHz | 888 | 896 | 0 | 888 | 888, 1,776, 2,664, 3,552 | 896, 1,792, 2,688, 3,584 | 3,584 |
    | 96 kHz | 966 | 1,024 | 1 | 997 | 997, 1,994, 2,991, 3,988 | 1,024, 2,048, 3,072, 4,096 | 4,096 |

    `c = 0` fails at 44.1 kHz (`u = 447 <= 480`) and at 96 kHz (`u = 966 <= 992`); `c = 3` at
    44.1 kHz and `c = 2` at 96 kHz exceed `K` (540 and 1,028). The `Δ` of each step against the
    previous floored arrival lies in `(K − q, K]` (at 96 kHz: 997, 970, 943 and 916), so each `P`
    is `K`. If the record finds a larger `L_max`, it recomputes `c(fs)` by the same rule and states
    it. A has no latency anywhere (its console EQ and compressor declare 0,
    `crates/parametric-eq/src/lib.rs:652`, `crates/compressor/src/lib.rs:295`), so these are the
    arrivals.
  - Sessions `C_1` to `C_4`: `C_j` is A after growths 1 to `j` of the chain at 96 kHz (one limiter
    and one soft clipper each). Each step `C_(j-1) -> C_j` restarts track 1 and carries every
    other node at `a(n) + P` (C1 holds, because the floor dominates every other node's arrival),
    and `C_4` reaches `p_max_samples(96 kHz, 128) = 4,096`.
  - Every source ring is set explicitly through `HostPrepareCaps::source_ring_frames` to
    `stall_ring_frames(96 kHz, 128) + p_max_samples(96 kHz, 128) + 128 = 9,856 + 4,096 + 128 =
    14,080` frames, so each growth has #1354 D2 step 7's headroom whether or not #1406 has changed
    the default; a default ring before #1406 would make every growth `LeadBound`.
  - All sessions are prepared through host-core as the C ABI prepares them, with live lanes; each
    `C_j` through #1354's warm preparation, with a `WarmConfig` whose `prime_bytes_max` is
    unbounded, since this run is what sets it.
- **D2. Measurement.** One run, two phases, each timing one render call per observation:
  1. **Move:** alternate A → B → A …, with one carrying swap every 32 blocks.
  2. **Prime:** 32 cycles. A cycle starts from a freshly prepared A on a fresh exchange (untimed)
     and grows A → `C_1` → `C_2` → `C_3` → `C_4`, one growth every 32 blocks. Each `C_j` is
     published `Primed` with `not_before` at the next block, and the timed observation is the
     render call that adopts it.

  Successors are prepared off the timed path. Every timed block's output peak is recorded
  (untimed). The record reports every distribution (p50, p90, p99, max) of the ordinary block, the
  move block and the adoption block of each growth step, the `prime_bytes()` of each growth step,
  the adoption block of each growth (which must be the block at `not_before`), and the host and
  build facts the console records already carry.
- **D3. The derivation (the only one).** Notation: `q = 128`, `fs = 96 kHz` (session A's rate),
  `D = q / fs`, `ceil_q(x)` rounds `x` samples up to whole quanta. From phase 1: `o`, the ordinary
  block's p99. From phase 2: `b_P`, the `prime_bytes()` of the growth `C_3 -> C_4` (at
  `ΣP = p_max_samples`), and `a_P`, its adoption block's p99. #1331 D7's headroom `h` is measured
  on the browser's 64-track app session at its own rate `fs_h` (48 kHz, as every browser
  qualification session runs, e.g. `hosts/host-web/qualification/live-control-session.json:5`);
  the record states `fs_h` beside `h`.
  1. `p_max_samples(fs, q) = 4 · ceil_q(L_max(fs))` at the session's quantum `q` (the notation's
     `q = 128` gives this record's values; at 96 kHz and `q = 127` it is `4 · 1,016 = 4,064`), so it
     is a whole number of quanta at every quantum, where `L_max(fs)` is the largest declared latency
     of any launch native effect at `fs` over its quality modes (the true-peak limiter's
     `Fs/100 + 6`, `crates/true-peak-limiter/src/lib.rs:236-242`, unless the record finds a larger
     one). That is four of the largest growths between host-declared discontinuities.
  2. **Ring headroom.** `default_source_ring_frames` grows by `p_max_samples(fs, q) + q`: a source
     whose read position leads render by `ΣP <= p_max_samples`, and whose next prime needs `P + q`
     frames queued, still leaves the producer the full stall tolerance. There is no deadline term:
     render reads nothing ahead while it waits for readiness. The record reports the ring bytes of
     session A before and after, at each launch rate.
  3. **`PRIME_BYTES_MAX`.** `β = (a_P − o) / b_P` is the adoption's extra time per prime byte, with
     the swap's and the prime's fixed costs charged to the bytes, so a prime larger than `b_P` is
     not under-estimated.
     - **Margin.** The adoption block may use at most `m = 0.7` of its quantum, the release
       gate's callback ceiling (*026 End-to-end release, performance, and listening
       qualification*, #26: P99.99 callback time below 70% of the quantum). The rest is the
       host's own work and the scheduler's jitter, which this bench does not time.
     - Native row: the adoption block fits when `o + β · b <= m · D`, so
       `b_native = floor((m · D − o) / β)`.
     - Browser row: the browser's ordinary block takes `1 / h` of its quantum at `fs_h` (#1331 D7).
       Taking the native ratio `(o + β · b) / o` of the adoption block to the ordinary block as the
       browser's, the adoption block fits when `(o + β · b) / o <= m · h`, so
       `b_browser = floor((m · h − 1) · o / β)`. The record states this ratio assumption. A byte
       count does not depend on the rate, so no rate conversion enters this row.
     - `PRIME_BYTES_MAX = min(b_native, b_browser)`. If it is below `b_P`, or `m · h <= 1`, the
       measured `P_MAX` prime does not fit the margin on that row: the record says so and derives
       no value, and the slice that writes it stops for an owner ruling. (Scaling down from `b_P`
       would charge the fixed costs too little.)
     - The estimate for the 64-track console at 96 kHz is about 512 KiB for the first growth and up
       to 2 MiB at `ΣP = p_max_samples`. The record confirms or corrects it; it is not a gate.

  `p_max_samples` and the headroom at 44.1, 48 and 88.2 kHz, and at every other quantum, come from
  the same formulas.
  `PRIME_BYTES_MAX` is one byte count for every rate: 96 kHz has the largest primes and the
  largest `o / D`. Quanta other than 128 are outside its timed measurement, and the record says so.
- **D4. Home.** A new `bench swap` subcommand in `tools/bench` (add `host-core` to its
  dependencies), with its own validator, and a `--swap` mode of the operator runner, or another
  home the bench policy accepts. `bash scripts/check-bench-policy.sh` must pass either way.
- **D5. Order.**
  - First commit: the subcommand, the validator, a short untimed self-test and the preflight, with
    no timed run.
  - Second commit: one operator run (`run-console-benchmark.sh` with the swap mode and
    `--step swap-carry-base`), its record, and a short report. The report states the per-lane
    overhead of the move block and of the adoption block at the first growth and at
    `ΣP = p_max_samples`, the measured `prime_bytes()` against the estimate, and every D3 value at
    each launch rate.
  - No tuning between them.

## Deliverables

1. D1-D5.
2. `artifacts/steps/swap-carry-base/` with the record and the report.
3. A comment on #1269, #1287, #1358, #1360 and #1406 with the move and adoption block p50 and p99
   against the ordinary p50 and p99, and D3's values.

## Authorized paths

- `tools/bench/` (the new subcommand and the `Cargo.toml` dependency)
- `scripts/operator/run-console-benchmark.sh`, `scripts/operator/preflight-console-benchmark.sh`
- new `scripts/*swap*` validator files, `scripts/check-bench-policy.sh` (only if the policy must
  learn the new subcommand), `scripts/test-bench-policy.sh`
- `artifacts/steps/swap-carry-base/`

## Non-goals

- No optimisation of the carry or the prime. A large number opens a weekly-pass issue; it is not
  chased here.
- No browser number (#1331), and no AArch64 timing (that waits for funding).
- No change to any engine constant: #1406 and #1360 write D3's values into code.

## Objective gates

1. The preflight passes. The self-test, without timing, renders A and B through one move swap,
   and one cycle A → `C_1` → … → `C_4` through four primed adoptions: each adoption happens at its
   `not_before` block with `carry == Carried` and `P == ceil_q(L_max)`, `C_4`'s `ΣP` equals
   `p_max_samples(96 kHz, 128)`, and
   every block has a nonzero output peak.
2. The validator refuses:
   - a record with a missing distribution or a missing D3 value;
   - a swap or adoption count different from the frozen one;
   - a run with no carrying swap (`carry != Carried`), or an adoption with `prime_bytes() == 0`;
   - an adoption later than its `not_before` block (the prime was not ready, so the run timed
     something else), or a `C_4` whose `ΣP` is not `p_max_samples(96 kHz, 128)`;
   - a run with any silent timed block, or an input other than the tone.
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

- Gate 2: a run whose swaps silently stopped carrying (every swap cold) would time the wrong thing;
  so would an adoption that primed nothing, one deferred because the bench fed its rings too late,
  a growth chain that never reached `P_MAX`, or blocks on the silent fast path. A record without
  D3's values would leave #1406 and #1360 without their bounds. The validator turns red on each.

## Dependencies

- *Carry strip delay lines and live send ramps across a plan swap* (#1284)
- *Prepare a warm successor whose carried nodes lead the predecessor by P* (#1354)
- *Adopt a warm successor with a raw-frame prime at the first ready block* (#1355)
- *Prove two Wasm instances on one shared memory in three browser engines and on iOS* (#1331)

#1284 makes every state family carry, so the measured move is the real one. Phase 2 prepares with
#1354 and adopts on #1355's real prime path. D3's browser row takes the slowest engine's headroom
`h` from #1331 D7.
