# Issue #215 de-versioning: the classification inventory

**Issue**: [#215](https://github.com/misofm/engine-v2/issues/215) (owner ruling, 2026-08-28) --
pre-launch there is no reason to denote *internal* types as `V1`. **Class**: a naming ruling plus
the line-by-line evidence for it. This file is the audit surface: every `V1`/`_v1` spelling in the
tree is named below exactly once, in class 1 (renamed) or class 2 (kept), with the reason.

## The rule

*A version suffix is contract identity, never decoration.* A name keeps its version **iff the name
itself is visible across a boundary that a future v2 must be able to coexist with** -- a wire tag, a
schema version, an exported ABI symbol, a gate-pinned or sealed spelling. A Rust/TypeScript name
that never leaves the binary is born unversioned even when its *bytes* go on a wire: the judgment is
about the **name's** visibility, not the payload's.

## Method

The inventory is mechanical, not sampled. `git ls-files` was tokenised with
`\b[A-Za-z_][A-Za-z0-9_]*\b` over every tracked text file (51 binary files skipped), and every token
matching `V1`/`_V1`/`_v1` was collected: **589 distinct spellings over 7 514 hit lines**. Each was
classified; the two proof greps at the bottom re-derive the same partition from the post-rename
tree.

| | count |
|---|---|
| distinct spellings found | 589 |
| **class 1** -- renamed | **420** |
| **class 2** -- kept | **169** |

## Scope: which trees were rewritten

Renames land in `crates/`, `tools/`, `hosts/`, `scripts/`, `docs/`, `fuzz/` and `AGENTS.md`.

Four trees are **frozen** and were not rewritten, because their content is a record rather than an
implementation:

| tree | why frozen |
|---|---|
| `.github/ISSUE_SPECS/**` (incl. `BRIEFS/`) | the accepted specification of each issue as it was accepted, carrying sha256 pins of the artifacts of its day. Rewriting names inside it would forge the record. |
| `artifacts/**` | sealed measurement evidence. |
| `dsp-research/**` | sealed listening evidence (`listening/issue033/*.schema.json` are hash-pinned packet members) and archived research. |
| `fixtures/**` | fixture **bytes** are digest identity. |

Consequence, stated plainly so the verifier does not read it as an omission: a class-1 name that is
*also* mentioned in a frozen tree keeps its old spelling there. Three such mentions exist --
`EqSvfWordsV1` in `artifacts/issue-loop-eq-r1/README.md`, `EffectProgramKeyV1` in a
`fixtures/session/v1/compressor-bank-observation.toml` comment, and the `Prepared*KernelV1` family
throughout `ISSUE_SPECS`. They are historical text about the code as it then was.

### The one fixture line

`fixtures/session/v1/compressor-bank-observation.toml` line 8 is a **comment** naming
`EffectProgramKeyV1`. It was left byte-identical. That fixture is reached only by `include_str!`
from `crates/miso-engine-host-core/tests/effect_observation.rs`; it is named by no `MANIFEST.tsv`,
no `MANIFEST.sha256` and no `sha256sum` line in `scripts/`. Not one byte under `fixtures/` changed
in this PR.

## Class 2 -- the version is contract identity, and stays


**C-ABI header macro** (56)

`MISO_ENGINE_EFFECT_AUTOMATION_RATE_BLOCK_V1`, `MISO_ENGINE_EFFECT_AUTOMATION_RATE_NONE_V1`, `MISO_ENGINE_EFFECT_AUTOMATION_RATE_SAMPLE_V1`, `MISO_ENGINE_EFFECT_CONTRACT_V1_H`, `MISO_ENGINE_EFFECT_DESCRIPTOR_BUFFER_TOO_SMALL_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_ENUM_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_FLAGS_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_FLOAT_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_HEADER_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_INSPECTION_ABI_VERSION_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_LENGTH_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_LIMIT_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_NULL_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_OFFSET_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_OK_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_ORDER_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_OVERFLOW_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_RESERVED_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_SEMANTIC_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_TEXT_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_UNAVAILABLE_V1`, `MISO_ENGINE_EFFECT_DESCRIPTOR_V1_H`, `MISO_ENGINE_EFFECT_LINK_AVERAGE_V1`, `MISO_ENGINE_EFFECT_LINK_DUAL_MONO_V1`, `MISO_ENGINE_EFFECT_LINK_MAXIMUM_V1`, `MISO_ENGINE_EFFECT_PARAMETER_AUTOMATABLE_V1`, `MISO_ENGINE_EFFECT_PARAMETER_CHANNEL_POLICY_PER_LANE_V1`, `MISO_ENGINE_EFFECT_PARAMETER_CHANNEL_POLICY_SHARED_V1`, `MISO_ENGINE_EFFECT_PARAMETER_DOMAIN_BOOLEAN_V1`, `MISO_ENGINE_EFFECT_PARAMETER_DOMAIN_CONTINUOUS_V1`, `MISO_ENGINE_EFFECT_PARAMETER_DOMAIN_ENUMERATION_V1`, `MISO_ENGINE_EFFECT_PARAMETER_HAS_MAXIMUM_V1`, `MISO_ENGINE_EFFECT_PARAMETER_HAS_MINIMUM_V1`, `MISO_ENGINE_EFFECT_PARAMETER_MAPPING_EXPONENTIAL_V1`, `MISO_ENGINE_EFFECT_PARAMETER_MAPPING_LINEAR_V1`, `MISO_ENGINE_EFFECT_PARAMETER_MAPPING_LOGARITHMIC_V1`, `MISO_ENGINE_EFFECT_PARAMETER_MAPPING_STEPPED_V1`, `MISO_ENGINE_EFFECT_PARAMETER_READABLE_V1`, `MISO_ENGINE_EFFECT_PARAMETER_UNIT_DB_V1`, `MISO_ENGINE_EFFECT_PARAMETER_UNIT_HZ_V1`, `MISO_ENGINE_EFFECT_PARAMETER_UNIT_LINEAR_V1`, `MISO_ENGINE_EFFECT_PARAMETER_UNIT_MILLISECONDS_V1`, `MISO_ENGINE_EFFECT_PARAMETER_UNIT_RATIO_V1`, `MISO_ENGINE_EFFECT_PARAMETER_UNIT_SAMPLES_V1`, `MISO_ENGINE_EFFECT_PORT_DUAL_MONO_PLANAR_V1`, `MISO_ENGINE_EFFECT_PORT_MAIN_INPUT_V1`, `MISO_ENGINE_EFFECT_PORT_MAIN_OUTPUT_V1`, `MISO_ENGINE_EFFECT_PORT_SIDECHAIN_INPUT_V1`, `MISO_ENGINE_EFFECT_QUALITY_DRAFT_V1`, `MISO_ENGINE_EFFECT_QUALITY_HIGH_V1`, `MISO_ENGINE_EFFECT_QUALITY_NORMAL_V1`, `MISO_ENGINE_EFFECT_SMOOTHING_LINEAR_V1`, `MISO_ENGINE_EFFECT_SMOOTHING_NONE_V1`, `MISO_ENGINE_EFFECT_SMOOTHING_ONE_POLE_99_V1`, `MISO_ENGINE_EFFECT_TAIL_FINITE_V1`, `MISO_ENGINE_EFFECT_TAIL_INFINITE_V1`

**exact class-2 list** (40)

`BUILTINS_AND_METERING_V1`, `CIDv1`, `CONTROL_BTLV_V1`, `C_ABI_V1_QUALIFICATION`, `DESCRIPTOR_V1`, `EFFECT_CONTRACT_V1`, `EFFECT_DESCRIPTOR_INSPECTION_ABI_VERSION_V1`, `EFFECT_DESCRIPTOR_WIRE_V1`, `EFFECT_INTERCHANGE_QUALIFICATION_V1`, `EFFECT_OBSERVATION_V1`, `EFFECT_PACKAGE_V1`, `EFFECT_STATE_MIGRATION_V1`, `EFFECT_STATE_V1`, `Issue007ListeningPreparationV1`, `Issue007ListeningQualificationV1`, `Issue007ListeningResponseV1`, `Issue007ListeningRevealV1`, `KernelBackendV1`, `MisoSessionSourceV1`, `NATIVE_PCM_REFERENCE_RUNNER_V1`, `PROTOCOL_MAJOR_V1`, `PROTOCOL_MINOR_V1`, `PreparedDeltaBankKernelV1`, `QUALITIES_V1`, `RetainedF64IncrementalV1`, `SESSION_SCHEMA_V1`, `SESSION_SCHEMA_VERSION_V1`, `V1`, `_V1`, `_v1`, `descriptor_v1_qualification`, `effect_descriptor_v1_reference`, `miso_effect_descriptor_reference_v1`, `miso_effect_package_reference_v1`, `miso_effect_state_reference_v1`, `miso_engine_effect_contract_v1`, `miso_engine_graph_v1`, `nmiso_engine_effect_descriptor_v1_inspect_observations`, `tv1`, `v1`

**wasm export symbol** (29)

`bmiso_engine_web_v1_`, `miso_engine_web_v1_abi_version`, `miso_engine_web_v1_buffer_capacity`, `miso_engine_web_v1_buffer_ptr`, `miso_engine_web_v1_command_report_ptr`, `miso_engine_web_v1_command_submit`, `miso_engine_web_v1_compile`, `miso_engine_web_v1_config_bytes`, `miso_engine_web_v1_config_new`, `miso_engine_web_v1_config_ptr`, `miso_engine_web_v1_console_track_count`, `miso_engine_web_v1_console_track_id`, `miso_engine_web_v1_dispose`, `miso_engine_web_v1_meter_header_ptr`, `miso_engine_web_v1_meter_lease`, `miso_engine_web_v1_meter_poll`, `miso_engine_web_v1_prepare`, `miso_engine_web_v1_render`, `miso_engine_web_v1_resource_ptr`, `miso_engine_web_v1_source_`, `miso_engine_web_v1_source_channels`, `miso_engine_web_v1_source_count`, `miso_engine_web_v1_source_frames`, `miso_engine_web_v1_source_id`, `miso_engine_web_v1_source_sample_rate`, `miso_engine_web_v1_source_seek`, `miso_engine_web_v1_source_start_frame`, `miso_engine_web_v1_source_submit`, `miso_engine_web_v1_status_ptr`

**C-ABI header type/function** (24)

`miso_engine_effect_automation_rate_v1`, `miso_engine_effect_descriptor_diagnostic_code_v1`, `miso_engine_effect_descriptor_diagnostic_v1`, `miso_engine_effect_descriptor_summary_v1`, `miso_engine_effect_descriptor_v1`, `miso_engine_effect_descriptor_v1_inspect`, `miso_engine_effect_descriptor_v1_inspect_observations`, `miso_engine_effect_enum_choice_record_v1`, `miso_engine_effect_link_mode_bits_v1`, `miso_engine_effect_observation_record_v1`, `miso_engine_effect_parameter_channel_policy_v1`, `miso_engine_effect_parameter_domain_v1`, `miso_engine_effect_parameter_flags_v1`, `miso_engine_effect_parameter_mapping_v1`, `miso_engine_effect_parameter_record_v1`, `miso_engine_effect_parameter_unit_v1`, `miso_engine_effect_port_layout_v1`, `miso_engine_effect_port_record_v1`, `miso_engine_effect_port_role_v1`, `miso_engine_effect_quality_record_v1`, `miso_engine_effect_quality_v1`, `miso_engine_effect_smoothing_rule_v1`, `miso_engine_effect_state_v1_new_export`, `miso_engine_effect_tail_kind_v1`

**appears only in sealed/frozen trees (issue specs, artifacts, research, fixtures)** (20)

`FallbackReasonV1`, `GateGainKernelV1`, `KernelV1`, `NativeSchedulerConfigV1`, `NativeSchedulerV1`, `NativeWorkerPoolV1`, `PreparedBiquadBankKernelV1`, `PreparedBuiltinInputBankV1`, `PreparedGateGainKernelV1`, `PreparedNativeGraphPlanV1`, `PreparedTptBankKernelV1`, `RackProgramSignatureV1`, `ReleaseQualificationReportV1`, `RenderWaveV1`, `SchedulerSelectionV1`, `ThirdPartyEffectAbiV1`, `WorkerLeaseV1`, `partition_stable_units_v1`, `partition_weighted_units_v1`, `verify_lane_payload_v1`

### Borderline class-2 rulings, with the reason

| spelling | ruling | reason |
|---|---|---|
| `miso.command.v1`, `miso.observe.v1`, `miso.ack.v1`, `miso.error.v1`, `miso.meter.v1`, `miso.meters.v1`, `miso.status.v1`, `miso.sessionmap.v1`, `miso.source.v1`, `miso.seek.v1`, `miso.ready.v1`, `miso.dispose.v1`, `miso.telemetry.v1`, `miso.unsupported.v1`, `miso.web.parameter-metadata.v1`, `miso.web.qualification.*.v1`, `miso.engine.effect-descriptor.identity.v1`, `miso.engine.effect-state.current-layout.v1`, `miso.effect-*.v1` | KEEP | wire tags. They are the *reason* a v2 can coexist. These are string literals, not identifiers, so they do not appear in the token counts above; the second proof grep pins them. |
| `miso_engine_web_v1_*` (28 exports) | KEEP | `pub extern "C"` Wasm exports. `miso-engine-v2-audio-worklet.js` reads them off the instance **by name**; `check-web-audioworklet.sh` and `test-web-audioworklet.mjs` pin the exact list. |
| `miso_engine_effect_*_v1`, `MISO_ENGINE_EFFECT_*_V1` (80) | KEEP | the C inspection ABI in `crates/miso-engine-effect-package/include/miso_engine_effect_descriptor_v1.h`, its `_H` guards, and the Rust `EFFECT_DESCRIPTOR_INSPECTION_ABI_VERSION_V1` mirror the ABI gate holds equal to the macro. A third-party compiles against these spellings. |
| `SESSION_SCHEMA_VERSION_V1`, `PROTOCOL_MAJOR_V1`, `PROTOCOL_MINOR_V1`, `ProtocolVersion::V1` | KEEP | they *are* the version. `schema_version` in every checked-in session document is unmoved data. |
| `docs/SESSION_SCHEMA_V1.md`, `EFFECT_CONTRACT_V1.md`, `EFFECT_PACKAGE_V1.md`, `EFFECT_STATE_V1.md`, `EFFECT_STATE_MIGRATION_V1.md`, `EFFECT_DESCRIPTOR_WIRE_V1.md`, `EFFECT_OBSERVATION_V1.md`, `EFFECT_INTERCHANGE_QUALIFICATION_V1.md`, `BUILTINS_AND_METERING_V1.md`, `CONTROL_BTLV_V1.md`, `C_ABI_V1_QUALIFICATION.md`, `NATIVE_PCM_REFERENCE_RUNNER_V1.md` | KEEP the file names | each is the normative text *of* a versioned contract and is referenced by path from `ISSUE_SPECS` (some with sha256). Doc **bodies** were rewritten where they name a class-1 type. Judgment call resolved by "when in doubt KEEP". |
| every `scripts/*-v1.{sh,py}` file name, `scripts/fixtures/parameter-metadata-v1-self-test.json`, `crates/miso-engine-effect-package/tests/{descriptor,package}_v1_qualification.rs` | KEEP | these are the qualification suites *for* the versioned contracts, and they are sweep rows. **No sweep row was renamed.** |
| `fixtures/*/v1/**` path segments | KEEP | fixture roots; `MANIFEST.tsv` / `MANIFEST.sha256` tree hashes are computed over them. |
| `DESCRIPTOR_V1` / `DESCRIPTOR_V2`, `QUALITIES_V1` / `QUALITIES_V2` (`effect-compiler/tests/migration.rs`) | KEEP | not a suffix on one type: a **pair** of test descriptors standing for the old and new sides of a migration. The `1` is the datum under test. |
| `miso_engine_graph_v1` | KEEP | it is literally inside `fixtures/graph/v1/*.dot` as `digraph miso_engine_graph_v1 {`, and `canonical.rs` emits that exact string. Renaming moves fixture bytes. |
| `MisoSessionSourceV1` | KEEP | it names no type. It is the **negative** payload of `check-session-map-shape.py --self-test`: the mutation that mints a `…V1` name and must be refused. It exists to defend this very ruling. |
| `KernelBackendV1`, `PreparedDeltaBankKernelV1` | KEEP | tombstones. Both types were **deleted** (#84 phase A). The surviving mentions are `check-parametric-eq-targets.sh`'s forbidden-pattern list and `docs/TARGET_MATRIX.md`'s deletion record; both must keep the deleted spelling or they stop forbidding it. |
| `miso_engine_effect_contract_v1` | KEEP | the deleted orphan header `include/miso_engine_effect_contract_v1.h`; `check-effect-runtime-policy.sh` forbids its return by exact name. |
| `Issue007Listening*V1`, `RetainedF64IncrementalV1` | KEEP | JSON-schema `title` fields inside hash-pinned listening packets, and archived research. No code. |
| `nmiso_engine_effect_descriptor_v1_inspect_observations`, `bmiso_engine_web_v1_` | KEEP | not names: `\n`/`\b`-prefixed grep patterns for class-2 symbols, tokenised as one word. |

### Borderline class-1 rulings, with the reason

| spelling | ruling | reason |
|---|---|---|
| the `.d.ts` types -- `MisoCommandKindV1`, `MisoCommandReasonV1`, `MisoAckV1`, `MisoStatusV1`, `MisoErrorV1`, `MisoSessionMapV1`, `MisoMeterFrameV1`, `MisoObservation*V1`, `MisoWeb*V1`, `MisoSource/SeekRequestV1`, `CreateMisoAudioWorkletHostOptionsV1`, `MisoUnsupportedBrowserV1`, `MisoTelemetryFrameV1`, `MisoCommandV1/RequestV1/AckV1` | RENAME | compile-time-only TypeScript names. Nothing reads them at runtime -- what the app reads by name is the `tag` string (class 2) and the enum **member** names and numeric values (unchanged). The repo had already ruled this: `check-session-map-shape.py` refuses any new `.d.ts` type ending in `V1`, citing #215, and `MisoSessionSource` already landed unversioned. Renaming makes the existing rule consistent instead of grandfathering nine exceptions. |
| `MisoAudioWorkletHostV1` (JS class) | RENAME | a `class` expression never exported by name; callers get it from `createMisoAudioWorkletHost()`. Renaming to `MisoAudioWorkletHost` also makes the class agree with the `.d.ts` interface of that name. |
| `WebPrepareConfigV1`, `WebStatusV1`, `WebResourceReportV1`, `WebMeterHeaderV1`, `WebCommandReportV1` | RENAME | their **bytes** are the browser wire, but the Rust names never leave the binary. Exactly the case the rule names. |
| `BoundEffectDescriptorWireV1`, `EffectDescriptorV1`, `ParameterDescriptorV1`, `PortDescriptorV1`, `QualityDescriptorV1`, `EnumChoiceV1`, the whole `EffectState*V1` / `EffectPackage*V1` family | RENAME | same test. The wire's identity is the class-2 string `miso.engine.effect-descriptor.identity.v1`; the Rust struct that encodes it is an implementation detail. |
| `SessionEditV1`, `SessionTomlV1` | RENAME | Rust model types. `"SessionEdit"` in `protocol/src/schema.rs` is a registry **string** and is untouched. |
| `session_structural_symmetry_v1`, `ChannelSymmetryWitnessV1`, `SymmetryEventV1`, `SeamSideV1`, `TrackInputRecordV1` | RENAME | named by #215 itself. `builtins-compiler/src/lib.rs` already carries a doc comment saying `track_input_delay_symmetric` was born unversioned because "the neighbours are on that issue's own rename list" -- these are the neighbours. |
| `BUILTIN_PARAMETER_DESCRIPTORS_V1`, `*_DESCRIPTOR_V1`, `*_PARAMETERS_V1`, `*_OBSERVATIONS_V1` (per-effect statics) | RENAME | crate-local statics. |
| the 189 `tools/miso-engine-audit` spellings | RENAME | one file, `fixture_builtins.rs`, holds the builtins fixture oracle. Nothing it names crosses a boundary; the strings it *writes* are separately class 2 and unchanged. |
| `MeterSnapshotV1`, `MeterRecordV1`, `EnumChoiceV1`, `ParameterDescriptorV1`, `MatrixRampV1`, `CASE_COUNT_V1`, `parse_manifest_v1`, `identity_section_v1`, `json_string_field_v1`, `program_v1` | RENAME, collision checked | each unversioned target already exists somewhere in the tree. All are in a different crate or a different namespace from the renamed item (`fixture_builtins.rs` refers to `miso_engine_builtins::MeterSnapshot` by full path; `RackPlan::program` is a method beside a `program` field, which Rust keeps in separate namespaces). Verified by `cargo check --workspace --all-targets`. |

## Class 1 -- renamed, grouped by declaration site

#### `crates/miso-engine-builtins` — 9

| old | new | declared in |
|---|---|---|
| `BUILTIN_PARAMETER_DESCRIPTORS_V1` | `BUILTIN_PARAMETER_DESCRIPTORS` | `crates/miso-engine-builtins/src/lib.rs` |
| `BuiltinFaderBankV1` | `BuiltinFaderBank` | `crates/miso-engine-builtins/src/lib.rs` |
| `BuiltinInputBankV1` | `BuiltinInputBank` | `crates/miso-engine-builtins/src/lib.rs` |
| `BuiltinMatrixBankV1` | `BuiltinMatrixBank` | `crates/miso-engine-builtins/src/lib.rs` |
| `BuiltinParameterDescriptorV1` | `BuiltinParameterDescriptor` | `crates/miso-engine-builtins/src/lib.rs` |
| `DisabledOrRateKeyedHertzV1` | `DisabledOrRateKeyedHertz` | `crates/miso-engine-builtins/src/lib.rs` |
| `FaderMuteRampBuiltinsV1` | `FaderMuteRampBuiltins` | `crates/miso-engine-builtins/src/lib.rs` |
| `builtin_filter_cutoff_maximum_hz_v1` | `builtin_filter_cutoff_maximum_hz` | `crates/miso-engine-builtins/src/lib.rs` |
| `validate_builtin_filter_cutoff_v1` | `validate_builtin_filter_cutoff` | `crates/miso-engine-builtins/src/lib.rs` |

#### `crates/miso-engine-builtins-compiler` — 9

| old | new | declared in |
|---|---|---|
| `BuiltinResourceReportV1` | `BuiltinResourceReport` | `crates/miso-engine-builtins-compiler/src/lib.rs` |
| `BuiltinRetainedLayoutV1` | `BuiltinRetainedLayout` | `crates/miso-engine-builtins-compiler/src/lib.rs` |
| `BuiltinStageKeyV1` | `BuiltinStageKey` | `crates/miso-engine-builtins-compiler/src/lib.rs` |
| `SessionPoolClassesV1` | `SessionPoolClasses` | `crates/miso-engine-builtins-compiler/src/lib.rs` |
| `TrackControlRecordV1` | `TrackControlRecord` | `crates/miso-engine-builtins-compiler/src/lib.rs` |
| `TrackFaderRecordV1` | `TrackFaderRecord` | `crates/miso-engine-builtins-compiler/src/lib.rs` |
| `TrackInputRecordV1` | `TrackInputRecord` | `crates/miso-engine-builtins-compiler/src/lib.rs` |
| `session_structural_symmetry_v1` | `session_structural_symmetry` | `crates/miso-engine-builtins-compiler/src/lib.rs` |
| `track_mono_source_v1` | `track_mono_source` | `crates/miso-engine-builtins-compiler/src/lib.rs` |

#### `crates/miso-engine-capi` — 1

| old | new | declared in |
|---|---|---|
| `engine_creation_is_transactional_and_validates_v1` | `engine_creation_is_transactional_and_validates` | `crates/miso-engine-capi/src/ffi.rs` |

#### `crates/miso-engine-compressor` — 10

| old | new | declared in |
|---|---|---|
| `COMPRESSOR_DESCRIPTOR_V1` | `COMPRESSOR_DESCRIPTOR` | `crates/miso-engine-compressor/src/lib.rs` |
| `COMPRESSOR_OBSERVATIONS_V1` | `COMPRESSOR_OBSERVATIONS` | `crates/miso-engine-compressor/src/lib.rs` |
| `COMPRESSOR_PARAMETERS_V1` | `COMPRESSOR_PARAMETERS` | `crates/miso-engine-compressor/src/lib.rs` |
| `GainReductionV1` | `GainReduction` | `crates/miso-engine-compressor/tests/observation.rs` |
| `ObservationCadenceV1` | `ObservationCadence` | `crates/miso-engine-compressor/src/lib.rs` |
| `ObservationChannelsV1` | `ObservationChannels` | `crates/miso-engine-compressor/src/lib.rs` |
| `ObservationCostV1` | `ObservationCost` | `crates/miso-engine-compressor/src/lib.rs` |
| `ObservationFoldV1` | `ObservationFold` | `crates/miso-engine-compressor/src/lib.rs` |
| `ObservationKindV1` | `ObservationKind` | `crates/miso-engine-compressor/src/lib.rs` |
| `PreparedCompressorGainMixKernelV1` | `PreparedCompressorGainMixKernel` | `crates/miso-engine-compressor/src/kernel.rs` |

#### `crates/miso-engine-conformance` — 1

| old | new | declared in |
|---|---|---|
| `PcmFixtureV1` | `PcmFixture` | `crates/miso-engine-conformance/src/fixture.rs` |

#### `crates/miso-engine-core` — 6

| old | new | declared in |
|---|---|---|
| `ArenaLeaseV1` | `ArenaLease` | `crates/miso-engine-core/src/realtime/disjoint.rs` |
| `ObservationPublisherV1` | `ObservationPublisher` | `crates/miso-engine-core/src/realtime/observe.rs` |
| `ObservationReaderV1` | `ObservationReader` | `crates/miso-engine-core/src/realtime/observe.rs` |
| `ObservationSlotV1` | `ObservationSlot` | `crates/miso-engine-core/src/realtime/observe.rs` |
| `ObservationWindowV1` | `ObservationWindow` | `crates/miso-engine-core/src/realtime/observe.rs` |
| `PlanUnitEligibilityV1` | `PlanUnitEligibility` | `crates/miso-engine-core/src/realtime/plan.rs` |

#### `crates/miso-engine-delay` — 2

| old | new | declared in |
|---|---|---|
| `DELAY_DESCRIPTOR_V1` | `DELAY_DESCRIPTOR` | `crates/miso-engine-delay/src/lib.rs` |
| `DELAY_PARAMETERS_V1` | `DELAY_PARAMETERS` | `crates/miso-engine-delay/src/lib.rs` |

#### `crates/miso-engine-effect-compiler` — 33

| old | new | declared in |
|---|---|---|
| `EFFECT_STATE_MIGRATION_V1_UNAVAILABLE_INDEX` | `EFFECT_STATE_MIGRATION_UNAVAILABLE_INDEX` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `EffectBankPreparationV1` | `EffectBankPreparation` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `EffectControlProducerV1` | `EffectControlProducer` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `EffectObservationHandleV1` | `EffectObservationHandle` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `EffectStateMigrationAdmissionV1` | `EffectStateMigrationAdmission` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `EffectStateMigrationDiagnosticCodeV1` | `EffectStateMigrationDiagnosticCode` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `EffectStateMigrationDiagnosticV1` | `EffectStateMigrationDiagnostic` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `EffectStateMigrationRegistrationV1` | `EffectStateMigrationRegistration` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `EffectStateMigrationStepFailureV1` | `EffectStateMigrationStepFailure` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `EffectStateMigrationStepReportV1` | `EffectStateMigrationStepReport` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `EffectStateMigrationStepV1` | `EffectStateMigrationStep` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `EffectStateMigrationWorkspaceRequirementsV1` | `EffectStateMigrationWorkspaceRequirements` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `EffectStateRestoreAdmissionV1` | `EffectStateRestoreAdmission` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `RegisteredEffectStateMigrationV1` | `RegisteredEffectStateMigration` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `ResolvedEffectStateMigrationV1` | `ResolvedEffectStateMigration` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `RestoredScalarEffectStateV1` | `RestoredScalarEffectState` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `StateMigrationRegistryV1` | `StateMigrationRegistry` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `UnpublishedEffectBankStateV1` | `UnpublishedEffectBankState` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `WireBoundNativeEffectFactoryV1` | `WireBoundNativeEffectFactory` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `attach_effect_console_v1` | `attach_effect_console` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `attach_effect_observation_v1` | `attach_effect_observation` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `bind_effect_state_migration_registration_v1` | `bind_effect_state_migration_registration` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `bind_native_effect_factory_state_v1` | `bind_native_effect_factory_state` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `launch_native_effect_registry_v1` | `launch_native_effect_registry` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `prepare_unpublished_effect_bank_state_v1` | `prepare_unpublished_effect_bank_state` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `resolve_effect_state_migration_v1` | `resolve_effect_state_migration` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `restore_scalar_effect_state_v1` | `restore_scalar_effect_state` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `restore_scalar_effect_state_with_migration_v1` | `restore_scalar_effect_state_with_migration` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `restore_unpublished_effect_bank_track_state_v1` | `restore_unpublished_effect_bank_track_state` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `restore_unpublished_effect_bank_track_state_with_migration_v1` | `restore_unpublished_effect_bank_track_state_with_migration` | `crates/miso-engine-effect-compiler/src/migration.rs` |
| `scalar_effect_state_v1_requirements` | `scalar_effect_state_requirements` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `snapshot_scalar_effect_state_v1` | `snapshot_scalar_effect_state` | `crates/miso-engine-effect-compiler/src/prepare.rs` |
| `snapshot_unpublished_effect_bank_track_state_v1` | `snapshot_unpublished_effect_bank_track_state` | `crates/miso-engine-effect-compiler/src/prepare.rs` |

#### `crates/miso-engine-effect-contract` — 17

| old | new | declared in |
|---|---|---|
| `ChannelSymmetryWitnessV1` | `ChannelSymmetryWitness` | `crates/miso-engine-effect-contract/src/symmetry.rs` |
| `EffectControlRecordV1` | `EffectControlRecord` | `crates/miso-engine-effect-contract/src/live.rs` |
| `EffectDescriptorV1` | `EffectDescriptor` | `crates/miso-engine-effect-contract/src/lib.rs` |
| `EffectProgramKeyV1` | `EffectProgramKey` | `crates/miso-engine-effect-contract/src/lib.rs` |
| `EnumChoiceV1` | `EnumChoice` | `crates/miso-engine-effect-contract/src/lib.rs` |
| `LiveConsoleRecordV1` | `LiveConsoleRecord` | `crates/miso-engine-effect-contract/src/symmetry.rs` |
| `ObservationDescriptorV1` | `ObservationDescriptor` | `crates/miso-engine-effect-contract/src/lib.rs` |
| `ObservationLaneV1` | `ObservationLane` | `crates/miso-engine-effect-contract/src/live.rs` |
| `ObservationSampleV1` | `ObservationSample` | `crates/miso-engine-effect-contract/src/lib.rs` |
| `ObservationTapV1` | `ObservationTap` | `crates/miso-engine-effect-contract/src/live.rs` |
| `ParameterDescriptorV1` | `ParameterDescriptor` | `crates/miso-engine-effect-contract/src/lib.rs` |
| `PortDescriptorV1` | `PortDescriptor` | `crates/miso-engine-effect-contract/src/lib.rs` |
| `PreparedPortsV1` | `PreparedPorts` | `crates/miso-engine-effect-contract/src/lib.rs` |
| `QualityDescriptorV1` | `QualityDescriptor` | `crates/miso-engine-effect-contract/src/lib.rs` |
| `SeamSideV1` | `SeamSide` | `crates/miso-engine-effect-contract/src/symmetry.rs` |
| `SymmetryEventV1` | `SymmetryEvent` | `crates/miso-engine-effect-contract/src/symmetry.rs` |
| `validate_descriptor_v1` | `validate_descriptor` | `crates/miso-engine-effect-contract/src/lib.rs` |

#### `crates/miso-engine-effect-package` — 78

| old | new | declared in |
|---|---|---|
| `ArtifactSelectionRequestV1` | `ArtifactSelectionRequest` | `crates/miso-engine-effect-package/src/package.rs` |
| `BorrowedEffectDescriptorViewV1` | `BorrowedEffectDescriptorView` | `crates/miso-engine-effect-package/src/wire.rs` |
| `BorrowedObservationV1` | `BorrowedObservation` | `crates/miso-engine-effect-package/src/wire.rs` |
| `BorrowedParameterV1` | `BorrowedParameter` | `crates/miso-engine-effect-package/src/wire.rs` |
| `BorrowedPortV1` | `BorrowedPort` | `crates/miso-engine-effect-package/src/wire.rs` |
| `BorrowedQualityV1` | `BorrowedQuality` | `crates/miso-engine-effect-package/src/wire.rs` |
| `BoundEffectDescriptorWireV1` | `BoundEffectDescriptorWire` | `crates/miso-engine-effect-package/src/wire.rs` |
| `BoundEffectStateMigrationEdgeV1` | `BoundEffectStateMigrationEdge` | `crates/miso-engine-effect-package/src/state.rs` |
| `EFFECT_DESCRIPTOR_WIRE_V1_UNAVAILABLE` | `EFFECT_DESCRIPTOR_WIRE_UNAVAILABLE` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EFFECT_PACKAGE_V1_UNAVAILABLE_INDEX` | `EFFECT_PACKAGE_UNAVAILABLE_INDEX` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EFFECT_PACKAGE_V1_UNAVAILABLE_OFFSET` | `EFFECT_PACKAGE_UNAVAILABLE_OFFSET` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EFFECT_STATE_V1_BUFFER_ENVELOPE_OUTPUT` | `EFFECT_STATE_BUFFER_ENVELOPE_OUTPUT` | `crates/miso-engine-effect-package/src/state.rs` |
| `EFFECT_STATE_V1_BUFFER_INITIAL_VALUE_SCRATCH` | `EFFECT_STATE_BUFFER_INITIAL_VALUE_SCRATCH` | `crates/miso-engine-effect-package/src/state.rs` |
| `EFFECT_STATE_V1_BUFFER_PAYLOAD_SCRATCH` | `EFFECT_STATE_BUFFER_PAYLOAD_SCRATCH` | `crates/miso-engine-effect-package/src/state.rs` |
| `EFFECT_STATE_V1_UNAVAILABLE_INDEX` | `EFFECT_STATE_UNAVAILABLE_INDEX` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EFFECT_STATE_V1_UNAVAILABLE_OFFSET` | `EFFECT_STATE_UNAVAILABLE_OFFSET` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EffectArtifactAuthoringV1` | `EffectArtifactAuthoring` | `crates/miso-engine-effect-package/src/package.rs` |
| `EffectArtifactKindV1` | `EffectArtifactKind` | `crates/miso-engine-effect-package/src/package.rs` |
| `EffectDescriptorBindingErrorKindV1` | `EffectDescriptorBindingErrorKind` | `crates/miso-engine-effect-package/src/wire.rs` |
| `EffectDescriptorBindingErrorV1` | `EffectDescriptorBindingError` | `crates/miso-engine-effect-package/src/wire.rs` |
| `EffectDescriptorEnumChoiceRecordV1` | `EffectDescriptorEnumChoiceRecord` | `crates/miso-engine-effect-package/src/ffi.rs` |
| `EffectDescriptorIdentityV1` | `EffectDescriptorIdentity` | `crates/miso-engine-effect-package/src/wire.rs` |
| `EffectDescriptorObservationRecordV1` | `EffectDescriptorObservationRecord` | `crates/miso-engine-effect-package/src/ffi.rs` |
| `EffectDescriptorParameterRecordV1` | `EffectDescriptorParameterRecord` | `crates/miso-engine-effect-package/src/ffi.rs` |
| `EffectDescriptorPortRecordV1` | `EffectDescriptorPortRecord` | `crates/miso-engine-effect-package/src/ffi.rs` |
| `EffectDescriptorQualityRecordV1` | `EffectDescriptorQualityRecord` | `crates/miso-engine-effect-package/src/ffi.rs` |
| `EffectDescriptorSummaryV1` | `EffectDescriptorSummary` | `crates/miso-engine-effect-package/src/ffi.rs` |
| `EffectDescriptorWireDiagnosticCodeV1` | `EffectDescriptorWireDiagnosticCode` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EffectDescriptorWireDiagnosticV1` | `EffectDescriptorWireDiagnostic` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EffectPackageAuthoringV1` | `EffectPackageAuthoring` | `crates/miso-engine-effect-package/src/package.rs` |
| `EffectPackageDiagnosticCodeV1` | `EffectPackageDiagnosticCode` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EffectPackageDiagnosticV1` | `EffectPackageDiagnostic` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EffectPackageLimitsV1` | `EffectPackageLimits` | `crates/miso-engine-effect-package/src/package.rs` |
| `EffectStateDerivedResourcesV1` | `EffectStateDerivedResources` | `crates/miso-engine-effect-package/src/state.rs` |
| `EffectStateDescriptorProvenanceV1` | `EffectStateDescriptorProvenance` | `crates/miso-engine-effect-package/src/state.rs` |
| `EffectStateDiagnosticCodeV1` | `EffectStateDiagnosticCode` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EffectStateDiagnosticV1` | `EffectStateDiagnostic` | `crates/miso-engine-effect-package/src/diagnostic.rs` |
| `EffectStateInitialValuesV1` | `EffectStateInitialValues` | `crates/miso-engine-effect-package/src/state.rs` |
| `EffectStateLimitsV1` | `EffectStateLimits` | `crates/miso-engine-effect-package/src/state.rs` |
| `EffectStateMigrationEdgeErrorV1` | `EffectStateMigrationEdgeError` | `crates/miso-engine-effect-package/src/state.rs` |
| `EffectStateReplayViewV1` | `EffectStateReplayView` | `crates/miso-engine-effect-package/src/state.rs` |
| `EffectStateRequirementsV1` | `EffectStateRequirements` | `crates/miso-engine-effect-package/src/state.rs` |
| `EffectStateSelectorV1` | `EffectStateSelector` | `crates/miso-engine-effect-package/src/state.rs` |
| `ParsedEffectStateV1` | `ParsedEffectState` | `crates/miso-engine-effect-package/src/state.rs` |
| `VerifiedArtifactV1` | `VerifiedArtifact` | `crates/miso-engine-effect-package/src/package.rs` |
| `VerifiedEffectArtifactIteratorV1` | `VerifiedEffectArtifactIterator` | `crates/miso-engine-effect-package/src/package.rs` |
| `VerifiedEffectDescriptorWireV1` | `VerifiedEffectDescriptorWire` | `crates/miso-engine-effect-package/src/wire.rs` |
| `VerifiedEffectPackageV1` | `VerifiedEffectPackage` | `crates/miso-engine-effect-package/src/package.rs` |
| `VerifiedEffectStateV1` | `VerifiedEffectState` | `crates/miso-engine-effect-package/src/state.rs` |
| `bind_effect_descriptor_wire_v1` | `bind_effect_descriptor_wire` | `crates/miso-engine-effect-package/src/wire.rs` |
| `bind_effect_state_migration_edge_v1` | `bind_effect_state_migration_edge` | `crates/miso-engine-effect-package/src/state.rs` |
| `bind_parsed_effect_state_v1` | `bind_parsed_effect_state` | `crates/miso-engine-effect-package/src/state.rs` |
| `effect_descriptor_identity_v1` | `effect_descriptor_identity` | `crates/miso-engine-effect-package/src/wire.rs` |
| `effect_descriptor_wire_v1_required_size` | `effect_descriptor_wire_required_size` | `crates/miso-engine-effect-package/src/wire.rs` |
| `effect_package_cid_v1` | `effect_package_cid` | `crates/miso-engine-effect-package/src/cid.rs` |
| `effect_package_v1_required_size` | `effect_package_required_size` | `crates/miso-engine-effect-package/src/package.rs` |
| `effect_state_bound_selector_v1` | `effect_state_bound_selector` | `crates/miso-engine-effect-package/src/state.rs` |
| `effect_state_derived_resources_v1` | `effect_state_derived_resources` | `crates/miso-engine-effect-package/src/state.rs` |
| `effect_state_descriptor_provenance_v1` | `effect_state_descriptor_provenance` | `crates/miso-engine-effect-package/src/state.rs` |
| `effect_state_expected_metadata_v1` | `effect_state_expected_metadata` | `crates/miso-engine-effect-package/src/state.rs` |
| `effect_state_replay_view_from_verified_v1` | `effect_state_replay_view_from_verified` | `crates/miso-engine-effect-package/src/state.rs` |
| `effect_state_v1_requirements` | `effect_state_requirements` | `crates/miso-engine-effect-package/src/state.rs` |
| `encode_effect_descriptor_wire_v1` | `encode_effect_descriptor_wire` | `crates/miso-engine-effect-package/src/wire.rs` |
| `encode_effect_package_v1` | `encode_effect_package` | `crates/miso-engine-effect-package/src/package.rs` |
| `encode_effect_state_v1` | `encode_effect_state` | `crates/miso-engine-effect-package/src/state.rs` |
| `inspect_effect_descriptor_v1` | `inspect_effect_descriptor` | `crates/miso-engine-effect-package/Cargo.toml` |
| `inspect_effect_state_selector_v1` | `inspect_effect_state_selector` | `crates/miso-engine-effect-package/src/state.rs` |
| `parse_effect_state_structure_v1` | `parse_effect_state_structure` | `crates/miso-engine-effect-package/src/state.rs` |
| `select_effect_package_artifact_v1` | `select_effect_package_artifact` | `crates/miso-engine-effect-package/src/package.rs` |
| `validate_effect_state_current_layout_v1` | `validate_effect_state_current_layout` | `crates/miso-engine-effect-package/src/state.rs` |
| `validate_effect_state_metadata_v1` | `validate_effect_state_metadata` | `crates/miso-engine-effect-package/src/state.rs` |
| `validate_effect_state_replay_configuration_v1` | `validate_effect_state_replay_configuration` | `crates/miso-engine-effect-package/src/state.rs` |
| `validate_effect_state_replay_v1` | `validate_effect_state_replay` | `crates/miso-engine-effect-package/src/state.rs` |
| `verify_canonical_package_v1` | `verify_canonical_package` | `crates/miso-engine-effect-package/tests/MUTATIONS.md` |
| `verify_effect_descriptor_wire_v1` | `verify_effect_descriptor_wire` | `crates/miso-engine-effect-package/src/wire.rs` |
| `verify_effect_package_cid_v1` | `verify_effect_package_cid` | `crates/miso-engine-effect-package/src/cid.rs` |
| `verify_effect_package_v1` | `verify_effect_package` | `crates/miso-engine-effect-package/src/package.rs` |
| `verify_effect_state_v1` | `verify_effect_state` | `crates/miso-engine-effect-package/src/state.rs` |

#### `crates/miso-engine-gate-expander` — 3

| old | new | declared in |
|---|---|---|
| `GATE_EXPANDER_DESCRIPTOR_V1` | `GATE_EXPANDER_DESCRIPTOR` | `crates/miso-engine-gate-expander/src/lib.rs` |
| `GATE_EXPANDER_OBSERVATIONS_V1` | `GATE_EXPANDER_OBSERVATIONS` | `crates/miso-engine-gate-expander/src/lib.rs` |
| `GATE_EXPANDER_PARAMETERS_V1` | `GATE_EXPANDER_PARAMETERS` | `crates/miso-engine-gate-expander/src/lib.rs` |

#### `crates/miso-engine-graph` — 4

| old | new | declared in |
|---|---|---|
| `GraphBankCohortV1` | `GraphBankCohort` | `crates/miso-engine-graph/src/lib.rs` |
| `GraphEffectControlBindingV1` | `GraphEffectControlBinding` | `crates/miso-engine-graph/src/lib.rs` |
| `GraphEffectObservationBindingV1` | `GraphEffectObservationBinding` | `crates/miso-engine-graph/src/lib.rs` |
| `UnitIdentityV1` | `UnitIdentity` | `crates/miso-engine-graph/src/runtime.rs` |

#### `crates/miso-engine-host-core` — 3

| old | new | declared in |
|---|---|---|
| `HostConsoleHandlesV1` | `HostConsoleHandles` | `crates/miso-engine-host-core/src/prepare.rs` |
| `HostConsoleRequestV1` | `HostConsoleRequest` | `crates/miso-engine-host-core/src/prepare.rs` |
| `StartedRenderSessionV1` | `StartedRenderSession` | `crates/miso-engine-host-core/src/render_session.rs` |

#### `crates/miso-engine-multiband-compressor` — 3

| old | new | declared in |
|---|---|---|
| `MULTIBAND_COMPRESSOR_DESCRIPTOR_V1` | `MULTIBAND_COMPRESSOR_DESCRIPTOR` | `crates/miso-engine-multiband-compressor/src/lib.rs` |
| `MULTIBAND_COMPRESSOR_OBSERVATIONS_V1` | `MULTIBAND_COMPRESSOR_OBSERVATIONS` | `crates/miso-engine-multiband-compressor/src/lib.rs` |
| `MULTIBAND_COMPRESSOR_PARAMETERS_V1` | `MULTIBAND_COMPRESSOR_PARAMETERS` | `crates/miso-engine-multiband-compressor/src/lib.rs` |

#### `crates/miso-engine-parametric-eq` — 7

| old | new | declared in |
|---|---|---|
| `EQ_BAND_DESCRIPTORS_V1` | `EQ_BAND_DESCRIPTORS` | `crates/miso-engine-parametric-eq/src/lib.rs` |
| `EQ_SECTION_COUNT_V1` | `EQ_SECTION_COUNT` | `crates/miso-engine-parametric-eq/src/lib.rs` |
| `EqBandDescriptorV1` | `EqBandDescriptor` | `crates/miso-engine-parametric-eq/src/lib.rs` |
| `EqBandKindV1` | `EqBandKind` | `crates/miso-engine-parametric-eq/src/lib.rs` |
| `EqSvfWordsV1` | `EqSvfWords` | `crates/miso-engine-parametric-eq/src/lib.rs` |
| `PARAMETRIC_EQ_DESCRIPTOR_V1` | `PARAMETRIC_EQ_DESCRIPTOR` | `crates/miso-engine-parametric-eq/src/lib.rs` |
| `design_svf_v1` | `design_svf` | `crates/miso-engine-parametric-eq/src/lib.rs` |

#### `crates/miso-engine-protocol` — 1

| old | new | declared in |
|---|---|---|
| `SessionEditV1` | `SessionEdit` | `crates/miso-engine-protocol/src/model.rs` |

#### `crates/miso-engine-rack` — 2

| old | new | declared in |
|---|---|---|
| `RackLocationV1` | `RackLocation` | `crates/miso-engine-rack/src/lib.rs` |
| `RackProgramV1` | `RackProgram` | `crates/miso-engine-rack/src/lib.rs` |

#### `crates/miso-engine-rack-compiler` — 2

| old | new | declared in |
|---|---|---|
| `CohortPoolClassV1` | `CohortPoolClass` | `crates/miso-engine-rack-compiler/src/lib.rs` |
| `program_v1` | `program` | `crates/miso-engine-rack-compiler/src/lib.rs` |

#### `crates/miso-engine-session` — 3

| old | new | declared in |
|---|---|---|
| `BUILTIN_AUTOMATION_EFFECT_ID_V1` | `BUILTIN_AUTOMATION_EFFECT_ID` | `crates/miso-engine-session/src/validate.rs` |
| `BUILTIN_AUTOMATION_TARGETS_V1` | `BUILTIN_AUTOMATION_TARGETS` | `crates/miso-engine-session/src/validate.rs` |
| `SessionTomlV1` | `SessionToml` | `crates/miso-engine-session/src/model.rs` |

#### `crates/miso-engine-soft-clip` — 3

| old | new | declared in |
|---|---|---|
| `PreparedSoftClipBankKernelV1` | `PreparedSoftClipBankKernel` | `crates/miso-engine-soft-clip/src/lib.rs` |
| `SOFT_CLIP_DESCRIPTOR_V1` | `SOFT_CLIP_DESCRIPTOR` | `crates/miso-engine-soft-clip/src/lib.rs` |
| `SOFT_CLIP_PARAMETERS_V1` | `SOFT_CLIP_PARAMETERS` | `crates/miso-engine-soft-clip/src/lib.rs` |

#### `crates/miso-engine-transient-shaper` — 3

| old | new | declared in |
|---|---|---|
| `TRANSIENT_SHAPER_COEFFICIENT_BITS_V1` | `TRANSIENT_SHAPER_COEFFICIENT_BITS` | `crates/miso-engine-transient-shaper/src/lib.rs` |
| `TRANSIENT_SHAPER_DESCRIPTOR_V1` | `TRANSIENT_SHAPER_DESCRIPTOR` | `crates/miso-engine-transient-shaper/src/lib.rs` |
| `TRANSIENT_SHAPER_PARAMETERS_V1` | `TRANSIENT_SHAPER_PARAMETERS` | `crates/miso-engine-transient-shaper/src/lib.rs` |

#### `crates/miso-engine-true-peak-limiter` — 3

| old | new | declared in |
|---|---|---|
| `TRUE_PEAK_LIMITER_DESCRIPTOR_V1` | `TRUE_PEAK_LIMITER_DESCRIPTOR` | `crates/miso-engine-true-peak-limiter/src/lib.rs` |
| `TRUE_PEAK_LIMITER_OBSERVATIONS_V1` | `TRUE_PEAK_LIMITER_OBSERVATIONS` | `crates/miso-engine-true-peak-limiter/src/lib.rs` |
| `TRUE_PEAK_LIMITER_PARAMETERS_V1` | `TRUE_PEAK_LIMITER_PARAMETERS` | `crates/miso-engine-true-peak-limiter/src/lib.rs` |

#### `hosts/miso-engine-host-web` — 28

| old | new | declared in |
|---|---|---|
| `CreateMisoAudioWorkletHostOptionsV1` | `CreateMisoAudioWorkletHostOptions` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoAckV1` | `MisoAck` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoAudioWorkletHostV1` | `MisoAudioWorkletHost` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.js` |
| `MisoCommandAckV1` | `MisoCommandAck` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoCommandKindV1` | `MisoCommandKind` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoCommandReasonV1` | `MisoCommandReason` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoCommandRequestV1` | `MisoCommandRequest` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoCommandV1` | `MisoCommand` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoErrorV1` | `MisoError` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoMeterFrameV1` | `MisoMeterFrame` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoObservationAckV1` | `MisoObservationAck` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoObservationBindingV1` | `MisoObservationBinding` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoObservationRequestV1` | `MisoObservationRequest` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoObservationSubscriptionV1` | `MisoObservationSubscription` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoSeekRequestV1` | `MisoSeekRequest` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoSessionMapV1` | `MisoSessionMap` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoSourceRequestV1` | `MisoSourceRequest` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoStatusV1` | `MisoStatus` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoTelemetryFrameV1` | `MisoTelemetryFrame` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoUnsupportedBrowserV1` | `MisoUnsupportedBrowser` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoWebBackendV1` | `MisoWebBackend` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoWebPrepareLimitsV1` | `MisoWebPrepareLimits` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `MisoWebResourceReportV1` | `MisoWebResourceReport` | `hosts/miso-engine-host-web/web/miso-engine-v2-audio-worklet-host.d.ts` |
| `WebCommandReportV1` | `WebCommandReport` | `hosts/miso-engine-host-web/src/lib.rs` |
| `WebMeterHeaderV1` | `WebMeterHeader` | `hosts/miso-engine-host-web/src/lib.rs` |
| `WebPrepareConfigV1` | `WebPrepareConfig` | `hosts/miso-engine-host-web/src/lib.rs` |
| `WebResourceReportV1` | `WebResourceReport` | `hosts/miso-engine-host-web/src/lib.rs` |
| `WebStatusV1` | `WebStatus` | `hosts/miso-engine-host-web/src/lib.rs` |

#### `tools/miso-engine-audit` — 189

| old | new | declared in |
|---|---|---|
| `BENCHMARK_KINDS_V1` | `BENCHMARK_KINDS` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `BENCHMARK_RATES_V1` | `BENCHMARK_RATES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `BOXED_INPUT_ENTRY_BYTES_V1` | `BOXED_INPUT_ENTRY_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `BOXED_STAGE_ENTRY_BYTES_V1` | `BOXED_STAGE_ENTRY_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `BOXED_STR_BYTES_V1` | `BOXED_STR_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `BOXED_TAIL_ENTRY_BYTES_V1` | `BOXED_TAIL_ENTRY_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `BenchmarkInputV1` | `BenchmarkInput` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `BenchmarkKindV1` | `BenchmarkKind` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `CASE_COUNT_V1` | `CASE_COUNT` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `CONSUMER_SEAL_BYTES_V1` | `CONSUMER_SEAL_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `CorpusClassV1` | `CorpusClass` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `CorpusCorruptionResultV1` | `CorpusCorruptionResult` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `CorpusMutationV1` | `CorpusMutation` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `CorpusRejectionV1` | `CorpusRejection` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `DiagnosticRecordV1` | `DiagnosticRecord` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `FADER_PROCESSOR_BYTES_V1` | `FADER_PROCESSOR_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `FUNCTIONAL_CASES_V1` | `FUNCTIONAL_CASES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `FixtureManifestEntryV1` | `FixtureManifestEntry` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `FixtureManifestV1` | `FixtureManifest` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `FixturePathClassV1` | `FixturePathClass` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `FunctionalCaseV1` | `FunctionalCase` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `GRAPH_NODE_BINDING_BYTES_V1` | `GRAPH_NODE_BINDING_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `GRAPH_OBSERVER_BINDING_BYTES_V1` | `GRAPH_OBSERVER_BINDING_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `GRAPH_TAP_PCM_SHA256_V1` | `GRAPH_TAP_PCM_SHA256` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `GraphFixtureExpectedV1` | `GraphFixtureExpected` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `INPUT_PROCESSOR_BYTES_V1` | `INPUT_PROCESSOR_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `IndependentResponseMeasurementV1` | `IndependentResponseMeasurement` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `IndependentResponseProcessorV1` | `IndependentResponseProcessor` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `JsonParserV1` | `JsonParser` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `JsonValueV1` | `JsonValue` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `MATRIX_PROCESSOR_BYTES_V1` | `MATRIX_PROCESSOR_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `METADATA_V1` | `METADATA` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `METER_CONSUMER_BYTES_V1` | `METER_CONSUMER_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `METER_OBSERVER_BYTES_V1` | `METER_OBSERVER_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `METER_QUEUE_HEADER_BYTES_V1` | `METER_QUEUE_HEADER_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `METER_REQUEST_SEAL_BYTES_V1` | `METER_REQUEST_SEAL_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `METER_SNAPSHOT_BYTES_V1` | `METER_SNAPSHOT_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `MatrixRampV1` | `MatrixRamp` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `MeterRecordV1` | `MeterRecord` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `MeterSnapshotV1` | `MeterSnapshot` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `OBSERVER_SEAL_BYTES_V1` | `OBSERVER_SEAL_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `PCM_INPUT_LEFT_V1` | `PCM_INPUT_LEFT` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `PCM_INPUT_RIGHT_V1` | `PCM_INPUT_RIGHT` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `PCM_PAYLOAD_COUNT_V1` | `PCM_PAYLOAD_COUNT` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `RESPONSE_ATTENUATED_TOTAL_LIMIT_DB_V1` | `RESPONSE_ATTENUATED_TOTAL_LIMIT_DB` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `RESPONSE_CASE_COUNT_V1` | `RESPONSE_CASE_COUNT` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `RESPONSE_CAST_STATE_TOLERANCE_DB_V1` | `RESPONSE_CAST_STATE_TOLERANCE_DB` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `RESPONSE_FUNDAMENTAL_TOLERANCE_DB_V1` | `RESPONSE_FUNDAMENTAL_TOLERANCE_DB` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `RESPONSE_IMPULSE_DFT_TOLERANCE_DB_V1` | `RESPONSE_IMPULSE_DFT_TOLERANCE_DB` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `RESPONSE_RBJ_SERIALIZATION_TOLERANCE_DB_V1` | `RESPONSE_RBJ_SERIALIZATION_TOLERANCE_DB` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `RESPONSE_RESIDUAL_LIMIT_DB_V1` | `RESPONSE_RESIDUAL_LIMIT_DB` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `RESPONSE_ROW_COUNT_V1` | `RESPONSE_ROW_COUNT` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ReferenceMeterConfigV1` | `ReferenceMeterConfig` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ReferenceMeterLaneV1` | `ReferenceMeterLane` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ReferenceMeterV1` | `ReferenceMeter` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ResetFixtureV1` | `ResetFixture` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ResourceRecordV1` | `ResourceRecord` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ResponseCaseV1` | `ResponseCase` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ResponseCoordinateV1` | `ResponseCoordinate` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ResponseCsvRowV1` | `ResponseCsvRow` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ResponseInvariantCoordinateV1` | `ResponseInvariantCoordinate` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ResponseMeasurementWordsV1` | `ResponseMeasurementWords` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `ResponseSectionV1` | `ResponseSection` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `STRIP_PREPARATION_BYTES_V1` | `STRIP_PREPARATION_BYTES` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `VerifiedCasesV1` | `VerifiedCases` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `apply_issue064_mutation_v1` | `apply_issue064_mutation` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `benchmark_field_from_pairs_v1` | `benchmark_field_from_pairs` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `benchmark_field_pair_v1` | `benchmark_field_pair` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `benchmark_field_v1` | `benchmark_field` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `benchmark_path_v1` | `benchmark_path` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `benchmark_text_mutation_v1` | `benchmark_text_mutation` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `canonical_benchmark_input_v1` | `canonical_benchmark_input` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `canonical_diagnostic_v1` | `canonical_diagnostic` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `canonical_meter_record_v1` | `canonical_meter_record` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `canonical_meter_snapshot_v1` | `canonical_meter_snapshot` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `canonical_resource_v1` | `canonical_resource` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `check_fixture_root_v1` | `check_fixture_root` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `check_read_only_fixture_root_v1` | `check_read_only_fixture_root` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `classify_fixture_path_v1` | `classify_fixture_path` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `clear_reference_meter_interval_v1` | `clear_reference_meter_interval` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `copied_checked_in_fixture_root_v1` | `copied_checked_in_fixture_root` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `decimal_digits_v1` | `decimal_digits` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `delay_three_v1` | `delay_three` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `derive_resource_record_v1` | `derive_resource_record` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `exact_json_keys_v1` | `exact_json_keys` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `expected_benchmark_fields_v1` | `expected_benchmark_fields` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `expected_diagnostics_v1` | `expected_diagnostics` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `expected_graph_meter_records_v1` | `expected_graph_meter_records` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `expected_resource_records_v1` | `expected_resource_records` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `expected_response_cases_v1` | `expected_response_cases` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `expected_response_coordinates_v1` | `expected_response_coordinates` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `expected_response_ids_v1` | `expected_response_ids` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `expected_window_meter_records_v1` | `expected_window_meter_records` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `fixture_source_frame_v1` | `fixture_source_frame` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `fixture_tree_hash_v1` | `fixture_tree_hash` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `frozen_cascade_probes_v1` | `frozen_cascade_probes` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `frozen_single_section_probes_v1` | `frozen_single_section_probes` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `functional_payload_ownership_v1` | `functional_payload_ownership` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `graph_fixture_expected_v1` | `graph_fixture_expected` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `graph_meter_snapshot_v1` | `graph_meter_snapshot` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `graph_tap_artifact_v1` | `graph_tap_artifact` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `identity_input_stage_v1` | `identity_input_stage` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `identity_section_v1` | `identity_section` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `independent_db_gain_v1` | `independent_db_gain` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `independent_rbj_magnitude_db_v1` | `independent_rbj_magnitude_db` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `independent_response_measurement_v1` | `independent_response_measurement` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `independent_sustained_metrics_v1` | `independent_sustained_metrics` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `is_coherent_measurement_probe_v1` | `is_coherent_measurement_probe` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `issue064_rejection_identity_v1` | `issue064_rejection_identity` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `issue064_semantic_rejection_v1` | `issue064_semantic_rejection` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `json_object_hex_v1` | `json_object_hex` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `json_object_string_v1` | `json_object_string` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `json_object_u64_v1` | `json_object_u64` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `json_object_v1` | `json_object` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `json_string_field_v1` | `json_string_field` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `json_u32_hex_field_v1` | `json_u32_hex_field` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `json_u64_field_v1` | `json_u64_field` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `json_u64_hex_field_v1` | `json_u64_hex_field` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `matrix_outputs_v1` | `matrix_outputs` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `matrix_outputs_v1_at` | `matrix_outputs_at` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `matrix_pcm_words_v1` | `matrix_pcm_words` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `matrix_ramp_outputs_v1` | `matrix_ramp_outputs` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `matrix_retarget_outputs_v1` | `matrix_retarget_outputs` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `mutate_issue064_semantic_hole_v1` | `mutate_issue064_semantic_hole` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_benchmark_input_v1` | `parse_benchmark_input` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_canonical_diagnostics_v1` | `parse_canonical_diagnostics` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_canonical_meter_records_v1` | `parse_canonical_meter_records` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_canonical_response_f64_v1` | `parse_canonical_response_f64` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_canonical_response_u32_v1` | `parse_canonical_response_u32` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_canonical_response_u64_v1` | `parse_canonical_response_u64` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_case_f64_17_v1` | `parse_case_f64_17` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_case_string_field_v1` | `parse_case_string_field` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_case_u32_field_v1` | `parse_case_u32_field` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_functional_case_v1` | `parse_functional_case` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_manifest_v1` | `parse_manifest` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_meter_record_v1` | `parse_meter_record` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_meter_snapshot_v1` | `parse_meter_snapshot` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_resource_record_v1` | `parse_resource_record` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_response_case_v1` | `parse_response_case` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_response_csv_row_v1` | `parse_response_csv_row` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_response_f64_v1` | `parse_response_f64` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `parse_response_section_v1` | `parse_response_section` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `pcm_words_v1` | `pcm_words` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `quoted_toml_string_v1` | `quoted_toml_string` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `read_pcm_words_v1` | `read_pcm_words` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `reference_meter_lane_v1` | `reference_meter_lane` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `reference_meter_observe_lane_v1` | `reference_meter_observe_lane` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `reference_sanitize_f32_v1` | `reference_sanitize_f32` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `refresh_manifest_entry_v1` | `refresh_manifest_entry` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `reject_manifest_valid_graph_tap_mutation_v1` | `reject_manifest_valid_graph_tap_mutation` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `render_reset_fixture_v1` | `render_reset_fixture` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `resource_maximum_track_identity_bytes_v1` | `resource_maximum_track_identity_bytes` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `resource_product_v1` | `resource_product` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `resource_sum_v1` | `resource_sum` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `resource_track_identity_bytes_v1` | `resource_track_identity_bytes` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `response_coordinate_v1` | `response_coordinate` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `response_invariant_coordinate_v1` | `response_invariant_coordinate` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `response_measurement_words_v1` | `response_measurement_words` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `retained_tpt_filter_chain_v1` | `retained_tpt_filter_chain` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `retained_tpt_outputs_v1` | `retained_tpt_outputs` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `sort_and_deduplicate_f64_v1` | `sort_and_deduplicate_f64` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `source_segment_v1` | `source_segment` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `validate_resource_record_v1` | `validate_resource_record` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_benchmark_inputs_v1` | `verify_benchmark_inputs` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_cases_v1` | `verify_cases` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_diagnostics_v1` | `verify_diagnostics` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_filter_pcm_semantics_v1` | `verify_filter_pcm_semantics` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_functional_cases_v1` | `verify_functional_cases` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_functional_fixture_completeness_v1` | `verify_functional_fixture_completeness` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_graph_lane_summary_v1` | `verify_graph_lane_summary` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_graph_tap_output_relation_v1` | `verify_graph_tap_output_relation` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_graph_tap_pdc_relation_v1` | `verify_graph_tap_pdc_relation` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_graph_tap_pdc_v1` | `verify_graph_tap_pdc` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_jsonl_payloads_v1` | `verify_jsonl_payloads` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_manifest_bytes_v1` | `verify_manifest_bytes` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_matrix_pcm_semantics_v1` | `verify_matrix_pcm_semantics` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_metadata_v1` | `verify_metadata` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_meter_corpus_v1` | `verify_meter_corpus` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_path_class_coverage_v1` | `verify_path_class_coverage` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_pcm_semantics_v1` | `verify_pcm_semantics` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_pcm_words_v1` | `verify_pcm_words` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_pinned_native_resource_abi_v1` | `verify_pinned_native_resource_abi` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_reference_oracle_v1` | `verify_reference_oracle` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_resources_v1` | `verify_resources` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_response_case_csv_tuples_v1` | `verify_response_case_csv_tuples` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_response_cases_v1` | `verify_response_cases` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_response_grid_v1` | `verify_response_grid` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_response_oracle_tolerances_v1` | `verify_response_oracle_tolerances` | `tools/miso-engine-audit/src/fixture_builtins.rs` |
| `verify_response_partition_equality_v1` | `verify_response_partition_equality` | `tools/miso-engine-audit/src/fixture_builtins.rs` |

## Pinned artifacts that embed a class-1 name

Nothing here is a pin **move**: each row is a spelling that a gate, oracle or mutation row asserts
by name, so it changes in the same commit as the code it names, and its identity, threshold and
expected value are untouched.

| pinned artifact | what it pins | old -> new |
|---|---|---|
| `scripts/check-command-kind-vocabulary.py` | the `.d.ts` enum it parses the kind vocabulary out of | `MisoCommandKindV1` -> `MisoCommandKind` |
| `scripts/check-command-reason-vocabulary.py` | the reason enum and the four `observe()` shapes | `MisoCommandReasonV1`, `MisoObservationSubscriptionV1`, `MisoObservationRequestV1`, `MisoObservationBindingV1`, `MisoObservationAckV1` -> unsuffixed |
| `scripts/check-session-map-shape.py` | the `.d.ts` session-map interface | `MisoSessionMapV1` -> `MisoSessionMap` (its `MisoSessionSourceV1` mutation payload is class 2 and unchanged) |
| `scripts/check-browser-expected-resources.py` | the Rust config struct it re-derives resources from | `WebPrepareConfigV1` -> `WebPrepareConfig` |
| `scripts/check-wasm-realtime-atomics.sh` | the observation slot type in the atomics scan | `ObservationSlotV1` -> `ObservationSlot` |
| `scripts/check-effect-interchange-benchmark-108.sh` | the migration quality table it extracts | `QualityDescriptorV1` -> `QualityDescriptor` |
| `scripts/check-effect-interchange-qualification.sh` | six interchange entry points | see class-1 table |
| `scripts/check-effect-package-v1.sh` | five package entry points | see class-1 table |
| `scripts/check-effect-state-migration-v1.sh` | eight migration entry points | see class-1 table |
| `scripts/build-web-audioworklet.sh` | the registry launcher the worklet build links | `launch_native_effect_registry_v1` -> `launch_native_effect_registry` |
| `scripts/run-console-benchmark.sh` | the rack-location enum in the workload | `RackLocationV1` -> `RackLocation` |
| `scripts/derive-intended-console-fixture.py` | the limiter descriptor it reads defaults from | `TRUE_PEAK_LIMITER_DESCRIPTOR_V1` -> `TRUE_PEAK_LIMITER_DESCRIPTOR` |
| `scripts/derive-mono-console-fixture.py` | the seam enumeration | `SeamSideV1` -> `SeamSide` |
| `MUTATIONS.md` rows in `builtins/`, `builtins-compiler/`, `effect-package/`, `graph/`, `graph-compiler/`, `host-core/`, `parametric-eq/`, `rack/` tests and `hosts/miso-engine-host-web/` | the mutated symbol named in the row | 14 spellings, all in the class-1 table |
| in-source `--self-test` mutation strings (`fixture_builtins.rs`, `state_vectors.rs`, `migration.rs`, `migration_terminal.rs`) | source text the self-test rewrites to prove the gate discriminates | same rename applied to both the source and the pattern |

**No sweep row was renamed.** `scripts/sweep.sh` is byte-identical; all 93 row names, and every
`scripts/*-v1.{sh,py}` file name they refer to, are class 2.

## The two proof greps

Completeness -- **no class-1 spelling survives in the live tree** (frozen trees excluded by the
scope ruling above):

```
git grep -nI -E '\b(MisoCommandKindV1|WebPrepareConfigV1|SessionEditV1|EffectDescriptorV1|ChannelSymmetryWitnessV1|TrackInputRecordV1|SymmetryEventV1|session_structural_symmetry_v1|BUILTIN_PARAMETER_DESCRIPTORS_V1|BuiltinInputBankV1|FaderMuteRampBuiltinsV1)\b' \
  -- crates tools hosts scripts docs fuzz AGENTS.md
```

...and its exhaustive form, which is what the verifier should actually run: every one of the 420
class-1 spellings, alternated, over the same paths. Both must print **nothing**.

Non-regression -- **every class-2 spelling is still there, at its original count**:

```
git grep -cI -E 'miso\.(command|observe|ack|error|meter|meters|status|sessionmap|source|seek|ready|dispose|telemetry|unsupported)\.v1' -- crates tools hosts scripts docs
git grep -nI -E '\b(miso_engine_web_v1_[a-z_]+|MISO_ENGINE_EFFECT_[A-Z0-9_]+_V1|SESSION_SCHEMA_VERSION_V1|PROTOCOL_MAJOR_V1|PROTOCOL_MINOR_V1|miso_engine_graph_v1|MisoSessionSourceV1|KernelBackendV1|PreparedDeltaBankKernelV1)\b' \
  -- crates tools hosts scripts docs | wc -l
```

Digest identity end to end: `git diff --stat` names no file under `fixtures/` or `artifacts/`, and
the derive-and-compare gates (`check-intended-console-fixture.sh`, `check-mono-console-fixture.sh`,
`check-builtins-fixtures.sh`, `check-effect-descriptor-v1.sh`, `check-effect-package-v1.sh`,
`check-effect-state-migration-v1.sh`) re-derive their fixtures from the renamed source and compare
byte-for-byte against the unmoved checked-in bytes.
