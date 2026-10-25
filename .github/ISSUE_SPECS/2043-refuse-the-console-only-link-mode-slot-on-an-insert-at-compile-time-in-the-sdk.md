# Refuse the console-only link mode slot on an insert at compile time in the SDK

Filed by root housekeeping (backlog sprint, 2026-10-25). Source: the #1369b verifier.

## Problem
The generated SDK effect builder types `linkMode` as
`EffectLinkModeLabel<E> | EffectConsoleLinkModeLabel<E>` for both racks. `"slot"` (the console slot's
declared mode) is valid only on a console entry, but the type accepts it on an insert too; the call then
throws `MisoUsageError` at runtime. An agent gets no compile-time signal.

## Evidence (main `c1a218022`)
- `sdk/src/generated/control-builders.ts:1501-1509`: `EffectConsoleLinkModeLabel` adds `"slot"`.
- `control-builders.ts:1518`: `protected abstract readonly rack: "console" | "inserts"` (runtime only).
- `control-builders.ts:1563-1571` (signature at `:1567`): `linkMode(mode: EffectLinkModeLabel<E> | EffectConsoleLinkModeLabel<E>)`,
  which picks the row table by `this.rack` and refuses `"slot"` for an insert at runtime.
- Generator: `sdk/codegen/generate.mjs`; consumers `sdk/src/core/live-controls.ts`,
  `sdk/src/core/control-builder-support.ts`; evals `sdk/test/control-builders-evals.mjs`.

## Smallest slice
Make the rack part of the builder's type: parametrise the effect builder by rack kind
(`EffectControlBuilders<E, R extends "console" | "inserts">`, with `linkMode` taking
`R extends "console" ? EffectConsoleLinkModeLabel<E> : EffectLinkModeLabel<E>`), or emit separate console
and insert classes. Change the generator, regenerate, keep the runtime refusal as the guard for untyped
callers. No wire or runtime behaviour changes.

## Gates
- A type-level test beside the existing `sdk/test/*-types.ts` files: `insert.linkMode("slot")` is a compile error,
  `console.linkMode("slot")` and `insert.linkMode("dual_mono")` compile.
- The SDK eval and type-test steps CI runs, `npm run check:generated`,
  `python3 scripts/check-sdk-deletions.py`, `lint-checks.sh`.

## Paths
`sdk/codegen/generate.mjs`, `sdk/src/generated/control-builders.ts`, `sdk/src/core/live-controls.ts`,
`sdk/src/core/control-builder-support.ts`, `sdk/test/`, this spec.

## Decision record (attempt 1, 2026-10-25)
- One parametrised class, not two: `EffectControlBuilders<E, R extends "console" | "inserts" = union>`,
  `rack: R`, `linkMode(mode: R extends "console" ? EffectConsoleLinkModeLabel<E> : EffectLinkModeLabel<E>)`.
  The default union keeps every existing caller compiling; it distributes to today's loose signature.
- `EffectEdits<E, R = LiveControlRack>`; `console()` returns `EffectEdits<E, "console">`, `insert()`
  `EffectEdits<E, "inserts">`, `effect(rack, index, id)` keeps the union. `EffectInstance` does not grow
  an `R`: the strip context and the `EffectEdits` constructor take `EffectInstance<E> & { rack: R }`.
- The runtime refusal stays for untyped callers; no wire or runtime change.
- Evidence: the `@ts-expect-error` row in `sdk/test/live-controls-types.ts` goes red (TS2578, unused
  directive) when the generator emits the union `linkMode` signature again.
