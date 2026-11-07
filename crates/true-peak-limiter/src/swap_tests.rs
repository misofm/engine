//! Issue #1747 gate 1: a render-time lane swap moves a whole strip of the limiter's bank. The
//! shared driver is `conformance::swap_probe`.
//!
//! Every strip draws its own parameters (ceiling, release, lookahead, per channel), so the lanes
//! of a bank carry different lookaheads and hence different box windows: a swap that left a
//! channel's `phase` or a ring word behind renders the destination's window position. The swap
//! does not rely on equal cursors across banks: each payload is written and restored relative to
//! its own bank's cursor (#1747 D1).

use super::rest_tests::{Draw, QUANTUM, RATE, limiter_bank};
use super::*;
use conformance::swap_probe::{self, Automate, Automation, Probe};
use effect_contract::BankWidth;

/// Two banks of one slot shape, each strip with its own parameters.
struct LimiterProbe<L: Lane> {
    /// `[ceiling dB, release ms, lookahead ms]` per strip, left and right.
    parameters: Vec<[[f32; PARAMETER_COUNT]; 2]>,
    /// Each strip's link mode (issue #1368): a swapped lane must take its mode with it.
    links: Vec<LinkMode>,
    draw: Draw,
    _lane: core::marker::PhantomData<L>,
}

impl<L: Lane> LimiterProbe<L> {
    fn new(seed: u64) -> Self {
        let mut draw = Draw(seed);
        let parameters = (0..2 * L::WIDTH)
            .map(|_| {
                [0, 1].map(|_| {
                    [
                        draw.within(-18.0, -1.0),
                        draw.within(10.0, 300.0),
                        draw.within(0.0, 10.0),
                    ]
                })
            })
            .collect();
        let links = (0..2 * L::WIDTH)
            .map(|_| {
                if draw.below(2) == 0 {
                    LinkMode::Maximum
                } else {
                    LinkMode::DualMono
                }
            })
            .collect();
        Self {
            parameters,
            links,
            draw,
            _lane: core::marker::PhantomData,
        }
    }
}

impl<L: Lane + 'static> Probe for LimiterProbe<L> {
    fn lanes(&self) -> usize {
        L::WIDTH
    }

    fn width(&self) -> BankWidth {
        BankWidth::for_lanes(L::WIDTH).expect("a bank width")
    }

    fn quantum(&self) -> u32 {
        QUANTUM
    }

    fn sizes(&self) -> StatePayloadSizes {
        quality(RATE).maximum_state
    }

    fn state_layout_version(&self) -> u32 {
        STATE_LAYOUT_VERSION
    }

    fn bind(&mut self, which: usize) -> Box<dyn PreparedNativeEffectBank> {
        let strips = &self.parameters[which * L::WIDTH..(which + 1) * L::WIDTH];
        let side = |channel: usize| strips.iter().map(|strip| strip[channel]).collect();
        let links = &self.links[which * L::WIDTH..(which + 1) * L::WIDTH];
        let (PreparedTruePeakLimiterBank { width, core }, _) =
            limiter_bank::<L>(side(0), side(1), false, links[0]);
        Box::new(PreparedTruePeakLimiterBank {
            width,
            core: core.with_lane_links(links),
        })
    }

    /// Loud, so gain reduction and the rings are non-trivial.
    fn input(&mut self, _strip: usize, left: &mut [f32], right: &mut [f32]) {
        let gain = self.draw.within(0.2, 3.0);
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            *l = gain * self.draw.within(-1.0, 1.0);
            *r = gain * self.draw.within(-1.0, 1.0);
        }
    }

    /// The ceiling and the release, each channel, in ascending `(parameter, channel)` order; on
    /// a random block each with probability one fifth.
    fn automate(&mut self, _strip: usize, how: Automate, first_sample: u64) -> Automation {
        let mut spans = Vec::new();
        for parameter_index in 0..2_u32 {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                let wanted = match how {
                    Automate::Nothing => false,
                    Automate::Random => self.draw.below(5) == 0,
                    Automate::Every => true,
                };
                if wanted {
                    spans.push(PreparedAutomationSpan {
                        channel,
                        parameter_index,
                        sample: first_sample,
                        value: if parameter_index == 0 {
                            self.draw.within(-24.0, 0.0)
                        } else {
                            self.draw.within(10.0, 300.0)
                        },
                    });
                }
            }
        }
        Automation {
            spans,
            targets: Vec::new(),
        }
    }

    /// The ceiling and release ramps' `remaining` words (`snapshot_lane`).
    fn ramp_in_flight(&self, left: &[u8], right: &[u8]) -> bool {
        [left, right].iter().any(|section| {
            [words::LIMIT_RAMP, words::RELEASE_RAMP].iter().any(|ramp| {
                let at = (ramp + 3) * 4;
                section[at..at + 4] != [0; 4]
            })
        })
    }

    fn heap_calls(&self, call: &mut dyn FnMut()) -> u64 {
        let mark = bench_support::alloc::current_thread_counters();
        call();
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        delta.allocations + delta.deallocations + delta.reallocations
    }
}

