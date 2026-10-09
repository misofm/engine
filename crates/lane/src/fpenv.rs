//! The canonical floating-point environment, pinned at every native render entry (issue #146).
//!
//! # The defect this exists for
//!
//! Master plan #83 D7 gives the engine its own denormal mechanism: [`crate::flush`] zeroes a
//! recursive state word below `FLUSH_EPS` (`1e-20`, about `2^-66`), a band that strictly contains
//! the subnormal range hardware flush-to-zero acts on. Gate G6 in `tests/g6_ftz_inert.rs` proves
//! that law is FTZ-inert on the quantities it covers.
//!
//! It does not cover the whole render. Issue #144's full-corpus reproducer
//! (`tools/wasm-gates/tests/g6_full_corpus_ftz.rs`) found 69-70 of 331 corpus rows
//! rendering off-pin under hardware FTZ+DAZ: transient intra-block denormals in the recursive SVF,
//! the feed-forward lane, scalar math and the effect/builtin chains, none of which is a *state*
//! word the D7 flush can reach. Browser Wasm is unaffected -- the core specification mandates
//! denormal correctness and forbids a flush-to-zero mode, confirmed by the three-browser digest
//! parity -- so the exposure is exactly the native hosts, whose audio thread may run with FTZ and
//! DAZ set (a host can set them to avoid denormal stalls; the C ABI contract does not forbid it).
//!
//! # The decision
//!
//! Chasing per-kernel FTZ inertness was rejected. The engine instead *pins the environment it
//! renders in*: at every native render entry the caller's control word is saved, the canonical
//! word is installed, the block is rendered, and the caller's exact word is restored by
//! [`CanonicalFpEnv`]'s `Drop`. Restoration is unconditional and bit-exact, on the success path,
//! on every early-rejection path and while an unwind is in flight, because the guard is an ordinary
//! stack value. The engine never asks the host to change its thread; it borrows the environment for
//! the length of one block and hands it back exactly as it found it.
//!
//! This is the *pinned* half of the rule the legacy predecessor wrote as attestation
//! ([`docs/research/legacy-v2old/02-numerics-determinism.md`](https://github.com/misofm/engine/blob/5379e46ca3b349b9d277d642c008bb7a9643fb76/docs/research/legacy-v2old/02-numerics-determinism.md),
//! removed by #1031: "At every native render-thread
//! entry/start, require and attest round-to-nearest-even with FTZ/DAZ disabled ... refuse before
//! audio with stable `fp_environment_invalid`"). Refusing a DAW's callback thread is not available
//! to an embedded engine, so the current engine pins per block and keeps the attestation as a start-of-session
//! check that pinning actually takes on this thread -- [`attest_fp_environment`], called from the
//! host facade's started-session handle.
//!
//! # The canonical word
//!
//! `x86`/`x86_64`: `CANONICAL_MXCSR` is `0x1F80`, the architectural default MXCSR value -- all
//! six SIMD floating-point exceptions masked, round-to-nearest-even, FTZ clear, DAZ clear, status
//! flags clear (Intel 64 and IA-32 Architectures Software Developer's Manual, Volume 1, "MXCSR
//! Control and Status Register"). Installing the whole word rather than clearing two bits also
//! removes a caller's directed rounding mode and any unmasked exception that would trap inside a
//! render, both of which break determinism exactly as FTZ does.
//!
//! `aarch64`: `CANONICAL_FPCR` is `0`, the FPCR value with `RMode` round-to-nearest, `FZ` and
//! `FZ16` clear, every floating-point trap enable clear and the FEAT_AFP `AH`/`NEP`/`FIZ`
//! alternate-handling bits clear (Arm Architecture Reference Manual for A-profile, `FPCR`,
//! Floating-point Control Register). AArch64 has no separate DAZ bit: `FZ` flushes both subnormal
//! inputs and subnormal results, so clearing it is the whole of the fix on that target.
//!
//! Every other target -- `wasm32` above all -- has no reachable floating-point control word and
//! needs none: the WebAssembly core specification fixes round-to-nearest-even and full subnormal
//! arithmetic, with no mode that could change either. There, [`CanonicalFpEnv`] is a zero-sized
//! value with **no `Drop` implementation at all**, so the browser artifact's shipped code is
//! byte-identical to what it was before this module existed and the render export's call-graph
//! gate cannot see drop glue that is not emitted.
//!
//! # Why this file carries `unsafe`
//!
//! This file is the one new unsafe site of issue #146 -- one file, two reasons, allowlisted by
//! `scripts/check-realtime-policy.sh` and `scripts/check-lane-policy.sh` and recorded in
//! `docs/REALTIME_DEPENDENCY_POLICY.md`:
//!
//! 1. On `aarch64` there is no stable `core::arch` intrinsic for FPCR, so `mrs`/`msr` are issued
//!    through `core::arch::asm!`. On `x86` no such block is needed: the control word is reached
//!    through the already-approved `_mm_getcsr`/`_mm_setcsr` helpers of [`crate::softfma`].
//! 2. An empty `asm!` on both, as the guard's scheduling barrier. See `scheduling_barrier`.
//!
//! Since issue #1494 the raw writer [`write_fp_control_word`] is also an `unsafe fn`, because its
//! soundness depends on the word its caller passes; [`CanonicalFpEnv`] calls it in `unsafe` blocks
//! that state why each word it writes meets the contract.
//!
//! # Realtime properties
//!
//! No allocation, lock or syscall; the barriers emit nothing. Shipped `x86_64` cdylib: `enter`
//! calls `softfma::read_mxcsr` then `write_mxcsr` out of line, and `Drop` calls `write_mxcsr` on
//! each exit path; each helper is a stack-slot `STMXCSR`/`LDMXCSR`. AArch64 (Android object):
//! inline `mrs`/`msr`, no call. Cost: `artifacts/issue146/fp-environment-benchmark.raw.jsonl`.

