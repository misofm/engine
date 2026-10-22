# Test the PDC input-arrival order and the estimate's fan-in count

Found by the #1384 verifier (2026-10-22). Both gaps are older than #1384: the same mutants survive on main 82053706c.

## Problem

Two mutants of the graph compiler survive every test of graph-compiler, graph, host-core and control-plane:

- **M4.** Sorting `input_arrivals` in reverse (`crates/graph-compiler/src/pdc.rs`, the `sort_by` near `:173`). `floored_input_arrival` (`crates/graph/src/lib.rs` near `:1759`) looks the list up by binary search, so a wrong order silently gives a fader arrival of 0: wrong plugin-delay compensation with no diagnostic.
- **M6.** Counting estimate inputs by edge source instead of destination (`crates/graph-compiler/src/estimate.rs` near `:54`). `maximum_inputs` and the reductions are then wrong on any graph with fan-in.

## Smallest closable slice

Two assertions in the crate that owns the logic (`crates/graph-compiler`), each in an existing test where one fits:

1. An exact `input_arrivals` order assertion on a node with at least three inputs whose sources sort differently from their insertion order, plus one floored arrival read back through `floored_input_arrival`.
2. A `maximum_inputs`/reductions assertion on a graph with a fan-in of at least three into one node and fan-out from another.

## Authorized paths

- `crates/graph-compiler/src/lib.rs` tests (or the existing graph-compiler test file that holds PDC and estimate tests)
- this spec

## Objective gates

1. M4 and M6, applied by hand, each turn the new assertion red; the PR records both.
2. `cargo test --locked -p graph-compiler -p graph --features graph/test-support`, `cargo fmt --all --check` and clippy on the package pass.

## Test value

Assertion 1 turns red when PDC input arrivals are out of order (a silent compensation error); assertion 2 when the estimate counts inputs per source. No test catches either today.

## Dependencies

#1384 (the dense-index refactor) merges first; the assertions run on its code.

## Evidence (attempt 1, 2026-10-22)

- Assertion 1 is a new unit test, `timing_lists_input_arrivals_in_node_id_order` (`crates/graph-compiler/src/lib.rs`): no existing timing test has two nodes with a nonzero arrival. The chain `c -> b -> a` schedules opposite to ID order, and all three feed the output (fan-in 3). It asserts the exact rows and `floored_input_arrival(a) == 5`.
- Assertion 2 is added to the existing `a_three_input_bus_keeps_one_reduction_on_its_input_stage` (`crates/graph-compiler/tests/compile_shapes.rs`): `semantic_estimate.audio_buffer_samples % (2 * quantum) == 3`, the bus input's fan-in, which is `maximum_inputs`. Under M6 its `reductions == 1` stays green: the track's `Input` stage fans out to two edges and takes the bus's place in the count. That is why M6 survived.
- M4 (`Reverse(entry.0)` in the `input_arrivals` sort): the gate (65 537-track test skipped) has one red test, the new one; the rows come out reversed. For the reversed rows, a standalone copy of the binary search gives 0 for `a`.
- M6 (`ends.source()` in the estimate's input count): one red test, the new assertion (2, expected 3). Nothing else turns red.
- Both mutants are restored; `git diff` is empty.
