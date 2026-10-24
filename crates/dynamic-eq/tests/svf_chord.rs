//! A retarget of a band keeps its nine filter words on one chord (#17 S2, the #1372a and #1928
//! finding).
//!
//! A band's bell words `c1, a2, a3` and its detector words `c1, a2, a3` are safe on a ramp only as
//! triples: a straight line between two designs is contractive at every point
//! (`effect_runtime::svf::transition_norm` is convex in the words), but a triple whose words sit at
//! different fractions of different ramps need not be. A one-ulp frequency nudge can keep some
//! words' design bits and change the others', so a retarget that skips a word with unchanged
//! target bits leaves that word on its old schedule while the others restart.

mod support;

use effect_contract::{LinkMode, PreparedEffect, StatePayloadOutput};
use effect_runtime::svf::{RAMP_PATH_NORM_TOLERANCE, transition_norm};
use support::{FREQUENCY, GAIN, Q, RATE, index, only, prepare, process, set_band, spans_both};

const BAND_Q: f32 = 2.0;
const BAND_GAIN_DB: f32 = 6.0;
/// A channel section holds 30 signal words and the 48 committed values, then the ramps as
/// `current, target, step, remaining`, 17 per band; band 0's design words are ramps 6, 7, 8 (the
/// bell's `c1, a2, a3`), 9, 10 (`m1`, `k A^2`), 13, 14, 15 (the detector's `c1, a2, a3`) and 16
/// (its `1/Q`).
const RAMP_BASE: usize = 30 + 48;
const DESIGN_RAMPS: [usize; 9] = [6, 7, 8, 9, 10, 13, 14, 15, 16];

/// Band 0's nine design words at `frequency`, in [`DESIGN_RAMPS`] order, as the effect rounds them.
fn design(frequency: f32) -> [f32; 9] {
    let bell = parametric_eq::design_svf(
        parametric_eq::EqBandKind::Bell,
        frequency,
        BAND_GAIN_DB,
        BAND_Q,
        1.0,
        engine::SampleRateHz(RATE),
    )
    .expect("an in-domain bell");
    let detector = parametric_eq::design_svf_words_f64(
        parametric_eq::EqBandKind::HighPass,
        f64::from(frequency),
        0.0,
        f64::from(BAND_Q),
        1.0,
        f64::from(RATE),
    );
    let amplitude = math::pow(10.0, f64::from(BAND_GAIN_DB) / 40.0);
    [
        bell.c1,
        bell.a2,
        bell.a3,
        bell.m1,
        (amplitude / f64::from(BAND_Q)) as f32,
        detector[0] as f32,
        detector[1] as f32,
        detector[2] as f32,
        (-detector[4]) as f32,
    ]
}

/// The left channel's current band-0 design words.
fn words(effect: &PreparedEffect) -> [f32; 9] {
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
    DESIGN_RAMPS.map(|ramp| {
        let at = (RAMP_BASE + 4 * ramp) * 4;
        f32::from_le_bytes(sections.1[at..at + 4].try_into().expect("a word"))
    })
}

/// A frequency near 1 kHz whose one-ulp successor keeps at least one SVF word's design bits and
/// moves at least one other's.
fn nudge_pair() -> (f32, f32) {
    let mut frequency = 1_000.0_f32;
    loop {
        let next = f32::from_bits(frequency.to_bits() + 1);
        let (from, to) = (design(frequency), design(next));
        let svf = [0, 1, 2, 5, 6, 7];
        let kept = svf.iter().any(|&w| from[w].to_bits() == to[w].to_bits());
        let moved = svf.iter().any(|&w| from[w].to_bits() != to[w].to_bits());
        if kept && moved {
            return (frequency, next);
        }
        frequency = next;
    }
}

/// Red on a retarget that restarts only the words whose design bits moved: from a 6 kHz design, a
/// move to about 1 kHz, then a one-ulp nudge halfway through its 64-sample ramp. After the nudge
/// every frame's nine words must lie on the chord from the nudge point to the new design (each at
/// one common fraction of its way), and both SVF triples stay contractive.
#[test]
fn a_mid_ramp_nudge_restarts_every_design_word_of_the_band_together() {
    let (target, nudged) = nudge_pair();
    let mut values = only(0);
    set_band(&mut values, 0, FREQUENCY, 6_000.0);
    set_band(&mut values, 0, Q, BAND_Q);
    set_band(&mut values, 0, GAIN, BAND_GAIN_DB);
    let mut effect = prepare(&values, LinkMode::DualMono);
    let input = support::noise(7, 256, 0.25);
    let frequency = index(0, FREQUENCY) as u32;
    let frame = |effect: &mut PreparedEffect, n: usize, spans: &[_]| {
        let (mut left, mut right) = ([input[n]], [input[n + 128]]);
        process(effect, &mut left, &mut right, n as u64, spans);
    };
    frame(&mut effect, 0, &spans_both(frequency, target, 0));
    for n in 1..32 {
        frame(&mut effect, n, &[]);
    }
    let start = words(&effect);
    let end = design(nudged);
    frame(&mut effect, 32, &spans_both(frequency, nudged, 32));
    for n in 33..128 {
        let now = words(&effect);
        for triple in [0, 5] {
            let norm = transition_norm(now[triple], now[triple + 1], now[triple + 2]);
            assert!(
                norm <= RAMP_PATH_NORM_TOLERANCE,
                "frame {n}: words {now:?} have norm {norm}"
            );
        }
        let fractions: Vec<f64> = (0..9)
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
