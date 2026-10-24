//! Issue #1370: a lane's detector link glides between modes (D1-D3, D6), at every width the build
//! has: the scalar instance, and banks bound at `Simd4` and `Simd8` through [`BankParts`] (the
//! factory's D4 width gate binds only the build's own width, as `padding_tests` explains).
//!
//! The law's exact half (the arrival frame and a resting lane's words) is
//! `kernel::glide_law_tests`.

use super::padding_tests::{LINKS, QUANTUM, Values, bind, draw_spans, draw_values, request};
use super::*;
use dsp_reference::randomized::Draw;
use dsp_reference::{
    ReferenceCompressorParameters, ReferenceLinkGlide, ReferencePeakCompressor,
    reference_link_blend,
};
use effect_contract::{EffectProcessBlock, PreparedSidechainPort, link_glide_weights};

const RATE: u32 = 48_000;
const BLOCK: usize = 48;
const BLOCKS: usize = 12;
/// The glide: retargeted before block `GLIDE_BLOCK`, it arrives inside the next one.
const GLIDE_BLOCK: usize = 4;
const GLIDE_SAMPLES: u32 = 64;

/// One prepared compressor under test: the scalar instance at `L = f32`, else a full bank.
enum Under {
    Scalar(Box<dyn PreparedNativeEffect>),
    Bank(Box<dyn PreparedNativeEffectBank>, BankWidth),
}

impl Under {
    fn new<L: Lane>(requests: &[PrepareEffectRequest<'_>]) -> Self {
        if L::WIDTH == 1 {
            let prepared = CompressorFactory.prepare(requests[0]).expect("prepare");
            return Self::Scalar(prepared.processor);
        }
        let mask = vec![true; L::WIDTH];
        let width = BankWidth::for_lanes(L::WIDTH).expect("a bank width");
        Self::Bank(Box::new(bind::<L>(requests, &mask)), width)
    }

    fn retarget(&mut self, lane: usize, mode: LinkMode, samples: u32) -> bool {
        match self {
            Self::Scalar(effect) => effect.retarget_link(lane, mode, samples),
            Self::Bank(bank, _) => bank.retarget_link(lane, mode, samples),
        }
    }

    /// One block of frame-major planes (`frame * lanes + lane`), with `spans[lane]` at its
    /// first sample.
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        first_sample: u64,
        spans: &[Vec<PreparedAutomationSpan>],
    ) {
        let (flat, offsets) = flatten(spans, first_sample);
        self.process_flat(left, right, first_sample, &flat, &offsets);
    }

