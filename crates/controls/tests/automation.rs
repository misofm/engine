//! The row rule of stored automation (#1566 D6): `automation_diagnostics` holds each stored
//! automation to the control row its target names, with the control validator's own codes.

mod common;

use common::{RATE, address, effects_fixture, id, insert_id, route_own, strip_builtin, strip_own};
use controls::{ControlAddress, ControlDiagnostic, Lane, Owner, Section, automation_diagnostics};
use session::{Automation, AutomationSegment, AutomationShape, ParameterUnit, SessionModel};

const FADER_DB: u32 = 5;
const DELAY_SAMPLES: u32 = 11;
const MATRIX_LL: u32 = 7;
const SOLO: u32 = 2;
const MATRIX_MODE: u32 = 1;
const FOLLOWS_MUTE: u32 = 7;
const PAN_WINDOW_SAMPLES: u32 = 3;
const EFFECTIVE_FADER_DB: u32 = 64;
const GAIN_DB: u32 = 1;
const OFFSET_DB: u32 = 1;
/// The EQ's `band-1-enabled` (prepared, not live) and `band-1-gain` (block rate).
const BAND_ENABLED: u32 = 1;
const BAND_GAIN: u32 = 4;

fn segment(unit: ParameterUnit, value: f32) -> AutomationSegment {
    AutomationSegment {
        shape: AutomationShape::Step,
        start_sample: 0,
        end_sample: 480,
        start_value: value,
        end_value: value,
        unit,
    }
}

fn automation(name: &str, target: ControlAddress, segments: Vec<AutomationSegment>) -> Automation {
    Automation {
        id: id(name),
        target,
        segments,
    }
}

/// The diagnostics of `model` with exactly `automations` stored.
fn judged(automations: Vec<Automation>) -> Vec<(&'static str, String)> {
    let mut model: SessionModel = effects_fixture();
    model.automation = automations;
    automation_diagnostics(&model, RATE)
        .into_iter()
        .map(|ControlDiagnostic { code, path }| (code, path))
        .collect()
}

fn one(name: &str, target: ControlAddress, unit: ParameterUnit, value: f32) -> Automation {
    automation(name, target, vec![segment(unit, value)])
}

fn eq_param(strip: &str, control: u32, lane: Lane) -> ControlAddress {
    address(
        Owner::Strip(id(strip)),
        Section::Inserts(insert_id("miso.parametric-eq")),
        control,
        lane,
    )
}

/// Gate 1: the row decides. A rebuild-only row, a derived row, the monitor row and a setting row
/// are refused at their target with `control.automation.target`, and so are a send's
/// `follows_mute` and a strip's `matrix_mode` (live, but ruled out of automation, #1632); a send's gain, a VCA's offset, a
/// builtin fader and a block-rate effect parameter are accepted.
///
/// Red if automation can target a value that is not live or a ruled exclusion, or cannot target a
/// live send or VCA value.
#[test]
fn the_row_decides_what_can_be_automated() {
    let refused = [
        (
            "delay",
            strip_builtin("vocal", DELAY_SAMPLES, Lane::Left),
            ParameterUnit::Samples,
        ),
        (
            "derived",
            strip_own("vocal", EFFECTIVE_FADER_DB, Lane::Left),
            ParameterUnit::Db,
        ),
        (
            "solo",
            strip_own("vocal", SOLO, Lane::Both),
            ParameterUnit::Linear,
        ),
        (
            "window",
            strip_own("vocal", PAN_WINDOW_SAMPLES, Lane::Both),
            ParameterUnit::Samples,
        ),
        (
            "prepared",
            eq_param("vocal", BAND_ENABLED, Lane::Left),
            ParameterUnit::Linear,
        ),
        // Live rows that a ruling keeps out of automation (#1632): a routing setting and a mode.
        (
            "follows-mute",
            route_own("send", FOLLOWS_MUTE),
            ParameterUnit::Linear,
        ),
        (
            "matrix-mode",
            strip_own("vocal", MATRIX_MODE, Lane::Both),
            ParameterUnit::Linear,
        ),
    ];
    for (name, target, unit) in refused {
        assert_eq!(
            judged(vec![one(name, target, unit, 0.0)]),
            [(
                "control.automation.target",
                format!("$.automation[id={name}].target")
            )],
            "{name}"
        );
    }
    let accepted = [
        ("send", route_own("send", GAIN_DB), ParameterUnit::Db, -6.0),
        (
            "riders",
            address(
                Owner::Vca(id("riders")),
                Section::Own,
                OFFSET_DB,
                Lane::Both,
            ),
            ParameterUnit::Db,
            -3.0,
        ),
        (
            "fader",
            strip_builtin("vocal", FADER_DB, Lane::Left),
            ParameterUnit::Db,
            -3.0,
        ),
        (
            "band",
            eq_param("vocal", BAND_GAIN, Lane::Left),
            ParameterUnit::Db,
            3.0,
        ),
        (
            "bus-band",
            eq_param("bus", BAND_GAIN, Lane::Right),
            ParameterUnit::Db,
            3.0,
        ),
    ];
    for (name, target, unit, value) in accepted {
        assert_eq!(
            judged(vec![one(name, target, unit, value)]),
            Vec::new(),
            "{name}"
        );
    }
}

