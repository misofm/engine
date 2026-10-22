//! Gate support for a bank's per-lane rest claim (issue #1745 gate 1, and #1746 after it).
//!
//! The one driver each effect's randomized rest test runs, so the three (later six) tests share
//! one schedule, one check and one reach rule; each effect's test stays in its own crate and binds
//! its own bank through [`Probe`]. A dev-dependency of each effect crate; nothing on a render path
//! reaches it.
//!
//! The check is the claim itself, stated per lane: at every block after which
//! [`Probe::lanes_at_rest`] sets lane `l`, a copy of the bank renders one more block of exact
//! `+0.0` through its whole kernel, and lane `l`'s state words must come out bit-identical to the
//! original's and both of its output planes exact `+0.0`.

/// One bank under test, bound by the effect's own test.
pub trait Probe: Sized {
    /// Lanes in the bank, `L::WIDTH`; padded lanes included.
    fn lanes(&self) -> usize;
    /// Frames per block.
    fn frames(&self) -> usize;
    /// Renders one block in place. Each plane is `frames * lanes` words, frame-major.
    fn render(&mut self, left: &mut [f32], right: &mut [f32]);
    /// The bank's rest mask (`PreparedNativeEffectBank::lanes_at_rest`).
    fn lanes_at_rest(&self) -> u32;
    /// Appends the bits of every state word of `lane`, both channels, to `out`. The effect's
    /// clocks (words a `+0.0` block advances by its length alone) are left out, and the test
    /// names them.
    fn lane_state(&self, lane: usize, out: &mut Vec<u32>);
    /// A copy of this bank whose next block runs the whole kernel: any bank-wide silent fast
    /// path claim withdrawn, so the check renders rather than skips.
    fn fork(&self) -> Self;
}

/// The shape of one randomized case.
#[derive(Clone, Copy, Debug)]
pub struct Schedule {
    /// Blocks rendered.
    pub blocks: usize,
    /// The longest run of signal blocks a lane draws.
    pub longest_signal: usize,
    /// The longest run of silent blocks a lane draws: long enough for the effect to rest.
    pub longest_silence: usize,
}

/// What one or more cases reached.
#[derive(Clone, Copy, Debug, Default)]
pub struct Reach {
    /// Lane-blocks the bank claimed at rest, each one checked.
    pub claims: usize,
    /// Of those, claims on a lane that had carried signal earlier in its case.
    pub earned: usize,
    /// Lane-blocks whose input was exact `+0.0` on both planes and that the bank refused.
    pub quiet_refusals: usize,
}

impl Reach {
    /// Adds another case's reach.
    pub fn add(&mut self, other: Self) {
        self.claims += other.claims;
        self.earned += other.earned;
        self.quiet_refusals += other.quiet_refusals;
    }

    /// The test proves something only if it saw a lane come to rest after signal and a silent
    /// lane that was not yet at rest.
    ///
    /// # Panics
    ///
    /// Panics with `context` if either was never reached.
    pub fn assert_reached(&self, context: &str) {
        assert!(
            self.earned > 0,
            "{context}: no lane came to rest after signal ({self:?})"
        );
        assert!(
            self.quiet_refusals > 0,
            "{context}: no silent lane was refused rest ({self:?})"
        );
    }
}

/// SplitMix64: a frozen, dependency-free generator.
struct Draw(u64);

impl Draw {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound.max(1) as u64) as usize
    }

    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1_u64 << 24) as f32 * 2.0 - 1.0
    }
}

/// Signal levels a lane draws: subnormal, below every flush threshold, quiet, ordinary and
/// hot, so lanes reach rest from every kind of tail.
const LEVELS: [f32; 8] = [1.0e-40, 1.0e-20, 1.0e-9, 1.0e-4, 0.05, 0.5, 1.0, 4.0];

#[derive(Clone, Copy)]
enum Segment {
    /// Signal at a level, for the blocks left.
    Signal(f32, usize),
    /// Every word `-0.0`, which is not silence, for the blocks left.
    NegativeZero(usize),
    /// Exact `+0.0`, for the blocks left.
    Silence(usize),
}

