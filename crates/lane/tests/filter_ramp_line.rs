//! Issue #1407 attempt 2: every input-filter ramp word lies on the line that ends at its target.
//!
//! Both filter-ramp bodies (`input_chain_ramp_block_filter`, dual, and
//! `input_chain_ramp_block_filter_mono`, the collapsed one) advance each of a section's six
//! coefficient words after a frame: the first four words of a ramp step from the word it started
//! at, `current + step`, and every later word is `target - step * remaining`, computed from the
//! ramp's target, its step and the countdown left after that frame -- never by adding the step to
//! the previous word again. A lane whose target is the disabled
//! identity (rule 2 of the live retarget law) holds its recursion words `c1`, `a2`, `a3` at their
//! current words until the completion snap. At countdown zero every word is the target.
//!
//! The oracle is the scalar `f32` expression itself, evaluated per lane with the same two
//! roundings (`mul`, then `sub`), so the check is bit for bit at every width this build has.
//!
//! Mutation evidence is in `.github/ISSUE_SPECS/1407-*.md` (attempt 2).

mod support;

use lane::Lane;
use lane::kernels::SvfCoef;
use lane::kernels::builtins::{
    INPUT_FILTER_LEADING_UPDATES, InputChainCoef, InputChainState, InputTrimRamp,
    input_chain_ramp_block_filter, input_chain_ramp_block_filter_mono,
};
use support::Xorshift64Star;

/// Random ramps per width and body. Every one renders its whole countdown frame by frame.
const CASES: usize = if cfg!(debug_assertions) { 200 } else { 4_000 };

const IDENTITY: [f32; 6] = [0.0, 0.0, 0.0, 1.0, 0.0, 0.0];

/// One lane's ramp: the words it started from, the words it holds now, its target, its step and
/// its countdown.
#[derive(Clone, Copy)]
struct LaneRamp {
    start: [f32; 6],
    current: [f32; 6],
    target: [f32; 6],
    step: [f32; 6],
    countdown: u32,
    disabling: bool,
}

fn unit(rng: &mut Xorshift64Star) -> f32 {
    (rng.next_u32() >> 8) as f32 * (1.0 / 16_777_216.0)
}

/// A word the live retarget law can hold: a recursion word in `[0, 1)` or a mix word in
/// `[-1.5, 1.5)`, sometimes exactly equal to its target so the step is `+0.0`.
fn random_ramp(rng: &mut Xorshift64Star) -> LaneRamp {
    let word = |rng: &mut Xorshift64Star, index: usize| {
        if index < 3 {
            unit(rng)
        } else {
            unit(rng) * 3.0 - 1.5
        }
    };
    let start: [f32; 6] = core::array::from_fn(|index| word(rng, index));
    let disabling = rng.next_u32().is_multiple_of(4);
    let target: [f32; 6] = if disabling {
        IDENTITY
    } else {
        core::array::from_fn(|index| {
            if rng.next_u32().is_multiple_of(8) {
                start[index]
            } else {
                word(rng, index)
            }
        })
    };
    // The step exactly as `InputStage::apply_prepared_filter` writes it: `+0.0` for a frozen
    // recursion word (rule 2) or a word already at its target, else `(target - current) / 64`.
    let step: [f32; 6] = core::array::from_fn(|index| {
        if (disabling && index < 3) || start[index].to_bits() == target[index].to_bits() {
            0.0
        } else {
            (target[index] - start[index]) * (1.0 / 64.0)
        }
    });
    // Mostly a ramp from its start; sometimes one already part-way along, its current words
    // where the law put them; sometimes a settled lane beside ramping ones.
    let countdown = match rng.next_u32() % 8 {
        0 => 0,
        1 | 2 => 1 + rng.next_u32() % 63,
        _ => 64,
    };
    let mut ramp = LaneRamp {
        start,
        current: start,
        target,
        step,
        countdown,
        disabling,
    };
    if (1..64).contains(&countdown) {
        ramp.current = core::array::from_fn(|index| expected(&ramp, index, i64::from(countdown)));
    }
    ramp
}

/// The word the law gives `ramp`'s word `index` after the frame that leaves `remaining`.
fn expected(ramp: &LaneRamp, index: usize, remaining: i64) -> f32 {
    if remaining <= 0 {
        ramp.target[index]
    } else if index < 3 && ramp.step[index] == 0.0 {
        // A recursion word with a zero step holds (rule 2's freeze).
        ramp.start[index]
    } else if remaining >= 60 {
        // The first four frames of a 64-update ramp step from the word it started at.
        let mut word = ramp.start[index];
        for _ in remaining..64 {
            word += ramp.step[index];
        }
        word
    } else {
        ramp.target[index] - ramp.step[index] * remaining as f32
    }
}

fn coef<L: Lane>(ramps: &[LaneRamp], pick: impl Fn(&LaneRamp) -> [f32; 6]) -> SvfCoef<L> {
    let word = |index: usize| {
        let values: Vec<f32> = ramps.iter().map(|ramp| pick(ramp)[index]).collect();
        L::load(&values)
    };
    SvfCoef {
        c1: word(0),
        a2: word(1),
        a3: word(2),
        m0: word(3),
        m1: word(4),
        m2: word(5),
    }
}

