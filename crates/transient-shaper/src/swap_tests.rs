//! Issue #1747: the render-time lane swap of the transient shaper's state.
//!
//! The shaper has no clock: every word a block of `+0.0` changes on a lane at rest is none (the
//! two envelopes floor at `+0.0` and stay, and a settled ramp holds), and its only per-bank words
//! are prepared constants (the follower coefficients, bypass, link mode, capacities) and the D7
//! record. So the contract's default no-op `advance_clocks` is exact, and nothing here overrides
//! it. What this module holds is gate 1: `effect_contract::swap_bank_lanes` exchanges two strips'
//! whole state, within one bank and between two banks of one slot, with a ramp in flight,
//! allocation-free, so every strip renders the bits of a twin that never moved.
//!
//! A child of `padding_tests`, so it reuses that module's bank binding, draws and payload reads.

use super::*;
use bench_support::alloc;

/// The rate every bank here runs at: the shaper's state shape does not depend on it.
const SWAP_RATE: u32 = 48_000;

// ---------------------------------------------------------------------------------------------
// Gate 1: a swapped strip renders its twin's bits
// ---------------------------------------------------------------------------------------------

/// One block of loud noise: uniform in `±level`, with the level drawn up to `+12 dB`.
fn loud(draw: &mut Draw, frames: usize) -> Vec<f32> {
    let level = [0.5_f32, 1.0, 2.0, 4.0][draw.below(4)];
    (0..frames)
        .map(|_| (draw.unit() * 2.0 - 1.0) * level)
        .collect()
}

/// A point span on every parameter and channel at a fresh drawn value, so ramps start moving.
fn every_span(draw: &mut Draw, first_sample: u64) -> Vec<PreparedAutomationSpan> {
    let mut spans = Vec::new();
    for (index, parameter) in TRANSIENT_SHAPER_PARAMETERS.iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            spans.push(PreparedAutomationSpan {
                channel,
                parameter_index: index as u32,
                sample: first_sample,
                value: draw.value(parameter),
            });
        }
    }
    spans
}

/// `true` if any ramp of a payload's two channel sections is in flight (`remaining > 0`).
fn ramp_in_flight((_, left, right): &Payload) -> bool {
    [left, right].iter().any(|section| {
        (0..PARAMETER_COUNT).any(|parameter| {
            let at = (2 + parameter * RAMP_WORDS + 3) * 4;
            u32::from_le_bytes([
                section[at],
                section[at + 1],
                section[at + 2],
                section[at + 3],
            ]) != 0
        })
    })
}

/// Two banks of one slot.
type Pair = [Box<dyn PreparedNativeEffectBank>; 2];

/// Renders one block through both banks of a pair, strip `s` sitting at `place[s]` (bank, lane)
/// with its own input and spans, and returns each strip's two output planes (raw bits) and report.
#[allow(clippy::type_complexity)]
fn render_pair(
    banks: &mut Pair,
    width: BankWidth,
    place: &[(usize, usize)],
    first_sample: u64,
    input: &[[Vec<f32>; 2]],
    spans: &[Vec<PreparedAutomationSpan>],
) -> Vec<([Vec<u32>; 2], ProcessReport)> {
    let lanes = width.lanes() as usize;
    let frames = input[0][0].len();
    let mut out = vec![([Vec::new(), Vec::new()], ProcessReport::default()); place.len()];
    for (index, bank) in banks.iter_mut().enumerate() {
        let mut occupant = vec![usize::MAX; lanes];
        for (strip, (at_bank, at_lane)) in place.iter().enumerate() {
            if *at_bank == index {
                occupant[*at_lane] = strip;
            }
        }
        let mut left = vec![0.0_f32; frames * lanes];
        let mut right = vec![0.0_f32; frames * lanes];
        let mut automation = Vec::new();
        let mut offsets = vec![0_u32];
        for (lane, strip) in occupant.iter().enumerate() {
            for frame in 0..frames {
                left[frame * lanes + lane] = input[*strip][0][frame];
                right[frame * lanes + lane] = input[*strip][1][frame];
            }
            automation.extend_from_slice(&spans[*strip]);
            offsets.push(automation.len() as u32);
        }
        let report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                frames as u32,
                width,
                first_sample,
                &automation,
                &offsets,
                QUANTUM,
            )
            .expect("a well-shaped bank block"),
        );
        for (lane, strip) in occupant.iter().enumerate() {
            let plane = |buffer: &[f32]| -> Vec<u32> {
                (0..frames)
                    .map(|frame| buffer[frame * lanes + lane].to_bits())
                    .collect()
            };
            out[*strip] = ([plane(&left), plane(&right)], report.reports[lane]);
        }
    }
    out
}

