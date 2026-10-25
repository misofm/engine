//! Build-time parameter-metadata codegen for the browser live controls (issue #137 D4).
//!
//! # Why this exists
//!
//! The app drives the engine through numeric IDs: a control row, a lane, an effect's parameter or
//! tap ID. Numbers are what a control frame carries, and deliberately so -- there is no string on
//! the hot path. But numbers are useless to a user interface without names, units,
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
//! consumer that kept its own table would go stale. Exactly four launch effects declare a
//! `sidechainInput` (`miso.compressor`, `miso.de-esser`, `miso.dynamic-eq` and
//! `miso.gate-expander`, each `"sidechain-in"`, each optional); the other six declare only their
//! main pair, and every effect gets the key.
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

/// JSON emission for the SDK documents (#382).
///
/// The documents are `Serialize` models written by `serde_json` as compact JSON, so
/// escaping, separators, nesting and whitespace are the serializer's, never a hand-placed comma.
/// [`Num`] writes a finite `f32` in [`super::number`]'s spelling through a raw value, so a
/// number's bytes never depend on the serializer's float printer.
mod json {
    use serde::ser::{Error as _, SerializeMap as _};
    use serde::{Serialize, Serializer};
    use serde_json::value::RawValue;

    /// A finite `f32`, spelled by [`super::number`].
    pub(crate) struct Num(pub(crate) f32);

    impl Serialize for Num {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            RawValue::from_string(super::number(self.0))
                .map_err(S::Error::custom)?
                .serialize(serializer)
        }
    }

    /// An object whose keys are data (an effect ID, a sample rate), in the given order.
    pub(crate) struct Entries<V>(pub(crate) Vec<(String, V)>);

    impl<V: Serialize> Serialize for Entries<V> {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut map = serializer.serialize_map(Some(self.0.len()))?;
            for (key, value) in &self.0 {
                map.serialize_entry(key, value)?;
            }
            map.end()
        }
    }

    /// Write `value` as one line of compact JSON with a trailing newline.
    pub(crate) fn document<T: Serialize>(value: &T) -> String {
        let mut text = serde_json::to_string(value).expect("an in-memory JSON write cannot fail");
        text.push('\n');
        text
    }
}

pub mod abi_layout;
pub mod response_frame;
pub mod session_keys;

use std::io::Write as _;
use std::path::PathBuf;

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
use host_web::ABI_VERSION;
use json::{Entries, Num};
use serde::Serialize;

/// The emitted file name, shipped beside the Wasm artifact.
pub const OUTPUT_NAME: &str = "miso-engine-v1-parameter-metadata.json";
/// The schema tag every consumer must check before reading a field.
pub const SCHEMA: &str = "miso.web.parameter-metadata.v1";
/// The launch sample rates a rate-keyed builtin domain is reported for.
pub const LAUNCH_RATES_HZ: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

fn usage() -> ! {
    eprintln!(
        "usage: parameter_metadata --write DIRECTORY | --check DIRECTORY \
| --print | --print-abi-layout | --print-session-keys | --print-response-frame"
    );
    std::process::exit(2)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Document<'a> {
    schema: &'a str,
    abi_version: u32,
    observation_vocabularies: ObservationVocabularies<'a>,
    builtins: Builtins<'a>,
    effects: Vec<Effect<'a>>,
    controls: Controls<'a>,
}

#[derive(Serialize)]
struct NamedValue<'a> {
    value: u32,
    name: &'a str,
}

#[derive(Serialize)]
struct ObservationVocabularies<'a> {
    kinds: Vec<NamedValue<'a>>,
    costs: Vec<NamedValue<'a>>,
    cadences: Vec<NamedValue<'a>>,
    folds: Vec<NamedValue<'a>>,
    channels: Vec<NamedValue<'a>>,
}

#[derive(Serialize)]
struct Builtins<'a> {
    response: Response<'a>,
    parameters: Vec<BuiltinParameter<'a>>,
}

fn named<'a>(rows: &[(u32, &'a str)]) -> Vec<NamedValue<'a>> {
    rows.iter()
        .map(|&(value, name)| NamedValue { value, name })
        .collect()
}

