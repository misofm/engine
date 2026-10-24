#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately.
//! The dynamic EQ against the independent `f64` references: the static bell response (#87's
//! oracle), the dynamic band (`dsp_reference::ReferenceDynamicEqBand`) downward and upward, and
//! the external detector. The detector, link and gain law of the shared band step are held once in
//! `effect-runtime`'s `tests/dynamic_band.rs`.

mod support;

use dsp_reference::{
    ReferenceDynamicBandLaw, ReferenceDynamicEqBand, ReferenceParametricEqCoefficients,
    ReferenceParametricEqKind,
};
use effect_contract::{EffectProcessBlock, LinkMode, ObservationSample};
use support::{
    ATTACK, DETECTOR, DIRECTION, FREQUENCY, GAIN, KNEE, Q, RANGE, RATE, RATIO, RELEASE, THRESHOLD,
    Values, defaults, only, prepare, prepare_connected, process, set_band, tone,
};

/// `|H(f)|` in dB of an impulse response.
fn response_db(impulse: &[f32], frequency: f64) -> f64 {
    let (mut re, mut im) = (0.0_f64, 0.0_f64);
    for (n, &h) in impulse.iter().enumerate() {
        let phase = core::f64::consts::TAU * frequency * n as f64 / f64::from(RATE);
        re += f64::from(h) * phase.cos();
        im -= f64::from(h) * phase.sin();
    }
    20.0 * re.hypot(im).log10()
}

/// Static mode (range 0, so the change stays exactly 0 dB): four renders of four cascaded bands,
/// 16 (frequency, gain, Q) rows in all, each band the cookbook bell. The impulse response matches
/// the product of the four reference bells within 0.05 dB at each band's centre, half an octave
/// and an octave either side, and at 20 Hz and 20 kHz.
#[test]
fn static_mode_is_the_cookbook_bell_cascade() {
    const ROWS: [[(f32, f32, f32); 4]; 4] = [
        [
            (50.0, 6.0, 0.7),
            (400.0, -9.0, 2.0),
            (2_500.0, 12.0, 4.0),
            (12_000.0, -3.0, 1.0),
        ],
        [
            (80.0, -12.0, 1.5),
            (700.0, 3.0, 0.3),
            (5_000.0, -18.0, 8.0),
            (16_000.0, 9.0, 0.5),
        ],
        [
            (120.0, 24.0, 0.1),
            (1_000.0, -24.0, 1.0),
            (3_200.0, 1.5, 12.0),
            (9_000.0, 18.0, 2.5),
        ],
        [
            (30.0, -6.0, 3.0),
            (250.0, 15.0, 6.0),
            (1_800.0, -1.0, 16.0),
            (19_000.0, -15.0, 0.8),
        ],
    ];
    let frames = 1 << 16;
    for (row, bands) in ROWS.iter().enumerate() {
        let mut values = defaults();
        for (band, &(frequency, gain, q)) in bands.iter().enumerate() {
            set_band(&mut values, band, FREQUENCY, frequency);
            set_band(&mut values, band, GAIN, gain);
            set_band(&mut values, band, Q, q);
            set_band(&mut values, band, RANGE, 0.0);
        }
        let mut effect = prepare(&values, LinkMode::DualMono);
        let mut left = vec![0.0_f32; frames];
        left[0] = 1.0;
        let mut right = left.clone();
        for (block, (l, r)) in left.chunks_mut(128).zip(right.chunks_mut(128)).enumerate() {
            process(&mut effect, l, r, (block * 128) as u64, &[]);
        }
        let references: Vec<_> = bands
            .iter()
            .map(|&(frequency, gain, q)| {
                ReferenceParametricEqCoefficients::design(
                    ReferenceParametricEqKind::Bell,
                    f64::from(RATE),
                    f64::from(frequency),
                    f64::from(gain),
                    f64::from(q),
                    1.0,
                )
                .expect("reference bell")
            })
            .collect();
        let mut probes = vec![20.0, 20_000.0];
        for &(frequency, _, _) in bands {
            for octave in [-1.0, -0.5, 0.0, 0.5, 1.0] {
                let probe = f64::from(frequency) * 2.0_f64.powf(octave);
                if probe < 0.5 * f64::from(RATE) {
                    probes.push(probe);
                }
            }
        }
        for probe in probes {
            let expected: f64 = references
                .iter()
                .map(|r| 20.0 * r.magnitude_at_hz(probe).expect("magnitude").log10())
                .sum();
            let ours = response_db(&left, probe);
            assert!(
                (ours - expected).abs() <= 0.05,
                "row {row} at {probe} Hz: {ours} dB, the reference {expected} dB"
            );
        }
        assert_eq!(left, right, "row {row}: the channels agree");
    }
}

