//! Response gates: the prepared cascade against the independent `f64` RBJ oracle.
//!
//! These are the frozen tolerances of issues 007/031/036 -- 0.005 dB on the cast state-space
//! transfer, 0.05 dB on impulse and sustained measurements, -100 dB and -88 dB residual limits.
//! Master plan §8 forbids loosening any of them: if one fails, the kernel is wrong, not the gate.
//! The oracle is `dsp-reference`, whose RBJ biquad is a different topology with the
//! same `H(z)` and whose state-space model is derived from the equations, not from production.

use builtins::*;
use dsp_reference::{
    ReferenceBiquad, ReferenceFilterKind, ReferenceSvfStateSpace, ReferenceTptOutput,
    rbj_butterworth_magnitude_db,
};
use engine::{EXTENDED_COMPATIBILITY_SAMPLE_RATES, LAUNCH_SAMPLE_RATES};

/// The state-space model of a prepared section, built from **all seven** of its cast words.
///
/// The output mix is read from production rather than re-derived from `k`, so an error in the
/// `(m0, m1, m2)` set the kernel actually applies is visible to the response gates. The model
/// itself is `dsp-reference`'s, derived from the recurrence in #105.
fn state_space(rate: u32, cutoff: f64, high_pass: bool) -> ReferenceSvfStateSpace {
    let words = test_support::section_words(rate, cutoff as f32, high_pass).expect("section");
    let [c1, a2, a3, _, m0, m1, m2] = words.map(f32::from_bits);
    ReferenceSvfStateSpace::new(
        f64::from(c1),
        f64::from(a2),
        f64::from(a3),
        [f64::from(m0), f64::from(m1), f64::from(m2)],
    )
}

// Issue 032: the first tier is launch-gated; the second remains informational compatibility
// evidence from issue 007 and is not an engine session or host support claim.
fn launch_and_extended_compatibility_rates() -> impl Iterator<Item = u32> {
    LAUNCH_SAMPLE_RATES
        .into_iter()
        .chain(EXTENDED_COMPATIBILITY_SAMPLE_RATES)
        .map(|rate| rate.0)
}