/// Render the whole document. Deterministic: registry order is `EffectId` order.
#[must_use]
pub fn render() -> String {
    let registry = launch_native_effect_registry().expect("launch effect registry");
    let document = Document {
        schema: SCHEMA,
        abi_version: ABI_VERSION,
        // Issue #143 D1: the observation vocabularies, so a consumer resolves a tap's raw `u32`s
        // the same way it resolves a parameter's -- from this document, never from a hand-written
        // table.
        observation_vocabularies: ObservationVocabularies {
            kinds: named(&[
                (ObservationKind::GainReductionDb as u32, "gainReductionDb"),
                (ObservationKind::GainChangeDb as u32, "gainChangeDb"),
            ]),
            costs: named(&[
                (ObservationCost::Resident as u32, "resident"),
                (ObservationCost::Computed as u32, "computed"),
            ]),
            cadences: named(&[
                (ObservationCadence::PerBlock as u32, "perBlock"),
                (ObservationCadence::PerWindow as u32, "perWindow"),
            ]),
            folds: named(&[
                (ObservationFold::Latest as u32, "latest"),
                (ObservationFold::PeakMagnitude as u32, "peakMagnitude"),
                (ObservationFold::PeakSigned as u32, "peakSigned"),
            ]),
            channels: named(&[
                (ObservationChannels::Shared as u32, "shared"),
                (ObservationChannels::PerLane as u32, "perLane"),
            ]),
        },
        builtins: Builtins {
            response: response_descriptor(builtins::input_filter_response_descriptor()),
            parameters: BUILTIN_PARAMETER_DESCRIPTORS
                .iter()
                .map(|parameter| builtin_parameter(parameter))
                .collect(),
        },
        effects: registry
            .descriptors()
            .map(|descriptor| {
                let response = registry
                    .get_ascii(descriptor.id.as_str())
                    .and_then(|factory| factory.response_analysis());
                effect(descriptor, response)
            })
            .collect(),
        controls: controls_object(registry),
    };
    json::document(&document)
}

/// The `controls` object (decision 17, slice S2b): the row tables of the `controls` crate, one
/// list per row source and one entry per launch effect.
///
/// Nothing here names a row. Every row comes from `controls::table_rows` or
/// `controls::effect_table_rows`, so a row added to a table reaches this document, and from it the
/// SDK's `controls.ts`, when the metadata is regenerated. A row's `name` is the declared machine
/// name; `displayName` is the label a person reads.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Controls<'a> {
    address: Address<'a>,
    strip_own: Vec<ControlRow<'a>>,
    builtins: Vec<ControlRow<'a>>,
    route_own: Vec<ControlRow<'a>>,
    vca_own: Vec<ControlRow<'a>>,
    session_own: Vec<ControlRow<'a>>,
    effects: Entries<EffectControls<'a>>,
}

/// The address vocabulary (decision 17, C2): the owner kinds, section kinds and lanes of a control
/// address, each with its stable code. The SDK builds its address types from it, so a kind the
/// core gains reaches the SDK when the assets are regenerated and not by a hand edit.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Address<'a> {
    owner_kinds: Vec<NamedCode<'a>>,
    section_kinds: Vec<NamedCode<'a>>,
    lanes: Vec<NamedCode<'a>>,
}

#[derive(Serialize)]
struct NamedCode<'a> {
    name: &'a str,
    code: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EffectControls<'a> {
    bypass: ControlRow<'a>,
    parameters: Vec<ControlRow<'a>>,
    link_mode: Option<ControlRow<'a>>,
    console_link_mode: Option<ControlRow<'a>>,
}

fn controls_object(registry: &effect_contract::NativeEffectRegistry) -> Controls<'static> {
    let codes = |entries: &[(&'static str, u8)]| {
        entries
            .iter()
            .map(|&(name, code)| NamedCode { name, code })
            .collect()
    };
    let rows = |source| {
        table_rows(source)
            .iter()
            .map(|row| control_row(row))
            .collect()
    };
    Controls {
        address: Address {
            owner_kinds: codes(&OwnerKind::ALL.map(|kind| (kind.name(), kind.code()))),
            section_kinds: codes(&SectionKind::ALL.map(|kind| (kind.name(), kind.code()))),
            lanes: codes(
                &Lane::ALL
                    .iter()
                    .map(|(lane, name)| (*name, lane.wire()))
                    .collect::<Vec<_>>(),
            ),
        },
        strip_own: rows(RowSource::StripOwn),
        builtins: rows(RowSource::Builtins),
        route_own: rows(RowSource::RouteOwn),
        vca_own: rows(RowSource::VcaOwn),
        session_own: rows(RowSource::SessionOwn),
        effects: Entries(
            registry
                .descriptors()
                .map(|descriptor| {
                    let EffectTableRows {
                        bypass,
                        parameters,
                        link_mode,
                        console_link_mode,
                    } = effect_table_rows(descriptor.id.as_str())
                        .expect("a registry effect has rows");
                    let effect = EffectControls {
                        bypass: control_row(&bypass),
                        parameters: parameters.iter().map(|row| control_row(row)).collect(),
                        link_mode: link_mode.as_ref().map(|row| control_row(row)),
                        console_link_mode: console_link_mode.as_ref().map(|row| control_row(row)),
                    };
                    (descriptor.id.as_str().to_owned(), effect)
                })
                .collect(),
        ),
    }
}

