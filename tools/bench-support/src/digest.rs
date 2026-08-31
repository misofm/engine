//! The one SHA-256 sink, and the counter that makes "no hashing inside the timer" structural.
//!
//! Audit #104 F1: the Issue-035 builtins benchmark hashed its own 1 KiB of output through 256
//! separate `Digest::update` calls *inside* the measured interval, so a workload on the order of
//! 1-2 microseconds reported a number dominated by 3-6 microseconds of SHA-256. Issue 072 fixed
//! that one benchmark by moving the hashing after `started.elapsed()`, and left the property
//! resting on a single `assert!` in a single tool.
//!
//! Every hash a bench takes now goes through [`Sha256Sink`], which counts its updates in a
//! thread-local. [`crate::timing::timed`] samples that counter on both sides of the clock and
//! panics if it moved. A benchmark cannot hash inside its timed region without the harness saying
//! so, in that benchmark's own test run, by name.

use core::cell::Cell;
use sha2::{Digest, Sha256};

thread_local! {
    static UPDATES: Cell<u64> = const { Cell::new(0) };
}

/// The current thread's sink-update count, to be compared with [`updates_since`].
#[must_use]
pub fn updates_mark() -> u64 {
    UPDATES.with(Cell::get)
}

/// How many sink updates this thread has made since `mark`.
#[must_use]
pub fn updates_since(mark: u64) -> u64 {
    updates_mark().saturating_sub(mark)
}

/// An incremental SHA-256 that counts its own updates.
pub struct Sha256Sink(Sha256);

impl Sha256Sink {
    /// A new, empty sink.
    #[must_use]
    pub fn new() -> Self {
        Self(Sha256::new())
    }

    /// Absorb `bytes`. Counted; never call this inside a timed interval.
    pub fn update(&mut self, bytes: impl AsRef<[u8]>) {
        UPDATES.with(|updates| updates.set(updates.get().saturating_add(1)));
        self.0.update(bytes);
    }

    /// The digest so far, without consuming the sink.
    #[must_use]
    pub fn snapshot_hex(&self) -> String {
        hex(&self.0.clone().finalize())
    }

    /// Consume the sink and return its lowercase hexadecimal digest.
    #[must_use]
    pub fn finish_hex(self) -> String {
        hex(&self.0.finalize())
    }
}

impl Default for Sha256Sink {
    fn default() -> Self {
        Self::new()
    }
}

/// The lowercase hexadecimal SHA-256 of `bytes`, in one call. Not counted: a one-shot digest of a
/// buffer that already exists is not incremental hashing inside a loop.
#[must_use]
pub fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn hex(digest: &[u8]) -> String {
    let mut text = String::with_capacity(digest.len() * 2);
    for byte in digest {
        text.push(char::from_digit(u32::from(byte >> 4), 16).expect("nibble"));
        text.push(char::from_digit(u32::from(byte & 0x0f), 16).expect("nibble"));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::{Sha256Sink, sha256_hex, updates_mark, updates_since};

    const EMPTY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";
    const ABC: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

    #[test]
    fn matches_the_published_vectors() {
        assert_eq!(sha256_hex(b""), EMPTY);
        assert_eq!(sha256_hex(b"abc"), ABC);
    }

    #[test]
    fn incremental_updates_match_a_one_shot_digest() {
        let mut sink = Sha256Sink::new();
        sink.update(b"a");
        sink.update(b"b");
        sink.update(b"c");
        assert_eq!(sink.finish_hex(), ABC);
    }

    #[test]
    fn snapshot_does_not_consume_the_sink() {
        let mut sink = Sha256Sink::new();
        sink.update(b"abc");
        assert_eq!(sink.snapshot_hex(), ABC);
        assert_eq!(sink.finish_hex(), ABC);
    }

    #[test]
    fn every_update_is_counted_and_a_one_shot_digest_is_not() {
        let mark = updates_mark();
        let mut sink = Sha256Sink::new();
        for _ in 0..8 {
            sink.update(b"x");
        }
        assert_eq!(updates_since(mark), 8);
        let _ = sha256_hex(b"x");
        assert_eq!(updates_since(mark), 8);
    }
}
