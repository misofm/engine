//! The row rule of stored automation (#1566 D6): `automation_diagnostics` holds each stored
//! automation to the control row its target names, with the control validator's own codes.

mod common;

use common::{RATE, address, effects_fixture, id, insert_id, route_own, strip_builtin, strip_own};
use controls::{ControlAddress, ControlDiagnostic, Lane, Owner, Section, automation_diagnostics};
use session::{
    Automation, AutomationOffset, AutomationSegment, AutomationShape, ParameterUnit, SessionModel,
};

const TRIM_DB: u32 = 2;
const FADER_DB: u32 = 5;
const PAN: u32 = 12;
const DELAY_SAMPLES: u32 = 11;
const MATRIX_LL: u32 = 7;
const SOLO: u32 = 2;
const MATRIX_MODE: u32 = 1;
const FOLLOWS_MUTE: u32 = 7;
const PAN_WINDOW_SAMPLES: u32 = 3;
const EFFECTIVE_FADER_DB: u32 = 64;
const GAIN_DB: u32 = 1;
const OFFSET_DB: u32 = 1;
/// The EQ's `band-1-gain` (block rate).
const BAND_GAIN: u32 = 4;
/// The true-peak limiter's `lookahead` (prepared, not live).
const LOOKAHEAD: u32 = 3;

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
        offset: AutomationOffset::default(),
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
            address(
                Owner::Strip(id("vocal")),
                Section::Inserts(insert_id("miso.true-peak-limiter")),
                LOOKAHEAD,
                Lane::Left,
            ),
            ParameterUnit::Milliseconds,
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
    // The same LPF ride under an HPF that stays on is refused on the LPF ride's sub-interval
    // [480, 960): the LPF entry's segment 0 defines its value there.
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

    // Two constants that tie are refused: the static rule is strict. Red if `<` becomes `<=`.
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![hpf("h", vec![ramp(Step, Hz, span, (1_000.0, 1_000.0))])]
        ),
        order("h", 0)
    );
    // A ride that ends one `f32` step under a static LPF is ordered in `f32` and refused within
    // the margin, which render's rounding could close. Red if the margin is removed.
    let under = f32::from_bits(1_000.0_f32.to_bits() - 1);
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![hpf("h", vec![ramp(Linear, Hz, span, (100.0, under))])]
        ),
        order("h", 0)
    );
    // Two parallel `exponential` rides, the LPF an octave above, are accepted, as two `linear`
    // ones are. Red if an exponential pair falls through to the mixed rule (its extremes overlap).
    assert_eq!(
        judged_with(
            with_static_lpf(1_000.0),
            vec![
                hpf("h", vec![ramp(Exponential, Hz, span, (100.0, 800.0))]),
                lpf("l", vec![ramp(Exponential, Hz, span, (200.0, 1_600.0))]),
            ]
        ),
        Vec::new()
    );
}

/// Root's ruling (a) on #1645 (review fold): render can lead a stored filter value by 64 samples
/// (#1637's grid lookahead) and lag it by 64 (the filter's jump ramp), so a rendered pair can read
/// stored values up to 128 samples apart, and the rule holds the HPF at each sample below the LPF
/// at every sample within 128 samples of it. An HPF that holds 900 Hz on `[0, 1000)` and then
/// steps to 100 Hz, under an LPF that falls linearly from 1,000 Hz to 300 Hz on `[1000, 1010)`, is
/// ordered at every single sample but crosses in render: the LPF reaches 300 Hz before the HPF's
/// jump ramp has left 900 Hz.
///
/// Red if the rule compares the two curves at the same sample only (the counterexample is then
/// accepted), if the reach is 64 rather than 128 (the same fall a hundred samples later, still
/// within one lead plus one lag of the HPF's 900 Hz, is then accepted), or if the reach is
/// unbounded (the fall two hundred samples later is then refused).
#[test]
fn input_filter_curves_keep_their_order_within_renders_reach() {
    use AutomationShape::{Linear, Step};
    use ParameterUnit::Hz;
    let pair = |fall: u64| {
        vec![
            hpf("h", vec![ramp(Step, Hz, (0, 1_000), (900.0, 100.0))]),
            lpf(
                "l",
                vec![ramp(Linear, Hz, (fall, fall + 10), (1_000.0, 300.0))],
            ),
        ]
    };
    assert_eq!(
        judged_with(with_static_lpf(1_000.0), pair(1_000)),
        [(
            "control.automation.order",
            "$.automation[id=l].segments[0]".to_owned()
        )]
    );
    assert_eq!(
        judged_with(with_static_lpf(1_000.0), pair(1_100)),
        [(
            "control.automation.order",
            "$.automation[id=l].segments[0]".to_owned()
        )]
    );
    assert_eq!(
        judged_with(with_static_lpf(1_000.0), pair(1_200)),
        Vec::new()
    );
}

