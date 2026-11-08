//! `RampLanes`, the one lane-vector walk of `LinearRamp`'s law (issue #2376).
//!
//! A bank whose frame loop reads a ramp every frame gathers its lanes' [`LinearRamp`]s into a
//! [`RampLanes`], advances it lane-wide, and scatters it back. The scalar ramp is the oracle:
//! every frame's value, every lane's in-flight bit, and every word a scatter writes back must be
//! the words [`LinearRamp::next_value`] leaves, bit for bit, at every width this build has, for a
//! walk split at any frame. The compressor, the transient shaper and the true-peak limiter render
//! through it, and so do the de-esser's and the dynamic EQ's live words (`WordRamp`, issue
//! #2390), so this file is where their lane walk is tested; mutation evidence is in the issues'
//! attempt records.

use effect_runtime::dynamic_band::{WordRamp, advance_word_ramps};
use effect_runtime::ramp::{LinearRamp, RampLanes};
use lane::Lane;

/// The widest bank: AVX2's eight lanes.
const LANES: usize = 8;

/// `xorshift64*` (Vigna 2016).
struct Rng(u64);

impl Rng {
    fn next_u32(&mut self) -> u32 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        (x.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 32) as u32
    }

    fn below(&mut self, bound: u32) -> u32 {
        self.next_u32() % bound
    }

    /// A finite value with magnitude in `[2^-20, 2^7]` and a random sign.
    fn value(&mut self) -> f32 {
        let bits = self.next_u32();
        let exponent = (bits >> 23) % 27 + 127 - 20;
        f32::from_bits((bits & 0x807F_FFFF) | (exponent << 23))
    }
}

/// Values a resting lane holds whose words an addition or a compare could disturb: both zeros, a
/// subnormal, a large word and a NaN.
const RESTING: [f32; 6] = [0.0, -0.0, 1.0e-40, -3.5e30, 0.25, f32::NAN];

/// One lane's ramp, in every phase a lane vector meets: at rest, armed by `set_target` over a
/// parameter window or a long link glide (past `u16::MAX`, up to `2^24`), restored mid-ramp with a
/// step `set_target` would not derive, or holding.
fn draw(rng: &mut Rng) -> LinearRamp {
    match rng.below(6) {
        0 => LinearRamp::fixed(RESTING[rng.below(5) as usize]),
        1 | 2 => {
            let mut ramp = LinearRamp::fixed(rng.value());
            ramp.set_target(rng.value(), rng.below(70) + 1);
            ramp
        }
        3 => {
            let mut ramp = LinearRamp::fixed(rng.value());
            ramp.set_target(rng.value(), (1 << 24) - rng.below(1 << 23));
            ramp
        }
        4 => LinearRamp::from_words(rng.value(), rng.value(), rng.value(), rng.below(70) + 1)
            .expect("a ramping ramp's words are accepted"),
        _ => LinearRamp::holding(rng.value(), rng.below(70)),
    }
}

fn words(ramp: &LinearRamp) -> [u32; 4] {
    [
        ramp.current().to_bits(),
        ramp.target().to_bits(),
        ramp.step().to_bits(),
        ramp.remaining(),
    ]
}

fn bits<L: Lane>(value: L) -> [u32; LANES] {
    let mut out = [0_u32; LANES];
    value.store_bits(&mut out[..L::WIDTH]);
    out
}

fn mask_lanes<L: Lane>(mask: L::Mask) -> [bool; LANES] {
    let mut out = [0.0_f32; LANES];
    L::select(mask, L::splat(1.0), L::zero()).store(&mut out[..L::WIDTH]);
    out.map(|word| word != 0.0)
}

fn mask_of<L: Lane>(lanes: &[bool; LANES]) -> L::Mask {
    L::load(&lanes.map(|on| if on { 1.0 } else { 0.0 })).gt(L::zero())
}

/// Every lane of a vector against its scalar ramp: `current`, the in-flight bit, and, scattered
/// into `scattered` (the ramps the walk was gathered from), all four words.
fn assert_lanes<L: Lane>(
    lanes: RampLanes<L>,
    expected: &[LinearRamp; LANES],
    scattered: &mut [LinearRamp; LANES],
    case: &str,
) {
    let current = bits(lanes.current());
    let flight = mask_lanes::<L>(lanes.in_flight());
    lanes.scatter(scattered.iter_mut());
    for lane in 0..L::WIDTH {
        assert_eq!(
            current[lane],
            expected[lane].current().to_bits(),
            "{case} lane {lane}: current"
        );
        assert_eq!(
            flight[lane],
            expected[lane].is_ramping(),
            "{case} lane {lane}: in flight"
        );
        assert_eq!(
            words(&scattered[lane]),
            words(&expected[lane]),
            "{case} lane {lane}: scatter"
        );
    }
}

