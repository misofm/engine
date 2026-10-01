## Finding

Found while implementing and reviewing #927. When a reader-less source claim is scheduled before a live claim, the colouring frees the reader-less claim's slot one op later and the live claim's Input takes it (`crates/graph/src/program.rs:836-846`), so two claims map to one physical buffer. The test-only copy oracle (the pre-#918 copy loop, which hoists every input write to the top of the block in claim order) then writes both claims into one slot and the later copy wins, while the in-place arm (#918 bank gather, #927 Output reader) lends the live claim's played planes and is correct. Reproduced on a hand-built plan whose level-0 order departs from node-ID order (the `DeadClaim` fixture with the dead claim moved to the front). Not reachable from a compiled Session V1: `GraphPreparedSourceSet::is_valid` requires claims sorted by `GraphNodeId`, the scheduler pops level 0 in node-ID order, and `GraphNodeId`'s `Ord` puts every Input before every input-less submix, so claim order equals schedule order and a reader-less earlier claim is also the earlier copy. On `main` the only production driver lends, so production always takes the correct arm. #927's clause (e) declines the Output reader whenever an earlier op names the slot; #918's bank clause (b) has no such clause and keeps the hand-built exposure (copy arm wrong, in-place arm right).

## Smallest closable slice

Refuse at bind any plan whose `source_input_buffers` contains a duplicate buffer (an exact, cheap check), with a typed layout error and one test that builds the hand-built shape and asserts the refusal; document in `GraphPreparedSourceSet` that two claims never share a slot. Prefer the refusal over reserving every claim's slot for the block. Authorized paths: `crates/graph/src/lib.rs` (the bind check), `crates/graph/src/runtime.rs` if the check belongs beside `source_plane_table`, their tests, and this spec.

## Non-goals

No change to the colouring, to #918's or #927's eligibility clauses, or to any rendered bit of a compiled plan.

## Objective gates

1. New test: the hand-built two-claims-one-slot plan is refused at bind; every compiled console workload and every graph fixture still binds.
2. `cargo test -p graph` (both configurations), `-p console-workload`, `-p graph-compiler`; `scripts/check-graph-policy.sh`, `check-realtime-policy.sh`.

Note for the successor: on a compiled builtins-less plan where an unrouted track immediately precedes a routed one in track order, the routed track's Input takes the dead slot and #927's clause (e) declines it (one copy per block); correct, but a lost in-place read worth one line in the evidence.
