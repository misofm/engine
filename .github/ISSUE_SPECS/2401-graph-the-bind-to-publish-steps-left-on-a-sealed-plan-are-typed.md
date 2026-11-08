# Graph: the bind-to-publish steps left on a sealed plan are typed too

Successor of #2388. #2388 made bind return a `graph::GraphBoundPlan`, a builder that is not a `PreparedRenderPlan`. Its bind reads, bind attaches, bind-table take and fade table are methods of the bound plan, and its seal, or its carry builder's, is the only way to a renderable plan. Some one-shot or bind-time steps were outside that scope. They still run on the sealed `PreparedRenderPlan`, or keep a run-time refusal, between bind and publication:

- **The mono-collapse join.** `PreparedRenderPlan::arm_mono_collapse` (`crates/engine/src/realtime/plan.rs` ~1350, doc-hidden) is the structural join of the mono collapse. `prepare_or_project` runs it on the sealed plan (`crates/host-core/src/prepare.rs` ~5882), and so do the graph-compiler and builtins-compiler test fixtures. Its doc says "until a caller does this, no chain collapses", so it is a once-per-plan bind step that nothing types.
- **The source-read offset.** `set_source_read_offset` is set on the sealed plan before the inventory (`prepare.rs` ~5769).
- **The revision gate.** `PreparedRenderPlan::take_revision_gate` is an engine-level one-shot: it returns `Some` once, then `None`. It is taken in control-plane (`crates/control-plane/src/compile.rs` ~2470), in host-core (`crates/host-core/src/limiter_linked_session.rs` ~442) and in many tests.
- **The held-gate stamp.** `PreparedRenderPlan::stamp_held_gate` (`prepare.rs` ~5905) stamps a successor's carried live differences on its gate pin after the seal.
- **The source donation.** `graph::donate_sources` (`crates/host-core/src/source.rs` ~1207, through `donate_consumers`) swaps source consumers between two plans neither of which render owns. It returns `false` for pairs its own check refuses.
- **An unreachable identity arm.** `graph::plan_identity(&mut plan)` returns `Option`, so `prepare_or_project` keeps an unreachable `host.inventory.identity` arm (`prepare.rs` ~5771). The plan is the graph plan bind just built.
- **Two attach refusals.** `GraphBoundPlan::attach_jump_lengths` and `attach_automation_program` keep their run-time `AlreadyAttached` refusals for a second call.

`force_mono_collapse_off` is not in scope. It is a test-only switch, toggled between renders, not a bind-time join.

## Contract
- Each step above either moves onto the bound plan (a method of `GraphBoundPlan`, before the seal) or is typed as a one-shot by value: a call that consumes the right to make it, so a second call does not type-check. Where a step must stay on the sealed plan, for example because publication needs it there, the issue records why. The step then keeps a test of its refusal.
- `prepare_or_project` reads `plan_identity` without an `Option`, for example from the bound plan before its seal. Its `host.inventory.identity` arm is removed.
- An engine-level step (`arm_mono_collapse`, `set_source_read_offset`, `take_revision_gate`, `stamp_held_gate`) may need a bound-plan surface in `engine`, or a graph-side wrapper. The brief chooses one, with a declared file set. This crosses into `crates/engine`.
- No rendered bit, plan word or fixture byte moves. This is a refactor.

## Scope
- `crates/engine/src/realtime/plan.rs`: the four engine-level steps, if they move or become typed.
- `crates/graph`: the bound-plan surface and the two attaches.
- `crates/host-core/src/prepare.rs` and `crates/host-core/src/source.rs`: `prepare_or_project`'s post-seal span and the donation.
- `crates/control-plane/src/compile.rs` and `crates/host-core/src/limiter_linked_session.rs`: the gate take.
- Every direct test caller of the moved steps in those crates and in `graph-compiler`, `builtins-compiler`, `capi` and the tools.

Brief it with a declared file set, and split it if it can't reach a closable slice in half a day. For example, the graph-side steps (the two attaches, `plan_identity`, the mono-collapse join) could come first, and the engine-level gate steps after.

## Gates
- Mutation evidence: each run-time rule a type replaces (a second gate take, a second attach, a second mono-collapse join) becomes a compile error, shown as PR evidence, or keeps a test.
- No bit moved: rendered output and `graph_fixture` bytes are identical to base (PR evidence, with #2375's method).
- `cargo test --locked` for engine, graph, graph-compiler, builtins-compiler, host-core (with and without `control-provider`), control-plane, source, capi and rack, plus `cargo test --locked --release -p audit -p console-workload`.
- Clippy `-D warnings`, `cargo doc -D warnings`, `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `graph_fixture --check` and the realtime policy tool.

## Evidence / attempt record
(empty)
