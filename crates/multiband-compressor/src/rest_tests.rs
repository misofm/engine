//! Issue #893: the multiband bank's per-lane rest claim and its clocks, at `Simd4` and `Simd8`.
//!
//! Crate-internal for the reason the compressor's are: the probe's `fork` copies both channels'
//! whole state (`Instance` is `Clone` under test), and the clock test compares the bank's
//! silent-fixed-point flag, which no public surface shows.
//!
//! The multiband's clocks are the two silence counters: every other word a block carries forward
//! is in [`lane_state`].

use super::*;
use conformance::rest_probe::{self, Reach, Schedule};
use dsp_reference::randomized::Draw;
use effect_contract::{PrepareEffectLimits, PreparedPorts, PreparedSidechainPort};

/// Not a divisor of the 64-sample smoothing, so a ramp is still in flight at a block end.
const FRAMES: usize = 48;

/// The prepared quantum.
const QUANTUM: u32 = 128;

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

/// Initial values for one track, every parameter drawn in its domain; two times in three both
/// bands take the fastest release (5 ms), so a compressing lane rests inside a case.
fn draw_values(draw: &mut Draw) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    core::array::from_fn(|slot| {
        let descriptor = &MULTIBAND_COMPRESSOR_PARAMETERS[slot / 2];
        InitialParameterValue {
            parameter_index: (slot / 2) as u32,
            channel: if slot.is_multiple_of(2) {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: draw.in_domain(
                descriptor.minimum.expect("a bounded domain"),
                descriptor.maximum.expect("a bounded domain"),
            ),
        }
    })
}

/// A bank of `W` tracks at `rate`, `link` and `bypass`, each track drawn by [`draw_values`].
fn bank<L: Lane, const W: usize>(
    draw: &mut Draw,
    rate: u32,
    link: LinkMode,
    bypass: bool,
) -> PreparedMultibandCompressorBank<L, W> {
    let fast = draw.chance(2, 3);
    let mut left = [[0.0; PARAMETER_COUNT]; W];
    let mut right = [[0.0; PARAMETER_COUNT]; W];
    let mut metadata = None;
    for track in 0..W {
        let mut values = draw_values(draw);
        if fast {
            for channel in 0..2 {
                values[4 * 2 + channel].value = 5.0;
                values[9 * 2 + channel].value = 5.0;
            }
        }
        let request = PrepareEffectRequest {
            sample_rate: rate,
            quantum: QUANTUM,
            quality: EffectQuality::Normal,
            bypass,
            link_mode: link,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: &values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: u64::MAX,
                maximum_scratch_bytes: u64::MAX,
                maximum_automation_spans_per_block: 32,
            },
            tail_bound: tail_bound(rate),
        };
        metadata = Some(
            expected_prepared_metadata(&MULTIBAND_COMPRESSOR_DESCRIPTOR, request)
                .expect("drawn values are in domain"),
        );
        (left[track], right[track]) = initial_defaults(&values).expect("drawn values are legal");
    }
    let metadata = metadata.expect("a track");
    PreparedMultibandCompressorBank {
        width: BankWidth::for_lanes(W).expect("a bank width"),
        automation_capacity: metadata.automation_capacity,
        state_sizes: metadata.state_sizes,
        instance: Instance::new(left, right, metadata).expect("prepares"),
        silent: SilentClaim::new(),
    }
}

/// One block through `process_bank`, with `spans` laid out by `offsets`.
fn process<L: Lane, const W: usize>(
    bank: &mut PreparedMultibandCompressorBank<L, W>,
    left: &mut [f32],
    right: &mut [f32],
    frames: usize,
    first_sample: u64,
    spans: &[PreparedAutomationSpan],
    offsets: &[u32],
) {
    let width = bank.width;
    bank.process_bank(
        EffectBankProcessBlock::new(
            &mut left[..frames * W],
            &mut right[..frames * W],
            None,
            frames as u32,
            width,
            first_sample,
            spans,
            offsets,
            QUANTUM,
        )
        .expect("a bank block"),
    );
}

