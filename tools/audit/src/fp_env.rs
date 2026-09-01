//! Issue #146 E5: what the canonical floating-point environment costs per rendered block.
//!
//! Every native render entry pins the environment for the length of one block: read the caller's
//! control word, write the canonical one, render, write the caller's word back. The claim made when
//! that was decided was "negligible"; this subject is the number behind it.
//!
//! The workload is the production shape -- a real `PreparedRenderPlan` at 48 kHz and a 128-frame
//! quantum, rendered `blocks` times -- and the measurement is a difference of two loops over that
//! same plan, one wrapping each block in [`CanonicalFpEnv`] and one not. The difference is the
//! guard and nothing else: same plan, same output storage, same clock, same order.
//!
//! Descriptive only (AGENTS.md): the workload and the record are frozen before timing, one
//! invocation with one warmup and one or two measured rounds, no threshold, no retry.

use crate::record::{json_f64_array, json_integer_array, metadata};
use bench_support::alloc as bench_alloc;
use bench_support::timing;
use core::num::NonZeroUsize;
use engine::realtime::{
    PlanarBufferMut, PrepareRenderPlan, PreparedRenderPlan, RenderEnvelope, RenderIo, RenderTime,
};
use engine::{QuantumFrames, SampleRateHz};
use lane::fpenv::CanonicalFpEnv;

/// The launch quantum and the block size the guard is amortised over.
const QUANTUM: usize = 128;

/// Blocks rendered once, untimed, before either measured loop of a round.
const WARMUP_BLOCKS: u64 = 4_096;

fn prepared_plan() -> PreparedRenderPlan {
    PreparedRenderPlan::prepare(PrepareRenderPlan {
        plan_id: 1,
        envelope: RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(QUANTUM as u32),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("two output channels"),
        },
        scratch: &[],
    })
    .expect("valid prepared benchmark plan")
}

/// Renders `blocks` quanta, each one wrapped in the render entry's guard.
fn render_guarded(plan: &mut PreparedRenderPlan, output: &mut [f32], blocks: u64) {
    for block in 0..blocks {
        let _fp_env = CanonicalFpEnv::enter();
        let io = RenderIo {
            input: None,
            output: PlanarBufferMut::try_new(output, 2, QUANTUM, QUANTUM).expect("output view"),
        };
        plan.render(
            io,
            RenderTime {
                absolute_sample: block * QUANTUM as u64,
            },
        )
        .expect("guarded render");
    }
}

/// Renders `blocks` quanta with no guard: the same loop, one construction shorter.
fn render_bare(plan: &mut PreparedRenderPlan, output: &mut [f32], blocks: u64) {
    for block in 0..blocks {
        let io = RenderIo {
            input: None,
            output: PlanarBufferMut::try_new(output, 2, QUANTUM, QUANTUM).expect("output view"),
        };
        plan.render(
            io,
            RenderTime {
                absolute_sample: block * QUANTUM as u64,
            },
        )
        .expect("bare render");
    }
}

/// One round: warm both arms, then time each once over the same plan and storage.
fn run_round(blocks: u64) -> (u64, u64) {
    let mut plan = prepared_plan();
    let mut output = vec![0.0_f32; QUANTUM * 2];
    timing::untimed(|| {
        render_bare(&mut plan, &mut output, WARMUP_BLOCKS);
        render_guarded(&mut plan, &mut output, WARMUP_BLOCKS);
    });
    let (bare_ns, ()) = timing::timed(|| render_bare(&mut plan, &mut output, blocks));
    let (guarded_ns, ()) = timing::timed(|| render_guarded(&mut plan, &mut output, blocks));
    (bare_ns, guarded_ns)
}

pub(crate) fn main() {
    bench_alloc::assert_installed();
    let (rounds, blocks) = parse_arguments();

    let mut bare = Vec::with_capacity(usize::from(rounds));
    let mut guarded = Vec::with_capacity(usize::from(rounds));
    for _ in 0..rounds {
        let (bare_ns, guarded_ns) = run_round(blocks);
        bare.push(bare_ns);
        guarded.push(guarded_ns);
    }
    let per_block = |values: &[u64]| {
        values
            .iter()
            .map(|value| *value as f64 / blocks as f64)
            .collect::<Vec<_>>()
    };
    let bare_per_block = per_block(&bare);
    let guarded_per_block = per_block(&guarded);
    let guard_per_block = guarded_per_block
        .iter()
        .zip(&bare_per_block)
        .map(|(guarded, bare)| guarded - bare)
        .collect::<Vec<_>>();

    println!(
        concat!(
            "{{\"schema_version\":1,\"benchmark\":\"fp_environment_guard\",",
            "\"cpu\":\"{}\",\"os\":\"{}\",\"power_mode\":\"{}\",",
            "\"compiler\":\"{}\",\"llvm_version\":\"{}\",",
            "\"target_triple\":\"{}\",\"compile_target_features\":\"{}\",",
            "\"runtime_or_browser\":\"{}\",\"sample_rate_hz\":48000,",
            "\"quantum_frames\":{},\"fixture\":\"canonical_fp_env_guard_over_prepared_render\",",
            "\"warmup_blocks\":{},\"blocks_per_round\":{},\"rounds\":{},",
            "\"bare_round_duration_ns\":{},\"guarded_round_duration_ns\":{},",
            "\"bare_ns_per_block\":{},\"guarded_ns_per_block\":{},",
            "\"guard_ns_per_block\":{},",
            "\"statistical_method\":\"per-round ns/block, guarded minus bare over one plan; ",
            "descriptive only; no threshold\"}}"
        ),
        metadata("MISO_ENGINE_BENCH_CPU_MODEL"),
        std::env::consts::OS,
        metadata("MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE"),
        metadata("MISO_ENGINE_BENCH_RUST_VERSION"),
        metadata("MISO_ENGINE_BENCH_LLVM_VERSION"),
        metadata("MISO_ENGINE_BENCH_TARGET_TRIPLE"),
        metadata("MISO_ENGINE_BENCH_TARGET_FEATURES"),
        metadata("MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER"),
        QUANTUM,
        WARMUP_BLOCKS,
        blocks,
        rounds,
        json_integer_array(bare.iter().copied().map(u128::from)),
        json_integer_array(guarded.iter().copied().map(u128::from)),
        json_f64_array(bare_per_block.iter().copied()),
        json_f64_array(guarded_per_block.iter().copied()),
        json_f64_array(guard_per_block.iter().copied()),
    );
}

fn parse_arguments() -> (u8, u64) {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let mut blocks = 1_000_000_u64;
    let mut rounds = None;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--blocks" => {
                index += 1;
                blocks = arguments
                    .get(index)
                    .and_then(|value| value.parse().ok())
                    .filter(|blocks| *blocks > 0)
                    .expect("--blocks requires a positive integer");
            }
            "--benchmark-rounds" => {
                index += 1;
                rounds = arguments
                    .get(index)
                    .and_then(|value| value.parse::<u8>().ok())
                    .filter(|rounds| matches!(rounds, 1 | 2));
                assert!(rounds.is_some(), "--benchmark-rounds requires 1 or 2");
            }
            other => panic!("unknown argument: {other}"),
        }
        index += 1;
    }
    (rounds.expect("--benchmark-rounds is required"), blocks)
}
