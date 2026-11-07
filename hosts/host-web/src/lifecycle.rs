//! The session's lifecycle word (#1332 S5, #2298): the one owner of whether a session is ready,
//! failed or disposed, shared by the control half and the render half on whatever threads they run.
//!
//! The word only moves forward. Its states are ordered, `STATE_READY < STATE_FAILED <
//! STATE_DISPOSED` (the ABI's own numbering), and its only writes are [`SessionLifecycle::fail`]
//! and [`SessionLifecycle::dispose`], each one `fetch_max`: a failure after a dispose leaves the
//! session disposed, and of two failures only the first moves the word out of ready, which
//! [`Ending`] reports so that the second records nothing of its own.
//!
//! A `fetch_max` is one read-modify-write. On wasm it lowers to a strong compare-exchange loop that
//! retries only when the word changed, and the word changes at most twice, so the loop is bounded:
//! no lock, allocation or syscall, which is why the render half may call it.

use crate::{STATE_DISPOSED, STATE_FAILED, STATE_READY};

// The word `--cfg loom` models: loom's atomic under loom, `core`'s otherwise, so
// `lifecycle_loom` explores the real transitions.
#[cfg(not(loom))]
use core::sync::atomic::{AtomicU32 as ModelledWord, Ordering};
#[cfg(loom)]
use loom::sync::atomic::{AtomicU32 as ModelledWord, Ordering};

// The order the transitions rely on; the ABI's constants already have it.
const _: () = assert!(STATE_READY < STATE_FAILED && STATE_FAILED < STATE_DISPOSED);

/// What a failure found: whether it ended the session or found it ended already.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
pub(crate) enum Ending {
    /// This failure moved the session out of [`STATE_READY`]: it is the session's failure.
    First,
    /// The session had already failed or been disposed; the word is unchanged.
    AlreadyEnded,
}

/// The session's lifecycle word: [`STATE_READY`] until the first failure ([`STATE_FAILED`]) or
/// the dispose ([`STATE_DISPOSED`]). One `fetch_max` (`AcqRel`) to move it, one `Acquire` load to
/// check it.
#[repr(transparent)]
pub(crate) struct SessionLifecycle(ModelledWord);

impl SessionLifecycle {
    /// A ready session's word.
    pub(crate) fn new() -> Self {
        Self(ModelledWord::new(STATE_READY))
    }

    /// The session's state: [`STATE_READY`], [`STATE_FAILED`] or [`STATE_DISPOSED`].
    pub(crate) fn state(&self) -> u32 {
        self.0.load(Ordering::Acquire)
    }

    /// Whether the session is ready.
    pub(crate) fn is_ready(&self) -> bool {
        self.state() == STATE_READY
    }

    /// Fail the session: [`Ending::First`] when this call moved it out of ready, and
    /// [`Ending::AlreadyEnded`] (the word unchanged) when it had failed or been disposed already.
    pub(crate) fn fail(&self) -> Ending {
        if self.0.fetch_max(STATE_FAILED, Ordering::AcqRel) == STATE_READY {
            Ending::First
        } else {
            Ending::AlreadyEnded
        }
    }

    /// Dispose the session, from any state.
    pub(crate) fn dispose(&self) {
        self.0.fetch_max(STATE_DISPOSED, Ordering::AcqRel);
    }

    /// Store `state` as it is, out of order, for the tests that drive a refusal of a session that
    /// is not ready and then recover it.
    #[cfg(all(test, feature = "test-support"))]
    pub(crate) fn force_for_test(&self, state: u32) {
        self.0.store(state, Ordering::Release);
    }
}

/// #2298: the control half's dispose or failure racing the render half's failure, on the real
/// word. Loom explores every order of the two read-modify-writes and asserts the transition rules:
/// the word ends at the greatest state written, a dispose is never undone, and of two failures
/// exactly one is the session's.
#[cfg(all(test, loom))]
mod lifecycle_loom {
    use super::{Ending, SessionLifecycle};
    use crate::{STATE_DISPOSED, STATE_FAILED};
    use loom::sync::Arc;

    /// What the control thread does while render fails.
    #[derive(Clone, Copy)]
    enum Control {
        Dispose,
        Fail,
    }

    fn race(control: Control) {
        loom::model(move || {
            let word = Arc::new(SessionLifecycle::new());
            let render_word = Arc::clone(&word);
            let render = loom::thread::spawn(move || render_word.fail());
            let control_ending = match control {
                Control::Dispose => {
                    word.dispose();
                    None
                }
                Control::Fail => Some(word.fail()),
            };
            let render_ending = render.join().expect("the render thread");
            match control_ending {
                None => {
                    assert_eq!(word.state(), STATE_DISPOSED, "a failure undid the dispose");
                }
                Some(control_ending) => {
                    assert_eq!(word.state(), STATE_FAILED);
                    assert_ne!(
                        control_ending == Ending::First,
                        render_ending == Ending::First,
                        "exactly one of two failures is the session's"
                    );
                }
            }
        });
    }

    #[test]
    fn lifecycle_loom() {
        race(Control::Dispose);
        race(Control::Fail);
    }
}
