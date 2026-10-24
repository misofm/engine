//! Decision 17, S2: every native effect parameter, bypass and link mode is a control row, with the
//! row's class, order and domain taken from the effect contract and the rulings.

mod common;

use std::collections::BTreeSet;

use common::{LAUNCH_EFFECTS, RATE, address, effects_fixture, id, insert_id, insert_row};
use controls::{
    BYPASS_CONTROL, ControlAddress, ControlClass, ControlError, ControlRow, LINK_MODE_CONTROL,
    Lane, MonitorOverlay, Owner, RebuildReason, Section, check, get, list, set,
};
use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    AutomationRate, EffectDescriptor, ParameterChannelPolicy, ParameterUnit, parameter_value_valid,
};
use session::{
    EffectParam, ParameterChannel, ParameterUnit as SessionUnit, SessionModel,
    canonical_session_json, parse_session_json,
};

fn descriptor_of(effect: &str) -> &'static EffectDescriptor {
    launch_native_effect_registry()
        .expect("the launch registry builds")
        .get_ascii(effect)
        .expect("a launch effect")
        .descriptor()
}

fn rows(model: &SessionModel) -> Vec<ControlRow> {
    list(model, &MonitorOverlay::default(), RATE).collect()
}

/// The lanes a row of this policy is listed on.
fn listed(policy: ParameterChannelPolicy) -> &'static [Lane] {
    match policy {
        ParameterChannelPolicy::PerLane => &[Lane::Left, Lane::Right],
        ParameterChannelPolicy::Shared => &[Lane::Both],
    }
}

/// The addresses of one effect instance, written out from the contract descriptor and not from the
/// registry's listing: the bypass, the parameters in descriptor order, then the link mode when the
/// effect supports more than one.
fn expected_instance(
    owner: &Owner,
    section: &Section,
    effect: &str,
    link_mode_row: bool,
) -> Vec<ControlAddress> {
    let descriptor = descriptor_of(effect);
    let at = |control, lane| ControlAddress {
        owner: owner.clone(),
        section: section.clone(),
        control,
        lane,
    };
    let mut out = vec![at(BYPASS_CONTROL, Lane::Both)];
    for parameter in descriptor.parameters {
        for &lane in listed(parameter.channel_policy) {
            out.push(at(parameter.id.0, lane));
        }
    }
    if link_mode_row && descriptor.supported_link_modes.bits().count_ones() > 1 {
        out.push(at(LINK_MODE_CONTROL, Lane::Both));
    }
    out
}

/// Red if the launch registry gains or loses an effect and this file's fixture list does not, so
/// every per-effect test below covers exactly the effects that ship.
#[test]
fn the_fixture_effect_list_is_the_launch_registry() {
    let registry = launch_native_effect_registry().expect("the launch registry builds");
    let shipped: BTreeSet<&str> = registry.descriptors().map(|d| d.id.as_str()).collect();
    let tested: BTreeSet<&str> = LAUNCH_EFFECTS.into_iter().collect();
    assert_eq!(tested, shipped);
    assert_eq!(registry.len(), LAUNCH_EFFECTS.len());
}

