//! Issue #1090 (console strip P2c): padded compressor banks, at both bank widths.
//!
//! These are unit tests rather than integration tests for two reasons:
//!
//! * **Both widths on every host.** D4 binds only the build's own width, so an x86 build never
//!   binds a `Simd4` bank through the factory and an AArch64 or wasm build never binds `Simd8`.
//!   [`BankParts`] is the factory's own validation and construction, minus exactly that one gate,
//!   and the bank is driven through the production `PreparedNativeEffectBank` trait bodies. So the
//!   class-A differential below runs at `Simd4` and `Simd8` wherever `cargo test` runs.
//! * **Internal witnesses.** Silent admission is invisible in the bits by design, so gate 3 reads
//!   the test-only [`SILENT_ADMISSIONS`](super::SILENT_ADMISSIONS) counter; and gate 4 plants an
//!   envelope word no render can reach.
//!
//! `tests/padding.rs` covers what goes through the factory: binding, payloads, the fixtures and a
//! bypassed lane behind the rack's shunt.

use super::*;
use dsp_reference::class_a;
use dsp_reference::randomized::{Draw, Profile, run_seeds};
use effect_contract::{
    AutomationSpanKind, EffectProcessBlock, EffectQuality, LinkMode, PrepareEffectLimits,
    PreparedPorts, PreparedSidechainPort,
};

const QUANTUM: u32 = 128;
const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];

type Values = [InitialParameterValue; PARAMETER_COUNT * 2];

/// A request as the session compiler prepares a console slot: unkeyed, unbypassed.
fn request(values: &Values, rate: u32, link: LinkMode) -> PrepareEffectRequest<'_> {
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
            maximum_total_state_bytes: 176,
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// `pairs[p]` on parameter `p`, left then right.
fn values_from(pairs: [(f32, f32); PARAMETER_COUNT]) -> Values {
    core::array::from_fn(|slot| InitialParameterValue {
        parameter_index: (slot / 2) as u32,
        channel: if slot % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: if slot % 2 == 0 {
            pairs[slot / 2].0
        } else {
            pairs[slot / 2].1
        },
    })
}

/// Random values in every parameter's domain, edges included; `symmetric` gives both channels the
/// same ones.
fn draw_values(draw: &mut Draw, symmetric: bool) -> Values {
    values_from(core::array::from_fn(|parameter| {
        let spec = &COMPRESSOR_PARAMETERS[parameter];
        let (low, high) = (spec.minimum.unwrap(), spec.maximum.unwrap());
        let left = draw.in_domain(low, high);
        let right = if symmetric {
            left
        } else {
            draw.in_domain(low, high)
        };
        (left, right)
    }))
}

/// The bank width of `L`, with the backend that executes it.
fn width_of<L: Lane>() -> (Backend, BankWidth) {
    match L::WIDTH {
        4 => (Backend::Simd4, BankWidth::Four),
        8 => (Backend::Simd8, BankWidth::Eight),
        width => panic!("no bank of width {width}"),
    }
}

/// The bank `bind_homogeneous_bank` binds for `requests` and `mask` at `L`, whatever this host's
/// own width is.
fn bind<L: Lane>(
    requests: &[PrepareEffectRequest<'_>],
    mask: &[bool],
) -> PreparedCompressorBank<L> {
    let (backend, width) = width_of::<L>();
    let parts = BankParts::validate(PrepareEffectBankRequest {
        backend,
        width,
        requests,
        active_mask: mask,
    })
    .expect("a well-formed bank request");
    assert!(parts.same_program, "every lane clones one program");
    parts.bank::<L>()
}

/// Lane `l` of a mask is active.
fn members_first(members: usize, lanes: usize) -> Vec<bool> {
    (0..lanes).map(|lane| lane < members).collect()
}

/// Every request of a bank: member `k` on the `k`-th active lane, `clone_of`'s request on every
/// padded lane.
fn lane_requests<'a>(
    members: &'a [Values],
    mask: &[bool],
    clone_of: usize,
    rate: u32,
    link: LinkMode,
) -> Vec<PrepareEffectRequest<'a>> {
    let mut next = 0;
    mask.iter()
        .map(|active| {
            if *active {
                next += 1;
                request(&members[next - 1], rate, link)
            } else {
                request(&members[clone_of], rate, link)
            }
        })
        .collect()
}

