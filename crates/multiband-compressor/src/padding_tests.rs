//! Issue #1893: the multiband compressor binds a padded bank (decision 12).
//!
//! A padded bank is `members < lanes` tracks bound as one bank. The padding contract
//! (`effect_contract::PrepareEffectBankRequest`) gives each padded lane a clone of an active
//! member's request and a `+0.0` feed, and discards its output. The effect owes the rest: a padded
//! lane never moves an active lane's bits, never reports, takes no automation and has no payload.
//!
//! Both tests bind through the public `bind_homogeneous_bank` at every width the build has
//! (`BankWidth::ALL`): the multiband has no D4 width gate, so the four-lane bank binds in the 8-lane
//! (AVX2) build too. NaNs fold to one word before any comparison (decision 10).

use super::*;
use dsp_reference::class_a;
use dsp_reference::randomized::{Draw, run_seeds};
use effect_contract::{
    EffectProcessBlock, PrepareEffectLimits, PreparedPorts, PreparedSidechainPort,
};

pub(super) const QUANTUM: u32 = 128;
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
pub(super) const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

pub(super) type Values = [InitialParameterValue; PARAMETER_COUNT * 2];
pub(super) type Payload = (Vec<u8>, Vec<u8>, Vec<u8>);

pub(super) fn request(values: &Values, rate: u32, link: LinkMode) -> PrepareEffectRequest<'_> {
    PrepareEffectRequest {
        sample_rate: rate,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: link,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: 32,
        },
        tail_bound: tail_bound(rate),
    }
}

/// One track's initial values, every parameter drawn in its domain.
pub(super) fn draw_values(draw: &mut Draw) -> Values {
    core::array::from_fn(|slot| {
        let descriptor = &MULTIBAND_COMPRESSOR_PARAMETERS[slot / 2];
        InitialParameterValue {
            parameter_index: (slot / 2) as u32,
            channel: if slot.is_multiple_of(2) {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: draw.in_domain(
                descriptor.minimum.expect("a bounded domain"),
                descriptor.maximum.expect("a bounded domain"),
            ),
        }
    })
}

/// One block's automation for one track: a point per parameter and channel, one time in four, in
/// the `parameter_index * 2 + channel` order the kernel takes.
pub(super) fn draw_spans(draw: &mut Draw, first_sample: u64) -> Vec<PreparedAutomationSpan> {
    let mut spans = Vec::new();
    for (index, descriptor) in MULTIBAND_COMPRESSOR_PARAMETERS.iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            if draw.chance(1, 4) {
                spans.push(PreparedAutomationSpan {
                    channel,
                    parameter_index: index as u32,
                    sample: first_sample,
                    value: draw.in_domain(
                        descriptor.minimum.expect("a bounded domain"),
                        descriptor.maximum.expect("a bounded domain"),
                    ),
                });
            }
        }
    }
    spans
}

/// Binds `requests` under `mask` through the public factory.
pub(super) fn bind(
    width: BankWidth,
    requests: &[PrepareEffectRequest<'_>],
    mask: &[bool],
) -> Box<dyn PreparedNativeEffectBank> {
    MultibandCompressorFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: width.backend(),
            width,
            requests,
            active_mask: mask,
        })
        .expect("a padded request is well formed")
        .expect("the multiband binds a padded bank")
        .processor
}

/// The mask with lanes `0..members` active: the planner's members-first layout.
pub(super) fn prefix_mask(width: BankWidth, members: usize) -> Vec<bool> {
    (0..width.lanes() as usize)
        .map(|lane| lane < members)
        .collect()
}

fn sections(sizes: StatePayloadSizes) -> Payload {
    (
        vec![0; sizes.common_bytes as usize],
        vec![0; sizes.left_bytes as usize],
        vec![0; sizes.right_bytes as usize],
    )
}

pub(super) fn bank_payload(
    bank: &dyn PreparedNativeEffectBank,
    lane: usize,
    sizes: StatePayloadSizes,
) -> Result<Payload, StatePayloadError> {
    let (mut common, mut left, mut right) = sections(sizes);
    bank.snapshot_track_state_payload(
        lane as u32,
        StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
    )?;
    Ok((common, left, right))
}

