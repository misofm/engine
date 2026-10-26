#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately.
//! The de-esser's own output law, on the shared band step (whose detector and gain law
//! `effect-runtime`'s `tests/dynamic_band.rs` holds against the oracle): every arm against the
//! independent `f64` reference, and the spec's split-band acceptance gate.

mod support;

use dsp_reference::{
    ReferenceBandTap, ReferenceDeEsser, ReferenceDeEsserOutput, ReferenceDynamicBandLaw,
};
use effect_contract::LinkMode;
use support::{RATE, defaults, prepare, process, set, tone};

const FRAMES: usize = 24_000;

/// 8 kHz at -6 dBFS from 50 ms to 250 ms over a -30 dBFS 1 kHz tone.
fn input(n: usize) -> f32 {
    let burst = if (2_400..12_000).contains(&n) {
        tone(8_000.0, n, 0.5)
    } else {
        0.0
    };
    burst + tone(1_000.0, n, 0.0316)
}

/// Renders `FRAMES` of `signal` on both channels through a scalar de-esser with `values`.
fn render(values: &support::Values, signal: impl Fn(usize) -> f32) -> Vec<f32> {
    let mut effect = prepare(values, LinkMode::DualMono);
    let mut out: Vec<f32> = (0..FRAMES).map(&signal).collect();
    let mut right = out.clone();
    for (start, (left, right)) in out.chunks_mut(128).zip(right.chunks_mut(128)).enumerate() {
        process(&mut effect, left, right, (start * 128) as u64, &[]);
    }
    out
}

/// Wideband `g x`, split `x - (1 - g) b` on the band tap, split on the high-pass tap (the high
/// shelf `g x + (1 - g) l`, `l` the Butterworth low-pass, #1984), and listen `f` each match the
/// reference sample for sample within `1e-5` (the input peaks near `0.53`; measured `2.9e-7`).
#[test]
fn every_output_arm_matches_the_reference() {
    use ReferenceDeEsserOutput::{Listen, Split, Wideband};
    for (mode, detector, listen, output, tap) in [
        (1.0, 1.0, 0.0, Wideband, ReferenceBandTap::BandPass),
        (2.0, 1.0, 0.0, Split, ReferenceBandTap::BandPass),
        (2.0, 2.0, 0.0, Split, ReferenceBandTap::HighPass),
        (2.0, 2.0, 1.0, Listen, ReferenceBandTap::HighPass),
    ] {
        let mut values = defaults();
        set(&mut values, 7, mode);
        set(&mut values, 8, detector);
        set(&mut values, 9, listen);
        let ours = render(&values, input);
        let law = ReferenceDynamicBandLaw {
            threshold_db: -20.0,
            ratio: 4.0,
            knee_db: 0.0,
            range_db: 20.0,
            attack_ms: 1.0,
            release_ms: 50.0,
            upward: false,
        };
        let mut reference =
            ReferenceDeEsser::new(f64::from(RATE), 6_500.0, 2.0, tap, law, output).expect("design");
        let mut deepest = 0.0_f64;
        for (n, &sample) in ours.iter().enumerate() {
            let (expected, change) = reference.process(f64::from(input(n)));
            deepest = deepest.min(change);
            assert!(
                (f64::from(sample) - expected).abs() <= 1.0e-5,
                "{output:?} {tap:?}: sample {n} is {sample}, the reference {expected}"
            );
        }
        assert!(
            deepest < -3.0,
            "{output:?} {tap:?}: the burst engages ({deepest} dB)"
        );
    }
}

/// `|X(bin)|` in dB of `samples` (a whole number of the bin's periods).
fn bin_db(samples: &[f32], frequency: f64) -> f64 {
    let (mut re, mut im) = (0.0_f64, 0.0_f64);
    for (n, &x) in samples.iter().enumerate() {
        let phase = core::f64::consts::TAU * frequency * n as f64 / f64::from(RATE);
        re += f64::from(x) * phase.cos();
        im -= f64::from(x) * phase.sin();
    }
    20.0 * re.hypot(im).log10()
}

/// In split mode at the defaults (6.5 kHz, Q 2, 20 dB range), with either detector, a 1 kHz tone
/// 2.7 octaves below the centre, inside the documented nominal region (at least 1.83 octaves away
/// at Q 2), moves by at most 0.1 dB while the 7 kHz burst over it is cut by at least 3 dB: the band
/// detector cuts the band, the high-pass detector is a dynamic high shelf (#1984).
#[test]
fn split_mode_cuts_the_band_and_leaves_an_out_of_band_tone() {
    let signal = |n: usize| {
        let burst = if (2_400..12_000).contains(&n) {
            tone(7_000.0, n, 0.5)
        } else {
            0.0
        };
        burst + tone(1_000.0, n, 0.0316)
    };
    let dry: Vec<f32> = (0..FRAMES).map(signal).collect();
    for detector in [1.0, 2.0] {
        let mut values = defaults();
        set(&mut values, 8, detector);
        let ours = render(&values, signal);
        // 100 ms inside the burst, after the attack: whole periods of both tones.
        let window = 7_200..12_000;
        let low = bin_db(&ours[window.clone()], 1_000.0) - bin_db(&dry[window.clone()], 1_000.0);
        let high = bin_db(&ours[window.clone()], 7_000.0) - bin_db(&dry[window], 7_000.0);
        assert!(
            low.abs() <= 0.1,
            "detector {detector}: the 1 kHz tone moved {low} dB"
        );
        assert!(
            high <= -3.0,
            "detector {detector}: the 7 kHz burst moved only {high} dB"
        );
    }
}

