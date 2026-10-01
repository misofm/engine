//! Signed zeros and subnormals survive the limiter delay line under both links and collapse.
//!
//! A NaN in track 1's left input trips recovery after the declared latency, zeroing that lane's
//! failing block and subsequent line fill only. Limiting, signed-zero and subnormal observations
//! keep every arm non-vacuous; current differential tests own PCM, reports and state equality.

use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ObservationSample, ParameterChannel, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPorts, PreparedSidechainPort,
};
use lane::Backend;
use true_peak_limiter::{TRUE_PEAK_LIMITER_DESCRIPTOR, TruePeakLimiterFactory};

const FRAMES: usize = 128;
const BLOCKS: usize = 96;
/// Tracks rendered through scalar instances, one instance each: as many as the widest bank.
const SCALAR_TRACKS: usize = 8;
/// The block that carries the one NaN, and where it sits.
const NAN_BLOCK: usize = 56;
const NAN_TRACK: usize = 1;
const NAN_FRAME: usize = 37;

/// What block `block` feeds every track.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Content {
    Noise,
    SignedZeros,
    Subnormals,
}

/// The schedule. Each non-noise run follows limiting noise, so the detector history turns over
/// from full-scale words into zeros or subnormals while the gain path is still releasing, and
/// back.
fn content(block: usize) -> Content {
    match block {
        24..=39 | 72..=79 => Content::SignedZeros,
        40..=55 | 80..=87 => Content::Subnormals,
        _ => Content::Noise,
    }
}

/// The console fixture's per-track limiter shape: ceiling, release and a 5 ms lookahead, the same
/// on both channels.
fn values(track: usize) -> [InitialParameterValue; 6] {
    let ceiling = -0.5 - 0.031_25 * track as f32;
    let release = 60.0 + 1.25 * track as f32;
    let lookahead = 5.0;
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: [ceiling, release, lookahead][index / 2],
    })
}

fn request(values: &[InitialParameterValue], link_mode: LinkMode) -> PrepareEffectRequest<'_> {
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
        link_mode,
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

/// SplitMix64 of one track, channel and absolute frame. Seeded by track and channel, never by
/// width, so track `k` hears the same samples in every arm.
fn hash(track: usize, channel: usize, frame: usize) -> u64 {
    let mut state = 0x1013_0000_u64 ^ ((track as u64) << 32) ^ ((channel as u64) << 24);
    state = state.wrapping_add((frame as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    let mut mixed = state;
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    mixed ^ (mixed >> 31)
}

/// The input sample of one track, channel and absolute frame.
fn sample(track: usize, channel: usize, frame: usize) -> f32 {
    let block = frame / FRAMES;
    if block == NAN_BLOCK && track == NAN_TRACK && channel == 0 && frame % FRAMES == NAN_FRAME {
        return f32::NAN;
    }
    let bits = hash(track, channel, frame);
    match content(block) {
        // +3 dBFS peak.
        Content::Noise => {
            let unit = ((bits >> 40) as f32 * (1.0 / 16_777_216.0)) * 2.0 - 1.0;
            unit * 1.412_537_5
        }
        Content::SignedZeros => f32::from_bits(((bits >> 63) as u32) << 31),
        // Sign and mantissa random, exponent field zero. A zero mantissa (a signed zero) is
        // possible and kept.
        Content::Subnormals => f32::from_bits((bits as u32) & 0x807F_FFFF),
    }
}

/// The fixed latency at 48 kHz, `N + 6` samples.
fn latency() -> usize {
    TRUE_PEAK_LIMITER_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("launch rate")
        .latency
        .0 as usize
}

/// How many whole blocks of line fill an empty line puts out before the first input word arrives.
fn fill_blocks() -> usize {
    latency() / FRAMES
}

/// The blocks a §4.4 reset zeroes: the block the NaN reaches the output in, which the check zeroes,
/// and the line fill of the reset instance after it.
fn reset_blocks() -> Vec<usize> {
    let reset = (NAN_BLOCK * FRAMES + NAN_FRAME + latency()) / FRAMES;
    (reset..=reset + fill_blocks()).collect()
}

/// What the non-vacuity checks read: the deepest reduction observed, how many output words were
/// `-0.0` and subnormal, and which `(block, lane)` pairs came out entirely `+0.0`.
///
/// The limiter's §4.4 reset is not in its `ProcessReport` (the counter is the instance's own
/// instrumentation), so the host-visible trace of it is the one it leaves in the audio: a whole
/// block of `+0.0` on the failed lane, then the line fill of its emptied line. Past the first line
/// fill no other lane's block of this scenario is entirely `+0.0`: a noise block's words are
/// nonzero, a signed-zero block's carry `-0.0` and a subnormal block's carry subnormals.
#[derive(Default)]
struct Witness {
    deepest: f32,
    negative_zeros: usize,
    subnormals: usize,
    zeroed: Vec<(usize, usize)>,
}

impl Witness {
    /// One block of `planes`, each `lanes` wide and lane-interleaved (`lanes == 1` for a scalar
    /// instance).
    fn block(&mut self, block: usize, planes: &[&[f32]], lanes: usize) {
        for word in planes.iter().flat_map(|plane| plane.iter()) {
            if word.to_bits() == 0x8000_0000 {
                self.negative_zeros += 1;
            }
            if word.is_subnormal() {
                self.subnormals += 1;
            }
        }
        for lane in 0..lanes {
            let zeroed = planes.iter().all(|plane| {
                plane
                    .iter()
                    .skip(lane)
                    .step_by(lanes)
                    .all(|word| word.to_bits() == 0)
            });
            if block >= fill_blocks() && zeroed {
                self.zeroed.push((block, lane));
            }
        }
    }
}

/// How a bank renders: through `process_bank` under a link mode, or collapsed.
#[derive(Clone, Copy)]
enum Body {
    Dual(LinkMode),
    Collapsed,
}

/// Runs the scenario through one bank of `width` under `body`.
fn bank_run(width: BankWidth, backend: Backend, body: Body) -> Witness {
    let lanes = width.lanes() as usize;
    let link_mode = match body {
        Body::Dual(link_mode) => link_mode,
        Body::Collapsed => LinkMode::Maximum,
    };
    let tables: Vec<_> = (0..lanes).map(values).collect();
    let requests: Vec<_> = tables
        .iter()
        .map(|values| request(values, link_mode))
        .collect();
    let mut bank: Box<dyn PreparedNativeEffectBank> = TruePeakLimiterFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend,
            width,
            requests: &requests,
            active_mask: width.full_mask(),
        })
        .expect("valid bank request")
        .expect("the width must bind");
    if let Body::Collapsed = body {
        assert!(bank.supports_mono_collapse(), "the limiter collapses");
    }
    let offsets = vec![0_u32; lanes + 1];
    let mut witness = Witness::default();
    for block in 0..BLOCKS {
        let mut left = vec![0.0_f32; FRAMES * lanes];
        let mut right = vec![0.0_f32; FRAMES * lanes];
        for frame in 0..FRAMES {
            for lane in 0..lanes {
                let at = block * FRAMES + frame;
                left[frame * lanes + lane] = sample(lane, 0, at);
                right[frame * lanes + lane] = match body {
                    Body::Dual(_) => sample(lane, 1, at),
                    // Never gathered by the collapsed body; a finite poison makes a read visible.
                    Body::Collapsed => f32::from_bits(0x7F7F_FFFF),
                };
            }
        }
        let block_request = EffectBankProcessBlock::new(
            &mut left,
            &mut right,
            None,
            FRAMES as u32,
            width,
            (block * FRAMES) as u64,
            &[],
            &offsets,
            FRAMES as u32,
        )
        .expect("bank block");
        match body {
            Body::Dual(_) => bank.process_bank(block_request),
            Body::Collapsed => bank.process_bank_mono(block_request),
        };
        match body {
            Body::Dual(_) => {
                witness.block(block, &[&left, &right], lanes);
            }
            Body::Collapsed => {
                witness.block(block, &[&left], lanes);
                // Make the right state current before reading the two-channel resident tap.
                bank.desymmetrize_channels();
            }
        }
        let mut observed = vec![ObservationSample::default(); lanes];
        assert!(bank.observe_resident_bank(0, &mut observed), "resident tap");
        for sample in &observed {
            witness.deepest = witness.deepest.max(sample.left).max(sample.right);
        }
    }
    witness
}

