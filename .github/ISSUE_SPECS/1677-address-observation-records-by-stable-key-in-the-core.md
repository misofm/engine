# Address observation records by stable key in the core

Successor of the batch 12b review (M1), decision 17. Code anchors verified on the batch 12b branch `codex/controls-batch-12b`. Issue number: assigned at filing (written here as #1677). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

An observation subscribe or unsubscribe names its tap by stable key (strip, section, slot or
insert ID, tap ID), the same key `OBSERVATION_ENDED` uses, and the core resolves it on the plan it
admits against. No client keeps an index space, so a structural edit can never make a record arm
another effect's tap.

## Context

- Today a browser observation record (kinds 7 and 8) carries a strip index and an effect index.
  The SDK takes the strip index from the newest control list and the effect index from
  `observationMap()`, and re-addresses records after a rebuild (`#remap`, `OBSERVED_STRIP` in
  `sdk/src/core/observation-subscriptions.ts`); the batch 12b fold makes it wait when the map's
  structure epoch differs from the list's.
- The core admits a record against the newest plan (`admit_on_newest_plan`).
- #1589 (keep only handles and callbacks in the SDK) is the related SDK slice.

## Decisions to take (the implementer proposes, the coordinator rules)

- The record shape: a stable key in place of the two indices, on both hosts, with the parity row.
- The core resolves the key on the plan it admits against and refuses an unknown key with a typed
  code before any ack.
- The SDK's re-addressing and its epoch wait are deleted.

## Objective gates

1. A subscribe sent between a rebuild's commit and its adoption arms the named tap, never another.
2. An unknown key is refused with the typed code and nothing changes.
3. SDK headless and browser evals; host-web and control-plane tests; parity row.

## Dependencies

- #1584 D6a/D6b; #1589.

## Size

One day.