/// Renders `frames` of `main` on both channels through a scalar de-esser at `rate` with `values`
/// and a connected sidechain carrying `side`, each of `spans` in the 128-frame block it starts;
/// returns the left channel.
fn render_connected(
    values: &support::Values,
    rate: u32,
    frames: usize,
    main: impl Fn(usize) -> f32,
    side: impl Fn(usize) -> f32,
    spans: &[effect_contract::PreparedAutomationSpan],
) -> Vec<f32> {
    use effect_contract::{
        EffectProcessBlock, NativeEffectFactory, PortId, PreparedPorts, PreparedSidechainPort,
    };
    let mut request = support::request(values, LinkMode::DualMono);
    request.sample_rate = rate;
    request.tail_bound = conformance::tail_bound_of(
        Box::new(de_esser::DeEsserFactory),
        rate,
        effect_contract::EffectQuality::Normal,
    );
    request.ports = PreparedPorts {
        sidechain: PreparedSidechainPort::Connected {
            id: PortId::new("sidechain-in").expect("port"),
            required: false,
        },
    };
    let mut effect = de_esser::DeEsserFactory
        .prepare(request)
        .expect("connected");
    let mut left: Vec<f32> = (0..frames).map(main).collect();
    let mut right = left.clone();
    let sidechain: Vec<f32> = (0..frames).map(side).collect();
    for start in (0..frames).step_by(128) {
        let end = (start + 128).min(frames);
        let block_spans: Vec<_> = spans
            .iter()
            .filter(|span| span.sample == start as u64)
            .copied()
            .collect();
        let block = EffectProcessBlock::new(
            &mut left[start..end],
            &mut right[start..end],
            Some((&sidechain[start..end], &sidechain[start..end])),
            start as u64,
            &block_spans,
            128,
        )
        .expect("block");
        effect.processor.process(block);
    }
    left
}

/// Split mode with the high-pass detector is a dynamic high shelf with no boost (#1984): with a
/// sidechain holding the range (-20 dB, `g = 0.1`), each low-level probe tone from 2 octaves below
/// to 2 octaves above the 6.5 kHz centre, at 96 kHz and Q 0.5, 1, 2 and 4, is at most +0.02 dB
/// over its dry level and within 0.01 dB of `|g + (1 - g) LP(w)|`, `LP` the Butterworth low-pass at
/// the centre and `w = tan(pi f / fs) / tan(pi f_c / fs)`, whatever Q is.
#[test]
fn split_with_the_high_pass_detector_is_a_shelf_without_boost() {
    const RATE_96: u32 = 96_000;
    const CENTRE: f64 = 6_500.0;
    const FRAMES_96: usize = 9_600;
    let sine = |frequency: f64, amplitude: f64| {
        move |n: usize| {
            let phase = core::f64::consts::TAU * frequency * n as f64 / f64::from(RATE_96);
            (amplitude * math::sin(phase)) as f32
        }
    };
    // Hann-windowed `|X(f)|` over the settled second half.
    let magnitude = |samples: &[f32], frequency: f64| {
        let tail = &samples[FRAMES_96 / 2..];
        let (mut re, mut im) = (0.0_f64, 0.0_f64);
        for (n, &x) in tail.iter().enumerate() {
            let window = 0.5 - 0.5 * (core::f64::consts::TAU * n as f64 / tail.len() as f64).cos();
            let phase = core::f64::consts::TAU * frequency * n as f64 / f64::from(RATE_96);
            re += window * f64::from(x) * phase.cos();
            im -= window * f64::from(x) * phase.sin();
        }
        re.hypot(im)
    };
    let held = 0.1_f64;
    let prewarp = |frequency: f64| (core::f64::consts::PI * frequency / f64::from(RATE_96)).tan();
    for q in [0.5_f32, 1.0, 2.0, 4.0] {
        let mut values = defaults();
        for (parameter, value) in [
            (1, q),
            (2, -60.0),
            (3, 20.0),
            (4, 20.0),
            (5, 0.1),
            (6, 500.0),
            (7, 2.0),
            (8, 2.0),
        ] {
            set(&mut values, parameter, value);
        }
        for octave in [-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0] {
            let frequency = CENTRE * 2.0_f64.powf(octave);
            let main = sine(frequency, 1.0e-3);
            let ours = render_connected(&values, RATE_96, FRAMES_96, main, sine(CENTRE, 1.0), &[]);
            let dry: Vec<f32> = (0..FRAMES_96).map(&main).collect();
            let got = 20.0 * (magnitude(&ours, frequency) / magnitude(&dry, frequency)).log10();
            let w = prewarp(frequency) / prewarp(CENTRE);
            // LP(jw) = 1 / (1 - w^2 + j sqrt2 w).
            let (dr, di) = (1.0 - w * w, core::f64::consts::SQRT_2 * w);
            let norm = dr * dr + di * di;
            let (lr, li) = (dr / norm, -di / norm);
            let (hr, hi) = (held + (1.0 - held) * lr, (1.0 - held) * li);
            let analytic = 20.0 * hr.hypot(hi).log10();
            assert!(got <= 0.02, "Q {q}, {octave:+} oct: boosted {got:.3} dB");
            assert!(
                (got - analytic).abs() <= 0.01,
                "Q {q}, {octave:+} oct: {got:.3} dB, the shelf {analytic:.3} dB"
            );
        }
    }
}

