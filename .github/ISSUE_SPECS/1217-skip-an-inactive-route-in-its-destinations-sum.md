# Skip an inactive route in its destination's sum

Slice 19 of *Submix strips and live aux sends* (#1196, the umbrella filed by *Record the submix, send and
VCA ruling*, #1197). Batch K3.

The design record cited below (`DESIGN`, `VERIFY-1` to `VERIFY-3`, `REVISION-1`, `REVISION-2` and
`APPLIED-3`) is committed in `docs/handoffs/submix-sends-2026-10-02/`.

## Product outcome

A muted send costs nothing and adds nothing: an undelayed muted route is neither mixed nor read by
its destination, so a session that mutes ten sends does ten sends' less work, and a bus whose every
contributor is muted renders exact `+0.0`.

This slice implements the contribution and activity rule every later live-send slice builds on
(DESIGN P4, 5.4), including the PDC-delayed case VERIFY-1 BLOCKER-1 found: **a delayed route is
never inactive**, so its compensation line can never replay stale audio on a later unmute.

No checked-in document has `mute: true`, so no pinned bit moves.

## Context (verified on `fe8ac679`)

- **After *Gate every route's coefficients through one function* (#1215) and *Mute a route in the session* (#1216):**
  - `PreparedRoute { node, transform, gate: RouteGate }`, with `RouteGate::silences()` true when
    muted (or, after *Let a route into a submix follow its source strip's mute in the session* (#1218),
    follow-zeroed on both lanes);
  - `graph::gated_route_coefficients` binds `[+0.0; 4]` for a silenced route, and the route op still
    runs (mixes zeros);
  - `plain_route_gains` declines a route whose gate is not open, so no fold involves a silenced
    route.
- **Reductions** (`crates/graph/src/runtime.rs`):
  - `reduce_plane` (`:399-410`): `[]` fills `0.0`; `[single]` copies, or does nothing when the input
    is the op's own output (`if *single != out`, `:402-406`), which is how an in-place read costs
    nothing; two or more inputs go to `reduce_many` (`:426-455`), in groups of `REDUCE_GROUP = 8`
    with `initial_store` true only for the first group, through `reduce_group` (`:458-475`) and
    `accumulate_group`.
  - The host-master forms are `reduce_plane_into` (`:562-577`) and `reduce_many_into` (`:583`).
  - D9 order: inputs are in stable edge-ID order (route-ID order for a bus input).
- **In-place lowering.** A single input is read in place when the op is its sole reader and the
  buffer is not dedicated (`crates/graph/src/program.rs:715-745`, `is_dedicated` `:231`). A route
  that is the sole reader of its tap therefore mixes in place and owns no buffer; when it is skipped,
  that buffer still holds the raw tap.
- **PDC staging belongs to the consumer.** `execute_op` (`runtime.rs:2970-3060`) stages every
  `op.staged` input unconditionally (`:2996-3006`) before its reduction. `DelayRef { line, staging }`
  is per consumer input (`program.rs:49-58`), and `InputRef { buffer, delay }` (`:62-65`) says, at
  lowering, whether an input is delayed. A route node has one input, so its compensation is 0
  (`crates/graph-compiler/src/pdc.rs:63-73`); a route's delay can only sit on its edge into its
  destination, and a route has exactly one consumer.
- **Bank gathers.** `bank_gather_source` (`runtime.rs:2949-2960`) lets a bank member read its single
  input directly. A route destination is a submix strip's `Input` stage or the session Output, never
  a bank member, so a gather never bypasses the reduction.
- **The op layout is a reported byte.** `size_of::<RuntimeOp>()` enters the resource estimate
  (`scalar_split_op_layout`, `runtime.rs:1298-1313`; `crates/graph-compiler/src/ids.rs:290-294`), so
  live-control state travels **beside** the ops, never as a `RuntimeOp` field
  (`RuntimeOp` at `runtime.rs:983-997`).
- **The estimate.** `resource_estimate` (`crates/graph-compiler/src/estimate.rs:15-140`) charges
  bind-time metadata in `graph_metadata_bytes`. `GraphResourceEstimate`
  (`crates/graph/src/lib.rs:357-389`) has struct literals in tests (`graph-compiler/tests/scale.rs:309`,
  `graph/tests/rt{1,9,10}_*.rs`), so this slice adds no field to it.

## Decisions frozen for this slice

- **D1. Inactive** (DESIGN P4). A route is inactive when its gate silences it **and** its input
  into its destination carries no compensation delay (`InputRef.delay` is `None`). For a prepared
  route the bit is fixed at bind. *Ramp live send coefficients on the render plane* (#1220) later rewrites a
  live route's bit once per block; build the table so it can (plain data, no trait object).
- **D2. A silenced delayed route is active**, with coefficients `[+0.0; 4]`. Its op keeps mixing and
  its consumer keeps staging it, so its line never holds stale audio.
- **D3. Contribution.** A destination's sum is `first + sum(active later inputs)` in D9 edge order:
  - the first input in edge order always owns the store: its contribution when active, `+0.0` when
    inactive;
  - each later input is added only when active;
  - with every input active the result is exactly today's reduction, so a lone active `-0.0` input
    keeps its sign;
  - an inactive first input is "fill `+0.0`, then accumulate the active rest with
    `initial_store = false`". Calling today's reduction on the filtered input list would make the
    first *active* input the store owner, which D3 forbids;
  - **in place** (VERIFY-2 MINOR 1): when the inactive route is the destination's only input and the
    destination would read it in place, the destination fills `+0.0` instead of leaving the raw tap
    in its buffer;
  - the same rules hold for the host-master forms (`reduce_plane_into`, `reduce_many_into`).
- **D4. An inactive route's op does not run**: no reduction, no mix.
- **D5. Mechanism.**
  - The executor owns a `RouteActivity` built at bind: `active: Box<[bool]>`, one entry per prepared
    route in canonical route-ID order, plus, beside the ops, each route op's route index and each
    destination op's (input position, route index) pairs.
  - `execute_op` takes it as a new borrowed argument (`Option<&mut RouteActivity>`), read once per route
    input per block. It is `&mut` from this slice because *Ramp live send coefficients on the render
    plane*'s route op writes the bit; freezing the borrow now keeps that slice off this signature.
  - `RouteActivity` exists **only** when the plan has a route that can be inactive (a silencing gate
    here; a live route from *Ramp live send coefficients on the render plane*). Otherwise nothing is
    built, `execute_op` sees `None`, and the plan's program, op layout and resource estimate are
    byte-for-byte today's.
  - Route destinations are never bank members: add `debug_assert!` in `bank_gather_source` that a
    gathered member's single input is not a route buffer tracked by `RouteActivity`.
- **D6. Resources** (VERIFY-2 MINOR 5). When `RouteActivity` is built, the graph-compiler estimate
  charges its bytes inside `graph_metadata_bytes`, with the formula in a doc comment.
  `resource_estimate` (`crates/graph-compiler/src/estimate.rs:15-27`) receives no route gates today;
  its only caller, `compile.rs:485`, passes it the `PreparedRoute` gates it has just built. No new
  `GraphResourceEstimate` field.
- **D7. Test-only counters** under `graph`'s `test-support` feature:
  `test_only_route_mix_counts()` / `test_only_route_mix_reset()`, counting route-op mixes executed
  per route node, and a test-only accessor reporting whether a bound plan built `RouteActivity`.

## Deliverables

1. `RouteActivity`, its construction at bind (D1, D2, D5), and the `execute_op` argument.
2. The D3 reductions in `reduce_plane`/`reduce_many` and their host-master forms, and D4.
3. D6 in `crates/graph-compiler/src/estimate.rs`, and the route gates passed at `compile.rs:485`.
4. D7, re-exported from `crates/graph/src/lib.rs` beside the existing `test_only_*` functions.
5. `crates/graph/tests/MUTATIONS.md` rows for the red mutations named in the gates.
6. `docs/SESSION_SCHEMA_V1.md`, the route paragraph: a muted undelayed route contributes nothing; a
   muted delayed route contributes its zero-coefficient mix; the first input in route-ID order owns
   the store. Also DESIGN 5.4's NaN note: a zero coefficient times a non-finite `input`-tap sample is
   NaN, so a muted delayed route can carry NaN to the bus input, where D7 sanitizes it.

## Authorized paths

- `crates/graph/src/{lib.rs,runtime.rs,program.rs}`, `crates/graph/tests/`, `crates/graph/tests/MUTATIONS.md`
- `crates/graph-compiler/src/estimate.rs`, `crates/graph-compiler/src/compile.rs` (the
  `resource_estimate` call at `:485` only, to pass the route gates) and
  `crates/graph-compiler/tests/` (one new test file)
- `crates/host-core/tests/route_mute.rs` (created by *Mute a route in the session*)
- `docs/SESSION_SCHEMA_V1.md`
- this spec

## Non-goals

- No live mute (*Ramp live send coefficients on the render plane*), and no follow-mute
  (*Let a route into a submix follow its source strip's mute in the session*).
- No deferred deactivation of a delayed muted route (DESIGN O6), and no fold of a master with an
  inactive contributor (O8).
- No change to staging: a delayed input is staged every block, as today.

## Hazards

- **Signed zero.** When the lowest-ID input is inactive it still owns the store and writes `+0.0`.
  The next active input adds, so a lone `-0.0` contributor becomes `+0.0`. The oracle implements
  exactly that, never "first active stores".
- **A delayed muted route must keep running.** Skipping it would leave its consumer staging from an
  arena buffer this block did not write (another op's samples), and the line would replay pre-mute
  audio on a later unmute. Gate 3 pins it.
- **An in-place route.** When inactive, its buffer holds the raw tap. The destination must neither
  alias nor add it: D3's in-place rule is on the destination side, never "read zeros from the route
  buffer". Gate 1's sole-contributor case is that chain.
- **The op layout.** A new `RuntimeOp` field moves a reported byte on every plan. Keep D5's state
  beside the ops.

## Objective gates

Render gates live in `crates/host-core/tests/route_mute.rs` (host-core is in the
`run-aarch64-tests.sh` list, so they also run 4-lane on arm64). Every contributor is fed a distinct
non-constant signal per lane.

1. **A muted route contributes per P4.** Against an independent scalar oracle that implements D3
   (not "the session without the route", which differs in signed zero, VERIFY-1 MINOR-1):
   - bus `b` has two undelayed contributors, one muted: the bus input equals the oracle bit for bit,
     and the muted route's mix counter does not advance;
   - bus `c` has one contributor, from a `post_pan` tap that its route reads in place, and that route
     is muted: `c`'s input is exact `+0.0` on both planes.

   *Test value: it turns red if a muted undelayed route is still mixed (adding `±0` terms that move
   signed zeros) or read, or if an inactive in-place route leaves the raw tap in its destination.*
2. **The first input owns the store.**
   - The lowest-ID route into a bus is muted and the next contributor's block is all `-0.0`: the bus
     input is `+0.0` on both planes.
   - The lowest-ID route is active with an all-`-0.0` block and every later route is muted: the bus
     input is `-0.0`.

   *Test value: it turns red if the store owner moves to the first active input, or if an active
   first input's `-0.0` is lost.*
3. **A muted delayed route stays active.**
   - Contributor `d`'s route into bus `e` carries a 486-sample compensation delay (a sibling
     contributor has a true-peak limiter insert, 48 kHz; assert the delay from the compiled plan), and
     is muted.
   - `e`'s input equals the oracle, which mixes `d`'s route with `[+0.0; 4]` through the 486-sample
     delay, and `d`'s route-mix counter advances every block.
   - The PR records the mutation "a delayed muted route goes inactive" as red here.

   *Test value: it turns red if a delayed muted route is skipped, which lets its consumer stage from
   an unwritten arena buffer.*
4. **Nothing moves without a silencing route.** A graph-compiler test compiles the same session with
   no muted route and with one: with the muted route, `graph_metadata_bytes` exceeds the unmuted
   compile's by at least the bytes bind allocates for `RouteActivity`, measured with
   `bench_support::alloc`'s thread-scoped counters around bind; and a test-only accessor reports that
   the unmuted plan builds no `RouteActivity`. That the unmuted estimate equals the base commit's is
   PR evidence (it is sealed in the canonical text, which gate 6 holds).
   *Test value: it turns red if `RouteActivity` is built for a plan that cannot need it, or is
   uncharged.*
5. **Render allocates nothing.** After warm-up, `allocations == 0` and `frees == 0` on the render
   thread for gate 1's and gate 3's sessions, measured with `bench_support::alloc`.
   *Test value: it turns red if the activity table or the per-input route indices are built or
   resized on the render thread.*
6. **Unchanged where nothing is muted.**
   - `cargo build --locked --release -p audit -p bench -p capi -p session-validator`
   - `bash scripts/check-graph-determinism.sh` (`target/issue6/fresh-process-determinism.json`
     identical to its base)
   - `cargo run --locked -p graph-compiler --bin graph_fixture -- --check`
   - `bash scripts/check-console-fixtures.sh target/release/session_validator`
   - `bash scripts/check-builtins-fixtures.sh . target/release/audit`
   - `./target/release/audit capi`
   - `cargo test --locked --release -p audit -p bench -p console-workload`, and every existing
     `route_folds` count unchanged
7. **Workspace, policy and 4-lane.**
   - the test-debug-a command (DESIGN section 7)
   - `bash scripts/check-graph-policy.sh` and `bash scripts/test-graph-policy.sh`
   - `bash scripts/check-realtime-policy.sh` and `bash scripts/test-realtime-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
   - `bash scripts/run-aarch64-tests.sh debug` on an arm64 host, or CI's `aarch64-debug` job at the
     K3 push

## Evidence

- The output of every gate command above, from the PR's head commit.
- Each new test's name with its one-sentence test-value answer.
- The `MUTATIONS.md` rows with their observed red results.

## Dependencies

- *Mute a route in the session* (#1216)

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- "Bit-identical" and "unchanged" gates are hard stops. NaNs are folded (decision 10).
- Render stays allocation-, lock- and syscall-free. The activity table is plain data, read once per
  route input per block.
- One implementation shape for every target.
- A test that greps source or prose is refused. A superseded test is deleted in the same PR.
- Commit on the K3 batch branch; do not push until the root closes the batch.
- Attempt budget: five attempts, one adversarial verdict each.
