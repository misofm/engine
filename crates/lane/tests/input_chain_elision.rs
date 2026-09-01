#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! The prepared-identity elision of `input_chain_block`: when a section may be skipped, and that
//! skipping it moves no bit.
//!
//! A builtin section whose prepared design is the exact identity -- `m0 = +1.0`,
//! `m1 = m2 = c1 = a2 = a3 = +0.0` -- over `+0.0` integrators is the map `v |-> v + 0.0`, so a run
//! of such sections is exactly one `add(+0.0)` at the run's position. `input_chain_block_elided`
//! is that rewrite and this file is its gate: for every one of the sixteen enabled/disabled section
//! patterns, at every width, over a `-0.0`-rich signal, the elided chain must produce the same
//! bits, the same retained state and the same report as `input_chain_block`.
//!
//! Two of the cases here are *divergence* evidence rather than identity evidence: they show what
//! the gate in `input_chain_plan` is buying, by forcing elision on a chain that does not qualify
//! and watching the bits move. Both are `-0.0` cases, and both are why the test is on bit
//! patterns and not on `==`.

use lane::kernels::SvfCoef;
use lane::kernels::builtins::{
    InputChainCoef, InputChainPlan, InputChainState, input_chain_block, input_chain_block_elided,
    input_chain_plan,
};
use lane::{Lane, Simd4, Simd8};

/// Frames per case. Long enough that a real section's recurrence is well past its transient.
const FRAMES: usize = 512;

/// Widest bank under test.
const MAX_WIDTH: usize = 8;

/// One section's design, per lane, as the builtins crate computes it.
#[derive(Clone, Copy, PartialEq)]
struct Design {
    c1: f32,
    a2: f32,
    a3: f32,
    m0: f32,
    m1: f32,
    m2: f32,
}

impl Design {
    /// The disabled section: `SvfSection::IDENTITY` in `builtins`.
    const IDENTITY: Self = Self {
        c1: 0.0,
        a2: 0.0,
        a3: 0.0,
        m0: 1.0,
        m1: 0.0,
        m2: 0.0,
    };

    /// A real Butterworth section, in `SvfSection::design`'s operation order.
    fn real(rate: f64, cutoff: f64, high_pass: bool) -> Self {
        let g = (core::f64::consts::PI * cutoff / rate).tan();
        let k = core::f64::consts::SQRT_2;
        let t1 = g * (g + k);
        let denominator = 1.0 + t1;
        let (m0, m1, m2) = if high_pass {
            (1.0, -(k as f32), -1.0)
        } else {
            (0.0, 0.0, 1.0)
        };
        Self {
            c1: (t1 / denominator) as f32,
            a2: (g / denominator) as f32,
            a3: (g * g / denominator) as f32,
            m0,
            m1,
            m2,
        }
    }
}

/// Builds one section's lane words from one design per lane.
fn coef<L: Lane>(lanes: &[Design; MAX_WIDTH]) -> SvfCoef<L> {
    let pick = |select: fn(&Design) -> f32| -> L {
        let words: Vec<f32> = lanes.iter().take(L::WIDTH).map(select).collect();
        L::load(&words)
    };
    SvfCoef {
        c1: pick(|design| design.c1),
        a2: pick(|design| design.a2),
        a3: pick(|design| design.a3),
        m0: pick(|design| design.m0),
        m1: pick(|design| design.m1),
        m2: pick(|design| design.m2),
    }
}

/// Builds a whole chain's coefficients from one design per channel, section and lane.
fn chain_coef<L: Lane>(sections: &[[[Design; MAX_WIDTH]; 2]; 2]) -> InputChainCoef<L> {
    let trim: Vec<f32> = (0..L::WIDTH)
        .map(|lane| 1.0 + lane as f32 * 0.125)
        .collect();
    InputChainCoef {
        trim: [L::load(&trim), L::load(&trim)],
        section: [
            [coef::<L>(&sections[0][0]), coef::<L>(&sections[0][1])],
            [coef::<L>(&sections[1][0]), coef::<L>(&sections[1][1])],
        ],
    }
}

