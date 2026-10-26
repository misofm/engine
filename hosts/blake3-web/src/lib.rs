//! The browser's BLAKE3-256 for canonical-PCM stem identities (issue #2111).
//!
//! `stem-store/incremental-blake3.js` hashes a stem through this module when it loads, and
//! through its own JavaScript BLAKE3 when it does not. The hash is the `blake3` crate's, built for
//! `wasm32-unknown-unknown` with `+simd128` and the crate's `wasm32_simd` feature, so it compresses
//! four chunks per `v128` instruction. The crate's `pure` feature keeps every native build of
//! this workspace on portable Rust with no C compiler; it does not turn the Wasm SIMD path off.
//!
//! # The boundary
//!
//! One hasher is one heap allocation that owns its BLAKE3 state and a staging buffer of
//! [`INPUT_CAPACITY`] bytes. JavaScript copies at most that many bytes of a stream chunk to
//! [`miso_engine_blake3_v1_input`] with one `set`, then calls [`miso_engine_blake3_v1_update`]
//! with the byte count. [`miso_engine_blake3_v1_finalize`] writes the 32-byte digest to the start
//! of the same buffer. No export takes a pointer other than a hasher handle, so a wrong length
//! can never reach memory outside the staging buffer: it is refused.
//!
//! A panic or an allocation failure traps the instance (`panic = "abort"`); the JavaScript caller
//! sees a `WebAssembly.RuntimeError`. Nothing here runs on an audio thread.

#![allow(unsafe_code)]

/// The staging buffer's size in bytes: 256 KiB, a whole number of 1,024-byte BLAKE3 chunks.
pub const INPUT_CAPACITY: usize = 256 * 1024;

/// The digest's size in bytes: BLAKE3's 32-byte default output.
const DIGEST_BYTES: usize = 32;

/// One incremental BLAKE3-256 hash and its staging buffer.
pub struct Hasher {
    /// The hash state.
    hash: blake3::Hasher,
    /// The bytes the next [`miso_engine_blake3_v1_update`] hashes, and the digest after
    /// [`miso_engine_blake3_v1_finalize`].
    input: Box<[u8]>,
}

/// Create a hasher. The caller owns the handle and releases it with
/// [`miso_engine_blake3_v1_free`].
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_blake3_v1_new() -> *mut Hasher {
    Box::into_raw(Box::new(Hasher {
        hash: blake3::Hasher::new(),
        input: vec![0; INPUT_CAPACITY].into_boxed_slice(),
    }))
}

/// The staging buffer's capacity in bytes, [`INPUT_CAPACITY`].
#[unsafe(no_mangle)]
pub extern "C" fn miso_engine_blake3_v1_input_capacity() -> u32 {
    // 256 KiB fits a `u32`.
    INPUT_CAPACITY as u32
}

/// The address of the hasher's staging buffer, stable for the hasher's life; null for a null
/// handle.
///
/// # Safety
///
/// `hasher` is null or a live handle from [`miso_engine_blake3_v1_new`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_blake3_v1_input(hasher: *mut Hasher) -> *mut u8 {
    // SAFETY: the caller passes null or a live handle from `new`, which no other reference
    // borrows during this call.
    match unsafe { hasher.as_mut() } {
        Some(hasher) => hasher.input.as_mut_ptr(),
        None => std::ptr::null_mut(),
    }
}

/// Hash the first `length` bytes of the staging buffer. Returns 1, or 0 when the handle is null or
/// `length` exceeds [`INPUT_CAPACITY`]; a refused call changes nothing.
///
/// # Safety
///
/// `hasher` is null or a live handle from [`miso_engine_blake3_v1_new`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_blake3_v1_update(hasher: *mut Hasher, length: u32) -> u32 {
    // SAFETY: as in `miso_engine_blake3_v1_input`.
    let Some(hasher) = (unsafe { hasher.as_mut() }) else {
        return 0;
    };
    let Some(bytes) = usize::try_from(length)
        .ok()
        .and_then(|length| hasher.input.get(..length))
    else {
        return 0;
    };
    hasher.hash.update(bytes);
    1
}

/// Write the 32-byte digest to the start of the staging buffer. Returns 1, or 0 for a null handle.
/// The hash state is unchanged, so the caller enforces one digest per hash.
///
/// # Safety
///
/// `hasher` is null or a live handle from [`miso_engine_blake3_v1_new`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_blake3_v1_finalize(hasher: *mut Hasher) -> u32 {
    // SAFETY: as in `miso_engine_blake3_v1_input`.
    let Some(hasher) = (unsafe { hasher.as_mut() }) else {
        return 0;
    };
    let digest = hasher.hash.finalize();
    hasher.input[..DIGEST_BYTES].copy_from_slice(digest.as_bytes());
    1
}

/// Release a hasher. A null handle is ignored.
///
/// # Safety
///
/// `hasher` is null or a live handle from [`miso_engine_blake3_v1_new`], and is not used again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn miso_engine_blake3_v1_free(hasher: *mut Hasher) {
    if !hasher.is_null() {
        // SAFETY: the caller passes a live handle from `new`, whose `Box::into_raw` this reverses,
        // and never uses it again.
        drop(unsafe { Box::from_raw(hasher) });
    }
}
