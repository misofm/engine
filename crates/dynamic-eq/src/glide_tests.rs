//! Issue #1969 (the #1370 shape): a lane's detector link glides between modes (D1-D3, D6), at
//! every width the build has: the scalar instance, and banks bound at `Simd4` and `Simd8` (the
//! width the build does not execute binds through `bind_bank::<false>`).
//!
//! The law's exact half (the arrival frame and a resting lane's level) is
//! `kernel::glide_law_tests`.

use super::*;
use dsp_reference::{
    ReferenceDynamicBandLaw, ReferenceDynamicEqBand, ReferenceLinkGlide, reference_link_blend,
};
use effect_contract::{
    PrepareEffectLimits, PreparedPorts, RegisteredTailBound, link_glide_weights,
};

const RATE: u32 = 48_000;
const QUANTUM: u32 = 128;
const BLOCK: usize = 48;
const LINKS: [LinkMode; 3] = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average];
/// The glide: retargeted before the block that starts at `GLIDE_FRAME`, so it crosses a block
/// boundary.
const GLIDE_FRAME: usize = 2_400;
const FRAMES: usize = 9_600;

type Values = Vec<InitialParameterValue>;

fn tail_bound() -> RegisteredTailBound {
    static BOUND: std::sync::OnceLock<RegisteredTailBound> = std::sync::OnceLock::new();
    *BOUND.get_or_init(|| {
        conformance::tail_bound_of(Box::new(DynamicEqFactory), RATE, EffectQuality::Normal)
    })
}

fn request(values: &[InitialParameterValue], link: LinkMode) -> PrepareEffectRequest<'_> {
    let quality = &QUALITIES[1];
    PrepareEffectRequest {
        sample_rate: RATE,
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
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 32,
        },
        tail_bound: tail_bound(),
    }
}

/// The descriptor defaults with `changes` (parameter index, value) on both channels.
fn values(changes: &[(usize, f32)]) -> Values {
    (0..PARAMETER_COUNT * 2)
        .map(|index| {
            let parameter = index / 2;
            let value = changes
                .iter()
                .rev()
                .find(|(changed, _)| *changed == parameter)
                .map_or(DYNAMIC_EQ_PARAMETERS[parameter].default_value, |(_, v)| *v);
            InitialParameterValue {
                parameter_index: parameter as u32,
                channel: if index % 2 == 0 {
                    ParameterChannel::Left
                } else {
                    ParameterChannel::Right
                },
                value,
            }
        })
        .collect()
}

/// Binds a bank at `width` through `bind_bank::<false>`, which serves every width this host has.
fn bind(
    width: BankWidth,
    requests: &[PrepareEffectRequest<'_>],
    mask: &[bool],
) -> Box<dyn PreparedNativeEffectBank> {
    bind_bank::<false>(PrepareEffectBankRequest {
        backend: width.backend(),
        width,
        requests,
        active_mask: mask,
    })
    .expect("a bank binds")
    .expect("one program")
    .processor
}

/// One prepared dynamic EQ under test: the scalar instance at one lane, else a bank.
enum Under {
    Scalar(Box<dyn PreparedNativeEffect>),
    Bank(Box<dyn PreparedNativeEffectBank>, BankWidth),
}

impl Under {
    fn new(width: Option<BankWidth>, requests: &[PrepareEffectRequest<'_>]) -> Self {
        let Some(width) = width else {
            let prepared = DynamicEqFactory.prepare(requests[0]).expect("prepare");
            return Self::Scalar(prepared.processor);
        };
        Self::Bank(bind(width, requests, &vec![true; requests.len()]), width)
    }

    /// Lane `lane`'s payload sections, `(common, left, right)`.
    fn snapshot(&self, lane: usize) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let sizes = QUALITIES[1].maximum_state;
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

    /// Lane `lane`'s link weight words of both channels in its payload: `w_link` then `w_avg`,
    /// each `current, target, step, remaining`, left then right.
    fn link_words(&self, lane: usize) -> [u32; 16] {
        let (_, left, right) = self.snapshot(lane);
        core::array::from_fn(|word| {
            let section = if word < 8 { &left } else { &right };
            let at = (STATE_LINK_WORD + word % 8) * 4;
            u32::from_le_bytes(section[at..at + 4].try_into().expect("a word"))
        })
    }

    fn retarget(&mut self, lane: usize, mode: LinkMode, samples: u32) -> bool {
        match self {
            Self::Scalar(effect) => effect.retarget_link(lane, mode, samples),
            Self::Bank(bank, _) => bank.retarget_link(lane, mode, samples),
        }
    }

    fn lanes_at_rest(&self) -> u32 {
        match self {
            Self::Scalar(_) => unreachable!("a bank call"),
            Self::Bank(bank, _) => bank.lanes_at_rest(),
        }
    }

    /// One block of frame-major planes, with the flattened spans `flat` at `offsets`; `mono`
    /// renders a bank collapsed.
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        first_sample: u64,
        (flat, offsets): (&[PreparedAutomationSpan], &[u32]),
        mono: bool,
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
                let block = EffectBankProcessBlock::new(
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
                .expect("a well-shaped bank block");
                if mono {
                    bank.process_bank_mono(block);
                } else {
                    bank.process_bank(block);
                }
            }
        }
    }
}

