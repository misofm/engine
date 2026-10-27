//! Issue #988: the detector screen's proof domain and exact observable identity.
use super::*;

#[test]
fn screen_bound_covers_the_attaining_phase_patterns() {
    let u = 2.0_f64.powi(-24);
    let gamma = 13.0 * u / (1.0 - 13.0 * u);
    let max_norm = (0..4)
        .map(|phase| {
            ANNEX2_FIR
                .iter()
                .map(|row| f64::from(row[phase]).abs())
                .sum::<f64>()
        })
        .fold(0.0_f64, f64::max);
    let minimum = f64::from(limit_coefficient(-24.0));
    let relative = f64::from(SCREEN_FACTOR) * max_norm * (1.0 + gamma) * (1.0 + u)
        + 64.0 * f64::from(f32::MIN_POSITIVE) / minimum;
    assert!(relative < 0.9915, "rounded phase bound {relative}");
    let coefficients = LimiterCoef::<f32>::new(LinkMode::DualMono, false);
    for ceiling in 0..=768 {
        let limit = limit_coefficient(-24.0 + ceiling as f32 / 32.0);
        let threshold = SCREEN_FACTOR * limit;
        let below = f32::from_bits(threshold.to_bits() - 1);
        for phase in 0..4 {
            let mut history = History::zero();
            for tap in (0..HISTORY_WORDS).rev() {
                history.shift(below.copysign(ANNEX2_FIR[tap][phase]));
            }
            assert!(taps_below(&history, &[], threshold));
            let peak = annex2_phases(&history, &coefficients.fir)
                .into_iter()
                .map(f32::abs)
                .fold(history.t6.abs(), f32::max);
            assert!(
                peak < limit,
                "ceiling {ceiling} phase {phase}: {peak} >= {limit}"
            );
            let required = if peak > limit { limit / peak } else { 1.0 };
            assert_eq!(required.to_bits(), 1.0_f32.to_bits());
        }
    }
}

struct ScreenPair<L: Lane> {
    candidate: LimiterCore<L>,
    oracle: LimiterCore<L>,
}

impl<L: Lane> ScreenPair<L> {
    fn new(rate: u32, link: LinkMode, bypass: bool, asymmetric: bool) -> Self {
        let tracks: Vec<_> = (0..L::WIDTH)
            .map(|lane| {
                let ceiling = -0.5 - lane as f32 * 0.5;
                linked_values(
                    [ceiling, 60.0, 5.0],
                    [if asymmetric { ceiling - 12.0 } else { ceiling }, 60.0, 5.0],
                )
            })
            .collect();
        Self {
            candidate: linked_core(&tracks, link, bypass, rate),
            oracle: linked_core(&tracks, link, bypass, rate),
        }
    }

    fn block(&mut self, left: &[f32], right: &[f32], expected: Option<bool>, label: &str) {
        let frames = left.len() / L::WIDTH;
        let (mut a, mut b) = (left.to_vec(), right.to_vec());
        let (mut x, mut y) = (left.to_vec(), right.to_vec());
        let before = SCREENED_ENGAGEMENTS.with(Cell::get);
        let reports = self.candidate.process_block(&mut a, &mut b, frames);
        let engaged = SCREENED_ENGAGEMENTS.with(Cell::get) != before;
        if let Some(expected) = expected {
            assert_eq!(engaged, expected, "{label}: admission");
        }
        FORCE_UNSCREENED.with(|force| force.set(true));
        let oracle_reports = self.oracle.process_block(&mut x, &mut y, frames);
        FORCE_UNSCREENED.with(|force| force.set(false));
        assert_same_words(&a, &x, label);
        assert_same_words(&b, &y, label);
        assert_eq!(reports, oracle_reports, "{label}: boundary reports");
        assert_eq!(
            state_bits(&self.candidate),
            state_bits(&self.oracle),
            "{label}: retained state"
        );
        for lane in 0..L::WIDTH {
            assert_eq!(
                core_snapshot(&self.candidate, lane),
                core_snapshot(&self.oracle, lane),
                "{label}: lane {lane} payload"
            );
        }
        assert_eq!(
            self.candidate.gain_linked, self.oracle.gain_linked,
            "{label}: linked claim"
        );
        assert_eq!(
            self.candidate.silent_fixed_point, self.oracle.silent_fixed_point,
            "{label}: silent claim"
        );
    }
}

