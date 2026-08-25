//! The ramping split's bit-identity gate (issue #149 phase 3).
//!
//! [`process_block`] chooses between a ramped segment and a flat one from the ramp counters. The
//! claim that makes that a class-A optimisation rather than a re-tuning is narrow and total:
//! **for every block, of every width, under every automation schedule, the split renders the same
//! bytes and leaves the same state as the unsplit form.** The unsplit form has not been deleted to
//! make that checkable — it is `FORCE_RAMPING == true`, which sends every segment down the ramped
//! path and performs the write-back, exactly as the code did before the split existed.
//!
//! So each test here runs one scenario twice from two freshly prepared instances, split on and
//! split off, and compares two things:
//!
//! * every rendered sample of every block, by bit pattern (`-0.0` and `+0.0` are different
//!   answers here, which is the whole reason the flat path excludes `-0.0`); and
//! * the entire instance afterwards — ramps, smoother, filter, rings, cursor, coefficient caches —
//!   by [`fingerprint`]. Output equality alone would not catch a ramp left one sample short.
//!
//! The scenarios are the split's boundaries: a ramp already in flight when the block opens, a ramp
//! arriving mid-block, a ramp arriving exactly on the last sample of a block, a parameter restated
//! at the value it already holds (the phase-1 hoist, which settles instead of arming and so is the
//! case that puts a *just-retargeted* bank on the flat path), and no traffic at all.

use super::*;
use miso_engine_effect_contract::{
    EffectQuality, PrepareEffectLimits, PreparedPortsV1, PreparedSidechainPort,
};
use miso_engine_lane::{Simd4, Simd8};

/// Ramp index of each band's threshold, ratio, attack, release and makeup.
const LOW_THRESHOLD: usize = 0;
const LOW_RATIO: usize = 1;
const LOW_ATTACK: usize = 2;
const LOW_MAKEUP: usize = 4;
const HIGH_THRESHOLD: usize = 5;
const HIGH_RELEASE: usize = 8;

/// One automation event: track, channel, ramp index, and the value to send.
#[derive(Clone, Copy)]
struct Event {
    track: usize,
    channel: ParameterChannel,
    ramp: usize,
    value: f32,
}

/// The block a scenario delivers each of its events on, with the events themselves.
type Schedule<'a> = &'a [(usize, &'a [Event])];

const fn event(track: usize, channel: ParameterChannel, ramp: usize, value: f32) -> Event {
    Event {
        track,
        channel,
        ramp,
        value,
    }
}

/// A prepare request at 48 kHz over `values`.
fn request(
    values: &[InitialParameterValue],
    link_mode: LinkMode,
    quantum: u32,
    bypass: bool,
) -> PrepareEffectRequest<'_> {
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum,
        quality: EffectQuality::Normal,
        bypass,
        link_mode,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: 32,
        },
    }
}

/// The descriptor's defaults as an initial-value table.
fn defaults() -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    core::array::from_fn(|index| InitialParameterValue {
        parameter_index: (index / 2) as u32,
        channel: if index.is_multiple_of(2) {
            ParameterChannel::Left
        } else {
            ParameterChannel::Right
        },
        value: MULTIBAND_COMPRESSOR_DESCRIPTOR_V1.parameters[index / 2].default_value,
    })
}

/// Per-track values that put no two lanes on the same program.
///
/// Lookahead cycles through 0, 5 and 20 ms so the per-track detector gather is load-bearing, and
/// the thresholds, ratios and makeups spread the lanes across the static curve. The left and right
/// channels are deliberately given different values: a channel-symmetric bank would not notice a
/// split that confused the two sides.
fn varied(track: usize) -> [InitialParameterValue; PARAMETER_COUNT * 2] {
    let mut values = defaults();
    let offset = track as f32;
    for channel in 0..2 {
        let tilt = offset + channel as f32 * 0.5;
        values[2 + channel].value = [0.0, 5.0, 20.0][track % 3];
        values[4 + channel].value = -30.0 - tilt;
        values[6 + channel].value = 2.0 + tilt * 0.5;
        values[8 + channel].value = 5.0 + tilt;
        values[10 + channel].value = 80.0 + tilt * 3.0;
        values[12 + channel].value = 1.0 + tilt * 0.25;
        values[14 + channel].value = -24.0 - tilt;
        values[16 + channel].value = 3.0 + tilt * 0.25;
        values[18 + channel].value = 1.0 + tilt;
        values[20 + channel].value = 120.0 + tilt * 5.0;
        values[22 + channel].value = -1.0 - tilt * 0.25;
    }
    values
}