/// A detector switch in split mode never boosts (#1984): with a sidechain holding the range
/// (-20 dB, `g = 0.1`), Q 4 at 96 kHz, a 6.5 kHz main tone at `1e-3` and the band detector settled,
/// the switch to the high-pass detector crossfades the band cut `x - (1 - g) b` into the high
/// shelf over 64 samples; every output sample of that ramp is at most the dry peak, because each
/// arm has `|H| <= 1` (a cut on the crossfaded tap would reach about +2.2 dB near `D = 0.6`).
#[test]
fn a_detector_switch_in_split_mode_never_boosts() {
    const RATE_96: u32 = 96_000;
    const CENTRE: f64 = 6_500.0;
    const SWITCH: usize = 38 * 128;
    let sine = |amplitude: f64| {
        move |n: usize| {
            let phase = core::f64::consts::TAU * CENTRE * n as f64 / f64::from(RATE_96);
            (amplitude * math::sin(phase)) as f32
        }
    };
    let mut values = defaults();
    for (parameter, value) in [
        (1, 4.0),
        (2, -60.0),
        (3, 20.0),
        (4, 20.0),
        (5, 0.1),
        (6, 500.0),
        (7, 2.0),
        (8, 1.0),
    ] {
        set(&mut values, parameter, value);
    }
    let main = sine(1.0e-3);
    let spans = support::spans_both(8, 2.0, SWITCH as u64);
    let ours = render_connected(&values, RATE_96, 2 * SWITCH, main, sine(1.0), &spans);
    let ramp = SWITCH..SWITCH + 64;
    let dry = ramp.clone().map(|n| main(n).abs()).fold(0.0_f32, f32::max);
    let peak = ours[ramp].iter().map(|y| y.abs()).fold(0.0_f32, f32::max);
    assert!(
        peak <= dry,
        "the switch ramp peaks at {:.2} dB over the dry tone",
        20.0 * f64::from(peak / dry).log10()
    );
}

/// With the sidechain connected, a steady -10 dBFS 1 kHz main tone (below the threshold through
/// the band) is reduced only while the 8 kHz burst plays on the sidechain: the wideband output is
/// `g x` with `g` the reference's change for the **sidechain** input, within `1e-5`.
#[test]
fn a_connected_sidechain_alone_drives_the_detector() {
    let mut values = defaults();
    set(&mut values, 7, 1.0);
    let main = |n: usize| tone(1_000.0, n, 0.316);
    let side = |n: usize| input(n) - tone(1_000.0, n, 0.0316);
    let left = render_connected(&values, RATE, FRAMES, main, side, &[]);
    let law = ReferenceDynamicBandLaw {
        threshold_db: -20.0,
        ratio: 4.0,
        knee_db: 0.0,
        range_db: 20.0,
        attack_ms: 1.0,
        release_ms: 50.0,
        upward: false,
    };
    let mut reference = ReferenceDeEsser::new(
        f64::from(RATE),
        6_500.0,
        2.0,
        ReferenceBandTap::BandPass,
        law,
        ReferenceDeEsserOutput::Wideband,
    )
    .expect("design");
    let mut deepest = 0.0_f64;
    for (n, &sample) in left.iter().enumerate() {
        let (_, change) = reference.process(f64::from(side(n)));
        deepest = deepest.min(change);
        let expected = f64::from(main(n)) * 10.0_f64.powf(change / 20.0);
        assert!(
            (f64::from(sample) - expected).abs() <= 1.0e-5,
            "sample {n} is {sample}, the reference {expected}"
        );
        if n < 2_400 {
            assert_eq!(
                sample.to_bits(),
                main(n).to_bits(),
                "no reduction before the burst"
            );
        }
    }
    assert!(deepest < -3.0, "the sidechain burst engages ({deepest} dB)");
}
