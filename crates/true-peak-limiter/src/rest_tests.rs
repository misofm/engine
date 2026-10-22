//! Issue #1745 gate 1: the per-lane rest mask of the true-peak limiter's bank is sound.
//!
//! Every lane the bank reports at rest is checked by rendering one more block of exact `+0.0`
//! through a fork of the bank whose fast-path claim is withdrawn: the lane's every state word must
//! keep its bits and both output planes must be `+0.0` (`conformance::rest_probe`). Both
//! vector widths the build has are bound through crate internals (`lane::each_vector_lane!`), so
//! the `Simd4` leg runs on an AVX2 host too.
//!
//! The clocks left out of [`Probe::lane_state`] are the two that a `+0.0` block advances by its
//! length alone: the bank's shared [`Cursors`](super::Cursors) (`main`, `ring`) and each lane's
//! van Herk `phase`. Every other word of `ChannelState` is compared, both channels, which is the
//! list `ChannelState::copy_state_from` copies.
//!
//! Cases draw per-lane parameters (ceilings above and below the signal, short and long
//! releases, every lookahead), the link mode, bypass, padding, the block length and, on the dual
//! body, live retargets of the ceiling and the release that leave a ramp open across a short
//! block's boundary. Between blocks the probe now and then plants one off-rest word on a lane
//! whose output was `+0.0`, as a restore can ([`plant`]). A quarter of the cases run the collapsed
//! body; those draw one parameter set per lane for both channels and carry no automation, which is
//! what keeps a cohort collapse eligible.
//!
//! Each leg of the predicate was dropped in turn (history, main ring, reduction, both gain rings,
//! `prefix`, `box_sum`, the ramps) and each turned this test red.

use super::*;
use conformance::rest_probe::{self, Probe, Reach, Schedule};
use effect_contract::{BankWidth, PrepareEffectLimits, PreparedPorts, PreparedSidechainPort};

/// SplitMix64, for the case parameters and the probe's own automation draws.
struct Draw(u64);

impl Draw {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound.max(1) as u64) as usize
    }

    /// A value in `low..high`.
    fn within(&mut self, low: f32, high: f32) -> f32 {
        low + (self.next() >> 40) as f32 / (1_u64 << 24) as f32 * (high - low)
    }

    /// A release: mostly short enough for a limited lane to finish inside a case.
    fn release(&mut self) -> f32 {
        if self.below(4) == 0 {
            self.within(10.0, 2000.0)
        } else {
            self.within(10.0, 12.0)
        }
    }
}

const RATE: u32 = 48_000;
const QUANTUM: u32 = 512;

/// One bank under test and what its fork needs to rebuild it.
struct LimiterProbe<L: Lane> {
    bank: PreparedTruePeakLimiterBank<L>,
    metadata: PreparedEffectMetadata,
    frames: usize,
    mono: bool,
    /// Automation draws; `None` on a collapsed case.
    automation: Option<Draw>,
    /// Draws for [`plant`]; the fork never plants.
    planting: Option<Draw>,
    first_sample: u64,
}

