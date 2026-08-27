//! Guest half of the issue #163 phase 2 wasm console arm.
//!
//! # What this is
//!
//! `wasm32-unknown-unknown`'s view of the console benchmark subject. Compiled as a `cdylib` with
//! `+simd128` and instantiated by the pinned wasmtime in `miso-engine-wasm-console`, it prepares
//! one of the sixteen console workloads through the real session/builtins/effect/graph compilers and
//! renders it one block per exported call.
//!
//! Everything it renders comes from [`miso_engine_console_workload`], which the native leg of the
//! same measurement links directly. This crate adds no DSP, no fixture and no model edit of its
//! own: it is an ABI, a lifetime for one prepared arm, and nothing else. That is deliberate and it
//! is the property the whole comparison rests on -- if the guest could compute anything the native
//! leg does not, the ratio between them would be partly a measurement of this file.
//!
//! # Why the clock is not here
//!
//! `wasm32-unknown-unknown` cannot construct a `std::time::Instant`, which
//! `docs/rulings/wasm-kernel-timing-interim.md` recorded as the second blocker to a wasm console
//! arm. The resolution is that the guest never times anything: it exposes
//! [`miso_console_render`], which renders exactly one block and returns, and the host takes a
//! timestamp on either side of the call.
//!
//! The native subject's timed region is `timing::timed(|| runtime.render(observation))`, whose
//! `#[104 F1]` guard panics if the timed body hashed anything. The guest reproduces that guard
//! *structurally* rather than by assertion: rendering and hashing are two different exports, so
//! the host cannot include a hash update inside a timed region without calling a second function
//! it can see itself calling. [`miso_console_hash_output`] is never called between a host's two
//! timestamps.
//!
//! # Why there is no environment and no argument
//!
//! The third recorded blocker. The guest reads no environment and takes no arguments; the round
//! marker, the eleven host metadata names and the candidate commit are the *host's* to carry, and
//! it writes them into records the guest never sees. Every export below is `u32` in, `u32` out --
//! no pointers, no memory contract to get wrong -- and the module imports nothing at all, so it
//! instantiates against an empty import object and cannot reach anything outside itself.

// Exporting from a `cdylib` requires `#[unsafe(no_mangle)]` under edition 2024, and there is no
// safe spelling of it. This is the same ownership boundary `tools/miso-engine-wasm-gate-guest`
// already holds for exactly the same reason, and `scripts/check-bench-policy.sh` names this file
// alongside it. Nothing in this crate dereferences a pointer, and no engine crate links it.
#![allow(unsafe_code)]

use miso_engine_bench_support::digest::Sha256Sink;
use miso_engine_console_workload::{
    PlanConfig, SOURCE_BLOCK_VALUES, SessionRuntime, SourceSignal, WORKLOADS,
};
use miso_engine_lane::Backend;
use std::sync::Mutex;

/// The single prepared arm this module holds between calls.
///
/// A `Mutex` rather than a `static mut`, exactly as the gate G5 guest does it: the workspace
/// denies `unsafe_code` outside audited boundaries and this is not a render path, so a lock that
/// is never contended (wasm32 without threads) costs nothing that matters.
static ARM: Mutex<Option<Arm>> = Mutex::new(None);

/// Input samples staged by the host, to be consumed by the next [`miso_console_prepare`].
///
/// The host computes the tone natively and pushes it across, one `f32` bit pattern at a time,
/// because `f32::sin` is a libm call and libm is not the same implementation on this target as it
/// is on the host's. Injecting the host's samples is what makes a digest difference between the
/// two legs a statement about the *engine* rather than about two C libraries. Staging happens
/// entirely before preparation and never inside a timed region.
static STAGED_SOURCE: Mutex<Vec<f32>> = Mutex::new(Vec::new());

/// One prepared workload, its running output digest, and what has happened to it so far.
struct Arm {
    /// The prepared plan being rendered.
    runtime: SessionRuntime,
    /// Digest of every block rendered since preparation, updated outside any timed region.
    hash: Sha256Sink,
    /// Blocks the plan refused. Reported to the host, which puts it in the record.
    render_errors: u32,
    /// The finished digest, once [`miso_console_finish_digest`] has been called.
    digest: Option<[u8; 32]>,
}

/// Number of console workloads this module can prepare, in the shared crate's emission order.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_workload_count() -> u32 {
    WORKLOADS.len() as u32
}

/// The backend this module was compiled for: `0` scalar, `1` `Simd4`, `2` `Simd8`.
///
/// The host asserts this matches what it asked for, so a wasm artifact built without `simd128`
/// cannot silently pass as the `simd128` one and be reported as a SIMD number.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_backend() -> u32 {
    match Backend::current() {
        Backend::Scalar => 0,
        Backend::Simd4 => 1,
        Backend::Simd8 => 2,
    }
}