/// Runs the scenario's track `track` through one scalar instance under `link_mode`.
fn scalar_run(track: usize, link_mode: LinkMode) -> Witness {
    let table = values(track);
    let mut effect: Box<dyn PreparedNativeEffect> = TruePeakLimiterFactory
        .prepare(request(&table, link_mode))
        .expect("prepare");
    let mut witness = Witness::default();
    for block in 0..BLOCKS {
        let mut left: Vec<f32> = (0..FRAMES)
            .map(|frame| sample(track, 0, block * FRAMES + frame))
            .collect();
        let mut right: Vec<f32> = (0..FRAMES)
            .map(|frame| sample(track, 1, block * FRAMES + frame))
            .collect();
        effect.process(
            EffectProcessBlock::new(
                &mut left,
                &mut right,
                None,
                (block * FRAMES) as u64,
                &[],
                FRAMES as u32,
            )
            .expect("block"),
        );
        witness.block(block, &[&left, &right], 1);
        let mut observed = ObservationSample::default();
        assert!(effect.observe_resident(0, &mut observed), "resident tap");
        witness.deepest = witness.deepest.max(observed.left).max(observed.right);
    }
    witness
}

/// Non-vacuity of one run. The scenario is about a limiter that limits, that carries signed zeros
/// and subnormals through its line, and whose §4.4 reset fires once, on lane `reset_lane` only.
fn check_witness(label: &str, witness: &Witness, reset_lane: Option<usize>) {
    assert!(
        witness.deepest > 0.1,
        "{label}: the scenario never limited (deepest {})",
        witness.deepest
    );
    assert!(
        witness.negative_zeros > 0,
        "{label}: no -0.0 reached the output"
    );
    assert!(
        witness.subnormals > 0,
        "{label}: no subnormal reached the output"
    );
    let expected: Vec<(usize, usize)> = reset_lane.map_or_else(Vec::new, |lane| {
        reset_blocks()
            .into_iter()
            .map(|block| (block, lane))
            .collect()
    });
    assert_eq!(
        witness.zeroed, expected,
        "{label}: (block, lane) zeroed by the §4.4 reset"
    );
}

#[test]
fn signed_zeros_subnormals_and_lane_local_recovery_survive_the_delay() {
    for &width in BankWidth::ALL.iter().rev() {
        for (name, body) in [
            ("maximum", Body::Dual(LinkMode::Maximum)),
            ("dual_mono", Body::Dual(LinkMode::DualMono)),
            ("collapsed", Body::Collapsed),
        ] {
            let witness = bank_run(width, width.backend(), body);
            check_witness(&format!("{width:?} {name}"), &witness, Some(NAN_TRACK));
        }
    }
    for (name, link_mode) in [
        ("maximum", LinkMode::Maximum),
        ("dual_mono", LinkMode::DualMono),
    ] {
        for track in 0..SCALAR_TRACKS {
            let witness = scalar_run(track, link_mode);
            check_witness(
                &format!("scalar {name} track {track}"),
                &witness,
                (track == NAN_TRACK).then_some(0),
            );
        }
    }
}
