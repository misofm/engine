//! Issue #1745 gate 1: the EQ bank's per-lane rest claim, at every vector width the build has.
//!
//! A crate-internal module so that it binds [`PreparedParametricEq`] at `Simd4` and `Simd8` on an
//! AVX2 host (`lane::each_vector_lane!`), reads every state word of a lane, and forks a bank with
//! its bank-wide claim withdrawn. The shared driver is `conformance::rest_probe`.

use super::ramping_elision::{Rng, automatable, draw, request};
use super::*;
use conformance::rest_probe::{self, Probe, Reach, Schedule};
use effect_contract::{EffectTargetRequest, NativeEffectTargetPreparation};

const LAUNCH_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// One EQ bank under the driver.
struct EqProbe<L: Lane, const W: usize> {
    eq: PreparedParametricEq<L, W>,
    /// Frames per block. A block shorter than the 64-sample ramp leaves a ramp in flight at a
    /// block boundary; a quantum-long one never does.
    frames: usize,
    /// Every lane's current prepare-time values, so a retune starts from where the lane is.
    values: Vec<Vec<InitialParameterValue>>,
    /// Render through the collapsed body (`process_bank_mono`) instead of the dual one.
    mono: bool,
    /// Retune a random member between blocks. Off on a fork, whose block must be the claim's.
    retune: Option<Rng>,
    /// A per-lane input gain. The driver's levels never reach the band near `1e-15` where the
    /// per-word flush clears one integrator of a low section and keeps the other; a gain of
    /// `1e-6` or `1e-8` puts its `1e-9` level there. A gain maps `+0.0` to `+0.0`, so a silent
    /// block stays silent.
    gain: Vec<f32>,
}

/// A field-by-field copy of a channel: every word, derived flags included.
fn copy_channel<L: Lane, const W: usize>(channel: &Channel<L, W>) -> Channel<L, W> {
    Channel {
        sections: channel.sections,
        remaining: channel.remaining,
        targets: channel.targets,
        identity: channel.identity,
        dry: channel.dry,
        silence: channel.silence,
    }
}

impl<L: Lane, const W: usize> EqProbe<L, W> {
    /// Retunes one random member's automatable parameter, about one block in ten: a ramp in
    /// flight on that lane, and its neighbours through the ramped kernel with a `+0.0` step.
    fn maybe_retune(&mut self) {
        let Some(rng) = self.retune.as_mut() else {
            return;
        };
        if !rng.chance(0.1) {
            return;
        }
        let members: Vec<usize> = (0..W).filter(|&lane| self.eq.active[lane]).collect();
        let lane = members[rng.below(members.len())];
        let Some((candidate, targets)) =
            retune_targets(rng, self.eq.sample_rate, &self.values[lane])
        else {
            return;
        };
        if targets
            .iter()
            .all(|target| self.eq.apply_prepared_target_lane(lane, target).is_ok())
        {
            self.values[lane] = candidate;
        }
    }
}

/// A retune of one random automatable parameter of a member whose current prepare-time values
/// are `values`, on both channels: the new values and their prepared targets.
fn retune_targets(
    rng: &mut Rng,
    sample_rate: u32,
    values: &[InitialParameterValue],
) -> Option<(Vec<InitialParameterValue>, Vec<PreparedEffectTarget>)> {
    let parameters = PARAMETRIC_EQ_DESCRIPTOR.parameters.len();
    let index = loop {
        let index = rng.below(parameters);
        if automatable(index) {
            break index;
        }
    };
    let mut candidate = values.to_vec();
    let mut changed = vec![false; parameters * 2];
    for side in 0..2 {
        candidate[index * 2 + side].value = draw(rng, index);
        changed[index * 2 + side] = true;
    }
    let mut targets = [PreparedEffectTarget {
        slot: 0,
        channel: ParameterChannel::Left,
        words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
    }; EQ_SECTION_COUNT * 2];
    let count = ParametricEqFactory
        .prepare_targets(
            EffectTargetRequest {
                sample_rate,
                values: &candidate,
                changed: &changed,
            },
            &mut targets,
        )
        .ok()?;
    Some((candidate, targets[..count].to_vec()))
}

impl<L: Lane, const W: usize> Probe for EqProbe<L, W> {
    fn lanes(&self) -> usize {
        W
    }

