//! Polyphase half-band interpolation and decimation for the frozen 63-tap two-times oversampler.
//!
//! The filter is the 63-tap Blackman half-band of `.github/ISSUE_SPECS/BRIEFS/019`: symmetric,
//! centre tap exactly `0.5`, and every other odd tap exactly zero. That structure is what makes a
//! two-times oversampler cheap, and the audit of issue #91 found the soft clipper paying for it
//! anyway — four full 31-tap convolutions per input sample where the half-band form needs the work
//! of two, half of the products landing on zero-stuffed samples.
//!
//! # The polyphase identity
//!
//! With the high-rate stream `I[2n] = X[n]`, `I[2n+1] = 0` and `h` the 63 taps:
//!
//! * even phase — `u[2n] = sum over t of h[t] * I[2n-t]`; a nonzero product needs `t` even, so only
//!   `h[2k]`, `k = 1..=30` survive. `h[62]`, `h[0]` and the odd taps other than the centre are
//!   exactly zero, and the centre tap multiplies an odd (stuffed) index.
//! * odd phase — every even tap now lands on an odd index, so only the centre survives:
//!   `u[2n+1] = 0.5 * X[n-15]`, one exact multiply.
//! * decimation, kept (even) phase — `y[2n] = sum over t of h[t] * s[2n-t]` splits the same way:
//!   the even taps read the even-phase shaped samples and the centre tap reads one odd-phase
//!   shaped sample.
//!
//! Dropping the skipped products is **bit-preserving**, not an approximation: each of them is an
//! exact `±0.0` added to an accumulator that starts at `+0.0` and can never become `-0.0` (a sum
//! is `-0.0` only when both addends are, and an exact cancellation rounds to `+0.0` under
//! round-to-nearest-even). The surviving products are accumulated in the same ascending tap order
//! the 63-tap form used, so the rounding sequence is identical.
//!
//! # Layout: a double-written power-of-two history
//!
//! A history is `HALFBAND63_ROWS * L::WIDTH` floats: 32 live rows, each mirrored at `row + 32`.
//! [`history_push`] writes both copies, so the 31-sample window ending at the newest sample is
//! always contiguous and every tap is one vector load at a constant offset from `pos + 32` — no
//! modulus, no per-lane cursor, no gather. `32` is a power of two, so advancing the position is
//! one mask.
//!
//! Reference: Vaidyanathan, *Multirate Systems and Filter Banks*, chapter 4.6 (half-band
//! polyphase decomposition).

use crate::Lane;

/// Rows of a half-band history: 32 live rows, each mirrored at `row + 32`.
pub const HALFBAND63_ROWS: usize = 64;

/// Live rows of a half-band history; [`HALFBAND63_ROWS`] is twice this.
pub const HALFBAND63_LIVE_ROWS: usize = 32;

/// Mask that advances a history position, since [`HALFBAND63_LIVE_ROWS`] is a power of two.
pub const HALFBAND63_POS_MASK: usize = HALFBAND63_LIVE_ROWS - 1;

/// Row offset of the newest sample from the base used by the tap loops.
pub const HALFBAND63_BASE: usize = HALFBAND63_LIVE_ROWS;

/// Number of even taps `h[2k]`, `k = 1..=30`, that survive the polyphase decomposition.
pub const HALFBAND63_EVEN_TAPS: usize = 30;

/// The even taps `h[2k]` of the frozen 63-tap Blackman half-band, `k = 1..=30` at index `k - 1`.
///
/// These are the `f32` literals of `.github/ISSUE_SPECS/BRIEFS/019`, which the soft-clip crate's
/// `descriptor_resources_and_independent_fir_design_are_frozen` checks bit for bit against
/// `dsp_reference::reference_halfband_63()`, the independent `f64` design. The table
/// is symmetric (`h[2k] = h[62-2k]`), so index `k-1` and index `30-k` are equal; the symmetry is
/// **not** exploited here, because folding `h*(a+b)` out of `h*a + h*b` changes the bits.
pub const HALFBAND63_EVEN: [f32; HALFBAND63_EVEN_TAPS] = [
    4.117_896_6e-5,
    -1.843_658_7e-4,
    4.762_265_3e-4,
    -9.890_399e-4,
    1.823_257_9e-3,
    -3.110_171_5e-3,
    5.017_224_7e-3,
    -7.761_148e-3,
    1.163_983_6e-2,
    -1.710_855_8e-2,
    2.496_969_9e-2,
    -3.690_095e-2,
    5.726_340_8e-2,
    -1.021_490_2e-1,
    3.169_724_3e-1,
    3.169_724_3e-1,
    -1.021_490_2e-1,
    5.726_340_8e-2,
    -3.690_095e-2,
    2.496_969_9e-2,
    -1.710_855_8e-2,
    1.163_983_6e-2,
    -7.761_148e-3,
    5.017_224_7e-3,
    -3.110_171_5e-3,
    1.823_257_9e-3,
    -9.890_399e-4,
    4.762_265_3e-4,
    -1.843_658_7e-4,
    4.117_896_6e-5,
];

