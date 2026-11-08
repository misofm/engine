# Engine: a plan's revision gate and source-read offset are set before its seal

Successor of #2401. #2388 and #2401 made a bound graph plan a builder: bind returns a `graph::GraphBoundPlan`, its one-shot steps are typed states, and its seal is the only way to a renderable `PreparedRenderPlan`. Three engine-level steps were outside that slice. They are methods of the sealed `PreparedRenderPlan` in `crates/engine/src/realtime/plan.rs`, so only `engine` can type them:

- **The gate take.** `PreparedRenderPlan::take_revision_gate` returns the plan's one `PlanRevisionGate` handle as `Some` once, then `None`. It is taken in production by control-plane (`crates/control-plane/src/compile.rs` ~2472) and by host-core (`crates/host-core/src/limiter_linked_session.rs` ~442), and at 77 more sites. Each site does `.take_revision_gate().expect(..)`, so a second take is a panic or an `Option` arm, never a type error.
- **The held-gate stamp.** `PreparedRenderPlan::stamp_held_gate` stamps writes on the pin the plan still holds. It returns `None` once the handle is taken. `prepare_or_project` (`crates/host-core/src/prepare.rs` ~5905) calls it on the sealed plan, and maps `None` to `successor_revision_refusal()`.
- **The source-read offset.** `PreparedRenderPlan::set_source_read_offset` is set on the sealed plan before the inventory (`prepare.rs` ~5789). Its doc says it is set once, before publication. Nothing stops a later call on an unpublished plan that has already rendered. Such a plan exists on a synchronous host, or in a test.

