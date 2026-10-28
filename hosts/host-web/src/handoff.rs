//! The handoff of a render half between two instances on one shared memory (issue #1332 S6).
//!
//! # The token registry
//!
//! `miso_engine_web_v1_host_release` deposits the render half it takes out of its instance in a
//! fixed registry of [`HANDOFF_SLOTS`] entries and returns a token; `miso_engine_web_v1_host_adopt`
//! on another instance (or the same one) takes it back out by that token. The registry is a set of
//! process statics, so on the browser's shared `WebAssembly.Memory` both instances address one copy
//! (the first instance initialises the data segments, #1331 recipe rule 1), and natively it is the
//! process's. A token is `handle << 32 | sequence`: the host handle the half belongs to, and a
//! sequence number unique to the release (never `0` and never `u32::MAX`), so a token that was
//! taken, forged or mistyped matches no entry and nothing is dereferenced for it.
//!
//! Each entry is two words. `half` is the half's address, or `0` when the entry is free; `token`
//! is the token that names it, `0` while it names none, or [`CLAIMED`] while an adoption decides.
//! A deposit claims a free entry by a compare-and-swap of `half` from `0`, then stores the token
//! (`Release`). An adoption claims the entry by a compare-and-swap of the token to [`CLAIMED`]
//! (`Acquire`, which sees the half the deposit stored before its token), then either confirms (the
//! token goes to `0` first and `half` to `0` last, so no deposit reuses the entry while the token
//! word is still being written) or cancels (the half's address and the token are stored back, and
//! the token is unconsumed). Every operation is a bounded scan of four entries: no allocation, no
//! free, no lock and no wait, so both exports may run on the render thread.
//!
//! # The instance reservation
//!
//! A second instance on the shared memory has no stack of its own: its `__stack_pointer` starts at
//! the first instance's stack top and its `__tls_base` at `0` (#1331 recipe rule 2). Before any of
//! its exports runs, the page gives it the stack and TLS block that
//! `miso_engine_web_v1_instance_reserve` allocated on the first instance: it calls
//! `__wasm_init_tls(tls_base)`, sets `__stack_pointer` to `stack_top`, then calls
//! `miso_engine_web_v1_instance_bind` with the reservation's record. The stack is
//! [`INSTANCE_STACK_BYTES`] (owner decision O6) with a [`CANARY_BYTES`] canary below its bottom;
//! the stack grows down, so an overflow writes the canary first. There is no guard page, so the
//! canary is the only check: `host_release` on a bound instance checks it and, if any byte moved,
//! fails the session before it hands the half back. The block is leaked for the instance's life:
//! the memory never shrinks and no instance can free another's stack.

#![allow(unsafe_code)]

use crate::RenderHost;
use core::sync::atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering};

/// How many released render halves the registry holds at once. One Worker instance holds one host
/// and releases one half; the other entries are slack for a release whose adoption is pending
/// while a second host (a test's, or a page that boots again) releases its own.
pub const HANDOFF_SLOTS: usize = 4;

/// The token word of an entry an adoption has claimed and not yet confirmed or cancelled.
const CLAIMED: u64 = u64::MAX;

struct Entry {
    half: AtomicUsize,
    token: AtomicU64,
}

impl Entry {
    const fn new() -> Self {
        Self {
            half: AtomicUsize::new(0),
            token: AtomicU64::new(0),
        }
    }
}

/// The half as one address. Nothing is freed; the address must come back through [`from_raw`].
fn into_raw(render: RenderHost) -> usize {
    Box::into_raw(render.half).expose_provenance()
}

/// The half [`into_raw`] gave out.
///
/// # Safety
///
/// `raw` came from [`into_raw`] and has not been passed here since.
unsafe fn from_raw(raw: usize) -> RenderHost {
    // SAFETY: the caller's contract: `raw` is a live `Box<RenderHalf>` given out once, whose
    // provenance `into_raw` exposed.
    let half = unsafe { Box::from_raw(core::ptr::with_exposed_provenance_mut(raw)) };
    RenderHost { half }
}

static ENTRIES: [Entry; HANDOFF_SLOTS] = [const { Entry::new() }; HANDOFF_SLOTS];
static NEXT_SEQUENCE: AtomicU32 = AtomicU32::new(1);