#![allow(unsafe_code)]

use core::fmt;
use core::marker::PhantomData;

/// The architectural default MXCSR word: exceptions masked, round-to-nearest-even, FTZ/DAZ clear.
#[cfg(target_arch = "x86_64")]
pub const CANONICAL_MXCSR: u32 = 0x1F80;

/// The canonical AArch64 FPCR word: round-to-nearest, `FZ`/`FZ16` clear, no traps, no FEAT_AFP
/// alternate handling.
#[cfg(target_arch = "aarch64")]
pub const CANONICAL_FPCR: u64 = 0;

/// The MXCSR bits that are *control* state: DAZ, the six exception masks, the rounding-control
/// field and FTZ.
///
/// Bits 0-5 are sticky exception *status* flags, which ordinary arithmetic sets and which say
/// nothing about how the next operation will round or flush. An attestation asks whether the
/// canonical control state is in effect, so it compares under this mask; a *restore* is still
/// bit-exact over the whole word, because a caller's sticky flags are the caller's.
#[cfg(target_arch = "x86_64")]
pub const MXCSR_CONTROL_MASK: u32 = 0xFFC0;

/// The FPCR bits that are control state: all of them.
///
/// AArch64 keeps its sticky exception flags in FPSR, a different register, so FPCR carries no
/// status bits to mask off.
#[cfg(target_arch = "aarch64")]
pub const FPCR_CONTROL_MASK: u64 = u64::MAX;

/// AArch64 `FPCR.FZ`: subnormal inputs and results are flushed to zero.
#[cfg(target_arch = "aarch64")]
pub const FPCR_FZ: u64 = 1 << 24;

/// AArch64 `FPCR.FZ16`: half-precision subnormal flush.
#[cfg(target_arch = "aarch64")]
pub const FPCR_FZ16: u64 = 1 << 19;

/// Whether this target has a floating-point control word the engine pins.
///
/// `false` on `wasm32`, where the core specification fixes the behaviour the control word would
/// otherwise select.
pub const FP_ENV_CONTROLLED: bool = cfg!(any(target_arch = "x86_64", target_arch = "aarch64"));

/// The kind of the target's floating-point control word.
#[cfg(target_arch = "x86_64")]
pub type FpControlWord = u32;

