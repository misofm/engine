//! Issue #1370 (slice 1370b): a lane's peak link glides between `DualMono` and `Maximum` (D1-D6),
//! at every width the build has: the scalar instance, and banks bound at `Simd4` and `Simd8`.

use super::*;
use dsp_reference::{
    ReferenceLinkGlide, ReferenceTruePeakLimiter, ReferenceTruePeakLimiterParameters,
    reference_link_blend, reference_true_peak_estimate,
};
use effect_contract::{BankWidth, link_glide_weights};

const RATE: u32 = 48_000;
const BLOCK: usize = 48;
/// The glide: retargeted before the block that starts at `GLIDE_FRAME`, so it crosses a block
/// boundary.
const GLIDE_FRAME: usize = 2_400;
const FRAMES: usize = 9_600;
const CEILING: f32 = -1.0;
const RELEASE: f32 = 100.0;
const LOOKAHEAD: f32 = 5.0;
const MODES: [LinkMode; 2] = [LinkMode::DualMono, LinkMode::Maximum];
/// The rendered gain's bound against the `f64` model, in dB, on every output word at or above
/// `1e-3`: E5's `1e-4` absolute bound on a `+10 dBFS` source is about `0.001 dB`, and the glide
/// adds the rounding of the production weight's iterated additions.
const TOLERANCE_DB: f64 = 0.01;

/// One prepared limiter under test: the scalar instance at one lane, else a bank.
enum Under {
    Scalar(Box<dyn PreparedNativeEffect>),
    Bank(Box<dyn PreparedNativeEffectBank>, BankWidth),
}

impl Under {
    fn new(width: Option<BankWidth>, requests: &[PrepareEffectRequest<'_>]) -> Self {
        let Some(width) = width else {
            let prepared = TruePeakLimiterFactory
                .prepare(requests[0])
                .expect("prepare");
            return Self::Scalar(prepared.processor);
        };
        let bank = TruePeakLimiterFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: width.backend(),
                width,
                requests,
                active_mask: width.full_mask(),
            })
            .expect("bank binding")
            .expect("bank available");
        Self::Bank(bank.processor, width)
    }

    /// A padded bank: lane 0 the one member.
    fn padded(width: BankWidth, request: PrepareEffectRequest<'_>) -> Self {
        let lanes = width.lanes() as usize;
        let requests = vec![request; lanes];
        let mask: Vec<bool> = (0..lanes).map(|lane| lane == 0).collect();
        let bank = TruePeakLimiterFactory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: width.backend(),
                width,
                requests: &requests,
                active_mask: &mask,
            })
            .expect("bank binding")
            .expect("bank available");
        Self::Bank(bank.processor, width)
    }

    /// Track `lane`'s payload.
    fn payload(&self, lane: usize) -> LanePayload {
        let sizes = quality(RATE).maximum_state;
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
        (common, left, right)
    }

    /// Lane `lane`'s left-channel link weight words: `current, target, step, remaining`.
    fn link_words(&self, lane: usize) -> [u32; 4] {
        link_words(&self.payload(lane).1)
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
                    EffectProcessBlock::new(left, right, None, first_sample, flat, 128)
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
                        128,
                    )
                    .expect("a well-shaped bank block"),
                );
            }
        }
    }
}

