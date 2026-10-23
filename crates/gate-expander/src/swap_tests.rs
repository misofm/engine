//! Issue #1747 gate 1: a render-time lane swap moves a gate/expander lane whole, at every bank
//! width the build has (`BankWidth::ALL`: `Simd4` and, with AVX2, `Simd8`).
//!
//! A crate-internal test for the same reason as `padding_tests`: the width the build does not
//! execute binds through `bind_bank::<false>`, the factory's own code without the D4 width check.
//!
//! The gate has no lane clock: `hold` is a countdown, but its value depends on the detector, and
//! a lane at rest holds it at zero. Its one bank-wide counter, `ramp_frames_left`, only chooses
//! where `run_block` splits a block between its ramping and settled variants, and the render is
//! partition-invariant, so no output bit or payload word reads it.

use super::*;
use dsp_reference::randomized::{Draw, Profile};
use effect_contract::{PrepareEffectLimits, PreparedPorts, swap_bank_lanes};

const QUANTUM: u32 = 128;
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

/// Not a divisor of the 64-sample smoothing: a span at a block's first sample is still ramping
/// when the block ends, so the ramps of every strip are in flight at a swap.
const FRAMES: usize = 48;
/// Blocks before the swap inside bank A.
const BEFORE: usize = 10;
/// Blocks between that swap and the swap across banks A and B.
const BETWEEN: usize = 6;
/// Blocks after the second swap.
const AFTER: usize = 20;

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

/// The four banks of one case: A and B swap, `A'` and `B'` are their twins and never swap.
struct Rig {
    banks: [Box<dyn PreparedNativeEffectBank>; 4],
    width: BankWidth,
    lanes: usize,
    sizes: StatePayloadSizes,
    version: u32,
}

/// One block of automation on one strip: each smoothed parameter, left then right, in the order
/// the effect admits, each with probability one third, or every one of them when `all`.
fn draw_spans(draw: &mut Draw, all: bool, first_sample: u64) -> Vec<PreparedAutomationSpan> {
    let mut spans = Vec::new();
    for (parameter, spec) in GATE_EXPANDER_PARAMETERS.iter().enumerate().take(RAMP_COUNT) {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            if all || draw.chance(1, 3) {
                spans.push(PreparedAutomationSpan {
                    channel,
                    parameter_index: parameter as u32,
                    sample: first_sample,
                    value: draw.in_domain(spec.minimum.unwrap(), spec.maximum.unwrap()),
                });
            }
        }
    }
    spans
}

/// One block of input on one channel: an ordinary, loud, tiny or exact level, so the envelope
/// attacks and releases and its state is never trivial.
fn draw_input(draw: &mut Draw) -> Vec<f32> {
    let profile = draw.pick(&[Profile::Loud, Profile::Clean, Profile::Tiny, Profile::Exact]);
    (0..FRAMES).map(|_| draw.sample(profile)).collect()
}

/// The payload of `lane` of `bank`, left then right.
fn payload(rig: &Rig, bank: usize, lane: usize) -> Vec<u8> {
    let (mut left, mut right) = (
        vec![0_u8; rig.sizes.left_bytes as usize],
        vec![0_u8; rig.sizes.right_bytes as usize],
    );
    let mut common = vec![0_u8; rig.sizes.common_bytes as usize];
    rig.banks[bank]
        .snapshot_track_state_payload(
            lane as u32,
            StatePayloadOutput::new(&mut common, &mut left, &mut right, rig.sizes)
                .expect("the prepared sizes"),
        )
        .expect("a snapshot");
    common.into_iter().chain(left).chain(right).collect()
}

/// Swaps `(0, first)` with `(bank, second)` (`bank` 0 swaps inside A) on the swapping pair,
/// counting the calling thread's allocations, and moves the two strips in `place` with it.
fn swap(
    rig: &mut Rig,
    place: &mut [(usize, usize)],
    first: usize,
    bank: usize,
    second: usize,
    scratch: &mut [u8],
) {
    let mark = bench_support::alloc::current_thread_counters();
    let [a, b, ..] = &mut rig.banks;
    let result = if bank == 0 {
        swap_bank_lanes(
            a.as_mut(),
            first,
            None,
            second,
            rig.version,
            rig.sizes,
            scratch,
        )
    } else {
        swap_bank_lanes(
            a.as_mut(),
            first,
            Some(b.as_mut()),
            second,
            rig.version,
            rig.sizes,
            scratch,
        )
    };
    let moved = bench_support::alloc::current_thread_delta_since(mark);
    result.expect("a swap of two populated lanes of one layout");
    assert_eq!(moved.allocations, 0, "a lane swap allocates nothing");
    for spot in place.iter_mut() {
        if *spot == (0, first) {
            *spot = (bank, second);
        } else if *spot == (bank, second) {
            *spot = (0, first);
        }
    }
}

