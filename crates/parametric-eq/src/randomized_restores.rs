//! Issue #1051: the stationary elision against the full cascade, across random restores.
//!
//! #1015 was a restored subnormal integrator in a live section: the stationary cascade admitted
//! it, elided the dead identity sections after it, and passed on a `-0.0` the executed identity
//! would have written as `+0.0`. No differential drew restored state then, so only its own
//! reproducers (`stationary_subnormal`) caught it. This one does, as a class rather than a case:
//!
//! * random legal targets per lane and channel, drawn from the descriptor's domains;
//! * random restores through [`Channel::restore_track`], with integrator words rewritten to the
//!   edge values around the elision's gates -- both zeros, subnormals of both signs, either side
//!   of `FLUSH_EPS`, tiny and ordinary normals -- on one or both channels;
//! * random ramps started and discontinuity resets, and random legal input, subnormal and
//!   signed-zero words included;
//! * every block rendered twice: the shipped decision (`stationary` when no ramp is in flight)
//!   and the full per-section cascade (`process_channels(.., false)`), dual and collapsed, at
//!   `f32`, `Simd4` and `Simd8`. Output words and integrator words are compared by bits, signed
//!   zeros included: that sign is the whole of #1015.
//!
//! Every value here is finite (a restore refuses anything else), so NaN encodings never arise.

use super::*;
use dsp_reference::randomized::{Draw, Profile, run_seeds};

const TEST: &str =
    "randomized_restores::the_stationary_cascade_renders_the_full_cascade_after_random_restores";
const REPLAY: &str = "cargo test -p parametric-eq --lib -- --exact \
                      randomized_restores::the_stationary_cascade_renders_the_full_cascade_after_random_restores";

/// Integrator words around the elision's gates.
fn integrator_word(draw: &mut Draw) -> f32 {
    let floor = f32::from_bits(INERT_MAGNITUDE_FLOOR);
    let value = match draw.below(10) {
        0 => 0.0,
        1 => -0.0,
        2 => draw.subnormal(),
        3 => f32::from_bits(1),
        4 => floor,
        5 => f32::from_bits(INERT_MAGNITUDE_FLOOR - 1),
        6 => draw.pick(&[1.0e-30_f32, 1.5e-20, 1.0e-12]),
        7 => draw.noise(0.5),
        8 => f32::from_bits(0x8000_0001),
        _ => draw.noise(1.0e-3),
    };
    if draw.chance(1, 2) { value } else { -value }
}

/// A legal value of `parameter`, favouring the domain edges.
fn value(draw: &mut Draw, parameter: &effect_contract::ParameterDescriptor) -> f32 {
    use effect_contract::ParameterDomain;
    let value = match parameter.domain {
        ParameterDomain::Boolean => draw.pick(&[0.0_f32, 1.0]),
        ParameterDomain::Enumeration => draw.pick(parameter.enum_choices).value,
        ParameterDomain::Continuous => draw.in_domain(
            parameter.minimum.unwrap_or(parameter.default_value),
            parameter.maximum.unwrap_or(parameter.default_value),
        ),
    };
    if effect_contract::parameter_value_valid(parameter, value) {
        value
    } else {
        parameter.default_value
    }
}

