//! Issue #1091 (console strip P2d): a padded true-peak limiter bank renders its members' per-node
//! bits.
//!
//! A padded bank binds `members < W` tracks. Every other lane carries a clone of a member's request,
//! is fed `+0.0` and has its output discarded (`effect_contract::PrepareEffectBankRequest`). This
//! file drives the production factory only, through the contract calls a host makes:
//!
//! * **Gate 1.** For every active count `1..W`, the members of a padded bank render the bits the
//!   same tracks render prepared per node, compared from the first block, while the `N + 6` line is
//!   still filling. Random parameters, input and automation under both link modes, and the console
//!   fixture's limiter shape under `maximum`, dual and collapsed, with a silence long enough for the
//!   silent fast path to engage. The members sit on lanes `0..members`, the contract's one layout.
//! * **Gate 3, the coupling rule.** The members' bits do not depend on what the padded lanes carry:
//!   a clone of any member, a mix of clones, or lanes that are no clone at all and move the bank's
//!   whole-bank decisions (the uniform-body gate, the linked-pair record).
//! * **Gate 4, the public half.** A member fed a non-finite sample fails alone: its bank-mates keep
//!   their bits, and it recovers to what its per-node twin renders after its own reset. The report
//!   half, which needs the crate's internals, is `a_failed_lane_is_recovered_and_reported_alone`.
//!
//! Every bank width this build binds is run: `Simd8` and `Simd4` on x86-64-v3, `Simd4` on AArch64,
//! where `scripts/run-aarch64-tests.sh` runs this file natively. A padded lane is fed what the bank
//! left in it, as `rack::BankChain` feeds it, so every block also checks that the limiter answers
//! `+0.0` with exactly `+0.0` there (gate 3's P2a clause; its state half, which needs the crate's
//! internals, is `a_padded_lane_stays_at_rest_through_the_lookahead`).

use dsp_reference::class_a;
use effect_contract::{
    AutomationSpanKind, BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality,
    InitialParameterValue, LinkMode, NativeEffectFactory, ParameterChannel,
    PrepareEffectBankRequest, PrepareEffectLimits, PrepareEffectRequest, PreparedAutomationSpan,
    PreparedPorts, PreparedSidechainPort,
};
use lane::Backend;
use true_peak_limiter::{
    TRUE_PEAK_LIMITER_DESCRIPTOR, TRUE_PEAK_LIMITER_PARAMETERS, TruePeakLimiterFactory,
};

const FRAMES: usize = 128;

/// One track's six initial values: `[ceiling dB, release ms, lookahead ms]` per channel.
type Values = [InitialParameterValue; 6];

/// Per member, the left and right output of a whole run.
type Planes = Vec<(Vec<f32>, Vec<f32>)>;

/// Deterministic SplitMix64, so a scenario is a seed.
struct Draw(u64);

impl Draw {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut mixed = self.0;
        mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        mixed ^ (mixed >> 31)
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }

    fn chance(&mut self, numerator: usize, denominator: usize) -> bool {
        self.below(denominator) < numerator
    }

    /// Uniform in `[0, 1)`, on the 2^-24 grid.
    fn fraction(&mut self) -> f32 {
        (self.next_u64() >> 40) as f32 * (1.0 / 16_777_216.0)
    }

    /// Uniform in `[-1, 1)`.
    fn unit(&mut self) -> f32 {
        self.fraction() * 2.0 - 1.0
    }
}

fn values(left: [f32; 3], right: [f32; 3]) -> Values {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: if index % 2 == 0 {
            left[index / 2]
        } else {
            right[index / 2]
        },
    })
}

fn descriptor_defaults() -> Values {
    let row: [f32; 3] =
        core::array::from_fn(|parameter| TRUE_PEAK_LIMITER_PARAMETERS[parameter].default_value);
    values(row, row)
}

/// The console fixture's limiter on track `track`, `channel: "both"`
/// (`fixtures/session/v1/console-sixty-four-track-*.json`).
fn fixture_track(track: usize) -> Values {
    let row = [
        -0.5 - 0.031_25 * track as f32,
        60.0 + 1.25 * track as f32,
        5.0,
    ];
    values(row, row)
}

