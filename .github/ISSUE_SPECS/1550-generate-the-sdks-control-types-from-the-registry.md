# Generate the SDK's control types from the registry

Slice S2b of decision 17 (`docs/rulings/one-control-abstraction-2026-10-13.md`, design points C1
and "How to add a control"). Split from the planner's S2. Code anchors verified on `main`
`df585e503`. Feature sprint mode (owner, 2026-10-13): one implementer, normal tests, a batch
review; no per-attempt verifier or mutation runs.

## Product outcome

The SDK's control types come from the core's row tables, with no hand-written list. A new row in
`controls`, in `BUILTIN_PARAMETER_DESCRIPTORS` or in an effect descriptor reaches the SDK's types
when the metadata is regenerated, and the existing drift gate refuses a stale copy.

## Context

- **The generator.** `tools/parameter-metadata` renders one JSON document
  (`render`, `tools/parameter-metadata/src/lib.rs:141-297`; top-level keys `schema`, `abiVersion`,
  `commandRecordBytes`, `maximumCommandRecords`, `commandKinds`, `commandReasons`,
  `observationVocabularies`, `builtins`, `effects`). Builtin rows come from
  `BUILTIN_PARAMETER_DESCRIPTORS` (`builtin_parameter`, `:711`) and effect rows from the registry
  (`effect_parameter`, `:617`). `--write DIR` / `--check DIR` (`run`, `:858-911`) write or compare
  `miso-engine-v1-parameter-metadata.json` and the ABI layout. The checked-in copies are in
  `sdk/assets/`; `sdk/codegen/assets.mjs` refreshes them.
- **The SDK side.** `sdk/codegen/generate.mjs` reads the two JSON assets (`:39-42`) and writes
  `sdk/src/generated/{catalog.ts,abi.ts,provenance.ts}`: `catalog.ts` is the frozen JSON plus
  derived types such as `EffectParameter<E>` and `BuiltinParameter`.
- **The gates.** `scripts/check-sdk-generated.sh [ARTIFACT_DIR]` re-derives JSON and TypeScript
  and compares byte for byte; `scripts/check-sdk-types.sh` runs `tsc --noEmit`.
- **The rows.** #1546 and #1549 give every row an address, a descriptor (with display strings) and a
  class. Some descriptor fields depend on the owner's shape or the rate: a route row's class
  depends on its destination (#1546 D5), and `hpf_hz`/`lpf_hz`'s maximum on the launch rate (#1546 D6).

## Decisions frozen for this slice

- **D1. One new JSON key.** The metadata gains a top-level `controls` object, after `effects`;
  every existing key keeps its bytes. `controls` holds one row list per row source:
  `stripOwn`, `builtins`, `routeOwn`, `vcaOwn`, `sessionOwn`, and `effects` (an object keyed by
  effect ID, each with its bypass, parameter and link-mode rows). A row carries `control`, `name`,
  `displayName`, `displayUnit`, `unit`, `kind`, `minimum`, `maximum`, `disabledValue`, `default`,
  `taper`, `step`, `precision`, `ladder`, `choices`, `ramp` (a `LiveRampRow` name, or `"fixed"`),
  `class`, `reason`, `reopen`, `scope`, `family`, `lanes` (`"perLane"` or `"shared"`) and
  `automatable`.
- **D2. Shape- and rate-keyed fields.** A field that depends on the owner's shape or the rate is
  emitted per case: route rows carry `class`/`reason`/`reopen` under `intoSubmix` and `intoOutput`;
  a rate-keyed `maximum` is an object keyed by each launch rate (44100, 48000, 88200, 96000).
- **D3. One source.** The generator reads the rows through `controls` (a `pub` row-table
  accessor), never through its own list. `tools/parameter-metadata` gains a `controls` dependency.
- **D4. Generated TypeScript.** `generate.mjs` adds `sdk/src/generated/controls.ts`: the frozen
  `CONTROLS` value; the types `ControlOwnerKind`, `ControlSection`, `ControlLane` and
  `ControlAddress` (owner kind and ID, section kind and slot ID, control, lane); and the name unions
  `StripOwnControlName`, `BuiltinControlName`, `RouteControlName`, `VcaControlName`,
  `SessionControlName` and `EffectControlName<E>`. It is exported where `catalog.ts`'s types are.
- **D5. No behaviour change.** No SDK runtime path reads the new file yet (#1563 does). The ABI
  layout document does not change.

## Deliverables

1. D1-D3 in `tools/parameter-metadata` and the `controls` accessor.
2. D4 in `sdk/codegen/generate.mjs` and the regenerated `sdk/src/generated/*` and `sdk/assets/*`.
3. The tests below.

## Authorized paths

- `tools/parameter-metadata/{Cargo.toml,src/lib.rs,tests/round_trip.rs}`, `crates/controls/**`,
  `Cargo.lock`.
- `sdk/codegen/generate.mjs`, `sdk/src/generated/*`, `sdk/assets/miso-engine-v1-parameter-metadata.json`,
  `sdk/src/index.ts` (one export line), `sdk/test/` (one eval).

## Non-goals

- No SDK runtime change, no builder, no wire encoder (#1563, #1564).
- No change to `commandKinds`, `builtins` or `effects` in the JSON.

## Objective gates

Commands: `cargo test --locked -p parameter-metadata`, `bash scripts/check-sdk-generated.sh`,
`bash scripts/check-sdk-types.sh`, `python3 -B scripts/check-sdk-deletions.py`,
`bash scripts/sdk-package.sh check <artifacts>`.

1. **The registry equals the JSON.** A `parameter-metadata` test walks `controls`' row tables and
   asserts one JSON row per table row, with equal fields (D1, D2), for every launch effect and every
   own and builtin row.
2. **Drift is refused.** `check-sdk-generated.sh` passes on the regenerated files (the existing
   byte-for-byte gate now covers `controls.ts`).
3. **Types compile and constrain.** An SDK type eval (`sdk/test/`) assigns a valid `ControlAddress`
   for a fader, a send gain and a compressor threshold, and a `// @ts-expect-error` line for an
   unknown row name, and `check-sdk-types.sh` passes.

## Test value

- Gate 1 turns red if a row exists in the core and not in the SDK catalog, or with a different
  domain, class or ramp.
- Gate 2 turns red if a hand edit or a stale asset makes the SDK disagree with the core.
- Gate 3 turns red if the generated types accept a row name the core does not have.

## Dependencies

- #1549 (effect rows and display strings).

## Supersedes and shrinks

- None.
