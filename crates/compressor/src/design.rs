//! Control-rate coefficient design: parameter values in, lane words out.
//!
//! Nothing in this module runs per sample in steady state. It is called at preparation, at a
//! reset, at a restore, and once per frame for at most the 64 frames a `Linear 64` ramp is in
//! flight — and then only for the lanes and the coefficients whose parameter actually moved.
//! That is the whole of the divergence the 83c audit recorded against this crate: the pre-audit
//! code evaluated `expf(-1 / (0.001 * ms * fs))` **per sample, unconditionally**, on the render
//! path, through the platform libm.
//!
//! # One law
//!
//! [`design_lane`] is the only place a coefficient is derived, so a ramp that has finished
//! produces exactly the words a fresh preparation at the same value would. A second derivation
//! anywhere would make a block boundary observable (gate E3) and a finished ramp differ from a
//! fresh prepare (gate E6).
//!
//! # Why the exponential is `f64`
//!
//! `c = 1 - exp(-1 / (tau * fs))` is a catastrophic cancellation for a long time constant. At the
//! release parameter's maximum, 5,000 ms at 96 kHz, `tau * fs` is 480,000 and `exp(-1/480000)` is
//! `0.99999791666...`; subtracting that from one in `f32` leaves about seven significant bits of
//! `c`. Evaluating the whole expression in `f64` and rounding **once** to `f32` keeps all 24.
//!
//! This is why the crate calls [`math::exp`] (the vendored `f64` exponential, D6)
//! rather than `effect_runtime::envelope::attack_release_coefficient`, which is the
//! shared `1 - expf(-1 / tau)` in `f32` and loses those bits. The two agree to `f32` precision for
//! short time constants and diverge for long ones; the compressor's release domain reaches
//! 5,000 ms, so it needs the accurate form. Reported to #83 rather than changed there: altering
//! `attack_release_coefficient` would move the frozen D1 digests of `effect-runtime`.

use effect_runtime::dynamics::GainComputerCoef;
use effect_runtime::params::{ParameterKind, ParameterMapping, ParameterSpec};

use crate::COMPRESSOR_PARAMETERS;

/// Parameters in the descriptor table.
pub(crate) const PARAMETER_COUNT: usize = 8;

/// Parameters that are smoothed (`Linear 64`); `lookahead` is the eighth and is not automatable.
pub(crate) const RAMP_COUNT: usize = 7;

/// Lane coefficient words produced by [`design_lane`].
pub(crate) const COEF_COUNT: usize = 8;

/// Samples a `Linear 64` ramp takes to reach its target (descriptor `smoothing_samples`).
pub(crate) const SMOOTHING_SAMPLES: u32 = 64;

/// Threshold `T` in dB, passed through unchanged.
pub(crate) const COEF_THRESHOLD: usize = 0;
/// `1/R - 1`, the slope change above the threshold.
pub(crate) const COEF_INV_RATIO_MINUS_ONE: usize = 1;
/// `W/2`, the half knee width in dB.
pub(crate) const COEF_HALF_KNEE: usize = 2;
/// `1/(2W)`, `+0.0` for a hard knee.
pub(crate) const COEF_INV_TWO_KNEE: usize = 3;
/// Attack rate coefficient.
pub(crate) const COEF_ATTACK: usize = 4;
/// Release rate coefficient.
pub(crate) const COEF_RELEASE: usize = 5;
/// Makeup gain in dB, passed through unchanged.
pub(crate) const COEF_MAKEUP: usize = 6;
/// Dry/wet mix, passed through unchanged.
pub(crate) const COEF_MIX: usize = 7;

/// Parameter indices whose change redesigns the static curve: threshold, ratio, knee.
const CURVE_PARAMETERS: u8 = 0b0000_0111;

/// Every smoothed parameter, for a full design at prepare, reset or restore.
pub(crate) const ALL_PARAMETERS: u8 = 0b0111_1111;

/// The widest [`lane::Lane`]; the number of lane columns a coefficient word holds.
///
/// Not a track ceiling. A bank is exactly one lane wide per track, the engine imposes no ceiling on
/// how many tracks a session has, and a cohort simply uses as many banks as it needs; this constant
/// is the width of the widest SIMD register the build can use and nothing else.
pub(crate) const MAX_WIDTH: usize = 8;

/// Coefficient words, coefficient-major so that one `Lane::load` fills one lane vector.
pub(crate) type CoefWords = [[f32; MAX_WIDTH]; COEF_COUNT];

