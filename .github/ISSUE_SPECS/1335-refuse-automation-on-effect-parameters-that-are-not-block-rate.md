# Refuse automation on effect parameters that are not block-rate

Stream I of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-13 E1).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

Some sessions carry stored automation aimed at an effect parameter that the effect keeps prepared:
its descriptor says `automatable: false`, or its `automation_rate` is not `Block`. An example is a
parametric EQ band's `enabled`. Such a session is refused on every path that accepts a session.
The paths are:

- host preparation (browser boot, C ABI boot and every C ABI rebuild);
- a C ABI transaction that edits only automation;
- `session-validator`;
- the SDK builder.

Each refusal is typed and names the automation by its ID. Today every one of these paths accepts
the session, and the automation could never be heard (decision 14 F1). This lands before the first
issue that renders stored automation (the first is filed from #1058's design), so that renderer
never meets an unrenderable target.

## Context

- **Descriptor rate.** `AutomationRate { Sample = 1, Block = 2, None = 3 }`
  (`crates/effect-contract/src/lib.rs:139`), on `ParameterDescriptor::automation_rate`
  (`:453`). A `None` parameter is never `automatable` (`:196-204`). No launch descriptor declares
  `Sample`. The only `Sample` rows are test doubles (`crates/effect-contract/src/step.rs:927`,
  `crates/conformance/src/effect.rs:51`, `crates/effect-compiler/tests/native_session.rs:34`).
  Thirteen launch parameters are `None` (`sdk/src/generated/catalog.ts`, `"automationRateName":
  "none"`), among them the EQ's `band-1-enabled` (id 1).
- **The session crate cannot see descriptors.** `validate_automation`
  (`crates/session/src/validate.rs:890-967`) checks only that an `inserts` or `console` target
  names a declared `(parameter_id, channel)` of that instance's `params`. `session` may not depend
  on `effect-contract` or `effect-compiler` (`scripts/check-effect-runtime-policy.sh:14`).
- **The one preparation.** `prepare_with_console_eligibility`
  (`crates/effect-compiler/src/prepare.rs:300-536`) runs for every host:
  - host-core calls it (`crates/host-core/src/prepare.rs:1344-1353`) for the browser
    (`hosts/host-web/src/lib.rs:6661`) and for the C ABI's boot and rebuilds;
  - `session-validator` runs it as stage 5, `prepare-effects`
    (`tools/session-validator/src/lib.rs:13-28`, `:87-94`).

  It collects `EffectDiagnostic { code, path }` values and returns them sorted (`:526-535`).
  Stable codes are frozen in `docs/EFFECT_CONTRACT_V1.md:207-227`.
- **The C ABI's automation-only edit skips preparation.**
  - `classify_live_delta` masks the automation table (`crates/host-core/src/live_delta.rs:234-236`,
    #1260 D2), so an edit that changes only automation (opcodes `0x0600`-`0x0603`,
    `crates/protocol/src/model.rs:103-110`) is a live delta with no records.
  - `commit_live` then commits it with no effect preparation
    (`crates/capi/src/runtime/control.rs:1065-1080`, `:1358-1386`).
  - The live admission does not look at descriptors.
- **Decision 14's probe.** An automation on `band-1-enabled` added to
  `fixtures/session/v1/observation-frame-shape.json` (an EQ insert, `:140-160`) passed all five
  stages of `cargo run -p session-validator -- validate`
  (`docs/rulings/live-update-versus-rebuild-2026-10-04.md:252-269`).
- **The SDK.** `resolveAutomationTarget` (`sdk/src/core/session.ts:1405-1477`) refuses a builtin
  row that is not `blockTarget` (`:1419-1424`). For an effect row it checks only the declared
  `(parameter, channel)` (`:1460-1475`). The row carries `automatable` and `automationRateName`
  (`sdk/src/generated/catalog.ts`). Its existing precedent test is
  `sdk/test/builder-evals.mjs:691-715`.

## Decisions frozen for this slice

- **D1. The rule.** An automation whose target rack is `inserts` or `console` is accepted only if
  the target instance is a native effect whose descriptor parameter with that ID has
  `automatable == true` and `automation_rate == AutomationRate::Block`. Builtin targets are
  unchanged.
  - Decision 15 E1 names `Block`. A `Sample` parameter has no rendering contract until an issue
    defines one for it, after #1058. No launch effect declares one.
- **D2. One function.** `effect_compiler::effect_automation_diagnostics(model: &SessionModel,
  registry: &NativeEffectRegistry) -> Vec<EffectDiagnostic>`:
  - **Finding the instance.** For each automation, find the target strip (track or submix,
    `model.strips()`). An `inserts` target is the insert whose `id` is `effect_id`. A `console`
    target's identity is the session console slot whose `slot` is `effect_id`.
  - **Skipped.** A third-party identity, an effect the registry lacks, and a parameter ID the
    descriptor lacks. Each of those already has its own diagnostic, so none is reported twice.
  - **Refused.** Anything else that breaks D1 is
    `EffectDiagnostic { code: "effect.automation.rate", path: "$.automation[id=<automation id>].target.parameter_id" }`.
- **D3. Preparation calls it.** `prepare_with_console_eligibility` appends D2's diagnostics before
  its final `diagnostics.is_empty()` check. So every host and `session-validator` stage 5 refuse,
  with no change of their own.
- **D4. The C ABI's live path routes such an edit to preparation.**
  - `classify_live_delta` runs D2 on `next` whenever `current.automation != next.automation`. It
    reuses the registry it already loads.
  - A non-empty result returns a new `LiveRebuild::AutomationTarget`, so the commit takes the
    rebuild path. That path's preparation refuses the transaction with the D3 diagnostic, as it
    refuses any other preparation failure.
  - Nothing is pushed or committed first, so no ack precedes the refusal. This answers the
    acked-batch question.
  - One authority (preparation) words the refusal; the classifier only routes.
- **D5. SDK.** `resolveAutomationTarget` fails an effect row whose `automatable` is false, or whose
  `automationRateName` is not `"block"`. It fails at `${path}.parameter`, with a message naming the
  parameter and saying it is prepared-only, the same wording as the builtin branch.
- **D6. The code is stable.** `effect.automation.rate` is added to
  `docs/EFFECT_CONTRACT_V1.md`'s frozen list (`:211-227`).

## Deliverables

1. D2 and D3 in `crates/effect-compiler/src/prepare.rs`, exported from `lib.rs`.
2. D4 in `crates/host-core/src/live_delta.rs`: the new variant, documented in the classifier's
   numbered rules (`:150-180`).
3. D5 in `sdk/src/core/session.ts`.
4. D6 in `docs/EFFECT_CONTRACT_V1.md`. Update decision 14's F1 status line in
   `docs/rulings/live-update-versus-rebuild-2026-10-04.md` only if root asks; S0 owns rulings.
5. The tests below.

## Authorized paths

- `crates/effect-compiler/src/{prepare,lib}.rs`, `crates/effect-compiler/tests/native_session.rs`
- `crates/host-core/src/live_delta.rs` (stream B owns it; root sequences the merge),
  `crates/host-core/tests/live_delta.rs`
- `crates/capi/src/runtime/live_tests.rs` (tests only)
- `tools/session-validator/tests/validate.rs`
- `sdk/src/core/session.ts`, `sdk/test/builder-evals.mjs`
- `docs/EFFECT_CONTRACT_V1.md`

## Non-goals

- No rendering of stored automation (#1058). No `AUTOMATION_ENQUEUE` change: refusing it while it
  is inert is *Refuse commands that would be acknowledged with no effect* (#1315).
- No change to which parameters are `Block`. The gate-expander, EQ and multiband slices (#1336,
  #1337, #1338) change descriptors, and this check follows them with no edit.
- No `session` crate dependency on effect crates. No type-level narrowing of the SDK's
  `AutomationTarget.parameter`, which is a plain string (`sdk/src/core/types.ts:337-344`).

## Hazards

- `fixtures/session/v1/canonical.json` automates parameter 1 of an insert whose identity is
  `parametric-eq`, without the `miso.` prefix. The launch registry lacks it, so D2 must skip it.
  Its existing `effect.native.unavailable` result must not gain a second diagnostic.

## Objective gates

1. **Preparation refuses** (`crates/effect-compiler/tests/native_session.rs`, new).
   - Take the observation-frame fixture plus one automation on the EQ insert's `band-1-enabled`,
     channel `left`. Preparation returns exactly one diagnostic, `effect.automation.rate`, at
     `$.automation[id=<id>].target.parameter_id`.
   - The same automation retargeted to a `Block` gain parameter declared in the instance's
     `params` prepares.
   - A console-slot target and a submix target refuse the same way.
2. **Validator** (`tools/session-validator/tests/validate.rs`, new). Decision 14's probe document
   fails at stage `prepare-effects` with that code, and stages 1-4 pass.
3. **C ABI automation-only edit** (`crates/capi/src/runtime/live_tests.rs`, new).
   - On a playing engine, a transaction that only adds the gate-1 automation is refused with the
     typed compile rejection.
   - The revision does not advance, no plan is replaced, and the next blocks are bit-identical to
     an engine that never received it.
   - A transaction that adds a `Block` target commits as before.
4. **Classifier** (`crates/host-core/tests/live_delta.rs`, new). The same delta gives
   `Err(LiveRebuild::AutomationTarget)`. A delta that adds a `Block` target stays live with no
   records, as #1260 D2 made it.
5. **SDK** (`sdk/test/builder-evals.mjs`, new, next to `:691-715`). An automation on an EQ insert's
   `band-1-enabled` throws a `MisoUsageError` that names the parameter, at `.target.parameter`. An
   automation on its `band-1-gain` builds.
6. **Commands:**
   - `cargo test --locked -p effect-compiler --features test-support`,
     `cargo test --locked -p session-validator`,
     `cargo test --locked -p host-core --features test-support --test live_delta`,
     `cargo test --locked -p capi`
   - the workspace debug leg (`test-debug-a` in `.github/workflows/qualification.yml`)
   - `mkdir -p target/ci/qualification-artifacts target/ci/qualification-named-twin && bash scripts/build-web-audioworklet.sh --named-twin target/ci/qualification-named-twin target/ci/qualification-artifacts`,
     then `bash scripts/check-sdk-generated.sh target/ci/qualification-artifacts`,
     `bash scripts/check-sdk-types.sh`,
     `bash scripts/check-sdk-headless.sh target/ci/qualification-artifacts`,
     `bash scripts/sdk-package.sh check target/ci/qualification-artifacts`
   - `bash scripts/check-effect-runtime-policy.sh`, `bash scripts/check-host-core-policy.sh`,
     `bash scripts/check-workspace-policy.sh`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`,
     `cargo fmt --all -- --check`
7. No digest moves. No fixture carries a refusable target, apart from the hazard's unavailable
   effect.

## Test value

- Gate 1 turns red if the check is missing, reads `automatable` but not the rate (or the reverse),
  resolves a console target through the strip entry instead of the slot identity, or reports
  twice. No test refuses an effect automation by rate today.
- Gate 2 turns red if the check lives somewhere `session-validator` does not run.
- Gate 3 turns red if the C ABI's live path still commits an automation-only edit without the
  check: decision 14's acked-but-inert gap.
- Gate 4 turns red if the classifier never routes a refusable target to the rebuild, or if it
  sends every automation edit there (a needless rebuild, against #1260).
- Gate 5 turns red if the SDK still builds a session the engine refuses.

## Dependencies

- None. It lands before any issue that renders stored automation (the first is filed from the
  design of *Research: render stored session automation in the engine, identically on every
  platform*, #1058).