// REALTIME_POLICY_BEGIN: #1332 S6, the handoff registry: bounded scans of atomics, nothing freed.
/// Deposit `render`, which belongs to the host `handle` names, and return its token, or hand it
/// back when every entry is taken.
pub(crate) fn deposit(handle: u32, render: RenderHost) -> Result<u64, RenderHost> {
    let mut sequence = NEXT_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    if sequence == 0 || sequence == u32::MAX {
        sequence = NEXT_SEQUENCE.fetch_add(2, Ordering::Relaxed).max(1);
    }
    let token = (u64::from(handle) << 32) | u64::from(sequence);
    let raw = into_raw(render);
    for entry in &ENTRIES {
        if entry
            .half
            .compare_exchange(0, raw, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
        {
            entry.token.store(token, Ordering::Release);
            return Ok(token);
        }
    }
    // SAFETY: `raw` came from `into_raw` just above and no entry took it.
    Err(unsafe { from_raw(raw) })
}

/// Take the half `token` names and offer it to `accept`. `None` when no entry holds that token
/// (taken, forged or never issued): nothing changes. `Some(Ok)` when `accept` kept it: the token
/// is consumed. `Some(Err)` when `accept` handed it back: it is deposited again under the same
/// token, unconsumed. While one instance decides on a token, another's `take` of the same token
/// finds no entry and is refused: an adopter never waits.
pub(crate) fn take<R>(
    token: u64,
    accept: impl FnOnce(RenderHost) -> Result<R, RenderHost>,
) -> Option<Result<R, ()>> {
    if token == 0 || token == CLAIMED {
        return None;
    }
    let entry = ENTRIES.iter().find(|entry| {
        entry
            .token
            .compare_exchange(token, CLAIMED, Ordering::AcqRel, Ordering::Relaxed)
            .is_ok()
    })?;
    let raw = entry.half.load(Ordering::Acquire);
    // SAFETY: the token named this entry, so the deposit stored a live half's address before it
    // stored the token, and the claim above is the only one that can succeed for it.
    let render = unsafe { from_raw(raw) };
    match accept(render) {
        Ok(value) => {
            entry.token.store(0, Ordering::Release);
            entry.half.store(0, Ordering::Release);
            Some(Ok(value))
        }
        Err(render) => {
            entry.half.store(into_raw(render), Ordering::Release);
            entry.token.store(token, Ordering::Release);
            Some(Err(()))
        }
    }
}

/// The host handle a token names.
pub(crate) const fn token_handle(token: u64) -> u32 {
    (token >> 32) as u32
}
// REALTIME_POLICY_END

/// The stack an instance reservation gives a second instance (owner decision O6).
pub const INSTANCE_STACK_BYTES: usize = 2 << 20;

/// The canary below the reserved stack's bottom.
pub const CANARY_BYTES: usize = 64;

/// The canary's byte pattern, repeated: `misostk!`.
const CANARY_PATTERN: [u8; 8] = *b"misostk!";

/// How many reservations a memory holds, which the registry below lists so that a bind names
/// only a reservation this module made.
const RESERVATIONS: usize = 4;

/// One instance reservation's record, at the address `miso_engine_web_v1_instance_reserve`
/// returns: four little-endian `u32` words a page reads before it starts the second instance.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WebInstanceReservation {
    /// `16`, the record's size.
    pub struct_size: u32,
    /// The TLS block's address: the argument of `__wasm_init_tls`.
    pub tls_base: u32,
    /// The stack's top, 16-aligned: the second instance's `__stack_pointer`.
    pub stack_top: u32,
    /// The stack's lowest address; the canary is the [`CANARY_BYTES`] below it.
    pub stack_bottom: u32,
}

/// The full-width addresses of one reservation, for a native caller whose addresses do not fit
/// the record's words.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Reservation {
    /// The record's address.
    pub record: usize,
    /// The canary's address.
    pub canary: usize,
    /// The stack's lowest address.
    pub stack_bottom: usize,
    /// The stack's top.
    pub stack_top: usize,
    /// The TLS block's address.
    pub tls_base: usize,
}

struct Reserved {
    record: AtomicUsize,
    canary: AtomicUsize,
}

static RESERVED: [Reserved; RESERVATIONS] = [const {
    Reserved {
        record: AtomicUsize::new(0),
        canary: AtomicUsize::new(0),
    }
}; RESERVATIONS];

