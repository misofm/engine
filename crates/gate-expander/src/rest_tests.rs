//! Issue #1746: the gate bank's per-lane rest claim and its bank-wide silent fast path, at
//! `Simd4` and `Simd8`.
//!
//! Crate-internal for the same reason as `padding_tests`: the tests bind the bank at every vector
//! width the build has, read every state word of a lane (padded lanes are not tracks, so no
//! payload carries them), and read the test-only admission counter and fast-path switch.
//!
//! The gate's one clock is bank-level: `ramp_frames_left`, the countdown that selects the ramping
//! segment. It is left out of [`Bank::lane_state`] and is compared, with everything else, by the
//! fast-path differential.

use super::padding_tests::{
    Draw, LINKS, QUANTUM, RATES, Values, prefix_mask, request, values_from,
};
use super::*;
use conformance::rest_probe::{self, Reach, Schedule};
use effect_contract::PreparedPorts;

/// Gates 2 and 3's block length. Not a divisor of the 64-sample smoothing, so a ramp is still in
/// flight at the end of the block it starts in.
const FRAMES: usize = 48;

/// Gate 1's block length: a 64-sample ramp that starts on a block's first sample ends blocks with
/// 43, 22 and then 1 sample left, so the predicate sees a ramp whose last step is still to come
/// (`remaining == 1`), which a block of 48 never leaves.
const PROBE_FRAMES: usize = 21;

/// The bank width of `L`.
fn width_of<L: Lane>() -> BankWidth {
    BankWidth::ALL
        .iter()
        .copied()
        .find(|width| width.lanes() as usize == L::WIDTH)
        .expect("a bank width")
}

/// A bank of `L::WIDTH` lanes built as `bind_bank` builds one: active lane `l` runs the next
/// member, and a padded lane clones member 0.
fn bound<L: Lane>(
    members: &[Values],
    mask: &[bool],
    rate: u32,
    link: LinkMode,
) -> PreparedGate<L, false> {
    let metadata =
        expected_prepared_metadata(&GATE_EXPANDER_DESCRIPTOR, request(&members[0], rate, link))
            .expect("metadata");
    let mut defaults = [initial_defaults(&members[0]).expect("valid"); MAX_WIDTH];
    let mut next = 0;
    for (lane, active) in mask.iter().enumerate() {
        if *active {
            defaults[lane] = initial_defaults(&members[next]).expect("valid");
            next += 1;
        }
    }
    PreparedGate::<L, false>::new(&metadata, Some(width_of::<L>()), defaults)
        .expect("a bank")
        .with_active_lanes(active_bits(mask))
}

/// A copy of `gate`: every field, the clock included.
fn copy<L: Lane>(gate: &PreparedGate<L, false>) -> PreparedGate<L, false> {
    PreparedGate {
        sample_rate: gate.sample_rate,
        automation_capacity: gate.automation_capacity,
        bank_width: gate.bank_width,
        defaults: gate.defaults,
        coef: gate.coef,
        state: gate.state,
        timing: gate.timing,
        ramp_frames_left: gate.ramp_frames_left,
        active: gate.active,
    }
}

/// Every word of lane `lane` of both channels: state (gain, open flag, hold countdown, every
/// field of every ramp) and the coefficients, which render never writes.
fn lane_words<L: Lane, const CONNECTED: bool>(
    gate: &PreparedGate<L, CONNECTED>,
    lane: usize,
    out: &mut Vec<u32>,
) {
    for channel in 0..2 {
        let state = gate.state[channel].lanes()[lane];
        let coef = gate.coef[channel].lanes()[lane];
        out.extend(
            [
                state.gain_db,
                state.hysteresis.open,
                state.hysteresis.hold,
                coef.attack,
                coef.release,
                coef.hold_samples,
                coef.bypass,
                coef.link_max,
                coef.link_avg,
            ]
            .map(f32::to_bits),
        );
        for ramp in state.ramps {
            out.extend([ramp.current, ramp.target, ramp.step, ramp.remaining].map(f32::to_bits));
        }
    }
}

/// Renders one bank block with `forbid` holding the fast-path switch.
fn render_block<L: Lane>(
    gate: &mut PreparedGate<L, false>,
    left: &mut [f32],
    right: &mut [f32],
    first_sample: u64,
    spans: &[PreparedAutomationSpan],
    offsets: &[u32],
    forbid: bool,
) -> BankProcessReport
where
    PreparedGate<L, false>: PreparedNativeEffectBank,
{
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        (left.len() / L::WIDTH) as u32,
        width_of::<L>(),
        first_sample,
        spans,
        offsets,
        QUANTUM,
    )
    .expect("a bank block");
    SILENT_FORBIDDEN.with(|cell| cell.set(forbid));
    let report = gate.process_bank(block);
    SILENT_FORBIDDEN.with(|cell| cell.set(false));
    report
}