/// An entry of `automation` with `offset` stored.
fn with_offset(automation: Automation, left_db: f32, right_db: f32) -> Automation {
    Automation {
        offset: AutomationOffset { left_db, right_db },
        ..automation
    }
}

/// #1650a gate 1 (D2, root's ruling R17-6): the hand-move offset belongs to a lane the entry
/// drives on the one row that takes it, `fader_db`, inside that row's `[-144, 24]` dB. A lane the
/// entry does not drive, and every lane of any other row (the input trim and the send gain until
/// #1658 and #1654 add them, a pan), hold exactly `+0.0`, else `control.automation.offset`; an
/// offset outside the domain is `control.domain`. Each is reported at its lane's path, after the
/// entry's segments.
///
/// Red if `takes_offset` admits another row (the stored value would be accepted and never heard),
/// if the lane rule is missing (a `left` entry could carry a right offset that no lane reads), if
/// the domain check is missing, or if `-0.0` passes on a lane that takes no offset (two spellings
/// of "none").
#[test]
fn the_offset_belongs_to_a_driven_fader_lane() {
    let fader = |name: &str, lane: Lane| {
        one(
            name,
            strip_builtin("vocal", FADER_DB, lane),
            ParameterUnit::Db,
            -6.0,
        )
    };
    // Accepted: a both entry with one offset per lane, a left entry with a left offset, the domain
    // ends, and a `-0.0` zero on a driven lane.
    for accepted in [
        with_offset(fader("both", Lane::Both), 3.0, -6.0),
        with_offset(fader("left", Lane::Left), 24.0, 0.0),
        with_offset(fader("right", Lane::Right), 0.0, -144.0),
        with_offset(fader("negative-zero", Lane::Both), -0.0, -0.0),
    ] {
        let name = accepted.id.as_str().to_owned();
        assert_eq!(judged(vec![accepted]), [], "{name}");
    }

    let refused = [
        // The lane rule: a lane the entry does not drive holds `+0.0`, and only `+0.0`.
        (
            with_offset(fader("left-right", Lane::Left), 0.0, 1.0),
            "control.automation.offset",
            ".offset.right_db",
        ),
        (
            with_offset(fader("right-left", Lane::Right), -0.0, 0.0),
            "control.automation.offset",
            ".offset.left_db",
        ),
        // The fader row's domain.
        (
            with_offset(fader("above", Lane::Both), 24.5, 0.0),
            "control.domain",
            ".offset.left_db",
        ),
        (
            with_offset(fader("below", Lane::Left), -144.5, 0.0),
            "control.domain",
            ".offset.left_db",
        ),
        // Rows that take no offset (R17-6): the trim, a send's gain, a pan.
        (
            with_offset(
                one(
                    "trim",
                    strip_builtin("vocal", TRIM_DB, Lane::Left),
                    ParameterUnit::Db,
                    0.0,
                ),
                2.0,
                0.0,
            ),
            "control.automation.offset",
            ".offset.left_db",
        ),
        (
            with_offset(
                one(
                    "send-gain",
                    route_own("send", GAIN_DB),
                    ParameterUnit::Db,
                    0.0,
                ),
                0.0,
                -3.0,
            ),
            "control.automation.offset",
            ".offset.right_db",
        ),
        (
            with_offset(
                one(
                    "pan",
                    strip_builtin("vocal", PAN, Lane::Both),
                    ParameterUnit::Linear,
                    0.0,
                ),
                0.0,
                -0.0,
            ),
            "control.automation.offset",
            ".offset.right_db",
        ),
    ];
    for (entry, code, suffix) in refused {
        let name = entry.id.as_str().to_owned();
        assert_eq!(
            judged(vec![entry]),
            [(code, format!("$.automation[id={name}]{suffix}"))],
            "{name}"
        );
    }

    // Both lanes report, after a segment fault of the same entry.
    let mut both = with_offset(
        one(
            "trim-both",
            strip_builtin("vocal", TRIM_DB, Lane::Both),
            ParameterUnit::Db,
            0.0,
        ),
        1.0,
        1.0,
    );
    both.segments[0].unit = ParameterUnit::Hz;
    assert_eq!(
        judged(vec![both]),
        [
            (
                "control.domain",
                "$.automation[id=trim-both].segments[0]".to_owned()
            ),
            (
                "control.automation.offset",
                "$.automation[id=trim-both].offset.left_db".to_owned()
            ),
            (
                "control.automation.offset",
                "$.automation[id=trim-both].offset.right_db".to_owned()
            ),
        ]
    );
}

