//! Issue #1747 gate 1 for the dynamic EQ (issue #2392 brought it under the driver): a render-time
//! lane swap moves a dynamic EQ lane whole, at every bank width the build has (`BankWidth::ALL`).
//! The shared driver is `conformance::swap_probe`.
//!
//! A crate-internal test for the same reason as `glide_tests`: the width the build does not
//! execute binds through `bind_bank::<false>`, the factory's own code without the width check.

use super::*;
use conformance::swap_probe::{self, Automate, Automation, Probe};
use dsp_reference::randomized::{Draw, Profile};
use effect_contract::{
    PrepareEffectLimits, PreparedPorts, RegisteredTailBound, decimal_to_f32,
    parameter_lattice_points,
};

const RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

fn tail_bound() -> RegisteredTailBound {
    static BOUND: std::sync::OnceLock<RegisteredTailBound> = std::sync::OnceLock::new();
    *BOUND.get_or_init(|| {
        conformance::tail_bound_of(Box::new(DynamicEqFactory), RATE, EffectQuality::Normal)
    })
}

fn request(values: &[InitialParameterValue], link: LinkMode) -> PrepareEffectRequest<'_> {
    let quality = &QUALITIES[1];
    PrepareEffectRequest {
        sample_rate: RATE,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: link,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::Unconnected {
                id: port_id("sidechain-in"),
                required: false,
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 128,
        },
        tail_bound: tail_bound(),
    }
}

/// Each parameter's lattice as `f32`s, by descriptor index, built once: the lattice is a pure
/// function of the descriptor, and rebuilding it per draw dominated the test's run time.
fn lattices() -> &'static [Vec<f32>] {
    static LATTICES: std::sync::OnceLock<Vec<Vec<f32>>> = std::sync::OnceLock::new();
    LATTICES.get_or_init(|| {
        DYNAMIC_EQ_PARAMETERS
            .iter()
            .map(|parameter| {
                parameter_lattice_points(parameter)
                    .expect("a lawful lattice")
                    .iter()
                    .map(|point| decimal_to_f32(&point.canonical).expect("a convertible point"))
                    .collect()
            })
            .collect()
    })
}

/// A random point of parameter `index`'s lattice, so an enumeration or a switch takes a choice.
fn value(draw: &mut Draw, index: usize) -> f32 {
    let lattice = &lattices()[index];
    lattice[draw.below(lattice.len())]
}

/// Random initial values on every parameter's lattice, each channel its own.
fn draw_values(draw: &mut Draw) -> Vec<InitialParameterValue> {
    (0..PARAMETER_COUNT * 2)
        .map(|slot| InitialParameterValue {
            parameter_index: (slot / 2) as u32,
            channel: if slot % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: value(draw, slot / 2),
        })
        .collect()
}

/// Two full banks of random per-lane values and link modes.
struct DynamicEqProbe {
    width: BankWidth,
    members: [Vec<Vec<InitialParameterValue>>; 2],
    /// Each lane's link mode (issue #1368): a swapped lane must take its mode with it.
    links: [Vec<LinkMode>; 2],
    sizes: StatePayloadSizes,
    /// One stream per strip.
    feeds: Vec<Draw>,
}

impl DynamicEqProbe {
    fn new(width: BankWidth, seed: u64) -> Self {
        let lanes = width.lanes() as usize;
        let mut draw = Draw::new(seed);
        let members: [Vec<Vec<InitialParameterValue>>; 2] =
            core::array::from_fn(|_| (0..lanes).map(|_| draw_values(&mut draw)).collect());
        let links: [Vec<LinkMode>; 2] =
            core::array::from_fn(|_| (0..lanes).map(|_| draw.pick(&LINKS)).collect());
        let sizes = DynamicEqFactory
            .prepare(request(&members[0][0], links[0][0]))
            .expect("a member prepares")
            .metadata
            .state_sizes;
        Self {
            width,
            members,
            links,
            sizes,
            feeds: (0..2 * lanes)
                .map(|strip| Draw::new(seed ^ (0x2392_1000 + strip as u64)))
                .collect(),
        }
    }
}

impl Probe for DynamicEqProbe {
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
        DYNAMIC_EQ_DESCRIPTOR.state_layout_version
    }

    fn bind(&mut self, which: usize) -> Box<dyn PreparedNativeEffectBank> {
        let mask = vec![true; self.lanes()];
        let requests: Vec<_> = self.members[which]
            .iter()
            .zip(&self.links[which])
            .map(|(values, &link)| request(values, link))
            .collect();
        bind_bank::<false>(PrepareEffectBankRequest {
            backend: self.width.backend(),
            width: self.width,
            requests: &requests,
            active_mask: &mask,
        })
        .expect("a valid bank request")
        .expect("one program banks")
        .processor
    }

    /// An ordinary, loud, tiny or exact level, so the detector attacks and releases.
    fn input(&mut self, strip: usize, left: &mut [f32], right: &mut [f32]) {
        let feed = &mut self.feeds[strip];
        for plane in [left, right] {
            let profile =
                feed.pick(&[Profile::Loud, Profile::Clean, Profile::Tiny, Profile::Exact]);
            plane.fill_with(|| feed.sample(profile));
        }
    }

    /// Every parameter, left then right; on a random block each with probability one third.
    fn automate(&mut self, strip: usize, how: Automate, first_sample: u64) -> Automation {
        let feed = &mut self.feeds[strip];
        let mut spans = Vec::new();
        for parameter in 0..DYNAMIC_EQ_PARAMETERS.len() {
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
                        value: value(feed, parameter),
                    });
                }
            }
        }
        Automation {
            spans,
            targets: Vec::new(),
        }
    }

    /// A channel section is the signal words, the committed values, then the ramps
    /// (`snapshot_lane`), `current`, `target`, `step` and `remaining` each.
    fn ramp_in_flight(&self, left: &[u8], right: &[u8]) -> bool {
        let ramps = SIGNAL_WORDS + PARAMETER_COUNT;
        [left, right].iter().any(|section| {
            (0..RAMPS).any(|ramp| payload::read_f32(section, ramps + ramp * 4 + 3) != 0.0)
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
/// from its own defaults on a full reset, and the swap allocates nothing. At every bank width
/// the build has.**
///
/// Test value: red when the dynamic EQ's `LaneSwap` exchange leaves a prepared per-lane default
/// or the link mode behind, or its verdict refuses a payload its own lane wrote; the dynamic EQ
/// had no end-to-end `swap_bank_lanes` test before #2392, only `glide_tests`' one direct
/// exchange.
#[test]
fn a_swapped_lane_renders_as_its_unswapped_twin() {
    bench_support::alloc::assert_installed();
    for width in BankWidth::ALL {
        for seed in 0..4_u64 {
            let seed = 0x2392_7000 + seed;
            swap_probe::check(&mut DynamicEqProbe::new(*width, seed), seed);
        }
    }
}
