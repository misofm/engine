//! The evaluation law: one segment table, one node time, one `f32`.

use crate::{Segment, Shape};

// REALTIME_POLICY_BEGIN
/// The curve's value at node time `t`.
///
/// The hold rule: before the first segment the first `start_value`; inside a segment its shape; at
/// and after a segment's `end` (and in a gap) its `end_value`. Inside a `linear` or `exponential`
/// segment `x = (t - start) as f64 / (end - start) as f64` (both differences are exact below
/// `2^53`), `linear` is `v0 + (v1 - v0)·x` and `exponential` is `v0·math::pow(v1/v0, x)`, all in
/// `f64` and rounded once to `f32`. A table with no segment holds `0.0`; the builder refuses one.
///
/// The segments must be ordered and disjoint (the builder and the session validator hold that) and
/// an `exponential` segment's values positive; a table that breaks either gives an unspecified
/// value, never a panic.
#[must_use]
pub fn value_at(segments: &[Segment], t: i64) -> f32 {
    let Some(first) = segments.first() else {
        return 0.0;
    };
    let Ok(t) = u64::try_from(t) else {
        return first.start_value;
    };
    let Some(index) = segments
        .partition_point(|segment| segment.start <= t)
        .checked_sub(1)
    else {
        return first.start_value;
    };
    let Some(segment) = segments.get(index) else {
        return first.start_value;
    };
    if t >= segment.end {
        return segment.end_value;
    }
    let v0 = f64::from(segment.start_value);
    let v1 = f64::from(segment.end_value);
    match segment.shape {
        Shape::Step => segment.start_value,
        Shape::Linear => (v0 + (v1 - v0) * fraction(segment, t)) as f32,
        Shape::Exponential => (v0 * math::pow(v1 / v0, fraction(segment, t))) as f32,
    }
}

/// `x = (t - t0) / (t1 - t0)` for `t` inside the segment, so the divisor is positive.
#[inline]
fn fraction(segment: &Segment, t: u64) -> f64 {
    (t - segment.start) as f64 / (segment.end - segment.start) as f64
}
// REALTIME_POLICY_END
