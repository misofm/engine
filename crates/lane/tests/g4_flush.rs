//! Gate G4: the flush law.
//!
//! Master plan #83 D7 and §3.6. `flush(x)` maps every `|x| < FLUSH_EPS` — which includes every
//! subnormal and `-0.0` — to exactly `+0.0`, leaves every `|x| >= FLUSH_EPS` unchanged bit for
//! bit, and passes NaN and infinity through. The same law holds at every width, since `flush` is
//! written once, generic over `Lane`.
//!
//! Red-mutation proven for this gate (see `tests/MUTATIONS.md`): `lt` becomes `le` in `flush`,
//! which fails at `x = +-FLUSH_EPS`.
//!
//! The pair law (issue #1328, amendment A9): `flush_pair(n1, n2, rest)` applies the per-word law
//! to each word and, when the rest threshold is armed (`REST_EPS`) *and* both magnitudes are below
//! it, zeroes the pair together. An unarmed threshold (`+0.0`) or one word at or above `REST_EPS`
//! leaves both words on the per-word law bit for bit; a NaN in either word passes through; `-0.0`
//! becomes `+0.0`. The threshold is armed only by the silence counter (`silence_step`): an effect
//! input exactly zero (either sign) for `N_SILENCE` frames in a row (`silence_frames` at the
//! effect's rate); a subnormal, a NaN, an
//! infinity or any other non-zero input resets it. Mutation evidence is in the issue's attempt
//! record.

mod support;

use lane::kernels::{silence_advance, silence_block};
use lane::{FLUSH_EPS, Lane, REST_EPS, flush, flush_pair, silence_frames, silence_step};
use support::Xorshift64Star;

/// Step through the subnormal range. The `--release` run is exhaustive (every one of the 2^23
/// subnormals of each sign); a debug run strides, to keep the workspace suite quick.
const SUBNORMAL_STRIDE: u32 = if cfg!(debug_assertions) { 1_021 } else { 1 };

/// Random normals swept per width.
const RANDOM_NORMALS: usize = if cfg!(debug_assertions) {
    20_000
} else {
    1_000_000
};

/// Applies `flush` at one width and returns the result bits of lane 0.
fn flush_bits<L: Lane>(value: f32) -> u32 {
    let mut bits = [0u32; 8];
    flush(L::splat(value)).store_bits(&mut bits);
    bits[0]
}

/// Asserts the flush law for one value at one width.
fn check<L: Lane>(width_name: &str, value: f32) {
    let actual = flush_bits::<L>(value);
    let magnitude = f32::from_bits(value.to_bits() & 0x7FFF_FFFF);
    if magnitude < FLUSH_EPS {
        assert_eq!(
            actual, 0x0000_0000,
            "{width_name}: flush({value:e}) must be +0.0, got {actual:#010x}"
        );
    } else {
        assert_eq!(
            actual,
            value.to_bits(),
            "{width_name}: flush({value:e}) must be unchanged"
        );
    }
}

/// Sweeps one width over the whole corpus.
fn sweep<L: Lane>(width_name: &str) {
    for value in [
        0.0f32,
        -0.0,
        FLUSH_EPS,
        -FLUSH_EPS,
        f32::from_bits(FLUSH_EPS.to_bits() + 1),
        f32::from_bits(FLUSH_EPS.to_bits() - 1),
        f32::from_bits((FLUSH_EPS.to_bits() + 1) | 0x8000_0000),
        f32::from_bits((FLUSH_EPS.to_bits() - 1) | 0x8000_0000),
        f32::MIN_POSITIVE,
        -f32::MIN_POSITIVE,
        1.0,
        -1.0,
        f32::MAX,
        f32::MIN,
    ] {
        check::<L>(width_name, value);
    }

    // Infinity and NaN pass through untouched: `abs(NaN) < eps` is false under the ordered
    // comparison, so a NaN reaches the once-per-block boundary check instead of being hidden here.
    for value in [f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(
            flush_bits::<L>(value),
            value.to_bits(),
            "{width_name}: flush({value:e}) must pass infinity through"
        );
    }
    for bits in [0x7FC0_0000u32, 0xFFC0_0001, 0x7F80_0001] {
        assert!(
            f32::from_bits(flush_bits::<L>(f32::from_bits(bits))).is_nan(),
            "{width_name}: flush of NaN {bits:#010x} must stay NaN"
        );
    }

    let mut bits = 1u32;
    while bits < 0x0080_0000 {
        check::<L>(width_name, f32::from_bits(bits));
        check::<L>(width_name, f32::from_bits(bits | 0x8000_0000));
        bits += SUBNORMAL_STRIDE;
    }

    let mut random = Xorshift64Star::new(0x0F1E_2D3C_4B5A_6978);
    for _ in 0..RANDOM_NORMALS {
        let value = random.next_moderate();
        check::<L>(width_name, value);
    }
}