/// The lanes that carry members, in member order.
fn member_lanes(mask: &[bool]) -> Vec<usize> {
    (0..mask.len()).filter(|lane| mask[*lane]).collect()
}

/// One block of a differential case.
struct Block {
    frames: usize,
    /// Per member: the left and right input.
    inputs: Vec<(Vec<f32>, Vec<f32>)>,
    /// Per member: the spans delivered at this block's first sample (their samples are set when
    /// the block is rendered).
    spans: Vec<Vec<PreparedAutomationSpan>>,
}

/// A padded bank's members, their mask, and what they are fed.
struct Case {
    rate: u32,
    link: LinkMode,
    /// Rendered through `process_bank_mono`: symmetric values, identical planes and symmetric
    /// automation, as the rack only collapses such a bank.
    mono: bool,
    members: Vec<Values>,
    mask: Vec<bool>,
    blocks: Vec<Block>,
}

/// One point on `parameter`, or on both channels of it for a collapsed case; now and then a span
/// the effect refuses (`Both`), which must be counted on the member's own report.
fn draw_spans(draw: &mut Draw, mono: bool) -> Vec<PreparedAutomationSpan> {
    let mut spans = Vec::new();
    if !draw.chance(1, 3) {
        return spans;
    }
    for _ in 0..1 + draw.below(2) {
        let parameter = draw.below(PARAMETER_COUNT);
        let spec = &COMPRESSOR_PARAMETERS[parameter];
        let value = draw.in_domain(spec.minimum.unwrap(), spec.maximum.unwrap());
        let span = |channel| PreparedAutomationSpan {
            kind: AutomationSpanKind::Point,
            channel,
            parameter_index: parameter as u32,
            start_sample: 0,
            end_sample: 0,
            start_value: value,
            end_value: value,
        };
        if draw.chance(1, 12) {
            spans.push(span(ParameterChannel::Both));
        } else if mono {
            spans.push(span(ParameterChannel::Left));
            spans.push(span(ParameterChannel::Right));
        } else {
            spans.push(span(
                draw.pick(&[ParameterChannel::Left, ParameterChannel::Right]),
            ));
        }
    }
    spans
}

/// Seed `seed`'s case at `width`: `1 + seed % (width - 1)` members, so a width's first
/// `3 * (width - 1)` seeds reach every active count under every link mode.
fn draw_case(seed: u64, width: usize) -> Case {
    let mut draw = Draw::new(seed ^ 0x1090);
    let members = 1 + seed as usize % (width - 1);
    let link = LINKS[(seed as usize / (width - 1)) % LINKS.len()];
    let mono = draw.chance(1, 3);
    let rate = draw.pick(&RATES);
    let mask = if draw.chance(1, 2) {
        members_first(members, width)
    } else {
        // Any non-empty mask is legal, not only the planner's members-first layout.
        let mut lanes: Vec<usize> = (0..width).collect();
        for index in (1..width).rev() {
            lanes.swap(index, draw.below(index + 1));
        }
        let mut mask = vec![false; width];
        for lane in &lanes[..members] {
            mask[*lane] = true;
        }
        mask
    };
    let values = (0..members).map(|_| draw_values(&mut draw, mono)).collect();
    let blocks = (0..24)
        .map(|_| {
            let frames = draw.frames(QUANTUM as usize);
            // A fifth of the blocks are silent on every member, so the silent fast path runs.
            let silent = draw.chance(1, 5);
            let inputs = (0..members)
                .map(|_| {
                    let profile = if silent {
                        Profile::Silence
                    } else {
                        draw.profile()
                    };
                    let left: Vec<f32> = (0..frames).map(|_| draw.sample(profile)).collect();
                    let right = if mono {
                        left.clone()
                    } else {
                        let profile = if silent {
                            Profile::Silence
                        } else {
                            draw.profile()
                        };
                        (0..frames).map(|_| draw.sample(profile)).collect()
                    };
                    (left, right)
                })
                .collect();
            let spans = (0..members).map(|_| draw_spans(&mut draw, mono)).collect();
            Block {
                frames,
                inputs,
                spans,
            }
        })
        .collect();
    Case {
        rate,
        link,
        mono,
        members: values,
        mask,
        blocks,
    }
}

