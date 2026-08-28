//! Build-time parameter-metadata codegen for the browser console (issue #137 D4).
//!
//! # Why this exists
//!
//! The app drives the engine through numeric IDs: a track index, a rack, an effect index and a
//! parameter ID. Numbers are what the command path takes, and deliberately so -- there is no
//! string on the hot path. But numbers are useless to a user interface without names, units,
//! ranges, defaults and enumerations, and the one place that knowledge lives is the effect
//! contract's descriptors. This tool emits it, once, at build time, next to the Wasm artifact, so
//! the app never introspects the module for metadata and never has to keep a hand-written copy in
//! step with the engine.
//!
//! # Completeness is structural
//!
//! The effect list comes from `launch_native_effect_registry()` through
//! `NativeEffectRegistry::descriptors`, so "an effect in the registry is missing from the output"
//! is not a rule anybody has to remember: there is no second list to fall out of step with. The
//! `--check` mode regenerates and compares byte for byte, which is what makes the emitted file a
//! build artifact rather than a document.
//!
//! # `liveUpdatable`
//!
//! Every parameter carries whether the live-console command path can actually move it. Issue #140
//! made that true of every builtin block target and every automatable effect parameter: a builtin
//! row is live exactly when its declared update rate is `blockTarget`, and an effect row is live
//! exactly when its descriptor declares it automatable. A caller that reads this file never has to
//! discover either at runtime. See the browser ABI's `.d.ts` header for the full statement.
//!
//! # `commandKinds` is the whole vocabulary, and `plane` says what each kind moves
//!
//! Every kind the wire decodes is a row here -- all eleven of them. Before the kind-vocabulary
//! gate this table stopped at `effectBypass` while the Rust constants, the host JS `COMMAND_KINDS`
//! set and the `.d.ts` enum all carried eight, and nothing noticed: an app reading its vocabulary
//! from this file could not learn that `observeSubscribe`/`observeUnsubscribe` exist.
//!
//! The two observation kinds are not DSP kinds, so each row carries `plane`. `applied` keeps its
//! issue #140 meaning -- the ABI applies this kind rather than declaring and refusing it, which is
//! true of all eleven -- and `plane` distinguishes the nine that move state the render thread reads
//! (`"render"`) from the two that bind or unbind an entry in the `miso.observe.v1` subscription
//! map and change nothing rendered (`"observation"`).
//!
//! `solo` (issue #210 phase 1) is a `"render"` kind with no row in `builtins`, and deliberately:
//! it moves what the render thread reads -- it composes into the fader section's mute -- but it is
//! console/monitor state rather than a strip DSP parameter, so it has no parameter descriptor, no
//! domain table and no session key to be automated from.
//!
//! `trimDb` and `polarityInvert` (issue #210 phase 3) are the opposite case and the ordinary one:
//! both are `"render"` kinds *and* strip DSP parameters, so each has a `builtins` row, and those
//! two rows flipped to `"blockTarget"` with `"smoothing": "linearNUpdates"` in the same change --
//! which is what makes their `liveUpdatable` flag `true` here. A reader that trusted the old
//! `false` would have concluded there was no write path; there now is.
//!
//! # Issue #127 (named nudge sizes)
//!
//! Each parameter carries `"nudge": null`. When #127 lands its ladder on
//! `ParameterDescriptor`, that slot becomes an object and nothing else in this schema moves --
//! which is the whole reason it is a declared null rather than an absent key.

use std::io::Write as _;
use std::path::{Path, PathBuf};

