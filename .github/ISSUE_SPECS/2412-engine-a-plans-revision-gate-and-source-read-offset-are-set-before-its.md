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
(empty)
