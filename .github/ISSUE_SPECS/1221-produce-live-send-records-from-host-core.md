# Produce live send records from host-core

Slice 23 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

Every host that prepares through host-core with live controls gets one producer per send: the
browser now, and the C ABI once #1053 and *Deliver value-only send and submix-strip edits to the
running C ABI plan* (#1225) attach live controls there. The producer takes a send's gain, matrix, mute and
source-lane mutes, checks them with the same function the compiler uses, and pushes one ramp record
or refuses with nothing pushed. Every byte the lanes retain is charged against the host's caps.

The standing console sessions keep every fold they have today when live controls are on, because
their routes go only to the output.

## Context (verified on `fe8ac679`)

Batch K2 renamed `HostLiveControlHandles.tracks` to `strips` (with `track_count`) and
`track_controls` to `strip_controls`, and made the solo state per strip; re-read
`crates/host-core/src/prepare.rs` line numbers, which K2 moved.

- **The live-control request.** `HostLiveControlRequest` (`crates/host-core/src/prepare.rs:279-312`)
  carries `control_queue_depth: Option<NonZeroUsize>` (`:286`). `None` attaches nothing, and the plan
  renders the byte-identical live-control-free path.
- **Effect channels are the template.**
  - With a depth set, host-core calls `attach_effect_live_controls` (`prepare.rs:887-892`) and
    builds the strip control requests (`:910-928`).
  - It charges the effect channels through `effect_control_resources`
    (`crates/effect-compiler/src/prepare.rs:687`) into `admitted_graph_and_model`, checked against
    `maximum_graph_session_plus_plan_bytes` with `host.graph.resource.limit`, and into the
    largest-allocation check with `host.resource.limit` (`prepare.rs:1073-1093`).
  - The report carries them as `HostPrepareReport.effect_control_resources` (`prepare.rs:256`, set at
    `:1265`). `HostPrepareReport` is built once (`:1236`) and by the test helper `report()`
    (`crates/host-core/tests/prepare.rs:44`); capi copies named fields of it
    (`crates/capi/src/runtime/compile.rs:425-436`), so a new field does not break capi.
- **Compile and bind.** host-core compiles with `GraphCompiler::compile_with_builtins` into an
  immutable binding, `let artifact = ...` (`prepare.rs:1026`), and binds with
  `into_bound_with_source_set` (`:1172`). The live handles are built once (`:1299-1306`), the only
  construction site of `HostLiveControlHandles` (`:342-366`).
- **After *Ramp live send coefficients on the render plane* (#1220):**
  - `PreparedBuiltinsGraphArtifact::attach_route_live_controls(&mut self, depth)` creates one bounded
    queue per route into a submix, in canonical route-ID order, and returns
    `GraphRouteControlProducer { route_id, .. }` with `free()` and
    `try_push(RouteControlRecord) -> Result<(), RouteQueueFull>`; a second call is refused;
  - `RouteControlRecord::new(target, mute, length)` returns `None` for `length > ROUTE_RAMP_LENGTH_MAXIMUM`
    (`1 << 22`) and for `mute` with a nonzero target;
  - `graph::route_control_resources(&producers)` states exactly the bytes the attach and the bind
    add;
  - the render plane ramps with the indexed ramp law, applies the activity and delayed-route rules,
    and never folds a live route.
- **After *Gate every route's coefficients through one function* (#1215), *Mute a route in the session* (#1216) and
  *Let a route into a submix follow its source strip's mute in the session* (#1218):** `graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted)
  -> Result<[f32; 4], RouteValueError>` is the single coefficient and domain authority, and the
  compiler's lowering uses it. host-core depends on graph-compiler (`crates/host-core/Cargo.toml:25`);
  host-web depends on host-core but not on graph (`hosts/host-web/Cargo.toml:27-30`).
- **Fold counts.** The engine's render plan exposes `bank_route_folds()`
  (`crates/engine/src/realtime/plan.rs:744`). The console benchmark's fold test
  (`tools/console-workload/tests/chain_shape.rs:346`) prepares through builtins-compiler directly,
  never through host-core, so it cannot see host-core's live-control path (VERIFY-1 MAJOR-10).
- **Live controls are opt-in.** The browser's `WebBootOptions::explicit_defaults()` and the SDK
  default the command queue to 0 (`hosts/host-web/src/lib.rs:1102-1104`, `:1137`;
  `sdk/src/core/abi.ts:186`). The producer's mixer opts in with 64. The C ABI prepares with none
  (`crates/capi/src/runtime/compile.rs:408-410`) until #1053.

## Decisions frozen for this slice

- **D1. Attachment.** `let artifact` at `prepare.rs:1026` becomes `let mut artifact` (VERIFY-2
  MINOR 8). When `control_queue_depth` is `Some(depth)`, host-core calls
  `artifact.attach_route_live_controls(depth)` after `compile_with_builtins` and before binding.
  When it is `None`, nothing is attached, and the plan is byte for byte today's. The attach precedes
  the cap check (`prepare.rs:1077-1093`), so the route charge joins `admitted_graph_and_model` beside
  the effect-control charge and is refused by the same `host.graph.resource.limit` /
  `host.resource.limit` checks.
- **D2. The producer**, in a new host-core module exported from `lib.rs`:

  ```rust
  pub struct RouteControlProducer { pub route_id: Box<str>, /* graph producer */ }
  #[derive(Clone, Copy, Debug, Eq, PartialEq)]
  pub enum RouteControlError { Domain, Length, Full }
  impl RouteControlProducer {
      pub fn free(&self) -> usize;
      /// Pure: validates and builds the record, pushes nothing.
      pub fn record(&self, gain_db: f32, matrix: [f32; 4], mute: bool,
                    source_lane_muted: [bool; 2], length: u32)
          -> Result<RouteControlRecord, RouteControlError>;          // Domain or Length
      /// Pushes a record built by `record`; `Full` pushes nothing.
      pub fn push(&mut self, record: RouteControlRecord) -> Result<(), RouteControlError>;
      /// `record` then `push`.
      pub fn set(&mut self, gain_db: f32, matrix: [f32; 4], mute: bool,
                 source_lane_muted: [bool; 2], length: u32) -> Result<(), RouteControlError>;
  }
  ```

  - `record` computes the target with
    `graph_compiler::route_coefficients(gain_db, matrix, mute, source_lane_muted)` (VERIFY-2 M1) and
    the record's `mute` as `mute || source_lane_muted == [true, true]`, and builds it with
    `RouteControlRecord::new`.
  - Errors in order: `Domain` when `route_coefficients` refuses; `Length` when `length > 1 << 22`;
    `Full` when the queue has no room, decided from `free()` before `try_push`. Every error pushes
    nothing.
  - The `record`/`push` split exists for staged all-or-nothing admission: validate every record,
    check every queue's room, and only then push. *Admit live send commands in the browser* (#1222) and the
    C ABI slices use it.
  - host-core re-exports `RouteControlRecord` and `RouteControlResources` so host-web needs no graph
    dependency.
- **D3. Handles.** `HostLiveControlHandles` gains `pub route_controls: Vec<RouteControlProducer>`:
  one per route into a submix, in canonical route-ID order, and empty when no channel was requested.
  Routes into the output have none.
- **D4. Resources** (VERIFY-2 MINOR 5). `HostPrepareReport` gains
  `pub route_control_resources: RouteControlResources`, from `graph::route_control_resources`. Its
  total joins the `admitted_graph_and_model` sum (refused with `host.graph.resource.limit` above
  `maximum_graph_session_plus_plan_bytes`), and its largest allocation joins the named-allocation
  check (`host.resource.limit`), exactly as the effect channels do. Without a depth it is all zero.
- **D5. Concurrency.** A host batching several routes checks every `free()` before any `push`.
  `set` itself never partially pushes.

## Deliverables

1. `RouteControlProducer` and `RouteControlError`, in a new host-core module exported from `lib.rs`,
   with the re-exports (D2).
2. The attachment in the preparation path (D1), the `route_controls` field (D3) and the resource
   charge and report field (D4).
3. host-core tests (gates below), in a new file `crates/host-core/tests/live_routes.rs`.
4. A rustdoc note on `HostLiveControlHandles::route_controls`: once #1053 attaches live controls in
   the C ABI, C ABI plans get these producers too, and their settled bits are identical to static
   routes by construction (gate 1).
5. `crates/host-core/tests/MUTATIONS.md` rows for the red mutations named in the gates.

## Authorized paths

- `crates/host-core/src/{prepare.rs,lib.rs}` and one new module beside them
- `crates/host-core/tests/live_routes.rs` (new), `crates/host-core/tests/prepare.rs` (the `report()`
  helper only), `crates/host-core/tests/MUTATIONS.md`
- this spec

## Non-goals

- No browser command kind or export (*Admit live send commands in the browser*).
- No C ABI path (*Deliver value-only send and submix-strip edits to the running C ABI plan*).
- No follow-mute composition (*Let a send follow its source strip's mute live in the browser*, #1224): the
  caller supplies `source_lane_muted`.
- No live output routes. No change to the render plane.

## Hazards

- **One authority.** The producer must not re-implement the coefficient arithmetic or the domain
  check. A copy can drift from the compiler's bits.
- **The acked-batch question.** `set` refuses before pushing, and `Full` is decided by `free()`
  before `try_push`. A refusal leaves the queue unchanged.
- **Folds in live-controlled plans.** A live route never folds. The standing sessions route only to
  the output, whose routes stay constants, so they must keep their folds. Gate 4 pins it.

## Objective gates

Every rendered comparison uses a stateless downstream: the destination bus has no console slots, no
stateful inserts and an identity input section (HPF and LPF off; VERIFY-2 MINOR 7). Sources are
distinct and non-constant per track and lane.

1. **The live target is the prepared constant.** For random gains (in the accepted domain),
   matrices, mutes and `source_lane_muted` values, the record a producer builds carries exactly the
   bits of `graph_compiler::route_coefficients` for the same arguments, and a plan prepared from a
   session holding those values (with `follows_mute` and the source mutes) renders the same settled
   block as the live one.
   *Test value: it turns red if the producer computes its target, or its follow-muted columns, by any
   path other than the compiler's.*
2. **A settled live edit through host-core equals a fresh plan.**
   `prepare_host_session_with_live_controls` with a depth; random gain, matrix and mute on two sends
   into two different buses, with `length` in {0, 480}. After the ramp (plus any compensation delay),
   the output is bit-identical to a host prepared from the edited session and fed the same sources
   from sample 0. Run at 48 kHz and 44.1 kHz.
   *Test value: it turns red if host-core attaches lanes to the wrong routes, in the wrong order, or
   pushes stale values.*
3. **Nothing partial.**
   - `depth + 1` `set` calls without a render: the last returns `Full`, and the queue holds exactly
     `depth` records.
   - A `Domain` refusal (a non-finite gain) and a `Length` refusal (`(1 << 22) + 1`) push nothing;
     `free()` is unchanged, and after a render no record was applied.
   - `record` alone pushes nothing, and `push` of its result equals `set` with the same arguments,
     bit for bit, in the applied state.

   *Test value: it turns red if a refused call pushes, or if `Full` is detected only after a push.*
4. **Live controls cost the standing sessions no fold.**
   - `fixtures/session/v1/console-sixty-four-track-intended.json` is prepared through host-core twice:
     with `control_queue_depth: None` and with `Some(64)`.
   - The two plans report the same `bank_route_folds()`, and it is nonzero. If the base commit
     already reports different counts for `None` and `Some(64)`, record both as PR evidence and stop
     for a Sol ruling; never commit a base-commit literal.
   - Over 8 blocks of the same source content, the two render bit-identical output.
   - `route_controls` is empty for this session (it has no route into a submix).

   *Test value: it turns red if attaching live controls binds or declines routes into the output and
   so loses the master fold (VERIFY-1 MAJOR-10). The console benchmark's fold test never prepares
   through host-core.*
5. **Handles.** `route_controls` lists exactly the routes into submixes, in canonical route-ID order,
   each with its `route_id`, for a session whose route IDs are declared out of canonical order; it is
   empty without a depth.
   *Test value: it turns red if host-core orders producers by declaration rather than by the order
   the render plane's lanes were attached in.*
6. **Resources.** With `maximum_graph_session_plus_plan_bytes` set one byte below the plan's admitted
   total including the route lanes, preparation refuses with `host.graph.resource.limit`; at the
   total it prepares, and `report.route_control_resources` equals `graph::route_control_resources`
   of the producers. Without a depth the field is zero (that the total equals the base commit's is PR
   evidence).
   *Test value: it turns red if the route queues or owner boxes are uncharged, or charged on a
   live-control-free plan.*
7. **Render allocates nothing.** In a two-thread test, `set` runs on one thread while `render` runs
   on another; `allocations == 0` and `frees == 0` around every render call after warm-up, measured
   with `bench_support::alloc`.
   *Test value: it turns red if host-core's attachment leaves any lane state to be built on the render
   thread.*
8. **Policy and 4-lane:**
   - `bash scripts/check-host-core-policy.sh` and `bash scripts/test-host-core-policy.sh`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - the test-debug-a command (DESIGN section 7)
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` at the K3
     push (`host-core` is in its list)
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator` and
     `./target/release/audit capi` (the C ABI prepares without route live controls, so nothing
     moves). **Conditional on #1053** (VERIFY-3 MINOR 10), on the model of *Let a route into a submix
     follow its source strip's mute in the session* D6: if #1053 has landed and its C ABI
     preparation requests live controls with a queue depth, either keep that request builtins-only
     (no route lanes) until *Deliver value-only send and submix-strip edits to the running C ABI
     plan*, or re-pin the capi resource oracles and `audit capi` here, each with its reason.
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name, with its one-sentence test-value answer.
- Gate 4's fold counts (and the base commit's, if the fallback applied).
- Any digest or canonical-text re-pin, with its reason. None is expected.

## Dependencies

- *Ramp live send coefficients on the render plane* (#1220)
- *Give every strip one mute owner and live-control producers in host-core* (#1211), for the
  `HostLiveControlHandles` shape batch K2 left

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" gates are hard stops. NaNs are folded (decision 10).
- Render stays free of allocation, locks and syscalls.
- No ack precedes a drop: every refusal is decided before the first push.
- A test that greps source or prose is refused.
- Commit on the K3 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