use miso_engine_bench_support::json::escape;
use miso_engine_builtins::{
    BUILTIN_PARAMETER_DESCRIPTORS_V1, BuiltinParameterDescriptorV1, BuiltinParameterDomain,
    BuiltinParameterMapping, BuiltinParameterReset, BuiltinParameterScope,
    BuiltinParameterUpdateRate, BuiltinSmoothingPolicy, builtin_filter_cutoff_maximum_hz_v1,
};
use miso_engine_effect_compiler::launch_native_effect_registry;
use miso_engine_effect_contract::{
    AutomationRate, EffectDescriptor, ObservationCadenceV1, ObservationChannelsV1,
    ObservationCostV1, ObservationDescriptor, ObservationFoldV1, ObservationKindV1,
    ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, ParameterMapping,
    ParameterUnit, SmoothingRule,
};
use miso_engine_host_web::{
    ABI_VERSION, COMMAND_EFFECT_BYPASS, COMMAND_EFFECT_PARAM, COMMAND_FADER_DB, COMMAND_MATRIX,
    COMMAND_MUTE, COMMAND_OBSERVE_SUBSCRIBE, COMMAND_OBSERVE_UNSUBSCRIBE, COMMAND_PAN,
    COMMAND_POLARITY_INVERT, COMMAND_REASON_BACKPRESSURE, COMMAND_REASON_DOMAIN,
    COMMAND_REASON_MALFORMED, COMMAND_REASON_NONE, COMMAND_REASON_OBSERVATION_UNBOUND,
    COMMAND_REASON_UNKNOWN_EFFECT, COMMAND_REASON_UNKNOWN_PARAMETER, COMMAND_REASON_UNKNOWN_RACK,
    COMMAND_REASON_UNKNOWN_TAP, COMMAND_REASON_UNKNOWN_TRACK, COMMAND_REASON_UNSUPPORTED_KIND,
    COMMAND_REASON_WRONG_STATE, COMMAND_RECORD_BYTES, COMMAND_SOLO, COMMAND_TRIM_DB,
    MAXIMUM_COMMAND_RECORDS,
};

/// The emitted file name, shipped beside the Wasm artifact.
pub const OUTPUT_NAME: &str = "miso-engine-v2-parameter-metadata.json";
/// The schema tag every consumer must check before reading a field.
pub const SCHEMA: &str = "miso.web.parameter-metadata.v1";
/// The launch sample rates a rate-keyed builtin domain is reported for.
pub const LAUNCH_RATES_HZ: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
/// The plane a `commandKinds` row applies on: DSP state the render thread reads.
pub const PLANE_RENDER: &str = "render";
/// The plane a `commandKinds` row applies on: the `miso.observe.v1` subscription map.
///
/// Kinds 7 and 8 are applied -- `admit_commands` binds or unbinds the tap and acknowledges
/// `COMMAND_REASON_NONE` -- but they move nothing the render thread reads. `applied` therefore
/// stays `true` for every kind (issue #140: nothing in this ABI is declared-and-refused), and
/// `plane` is what tells a consumer which of the two things "applied" means for that kind.
pub const PLANE_OBSERVATION: &str = "observation";

fn usage() -> ! {
    eprintln!(
        "usage: miso_engine_parameter_metadata --write DIRECTORY | --check DIRECTORY | --print"
    );
    std::process::exit(2)
}

fn output_path(directory: &Path) -> PathBuf {
    if !directory.is_dir() {
        eprintln!("{} is not a directory", directory.display());
        std::process::exit(2);
    }
    directory.join(OUTPUT_NAME)
}

