//! Issue #1781 gates 1 and 2: the builtin input, fader and matrix stages' per-lane rest claims,
//! at every vector width the build has, and the input's clock advance.
//!
//! A crate-internal module so that it binds the stages at `Simd4` and, on an AVX2 host, `Simd8`
//! (`lane::each_vector_lane!`), reads every state word of a lane, padded lanes included, and forks
//! a stage. The shared driver is `conformance::rest_probe`. Each probe's `render` applies its own
//! random retargets between blocks, so ramps are in flight at some boundaries; a fork applies
//! none, because its block is the claim's.

use super::*;
use conformance::rest_probe::{self, Probe, Reach, Schedule};

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// SplitMix64.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound.max(1) as u64) as usize
    }

    fn chance(&mut self, numerator: usize, denominator: usize) -> bool {
        self.below(denominator) < numerator
    }

    fn pick<T: Copy>(&mut self, values: &[T]) -> T {
        values[self.below(values.len())]
    }
}

fn selector(rng: &mut Rng) -> BuiltinLaneSelector {
    rng.pick(&[
        BuiltinLaneSelector::Left,
        BuiltinLaneSelector::Right,
        BuiltinLaneSelector::Both,
    ])
}

/// The bits of every lane of `value`.
fn bits<L: Lane>(value: L, lane: usize) -> u32 {
    lane_read::<L>(value)[lane].to_bits()
}

fn mask_bit<L: Lane>(mask: L::Mask, lane: usize) -> u32 {
    bits::<L>(L::select(mask, L::splat(1.0), L::zero()), lane)
}

/// The schedule of every probe here: about 0.6 s at 48 kHz, with silence runs long enough for the
/// input's joint flush to arm and its tails to rest.
fn schedule(frames: usize) -> Schedule {
    Schedule {
        blocks: 30_720 / frames,
        longest_signal: 768 / frames,
        longest_silence: 11_520 / frames,
    }
}

// ---------------------------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------------------------

struct InputProbe<L: Lane> {
    stage: InputStage<L>,
    frames: usize,
    rate: u32,
    members: usize,
    /// Retarget a random member between blocks. `None` on a fork.
    retune: Option<Rng>,
    /// Claims on a lane whose trim carries the sign bit on some channel.
    inverted_claims: Cell<usize>,
}

fn input_parameters(rng: &mut Rng) -> BuiltinParameters {
    let mut parameters = BuiltinParameters::default();
    for channel in [&mut parameters.left, &mut parameters.right] {
        channel.polarity_invert = rng.chance(1, 2);
        channel.trim_db = rng.pick(&[-18.0, -6.0, 0.0, 6.0]);
        channel.hpf_hz = rng.pick(&[0.0, 30.0, 120.0]);
        channel.lpf_hz = rng.pick(&[0.0, 6_000.0, 16_000.0]);
    }
    parameters
}

impl<L: Lane> InputProbe<L> {
    fn new(rng: &mut Rng, frames: usize) -> Self {
        let rate = rng.pick(&RATES);
        let members = 1 + rng.below(L::WIDTH);
        let tracks: Vec<PreparedInputTrack> = (0..members)
            .map(|_| prepare_input_track(rate, &input_parameters(rng)).expect("a track"))
            .collect();
        Self {
            stage: InputStage::<L>::new(&tracks),
            frames,
            rate,
            members,
            retune: Some(Rng(rng.next() | 1)),
            inverted_claims: Cell::new(0),
        }
    }

    /// About one block in ten: a trim, polarity or filter retarget of one member, over a window
    /// that may outlast the block.
    fn maybe_retune(&mut self) {
        let Some(rng) = self.retune.as_mut() else {
            return;
        };
        if !rng.chance(1, 10) {
            return;
        }
        let lane = rng.below(self.members);
        let window = rng.pick(&[0, 17, 64, 200]);
        match rng.below(3) {
            0 => {
                let gain = checked_trim_gain(rng.pick(&[-12.0, -3.0, 0.0, 9.0])).expect("gain");
                let lanes = selector(rng);
                self.stage.set_trim_db(lane, lanes, gain, window);
            }
            1 => {
                let lanes = selector(rng);
                let inverted = rng.chance(1, 2);
                self.stage
                    .set_polarity_invert(lane, lanes, inverted, window);
            }
            _ => {
                let hpf = rng.pick(&[0.0, 40.0, 200.0]);
                let lpf = rng.pick(&[0.0, 5_000.0, 12_000.0]);
                let pair = prepare_input_filter_pair(self.rate, hpf, lpf).expect("a pair");
                for mut target in pair.targets {
                    target.lanes = selector(rng);
                    self.stage.apply_prepared_filter(lane, target);
                }
            }
        }
    }