/// The runtime's domain description of one descriptor row.
///
/// The compressor keeps no domain predicate of its own: `params::parameter_value_valid` is the
/// workspace's one implementation, and this table is the descriptor rows expressed in the shape it
/// takes. `tests/contract.rs` asserts row by row that the two agree, so the table cannot drift
/// away from the descriptor it is derived from. (#95 makes the contract's own predicate public
/// and this table goes away with it.)
const fn spec(index: usize) -> ParameterSpec {
    let row = &COMPRESSOR_PARAMETERS[index];
    ParameterSpec {
        kind: ParameterKind::Continuous,
        minimum: match row.minimum {
            Some(value) => value,
            None => 0.0,
        },
        maximum: match row.maximum {
            Some(value) => value,
            None => 0.0,
        },
        mapping: match row.mapping {
            effect_contract::ParameterMapping::Logarithmic => ParameterMapping::Logarithmic,
            _ => ParameterMapping::Linear,
        },
        default: row.default_value,
    }
}

/// Domain, mapping and default of every parameter, in table order.
pub(crate) const PARAMETER_SPECS: [ParameterSpec; PARAMETER_COUNT] = [
    spec(0),
    spec(1),
    spec(2),
    spec(3),
    spec(4),
    spec(5),
    spec(6),
    spec(7),
];

/// One-pole *rate* coefficient `c = 1 - exp(-1 / (0.001 * time_ms * sample_rate))`.
///
/// Evaluated entirely in `f64` and rounded once, for the reason in the module documentation. The
/// result is held in `[0, 1]`: a coefficient outside that interval turns the smoother into a
/// divergent recurrence, and no rounding at the extremes may be able to produce one.
///
/// A non-positive or non-finite time gives `1.0`, the continuous extension as `tau` goes to zero
/// (an instantaneous smoother). The parameter domains make that unreachable through the contract;
/// it is the answer for a value that arrived some other way.
pub(crate) fn rate_coefficient(time_ms: f32, sample_rate: u32) -> f32 {
    let tau_samples = 0.001_f64 * f64::from(time_ms) * f64::from(sample_rate);
    // A NaN time fails this ordered compare and takes the instantaneous answer, which is why the
    // test is written as `> 0.0` on the positive side rather than as a negated comparison.
    if tau_samples <= 0.0 || tau_samples.is_nan() {
        return 1.0;
    }
    let value = (1.0 - math::exp(-1.0 / tau_samples)) as f32;
    if value < 0.0 {
        return 0.0;
    }
    if value > 1.0 {
        return 1.0;
    }
    value
}

/// Designs the coefficient words of one lane from that lane's current parameter values.
///
/// `changed` is a bitmask of the parameter indices that moved; only the coefficients that depend
/// on them are recomputed. `ALL_PARAMETERS` designs everything, which is what preparation, both
/// resets and a restore use.
///
/// The static-curve words come from [`GainComputerCoef`], the workspace's one transcription of
/// Giannoulis, Massberg and Reiss equation 4, so this crate and `multiband-compressor`
/// cannot drift apart on the knee.
pub(crate) fn design_lane(
    values: &[f32; RAMP_COUNT],
    sample_rate: u32,
    changed: u8,
    words: &mut CoefWords,
    lane: usize,
) {
    if changed & CURVE_PARAMETERS != 0 {
        let curve = GainComputerCoef::<f32>::new(values[0], values[1], values[2]);
        words[COEF_THRESHOLD][lane] = curve.threshold_db;
        words[COEF_INV_RATIO_MINUS_ONE][lane] = curve.inv_ratio_minus_one;
        words[COEF_HALF_KNEE][lane] = curve.half_knee_db;
        words[COEF_INV_TWO_KNEE][lane] = curve.inv_two_knee;
    }
    if changed & (1 << 3) != 0 {
        words[COEF_ATTACK][lane] = rate_coefficient(values[3], sample_rate);
    }
    if changed & (1 << 4) != 0 {
        words[COEF_RELEASE][lane] = rate_coefficient(values[4], sample_rate);
    }
    if changed & (1 << 5) != 0 {
        words[COEF_MAKEUP][lane] = values[5];
    }
    if changed & (1 << 6) != 0 {
        words[COEF_MIX][lane] = values[6];
    }
}

/// Detector read-back distance `D = N - L` in samples, derived only at prepare, restore and a
/// full reset (BRIEFS/013).
///
/// `L = floor(f64(lookahead_ms) * Fs / 1000 + 0.5)` clamped to the latency `N = ring_length - 1`,
/// and `D = N - L`, so `D == 0` reads the entry written this frame and `D == N` reads the oldest.
/// Kept in `f64` and unchanged from the pre-audit code: it is a frozen product rule, not an
/// implementation detail.
pub(crate) fn detector_delay(lookahead_ms: f32, sample_rate: u32, ring_length: usize) -> u32 {
    let latency = ring_length - 1;
    let lookahead =
        ((f64::from(lookahead_ms) * f64::from(sample_rate) / 1000.0) + 0.5).floor() as usize;
    let clamped = if lookahead < latency {
        lookahead
    } else {
        latency
    };
    (latency - clamped) as u32
}
