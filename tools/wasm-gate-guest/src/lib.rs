//! Guest half of gate G5: the cross-target digest harness.
//!
//! Built two ways from one source. As an `rlib` it is linked by `wasm-gates` and run
//! natively; as a `cdylib` for `wasm32-unknown-unknown` it is instantiated by wasmtime and driven
//! through the `extern "C"` exports below. Because both legs execute the same
//! [`corpus`] module, a digest difference between them is a difference in the *target*, which is
//! exactly what master plan #83 D5 claims cannot happen.
//!
//! The exports are deliberately primitive: `u32` in, `u32` out, no pointers, no memory contract to
//! get wrong, and no imports at all, so the module instantiates with an empty import object and
//! cannot reach anything outside itself.
//!
//! This crate is dev/tooling. Nothing in the engine depends on it, and it is the only place in the
//! workspace that may pull a WebAssembly runtime (its host counterpart does).

#![allow(unsafe_code)]

use std::sync::Mutex;
use wasm_gate_corpus as corpus;

/// The last digest computed, so that reading eight words costs one run instead of eight.
///
/// A `Mutex` rather than a `static mut`: the workspace denies `unsafe_code` outside the audited
/// boundaries, and this is not a render path, so a lock is free of consequence here.
static LAST: Mutex<Option<Cached>> = Mutex::new(None);

/// One memoised digest.
struct Cached {
    /// Corpus case index.
    case: u32,
    /// Width index, as [`corpus::digest_case`] numbers them.
    width: u32,
    /// The digest of that case at that width.
    digest: [u8; 32],
}

/// Digest of one case at one width, memoised across the eight word reads.
fn digest(case: u32, width: u32) -> [u8; 32] {
    let mut cache = LAST
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(last) = cache.as_ref()
        && last.case == case
        && last.width == width
    {
        return last.digest;
    }
    let digest = corpus::digest_case(case as usize, width as usize);
    *cache = Some(Cached {
        case,
        width,
        digest,
    });
    digest
}

/// Number of corpus cases this module can digest.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_case_count() -> u32 {
    corpus::CASE_COUNT as u32
}

/// Number of leading cases that are lane cases; the rest replay the `math` M3 corpus.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_lane_case_count() -> u32 {
    corpus::LANE_CASE_COUNT as u32
}

/// Number of lane widths every case is digested at.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_widths() -> u32 {
    corpus::WIDTHS as u32
}

/// The production backend this module was compiled for: `1` `Simd4`, the one wasm width (`2` is
/// `Simd8` and `0` the scalar backend, which no wasm build selects since #1038 and #1062).
///
/// The host asserts this matches what it asked for, so a guest built for another width cannot
/// silently pass as the `simd128` one.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_backend() -> u32 {
    // By width, not by variant: `Backend::Scalar` exists only in `lane/test-support` builds
    // (#1059), so a match on the variants compiles in only one of them.
    match lane::Backend::current().width() {
        4 => 1,
        8 => 2,
        _ => 0,
    }
}

/// Lanes on which this module's `Lane::max`/`Lane::min` disagree with the scalar oracle at
/// `width`, over the pool that separates their per-backend lowerings. Zero is the only admissible
/// answer, and the host fails the leg on anything else.
///
/// This is the only wasm execution of that truth table anywhere in the workspace: the lane crate's
/// gate G1 runs it natively, and wasm's `f32x4.pmax`/`f32x4.pmin` lowering has no native leg.
///
/// Traps on an out-of-range argument, which the host reports as a failure rather than a mismatch.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_minmax_lowering_mismatches(width: u32) -> u32 {
    corpus::minmax_lowering_mismatches(width as usize)
}

/// Lanes on which this module's `f64` lanes (`lane::LaneF64`, `lane::Widen`) disagree with the
/// scalar `f64` oracle at `width` (issue #949). Zero is the only admissible answer, and the host
/// fails the leg on anything else.
///
/// Like [`miso_gate_minmax_lowering_mismatches`], this is the only wasm execution of that
/// differential: `crates/lane/tests/f64_lane.rs` runs gates 1 to 3 natively, and neither wasm
/// lowering (`f64x2` under `simd128`, a per-lane array without it) has a native leg.
///
/// Traps on an out-of-range argument, which the host reports as a failure rather than a mismatch.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_f64_lane_mismatches(width: u32) -> u32 {
    corpus::f64_lane_mismatches(width as usize)
}

