//! Issue #1747 gate 1: a render-time lane swap moves a whole strip of the limiter's bank.
//!
//! Two banks `A` and `B` of one slot shape and their twins, which never swap. Every strip draws its
//! own parameters (ceiling, release, lookahead, per channel), its own loud input and its own
//! automation, all keyed by the strip and the block, never by the lane, so a strip's input and
//! automation follow it to its new lane. The block is shorter than the ramp window, and the block
//! before the swap retargets every strip, so every swapped lane has a ramp in flight. The swaps are
//! one within `A` (`second = None`) and one across `A` and `B`. Every strip's output must keep its
//! twin's bits block for block, and its payload must equal its twin's at the end.

use super::rest_tests::{Draw, QUANTUM, RATE, limiter_bank};
use super::*;

/// The block length: shorter than `RAMP_UPDATES`, so a ramp opened in one block is in flight at
/// the next block boundary.
const FRAMES: usize = 40;
const BEFORE: u64 = 12;
const AFTER: u64 = 16;

/// The strip's input for `block`: loud, so gain reduction and the rings are non-trivial.
fn strip_input(seed: u64, strip: usize, block: u64, left: &mut [f32], right: &mut [f32]) {
    let mut draw = Draw(seed ^ (strip as u64).wrapping_mul(0xA24B_AED4_963E_E407) ^ block << 32);
    let gain = draw.within(0.2, 3.0);
    for (l, r) in left.iter_mut().zip(right.iter_mut()) {
        *l = gain * draw.within(-1.0, 1.0);
        *r = gain * draw.within(-1.0, 1.0);
    }
}

/// The strip's automation for `block`, in ascending `(parameter, channel)` order. The block
/// before the swap retargets the ceiling on both channels of every strip.
fn strip_spans(
    seed: u64,
    strip: usize,
    block: u64,
    first_sample: u64,
) -> Vec<PreparedAutomationSpan> {
    let mut draw = Draw(!seed ^ (strip as u64).wrapping_mul(0x9FB2_1C65_1E98_DF25) ^ block << 40);
    let forced = block + 1 == BEFORE;
    let mut spans = Vec::new();
    for parameter_index in 0..2_u32 {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            let wanted = (forced && parameter_index == 0) || draw.below(5) == 0;
            if !wanted {
                continue;
            }
            spans.push(PreparedAutomationSpan {
                channel,
                parameter_index,
                sample: first_sample,
                value: if parameter_index == 0 {
                    draw.within(-24.0, 0.0)
                } else {
                    draw.within(10.0, 300.0)
                },
            });
        }
    }
    spans
}

/// One bank and the strip at each of its lanes.
struct Seat<L: Lane> {
    bank: PreparedTruePeakLimiterBank<L>,
    strips: Vec<usize>,
}

/// Renders one block of every bank, each lane fed its strip's input and automation, and returns
/// each strip's output, `(left, right)` per frame.
fn render_all<L: Lane>(seats: &mut [Seat<L>], seed: u64, block: u64) -> Vec<Vec<(u32, u32)>> {
    let first_sample = block * FRAMES as u64;
    let strips = seats.len() * L::WIDTH;
    let mut outputs = vec![Vec::new(); strips];
    let mut strip_left = vec![0.0_f32; FRAMES];
    let mut strip_right = vec![0.0_f32; FRAMES];
    for seat in seats.iter_mut() {
        let mut left = vec![0.0_f32; FRAMES * L::WIDTH];
        let mut right = vec![0.0_f32; FRAMES * L::WIDTH];
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32];
        for (lane, &strip) in seat.strips.iter().enumerate() {
            strip_input(seed, strip, block, &mut strip_left, &mut strip_right);
            for frame in 0..FRAMES {
                left[frame * L::WIDTH + lane] = strip_left[frame];
                right[frame * L::WIDTH + lane] = strip_right[frame];
            }
            spans.extend(strip_spans(seed, strip, block, first_sample));
            offsets.push(spans.len() as u32);
        }
        let process = EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            FRAMES as u32,
            seat.bank.width,
            first_sample,
            &spans,
            &offsets,
            QUANTUM,
        )
        .expect("bank block");
        let report = seat.bank.process_bank(process);
        assert!(report.reports.iter().all(|track| track.invalid_spans == 0));
        for (lane, &strip) in seat.strips.iter().enumerate() {
            outputs[strip] = (0..FRAMES)
                .map(|frame| {
                    (
                        left[frame * L::WIDTH + lane].to_bits(),
                        right[frame * L::WIDTH + lane].to_bits(),
                    )
                })
                .collect();
        }
    }
    outputs
}

