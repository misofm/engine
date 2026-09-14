# Expose structured parameter edits with authoritative unit metadata

Astra XHIGH scoped this issue at `551f6d7e`; root approves this bounded
amendment on 2026-09-14 under the owner’s #804 delivery request. This replaces
#147’s blanket unit-suffix naming migration. Luna XHIGH implements bounded
tranches; fresh Astra MEDIUM verifies one coherent attempt. Maximum five
attempts, with root commits at every focused-green tranche before layering.

Tranche 1: Rust builtin unit authority, metadata generation/validation, and SDK
builtin unit consumption. Tranche 2: object-shaped live edits and their focused
SDK/type/real-engine tests. One adversarial verdict after the complete attempt;
no unrelated DSP or general API redesign.

## Problem and smallest product slice

The effect catalog already describes every parameter's semantic key in `name`, numeric ID, `unit`/`unitName`, domain, bounds, default, channel policy, smoothing and `liveUpdatable`. For example compressor parameter 1 is `threshold`, with unit `db`. Renaming it to `threshold_db` would break the generated SDK's public parameter names and require avoidable caller migration. Session `effect()` already accepts a typed parameter object; the missing structured entry point is the live console's positional `EffectEdits.parameter(name, value, options)`.

Builtin metadata lacks explicit unit fields. The SDK currently infers units from a private `BUILTIN_UNIT_BY_MAPPING` table in `sdk/src/core/session.ts`, while Rust separately derives them in `builtin_parameter_lattice_points`. Mapping alone is insufficient: `delay_samples` and matrix coefficients both have a linear mapping, but their units differ.

Deliver an additive object form for live effect parameter edits and explicit builtin units from the existing Rust authority. Keep the stable numeric runtime ABI and existing semantic key spellings.

## Frozen contract and decisions

1. The metadata `name` field is the stable semantic parameter key. Preserve all existing spellings, including `threshold`, `band-1-gain`, existing keys containing spaces, and builtin `hpf_hz`/`lpf_hz`. Keys must be nonempty and unique within their owning effect or builtin table. Units come from explicit unit metadata, never parsed suffixes. Do not introduce a duplicate `key` column, suffix aliases, normalized-key lookup table, or a naming migration. Human labels can gain a separate surface in a future issue if needed; existing key identity cannot be changed as a cosmetic label edit.
2. Add numeric `unit` and string `unitName` to every builtin metadata row, using the same `ParameterUnit` vocabulary as effects. Extract the unit derivation already in `crates/builtins/src/lib.rs::builtin_parameter_lattice_points` into one shared function, used by that lattice adapter and the metadata generator. Preserve its semantics: boolean is `linear`, cutoff is `hz`, trim/fader is `db`, delay is `samples`, matrix/pan is `linear`. Do not create a second manually maintained table or add redundant unit state to every descriptor.
3. Preserve builtin domain fields, especially `disabledValue`, `maximumByRate`, and `domain`. A rate-keyed cutoff is not one universal inclusive range; zero remains a distinct disabled sentinel. Preserve effect domain/enumeration/lattice fields. Capabilities continue to derive from the engine's existing authority; this issue does not flip HPF/LPF live flags or claim a new automation path.
4. Add the following overload, preserving the existing positional overload unchanged:

   ```ts
   compressor.parameter({
     key: "threshold",
     value: -18,
     channel: "both",
     smoothingSamples: 64,
   });
   ```

   Export a catalog-derived discriminated union `LiveEffectParameterEdit<E>` keyed by the existing live parameter-name union. Each key selects its exact numeric/boolean/enum value type and channel policy. The union must reject mismatched key/value pairs without generic-union widening. A prepared-only parameter remains impossible in the live type and refused at runtime. Object `key` resolves the catalog `name`; it is an argument label, not a second parameter identity.

5. Normalize both overloads immediately to the same existing validation/record-construction path. Values are in the descriptor's declared units; callers need no unit suffix or unit-conversion heuristic. Validate unknown keys, incorrect value types, enum labels, nonfinite/out-of-range values, lane policy and smoothing before transport. Object form must reject unknown own fields, including a guessed `unit` field, so JavaScript callers cannot accidentally submit a value in a different unit. Preserve current float-based live values and the agent handle's exact-decimal persisted lattice behavior.
6. Building an edit remains pure. The result is the same `LaneEdit` consumed by `EngineConsole.submit` or existing `ConsoleWriter.submitEdits`; no new writer, queue, command kind, acknowledgment type, ABI structure, transport, or effect-slot lookup. A successful object-edit acknowledgment is the actual engine report at `appliedAtSample`; failures admit zero.
7. Replace SDK `BUILTIN_UNIT_BY_MAPPING` with each generated builtin row's `unitName`. Existing session output must remain unchanged. Do not expand prepared-only automation eligibility just because units are now available.

## Exact implementation paths