/// Gate 2: a target that breaks a lane or inactive rule is reported at the target, once; a segment
/// whose unit or value the row refuses is `control.domain` at that segment; an address with no row
/// is `control.address.unknown`.
///
/// Red if a segment can carry a lane, unit or value its row refuses, or if stored automation is
/// refused with a code that `SET_CONTROL` does not use for the same fault.
#[test]
fn lane_unit_domain_and_unknown_rows_are_refused_with_the_validators_codes() {
    let at = |name: &str, suffix: &str| format!("$.automation[id={name}]{suffix}");
    // `left` on a shared row.
    assert_eq!(
        judged(vec![one(
            "lane",
            address(Owner::Route(id("send")), Section::Own, GAIN_DB, Lane::Left),
            ParameterUnit::Db,
            0.0,
        )]),
        [("control.lane", at("lane", ".target"))]
    );
    // A matrix row on a pan strip: automatable, and the strip's shape does not use it.
    assert_eq!(
        judged(vec![one(
            "inactive",
            strip_builtin("vocal", MATRIX_LL, Lane::Both),
            ParameterUnit::Linear,
            0.5,
        )]),
        [("control.inactive", at("inactive", ".target"))]
    );
    // The same row on the matrix-mode strip is live.
    assert_eq!(
        judged(vec![one(
            "active",
            strip_builtin("guitar", MATRIX_LL, Lane::Both),
            ParameterUnit::Linear,
            0.5,
        )]),
        Vec::new()
    );
    // A segment in another unit, and a value outside the fader's range, each at its own index.
    let fader = || strip_builtin("vocal", FADER_DB, Lane::Left);
    assert_eq!(
        judged(vec![automation(
            "unit",
            fader(),
            vec![
                segment(ParameterUnit::Db, 0.0),
                segment(ParameterUnit::Hz, 0.0)
            ],
        )]),
        [("control.domain", at("unit", ".segments[1]"))]
    );
    assert_eq!(
        judged(vec![automation(
            "value",
            fader(),
            vec![
                segment(ParameterUnit::Db, 30.0),
                segment(ParameterUnit::Db, -3.0)
            ],
        )]),
        [("control.domain", at("value", ".segments[0]"))]
    );
    // A ramp from an acceptable value to a refused one is refused once, at its segment.
    let mut ramp = segment(ParameterUnit::Db, 0.0);
    ramp.end_value = f32::NAN;
    assert_eq!(
        judged(vec![automation("nan", fader(), vec![ramp])]),
        [("control.domain", at("nan", ".segments[0]"))]
    );
    // No row: an unknown builtin, an unknown effect parameter, an unknown own row of a route.
    for (name, target) in [
        ("builtin", strip_builtin("vocal", 99, Lane::Both)),
        ("param", eq_param("vocal", 9_999, Lane::Left)),
        (
            "own",
            address(Owner::Route(id("send")), Section::Own, 99, Lane::Both),
        ),
    ] {
        assert_eq!(
            judged(vec![one(name, target, ParameterUnit::Db, 0.0)]),
            [("control.address.unknown", at(name, ".target"))],
            "{name}"
        );
    }
}

