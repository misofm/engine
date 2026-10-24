//! A retarget of the band filter keeps its three SVF words on one chord (#15 S2, the #1372a
//! finding).
//!
//! The words `c1, a2, a3` are safe on a ramp only as a triple: a straight line between two designs
//! is contractive at every point (`effect_runtime::svf::transition_norm` is convex in the words),
//! but a triple whose words sit at different fractions of different ramps need not be. A
//! one-ulp frequency nudge can keep one word's design bits and change the others', so a retarget
//! that skips a word with unchanged target bits leaves that word on its old schedule while the
//! others restart.

mod support;

use effect_contract::{LinkMode, PreparedEffect, StatePayloadOutput};
use effect_runtime::svf::{RAMP_PATH_NORM_TOLERANCE, transition_norm};
use support::{RATE, defaults, prepare, process, set, spans_both};

const Q: f32 = 8.0;
/// Committed values and the first seven channel words come first in a channel section, then the
/// twelve ramps as `current, target, step, remaining`; `c1, a2, a3` are ramps 5, 6 and 7.
const C1_CURRENT: usize = 7 + 10 + 5 * 4;

fn design(frequency: f32) -> [f32; 3] {
    let words = parametric_eq::design_svf_words_f64(
        parametric_eq::EqBandKind::HighPass,
        f64::from(frequency),
        0.0,
        f64::from(Q),
        1.0,
        f64::from(RATE),
    );
    [words[0] as f32, words[1] as f32, words[2] as f32]
}

/// The left channel's current `c1, a2, a3`.
fn words(effect: &PreparedEffect) -> [f32; 3] {
    let sizes = effect.metadata.state_sizes;
    let mut sections = (
        vec![0; sizes.common_bytes as usize],
        vec![0; sizes.left_bytes as usize],
        vec![0; sizes.right_bytes as usize],
    );
    effect
        .processor
        .snapshot_state_payload(
            StatePayloadOutput::new(&mut sections.0, &mut sections.1, &mut sections.2, sizes)
                .expect("sizes"),
        )
        .expect("snapshot");
    core::array::from_fn(|word| {
        let at = (C1_CURRENT + 4 * word) * 4;
        f32::from_le_bytes(sections.1[at..at + 4].try_into().expect("a word"))
    })
}

/// A frequency near 1 kHz whose one-ulp successor keeps `a2`'s design bits and moves `c1` or
/// `a3`'s.
fn nudge_pair() -> (f32, f32) {
    let mut frequency = 1_000.0_f32;
    loop {
        let next = f32::from_bits(frequency.to_bits() + 1);
        let (from, to) = (design(frequency), design(next));
        if from[1].to_bits() == to[1].to_bits()
            && (from[0].to_bits() != to[0].to_bits() || from[2].to_bits() != to[2].to_bits())
        {
            return (frequency, next);
        }
        frequency = next;
    }
}

/// Red on a retarget that restarts only the words whose design bits moved: from a 12 kHz design,
/// a move to about 1 kHz, then a one-ulp nudge halfway through its 64-sample ramp. After the nudge
/// every frame's triple must lie on the chord from the nudge point to the new design (each word at
/// one common fraction of its way), and stay contractive.
#[test]
fn a_mid_ramp_nudge_restarts_the_three_svf_words_together() {
    let (target, nudged) = nudge_pair();
    let mut values = defaults();
    set(&mut values, 0, 12_000.0);
    set(&mut values, 1, Q);
    let mut effect = prepare(&values, LinkMode::DualMono);
    let input = support::noise(7, 256, 0.25);
    let frame = |effect: &mut PreparedEffect, n: usize, spans: &[_]| {
        let (mut left, mut right) = ([input[n]], [input[n + 128]]);
        process(effect, &mut left, &mut right, n as u64, spans);
    };
    frame(&mut effect, 0, &spans_both(0, target, 0));
    for n in 1..32 {
        frame(&mut effect, n, &[]);
    }
    let start = words(&effect);
    let end = design(nudged);
    frame(&mut effect, 32, &spans_both(0, nudged, 32));
    for n in 33..128 {
        let now = words(&effect);
        let norm = transition_norm(now[0], now[1], now[2]);
        assert!(
            norm <= RAMP_PATH_NORM_TOLERANCE,
            "frame {n}: words {now:?} have norm {norm}"
        );
        let fractions: Vec<f64> = (0..3)
            .filter(|&word| start[word] != end[word])
            .map(|word| f64::from(now[word] - start[word]) / f64::from(end[word] - start[word]))
            .collect();
        let (low, high) = fractions
            .iter()
            .fold((f64::MAX, f64::MIN), |(low, high), &f| {
                (low.min(f), high.max(f))
            });
        assert!(
            high - low <= 1.0e-3,
            "frame {n}: words {now:?} are off the chord from {start:?} to {end:?} \
             (fractions {fractions:?})"
        );
        frame(&mut effect, n, &[]);
    }
    assert_eq!(words(&effect).map(f32::to_bits), end.map(f32::to_bits));
}