/// The settled link words of both channels at `mode`.
fn rest_words(mode: LinkMode) -> [u32; 16] {
    let rest = link_glide_weights(mode).map(f32::to_bits);
    let channel = [rest[0], rest[0], 0, 0, rest[1], rest[1], 0, 0];
    core::array::from_fn(|word| channel[word % 8])
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

/// One parameter at `value` on both channels.
fn both(parameter: u32, value: f32) -> [PreparedAutomationSpan; 2] {
    [ParameterChannel::Left, ParameterChannel::Right].map(|channel| PreparedAutomationSpan {
        channel,
        parameter_index: parameter,
        sample: 0,
        value,
    })
}

const THRESHOLD: f32 = -30.0;
const RANGE: f32 = 18.0;
/// The range a span moves to mid-glide: the model's deepest change stays well above `-RANGE`,
/// so the move changes no model word, and only a glide the move cuts parts from it.
const CHANGED_RANGE: f32 = 24.0;
/// Band 0's range parameter; bands 1 to 3 are disabled.
const RANGE_PARAMETER: u32 = 6;

/// Band 0 alone: 1 kHz, Q 1, a 0 dB static bell, threshold -30 dB, ratio 4, a hard knee, range
/// 18 dB, attack 1 ms, release 50 ms, downward, its own detector.
fn oracle_values() -> Values {
    values(&[
        (1, 1_000.0),
        (2, 1.0),
        (3, 0.0),
        (4, THRESHOLD),
        (5, 4.0),
        (6, RANGE),
        (7, 1.0),
        (8, 50.0),
        (11, 0.0),
        (12, 0.0),
        (24, 0.0),
        (36, 0.0),
    ])
}

/// 1 kHz tones at the band's centre, 20 dB apart: the left at -14 dBFS, cut by about 12 dB,
/// and the right at -34 dBFS, under the threshold alone and cut by either link.
fn oracle_signals() -> (Vec<f32>, Vec<f32>) {
    let tone = |amplitude: f64| -> Vec<f32> {
        (0..FRAMES)
            .map(|n| {
                let phase = core::f64::consts::TAU * 1_000.0 * n as f64 / f64::from(RATE);
                (amplitude * math::sin(phase)) as f32
            })
            .collect()
    };
    (tone(0.2), tone(0.02))
}

/// One glide case: its length and the frame of the range span on every lane mid-glide.
#[derive(Clone, Copy, Debug)]
struct GlideCase {
    samples: u32,
    change: usize,
}

const CASES: [GlideCase; 2] = [
    GlideCase {
        samples: 64,
        change: GLIDE_FRAME + BLOCK,
    },
    // Longer than the 64-sample parameter ramp, and crossed by one: the bank's ramp clock must
    // keep covering the glide when the range span reloads it.
    GlideCase {
        samples: 4_800,
        change: 4_800,
    },
];

/// The `f64` model of one lane, both channels: `from` until the glide, then the glide to `to`
/// (a step to `from` itself for a lane that rests). Returns the outputs and the deepest change.
fn oracle_trace(
    from: LinkMode,
    to: LinkMode,
    samples: u32,
    left: &[f32],
    right: &[f32],
) -> (Vec<f64>, Vec<f64>, f64) {
    let law = ReferenceDynamicBandLaw {
        threshold_db: f64::from(THRESHOLD),
        ratio: 4.0,
        knee_db: 0.0,
        range_db: f64::from(RANGE),
        attack_ms: 1.0,
        release_ms: 50.0,
        upward: false,
    };
    let channel = || {
        ReferenceDynamicEqBand::new(f64::from(RATE), 1_000.0, 1.0, 0.0, law, false).expect("design")
    };
    let (mut model_left, mut model_right) = (channel(), channel());
    let weights = |mode: LinkMode| link_glide_weights(mode).map(f64::from);
    let mut glide = ReferenceLinkGlide::new(weights(from), weights(from), 0);
    let (mut out_left, mut out_right, mut deepest) = (Vec::new(), Vec::new(), 0.0_f64);
    for frame in 0..FRAMES {
        if frame == GLIDE_FRAME {
            let samples = if from == to { 0 } else { samples };
            glide = ReferenceLinkGlide::new(weights(from), weights(to), samples);
        }
        let (x_left, x_right) = (f64::from(left[frame]), f64::from(right[frame]));
        let (band_left, tap_left) = model_left.taps(x_left, 0.0);
        let (band_right, tap_right) = model_right.taps(x_right, 0.0);
        let (w, _) = glide.next_weights();
        let (d_left, d_right) = reference_link_blend(tap_left, tap_right, w);
        let (y_left, change_left) = model_left.finish(x_left, band_left, d_left);
        let (y_right, change_right) = model_right.finish(x_right, band_right, d_right);
        out_left.push(y_left);
        out_right.push(y_right);
        deepest = deepest.min(change_left).min(change_right);
    }
    (out_left, out_right, deepest)
}

/// The oracle's bound on an output sample against the `f64` model: the SVF against the cookbook
/// biquad and the blend's weights, the production ramp's iterated additions against the exact
/// line (measured `9.3e-7` over every case here).
const TOLERANCE: f64 = 1.0e-5;

/// **Gate 1.** Each glide direction, over 64 samples and over 4,800 samples, each crossed by a
/// live range span on every lane (the gliding lane and its bank-mates) and each crossing a block
/// boundary, on 1 kHz tones 20 dB apart: every lane's output stays within `1e-5` of the `f64`
/// dynamic EQ band whose detector is the reference blend, and after it the target law, while the
/// bank-mates rest at their own modes; and the glide arrives exactly (its weights at the target,
/// no countdown left). At the scalar instance, `Simd4` and `Simd8`.
#[test]
fn a_glide_tracks_the_f64_blend_into_the_target_law() {
    let (source_left, source_right) = oracle_signals();
    let values = oracle_values();
    for case in CASES {
        for width in widths() {
            for from in LINKS {
                for to in LINKS.into_iter().filter(|mode| *mode != from) {
                    oracle_case(
                        width,
                        case,
                        from,
                        to,
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
    case: GlideCase,
    from: LinkMode,
    to: LinkMode,
    (source_left, source_right): (&[f32], &[f32]),
    values: &Values,
) {
    let context = format!("{width:?} {case:?} {from:?} -> {to:?}");
    let lanes = lanes(width);
    let modes: Vec<LinkMode> = (0..lanes)
        .map(|lane| if lane == 0 { from } else { LINKS[lane % 3] })
        .collect();
    let requests: Vec<_> = modes.iter().map(|mode| request(values, *mode)).collect();
    let mut under = Under::new(width, &requests);
    let mut left = interleave(&vec![source_left.to_vec(); lanes]);
    let mut right = interleave(&vec![source_right.to_vec(); lanes]);
    for block in 0..FRAMES / BLOCK {
        let first = block * BLOCK;
        if first == GLIDE_FRAME {
            assert!(under.retarget(0, to, case.samples), "a supported glide");
        }
        let spans = if first == case.change {
            vec![both(RANGE_PARAMETER, CHANGED_RANGE).to_vec(); lanes]
        } else {
            vec![Vec::new(); lanes]
        };
        let (flat, offsets) = flatten(&spans, first as u64);
        let range = first * lanes..(first + BLOCK) * lanes;
        under.process(
            &mut left[range.clone()],
            &mut right[range],
            first as u64,
            (&flat, &offsets),
            false,
        );
    }
    assert_eq!(
        under.link_words(0),
        rest_words(to),
        "{context}: the glide did not arrive exactly"
    );
    for (lane, mode) in modes.iter().enumerate() {
        let target = if lane == 0 { to } else { *mode };
        let (model_left, model_right, deepest) =
            oracle_trace(*mode, target, case.samples, source_left, source_right);
        assert!(
            deepest > -f64::from(RANGE) + 4.0,
            "{context}: the model reached the range ({deepest} dB)"
        );
        for frame in 0..FRAMES {
            for (rendered, model) in [
                (left[frame * lanes + lane], model_left[frame]),
                (right[frame * lanes + lane], model_right[frame]),
            ] {
                let deviation = (f64::from(rendered) - model).abs();
                assert!(
                    deviation <= TOLERANCE,
                    "{context} lane {lane} frame {frame}: {rendered}, the model {model}"
                );
            }
        }
    }
    // Not vacuous: the quiet channel's model output moves with the glide.
    let (_, glided, _) = oracle_trace(from, to, case.samples, source_left, source_right);
    let (_, rested, _) = oracle_trace(from, from, case.samples, source_left, source_right);
    assert!(
        glided[GLIDE_FRAME..]
            .iter()
            .zip(&rested[GLIDE_FRAME..])
            .any(|(a, b)| (a - b).abs() > 1.0e-3),
        "{context}: the glide changed nothing"
    );
}

/// One bank-mate's prepared values: its own bands, thresholds, static gains and directions,
/// band 3 disabled on some.
fn member_values(lane: usize, seed: u64) -> Values {
    let pick = |salt: u64| (seed.wrapping_mul(31) + lane as u64 * 7 + salt) % 5;
    values(&[
        (1, 300.0 + 400.0 * pick(1) as f32),
        (3, -6.0 + 3.0 * pick(2) as f32),
        (4, -40.0 + 5.0 * pick(3) as f32),
        (9, if pick(4) % 2 == 0 { 1.0 } else { 2.0 }),
        (13, 2_000.0 + 500.0 * pick(5) as f32),
        (16, -35.0 + 4.0 * pick(6) as f32),
        (36, if pick(7) == 0 { 0.0 } else { 1.0 }),
    ])
}

/// Block `block`'s spans for a bank-mate: a threshold move, then a band move and a direction
/// change, each starting a 64-sample ramp.
fn member_spans(block: usize, lane: usize) -> Vec<PreparedAutomationSpan> {
    match (block + lane) % 4 {
        0 => both(4, -45.0 + lane as f32).to_vec(),
        1 => [both(1, 700.0 + 30.0 * lane as f32), both(9, 2.0)].concat(),
        2 => [both(2, 3.0), both(9, 1.0)].concat(),
        _ => Vec::new(),
    }
}

/// **Gates 2 and 6.** A full bank of distinct members at mixed modes, where lane 0 glides while
/// its bank-mates rest or ramp parameters: every other lane renders bit for bit as its own
/// scalar instance, and the retarget and every gliding block allocate and free nothing. At
/// `Simd4` and `Simd8`.
#[test]
fn a_glide_moves_no_bank_mates_bit_and_allocates_nothing() {
    bench_support::alloc::assert_installed();
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        for seed in 0..3_u64 {
            let members: Vec<Values> = (0..lanes).map(|lane| member_values(lane, seed)).collect();
            let modes: Vec<LinkMode> = (0..lanes)
                .map(|lane| LINKS[(lane + seed as usize) % 3])
                .collect();
            // Neither lane 0's mode nor lane 1's, so a retarget that reaches a bank-mate moves it.
            let target = LINKS[(seed as usize + 2) % 3];
            let requests: Vec<_> = members
                .iter()
                .zip(&modes)
                .map(|(values, mode)| request(values, *mode))
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
                    (0..lanes).map(|lane| member_spans(block, lane)).collect();
                spans[0].clear();
                let (flat, offsets) = flatten(&spans, first_sample);
                let mut left =
                    interleave(&inputs.iter().map(|(l, _)| l.clone()).collect::<Vec<_>>());
                let mut right =
                    interleave(&inputs.iter().map(|(_, r)| r.clone()).collect::<Vec<_>>());
                let mark = bench_support::alloc::current_thread_counters();
                if block == glide_block {
                    assert!(bank.retarget(0, target, 64), "a supported glide");
                }
                bank.process(
                    &mut left,
                    &mut right,
                    first_sample,
                    (&flat, &offsets),
                    false,
                );
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
                    let own = flatten(core::slice::from_ref(&spans[lane]), first_sample);
                    scalar.process(
                        &mut own_left,
                        &mut own_right,
                        first_sample,
                        (&own.0, &own.1),
                        false,
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
/// another mode leaves that mode at rest, and a forged weight outside `[0, 1]`, a settled weight
/// off its target, a target that is no mode's, or channels at two modes are refused and change
/// nothing.
#[test]
fn a_lane_snapshotted_mid_glide_continues_bit_for_bit_and_forged_weights_are_refused() {
    let values = oracle_values();
    let prepare = |mode| Under::new(None, &[request(&values, mode)]);
    let sizes = QUALITIES[1].maximum_state;
    let restore = |under: &mut Under, payload: &(Vec<u8>, Vec<u8>, Vec<u8>)| {
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
        for (block, (left, right)) in left
            .chunks_mut(BLOCK)
            .zip(right.chunks_mut(BLOCK))
            .enumerate()
        {
            let first_sample = (range.start + block * BLOCK) as u64;
            under.process(left, right, first_sample, (&[], &[0, 0]), false);
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
    assert!(source.retarget(0, LinkMode::Average, 96));
    render(&mut source, 1_024..1_044);
    let payload = source.snapshot(0);

    let mut successor = prepare(LinkMode::Average);
    restore(&mut successor, &payload).expect("a mid-glide payload restores");
    assert_eq!(successor.link_words(0), source.link_words(0));
    for range in [1_044..1_100, 1_100..1_400] {
        assert_eq!(
            render(&mut successor, range.clone()),
            render(&mut source, range.clone()),
            "frames {range:?}: the restored glide parted from the uninterrupted one"
        );
    }

    // Under another mode the lane keeps its own, at rest, whatever glide it held before: here it
    // is itself mid-glide to Maximum, and the restore replaces that glide too.
    let mut other = prepare(LinkMode::DualMono);
    assert!(other.retarget(0, LinkMode::Maximum, 96));
    render(&mut other, 0..20);
    restore(&mut other, &payload).expect("restores under another mode");
    assert_eq!(
        other.link_words(0),
        rest_words(LinkMode::Maximum),
        "a payload of another mode must leave the lane's own mode at rest"
    );

    let mut target = prepare(LinkMode::Average);
    let before = target.snapshot(0);
    let first = STATE_LINK_WORD * 4;
    // (word, value): a current outside `[0, 1]` either way, a target that is no mode's (the
    // `w_link` and the `w_avg` one), and a settled `w_avg` weight off its target.
    let forgeries: [&[(usize, f32)]; 5] = [
        &[(0, 1.5)],
        &[(0, -0.25)],
        &[(1, 0.5)],
        &[(5, 0.5)],
        &[(4, 0.5), (5, 1.0), (6, 0.0), (7, 0.0)],
    ];
    for words in forgeries {
        let mut forged = payload.clone();
        for &(word, value) in words {
            forged.1[first + word * 4..first + word * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
        assert_eq!(
            restore(&mut target, &forged)
                .expect_err("a forged weight")
                .code,
            "effect.state.link",
            "{words:?}"
        );
        assert_eq!(target.snapshot(0), before, "a refused restore wrote");
    }
    // The right channel at another mode than the left.
    let mut split = payload.clone();
    let rest = rest_words(LinkMode::Maximum);
    for (word, bits) in rest[..8].iter().enumerate() {
        split.2[first + word * 4..first + word * 4 + 4].copy_from_slice(&bits.to_le_bytes());
    }
    assert_eq!(
        restore(&mut target, &split)
            .expect_err("channels at two modes")
            .code,
        "effect.state.link"
    );
    assert_eq!(target.snapshot(0), before, "a refused restore wrote");
}

/// **D1 refusals.** A glide longer than `LINK_GLIDE_MAX_SAMPLES`, a padded bank lane, a lane
/// past the width and track 1 of a scalar instance each return `false` and leave every payload
/// word as it was. (The descriptor supports every mode and links per lane, so the mode and
/// `lane_link` refusals have no input that reaches them here.)
#[test]
fn an_out_of_range_glide_is_refused_and_changes_nothing() {
    let values = oracle_values();
    let too_long = effect_contract::LINK_GLIDE_MAX_SAMPLES + 1;
    let mut scalar = Under::new(None, &[request(&values, LinkMode::DualMono)]);
    let before = scalar.snapshot(0);
    assert!(
        !scalar.retarget(0, LinkMode::Maximum, too_long),
        "2^24 + 1 samples"
    );
    assert!(
        !scalar.retarget(1, LinkMode::Maximum, 64),
        "track 1 of a scalar instance"
    );
    assert_eq!(scalar.snapshot(0), before);

    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let requests = vec![request(&values, LinkMode::DualMono); lanes];
        let mask: Vec<bool> = (0..lanes).map(|lane| lane == 0).collect();
        let mut bank = Under::Bank(bind(width, &requests, &mask), width);
        let before: Vec<_> = (0..lanes).map(|lane| bank.snapshot(lane)).collect();
        assert!(
            !bank.retarget(0, LinkMode::Maximum, too_long),
            "2^24 + 1 samples"
        );
        assert!(!bank.retarget(1, LinkMode::Maximum, 64), "a padded lane");
        assert!(
            !bank.retarget(lanes, LinkMode::Maximum, 64),
            "a lane past the width"
        );
        let after: Vec<_> = (0..lanes).map(|lane| bank.snapshot(lane)).collect();
        assert_eq!(after, before);
        // The bound itself is accepted.
        assert!(bank.retarget(0, LinkMode::Maximum, too_long - 1));
    }
}

/// A glide in flight is never at rest: on a silent bank at rest, the retargeted lane leaves
/// `lanes_at_rest` until its glide arrives (so no block skips it and the silent path never
/// freezes it), its weights arrive exactly over `+0.0` blocks, and it rests again after. At
/// `Simd4` and `Simd8`.
#[test]
fn a_silent_bank_never_skips_a_glide() {
    let values = oracle_values();
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let all = (1_u32 << lanes) - 1;
        let requests = vec![request(&values, LinkMode::DualMono); lanes];
        let mut bank = Under::new(Some(width), &requests);
        let silent = |bank: &mut Under, block: usize| {
            let (mut left, mut right) = (vec![0.0; BLOCK * lanes], vec![0.0; BLOCK * lanes]);
            let offsets = vec![0_u32; lanes + 1];
            bank.process(
                &mut left,
                &mut right,
                (block * BLOCK) as u64,
                (&[], &offsets),
                false,
            );
            assert!(
                left.iter().chain(&right).all(|word| word.to_bits() == 0),
                "{width:?} block {block}: silence in, silence out"
            );
        };
        silent(&mut bank, 0);
        assert_eq!(bank.lanes_at_rest(), all, "{width:?}: a silent bank rests");
        // Three blocks and a frame.
        let samples = 3 * BLOCK as u32 + 1;
        assert!(bank.retarget(0, LinkMode::Average, samples));
        for block in 1..=4 {
            assert_eq!(
                bank.lanes_at_rest(),
                all & !1,
                "{width:?} block {block}: a glide in flight is at rest"
            );
            silent(&mut bank, block);
        }
        assert_eq!(
            bank.link_words(0),
            rest_words(LinkMode::Average),
            "{width:?}: the glide did not arrive over silent blocks"
        );
        assert_eq!(bank.lanes_at_rest(), all, "{width:?}: it rests again");
    }
}

/// A collapsed bank (issue #973) glides as the dual bank does on `L == R`: lane 0 glides
/// while the bank renders collapsed, the disengage copy carries its weights to the right channel
/// mid-glide, and the dual blocks after it, on channels 20 dB apart, render both channels bit for
/// bit as a bank that never collapsed. At `Simd4` and `Simd8`.
#[test]
fn a_collapsed_glide_matches_the_dual_glide_and_survives_the_disengage() {
    let values = oracle_values();
    let (source, _) = oracle_signals();
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let requests: Vec<_> = (0..lanes)
            .map(|lane| request(&values, LINKS[lane % 3]))
            .collect();
        let mut collapsed = Under::new(Some(width), &requests);
        let mut dual = Under::new(Some(width), &requests);
        let offsets = vec![0_u32; lanes + 1];
        for block in 0..16 {
            let first = block * BLOCK;
            if block == 2 {
                assert!(collapsed.retarget(0, LinkMode::Maximum, 200));
                assert!(dual.retarget(0, LinkMode::Maximum, 200));
            }
            let mono = block < 4;
            if block == 4 {
                let Under::Bank(bank, _) = &mut collapsed else {
                    unreachable!("a bank")
                };
                bank.desymmetrize_channels();
                assert!(
                    bank.channels_agree(),
                    "{width:?}: the disengage copy agrees"
                );
            }
            let plane: Vec<f32> = (0..BLOCK * lanes)
                .map(|index| source[first + index / lanes] * (1.0 + index as f32 % 3.0))
                .collect();
            // `L == R` while collapsed; after the disengage the right is 20 dB down, so the dual
            // blocks show how far each bank's glide has come.
            let other: Vec<f32> = if mono {
                plane.clone()
            } else {
                plane.iter().map(|word| 0.1 * word).collect()
            };
            let (mut left, mut right) = (plane.clone(), other.clone());
            collapsed.process(&mut left, &mut right, first as u64, (&[], &offsets), mono);
            let (mut dual_left, mut dual_right) = (plane, other);
            dual.process(
                &mut dual_left,
                &mut dual_right,
                first as u64,
                (&[], &offsets),
                false,
            );
            let bits = |plane: &[f32]| plane.iter().map(|word| word.to_bits()).collect::<Vec<_>>();
            assert_eq!(
                bits(&left),
                bits(&dual_left),
                "{width:?} block {block}: left"
            );
            if !mono {
                assert_eq!(
                    bits(&right),
                    bits(&dual_right),
                    "{width:?} block {block}: right"
                );
            }
        }
    }
}