/// Allocate one instance's stack and TLS block and its record, leaked for the instance's life
/// (#1332 S6). `tls_size` and `tls_align` are the module's `__tls_size` and `__tls_align`
/// globals, which only the page can read. `None` for an alignment that is not a power of two, a
/// size the layout cannot hold, a refused allocation or a full list of reservations.
///
/// Control work: it allocates, and only the first instance calls it, before the second starts.
pub(crate) fn reserve(tls_size: u32, tls_align: u32) -> Option<Reservation> {
    let slot = RESERVED
        .iter()
        .find(|slot| slot.record.load(Ordering::Acquire) == 0)?;
    let tls_align = usize::try_from(tls_align.max(16)).ok()?;
    if !tls_align.is_power_of_two() {
        return None;
    }
    let tls_size = usize::try_from(tls_size).ok()?;
    let align = tls_align.max(16);
    // [canary][stack][TLS], each part aligned: the canary and the stack are multiples of 16.
    let tls_offset = (CANARY_BYTES + INSTANCE_STACK_BYTES).checked_next_multiple_of(align)?;
    let bytes = tls_offset.checked_add(tls_size.max(1))?;
    let layout = core::alloc::Layout::from_size_align(bytes, align).ok()?;
    // SAFETY: the layout has a nonzero size.
    let base = unsafe { std::alloc::alloc_zeroed(layout) };
    if base.is_null() {
        return None;
    }
    // SAFETY: the canary is the block's first `CANARY_BYTES`, inside the allocation.
    let canary = unsafe { core::slice::from_raw_parts_mut(base, CANARY_BYTES) };
    for chunk in canary.as_chunks_mut::<{ CANARY_PATTERN.len() }>().0 {
        *chunk = CANARY_PATTERN;
    }
    let base = base.expose_provenance();
    let stack_bottom = base + CANARY_BYTES;
    let stack_top = base + CANARY_BYTES + INSTANCE_STACK_BYTES;
    let tls_base = base + tls_offset;
    let word = |address: usize| u32::try_from(address).unwrap_or(0);
    let record: &'static mut WebInstanceReservation = Box::leak(Box::new(WebInstanceReservation {
        struct_size: u32::try_from(size_of::<WebInstanceReservation>()).unwrap_or(0),
        tls_base: word(tls_base),
        stack_top: word(stack_top),
        stack_bottom: word(stack_bottom),
    }));
    let record = core::ptr::from_mut(record).expose_provenance();
    if slot
        .record
        .compare_exchange(0, record, Ordering::AcqRel, Ordering::Relaxed)
        .is_err()
    {
        // Another reservation took the slot meanwhile: this one stays leaked, unlisted.
        return None;
    }
    slot.canary.store(base, Ordering::Release);
    Some(Reservation {
        record,
        canary: base,
        stack_bottom,
        stack_top,
        tls_base,
    })
}

thread_local! {
    /// The canary address of the reservation this instance runs on, `0` for none (the first
    /// instance's stack is the module's own). Const and destructor-free (issue #1333 D5).
    static BOUND_CANARY: core::cell::Cell<usize> = const { core::cell::Cell::new(0) };
}

// REALTIME_POLICY_BEGIN: #1332 S6, the bind and the canary check: loads and compares only.
/// Bind this instance to the reservation whose record is at `record`: `true` when this module
/// made it and its canary is intact. Nothing is dereferenced for an address it did not make.
pub(crate) fn bind(record: usize) -> bool {
    if record == 0 {
        return false;
    }
    let Some(slot) = RESERVED
        .iter()
        .find(|slot| slot.record.load(Ordering::Acquire) == record)
    else {
        return false;
    };
    let canary = slot.canary.load(Ordering::Acquire);
    if canary == 0 || !canary_intact(canary) {
        return false;
    }
    BOUND_CANARY.with(|bound| bound.set(canary));
    true
}

/// Whether the canary of the reservation this instance is bound to is intact; `true` for an
/// instance bound to none.
pub(crate) fn bound_canary_intact() -> bool {
    let canary = BOUND_CANARY.try_with(core::cell::Cell::get).unwrap_or(0);
    canary == 0 || canary_intact(canary)
}

fn canary_intact(canary: usize) -> bool {
    let canary: *const u8 = core::ptr::with_exposed_provenance(canary);
    (0..CANARY_BYTES).all(|offset| {
        // SAFETY: `canary` is a listed reservation's block, whose first `CANARY_BYTES` are the
        // canary; the block is leaked, so it is live. A volatile read: an overflow writes it
        // behind the compiler's back.
        let byte = unsafe { canary.add(offset).read_volatile() };
        byte == CANARY_PATTERN[offset % CANARY_PATTERN.len()]
    })
}
// REALTIME_POLICY_END

/// Overwrite one byte of `reservation`'s canary, as a stack overflow would (#1332 S6 gate).
/// Test support only.
#[cfg(feature = "test-support")]
pub(crate) fn overwrite_canary_for_test(reservation: &Reservation) {
    let canary: *mut u8 = core::ptr::with_exposed_provenance_mut(reservation.canary);
    // SAFETY: the canary is the first `CANARY_BYTES` of a live, leaked reservation block.
    unsafe { canary.add(CANARY_BYTES - 1).write_volatile(0) };
}
