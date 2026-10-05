# Record the swap block's cost on the 64-track console

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8; plan risk 5).
Slice 16 of *Swap a rebuilt plan without an audio gap* (#1269). Tooling and evidence only: no engine
change. Code anchors verified on `main` at `6fb211594`. It may run beside stream A's current
feature slice.

## Product outcome

The owner, stream C and the weekly performance pass get one measured answer on the 64-track
app-shape console at native 8-lane (AVX2), the primary benchmark target. It measures how long these
render calls take, compared with an ordinary render call:
- the call that applies a carrying swap (move mode);
- the call that copies state into a warm successor (copy mode, *Carry plan state by copy as well as
  by move*, #1322);
- an ordinary call while a second thread renders a successor forward.

From these numbers it derives the native inputs that D15-8's fallbacks need: `k_max`, the
catch-up deadline and the ring headroom behind `P_max` (plan risk 5). *Pre-roll a successor whose
latency grows* (#1287) freezes the three constants from them. For each constant it takes the
tighter of this record and the browser measurement of *Prove two Wasm instances on one shared
memory in three browser engines and on iOS* (#1331). The record also says whether a whole-bank move
(#1269, Deferred) earns a brief.

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
  app-shape fixture is `fixtures/session/v1/console-sixty-four-track-app.json`.
- The default source ring is `ceil(100 ms · fs / quantum) + 2` quanta
  (`default_source_ring_frames`, `crates/host-core/src/prepare.rs:58-77`). That is the headroom the
  warm successor's read-ahead must share with the producer's stall tolerance.
- AGENTS.md: benchmarks are descriptive. Freeze the workload and the validator before timing. One
  invocation, one warmup, two measured rounds, no tuning and no retry. A runner defect moves to a
  tooling issue.

## Decisions frozen for this slice

- **D1. Subject.** Session A is the 64-track app-shape fixture. Session B is A plus one muted track
  whose ID sorts first, on an existing source, so every bank shifts a lane. That is the worst carry
  shape for lane copies. Both are prepared through host-core as the C ABI prepares them, with live
  lanes.
- **D2. Measurement.** One run has three phases, each timing one render call per observation:
  1. **Move:** alternate A → B → A …, with one carrying swap every 32 blocks.
  2. **Copy:** every 32 blocks, copy the running plan into a prepared successor and time that
     block's render call plus the copy call.
  3. **Catch-up:** a second thread renders a copied successor forward as fast as it can, and its
     output is discarded. Meanwhile the render thread renders the predecessor's ordinary blocks.
     Record the second thread's blocks per second and the render thread's ordinary block times.

  Successors are prepared off the timed path. The record reports:
  - every distribution (p50, p90, p99, max);
  - the bytes and moves per carry (`carry_program_copy_bytes`);
  - the host and build facts the console records already carry.
- **D3. Derived inputs** (reported, not frozen here), at 48 kHz and a 128-frame quantum, with `D`
  the quantum's duration:
  - `k_max`: the largest `k` with `copy_p99 + k · ordinary_p99 ≤ D`. This is the render-thread
    pre-roll fallback's bound.
  - `r`: the catch-up thread's blocks per `D`, under contention. The render samples needed to lead
    by `P` are `P / (r − 1)`. If `r ≤ 1`, the record states that the catch-up cannot lead on this
    machine.
  - Ring headroom: the default ring's frames minus its stall frames, at each launch rate, beside the
    `P` of a true-peak limiter (`Fs/100 + 6`, rounded up to whole quanta) at each launch rate.
- **D4. Home.** A new `bench swap` subcommand in `tools/bench` (add `host-core` to its
  dependencies), with its own validator, and a `--swap` mode of the operator runner, or another
  home the bench policy accepts. `bash scripts/check-bench-policy.sh` must pass either way.
- **D5. Order.**
  - First commit: the subcommand, the validator, a short untimed self-test and the preflight, with
    no timed run.
  - Second commit: one operator run (`run-console-benchmark.sh` with the swap mode and
    `--step swap-carry-base`), its record, and a short report. The report states the per-lane
    overhead of the swap block and of the copy block, and D3's derived inputs.
  - No tuning between them.

## Deliverables

1. D1-D5.
2. `artifacts/steps/swap-carry-base/` with the record and the report.
3. A comment on #1269 and on #1287 with the move and copy block p50 and p99 against the ordinary
   p50 and p99, and D3's derived inputs.

## Authorized paths

- `tools/bench/` (the new subcommand and the `Cargo.toml` dependency)
- `scripts/operator/run-console-benchmark.sh`, `scripts/operator/preflight-console-benchmark.sh`
- new `scripts/*swap*` validator files, `scripts/check-bench-policy.sh` (only if the policy must
  learn the new subcommand), `scripts/test-bench-policy.sh`
- `artifacts/steps/swap-carry-base/`

## Non-goals

- No optimisation of the carry. A large number opens a weekly-pass issue; it is not chased here.
- No browser number (#1331), and no AArch64 timing (that waits for funding).
- No freezing of `k_max`, the deadline or `P_max` (#1287).

## Objective gates

1. The preflight passes, and the self-test renders A and B through one move swap and one copy
   without timing.
2. The validator refuses:
   - a record with a missing distribution or a missing derived input;
   - a swap or copy count different from the frozen one;
   - a run with no carrying swap or no carrying copy (`carry != Carried`).
3. Exactly one timed invocation, recorded with its warmup and its two rounds.
4. Commands:
   - `cargo test --locked -p bench`
   - `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh`
   - the preflight of the home D4 chose (for the console runner,
     `bash scripts/operator/preflight-console-benchmark.sh` with the swap mode's flag and
     `--step swap-carry-base`)
   - `bash scripts/check-workspace-policy.sh`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Test value

- Gate 2: a run whose successors silently stopped carrying (every swap or copy cold) would time the
  wrong thing. A record without D3's inputs would leave #1287 without its bounds. The validator
  turns red on either.

## Dependencies

- *Carry strip delay lines and live send ramps across a plan swap* (#1284): every state family
  carries in both modes, so the measured carry is the real one.
- *Carry plan state by copy as well as by move* (#1322).
