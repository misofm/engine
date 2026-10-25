//! Issue #1370 (slice 1370b): a lane's detector link glides between modes (D1-D3, D6), at every
//! width the build has: the scalar instance, and banks bound at `Simd4` and `Simd8` (the width the
//! build does not execute binds through `bind_bank::<false>`, as `padding_tests` explains).

use super::padding_tests::{Draw, LINKS, QUANTUM, Values, bind, request, values_from};
use super::*;
use dsp_reference::{
    ReferenceLinkGlide, ReferenceTransientShaper, ReferenceTransientShaperParameters,
    reference_link_blend,
};
use effect_contract::{BankWidth, EffectProcessBlock, link_glide_weights};

const RATE: u32 = 48_000;
const BLOCK: usize = 48;
/// The glide: retargeted before the block that starts at `GLIDE_FRAME`, so it crosses a block
/// boundary.
const GLIDE_FRAME: usize = 2_400;
const GLIDE_SAMPLES: u32 = 64;
const FRAMES: usize = 9_600;
/// `tests/oracle.rs`'s gate on the rendered gain against the `f64` model (its impulse, step and
/// decay rows); the glide adds the rounding of the production weights' iterated additions.
const TOLERANCE_DB: f64 = 0.01;

/// One prepared shaper under test: the scalar instance at one lane, else a bank.
enum Under {
    Scalar(Box<dyn PreparedNativeEffect>),
    Bank(Box<dyn PreparedNativeEffectBank>, BankWidth),
}

impl Under {
    fn new(width: Option<BankWidth>, requests: &[PrepareEffectRequest<'_>]) -> Self {
        let Some(width) = width else {
            let prepared = TransientShaperFactory
                .prepare(requests[0])
                .expect("prepare");
            return Self::Scalar(prepared.processor);
        };
        let mask = vec![true; requests.len()];
        let bank = bind(width, requests, &mask)
            .expect("a bank binds")
            .expect("one program");
        Self::Bank(bank.processor, width)
    }

    /// Lane `lane`'s left-channel link weight words in its payload: `w_link` then `w_avg`, each
    /// `current, target, step, remaining`.
    fn link_words(&self, lane: usize) -> [u32; 8] {
        let sizes = TRANSIENT_SHAPER_DESCRIPTOR.qualities[0].maximum_state;
        let (mut common, mut left, mut right) = (
            vec![0; sizes.common_bytes as usize],
            vec![0; sizes.left_bytes as usize],
            vec![0; sizes.right_bytes as usize],
        );
        let output =
            StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes");
        match self {
            Self::Scalar(effect) => effect.snapshot_state_payload(output),
            Self::Bank(bank, _) => bank.snapshot_track_state_payload(lane as u32, output),
        }
        .expect("snapshot");
        core::array::from_fn(|word| {
            let at = (LINK_WORD + word) * 4;
            u32::from_le_bytes(left[at..at + 4].try_into().expect("a word"))
        })
    }

    fn retarget(&mut self, lane: usize, mode: LinkMode, samples: u32) -> bool {
        match self {
            Self::Scalar(effect) => effect.retarget_link(lane, mode, samples),
            Self::Bank(bank, _) => bank.retarget_link(lane, mode, samples),
        }
    }

    /// One block of frame-major planes, with the flattened spans `flat` at `offsets`.
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        first_sample: u64,
        flat: &[PreparedAutomationSpan],
        offsets: &[u32],
    ) {
        let frames = left.len() / (offsets.len() - 1);
        match self {
            Self::Scalar(effect) => {
                effect.process(
                    EffectProcessBlock::new(left, right, None, first_sample, flat, QUANTUM)
                        .expect("a well-shaped block"),
                );
            }
            Self::Bank(bank, width) => {
                bank.process_bank(
                    EffectBankProcessBlock::new(
                        left,
                        right,
                        None,
                        frames as u32,
                        *width,
                        first_sample,
                        flat,
                        offsets,
                        QUANTUM,
                    )
                    .expect("a well-shaped bank block"),
                );
            }
        }
    }
}

