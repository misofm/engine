# Gate every route's coefficients through one function

Slice 18a of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3, pushed once with slices 18a-26. It may be drafted while batch K2 is in
review, but it is implemented on the K2 head and merges after *Drive submix strips from the SDK live
controls* (#1214). It is the class-A first half of the former slice 18 (VERIFY-3 MINOR 4.1); *Mute a route
in the session* (#1216) is the second half.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

No audible change. Every prepared route carries a gate, and the coefficients a plan binds for a route
come from one function that a later live producer will call too. This freezes the coefficient
interface every later live-send slice uses (DESIGN P11, 5.7), so that *Mute a route in the session*,
*Let a route into a submix follow its source strip's mute in the session* (#1218) and the live-send slices
only set gate values and never change the interface or its 17 `PreparedRoute` literals again.

In this slice every gate is `RouteGate::OPEN`, so every plan binds exactly today's bits.

## Context (verified on `fe8ac679`)

Batches K1 and K2 changed the route source (`RouteSource::Submix { submix_id, tap }`) and the submix
model; none of the anchors below moved in substance. Re-read line numbers before editing.

- The compiler lowers each route at `crates/graph-compiler/src/compile.rs:322-351`: `route_transform`
  (`crates/graph-compiler/src/ids.rs:273-289`, `math::db_to_gain_f32`, finite and non-subnormal
  checks) then `route_transforms.push(PreparedRoute { node, transform })` (`compile.rs:345`). A
  refusal is `graph.gain.non_finite` at `$.routes[id=<id>].gain_db` (`compile.rs:323-327`).
- `RouteTransform` (`crates/graph/src/lib.rs:742-749`) and `PreparedRoute { node, transform }`
  (`:2204-2207`) carry the **unfolded** gain and matrix. The canonical text writes one
  `route-transform` row per route from those bits (`crates/graph-compiler/src/canonical.rs:262-273`).
- The runtime folds the gain at bind with the private `const fn folded_route`
  (`crates/graph/src/runtime.rs:5990-5997`). Two callers: `node_kind`'s route arm (`:4062-4063`,
  `NodeKind::Route(folded_route(&transform))`) and `plain_route_gains` (`:6110-6122`), which the fold
  planner asks through `PlanningMetadata` (`:6000-6014`; impls for `RuntimeParts` `:6016` and
  `BorrowedPlanningMetadata` `:6089`). The runtime keeps routes as
  `BTreeMap<GraphNodeId, RouteTransform>` (`:3891`, filled at `:3984`).
- The test-only `route_folds_over_program` (`runtime.rs:6811-6816`) takes
  `&BTreeMap<GraphNodeId, RouteTransform>`; its callers are `crates/graph/src/program/tests.rs:1782`
  and `:2071` (VERIFY-3 MINOR 22).
- `crates/graph` cannot call `math` (`scripts/check-graph-policy.sh:21-22` pins its production
  dependencies to `effect-contract`, `engine`, `lane` and `rack`). host-core depends on
  graph-compiler (`crates/host-core/Cargo.toml:25`).
- **Every `PreparedRoute {` literal** (each gains one field in this slice):
  `crates/builtins-compiler/src/lib.rs:6178`; `crates/graph-compiler/src/compile.rs:345`;
  `crates/graph-compiler/tests/scale.rs:417`; `crates/graph/src/lib.rs:3514`, `:4634`, `:4690`,
  `:5544`, `:5548`, `:6243`, `:6516`, `:6520`; `crates/graph/src/runtime.rs:10313`, `:10836`,
  `:12747`; `crates/graph/tests/rt1_direct_bank_alloc.rs:230`,
  `crates/graph/tests/rt9_resident_bank_input_alloc.rs:424`,
  `crates/graph/tests/rt10_source_in_place_alloc.rs:274`. Re-run `git grep -n 'PreparedRoute {'` at
  the K3 base before editing.

## Decisions frozen for this slice

- **D1. The coefficient interface** (DESIGN 5.7; frozen here for every later slice).
  - In `crates/graph`:

    ```rust
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct RouteGate { pub mute: bool, pub follow_zeroed: [bool; 2] }
    impl RouteGate {
        pub const OPEN: Self = Self { mute: false, follow_zeroed: [false; 2] };
        /// Muted, or follow-zeroed on both source lanes: the route contributes nothing.
        pub const fn silences(self) -> bool;
    }
    pub struct PreparedRoute { pub node: GraphNodeId, pub transform: RouteTransform, pub gate: RouteGate }
    pub fn gated_route_coefficients(transform: &RouteTransform, gate: RouteGate) -> [f32; 4];
    ```

    `gated_route_coefficients` returns `[+0.0; 4]` when `gate.silences()`. Otherwise it returns
    exactly today's `folded_route` bits `[gain * ll, gain * lr, gain * rl, gain * rr]`, with `ll` and
    `rl` set to `+0.0` when `follow_zeroed[0]` and `lr` and `rr` set to `+0.0` when
    `follow_zeroed[1]`. It replaces `folded_route`: `node_kind` binds
    `gated_route_coefficients(&route.transform, route.gate)`, and the fold planner uses it with
    `RouteGate::OPEN` for the routes it may fold.
  - `mute` and `follow_zeroed` exist from this slice so that the 17 `PreparedRoute` literals change
    once; every literal and the compiler set `RouteGate::OPEN` until *Mute a route in the session*
    and *Let a route into a submix follow its source strip's mute in the session*.
  - In `crates/graph-compiler`, exported from `lib.rs`:

    ```rust
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RouteValueError { Domain }
    pub fn route_coefficients(gain_db: f32, matrix: [f32; 4], mute: bool,
                              source_lane_muted: [bool; 2]) -> Result<[f32; 4], RouteValueError>;
    ```

    It runs today's `route_transform` checks, then `gated_route_coefficients` with
    `RouteGate { mute, follow_zeroed: source_lane_muted }`, and returns `Domain` if `route_transform`
    refuses **or** any folded coefficient is not finite (VERIFY-2 MINOR 3: a finite gain times a
    finite coefficient can overflow, for example +700 dB with `ll = 1e10`).
  - The compiler's route lowering calls `route_coefficients(gain_db, matrix, false, [false; 2])` for
    its domain check and keeps `route_transform`'s unfolded transform plus `RouteGate::OPEN` in
    `PreparedRoute`. A `Domain` refusal is `graph.gain.non_finite` at `$.routes[id=<id>].gain_db`, as
    today. No checked-in session folds to a non-finite coefficient, so nothing that compiles today is
    refused.
- **D2. The runtime carries the gate.** The runtime's route map holds the gate with the transform
  (`BTreeMap<GraphNodeId, (RouteTransform, RouteGate)>`, or the `PreparedRoute` itself), and
  `PlanningMetadata::route` returns both (both impls). `plain_route_gains` binds
  `gated_route_coefficients` of what it gets; it does not yet decline a non-open gate (none exists
  until *Mute a route in the session*, which adds the decline). `route_folds_over_program` keeps its
  `&BTreeMap<GraphNodeId, RouteTransform>` signature and wraps each entry with `RouteGate::OPEN`, so
  `crates/graph/src/program/tests.rs` is untouched.
- **D3. Graph text.** Unchanged. No gate is set, so no row is added and no digest moves.

## Deliverables

1. Graph: `RouteGate`, `PreparedRoute.gate`, `gated_route_coefficients` replacing `folded_route`,
   the gate in the runtime's route map and in both `PlanningMetadata` impls (D1, D2), exported from
   `lib.rs`.
2. Graph compiler: `route_coefficients` and `RouteValueError`, exported from `lib.rs`, and the
   lowering (D1).
3. `gate: RouteGate::OPEN` in every `PreparedRoute {` literal in the Context.
4. One graph-compiler unit test (gate 2), which *Mute a route in the session* and *Let a route into a
   submix follow its source strip's mute in the session* later extend.

## Authorized paths

- `crates/graph/src/{lib.rs,runtime.rs}` and
  `crates/graph/tests/{rt1_direct_bank_alloc,rt9_resident_bank_input_alloc,rt10_source_in_place_alloc}.rs`
  (the `PreparedRoute` literals)
- `crates/graph-compiler/src/{compile.rs,ids.rs,lib.rs}`, `crates/graph-compiler/tests/scale.rs` (the
  literal) and one new test file in `crates/graph-compiler/tests/`, for example
  `route_coefficients.rs`
- `crates/builtins-compiler/src/lib.rs` (the `PreparedRoute` literal at `:6178` only)
- this spec

## Non-goals

- No session field, wire field, opcode, SDK change, document migration or canonical-text row (*Mute
  a route in the session*).
- No fold decline: every gate is open (*Mute a route in the session*).
- No behaviour change of any kind; no performance work.

## Hazards

- **One function, two layers.** `crates/graph` must not grow a `math` dependency; the domain check
  and the dB conversion stay in graph-compiler. The bits a plan binds and the bits a live producer
  will push are the same only because both go through `gated_route_coefficients`. Gate 2 pins it.
- **Exact bits.** With `RouteGate::OPEN`, `gated_route_coefficients` must return `folded_route`'s
  four products exactly (same operand order, no fused multiply-add). Gate 1 holds every existing
  plan to its base bits.

## Objective gates

1. **No plan moves.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/check-graph-determinism.sh`, with `target/issue6/fresh-process-determinism.json`
     identical to its base
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`, with no re-pin
   - `cargo test --locked --release -p audit -p bench -p console-workload`, with every existing
     `route_folds` count unchanged
   - `./target/release/audit capi`, unchanged
2. **One coefficient function.** A graph-compiler unit test draws random finite gains and matrices
   and asserts:
   - `route_coefficients(gain_db, matrix, false, [false; 2])` equals, bit for bit, the constant the
     runtime binds for the same route (`gated_route_coefficients` of the compiled `PreparedRoute`);
   - for random `mute` and `source_lane_muted`, `route_coefficients` equals
     `gated_route_coefficients(&transform, RouteGate { mute, follow_zeroed: source_lane_muted })`,
     which is `[+0.0; 4]` when muted or both lanes are zeroed and zeroes exactly the named column
     otherwise;
   - a gain of 700 dB with `ll = 1e10` returns `Domain` and refuses compile with
     `graph.gain.non_finite`.

   *Test value: it turns red if the prepared and the domain-checked paths compute coefficients
   differently, if a gate zeroes the wrong column, or if an overflowing fold is accepted.*
3. **Workspace, policy and 4-lane.**
   - the test-debug-a command (DESIGN section 7)
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `for x in graph builtins realtime workspace; do bash scripts/check-$x-policy.sh && bash scripts/test-$x-policy.sh || exit 1; done`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job at the
     K3 push (`graph`, `graph-compiler` and `builtins-compiler` are in it)

No allocation gate: this slice adds no render-thread state.

## Evidence

- The output of every gate command above, from the PR's head commit.
- The new test's name with its one-sentence test-value answer.
- A one-time "no bit moved" comparison against the base (PR evidence, not a committed test).

## Dependencies

- *Build submix strips and bus taps in the SDK and teach agents to author them* (#1205, batch K1 closed and
  pushed)
- Merges after *Drive submix strips from the SDK live controls* (#1214, batch K2)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" and "unchanged" gates are hard stops. NaNs are folded (decision 10).
- Render stays allocation-, lock- and syscall-free.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the K3 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
