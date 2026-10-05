//! The SVF stability predicate shared by every effect that iterates the TPT state-variable filter.
//!
//! The parametric EQ checks its section designs and ramp paths with it, and the multiband
//! compressor's crossover words satisfy the same preconditions (`c1, a2, a3 < 1`), so one
//! definition serves both (#1366). Zavalishin, *The Art of VA Filter Design*, rev. 2.1.2, ch. 3-4
//! (the TPT SVF); Wishnick, DAFx-14, and Laroche, JAES 55(6), 2007 (a contractive transition matrix
//! bounds the state under arbitrary coefficient variation).

/// Spectral norm of the zero-input state transition `M = [[1-2c1, -2a2], [2a2, 1-2a3]]`, in `f64`.
///
/// A linear ramp between two word triples is safe exactly when every point of it is contractive,
/// and `‖M‖₂` is convex in the words, so checking the endpoints checks the whole ramp. This is the
/// stability predicate that replaces the delta realization's Jury test: it is a statement about the
/// matrix the kernel actually iterates, not about a polynomial nothing evaluates.
#[must_use]
pub fn transition_norm(c1: f32, a2: f32, a3: f32) -> f64 {
    let a00 = 1.0 - 2.0 * f64::from(c1);
    let a01 = -2.0 * f64::from(a2);
    let a10 = 2.0 * f64::from(a2);
    let a11 = 1.0 - 2.0 * f64::from(a3);
    let first = a00 * a00 + a10 * a10;
    let second = a01 * a01 + a11 * a11;
    let cross = a00 * a01 + a10 * a11;
    let difference = first - second;
    let largest =
        0.5 * (first + second + math::sqrt(difference * difference + 4.0 * cross * cross));
    math::sqrt(largest)
}

/// Largest spectral norm accepted from a rounded word set: one `f32` rounding above contractive.
pub const NORM_TOLERANCE: f64 = 1.0 + 1.0 / 4_194_304.0;

/// Largest spectral norm accepted on the remaining path of a restored in-flight ramp (#1278
/// attempt 3).
///
/// The path is the `f32` walk the render takes, `current + step` once per sample, between two
/// designs that each pass [`NORM_TOLERANCE`]. The transition matrix is affine in the words, so the
/// exact straight line between them is no less contractive than its worse end, but the walk is not
/// that line: its rounding, and a retarget that starts a new line from a rounded point, put it a
/// few ulps off, which a design-exact limit refuses: the conformance differential found the EQ
/// refusing its own mid-ramp snapshots, and a 10 kHz bell at Q 0.1 moving from -24 dB to +24 dB
/// was refused at every sample (`tests/carry.rs`).
///
/// The bound is derived, at every rate and through any chain of retargets. The norm `f` of the
/// transition matrix is convex in the words (the matrix is affine in them) and Lipschitz with
/// `L <= 4` in the max-norm of `(c1, a2, a3)`; each `f32` add rounds by at most `u = 2^-25`, since
/// the words stay below 1, and the scaled step lands within about `u` of the target. So a walk
/// starting at excess `E` stays below `1 + E` whenever `E >= 2^-22 + 65 * 4 * 2^-25`, about
/// `8.0e-6` (`2^-16.9`), and a fresh design starts within `2^-22`: every point the effect reaches
/// is within `1 + 8.0e-6`, 30 times inside this bound, and the same induction keeps a retarget
/// from an accepted forged start inside it. Cross-check: the largest excess measured over a grid
/// of every band kind and frequency, gain and Q moves was `3.7e-6` (`2^-18`) at 48 kHz, and a
/// four-rate simulation agrees. A forged path held to the bound gains at most
/// `(1 + 2^-12)^64 < 1.016` over the at most `RAMP_SAMPLES` samples it lasts before the target's
/// exact words snap in.
pub const RAMP_PATH_NORM_TOLERANCE: f64 = 1.0 + 1.0 / 4_096.0;
