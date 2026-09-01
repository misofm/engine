#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Shared fixtures for the parametric-EQ acceptance gates.
//!
//! The frozen grids, the block driver and the payload accessors live here so that every gate reads
//! the same rows and the same words; the gates themselves stay about the property they prove.

#![allow(dead_code, unreachable_pub)]

use effect_contract::{
    AutomationSpanKind, EffectProcessBlock, EffectQuality as Quality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PrepareEffectLimits, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedNativeEffect, PreparedPorts, PreparedSidechainPort,
    ProcessReport, StatePayloadOutput,
};
use parametric_eq::{EQ_SECTION_COUNT, EqBandKind, PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory};

/// Bytes in each channel section of a version-2 payload.
pub const LANE_BYTES: usize = 304;
/// Bytes in the common section: the shared codec's two-word header (version, data word count).
/// The two channels share no state, so the effect adds no common words of its own.
pub const COMMON_BYTES: usize = 8;
/// Words one band occupies in a lane section.
pub const WORDS_PER_BAND: usize = 19;

/// The four launch sample rates.
pub const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
/// The frozen design frequencies of the 1,488-row grid.
pub const FROZEN_FREQUENCIES: [f32; 6] = [10.0, 20.0, 100.0, 1_000.0, 10_000.0, 20_000.0];
/// The frozen Q values of the 1,488-row grid.
pub const FROZEN_QS: [f32; 4] = [0.1, core::f32::consts::FRAC_1_SQRT_2, 1.0, 18.0];
/// The frozen gains of the 1,488-row grid.
pub const FROZEN_GAINS: [f32; 5] = [-24.0, -6.0, 0.0, 6.0, 24.0];
/// The frozen shelf slopes of the 1,488-row grid.
pub const FROZEN_SLOPES: [f32; 3] = [0.1, 0.5, 1.0];
/// Every family, in the frozen order.
pub const FROZEN_KINDS: [EqBandKind; 6] = [
    EqBandKind::Bell,
    EqBandKind::LowShelf,
    EqBandKind::HighShelf,
    EqBandKind::LowPass,
    EqBandKind::HighPass,
    EqBandKind::Notch,
];
/// The two frozen parameter corners each time-domain gate drives.
pub const FROZEN_EDGES: [(f32, f32, f32, f32); 2] =
    [(10.0, -24.0, 0.1, 0.1), (20_000.0, 24.0, 18.0, 1.0)];

/// One row of the frozen 1,488-row design grid.
#[derive(Clone, Copy, Debug)]
pub struct GridRow {
    /// Section family.
    pub kind: EqBandKind,
    /// Sample rate in Hz.
    pub rate: u32,
    /// Design frequency in Hz.
    pub frequency: f32,
    /// Gain in dB.
    pub gain: f32,
    /// Quality factor.
    pub q: f32,
    /// Shelf slope.
    pub slope: f32,
}

/// The 1,488 frozen rows, in the frozen order.
///
/// The loop shape is the one issue #42 froze and issues #44/#45 measured against; it is reproduced
/// literally so a row count assertion means the same thing it did then.
#[must_use]
pub fn frozen_grid() -> Vec<GridRow> {
    let mut rows = Vec::new();
    for rate in LAUNCH_RATES {
        for frequency in FROZEN_FREQUENCIES {
            for q in FROZEN_QS {
                for gain in FROZEN_GAINS {
                    rows.push(GridRow {
                        kind: EqBandKind::Bell,
                        rate,
                        frequency,
                        gain,
                        q,
                        slope: 1.0,
                    });
                }
                for kind in [EqBandKind::LowPass, EqBandKind::HighPass, EqBandKind::Notch] {
                    rows.push(GridRow {
                        kind,
                        rate,
                        frequency,
                        gain: 0.0,
                        q,
                        slope: 1.0,
                    });
                }
            }
            for gain in FROZEN_GAINS {
                for slope in FROZEN_SLOPES {
                    for kind in [EqBandKind::LowShelf, EqBandKind::HighShelf] {
                        rows.push(GridRow {
                            kind,
                            rate,
                            frequency,
                            gain,
                            q: 1.0,
                            slope,
                        });
                    }
                }
            }
        }
    }
    rows
}

