## Finding

Issue #169 ruled that banking costs no arena buffers, and its evidence was the builtins-less compile of `console-sixty-four-track.json`, where the banked and per-node arenas were both 193 buffers. While implementing #925 (identity-bound builtin stages lowered as aliases), the same fixture compiled the way every host compiles it (`compile_with_builtins`) measures **banked 256 against per-node 193**, identical at base `3c93469d`, so the claim never held on the host path. Derivation: with builtins, `PostInputBuiltins` is itself a builtin bank member; the chain `builtins -> EQ -> compressor -> fader -> matrix` merges per cohort and the eight spans overlap into one, so the `Input` slots the post-input ops free are held for the whole span (#169's hold rule, required because a merged chain runs a later cohort's compressor before later banks read their inputs): 64 inputs + 64 dedicated post-input + 64 dedicated EQ + 64 compressor outputs = 256. Per node, the effect ops break the chain and each post-input bank's window releases its eight input slots when it closes: 64 + 8 + 56 + 64 + 1 = 193. The difference is 63 stereo buffers, about 63 KiB at a 128-frame quantum on that fixture; memory, not copies, and independent of stem length.

On the builtins-less plan the identity post-input copy level absorbed the input retirements, which is why the two arms were equal there; #925 removes that level and pins the builtins-less arms at 192 banked and 129 per node (both below the old 193, so no plan grew).

## Smallest closable slice

Narrow the hold in `program::lower`'s colouring so a slot freed inside a merged span is reused once no later cohort of the span can still read it, without changing which chains merge or any rendered bit. Authorized paths: `crates/graph/src/program.rs` (colouring passes only), `crates/graph/src/program/tests.rs`, a focused resource/dataflow regression in `crates/graph-compiler/src/lib.rs`, and this spec. The random-graph corpus's dataflow proof must stay green with zero divergence.

### Housekeeping reconciliation, 2026-10-01 (#1136)

The successor measurement-only test `the_merged_span_hold_costs_the_input_slots` pins 256 banked / 129 per-node buffers. Its comments record that #1098 moved the EQ into inserts for the per-node arm, so the original 193 comparison is historical. Housekeeping retires this exact-count restriction because it defends no distinct dataflow or resource-admission behavior and would reject a colouring improvement. The finding and measurements remain here and in git history; independent random lowering, bank-window/dataflow and transactional cap gates remain active. This is test-reference maintenance only: no colouring optimization or achievement of this issue's product objectives is claimed. Freeze a current comparable reproduction and its behavioral regression before implementing the still-open issue.

## Non-goals

No change to cohort merging (#202), to bank eligibility, or to any kernel. No benchmark row is expected to move; the outcome is memory.

## Objective gates

1. `compile_with_builtins` on the console fixture: banked arena equals per-node arena, pinned with the corrected rationale.
2. The random-graph lowering corpus (`lowering_preserves_dataflow_and_bounds_the_arena_on_random_graphs`) green with zero dataflow divergence; `scripts/check-graph-determinism.sh` bit-identical.
3. `cargo test -p graph -p graph-compiler -p console-workload` green; `check-graph-policy.sh`, `tools/realtime-policy`.

## Dependencies

After "Lower identity-bound track stages as aliases" (#925), which records the measurement.

## Decision record (attempt 1, 2026-10-24)

- Scope note accepted with root choice (A): release keyed on render position. As built, `render_bounds` gives each op `earliest` (first op of its chainable group) and `latest` (first op of its own bank); a freed slot goes to op `p` only if every op that touched its previous value has `latest < earliest[p]`, or `latest == earliest[p]` with `p` outside the group's opening bank (a later chain slot, which reaches the arena only through the run's final scatter). This is stricter than the scope note's `exec(p) >= exec(o)`: a declined merge can start a later bank's run after a reader the predicted rule thought was earlier, so the reader bound is the reader's own bank, not its group.
- Gate 1 measured: console fixture with builtins, banked 256 -> 193, equal to the bank-unaware colouring (`the_banked_console_arena_equals_the_bank_unaware_arena`, red at 256 vs 193 on base).
- Zero divergence held in the merged arms of both random corpora and in the two window tests; `assert_no_slot_is_recycled_inside_a_bank_window` now asserts the recycler renders in a later run. Re-pinned corpus measurements: redirected lanes 1669 -> 1737, narrow-arm divergences 885 -> 1015.
- Open: `graph-compiler/tests/live_routes.rs::no_live_route_folds` goes red. The denser colouring gives the session Output the slot of `t1-bus`, and `runtime::foldable_lane` declines a fold when `producer_op.output == program.output || route_op.output == program.output` (a slot-coincidence clause of the kind #916 removed from `chains_into`). The fix is in `crates/graph/src/runtime.rs`, outside this issue's paths; it needs a root decision.