    fn frames(&self) -> usize {
        self.frames
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        self.maybe_retune();
        for (index, (l, r)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
            let gain = self.gain[index % W];
            *l *= gain;
            *r *= gain;
        }
        let width = BankWidth::for_lanes(W).expect("a bank width");
        let offsets = vec![0_u32; W + 1];
        let block = EffectBankProcessBlock::new(
            left,
            right,
            None,
            self.frames as u32,
            width,
            0,
            &[],
            &offsets,
            self.eq.quantum(),
        )
        .expect("a bank block");
        if self.mono {
            self.eq.process_bank_mono(block);
            // The seam's copy: a collapsed track's right plane is its left one.
            right.copy_from_slice(left);
        } else {
            self.eq.process_bank(block);
        }
    }

    fn lanes_at_rest(&self) -> u32 {
        PreparedNativeEffectBank::lanes_at_rest(&self.eq)
    }

    /// The lane's per-track state payload, both channels, minus the silence counter (the one
    /// clock), plus the ramp target words the payload recomputes rather than carries.
    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        for channel in [&self.eq.left, &self.eq.right] {
            let mut words = [0_u32; STATE_LANE_WORDS];
            channel.snapshot_track(lane, &mut words);
            out.extend(
                words
                    .iter()
                    .enumerate()
                    .filter(|&(word, _)| word != STATE_SILENCE_WORD)
                    .map(|(_, bits)| *bits),
            );
            for section in &channel.sections {
                for index in 0..EQ_COEFFICIENT_WORDS {
                    out.push(lane_get(coef_word(&section.target, index), lane).to_bits());
                }
            }
        }
    }

    fn fork(&self) -> Self {
        let mut eq = copy_bank(&self.eq);
        // Withdrawn: the check's block runs the whole kernel.
        eq.silent_fixed_point = false;
        Self {
            eq,
            frames: self.frames,
            values: self.values.clone(),
            mono: self.mono,
            retune: None,
            gain: self.gain.clone(),
        }
    }
}

/// A field-by-field copy of a bank: every word, the bank-wide witness included.
fn copy_bank<L: Lane, const W: usize>(
    eq: &PreparedParametricEq<L, W>,
) -> PreparedParametricEq<L, W> {
    PreparedParametricEq {
        sample_rate: eq.sample_rate,
        bypass: eq.bypass,
        initial: eq.initial,
        initial_words: eq.initial_words,
        left: copy_channel(&eq.left),
        right: copy_channel(&eq.right),
        active: eq.active,
        silent_fixed_point: eq.silent_fixed_point,
        rest: eq.rest.clone(),
    }
}

/// One dual-body bank block of `frames` frames, no automation.
fn process<L: Lane, const W: usize>(
    eq: &mut PreparedParametricEq<L, W>,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
) {
    let width = BankWidth::for_lanes(W).expect("a bank width");
    let offsets = vec![0_u32; W + 1];
    let block = EffectBankProcessBlock::new(
        &mut left[..frames * W],
        &mut right[..frames * W],
        None,
        frames as u32,
        width,
        0,
        &[],
        &offsets,
        eq.quantum(),
    )
    .expect("a bank block");
    eq.process_bank(block);
}

/// One member's prepare-time values; `symmetric` gives both channels the same ones.
fn configuration(rng: &mut Rng, symmetric: bool) -> Vec<InitialParameterValue> {
    let parameters = PARAMETRIC_EQ_DESCRIPTOR.parameters.len();
    let mut values = Vec::with_capacity(parameters * 2);
    for index in 0..parameters {
        let left = draw(rng, index);
        let right = if symmetric || (index < 24 && index % 6 < 2) {
            left
        } else {
            draw(rng, index)
        };
        for (channel, value) in [
            (ParameterChannel::Left, left),
            (ParameterChannel::Right, right),
        ] {
            values.push(InitialParameterValue {
                parameter_index: index as u32,
                channel,
                value,
            });
        }
    }
    values
}