/// Every automation of the session is judged, in the session's order, and a refused target hides
/// its segments' faults (one fault, one report).
#[test]
fn every_automation_is_judged_and_a_refused_target_reports_once() {
    let diagnostics = judged(vec![
        one("a-send", route_own("send", GAIN_DB), ParameterUnit::Db, 0.0),
        // A refused target with a refused segment: the target is the report.
        one(
            "b-solo",
            strip_own("vocal", SOLO, Lane::Both),
            ParameterUnit::Hz,
            99.0,
        ),
        one(
            "c-send-hz",
            route_own("send", GAIN_DB),
            ParameterUnit::Hz,
            0.0,
        ),
    ]);
    assert_eq!(
        diagnostics,
        [
            (
                "control.automation.target",
                "$.automation[id=b-solo].target".to_owned()
            ),
            (
                "control.domain",
                "$.automation[id=c-send-hz].segments[0]".to_owned()
            ),
        ]
    );
}

/// A target in an effect that has no rows is the effect's fault, which effect preparation reports
/// with its own code: a `cid` effect and a native effect the launch registry lacks add nothing
/// here.
///
/// Red if one unavailable effect is reported twice, once as the effect and once as its automation.
#[test]
fn a_target_in_an_unavailable_effect_is_reported_by_the_effect_alone() {
    let mut model = effects_fixture();
    let target = eq_param("vocal", BAND_GAIN, Lane::Left);
    model.automation = vec![one("ride", target, ParameterUnit::Db, 0.0)];
    assert_eq!(automation_diagnostics(&model, RATE), Vec::new());

    let identity = session::EffectIdentity::ThirdPartyCid {
        cid: "bafyopaque".to_owned(),
    };
    let insert = id("ins-parametric-eq");
    let effect = |model: &mut SessionModel| {
        model.tracks[0]
            .inserts
            .effects
            .iter_mut()
            .find(|effect| effect.id == insert)
            .expect("the fixture's EQ insert")
            .identity = identity.clone();
    };
    effect(&mut model);
    assert_eq!(automation_diagnostics(&model, RATE), Vec::new());

    let mut model = effects_fixture();
    model.tracks[0].inserts.effects[0].identity = session::EffectIdentity::Native {
        effect_id: id("miso.no-such-effect"),
    };
    let target = address(
        Owner::Strip(id("vocal")),
        Section::Inserts(model.tracks[0].inserts.effects[0].id.clone()),
        BAND_GAIN,
        Lane::Left,
    );
    model.automation = vec![one("ride", target, ParameterUnit::Db, 0.0)];
    assert_eq!(automation_diagnostics(&model, RATE), Vec::new());
}

// #1645: the shape rule and the input filter order rule, on the checked-in builtin automation.

const POLARITY_INVERT: u32 = 1;
const HPF_HZ: u32 = 3;
const LPF_HZ: u32 = 4;
const MUTE: u32 = 6;

/// `fixtures/session/v1/builtins-automation.json` (48 kHz; `vocal` has HPF 20 Hz and LPF 20 kHz on
/// both lanes; `bus` is a matrix-mode submix).
fn builtins_fixture() -> SessionModel {
    session::parse_session_json(include_str!(
        "../../../fixtures/session/v1/builtins-automation.json"
    ))
    .expect("fixture parses")
}

fn ramp(
    shape: AutomationShape,
    unit: ParameterUnit,
    (start_sample, end_sample): (u64, u64),
    (start_value, end_value): (f32, f32),
) -> AutomationSegment {
    AutomationSegment {
        shape,
        start_sample,
        end_sample,
        start_value,
        end_value,
        unit,
    }
}

