//! Build-time parameter-metadata codegen for the browser live controls (issue #137 D4).
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
//! Every parameter carries whether the live-control command path can actually move it. Issue #140
//! made that true of every builtin block target and every automatable effect parameter: a builtin
//! row is live exactly when its declared update rate is `blockTarget`, and an effect row is live
//! exactly when its descriptor declares it automatable. A caller that reads this file never has to
//! discover either at runtime. See the browser ABI's `.d.ts` header for the full statement.
//!
//! # `commandKinds` is the whole vocabulary, and `plane` says what each kind moves
//!
//! Every kind the wire decodes is a row here -- all twelve of them. Before the kind-vocabulary
//! gate this table stopped at `effectBypass` while the Rust constants, the host JS `COMMAND_KINDS`
//! set and the `.d.ts` enum all carried eight, and nothing noticed: an app reading its vocabulary
//! from this file could not learn that `observeSubscribe`/`observeUnsubscribe` exist.
//!
//! The two observation kinds are not DSP kinds, so each row carries `plane`. `applied` keeps its
//! issue #140 meaning -- the ABI applies this kind rather than declaring and refusing it, which is
//! true of all twelve -- and `plane` distinguishes the ten that move state the render thread reads
//! (`"render"`) from the two that bind or unbind an entry in the `miso.observe.v1` subscription
//! map and change nothing rendered (`"observation"`).
//!
//! `solo` (issue #210 phase 1) is a `"render"` kind with no row in `builtins`, and deliberately:
//! it moves what the render thread reads -- it composes into the fader section's mute -- but it is
//! live-control/monitor state rather than a strip DSP parameter, so it has no parameter descriptor,
//! no domain table and no session key to be automated from.
//!
//! `trimDb`, `polarityInvert` and `inputFilters` are the opposite case and the ordinary one:
//! all are `"render"` kinds *and* strip DSP parameters, so each has a `builtins` row. The filter
//! rows use the fixed `"linear64CoefficientUpdates"` policy because the command carries prepared
//! coefficient targets; their `liveUpdatable` flag is derived from `"blockTarget"`.
//!
//! # Decision 17, slice S2b (`controls`)
//!
//! The top-level `controls` object lists the rows of the `controls` crate's tables (`stripOwn`,
//! `builtins`, `routeOwn`, `vcaOwn`, `sessionOwn`, and `effects` by effect ID). It is written
//! from `controls::table_rows` and `controls::effect_table_rows` and from no list of its own, so
//! a row added to a table reaches the SDK's `controls.ts` when the metadata is regenerated. The
//! keys before it keep their bytes. A row's `name` is the declared machine name (the identifier);
//! `displayName` is the label a person reads, which the `effects` block above still writes as
//! `name`.
//!
//! # Issue #242 (parameter lattice and named step sizes)
//!
//! Each parameter carries a populated `"step"` object: the row's decimal step, the unit that
//! decimal is read in, the pinned rendering precision, and #127's five named ladder multiples.
//! Together with the row's `minimum`/`maximum` this is the parameter's LATTICE -- the finite set
//! of legal persisted values -- so the slot is the descriptor's declaration, never a restatement.
//! It occupies the slot that shipped as `"nudge": null` before #242 renamed the vocabulary
//! (#239 Amendment 2); the rename is total, and `nudge` is retired.

//! # Issue #278 (the port table)
//!
//! Each effect carries its declared `"ports"` -- id, role, `required` and lane layout -- for the
//! same reason it carries its parameters: the descriptor is the only thing that knows them, and a
//! consumer that kept its own table would go stale. Exactly two launch effects declare a
//! `sidechainInput` (`miso.compressor` and `miso.gate-expander`, both `"sidechain-in"`, both
//! optional); the other six declare only their main pair, and every effect gets the key.
//!
//! This is an ADDITIVE extension of the v1 surface, handled exactly as #143's `observations` was:
//! the schema tag and `abiVersion` are unmoved, and the new key is *never absent*, so there is no
//! "document predates ports" state for a reader to have to distinguish.

macro_rules! append {
    ($out:expr, $($arguments:tt)*) => {
        std::fmt::Write::write_fmt($out, format_args!($($arguments)*))
            .expect("writing to a String cannot fail")
    };
}

pub mod abi_layout;

use std::io::Write as _;
use std::path::PathBuf;

