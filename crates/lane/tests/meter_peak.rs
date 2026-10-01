//! Gate G1 of issue #943: the banked sample-peak kernel is the scalar meter's peak, bit for bit.
//!
//! `meter_sample_peak_block` computes, per lane of one AoSoA block, the maximum of the carried
//! peak and every sanitized magnitude. Each vector width is held against two references, lane by
//! lane and by bits:
//!
//! 1. the same kernel at `Lane = f32` over the de-interleaved lane, which is the scalar `Lane`
//!    oracle of master plan §3.2; and
//! 2. an independent oracle written here, which is the builtin meter's own sample-serial loop:
//!    `normal_or_zero`, then `abs`, then `if a > p { a } else { p }`.
//!
//! The accumulator is carried across 64 blocks, so a sanitization or `max` that differed only once
//! the peak was non-zero would still be seen. The seeded-`+0.0` partial merged into the carried
//! peak with the same select form is checked against both as well: that merge is the whole of what
//! the graph hands a meter, and its exactness is the reassociation argument the kernel's doc makes.

#![allow(missing_docs)]

use lane::Lane;
use lane::kernels::builtins::meter_sample_peak_block;

/// Xorshift64\*: seeded and portable, so every host sees the same inputs.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next_u64() % n as u64) as usize
    }
}

/// `2^k` from the exponent field: exact on every host.
fn pow2(k: i32) -> f32 {
    f32::from_bits(((127 + k) as u32) << 23)
}

/// The brief's hostile pool: NaN, both infinities, both zeros, the smallest and largest
/// subnormal of both signs, `±MIN_POSITIVE`, `±1.0`, `±f32::MAX`, and `±2^k` for `k` in
/// `-34..=5`. Two extra NaN payloads make sure the kernel never forwards one.
fn hostile_pool() -> Vec<f32> {
    let mut pool = vec![
        f32::NAN,
        -f32::NAN,
        f32::from_bits(0x7FC0_0001),
        f32::from_bits(0xFF80_0001),
        f32::INFINITY,
        f32::NEG_INFINITY,
        0.0,
        -0.0,
        f32::from_bits(0x0000_0001),
        f32::from_bits(0x8000_0001),
        f32::from_bits(0x007F_FFFF),
        f32::from_bits(0x807F_FFFF),
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        1.0,
        -1.0,
        f32::MAX,
        -f32::MAX,
    ];
    for k in -34..=5 {
        pool.push(pow2(k));
        pool.push(-pow2(k));
    }
    pool
}

/// The builtin meter's sample-serial peak step (`builtins::MeterAccumulator`), written out here
/// rather than borrowed so this oracle does not share a line with the kernel.
fn oracle_step(peak: f32, x: f32) -> f32 {
    let sample = if x.is_finite() && !x.is_subnormal() {
        x
    } else {
        0.0
    };
    let a = sample.abs();
    if a > peak { a } else { peak }
}

fn store<L: Lane>(value: L) -> Vec<f32> {
    let mut out = vec![0.0_f32; L::WIDTH];
    value.store(&mut out);
    out
}

/// Fills one AoSoA block: two thirds from the hostile pool, the rest a small tone whose amplitude
/// differs per lane so the lanes' peaks differ.
fn fill(rng: &mut Rng, pool: &[f32], words: &mut [f32], width: usize, block: usize) {
    for (index, word) in words.iter_mut().enumerate() {
        *word = if rng.below(3) == 0 {
            let lane = (index % width) as f32;
            let phase = (index / width + block * 7) as f32;
            (0.125 + 0.0625 * lane) * (((phase * 0.37) % 2.0) - 1.0)
        } else {
            pool[rng.below(pool.len())]
        };
    }
}

/// Returns the number of lane-blocks compared.
fn identity_at<L: Lane>(seed: u64) -> usize {
    let pool = hostile_pool();
    let width = L::WIDTH;
    let mut rng = Rng(seed);
    let mut compared = 0;
    for frames in [1_usize, 2, 3, 127, 128, 129] {
        let mut carried = L::zero();
        let mut scalar = vec![0.0_f32; width];
        let mut oracle = vec![0.0_f32; width];
        let mut merged = vec![0.0_f32; width];
        for block in 0..64 {
            let mut words = vec![0.0_f32; frames * width];
            fill(&mut rng, &pool, &mut words, width, block);
            carried = meter_sample_peak_block::<L>(&words, frames, carried);
            let partial = store(meter_sample_peak_block::<L>(&words, frames, L::zero()));
            let vector = store(carried);
            for lane in 0..width {
                let deinterleaved: Vec<f32> = (0..frames)
                    .map(|frame| words[frame * width + lane])
                    .collect();
                scalar[lane] = meter_sample_peak_block::<f32>(&deinterleaved, frames, scalar[lane]);
                oracle[lane] = deinterleaved
                    .iter()
                    .fold(oracle[lane], |p, &x| oracle_step(p, x));
                merged[lane] = if partial[lane] > merged[lane] {
                    partial[lane]
                } else {
                    merged[lane]
                };
                let context = format!("width {width} frames {frames} block {block} lane {lane}");
                assert_eq!(
                    vector[lane].to_bits(),
                    scalar[lane].to_bits(),
                    "{context}: the vector kernel against the f32 kernel"
                );
                assert_eq!(
                    vector[lane].to_bits(),
                    oracle[lane].to_bits(),
                    "{context}: the vector kernel against the meter's serial loop"
                );
                assert_eq!(
                    merged[lane].to_bits(),
                    oracle[lane].to_bits(),
                    "{context}: the seeded-zero partial merged with the select form"
                );
                compared += 1;
            }
        }
        assert!(
            oracle.iter().any(|peak| *peak != 0.0),
            "frames {frames}: the carried peaks must carry signal"
        );
    }
    compared
}

