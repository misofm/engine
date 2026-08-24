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
//! The effect list comes from `launch_native_effect_registry_v1()` through
//! `NativeEffectRegistry::descriptors`, so "an effect in the registry is missing from the output"
//! is not a rule anybody has to remember: there is no second list to fall out of step with. The
//! `--check` mode regenerates and compares byte for byte, which is what makes the emitted file a
//! build artifact rather than a document.
//!
//! # `liveUpdatable`
//!
//! Every parameter carries whether the live-console command path can actually move it. That is not
//! a convenience: the engine has no general post-preparation write path, so most parameters are
//! `false`, and a caller that reads this file never has to discover that at runtime. See the
//! browser ABI's `.d.ts` header for the full statement of which surfaces move and why.
//!
//! # Issue #127 (named nudge sizes)
//!
//! Each parameter carries `"nudge": null`. When #127 lands its ladder on
//! `ParameterDescriptorV1`, that slot becomes an object and nothing else in this schema moves --
//! which is the whole reason it is a declared null rather than an absent key.

use std::io::Write as _;
use std::path::{Path, PathBuf};

use miso_engine_bench_support::json::escape;
use miso_engine_builtins::{
    BUILTIN_PARAMETER_DESCRIPTORS_V1, BuiltinParameterDescriptorV1, BuiltinParameterDomain,
    BuiltinParameterMapping, BuiltinParameterReset, BuiltinParameterScope,
    BuiltinParameterUpdateRate, BuiltinSmoothingPolicy, builtin_filter_cutoff_maximum_hz_v1,
};
use miso_engine_effect_compiler::launch_native_effect_registry_v1;
use miso_engine_effect_contract::{
    AutomationRate, EffectDescriptorV1, ParameterChannelPolicy, ParameterDescriptorV1,
    ParameterDomain, ParameterMapping, ParameterUnit, SmoothingRule,
};
use miso_engine_host_web::{
    ABI_VERSION, COMMAND_EFFECT_BYPASS, COMMAND_EFFECT_PARAM, COMMAND_FADER_DB, COMMAND_MATRIX,
    COMMAND_MUTE, COMMAND_PAN, COMMAND_REASON_BACKPRESSURE, COMMAND_REASON_DOMAIN,
    COMMAND_REASON_MALFORMED, COMMAND_REASON_NONE, COMMAND_REASON_UNKNOWN_EFFECT,
    COMMAND_REASON_UNKNOWN_PARAMETER, COMMAND_REASON_UNKNOWN_RACK, COMMAND_REASON_UNKNOWN_TRACK,
    COMMAND_REASON_UNSUPPORTED_KIND, COMMAND_REASON_WRONG_STATE, COMMAND_RECORD_BYTES,
    MAXIMUM_COMMAND_RECORDS,
};

/// The emitted file name, shipped beside the Wasm artifact.
pub const OUTPUT_NAME: &str = "miso-engine-v2-parameter-metadata.json";
/// The schema tag every consumer must check before reading a field.
pub const SCHEMA: &str = "miso.web.parameter-metadata.v1";
/// The launch sample rates a rate-keyed builtin domain is reported for.
pub const LAUNCH_RATES_HZ: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

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
    let registry = launch_native_effect_registry_v1().expect("launch effect registry");
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
        (COMMAND_PAN, "pan", true),
        (COMMAND_MATRIX, "matrix", true),
        (COMMAND_FADER_DB, "faderDb", false),
        (COMMAND_MUTE, "mute", false),
        (COMMAND_EFFECT_PARAM, "effectParam", false),
        (COMMAND_EFFECT_BYPASS, "effectBypass", false),
    ];
    for (index, (value, name, applied)) in kinds.iter().enumerate() {
        out.push_str(&format!(
            "    {{ \"value\": {value}, \"name\": \"{name}\", \"applied\": {applied} }}{}\n",
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
    ];
    for (index, (value, name)) in reasons.iter().enumerate() {
        out.push_str(&format!(
            "    {{ \"value\": {value}, \"name\": \"{name}\" }}{}\n",
            comma(index, reasons.len())
        ));
    }
    out.push_str("  ],\n");
    out.push_str("  \"builtins\": {\n    \"parameters\": [\n");
    let builtins = BUILTIN_PARAMETER_DESCRIPTORS_V1;
    for (index, parameter) in builtins.iter().enumerate() {
        out.push_str(&builtin_parameter(parameter));
        out.push_str(&format!("{}\n", comma(index, builtins.len())));
    }
    out.push_str("    ]\n  },\n");
    out.push_str("  \"effects\": [\n");
    let descriptors: Vec<&'static EffectDescriptorV1> = registry.descriptors().collect();
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
/// finite by `validate_descriptor_v1` before a factory may enter the registry.
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

fn effect(descriptor: &EffectDescriptorV1) -> String {
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
    out.push_str("      ]\n    }");
    out
}

fn effect_parameter(parameter: &ParameterDescriptorV1) -> String {
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
    // The live-console command path has no post-preparation effect write path at all; every
    // effect parameter is refused with COMMAND_REASON_UNSUPPORTED_KIND after its address and
    // domain have been checked. Stating that here is what keeps an app from discovering it live.
    out.push_str("          \"liveUpdatable\": false,\n");
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
