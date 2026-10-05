//! Issue #1409 gate 1: every effect parameter ramp stays inside its endpoints.
//!
//! The D11 law is `step = (target - current) / n` once, then `current = ramp_toward(current,
//! step, target)` per sample and an exact assignment of `target` on the last. `ramp_toward` is
//! `current + step` held inside `[min(current, target), max(current, target)]` (issue #1408).
//! Three statements of the law must agree word for word: [`LinearRamp::next_value`], the
//! contract's [`ParameterSmoother`] (`Linear`), and `lane::kernels::ramp_block` driven through
//! [`LinearRamp::advance_block`] in random partitions at every width this build has. Every word
//! lies between the ramp's start and its target, and the words are the unclamped law's bits up to
//! the first frame at which the unclamped word leaves that interval.
//!
//! The restore half: a restored ramp carries its step verbatim, so the step need not be the one
//! `set_target` would derive. For any finite step, of either sign and any magnitude, the clamped
//! walk stays between the restored `current` and `target` (#1409 D2), which is why
//! [`ramp_path_inside`] decides a path from its two endpoints; it must agree with walking it.
//!
//! Mutation evidence is in the issue's attempt record.

use effect_contract::{ParameterSmoother, SmoothingRule};
use effect_runtime::ramp::LinearRamp;
use effect_runtime::state_payload::ramp_path_inside;
use lane::Lane;
use lane::kernels::ramp_block;

/// Random `(start, target, n)` ramps. The `--release` run is the gate's 100,000; a debug run takes
/// a prefix of the same seeded sequence, to keep the workspace suite quick.
const RANDOM_RAMPS: usize = if cfg!(debug_assertions) {
    4_000
} else {
    100_000
};

/// Random restored ramps for the restore half.
const RESTORED_RAMPS: usize = if cfg!(debug_assertions) {
    20_000
} else {
    200_000
};

/// Longest random window.
const MAX_WINDOW: u32 = 4_096;

/// The longest window an effect ramp receives, and every restore validator's `remaining` bound.
const EFFECT_WINDOW: u32 = 64;

/// The probe's domain-edge targets (ratio floor and mix top, delay feedback and damping limits,
/// ratio top, threshold floor, gate range top, a ceiling-like value).
const EDGES: [f32; 9] = [1.0, 0.95, -0.95, 0.995, 20.0, -80.0, 96.0, -3.0, 0.5];

/// `xorshift64*` (Vigna 2016).
struct Rng(u64);

impl Rng {
    const fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn below(&mut self, bound: u32) -> u32 {
        self.next_u32() % bound
    }
}

/// A finite value with magnitude in `[2^-20, 2^7]` and a random sign.
fn magnitude_value(rng: &mut Rng) -> f32 {
    let bits = rng.next_u32();
    let mantissa = bits & 0x007F_FFFF;
    let exponent = (bits >> 23) % 27 + 127 - 20;
    f32::from_bits((bits & 0x8000_0000) | (exponent << 23) | mantissa)
}

/// `value` moved `ulps` steps along the `f32` grid, away from zero when `outward`.
fn ulps_from(value: f32, ulps: u32, outward: bool) -> f32 {
    let bits = value.to_bits();
    f32::from_bits(if outward { bits + ulps } else { bits - ulps })
}

/// One random `(start, target, n)`: half with an edge target and a start `1..=4096` ulps from it,
/// on either side; half with both ends drawn from magnitudes.
fn random_ramp(rng: &mut Rng, index: usize) -> (f32, f32, u32) {
    loop {
        let (start, target) = if index.is_multiple_of(2) {
            let target = EDGES[rng.below(EDGES.len() as u32) as usize];
            let ulps = rng.below(4_096) + 1;
            (ulps_from(target, ulps, rng.below(2) == 0), target)
        } else {
            (magnitude_value(rng), magnitude_value(rng))
        };
        // Half the windows are the effects' 64; the other half anywhere in `1..=4096`.
        let samples = if rng.below(2) == 0 {
            EFFECT_WINDOW
        } else {
            rng.below(MAX_WINDOW) + 1
        };
        if start.to_bits() != target.to_bits() {
            return (start, target, samples);
        }
    }
}