impl<L: Lane> LimiterProbe<L> {
    fn new(draw: &mut Draw, frames: usize, mono: bool) -> Self {
        let width = BankWidth::for_lanes(L::WIDTH).expect("a bank width");
        let values: [InitialParameterValue; PARAMETER_COUNT * 2] =
            core::array::from_fn(|index| InitialParameterValue {
                parameter_index: (index / 2) as u32,
                channel: if index % 2 == 0 {
                    ParameterChannel::Left
                } else {
                    ParameterChannel::Right
                },
                value: TRUE_PEAK_LIMITER_PARAMETERS[index / 2].default_value,
            });
        let quality = QUALITIES
            .iter()
            .find(|quality| quality.sample_rate == RATE)
            .expect("launch rate");
        let request = PrepareEffectRequest {
            sample_rate: RATE,
            quantum: QUANTUM,
            quality: EffectQuality::Normal,
            bypass: draw.below(5) == 0,
            link_mode: if draw.below(2) == 0 {
                LinkMode::Maximum
            } else {
                LinkMode::DualMono
            },
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: &values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
                maximum_scratch_bytes: 24,
                maximum_automation_spans_per_block: 16,
            },
            tail_bound: conformance::tail_bound_of(
                Box::new(TruePeakLimiterFactory),
                RATE,
                EffectQuality::Normal,
            ),
        };
        let metadata =
            expected_prepared_metadata(&TRUE_PEAK_LIMITER_DESCRIPTOR, request).expect("metadata");
        let members = 1 + draw.below(L::WIDTH);
        let mut left = Vec::with_capacity(L::WIDTH);
        let mut right = Vec::with_capacity(L::WIDTH);
        for _ in 0..members {
            let one = [
                draw.within(-24.0, 0.0),
                draw.release(),
                draw.within(0.0, 10.0),
            ];
            let other = if mono {
                one
            } else {
                [
                    draw.within(-24.0, 0.0),
                    draw.release(),
                    draw.within(0.0, 10.0),
                ]
            };
            left.push(one);
            right.push(other);
        }
        // Padded lanes carry a clone of a member's parameters, as the factory seeds them.
        for lane in members..L::WIDTH {
            left.push(left[lane % members]);
            right.push(right[lane % members]);
        }
        let core =
            LimiterCore::<L>::new(metadata, left.into_boxed_slice(), right.into_boxed_slice())
                .expect("core")
                .with_active_lanes(every_lane(members));
        Self {
            bank: PreparedTruePeakLimiterBank { width, core },
            metadata,
            frames,
            mono,
            automation: (!mono).then(|| Draw(draw.next())),
            planting: Some(Draw(draw.next())),
            first_sample: 0,
        }
    }

    /// This block's automation: now and then one member retargets its ceiling or its release on
    /// one channel. Spans are per track, in ascending `(parameter, channel)` order.
    fn automation(&mut self) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32];
        let first_sample = self.first_sample;
        let active = self.bank.core.active;
        for track in 0..L::WIDTH {
            let Some(draw) = self.automation.as_mut() else {
                offsets.push(0);
                continue;
            };
            if active & (1 << track) != 0 && draw.below(24) == 0 {
                let parameter_index = draw.below(2) as u32;
                spans.push(PreparedAutomationSpan {
                    channel: if draw.below(2) == 0 {
                        ParameterChannel::Left
                    } else {
                        ParameterChannel::Right
                    },
                    parameter_index,
                    sample: first_sample,
                    value: if parameter_index == 0 {
                        draw.within(-24.0, 0.0)
                    } else {
                        draw.release()
                    },
                });
            }
            offsets.push(spans.len() as u32);
        }
        (spans, offsets)
    }
}

/// Moves one word of lane `lane` off its rest value, every other word left as it is: the shape a
/// restored payload can install between two blocks (`commit_lane` writes each word
/// independently). It is the only way to reach a lane whose detector history is stale behind a
/// drained delay line, or whose gain words are off rest while the line and the recursive word are
/// at it, so it is what gives the predicate's history, gain-ring, `prefix` and `box_sum` legs a
/// lane to refuse.
fn plant(channel: &mut ChannelState, lane: usize, shape: &Shape, draw: &mut Draw) {
    let width = channel.width;
    match draw.below(5) {
        0 => {
            for tap in 0..HISTORY_WORDS {
                channel.history[tap * width + lane] = draw.within(-1.0e-3, 1.0e-3);
            }
        }
        1 => channel.required_ring[draw.below(shape.ring) * width + lane] = 0.5,
        2 => channel.box_ring[draw.below(shape.ring) * width + lane] = 0.5,
        3 => channel.prefix[lane] = 0.5,
        _ => channel.box_sum[lane] -= 1.0,
    }
}

impl<L: Lane> Probe for LimiterProbe<L> {
    fn lanes(&self) -> usize {
        L::WIDTH
    }

    fn frames(&self) -> usize {
        self.frames
    }

    fn render(&mut self, left: &mut [f32], right: &mut [f32]) {
        let (spans, offsets) = self.automation();
        let block = EffectBankProcessBlock::new(
            left,
            right,
            None,
            self.frames as u32,
            self.bank.width,
            self.first_sample,
            &spans,
            &offsets,
            QUANTUM,
        )
        .expect("bank block");
        let report = if self.mono {
            self.bank.process_bank_mono(block)
        } else {
            self.bank.process_bank(block)
        };
        assert!(
            report.reports.iter().all(|track| track.invalid_spans == 0),
            "a drawn span was refused"
        );
        if self.mono {
            // The collapsed body writes the one live plane; the right one is its copy.
            right.copy_from_slice(left);
        }
        self.first_sample += self.frames as u64;
        // Now and then, on a lane whose output was `+0.0`, plant one off-rest word (see
        // [`plant`]); on the collapsed body, into the one channel that advances.
        let shape = self.bank.core.shape;
        let Some(planting) = self.planting.as_mut() else {
            return;
        };
        for lane in 0..L::WIDTH {
            let silent = (0..self.frames).all(|frame| {
                left[frame * L::WIDTH + lane].to_bits() == 0
                    && right[frame * L::WIDTH + lane].to_bits() == 0
            });
            if silent && planting.below(128) == 0 {
                let channel = if self.mono || planting.below(2) == 0 {
                    &mut self.bank.core.left
                } else {
                    &mut self.bank.core.right
                };
                plant(channel, lane, &shape, planting);
                self.bank.core.silent_fixed_point = false;
            }
        }
    }

