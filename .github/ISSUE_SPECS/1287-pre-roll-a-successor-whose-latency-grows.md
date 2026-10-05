# Pre-roll a successor whose latency grows

Stream C of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8, D15-12, D15-17).
Formerly slice 17 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`main` at `6fb211594`.

This issue is the umbrella of the **warm successor** and owns its proof obligation. Its own
deliverable is the **first slice** below; the later slices are listed at the end and are filed
separately. Owner question Q2 of #1269 (pre-roll, a reserve or a gap) is answered by decision 15
D15-8. The former one-callback pre-roll survives only as the bounded fallback, and there is no
reserve.

## Product outcome (whole issue)

Adding a latent effect that raises a node's latency while audio plays (the output's or a
submix's) neither gaps nor jumps the mix. Every unchanged path continues bit for bit. The mix
becomes later only in the source-read clock, which hosts anchor seeks on (D15-12). Render pays one
bounded state copy and a pointer swap, never `k` blocks of DSP in one callback. There is no
permanent latency reserve. Any fallback is counted and reported through the watermark's outcome
flags.

## Product outcome (this slice)

A floor on a node that reads a source is realised as a source-claim compensation line. A plan
compiled with every surviving node floored at its predecessor's arrival plus `P` renders the
predecessor's mix, delayed by exactly `P`. The proof below is recorded here and checked by gates
1-3. Both the catch-up and its fallbacks rely on this relation.

## Context

- **History fill is impossible.** The samples a grown edge needs are future samples, not past
  ones (round 1 C2, round 2 Q-C2). The remedies are to compute them, never move the timing, or
  accept a transition. Decision 15 picks the first, done off the render thread.
- **The swap today.** `RealtimePlanOwner::enter_block`
  (`crates/engine/src/realtime/plan_exchange.rs:375-441`) does three things in one block: it takes
  a candidate, adopts the clock (`:418`), and runs the carry. `GraphExecutor::adopt_predecessor`
  (`crates/graph/src/lib.rs:3139`) moves the sources (`adopt_sources`,
  `crates/source/src/lib.rs:1851`) and copies the input lanes. Producers move off-render through
  `SourceControlSet::adopt_persisting` (`crates/host-core/src/source.rs:286`).