/// A prepared instance of `W` tracks, each with its own program.
fn instance<L: Lane, const W: usize>(link_mode: LinkMode, bypass: bool) -> Instance<L, W> {
    let mut left = [[0.0; PARAMETER_COUNT]; W];
    let mut right = [[0.0; PARAMETER_COUNT]; W];
    let mut metadata = None;
    for (track, (left_track, right_track)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
        let values = varied(track);
        let request = request(&values, link_mode, 128, bypass);
        let prepared = expected_prepared_metadata(&MULTIBAND_COMPRESSOR_DESCRIPTOR_V1, request)
            .expect("the descriptor accepts its own defaults");
        let (track_left, track_right) =
            initial_defaults(&values).expect("the defaults are in domain");
        *left_track = track_left;
        *right_track = track_right;
        metadata = Some(prepared);
    }
    Instance::<L, W>::new(left, right, metadata.expect("at least one track")).expect("prepares")
}

/// A deterministic, wide-band, non-symmetric stimulus.
///
/// Two decorrelated tones plus a slow sweep, offset per lane and per channel, scaled so the bands
/// spend time on both sides of every threshold. No lane is silent and no lane is a copy of another,
/// which is what makes a per-lane confusion in the split observable.
///
/// It is a function of the **absolute** sample index, not of the block index, so that the same
/// stretch of signal is delivered however the caller partitions it.
fn stimulus(sample: usize, track: usize, channel: usize) -> f32 {
    let index = sample as f32;
    let seed = track as f32 * 0.37 + channel as f32 * 0.11 + 1.0;
    let low = miso_engine_math::sin(f64::from(index) * 0.017 * f64::from(seed)) as f32;
    let high = miso_engine_math::sin(f64::from(index) * 0.41 * f64::from(seed) + 0.7) as f32;
    let envelope =
        0.05 + 0.9 * (0.5 + 0.5 * miso_engine_math::sin(f64::from(index) * 0.0007) as f32);
    (0.7 * low + 0.3 * high) * envelope
}

/// Every mutable word of an instance, as bit patterns, in a fixed order.
///
/// This is deliberately the whole of the state and not a summary: the ramps (so a split that
/// stopped a ramp one sample early is caught), the branching smoother, the crossover filter, both
/// delay rings, the cursor, and the derived per-track coefficient caches.
fn fingerprint<L: Lane, const W: usize>(instance: &Instance<L, W>) -> Vec<u32> {
    let mut words = vec![instance.cursor as u32];
    for side in &instance.sides {
        for lane in [
            side.coefficients.nc1,
            side.coefficients.a2,
            side.coefficients.a3,
            side.coefficients.nk2,
            side.filter.a.ic1,
            side.filter.a.ic2,
            side.filter.b.ic1,
            side.filter.b.ic2,
            side.gain_db[LOW_BAND],
            side.gain_db[HIGH_BAND],
        ] {
            for track in 0..W {
                words.push(lane_value(lane, track).to_bits());
            }
        }
        for track in 0..W {
            for ramp in &side.ramps[track] {
                words.push(ramp.current.to_bits());
                words.push(ramp.target.to_bits());
                words.push(ramp.step.to_bits());
                words.push(ramp.remaining);
            }
            for cache in &side.cache[track] {
                words.extend_from_slice(&cache.key);
                words.push(cache.inv_ratio_minus_one.to_bits());
                words.push(cache.attack.to_bits());
                words.push(cache.release.to_bits());
            }
            for value in side.designed[track] {
                words.push(value.to_bits());
            }
            words.push(side.crossover_hz[track].to_bits());
            words.push(side.lookahead_ms[track].to_bits());
            words.push(side.detector_offset[track] as u32);
        }
        for value in side.low_ring.iter().chain(side.high_ring.iter()) {
            words.push(value.to_bits());
        }
    }
    words
}

