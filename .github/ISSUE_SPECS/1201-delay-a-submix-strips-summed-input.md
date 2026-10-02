# Delay a submix strip's summed input

Slice 04 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K1.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A submix's `delay_samples` now delays its **summed** input, per lane, before the bus's input section
runs: the delay DiGiCo, S6L and SSL give a bus. Like a track's delay, it is a musical time shift
that plugin-delay compensation never compensates.

## Context (verified on `fe8ac679`; slices 01-03 changed the sites named "after")

- **After *Render a submix strip on its summed input* (#1200):** a submix lowers to the track's
  `TrackStage` chain keyed by its ID; its `Input` stage has no source binding and lowers to
  `NodeKind::Identity`, a D9 reduction of the routes that target it. That slice deliberately left a
  submix's `delay_samples` unlowered (its D5, an in-batch interim this slice ends).
- **The track delay.**
  - The graph compiler emits one `PreparedTrackDelay { node, left_samples, right_samples }`
    (`crates/graph/src/lib.rs:2221-2228`; `node` is "the `TrackStage::Input` node this delay is
    applied at") per track with a nonzero lane delay (`crates/graph-compiler/src/compile.rs:460-471`,
    track-only with a source-semantics comment since slice 01).
  - Their bytes are charged to the estimate (`compile.rs:472-484`, passed to `resource_estimate` at
    `:485-497`).
  - At bind, `node_kind` (`crates/graph/src/runtime.rs:4025-4067`) takes the entry out of its
    `track_delays` map (`:3906`) **only inside the source-input branch** (`:4026-4041`), producing
    `NodeKind::TrackDelay { line, channels_agree }` (`:805-813`) with a `TrackDelayLine`
    (`:720-725`, `process` `:762-768` inside a `REALTIME_POLICY` region).
  - `execute_op` (`:2970-3060`) runs the `TrackDelay` line in place and **returns before any
    reduction** (`:2980-2989`): the arm exists to align a source, which has no graph inputs.
  - The delay contributes nothing to node latency, `TimingResult::total_delay` or
    `inserted_delays` (doc at `:711-719`).
- **A source-less node with a delay entry.** A submix's `Input` is not a source input, not a bank
  member and not bound, so it reaches the `Identity` fallback; an entry for it would sit unused in
  the map.
- **The fold.** A bus `Input` can be a fold master (`route_fold`, `runtime.rs:6351`). A master's
  inputs are rewritten to its own output (`:5257-5259`), so its reduction is a no-op over the folded
  sum and its node kind still runs after it. A delay arm that runs after the reduction therefore
  delays the folded sum too.
- **Exhaustive matches on `NodeKind`** include `execute_op`'s kind match (`:3031` onward) and the
  channel-symmetry witness (`:1490-1500`, where `TrackDelay` answers `designed(channels_agree)`).
- **Existing tests:** `crates/graph-compiler/tests/track_delay.rs`,
  `crates/host-core/tests/track_delay.rs`; red-mutation ledger `crates/graph/tests/MUTATIONS.md`.

## Decisions frozen for this slice

- **D1. Emission.** The graph compiler emits a `PreparedTrackDelay` for every **strip** with a
  nonzero lane delay: tracks as today, then submixes, keyed by the strip's `TrackStage::Input` node.
  The estimate charge covers both through the same vector. The source-semantics comment at
  `track_delays` is rewritten to say a submix's entry lowers to D2's arm.
- **D2. The arm.** `NodeKind::SumDelay { line: u32, channels_agree: bool }` (the D5 witness needs
  `channels_agree`, as `TrackDelay` carries it). In `node_kind`, after the route branch and
  before the `Identity` fallback, a node that still has a `track_delays` entry lowers to `SumDelay`
  with a new `TrackDelayLine` (the same type and the same `process`). Source inputs keep the
  `TrackDelay` branch exactly as today.
- **D3. Execution.** `SumDelay` takes the ordinary path through `execute_op`: staging and the D9
  reduction into the node's buffer first, then, in the kind match, `track_delays[line].process` on
  that buffer in place. It never returns early.
- **D4. PDC is untouched.** A `SumDelay` contributes nothing to latency, total delay or inserted
  delays, exactly as `TrackDelay`.
- **D5. Every exhaustive match** gains the arm. In the channel-symmetry witness it answers like
  `TrackDelay`: lanes agree iff their declared delays are equal. A bus is never collapse-eligible
  anyway (DESIGN 2.2b).

## Deliverables

1. Graph compiler: D1.
2. Graph runtime: D2, D3, D5.
3. `crates/graph/tests/MUTATIONS.md`: a row for gate 2's mutation.
4. `docs/SESSION_SCHEMA_V1.md`: a submix's `delay_samples` delays its summed input and is not
   compensated.

## Authorized paths

- `crates/graph-compiler/src/compile.rs`, `crates/graph-compiler/tests/` (one new test or
  extension of `track_delay.rs`)
- `crates/graph/src/{runtime.rs,lib.rs}` and `crates/graph/tests/`, `crates/graph/tests/MUTATIONS.md`
- `crates/host-core/tests/submix_strip.rs`
- `docs/SESSION_SCHEMA_V1.md`
- this spec

## Non-goals

- No change to a track's `TrackDelay` arm, its witness or its tests.
- No PDC change, and no compensation of any delay.
- No console slots on submixes (next slice).

## Hazards

- **Running before the sum.** Placing the arm in `execute_op`'s early-return path (as `TrackDelay`
  is) would delay nothing useful: the reduction would then overwrite or never read the delayed
  words. Gate 2's mutation pins the order.