fn request(values: &Values, link: LinkMode) -> PrepareEffectRequest<'_> {
    let quality = TRUE_PEAK_LIMITER_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("launch rate");
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: FRAMES as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: link,
        ports: PreparedPorts {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 24,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// The bank widths this build binds, each with the backend it validates against. A bank wider
/// than the backend declines, so the loop runs every width that binds instead of returning early
/// on the other one.
fn bank_widths() -> Vec<(BankWidth, Backend)> {
    let widths: Vec<(BankWidth, Backend)> = [
        (BankWidth::Four, Backend::Simd4),
        (BankWidth::Eight, Backend::Simd8),
    ]
    .into_iter()
    .filter(|(width, _)| width.lanes() as usize <= Backend::current().width())
    .collect();
    let native = BankWidth::for_backend(Backend::current()).expect("a product target");
    assert!(
        widths.iter().any(|(width, _)| *width == native),
        "the native width binds"
    );
    widths
}

/// What the members of one run carry, block by block.
struct Scenario {
    label: String,
    link: LinkMode,
    /// Collapsed: the bank runs `process_bank_mono` and both channels of every member carry one
    /// signal. The per-node oracle stays dual.
    mono: bool,
    members: Vec<Values>,
    /// Per member, `blocks * FRAMES` samples of each channel.
    left: Vec<Vec<f32>>,
    right: Vec<Vec<f32>>,
    /// Per block, per member.
    spans: Vec<Vec<Vec<PreparedAutomationSpan>>>,
    blocks: usize,
}

fn point(
    first: u64,
    parameter: u32,
    channel: ParameterChannel,
    value: f32,
) -> PreparedAutomationSpan {
    PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index: parameter,
        start_sample: first,
        end_sample: first,
        start_value: value,
        end_value: value,
    }
}

/// A ceiling anywhere in `[-24, 0]` dB, never `-0.0` (which the limiter refuses).
fn ceiling(draw: &mut Draw) -> f32 {
    0.0 - 24.0 * draw.fraction()
}

/// A release anywhere in `[10, 2000)` ms, skewed toward the short end where it is audible.
fn release(draw: &mut Draw) -> f32 {
    let fraction = draw.fraction();
    10.0 + 1990.0 * fraction * fraction * fraction
}

/// `members` random tracks over 16 blocks: parameters anywhere in their domains (lookahead split
/// between the channels half the time), and each block of each member one of loud noise that
/// limits, quiet noise, `+0.0` silence, `-0.0`, subnormals or a square at the ceiling's scale, with
/// point automation on a quarter of them.
fn random_scenario(seed: u64, link: LinkMode, members: usize) -> Scenario {
    const BLOCKS: usize = 16;
    let mut draw = Draw(seed);
    let row = |draw: &mut Draw| {
        [
            ceiling(draw),
            release(draw),
            [0.0, 0.25, 1.0, 2.5, 5.0, 7.5, 10.0][draw.below(7)],
        ]
    };
    let tracks: Vec<Values> = (0..members)
        .map(|_| {
            let left = row(&mut draw);
            let right = if draw.chance(1, 2) {
                left
            } else {
                row(&mut draw)
            };
            values(left, right)
        })
        .collect();
    let signal = |draw: &mut Draw| -> Vec<f32> {
        let kind = draw.below(6);
        (0..FRAMES)
            .map(|frame| match kind {
                0 => draw.unit() * 3.0,
                1 => draw.unit() * 0.2,
                2 => 0.0,
                3 => -0.0,
                4 => f32::from_bits((draw.next_u64() as u32) & 0x807F_FFFF),
                _ => {
                    if (frame / 7) % 2 == 0 {
                        0.9
                    } else {
                        -0.9
                    }
                }
            })
            .collect()
    };
    let mut left = vec![Vec::new(); members];
    let mut right = vec![Vec::new(); members];
    let mut spans = Vec::with_capacity(BLOCKS);
    for block in 0..BLOCKS {
        let first = (block * FRAMES) as u64;
        let mut per_member = Vec::with_capacity(members);
        for member in 0..members {
            left[member].extend(signal(&mut draw));
            right[member].extend(signal(&mut draw));
            let mut lane_spans = Vec::new();
            if draw.chance(1, 4) {
                for (parameter, channel) in [
                    (0, ParameterChannel::Left),
                    (0, ParameterChannel::Right),
                    (1, ParameterChannel::Left),
                    (1, ParameterChannel::Right),
                ] {
                    if draw.chance(1, 2) {
                        let value = if parameter == 0 {
                            ceiling(&mut draw)
                        } else {
                            release(&mut draw)
                        };
                        lane_spans.push(point(first, parameter, channel, value));
                    }
                }
            }
            per_member.push(lane_spans);
        }
        spans.push(per_member);
    }
    Scenario {
        label: format!("random {seed:#x} {link:?}"),
        link,
        mono: false,
        members: tracks,
        left,
        right,
        spans,
        blocks: BLOCKS,
    }
}

