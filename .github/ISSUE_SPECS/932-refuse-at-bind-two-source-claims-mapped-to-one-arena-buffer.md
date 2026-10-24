## Finding

Found while implementing and reviewing #927. When a reader-less source claim is scheduled before a live claim, the colouring frees the reader-less claim's slot one op later and the live claim's Input takes it (`crates/graph/src/program.rs:836-846`), so two claims map to one physical buffer. The test-only copy oracle (the pre-#918 copy loop, which hoists every input write to the top of the block in claim order) then writes both claims into one slot and the later copy wins, while the in-place arm (#918 bank gather, #927 Output reader) lends the live claim's played planes and is correct. Reproduced on a hand-built plan whose level-0 order departs from node-ID order (the `DeadClaim` fixture with the dead claim moved to the front). Not reachable from a compiled Session V1: `GraphPreparedSourceSet::is_valid` requires claims sorted by `GraphNodeId`, the scheduler pops level 0 in node-ID order, and `GraphNodeId`'s `Ord` puts every Input before every input-less submix, so claim order equals schedule order and a reader-less earlier claim is also the earlier copy. On `main` the only production driver lends, so production always takes the correct arm. #927's clause (e) declines the Output reader whenever an earlier op names the slot; #918's bank clause (b) has no such clause and keeps the hand-built exposure (copy arm wrong, in-place arm right).

## Smallest closable slice

Refuse at bind any plan whose `source_input_buffers` contains a duplicate buffer (an exact, cheap check), with a typed layout error and one test that builds the hand-built shape and asserts the refusal; document in `GraphPreparedSourceSet` that two claims never share a slot. Prefer the refusal over reserving every claim's slot for the block. Authorized paths: `crates/graph/src/lib.rs` (the bind check), `crates/graph/src/runtime.rs` if the check belongs beside `source_plane_table`, their tests, and this spec.

## Non-goals

No change to the colouring, to #918's or #927's eligibility clauses, or to any rendered bit of a compiled plan.

## Objective gates

1. New test: the hand-built two-claims-one-slot plan is refused at bind; every compiled console workload and every graph fixture still binds.
2. `cargo test -p graph` (both configurations), `-p console-workload`, `-p graph-compiler`; `scripts/check-graph-policy.sh`, `tools/realtime-policy`.

Note for the successor: on a compiled builtins-less plan where an unrouted track immediately precedes a routed one in track order, the routed track's Input takes the dead slot and #927's clause (e) declines it (one copy per block); correct, but a lost in-place read worth one line in the evidence.

## Decision and evidence record

- Root decision (2026-10-24, scope note accepted): the check sits in `PreparedGraphPlan::bind_optional_source_set` right after `self.lowered()`, maps every claim to `program.node_buffer[..]` (the expression `GraphExecutor::new` uses), sorts, and refuses an adjacent duplicate with the new code `graph.scheduler.source_slot`. Control plane only, O(claims log claims). The colouring and the #918/#927 clauses are unchanged; `runtime.rs` changed in its `source_plane_table` doc only.
- Reachability: `has_valid_structural_layout` requires sorted levels and strictly later edge levels, not dependency depth, so the hand-built form is a reader-less Input `b` alone in level 0 and a live Input `a` in level 1 (the spec's "dead claim at the front" form cannot exist in one sorted level).
- Test `bind_refuses_two_source_claims_mapped_to_one_buffer`: the two-level shape is refused with the new code and returns both claims; with both Inputs in level 0 the same plan binds. Red on the reverted check (check forced to `false`): the test panics "two claims on one buffer accepted".
- Residual (not in this slice): a hand-built plan can put a value no claim owns (for example an unclaimed, processor-bound Input `b` with no reader) before a claimed Input `a`; `a` then takes that slot (probe: both map to buffer 0) and bind accepts it, so the block-top copy is still exposed there. No compiled Session V1 plan has this shape. The `source_plane_table` doc states it.
- Successor note (from the spec, now superseded by the refusal): a compiled builtins-less plan where an unrouted track immediately precedes a routed one in track order maps both claims to one slot. Before this change #927's clause (e) declined the in-place read (one copy per block, correct because the dead claim copies first). With this change bind refuses that plan with a source set. It is not a production path: source sets bind through `builtins-compiler`'s `into_bound_with_source_set`, and with builtins every Input has a reader. All graph, graph-compiler, builtins-compiler, console-workload and source tests still bind.