/// One row of a table.
///
/// The fields that depend on the rate are emitted per launch rate when they differ across the four
/// (`maximum`); the fields that depend on a route's destination and tap are emitted per case
/// (`intoSubmix`; `intoOutputBeforeFader`, a route into the output from a tap before the fader,
/// #1391; `intoOutput`, from `post_fader` or `post_pan`), each case carrying the class and whether
/// the row applies.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ControlRow<'a> {
    control: u32,
    name: &'a str,
    display_name: &'a str,
    display_unit: &'a str,
    unit: &'a str,
    kind: &'a str,
    minimum: Num,
    maximum: RowMaximum,
    disabled_value: Option<Num>,
    default: Num,
    taper: &'a str,
    step: Step<'a>,
    choices: Vec<Choice<'a>>,
    ramp: &'a str,
    scope: &'a str,
    family: &'a str,
    lanes: &'a str,
    #[serde(flatten)]
    class: RowClass<'a>,
}

/// One `maximum`, or one per launch rate when they differ.
#[derive(Serialize)]
#[serde(untagged)]
enum RowMaximum {
    One(Num),
    ByRate(Entries<Num>),
}

/// A route row carries its class per destination; every other row carries it in place.
#[derive(Serialize)]
#[serde(untagged)]
enum RowClass<'a> {
    #[serde(rename_all = "camelCase")]
    Route {
        into_submix: ClassCase<'a>,
        into_output_before_fader: ClassCase<'a>,
        into_output: ClassCase<'a>,
    },
    #[serde(rename_all = "camelCase")]
    InPlace {
        #[serde(flatten)]
        fields: ClassFields<'a>,
        #[serde(skip_serializing_if = "Option::is_none")]
        on_submix: Option<bool>,
    },
}

/// A route row's class under one destination.
#[derive(Serialize)]
struct ClassCase<'a> {
    #[serde(flatten)]
    fields: ClassFields<'a>,
    active: bool,
}

/// `class`, `reason`, `reasonIssue`, `reopen` and `automatable` of a row under one class.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ClassFields<'a> {
    class: &'a str,
    reason: Option<&'a str>,
    reason_issue: Option<u32>,
    reopen: Option<&'a str>,
    automatable: bool,
}

#[derive(Serialize)]
struct Choice<'a> {
    value: Num,
    label: &'a str,
}

fn control_row(row: &TableRow) -> ControlRow<'static> {
    let descriptor = &row.descriptor;
    let maxima = LAUNCH_RATES_HZ.map(|rate| row.at_rate(rate).maximum);
    let maximum = if maxima
        .iter()
        .all(|maximum| maximum.to_bits() == maxima[0].to_bits())
    {
        RowMaximum::One(Num(maxima[0]))
    } else {
        RowMaximum::ByRate(Entries(
            LAUNCH_RATES_HZ
                .iter()
                .zip(maxima)
                .map(|(rate, maximum)| (rate.to_string(), Num(maximum)))
                .collect(),
        ))
    };
    let class = match (row.into_output, row.into_output_before_fader) {
        (Some(output), Some(before_fader)) => RowClass::Route {
            into_submix: ClassCase {
                fields: class_fields(descriptor, descriptor.class),
                active: true,
            },
            into_output_before_fader: ClassCase {
                fields: class_fields(descriptor, before_fader.class),
                active: before_fader.active,
            },
            into_output: ClassCase {
                fields: class_fields(descriptor, output.class),
                active: output.active,
            },
        },
        _ => RowClass::InPlace {
            fields: class_fields(descriptor, descriptor.class),
            on_submix: (!row.on_submix).then_some(false),
        },
    };
    ControlRow {
        control: row.control,
        name: descriptor.name,
        display_name: descriptor.display_name,
        display_unit: descriptor.display_unit,
        unit: unit_name(descriptor.unit),
        kind: control_kind_name(descriptor.kind),
        minimum: Num(descriptor.minimum),
        maximum,
        disabled_value: descriptor.disabled_value.map(Num),
        default: Num(descriptor.default),
        taper: taper_name(descriptor.taper),
        step: step_object(descriptor.lattice),
        choices: descriptor
            .choices
            .iter()
            .map(|choice| Choice {
                value: Num(choice.value),
                label: choice.label,
            })
            .collect(),
        ramp: match descriptor.motion {
            Motion::Ramp(ramp) => ramp_name(ramp),
            Motion::Fixed => "fixed",
        },
        scope: match descriptor.scope {
            ControlScope::Model => "model",
            ControlScope::Monitor => "monitor",
        },
        family: family_name(descriptor.family),
        lanes: match descriptor.lanes {
            controls::Lanes::PerLane => "perLane",
            controls::Lanes::Shared => "shared",
        },
        class,
    }
}

