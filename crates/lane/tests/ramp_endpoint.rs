//! Issue #1408 gate 1: the D11 ramp update never passes its target.
//!
//! `ramp_toward(current, step, target)` is `current + step` held inside
//! `[min(current, target), max(current, target)]`, with the strict-comparison operand order that
//! keeps every in-range word's bits (signed zeros included) and passes a NaN through. Every builtin
//! ramp (trim and polarity, fader and mute, the matrix words) advances through it. The law is one
//! generic body, so it is checked at every width this build has.
//!
//! Mutation evidence is in the issue's attempt record.

mod support;

use lane::Lane;
use lane::kernels::builtins::ramp_toward;
use support::Xorshift64Star;

/// Random `(start, target, n)` ramps per width. The `--release` run is the gate's 100,000; a debug
/// run takes a prefix of the same seeded sequence, to keep the workspace suite quick.
const RANDOM_RAMPS: usize = if cfg!(debug_assertions) {
    4_000
} else {
    100_000
};

/// Longest random window.
const MAX_WINDOW: u32 = 4_096;

/// `ramp_toward` at one width on splatted operands, returning every lane's bits.
fn toward_bits<L: Lane>(current: f32, step: f32, target: f32) -> Vec<u32> {
    let mut bits = [0_u32; 8];
    ramp_toward(L::splat(current), L::splat(step), L::splat(target)).store_bits(&mut bits);
    bits[..L::WIDTH].to_vec()
}

fn assert_toward<L: Lane>(current: f32, step: f32, target: f32, expected: f32) {
    let name = core::any::type_name::<L>();
    let actual = toward_bits::<L>(current, step, target);
    assert!(
        actual.iter().all(|&bits| bits == expected.to_bits()),
        "{name}: ramp_toward({current:e}, {step:e}, {target:e}) = {:?}, expected {expected:e} \
         ({:#010x})",
        actual
            .iter()
            .map(|&bits| f32::from_bits(bits))
            .collect::<Vec<_>>(),
        expected.to_bits()
    );
}

/// The fixed cases: past the target, in range, signed zeros, a settled word, a NaN step.
fn fixed_cases<L: Lane>() {
    // Strictly past the target, in either direction: the word holds the target.
    assert_toward::<L>(0.5, 0.75, 1.0, 1.0);
    assert_toward::<L>(-0.5, -0.75, -1.0, -1.0);
    // One ulp short of the +24 dB trim word, stepping two ulps: the D11 overshoot in miniature.
    let trim = 15.848_932_f32;
    assert_toward::<L>(
        f32::from_bits(trim.to_bits() - 1),
        // 2^-19: two ulps in [8, 16).
        f32::from_bits(0x3600_0000),
        trim,
        trim,
    );
    assert_toward::<L>(-0.25, 1.0, 0.5, 0.5);
    // In range: exactly `current + step`, including landing on the target.
    assert_toward::<L>(0.25, 0.25, 1.0, 0.5);
    assert_toward::<L>(-0.25, -0.5, -1.0, -0.75);
    assert_toward::<L>(0.5, 0.5, 1.0, 1.0);
    assert_toward::<L>(1.0, -1.5, -2.0, -0.5);
    // A `+0.0` word next to a `-0.0` endpoint keeps its sign: `-1 + 1` is `+0.0` and the
    // target is `-0.0`. A clamp written `next.min(high)` would return `-0.0` here.
    assert_toward::<L>(-1.0, 1.0, -0.0, 0.0);
    // A `-0.0` word next to a `+0.0` endpoint keeps its sign. A clamp written `next.max(low)`
    // would return `+0.0` here.
    assert_toward::<L>(-0.0, -0.0, 0.0, -0.0);
    // Once `current == target`, any nonzero step (infinities included) returns the target.
    for target in [1.0_f32, -1.0, 15.848_932, -15.848_932, 0.0, -0.0, 1.0e-6] {
        for step in [
            1.0_f32,
            -1.0,
            1.0e-30,
            -1.0e-30,
            f32::from_bits(1),
            -f32::from_bits(1),
            f32::INFINITY,
            f32::NEG_INFINITY,
        ] {
            assert_toward::<L>(target, step, target, target);
        }
    }
    // A NaN step passes through instead of being hidden, mid-ramp and settled.
    for (current, target) in [(0.5_f32, 1.0_f32), (1.0, 1.0), (-0.25, -1.0)] {
        let actual = toward_bits::<L>(current, f32::NAN, target);
        assert!(
            actual.iter().all(|&bits| f32::from_bits(bits).is_nan()),
            "{}: a NaN step from {current} toward {target} must stay NaN",
            core::any::type_name::<L>()
        );
    }
}