/// Gate 1, shape. Red if an effect parameter, bypass or link mode is not a control, is listed
/// twice, is out of the ruled order, or a console slot grows a link-mode row.
#[test]
fn every_instance_lists_its_bypass_parameters_and_link_mode_once_in_order() {
    let model = effects_fixture();
    let all = rows(&model);
    for strip in ["vocal", "guitar", "bus"] {
        let owner = Owner::Strip(id(strip));
        // Chain order (ruling C4): the pre-insert slot, the inserts, the post-insert slot.
        let mut expected = expected_instance(
            &owner,
            &Section::Console(id("glue")),
            "miso.compressor",
            false,
        );
        for effect in LAUNCH_EFFECTS {
            expected.extend(expected_instance(
                &owner,
                &Section::Inserts(insert_id(effect)),
                effect,
                true,
            ));
        }
        expected.extend(expected_instance(
            &owner,
            &Section::Console(id("ceiling")),
            "miso.true-peak-limiter",
            false,
        ));
        let actual: Vec<ControlAddress> = all
            .iter()
            .filter(|row| {
                row.address.owner == owner
                    && matches!(
                        row.address.section,
                        Section::Console(_) | Section::Inserts(_)
                    )
            })
            .map(|row| row.address.clone())
            .collect();
        assert_eq!(actual, expected, "{strip}");
    }
    let unique: BTreeSet<String> = all.iter().map(|row| format!("{:?}", row.address)).collect();
    assert_eq!(unique.len(), all.len(), "an address is listed twice");
    // The effect rows follow the strip's builtin rows, console first.
    let vocal: Vec<&ControlRow> = all
        .iter()
        .filter(|row| row.address.owner == Owner::Strip(id("vocal")))
        .collect();
    let sections: Vec<u8> = vocal.iter().map(|row| row.address.section.code()).collect();
    let first_effect = sections.iter().position(|&code| code == 5).unwrap();
    assert!(
        sections[..first_effect]
            .iter()
            .all(|&code| code == 6 || code == 4)
    );
    assert!(
        sections[first_effect..]
            .iter()
            .all(|&code| code == 5 || code == 2)
    );
}

/// A `cid` effect and an unknown identity have no rows, and a console slot has no link-mode row.
/// Red if an effect that the launch registry does not carry is listed anyway.
#[test]
fn an_unknown_or_third_party_effect_has_no_rows() {
    let mut model = effects_fixture();
    let track = &mut model.tracks[0];
    track.inserts.effects[0].identity = session::EffectIdentity::ThirdPartyCid {
        cid: "bafkreiaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
    };
    track.inserts.effects[1].identity = session::EffectIdentity::Native {
        effect_id: id("no-such-effect"),
    };
    let owner = Owner::Strip(track.id.clone());
    let unlisted = [
        track.inserts.effects[0].id.clone(),
        track.inserts.effects[1].id.clone(),
    ];
    assert!(rows(&model).iter().all(|row| {
        row.address.owner != owner
            || !matches!(&row.address.section, Section::Inserts(effect) if unlisted.contains(effect))
    }));
}

/// The classes the rulings give, written out per row: decision 14's limiter lookahead and decision
/// 15 E2's EQ and crossover rows, E4's two bypass shunts and #1367's link mode.
fn expected_prepared(effect: &str, parameter: u32) -> Option<RebuildReason> {
    match (effect, parameter) {
        ("miso.true-peak-limiter", 3) => Some(RebuildReason::Glitch),
        ("miso.parametric-eq", 1 | 2 | 17 | 18 | 33 | 34 | 49 | 50) => {
            Some(RebuildReason::RuledLiveNotBuilt { issue: 1337 })
        }
        _ => None,
    }
}

fn rebuild_reason(class: ControlClass) -> Option<RebuildReason> {
    match class {
        ControlClass::RebuildOnly { reason, .. } => Some(reason),
        ControlClass::Live => None,
        ControlClass::Derived => panic!("an effect row is never derived"),
    }
}

