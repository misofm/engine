# Graph: split plan building from rendering so the carry install sequence is a typestate

Successor of #2302 (GR-6), filed when the implicit-state-machine tracker #2258 closed. #2302 closed only GR-6's re-install gaps: a second fade table or carry program on one plan is now refused. The type split GR-6 asked for is not done.

## Problem
`GraphExecutor` holds both the build-time carry fields (bind, take, program, sections, fade arms, seal) and the render state. The install order is enforced by run-time refusals (`No*CarryProgram`, and #2302's re-install refusals) spread over about sixteen install and take functions, and by `prepare_or_project`'s interleaved install sequence in `crates/host-core/src/prepare.rs`. A render-ready plan with a half-installed carry is representable, and only the refusals keep it from rendering.

## Contract
- A `GraphCarryBuilder`, or an equivalent typestate, owns the install sequence. Only its sealed output can become a renderable executor, so a half-installed carry does not type-check.
- Re-installing a section with the same program keeps its documented replace semantics.
- No rendered bit, plan word or fixture byte moves. This is a refactor.

## Scope warning
This touches `GraphExecutor`'s carry fields, every install and take function, `prepare_or_project`, and every direct test caller in four crates. It must be briefed with a declared file set, and split if it cannot reach a closable slice in half a day, for example the builder first and the callers moved after.

## Gates
- Mutation evidence: each install-order refusal the typestate replaces either becomes a compile error, shown as PR evidence, or keeps a test.
- No bit moved: rendered output and `graph_fixture` bytes are identical to base (PR evidence).
- Workspace tests are green for graph, graph-compiler, host-core and rack.

## Evidence / attempt record
(empty)