#[test]
fn ramp_toward_holds_the_target_and_keeps_in_range_bits_at_every_width() {
    lane::each_lane!(|L| fixed_cases::<L>());
}

/// A finite value with magnitude in `[2^-20, 2^4)` and a random sign.
fn magnitude_value(rng: &mut Xorshift64Star) -> f32 {
    let bits = rng.next_u32();
    let mantissa = bits & 0x007F_FFFF;
    let exponent = (bits >> 23) % 24 + 127 - 20;
    f32::from_bits((bits & 0x8000_0000) | (exponent << 23) | mantissa)
}

/// Iterates `RANDOM_RAMPS` seeded D11 ramps at one width, `W` independent ramps per vector, with
/// and without the clamp, and returns how many ramps the unclamped law took past their target.
fn random_ramps<L: Lane>() -> usize {
    let name = core::any::type_name::<L>();
    let width = L::WIDTH;
    let mut rng = Xorshift64Star::new(0x1408_0000_0000_0001);
    let mut overshooting = 0;
    let one = L::splat(1.0);
    let zero = L::zero();
    let mut first = 0;
    while first < RANDOM_RAMPS {
        let mut start = [0.0_f32; 8];
        let mut target = [0.0_f32; 8];
        let mut step = [0.0_f32; 8];
        let mut window = [0_u32; 8];
        for lane in 0..width {
            start[lane] = magnitude_value(&mut rng);
            target[lane] = magnitude_value(&mut rng);
            window[lane] = rng.next_u32() % MAX_WINDOW + 1;
            // The control side's one division per event.
            step[lane] = (target[lane] - start[lane]) / window[lane] as f32;
        }
        let longest = window[..width].iter().copied().max().expect("a lane");
        let target_vector = L::load(&target[..width]);
        let step_vector = L::load(&step[..width]);
        let mut remaining = L::load(&window.map(|samples| samples as f32)[..width]);
        let mut clamped = L::load(&start[..width]);
        let mut unclamped = clamped;
        // Per lane: whether the unclamped word has left the interval yet.
        let mut left_interval = [false; 8];
        for frame in 0..longest {
            remaining = remaining.sub(one);
            let done = remaining.le(zero);
            clamped = L::select(
                done,
                target_vector,
                ramp_toward(clamped, step_vector, target_vector),
            );
            unclamped = L::select(done, target_vector, unclamped.add(step_vector));
            let mut clamped_bits = [0_u32; 8];
            let mut unclamped_bits = [0_u32; 8];
            clamped.store_bits(&mut clamped_bits);
            unclamped.store_bits(&mut unclamped_bits);
            for lane in 0..width {
                if frame >= window[lane] {
                    continue;
                }
                let low = start[lane].min(target[lane]);
                let high = start[lane].max(target[lane]);
                let word = f32::from_bits(clamped_bits[lane]);
                assert!(
                    (low..=high).contains(&word),
                    "{name}: ramp {start:e} -> {target:e} over {n}: frame {frame} word {word:e} \
                     leaves [{low:e}, {high:e}]",
                    start = start[lane],
                    target = target[lane],
                    n = window[lane],
                );
                let old = f32::from_bits(unclamped_bits[lane]);
                if !left_interval[lane] && !(low..=high).contains(&old) {
                    left_interval[lane] = true;
                    overshooting += 1;
                }
                if !left_interval[lane] {
                    assert_eq!(
                        clamped_bits[lane],
                        unclamped_bits[lane],
                        "{name}: ramp {start:e} -> {target:e} over {n}: frame {frame} moved \
                         while the unclamped word {old:e} was in range",
                        start = start[lane],
                        target = target[lane],
                        n = window[lane],
                    );
                }
                if frame + 1 == window[lane] {
                    assert_eq!(
                        clamped_bits[lane],
                        target[lane].to_bits(),
                        "{name}: the last frame assigns the target"
                    );
                }
            }
        }
        first += width;
    }
    overshooting
}

#[test]
fn random_ramps_stay_inside_their_endpoints_at_every_width() {
    lane::each_lane!(|L| {
        let overshooting = random_ramps::<L>();
        // The sample must reach the clamp, or the bit-identity half proves nothing about it.
        assert!(
            overshooting > 0,
            "{}: no random ramp passed its target under the unclamped law",
            core::any::type_name::<L>()
        );
    });
}