/// A scalar instance with only band 0 enabled at `values`, run one frame per block on both
/// channels, with the sidechain `side` when connected: the output and the band's change per frame.
fn render_band(
    values: &Values,
    connected: bool,
    main: impl Fn(usize) -> f32,
    side: impl Fn(usize) -> f32,
    frames: usize,
) -> (Vec<f32>, Vec<f32>) {
    let mut effect = if connected {
        prepare_connected(values)
    } else {
        prepare(values, LinkMode::DualMono)
    };
    let (mut out, mut change) = (Vec::with_capacity(frames), Vec::with_capacity(frames));
    for n in 0..frames {
        let (mut left, mut right) = ([main(n)], [main(n)]);
        let sidechain = [side(n)];
        let block = EffectProcessBlock::new(
            &mut left,
            &mut right,
            connected.then_some((&sidechain[..], &sidechain[..])),
            n as u64,
            &[],
            128,
        )
        .expect("block");
        effect.processor.process(block);
        let mut sample = ObservationSample::default();
        assert!(effect.processor.observe_resident(0, &mut sample));
        assert_eq!(
            left[0].to_bits(),
            right[0].to_bits(),
            "frame {n}: dual mono"
        );
        out.push(left[0]);
        change.push(sample.left);
    }
    (out, change)
}

/// The first frame at or after `from` where `|change|` reaches `fraction` of `plateau`'s
/// magnitude (`rising`) or falls to it.
fn crossing(change: &[f64], from: usize, plateau: f64, fraction: f64, rising: bool) -> usize {
    let level = fraction * plateau.abs();
    from + change[from..]
        .iter()
        .position(|c| {
            if rising {
                c.abs() >= level
            } else {
                c.abs() <= level
            }
        })
        .expect("a crossing")
}

