//! The gain law's two independent-oracle gates (#90 evals E4 and E5).
//!
//! Both drive the crate through its public factory only and measure with
//! `miso-engine-dsp-reference`, which owns its own `f64` Annex-2 table, its own rings and a
//! brute-force minimum and box sum. They live outside `src` so that production stays lean and so
//! that the `f64` measurement arithmetic — the one place a `powf` and a `log10` are the right
//! tool — is nowhere near a render path (`scripts/check-math-policy.sh`).

use miso_engine_dsp_reference::{
    ReferenceTruePeakLimiter, ReferenceTruePeakLimiterParameters, reference_true_peak_estimate,
};
use miso_engine_effect_contract::{
    EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory,
    ParameterChannel, PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect,
    PreparedPortsV1, PreparedSidechainPort, ProcessReport,
};
use miso_engine_true_peak_limiter::{
    TRUE_PEAK_LIMITER_DESCRIPTOR_V1, TRUE_PEAK_LIMITER_PARAMETERS_V1, TruePeakLimiterFactory,
};

/// Deterministic SplitMix64 noise, so a corpus is a seed and never a file.
struct Noise(u64);

impl Noise {
    fn next(&mut self) -> f32 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut mixed = self.0;
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        mixed ^= mixed >> 31;
        ((mixed >> 40) as f32 * (1.0 / 16_777_216.0)) * 2.0 - 1.0
    }
}

fn values_with(ceiling: f32, release: f32, lookahead: f32) -> [InitialParameterValue; 6] {
    let mut values: [InitialParameterValue; 6] =
        core::array::from_fn(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: TRUE_PEAK_LIMITER_PARAMETERS_V1[index / 2].default_value,
        });
    for (index, value) in [ceiling, ceiling, release, release, lookahead, lookahead]
        .into_iter()
        .enumerate()
    {
        values[index].value = value;
    }
    values
}

fn request_at_rate(values: &[InitialParameterValue], sample_rate: u32) -> PrepareEffectRequest<'_> {
    let quality = TRUE_PEAK_LIMITER_DESCRIPTOR_V1
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == sample_rate)
        .expect("launch rate");
    PrepareEffectRequest {
        sample_rate,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
    }
}

fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    request_at_rate(values, 48_000)
}

fn render(
    effect: &mut dyn PreparedNativeEffect,
    left: &mut [f32],
    right: &mut [f32],
    block: usize,
) -> ProcessReport {
    let quantum = effect.metadata().quantum;
    let mut report = ProcessReport::default();
    for (index, (left, right)) in left
        .chunks_mut(block)
        .zip(right.chunks_mut(block))
        .enumerate()
    {
        let next = effect.process(
            EffectProcessBlock::new(left, right, None, (index * block) as u64, &[], quantum)
                .expect("block"),
        );
        report.invalid_spans = report.invalid_spans.saturating_add(next.invalid_spans);
    }
    report
}

