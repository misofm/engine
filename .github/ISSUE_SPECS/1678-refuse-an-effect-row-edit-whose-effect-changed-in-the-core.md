# Refuse an effect-row edit whose effect changed, in the core

Successor of the batch 12b review (m4), decision 17. Code anchors verified on the batch 12b branch `codex/controls-batch-12b`. Issue number: assigned at filing (written here as #1678). Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch review.

## Product outcome

A `SET_CONTROL` edit on an effect section's row names the native effect it was written for, and
the core refuses it with a typed code when that slot or insert now runs another effect. Every
client (SDK, C ABI app) gets the same protection, and the SDK stops probing the control list
before each named-row edit.

## Context

- #1562b-K resolves named effect rows from the control list's section headers and checks each
  effect-row edit just before encoding (`effectRowResolver`, `sdk/src/core/live-controls.ts:122`),
  at the cost of a list probe per transaction; C ABI clients have no such check, so after a rebuild
  a numeric effect row can land on another effect's parameter.

## Decisions frozen for this slice

- **D1.** An effect-section `SET_CONTROL` address may carry the expected native effect ID; the
  core refuses a mismatch with a typed code before commit, nothing changes.
- **D2.** The SDK sends the expected ID from its builders and deletes the resolver hook.

## Objective gates

1. After a rebuild that changes an insert's effect, an edit written for the old effect is refused
   with the typed code on both hosts (parity row); the same edit without the expected ID keeps
   today's behaviour.
2. Wire re-pins with the repository tooling; SDK evals; protocol, control-plane, capi, host-web
   tests.

## Dependencies

- #1562b-K.

## Size

Half a day to a day.
