//! Issue #1092 (console strip P2e): the gate/expander binds a padded bank.
//!
//! A padded bank is `members < lanes` tracks bound as one bank of the build's width. The padding
//! contract (`effect_contract::PrepareEffectBankRequest`) owes each padded lane a clone of an
//! active member's request and a `+0.0` feed, and it discards the padded lane's output. The effect
//! owes the rest, and these tests hold the gate to it:
//!
//! * gate 1: every active lane of a padded bank is bit-identical to its track rendered per node
//!   (the scalar instance `prepare` returns), on random parameters and automation, on random input
//!   and on the conformance PCM fixtures, while every padded lane writes `+0.0` and reports nothing;
//! * gate 2: no lane is ever prepared with a parameter outside its declared domain;
//! * gate 3: an active lane's bits do not depend on which member the padded lanes clone;
//! * gate 4: a non-finite lane recovers and is reported alone, and a tripping lane (the wet path of
//!   a bypassed track, which the rack's `BypassShunt` later discards) leaves its bank-mates' bits.
//!
//! Every test runs at every bank width the build has (`BankWidth::ALL`). The build's native width
//! binds through the public `bind_homogeneous_bank`; the other width binds through
//! `bind_bank::<false>`, the same code without the D4 width check. So the four-lane bank runs in
//! the 8-lane (AVX2) build as well as in a 4-lane (NEON/simd128) one, which has no eight-lane bank
//! (#1112).
//!
//! NaNs fold to one word before any comparison (decision 10).

use super::*;
use effect_contract::{PrepareEffectLimits, PreparedPorts};

const QUANTUM: u32 = 128;
const WIDTHS: &[BankWidth] = BankWidth::ALL;
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

type Values = Vec<InitialParameterValue>;

/// Whether a padded lane's state stays exactly at its prepared value when fed `+0.0`. Not for the gate: fed `+0.0` its detector sits at the level floor, so a padded lane closes and
/// its gain releases toward the range, as an idle track's does; its parameters never move.
const PADDED_STATE_IS_CONSTANT: bool = false;

// ---------------------------------------------------------------------------------------------
// The effect under test
// ---------------------------------------------------------------------------------------------

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
    }
}

fn native(width: BankWidth) -> bool {
    BankWidth::for_backend(Backend::current()) == Some(width)
}

fn backend(width: BankWidth) -> Backend {
    width.backend()
}

/// Binds through the public factory at the native width, and through `bind_bank` otherwise.
fn bind(
    width: BankWidth,
    requests: &[PrepareEffectRequest<'_>],
    mask: &[bool],
) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
    let request = PrepareEffectBankRequest {
        backend: backend(width),
        width,
        requests,
        active_mask: mask,
    };
    if native(width) {
        GateExpanderFactory.bind_homogeneous_bank(request)
    } else {
        bind_bank::<false>(request)
    }
}

fn scalar(request: PrepareEffectRequest<'_>) -> Box<dyn PreparedNativeEffect> {
    GateExpanderFactory
        .prepare(request)
        .expect("a valid member prepares")
}

/// Initial values from one `(left, right)` pair per parameter.
fn values_from(pairs: [(f32, f32); PARAMETER_COUNT]) -> Values {
    (0..PARAMETER_COUNT * 2)
        .map(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index % 2 == 0 {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: if index % 2 == 0 {
                pairs[index / 2].0
            } else {
                pairs[index / 2].1
            },
        })
        .collect()
}

/// The descriptor defaults.
fn default_values() -> Values {
    values_from(GATE_EXPANDER_PARAMETERS.map(|p| (p.default_value, p.default_value)))
}

/// The crate's audible fixture set (`tests/support`'s `active_values`): a -20 dB threshold, a
/// 20:1 expander with 48 dB of range, 6 dB of hysteresis and fast times, and a lane-distinct
/// variant of it.
fn fixture_values(member: usize) -> Values {
    let threshold = -20.0 - 3.0 * member as f32;
    values_from([
        (threshold, threshold - 1.0),
        (20.0, 12.0),
        (48.0, 60.0),
        (6.0, 3.0),
        (1.0, 0.5),
        (0.0, 5.0),
        (5.0, 20.0),
    ])
}

/// A lane that trips D7 at the bank input. The gate never raises a level (`gain <= 1`), so only a
/// word already at the bound trips it: `1e30`, the "tripping value" of the spec's gate 4.
const TRIPPING_WORDS: [f32; 3] = [1.0e30, f32::INFINITY, f32::NAN];

