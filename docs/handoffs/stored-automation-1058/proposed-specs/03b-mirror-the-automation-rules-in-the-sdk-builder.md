# Mirror the stored automation rules in the SDK builder

Proposed by *Research: render stored session automation in the engine, identically on every
platform* (#1058), answer A3, under decision 15 D15-16
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`). A draft for root
to file. Code anchors verified on `6ee64f484`.

## Product outcome

The SDK builder refuses, at the field the author wrote and with the engine's own code, every
stored automation the engine refuses under drafts 01, 02 and 03a. An author learns of a duplicate
lane, a fader ride to +30 dB or an HPF that crosses the LPF at the `.automation()` call that wrote
it, not at boot. The builder never builds a session the engine refuses, and never refuses one it
accepts.

## Context

- **The SDK builder.** `automation()` (`sdk/src/core/session.ts:886-899`) calls
  `#validateAutomation` (`:969-985`). `normalizeAutomation` (`:1479-1537`) calls
  `resolveAutomationTarget` (`:1405-1477`; the builtin branch `:1411-1434`) and builds each
  segment: an effect value through `automationScalar` (`:1540-1554`, which already checks the
  catalog domain), a builtin value through `f32` only, and every `exponential` value positive.
  The segment's `unit` is the row's (`unit: resolved.unit`), so an SDK author cannot write a unit
  mismatch. Static builtin values are checked by `builtinNumber` (`:400-409`) and `builtinFilter`
  (`:419-440`, rate-keyed). Refusals carry the engine's code from `CODE` (`:205-218`) through
  `fail` (`:220-222`).
- **The catalog** gives each builtin row its `domain`, `minimum`, `maximum`, `maximumByRate`,
  `disabledValue` and `unitName` (`sdk/src/generated/catalog.ts:261`, the `builtins.parameters`
  rows), and each effect row its `domainName`, `minimum`, `maximum` and `unitName`.
- **The engine rules mirrored here.** Draft 01 D1-D3: `automation.duplicate_lane`, pan or matrix by
  strip kind (`reference.missing_entity` at the parameter), `automation.discontinuity_spacing`.
  Draft 02 D1-D2: `builtin.automation.{unit,domain,shape}` and `builtin.filter.order` over curves.
  Draft 03a D1: `effect.automation.{unit,domain,shape}`.
- **SDK precedent tests.** `sdk/test/builder-evals.mjs:691-728` (builtin target rows) and the
  engine-parity helper `engineFirstCode` (`:568-574`, `:645-651`), which boots a hand-edited model
  in the headless engine and returns its first code.

## Decisions frozen for this slice

- **D1. The rules, at the author's path, with the engine's code** (each code added to `CODE`):
  - duplicate lane (draft 01 D1): when `automation()` adds an entry whose lane an entry already
    in the builder holds, the builder refuses the call with `automation.duplicate_lane` at
    `automation("<id>").target.channel`, where `<id>` is the entry the engine refuses: of the
    overlapping pair, the one whose `id` is greater (later in canonical order), whichever call
    came first. The new entry is not added. So the code and the refused entry match the engine's
    for every call order;
  - discontinuity spacing (draft 01 D3): `automation.discontinuity_spacing` at
    `.segments[<k>].startSample` or `.endSample`;
  - pan on a matrix strip, matrix on a pan strip (draft 01 D2): `reference.missing_entity` at
    `.target.parameter`;
  - builtin domain and shape (draft 02 D1): `builtin.automation.domain` through the same tests as
    `builtinNumber` and `builtinFilter` at the session rate, and `builtin.automation.shape`;
  - filter order (draft 02 D2): the same split, margin and code `builtin.filter.order`;
  - effect shape (draft 03a D1): `effect.automation.shape`; effect domain stays
    `automationScalar`'s check, which now carries `effect.automation.domain`.
- **D2. Units need no SDK rule:** the builder writes the row's unit.
- **D3. One resolution path.** The rules run where the builder resolves the target, so they apply
  to every target kind it resolves (a submix target too, once draft 03c lands, whichever of the
  two merges second).
- **D4. The acked-batch question** does not arise: the builder commits nothing.

## Deliverables

1. D1-D3 in `sdk/src/core/session.ts`.
2. The tests below.

## Authorized paths

- `sdk/src/core/session.ts`, `sdk/test/builder-evals.mjs`

## Non-goals

- Any engine rule (drafts 01, 02, 03a). Submix authoring (draft 03c).
- A type-level narrowing of `AutomationTarget.parameter` (#1335's non-goal stands).

## Hazards

- **Two spellings of each rule.** The SDK mirror is a second spelling of engine rules. Every SDK
  refusal test pairs with an `engineFirstCode` assertion on the same defect, written by hand into
  the model, so the two cannot drift silently.
- **The filter order rule in TypeScript** must evaluate exponential values as the engine does at
  the two ends. JavaScript `Math.pow` is not `crates/math`; the `2^-22` margin is far wider than
  the difference, but a value exactly at the margin may differ. The test set avoids margin ties; a
  tie is the engine's to decide.
- **Shared file.** Draft 03c edits `session.ts` too; root sequences the merge.

## Objective gates

1. **SDK** (`sdk/test/builder-evals.mjs`, new, next to `:691-728`). Each D1 refusal throws a
   `MisoUsageError` with the engine's code at the stated path, and `engineFirstCode` on the same
   defect returns the same code. Two overlapping entries added in either call order are refused
   at the path of the one whose `id` is greater, the entry the engine refuses. A `left` and a
   `right` entry on one parameter, a 64-sample
   discontinuity spacing, an HPF `exponential` from 20 to 200 Hz and an HPF `step` to `0` build,
   and the engine boots each result.
2. **Commands:**
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
     `bash scripts/sdk-package.sh check target/ci/qualification-artifacts`
   - `bash scripts/check-workspace-policy.sh`
3. **No rendered bit moves.** No engine or artifact code changes; the browser legs of the
   `browser` job in `.github/workflows/qualification.yml` pass with unchanged digests.

## Test value

- Gate 1 turns red if the SDK builds a session the engine refuses (a rule left out), names a
  different entry than the engine for a duplicate lane (the newer call instead of the greater
  `id`), refuses one
  the engine accepts (a rule too wide: `left` plus `right`, the 64-sample spacing, the filters'
  legal shapes), or reports a code other than the engine's.

## Dependencies

- Draft 01 *Validate stored automation lanes in the session crate and state the hold rule*,
  draft 02 *Validate builtin automation targets against their rows at preparation* and draft 03a
  *Validate effect automation units, domains and shapes at preparation*: D1 mirrors their rules,
  so each must be on `main` first.
- Batch: R3.
