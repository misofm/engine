//! Descriptive before/after timing. Not a gate: it is run once, by hand, and reported.
//!
//! `cargo test -p miso-engine-builtins --release --test speed -- --ignored --nocapture`
//!
//! The audit measured the pre-#85 shape at **14.3 ns** per track-frame for the eight-lane bank and
//! **12.65 ns** for the scalar chain, with the matrix ramp at **3.35 ns**. Those numbers came from
//! a per-sample `#[inline(never)]` kernel called four times per frame with slice and mask
//! re-validation around each call; nothing here is tuned to beat them, and nothing is hashed or
//! checked inside a timed interval (master plan §1.5).

use std::time::Instant;

use miso_engine_builtins::*;
use miso_engine_core::KernelBackendV1;
use miso_engine_effect_contract::BankWidth;

const FRAMES: usize = 128;
const BLOCKS: usize = 20_000;

fn parameters(index: usize) -> BuiltinParameters {
    BuiltinParameters {
        left: ChannelParameters {
            hpf_hz: 100.0,
            lpf_hz: 1_000.0,
            ..ChannelParameters::default()
        },
        right: ChannelParameters {
            hpf_hz: 100.0,
            lpf_hz: 1_000.0,
            ..ChannelParameters::default()
        },
        smoothing_samples: 64,
        ..BuiltinParameters::default()
    }
    .tap(index)
}

/// Keeps every prepared chain distinct without changing the work per frame.
trait Tap {
    fn tap(self, index: usize) -> Self;
}

impl Tap for BuiltinParameters {
    fn tap(mut self, index: usize) -> Self {
        self.left.trim_db = index as f32 * 0.125;
        self
    }
}

fn signal(len: usize) -> Vec<f32> {
    let mut state = 0x5EED_0085_u64;
    (0..len)
        .map(|_| {
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            ((state.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32 as i32 as f32)
                / 2_147_483_648.0
        })
        .collect()
}

#[test]
#[ignore = "descriptive timing, run by hand"]
fn bench_input_stage_ns_per_track_frame() {
    // W = 1: one track through the scalar `Lane`.
    let mut chain = BuiltinChain::new(48_000, parameters(0)).expect("prepare");
    let mut left = signal(FRAMES);
    let mut right = signal(FRAMES);
    for _ in 0..256 {
        chain.process_input(DualMonoBlock::new(&mut left, &mut right, 0).expect("warm-up block"));
    }
    // The source is copied back in every block. Without it, feeding a filter its own output
    // twenty thousand times drives every sample into the subnormal range, and the microseconds
    // x86 spends there would be the whole measurement.
    let source_left = signal(FRAMES);
    let source_right = signal(FRAMES);
    let start = Instant::now();
    for _ in 0..BLOCKS {
        left.copy_from_slice(&source_left);
        right.copy_from_slice(&source_right);
        chain.process_input(DualMonoBlock::new(&mut left, &mut right, 0).expect("block"));
    }
    let scalar = start.elapsed().as_secs_f64() * 1e9 / (BLOCKS * FRAMES) as f64;

    // W = 8: eight tracks through one bank.
    let inputs: Vec<InputBuiltins> = (0..8)
        .map(|index| {
            BuiltinChain::new(48_000, parameters(index))
                .expect("prepare")
                .into_input_builtins()
        })
        .collect();
    let mut bank = BuiltinInputBankV1::new(KernelBackendV1::X86Avx2Fma, BankWidth::Eight, inputs)
        .expect("bank");
    let mut bank_left = signal(FRAMES * 8);
    let mut bank_right = signal(FRAMES * 8);
    for _ in 0..256 {
        bank.process(&mut bank_left, &mut bank_right, FRAMES as u32);
    }
    let bank_source_left = signal(FRAMES * 8);
    let bank_source_right = signal(FRAMES * 8);
    let start = Instant::now();
    for _ in 0..BLOCKS {
        bank_left.copy_from_slice(&bank_source_left);
        bank_right.copy_from_slice(&bank_source_right);
        bank.process(&mut bank_left, &mut bank_right, FRAMES as u32);
    }
    let bank_ns = start.elapsed().as_secs_f64() * 1e9 / (BLOCKS * FRAMES * 8) as f64;

    // An identity chain: no filters at all. Its two disabled sections still run their arithmetic,
    // because master plan §4.2 forbids an `enabled` flag at render — a bank slot can hold a
    // filtered and an unfiltered track in the same section, so the branch could not be per lane.
    let mut identity = BuiltinChain::new(48_000, BuiltinParameters::default()).expect("prepare");
    let mut identity_left = signal(FRAMES);
    let mut identity_right = signal(FRAMES);
    for _ in 0..256 {
        identity.process_input(
            DualMonoBlock::new(&mut identity_left, &mut identity_right, 0).expect("warm-up"),
        );
    }
    let start = Instant::now();
    for _ in 0..BLOCKS {
        identity_left.copy_from_slice(&source_left);
        identity_right.copy_from_slice(&source_right);
        identity.process_input(
            DualMonoBlock::new(&mut identity_left, &mut identity_right, 0).expect("block"),
        );
    }
    let identity_ns = start.elapsed().as_secs_f64() * 1e9 / (BLOCKS * FRAMES) as f64;

    // The matrix, ramping continuously.
    let mut matrix = BuiltinChain::new(48_000, parameters(0)).expect("prepare");
    let mut matrix_left = signal(FRAMES);
    let mut matrix_right = signal(FRAMES);
    let matrix_source = signal(FRAMES);
    let start = Instant::now();
    for block in 0..BLOCKS {
        matrix_left.copy_from_slice(&matrix_source);
        matrix_right.copy_from_slice(&matrix_source);
        matrix
            .set_matrix_target(Matrix2x2 {
                ll: if block % 2 == 0 { 0.5 } else { 1.0 },
                lr: 0.25,
                rl: -0.25,
                rr: 0.5,
            })
            .expect("target");
        matrix.process_matrix(
            DualMonoBlock::new(&mut matrix_left, &mut matrix_right, 0).expect("block"),
        );
    }
    let matrix_ns = start.elapsed().as_secs_f64() * 1e9 / (BLOCKS * FRAMES) as f64;

    println!("input stage W=1: {scalar:.2} ns per track-frame (audit measured 12.65 before #85)");
    println!("input stage W=8: {bank_ns:.2} ns per track-frame (audit measured 14.30 before #85)");
    println!("matrix ramping:  {matrix_ns:.2} ns per track-frame (audit measured 3.35 before #85)");
    println!(
        "identity chain W=1: {identity_ns:.2} ns per track-frame (two disabled sections, §4.2)"
    );
}
