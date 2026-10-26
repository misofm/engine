//! Inter-channel correlation of the tail (the note's test 3).

mod common;

use common::{DECAY, DIFFUSION, SIZE, WIDTH, defaults, impulse_response};

/// `r(0) = sum L R / sqrt(sum L^2 sum R^2)` from `floor(0.1 fs)` to the end.
fn tail_correlation(left: &[f32], right: &[f32], sample_rate: u32) -> f64 {
    let start = (sample_rate / 10) as usize;
    let (mut lr, mut ll, mut rr) = (0.0_f64, 0.0_f64, 0.0_f64);
    for (l, r) in left[start..].iter().zip(&right[start..]) {
        let (l, r) = (f64::from(*l), f64::from(*r));
        lr += l * r;
        ll += l * l;
        rr += r * r;
    }
    lr / (ll * rr).sqrt()
}

/// At width 1 a centred impulse gives a decorrelated tail, `|r(0)| <= 0.1`, over size
/// {0.25, 1, 2} x diffusion {0, 0.7, 1} x decay {1.8, 8} s; at width 0 the two outputs are the
/// same signal, `r(0) >= 0.99`. Render length `max(1, 0.5 T)` s, 48 kHz.
///
/// Red for: output taps whose line rows are not orthogonal (the two channels share most of their
/// energy), or a width matrix (R10) that does not collapse to mono at 0 or does not keep the side
/// signal at 1. No other test measures the relation between the two outputs.
#[test]
fn tail_is_decorrelated_at_width_one_and_mono_at_width_zero() {
    const RATE: u32 = 48_000;
    let mut worst = 0.0_f64;
    for size in [0.25_f32, 1.0, 2.0] {
        for diffusion in [0.0_f32, 0.7, 1.0] {
            for decay in [1_800.0_f32, 8_000.0] {
                let mut values = defaults();
                values[SIZE] = size;
                values[DIFFUSION] = diffusion;
                values[DECAY] = decay;
                let frames = (f64::from(RATE) * (f64::from(decay) / 2000.0).max(1.0)) as usize;
                let (left, right) = impulse_response(&values, RATE, (1.0, 1.0), frames);
                let r = tail_correlation(&left, &right, RATE);
                println!("size {size} diffusion {diffusion} decay {decay}: r(0) = {r:+.4}");
                worst = worst.max(r.abs());
            }
        }
    }
    assert!(worst <= 0.1, "worst |r(0)| {worst}");

    let mut mono = defaults();
    mono[WIDTH] = 0.0;
    let (left, right) = impulse_response(&mono, RATE, (1.0, 1.0), RATE as usize);
    let r = tail_correlation(&left, &right, RATE);
    println!("width 0: r(0) = {r}");
    assert!(r >= 0.99, "width 0 r(0) {r}");
}