- `crates/builtins/src/lib.rs`: extract existing builtin-unit authority for reuse; no DSP edit.
- `tools/parameter-metadata/src/lib.rs`: emit builtin unit fields using that authority.
- `tools/parameter-metadata/tests/round_trip.rs`: check actual emitted key uniqueness and builtin unit correspondence, including delay versus matrix and Hz cutoffs.
- `scripts/check-parameter-metadata-v1.py` and `scripts/fixtures/parameter-metadata-v1-self-test.json`: accept and validate additive builtin fields, key uniqueness, unit vocabulary and red mutations. Keep unrelated gates intact.
- `sdk/assets/miso-engine-v1-parameter-metadata.json` and `sdk/src/generated/catalog.ts`: regenerate through existing scripts, never hand-edit. `sdk/codegen/generate.mjs` needs no new table or generator framework.
- `sdk/src/core/session.ts`: consume generated builtin `unitName`, removing its private conversion table.
- `sdk/src/core/console.ts`: one exported object-edit union and additive overload sharing the existing validator.
- `sdk/test/console-types.ts`, `sdk/test/console-evals.mjs`, `sdk/test/agent-evals.mjs`, `sdk/test/builder-evals.mjs`, and `sdk/test/barrel-surface.ts`: extend existing relevant tests only; no parallel harness.
- `sdk/README.md`: document the name-as-key/unit contract and object submission example. Existing root barrel `export *` exposes the new type automatically.

## Minimum meaningful acceptance

1. Metadata generator evidence covers all builtin rows and all effect names. Builtin delay reports `samples` while linear matrix/pan report `linear`; HPF/LPF report `hz` with their original disabled/rate-keyed domain. A duplicate or empty key and an incorrect unit fail the existing validator's red probes. Existing IDs, name spellings, capabilities, command numbers and schema tag remain unchanged.
2. Strict type probes accept the compressor example and representative boolean/enum inputs where actually live; reject unknown/prepared-only keys, wrong scalar type, enum label, shared-lane address, unknown fields and key/value union mismatches. Positional calls and existing exported type consumers still compile.
3. Exercise object edits through an actual headless Wasm console, including a command that measurably changes PCM and reports the true application sample. Confirm the equivalent positional edit yields identical addressing/values and settled output. Extend the existing browser transport test to observe the same generated record and actual whole-batch report. Reuse existing rejection/backpressure/torn-ack probes; do not invent synthetic success receipts.
4. Bad object fields/values fail before any transport call. Existing agent lattice round trips and session builder canonical-output evidence stay green after builtin unit authority moves.
5. Run proportional gates once at the coherent checkpoint, recording commands/results. Reuse the existing current artifact directory for this metadata/SDK-only slice; the #804 DSP release will rebuild immutable artifacts at its own boundary:

   ```sh
   cargo test --locked -p parameter-metadata
   python3 -B scripts/check-parameter-metadata-v1.py --self-test
   bash scripts/check-sdk-generated.sh
   bash scripts/check-sdk-types.sh
   bash scripts/check-sdk-headless.sh /absolute/current-artifact-directory
   bash scripts/sdk-package.sh check /absolute/current-artifact-directory
   ```

   Regenerate source assets before these checks using `node sdk/codegen/assets.mjs` then `node sdk/codegen/generate.mjs`. Refresh any artifact metadata required by the existing package consistency gate through the existing artifact builder; never rewrite a published immutable artifact in place.

## Compatibility and handoff

Existing effect/parameter IDs, semantic names, positional APIs, canonical session data, exact-decimal agent values, metadata schema V1 identity, ABI layout and wire bytes remain compatible. The metadata change adds fields; repository strict readers move in the same commit and packaged readers receive the matching catalog. No renamed DSP descriptor strings or descriptor-identity repin is needed.

#804's dedicated EQ cuts can subsequently use keys such as `hpf-enabled`, `hpf-frequency`, `hpf-q`, `lpf-enabled`, `lpf-frequency`, and `lpf-q` with explicit units and appended numeric IDs, as approved by its DSP brief. These are accepted automatically by the existing catalog-derived object type when their actual live capability becomes true. This issue neither implements those filters nor prematurely advertises them.

Root owns coherent checkpoint commits, model-workflow assignment, independent verification, GitHub synchronization and final package delivery. No benchmark, expanded research corpus, DSP audit, or app migration is required to close this bounded slice.

## Attempt 1 — metadata tranche checkpoint

Luna XHIGH completed the eight-path metadata/unit tranche. Shared Rust unit
lookup now feeds both lattice construction and generated builtin metadata;
the SDK consumes generated unit names. Key uniqueness and wrong-unit mutations
are covered by the existing validator and round-trip tests. No DSP or capability
flags changed. Evidence: `/tmp/147-tranche1-evidence.md`.

Passed: `cargo test --locked -p parameter-metadata`, metadata validator self-test
and current-asset validation, `bash scripts/check-sdk-generated.sh`, formatting,
and diff hygiene. The first SDK type check reported missing node_modules (exit
2); root installed locked dependencies with `npm ci --ignore-scripts` and reran
`bash scripts/check-sdk-types.sh`, which passed. This is a compiling checkpoint,
not the complete attempt or adversarial PASS; object-edit tranche and its
real-engine/package evidence follow.
