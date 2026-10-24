//! Issue #1370: a lane's detector link glides between modes (D1-D3, D6), at every width the build
//! has: the scalar instance, and banks bound at `Simd4` and `Simd8` (the width the build does not
//! execute binds through `bind_bank::<false>`, as `padding_tests` explains).
//!
//! The law's exact half (the arrival frame and a resting lane's level) is
//! `kernel::glide_law_tests`.

use super::padding_tests::{Draw, LINKS, QUANTUM, Values, bind, request, values_from};
use super::*;
use dsp_reference::{
    ReferenceGateExpanderParameters, ReferenceGateLink, ReferenceGateLinkGlide, ReferenceGatePhase,
    ReferenceGateTiming, ReferenceLinkGlide, reference_gate_expander_process_with_link_glide,
};
use effect_contract::{EffectProcessBlock, link_glide_weights};

const RATE: u32 = 48_000;
const BLOCK: usize = 48;
/// The glide: retargeted before the block that starts at `GLIDE_FRAME`, over `GLIDE_SAMPLES`, so
/// it crosses a block boundary.
const GLIDE_FRAME: usize = 2_400;
const GLIDE_SAMPLES: u32 = 64;
const FRAMES: usize = 4_800;
/// `tests/oracle.rs`'s bound on the rendered gain against the `f64` model, derived there; the
/// blend adds a level error of a few `f32` ulps (its weights are exact multiples of `1/64`).
const TOLERANCE_DB: f64 = 0.02;

/// One prepared gate under test: the scalar instance at one lane, else a bank.
enum Under {
    Scalar(Box<dyn PreparedNativeEffect>),
    Bank(Box<dyn PreparedNativeEffectBank>, BankWidth),
}

