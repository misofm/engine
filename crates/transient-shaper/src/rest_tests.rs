//! Issue #1746 gates 1 and 2: the transient-shaper bank's per-lane rest claim and its bank-wide
//! silent fast path, at `Simd4` and `Simd8`.
//!
//! Crate-internal for the reason `padding_tests` is: it binds the bank at every vector width the
//! build has, and it reads and copies envelope state of padded lanes, which no payload carries.
//!
//! The transient shaper has no clocks: every state word of a lane is in [`lane_state`].

use super::padding_tests::{
    Draw, LINKS, QUANTUM, RATES, Values, backend, fixture_values, prefix_mask, request,
};
use super::*;
use conformance::rest_probe::{self, Reach, Schedule};
use core::cell::Cell;
use effect_contract::BankWidth;

/// Every state word of `lane`, both channels: per channel the two envelope words, then the four
/// fields of each of the three ramps. This is the payload's fourteen words per channel, and it is
/// every word `Shaper::run` writes.
fn lane_state<L: Lane, const W: usize>(shaper: &Shaper<L, W>, lane: usize, out: &mut Vec<u32>) {
    for (env, ramps) in [
        (&shaper.left_env, &shaper.left),
        (&shaper.right_env, &shaper.right),
    ] {
        let mut fast = [0_u32; 8];
        let mut slow = [0_u32; 8];
        env.fast.store_bits(&mut fast[..W]);
        env.slow.store_bits(&mut slow[..W]);
        out.extend([fast[lane], slow[lane]]);
        for ramp in &ramps.ramps[lane] {
            out.extend([
                ramp.current.to_bits(),
                ramp.target.to_bits(),
                ramp.step.to_bits(),
                ramp.remaining,
            ]);
        }
    }
}

/// A copy of `bank`, field by field: the shaper derives no `Clone`, and the test adds none.
fn copy_bank<L: Lane, const W: usize>(
    bank: &PreparedTransientShaperBank<L, W>,
) -> PreparedTransientShaperBank<L, W> {
    let shaper = &bank.shaper;
    PreparedTransientShaperBank {
        shaper: Shaper {
            coefficients: shaper.coefficients,
            left_env: shaper.left_env,
            right_env: shaper.right_env,
            left: shaper.left,
            right: shaper.right,
            bypass: shaper.bypass,
            link_mode: shaper.link_mode,
            automation_capacity: shaper.automation_capacity,
            state_sizes: shaper.state_sizes,
            nonfinite: shaper.nonfinite,
            active: shaper.active,
        },
        width: bank.width,
        quantum: bank.quantum,
    }
}

/// The bank width of `W` lanes.
fn width_of<const W: usize>() -> BankWidth {
    match W {
        4 => BankWidth::Four,
        8 => BankWidth::Eight,
        _ => unreachable!("a vector width"),
    }
}

/// Binds a bank of `W` lanes with active lane `l` running `members[l]` (members first), every
/// padded lane cloning member 0.
fn bind_bank_of<L: Lane, const W: usize>(
    members: &[Values],
    rate: u32,
    link: LinkMode,
) -> PreparedTransientShaperBank<L, W> {
    let width = width_of::<W>();
    let mask = prefix_mask(width, members.len());
    let requests: Vec<PrepareEffectRequest<'_>> = (0..W)
        .map(|lane| request(&members[if mask[lane] { lane } else { 0 }], rate, link))
        .collect();
    super::bind::<L, W, false>(
        &TransientShaperFactory,
        PrepareEffectBankRequest {
            backend: backend(width),
            width,
            requests: &requests,
            active_mask: &mask,
        },
    )
    .expect("a well-formed request")
    .expect("the bank binds")
    .0
}

/// Renders one block through the bank's real `process_bank`, the fast path forbidden if `forbid`.
#[allow(clippy::too_many_arguments)]
fn process<L: Lane, const W: usize>(
    bank: &mut PreparedTransientShaperBank<L, W>,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    first_sample: u64,
    spans: &[PreparedAutomationSpan],
    offsets: &[u32],
    forbid: bool,
) {
    SILENT_FORBIDDEN.with(|cell| cell.set(forbid));
    bank.process_bank(
        EffectBankProcessBlock::new(
            left,
            right,
            None,
            frames as u32,
            width_of::<W>(),
            first_sample,
            spans,
            offsets,
            QUANTUM,
        )
        .expect("a bank block"),
    );
    SILENT_FORBIDDEN.with(|cell| cell.set(false));
}

fn admissions() -> usize {
    SILENT_ADMISSIONS.with(Cell::get)
}

// ---------------------------------------------------------------------------------------------
// Gate 1
// ---------------------------------------------------------------------------------------------

/// One bank under the shared driver.
struct Bank<L: Lane, const W: usize> {
    bank: PreparedTransientShaperBank<L, W>,
    frames: usize,
    first_sample: u64,
    /// Draws a target now and then, so ramps are in flight at block ends; `None` in a fork, whose
    /// one block is the claim's block of `+0.0` with no automation.
    automation: Option<Draw>,
    /// A fork forbids the fast path, so its block runs the whole kernel.
    forbid: bool,
    /// Plants envelope words now and then; `None` in a fork.
    plant: Option<Draw>,
}