/// One track's two channels' targets, drawn until they design.
fn targets(draw: &mut Draw, rate: SampleRateHz) -> [[BandTarget; EQ_SECTION_COUNT]; 2] {
    for _ in 0..64 {
        let mut values: Vec<InitialParameterValue> =
            effect_contract::default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect();
        for slot in &mut values {
            if draw.chance(2, 3) {
                slot.value = value(
                    draw,
                    &PARAMETRIC_EQ_DESCRIPTOR.parameters[slot.parameter_index as usize],
                );
            }
        }
        if let (Ok(mut left), Ok(mut right)) = (
            physical_targets(&values, 0, rate),
            physical_targets(&values, 1, rate),
        ) {
            // The gains whose linear factor is an exact half or double, where a section's
            // coefficients sit on the rounding ties of the smallest subnormals (#1015's shelf).
            for band in left.iter_mut().chain(right.iter_mut()) {
                if draw.chance(1, 4) {
                    band.gain = draw.pick(&[-6.020_6_f32, 6.020_6]);
                }
            }
            // Half the tracks run one live section, every other one dead: the shape the
            // stationary elision exists for, and #1015's.
            if draw.chance(1, 2) {
                let live = draw.below(EQ_SECTION_COUNT);
                for (section, (l, r)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
                    l.enabled = section == live;
                    r.enabled = section == live;
                }
                // A general band, half the time a shelf at a cut: coefficients of at most one
                // half, the other shape #1015 needed.
                if (1..EQ_SECTION_COUNT - 1).contains(&live) && draw.chance(1, 2) {
                    let kind = draw.pick(&[EqBandKind::HighShelf, EqBandKind::LowShelf]);
                    let gain = draw.pick(&[-6.020_6_f32, -24.0, -12.0]);
                    for band in [&mut left[live], &mut right[live]] {
                        band.kind = kind;
                        band.gain = gain;
                    }
                }
            }
            if (0..EQ_SECTION_COUNT).all(|section| {
                left[section].words(rate).is_ok() && right[section].words(rate).is_ok()
            }) {
                return [left, right];
            }
        }
    }
    let values: Vec<InitialParameterValue> =
        effect_contract::default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect();
    [
        physical_targets(&values, 0, rate).expect("the defaults"),
        physical_targets(&values, 1, rate).expect("the defaults"),
    ]
}

/// What one scenario reached.
#[derive(Default)]
struct Reach {
    /// Blocks the shipped decision rendered stationary after a restore with a non-zero integrator.
    stationary_after_restore: u64,
    restores: u64,
    tiny_cases: u64,
}

fn frame_input(draw: &mut Draw, profile: Profile) -> f32 {
    draw.sample(profile)
}

