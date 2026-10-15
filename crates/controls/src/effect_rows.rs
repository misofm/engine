//! The rows of a native effect instance (decision 17, S2): its bypass, every parameter its
//! descriptor declares, and its link mode.
//!
//! The effect contract's [`ParameterDescriptor`] says what each parameter is, so this module
//! derives the control descriptor from it and adds no table of its own. An instance is a console
//! entry (section `Console(slot_id)`) or an insert (section `Inserts(effect_id)`); a `cid` effect
//! or an identity the launch registry does not carry has no rows.
//!
//! In an effect section the control `0` is the instance's bypass, a parameter row has the
//! parameter's ID, and an insert with a choice of link modes has [`LINK_MODE_CONTROL`].

use effect_compiler::{launch_native_effect_registry, lowers_session_bypass};
use effect_contract::control::PreparedReason;
use effect_contract::{
    AutomationRate, EffectDescriptor, LinkMode as ContractLinkMode, ObservationCost,
    ObservationDescriptor, ParameterChannelPolicy, ParameterDescriptor, ParameterDomain,
    ParameterLattice, ParameterMapping, ParameterUnit, normalize_zero,
};
use session::{
    ConsoleEntry, Effect, EffectIdentity, EffectParam, LinkMode, ParameterChannel,
    ParameterUnit as SessionUnit, SessionModel, StableId,
};

use crate::descriptor::{
    AutomationRule, BASE, Choice, ControlDescriptor, ControlScope, DomainRule, Lanes, Motion,
    ruled_live_not_built,
};
use crate::error::ControlError;
use crate::lane::{Side, is_on, toggle};
use crate::registry::{ControlRow, Resolved, push_control};
use crate::{ControlClass, ControlKind, Family, RebuildReason, Taper};
use session::{ControlAddress, Lane, Owner, Section};

/// The control ID of an effect's bypass.
pub const BYPASS_CONTROL: u32 = 0;

/// The control ID of an insert's link mode. No effect parameter ID reaches it (the registry test
/// refuses a descriptor that does).
pub const LINK_MODE_CONTROL: u32 = 0x8000_0000;

/// The delay's bypass is a prepared bypass until #1339 gives it a live shunt (decision 15 E4).
const DELAY_BYPASS_ISSUE: u32 = 1339;
/// The multiband compressor's bypass is a prepared bypass until #1340 (decision 15 E4).
const MULTIBAND_BYPASS_ISSUE: u32 = 1340;
/// The multiband compressor's link mode is live once #1367 lands (decision 15 E2).
const MULTIBAND_LINK_ISSUE: u32 = 1367;

const DELAY_ID: &str = "miso.delay";
const MULTIBAND_ID: &str = "miso.multiband-compressor";

/// Every link mode with the label a frontend shows, in code order. The choice lists below are
/// selected from it by the set bits of an effect's `supported_link_modes`, so a code or label is
/// written once.
const LINK_MODES: [(ContractLinkMode, &str); 3] = [
    (ContractLinkMode::DualMono, "dual_mono"),
    (ContractLinkMode::Maximum, "maximum"),
    (ContractLinkMode::Average, "average"),
];

/// The `N` choices whose mode bit is set in `bits` (bit `code - 1`, as `LinkModeSet` has it).
const fn link_choices_of<const N: usize>(bits: u32) -> [Choice; N] {
    let mut out = [Choice {
        value: 0.0,
        label: "",
    }; N];
    let (mut count, mut index) = (0, 0);
    while index < LINK_MODES.len() {
        let (mode, label) = LINK_MODES[index];
        if bits & (1 << (mode as u32 - 1)) != 0 {
            out[count] = Choice {
                value: mode as u32 as f32,
                label,
            };
            count += 1;
        }
        index += 1;
    }
    assert!(count == N, "the choice count is the number of set bits");
    out
}

const LINK_CHOICES_MAXIMUM: [Choice; 2] = link_choices_of(0b011);
const LINK_CHOICES_AVERAGE: [Choice; 2] = link_choices_of(0b101);
const LINK_CHOICES_ALL: [Choice; 3] = link_choices_of(0b111);

/// The launch descriptor of a native effect identity, or `None` for a `cid` effect, an unknown
/// effect or a registry that failed to build.
fn contract(identity: &EffectIdentity) -> Option<&'static EffectDescriptor> {
    let EffectIdentity::Native { effect_id } = identity else {
        return None;
    };
    let registry = launch_native_effect_registry().ok()?;
    registry
        .get_ascii(effect_id.as_str())
        .map(|factory| factory.descriptor())
}