#[test]
fn g4_flush_law_holds_at_every_width() {
    lane::each_lane!(|L| sweep::<L>(core::any::type_name::<L>()));
}

#[test]
fn g4_flush_is_lane_wise() {
    // Mixed lanes: only the lanes below the threshold are cleared, and the others keep their bits.
    let lanes = [
        1.0f32,
        1.0e-30,
        -1.0e-30,
        -2.0,
        f32::from_bits(1),
        FLUSH_EPS,
        -FLUSH_EPS,
        0.5,
    ];
    let mut bits = [0u32; 8];
    // At the build's own width, one vector per `WIDTH` lanes: eight lanes in one where `avx2` is
    // enabled, two of four in a 4-lane (NEON/simd128) build (issue #1112).
    let width = <lane::Native as Lane>::WIDTH;
    for (input, output) in lanes.chunks(width).zip(bits.chunks_mut(width)) {
        flush(lane::Native::load(input)).store_bits(output);
    }
    let expected = [
        1.0f32.to_bits(),
        0,
        0,
        (-2.0f32).to_bits(),
        0,
        FLUSH_EPS.to_bits(),
        (-FLUSH_EPS).to_bits(),
        0.5f32.to_bits(),
    ];
    assert_eq!(bits, expected, "G4: flush must act lane by lane");
}

/// The pair law restated on plain `f32` comparisons, independent of `Lane`: the oracle the vector
/// widths are held to bit for bit. `armed` is the silence counter's verdict for the frame.
fn pair_oracle(n1: f32, n2: f32, armed: bool) -> (u32, u32) {
    let rest = armed && n1.abs() < REST_EPS && n2.abs() < REST_EPS;
    let word = |n: f32| {
        if rest || n.abs() < FLUSH_EPS {
            0
        } else {
            n.to_bits()
        }
    };
    (word(n1), word(n2))
}

/// The two thresholds [`silence_step`] writes: armed, and not.
fn threshold(armed: bool) -> f32 {
    if armed { REST_EPS } else { 0.0 }
}

/// Runs `flush_pair` lane-wise at one width over `(n1, n2, armed)` triples, `L::WIDTH` per vector.
fn pair_bits<L: Lane>(pairs: &[(f32, f32, bool)]) -> Vec<(u32, u32)> {
    let mut out = Vec::with_capacity(pairs.len());
    for chunk in pairs.chunks(L::WIDTH) {
        let mut first = vec![0.0f32; L::WIDTH];
        let mut second = vec![0.0f32; L::WIDTH];
        let mut rest = vec![0.0f32; L::WIDTH];
        for (lane, &(n1, n2, armed)) in chunk.iter().enumerate() {
            first[lane] = n1;
            second[lane] = n2;
            rest[lane] = threshold(armed);
        }
        let (ic1, ic2) = flush_pair(L::load(&first), L::load(&second), L::load(&rest));
        let (mut bits1, mut bits2) = ([0u32; 8], [0u32; 8]);
        ic1.store_bits(&mut bits1);
        ic2.store_bits(&mut bits2);
        out.extend((0..chunk.len()).map(|lane| (bits1[lane], bits2[lane])));
    }
    out
}

/// Magnitudes around both thresholds, with both signs and both zeros.
fn pair_edges() -> Vec<f32> {
    let mut edges = Vec::new();
    for magnitude in [
        0.0f32,
        f32::from_bits(1),
        f32::MIN_POSITIVE,
        f32::from_bits(FLUSH_EPS.to_bits() - 1),
        FLUSH_EPS,
        f32::from_bits(FLUSH_EPS.to_bits() + 1),
        1.0e-17,
        f32::from_bits(REST_EPS.to_bits() - 1),
        REST_EPS,
        f32::from_bits(REST_EPS.to_bits() + 1),
        5.0e-15,
        1.0e-6,
        1.0,
        f32::MAX,
        f32::INFINITY,
        f32::NAN,
        f32::from_bits(0x7F80_0001),
    ] {
        edges.push(magnitude);
        edges.push(-magnitude);
    }
    edges
}

