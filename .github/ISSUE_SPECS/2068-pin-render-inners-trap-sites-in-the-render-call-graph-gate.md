# Pin render_inner's trap sites in the render call-graph gate

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: the verification of #2041
(bound the engine SPSC cursor so render inlines no slot bounds check).

## Problem
The render call-graph gate admits `PreparedRenderPlan::render_inner` as a trap owner by name only,
with no count of its `unreachable` sites. The allow-list reason is one checked slice index
(`PlanarBufferMut::plane_mut`) in a dead branch. If a later change inlines a new checked index into
`render_inner` (for example an SPSC slot index like the one #2041 removed), the gate stays green.

## Evidence
- `scripts/check-web-audioworklet-callgraph.py:170-175`: the comment and
  `TRAP_ALLOW_LIST = frozenset({"18PreparedRenderPlan12render_inner"})`.
- `scripts/check-web-audioworklet-callgraph.py:545-551`: an owner given as `name=count` is pinned to
  its site count; `TRAP_ALLOW_LIST` entries join `allowed_owners` with no count.

## Smallest slice
Measure `render_inner`'s `unreachable` site count on the shipped module and pin it at that count,
through the same `name=count` mechanism the trap-owner arguments use, with the reason in the comment.
A changed count fails with the owner name, the found count and the pinned count.

## Gates
- `scripts/check-web-audioworklet.sh` passes at the measured count.
- PR evidence: one added checked index in `render_inner` (a mutant, restored) turns the gate red.
- `lint-checks.sh --self-tests` passes (add a self-test row only if the script's self-tests cover
  the allow list).

## Paths
`scripts/check-web-audioworklet-callgraph.py`; this spec.
