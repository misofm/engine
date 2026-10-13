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

use effect_compiler::launch_native_effect_registry;
use effect_contract::control::PreparedReason;
use effect_contract::{
    AutomationRate, EffectDescriptor, LinkMode as ContractLinkMode, ParameterChannelPolicy,
    ParameterDescriptor, ParameterDomain, ParameterLattice, ParameterMapping, ParameterUnit,
    normalize_zero,
};
use session::{
    ConsoleEntry, Effect, EffectIdentity, EffectParam, LinkMode, ParameterChannel,
    ParameterUnit as SessionUnit, SessionModel, StableId,
};

use crate::address::{ControlAddress, Lane, Owner, Section};
use crate::descriptor::{
    BASE, Choice, ControlDescriptor, ControlScope, Lanes, Motion, ruled_live_not_built,
};
use crate::error::ControlError;
use crate::lane::{Side, is_on, toggle};
use crate::registry::{ControlRow, push_control};
use crate::{ControlClass, ControlKind, Family, RebuildReason, Taper};

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

const LINK_CHOICES_MAXIMUM: [Choice; 2] = [
    Choice {
        value: 1.0,
        label: "dual_mono",
    },
    Choice {
        value: 2.0,
        label: "maximum",
    },
];
const LINK_CHOICES_AVERAGE: [Choice; 2] = [
    Choice {
        value: 1.0,
        label: "dual_mono",
    },
    Choice {
        value: 3.0,
        label: "average",
    },
];
const LINK_CHOICES_ALL: [Choice; 3] = [
    Choice {
        value: 1.0,
        label: "dual_mono",
    },
    Choice {
        value: 2.0,
        label: "maximum",
    },
    Choice {
        value: 3.0,
        label: "average",
    },
];

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
        class: if parameter.automation_rate == AutomationRate::None {
            class_of(parameter.prepared_reason)
        } else {
            ControlClass::Live
        },
        scope: ControlScope::Model,
        family: Family::Effect,
        active: true,
    }
}

/// The bypass row of an instance of `effect_id`.
pub(crate) fn bypass_descriptor(effect_id: &str) -> ControlDescriptor {
    let class = match effect_id {
        DELAY_ID => ruled_live_not_built(
            DELAY_BYPASS_ISSUE,
            "#1339 gives the delay a live bypass shunt; its last step flips the row",
        ),
        MULTIBAND_ID => ruled_live_not_built(
            MULTIBAND_BYPASS_ISSUE,
            "#1340 gives the multiband compressor a live bypass shunt; its last step flips the row",
        ),
        _ => ControlClass::Live,
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

/// The contract unit as the session spells it.
const fn session_unit(unit: ParameterUnit) -> SessionUnit {
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