/// The kind of the target's floating-point control word.
#[cfg(target_arch = "aarch64")]
pub type FpControlWord = u64;

/// The kind of the target's floating-point control word: the unit type, because this target has no
/// control word to name.
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub type FpControlWord = ();

/// Reads this thread's floating-point control word.
#[cfg(target_arch = "x86_64")]
#[inline]
#[must_use]
pub fn read_fp_control_word() -> FpControlWord {
    crate::softfma::read_mxcsr()
}

/// Writes this thread's floating-point control word.
///
/// [`CanonicalFpEnv`] is the safe way to change the word: it installs the canonical word and writes
/// the caller's own word back on drop. This raw writer is for that guard and for tests that install
/// other words on purpose. The write affects only the calling thread.
///
/// # Safety
///
/// - `value` sets no reserved bit: on `x86_64` none of MXCSR bits 16-31, where `LDMXCSR` raises
///   #GP; on AArch64 no `RES0` bit of FPCR.
/// - Rust assumes the floating-point environment of [`canonical_fp_control_word`] (on `x86_64`,
///   the `core::arch` documentation of `_mm_setcsr` names the default exception masks, rounding and
///   DAZ). Only the canonical word, or on `x86_64` a word that differs from it only in the MXCSR
///   status flags (bits 0-5), is inside that environment; the caller treats every other word as
///   outside it. A word read from this thread and handed back can be outside it too, because a host
///   can run with FTZ/DAZ (AArch64: `FZ`) set. The caller lets no compiled floating-point code run
///   under a word outside the environment except the code it deliberately measures. A hand-back
///   write meets this when it returns the thread to the word its caller already ran under and the
///   writer runs no floating-point code after it.
/// - Unless the write installs a word inside the environment or hands back a word read from this
///   thread, the caller restores the previous word before it returns, itself or through a guard
///   that does ([`CanonicalFpEnv`]'s `Drop`). A hand-back write is itself that restore. A word
///   inside the environment needs no restore, because compiled code may run under it; this is why
///   [`CanonicalFpEnv::enter`] stays sound when safe code passes the guard to `mem::forget`.
///
/// Calling it from safe code does not compile. rustc reports E0133 (call to unsafe function) for
/// the first fence; the fence carries no code because stable rustdoc does not check one (issue
/// #1422 D2).
///
/// ```compile_fail
/// let word = lane::fpenv::read_fp_control_word();
/// lane::fpenv::write_fp_control_word(word);
/// ```
///
/// The twin differs from it only in the `unsafe` block and its `SAFETY` comment, so a renamed item
/// turns it red:
///
/// ```
/// let word = lane::fpenv::read_fp_control_word();
/// // SAFETY: `word` was just read from this thread, so it sets no reserved bit, and writing it
/// // back leaves the thread's word unchanged.
/// unsafe { lane::fpenv::write_fp_control_word(word) };
/// ```
#[cfg(target_arch = "x86_64")]
#[inline]
pub unsafe fn write_fp_control_word(value: FpControlWord) {
    // SAFETY: this function's `# Safety` contract is `write_mxcsr`'s, and the caller upholds it.
    unsafe { crate::softfma::write_mxcsr(value) };
}

/// Reads this thread's floating-point control word.
#[cfg(target_arch = "aarch64")]
#[inline]
#[must_use]
pub fn read_fp_control_word() -> FpControlWord {
    let value: u64;
    // SAFETY: `MRS Xt, FPCR` reads an unprivileged, always-accessible system register on every
    // AArch64 profile the engine targets, and cannot trap at EL0. The output constraint gives the
    // compiler an ordinary register result.
    //
    // `nostack` is accurate -- no stack slot is touched. `nomem` is deliberately **not** given, to
    // the read as well as the write: the pair is an optimization barrier, and a block of assembly
    // that advertises itself as memory-free is one the optimizer may sink or hoist arithmetic
    // across. What FPCR selects is a side effect the compiler does not model, so the assembly must
    // not claim to have none.
    unsafe {
        core::arch::asm!("mrs {value}, fpcr", value = out(reg) value, options(nostack));
    }
    value
}