/// **A swapped strip renders and holds exactly the bits it would have unswapped** (issue #1747),
/// re-seeds from its own defaults on a full reset, and the swap reaches the allocator zero times.
/// Each strip draws its own link mode (issue #1368): a swap that left the mode in place would
/// render the strip at its new neighbour's law.
#[test]
fn a_swapped_strip_keeps_its_twins_bits() {
    bench_support::alloc::assert_installed();
    lane::each_vector_lane!(|L| {
        let width = L::WIDTH;
        for seed in 0..3_u64 {
            let seed = 0x1747_5000 ^ seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ width as u64;
            swap_probe::check(&mut LimiterProbe::<L>::new(seed), seed);
        }
    });
}

/// One lane's state payload, whole.
fn lane_payload<L: Lane>(bank: &PreparedTruePeakLimiterBank<L>, lane: usize) -> Vec<u8> {
    let sizes = quality(RATE).maximum_state;
    let mut payload = vec![0_u8; sizes.total().expect("a payload total") as usize];
    let (common, rest) = payload.split_at_mut(sizes.common_bytes as usize);
    let (left, right) = rest.split_at_mut(sizes.left_bytes as usize);
    bank.snapshot_track_state_payload(
        lane as u32,
        StatePayloadOutput::new(common, left, right, sizes).expect("the prepared sizes"),
    )
    .expect("a member's snapshot");
    payload
}