/// The shared driver: every strip keeps its own input and automation stream wherever it sits.
/// Strip `s` starts on bank `s / lanes`, lane `s % lanes`, and stays there in the twins. The
/// swapping pair swaps two lanes of A, then a lane of A with a lane of B. The block before the
/// first swap automates every parameter of every strip, so every ramp is in flight across it. The
/// two blocks before the second swap automate only one of its two strips, every parameter, and the
/// block after it automates nothing, so a ramping lane moves into a bank that has settled and must
/// keep ramping there with no new target to restart it.
fn run(rig: &mut Rig, seed: u64) {
    let lanes = rig.lanes;
    let strips = 2 * lanes;
    let mut draw = Draw::new(seed);
    let mut feeds: Vec<Draw> = (0..strips)
        .map(|strip| Draw::new(seed ^ (0x1747_0000 + strip as u64)))
        .collect();
    let mut place: Vec<(usize, usize)> = (0..strips).map(|s| (s / lanes, s % lanes)).collect();
    let inside = draw.below(lanes);
    let inside_other = (inside + 1 + draw.below(lanes - 1)) % lanes;
    let across = (draw.below(lanes), draw.below(lanes));
    let mover = if draw.chance(1, 2) {
        (0, across.0)
    } else {
        (1, across.1)
    };
    let total = rig.sizes.total().expect("a payload size") as usize;
    let mut scratch = vec![0_u8; 2 * total];
    let mut first_sample = 0_u64;
    for block in 0..BEFORE + BETWEEN + AFTER {
        if block == BEFORE {
            swap(rig, &mut place, inside, 0, inside_other, &mut scratch);
        }
        if block == BEFORE + BETWEEN {
            swap(rig, &mut place, across.0, 1, across.1, &mut scratch);
        }
        let everyone = block + 1 == BEFORE;
        let only_mover = (BEFORE + BETWEEN - 2..BEFORE + BETWEEN).contains(&block);
        let nobody = block == BEFORE + BETWEEN;
        let mut left = vec![vec![0.0_f32; FRAMES * lanes]; 4];
        let mut right = left.clone();
        let mut spans: Vec<Vec<Vec<PreparedAutomationSpan>>> = vec![vec![Vec::new(); lanes]; 4];
        for (strip, feed) in feeds.iter_mut().enumerate() {
            let input = (draw_input(feed), draw_input(feed));
            let automation = if nobody {
                Vec::new()
            } else if only_mover {
                if place[strip] == mover {
                    draw_spans(feed, true, first_sample)
                } else {
                    Vec::new()
                }
            } else {
                draw_spans(feed, everyone, first_sample)
            };
            let twin = (2 + strip / lanes, strip % lanes);
            for (bank, lane) in [place[strip], twin] {
                for frame in 0..FRAMES {
                    left[bank][frame * lanes + lane] = input.0[frame];
                    right[bank][frame * lanes + lane] = input.1[frame];
                }
                spans[bank][lane].clone_from(&automation);
            }
        }
        let mut reports = Vec::new();
        for bank in 0..4 {
            let mut offsets = vec![0_u32];
            for lane in &spans[bank] {
                offsets.push(offsets[offsets.len() - 1] + lane.len() as u32);
            }
            let flat: Vec<PreparedAutomationSpan> = spans[bank].concat();
            let process = EffectBankProcessBlock::new(
                &mut left[bank],
                &mut right[bank],
                None,
                FRAMES as u32,
                rig.width,
                first_sample,
                &flat,
                &offsets,
                QUANTUM,
            )
            .expect("a bank block");
            reports.push(rig.banks[bank].process_bank(process));
        }
        for (strip, (bank, lane)) in place.iter().copied().enumerate() {
            let (twin, twin_lane) = (2 + strip / lanes, strip % lanes);
            for frame in 0..FRAMES {
                for (plane, name) in [(&left, "left"), (&right, "right")] {
                    assert_eq!(
                        plane[bank][frame * lanes + lane].to_bits(),
                        plane[twin][frame * lanes + twin_lane].to_bits(),
                        "W{lanes} seed {seed:#x}: strip {strip} {name} frame {frame} of block \
                         {block} differs from its twin"
                    );
                }
            }
            assert_eq!(
                reports[bank].reports[lane], reports[twin].reports[twin_lane],
                "W{lanes} seed {seed:#x}: strip {strip} report of block {block}"
            );
        }
        first_sample += FRAMES as u64;
    }
    for (strip, (bank, lane)) in place.iter().copied().enumerate() {
        assert_eq!(
            payload(rig, bank, lane),
            payload(rig, 2 + strip / lanes, strip % lanes),
            "W{lanes} seed {seed:#x}: strip {strip}'s payload differs from its twin's"
        );
    }
}

/// One case at `width`: two full banks of random per-lane values at one rate and link.
fn case(width: BankWidth, seed: u64) {
    let lanes = width.lanes() as usize;
    let mut draw = Draw::new(seed);
    let rate = draw.pick(&RATES);
    let link = draw.pick(&LINKS);
    let mask = vec![true; lanes];
    let members: [Vec<Vec<InitialParameterValue>>; 2] =
        core::array::from_fn(|_| (0..lanes).map(|_| draw_values(&mut draw)).collect());
    let bank = |which: usize| -> Box<dyn PreparedNativeEffectBank> {
        let requests: Vec<_> = members[which]
            .iter()
            .map(|values| request(values, rate, link))
            .collect();
        bind_bank::<false>(PrepareEffectBankRequest {
            backend: width.backend(),
            width,
            requests: &requests,
            active_mask: &mask,
        })
        .expect("a valid bank request")
        .expect("one program banks")
        .processor
    };
    let sizes = GateExpanderFactory
        .prepare(request(&members[0][0], rate, link))
        .expect("a member prepares")
        .metadata
        .state_sizes;
    let mut rig = Rig {
        banks: [bank(0), bank(1), bank(0), bank(1)],
        width,
        lanes,
        sizes,
        version: GATE_EXPANDER_DESCRIPTOR.state_layout_version,
    };
    run(&mut rig, seed);
}

/// **Gate 1: a strip whose lane is swapped, inside one bank and across two, with its ramps in
/// flight, renders bit for bit as its twin that never moved, and ends on the twin's payload; the
/// swap allocates nothing. At every bank width the build has.**
#[test]
fn a_swapped_lane_renders_as_its_unswapped_twin() {
    bench_support::alloc::assert_installed();
    for width in BankWidth::ALL {
        for seed in 0..4_u64 {
            case(*width, 0x1747_6000 + seed);
        }
    }
}