impl<L: Lane, const W: usize> rest_probe::Probe for Bank<L, W> {
    fn lanes(&self) -> usize {
        W
    }

    fn frames(&self) -> usize {
        self.frames
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        let mut offsets = [0_u32; 9];
        let mut spans = Vec::new();
        for lane in 0..W {
            if let Some(draw) = self.automation.as_mut()
                && self.bank.shaper.active & (1 << lane) != 0
                && draw.chance(1, 8)
            {
                spans.extend(draw.spans(self.first_sample));
            }
            offsets[lane + 1] = spans.len() as u32;
        }
        if let Some(draw) = self.plant.as_mut() {
            for lane in 0..W {
                if self.bank.shaper.active & (1 << lane) != 0 && draw.chance(1, 256) {
                    plant(&mut self.bank.shaper, lane, draw);
                }
            }
        }
        process(
            &mut self.bank,
            left,
            right,
            self.frames,
            self.first_sample,
            &spans,
            &offsets[..=W],
            self.forbid,
        );
        self.first_sample += self.frames as u64;
    }

    fn lanes_at_rest(&self) -> u32 {
        PreparedNativeEffectBank::lanes_at_rest(&self.bank)
    }

    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        lane_state(&self.bank.shaper, lane, out);
    }

    fn fork(&self) -> Self {
        Self {
            bank: copy_bank(&self.bank),
            frames: self.frames,
            first_sample: self.first_sample,
            automation: None,
            forbid: true,
            plant: None,
        }
    }
}

/// Plants one envelope word (or both) of one channel of `lane`, as a state restore at a block
/// boundary may (`restore_track_state_payload` accepts any normal positive envelope pair).
///
/// The driver's signal levels never leave a woken **fast** follower beside a flushed slow one:
/// that needs inputs between about `2e-19` (the fast attack's `k * u` clears the flush threshold)
/// and `5e-18` (the slow attack's does), and every level the driver draws is outside that band.
/// Without the plant, dropping either fast word from the predicate stays green. A slow-only word is
/// what every decay reaches (the fast follower releases five times faster); it is planted too.
fn plant<L: Lane, const W: usize>(shaper: &mut Shaper<L, W>, lane: usize, draw: &mut Draw) {
    const WORDS: [f32; 3] = [1.0e-19, 3.0e-18, 1.0e-6];
    // Each choice from the high bits: the low bits right after a passed `chance(1, 256)` are not
    // independent of it, and drawing the channel from them planted the left channel only.
    let mut pick = |count: usize| ((draw.unit() * count as f32) as usize).min(count - 1);
    let channel = pick(2);
    let value = WORDS[pick(WORDS.len())];
    let words = pick(3);
    let env = if channel == 0 {
        &mut shaper.left_env
    } else {
        &mut shaper.right_env
    };
    match words {
        0 => env.fast = replace_lane(env.fast, lane, value),
        1 => env.slow = replace_lane(env.slow, lane, value),
        _ => {
            env.fast = replace_lane(env.fast, lane, value);
            env.slow = replace_lane(env.slow, lane, value);
        }
    }
}

/// One randomized case at `W` lanes: random members, rate, link, bypass and block length.
///
/// A 48-frame block is shorter than the 64-sample smoothing, so a target drawn at a block start
/// is still in flight at its end and the ramp leg is reached; a 128-frame block halves the
/// blocks a slow follower needs to flush. Gate 2 covers rest reached from woken followers.
fn case<L: Lane, const W: usize>(seed: u64) -> Reach {
    let mut draw = Draw::new(seed);
    let count = 1 + draw.below(W);
    let members: Vec<Values> = (0..count).map(|_| draw.values()).collect();
    let rate = RATES[draw.below(RATES.len())];
    let link = LINKS[draw.below(LINKS.len())];
    let mut bank = bind_bank_of::<L, W>(&members, rate, link);
    bank.shaper.bypass = draw.chance(1, 4);
    let frames = if draw.chance(1, 2) { 48 } else { 128 };
    let mut probe = Bank::<L, W> {
        bank,
        frames,
        first_sample: 0,
        automation: Some(Draw::new(!seed)),
        forbid: false,
        plant: Some(Draw::new(seed ^ 0x5eed)),
    };
    rest_probe::check(
        &mut probe,
        seed,
        Schedule {
            blocks: 800,
            longest_signal: 4,
            longest_silence: 300,
        },
    )
}

/// **Gate 1: a lane the bank reports at rest stays bit for bit at rest through a block of `+0.0`,
/// and writes `+0.0`, at `Simd4` and `Simd8`.**
#[test]
fn a_lane_reported_at_rest_stays_at_rest_through_silence() {
    lane::each_vector_lane!(|L, N| {
        let mut reach = Reach::default();
        for seed in 0..6_u64 {
            reach.add(case::<L, N>(0x1746_0000 + seed));
        }
        reach.assert_reached(&format!("W{N}"));
        std::eprintln!("W{N}: {reach:?}");
    });
}