/// Discards any staged input samples. Returns `1`.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_reset_source() -> u32 {
    STAGED_SOURCE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clear();
    1
}

/// Appends one input sample, as an `f32` bit pattern. Returns `1`.
///
/// One `u32` per sample keeps the ABI free of pointers and of any contract over this module's
/// linear memory. A full 128-track table is 32,768 crossings, taken once per workload, entirely
/// outside every timed region.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_push_source_word(bits: u32) -> u32 {
    STAGED_SOURCE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .push(f32::from_bits(bits));
    1
}

/// Input samples staged since the last reset.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_staged_source_len() -> u32 {
    STAGED_SOURCE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .len() as u32
}

/// Prepares workload `index` from the staged input table, replacing whatever arm was held before.
///
/// Returns `1` on success, and `0` if `index` names no workload or if the staged table does not
/// carry exactly one block per track of that workload. A short table is refused rather than
/// completed locally: completing it would silently reintroduce the libm difference this staging
/// exists to remove.
///
/// Preparation runs the four real compilers inside the guest. It is not timed and is not part of
/// any reported number.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_prepare(index: u32) -> u32 {
    let Some(workload) = WORKLOADS.get(index as usize).copied() else {
        return 0;
    };
    let staged = STAGED_SOURCE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone();
    if staged.len() != workload.tracks() as usize * SOURCE_BLOCK_VALUES {
        return 0;
    }
    let mut arm = ARM
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    *arm = Some(Arm {
        runtime: SessionRuntime::build_full(
            workload,
            PlanConfig::BASELINE,
            Backend::current(),
            SourceSignal::Injected(staged),
        ),
        hash: Sha256Sink::new(),
        render_errors: 0,
        digest: None,
    });
    1
}

/// Renders exactly one block at observation index `observation`.
///
/// **This is the only export a host may hold a clock around.** It performs one call to the
/// production render entry and nothing else: no hashing, no draining, no bookkeeping the native
/// subject does not also do inside its own timed region.
///
/// Returns `1` if the block rendered, `0` if the plan refused it or no arm is prepared. A refusal
/// is counted rather than reported through the return alone, so a run that fails to render still
/// reports how often it failed.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_render(observation: u32) -> u32 {
    let mut guard = ARM
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(arm) = guard.as_mut() else {
        return 0;
    };
    match arm.runtime.render(u64::from(observation)) {
        Ok(()) => 1,
        Err(_) => {
            arm.render_errors = arm.render_errors.saturating_add(1);
            0
        }
    }
}

/// Folds the block currently in the output buffer into this arm's running digest.
///
/// Outside the clock, always -- the host calls it between two timed regions, never inside one.
/// Returns `1` on success, `0` if no arm is prepared.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_hash_output() -> u32 {
    let mut guard = ARM
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(arm) = guard.as_mut() else {
        return 0;
    };
    let Arm { runtime, hash, .. } = arm;
    runtime.hash_output(hash);
    1
}

/// Finalises this arm's digest so it can be read a word at a time. Returns `1` on success.
///
/// Idempotent per prepared arm: the digest is computed once and memoised, so reading eight words
/// costs one finalisation rather than eight.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_finish_digest() -> u32 {
    let mut guard = ARM
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(arm) = guard.as_mut() else {
        return 0;
    };
    if arm.digest.is_none() {
        let mut bytes = [0_u8; 32];
        let hex = core::mem::replace(&mut arm.hash, Sha256Sink::new()).finish_hex();
        let hex = hex.as_bytes();
        if hex.len() != 64 {
            return 0;
        }
        for (index, byte) in bytes.iter_mut().enumerate() {
            let hi = (hex[index * 2] as char).to_digit(16);
            let lo = (hex[index * 2 + 1] as char).to_digit(16);
            match (hi, lo) {
                (Some(hi), Some(lo)) => *byte = (hi as u8) << 4 | lo as u8,
                _ => return 0,
            }
        }
        arm.digest = Some(bytes);
    }
    1
}

/// Word `index` (0..8, big-endian within the digest) of the finished digest.
///
/// Eight `u32` reads rather than one pointer, so the ABI stays `u32`-in/`u32`-out and the host
/// never has to reason about the guest's linear memory.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_digest_word(index: u32) -> u32 {
    let guard = ARM
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let Some(arm) = guard.as_ref() else {
        return 0;
    };
    let Some(digest) = arm.digest.as_ref() else {
        return 0;
    };
    let Some(chunk) = digest.chunks_exact(4).nth(index as usize) else {
        return 0;
    };
    u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]])
}

/// Blocks this arm's plan refused since it was prepared.
#[unsafe(no_mangle)]
pub extern "C" fn miso_console_render_errors() -> u32 {
    let guard = ARM
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    guard.as_ref().map_or(0, |arm| arm.render_errors)
}