/// Renders `blocks` blocks of `frames` frames under `schedule`, split on or forced off.
///
/// Returns every rendered sample of every block as a bit pattern, followed by the instance's
/// closing fingerprint, so one comparison covers both halves of the claim.
fn run<L: Lane, const W: usize, const FORCE_RAMPING: bool>(
    link_mode: LinkMode,
    bypass: bool,
    blocks: usize,
    frames: usize,
    schedule: Schedule<'_>,
) -> Vec<u32> {
    let mut instance = instance::<L, W>(link_mode, bypass);
    let mut rendered = Vec::new();
    let mut left = vec![0.0f32; frames * W];
    let mut right = vec![0.0f32; frames * W];
    for block in 0..blocks {
        for (index, events) in schedule.iter().copied() {
            if index != block {
                continue;
            }
            for track in 0..W {
                let spans: Vec<PreparedAutomationSpan> = events
                    .iter()
                    .filter(|event| event.track == track)
                    .map(|event| PreparedAutomationSpan {
                        kind: AutomationSpanKind::Point,
                        channel: event.channel,
                        parameter_index: (event.ramp + 2) as u32,
                        start_sample: (block * frames) as u64,
                        end_sample: (block * frames) as u64,
                        start_value: event.value,
                        end_value: event.value,
                    })
                    .collect();
                let mut report = ProcessReport::default();
                instance.apply_automation(track, &spans, 32, (block * frames) as u64, &mut report);
                assert_eq!(
                    report.invalid_spans, 0,
                    "the scenario's spans are admissible"
                );
            }
        }
        for frame in 0..frames {
            for track in 0..W {
                let sample = block * frames + frame;
                left[frame * W + track] = stimulus(sample, track, 0);
                right[frame * W + track] = stimulus(sample, track, 1);
            }
        }
        let mut reports = vec![ProcessReport::default(); W];
        render::<L, W, FORCE_RAMPING>(&mut instance, &mut left, &mut right, frames, &mut reports);
        rendered.extend(left.iter().chain(right.iter()).map(|value| value.to_bits()));
    }
    rendered.extend(fingerprint(&instance));
    rendered
}

/// Runs one scenario at all three widths and both non-trivial link modes, split on and off.
fn identical(blocks: usize, frames: usize, schedule: Schedule<'_>) {
    for (link_mode, bypass) in [
        (LinkMode::DualMono, false),
        (LinkMode::Maximum, false),
        (LinkMode::Average, false),
        (LinkMode::DualMono, true),
    ] {
        let split = run::<f32, 1, false>(link_mode, bypass, blocks, frames, schedule);
        let unsplit = run::<f32, 1, true>(link_mode, bypass, blocks, frames, schedule);
        assert_eq!(split, unsplit, "scalar, {link_mode:?}, bypass {bypass}");

        let split = run::<Simd4, 4, false>(link_mode, bypass, blocks, frames, schedule);
        let unsplit = run::<Simd4, 4, true>(link_mode, bypass, blocks, frames, schedule);
        assert_eq!(split, unsplit, "simd4, {link_mode:?}, bypass {bypass}");

        let split = run::<Simd8, 8, false>(link_mode, bypass, blocks, frames, schedule);
        let unsplit = run::<Simd8, 8, true>(link_mode, bypass, blocks, frames, schedule);
        assert_eq!(split, unsplit, "simd8, {link_mode:?}, bypass {bypass}");
    }
}

/// No automation at all: every block is one flat segment. The case the split exists for.
#[test]
fn a_settled_bank_renders_what_the_ramped_path_rendered() {
    identical(6, 128, &[]);
}

/// A sixty-four sample window opened at the top of a hundred-and-twenty-eight frame block, so the
/// ramp arrives on frame 63 and the rest of the block is flat. The split must cut the block in the
/// same place the unsplit form did and must not shorten the window by a sample.
#[test]
fn a_ramp_arriving_mid_block_arrives_on_the_same_sample() {
    identical(
        6,
        128,
        &[(
            1,
            &[
                event(0, ParameterChannel::Left, LOW_THRESHOLD, -34.0),
                event(0, ParameterChannel::Right, HIGH_THRESHOLD, -21.0),
            ],
        )],
    );
}