/// One random word whose exponent straddles both thresholds (`2^-90 .. 2^-30`), or, one time in
/// eight, an arbitrary bit pattern (NaN, infinity, subnormal, huge).
fn pair_word(random: &mut Xorshift64Star) -> f32 {
    let bits = random.next_u32();
    if bits & 7 == 0 {
        return random.next_bit_pattern();
    }
    let exponent = ((bits >> 3) % 60) + 127 - 90;
    f32::from_bits((bits & 0x8000_0000) | (exponent << 23) | (random.next_u32() & 0x007F_FFFF))
}

fn pair_sweep<L: Lane>(width_name: &str) {
    let edges = pair_edges();
    let mut pairs: Vec<(f32, f32, bool)> = edges
        .iter()
        .flat_map(|&n1| edges.iter().map(move |&n2| (n1, n2)))
        .flat_map(|(n1, n2)| [false, true].map(|armed| (n1, n2, armed)))
        .collect();
    let mut random = Xorshift64Star::new(0x1328_FA17_0000_0001);
    pairs.extend((0..RANDOM_NORMALS).map(|_| {
        (
            pair_word(&mut random),
            pair_word(&mut random),
            random.next_u32() & 1 == 0,
        )
    }));
    let actual = pair_bits::<L>(&pairs);
    for (&(n1, n2, armed), &(ic1, ic2)) in pairs.iter().zip(&actual) {
        assert_eq!(
            (ic1, ic2),
            pair_oracle(n1, n2, armed),
            "{width_name}: flush_pair({n1:e}, {n2:e}, {}) breaks the pair law",
            threshold(armed)
        );
    }
}

#[test]
fn g4_pair_law_holds_at_every_width() {
    lane::each_lane!(|L| pair_sweep::<L>(core::any::type_name::<L>()));
}

fn pair_cases<L: Lane>(width_name: &str) {
    let below = f32::from_bits(REST_EPS.to_bits() - 1);
    let small_pairs = [(below, -below), (-5.0e-15, 2.0e-20), (1.35e-16, -2.0e-21)];
    // Both words below `REST_EPS`, each above `FLUSH_EPS`, armed: the per-word law alone keeps
    // them, the pair rule zeroes both.
    for (n1, n2) in small_pairs {
        assert_eq!(
            pair_bits::<L>(&[(n1, n2, true)]),
            [(0, 0)],
            "{width_name}: both below REST_EPS, armed, must both become +0.0"
        );
    }
    // The same pairs unarmed -- an effect input that is live, or silent for fewer than
    // `N_SILENCE` frames -- keep the per-word law bit for bit (amendment A9): a section applies
    // its whole response to whatever its effect is given.
    for (n1, n2) in small_pairs {
        let expected = (flush(n1).to_bits(), flush(n2).to_bits());
        assert_eq!(
            pair_bits::<L>(&[(n1, n2, false)]),
            [expected],
            "{width_name}: flush_pair({n1:e}, {n2:e}) unarmed must follow the per-word law"
        );
    }
    // One word at or above `REST_EPS`, armed: each word keeps the per-word law, so a small partner
    // word is kept and a word below `FLUSH_EPS` is still flushed.
    for (n1, n2) in [
        (REST_EPS, below),
        (-below, -REST_EPS),
        (0.5, 5.0e-15),
        (6.0e-20, 0.25),
        (1.0, 1.0e-21),
        (f32::INFINITY, 1.0e-16),
    ] {
        let expected = (flush(n1).to_bits(), flush(n2).to_bits());
        assert_eq!(
            pair_bits::<L>(&[(n1, n2, true)]),
            [expected],
            "{width_name}: flush_pair({n1:e}, {n2:e}) armed must follow the per-word law"
        );
    }
    // NaN in either word passes through both rules, armed or not, and its partner follows the
    // per-word law: `max_u32` of a NaN magnitude is a NaN, which no ordered compare admits.
    for nan in [
        f32::NAN,
        f32::from_bits(0xFFC0_0001),
        f32::from_bits(0x7F80_0001),
    ] {
        for partner in [0.0f32, 1.0e-21, 5.0e-15, 0.5] {
            for armed in [false, true] {
                let [(ic1, ic2)] = pair_bits::<L>(&[(nan, partner, armed)])[..] else {
                    unreachable!()
                };
                assert_eq!(
                    ic1,
                    nan.to_bits(),
                    "{width_name}: a NaN first word must pass"
                );
                assert_eq!(ic2, flush(partner).to_bits(), "{width_name}: NaN's partner");
                let [(ic1, ic2)] = pair_bits::<L>(&[(partner, nan, armed)])[..] else {
                    unreachable!()
                };
                assert_eq!(
                    ic2,
                    nan.to_bits(),
                    "{width_name}: a NaN second word must pass"
                );
                assert_eq!(ic1, flush(partner).to_bits(), "{width_name}: NaN's partner");
            }
        }
    }
    // `-0.0` becomes `+0.0` in either position, alone or beside a kept word, armed or not.
    for armed in [false, true] {
        for (n1, n2) in [(-0.0f32, -0.0f32), (-0.0, 0.5), (0.5, -0.0)] {
            let [(ic1, ic2)] = pair_bits::<L>(&[(n1, n2, armed)])[..] else {
                unreachable!()
            };
            assert_eq!(
                ic1,
                if n1 == 0.5 { n1.to_bits() } else { 0 },
                "{width_name}: -0.0"
            );
            assert_eq!(
                ic2,
                if n2 == 0.5 { n2.to_bits() } else { 0 },
                "{width_name}: -0.0"
            );
        }
    }
}