/// E4 (hard gate): the true-peak estimate of the output never exceeds the ceiling.
#[test]
fn output_true_peak_never_exceeds_the_ceiling() {
    let frames = 2048_usize;
    let mut worst = f64::NEG_INFINITY;
    let mut worst_case = String::new();
    for rate in [44_100_u32, 48_000, 88_200, 96_000] {
        for link in [LinkMode::DualMono, LinkMode::Maximum] {
            for ceiling in [-1.0_f32, -6.0, -12.0] {
                for lookahead in [0.0_f32, 1.0, 5.0, 10.0] {
                    for release in [10.0_f32, 2000.0] {
                        for corpus in 0..5 {
                            let (mut left, mut right) = corpus_signal(corpus, frames, rate);
                            let values = values_with(ceiling, release, lookahead);
                            let mut preparation = request_at_rate(&values, rate);
                            preparation.link_mode = link;
                            let mut effect = TruePeakLimiterFactory
                                .prepare(preparation)
                                .expect("prepare");
                            render(effect.as_mut(), &mut left, &mut right, 128);
                            let ceiling_gain = f64::from(10.0_f32).powf(f64::from(ceiling) / 20.0);
                            for (side, channel) in [("left", &left), ("right", &right)] {
                                let measured: Vec<f64> =
                                    channel.iter().map(|x| f64::from(*x)).collect();
                                let estimate =
                                    reference_true_peak_estimate(&measured).expect("finite output");
                                let excess_db = 20.0 * (estimate / ceiling_gain).log10();
                                if excess_db > worst {
                                    worst = excess_db;
                                    worst_case = format!(
                                        "{rate} {link:?} ceiling {ceiling} lookahead \
                                         {lookahead} release {release} corpus {corpus} {side}"
                                    );
                                }
                                assert!(
                                    estimate <= ceiling_gain,
                                    "{rate} {link:?} ceiling {ceiling} lookahead {lookahead} \
                                     release {release} corpus {corpus} {side}: estimate \
                                     {estimate} exceeds {ceiling_gain} by {excess_db} dB"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    // Descriptive, and the number the #49 guard decision needs: the whole matrix stays this
    // far under the user ceiling with the frozen 1 dB internal guard and `W_MIN = 32`.
    println!("worst true-peak margin {worst:+.4} dB at {worst_case}");
    assert!(worst < 0.0, "worst margin {worst} dB at {worst_case}");
}

/// The seeded corpora of the ceiling gate. Never a file, always a seed.
fn corpus_signal(index: usize, frames: usize, rate: u32) -> (Vec<f32>, Vec<f32>) {
    let mut left = vec![0.0_f32; frames];
    let mut right = vec![0.0_f32; frames];
    match index {
        0..=2 => {
            // Bipolar noise at +6 dBFS, three seeds.
            let mut noise = Noise(0xC0DE_0000 + index as u64);
            for frame in 0..frames {
                left[frame] = noise.next() * 2.0;
                right[frame] = noise.next() * 2.0;
            }
        }
        3 => {
            // A near-Nyquist sine at +3 dB: the classic inter-sample overshoot generator.
            let step = core::f64::consts::TAU * 0.49;
            for frame in 0..frames {
                let phase = step * frame as f64;
                left[frame] = (phase.sin() * 1.4125) as f32;
                right[frame] = ((phase + 0.37).sin() * 1.4125) as f32;
            }
        }
        _ => {
            // An impulse train at the highest rate the ring can carry.
            let period = (rate / 1000).max(1) as usize;
            for frame in 0..frames {
                if frame.is_multiple_of(period) {
                    left[frame] = if (frame / period).is_multiple_of(2) {
                        4.0
                    } else {
                        -4.0
                    };
                    right[frame] = -left[frame];
                }
            }
        }
    }
    (left, right)
}

/// E5: the production law tracks the independent `f64` oracle.
#[test]
fn production_tracks_the_f64_oracle() {
    let frames = 4096_usize;
    for lookahead in [0.0_f32, 5.0, 10.0] {
        for ceiling in [-1.0_f32, -6.0] {
            let values = values_with(ceiling, 100.0, lookahead);
            let mut effect = TruePeakLimiterFactory
                .prepare(request(&values))
                .expect("prepare");
            let mut noise = Noise(0x5150 + lookahead as u64);
            let source: Vec<f32> = (0..frames).map(|_| noise.next() * 3.0).collect();
            let mut left = source.clone();
            let mut right = source.clone();
            render(effect.as_mut(), &mut left, &mut right, 128);

            let mut oracle = ReferenceTruePeakLimiter::new(
                48_000.0,
                ReferenceTruePeakLimiterParameters {
                    ceiling_db: f64::from(ceiling),
                    release_ms: 100.0,
                    lookahead_ms: f64::from(lookahead),
                },
            )
            .expect("oracle");
            let mut worst = 0.0_f64;
            for frame in 0..frames {
                let x = f64::from(source[frame]);
                let peak = oracle.detect(x);
                let expected = oracle.apply(x, peak);
                let error = (f64::from(left[frame]) - expected).abs();
                if error > worst {
                    worst = error;
                }
            }
            assert!(
                worst <= 1.0e-4,
                "lookahead {lookahead} ceiling {ceiling}: worst deviation {worst}"
            );
        }
    }
}
