//! Issue #1747: the soft clip's one clock, and the render-time lane swap of its state.
//!
//! * `advance_clocks` moves the bank's shared history position `pos` as a rendered block of
//!   `+0.0` would, and nothing else needs to move for a lane at rest;
//! * `effect_contract::swap_bank_lanes` exchanges two strips' whole state, within one bank and
//!   between two banks of one slot, with a ramp in flight, allocation-free, so every strip renders
//!   the bits of a twin that never moved (the shared driver `conformance::swap_probe`).
//!
//! A child of `padding_tests`, so it reuses that module's bank binding, draws and payload reads.

use super::*;
use bench_support::alloc;
use conformance::swap_probe::{self, Automate, Automation, Probe};

/// The rate every bank here runs at: the soft clip's state shape does not depend on it.
const SWAP_RATE: u32 = 48_000;

/// Every state word of a concrete bank, both channels, raw bits: the ramp vectors, the shared
/// position, the three histories, the control-plane ramps, the D7 record and the active mask.
fn state_words<L: Lane>(bank: &PreparedSoftClipBank<L>) -> Vec<u32> {
    let inner = &bank.inner;
    let mut words = vec![
        inner.active,
        inner.nonfinite.nonfinite_blocks as u32,
        (inner.nonfinite.nonfinite_blocks >> 32) as u32,
        inner.nonfinite.nonfinite_lanes,
    ];
    for channel in [&inner.left, &inner.right] {
        for parameter in 0..PARAMETER_COUNT {
            for lane in 0..L::WIDTH {
                words.push(lane_of(channel.state.field(parameter), lane).to_bits());
            }
        }
        words.push(channel.history.pos);
        for history in [&channel.history.x, &channel.history.e, &channel.history.dry] {
            words.extend(history.iter().map(|word| word.to_bits()));
        }
        for ramp in channel.ramps.iter().flatten() {
            words.extend([
                ramp.current.to_bits(),
                ramp.target.to_bits(),
                ramp.step.to_bits(),
                ramp.remaining,
            ]);
        }
    }
    words
}

