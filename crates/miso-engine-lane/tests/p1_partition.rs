//! Gate P1: partition invariance.
//!
//! Master plan #83 §10. Rendering a stream in blocks of 1, 7, 64, 128 or 512 frames must produce
//! the same bits as rendering it in one call: a block boundary is a bookkeeping event, never a
//! numeric one. State and coefficient ramps are carried across the blocks exactly as a prepared
//! plan carries them.
//!
//! Red-mutation proven for this gate (see `tests/MUTATIONS.md`): drop the state write-back at the
//! end of `svf_block`, which fails at partition 1 against the one-shot run.

mod support;

use miso_engine_lane::{Lane, Simd4, Simd8};
use support::{ALL_KERNELS, ALL_SIGNALS, Kernel, MAX_WIDTH, Signal, interleave, run_kernel};

/// Frames per case: enough to cross every partition boundary and leave both ramp windows.
const FRAMES: usize = 4_096;

/// The partitions of master plan §10.
const PARTITIONS: &[usize] = &[1, 7, 64, 128, 512];

/// Runs one case at one width and returns the whole output block's bits.
fn run<L: Lane>(kernel: Kernel, signal: Signal, partition: usize) -> Vec<u32> {
    let lanes: Vec<Vec<f32>> = (0..L::WIDTH)
        .map(|lane| {
            let mut samples = vec![0.0f32; FRAMES];
            signal.fill(&mut samples, 0x9A17_0000 + lane as u64);
            samples
        })
        .collect();
    let mut block = interleave(&lanes, L::WIDTH, FRAMES);
    run_kernel::<L>(kernel, &mut block, FRAMES, signal.state_seed(), partition);
    block.iter().map(|value| value.to_bits()).collect()
}

/// Checks every partition of one kernel at one width.
fn check<L: Lane>(width_name: &str) {
    for kernel in ALL_KERNELS {
        for signal in ALL_SIGNALS {
            let one_shot = run::<L>(*kernel, *signal, FRAMES);
            for partition in PARTITIONS {
                let partitioned = run::<L>(*kernel, *signal, *partition);
                let first_difference = one_shot
                    .iter()
                    .zip(partitioned.iter())
                    .position(|(left, right)| left != right);
                assert_eq!(
                    first_difference,
                    None,
                    "P1 {kernel} / {signal} at {width_name}: partition {partition} differs from \
                     the one-shot run at sample {index:?}",
                    kernel = kernel.name(),
                    signal = signal.name(),
                    index = first_difference,
                );
            }
        }
    }
}

#[test]
fn p1_every_kernel_is_partition_invariant() {
    assert_eq!(MAX_WIDTH, 8, "the corpus is built for eight lanes");
    check::<f32>("f32");
    check::<Simd4>("Simd4");
    check::<Simd8>("Simd8");
}