use bench_support::json::escape;
use builtins::{
    BUILTIN_PARAMETER_DESCRIPTORS, BuiltinParameterDescriptor, BuiltinParameterDomain,
    BuiltinParameterMapping, BuiltinParameterReset, BuiltinParameterScope,
    BuiltinParameterUpdateRate, BuiltinSmoothingPolicy, builtin_filter_cutoff_maximum_hz,
    builtin_parameter_unit,
};
use controls::{
    ControlClass, ControlDescriptor, ControlKind, ControlScope, EffectTableRows, Family, Lane,
    LiveRampRow, Motion, OwnerKind, RebuildReason, RowSource, SectionKind, TableRow, Taper,
    effect_table_rows, table_rows,
};
use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    AutomationRate, EffectDescriptor, NativeEffectResponseFactory, ObservationCadence,
    ObservationChannels, ObservationCost, ObservationDescriptor, ObservationFold, ObservationKind,
    ParameterChannelPolicy, ParameterDescriptor, ParameterDomain, ParameterLattice,
    ParameterMapping, ParameterUnit, PortDescriptor, PortLayout, PortRole,
    ResponseAmplitudeReference, ResponseAnalysisDescriptor, ResponseBypassSemantics,
    ResponseChannelLayout, ResponseQueryCadence, ResponseSectionOutput, ResponseTotalScope,
    SmoothingRule, StepSize, StepUnit,
};
use host_web::{
    ABI_VERSION, COMMAND_EFFECT_BYPASS, COMMAND_EFFECT_PARAM, COMMAND_FADER_DB,
    COMMAND_INPUT_FILTERS, COMMAND_MATRIX, COMMAND_MUTE, COMMAND_OBSERVE_SUBSCRIBE,
    COMMAND_OBSERVE_UNSUBSCRIBE, COMMAND_PAN, COMMAND_POLARITY_INVERT, COMMAND_REASON_BACKPRESSURE,
    COMMAND_REASON_DOMAIN, COMMAND_REASON_MALFORMED, COMMAND_REASON_NONE,
    COMMAND_REASON_NOT_SOLOABLE, COMMAND_REASON_OBSERVATION_UNBOUND, COMMAND_REASON_UNKNOWN_EFFECT,
    COMMAND_REASON_UNKNOWN_PARAMETER, COMMAND_REASON_UNKNOWN_RACK, COMMAND_REASON_UNKNOWN_ROUTE,
    COMMAND_REASON_UNKNOWN_TAP, COMMAND_REASON_UNKNOWN_TRACK, COMMAND_REASON_UNKNOWN_VCA,
    COMMAND_REASON_UNSUPPORTED_KIND, COMMAND_REASON_WRONG_STATE, COMMAND_RECORD_BYTES,
    COMMAND_ROUTE_GAIN_DB, COMMAND_ROUTE_MATRIX, COMMAND_ROUTE_MUTE, COMMAND_SOLO, COMMAND_TRIM_DB,
    COMMAND_VCA_FADER_DB, COMMAND_VCA_MUTE, MAXIMUM_COMMAND_RECORDS,
};

/// The emitted file name, shipped beside the Wasm artifact.
pub const OUTPUT_NAME: &str = "miso-engine-v1-parameter-metadata.json";
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
        "usage: parameter_metadata --write DIRECTORY | --check DIRECTORY \
| --print | --print-abi-layout"
    );
    std::process::exit(2)
}

/// Render the whole document. Deterministic: registry order is `EffectId` order.
#[must_use]
pub fn render() -> String {
    let registry = launch_native_effect_registry().expect("launch effect registry");
    let mut out = String::with_capacity(1 << 16);
    out.push_str("{\n");
    append!(&mut out, "  \"schema\": \"{SCHEMA}\",\n");
    append!(&mut out, "  \"abiVersion\": {ABI_VERSION},\n");
    append!(
        &mut out,
        "  \"commandRecordBytes\": {COMMAND_RECORD_BYTES},\n"
    );
    append!(
        &mut out,
        "  \"maximumCommandRecords\": {MAXIMUM_COMMAND_RECORDS},\n"
    );
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
        (COMMAND_INPUT_FILTERS, "inputFilters", true, PLANE_RENDER),
        (COMMAND_ROUTE_GAIN_DB, "routeGainDb", true, PLANE_RENDER),
        (COMMAND_ROUTE_MUTE, "routeMute", true, PLANE_RENDER),
        (COMMAND_ROUTE_MATRIX, "routeMatrix", true, PLANE_RENDER),
        (COMMAND_VCA_FADER_DB, "vcaFaderDb", true, PLANE_RENDER),
        (COMMAND_VCA_MUTE, "vcaMute", true, PLANE_RENDER),
    ];
    for (index, (value, name, applied, plane)) in kinds.iter().enumerate() {
        append!(
            &mut out,
            "    {{ \"value\": {value}, \"name\": \"{name}\", \"applied\": {applied}, \
             \"plane\": \"{plane}\" }}{}\n",
            comma(index, kinds.len())
        );
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
        (COMMAND_REASON_NOT_SOLOABLE, "notSoloable"),
        (COMMAND_REASON_UNKNOWN_ROUTE, "unknownRoute"),
        (COMMAND_REASON_UNKNOWN_VCA, "unknownVca"),
    ];
    for (index, (value, name)) in reasons.iter().enumerate() {
        append!(
            &mut out,
            "    {{ \"value\": {value}, \"name\": \"{name}\" }}{}\n",
            comma(index, reasons.len())
        );
    }
    out.push_str("  ],\n");
    // Issue #143 D1: the observation vocabularies, so a consumer resolves a tap's raw `u32`s the
    // same way it resolves a parameter's -- from this document, never from a hand-written table.
    out.push_str("  \"observationVocabularies\": {\n");
    let vocabularies: [(&str, &[(u32, &str)]); 5] = [
        (
            "kinds",
            &[(ObservationKind::GainReductionDb as u32, "gainReductionDb")],
        ),
        (
            "costs",
            &[
                (ObservationCost::Resident as u32, "resident"),
                (ObservationCost::Computed as u32, "computed"),
            ],
        ),
        (
            "cadences",
            &[
                (ObservationCadence::PerBlock as u32, "perBlock"),
                (ObservationCadence::PerWindow as u32, "perWindow"),
            ],
        ),
        (
            "folds",
            &[
                (ObservationFold::Latest as u32, "latest"),
                (ObservationFold::PeakMagnitude as u32, "peakMagnitude"),
            ],
        ),
        (
            "channels",
            &[
                (ObservationChannels::Shared as u32, "shared"),
                (ObservationChannels::PerLane as u32, "perLane"),
            ],
        ),
    ];
    for (index, (name, rows)) in vocabularies.iter().enumerate() {
        append!(&mut out, "    \"{name}\": [");
        for (row, (value, label)) in rows.iter().enumerate() {
            append!(
                &mut out,
                "{{ \"value\": {value}, \"name\": \"{label}\" }}{}",
                if row + 1 == rows.len() { "" } else { ", " }
            );
        }
        append!(&mut out, "]{}\n", comma(index, vocabularies.len()));
    }
    out.push_str("  },\n");
    out.push_str("  \"builtins\": {\n    \"response\": ");
    response_descriptor(&mut out, builtins::input_filter_response_descriptor());
    out.push_str(",\n    \"parameters\": [\n");
    let builtins = BUILTIN_PARAMETER_DESCRIPTORS;
    for (index, parameter) in builtins.iter().enumerate() {
        builtin_parameter(&mut out, parameter);
        append!(&mut out, "{}\n", comma(index, builtins.len()));
    }
    out.push_str("    ]\n  },\n");
    out.push_str("  \"effects\": [\n");
    for (index, descriptor) in registry.descriptors().enumerate() {
        let response = registry
            .get_ascii(descriptor.id.as_str())
            .and_then(|factory| factory.response_analysis());
        effect(&mut out, descriptor, response);
        append!(&mut out, "{}\n", comma(index, registry.len()));
    }
    out.push_str("  ],\n");
    controls_object(&mut out, registry);
    out.push_str("}\n");
    out
}

