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
### Ruling (root, under the owner's delegation of 2026-11-09)
Recorded in `docs/rulings/bank-contract-defect-counter-2026-11-09.md` (authority kind: owner-delegated answer). Branch `codex/issue-2425-defect-counter-ruling`; no implementation in it.

- **Q1: yes, on the wire, as one ID.** One new frozen `protocol::CounterId`, `BankContractDefects = 23`, counts over every plan the session adopted the defects the engine contained: every recovery `graph::lane_swap_defect_count` counts and every refused restore `graph::carry_refusal_count` counts. 0 in a correct build.
- **Q2.** One combined ID: a host does the same two things with either total (report a field defect; reconcile a restart that `StripsRestarted` did not list), so two IDs are granularity nobody acts on. Accumulating, as every served counter is; a per-plan value is not a non-resetting counter and a host that misses the plan's lifetime misses the defect. Read off the healthy render path: the ruling prefers a mirror stored only on the defect path (the `live_block_errors` #1521 D3 pattern), current by the first control call after the defect's block; a read at reclaim in `synchronize_plan_epochs` is the fallback, with its lag recorded.
- **Q3.** The two graph functions stay as the per-plan readers for tests, the #1286 cost record and #2424. #2424 does not make the counter unnecessary: a rebuild repairs the residue but does not tell the host its audio took a defect, and no rebuild follows a refused carry at all. #2424 adds no `CounterId`; it rebuilds on the lane-swap total only and builds its render-side flag on the defect path so the counter's mirror can share it.
- **Why the precedent is read this way.** The "deliberately off the wire" status of `carry_refusal_count` traces to #1279's review NIT 4 (nothing in production reads it; left to root) and root's 2026-10-12 ruling that surfaced it in #1286's descriptive bench record. That was a scoping call, not a principle: `TransitionReprepareRefusals` (17) and `LiveBlockErrors` (19) are wire counters that stay 0 in a correct build, because D15-17 makes a fallback decided after submit visible to the host. A bank-contract recovery is such a fallback.
- **Rejected:** no counter (defers #1279 NIT 4 a third time and leaves the carry total with no reader); two IDs; a fold into `StripsRestarted` (frozen meaning, hides the defect); a watermark outcome flag (a packer episode belongs to no revision, and a flag carries no count); a per-plan value; counting #2424's rebuilds instead (the remedy, not the host's fact).
- **Reopening:** a product need for a host to tell the two totals apart; a reopening allocates a new ID and never changes 23's meaning.

The ready-to-file implementation spec is with root (title: "Counters: `BankContractDefects` (23) puts the lane-swap defect and carry-refusal totals on the wire"); its gates are the ones this spec's "Gates" lists for an implementation, plus the no-render-cost evidence. This issue closes on the ruling record; the implementation is the successor's.