#[test]
fn g1_banked_sample_peak_is_the_scalar_meters_peak_at_every_width() {
    // Every width this build has, each with its own seed.
    lane::each_lane!(|L, N| {
        let seed = match N {
            1 => 0xDEAD_BEEF_CAFE_F00D,
            4 => 0x1234_5678_9ABC_DEF1,
            _ => 0x9E37_79B9_7F4A_7C15,
        };
        assert_eq!(identity_at::<L>(seed), 6 * 64 * N);
    });
}

/// A lane fed only invalid values -- NaN, both infinities, both zeros and subnormals of both
/// signs -- ends at exactly `+0.0`, while its neighbours carry a peak.
fn invalid_lane_at<L: Lane>() {
    let invalid = [
        f32::NAN,
        f32::from_bits(0xFFC0_1234),
        f32::INFINITY,
        f32::NEG_INFINITY,
        -0.0,
        0.0,
        f32::from_bits(0x0000_0001),
        f32::from_bits(0x807F_FFFF),
    ];
    let width = L::WIDTH;
    for frames in [1_usize, 2, 3, 127, 128, 129] {
        let mut words = vec![0.0_f32; frames * width];
        for (index, word) in words.iter_mut().enumerate() {
            let lane = index % width;
            *word = if lane == 1 {
                invalid[(index / width) % invalid.len()]
            } else {
                pow2(-(lane as i32))
            };
        }
        let peak = store(meter_sample_peak_block::<L>(&words, frames, L::zero()));
        for (lane, value) in peak.iter().enumerate() {
            let expected = if lane == 1 {
                0x0000_0000
            } else {
                pow2(-(lane as i32)).to_bits()
            };
            assert_eq!(value.to_bits(), expected, "frames {frames} lane {lane}");
        }
    }
    // Every lane at once, which is the only form the `f32` width can take.
    let mut only_invalid = vec![0.0_f32; 129 * width];
    for (index, word) in only_invalid.iter_mut().enumerate() {
        *word = invalid[index % invalid.len()];
    }
    for value in store(meter_sample_peak_block::<L>(&only_invalid, 129, L::zero())) {
        assert_eq!(value.to_bits(), 0x0000_0000);
    }
}

#[test]
fn g1_a_lane_fed_only_invalid_values_ends_at_positive_zero() {
    lane::each_lane!(|L| invalid_lane_at::<L>());
}

/// The sanitization boundaries, swept on the bits: every exponent of both signs, with the
/// mantissas that sit on each boundary. At each width the kernel over a one-frame block must equal
/// the oracle step from `+0.0`.
fn boundary_sweep_at<L: Lane>() {
    let mantissas = [0, 1, 2, 0x0040_0000, 0x007F_FFFE, 0x007F_FFFF];
    let mut values = Vec::new();
    for sign in [0_u32, 0x8000_0000] {
        for exponent in 0_u32..=255 {
            for mantissa in mantissas {
                values.push(f32::from_bits(sign | (exponent << 23) | mantissa));
            }
        }
    }
    while values.len() % L::WIDTH != 0 {
        values.push(0.0);
    }
    for chunk in values.chunks_exact(L::WIDTH) {
        let peak = store(meter_sample_peak_block::<L>(chunk, 1, L::zero()));
        for (lane, x) in chunk.iter().enumerate() {
            assert_eq!(
                peak[lane].to_bits(),
                oracle_step(0.0, *x).to_bits(),
                "width {} input {:#010x}",
                L::WIDTH,
                x.to_bits()
            );
        }
    }
}

#[test]
fn g1_sanitization_boundaries_match_normal_or_zero() {
    lane::each_lane!(|L| boundary_sweep_at::<L>());
}

/// The domain argument, witnessed. On the sanitized domain `{+0.0} ∪ [MIN_POSITIVE, MAX]` the D8
/// select form is commutative and associative by bits, which is why the kernel's
/// `L::max(c, peak)` and the expected-green mutation `L::max(peak, c)` agree, and why a
/// seeded-zero partial merged into the window equals the serial loop. Off the domain it is not:
/// `-0.0` and NaN break it, which is what the sanitization is for.
#[test]
fn g1_select_max_is_order_free_only_on_the_sanitized_domain() {
    let select_max = |a: f32, b: f32| if a > b { a } else { b };
    let mut domain: Vec<f32> = hostile_pool()
        .into_iter()
        .map(|x| oracle_step(0.0, x))
        .collect();
    domain.sort_by(f32::total_cmp);
    domain.dedup_by(|a, b| a.to_bits() == b.to_bits());
    assert_eq!(domain[0].to_bits(), 0, "+0.0 is in the domain");
    for &a in &domain {
        assert!(a.to_bits() == 0 || (f32::MIN_POSITIVE..=f32::MAX).contains(&a));
        for &b in &domain {
            assert_eq!(select_max(a, b).to_bits(), select_max(b, a).to_bits());
            assert_eq!(
                Lane::max(a, b).to_bits(),
                Lane::max(b, a).to_bits(),
                "Lane::max(c, peak) and Lane::max(peak, c) agree on the domain"
            );
            for &c in &domain {
                assert_eq!(
                    select_max(select_max(a, b), c).to_bits(),
                    select_max(a, select_max(b, c)).to_bits()
                );
            }
        }
    }
    assert_ne!(
        select_max(0.0, -0.0).to_bits(),
        select_max(-0.0, 0.0).to_bits()
    );
    assert_ne!(
        select_max(1.0, f32::NAN).to_bits(),
        select_max(f32::NAN, 1.0).to_bits()
    );
}