/// A bank of random members (the rest padded with clones), at a random launch rate.
fn probe<L: Lane, const W: usize>(rng: &mut Rng, mono: bool, frames: usize) -> EqProbe<L, W> {
    loop {
        let rate = LAUNCH_RATES[rng.below(LAUNCH_RATES.len())];
        let members = 1 + rng.below(W);
        let configured: Vec<Vec<InitialParameterValue>> = (0..members)
            .map(|_| {
                let symmetric = mono || rng.chance(0.3);
                configuration(rng, symmetric)
            })
            .collect();
        let values: Vec<Vec<InitialParameterValue>> = (0..W)
            .map(|lane| configured[lane.min(members - 1)].clone())
            .collect();
        let requests: Vec<_> = values.iter().map(|lane| request(lane, rate)).collect();
        let mask: Vec<bool> = (0..W).map(|lane| lane < members).collect();
        let Ok(metadata) = expected_prepared_metadata(&PARAMETRIC_EQ_DESCRIPTOR, requests[0])
        else {
            continue;
        };
        let Ok(eq) = prepare_width::<L, W>(metadata, &requests, &mask) else {
            continue;
        };
        let retune = (!mono).then(|| Rng(rng.next() | 1));
        let gain = (0..W)
            .map(|_| [1.0, 1.0, 1.0e-6, 1.0e-8][rng.below(4)])
            .collect();
        return EqProbe {
            eq,
            frames,
            values,
            mono,
            retune,
            gain,
        };
    }
}

fn claim_holds<L: Lane, const W: usize>(label: &str) {
    // The same span of time at both block lengths: about 0.6 s at 48 kHz, the silence runs long
    // enough for the joint flush to arm (85 ms) and for most designs to come to rest after it.
    let schedule = |frames: usize| Schedule {
        blocks: 30_720 / frames,
        longest_signal: 768 / frames,
        longest_silence: 11_520 / frames,
    };
    let mut reach = Reach::default();
    for seed in 0..4_u64 {
        let mut rng = Rng(0x1745_0001 + seed * 0x9e37);
        let mono = seed == 3;
        let frames = if seed % 2 == 0 { 32 } else { 128 };
        let mut bank = probe::<L, W>(&mut rng, mono, frames);
        reach.add(rest_probe::check(
            &mut bank,
            0x1745 + seed,
            schedule(frames),
        ));
    }
    eprintln!("{label}: {reach:?}");
    reach.assert_reached(label);
}

/// Gate 1: at every block after which the EQ bank sets a lane's rest bit, one more block of
/// `+0.0` through the whole kernel leaves that lane's state bits (the payload minus the silence
/// clock, plus the ramp targets) where they were and writes `+0.0` on both planes. Random members
/// and padded clones, random rates, retunes between blocks (ramps in flight, and neighbours
/// through the ramped kernel), the dual body and, on one seed per width, the collapsed one.
#[test]
fn a_lane_the_eq_bank_calls_at_rest_stays_there_on_a_silent_block() {
    lane::each_vector_lane!(|L, W| claim_holds::<L, W>(&width_label(W)));
}

/// Every word of a bank a render or a control call can move: per lane, both channels' state
/// payload words (silence counter included), ramp target words and band enables and families;
/// per channel, the identity flags and dry masks; and the bank's fixed-point witness. The rest
/// planes are scratch and `initial`, `initial_words` and `active` are prepare-time constants.
fn bank_words<L: Lane, const W: usize>(eq: &PreparedParametricEq<L, W>) -> Vec<u32> {
    let mut out = vec![u32::from(eq.silent_fixed_point)];
    for channel in [&eq.left, &eq.right] {
        for lane in 0..W {
            let mut words = [0_u32; STATE_LANE_WORDS];
            channel.snapshot_track(lane, &mut words);
            out.extend(words);
            for section in 0..EQ_SECTION_COUNT {
                for index in 0..EQ_COEFFICIENT_WORDS {
                    let word = coef_word(&channel.sections[section].target, index);
                    out.push(lane_get(word, lane).to_bits());
                }
                let band = channel.targets[lane][section];
                out.push(u32::from(band.enabled));
                out.push(band.kind as u32);
            }
        }
        for section in 0..EQ_SECTION_COUNT {
            out.push(u32::from(channel.identity[section]));
            let dry = L::select(channel.dry[section], L::splat(1.0), L::zero());
            out.extend((0..W).map(|lane| lane_get(dry, lane).to_bits()));
        }
    }
    out
}

/// One frame count for a block: short blocks of several lengths and the whole quantum.
fn block_frames(rng: &mut Rng) -> usize {
    [1, 17, 32, 64, 100, 128][rng.below(6)]
}