// ---------------------------------------------------------------------------------------------
// Gate 2
// ---------------------------------------------------------------------------------------------

/// Blocks of tone every lane starts with; lane `l` plays `STAGGER * l` blocks more.
const TONE_BLOCKS: usize = 4;
const STAGGER: usize = 8;
/// Whole-bank silence after the last lane's tone: long enough for every follower to flush.
const SILENT_BLOCKS: usize = 520;
/// Tone again at the end, on every lane.
const CODA_BLOCKS: usize = 4;
/// The tone's level (lane-distinct noise): low enough that its slow follower flushes inside the silence, high enough
/// that both followers wake (the slow attack's `k * u` clears the flush threshold).
const TONE_LEVEL: f32 = 1.0e-15;

/// The fast-path differential at one width, link and member count: two copies of one bank render
/// the same blocks, one with the fast path allowed and one with it forbidden. Returns the blocks
/// the allowed arm skipped and the whole-bank silent blocks it saw.
fn differential<L: Lane, const W: usize>(count: usize, link: LinkMode) -> (usize, usize) {
    let members: Vec<Values> = (0..count).map(fixture_values).collect();
    let mut allowed = bind_bank_of::<L, W>(&members, 48_000, link);
    let mut forbidden = copy_bank(&allowed);
    let frames = QUANTUM as usize;
    let tone_end = TONE_BLOCKS + STAGGER * (count - 1);
    let blocks = tone_end + SILENT_BLOCKS + CODA_BLOCKS;
    let offsets = [0_u32; 9];
    let mut draw = Draw::new(0x1746_2000 + count as u64);
    let (mut skipped, mut silent) = (0, 0);
    let context = |block: usize| format!("W{W}, {count} members, {link:?}, block {block}");
    for block in 0..blocks {
        let mut left = vec![0.0_f32; frames * W];
        let mut right = vec![0.0_f32; frames * W];
        for lane in 0..count {
            let playing = block < TONE_BLOCKS + STAGGER * lane || block >= tone_end + SILENT_BLOCKS;
            if !playing {
                continue;
            }
            for frame in 0..frames {
                left[frame * W + lane] = TONE_LEVEL * (draw.unit() * 2.0 - 1.0);
                right[frame * W + lane] = TONE_LEVEL * (draw.unit() * 2.0 - 1.0);
            }
        }
        let quiet = left.iter().chain(&right).all(|word| word.to_bits() == 0);
        silent += usize::from(quiet);
        let (mut control_left, mut control_right) = (left.clone(), right.clone());
        let before = admissions();
        process(
            &mut allowed,
            &mut left,
            &mut right,
            frames,
            (block * frames) as u64,
            &[],
            &offsets[..=W],
            false,
        );
        skipped += admissions() - before;
        let before = admissions();
        process(
            &mut forbidden,
            &mut control_left,
            &mut control_right,
            frames,
            (block * frames) as u64,
            &[],
            &offsets[..=W],
            true,
        );
        assert_eq!(
            admissions(),
            before,
            "{}: the control arm skipped",
            context(block)
        );
        for (word, (fast, slow)) in left
            .iter()
            .chain(&right)
            .zip(control_left.iter().chain(&control_right))
            .enumerate()
        {
            assert_eq!(
                fast.to_bits(),
                slow.to_bits(),
                "{}: output word {word}",
                context(block)
            );
        }
        for lane in 0..W {
            let (mut fast, mut slow) = (Vec::new(), Vec::new());
            lane_state(&allowed.shaper, lane, &mut fast);
            lane_state(&forbidden.shaper, lane, &mut slow);
            assert_eq!(fast, slow, "{}: lane {lane} state", context(block));
        }
        assert_eq!(
            allowed.shaper.nonfinite,
            forbidden.shaper.nonfinite,
            "{}: the D7 record",
            context(block)
        );
    }
    (skipped, silent)
}

/// **Gate 2: a bank renders bit-identically with the silent fast path and without it,** through
/// tone, a silence in which lanes come to rest on different blocks, and tone again, at `Simd4` and
/// `Simd8`, full and padded, under every link mode. Every output word and every lane's whole state
/// is compared after every block (the shaper has no clocks), and the fast path must be taken, but
/// not on every whole-bank silent block, so the staggered rests were reached before the bank-wide
/// claim held.
///
/// Red if the bank-wide rule is weakened from "every lane at rest" to "some lane at rest": the
/// first lane to rest (or a padded lane, at rest from the start) would freeze its bank-mates'
/// decaying followers.
#[test]
fn the_silent_fast_path_renders_the_kernels_bits() {
    lane::each_vector_lane!(|L, N| {
        for count in [N, N - 1, 1] {
            for link in LINKS {
                let (skipped, silent) = differential::<L, N>(count, link);
                assert!(
                    skipped > 0,
                    "W{N}, {count} members, {link:?}: never skipped"
                );
                assert!(
                    skipped < silent,
                    "W{N}, {count} members, {link:?}: skipped every silent block"
                );
            }
        }
    });
}