#[test]
fn g4_pair_law_cases_at_every_width() {
    lane::each_lane!(|L| pair_cases::<L>(core::any::type_name::<L>()));
}

/// The silence counter's integer run, restated without `Lane`: `run` counts consecutive inputs
/// equal to zero (IEEE `==`, both signs), saturating at `2^24`; any other input -- a subnormal, a
/// NaN, an infinity -- resets it. The frame is armed once `run >= armed_after`.
fn counter_oracle(run: u32, x: f32, armed_after: u32) -> (u32, bool) {
    let next = if x == 0.0 { (run + 1).min(1 << 24) } else { 0 };
    (next, next >= armed_after)
}

/// Inputs the counter must tell apart: both zeros count, everything else resets.
const COUNTER_INPUTS: [f32; 10] = [
    0.0,
    -0.0,
    f32::from_bits(1),
    f32::from_bits(0x8000_0001),
    f32::MIN_POSITIVE,
    1.0e-30,
    -0.5,
    f32::INFINITY,
    f32::NAN,
    f32::from_bits(0xFFC0_0001),
];

fn counter_law<L: Lane>(width_name: &str, armed_after: u32) {
    let n_lanes = L::splat(armed_after as f32);
    // Lane-wise schedules: lane `l` sees a zero run with a reset every `period(l)` frames, so the
    // lanes cross `N_SILENCE` on different frames and a counter shared across lanes is caught.
    let period = |lane: usize| {
        let n = armed_after as usize;
        if lane % 3 == 2 {
            n - 3
        } else {
            n + 50 + 3 * lane + usize::from(lane % 2 == 1) * 400
        }
    };
    let frames = 3 * armed_after as usize;
    let mut run = L::zero();
    let mut oracle = [0u32; 8];
    let mut seen = [0usize; 2];
    for frame in 0..frames {
        let mut input = [0.0f32; 8];
        for (lane, word) in input.iter_mut().enumerate().take(L::WIDTH) {
            if frame % period(lane) == period(lane) - 1 {
                *word = COUNTER_INPUTS[2 + (frame + lane) % (COUNTER_INPUTS.len() - 2)];
            } else if (frame + lane) % 5 == 0 {
                *word = -0.0;
            }
        }
        let rest = silence_step(L::load(&input[..L::WIDTH]), &mut run, n_lanes);
        let (mut run_bits, mut rest_bits) = ([0u32; 8], [0u32; 8]);
        run.store_bits(&mut run_bits);
        rest.store_bits(&mut rest_bits);
        for lane in 0..L::WIDTH {
            let (next, armed) = counter_oracle(oracle[lane], input[lane], armed_after);
            oracle[lane] = next;
            seen[usize::from(armed)] += 1;
            assert_eq!(
                run_bits[lane],
                (next as f32).to_bits(),
                "{width_name}: frame {frame}, lane {lane}: run after input {:e}",
                input[lane]
            );
            assert_eq!(
                rest_bits[lane],
                threshold(armed).to_bits(),
                "{width_name}: frame {frame}, lane {lane}: threshold at run {next}"
            );
        }
    }
    assert!(
        seen[0] > 0 && seen[1] > 0,
        "{width_name}: both thresholds must occur"
    );
    // Every class of input from a long run: both zeros extend it, everything else resets it.
    for x in COUNTER_INPUTS {
        let mut run = L::splat(armed_after as f32 + 7.0);
        let rest = silence_step(L::splat(x), &mut run, n_lanes);
        let (next, armed) = counter_oracle(armed_after + 7, x, armed_after);
        let (mut run_bits, mut rest_bits) = ([0u32; 8], [0u32; 8]);
        run.store_bits(&mut run_bits);
        rest.store_bits(&mut rest_bits);
        assert_eq!(
            run_bits[0],
            (next as f32).to_bits(),
            "{width_name}: run on {x:e}"
        );
        assert_eq!(
            rest_bits[0],
            threshold(armed).to_bits(),
            "{width_name}: threshold on {x:e}"
        );
    }
    // Saturation: the `f32` counter stops at `2^24` and never wraps or disarms.
    let mut run = L::splat(16_777_214.0);
    for expected in [16_777_215.0f32, 16_777_216.0, 16_777_216.0, 16_777_216.0] {
        let rest = silence_step(L::zero(), &mut run, n_lanes);
        let (mut run_bits, mut rest_bits) = ([0u32; 8], [0u32; 8]);
        run.store_bits(&mut run_bits);
        rest.store_bits(&mut rest_bits);
        assert_eq!(run_bits[0], expected.to_bits(), "{width_name}: saturation");
        assert_eq!(
            rest_bits[0],
            REST_EPS.to_bits(),
            "{width_name}: armed past N"
        );
    }
}