/// Writes this thread's floating-point control word.
///
/// [`CanonicalFpEnv`] is the safe way to change the word: it installs the canonical word and writes
/// the caller's own word back on drop. This raw writer is for that guard and for tests that install
/// other words on purpose. The write affects only the calling thread.
///
/// # Safety
///
/// - `value` sets no reserved bit: on `x86_64` none of MXCSR bits 16-31, where `LDMXCSR` raises
///   #GP; on AArch64 no `RES0` bit of FPCR.
/// - Rust assumes the floating-point environment of [`canonical_fp_control_word`] (on `x86_64`,
///   the `core::arch` documentation of `_mm_setcsr` names the default exception masks, rounding and
///   DAZ). Only the canonical word, or on `x86_64` a word that differs from it only in the MXCSR
///   status flags (bits 0-5), is inside that environment; the caller treats every other word as
///   outside it. A word read from this thread and handed back can be outside it too, because a host
///   can run with FTZ/DAZ (AArch64: `FZ`) set. The caller lets no compiled floating-point code run
///   under a word outside the environment except the code it deliberately measures. A hand-back
///   write meets this when it returns the thread to the word its caller already ran under and the
///   writer runs no floating-point code after it.
/// - Unless the write installs a word inside the environment or hands back a word read from this
///   thread, the caller restores the previous word before it returns, itself or through a guard
///   that does ([`CanonicalFpEnv`]'s `Drop`). A hand-back write is itself that restore. A word
///   inside the environment needs no restore, because compiled code may run under it; this is why
///   [`CanonicalFpEnv::enter`] stays sound when safe code passes the guard to `mem::forget`.
///
/// Calling it from safe code does not compile. rustc reports E0133 (call to unsafe function) for
/// the first fence; the fence carries no code because stable rustdoc does not check one (issue
/// #1422 D2).
///
/// ```compile_fail
/// let word = lane::fpenv::read_fp_control_word();
/// lane::fpenv::write_fp_control_word(word);
/// ```
///
/// The twin differs from it only in the `unsafe` block and its `SAFETY` comment, so a renamed item
/// turns it red:
///
/// ```
/// let word = lane::fpenv::read_fp_control_word();
/// // SAFETY: `word` was just read from this thread, so it sets no reserved bit, and writing it
/// // back leaves the thread's word unchanged.
/// unsafe { lane::fpenv::write_fp_control_word(word) };
/// ```
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn write_fp_control_word(value: FpControlWord) {
    // SAFETY: `MSR FPCR, Xt` writes an unprivileged, always-accessible system register on every
    // AArch64 profile the engine targets, affects only the calling thread, and cannot trap at EL0.
    // The caller upholds this function's `# Safety` contract: `value` sets no `RES0` bit, and the
    // caller confines a word outside Rust's floating-point model and restores the previous word
    // where the contract requires it.
    // `nomem` is omitted for the reason given on the read.
    unsafe {
        core::arch::asm!("msr fpcr, {value}", value = in(reg) value, options(nostack));
    }
}

/// Reads this thread's floating-point control word; there is none on this target.
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[inline]
#[must_use]
pub fn read_fp_control_word() -> FpControlWord {}

