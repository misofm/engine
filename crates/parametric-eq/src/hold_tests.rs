//! Issue #1372: the hold of the recursion words on a ramp toward the identity (owner ruling of
//! 2026-11-04) and the rest of a band that finished turning off (owner ruling of 2026-11-05).
//! `docs/derivations/2091-eq-switched-system-strict-decay.md` §6.3 and §7.3.
//!
//! Each test runs at `W = 1` and at every vector width the build has, on the last lane of the
//! bank, beside lanes that keep a settled design.

use super::*;

const RATE: SampleRateHz = SampleRateHz(48_000);
const SECTION: usize = BAND_SECTION_OFFSET;

/// The EQ's slowest design: a bell at 10 Hz, Q 18, +24 dB.
fn extreme() -> BandTarget {
    BandTarget {
        enabled: true,
        kind: EqBandKind::Bell,
        frequency: 10.0,
        gain: 24.0,
        q: 18.0,
        slope: 1.0,
    }
}

fn disabled() -> BandTarget {
    BandTarget {
        enabled: false,
        ..extreme()
    }
}

/// A channel pair whose band section holds `band` on every lane; every other section is off.
fn pair<L: Lane, const W: usize>(band: BandTarget) -> (Channel<L, W>, Channel<L, W>) {
    let mut sections = [disabled(); EQ_SECTION_COUNT];
    sections[SECTION] = band;
    (
        Channel::<L, W>::new([sections; W], RATE).expect("legal"),
        Channel::<L, W>::new([sections; W], RATE).expect("legal"),
    )
}

/// One frame of input on every lane of both channels.
fn frame<L: Lane, const W: usize>(channels: (&mut Channel<L, W>, &mut Channel<L, W>), x: f32) {
    let stationary = channels.0.no_ramp_in_flight() && channels.1.no_ramp_in_flight();
    let mut left = vec![x; W];
    let mut right = vec![-x; W];
    process_channels_from_input(channels, &mut left, &mut right, 1, stationary);
}

/// `w + j s`, walked in `f32` one add per step, per word.
fn walk(start: [f32; 6], step: [f32; 6], steps: usize) -> [f32; 6] {
    let mut words = start;
    for _ in 0..steps {
        for (word, step) in words.iter_mut().zip(step) {
            *word += step;
        }
    }
    words
}

fn ramp_step(from: [f32; 6], to: [f32; 6]) -> [f32; 6] {
    core::array::from_fn(|index| (to[index] - from[index]) * RAMP_SCALE)
}

fn bits(words: [f32; 6]) -> [u32; 6] {
    words.map(f32::to_bits)
}

/// The recursion words of `recursion` and the mix words of `mix`.
fn splice(recursion: [f32; 6], mix: [f32; 6]) -> [f32; 6] {
    core::array::from_fn(|index| {
        if index < 3 {
            recursion[index]
        } else {
            mix[index]
        }
    })
}

/// Rule 1. A lane settled at the extreme design is disabled; one frame per block. Frames 0 to 60
/// run today's walk `fl(w0 + j s)` word for word; on frames 61 to 63 the recursion words
/// `(c1, a2, a3)` repeat frame 60's bits (weight 4/64 = 1/16) while the mix words keep walking;
/// frame 64 is the identity. Every other lane keeps the design.
///
/// Red on an off-by-one stop (frame 60 or 61 not the floor), on held mix words, on a hold that
/// lands on a neighbour lane, or with no hold (base: the recursion words keep walking to 1/64).
#[test]
fn a_disable_holds_the_recursion_words_at_one_sixteenth() {
    fn run<L: Lane, const W: usize>(width: &str) {
        let design = extreme().words(RATE).expect("legal").to_array();
        let identity = EqSvfWords::IDENTITY.to_array();
        let step = ramp_step(design, identity);
        let lane = W - 1;
        let (mut left, mut right) = pair::<L, W>(extreme());
        left.apply_prepared_target(lane, SECTION, disabled(), EqSvfWords::IDENTITY);
        right.apply_prepared_target(lane, SECTION, disabled(), EqSvfWords::IDENTITY);
        for j in 0..=RAMP_SAMPLES as usize {
            let expected = if j == RAMP_SAMPLES as usize {
                identity
            } else {
                splice(walk(design, step, j.min(60)), walk(design, step, j))
            };
            for channel in [&left, &right] {
                assert_eq!(
                    bits(channel.lane_words(SECTION, lane).to_array()),
                    bits(expected),
                    "{width}: frame {j} of the disable"
                );
                for other in 0..lane {
                    assert_eq!(
                        bits(channel.lane_words(SECTION, other).to_array()),
                        bits(design),
                        "{width}: frame {j}: lane {other} keeps its design"
                    );
                }
            }
            frame((&mut left, &mut right), 0.25);
        }
    }
    run::<f32, 1>(&width_label(1));
    lane::each_vector_lane!(|L, W| run::<L, W>(&width_label(W)));
}