/// Render the whole document. Deterministic: registry order is `EffectId` order.
#[must_use]
pub fn render() -> String {
    let registry = launch_native_effect_registry().expect("launch effect registry");
    let mut out = String::with_capacity(1 << 16);
    out.push_str("{\n");
    out.push_str(&format!("  \"schema\": \"{SCHEMA}\",\n"));
    out.push_str(&format!("  \"abiVersion\": {ABI_VERSION},\n"));
    out.push_str(&format!(
        "  \"commandRecordBytes\": {COMMAND_RECORD_BYTES},\n"
    ));
    out.push_str(&format!(
        "  \"maximumCommandRecords\": {MAXIMUM_COMMAND_RECORDS},\n"
    ));
    out.push_str("  \"commandKinds\": [\n");
    let kinds = [
        (COMMAND_PAN, "pan", true, PLANE_RENDER),
        (COMMAND_MATRIX, "matrix", true, PLANE_RENDER),
        (COMMAND_FADER_DB, "faderDb", true, PLANE_RENDER),
        (COMMAND_MUTE, "mute", true, PLANE_RENDER),
        (COMMAND_EFFECT_PARAM, "effectParam", true, PLANE_RENDER),
        (COMMAND_EFFECT_BYPASS, "effectBypass", true, PLANE_RENDER),
        (
            COMMAND_OBSERVE_SUBSCRIBE,
            "observeSubscribe",
            true,
            PLANE_OBSERVATION,
        ),
        (
            COMMAND_OBSERVE_UNSUBSCRIBE,
            "observeUnsubscribe",
            true,
            PLANE_OBSERVATION,
        ),
        (COMMAND_SOLO, "solo", true, PLANE_RENDER),
        (COMMAND_TRIM_DB, "trimDb", true, PLANE_RENDER),
        (
            COMMAND_POLARITY_INVERT,
            "polarityInvert",
            true,
            PLANE_RENDER,
        ),
    ];
    for (index, (value, name, applied, plane)) in kinds.iter().enumerate() {
        out.push_str(&format!(
            "    {{ \"value\": {value}, \"name\": \"{name}\", \"applied\": {applied}, \
             \"plane\": \"{plane}\" }}{}\n",
            comma(index, kinds.len())
        ));
    }
    out.push_str("  ],\n");
    out.push_str("  \"commandReasons\": [\n");
    let reasons = [
        (COMMAND_REASON_NONE, "none"),
        (COMMAND_REASON_MALFORMED, "malformed"),
        (COMMAND_REASON_UNKNOWN_TRACK, "unknownTrack"),
        (COMMAND_REASON_UNKNOWN_RACK, "unknownRack"),
        (COMMAND_REASON_UNKNOWN_EFFECT, "unknownEffect"),
        (COMMAND_REASON_UNKNOWN_PARAMETER, "unknownParameter"),
        (COMMAND_REASON_DOMAIN, "domain"),
        (COMMAND_REASON_UNSUPPORTED_KIND, "unsupportedKind"),
        (COMMAND_REASON_BACKPRESSURE, "backpressure"),
        (COMMAND_REASON_WRONG_STATE, "wrongState"),
        // Issue #143 added these two and #151 found the drift they caused: a vocabulary that stops
        // at `wrongState` tells every consumer that reasons 10 and 11 do not exist, and the only
        // reasons the observation path ever returns are exactly those two.
        // `scripts/check-command-reason-vocabulary.py` now holds this table, the Rust constants,
        // the host JS bound, the `.d.ts` enum and the schema gate's list to one another.
        (COMMAND_REASON_UNKNOWN_TAP, "unknownTap"),
        (COMMAND_REASON_OBSERVATION_UNBOUND, "observationUnbound"),
    ];
    for (index, (value, name)) in reasons.iter().enumerate() {
        out.push_str(&format!(
            "    {{ \"value\": {value}, \"name\": \"{name}\" }}{}\n",
            comma(index, reasons.len())
        ));
    }
    out.push_str("  ],\n");
    // Issue #143 D1: the observation vocabularies, so a consumer resolves a tap's raw `u32`s the
    // same way it resolves a parameter's -- from this document, never from a hand-written table.
    out.push_str("  \"observationVocabularies\": {\n");
    let vocabularies: [(&str, &[(u32, &str)]); 5] = [
        (
            "kinds",
            &[(ObservationKindV1::GainReductionDb as u32, "gainReductionDb")],
        ),
        (
            "costs",
            &[
                (ObservationCostV1::Resident as u32, "resident"),
                (ObservationCostV1::Computed as u32, "computed"),
            ],
        ),
        (
            "cadences",
            &[
                (ObservationCadenceV1::PerBlock as u32, "perBlock"),
                (ObservationCadenceV1::PerWindow as u32, "perWindow"),
            ],
        ),
        (
            "folds",
            &[
                (ObservationFoldV1::Latest as u32, "latest"),
                (ObservationFoldV1::PeakMagnitude as u32, "peakMagnitude"),
            ],
        ),
        (
            "channels",
            &[
                (ObservationChannelsV1::Shared as u32, "shared"),
                (ObservationChannelsV1::PerLane as u32, "perLane"),
            ],
        ),
    ];
    for (index, (name, rows)) in vocabularies.iter().enumerate() {
        out.push_str(&format!("    \"{name}\": ["));
        for (row, (value, label)) in rows.iter().enumerate() {
            out.push_str(&format!(
                "{{ \"value\": {value}, \"name\": \"{label}\" }}{}",
                if row + 1 == rows.len() { "" } else { ", " }
            ));
        }
        out.push_str(&format!("]{}\n", comma(index, vocabularies.len())));
    }
    out.push_str("  },\n");
    out.push_str("  \"builtins\": {\n    \"parameters\": [\n");
    let builtins = BUILTIN_PARAMETER_DESCRIPTORS_V1;
    for (index, parameter) in builtins.iter().enumerate() {
        out.push_str(&builtin_parameter(parameter));
        out.push_str(&format!("{}\n", comma(index, builtins.len())));
    }
    out.push_str("    ]\n  },\n");
    out.push_str("  \"effects\": [\n");
    let descriptors: Vec<&'static EffectDescriptor> = registry.descriptors().collect();
    assert_eq!(
        descriptors.len(),
        registry.len(),
        "every registered effect is emitted"
    );
    for (index, descriptor) in descriptors.iter().enumerate() {
        out.push_str(&effect(descriptor));
        out.push_str(&format!("{}\n", comma(index, descriptors.len())));
    }
    out.push_str("  ]\n}\n");
    out
}

