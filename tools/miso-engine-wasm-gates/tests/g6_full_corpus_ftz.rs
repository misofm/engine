//! Gate G6 over the complete cross-target corpus: a caller's FTZ+DAZ never reaches a render.
//!
//! # What this gate said before, and what it says now
//!
//! Issue #144 landed this file as an `#[ignore]`d reproducer. It rendered the whole gate corpus
//! twice -- once with MXCSR's FTZ and DAZ clear, once with them genuinely set -- and found 69-70 of
//! the 331 comparisons diverging: transient intra-block denormals in the recursive SVF, the
//! feed-forward lane, scalar math and the effect/builtin chains, none of which is a recursive state
//! word the master-plan D7 flush law can reach. The reproducer asserted FTZ inertness, which was
//! false, so it could only be carried ignored.
//!
//! Issue #146 fixed the defect at the boundary instead of in the kernels: every native render entry
//! saves the caller's floating-point control word, installs the canonical one, renders, and
//! restores the caller's exact word ([`miso_engine_lane::fpenv`]). This file is now the standing
//! gate for the *opposite*, true claim -- with the caller's thread in FTZ+DAZ, a render that goes
//! through an entry lands byte-for-byte on the frozen pins -- with the old reproducer retained as
//! the control arm that keeps the new claim from passing vacuously.
//!
//! The three arms, all over the full corpus at every applicable width:
//!
//! | arm | caller word | entry guard | must |
//! |---|---|---|---|
//! | canonical | FTZ/DAZ clear | none | match the pins |
//! | guarded | FTZ+DAZ set | entered | match the pins |
//! | control | FTZ+DAZ set | none | **differ** from the pins |
//!
//! Red mutation (recorded in `../MUTATIONS.md`): delete the `CanonicalFpEnv::enter()` line from one
//! render entry -- modelled here by dropping it from `guarded_report` -- and the guarded arm
//! collapses onto the control arm's ~70 diverging rows.

use miso_engine_wasm_gate_corpus as corpus;

fn render_corpus() -> Vec<(usize, usize, [u8; 32])> {
    let mut rendered = Vec::new();
    for case in 0..corpus::CASE_COUNT {
        let widths = if corpus::is_width_dependent(case) {
            0..corpus::WIDTHS
        } else {
            0..1
        };
        for width in widths {
            rendered.push((case, width, corpus::digest_case(case, width)));
        }
    }
    rendered
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86 {
    use super::*;
    use miso_engine_lane::fpenv::CanonicalFpEnv;
    use miso_engine_lane::softfma::{MXCSR_DAZ, MXCSR_FTZ, read_mxcsr, write_mxcsr};
    use miso_engine_wasm_gates::{Report, native_report};

    /// The smallest divergence the control arm may show and still be a real control.
    ///
    /// The measured figure on the delivery host is 70 of 331 comparisons. This floor exists so that
    /// a future corpus change which happened to leave only one denormal-sensitive row cannot let
    /// the guarded arm pass while proving nothing; it is deliberately far below the measured
    /// number, because the control's job is to be non-empty, not to pin a count.
    const CONTROL_ARM_FLOOR: usize = 16;

    struct MxcsrGuard {
        saved: u32,
    }

    impl MxcsrGuard {
        fn set(value: u32) -> Self {
            let saved = read_mxcsr();
            write_mxcsr(value);
            assert_eq!(
                read_mxcsr(),
                value,
                "G6 corpus gate did not install the requested MXCSR word"
            );
            Self { saved }
        }
    }

    impl Drop for MxcsrGuard {
        fn drop(&mut self) {
            write_mxcsr(self.saved);
        }
    }

    fn render_with_mxcsr(value: u32) -> Vec<(usize, usize, [u8; 32])> {
        let _guard = MxcsrGuard::set(value);
        render_corpus()
    }

    /// The corpus rendered against its pins with `value` as the caller's control word and **no**
    /// render entry: this is what a host got before issue #146.
    fn unguarded_report(value: u32) -> Report {
        let _caller = MxcsrGuard::set(value);
        native_report()
    }

    /// The corpus rendered against its pins with `value` as the caller's control word, through the
    /// same guard every native render entry installs.
    fn guarded_report(value: u32) -> Report {
        let _caller = MxcsrGuard::set(value);
        let _entry = CanonicalFpEnv::enter();
        native_report()
    }

    fn describe(report: &Report) -> String {
        report
            .mismatches
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// E1: a caller that has set FTZ+DAZ still gets the frozen pins out of a guarded render.
    #[test]
    fn g6_full_wasm_gate_corpus_is_canonical_under_caller_ftz() {
        let saved = read_mxcsr();
        let clear = saved & !(MXCSR_FTZ | MXCSR_DAZ);
        let hostile = clear | MXCSR_FTZ | MXCSR_DAZ;

        let canonical = unguarded_report(clear);
        assert!(
            canonical.mismatches.is_empty(),
            "the pins do not describe this host with FTZ clear, so G6 has no baseline:\n{}",
            describe(&canonical)
        );

        let control = unguarded_report(hostile);
        assert!(
            control.mismatches.len() >= CONTROL_ARM_FLOOR,
            "G6 is vacuous: the corpus rendered under FTZ+DAZ without a render entry diverged in \
             only {} of {} comparisons, so the guarded arm proves nothing",
            control.mismatches.len(),
            control.comparisons
        );

        let guarded = guarded_report(hostile);
        assert!(
            guarded.mismatches.is_empty(),
            "issue #146: a render entry's canonical environment did not normalise {} of {} \
             comparisons under a caller's FTZ+DAZ:\n{}",
            guarded.mismatches.len(),
            guarded.comparisons,
            describe(&guarded)
        );
        assert_eq!(
            guarded.comparisons, canonical.comparisons,
            "both arms must compare the whole corpus"
        );

        assert_eq!(read_mxcsr(), saved, "G6 corpus gate leaked MXCSR state");
    }

    /// E3 class-A identity: a caller who never set FTZ renders the same bytes with the guard as
    /// without it, so pinning the environment is a no-op transition by value for every existing
    /// host.
    #[test]
    fn g6_the_guard_is_an_identity_for_a_caller_who_never_set_ftz() {
        let saved = read_mxcsr();
        let clear = saved & !(MXCSR_FTZ | MXCSR_DAZ);

        let without = render_with_mxcsr(clear);
        let with = {
            let _caller = MxcsrGuard::set(clear);
            let _entry = CanonicalFpEnv::enter();
            render_corpus()
        };

        assert_eq!(
            without.len(),
            with.len(),
            "both runs must render the full corpus"
        );
        let mut mismatches = Vec::new();
        for (left, right) in without.iter().zip(&with) {
            assert_eq!(left.0, right.0, "case ordering changed between runs");
            assert_eq!(left.1, right.1, "width ordering changed between runs");
            if left.2 != right.2 {
                mismatches.push(format!(
                    "case {} ({}) / {}: unguarded={} guarded={}",
                    left.0,
                    corpus::case_name(left.0),
                    corpus::width_name(left.1),
                    miso_engine_wasm_gates::hex(&left.2),
                    miso_engine_wasm_gates::hex(&right.2)
                ));
            }
        }
        assert!(
            mismatches.is_empty(),
            "the guard changed {} rows for a caller who had not set FTZ:\n{}",
            mismatches.len(),
            mismatches.join("\n")
        );
        assert_eq!(read_mxcsr(), saved, "G6 corpus gate leaked MXCSR state");
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[test]
fn g6_full_wasm_gate_corpus_portable_smoke() {
    let rendered = render_corpus();
    assert!(!rendered.is_empty(), "the full corpus must not be empty");
}