/// Every width the build has: the scalar instance, then each bank width.
fn widths() -> Vec<Option<BankWidth>> {
    core::iter::once(None)
        .chain(BankWidth::ALL.iter().copied().map(Some))
        .collect()
}

fn lanes(width: Option<BankWidth>) -> usize {
    width.map_or(1, |width| width.lanes() as usize)
}

/// `spans[lane]` at `first_sample`, flattened in lane order, and the lanes' offsets into it.
fn flatten(
    spans: &[Vec<PreparedAutomationSpan>],
    first_sample: u64,
) -> (Vec<PreparedAutomationSpan>, Vec<u32>) {
    let mut flat = Vec::new();
    let mut offsets = vec![0_u32];
    for lane_spans in spans {
        flat.extend(lane_spans.iter().map(|span| PreparedAutomationSpan {
            sample: first_sample,
            ..*span
        }));
        offsets.push(flat.len() as u32);
    }
    (flat, offsets)
}

/// `planes[lane]`, frame-major.
fn interleave(planes: &[Vec<f32>]) -> Vec<f32> {
    let frames = planes[0].len();
    (0..frames * planes.len())
        .map(|index| planes[index % planes.len()][index / planes.len()])
        .collect()
}

/// Deterministic noise in `[-scale, scale]`.
fn noise(seed: u64, frames: usize, scale: f32) -> Vec<f32> {
    let mut state = seed | 1;
    (0..frames)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            scale * ((state >> 40) as f32 / (1_u64 << 23) as f32 - 1.0)
        })
        .collect()
}

const ATTACK: f32 = 0.9;
const SUSTAIN: f32 = 0.6;
const CHANGED_ATTACK: f32 = 0.3;

fn oracle_values() -> Values {
    values_from([(ATTACK, ATTACK), (SUSTAIN, SUSTAIN), (1.0, 1.0)])
}

/// The left channel: 440 Hz bursts every 20 ms at -6 dBFS, each decaying with a 4 ms constant,
/// so its envelopes carry transients. The right: a steady 330 Hz tone 20 dB lower, whose own
/// envelopes carry next to none, so a linked detector moves its gain.
fn oracle_signals() -> (Vec<f32>, Vec<f32>) {
    let tau = core::f64::consts::TAU;
    let left = (0..FRAMES)
        .map(|frame| {
            let decay = (-((frame % 960) as f64) / 192.0).exp();
            (0.5 * decay * (tau * 440.0 * frame as f64 / 48_000.0).sin()) as f32
        })
        .collect();
    let right = (0..FRAMES)
        .map(|frame| (0.05 * (tau * 330.0 * frame as f64 / 48_000.0).sin()) as f32)
        .collect();
    (left, right)
}

/// One glide case of the oracle test: its length and, when given, the frame of a live attack
/// amount change on every lane mid-glide.
#[derive(Clone, Copy, Debug)]
struct GlideCase {
    samples: u32,
    change: Option<usize>,
}

const CASES: [GlideCase; 2] = [
    GlideCase {
        samples: GLIDE_SAMPLES,
        change: None,
    },
    // Longer than the 64-sample parameter ramp, and crossed by one on every lane: the ramp prefix
    // must keep covering the glide while the parameter ramp runs inside it and after it ends.
    GlideCase {
        samples: 4_800,
        change: Some(4_800),
    },
];