/// The four section designs of one enabled/disabled pattern; bit `i` of `pattern` enables section
/// `i` of the flattened `[channel][section]` order, and bit `i` of `shapes` makes that section a
/// **high-pass** rather than a low-pass.
///
/// The shape axis is not decoration, and a gate that omits it is not a gate. The console's chain is
/// a high-pass in section 0 and a low-pass in section 1, and a real *low-pass* washes the sign of a
/// `-0.0` input on its own — `m0 = m1 = 0`, so the direct term that carries the sign is multiplied
/// away. A real *high-pass* does not: `m0 = 1`, so `-0.0` survives `m0 * v0` and reaches the output.
/// Tying each section's shape to its index therefore hides the one mixed case that can actually
/// move — an identity section whose `add(+0.0)` is misplaced past a following high-pass — which is
/// exactly the misplacement `input_chain_block_elided` exists to get right. Both axes are swept.
fn designs(pattern: usize, shapes: usize) -> [[[Design; MAX_WIDTH]; 2]; 2] {
    let mut out = [[[Design::IDENTITY; MAX_WIDTH]; 2]; 2];
    for index in 0..4 {
        if pattern & (1 << index) == 0 {
            continue;
        }
        let (channel, section) = (index / 2, index % 2);
        let high_pass = shapes & (1 << index) != 0;
        for (lane, design) in out[channel][section].iter_mut().enumerate() {
            // A different cutoff per lane, so a body that broadcast one lane's coefficients would
            // be caught here as well as by G2.
            let cutoff = if high_pass {
                30.0 + 5.0 * lane as f64
            } else {
                17_250.0 - 250.0 * lane as f64
            };
            *design = Design::real(48_000.0, cutoff, high_pass);
        }
    }
    out
}

/// A `-0.0`-rich signal: the sign of a zero is the whole subject of this gate, so zeros are
/// over-represented and half of them are negative.
fn signal(seed: u64) -> Vec<f32> {
    let mut state = seed | 1;
    (0..FRAMES * MAX_WIDTH)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            let draw = (state >> 33) as u32;
            match draw % 5 {
                0 => -0.0,
                1 => 0.0,
                2 => -((draw >> 8) as f32 / 8.388_608e6),
                3 => (draw >> 8) as f32 / 8.388_608e6,
                _ => ((draw >> 8) as f32 / 8.388_608e6) - 0.5,
            }
        })
        .collect()
}

/// Runs both kernels over the same case and returns `(output bits, state bits, report bits)` for
/// each, so a divergence anywhere in the trio is visible.
type Run = (Vec<u32>, Vec<u32>, Vec<u32>);

fn run<L: Lane>(
    c: &InputChainCoef<L>,
    seed: &InputChainState<L>,
    plan: &InputChainPlan,
    samples: &[f32],
) -> (Run, Run) {
    let frames = samples.len() / L::WIDTH / 2;
    let split = frames * L::WIDTH;
    let mut one = (
        samples[..split].to_vec(),
        samples[split..split * 2].to_vec(),
        *seed,
    );
    let mut two = (one.0.clone(), one.1.clone(), *seed);

    let reference = input_chain_block::<L>(&mut one.0, &mut one.1, frames, c, &mut one.2);
    let elided = input_chain_block_elided::<L>(&mut two.0, &mut two.1, frames, c, &mut two.2, plan);

    let pack = |io: &(Vec<f32>, Vec<f32>, InputChainState<L>),
                report: &lane::kernels::builtins::InputChainReport<L>|
     -> Run {
        let output: Vec<u32> =
            io.0.iter()
                .chain(io.1.iter())
                .map(|sample| sample.to_bits())
                .collect();
        let mut state = Vec::new();
        for channel in &io.2.section {
            for section in channel {
                state.extend(bits::<L>(section.ic1));
                state.extend(bits::<L>(section.ic2));
            }
        }
        let mut counters = Vec::new();
        for channel in 0..2 {
            counters.extend(bits::<L>(report.sanitized[channel]));
            counters.extend(bits::<L>(L::select(
                report.nonfinite[channel],
                L::splat(1.0),
                L::zero(),
            )));
        }
        (output, state, counters)
    };

    (pack(&one, &reference), pack(&two, &elided))
}

/// One `u32` per lane of a lane value.
fn bits<L: Lane>(value: L) -> Vec<u32> {
    let mut words = [0_u32; MAX_WIDTH];
    value.store_bits(&mut words[..L::WIDTH]);
    words[..L::WIDTH].to_vec()
}

/// The whole plan, decided from the words, for every section pattern and shape at one width.
fn check_pattern<L: Lane>(width: &str, pattern: usize, shapes: usize) {
    let c = chain_coef::<L>(&designs(pattern, shapes));
    let seed = InputChainState::<L>::default();
    let plan = input_chain_plan::<L>(&c, &seed);

    // The plan must be exactly the complement of the pattern: a section is elided when and only
    // when its design is the identity. The shape of a real section cannot move this.
    for index in 0..4 {
        let (channel, section) = (index / 2, index % 2);
        assert_eq!(
            plan.elided[channel][section],
            pattern & (1 << index) == 0,
            "width={width}, pattern={pattern:04b}, shapes={shapes:04b}, channel={channel}, \
             section={section}"
        );
    }

    for seed_index in 0..2 {
        let samples = signal(0x00E1_1DE0 + seed_index);
        let (reference, elided) = run::<L>(&c, &seed, &plan, &samples);
        assert_eq!(
            reference.0, elided.0,
            "width={width}, pattern={pattern:04b}, shapes={shapes:04b}, seed={seed_index}, output"
        );
        assert_eq!(
            reference.1, elided.1,
            "width={width}, pattern={pattern:04b}, shapes={shapes:04b}, seed={seed_index}, \
             retained state"
        );
        assert_eq!(
            reference.2, elided.2,
            "width={width}, pattern={pattern:04b}, shapes={shapes:04b}, seed={seed_index}, report"
        );
    }
}