/// #1651 T2 (the predicate split): the readers that mean "the fader" test the fader row
/// ([`controls::fader_row`]), not every rendered row ([`controls::automation_row_renders`]). On
/// `vocal`, a `both` mute entry (row 6) listed before a `left` fader entry: the fader cells
/// ([`controls::fader_automation`], [`controls::automated_fader_lanes`]) are the left fader entry
/// alone, the right lane has no hand offset and its `S` is that of a session without the mute
/// entry ([`controls::effective`]), and [`controls::mask_values`] masks the right lane's static
/// fader dB as a control value (a fader edit there stays a `FaderDb` record, not an offsets write).
///
/// Red when the mute row joins [`controls::automation_row_renders`] (#1651 T3a) and any of these
/// readers still tests that list: the mute entry, first in the table, would take both lanes'
/// fader cells, give the right lane a `+0.0` hand offset, and keep the right lane's static dB
/// out of the mask. Before T3a the mute row does not render, so the predicate asserts hold the
/// split itself: the fader row admits no mute target, and every fader target renders.
#[test]
fn a_mute_entry_reaches_no_fader_only_reader() {
    use controls::{
        MonitorOverlay, automated_fader_lanes, automation_row_renders, effective, fader_automation,
        fader_row, mask_values,
    };

    let mute = strip_builtin("vocal", MUTE, Lane::Both);
    let fader = strip_builtin("vocal", FADER_DB, Lane::Left);
    assert!(!fader_row(&mute), "the mute row is not the fader row");
    for lane in [Lane::Left, Lane::Right, Lane::Both] {
        let target = strip_builtin("vocal", FADER_DB, lane);
        assert!(fader_row(&target) && automation_row_renders(&target));
    }
    assert!(!fader_row(&strip_builtin("vocal", TRIM_DB, Lane::Left)));

    let entries = vec![
        one("mute-ride", mute, ParameterUnit::Linear, 1.0),
        with_offset(one("fader-ride", fader, ParameterUnit::Db, -6.0), 2.0, 0.0),
    ];
    let mut model = builtins_fixture();
    model.automation = entries;
    let vocal = id("vocal");

    let lanes = fader_automation(&model, &vocal);
    assert_eq!(lanes[0].map(|entry| entry.id.as_str()), Some("fader-ride"));
    assert!(lanes[1].is_none(), "the mute entry drives no fader cell");
    assert_eq!(automated_fader_lanes(&model, &vocal), [true, false]);

    let overlay = MonitorOverlay::default();
    let strip = effective(&model, &overlay).strips[0];
    assert_eq!(strip.fader_offset, [Some(2.0), None]);
    let mut fader_only = model.clone();
    fader_only.automation.remove(0);
    let reference = effective(&fader_only, &overlay).strips[0];
    assert_eq!(
        strip.offsets_sum.map(f64::to_bits),
        reference.offsets_sum.map(f64::to_bits),
        "the mute entry moves no lane's S"
    );

    // The mask: `from` moves both static fader lanes. The automated left lane keeps `into`'s
    // static dB (its fallback); the right lane, which only the mute entry names, takes `from`'s.
    let mut from = model.clone();
    from.tracks[0].fader.left_db = -9.0;
    from.tracks[0].fader.right_db = -9.0;
    let mut into = model.clone();
    let kept = into.tracks[0].fader.left_db;
    mask_values(&mut into, &from);
    assert_eq!(into.tracks[0].fader.left_db.to_bits(), kept.to_bits());
    assert_eq!(into.tracks[0].fader.right_db.to_bits(), (-9.0f32).to_bits());
}

const VCA_OFFSET_DB: u32 = 1;
const VCA_MUTE: u32 = 2;

fn vca_own(vca: &str, control: u32, lane: Lane) -> ControlAddress {
    address(Owner::Vca(id(vca)), Section::Own, control, lane)
}

fn vca(name: &str, offset: [f32; 2], members: &[&str]) -> session::Vca {
    session::Vca {
        id: id(name),
        fader: session::DualMonoFader {
            left_db: offset[0],
            right_db: offset[1],
            left_mute: false,
            right_mute: false,
        },
        members: members.iter().map(|member| id(member)).collect(),
    }
}

/// The fixture with VCA `riders` (over `vocal`) ridden on its left `offset_db` lane (`ride`, hand
/// offset `0`) and stepped on both `mute` lanes (`steps`).
fn ridden_vca() -> SessionModel {
    let mut model = effects_fixture();
    model.automation = vec![
        one(
            "ride",
            vca_own("riders", VCA_OFFSET_DB, Lane::Left),
            ParameterUnit::Db,
            -6.0,
        ),
        one(
            "steps",
            vca_own("riders", VCA_MUTE, Lane::Both),
            ParameterUnit::Linear,
            1.0,
        ),
    ];
    model
}