/// The class fields of a row under `class`.
///
/// `automatable` is the descriptor's own computation, never a second rule (a live model value that
/// render reads).
fn class_fields(descriptor: &ControlDescriptor, class: ControlClass) -> ClassFields<'static> {
    let under = ControlDescriptor {
        class,
        ..*descriptor
    };
    let (name, reason, reason_issue, reopen) = match class {
        ControlClass::Live => ("live", None, None, None),
        ControlClass::Derived => ("derived", None, None, None),
        ControlClass::RebuildOnly { reason, reopen } => {
            let (name, issue) = rebuild_reason(reason);
            ("rebuildOnly", Some(name), issue, Some(reopen))
        }
    };
    ClassFields {
        class: name,
        reason,
        reason_issue,
        reopen,
        automatable: under.automatable(),
    }
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Effect<'a> {
    id: &'a str,
    display_name: &'a str,
    contract_major: u16,
    contract_minor: u16,
    state_layout_version: u32,
    parameters: Vec<EffectParameter<'a>>,
    /// Issue #278: the declared port table, in the descriptor's own order (role ascending), and
    /// never absent -- `[]` for an effect that declares no port at all, for the same reason
    /// `observations` is never absent. Until this shipped, the document published every effect's
    /// parameters and taps but nothing about its ports, so `sidechain.port_id` was the one session
    /// field an SDK builder could not check before boot: a misspelling parsed, validated, compiled
    /// and then failed preparation with `effect.sidechain.unknown_port`. The engine's refusal is
    /// unmoved; this is what lets an authoring layer refuse the same thing first, by name.
    ports: Vec<Port<'a>>,
    /// Issue #143: never absent. An effect that declares no tap emits `[]`, so a consumer reads one
    /// shape for every effect and "this build has no menu for that effect" is impossible to
    /// confuse with "this document predates observation".
    observations: Vec<Observation<'a>>,
    response: Option<Response<'a>>,
}

fn effect<'a>(
    descriptor: &'a EffectDescriptor,
    response: Option<&dyn NativeEffectResponseFactory>,
) -> Effect<'a> {
    Effect {
        id: descriptor.id.as_str(),
        display_name: descriptor.display_name,
        contract_major: descriptor.contract_major,
        contract_minor: descriptor.contract_minor,
        state_layout_version: descriptor.state_layout_version,
        parameters: descriptor.parameters.iter().map(effect_parameter).collect(),
        ports: descriptor
            .ports
            .iter()
            .map(|port| effect_port(port))
            .collect(),
        observations: descriptor
            .observations
            .iter()
            .map(|observation| effect_observation(observation))
            .collect(),
        response: response.map(|factory| response_descriptor(factory.analysis_descriptor())),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Response<'a> {
    id: u32,
    name: &'a str,
    axis_unit: &'a str,
    unit: &'a str,
    amplitude_reference: &'a str,
    channels: &'a str,
    mode: &'a str,
    cadence: &'a str,
    section_output: &'a str,
    cost: &'a str,
    floor_db: Num,
    total_scope: &'a str,
    bypass: &'a str,
    sections: Vec<ResponseSection<'a>>,
}