/// Whether the instance `identity` has control rows at all: a native effect the launch registry
/// carries. A `cid` effect and an identity the registry lacks have none, so no row sees a change
/// to their values.
pub(crate) fn has_rows(identity: &EffectIdentity) -> bool {
    contract(identity).is_some()
}

/// Whether an insert of `identity` has a link-mode row: the effect supports a choice of modes.
pub(crate) fn has_link_mode_row(identity: &EffectIdentity) -> bool {
    contract(identity).is_some_and(|descriptor| link_choices(descriptor).is_some())
}

/// The numeric code of a link mode: the `effect_contract::LinkMode` code.
fn link_code(mode: LinkMode) -> f32 {
    match mode {
        LinkMode::DualMono => ContractLinkMode::DualMono as u32 as f32,
        LinkMode::Maximum => ContractLinkMode::Maximum as u32 as f32,
        LinkMode::Average => ContractLinkMode::Average as u32 as f32,
    }
}

fn link_mode_of(code: f32) -> Option<LinkMode> {
    [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
        .into_iter()
        .find(|mode| link_code(*mode) == code)
}

/// The choices of the link modes an effect supports, or `None` when it supports only one.
pub(crate) fn link_choices(descriptor: &EffectDescriptor) -> Option<&'static [Choice]> {
    match descriptor.supported_link_modes.bits() {
        3 => Some(&LINK_CHOICES_MAXIMUM),
        5 => Some(&LINK_CHOICES_AVERAGE),
        7 => Some(&LINK_CHOICES_ALL),
        _ => None,
    }
}

fn class_of(reason: Option<PreparedReason>) -> ControlClass {
    // The descriptor validator refuses a prepared parameter without a reason.
    let reason = reason.unwrap_or(PreparedReason {
        reason: RebuildReason::Correctness,
        reopen: "undeclared",
    });
    ControlClass::RebuildOnly {
        reason: reason.reason,
        reopen: reason.reopen,
    }
}

/// The class of one effect parameter's row. A row is `Live` only when the plan keeps a cell that
/// render reads each block, and that is a `Block`-rate parameter (`effect_contract::live`): a
/// `None`-rate parameter is prepared (its descriptor states why). The sample rate is retired
/// (#1636). A value is automatable only if it is live (decision 14 rule 4).
fn parameter_class(parameter: &ParameterDescriptor) -> ControlClass {
    match parameter.automation_rate {
        AutomationRate::Block => ControlClass::Live,
        AutomationRate::None => class_of(parameter.prepared_reason),
    }
}

/// The control descriptor of one effect parameter: everything comes from the contract's row.
pub(crate) fn parameter_descriptor(parameter: &'static ParameterDescriptor) -> ControlDescriptor {
    let (kind, minimum, maximum) = match parameter.domain {
        ParameterDomain::Continuous => (
            ControlKind::Continuous,
            parameter.minimum.unwrap_or(0.0),
            parameter.maximum.unwrap_or(0.0),
        ),
        ParameterDomain::Boolean => (ControlKind::Toggle, 0.0, 1.0),
        ParameterDomain::Enumeration => (
            ControlKind::Enumeration,
            parameter.enum_choices.first().map_or(0.0, |c| c.value),
            parameter.enum_choices.last().map_or(0.0, |c| c.value),
        ),
    };
    ControlDescriptor {
        name: parameter.name,
        display_name: parameter.display_name,
        unit: parameter.unit,
        display_unit: parameter.display_unit,
        kind,
        lanes: match parameter.channel_policy {
            ParameterChannelPolicy::PerLane => Lanes::PerLane,
            ParameterChannelPolicy::Shared => Lanes::Shared,
        },
        minimum,
        maximum,
        disabled_value: None,
        default: parameter.default_value,
        taper: match parameter.mapping {
            ParameterMapping::Linear if parameter.unit == ParameterUnit::Db => Taper::Decibel,
            ParameterMapping::Linear => Taper::Linear,
            ParameterMapping::Logarithmic => Taper::Logarithmic,
            ParameterMapping::Exponential => Taper::Exponential,
            ParameterMapping::Stepped => Taper::Stepped,
        },
        lattice: parameter.lattice,
        choices: parameter.enum_choices,
        motion: Motion::Fixed,
        class: parameter_class(parameter),
        scope: ControlScope::Model,
        family: Family::Effect,
        active: true,
        domain: DomainRule::Contract(parameter),
        automation: AutomationRule::Computed,
    }
}

