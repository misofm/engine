// Vendored scalar libm layer.
//
// The function bodies in this module tree are vendored from rust-lang/libm 0.2.16 (MIT); see
// `LICENSE-libm.txt` for the licence text and `VENDORED.md` for the exact file list and the edits
// applied. Every target-conditional path, intrinsic fast path, `force_eval!` FP-flag statement and
// `no_panic` attribute has been removed, so the operation sequence is identical on x86_64,
// aarch64 and wasm32. That property is what gate M3 pins.
//
// `floor`/`sqrt` are not vendored verbatim: libm 0.2.16 only carries them in a generic form built
// on its private `support::Float` trait. They are re-derived here in the non-generic musl shape
// (`floor`) and from an exact integer square root (`sqrt`), and are proven correctly rounded by
// the crate's tests.
#![allow(
    clippy::all,
    clippy::approx_constant,
    clippy::eq_op,
    clippy::excessive_precision,
    clippy::float_cmp,
    clippy::many_single_char_names,
    clippy::needless_late_init,
    clippy::needless_return,
    clippy::unreadable_literal,
    clippy::zero_divided_by_zero
)]

/// Safe replacement for libm's `i!` indexing macro.
///
/// Upstream uses `get_unchecked` in release builds and `get(..).unwrap()` in debug ones.
/// Bounds-checked indexing produces the same values, keeps the crate free of `unsafe`, and is what
/// the workspace lint policy requires. It is also no worse for the render path than upstream's
/// debug form: every index here is provably in range, so the check is a branch that never fires,
/// not a new panic path.
macro_rules! i {
    ($array:expr, $index:expr) => {
        $array[$index]
    };
    ($array:expr, $index:expr, = , $rhs:expr) => {
        $array[$index] = $rhs;
    };
    ($array:expr, $index:expr, += , $rhs:expr) => {
        $array[$index] += $rhs;
    };
    ($array:expr, $index:expr, -= , $rhs:expr) => {
        $array[$index] -= $rhs;
    };
    ($array:expr, $index:expr, &= , $rhs:expr) => {
        $array[$index] &= $rhs;
    };
    ($array:expr, $index:expr, == , $rhs:expr) => {
        $array[$index] == $rhs
    };
}

/// Safe replacement for libm's `div!` macro (upstream uses `unchecked_div` in release builds).
macro_rules! div {
    ($a:expr, $b:expr) => {
        $a / $b
    };
}

mod atan;
mod atan2;
mod cos;
mod cosf;
mod exp;
mod exp2;
mod exp2f;
mod expf;
mod expm1;
mod expm1f;
mod floor;
mod k_cos;
mod k_cosf;
mod k_sin;
mod k_sinf;
mod k_tan;
mod k_tanf;
mod log;
mod log10;
mod log10f;
mod log2;
mod log2f;
mod logf;
mod pow;
mod powf;
mod rem_pio2;
mod rem_pio2_large;
mod rem_pio2f;
mod sin;
mod sinf;
mod sqrt;
mod tan;
mod tanf;
mod tanh;
mod tanhf;

pub(crate) use atan::atan;
pub(crate) use atan2::atan2;
pub(crate) use cos::cos;
pub(crate) use cosf::cosf;
pub(crate) use exp::exp;
pub(crate) use exp2::exp2;
pub(crate) use exp2f::exp2f;
pub(crate) use expf::expf;
pub(crate) use expm1::expm1;
pub(crate) use expm1f::expm1f;
pub(crate) use floor::{floor, floorf};
pub(crate) use k_cos::k_cos;
pub(crate) use k_cosf::k_cosf;
pub(crate) use k_sin::k_sin;
pub(crate) use k_sinf::k_sinf;
pub(crate) use k_tan::k_tan;
pub(crate) use k_tanf::k_tanf;
pub(crate) use log::log;
pub(crate) use log2::log2;
pub(crate) use log2f::log2f;
pub(crate) use log10::log10;
pub(crate) use log10f::log10f;
pub(crate) use logf::logf;
pub(crate) use pow::pow;
pub(crate) use powf::powf;
pub(crate) use rem_pio2::rem_pio2;
pub(crate) use rem_pio2_large::rem_pio2_large;
pub(crate) use rem_pio2f::rem_pio2f;
pub(crate) use sin::sin;
pub(crate) use sinf::sinf;
pub(crate) use sqrt::{sqrt, sqrtf};
pub(crate) use tan::tan;
pub(crate) use tanf::tanf;
pub(crate) use tanh::tanh;
pub(crate) use tanhf::tanhf;