pub(super) fn payload_of(effect: &dyn PreparedNativeEffect, sizes: StatePayloadSizes) -> Payload {
    let (mut common, mut left, mut right) = sections(sizes);
    effect
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
        )
        .expect("a scalar instance snapshots");
    (common, left, right)
}

/// One block: its length, and per member its two input planes and its spans.
struct Block {
    frames: usize,
    input: Vec<[Vec<f32>; 2]>,
    spans: Vec<Vec<PreparedAutomationSpan>>,
}

/// `count` blocks of drawn lengths, input profiles and automation for `members` tracks.
fn random_blocks(draw: &mut Draw, members: usize, count: usize) -> Vec<Block> {
    let mut first_sample = 0;
    (0..count)
        .map(|_| {
            let frames = draw.frames(QUANTUM as usize);
            let plane = |draw: &mut Draw| {
                let profile = draw.profile();
                (0..frames).map(|_| draw.sample(profile)).collect()
            };
            let block = Block {
                frames,
                input: (0..members)
                    .map(|_| [plane(&mut *draw), plane(&mut *draw)])
                    .collect(),
                spans: (0..members)
                    .map(|_| {
                        if draw.chance(1, 3) {
                            draw_spans(draw, first_sample)
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

/// Renders `blocks` through a padded bank (members first, every padded lane a clone of member
/// `clone_of`, member `k` at its own link mode `links[k]`, issue #1367) and through each member's
/// scalar instance at that mode, and checks every block: an active lane's
/// words and report are its scalar instance's, and a padded lane writes `+0.0` and reports nothing
/// although every other block hands it stray spans, one of them malformed. At the end each active
/// lane's payload is its scalar instance's, and a padded lane's payload is refused.
fn render(
    width: BankWidth,
    members: &[Values],
    clone_of: usize,
    rate: u32,
    links: &[LinkMode],
    blocks: &[Block],
    context: &str,
) {
    let lanes = width.lanes() as usize;
    let mask = prefix_mask(width, members.len());
    let requests: Vec<PrepareEffectRequest<'_>> = (0..lanes)
        .map(|lane| {
            let member = if lane < members.len() { lane } else { clone_of };
            request(&members[member], rate, links[member])
        })
        .collect();
    let mut bank = bind(width, &requests, &mask);
    let prepared: Vec<PreparedEffect> = members
        .iter()
        .zip(links)
        .map(|(values, link)| {
            MultibandCompressorFactory
                .prepare(request(values, rate, *link))
                .expect("a valid member prepares")
        })
        .collect();
    let sizes = prepared[0].metadata.state_sizes;
    let mut scalars: Vec<Box<dyn PreparedNativeEffect>> = prepared
        .into_iter()
        .map(|effect| effect.processor)
        .collect();
    let mut first_sample = 0_u64;
    for (index, block) in blocks.iter().enumerate() {
        let frames = block.frames;
        let mut left = vec![0.0_f32; frames * lanes];
        let mut right = vec![0.0_f32; frames * lanes];
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32];
        for lane in 0..lanes {
            if lane < members.len() {
                for frame in 0..frames {
                    left[frame * lanes + lane] = block.input[lane][0][frame];
                    right[frame * lanes + lane] = block.input[lane][1][frame];
                }
                spans.extend_from_slice(&block.spans[lane]);
            } else if index % 2 == 1 {
                spans.extend(
                    [ParameterChannel::Both, ParameterChannel::Left].map(|channel| {
                        PreparedAutomationSpan {
                            channel,
                            parameter_index: 1,
                            sample: first_sample,
                            value: -20.0,
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
                width,
                first_sample,
                &spans,
                &offsets,
                QUANTUM,
            )
            .expect("a well-shaped bank block"),
        );
        for lane in members.len()..lanes {
            for frame in 0..frames {
                let at = frame * lanes + lane;
                assert_eq!(
                    (left[at].to_bits(), right[at].to_bits()),
                    (0, 0),
                    "{context}: block {index}: padded lane {lane} fed +0.0 wrote another \
                     word at frame {frame}"
                );
            }
            assert_eq!(
                report.reports[lane],
                ProcessReport::default(),
                "{context}: block {index}: padded lane {lane} was reported"
            );
        }
        for (lane, scalar) in scalars.iter_mut().enumerate() {
            let [mut own_left, mut own_right] = block.input[lane].clone();
            let own = scalar.process(
                EffectProcessBlock::new(
                    &mut own_left,
                    &mut own_right,
                    None,
                    first_sample,
                    &block.spans[lane],
                    QUANTUM,
                )
                .expect("a well-shaped block"),
            );
            for frame in 0..frames {
                let at = frame * lanes + lane;
                let bank_words = (class_a::bits(left[at]), class_a::bits(right[at]));
                let own_words = (
                    class_a::bits(own_left[frame]),
                    class_a::bits(own_right[frame]),
                );
                assert_eq!(
                    bank_words, own_words,
                    "{context}: block {index}: lane {lane} frame {frame}: the padded bank \
                     rendered {bank_words:#010x?}, the track per node {own_words:#010x?}"
                );
            }
            assert_eq!(
                report.reports[lane], own,
                "{context}: block {index}: lane {lane} report"
            );
        }
        first_sample += frames as u64;
    }
    for (lane, scalar) in scalars.iter().enumerate() {
        assert_eq!(
            bank_payload(bank.as_ref(), lane, sizes).expect("an active lane snapshots"),
            payload_of(scalar.as_ref(), sizes),
            "{context}: lane {lane}: state after the render"
        );
    }
    for lane in members.len()..lanes {
        assert_eq!(
            bank_payload(bank.as_ref(), lane, sizes)
                .err()
                .map(|error| error.code),
            Some("effect.state.track"),
            "{context}: padded lane {lane} has no payload"
        );
    }
}

/// The conformance PCM fixtures.
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

/// Every active lane of a padded bank renders its track's per-node bits.
///
/// Random arm: per width, drawn seeds each bind `1..lanes` members at a drawn rate, each member at
/// its own drawn link mode (issue #1367: a bank mixes modes), with drawn parameters, input
/// profiles (hostile words included, so D7 trips on active lanes) and automation, the padded lanes
/// cloning a member drawn per seed. Fixture arm: each conformance PCM
/// fixture at its own rate, every padded member count at every width, member `k` reading the
/// fixture from frame `29k` on at its own level and at the link mode `k` places after member 0's
/// (so every bank of two or more members mixes modes), played twice over in blocks of 128, 37
/// and 1.
///
/// Red if a padded lane's state or a bank-wide decision (the silent fixed point, the rest claim)
/// moves an active lane's bit, if an active lane reads a clone's parameters or renders at another
/// lane's link mode (a mixed bank that links a DualMono lane, an arm taken from one lane's mode),
/// if a padded lane takes automation or reports, if a padded lane has a payload, or if the #1088
/// decline returns (the bank does not bind).
#[test]
fn every_padded_bank_renders_its_members_per_node_bits() {
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let mut mixed_banks = 0;
        let ran = run_seeds(
            "every_padded_bank_renders_its_members_per_node_bits",
            "cargo test -p multiband-compressor --lib every_padded_bank_renders",
            16,
            |seed| {
                let mut draw = Draw::new(0x1893_0000 + seed * 16 + lanes as u64);
                let members = 1 + draw.below(lanes - 1);
                let clone_of = draw.below(members);
                let rate = draw.pick(&RATES);
                let links: Vec<LinkMode> = (0..members).map(|_| draw.pick(&LINKS)).collect();
                mixed_banks += usize::from(links.iter().any(|link| *link != links[0]));
                let values: Vec<Values> = (0..members).map(|_| draw_values(&mut draw)).collect();
                let blocks = random_blocks(&mut draw, members, 8);
                render(
                    width,
                    &values,
                    clone_of,
                    rate,
                    &links,
                    &blocks,
                    &format!(
                        "{width:?} seed {seed}: {members} members at {links:?}, clone of {clone_of}"
                    ),
                );
            },
        );
        assert!(ran > 0, "the random arm ran no seed");
        assert!(
            dsp_reference::randomized::replaying() || mixed_banks > 0,
            "{width:?}: no drawn bank mixed link modes"
        );
        for (index, bytes) in FIXTURES.iter().enumerate() {
            let fixture =
                conformance::PcmFixture::parse(bytes, conformance::FixtureLimits::default())
                    .expect("a checked-in fixture");
            let frames = fixture.frames() as usize;
            let channels = usize::from(fixture.channels());
            let samples = fixture.samples();
            let plane = |channel: usize, member: usize, frame: usize| {
                let channel = channel.min(channels - 1);
                let level = [1.0_f32, 0.25, 0.03, 2.0][member % 4];
                samples[channel * frames + (frame + 29 * member) % frames] * level
            };
            for members in 1..lanes {
                let mut draw = Draw::new((index * 100 + members * 10 + lanes) as u64);
                let values: Vec<Values> = (0..members).map(|_| draw_values(&mut draw)).collect();
                let mut blocks = Vec::new();
                let mut played = 0;
                for size in [128, 37, 1].into_iter().cycle() {
                    if played >= 2 * frames {
                        break;
                    }
                    let length = size.min(2 * frames - played);
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
                render(
                    width,
                    &values,
                    (index + members) % members,
                    fixture.rate().0,
                    &(0..members)
                        .map(|member| LINKS[(index + members + member) % LINKS.len()])
                        .collect::<Vec<_>>(),
                    &blocks,
                    &format!("fixture {index} {width:?} {members} members"),
                );
            }
        }
    }
}

/// A non-finite word planted in a padded lane is recovered there and charged to nobody.
///
/// Lanes `0..lanes - 2` carry members. Block 2 puts a NaN on every seventh frame of one lane's left
/// input, which makes that lane's filter state non-finite. Planted in the last active lane (the
/// control that the plant trips), that lane alone is zeroed and charged one left block. Planted in
/// the last padded lane (against the caller's `+0.0` duty, to reach the case), that lane is zeroed,
/// nothing is charged, and the next block it writes `+0.0` again, so its state recovered. Either
/// way every other lane keeps the bits of an unplanted control bank. Each case runs twice: as dual
/// blocks, and as collapsed blocks (`process_bank_mono`, issue #1871) on members whose right values
/// equal their left ones and on identical planes, where a failed active lane counts on both channels.
///
/// Red if the D7 charge mask is missing on either body (the padded lane is charged), if a padded lane's failure is
/// not recovered (it writes NaN on), or if recovery reaches a bank-mate.
#[test]
fn a_planted_nonfinite_state_in_a_padded_lane_recovers_uncharged() {
    const FRAMES: usize = 64;
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let members = lanes - 2;
        let mask = prefix_mask(width, members);
        let mut draw = Draw::new(0x1893_0002 + lanes as u64);
        let drawn: Vec<Values> = (0..members).map(|_| draw_values(&mut draw)).collect();
        // The collapsed arm's members have each right value equal to its left one, so the bank's
        // channels agree and the collapsed block is the one the rack would run.
        let symmetric: Vec<Values> = drawn
            .iter()
            .map(|values| {
                let mut values = *values;
                for slot in (1..values.len()).step_by(2) {
                    values[slot].value = values[slot - 1].value;
                }
                values
            })
            .collect();
        for (target, padded, mono) in [
            (members - 1, false, false),
            (lanes - 1, true, false),
            (members - 1, false, true),
            (lanes - 1, true, true),
        ] {
            let values = if mono { &symmetric } else { &drawn };
            let requests: Vec<PrepareEffectRequest<'_>> = (0..lanes)
                .map(|lane| request(&values[lane.min(members - 1)], 48_000, LinkMode::DualMono))
                .collect();
            let context = format!("{width:?}: NaN in lane {target} (padded {padded}, mono {mono})");
            let mut bank = bind(width, &requests, &mask);
            let mut control = bind(width, &requests, &mask);
            assert!(
                !mono || bank.channels_agree(),
                "{context}: the channels agree"
            );
            let offsets = vec![0_u32; lanes + 1];
            for block in 0..4 {
                let mut input = Draw::new(block as u64);
                let mut left = vec![0.0_f32; FRAMES * lanes];
                let mut right = vec![0.0_f32; FRAMES * lanes];
                for frame in 0..FRAMES {
                    for lane in 0..members {
                        left[frame * lanes + lane] = input.noise(0.7);
                        right[frame * lanes + lane] = input.noise(0.7);
                    }
                }
                if mono {
                    right.clone_from(&left);
                }
                let (mut control_left, mut control_right) = (left.clone(), right.clone());
                let planted = block == 2;
                if planted {
                    for frame in (0..FRAMES).step_by(7) {
                        left[frame * lanes + target] = f32::NAN;
                    }
                }
                let run = |bank: &mut dyn PreparedNativeEffectBank,
                           left: &mut [f32],
                           right: &mut [f32]| {
                    let block = EffectBankProcessBlock::new(
                        left,
                        right,
                        None,
                        FRAMES as u32,
                        width,
                        (block * FRAMES) as u64,
                        &[],
                        &offsets,
                        QUANTUM,
                    )
                    .expect("a well-shaped bank block");
                    if mono {
                        bank.process_bank_mono(block)
                    } else {
                        bank.process_bank(block)
                    }
                };
                let report = run(bank.as_mut(), &mut left, &mut right);
                let mut expected = run(control.as_mut(), &mut control_left, &mut control_right);
                if planted && !padded {
                    // A collapsed failure counts on both channels: the right plane is the left.
                    expected.reports[target].nonfinite_left_blocks += 1;
                    expected.reports[target].nonfinite_right_blocks += u64::from(mono);
                }
                assert_eq!(report, expected, "{context}: block {block}: the reports");
                for lane in (0..lanes).filter(|lane| *lane != target) {
                    for frame in 0..FRAMES {
                        let at = frame * lanes + lane;
                        // A collapsed block never writes the right plane.
                        let right_of = |plane: &[f32]| if mono { 0 } else { plane[at].to_bits() };
                        assert_eq!(
                            (left[at].to_bits(), right_of(&right)),
                            (control_left[at].to_bits(), right_of(&control_right)),
                            "{context}: block {block}: bank-mate {lane} frame {frame} moved"
                        );
                    }
                }
                if planted || (padded && block > 2) {
                    assert!(
                        (0..FRAMES).all(|frame| left[frame * lanes + target].to_bits() == 0
                            && (mono || right[frame * lanes + target].to_bits() == 0)),
                        "{context}: block {block}: lane {target} is not +0.0 on both channels"
                    );
                }
            }
        }
    }
}

/// Issue #1367 D1: a padded lane takes the first member's link mode, whatever its request says, so
/// padding never forces the mixed arm: members at `Maximum` padded with a request at another mode
/// keep the uniform `LINK_MAXIMUM` arm, at every width.
///
/// Red if bind reads a padded lane's own mode (the `active_mask` guard dropped).
#[test]
fn padding_keeps_the_members_uniform_link_arm() {
    fn arm<L: Lane, const W: usize>(width: BankWidth) -> u8 {
        let values = draw_values(&mut Draw::new(0x1367));
        let mask = prefix_mask(width, W - 1);
        let mut requests = vec![request(&values, 48_000, LinkMode::Maximum); W];
        requests[W - 1] = request(&values, 48_000, LinkMode::DualMono);
        let request = PrepareEffectBankRequest {
            backend: width.backend(),
            width,
            requests: &requests,
            active_mask: &mask,
        };
        bind_instance::<L, W>(&MultibandCompressorFactory, request)
            .expect("a padded request is well formed")
            .expect("one program")
            .0
            .link_lanes
            .arm
    }
    for &width in BankWidth::ALL {
        let arm = effect_contract::match_bank_width!(width, |L, N| arm::<L, N>(width));
        assert_eq!(arm, LINK_MAXIMUM, "{width:?}");
    }
}

/// The registry's tail-bound entry at `rate`, computed once per launch rate: a registry admission
/// derives every row's bound, and per request it dominated this module's debug test time (#1972).
fn tail_bound(rate: u32) -> effect_contract::RegisteredTailBound {
    static BOUNDS: [std::sync::OnceLock<effect_contract::RegisteredTailBound>; 4] =
        [const { std::sync::OnceLock::new() }; 4];
    let index = [44_100, 48_000, 88_200, 96_000]
        .iter()
        .position(|launch| *launch == rate)
        .expect("a launch rate");
    *BOUNDS[index].get_or_init(|| {
        conformance::tail_bound_of(
            Box::new(MultibandCompressorFactory),
            rate,
            EffectQuality::Normal,
        )
    })
}