fn comma(index: usize, total: usize) -> &'static str {
    if index + 1 == total { "" } else { "," }
}

/// Finite `f32` as JSON. Non-finite values cannot occur: every descriptor field is validated
/// finite by `validate_descriptor` before a factory may enter the registry.
fn number(value: f32) -> String {
    assert!(value.is_finite(), "descriptor values are finite");
    let text = format!("{value:?}");
    if text.contains('.') || text.contains('e') {
        text
    } else {
        format!("{text}.0")
    }
}

fn optional_number(value: Option<f32>) -> String {
    value.map_or_else(|| "null".to_owned(), number)
}

fn effect(descriptor: &EffectDescriptor) -> String {
    let mut out = String::new();
    out.push_str("    {\n");
    out.push_str(&format!(
        "      \"id\": \"{}\",\n",
        escape(descriptor.id.as_str())
    ));
    out.push_str(&format!(
        "      \"displayName\": \"{}\",\n",
        escape(descriptor.display_name)
    ));
    out.push_str(&format!(
        "      \"contractMajor\": {}, \"contractMinor\": {}, \"stateLayoutVersion\": {},\n",
        descriptor.contract_major, descriptor.contract_minor, descriptor.state_layout_version
    ));
    out.push_str("      \"parameters\": [\n");
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        out.push_str(&effect_parameter(parameter));
        out.push_str(&format!("{}\n", comma(index, descriptor.parameters.len())));
    }
    out.push_str("      ],\n");
    // Issue #143: never absent. An effect that declares no tap emits `[]`, so a consumer reads one
    // shape for every effect and "this build has no menu for that effect" is impossible to
    // confuse with "this document predates observation".
    out.push_str("      \"observations\": [");
    for (index, observation) in descriptor.observations.iter().enumerate() {
        out.push('\n');
        out.push_str(&effect_observation(observation));
        out.push_str(comma(index, descriptor.observations.len()));
    }
    if descriptor.observations.is_empty() {
        out.push_str("]\n    }");
    } else {
        out.push_str("\n      ]\n    }");
    }
    out
}