    /// [`process`](Self::process) with the spans already flattened: no allocation of its own.
    fn process_flat(
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

/// `frames` frames of `lanes` strips, frame-major.
fn interleave(planes: &[Vec<f32>]) -> Vec<f32> {
    let frames = planes[0].len();
    (0..frames * planes.len())
        .map(|index| planes[index % planes.len()][index / planes.len()])
        .collect()
}

fn values(threshold: f32, ratio: f32, knee: f32, attack: f32, release: f32) -> Values {
    let row = [threshold, ratio, knee, attack, release, 0.0, 1.0];
    core::array::from_fn(|slot| InitialParameterValue {
        parameter_index: (slot / 2) as u32,
        channel: if slot % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: row[slot / 2],
    })
}

/// Lane `lane`'s rest mode in the oracle case: lane 0 glides from `from`, the others rest at
/// varied modes.
fn rest_mode(lane: usize, from: LinkMode) -> LinkMode {
    if lane == 0 {
        from
    } else {
        LINKS[lane % LINKS.len()]
    }
}

/// One glide case of the oracle test.
#[derive(Clone, Copy, Debug)]
struct GlideCase {
    samples: u32,
    /// A live makeup change, 0 to 3 dB, on every lane before this block: a 64-sample parameter
    /// ramp the glide crosses.
    change: Option<usize>,
    /// Detect on a connected sidechain key (the scalar instance only: a sidechain never banks).
    sidechain: bool,
}

const CASES: [GlideCase; 3] = [
    GlideCase {
        samples: GLIDE_SAMPLES,
        change: None,
        sidechain: false,
    },
    GlideCase {
        samples: 300,
        change: Some(GLIDE_BLOCK + 2),
        sidechain: false,
    },
    GlideCase {
        samples: 300,
        change: Some(GLIDE_BLOCK + 2),
        sidechain: true,
    },
];

const CHANGED_MAKEUP_DB: f32 = 3.0;

/// Lane `lane`'s left-channel link weight words in a payload: `w_link` then `w_avg`, each
/// `current, target, step, remaining`.
fn payload_link_words(under: &Under, lane: usize) -> [u32; 8] {
    let sizes = COMPRESSOR_DESCRIPTOR.qualities[0].maximum_state;
    let (mut left, mut right) = (
        vec![0; sizes.left_bytes as usize],
        vec![0; sizes.right_bytes as usize],
    );
    let output = StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("sizes");
    match under {
        Under::Scalar(effect) => effect.snapshot_state_payload(output),
        Under::Bank(bank, _) => bank.snapshot_track_state_payload(lane as u32, output),
    }
    .expect("snapshot");
    link_words(&left)
}

/// Renders the oracle case once at `L`: the worst deviation from the `f64` compressor fed the
/// reference detector, and the right planes, frame-major. Asserts that the glide arrived exactly.
fn oracle_case<L: Lane>(from: LinkMode, to: Option<LinkMode>, case: GlideCase) -> (f64, Vec<f32>) {
    let lanes = L::WIDTH;
    let parameters = values(-24.0, 4.0, 6.0, 2.0, 80.0);
    let modes: Vec<LinkMode> = (0..lanes).map(|lane| rest_mode(lane, from)).collect();
    let requests: Vec<_> = modes
        .iter()
        .map(|mode| {
            let mut request = request(&parameters, RATE, *mode);
            if case.sidechain {
                request.ports.sidechain = PreparedSidechainPort::Connected {
                    id: port_id("sidechain-in"),
                    required: false,
                };
            }
            request
        })
        .collect();
    let mut under = Under::new::<L>(&requests);
    let frames = BLOCK * BLOCKS;
    // Channels 20 dB apart, every lane its own signal; under a sidechain the key is another pair
    // 20 dB apart, so the detector is not the main signal.
    let inputs: Vec<(Vec<f32>, Vec<f32>)> = (0..lanes as u64)
        .map(|lane| {
            (
                noise(lane + 1, frames, 0.8),
                noise(lane + 101, frames, 0.08),
            )
        })
        .collect();
    let key = (noise(201, frames, 0.6), noise(202, frames, 0.06));
    let mut left = interleave(&inputs.iter().map(|(l, _)| l.clone()).collect::<Vec<_>>());
    let mut right = interleave(&inputs.iter().map(|(_, r)| r.clone()).collect::<Vec<_>>());
    for block in 0..BLOCKS {
        if block == GLIDE_BLOCK
            && let Some(to) = to
        {
            assert!(under.retarget(0, to, case.samples), "a supported glide");
        }
        let first = (block * BLOCK) as u64;
        let spans: Vec<Vec<PreparedAutomationSpan>> = (0..lanes)
            .map(|_| {
                if case.change != Some(block) {
                    return Vec::new();
                }
                [ParameterChannel::Left, ParameterChannel::Right]
                    .map(|channel| PreparedAutomationSpan {
                        channel,
                        parameter_index: 5,
                        sample: first,
                        value: CHANGED_MAKEUP_DB,
                    })
                    .to_vec()
            })
            .collect();
        let range = block * BLOCK * lanes..(block + 1) * BLOCK * lanes;
        if case.sidechain {
            let Under::Scalar(effect) = &mut under else {
                unreachable!("a sidechain never banks")
            };
            let (flat, _) = flatten(&spans, first);
            let frames = block * BLOCK..(block + 1) * BLOCK;
            effect.process(
                EffectProcessBlock::new(
                    &mut left[range.clone()],
                    &mut right[range],
                    Some((&key.0[frames.clone()], &key.1[frames])),
                    first,
                    &flat,
                    QUANTUM,
                )
                .expect("a well-shaped block"),
            );
        } else {
            under.process(&mut left[range.clone()], &mut right[range], first, &spans);
        }
    }
    if let Some(to) = to {
        let rest = link_glide_weights(to).map(f32::to_bits);
        assert_eq!(
            payload_link_words(&under, 0),
            [rest[0], rest[0], 0, 0, rest[1], rest[1], 0, 0],
            "W{lanes} {case:?} {from:?} -> {to:?}: the glide did not arrive exactly"
        );
    }
    let reference_parameters = ReferenceCompressorParameters {
        threshold_db: -24.0,
        ratio: 4.0,
        knee_db: 6.0,
        attack_ms: 2.0,
        release_ms: 80.0,
        makeup_db: 0.0,
        mix: 1.0,
    };
    let weights = |mode: LinkMode| link_glide_weights(mode).map(f64::from);
    let mut worst = 0.0_f64;
    for (lane, (input_left, input_right)) in inputs.iter().enumerate() {
        let mut channels = [
            ReferencePeakCompressor::new(f64::from(RATE), reference_parameters).expect("oracle"),
            ReferencePeakCompressor::new(f64::from(RATE), reference_parameters).expect("oracle"),
        ];
        let start = weights(modes[lane]);
        let mut glide = match to {
            Some(to) if lane == 0 => ReferenceLinkGlide::new(start, weights(to), case.samples),
            _ => ReferenceLinkGlide::new(start, start, 0),
        };
        for frame in 0..frames {
            let (l, r) = (f64::from(input_left[frame]), f64::from(input_right[frame]));
            let (key_left, key_right) = if case.sidechain {
                (f64::from(key.0[frame]), f64::from(key.1[frame]))
            } else {
                (l, r)
            };
            let now = if frame < GLIDE_BLOCK * BLOCK {
                start
            } else {
                glide.next_weights().0
            };
            // The makeup ramp: `k / 64` of the way on the `k`-th frame of the change, exact on
            // the 64th.
            if let Some(block) = case.change
                && let Some(k) = (frame + 1).checked_sub(block * BLOCK)
                && (1..=64).contains(&k)
            {
                let makeup = f64::from(CHANGED_MAKEUP_DB) * k as f64 / 64.0;
                channels.iter_mut().for_each(|channel| channel.set_makeup_db(makeup));
            }
            let (detector_left, detector_right) = reference_link_blend(key_left, key_right, now);
            let expected = [
                channels[0].process_sample(l, detector_left),
                channels[1].process_sample(r, detector_right),
            ];
            let index = frame * lanes + lane;
            for (actual, expected) in [left[index], right[index]].into_iter().zip(expected) {
                worst = worst.max((f64::from(actual) - expected).abs());
            }
        }
    }
    (worst, right)
}

/// **Gate 1.** Each glide direction, crossing a block boundary, on channels 20 dB apart: over 64
/// samples; over 300 samples crossed by a live makeup ramp on every lane (the gliding lane and its
/// bank-mates); and, at the scalar instance, the same on a connected sidechain key 20 dB apart.
/// Every lane's output stays within the oracle's `2e-5` of the `f64` compressor fed the
/// reference blend (`dsp_reference::reference_link_blend`) of its detector source, and after it
/// the target law, while the bank-mates rest at their own modes; and the glide arrives exactly
/// (its weights at the target, no countdown left). The bound and its derivation are
/// `tests/oracle.rs`'s: the blend adds a detector error of a few `f32` ulps, far inside it. At the
/// scalar instance, `Simd4` and `Simd8`.
#[test]
fn a_glide_tracks_the_f64_blend_into_the_target_law() {
    lane::each_lane!(|L| {
        for case in CASES {
            if case.sidechain && L::WIDTH != 1 {
                continue;
            }
            for from in LINKS {
                for to in LINKS.into_iter().filter(|mode| *mode != from) {
                    let (worst, glided) = oracle_case::<L>(from, Some(to), case);
                    assert!(
                        worst <= 2.0e-5,
                        "W{} {case:?} {from:?} -> {to:?}: worst deviation {worst:e}",
                        L::WIDTH
                    );
                    // Not vacuous: the quiet channel renders differently once the link moves.
                    let (_, rested) = oracle_case::<L>(from, None, case);
                    let lanes = L::WIDTH;
                    assert!(
                        (0..BLOCK * BLOCKS).any(|frame| glided[frame * lanes].to_bits()
                            != rested[frame * lanes].to_bits()),
                        "W{} {case:?} {from:?} -> {to:?}: the glide changed nothing",
                        L::WIDTH
                    );
                }
            }
        }
    });
}

/// **Gates 2 and 6.** A full bank of distinct members at mixed modes, where lane 0 glides while
/// its bank-mates rest or ramp parameters: every other lane renders bit for bit as its own
/// scalar instance, and the retarget and the switching block allocate and free nothing. At
/// `Simd4` and `Simd8`.
#[test]
fn a_glide_moves_no_bank_mates_bit_and_allocates_nothing() {
    bench_support::alloc::assert_installed();
    lane::each_vector_lane!(|L| {
        let lanes = L::WIDTH;
        for seed in 0..6_u64 {
            let mut draw = Draw::new(0x1370_0000 + seed);
            let members: Vec<Values> = (0..lanes).map(|_| draw_values(&mut draw, false)).collect();
            let modes: Vec<LinkMode> = (0..lanes)
                .map(|lane| LINKS[(lane + seed as usize) % LINKS.len()])
                .collect();
            let target = LINKS[(seed as usize + 1) % LINKS.len()];
            let requests: Vec<_> = members
                .iter()
                .zip(&modes)
                .map(|(values, mode)| request(values, RATE, *mode))
                .collect();
            let mut bank = Under::new::<L>(&requests);
            let mut scalars: Vec<Under> = requests
                .iter()
                .map(|request| Under::new::<f32>(core::slice::from_ref(request)))
                .collect();
            let mut first_sample = 0_u64;
            for block in 0..BLOCKS {
                let inputs: Vec<(Vec<f32>, Vec<f32>)> = (0..lanes)
                    .map(|lane| {
                        let salt = seed * 1_000 + (block * lanes + lane) as u64;
                        (noise(salt, BLOCK, 0.9), noise(salt + 500, BLOCK, 0.2))
                    })
                    .collect();
                let mut spans: Vec<Vec<PreparedAutomationSpan>> =
                    (0..lanes).map(|_| draw_spans(&mut draw, false)).collect();
                spans[0].clear();
                spans.iter_mut().flatten().for_each(|span| {
                    if span.channel == ParameterChannel::Both {
                        span.channel = ParameterChannel::Left;
                    }
                });
                let mut left =
                    interleave(&inputs.iter().map(|(l, _)| l.clone()).collect::<Vec<_>>());
                let mut right =
                    interleave(&inputs.iter().map(|(_, r)| r.clone()).collect::<Vec<_>>());
                let (flat, offsets) = flatten(&spans, first_sample);
                let mark = bench_support::alloc::current_thread_counters();
                if block == GLIDE_BLOCK {
                    assert!(bank.retarget(0, target, GLIDE_SAMPLES), "a supported glide");
                }
                bank.process_flat(&mut left, &mut right, first_sample, &flat, &offsets);
                let delta = bench_support::alloc::current_thread_delta_since(mark);
                if block >= GLIDE_BLOCK {
                    assert_eq!(
                        delta.allocations + delta.deallocations + delta.reallocations,
                        0,
                        "W{lanes} seed {seed} block {block}: a gliding block touched the heap"
                    );
                }
                for (lane, scalar) in scalars.iter_mut().enumerate().skip(1) {
                    let (mut own_left, mut own_right) = inputs[lane].clone();
                    scalar.process(
                        &mut own_left,
                        &mut own_right,
                        first_sample,
                        core::slice::from_ref(&spans[lane]),
                    );
                    for frame in 0..BLOCK {
                        assert_eq!(
                            (
                                left[frame * lanes + lane].to_bits(),
                                right[frame * lanes + lane].to_bits()
                            ),
                            (own_left[frame].to_bits(), own_right[frame].to_bits()),
                            "W{lanes} seed {seed} block {block} lane {lane} frame {frame}: a \
                             bank-mate of the glide moved"
                        );
                    }
                }
                first_sample += BLOCK as u64;
            }
        }
    });
}

/// The weight words of one payload channel section: `w_link` and `w_avg`, each `current, target,
/// step, remaining`.
fn link_words(section: &[u8]) -> [u32; 8] {
    let first = STATE_HEADER_WORDS - 8;
    core::array::from_fn(|word| {
        let at = (first + word) * 4;
        u32::from_le_bytes(section[at..at + 4].try_into().expect("a word"))
    })
}

/// **Gate 5.** A lane snapshotted mid-glide and restored into an instance prepared with the
/// glide's target mode renders bit for bit as the uninterrupted one. A payload restored under
/// another mode leaves that mode at rest, and a forged weight outside `[0, 1]`, or a target that
/// is no mode's, is refused and changes nothing.
#[test]
fn a_lane_snapshotted_mid_glide_continues_bit_for_bit_and_forged_weights_are_refused() {
    let parameters = values(-30.0, 6.0, 3.0, 1.0, 60.0);
    let prepare = |mode| {
        CompressorFactory
            .prepare(request(&parameters, RATE, mode))
            .expect("prepare")
            .processor
    };
    let sizes = CompressorFactory
        .prepare(request(&parameters, RATE, LinkMode::DualMono))
        .expect("prepare")
        .metadata
        .state_sizes;
    let snapshot = |effect: &dyn PreparedNativeEffect| {
        let (mut left, mut right) = (
            vec![0; sizes.left_bytes as usize],
            vec![0; sizes.right_bytes as usize],
        );
        effect
            .snapshot_state_payload(
                StatePayloadOutput::new(&mut [], &mut left, &mut right, sizes).expect("sizes"),
            )
            .expect("snapshot");
        (left, right)
    };
    let restore = |effect: &mut dyn PreparedNativeEffect, left: &[u8], right: &[u8]| {
        effect.restore_state_payload(
            COMPRESSOR_DESCRIPTOR.state_layout_version,
            StatePayloadInput::new(&[], left, right, sizes).expect("sizes"),
        )
    };
    let (input_left, input_right) = (noise(7, 256, 0.9), noise(8, 256, 0.09));
    let render = |effect: &mut dyn PreparedNativeEffect, range: core::ops::Range<usize>| {
        let (mut left, mut right) = (
            input_left[range.clone()].to_vec(),
            input_right[range.clone()].to_vec(),
        );
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                range.start as u64,
                &[],
                QUANTUM,
            )
            .expect("a block"),
        );
        (left, right)
    };
    let mut source = prepare(LinkMode::DualMono);
    render(source.as_mut(), 0..64);
    assert!(source.retarget_link(0, LinkMode::Average, 96));
    render(source.as_mut(), 64..84);
    let (left, right) = snapshot(source.as_ref());

