# Ramp live send coefficients on the render plane

Slice 22 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A send's level, on/off and 2x2 can change on the running plan with a declicked ramp and no plan
rebuild. After the ramp, the output is bit-identical to a plan freshly prepared with the new values.

This slice delivers the portable render core:

- the record and the render-plane lane of each live route;
- the drain, the indexed ramp (through *Ramp a send's coefficients with the indexed ramp kernel*, #1219) and
  the per-block activity write, including the PDC-delayed case;
- the graph-level queue plumbing, attached after compile;
- the fold exclusion that keeps a live route from being folded away.

host-core's domain-checked producer is the next slice, *Produce live send records from host-core* (#1221);
the browser and C ABI bindings follow it.

## Context (verified on `fe8ac679`)

Batches K1 and K2 do not move these anchors in substance; slices 18a-21 add what the next bullets say.

- **After *Gate every route's coefficients through one function* (#1215), *Mute a route in the session* (#1216),
  *Skip an inactive route in its destination's sum* (#1217) and *Let a route into a submix follow its source
  strip's mute in the session* (#1218):**
  - `PreparedRoute { node, transform, gate: RouteGate }`; `graph::gated_route_coefficients` binds a
    route's prepared coefficients; `RouteGate::silences()` is muted or follow-zeroed on both lanes;
  - `graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted)` is the single
    coefficient and domain authority;
  - the executor's `RouteActivity` (`active: Box<[bool]>` per route, beside the ops) is built at bind
    only when some route can be inactive, and `execute_op` reads it; a silenced delayed route stays
    active; the first input in D9 order owns the store.
- **After *Ramp a send's coefficients with the indexed ramp kernel*:** `lane::kernels::IndexedRamp`
  (`settled`, `new`, `coefficients_at`), `INDEXED_RAMP_LENGTH_MAXIMUM = 1 << 22`, and the
  `#[inline(never)]` kernel `route_mix_ramp_block<L>(left, right, &ramp, position)`, whose frame `f`
  uses `k = position + f + 1`.
- **The route op today.** `node_kind` (`crates/graph/src/runtime.rs:4025-4067`) checks, in order:
  source inputs, bank membership, `GraphNodeBinding`s, effects, then routes (`:4062-4063`). A
  processor binding would turn a route into `NodeKind::Bound(processor)` and skip the mix, so a live
  route must **not** be a `GraphNodeBinding` (VERIFY-1 MINOR-11). `NodeKind::Route([f32; 4])` is at
  `:826`; its `execute_op` arm runs `mix2x2_block::<FrameLane>` (`:3048-3051`), and `FrameLane` is
  `lane::Native` (`:316`).
- **The template for a live lane is the effect one:** `GraphEffectControlBinding { node, control }`
  (`crates/graph/src/lib.rs:833-838`) travels beside the prepared effects in `PreparedGraphPlan`
  (`:803`); the runtime keys bindings in a map at bind (`runtime.rs:3895`, filled from `:3926`,
  `:3992`) and `node_kind` takes one out for its node (`:4050`), producing
  `NodeKind::LiveControlEffect` (`:824`). The plan-level attach to copy is
  `PreparedGraphPlan::with_builtin_banks` (`crates/graph/src/lib.rs:1278`), which validates and
  attaches before bind without exposing the plan's parts.
- **What must not grow.** `PreparedGraphPlanParts` (`crates/graph/src/lib.rs:1671`) has struct
  literals at `crates/graph-compiler/src/compile.rs:812`, `crates/builtins-compiler/src/lib.rs:5112`,
  `:6142`, `crates/graph-compiler/tests/scale.rs:278`, `crates/source/src/lib.rs:2406` and several
  `crates/graph/src/lib.rs` tests; `GraphBuiltinsCompileRequest`
  (`crates/graph-compiler/src/lib.rs:42-58`) has 38 literal sites. This slice adds a field to
  neither. `size_of::<RuntimeOp>()` is a reported byte (`runtime.rs:1298-1313`), so no field is
  added to `RuntimeOp` either.
- **The sealed artifact.** `builtins_compiler::PreparedBuiltinsGraphArtifact` (`crates/builtins-compiler/src/lib.rs:1712`,
  `impl` `:2809`) is bound by `into_bound` (`:2964`) and `into_bound_with_source_set` (`:3014`).
  `GraphCompiler::compile_with_builtins` (`crates/graph-compiler/src/compile.rs:78-80`) returns it
  under the alias `graph_compiler::PreparedGraphBuiltinsArtifact` (`crates/graph-compiler/src/lib.rs:98`).
  Its seal doctests are `crates/graph-compiler/src/lib.rs:60-97`.
- **Queues.** `engine::realtime::spsc::bounded_spsc` (`crates/engine/src/realtime/spsc.rs:236`);
  producer `available_capacity` (`:328`) and `try_push` (`:349`); consumer `available_at_entry`
  (`:420`) and `try_pop` (`:440`). The bounded-drain pattern is the builtins input drain
  (`crates/builtins-compiler/src/lib.rs:457-475`). `bounded_spsc_retained_payload` (`spsc.rs:160`)
  states a queue's retained bytes. Render-path code sits between `// REALTIME_POLICY_BEGIN` and
  `// REALTIME_POLICY_END` markers, which `scripts/check-realtime-policy.sh` scans.
- **PDC staging belongs to the consumer** (`execute_op`, `runtime.rs:2970`, staging `:2996-3006`;
  `DelayRef` per consumer input, `crates/graph/src/program.rs:49-58`). A route has exactly one
  consumer, and "delayed" is that consumer input's `InputRef.delay`.
- **The fold.** Fold planning asks `PlanningMetadata` (`runtime.rs:6007-6014`; impls for
  `RuntimeParts` `:6016` and `BorrowedPlanningMetadata` `:6089`), whose doc requires its exclusions
  to stay coupled to `node_kind`'s arms. `plain_route_gains` (`:6110-6122`) checks source, bank
  membership, binding, effect and (since slice 18b) the gate. A bus `Input` can be a fold master
  (`route_fold`, `:6351`), so without an exclusion a live route could be folded away: never drained,
  its records never applied (VERIFY-2 M3). `bank_route_folds()` reports a plan's folds.
- **The live-owner charge precedent** is #1100: every byte a live owner allocates beyond the
  live-control-free path is charged (`crates/graph-compiler/src/estimate.rs:154-172`).
- **The browser kernel-shape gate** runs `--kernel-shape --kernel-pattern '4wide6f32x4' --kernel-min 11`
  (`scripts/check-web-audioworklet.sh:470-471`). Its rule 3 counts functions matching the pattern
  that carry `f32x4` arithmetic with more vector than scalar instructions; `K` is a ratchet raised
  when a wave adds kernels.

## Decisions frozen for this slice

- **D1. Which routes get a lane.** When route live controls are attached (D7), every route whose
  destination is a **submix input** gets one. Routes into the output stay prepared constants, keep
  the single-master fold, and keep structural edits.
- **D2. The record.**

  ```rust
  pub const ROUTE_RAMP_LENGTH_MAXIMUM: u32 = lane::kernels::INDEXED_RAMP_LENGTH_MAXIMUM;
  #[derive(Clone, Copy, Debug, PartialEq)]
  pub struct RouteControlRecord { target: [f32; 4], mute: bool, length: u32 }
  impl RouteControlRecord {
      /// `None` when `length > ROUTE_RAMP_LENGTH_MAXIMUM`, or when `mute` is true and any target
      /// coefficient is not `+0.0` (bitwise).
      pub fn new(target: [f32; 4], mute: bool, length: u32) -> Option<Self>;
      pub const fn target(&self) -> [f32; 4];
      pub const fn mute(&self) -> bool;
      pub const fn length(&self) -> u32;
  }
  ```

  `target` is `route_coefficients`' output, `mute` is true when the route's gate silences it, and
  `length` is the ramp in samples (0 is a step at the block boundary). The render side also
  `debug_assert!`s the bound (VERIFY-2 MINOR 4).
- **D3. Live route state and the law** (DESIGN P5, 5.7). Per live route: an `IndexedRamp`, a
  `position: u32` and a `mute: bool`.
  - **At bind:** `IndexedRamp::settled(prepared)`, where `prepared` is the route's bound
    `gated_route_coefficients`; `position = length = 0`; `mute = gate.silences()`. An attached, idle
    lane renders exactly the prepared bits (VERIFY-2 MINOR 4).
  - **On a record:** `ramp = IndexedRamp::new(ramp.coefficients_at(position), record.target, record.length)`,
    `mute = record.mute`, `position = 0`. Several records in one drain apply in order, each ramping from the previous record's
    `coefficients_at(0)` (for a step, `length == 0`, that is its target), so the last record wins.
  - **Per block:** the mix is `route_mix_ramp_block::<FrameLane>(left, right, &ramp, position)`, then
    `position = min(position + quantum, ramp.length)`. Position saturates and never wraps.
- **D4. Activity, decided once per block** (DESIGN P4).
  - At the op's start, after the drain, the route is **inactive for this block** iff
    `mute && position >= ramp.length && !delayed`.
  - The op writes that bit into `RouteActivity` before anything reads it; the destination's
    reduction runs later in the same block (units run once per block in topological order on one
    thread) and reads it there. The bit never changes inside a block.
  - The block in which `k` reaches `length` is mixed whole; frames after the snap use exact `target`.
  - A `length == 0` mute is inactive from the block whose drain applied it.
  - An unmute record makes the route active in the block that drains it, ramping from the current
    coefficients.
  - **A delayed route is never inactive** (VERIFY-1 BLOCKER-1): muted, it keeps mixing its zero
    target into its own buffer, and its consumer keeps staging it, so the fade reaches the bus whole,
    `d` samples later and aligned, and the line holds only zero-coefficient output when an unmute
    arrives. `delayed` is read at bind from the consumer's `InputRef.delay`.
  - A plan with any live route builds `RouteActivity` (the condition *Skip an inactive route in its
    destination's sum* left open for this slice).
- **D5. Drain.** At the route op's start, **every block and including while inactive**, pop at most
  `available_at_entry()` records and apply each in order (D3). Nothing is dropped: a record that
  arrives later is applied in a later block. The drain sits inside a `REALTIME_POLICY` region.
- **D6. Graph shape.**
  - `NodeKind::LiveRoute(Box<LiveRoute>)`. `LiveRoute` holds the D3 state, the consumer half, the
    route's `RouteActivity` index and the `delayed` flag.
  - `node_kind`'s route branch produces it when a route-control binding exists for the node, and
    `NodeKind::Route` otherwise.
  - `pub struct GraphRouteControlBinding { pub node: GraphNodeId, pub control: Box<RouteControlLane> }`,
    held beside `effect_controls` in `PreparedGraphPlan` (a private field that `PreparedGraphPlan::new`
    sets empty) and keyed at bind exactly as effect controls are.
  - A live route is never a `GraphNodeBinding`, and it never folds: `PlanningMetadata` gains
    `fn has_route_control(&self, node: &GraphNodeId) -> bool` in both impls, and `plain_route_gains`
    declines a node that has one (VERIFY-2 M3).
- **D7. Queue creation, after compile and before bind.** It touches neither
  `GraphBuiltinsCompileRequest` nor `PreparedGraphPlanParts`:

  ```rust
  // crates/graph
  impl PreparedGraphPlan {
      pub fn attach_route_controls(&mut self, depth: NonZeroUsize)
          -> Result<Vec<GraphRouteControlProducer>, GraphRouteControlError>;
  }
  pub struct GraphRouteControlProducer { pub route_id: Box<str>, /* producer half */ }
  impl GraphRouteControlProducer {
      pub fn free(&self) -> usize;                                   // available_capacity
      pub fn try_push(&mut self, record: RouteControlRecord) -> Result<(), RouteQueueFull>;
  }
  pub fn route_control_resources(producers: &[GraphRouteControlProducer]) -> RouteControlResources;

  // crates/builtins-compiler
  impl<R> PreparedBuiltinsGraphArtifact<R> {
      pub fn attach_route_live_controls(&mut self, depth: NonZeroUsize)
          -> Result<Vec<GraphRouteControlProducer>, GraphRouteControlError>;
  }
  ```

  - It creates one `bounded_spsc::<RouteControlRecord>(depth, QueueGeneration(0))` per D1 route
    (`crates/engine/src/realtime/spsc.rs:236-238`; `QueueGeneration(0)` as the builtin track queues
    use, `crates/builtins-compiler/src/lib.rs:3432`), in canonical route-ID
    order, modelled on `with_builtin_banks`: the plan validates and records the bindings; callers
    never see its parts.
  - The artifact method delegates to the plan, keeping the seal doctests green. Calling it twice is
    refused (`GraphRouteControlError::AlreadyAttached`).
  - With no call, the plan is byte for byte today's.
- **D8. Resources** (VERIFY-2 MINOR 5). `route_control_resources` states, exactly, the bytes the
  attach and the bind add: each queue's `bounded_spsc_retained_payload::<RouteControlRecord>`, each
  `Box<LiveRoute>`, the producer table, the route IDs, and the `RouteActivity` table with its per-op
  route indices whenever the compile-time estimate did not already charge it (a plan whose live
  routes have no silencing gate), plus the largest single allocation. The
  next slice charges them in host-core.
- **D9. Kernel shape in the browser** (VERIFY-2 MINOR 6). The `LiveRoute` arm calls the
  `#[inline(never)]` kernel, so its `f32x4` instantiation is a named function the kernel-shape gate
  counts. Raise the `--kernel-min` literal at `scripts/check-web-audioworklet.sh:471` to the
  qualifying-kernel count the gate reports on this slice's base plus one (13 if the base still
  carries twelve, `:460-461`), and update the ratchet comment at `:454-469`; if the
  gate's count shows the instantiation does not qualify, fix the kernel's shape rather than the gate.

## Deliverables

1. In `crates/graph`: `ROUTE_RAMP_LENGTH_MAXIMUM`, `RouteControlRecord`, `RouteControlLane`,
   `GraphRouteControlBinding`, `GraphRouteControlProducer`, `RouteQueueFull`,
   `GraphRouteControlError`, `RouteControlResources`, `route_control_resources`,
   `PreparedGraphPlan::attach_route_controls` (D2, D6, D7, D8), exported from `lib.rs`.
2. The runtime: `NodeKind::LiveRoute` with the drain (D5), the ramp and mix (D3), the activity write
   (D4); binding of route controls at bind; `has_route_control` in both `PlanningMetadata` impls;
   a test-only drained-record counter beside slice 19's mix counter.
3. `PreparedBuiltinsGraphArtifact::attach_route_live_controls` (D7).
4. D9.
5. A "Live routes" section in `docs/BUILTINS_AND_METERING_V1.md`: D1-D5, the law under the name
   **indexed ramp**, why it is not D11 (coefficients are a pure function of the frame index, so the
   ramp vectorises over frames and is width-independent), and the delayed-route rule.
6. `crates/graph/tests/MUTATIONS.md` rows for each red mutation named in the gates.

## Authorized paths

- `crates/graph/src/{lib.rs,runtime.rs,program.rs}`, `crates/graph/tests/`, `crates/graph/tests/MUTATIONS.md`
- `crates/builtins-compiler/src/lib.rs` (only the `impl<R> PreparedBuiltinsGraphArtifact<R>` block, D7)
- `crates/graph-compiler/src/{compile.rs,lib.rs,ids.rs}`, only if a binding helper is needed
- `crates/graph-compiler/tests/live_routes.rs` (new)
- `crates/host-core/tests/route_mute.rs` (amendment A1, the MINOR-1 case only)
- `scripts/check-web-audioworklet.sh` (the `--kernel-min` value at `:471` and its ratchet comment at `:454-469` only)
- `docs/BUILTINS_AND_METERING_V1.md`
- this spec

## Non-goals

- No host-core producer, domain check or resource admission (*Produce live send records from
  host-core*).
- No browser kind and no C ABI path.
- No live output routes (DESIGN O9).
- No fused reduction, no epilogue fold of live routes, and no deferred deactivation of delayed sends
  (DESIGN O1, O4, O6).
- No `controlSmoothing` dependency: the caller supplies `length` until #1054 lands.
- No change to D11 or to any fader, matrix or effect ramp.

## Hazards

- **The block in which a mute ramp ends.** It must be mixed whole. Deciding activity after the mix,
  or mid-block, loses the start of the fade's last block (VERIFY-1 MAJOR-1).
- **A delayed route going inactive** cuts the fade still in its line and later releases stale arena
  samples (VERIFY-1 BLOCKER-1). D4 forbids it; gate 3 is its regression test.
- **Drain while inactive.** An inactive op that returns before draining never sees its unmute.
- **In-place routes.** A live route that is the sole reader of its tap mixes in place; when inactive
  its buffer holds the raw tap, which the destination never loads (slice 19's rule).
- **The fold.** A live route on a bus that can fold would be replaced by an epilogue that never
  drains. D6's exclusion and gate 7 pin it.
- **Kernel shape.** If the `f32x4` instantiation is inlined into `execute_op`, the gate cannot see it.

## Objective gates

Render gates live in `crates/graph-compiler/tests/live_routes.rs`, which compiles a session, attaches
route controls with `attach_route_live_controls`, binds input processors that write a distinct
non-constant signal per track and lane (the `bank_levels.rs` compile-bind-render pattern), and
renders. The "fresh plan" for a comparison is the edited session compiled and fed the same sources
from sample 0, with a stateless downstream: no console slots, no stateful inserts and an identity
input section on the destination bus (VERIFY-2 MINOR 7).

1. **Idle attached lanes change nothing.** A session with muted, unmuted, delayed and undelayed sends
   renders bit-identically with route controls attached and idle as without them, for 16 blocks.
   *Test value: it turns red if the bind-time state is not the prepared coefficients and gate (for
   example `start = 0`), or if attaching changes activity.*
2. **The block in which a mute ramp ends is mixed whole** (VERIFY-1 MAJOR-1). A send into a bus at a
   128-frame quantum, 48 kHz, receives a mute with `length = 480` before block 0: `k` reaches 480 at
   frame index 95 of block 3. Block 3 at the bus input equals the oracle (mixed whole, exact zeros
   after the snap); from block 4 the route-mix counter stops and the bus equals the P4 oracle
   without that input.
   *Test value: it turns red if activity is written after the mix or decided mid-block, which drops
   frames 0-94 of the fade's last block.*
3. **A delayed send round-trips through mute without a cut or stale audio** (VERIFY-1 BLOCKER-1).
   - Track `a` has a true-peak limiter insert (486 samples at 48 kHz) and routes `post_pan` into bus
     `b`; its source is digital silence, so its contribution is exact `+0.0` (asserted once from a
     plan of the same session without track `c`). Track `c` sends `post_pan` into `b`, so its edge
     carries exactly 486 samples of compensation (assert it from the compiled plan). `b` has no
     console slots, no inserts and an identity input section, and is observed through a unity route
     from its `input` tap to the output.
   - Live sequence on `c`'s send: mute (`length` 480), 10 idle blocks, unmute (`length` 480).
   - Every output sample over the round trip equals a scalar oracle that applies `c(k)` to `c`'s tap,
     delays the result 486 samples and sums it with `a`'s `+0.0` in D9 order. From the unmute ramp's
     end plus 486 samples, the output equals a fresh plan with the final values.
   - The PR records the mutation "a delayed muted route goes inactive" as red here.

   *Test value: it turns red if a delayed route is skipped while muted (the fade still in its line is
   cut, and on unmute stale or foreign arena samples reach the bus), or if its line is not fed
   zero-coefficient output.*
4. **A settled live edit equals a fresh plan.** For random gain, matrix and mute changes on
   undelayed and delayed sends to two different buses, with targets from
   `graph_compiler::route_coefficients` and `length` in {0, 1, 37, 480, 4800}, every block after the
   ramp (plus the compensation delay, where there is one) is bit-identical, NaNs folded, to a fresh
   plan. Run at 48 kHz and 44.1 kHz.
   *Test value: it turns red if the settled coefficients are computed rather than assigned, if a
   record reaches the wrong route, or if a muted route leaves a contribution the prepared plan does
   not have.*
5. **The drain is bounded and lossless.**
   - `depth + 1` `try_push` calls without a render: the last reports full, and the queue holds
     exactly `depth` records.
   - After one render, exactly the records available at the drain's entry are applied (the
     drained-record counter), in order; the final state equals applying them one by one.
   - A record pushed while the route is inactive is applied at the next block (an unmute after a
     settled mute becomes audible).
   - `RouteControlRecord::new` returns `None` for `length = (1 << 22) + 1` and for `mute = true` with
     a nonzero target.

   *Test value: it turns red if the drain drops or reorders a record, or is skipped while the route
   is inactive.* (Verdict MINOR-2 struck "is unbounded": single-threaded, an unbounded drain applies
   exactly what a bounded one does. The bound is structural: the drain loops over
   `available_at_entry()`, at most the capacity.)
6. **Render allocates nothing.** In a two-thread test, `try_push` runs on one thread while `render`
   runs on another; `allocations == 0` and `frees == 0` around every render call after warm-up,
   measured with `bench_support::alloc`'s thread-scoped counters.
   *Test value: it turns red if the drain, a record's application or the activity write allocates or
   frees on the render thread.*
7. **No live route folds** (VERIFY-2 M3). Eight banked tracks route `post_pan` into one bus whose
   input could be a fold master, with route controls attached: `bank_route_folds()` (a plan-wide
   count) is 0, the bus being the plan's only fold candidate, and a gain record on one of the eight
   sends changes the bus output to the fresh plan's bits after its ramp. The same session without
   route controls keeps its folds (a nonzero count, recorded in the PR).
   *Test value: it turns red if the fold planner folds a live route, which would never drain its
   queue.*
8. **Resources cover the allocation.** `route_control_resources` of the attached producers equals the
   D8 formula computed in the test from the plan's route count, queue capacity and ID lengths, and is
   at least the bytes the attach and the bind allocate, measured with `bench_support::alloc`'s
   thread-scoped counters around them (the #1100 "at least" precedent; allocator rounding makes exact
   byte equality fragile). Run it once for a plan whose live routes have no silencing gate (the
   `RouteActivity` table is charged here) and once for a plan with a muted route (it is not).
   *Test value: it turns red if a queue, an owner box, a route ID or an uncharged `RouteActivity` table
   is left out of the charge the next slice admits against the host's caps.*
9. **No plan without route controls moves.**
   - The existing graph and graph-compiler suites pass unchanged.
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check` and
     `bash scripts/check-graph-determinism.sh` pass (`target/issue6/fresh-process-determinism.json`
     identical to its base).
   - Every existing `route_folds` count is unchanged.
10. **Policy, 4-lane and browser shape.**
    - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
    - `bash scripts/check-graph-policy.sh` and `bash scripts/test-graph-policy.sh`
    - `bash scripts/check-builtins-policy.sh` and `bash scripts/test-builtins-policy.sh`
    - the test-debug-a command (DESIGN section 7)
    - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job at the
      K3 push (`graph`, `graph-compiler` and `builtins-compiler` are in its list)
    - `bash scripts/build-web-audioworklet.sh --named-twin <B> <A>`, then
      `bash scripts/check-web-audioworklet.sh <A> <B>/miso-engine-v1-audio-worklet.simd128.named.wasm`
      (with the raised literal; record the qualifying count before and after)
    - `cargo fmt --all -- --check`
    - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Amendment A1 (root, from the #1217 attempt-2 PASS verdict)

- **MINOR-1.** Add a case with three or more contributors (open, muted, open) to
  `crates/host-core/tests/route_mute.rs`. The mutation "drop the open routes before the muted one"
  (MC: `route_segments` forgets that the opening run stored) must turn it red. That file is added
  to the authorized paths for this case alone.
- **NIT-1.** The live tests cover the arena-form `+0.0` fill of a bus whose every input becomes
  inactive.
- **From the #1219 attempt-2 verdict.**
  - Record the linked kernel's `f32x4` vector:scalar count.
  - Wherever the ramp law is documented, qualify it with the corrected bound from that verdict's
    MINOR-A. With a subnormal step, the overshoot is below `(length - 1) * 2^-149` and never more
    than `2^-128`. "Monotone" holds only for `k < length`, and the snap steps back to the target.

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name, with its one-sentence test-value answer.
- The `MUTATIONS.md` rows and the red mutation results for gates 2, 3 and 7.
- The kernel-shape gate's count before and after.
- Any digest or canonical-text re-pin, with its reason. None is expected.

### Attempt 1 record (Terra)

**Implementation.**

- **Graph, `lib.rs`.**
  - `ROUTE_RAMP_LENGTH_MAXIMUM`.
  - `RouteControlRecord` (D2): `new` refuses a `length` past the maximum, and a mute whose target
    is not four `+0.0` bitwise.
  - `RouteControlLane`: the consumer half.
  - `GraphRouteControlBinding`.
  - `GraphRouteControlProducer`: `route_id`, `free`, `try_push`.
  - `RouteQueueFull { record }`, `GraphRouteControlError`, `RouteControlResources` and
    `route_control_resources`.
  - `PreparedGraphPlan.route_controls: Option<Vec<_>>`, which `new` sets to `None`, and
    `attach_route_controls` (D7).
    - D1 selects the routes whose `RouteDestination` edge lands on a strip `Input` (or the legacy
      `Submix`) node, in canonical route-ID order.
    - Each gets one `bounded_spsc(depth, QueueGeneration(0))`.
    - A second call returns `AlreadyAttached`.
  - `GraphExecutor::new` hands the lanes to `RuntimeParts::with_route_controls`. Neither
    `PreparedGraphPlanParts` nor `GraphBuiltinsCompileRequest` nor `RuntimeOp` gained a field.
- **Runtime, `runtime.rs`.**
  - `NodeKind::LiveRoute(Box<LiveRoute>)`. `LiveRoute` holds `{ ramp, position, mute, delayed,
    route, control }`, and at bind it is `settled(prepared)` with `mute = gate.silences()` (D3).
  - `node_kind`'s route branch takes a lane when one exists (D6).
  - In `execute_op`, a live route drains `available_at_entry()` records first, every block and
    while inactive, applying each by D3 (D5). It then decides `!(mute && position >= length &&
    !delayed)` and writes `RouteActivity::set_active` before anything reads it, returning early
    when inactive (D4). Its mix arm calls `route_mix_ramp_block::<FrameLane>` and saturates
    `position`. All of this is in a `REALTIME_POLICY` region.
  - `build_sequential` builds the table when a gate silences or a lane exists.
  - `route_activity` also returns each route's `delayed` (its consumer `InputRef.delay`), and
    `bind_live_routes` stores the route index and `delayed` in each `LiveRoute`.
  - Both `PlanningMetadata` impls gain `has_route_control`, and `plain_route_gains` declines such
    a route (VERIFY-2 M3).
  - Test-only per-route `test_only_route_drained_counts`, reset with the mix counts.
- **Builtins compiler.** `PreparedBuiltinsGraphArtifact::attach_route_live_controls` delegates to
  the plan (D7). The seal doctests are unchanged and pass.
- **D9.** `--kernel-min` 11 to 13 (base 12, plus the linked kernel), and the ratchet comment is
  updated.
- **Docs.** `docs/BUILTINS_AND_METERING_V1.md` gains "Live routes (issue #1220)": D1 to D5 and D8,
  the **indexed ramp**, why it is not D11, the subnormal-step overshoot qualifier (#1219 attempt-2
  MINOR-A wording), and the
  delayed-route rule. The `RouteActivity` and `route_activity_bound_bytes` doc comments are updated.

**Deviations.**

1. **D8's charge is broken out and slightly wider.**
   - `RouteControlResources` has `routes`, `queue_bytes`, `owner_bytes`, `producer_table_bytes`,
     `route_id_bytes`, `activity_bytes`, `total_bytes` and `largest_allocation_bytes`.
   - `owner_bytes` also charges each lane's `Box<RouteControlLane>` and the plan's
     `GraphRouteControlBinding` entry, which is held from attach until bind consumes it. So the
     charge bounds both the attached and the bound state.
   - `graph::LIVE_ROUTE_OWNER_BYTES` (`size_of::<LiveRoute>()`) is public, so that gate 8 computes
     the formula from first principles.
   - Each producer carries the attach's uncharged activity bytes, and the function takes their
     maximum: one attach's producers describe one plan.
2. **Error variants beyond `AlreadyAttached`.** `BoundRoute` covers a live route the plan requires
   a processor for, which would make it a `GraphNodeBinding`. `QueueCapacity` covers an
   unallocatable depth.
3. **`LiveRoute`'s activity index and `delayed` are set after the table is built**
   (`bind_live_routes`), not in `node_kind`. The consumer's delay is known only from the program
   walk.
4. **The gate 2 sends read `input` taps**, which are the source exactly, so the oracle needs no
   strip model. Gates 3 and 4 use `post_pan` as specified. Gate 3's `c` source is strictly
   positive, so every zero `c-b` mixes is `+0.0`, and `a`'s `+0.0` sum has one sign.
5. **Gate 6.** Before each block, the render thread waits until the producer thread has pushed
   since the last one, and it asserts that the block applied a record. Its red mutation aborts the
   process: `bench_support`'s armed audit raises `SIGABRT` on any render-thread allocation, before
   the counter assertion.
6. **Gate 4** runs 10 trials per rate. Each trial edits `c-b` (delayed) and `e-x` (undelayed),
   and every other trial also edits `a-b`, with the mute drawn at 30%. Every length in {0, 1, 37,
   480, 4800} occurs.

**Tests**, in `crates/graph-compiler/tests/live_routes.rs` unless stated. Each mutation was
applied, run red and reverted. The rows are 1220-1 to 1220-12 in `crates/graph/tests/MUTATIONS.md`.

- `idle_attached_lanes_change_nothing` (gate 1). It is red if an idle lane's bind state is not
  the prepared, gated coefficients, or if attaching changes activity (1220-1, 1220-2).
- `the_block_in_which_a_mute_ramp_ends_is_mixed_whole` (gate 2, against the scalar oracle). It is
  red if activity is decided after the mix or mid-block, which drops frames 0 to 94 of block 3
  (1220-3).
- `a_bus_whose_every_send_goes_inactive_is_refilled_with_positive_zero` (#1217 NIT-1). It is red
  if the arena-form sum skips an all-inactive destination and leaves a stale earlier sum on the
  bus (1220-10, V3c). No other test reaches a bus that was dirty before going all-inactive.
- `a_delayed_send_round_trips_through_mute_without_a_cut_or_stale_audio` (gate 3). The delay of
  486 is asserted from the compiled plan, and `a`'s `+0.0` from the plan without `c`. It is red if
  a delayed muted live route goes inactive and its fade in the line is cut (1220-4), or if its line
  is not fed the zero-coefficient mix.
- `a_settled_live_edit_equals_a_fresh_plan` (gate 4, 48 kHz and 44.1 kHz). It is red if a record
  reaches the wrong route (1220-5a), or if a muted route keeps a contribution the prepared plan
  lacks (1220-5b). The lane suite's #1219 M2 holds "settled computed".
- `the_drain_is_bounded_and_lossless` (gate 5). It is red if the drain drops or bounds records
  wrongly (1220-6a), or is skipped while inactive (1220-6b). The order is checked both ways: the
  output must equal "last record only" and differ from "reversed".
- `live_routes_render_without_allocating` (gate 6). It is red if a drain, an application or the
  activity write allocates on the render thread (1220-7). This is the only test that drains
  records pushed concurrently by another thread.
- `no_live_route_folds` (gate 7). It is red if the fold folds a live route (1220-8). The prepared
  bus folds 8 lanes and the live one 0.
- `route_control_resources_cover_the_allocation` (gate 8). It is red if a queue, a box, an ID or
  the uncharged activity table leaves the charge (1220-9a, 1220-9b).
  - No silencing gate: charged 1659 bytes, retained 1459.
  - A muted route: charged 1444, retained 1316.
- `route_controls_attach_once` (D7). It is red if a second attach replaces the first's lanes and
  orphans its producers (1220-12).
- `crates/host-core/tests/route_mute.rs::a_middle_inactive_route_keeps_every_active_contribution`
  (A1 MINOR-1, adapted from the verifier's probe). It is red if a later active run stores over an
  earlier one (1220-11, MC: `Bus, 3 inputs, muted [1], plane 0: sample 0: 0.29300022 !=
  0.36220396`). It is red in this test alone.

**Gates** (head of this commit, on base `0268a1c74`; x86-64-v3 AVX2, AMD EPYC 7313P).

- Gate 9.
  - The release build of `audit`, `bench`, `capi` and `session-validator`: rc 0.
  - `cargo test --release -p audit -p bench -p console-workload`: 110 passed, 0 failed, 2
    ignored. The `route_folds` counts are unchanged.
  - `graph_fixture -- --check`: rc 0.
  - `check-graph-determinism.sh`: PASS (100/100), and the JSON is `cmp`-identical to the one
    `graph_fixture` built at base `cdde008f2` produced. This is a compile fingerprint.
  - Render bits: the browser identity fixture's PCM digests pass (`check-browser-expected-resources`).
    `idle_attached_lanes_change_nothing` covers attach-idle against no attach.
- Gate 10.
  - test-debug-a (DESIGN 7, `--no-fail-fast`): rc 0. 1205 passed, 0 failed, 9 ignored, 107
    binaries; that is 1194 plus the 11 new tests.
  - The realtime check and test policies: ok (54 regions in 15 files). Graph: PASS and ok.
    Builtins: ok and ok. Workspace policy: ok.
  - `cargo fmt --all -- --check`: rc 0. Workspace `clippy --all-targets --all-features -D
    warnings`: rc 0. (Verdict MINOR-3 corrected an earlier sentence here: `cargo clippy --locked -p
    graph -p builtins-compiler --all-targets -- -D warnings`, without features, is **not** clean.
    It reports two dead-code errors in `builtins-compiler`'s lib test target, `initial_matrix_state`
    and `BoundaryVariant::{Nonadjacent, NonadjacentOutputConflict}`, identical at parent
    `0268a1c74`, so pre-existing; filed as its own issue, see the verdict record below.)
  - Browser (the D9 instantiation is now linked):
    - `build-web-audioworklet.sh --named-twin`: rc 0. The module is `c21d647a...`, 2,740,212 bytes
      against 2,726,858 at base (+13,354, +0.49%).
    - `check-web-audioworklet.sh`: rc 0. The render closure's only trap owner is
      `PreparedRenderPlan::render_inner`, as before.
    - Kernel shape **before: 12** (`f32x4_arith=9350`, base `0268a1c74`, where `--kernel-min 13` is
      red). **After: 13** (`f32x4_arith=9395`).
    - `route_mix_ramp_block<f32x4>` is **22 vector : 0 scalar**, with no `unreachable`.
    - `check-browser-expected-resources.py --artifacts`: rc 0. The digests agree, and the self-test
      shows 32 red mutations.
    - `check-scalar-oracle-absent.py`: rc 0. `test-web-audioworklet.sh`: rc 0. The V8 spill gate
      (Node v22.23.2): ok.
  - `run-aarch64-tests.sh debug`: no arm64 host here, so it runs **at batch push** (CI
    `aarch64-debug`). On this host the live routes run `FrameLane = Simd8`. The simd128 module
    carries the 4-lane instantiation.
- **No re-pin.**

### K3 follow-up record (after the attempt 1 PASS verdict)

Applied in the K3 follow-up commit (on `eff44271d`, branch `codex/batch-submix-k3`):

- **MINOR-1.** `route_control_resources` charges each route ID twice: the producer's copy and the
  binding's node-ID copy, which the plan holds from attach until bind. Its rustdoc and
  `docs/BUILTINS_AND_METERING_V1.md` call it an upper bound on the attached and the bound state;
  every part is the exact size of what it names except the activity table, which is
  `route_activity_bound_bytes`. New test `route_control_resources_cover_the_attached_state`
  (two 127-byte route IDs, a muted plan): what the attach alone retains is within the charge. Test
  value: red if the charge counts each route ID once. Mutation (count once): RED, "the charge 1694
  covers the 1804 bytes the attach retains". Gate 8's formula counts the IDs twice; its numbers are
  now 1,663 (no silencing gate) and 1,448 (muted).
- **MINOR-2.** Gate 5's test value (above and in the test's doc) no longer claims an unbounded
  drain is red; the bound is structural.
- **MINOR-3.** The record's clippy sentence is corrected; the pre-existing dead code is filed as
  *Remove the dead code builtins-compiler reports under no-features clippy* (#1235).
- **NIT-1.** `scripts/check-web-audioworklet-callgraph.py`'s `KERNEL_ROSTER` gains
  `route-mix-ramp f32x4` (`lane7kernels20route_mix_ramp_block.*4wide6f32x4`, ceiling 0.10; head:
  vector 22, scalar 0, budget 8). Mutation: `route_mix_settled_tail` made `#[inline(always)]`,
  module rebuilt: the row fails (`vector=22 scalar=18 budget=8.0`), while the base's analyser,
  without the row, passes the same module. The analyser's self-test passes.
- **NIT-2, NIT-3.** No change (recorded by the verdict as acceptable).

## Verdict

- **Attempt 1** (`d405abb37`): Sol PASS. Three MINORs and three NITs, applied or answered above.
  `docs/handoffs/submix-sends-2026-10-02/verdicts/1220-attempt1.md`; probes `docs/handoffs/submix-sends-2026-10-02/verdicts/1220-attempt1-verifier-scratch.rs`.

## Dependencies

- *Let a route into a submix follow its source strip's mute in the session* (#1218)
- *Ramp a send's coefficients with the indexed ramp kernel* (#1219)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- Render stays free of allocation, locks and syscalls. Only `crates/lane` names `wide` or
  intrinsics.
- One kernel shape for every target, generic over `Lane`.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the K3 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