fn effect_observation(observation: &ObservationDescriptor) -> String {
    // `subscribable` is derived from the cost class exactly as `liveUpdatable` is derived from
    // `automatable`: a `Resident` tap is a copy out of state the block already wrote and the
    // subscribe path binds it; a `Computed` tap has no implementation in V1 and the subscribe path
    // answers `unsupportedKind`. The two statements are the same statement, which is why this is
    // derived rather than written down -- and why the schema gate refuses a computed tap that
    // claims to be subscribable.
    let subscribable = matches!(observation.cost, ObservationCostV1::Resident);
    format!(
        "        {{ \"id\": {}, \"name\": \"{}\", \"displayUnit\": \"{}\", \
\"kind\": {}, \"kindName\": \"{}\", \"unit\": {}, \"unitName\": \"{}\", \
\"cost\": {}, \"costName\": \"{}\", \"cadence\": {}, \"cadenceName\": \"{}\", \
\"fold\": {}, \"foldName\": \"{}\", \"channels\": {}, \"channelsName\": \"{}\", \
\"minimum\": {}, \"maximum\": {}, \"subscribable\": {} }}",
        observation.id.0,
        escape(observation.display_name),
        escape(observation.display_unit),
        observation.kind as u32,
        observation_kind_name(observation.kind),
        observation.unit as u32,
        unit_name(observation.unit),
        observation.cost as u32,
        observation_cost_name(observation.cost),
        observation.cadence as u32,
        observation_cadence_name(observation.cadence),
        observation.fold as u32,
        observation_fold_name(observation.fold),
        observation.channels as u32,
        observation_channels_name(observation.channels),
        number(observation.minimum),
        number(observation.maximum),
        subscribable,
    )
}

const fn observation_kind_name(kind: ObservationKindV1) -> &'static str {
    match kind {
        ObservationKindV1::GainReductionDb => "gainReductionDb",
    }
}

const fn observation_cost_name(cost: ObservationCostV1) -> &'static str {
    match cost {
        ObservationCostV1::Resident => "resident",
        ObservationCostV1::Computed => "computed",
    }
}

const fn observation_cadence_name(cadence: ObservationCadenceV1) -> &'static str {
    match cadence {
        ObservationCadenceV1::PerBlock => "perBlock",
        ObservationCadenceV1::PerWindow => "perWindow",
    }
}

const fn observation_fold_name(fold: ObservationFoldV1) -> &'static str {
    match fold {
        ObservationFoldV1::Latest => "latest",
        ObservationFoldV1::PeakMagnitude => "peakMagnitude",
    }
}

const fn observation_channels_name(channels: ObservationChannelsV1) -> &'static str {
    match channels {
        ObservationChannelsV1::Shared => "shared",
        ObservationChannelsV1::PerLane => "perLane",
    }
}

