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

### Attempt 1 record (Terra)

- **Sites** (anchors moved after #1200; found by symbol). D1: `compile.rs` `track_delays` now
  iterates `model.strips()` (tracks, then submixes), keyed by `track_node(strip.id, Input)`; the
  estimate charge is unchanged code over the same vector; comment rewritten. D2: `NodeKind::SumDelay
  { line, channels_agree }`; `node_kind` lowers a remaining `track_delays` entry to it after the
  route branch, before `Identity`, with a new `TrackDelayLine` in the same line vector; the
  source-input `TrackDelay` branch is untouched. D3: `execute_op`'s kind match runs
  `track_delays[line].process` on the reduced output, after staging and the D9 reduction; no early
  return. D4: nothing added to latency, timing or inserted delays. D5: the witness answers
  `designed(channels_agree)` in its own arm (the `TrackDelay` arm is unchanged). `PreparedTrackDelay`
  docs say "strip". `SESSION_SCHEMA_V1.md`: the interim sentence is replaced, and the lane-key
  paragraph says a submix's delay delays its sum, uncompensated.
- **Gates** (x86-64 AVX2 host, native 8-lane):
  - 1: `host-core/tests/submix_strip.rs::a_bus_delay_shifts_its_summed_input_and_pdc_never_sees_it`
    green. Impulses at samples 3 (0.25) and 10 (0.5) on `t0`, `t1`; the left plane is bit-identical
    to the undelayed session's left shifted by 37, the right plane bit-identical to the undelayed
    right, the left peak is the right peak + 37, and `output_latency` and `inserted_delays` equal
    the undelayed artifact's.
  - 2: `graph/src/lib.rs::tests::a_bus_delay_runs_on_the_sum_after_the_reduction` green (3 bound
    inputs through identity routes `r1`, `r2`, `r0` into the bus `Input`, delay 5, quantum 4, 16
    blocks; scalar oracle `(a + b) + c` in route-ID order, then shift; the test also asserts the
    fixture's bits tell route-ID order from track order).
  - 3: `a_folded_bus_still_delays_its_sum` green: `bank_route_folds()` 8 with the 7-sample delay and
    8 without; output bit-identical to the fold-declined bind (0 folds) and to the undelayed output
    shifted by 7, both planes.
  - 4: `a_delayed_bus_renders_without_allocating` green (gate 1's session; render audit and
    thread-scoped counters exact zero after block 0). The #1200 gate-8 body became the shared
    `assert_renders_without_allocating`; the PDC test's artifact pipeline became `graph_artifact`.
  - 5: release build of audit/bench/capi/session-validator ok; `check-graph-determinism.sh` PASS
    100/100; `graph_fixture --check` clean; `check-console-fixtures.sh` ok;
    `check-builtins-fixtures.sh` ok (50 files); `trace-graph-audit.sh` PASS (1,000,000 blocks);
    `graph-compiler/tests/track_delay.rs` 8/8 unchanged. So no digest of a session without a submix
    delay moved.
  - 6: test-debug-a workspace command rc 0 (98 binaries, 1129 passed); `cargo fmt --check`; workspace
    clippy `-D warnings` clean; graph, realtime and workspace `check-*`/`test-*` policy pairs ok.
    Also `cargo test --release -p audit -p bench -p console-workload` ok (110 passed).
    `run-aarch64-tests.sh debug`: at batch push (no arm64 host).
- **Mutation runs** (each reverted after):
  - Gate 2 (ledger row 1201-2): the `SumDelay` line in the early-return path, before the
    reduction -> `a_bus_delay_runs_on_the_sum_after_the_reduction` red: `lane 0 sample 5: 0 !=
    5398.029`.
  - D1 tracks-only (`.filter(StripKind::Track)`) -> gate 1 red (`sample 3: 0.25 != 0.0`) and gate 3
    red.
  - Fold master's kind replaced by `Identity` at bind -> gate 3 red (`the folded bus against the
    reduction: sample 0`); gate 1 also red, because its two-track bus folds too.
  - A delayed bus treated as a source by both planning metadata impls (declines the fold) -> gate 3
    red only (`the delay costs the bus no fold`, 0 != 8); gate 1 stays green, its bits unchanged.
  - A `Vec` allocated in the `SumDelay` arm -> gate 4 red (the test process aborts under the render
    audit); #1200's gate 8 stays green (no bus delay).
- **Test value.**
  - Gate 1: red if a submix's delay is not lowered, delays one contributor rather than the sum,
    delays the wrong lane, or is charged to PDC; no existing test delays a sum.
  - Gate 2: red if the arm processes the bus buffer before the reduction writes it (or the
    reduction runs after the line); no other test reaches a delayed reduction at the graph level.
  - Gate 3: red if a delayed bus loses the route fold, or a fold master skips its kind; nothing
    else checks fold counts on a delayed bus.
  - Gate 4: red if the bus line is sized or grown on the render thread.
- **Deviations.**
  - The D5 witness arm has no new test: a bus is never collapse-eligible, so no rendered behavior
    can tell its answer; the non-goal keeps the `TrackDelay` witness test unchanged.
  - A bind-time `debug_assert!` that every delay entry is consumed was tried and dropped: harness
    tests in `graph-compiler/tests/bank_levels.rs` bind delayed track inputs to processors (not a
    source set), so their entries were never consumed before this slice either. Gate 1 covers the
    hazard for a submix.
  - A delay entry on a node that is neither a source input nor bound now delays that node's sum
    instead of being dropped. Only a harness reaches such a track input; the workspace and fixture
    gates are green.

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
