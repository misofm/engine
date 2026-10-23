//! Issue #1747 gate 1: a render-time lane swap moves a gate/expander lane whole, at every bank
//! width the build has (`BankWidth::ALL`: `Simd4` and, with AVX2, `Simd8`). The shared driver is
//! `conformance::swap_probe`.
//!
//! A crate-internal test for the same reason as `padding_tests`: the width the build does not
//! execute binds through `bind_bank::<false>`, the factory's own code without the D4 width check.
//!
//! The hold countdown `hold` is detector state, carried in the payload like the envelope. The
//! bank-wide counter `ramp_frames_left` only chooses where `run_block` splits a block between its
//! ramping and settled variants; a restore raises it to cover the restored ramps, which the
//! driver's quiet block after a ramping lane lands in a settled bank checks.

use super::*;
use conformance::swap_probe::{self, Automate, Automation, Probe};
use dsp_reference::randomized::{Draw, Profile};
use effect_contract::{PrepareEffectLimits, PreparedPorts};

const QUANTUM: u32 = 128;
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

/// A request as the session compiler prepares a console slot: unkeyed, unbypassed.
fn request(
    values: &[InitialParameterValue],
    rate: u32,
    link: LinkMode,
) -> PrepareEffectRequest<'_> {
    let quality = GATE_EXPANDER_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == rate)
        .expect("a launch rate");
    PrepareEffectRequest {
        sample_rate: rate,
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
            maximum_scratch_bytes: quality.scratch_fixed_bytes,
            maximum_automation_spans_per_block: 16,
        },
        tail_bound: conformance::tail_bound_of(
            Box::new(crate::GateExpanderFactory),
            rate,
            EffectQuality::Normal,
        ),
    }
}

/// Random initial values in every parameter's domain, each channel its own.
fn draw_values(draw: &mut Draw) -> Vec<InitialParameterValue> {
    (0..PARAMETER_COUNT * 2)
        .map(|slot| {
            let spec = &GATE_EXPANDER_PARAMETERS[slot / 2];
            InitialParameterValue {
                parameter_index: (slot / 2) as u32,
                channel: if slot % 2 == 0 {
                    ParameterChannel::Left
                } else {
                    ParameterChannel::Right
                },
                value: draw.in_domain(spec.minimum.unwrap(), spec.maximum.unwrap()),
            }
        })
        .collect()
}

/// Two full banks of random per-lane values at one rate and link.
struct GateProbe {
    width: BankWidth,
    members: [Vec<Vec<InitialParameterValue>>; 2],
    rate: u32,
    link: LinkMode,
    sizes: StatePayloadSizes,
    /// One stream per strip.
    feeds: Vec<Draw>,
}

impl GateProbe {
    fn new(width: BankWidth, seed: u64) -> Self {
        let lanes = width.lanes() as usize;
        let mut draw = Draw::new(seed);
        let rate = draw.pick(&RATES);
        let link = draw.pick(&LINKS);
        let members: [Vec<Vec<InitialParameterValue>>; 2] =
            core::array::from_fn(|_| (0..lanes).map(|_| draw_values(&mut draw)).collect());
        let sizes = GateExpanderFactory
            .prepare(request(&members[0][0], rate, link))
            .expect("a member prepares")
            .metadata
            .state_sizes;
        Self {
            width,
            members,
            rate,
            link,
            sizes,
            feeds: (0..2 * lanes)
                .map(|strip| Draw::new(seed ^ (0x1747_0000 + strip as u64)))
                .collect(),
        }
    }
}

impl Probe for GateProbe {
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
        GATE_EXPANDER_DESCRIPTOR.state_layout_version
    }

    fn bind(&mut self, which: usize) -> Box<dyn PreparedNativeEffectBank> {
        let mask = vec![true; self.lanes()];
        let requests: Vec<_> = self.members[which]
            .iter()
            .map(|values| request(values, self.rate, self.link))
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
        for (parameter, spec) in GATE_EXPANDER_PARAMETERS.iter().enumerate().take(RAMP_COUNT) {
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

    /// A channel section's ramps are quadruples from `STATE_RAMP_WORD`, `remaining` last.
    fn ramp_in_flight(&self, left: &[u8], right: &[u8]) -> bool {
        [left, right].iter().any(|section| {
            (0..RAMP_COUNT)
                .any(|ramp| payload::read_f32(section, STATE_RAMP_WORD + ramp * 4 + 3) != 0.0)
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
#[test]
fn a_swapped_lane_renders_as_its_unswapped_twin() {
    bench_support::alloc::assert_installed();
    for width in BankWidth::ALL {
        for seed in 0..4_u64 {
            let seed = 0x1747_6000 + seed;
            swap_probe::check(&mut GateProbe::new(*width, seed), seed);
        }
    }
}