/// Gate G4 (issue #1328, amendment A9): the silence counter counts exactly-zero effect inputs,
/// resets on every other input, arms the threshold at `N_SILENCE` and saturates at `2^24`, lane by
/// lane, at every width, at every launch rate's `N_SILENCE` and at a short one.
#[test]
fn g4_silence_counter_law_at_every_width() {
    for armed_after in [16, 44_100, 48_000, 88_200, 96_000]
        .map(|rate| if rate == 16 { 16 } else { silence_frames(rate) })
    {
        lane::each_lane!(|L| counter_law::<L>(core::any::type_name::<L>(), armed_after));
    }
}

/// `N_SILENCE` per launch rate: the silence time, `4096 / 48000` s, in frames, rounded up.
#[test]
fn g4_silence_frames_are_the_silence_time_at_each_rate() {
    assert_eq!(
        [44_100, 48_000, 88_200, 96_000].map(silence_frames),
        [3_764, 4_096, 7_527, 8_192]
    );
}

fn advance_matches_blocks<L: Lane>(width_name: &str) {
    for start in [
        0.0f32,
        1.0,
        1022.0,
        1023.0,
        1024.0,
        16_777_000.0,
        16_777_216.0,
    ] {
        for frames in [1usize, 2, 127, 128, 1000, 4096] {
            let input = vec![0.0f32; frames * L::WIDTH];
            let mut plane = vec![0.0f32; frames * L::WIDTH];
            let mut walked = L::splat(start);
            silence_block::<L>(&input, frames, &mut walked, &mut plane, L::splat(4_096.0));
            let mut jumped = L::splat(start);
            silence_advance::<L>(&mut jumped, frames);
            let (mut a, mut b) = ([0u32; 8], [0u32; 8]);
            walked.store_bits(&mut a);
            jumped.store_bits(&mut b);
            assert_eq!(
                a, b,
                "{width_name}: silence_advance from {start} over {frames} frames"
            );
        }
    }
}

/// `silence_advance` (the silent fast path's counter) is the frame-by-frame counter over an
/// all-zero block, saturation included.
#[test]
fn g4_silence_advance_is_the_counter_over_a_zero_block() {
    lane::each_lane!(|L| advance_matches_blocks::<L>(core::any::type_name::<L>()));
}

fn armable_is_exact<L: Lane>(width_name: &str) {
    for armed_after in [1_u32, 2, 3, 128, 3_764, 4_096] {
        for frames in [1_usize, 2, 3, 127, 128, 129] {
            for run in (0..armed_after + 3).chain([16_777_215, 16_777_216]) {
                // Some frame `f` of the block counts `run + f + 1` if every input is zero.
                let expected = (0..frames).any(|f| run as u64 + f as u64 + 1 >= armed_after as u64);
                assert_eq!(
                    lane::silence_armable(
                        L::splat(run as f32),
                        frames,
                        L::splat(armed_after as f32)
                    ),
                    expected,
                    "{width_name}: run {run}, {frames} frames, window {armed_after}"
                );
            }
        }
    }
}

/// `silence_armable` answers whether some frame of the block can carry an armed threshold, exactly:
/// a block whose last frame is the first that can arm is armable, and one frame earlier is not
/// (issue #1328, amendment A9). The kernels skip the joint rule's arithmetic on a block it says
/// cannot arm, so a test that erred late would delay an arming by a block.
#[test]
fn g4_silence_armable_is_exact() {
    lane::each_lane!(|L| armable_is_exact::<L>(core::any::type_name::<L>()));
}