#[derive(Serialize)]
struct ResponseSection<'a> {
    id: u32,
    name: &'a str,
}

fn response_descriptor(descriptor: &ResponseAnalysisDescriptor) -> Response<'static> {
    Response {
        id: descriptor.id,
        name: descriptor.name,
        axis_unit: response_axis_unit(descriptor.axis_unit),
        unit: response_axis_unit(descriptor.unit),
        amplitude_reference: response_amplitude_reference(descriptor.amplitude_reference),
        channels: response_channels(descriptor.channels),
        mode: response_mode(descriptor.mode),
        cadence: response_cadence(descriptor.cadence),
        section_output: response_section_output(descriptor.section_output),
        cost: "computed",
        floor_db: Num(descriptor.floor_db),
        total_scope: response_scope(descriptor.total_scope),
        bypass: response_bypass(descriptor.bypass),
        sections: descriptor
            .sections
            .iter()
            .map(|section| ResponseSection {
                id: section.id,
                name: section.name,
            })
            .collect(),
    }
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

/// One declared port row.
///
/// Both the raw scalar and its name ride, exactly as they do on a parameter's `unit` and an
/// observation's `kind`: the descriptor wire and the C inspect record carry the `u32`, and a
/// consumer that resolves it from this document never keeps a second table of its own.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Port<'a> {
    id: &'a str,
    role: u32,
    role_name: &'a str,
    required: bool,
    layout: u32,
    layout_name: &'a str,
}

fn effect_port(port: &PortDescriptor) -> Port<'_> {
    Port {
        id: port.id.as_str(),
        role: port.role as u32,
        role_name: port_role_name(port.role),
        required: port.required,
        layout: port.layout as u32,
        layout_name: port_layout_name(port.layout),
    }
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Observation<'a> {
    id: u32,
    name: &'a str,
    display_unit: &'a str,
    kind: u32,
    kind_name: &'a str,
    unit: u32,
    unit_name: &'a str,
    cost: u32,
    cost_name: &'a str,
    cadence: u32,
    cadence_name: &'a str,
    fold: u32,
    fold_name: &'a str,
    channels: u32,
    channels_name: &'a str,
    minimum: Num,
    maximum: Num,
    subscribable: bool,
}

fn effect_observation(observation: &ObservationDescriptor) -> Observation<'static> {
    // `subscribable` is derived from the cost class exactly as `liveUpdatable` is derived from
    // `automatable`: a `Resident` tap is a copy out of state the block already wrote and the
    // subscribe path binds it; a `Computed` tap has no implementation in V1 and the subscribe path
    // answers `unsupportedKind`. The two statements are the same statement, which is why this is
    // derived rather than written down -- and why the schema gate refuses a computed tap that
    // claims to be subscribable.
    let subscribable = matches!(observation.cost, ObservationCost::Resident);
    Observation {
        id: observation.id.0,
        name: observation.display_name,
        display_unit: observation.display_unit,
        kind: observation.kind as u32,
        kind_name: observation_kind_name(observation.kind),
        unit: observation.unit as u32,
        unit_name: unit_name(observation.unit),
        cost: observation.cost as u32,
        cost_name: observation_cost_name(observation.cost),
        cadence: observation.cadence as u32,
        cadence_name: observation_cadence_name(observation.cadence),
        fold: observation.fold as u32,
        fold_name: observation_fold_name(observation.fold),
        channels: observation.channels as u32,
        channels_name: observation_channels_name(observation.channels),
        minimum: Num(observation.minimum),
        maximum: Num(observation.maximum),
        subscribable,
    }
}

const fn observation_kind_name(kind: ObservationKind) -> &'static str {
    match kind {
        ObservationKind::GainReductionDb => "gainReductionDb",
        ObservationKind::GainChangeDb => "gainChangeDb",
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
        ObservationFold::PeakSigned => "peakSigned",
    }
}

const fn observation_channels_name(channels: ObservationChannels) -> &'static str {
    match channels {
        ObservationChannels::Shared => "shared",
        ObservationChannels::PerLane => "perLane",
    }
}

/// One parameter's `step` declaration.
///
/// This is the #242 lattice authority as it crosses the agent/SDK boundary: a decimal string, not
/// a float, so a catalog round-trip is lossless. `size` is the shortest spelling that names the
/// declared `f32` exactly -- a ratio row's authority really is the decimal `1.02`, and rendering
/// it at the row's own eight-decimal value precision would spell it `1.01999998`.
#[derive(Serialize)]
struct Step<'a> {
    unit: &'a str,
    size: String,
    precision: u8,
    ladder: Ladder,
}

