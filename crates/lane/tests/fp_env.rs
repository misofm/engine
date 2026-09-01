//! Issue #146 E2: the canonical floating-point environment is installed and handed back exactly.
//!
//! These are the guard's own properties, proved on the one target this host can execute: the
//! canonical word really takes, the caller's word comes back bit-for-bit including while an unwind
//! is in flight, and arithmetic inside the guard is denormal-correct even though the caller asked
//! for flush-to-zero. The engine-wide claim -- that a whole rendered block is unchanged by a
//! caller's FTZ -- is gate G6 over the full corpus
//! (`tools/wasm-gates/tests/g6_full_corpus_ftz.rs`).
//!
//! Red mutations (recorded in `tests/MUTATIONS.md`): drop the `Drop` implementation of
//! `CanonicalFpEnv`, or make `enter` install the caller's word instead of the canonical one.

use lane::fpenv::{
    self, CanonicalFpEnv, FP_ENV_CONTROLLED, attest_fp_environment, canonical_fp_control_word,
    read_fp_control_word,
};

#[test]
fn attestation_passes_on_this_thread() {
    attest_fp_environment().expect("the canonical environment must be attestable on this thread");
}

#[test]
fn the_canonical_word_is_the_word_inside_the_guard() {
    let outside = read_fp_control_word();
    {
        let pinned = CanonicalFpEnv::enter();
        assert_eq!(
            read_fp_control_word(),
            canonical_fp_control_word(),
            "the guard must install the canonical control word"
        );
        assert_eq!(
            pinned.saved_word(),
            outside,
            "the guard must remember the caller's exact word"
        );
    }
    assert_eq!(
        read_fp_control_word(),
        outside,
        "the guard must restore the caller's exact word"
    );
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86 {
    use super::{CanonicalFpEnv, attest_fp_environment, fpenv, read_fp_control_word};
    use lane::softfma::{MXCSR_DAZ, MXCSR_FTZ, read_mxcsr, write_mxcsr};

    /// MXCSR rounding-control field: `10` is round-toward-`+inf`.
    const MXCSR_RC_UP: u32 = 0x4000;

    /// MXCSR precision-exception status flag; a caller may arrive with sticky flags set.
    const MXCSR_PE_FLAG: u32 = 0x0020;

    /// Restores the calling thread's word however the body leaves, so one failing assertion cannot
    /// poison the rest of the test binary's thread.
    struct Restore(u32);

    impl Drop for Restore {
        fn drop(&mut self) {
            write_mxcsr(self.0);
        }
    }

    /// A control word no host would want a render to inherit: flush-to-zero, denormals-are-zero,
    /// a directed rounding mode, and a sticky status flag that must survive the round trip.
    fn hostile_word(base: u32) -> u32 {
        (base & !0x6000) | MXCSR_FTZ | MXCSR_DAZ | MXCSR_RC_UP | MXCSR_PE_FLAG
    }

    #[test]
    fn a_hostile_caller_word_is_normalised_and_returned_bit_exactly() {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        let hostile = hostile_word(saved);
        write_mxcsr(hostile);
        assert_eq!(
            read_mxcsr(),
            hostile,
            "the test must really install FTZ+DAZ"
        );

        {
            let _pinned = CanonicalFpEnv::enter();
            assert_eq!(
                read_mxcsr(),
                fpenv::CANONICAL_MXCSR,
                "the guard must install 0x1F80: masked, round-to-nearest-even, no FTZ, no DAZ"
            );
        }
        assert_eq!(
            read_mxcsr(),
            hostile,
            "every bit of the caller's word must come back, status flags included"
        );
    }

    #[test]
    fn the_caller_word_survives_an_unwind_through_the_guard() {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        let hostile = hostile_word(saved);
        write_mxcsr(hostile);

        let outcome = std::panic::catch_unwind(|| {
            let _pinned = CanonicalFpEnv::enter();
            assert_eq!(read_mxcsr(), fpenv::CANONICAL_MXCSR);
            panic!("a render entry that panics still restores the caller's environment");
        });
        assert!(outcome.is_err(), "the body must have panicked");
        assert_eq!(
            read_mxcsr(),
            hostile,
            "the guard must restore the caller's word while unwinding"
        );
    }

    #[test]
    fn denormal_arithmetic_is_correct_inside_the_guard_and_flushed_outside_it() {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        write_mxcsr(hostile_word(saved));

        // `2^-100 * 2^-30 = 2^-130` is exactly representable as an `f32` subnormal (the normal
        // range stops at `2^-126`), so flush-to-zero is observable on it and nothing else is.
        let small = std::hint::black_box(f32::from_bits(0x0D80_0000));
        let scale = std::hint::black_box(f32::from_bits(0x3080_0000));

        // Each product is black-boxed *inside* the region it belongs to. The guard's barrier stops
        // memory operations crossing it, which is what anchors a render; a product held entirely in
        // registers has no load to be anchored by, and in a release build the optimizer really will
        // schedule this multiply outside the guarded region without the `black_box` on its result.
        // That asymmetry is the point of this test, and it is why the assertion below is about two
        // *bit patterns* produced under two environments rather than about the guard's own code.
        let flushed =
            std::hint::black_box(std::hint::black_box(small) * std::hint::black_box(scale));
        let canonical = {
            let _pinned = CanonicalFpEnv::enter();
            std::hint::black_box(std::hint::black_box(small) * std::hint::black_box(scale))
        };

        // Compared as bits, never as floats: outside the guard DAZ is still set, and a `== 0.0`
        // there would read a subnormal operand as zero and pass vacuously.
        assert_eq!(
            flushed.to_bits(),
            0,
            "the test host must genuinely flush this product with FTZ set"
        );
        assert_eq!(
            canonical.to_bits(),
            0x0008_0000,
            "inside the guard the same product must be the exact IEEE subnormal `2^-130`"
        );
    }

    /// Attestation is about control state, not about what a previous operation left sticky.
    #[test]
    fn sticky_status_flags_do_not_fail_the_attestation_but_a_control_bit_does() {
        let saved = read_mxcsr();
        let _restore = Restore(saved);

        // Every sticky exception flag set, control state exactly canonical.
        let flagged = fpenv::CANONICAL_MXCSR | 0x003F;
        write_mxcsr(flagged);
        assert!(
            fpenv::in_canonical_fp_environment(),
            "sticky status flags must not be read as a non-canonical environment"
        );
        attest_fp_environment().expect("sticky flags must not fail the attestation");
        assert_eq!(
            read_mxcsr(),
            flagged,
            "attestation must leave even the sticky flags alone"
        );

        // One control bit, and the answer changes.
        write_mxcsr(fpenv::CANONICAL_MXCSR | MXCSR_FTZ);
        assert!(
            !fpenv::in_canonical_fp_environment(),
            "a set FTZ bit must be read as a non-canonical environment"
        );
        write_mxcsr(fpenv::CANONICAL_MXCSR | MXCSR_DAZ);
        assert!(
            !fpenv::in_canonical_fp_environment(),
            "a set DAZ bit must be read as a non-canonical environment"
        );
    }

    #[test]
    fn attestation_passes_from_a_hostile_caller_word() {
        let saved = read_mxcsr();
        let _restore = Restore(saved);
        let hostile = hostile_word(saved);
        write_mxcsr(hostile);
        attest_fp_environment().expect("a hostile word must still be pinnable");
        assert_eq!(
            read_fp_control_word(),
            hostile,
            "attestation must leave the thread exactly as it found it"
        );
    }
}

#[test]
fn the_target_declares_whether_it_pins() {
    assert_eq!(
        FP_ENV_CONTROLLED,
        cfg!(any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64"
        )),
        "FP_ENV_CONTROLLED must describe this target"
    );
    if !FP_ENV_CONTROLLED {
        assert_eq!(
            core::mem::size_of::<CanonicalFpEnv>(),
            0,
            "a target without a control word must pay nothing for the guard"
        );
    }
}
