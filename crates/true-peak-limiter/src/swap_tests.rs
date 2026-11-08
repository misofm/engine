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

/// The writers [`every_writer_leaves_each_lane_a_payload_that_restores`] drives, by step.
const WRITERS: [&str; 12] = [
    "dual block",
    "collapsed run",
    "rest, then skipped blocks",
    "full reset",
    "discontinuity reset",
    "lane reset",
    "own restore",
    "carry across banks",
    "swap within a bank",
    "swap across banks",
    "D7 recovery",
    "explicit desymmetrize",
];

/// What one width's walks reached, beyond the writers themselves.
#[derive(Default, Debug)]
struct WriterReach {
    steps: [u32; WRITERS.len()],
    skipped_blocks: u32,
    whole_recoveries: u32,
    lane_recoveries: u32,
    nonfinite_payloads: u32,
    payloads: u32,
}

/// A bank of `L` whose strips draw their own ceiling, release, lookahead and link mode. Releases
/// are short so a walk can bring the bank to rest.
fn drawn_bank<L: Lane>(draw: &mut Draw) -> PreparedTruePeakLimiterBank<L> {
    let strip = |draw: &mut Draw| {
        [
            draw.within(-18.0, -1.0),
            draw.within(10.0, 12.0),
            draw.within(0.0, 10.0),
        ]
    };
    let mut left = Vec::with_capacity(L::WIDTH);
    let mut right = Vec::with_capacity(L::WIDTH);
    let mut links = Vec::with_capacity(L::WIDTH);
    for _ in 0..L::WIDTH {
        let one = strip(draw);
        left.push(one);
        right.push(if draw.below(2) == 0 { one } else { strip(draw) });
        links.push(if draw.below(2) == 0 {
            LinkMode::Maximum
        } else {
            LinkMode::DualMono
        });
    }
    let (PreparedTruePeakLimiterBank { width, core }, _) =
        limiter_bank::<L>(left, right, draw.below(6) == 0, links[0]);
    PreparedTruePeakLimiterBank {
        width,
        core: core.with_lane_links(&links),
    }
}

/// One block of `frames` frames on `bank`, dual or collapsed, carrying `spans`. A block whose
/// output failed the §4.4 check counts as a whole recovery when every lane failed, and as a lane
/// recovery otherwise.
fn walk_render<L: Lane>(
    bank: &mut PreparedTruePeakLimiterBank<L>,
    (left, right): (&mut [f32], &mut [f32]),
    frames: usize,
    first: &mut u64,
    (spans, offsets): (&[PreparedAutomationSpan], &[u32]),
    mono: bool,
    reach: &mut WriterReach,
) {
    let block = EffectBankProcessBlock::new(
        left,
        right,
        None,
        frames as u32,
        bank.width,
        *first,
        spans,
        offsets,
        QUANTUM,
    )
    .expect("a bank block");
    let report = if mono {
        bank.process_bank_mono(block)
    } else {
        bank.process_bank(block)
    };
    *first += frames as u64;
    let mut failed = 0;
    for (lane, track) in report.reports.iter().enumerate().take(L::WIDTH) {
        assert_eq!(track.invalid_spans, 0, "a drawn span was refused");
        if track.nonfinite_left_blocks + track.nonfinite_right_blocks != 0 {
            failed |= 1 << lane;
        }
    }
    if failed == every_lane(L::WIDTH) {
        reach.whole_recoveries += 1;
    } else if failed != 0 {
        reach.lane_recoveries += 1;
    }
}

/// What [`walk_block`] renders.
#[derive(Clone, Copy, PartialEq)]
enum Walk {
    /// A dual block of drawn length: loud, `+0.0`, or loud with a non-finite word on lane 0 or
    /// on every lane at one frame; now and then a ceiling or release retarget.
    Dual,
    /// A collapsed block of drawn length, loud or `+0.0`, without automation.
    Mono,
    /// A loud dual block of the whole quantum with a non-finite word at one frame on each lane
    /// of the mask (none for `0`), without automation.
    Poisoned(u32),
}