fn screen_identity<L: Lane>() {
    let lengths = [0, 1, 5, 11, 12, 13, 31, 32, 33, 64, 127, 128, 1024];
    let mut draw = Draw(0x988_2026_1027);
    for rate in [44_100, 48_000, 88_200, 96_000] {
        for link in [LinkMode::Maximum, LinkMode::DualMono] {
            for bypass in [false, true] {
                for asymmetric in [false, true] {
                    // Retained histories cross all short/long block boundaries.
                    for factor in [0.45, 0.48, 0.495] {
                        let mut pair = ScreenPair::<L>::new(rate, link, bypass, asymmetric);
                        for frames in lengths {
                            let mut left = vec![0.0; frames * L::WIDTH];
                            let mut right = left.clone();
                            for (channel, plane) in [&mut left, &mut right].into_iter().enumerate()
                            {
                                for (word, sample) in plane.iter_mut().enumerate() {
                                    let lane = word % L::WIDTH;
                                    let limit = pair.candidate.left.limit[lane]
                                        .current
                                        .min(pair.candidate.right.limit[lane].current);
                                    *sample = draw.unit()
                                        * limit
                                        * factor
                                        * if channel == 0 { 1.0 } else { -1.0 };
                                }
                            }
                            pair.block(&left, &right, if frames == 0 { Some(false) } else if factor < 0.49 { Some(true) } else { None },
                                &format!("W{} {rate} {link:?} {bypass} {asymmetric} frames{frames} factor{factor}", L::WIDTH));
                        }
                    }
                }
            }
        }
    }
    // Refusal and boundary witnesses are fresh so no older tap can hide the reason.
    for channel in 0..2 {
        for lane in 0..L::WIDTH {
            for special in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 1.5] {
                let mut pair = ScreenPair::<L>::new(48_000, LinkMode::Maximum, false, false);
                let mut planes = [vec![0.02; 33 * L::WIDTH], vec![0.03; 33 * L::WIDTH]];
                planes[channel][31 * L::WIDTH + lane] = special;
                pair.block(
                    &planes[0],
                    &planes[1],
                    Some(false),
                    "one input tap rejects whole bank",
                );
                pair.block(
                    &vec![0.02; 5 * L::WIDTH],
                    &vec![0.03; 5 * L::WIDTH],
                    Some(false),
                    "loud or nonfinite history alone rejects",
                );
            }
            for equality in [false, true] {
                let mut pair = ScreenPair::<L>::new(48_000, LinkMode::Maximum, false, true);
                let threshold = SCREEN_FACTOR * pair.candidate.right.limit[lane].current;
                let tap = if equality {
                    threshold
                } else {
                    f32::from_bits(threshold.to_bits() - 1)
                };
                let mut planes = [vec![0.02; 33 * L::WIDTH], vec![0.03; 33 * L::WIDTH]];
                planes[channel][31 * L::WIDTH + lane] = tap;
                pair.block(
                    &planes[0],
                    &planes[1],
                    Some(!equality),
                    "ordered threshold with asymmetric ceilings",
                );
            }
        }
    }
    for phase in 0..4 {
        let mut pair = ScreenPair::<L>::new(48_000, LinkMode::Maximum, false, false);
        let mut plane = vec![0.0; 12 * L::WIDTH];
        for lane in 0..L::WIDTH {
            let threshold = SCREEN_FACTOR * pair.candidate.left.limit[lane].current;
            let below = f32::from_bits(threshold.to_bits() - 1);
            for tap in 0..12 {
                plane[(11 - tap) * L::WIDTH + lane] = below.copysign(ANNEX2_FIR[tap][phase]);
            }
        }
        pair.block(
            &plane,
            &plane,
            Some(true),
            "attaining sign pattern renders identically",
        );
    }
    for special in [
        -0.0,
        f32::from_bits(1),
        -f32::MIN_POSITIVE,
        2.0_f32.powi(-24),
    ] {
        let mut pair = ScreenPair::<L>::new(48_000, LinkMode::DualMono, true, false);
        for frames in lengths {
            pair.block(
                &vec![special; frames * L::WIDTH],
                &vec![-special; frames * L::WIDTH],
                Some(frames != 0),
                "signed-zero/subnormal history bits",
            );
        }
    }
    // A stale history tap is a separate admission leg, even when never emitted this block.
    let mut pair = ScreenPair::<L>::new(48_000, LinkMode::Maximum, false, false);
    pair.candidate.left.history[11 * L::WIDTH] = 1.5;
    pair.oracle.left.history[11 * L::WIDTH] = 1.5;
    pair.block(
        &vec![0.02; 33 * L::WIDTH],
        &vec![0.03; 33 * L::WIDTH],
        Some(false),
        "oldest history tap",
    );
    for release in [false, true] {
        let mut pair = ScreenPair::<L>::new(48_000, LinkMode::Maximum, false, false);
        for core in [&mut pair.candidate, &mut pair.oracle] {
            if release {
                core.right.release[0].set_target(release_coefficient(200.0, 48_000), RAMP_UPDATES);
            } else {
                core.left.limit[0].set_target(limit_coefficient(-12.0), RAMP_UPDATES);
            }
        }
        pair.block(
            &vec![0.02; 33 * L::WIDTH],
            &vec![0.03; 33 * L::WIDTH],
            Some(false),
            "moving coefficient refuses",
        );
    }
    if L::WIDTH > 1 {
        let mut pair = ScreenPair::<L>::new(48_000, LinkMode::DualMono, false, false);
        for core in [&mut pair.candidate, &mut pair.oracle] {
            core.left.phase[1] = 1;
        }
        pair.block(
            &vec![0.02; 33 * L::WIDTH],
            &vec![0.03; 33 * L::WIDTH],
            Some(false),
            "ragged phase refuses",
        );
    }
    let mut core = ScreenPair::<L>::new(48_000, LinkMode::Maximum, false, false).candidate;
    let before = SCREENED_ENGAGEMENTS.with(Cell::get);
    core.process_block_mono(&mut vec![0.02; 33 * L::WIDTH], 33);
    assert_eq!(
        SCREENED_ENGAGEMENTS.with(Cell::get),
        before,
        "collapsed body never screens"
    );
}

/// The new path's full output/state differential catches a lost short history, channel-limit
/// mixup, skipped gain-ring write, or admission during a ramp even if delayed output looks quiet.
#[test]
fn screen_preserves_every_word_and_refuses_unproven_blocks() {
    lane::each_lane!(|L| screen_identity::<L>());
}