    let mut successor = prepare(LinkMode::Average);
    restore(successor.as_mut(), &left, &right).expect("a mid-glide payload restores");
    for range in [84..128, 128..256] {
        let expected = render(source.as_mut(), range.clone());
        let actual = render(successor.as_mut(), range.clone());
        assert_eq!(
            (bits(&actual.0), bits(&actual.1)),
            (bits(&expected.0), bits(&expected.1)),
            "frames {range:?}: the restored glide parted from the uninterrupted one"
        );
    }

    // Under another mode the lane keeps its own, at rest: Maximum's weights `[1, 0]`, whatever
    // glide it held before.
    // The lane is itself mid-glide to Maximum: the restore replaces that glide too.
    let mut other = prepare(LinkMode::DualMono);
    assert!(other.retarget_link(0, LinkMode::Maximum, 96));
    render(other.as_mut(), 0..20);
    restore(other.as_mut(), &left, &right).expect("restores under another mode");
    let one = 1.0_f32.to_bits();
    assert_eq!(
        link_words(&snapshot(other.as_ref()).0),
        [one, one, 0, 0, 0, 0, 0, 0],
        "a payload of another mode must leave the lane's own mode at rest"
    );

    let mut target = prepare(LinkMode::Average);
    let before = snapshot(target.as_ref());
    let first = (STATE_HEADER_WORDS - 8) * 4;
    for (word, value) in [(0, 1.5_f32), (0, -0.25), (1, 0.5), (5, 0.5)] {
        let mut forged = left.clone();
        forged[first + word * 4..first + word * 4 + 4].copy_from_slice(&value.to_le_bytes());
        assert_eq!(
            restore(target.as_mut(), &forged, &right)
                .expect_err("a forged weight")
                .code,
            "effect.state.link",
            "word {word} = {value}"
        );
        assert_eq!(snapshot(target.as_ref()), before, "a refused restore wrote");
    }
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}
