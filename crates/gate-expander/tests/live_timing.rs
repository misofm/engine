//! #1336: the attack, hold and release are live values.
//!
//! The coefficient ramp (gate 1) and the hold rule (gate 2) are read from the effect's own state
//! payload after each one-frame block: per channel, word 1 is the open flag, word 2 the hold
//! countdown and words 22 to 29 the attack and release coefficient ramps
//! `(current, target, step, remaining)`.

mod support;

use effect_contract::{ParameterChannel, PreparedAutomationSpan, PreparedEffect};
use effect_runtime::envelope::attack_release_coefficient;
use effect_runtime::ramp::LinearRamp;
use support::{initial_values, prepare, render_scalar_sidechain, request, set_parameter, snapshot};

const RATE: u32 = 48_000;
const ATTACK_WORD: usize = 22;
const RELEASE_WORD: usize = 26;

fn word(bytes: &[u8], index: usize) -> f32 {
    f32::from_le_bytes(
        bytes[index * 4..index * 4 + 4]
            .try_into()
            .expect("four bytes"),
    )
}

/// One span per channel for parameter `index` (descriptor position); [`frames`] stamps the sample.
fn spans(index: u32, value: f32) -> [PreparedAutomationSpan; 2] {
    [ParameterChannel::Left, ParameterChannel::Right].map(|channel| PreparedAutomationSpan {
        channel,
        parameter_index: index,
        sample: 0,
        value,
    })
}

/// Renders `frames` frames of a constant `level` on both channels, one frame per block, with
/// `automation` in the first block, and returns the left payload section after each frame.
fn frames(
    effect: &mut PreparedEffect,
    at: &mut u64,
    frames: usize,
    level: f32,
    automation: &[PreparedAutomationSpan],
) -> Vec<Vec<u8>> {
    let stamped: Vec<_> = automation
        .iter()
        .map(|span| PreparedAutomationSpan {
            sample: *at,
            ..*span
        })
        .collect();
    (0..frames)
        .map(|frame| {
            let (mut left, mut right) = ([level], [level]);
            let spans = if frame == 0 { stamped.as_slice() } else { &[] };
            let report =
                render_scalar_sidechain(effect, &mut left, &mut right, None, 1, spans, *at);
            assert_eq!(report.invalid_spans, 0, "every span is admitted");
            *at += 1;
            snapshot(effect).1
        })
        .collect()
}

/// Gate 1. Attack 1 -> 50 ms, retargeted to 10 ms 20 frames in, and release 100 -> 2000 ms at the
/// same frame: each coefficient word follows an independent `LinearRamp` fed the same targets bit
/// for bit (so the mid-ramp retarget starts from the live coefficient), stays between its endpoints,
/// and lands on the new design exactly on its 64th frame.
///
/// Test value: a coefficient that jumps to the new design, retargets from the old target instead of
/// the live word, never moves (the kernel still reading a prepared word), or moves the other rate's
/// ramp turns it red; no other test reads the rate words.
#[test]
fn a_changed_attack_or_release_ramps_its_coefficient_from_the_live_value() {
    let mut effect = prepare(request(&initial_values()));
    let mut at = 0;
    let design = |ms: f32| attack_release_coefficient(ms, RATE);
    let mut attack = LinearRamp::fixed(design(1.0));
    let mut release = LinearRamp::fixed(design(100.0));
    let check = |sections: &[Vec<u8>], attack: &mut LinearRamp, release: &mut LinearRamp| {
        for section in sections {
            for (ramp, index, name) in [
                (&mut *attack, ATTACK_WORD, "attack"),
                (&mut *release, RELEASE_WORD, "release"),
            ] {
                let expected = ramp.next_value();
                assert_eq!(
                    word(section, index).to_bits(),
                    expected.to_bits(),
                    "{name} coefficient"
                );
                assert_eq!(
                    word(section, index + 1).to_bits(),
                    ramp.target.to_bits(),
                    "{name} target"
                );
                assert_eq!(
                    word(section, index + 3),
                    ramp.remaining as f32,
                    "{name} remaining"
                );
            }
        }
    };
    let first = frames(&mut effect, &mut at, 20, 0.5, &spans(4, 50.0));
    attack.set_target(design(50.0), 64);
    check(&first, &mut attack, &mut release);
    let (low, high) = (design(50.0), design(1.0));
    assert!(
        first
            .iter()
            .all(|s| (low..=high).contains(&word(s, ATTACK_WORD)))
    );
    assert!(
        word(&first[19], ATTACK_WORD) < high && word(&first[19], ATTACK_WORD) > low,
        "mid-ramp"
    );

    let mut both = spans(4, 10.0).to_vec();
    both.extend(spans(6, 2000.0));
    // Descriptor order, left before right: attack L, attack R, release L, release R.
    both.sort_by_key(|span| (span.parameter_index, span.channel != ParameterChannel::Left));
    let second = frames(&mut effect, &mut at, 70, 0.5, &both);
    attack.set_target(design(10.0), 64);
    release.set_target(design(2000.0), 64);
    let start = attack.current;
    check(&second, &mut attack, &mut release);
    for section in &second {
        let c = word(section, ATTACK_WORD);
        assert!(
            c >= start.min(design(10.0)) && c <= start.max(design(10.0)),
            "attack {c:e} inside its endpoints"
        );
    }
    assert_eq!(
        word(&second[63], ATTACK_WORD).to_bits(),
        design(10.0).to_bits(),
        "attack lands on frame 64"
    );
    assert_eq!(
        word(&second[63], RELEASE_WORD).to_bits(),
        design(2000.0).to_bits(),
        "release lands on frame 64"
    );
    assert_ne!(
        word(&second[62], RELEASE_WORD).to_bits(),
        design(2000.0).to_bits(),
        "release still moving on frame 63"
    );
}