/// #1655 D5, D6: a VCA's rows 1 and 2 render ([`controls::automation_row_renders`]), and its
/// `offset_db` takes a per-lane hand-move offset in the fader domain: a `both` entry may hold two
/// different offsets (unlike a send's shared gain), a lane the entry does not drive holds `+0.0`,
/// an offset outside `[-144, 24]` dB is `control.domain`, and the `mute` row takes none.
///
/// Red if the VCA rows stay masked from rendering, if the VCA offset takes no offset (the accepted
/// cases are refused), or if the lane or domain rule is missing on the VCA row.
#[test]
fn vca_rows_render_and_take_a_per_lane_offset() {
    use controls::{automation_row_renders, fader_row, vca_mute_row, vca_offset_row};
    for lane in [Lane::Left, Lane::Right, Lane::Both] {
        let offset = vca_own("riders", VCA_OFFSET_DB, lane);
        let mute = vca_own("riders", VCA_MUTE, lane);
        assert!(vca_offset_row(&offset) && automation_row_renders(&offset));
        assert!(vca_mute_row(&mute) && automation_row_renders(&mute));
        assert!(!vca_offset_row(&mute) && !vca_mute_row(&offset));
        assert!(!fader_row(&offset), "a VCA ride is not a strip's fader");
    }
    let ride = |name: &str, lane: Lane| {
        one(
            name,
            vca_own("riders", VCA_OFFSET_DB, lane),
            ParameterUnit::Db,
            -3.0,
        )
    };
    for accepted in [
        with_offset(ride("both", Lane::Both), 3.0, -6.0),
        with_offset(ride("left", Lane::Left), 24.0, 0.0),
        with_offset(ride("right", Lane::Right), 0.0, -144.0),
    ] {
        let name = accepted.id.as_str().to_owned();
        assert_eq!(judged(vec![accepted]), [], "{name}");
    }
    let refused = [
        (
            with_offset(ride("left-right", Lane::Left), 0.0, 1.0),
            "control.automation.offset",
            ".offset.right_db",
        ),
        (
            with_offset(ride("above", Lane::Both), 0.0, 24.5),
            "control.domain",
            ".offset.right_db",
        ),
        (
            with_offset(
                one(
                    "mute",
                    vca_own("riders", VCA_MUTE, Lane::Left),
                    ParameterUnit::Linear,
                    1.0,
                ),
                1.0,
                0.0,
            ),
            "control.automation.offset",
            ".offset.left_db",
        ),
    ];
    for (entry, code, suffix) in refused {
        let name = entry.id.as_str().to_owned();
        assert_eq!(
            judged(vec![entry]),
            [(code, format!("$.automation[id={name}]{suffix}"))],
            "{name}"
        );
    }
}

/// #1655 D5 and R17-1 at the controls level: a `SET_CONTROL` on an automated VCA `offset_db` lane
/// writes the driving entry's offset of that lane and leaves the static offset (the fallback); a
/// lane no entry drives keeps the static write; `get` and the list report the offset with
/// `automated = true`. A hand VCA mute on an automated mute lane stays the static (hand) mute,
/// listed `automated`. The member's own rows are not marked. The in-place path
/// ([`controls::OwnerEdits`], read through [`controls::ModelView`]) gives the dense model, the
/// dense rows and the dense effective `S`.
///
/// Red if a hand move on an automated VCA offset writes the static offset (acked, never heard), if
/// it writes the lane the entry does not drive, if the list hides the offset or the flag, if a VCA
/// ride marks its member's own fader as automated, or if the in-place path does not clone the
/// entry with the VCA.
#[test]
fn a_hand_move_on_an_automated_vca_offset_writes_its_offset() {
    use controls::{
        LocalEffective, ModelView, MonitorOverlay, OwnerEdits, effective, get, owner_index,
        owner_rows_in, owner_rows_with, set,
    };
    let committed = ridden_vca();
    let left = vca_own("riders", VCA_OFFSET_DB, Lane::Left);
    let right = vca_own("riders", VCA_OFFSET_DB, Lane::Right);
    let mute = vca_own("riders", VCA_MUTE, Lane::Left);
    let riders = |model: &SessionModel| model.vcas[0].clone();
    let static_left = riders(&committed).fader.left_db;

    let mut whole = committed.clone();
    set(&mut whole, &left, 3.0, None, RATE).expect("left");
    set(&mut whole, &right, -1.0, None, RATE).expect("right");
    set(&mut whole, &mute, 0.0, None, RATE).expect("mute");
    assert_eq!(whole.automation[0].offset.left_db, 3.0);
    assert_eq!(
        whole.automation[0].offset.right_db.to_bits(),
        0.0f32.to_bits()
    );
    assert_eq!(
        riders(&whole).fader.left_db.to_bits(),
        static_left.to_bits()
    );
    assert_eq!(riders(&whole).fader.right_db, -1.0);
    assert!(
        !riders(&whole).fader.left_mute,
        "the hand mute is the static mute"
    );
    let overlay = MonitorOverlay::default();
    assert_eq!(get(&whole, &overlay, &left, RATE), Ok(3.0));
    let left_row = common::row_of(&whole, &left);
    assert!(left_row.automated && left_row.value == 3.0);
    let right_row = common::row_of(&whole, &right);
    assert!(!right_row.automated && right_row.value == -1.0);
    let mute_row = common::row_of(&whole, &mute);
    assert!(mute_row.automated && mute_row.value == 0.0);
    let member = common::row_of(&whole, &strip_builtin("vocal", FADER_DB, Lane::Left));
    assert!(!member.automated, "the member's own fader is not automated");

    // The in-place path: the same writes on the owners model.
    let mut edits = OwnerEdits::new(&committed);
    edits.set(&left, 3.0, None, RATE).expect("in-place left");
    edits.set(&right, -1.0, None, RATE).expect("in-place right");
    edits.set(&mute, 0.0, None, RATE).expect("in-place mute");
    let (owners, slots) = edits.finish();
    assert_eq!(owners.automation.len(), 1, "the ride alone is cloned");
    assert_eq!(slots.automation, [(0, 0)]);
    let view = ModelView::edited(&committed, &owners);
    assert_eq!(view.to_model(), whole);
    assert_eq!(
        view.vca_automation(&id("riders"))[0].map(|entry| entry.offset.left_db),
        Some(3.0)
    );
    let owner = owner_index(&committed, &Owner::Vca(id("riders"))).expect("riders");
    let dense = effective(&whole, &overlay);
    let (mut dense_rows, mut view_rows) = (Vec::new(), Vec::new());
    owner_rows_with(&whole, &dense, &overlay, owner, RATE, &mut dense_rows);
    owner_rows_in(view, &dense, &overlay, owner, RATE, &mut view_rows);
    assert_eq!(view_rows, dense_rows);
    // `S` of the member reads the edited offset on the in-place path too.
    let local = LocalEffective::new(view, &overlay);
    let vocal = controls::strip_index(&whole, &id("vocal")).expect("vocal");
    assert_eq!(
        local.strip(vocal).offsets_sum.map(f64::to_bits),
        dense.strips[vocal].offsets_sum.map(f64::to_bits)
    );
    assert_eq!(
        dense.strips[vocal].offsets_sum[0], 3.0,
        "S holds the hand offset"
    );
}