/// The independent oracle's spelling of a section family.
#[must_use]
pub fn reference_kind(kind: EqBandKind) -> dsp_reference::ReferenceParametricEqKind {
    use dsp_reference::ReferenceParametricEqKind as Reference;
    match kind {
        EqBandKind::Bell => Reference::Bell,
        EqBandKind::LowShelf => Reference::LowShelf,
        EqBandKind::HighShelf => Reference::HighShelf,
        EqBandKind::LowPass => Reference::LowPass,
        EqBandKind::HighPass => Reference::HighPass,
        EqBandKind::Notch => Reference::Notch,
    }
}

/// The reference SVF's spelling of a section family.
#[must_use]
pub fn reference_svf_kind(kind: EqBandKind) -> dsp_reference::ReferenceSvfKind {
    use dsp_reference::ReferenceSvfKind as Reference;
    match kind {
        EqBandKind::Bell => Reference::Bell,
        EqBandKind::LowShelf => Reference::LowShelf,
        EqBandKind::HighShelf => Reference::HighShelf,
        EqBandKind::LowPass => Reference::LowPass,
        EqBandKind::HighPass => Reference::HighPass,
        EqBandKind::Notch => Reference::Notch,
    }
}

/// Every parameter at its descriptor default, per channel, in the order the contract requires.
#[must_use]
pub fn values() -> Vec<InitialParameterValue> {
    let mut values = Vec::new();
    for (index, parameter) in PARAMETRIC_EQ_DESCRIPTOR.parameters.iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel,
                value: parameter.default_value,
            });
        }
    }
    values
}

/// Overwrites one initial value.
pub fn set_initial(
    values: &mut [InitialParameterValue],
    parameter_index: usize,
    channel: ParameterChannel,
    value: f32,
) {
    let offset = parameter_index * 2
        + match channel {
            ParameterChannel::Left => 0,
            ParameterChannel::Right => 1,
            ParameterChannel::Both => panic!("initial values are per lane"),
        };
    values[offset].value = value;
}

/// Band one enabled with the given parameters on both channels; the other three stay disabled.
#[must_use]
pub fn single_section_values(
    kind: EqBandKind,
    frequency: f32,
    gain: f32,
    q: f32,
    slope: f32,
) -> Vec<InitialParameterValue> {
    let mut configured = values();
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set_initial(&mut configured, 0, channel, 1.0);
        set_initial(&mut configured, 1, channel, kind as u32 as f32);
        set_initial(&mut configured, 2, channel, frequency);
        set_initial(&mut configured, 3, channel, gain);
        set_initial(&mut configured, 4, channel, q);
        set_initial(&mut configured, 5, channel, slope);
    }
    configured
}

/// A prepare request at 48 kHz with a 128-frame quantum.
#[must_use]
pub fn request<'a>(values: &'a [InitialParameterValue], bypass: bool) -> PrepareEffectRequest<'a> {
    request_at_rate(values, bypass, 48_000)
}

/// A prepare request at `sample_rate` with a 128-frame quantum.
#[must_use]
pub fn request_at_rate<'a>(
    values: &'a [InitialParameterValue],
    bypass: bool,
    sample_rate: u32,
) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate,
        quantum: 128,
        quality: Quality::Normal,
        bypass,
        link_mode: LinkMode::DualMono,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        // Version 2 is 616 bytes; production admits megabytes (`maximum_effect_state_bytes` is
        // 100 MB over the C ABI and 16 MB in the web host), so 1,024 is a test-side headroom
        // number and not a contract change.
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1_024,
            maximum_scratch_bytes: 1,
            maximum_automation_spans_per_block: 48,
        },
    }
}

/// A point automation span.
#[must_use]
pub fn point(
    parameter_index: u32,
    channel: ParameterChannel,
    sample: u64,
    value: f32,
) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index,
        start_sample: sample,
        end_sample: sample,
        start_value: value,
        end_value: value,
    }
}

/// A whole payload: common header, left lane, right lane.
pub type Payload = ([u8; COMMON_BYTES], [u8; LANE_BYTES], [u8; LANE_BYTES]);

/// Snapshots a prepared scalar effect.
#[must_use]
pub fn snapshot(effect: &dyn PreparedNativeEffect) -> Payload {
    let mut common = [0_u8; COMMON_BYTES];
    let mut left = [0_u8; LANE_BYTES];
    let mut right = [0_u8; LANE_BYTES];
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(
                &mut common,
                &mut left,
                &mut right,
                effect.metadata().state_sizes,
            )
            .expect("state output"),
        )
        .expect("snapshot");
    (common, left, right)
}