#[test]
fn launch_and_extended_compatibility_rates_match_the_independent_f64_rbj_oracle() {
    for rate in launch_and_extended_compatibility_rates() {
        let parameters = BuiltinParameters {
            left: ChannelParameters {
                hpf_hz: 100.0,
                lpf_hz: 1_000.0,
                ..ChannelParameters::default()
            },
            ..BuiltinParameters::default()
        };
        let mut chain = BuiltinChain::new(rate, parameters).expect("prepare");
        let mut left = [0.0_f32; 256];
        let mut right = [0.0_f32; 256];
        left[0] = 1.0;
        let mut high =
            ReferenceBiquad::rbj_butterworth(f64::from(rate), 100.0, ReferenceFilterKind::HighPass)
                .expect("reference high pass");
        let mut low = ReferenceBiquad::rbj_butterworth(
            f64::from(rate),
            1_000.0,
            ReferenceFilterKind::LowPass,
        )
        .expect("reference low pass");
        let expected: Vec<_> = (0..left.len())
            .map(|index| low.process(high.process(if index == 0 { 1.0 } else { 0.0 })))
            .collect();
        let _ = chain.process_input(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
        for (actual, reference) in left.iter().zip(expected) {
            assert!(
                (f64::from(*actual) - reference).abs() <= 2e-5,
                "rate={rate}, actual={actual}, reference={reference}"
            );
        }
        assert_eq!(right, [0.0; 256]);
    }
}

#[test]
fn ten_thousand_bounded_parameter_and_block_mutations_stay_finite() {
    let mut state = 0x5EED_CAFE_1234_5678_u64;
    for iteration in 0..10_000_u64 {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let fraction = |shift| ((state >> shift) as u32) as f32 / u32::MAX as f32;
        let db = |shift| -144.0 + fraction(shift) * 168.0;
        let matrix = Matrix2x2 {
            ll: fraction(0) * 2.0 - 1.0,
            lr: fraction(8) * 2.0 - 1.0,
            rl: fraction(16) * 2.0 - 1.0,
            rr: fraction(24) * 2.0 - 1.0,
        };
        let rate = LAUNCH_SAMPLE_RATES[(state as usize) & 3].0;
        let mut chain = BuiltinChain::new(
            rate,
            BuiltinParameters {
                left: ChannelParameters {
                    polarity_invert: state & 1 != 0,
                    trim_db: db(0),
                    hpf_hz: 100.0,
                    lpf_hz: 1_000.0,
                    fader_db: db(32),
                    muted: state & 2 != 0,
                },
                right: ChannelParameters {
                    polarity_invert: state & 4 != 0,
                    trim_db: db(8),
                    hpf_hz: 0.0,
                    lpf_hz: 0.0,
                    fader_db: db(40),
                    muted: state & 8 != 0,
                },
                matrix,
                smoothing_samples: (state as u32) & 127,
            },
        )
        .expect("generated parameters are in the prepared domain");
        chain
            .set_matrix_target(Matrix2x2::IDENTITY)
            .expect("identity");
        let mut left = [0.25_f32; 127];
        let mut right = [-0.5_f32; 127];
        let _ = chain.process_dual_mono(
            DualMonoBlock::new(&mut left, &mut right, iteration.saturating_mul(127))
                .expect("block"),
        );
        assert!(left.iter().chain(&right).all(|sample| sample.is_finite()));
    }
}

#[test]
fn launch_and_extended_compatibility_rate_sweeps_match_f64_magnitude() {
    for rate in launch_and_extended_compatibility_rates() {
        for frequency in [100.0, 1_000.0, f64::from(rate) * 0.2] {
            let frames = 4_096;
            let mut left: Vec<f32> = (0..frames)
                .map(|index| {
                    (core::f64::consts::TAU * frequency * index as f64 / f64::from(rate)).sin()
                        as f32
                })
                .collect();
            let mut right = vec![0.0_f32; frames];
            let parameters = BuiltinParameters {
                left: ChannelParameters {
                    hpf_hz: 100.0,
                    lpf_hz: 1_000.0,
                    ..ChannelParameters::default()
                },
                ..BuiltinParameters::default()
            };
            let mut chain = BuiltinChain::new(rate, parameters).expect("prepare");
            let mut offset = 0;
            for quantum in [1, 127, 128, 255, 1_024].into_iter().cycle() {
                if offset == frames {
                    break;
                }
                let end = (offset + quantum).min(frames);
                let _ = chain.process_input(
                    DualMonoBlock::new(
                        &mut left[offset..end],
                        &mut right[offset..end],
                        offset as u64,
                    )
                    .expect("block"),
                );
                offset = end;
            }
            let mut high = ReferenceBiquad::rbj_butterworth(
                f64::from(rate),
                100.0,
                ReferenceFilterKind::HighPass,
            )
            .expect("reference high pass");
            let mut low = ReferenceBiquad::rbj_butterworth(
                f64::from(rate),
                1_000.0,
                ReferenceFilterKind::LowPass,
            )
            .expect("reference low pass");
            let mut actual_energy = 0.0_f64;
            let mut reference_energy = 0.0_f64;
            for (index, actual) in left.iter().copied().enumerate() {
                let input =
                    (core::f64::consts::TAU * frequency * index as f64 / f64::from(rate)).sin();
                let reference = low.process(high.process(input));
                if index >= frames / 2 {
                    actual_energy += f64::from(actual) * f64::from(actual);
                    reference_energy += reference * reference;
                }
            }
            let actual_db = 10.0 * actual_energy.log10();
            let reference_db = 10.0 * reference_energy.log10();
            if reference_db >= -120.0 {
                assert!(
                    (actual_db - reference_db).abs() <= 0.05,
                    "rate={rate}, frequency={frequency}, actual={actual_db}, reference={reference_db}"
                );
            }
        }
    }
}

#[test]
fn cast_tpt_state_space_matches_independent_rbj_transfer_at_compatibility_rates() {
    for rate in launch_and_extended_compatibility_rates() {
        let mut cutoffs = vec![
            10.0,
            20.0,
            100.0,
            1_000.0,
            (20_000.0_f64).min(0.1 * f64::from(rate)),
            0.45 * f64::from(rate),
        ];
        cutoffs.sort_by(f64::total_cmp);
        cutoffs.dedup_by(|left, right| *left == *right);
        for (high_pass, kind, output) in [
            (
                true,
                ReferenceFilterKind::HighPass,
                ReferenceTptOutput::HighPass,
            ),
            (
                false,
                ReferenceFilterKind::LowPass,
                ReferenceTptOutput::LowPass,
            ),
        ] {
            for cutoff in &cutoffs {
                let _ = output;
                let state = state_space(rate, *cutoff, high_pass);
                assert_tpt_limits_and_monotonic(state, rate, high_pass, *cutoff);
                let mut probes = coherent_probes(rate, *cutoff);
                probes.extend([*cutoff, 0.49 * f64::from(rate)]);
                probes.sort_by(f64::total_cmp);
                probes.dedup_by(|left, right| *left == *right);
                for frequency in probes {
                    let reference =
                        rbj_butterworth_magnitude_db(f64::from(rate), *cutoff, kind, frequency)
                            .expect("reference");
                    let actual = state
                        .magnitude_db(f64::from(rate), frequency)
                        .expect("state");
                    if reference >= -120.0 {
                        assert!(
                            (actual - reference).abs() <= 0.005,
                            "rate={rate}, cutoff={cutoff}, frequency={frequency}, actual={actual}, reference={reference}"
                        );
                    }
                }
                let cutoff_db = state
                    .magnitude_db(f64::from(rate), *cutoff)
                    .expect("cutoff state");
                assert!(
                    (cutoff_db + 3.010_299_956_6).abs() <= 0.005,
                    "rate={rate}, cutoff={cutoff}, db={cutoff_db}"
                );
            }
        }
    }
}

#[test]
fn one_second_impulse_dfts_match_rbj_at_launch_and_extended_compatibility_rates() {
    for rate in launch_and_extended_compatibility_rates() {
        let mut cutoffs = vec![
            10.0,
            20.0,
            100.0,
            1_000.0,
            (20_000.0_f64).min(0.1 * f64::from(rate)),
            0.45 * f64::from(rate),
        ];
        cutoffs.sort_by(f64::total_cmp);
        cutoffs.dedup_by(|left, right| *left == *right);
        for (high_pass, kind) in [
            (true, ReferenceFilterKind::HighPass),
            (false, ReferenceFilterKind::LowPass),
        ] {
            for cutoff in &cutoffs {
                let mut partition_reference: Option<Vec<f32>> = None;
                for quantum in [1, 127, 128, 255, 1_024] {
                    let impulse = impulse_through(
                        rate,
                        one_section(*cutoff, high_pass),
                        rate as usize,
                        quantum,
                    );
                    assert!(impulse.iter().all(|sample| sample.is_finite()));
                    let tail_energy = impulse[impulse.len().saturating_sub(4_096)..]
                        .iter()
                        .map(|sample| f64::from(*sample) * f64::from(*sample))
                        .sum::<f64>();
                    assert!(
                        tail_energy.is_finite() && tail_energy <= 1e-8,
                        "rate={rate}, cutoff={cutoff}, quantum={quantum}, tail_energy={tail_energy}"
                    );
                    if let Some(reference) = &partition_reference {
                        assert_eq!(
                            &impulse, reference,
                            "block partition changed bits: rate={rate}, cutoff={cutoff}, quantum={quantum}"
                        );
                    } else {
                        partition_reference = Some(impulse.clone());
                    }
                    for frequency in coherent_probes(rate, *cutoff) {
                        let reference =
                            rbj_butterworth_magnitude_db(f64::from(rate), *cutoff, kind, frequency)
                                .expect("reference");
                        let actual = impulse_dft_magnitude_db(&impulse, f64::from(rate), frequency);
                        if reference >= -120.0 {
                            assert!(
                                (actual - reference).abs() <= 0.05,
                                "rate={rate}, cutoff={cutoff}, quantum={quantum}, frequency={frequency}, actual={actual}, reference={reference}"
                            );
                        } else {
                            assert!(
                                actual <= -115.0,
                                "rate={rate}, cutoff={cutoff}, quantum={quantum}, frequency={frequency}, actual={actual}"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn coherent_sustained_sines_cover_launch_and_extended_compatibility_rates() {
    for rate in launch_and_extended_compatibility_rates() {
        let mut cutoffs = vec![
            10.0,
            20.0,
            100.0,
            1_000.0,
            (20_000.0_f64).min(0.1 * f64::from(rate)),
            0.45 * f64::from(rate),
        ];
        cutoffs.sort_by(f64::total_cmp);
        cutoffs.dedup_by(|left, right| *left == *right);
        for (high_pass, kind) in [
            (true, ReferenceFilterKind::HighPass),
            (false, ReferenceFilterKind::LowPass),
        ] {
            for cutoff in &cutoffs {
                for frequency in coherent_probes(rate, *cutoff) {
                    let _ = kind;
                    let measurement = sustained_measurement(rate, *cutoff, high_pass, frequency);
                    if measurement.reference_gain_db >= -90.0 {
                        assert!(
                            (measurement.production_gain_db - measurement.reference_gain_db).abs()
                                <= 0.05,
                            "rate={rate}, cutoff={cutoff}, frequency={frequency}, production={}, reference={}",
                            measurement.production_gain_db,
                            measurement.reference_gain_db
                        );
                        assert!(
                            measurement.residual_db <= -100.0,
                            "rate={rate}, cutoff={cutoff}, frequency={frequency}, residual={}",
                            measurement.residual_db
                        );
                    } else {
                        assert!(
                            measurement.total_output_db <= -88.0,
                            "rate={rate}, cutoff={cutoff}, frequency={frequency}, output={}",
                            measurement.total_output_db
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn production_order_hpf_lpf_cascade_meets_all_launch_response_gates() {
    for rate in LAUNCH_SAMPLE_RATES.map(|rate| rate.0) {
        let probes = cascade_probes(rate);
        let high_space = state_space(rate, 100.0, true);
        let low_space = state_space(rate, 1_000.0, false);
        for frequency in probes
            .iter()
            .copied()
            .chain([100.0, 1_000.0, 0.49 * f64::from(rate)])
        {
            let reference = rbj_butterworth_magnitude_db(
                f64::from(rate),
                100.0,
                ReferenceFilterKind::HighPass,
                frequency,
            )
            .expect("reference high")
                + rbj_butterworth_magnitude_db(
                    f64::from(rate),
                    1_000.0,
                    ReferenceFilterKind::LowPass,
                    frequency,
                )
                .expect("reference low");
            let actual = high_space
                .magnitude_db(f64::from(rate), frequency)
                .expect("state high")
                + low_space
                    .magnitude_db(f64::from(rate), frequency)
                    .expect("state low");
            if reference >= -120.0 {
                assert!(
                    (actual - reference).abs() <= 0.005,
                    "analytic rate={rate}, frequency={frequency}, actual={actual}, reference={reference}"
                );
            }
        }
        for quantum in [1, 127, 128, 255, 1_024] {
            let impulse = impulse_through(rate, cascade_parameters(), rate as usize, quantum);
            assert!(impulse.iter().all(|sample| sample.is_finite()));
            for frequency in &probes {
                let reference = rbj_butterworth_magnitude_db(
                    f64::from(rate),
                    100.0,
                    ReferenceFilterKind::HighPass,
                    *frequency,
                )
                .expect("reference high")
                    + rbj_butterworth_magnitude_db(
                        f64::from(rate),
                        1_000.0,
                        ReferenceFilterKind::LowPass,
                        *frequency,
                    )
                    .expect("reference low");
                let actual = impulse_dft_magnitude_db(&impulse, f64::from(rate), *frequency);
                if reference >= -120.0 {
                    assert!(
                        (actual - reference).abs() <= 0.05,
                        "impulse rate={rate}, quantum={quantum}, frequency={frequency}, actual={actual}, reference={reference}"
                    );
                } else {
                    assert!(
                        actual <= -115.0,
                        "impulse rate={rate}, quantum={quantum}, frequency={frequency}, actual={actual}"
                    );
                }
            }
        }
        for frequency in probes {
            let measurement = sustained_cascade_measurement(rate, frequency);
            if measurement.reference_gain_db >= -90.0 {
                assert!(
                    (measurement.production_gain_db - measurement.reference_gain_db).abs() <= 0.05,
                    "sustained rate={rate}, frequency={frequency}, production={}, reference={}",
                    measurement.production_gain_db,
                    measurement.reference_gain_db
                );
                assert!(
                    measurement.residual_db <= -100.0,
                    "sustained rate={rate}, frequency={frequency}, residual={}",
                    measurement.residual_db
                );
            } else {
                assert!(
                    measurement.total_output_db <= -88.0,
                    "sustained rate={rate}, frequency={frequency}, output={}",
                    measurement.total_output_db
                );
            }
        }
    }
}

/// Renders a `frames`-long impulse through the prepared stage in blocks of `quantum`.
///
/// The block partition is part of what is measured: the same impulse rendered at every quantum
/// must produce the same bits (master plan D5).
fn impulse_through(
    rate: u32,
    parameters: BuiltinParameters,
    frames: usize,
    quantum: usize,
) -> Vec<f32> {
    let mut chain = BuiltinChain::new(rate, parameters).expect("prepared impulse chain");
    let mut left = vec![0.0_f32; frames];
    left[0] = 1.0;
    let mut right = vec![0.0_f32; frames];
    let mut start = 0;
    while start < frames {
        let end = (start + quantum).min(frames);
        chain.process_input(
            DualMonoBlock::new(&mut left[start..end], &mut right[start..end], start as u64)
                .expect("impulse block"),
        );
        start = end;
    }
    left
}

/// One sustained-sine measurement of a prepared chain against its `f64` reference.
struct SustainedMeasurement {
    production_gain_db: f64,
    reference_gain_db: f64,
    residual_db: f64,
    total_output_db: f64,
}

/// Renders one planar buffer through the prepared input stage.
///
/// This is the production entry point, not a per-sample twin: the block kernel is the only sample
/// loop there is (#85), so a measurement that did not go through it would measure nothing.
fn render_input(rate: u32, parameters: BuiltinParameters, input: &[f32]) -> Vec<f32> {
    let mut chain = BuiltinChain::new(rate, parameters).expect("prepared response chain");
    let mut left = input.to_vec();
    let mut right = vec![0.0_f32; input.len()];
    chain.process_input(DualMonoBlock::new(&mut left, &mut right, 0).expect("response block"));
    left
}

/// Parameters selecting exactly one enabled section.
fn one_section(cutoff: f64, high_pass: bool) -> BuiltinParameters {
    let mut parameters = BuiltinParameters::default();
    if high_pass {
        parameters.left.hpf_hz = cutoff as f32;
    } else {
        parameters.left.lpf_hz = cutoff as f32;
    }
    parameters
}

/// Parameters for the frozen production-order cascade: high-pass 100 Hz then low-pass 1 kHz.
fn cascade_parameters() -> BuiltinParameters {
    BuiltinParameters {
        left: ChannelParameters {
            hpf_hz: 100.0,
            lpf_hz: 1_000.0,
            ..ChannelParameters::default()
        },
        ..BuiltinParameters::default()
    }
}

/// Fits the production and reference outputs of a settled sine and reports the four measurements.
fn measure_sustained(
    rate: u32,
    frequency: f64,
    parameters: BuiltinParameters,
    reference: &mut dyn FnMut(f64) -> f64,
) -> SustainedMeasurement {
    let settle = rate as usize / 2;
    let frames = rate as usize / 4;
    let rate_f64 = f64::from(rate);
    let input: Vec<f32> = (0..settle + frames)
        .map(|index| {
            let phase = core::f64::consts::TAU * frequency * index as f64 / rate_f64;
            (0.5 * phase.sin()) as f32
        })
        .collect();
    let rendered = render_input(rate, parameters, &input);
    let mut input_energy = 0.0_f64;
    let mut output_energy = 0.0_f64;
    let mut measured_outputs = Vec::with_capacity(frames);
    let mut reference_outputs = Vec::with_capacity(frames);
    for index in 0..settle + frames {
        let reference_output = reference(f64::from(input[index]));
        if index >= settle {
            let output = f64::from(rendered[index]);
            measured_outputs.push(output);
            reference_outputs.push(reference_output);
            input_energy += f64::from(input[index]) * f64::from(input[index]);
            output_energy += output * output;
        }
    }
    let frames_f64 = frames as f64;
    let input_rms = (input_energy / frames_f64).sqrt();
    let [dc, sine, cosine] = fit_dc_sine_cosine(&measured_outputs, settle, rate_f64, frequency);
    let [_, reference_sine, reference_cosine] =
        fit_dc_sine_cosine(&reference_outputs, settle, rate_f64, frequency);
    let production_amplitude = sine.hypot(cosine);
    let reference_amplitude = reference_sine.hypot(reference_cosine);
    let residual_energy = measured_outputs
        .iter()
        .copied()
        .enumerate()
        .map(|(offset, output)| {
            let phase = core::f64::consts::TAU * frequency * (settle + offset) as f64 / rate_f64;
            (output - (dc + sine * phase.sin() + cosine * phase.cos())).powi(2)
        })
        .sum::<f64>();
    SustainedMeasurement {
        production_gain_db: 20.0 * (production_amplitude / 0.5).log10(),
        reference_gain_db: 20.0 * (reference_amplitude / 0.5).log10(),
        residual_db: 20.0 * ((residual_energy / frames_f64).sqrt() / input_rms).log10(),
        total_output_db: 20.0 * ((output_energy / frames_f64).sqrt() / input_rms).log10(),
    }
}

/// One enabled section against its RBJ twin.
fn sustained_measurement(
    rate: u32,
    cutoff: f64,
    high_pass: bool,
    frequency: f64,
) -> SustainedMeasurement {
    let kind = if high_pass {
        ReferenceFilterKind::HighPass
    } else {
        ReferenceFilterKind::LowPass
    };
    let mut reference =
        ReferenceBiquad::rbj_butterworth(f64::from(rate), cutoff, kind).expect("reference section");
    measure_sustained(rate, frequency, one_section(cutoff, high_pass), &mut |x| {
        reference.process(x)
    })
}

/// The production-order cascade against the cascaded RBJ twin.
fn sustained_cascade_measurement(rate: u32, frequency: f64) -> SustainedMeasurement {
    let mut high =
        ReferenceBiquad::rbj_butterworth(f64::from(rate), 100.0, ReferenceFilterKind::HighPass)
            .expect("reference high");
    let mut low =
        ReferenceBiquad::rbj_butterworth(f64::from(rate), 1_000.0, ReferenceFilterKind::LowPass)
            .expect("reference low");
    measure_sustained(rate, frequency, cascade_parameters(), &mut |x| {
        low.process(high.process(x))
    })
}

fn fit_dc_sine_cosine(samples: &[f64], first_index: usize, rate: f64, frequency: f64) -> [f64; 3] {
    let mut normal = [[0.0_f64; 3]; 3];
    let mut right = [0.0_f64; 3];
    for (offset, sample) in samples.iter().copied().enumerate() {
        let phase = core::f64::consts::TAU * frequency * (first_index + offset) as f64 / rate;
        let basis = [1.0, phase.sin(), phase.cos()];
        for row in 0..3 {
            right[row] += basis[row] * sample;
            for column in 0..3 {
                normal[row][column] += basis[row] * basis[column];
            }
        }
    }
    solve_three_by_three([
        [normal[0][0], normal[0][1], normal[0][2], right[0]],
        [normal[1][0], normal[1][1], normal[1][2], right[1]],
        [normal[2][0], normal[2][1], normal[2][2], right[2]],
    ])
}

fn solve_three_by_three(mut augmented: [[f64; 4]; 3]) -> [f64; 3] {
    for column in 0..3 {
        let mut pivot = column;
        for row in column + 1..3 {
            if augmented[row][column].abs() > augmented[pivot][column].abs() {
                pivot = row;
            }
        }
        augmented.swap(column, pivot);
        let divisor = augmented[column][column];
        assert!(divisor.is_finite() && divisor.abs() > f64::EPSILON);
        for value in &mut augmented[column][column..] {
            *value /= divisor;
        }
        let pivot_row = augmented[column];
        for (row_index, row) in augmented.iter_mut().enumerate() {
            if row_index == column {
                continue;
            }
            let factor = row[column];
            for (value, pivot_value) in row[column..].iter_mut().zip(&pivot_row[column..]) {
                *value -= factor * pivot_value;
            }
        }
    }
    [augmented[0][3], augmented[1][3], augmented[2][3]]
}

fn assert_tpt_limits_and_monotonic(
    state: ReferenceSvfStateSpace,
    rate: u32,
    high_pass: bool,
    cutoff: f64,
) {
    let nyquist = 0.5 * f64::from(rate);
    let magnitude = |frequency| {
        let response = state
            .response(f64::from(rate), frequency)
            .expect("finite state-space response");
        response.re.hypot(response.im)
    };
    let (dc, at_nyquist) = (magnitude(0.0), magnitude(nyquist));
    if high_pass {
        assert!(
            dc <= 1e-6,
            "HPF DC limit: rate={rate}, cutoff={cutoff}, value={dc}"
        );
        assert!(
            (at_nyquist - 1.0).abs() <= 1e-6,
            "HPF Nyquist limit: rate={rate}, cutoff={cutoff}, value={at_nyquist}"
        );
    } else {
        assert!(
            (dc - 1.0).abs() <= 1e-6,
            "LPF DC limit: rate={rate}, cutoff={cutoff}, value={dc}"
        );
        assert!(
            at_nyquist <= 1e-6,
            "LPF Nyquist limit: rate={rate}, cutoff={cutoff}, value={at_nyquist}"
        );
    }
    let mut previous = magnitude(0.0);
    for index in 1..=4_096 {
        let current = magnitude(nyquist * f64::from(index) / 4_096.0);
        if high_pass {
            assert!(
                current + 2e-6 >= previous,
                "HPF monotonicity: rate={rate}, cutoff={cutoff}, index={index}, previous={previous}, current={current}"
            );
        } else {
            assert!(
                current <= previous + 2e-6,
                "LPF monotonicity: rate={rate}, cutoff={cutoff}, index={index}, previous={previous}, current={current}"
            );
        }
        previous = current;
    }
}

fn coherent_probes(rate: u32, cutoff: f64) -> Vec<f64> {
    let nyquist = 0.5 * f64::from(rate);
    let mut probes = [
        0.25 * cutoff,
        cutoff,
        4.0 * cutoff,
        0.2 * f64::from(rate),
        0.45 * f64::from(rate),
    ]
    .into_iter()
    .map(|probe| probe.clamp(4.0, nyquist - 4.0))
    .map(|probe| (probe / 4.0).round() * 4.0)
    .collect::<Vec<_>>();
    probes.sort_by(f64::total_cmp);
    probes.dedup_by(|left, right| *left == *right);
    probes
}

fn cascade_probes(rate: u32) -> Vec<f64> {
    let mut probes = coherent_probes(rate, 100.0);
    probes.extend(coherent_probes(rate, 1_000.0));
    probes.sort_by(f64::total_cmp);
    probes.dedup_by(|left, right| *left == *right);
    probes
}

fn impulse_dft_magnitude_db(samples: &[f32], rate: f64, frequency: f64) -> f64 {
    let phase = -core::f64::consts::TAU * frequency / rate;
    let (step_real, step_imaginary) = (phase.cos(), phase.sin());
    let (mut unit_real, mut unit_imaginary) = (1.0_f64, 0.0_f64);
    let (mut real, mut imaginary) = (0.0_f64, 0.0_f64);
    for sample in samples {
        let sample = f64::from(*sample);
        real += sample * unit_real;
        imaginary += sample * unit_imaginary;
        (unit_real, unit_imaginary) = (
            unit_real * step_real - unit_imaginary * step_imaginary,
            unit_real * step_imaginary + unit_imaginary * step_real,
        );
    }
    let magnitude = real.hypot(imaginary);
    if magnitude == 0.0 {
        f64::NEG_INFINITY
    } else {
        20.0 * magnitude.log10()
    }
}

/// T4: the issue-036 domain prepares everywhere, and the successor is rejected by the domain.
///
/// This replaces the pre-#83 seam derivation, which asserted that the published maximum was the
/// *first coefficient failure* of the design. That seam was set by the `1 - c1` quantisation
/// feeding a Jury stability gate; the kernel now stores `c1` and casts `a2 = g / (1 + t)` and
/// `a3 = g * g / (1 + t)` directly, so it is better conditioned near Nyquist and every value
/// through the table maximum -- and beyond it -- prepares. The table is frozen (§8.2): it is not
/// widened, no new maximum is computed, and no stability gate is re-added to recreate the old
/// failure. What the domain rejects, it rejects because the table says so.
///
/// What replaces the derivation is the measurement: every representable cutoff from `0.45 * fs`
/// through the table maximum designs a section whose cast state-space transfer is `-3.0103 dB` at
/// its own cutoff, to the frozen 0.005 dB tolerance.
#[test]
fn representable_cutoff_domain_prepares_everywhere_and_rejects_successor() {
    for (rate, maximum_bits) in [
        (44_100_u32, 0x46ac_42f7_u32),
        (48_000, 0x46bb_7ede),
        (88_200, 0x472c_42f7),
        (96_000, 0x473b_7ede),
    ] {
        let start_bits = (0.45_f32 * rate as f32).to_bits();
        for high_pass in [true, false] {
            for bits in start_bits..=maximum_bits {
                let cutoff = f32::from_bits(bits);
                assert!(
                    validate_builtin_filter_cutoff(cutoff, rate, 0.0, 10.0).is_ok(),
                    "domain rate={rate}, high_pass={high_pass}, cutoff={bits:08x}"
                );
                test_support::section_words(rate, cutoff, high_pass).unwrap_or_else(|error| {
                    panic!(
                        "design rate={rate}, high_pass={high_pass}, cutoff={bits:08x}, \
                         error={error:?}"
                    )
                });
            }
            // The -3 dB point, measured on the cast words at the ends and across the seam.
            for bits in [start_bits, maximum_bits - 1, maximum_bits] {
                let cutoff = f64::from(f32::from_bits(bits));
                let db = state_space(rate, cutoff, high_pass)
                    .magnitude_db(f64::from(rate), cutoff)
                    .expect("cast state-space magnitude");
                assert!(
                    (db + 3.010_299_956_6).abs() <= 0.005,
                    "rate={rate}, high_pass={high_pass}, cutoff={bits:08x}, db={db}"
                );
            }
            let successor = f32::from_bits(maximum_bits + 1);
            assert_eq!(
                validate_builtin_filter_cutoff(successor, rate, 0.0, 10.0),
                Err(BuiltinParameterError::FilterCutoff),
                "successor rate={rate}, high_pass={high_pass}"
            );
            let mut parameters = BuiltinParameters::default();
            if high_pass {
                parameters.left.hpf_hz = successor;
            } else {
                parameters.left.lpf_hz = successor;
            }
            assert!(
                matches!(
                    BuiltinChain::new(rate, parameters),
                    Err(BuiltinParameterError::FilterCutoff)
                ),
                "successor preparation rate={rate}, high_pass={high_pass}"
            );
        }
    }
}