/// Issue #2324: a lane whose last block left a non-finite input word in its main ring (and, for
/// a word near the block's end, its detector history) -- the output has not reached the word,
/// so the §4.4 check has not fired -- restores its own payload and swaps out and back, and then
/// renders, reports and recovers bit for bit as a twin that did neither.
///
/// Each poison sits at the first frame whose word is still in the main ring at the block's end
/// (`QUANTUM - B`, in the main ring alone) and at the last frame (in the history too), on either
/// channel, as NaN and as both infinities.
///
/// Test value: a restore that refuses a non-finite history or main-ring word, which the kernel
/// itself leaves there (base: `effect.state.ring` and `effect.state.history`), so that the lane's
/// own payload is unrestorable, the plan-swap carry cannot carry it and a swap with it refuses.
#[test]
fn a_lane_holding_a_non_finite_delay_word_restores_and_swaps_exactly() {
    fn at<L: Lane + 'static>() {
        let lanes = L::WIDTH;
        let main = Shape::new(RATE).expect("a launch rate").main;
        let quantum = QUANTUM as usize;
        let sizes = quality(RATE).maximum_state;
        let parameters: Vec<[f32; PARAMETER_COUNT]> = (0..lanes)
            .map(|lane| {
                [
                    -3.0 - lane as f32,
                    40.0 + 10.0 * lane as f32,
                    1.0 + lane as f32,
                ]
            })
            .collect();
        let bank = || {
            limiter_bank::<L>(
                parameters.clone(),
                parameters.clone(),
                false,
                LinkMode::Maximum,
            )
            .0
        };
        let offsets = vec![0_u32; lanes + 1];
        let render = |bank: &mut PreparedTruePeakLimiterBank<L>,
                      left: &mut [f32],
                      right: &mut [f32],
                      first: u64| {
            bank.process_bank(
                EffectBankProcessBlock::new(
                    left,
                    right,
                    None,
                    QUANTUM,
                    bank.width,
                    first,
                    &[],
                    &offsets,
                    QUANTUM,
                )
                .expect("a bank block"),
            )
        };
        let mut scratch = vec![
            0_u8;
            effect_contract::swap_bank_lanes_scratch_bytes(&bank(), sizes)
                .expect("declared defaults")
        ];
        for poison in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            for frame in [quantum - main, quantum - 1] {
                for channel in 0..2 {
                    let label = format!("W{lanes} {poison} at frame {frame} on channel {channel}");
                    let (mut moved, mut twin) = (bank(), bank());
                    let mut draw = Draw(0x2324_0001 ^ frame as u64);
                    let mut first = 0_u64;
                    for block in 0..5 {
                        let mut left: Vec<f32> = (0..quantum * lanes)
                            .map(|_| draw.within(-2.0, 2.0))
                            .collect();
                        let mut right: Vec<f32> = (0..quantum * lanes)
                            .map(|_| draw.within(-2.0, 2.0))
                            .collect();
                        if block == 4 {
                            [&mut left, &mut right][channel][frame * lanes] = poison;
                        }
                        let (mut twin_left, mut twin_right) = (left.clone(), right.clone());
                        render(&mut moved, &mut left, &mut right, first);
                        render(&mut twin, &mut twin_left, &mut twin_right, first);
                        if block == 4 {
                            assert!(
                                left.iter().chain(&right).all(|word| word.is_finite()),
                                "{label}: the poisoned block's output is finite"
                            );
                        }
                        first += u64::from(QUANTUM);
                    }
                    let own = lane_payload(&moved, 0);
                    assert!(
                        own.as_chunks::<4>()
                            .0
                            .iter()
                            .any(|word| !f32::from_le_bytes(*word).is_finite()),
                        "{label}: lane 0's payload holds the poison"
                    );
                    let (common, rest) = own.split_at(sizes.common_bytes as usize);
                    let (left, right) = rest.split_at(sizes.left_bytes as usize);
                    assert_eq!(
                        moved.restore_track_state_payload(
                            0,
                            STATE_LAYOUT_VERSION,
                            StatePayloadInput::new(common, left, right, sizes).expect("sizes"),
                        ),
                        Ok(()),
                        "{label}: lane 0 restores its own payload"
                    );
                    for _ in 0..2 {
                        assert_eq!(
                            effect_contract::swap_bank_lanes(
                                &mut moved,
                                0,
                                None,
                                1,
                                STATE_LAYOUT_VERSION,
                                sizes,
                                &mut scratch,
                            ),
                            Ok(()),
                            "{label}: lane 0 swaps with lane 1"
                        );
                    }
                    for lane in 0..lanes {
                        assert_eq!(
                            lane_payload(&moved, lane),
                            lane_payload(&twin, lane),
                            "{label}: lane {lane}'s payload"
                        );
                    }
                    let mut recovered = false;
                    for block in 0..2 {
                        let mut left: Vec<f32> = (0..quantum * lanes)
                            .map(|_| draw.within(-2.0, 2.0))
                            .collect();
                        let mut right: Vec<f32> = (0..quantum * lanes)
                            .map(|_| draw.within(-2.0, 2.0))
                            .collect();
                        let (mut twin_left, mut twin_right) = (left.clone(), right.clone());
                        let report = render(&mut moved, &mut left, &mut right, first);
                        let twin_report = render(&mut twin, &mut twin_left, &mut twin_right, first);
                        assert_eq!(report, twin_report, "{label}: block {block}'s report");
                        for (plane, twin_plane) in [(&left, &twin_left), (&right, &twin_right)] {
                            assert!(
                                plane
                                    .iter()
                                    .zip(twin_plane.iter())
                                    .all(|(a, b)| a.to_bits() == b.to_bits()),
                                "{label}: block {block}'s output"
                            );
                        }
                        let charged = report.reports[0];
                        recovered |=
                            charged.nonfinite_left_blocks + charged.nonfinite_right_blocks != 0;
                        first += u64::from(QUANTUM);
                    }
                    assert!(
                        recovered,
                        "{label}: the poison reaches the output and is recovered"
                    );
                }
            }
        }
    }
    lane::each_vector_lane!(|L| at::<L>());
}
