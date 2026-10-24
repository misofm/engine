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
