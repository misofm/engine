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
use crate::design::COEF_RELEASE;
use crate::kernel::REDUCTION_REST_DB;
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
        bank.instance.silent.withdraw();
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

// ---------------------------------------------------------------------------------------------
// Issue #1784: rest after real signal
// ---------------------------------------------------------------------------------------------

/// The unit roundoff of `f32` round-to-nearest, `2^-24`.
const ROUNDOFF: f64 = 1.0 / 16_777_216.0;

/// Blocks of tone before the silence: 4,800 samples, 100 ms at 48 kHz, so the reduction settles.
const TONE_BLOCKS: usize = 100;

/// Samples after which a release from `reduction` dB has snapped to `+0.0`, at release rate
/// coefficient `c` (#1784 D4).
///
/// With a `+0.0` detector the curve target is `+0.0`, so the release step is
/// `g' = snap(fma(c, 0 - g, g))`: `0 - g` is exact, and the multiply and the add round once each
/// (`Lane::fma` is two roundings on every target), so `|g'| <= |g| (1 - c(1 - u)) (1 + u)` with
/// `u = 2^-24`. The word therefore reaches `REDUCTION_REST_DB`, where the snap writes `+0.0`,
/// within `ln(|g| / REDUCTION_REST_DB) / (-ln(1 - c(1 - u)) - u)` steps, plus the snapping step.
fn release_bound(reduction: f32, c: f32) -> u64 {
    if reduction.to_bits() == 0 {
        return 0;
    }
    let c = f64::from(c);
    let rate = -math::log(1.0 - c * (1.0 - ROUNDOFF)) - ROUNDOFF;
    let steps = math::log(f64::from(reduction.abs()) / f64::from(REDUCTION_REST_DB)) / rate;
    steps.ceil() as u64 + 1
}

/// The default parameters with threshold `threshold_db`.
fn defaults_at(threshold_db: f32) -> Values {
    let mut values: Values = core::array::from_fn(|slot| InitialParameterValue {
        parameter_index: (slot / 2) as u32,
        channel: if slot % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: COMPRESSOR_PARAMETERS[slot / 2].default_value,
    });
    set(&mut values, 0, threshold_db);
    values
}

/// **Issue #1784 gate 1: a lane that has reduced gain on a real tone lands at its exact rest
/// within its release bound** ([`release_bound`]), at the default 100 ms release and thresholds
/// from the -18 dB default down to the -80 dB floor, at `Simd4` and `Simd8`. The link mode only
/// changes the reduction the release starts from, so one mode is run. Measured at the -18 dB
/// default: 79,200 samples (230,928 before the #1784 snap) against a bound of 79,185, which is
/// 79,200 at the block end.
#[test]
fn a_compressing_lane_lands_at_rest_within_its_release_bound() {
    const THRESHOLDS: [f32; 8] = [-18.0, -40.0, -60.0, -80.0, -24.0, -30.0, -50.0, -70.0];
    lane::each_vector_lane!(|L| {
        let link = LinkMode::DualMono;
        let lanes = L::WIDTH;
        let members: Vec<Values> = THRESHOLDS[..lanes]
            .iter()
            .map(|t| defaults_at(*t))
            .collect();
        let mask = members_first(lanes, lanes);
        let requests = lane_requests(&members, &mask, 0, 48_000, link);
        let mut bank = Bank::<L> {
            bank: bind::<L>(&requests, &mask),
            members: members.clone(),
            mask,
            rate: 48_000,
            link,
            mono: false,
            first_sample: 0,
            automation: None,
        };
        let mut sample = 0_u64;
        for _ in 0..TONE_BLOCKS {
            let mut left = vec![0.0_f32; FRAMES * lanes];
            let mut right = vec![0.0_f32; FRAMES * lanes];
            for frame in 0..FRAMES {
                let t = (sample + frame as u64) as f64;
                for lane in 0..lanes {
                    let phase = t * (0.05 + 0.007 * lane as f64);
                    left[frame * lanes + lane] = (0.5 * math::sin(phase)) as f32;
                    right[frame * lanes + lane] = (0.4 * math::cos(phase)) as f32;
                }
            }
            rest_probe::Probe::render(&mut bank, &mut left, &mut right);
            sample += FRAMES as u64;
        }
        let context = format!("W{lanes}, {link:?}");
        let bounds: Vec<u64> = (0..lanes)
            .map(|lane| {
                [&bank.bank.instance.left, &bank.bank.instance.right]
                    .iter()
                    .map(|channel| {
                        let mut reduction = [0.0_f32; MAX_WIDTH];
                        channel.gain_reduction_db.store(&mut reduction);
                        assert!(
                            reduction[lane] < -1.0,
                            "{context}: lane {lane} did not reduce gain on the tone"
                        );
                        release_bound(reduction[lane], channel.words[COEF_RELEASE][lane])
                    })
                    .max()
                    .expect("two channels")
            })
            .collect();
        let longest = *bounds.iter().max().expect("a lane");
        let mut first_rest = vec![None; lanes];
        let mut silent = 0_u64;
        while silent <= longest && first_rest.iter().any(Option::is_none) {
            let mut left = vec![0.0_f32; FRAMES * lanes];
            let mut right = vec![0.0_f32; FRAMES * lanes];
            rest_probe::Probe::render(&mut bank, &mut left, &mut right);
            silent += FRAMES as u64;
            let rest = rest_probe::Probe::lanes_at_rest(&bank);
            for (lane, first) in first_rest.iter_mut().enumerate() {
                if rest & (1 << lane) != 0 && first.is_none() {
                    *first = Some(silent);
                }
            }
        }
        for lane in 0..lanes {
            let reached = first_rest[lane].unwrap_or_else(|| {
                panic!("{context}: lane {lane} not at rest {silent} samples into the silence")
            });
            let bound = bounds[lane].next_multiple_of(FRAMES as u64);
            assert!(
                reached <= bound,
                "{context}: lane {lane} rested after {reached} samples, bound {bound}"
            );
        }
        std::eprintln!("{context}: rest after {first_rest:?} samples, bounds {bounds:?}");
    });
}