/// The console fixture's limiter, `link_mode: maximum`, on tracks `first..first + members`: quiet
/// material below every ceiling, then silence long enough for the silent fast path to engage, then
/// material that limits. Collapsed when `mono`.
fn fixture_scenario(first: usize, members: usize, mono: bool) -> Scenario {
    const BLOCKS: usize = 22;
    let mut draw = Draw(0x1091_0001 ^ first as u64);
    let level = |block: usize| match block {
        0..=5 => 0.3,
        6..=11 => 0.0,
        _ => 1.4,
    };
    let mut left = vec![Vec::new(); members];
    let mut right = vec![Vec::new(); members];
    for block in 0..BLOCKS {
        for member in 0..members {
            for _ in 0..FRAMES {
                // Silence is `+0.0`: `unit * 0.0` would be `-0.0` for half the draws, which the
                // silent fast path rightly refuses.
                let mut draw_sample = || match level(block) {
                    0.0 => 0.0,
                    level => draw.unit() * level,
                };
                let sample = draw_sample();
                left[member].push(sample);
                right[member].push(if mono { sample } else { draw_sample() });
            }
        }
    }
    Scenario {
        label: format!("fixture tracks {first}.. mono {mono}"),
        link: LinkMode::Maximum,
        mono,
        members: (first..first + members).map(fixture_track).collect(),
        left,
        right,
        spans: vec![vec![Vec::new(); members]; BLOCKS],
        blocks: BLOCKS,
    }
}

/// The oracle: each member prepared on its own and rendered block by block.
fn per_node(scenario: &Scenario) -> Planes {
    scenario
        .members
        .iter()
        .enumerate()
        .map(|(member, values)| {
            let mut effect = TruePeakLimiterFactory
                .prepare(request(values, scenario.link))
                .expect("prepare");
            let mut left = scenario.left[member].clone();
            let mut right = scenario.right[member].clone();
            for block in 0..scenario.blocks {
                let range = block * FRAMES..(block + 1) * FRAMES;
                effect.process(
                    EffectProcessBlock::new(
                        &mut left[range.clone()],
                        &mut right[range],
                        None,
                        (block * FRAMES) as u64,
                        &scenario.spans[block][member],
                        FRAMES as u32,
                    )
                    .expect("block"),
                );
            }
            (left, right)
        })
        .collect()
}

/// The bank arm. `layout[lane]` is the member on that lane, or `None` for a padded lane carrying
/// `padding[lane]`. The planes are the bank's resident block: `+0.0` at bind, then whatever the
/// bank left in a padded lane, which must stay `+0.0`.
fn banked(
    scenario: &Scenario,
    width: BankWidth,
    backend: Backend,
    layout: &[Option<usize>],
    padding: &[Values],
) -> Planes {
    let lanes = width.lanes() as usize;
    assert_eq!(layout.len(), lanes);
    let requests: Vec<PrepareEffectRequest<'_>> = layout
        .iter()
        .zip(padding)
        .map(|(member, padding)| {
            let values = member.map_or(padding, |member| &scenario.members[member]);
            request(values, scenario.link)
        })
        .collect();
    let mask: Vec<bool> = layout.iter().map(Option::is_some).collect();
    let mut bank = TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: &mask,
        })
        .expect("a padded request is well formed")
        .expect("a padded limiter bank binds");
    let mut left = vec![0.0_f32; FRAMES * lanes];
    let mut right = vec![0.0_f32; FRAMES * lanes];
    let mut output: Planes = scenario
        .members
        .iter()
        .map(|_| (Vec::new(), Vec::new()))
        .collect();
    for block in 0..scenario.blocks {
        let range = block * FRAMES..(block + 1) * FRAMES;
        let mut spans = Vec::new();
        let mut offsets = vec![0_u32];
        for (lane, member) in layout.iter().enumerate() {
            if let Some(member) = *member {
                for (frame, (l, r)) in scenario.left[member][range.clone()]
                    .iter()
                    .zip(&scenario.right[member][range.clone()])
                    .enumerate()
                {
                    left[frame * lanes + lane] = *l;
                    right[frame * lanes + lane] = *r;
                }
                spans.extend_from_slice(&scenario.spans[block][member]);
            }
            offsets.push(spans.len() as u32);
        }
        let process = EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            FRAMES as u32,
            width,
            (block * FRAMES) as u64,
            &spans,
            &offsets,
            FRAMES as u32,
        )
        .expect("bank block");
        if scenario.mono {
            bank.process_bank_mono(process);
        } else {
            bank.process_bank(process);
        }
        for (lane, member) in layout.iter().enumerate() {
            let words = |plane: &[f32]| -> Vec<f32> {
                (0..FRAMES)
                    .map(|frame| plane[frame * lanes + lane])
                    .collect()
            };
            match *member {
                Some(member) => {
                    output[member].0.extend(words(&left));
                    output[member].1.extend(words(&right));
                }
                None => {
                    for (name, plane) in [("left", &left), ("right", &right)] {
                        assert!(
                            words(plane).iter().all(|word| word.to_bits() == 0),
                            "{} {width:?}: padded lane {lane} {name} is not +0.0 after block \
                             {block}",
                            scenario.label
                        );
                    }
                }
            }
        }
    }
    output
}