/// The lowering probe of issue #949, gate 5: the energy shape `e = e.add(w.mul(w))` at `Simd4`.
///
/// `scripts/run-wasm-gates.sh` (`check_f64_lane_lowering`) disassembles this function's body in
/// the `simd128` guest and fails unless it contains `f64x2.promote_low_f32x4`, `f64x2.mul` and
/// `f64x2.add` and none of the scalar `f64.promote_f32`, `f64.mul` or `f64.add`. `#[inline(never)]`
/// and the export keep it one named function to census. The inputs are an xorshift of `seed` with
/// bit 30, the top exponent bit, cleared, so every value is finite and below 2 in magnitude; the
/// result is a fold of the accumulator's bits, so nothing the probe computes is dead.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn miso_gate_f64_lane_probe(seed: u32) -> u32 {
    use lane::{Lane, LaneF64, Simd4, Widen};

    let mut frames = [0.0_f32; 4 * 64];
    let mut state = seed | 1;
    for value in &mut frames {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        *value = f32::from_bits(state & 0xBFFF_FFFF);
    }
    let mut energy = <Simd4 as Widen>::F64::load(&[0.5; 4]);
    for frame in frames.chunks_exact(4) {
        let w = Simd4::load(frame).widen();
        energy = energy.add(w.mul(w));
    }
    let mut out = [0.0_f64; 4];
    energy.store(&mut out);
    out.iter().fold(0_u32, |fold, value| {
        let bits = value.to_bits();
        fold ^ (bits as u32) ^ ((bits >> 32) as u32)
    })
}

/// Lane fields on which this module's full meter block pass (`lane::kernels::builtins::meter_block`)
/// disagrees with the builtin meter's scalar loop at `width` (issue #950). Zero is the only
/// admissible answer, and the host fails the leg on anything else.
///
/// Like [`miso_gate_f64_lane_mismatches`], this is the only wasm execution of that differential:
/// `crates/lane/tests/meter_block.rs` runs it natively.
///
/// Traps on an out-of-range argument, which the host reports as a failure rather than a mismatch.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_meter_block_mismatches(width: u32) -> u32 {
    corpus::meter_block_mismatches(width as usize)
}

/// The lowering probe of issue #950: the real `lane::kernels::builtins::meter_block` at `Simd4`,
/// the kernel the graph's banked full meter pass runs on the browser's four-lane banks.
///
/// `scripts/run-wasm-gates.sh` (`check_f64_lane_lowering`) disassembles this function's body in
/// the `simd128` guest and fails unless it contains `f64x2.promote_low_f32x4`, `f64x2.mul` and
/// `f64x2.add` and none of the scalar `f64.promote_f32`, `f64.mul`, `f64.add`, `f32.add`, `f32.gt`
/// or `f32.abs`. `#[inline(never)]` and the export keep it one named function to census. The
/// inputs are an xorshift of `seed` taken as raw bits, so NaNs, infinities and subnormals reach
/// the kernel's sanitization too; the result is a fold of every output's bits, so nothing the probe
/// computes is dead.
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn miso_gate_meter_block_probe(seed: u32) -> u32 {
    use lane::kernels::builtins::meter_block;
    use lane::{Lane, LaneF64, Simd4, Widen};

    let mut words = [0.0_f32; 4 * 64];
    let mut state = seed | 1;
    for value in &mut words {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        *value = f32::from_bits(state);
    }
    let block = meter_block::<Simd4>(&words, 64, <Simd4 as Widen>::F64::load(&[0.5; 4]));
    let mut counts = [0.0_f32; 12];
    block.peak.store(&mut counts[..4]);
    block.clipped.store(&mut counts[4..8]);
    block.sanitized.store(&mut counts[8..]);
    let mut energy = [0.0_f64; 4];
    block.energy.store(&mut energy);
    let fold = counts
        .iter()
        .fold(0_u32, |fold, value| fold.rotate_left(5) ^ value.to_bits());
    energy.iter().fold(fold, |fold, value| {
        let bits = value.to_bits();
        fold ^ (bits as u32) ^ ((bits >> 32) as u32)
    })
}

/// Word `word` (0..8) of the little-endian SHA-256 digest of `case` at `width`.
///
/// Traps on an out-of-range argument, which the host reports as a failure rather than a mismatch.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_digest_word(case: u32, width: u32, word: u32) -> u32 {
    let digest = digest(case, width);
    let offset = (word as usize) * 4;
    assert!(offset + 4 <= digest.len(), "digest word out of range");
    u32::from_le_bytes([
        digest[offset],
        digest[offset + 1],
        digest[offset + 2],
        digest[offset + 3],
    ])
}