/// 1 kHz at -6 dBFS from 50 ms to 300 ms, through a 1 kHz band, downward with a +3 dB static bell
/// and a 6 dB knee, and upward against the +6 dB range clamp. Per frame the change is within
/// 0.1 dB of the reference's and the output within `2e-4` of it (the input peaks at 0.5 under a
/// gain of up to +9 dB); the 90 % attack and 10 % release crossings are within the greater of one
/// sample and 2 % of the reference's. Then 1e6 frames of the same burst, repeated, stay finite.
#[test]
fn dynamic_mode_matches_the_reference_band() {
    const FRAMES: usize = 30_000;
    let burst = |n: usize| {
        if (2_400..14_400).contains(&n) {
            tone(1_000.0, n, 0.5)
        } else {
            0.0
        }
    };
    for (upward, gain, knee, threshold, range, plateau) in [
        (false, 3.0, 6.0, -20.0, 12.0, None),
        (true, 0.0, 0.0, -30.0, 6.0, Some(6.0)),
    ] {
        let mut values = only(0);
        set_band(&mut values, 0, FREQUENCY, 1_000.0);
        set_band(&mut values, 0, Q, 1.5);
        set_band(&mut values, 0, GAIN, gain);
        set_band(&mut values, 0, THRESHOLD, threshold);
        set_band(&mut values, 0, RATIO, 4.0);
        set_band(&mut values, 0, RANGE, range);
        set_band(&mut values, 0, KNEE, knee);
        set_band(&mut values, 0, ATTACK, 2.0);
        set_band(&mut values, 0, RELEASE, 60.0);
        set_band(&mut values, 0, DIRECTION, if upward { 2.0 } else { 1.0 });
        let (out, change) = render_band(&values, false, burst, |_| 0.0, FRAMES);
        let law = ReferenceDynamicBandLaw {
            threshold_db: f64::from(threshold),
            ratio: 4.0,
            knee_db: f64::from(knee),
            range_db: f64::from(range),
            attack_ms: 2.0,
            release_ms: 60.0,
            upward,
        };
        let mut reference =
            ReferenceDynamicEqBand::new(f64::from(RATE), 1_000.0, 1.5, f64::from(gain), law, false)
                .expect("design");
        let mut expected = Vec::with_capacity(FRAMES);
        for (n, (&y, &c)) in out.iter().zip(&change).enumerate() {
            let (y_ref, c_ref) = reference.process(f64::from(burst(n)), 0.0);
            expected.push(c_ref);
            assert!(
                (f64::from(c) - c_ref).abs() <= 0.1,
                "up {upward}: frame {n} change {c} dB, the reference {c_ref} dB"
            );
            assert!(
                (f64::from(y) - y_ref).abs() <= 2.0e-4,
                "up {upward}: frame {n} output {y}, the reference {y_ref}"
            );
        }
        let ours: Vec<f64> = change.iter().map(|&c| f64::from(c)).collect();
        let held = expected[14_399];
        assert!(
            if upward { held > 3.0 } else { held < -3.0 },
            "up {upward}: the burst engages ({held} dB)"
        );
        if let Some(clamp) = plateau {
            // The per-sample level ripples with the tone and the attack follows it, so the boost
            // settles just under the clamp (unclamped, the curve asks for +18 dB).
            let peak = ours.iter().copied().fold(f64::MIN, f64::max);
            assert!(
                peak <= clamp && peak >= clamp - 0.1,
                "up {upward}: the range holds the boost at {clamp} dB (peak {peak} dB)"
            );
        }
        let timing = [
            (2_400, 0.9, true, "attack"),
            (14_400, 0.1, false, "release"),
        ];
        for (from, fraction, rising, name) in timing {
            let mine = crossing(&ours, from, held, fraction, rising) - from;
            let theirs = crossing(&expected, from, held, fraction, rising) - from;
            let allowed = (0.02 * theirs as f64).max(1.0);
            assert!(
                (mine as f64 - theirs as f64).abs() <= allowed,
                "up {upward}: {name} {mine} samples, the reference {theirs}"
            );
        }
        // 1e6 frames of the repeated burst, in quanta: every output word finite.
        let mut effect = prepare(&values, LinkMode::DualMono);
        let mut left = vec![0.0_f32; 128];
        for block in 0..1_000_000 / 128 {
            for (frame, word) in left.iter_mut().enumerate() {
                *word = burst((block * 128 + frame) % FRAMES);
            }
            let mut right = left.clone();
            process(
                &mut effect,
                &mut left,
                &mut right,
                (block * 128) as u64,
                &[],
            );
            assert!(
                left.iter().chain(&right).all(|w| w.is_finite()),
                "up {upward}: block {block} finite"
            );
        }
    }
}

/// With the sidechain connected and the external detector, a steady 1 kHz main tone at -30 dBFS,
/// below the threshold through the band, is cut only while a -6 dBFS 1 kHz burst plays on the
/// sidechain: bit for bit the input before the burst, and within `2e-4` of the reference band
/// driven by the **sidechain** everywhere.
#[test]
fn a_connected_sidechain_alone_drives_the_external_detector() {
    const FRAMES: usize = 24_000;
    let main = |n: usize| tone(1_000.0, n, 0.0316);
    let side = |n: usize| {
        if (2_400..12_000).contains(&n) {
            tone(1_000.0, n, 0.5)
        } else {
            0.0
        }
    };
    let mut values = only(0);
    set_band(&mut values, 0, FREQUENCY, 1_000.0);
    set_band(&mut values, 0, DETECTOR, 2.0);
    let (out, _) = render_band(&values, true, main, side, FRAMES);
    let law = ReferenceDynamicBandLaw {
        threshold_db: -20.0,
        ratio: 2.0,
        knee_db: 6.0,
        range_db: 12.0,
        attack_ms: 5.0,
        release_ms: 100.0,
        upward: false,
    };
    let mut reference =
        ReferenceDynamicEqBand::new(f64::from(RATE), 1_000.0, 1.0, 0.0, law, true).expect("design");
    let mut deepest = 0.0_f64;
    for (n, &y) in out.iter().enumerate() {
        let (y_ref, change) = reference.process(f64::from(main(n)), f64::from(side(n)));
        deepest = deepest.min(change);
        assert!(
            (f64::from(y) - y_ref).abs() <= 2.0e-4,
            "frame {n}: {y}, the reference {y_ref}"
        );
        if n < 2_400 {
            assert_eq!(
                y.to_bits(),
                main(n).to_bits(),
                "frame {n}: no change before the burst"
            );
        }
    }
    assert!(deepest < -3.0, "the sidechain burst engages ({deepest} dB)");
}
