//! Issue #1367b: a lane's detector link glides between modes (#1367 D3-D4, the #1370 shape), at
//! every width the build has: the scalar instance, and banks bound at `Simd4` and `Simd8` through
//! the public `bind_homogeneous_bank` (the multiband has no width gate, `padding_tests`).

use super::padding_tests::{
    LINKS, Payload, QUANTUM, Values, bank_payload, bind, draw_spans, draw_values, payload_of,
    prefix_mask, request,
};
use super::*;
use dsp_reference::randomized::Draw;
use dsp_reference::{ReferenceLinkGlide, reference_link_blend};
use effect_contract::{EffectProcessBlock, link_glide_weights};
use std::cell::RefCell;

const RATE: u32 = 48_000;
/// A block length no glide length divides, so glides start and arrive mid-block.
const BLOCK: usize = 37;

thread_local! {
    /// Every [`glide_levels`] call of this thread while armed: the two band words in, then the
    /// two levels out, each across the lanes (`None`: disarmed, nothing is kept).
    static RECORD: RefCell<Option<Vec<[[f32; 8]; 4]>>> = const { RefCell::new(None) };
}

/// The hook [`glide_levels`] calls in a test build: keeps its words while [`RECORD`] is armed.
pub(crate) fn record<L: Lane>(raw: (L, L), levels: (L, L)) {
    RECORD.with(|record| {
        if let Some(record) = record.borrow_mut().as_mut() {
            let mut words = [[0.0_f32; 8]; 4];
            for (word, value) in words.iter_mut().zip([raw.0, raw.1, levels.0, levels.1]) {
                value.store(&mut word[..L::WIDTH]);
            }
            record.push(words);
        }
    });
}

/// Runs `body` with [`RECORD`] armed and returns what it kept.
fn recorded(body: impl FnOnce()) -> Vec<[[f32; 8]; 4]> {
    RECORD.with(|record| *record.borrow_mut() = Some(Vec::new()));
    body();
    RECORD
        .with(|record| record.borrow_mut().take())
        .unwrap_or_default()
}

/// One prepared multiband under test: the scalar instance at one lane, else a bank.
enum Under {
    Scalar(Box<dyn PreparedNativeEffect>),
    Bank(Box<dyn PreparedNativeEffectBank>, BankWidth),
}

