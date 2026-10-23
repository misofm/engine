//! Issue #1747: the render-time lane swap of the transient shaper's state.
//!
//! The shaper has no clock: every word a block of `+0.0` changes on a lane at rest is none (the
//! two envelopes floor at `+0.0` and stay, and a settled ramp holds), and its only per-bank words
//! are prepared constants (the follower coefficients, bypass, link mode, capacities) and the D7
//! record. So the contract's default no-op `advance_clocks` is exact, and nothing here overrides
//! it. What this module holds is gate 1: `effect_contract::swap_bank_lanes` exchanges two strips'
//! whole state, within one bank and between two banks of one slot, with a ramp in flight,
//! allocation-free, so every strip renders the bits of a twin that never moved. The shared driver
//! is `conformance::swap_probe`.
//!
//! A child of `padding_tests`, so it reuses that module's bank binding, draws and payload reads.

use super::*;
use bench_support::alloc;
use conformance::swap_probe::{self, Automate, Automation, Probe};

/// The rate every bank here runs at: the shaper's state shape does not depend on it.
const SWAP_RATE: u32 = 48_000;

// ---------------------------------------------------------------------------------------------
// Gate 1: a swapped strip renders its twin's bits (driver: `conformance::swap_probe`)
// ---------------------------------------------------------------------------------------------

/// Two full banks of one slot, every strip on its own drawn parameters.
struct ShaperProbe {
    width: BankWidth,
    link: LinkMode,
    values: Vec<Values>,
    sizes: StatePayloadSizes,
    draw: Draw,
}

impl ShaperProbe {
    fn new(width: BankWidth, link: LinkMode, seed: u64) -> Self {
        let strips = 2 * width.lanes() as usize;
        let mut draw = Draw::new(seed);
        let values = (0..strips).map(|_| draw.values()).collect();
        let sizes = scalar(request(&default_values(), SWAP_RATE, LinkMode::DualMono))
            .metadata
            .state_sizes;
        Self {
            width,
            link,
            values,
            sizes,
            draw,
        }
    }
}

impl Probe for ShaperProbe {
    fn lanes(&self) -> usize {
        self.width.lanes() as usize
    }

    fn width(&self) -> BankWidth {
        self.width
    }

    fn quantum(&self) -> u32 {
        QUANTUM
    }

    fn sizes(&self) -> StatePayloadSizes {
        self.sizes
    }

    fn state_layout_version(&self) -> u32 {
        TRANSIENT_SHAPER_DESCRIPTOR.state_layout_version
    }

    fn bind(&mut self, which: usize) -> Box<dyn PreparedNativeEffectBank> {
        let lanes = self.lanes();
        let requests: Vec<PrepareEffectRequest<'_>> = (0..lanes)
            .map(|lane| request(&self.values[which * lanes + lane], SWAP_RATE, self.link))
            .collect();
        bind(self.width, &requests, &vec![true; lanes])
            .expect("a well-formed request")
            .expect("the shaper binds")
            .processor
    }

    /// Loud noise: uniform in `±level`, with the level drawn up to `+12 dB`.
    fn input(&mut self, _strip: usize, left: &mut [f32], right: &mut [f32]) {
        for plane in [left, right] {
            let level = [0.5_f32, 1.0, 2.0, 4.0][self.draw.below(4)];
            plane.fill_with(|| (self.draw.unit() * 2.0 - 1.0) * level);
        }
    }

    /// The padding tests' random spans on one block in three, or a point span on every
    /// parameter and channel at a fresh drawn value.
    fn automate(&mut self, _strip: usize, how: Automate, first_sample: u64) -> Automation {
        let spans = match how {
            Automate::Nothing => Vec::new(),
            Automate::Random if self.draw.chance(1, 3) => self.draw.spans(first_sample),
            Automate::Random => Vec::new(),
            Automate::Every => {
                let mut spans = Vec::new();
                for (index, parameter) in TRANSIENT_SHAPER_PARAMETERS.iter().enumerate() {
                    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                        spans.push(PreparedAutomationSpan {
                            channel,
                            parameter_index: index as u32,
                            sample: first_sample,
                            value: self.draw.value(parameter),
                        });
                    }
                }
                spans
            }
        };
        Automation {
            spans,
            targets: Vec::new(),
        }
    }

    /// A channel section is the two envelope words, then the ramps, `RAMP_WORDS` each,
    /// `remaining` last.
    fn ramp_in_flight(&self, left: &[u8], right: &[u8]) -> bool {
        [left, right].iter().any(|section| {
            (0..PARAMETER_COUNT).any(|parameter| {
                let at = (2 + parameter * RAMP_WORDS + 3) * 4;
                section[at..at + 4] != [0; 4]
            })
        })
    }

    fn heap_calls(&self, call: &mut dyn FnMut()) -> u64 {
        alloc::assert_installed();
        let mark = alloc::current_thread_counters();
        call();
        let delta = alloc::current_thread_delta_since(mark);
        delta.allocations + delta.deallocations + delta.reallocations
    }
}

/// Gate 1 of #1747 (`conformance::swap_probe::check`): a strip whose lane is swapped, inside one
/// bank and across two, with its ramps in flight, renders bit for bit as its twin that never
/// moved, holds its twin's payload, re-seeds from its own defaults on a full reset, and the swap
/// reaches the allocator zero times.
///
/// It runs at every link mode, seed `s` at `LINKS[s % 3]`, because the linked detectors read both
/// channels of a lane and a restore must carry both.
#[test]
fn a_swapped_strip_renders_its_unmoved_twins_bits() {
    for width in WIDTHS.iter().copied() {
        for seed in 0..6_u64 {
            let link = LINKS[seed as usize % LINKS.len()];
            let seed = 0x1747_5000 + seed;
            swap_probe::check(&mut ShaperProbe::new(width, link, seed), seed);
        }
    }
}