fn admissions() -> usize {
    SILENT_ADMISSIONS.with(core::cell::Cell::get)
}

/// Sets parameter `parameter` to `value` on both channels.
fn set(values: &mut Values, parameter: usize, value: f32) {
    set_pair(values, parameter, value, value);
}

/// Sets parameter `parameter` to `left` and `right` on the two channels.
fn set_pair(values: &mut Values, parameter: usize, left: f32, right: f32) {
    values[parameter * 2].value = left;
    values[parameter * 2 + 1].value = right;
}

/// Fast attack and release and a short hold, so a lane comes back to rest inside a case. The two
/// channels take different times, with the slower release on the left, so a predicate that read
/// one channel's coefficients for the other would claim the right channel at rest while its gain
/// still moves.
fn fast(values: &mut Values, hold_ms: f32) {
    set_pair(values, 4, 0.1, 1.0);
    set_pair(values, 5, hold_ms, hold_ms * 2.0);
    set_pair(values, 6, 40.0, 5.0);
}

// ---------------------------------------------------------------------------------------------
// Gate 1
// ---------------------------------------------------------------------------------------------

/// One bank under the shared driver.
struct Bank<L: Lane> {
    gate: PreparedGate<L, false>,
    first_sample: u64,
    /// Draws a target now and then, so ramps are in flight at block ends, and plants a NaN gain
    /// now and then; `None` in a fork, whose one block is the claim's block of `+0.0` with no
    /// automation.
    automation: Option<Draw>,
    /// A fork renders the whole kernel: the fast path is forbidden.
    forbid: bool,
}

impl<L: Lane> rest_probe::Probe for Bank<L>
where
    PreparedGate<L, false>: PreparedNativeEffectBank,
{
    fn lanes(&self) -> usize {
        L::WIDTH
    }

    fn frames(&self) -> usize {
        PROBE_FRAMES
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        let mut offsets = vec![0_u32; L::WIDTH + 1];
        let mut spans = Vec::new();
        for lane in 0..L::WIDTH {
            if let Some(draw) = self.automation.as_mut()
                && self.gate.is_active(lane)
                && draw.chance(1, 8)
            {
                spans.extend(draw.spans(self.first_sample));
            }
            offsets[lane + 1] = spans.len() as u32;
        }
        render_block(
            &mut self.gate,
            left,
            right,
            self.first_sample,
            &spans,
            &offsets,
            self.forbid,
        );
        // Now and then a NaN gain word planted after the block (the D7 case: a non-finite state
        // behind a finite output), which the next block's `finish_block` resets, so the
        // predicate must refuse the lane.
        if let Some(draw) = self.automation.as_mut()
            && draw.chance(1, 64)
        {
            let (lane, channel) = (draw.below(L::WIDTH), draw.below(2));
            self.gate.inject_nonfinite_gain(lane, channel);
        }
        self.first_sample += PROBE_FRAMES as u64;
    }

    fn lanes_at_rest(&self) -> u32 {
        PreparedNativeEffectBank::lanes_at_rest(&self.gate)
    }

    /// Every word of the lane but the bank's clock, `ramp_frames_left`.
    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        lane_words(&self.gate, lane, out);
    }

    fn fork(&self) -> Self {
        Self {
            gate: copy(&self.gate),
            first_sample: self.first_sample,
            automation: None,
            forbid: true,
        }
    }
}