impl Under {
    fn new(width: Option<BankWidth>, requests: &[PrepareEffectRequest<'_>]) -> Self {
        match width {
            None => Self::Scalar(
                MultibandCompressorFactory
                    .prepare(requests[0])
                    .expect("prepare")
                    .processor,
            ),
            Some(width) => Self::Bank(bind(width, requests, &vec![true; requests.len()]), width),
        }
    }

    fn payload(&self, lane: usize) -> Payload {
        let sizes = sizes();
        match self {
            Self::Scalar(effect) => payload_of(effect.as_ref(), sizes),
            Self::Bank(bank, _) => bank_payload(bank.as_ref(), lane, sizes).expect("snapshot"),
        }
    }

    /// Lane `lane`'s link weight words on both channels: `w_link` then `w_avg`, each `current,
    /// target, step, remaining`.
    fn link_words(&self, lane: usize) -> [[u32; 8]; 2] {
        let payload = self.payload(lane);
        [&payload.1, &payload.2].map(|section| link_section(section))
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

fn sizes() -> StatePayloadSizes {
    MULTIBAND_COMPRESSOR_DESCRIPTOR.qualities[1].maximum_state
}

/// The eight link weight words of one payload section.
fn link_section(section: &[u8]) -> [u32; 8] {
    core::array::from_fn(|word| {
        let at = (LINK_WORD + word) * 4;
        u32::from_le_bytes(section[at..at + 4].try_into().expect("a word"))
    })
}

/// The link words of a lane at rest at `mode`.
fn rest_words(mode: LinkMode) -> [u32; 8] {
    let [link, average] = link_glide_weights(mode).map(f32::to_bits);
    [link, link, 0, 0, average, average, 0, 0]
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

/// The descriptor's defaults: a 1 kHz crossover and both bands compressing from -18 dB.
fn defaults() -> Values {
    core::array::from_fn(|slot| InitialParameterValue {
        parameter_index: (slot / 2) as u32,
        channel: if slot.is_multiple_of(2) {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: MULTIBAND_COMPRESSOR_PARAMETERS[slot / 2].default_value,
    })
}

/// A low-threshold point on both channels at `value`.
fn threshold_spans(value: f32) -> Vec<PreparedAutomationSpan> {
    [ParameterChannel::Left, ParameterChannel::Right]
        .into_iter()
        .map(|channel| PreparedAutomationSpan {
            channel,
            parameter_index: 1,
            sample: 0,
            value,
        })
        .collect()
}

/// Lane `lane`'s levels of one recorded band under its own mode's prepared law.
fn law(words: &[[f32; 8]; 4], lane: usize, mode: LinkMode) -> (u32, u32) {
    let lanes = LinkLanes::<f32>::new(&[mode]);
    let (near, far) = link_levels::<f32, LINK_MIXED>(words[0][lane], words[1][lane], &lanes);
    (near.to_bits(), far.to_bits())
}

fn bits(values: &[f32]) -> Vec<u32> {
    values.iter().map(|value| value.to_bits()).collect()
}

/// **#1367 gate 3.** Each glide direction, over 64 samples and over 4,800 samples, retargeted at
/// a block boundary and crossed by a low-threshold point on every lane (so the segments are cut
/// mid-glide), on channels 20 dB apart: while it glides, both bands' level words of the gliding
/// lane match the `f64` blend of its band words under the reference weights, and from the arrival
/// frame each is the target mode's prepared law bit for bit, as every bank-mate's is its own
/// mode's on every frame; the glide arrives exactly. At the scalar instance, `Simd4` and `Simd8`.
#[test]
fn a_glide_tracks_the_f64_blend_into_the_target_law() {
    const RETARGET_BLOCK: usize = 4;
    /// Relative to the larger band word: the production weights' iterated additions round each
    /// step (at most half an ulp of a word in `[0, 1]`, `2^-25`, per frame), so 4,800 steps bound
    /// a weight's drift by `1.5e-4`; the deviation stays well inside that worst case.
    const TOLERANCE: f64 = 1.5e-4;
    let values = defaults();
    let mut worst = 0.0_f64;
    for width in widths() {
        let lanes = lanes(width);
        for (samples, crossing) in [(64_u32, 1_usize), (4_800, 70)] {
            for from in LINKS {
                for to in LINKS.into_iter().filter(|mode| *mode != from) {
                    let context = format!("{width:?} {samples} {from:?} -> {to:?}");
                    let modes: Vec<LinkMode> = (0..lanes)
                        .map(|lane| if lane == 0 { from } else { LINKS[lane % 3] })
                        .collect();
                    let requests: Vec<_> = modes
                        .iter()
                        .map(|mode| request(&values, RATE, *mode))
                        .collect();
                    let mut under = Under::new(width, &requests);
                    let start = link_glide_weights(from).map(f64::from);
                    let target = link_glide_weights(to).map(f64::from);
                    let mut reference = ReferenceLinkGlide::new(start, target, samples);
                    let weights: Vec<([f64; 2], bool)> = (0..samples + BLOCK as u32)
                        .map(|_| reference.next_weights())
                        .collect();
                    let retarget_frame = RETARGET_BLOCK * BLOCK;
                    let blocks = RETARGET_BLOCK + samples as usize / BLOCK + 3;
                    let mut gliding_frames = 0;
                    for block in 0..blocks {
                        let first = block * BLOCK;
                        let inputs: Vec<(Vec<f32>, Vec<f32>)> = (0..lanes)
                            .map(|lane| {
                                let seed = (block * lanes + lane) as u64;
                                (noise(seed, BLOCK, 0.5), noise(seed + 9_000, BLOCK, 0.05))
                            })
                            .collect();
                        let spans: Vec<Vec<PreparedAutomationSpan>> = (0..lanes)
                            .map(|_| {
                                if block == RETARGET_BLOCK + crossing {
                                    threshold_spans(-27.5)
                                } else {
                                    Vec::new()
                                }
                            })
                            .collect();
                        let (flat, offsets) = flatten(&spans, first as u64);
                        let mut left =
                            interleave(&inputs.iter().map(|(l, _)| l.clone()).collect::<Vec<_>>());
                        let mut right =
                            interleave(&inputs.iter().map(|(_, r)| r.clone()).collect::<Vec<_>>());
                        if block == RETARGET_BLOCK {
                            assert!(under.retarget(0, to, samples), "{context}: a glide");
                        }
                        let record = recorded(|| {
                            under.process(&mut left, &mut right, first as u64, &flat, &offsets);
                        });
                        if record.is_empty() {
                            continue;
                        }
                        assert_eq!(record.len(), 2 * BLOCK, "{context} block {block}");
                        for (index, words) in record.iter().enumerate() {
                            let frame = first + index / 2;
                            let (near, far) = (words[2][0], words[3][0]);
                            let (w, in_flight) = weights[frame - retarget_frame];
                            if in_flight {
                                gliding_frames += usize::from(index % 2 == 0);
                                let (left, right) =
                                    (f64::from(words[0][0]), f64::from(words[1][0]));
                                let (e_near, e_far) = reference_link_blend(left, right, w);
                                let scale = left.abs().max(right.abs()).max(1.0e-30);
                                for (got, expected) in [(near, e_near), (far, e_far)] {
                                    let deviation = (f64::from(got) - expected).abs() / scale;
                                    worst = worst.max(deviation);
                                    assert!(
                                        deviation <= TOLERANCE,
                                        "{context} frame {frame}: {got} against {expected}"
                                    );
                                }
                            } else {
                                assert_eq!(
                                    (near.to_bits(), far.to_bits()),
                                    law(words, 0, to),
                                    "{context} frame {frame}: not the target law"
                                );
                            }
                            for (lane, mode) in modes.iter().enumerate().skip(1) {
                                assert_eq!(
                                    (words[2][lane].to_bits(), words[3][lane].to_bits()),
                                    law(words, lane, *mode),
                                    "{context} frame {frame} lane {lane}: a resting lane left \
                                     its law"
                                );
                            }
                        }
                    }
                    assert_eq!(
                        gliding_frames,
                        samples as usize - 1,
                        "{context}: every frame in flight took the blend"
                    );
                    assert_eq!(
                        under.link_words(0),
                        [rest_words(to); 2],
                        "{context}: the glide arrived exactly"
                    );
                }
            }
        }
    }
    eprintln!("worst relative deviation from the f64 blend: {worst:e}");
}

/// The law's exact half (#1367 gate 3): from the frame a glide arrives on, a lane's level is its
/// target mode's prepared law bit for bit, and a lane resting beside a glide takes its own mode's
/// word on every frame, non-finite partner words included.
#[test]
fn a_glide_lands_on_its_target_law_and_a_resting_lane_never_leaves_its_own() {
    fn check<L: Lane>() {
        const SAMPLES: u32 = 64;
        const FRAMES: u32 = 96;
        for from in LINKS {
            for to in LINKS.into_iter().filter(|mode| *mode != from) {
                let mut modes: [LinkMode; 8] = core::array::from_fn(|lane| LINKS[lane % 3]);
                modes[0] = from;
                let mut ramps: [[LinearRamp; LINK_WEIGHTS]; 8] = core::array::from_fn(|lane| {
                    link_glide_weights(modes[lane]).map(LinearRamp::fixed)
                });
                modes[0] = to;
                for (ramp, weight) in ramps[0].iter_mut().zip(link_glide_weights(to)) {
                    ramp.set_target(weight, SAMPLES);
                }
                let lanes = LinkLanes::<L>::new(&modes[..L::WIDTH]);
                for frame in 1..=FRAMES {
                    // The frame's weights and which lanes still glide after it, as the segment
                    // split hands them to the kernel.
                    let mut weights = GlideWeights::<L>::idle();
                    let mut gliding = 0_u32;
                    for (lane, lane_ramps) in ramps.iter_mut().enumerate().take(L::WIDTH) {
                        for (weight, ramp) in lane_ramps.iter_mut().enumerate() {
                            let mut words = [0.0_f32; 8];
                            weights.current[weight].store(&mut words[..L::WIDTH]);
                            words[lane] = ramp.next_value();
                            weights.current[weight] = L::load(&words[..L::WIDTH]);
                            if ramp.is_ramping() {
                                gliding |= 1 << lane;
                            }
                        }
                    }
                    weights.gliding = lanes_mask::<L>(gliding);
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
                    // `0 * inf` and `inf - inf` are NaN, so only the `select` keeps a resting
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
                    let law = link_levels::<L, LINK_MIXED>(own, partner, &lanes);
                    let got = glide_levels::<L>(own, partner, &lanes, &weights, &weights);
                    let lane_bits = |value: L| {
                        let mut out = [0_u32; 8];
                        value.store_bits(&mut out[..L::WIDTH]);
                        out
                    };
                    for (got, law) in [(got.0, law.0), (got.1, law.1)] {
                        let (got, law) = (lane_bits(got), lane_bits(law));
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
                assert!(
                    ramps.iter().flatten().all(|ramp| !ramp.is_ramping()),
                    "the glide arrived"
                );
            }
        }
    }
    lane::each_lane!(|L| check::<L>());
}

/// **#1367 gates 3 and 6.** A full bank of distinct members at mixed modes, under drawn
/// automation on every lane, where lane 0 glides: every lane, the gliding one included, renders
/// bit for bit as its own scalar instance given the same glide (the bank's segments are cut by
/// its bank-mates' ramps, the scalar's are not), and the retarget and the switching blocks
/// allocate and free nothing. At `Simd4` and `Simd8`.
#[test]
fn a_glide_moves_no_bank_mates_bit_and_allocates_nothing() {
    bench_support::alloc::assert_installed();
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        for seed in 0..4_u64 {
            let mut draw = Draw::new(0x1367_b000 + seed);
            let members: Vec<Values> = (0..lanes).map(|_| draw_values(&mut draw)).collect();
            let modes: Vec<LinkMode> = (0..lanes)
                .map(|lane| LINKS[(lane + seed as usize) % LINKS.len()])
                .collect();
            let target = LINKS[(seed as usize + 1) % LINKS.len()];
            let samples = [64, 300, 1_000, 5][seed as usize];
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
            let glide_block = 3;
            for block in 0..40 {
                let first_sample = (block * BLOCK) as u64;
                let inputs: Vec<(Vec<f32>, Vec<f32>)> = (0..lanes)
                    .map(|lane| {
                        let salt = seed * 10_000 + (block * lanes + lane) as u64;
                        (noise(salt, BLOCK, 0.5), noise(salt + 5_000, BLOCK, 0.05))
                    })
                    .collect();
                let spans: Vec<Vec<PreparedAutomationSpan>> = (0..lanes)
                    .map(|_| {
                        if draw.chance(1, 3) {
                            draw_spans(&mut draw, first_sample)
                        } else {
                            Vec::new()
                        }
                    })
                    .collect();
                let (flat, offsets) = flatten(&spans, first_sample);
                let mut left =
                    interleave(&inputs.iter().map(|(l, _)| l.clone()).collect::<Vec<_>>());
                let mut right =
                    interleave(&inputs.iter().map(|(_, r)| r.clone()).collect::<Vec<_>>());
                let mark = bench_support::alloc::current_thread_counters();
                if block == glide_block {
                    assert!(bank.retarget(0, target, samples), "a supported glide");
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
                if block == glide_block {
                    assert!(scalars[0].retarget(0, target, samples));
                }
                for (lane, scalar) in scalars.iter_mut().enumerate() {
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
                            "{width:?} seed {seed} block {block} lane {lane} frame {frame}: the \
                             bank lane parted from its scalar instance"
                        );
                    }
                }
            }
            assert_eq!(bank.link_words(0), [rest_words(target); 2], "arrived");
        }
    }
}

/// **#1367 gate 4.** A lane snapshotted mid-glide and restored into an instance prepared with the
/// glide's target mode renders bit for bit as the uninterrupted one, on a scalar instance and on a
/// bank lane. A payload restored under another mode leaves that mode at rest, and a forged
/// weight (off `[0, 1]`, `-0.0`, a settled weight off its target, a moving one with no step, a
/// target that is no mode's) or channels at two modes are refused and change nothing.
#[test]
fn a_lane_snapshotted_mid_glide_continues_bit_for_bit_and_forged_weights_are_refused() {
    let values = defaults();
    let sizes = sizes();
    let render = |under: &mut Under, lanes: usize, range: core::ops::Range<usize>| {
        let mut out = Vec::new();
        for first in range.step_by(BLOCK) {
            let mut left = interleave(&vec![noise(first as u64, BLOCK, 0.5); lanes]);
            let mut right = interleave(&vec![noise(first as u64 + 1, BLOCK, 0.05); lanes]);
            under.process(
                &mut left,
                &mut right,
                first as u64,
                &[],
                &vec![0; lanes + 1],
            );
            out.push((bits(&left), bits(&right)));
        }
        out
    };
    let restore = |under: &mut Under, lane: usize, payload: &Payload| {
        let input =
            StatePayloadInput::new(&payload.0, &payload.1, &payload.2, sizes).expect("sizes");
        match under {
            Under::Scalar(effect) => effect.restore_state_payload(1, input),
            Under::Bank(bank, _) => bank.restore_track_state_payload(lane as u32, 1, input),
        }
    };
    for width in widths() {
        let lanes = lanes(width);
        let prepare = |mode| Under::new(width, &vec![request(&values, RATE, mode); lanes]);
        let mut source = prepare(LinkMode::DualMono);
        render(&mut source, lanes, 0..10 * BLOCK);
        assert!(source.retarget(0, LinkMode::Average, 96));
        render(&mut source, lanes, 10 * BLOCK..11 * BLOCK);
        let payload = source.payload(0);
        let mut successor = prepare(LinkMode::DualMono);
        // The carry seats the predecessor's mode first (#1371 Amendment 2), then restores; the
        // resting bank-mates carry their own payloads.
        assert!(successor.retarget(0, LinkMode::Average, 0));
        restore(&mut successor, 0, &payload).expect("a mid-glide payload restores");
        for lane in 1..lanes {
            restore(&mut successor, lane, &source.payload(lane)).expect("a resting lane");
        }
        assert_eq!(
            render(&mut successor, lanes, 11 * BLOCK..20 * BLOCK),
            render(&mut source, lanes, 11 * BLOCK..20 * BLOCK),
            "{width:?}: the restored glide parted from the uninterrupted one"
        );

        // Under another mode the lane keeps its own, at rest.
        let mut other = prepare(LinkMode::Maximum);
        restore(&mut other, 0, &payload).expect("restores under another mode");
        assert_eq!(
            other.link_words(0),
            [rest_words(LinkMode::Maximum); 2],
            "{width:?}: a payload of another mode leaves the lane's own mode at rest"
        );

        let mut target = prepare(LinkMode::Average);
        let before = target.payload(0);
        let first = LINK_WORD * 4;
        let mut forgeries: Vec<Payload> = [
            (0, 1.5_f32),
            (0, -0.25),
            (0, -0.0),
            (2, 0.0),
            (1, 0.5),
            (5, 0.5),
        ]
        .into_iter()
        .map(|(word, value)| {
            let mut forged = payload.clone();
            forged.1[first + word * 4..first + word * 4 + 4].copy_from_slice(&value.to_le_bytes());
            forged
        })
        .collect();
        // A settled weight off its target.
        let mut settled = target.payload(0);
        settled.2[first..first + 4].copy_from_slice(&0.5_f32.to_le_bytes());
        forgeries.push(settled);
        // Channels at two modes.
        let mut split = payload.clone();
        for (index, weight) in link_glide_weights(LinkMode::Maximum).iter().enumerate() {
            let at = first + (index * 4 + 1) * 4;
            split.2[at..at + 4].copy_from_slice(&weight.to_le_bytes());
        }
        forgeries.push(split);
        for (index, forged) in forgeries.iter().enumerate() {
            assert_eq!(
                restore(&mut target, 0, forged).expect_err("a forgery").code,
                "effect.state.link",
                "{width:?} forgery {index}"
            );
            assert_eq!(
                target.payload(0),
                before,
                "{width:?} forgery {index}: a refused restore wrote"
            );
        }
    }
}

/// **D1 refusals.** A glide longer than `LINK_GLIDE_MAX_SAMPLES`, a padded bank lane, a lane past
/// the width and track 1 of a scalar instance each return `false` and leave every payload word as
/// it was; the bound itself is accepted.
#[test]
fn an_out_of_range_glide_is_refused_and_changes_nothing() {
    let values = defaults();
    let too_long = effect_contract::LINK_GLIDE_MAX_SAMPLES + 1;
    let scalar_request = request(&values, RATE, LinkMode::DualMono);
    let mut scalar = Under::new(None, core::slice::from_ref(&scalar_request));
    let before = scalar.payload(0);
    assert!(!scalar.retarget(0, LinkMode::Maximum, too_long), "2^24 + 1");
    assert!(!scalar.retarget(1, LinkMode::Maximum, 64), "track 1");
    assert_eq!(scalar.payload(0), before);
    assert!(scalar.retarget(0, LinkMode::Maximum, too_long - 1), "2^24");

    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let requests = vec![request(&values, RATE, LinkMode::DualMono); lanes];
        let mut bank = Under::Bank(bind(width, &requests, &prefix_mask(width, 1)), width);
        let before = bank.payload(0);
        assert!(!bank.retarget(0, LinkMode::Maximum, too_long), "2^24 + 1");
        assert!(!bank.retarget(1, LinkMode::Maximum, 64), "a padded lane");
        assert!(
            !bank.retarget(lanes, LinkMode::Maximum, 64),
            "past the width"
        );
        assert_eq!(bank.payload(0), before);
        assert!(bank.retarget(0, LinkMode::Maximum, too_long - 1), "2^24");
    }
}

/// **A glide in flight is never quiet.** A bank whose every lane is at rest on `+0.0` input holds
/// the silent fixed point (its quiet blocks are skipped); lane 0 glides: `lanes_at_rest` drops the
/// lane while it glides and reports it again from the block it arrives in, its countdown moves on
/// every block, and the glide arrives exactly. At `Simd4` and `Simd8`.
#[test]
fn a_silent_bank_never_skips_a_glide() {
    fn check<L: Lane, const W: usize>() {
        const GLIDE: u32 = 300;
        let values = defaults();
        let requests: Vec<_> = (0..W)
            .map(|lane| request(&values, RATE, LINKS[lane % LINKS.len()]))
            .collect();
        let width = BankWidth::ALL
            .iter()
            .copied()
            .find(|width| width.lanes() as usize == W)
            .expect("a width");
        let mask = vec![true; W];
        let request = PrepareEffectBankRequest {
            backend: width.backend(),
            width,
            requests: &requests,
            active_mask: &mask,
        };
        let (instance, metadata) = bind_instance::<L, W>(&MultibandCompressorFactory, request)
            .expect("binds")
            .expect("one program");
        let mut bank = PreparedMultibandCompressorBank::<L, W> {
            width,
            automation_capacity: metadata.automation_capacity,
            state_sizes: metadata.state_sizes,
            instance,
            silent_fixed_point: false,
        };
        let all = all_lanes::<W>();
        let offsets = vec![0_u32; W + 1];
        let render = |bank: &mut PreparedMultibandCompressorBank<L, W>, first: usize| {
            let (mut left, mut right) = (vec![0.0_f32; BLOCK * W], vec![0.0_f32; BLOCK * W]);
            bank.process_bank(
                EffectBankProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    BLOCK as u32,
                    width,
                    first as u64,
                    &[],
                    &offsets,
                    QUANTUM,
                )
                .expect("a block"),
            );
        };
        render(&mut bank, 0);
        assert_eq!(bank.lanes_at_rest(), all, "W{W}: every lane at rest");
        assert!(
            bank.silent_fixed_point,
            "W{W}: the quiet blocks are skipped"
        );
        assert!(bank.retarget_link(0, LinkMode::Average, GLIDE), "a glide");
        let words = |bank: &PreparedMultibandCompressorBank<L, W>| {
            let payload = bank_payload(bank, 0, sizes()).expect("snapshot");
            [link_section(&payload.1), link_section(&payload.2)]
        };
        for block in 1..=10_u32 {
            render(&mut bank, block as usize * BLOCK);
            let remaining = GLIDE.saturating_sub(block * BLOCK as u32);
            let context = format!("W{W} block {block}");
            for channel in words(&bank) {
                assert_eq!(channel[3], remaining, "{context}: the w_link countdown");
            }
            let expected = if remaining == 0 { all } else { all & !1 };
            assert_eq!(
                bank.lanes_at_rest(),
                expected,
                "{context}: the gliding lane"
            );
        }
        assert_eq!(words(&bank), [rest_words(LinkMode::Average); 2], "arrived");
    }
    lane::each_vector_lane!(|L, W| check::<L, W>());
}

/// **The collapsed body.** A collapse-eligible bank (every lane's channels equal) whose lane 0
/// glides renders collapsed blocks (`process_bank_mono`) that are the left plane of a dual bank
/// fed `L == R`, bit for bit; after the disengage copy the channels agree, the weights are the
/// dual bank's, and the two banks render the same bits. At `Simd4` and `Simd8`.
#[test]
fn a_collapsed_glide_matches_the_dual_glide_and_survives_the_disengage() {
    let values = defaults();
    for &width in BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let requests: Vec<_> = (0..lanes)
            .map(|lane| request(&values, RATE, LINKS[lane % LINKS.len()]))
            .collect();
        let (mut dual, mut collapsed) = (
            Under::new(Some(width), &requests),
            Under::new(Some(width), &requests),
        );
        for under in [&mut dual, &mut collapsed] {
            assert!(under.retarget(0, LinkMode::Maximum, 300), "a glide");
        }
        let offsets = vec![0_u32; lanes + 1];
        for block in 0..16 {
            let first = block * BLOCK;
            let plane = interleave(&vec![noise(first as u64, BLOCK, 0.5); lanes]);
            let (mut dual_left, mut dual_right) = (plane.clone(), plane.clone());
            dual.process(&mut dual_left, &mut dual_right, first as u64, &[], &offsets);
            let (mut mono_left, mut mono_right) = (plane.clone(), plane);
            let (Under::Bank(bank, _), true) = (&mut collapsed, block < 4) else {
                collapsed.process(&mut mono_left, &mut mono_right, first as u64, &[], &offsets);
                assert_eq!(
                    (bits(&mono_left), bits(&mono_right)),
                    (bits(&dual_left), bits(&dual_right)),
                    "{width:?} block {block}: the disengaged bank parted from the dual one"
                );
                continue;
            };
            bank.process_bank_mono(
                EffectBankProcessBlock::new(
                    &mut mono_left,
                    &mut mono_right,
                    None,
                    BLOCK as u32,
                    width,
                    first as u64,
                    &[],
                    &offsets,
                    QUANTUM,
                )
                .expect("a well-shaped bank block"),
            );
            assert_eq!(
                bits(&mono_left),
                bits(&dual_left),
                "{width:?} block {block}: a collapsed glide block is not the dual one"
            );
            if block == 3 {
                bank.desymmetrize_channels();
                assert!(bank.channels_agree(), "{width:?}: the disengage copy");
                for lane in 0..lanes {
                    assert_eq!(
                        collapsed.link_words(lane),
                        dual.link_words(lane),
                        "{width:?} lane {lane}: the disengaged weights"
                    );
                }
            }
        }
    }
}