    /// Now and then, after a block, a `-0.0` integrator on a resting lane through the
    /// fault-injection hook (`set_lane_state_words`): no render leaves one at a block boundary in
    /// a designed section (its update maps it to `+0.0`), so this is how the predicate's raw-bits
    /// term is reached. A designed section's `+0.0` block then moves the word, so the lane must be
    /// refused. `None` on a fork.
    fn maybe_inject(&mut self) {
        let Some(rng) = self.retune.as_mut() else {
            return;
        };
        if rng.chance(1, 8) {
            let lane = rng.below(L::WIDTH);
            let mut words = self.stage.lane_state_words(lane);
            if words.iter().all(|word| *word == 0) {
                words[rng.below(words.len())] = (-0.0_f32).to_bits();
                self.stage.set_lane_state_words(lane, words);
            }
        }
    }

    /// Every word a render or a control call can move for `lane`, both channels, but the silence
    /// counters (the clock); and the stage-wide caches a block could move.
    fn words(&self, lane: usize, out: &mut Vec<u32>) {
        let stage = &self.stage;
        let coef = |coef: &SvfCoef<L>, out: &mut Vec<u32>| {
            for word in [coef.c1, coef.a2, coef.a3, coef.m0, coef.m1, coef.m2] {
                out.push(bits::<L>(word, lane));
            }
        };
        for channel in 0..2 {
            for section in 0..2 {
                let state = &stage.state.section[channel][section];
                out.push(bits::<L>(state.ic1, lane));
                out.push(bits::<L>(state.ic2, lane));
                coef(&stage.coef.section[channel][section], out);
                coef(&stage.filter_target[channel][section], out);
                coef(&stage.filter_step[channel][section], out);
                out.push(stage.filter_remaining[channel][section][lane]);
                out.push(u32::from(stage.plan.elided[channel][section]));
            }
            out.push(bits::<L>(stage.coef.trim[channel], lane));
            out.push(bits::<L>(stage.ramp.current[channel], lane));
            out.push(bits::<L>(stage.ramp.target[channel], lane));
            out.push(bits::<L>(stage.ramp.step[channel], lane));
            out.push(bits::<L>(stage.ramp.remaining[channel], lane));
            out.push(stage.remaining[channel][lane]);
            out.push(stage.lifetime_recovered[channel] as u32);
        }
        out.push(u32::from(stage.ramping));
        out.push(u32::from(stage.filter_ramping));
        out.push(u32::from(stage.symmetry >> lane & 1));
    }
}

impl<L: Lane> Probe for InputProbe<L> {
    fn lanes(&self) -> usize {
        L::WIDTH
    }

    fn frames(&self) -> usize {
        self.frames
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.maybe_retune();
        let _ = self.stage.process(left, right, self.frames);
        self.maybe_inject();
    }

    fn lanes_at_rest(&self) -> u32 {
        let mask = self.stage.lanes_at_rest();
        let inverted = (0..L::WIDTH).filter(|&lane| {
            mask & (1 << lane) != 0
                && (0..2).any(|channel| self.stage.trim_signed(lane, channel).is_sign_negative())
        });
        self.inverted_claims
            .set(self.inverted_claims.get() + inverted.count());
        mask
    }

    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        self.words(lane, out);
    }

    fn fork(&self) -> Self {
        Self {
            stage: self.stage.clone(),
            frames: self.frames,
            rate: self.rate,
            members: self.members,
            retune: None,
            inverted_claims: Cell::new(0),
        }
    }
}