/// One randomized case at `L`: random members (most with fast times, so a gating lane comes back
/// to rest inside the case), a random rate and link mode.
fn case<L: Lane>(seed: u64) -> Reach
where
    PreparedGate<L, false>: PreparedNativeEffectBank,
{
    let lanes = L::WIDTH;
    let mut draw = Draw::new(seed);
    let count = 1 + draw.below(lanes);
    let members: Vec<Values> = (0..count)
        .map(|_| {
            let mut values = draw.values();
            if draw.chance(3, 4) {
                let hold = [0.0, 1.0, 10.0][draw.below(3)];
                fast(&mut values, hold);
            }
            values
        })
        .collect();
    let mask: Vec<bool> = (0..lanes).map(|lane| lane < count).collect();
    let rate = RATES[draw.below(RATES.len())];
    let link = LINKS[draw.below(LINKS.len())];
    let mut bank = Bank::<L> {
        gate: bound::<L>(&members, &mask, rate, link),
        first_sample: 0,
        automation: Some(Draw::new(!seed)),
        forbid: false,
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
/// and writes `+0.0`, at `Simd4` and `Simd8`.**
#[test]
fn a_lane_reported_at_rest_stays_at_rest_through_silence() {
    lane::each_vector_lane!(|L| {
        let mut reach = Reach::default();
        for seed in 0..6_u64 {
            reach.add(case::<L>(0x1746_0000 + seed));
        }
        reach.assert_reached(&format!("W{}", L::WIDTH));
        std::eprintln!("W{}: {reach:?}", L::WIDTH);
    });
}

// ---------------------------------------------------------------------------------------------
// Gate 2
// ---------------------------------------------------------------------------------------------

/// The members of the differential: lane `l` releases in `5 + 3 l` ms, so lanes come to rest on
/// different blocks.
fn staggered(count: usize) -> Vec<Values> {
    (0..count)
        .map(|member| {
            let mut values =
                values_from(GATE_EXPANDER_PARAMETERS.map(|p| (p.default_value, p.default_value)));
            fast(&mut values, 0.0);
            set(&mut values, 6, 5.0 + 3.0 * member as f32);
            values
        })
        .collect()
}

const TONE_BLOCKS: usize = 20;
const SILENT_BLOCKS: usize = 420;
/// The silent block that retargets lane 0's threshold to its own value: every ramp stays settled,
/// but the bank's `ramp_frames_left` clock is reloaded while the bank is at rest.
const RELOAD_BLOCK: usize = TONE_BLOCKS + 400;
/// The silent block, after the reload block and with the bank at rest, in which lane 0's right
/// plane alone carries noise: the fast path must refuse it on the right plane's leg.
const RIGHT_ONLY_BLOCK: usize = RELOAD_BLOCK + 5;

/// One block's input: noise on every active lane, or `+0.0`.
fn input(block: usize, lanes: usize, mask: &[bool]) -> (Vec<f32>, Vec<f32>) {
    let mut left = vec![0.0_f32; FRAMES * lanes];
    let mut right = vec![0.0_f32; FRAMES * lanes];
    if (TONE_BLOCKS..TONE_BLOCKS + SILENT_BLOCKS).contains(&block) {
        return (left, right);
    }
    let mut draw = Draw::new(block as u64);
    for frame in 0..FRAMES {
        for (lane, active) in mask.iter().enumerate() {
            if *active {
                left[frame * lanes + lane] = 0.5 * (draw.unit() * 2.0 - 1.0);
                right[frame * lanes + lane] = 0.25 * (draw.unit() * 2.0 - 1.0);
            }
        }
    }
    (left, right)
}

/// Renders the signal/silence/signal schedule. Returns per block the output words, the reports and
/// every lane's words plus the clock, per block whether the fast path took it, and the first block
/// each lane was reported at rest.
fn schedule<L: Lane>(
    members: &[Values],
    mask: &[bool],
    link: LinkMode,
    forbid: bool,
) -> (Vec<Vec<u32>>, Vec<bool>, Vec<Option<usize>>)
where
    PreparedGate<L, false>: PreparedNativeEffectBank,
{
    let lanes = L::WIDTH;
    let mut gate = bound::<L>(members, mask, 48_000, link);
    let mut trace = Vec::new();
    let mut taken = Vec::new();
    let mut first_rest = vec![None; lanes];
    for block in 0..TONE_BLOCKS * 2 + SILENT_BLOCKS {
        let (mut left, mut right) = input(block, lanes, mask);
        if block == RIGHT_ONLY_BLOCK {
            let mut draw = Draw::new(block as u64);
            for frame in 0..FRAMES {
                right[frame * lanes] = 0.25 * (draw.unit() * 2.0 - 1.0);
            }
        }
        let first_sample = (block * FRAMES) as u64;
        let spans: Vec<PreparedAutomationSpan> = if block == RELOAD_BLOCK {
            vec![PreparedAutomationSpan {
                channel: ParameterChannel::Left,
                parameter_index: 0,
                sample: first_sample,
                value: lane_get(gate.state[0].ramps[0].current, 0),
            }]
        } else {
            Vec::new()
        };
        let mut offsets = vec![spans.len() as u32; lanes + 1];
        offsets[0] = 0;
        let before = admissions();
        let report = render_block(
            &mut gate,
            &mut left,
            &mut right,
            first_sample,
            &spans,
            &offsets,
            forbid,
        );
        taken.push(admissions() != before);
        let mut words: Vec<u32> = left.iter().chain(&right).map(|w| w.to_bits()).collect();
        for lane in 0..lanes {
            lane_words(&gate, lane, &mut words);
            let track = report.reports[lane];
            words.extend([
                track.invalid_spans as u32,
                track.nonfinite_left_blocks as u32,
                track.nonfinite_right_blocks as u32,
            ]);
        }
        words.push(gate.ramp_frames_left);
        trace.push(words);
        let rest = PreparedNativeEffectBank::lanes_at_rest(&gate);
        for (lane, first) in first_rest.iter_mut().enumerate() {
            if rest & (1 << lane) != 0 && first.is_none() {
                *first = Some(block);
            }
        }
    }
    (trace, taken, first_rest)
}

/// The scalar instance with a connected sidechain: noise, then silence until it rests, then
/// `+0.0` main input under a sidechain that carries noise, then silence again.
const SIDE_BLOCKS: [usize; 4] = [20, 80, 10, 20];

/// Renders [`SIDE_BLOCKS`] on one scalar instance with a connected sidechain. Returns per block
/// the output words, every state and coefficient word, the report and the clock, and per block
/// whether the fast path took it.
fn side_schedule(forbid: bool) -> (Vec<Vec<u32>>, Vec<bool>) {
    let values = staggered(1).remove(0);
    let mut preparation = request(&values, 48_000, LinkMode::DualMono);
    preparation.ports = PreparedPorts {
        sidechain: PreparedSidechainPort::Connected {
            id: port_id("sidechain-in"),
            required: false,
        },
    };
    let metadata =
        expected_prepared_metadata(&GATE_EXPANDER_DESCRIPTOR, preparation).expect("metadata");
    let defaults = [initial_defaults(&values).expect("valid"); MAX_WIDTH];
    let mut gate = PreparedGate::<f32, true>::new(&metadata, None, defaults).expect("connected");
    let [tone, rest, side, _] = SIDE_BLOCKS;
    let mut trace = Vec::new();
    let mut taken = Vec::new();
    for block in 0..SIDE_BLOCKS.iter().sum() {
        let mut draw = Draw::new(0x51DE + block as u64);
        let mut noise = || -> Vec<f32> {
            (0..FRAMES)
                .map(|_| 0.5 * (draw.unit() * 2.0 - 1.0))
                .collect()
        };
        let silent = vec![0.0_f32; FRAMES];
        let (mut left, mut right, side_left, side_right) = if block < tone {
            (noise(), noise(), noise(), noise())
        } else if (tone + rest..tone + rest + side).contains(&block) {
            (silent.clone(), silent.clone(), noise(), noise())
        } else {
            (silent.clone(), silent.clone(), silent.clone(), silent)
        };
        let process = EffectProcessBlock::new(
            &mut left,
            &mut right,
            Some((&side_left, &side_right)),
            (block * FRAMES) as u64,
            &[],
            QUANTUM,
        )
        .expect("a block");
        let before = admissions();
        SILENT_FORBIDDEN.with(|cell| cell.set(forbid));
        let report = PreparedNativeEffect::process(&mut gate, process);
        SILENT_FORBIDDEN.with(|cell| cell.set(false));
        taken.push(admissions() != before);
        let mut words: Vec<u32> = left.iter().chain(&right).map(|w| w.to_bits()).collect();
        lane_words(&gate, 0, &mut words);
        words.extend([
            report.invalid_spans as u32,
            report.nonfinite_left_blocks as u32,
            report.nonfinite_right_blocks as u32,
            gate.ramp_frames_left,
        ]);
        trace.push(words);
    }
    (trace, taken)
}

/// Gate 2's scalar arm: the connected instance at rest renders `+0.0` main input under an active
/// sidechain bit for bit as the whole kernel does.
fn connected_scalar() {
    let (fast, taken) = side_schedule(false);
    let (slow, forbidden) = side_schedule(true);
    assert!(
        !forbidden.contains(&true),
        "scalar: the control arm took the fast path"
    );
    let [tone, rest, _, _] = SIDE_BLOCKS;
    assert!(
        taken[tone + rest - 1],
        "scalar: the instance was not at rest before the sidechain block"
    );
    for (block, (fast, slow)) in fast.iter().zip(&slow).enumerate() {
        assert!(fast == slow, "scalar: block {block} moved a bit");
    }
}

/// **Gate 2: a bank whose lanes come to rest on different blocks renders bit-identically with
/// the silent fast path and without it, at `Simd4` and `Simd8`, full and padded, every link mode,
/// plus one connected scalar arm under a live sidechain.**
#[test]
fn the_silent_fast_path_renders_the_kernel_bits() {
    fn at<L: Lane>()
    where
        PreparedGate<L, false>: PreparedNativeEffectBank,
    {
        let lanes = L::WIDTH;
        for members in [lanes, lanes - 1] {
            for link in LINKS {
                let context = format!("W{lanes}, {members} members, {link:?}");
                let mask = prefix_mask(width_of::<L>(), members);
                let values = staggered(members);
                let (fast, taken, first_rest) = schedule::<L>(&values, &mask, link, false);
                let (slow, forbidden, _) = schedule::<L>(&values, &mask, link, true);
                assert!(
                    !forbidden.contains(&true),
                    "{context}: the control arm took the fast path"
                );
                for (block, (fast, slow)) in fast.iter().zip(&slow).enumerate() {
                    assert!(fast == slow, "{context}: block {block} moved a bit");
                }
                // Staggered rests: the first lane to rest did not make the bank skip, the bank
                // skipped once every lane rested, the reload block was skipped, and the block with
                // noise on one right plane was not, though the bank rested going into it.
                let mut rests: Vec<usize> = first_rest[..members]
                    .iter()
                    .map(|first| first.expect("every member came to rest"))
                    .collect();
                rests.sort_unstable();
                rests.dedup();
                assert!(rests.len() > 1, "{context}: every lane rested on one block");
                let count = taken.iter().filter(|taken| **taken).count();
                assert!(
                    count > 0 && count < SILENT_BLOCKS,
                    "{context}: the fast path took {count} of {SILENT_BLOCKS} silent blocks"
                );
                let all = *rests.last().expect("a rest");
                assert!(all < RELOAD_BLOCK, "{context}: the bank rested too late");
                assert!(
                    taken[RELOAD_BLOCK],
                    "{context}: the fast path refused the reload block"
                );
                assert!(
                    taken[RIGHT_ONLY_BLOCK - 1] && !taken[RIGHT_ONLY_BLOCK],
                    "{context}: the right-plane block was not the first block refused after rest"
                );
            }
        }
    }
    lane::each_vector_lane!(|L| at::<L>());
    connected_scalar();
}

// ---------------------------------------------------------------------------------------------
// Gate 3
// ---------------------------------------------------------------------------------------------

/// **Gate 3: a block the silent fast path takes allocates and frees nothing.**
///
/// A unit test rather than a row of an allocation test: the crate has no `tests/allocation.rs`,
/// the conformance harness renders one block (no lane rests in it), and only this module can see
/// the admission counter that proves every measured block took the fast path.
#[test]
fn the_silent_fast_path_allocates_nothing() {
    fn at<L: Lane>()
    where
        PreparedGate<L, false>: PreparedNativeEffectBank,
    {
        bench_support::alloc::assert_installed();
        let lanes = L::WIDTH;
        let mask = vec![true; lanes];
        let mut gate = bound::<L>(&staggered(lanes), &mask, 48_000, LinkMode::DualMono);
        let offsets = vec![0_u32; lanes + 1];
        let mut left = vec![0.0_f32; FRAMES * lanes];
        let mut right = vec![0.0_f32; FRAMES * lanes];
        let mut first_sample = 0;
        for block in 0..TONE_BLOCKS + SILENT_BLOCKS {
            let (l, r) = input(block, lanes, &mask);
            left.copy_from_slice(&l);
            right.copy_from_slice(&r);
            render_block(
                &mut gate,
                &mut left,
                &mut right,
                first_sample,
                &[],
                &offsets,
                false,
            );
            first_sample += FRAMES as u64;
        }
        assert_eq!(
            PreparedNativeEffectBank::lanes_at_rest(&gate),
            all_lanes(lanes),
            "W{lanes}: the bank is at rest"
        );
        const BLOCKS: usize = 256;
        let before = admissions();
        let mark = bench_support::alloc::current_thread_counters();
        for _ in 0..BLOCKS {
            render_block(
                &mut gate,
                &mut left,
                &mut right,
                first_sample,
                &[],
                &offsets,
                false,
            );
            first_sample += FRAMES as u64;
        }
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        assert_eq!(
            admissions() - before,
            BLOCKS,
            "W{lanes}: every block took the fast path"
        );
        assert_eq!(
            (delta.allocations, delta.deallocations),
            (0, 0),
            "W{lanes}: the fast path allocated or freed"
        );
    }
    lane::each_vector_lane!(|L| at::<L>());
}