// ---------------------------------------------------------------------------------------------
// Draws
// ---------------------------------------------------------------------------------------------

struct Draw(u64);

impl Draw {
    fn new(seed: u64) -> Self {
        Self(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }

    fn chance(&mut self, numerator: usize, denominator: usize) -> bool {
        self.below(denominator) < numerator
    }

    /// Uniform in `[0, 1)`.
    fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1_u64 << 24) as f32
    }

    /// A value inside one parameter's domain: an end, the default, or uniform between the ends.
    fn value(&mut self, parameter: &ParameterDescriptor) -> f32 {
        let minimum = parameter.minimum.expect("bounded");
        let maximum = parameter.maximum.expect("bounded");
        let value = match self.below(8) {
            0 => minimum,
            1 => maximum,
            2 => parameter.default_value,
            _ => minimum + self.unit() * (maximum - minimum),
        };
        // `-0.0` is refused at preparation, and the uniform draw may round to it.
        if value == 0.0 {
            0.0
        } else {
            value.clamp(minimum, maximum)
        }
    }

    fn values(&mut self) -> Values {
        let pairs = core::array::from_fn(|index| {
            let parameter = &GATE_EXPANDER_PARAMETERS[index];
            (self.value(parameter), self.value(parameter))
        });
        values_from(pairs)
    }

    /// One block's automation for one track: a point span per automatable parameter and channel,
    /// each with probability one in four, in ascending `(parameter, channel)` order.
    fn spans(&mut self, first_sample: u64) -> Vec<PreparedAutomationSpan> {
        let mut spans = Vec::new();
        for (index, parameter) in GATE_EXPANDER_PARAMETERS.iter().enumerate() {
            if !parameter.automatable {
                continue;
            }
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                if !self.chance(1, 4) {
                    continue;
                }
                let value = self.value(parameter);
                spans.push(PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel,
                    parameter_index: index as u32,
                    start_sample: first_sample,
                    end_sample: first_sample,
                    start_value: value,
                    end_value: value,
                });
            }
        }
        spans
    }

    /// One block of one channel: noise at a drawn level, silence, `-0.0`, subnormals or bursts.
    fn signal(&mut self, frames: usize) -> Vec<f32> {
        let level = [1.0e-4_f32, 0.01, 0.1, 0.5, 1.0, 4.0][self.below(6)];
        let profile = self.below(10);
        (0..frames)
            .map(|frame| {
                let noise = self.unit() * 2.0 - 1.0;
                match profile {
                    0 => 0.0,
                    1 => -0.0,
                    2 => f32::from_bits(1 + self.below(0x007f_ffff) as u32) * noise.signum(),
                    3 if (frame / 16) % 2 == 1 => noise * 1.0e-5,
                    _ => noise * level,
                }
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------------------------
// The padded-bank differential
// ---------------------------------------------------------------------------------------------

/// One NaN-folded word.
fn fold(word: f32) -> u32 {
    if word.is_nan() {
        f32::NAN.to_bits()
    } else {
        word.to_bits()
    }
}

/// The mask with lanes `0..members` active: the planner's members-first layout.
fn prefix_mask(width: BankWidth, members: usize) -> Vec<bool> {
    (0..width.lanes() as usize)
        .map(|lane| lane < members)
        .collect()
}

/// One block of a render: its length, and per member its two input planes and its spans.
struct Block {
    frames: usize,
    input: Vec<[Vec<f32>; 2]>,
    spans: Vec<Vec<PreparedAutomationSpan>>,
}

/// A padded bank's shape: which lanes carry which member, and which member the padding clones.
struct Padded<'a> {
    width: BankWidth,
    mask: &'a [bool],
    members: &'a [Values],
    clone_of: usize,
    rate: u32,
    link: LinkMode,
}

impl Padded<'_> {
    /// The member each lane carries, or `None` for a padded lane.
    fn lanes(&self) -> Vec<Option<usize>> {
        let mut next = 0;
        self.mask
            .iter()
            .map(|active| {
                active.then(|| {
                    next += 1;
                    next - 1
                })
            })
            .collect()
    }

    fn bind(&self) -> Box<dyn PreparedNativeEffectBank> {
        let lanes = self.lanes();
        assert_eq!(
            lanes.iter().flatten().count(),
            self.members.len(),
            "one member per active lane"
        );
        let requests: Vec<PrepareEffectRequest<'_>> = lanes
            .iter()
            .map(|member| {
                let values = &self.members[member.unwrap_or(self.clone_of)];
                request(values, self.rate, self.link)
            })
            .collect();
        bind(self.width, &requests, self.mask)
            .expect("a padded request is well formed")
            .expect("the gate binds a padded bank")
    }
}

/// Renders `blocks` through a padded bank and through each member's scalar instance, and checks
/// every block: each active lane's output words and report equal its scalar instance's, and each
/// padded lane writes `+0.0` and reports nothing, although every other block hands it stray spans.
/// At the end each active lane's state payload equals its scalar instance's. Returns each member's
/// output words, NaN folded.
fn render(padded: &Padded<'_>, blocks: &[Block], context: &str) -> Vec<Vec<u32>> {
    let lanes = padded.lanes();
    let width = lanes.len();
    let mut bank = padded.bind();
    let mut scalars: Vec<Box<dyn PreparedNativeEffect>> = padded
        .members
        .iter()
        .map(|values| scalar(request(values, padded.rate, padded.link)))
        .collect();
    let sizes = scalars[0].metadata().state_sizes;
    // Gate 3's padded-lane clause (P2a verdict, L4): fed `+0.0`, a padded lane keeps its state
    // finite and at rest, block after block. "At rest" is an idle track's state: the state of a
    // scalar instance of the member the lane clones, fed `+0.0` and no automation.
    let idle_request = request(&padded.members[padded.clone_of], padded.rate, padded.link);
    let rest = payload_of(scalar(idle_request).as_ref(), sizes);
    let mut idle: Vec<Option<Box<dyn PreparedNativeEffect>>> = lanes
        .iter()
        .map(|member| member.is_none().then(|| scalar(idle_request)))
        .collect();
    let mut outputs = vec![Vec::new(); padded.members.len()];
    let mut first_sample = 0_u64;
    for (index, block) in blocks.iter().enumerate() {
        let frames = block.frames;
        let mut left = vec![0.0_f32; frames * width];
        let mut right = vec![0.0_f32; frames * width];
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32];
        for (lane, member) in lanes.iter().enumerate() {
            if let Some(member) = *member {
                for frame in 0..frames {
                    left[frame * width + lane] = block.input[member][0][frame];
                    right[frame * width + lane] = block.input[member][1][frame];
                }
                spans.extend_from_slice(&block.spans[member]);
            } else if index % 2 == 1 {
                // A padded lane carries no track, so no span is its to apply: these strays, one of
                // them malformed, are ignored rather than applied or counted.
                spans.extend(
                    [ParameterChannel::Both, ParameterChannel::Left].map(|channel| {
                        PreparedAutomationSpan {
                            kind: AutomationSpanKind::Point,
                            channel,
                            parameter_index: 0,
                            start_sample: first_sample,
                            end_sample: first_sample,
                            start_value: -30.0,
                            end_value: -30.0,
                        }
                    }),
                );
            }
            offsets.push(spans.len() as u32);
        }
        let report = bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                frames as u32,
                padded.width,
                first_sample,
                &spans,
                &offsets,
                QUANTUM,
            )
            .expect("a well-shaped bank block"),
        );
        for (lane, member) in lanes.iter().enumerate() {
            let Some(member) = *member else {
                for frame in 0..frames {
                    assert_eq!(
                        (
                            left[frame * width + lane].to_bits(),
                            right[frame * width + lane].to_bits()
                        ),
                        (0, 0),
                        "{context}: block {index}: padded lane {lane} fed +0.0 wrote a word other \
                         than +0.0 at frame {frame}"
                    );
                }
                assert_eq!(
                    report.reports[lane],
                    ProcessReport::default(),
                    "{context}: block {index}: padded lane {lane} was reported"
                );
                continue;
            };
            let [mut own_left, mut own_right] = block.input[member].clone();
            let own = scalars[member].process(
                EffectProcessBlock::new(
                    &mut own_left,
                    &mut own_right,
                    None,
                    first_sample,
                    &block.spans[member],
                    QUANTUM,
                )
                .expect("a well-shaped block"),
            );
            for frame in 0..frames {
                let bank_words = (
                    fold(left[frame * width + lane]),
                    fold(right[frame * width + lane]),
                );
                let own_words = (fold(own_left[frame]), fold(own_right[frame]));
                assert_eq!(
                    bank_words, own_words,
                    "{context}: block {index}: lane {lane} (member {member}) frame {frame}: the \
                     padded bank rendered {bank_words:#010x?}, the track per node {own_words:#010x?}"
                );
            }
            assert_eq!(
                report.reports[lane], own,
                "{context}: block {index}: lane {lane} (member {member}) report"
            );
            outputs[member].extend(own_left.iter().chain(&own_right).map(|word| fold(*word)));
        }
        for (lane, idle) in idle.iter_mut().enumerate() {
            let Some(idle) = idle else { continue };
            let (mut silent_left, mut silent_right) = (vec![0.0; frames], vec![0.0; frames]);
            idle.process(
                EffectProcessBlock::new(
                    &mut silent_left,
                    &mut silent_right,
                    None,
                    first_sample,
                    &[],
                    QUANTUM,
                )
                .expect("a well-shaped block"),
            );
            let state = bank_payload(bank.as_ref(), lane, sizes);
            let (_, left_words, right_words) = &state;
            assert!(
                left_words
                    .chunks_exact(4)
                    .chain(right_words.chunks_exact(4))
                    .all(
                        |word| f32::from_le_bytes([word[0], word[1], word[2], word[3]]).is_finite()
                    ),
                "{context}: block {index}: padded lane {lane} holds a non-finite state word"
            );
            assert_eq!(
                state,
                payload_of(idle.as_ref(), sizes),
                "{context}: block {index}: padded lane {lane}'s state is not an idle track's"
            );
            if PADDED_STATE_IS_CONSTANT {
                assert_eq!(
                    state, rest,
                    "{context}: block {index}: padded lane {lane}'s state moved from its prepared \
                     value"
                );
            }
        }
        first_sample += frames as u64;
    }
    for (lane, member) in lanes.iter().enumerate() {
        let Some(member) = *member else { continue };
        assert_eq!(
            bank_payload(bank.as_ref(), lane, sizes),
            payload_of(scalars[member].as_ref(), sizes),
            "{context}: lane {lane} (member {member}): state after the render"
        );
    }
    outputs
}