fn input_claim_holds<L: Lane>(label: &str) {
    let mut reach = Reach::default();
    let mut inverted = 0;
    for seed in 0..4_u64 {
        let mut rng = Rng(0x1781_0001 + seed * 0x9e37);
        let frames = if seed % 2 == 0 { 32 } else { 128 };
        let mut probe = InputProbe::<L>::new(&mut rng, frames);
        reach.add(rest_probe::check(
            &mut probe,
            0x1781 + seed,
            schedule(frames),
        ));
        inverted += probe.inverted_claims.get();
    }
    eprintln!("{label}: {reach:?}, polarity-inverted claims {inverted}");
    reach.assert_reached(label);
    assert!(
        inverted > 0,
        "{label}: no polarity-inverted lane was claimed"
    );
}

/// Gate 1 for the input stage: at every block after which the stage sets a lane's rest bit, one
/// more block of `+0.0` through the whole kernel leaves that lane's state words (integrators,
/// coefficients, trim and filter ramps, countdowns, and the stage's ramp flags, elision plan and
/// symmetry bit; the silence counter is the clock) where they were and writes `+0.0` on both
/// planes. Random members with polarity inversion, trims and filters, padded lanes, and random
/// trim, polarity and filter retargets between blocks, so ramps are in flight at some
/// boundaries; polarity-inverted lanes must be claimed.
#[test]
fn an_input_lane_called_at_rest_stays_there_on_a_silent_block() {
    lane::each_vector_lane!(|L, W| input_claim_holds::<L>(&format!("input W={W}")));
}

/// Every lane's state words, silence counters included.
fn input_words<L: Lane>(stage: &InputStage<L>) -> Vec<u32> {
    let probe = InputProbe {
        stage: stage.clone(),
        frames: 0,
        rate: 0,
        members: 0,
        retune: None,
        inverted_claims: Cell::new(0),
    };
    let mut out = Vec::new();
    for lane in 0..L::WIDTH {
        probe.words(lane, &mut out);
        for channel in 0..2 {
            out.push(bits::<L>(stage.state.silence[channel], lane));
        }
    }
    out
}

fn input_clocks_advance<L: Lane>(label: &str) {
    let all = (1_u32 << L::WIDTH) - 1;
    for seed in 0..3_u64 {
        let mut rng = Rng(0x1781_0101 + seed * 0x9e37);
        let mut probe = InputProbe::<L>::new(&mut rng, 128);
        let window = lane::silence_frames(probe.rate) as f32;
        // Counters that differ by lane and channel: some a few frames short of the arming edge,
        // some past it, some a block short of the saturation `2^24`.
        for (channel, silence) in probe.stage.state.silence.iter_mut().enumerate() {
            let mut words = lane_read::<L>(*silence);
            for (lane, word) in words.iter_mut().enumerate().take(L::WIDTH) {
                *word = match (lane + channel) % 4 {
                    0 => (lane * 7) as f32,
                    1 => window - (lane + 1) as f32,
                    2 => window + 3.0,
                    _ => 16_777_216.0 - (lane * 40) as f32,
                };
            }
            *silence = lane_words::<L>(&words);
        }
        assert_eq!(
            probe.stage.lanes_at_rest(),
            all,
            "{label}: a fresh stage rests"
        );
        let mut rendered = probe.stage.clone();
        let mut skipped = probe.stage.clone();
        let blocks = 2 * window as usize / 64 + 8;
        let mut left = vec![0.0_f32; 128 * L::WIDTH];
        let mut right = vec![0.0_f32; 128 * L::WIDTH];
        for block in 0..blocks {
            let frames = rng.pick(&[1, 17, 32, 64, 100, 128]);
            left.fill(0.0);
            right.fill(0.0);
            let words = frames * L::WIDTH;
            let _ = rendered.process(&mut left[..words], &mut right[..words], frames);
            skipped.advance_clocks(all, frames);
            assert_eq!(
                input_words(&rendered),
                input_words(&skipped),
                "{label}: block {block} of {frames} frames"
            );
        }
        // Signal after the skips renders the same bits in both.
        for (index, (l, r)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
            *l = ((index * 37) % 101) as f32 / 50.0 - 1.0;
            *r = ((index * 11) % 97) as f32 / 48.0 - 1.0;
        }
        let (mut other_left, mut other_right) = (left.clone(), right.clone());
        let _ = rendered.process(&mut left, &mut right, 128);
        let _ = skipped.process(&mut other_left, &mut other_right, 128);
        assert!(
            left.iter()
                .zip(&other_left)
                .all(|(a, b)| a.to_bits() == b.to_bits())
                && right
                    .iter()
                    .zip(&other_right)
                    .all(|(a, b)| a.to_bits() == b.to_bits()),
            "{label}: the block after the skips"
        );
        // A lane outside the mask keeps its counters; a lane inside it advances.
        let mut partial = probe.stage.clone();
        partial.state.silence = [L::zero(); 2];
        partial.advance_clocks(all & !1, 16);
        for channel in 0..2 {
            assert_eq!(bits::<L>(partial.state.silence[channel], 0), 0, "{label}");
            assert_eq!(
                lane_read::<L>(partial.state.silence[channel])[1],
                16.0,
                "{label}"
            );
        }
    }
}