/// The bypass row of an instance of `effect_id`.
pub(crate) fn bypass_descriptor(effect_id: &str) -> ControlDescriptor {
    // The one place that decides whether a session bypass is live is `lowers_session_bypass`; the
    // issue that flips each prepared bypass is a fact of the effect, kept beside it.
    let class = if lowers_session_bypass(effect_id) {
        ControlClass::Live
    } else {
        match effect_id {
            DELAY_ID => ruled_live_not_built(
                DELAY_BYPASS_ISSUE,
                "#1339 gives the delay a live bypass shunt; its last step flips the row",
            ),
            MULTIBAND_ID => ruled_live_not_built(
                MULTIBAND_BYPASS_ISSUE,
                "#1340 gives the multiband compressor a live bypass shunt; its last step flips the row",
            ),
            // An effect that gains a prepared bypass before root numbers its issue.
            _ => ruled_live_not_built(0, "a prepared bypass whose live path root has not numbered"),
        }
    };
    ControlDescriptor {
        name: "bypass",
        display_name: "Bypass",
        kind: ControlKind::Toggle,
        taper: Taper::Stepped,
        lattice: ParameterLattice::indices(),
        // The bypass crossfade is the one ramp with no per-edit length (#1341).
        motion: Motion::Fixed,
        class,
        family: Family::Effect,
        ..BASE
    }
}

/// The link-mode row of an insert of `effect_id` whose supported modes are `choices`.
pub(crate) fn link_mode_descriptor(
    effect_id: &str,
    choices: &'static [Choice],
) -> ControlDescriptor {
    let class = if effect_id == MULTIBAND_ID {
        ruled_live_not_built(
            MULTIBAND_LINK_ISSUE,
            "decision 15 E2, #1367 makes the link mode live",
        )
    } else {
        ControlClass::RebuildOnly {
            reason: RebuildReason::Optimisation,
            reopen: "decision 15 E2, #1368-#1371",
        }
    };
    ControlDescriptor {
        name: "link_mode",
        display_name: "Link Mode",
        kind: ControlKind::Enumeration,
        minimum: 1.0,
        maximum: choices.last().map_or(1.0, |choice| choice.value),
        default: 1.0,
        taper: Taper::Stepped,
        lattice: ParameterLattice::indices(),
        choices,
        motion: Motion::Fixed,
        class,
        family: Family::Effect,
        ..BASE
    }
}

/// The committed value of `parameter` on `side`: the lane's own entry, else the `both` entry,
/// else the default (the same resolution as preparation's `resolve_initial_values`).
fn resolved(params: &[EffectParam], parameter: &ParameterDescriptor, side: Side) -> f32 {
    let entry = |channel| {
        params
            .iter()
            .find(|item| item.parameter_id == parameter.id.0 && item.channel == channel)
            .map(|item| item.value)
    };
    let own = match parameter.channel_policy {
        ParameterChannelPolicy::Shared => None,
        ParameterChannelPolicy::PerLane => {
            entry(side.pick(ParameterChannel::Left, ParameterChannel::Right))
        }
    };
    normalize_zero(
        own.or_else(|| entry(ParameterChannel::Both))
            .unwrap_or(parameter.default_value),
    )
}

/// One effect instance as a listing sees it.
pub(crate) struct Instance<'a> {
    pub(crate) identity: &'a EffectIdentity,
    pub(crate) bypass: bool,
    /// `Some` for an insert; a console slot's link mode is a slot declaration and has no row.
    pub(crate) link_mode: Option<LinkMode>,
    pub(crate) params: &'a [EffectParam],
}

impl<'a> Instance<'a> {
    pub(crate) fn insert(effect: &'a Effect) -> Self {
        Self {
            identity: &effect.identity,
            bypass: effect.bypass,
            link_mode: Some(effect.link_mode),
            params: &effect.params,
        }
    }

    pub(crate) fn console(identity: &'a EffectIdentity, entry: &'a ConsoleEntry) -> Self {
        Self {
            identity,
            bypass: entry.bypass,
            link_mode: None,
            params: &entry.params,
        }
    }
}

