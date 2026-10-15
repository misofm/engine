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

## Decision record: observation records addressed by stable key (batch 13b, lane W)

**What shipped.** A browser observation record (kinds 7 and 8) names its tap by stable key, the
same key `OBSERVATION_ENDED` uses: the strip ID, the rack (`1` inserts, `3` console), the console
slot's or insert's ID and the effect-local tap ID. It carries no index of any plan.

- **Record.** 272 bytes, little endian: `kind` u8 @0, `rack` u8 @1, `stripIdBytes` u8 @2,
  `effectIdBytes` u8 @3, `tapId` u32 @4, `windowBlocks` u32 @8, a required-zero u32 @12, `stripId`
  128 bytes @16, `effectId` 128 bytes @144, each ID zero padded (an ID is 1 to 127 bytes, so the
  last byte of a field is always zero). `COMMAND_RECORD_BYTES` 48 to 272, new constant
  `RECORD_ID_FIELD_BYTES` 128; `ABI_VERSION` did not move (prelaunch, V1). The command staging grows
  from 12,288 to 69,632 bytes, allocated at boot as before (`MAXIMUM_COMMAND_RECORDS` is 256).
- **Core.** One function in control-plane resolves the key:
  `SessionState::resolve_observation_key(&ObservationKeyRef { strip_id, rack, effect_id, tap_id })`
  returns `ResolvedObservationTap { producer, tap_index }` or an `ObservationKeyError`
  (`UnknownStrip`, `UnknownEffect`, `UnknownTap`, `ComputedTap`), writing nothing. It resolves on the
  newest committed plan: the strip in the committed model (so a strip with no such effect is an
  unknown effect, not an unknown strip), the effect among `effect_producers()` (the pending
  candidate's while a rebuild waits for its block), the tap in the descriptor's menu
  (`resolve_observation_tap`, also exported). Byte compares, no allocation, so it runs inside
  `process()`'s message handler; the C ABI's future observe edit (#1584 D1-D5) calls the same
  function. host-web only decodes the staged bytes into the borrowed key (a rack code outside 1 and
  3 is `unknownRack` there) and maps the refusal to the wire's reason (`unknownTrack`,
  `unknownEffect`, `unknownTap`, `unsupportedKind` for a computed tap), all in pass one before the
  room pass and any write, so nothing is queued and no ack precedes a drop. It then reaches its
  per-slot tables from the producer position through the new `PlanShape::producer_slots` (the
  inverse of `EffectInstance::producer`, built with it) and refuses `observationUnbound` for an
  effect with no reader. A malformed record (a nonzero reserved word, a length that fills its field,
  a byte after the end of an ID) is `malformed`. The codes keep their numbers and names. The
  selected read still resolves its IDs on the rendering plan through
  `ReadyOwnership::effect_by_id`, which is now only the read's.
- **Edits to control-plane.** `control.rs` (the function, its key and error types and
  `resolve_observation_tap`) and its re-exports in `lib.rs`; no edit to `prepare.rs`, host-core or
  capi.
- **Worklet.** `command()` takes `{ kind, rack, stripId, effectId, tapId, windowBlocks }`;
  `observe()` takes `{ stripId, rack, effectId, tapId, windowBlocks, armed }` and its binding map is
  `{ stripId, rack, effectId, tapId, windowBlocks }`, sorted by those. `frameSlot`, `trackIndex` and
  `effectIndex` leave the subscription map (a strip's gain-reduction slot is its position in the
  control list's strip order). An ID over 127 UTF-8 bytes or a non-string rejects locally. The
  worklet's `receiveCommand` is unchanged apart from the record size; `process()` is untouched.
- **SDK (thin client).** `LaneEdit` is `{ kind, rack, stripId, effectId, tapId, windowBlocks }`;
  `observationEdit()` builds it from a key; `effect(...).observe()` needs a stable ID
  (`console(slot, ...)`, `insert(id, ...)`, or an index resolved by `withSession`). The managed
  owner builds each record from the selection and sends it. Deleted: `#remap`, `OBSERVED_STRIP`, the
  strip-index re-addressing in `submitObservation`, `#settle`'s epoch wait and
  `MAXIMUM_STRUCTURE_WAITS`, `submitManagedAt`, `structureBehindError` and
  `sdk.observation.structure_behind`, `syncStructure`, `ObservationMap.structureEpoch`, the
  `stripOrder` of `LiveControlTransport`, `LiveControlEdits.update`, and the strip and effect index
  plumbing of the generated builders (`StripAddressing`, `EffectInstance`).
- **C ABI.** It does not admit observation records (no subscribe export), so there is no parity row.

**Gates and tests.**

1. A subscribe between a rebuild's commit and its adoption arms the named tap, never another:
   control-plane `a_stable_key_resolves_on_the_newest_plan_and_an_unknown_key_is_refused` (the key
   follows the effect when a committed removal moves its producer, and an unknown strip, insert,
   rack, tap or reserved tap zero is refused with its own error); host-web `structural_apply::the_catalogs_describe_the_successor_from_the_adoption_block_on` (the
   record for `track`'s compressor, which is index 1 in the successor, arms exactly it and not
   `a-track`'s) and `a_tap_command_admitted_at_sample_zero_survives_the_hand_over`; SDK
   `structure-evals.mjs` "#1677 -- an observation record names its tap by stable key" (subscribe,
   close and update in the window, headless and through the browser transport over the real core,
   on `swap_t3_multi_and_limiter` where two effects with the same tap swap indices);
   `measurement-evals.mjs` (the browser engine sends the key with a stale map, without a wait).
2. An unknown key is refused with its typed code and nothing changes: control-plane (above); host-web
   `observation_misuse_is_typed_and_all_or_nothing` (malformed forms, unknown rack, effect, strip,
   tap), the in-window refusals above, and `structure-evals.mjs` (a valid record in front of an
   unknown strip, effect, tap or rack is not armed either; a tap the replacement no longer has is
   refused against the newest plan).
3. SDK headless and browser evals (554 pass), `test-web-audioworklet.mjs`, `direct-oracle.mjs`,
   the writer evals for the record encoding (`writer-evals.mjs`), the layout checks
   (`check-abi-layout-v1.py` and its self-test, `check-parameter-metadata-v1.py`,
   `check-command-kind-vocabulary.py`, `check-command-reason-vocabulary.py`).

**Departures.** The test fake of `test-web-audioworklet.mjs` now has three pages of memory (the
command staging no longer fits one). The `observe()` map lost `frameSlot` (above).
