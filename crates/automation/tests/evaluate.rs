//! Gate 1: the evaluation law and the hold rule.

mod common;

use automation::{Shape, value_at};
use common::seg;

fn table() -> [automation::Segment; 4] {
    [
        seg(100, 200, 1.0, 3.0, Shape::Linear),
        seg(300, 400, 5.0, 9.0, Shape::Step),
        seg(400, 1400, 20.0, 20_000.0, Shape::Exponential),
        seg(2000, 2100, 0.5, 0.25, Shape::Linear),
    ]
}

/// Red if the hold rule sits on the wrong side of a boundary: before the first segment, at a
/// segment's end, in a gap, and after the last.
#[test]
fn hold_rule_before_inside_after_and_between() {
    let t = table();
    assert_eq!(value_at(&t, -50), 1.0);
    assert_eq!(value_at(&t, 0), 1.0);
    assert_eq!(value_at(&t, 99), 1.0);
    assert_eq!(value_at(&t, 100), 1.0);
    assert_eq!(value_at(&t, 150), 2.0);
    assert_eq!(
        value_at(&t, 200),
        3.0,
        "a segment's end_value holds from its end"
    );
    assert_eq!(value_at(&t, 250), 3.0, "and across a gap");
    assert_eq!(value_at(&t, 299), 3.0);
    assert_eq!(value_at(&t, 300), 5.0);
    assert_eq!(
        value_at(&t, 399),
        5.0,
        "a step holds start_value on [start, end)"
    );
    assert_eq!(
        value_at(&t, 400),
        20.0,
        "the next segment starts at its start_value"
    );
    assert_eq!(value_at(&t, 1400), 20_000.0);
    assert_eq!(value_at(&t, 1800), 20_000.0);
    assert_eq!(value_at(&t, 2100), 0.25);
    assert_eq!(value_at(&t, 1 << 40), 0.25);
}

/// Red if a step is evaluated as a ramp or its end value is held inside the segment.
#[test]
fn step_end_value_applies_only_from_its_end() {
    let t = [seg(10, 20, 2.0, 7.0, Shape::Step)];
    assert_eq!(value_at(&t, 19), 2.0);
    assert_eq!(value_at(&t, 20), 7.0);
}

/// Red if linear is not `v0 + (v1 - v0)·x` at x = 0, 0.5 and 1.
#[test]
fn linear_at_zero_half_and_one() {
    let t = [seg(1000, 2000, -6.0, 6.0, Shape::Linear)];
    assert_eq!(value_at(&t, 1000), -6.0);
    assert_eq!(value_at(&t, 1500), 0.0);
    assert_eq!(value_at(&t, 2000), 6.0);
    let x = 250.0_f64 / 1000.0;
    assert_eq!(value_at(&t, 1250), (-6.0_f64 + 12.0 * x) as f32);
}

/// Red if exponential is evaluated in `f32`, with a platform `powf`, or with a different `x`.
#[test]
fn exponential_is_scalar_f64_through_math_pow() {
    let t = [seg(0, 48_000, 20.0, 20_000.0, Shape::Exponential)];
    let mut f32_differs = false;
    for at in [1_i64, 7, 480, 12_345, 24_000, 33_333, 47_999] {
        let x = at as f64 / 48_000.0;
        let expected =
            (20.0_f32 as f64 * math::pow(20_000.0_f32 as f64 / 20.0_f32 as f64, x)) as f32;
        assert_eq!(value_at(&t, at).to_bits(), expected.to_bits(), "t = {at}");
        let narrow = 20.0_f32 * (math::powf(20_000.0_f32 / 20.0_f32, x as f32));
        f32_differs |= narrow.to_bits() != expected.to_bits();
    }
    assert!(
        f32_differs,
        "the f32 formula must differ on at least one input"
    );
    assert_eq!(value_at(&t, 0), 20.0);
    assert_eq!(value_at(&t, 48_000), 20_000.0);
}

/// An empty table holds nothing; the builder refuses it, the evaluator does not panic.
#[test]
fn empty_table_is_zero() {
    assert_eq!(value_at(&[], 5), 0.0);
}