/// The same window against sixty-four frame blocks, so the ramp's final sample is the block's
/// final sample: the snap and the block boundary coincide, and the next block opens fully settled.
#[test]
fn a_ramp_arriving_on_a_block_boundary_arrives_on_the_same_sample() {
    identical(
        6,
        64,
        &[(
            1,
            &[
                event(0, ParameterChannel::Left, LOW_THRESHOLD, -40.0),
                event(0, ParameterChannel::Right, LOW_MAKEUP, 3.5),
            ],
        )],
    );
}

/// A window opened against thirty-two frame blocks, so it is still in flight when the next two
/// blocks open: the split has to take the ramped path on a block it did not start.
#[test]
fn a_ramp_in_flight_when_a_block_opens_stays_in_flight() {
    identical(
        8,
        32,
        &[(
            1,
            &[
                event(0, ParameterChannel::Left, LOW_RATIO, 9.0),
                event(0, ParameterChannel::Left, LOW_ATTACK, 22.0),
                event(0, ParameterChannel::Right, HIGH_RELEASE, 400.0),
            ],
        )],
    );
}

/// Two overlapping windows on different tracks, retargeted again while the first is still open.
///
/// This is the case whole-bank granularity is about: one track's window keeps every other track on
/// the ramped path, and a retarget mid-flight re-derives the step from the value in force.
#[test]
fn overlapping_windows_on_different_tracks_agree() {
    identical(
        8,
        128,
        &[
            (
                1,
                &[
                    event(0, ParameterChannel::Left, LOW_THRESHOLD, -44.0),
                    event(1, ParameterChannel::Right, HIGH_THRESHOLD, -12.0),
                ],
            ),
            (1, &[event(2, ParameterChannel::Left, LOW_MAKEUP, 6.0)]),
            (
                2,
                &[
                    event(0, ParameterChannel::Left, LOW_THRESHOLD, -50.0),
                    event(3, ParameterChannel::Right, LOW_RATIO, 15.0),
                ],
            ),
        ],
    );
}

/// A parameter restated at the value it already holds — issue #144 item 6's hoist.
///
/// `set_target` settles rather than arming, so the bank stays on the flat path through a block
/// that carried automation traffic. That is precisely the arrangement in which a split keyed on
/// the counters could disagree with one keyed on "did automation arrive", and it does not.
#[test]
fn a_restated_parameter_stays_on_the_flat_path() {
    const RESTATED: &[Event] = &[
        event(0, ParameterChannel::Left, LOW_THRESHOLD, -30.0),
        event(0, ParameterChannel::Right, LOW_THRESHOLD, -30.5),
        event(1, ParameterChannel::Left, HIGH_THRESHOLD, -25.0),
    ];
    identical(6, 128, &[(1, RESTATED), (2, RESTATED), (3, RESTATED)]);
}

/// A restatement of a value a ramp is still travelling towards, delivered mid-flight.
///
/// The hoist cancels the flight by settling at the value in force, which drops the bank onto the
/// flat path in the middle of a window rather than at its end.
#[test]
fn a_restatement_mid_flight_agrees() {
    identical(
        8,
        128,
        &[
            (1, &[event(0, ParameterChannel::Left, LOW_THRESHOLD, -44.0)]),
            (2, &[event(0, ParameterChannel::Left, LOW_THRESHOLD, -44.0)]),
        ],
    );
}

/// The split must not make a block's rendering depend on how the caller partitions it.
///
/// One hundred and twenty-eight frames rendered as one block, as two of sixty-four and as four of
/// thirty-two, with a window open across every boundary, all produce the same samples and the same
/// closing state. This is the property `plan_segment` protects by cutting on absolute ramp
/// positions, restated here with the split in force.
#[test]
fn the_split_is_partition_invariant() {
    const SCHEDULE: Schedule<'static> = &[(
        0,
        &[
            event(0, ParameterChannel::Left, LOW_THRESHOLD, -47.0),
            event(1, ParameterChannel::Right, HIGH_THRESHOLD, -9.0),
        ],
    )];
    let whole = run::<Simd4, 4, false>(LinkMode::Maximum, false, 1, 128, SCHEDULE);
    let halves = run::<Simd4, 4, false>(LinkMode::Maximum, false, 2, 64, SCHEDULE);
    let quarters = run::<Simd4, 4, false>(LinkMode::Maximum, false, 4, 32, SCHEDULE);
    assert_eq!(whole, halves, "one block against two");
    assert_eq!(whole, quarters, "one block against four");
}