/// One block on `bank` (see [`Walk`]).
fn walk_block<L: Lane>(
    bank: &mut PreparedTruePeakLimiterBank<L>,
    first: &mut u64,
    draw: &mut Draw,
    walk: Walk,
    reach: &mut WriterReach,
) {
    let lanes = L::WIDTH;
    let mono = walk == Walk::Mono;
    let frames = match walk {
        Walk::Poisoned(_) => QUANTUM as usize,
        _ => [1, 5, 12, 13, 64, 128, 300, 512][draw.below(8)],
    };
    let words = frames * lanes;
    let (mut left, mut right) = (vec![0.0_f32; words], vec![0.0_f32; words]);
    if matches!(walk, Walk::Poisoned(_)) || draw.below(5) != 0 {
        let gain = draw.within(0.2, 3.0);
        for word in left.iter_mut().chain(right.iter_mut()) {
            *word = gain * draw.within(-1.0, 1.0);
        }
        let poisoned = match walk {
            Walk::Mono => 0,
            Walk::Poisoned(mask) => mask,
            Walk::Dual if draw.below(5) == 0 => {
                if draw.below(2) == 0 {
                    every_lane(lanes)
                } else {
                    1
                }
            }
            Walk::Dual => 0,
        };
        if mono {
            right.copy_from_slice(&left);
        } else if poisoned != 0 {
            let frame = draw.below(frames);
            for lane in (0..lanes).filter(|lane| poisoned & (1 << lane) != 0) {
                let poison = [f32::NAN, f32::INFINITY, f32::NEG_INFINITY][draw.below(3)];
                let plane = if draw.below(2) == 0 {
                    &mut left
                } else {
                    &mut right
                };
                plane[frame * lanes + lane] = poison;
            }
        }
    }
    let mut spans = Vec::new();
    let mut offsets = vec![0_u32];
    for _ in 0..lanes {
        if walk == Walk::Dual && draw.below(8) == 0 {
            let parameter_index = draw.below(2) as u32;
            spans.push(PreparedAutomationSpan {
                channel: if draw.below(2) == 0 {
                    ParameterChannel::Left
                } else {
                    ParameterChannel::Right
                },
                parameter_index,
                sample: *first,
                value: if parameter_index == 0 {
                    draw.within(-24.0, 0.0)
                } else {
                    draw.within(10.0, 12.0)
                },
            });
        }
        offsets.push(spans.len() as u32);
    }
    walk_render(
        bank,
        (&mut left, &mut right),
        frames,
        first,
        (&spans, &offsets),
        mono,
        reach,
    );
}

/// One payload's three sections as a restore input.
fn payload_input(payload: &[u8]) -> StatePayloadInput<'_> {
    let sizes = quality(RATE).maximum_state;
    let (common, rest) = payload.split_at(sizes.common_bytes as usize);
    let (left, right) = rest.split_at(sizes.left_bytes as usize);
    StatePayloadInput::new(common, left, right, sizes).expect("the prepared sizes")
}

/// Every lane of `bank` snapshots a payload that validates, restores into `twin` (whose cursors
/// are elsewhere) and restores into the lane itself, writing itself back word for word.
fn own_payloads_restore<L: Lane + 'static>(
    bank: &mut PreparedTruePeakLimiterBank<L>,
    twin: &mut PreparedTruePeakLimiterBank<L>,
    reach: &mut WriterReach,
    context: &str,
) {
    for lane in 0..L::WIDTH {
        let payload = lane_payload(bank, lane);
        let track = lane as u32;
        assert_eq!(
            bank.validate_track_state_payload(
                track,
                STATE_LAYOUT_VERSION,
                &payload_input(&payload)
            ),
            Ok(()),
            "{context}: lane {lane}'s own payload does not validate"
        );
        assert_eq!(
            twin.restore_track_state_payload(track, STATE_LAYOUT_VERSION, payload_input(&payload)),
            Ok(()),
            "{context}: lane {lane}'s own payload is refused by a twin"
        );
        assert_eq!(
            bank.restore_track_state_payload(track, STATE_LAYOUT_VERSION, payload_input(&payload)),
            Ok(()),
            "{context}: lane {lane}'s own payload is refused by its lane"
        );
        assert_eq!(
            lane_payload(bank, lane),
            payload,
            "{context}: lane {lane}'s payload does not survive its own restore"
        );
        reach.payloads += 1;
        if payload
            .as_chunks::<4>()
            .0
            .iter()
            .any(|word| !f32::from_le_bytes(*word).is_finite())
        {
            reach.nonfinite_payloads += 1;
        }
    }
}