#[derive(Serialize)]
struct Ladder {
    xs: u8,
    sm: u8,
    md: u8,
    lg: u8,
    xl: u8,
}

fn step_object(lattice: ParameterLattice) -> Step<'static> {
    Step {
        unit: step_unit_name(lattice.step_unit),
        size: lattice.step.to_string(),
        precision: lattice.precision,
        ladder: Ladder {
            xs: lattice.ladder.multiple(StepSize::Xs),
            sm: lattice.ladder.multiple(StepSize::Sm),
            md: lattice.ladder.multiple(StepSize::Md),
            lg: lattice.ladder.multiple(StepSize::Lg),
            xl: lattice.ladder.multiple(StepSize::Xl),
        },
    }
}

const fn step_unit_name(unit: StepUnit) -> &'static str {
    match unit {
        StepUnit::Absolute => "absolute",
        StepUnit::Cents => "cents",
        StepUnit::Ratio => "ratio",
        StepUnit::Index => "index",
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EffectParameter<'a> {
    id: u32,
    name: &'a str,
    display_unit: &'a str,
    unit: u32,
    unit_name: &'a str,
    domain: u32,
    domain_name: &'a str,
    minimum: Option<Num>,
    maximum: Option<Num>,
    default: Num,
    mapping: u32,
    mapping_name: &'a str,
    automation_rate: u32,
    automation_rate_name: &'a str,
    channel_policy: u32,
    channel_policy_name: &'a str,
    smoothing: u32,
    smoothing_name: &'a str,
    smoothing_samples: u32,
    readable: bool,
    automatable: bool,
    /// Issue #140 A: the live-control command path feeds an admitted parameter into the running
    /// plan as a `PreparedAutomationSpan`, so a parameter is live exactly when its own descriptor
    /// says it can be automated. The two statements are the same statement, which is why this is
    /// derived from `automatable` rather than written down. A parameter that declares
    /// `AutomationRate::None` has no span the effect would accept and stays `false`.
    live_updatable: bool,
    enum_choices: Vec<Choice<'a>>,
    /// Issue #242: the lattice declaration this row's legal persisted values are generated from.
    step: Step<'a>,
}

fn effect_parameter(parameter: &ParameterDescriptor) -> EffectParameter<'static> {
    EffectParameter {
        id: parameter.id.0,
        name: parameter.display_name,
        display_unit: parameter.display_unit,
        unit: parameter.unit as u32,
        unit_name: unit_name(parameter.unit),
        domain: parameter.domain as u32,
        domain_name: domain_name(parameter.domain),
        minimum: parameter.minimum.map(Num),
        maximum: parameter.maximum.map(Num),
        default: Num(parameter.default_value),
        mapping: parameter.mapping as u32,
        mapping_name: mapping_name(parameter.mapping),
        automation_rate: parameter.automation_rate as u32,
        automation_rate_name: automation_rate_name(parameter.automation_rate),
        channel_policy: parameter.channel_policy as u32,
        channel_policy_name: channel_policy_name(parameter.channel_policy),
        smoothing: parameter.smoothing as u32,
        smoothing_name: smoothing_name(parameter.smoothing),
        smoothing_samples: parameter.smoothing_samples,
        readable: parameter.readable,
        automatable: parameter.automatable,
        live_updatable: parameter.automatable,
        enum_choices: parameter
            .enum_choices
            .iter()
            .map(|choice| Choice {
                value: Num(choice.value),
                label: choice.label,
            })
            .collect(),
        step: step_object(parameter.lattice),
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BuiltinParameter<'a> {
    id: u32,
    name: &'a str,
    unit: u32,
    unit_name: &'a str,
    scope: &'a str,
    mapping: &'a str,
    domain: &'a str,
    minimum: Option<Num>,
    maximum: Option<Num>,
    maximum_by_rate: Option<Entries<Option<Num>>>,
    default: Num,
    update_rate: &'a str,
    smoothing: &'a str,
    reset: &'a str,
    disabled_value: Option<Num>,
    live_updatable: bool,
    step: Step<'a>,
}