/// The flat path's precondition, stated directly rather than only asserted in debug builds.
///
/// A settled bank must hold `remaining == 0`, `step == +0.0`, and a `current` that survives
/// `x + 0.0` bit for bit — finite, and not `-0.0`. Every door a parameter word enters by is
/// covered: the prepared defaults, an automation point, a snap at the end of a window, and a
/// restatement that settles without arming.
#[test]
fn a_settled_bank_meets_the_flat_paths_precondition() {
    let mut instance = instance::<Simd4, 4>(LinkMode::Average, false);
    assert!(instance.flat_path_is_identity(), "prepared defaults");

    let mut left = vec![0.0f32; 128 * 4];
    let mut right = vec![0.0f32; 128 * 4];
    let mut reports = vec![ProcessReport::default(); 4];
    let mut report = ProcessReport::default();

    // A zero sent to a parameter whose domain contains zero, as `-0.0`: `normalize_zero` is what
    // keeps it off the flat path's excluded value, so send the bit pattern that would break it.
    let spans = [PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel: ParameterChannel::Left,
        parameter_index: (LOW_MAKEUP + 2) as u32,
        start_sample: 0,
        end_sample: 0,
        start_value: -0.0,
        end_value: -0.0,
    }];
    instance.apply_automation(0, &spans, 32, 0, &mut report);
    assert_eq!(report.invalid_spans, 0, "-0.0 is inside the makeup domain");

    // Drive past the end of the sixty-four sample window, then confirm the bank settled and that
    // the value it settled on is `+0.0` and not the `-0.0` that was sent.
    for _ in 0..2 {
        render::<Simd4, 4, false>(&mut instance, &mut left, &mut right, 128, &mut reports);
    }
    assert!(instance.flat_path_is_identity(), "after a window closed");
    let settled = instance.sides[0].ramps[0][LOW_MAKEUP];
    assert_eq!(settled.remaining, 0);
    assert_eq!(settled.current.to_bits(), 0.0f32.to_bits(), "not -0.0");

    // A discontinuity reset snaps every ramp; the precondition must survive it.
    instance.reset(ResetKind::DiscontinuityKeepParameters);
    assert!(
        instance.flat_path_is_identity(),
        "after a discontinuity reset"
    );
}

/// The ramped path must actually advance, and each channel must advance its own ramps.
///
/// Every test above compares the split against the unsplit form, so a fault in the ramped kernel
/// the two arms *share* cancels out of them: both would render the same wrong thing and agree.
/// This is the other half of the claim, and it is stated per channel because deleting the right
/// channel's advance outright — `segments[1]`'s line in [`run_segment`] — passes every other gate
/// in this crate, which is a gap this test closes rather than a property it assumes.
///
/// Makeup is the parameter it moves because makeup is added to the smoother's output
/// unconditionally, so a window that advances changes the *first* sample it covers whatever the
/// detector happens to be doing. A window that does not advance renders the value it started from
/// until the snap, which is exactly what the assertion below distinguishes.
fn each_channel_advances_its_own_ramps<L: Lane, const W: usize>(label: &str) {
    // The effect declares `Fs/50` of lookahead latency — 960 samples at 48 kHz — so the ring has
    // to be full before any of this is observable at all. The window is opened well past that, and
    // the block it opens on is the block compared.
    const FRAMES: usize = 128;
    const BLOCKS: usize = 14;
    const OPENED: usize = 12;
    let quiet = run::<L, W, false>(LinkMode::DualMono, false, BLOCKS, FRAMES, &[]);
    let left_only = run::<L, W, false>(
        LinkMode::DualMono,
        false,
        BLOCKS,
        FRAMES,
        &[(OPENED, &[event(0, ParameterChannel::Left, LOW_MAKEUP, 9.0)])],
    );
    let right_only = run::<L, W, false>(
        LinkMode::DualMono,
        false,
        BLOCKS,
        FRAMES,
        &[(
            OPENED,
            &[event(0, ParameterChannel::Right, LOW_MAKEUP, 9.0)],
        )],
    );
    // `run` appends a block as the whole left plane followed by the whole right plane, so the
    // opening block's first frame of track 0 is at these two offsets.
    let left = OPENED * 2 * FRAMES * W;
    let right = left + FRAMES * W;
    assert_ne!(
        left_only[left], quiet[left],
        "{label}: a left window must move its own first sample"
    );
    assert_eq!(
        left_only[right], quiet[right],
        "{label}: and must leave the right channel alone"
    );
    assert_ne!(
        right_only[right], quiet[right],
        "{label}: a right window must move its own first sample"
    );
    assert_eq!(
        right_only[left], quiet[left],
        "{label}: and must leave the left channel alone"
    );
}