/// Red test 1 and 2: every mix of identity and real sections, at every width, is bit-identical to
/// the unelided chain -- output, retained state and report alike.
///
/// `pattern` runs over all sixteen combinations, so it includes the all-identity shape (the one
/// the `dispatch_only` benchmark row measures), the all-real shape (`builtins_only`), and every
/// mixed shape in which the `add(+0.0)`'s *position* in the chain is what is under test: an
/// identity high-pass before a real low-pass must hand the low-pass `v + 0.0`, not `v`.
#[test]
fn elision_is_bit_identical_at_every_width_and_section_pattern() {
    for pattern in 0..16 {
        for shapes in 0..16 {
            check_pattern::<f32>("1", pattern, shapes);
            check_pattern::<Simd4>("4", pattern, shapes);
            check_pattern::<Simd8>("8", pattern, shapes);
        }
    }
}

/// Red test 1, the SIMD arm: one non-identity lane in a section forbids eliding that section for
/// the whole bank, because the kernel body has no per-lane branch.
#[test]
fn a_single_non_identity_lane_blocks_the_whole_section() {
    fn check<L: Lane>(width: &str) {
        for poisoned in 0..L::WIDTH {
            let mut sections = [[[Design::IDENTITY; MAX_WIDTH]; 2]; 2];
            sections[0][0][poisoned] = Design::real(48_000.0, 40.0, true);
            let c = chain_coef::<L>(&sections);
            let seed = InputChainState::<L>::default();
            let plan = input_chain_plan::<L>(&c, &seed);
            assert!(
                !plan.elided[0][0],
                "width={width}, poisoned lane={poisoned}: one real lane must block the section"
            );
            assert!(
                plan.elided[0][1] && plan.elided[1][0] && plan.elided[1][1],
                "width={width}: the other three sections are untouched"
            );
            let samples = signal(0x0A11_1DE5 + poisoned as u64);
            let (reference, elided) = run::<L>(&c, &seed, &plan, &samples);
            assert_eq!(reference, elided, "width={width}, poisoned lane={poisoned}");
        }
    }
    check::<f32>("1");
    check::<Simd4>("4");
    check::<Simd8>("8");
}

/// Red test 3, the state arm: a chain whose coefficients are the identity but whose integrators are
/// not `+0.0` must not be elided, and forcing the elision anyway moves bits.
///
/// The forced arm is the evidence that the state words belong in the gate at all. With both
/// sections seeded at `-1.0` the chain emits `-0.0` for a `-0.0` input, where the elided form emits
/// `+0.0`: the identity mix's `m1 * v1` term is `+0.0 * (negative)`, which is `-0.0`, and `-0.0`
/// added to the `-0.0` the direct term carries stays `-0.0`.
#[test]
fn identity_coefficients_over_non_zero_state_are_not_elidable() {
    fn check<L: Lane>(width: &str) {
        let c = chain_coef::<L>(&designs(0, 0));
        let mut seed = InputChainState::<L>::default();
        for channel in &mut seed.section {
            for section in channel.iter_mut() {
                section.ic1 = L::splat(-1.0);
                section.ic2 = L::splat(-1.0);
            }
        }
        let plan = input_chain_plan::<L>(&c, &seed);
        assert_eq!(
            plan,
            InputChainPlan::NONE,
            "width={width}: non-zero retained state forbids elision"
        );
        let samples = signal(0x5EED_0F17);
        let (reference, elided) = run::<L>(&c, &seed, &plan, &samples);
        assert_eq!(reference, elided, "width={width}, decided plan");

        // The forced arm: what the plan is preventing.
        let forced = InputChainPlan {
            elided: [[true; 2]; 2],
        };
        let (reference, wrong) = run::<L>(&c, &seed, &forced, &samples);
        assert_ne!(
            reference.0, wrong.0,
            "width={width}: eliding a chain with seeded state must be observable, or this gate is \
             vacuous"
        );
    }
    check::<f32>("1");
    check::<Simd4>("4");
    check::<Simd8>("8");
}