/// #1655 mask treatment: [`controls::mask_values`] copies a VCA-offset entry's offset as a control
/// value and keeps the static offset of an automated VCA lane as structure (its fallback), while
/// the lane no entry drives, the VCA mutes (a hand mute ORs) and the members stay control values.
///
/// Red if a VCA offset change is seen as structure (a live hand move would rebuild), or if a change
/// of an automated VCA lane's static offset is hidden as a value (it would never rebuild).
#[test]
fn the_mask_copies_a_vca_offset_and_keeps_its_fallback() {
    use controls::mask_values;
    let mut from = ridden_vca();
    from.automation[0].offset.left_db = 2.0;
    from.vcas[0].fader.left_db = -9.0;
    from.vcas[0].fader.right_db = -9.0;
    from.vcas[0].fader.left_mute = false;
    from.vcas[0].members.push(id("guitar"));
    let mut into = ridden_vca();
    let kept = into.vcas[0].fader.left_db;
    mask_values(&mut into, &from);
    assert_eq!(into.automation[0].offset.left_db, 2.0);
    assert_eq!(into.vcas[0].fader.left_db.to_bits(), kept.to_bits());
    assert_eq!(into.vcas[0].fader.right_db, -9.0);
    assert!(!into.vcas[0].fader.left_mute);
    assert_eq!(into.vcas[0].members, from.vcas[0].members);
}