#[test]
fn the_ramped_path_advances_both_channels() {
    each_channel_advances_its_own_ramps::<f32, 1>("scalar");
    each_channel_advances_its_own_ramps::<Simd4, 4>("simd4");
    each_channel_advances_its_own_ramps::<Simd8, 8>("simd8");
}

/// D11's snap, pinned: a window ends *on* its target, on the exact sample it was sent to.
///
/// The split must not move where a ramp arrives, and `plan_segment` is what holds that: it snaps a
/// ramp one sample from its target at a segment boundary, because the snap is an assignment and an
/// assignment cannot happen inside a vectorised run. Nothing in this crate pinned that before, so
/// the snap could be removed outright — leaving the window to arrive at `current + step` iterated,
/// which is `target` only up to a rounding error — without any gate noticing.
///
/// The test is written so that it can only pass for the right reason: it first asserts that the
/// chosen window genuinely accumulates an error, so that "arrived exactly on target" is a claim
/// about the snap and not an accident of a window whose step happens to divide exactly.
#[test]
fn a_window_lands_on_its_target_on_the_exact_sample() {
    const TARGET: f32 = 8.3;
    let mut instance = instance::<f32, 1>(LinkMode::DualMono, false);
    let start = instance.sides[0].ramps[0][LOW_MAKEUP].current;
    let step = (TARGET - start) / SMOOTHING_SAMPLES as f32;

    // The iterated form, which is what the ramp would arrive at with the snap removed.
    let mut walked = start;
    for _ in 0..SMOOTHING_SAMPLES {
        walked += step;
    }
    assert_ne!(
        walked.to_bits(),
        TARGET.to_bits(),
        "this window must accumulate an error, or it cannot witness the snap"
    );

    let spans = [PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel: ParameterChannel::Left,
        parameter_index: (LOW_MAKEUP + 2) as u32,
        start_sample: 0,
        end_sample: 0,
        start_value: TARGET,
        end_value: TARGET,
    }];
    let mut report = ProcessReport::default();
    instance.apply_automation(0, &spans, 32, 0, &mut report);
    assert_eq!(report.invalid_spans, 0);
    assert_eq!(
        instance.sides[0].ramps[0][LOW_MAKEUP].remaining, SMOOTHING_SAMPLES,
        "the window opens at its full length"
    );

    // One frame per block, so the ramp's write-back happens after every single sample and the
    // countdown can be read at every point in the window.
    let mut left = [0.0f32; 1];
    let mut right = [0.0f32; 1];
    let mut reports = [ProcessReport::default()];
    for sample in 1..=SMOOTHING_SAMPLES {
        left[0] = stimulus(sample as usize, 0, 0);
        right[0] = stimulus(sample as usize, 0, 1);
        render::<f32, 1, false>(&mut instance, &mut left, &mut right, 1, &mut reports);
        let ramp = instance.sides[0].ramps[0][LOW_MAKEUP];
        assert_eq!(
            ramp.remaining,
            SMOOTHING_SAMPLES - sample,
            "the window counts down one sample per frame"
        );
        if sample < SMOOTHING_SAMPLES {
            assert_ne!(
                ramp.current.to_bits(),
                TARGET.to_bits(),
                "the window must not arrive early, at sample {sample}"
            );
        }
    }
    assert_eq!(
        instance.sides[0].ramps[0][LOW_MAKEUP].current.to_bits(),
        TARGET.to_bits(),
        "the window arrives exactly on its target, on its last sample"
    );
}