/// The `f64` model's output for one lane: `from` until the glide, then the glide to `to` (a step
/// glide to `from` itself for a lane that rests), and the case's amount change as the 64-sample
/// linear ramp the descriptor states.
fn oracle_trace(
    from: LinkMode,
    to: LinkMode,
    case: GlideCase,
    left: &[f32],
    right: &[f32],
) -> (Vec<f64>, Vec<f64>) {
    let parameters = |attack: f64| ReferenceTransientShaperParameters {
        attack_amount: attack,
        sustain_amount: f64::from(SUSTAIN),
        mix: 1.0,
    };
    let reference = || {
        ReferenceTransientShaper::new(f64::from(RATE), parameters(f64::from(ATTACK)))
            .expect("reference")
    };
    let (mut model_left, mut model_right) = (reference(), reference());
    let weights = |mode: LinkMode| link_glide_weights(mode).map(f64::from);
    let samples = if from == to { 0 } else { case.samples };
    let mut glide = ReferenceLinkGlide::new(weights(from), weights(to), samples);
    let (mut out_left, mut out_right) = (Vec::new(), Vec::new());
    for frame in 0..FRAMES {
        let in_force = if frame < GLIDE_FRAME {
            weights(from)
        } else {
            glide.next_weights().0
        };
        if let Some(change) = case.change
            && (change..change + RAMP_SAMPLES as usize).contains(&frame)
        {
            let k = (frame - change + 1) as f64 / f64::from(RAMP_SAMPLES);
            let attack = f64::from(ATTACK) + (f64::from(CHANGED_ATTACK) - f64::from(ATTACK)) * k;
            model_left.set_parameters(parameters(attack)).expect("ramp");
            model_right
                .set_parameters(parameters(attack))
                .expect("ramp");
        }
        let (x_left, x_right) = (f64::from(left[frame]), f64::from(right[frame]));
        let (d_left, d_right) = reference_link_blend(x_left, x_right, in_force);
        out_left.push(model_left.process_sample(x_left, d_left));
        out_right.push(model_right.process_sample(x_right, d_right));
    }
    (out_left, out_right)
}

/// The worst deviation of one rendered channel's gain from the model's, in dB.
fn deviation(rendered: &[f32], model: &[f64], dry: &[f32]) -> f64 {
    let mut worst = 0.0_f64;
    for frame in 0..rendered.len() {
        if dry[frame].abs() < 1.0e-4 {
            continue;
        }
        let dry = f64::from(dry[frame]).abs();
        let applied = 20.0 * math::log10(f64::from(rendered[frame]).abs() / dry);
        let expected = 20.0 * math::log10(model[frame].abs() / dry);
        worst = worst.max((applied - expected).abs());
    }
    worst
}

/// The attack change's spans, both channels, on every lane at `first_sample`.
fn change_spans(lanes: usize, first_sample: u64) -> Vec<Vec<PreparedAutomationSpan>> {
    let span = |channel| PreparedAutomationSpan {
        channel,
        parameter_index: 0,
        sample: first_sample,
        value: CHANGED_ATTACK,
    };
    vec![vec![span(ParameterChannel::Left), span(ParameterChannel::Right)]; lanes]
}

/// **Gate 1.** Each glide direction, over 64 samples and over 4,800 samples crossed by a live
/// attack amount change on every lane (the gliding lane and its bank-mates), each glide crossing
/// a block boundary, on a bursting left channel and a steady right one 20 dB lower: every lane's
/// gain stays within `0.01 dB` of the `f64` shaper whose detector is the reference blend, and
/// after it the target law, while the bank-mates rest at their own modes; and the glide arrives
/// exactly (its weights at the target, no countdown left). At the scalar instance, `Simd4` and
/// `Simd8`.
#[test]
fn a_glide_tracks_the_f64_blend_into_the_target_law() {
    let (source_left, source_right) = oracle_signals();
    let values = oracle_values();
    for case in CASES {
        for width in widths() {
            let lanes = lanes(width);
            for from in LINKS {
                for to in LINKS.into_iter().filter(|mode| *mode != from) {
                    oracle_case(
                        width,
                        lanes,
                        case,
                        (from, to),
                        (&source_left, &source_right),
                        &values,
                    );
                }
            }
        }
    }
}