/// Test value: red if the lane walk leaves the scalar law on any lane: a snap that adds the step
/// instead of assigning the target, a step left in force after the snap that a later frame reads,
/// a gated lane that moves (or an ungated in-flight lane missing from the returned mask), a
/// countdown narrowed below the longest link glide, or a gather or scatter that drops a word. No
/// other test checks the gated walk against the scalar oracle across random block splits.
#[test]
fn a_lane_vector_walks_the_scalar_law_word_for_word() {
    fn check<L: Lane>(rng: &mut Rng) {
        for trial in 0..2_000 {
            let mut gathered: [LinearRamp; LANES] = core::array::from_fn(|_| draw(rng));
            let mut expected = gathered;
            let mut lanes = RampLanes::<L>::gather(gathered.iter());
            let frames = rng.below(90);
            for frame in 0..frames {
                let case = format!("W{} trial {trial} frame {frame}", L::WIDTH);
                if rng.below(3) == 0 {
                    let gate: [bool; LANES] = core::array::from_fn(|_| rng.below(2) == 0);
                    let moved = mask_lanes::<L>(lanes.advance_where(mask_of::<L>(&gate)));
                    for lane in 0..L::WIDTH {
                        let in_flight = expected[lane].is_ramping();
                        assert_eq!(
                            moved[lane],
                            gate[lane] && in_flight,
                            "{case} lane {lane}: moved"
                        );
                        if gate[lane] {
                            let _ = expected[lane].next_value();
                        }
                    }
                } else {
                    let value = bits(lanes.advance());
                    for lane in 0..L::WIDTH {
                        assert_eq!(
                            value[lane],
                            expected[lane].next_value().to_bits(),
                            "{case} lane {lane}: value"
                        );
                    }
                }
                // A block boundary at a random frame: scatter, check, and gather again.
                if rng.below(8) == 0 {
                    assert_lanes(lanes, &expected, &mut gathered, &case);
                    lanes = RampLanes::<L>::gather(gathered.iter());
                }
            }
            let case = format!("W{} trial {trial} end", L::WIDTH);
            assert_lanes(lanes, &expected, &mut gathered, &case);
        }
    }
    let mut rng = Rng(0x2376_0001);
    lane::each_lane!(|L| check::<L>(&mut rng));
}

/// Test value: red if a lane at rest is not a fixed point of the walk -- an `advance` whose value
/// or words differ from [`RampLanes::current`] there (a snap that recomputes the target, a `-0.0`
/// or NaN rewritten by an addition), or a `resting` vector that is not the vector its fixed ramps
/// gather. The limiter's stationary dispatch reads `current` instead of advancing, and is exact
/// only because of this.
#[test]
fn a_lane_at_rest_advances_to_itself() {
    fn check<L: Lane>(offset: usize) {
        let values: [f32; LANES] =
            core::array::from_fn(|lane| RESTING[(lane + offset) % RESTING.len()]);
        let fixed = values.map(LinearRamp::fixed);
        let gathered = RampLanes::<L>::gather(fixed.iter());
        let resting = RampLanes::<L>::resting(L::load(&values));
        for (name, mut lanes) in [("gathered", gathered), ("resting", resting)] {
            let all = L::zero().eq(L::zero());
            for frame in 0..4 {
                let case = format!("W{} offset {offset} {name} frame {frame}", L::WIDTH);
                let before = bits(lanes.current());
                assert_eq!(bits(lanes.advance()), before, "{case}: advanced value");
                assert!(
                    !L::mask_any(lanes.advance_where(all)),
                    "{case}: a lane moved"
                );
                assert!(!L::mask_any(lanes.in_flight()), "{case}: in flight");
                let mut scattered = fixed;
                assert_lanes(lanes, &fixed, &mut scattered, &case);
            }
        }
    }
    // Every offset, so every width, the scalar one included, meets each resting value (NaN and
    // `-0.0` among them) on its lane 0.
    for offset in 0..RESTING.len() {
        lane::each_lane!(|L| check::<L>(offset));
    }
}

/// Test value: red if the dynamic band's live words leave the scalar law while they stay lane
/// words: `advance_word_ramps` gated on the wrong lanes, a word dropped or swapped on its way
/// through the shared walk, or a later ramp of the slice skipped. The de-esser and the dynamic EQ
/// keep these words across blocks and never scatter them, so the lane-vector test above cannot
/// see this path.
#[test]
fn a_word_ramp_walks_the_scalar_law_word_for_word() {
    const RAMPS: usize = 3;
    fn check<L: Lane>(rng: &mut Rng) {
        for trial in 0..500 {
            let mut expected: [[LinearRamp; LANES]; RAMPS] =
                core::array::from_fn(|_| core::array::from_fn(|_| draw(rng)));
            let load = |ramps: &[LinearRamp; LANES], word: fn(&LinearRamp) -> f32| {
                L::load(&ramps.each_ref().map(word))
            };
            let mut words: [WordRamp<L>; RAMPS] = core::array::from_fn(|ramp| WordRamp {
                current: load(&expected[ramp], LinearRamp::current),
                target: load(&expected[ramp], LinearRamp::target),
                step: load(&expected[ramp], LinearRamp::step),
                remaining: load(&expected[ramp], |ramp| ramp.remaining() as f32),
            });
            for frame in 0..rng.below(90) {
                advance_word_ramps(&mut words);
                for (ramp, (words, expected)) in words.iter().zip(&mut expected).enumerate() {
                    let got =
                        [words.current, words.target, words.step, words.remaining].map(bits::<L>);
                    for (lane, expected) in expected.iter_mut().enumerate().take(L::WIDTH) {
                        let _ = expected.next_value();
                        let want = words_of(expected);
                        let got = got.map(|word| word[lane]);
                        assert_eq!(
                            got,
                            want,
                            "W{} trial {trial} frame {frame} ramp {ramp} lane {lane}",
                            L::WIDTH
                        );
                    }
                }
            }
        }
    }
    /// The four words as a `WordRamp` holds them: the countdown as an `f32` integer.
    fn words_of(ramp: &LinearRamp) -> [u32; 4] {
        [
            ramp.current().to_bits(),
            ramp.target().to_bits(),
            ramp.step().to_bits(),
            (ramp.remaining() as f32).to_bits(),
        ]
    }
    let mut rng = Rng(0x2390_0001);
    lane::each_lane!(|L| check::<L>(&mut rng));
}