/// `bank` renders `blocks` blocks of `+0.0`; `twin` skips each with `advance_clocks` over every
/// lane. Every word of the two is then equal.
fn zero_blocks_match<L: Lane, const W: usize>(
    rng: &mut Rng,
    bank: &mut PreparedParametricEq<L, W>,
    twin: &mut PreparedParametricEq<L, W>,
    blocks: usize,
    label: &str,
) {
    let mut left = vec![0.0_f32; 128 * W];
    let mut right = vec![0.0_f32; 128 * W];
    for block in 0..blocks {
        let frames = block_frames(rng);
        left.fill(0.0);
        right.fill(0.0);
        process(bank, &mut left, &mut right, frames);
        PreparedNativeEffectBank::advance_clocks(twin, all_lanes::<W>(), frames as u32);
        let (rendered, skipped) = (bank_words(bank), bank_words(twin));
        let first = rendered.iter().zip(&skipped).position(|(a, b)| a != b);
        assert_eq!(
            first, None,
            "{label}: block {block} of {frames} frames, first word"
        );
    }
}

fn clocks_advance_as_a_zero_block<L: Lane, const W: usize>(label: &str) {
    let all = all_lanes::<W>();
    for seed in 0..3_u64 {
        let mut rng = Rng(0x1747_0001 + seed * 0x9e37);
        let mut bank = probe::<L, W>(&mut rng, false, 128);
        bank.retune = None;
        let eq = &mut bank.eq;
        // Fresh: at rest with every counter at `0`, so the run crosses the arming edge
        // (`N_SILENCE`, at most 8,192 frames at a launch rate) part way through.
        assert_eq!(PreparedNativeEffectBank::lanes_at_rest(eq), all, "{label}");
        let mut twin = copy_bank(eq);
        let blocks = 2 * lane::silence_frames(eq.sample_rate) as usize / 64 + 8;
        zero_blocks_match(&mut rng, eq, &mut twin, blocks, &format!("{label} fresh"));
        // Counters that differ by lane and by channel, some a few frames short of the arming
        // edge, some a block short of the saturation `2^24`.
        let window = lane::silence_frames(eq.sample_rate) as f32;
        for (side, channel) in [&mut eq.left, &mut eq.right].into_iter().enumerate() {
            for lane in 0..W {
                let run = match (lane + side) % 4 {
                    0 => (lane * 7) as f32,
                    1 => window - (lane + 1) as f32,
                    2 => window + 3.0,
                    _ => SILENCE_SATURATION - (lane * 40) as f32,
                };
                lane_set(&mut channel.silence, lane, run);
            }
        }
        // Withdrawn, so the first block runs the kernel over these counters.
        eq.silent_fixed_point = false;
        let mut twin = copy_bank(eq);
        zero_blocks_match(&mut rng, eq, &mut twin, 24, &format!("{label} differing"));
        // A lane outside the mask keeps its counters; a lane inside it advances.
        let mut twin = copy_bank(eq);
        twin.left.silence = L::zero();
        twin.right.silence = L::zero();
        PreparedNativeEffectBank::advance_clocks(&mut twin, all & !1, 16);
        for channel in [&twin.left, &twin.right] {
            assert_eq!(lane_get(channel.silence, 0).to_bits(), 0, "{label}: lane 0");
            if W > 1 {
                assert_eq!(lane_get(channel.silence, 1), 16.0, "{label}: lane 1");
            }
        }
    }
}

/// Issue #1747: a resting bank that skips a block through `advance_clocks` ends bit-identical to
/// one that rendered the block of `+0.0` -- every payload word and every internal word, across
/// the silence counters' arming edge, at their saturation, at block lengths shorter than the
/// quantum, and with lanes whose counters differ.
#[test]
fn advance_clocks_leaves_a_resting_bank_where_a_rendered_zero_block_does() {
    lane::each_vector_lane!(|L, W| clocks_advance_as_a_zero_block::<L, W>(&width_label(W)));
}

/// The EQ's payload sizes, as the contract's swap takes them.
const SWAP_SIZES: effect_contract::StatePayloadSizes = effect_contract::StatePayloadSizes {
    common_bytes: STATE_SIZES.common as u32,
    left_bytes: STATE_SIZES.left as u32,
    right_bytes: STATE_SIZES.right as u32,
};

