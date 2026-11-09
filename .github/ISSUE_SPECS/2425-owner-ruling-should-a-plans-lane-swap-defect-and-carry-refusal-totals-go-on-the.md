# Owner ruling: should a plan's lane-swap defect and carry-refusal totals go on the wire?

Successor of #2419. This issue asks for an owner ruling; it carries no implementation until the ruling is recorded.

## Problem
Two render-time defect totals are in-process reads only.
- **`graph::lane_swap_defect_count(plan)`** (#2414, #2419): lane-swap recoveries.
  - It counts each stranded stage swap, and each slot that refused a swap-back in a chain's swap or a row's trade-back.
  - Each recovery reset two lanes and pinned both strips.
- **`graph::carry_refusal_count(plan)`** (#1279 D6): console-effect restores that a plan swap's carry refused.
  - Each one left the lane as prepared.
  - This total was deliberately kept off the wire.

A nonzero value of either total means a bank broke its contract. Today a host sees it only by holding the plan and calling the function in process. Two hosts cannot do that:
- the browser host's Worker, which hands the render half to the AudioWorklet;
- a C-ABI app, which reads counters through `protocol::CounterId`.

The frozen counter registry (`protocol::CounterId`, `crates/protocol/src/message_wire.rs`) holds the non-resetting counters a host reads over the protocol and C ABI. A new ID is a protocol and C-ABI surface: frozen once shipped, never reallocated (#1629), with an exported wire identity.

## Question for the owner
1. Should either or both totals be registered `CounterId`s, so every host reads them the same way?
2. If so:
   - one ID per total, or one combined "bank contract defect" ID;
   - whether the value is per plan or accumulates across plan replacements, as the other non-resetting counters do;
   - who reads it on the render side (the plan owner after each block, or at retirement) without adding render cost.
3. If not, which in-process reader is the supported one, and whether the lane-swap rebuild successor (#SUCCESSOR-REBUILD) makes a wire counter unnecessary.

## Contract (after the ruling)
- The ruling is recorded in `docs/rulings/` and cited here.
- If counters are added:
  - each has a new frozen `CounterId`, documented with its unit and reset semantics;
  - it is wired through host-core, the C ABI and the browser host, with one parity row per host behaviour;
  - render's cost is unchanged: the value is read off the plan, not maintained per block.
- If no counter is added, the in-process functions' docs say so and point at the ruling.

## Non-goals
- Changing what either total counts.
- The rebuild on a nonzero total (#SUCCESSOR-REBUILD).

## Gates
- The ruling document.
- For an implementation:
  - the protocol's frozen-ID tests extended;
  - a host-core test that strands a lane swap (as #2419's tests do) and reads the counter through the C ABI and the protocol;
  - gates for every touched crate, including `capi` and the web host tests.

## Evidence / attempt record
(empty)