fn effect_parameter(parameter: &ParameterDescriptor) -> String {
    let mut out = String::new();
    out.push_str("        {\n");
    out.push_str(&format!("          \"id\": {},\n", parameter.id.0));
    out.push_str(&format!(
        "          \"name\": \"{}\",\n",
        escape(parameter.display_name)
    ));
    out.push_str(&format!(
        "          \"displayUnit\": \"{}\",\n",
        escape(parameter.display_unit)
    ));
    out.push_str(&format!(
        "          \"unit\": {}, \"unitName\": \"{}\",\n",
        parameter.unit as u32,
        unit_name(parameter.unit)
    ));
    out.push_str(&format!(
        "          \"domain\": {}, \"domainName\": \"{}\",\n",
        parameter.domain as u32,
        domain_name(parameter.domain)
    ));
    out.push_str(&format!(
        "          \"minimum\": {}, \"maximum\": {}, \"default\": {},\n",
        optional_number(parameter.minimum),
        optional_number(parameter.maximum),
        number(parameter.default_value)
    ));
    out.push_str(&format!(
        "          \"mapping\": {}, \"mappingName\": \"{}\",\n",
        parameter.mapping as u32,
        mapping_name(parameter.mapping)
    ));
    out.push_str(&format!(
        "          \"automationRate\": {}, \"automationRateName\": \"{}\",\n",
        parameter.automation_rate as u32,
        automation_rate_name(parameter.automation_rate)
    ));
    out.push_str(&format!(
        "          \"channelPolicy\": {}, \"channelPolicyName\": \"{}\",\n",
        parameter.channel_policy as u32,
        channel_policy_name(parameter.channel_policy)
    ));
    out.push_str(&format!(
        "          \"smoothing\": {}, \"smoothingName\": \"{}\", \"smoothingSamples\": {},\n",
        parameter.smoothing as u32,
        smoothing_name(parameter.smoothing),
        parameter.smoothing_samples
    ));
    out.push_str(&format!(
        "          \"readable\": {}, \"automatable\": {},\n",
        parameter.readable, parameter.automatable
    ));
    // Issue #140 A: the live-console command path now feeds an admitted parameter into the
    // running plan as a `PreparedAutomationSpan`, so a parameter is live exactly when its own
    // descriptor says it can be automated. The two statements are the same statement, which is
    // why this is derived from `automatable` rather than written down. A parameter that declares
    // `AutomationRate::None` has no span the effect would accept and stays `false`.
    out.push_str(&format!(
        "          \"liveUpdatable\": {},\n",
        parameter.automatable
    ));
    out.push_str("          \"enumChoices\": [");
    for (index, choice) in parameter.enum_choices.iter().enumerate() {
        out.push_str(&format!(
            "{{ \"value\": {}, \"label\": \"{}\" }}{}",
            number(choice.value),
            escape(choice.label),
            if index + 1 == parameter.enum_choices.len() {
                ""
            } else {
                ", "
            }
        ));
    }
    out.push_str("],\n");
    // Issue #127 slot. A declared null, not an absent key, so adding the ladder is additive.
    out.push_str("          \"nudge\": null\n        }");
    out
}

fn builtin_parameter(parameter: &BuiltinParameterDescriptorV1) -> String {
    // A rate-keyed cutoff has no single maximum: `builtin_filter_cutoff_maximum_hz_v1` gives one
    // per launch rate, so the row carries the exact `f32` for each rather than a number that would
    // be wrong at three of the four.
    let mut maximum_by_rate = String::from("null");
    let (minimum, maximum, domain_name) = match parameter.domain {
        BuiltinParameterDomain::BooleanExact => (None, None, "booleanExact"),
        BuiltinParameterDomain::FiniteInclusive { minimum, maximum } => {
            (Some(minimum), Some(maximum), "finiteInclusive")
        }
        BuiltinParameterDomain::DisabledOrRateKeyedHertzV1 { minimum_hz, .. } => {
            maximum_by_rate = format!(
                "{{ {} }}",
                LAUNCH_RATES_HZ
                    .iter()
                    .map(|rate| format!(
                        "\"{rate}\": {}",
                        optional_number(builtin_filter_cutoff_maximum_hz_v1(*rate))
                    ))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            (Some(minimum_hz), None, "disabledOrRateKeyedHertz")
        }
    };
    // `matrix_ll/lr/rl/rr` are the only builtin parameters the ABI declares as `BlockTarget`, and
    // they are exactly the ones the live-console command path applies. The two statements are the
    // same statement, which is why this is derived from the descriptor rather than written down.
    let live = matches!(
        parameter.update_rate,
        BuiltinParameterUpdateRate::BlockTarget
    );
    format!(
        "      {{ \"id\": {}, \"name\": \"{}\", \"scope\": \"{}\", \"mapping\": \"{}\", \
\"domain\": \"{}\", \"minimum\": {}, \"maximum\": {}, \"maximumByRate\": {}, \"default\": {}, \
\"updateRate\": \"{}\", \"smoothing\": \"{}\", \"reset\": \"{}\", \"disabledValue\": {}, \
\"liveUpdatable\": {}, \"nudge\": null }}",
        parameter.id,
        escape(parameter.name),
        match parameter.scope {
            BuiltinParameterScope::PerLane => "perLane",
            BuiltinParameterScope::MatrixShared => "matrixShared",
        },
        match parameter.mapping {
            BuiltinParameterMapping::Boolean => "boolean",
            BuiltinParameterMapping::DecibelAmplitude => "decibelAmplitude",
            BuiltinParameterMapping::Hertz => "hertz",
            BuiltinParameterMapping::Linear => "linear",
        },
        domain_name,
        optional_number(minimum),
        optional_number(maximum),
        maximum_by_rate,
        number(parameter.default),
        match parameter.update_rate {
            BuiltinParameterUpdateRate::PreparedOnly => "preparedOnly",
            BuiltinParameterUpdateRate::BlockTarget => "blockTarget",
        },
        match parameter.smoothing {
            BuiltinSmoothingPolicy::None => "none",
            BuiltinSmoothingPolicy::LinearNUpdates => "linearNUpdates",
        },
        match parameter.reset {
            BuiltinParameterReset::RestorePreparedValue => "restorePreparedValue",
            BuiltinParameterReset::KeepTargetResetCurrent => "keepTargetResetCurrent",
        },
        optional_number(parameter.disabled_value),
        live,
    )
}

const fn unit_name(unit: ParameterUnit) -> &'static str {
    match unit {
        ParameterUnit::Db => "db",
        ParameterUnit::Hz => "hz",
        ParameterUnit::Milliseconds => "milliseconds",
        ParameterUnit::Samples => "samples",
        ParameterUnit::Linear => "linear",
        ParameterUnit::Ratio => "ratio",
    }
}