/// Writes this thread's floating-point control word; there is none on this target, so the write
/// does nothing.
///
/// It is `unsafe` all the same, so the API has one shape on every target and code written against
/// it here keeps compiling where the word exists.
///
/// [`CanonicalFpEnv`] is the safe way to change the word: it installs the canonical word and writes
/// the caller's own word back on drop. This raw writer is for that guard and for tests that install
/// other words on purpose. The write affects only the calling thread.
///
/// # Safety
///
/// - `value` sets no reserved bit: on `x86_64` none of MXCSR bits 16-31, where `LDMXCSR` raises
///   #GP; on AArch64 no `RES0` bit of FPCR.
/// - Rust assumes the floating-point environment of [`canonical_fp_control_word`] (on `x86_64`,
///   the `core::arch` documentation of `_mm_setcsr` names the default exception masks, rounding and
///   DAZ). Only the canonical word, or on `x86_64` a word that differs from it only in the MXCSR
///   status flags (bits 0-5), is inside that environment; the caller treats every other word as
///   outside it. A word read from this thread and handed back can be outside it too, because a host
///   can run with FTZ/DAZ (AArch64: `FZ`) set. The caller lets no compiled floating-point code run
///   under a word outside the environment except the code it deliberately measures. A hand-back
///   write meets this when it returns the thread to the word its caller already ran under and the
///   writer runs no floating-point code after it.
/// - Unless the write installs a word inside the environment or hands back a word read from this
///   thread, the caller restores the previous word before it returns, itself or through a guard
///   that does ([`CanonicalFpEnv`]'s `Drop`). A hand-back write is itself that restore. A word
///   inside the environment needs no restore, because compiled code may run under it; this is why
///   [`CanonicalFpEnv::enter`] stays sound when safe code passes the guard to `mem::forget`.
///
/// The two fences below are documentation only: this variant exists only on targets without a
/// control word (wasm32 here), rustdoc does not run doctests for `wasm32-unknown-unknown` in this
/// repository, and a native doctest run never compiles them. rustc reports E0133 (call to unsafe
/// function) for the first; the fence carries no code because stable rustdoc does not check one
/// (issue #1422 D2).
///
/// ```compile_fail
/// let word = lane::fpenv::read_fp_control_word();
/// lane::fpenv::write_fp_control_word(word);
/// ```
///
/// ```
/// let word = lane::fpenv::read_fp_control_word();
/// // SAFETY: there is no control word on this target, so the write does nothing.
/// unsafe { lane::fpenv::write_fp_control_word(word) };
/// ```
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[inline]
pub unsafe fn write_fp_control_word(value: FpControlWord) {
    let () = value;
}

/// The control-state half of a floating-point control word.
///
/// Two words with the same control bits round and flush identically; they may still differ in the
/// sticky exception flags a previous operation left behind. Attestation compares these; restoration
/// never does.
#[cfg(target_arch = "x86_64")]
#[inline]
#[must_use]
pub fn fp_control_bits(word: FpControlWord) -> FpControlWord {
    word & MXCSR_CONTROL_MASK
}

/// The control-state half of a floating-point control word.
#[cfg(target_arch = "aarch64")]
#[inline]
#[must_use]
pub fn fp_control_bits(word: FpControlWord) -> FpControlWord {
    word & FPCR_CONTROL_MASK
}

/// The control-state half of a floating-point control word; this target has none, so the unit
/// value passes through.
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[inline]
#[must_use]
pub fn fp_control_bits(word: FpControlWord) -> FpControlWord {
    word
}

/// Whether this thread is currently in the canonical floating-point control state.
///
/// This is what a render entry's session-start attestation asks after installing the guard, and
/// what a host asks if it wants to check the state it is about to render in. Sticky exception flags
/// are ignored; see [`fp_control_bits`].
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline]
#[must_use]
pub fn in_canonical_fp_environment() -> bool {
    fp_control_bits(read_fp_control_word()) == fp_control_bits(canonical_fp_control_word())
}

/// Whether this thread is currently in the canonical floating-point control state.
///
/// Always `true` on a target with no floating-point control word: the WebAssembly core
/// specification fixes round-to-nearest-even and full subnormal arithmetic, and offers no mode
/// that could select anything else.
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[inline]
#[must_use]
pub fn in_canonical_fp_environment() -> bool {
    true
}