/// Every strip's state payload, by strip.
fn payloads<L: Lane>(seats: &[Seat<L>]) -> Vec<Vec<u8>> {
    let sizes = quality(RATE).maximum_state;
    let total = sizes.total().expect("total") as usize;
    let mut out = vec![Vec::new(); seats.len() * L::WIDTH];
    for seat in seats {
        for (lane, &strip) in seat.strips.iter().enumerate() {
            let mut payload = vec![0_u8; total];
            let (common, rest) = payload.split_at_mut(sizes.common_bytes as usize);
            let (left, right) = rest.split_at_mut(sizes.left_bytes as usize);
            seat.bank
                .snapshot_track_state_payload(
                    lane as u32,
                    StatePayloadOutput::new(common, left, right, sizes).expect("output"),
                )
                .expect("snapshot");
            out[strip] = payload;
        }
    }
    out
}

/// A seat of strips `strips`, each drawing its own parameters from `seed`.
fn seat<L: Lane>(seed: u64, strips: core::ops::Range<usize>, link_mode: LinkMode) -> Seat<L> {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for strip in strips.clone() {
        let mut draw = Draw(seed ^ 0x5EA7 ^ (strip as u64) << 48);
        for side in [&mut left, &mut right] {
            side.push([
                draw.within(-18.0, -1.0),
                draw.within(10.0, 300.0),
                draw.within(0.0, 10.0),
            ]);
        }
    }
    Seat {
        bank: limiter_bank::<L>(left, right, false, link_mode).0,
        strips: strips.collect(),
    }
}

/// **A swapped strip renders and holds exactly the bits it would have unswapped** (issue #1747).
///
/// The allocation count of the two swap calls is exact: zero, on this thread, after the banks have
/// rendered `BEFORE` blocks.
#[test]
fn a_swapped_strip_keeps_its_twins_bits() {
    lane::each_vector_lane!(|L| {
        let width = L::WIDTH;
        for seed in 0..3_u64 {
            let seed = 0x1747_5000 ^ seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ width as u64;
            let link_mode = if seed.is_multiple_of(2) {
                LinkMode::Maximum
            } else {
                LinkMode::DualMono
            };
            let build = || {
                [
                    seat::<L>(seed, 0..width, link_mode),
                    seat::<L>(seed, width..2 * width, link_mode),
                ]
            };
            let (mut swapped, mut twins) = (build(), build());
            for block in 0..BEFORE {
                assert_eq!(
                    render_all(&mut swapped, seed, block),
                    render_all(&mut twins, seed, block)
                );
            }
            let (within, across) = ((0, width - 1), (1, width / 2));
            for (seat, lane) in [(0, within.0), (0, within.1), (0, across.0), (1, across.1)] {
                let core = &swapped[seat].bank.core;
                assert!(
                    core.left.limit[lane].remaining > 0 && core.right.limit[lane].remaining > 0,
                    "a swapped lane has a ramp in flight"
                );
            }
            assert_eq!(swapped[0].bank.core.cursors, swapped[1].bank.core.cursors);
            let sizes = quality(RATE).maximum_state;
            let scratch_bytes =
                effect_contract::swap_bank_lanes_scratch_bytes(&swapped[0].bank, sizes)
                    .expect("scratch bytes");
            let mut scratch = vec![0_u8; scratch_bytes];
            let mark = bench_support::alloc::current_thread_counters();
            let first = effect_contract::swap_bank_lanes(
                &mut swapped[0].bank,
                within.0,
                None,
                within.1,
                STATE_LAYOUT_VERSION,
                sizes,
                &mut scratch,
            );
            let (a, b) = swapped.split_at_mut(1);
            let second = effect_contract::swap_bank_lanes(
                &mut a[0].bank,
                across.0,
                Some(&mut b[0].bank),
                across.1,
                STATE_LAYOUT_VERSION,
                sizes,
                &mut scratch,
            );
            let allocations = bench_support::alloc::current_thread_delta_since(mark).allocations;
            assert_eq!(first, Ok(()));
            assert_eq!(second, Ok(()));
            assert_eq!(allocations, 0, "the swap allocated");
            swapped[0].strips.swap(within.0, within.1);
            let moved = swapped[0].strips[across.0];
            swapped[0].strips[across.0] = swapped[1].strips[across.1];
            swapped[1].strips[across.1] = moved;
            for block in BEFORE..BEFORE + AFTER {
                let ours = render_all(&mut swapped, seed, block);
                let theirs = render_all(&mut twins, seed, block);
                for (strip, (ours, theirs)) in ours.iter().zip(theirs.iter()).enumerate() {
                    assert_eq!(
                        ours, theirs,
                        "W{width} seed {seed:x}: strip {strip}, block {block}"
                    );
                }
            }
            assert_eq!(payloads(&swapped), payloads(&twins));
            // The prepared defaults moved with each strip: a full reset re-seeds every strip from
            // its own defaults, wherever it now sits.
            for seat in swapped.iter_mut().chain(twins.iter_mut()) {
                seat.bank.reset(ResetKind::FullToDefaults);
            }
            let block = BEFORE + AFTER;
            assert_eq!(
                render_all(&mut swapped, seed, block),
                render_all(&mut twins, seed, block),
                "W{width} seed {seed:x}: after a full reset"
            );
            assert_eq!(payloads(&swapped), payloads(&twins));
        }
    });
}