/// What a render produced, per member.
#[derive(Debug, PartialEq)]
struct Rendered {
    /// Class-A words of every output sample, left then right, per block.
    outputs: Vec<Vec<u32>>,
    /// Every block's report.
    reports: Vec<Vec<ProcessReport>>,
    /// Class-A words of the final payload, left then right.
    payloads: Vec<Vec<u32>>,
}

fn words(plane: &[f32]) -> impl Iterator<Item = u32> + '_ {
    plane.iter().map(|sample| class_a::bits(*sample))
}

fn payload_words(bytes: &[u8]) -> impl Iterator<Item = u32> + '_ {
    class_a::le_words(bytes).map(u32::from_le_bytes)
}

fn at(spans: &[PreparedAutomationSpan], first_sample: u64) -> Vec<PreparedAutomationSpan> {
    spans
        .iter()
        .map(|span| PreparedAutomationSpan {
            start_sample: first_sample,
            end_sample: first_sample,
            ..*span
        })
        .collect()
}

/// Every member rendered alone, as the graph renders a track per node: the oracle.
fn per_node(case: &Case) -> Rendered {
    let members = case.members.len();
    let mut rendered = Rendered {
        outputs: vec![Vec::new(); members],
        reports: vec![Vec::new(); members],
        payloads: Vec::new(),
    };
    for (member, values) in case.members.iter().enumerate() {
        let mut effect = CompressorFactory
            .prepare(request(values, case.rate, case.link))
            .expect("a member prepares");
        let mut first_sample = 0;
        for block in &case.blocks {
            let (mut left, mut right) = block.inputs[member].clone();
            let spans = at(&block.spans[member], first_sample);
            let report = effect.process(
                EffectProcessBlock::new(&mut left, &mut right, None, first_sample, &spans, QUANTUM)
                    .expect("a bounded block"),
            );
            rendered.outputs[member].extend(words(&left).chain(words(&right)));
            rendered.reports[member].push(report);
            first_sample += block.frames as u64;
        }
        let sizes = effect.metadata().state_sizes;
        let (mut left, mut right) = (
            vec![0_u8; sizes.left_bytes as usize],
            vec![0_u8; sizes.right_bytes as usize],
        );
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("sizes"),
            )
            .expect("a snapshot");
        rendered
            .payloads
            .push(payload_words(&left).chain(payload_words(&right)).collect());
    }
    rendered
}