/// Gate 1, classes. Red if an effect row carries a class that contradicts decision 14 or 15, if a
/// `None` parameter has no reason in its row, or if a parameter the rulings make live is
/// rebuild-only (or the reverse).
#[test]
fn effect_rows_carry_the_ruled_class() {
    let model = effects_fixture();
    let all = rows(&model);
    for effect in LAUNCH_EFFECTS {
        let descriptor = descriptor_of(effect);
        let section = Section::Inserts(insert_id(effect));
        let of = |control: u32| {
            all.iter()
                .find(|row| {
                    row.address.owner == Owner::Strip(id("vocal"))
                        && row.address.section == section
                        && row.address.control == control
                })
                .unwrap_or_else(|| panic!("{effect} {control}"))
        };
        for parameter in descriptor.parameters {
            let row = of(parameter.id.0);
            assert_eq!(
                rebuild_reason(row.descriptor.class),
                expected_prepared(effect, parameter.id.0),
                "{effect} parameter {}",
                parameter.id.0
            );
            // The contract's rate and the control's class agree: prepared iff `None`.
            assert_eq!(
                parameter.automation_rate == AutomationRate::None,
                rebuild_reason(row.descriptor.class).is_some()
            );
            if let ControlClass::RebuildOnly { reopen, .. } = row.descriptor.class {
                assert!(!reopen.is_empty());
            }
            assert_eq!(row.descriptor.automatable(), parameter.automatable);
        }
        let bypass = match effect {
            "miso.multiband-compressor" => Some(RebuildReason::RuledLiveNotBuilt { issue: 1340 }),
            _ => None,
        };
        assert_eq!(
            rebuild_reason(of(BYPASS_CONTROL).descriptor.class),
            bypass,
            "{effect} bypass"
        );
        let link = all.iter().find(|row| {
            row.address.section == section
                && row.address.owner == Owner::Strip(id("vocal"))
                && row.address.control == LINK_MODE_CONTROL
        });
        match (effect, link) {
            ("miso.multiband-compressor", Some(row)) => assert_eq!(
                rebuild_reason(row.descriptor.class),
                Some(RebuildReason::RuledLiveNotBuilt { issue: 1367 })
            ),
            (_, Some(row)) => assert_eq!(
                rebuild_reason(row.descriptor.class),
                Some(RebuildReason::Optimisation),
                "{effect}"
            ),
            (_, None) => assert_eq!(
                descriptor.supported_link_modes.bits().count_ones(),
                1,
                "{effect} supports several link modes and lists no row"
            ),
        }
    }
}

/// The registry test: no descriptor parameter may share an ID with the bypass or the link mode.
/// Red if an effect declares parameter `0` or one at or above [`LINK_MODE_CONTROL`].
#[test]
fn no_parameter_id_reaches_the_reserved_controls() {
    for effect in LAUNCH_EFFECTS {
        for parameter in descriptor_of(effect).parameters {
            assert!(
                parameter.id.0 != BYPASS_CONTROL && parameter.id.0 < LINK_MODE_CONTROL,
                "{effect} {}",
                parameter.id.0
            );
        }
    }
}

fn next_up(value: f32) -> f32 {
    if value == 0.0 {
        return f32::from_bits(1);
    }
    let bits = value.to_bits();
    f32::from_bits(if value > 0.0 { bits + 1 } else { bits - 1 })
}

fn next_down(value: f32) -> f32 {
    -next_up(-value)
}

/// Gate 4. Red if the control validator and the effect contract disagree on a parameter's domain
/// at its minimum, its maximum, their neighbouring `f32` values, either zero or NaN.
#[test]
fn check_accepts_exactly_what_the_contract_accepts() {
    let model = effects_fixture();
    let all = rows(&model);
    for effect in LAUNCH_EFFECTS {
        for parameter in descriptor_of(effect).parameters {
            let (low, high) = match (parameter.minimum, parameter.maximum) {
                (Some(low), Some(high)) => (low, high),
                _ if parameter.enum_choices.is_empty() => (0.0, 1.0),
                _ => (
                    parameter.enum_choices.first().unwrap().value,
                    parameter.enum_choices.last().unwrap().value,
                ),
            };
            let probes = [
                low,
                high,
                next_up(low),
                next_down(low),
                next_up(high),
                next_down(high),
                0.0,
                -0.0,
                1.0,
                f32::NAN,
                f32::INFINITY,
            ];
            let row = all
                .iter()
                .find(|row| {
                    row.address.section == Section::Inserts(insert_id(effect))
                        && row.address.control == parameter.id.0
                })
                .unwrap();
            for probe in probes {
                assert_eq!(
                    check(&row.descriptor, row.address.lane, probe, None).is_ok(),
                    parameter_value_valid(parameter, probe),
                    "{effect} parameter {} value {probe}",
                    parameter.id.0
                );
            }
        }
    }
}