/// The `controls` object (decision 17, slice S2b): the row tables of the `controls` crate, one
/// list per row source and one entry per launch effect.
///
/// Nothing here names a row. Every row comes from `controls::table_rows` or
/// `controls::effect_table_rows`, so a row added to a table reaches this document, and from it the
/// SDK's `controls.ts`, when the metadata is regenerated. A row's `name` is the declared machine
/// name; `displayName` is the label a person reads.
/// The address vocabulary (decision 17, C2): the owner kinds, section kinds and lanes of a control
/// address, each with its stable code. The SDK builds its address types from it, so a kind the
/// core gains reaches the SDK when the assets are regenerated and not by a hand edit.
fn address_object(out: &mut String) {
    fn list(out: &mut String, key: &str, entries: &[(&str, u8)], last: bool) {
        append!(out, "      \"{key}\": [");
        for (index, (name, code)) in entries.iter().enumerate() {
            append!(
                out,
                "{{ \"name\": \"{}\", \"code\": {code} }}{}",
                escape(name),
                if index + 1 == entries.len() { "" } else { ", " }
            );
        }
        append!(out, "]{}\n", if last { "" } else { "," });
    }
    out.push_str("    \"address\": {\n");
    let owners = OwnerKind::ALL.map(|kind| (kind.name(), kind.code()));
    list(out, "ownerKinds", &owners, false);
    let sections = SectionKind::ALL.map(|kind| (kind.name(), kind.code()));
    list(out, "sectionKinds", &sections, false);
    let lanes: Vec<(&str, u8)> = Lane::ALL
        .iter()
        .map(|(lane, name)| (*name, lane.wire()))
        .collect();
    list(out, "lanes", &lanes, true);
    out.push_str("    },\n");
}

fn controls_object(out: &mut String, registry: &effect_contract::NativeEffectRegistry) {
    out.push_str("  \"controls\": {\n");
    address_object(out);
    let sources = [
        ("stripOwn", RowSource::StripOwn),
        ("builtins", RowSource::Builtins),
        ("routeOwn", RowSource::RouteOwn),
        ("vcaOwn", RowSource::VcaOwn),
        ("sessionOwn", RowSource::SessionOwn),
    ];
    for (name, source) in sources {
        append!(out, "    \"{name}\": [\n");
        let rows = table_rows(source);
        for (index, row) in rows.iter().enumerate() {
            out.push_str("      ");
            control_row(out, row);
            append!(out, "{}\n", comma(index, rows.len()));
        }
        out.push_str("    ],\n");
    }
    out.push_str("    \"effects\": {\n");
    for (index, descriptor) in registry.descriptors().enumerate() {
        let EffectTableRows {
            bypass,
            parameters,
            link_mode,
        } = effect_table_rows(descriptor.id.as_str()).expect("a registry effect has rows");
        append!(out, "      \"{}\": {{\n", escape(descriptor.id.as_str()));
        out.push_str("        \"bypass\": ");
        control_row(out, &bypass);
        out.push_str(",\n        \"parameters\": [\n");
        for (row, parameter) in parameters.iter().enumerate() {
            out.push_str("          ");
            control_row(out, parameter);
            append!(out, "{}\n", comma(row, parameters.len()));
        }
        out.push_str("        ],\n        \"linkMode\": ");
        match &link_mode {
            Some(row) => control_row(out, row),
            None => out.push_str("null"),
        }
        append!(out, "\n      }}{}\n", comma(index, registry.len()));
    }
    out.push_str("    }\n  }\n");
}