fn oracle_case(
    width: Option<BankWidth>,
    lanes: usize,
    case: GlideCase,
    (from, to): (LinkMode, LinkMode),
    (source_left, source_right): (&[f32], &[f32]),
    values: &Values,
) {
    let context = format!("{width:?} {case:?} {from:?} -> {to:?}");
    let modes: Vec<LinkMode> = (0..lanes)
        .map(|lane| {
            if lane == 0 {
                from
            } else {
                LINKS[lane % LINKS.len()]
            }
        })
        .collect();
    let requests: Vec<_> = modes
        .iter()
        .map(|mode| request(values, RATE, *mode))
        .collect();
    let mut under = Under::new(width, &requests);
    let mut left = interleave(&vec![source_left.to_vec(); lanes]);
    let mut right = interleave(&vec![source_right.to_vec(); lanes]);
    for block in 0..FRAMES / BLOCK {
        let first = block * BLOCK;
        if first == GLIDE_FRAME {
            assert!(under.retarget(0, to, case.samples), "a supported glide");
        }
        let spans = if case.change == Some(first) {
            change_spans(lanes, first as u64)
        } else {
            vec![Vec::new(); lanes]
        };
        let (flat, offsets) = flatten(&spans, first as u64);
        let range = first * lanes..(first + BLOCK) * lanes;
        under.process(
            &mut left[range.clone()],
            &mut right[range],
            first as u64,
            &flat,
            &offsets,
        );
    }
    let rest = link_glide_weights(to).map(f32::to_bits);
    assert_eq!(
        under.link_words(0),
        [rest[0], rest[0], 0, 0, rest[1], rest[1], 0, 0],
        "{context}: the glide did not arrive exactly"
    );
    for (lane, mode) in modes.iter().enumerate() {
        let target = if lane == 0 { to } else { *mode };
        let (model_left, model_right) =
            oracle_trace(*mode, target, case, source_left, source_right);
        let lane_of = |planes: &[f32]| -> Vec<f32> {
            (0..FRAMES)
                .map(|frame| planes[frame * lanes + lane])
                .collect()
        };
        let worst = deviation(&lane_of(&left), &model_left, source_left).max(deviation(
            &lane_of(&right),
            &model_right,
            source_right,
        ));
        assert!(
            worst <= TOLERANCE_DB,
            "{context} lane {lane}: worst deviation {worst:e} dB"
        );
    }
    // Not vacuous: the quiet channel's model gain moves with the glide.
    let (_, glided) = oracle_trace(from, to, case, source_left, source_right);
    let (_, rested) = oracle_trace(from, from, case, source_left, source_right);
    let moved = deviation(
        &glided[GLIDE_FRAME..]
            .iter()
            .map(|value| *value as f32)
            .collect::<Vec<_>>(),
        &rested[GLIDE_FRAME..],
        &source_right[GLIDE_FRAME..],
    );
    assert!(
        moved > 1.0,
        "{context}: the glide changed nothing ({moved} dB)"
    );
}

/// **Gates 2 and 6.** A full bank of distinct members at mixed modes, where lane 0 glides while
/// its bank-mates rest or ramp parameters: every other lane renders bit for bit as its own scalar
/// instance, and the retarget and the switching blocks allocate and free nothing. At `Simd4` and
/// `Simd8`.
#[test]
fn a_glide_moves_no_bank_mates_bit_and_allocates_nothing() {
    bench_support::alloc::assert_installed();
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        for seed in 0..6_u64 {
            let mut draw = Draw::new(0x1370_2000 + seed);
            let members: Vec<Values> = (0..lanes).map(|_| draw.values()).collect();
            let modes: Vec<LinkMode> = (0..lanes)
                .map(|lane| LINKS[(lane + seed as usize) % LINKS.len()])
                .collect();
            let target = LINKS[(seed as usize + 1) % LINKS.len()];
            let requests: Vec<_> = members
                .iter()
                .zip(&modes)
                .map(|(values, mode)| request(values, RATE, *mode))
                .collect();
            let mut bank = Under::new(Some(width), &requests);
            let mut scalars: Vec<Under> = requests
                .iter()
                .map(|request| Under::new(None, core::slice::from_ref(request)))
                .collect();
            let glide_block = 4;
            for block in 0..12 {
                let first_sample = (block * BLOCK) as u64;
                let inputs: Vec<(Vec<f32>, Vec<f32>)> = (0..lanes)
                    .map(|lane| {
                        let salt = seed * 1_000 + (block * lanes + lane) as u64;
                        (noise(salt, BLOCK, 0.5), noise(salt + 500, BLOCK, 0.02))
                    })
                    .collect();
                let mut spans: Vec<Vec<PreparedAutomationSpan>> =
                    (0..lanes).map(|_| draw.spans(first_sample)).collect();
                spans[0].clear();
                let (flat, offsets) = flatten(&spans, first_sample);
                let mut left =
                    interleave(&inputs.iter().map(|(l, _)| l.clone()).collect::<Vec<_>>());
                let mut right =
                    interleave(&inputs.iter().map(|(_, r)| r.clone()).collect::<Vec<_>>());
                let mark = bench_support::alloc::current_thread_counters();
                if block == glide_block {
                    assert!(bank.retarget(0, target, GLIDE_SAMPLES), "a supported glide");
                }
                bank.process(&mut left, &mut right, first_sample, &flat, &offsets);
                let delta = bench_support::alloc::current_thread_delta_since(mark);
                if block >= glide_block {
                    assert_eq!(
                        delta.allocations + delta.deallocations + delta.reallocations,
                        0,
                        "{width:?} seed {seed} block {block}: a gliding block touched the heap"
                    );
                }
                for (lane, scalar) in scalars.iter_mut().enumerate().skip(1) {
                    let (mut own_left, mut own_right) = inputs[lane].clone();
                    let (own_flat, own_offsets) =
                        flatten(core::slice::from_ref(&spans[lane]), first_sample);
                    scalar.process(
                        &mut own_left,
                        &mut own_right,
                        first_sample,
                        &own_flat,
                        &own_offsets,
                    );
                    for frame in 0..BLOCK {
                        assert_eq!(
                            (
                                left[frame * lanes + lane].to_bits(),
                                right[frame * lanes + lane].to_bits()
                            ),
                            (own_left[frame].to_bits(), own_right[frame].to_bits()),
                            "{width:?} seed {seed} block {block} lane {lane} frame {frame}: a \
                             bank-mate of the glide moved"
                        );
                    }
                }
            }
        }
    }
}