type Payload = (Vec<u8>, Vec<u8>, Vec<u8>);

fn bank_payload(
    bank: &dyn PreparedNativeEffectBank,
    lane: usize,
    sizes: StatePayloadSizes,
) -> Payload {
    let (mut common, mut left, mut right) = sections(sizes);
    bank.snapshot_track_state_payload(
        lane as u32,
        StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
    )
    .expect("a bank lane snapshots");
    (common, left, right)
}

fn payload_of(effect: &dyn PreparedNativeEffect, sizes: StatePayloadSizes) -> Payload {
    let (mut common, mut left, mut right) = sections(sizes);
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
        )
        .expect("a scalar instance snapshots");
    (common, left, right)
}

fn sections(sizes: StatePayloadSizes) -> Payload {
    (
        vec![0; sizes.common_bytes as usize],
        vec![0; sizes.left_bytes as usize],
        vec![0; sizes.right_bytes as usize],
    )
}

/// `count` blocks of drawn lengths, input and automation for `members` tracks.
fn random_blocks(draw: &mut Draw, members: usize, count: usize) -> Vec<Block> {
    let mut first_sample = 0;
    (0..count)
        .map(|_| {
            let frames = if draw.chance(1, 2) {
                QUANTUM as usize
            } else {
                1 + draw.below(QUANTUM as usize)
            };
            let block = Block {
                frames,
                input: (0..members)
                    .map(|_| [draw.signal(frames), draw.signal(frames)])
                    .collect(),
                spans: (0..members)
                    .map(|_| {
                        if draw.chance(1, 3) {
                            draw.spans(first_sample)
                        } else {
                            Vec::new()
                        }
                    })
                    .collect(),
            };
            first_sample += frames as u64;
            block
        })
        .collect()
}

