//! Issue #1745 gate 1: the compressor bank's per-lane rest claim, at `Simd4` and `Simd8`.
//!
//! A crate-internal test for the same reason as `padding_tests`: it binds the bank at every vector
//! width the build has, and its `fork` copies channel state no public surface exposes (padded lanes
//! are not tracks, so the payload cannot carry them).
//!
//! The compressor has no clocks: every state word of a lane is in [`lane_state`].

use super::padding_tests::{
    LINKS, QUANTUM, RATES, Values, at, bind, draw_spans, draw_values, lane_requests, lane_state,
    members_first, width_of,
};
use super::*;
use conformance::rest_probe::{self, Reach, Schedule};
use dsp_reference::randomized::Draw;

/// Not a divisor of the 64-sample smoothing, so a target's ramp is still in flight at the end of
/// the block it starts in.
const FRAMES: usize = 48;

/// One bank under the shared driver, with what it was bound from so `fork` can rebind it.
struct Bank<L: Lane> {
    bank: PreparedCompressorBank<L>,
    members: Vec<Values>,
    mask: Vec<bool>,
    rate: u32,
    link: LinkMode,
    /// Render through the collapsed body; the members are symmetric.
    mono: bool,
    first_sample: u64,
    /// Draws a target now and then, so ramps are in flight at block ends; `None` in a fork, whose
    /// one block is the claim's block of `+0.0` with no automation.
    automation: Option<Draw>,
}

impl<L: Lane> Bank<L> {
    fn bound(&self) -> PreparedCompressorBank<L> {
        let requests = lane_requests(&self.members, &self.mask, 0, self.rate, self.link);
        bind::<L>(&requests, &self.mask)
    }
}

impl<L: Lane> rest_probe::Probe for Bank<L> {
    fn lanes(&self) -> usize {
        L::WIDTH
    }

    fn frames(&self) -> usize {
        FRAMES
    }

    /// The real bank body. Collapsed, the right plane is what the seam writes: the left one.
    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        let (_, width) = width_of::<L>();
        let mut offsets = vec![0_u32; L::WIDTH + 1];
        let mut spans = Vec::new();
        for lane in 0..L::WIDTH {
            if let Some(draw) = self.automation.as_mut()
                && self.mask[lane]
                && draw.chance(1, 8)
            {
                spans.extend(at(&draw_spans(draw, self.mono), self.first_sample));
            }
            offsets[lane + 1] = spans.len() as u32;
        }
        let block = EffectBankProcessBlock::new(
            left,
            right,
            None,
            FRAMES as u32,
            width,
            self.first_sample,
            &spans,
            &offsets,
            QUANTUM,
        )
        .expect("a bank block");
        if self.mono {
            self.bank.process_bank_mono(block);
            right.copy_from_slice(left);
        } else {
            self.bank.process_bank(block);
        }
        self.first_sample += FRAMES as u64;
    }

    fn lanes_at_rest(&self) -> u32 {
        PreparedNativeEffectBank::lanes_at_rest(&self.bank)
    }

    /// Both channels' envelope, coefficient words and every field of every ramp: the
    /// `copy_state_from` list, which is every word the kernel carries across a block.
    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        out.extend(lane_state(&self.bank.instance.left, lane));
        out.extend(lane_state(&self.bank.instance.right, lane));
    }

    /// A fresh bank from the same requests carrying this one's state, with no claim standing.
    fn fork(&self) -> Self {
        let mut bank = self.bound();
        bank.instance.left.copy_state_from(&self.bank.instance.left);
        bank.instance
            .right
            .copy_state_from(&self.bank.instance.right);
        bank.instance.silent_fixed_point = false;
        Self {
            bank,
            members: self.members.clone(),
            mask: self.mask.clone(),
            rate: self.rate,
            link: self.link,
            mono: self.mono,
            first_sample: self.first_sample,
            automation: None,
        }
    }
}

/// Sets parameter `parameter` to `value` on both channels.
fn set(values: &mut Values, parameter: usize, value: f32) {
    values[parameter * 2].value = value;
    values[parameter * 2 + 1].value = value;
}

/// One randomized case at `L`: random members (some with the fastest release, so a compressing
/// lane comes back to rest inside the case, some with a 0 dB threshold), random rate and link.
fn case<L: Lane>(seed: u64, mono: bool) -> Reach {
    let lanes = L::WIDTH;
    let mut draw = Draw::new(seed);
    let count = 1 + draw.below(lanes);
    let members: Vec<Values> = (0..count)
        .map(|_| {
            let mut values = draw_values(&mut draw, mono);
            if draw.chance(2, 3) {
                set(&mut values, 4, 5.0);
            }
            if draw.chance(1, 3) {
                set(&mut values, 0, 0.0);
            }
            values
        })
        .collect();
    let mask = members_first(count, lanes);
    let rate = draw.pick(&RATES);
    let link = draw.pick(&LINKS);
    let requests = lane_requests(&members, &mask, 0, rate, link);
    let bank = bind::<L>(&requests, &mask);
    let mut bank = Bank::<L> {
        bank,
        members: members.clone(),
        mask,
        rate,
        link,
        mono,
        first_sample: 0,
        automation: Some(Draw::new(!seed)),
    };
    rest_probe::check(
        &mut bank,
        seed,
        Schedule {
            blocks: 800,
            longest_signal: 6,
            longest_silence: 300,
        },
    )
}

/// **Gate 1: a lane the bank reports at rest stays bit for bit at rest through a block of `+0.0`,
/// and writes `+0.0`, at `Simd4` and `Simd8`, dual and collapsed.**
#[test]
fn a_lane_reported_at_rest_stays_at_rest_through_silence() {
    lane::each_vector_lane!(|L| {
        let mut reach = Reach::default();
        for seed in 0..4_u64 {
            reach.add(case::<L>(0x1745_0000 + seed, false));
        }
        for seed in 0..2_u64 {
            reach.add(case::<L>(0x1745_1000 + seed, true));
        }
        reach.assert_reached(&format!("W{}", L::WIDTH));
        std::eprintln!("W{}: {reach:?}", L::WIDTH);
    });
}