/// Lane `lane`'s whole state payload: `[common, left, right]`.
fn swap_payload(bank: &dyn PreparedNativeEffectBank, lane: usize) -> [Vec<u8>; 3] {
    let mut common = vec![0; STATE_SIZES.common];
    let mut left = vec![0; STATE_SIZES.left];
    let mut right = vec![0; STATE_SIZES.right];
    bank.snapshot_track_state_payload(
        lane as u32,
        StatePayloadOutput {
            common: &mut common,
            left: &mut left,
            right: &mut right,
        },
    )
    .expect("a lane snapshot");
    [common, left, right]
}

/// Two full banks of one slot shape (rate, quantum, width), every lane a member with its own
/// random targets, the general bands' enables and families included.
fn swap_banks<L: Lane, const W: usize>(
    rng: &mut Rng,
) -> (
    [PreparedParametricEq<L, W>; 2],
    Vec<Vec<InitialParameterValue>>,
    u32,
) {
    loop {
        let rate = LAUNCH_RATES[rng.below(LAUNCH_RATES.len())];
        let configs: Vec<Vec<InitialParameterValue>> = (0..2 * W)
            .map(|_| {
                let symmetric = rng.chance(0.3);
                configuration(rng, symmetric)
            })
            .collect();
        let prepare = |members: &[Vec<InitialParameterValue>]| {
            let requests: Vec<_> = members.iter().map(|lane| request(lane, rate)).collect();
            let metadata =
                expected_prepared_metadata(&PARAMETRIC_EQ_DESCRIPTOR, requests[0]).ok()?;
            prepare_width::<L, W>(metadata, &requests, &[true; W]).ok()
        };
        let (Some(a), Some(b)) = (prepare(&configs[..W]), prepare(&configs[W..])) else {
            continue;
        };
        return ([a, b], configs, rate);
    }
}

/// Swaps lane `first.1` of bank `first.0` with lane `second.1` of bank `second.0` through the
/// contract's swap: `second = None` when both are one bank.
fn swap<L: Lane, const W: usize>(
    banks: &mut [PreparedParametricEq<L, W>; 2],
    first: (usize, usize),
    second: (usize, usize),
    scratch: &mut [u8],
) -> Result<(), effect_contract::LaneSwapRefusal> {
    let [a, b] = banks;
    let (one, other) = if first.0 == 0 { (a, b) } else { (b, a) };
    let other: Option<&mut dyn PreparedNativeEffectBank> = if first.0 == second.0 {
        None
    } else {
        Some(other)
    };
    effect_contract::swap_bank_lanes(
        one,
        first.1,
        other,
        second.1,
        STATE_LAYOUT_VERSION,
        SWAP_SIZES,
        scratch,
    )
}

/// [`swap`], required to succeed and to allocate nothing, measured after the same swap on copies
/// has warmed the process statics.
fn measured_swap<L: Lane, const W: usize>(
    banks: &mut [PreparedParametricEq<L, W>; 2],
    first: (usize, usize),
    second: (usize, usize),
    scratch: &mut [u8],
    label: &str,
) {
    let mut copies = [copy_bank(&banks[0]), copy_bank(&banks[1])];
    assert_eq!(
        swap(&mut copies, first, second, scratch),
        Ok(()),
        "{label}: warm-up"
    );
    let mark = bench_support::alloc::current_thread_counters();
    let result = swap(banks, first, second, scratch);
    let delta = bench_support::alloc::current_thread_delta_since(mark);
    assert_eq!(result, Ok(()), "{label}: the swap");
    assert_eq!(
        (delta.allocations, delta.deallocations, delta.reallocations),
        (0, 0, 0),
        "{label}: the swap allocated"
    );
}

/// The strips' run: `banks` move strips between lanes, `twins` keep every strip at home (strip
/// `s` at bank `s / W`, lane `s % W`). `at[s]` is strip `s`'s place in `banks`.
struct SwapRun<L: Lane, const W: usize> {
    banks: [PreparedParametricEq<L, W>; 2],
    twins: [PreparedParametricEq<L, W>; 2],
    at: Vec<(usize, usize)>,
    values: Vec<Vec<InitialParameterValue>>,
    rate: u32,
}