/// Gate 5 (the table check, extended). Red if a row of any owner reaches a frontend with no name,
/// or with no unit text where its unit is a physical one.
#[test]
fn every_row_has_a_display_name_and_unit_text() {
    for row in rows(&effects_fixture()) {
        let d = &row.descriptor;
        let at = format!("{:?}", row.address);
        assert!(!d.name.is_empty() && !d.display_name.is_empty(), "{at}");
        if d.unit != ParameterUnit::Linear && d.unit != ParameterUnit::Ratio {
            assert!(!d.display_unit.is_empty(), "{at}");
        }
    }
}

/// The compressor's first per-lane parameter and its `Both` start value, for the lane tests.
fn per_lane_parameter() -> (u32, f32, f32, f32) {
    let parameter = descriptor_of("miso.compressor")
        .parameters
        .iter()
        .find(|parameter| {
            parameter.channel_policy == ParameterChannelPolicy::PerLane
                && parameter.automation_rate != AutomationRate::None
                && parameter.enum_choices.is_empty()
                && parameter.minimum.is_some()
        })
        .expect("a continuous per-lane parameter");
    let (low, high) = (parameter.minimum.unwrap(), parameter.maximum.unwrap());
    // Three distinct in-domain values.
    let span = high - low;
    (
        parameter.id.0,
        low + span * 0.25,
        low + span * 0.5,
        low + span * 0.75,
    )
}

fn params_of(model: &SessionModel) -> Vec<(u32, ParameterChannel, f32)> {
    model.tracks[0].inserts.effects[1]
        .params
        .iter()
        .map(|item| (item.parameter_id, item.channel, item.value))
        .collect()
}

const COMPRESSOR: &str = "miso.compressor";

/// Gate 3. Red if a one-lane write changes the other lane, loses the old value, merges entries
/// back, writes when nothing changed, or leaves a model that does not round-trip.
#[test]
fn a_one_lane_write_splits_the_both_entry_and_the_model_round_trips() {
    let (parameter, start, left, both) = per_lane_parameter();
    let mut model = effects_fixture();
    model.tracks[0].inserts.effects[1].params = vec![EffectParam {
        parameter_id: parameter,
        channel: ParameterChannel::Both,
        unit: SessionUnit::Db,
        value: start,
    }];
    let at = |lane| insert_row("vocal", COMPRESSOR, parameter, lane);
    let committed = |model: &SessionModel, lane| {
        get(model, &MonitorOverlay::default(), &at(lane), RATE).unwrap()
    };

    // A write equal to the committed value changes nothing: no split.
    let before = model.clone();
    set(&mut model, &at(Lane::Left), start, None, RATE).unwrap();
    set(&mut model, &at(Lane::Both), start, None, RATE).unwrap();
    assert_eq!(model, before);

    // A `left` write: explicit left (new) and right (old), and no `both`.
    set(&mut model, &at(Lane::Left), left, None, RATE).unwrap();
    assert_eq!(
        params_of(&model),
        [
            (parameter, ParameterChannel::Left, left),
            (parameter, ParameterChannel::Right, start)
        ]
    );
    assert_eq!(committed(&model, Lane::Left), left);
    assert_eq!(committed(&model, Lane::Right), start);

    // A `both` write of equal values does not merge the entries back.
    set(&mut model, &at(Lane::Both), both, None, RATE).unwrap();
    assert_eq!(
        params_of(&model),
        [
            (parameter, ParameterChannel::Left, both),
            (parameter, ParameterChannel::Right, both)
        ]
    );

    // The entries carry the unit session validation requires, and the model round-trips.
    let json = canonical_session_json(&model).expect("the written model is valid");
    let parsed = parse_session_json(&json).expect("the canonical JSON parses");
    assert_eq!(canonical_session_json(&parsed).unwrap(), json);
    let vocal = |model: &SessionModel| {
        model
            .tracks
            .iter()
            .find(|track| track.id == id("vocal"))
            .unwrap()
            .inserts
            .effects[1]
            .params
            .clone()
    };
    assert_eq!(vocal(&parsed), vocal(&model));

    // A right-lane write on a fresh `both` entry splits the other way.
    let mut model = before;
    set(&mut model, &at(Lane::Right), left, None, RATE).unwrap();
    assert_eq!(
        params_of(&model),
        [
            (parameter, ParameterChannel::Left, start),
            (parameter, ParameterChannel::Right, left)
        ]
    );
}

