//! Issue #1746 gates 1 and 2: the soft-clip bank's per-lane rest claim and its bank-wide silent
//! fast path, at `Simd4` and `Simd8`.
//!
//! Crate-internal for the same reason as `padding_tests`: it binds the bank at every vector width
//! the build has, and its fork copies channel state no public surface exposes (padded lanes are not
//! tracks, and the payload carries no mirror rows and no cursor).
//!
//! The soft clip's one clock is the shared cursor `history.pos` of each channel: [`lane_state`]
//! leaves it out (gate 1), and gate 2 compares it beside every lane word.

use super::padding_tests::{
    Draw, QUANTUM, RATES, Values, backend, prefix_mask, request, values_from,
};
use super::*;
use conformance::rest_probe::{self, Reach, Schedule};
use effect_contract::LinkMode;

/// Not a divisor of the 64-sample smoothing, so a target's ramp is still in flight at the end of
/// the block it starts in, and shorter than the 32-row history, so a lane takes more than one
/// silent block to rest.
const FRAMES: usize = 24;

/// The bank width of an `L`-wide cohort.
fn width_of<L: Lane>() -> BankWidth {
    *BankWidth::ALL
        .iter()
        .find(|width| width.lanes() as usize == L::WIDTH)
        .expect("a bank width per vector lane")
}

/// Binds a bank of `L::WIDTH` lanes, member `l` on active lane `l` and member 0 on every padded
/// lane, through `prepare_bank` without the D4 width check.
fn bind<L: Lane>(members: &[Values], mask: &[bool], rate: u32) -> PreparedSoftClipBank<L> {
    let width = width_of::<L>();
    let requests: Vec<PrepareEffectRequest<'_>> = mask
        .iter()
        .enumerate()
        .map(|(lane, active)| {
            request(
                &members[if *active { lane } else { 0 }],
                rate,
                LinkMode::DualMono,
            )
        })
        .collect();
    prepare_bank::<L, false>(
        &SoftClipFactory,
        PrepareEffectBankRequest {
            backend: backend(width),
            width,
            requests: &requests,
            active_mask: mask,
        },
    )
    .expect("a well-formed request")
    .expect("the bank binds")
    .0
}

/// Copies every word of `from`, the cursor included, onto `to`.
fn copy_channel<L: Lane>(to: &mut Channel<L>, from: &Channel<L>) {
    to.state = from.state;
    to.history.pos = from.history.pos;
    to.history.x.copy_from_slice(&from.history.x);
    to.history.e.copy_from_slice(&from.history.e);
    to.history.dry.copy_from_slice(&from.history.dry);
    to.ramps.copy_from_slice(&from.ramps);
}

/// Every word of `lane` in one channel, the cursor excluded: the three state words, every field of
/// the three ramps, and the lane's 64 rows of each history.
fn channel_lane_state<L: Lane>(channel: &Channel<L>, lane: usize, out: &mut Vec<u32>) {
    for parameter in 0..PARAMETER_COUNT {
        out.push(lane_of(channel.state.field(parameter), lane).to_bits());
        let ramp = &channel.ramps[lane][parameter];
        out.extend([
            ramp.current.to_bits(),
            ramp.target.to_bits(),
            ramp.step.to_bits(),
            ramp.remaining,
        ]);
    }
    for history in [&channel.history.x, &channel.history.e, &channel.history.dry] {
        out.extend(
            history
                .iter()
                .skip(lane)
                .step_by(L::WIDTH)
                .map(|word| word.to_bits()),
        );
    }
}

/// Every word of `lane`, both channels, the cursor excluded.
fn lane_state<L: Lane>(bank: &PreparedSoftClipBank<L>, lane: usize, out: &mut Vec<u32>) {
    channel_lane_state(&bank.inner.left, lane, out);
    channel_lane_state(&bank.inner.right, lane, out);
}

/// Runs `body` with the silent fast path forbidden on this thread.
fn forbidden<T>(body: impl FnOnce() -> T) -> T {
    SILENT_FORBIDDEN.with(|flag| flag.set(true));
    let result = body();
    SILENT_FORBIDDEN.with(|flag| flag.set(false));
    result
}

/// Blocks the fast path has skipped on this thread so far.
fn admissions() -> usize {
    SILENT_ADMISSIONS.with(core::cell::Cell::get)
}

/// Renders one bank block in place, `spans[l]` on lane `l`.
fn render_block<L: Lane>(
    bank: &mut PreparedSoftClipBank<L>,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
    spans: &[Vec<PreparedAutomationSpan>],
) -> BankProcessReport {
    let mut offsets = vec![0_u32; L::WIDTH + 1];
    let mut automation = Vec::new();
    for lane in 0..L::WIDTH {
        if let Some(own) = spans.get(lane) {
            automation.extend_from_slice(own);
        }
        offsets[lane + 1] = automation.len() as u32;
    }
    let frames = left.len() / L::WIDTH;
    bank.process_bank(
        EffectBankProcessBlock::new(
            left,
            right,
            None,
            frames as u32,
            width_of::<L>(),
            first_sample,
            &automation,
            &offsets,
            QUANTUM,
        )
        .expect("a bank block"),
    )
}