/// Append the rows of one instance, in order: bypass, the parameters in descriptor order, then the
/// link mode of an insert with a choice of modes.
pub(crate) fn push_rows(
    out: &mut Vec<ControlRow>,
    strip: &StableId,
    section: &Section,
    instance: &Instance<'_>,
) {
    let Some(descriptor) = contract(instance.identity) else {
        return;
    };
    let owner = Owner::Strip(strip.clone());
    let effect_id = descriptor.id.as_str();
    push_control(
        out,
        &owner,
        section,
        BYPASS_CONTROL,
        &bypass_descriptor(effect_id),
        |_| toggle(instance.bypass),
    );
    for parameter in descriptor.parameters {
        push_control(
            out,
            &owner,
            section,
            parameter.id.0,
            &parameter_descriptor(parameter),
            |side| resolved(instance.params, parameter, side),
        );
    }
    if let (Some(mode), Some(choices)) = (instance.link_mode, link_choices(descriptor)) {
        push_control(
            out,
            &owner,
            section,
            LINK_MODE_CONTROL,
            &link_mode_descriptor(effect_id, choices),
            |_| link_code(mode),
        );
    }
}

/// Find one row of an instance by its control ID, as [`push_rows`] would list it: `None` for a
/// `cid` effect, an identity the registry lacks, or an ID the effect does not have.
pub(crate) fn resolve(instance: &Instance<'_>, control: u32) -> Option<Resolved> {
    let descriptor = contract(instance.identity)?;
    let effect_id = descriptor.id.as_str();
    Some(match control {
        BYPASS_CONTROL => {
            Resolved::stored(bypass_descriptor(effect_id), |_| toggle(instance.bypass))
        }
        LINK_MODE_CONTROL => {
            let (mode, choices) = (instance.link_mode?, link_choices(descriptor)?);
            Resolved::stored(link_mode_descriptor(effect_id, choices), |_| {
                link_code(mode)
            })
        }
        control => {
            let parameter = descriptor
                .parameters
                .iter()
                .find(|parameter| parameter.id.0 == control)?;
            Resolved::stored(parameter_descriptor(parameter), |side| {
                resolved(instance.params, parameter, side)
            })
        }
    })
}

/// The contract unit as the session spells it.
pub(crate) const fn session_unit(unit: ParameterUnit) -> SessionUnit {
    match unit {
        ParameterUnit::Db => SessionUnit::Db,
        ParameterUnit::Hz => SessionUnit::Hz,
        ParameterUnit::Milliseconds => SessionUnit::Milliseconds,
        ParameterUnit::Samples => SessionUnit::Samples,
        ParameterUnit::Linear => SessionUnit::Linear,
        ParameterUnit::Ratio => SessionUnit::Ratio,
    }
}

/// The parts of one instance that a write reaches.
struct InstanceMut<'a> {
    bypass: &'a mut bool,
    link_mode: Option<&'a mut LinkMode>,
    params: &'a mut Vec<EffectParam>,
}

/// The identity of the instance at `section` of strip `id`.
fn identity_of<'a>(
    model: &'a SessionModel,
    id: &StableId,
    section: &Section,
) -> Option<&'a EffectIdentity> {
    let strip = model.strips().find(|strip| strip.id == id)?;
    match section {
        Section::Console(slot) => model
            .console
            .slots()
            .find(|candidate| candidate.slot == *slot)
            .map(|candidate| &candidate.identity),
        Section::Inserts(effect) => strip
            .inserts
            .effects
            .iter()
            .find(|candidate| candidate.id == *effect)
            .map(|candidate| &candidate.identity),
        Section::Own | Section::Builtins => None,
    }
}

/// The declared observation tap `tap_id` of the effect at `section` of strip `id`, if the strip,
/// the instance and the tap exist and the tap is [`ObservationCost::Resident`] (a computed tap has
/// no subscription path). `None` for a `cid` effect, an unknown effect and a section that is not
/// an effect's.
pub(crate) fn resident_observation_tap(
    model: &SessionModel,
    id: &StableId,
    section: &Section,
    tap_id: u32,
) -> Option<&'static ObservationDescriptor> {
    let descriptor = identity_of(model, id, section).and_then(contract)?;
    descriptor
        .observations
        .iter()
        .find(|tap| tap.id.0 == tap_id && tap.cost == ObservationCost::Resident)
}