/// Renders one block of `frames` frames through a concrete bank: lane `l` gets `input[l]` (both
/// channels) and `spans[l]`.
fn render_concrete<L: Lane>(
    bank: &mut PreparedSoftClipBank<L>,
    width: BankWidth,
    first_sample: u64,
    input: &[[Vec<f32>; 2]],
    spans: &[Vec<PreparedAutomationSpan>],
) {
    let lanes = L::WIDTH;
    let frames = input[0][0].len();
    let mut left = vec![0.0_f32; frames * lanes];
    let mut right = vec![0.0_f32; frames * lanes];
    let mut automation = Vec::new();
    let mut offsets = vec![0_u32];
    for lane in 0..lanes {
        for frame in 0..frames {
            left[frame * lanes + lane] = input[lane][0][frame];
            right[frame * lanes + lane] = input[lane][1][frame];
        }
        automation.extend_from_slice(&spans[lane]);
        offsets.push(automation.len() as u32);
    }
    bank.process_bank(
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
}

/// `advance_clocks` on a bank at rest is a rendered block of `+0.0`, word for word.
///
/// Two equal fresh banks with a padded lane: one renders blocks of `+0.0` and no automation through
/// its whole kernel (the silent fast path forbidden, since that path *is* the cursor advance this
/// test checks against), and its twin skips each block with `advance_clocks(every lane, frames)`.
/// The lengths include one frame, lengths below and at the quantum, and totals that wrap the
/// 32-row ring many times. After every block the twin's every state word and every lane's payload
/// equals the rendered bank's, the rendered bank's histories hold only `+0.0` (a zero block pushes
/// `+0.0`, so a resting lane already holds what the block would write), and its position is the
/// block lengths' sum modulo 32.
///
/// Red if `advance_clocks` is left the default no-op, advances only one channel's position, steps
/// by a length other than `frames` modulo 32, or writes any other word.
#[test]
fn advancing_the_clocks_is_a_rendered_zero_block() {
    fn at<L: Lane>(width: BankWidth) {
        let lanes = L::WIDTH;
        let mask = prefix_mask(width, lanes - 1);
        let all_lanes = (1_u32 << lanes) - 1;
        let sizes = scalar(request(&default_values(), SWAP_RATE, LinkMode::DualMono))
            .metadata
            .state_sizes;
        for seed in 0..4_u64 {
            let mut draw = Draw::new(0x1747_0000 + seed);
            let values: Vec<Values> = (0..lanes).map(|_| draw.values()).collect();
            let mut rendered = concrete::<L>(width, &mask, &values);
            let mut skipped = concrete::<L>(width, &mask, &values);
            assert!(
                [&rendered.inner.left, &rendered.inner.right]
                    .iter()
                    .all(
                        |channel| channel.ramps.iter().flatten().all(|r| r.remaining == 0)
                            && channel.history.pos == 0
                    ),
                "seed {seed}: a fresh bank is settled at position 0"
            );
            let mut lengths = vec![1_usize, 7, 31, 32, 33, 64, 127, QUANTUM as usize, 5];
            lengths.extend((0..8).map(|_| 1 + draw.below(QUANTUM as usize)));
            let mut first_sample = 0_u64;
            for (block, frames) in lengths.into_iter().enumerate() {
                let context = format!("{width:?} seed {seed} block {block} ({frames} frames)");
                let zeros: Vec<[Vec<f32>; 2]> = (0..lanes)
                    .map(|_| [vec![0.0; frames], vec![0.0; frames]])
                    .collect();
                let none: Vec<Vec<PreparedAutomationSpan>> = vec![Vec::new(); lanes];
                SILENT_FORBIDDEN.with(|flag| flag.set(true));
                render_concrete(&mut rendered, width, first_sample, &zeros, &none);
                SILENT_FORBIDDEN.with(|flag| flag.set(false));
                assert_eq!(
                    skipped.lanes_at_rest(),
                    all_lanes,
                    "{context}: every lane is at rest before the skip"
                );
                skipped.advance_clocks(all_lanes, frames as u32);
                first_sample += frames as u64;

                let words = state_words(&rendered);
                assert_eq!(state_words(&skipped), words, "{context}: a state word");
                for lane in 0..lanes {
                    assert_eq!(
                        bank_payload(&skipped, lane, sizes),
                        bank_payload(&rendered, lane, sizes),
                        "{context}: lane {lane}'s payload"
                    );
                }
                for channel in [&rendered.inner.left, &rendered.inner.right] {
                    assert!(
                        [&channel.history.x, &channel.history.e, &channel.history.dry]
                            .iter()
                            .all(|history| history.iter().all(|word| word.to_bits() == 0)),
                        "{context}: a zero block pushed a word other than +0.0"
                    );
                }
                let expected = (first_sample % LIVE_ROWS as u64) as u32;
                assert_eq!(
                    (
                        rendered.inner.left.history.pos,
                        rendered.inner.right.history.pos
                    ),
                    (expected, expected),
                    "{context}: the rendered position is not the block lengths' sum modulo 32"
                );
            }
        }
    }
    lane::each_vector_lane!(|L, N| at::<L>(BankWidth::for_lanes(N).expect("a bank width")));
}

// ---------------------------------------------------------------------------------------------
// Gate 1: a swapped strip renders its twin's bits (driver: `conformance::swap_probe`)
// ---------------------------------------------------------------------------------------------

/// Two full banks of one slot, every strip on its own drawn parameters.
struct SoftClipProbe {
    width: BankWidth,
    values: Vec<Values>,
    sizes: StatePayloadSizes,
    draw: Draw,
}

impl SoftClipProbe {
    fn new(width: BankWidth, seed: u64) -> Self {
        let strips = 2 * width.lanes() as usize;
        let mut draw = Draw::new(seed);
        let values = (0..strips).map(|_| draw.values()).collect();
        let sizes = scalar(request(&default_values(), SWAP_RATE, LinkMode::DualMono))
            .metadata
            .state_sizes;
        Self {
            width,
            values,
            sizes,
            draw,
        }
    }
}

impl Probe for SoftClipProbe {
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
        SOFT_CLIP_DESCRIPTOR.state_layout_version
    }

    fn bind(&mut self, which: usize) -> Box<dyn PreparedNativeEffectBank> {
        let lanes = self.lanes();
        let requests: Vec<PrepareEffectRequest<'_>> = (0..lanes)
            .map(|lane| {
                request(
                    &self.values[which * lanes + lane],
                    SWAP_RATE,
                    LinkMode::DualMono,
                )
            })
            .collect();
        bind(self.width, &requests, &vec![true; lanes])
            .expect("a well-formed request")
            .expect("the soft clip binds")
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
                for (index, parameter) in SOFT_CLIP_PARAMETERS.iter().enumerate() {
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

    /// A channel section opens with the ramps, `RAMP_WORDS` each, `remaining` last.
    fn ramp_in_flight(&self, left: &[u8], right: &[u8]) -> bool {
        [left, right].iter().any(|section| {
            (0..PARAMETER_COUNT).any(|parameter| {
                let at = (parameter * RAMP_WORDS + 3) * 4;
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
/// reaches the allocator zero times. The driver's random block lengths move the shared position
/// `pos` off any multiple of the ring, so a restore placed against the wrong position is red.
#[test]
fn a_swapped_strip_renders_its_unmoved_twins_bits() {
    for width in WIDTHS.iter().copied() {
        for seed in 0..6_u64 {
            let seed = 0x1747_5000 + seed;
            swap_probe::check(&mut SoftClipProbe::new(width, seed), seed);
        }
    }
}