// ---------------------------------------------------------------------------------------------
// Gate 1: the per-lane claim
// ---------------------------------------------------------------------------------------------

/// Restores `lane` of `channel` as a track restore would (`apply_lane_words`), keeping its ramps,
/// with non-zero words in the interpolator input history `X` and `+0.0` in `e` and the dry
/// history. The decoder admits that payload (`x_history_word_valid`), but rendering never writes
/// it, since a rendered `X` row is `flush(2g * dry)`: this is the only way a lane reaches a
/// non-zero `X` beside a zero dry history, which the predicate's `X` leg exists for.
fn plant_interpolator_history<L: Lane>(channel: &mut Channel<L>, lane: usize, draw: &mut Draw) {
    let mut x = [0.0_f32; X_HISTORY_AGES];
    x[draw.below(X_HISTORY_AGES)] = if draw.chance(1, 2) { 0.5 } else { -0.25 };
    let restore = LaneRestore {
        ramps: channel.ramps[lane],
        x,
        e: [0.0; E_HISTORY_AGES],
        dry: [0.0; DRY_HISTORY_AGES],
    };
    apply_lane_words(channel, lane, &restore);
}

/// One bank under the shared driver, with what it was bound from so `fork` can rebind it.
struct Bank<L: Lane> {
    bank: PreparedSoftClipBank<L>,
    members: Vec<Values>,
    mask: Vec<bool>,
    rate: u32,
    first_sample: u64,
    /// Draws a target now and then, so ramps are in flight at block ends; `None` in a fork, whose
    /// one block is the claim's block of `+0.0` with no automation.
    automation: Option<Draw>,
    /// A fork renders its whole kernel: the fast path is forbidden for its blocks.
    fork: bool,
}

impl<L: Lane> rest_probe::Probe for Bank<L> {
    fn lanes(&self) -> usize {
        L::WIDTH
    }

    fn frames(&self) -> usize {
        FRAMES
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        let mut spans = vec![Vec::new(); L::WIDTH];
        for (lane, own) in spans.iter_mut().enumerate() {
            if let Some(draw) = self.automation.as_mut()
                && self.mask[lane]
                && draw.chance(1, 8)
            {
                *own = draw.spans(self.first_sample);
            }
        }
        let first_sample = self.first_sample;
        let bank = &mut self.bank;
        if self.fork {
            forbidden(|| render_block(bank, left, right, first_sample, &spans));
        } else {
            render_block(bank, left, right, first_sample, &spans);
        }
        self.first_sample += FRAMES as u64;
        if let Some(draw) = self.automation.as_mut() {
            for lane in (0..L::WIDTH).filter(|&lane| self.mask[lane]) {
                if draw.chance(1, 24) {
                    plant_interpolator_history(&mut self.bank.inner.left, lane, draw);
                }
            }
        }
    }

    fn lanes_at_rest(&self) -> u32 {
        PreparedNativeEffectBank::lanes_at_rest(&self.bank)
    }

    /// Both channels' state words, ramps and history rows; the cursor `history.pos` is the clock.
    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        lane_state(&self.bank, lane, out);
    }

    /// A fresh bank from the same requests carrying this one's state, fast path forbidden.
    fn fork(&self) -> Self {
        let mut bank = bind::<L>(&self.members, &self.mask, self.rate);
        copy_channel(&mut bank.inner.left, &self.bank.inner.left);
        copy_channel(&mut bank.inner.right, &self.bank.inner.right);
        Self {
            bank,
            members: self.members.clone(),
            mask: self.mask.clone(),
            rate: self.rate,
            first_sample: self.first_sample,
            automation: None,
            fork: true,
        }
    }
}

/// One randomized case at `L`: random members, member count and rate, with automation now and
/// then; some members sit at identity (`mix = 0`, `output = 0 dB`), where the dry path is the
/// output.
fn case<L: Lane>(seed: u64) -> Reach {
    let lanes = L::WIDTH;
    let mut draw = Draw::new(seed);
    let count = 1 + draw.below(lanes);
    let members: Vec<Values> = (0..count)
        .map(|_| {
            if draw.chance(1, 4) {
                values_from([(6.0, 6.0), (0.0, 0.0), (0.0, 0.0)])
            } else {
                draw.values()
            }
        })
        .collect();
    let mask = prefix_mask(width_of::<L>(), count);
    let rate = RATES[draw.below(RATES.len())];
    let mut bank = Bank::<L> {
        bank: bind::<L>(&members, &mask, rate),
        members,
        mask,
        rate,
        first_sample: 0,
        automation: Some(Draw::new(!seed)),
        fork: false,
    };
    rest_probe::check(
        &mut bank,
        seed,
        Schedule {
            blocks: 600,
            longest_signal: 6,
            longest_silence: 12,
        },
    )
}

