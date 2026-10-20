#![allow(clippy::disallowed_methods)]
// D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! E2: the `f32` two-stage crossover against the independent `f64` oracle, and its flatness.
//!
//! The `f64` mapping is pinned separately, before any production line moved, in
//! `tests/lr4_two_section_mapping_f64.rs`. This gate is the `f32` production form: the same
//! `lr4_step` the render path runs, at `WIDTH = 1`.

use lane::Lane;
use multiband_compressor::{Lr4State, lr4_coefficients, lr4_step};

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// The design guards its domain and its own half-power self-check.
#[test]
fn the_design_guards_its_domain() {
    assert!(lr4_coefficients::<f32>(0, 1_000.0).is_none());
    for crossover in [79.0f32, 8_001.0, f32::NAN, f32::INFINITY, -0.0] {
        assert!(
            lr4_coefficients::<f32>(48_000, crossover).is_none(),
            "{crossover}"
        );
    }
    for rate in RATES {
        for crossover in [80.0f32, 1_000.0, 8_000.0] {
            let coefficients = lr4_coefficients::<f32>(rate, crossover).expect("design");
            // `nk2` is exactly `-2 * sqrt(2)`, rounded once, on every design.
            assert_eq!(
                coefficients.nk2.to_bits(),
                (-2.0 * core::f32::consts::SQRT_2).to_bits()
            );
            assert!(coefficients.a2 > 0.0 && coefficients.a3 > 0.0);
        }
    }
}

/// A subnormal-seeded state reaches exactly zero, and stays there, at every width (D7).
#[test]
fn the_recursive_words_are_flushed() {
    fn check<L: Lane>() {
        let coefficients = lr4_coefficients::<L>(48_000, 1_000.0).expect("design");
        let mut state = Lr4State::<L> {
            a: lane::kernels::SvfState {
                ic1: L::splat(1.0e-40),
                ic2: L::splat(-1.0e-40),
            },
            b: lane::kernels::SvfState {
                ic1: L::splat(1.0e-40),
                ic2: L::splat(-1.0e-40),
            },
        };
        let mut run = L::zero();
        for _ in 0..4 {
            let rest = lane::silence_step(L::zero(), &mut run, L::splat(4_096.0));
            let _ = lr4_step(L::zero(), rest, &coefficients, &mut state);
        }
        let mut words = [0u32; 8];
        for value in [state.a.ic1, state.a.ic2, state.b.ic1, state.b.ic2] {
            value.store_bits(&mut words[..L::WIDTH]);
            for word in &words[..L::WIDTH] {
                assert_eq!(*word, 0, "a flushed state word must be exactly +0.0");
            }
        }
    }
    lane::each_lane!(|L| check::<L>());
}