/// Class-A identity of every member's output, NaN folded (decision 10). A collapsed bank leaves
/// its right plane unprocessed, so only the left plane is compared there.
fn assert_members_match(shipped: &Planes, oracle: &Planes, mono: bool, what: &str) {
    for (member, (shipped, oracle)) in shipped.iter().zip(oracle).enumerate() {
        let channels = if mono {
            vec![("left", &shipped.0, &oracle.0)]
        } else {
            vec![
                ("left", &shipped.0, &oracle.0),
                ("right", &shipped.1, &oracle.1),
            ]
        };
        for (name, shipped, oracle) in channels {
            assert_eq!(
                shipped.len(),
                oracle.len(),
                "{what}: member {member} {name} length"
            );
            if let Some(index) = shipped
                .iter()
                .zip(oracle.iter())
                .position(|(a, b)| !class_a::same(*a, *b))
            {
                panic!(
                    "{what}: member {member} {name} sample {index} (block {}): banked {:#010x}, \
                     per node {:#010x}",
                    index / FRAMES,
                    shipped[index].to_bits(),
                    oracle[index].to_bits()
                );
            }
        }
    }
}

/// The contract's one layout: members on lanes `0..members`, padded lanes after them
/// (`validate_shape` refuses any other, #1088 verdict L3).
fn members_first(lanes: usize, members: usize) -> Vec<Option<usize>> {
    (0..lanes)
        .map(|lane| (lane < members).then_some(lane))
        .collect()
}

/// Every padded lane a clone of `member`.
fn clones_of(scenario: &Scenario, lanes: usize, member: usize) -> Vec<Values> {
    vec![scenario.members[member]; lanes]
}

/// **Gate 1: a padded bank's members render their per-node bits**, for every active count `1..W`
/// at every width this build binds.
///
/// Red if a member's bits move with its bank, from the first block on: a binding that seeds a
/// member from the wrong request (a clone's), a padded lane fed back into a member, a whole-bank
/// decision that is not bit-neutral per lane, or a padded lane that stops answering `+0.0` with
/// `+0.0` (the assertion inside [`banked`]).
#[test]
fn a_padded_bank_renders_its_members_per_node_bits() {
    for (width, backend) in bank_widths() {
        let lanes = width.lanes() as usize;
        for members in 1..lanes {
            let mut scenarios = vec![
                random_scenario(0x1091_0100 + members as u64, LinkMode::DualMono, members),
                random_scenario(0x1091_0200 + members as u64, LinkMode::Maximum, members),
                fixture_scenario(64 - members, members, false),
                fixture_scenario(64 - members, members, true),
            ];
            for scenario in &mut scenarios {
                scenario.label = format!("{} {width:?} {members}/{lanes}", scenario.label);
            }
            for scenario in &scenarios {
                let oracle = per_node(scenario);
                let padding = clones_of(scenario, lanes, 0);
                let layout = members_first(lanes, members);
                let shipped = banked(scenario, width, backend, &layout, &padding);
                assert_members_match(&shipped, &oracle, scenario.mono, &scenario.label);
            }
        }
    }
}