/// **Gate 5.** A lane snapshotted mid-glide and restored into an instance prepared with the
/// glide's target mode renders bit for bit as the uninterrupted one. A payload restored under
/// another mode leaves that mode at rest, and a forged weight outside `[0, 1]`, or a target that
/// is no mode's, is refused and changes nothing.
#[test]
fn a_lane_snapshotted_mid_glide_continues_bit_for_bit_and_forged_weights_are_refused() {
    let values = oracle_values();
    let prepare = |mode| {
        TransientShaperFactory
            .prepare(request(&values, RATE, mode))
            .expect("prepare")
    };
    let sizes = prepare(LinkMode::DualMono).metadata.state_sizes;
    let snapshot = |effect: &dyn PreparedNativeEffect| {
        let (mut common, mut left, mut right) = (
            vec![0; sizes.common_bytes as usize],
            vec![0; sizes.left_bytes as usize],
            vec![0; sizes.right_bytes as usize],
        );
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut common, &mut left, &mut right, sizes).expect("sizes"),
            )
            .expect("snapshot");
        (common, left, right)
    };
    let restore = |effect: &mut dyn PreparedNativeEffect, payload: &(Vec<u8>, Vec<u8>, Vec<u8>)| {
        effect.restore_state_payload(
            1,
            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("sizes"),
        )
    };
    let (input_left, input_right) = oracle_signals();
    let render = |effect: &mut dyn PreparedNativeEffect, range: core::ops::Range<usize>| {
        let (mut left, mut right) = (
            input_left[range.clone()].to_vec(),
            input_right[range.clone()].to_vec(),
        );
        let quantum = QUANTUM as usize;
        for (block, (left, right)) in left
            .chunks_mut(quantum)
            .zip(right.chunks_mut(quantum))
            .enumerate()
        {
            let first_sample = (range.start + block * quantum) as u64;
            effect.process(
                EffectProcessBlock::new(left, right, None, first_sample, &[], QUANTUM)
                    .expect("a block"),
            );
        }
        let bits = |values: &[f32]| {
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        };
        (bits(&left), bits(&right))
    };
    let mut source = prepare(LinkMode::DualMono).processor;
    render(source.as_mut(), 0..1_024);
    assert!(source.retarget_link(0, LinkMode::Average, 96));
    render(source.as_mut(), 1_024..1_044);
    let payload = snapshot(source.as_ref());

    let mut successor = prepare(LinkMode::Average).processor;
    restore(successor.as_mut(), &payload).expect("a mid-glide payload restores");
    for range in [1_044..1_100, 1_100..1_400] {
        assert_eq!(
            render(successor.as_mut(), range.clone()),
            render(source.as_mut(), range.clone()),
            "frames {range:?}: the restored glide parted from the uninterrupted one"
        );
    }

    // Under another mode the lane keeps its own, at rest: Maximum's weights `[1, 0]`. The lane
    // is itself mid-glide to Maximum: the restore replaces that glide too.
    let mut other = prepare(LinkMode::DualMono).processor;
    assert!(other.retarget_link(0, LinkMode::Maximum, 96));
    render(other.as_mut(), 0..20);
    restore(other.as_mut(), &payload).expect("restores under another mode");
    let first = LINK_WORD * 4;
    let words = |section: &[u8]| -> Vec<u32> {
        section[first..first + 32]
            .as_chunks::<4>()
            .0
            .iter()
            .map(|word| u32::from_le_bytes(*word))
            .collect()
    };
    let one = 1.0_f32.to_bits();
    let other_payload = snapshot(other.as_ref());
    for section in [&other_payload.1, &other_payload.2] {
        assert_eq!(
            words(section),
            [one, one, 0, 0, 0, 0, 0, 0],
            "a payload of another mode must leave the lane's own mode at rest"
        );
    }

    let mut target = prepare(LinkMode::Average).processor;
    let before = snapshot(target.as_ref());
    for (word, value) in [(0, 1.5_f32), (0, -0.25), (0, -0.0), (1, 0.5), (5, 0.5)] {
        let mut forged = payload.clone();
        forged.1[first + word * 4..first + word * 4 + 4].copy_from_slice(&value.to_le_bytes());
        assert_eq!(
            restore(target.as_mut(), &forged)
                .expect_err("a forged weight")
                .code,
            "effect.state.link",
            "word {word} = {value}"
        );
        assert_eq!(snapshot(target.as_ref()), before, "a refused restore wrote");
    }
    // Channels that name different modes are refused as well.
    let mut split = payload.clone();
    let maximum = link_glide_weights(LinkMode::Maximum);
    for (index, weight) in maximum.iter().enumerate() {
        let at = first + (index * 4 + 1) * 4;
        split.2[at..at + 4].copy_from_slice(&weight.to_le_bytes());
    }
    assert_eq!(
        restore(target.as_mut(), &split)
            .expect_err("channels apart")
            .code,
        "effect.state.link"
    );
    assert_eq!(snapshot(target.as_ref()), before, "a refused restore wrote");
}

