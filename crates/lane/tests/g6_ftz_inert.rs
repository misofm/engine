//! Gate G6: hardware flush-to-zero is inert.
//!
//! Master plan #83 D7 and the revision-4 amendment. The engine's denormal mechanism is the D7
//! `flush`, whose threshold (`1e-20`, about `2^-66`) sits far above the top of the subnormal range
//! (`2^-126`). The flush band therefore strictly contains the band hardware FTZ/DAZ acts on, so a
//! host that forces FTZ — Chrome does inside an `AudioWorklet` — must render the same bits as one
//! that does not. This gate asserts that on the flushed quantities, and asserts the opposite on an
//! *unflushed* arithmetic arm so that the gate cannot pass vacuously by failing to enable FTZ.
//!
//! The control word is reached through `lane::fpenv` (MXCSR on x86, FPCR on AArch64), whose
//! `unsafe` is allowlisted inside `lane`; the workspace forbids it everywhere else.
//!
//! Red-mutation proven for this gate (see `tests/MUTATIONS.md`): raise `FLUSH_EPS` below the
//! subnormal boundary (`1e-40`), which lets subnormal state words survive the flush and makes the
//! FTZ-on and FTZ-off digests differ.

mod support;

use std::hint::black_box;

use lane::{Lane, Simd8, flush};
use support::{Kernel, MAX_WIDTH, Signal, interleave, run_kernel};

/// Frames of the kernel arms.
const FRAMES: usize = 2_048;

/// Bits of the flush law over the subnormal range and its neighbourhood.
#[inline(never)]
fn flush_digest() -> Vec<u32> {
    let mut bits = Vec::new();
    let mut lanes = [0.0f32; MAX_WIDTH];
    let mut pattern = 1u32;
    while pattern < 0x0080_0000 {
        for (index, lane) in lanes.iter_mut().enumerate() {
            *lane = f32::from_bits(pattern.wrapping_add(index as u32 * 4_099));
        }
        let mut out = [0u32; MAX_WIDTH];
        flush(Simd8::load(black_box(&lanes))).store_bits(&mut out);
        bits.extend_from_slice(&out);
        pattern = pattern.wrapping_add(4_093);
    }
    bits
}

/// Bits of a kernel arm whose recurrences are flushed.
#[inline(never)]
fn kernel_digest(kernel: Kernel, signal: Signal) -> Vec<u32> {
    let lanes: Vec<Vec<f32>> = (0..MAX_WIDTH)
        .map(|lane| {
            let mut samples = vec![0.0f32; FRAMES];
            signal.fill(&mut samples, 0x6F72_0000 + lane as u64);
            samples
        })
        .collect();
    let mut block = interleave(&lanes, MAX_WIDTH, FRAMES);
    run_kernel::<Simd8>(kernel, black_box(&mut block), FRAMES, 1.0e-40, FRAMES);
    block.iter().map(|value| value.to_bits()).collect()
}

/// Bits of an arm that is deliberately *not* flushed: products that land in the subnormal range.
///
/// This is the control. Hardware FTZ changes these bits, which is what proves the FTZ bit was
/// really set when the other arms did not change.
#[inline(never)]
fn unflushed_digest() -> Vec<u32> {
    let mut bits = Vec::new();
    for step in 0..4_096u32 {
        let value = f32::from_bits(0x0C00_0000u32.wrapping_add(step * 977));
        let scale = f32::from_bits(0x0C80_0000u32.wrapping_sub(step * 13));
        let product = black_box(value) * black_box(scale);
        bits.push(product.to_bits());
        let sum = black_box(product) + black_box(f32::from_bits(0x0000_0100));
        bits.push(sum.to_bits());
    }
    bits
}

/// Collects every arm once, under whatever MXCSR the caller has set.
fn all_arms() -> (Vec<u32>, Vec<u32>, Vec<u32>, Vec<u32>) {
    (
        flush_digest(),
        kernel_digest(Kernel::SvfLow, Signal::Noise),
        kernel_digest(Kernel::OnePole, Signal::Subnormal),
        unflushed_digest(),
    )
}

/// The control-word bits that make hardware flush: MXCSR's FTZ and DAZ on x86, FPCR's `FZ` on
/// AArch64 (which flushes subnormal inputs and results alike; issue #1017).
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const FLUSH_BITS: lane::fpenv::FpControlWord = lane::softfma::MXCSR_FTZ | lane::softfma::MXCSR_DAZ;

/// See the x86 definition.
#[cfg(target_arch = "aarch64")]
const FLUSH_BITS: lane::fpenv::FpControlWord = lane::fpenv::FPCR_FZ;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[test]
fn g6_flush_makes_hardware_ftz_inert() {
    use lane::fpenv::{read_fp_control_word, write_fp_control_word};

    let saved = read_fp_control_word();
    assert_eq!(
        saved & FLUSH_BITS,
        0,
        "G6: the test host must start with hardware flush-to-zero clear"
    );

    let without = all_arms();
    write_fp_control_word(saved | FLUSH_BITS);
    let with = all_arms();
    write_fp_control_word(saved);
    assert_eq!(
        read_fp_control_word(),
        saved,
        "G6: the control word must be restored"
    );

    assert_eq!(with.0, without.0, "G6: the flush law must be FTZ-inert");
    assert_eq!(
        with.1, without.1,
        "G6: svf_block must be FTZ-inert on a normal signal"
    );
    assert_eq!(
        with.2, without.2,
        "G6: one_pole_block must be FTZ-inert with a subnormal-seeded state"
    );
    assert_ne!(
        with.3, without.3,
        "G6 is vacuous: FTZ did not change the unflushed control arm"
    );
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[test]
fn g6_flush_makes_hardware_ftz_inert() {
    // wasm has no flush-to-zero mode to set (the core specification forbids one); the wasm leg of
    // G6 runs the same corpus under `wasmtime` (issue #83, job 83d).
    let arms = all_arms();
    assert!(!arms.0.is_empty(), "G6: the corpus must not be empty");
}