/// #1655 batch 19 amendment, event lanes: [`controls::fader_event_lanes`] holds on a lane whose own
/// fader is automated or that an automated VCA `offset_db` lane reaches (directly or through a
/// nested VCA; lane `l` to lane `l`), and [`controls::mute_event_lanes`] by the same rule on the
/// mute rows. A VCA ride reaches no own-entry reader: [`controls::fader_automation`] and the
/// member's `fader_offset` stay empty.
///
/// Red if the predicate misses a VCA ride (the member would render no VCA curve), takes it across
/// lanes or rows (an offset ride making a mute event lane), ignores nesting, or if a VCA ride leaks
/// into a member's own-entry reader.
#[test]
fn event_lanes_take_the_own_entry_and_every_reaching_vca_ride() {
    use controls::{
        MonitorOverlay, effective, fader_automation, fader_event_lanes, mute_event_lanes,
    };
    let mut model = effects_fixture();
    model.vcas = vec![
        vca("outer", [0.0, 0.0], &["riders"]),
        vca("riders", [0.0, 0.0], &["vocal"]),
    ];
    let lanes = |model: &SessionModel, strip: &str| {
        let strip = id(strip);
        let reach = model.vca_reach_of(&strip);
        (
            fader_event_lanes(model, &strip, &reach),
            mute_event_lanes(model, &strip, &reach),
        )
    };
    assert_eq!(lanes(&model, "vocal"), ([false; 2], [false; 2]));

    // A VCA offset ride on the right lane: vocal's right fader lane, nothing else.
    model.automation = vec![one(
        "ride",
        vca_own("riders", VCA_OFFSET_DB, Lane::Right),
        ParameterUnit::Db,
        -6.0,
    )];
    assert_eq!(lanes(&model, "vocal"), ([false, true], [false; 2]));
    assert_eq!(lanes(&model, "guitar"), ([false; 2], [false; 2]));
    assert_eq!(fader_automation(&model, &id("vocal")), [None, None]);
    let strip = effective(&model, &MonitorOverlay::default()).strips[0];
    assert_eq!(strip.fader_offset, [None, None]);

    // A nested VCA's mute steps on the left lane, and the strip's own left fader ride.
    model.automation.push(one(
        "steps",
        vca_own("outer", VCA_MUTE, Lane::Left),
        ParameterUnit::Linear,
        1.0,
    ));
    model.automation.push(one(
        "own",
        strip_builtin("vocal", FADER_DB, Lane::Left),
        ParameterUnit::Db,
        -6.0,
    ));
    assert_eq!(lanes(&model, "vocal"), ([true, true], [true, false]));
    assert_eq!(lanes(&model, "guitar"), ([false; 2], [false; 2]));

    // #1710: the one-pass index gives every owner the entries each scanning reader gives, on this
    // mixed strip and VCA table with a `both` mute entry and a later second ride of a driven lane.
    {
        use controls::{
            AutomationIndex, ModelView, automated_fader_lanes, continued_fader_lanes,
            continued_fader_lanes_in, continued_mute_lanes, continued_mute_lanes_in,
            mute_automation, vca_mute_automation, vca_mute_row, vca_offset_automation,
        };
        let mut mixed = model.clone();
        mixed.automation.push(one(
            "both-mute",
            strip_builtin("guitar", MUTE, Lane::Both),
            ParameterUnit::Linear,
            1.0,
        ));
        mixed.automation.push(one(
            "late",
            strip_builtin("vocal", FADER_DB, Lane::Both),
            ParameterUnit::Db,
            -3.0,
        ));
        let index = AutomationIndex::of(&mixed);
        for strip in mixed.strips().map(|strip| strip.id.clone()) {
            assert_eq!(index.fader(&strip), fader_automation(&mixed, &strip));
            assert_eq!(index.mute(&strip), mute_automation(&mixed, &strip));
        }
        for vca in mixed.vcas.iter().map(|vca| &vca.id) {
            assert_eq!(index.vca_offset(vca), vca_offset_automation(&mixed, vca));
            assert_eq!(index.vca_mute(vca), vca_mute_automation(&mixed, vca));
        }
        // #1844: the event lanes through the VCA reach, and the VCA-mute flag, as the scans give,
        // also on the table before the second ride (where a VCA lane decides a fader event lane).
        for table in [&model, &mixed] {
            let index = AutomationIndex::of(table);
            let reach = table.vca_reach();
            for (strip, reach) in table.strips().map(|strip| strip.id.clone()).zip(&reach) {
                assert_eq!(
                    (
                        index.automated_fader_lanes(&strip),
                        index.fader_event_lanes(&strip, reach),
                        index.mute_event_lanes(&strip, reach),
                        index.any_vca_mute(),
                    ),
                    (
                        automated_fader_lanes(table, &strip),
                        fader_event_lanes(table, &strip, reach),
                        mute_event_lanes(table, &strip, reach),
                        table
                            .automation
                            .iter()
                            .any(|entry| vca_mute_row(&entry.target)),
                    ),
                );
            }
        }
        // #1864: the continued lanes through indexed views and `vca_reach` slices, as the scans
        // give, on every pair of these tables, a moved VCA ride and a member leaving its VCA.
        let mut moved = model.clone();
        moved.automation[0].segments[0].end_value = -9.0;
        let mut left = model.clone();
        left.vcas
            .iter_mut()
            .for_each(|vca| vca.members.retain(|m| m.as_str() != "vocal"));
        let tables = [&model, &mixed, &moved, &left];
        let indices = tables.map(AutomationIndex::of);
        let reaches = tables.map(SessionModel::vca_reach);
        for (a, b) in (0..4).flat_map(|a| (0..4).map(move |b| (a, b))) {
            let view = |at: usize| ModelView::whole(tables[at]).indexed(&indices[at]);
            for (strip, at) in model.strips().map(|strip| strip.id.clone()).zip(0..) {
                let (before, after) = (
                    (view(a), &reaches[a][at][..]),
                    (view(b), &reaches[b][at][..]),
                );
                assert_eq!(
                    (
                        continued_fader_lanes_in(before, after, &strip),
                        continued_mute_lanes_in(before, after, &strip),
                    ),
                    (
                        continued_fader_lanes(tables[a], tables[b], &strip),
                        continued_mute_lanes(tables[a], tables[b], &strip),
                    ),
                );
            }
        }
    }

    // The strip's own mute entry alone.
    model.automation = vec![one(
        "own-mute",
        strip_builtin("guitar", MUTE, Lane::Right),
        ParameterUnit::Linear,
        1.0,
    )];
    assert_eq!(lanes(&model, "guitar"), ([false; 2], [false, true]));
}