/// **D1 refusals.** A glide longer than `LINK_GLIDE_MAX_SAMPLES`, a padded bank lane and track 1
/// of a scalar instance each return `false` and leave every payload word as it was.
#[test]
fn an_out_of_range_glide_is_refused_and_changes_nothing() {
    let values = oracle_values();
    let too_long = effect_contract::LINK_GLIDE_MAX_SAMPLES + 1;
    let scalar_request = request(&values, RATE, LinkMode::DualMono);
    let mut scalar = Under::new(None, core::slice::from_ref(&scalar_request));
    let before = scalar.link_words(0);
    assert!(
        !scalar.retarget(0, LinkMode::Maximum, too_long),
        "2^24 + 1 samples"
    );
    assert!(
        !scalar.retarget(1, LinkMode::Maximum, 64),
        "track 1 of a scalar instance"
    );
    assert_eq!(scalar.link_words(0), before);

    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let requests = vec![request(&values, RATE, LinkMode::DualMono); lanes];
        let mask: Vec<bool> = (0..lanes).map(|lane| lane == 0).collect();
        let bank = bind(width, &requests, &mask)
            .expect("a bank binds")
            .expect("one program");
        let mut bank = Under::Bank(bank.processor, width);
        let before = bank.link_words(0);
        assert!(
            !bank.retarget(0, LinkMode::Maximum, too_long),
            "2^24 + 1 samples"
        );
        assert!(!bank.retarget(1, LinkMode::Maximum, 64), "a padded lane");
        assert!(
            !bank.retarget(lanes, LinkMode::Maximum, 64),
            "a lane past the width"
        );
        assert_eq!(bank.link_words(0), before);
        // The bound itself is accepted.
        assert!(bank.retarget(0, LinkMode::Maximum, too_long - 1));
    }
}