/// Gate 2. Hold 100 ms (`H0 = 4,800`) on an open lane whose countdown is 3,000: 20 ms gives
/// `r' = 0` and the gate closes on that block's first frame; 200 ms gives `r' = 7,800` and it closes
/// after exactly 7,800 more holding frames; a closed lane keeps `r = 0` and its next opening reloads
/// the new count.
///
/// Test value: a hold that applies only at the next reload (shortening ignored, lengthening not
/// extending the current window) or a countdown left above `H1` turns it red.
#[test]
fn a_changed_hold_keeps_the_window_start() {
    let open_with_countdown = || {
        let mut values = initial_values();
        set_parameter(&mut values, 5, 100.0, 100.0);
        let mut effect = prepare(request(&values));
        let mut at = 0;
        frames(&mut effect, &mut at, 1, 0.5, &[]);
        let last = frames(&mut effect, &mut at, 1_800, 0.0, &[])
            .pop()
            .expect("frames");
        assert_eq!(
            (word(&last, 1), word(&last, 2)),
            (1.0, 3_000.0),
            "open, 3,000 of 4,800 left"
        );
        (effect, at)
    };

    let (mut effect, mut at) = open_with_countdown();
    let first = &frames(&mut effect, &mut at, 1, 0.0, &spans(5, 20.0))[0];
    assert_eq!(
        (word(first, 1), word(first, 2)),
        (0.0, 0.0),
        "r' = 0 closes on the first frame"
    );
    assert!(
        word(first, 0) < 0.0,
        "the release starts on the first frame"
    );

    let (mut effect, mut at) = open_with_countdown();
    let held = frames(&mut effect, &mut at, 7_801, 0.0, &spans(5, 200.0));
    assert_eq!(
        word(&held[0], 2),
        7_799.0,
        "r' = 7,800, then one holding frame"
    );
    assert_eq!(
        (word(&held[7_799], 1), word(&held[7_799], 2)),
        (1.0, 0.0),
        "still open after 7,800 frames"
    );
    assert_eq!(word(&held[7_800], 1), 0.0, "closed on the next frame");

    let closed = frames(&mut effect, &mut at, 1, 0.0, &spans(5, 500.0))
        .pop()
        .expect("frame");
    assert_eq!(
        (word(&closed, 1), word(&closed, 2)),
        (0.0, 0.0),
        "a closed lane keeps r = 0"
    );
    let opened = frames(&mut effect, &mut at, 1, 0.5, &[])
        .pop()
        .expect("frame");
    assert_eq!(
        (word(&opened, 1), word(&opened, 2)),
        (1.0, 24_000.0),
        "the opening reloads H1"
    );
}