/// Gate 2 for the input stage: a resting stage that skips each block through `advance_clocks`
/// ends with every state word, silence counters included, equal to a twin that rendered the
/// block of `+0.0`: across the arming edge, at the saturation, at block lengths shorter than the
/// quantum, with counters that differ by lane and channel; and the next signal block renders the
/// same bits.
#[test]
fn input_advance_clocks_leaves_the_stage_where_a_rendered_zero_block_does() {
    lane::each_vector_lane!(|L, W| input_clocks_advance::<L>(&format!("input W={W}")));
}

// ---------------------------------------------------------------------------------------------
// Fader
// ---------------------------------------------------------------------------------------------

struct FaderProbe<L: Lane> {
    stage: FaderRampStage<L>,
    frames: usize,
    members: usize,
    retune: Option<Rng>,
    /// Claims on an unmuted lane, and refusals of a settled lane whose gain carries the sign bit.
    unmuted_claims: Cell<usize>,
}

impl<L: Lane> FaderProbe<L> {
    fn new(rng: &mut Rng, frames: usize) -> Self {
        let members = 1 + rng.below(L::WIDTH);
        let lanes: Vec<(FaderLane, FaderLane)> = (0..members)
            .map(|_| {
                let mut fader = || FaderLane {
                    // A gain word with the sign bit set, which no session prepares, so the
                    // predicate's sign term is reached.
                    gain: rng.pick(&[1.0, 0.5, 2.0, 0.031_25, -0.5, -0.0]),
                    muted: rng.chance(1, 4),
                };
                (fader(), fader())
            })
            .collect();
        Self {
            stage: FaderRampStage::<L>::new(&lanes),
            frames,
            members,
            retune: Some(Rng(rng.next() | 1)),
            unmuted_claims: Cell::new(0),
        }
    }

    fn maybe_retune(&mut self) {
        let Some(rng) = self.retune.as_mut() else {
            return;
        };
        if !rng.chance(1, 8) {
            return;
        }
        let lane = rng.below(self.members);
        let window = rng.pick(&[0, 7, 40, 100, 300]);
        let lanes = selector(rng);
        if rng.chance(1, 2) {
            let gain = rng.pick(&[1.0, 0.25, 1.5, 0.0]);
            self.stage.set_fader_gain(lane, lanes, gain, window);
        } else {
            let muted = rng.chance(1, 2);
            self.stage.set_mute(lane, lanes, muted, window);
        }
    }
}

impl<L: Lane> Probe for FaderProbe<L> {
    fn lanes(&self) -> usize {
        L::WIDTH
    }

    fn frames(&self) -> usize {
        self.frames
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.maybe_retune();
        self.stage.process(left, right, self.frames);
    }

    fn lanes_at_rest(&self) -> u32 {
        let mask = self.stage.lanes_at_rest();
        let unmuted = (0..L::WIDTH).filter(|&lane| {
            mask & (1 << lane) != 0 && (0..2).any(|channel| !self.stage.muted[channel][lane])
        });
        self.unmuted_claims
            .set(self.unmuted_claims.get() + unmuted.count());
        mask
    }