/// One row of a table as one line of JSON.
///
/// The fields that depend on the rate are emitted per launch rate when they differ across the four
/// (`maximum`); the fields that depend on a route's destination are emitted per case
/// (`intoSubmix`, `intoOutput`), each case carrying the class and whether the row applies.
fn control_row(out: &mut String, row: &TableRow) {
    let descriptor = &row.descriptor;
    append!(
        out,
        "{{ \"control\": {}, \"name\": \"{}\", \"displayName\": \"{}\", \"displayUnit\": \"{}\", \
\"unit\": \"{}\", \"kind\": \"{}\", \"minimum\": {}, \"maximum\": ",
        row.control,
        escape(descriptor.name),
        escape(descriptor.display_name),
        escape(descriptor.display_unit),
        unit_name(descriptor.unit),
        control_kind_name(descriptor.kind),
        number(descriptor.minimum),
    );
    let maxima = LAUNCH_RATES_HZ.map(|rate| row.at_rate(rate).maximum);
    if maxima
        .iter()
        .all(|maximum| maximum.to_bits() == maxima[0].to_bits())
    {
        out.push_str(&number(maxima[0]));
    } else {
        out.push_str("{ ");
        for (index, (rate, maximum)) in LAUNCH_RATES_HZ.iter().zip(maxima).enumerate() {
            append!(
                out,
                "\"{rate}\": {}{}",
                number(maximum),
                if index + 1 == LAUNCH_RATES_HZ.len() {
                    ""
                } else {
                    ", "
                }
            );
        }
        out.push_str(" }");
    }
    append!(
        out,
        ", \"disabledValue\": {}, \"default\": {}, \"taper\": \"{}\", \"step\": ",
        optional_number(descriptor.disabled_value),
        number(descriptor.default),
        taper_name(descriptor.taper),
    );
    step_object(out, descriptor.lattice);
    out.push_str(", \"choices\": [");
    for (index, choice) in descriptor.choices.iter().enumerate() {
        append!(
            out,
            "{{ \"value\": {}, \"label\": \"{}\" }}{}",
            number(choice.value),
            escape(choice.label),
            if index + 1 == descriptor.choices.len() {
                ""
            } else {
                ", "
            }
        );
    }
    append!(
        out,
        "], \"ramp\": \"{}\", \"scope\": \"{}\", \"family\": \"{}\", \"lanes\": \"{}\"",
        match descriptor.motion {
            Motion::Ramp(ramp) => ramp_name(ramp),
            Motion::Fixed => "fixed",
        },
        match descriptor.scope {
            ControlScope::Model => "model",
            ControlScope::Monitor => "monitor",
        },
        family_name(descriptor.family),
        match descriptor.lanes {
            controls::Lanes::PerLane => "perLane",
            controls::Lanes::Shared => "shared",
        },
    );
    if let Some(output) = row.into_output {
        out.push_str(", \"intoSubmix\": ");
        class_case(out, descriptor, descriptor.class, true);
        out.push_str(", \"intoOutput\": ");
        class_case(out, descriptor, output.class, output.active);
    } else {
        out.push_str(", ");
        class_fields(out, descriptor, descriptor.class);
        if !row.on_submix {
            out.push_str(", \"onSubmix\": false");
        }
    }
    out.push_str(" }");
}

/// A route row's class under one destination.
fn class_case(out: &mut String, descriptor: &ControlDescriptor, class: ControlClass, active: bool) {
    out.push_str("{ ");
    class_fields(out, descriptor, class);
    append!(out, ", \"active\": {active} }}");
}

/// `class`, `reason`, `reasonIssue`, `reopen` and `automatable` of a row under `class`.
///
/// `automatable` is the descriptor's own computation, never a second rule (a live model value that
/// render reads).
fn class_fields(out: &mut String, descriptor: &ControlDescriptor, class: ControlClass) {
    let under = ControlDescriptor {
        class,
        ..*descriptor
    };
    let (name, reason, issue, reopen) = match class {
        ControlClass::Live => ("live", None, None, None),
        ControlClass::Derived => ("derived", None, None, None),
        ControlClass::RebuildOnly { reason, reopen } => {
            let (name, issue) = rebuild_reason(reason);
            ("rebuildOnly", Some(name), issue, Some(reopen))
        }
    };
    append!(
        out,
        "\"class\": \"{name}\", \"reason\": {}, \"reasonIssue\": {}, \"reopen\": {}, \
\"automatable\": {}",
        reason.map_or_else(|| "null".to_owned(), |text| format!("\"{text}\"")),
        issue.map_or_else(|| "null".to_owned(), |issue| issue.to_string()),
        reopen.map_or_else(|| "null".to_owned(), |text| format!("\"{}\"", escape(text))),
        under.automatable(),
    );
}