/// Rule 3. A disabled lane is enabled toward the extreme design at frame 0 and disabled again at
/// frame `d` in 1 to 3, one frame per block (a quantum of at most three frames). The recursion
/// words keep their rising walk toward the design until their weight reaches 1/16 at frame 4,
/// and then hold until frame `d + 64`; the mix words walk from frame `d`'s words toward the
/// identity's for 64 frames.
///
/// Red on a hold frozen below the floor or on recursion words that turn back toward the identity
/// at the disable (base: they do).
#[test]
fn a_disable_within_three_frames_of_an_enable_rises_to_the_floor() {
    fn run<L: Lane, const W: usize>(width: &str, d: usize) {
        let design = extreme().words(RATE).expect("legal");
        let identity = EqSvfWords::IDENTITY.to_array();
        let up = ramp_step(identity, design.to_array());
        let lane = W - 1;
        let (mut left, mut right) = pair::<L, W>(disabled());
        left.apply_prepared_target(lane, SECTION, extreme(), design);
        right.apply_prepared_target(lane, SECTION, extreme(), design);
        let at_disable = walk(identity, up, d);
        let down = ramp_step(at_disable, identity);
        for f in 0..=d + RAMP_SAMPLES as usize {
            if f == d {
                left.apply_prepared_target(lane, SECTION, disabled(), EqSvfWords::IDENTITY);
                right.apply_prepared_target(lane, SECTION, disabled(), EqSvfWords::IDENTITY);
            }
            let expected = if f == d + RAMP_SAMPLES as usize {
                identity
            } else if f < d {
                walk(identity, up, f)
            } else {
                splice(walk(identity, up, f.min(4)), walk(at_disable, down, f - d))
            };
            for channel in [&left, &right] {
                assert_eq!(
                    bits(channel.lane_words(SECTION, lane).to_array()),
                    bits(expected),
                    "{width}: disable at frame {d}, frame {f}"
                );
            }
            frame((&mut left, &mut right), 0.25);
        }
    }
    for d in 1..=3 {
        run::<f32, 1>(&width_label(1), d);
        lane::each_vector_lane!(|L, W| run::<L, W>(&width_label(W), d));
    }
}

