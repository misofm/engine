//! Guest half of gate G5: the cross-target digest harness.
//!
//! Built two ways from one source. As an `rlib` it is linked by `miso-engine-wasm-gates` and run
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

use miso_engine_wasm_gate_corpus as corpus;
use std::sync::Mutex;

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

/// Number of leading cases that are lane cases; the rest replay the `miso-engine-math` M3 corpus.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_lane_case_count() -> u32 {
    corpus::LANE_CASE_COUNT as u32
}

/// Number of lane widths every case is digested at.
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_widths() -> u32 {
    corpus::WIDTHS as u32
}

/// The production backend this module was compiled for: `0` scalar, `1` `Simd4`, `2` `Simd8`.
///
/// The host asserts this matches what it asked for, so a wasm artifact built without `simd128`
/// cannot silently pass as the `simd128` one (or the reverse).
#[unsafe(no_mangle)]
pub extern "C" fn miso_gate_backend() -> u32 {
    match miso_engine_lane::Backend::current() {
        miso_engine_lane::Backend::Scalar => 0,
        miso_engine_lane::Backend::Simd4 => 1,
        miso_engine_lane::Backend::Simd8 => 2,
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