/// **Issue #1784 B1: `REDUCTION_REST_DB` is the edge of the applied gain's plateau next to zero,**
/// re-derived from the real `fast_gain_from_db`: the gain at `-REDUCTION_REST_DB` is the plateau
/// value `fast_gain_from_db(-1e-30)`, pinned at `0x3f7f_fffe` (`1 - 2` ulps, the step the snap
/// takes), the gain at the next `f32` below is not, and every 997th word from the smallest normal
/// magnitude to the edge is on the plateau. (An exhaustive release-build sweep of every word up to
/// `1e-3` dB found the plateau contiguous and never re-entered: #1784 D2.)
#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "#1784 B1: re-derives the reduction rest floor from the fast tier, as its bound gate"
)]
fn the_reduction_rest_floor_is_the_fast_gain_plateau_edge() {
    use math::fast_db::fast_gain_from_db;
    let gain = |db: f32| fast_gain_from_db::<f32>(db).to_bits();
    let plateau = gain(-1.0e-30);
    assert_eq!(plateau, 0x3f7f_fffe, "the plateau value moved");
    assert_eq!(
        gain(-REDUCTION_REST_DB),
        plateau,
        "the edge is off the plateau"
    );
    let beyond = -f32::from_bits(REDUCTION_REST_DB.to_bits() + 1);
    assert_ne!(gain(beyond), plateau, "the plateau extends past the edge");
    for bits in (f32::MIN_POSITIVE.to_bits()..=REDUCTION_REST_DB.to_bits()).step_by(997) {
        let db = -f32::from_bits(bits);
        assert_eq!(gain(db), plateau, "{db:e} dB is off the plateau");
    }
}

/// Whether `bank`'s silent claim stands.
#[cfg(debug_assertions)]
fn claim_stands<L: Lane>(bank: &PreparedCompressorBank<L>) -> bool {
    bank.instance.silent.proved()
}

/// A full bank of `L` of random members, prepared at rest, whose first `+0.0` block earns its
/// silent claim.
#[cfg(debug_assertions)]
fn resting_bank<L: Lane>() -> PreparedCompressorBank<L> {
    let lanes = L::WIDTH;
    let mut draw = Draw::new(0x2320_0001 ^ lanes as u64);
    let members: Vec<Values> = (0..lanes).map(|_| draw_values(&mut draw, true)).collect();
    let mask = members_first(lanes, lanes);
    let mut bank = bind::<L>(
        &lane_requests(&members, &mask, 0, 48_000, LinkMode::Maximum),
        &mask,
    );
    let mut left = vec![0.0_f32; QUANTUM as usize * lanes];
    let mut right = vec![0.0_f32; QUANTUM as usize * lanes];
    quiet_block(&mut bank, &mut left, &mut right, false);
    assert!(claim_stands(&bank), "W{lanes}: the claim stands");
    bank
}

/// One quantum-long block through the dual (`mono == false`) or the collapsed body.
#[cfg(debug_assertions)]
fn quiet_block<L: Lane>(
    bank: &mut PreparedCompressorBank<L>,
    left: &mut [f32],
    right: &mut [f32],
    mono: bool,
) {
    let (_, width) = width_of::<L>();
    let offsets = vec![0_u32; L::WIDTH + 1];
    let block =
        EffectBankProcessBlock::new(left, right, None, QUANTUM, width, 0, &[], &offsets, QUANTUM)
            .expect("a bank block");
    if mono {
        bank.process_bank_mono(block);
    } else {
        bank.process_bank(block);
    }
}

/// Moves lane 0's envelope on `channel` off its `+0.0` rest without withdrawing the claim.
#[cfg(debug_assertions)]
fn plant_reduction<L: Lane>(channel: &mut Channel<L>) {
    let mut words = [0.0_f32; MAX_WIDTH];
    channel.gain_reduction_db.store(&mut words[..L::WIDTH]);
    words[0] = -3.0;
    channel.gain_reduction_db = L::load(&words[..L::WIDTH]);
}

