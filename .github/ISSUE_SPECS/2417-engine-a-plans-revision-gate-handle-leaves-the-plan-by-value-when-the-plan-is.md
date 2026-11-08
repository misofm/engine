# Engine: a plan's revision-gate handle leaves the plan by value when the plan is built

Successor of #2412. #2412 typed the source-read offset: it is set on the bound plan, and a sealed `PreparedRenderPlan` has no setter. Two engine-level steps of the same family stay as run-time rules on the sealed plan, in `crates/engine/src/realtime/plan.rs`:

- **The gate take.** `PreparedRenderPlan::take_revision_gate(&mut self) -> Option<PlanRevisionGate>` returns the plan's one control handle once, then `None`. Production takes it in control-plane (`crates/control-plane/src/compile.rs`, after `preparer.prepare`) and host-core (`crates/host-core/src/limiter_linked_session.rs`, `Console` construction). About 77 test sites take it too. Each does `.take_revision_gate().expect(..)`, so a second take is a panic or an `Option` arm, never a type error.
- **The held-gate stamp.** `PreparedRenderPlan::stamp_held_gate` stamps a write on the pin the plan still holds. It returns `None` once the handle is taken. `prepare_or_project` (`crates/host-core/src/prepare.rs`, the carry retargets, preparation's last fallible step) calls it on the sealed plan and maps `None` to `successor_revision_refusal()`.

They cannot be typed one at a time. The stamp's `None` arm exists because the take can come first. And no take on the sealed plan can be one-shot: a `&mut self` method on a type that persists can always be called again. So the handle has to leave the plan by value, where the plan is built.

Low priority: no defect is open. Today every site unwraps the `Option` once.

## Contract
- `PreparedRenderPlan::prepare` and `prepare_with_executor` return the plan together with its `PlanRevisionGate`, for example `(PreparedRenderPlan, PlanRevisionGate)`. The pin leaves at construction. The plan keeps the shared `Arc<RevisionGate>` for render's snapshot. The `gate_pin` field and `take_revision_gate` are removed. `owns_revision_gate` (`plan_exchange.rs`) stays.
- Graph's bind keeps the handle in `GraphBoundPlan`, and both seals return it: `GraphBoundPlan::seal` and `GraphCarryBuilder::seal` give `(PreparedRenderPlan, PlanRevisionGate)`. A second take does not type-check, because there is no take.
- `stamp_held_gate` is removed. `prepare_or_project` stamps the carry retargets with `PlanRevisionGate::stamp`, on the handle the seal returned. It is the same pin with the same open-stamp rule: the commit stamps the open revision again and publishes it. The `None` mapping goes. A refused revision (`Err`) stays a real refusal (`successor_revision_refusal()`).
- `PreparedHost` carries the handle from `prepare_or_project` to its two production consumers, control-plane's `compile.rs` and `limiter_linked_session`, which move it out instead of taking it.
- builtins-compiler's sealed fixture (`PreparedBuiltinsGraphSealed`) holds the handle instead of its lazy `test_gate`, so `test_only_write` stops taking it on first use.
- No rendered bit, plan word or fixture byte moves. This is a refactor.

## Scope and counts (at `00a9b9dcf`)
- About 22 `PreparedRenderPlan::prepare`/`prepare_with_executor` calls in 10 files: engine `realtime/mod.rs` (12), `plan.rs`, `plan_exchange.rs`, `tests/host_attachment.rs`, graph's bind, control-plane `plan.rs`, source lib, `bench-support/tests/plan_exchange_ceiling.rs`, audit `realtime.rs`, host-core `tests/host_attachment_render.rs`.
- 93 `.seal()` calls in 27 files. Most are in graph's lib (20) and tests, graph-compiler's lib (14) and tests, source lib (12) and builtins-compiler (8). Some are builtins-compiler's or graph-compiler's own seals, which return a struct with a `plan` field, so adding a `gate` field does not touch their callers.
- 79 `take_revision_gate()` calls in 41 files (82 matches in 43 files, docs included). 57 are in host-core. A single `(plan, gate)` helper does not absorb all of host-core's sites, because they come in four shapes:
  - a gate moved out of a `PreparedHost` that is then moved whole (`Self { prepared, gate }`);
  - an `Option` arm (`stored_fader_automation.rs`, graph-compiler `silent_bank_skip.rs`, `live_routes.rs`);
  - a destructure of `PreparedHost` (`let PreparedHost { plan, sources, inventory, .. } = host`), which gains `gate`;
  - `.take_revision_gate().expect(..).publish(revision)` on a successor.
  The brief names the helper or helpers, for example `PreparedHost::into_parts`, or a `gate` field moved out before the host is.

## Split (each part about half a day)
1. **Engine and graph.** `prepare` returns the handle, and bind and both seals carry it. Fix the engine, graph, graph-compiler, builtins-compiler, source, audit, bench, bench-support and console-workload sites. host-core's `prepare_or_project` takes the seal's handle and stamps on it, and `PreparedHost` gains the field. In this part, add a doc-hidden test-support accessor that host-core's existing tests call in place of `take_revision_gate`. This part closes the type rule for the take and the stamp in production code.
2. **host-core's test sites.** Move the 57 sites to the field or helper the brief names, and remove the accessor from part 1. If part 1 fits with every site moved, there is no part 2.

## Gates
- Mutation evidence: a second take, a stamp after the hand-out and a stamp on a sealed plan are each a compile error. Show it as PR evidence, or commit one `compile_fail` doctest pair (#1422 D2) on the seal's return.
- No bit moved: rendered output and `graph_fixture` bytes match the base, using #2375's method with one `write` per row (#2401's lesson). Run it on #2412's suite set: graph, graph-compiler and builtins-compiler on every target; source lib; host-core lib plus `successor_swap`, `warm_successor`, `withdrawn_successor`, `live_routes`, `browser_successor`, `swap_block_realtime`, `timeline`, `collapse_arming`, `randomized` and `source_read_clock` (with `control-provider`); control-plane lib.
- `cargo test --locked` for engine, graph, graph-compiler, builtins-compiler, host-core (with and without `control-provider`), control-plane, source, capi and rack, plus `cargo test --locked --release -p audit -p bench -p console-workload`.
- Builds without test-support features: `cargo check --locked --workspace` and `cargo build --locked --release -p bench -p bench-support -p wasm-gates`.
- The web call-graph gate: `build-web-audioworklet.sh --named-twin`, then `check-web-audioworklet.sh --without-metadata-regeneration`. The take and the stamp are control-side, so no render `call_indirect` should move.
- Clippy `-D warnings`, `cargo doc -D warnings`, `cargo fmt --check`, `scripts/check-workspace-policy.sh`, `scripts/test-workspace-policy.sh`, `scripts/check-rack-policy.sh` (rack source never spells the graph path, comments included), `graph_fixture --check` and the realtime policy tool.

## Evidence / attempt record
(empty)
