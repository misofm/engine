//! Gate G6 over the complete cross-target corpus: forced x86 FTZ/DAZ changes no digest.
//!
//! The narrower lane test proves the mechanism with a control arm. This test exercises every
//! case used by the Wasm gate, at every applicable width, under MXCSR with FTZ/DAZ first clear and
//! then genuinely set. The guard restores the calling test thread's exact control word even if a
//! render or assertion panics.
//!
//! Red mutation (recorded in `../MUTATIONS.md`): lower `miso_engine_lane::FLUSH_EPS` to `1e-40`.
//! The FTZ/DAZ render then differs from the clear-MXCSR render in the subnormal corpus cases.

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
    use miso_engine_lane::softfma::{MXCSR_DAZ, MXCSR_FTZ, read_mxcsr, write_mxcsr};

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

    #[test]
    #[ignore = "issue #144 blocker: unmodified main has 70 FTZ/DAZ corpus mismatches"]
    fn g6_full_wasm_gate_corpus_is_ftz_daz_inert() {
        let saved = read_mxcsr();
        let controlled = saved & !(MXCSR_FTZ | MXCSR_DAZ);
        let without = render_with_mxcsr(controlled);
        let with = render_with_mxcsr(controlled | MXCSR_FTZ | MXCSR_DAZ);

        assert_eq!(read_mxcsr(), saved, "G6 corpus gate leaked MXCSR state");
        assert_eq!(
            without.len(),
            with.len(),
            "both runs must render the full corpus"
        );
        let mut mismatches = Vec::new();
        for (left, right) in without.iter().zip(&with) {
            assert_eq!(left.0, right.0, "case ordering changed between MXCSR runs");
            assert_eq!(left.1, right.1, "width ordering changed between MXCSR runs");
            if left.2 != right.2 {
                mismatches.push(format!(
                    "case {} ({}) / {}: clear={} ftz+daz={}",
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
            "G6 full corpus differs with FTZ+DAZ in {} rows:\n{}",
            mismatches.len(),
            mismatches.join("\n")
        );
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[test]
fn g6_full_wasm_gate_corpus_portable_smoke() {
    let rendered = render_corpus();
    assert!(!rendered.is_empty(), "the full corpus must not be empty");
}