// -------------------------------------------------------------------------------------------
// The descriptive cost of the split (issue #149 phase 3)
// -------------------------------------------------------------------------------------------
//
// Run with:
//
// ```text
// cargo test --release -p miso-engine-multiband-compressor --lib \
//     -- --ignored --nocapture the_split_costs
// ```
//
// **Where this measures, and why not the console fixture.** The standing issue-149 benchmark is a
// sixty-four track console session, and that session contains no multiband compressor: every track
// carries a parametric EQ in `simd1` and a compressor in `dynamic`, and `simd2` is empty on all
// sixty-four. The nine-track ragged strip and the hundred-and-twenty-eight track stretch are the
// same file cloned. So the console fixture cannot move for this change, and putting a multiband
// into it would redefine all three workloads and end their comparability with the phase-1 and
// phase-2 records. The measurement therefore happens where the effect is, at the bank boundary —
// the same choice, for the same reason, that the stationary hoist's ruling records for the EQ.
//
// **Paired alternation** (#104). The six subjects are interleaved observation by observation, not
// run one after another, so every drift a run suffers is shared by all of them and the split's
// delta is a distribution rather than a difference of two summaries. One warmup pass and two
// measured rounds; nothing is tuned or retried between them.
//
// The two arms of each pair differ only in `FORCE_RAMPING`, so the paired delta is the split and
// nothing else: same instance construction, same stimulus, same automation, same plumbing. The
// automation is applied outside the timed region, because admitting a span is not what changed.

/// Observations per round, matching the console benchmark's.
#[cfg(test)]
const OBSERVATIONS: usize = 1_000;

/// What automation traffic a subject carries.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Arm {
    /// No traffic. Every block is one flat segment: the case the split exists for.
    Quiet,
    /// Every parameter re-sent at the value it already holds. The phase-1 hoist settles each one,
    /// so the bank stays flat through a block that carried a full automation refresh.
    Restated,
    /// Two parameters alternating over a quarter of a dB, so a sixty-four sample window is open
    /// over the first half of every hundred-and-twenty-eight frame block.
    Moving,
}

impl Arm {
    const fn label(self) -> &'static str {
        match self {
            Self::Quiet => "quiet   ",
            Self::Restated => "restated",
            Self::Moving => "moving  ",
        }
    }
}

/// Nearest-rank percentile, the same rule the console benchmark uses.
fn percentile(sorted: &[f64], fraction: f64) -> f64 {
    let rank = (fraction * sorted.len() as f64).ceil().max(1.0) as usize;
    sorted[rank.min(sorted.len()) - 1]
}

/// Sends one arm's traffic for `block`, outside the timed region.
fn traffic<L: Lane, const W: usize>(instance: &mut Instance<L, W>, arm: Arm, block: usize) {
    if arm == Arm::Quiet {
        return;
    }
    let first = (block * 128) as u64;
    for track in 0..W {
        let mut spans = Vec::new();
        for ramp in 0..RAMP_COUNT {
            for channel in [ParameterChannel::Left, ParameterChannel::Right] {
                let side = usize::from(channel == ParameterChannel::Right);
                let held = instance.sides[side].ramps[track][ramp].target;
                let value = match arm {
                    Arm::Restated => held,
                    // Only the two thresholds move, which is what a console fader-style refresh
                    // looks like: most of the table is restated and a little of it is moving.
                    Arm::Moving if ramp == LOW_THRESHOLD || ramp == HIGH_THRESHOLD => {
                        if block.is_multiple_of(2) {
                            held + 0.25
                        } else {
                            held - 0.25
                        }
                    }
                    _ => held,
                };
                spans.push(PreparedAutomationSpan {
                    kind: AutomationSpanKind::Point,
                    channel,
                    parameter_index: (ramp + 2) as u32,
                    start_sample: first,
                    end_sample: first,
                    start_value: value,
                    end_value: value,
                });
            }
        }
        let mut report = ProcessReport::default();
        instance.apply_automation(track, &spans, 32, first, &mut report);
    }
}