/// Reads word `position` of a lane section.
#[must_use]
pub fn word(payload: &[u8], position: usize) -> u32 {
    u32::from_le_bytes(
        payload[position * 4..position * 4 + 4]
            .try_into()
            .expect("full state word"),
    )
}

/// Reads word `word_index` of band `band` of a lane section.
#[must_use]
pub fn band_word(payload: &[u8], band: usize, word_index: usize) -> u32 {
    word(payload, band * WORDS_PER_BAND + word_index)
}

/// Renders `frames` frames of silence with the given automation.
pub fn process_zeros(
    effect: &mut dyn PreparedNativeEffect,
    first_sample: u64,
    frames: usize,
    automation: &[PreparedAutomationSpan],
) -> ProcessReport {
    let mut left = vec![0.0; frames];
    let mut right = vec![0.0; frames];
    let quantum = effect.metadata().quantum;
    let block = EffectProcessBlock::new(
        &mut left,
        &mut right,
        None,
        first_sample,
        automation,
        quantum,
    )
    .expect("block");
    effect.process(block)
}

/// Drives a one-second impulse through the public factory in 128-frame blocks.
#[must_use]
pub fn one_second_impulse(
    kind: EqBandKind,
    frequency: f32,
    gain: f32,
    q: f32,
    slope: f32,
    rate: u32,
) -> (Vec<f32>, u64, u64) {
    let configured = single_section_values(kind, frequency, gain, q, slope);
    let mut effect = ParametricEqFactory
        .prepare(request_at_rate(&configured, false, rate))
        .expect("frozen impulse design must prepare");
    let mut left = vec![0.0_f32; rate as usize];
    let mut right = vec![0.0_f32; rate as usize];
    left[0] = 1.0;
    right[0] = 1.0;
    let mut recovered_left = 0_u64;
    let mut recovered_right = 0_u64;
    for first in (0..left.len()).step_by(128) {
        let end = (first + 128).min(left.len());
        let report = effect.process(
            EffectProcessBlock::new(
                &mut left[first..end],
                &mut right[first..end],
                None,
                first as u64,
                &[],
                128,
            )
            .expect("one-second block"),
        );
        recovered_left += report.nonfinite_left_blocks;
        recovered_right += report.nonfinite_right_blocks;
    }
    (left, recovered_left, recovered_right)
}

/// Magnitude in dB of the DFT of an `f32` window at `frequency`.
#[must_use]
pub fn impulse_dft_db(samples: &[f32], rate: u32, frequency: f64) -> f64 {
    dft_db(
        samples.iter().map(|sample| f64::from(*sample)),
        rate,
        frequency,
    )
}

/// Magnitude in dB of the DFT of a window at `frequency`, in the frozen accumulation order.
#[must_use]
pub fn dft_db(samples: impl Iterator<Item = f64>, rate: u32, frequency: f64) -> f64 {
    let phase = -core::f64::consts::TAU * frequency / f64::from(rate);
    let (step_re, step_im) = (phase.cos(), phase.sin());
    let (mut unit_re, mut unit_im) = (1.0_f64, 0.0_f64);
    let (mut re, mut im) = (0.0_f64, 0.0_f64);
    for sample in samples {
        re += sample * unit_re;
        im += sample * unit_im;
        (unit_re, unit_im) = (
            unit_re * step_re - unit_im * step_im,
            unit_re * step_im + unit_im * step_re,
        );
    }
    let magnitude = re.hypot(im);
    if magnitude == 0.0 {
        f64::NEG_INFINITY
    } else {
        20.0 * magnitude.log10()
    }
}

/// SplitMix64, the frozen seeded generator of the stability gates.
pub fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

/// A seeded value in `[0, 1]`.
pub fn seeded_unit_interval(state: &mut u64) -> f32 {
    let high_24 = (splitmix64(state) >> 40) as u32;
    high_24 as f32 / ((1_u32 << 24) - 1) as f32
}

/// A seeded sample in `[-0.99, -0.01] u [0.01, 0.99]`.
pub fn deterministic_noise(state: &mut u64) -> f32 {
    let word = splitmix64(state);
    let sign = if word & 1 == 0 { -1.0 } else { 1.0 };
    let magnitude = 0.01 + ((word >> 40) as u32 as f32 / ((1_u32 << 24) - 1) as f32) * 0.98;
    sign * magnitude
}

/// Number of cascade sections, re-exported so gates need one import.
pub const SECTIONS: usize = EQ_SECTION_COUNT;