/// The unclamped D11 law's `n` words, the oracle the clamped words must match while it is in
/// range.
fn unclamped_words(start: f32, target: f32, samples: u32) -> Vec<f32> {
    let step = (target - start) / samples as f32;
    let mut current = start;
    (1..=samples)
        .map(|sample| {
            current = if sample == samples {
                target
            } else {
                current + step
            };
            current
        })
        .collect()
}

/// `ramp_block` at one width, splatted, partitioned at random through `advance_block`: the gain
/// it applies to a unit input on each frame, checked equal on every lane.
fn block_words<L: Lane>(rng: &mut Rng, start: f32, target: f32, samples: u32) -> Vec<f32> {
    let name = core::any::type_name::<L>();
    let mut ramp = LinearRamp::fixed(start);
    ramp.set_target(target, samples);
    let mut words = Vec::with_capacity(samples as usize);
    let mut io = vec![0.0_f32; 512 * L::WIDTH];
    while words.len() < samples as usize {
        let left = samples as usize - words.len();
        let frames = (rng.below(97) as usize + 1).min(left);
        let segment = ramp.advance_block::<L>(frames);
        let block = &mut io[..frames * L::WIDTH];
        block.fill(1.0);
        let _ = ramp_block::<L>(block, frames, &segment);
        for frame in block.chunks_exact(L::WIDTH) {
            assert!(
                frame
                    .iter()
                    .all(|word| word.to_bits() == frame[0].to_bits()),
                "{name}: lanes of a splatted ramp differ: {frame:?}"
            );
            words.push(frame[0]);
        }
    }
    words
}

/// Checks one random ramp against the three statements of the law and the unclamped oracle.
/// Returns whether the unclamped law took it outside its endpoints.
fn check_ramp(rng: &mut Rng, start: f32, target: f32, samples: u32) -> bool {
    let context = format!("ramp {start:e} -> {target:e} over {samples}");
    let mut ramp = LinearRamp::fixed(start);
    ramp.set_target(target, samples);
    let scalar: Vec<f32> = (0..samples).map(|_| ramp.next_value()).collect();
    assert!(!ramp.is_ramping(), "{context}: the ramp has ended");

    let mut smoother =
        ParameterSmoother::new(start, SmoothingRule::Linear, samples).expect("a finite start");
    assert!(smoother.set_target(target), "{context}: finite target");
    for (sample, word) in scalar.iter().enumerate() {
        assert_eq!(
            smoother.next_value().to_bits(),
            word.to_bits(),
            "{context}: ParameterSmoother differs from LinearRamp at sample {sample}"
        );
    }

    lane::each_lane!(|L| {
        let blocks = block_words::<L>(rng, start, target, samples);
        for (sample, (block, word)) in blocks.iter().zip(&scalar).enumerate() {
            assert_eq!(
                block.to_bits(),
                word.to_bits(),
                "{context}: ramp_block at {} differs from LinearRamp at sample {sample}",
                core::any::type_name::<L>()
            );
        }
    });

    let (low, high) = (start.min(target), start.max(target));
    let oracle = unclamped_words(start, target, samples);
    let mut left_interval = false;
    for (sample, (word, old)) in scalar.iter().zip(&oracle).enumerate() {
        assert!(
            (low..=high).contains(word),
            "{context}: sample {sample} word {word:e} leaves [{low:e}, {high:e}]"
        );
        left_interval |= !(low..=high).contains(old);
        if !left_interval {
            assert_eq!(
                word.to_bits(),
                old.to_bits(),
                "{context}: sample {sample} moved while the unclamped word {old:e} was in range"
            );
        }
    }
    assert_eq!(
        scalar.last().map(|word| word.to_bits()),
        Some(target.to_bits()),
        "{context}: the last sample assigns the target"
    );
    left_interval
}

#[test]
fn every_statement_of_the_law_stays_inside_its_endpoints_and_agrees() {
    let mut rng = Rng::new(0x1409_0000_0000_0001);
    let mut overshooting = [0_usize; 2];
    for index in 0..RANDOM_RAMPS {
        let (start, target, samples) = random_ramp(&mut rng, index);
        if check_ramp(&mut rng, start, target, samples) {
            overshooting[index % 2] += 1;
        }
    }
    // Both halves must reach the clamp, or the bit-identity half proves nothing about it.
    assert!(
        overshooting.iter().all(|&count| count > 0),
        "no ramp of a half passed its target under the unclamped law: {overshooting:?}"
    );
}