const fn rebuild_reason(reason: RebuildReason) -> (&'static str, Option<u32>) {
    match reason {
        RebuildReason::NewMemory => ("newMemory", None),
        RebuildReason::NewGraph => ("newGraph", None),
        RebuildReason::ProcessingOrder => ("processingOrder", None),
        RebuildReason::Latency => ("latency", None),
        RebuildReason::Glitch => ("glitch", None),
        RebuildReason::Optimisation => ("optimisation", None),
        RebuildReason::Correctness => ("correctness", None),
        RebuildReason::RuledLiveNotBuilt { issue } => ("ruledLiveNotBuilt", Some(issue)),
    }
}

const fn control_kind_name(kind: ControlKind) -> &'static str {
    match kind {
        ControlKind::Continuous => "continuous",
        ControlKind::Toggle => "toggle",
        ControlKind::Enumeration => "enumeration",
    }
}

const fn taper_name(taper: Taper) -> &'static str {
    match taper {
        Taper::Linear => "linear",
        Taper::Logarithmic => "logarithmic",
        Taper::Exponential => "exponential",
        Taper::Decibel => "decibel",
        Taper::Stepped => "stepped",
    }
}

const fn family_name(family: Family) -> &'static str {
    match family {
        Family::StripLevel => "stripLevel",
        Family::StripMatrix => "stripMatrix",
        Family::StripInput => "stripInput",
        Family::Route => "route",
        Family::Effect => "effect",
        Family::Setting => "setting",
    }
}

const fn ramp_name(ramp: LiveRampRow) -> &'static str {
    match ramp {
        LiveRampRow::Fader => "fader",
        LiveRampRow::InputTrim => "inputTrim",
        LiveRampRow::SendGain => "sendGain",
        LiveRampRow::VcaOffset => "vcaOffset",
        LiveRampRow::LinkGlide => "linkGlide",
        LiveRampRow::Mute => "mute",
        LiveRampRow::Solo => "solo",
        LiveRampRow::PolarityInvert => "polarityInvert",
        LiveRampRow::SendMute => "sendMute",
        LiveRampRow::SendFollowsMute => "sendFollowsMute",
        LiveRampRow::VcaMute => "vcaMute",
        LiveRampRow::BypassCrossfade => "bypassCrossfade",
        LiveRampRow::Pan => "pan",
        LiveRampRow::Matrix => "matrix",
        LiveRampRow::SendMatrix => "sendMatrix",
    }
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

fn effect(
    out: &mut String,
    descriptor: &EffectDescriptor,
    response: Option<&dyn NativeEffectResponseFactory>,
) {
    out.push_str("    {\n");
    append!(
        out,
        "      \"id\": \"{}\",\n",
        escape(descriptor.id.as_str())
    );
    append!(
        out,
        "      \"displayName\": \"{}\",\n",
        escape(descriptor.display_name)
    );
    append!(
        out,
        "      \"contractMajor\": {}, \"contractMinor\": {}, \"stateLayoutVersion\": {},\n",
        descriptor.contract_major,
        descriptor.contract_minor,
        descriptor.state_layout_version
    );
    out.push_str("      \"parameters\": [\n");
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        effect_parameter(out, parameter);
        append!(out, "{}\n", comma(index, descriptor.parameters.len()));
    }
    out.push_str("      ],\n");
    // Issue #278: the declared port table, in the descriptor's own order (role ascending), and
    // never absent -- `[]` for an effect that declares no port at all, for the same reason
    // `observations` is never absent. Until this shipped, the document published every effect's
    // parameters and taps but nothing about its ports, so `sidechain.port_id` was the one session
    // field an SDK builder could not check before boot: a misspelling parsed, validated, compiled
    // and then failed preparation with `effect.sidechain.unknown_port`. The engine's refusal is
    // unmoved; this is what lets an authoring layer refuse the same thing first, by name.
    out.push_str("      \"ports\": [");
    for (index, port) in descriptor.ports.iter().enumerate() {
        out.push('\n');
        effect_port(out, port);
        out.push_str(comma(index, descriptor.ports.len()));
    }
    if descriptor.ports.is_empty() {
        out.push_str("],\n");
    } else {
        out.push_str("\n      ],\n");
    }
    // // Issue #143: never absent. An effect that declares no tap emits `[]`, so a consumer reads one
    // shape for every effect and "this build has no menu for that effect" is impossible to
    // confuse with "this document predates observation".
    out.push_str("      \"observations\": [");
    for (index, observation) in descriptor.observations.iter().enumerate() {
        out.push('\n');
        effect_observation(out, observation);
        out.push_str(comma(index, descriptor.observations.len()));
    }
    if descriptor.observations.is_empty() {
        out.push_str("],\n");
    } else {
        out.push_str("\n      ],\n");
    }
    out.push_str("      \"response\": ");
    match response {
        Some(factory) => response_descriptor(out, factory.analysis_descriptor()),
        None => out.push_str("null"),
    }
    out.push_str("\n    }");
}