/// `|x|` for `f64`. `f64::abs` is available in `core`, is a single bit mask on every target and is
/// exact, so it replaces libm's `fabs` file.
#[inline]
pub(crate) fn fabs(x: f64) -> f64 {
    x.abs()
}

/// `|x|` for `f32`; see [`fabs`].
#[inline]
pub(crate) fn fabsf(x: f32) -> f32 {
    x.abs()
}

#[inline]
pub(crate) fn get_high_word(x: f64) -> u32 {
    (x.to_bits() >> 32) as u32
}

#[inline]
pub(crate) fn with_set_high_word(f: f64, hi: u32) -> f64 {
    let mut tmp = f.to_bits();
    tmp &= 0x00000000_ffffffff;
    tmp |= (hi as u64) << 32;
    f64::from_bits(tmp)
}

#[inline]
pub(crate) fn with_set_low_word(f: f64, lo: u32) -> f64 {
    let mut tmp = f.to_bits();
    tmp &= 0xffffffff_00000000;
    tmp |= lo as u64;
    f64::from_bits(tmp)
}

/// `x * 2^n` for `f64`, exact when representable.
///
/// musl `src/math/scalbn.c`, written out here rather than vendored because libm 0.2.16 only ships
/// the generic form. The three-step prescale keeps every intermediate representable and avoids
/// double rounding in the subnormal range.
pub(crate) fn scalbn(x: f64, mut n: i32) -> f64 {
    let x1p1023 = f64::from_bits(0x7fe0000000000000); // 2^1023
    let x1p53 = f64::from_bits(0x4340000000000000); // 2^53
    let x1p_1022 = f64::from_bits(0x0010000000000000); // 2^-1022

    let mut y = x;

    if n > 1023 {
        y *= x1p1023;
        n -= 1023;
        if n > 1023 {
            y *= x1p1023;
            n -= 1023;
            if n > 1023 {
                n = 1023;
            }
        }
    } else if n < -1022 {
        // Make sure the final `n < -53` to avoid double rounding in the subnormal range.
        y *= x1p_1022 * x1p53;
        n += 1022 - 53;
        if n < -1022 {
            y *= x1p_1022 * x1p53;
            n += 1022 - 53;
            if n < -1022 {
                n = -1022;
            }
        }
    }

    y * f64::from_bits(((0x3ff + n) as u64) << 52)
}

/// `x * 2^n` for `f32`; see [`scalbn`].
pub(crate) fn scalbnf(x: f32, mut n: i32) -> f32 {
    let x1p127 = f32::from_bits(0x7f000000); // 2^127
    let x1p24 = f32::from_bits(0x4b800000); // 2^24
    let x1p_126 = f32::from_bits(0x00800000); // 2^-126

    let mut y = x;

    if n > 127 {
        y *= x1p127;
        n -= 127;
        if n > 127 {
            y *= x1p127;
            n -= 127;
            if n > 127 {
                n = 127;
            }
        }
    } else if n < -126 {
        y *= x1p_126 * x1p24;
        n += 126 - 24;
        if n < -126 {
            y *= x1p_126 * x1p24;
            n += 126 - 24;
            if n < -126 {
                n = -126;
            }
        }
    }

    y * f32::from_bits(((0x7f + n) as u32) << 23)
}