- **The fold master.** A folded bus with a delay must still delay its sum. Gate 3 pins it.
- **Unused entries.** Every emitted entry must be consumed at bind; a submix entry that falls to
  `Identity` is a silent no-op. Gate 1 catches it.

## Objective gates

1. **The bus delay runs after the sum** (`crates/host-core/tests/submix_strip.rs`, 48 kHz).
   - A bus with `delay_samples` 37 on the left and 0 on the right has two contributors; an impulse
     is fed to both.
   - The bus output's left plane peaks 37 samples after its right plane; the right plane is
     bit-identical to the same session with both delays 0.
   - The plan's output latency and inserted delays equal the undelayed session's.

   *Test value: it turns red if a submix's delay is not lowered, runs on one contributor instead of
   the sum, or is compensated by PDC. No existing test delays a sum.*
2. **The delay follows the reduction** (graph-level test).
   - A plan whose bus `Input` has three route inputs and a delay of 5 samples on both lanes: the
     output equals a scalar oracle that sums the inputs in D9 order, then delays the sum.
   - The PR records one mutation run: running the line before the reduction (in the early-return
     path) turns this red; the row goes in `MUTATIONS.md`.

   *Test value: it turns red if the arm processes the node's buffer before the reduction writes
   it.*
3. **A folded bus still delays its sum.** Eight banked tracks routed `post_pan` into one bus with a
   7-sample delay on both lanes:
   - `bank_route_folds()` equals the same session's count without the delay;
   - the output is bit-identical to the same plan with `graph::test_only_set_route_fold_declined`.

   *Test value: it turns red if the fold rewrite of a master skips its kind, so a folded bus loses
   its delay.*
4. **Render allocates nothing.** After warm-up, `allocations == 0` and `frees == 0` around every
   render call for gate 1's session, measured with `bench_support::alloc`'s thread-scoped counters.
   *Test value: it turns red if the bus delay line is sized or grown on the render thread instead of
   at bind.*
5. **Unchanged where no submix delay exists.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/check-graph-determinism.sh`
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-console-fixtures.sh target/release/session_validator`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `bash scripts/trace-graph-audit.sh target/release/audit` (the slice adds a render arm)
   - every existing `track_delay.rs` test passes unchanged.
6. **Workspace, 4-lane and policy.**
   - the test-debug-a workspace command (DESIGN section 7);
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job at the
     batch push;
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   - `for x in graph realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`

## Evidence

- The gate-2 mutation run's output.
- Confirmation that no digest moved for sessions without a submix delay.

## Dependencies

- *Render a submix strip on its summed input* (#1200)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" and "unchanged" gates are hard stops. NaNs are folded (decision 10).
- Render stays allocation-, lock- and syscall-free; the delay line's `process` stays inside its
  `REALTIME_POLICY` region.
- One implementation shape for every target.
- A test that greps source or prose is refused.
- Commit on the K1 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