impl<L: Lane, const W: usize> SwapRun<L, W> {
    /// Retunes strip `strip` in both runs, wherever it sits; `true` if the retune applied.
    fn retune(&mut self, rng: &mut Rng, strip: usize) -> bool {
        let Some((candidate, targets)) = retune_targets(rng, self.rate, &self.values[strip]) else {
            return false;
        };
        let (bank, lane) = self.at[strip];
        let moved = &mut self.banks[bank];
        let home = &mut self.twins[strip / W];
        let mut applied = true;
        for target in &targets {
            let one = moved.apply_prepared_target_lane(lane, target);
            let two = home.apply_prepared_target_lane(strip % W, target);
            assert_eq!(one, two, "a retune applies alike in both runs");
            applied &= one.is_ok();
        }
        if applied {
            self.values[strip] = candidate;
        }
        applied
    }

    /// One block of random input per strip (now and then silence) through all four banks; every
    /// strip's output is bit-equal in the two runs.
    fn block(&mut self, rng: &mut Rng, frames: usize, label: &str) {
        let strips = 2 * W;
        let mut input = vec![[vec![0.0_f32; frames], vec![0.0_f32; frames]]; strips];
        for planes in &mut input {
            if rng.chance(0.15) {
                continue;
            }
            for word in planes.iter_mut().flatten() {
                *word = 2.0 * rng.unit() - 1.0;
            }
        }
        let mut moved = [
            [vec![0.0; frames * W], vec![0.0; frames * W]],
            [vec![0.0; frames * W], vec![0.0; frames * W]],
        ];
        let mut home = moved.clone();
        for (strip, planes) in input.iter().enumerate() {
            let (bank, lane) = self.at[strip];
            for side in 0..2 {
                for frame in 0..frames {
                    moved[bank][side][frame * W + lane] = planes[side][frame];
                    home[strip / W][side][frame * W + strip % W] = planes[side][frame];
                }
            }
        }
        for bank in 0..2 {
            let [left, right] = &mut moved[bank];
            process(&mut self.banks[bank], left, right, frames);
            let [left, right] = &mut home[bank];
            process(&mut self.twins[bank], left, right, frames);
        }
        for strip in 0..strips {
            let (bank, lane) = self.at[strip];
            for side in 0..2 {
                for frame in 0..frames {
                    assert_eq!(
                        moved[bank][side][frame * W + lane].to_bits(),
                        home[strip / W][side][frame * W + strip % W].to_bits(),
                        "{label}: strip {strip}, channel {side}, frame {frame}"
                    );
                }
            }
        }
    }

    /// `true` if strip `strip` has a ramp in flight in `banks`.
    fn in_flight(&self, strip: usize) -> bool {
        let (bank, lane) = self.at[strip];
        let eq = &self.banks[bank];
        [&eq.left, &eq.right]
            .iter()
            .any(|channel| channel.remaining.iter().any(|section| section[lane] > 0))
    }

    /// Retunes `strips` until each has a ramp in flight, then renders one block shorter than the
    /// ramp, so the ramps are still in flight after it.
    fn ramps_in_flight(&mut self, rng: &mut Rng, strips: [usize; 2], label: &str) {
        for strip in strips {
            for _ in 0..64 {
                if self.in_flight(strip) {
                    break;
                }
                self.retune(rng, strip);
            }
        }
        let frames = [16, 32, 48][rng.below(3)];
        self.block(rng, frames, label);
        for strip in strips {
            assert!(
                self.in_flight(strip),
                "{label}: strip {strip} has no ramp in flight"
            );
        }
    }

    /// `count` blocks of random lengths, retuning a random strip now and then.
    fn blocks(&mut self, rng: &mut Rng, count: usize, label: &str) {
        for _ in 0..count {
            if rng.chance(0.4) {
                let strip = rng.below(2 * W);
                self.retune(rng, strip);
            }
            let frames = block_frames(rng);
            self.block(rng, frames, label);
        }
    }

    /// Swaps the strips at `first` and `second` in `banks`, and follows them.
    fn swap(
        &mut self,
        first: (usize, usize),
        second: (usize, usize),
        scratch: &mut [u8],
        label: &str,
    ) {
        measured_swap(&mut self.banks, first, second, scratch, label);
        for place in &mut self.at {
            if *place == first {
                *place = second;
            } else if *place == second {
                *place = first;
            }
        }
    }

