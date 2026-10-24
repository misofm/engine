# Hold doctests to the realtime-policy tool's claims (#1527 rule iv)

Split from #1527 (root decision, 2026-10-24): rule (iv), doctests.

## Problem

#1527's rule (iv) asks the realtime-policy tool to treat Rust doctests the same way as the other build components it reads, so that a doctest cannot hide code from the realtime claims. Rules (i)-(iii), D1-D2 and D4-D6 stay in #1527 and live in `scripts/check-workspace-policy.sh`'s Python `tomllib` pass. Rule (iv) belongs in the Rust tool (`tools/realtime-policy`), because only the tool parses Rust sources. The full text of rule (iv) is in #1527's spec.

## Smallest closable slice

Implement rule (iv) in the tool as #1527's spec states it, with one table-driven test row per form. Sequence it after #1528, because both add `report.rs` classes.

## Gates

1. The tool's real-tree run is unchanged, or each new finding is a real defect that is fixed in the same PR.
2. Each new row is refused by the tool and accepted by the parent commit's tool (PR evidence).

