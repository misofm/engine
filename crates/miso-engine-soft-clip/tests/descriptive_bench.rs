//! Descriptive throughput of the production `process_bank` shape (issue #91, plan step 6.0).
//!
//! Not a gate. One warmup and two measured rounds, per `AGENTS.md`; nothing but the prepared
//! bank's own render entry is inside the timed region, and no hashing, allocation or checking
//! happens there. The number reported is nanoseconds per track-channel-sample, which is the unit
//! the audit measured the old kernel in (296 ns for the W8 bank).
//!
//! Measured on the delivery host (`x86_64`, Zen 5 class, release profile with the workspace
//! `x86-64-v3` pin), W8 bank, 128-frame blocks, drive +12 dB:
//!
//! | | round 0 | round 1 |
//! |---|---:|---:|
//! | before (issue #91's five hand-written copies) | 246.229 ns | 246.587 ns |
//! | after (one polyphase block kernel) | 4.346 ns | 3.904 ns |
//!
//! That is 63x, against the plan's expectation of at least 20x. The auditor measured 296 ns and a
//! 3.7 ns polyphase reference on a Ryzen 9700X, so the ratio reproduces on a second machine.
//!
//! Run with:
//! `cargo test --release -p miso-engine-soft-clip --test descriptive_bench -- --ignored --nocapture`

use std::time::Instant;

use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectQuality, InitialParameterValue, LinkMode,
    NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PrepareEffectLimits,
    PrepareEffectRequest, PreparedNativeEffectBank, PreparedPortsV1, PreparedSidechainPort,
};
use miso_engine_soft_clip::{SOFT_CLIP_PARAMETERS_V1, SoftClipFactory};

const LANES: usize = 8;
const FRAMES: usize = 128;
const WARMUP_BLOCKS: usize = 200;
const ROUND_BLOCKS: usize = 4_000;

fn initial_values(drive_db: f32) -> Vec<InitialParameterValue> {
    (0..SOFT_CLIP_PARAMETERS_V1.len() * 2)
        .map(|index| InitialParameterValue {
            parameter_index: (index / 2) as u32,
            channel: if index.is_multiple_of(2) {
                ParameterChannel::Left
            } else {
                ParameterChannel::Right
            },
            value: if index / 2 == 0 {
                drive_db
            } else {
                SOFT_CLIP_PARAMETERS_V1[index / 2].default_value
            },
        })
        .collect()
}

fn request<'a>(values: &'a [InitialParameterValue]) -> PrepareEffectRequest<'a> {
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: FRAMES as u32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 16,
            maximum_automation_spans_per_block: 16,
        },
    }
}

fn signal(index: usize) -> f32 {
    (index as f32 * 0.073).sin() * 0.8 + (index as f32 * 0.017_1).sin() * 0.3
}

#[test]
#[ignore = "descriptive benchmark; run explicitly in release"]
fn descriptive_bank_throughput() {
    let values = initial_values(12.0);
    let requests: Vec<PrepareEffectRequest<'_>> = (0..LANES).map(|_| request(&values)).collect();
    let mut bank: Box<dyn PreparedNativeEffectBank> = SoftClipFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: bench_backend(),
            width: BankWidth::Eight,
            requests: &requests,
        })
        .expect("bind soft-clip bank")
        .expect("soft-clip bank is available on this host");

    let mut left = vec![0.0_f32; FRAMES * LANES];
    let mut right = vec![0.0_f32; FRAMES * LANES];
    for frame in 0..FRAMES {
        for lane in 0..LANES {
            left[frame * LANES + lane] = signal(frame * LANES + lane);
            right[frame * LANES + lane] = signal(frame * LANES + lane + 7);
        }
    }
    let offsets = [0_u32; LANES + 1];

    let mut first_sample = 0_u64;
    let mut run = |blocks: usize, first_sample: &mut u64| -> u128 {
        let start = Instant::now();
        for _ in 0..blocks {
            let block = EffectBankProcessBlock::new(
                &mut left,
                &mut right,
                None,
                FRAMES as u32,
                BankWidth::Eight,
                *first_sample,
                &[],
                &offsets,
                FRAMES as u32,
            )
            .expect("bank block");
            let _report = bank.process_bank(block);
            *first_sample += FRAMES as u64;
        }
        start.elapsed().as_nanos()
    };

    let _ = run(WARMUP_BLOCKS, &mut first_sample);
    for round in 0..2 {
        let elapsed = run(ROUND_BLOCKS, &mut first_sample);
        let samples = (ROUND_BLOCKS * FRAMES * LANES * 2) as f64;
        println!(
            "issue_091_soft_clip_bank round={round} blocks={ROUND_BLOCKS} \
             ns_per_track_channel_sample={:.3}",
            elapsed as f64 / samples
        );
    }
}

fn bench_backend() -> miso_engine_core::KernelBackendV1 {
    miso_engine_core::KernelBackendV1::X86Avx2
}