/// One timed subject: an instance, its buffers, and the arm it carries.
struct Subject<L: Lane, const W: usize> {
    arm: Arm,
    split: bool,
    instance: Instance<L, W>,
    left: Vec<f32>,
    right: Vec<f32>,
    reports: Vec<ProcessReport>,
    samples: Vec<f64>,
}

impl<L: Lane, const W: usize> Subject<L, W> {
    fn new(arm: Arm, split: bool) -> Self {
        Self {
            arm,
            split,
            instance: instance::<L, W>(LinkMode::Maximum, false),
            left: vec![0.0; 128 * W],
            right: vec![0.0; 128 * W],
            reports: vec![ProcessReport::default(); W],
            samples: Vec::with_capacity(OBSERVATIONS),
        }
    }

    /// Renders one block, timing only the render itself.
    fn observe(&mut self, block: usize, record: bool) {
        traffic(&mut self.instance, self.arm, block);
        for frame in 0..128 {
            for track in 0..W {
                let sample = block * 128 + frame;
                self.left[frame * W + track] = stimulus(sample, track, 0);
                self.right[frame * W + track] = stimulus(sample, track, 1);
            }
        }
        let start = std::time::Instant::now();
        if self.split {
            render::<L, W, false>(
                &mut self.instance,
                &mut self.left,
                &mut self.right,
                128,
                &mut self.reports,
            );
        } else {
            render::<L, W, true>(
                &mut self.instance,
                &mut self.left,
                &mut self.right,
                128,
                &mut self.reports,
            );
        }
        let elapsed = start.elapsed().as_secs_f64() * 1.0e9;
        if record {
            self.samples.push(elapsed);
        }
    }
}

/// Measures one width: three arms, split on and off, alternated per observation.
fn measure<L: Lane, const W: usize>(width: &str) {
    const WARMUP: usize = 64;
    for round in 0..3 {
        let mut subjects: Vec<Subject<L, W>> = Vec::new();
        for arm in [Arm::Quiet, Arm::Restated, Arm::Moving] {
            for split in [true, false] {
                subjects.push(Subject::new(arm, split));
            }
        }
        for block in 0..WARMUP {
            for subject in subjects.iter_mut() {
                subject.observe(block, false);
            }
        }
        for observation in 0..OBSERVATIONS {
            for subject in subjects.iter_mut() {
                subject.observe(WARMUP + observation, true);
            }
        }
        if round == 0 {
            continue;
        }
        for pair in subjects.chunks(2) {
            let (on, off) = (&pair[0], &pair[1]);
            assert!(on.split && !off.split);
            let mut paired: Vec<f64> = off
                .samples
                .iter()
                .zip(on.samples.iter())
                .map(|(without, with)| without - with)
                .collect();
            let mut with: Vec<f64> = on.samples.clone();
            let mut without: Vec<f64> = off.samples.clone();
            for series in [&mut paired, &mut with, &mut without] {
                series.sort_by(|a, b| a.partial_cmp(b).expect("no NaN in a timing series"));
            }
            let frames = (128 * W) as f64;
            println!(
                "{width} round {round}  {}  split off {:8.0} ns  split on {:8.0} ns  \
                 paired delta p50 {:+7.0} ns/block ({:+6.2}%, {:+5.2} ns/frame/track)",
                on.arm.label(),
                percentile(&without, 0.5),
                percentile(&with, 0.5),
                percentile(&paired, 0.5),
                100.0 * percentile(&paired, 0.5) / percentile(&without, 0.5),
                percentile(&paired, 0.5) / frames,
            );
        }
    }
}

/// The descriptive cost of the ramping split. Not a gate (AGENTS.md); run with `--ignored`.
#[test]
#[ignore = "descriptive measurement, not a gate"]
fn the_split_costs_what_it_costs() {
    println!("multiband ramping split, paired alternation, {OBSERVATIONS} observations/round");
    measure::<f32, 1>("scalar");
    measure::<Simd4, 4>("simd4 ");
    measure::<Simd8, 8>("simd8 ");
}