/// Every word of lane `track` that a block carries forward, both channels, but the silence
/// counter (a clock): the crossover state words, both smoother words, every field of every ramp
/// (the crossover's three word ramps included, issue #1338), and the crossover target.
fn lane_state<L: Lane, const W: usize>(
    instance: &Instance<L, W>,
    track: usize,
    out: &mut Vec<u32>,
) {
    for side in &instance.sides {
        let words = [
            side.filter.a.ic1,
            side.filter.a.ic2,
            side.filter.b.ic1,
            side.filter.b.ic2,
            side.gain_db[LOW_BAND],
            side.gain_db[HIGH_BAND],
        ];
        out.extend(words.iter().map(|word| lane_value(*word, track).to_bits()));
        for ramp in &side.ramps[track] {
            out.extend([
                ramp.current.to_bits(),
                ramp.target.to_bits(),
                ramp.step.to_bits(),
                ramp.remaining,
            ]);
        }
        out.push(side.crossover_hz[track].to_bits());
    }
}

/// One bank under the shared driver.
struct Probe<L: Lane, const W: usize> {
    bank: PreparedMultibandCompressorBank<L, W>,
    first_sample: u64,
    /// Draws a point now and then, so ramps are in flight at block ends; `None` in a fork, whose
    /// one block is the claim's block of `+0.0` with no automation.
    automation: Option<Draw>,
}

impl<L: Lane, const W: usize> rest_probe::Probe for Probe<L, W> {
    fn lanes(&self) -> usize {
        W
    }

    fn frames(&self) -> usize {
        FRAMES
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32; W + 1];
        for track in 0..W {
            if let Some(draw) = self.automation.as_mut()
                && draw.chance(1, 64)
            {
                // Every parameter, the crossover included (live since #1338).
                let parameter = draw.below(PARAMETER_COUNT);
                let descriptor = &MULTIBAND_COMPRESSOR_PARAMETERS[parameter];
                spans.push(PreparedAutomationSpan {
                    channel: draw.pick(&[ParameterChannel::Left, ParameterChannel::Right]),
                    parameter_index: parameter as u32,
                    sample: self.first_sample,
                    value: draw.in_domain(
                        descriptor.minimum.expect("a bounded domain"),
                        descriptor.maximum.expect("a bounded domain"),
                    ),
                });
            }
            offsets[track + 1] = spans.len() as u32;
        }
        process(
            &mut self.bank,
            left,
            right,
            FRAMES,
            self.first_sample,
            &spans,
            &offsets,
        );
        self.first_sample += FRAMES as u64;
    }

    fn lanes_at_rest(&self) -> u32 {
        PreparedNativeEffectBank::lanes_at_rest(&self.bank)
    }

    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        lane_state(&self.bank.instance, lane, out);
    }

    /// The same bank, whole state cloned, with the claim withdrawn so its block runs the kernel.
    fn fork(&self) -> Self {
        Self {
            bank: PreparedMultibandCompressorBank {
                width: self.bank.width,
                automation_capacity: self.bank.automation_capacity,
                state_sizes: self.bank.state_sizes,
                instance: self.bank.instance.clone(),
                silent: SilentClaim::new(),
            },
            first_sample: self.first_sample,
            automation: None,
        }
    }
}

fn case<L: Lane, const W: usize>(seed: u64) -> Reach {
    let mut draw = Draw::new(seed);
    let rate = draw.pick(&RATES);
    let link = draw.pick(&LINKS);
    let bypass = draw.chance(1, 8);
    let mut probe = Probe::<L, W> {
        bank: bank::<L, W>(&mut draw, rate, link, bypass),
        first_sample: 0,
        automation: Some(Draw::new(!seed)),
    };
    rest_probe::check(
        &mut probe,
        seed,
        Schedule {
            blocks: 2_400,
            longest_signal: 6,
            longest_silence: 1_500,
        },
    )
}

