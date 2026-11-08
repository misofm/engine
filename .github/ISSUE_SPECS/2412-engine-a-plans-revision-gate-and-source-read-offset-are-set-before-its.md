# Engine: a plan's revision gate and source-read offset are set before its seal

Successor of #2401. #2388 and #2401 made a bound graph plan a builder: bind returns a `graph::GraphBoundPlan`, its one-shot steps are typed states, and its seal is the only way to a renderable `PreparedRenderPlan`. Three engine-level steps were outside that slice. They are methods of the sealed `PreparedRenderPlan` in `crates/engine/src/realtime/plan.rs`, so only `engine` can type them:

- **The gate take.** `PreparedRenderPlan::take_revision_gate` returns the plan's one `PlanRevisionGate` handle as `Some` once, then `None`. It is taken in production by control-plane (`crates/control-plane/src/compile.rs` ~2472) and by host-core (`crates/host-core/src/limiter_linked_session.rs` ~442), and at 77 more sites. Each site does `.take_revision_gate().expect(..)`, so a second take is a panic or an `Option` arm, never a type error.
- **The held-gate stamp.** `PreparedRenderPlan::stamp_held_gate` stamps writes on the pin the plan still holds. It returns `None` once the handle is taken. `prepare_or_project` (`crates/host-core/src/prepare.rs` ~5905) calls it on the sealed plan, and maps `None` to `successor_revision_refusal()`.
- **The source-read offset.** `PreparedRenderPlan::set_source_read_offset` is set on the sealed plan before the inventory (`prepare.rs` ~5789). Its doc says it is set once, before publication. Nothing stops a later call on an unpublished plan that has already rendered. Such a plan exists on a synchronous host, or in a test.

## Contract
- The gate handle is handed out by value, once, so a second take does not type-check. The likely shape is that the step that makes the plan renderable, the graph seal or an engine wrapper's, returns `(PreparedRenderPlan, PlanRevisionGate)`. `take_revision_gate`'s `Option` is then unrepresentable and is removed.
- `stamp_held_gate` runs before that hand-out, on the plan that still holds its pin. For example, it becomes a method of the bound plan or of an engine-side wrapper. Its `None` arm is removed, and so is `prepare_or_project`'s `None` mapping. Its `Some(Err)` (a refused revision) stays: it is a real refusal.
- `set_source_read_offset` is set before the seal: a bound-plan method, or an argument of the seal. Once sealed, a plan's offset can't change.
- `host-core`'s `test_only_set_source_read_offset` writes the offset on a sealed host. It and its three callers (`tests/timeline.rs`, `tests/source_read_clock.rs`, `tests/warm_successor.rs`, plus control-plane `compile.rs` ~2268) either get a test-support path or move to setting the offset before the seal. The brief chooses.
- The gate travels with the prepared host (`PreparedHost`) from `prepare_or_project` to the control-plane and `limiter_linked_session` takes.
- No rendered bit, plan word or fixture byte moves. This is a refactor.

## Scope
- `crates/engine/src/realtime/plan.rs`, plus `plan_exchange.rs` where it checks a gate's ownership (`owns_revision_gate`), and the engine's own tests in `realtime/mod.rs`.
- `crates/graph`: the bound-plan surface and the seal's return.
- `crates/builtins-compiler`: `PreparedBuiltinsGraphBound::seal` and its test-support `test_only_write`, which takes the gate lazily today.
- `crates/host-core/src/prepare.rs`: `prepare_or_project`'s post-seal span, and `PreparedHost`.
- `crates/control-plane/src/compile.rs` and `crates/host-core/src/limiter_linked_session.rs`: the production takes.
- Test sites: 79 `take_revision_gate()` calls in 41 files. 57 are in host-core, 7 in engine, 5 in graph-compiler and 4 in graph. There is one each in builtins-compiler, capi, control-plane, bench, bench-support and console-workload. Most are `prepared.plan.take_revision_gate().expect(..)` right after a prepare. One `(plan, gate)` helper per crate's test support, for example `PreparedHost::into_plan_and_gate` or a host-core test helper that prepares and splits, absorbs most of them mechanically. The brief names it.

Low priority: no defect is open. Each rule is held today by a run-time `Option` that every site unwraps.

Brief it with a declared file set. Split it if it can't reach a closable slice in half a day. For example, the gate take and the stamp could land first, and the source-read offset after.

## Gates
- Mutation evidence: a second gate take, a stamp after the hand-out, and an offset set after the seal each become a compile error, shown as PR evidence, or keep a test.
- No bit moved: rendered output and `graph_fixture` bytes are identical to base. This is PR evidence, using #2375's method, with each row written in one `write` so concurrent test threads can't interleave rows (#2401's lesson).
- `cargo test --locked` for engine, graph, graph-compiler, builtins-compiler, host-core (with and without `control-provider`), control-plane, source, capi and rack, plus `cargo test --locked --release -p audit -p bench -p console-workload`.
- The builds without test-support features (#2388's lesson): `cargo check --locked --workspace`, and `cargo build --locked --release -p bench -p bench-support -p wasm-gates`.
- Clippy `-D warnings`, `cargo doc -D warnings`, `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `graph_fixture --check` and the realtime policy tool.

## Evidence / attempt record

### Attempt 1 (Terra), base `origin/main` @ `00a9b9dcf`

**Slice.** The source-read offset only. The gate take and the held-gate stamp are left, with the
reason and a design under "Left for a successor" below. The issue stays open for them.

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

**Left for a successor: the gate take and the held-gate stamp.**
- They cannot be typed one at a time. `stamp_held_gate`'s `None` arm exists because the take can
  come first; typing the stamp means handing the gate out at a point the stamp precedes, which is
  the take's change. And the hand-out cannot sit on the sealed plan: any `&mut self` take on a
  type that persists can be called twice. The gate has to leave the plan by value where the plan
  is built.
- Design, as this attempt found it. `PreparedRenderPlan::prepare` and `prepare_with_executor`
  return the plan with its `PlanRevisionGate` (the pin leaves at construction; the plan keeps the
  shared gate for render; `gate_pin`, `take_revision_gate` and `stamp_held_gate` are removed;
  `owns_revision_gate` stays). Graph's bind keeps the handle in the `GraphBoundPlan`, and both
  seals (`GraphBoundPlan::seal`, `GraphCarryBuilder::seal`) return `(PreparedRenderPlan,
  PlanRevisionGate)`. `prepare_or_project` stamps the carry retargets on the handle it got from
  the seal (`PlanRevisionGate::stamp`, the same pin and the same open-stamp rule), so the
  `None` mapping goes and `Some(Err)` stays a refusal. `PreparedHost` gains the handle, which the
  control-plane and `limiter_linked_session` move out instead of taking. builtins-compiler's
  sealed fixture holds the handle instead of its lazy `test_gate`.
- Why not in this attempt: it touches every plan construction, not only the 79 takes. At
  `00a9b9dcf` there are about 22 `prepare`/`prepare_with_executor` calls (10 files) and 93 `.seal()`
  calls (27 files) besides the takes (43 files), with host-core's 57 takes in shapes a single
  helper does not absorb (a field moved out of a `PreparedHost` that is then moved whole, an
  `Option` arm in `stored_fader_automation.rs` and `silent_bank_skip.rs`, a destructure). That
  is past half a day with its gates and its no-bit-moved run, and the issue is low priority with
  no defect open: every site unwraps the `Option` once today.