/// A `both` write adds a `both` entry when the parameter has none, updates a `both` entry in
/// place, and keeps the parameters in ID-then-channel order. Red if a `both` write on an absent
/// parameter splits, or the entries go out of canonical order.
#[test]
fn a_both_write_adds_or_updates_and_keeps_canonical_order() {
    let (parameter, start, left, both) = per_lane_parameter();
    let mut model = effects_fixture();
    let at = |lane| insert_row("vocal", COMPRESSOR, parameter, lane);

    // An absent parameter, then an existing `both` entry.
    set(&mut model, &at(Lane::Both), start, None, RATE).unwrap();
    assert_eq!(
        params_of(&model),
        [(parameter, ParameterChannel::Both, start)]
    );
    set(&mut model, &at(Lane::Both), both, None, RATE).unwrap();
    assert_eq!(
        params_of(&model),
        [(parameter, ParameterChannel::Both, both)]
    );

    // A write to a higher parameter ID sorts after it; a later write to a lower one sorts before.
    let high = descriptor_of(COMPRESSOR)
        .parameters
        .iter()
        .rev()
        .find(|p| {
            p.channel_policy == ParameterChannelPolicy::PerLane
                && p.enum_choices.is_empty()
                && p.minimum.is_some()
                && p.id.0 > parameter
        })
        .expect("a later continuous per-lane parameter");
    let mut model = effects_fixture();
    // A value other than the default, because a write equal to the committed value adds nothing.
    let away = if high.maximum.unwrap() != high.default_value {
        high.maximum.unwrap()
    } else {
        high.minimum.unwrap()
    };
    set(
        &mut model,
        &insert_row("vocal", COMPRESSOR, high.id.0, Lane::Both),
        away,
        None,
        RATE,
    )
    .unwrap();
    set(&mut model, &at(Lane::Both), start, None, RATE).unwrap();
    let ids: Vec<u32> = params_of(&model).iter().map(|entry| entry.0).collect();
    assert_eq!(ids, [parameter, high.id.0]);

    // Only a `left` entry exists (the right lane still resolves to the default): a `both` write
    // sets both lanes through explicit entries.
    let mut model = effects_fixture();
    set(&mut model, &at(Lane::Left), left, None, RATE).unwrap();
    assert_eq!(
        params_of(&model),
        [(parameter, ParameterChannel::Left, left)]
    );
    assert_eq!(
        get(&model, &MonitorOverlay::default(), &at(Lane::Right), RATE),
        Ok(descriptor_default(parameter))
    );
    set(&mut model, &at(Lane::Both), both, None, RATE).unwrap();
    assert_eq!(
        params_of(&model),
        [
            (parameter, ParameterChannel::Left, both),
            (parameter, ParameterChannel::Right, both)
        ]
    );
}

fn descriptor_default(parameter: u32) -> f32 {
    descriptor_of(COMPRESSOR)
        .parameters
        .iter()
        .find(|p| p.id.0 == parameter)
        .unwrap()
        .default_value
}