fn response_descriptor(out: &mut String, descriptor: &ResponseAnalysisDescriptor) {
    out.push_str("{ ");
    append!(
        out,
        "\"id\": {}, \"name\": \"{}\", \"axisUnit\": \"{}\", \"unit\": \"{}\", ",
        descriptor.id,
        escape(descriptor.name),
        response_axis_unit(descriptor.axis_unit),
        response_axis_unit(descriptor.unit),
    );
    append!(
        out,
        "\"amplitudeReference\": \"{}\", \"channels\": \"{}\", \"mode\": \"{}\", ",
        response_amplitude_reference(descriptor.amplitude_reference),
        response_channels(descriptor.channels),
        response_mode(descriptor.mode),
    );
    append!(
        out,
        "\"cadence\": \"{}\", \"sectionOutput\": \"{}\", \"cost\": \"computed\", ",
        response_cadence(descriptor.cadence),
        response_section_output(descriptor.section_output),
    );
    append!(
        out,
        "\"floorDb\": {}, \"totalScope\": \"{}\", \"bypass\": \"{}\", \"sections\": [",
        number(descriptor.floor_db),
        response_scope(descriptor.total_scope),
        response_bypass(descriptor.bypass),
    );
    for (index, section) in descriptor.sections.iter().enumerate() {
        append!(
            out,
            "{{ \"id\": {}, \"name\": \"{}\" }}{}",
            section.id,
            escape(section.name),
            comma(index, descriptor.sections.len()),
        );
    }
    out.push_str("] }");
}

const fn response_axis_unit(unit: ParameterUnit) -> &'static str {
    match unit {
        ParameterUnit::Hz => "hz",
        ParameterUnit::Db => "db",
        _ => "unknown",
    }
}

const fn response_amplitude_reference(reference: ResponseAmplitudeReference) -> &'static str {
    match reference {
        ResponseAmplitudeReference::Unity => "unity",
    }
}

const fn response_channels(channels: ResponseChannelLayout) -> &'static str {
    match channels {
        ResponseChannelLayout::Independent => "independent",
    }
}

const fn response_mode(mode: effect_contract::ResponseAnalysisMode) -> &'static str {
    match mode {
        effect_contract::ResponseAnalysisMode::RequestedConfiguration => "requestedConfiguration",
    }
}

const fn response_cadence(cadence: ResponseQueryCadence) -> &'static str {
    match cadence {
        ResponseQueryCadence::ExplicitQuery => "explicitQuery",
    }
}

const fn response_section_output(output: ResponseSectionOutput) -> &'static str {
    match output {
        ResponseSectionOutput::TotalAndOptionalSections => "totalAndOptionalSections",
    }
}

const fn response_scope(scope: ResponseTotalScope) -> &'static str {
    match scope {
        ResponseTotalScope::ParametricEqCascade => "parametricEqCascade",
        ResponseTotalScope::BuiltinInputFilterSubtotal => "builtinInputFilterSubtotal",
    }
}

const fn response_bypass(bypass: ResponseBypassSemantics) -> &'static str {
    match bypass {
        ResponseBypassSemantics::EffectWideIdentityWithSections => "effectWideIdentityWithSections",
        ResponseBypassSemantics::NoEffectBypass => "noEffectBypass",
    }
}

/// Emit one declared port row.
///
/// Both the raw scalar and its name ride, exactly as they do on a parameter's `unit` and an
/// observation's `kind`: the descriptor wire and the C inspect record carry the `u32`, and a
/// consumer that resolves it from this document never keeps a second table of its own.
fn effect_port(out: &mut String, port: &PortDescriptor) {
    append!(
        out,
        "        {{ \"id\": \"{}\", \"role\": {}, \"roleName\": \"{}\", \
\"required\": {}, \"layout\": {}, \"layoutName\": \"{}\" }}",
        escape(port.id.as_str()),
        port.role as u32,
        port_role_name(port.role),
        port.required,
        port.layout as u32,
        port_layout_name(port.layout),
    );
}

const fn port_role_name(role: PortRole) -> &'static str {
    match role {
        PortRole::MainInput => "mainInput",
        PortRole::MainOutput => "mainOutput",
        PortRole::SidechainInput => "sidechainInput",
    }
}

const fn port_layout_name(layout: PortLayout) -> &'static str {
    match layout {
        PortLayout::DualMonoPlanar => "dualMonoPlanar",
    }
}

fn effect_observation(out: &mut String, observation: &ObservationDescriptor) {
    // `subscribable` is derived from the cost class exactly as `liveUpdatable` is derived from
    // `automatable`: a `Resident` tap is a copy out of state the block already wrote and the
    // subscribe path binds it; a `Computed` tap has no implementation in V1 and the subscribe path
    // answers `unsupportedKind`. The two statements are the same statement, which is why this is
    // derived rather than written down -- and why the schema gate refuses a computed tap that
    // claims to be subscribable.
    let subscribable = matches!(observation.cost, ObservationCost::Resident);
    append!(
        out,
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
    );
}

const fn observation_kind_name(kind: ObservationKind) -> &'static str {
    match kind {
        ObservationKind::GainReductionDb => "gainReductionDb",
    }
}