/// **Issue #893 gates 1 and 2: a lane the multiband bank reports at rest stays bit for bit at
/// rest through a block of `+0.0`, through the whole kernel, and writes `+0.0`,** at `Simd4` and
/// `Simd8`, random rates, link modes, prepared bypass, and points that start ramps. The driver's
/// schedule carries `-0.0` blocks and subnormal signal (`1e-40`), which a lane must not take for
/// rest, and a lane only earns a claim after signal by reaching the rest state for real.
#[test]
fn a_lane_reported_at_rest_stays_at_rest_through_silence() {
    lane::each_vector_lane!(|L, W| {
        let mut reach = Reach::default();
        for seed in 0..4_u64 {
            reach.add(case::<L, W>(0x0893_0000 + seed));
        }
        reach.assert_reached(&format!("W{W}"));
        std::eprintln!("W{W}: {reach:?}");
    });
}

/// Every word of `bank` a block can move: each track's state payload (silence counters
/// included) and the silent-fixed-point flag.
fn bank_words<L: Lane, const W: usize>(bank: &PreparedMultibandCompressorBank<L, W>) -> Vec<u8> {
    let sizes = bank.state_sizes;
    let mut out = vec![u8::from(bank.silent.proved())];
    for track in 0..W {
        let mut common = vec![0; sizes.common_bytes as usize];
        let mut left = vec![0; sizes.left_bytes as usize];
        let mut right = vec![0; sizes.right_bytes as usize];
        bank.snapshot_track_state_payload(
            track as u32,
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
        )
        .expect("snapshot");
        out.extend(common);
        out.extend(left);
        out.extend(right);
    }
    out
}

/// A copy of `bank` with every state word.
fn copy<L: Lane, const W: usize>(
    bank: &PreparedMultibandCompressorBank<L, W>,
) -> PreparedMultibandCompressorBank<L, W> {
    PreparedMultibandCompressorBank {
        width: bank.width,
        automation_capacity: bank.automation_capacity,
        state_sizes: bank.state_sizes,
        instance: bank.instance.clone(),
        silent: bank.silent,
    }
}

/// `kernel` renders each `+0.0` block through the kernel (its claim withdrawn first), `fast`
/// through `process_bank` as it stands (the in-bank fast path once its claim is earned), and
/// `skip` skips it with `advance_clocks` over every lane. After every block the three are equal
/// word for word, and `fast` wrote `+0.0`.
fn zero_blocks_match<L: Lane, const W: usize>(
    draw: &mut Draw,
    banks: &mut [PreparedMultibandCompressorBank<L, W>; 3],
    blocks: usize,
    label: &str,
) {
    let offsets = vec![0_u32; W + 1];
    let mut left = vec![0.0_f32; QUANTUM as usize * W];
    let mut right = vec![0.0_f32; QUANTUM as usize * W];
    let [kernel, fast, skip] = banks;
    for block in 0..blocks {
        let frames = draw.pick(&[1, 17, 32, 48, 100, 128]);
        left.fill(0.0);
        right.fill(0.0);
        kernel.silent.withdraw();
        process(kernel, &mut left, &mut right, frames, 0, &[], &offsets);
        left.fill(0.0);
        right.fill(0.0);
        process(fast, &mut left, &mut right, frames, 0, &[], &offsets);
        assert!(
            left.iter().chain(&right).all(|word| word.to_bits() == 0),
            "{label}: block {block} wrote a word other than +0.0"
        );
        PreparedNativeEffectBank::advance_clocks(skip, all_lanes::<W>(), frames as u32);
        let rendered = bank_words(kernel);
        for (name, other) in [("fast path", &*fast), ("advance_clocks", &*skip)] {
            let first = rendered
                .iter()
                .zip(&bank_words(other))
                .position(|(a, b)| a != b);
            assert_eq!(
                first, None,
                "{label}: {name}, block {block} of {frames} frames, first byte"
            );
        }
    }
}

