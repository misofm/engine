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
/// drives on a row that takes it (here `fader_db`; the send gain is
/// `the_send_gain_takes_one_shared_offset`), inside that row's `[-144, 24]` dB. A lane the entry
/// does not drive, and every lane of a row that takes none (the input trim until #1658, a pan),
/// hold exactly `+0.0`, else `control.automation.offset`; an
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
        // Rows that take no offset (R17-6): the trim, a pan; and a send gain whose two fields
        // differ (its shared row holds one offset, #1654).
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

const ROUTE_MUTE: u32 = 2;
const ROUTE_MATRIX_LL: u32 = 3;
const ROUTE_MATRIX_LR: u32 = 4;
const ROUTE_MATRIX_RR: u32 = 6;

/// #1654 gate 8 (root's ruling R17-6): a send's `gain_db` takes one hand-move offset, held in both
/// fields of its shared row, inside `[-144, 24]` dB; `right_db` whose bits differ from `left_db`'s
/// is `control.automation.offset` at `.offset.right_db` (so `+0.0` with `-0.0` is refused, and an
/// out-of-domain pair is one `control.domain` at `.offset.left_db`). The send's `mute` and matrix
/// rows take no offset: a non-zero field there is `control.automation.offset`.
///
/// Red if the send gain still takes no offset (the accepted cases are refused), if its two fields
/// may differ (render reads one offset, so the other field would be acked and never heard), if a
/// shared row's domain check uses a single lane (every accepted case turns `control.domain`), or
/// if the offset reaches the send's mute or matrix rows.
#[test]
fn the_send_gain_takes_one_shared_offset() {
    let send = |name: &str, control: u32, unit: ParameterUnit, value: f32| {
        one(name, route_own("send", control), unit, value)
    };
    let gain = |name: &str| send(name, GAIN_DB, ParameterUnit::Db, -6.0);
    for accepted in [
        with_offset(gain("plus-three"), 3.0, 3.0),
        with_offset(gain("negative-zero"), -0.0, -0.0),
        with_offset(gain("top"), 24.0, 24.0),
        with_offset(gain("bottom"), -144.0, -144.0),
    ] {
        let name = accepted.id.as_str().to_owned();
        assert_eq!(judged(vec![accepted]), [], "{name}");
    }
    let refused = [
        (
            with_offset(gain("zeros-differ"), 0.0, -0.0),
            "control.automation.offset",
            ".offset.right_db",
        ),
        (
            with_offset(gain("fields-differ"), 1.0, 2.0),
            "control.automation.offset",
            ".offset.right_db",
        ),
        (
            with_offset(gain("above"), 30.0, 30.0),
            "control.domain",
            ".offset.left_db",
        ),
        (
            with_offset(
                send("matrix", ROUTE_MATRIX_LR, ParameterUnit::Linear, 0.5),
                0.5,
                0.0,
            ),
            "control.automation.offset",
            ".offset.left_db",
        ),
        (
            with_offset(
                send("mute", ROUTE_MUTE, ParameterUnit::Linear, 1.0),
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
}

/// #1654 D3: a send's rows 1-6 render ([`controls::automation_row_renders`]) and `follows_mute`
/// does not; only `gain_db` is the send's offset row; none reaches a fader-only reader
/// ([`controls::fader_row`]); [`controls::route_automation`] gives each row its first entry, in
/// row order; [`controls::automated_cell`] holds on a driven row; and
/// [`controls::continued_route_rows`] holds on a row whose table both models keep.
///
/// Red if a send row stays masked from rendering, if `follows_mute` joins the list, if the route
/// enumeration misplaces a row, or if an entry of another route is read.
#[test]
fn send_rows_render_and_enumerate_in_row_order() {
    use controls::{
        automated_cell, automation_row_renders, continued_route_rows, fader_row, route_automation,
        route_gain_row, route_row,
    };
    for control in [GAIN_DB, ROUTE_MUTE, ROUTE_MATRIX_LL, 5, ROUTE_MATRIX_RR] {
        let target = route_own("send", control);
        assert!(route_row(&target) && automation_row_renders(&target));
        assert!(!fader_row(&target));
        assert_eq!(route_gain_row(&target), control == GAIN_DB);
    }
    let follows = route_own("send", FOLLOWS_MUTE);
    assert!(!route_row(&follows) && !automation_row_renders(&follows));

    let mut model = effects_fixture();
    model.automation = vec![
        one(
            "lr",
            route_own("send", ROUTE_MATRIX_LR),
            ParameterUnit::Linear,
            0.25,
        ),
        one("gain", route_own("send", GAIN_DB), ParameterUnit::Db, -6.0),
        one(
            "other",
            route_own("bus-out", GAIN_DB),
            ParameterUnit::Db,
            -6.0,
        ),
    ];
    let rows = route_automation(&model, &id("send"));
    let names = rows.map(|entry| entry.map(|entry| entry.id.as_str().to_owned()));
    assert_eq!(
        names,
        [
            Some("gain".to_owned()),
            None,
            None,
            Some("lr".to_owned()),
            None,
            None
        ]
    );
    assert!(automated_cell(&model, &route_own("send", ROUTE_MATRIX_LR)));
    assert!(!automated_cell(&model, &route_own("send", ROUTE_MATRIX_LL)));

    let mut next = model.clone();
    next.automation[0].segments[0].start_value = 0.5;
    next.automation[0].segments[0].end_value = 0.5;
    assert_eq!(
        continued_route_rows(&model, &next, &id("send")),
        [true, false, false, false, false, false]
    );
}

/// The `send` session with a gain ride and a `matrix_lr` ride on the send.
fn ridden_send() -> SessionModel {
    let mut model = effects_fixture();
    model.automation = vec![
        one("gain", route_own("send", GAIN_DB), ParameterUnit::Db, -6.0),
        one(
            "lr",
            route_own("send", ROUTE_MATRIX_LR),
            ParameterUnit::Linear,
            0.25,
        ),
    ];
    model
}

/// #1654 R17-6 and R17-1 at the controls level: a `SET_CONTROL` on an automated send gain writes
/// the driving entry's offset (both fields, one value) and leaves the static gain (the fallback);
/// `get` and the list report the offset as the row's value with `automated = true`; an automated
/// matrix row keeps automation-wins (the hand value is the stored fallback, listed `automated`);
/// an un-automated row lists `automated = false`. The in-place path ([`controls::OwnerEdits`],
/// read through [`controls::ModelView`]) gives the dense model and the dense rows of the whole
/// `set`.
///
/// Red if a hand send gain move writes the static gain (automation wins, the move is acked and
/// never heard), if it writes one field only, if the list hides the offset or the flag, or if the
/// in-place path does not clone the entry with the route (its write would be lost on the owners
/// model).
#[test]
fn a_hand_move_on_an_automated_send_gain_writes_its_offset() {
    use controls::{
        ModelView, MonitorOverlay, OwnerEdits, effective, get, owner_index, owner_rows_in,
        owner_rows_with, set,
    };
    let committed = ridden_send();
    let gain = route_own("send", GAIN_DB);
    let lr = route_own("send", ROUTE_MATRIX_LR);
    let ll = route_own("send", ROUTE_MATRIX_LL);
    let route = |model: &SessionModel| {
        model
            .routes
            .iter()
            .find(|route| route.id == id("send"))
            .cloned()
            .expect("send")
    };
    let static_gain = route(&committed).gain_db;

    let mut whole = committed.clone();
    set(&mut whole, &gain, 3.0, None, RATE).expect("gain");
    set(&mut whole, &lr, 0.75, None, RATE).expect("matrix");
    assert_eq!(whole.automation[0].offset.left_db, 3.0);
    assert_eq!(whole.automation[0].offset.right_db, 3.0);
    assert_eq!(route(&whole).gain_db.to_bits(), static_gain.to_bits());
    assert_eq!(route(&whole).channel_matrix.lr, 0.75);
    let overlay = MonitorOverlay::default();
    assert_eq!(get(&whole, &overlay, &gain, RATE), Ok(3.0));
    let gain_row = common::row_of(&whole, &gain);
    assert!(gain_row.automated && gain_row.value == 3.0);
    let lr_row = common::row_of(&whole, &lr);
    assert!(lr_row.automated && lr_row.value == 0.75);
    assert!(!common::row_of(&whole, &ll).automated);

    // A send no entry drives keeps the static write.
    let mut plain = effects_fixture();
    set(&mut plain, &gain, 3.0, None, RATE).expect("plain gain");
    assert_eq!(route(&plain).gain_db, 3.0);
    assert!(!common::row_of(&plain, &gain).automated);

    // The in-place path: the same writes on the owners model.
    let mut edits = OwnerEdits::new(&committed);
    edits.set(&gain, 3.0, None, RATE).expect("in-place gain");
    edits.set(&lr, 0.75, None, RATE).expect("in-place matrix");
    let (owners, slots) = edits.finish();
    assert_eq!(owners.automation.len(), 1, "the gain entry alone is cloned");
    assert_eq!(slots.automation, [(0, 0)]);
    let view = ModelView::edited(&committed, &owners);
    assert_eq!(view.to_model(), whole);
    assert_eq!(
        view.route_automation(&id("send"))[0].map(|entry| entry.offset.left_db),
        Some(3.0)
    );
    let owner = owner_index(&committed, &controls::Owner::Route(id("send"))).expect("send");
    let dense = effective(&whole, &overlay);
    let (mut dense_rows, mut view_rows) = (Vec::new(), Vec::new());
    owner_rows_with(&whole, &dense, &overlay, owner, RATE, &mut dense_rows);
    owner_rows_in(view, &dense, &overlay, owner, RATE, &mut view_rows);
    assert_eq!(view_rows, dense_rows);
}

/// #1654 mask treatment: [`controls::mask_values`] copies a send-gain entry's offset as a control
/// value and keeps the static gain of an automated send as structure (its fallback), while an
/// un-automated send's gain and an automated send's matrix fallback stay control values.
///
/// Red if a send offset change is seen as structure (a live hand move would rebuild), or if a
/// change of an automated send's static gain is hidden as a value (it would never rebuild).
#[test]
fn the_mask_copies_a_send_offset_and_keeps_its_fallback() {
    use controls::mask_values;
    let mut from = ridden_send();
    from.automation[0].offset.left_db = 2.0;
    from.automation[0].offset.right_db = 2.0;
    for route in &mut from.routes {
        route.gain_db = -9.0;
        route.channel_matrix.lr = 0.5;
    }
    let mut into = ridden_send();
    let kept = into
        .routes
        .iter()
        .find(|route| route.id == id("send"))
        .expect("send")
        .gain_db;
    mask_values(&mut into, &from);
    assert_eq!(into.automation[0].offset.left_db, 2.0);
    assert_eq!(into.automation[0].offset.right_db, 2.0);
    for route in &into.routes {
        let expected = if route.id == id("send") { kept } else { -9.0 };
        assert_eq!(route.gain_db.to_bits(), expected.to_bits(), "{}", route.id);
        assert_eq!(route.channel_matrix.lr, 0.5);
    }
}