#[allow(clippy::too_many_lines)]
fn scenario<L: Lane, const W: usize>(seed: u64, mono: bool, reach: &mut Reach) {
    let mut draw = Draw::new(seed ^ ((W as u64) << 32) ^ (u64::from(mono) << 40));
    let rate = SampleRateHz(draw.pick(&[44_100, 48_000, 88_200, 96_000]));
    let tracks: Vec<[[BandTarget; EQ_SECTION_COUNT]; 2]> =
        (0..W).map(|_| targets(&mut draw, rate)).collect();
    let side = |channel: usize| -> [[BandTarget; EQ_SECTION_COUNT]; W] {
        core::array::from_fn(|track| tracks[track][if mono { 0 } else { channel }])
    };
    let build = |channel: usize| Channel::<L, W>::new(side(channel), rate).expect("drawn legal");
    // Arm 0 renders the shipped decision, arm 1 the full per-section cascade.
    let mut arms = [(build(0), build(1)), (build(0), build(1))];
    let mut restored_live = false;
    // A restore that set a live section's integrators to one tiny word `t` makes the next block's
    // first frame on that lane `t` too: #1015's reproducer, as a draw.
    let mut mirror: Option<(usize, f32)> = None;
    for block in 0..16 {
        let context = format!("W{W} seed {seed} mono {mono} block {block}");
        // --- Boundary operations, identical on both arms ------------------------------------
        if draw.chance(1, 3) {
            // Tracks with one live section first: there a restored live integrator reaches the
            // output through elided identities alone.
            let sparse: Vec<usize> = (0..W)
                .filter(|track| {
                    arms[0].0.targets[*track]
                        .iter()
                        .filter(|band| band.enabled)
                        .count()
                        == 1
                })
                .collect();
            let track = if !sparse.is_empty() && draw.chance(2, 3) {
                draw.pick(&sparse)
            } else {
                draw.below(W)
            };
            let channels: &[usize] = if mono {
                &[0]
            } else {
                draw.pick(&[&[0_usize][..], &[1], &[0, 1]])
            };
            for &channel in channels {
                let mut words = [0_u32; STATE_LANE_WORDS];
                let source = if channel == 0 { &arms[0].0 } else { &arms[0].1 };
                source.snapshot_track(track, &mut words);
                for _ in 0..=draw.below(3) {
                    // Live sections are where #1015 lived; both integrators get one word half the
                    // time, the shape of its reproducer.
                    let live: Vec<usize> = (0..EQ_SECTION_COUNT)
                        .filter(|section| source.targets[track][*section].enabled)
                        .collect();
                    let section = if !live.is_empty() && draw.chance(2, 3) {
                        draw.pick(&live)
                    } else {
                        draw.below(EQ_SECTION_COUNT)
                    };
                    let tiny = draw.chance(1, 2);
                    let word = if tiny {
                        // `-2^-149` half the time: under a coefficient of at most one half its
                        // product is `-0.0`, the one word that tells an elided identity from an
                        // executed one.
                        f32::from_bits(draw.pick(&[
                            0x8000_0001_u32,
                            0x8000_0001,
                            0x8000_0001,
                            0x8000_0002,
                            0x8000_0003,
                            1,
                            2,
                            3,
                        ]))
                    } else {
                        integrator_word(&mut draw)
                    };
                    if tiny {
                        mirror = Some((track, word));
                    }
                    let integrators: &[usize] = if tiny || draw.chance(1, 2) {
                        &[0, 1]
                    } else {
                        draw.pick(&[&[0_usize][..], &[1]])
                    };
                    for &integrator in integrators {
                        words[section * STATE_WORDS_PER_BAND + integrator] = word.to_bits();
                    }
                    restored_live |= word != 0.0;
                }
                let configuration = source.targets[track];
                let results: Vec<_> = arms
                    .iter_mut()
                    .map(|arm| {
                        let target = if channel == 0 { &mut arm.0 } else { &mut arm.1 };
                        target
                            .restore_track(track, &words, &configuration, rate)
                            .map_err(|error| error.code)
                    })
                    .collect();
                // Both arms hold the same payload, so they accept or refuse it together. A refusal
                // is legal here: the restore validates an in-flight ramp's every future step, and
                // that check refuses some ramps the channel itself is running (see the #1051
                // evidence), so a mid-ramp snapshot is not always restorable.
                assert_eq!(
                    results[0], results[1],
                    "{context}: the arms' restores disagree"
                );
                if results[0].is_ok() {
                    reach.restores += 1;
                }
            }
        }
        if draw.chance(1, 6) {
            // A retarget keeps the band's enable and family (they are not automatable) and moves
            // its numeric fields, the way a prepared target does.
            let (section, track) = (draw.below(EQ_SECTION_COUNT), draw.below(W));
            let drawn = targets(&mut draw, rate)[0][section].numeric();
            let channels = if mono { 1 } else { 1 + draw.below(3) };
            for (bit, index) in [(1, 0), (2, 1)] {
                if channels & bit == 0 {
                    continue;
                }
                let mut band =
                    if index == 0 { &arms[0].0 } else { &arms[0].1 }.targets[track][section];
                for (field, value) in drawn.into_iter().enumerate() {
                    band.set_numeric(field, value);
                }
                let Ok(words) = band.words(rate) else {
                    continue;
                };
                for arm in &mut arms {
                    let channel = if index == 0 { &mut arm.0 } else { &mut arm.1 };
                    channel.apply_prepared_target(track, section, band, words);
                }
            }
        }
        if draw.chance(1, 16) {
            for arm in &mut arms {
                arm.0.discontinuity_reset();
                arm.1.discontinuity_reset();
            }
        }
        // --- Input ----------------------------------------------------------------------------
        let frames = draw.frames(128);
        let words = frames * W;
        let mut left = vec![0.0_f32; words];
        let mut right = vec![0.0_f32; words];
        for lane in 0..W {
            let profile = loop {
                let profile = draw.profile();
                if profile.is_finite_legal() {
                    break profile;
                }
            };
            for frame in 0..frames {
                left[frame * W + lane] = match mirror {
                    Some((track, word)) if frame == 0 && track == lane => word,
                    _ => frame_input(&mut draw, profile),
                };
                right[frame * W + lane] = if mono || draw.chance(1, 2) || frame == 0 {
                    left[frame * W + lane]
                } else {
                    frame_input(&mut draw, profile)
                };
            }
        }
        mirror = None;
        // --- Render both arms -------------------------------------------------------------------
        let mut rendered = Vec::with_capacity(2);
        let mut stationary_now = false;
        for (index, arm) in arms.iter_mut().enumerate() {
            let (mut l, mut r) = (left.clone(), right.clone());
            if mono {
                let stationary = index == 0 && arm.0.no_ramp_in_flight();
                stationary_now |= stationary;
                process_channels_mono::<L, W>(&mut arm.0, &mut l, frames, stationary);
            } else {
                let stationary =
                    index == 0 && arm.0.no_ramp_in_flight() && arm.1.no_ramp_in_flight();
                stationary_now |= stationary;
                process_channels::<L, W>(
                    (&mut arm.0, &mut arm.1),
                    &mut l,
                    &mut r,
                    frames,
                    stationary,
                );
            }
            let mut state = [[0_u32; EQ_SECTION_COUNT * 2 * MAX_LANES]; 2];
            arm.0.state_bits(&mut state[0]);
            arm.1.state_bits(&mut state[1]);
            rendered.push((l, r, state));
        }
        if stationary_now && restored_live {
            reach.stationary_after_restore += 1;
        }
        let (shipped, full) = (&rendered[0], &rendered[1]);
        for (plane, a, b) in [
            ("left", &shipped.0, &full.0),
            ("right", &shipped.1, &full.1),
        ] {
            if mono && plane == "right" {
                continue;
            }
            if let Some(word) = (0..words).find(|&word| a[word].to_bits() != b[word].to_bits()) {
                panic!(
                    "{context}: {plane} frame {} lane {}: the shipped cascade rendered {:#010x} \
                     where the full cascade rendered {:#010x}",
                    word / W,
                    word % W,
                    a[word].to_bits(),
                    b[word].to_bits()
                );
            }
        }
        assert!(
            shipped.2[0] == full.2[0] && (mono || shipped.2[1] == full.2[1]),
            "{context}: the shipped cascade left other integrator words than the full cascade"
        );
    }
}