    fn lanes_at_rest(&self) -> u32 {
        PreparedNativeEffectBank::lanes_at_rest(&self.bank)
    }

    fn lane_state(&self, lane: usize, out: &mut Vec<u32>) {
        for channel in [&self.bank.core.left, &self.bank.core.right] {
            let width = channel.width;
            for plane in [
                &channel.history,
                &channel.main_ring,
                &channel.required_ring,
                &channel.box_ring,
            ] {
                out.extend(
                    plane
                        .iter()
                        .skip(lane)
                        .step_by(width)
                        .map(|word| word.to_bits()),
                );
            }
            out.extend([
                channel.reduction[lane].to_bits(),
                channel.prefix[lane].to_bits(),
                channel.box_sum[lane].to_bits(),
                channel.lookahead_ms[lane].to_bits(),
                channel.lane[lane].window,
                channel.lane[lane].end_offset,
                channel.lane[lane].box_offset,
            ]);
            for ramp in [channel.limit[lane], channel.release[lane]] {
                out.extend([
                    ramp.current.to_bits(),
                    ramp.target.to_bits(),
                    ramp.step.to_bits(),
                    ramp.remaining,
                ]);
            }
            // `phase` is a clock (module note).
        }
    }

    fn fork(&self) -> Self {
        let core = &self.bank.core;
        let mut copy = LimiterCore::<L>::new(
            self.metadata,
            core.left_defaults.clone(),
            core.right_defaults.clone(),
        )
        .expect("core")
        .with_active_lanes(core.active);
        copy.left.copy_state_from(&core.left);
        copy.right.copy_state_from(&core.right);
        copy.cursors = core.cursors;
        copy.gain_linked = core.gain_linked;
        copy.silent_bypass = core.silent_bypass;
        copy.silent_fixed_point = false;
        Self {
            bank: PreparedTruePeakLimiterBank {
                width: self.bank.width,
                core: copy,
            },
            metadata: self.metadata,
            frames: self.frames,
            mono: self.mono,
            // The checking block is the `+0.0` block alone.
            automation: None,
            planting: None,
            first_sample: self.first_sample,
        }
    }
}

/// One width's cases.
fn cases<L: Lane>(seeds: core::ops::Range<u64>) -> Reach {
    let mut reach = Reach::default();
    for seed in seeds {
        let mut draw = Draw(0x1745_7000 ^ seed.wrapping_mul(0x9E37_79B9) ^ L::WIDTH as u64);
        // Rings drain in `B = 486` samples, a limited lane's 10 ms release in some 22 000.
        let (frames, schedule) = match draw.below(4) {
            // Shorter than the twelve detector taps, so a stale history outlives a block.
            3 => (
                8,
                Schedule {
                    blocks: 320,
                    longest_signal: 8,
                    longest_silence: 160,
                },
            ),
            0 => (
                16,
                Schedule {
                    blocks: 320,
                    longest_signal: 6,
                    longest_silence: 120,
                },
            ),
            1 => (
                128,
                Schedule {
                    blocks: 200,
                    longest_signal: 3,
                    longest_silence: 180,
                },
            ),
            _ => (
                512,
                Schedule {
                    blocks: 90,
                    longest_signal: 2,
                    longest_silence: 60,
                },
            ),
        };
        let mono = draw.below(4) == 0;
        let mut probe = LimiterProbe::<L>::new(&mut draw, frames, mono);
        reach.add(rest_probe::check(&mut probe, draw.next(), schedule));
    }
    reach
}

/// **Every lane the limiter bank reports at rest stays put through a block of `+0.0`.**
#[test]
fn a_lane_the_limiter_bank_reports_at_rest_is_a_fixed_point() {
    lane::each_vector_lane!(|L| {
        let reach = cases::<L>(0..8);
        println!("W{}: {reach:?}", L::WIDTH);
        reach.assert_reached(&format!("W{}", L::WIDTH));
    });
}