/// The payload carries the recursion weight and its increment; the step count is derived. A lane
/// snapshotted at frame 62 of a disable (held at weight 1/16) restores; the same payload with the
/// weight forged below the floor, or with a rising increment at the floor, is refused and moves
/// nothing.
///
/// Red on a restore that admits recursion words held below the floor.
#[test]
fn a_restore_refuses_a_weight_held_below_the_floor() {
    fn run<L: Lane, const W: usize>(width: &str) {
        let lane = W - 1;
        let (mut left, mut right) = pair::<L, W>(extreme());
        left.apply_prepared_target(lane, SECTION, disabled(), EqSvfWords::IDENTITY);
        right.apply_prepared_target(lane, SECTION, disabled(), EqSvfWords::IDENTITY);
        for _ in 0..62 {
            frame((&mut left, &mut right), 0.25);
        }
        let mut words = [0_u32; STATE_LANE_WORDS];
        left.snapshot_track(lane, &mut words);
        let base = SECTION * STATE_WORDS_PER_BAND;
        assert_eq!(
            words[base + STATE_WEIGHT_WORD],
            HOLD_WEIGHT.to_bits(),
            "{width}: held at the floor"
        );
        let configuration =
            Channel::<L, W>::decode_configuration(&words, &left.targets[lane]).expect("config");
        let (mut receiver, _) = pair::<L, W>(extreme());
        receiver
            .restore_track(lane, &words, &configuration, RATE)
            .expect("the held payload restores");
        for (forged_weight, forged_step) in [
            (HOLD_WEIGHT / 2.0, 0.0_f32),
            (HOLD_WEIGHT, HOLD_WEIGHT * RAMP_SCALE),
        ] {
            let mut forged = words;
            forged[base + STATE_WEIGHT_WORD] = forged_weight.to_bits();
            forged[base + STATE_WEIGHT_STEP_WORD] = forged_step.to_bits();
            let mut before = [0_u32; STATE_LANE_WORDS];
            receiver.snapshot_track(lane, &mut before);
            assert!(
                receiver
                    .restore_track(lane, &forged, &configuration, RATE)
                    .is_err(),
                "{width}: weight {forged_weight}, increment {forged_step} must be refused"
            );
            let mut after = [0_u32; STATE_LANE_WORDS];
            receiver.snapshot_track(lane, &mut after);
            assert_eq!(before, after, "{width}: a refusal moves nothing");
        }
    }
    run::<f32, 1>(&width_label(1));
    lane::each_vector_lane!(|L, W| run::<L, W>(&width_label(W)));
}

/// The rest ruling. A lane filtering a signal through a +9 dB bell is disabled live and fed the
/// signal past frame 64 of the disable, then one block of `+0.0`: the lane is at exact rest (its
/// bit in `rest_lanes`), and its band's integrators are `+0.0`.
///
/// Red on a disabled band that keeps the state of a signal long gone and so never rests (base).
#[test]
fn a_finished_disable_reaches_exact_rest() {
    fn run<L: Lane, const W: usize>(width: &str) {
        let bell = BandTarget {
            frequency: 1_000.0,
            gain: 9.0,
            q: 1.0,
            ..extreme()
        };
        let lane = W - 1;
        let (mut left, mut right) = pair::<L, W>(bell);
        let noise = |n: usize| ((n * 7_919 % 1_013) as f32 / 1_013.0 - 0.5) * 0.8;
        for n in 0..200 {
            frame((&mut left, &mut right), noise(n));
        }
        left.apply_prepared_target(lane, SECTION, disabled(), EqSvfWords::IDENTITY);
        right.apply_prepared_target(lane, SECTION, disabled(), EqSvfWords::IDENTITY);
        for n in 200..300 {
            frame((&mut left, &mut right), noise(n));
        }
        let stationary = left.no_ramp_in_flight() && right.no_ramp_in_flight();
        assert!(stationary, "{width}: the disable has ended");
        let mut l = vec![0.0_f32; 16 * W];
        let mut r = vec![0.0_f32; 16 * W];
        process_channels_from_input((&mut left, &mut right), &mut l, &mut r, 16, stationary);
        for channel in [&left, &right] {
            let state = channel.sections[SECTION].state;
            assert_eq!(
                [lane_bits(state.ic1, lane), lane_bits(state.ic2, lane)],
                [0, 0],
                "{width}: the band's integrators are +0.0"
            );
            assert_ne!(
                channel.rest_lanes() & (1 << lane),
                0,
                "{width}: the lane is at exact rest"
            );
        }
    }
    run::<f32, 1>(&width_label(1));
    lane::each_vector_lane!(|L, W| run::<L, W>(&width_label(W)));
}