/// The dense half: many one-block cases of #1015's shape, each cheap. Every track runs one live
/// section (often a cut shelf, whose coefficients are at most one half), both of that section's
/// integrators are restored to one tiny word, and the block's first frame on that track is the
/// same word. Whether the live section then emits `-0.0` depends on its design, so the case is
/// drawn many times over rates, families, frequencies and Q.
fn tiny_state_cases<L: Lane, const W: usize>(seed: u64, mono: bool, reach: &mut Reach) {
    let mut draw = Draw::new(seed ^ 0x1015_0000 ^ ((W as u64) << 32) ^ (u64::from(mono) << 40));
    for case in 0..24 {
        let context = format!("W{W} seed {seed} mono {mono} tiny case {case}");
        let rate = SampleRateHz(draw.pick(&[44_100, 48_000, 88_200, 96_000]));
        let tracks: Vec<[[BandTarget; EQ_SECTION_COUNT]; 2]> = (0..W)
            .map(|_| {
                let mut track = targets(&mut draw, rate);
                let live = 1 + draw.below(EQ_SECTION_COUNT - 2);
                let kind = draw.pick(&[
                    EqBandKind::HighShelf,
                    EqBandKind::LowShelf,
                    EqBandKind::HighShelf,
                    EqBandKind::Bell,
                    EqBandKind::Notch,
                ]);
                let gain = draw.pick(&[-6.020_6_f32, -24.0, -12.0, -9.0, 6.020_6]);
                for channel in &mut track {
                    for (section, band) in channel.iter_mut().enumerate() {
                        band.enabled = section == live;
                    }
                    channel[live].kind = kind;
                    channel[live].gain = gain;
                }
                if (0..EQ_SECTION_COUNT).all(|section| track[0][section].words(rate).is_ok()) {
                    track
                } else {
                    targets(&mut draw, rate)
                }
            })
            .collect();
        let side = |channel: usize| -> [[BandTarget; EQ_SECTION_COUNT]; W] {
            core::array::from_fn(|track| tracks[track][if mono { 0 } else { channel }])
        };
        let build =
            |channel: usize| Channel::<L, W>::new(side(channel), rate).expect("drawn legal");
        let mut arms = [(build(0), build(1)), (build(0), build(1))];
        let frames = 1 + draw.below(16);
        let mut left = vec![0.0_f32; frames * W];
        let mut right = vec![0.0_f32; frames * W];
        for track in 0..W {
            let word = f32::from_bits(draw.pick(&[
                0x8000_0001_u32,
                0x8000_0001,
                0x8000_0002,
                0x8000_0003,
                1,
                2,
                INERT_MAGNITUDE_FLOOR - 1,
                INERT_MAGNITUDE_FLOOR | 0x8000_0000,
            ]));
            for channel in if mono { 0..1 } else { 0..2 } {
                let mut words = [0_u32; STATE_LANE_WORDS];
                let source = if channel == 0 { &arms[0].0 } else { &arms[0].1 };
                source.snapshot_track(track, &mut words);
                let configuration = source.targets[track];
                for section in 0..EQ_SECTION_COUNT {
                    if configuration[section].enabled {
                        words[section * STATE_WORDS_PER_BAND] = word.to_bits();
                        words[section * STATE_WORDS_PER_BAND + 1] = word.to_bits();
                    }
                }
                for arm in &mut arms {
                    let target = if channel == 0 { &mut arm.0 } else { &mut arm.1 };
                    target
                        .restore_track(track, &words, &configuration, rate)
                        .expect("a finite integrator restores on a settled track");
                }
            }
            for frame in 0..frames {
                let sample = if frame == 0 {
                    word
                } else {
                    draw.sample(Profile::Subnormal)
                };
                left[frame * W + track] = sample;
                right[frame * W + track] = sample;
            }
        }
        reach.tiny_cases += 1;
        let mut rendered = Vec::with_capacity(2);
        for (index, arm) in arms.iter_mut().enumerate() {
            let (mut l, mut r) = (left.clone(), right.clone());
            if mono {
                let stationary = index == 0 && arm.0.no_ramp_in_flight();
                process_channels_mono::<L, W>(&mut arm.0, &mut l, frames, stationary);
            } else {
                let stationary =
                    index == 0 && arm.0.no_ramp_in_flight() && arm.1.no_ramp_in_flight();
                process_channels::<L, W>(
                    (&mut arm.0, &mut arm.1),
                    &mut l,
                    &mut r,
                    frames,
                    stationary,
                );
            }
            rendered.push((l, r));
        }
        let planes = if mono { 1 } else { 2 };
        for (plane, (a, b)) in [
            (&rendered[0].0, &rendered[1].0),
            (&rendered[0].1, &rendered[1].1),
        ]
        .into_iter()
        .enumerate()
        .take(planes)
        {
            if let Some(word) = (0..frames * W).find(|&word| a[word].to_bits() != b[word].to_bits())
            {
                panic!(
                    "{context}: plane {plane} frame {} lane {}: the shipped cascade rendered \
                     {:#010x} where the full cascade rendered {:#010x} ({:?})",
                    word / W,
                    word % W,
                    a[word].to_bits(),
                    b[word].to_bits(),
                    tracks[word % W][0]
                        .iter()
                        .filter(|band| band.enabled)
                        .collect::<Vec<_>>()
                );
            }
        }
    }
}

#[test]
fn the_stationary_cascade_renders_the_full_cascade_after_random_restores() {
    let mut reach = Reach::default();
    run_seeds(TEST, REPLAY, 8, |seed| {
        for mono in [false, true] {
            scenario::<f32, 1>(seed, mono, &mut reach);
            scenario::<Simd4, 4>(seed, mono, &mut reach);
            scenario::<Simd8, 8>(seed, mono, &mut reach);
            tiny_state_cases::<f32, 1>(seed, mono, &mut reach);
            tiny_state_cases::<Simd4, 4>(seed, mono, &mut reach);
            tiny_state_cases::<Simd8, 8>(seed, mono, &mut reach);
        }
    });
    if dsp_reference::randomized::replaying() {
        return;
    }
    assert!(
        reach.restores > 0 && reach.stationary_after_restore > 0 && reach.tiny_cases > 0,
        "the stationary cascade must run after restores of live integrator words"
    );
}