impl Under {
    fn new(width: Option<BankWidth>, requests: &[PrepareEffectRequest<'_>]) -> Self {
        let Some(width) = width else {
            let prepared = GateExpanderFactory.prepare(requests[0]).expect("prepare");
            return Self::Scalar(prepared.processor);
        };
        let mask = vec![true; requests.len()];
        let bank = bind(width, requests, &mask)
            .expect("a bank binds")
            .expect("one program");
        Self::Bank(bank.processor, width)
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

const THRESHOLD: f32 = -40.0;
const HYSTERESIS: f32 = 6.0;

fn oracle_values() -> Values {
    values_from([
        (THRESHOLD, THRESHOLD),
        (4.0, 4.0),
        (48.0, 48.0),
        (HYSTERESIS, HYSTERESIS),
        (1.0, 1.0),
        (5.0, 5.0),
        (100.0, 100.0),
    ])
}

/// Square waves 20 dB apart, the sign flipping every eight samples: the left at -30 dBFS, open
/// under every mode, and the right at -50 dBFS, closed alone and opened by either link.
fn oracle_signals() -> (Vec<f32>, Vec<f32>) {
    let (loud, quiet) = (0.031_622_777_f32, 0.003_162_277_7_f32);
    let sign = |frame: usize| if (frame / 8) % 2 == 0 { 1.0 } else { -1.0 };
    (
        (0..FRAMES).map(|frame| sign(frame) * loud).collect(),
        (0..FRAMES).map(|frame| sign(frame) * quiet).collect(),
    )
}

fn reference_link(mode: LinkMode) -> ReferenceGateLink {
    match mode {
        LinkMode::DualMono => ReferenceGateLink::DualMono,
        LinkMode::Maximum => ReferenceGateLink::Maximum,
        LinkMode::Average => ReferenceGateLink::Average,
    }
}

/// The `f64` model of one lane: `from` until the glide, then the glide to `to` (a step glide to
/// `from` itself for a lane that rests).
fn oracle_trace(
    from: LinkMode,
    to: LinkMode,
    left: &[f32],
    right: &[f32],
) -> dsp_reference::ReferenceGateTrace {
    let parameters = ReferenceGateExpanderParameters {
        threshold_db: f64::from(THRESHOLD),
        ratio: 4.0,
        range_db: 48.0,
    };
    let timing = ReferenceGateTiming {
        sample_rate: RATE,
        attack_ms: 1.0,
        hold_ms: 5.0,
        release_ms: 100.0,
    };
    let weights = |mode: LinkMode| link_glide_weights(mode).map(f64::from);
    let samples = if from == to { 0 } else { GLIDE_SAMPLES };
    let (left, right): (Vec<f64>, Vec<f64>) = (
        left.iter().map(|x| f64::from(*x)).collect(),
        right.iter().map(|x| f64::from(*x)).collect(),
    );
    reference_gate_expander_process_with_link_glide(
        (parameters, parameters),
        (f64::from(HYSTERESIS), f64::from(HYSTERESIS)),
        (timing, timing),
        reference_link(from),
        (&left, &right),
        ReferenceGateLinkGlide {
            frame: GLIDE_FRAME,
            glide: ReferenceLinkGlide::new(weights(from), weights(to), samples),
        },
    )
    .expect("reference render")
}

/// The worst deviation of one rendered channel from the model's gain, in dB; an open model gate
/// at unity must be the exact identity.
fn deviation(rendered: &[f32], gain_db: &[f64], dry: &[f64], phase: &[ReferenceGatePhase]) -> f64 {
    let mut worst = 0.0_f64;
    for frame in 0..rendered.len() {
        if phase[frame] == ReferenceGatePhase::Open && gain_db[frame] == 0.0 {
            assert_eq!(
                rendered[frame].to_bits(),
                (dry[frame] as f32).to_bits(),
                "frame {frame}"
            );
        }
        let applied = 20.0 * (f64::from(rendered[frame]).abs() / dry[frame].abs()).log10();
        worst = worst.max((applied - gain_db[frame]).abs());
    }
    worst
}

/// **Gate 1.** Each glide direction over 64 samples, crossing a block boundary, on square waves
/// 20 dB apart: every lane's gain stays within the oracle's `0.02 dB` of the `f64` gate whose
/// level is the reference blend, and after it the target law, while the bank-mates rest at their
/// own modes. No model level comes within `1e-3 dB` of a decision threshold, so the comparison
/// measures the levels, not a flipped transition. At the scalar instance, `Simd4` and `Simd8`.
#[test]
fn a_glide_tracks_the_f64_blend_into_the_target_law() {
    let (source_left, source_right) = oracle_signals();
    let values = oracle_values();
    for width in widths() {
        let lanes = lanes(width);
        for from in LINKS {
            for to in LINKS.into_iter().filter(|mode| *mode != from) {
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
                    .map(|mode| request(&values, RATE, *mode))
                    .collect();
                let mut under = Under::new(width, &requests);
                let mut left = interleave(&vec![source_left.clone(); lanes]);
                let mut right = interleave(&vec![source_right.clone(); lanes]);
                let (flat, offsets) = flatten(&vec![Vec::new(); lanes], 0);
                for block in 0..FRAMES / BLOCK {
                    if block * BLOCK == GLIDE_FRAME {
                        assert!(under.retarget(0, to, GLIDE_SAMPLES), "a supported glide");
                    }
                    let range = block * BLOCK * lanes..(block + 1) * BLOCK * lanes;
                    under.process(
                        &mut left[range.clone()],
                        &mut right[range],
                        (block * BLOCK) as u64,
                        &flat,
                        &offsets,
                    );
                }
                for (lane, mode) in modes.iter().enumerate() {
                    let target = if lane == 0 { to } else { *mode };
                    let trace = oracle_trace(*mode, target, &source_left, &source_right);
                    for level in trace.level_db_left.iter().chain(&trace.level_db_right) {
                        let clear = (level - f64::from(THRESHOLD))
                            .abs()
                            .min((level - f64::from(THRESHOLD - HYSTERESIS)).abs());
                        assert!(
                            clear >= 1.0e-3,
                            "{from:?} -> {to:?}: level {level} dB on a threshold"
                        );
                    }
                    let lane_of = |planes: &[f32]| -> Vec<f32> {
                        (0..FRAMES)
                            .map(|frame| planes[frame * lanes + lane])
                            .collect()
                    };
                    let worst = deviation(
                        &lane_of(&left),
                        &trace.gain_db_left,
                        &trace.dry_left,
                        &trace.phase_left,
                    )
                    .max(deviation(
                        &lane_of(&right),
                        &trace.gain_db_right,
                        &trace.dry_right,
                        &trace.phase_right,
                    ));
                    assert!(
                        worst <= TOLERANCE_DB,
                        "{width:?} {from:?} -> {to:?} lane {lane}: worst deviation {worst:e} dB"
                    );
                }
                // Not vacuous: the quiet channel's model level moves with the glide.
                let glided = oracle_trace(from, to, &source_left, &source_right);
                let rested = oracle_trace(from, from, &source_left, &source_right);
                assert!(
                    glided.level_db_right[GLIDE_FRAME..]
                        .iter()
                        .zip(&rested.level_db_right[GLIDE_FRAME..])
                        .any(|(a, b)| (a - b).abs() > 1.0),
                    "{from:?} -> {to:?}: the glide changed nothing"
                );
            }
        }
    }
}

/// **Gates 2 and 6.** A full bank of distinct members at mixed modes, where lane 0 glides while
/// its bank-mates rest or ramp parameters: every other lane renders bit for bit as its own
/// scalar instance, and the retarget and the switching block allocate and free nothing. At
/// `Simd4` and `Simd8`.
#[test]
fn a_glide_moves_no_bank_mates_bit_and_allocates_nothing() {
    bench_support::alloc::assert_installed();
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        for seed in 0..6_u64 {
            let mut draw = Draw::new(0x1370_1000 + seed);
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
        GateExpanderFactory
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
            STATE_LAYOUT_VERSION,
            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("sizes"),
        )
    };
    let (input_left, input_right) = (noise(7, 256, 0.05), noise(8, 256, 0.005));
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
        let bits = |values: &[f32]| {
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        };
        (bits(&left), bits(&right))
    };
    let mut source = prepare(LinkMode::DualMono).processor;
    render(source.as_mut(), 0..64);
    assert!(source.retarget_link(0, LinkMode::Average, 96));
    render(source.as_mut(), 64..84);
    let payload = snapshot(source.as_ref());

    let mut successor = prepare(LinkMode::Average).processor;
    restore(successor.as_mut(), &payload).expect("a mid-glide payload restores");
    for range in [84..128, 128..256] {
        assert_eq!(
            render(successor.as_mut(), range.clone()),
            render(source.as_mut(), range.clone()),
            "frames {range:?}: the restored glide parted from the uninterrupted one"
        );
    }

    // Under another mode the lane keeps its own, at rest: Maximum's weights `[1, 0]`.
    let mut other = prepare(LinkMode::Maximum).processor;
    restore(other.as_mut(), &payload).expect("restores under another mode");
    let first = STATE_LINK_WORD * 4;
    let words = |section: &[u8]| -> Vec<u32> {
        section[first..first + 32]
            .as_chunks::<4>()
            .0
            .iter()
            .map(|word| u32::from_le_bytes(*word))
            .collect()
    };
    let one = 1.0_f32.to_bits();
    assert_eq!(
        words(&snapshot(other.as_ref()).1),
        [one, one, 0, 0, 0, 0, 0, 0],
        "a payload of another mode must leave the lane's own mode at rest"
    );

    let mut target = prepare(LinkMode::Average).processor;
    let before = snapshot(target.as_ref());
    for (word, value) in [(0, 1.5_f32), (0, -0.25), (1, 0.5), (5, 0.5)] {
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
}