fn clocks_advance_as_a_zero_block<L: Lane, const W: usize>() {
    for seed in 0..3_u64 {
        let mut draw = Draw::new(0x0893_1000 + seed);
        let rate = draw.pick(&RATES);
        let link = draw.pick(&LINKS);
        let bypass = seed == 2;
        let label = format!("W{W}, {rate} Hz, {link:?}, bypass {bypass}");
        let fresh = bank::<L, W>(&mut draw, rate, link, bypass);
        // Fresh: at rest with every counter at `0`, so the run crosses the arming edge.
        assert_eq!(
            PreparedNativeEffectBank::lanes_at_rest(&fresh),
            all_lanes::<W>(),
            "{label}"
        );
        let mut banks = [copy(&fresh), copy(&fresh), fresh];
        let blocks = 2 * lane::silence_frames(rate) as usize / 64 + 8;
        zero_blocks_match(&mut draw, &mut banks, blocks, &format!("{label}, fresh"));
        // Counters that differ by lane and channel: a few frames short of the arming edge, just
        // past it, and a block short of the saturation `2^24`.
        let window = lane::silence_frames(rate) as f32;
        for bank in &mut banks {
            for (index, side) in bank.instance.sides.iter_mut().enumerate() {
                for track in 0..W {
                    let run = match (track + index) % 4 {
                        0 => (track * 7) as f32,
                        1 => window - (track + 1) as f32,
                        2 => window + 3.0,
                        _ => SILENCE_SATURATION - (track * 40) as f32,
                    };
                    side.silence = set_lane_value(side.silence, track, run);
                }
            }
            bank.silent.withdraw();
        }
        zero_blocks_match(&mut draw, &mut banks, 24, &format!("{label}, differing"));
    }
}

/// **Issue #893: a resting bank that skips a `+0.0` block, through `advance_clocks` or its own
/// in-bank fast path, ends bit-identical to one that rendered the block through the kernel** --
/// every payload word and the claim flag, across the silence counters' arming edge, at their
/// saturation, at block lengths shorter than the quantum, and under prepared bypass.
#[test]
fn skipping_a_silent_block_leaves_the_words_a_rendered_one_does() {
    lane::each_vector_lane!(|L, W| clocks_advance_as_a_zero_block::<L, W>());
}

/// The registry's tail-bound entry at `rate`, computed once per launch rate: a registry admission
/// derives every row's bound, and per request it dominated this module's debug test time (#1972).
fn tail_bound(rate: u32) -> effect_contract::RegisteredTailBound {
    static BOUNDS: [std::sync::OnceLock<effect_contract::RegisteredTailBound>; 4] =
        [const { std::sync::OnceLock::new() }; 4];
    let index = [44_100, 48_000, 88_200, 96_000]
        .iter()
        .position(|launch| *launch == rate)
        .expect("a launch rate");
    *BOUNDS[index].get_or_init(|| {
        conformance::tail_bound_of(
            Box::new(MultibandCompressorFactory),
            rate,
            EffectQuality::Normal,
        )
    })
}

/// Whether `bank`'s silent claim stands.
#[cfg(debug_assertions)]
fn claim_stands<L: Lane, const W: usize>(bank: &PreparedMultibandCompressorBank<L, W>) -> bool {
    bank.silent.proved()
}

/// A bank of random tracks, prepared at rest, whose first `+0.0` block earns its silent claim.
#[cfg(debug_assertions)]
fn resting_bank<L: Lane, const W: usize>() -> PreparedMultibandCompressorBank<L, W> {
    let mut draw = Draw::new(0x2320_0003 ^ W as u64);
    let mut bank = bank::<L, W>(&mut draw, 48_000, LinkMode::Maximum, false);
    let frames = QUANTUM as usize;
    let (mut left, mut right) = (vec![0.0_f32; frames * W], vec![0.0_f32; frames * W]);
    process(
        &mut bank,
        &mut left,
        &mut right,
        frames,
        0,
        &[],
        &vec![0; W + 1],
    );
    assert!(claim_stands(&bank), "W{W}: the claim stands");
    bank
}