// ---------------------------------------------------------------------------------------------
// Gate 1: a padded bank renders its members' per-node bits
// ---------------------------------------------------------------------------------------------

/// Gate 1 on random parameters, automation and input: every active count `1..W` at both widths,
/// every link mode and every launch rate, members first as the planner lays them out.
///
/// Red if a padded lane is bound as zeros, runs a padded lane's automation, lets a padded lane's
/// recovery or report reach an active lane, or restores the #1088 decline (the bank does not bind).
#[test]
fn every_padded_bank_renders_its_members_per_node_bits() {
    let mut cases = 0;
    for &width in WIDTHS {
        let lanes = width.lanes() as usize;
        for members in 1..lanes {
            for (index, link) in LINKS.into_iter().enumerate() {
                let seed = (lanes * 100 + members * 10 + index) as u64;
                let mut draw = Draw::new(seed);
                let rate = RATES[draw.below(RATES.len())];
                let values: Vec<Values> = (0..members).map(|_| draw.values()).collect();
                let blocks = random_blocks(&mut draw, members, 10);
                let mask = prefix_mask(width, members);
                render(
                    &Padded {
                        width,
                        mask: &mask,
                        members: &values,
                        clone_of: 0,
                        rate,
                        link,
                    },
                    &blocks,
                    &format!("{width:?} {members} members {link:?} {rate} Hz seed {seed}"),
                );
                cases += 1;
            }
        }
    }
    let padded_counts: usize = WIDTHS.iter().map(|width| width.lanes() as usize - 1).sum();
    assert_eq!(cases, padded_counts * LINKS.len());
}