const fn observation_cost_name(cost: ObservationCost) -> &'static str {
    match cost {
        ObservationCost::Resident => "resident",
        ObservationCost::Computed => "computed",
    }
}

const fn observation_cadence_name(cadence: ObservationCadence) -> &'static str {
    match cadence {
        ObservationCadence::PerBlock => "perBlock",
        ObservationCadence::PerWindow => "perWindow",
    }
}

const fn observation_fold_name(fold: ObservationFold) -> &'static str {
    match fold {
        ObservationFold::Latest => "latest",
        ObservationFold::PeakMagnitude => "peakMagnitude",
    }
}

const fn observation_channels_name(channels: ObservationChannels) -> &'static str {
    match channels {
        ObservationChannels::Shared => "shared",
        ObservationChannels::PerLane => "perLane",
    }
}

/// Emit one parameter's `step` declaration.
///
/// This is the #242 lattice authority as it crosses the agent/SDK boundary: a decimal string, not
/// a float, so a catalog round-trip is lossless. `size` is the shortest spelling that names the
/// declared `f32` exactly -- a ratio row's authority really is the decimal `1.02`, and rendering
/// it at the row's own eight-decimal value precision would spell it `1.01999998`.
fn step_object(out: &mut String, lattice: ParameterLattice) {
    append!(
        out,
        "{{ \"unit\": \"{}\", \"size\": \"{}\", \"precision\": {}, \"ladder\": \
{{ \"xs\": {}, \"sm\": {}, \"md\": {}, \"lg\": {}, \"xl\": {} }} }}",
        step_unit_name(lattice.step_unit),
        lattice.step,
        lattice.precision,
        lattice.ladder.multiple(StepSize::Xs),
        lattice.ladder.multiple(StepSize::Sm),
        lattice.ladder.multiple(StepSize::Md),
        lattice.ladder.multiple(StepSize::Lg),
        lattice.ladder.multiple(StepSize::Xl),
    );
}

const fn step_unit_name(unit: StepUnit) -> &'static str {
    match unit {
        StepUnit::Absolute => "absolute",
        StepUnit::Cents => "cents",
        StepUnit::Ratio => "ratio",
        StepUnit::Index => "index",
    }
}

fn effect_parameter(out: &mut String, parameter: &ParameterDescriptor) {
    out.push_str("        {\n");
    append!(out, "          \"id\": {},\n", parameter.id.0);
    append!(
        out,
        "          \"name\": \"{}\",\n",
        escape(parameter.display_name)
    );
    append!(
        out,
        "          \"displayUnit\": \"{}\",\n",
        escape(parameter.display_unit)
    );
    append!(
        out,
        "          \"unit\": {}, \"unitName\": \"{}\",\n",
        parameter.unit as u32,
        unit_name(parameter.unit)
    );
    append!(
        out,
        "          \"domain\": {}, \"domainName\": \"{}\",\n",
        parameter.domain as u32,
        domain_name(parameter.domain)
    );
    append!(
        out,
        "          \"minimum\": {}, \"maximum\": {}, \"default\": {},\n",
        optional_number(parameter.minimum),
        optional_number(parameter.maximum),
        number(parameter.default_value)
    );
    append!(
        out,
        "          \"mapping\": {}, \"mappingName\": \"{}\",\n",
        parameter.mapping as u32,
        mapping_name(parameter.mapping)
    );
    append!(
        out,
        "          \"automationRate\": {}, \"automationRateName\": \"{}\",\n",
        parameter.automation_rate as u32,
        automation_rate_name(parameter.automation_rate)
    );
    append!(
        out,
        "          \"channelPolicy\": {}, \"channelPolicyName\": \"{}\",\n",
        parameter.channel_policy as u32,
        channel_policy_name(parameter.channel_policy)
    );
    append!(
        out,
        "          \"smoothing\": {}, \"smoothingName\": \"{}\", \"smoothingSamples\": {},\n",
        parameter.smoothing as u32,
        smoothing_name(parameter.smoothing),
        parameter.smoothing_samples
    );
    append!(
        out,
        "          \"readable\": {}, \"automatable\": {},\n",
        parameter.readable,
        parameter.automatable
    );
    // Issue #140 A: the live-control command path now feeds an admitted parameter into the
    // running plan as a `PreparedAutomationSpan`, so a parameter is live exactly when its own
    // descriptor says it can be automated. The two statements are the same statement, which is
    // why this is derived from `automatable` rather than written down. A parameter that declares
    // `AutomationRate::None` has no span the effect would accept and stays `false`.
    append!(
        out,
        "          \"liveUpdatable\": {},\n",
        parameter.automatable
    );
    out.push_str("          \"enumChoices\": [");
    for (index, choice) in parameter.enum_choices.iter().enumerate() {
        append!(
            out,
            "{{ \"value\": {}, \"label\": \"{}\" }}{}",
            number(choice.value),
            escape(choice.label),
            if index + 1 == parameter.enum_choices.len() {
                ""
            } else {
                ", "
            }
        );
    }
    out.push_str("],\n");
    // Issue #242: the lattice declaration this row's legal persisted values are generated from.
    out.push_str("          \"step\": ");
    step_object(out, parameter.lattice);
    out.push_str("\n        }");
}