/// Bypass, link mode and a rebuild-only parameter are written through the same `set`. Red if a
/// bypass write does not reach the model, a link mode outside the supported set is accepted, a
/// console slot's link mode is addressable, or a prepared parameter refuses a write.
#[test]
fn bypass_link_mode_and_prepared_rows_are_written_by_set() {
    let mut model = effects_fixture();
    let bypass = insert_row("vocal", COMPRESSOR, BYPASS_CONTROL, Lane::Both);
    set(&mut model, &bypass, 1.0, None, RATE).unwrap();
    assert!(model.tracks[0].inserts.effects[1].bypass);
    assert_eq!(
        get(&model, &MonitorOverlay::default(), &bypass, RATE),
        Ok(1.0)
    );
    // The bypass is shared: a one-lane write is refused.
    let left_bypass = insert_row("vocal", COMPRESSOR, BYPASS_CONTROL, Lane::Left);
    assert_eq!(
        set(&mut model, &left_bypass, 0.0, None, RATE),
        Err(ControlError::Lane)
    );
    // An explicit ramp on a fixed-ramp row is refused, as for any row.
    assert_eq!(
        set(&mut model, &bypass, 0.0, Some(64), RATE),
        Err(ControlError::RampFixed)
    );

    let link = insert_row("vocal", COMPRESSOR, LINK_MODE_CONTROL, Lane::Both);
    set(&mut model, &link, 2.0, None, RATE).unwrap();
    assert_eq!(
        model.tracks[0].inserts.effects[1].link_mode,
        session::LinkMode::Maximum
    );
    assert_eq!(
        set(&mut model, &link, 4.0, None, RATE),
        Err(ControlError::Domain)
    );
    // The EQ supports only dual mono: it has no link-mode row.
    let eq_link = insert_row("vocal", "miso.parametric-eq", LINK_MODE_CONTROL, Lane::Both);
    assert_eq!(
        set(&mut model, &eq_link, 1.0, None, RATE),
        Err(ControlError::AddressUnknown)
    );
    // A console slot's link mode is a slot declaration, not a row.
    let console_link = address(
        Owner::Strip(id("vocal")),
        Section::Console(id("glue")),
        LINK_MODE_CONTROL,
        Lane::Both,
    );
    assert_eq!(
        set(&mut model, &console_link, 1.0, None, RATE),
        Err(ControlError::AddressUnknown)
    );
    let console_bypass = address(
        Owner::Strip(id("vocal")),
        Section::Console(id("glue")),
        BYPASS_CONTROL,
        Lane::Both,
    );
    set(&mut model, &console_bypass, 1.0, None, RATE).unwrap();
    assert!(model.tracks[0].console[0].bypass);

    // A prepared parameter is written like any other; its class says only how it applies.
    let lookahead = |lane| {
        address(
            Owner::Strip(id("vocal")),
            Section::Console(id("ceiling")),
            3,
            lane,
        )
    };
    let left = lookahead(Lane::Left);
    let row = controls::list(&model, &MonitorOverlay::default(), RATE)
        .find(|row| row.address == left)
        .unwrap();
    assert!(rebuild_reason(row.descriptor.class).is_some());
    set(&mut model, &left, 2.0, None, RATE).unwrap();
    assert_eq!(
        get(&model, &MonitorOverlay::default(), &left, RATE),
        Ok(2.0)
    );
    let right = lookahead(Lane::Right);
    assert_ne!(
        get(&model, &MonitorOverlay::default(), &right, RATE),
        Ok(2.0)
    );
}

/// An effect row's machine `name` is the contract's declared `name` (validated `[a-z][a-z0-9_]*`
/// and unique per effect by `validate_descriptor`), never its display text. Red if a row's `name`
/// is derived from, or equal to, the display label.
#[test]
fn effect_row_names_are_the_contracts_declared_names() {
    let all = rows(&effects_fixture());
    for effect in LAUNCH_EFFECTS {
        for parameter in descriptor_of(effect).parameters {
            let row = all
                .iter()
                .find(|row| {
                    row.address.section == Section::Inserts(insert_id(effect))
                        && row.address.control == parameter.id.0
                })
                .unwrap();
            assert_eq!(row.descriptor.name, parameter.name, "{effect}");
            assert_eq!(
                row.descriptor.display_name, parameter.display_name,
                "{effect}"
            );
        }
    }
    let name_of = |effect: &str, control: u32| {
        all.iter()
            .find(|row| {
                row.address.section == Section::Inserts(insert_id(effect))
                    && row.address.control == control
            })
            .unwrap()
            .descriptor
            .name
    };
    assert_eq!(name_of("miso.delay", 1), "delay_time_ms");
    assert_eq!(name_of("miso.parametric-eq", 1), "band_1_enabled");
    assert_eq!(name_of("miso.compressor", 1), "threshold_db");
    assert_eq!(
        all.iter()
            .find(|row| row.address.control == BYPASS_CONTROL)
            .unwrap()
            .descriptor
            .name,
        "bypass"
    );
}
