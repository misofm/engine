//! Independent offline `f64` reference for the detector link glide (issue #1370 D2-D3).
//!
//! A lane's detector link moves between modes through two linear weights per channel: `w_link`
//! (0 = own, 1 = the linked combination) and `w_avg` (0 = maximum, 1 = average). Dual mono rests
//! at `(0, 0)`, maximum at `(1, 0)` and average at `(1, 1)`. While the weights move, a channel's
//! detector is the convex blend
//!
//! ```text
//! combined = max + w_avg * (avg - max)
//! d        = own + w_link * (combined - own)
//! ```
//!
//! with `own = |x|`, `max = max(|l|, |r|)` and `avg = 0.5 |l| + 0.5 |r|`, so `d` lies in
//! `[own, max(own, max)]` and never below the channel's own magnitude (Giannoulis, Massberg and
//! Reiss, JAES 60(6), 2012, for the link laws). The weights are evaluated exactly here, as the
//! straight line `w[k] = start + (target - start) * k / n` for `k = 1 ..= n`, rather than by the
//! production ramp's iterated additions; the production words agree within the rounding of those
//! additions. Nothing here imports production code.

/// One glide of the two weights, `[w_link, w_avg]`, from `start` to `target` in `samples` frames.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceLinkGlide {
    start: [f64; 2],
    target: [f64; 2],
    samples: u32,
    frame: u32,
}

impl ReferenceLinkGlide {
    /// A glide from `start` to `target` over `samples` frames; `samples == 0` is a step.
    #[must_use]
    pub const fn new(start: [f64; 2], target: [f64; 2], samples: u32) -> Self {
        Self {
            start,
            target,
            samples,
            frame: 0,
        }
    }

    /// The weights of the next frame, and whether the glide is still in flight after it (`false`
    /// from the frame it arrives on, which holds `target` exactly).
    pub fn next_weights(&mut self) -> ([f64; 2], bool) {
        if self.frame >= self.samples {
            return (self.target, false);
        }
        self.frame += 1;
        if self.frame == self.samples {
            return (self.target, false);
        }
        let k = f64::from(self.frame) / f64::from(self.samples);
        let weights = [
            self.start[0] + (self.target[0] - self.start[0]) * k,
            self.start[1] + (self.target[1] - self.start[1]) * k,
        ];
        (weights, true)
    }
}

/// The blended detector of both channels of one frame under `weights` (`[w_link, w_avg]`).
#[must_use]
pub fn reference_link_blend(left: f64, right: f64, weights: [f64; 2]) -> (f64, f64) {
    let (own_left, own_right) = (left.abs(), right.abs());
    let maximum = own_left.max(own_right);
    let average = 0.5 * own_left + 0.5 * own_right;
    let combined = maximum + weights[1] * (average - maximum);
    (
        own_left + weights[0] * (combined - own_left),
        own_right + weights[0] * (combined - own_right),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rest_weights_give_the_three_link_laws_and_the_glide_arrives_exactly() {
        let (l, r) = (0.5, -0.05);
        assert_eq!(reference_link_blend(l, r, [0.0, 0.0]), (0.5, 0.05));
        assert_eq!(reference_link_blend(l, r, [1.0, 0.0]), (0.5, 0.5));
        assert_eq!(reference_link_blend(l, r, [1.0, 1.0]), (0.275, 0.275));
        let mut glide = ReferenceLinkGlide::new([0.0, 0.0], [1.0, 1.0], 4);
        assert_eq!(glide.next_weights(), ([0.25, 0.25], true));
        assert_eq!(glide.next_weights(), ([0.5, 0.5], true));
        assert_eq!(glide.next_weights(), ([0.75, 0.75], true));
        assert_eq!(glide.next_weights(), ([1.0, 1.0], false));
        assert_eq!(glide.next_weights(), ([1.0, 1.0], false));
        assert_eq!(
            ReferenceLinkGlide::new([1.0, 0.0], [0.0, 0.0], 0).next_weights(),
            ([0.0, 0.0], false)
        );
    }
}