/// The mutable instance at `section` of strip `id`.
fn instance_mut<'a>(
    model: &'a mut SessionModel,
    id: &StableId,
    section: &Section,
) -> Option<InstanceMut<'a>> {
    let (console, inserts) = if let Some(track) = model.tracks.iter_mut().find(|t| &t.id == id) {
        (&mut track.console, &mut track.inserts.effects)
    } else {
        let submix = model.submixes.iter_mut().find(|s| &s.id == id)?;
        (&mut submix.console, &mut submix.inserts.effects)
    };
    match section {
        Section::Console(slot) => {
            let entry = console.iter_mut().find(|entry| entry.slot == *slot)?;
            Some(InstanceMut {
                bypass: &mut entry.bypass,
                link_mode: None,
                params: &mut entry.params,
            })
        }
        Section::Inserts(effect) => {
            let effect = inserts
                .iter_mut()
                .find(|candidate| candidate.id == *effect)?;
            Some(InstanceMut {
                bypass: &mut effect.bypass,
                link_mode: Some(&mut effect.link_mode),
                params: &mut effect.params,
            })
        }
        Section::Own | Section::Builtins => None,
    }
}

/// Write `value` to an effect row of strip `address.owner`. The caller has run [`crate::check`]
/// on the row's descriptor, so the value is in domain and the lane is one the row accepts.
///
/// # Errors
///
/// [`ControlError::AddressUnknown`] if the instance or the control does not exist.
pub(crate) fn write(
    model: &mut SessionModel,
    address: &ControlAddress,
    value: f32,
) -> Result<(), ControlError> {
    let Owner::Strip(strip) = &address.owner else {
        return Err(ControlError::AddressUnknown);
    };
    let descriptor = identity_of(model, strip, &address.section)
        .and_then(contract)
        .ok_or(ControlError::AddressUnknown)?;
    let mut instance =
        instance_mut(model, strip, &address.section).ok_or(ControlError::AddressUnknown)?;
    match address.control {
        BYPASS_CONTROL => *instance.bypass = is_on(value),
        LINK_MODE_CONTROL => {
            let target = instance
                .link_mode
                .as_deref_mut()
                .ok_or(ControlError::AddressUnknown)?;
            *target = link_mode_of(value).ok_or(ControlError::Domain)?;
        }
        control => {
            let parameter = descriptor
                .parameters
                .iter()
                .find(|parameter| parameter.id.0 == control)
                .ok_or(ControlError::AddressUnknown)?;
            write_parameter(
                instance.params,
                parameter,
                address.lane,
                normalize_zero(value),
            );
        }
    }
    Ok(())
}

/// Write one parameter value (decision 17, D5): a one-lane write on a `both` entry splits it into
/// explicit `left` and `right` entries, a `both` write updates what exists or adds a `both`
/// entry, and the parameters stay in canonical order (ID, then channel).
fn write_parameter(
    params: &mut Vec<EffectParam>,
    parameter: &ParameterDescriptor,
    lane: Lane,
    value: f32,
) {
    let id = parameter.id.0;
    let unit = session_unit(parameter.unit);
    let find = |params: &[EffectParam], channel| {
        params
            .iter()
            .position(|item| item.parameter_id == id && item.channel == channel)
    };
    let set = |params: &mut Vec<EffectParam>, channel, value| match find(params, channel) {
        Some(index) => params[index].value = value,
        None => params.push(EffectParam {
            parameter_id: id,
            channel,
            unit,
            value,
        }),
    };
    if parameter.channel_policy == ParameterChannelPolicy::Shared {
        set(params, ParameterChannel::Both, value);
    } else {
        let both = find(params, ParameterChannel::Both);
        let lanes = find(params, ParameterChannel::Left)
            .or_else(|| find(params, ParameterChannel::Right))
            .is_some();
        match lane {
            Lane::Both => {
                if both.is_some() || !lanes {
                    set(params, ParameterChannel::Both, value);
                }
                if lanes {
                    set(params, ParameterChannel::Left, value);
                    set(params, ParameterChannel::Right, value);
                }
            }
            Lane::Left | Lane::Right => {
                if let Some(index) = both {
                    // Split: the other lane keeps the value the `both` entry gave it.
                    let old = params.remove(index).value;
                    let other = if lane == Lane::Left {
                        ParameterChannel::Right
                    } else {
                        ParameterChannel::Left
                    };
                    set(params, other, old);
                }
                set(params, lane, value);
            }
        }
    }
    params.sort_by(|left, right| {
        left.parameter_id
            .cmp(&right.parameter_id)
            .then(left.channel.cmp(&right.channel))
    });
}