/// Renders one `+0.0` block through the dual or the collapsed body and requires it to panic with
/// the claim-implies-scan assertion.
#[cfg(debug_assertions)]
fn refuses_stale_claim<L: Lane>(arm: &str, mut bank: PreparedCompressorBank<L>, mono: bool) {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    let lanes = L::WIDTH;
    let mut left = vec![0.0_f32; QUANTUM as usize * lanes];
    let mut right = vec![0.0_f32; QUANTUM as usize * lanes];
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        quiet_block(&mut bank, &mut left, &mut right, mono);
    }));
    let Err(payload) = outcome else {
        panic!("W{lanes} {arm}: a stale claim was believed");
    };
    let message = payload
        .downcast::<String>()
        .map(|message| *message)
        .unwrap_or_default();
    assert!(
        message.contains("a standing silent claim the rest scan refutes"),
        "W{lanes} {arm}: {message}"
    );
}

/// **A claim the rest scan refutes is caught where it is trusted** (issue #2320).
///
/// Each arm brings a bank to rest with its silent claim standing, then moves lane 0's envelope off
/// rest *without* withdrawing the claim, as a future between-block writer that forgot to withdraw
/// it would, and renders a `+0.0` block: the dual body with the word on the right channel, the
/// collapsed body with it on the left. The quiet test reads no envelope, so the fast path would
/// freeze the release and move bits; in debug builds each body must refuse with the
/// claim-implies-scan assertion instead.
///
/// Test value: deleting or weakening the assertion on either body (for instance scanning only the
/// left channel on the dual one) turns this red; every other test that plants an off-rest word
/// withdraws the claim by hand, so none exercises a stale claim.
#[cfg(debug_assertions)]
#[test]
fn a_stale_silent_claim_is_refused_where_it_is_trusted() {
    lane::each_vector_lane!(|L| {
        let mut bank = resting_bank::<L>();
        plant_reduction(&mut bank.instance.right);
        refuses_stale_claim("dual fast path", bank, false);

        let mut bank = resting_bank::<L>();
        plant_reduction(&mut bank.instance.left);
        refuses_stale_claim("collapsed fast path", bank, true);
    });
}

/// Restores lane 0 of `into` from `from`'s lane 0 payload through the contract's payload calls.
#[cfg(debug_assertions)]
fn restore_lane_zero(
    into: &mut dyn PreparedNativeEffectBank,
    from: &dyn PreparedNativeEffectBank,
    sizes: StatePayloadSizes,
    version: u32,
) {
    let total = sizes.total().expect("sizes") as usize;
    let mut payload = vec![0_u8; total];
    let (common, rest) = payload.split_at_mut(sizes.common_bytes as usize);
    let (left, right) = rest.split_at_mut(sizes.left_bytes as usize);
    from.snapshot_track_state_payload(
        0,
        StatePayloadOutput::new(common, left, right, sizes).expect("output"),
    )
    .expect("snapshot");
    let (common, rest) = payload.split_at(sizes.common_bytes as usize);
    let (left, right) = rest.split_at(sizes.left_bytes as usize);
    into.restore_track_state_payload(
        0,
        version,
        StatePayloadInput::new(common, left, right, sizes).expect("input"),
    )
    .expect("restore");
}

/// **A restore withdraws a standing claim** (issue #2320): a resting bank whose claim stands takes
/// an off-rest payload (a donor's lane 0 with its envelope at -3 dB) and renders a `+0.0` block.
/// The claim must be gone, so the block runs the kernel and the release moves on; in debug builds a
/// claim that survived would trip the fast path's claim-implies-scan assertion.
///
/// Test value: deleting the withdrawal from `Instance::restore` turns this red (#2320 mutation
/// M10, which no other test caught); a stale claim there would freeze the restored release.
#[cfg(debug_assertions)]
#[test]
fn a_restore_off_rest_withdraws_the_standing_claim() {
    lane::each_vector_lane!(|L| {
        let lanes = L::WIDTH;
        let mut donor = resting_bank::<L>();
        plant_reduction(&mut donor.instance.left);
        plant_reduction(&mut donor.instance.right);
        let mut bank = resting_bank::<L>();
        let sizes = bank.instance.state_sizes;
        let version = COMPRESSOR_DESCRIPTOR.state_layout_version;
        restore_lane_zero(&mut bank, &donor, sizes, version);
        assert_eq!(
            PreparedNativeEffectBank::lanes_at_rest(&bank) & 1,
            0,
            "W{lanes}: the restored lane is off rest"
        );
        assert!(
            !claim_stands(&bank),
            "W{lanes}: the restore withdrew the claim"
        );
        let mut left = vec![0.0_f32; QUANTUM as usize * lanes];
        let mut right = vec![0.0_f32; QUANTUM as usize * lanes];
        quiet_block(&mut bank, &mut left, &mut right, false);
    });
}