/// The canonical control word for this target.
#[inline]
#[must_use]
pub fn canonical_fp_control_word() -> FpControlWord {
    #[cfg(target_arch = "x86_64")]
    {
        CANONICAL_MXCSR
    }
    #[cfg(target_arch = "aarch64")]
    {
        CANONICAL_FPCR
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {}
}

/// The pinned floating-point environment of one render call.
///
/// Construct one at the top of a render entry and let it live for the whole call. On construction
/// the caller's control word is saved and the canonical word installed; on drop the caller's exact
/// word is written back, whether the entry returned a value, returned a rejection code, or is
/// unwinding.
///
/// The guard is neither `Send` nor `Sync`: a control word belongs to one thread, so a guard that
/// could be moved or shared across threads could restore one thread's word onto another's.
/// rustc reports E0277 for each (`Send`, then `Sync`, is not implemented); the fences carry no
/// code because stable rustdoc does not check one.
///
/// ```compile_fail
/// fn requires<T: Send>() {}
/// requires::<lane::fpenv::CanonicalFpEnv>();
/// ```
///
/// ```compile_fail
/// fn requires<T: Sync>() {}
/// requires::<lane::fpenv::CanonicalFpEnv>();
/// ```
///
/// The twin of both: each doctest above differs from it in its one bound only, so a renamed type
/// turns it red (issue #1422 D2).
///
/// ```
/// fn requires<T>() {}
/// requires::<lane::fpenv::CanonicalFpEnv>();
/// ```
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub struct CanonicalFpEnv {
    saved: FpControlWord,
    _not_send_not_sync: PhantomData<*const ()>,
}

/// An empty assembly block that no load or store may be moved across.
///
/// Installing a control word is a side effect the optimizer does not model. `_mm_setcsr` lowers to
/// `llvm.x86.sse.ldmxcsr`, which is declared as touching only its own argument's memory, so nothing
/// in the intrinsic itself tells LLVM that the arithmetic around it now means something different.
/// This barrier says the only thing that can be said in one line: no memory operation crosses here.
/// That is enough for a render, whose every value is loaded from and stored to the output block,
/// the plan state and the source rings -- the arithmetic is anchored by the loads it depends on.
///
/// It is *not* enough for a computation held entirely in registers, and
/// `crates/lane/tests/fp_env.rs` proves that rather than assuming it: its
/// register-only subnormal product needs a `black_box` of its own, and without one the optimizer
/// really does schedule the multiply outside the guarded region in a release build. A render never
/// has that shape, and a caller who wants a register-only value computed under the canonical
/// environment must anchor it the same way.
///
/// The barrier emits no instructions.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[inline(always)]
fn scheduling_barrier() {
    // SAFETY: an empty assembly template with no operands. It cannot fault, cannot write a
    // register and cannot diverge. `nomem` is deliberately absent -- being a memory clobber is the
    // entire purpose -- and `preserves_flags` is accurate because the block contains nothing.
    unsafe {
        core::arch::asm!("", options(nostack, preserves_flags));
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
impl CanonicalFpEnv {
    /// Save the caller's control word and install the canonical one.
    #[inline]
    #[must_use]
    pub fn enter() -> Self {
        let saved = read_fp_control_word();
        // SAFETY: the canonical word sets no reserved bit and is the environment Rust assumes, so
        // compiled code may run under it and the contract asks for no restore. Soundness therefore
        // does not depend on `Drop` running: a guard passed to `mem::forget` leaves the canonical
        // word installed, which loses the caller's word but leaves no code outside Rust's model.
        // This guard's `Drop` writes `saved` back so that the caller gets its own word again.
        unsafe { write_fp_control_word(canonical_fp_control_word()) };
        scheduling_barrier();
        Self {
            saved,
            _not_send_not_sync: PhantomData,
        }
    }

    /// The exact control word this guard will restore.
    #[inline]
    #[must_use]
    pub fn saved_word(&self) -> FpControlWord {
        self.saved
    }
}

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
impl Drop for CanonicalFpEnv {
    #[inline]
    fn drop(&mut self) {
        scheduling_barrier();
        // SAFETY: `enter` read `self.saved` from this thread (the guard is `!Send`, so this is the
        // same thread), so it sets no reserved bit. Writing it hands the caller's own word back,
        // which is the restore: it returns the thread to the word the caller already ran under,
        // and the guard runs no floating-point code after this write (its barrier is above).
        unsafe { write_fp_control_word(self.saved) };
    }
}

/// The pinned floating-point environment of one render call.
///
/// On a target without a floating-point control word this is a zero-sized value with no `Drop`
/// implementation, so a render entry that constructs one emits no code for it at all. It is still
/// neither `Send` nor `Sync`, so a host cannot write code against the portable guard that would
/// stop compiling on a target that does pin.
///
/// The two fences below are documentation only: this type exists only on targets without a
/// control word (wasm32 here), rustdoc does not run doctests for `wasm32-unknown-unknown` in this
/// repository, and a native doctest run never compiles them (issue #1422 non-goal).
///
/// ```compile_fail
/// fn requires_send<T: Send>() {}
/// requires_send::<lane::fpenv::CanonicalFpEnv>();
/// ```
///
/// ```compile_fail
/// fn requires_sync<T: Sync>() {}
/// requires_sync::<lane::fpenv::CanonicalFpEnv>();
/// ```
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub struct CanonicalFpEnv {
    _not_send_not_sync: PhantomData<*const ()>,
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
impl CanonicalFpEnv {
    /// Enter the canonical environment; a no-op on a target without a control word.
    #[inline]
    #[must_use]
    pub fn enter() -> Self {
        Self {
            _not_send_not_sync: PhantomData,
        }
    }

    /// The exact control word this guard will restore; there is none on this target.
    #[inline]
    #[must_use]
    pub fn saved_word(&self) -> FpControlWord {}
}

/// Why a render thread's floating-point environment could not be pinned.
///
/// The stable name the legacy predecessor gave this refusal is `fp_environment_invalid`; it is what
/// [`fmt::Display`] prints, so a host that surfaces the rejection surfaces that exact token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FpEnvironmentRejection {
    /// The control word the thread carried when the attestation started.
    pub observed: FpControlWord,
    /// The control word the attestation installed and then read back.
    pub readback: FpControlWord,
    /// Which half of the attestation failed.
    pub failure: FpEnvironmentFailure,
}

/// Which half of [`attest_fp_environment`] failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FpEnvironmentFailure {
    /// The canonical word did not take: the control word read back as something else.
    CanonicalWordRefused,
    /// The caller's word was not restored bit-exactly when the guard dropped.
    RestoreRefused,
}

impl fmt::Display for FpEnvironmentRejection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let half = match self.failure {
            FpEnvironmentFailure::CanonicalWordRefused => "canonical word refused",
            FpEnvironmentFailure::RestoreRefused => "caller word not restored",
        };
        write!(formatter, "fp_environment_invalid: {half}")
    }
}