/// The link weight's four words in one channel section.
fn link_words(section: &[u8]) -> [u32; 4] {
    core::array::from_fn(|word| read_u32(section, words::LINK_RAMP + word))
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

fn request_with(values: &[InitialParameterValue], link: LinkMode) -> PrepareEffectRequest<'_> {
    let mut request = request_at_rate(values, RATE);
    request.link_mode = link;
    request
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

/// Lane `lane` of a frame-major plane of `lanes` lanes.
fn lane_of(planes: &[f32], lanes: usize, lane: usize) -> Vec<f32> {
    (0..planes.len() / lanes)
        .map(|frame| planes[frame * lanes + lane])
        .collect()
}

/// The oracle's channels: bipolar noise at `+6 dBFS` on the left and at `-14 dBFS` on the right,
/// 20 dB apart. The right is under the limit alone and limited by a linked left.
fn oracle_signals() -> (Vec<f32>, Vec<f32>) {
    let mut noise = Noise(0x1370_b000);
    let left = (0..FRAMES).map(|_| noise.next() * 2.0).collect();
    let right = (0..FRAMES).map(|_| noise.next() * 0.2).collect();
    (left, right)
}

/// One glide case of the oracle test: its length and, when given, the frame of a live ceiling
/// and release change on every lane mid-glide.
#[derive(Clone, Copy, Debug)]
struct GlideCase {
    samples: u32,
    change: Option<usize>,
}

const CASES: [GlideCase; 2] = [
    GlideCase {
        samples: 64,
        change: None,
    },
    // Longer than the 64-update coefficient ramps, and crossed by both on every lane: a ceiling
    // (the limit coefficient) and a release (the release rate) retarget, which must neither cut
    // the glide nor its D5 countdown.
    GlideCase {
        samples: 4_800,
        change: Some(4_800),
    },
];

const CHANGED_CEILING: f32 = -3.0;
const CHANGED_RELEASE: f32 = 50.0;

/// The `f64` model of one lane: `from` until the glide, then the glide to `to` (a step glide to
/// `from` itself for a lane that rests), and the case's live change.
fn oracle_trace(
    from: LinkMode,
    to: LinkMode,
    case: GlideCase,
    left: &[f32],
    right: &[f32],
) -> (Vec<f64>, Vec<f64>) {
    let parameters = ReferenceTruePeakLimiterParameters {
        ceiling_db: f64::from(CEILING),
        release_ms: f64::from(RELEASE),
        lookahead_ms: f64::from(LOOKAHEAD),
    };
    let reference =
        || ReferenceTruePeakLimiter::new(f64::from(RATE), parameters).expect("reference");
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
        if case.change == Some(frame) {
            for model in [&mut model_left, &mut model_right] {
                model
                    .retarget(
                        f64::from(CHANGED_CEILING),
                        f64::from(CHANGED_RELEASE),
                        f64::from(RATE),
                        RAMP_UPDATES,
                    )
                    .expect("retarget");
            }
        }
        let (x_left, x_right) = (f64::from(left[frame]), f64::from(right[frame]));
        let (peak_left, peak_right) = (model_left.detect(x_left), model_right.detect(x_right));
        let (linked_left, linked_right) = reference_link_blend(peak_left, peak_right, in_force);
        out_left.push(model_left.apply(x_left, linked_left));
        out_right.push(model_right.apply(x_right, linked_right));
    }
    (out_left, out_right)
}

/// The worst deviation of one rendered channel from the model, in dB, on every model word at or
/// above `1e-3`.
fn deviation(rendered: &[f32], model: &[f64]) -> f64 {
    rendered
        .iter()
        .zip(model)
        .filter(|(_, model)| model.abs() >= 1.0e-3)
        .map(|(rendered, model)| (20.0 * math::log10((f64::from(*rendered) / model).abs())).abs())
        .fold(0.0, f64::max)
}

/// The live change's spans, both channels, on every lane at `first_sample`.
fn change_spans(lanes: usize, first_sample: u64) -> Vec<Vec<PreparedAutomationSpan>> {
    let (left, right) = (ParameterChannel::Left, ParameterChannel::Right);
    vec![
        vec![
            point_span(first_sample, 0, left, CHANGED_CEILING),
            point_span(first_sample, 0, right, CHANGED_CEILING),
            point_span(first_sample, 1, left, CHANGED_RELEASE),
            point_span(first_sample, 1, right, CHANGED_RELEASE),
        ];
        lanes
    ]
}

