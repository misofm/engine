# Keep every node's latency from dropping during playback

Stream A of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-8).
Slice 15 of *Swap a rebuilt plan without an audio gap* (#1269). Code anchors verified on `main` at
`6fb211594`.

## Product outcome

Removing a latent effect (a limiter), or the track that holds it, while audio plays no longer moves
the rest of the mix in time, at the output or inside a submix. The successor keeps every surviving
node's input timing, so every other path keeps its compensation and continues bit for bit. The
lower latency comes back at the next discontinuity the host declares, a stop or a seek of every
source (D15-8, *Reset latency floors at a host-declared discontinuity*, #1323). There is no
permanent latency reserve.

## Context

- PDC computes timings in `timings` (`crates/graph-compiler/src/pdc.rs:38-160`, called from
  `crates/graph-compiler/src/compile.rs:450`). For each node in schedule order:
  - `max` is the latest incoming arrival (`:61-65`);
  - every incoming edge is delayed by `max - source` (`:66-88`);
  - the node's arrival is `max + latency`, and the output latency is the `Output` node's arrival
    (`:145-149`).
- When a latent effect is removed, `max` drops at every node downstream of it, so every
  compensation line into those nodes shortens. With the head-aligned copy of *Carry compensation
  lines across a plan swap* (#1283, D4), every unchanged path into them would then skip that many
  samples.
- The output latency is reported as `HostPrepareReport::latency_samples`
  (`crates/host-core/src/prepare.rs:224`, filled at `:1841`) and as the C ABI's plan resource
  report (`crates/control-plane/src/compile.rs:664`).
- VST3 notes that a plug-in latency change may interrupt playback while the host recomputes delay
  compensation (`IAudioProcessor::getLatencySamples`). Floors remove the decrease case.
- The full carry comes from *Carry strip delay lines and live send ramps across a plan swap*
  (#1284).

## Decisions frozen for this slice

- **D1. Floors.** The graph compile request (`GraphBuiltinsCompileRequest`,
  `crates/graph-compiler/src/lib.rs:42`) takes an optional map from `GraphNodeId` to an input
  floor, empty by default. In `timings`, a node's `max` is `max(latest incoming arrival, floor)`,
  and every incoming edge is compensated to it. With no floors, neither the program nor its bytes
  change.
- **D2. Who sets them.** The inventory records each node's floored `max` at preparation. A
  successor is compiled with a floor for every node present in both plans, equal to the
  predecessor's recorded `max` for that node. Two cases compile with no floors:
  - a first compile, a fresh C ABI `compile_session` and a browser boot;
  - a successor prepared at a declared discontinuity (#1323).

  Floors therefore last until the next declared discontinuity, never for the session's life.
  *Grow latency during playback by adopting a primed warm successor* (#1287) adds its own `P` on
  top of these floors, on carried nodes only, through the same map.
- **D3. A path whose own latency dropped.** A floored single-input node whose upstream latency
  dropped gains a new compensation line, which starts at rest. That path's strip lost a latent
  effect, so it is already in the restart set (#1279 D1) and gets the duck-swap of *Duck-swap a
  strip whose state cannot continue across a plan swap* (#1324).
- **D4. Report.** `latency_samples` reports the floored output arrival, which is the real one.
- **D5. Growth is not handled here.** A node whose natural `max` exceeds its floor grows. #1287's
  warm successor makes that exact, and its counted fallbacks are the documented transition (D15-8).
- **D6. Documentation.** In the header comment and `docs/C_ABI_V1_QUALIFICATION.md`: no node's
  latency, the output's included, drops across a structural transaction, and it returns to its
  natural value at the next declared discontinuity (#1323).

## Deliverables

1. D1 in `crates/graph-compiler` (the request field and `timings`), plumbed through the builtins
   graph compile request.
2. D2-D4 in `crates/host-core`: the inventory field, successor preparation and the report. Also a
   `test-support` constructor of an inventory that carries only floors, so a test can compile a
   fresh reference with the same floors.
3. D6.

## Authorized paths

- `crates/graph-compiler/src/pdc.rs`, `crates/graph-compiler/src/compile.rs`,
  `crates/graph-compiler/src/lib.rs`, `crates/graph-compiler/tests/`
- `crates/builtins-compiler/src/lib.rs` (request plumbing only)
- `crates/host-core/src/prepare.rs`, `crates/host-core/tests/successor_swap.rs`
- `crates/capi/src/runtime/tests.rs`, `crates/capi/include/miso_engine_v1.h` (comments only),
  `docs/C_ABI_V1_QUALIFICATION.md`. The capi files are stream B's, and root orders the merge.

## Non-goals

- No lead floors and no warm successor (#1287), and no latency reserve (rejected by D15-8).
- No discontinuity declaration (#1323).

## Objective gates

1. **Gap-free acceptance, output.** Session A has three tracks, the first with a true-peak limiter
   insert. Session B removes the first track. The swapped run (swap after block 6) is bit-identical
   in every block to a fresh B compiled with A's floors and fed the same PCM from frame 0, at all
   four launch rates. Without floors, the remaining tracks jump by the limiter's latency at block 7.
2. **Gap-free acceptance, inside a submix.** Two tracks feed a submix, one through a limiter
   insert, and B removes the limiter's track. The other track and everything downstream are
   bit-identical to the floored reference.
3. **Through the C ABI.** Gate 1's edit as a `RemoveTrack` transaction:
   `miso_engine_v1_plan_resources` reports an unchanged `latency_samples` after the swap, and the
   output equals gate 1's reference.
4. **No change without floors.** These keep their digests: `cargo test --locked -p graph-compiler`
   (the graph fixture corpus), `bash scripts/check-graph-determinism.sh`, and
   `cargo test --locked -p console-workload`.
5. **Floors chain.** A second removal after gate 1's swap keeps the first floors: the inventory
   records floored values, not natural ones. Output latency is still A's.
6. Commands:
   - `cargo test --locked -p graph-compiler -p builtins-compiler -p host-core -p capi --features builtins-compiler/test-support,host-core/test-support`
   - `bash scripts/check-capi-abi.sh`, `bash scripts/check-workspace-policy.sh`,
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`

## Test value

- Gate 2: floors applied only at the output leave a submix's paths skipping. It turns red.
- Gate 3: a report that keeps the natural latency while the plan renders the floored one turns it
  red.
- Gate 4: floors that change the program when absent turn it red.
- Gate 5: an inventory that records the natural `max` drops latency at the second edit. It turns
  red.
- Superseded: the former gate 5 (growth pinned as a gap of `Δ` zeros) is not written. #1287's
  warm-successor gates own growth.

## Dependencies

- *Carry strip delay lines and live send ramps across a plan swap* (#1284).

## Amendment (controls coordinator, batch 15 check, 2026-10-15)

Checked against `228fca26c`.

**Moved anchors.**
- `timings` is `crates/graph-compiler/src/pdc.rs:37-160`. `max` is `:61-65`, the edge loop
  `:66-99`, the arrival `max + latency` `:100-104`, the output latency `:143-150`. It is called
  from `crates/graph-compiler/src/compile.rs:422`, inside `compile_with_builtins` (`:78`).
- `GraphBuiltinsCompileRequest` is `crates/graph-compiler/src/lib.rs:41`. Its fields are
  `plan_id`, `effects`, `builtins`, `caps` and `dispatch`. No floor field exists yet.
- `HostPrepareReport::latency_samples` is `crates/host-core/src/prepare.rs:273`, filled at
  `:3134`. The C ABI copies it at `crates/control-plane/src/compile.rs:1003` into
  `PlanResources::latency_samples` (`crates/control-plane/src/adapter.rs:104`). The ABI field is
  `crates/capi/src/abi.rs:283`.
- The inventory is `PlanStateInventory` (`prepare.rs:711`). It already holds the compensation
  lines (`delay_lines`, #1283 D1), the strip delays and live sends (#1284) and the restart set
  `restarted` (#1277 D6, `prepare.rs:744`; read by `restarted_strips`, `:688`). D3's "restart set
  (#1279 D1)" is this field now.
- Successors are prepared by the control plane: `prepare_runtime`
  (`crates/control-plane/src/compile.rs:901`) is called with a `SuccessorBase` at
  `crates/control-plane/src/control.rs:1662-1683`. `SuccessorBase` is `prepare.rs:1070`. The floors
  travel in the inventory, so D2 needs no control-plane change.
- The header's latency text is `crates/capi/include/miso_engine_v1.h:54` and `:81` (D6's place).

**Authorized paths.**
- `crates/builtins-compiler` does not build the graph request. The one product site is
  `crates/host-core/src/prepare.rs:2466`. Deliverable 1's "plumbed through the builtins graph
  compile request" means that site. `crates/builtins-compiler/src/lib.rs` leaves the list.
- The request is a struct literal in 21 files. A new field breaks each one. Outside the list are
  `crates/graph-compiler/src/bin/graph_fixture.rs`, `crates/host-core/tests/live_routes.rs`,
  `route_mute.rs`, `submix_strip.rs`, `tools/audit/src/builtins_graph.rs`, `fixture_builtins.rs`,
  `graph.rs` and `tools/console-workload/src/lib.rs`. Proposed: add them for the neutral field only
  (an empty floor map). Root confirms at filing.
- **B15-R9, root decision under owner delegation (2026-10-15): the eight files above are allowed, for a neutral-field-only
  addition to the graph request struct literal.** No other edit to them is authorized.

**Decisions.**
- D1-D6 hold against the code and against decision 17. Decision 17 does not touch latency (C11
  keeps the render rules), and a floor is not a control.
- Both hosts: since #1381a the browser runs `control_plane::SessionState`
  (`hosts/host-web/src/lib.rs:6-16`), so a browser rebuild gets floors through the same code. A
  browser boot is still a no-floor case.
- The browser reports no plan latency. Decision 17 C13 lists "plan latency and tail" as a one-host
  difference for root to file; no issue is filed yet. So D4 and gate 3 cover the C ABI only.
- Empty boot (#1682): an empty plan shares only the output node with its successor, at arrival 0.
  Its floor changes nothing.
- Stored automation: #1637's node time is `τ(r) = timeline(r + ΣP - a(n))`. With floors, `a(n)`
  must be the floored arrival, the one the plan renders. #1647 (open) reads `a(n)`. Whichever of
  #1285 and #1647 lands second makes it read the floored value.

**Dependencies.**
- #1284 is closed. #1283, #1279 and #1277 are closed.
- Batch order (root, 2026-10-15): batch 15a is the controls lane, #1344, #1398 and #1310.
  #1324 left batch 15a (root ruling, 2026-10-15); it lands in a later batch after #1288, #1391,
  #1363 and #1325. The rest of batch 15 follows. #1324 is not ordered before this slice. This
  slice keeps its no-duck fallback until #1324 lands: a restarted strip starts at rest with no
  duck.
- #1324 is open and not in batch 15/16. D3's duck-swap needs it. Until it lands, a restarted
  strip starts at rest with no duck.
- #1287 is open. D2 names it as later work only. #1323 is in the batch.

**Gates and commands.**
- Every named file and script exists: `crates/host-core/tests/successor_swap.rs`,
  `crates/capi/src/runtime/tests.rs`, `docs/C_ABI_V1_QUALIFICATION.md`,
  `scripts/check-graph-determinism.sh`, `scripts/check-capi-abi.sh`,
  `scripts/check-workspace-policy.sh`, `tools/console-workload`.
- Gate 6's command is valid. Its features exist.

## Amendment (controls coordinator, batch 15a fold, 2026-10-16)

- **Browser latency.** "No issue is filed yet" (batch-15 amendment, Decisions) is void. It is
  #1688 (*Report the plan latency on the browser*), which lands after this slice and adds the
  browser's parity row on the floored latency. D4 and gate 3 still cover the C ABI only here.
- **Anchor.** The `restarted` field of `PlanStateInventory` is `prepare.rs:750` at `228fca26c`
  (the batch-15 amendment says `:744`). On `main` at `c5b5c88b2` it is `:757`.
