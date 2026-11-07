//! The controller's token lock (#2359): one word that says whether an affine prepared token
//! (structural, session command or observation read) is outstanding, and the one RAII lease that
//! holds it.
//!
//! The word counts acquisitions and releases: even is idle, odd is one lease outstanding, and the
//! lease remembers the odd value it took. Only this module reads or writes the word, so the parity
//! has one home. A lease is released exactly once, by a compare-and-swap from its own generation
//! to the next one, whether the token completes (the controller drops the lease after its last
//! write) or is dropped unused, on any thread. Allocation-free: acquiring clones an `Arc`.

// Under `--cfg loom` this crate's own test build models the word with loom's atomic and `Arc`
// (`token_lease_loom`); a dependent's loom build, which has no `cfg(test)` here, keeps `std`'s.
#[cfg(all(loom, test))]
use loom::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
#[cfg(not(all(loom, test)))]
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

/// The lock word's owner, kept by the controller.
pub(super) struct TokenLock {
    word: Arc<AtomicU64>,
}

/// Why [`TokenLock::acquire`] took no lease.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum LeaseRefusal {
    /// A lease is outstanding.
    Outstanding,
    /// The word cannot count another acquire and its release (`2^63` tokens; unreachable).
    Exhausted,
}

/// The token lock, held: affine, not `Clone`, made only by [`TokenLock::acquire`], and released
/// on drop.
pub(super) struct TokenLease {
    word: Arc<AtomicU64>,
    /// The odd value this lease took; below `u64::MAX`, so its release cannot overflow.
    generation: u64,
}

impl TokenLock {
    /// An idle lock.
    pub(super) fn new() -> Self {
        Self {
            word: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Whether no lease is outstanding.
    pub(super) fn is_idle(&self) -> bool {
        self.word.load(Ordering::Acquire) & 1 == 0
    }

    /// Take the lock.
    pub(super) fn acquire(&self) -> Result<TokenLease, LeaseRefusal> {
        let current = self.word.load(Ordering::Acquire);
        if current & 1 != 0 {
            return Err(LeaseRefusal::Outstanding);
        }
        // `current` is even, so `current + 1` is odd; refusing `u64::MAX - 1` keeps the
        // release's `generation + 1` in range.
        if current >= u64::MAX - 1 {
            return Err(LeaseRefusal::Exhausted);
        }
        let generation = current + 1;
        self.word
            .compare_exchange(current, generation, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| LeaseRefusal::Outstanding)?;
        Ok(TokenLease {
            word: Arc::clone(&self.word),
            generation,
        })
    }

    /// Whether this lock issued `lease`.
    pub(super) fn issued(&self, lease: &TokenLease) -> bool {
        Arc::ptr_eq(&self.word, &lease.word)
    }

    /// Whether this lock issued `lease` and `lease` still holds it.
    pub(super) fn is_current(&self, lease: &TokenLease) -> bool {
        self.issued(lease) && self.word.load(Ordering::Acquire) == lease.generation
    }

    /// The raw word, for tests that fake a stale lease.
    #[cfg(test)]
    pub(super) fn word(&self) -> &AtomicU64 {
        &self.word
    }
}

impl Drop for TokenLease {
    fn drop(&mut self) {
        // Fails only for a lease no longer current (a test's faked staleness): the word moved on
        // without it and is not this lease's to release.
        let _ = self.word.compare_exchange(
            self.generation,
            self.generation + 1,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The word's arithmetic edge: a lock whose next lease could not be released refuses it, where
    /// the base saturated the release and left the lock held forever. Red if `acquire` takes the
    /// last odd value.
    #[cfg(not(loom))]
    #[test]
    fn an_exhausted_lock_refuses_rather_than_wedging() {
        let lock = TokenLock::new();
        lock.word.store(u64::MAX - 3, Ordering::Release);
        let lease = lock.acquire().expect("the last releasable generation");
        assert_eq!(lock.acquire().map(drop), Err(LeaseRefusal::Outstanding));
        drop(lease);
        assert!(lock.is_idle());
        assert_eq!(lock.acquire().map(drop), Err(LeaseRefusal::Exhausted));
        assert!(lock.is_idle());
    }

    /// Issue #2359: the lease handoff under `loom`. The controller thread acquires a lease and
    /// hands it to a host thread, which writes the guarded data (the stand-in for the reservation
    /// counts a token returns before its lease) and drops the lease, while the controller thread
    /// tries again and, holding the lock, reads that data. A stale lease from another lock is
    /// dropped beside them.
    ///
    /// The guarded data is a `Relaxed` word, so only the lease's own orderings make the holder's
    /// write visible: loom lets a `Relaxed` load return any value the happens-before order allows,
    /// so without them it returns the stale 0 (the crate denies `unsafe`, so no `UnsafeCell`).
    ///
    /// Red if two leases are ever held at once or the release does not order the holder's writes
    /// before the next holder's reads (a drop CAS or the acquire weakened to `Relaxed`, or an
    /// acquire that does not see the outstanding lease), if a drop on another
    /// thread fails to release (the lock stays held after both threads finish), or if a foreign
    /// lease's drop releases this lock.
    #[cfg(loom)]
    mod token_lease_loom {
        use super::*;

        #[test]
        fn a_lease_dropped_on_another_thread_releases_exactly_once() {
            loom::model(|| {
                let lock = Arc::new(TokenLock::new());
                let other = TokenLock::new();
                let guarded = Arc::new(AtomicU64::new(0));
                let first = lock.acquire().expect("an idle lock");
                let foreign = other.acquire().expect("another idle lock");
                assert!(!lock.issued(&foreign));

                let host_guarded = Arc::clone(&guarded);
                let host = loom::thread::spawn(move || {
                    drop(foreign);
                    host_guarded.store(1, Ordering::Relaxed);
                    drop(first);
                });

                if let Ok(second) = lock.acquire() {
                    // The first holder's write happened before its release, which this acquire
                    // observed. Read before any other load of the word, so the acquire's own
                    // ordering is the one that makes it visible.
                    let seen = guarded.load(Ordering::Relaxed);
                    assert_eq!(seen, 1, "the first holder's write precedes this lease");
                    assert!(lock.is_current(&second));
                    drop(second);
                    host.join().expect("host thread");
                } else {
                    host.join().expect("host thread");
                    assert!(lock.is_idle(), "the host's drop released the lock");
                }
                assert!(lock.is_idle());
                assert!(other.is_idle());
                let third = lock.acquire().expect("the lock is idle again");
                assert!(lock.is_current(&third));
                // The host's write is ordered before `third` by its release.
                assert_eq!(guarded.load(Ordering::Relaxed), 1);
            });
        }
    }
}
