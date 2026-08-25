//! Issue #127: the named nudge ladder, its arithmetic, and its refusals.
//!
//! Every test here is a statement about one rule, and every rule has a red mutation recorded in
//! `tests/MUTATIONS.md`.

#![allow(unsafe_code)]

use core::alloc::Layout;
use core::cell::Cell;
use std::alloc::{GlobalAlloc, System};

use miso_engine_effect_contract::{
    AutomationRate, EnumChoiceV1, NudgeErrorV1, NudgeLadderV1, NudgeRatioClassV1, NudgeRuleV1,
    NudgeSizeV1, NudgeStepUnitV1, ParameterChannelPolicy, ParameterDescriptorV1, ParameterDomain,
    ParameterId, ParameterMapping, ParameterUnit, SmoothingRule, check_nudge_ladder_v1,
    nudge_neighbours_v1, nudge_parameter_value_v1, resolve_nudge_ladder_v1,
};

// The counters are thread-local, not global: the test harness runs these tests concurrently, and a
// global counter would attribute another test's allocation to this one. `Cell<u64>` has no
// destructor, so reading it from inside the allocator cannot re-enter the allocator.
thread_local! {
    static ARMED: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
}

fn count_allocation() {
    if ARMED.try_with(Cell::get).unwrap_or(false) {
        let _ = ALLOCATIONS.try_with(|count| count.set(count.get() + 1));
    }
}

struct CountingAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

// SAFETY: every request is forwarded to `System` unchanged; the only added work is one relaxed
// atomic counter, read after the armed region and never affecting the allocation itself.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count_allocation();
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count_allocation();
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count_allocation();
        // SAFETY: forwards the original allocation arguments unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

const fn base() -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
        id: ParameterId(1),
        display_name: "Test",
        display_unit: "u",
        unit: ParameterUnit::Linear,
        domain: ParameterDomain::Continuous,
        minimum: Some(0.0),
        maximum: Some(1.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable: true,
        enum_choices: &[],
        nudge: None,
    }
}

/// A dB level on a linear mapping: `xs` is 0.5 dB anywhere in the domain.
fn decibels() -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
        unit: ParameterUnit::Db,
        minimum: Some(-60.0),
        maximum: Some(12.0),
        default_value: 0.0,
        nudge: Some(NudgeLadderV1::absolute(0.5)),
        ..base()
    }
}

/// A frequency on a logarithmic mapping: `xs` is 20 cents anywhere in the domain.
fn hertz() -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
        unit: ParameterUnit::Hz,
        mapping: ParameterMapping::Logarithmic,
        minimum: Some(20.0),
        maximum: Some(20_000.0),
        default_value: 1_000.0,
        nudge: Some(NudgeLadderV1::cents(20.0)),
        ..base()
    }
}

static CHOICES: [EnumChoiceV1; 6] = [
    EnumChoiceV1 {
        value: 0.0,
        label: "a",
    },
    EnumChoiceV1 {
        value: 1.0,
        label: "b",
    },
    EnumChoiceV1 {
        value: 2.0,
        label: "c",
    },
    EnumChoiceV1 {
        value: 3.0,
        label: "d",
    },
    EnumChoiceV1 {
        value: 4.0,
        label: "e",
    },
    EnumChoiceV1 {
        value: 5.0,
        label: "f",
    },
];

/// A six-choice enumeration: `xs` is one choice.
fn enumeration() -> ParameterDescriptorV1 {
    ParameterDescriptorV1 {
        domain: ParameterDomain::Enumeration,
        mapping: ParameterMapping::Stepped,
        minimum: None,
        maximum: None,
        default_value: 0.0,
        enum_choices: &CHOICES,
        nudge: Some(NudgeLadderV1::steps(1)),
        ..base()
    }
}

/// A linear-dB ladder steps by equal decibels, exactly as declared.
///
/// Red mutation: `resolve_nudge_ladder_parts_v1`'s `Absolute` arm divides by `b` instead of
/// `b - a`.
#[test]
fn a_linear_decibel_ladder_steps_by_equal_decibels() {
    let p = decibels();
    let mut value = 0.0f32;
    for expected in [0.5f32, 1.0, 1.5, 2.0] {
        value = nudge_parameter_value_v1(&p, value, NudgeSizeV1::Xs, 1).unwrap();
        assert!(
            (value - expected).abs() < 1e-4,
            "an xs rung is 0.5 dB: got {value}, wanted {expected}"
        );
    }
    // The multipliers are the ladder: one md is five xs rungs, one xl is thirty.
    assert!((nudge_parameter_value_v1(&p, 0.0, NudgeSizeV1::Md, 1).unwrap() - 2.5).abs() < 1e-4);
    assert!((nudge_parameter_value_v1(&p, 0.0, NudgeSizeV1::Xl, -1).unwrap() + 15.0).abs() < 1e-3);
}

