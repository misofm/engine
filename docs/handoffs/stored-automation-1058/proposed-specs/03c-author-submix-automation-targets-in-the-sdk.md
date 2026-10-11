# Author submix automation targets in the SDK and enginectl

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A3, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

An author can automate a submix strip (a bus fader ride, a bus EQ sweep) through the SDK builder
and through `enginectl`, as the engine already accepts. This closes the known gap of *Submix strips
and live aux sends* (#1196): today only hand-written JSON can name a submix target.

## Context

- **The engine accepts it.** An automation `entity_id` may name a track or a submix (#1199 D6,
  `crates/session/src/validate.rs:910-921`), and a `console` target addresses that submix's entry
  for the slot (#1202).
- **The SDK builder resolves tracks only.** `#validateAutomation`
  (`sdk/src/core/session.ts:969-985`) and `normalizeAutomation` (`:1479-1537`) look up
  `target.trackId` among tracks; `resolveAutomationTarget` (`:1405-1477`) takes a `TrackEntry`.
  The target type has `trackId` only (`sdk/src/core/types.ts:337-344`).
- **`enginectl`** lives in the SDK (`sdk/src/cli/session-request.ts`); its `automation` reader
  accepts the keys `trackId`, `rack`, `slotId`, `parameter`, `channel` (`:471-500`, key list at
  `:475`). There is no `enginectl` under `tools/`.
- **The known gap** (`.github/ISSUE_SPECS/1196-submix-strips-and-live-aux-sends.md:227-233`); the
  skill tells authors to write JSON instead (`.claude/skills/author-session/SKILL.md:108-112`).

## Decisions frozen for this slice

- **D1. The target type.** `AutomationTarget` takes exactly one of `trackId` and `submixId`; both
  or neither is a `MisoUsageError` at `.target`. An unknown `submixId` is refused with
  `reference.missing_entity` at `.target.submixId`.
- **D2. Resolution.** A submix target resolves against the builder's submixes for every rack: its
  inserts, its console entry for the slot, and its builtins. The model writes
  `entity_id: <submixId>`. Every rule the builder applies to a track target applies to a submix
  target through the same resolution path (draft 03b D3).
- **D3. `enginectl`.** The reader accepts `submixId` as the alternative to `trackId`, with the same
  exactly-one rule.
- **D4. Docs.** The skill's sentence at `SKILL.md:110-112` and `sdk/README.md`'s automation
  sentence say a submix target is authored like a track target. Root updates #1196's record.
- **D5. The acked-batch question** does not arise: the builder commits nothing.

## Deliverables

1. D1-D3 in `sdk/src/core/types.ts`, `sdk/src/core/session.ts`, `sdk/src/cli/session-request.ts`.
2. D4.
3. The tests below.

## Authorized paths

- `sdk/src/core/types.ts`, `sdk/src/core/session.ts`, `sdk/src/cli/session-request.ts`
- `sdk/test/builder-evals.mjs`, `sdk/test/enginectl-cli.mjs`
- `sdk/README.md` (the automation sentence only), `.claude/skills/author-session/SKILL.md` (the
  automation paragraph only)

## Non-goals

- Any engine change: the engine already accepts submix targets.
- VCA automation (out of scope, `docs/rulings/submix-strips-sends-and-vca-2026-10-02.md:129`) and
  send automation (owner question OQ2).

## Hazards

- **Shared file.** Draft 03b edits `session.ts` too; root sequences the merge, and whichever lands
  second makes its rules or its target kind cover the other's (03b D3).
- **The type change** is additive for callers that pass `trackId`; the SDK type checks
  (`check-sdk-types.sh`) must still pass for every existing example.

## Objective gates

1. **Builder** (`sdk/test/builder-evals.mjs`, new). A `{ submixId }` target builds for the
   `builtins`, `inserts` and `console` racks; the engine boots each result (`validate` with the
   headless asset); the model's `entity_id` is the submix ID. `{ trackId, submixId }`, neither,
   and an unknown `submixId` are refused at the stated paths.
2. **`enginectl`** (`sdk/test/enginectl-cli.mjs`, new case). A request whose automation names
   `submixId` writes `entity_id` equal to that submix; one that names both is refused.
3. **Commands:**
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
     `bash scripts/sdk-package.sh check target/ci/qualification-artifacts`
   - `cargo test --locked -p session-validator` (the skill's commands still run)
4. **No rendered bit moves.** No engine or artifact code changes; the browser legs pass with
   unchanged digests.

## Test value

- Gate 1 turns red if a submix target cannot be authored for a rack, writes the wrong
  `entity_id`, or if the exactly-one rule is missing. No SDK test authors a submix target today.
- Gate 2 turns red if `enginectl` drops or rejects `submixId`.

## Dependencies

- None. It may land before or after draft 03b *Mirror the stored automation rules in the SDK
  builder* (see Hazards).
- Batch: R3.