/// The diagnostics of `model` with `added` stored after its own automation.
fn judged_with(mut model: SessionModel, added: Vec<Automation>) -> Vec<(&'static str, String)> {
    model.automation.extend(added);
    let rate = model.sample_rate_hz;
    automation_diagnostics(&model, rate)
        .into_iter()
        .map(|ControlDiagnostic { code, path }| (code, path))
        .collect()
}

fn first_segment(name: &str) -> String {
    format!("$.automation[id={name}].segments[0]")
}

/// Gate 1: each row refuses a segment its unit, domain or shape rule refuses, once, at the
/// segment; the fixture, a filter's legal `exponential` and "off" by `step` are accepted.
///
/// Red for each row rule left out, for a domain check that ignores the session rate (the 44.1 kHz
/// case), for a shape rule that refuses the filters' `exponential`, or for a filter rule that
/// refuses "off" by `step`.
#[test]
fn builtin_rows_refuse_a_segment_by_unit_domain_and_shape() {
    use AutomationShape::{Exponential, Linear, Step};
    use ParameterUnit::{Db, Hz};
    let lin = ParameterUnit::Linear;
    assert_eq!(judged_with(builtins_fixture(), Vec::new()), Vec::new());
    let vocal = |control| strip_builtin("vocal", control, Lane::Left);
    let span = (0, 960);
    let refused = [
        (
            "fader-unit",
            vocal(FADER_DB),
            ramp(Linear, lin, span, (0.0, 0.5)),
            "control.domain",
        ),
        (
            "fader-high",
            vocal(FADER_DB),
            ramp(Linear, Db, span, (0.0, 24.001)),
            "control.domain",
        ),
        (
            "matrix-high",
            strip_builtin("bus", MATRIX_LL, Lane::Both),
            ramp(Linear, lin, span, (1.0, 1.5)),
            "control.domain",
        ),
        (
            "polarity-half",
            vocal(POLARITY_INVERT),
            ramp(Step, lin, span, (0.0, 0.5)),
            "control.domain",
        ),
        (
            "mute-linear",
            vocal(MUTE),
            ramp(Linear, lin, span, (0.0, 1.0)),
            "control.automation.shape",
        ),
        (
            "fader-exp",
            vocal(FADER_DB),
            ramp(Exponential, Db, span, (-6.0, -3.0)),
            "control.automation.shape",
        ),
        (
            "hpf-from-off",
            vocal(HPF_HZ),
            ramp(Linear, Hz, span, (0.0, 100.0)),
            "control.automation.shape",
        ),
        (
            "hpf-exp-to-off",
            vocal(HPF_HZ),
            ramp(Exponential, Hz, span, (100.0, 0.0)),
            "control.automation.shape",
        ),
    ];
    for (name, target, segment, code) in refused {
        assert_eq!(
            judged_with(
                builtins_fixture(),
                vec![automation(name, target, vec![segment])]
            ),
            [(code, first_segment(name))],
            "{name}"
        );
    }
    // 23 kHz is in domain at 48 kHz and above the 44.1 kHz bound (about 22,049 Hz).
    let high = || {
        automation(
            "hpf-high",
            vocal(HPF_HZ),
            vec![ramp(Step, Hz, span, (23_000.0, 23_000.0))],
        )
    };
    let mut at_44k1 = builtins_fixture();
    at_44k1.sample_rate_hz = 44_100;
    let builtins = &mut at_44k1.tracks[0].builtins;
    builtins.left.lpf_hz = 0.0;
    builtins.right.lpf_hz = 0.0;
    assert_eq!(
        judged_with(at_44k1, vec![high()]),
        [("control.domain", first_segment("hpf-high"))]
    );
    let mut at_48k = builtins_fixture();
    let builtins = &mut at_48k.tracks[0].builtins;
    builtins.left.lpf_hz = 0.0;
    builtins.right.lpf_hz = 0.0;
    assert_eq!(judged_with(at_48k, vec![high()]), Vec::new());
    let accepted = [
        ("hpf-exp", ramp(Exponential, Hz, span, (20.0, 200.0))),
        ("hpf-on-by-step", ramp(Step, Hz, span, (0.0, 80.0))),
    ];
    for (name, segment) in accepted {
        assert_eq!(
            judged_with(
                builtins_fixture(),
                vec![automation(name, vocal(HPF_HZ), vec![segment])]
            ),
            Vec::new(),
            "{name}"
        );
    }
}