/// The centre tap `h[31]`, exactly one half.
pub const HALFBAND63_CENTER: f32 = 0.5;

/// Index in [`HALFBAND63_EVEN`] where the centre tap is accumulated by the decimator.
///
/// The 63-tap ascending order is `h[2], h[4], .., h[30], h[31], h[32], .., h[60]`, so the centre
/// tap's contribution sits between `k = 15` and `k = 16`. Keeping it there is what preserves the
/// bits.
pub const HALFBAND63_CENTER_SPLIT: usize = 15;

/// One row of an AoSoA history as a slice of `L::WIDTH` values.
///
/// # Panics
///
/// Panics if `row` is not a whole row of `history`.
#[inline(always)]
#[must_use]
pub fn history_row<L: Lane>(history: &[f32], row: usize) -> &[f32] {
    &history[row * L::WIDTH..(row + 1) * L::WIDTH]
}

/// Writes `value` to rows `pos` and `pos + 32` of an AoSoA half-band history.
///
/// Both copies are written so that the 31-row window ending at `pos + 32` is contiguous whatever
/// `pos` is. `history` is `HALFBAND63_ROWS * L::WIDTH` floats; `pos` is in `0..32`.
///
/// # Panics
///
/// Panics in debug builds if `history` is not [`HALFBAND63_ROWS`] rows, or `pos` is not a live row.
#[inline(always)]
pub fn history_push<L: Lane>(history: &mut [f32], pos: usize, value: L) {
    debug_assert_eq!(history.len(), HALFBAND63_ROWS * L::WIDTH);
    debug_assert!(pos < HALFBAND63_LIVE_ROWS);
    let width = L::WIDTH;
    value.store(&mut history[pos * width..]);
    value.store(&mut history[(pos + HALFBAND63_LIVE_ROWS) * width..]);
}

/// Advances a history position by one row.
#[inline(always)]
#[must_use]
pub const fn history_advance(pos: usize) -> usize {
    (pos + 1) & HALFBAND63_POS_MASK
}

/// Even-phase output of the two-times interpolator: `u = sum_{k=1..=30} h[2k] * X[n-k]`.
///
/// `base` is `pos + 32` after [`history_push`] has stored `X[n]` at `pos`, so row `base - k` holds
/// `X[n-k]` for `k = 1..=31`.
///
/// Frozen operation order: `acc = +0.0`; then for `k = 1, 2, ..., 30` in that order,
/// `p = splat(h[2k]) * load(row base-k)` and `acc = acc + p`. Two operations per tap, no fusion:
/// the 63-tap form this replaces had none either, and adding one would change every pinned bit.
///
/// # Panics
///
/// Panics if `history` is shorter than `base` rows, or `base` is below 31 rows.
#[inline(always)]
#[must_use]
pub fn halfband2x_interp_even<L: Lane>(history: &[f32], base: usize) -> L {
    debug_assert!(base > HALFBAND63_EVEN_TAPS);
    debug_assert!(base * L::WIDTH <= history.len());
    let width = L::WIDTH;
    let window = &history[(base - HALFBAND63_EVEN_TAPS) * width..base * width];
    let mut acc = L::zero();
    // `rev()` walks the window newest row first, which is `k` ascending.
    for (row, tap) in window.chunks_exact(width).rev().zip(HALFBAND63_EVEN) {
        acc = acc.add(L::splat(tap).mul(L::load(row)));
    }
    acc
}

/// Kept (even) phase of the two-times decimator, with the odd-phase sample supplied by the caller.
///
/// `y = sum_{k=1..=15} h[2k] * e[n-k]  +  0.5 * odd  +  sum_{k=16..=30} h[2k] * e[n-k]`, in that
/// order — the centre tap sits between `k = 15` and `k = 16` because that is where the ascending
/// 63-tap order puts it, and moving it changes the bits.
///
/// `base` is `pos + 32` after [`history_push`] has stored `e[n]` at `pos`; `odd` is the shaped
/// odd-phase sample `s[2n-31]`, which the caller recomputes rather than storing.
///
/// # Panics
///
/// Panics if `history` is shorter than `base` rows, or `base` is below 31 rows.
#[inline(always)]
#[must_use]
pub fn halfband2x_decim_even<L: Lane>(history: &[f32], base: usize, odd: L) -> L {
    debug_assert!(base > HALFBAND63_EVEN_TAPS);
    debug_assert!(base * L::WIDTH <= history.len());
    let width = L::WIDTH;
    let window = &history[(base - HALFBAND63_EVEN_TAPS) * width..base * width];
    let mut acc = L::zero();
    for (index, (row, tap)) in window
        .chunks_exact(width)
        .rev()
        .zip(HALFBAND63_EVEN)
        .enumerate()
    {
        if index == HALFBAND63_CENTER_SPLIT {
            acc = acc.add(L::splat(HALFBAND63_CENTER).mul(odd));
        }
        acc = acc.add(L::splat(tap).mul(L::load(row)));
    }
    acc
}