/// One `swap_bank_lanes` call between `(bank, lane)` positions `a` and `b`, `b` in the same bank
/// or (with `a` in bank 0) in bank 1.
fn swap(
    banks: &mut Pair,
    a: (usize, usize),
    b: (usize, usize),
    sizes: StatePayloadSizes,
    scratch: &mut [u8],
) -> Result<(), effect_contract::LaneSwapRefusal> {
    let version = TRANSIENT_SHAPER_DESCRIPTOR.state_layout_version;
    if a.0 == b.0 {
        effect_contract::swap_bank_lanes(
            banks[a.0].as_mut(),
            a.1,
            None,
            b.1,
            version,
            sizes,
            scratch,
        )
    } else {
        let [first, second] = banks;
        effect_contract::swap_bank_lanes(
            first.as_mut(),
            a.1,
            Some(second.as_mut()),
            b.1,
            version,
            sizes,
            scratch,
        )
    }
}

/// Gate 1 of #1747: two banks A and B of one slot and their twins, every strip on its own drawn
/// parameters and automation and loud noise. After three blocks two lanes of A swap; after three
/// more a lane of A swaps with a lane of B; three more blocks follow. Each swap lands while every
/// swapped strip has a ramp in flight, and each strip's input and spans follow it to its new
/// lane. Every block, every strip's output bits and report equal its unmoved twin's, and after
/// each swap and at the end every strip's payload equals its twin's. Each measured swap call,
/// after an unmeasured exchange and exchange back of the same pair, makes exactly zero
/// allocations and frees.
///
/// It runs at every link mode, seed `s` at `LINKS[s % 3]`, because the linked detectors read both
/// channels of a lane and a restore must carry both.
///
/// Red if a restore leaves a lane field behind (an envelope word, a ramp's step or countdown), if
/// it writes one channel's words into the other, if the swap allocates, or if a strip's prepared
/// defaults stay behind on its old lane (the last block, after a full reset of every bank).
#[test]
fn a_swapped_strip_renders_its_unmoved_twins_bits() {
    for width in WIDTHS.iter().copied() {
        let lanes = width.lanes() as usize;
        let strips = 2 * lanes;
        let sizes = scalar(request(&default_values(), SWAP_RATE, LinkMode::DualMono))
            .metadata
            .state_sizes;
        for seed in 0..6_u64 {
            let link = LINKS[seed as usize % LINKS.len()];
            let mut draw = Draw::new(0x1747_5000 + seed);
            let values: Vec<Values> = (0..strips).map(|_| draw.values()).collect();
            let mask = vec![true; lanes];
            let make = |bank: usize| {
                let requests: Vec<PrepareEffectRequest<'_>> = (0..lanes)
                    .map(|lane| request(&values[bank * lanes + lane], SWAP_RATE, link))
                    .collect();
                bind(width, &requests, &mask)
                    .expect("a well-formed request")
                    .expect("the shaper binds")
                    .processor
            };
            let mut banks: Pair = [make(0), make(1)];
            let mut twins: Pair = [make(0), make(1)];
            let total = effect_contract::swap_bank_lanes_scratch_bytes(banks[0].as_ref(), sizes)
                .expect("a bank that declares its defaults");
            let mut scratch = vec![0_u8; total];
            let home: Vec<(usize, usize)> = (0..strips).map(|s| (s / lanes, s % lanes)).collect();
            let mut place = home.clone();
            let within = {
                let first = draw.below(lanes);
                (first, (first + 1 + draw.below(lanes - 1)) % lanes)
            };
            let across = (draw.below(lanes), draw.below(lanes));
            let swaps = [
                ((0, within.0), (0, within.1)),
                ((0, across.0), (1, across.1)),
            ];
            let mut first_sample = 0_u64;
            for (phase, next_swap) in swaps.iter().map(Some).chain([None]).enumerate() {
                for block in 0..3 {
                    let context =
                        format!("{width:?} {link:?} seed {seed} phase {phase} block {block}");
                    let before_swap = block == 2 && next_swap.is_some();
                    let frames = if before_swap {
                        1 + draw.below(48)
                    } else {
                        1 + draw.below(QUANTUM as usize)
                    };
                    let input: Vec<[Vec<f32>; 2]> = (0..strips)
                        .map(|_| [loud(&mut draw, frames), loud(&mut draw, frames)])
                        .collect();
                    let spans: Vec<Vec<PreparedAutomationSpan>> = (0..strips)
                        .map(|_| {
                            if before_swap {
                                every_span(&mut draw, first_sample)
                            } else if draw.chance(1, 3) {
                                draw.spans(first_sample)
                            } else {
                                Vec::new()
                            }
                        })
                        .collect();
                    let moved =
                        render_pair(&mut banks, width, &place, first_sample, &input, &spans);
                    let still = render_pair(&mut twins, width, &home, first_sample, &input, &spans);
                    for strip in 0..strips {
                        assert_eq!(
                            moved[strip], still[strip],
                            "{context}: strip {strip} at {:?} (home {:?})",
                            place[strip], home[strip]
                        );
                    }
                    first_sample += frames as u64;
                }
                let Some(&(a, b)) = next_swap else { continue };
                let context =
                    format!("{width:?} {link:?} seed {seed} swap {phase} {a:?} <-> {b:?}");
                let strip_at = |at: (usize, usize)| place.iter().position(|p| *p == at).unwrap();
                let (sa, sb) = (strip_at(a), strip_at(b));
                for strip in [sa, sb] {
                    let (bank, lane) = home[strip];
                    assert!(
                        ramp_in_flight(&bank_payload(twins[bank].as_ref(), lane, sizes)),
                        "{context}: strip {strip} has a ramp in flight at the swap"
                    );
                }
                swap(&mut banks, a, b, sizes, &mut scratch).expect("warm-up exchange");
                swap(&mut banks, a, b, sizes, &mut scratch).expect("warm-up exchange back");
                alloc::assert_installed();
                let mark = alloc::current_thread_counters();
                let swapped = swap(&mut banks, a, b, sizes, &mut scratch);
                let delta = alloc::current_thread_delta_since(mark);
                assert_eq!(swapped, Ok(()), "{context}: the swap");
                assert_eq!(
                    (delta.allocations, delta.deallocations, delta.reallocations),
                    (0, 0, 0),
                    "{context}: the swap reached the allocator"
                );
                place.swap(sa, sb);
                for strip in 0..strips {
                    let (bank, lane) = place[strip];
                    let (twin_bank, twin_lane) = home[strip];
                    assert_eq!(
                        bank_payload(banks[bank].as_ref(), lane, sizes),
                        bank_payload(twins[twin_bank].as_ref(), twin_lane, sizes),
                        "{context}: strip {strip}'s payload after the swap"
                    );
                }
            }
            assert_ne!(place, home, "{width:?} seed {seed}: the strips moved");
            for strip in 0..strips {
                let (bank, lane) = place[strip];
                let (twin_bank, twin_lane) = home[strip];
                assert_eq!(
                    bank_payload(banks[bank].as_ref(), lane, sizes),
                    bank_payload(twins[twin_bank].as_ref(), twin_lane, sizes),
                    "{width:?} seed {seed}: strip {strip}'s payload at the end"
                );
            }
            // A full reset re-seeds each lane from its prepared defaults, which moved with it.
            for bank in banks.iter_mut().chain(twins.iter_mut()) {
                bank.reset(ResetKind::FullToDefaults);
            }
            let frames = 1 + draw.below(QUANTUM as usize);
            let input: Vec<[Vec<f32>; 2]> = (0..strips)
                .map(|_| [loud(&mut draw, frames), loud(&mut draw, frames)])
                .collect();
            let none: Vec<Vec<PreparedAutomationSpan>> = vec![Vec::new(); strips];
            let moved = render_pair(&mut banks, width, &place, first_sample, &input, &none);
            let still = render_pair(&mut twins, width, &home, first_sample, &input, &none);
            for strip in 0..strips {
                let (bank, lane) = place[strip];
                let (twin_bank, twin_lane) = home[strip];
                assert_eq!(
                    moved[strip], still[strip],
                    "{width:?} {link:?} seed {seed}: strip {strip}'s block after a full reset"
                );
                assert_eq!(
                    bank_payload(banks[bank].as_ref(), lane, sizes),
                    bank_payload(twins[twin_bank].as_ref(), twin_lane, sizes),
                    "{width:?} {link:?} seed {seed}: strip {strip}'s payload after a full reset"
                );
            }
        }
    }
}