/// The fixture with a static LPF of `lpf_hz` on both of `vocal`'s lanes.
fn with_static_lpf(lpf_hz: f32) -> SessionModel {
    let mut model = builtins_fixture();
    let builtins = &mut model.tracks[0].builtins;
    builtins.left.lpf_hz = lpf_hz;
    builtins.right.lpf_hz = lpf_hz;
    model
}

fn hpf(name: &str, segments: Vec<AutomationSegment>) -> Automation {
    automation(name, strip_builtin("vocal", HPF_HZ, Lane::Both), segments)
}

fn lpf(name: &str, segments: Vec<AutomationSegment>) -> Automation {
    automation(name, strip_builtin("vocal", LPF_HZ, Lane::Both), segments)
}

/// Gate 2: the HPF stays below an enabled LPF on every lane at all times: static LPF 1,000 Hz
/// unless stated.
///
/// Red if the check samples only one curve's boundaries (the hold after a step), uses the end-point
/// rule on mixed pieces (the `exponential` against `linear` case), checks a lane whose HPF is off,
/// or reads the static value of an automated side; the last case is red if the rule reads anything
/// but the model it is given.
#[test]
fn input_filter_curves_keep_the_hpf_below_the_lpf() {
    use AutomationShape::{Exponential, Linear, Step};
    use ParameterUnit::Hz;
    let span = (0, 960);
    let order = |name: &str, k: usize| {
        vec![(
            "control.automation.order",
            format!("$.automation[id={name}].segments[{k}]"),
        )]
    };
    // Refused: a ride that reaches the LPF at its end.
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![hpf("h", vec![ramp(Linear, Hz, span, (200.0, 1_000.0))])]
        ),
        order("h", 0)
    );
    // Refused on the hold after a step that is ordered while it lasts.
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![hpf("h", vec![ramp(Step, Hz, (0, 480), (100.0, 1_200.0))])]
        ),
        order("h", 0)
    );
    // Refused, by design: mixed pieces, ordered at both ends, whose extremes overlap. Reported at
    // the LPF entry.
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![
                hpf("h", vec![ramp(Exponential, Hz, span, (100.0, 900.0))]),
                lpf("l", vec![ramp(Linear, Hz, span, (500.0, 2_000.0))]),
            ]
        ),
        order("l", 0)
    );
    // Accepted: a ride under the static LPF, and two ordered linear rides over the same samples.
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![hpf("h", vec![ramp(Linear, Hz, span, (100.0, 900.0))])]
        ),
        Vec::new()
    );
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![
                hpf("h", vec![ramp(Linear, Hz, span, (100.0, 900.0))]),
                lpf("l", vec![ramp(Linear, Hz, span, (500.0, 2_000.0))]),
            ]
        ),
        Vec::new()
    );
    // Accepted: the HPF steps to "off" before an LPF ride passes below its former value.
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![
                hpf("h", vec![ramp(Step, Hz, (0, 480), (800.0, 0.0))]),
                lpf("l", vec![ramp(Linear, Hz, (480, 960), (2_000.0, 300.0))]),
            ]
        ),
        Vec::new()
    );
    // The same LPF ride under an HPF that stays on is refused in the gap's hold: the LPF entry's
    // segment 0 defines its value there.
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![
                hpf("h", vec![ramp(Step, Hz, (0, 480), (800.0, 800.0))]),
                lpf("l", vec![ramp(Linear, Hz, (480, 960), (2_000.0, 300.0))]),
            ]
        ),
        order("l", 0)
    );
    // The static LPF moves under an unchanged HPF ride to 900 Hz.
    let ride = || vec![hpf("h", vec![ramp(Linear, Hz, span, (100.0, 900.0))])];
    assert_eq!(judged_with(with_static_lpf(1_000.0), ride()), Vec::new());
    assert_eq!(judged_with(with_static_lpf(500.0), ride()), order("h", 0));
}
