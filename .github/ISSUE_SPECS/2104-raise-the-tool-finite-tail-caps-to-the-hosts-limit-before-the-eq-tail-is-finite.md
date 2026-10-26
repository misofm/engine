# Raise the tool finite-tail caps to the hosts' limit before the EQ tail is finite

Filed by root from the #1374 verification (2026-10-26, MINOR-2).

## Problem
console-workload's `graph_caps` (tools/console-workload/src/lib.rs ~2483) and the audit tools cap finite tails at `maximum_finite_tail_samples: 10_000_000`. Since #1374 the delay states a finite tail of 35M to 77M samples. Today the console-workload sends fixture's output tail stays Infinite only because the parametric EQ is Infinite, and an Infinite tail wins in composition. When #1372 makes the EQ finite, that fixture is refused with `graph.tail.limit`. Hosts already use `u64::MAX` (host-core).

## Smallest slice
Raise the tool caps to `u64::MAX`, as host-core does, or derive them from the same constant. graph_fixture prepares no effects and is not exposed.

## Gates
- console-workload's tests and the audit tests pass.
- One row composes a finite delay tail (for example a delay followed by a finite-tail effect) through console-workload's caps and gets a Finite tail. This catches the cap coming back.

Order: before #1372 merges.

## Paths
tools/console-workload/src/lib.rs, the audit tool cap sites, their tests, this spec.
