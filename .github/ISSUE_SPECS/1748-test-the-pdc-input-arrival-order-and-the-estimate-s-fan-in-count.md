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