- **Timing.** `pdc::timings` (`crates/graph-compiler/src/pdc.rs:37-120`) gives each node the
  maximum incoming arrival and compensates every incoming edge to it. A node with no incoming edge
  gets arrival 0 (`:61-66`). A track's input stage reads its source through a claim
  (`GraphSourceInputClaim`), not through a graph edge. So a floor on it, under *Keep every node's
  latency from dropping during playback* (#1285) D1, would raise its arrival with no line to delay
  its content.
- **Clocks.** The graph driver reads every source at the block's absolute sample
  (`consumer.begin_block_at(first_sample)`, `crates/source/src/lib.rs:1721`).
  `PreparedRenderPlan` keeps one clock: `next_absolute_sample` (`crates/engine/src/realtime/plan.rs:609`),
  passed to the executor as `RenderTime` (`:943`).
- **Ring headroom.** `default_source_ring_frames` (`crates/host-core/src/prepare.rs:65`) is 100 ms
  (`SOURCE_STALL_TOLERANCE_MS`, `:57`) plus two quanta.

## The warm successor (decision 15; binding on every slice)

- **W1. Successor.** Prepared with every surviving node floored at its predecessor arrival plus
  `P`. Here `P = k * quantum`, and `k = ceil(Δ / quantum)` for the largest growth `Δ` over the
  surviving nodes.
- **W2. Snapshot at B.** At block B, render copies the predecessor's state into the successor.
  The copy uses copy-mode carry: a bounded memcpy and allocation-free payload calls. It arms each
  ring's peek, and returns the successor through the capacity-1 return queue.
- **W3. Catch-up.** The control thread renders the successor forward with the FP environment
  pinned, reading sources through the peeks. On the C ABI this runs in bounded
  `miso_engine_v1_service` slices; in the browser it runs in the Worker.
- **W4. Exact adoption.** Once the successor leads render by `P`, the control thread publishes
  "adopt exactly at S, else return". At S, render swaps the pointer and moves each consumer with
  its read index at `S + P`. The plan's source-read clock then exceeds its render clock by `ΣP`.
- **W5. Edits during the window.**
  - At publication the control plane writes the retarget records that *Carry fader, mute and pan
    ramps across a plan swap* (#1277) D5 holds, then the held live edits.
  - A live edit is held in the control plane, written to the successor's cells at publication, and
    applied at S. Its revision completes with S.
  - A structural edit supersedes the catch-up by compare-and-swap; the catch-up restarts from a
    new B.
- **W6. Bounds.** `ΣP <= P_max`, which is set by ring headroom. Floors and `ΣP` reset at a
  host-declared discontinuity.
  - #1323 lands before any read-ahead exists. So the catch-up slices reset `ΣP` themselves:
    #1323's discontinuity successor gets source-read offset 0 (#1355 D1).
  - Observers (meters, observation taps, spectrum) are carried through the catch-up (#1327 D4).
    A warm successor whose observers are not carried is never adopted.
- **W7. Fallbacks.** These apply on a missed deadline, a host that renders nothing, a peek
  divergence that recurs past the deadline, or a browser that is not isolated.
  - First, render-thread pre-roll bounded by `k_max`.
  - Then the transition: the D15-9 duck-swap (#1324) of the strips whose arrival grows.
  - Each fallback is counted and reported through the watermark's `preroll_fallback` or
    `transition_fallback` flag.
- **W8. Deadline.** It is counted in render samples from B, so a paused host never falls back. A
  host-declared stop turns a pending catch-up into a plain rebuild, applied at the next render.
- **W9. Constants.** `k_max`, the deadline and `P_max` are engine constants. Their values come from
  the records of *Record the swap block's cost on the 64-track console* (#1286) and *Prove two Wasm
  instances on one shared memory in three browser engines and on iOS* (#1331).

## Proof obligation (kept; this slice records it)

Derive the floors and alignment that make every unchanged edge bit-continuous when any surviving
node grows, the output or a submix. Show it on both cases. The derivation to verify and record
here:

1. Let `a(n)` be the predecessor's arrival at node `n`, and let the successor have
   `a'(n) = a(n) + P` at every surviving node.
2. On an edge `M -> N` between surviving nodes, the compensation is unchanged:
   `c' = a'(N) - (a'(M) + lat(M)) = c`. Its carried line continues as it is.
3. A source-reading node `N` gets a new source-claim line of `a'(N) = a(N) + P` samples. These are
   the only lines among surviving nodes that grow.
4. New or edited nodes take whatever compensation the floors give. They belong to the edited path,
   which D15-9 transitions.
5. The successor's output at render sample `r` equals the predecessor's if the successor reads
   source frame `r + P`. That is W4's source-read clock.
6. So the state copied at B is the successor's state at its source-read sample `B + P`, provided
   every source-claim line holds source frames `[B, B + P)` from the rings.

## Decisions frozen for this slice

- **D1. Source-claim line.** When the floor on a source-reading node exceeds its incoming arrival
  (0), the compiler inserts a source-claim compensation line of `floor` samples on each of that
  node's claims. It is reported beside `InsertedDelay` and runs on the existing PDC delay kernel
  (`pdc_delay_block`, `crates/lane/src/kernels.rs:990`). A claim read in place (#918) through such a line becomes a copy.
- **D2. No change without floors.** Without floors the program and its bytes do not change.
- **D3. Record.** Before implementing, write the proof above into this spec's decision record,
  with any correction the derivation finds.

## Deliverables

1. D3, then D1-D2 in the graph compiler and the graph runtime.
2. `crates/graph-compiler/tests/latency_growth.rs` (gates 1-2), and
   `crates/host-core/tests/latency_growth.rs` (gate 3).

## Authorized paths

- `crates/graph-compiler/src/pdc.rs`, `crates/graph-compiler/src/lib.rs`,
  `crates/graph-compiler/tests/latency_growth.rs` (new)
- `crates/graph/src/lib.rs` (the claim-line runtime only)
- `crates/host-core/tests/latency_growth.rs` (new), `.github/ISSUE_SPECS/1287-pre-roll-a-successor-whose-latency-grows.md`

## Non-goals

- No catch-up, peek, return queue or adoption in this slice (W2-W8 belong to the later slices).
- No latency reserve and no render-thread pre-roll outside W7.

## Hazards

- **Ownership.** `crates/graph` and `crates/graph-compiler` are stream A's files. This slice lands
  after #1285, in the merge order root sets.
- **Possible narrowing for unedited strips, settled by this slice's proof.**
  - Step 6 suggests that, for a strip with no edit, the only content missing at B is raw ring
    frames `[B, B + P)`. Those could be filled without processing.
  - An edited strip whose upstream nodes feed a new latency-`P` node needs `P` processed samples,
    which raw frames cannot supply. So D15-8's catch-up stands, and #1320, #1321 and #1354-#1361
    proceed.
  - The proof records whether unedited strips need only the fill. If they do, a later issue may
    narrow the catch-up's work for them.

## Objective gates

1. **Lemma, output growth.** Predecessor A is two tracks routed to the output, built the way
   `crates/graph-compiler/tests/compile_shapes.rs` builds its sessions. Successor B adds a muted track with a true-peak
   limiter insert. B is compiled with every surviving node floored at A plus `P`, for `k` derived
   at 48 kHz and a 128-frame quantum. Every edge between surviving nodes has A's compensation.
   Every source claim of a surviving track has a line of exactly its node's floor. No other line
   differs.
2. **Lemma, submix growth.** The same, with the limiter added inside a submix whose arrival grows
   while the output's does not.
3. **Delay relation.** A plan compiled from A with every node floored at A plus `P` renders A's
   output delayed by exactly `P` samples, bit for bit, from frame 0. Check all four launch rates at
   both bank widths. Before that, `P` samples of `+0.0`.
4. **No change without floors.** `cargo test --locked -p graph-compiler` and the graph fixture
   corpus pass unchanged.
5. Commands:
   - `cargo test --locked -p graph-compiler`
   - `cargo test --locked -p graph --features graph/test-support`
   - `cargo test --locked -p host-core --features host-core/test-support`
   - `bash scripts/check-realtime-policy.sh`, `bash scripts/check-workspace-policy.sh`,
     `bash scripts/check-cross-targets.sh`
   - `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`

## Test value

- Gate 1: floors that leave a source-reading node's arrival raised with no claim line, as #1285 D1
  alone would, show no line. Red.
- Gate 2: a `P` sized from the output's growth alone leaves a submix-internal edge changed. Red.
- Gate 3: a claim line padded at the wrong end, or a line on only one channel of a dual-mono
  track, breaks the exact delay. Red.

## Dependencies

- *Keep every node's latency from dropping during playback* (#1285): floors.

Each later slice below names its own dependencies in its own body.

## Later slices (filed)

Every slice must also meet the old gate 5: zero allocations and frees on render.

1. *Keep source transfer blocks in a shared pool, immutable from publication to release* (#1353).
   This is the storage the peeks read.
2. *Give the source ring a read-only peek cursor that gates release* (#1320).
3. *Render a successor plan off the render thread with a pinned floating-point environment*
   (#1321).
4. *Snapshot a running plan into a returned successor at a block* (#1354): W2, including the
   observers (#1327 D4).
5. *Catch up a returned successor and adopt it exactly at a scheduled sample* (#1355):
   - W3-W4 and W6, including the `ΣP` reset at #1323's declaration;
   - it takes the old gates 1, 2 and 4 (gate 4 now per D15-12).
6. *Hold live edits during a catch-up and apply them at the adoption sample* (#1356): W5, first
   bullet.
7. *Supersede a running catch-up by a structural edit* (#1357): W5, second bullet.
8. *Fall back from a missed catch-up deadline: bounded render-thread pre-roll, then the transition*
   (#1358): W7-W9, including the old gate 3.
9. *Turn a pending catch-up into a plain rebuild at a host-declared stop* (#1359): W8, last
   sentence.
10. *Run the C ABI catch-up from miso_engine_v1_service and report its outcome* (#1360).
11. *Run the browser catch-up in the Worker's service loop* (#1361).