    /// Both channels' ramp words, countdown, remembered gain, mute flag, mute ramp flag and mute
    /// mask bit. The kernel's `f32` countdown word is residue, recomputed from the `u32` before
    /// every read and absent from the lane export, so it is left out.
    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        for channel in 0..2 {
            let ramp = &self.stage.ramp[channel];
            out.push(bits::<L>(ramp.current, lane));
            out.push(bits::<L>(ramp.target, lane));
            out.push(bits::<L>(ramp.step, lane));
            out.push(mask_bit::<L>(ramp.mute, lane));
            out.push(self.stage.remaining[channel][lane]);
            out.push(self.stage.fader_gain[channel][lane].to_bits());
            out.push(u32::from(self.stage.muted[channel][lane]));
            out.push(u32::from(self.stage.mute_ramp[channel][lane]));
        }
    }

    fn fork(&self) -> Self {
        Self {
            stage: self.stage.clone(),
            frames: self.frames,
            members: self.members,
            retune: None,
            unmuted_claims: Cell::new(0),
        }
    }
}

fn fader_claim_holds<L: Lane>(label: &str) {
    let mut reach = Reach::default();
    let mut unmuted = 0;
    for seed in 0..4_u64 {
        let mut rng = Rng(0x1781_0201 + seed * 0x9e37);
        let frames = if seed % 2 == 0 { 32 } else { 128 };
        let mut probe = FaderProbe::<L>::new(&mut rng, frames);
        reach.add(rest_probe::check(
            &mut probe,
            0x0017_8102 + seed,
            Schedule {
                blocks: 6_144 / frames,
                longest_signal: 512 / frames,
                longest_silence: 1_024 / frames,
            },
        ));
        unmuted += probe.unmuted_claims.get();
    }
    eprintln!("{label}: {reach:?}, unmuted claims {unmuted}");
    reach.assert_reached(label);
    assert!(unmuted > 0, "{label}: no unmuted lane was claimed");
}

/// Gate 1 for the fader stage: at every block after which the stage sets a lane's rest bit, one
/// more block of `+0.0` leaves that lane's ramp words, countdowns, remembered gains and mute
/// state where they were and writes `+0.0` on both planes. Random gains (one with the sign bit),
/// mutes, padded lanes, and gain and mute retargets between blocks over windows that outlast a
/// block, so ramps are in flight at some boundaries and settled lanes share banks with ramping
/// ones.
#[test]
fn a_fader_lane_called_at_rest_stays_there_on_a_silent_block() {
    lane::each_vector_lane!(|L, W| fader_claim_holds::<L>(&format!("fader W={W}")));
}

// ---------------------------------------------------------------------------------------------
// Matrix
// ---------------------------------------------------------------------------------------------

struct MatrixProbe<L: Lane> {
    stage: MatrixStage<L>,
    frames: usize,
    members: usize,
    retune: Option<Rng>,
    /// Claims on a lane with some coefficient carrying the sign bit.
    negative_claims: Cell<usize>,
}

/// A random matrix: identity, a pan, a row with one negative coefficient, a row with two (the
/// `-0.0` row), signed zeros.
fn matrix(rng: &mut Rng) -> Matrix2x2 {
    let word = |rng: &mut Rng| rng.pick(&[1.0, 0.5, 0.0, -0.0, -0.25, -1.0, 0.707]);
    match rng.below(4) {
        0 => Matrix2x2::IDENTITY,
        1 => pan_matrix(0.3, -0.6).expect("a pan"),
        _ => Matrix2x2 {
            ll: word(rng),
            lr: word(rng),
            rl: word(rng),
            rr: word(rng),
        },
    }
}

impl<L: Lane> MatrixProbe<L> {
    fn new(rng: &mut Rng, frames: usize) -> Self {
        let members = 1 + rng.below(L::WIDTH);
        let lanes: Vec<(Matrix2x2, u32)> = (0..members)
            .map(|_| (matrix(rng), rng.pick(&[0, 24, 300])))
            .collect();
        Self {
            stage: MatrixStage::<L>::new(&lanes),
            frames,
            members,
            retune: Some(Rng(rng.next() | 1)),
            negative_claims: Cell::new(0),
        }
    }