    /// Every strip's payload, where it sits now, equals its twin's.
    fn assert_payloads(&self, label: &str) {
        for strip in 0..2 * W {
            let (bank, lane) = self.at[strip];
            assert_eq!(
                swap_payload(&self.banks[bank], lane),
                swap_payload(&self.twins[strip / W], strip % W),
                "{label}: strip {strip}'s payload"
            );
        }
    }
}

/// `true` if lanes `first` and `second` of `eq` were prepared with a different enable or family
/// on some general band: a swap that left `initial` behind would restore, retune and reset them
/// against the other strip's bands.
fn general_bands_differ<L: Lane, const W: usize>(
    eq: &PreparedParametricEq<L, W>,
    first: usize,
    second: usize,
) -> bool {
    (0..2).any(|channel| {
        (HPF_SECTION + 1..LPF_SECTION).any(|section| {
            let (one, two) = (
                eq.initial[first][channel][section],
                eq.initial[second][channel][section],
            );
            one.enabled != two.enabled || one.kind != two.kind
        })
    })
}

fn swapped_lanes_render_as_unswapped<L: Lane, const W: usize>(label: &str) {
    bench_support::alloc::assert_installed();
    bench_support::alloc::set_mode(bench_support::alloc::Mode::Count);
    for seed in 0..3_u64 {
        let mut rng = Rng(0x1747_0101 + seed * 0x9e37);
        let (banks, values, rate) = swap_banks::<L, W>(&mut rng);
        let scratch_bytes = effect_contract::swap_bank_lanes_scratch_bytes(&banks[0], SWAP_SIZES)
            .expect("the scratch size");
        let mut scratch = vec![0_u8; scratch_bytes];
        let twins = [copy_bank(&banks[0]), copy_bank(&banks[1])];
        let mut run = SwapRun {
            banks,
            twins,
            at: (0..2 * W).map(|strip| (strip / W, strip % W)).collect(),
            values,
            rate,
        };
        let label = format!("{label} seed {seed}");
        run.blocks(&mut rng, 6, &label);
        // Within bank A: two distinct lanes, each with a ramp in flight.
        let i = rng.below(W);
        let j = (i + 1 + rng.below(W - 1)) % W;
        assert!(
            general_bands_differ(&run.banks[0], i, j),
            "{label}: the lanes swapped within bank A share every general band's enable and family"
        );
        run.ramps_in_flight(&mut rng, [i, j], &label);
        run.swap((0, i), (0, j), &mut scratch, &label);
        run.blocks(&mut rng, 6, &label);
        // Across A and B: whichever strips now sit at the two places.
        let (first, second) = ((0, rng.below(W)), (1, rng.below(W)));
        let strips = [first, second].map(|place| {
            run.at
                .iter()
                .position(|&at| at == place)
                .expect("a strip at every place")
        });
        run.ramps_in_flight(&mut rng, strips, &label);
        run.swap(first, second, &mut scratch, &label);
        // Long enough for every ramp to land and for the landed words to render.
        run.blocks(&mut rng, 8, &label);
        run.assert_payloads(&label);
        // The prepared defaults moved with each strip: a full reset re-seeds every strip from its
        // own bands and designed words, wherever it now sits.
        for eq in run.banks.iter_mut().chain(run.twins.iter_mut()) {
            PreparedNativeEffectBank::reset(eq, ResetKind::FullToDefaults);
        }
        let frames = block_frames(&mut rng);
        run.block(&mut rng, frames, &format!("{label} after a full reset"));
        run.assert_payloads(&format!("{label} after a full reset"));
    }
}

/// Issue #1747 gate 1 for the EQ: two banks of one slot shape render random input with per-lane
/// targets; two lanes swap through `effect_contract::swap_bank_lanes`, once within a bank and
/// once across the two, each with ramps in flight on both lanes; inputs and later retunes follow
/// the strips. The lanes swapped within a bank differ in some general band's enable or family.
/// Every strip's output is bit-equal to a twin run that never swaps, every strip's payload is
/// equal at the end, and the swap allocates nothing. Last, every bank and twin takes a full reset
/// and renders one more block, bit-equal again, so a swap that left a strip's prepared defaults
/// behind is red.
#[test]
fn swapped_eq_lanes_render_exactly_as_unswapped_ones() {
    lane::each_vector_lane!(|L, W| swapped_lanes_render_as_unswapped::<L, W>(&width_label(W)));
}