/// A logarithmic ladder steps by an equal ratio -- twenty cents, at either end of the domain.
///
/// This is the property that makes a per-decade banding table unnecessary: the mapping supplies
/// equal-ratio stepping, and the declared unit is already the ratio unit.
///
/// Red mutation: the `Cents` arm divides by 1200 twice.
#[test]
fn a_logarithmic_ladder_steps_by_an_equal_ratio_everywhere() {
    let p = hertz();
    let cents = |from: f32, to: f32| 1200.0 * (f64::from(to / from)).log2();
    for start in [40.0f32, 1_000.0, 8_000.0] {
        let up = nudge_parameter_value_v1(&p, start, NudgeSizeV1::Xs, 1).unwrap();
        let snapped = nudge_parameter_value_v1(&p, start, NudgeSizeV1::Xs, 1).unwrap();
        assert_eq!(up.to_bits(), snapped.to_bits(), "the nudge is a function");
        let down = nudge_parameter_value_v1(&p, up, NudgeSizeV1::Xs, -1).unwrap();
        let step = cents(down, up);
        assert!(
            (step - 20.0).abs() < 0.5,
            "one xs rung is 20 cents at {start} Hz, got {step}"
        );
    }
}

/// A stepped ladder advances whole choices, and the larger rungs run off the end and clamp.
///
/// Red mutation: the `Steps` arm divides by `len` instead of `len - 1`.
#[test]
fn a_stepped_ladder_advances_whole_choices_and_clamps() {
    let p = enumeration();
    assert_eq!(
        nudge_parameter_value_v1(&p, 0.0, NudgeSizeV1::Xs, 1),
        Ok(1.0)
    );
    assert_eq!(
        nudge_parameter_value_v1(&p, 0.0, NudgeSizeV1::Sm, 1),
        Ok(3.0)
    );
    assert_eq!(
        nudge_parameter_value_v1(&p, 0.0, NudgeSizeV1::Md, 1),
        Ok(5.0)
    );
    // `lg` is ten choices and `xl` is thirty: both run off a six-choice enumeration, and the
    // clamp is exact rather than an error.
    assert_eq!(
        nudge_parameter_value_v1(&p, 0.0, NudgeSizeV1::Lg, 1),
        Ok(5.0)
    );
    assert_eq!(
        nudge_parameter_value_v1(&p, 5.0, NudgeSizeV1::Xl, -1),
        Ok(0.0)
    );
}

/// The declared endpoints are reached exactly, not nearly.
///
/// Red mutation: drop the `clamp(0.0, 1.0)` in `nudge_parameter_value_v1` and let `map_normalized`
/// refuse an out-of-range `x` instead.
#[test]
fn a_nudge_past_an_endpoint_lands_on_the_declared_endpoint_bits() {
    for p in [decibels(), hertz()] {
        let top = nudge_parameter_value_v1(&p, p.default_value, NudgeSizeV1::Xl, 1_000).unwrap();
        let bottom =
            nudge_parameter_value_v1(&p, p.default_value, NudgeSizeV1::Xl, -1_000).unwrap();
        assert_eq!(top.to_bits(), p.maximum.unwrap().to_bits());
        assert_eq!(bottom.to_bits(), p.minimum.unwrap().to_bits());
        // And the edge is a fixed point in the direction that would leave the domain.
        assert_eq!(
            nudge_parameter_value_v1(&p, top, NudgeSizeV1::Xs, 1)
                .unwrap()
                .to_bits(),
            top.to_bits()
        );
    }
}

/// The declared endpoint on the side a nudge of this sign travels toward.
fn endpoint(p: &ParameterDescriptorV1, upward: bool) -> f32 {
    match (p.domain, upward) {
        (ParameterDomain::Enumeration, true) => p.enum_choices.last().unwrap().value,
        (ParameterDomain::Enumeration, false) => p.enum_choices[0].value,
        (_, true) => p.maximum.unwrap(),
        (_, false) => p.minimum.unwrap(),
    }
}

/// Whether a value sits on either declared endpoint.
fn clamped(p: &ParameterDescriptorV1, value: f32) -> bool {
    value.to_bits() == endpoint(p, true).to_bits()
        || value.to_bits() == endpoint(p, false).to_bits()
}