/// The probe's first domain-edge example, pinned: a ratio ramp from `1.0000039` to its floor
/// `1.0` took the unclamped word to `0.99999994` at sample 34; the clamped word holds `1.0`.
#[test]
fn the_ratio_floor_example_holds_its_target() {
    let start = f32::from_bits(1.0_f32.to_bits() + 33);
    let oracle = unclamped_words(start, 1.0, EFFECT_WINDOW);
    assert_eq!(oracle[33].to_bits(), 0.999_999_94_f32.to_bits());
    let mut ramp = LinearRamp::fixed(start);
    ramp.set_target(1.0, EFFECT_WINDOW);
    let words: Vec<f32> = (0..EFFECT_WINDOW).map(|_| ramp.next_value()).collect();
    assert!(words.iter().all(|&word| (1.0..=start).contains(&word)));
    assert_eq!(words[33].to_bits(), 1.0_f32.to_bits());
}

/// A finite `f32` of any magnitude (subnormals included) and either sign.
fn any_finite(rng: &mut Rng) -> f32 {
    loop {
        let value = f32::from_bits(rng.next_u32());
        if value.is_finite() {
            return value;
        }
    }
}

/// A restored ramp of [`EFFECT_WINDOW`] or fewer samples with a step drawn independently of its
/// endpoints, as a forged or foreign payload could carry it.
fn restored_ramp(rng: &mut Rng) -> LinearRamp {
    let current = magnitude_value(rng);
    let target = if rng.below(4) == 0 {
        current
    } else {
        magnitude_value(rng)
    };
    let step = match rng.below(4) {
        0 => any_finite(rng),
        1 => (target - current) / (rng.below(EFFECT_WINDOW) + 1) as f32 * magnitude_value(rng),
        2 => 0.0,
        _ => f32::from_bits(rng.next_u32() & 0x807F_FFFF),
    };
    LinearRamp {
        current,
        target,
        step,
        remaining: rng.below(EFFECT_WINDOW + 2),
    }
}

/// `ramp_path_inside` stated as a walk: the rules it keeps, and every value `next_value` visits.
fn walked_inside(ramp: LinearRamp, (low, high): (f32, f32), max_remaining: u32) -> bool {
    let inside = |value: f32| (low..=high).contains(&value);
    if ramp.remaining > max_remaining
        || !ramp.step.is_finite()
        || !inside(ramp.current)
        || !inside(ramp.target)
    {
        return false;
    }
    if ramp.remaining == 0 {
        return ramp.step.to_bits() == 0;
    }
    let mut walk = ramp;
    (0..ramp.remaining).all(|_| inside(walk.next_value()))
}

#[test]
fn a_restored_ramp_with_any_finite_step_stays_inside_and_the_validator_agrees() {
    let mut rng = Rng::new(0x1409_0000_0000_0002);
    let mut refused_by_bounds = 0_usize;
    for _ in 0..RESTORED_RAMPS {
        let ramp = restored_ramp(&mut rng);
        let context = format!("{ramp:?}");
        let (low, high) = (ramp.current.min(ramp.target), ramp.current.max(ramp.target));
        // D2: every word the restored ramp produces lies between its endpoints.
        let mut walk = ramp;
        for sample in 0..ramp.remaining {
            let word = walk.next_value();
            assert!(
                (low..=high).contains(&word),
                "{context}: sample {sample} word {word:e} leaves [{low:e}, {high:e}]"
            );
        }
        // The validator against the walk, on bounds that hold both endpoints, that cut one off by
        // an ulp, and that are random.
        let bounds = [
            (low, high),
            (low.next_up(), high),
            (low, high.next_down()),
            (magnitude_value(&mut rng), magnitude_value(&mut rng)),
        ];
        for bound in bounds {
            let expected = walked_inside(ramp, bound, EFFECT_WINDOW);
            assert_eq!(
                ramp_path_inside(ramp, bound, EFFECT_WINDOW),
                expected,
                "{context} within {bound:?}"
            );
            refused_by_bounds += usize::from(!expected);
        }
    }
    assert!(refused_by_bounds > 0, "no restored ramp was refused");
}