/// **Gate 1: a lane the bank reports at rest stays bit for bit at rest through a block of `+0.0`,
/// and writes `+0.0`, at `Simd4` and `Simd8`.**
#[test]
fn a_lane_reported_at_rest_stays_at_rest_through_silence() {
    lane::each_vector_lane!(|L| {
        let mut reach = Reach::default();
        for seed in 0..6_u64 {
            reach.add(case::<L>(0x1746_5C00 + seed));
        }
        reach.assert_reached(&format!("W{}", L::WIDTH));
        std::eprintln!("W{}: {reach:?}", L::WIDTH);
    });
}

// ---------------------------------------------------------------------------------------------
// Gate 2: the bank-wide fast path is bit-identical to rendering
// ---------------------------------------------------------------------------------------------

/// Tone blocks before and after the silence.
const TONE_BLOCKS: usize = 6;

/// Silent blocks; lane `l` takes an automation point at silent block `2 * l`, so the lanes come to
/// rest on different blocks and the bank only after the last one.
const SILENT_BLOCKS: usize = 40;

/// One whole render: every block's output words, and after every block every lane's words and both
/// cursors. Returns them with the fast-path admissions it made.
fn render_tone_silence_tone<L: Lane>(members: &[Values], mask: &[bool]) -> (Vec<u32>, usize) {
    let lanes = L::WIDTH;
    let mut bank = bind::<L>(members, mask, 48_000);
    let mut record = Vec::new();
    let start = admissions();
    let mut first_sample = 0_u64;
    for block in 0..2 * TONE_BLOCKS + SILENT_BLOCKS {
        let silent = (TONE_BLOCKS..TONE_BLOCKS + SILENT_BLOCKS).contains(&block);
        let mut left = vec![0.0_f32; FRAMES * lanes];
        let mut right = vec![0.0_f32; FRAMES * lanes];
        let mut spans = vec![Vec::new(); lanes];
        for lane in (0..lanes).filter(|&lane| mask[lane]) {
            if silent {
                if block - TONE_BLOCKS == 2 * lane {
                    spans[lane].push(PreparedAutomationSpan {
                        channel: ParameterChannel::Left,
                        parameter_index: 2,
                        sample: first_sample,
                        value: 0.25 + 0.05 * lane as f32,
                    });
                }
                continue;
            }
            for frame in 0..FRAMES {
                let t = (first_sample + frame as u64) as f64;
                let phase = t * (0.03 + 0.011 * lane as f64);
                left[frame * lanes + lane] = (0.6 * math::sin(phase)) as f32;
                right[frame * lanes + lane] = (0.4 * math::cos(1.3 * phase)) as f32;
            }
        }
        render_block(&mut bank, &mut left, &mut right, first_sample, &spans);
        first_sample += FRAMES as u64;
        record.extend(left.iter().chain(&right).map(|word| word.to_bits()));
        for lane in 0..lanes {
            lane_state(&bank, lane, &mut record);
        }
        record.extend([bank.inner.left.history.pos, bank.inner.right.history.pos]);
    }
    (record, admissions() - start)
}

/// **Gate 2: tone, then silence in which the lanes come to rest on different blocks, then tone
/// again renders bit-identically with the bank-wide fast path and without it**, every output word
/// of every block and every lane word and cursor after every block, full and padded banks, at
/// `Simd4` and `Simd8`; and the fast path is taken, but not on every silent block.
#[test]
fn the_silent_fast_path_renders_what_the_kernel_renders() {
    lane::each_vector_lane!(|L| {
        let lanes = L::WIDTH;
        let mut draw = Draw::new(0x1746_0002 + lanes as u64);
        for members in [lanes, lanes / 2] {
            let values: Vec<Values> = (0..members).map(|_| draw.values()).collect();
            let mask = prefix_mask(width_of::<L>(), members);
            let (fast, admitted) = render_tone_silence_tone::<L>(&values, &mask);
            let (kernel, refused) = forbidden(|| render_tone_silence_tone::<L>(&values, &mask));
            let context = format!("W{lanes}, {members} members");
            std::eprintln!("{context}: fast path took {admitted} of {SILENT_BLOCKS} silent blocks");
            assert_eq!(refused, 0, "{context}: the control arm took the fast path");
            assert!(
                admitted > 0 && admitted < SILENT_BLOCKS,
                "{context}: the fast path took {admitted} of {SILENT_BLOCKS} silent blocks"
            );
            assert_eq!(fast.len(), kernel.len(), "{context}: record length");
            if let Some(word) = (0..fast.len()).find(|&word| fast[word] != kernel[word]) {
                panic!(
                    "{context}: record word {word} differs: {:#x} with the fast path, {:#x} without",
                    fast[word], kernel[word]
                );
            }
        }
    });
}
