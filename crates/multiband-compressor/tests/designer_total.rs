//! #1366 gate 1: the crossover designer is total over its legal domain.
//!
//! `design_lr4_words` has no failure branch, so render may call it (#1338). That is licensed only
//! by this proof: for every `f32` crossover in `[80, 8000]` at every launch rate, the checked
//! `design_lr4` accepts, returns exactly the infallible form's words, and the words are a
//! contractive SVF transition within `effect_runtime::svf::NORM_TOLERANCE`. The exhaustive sweep
//! is `#[ignore]` and runs in release; the per-PR test runs the same checks on every 1024th `f32`
//! plus both ends.

use effect_runtime::svf::{NORM_TOLERANCE, transition_norm};
use multiband_compressor::{design_lr4, design_lr4_words};

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const LOWEST_HZ: f32 = 80.0;
const HIGHEST_HZ: f32 = 8_000.0;

/// Runs the three checks for one crossover and returns its norm excess over 1.
fn check(sample_rate: u32, crossover_hz: f32) -> f64 {
    let Some(words) = design_lr4(sample_rate, crossover_hz) else {
        panic!("design_lr4 refused {crossover_hz:?} Hz at {sample_rate} Hz");
    };
    let raw = design_lr4_words(sample_rate, crossover_hz);
    assert_eq!(
        words.map(f32::to_bits),
        raw.map(f32::to_bits),
        "checked and infallible designs differ at {crossover_hz:?} Hz, {sample_rate} Hz"
    );
    let norm = transition_norm(words[0], words[1], words[2]);
    assert!(
        norm <= NORM_TOLERANCE,
        "norm {norm} exceeds the tolerance at {crossover_hz:?} Hz, {sample_rate} Hz"
    );
    norm - 1.0
}

/// Sweeps every `stride`-th bit pattern of the domain, both ends always included, at every rate.
fn sweep(stride: usize) -> (f64, u32, f32, u64) {
    let lowest = LOWEST_HZ.to_bits();
    let highest = HIGHEST_HZ.to_bits();
    std::thread::scope(|scope| {
        let workers: Vec<_> = RATES
            .into_iter()
            .map(|rate| {
                scope.spawn(move || {
                    let mut worst = (f64::NEG_INFINITY, rate, LOWEST_HZ, 0_u64);
                    let tail =
                        (!((highest - lowest) as usize).is_multiple_of(stride)).then_some(highest);
                    let bits = (lowest..=highest).step_by(stride).chain(tail);
                    for pattern in bits {
                        let crossover = f32::from_bits(pattern);
                        let excess = check(rate, crossover);
                        worst.3 += 1;
                        if excess > worst.0 {
                            worst = (excess, rate, crossover, worst.3);
                        }
                    }
                    worst
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().expect("sweep worker"))
            .fold((f64::NEG_INFINITY, 0, 0.0, 0), |acc, next| {
                let count = acc.3 + next.3;
                if next.0 > acc.0 {
                    (next.0, next.1, next.2, count)
                } else {
                    (acc.0, acc.1, acc.2, count)
                }
            })
    })
}

fn report(label: &str, (excess, rate, crossover, count): (f64, u32, f32, u64)) {
    println!(
        "{label}: {count} designs; largest norm excess {excess:e} at {crossover:?} Hz, {rate} Hz \
         (tolerance excess {:e})",
        NORM_TOLERANCE - 1.0
    );
}

#[test]
fn designer_is_total_on_a_strided_domain() {
    report("strided", sweep(1024));
}

#[test]
#[ignore = "exhaustive sweep of every f32 crossover; run in release"]
fn designer_is_total_on_every_f32_in_the_domain() {
    report("exhaustive", sweep(1));
}