const fn domain_name(domain: ParameterDomain) -> &'static str {
    match domain {
        ParameterDomain::Continuous => "continuous",
        ParameterDomain::Boolean => "boolean",
        ParameterDomain::Enumeration => "enumeration",
    }
}

const fn mapping_name(mapping: ParameterMapping) -> &'static str {
    match mapping {
        ParameterMapping::Linear => "linear",
        ParameterMapping::Logarithmic => "logarithmic",
        ParameterMapping::Exponential => "exponential",
        ParameterMapping::Stepped => "stepped",
    }
}

const fn automation_rate_name(rate: AutomationRate) -> &'static str {
    match rate {
        AutomationRate::Sample => "sample",
        AutomationRate::Block => "block",
        AutomationRate::None => "none",
    }
}

const fn channel_policy_name(policy: ParameterChannelPolicy) -> &'static str {
    match policy {
        ParameterChannelPolicy::Shared => "shared",
        ParameterChannelPolicy::PerLane => "perLane",
    }
}

const fn smoothing_name(rule: SmoothingRule) -> &'static str {
    match rule {
        SmoothingRule::None => "none",
        SmoothingRule::Linear => "linear",
        SmoothingRule::OnePole99 => "onePole99",
    }
}

/// Command-line entry point: `--write DIR`, `--check DIR` or `--print`.
pub fn run() {
    let mut arguments = std::env::args().skip(1);
    let mode = arguments.next().unwrap_or_else(|| usage());
    let document = render();
    match mode.as_str() {
        "--print" => {
            if arguments.next().is_some() {
                usage();
            }
            print!("{document}");
        }
        "--write" | "--check" => {
            let directory = PathBuf::from(arguments.next().unwrap_or_else(|| usage()));
            if arguments.next().is_some() {
                usage();
            }
            let path = output_path(&directory);
            if mode == "--write" {
                let mut file = std::fs::File::create(&path).unwrap_or_else(|error| {
                    eprintln!("cannot create {}: {error}", path.display());
                    std::process::exit(2)
                });
                file.write_all(document.as_bytes()).expect("write metadata");
                println!("wrote {}", path.display());
            } else {
                let existing = std::fs::read_to_string(&path).unwrap_or_else(|error| {
                    eprintln!("cannot read {}: {error}", path.display());
                    std::process::exit(1)
                });
                if existing != document {
                    eprintln!("{} is stale; regenerate with --write", path.display());
                    std::process::exit(1);
                }
                println!("{} is current", path.display());
            }
        }
        _ => usage(),
    }
}