/// **Gate 1.** Each glide direction, over 64 samples and over 4,800 samples crossed by a live
/// ceiling and release change on every lane (the gliding lane and its bank-mates), each glide
/// crossing a block boundary, on two channels 20 dB apart: every lane's output stays within
/// `0.01 dB` of the `f64` limiter whose peak is the reference blend, and after it the target law,
/// while the bank-mates rest at their own modes; and the glide arrives exactly (its weight at the
/// target, no countdown left). At the scalar instance, `Simd4` and `Simd8`.
#[test]
fn a_glide_tracks_the_f64_blend_into_the_target_law() {
    let (source_left, source_right) = oracle_signals();
    let values = values_with(CEILING, RELEASE, LOOKAHEAD);
    for case in CASES {
        for width in widths() {
            let lanes = lanes(width);
            for (from, to) in [MODES, [MODES[1], MODES[0]]].map(|[a, b]| (a, b)) {
                let context = format!("{width:?} {case:?} {from:?} -> {to:?}");
                let modes: Vec<LinkMode> = (0..lanes)
                    .map(|lane| if lane == 0 { from } else { MODES[lane % 2] })
                    .collect();
                let requests: Vec<_> = modes
                    .iter()
                    .map(|mode| request_with(&values, *mode))
                    .collect();
                let mut under = Under::new(width, &requests);
                let mut left = interleave(&vec![source_left.clone(); lanes]);
                let mut right = interleave(&vec![source_right.clone(); lanes]);
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
                let rest = link_glide_weights(to)[0].to_bits();
                assert_eq!(
                    under.link_words(0),
                    [rest, rest, 0, 0],
                    "{context}: the glide did not arrive exactly"
                );
                for (lane, mode) in modes.iter().enumerate() {
                    let target = if lane == 0 { to } else { *mode };
                    let (model_left, model_right) =
                        oracle_trace(*mode, target, case, &source_left, &source_right);
                    let worst = deviation(&lane_of(&left, lanes, lane), &model_left)
                        .max(deviation(&lane_of(&right, lanes, lane), &model_right));
                    assert!(
                        worst <= TOLERANCE_DB,
                        "{context} lane {lane}: worst deviation {worst:e} dB"
                    );
                }
                // Not vacuous: the quiet channel's model output moves with the glide.
                let (_, glided) = oracle_trace(from, to, case, &source_left, &source_right);
                let (_, rested) = oracle_trace(from, from, case, &source_left, &source_right);
                let moved = glided
                    .iter()
                    .zip(&rested)
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0, f64::max);
                assert!(
                    moved > 0.01,
                    "{context}: the glide changed nothing ({moved})"
                );
            }
        }
    }
}

/// The ceiling fixture: a near-Nyquist sine, the inter-sample overshoot generator, at `+3 dBFS`
/// on the left and `-17 dBFS` on the right, in gated bursts so the ramp re-arrives at the
/// requirement again and again.
fn ceiling_signals() -> (Vec<f32>, Vec<f32>) {
    let step = core::f64::consts::TAU * 0.49;
    let burst = |frame: usize| if (frame / 600) % 4 == 3 { 0.0 } else { 1.0 };
    let left = (0..FRAMES)
        .map(|frame| (burst(frame) * math::sin(step * frame as f64) * 1.4125) as f32)
        .collect();
    let right = (0..FRAMES)
        .map(|frame| (burst(frame) * math::sin(step * frame as f64 + 0.37) * 0.1413) as f32)
        .collect();
    (left, right)
}

/// **Gate 3 (D4).** Every switch direction at every ramp length in `{0, 1, 64, 4800}`, at the
/// scalar instance, `Simd4` and `Simd8`, on a full-scale fixture with inter-sample peaks whose
/// channels sit 20 dB apart: no output's true-peak estimate (the `f64` Annex-2 reference) exceeds
/// the ceiling, on the switching lane or a bank-mate, on either channel.
#[test]
fn no_switch_puts_a_sample_over_the_ceiling() {
    let (source_left, source_right) = ceiling_signals();
    let values = values_with(CEILING, RELEASE, 0.0);
    let ceiling = math::db_to_gain(f64::from(CEILING));
    let mut worst = f64::NEG_INFINITY;
    for width in widths() {
        let lanes = lanes(width);
        for (from, to) in [MODES, [MODES[1], MODES[0]]].map(|[a, b]| (a, b)) {
            for samples in [0_u32, 1, 64, 4_800] {
                let context = format!("{width:?} {from:?} -> {to:?} over {samples}");
                let requests: Vec<_> = (0..lanes)
                    .map(|lane| {
                        request_with(&values, if lane == 0 { from } else { MODES[lane % 2] })
                    })
                    .collect();
                let mut under = Under::new(width, &requests);
                let mut left = interleave(&vec![source_left.clone(); lanes]);
                let mut right = interleave(&vec![source_right.clone(); lanes]);
                let offsets = vec![0_u32; lanes + 1];
                for block in 0..FRAMES / BLOCK {
                    let first = block * BLOCK;
                    if first == GLIDE_FRAME {
                        assert!(under.retarget(0, to, samples), "a supported switch");
                    }
                    let range = first * lanes..(first + BLOCK) * lanes;
                    under.process(
                        &mut left[range.clone()],
                        &mut right[range],
                        first as u64,
                        &[],
                        &offsets,
                    );
                }
                for lane in 0..lanes {
                    for (side, plane) in [("left", &left), ("right", &right)] {
                        let measured: Vec<f64> = lane_of(plane, lanes, lane)
                            .iter()
                            .map(|word| f64::from(*word))
                            .collect();
                        let estimate =
                            reference_true_peak_estimate(&measured).expect("finite output");
                        let margin = 20.0 * math::log10(estimate / ceiling);
                        worst = worst.max(margin);
                        assert!(
                            estimate <= ceiling,
                            "{context} lane {lane} {side}: estimate {estimate} over the ceiling \
                             {ceiling} by {margin} dB"
                        );
                    }
                }
            }
        }
    }
    println!("worst true-peak margin over every switch: {worst:+.4} dB");
}