/// Red test 3, the coefficient arm: the gate is on **bit patterns**, and `==` is not enough.
///
/// `m1 = m2 = -0.0` is `== 0.0` in every lane, so a gate written with float equality would elide
/// this chain. It must not: `-0.0 * v1` is `-0.0` for a non-negative `v1`, and the two `-0.0` mix
/// terms carry the sign of a `-0.0` input all the way to the output, where the elided form washes
/// it to `+0.0`.
#[test]
fn negative_zero_mix_words_are_not_the_identity() {
    fn check<L: Lane>(width: &str) {
        let mut sections = [[[Design::IDENTITY; MAX_WIDTH]; 2]; 2];
        for channel in &mut sections {
            for section in channel.iter_mut() {
                for design in section.iter_mut() {
                    design.m1 = -0.0;
                    design.m2 = -0.0;
                }
            }
        }
        let c = chain_coef::<L>(&sections);
        let seed = InputChainState::<L>::default();
        let plan = input_chain_plan::<L>(&c, &seed);
        assert_eq!(
            plan,
            InputChainPlan::NONE,
            "width={width}: `-0.0` mix words are `== 0.0` but are not the identity"
        );
        let samples = signal(0x0BAD_2E80);
        let (reference, elided) = run::<L>(&c, &seed, &plan, &samples);
        assert_eq!(reference, elided, "width={width}, decided plan");

        let forced = InputChainPlan {
            elided: [[true; 2]; 2],
        };
        let (reference, wrong) = run::<L>(&c, &seed, &forced, &samples);
        assert_ne!(
            reference.0, wrong.0,
            "width={width}: eliding `-0.0` mix words must be observable, or the bitwise test is \
             gold-plating"
        );
    }
    check::<f32>("1");
    check::<Simd4>("4");
    check::<Simd8>("8");
}

/// The appendix's correction, made testable: a `-0.0` **state** word is genuinely inert, and the
/// bitwise state check is therefore conservative rather than load-bearing for that one pattern.
///
/// The original finding attributed the `-0.0` divergence to the state words. It belongs to the mix
/// words (`negative_zero_mix_words_are_not_the_identity`). With exact identity coefficients and
/// both integrators in `{+0.0, -0.0}` the section really is `v |-> v + 0.0`: `ic1 = -0.0` forces
/// `d1 = fma(-0.0, -0.0, +0.0 * v3) = (+0.0) + (+/-0.0) = +0.0`, so `v1 = (-0.0) + (+0.0) = +0.0`,
/// and the only path to `v2 = -0.0` needs `ic1 = -0.0`, which has just been shown to wash `v1`.
///
/// So for every one of the 256 sign assignments to the eight integrator words of an
/// identity-coefficient chain: the plan refuses to elide exactly the sections that carry a `-0.0`
/// -- the gate is on bit patterns, and `-0.0` is not the identity pattern -- and forcing the
/// elision on all four anyway moves **no output sample bit**. (The retained state words DO differ
/// under forced elision -- the unelided kernel washes a seeded `-0.0` integrator to `+0.0` on the
/// first frame while the elided kernel never writes it -- but forced elision is unreachable
/// through the gate, so output inertness is the whole reachable claim.) The second half is what
/// the appendix asserts, and it is the half a reader is entitled to see run.
#[test]
fn negative_zero_state_words_are_inert_but_still_fail_the_bitwise_gate() {
    fn check<L: Lane>(width: &str) {
        let c = chain_coef::<L>(&designs(0, 0));
        for signs in 0..256_usize {
            let mut seed = InputChainState::<L>::default();
            let mut negative = [[false; 2]; 2];
            for bit in 0..8 {
                if signs & (1 << bit) == 0 {
                    continue;
                }
                let (channel, section, second) = (bit / 4, (bit / 2) % 2, bit % 2 == 1);
                negative[channel][section] = true;
                let state = &mut seed.section[channel][section];
                if second {
                    state.ic2 = L::splat(-0.0);
                } else {
                    state.ic1 = L::splat(-0.0);
                }
            }

            let plan = input_chain_plan::<L>(&c, &seed);
            for (channel, (elided, negative)) in plan.elided.iter().zip(negative.iter()).enumerate()
            {
                for (section, (elided, negative)) in elided.iter().zip(negative.iter()).enumerate()
                {
                    assert_eq!(
                        *elided, !*negative,
                        "width={width}, signs={signs:08b}, channel={channel}, section={section}: \
                         `-0.0` is not the identity bit pattern"
                    );
                }
            }

            // The inertness itself: force the elision the gate refused and watch nothing move --
            // not the samples, not the retained state, not the report.
            let forced = InputChainPlan {
                elided: [[true; 2]; 2],
            };
            let samples = signal(0x0E10_0000 + signs as u64);
            let (reference, wrong) = run::<L>(&c, &seed, &forced, &samples);
            assert_eq!(
                reference.0, wrong.0,
                "width={width}, signs={signs:08b}: a `-0.0` integrator must be inert under \
                 identity coefficients"
            );
        }
    }
    check::<f32>("1");
    check::<Simd4>("4");
    check::<Simd8>("8");
}