    fn maybe_retune(&mut self) {
        let Some(rng) = self.retune.as_mut() else {
            return;
        };
        if !rng.chance(1, 8) {
            return;
        }
        let lane = rng.below(self.members);
        let target = matrix(rng);
        let window = rng.pick(&[0, 9, 64, 300]);
        self.stage
            .set_target_over(lane, target, window)
            .expect("a valid matrix");
    }
}

impl<L: Lane> Probe for MatrixProbe<L> {
    fn lanes(&self) -> usize {
        L::WIDTH
    }

    fn frames(&self) -> usize {
        self.frames
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.maybe_retune();
        self.stage.process(left, right, self.frames);
    }

    fn lanes_at_rest(&self) -> u32 {
        let mask = self.stage.lanes_at_rest();
        let current = self.stage.read_current();
        let negative = (0..L::WIDTH).filter(|&lane| {
            let m = current[lane];
            mask & (1 << lane) != 0
                && [m.ll, m.lr, m.rl, m.rr]
                    .iter()
                    .any(|word| word.is_sign_negative())
        });
        self.negative_claims
            .set(self.negative_claims.get() + negative.count());
        mask
    }

    /// The ramp words, settled coefficients, identity bit, window and countdown. The kernel's
    /// `f32` countdown word is residue, recomputed from the `u32` before every read and absent
    /// from the lane export, so it is left out.
    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        let stage = &self.stage;
        for index in 0..4 {
            out.push(bits::<L>(stage.ramp.current[index], lane));
            out.push(bits::<L>(stage.ramp.target[index], lane));
            out.push(bits::<L>(stage.ramp.step[index], lane));
        }
        for word in [stage.coef.ll, stage.coef.lr, stage.coef.rl, stage.coef.rr] {
            out.push(bits::<L>(word, lane));
        }
        out.push(mask_bit::<L>(stage.coef.identity, lane));
        out.push(stage.smoothing_samples[lane]);
        out.push(stage.remaining[lane]);
    }

    fn fork(&self) -> Self {
        Self {
            stage: self.stage.clone(),
            frames: self.frames,
            members: self.members,
            retune: None,
            negative_claims: Cell::new(0),
        }
    }
}

fn matrix_claim_holds<L: Lane>(label: &str) {
    let mut reach = Reach::default();
    let mut negative = 0;
    for seed in 0..4_u64 {
        let mut rng = Rng(0x1781_0301 + seed * 0x9e37);
        let frames = if seed % 2 == 0 { 32 } else { 128 };
        let mut probe = MatrixProbe::<L>::new(&mut rng, frames);
        reach.add(rest_probe::check(
            &mut probe,
            0x0017_8103 + seed,
            Schedule {
                blocks: 6_144 / frames,
                longest_signal: 512 / frames,
                longest_silence: 1_024 / frames,
            },
        ));
        negative += probe.negative_claims.get();
    }
    eprintln!("{label}: {reach:?}, negative-coefficient claims {negative}");
    reach.assert_reached(label);
    assert!(
        negative > 0,
        "{label}: no lane with a negative coefficient was claimed"
    );
}

/// Gate 1 for the matrix stage: at every block after which the stage sets a lane's rest bit, one
/// more block of `+0.0` leaves that lane's ramp and coefficient words, identity bit, window and
/// countdown where they were and writes `+0.0` on both planes. Random matrices with negative
/// coefficients and signed zeros (rows whose two coefficients both carry the sign bit write
/// `-0.0` and must be refused), padded identity lanes, and retargets between blocks, so ramps are
/// in flight at some boundaries; lanes with a negative coefficient must be claimed.
#[test]
fn a_matrix_lane_called_at_rest_stays_there_on_a_silent_block() {
    lane::each_vector_lane!(|L, W| matrix_claim_holds::<L>(&format!("matrix W={W}")));
}