fn builtin_parameter(parameter: &BuiltinParameterDescriptor) -> BuiltinParameter<'static> {
    let unit = builtin_parameter_unit(parameter);
    // A rate-keyed cutoff has no single maximum: `builtin_filter_cutoff_maximum_hz` gives one
    // per launch rate, so the row carries the exact `f32` for each rather than a number that would
    // be wrong at three of the four.
    let (minimum, maximum, domain) = match parameter.domain {
        BuiltinParameterDomain::BooleanExact => (None, None, "booleanExact"),
        BuiltinParameterDomain::FiniteInclusive { minimum, maximum } => {
            (Some(minimum), Some(maximum), "finiteInclusive")
        }
        BuiltinParameterDomain::DisabledOrRateKeyedHertz { minimum_hz, .. } => {
            (Some(minimum_hz), None, "disabledOrRateKeyedHertz")
        }
    };
    let maximum_by_rate = matches!(
        parameter.domain,
        BuiltinParameterDomain::DisabledOrRateKeyedHertz { .. }
    )
    .then(|| {
        Entries(
            LAUNCH_RATES_HZ
                .iter()
                .map(|rate| {
                    (
                        rate.to_string(),
                        builtin_filter_cutoff_maximum_hz(*rate).map(Num),
                    )
                })
                .collect(),
        )
    });
    BuiltinParameter {
        id: parameter.id,
        name: parameter.name,
        unit: unit as u32,
        unit_name: unit_name(unit),
        scope: match parameter.scope {
            BuiltinParameterScope::PerLane => "perLane",
            BuiltinParameterScope::MatrixShared => "matrixShared",
        },
        mapping: match parameter.mapping {
            BuiltinParameterMapping::Boolean => "boolean",
            BuiltinParameterMapping::DecibelAmplitude => "decibelAmplitude",
            BuiltinParameterMapping::Hertz => "hertz",
            BuiltinParameterMapping::Linear => "linear",
        },
        domain,
        minimum: minimum.map(Num),
        maximum: maximum.map(Num),
        maximum_by_rate,
        default: Num(parameter.default),
        update_rate: match parameter.update_rate {
            BuiltinParameterUpdateRate::PreparedOnly => "preparedOnly",
            BuiltinParameterUpdateRate::BlockTarget => "blockTarget",
        },
        smoothing: match parameter.smoothing {
            BuiltinSmoothingPolicy::None => "none",
            BuiltinSmoothingPolicy::LinearNUpdates => "linearNUpdates",
            BuiltinSmoothingPolicy::Linear64CoefficientUpdates => "linear64CoefficientUpdates",
        },
        reset: match parameter.reset {
            BuiltinParameterReset::RestorePreparedValue => "restorePreparedValue",
            BuiltinParameterReset::KeepTargetResetCurrent => "keepTargetResetCurrent",
        },
        disabled_value: parameter.disabled_value.map(Num),
        // `matrix_ll/lr/rl/rr` are the only builtin parameters the ABI declares as `BlockTarget`,
        // and they are exactly the ones the live-control command path applies. The two statements
        // are the same statement, which is why this is derived from the descriptor rather than
        // written down.
        live_updatable: matches!(
            parameter.update_rate,
            BuiltinParameterUpdateRate::BlockTarget
        ),
        step: step_object(parameter.lattice),
    }
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

/// Command-line entry point: `--write DIR`, `--check DIR`, `--print`, `--print-abi-layout`,
/// `--print-session-keys` or `--print-response-frame`.
///
/// `--write`/`--check` cover **both** emitted documents. They are one tool because they are one
/// transcription discipline over one engine: splitting them into two binaries would let an
/// artifact directory hold a current parameter metadata beside a stale ABI layout, which is the
/// drift this tool exists to make impossible. `--print` keeps naming the parameter metadata alone
/// so every existing caller reads what it always read; the layout has its own print mode.
///
/// `--print-session-keys` is the SDK's session key tables (issue #1569). It is an SDK source asset,
/// not a member of the release artifact set, so `--write`/`--check` leave it out and
/// `sdk/codegen/assets.mjs` renders it itself. `--print-response-frame` is the same kind of asset:
/// the `RESPONSE_GET` frame table of the protocol crate (issue #1623).
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
        "--print-session-keys" => {
            if arguments.next().is_some() {
                usage();
            }
            print!("{}", session_keys::render());
        }
        "--print-response-frame" => {
            if arguments.next().is_some() {
                usage();
            }
            print!("{}", response_frame::render());
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