fn words<L: Lane>(coef: &SvfCoef<L>) -> [Vec<f32>; 6] {
    let read = |value: L| {
        let mut out = [0.0_f32; 8];
        value.store(&mut out);
        out[..L::WIDTH].to_vec()
    };
    [
        read(coef.c1),
        read(coef.a2),
        read(coef.a3),
        read(coef.m0),
        read(coef.m1),
        read(coef.m2),
    ]
}

/// One random bank of `[channel][section][lane]` ramps rendered frame by frame through one body,
/// every word checked after every frame.
fn check_body<L: Lane>(rng: &mut Xorshift64Star, mono: bool) {
    let name = core::any::type_name::<L>();
    let ramps: [[Vec<LaneRamp>; 2]; 2] = core::array::from_fn(|_| {
        core::array::from_fn(|_| (0..L::WIDTH).map(|_| random_ramp(rng)).collect())
    });
    let mut c = InputChainCoef {
        trim: [L::splat(1.0); 2],
        section: core::array::from_fn(|channel| {
            core::array::from_fn(|section| coef::<L>(&ramps[channel][section], |ramp| ramp.current))
        }),
        silence: L::splat(lane::silence_frames(48_000) as f32),
    };
    let target: [[SvfCoef<L>; 2]; 2] = core::array::from_fn(|channel| {
        core::array::from_fn(|section| coef::<L>(&ramps[channel][section], |ramp| ramp.target))
    });
    let step: [[SvfCoef<L>; 2]; 2] = core::array::from_fn(|channel| {
        core::array::from_fn(|section| coef::<L>(&ramps[channel][section], |ramp| ramp.step))
    });
    let mut remaining: [[L; 2]; 2] = core::array::from_fn(|channel| {
        core::array::from_fn(|section| {
            let counts: Vec<f32> = ramps[channel][section]
                .iter()
                .map(|ramp| ramp.countdown as f32)
                .collect();
            L::load(&counts)
        })
    });
    let mut state = InputChainState::<L>::default();
    let mut trim = InputTrimRamp {
        current: [L::splat(1.0); 2],
        target: [L::splat(1.0); 2],
        step: [L::zero(); 2],
        remaining: [L::zero(); 2],
    };
    let channels = if mono { 1 } else { 2 };
    for frame in 1..=65_i64 {
        // The owner's leading countdown, rebuilt from the ramp countdown before every block (this
        // test's blocks are one frame): the countdown less `64 - INPUT_FILTER_LEADING_UPDATES`.
        let floor = L::splat((64 - INPUT_FILTER_LEADING_UPDATES) as f32);
        let leading: [[L; 2]; 2] = core::array::from_fn(|channel| {
            core::array::from_fn(|section| remaining[channel][section].sub(floor))
        });
        let mut left = vec![0.25_f32; L::WIDTH];
        let mut right = vec![-0.25_f32; L::WIDTH];
        if mono {
            input_chain_ramp_block_filter_mono(
                &mut left,
                1,
                &mut c,
                &mut state,
                &mut trim,
                false,
                &target,
                &step,
                &mut remaining,
                leading,
            );
        } else {
            input_chain_ramp_block_filter(
                &mut left,
                &mut right,
                1,
                &mut c,
                &mut state,
                &mut trim,
                false,
                &target,
                &step,
                &mut remaining,
                leading,
            );
        }
        for (channel, (coefficients, channel_ramps)) in
            c.section.iter().zip(&ramps).take(channels).enumerate()
        {
            for (section, (coefficient, section_ramps)) in
                coefficients.iter().zip(channel_ramps).enumerate()
            {
                let actual = words::<L>(coefficient);
                for (lane, ramp) in section_ramps.iter().enumerate() {
                    let left_after = i64::from(ramp.countdown) - frame;
                    for (index, actual) in actual.iter().enumerate() {
                        let want = expected(ramp, index, left_after);
                        assert_eq!(
                            actual[lane].to_bits(),
                            want.to_bits(),
                            "{name} mono {mono}: channel {channel} section {section} lane {lane} \
                             word {index} after frame {frame} (countdown {}, disabling {}): \
                             {:e} != {want:e}",
                            ramp.countdown,
                            ramp.disabling,
                            actual[lane],
                        );
                    }
                }
            }
        }
    }
}

/// Both filter-ramp bodies, at every width: the first four words of a ramp step from its start,
/// each later word is `target - step * remaining`, a recursion word with a zero step holds, and
/// countdown zero is the target.
///
/// Red: revert either body's coefficient update to `current + step` on every frame (the
/// accumulated law #1407 attempt 1 left in place); compute every word from the target
/// (`target - step * remaining` from the first frame, whose rounding of nearly the whole distance
/// the proven bound cannot absorb at small block sizes); or drop the zero-step hold (rule 2's
/// frozen recursion then jumps to the identity's zeros).
#[test]
fn every_filter_ramp_word_lies_on_the_line_to_its_target() {
    lane::each_lane!(|L| {
        let mut rng = Xorshift64Star::new(0x1407_0002 ^ L::WIDTH as u64);
        for _ in 0..CASES {
            check_body::<L>(&mut rng, false);
            check_body::<L>(&mut rng, true);
        }
    });
}