/// Gate 1, the law's exact half: from the frame a glide arrives on, the lane's detector is its
/// target mode's prepared law bit for bit, and a lane that rests beside a glide takes its own
/// mode's word on every frame, non-finite partner words included.
#[test]
fn a_glide_lands_on_its_target_law_and_a_resting_lane_never_leaves_its_own() {
    fn check<L: Lane>() {
        const SAMPLES: u32 = 64;
        const FRAMES: u32 = 96;
        for from in LINKS {
            for to in LINKS.into_iter().filter(|mode| *mode != from) {
                let mut modes: [LinkMode; 8] = core::array::from_fn(|lane| LINKS[lane % 3]);
                modes[0] = from;
                // Each lane at rest at its mode, then lane 0 retargeted: its masks take the
                // target at once (`Shaper::retarget_link`), its weights glide.
                let mut ramps: [[LinearRamp; LINK_WEIGHTS]; 8] = core::array::from_fn(|lane| {
                    link_glide_weights(modes[lane]).map(LinearRamp::fixed)
                });
                modes[0] = to;
                for (ramp, weight) in ramps[0].iter_mut().zip(link_glide_weights(to)) {
                    ramp.set_target(weight, SAMPLES);
                }
                let lanes = LinkLanes::<L>::new(&modes[..L::WIDTH]);
                let gather = || {
                    [
                        LinkRamp::<L>::gather(&ramps, 0),
                        LinkRamp::<L>::gather(&ramps, 1),
                    ]
                };
                let (mut weights_left, mut weights_right) = (gather(), gather());
                for frame in 1..=FRAMES {
                    let words = |scale: f32, salt: u32| -> [f32; 8] {
                        core::array::from_fn(|lane| {
                            let hash = (frame.wrapping_mul(2_654_435_761)
                                ^ (lane as u32 + salt).wrapping_mul(40_503))
                                % 2_001;
                            scale * (hash as f32 / 1_000.0 - 1.0)
                        })
                    };
                    let mut partner = words(0.09, 7);
                    // Hostile right words on every third frame, the arrival frame included: a
                    // blend at rest weights equals the prepared law on finite words, but
                    // `0 * inf` and `inf - inf` are NaN, so only the `select` keeps a dual-mono
                    // lane clear of its partner's non-finite word.
                    if frame % 3 == SAMPLES % 3 {
                        for (lane, word) in partner.iter_mut().enumerate() {
                            *word = if (frame as usize + lane).is_multiple_of(2) {
                                f32::INFINITY
                            } else {
                                f32::NAN
                            };
                        }
                    }
                    let (own, partner) = (L::load(&words(0.9, 1)), L::load(&partner));
                    let law = link::<L, LINK_MIXED>(own, partner, &lanes);
                    let got = glide_link::<L, false>(
                        own,
                        partner,
                        &lanes,
                        &mut weights_left,
                        &mut weights_right,
                    );
                    let bits = |value: L| {
                        let mut out = [0_u32; 8];
                        value.store_bits(&mut out[..L::WIDTH]);
                        out
                    };
                    for (got, law) in [(got.0, law.0), (got.1, law.1)] {
                        let (got, law) = (bits(got), bits(law));
                        for lane in 0..L::WIDTH {
                            if lane == 0 && frame < SAMPLES {
                                continue;
                            }
                            assert_eq!(
                                got[lane],
                                law[lane],
                                "W{} {from:?} -> {to:?} frame {frame} lane {lane}: not the \
                                 prepared law's word",
                                L::WIDTH
                            );
                        }
                    }
                }
                let mut remaining = [0.0_f32; 8];
                for weight in weights_left.iter().chain(&weights_right) {
                    weight.remaining.store(&mut remaining);
                    assert!(
                        remaining[..L::WIDTH].iter().all(|word| *word == 0.0),
                        "the glide arrived"
                    );
                }
            }
        }
    }
    lane::each_lane!(|L| check::<L>());
}