/// The members rendered as one padded bank at `L`, padded lanes cloning member `clone_of`.
///
/// Also asserts the contract's two obligations on the padded lanes themselves, every block: fed
/// `+0.0`, a padded lane writes `+0.0` (so the next slot of a chain and the next block are fed
/// `+0.0` too), and its report stays empty. And a padded lane is not a track for a payload.
fn banked<L: Lane>(case: &Case, clone_of: usize) -> Rendered {
    let lanes = L::WIDTH;
    let (_, width) = width_of::<L>();
    let requests = lane_requests(&case.members, &case.mask, clone_of, case.rate, case.link);
    let mut bank = bind::<L>(&requests, &case.mask);
    let active = member_lanes(&case.mask);
    let members = active.len();
    let mut rendered = Rendered {
        outputs: vec![Vec::new(); members],
        reports: vec![Vec::new(); members],
        payloads: Vec::new(),
    };
    let mut first_sample = 0;
    for (index, block) in case.blocks.iter().enumerate() {
        let frames = block.frames;
        let mut left = vec![0.0_f32; frames * lanes];
        let mut right = vec![0.0_f32; frames * lanes];
        let mut automation = Vec::new();
        let mut offsets = vec![0_u32; lanes + 1];
        for lane in 0..lanes {
            if let Some(member) = active.iter().position(|each| *each == lane) {
                let (input_left, input_right) = &block.inputs[member];
                for frame in 0..frames {
                    left[frame * lanes + lane] = input_left[frame];
                    right[frame * lanes + lane] = input_right[frame];
                }
                automation.extend(at(&block.spans[member], first_sample));
            }
            offsets[lane + 1] = automation.len() as u32;
        }
        let process = EffectBankProcessBlock::new(
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
        .expect("a bounded bank block");
        let report = if case.mono {
            bank.process_bank_mono(process)
        } else {
            bank.process_bank(process)
        };
        if case.mono {
            // The collapsed body renders the left plane only; the rack's seam writes it to the
            // right. The oracle's right plane is its left, since its inputs and values are.
            right.copy_from_slice(&left);
        }
        for lane in (0..lanes).filter(|lane| !case.mask[*lane]) {
            for frame in 0..frames {
                assert_eq!(
                    (
                        left[frame * lanes + lane].to_bits(),
                        right[frame * lanes + lane].to_bits()
                    ),
                    (0, 0),
                    "W{lanes} block {index}: padded lane {lane} frame {frame} is not +0.0"
                );
            }
            assert_eq!(
                report.reports[lane],
                ProcessReport::default(),
                "W{lanes} block {index}: padded lane {lane} was reported"
            );
        }
        for (member, lane) in active.iter().enumerate() {
            let left = (0..frames).map(|frame| left[frame * lanes + lane]);
            let right = (0..frames).map(|frame| right[frame * lanes + lane]);
            rendered.outputs[member]
                .extend(left.chain(right).map(class_a::bits).collect::<Vec<_>>());
            rendered.reports[member].push(report.reports[*lane]);
        }
        first_sample += frames as u64;
    }
    if case.mono {
        // The rack's disengage seam, before anyone reads the right channel's state.
        bank.desymmetrize_channels();
    }
    let sizes = bank.instance.metadata.state_sizes;
    for lane in 0..lanes {
        let (mut left, mut right) = (
            vec![0_u8; sizes.left_bytes as usize],
            vec![0_u8; sizes.right_bytes as usize],
        );
        let snapshot = bank.snapshot_track_state_payload(
            lane as u32,
            StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("sizes"),
        );
        if case.mask[lane] {
            snapshot.expect("an active lane is a track");
            rendered
                .payloads
                .push(payload_words(&left).chain(payload_words(&right)).collect());
        } else {
            assert_eq!(
                snapshot.err().map(|error| error.code),
                Some("effect.state.track"),
                "a padded lane is not a track"
            );
        }
    }
    rendered
}

/// Compares a banked render against the oracle, member by member, with the first difference.
fn assert_same(context: &str, oracle: &Rendered, banked: &Rendered, case: &Case) {
    for member in 0..case.members.len() {
        let (want, got) = (&oracle.outputs[member], &banked.outputs[member]);
        if let Some(word) = want.iter().zip(got).position(|(a, b)| a != b) {
            panic!(
                "{context}: member {member} output word {word}: per node {:#010x}, banked {:#010x}",
                want[word], got[word]
            );
        }
        assert_eq!(want.len(), got.len(), "{context}: member {member} length");
        for (block, (want, got)) in oracle.reports[member]
            .iter()
            .zip(&banked.reports[member])
            .enumerate()
        {
            assert_eq!(want, got, "{context}: member {member} block {block} report");
        }
        assert_eq!(
            oracle.payloads[member], banked.payloads[member],
            "{context}: member {member} payload"
        );
    }
}

/// What the differential reached, for its non-vacuity assertions.
#[derive(Default)]
struct Reach {
    /// Blocks in which one member's D7 check rejected a block and another member's did not: the
    /// blocks a whole-bank recovery would get wrong.
    mixed_rejections: usize,
    /// Points on a member's parameter and channel less than one ramp after the previous point on
    /// it, so the new target cuts a ramp in flight (the #1069 shape).
    cut_ramps: usize,
    /// Blocks the silent fast path skipped in the banked renders.
    silent_admissions: usize,
    /// Cases rendered collapsed.
    collapsed: usize,
}

/// Gates 1 and 2 at one width: every seed's padded bank, under every clone source, renders every
/// member's bits, reports and payload as the member does alone.
fn padded_differential<L: Lane>(seed: u64, reach: &mut Reach) {
    let case = draw_case(seed, L::WIDTH);
    let oracle = per_node(&case);
    reach.collapsed += usize::from(case.mono);
    for (block, _) in case.blocks.iter().enumerate() {
        let rejected = |member: usize| {
            let report = oracle.reports[member][block];
            report.nonfinite_left_blocks + report.nonfinite_right_blocks != 0
        };
        let members = case.members.len();
        reach.mixed_rejections +=
            usize::from((0..members).any(rejected) && !(0..members).all(rejected));
    }
    for member in 0..case.members.len() {
        let mut last = [[None::<u64>; 2]; PARAMETER_COUNT];
        let mut first_sample = 0_u64;
        for block in &case.blocks {
            for span in &block.spans[member] {
                let channel = match span.channel {
                    ParameterChannel::Left => 0,
                    ParameterChannel::Right => 1,
                    ParameterChannel::Both => continue,
                };
                let previous = &mut last[span.parameter_index as usize][channel];
                if previous
                    .is_some_and(|at| first_sample - at < u64::from(design::SMOOTHING_SAMPLES))
                {
                    reach.cut_ramps += 1;
                }
                *previous = Some(first_sample);
            }
            first_sample += block.frames as u64;
        }
    }
    for clone_of in 0..case.members.len() {
        let admitted = SILENT_ADMISSIONS.with(core::cell::Cell::get);
        let banked = banked::<L>(&case, clone_of);
        let context = format!(
            "seed {seed} W{} {:?} {} members {:?} mono {} clone of {clone_of}",
            L::WIDTH,
            case.link,
            case.rate,
            case.mask,
            case.mono
        );
        assert_same(&context, &oracle, &banked, &case);
        reach.silent_admissions += SILENT_ADMISSIONS.with(core::cell::Cell::get) - admitted;
    }
}

/// Gates 1 and 2: a padded bank's members render their per-node bits, at `Simd4` and `Simd8`.
///
/// Seeds reach every active count `1..W` under every link mode at both widths, members-first and
/// scattered masks, every launch rate, dual and collapsed. Inputs mix every `Profile` (hostile
/// NaNs, infinities and words past `1e30` among them) with whole-bank silence; members are
/// automated with points that often land inside the previous point's 64-sample ramp, because
/// blocks are as short as one frame (the #1069 shape); and every member serves in turn as the
/// padded lanes' clone source.
///
/// Red for, among others:
/// * a whole-bank D7 recovery (the pre-#1090 `kernel::finish_channel`): a hostile word on one
///   member zeroes and resets its bank-mates;
/// * a padded lane's failure reported, or its span applied or counted;
/// * a padded lane that does not stay at `+0.0` out, or a padded lane counted as a track;
/// * active bits that depend on the clone source (every source is rendered);
/// * a link that reads another lane (every link mode, heterogeneous neighbours).
#[test]
fn a_padded_bank_renders_each_member_as_its_own_instance() {
    let mut reach = Reach::default();
    run_seeds(
        "a_padded_bank_renders_each_member_as_its_own_instance",
        "cargo test -p compressor --lib -- --exact \
         padding_tests::a_padded_bank_renders_each_member_as_its_own_instance",
        21,
        |seed| {
            padded_differential::<Simd4>(seed, &mut reach);
            padded_differential::<Simd8>(seed, &mut reach);
        },
    );
    println!(
        "reach: {} mixed rejections, {} cut ramps, {} silent admissions, {} collapsed cases",
        reach.mixed_rejections, reach.cut_ramps, reach.silent_admissions, reach.collapsed
    );
    if !dsp_reference::randomized::replaying() {
        assert!(
            reach.mixed_rejections > 0,
            "no block rejected one member alone"
        );
        assert!(reach.cut_ramps > 0, "no point cut a ramp in flight");
        assert!(
            reach.silent_admissions > 0,
            "no banked block took the silent fast path"
        );
        assert!(reach.collapsed > 0, "no case rendered collapsed");
    }
}

/// Gate 3 at one width: with every member silent, a padded bank takes the silent fast path on
/// every block after the one that earns it, exactly as a full bank does.
fn silent_admission<L: Lane>(mono: bool) {
    let lanes = L::WIDTH;
    let (_, width) = width_of::<L>();
    const TONE_BLOCKS: usize = 4;
    const SILENT_BLOCKS: usize = 16;
    // A threshold above the signal's level, so the envelope rests at exactly `+0.0` dB and the
    // first silent block earns the claim (see `tests/silent_fixed_point.rs` for why a compressing
    // detector would take a thousand blocks to).
    let members: Vec<Values> = (0..lanes)
        .map(|member| {
            let threshold = -6.0 - member as f32;
            values_from([
                (threshold, threshold),
                (8.0, 8.0),
                (6.0, 6.0),
                (10.0, 10.0),
                (50.0, 50.0),
                (3.0, 3.0),
                (1.0, 1.0),
            ])
        })
        .collect();
    for count in 1..=lanes {
        let mask = members_first(count, lanes);
        let requests = lane_requests(
            &members[..count],
            &mask,
            count - 1,
            48_000,
            LinkMode::Maximum,
        );
        let mut bank = bind::<L>(&requests, &mask);
        let offsets = vec![0_u32; lanes + 1];
        let mut admitted = Vec::new();
        let mut draw = Draw::new(0x1090_0003);
        for block in 0..TONE_BLOCKS + SILENT_BLOCKS {
            let mut left = vec![0.0_f32; QUANTUM as usize * lanes];
            let mut right = vec![0.0_f32; QUANTUM as usize * lanes];
            if block < TONE_BLOCKS {
                // About -26 dBFS: under every member's threshold and knee.
                for frame in 0..QUANTUM as usize {
                    for lane in 0..count {
                        left[frame * lanes + lane] = draw.noise(0.05);
                        right[frame * lanes + lane] = draw.noise(0.05);
                    }
                }
            }
            let before = SILENT_ADMISSIONS.with(core::cell::Cell::get);
            let process = EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                QUANTUM,
                width,
                block as u64 * u64::from(QUANTUM),
                &[],
                &offsets,
                QUANTUM,
            )
            .expect("a bank block");
            if mono {
                bank.process_bank_mono(process);
            } else {
                bank.process_bank(process);
            }
            admitted.push(SILENT_ADMISSIONS.with(core::cell::Cell::get) - before);
        }
        let expected: Vec<usize> = (0..TONE_BLOCKS + SILENT_BLOCKS)
            .map(|block| usize::from(block > TONE_BLOCKS))
            .collect();
        assert_eq!(
            admitted, expected,
            "W{lanes}, {count} of {lanes} lanes active, mono {mono}: blocks the fast path skipped"
        );
    }
}