/// #1655 D5 (root's ruling of 2026-10-14): with equal rendered entries,
/// [`controls::rendered_automation_changed`] is `true` when the strips an automated VCA reaches
/// differ (a member added or removed, a nesting edit that reaches another strip), and `false` for a
/// reach change of a VCA no rendered entry drives, a nesting edit that reaches no other strip, and
/// a hand offset change.
///
/// Red if an automated VCA's membership edit commits live (no event state for the new pair), or if
/// a non-automated VCA's edit or a hand move rebuilds (D15-6's live set narrowed beyond the
/// ruling).
#[test]
fn a_reach_change_of_an_automated_vca_rebuilds() {
    use controls::rendered_automation_changed;
    let mut current = ridden_vca();
    current.automation.truncate(1);
    current.vcas.push(vca("plain", [0.0, 0.0], &["guitar"]));
    let changed = |edit: &dyn Fn(&mut SessionModel)| {
        let mut next = current.clone();
        edit(&mut next);
        rendered_automation_changed(&current, &next)
    };
    assert!(!changed(&|_| {}));
    // The automated VCA: a member added, a member removed, a nested VCA that reaches guitar.
    assert!(changed(&|next| next.vcas[0].members.push(id("guitar"))));
    assert!(changed(&|next| next.vcas[0].members.clear()));
    assert!(changed(&|next| next.vcas[0].members.push(id("plain"))));
    // A VCA no entry drives: its members, a parent over the automated VCA (riders' own reach is
    // unchanged), and a hand offset change.
    assert!(!changed(&|next| next.vcas[1].members.push(id("vocal"))));
    assert!(!changed(&|next| next.vcas[1].members.push(id("riders"))));
    assert!(!changed(&|next| next.automation[0].offset.left_db = 3.0));
    // The mute row counts too.
    current.automation = vec![one(
        "steps",
        vca_own("riders", VCA_MUTE, Lane::Both),
        ParameterUnit::Linear,
        1.0,
    )];
    let mut next = current.clone();
    next.vcas[0].members.push(id("guitar"));
    assert!(rendered_automation_changed(&current, &next));
}