/// From a grid point, a nudge is exactly reversible in every rung and both directions.
///
/// The first nudge from an arbitrary value snaps to the grid by at most half an `xs` rung; from
/// then on `+1` then `-1` restores the exact starting bits, because both directions are integer
/// arithmetic on the same grid index. Away from the endpoints, that is the whole round-trip claim.
///
/// Red mutation: `nudge_parameter_value_v1` adds `count * step` to `x` directly instead of
/// rounding to the grid first -- the round trip then drifts by an ulp at a time.
#[test]
fn a_grid_nudge_is_exactly_reversible() {
    for p in [decibels(), hertz(), enumeration()] {
        let mut checked = 0;
        for size in NudgeSizeV1::ALL {
            let start = nudge_parameter_value_v1(&p, p.default_value, size, 1).unwrap();
            for count in [1, 2, 7] {
                let up = nudge_parameter_value_v1(&p, start, size, count).unwrap();
                let back = nudge_parameter_value_v1(&p, up, size, -count).unwrap();
                // At a domain edge the clamp is one-way, and the documented asymmetry is exactly
                // that: the value that came back is the edge, and stepping down from an edge lands
                // on the grid rather than back where it started. Off the edge the round trip is
                // bit-exact.
                if clamped(&p, up) {
                    assert_eq!(
                        up.to_bits(),
                        endpoint(&p, count > 0).to_bits(),
                        "a clamped nudge lands on the declared endpoint"
                    );
                    continue;
                }
                checked += 1;
                assert_eq!(
                    back.to_bits(),
                    start.to_bits(),
                    "{} {}: +{count} then -{count} must restore the exact bits",
                    p.display_name,
                    size.as_str()
                );
            }
        }
        assert!(
            checked > 0,
            "{} never nudged clear of an edge",
            p.display_name
        );
    }
}

/// The five resolved rungs strictly ascend, for every launch-shaped parameter kind.
///
/// The multipliers make this true by construction, which is exactly why it is asserted: an
/// off-by-one or a duplicate in the multiplier table is otherwise invisible.
///
/// Red mutation: change `NudgeRatioClassV1::Human`'s multipliers from `[1, 3, 5, 10, 30]` to
/// `[1, 3, 3, 10, 30]`.
#[test]
fn the_multiplier_table_is_a_strict_ladder() {
    for class in [NudgeRatioClassV1::Human, NudgeRatioClassV1::Wide] {
        let multipliers = class.multipliers();
        assert_eq!(multipliers[0], 1, "xs is the unit rung");
        assert!(
            multipliers.windows(2).all(|pair| pair[0] < pair[1]),
            "{class:?} multipliers must strictly ascend"
        );
    }
    for p in [decibels(), hertz(), enumeration()] {
        let resolved = resolve_nudge_ladder_v1(&p).unwrap();
        let steps: Vec<f32> = NudgeSizeV1::ALL
            .into_iter()
            .map(|size| resolved.step_normalized(size))
            .collect();
        assert!(steps.windows(2).all(|pair| pair[0] < pair[1]));
        assert_eq!(check_nudge_ladder_v1(&p), Ok(()));
    }
}

/// Every way a ladder can be wrong names the rule it broke.
///
/// Red mutation: any single rule dropped from `check_nudge_ladder_parts_v1`.
#[test]
fn a_broken_ladder_names_the_rule_it_broke() {
    let step = ParameterDescriptorV1 {
        nudge: Some(NudgeLadderV1::absolute(0.0)),
        ..decibels()
    };
    assert_eq!(check_nudge_ladder_v1(&step), Err(NudgeRuleV1::Step));
    let negative = ParameterDescriptorV1 {
        nudge: Some(NudgeLadderV1::absolute(-0.5)),
        ..decibels()
    };
    assert_eq!(check_nudge_ladder_v1(&negative), Err(NudgeRuleV1::Step));
    let fractional_steps = ParameterDescriptorV1 {
        nudge: Some(NudgeLadderV1 {
            xs: 1.5,
            step_unit: NudgeStepUnitV1::Steps,
            ratio_class: NudgeRatioClassV1::Human,
        }),
        ..enumeration()
    };
    assert_eq!(
        check_nudge_ladder_v1(&fractional_steps),
        Err(NudgeRuleV1::Step)
    );

    // A boolean has nothing between its two values.
    let boolean = ParameterDescriptorV1 {
        domain: ParameterDomain::Boolean,
        mapping: ParameterMapping::Stepped,
        minimum: None,
        maximum: None,
        nudge: Some(NudgeLadderV1::absolute(0.5)),
        ..base()
    };
    assert_eq!(check_nudge_ladder_v1(&boolean), Err(NudgeRuleV1::Domain));
    // An absolute step has no constant-unit meaning on a logarithmic mapping.
    let wrong_unit = ParameterDescriptorV1 {
        nudge: Some(NudgeLadderV1::absolute(0.5)),
        ..hertz()
    };
    assert_eq!(check_nudge_ladder_v1(&wrong_unit), Err(NudgeRuleV1::Domain));
    // An exponential mapping has no constant-unit step at all.
    let exponential = ParameterDescriptorV1 {
        mapping: ParameterMapping::Exponential,
        nudge: Some(NudgeLadderV1::absolute(0.5)),
        ..decibels()
    };
    assert_eq!(
        check_nudge_ladder_v1(&exponential),
        Err(NudgeRuleV1::Domain)
    );
    // `xl` may not cross the whole continuous domain: 3 dB times thirty is 90 dB of a 72 dB range.
    let too_coarse = ParameterDescriptorV1 {
        nudge: Some(NudgeLadderV1::absolute(3.0)),
        ..decibels()
    };
    assert_eq!(check_nudge_ladder_v1(&too_coarse), Err(NudgeRuleV1::Domain));
    // A parameter that declares nothing breaks nothing.
    assert_eq!(check_nudge_ladder_v1(&base()), Ok(()));
}

