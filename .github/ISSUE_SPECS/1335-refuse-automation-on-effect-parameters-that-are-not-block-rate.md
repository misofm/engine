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
  (`docs/rulings/live-update-versus-rebuild-2026-10-04.md:263-272`).
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
     `cargo test --locked -p host-core --features test-support,control-provider --test live_delta`,
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

## Attempt record

### Attempt 1 (implementer, 2026-10-05)

**Delivered.**
- D1-D3: `effect_compiler::effect_automation_diagnostics` in `crates/effect-compiler/src/prepare.rs`
  (exported through `lib.rs`'s `pub use prepare::*`). `prepare_with_console_eligibility` appends
  its diagnostics before the final `is_empty()` check.
- D4: `LiveRebuild::AutomationTarget` in `crates/host-core/src/live_delta.rs`. It is the new step 4
  of the classifier's numbered rules (the per-track step is now 5). It runs only when
  `current.automation != next.automation`, and it shares the call's lazily loaded registry through
  a new `load_registry` helper, which `parameter_records` uses too.
- D5: `resolveAutomationTarget` (`sdk/src/core/session.ts`) refuses an effect row that is not
  `automatable` or whose `automationRateName` is not `"block"`. It fails at `${path}.parameter`
  with the builtin branch's wording, and the check runs before the channel and declaration checks.
- D6: `effect.automation.rate` is appended to `docs/EFFECT_CONTRACT_V1.md`'s frozen list, with
  one paragraph giving its path. The decision 14 F1 line is untouched, since root did not ask.

**Deviations, all inside the authorized paths.**
- The test double in `crates/effect-compiler/tests/native_session.rs` (`parametric-eq`) declared
  `AutomationRate::Sample`, and `canonical.json` automates it. Under D1, four existing tests then
  failed with an extra `effect.automation.rate`. The double now declares `Block`.
  `retired_multiband_parameter_id_two_rejects_before_native_publication` retargets the
  `canonical.json` insert to the multiband, whose parameter 1 (crossover) is not block-rate, so it
  now clears `model.automation`. This follows the precedent of the console-eligibility test in the
  same file. The spec's hazard holds only on the launch registry; it did not anticipate the test
  registry, which carries an unprefixed `parametric-eq`.
- Gate 1's "channel `left`": the observation fixture declares `band-1-enabled` as `both`. The test
  redeclares it on `left` (it is per-lane) and automates `left`. Gate 2 keeps decision 14's probe
  verbatim (`both`).
- Gate 3 uses `eq_session` (the nine-track EQ fixture). The refused target is the `eq0` console
  EQ's `band-1-enabled`, and the accepted one is its `band-1-gain` on `left`, as declared. A second
  `Rig` that never receives the transaction is the bit-identity reference.
- Gate 6's `cargo test -p host-core --features test-support --test live_delta` runs **0 tests**:
  the file is `#![cfg(feature = "control-provider")]`. The real run is
  `--features test-support,control-provider` (the workspace leg enables it through unification).

**Mutation runs.** Each mutation was applied, run and reverted with `git checkout`.
- M1, no call in preparation: gate 1 is red (3 tests) and gate 2 is red.
- M2, refuse only `!automatable`: `an_automatable_sample_rate_target_is_refused` is red.
- M2b, refuse only `rate == None`: the same test is red.
- M3, resolve a console target among the strip's inserts:
  `console_and_submix_automations_on_a_prepared_parameter_are_refused` is red.
- M4, walk only the tracks: the same test is red at the submix assertion.
- M5, report an automation whose effect the registry lacks:
  `unavailable_factory_and_resource_caps_return_no_partial_session` is red (the hazard assertion),
  and so is session-validator's existing `fixtures_distinguish_schema_examples_from_launch_effects`.
- M10, run the check twice: gate 1 is red (3 tests).
- M6, no classifier step 4: gate 4 is red, and gate 3 is red (the edit commits live).
- M7, step 4 routes every automation change: gate 4 is red at the block-rate assertion, as are the
  existing `model_only_edits_are_live_with_no_records` and gate 3 (no live commit).
- M8, no SDK check, and M8b, refuse only `"sample"`: gate 5 ("refused by name") is red.
- M9, drop `!automatable ||` and keep only `rate != Block`: **green, an equivalent mutant.**
  `NativeEffectRegistry::new` validates descriptors, and `parameter_automation_smoothing_valid`
  makes `Block`/`Sample` imply `automatable`. No registered descriptor can tell the two apart.

**Gate 6 results.**
- `cargo test --locked -p effect-compiler --features test-support`: 38 passed, 0 failed.
- `cargo test --locked -p session-validator`: 19 passed, 0 failed.
- `cargo test --locked -p host-core --features test-support --test live_delta`: 0 tests (see
  above).
  - With `,control-provider`: 31 passed, 0 failed.
- `cargo test --locked -p capi`: 84 passed, 0 failed.
- Workspace debug leg (`test-debug-a`, exact command): 1432 passed, 0 failed, 10 ignored.
  - Also run: `test-debug-b`'s DSP and conformance leg (the conformance double declares `Sample`),
    812 passed, 0 failed.
- The web artifact build succeeded, and so did these SDK checks:
  - `check-sdk-generated`: current.
  - `check-sdk-types`: passed, after a one-time `npm ci` in `sdk/`.
  - `check-sdk-headless`: 361 passed, 0 failed.
  - `sdk-package.sh check`: 18 passed, gate passed.
- Policy scripts: `check-effect-runtime-policy`, `check-host-core-policy` and
  `check-workspace-policy` all report ok.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --all -- --check`: clean.

**Gate 7.** No fixture, generated file or digest changed. `fixtures_distinguish_schema_examples_from_launch_effects`
still passes every launch fixture through stage 5. Only `canonical.json` stops there, with its one
`effect.native.unavailable`.

### Attempt 1 folds (implementer, 2026-10-05)

The verdict was PASS. These changes fold its MINOR and NIT findings. All of them are in
`crates/effect-compiler/tests/native_session.rs` and this spec.

- **MINOR 1.** New test `automation_skips_targets_already_refused_for_another_reason`, on
  `canonical.json`'s automated `eq` insert:
  - with a `cid` identity, preparation returns exactly
    `[effect.third_party.unavailable_at_launch @ $.tracks[id=vocal].effects[id=eq]]`;
  - retargeted to the multiband, with its retired parameter 2 declared and automated, it returns
    exactly `[effect.parameter.unknown @ $.tracks[id=vocal].effects[id=eq]]`.
- **MINOR 2.** The hazard block in `unavailable_factory_and_resource_caps_return_no_partial_session`
  is deleted. M5 above is corrected: session-validator's fixture test was red too.
- **NIT 1.** The console case now also refuses the `post_insert` `limiter` slot's `lookahead`
  (parameter 3, rate `None`).
- **NIT 2.** Gate 6's `live_delta` command now has `--features test-support,control-provider`.

**Mutation runs.** Each was applied to `prepare.rs`, run and reverted with `git checkout`.
- M11, report the rate refusal for a third-party identity: the new test is red at its `cid` case.
- M12, report it for a parameter ID the descriptor lacks: the new test is red at its
  unknown-parameter case.
- M3b, a console target takes the first slot: `console_and_submix_automations_on_a_prepared_parameter_are_refused`
  is red at the `lookahead` assertion.
- M5, again after the deletion: session-validator's `fixtures_distinguish_schema_examples_from_launch_effects`
  is red. `unavailable_factory_and_resource_caps_return_no_partial_session` is red too, but only
  because its existing empty-registry assertion reads `diagnostics[0]`, and the extra
  `$.automation...` diagnostic sorts first.

**Commands.** `cargo test --locked -p effect-compiler --features test-support`,
`cargo clippy --locked -p effect-compiler --all-targets --all-features -- -D warnings`,
`cargo fmt --all -- --check` and `bash scripts/check-workspace-policy.sh`: 39 passed and
0 failed; clippy, fmt and the workspace policy are clean.