impl core::error::Error for FpEnvironmentRejection {}

/// Attest, on the calling thread, that the canonical environment can be pinned and unpinned.
///
/// This is the start-of-session half of issue #146: [`CanonicalFpEnv`] pins per block, and a host
/// calls this once from the render thread it is about to render on, so that a thread whose control
/// word cannot be written is refused before audio rather than silently rendering off-pin. It is a
/// control-plane call -- it runs before the first block, not inside one -- but it allocates
/// nothing and would be render-safe anyway.
///
/// # Errors
///
/// Returns [`FpEnvironmentRejection`] if the canonical word does not read back after installation,
/// or if the caller's exact word is not restored when the guard drops.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
pub fn attest_fp_environment() -> Result<(), FpEnvironmentRejection> {
    let observed = read_fp_control_word();
    let readback = {
        let _pinned = CanonicalFpEnv::enter();
        read_fp_control_word()
    };
    if fp_control_bits(readback) != fp_control_bits(canonical_fp_control_word()) {
        return Err(FpEnvironmentRejection {
            observed,
            readback,
            failure: FpEnvironmentFailure::CanonicalWordRefused,
        });
    }
    let restored = read_fp_control_word();
    if restored != observed {
        return Err(FpEnvironmentRejection {
            observed,
            readback: restored,
            failure: FpEnvironmentFailure::RestoreRefused,
        });
    }
    Ok(())
}

/// Attest, on the calling thread, that the canonical environment can be pinned and unpinned.
///
/// Always `Ok` on a target with no floating-point control word: there is no mode to install and
/// nothing that could fail. The function exists on every target so a host writes one call.
///
/// # Errors
///
/// Never on this target.
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
pub fn attest_fp_environment() -> Result<(), FpEnvironmentRejection> {
    let _pinned = CanonicalFpEnv::enter();
    Ok(())
}