/// Refusals are typed: an undeclared ladder, a zero count and an out-of-domain value are three
/// different answers, and none of them is a guess.
///
/// Red mutation: `nudge_parameter_value_v1` returns `Ok(current)` for a zero count.
#[test]
fn every_refusal_is_typed() {
    assert_eq!(
        nudge_parameter_value_v1(&base(), 0.5, NudgeSizeV1::Xs, 1),
        Err(NudgeErrorV1::Undeclared)
    );
    assert_eq!(
        nudge_parameter_value_v1(&decibels(), 0.0, NudgeSizeV1::Xs, 0),
        Err(NudgeErrorV1::Count)
    );
    assert_eq!(
        nudge_parameter_value_v1(&decibels(), 100.0, NudgeSizeV1::Xs, 1),
        Err(NudgeErrorV1::Value)
    );
    assert_eq!(
        nudge_parameter_value_v1(&decibels(), f32::NAN, NudgeSizeV1::Xs, 1),
        Err(NudgeErrorV1::Value)
    );
    // The size vocabulary is closed: an unknown name is refused here, once, rather than mapped to
    // a neighbouring rung anywhere downstream.
    assert_eq!(NudgeSizeV1::parse("xs"), Some(NudgeSizeV1::Xs));
    assert_eq!(NudgeSizeV1::parse("xl"), Some(NudgeSizeV1::Xl));
    assert_eq!(NudgeSizeV1::parse("XS"), None);
    assert_eq!(NudgeSizeV1::parse("xxs"), None);
    assert_eq!(NudgeSizeV1::parse(""), None);
    for size in NudgeSizeV1::ALL {
        assert_eq!(NudgeSizeV1::parse(size.as_str()), Some(size));
    }
}

/// A describe-style read answers with the value one rung either side, resolved exactly as a nudge
/// would resolve it.
#[test]
fn a_describe_read_answers_with_both_neighbours() {
    let p = decibels();
    let start = nudge_parameter_value_v1(&p, 0.0, NudgeSizeV1::Xs, 1).unwrap();
    for size in NudgeSizeV1::ALL {
        let (down, up) = nudge_neighbours_v1(&p, start, size).unwrap();
        assert_eq!(
            down.to_bits(),
            nudge_parameter_value_v1(&p, start, size, -1)
                .unwrap()
                .to_bits()
        );
        assert_eq!(
            up.to_bits(),
            nudge_parameter_value_v1(&p, start, size, 1)
                .unwrap()
                .to_bits()
        );
        assert!(
            down < start && start < up,
            "{} brackets the value",
            size.as_str()
        );
    }
    assert_eq!(
        nudge_neighbours_v1(&base(), 0.5, NudgeSizeV1::Xs),
        Err(NudgeErrorV1::Undeclared)
    );
}

/// Resolving and applying a ladder allocates nothing.
///
/// The engine this one replaces derived its step table per call and leaked it; the derivation here
/// is `Copy` arithmetic and the memo lives on the registry. This is the counter that proves it.
///
/// Red mutation: make `resolve_nudge_ladder_parts_v1` build a `Vec` of the five steps.
#[test]
fn resolving_and_applying_a_ladder_allocates_nothing() {
    let parameters = [decibels(), hertz(), enumeration()];
    // Warm anything lazy before arming.
    for p in &parameters {
        let _ = nudge_parameter_value_v1(p, p.default_value, NudgeSizeV1::Xs, 1);
    }
    ARMED.set(true);
    let mut sink = 0.0f32;
    for p in &parameters {
        for size in NudgeSizeV1::ALL {
            let resolved = resolve_nudge_ladder_v1(p).unwrap();
            sink += resolved.step_normalized(size);
            if let Ok(value) = nudge_parameter_value_v1(p, p.default_value, size, 3) {
                sink += value;
            }
        }
    }
    ARMED.set(false);
    assert!(sink.is_finite());
    assert_eq!(
        ALLOCATIONS.get(),
        0,
        "resolving and applying a ladder must not allocate"
    );
}