/// Gate 3: a bank whose members are all silent takes silent admission, padded or not, dual or
/// collapsed, at `Simd4` and `Simd8`. The full bank (every lane active) is the control.
///
/// Red for a padded lane that is not at rest on `+0.0` input: one bound from all-zero parameters
/// in place of the clone, one given a ramp or a span, one fed or left anything but `+0.0`. Any of
/// them keeps the whole bank on its slow body, which moves no bit and so is visible only here.
#[test]
fn a_silent_padded_bank_takes_the_silent_fast_path() {
    silent_admission::<Simd4>(false);
    silent_admission::<Simd4>(true);
    silent_admission::<Simd8>(false);
    silent_admission::<Simd8>(true);
}

/// Writes `value` into one lane of an envelope vector.
fn plant<L: Lane>(envelope: &mut L, lane: usize, value: f32) {
    let mut words = [0.0_f32; MAX_WIDTH];
    envelope.store(&mut words);
    words[lane] = value;
    *envelope = L::load(&words);
}

/// Gate 4 at one width: a planted state word on one lane is recovered on that lane alone.
///
/// * **On a member**, the planted word is an envelope of `f32::MAX` dB. The kernel clamps every
///   envelope it computes into `[-100, 0]`, so no render reaches it; from it the next applied gain
///   is `2^127`, the lane's output leaves the D7 bound, its block is rejected once, and the reset
///   envelope brings it back. (A planted NaN or infinity would not test D7: the envelope becomes
///   NaN, `fast_gain_from_db` clamps a NaN to `2^-126`, and the output stays in bounds. That is the
///   same per node, and it is not this slice's.)
/// * **On a padded lane**, which is fed `+0.0` and so can never leave the bound through its
///   envelope, the planted word is a NaN `mix`: with a non-zero makeup the lane writes NaN on every
///   block. Every block is rejected, recovered to `+0.0`, and reported against nobody.
fn planted_state<L: Lane>() {
    let lanes = L::WIDTH;
    let (_, width) = width_of::<L>();
    let rate = 48_000;
    let members: Vec<Values> = (0..lanes)
        .map(|member| {
            let threshold = -12.0 - 3.0 * member as f32;
            values_from([
                (threshold, threshold - 1.0),
                (4.0, 4.0),
                (6.0, 6.0),
                (5.0, 5.0),
                (80.0, 80.0),
                (3.0, 3.0),
                (1.0, 1.0),
            ])
        })
        .collect();
    for count in 1..=lanes {
        for target in 0..lanes {
            for right_channel in [false, true] {
                let mask = members_first(count, lanes);
                let requests = lane_requests(&members[..count], &mask, 0, rate, LinkMode::DualMono);
                let mut bank = bind::<L>(&requests, &mask);
                let mut scalars: Vec<PreparedCompressor> = (0..count)
                    .map(|member| {
                        let request = request(&members[member], rate, LinkMode::DualMono);
                        let metadata = expected_prepared_metadata(&COMPRESSOR_DESCRIPTOR, request)
                            .expect("metadata");
                        let (left, right) =
                            initial_defaults(request.initial_values).expect("values");
                        PreparedCompressor {
                            instance: Instance::new(
                                metadata,
                                &[left; MAX_WIDTH],
                                &[right; MAX_WIDTH],
                            ),
                        }
                    })
                    .collect();
                let offsets = vec![0_u32; lanes + 1];
                let member = target < count;
                let context = format!(
                    "W{lanes}, {count} active, planted on {} lane {target} {}",
                    if member { "member" } else { "padded" },
                    if right_channel { "right" } else { "left" }
                );
                let mut draw = Draw::new(0x1090_0004 + target as u64);
                const PLANTED: usize = 3;
                for block in 0..8_usize {
                    if block == PLANTED {
                        let channel = if right_channel {
                            &mut bank.instance.right
                        } else {
                            &mut bank.instance.left
                        };
                        if member {
                            plant(&mut channel.gain_reduction_db, target, f32::MAX);
                            let scalar = &mut scalars[target].instance;
                            let channel = if right_channel {
                                &mut scalar.right
                            } else {
                                &mut scalar.left
                            };
                            plant(&mut channel.gain_reduction_db, 0, f32::MAX);
                        } else {
                            channel.words[crate::design::COEF_MIX][target] = f32::NAN;
                        }
                    }
                    let frames = QUANTUM as usize;
                    let inputs: Vec<(Vec<f32>, Vec<f32>)> = (0..count)
                        .map(|_| {
                            (
                                (0..frames).map(|_| draw.noise(0.5)).collect(),
                                (0..frames).map(|_| draw.noise(0.5)).collect(),
                            )
                        })
                        .collect();
                    let mut left = vec![0.0_f32; frames * lanes];
                    let mut right = vec![0.0_f32; frames * lanes];
                    for (lane, (input_left, input_right)) in inputs.iter().enumerate() {
                        for frame in 0..frames {
                            left[frame * lanes + lane] = input_left[frame];
                            right[frame * lanes + lane] = input_right[frame];
                        }
                    }
                    let report = bank.process_bank(
                        EffectBankProcessBlock::new(
                            &mut left,
                            &mut right,
                            None,
                            QUANTUM,
                            width,
                            (block * frames) as u64,
                            &[],
                            &offsets,
                            QUANTUM,
                        )
                        .expect("a bank block"),
                    );
                    for lane in 0..lanes {
                        let tripped = member && block == PLANTED && lane == target;
                        let expected = ProcessReport {
                            nonfinite_left_blocks: u64::from(tripped && !right_channel),
                            nonfinite_right_blocks: u64::from(tripped && right_channel),
                            ..ProcessReport::default()
                        };
                        assert_eq!(
                            report.reports[lane], expected,
                            "{context}: block {block} lane {lane} report"
                        );
                        if lane >= count {
                            assert!(
                                (0..frames).all(|frame| left[frame * lanes + lane].to_bits() == 0
                                    && right[frame * lanes + lane].to_bits() == 0),
                                "{context}: block {block} padded lane {lane} is not +0.0"
                            );
                            continue;
                        }
                        let (mut want_left, mut want_right) = inputs[lane].clone();
                        let want = scalars[lane].process(
                            EffectProcessBlock::new(
                                &mut want_left,
                                &mut want_right,
                                None,
                                (block * frames) as u64,
                                &[],
                                QUANTUM,
                            )
                            .expect("a block"),
                        );
                        assert_eq!(want, report.reports[lane], "{context}: per node report");
                        for frame in 0..frames {
                            assert_eq!(
                                (
                                    class_a::bits(left[frame * lanes + lane]),
                                    class_a::bits(right[frame * lanes + lane])
                                ),
                                (
                                    class_a::bits(want_left[frame]),
                                    class_a::bits(want_right[frame])
                                ),
                                "{context}: block {block} lane {lane} frame {frame}"
                            );
                        }
                    }
                }
                if !member {
                    // Non-vacuity: the planted padded lane really was rejected, every block since.
                    let channel = if right_channel {
                        &bank.instance.right
                    } else {
                        &bank.instance.left
                    };
                    assert!(channel.words[crate::design::COEF_MIX][target].is_nan());
                }
            }
        }
    }
}

/// Gate 4: a planted state word on one lane is rejected, recovered and reported on that lane and
/// channel alone, at `Simd4` and `Simd8`, on every lane of every active count. The members all
/// equal their per-node instances, the planted one included (whose own instance carries the same
/// word), before, during and after the rejection. A padded lane's rejections are recovered to
/// `+0.0` and charged to nobody.
///
/// Red for a whole-bank recovery (every member's bits and reports move), for a recovery that zeroes
/// the lane but keeps its envelope (the lane stays at `2^127` gain and trips every block), for a
/// padded lane reported or left writing NaN, and for a report on the wrong lane or channel.
#[test]
fn a_planted_envelope_recovers_its_own_lane_alone() {
    planted_state::<Simd4>();
    planted_state::<Simd8>();
}