/// `+0.0` blocks until every lane of `bank` is at rest (at most 120), then, if it is, one to
/// three blocks skipped through `advance_clocks`.
fn rest_then_skip<L: Lane>(
    bank: &mut PreparedTruePeakLimiterBank<L>,
    first: &mut u64,
    draw: &mut Draw,
    reach: &mut WriterReach,
) {
    let lanes = L::WIDTH;
    let every = every_lane(lanes);
    let words = QUANTUM as usize * lanes;
    let offsets = vec![0_u32; lanes + 1];
    for _ in 0..120 {
        if bank.lanes_at_rest() == every {
            break;
        }
        let (mut left, mut right) = (vec![0.0_f32; words], vec![0.0_f32; words]);
        walk_render(
            bank,
            (&mut left, &mut right),
            QUANTUM as usize,
            first,
            (&[], &offsets),
            false,
            reach,
        );
    }
    if bank.lanes_at_rest() == every {
        for _ in 0..1 + draw.below(3) {
            let frames = 1 + draw.below(QUANTUM as usize) as u32;
            bank.advance_clocks(every, frames);
            *first += u64::from(frames);
            reach.skipped_blocks += 1;
        }
    }
}

/// **Every writer leaves each lane a payload that restores** (issue #2393).
///
/// A restore ties each detector history tap to its main-ring word, so a writer that moved one
/// without the other would make a live lane's own snapshot unrestorable, #2324's failure. Two
/// banks of one width, every strip with its own parameters and link mode, take a random walk of
/// every writer of the history or the main ring ([`WRITERS`]):
///
/// * dual blocks of drawn length on either bank: loud, `+0.0`, or carrying a non-finite input
///   word on one lane or on all, so that a lane holds a non-finite delay word while the other
///   writers act on it; with ceiling and release retargets;
/// * the D7 recovery, whole and per lane: a poisoned block, then the two that bring the word to
///   the output;
/// * collapsed runs on the first bank, ended by whatever writer comes next (a dual block, a
///   reset, a lane reset, a carry into it) or, before a step that snapshots it or skips its
///   blocks, by `desymmetrize_channels`, as the contract requires;
/// * a rest reached by `+0.0` blocks, then blocks skipped through `advance_clocks`;
/// * both resets and `reset_lane`, either kind;
/// * a lane's own restore, and the plan-swap carry's restore of one bank's lane into the other's;
/// * lane swaps within a bank and across the two, which also move a strip's defaults, and so its
///   lookahead, seated at the next reset to defaults.
///
/// After every step outside a collapse, every lane of both banks snapshots, and its payload
/// validates, restores into a twin bank whose cursors are elsewhere, and restores into the lane
/// itself, writing itself back word for word.
///
/// Test value: a writer that moves a lane's history without its main ring, or the ring without
/// the history (a per-lane D7 recovery or `reset_lane` that clears one of the two, a disengage
/// copy that takes one, a restore that re-ages the history or does not re-age the ring), makes
/// the lane's own payload unrestorable under the #2393 tie; no other test drives the per-lane
/// recoveries, the skipped blocks and the cross-bank carry and swaps through one walk and
/// restores every lane after each step.
#[test]
fn every_writer_leaves_each_lane_a_payload_that_restores() {
    fn at<L: Lane + 'static>(seed: u64, reach: &mut WriterReach) {
        let lanes = L::WIDTH;
        let sizes = quality(RATE).maximum_state;
        let mut draw = Draw(0x2393_0000 ^ seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ lanes as u64);
        let mut banks = [drawn_bank::<L>(&mut draw), drawn_bank::<L>(&mut draw)];
        let mut firsts = [0_u64; 2];
        let mut twin = drawn_bank::<L>(&mut draw);
        let mut twin_first = 0_u64;
        for _ in 0..1 + draw.below(5) {
            walk_block(
                &mut twin,
                &mut twin_first,
                &mut draw,
                Walk::Dual,
                &mut WriterReach::default(),
            );
        }
        let mut scratch = vec![
            0_u8;
            effect_contract::swap_bank_lanes_scratch_bytes(&banks[0], sizes)
                .expect("declared defaults")
        ];
        let desymmetrize = WRITERS.len() - 1;
        for step in 0..120 {
            let writer = draw.below(desymmetrize);
            // Before a step that snapshots the first bank or skips its blocks, as the contract
            // requires of a collapsed bank.
            if banks[0].core.collapse.engaged() && matches!(writer, 2 | 6..=9) {
                banks[0].desymmetrize_channels();
                reach.steps[desymmetrize] += 1;
            }
            let which = draw.below(2);
            let lane = draw.below(lanes);
            let other = draw.below(lanes);
            let context = format!("W{lanes} seed {seed} step {step}: {}", WRITERS[writer]);
            match writer {
                0 => walk_block(
                    &mut banks[which],
                    &mut firsts[which],
                    &mut draw,
                    Walk::Dual,
                    reach,
                ),
                1 => {
                    for _ in 0..1 + draw.below(3) {
                        walk_block(&mut banks[0], &mut firsts[0], &mut draw, Walk::Mono, reach);
                    }
                }
                2 => rest_then_skip(&mut banks[which], &mut firsts[which], &mut draw, reach),
                3 => banks[which].reset(ResetKind::FullToDefaults),
                4 => banks[which].reset(ResetKind::DiscontinuityKeepParameters),
                5 => {
                    let kind = [
                        ResetKind::FullToDefaults,
                        ResetKind::DiscontinuityKeepParameters,
                    ][draw.below(2)];
                    banks[which].reset_lane(lane, kind);
                }
                6 | 7 => {
                    let payload = lane_payload(&banks[which], lane);
                    let (destination, target) = if writer == 6 {
                        (which, lane)
                    } else {
                        (1 - which, other)
                    };
                    assert_eq!(
                        banks[destination].restore_track_state_payload(
                            target as u32,
                            STATE_LAYOUT_VERSION,
                            payload_input(&payload),
                        ),
                        Ok(()),
                        "{context}"
                    );
                }
                8 => assert_eq!(
                    effect_contract::swap_bank_lanes(
                        &mut banks[which],
                        lane,
                        None,
                        other,
                        STATE_LAYOUT_VERSION,
                        sizes,
                        &mut scratch,
                    ),
                    Ok(()),
                    "{context}"
                ),
                10 => {
                    // The word reaches the output `B < QUANTUM` frames later, so within two
                    // blocks; every lane at once is the whole recovery, one lane the per-lane one.
                    let mask = if draw.below(2) == 0 {
                        every_lane(lanes)
                    } else {
                        1 << lane
                    };
                    for poisoned in [mask, 0, 0] {
                        walk_block(
                            &mut banks[which],
                            &mut firsts[which],
                            &mut draw,
                            Walk::Poisoned(poisoned),
                            reach,
                        );
                    }
                }
                _ => {
                    let [first, second] = &mut banks;
                    assert_eq!(
                        effect_contract::swap_bank_lanes(
                            first,
                            lane,
                            Some(second),
                            other,
                            STATE_LAYOUT_VERSION,
                            sizes,
                            &mut scratch,
                        ),
                        Ok(()),
                        "{context}"
                    );
                }
            }
            reach.steps[writer] += 1;
            for (index, bank) in banks.iter_mut().enumerate() {
                if !bank.core.collapse.engaged() {
                    own_payloads_restore(
                        bank,
                        &mut twin,
                        reach,
                        &format!("{context}, bank {index}"),
                    );
                }
            }
        }
    }
    lane::each_vector_lane!(|L| {
        let mut reach = WriterReach::default();
        for seed in 0..4 {
            at::<L>(seed, &mut reach);
        }
        println!("W{}: {reach:?}", L::WIDTH);
        for (writer, count) in WRITERS.iter().zip(reach.steps) {
            assert!(count > 0, "W{}: no {writer}", L::WIDTH);
        }
        assert!(
            reach.skipped_blocks > 0
                && reach.whole_recoveries > 0
                && reach.lane_recoveries > 0
                && reach.nonfinite_payloads > 0,
            "W{}: {reach:?}",
            L::WIDTH
        );
    });
}