fn builtin_parameter(out: &mut String, parameter: &BuiltinParameterDescriptor) {
    let unit = builtin_parameter_unit(parameter);
    // A rate-keyed cutoff has no single maximum: `builtin_filter_cutoff_maximum_hz` gives one
    // per launch rate, so the row carries the exact `f32` for each rather than a number that would
    // be wrong at three of the four.
    let (minimum, maximum, domain_name) = match parameter.domain {
        BuiltinParameterDomain::BooleanExact => (None, None, "booleanExact"),
        BuiltinParameterDomain::FiniteInclusive { minimum, maximum } => {
            (Some(minimum), Some(maximum), "finiteInclusive")
        }
        BuiltinParameterDomain::DisabledOrRateKeyedHertz { minimum_hz, .. } => {
            (Some(minimum_hz), None, "disabledOrRateKeyedHertz")
        }
    };
    // `matrix_ll/lr/rl/rr` are the only builtin parameters the ABI declares as `BlockTarget`, and
    // they are exactly the ones the live-control command path applies. The two statements are the
    // same statement, which is why this is derived from the descriptor rather than written down.
    let live = matches!(
        parameter.update_rate,
        BuiltinParameterUpdateRate::BlockTarget
    );
    append!(
        out,
        "      {{ \"id\": {}, \"name\": \"{}\", \"unit\": {}, \"unitName\": \"{}\", \"scope\": \"{}\", \"mapping\": \"{}\", \
\"domain\": \"{}\", \"minimum\": {}, \"maximum\": {}, \"maximumByRate\": ",
        parameter.id,
        escape(parameter.name),
        unit as u32,
        unit_name(unit),
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
    );
    if matches!(
        parameter.domain,
        BuiltinParameterDomain::DisabledOrRateKeyedHertz { .. }
    ) {
        out.push_str("{ ");
        for (index, rate) in LAUNCH_RATES_HZ.iter().enumerate() {
            append!(
                out,
                "\"{rate}\": {}{}",
                optional_number(builtin_filter_cutoff_maximum_hz(*rate)),
                if index + 1 == LAUNCH_RATES_HZ.len() {
                    ""
                } else {
                    ", "
                },
            );
        }
        out.push_str(" }");
    } else {
        out.push_str("null");
    }
    append!(
        out,
        ", \"default\": {}, \"updateRate\": \"{}\", \"smoothing\": \"{}\", \"reset\": \"{}\", \"disabledValue\": {}, \
\"liveUpdatable\": {}, \"step\": ",
        number(parameter.default),
        match parameter.update_rate {
            BuiltinParameterUpdateRate::PreparedOnly => "preparedOnly",
            BuiltinParameterUpdateRate::BlockTarget => "blockTarget",
        },
        match parameter.smoothing {
            BuiltinSmoothingPolicy::None => "none",
            BuiltinSmoothingPolicy::LinearNUpdates => "linearNUpdates",
            BuiltinSmoothingPolicy::Linear64CoefficientUpdates => "linear64CoefficientUpdates",
        },
        match parameter.reset {
            BuiltinParameterReset::RestorePreparedValue => "restorePreparedValue",
            BuiltinParameterReset::KeepTargetResetCurrent => "keepTargetResetCurrent",
        },
        optional_number(parameter.disabled_value),
        live,
    );
    step_object(out, parameter.lattice);
    out.push_str(" }");
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

/// Command-line entry point: `--write DIR`, `--check DIR`, `--print` or `--print-abi-layout`.
///
/// `--write`/`--check` cover **both** emitted documents. They are one tool because they are one
/// transcription discipline over one engine: splitting them into two binaries would let an
/// artifact directory hold a current parameter metadata beside a stale ABI layout, which is the
/// drift this tool exists to make impossible. `--print` keeps naming the parameter metadata alone
/// so every existing caller reads what it always read; the layout has its own print mode.
pub fn run() {
    let mut arguments = std::env::args().skip(1);
    let mode = arguments.next().unwrap_or_else(|| usage());
    match mode.as_str() {
        "--print" => {
            if arguments.next().is_some() {
                usage();
            }
            print!("{}", render());
        }
        "--print-abi-layout" => {
            if arguments.next().is_some() {
                usage();
            }
            print!("{}", abi_layout::render());
        }
        "--write" | "--check" => {
            let directory = PathBuf::from(arguments.next().unwrap_or_else(|| usage()));
            if arguments.next().is_some() {
                usage();
            }
            if !directory.is_dir() {
                eprintln!("{} is not a directory", directory.display());
                std::process::exit(2);
            }
            let documents = [
                (OUTPUT_NAME, render()),
                (abi_layout::OUTPUT_NAME, abi_layout::render()),
            ];
            for (name, document) in documents {
                let path = directory.join(name);
                if mode == "--write" {
                    let mut file = std::fs::File::create(&path).unwrap_or_else(|error| {
                        eprintln!("cannot create {}: {error}", path.display());
                        std::process::exit(2)
                    });
                    file.write_all(document.as_bytes()).expect("write document");
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
        }
        _ => usage(),
    }
}