/// Renders one `+0.0` block through the dual (`mono == false`) or the collapsed body and requires
/// it to panic with the claim-implies-scan assertion.
#[cfg(debug_assertions)]
fn refuses_stale_claim<L: Lane, const W: usize>(
    arm: &str,
    mut bank: PreparedMultibandCompressorBank<L, W>,
    mono: bool,
) {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    let frames = QUANTUM as usize;
    let (mut left, mut right) = (vec![0.0_f32; frames * W], vec![0.0_f32; frames * W]);
    let offsets = vec![0_u32; W + 1];
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        if mono {
            let width = bank.width;
            let block = EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                QUANTUM,
                width,
                0,
                &[],
                &offsets,
                QUANTUM,
            )
            .expect("a bank block");
            bank.process_bank_mono(block);
        } else {
            process(&mut bank, &mut left, &mut right, frames, 0, &[], &offsets);
        }
    }));
    let Err(payload) = outcome else {
        panic!("W{W} {arm}: a stale claim was believed");
    };
    let message = payload
        .downcast::<String>()
        .map(|message| *message)
        .unwrap_or_default();
    assert!(
        message.contains("a standing silent claim the rest scan refutes"),
        "W{W} {arm}: {message}"
    );
}

/// **A claim the rest scan refutes is caught where it is trusted** (issue #2320).
///
/// Each arm brings a bank to rest with its silent claim standing, then moves one crossover
/// integrator of track 0 off rest *without* withdrawing the claim, as a future between-block
/// writer that forgot to withdraw it would, and renders a `+0.0` block: the dual body with the
/// word on the right side, the collapsed body with it on the left. The quiet test reads no
/// integrator, so the fast path would cut the track's tail and move bits; in debug builds each
/// body must refuse with the claim-implies-scan assertion instead.
///
/// Test value: deleting or weakening the assertion on either body (for instance scanning only the
/// left side on the dual one) turns this red; every other test that plants an off-rest word
/// withdraws the claim by hand, so none exercises a stale claim.
#[cfg(debug_assertions)]
#[test]
fn a_stale_silent_claim_is_refused_where_it_is_trusted() {
    lane::each_vector_lane!(|L, W| {
        let mut bank = resting_bank::<L, W>();
        let side = &mut bank.instance.sides[1].filter.a;
        side.ic1 = set_lane_value(side.ic1, 0, 0.5);
        refuses_stale_claim("dual fast path", bank, false);

        let mut bank = resting_bank::<L, W>();
        let side = &mut bank.instance.sides[0].filter.a;
        side.ic1 = set_lane_value(side.ic1, 0, 0.5);
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
/// an off-rest payload (a donor's track 0 with one crossover integrator at `0.5`) and renders a
/// `+0.0` block. The claim must be gone, so the block runs the kernel and the track's tail rings
/// out; in debug builds a claim that survived would trip the fast path's claim-implies-scan
/// assertion.
///
/// Test value: deleting the withdrawal from `restore_track_state_payload` turns this red (#2320
/// mutation M12, which no other test caught); a stale claim there would cut the restored tail.
#[cfg(debug_assertions)]
#[test]
fn a_restore_off_rest_withdraws_the_standing_claim() {
    lane::each_vector_lane!(|L, W| {
        let mut donor = resting_bank::<L, W>();
        for side in &mut donor.instance.sides {
            side.filter.a.ic1 = set_lane_value(side.filter.a.ic1, 0, 0.5);
        }
        let mut bank = resting_bank::<L, W>();
        let sizes = bank.state_sizes;
        let version = MULTIBAND_COMPRESSOR_DESCRIPTOR.state_layout_version;
        restore_lane_zero(&mut bank, &donor, sizes, version);
        assert_eq!(
            PreparedNativeEffectBank::lanes_at_rest(&bank) & 1,
            0,
            "W{W}: the restored track is off rest"
        );
        assert!(!claim_stands(&bank), "W{W}: the restore withdrew the claim");
        let frames = QUANTUM as usize;
        let (mut left, mut right) = (vec![0.0_f32; frames * W], vec![0.0_f32; frames * W]);
        process(
            &mut bank,
            &mut left,
            &mut right,
            frames,
            0,
            &[],
            &[0; W + 1],
        );
    });
}