/// **Gate 3, the coupling rule: the members' bits do not depend on the padded lanes.**
///
/// Each padding below renders the bits of the planner's (a clone of the first member), which are
/// the per-node bits: a clone of every other member, a different clone on each padded lane, the
/// descriptor defaults, and an asymmetric lane that is no clone at all (a split 0/10 ms lookahead
/// and far-apart ceilings). The last moves the fixture's bank off the uniform body and off the
/// linked-pair body. The last two are outside the contract; they are here because a whole-bank
/// decision they move must still be bit-neutral per lane.
///
/// Red if an active lane reads a padded lane's words: a binding that seeds members from the wrong
/// request, a uniform body that trusts a lane the gate did not check, or a linked record that
/// covers a lane it should not.
#[test]
fn the_members_bits_do_not_depend_on_the_padded_lanes() {
    let asymmetric = values([-24.0, 10.0, 0.0], [-3.0, 2000.0, 10.0]);
    for (width, backend) in bank_widths() {
        let lanes = width.lanes() as usize;
        for members in 1..lanes {
            let scenarios = [
                random_scenario(0x1091_0300 + members as u64, LinkMode::Maximum, members),
                fixture_scenario(members, members, false),
                fixture_scenario(members, members, true),
            ];
            for scenario in &scenarios {
                let label = format!("{} {width:?} {members}/{lanes}", scenario.label);
                let oracle = per_node(scenario);
                let layout = members_first(lanes, members);
                let reference = banked(
                    scenario,
                    width,
                    backend,
                    &layout,
                    &clones_of(scenario, lanes, 0),
                );
                assert_members_match(&reference, &oracle, scenario.mono, &label);
                let mut paddings: Vec<(String, Vec<Values>)> = (1..members)
                    .map(|member| {
                        (
                            format!("clones of member {member}"),
                            clones_of(scenario, lanes, member),
                        )
                    })
                    .collect();
                paddings.push((
                    "a clone per padded lane".into(),
                    (0..lanes)
                        .map(|lane| scenario.members[lane % members])
                        .collect(),
                ));
                paddings.push((
                    "descriptor defaults".into(),
                    vec![descriptor_defaults(); lanes],
                ));
                paddings.push(("an asymmetric non-clone".into(), vec![asymmetric; lanes]));
                for (name, padding) in paddings {
                    let shipped = banked(scenario, width, backend, &layout, &padding);
                    assert_members_match(
                        &shipped,
                        &reference,
                        scenario.mono,
                        &format!("{label}: {name} against clones of member 0"),
                    );
                }
            }
        }
    }
}

/// **Gate 4, the public half: a member fed a non-finite sample fails alone.**
///
/// One member's left input carries a NaN. It reaches the output `N + 6` samples later, where the
/// §4.4 check zeroes and resets that lane. Its bank-mates render their per-node bits throughout,
/// and the failed member renders what its twin renders after the twin's own (whole) reset, even
/// though the bank's shared cursors ran on. The twin's zeroed block proves the check fired.
///
/// Red on the old whole-bank recovery, which zeroes every bank-mate's block and resets them.
#[test]
fn a_member_fed_a_non_finite_sample_fails_alone() {
    for (width, backend) in bank_widths() {
        let lanes = width.lanes() as usize;
        for members in 2..=lanes {
            for mono in [false, true] {
                let mut scenario = fixture_scenario(0, members, mono);
                scenario.label = format!("{} {width:?} {members}/{lanes}", scenario.label);
                let failing = members - 1;
                // Block 12 is loud, so the failure lands mid-limiting.
                let poisoned = 12 * FRAMES + 17;
                scenario.left[failing][poisoned] = f32::NAN;
                if mono {
                    scenario.right[failing][poisoned] = f32::NAN;
                }
                let oracle = per_node(&scenario);
                let zeroed = (0..scenario.blocks).any(|block| {
                    oracle[failing].0[block * FRAMES..(block + 1) * FRAMES]
                        .iter()
                        .all(|word| word.to_bits() == 0)
                        && scenario.left[failing][block * FRAMES..(block + 1) * FRAMES]
                            .iter()
                            .any(|word| *word != 0.0)
                });
                assert!(
                    zeroed,
                    "{}: the twin's boundary check never fired",
                    scenario.label
                );
                let layout = members_first(lanes, members);
                let shipped = banked(
                    &scenario,
                    width,
                    backend,
                    &layout,
                    &clones_of(&scenario, lanes, 0),
                );
                assert_members_match(&shipped, &oracle, mono, &scenario.label);
            }
        }
    }
}
