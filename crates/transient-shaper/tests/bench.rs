//! MQ-1: frozen, descriptive in-kernel throughput measurement for the transient shaper.
//!
//! Run only through `scripts/run-issue880-mq1-benchmark.sh`. One invocation prepares the production
//! Simd8 bank and scalar instance, then runs one warmup and two measured rounds for each. A
//! lane-sample is one track frame; both dual-mono channels are processed for each lane-sample.

use std::time::Instant;

use effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedNativeEffect, PreparedNativeEffectBank, PreparedPorts,
    PreparedSidechainPort, ResetKind,
};
use lane::Backend;
use sha2::{Digest, Sha256};
use transient_shaper::{TRANSIENT_SHAPER_DESCRIPTOR, TransientShaperFactory};

const SAMPLE_RATE: u32 = 48_000;
const BLOCK_FRAMES: usize = 128;
const SECONDS: usize = 4;
const FRAMES: usize = SAMPLE_RATE as usize * SECONDS;
const BLOCKS: usize = FRAMES / BLOCK_FRAMES;
const LANES: usize = 8;
const ATTACK: f32 = 0.75;
const SUSTAIN: f32 = -0.5;
const MIX: f32 = 1.0;
const LEFT_SEED: u64 = 0x8800_0001;
const RIGHT_SEED: u64 = 0x8800_0002;

fn values() -> [InitialParameterValue; 6] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index % 2 == 0 {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: [ATTACK, ATTACK, SUSTAIN, SUSTAIN, MIX, MIX][index],
    })
}

fn request(values: &[InitialParameterValue]) -> PrepareEffectRequest<'_> {
    let quality = TRANSIENT_SHAPER_DESCRIPTOR
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == SAMPLE_RATE)
        .expect("frozen 48 kHz launch rate");
    PrepareEffectRequest {
        sample_rate: SAMPLE_RATE,
        quantum: BLOCK_FRAMES as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
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

/// SplitMix64 with a frozen seed gives sparse impulses, exponential decays and a quiet noise bed.
fn next_random(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut mixed = *state;
    mixed = (mixed ^ (mixed >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    mixed = (mixed ^ (mixed >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    mixed ^ (mixed >> 31)
}

fn programme(frames: usize, lanes: usize, seed: u64) -> Vec<f32> {
    let mut samples = vec![0.0; frames * lanes];
    for lane in 0..lanes {
        let mut state = seed ^ (lane as u64).wrapping_mul(0xd6e8_feb8_6659_fd93);
        let mut decay = 0.0_f32;
        for frame in 0..frames {
            let random = next_random(&mut state);
            if random & 0x7ff == 0 {
                let sign = if random & 0x8000 == 0 { 1.0 } else { -1.0 };
                let amplitude = 0.25 + ((random >> 16) as u32 as f32 / u32::MAX as f32) * 0.7;
                decay += sign * amplitude;
            }
            decay *= 0.985;
            let bed = ((random >> 40) as u32 as f32 / u32::MAX as f32 - 0.5) * 0.004;
            samples[frame * lanes + lane] = decay + bed;
        }
    }
    samples
}

fn first_lane(samples: &[f32]) -> Vec<f32> {
    samples.iter().step_by(LANES).copied().collect()
}

fn fixture_digest(left: &[f32], right: &[f32]) -> String {
    let mut hasher = Sha256::new();
    for sample in left.iter().chain(right) {
        hasher.update(sample.to_bits().to_le_bytes());
    }
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        write!(hex, "{byte:02x}").expect("write to string");
    }
    hex
}

fn bank_round(
    bank: &mut dyn PreparedNativeEffectBank,
    source_left: &[f32],
    source_right: &[f32],
) -> f64 {
    bank.reset(ResetKind::FullToDefaults);
    let mut left = source_left.to_vec();
    let mut right = source_right.to_vec();
    let offsets = [0_u32; LANES + 1];

    let start = Instant::now();
    for block in 0..BLOCKS {
        let first = block * BLOCK_FRAMES * LANES;
        let end = first + BLOCK_FRAMES * LANES;
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut left[first..end],
                &mut right[first..end],
                None,
                BLOCK_FRAMES as u32,
                BankWidth::Eight,
                (block * BLOCK_FRAMES) as u64,
                &[],
                &offsets,
                BLOCK_FRAMES as u32,
            )
            .expect("valid frozen bank block"),
        );
    }
    start.elapsed().as_secs_f64() * 1.0e9 / (FRAMES * LANES) as f64
}

fn scalar_round(
    effect: &mut dyn PreparedNativeEffect,
    source_left: &[f32],
    source_right: &[f32],
) -> f64 {
    effect.reset(ResetKind::FullToDefaults);
    let mut left = source_left.to_vec();
    let mut right = source_right.to_vec();

    let start = Instant::now();
    for block in 0..BLOCKS {
        let first = block * BLOCK_FRAMES;
        let end = first + BLOCK_FRAMES;
        effect.process(
            EffectProcessBlock::new(
                &mut left[first..end],
                &mut right[first..end],
                None,
                first as u64,
                &[],
                BLOCK_FRAMES as u32,
            )
            .expect("valid frozen scalar block"),
        );
    }
    start.elapsed().as_secs_f64() * 1.0e9 / FRAMES as f64
}

#[test]
#[ignore = "descriptive benchmark; run once through the Issue-880 MQ-1 operator script"]
fn mq1_transient_shaper_ns_per_lane_sample() {
    assert_eq!(
        Backend::current(),
        Backend::Simd8,
        "MQ-1 requires native Simd8"
    );
    assert_eq!(FRAMES % BLOCK_FRAMES, 0);

    let values = values();
    let requests: [PrepareEffectRequest<'_>; LANES] = core::array::from_fn(|_| request(&values));
    let mut bank = TransientShaperFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd8,
            width: BankWidth::Eight,
            requests: &requests,
        })
        .expect("Simd8 bank binding")
        .expect("native Simd8 bank available");
    let mut scalar = TransientShaperFactory
        .prepare(request(&values))
        .expect("scalar prepare");

    let left = programme(FRAMES, LANES, LEFT_SEED);
    let right = programme(FRAMES, LANES, RIGHT_SEED);
    let fixture_sha256 = fixture_digest(&left, &right);
    let scalar_left = first_lane(&left);
    let scalar_right = first_lane(&right);

    let _bank_warmup = bank_round(&mut *bank, &left, &right);
    let bank_first = bank_round(&mut *bank, &left, &right);
    let bank_second = bank_round(&mut *bank, &left, &right);
    let _scalar_warmup = scalar_round(&mut *scalar, &scalar_left, &scalar_right);
    let scalar_first = scalar_round(&mut *scalar, &scalar_left, &scalar_right);
    let scalar_second = scalar_round(&mut *scalar, &scalar_left, &scalar_right);

    println!(
        "MQ1_RESULT {{\"fixture_sha256\":\"{fixture_sha256}\",\"bank_ns_per_lane_sample\":[{bank_first:.6},{bank_second:.6}],\"scalar_ns_per_lane_sample\":[{scalar_first:.6},{scalar_second:.6}]}}"
    );
}