/// #1655 D2 (root's ruling of 2026-10-14, class B order): on a member lane, `S` is the lane's own
/// hand offset, then the static offsets of the reaching VCAs whose lane is not automated, then the
/// hand offsets of the automated ones, each group in ascending VCA ID. The values are chosen so
/// the two orders give different `f64` bits: VCA `a` (static `1`), `b` (automated, hand
/// `2^-60`), `c` (static `-1`). `fader_db` stays the static composition. With no VCA offset
/// automation (none, or a VCA mute entry only) every composed value keeps the bits of the
/// ascending-ID sum, and the in-place path agrees with the dense one.
///
/// Red if the automated VCA's static offset (its fallback) enters `S`, if the hand offsets are
/// summed in ID order among the statics, or if a session without VCA offset automation moves a
/// bit of `S` or of the effective fader.
#[test]
fn the_offset_sum_puts_the_static_vca_offsets_first() {
    use controls::{LocalEffective, ModelView, MonitorOverlay, effective};
    use session::VcaSum;
    let tiny = f32::from_bits(0x2180_0000); // 2^-60
    assert_eq!(f64::from(tiny).to_bits(), 0x3C30_0000_0000_0000); // 2^-60
    let mut model = effects_fixture();
    model.vcas = vec![
        vca("a", [1.0, 1.0], &["vocal"]),
        vca("b", [-12.0, 5.0], &["vocal"]),
        vca("c", [-1.0, -1.0], &["vocal"]),
    ];
    let overlay = MonitorOverlay::default();
    let vocal = controls::strip_index(&model, &id("vocal")).expect("vocal");
    let ascending = |values: [f32; 3]| {
        let mut sum = VcaSum::new();
        values.into_iter().for_each(|value| sum.add(value));
        sum
    };

    // No VCA automation: S and the fader are the ascending-ID sum, bit for bit.
    let base = effective(&model, &overlay).strips[vocal];
    for (lane, b) in [-12.0, 5.0].into_iter().enumerate() {
        let sum = ascending([1.0, b, -1.0]);
        let own = &model.tracks[vocal].fader;
        let own = [own.left_db, own.right_db][lane];
        assert_eq!(
            base.offsets_sum[lane].to_bits(),
            sum.sum().unwrap().to_bits()
        );
        assert_eq!(base.fader_db[lane].to_bits(), sum.compose(own).to_bits());
    }
    // A VCA mute entry alone moves no bit.
    let mut muted = model.clone();
    muted.automation = vec![one(
        "steps",
        vca_own("b", VCA_MUTE, Lane::Both),
        ParameterUnit::Linear,
        1.0,
    )];
    assert_eq!(effective(&muted, &overlay).strips[vocal], base);

    // `b` ridden on the left lane with hand offset 2^-60.
    let mut ridden = model.clone();
    ridden.automation = vec![with_offset(
        one(
            "ride",
            vca_own("b", VCA_OFFSET_DB, Lane::Left),
            ParameterUnit::Db,
            -6.0,
        ),
        tiny,
        0.0,
    )];
    let strip = effective(&ridden, &overlay).strips[vocal];
    let d2 = ascending([1.0, -1.0, tiny]).sum().unwrap();
    let id_order = ascending([1.0, tiny, -1.0]).sum().unwrap();
    assert_ne!(
        d2.to_bits(),
        id_order.to_bits(),
        "the values tell the orders apart"
    );
    assert_eq!(strip.offsets_sum[0].to_bits(), d2.to_bits());
    assert_eq!(
        strip.offsets_sum[1].to_bits(),
        base.offsets_sum[1].to_bits(),
        "the right lane is not ridden"
    );
    assert_eq!(
        strip.fader_db.map(f32::to_bits),
        base.fader_db.map(f32::to_bits)
    );
    let local = LocalEffective::new(ModelView::whole(&ridden), &overlay).strip(vocal);
    assert_eq!(local, strip);

    // With the member's own fader ridden too, its hand offset comes first.
    ridden.automation.push(with_offset(
        one(
            "own",
            strip_builtin("vocal", FADER_DB, Lane::Left),
            ParameterUnit::Db,
            -6.0,
        ),
        0.5,
        0.0,
    ));
    let strip = effective(&ridden, &overlay).strips[vocal];
    let mut sum = VcaSum::new();
    [0.5, 1.0, -1.0, tiny]
        .into_iter()
        .for_each(|value| sum.add(value));
    assert_eq!(strip.offsets_sum[0].to_bits(), sum.sum().unwrap().to_bits());
}

/// #1655 D6 (V2): a member lane's carried curve continues verbatim only while its own table and
/// every reaching automated VCA table are the same set, bit for bit: a lane with no own entry
/// that a VCA ride reaches continues; a changed VCA table, a VCA ride added on the lane (a nested
/// VCA) and a member that leaves the VCA each reposition; the mute row takes the VCA mute tables.
///
/// Red if the carry predicates read only the own entry (a VCA table edit would continue a stale
/// cursor, or an event lane with no own entry would never continue).
#[test]
fn a_member_lane_continues_only_while_its_reaching_vca_tables_do() {
    use controls::{continued_fader_lanes, continued_mute_lanes};
    let current = ridden_vca();
    let vocal = id("vocal");
    let both = |next: &SessionModel| {
        (
            continued_fader_lanes(&current, next, &vocal),
            continued_mute_lanes(&current, next, &vocal),
        )
    };
    assert_eq!(both(&current.clone()), ([true, false], [true, true]));
    let mut moved = current.clone();
    moved.automation[0].segments[0].end_value = -9.0;
    assert_eq!(both(&moved), ([false, false], [true, true]));
    let mut nested = current.clone();
    nested.vcas.push(vca("outer", [0.0, 0.0], &["riders"]));
    nested.automation.push(one(
        "outer-ride",
        vca_own("outer", VCA_OFFSET_DB, Lane::Left),
        ParameterUnit::Db,
        -3.0,
    ));
    assert_eq!(both(&nested), ([false, false], [true, true]));
    let mut left = current.clone();
    left.vcas[0].members.clear();
    assert_eq!(both(&left), ([false, false], [false, false]));
    // An own fader entry on the lane: the own table and the VCA table must both continue.
    let mut own = current.clone();
    own.automation.push(one(
        "own",
        strip_builtin("vocal", 5, Lane::Left),
        ParameterUnit::Db,
        -2.0,
    ));
    let mut own_moved = own.clone();
    own_moved.automation[2].segments[0].end_value = -4.0;
    assert_eq!(
        continued_fader_lanes(&own, &own.clone(), &vocal),
        [true, false]
    );
    assert_eq!(
        continued_fader_lanes(&own, &own_moved, &vocal),
        [false, false]
    );
    assert_eq!(
        continued_fader_lanes(&current, &own, &vocal),
        [false, false]
    );
}