**Narrowed at attempt 2 (Sol's attempt-1 verdict).** #2412 closes on the source-read offset. The
gate take and the held-gate stamp move to the successor #2417, which carries their design,
counts and split; the first two bullets above are its problem statement, kept here for the record.

## Contract
- `set_source_read_offset` is set before the seal: a bound-plan method, or an argument of the seal. Once sealed, a plan's offset can't change.
- `host-core`'s `test_only_set_source_read_offset` writes the offset on a sealed host. It and its three callers (`tests/timeline.rs`, `tests/source_read_clock.rs`, `tests/warm_successor.rs`, plus control-plane `compile.rs` ~2268) either get a test-support path or move to setting the offset before the seal. The brief chooses.
- No rendered bit, plan word or fixture byte moves. This is a refactor.

## Scope
- `crates/engine/src/realtime/plan.rs` and `plan_exchange.rs` (the plan's offset and the publisher's source-read clock), and the engine's own tests in `realtime/mod.rs`.
- `crates/graph`: the bound-plan surface and the executor's offset.
- `crates/host-core/src/prepare.rs`: `prepare_or_project`'s offset and the test-support path; the three host-core test files above.
- `crates/control-plane/src/compile.rs` and `control.rs`: the test hook.
- `scripts/check-web-audioworklet-callgraph.py`: the render path's pinned indirect sites.

Low priority: no defect is open.

## Gates
- Mutation evidence: an offset set after the seal is a compile error, shown as PR evidence, or keep a test.
- No bit moved: rendered output and `graph_fixture` bytes are identical to base. This is PR evidence, using #2375's method, with each row written in one `write` so concurrent test threads can't interleave rows (#2401's lesson).
- `cargo test --locked` for engine, graph, graph-compiler, builtins-compiler, host-core (with and without `control-provider`), control-plane, source, capi and rack, plus `cargo test --locked --release -p audit -p bench -p console-workload`.
- The builds without test-support features (#2388's lesson): `cargo check --locked --workspace`, and `cargo build --locked --release -p bench -p bench-support -p wasm-gates`.
- The web call-graph gate: `build-web-audioworklet.sh --named-twin`, then `check-web-audioworklet.sh --without-metadata-regeneration` (rc 0).
- Clippy `-D warnings`, `cargo doc -D warnings`, `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `scripts/test-workspace-policy.sh`, `scripts/check-rack-policy.sh`, `graph_fixture --check` and the realtime policy tool.

## Evidence / attempt record

### Attempt 1 (Terra), base `origin/main` @ `00a9b9dcf`

**Slice.** The source-read offset only. The gate take and the held-gate stamp are left, with the
reason and a design under "Left for a successor" below; at attempt 2 they move to #2417.

**Claims re-verified at base** (`00a9b9dcf`).
- `PreparedRenderPlan::take_revision_gate` (`crates/engine/src/realtime/plan.rs:1007`) returns
  `Some` once, then `None`. It is taken at `crates/control-plane/src/compile.rs:2472`,
  `crates/host-core/src/limiter_linked_session.rs:442`, builtins-compiler's lazy `test_gate`
  (`crates/builtins-compiler/src/lib.rs:6656`) and the test sites (82 textual matches in 43
  files, docs included). Holds.
- `PreparedRenderPlan::stamp_held_gate` (`plan.rs:1027`) returns `None` once the handle is
  taken; `prepare_or_project` maps that arm to `successor_revision_refusal()`
  (`crates/host-core/src/prepare.rs:5905`). Holds.
- `PreparedRenderPlan::set_source_read_offset` (`plan.rs:1079`) is public on the sealed plan
  and is called after the seal (`prepare.rs:5789`). Nothing stopped a later call on a plan that
  had rendered. Holds.
- `host_core::test_only_set_source_read_offset` (`prepare.rs:932`) writes a sealed host's plan
  and inventory. Its callers are `tests/timeline.rs:774`, `tests/source_read_clock.rs:89`,
  `tests/warm_successor.rs:562` and control-plane `compile.rs:2268`. Holds.

**Design (the offset).**
- The offset belongs to the executor, which is the only reader of it at render
  (`GraphExecutor::source_read_offset`, read at block entry and by a displaced warm
  predecessor's prime replay). The engine trait hook `PreparedPlanExecutor::set_source_read_offset(&mut self, u64)`
  becomes a getter, `PreparedPlanExecutor::source_read_offset(&self) -> u64` (default 0).
  `PreparedRenderPlan` loses its `source_read_offset` field and its setter; its
  `source_read_offset()` getter reads the executor's (0 without one). The publisher's
  source-read clock (`PlanClocks::of`, after each block) reads it through that getter: one
  virtual call returning a field, with no allocation, lock or syscall.
- Graph sets it on the bound plan: `GraphBoundPlan::set_source_read_offset(&mut self, u64)`, on
  every state, before the seal. Not one-shot, for #2401's reason for the mono-collapse join: a
  bound plan cannot render, so a later call before the seal replaces an earlier one and no block
  ran under either. Only graph can name the executor's field, so outside graph a sealed plan's
  offset cannot change at all, and inside graph only the bound plan writes it.
- `prepare_or_project` computes the offset where it reads the plan identity, just before the
  hand-over builder takes the bound plan, and sets it there. The computation reads only
  `successor` and `warm`, and is infallible, so moving it ahead of the carry install changes no
  outcome or error precedence. No hand-over section reads it: `GraphCarryBuilder::seal` only moves
  staged sections into the executor, and the executor's two readers run at render.
- Test path: `test_only_set_source_read_offset(&mut PreparedHost, u64)` is replaced by
  `host_core::test_only_set_next_source_read_offset(u64)` (test-support). It schedules the
  offset of the next preparation on this thread that reaches its seal; `prepare_or_project`
  takes it in place of the computed one and sets it on the bound plan and the inventory row
  together, as before. A projection returns before the take. So even the test path writes the
  offset before the seal, and no setter exists on a sealed plan in any build. The three host-core
  callers set it before they prepare; control-plane's `compile.rs` hands its own scheduled offset
  to it before `preparer.prepare` instead of writing the prepared host afterwards. The
  `lead_bound` caller in `warm_successor.rs` gains an assertion that the inventory carries the
  offset, which it used to write directly.

**Mutation evidence: the rule is a compile error.** The committed evidence is one
`compile_fail,E0599` doctest pair on `GraphBoundPlan::set_source_read_offset`: a sealed plan has
no setter, its plain twin sets the bound plan's (#1422 D2). Test value: red if
`PreparedRenderPlan` offers a setter again, so a plan that has rendered could have its source-read
clock moved; no other test sees a type-level change.

**Tests.** None new beyond the doctest pair, and none deleted. The engine test
`the_publisher_reads_both_clocks_of_the_active_plan_after_each_block` keeps its claim; its executor
now fixes its offset at construction, and its test value's last clause reads "a plan does not
report its executor's offset" in place of "does not forward its offset".

**Mutations of the change (head).** The tree was restored after each.

| Mutation | Run | Result |
|---|---|---|
| a `pub fn set_source_read_offset` restored on `PreparedRenderPlan` | `graph --doc` | red: the new `compile_fail` doctest |
| `PreparedRenderPlan::source_read_offset` returns 0 | `engine --lib` | red: `the_publisher_reads_both_clocks_of_the_active_plan_after_each_block` |
| `GraphBoundPlan::set_source_read_offset` writes nothing | `host-core` `timeline`, `warm_successor`, `source_read_clock` | red: `a_discontinuity_keeps_the_sources_and_the_timeline_in_step`, `a_warm_successor_waits_for_every_carried_ring_and_a_held_seek` (the production warm path) and all 3 of `source_read_clock` |
| `GraphExecutor`'s `source_read_offset` impl removed (the trait default 0) | `host-core` `source_read_clock`; control-plane lib | red: all 3 of `source_read_clock`; `a_declaration_resets_an_offset_with_no_raised_floor` |
| `prepare_or_project` takes the scheduled offset and ignores it | the three host-core files; control-plane lib | red: all 3 of `source_read_clock`, `a_discontinuity_keeps_the_sources_and_the_timeline_in_step`, `lead_bound_holds_at_the_exact_thresholds`, `a_declaration_resets_an_offset_with_no_raised_floor` |

**No bit moved.** #2375's scratch instrumentation, never committed, applied by one script
identically to an archive of the base (`00a9b9dcf`) and of the head commit:
`PreparedRenderPlan::render_block` FNV-1a folds every rendered output block into a thread-local
accumulator, whose destructor appends `<test binary> <test thread> <blocks> <hash>` to a file in
one `write` (#2401's lesson: no row interleaved on either tree). Suites, each tree in its own
target dir, with builtins-compiler, graph, effect-compiler and session test-support:
- `graph`, `graph-compiler` and `builtins-compiler`, every target;
- `source` lib;
- `host-core` lib and `successor_swap`, `warm_successor`, `withdrawn_successor`, `live_routes`,
  `browser_successor`, `swap_block_realtime`, `timeline`, `collapse_arming`, `randomized` and
  `source_read_clock` (host-core test-support and `control-provider` on). `source_read_clock` is
  added to #2401's set because it renders under nonzero offsets;
- `control-plane` lib (`test-support`), added for its warm-successor and offset tests, which
  render through the control plane's own preparation.

817 tests passed on each tree, 348 rows each. Excluded, as #2375, #2388 and #2401 did:
graph-compiler's `live_routes_render_without_allocating` and host-core's
`live_sends_render_without_allocating`, which publish from a concurrent producer thread; they
differ between base and head here, and the first differed between two reruns of the base alone
(`2de2059010254d6c`, `433f9f526fb072dc`). The other 346 rows (62 889 blocks) are identical on
base and head; the sorted listing has sha256
`42f79994865b97d9d2bc3f3adf8d4ebbec1fc8c0bc9794148cc762c9c46e8854` on both. `graph_fixture
--check` passes at head, so the fixture bytes are unchanged. The one render-path change is the
publisher's source-read clock reading the offset through the executor (one virtual call returning
a field); the realtime policy tool reports 299 marked regions in 48 files on both trees.

**Gates (head).** `F` = builtins-compiler, graph, effect-compiler, session and engine
test-support plus `engine/realtime-audit`.
- `cargo test --locked -p engine -p graph -p graph-compiler -p builtins-compiler -p source -p rack
  --all-targets` (`F`): 655 passed, 4 ignored; `--doc`: 24 passed.
- `cargo test --locked -p host-core --all-targets` (`F`, host-core test-support): 473 passed, 8
  ignored; with `host-core/control-provider`: 618 passed, 8 ignored; `--doc`: 9 passed.
- `cargo test --locked -p control-plane -p capi --all-targets` (control-plane, host-core, graph,
  session and engine test-support): 340 passed.
- `cargo test --locked --release -p audit -p bench -p console-workload`: 103 passed.
- No test-support features: `cargo check --locked --workspace` and `cargo build --locked
  --release -p bench -p bench-support -p wasm-gates`: clean.
- `cargo clippy --locked -p engine -p graph -p host-core -p control-plane --all-targets
  -- -D warnings`, with the test-support features and without them: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps` for the same four, with and without
  test-support: clean.
- `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `scripts/test-workspace-policy.sh`,
  `scripts/check-rack-policy.sh`, `graph_fixture --check` and `realtime_policy` (299 regions in
  48 files): pass.

**Left for a successor: the gate take and the held-gate stamp.** They cannot be typed one at a
time: the stamp's `None` arm exists because the take can come first, and no take on the sealed
plan can be one-shot. The handle has to leave the plan by value where the plan is built, which
touches every plan construction (about 22 `prepare` calls, 93 `.seal()` calls) besides the 79
takes. Past half a day with its gates; the design, counts and split are #2417's spec.

### Sol verdict on attempt 1 (`20b7c92cc`): FAIL, one BLOCKER; the slice's design is sound

1. BLOCKER: the web call-graph gate fails at head. `PlanClocks::of` (engine `plan_exchange.rs`,
   called from `RealtimePlanOwner::render_active`) now dispatches through the executor, which adds
   a `call_indirect`: `RealtimePlanOwner13render_active: pinned 0, found 1`, `indirect_sites=12`.
   Pin it in `scripts/check-web-audioworklet-callgraph.py` and add the gate to the gate list.
2. MINOR: move `GraphExecutor::source_read_offset` inside the `REALTIME_POLICY` region below it.
3. MINOR: take the test-only thread-local cell at the top of `prepare_or_project` when preparing,
   so an early refusal can't leave it for a later preparation; reword its doc to match.
4. MINOR: narrow the contract to the offset slice, which #2412 closes on.
5. Write the gate-take and stamp work as a stateless low-priority successor spec, with its own
   half-day split, and put a `#2417` placeholder here.

### Attempt 2 (Terra), on attempt 1 merged with `origin/main` @ `3bf66dd46`

- **1.** `INDIRECT_SITES["miso_engine_web_v1_render"]` pins
  `RealtimePlanOwner::render_active` at 1, with its reason: `PlanClocks::of` reads the active
  plan's `PreparedPlanExecutor::source_read_offset` once per rendered block, through the plan's
  `Box<dyn PreparedPlanExecutor>`, a getter returning a field with no allocation or drop, covered
  by the render-locked count. `build-web-audioworklet.sh --named-twin` then
  `check-web-audioworklet.sh --without-metadata-regeneration`: rc 0, `miso_engine_web_v1_render`
  `indirect_sites=12`. The analyser's `--self-test` passes. The gate is in the gate list.
- **2.** The getter is the first item of the `REALTIME_POLICY` region, with a doc line.
- **3.** `prepare_or_project` takes the scheduled offset as its first step when `mode` is
  `Prepare` (a projection leaves it), and uses it where it sets the offset. A preparation refused
  anywhere has already taken it. The setter's doc says so.
- **4.** Contract, scope and gates now cover the offset only; the narrowing note under the
  problem statement points at #2417.
- **5.** The successor spec is written, ready to file (design, counts at `00a9b9dcf`, a two-part
  split: engine and graph first, then host-core's test sites).
- Merge: `origin/main` merged without conflict. #2392's lane-swap pairing changed rack and the
  effect crates, none of this slice's files.
- No bit moved is not rerun: the attempt-2 changes are a script table, a moved getter, a doc line
  and the test-only cell's take point; the render path is attempt 1's.

**Gates (attempt 2 head, merged tree).**
- `cargo test --locked -p engine -p graph --all-targets` (graph and engine test-support,
  `engine/realtime-audit`): 243 passed; `--doc`: 12 passed.
- `cargo test --locked -p host-core --all-targets` (workspace test-support features,
  `control-provider`): 618 passed, 8 ignored.
- `cargo test --locked -p control-plane --all-targets --features test-support`: 102 passed.
- `cargo check --locked --workspace` (no test-support): clean.
- `cargo clippy -D warnings` and `cargo doc -D warnings` for engine, graph, host-core and
  control-plane, with and without the test-support features: clean.
- The web call-graph gate: rc 0 (above); `check-web-audioworklet-callgraph.py --self-test` passes.
- `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `scripts/test-workspace-policy.sh`,
  `scripts/check-rack-policy.sh`, `graph_fixture --check` and `realtime_policy` (312 marked
  regions in 48 files at the merged tree): pass.
