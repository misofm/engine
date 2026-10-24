//! Issue #1747 gate 1: a render-time lane swap moves a compressor lane whole, at `Simd4` and
//! `Simd8`. The shared driver is `conformance::swap_probe`.
//!
//! A crate-internal test for the same reason as `padding_tests`: it binds the bank at every vector
//! width the build has (`lane::each_vector_lane!`), which the factory's D4 width check does not.
//!
//! The compressor has no clocks (`rest_tests`): its `advance_clocks` moves no state or payload
//! word (it only sets the bank flag `silent_fixed_point` on a whole-bank mask), so only the lane
//! swap is tested here.

use super::padding_tests::{
    LINKS, RATES, Values, bind, draw_values, mixed_lane_requests, width_of,
};
use super::*;
use conformance::swap_probe::{self, Automate, Automation, Probe};
use dsp_reference::randomized::{Draw, Profile};
use effect_runtime::state_payload::{RAMP_WORDS, read_ramp};

/// Two full banks of random per-lane values and link modes at one rate.
struct CompressorProbe<L: Lane> {
    members: [Vec<Values>; 2],
    rate: u32,
    /// Each lane's link mode (issue #1368): a swapped lane must take its mode with it.
    links: [Vec<LinkMode>; 2],
    sizes: StatePayloadSizes,
    /// One stream per strip.
    feeds: Vec<Draw>,
    _lane: core::marker::PhantomData<L>,
}

impl<L: Lane + 'static> CompressorProbe<L> {
    fn new(seed: u64) -> Self {
        let lanes = L::WIDTH;
        let mut draw = Draw::new(seed);
        let rate = draw.pick(&RATES);
        let members: [Vec<Values>; 2] =
            core::array::from_fn(|_| (0..lanes).map(|_| draw_values(&mut draw, false)).collect());
        let links: [Vec<LinkMode>; 2] =
            core::array::from_fn(|_| (0..lanes).map(|_| draw.pick(&LINKS)).collect());
        let sizes = CompressorFactory
            .prepare(super::padding_tests::request(
                &members[0][0],
                rate,
                links[0][0],
            ))
            .expect("a member prepares")
            .metadata
            .state_sizes;
        Self {
            members,
            rate,
            links,
            sizes,
            feeds: (0..2 * lanes)
                .map(|strip| Draw::new(seed ^ (0x1747_0000 + strip as u64)))
                .collect(),
            _lane: core::marker::PhantomData,
        }
    }
}

impl<L: Lane + 'static> Probe for CompressorProbe<L> {
    fn lanes(&self) -> usize {
        L::WIDTH
    }

    fn width(&self) -> BankWidth {
        width_of::<L>().1
    }

    fn quantum(&self) -> u32 {
        super::padding_tests::QUANTUM
    }

    fn sizes(&self) -> StatePayloadSizes {
        self.sizes
    }

    fn state_layout_version(&self) -> u32 {
        COMPRESSOR_DESCRIPTOR.state_layout_version
    }

    fn bind(&mut self, which: usize) -> Box<dyn PreparedNativeEffectBank> {
        let mask = vec![true; L::WIDTH];
        let requests = mixed_lane_requests(
            &self.members[which],
            &mask,
            0,
            self.rate,
            &self.links[which],
        );
        Box::new(bind::<L>(&requests, &mask))
    }

    /// An ordinary, loud, tiny or exact level, so the envelope attacks and releases.
    fn input(&mut self, strip: usize, left: &mut [f32], right: &mut [f32]) {
        let feed = &mut self.feeds[strip];
        for plane in [left, right] {
            let profile =
                feed.pick(&[Profile::Loud, Profile::Clean, Profile::Tiny, Profile::Exact]);
            plane.fill_with(|| feed.sample(profile));
        }
    }

    /// Each smoothed parameter, left then right, in the order the effect admits; on a random
    /// block each with probability one third.
    fn automate(&mut self, strip: usize, how: Automate, first_sample: u64) -> Automation {
        let feed = &mut self.feeds[strip];
        let mut spans = Vec::new();
        for (parameter, spec) in COMPRESSOR_PARAMETERS.iter().enumerate().take(RAMP_COUNT) {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                let wanted = match how {
                    Automate::Nothing => false,
                    Automate::Random => feed.chance(1, 3),
                    Automate::Every => true,
                };
                if wanted {
                    spans.push(PreparedAutomationSpan {
                        channel,
                        parameter_index: parameter as u32,
                        sample: first_sample,
                        value: feed.in_domain(spec.minimum.unwrap(), spec.maximum.unwrap()),
                    });
                }
            }
        }
        Automation {
            spans,
            targets: Vec::new(),
        }
    }

    /// A channel section is the gain-reduction word, then every parameter ramp and the two
    /// coefficient ramps (`state::write_channel`).
    fn ramp_in_flight(&self, left: &[u8], right: &[u8]) -> bool {
        let ramps = (state::STATE_HEADER_WORDS - 1) / RAMP_WORDS;
        [left, right].iter().any(|section| {
            (0..ramps).any(|ramp| read_ramp(section, 1 + ramp * RAMP_WORDS).remaining > 0)
        })
    }

    fn heap_calls(&self, call: &mut dyn FnMut()) -> u64 {
        let mark = bench_support::alloc::current_thread_counters();
        call();
        let delta = bench_support::alloc::current_thread_delta_since(mark);
        delta.allocations + delta.deallocations + delta.reallocations
    }
}

/// **Gate 1: a strip whose lane is swapped, inside one bank and across two, with its ramps in
/// flight, renders bit for bit as its twin that never moved, holds its twin's payload, re-seeds
/// from its own defaults on a full reset, and the swap allocates nothing. At `Simd4` and
/// `Simd8`.**
#[test]
fn a_swapped_lane_renders_as_its_unswapped_twin() {
    bench_support::alloc::assert_installed();
    lane::each_vector_lane!(|L| {
        for seed in 0..4_u64 {
            let seed = 0x1747_c000 + seed;
            swap_probe::check(&mut CompressorProbe::<L>::new(seed), seed);
        }
    });
}