/// Runs one randomized case on `probe` and checks every claim it makes.
///
/// Each lane draws its own schedule: runs of signal at a random level (with scattered `+0.0`
/// and `-0.0` frames), an occasional block of `-0.0`, and runs of exact silence; a quarter of
/// the lanes start silent.
///
/// # Panics
///
/// Panics when a claimed lane's state moves, or its output is not `+0.0`, on the checking
/// block, or when the mask names a lane past the bank.
pub fn check<P: Probe>(probe: &mut P, seed: u64, schedule: Schedule) -> Reach {
    let lanes = probe.lanes();
    let frames = probe.frames();
    let words = frames * lanes;
    let mut draw = Draw(seed);
    let mut segments: Vec<Segment> = (0..lanes)
        .map(|_| {
            if draw.below(4) == 0 {
                Segment::Silence(1 + draw.below(schedule.longest_silence))
            } else {
                Segment::Signal(LEVELS[draw.below(LEVELS.len())], 0)
            }
        })
        .collect();
    let mut carried = vec![false; lanes];
    let mut reach = Reach::default();
    let mut left = vec![0.0_f32; words];
    let mut right = vec![0.0_f32; words];
    let mut before = Vec::new();
    let mut after = Vec::new();
    for block in 0..schedule.blocks {
        let mut quiet = vec![true; lanes];
        for lane in 0..lanes {
            segments[lane] = match segments[lane] {
                Segment::Signal(_, 0) | Segment::NegativeZero(0) | Segment::Silence(0) => {
                    match draw.below(8) {
                        0..=3 => Segment::Silence(1 + draw.below(schedule.longest_silence)),
                        4 => Segment::NegativeZero(1),
                        _ => Segment::Signal(
                            LEVELS[draw.below(LEVELS.len())],
                            1 + draw.below(schedule.longest_signal),
                        ),
                    }
                }
                other => other,
            };
            let (fill, next) = match segments[lane] {
                Segment::Signal(level, remaining) => {
                    (Some(level), Segment::Signal(level, remaining - 1))
                }
                Segment::NegativeZero(remaining) => (None, Segment::NegativeZero(remaining - 1)),
                Segment::Silence(remaining) => (Some(0.0), Segment::Silence(remaining - 1)),
            };
            let negative = matches!(segments[lane], Segment::NegativeZero(_));
            segments[lane] = next;
            for frame in 0..frames {
                let index = frame * lanes + lane;
                let (l, r) = match fill {
                    _ if negative => (-0.0, -0.0),
                    Some(level) if level != 0.0 => match draw.below(16) {
                        0 => (0.0, 0.0),
                        1 => (-0.0, 0.0),
                        _ => (draw.unit() * level, draw.unit() * level),
                    },
                    _ => (0.0, 0.0),
                };
                left[index] = l;
                right[index] = r;
                if l.to_bits() != 0 || r.to_bits() != 0 {
                    quiet[lane] = false;
                }
            }
            carried[lane] |= !quiet[lane];
        }
        probe.render(&mut left, &mut right);
        let mask = probe.lanes_at_rest();
        assert_eq!(
            mask >> lanes,
            0,
            "block {block}: the rest mask {mask:#b} names a lane past the bank's {lanes}"
        );
        reach.quiet_refusals += quiet
            .iter()
            .enumerate()
            .filter(|&(lane, &silent)| silent && mask & (1 << lane) == 0)
            .count();
        if mask == 0 {
            continue;
        }
        let mut fork = probe.fork();
        let mut zero_left = vec![0.0_f32; words];
        let mut zero_right = vec![0.0_f32; words];
        fork.render(&mut zero_left, &mut zero_right);
        for lane in (0..lanes).filter(|lane| mask & (1 << lane) != 0) {
            reach.claims += 1;
            if carried[lane] {
                reach.earned += 1;
            }
            before.clear();
            after.clear();
            probe.lane_state(lane, &mut before);
            fork.lane_state(lane, &mut after);
            if let Some(word) =
                (0..before.len().max(after.len())).find(|&word| before.get(word) != after.get(word))
            {
                panic!(
                    "block {block}, lane {lane}: claimed at rest, but a block of +0.0 moved \
                     state word {word} from {:?} to {:?}",
                    before.get(word).copied().map(f32::from_bits),
                    after.get(word).copied().map(f32::from_bits),
                );
            }
            for frame in 0..frames {
                let index = frame * lanes + lane;
                assert!(
                    zero_left[index].to_bits() == 0 && zero_right[index].to_bits() == 0,
                    "block {block}, lane {lane}: claimed at rest, but a block of +0.0 wrote \
                     ({}, {}) at frame {frame}",
                    zero_left[index],
                    zero_right[index],
                );
            }
        }
    }
    reach
}