/// The conformance PCM fixtures, in their rows' order.
const FIXTURES: [&[u8]; 7] = [
    include_bytes!(
        "../../../fixtures/conformance/v1/multitone-near-nyquist-096000-dual-mono.mepcm"
    ),
    include_bytes!("../../../fixtures/conformance/v1/prng-noise-048000-dual-mono.mepcm"),
    include_bytes!("../../../fixtures/conformance/v1/rate-044100-impulse-dual-mono.mepcm"),
    include_bytes!("../../../fixtures/conformance/v1/rate-048000-impulse-dual-mono.mepcm"),
    include_bytes!("../../../fixtures/conformance/v1/rate-088200-impulse-dual-mono.mepcm"),
    include_bytes!("../../../fixtures/conformance/v1/rate-096000-impulse-dual-mono.mepcm"),
    include_bytes!("../../../fixtures/conformance/v1/sine-048000-mono.mepcm"),
];

/// Gate 1 on the fixtures: each conformance PCM fixture, at its own rate, through padded banks of
/// every active count at both widths, with the crate's audible fixture parameters and the
/// descriptor defaults. Member `k` reads the fixture from frame `29k` on, at its own level, so the
/// lanes of a bank differ. The fixture is played four times over in blocks of 128, 64, 37 and 1.
///
/// Red for the same defects as the random test; this one pins them on the shared signals.
#[test]
fn padded_banks_render_the_fixtures_per_node() {
    for (index, bytes) in FIXTURES.iter().enumerate() {
        let fixture = conformance::PcmFixture::parse(bytes, conformance::FixtureLimits::default())
            .expect("a checked-in fixture");
        let frames = fixture.frames() as usize;
        let channels = usize::from(fixture.channels());
        let samples = fixture.samples();
        let plane = |channel: usize, member: usize, frame: usize| {
            let channel = channel.min(channels - 1);
            let level = [1.0_f32, 0.25, 0.03, 2.0][member % 4];
            samples[channel * frames + (frame + 29 * member) % frames] * level
        };
        for &width in WIDTHS {
            let lanes = width.lanes() as usize;
            for members in 1..lanes {
                let values: Vec<Values> = (0..members)
                    .map(|member| {
                        if member % 3 == 2 {
                            default_values()
                        } else {
                            fixture_values(member)
                        }
                    })
                    .collect();
                let mut blocks = Vec::new();
                let mut played = 0;
                for size in [128, 64, 37, 1].into_iter().cycle() {
                    if played >= 4 * frames {
                        break;
                    }
                    let length = size.min(4 * frames - played);
                    blocks.push(Block {
                        frames: length,
                        input: (0..members)
                            .map(|member| {
                                [0, 1].map(|channel| {
                                    (played..played + length)
                                        .map(|frame| plane(channel, member, frame))
                                        .collect()
                                })
                            })
                            .collect(),
                        spans: vec![Vec::new(); members],
                    });
                    played += length;
                }
                let mask = prefix_mask(width, members);
                let link = LINKS[(index + members) % LINKS.len()];
                render(
                    &Padded {
                        width,
                        mask: &mask,
                        members: &values,
                        clone_of: 0,
                        rate: fixture.rate().0,
                        link,
                    },
                    &blocks,
                    &format!("fixture {index} {width:?} {members} members {link:?}"),
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 2: no lane is prepared outside its declared domain
// ---------------------------------------------------------------------------------------------

/// Gate 2: every lane of a padded bank, padded or active, is prepared inside the declared domains.
///
/// Each lane's state payload carries its three times and the current and target of its four
/// smoothed parameters; each must lie inside the descriptor's `[minimum, maximum]`. The payload
/// must also restore into a fresh scalar instance, whose restore refuses any time or parameter
/// outside its domain. Red if a padded lane is filled with zeros instead of its clone -- a zero
/// ratio, attack or release is outside the domain -- which is the defect this slice removed.
#[test]
fn no_lane_is_prepared_outside_its_declared_domain() {
    for &width in WIDTHS {
        let lanes = width.lanes() as usize;
        for members in 1..=lanes {
            let values: Vec<Values> = (0..members).map(fixture_values).collect();
            let mask = prefix_mask(width, members);
            let bank = Padded {
                width,
                mask: &mask,
                members: &values,
                clone_of: members - 1,
                rate: 48_000,
                link: LinkMode::DualMono,
            }
            .bind();
            let sizes = scalar(request(&values[0], 48_000, LinkMode::DualMono))
                .metadata()
                .state_sizes;
            for lane in 0..lanes {
                let (common, left, right) = bank_payload(bank.as_ref(), lane, sizes);
                for (channel, section) in [(0, &left), (1, &right)] {
                    let word = |index: usize| payload::read_f32(section, index);
                    let mut checked = vec![(4, word(3)), (5, word(4)), (6, word(5))];
                    for parameter in 0..RAMP_COUNT {
                        checked.push((parameter, word(STATE_RAMP_WORD + parameter * 4)));
                        checked.push((parameter, word(STATE_RAMP_WORD + parameter * 4 + 1)));
                    }
                    for (parameter, value) in checked {
                        let descriptor = &GATE_EXPANDER_PARAMETERS[parameter];
                        let (minimum, maximum) = (
                            descriptor.minimum.expect("bounded"),
                            descriptor.maximum.expect("bounded"),
                        );
                        assert!(
                            (minimum..=maximum).contains(&value),
                            "{width:?} {members} members: lane {lane} channel {channel} prepared \
                             {} = {value}, outside [{minimum}, {maximum}]",
                            descriptor.display_name
                        );
                    }
                }
                let mut fresh = scalar(request(&values[0], 48_000, LinkMode::DualMono));
                fresh
                    .restore_state_payload(
                        STATE_LAYOUT_VERSION,
                        StatePayloadInput::new(&common, &left, &right, sizes).expect("sizes"),
                    )
                    .unwrap_or_else(|error| {
                        panic!(
                            "{width:?} {members} members: lane {lane}'s prepared state is refused \
                             by a scalar restore: {}",
                            error.code
                        )
                    });
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 3: the clone source does not reach an active lane
// ---------------------------------------------------------------------------------------------

/// Gate 3: an active lane's bits are the same whichever member the padded lanes clone.
///
/// Every render also matches the members' scalar instances and checks each padded lane (gate 1's
/// checks), and the renders of one mask must agree word for word across every clone source,
/// including a member at every parameter's minimum and one at every maximum. Red if a factory reads
/// its members from anywhere but lanes `0..members` -- from the tail of the requests, say, as if
/// padding came first -- so that an active lane runs a clone; or if a whole-bank decision reads a
/// padded lane's parameters.
#[test]
fn active_lanes_do_not_depend_on_the_clone_source() {
    for &width in WIDTHS {
        let lanes = width.lanes() as usize;
        let masks: Vec<Vec<bool>> = [1, 2, lanes / 2 + 1, lanes - 1]
            .into_iter()
            .map(|members| prefix_mask(width, members))
            .collect();
        for (index, mask) in masks.iter().enumerate() {
            let members = mask.iter().filter(|active| **active).count();
            let mut draw = Draw::new(0x1092_0300 + (lanes * 10 + index) as u64);
            // Members that differ as much as the domains allow: one at every minimum and one at
            // every maximum, then drawn ones.
            let mut values: Vec<Values> = (0..members).map(|_| draw.values()).collect();
            values[0] = values_from(GATE_EXPANDER_PARAMETERS.map(|p| {
                let minimum = p.minimum.expect("bounded");
                (minimum, minimum)
            }));
            if members > 1 {
                values[members - 1] = values_from(GATE_EXPANDER_PARAMETERS.map(|p| {
                    let maximum = p.maximum.expect("bounded");
                    (maximum, maximum)
                }));
            }
            let blocks = random_blocks(&mut draw, members, 8);
            let link = LINKS[index % LINKS.len()];
            let mut reference: Option<Vec<Vec<u32>>> = None;
            for clone_of in 0..members {
                let outputs = render(
                    &Padded {
                        width,
                        mask,
                        members: &values,
                        clone_of,
                        rate: 48_000,
                        link,
                    },
                    &blocks,
                    &format!("{width:?} mask {mask:?} clone of member {clone_of}"),
                );
                match &reference {
                    None => reference = Some(outputs),
                    Some(first) => assert_eq!(
                        &outputs, first,
                        "{width:?} mask {mask:?}: the active lanes moved when the padding cloned \
                         member {clone_of} instead of member 0"
                    ),
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Gate 4: D7 attributes active lanes only
// ---------------------------------------------------------------------------------------------

/// A gate bank of `L::WIDTH` lanes built the way `bind_bank` builds one, for fault injection:
/// active lane `l` runs `values[l]`, and a padded lane clones the first active lane's values.
fn injectable<L: Lane>(
    width: BankWidth,
    mask: &[bool],
    values: &[Values],
) -> PreparedGate<L, false> {
    let first = mask.iter().position(|active| *active).expect("a member");
    let mut defaults = [initial_defaults(&values[first]).expect("valid"); MAX_WIDTH];
    for (lane, active) in mask.iter().enumerate() {
        let source = if *active { lane } else { first };
        defaults[lane] = initial_defaults(&values[source]).expect("valid");
    }
    let metadata = expected_prepared_metadata(
        &GATE_EXPANDER_DESCRIPTOR,
        request(&values[0], 48_000, LinkMode::DualMono),
    )
    .expect("metadata");
    PreparedGate::<L, false>::new(metadata, Some(width), defaults)
        .expect("a bank")
        .with_active_lanes(active_bits(mask))
}

/// Renders one block of `frames` through a bank: active lanes carry lane-distinct noise, padded
/// lanes `+0.0`. Returns the output planes and the report.
fn one_block<L: Lane>(
    bank: &mut PreparedGate<L, false>,
    width: BankWidth,
    mask: &[bool],
    block: usize,
    hostile: Option<(usize, f32)>,
) -> (Vec<f32>, Vec<f32>, BankProcessReport)
where
    PreparedGate<L, false>: PreparedNativeEffectBank,
{
    const FRAMES: usize = 64;
    let lanes = L::WIDTH;
    let mut left = vec![0.0_f32; FRAMES * lanes];
    let mut right = vec![0.0_f32; FRAMES * lanes];
    for (lane, active) in mask.iter().enumerate() {
        if !active {
            continue;
        }
        let mut draw = Draw::new((block * 64 + lane) as u64);
        let signal = [draw.signal(FRAMES), draw.signal(FRAMES)];
        for frame in 0..FRAMES {
            left[frame * lanes + lane] = signal[0][frame];
            right[frame * lanes + lane] = signal[1][frame];
        }
    }
    if let Some((lane, word)) = hostile {
        for frame in (0..FRAMES).step_by(7) {
            left[frame * lanes + lane] = word;
        }
    }
    let offsets = vec![0_u32; lanes + 1];
    let report = bank.process_bank(
        EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            FRAMES as u32,
            width,
            block as u64 * FRAMES as u64,
            &[],
            &offsets,
            QUANTUM,
        )
        .expect("a well-shaped bank block"),
    );
    (left, right, report)
}

/// Gate 4 for the gate/expander, at one width: a NaN planted in one lane's gain state.
///
/// * Planted in an active lane: that lane's left channel is zeroed and reset, it alone is reported,
///   and every other lane -- its right channel included -- keeps the uninjected control's bits.
/// * Planted in a padded lane: it is recovered (its gain is back at rest), nothing is reported,
///   and every active lane keeps the control's bits.
fn planted_nonfinite_state<L: Lane>(width: BankWidth)
where
    PreparedGate<L, false>: PreparedNativeEffectBank,
{
    let lanes = L::WIDTH;
    // Members first: lanes `0..lanes - 2` carry members and the last two are padded.
    let mask = prefix_mask(width, lanes - 2);
    let (active_lane, padded_lane) = (lanes - 3, lanes - 1);
    let values: Vec<Values> = (0..lanes).map(fixture_values).collect();
    for (target, is_padded) in [(active_lane, false), (padded_lane, true)] {
        let mut bank = injectable::<L>(width, &mask, &values);
        let mut control = injectable::<L>(width, &mask, &values);
        for block in 0..4 {
            if block == 3 {
                bank.inject_nonfinite_gain(target, 0);
            }
            let (left, right, report) = one_block(&mut bank, width, &mask, block, None);
            let (control_left, control_right, control_report) =
                one_block(&mut control, width, &mask, block, None);
            let frames = left.len() / lanes;
            for (lane, active) in mask.iter().enumerate() {
                let hit = block == 3 && lane == target;
                let expected = if hit && !is_padded {
                    ProcessReport {
                        nonfinite_left_blocks: frames as u64,
                        ..ProcessReport::default()
                    }
                } else {
                    control_report.reports[lane]
                };
                assert_eq!(
                    report.reports[lane], expected,
                    "{width:?}: planted in lane {target} (padded {is_padded}), block {block}: \
                     lane {lane}'s report"
                );
                if !active {
                    assert_eq!(report.reports[lane], ProcessReport::default());
                }
                for frame in 0..frames {
                    let at = frame * lanes + lane;
                    let expected_left = if hit { 0 } else { control_left[at].to_bits() };
                    assert_eq!(
                        left[at].to_bits(),
                        expected_left,
                        "{width:?}: planted in lane {target}, block {block}: lane {lane} left \
                         frame {frame}"
                    );
                    assert_eq!(
                        right[at].to_bits(),
                        control_right[at].to_bits(),
                        "{width:?}: planted in lane {target}, block {block}: lane {lane} right \
                         frame {frame}"
                    );
                }
            }
        }
        assert_eq!(
            lane_get(bank.state[0].gain_db, target).to_bits(),
            0,
            "{width:?}: lane {target} is recovered to rest"
        );
    }
}

#[test]
fn a_planted_nonfinite_state_recovers_and_reports_its_lane_alone() {
    lane::each_vector_lane!(|L, N| planted_nonfinite_state::<L>(
        BankWidth::for_lanes(N).expect("a bank width")
    ));
}

/// Gate 4's bypassed lane: a lane fed a tripping value leaves every bank-mate's bits.
///
/// At the effect a session-bypassed track is an active lane prepared with `bypass = false` whose
/// output the rack's `BypassShunt` later replaces with its dry block (#1087), so its wet path sees
/// what an enabled lane would. Here one active lane is fed `1e30`, an infinity or a NaN on every
/// seventh frame of every other block, and every other lane must keep the bits of a control render
/// in which that lane carries ordinary input; padded lanes stay `+0.0`. The tripping lane runs a
/// zero range, so the gate is the identity on it and `1e30` reaches its output; it is recovered
/// and reported on every hostile block.
///
/// Red if D7 recovery zeroes or resets the whole bank (the coupling P1's verdict M2 measured), or
/// reports a lane that did not trip.
#[test]
fn a_tripping_lane_leaves_every_bank_mates_bits() {
    fn at<L: Lane>(width: BankWidth)
    where
        PreparedGate<L, false>: PreparedNativeEffectBank,
    {
        let lanes = L::WIDTH;
        for members in [2, lanes - 1, lanes] {
            let mask = prefix_mask(width, members);
            for (index, word) in TRIPPING_WORDS.into_iter().enumerate() {
                let hot = index % members;
                let mut values: Vec<Values> = (0..lanes).map(fixture_values).collect();
                values[hot][4].value = 0.0;
                values[hot][5].value = 0.0;
                let mut bank = injectable::<L>(width, &mask, &values);
                let mut control = injectable::<L>(width, &mask, &values);
                for block in 0..6 {
                    let hostile = (block % 2 == 1).then_some((hot, word));
                    let (left, right, report) = one_block(&mut bank, width, &mask, block, hostile);
                    let (control_left, control_right, control_report) =
                        one_block(&mut control, width, &mask, block, None);
                    let frames = left.len() / lanes;
                    for lane in (0..lanes).filter(|lane| *lane != hot) {
                        assert_eq!(report.reports[lane], control_report.reports[lane]);
                        for frame in 0..frames {
                            let at = frame * lanes + lane;
                            assert_eq!(
                                (left[at].to_bits(), right[at].to_bits()),
                                (control_left[at].to_bits(), control_right[at].to_bits()),
                                "{width:?} {members} members: lane {hot} fed {word:e}, block \
                                 {block}: bank-mate {lane} frame {frame} moved"
                            );
                        }
                    }
                    if hostile.is_some() {
                        assert!(
                            (0..frames).all(|frame| left[frame * lanes + hot].to_bits() == 0),
                            "{width:?}: lane {hot} fed {word:e}: its left block is zeroed"
                        );
                        assert_eq!(
                            report.reports[hot].nonfinite_left_blocks, frames as u64,
                            "{width:?}: lane {hot} fed {word:e} tripped D7 and is reported"
                        );
                    }
                }
            }
        }
    }
    lane::each_vector_lane!(|L, N| at::<L>(BankWidth::for_lanes(N).expect("a bank width")));
}