/// **Gates 2 and 6.** A full bank of distinct members at mixed modes, where lane 0 glides while
/// its bank-mates rest or ramp the ceiling and the release: every other lane renders bit for bit
/// as its own scalar instance, and the retarget and the switching blocks allocate and free
/// nothing. Half the seeds share one lookahead, so the uniform body renders the bank-mates as
/// well as the per-lane one. At `Simd4` and `Simd8`.
#[test]
fn a_glide_moves_no_bank_mates_bit_and_allocates_nothing() {
    bench_support::alloc::assert_installed();
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        for seed in 0..6_u64 {
            let mut noise = Noise(0x1370_b002 + seed);
            let members: Vec<_> = (0..lanes)
                .map(|lane| {
                    let lookahead = if seed % 2 == 0 {
                        LOOKAHEAD
                    } else {
                        lane as f32
                    };
                    values_with(-1.0 - lane as f32, 50.0 + 40.0 * lane as f32, lookahead)
                })
                .collect();
            let modes: Vec<LinkMode> = (0..lanes)
                .map(|lane| MODES[(lane + seed as usize) % 2])
                .collect();
            let target = MODES[(seed as usize + 1) % 2];
            let requests: Vec<_> = members
                .iter()
                .zip(&modes)
                .map(|(values, mode)| request_with(values, *mode))
                .collect();
            let mut bank = Under::new(Some(width), &requests);
            let mut scalars: Vec<Under> = requests
                .iter()
                .map(|request| Under::new(None, core::slice::from_ref(request)))
                .collect();
            let glide_block = 4;
            for block in 0..24 {
                let first_sample = (block * BLOCK) as u64;
                let inputs: Vec<(Vec<f32>, Vec<f32>)> = (0..lanes)
                    .map(|_| {
                        let left = (0..BLOCK).map(|_| noise.next() * 2.0).collect();
                        let right = (0..BLOCK).map(|_| noise.next() * 0.2).collect();
                        (left, right)
                    })
                    .collect();
                let spans: Vec<Vec<PreparedAutomationSpan>> = (0..lanes)
                    .map(|lane| {
                        if lane == 0 || (block + lane) % 3 != 0 {
                            return Vec::new();
                        }
                        let ceiling = -2.0 - (block % 5) as f32;
                        let release = 20.0 + 30.0 * (block % 4) as f32;
                        vec![
                            point_span(first_sample, 0, ParameterChannel::Left, ceiling),
                            point_span(first_sample, 1, ParameterChannel::Right, release),
                        ]
                    })
                    .collect();
                let (flat, offsets) = flatten(&spans, first_sample);
                let mut left =
                    interleave(&inputs.iter().map(|(l, _)| l.clone()).collect::<Vec<_>>());
                let mut right =
                    interleave(&inputs.iter().map(|(_, r)| r.clone()).collect::<Vec<_>>());
                let mark = bench_support::alloc::current_thread_counters();
                if block == glide_block {
                    assert!(bank.retarget(0, target, 300), "a supported glide");
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
/// another mode leaves that mode at rest, and a forged weight outside `[0, 1]`, a `-0.0`, a
/// target that is no supported mode's, or channels that name different modes, is refused and
/// changes nothing.
#[test]
fn a_lane_snapshotted_mid_glide_continues_bit_for_bit_and_forged_weights_are_refused() {
    let values = values_with(CEILING, RELEASE, LOOKAHEAD);
    let prepare = |mode| Under::new(None, &[request_with(&values, mode)]);
    let sizes = quality(RATE).maximum_state;
    let restore = |under: &mut Under, payload: &LanePayload| {
        let Under::Scalar(effect) = under else {
            unreachable!("a scalar instance")
        };
        effect.restore_state_payload(
            STATE_LAYOUT_VERSION,
            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("sizes"),
        )
    };
    let (input_left, input_right) = oracle_signals();
    let render = |under: &mut Under, range: core::ops::Range<usize>| {
        let (mut left, mut right) = (
            input_left[range.clone()].to_vec(),
            input_right[range.clone()].to_vec(),
        );
        for (block, (left, right)) in left.chunks_mut(128).zip(right.chunks_mut(128)).enumerate() {
            under.process(
                left,
                right,
                (range.start + block * 128) as u64,
                &[],
                &[0, 0],
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
    let mut source = prepare(LinkMode::DualMono);
    render(&mut source, 0..1_024);
    assert!(source.retarget(0, LinkMode::Maximum, 96));
    render(&mut source, 1_024..1_044);
    let payload = source.payload(0);

    let mut successor = prepare(LinkMode::Maximum);
    restore(&mut successor, &payload).expect("a mid-glide payload restores");
    for range in [1_044..1_100, 1_100..2_400] {
        assert_eq!(
            render(&mut successor, range.clone()),
            render(&mut source, range.clone()),
            "frames {range:?}: the restored glide parted from the uninterrupted one"
        );
    }

    // Under another mode the lane keeps its own, at rest: `DualMono`'s weight `0`. The lane is
    // itself mid-glide to `DualMono`: the restore replaces that glide too.
    let mut other = prepare(LinkMode::Maximum);
    assert!(other.retarget(0, LinkMode::DualMono, 96));
    render(&mut other, 0..20);
    restore(&mut other, &payload).expect("restores under another mode");
    let other_payload = other.payload(0);
    for section in [&other_payload.1, &other_payload.2] {
        assert_eq!(
            link_words(section),
            [0, 0, 0, 0],
            "a payload of another mode must leave the lane's own mode at rest"
        );
    }

    let mut target = prepare(LinkMode::Maximum);
    let before = target.payload(0);
    let first = words::LINK_RAMP * 4;
    for (word, value) in [(0, 1.5_f32), (0, -0.25), (0, -0.0), (1, 0.5)] {
        let mut forged = payload.clone();
        forged.1[first + word * 4..first + word * 4 + 4].copy_from_slice(&value.to_le_bytes());
        assert_eq!(
            restore(&mut target, &forged)
                .expect_err("a forged weight")
                .code,
            "effect.state.link",
            "word {word} = {value}"
        );
        assert_eq!(target.payload(0), before, "a refused restore wrote");
    }
    // Channels that name different modes are refused as well.
    let mut split = payload.clone();
    split.2[first + 4..first + 8].copy_from_slice(&0.0_f32.to_le_bytes());
    assert_eq!(
        restore(&mut target, &split)
            .expect_err("channels apart")
            .code,
        "effect.state.link"
    );
    assert_eq!(target.payload(0), before, "a refused restore wrote");
}

/// **D1 refusals.** `Average`, which the limiter does not support, a glide longer than
/// `LINK_GLIDE_MAX_SAMPLES`, a padded bank lane, a lane past the width and track 1 of a scalar
/// instance each return `false` and leave every payload word as it was.
#[test]
fn an_unsupported_or_out_of_range_glide_is_refused_and_changes_nothing() {
    let values = values_with(CEILING, RELEASE, LOOKAHEAD);
    let too_long = effect_contract::LINK_GLIDE_MAX_SAMPLES + 1;
    let mut scalar = Under::new(None, &[request_with(&values, LinkMode::DualMono)]);
    let before = scalar.payload(0);
    assert!(!scalar.retarget(0, LinkMode::Average, 64), "average");
    assert!(
        !scalar.retarget(0, LinkMode::Maximum, too_long),
        "2^24 + 1 samples"
    );
    assert!(
        !scalar.retarget(1, LinkMode::Maximum, 64),
        "track 1 of a scalar instance"
    );
    assert_eq!(scalar.payload(0), before);

    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let mut bank = Under::padded(width, request_with(&values, LinkMode::DualMono));
        let before = bank.payload(0);
        assert!(!bank.retarget(0, LinkMode::Average, 64), "average");
        assert!(
            !bank.retarget(0, LinkMode::Maximum, too_long),
            "2^24 + 1 samples"
        );
        assert!(!bank.retarget(1, LinkMode::Maximum, 64), "a padded lane");
        assert!(
            !bank.retarget(lanes, LinkMode::Maximum, 64),
            "a lane past the width"
        );
        assert_eq!(bank.payload(0), before);
        // The bound itself is accepted.
        assert!(bank.retarget(0, LinkMode::Maximum, too_long - 1));
    }
}

/// Gate 1, the law's exact half: from the frame a glide arrives on, the lane's peak is its target
/// mode's law bit for bit, and a lane that rests beside a glide takes its own mode's word on
/// every frame, non-finite partner peaks included.
#[test]
fn a_glide_lands_on_its_target_law_and_a_resting_lane_never_leaves_its_own() {
    fn check<L: Lane>() {
        const SAMPLES: u32 = 64;
        const FRAMES: u32 = 96;
        for (from, to) in [MODES, [MODES[1], MODES[0]]].map(|[a, b]| (a, b)) {
            let mut coef = LimiterCoef::<L>::new(LinkMode::DualMono, false);
            let mut ramps = [LinearRamp::fixed(0.0); MAXIMUM_WIDTH];
            for lane in 0..L::WIDTH {
                let mode = if lane == 0 { from } else { MODES[lane % 2] };
                coef.set_lane_link(lane, mode, every_lane(L::WIDTH));
                ramps[lane] = LinearRamp::fixed(link_glide_weights(mode)[0]);
            }
            // Lane 0 retargeted: its mask takes the target at once (`retarget_link`), its weight
            // glides.
            coef.set_lane_link(0, to, every_lane(L::WIDTH));
            ramps[0].set_target(link_glide_weights(to)[0], SAMPLES);
            let mut weight = RampLanes::<L>::gather_link(&ramps[..L::WIDTH]);
            for frame in 1..=FRAMES {
                let peaks = |scale: f32, salt: u32| -> [f32; MAXIMUM_WIDTH] {
                    core::array::from_fn(|lane| {
                        let hash = (frame.wrapping_mul(2_654_435_761)
                            ^ (lane as u32 + salt).wrapping_mul(40_503))
                            % 2_001;
                        scale * hash as f32 / 1_000.0
                    })
                };
                let mut partner = peaks(0.09, 7);
                // Hostile partner peaks on every third frame, the arrival frame included: a blend
                // at rest weights equals the prepared law on finite words, but `0 * inf` and
                // `inf - inf` are NaN, so only the `select` keeps a dual-mono lane clear.
                if frame % 3 == SAMPLES % 3 {
                    for (lane, word) in partner.iter_mut().enumerate() {
                        *word = if (frame as usize + lane).is_multiple_of(2) {
                            f32::INFINITY
                        } else {
                            f32::NAN
                        };
                    }
                }
                let own = L::load(&peaks(1.4, 1));
                let partner = L::load(&partner);
                let linked = partner.max(own);
                let law = L::select(coef.link, linked, own);
                let got = glide_peak(own, linked, law, &mut weight);
                let bits = |value: L| {
                    let mut out = [0_u32; MAXIMUM_WIDTH];
                    value.store_bits(&mut out[..L::WIDTH]);
                    out
                };
                let (got, law) = (bits(got), bits(law));
                for lane in 0..L::WIDTH {
                    if lane == 0 && frame < SAMPLES {
                        continue;
                    }
                    assert_eq!(
                        got[lane],
                        law[lane],
                        "W{} {from:?} -> {to:?} frame {frame} lane {lane}: not the prepared \
                         law's peak",
                        L::WIDTH
                    );
                }
            }
            let mut remaining = [0.0_f32; MAXIMUM_WIDTH];
            weight.remaining.store(&mut remaining);
            assert!(
                remaining[..L::WIDTH].iter().all(|word| *word == 0.0),
                "the glide arrived"
            );
        }
    }
    lane::each_lane!(|L| check::<L>());
}

/// **The collapsed body.** A core whose lane 0 glides renders collapsed blocks
/// (`process_block_mono`) that are the left plane of a core fed `L == R`, bit for bit, and after
/// the disengage copy the two cores render and hold the same words: a collapsed block advances
/// the glide as the dual one does. At every width the build has.
///
/// And a glide in flight is never quiet: on a core holding the silent rest claim, a glide
/// started on `+0.0` input moves its weights on every block, dual and collapsed, as a twin whose
/// claim is withdrawn before every block (so it renders every block), and arrives on its frame.
#[test]
fn a_collapsed_block_advances_a_glide_as_the_dual_block_on_equal_channels() {
    fn silent<L: Lane>(mono: bool) {
        const GLIDE: u32 = 300;
        let tracks = vec![values_with(CEILING, RELEASE, LOOKAHEAD); L::WIDTH];
        let core = || linked_core::<L>(&tracks, LinkMode::DualMono, false, RATE);
        let (mut fast, mut slow) = (core(), core());
        let render = |core: &mut LimiterCore<L>, withdraw: bool| {
            if withdraw {
                core.silent_fixed_point = false;
            }
            let mut left = vec![0.0_f32; BLOCK * L::WIDTH];
            if mono {
                core.process_block_mono(&mut left, BLOCK);
            } else {
                let mut right = left.clone();
                core.process_block(&mut left, &mut right, BLOCK);
            }
        };
        let link_bits = |core: &LimiterCore<L>| -> Vec<u32> {
            let channels = if mono {
                vec![&core.left]
            } else {
                vec![&core.left, &core.right]
            };
            channels
                .into_iter()
                .flat_map(|channel| channel.link[..L::WIDTH].iter())
                .flat_map(|ramp| {
                    [
                        ramp.current.to_bits(),
                        ramp.target.to_bits(),
                        ramp.step.to_bits(),
                        ramp.remaining,
                    ]
                })
                .collect()
        };
        for _ in 0..4 {
            render(&mut fast, false);
            render(&mut slow, true);
        }
        assert!(
            fast.silent_engagements() > 0,
            "the rest claim holds before the glide"
        );
        for core in [&mut fast, &mut slow] {
            assert!(core.retarget_link(0, LinkMode::Maximum, GLIDE));
        }
        for block in 1..=8_u32 {
            render(&mut fast, false);
            render(&mut slow, true);
            let context = format!("W{} mono {mono} block {block}", L::WIDTH);
            assert_eq!(
                link_bits(&fast),
                link_bits(&slow),
                "{context}: a skipped glide"
            );
            assert_eq!(
                fast.left.link[0].remaining,
                GLIDE.saturating_sub(block * BLOCK as u32),
                "{context}: the glide's countdown"
            );
        }
        assert_eq!(
            fast.left.link[0],
            LinearRamp::fixed(1.0),
            "the glide arrived"
        );
    }
    fn check<L: Lane>() {
        let tracks = vec![values_with(CEILING, RELEASE, LOOKAHEAD); L::WIDTH];
        let core = || linked_core::<L>(&tracks, LinkMode::DualMono, false, RATE);
        let (mut dual, mut collapsed) = (core(), core());
        for core in [&mut dual, &mut collapsed] {
            assert!(core.retarget_link(0, LinkMode::Maximum, 300));
        }
        let mut noise = Noise(0x1370_b005);
        for block in 0..16 {
            let plane: Vec<f32> = (0..BLOCK * L::WIDTH).map(|_| noise.next() * 2.0).collect();
            let (mut dual_left, mut dual_right) = (plane.clone(), plane.clone());
            dual.process_block(&mut dual_left, &mut dual_right, BLOCK);
            let mut mono_left = plane.clone();
            if block < 4 {
                collapsed.process_block_mono(&mut mono_left, BLOCK);
                assert_same_words(
                    &mono_left,
                    &dual_left,
                    &format!("W{} block {block}: a collapsed glide block", L::WIDTH),
                );
                if block == 3 {
                    collapsed.desymmetrize();
                    assert_eq!(state_bits(&collapsed), state_bits(&dual), "disengaged");
                }
                continue;
            }
            let mut mono_right = plane;
            collapsed.process_block(&mut mono_left, &mut mono_right, BLOCK);
            assert_same_words(&mono_left, &dual_left, &format!("block {block}: left"));
            assert_same_words(&mono_right, &dual_right, &format!("block {block}: right"));
        }
        assert_eq!(
            state_bits(&collapsed),
            state_bits(&dual),
            "the glide arrived alike"
        );
        silent::<L>(false);
        silent::<L>(true);
    }
    lane::each_lane!(|L| check::<L>());
}

/// **Gate 4 (D5, as ruled 2026-10-25).** On a uniform bank at `DualMono`, on `+6 dBFS`
/// stationary noise on both channels with no burst, lane 0 switches into `Maximum` over 4,800
/// samples; while it glides every bank-mate switches over 64 (a countdown written without `max`
/// would end inside lane 0's glide), and a NaN on every lane fails the §4.4 check and resets the
/// whole bank, which states the record from the cleared words while the glide continues (the
/// glide's blocks must clear it again). Then:
///
/// * no block takes the #990 linked pair before lane 0's glide plus `N + R` frames;
/// * the record returns within `WATCH_BOUND` frames of that, through the per-block watch of the
///   `W` reduction words (`d` can stay apart past the countdown on stationary material: a single
///   check at its end never relinked here), and the pair links on every block after;
/// * every block's output and state equal the reference kernel's, which never links, and the
///   record never stands over disagreeing gain words (`LinkedPair::compare`).
///
/// At every width the build has.
#[test]
fn the_linked_pair_returns_once_the_words_agree() {
    fn check<L: Lane>() {
        const LONG: u32 = 4_800;
        const SHORT: u32 = 64;
        const SWITCH: usize = 10;
        const MATES: usize = 20;
        /// Frame 1,440: its NaN leaves the delay line at about frame 1,926, inside lane 0's glide.
        const POISON: usize = 30;
        const BLOCKS: usize = 300;
        /// 100 ms at 48 kHz. Measured on this fixture: the pair returned 47 (W1), 47 (W4) and
        /// 1,247 (W8) frames after the countdown.
        const WATCH_BOUND: usize = 4_800;
        let tracks = vec![values_with(CEILING, RELEASE, LOOKAHEAD); L::WIDTH];
        let label = format!("W{}", L::WIDTH);
        let mut pair = LinkedPair::<L>::new(&tracks, LinkMode::DualMono, false, RATE, label);
        let shape = Shape::new(RATE).expect("a shape");
        let countdown = SWITCH * BLOCK + LONG as usize + shape.n + shape.ring;
        let mut noise = Noise(0x1370_b006);
        let mut first_engaged = None;
        for block in 0..BLOCKS {
            if block == SWITCH {
                for core in [&mut pair.shipped, &mut pair.oracle] {
                    assert!(core.retarget_link(0, LinkMode::Maximum, LONG));
                }
            }
            if block == MATES {
                for core in [&mut pair.shipped, &mut pair.oracle] {
                    for lane in 1..L::WIDTH {
                        assert!(core.retarget_link(lane, LinkMode::Maximum, SHORT));
                    }
                }
            }
            let words = BLOCK * L::WIDTH;
            let mut left: Vec<f32> = (0..words).map(|_| noise.next() * 2.0).collect();
            let right: Vec<f32> = (0..words).map(|_| noise.next() * 2.0).collect();
            if block == POISON {
                left[..L::WIDTH].fill(f32::NAN);
            }
            let engaged = pair.dual(
                &left,
                &right,
                &no_spans(L::WIDTH),
                &format!("block {block}"),
            );
            if engaged {
                assert!(
                    block * BLOCK >= countdown,
                    "W{} block {block}: linked before the rings were rewritten",
                    L::WIDTH
                );
                first_engaged.get_or_insert(block);
            } else if let Some(first) = first_engaged {
                panic!(
                    "W{} block {block}: the linked pair left again after block {first}",
                    L::WIDTH
                );
            }
        }
        let first = first_engaged.expect("the linked pair never returned") * BLOCK;
        println!(
            "W{}: the linked pair returned {} frames after the countdown",
            L::WIDTH,
            first - countdown
        );
        assert!(
            first <= countdown + WATCH_BOUND,
            "W{}: returned at frame {first}, past the bound",
            L::WIDTH
        );
    }
    lane::each_lane!(|L| check::<L>());
}
