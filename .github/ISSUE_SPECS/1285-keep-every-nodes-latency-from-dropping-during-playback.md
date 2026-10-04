# Keep every node's latency from dropping during playback

Slice 15 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on
`54b0a1bf8` (unchanged at `24029badb`).

## Product outcome

Removing a latent effect (a limiter), or the track that holds it, while audio plays no longer moves
the rest of the mix in time, at the output or inside a submix. The successor keeps every surviving
node's input timing, so every other path keeps its compensation and continues bit for bit. The lower
latency comes back when the host compiles the session again (for example while stopped).

## Context

- PDC (`crates/graph-compiler/src/pdc.rs:38-160`, `timings`): for each node in schedule order, `max`
  is the latest incoming arrival (`:60-66`); every incoming edge is delayed by `max - source`
  (`:67-89`); the node's arrival is `max + latency`. The output latency is the `Output` node's
  arrival. When a latent effect is removed, `max` drops at every node downstream of it, so every
  compensation line into those nodes shortens, and with the head-aligned copy of *Carry compensation
  lines across a plan swap* (slice 13) every unchanged path into them skips that many samples.
- The output latency is reported as `HostPrepareReport::latency_samples`
  (`crates/host-core/src/prepare.rs`, from `artifact.report().output_latency`) and as
  `PlanResourceReport::latency_samples` on the C ABI (`crates/capi/src/runtime/compile.rs:444`).
- VST3 notes that a plug-in latency change may interrupt playback while the host recomputes delay
  compensation (`IAudioProcessor::getLatencySamples`). Floors remove the decrease case.
- The full carry comes from *Carry strip delay lines and live send ramps across a plan swap*
  (#1284).

## Decisions frozen for this slice

- **D1. Floors.** The graph compile request takes an optional map from `GraphNodeId` to an input
  floor (default empty). In `timings`, a node's `max` is `max(latest incoming arrival, floor)`, and
  every incoming edge is compensated to it. With no floors the program and its bytes do not change.
- **D2. Who sets them.** A successor is compiled with, for every node present in both plans, the
  floor equal to the predecessor's `max` for that node. The inventory records each node's `max` at
  preparation. A first compile, a fresh C ABI `compile_session` and a browser boot use no floors.
  A floored single-input node whose upstream latency dropped gains a new compensation line; it
  starts at rest (that path was edited).
- **D3. Report.** `latency_samples` reports the floored output arrival, which is the real one.
- **D4. Growth is not handled here.** A node whose natural `max` exceeds its floor grows; slice 13's
  head-aligned copy then gives the paths into it a gap of the growth. *Pre-roll a successor whose
  latency grows* (slice 17) addresses that, subject to owner question Q2.
- **D5. Documentation.** The header comment and `docs/C_ABI_V1_QUALIFICATION.md`: no node's latency,
  the output's included, drops across a structural transaction; compile the session again to lower
  it.

## Deliverables

1. D1 in `crates/graph-compiler` (request field, `timings`), plumbed through the builtins graph
   compile request.
2. D2-D3 in `crates/host-core` (inventory field, successor preparation, report); a `test-support`
   constructor of an inventory that carries only floors, so a test can compile a fresh reference with
   the same floors.
3. D5.

## Authorized paths

- `crates/graph-compiler/src/pdc.rs`, `crates/graph-compiler/src/lib.rs`, `crates/graph-compiler/tests/`
- `crates/builtins-compiler/src/lib.rs` (request plumbing only)
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`
- `crates/capi/src/runtime/tests.rs`
- `crates/capi/include/miso_engine_v1.h` (comments), `docs/C_ABI_V1_QUALIFICATION.md`

## Non-goals

- No pre-roll and no latency reserve (slice 17, owner Q2).

## Objective gates

1. **Gap-free acceptance, output.** Session A: three tracks, the first with a true-peak limiter
   insert. Session B removes the first track. The swapped run (swap after block 6) is bit-identical,
   for every block, to a fresh B compiled with A's floors and fed the same PCM from frame 0, at all
   four launch rates. Without floors, the remaining tracks jump by the limiter's latency at block 7.
2. **Gap-free acceptance, inside a submix.** Two tracks feed a submix, one through a limiter insert;
   B removes the limiter's track. The other track and everything downstream are bit-identical to the
   floored reference.
3. **Through the C ABI.** Gate 1's edit as a `RemoveTrack` transaction: `miso_engine_v1_plan_resources`
   reports unchanged `latency_samples` after the swap, and the output equals gate 1's reference.
4. **No change without floors.** `cargo test --locked -p graph-compiler` (graph fixture corpus),
   `bash scripts/check-graph-determinism.sh` and `cargo test --locked -p console-workload` keep their
   digests.
5. **Growth is the documented transition.** A swap that raises one node's `max` by `Δ` gives each
   unchanged path into it its pending samples, then exactly `Δ` samples of `+0.0`, then the reference
   delayed by `Δ`. (Slice 17 replaces this test if Q2 chooses pre-roll.)
6. Commands:
   - `cargo test --locked -p graph-compiler -p builtins-compiler -p host-core -p capi --features builtins-compiler/test-support,host-core/test-support`
   - the umbrella's inherited gates.

## Test value

- Gate 2: floors applied only at the output leave a submix's paths skipping; it turns red.
- Gate 3: a report that keeps the natural latency while the plan renders the floored one turns it red.
- Gate 4: floors that change the program when absent turn it red.

## Dependencies

- *Carry strip delay lines and live send ramps across a plan swap* (#1284).
