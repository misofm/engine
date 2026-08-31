//! Independent `f64` curve, transition and whole-signal primitives for the launch gate/expander.
//!
//! This small oracle deliberately owns no production types. It is a direct transcription of the
//! frozen mathematical contract for focused conformance tests, not a realtime processor.

/// One f64 gate/expander curve input.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceGateExpanderParameters {
    /// Threshold in dB.
    pub threshold_db: f64,
    /// Downward-expansion ratio.
    pub ratio: f64,
    /// Maximum attenuation in dB.
    pub range_db: f64,
}

/// The externally observable hysteresis state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceGatePhase {
    /// The gate is attenuating according to the expansion curve.
    Closed,
    /// The gate is fully open.
    Open,
}

/// The input is outside the frozen launch domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceGateExpanderError {
    /// A curve parameter or level was non-finite or invalid.
    InvalidInput,
}

/// Computes the frozen static attenuation target in dB.
///
/// The detector level is expressed in dB after its `[-160, 24]` clamp. An open gate is exact
/// unity; a closed gate applies the hard downward-expansion curve and the explicit range cap.
pub fn reference_gate_expander_gain_reduction_db(
    level_db: f64,
    parameters: ReferenceGateExpanderParameters,
    phase: ReferenceGatePhase,
) -> Result<f64, ReferenceGateExpanderError> {
    if !level_db.is_finite()
        || !(-160.0..=24.0).contains(&level_db)
        || !parameters.threshold_db.is_finite()
        || !(-80.0..=0.0).contains(&parameters.threshold_db)
        || !parameters.ratio.is_finite()
        || !(1.0..=20.0).contains(&parameters.ratio)
        || !parameters.range_db.is_finite()
        || !(0.0..=96.0).contains(&parameters.range_db)
    {
        return Err(ReferenceGateExpanderError::InvalidInput);
    }
    if phase == ReferenceGatePhase::Open {
        return Ok(0.0);
    }
    Ok(
        ((parameters.ratio - 1.0) * (level_db - parameters.threshold_db))
            .clamp(-parameters.range_db, 0.0),
    )
}

/// Level below which the detector's logarithm is floored, mirroring the production kernel.
const LEVEL_FLOOR: f64 = 1.0e-8;

/// Hysteresis band in dB, and the timing that drives one channel's detector.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceGateTiming {
    /// Render sample rate in hertz.
    pub sample_rate: u32,
    /// Attack time constant in milliseconds.
    pub attack_ms: f64,
    /// Hold time in milliseconds.
    pub hold_ms: f64,
    /// Release time constant in milliseconds.
    pub release_ms: f64,
    /// Lookahead in milliseconds, at most the fixed latency.
    pub lookahead_ms: f64,
}

/// How the two channels' detectors are linked.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceGateLink {
    /// Each channel detects its own source.
    DualMono,
    /// Both channels detect the larger of the two magnitudes.
    Maximum,
    /// Both channels detect the mean of the two magnitudes.
    Average,
}

/// Everything one `f64` render produces, per channel and per sample.
#[derive(Clone, Debug, PartialEq)]
pub struct ReferenceGateTrace {
    /// Left output samples.
    pub out_left: Vec<f64>,
    /// Right output samples.
    pub out_right: Vec<f64>,
    /// Left smoothed gain reduction in dB.
    pub gain_db_left: Vec<f64>,
    /// Right smoothed gain reduction in dB.
    pub gain_db_right: Vec<f64>,
    /// Left detector level in dB after the `[-160, 24]` clamp.
    pub level_db_left: Vec<f64>,
    /// Right detector level in dB after the `[-160, 24]` clamp.
    pub level_db_right: Vec<f64>,
    /// Left hysteresis phase after the transition of that sample.
    pub phase_left: Vec<ReferenceGatePhase>,
    /// Right hysteresis phase after the transition of that sample.
    pub phase_right: Vec<ReferenceGatePhase>,
    /// Left delayed dry sample, the identity output.
    pub dry_left: Vec<f64>,
    /// Right delayed dry sample, the identity output.
    pub dry_right: Vec<f64>,
}

/// `floor(x + 0.5)`, the rounding the production preparation uses for sample counts.
fn round_half_up(value: f64) -> f64 {
    (value + 0.5).floor()
}

/// One channel's derived, sample-domain timing.
#[derive(Clone, Copy)]
struct Derived {
    detector_delay: usize,
    hold_samples: u64,
    attack_rate: f64,
    release_rate: f64,
}

fn derive(
    timing: ReferenceGateTiming,
    latency: usize,
) -> Result<Derived, ReferenceGateExpanderError> {
    if timing.sample_rate == 0
        || !timing.attack_ms.is_finite()
        || !timing.hold_ms.is_finite()
        || !timing.release_ms.is_finite()
        || !timing.lookahead_ms.is_finite()
        || timing.attack_ms <= 0.0
        || timing.release_ms <= 0.0
        || timing.hold_ms < 0.0
        || timing.lookahead_ms < 0.0
    {
        return Err(ReferenceGateExpanderError::InvalidInput);
    }
    let rate = f64::from(timing.sample_rate);
    let lookahead = round_half_up(timing.lookahead_ms * rate / 1000.0);
    if lookahead < 0.0 || lookahead > usize::MAX as f64 {
        return Err(ReferenceGateExpanderError::InvalidInput);
    }
    let lookahead = (lookahead as usize).min(latency);
    let hold = round_half_up(timing.hold_ms * rate / 1000.0);
    if hold < 0.0 || hold > u64::MAX as f64 {
        return Err(ReferenceGateExpanderError::InvalidInput);
    }
    Ok(Derived {
        detector_delay: latency - lookahead,
        hold_samples: hold as u64,
        // `1 - exp(-1 / (tau * fs))`, the per-sample rate coefficient of `G += b * (C - G)`.
        attack_rate: 1.0 - (-1000.0 / (timing.attack_ms * rate)).exp(),
        release_rate: 1.0 - (-1000.0 / (timing.release_ms * rate)).exp(),
    })
}

/// Per-channel mutable model state.
struct ChannelState {
    gain_db: f64,
    open: bool,
    hold_remaining: u64,
}

/// Renders a whole signal through the `f64` transcription of the gate/expander.
///
/// A full transcription of brief 014's gain computer as amended by #89: the `log2`/`exp2`
/// realisation of the dB conversions, the single-rounding `G + b * (C - G)` one-pole and the
/// `1e-20` flush band. Hold counters and ring taps are integers and the rings are plain vectors,
/// so nothing about the production kernel's layout can leak into the oracle.
///
/// The fixed latency is `sample_rate / 100`, as the descriptor's qualities pin it. Parameters are
/// static for the whole render: ramps belong to the control plane and are not modelled.
///
/// # Errors
///
/// [`ReferenceGateExpanderError::InvalidInput`] if a parameter, a timing or a sample is outside
/// the frozen launch domain, or if the two channels differ in length.
pub fn reference_gate_expander_process(
    params_left: ReferenceGateExpanderParameters,
    params_right: ReferenceGateExpanderParameters,
    hysteresis_db: (f64, f64),
    timing: (ReferenceGateTiming, ReferenceGateTiming),
    link: ReferenceGateLink,
    main: (&[f64], &[f64]),
    sidechain: Option<(&[f64], &[f64])>,
) -> Result<ReferenceGateTrace, ReferenceGateExpanderError> {
    let frames = main.0.len();
    if main.1.len() != frames
        || timing.0.sample_rate != timing.1.sample_rate
        || sidechain.is_some_and(|(l, r)| l.len() != frames || r.len() != frames)
    {
        return Err(ReferenceGateExpanderError::InvalidInput);
    }
    for value in main.0.iter().chain(main.1) {
        if !value.is_finite() {
            return Err(ReferenceGateExpanderError::InvalidInput);
        }
    }
    if let Some((left, right)) = sidechain {
        for value in left.iter().chain(right) {
            if !value.is_finite() {
                return Err(ReferenceGateExpanderError::InvalidInput);
            }
        }
    }
    for (hysteresis, parameters) in [hysteresis_db.0, hysteresis_db.1]
        .into_iter()
        .zip([params_left, params_right])
    {
        if !hysteresis.is_finite()
            || !(0.0..=24.0).contains(&hysteresis)
            || reference_gate_expander_gain_reduction_db(
                0.0,
                parameters,
                ReferenceGatePhase::Closed,
            )
            .is_err()
        {
            return Err(ReferenceGateExpanderError::InvalidInput);
        }
    }
    let latency = timing.0.sample_rate as usize / 100;
    let derived = (derive(timing.0, latency)?, derive(timing.1, latency)?);

    let mut state = [
        ChannelState {
            gain_db: 0.0,
            open: true,
            hold_remaining: derived.0.hold_samples,
        },
        ChannelState {
            gain_db: 0.0,
            open: true,
            hold_remaining: derived.1.hold_samples,
        },
    ];
    let detector_source = sidechain.unwrap_or(main);
    let mut trace = ReferenceGateTrace {
        out_left: Vec::with_capacity(frames),
        out_right: Vec::with_capacity(frames),
        gain_db_left: Vec::with_capacity(frames),
        gain_db_right: Vec::with_capacity(frames),
        level_db_left: Vec::with_capacity(frames),
        level_db_right: Vec::with_capacity(frames),
        phase_left: Vec::with_capacity(frames),
        phase_right: Vec::with_capacity(frames),
        dry_left: Vec::with_capacity(frames),
        dry_right: Vec::with_capacity(frames),
    };
    let at = |signal: &[f64], index: isize| -> f64 {
        if index < 0 {
            0.0
        } else {
            signal[index as usize]
        }
    };

    for frame in 0..frames {
        let index = frame as isize;
        let dry = (
            at(main.0, index - latency as isize),
            at(main.1, index - latency as isize),
        );
        for channel in 0..2 {
            let (parameters, hysteresis, derived, partner) = if channel == 0 {
                (params_left, hysteresis_db.0, derived.0, 1)
            } else {
                (params_right, hysteresis_db.1, derived.1, 0)
            };
            let tap = index - derived.detector_delay as isize;
            let sources = [at(detector_source.0, tap), at(detector_source.1, tap)];
            let own = sources[channel].abs();
            let other = sources[partner].abs();
            let level = match link {
                ReferenceGateLink::DualMono => own,
                ReferenceGateLink::Maximum => own.max(other),
                ReferenceGateLink::Average => 0.5 * own + 0.5 * other,
            };
            let level_db = (20.0 * level.max(LEVEL_FLOOR).log10()).clamp(-160.0, 24.0);

            // Transition, in the frozen order of brief 014: an opening trigger reloads the hold,
            // a re-arm inside the hysteresis band reloads it too, and only an expired hold closes.
            let slot = &mut state[channel];
            let threshold = parameters.threshold_db;
            if !slot.open {
                if level_db >= threshold {
                    slot.open = true;
                    slot.hold_remaining = derived.hold_samples;
                }
            } else if level_db >= threshold - hysteresis {
                slot.hold_remaining = derived.hold_samples;
            } else if slot.hold_remaining != 0 {
                slot.hold_remaining -= 1;
            } else {
                slot.open = false;
            }
            let phase = if slot.open {
                ReferenceGatePhase::Open
            } else {
                ReferenceGatePhase::Closed
            };
            let target = reference_gate_expander_gain_reduction_db(level_db, parameters, phase)?;
            let rate = if target > slot.gain_db {
                derived.attack_rate
            } else {
                derived.release_rate
            };
            let mut gain_db = slot.gain_db + rate * (target - slot.gain_db);
            if gain_db.abs() < 1.0e-20 {
                gain_db = 0.0;
            }
            slot.gain_db = gain_db;
            let z = if channel == 0 { dry.0 } else { dry.1 };
            let out = if gain_db == 0.0 {
                z
            } else {
                z * 10.0_f64.powf(gain_db / 20.0)
            };
            if channel == 0 {
                trace.level_db_left.push(level_db);
                trace.phase_left.push(phase);
                trace.gain_db_left.push(gain_db);
                trace.dry_left.push(z);
                trace.out_left.push(out);
            } else {
                trace.level_db_right.push(level_db);
                trace.phase_right.push(phase);
                trace.gain_db_right.push(gain_db);
                trace.dry_right.push(z);
                trace.out_right.push(out);
            }
        }
    }
    Ok(trace)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn timing(lookahead_ms: f64, hold_ms: f64) -> ReferenceGateTiming {
        ReferenceGateTiming {
            sample_rate: 48_000,
            attack_ms: 1.0,
            hold_ms,
            release_ms: 5.0,
            lookahead_ms,
        }
    }

    #[test]
    fn curve_is_identity_at_ratio_one_and_range_limited_when_closed() {
        let identity = ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: 1.0,
            range_db: 80.0,
        };
        assert_eq!(
            reference_gate_expander_gain_reduction_db(-80.0, identity, ReferenceGatePhase::Closed),
            Ok(0.0)
        );
        let limited = ReferenceGateExpanderParameters {
            ratio: 20.0,
            range_db: 12.0,
            ..identity
        };
        assert_eq!(
            reference_gate_expander_gain_reduction_db(-80.0, limited, ReferenceGatePhase::Closed),
            Ok(-12.0)
        );
    }

    #[test]
    fn hand_computed_curve_point_is_range_clamped() {
        let parameters = ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: 4.0,
            range_db: 48.0,
        };
        // (rho - 1) * (X - T) = 3 * (-60 + 40) = -60, clamped by the -48 dB range.
        assert_eq!(
            reference_gate_expander_gain_reduction_db(
                -60.0,
                parameters,
                ReferenceGatePhase::Closed
            ),
            Ok(-48.0)
        );
    }

    #[test]
    fn hold_expiry_closes_exactly_one_sample_after_the_countdown_reaches_zero() {
        // Two-sample hold, and a full 10 ms lookahead so the detector tap is the current sample.
        // The level is above the threshold for one sample, then silent: the gate stays open for
        // the two held samples and closes on the third silent one.
        let parameters = ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: 4.0,
            range_db: 48.0,
        };
        let rate = 48_000_u32;
        let latency = rate as usize / 100;
        let frames = latency + 8;
        let mut left = vec![0.0_f64; frames];
        left[0] = 1.0;
        let right = vec![0.0_f64; frames];
        let hold_ms = 2.0 * 1000.0 / f64::from(rate);
        let trace = reference_gate_expander_process(
            parameters,
            parameters,
            (6.0, 6.0),
            (timing(10.0, hold_ms), timing(10.0, hold_ms)),
            ReferenceGateLink::DualMono,
            (&left, &right),
            None,
        )
        .expect("reference render");
        assert_eq!(trace.phase_left[0], ReferenceGatePhase::Open, "trigger");
        assert_eq!(trace.phase_left[1], ReferenceGatePhase::Open, "hold 2");
        assert_eq!(trace.phase_left[2], ReferenceGatePhase::Open, "hold 1");
        assert_eq!(trace.phase_left[3], ReferenceGatePhase::Closed, "expired");
        assert_eq!(trace.gain_db_left[0], 0.0, "an open gate is exact unity");
    }

    #[test]
    fn a_level_exactly_at_the_threshold_opens_a_closed_gate() {
        // `>=`, not `>`: brief 014 opens *at* the threshold. The level is built so that its dB
        // value is exactly the threshold, which is the only sample where the two differ.
        let parameters = ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: 4.0,
            range_db: 48.0,
        };
        let rate = 48_000_u32;
        let latency = rate as usize / 100;
        let amplitude = 10.0_f64.powf(-40.0 / 20.0);
        // Zero hold, so the gate is closed by the time the trigger arrives.
        let mut left = vec![0.0_f64; latency + 8];
        left[latency + 4] = amplitude;
        let right = vec![0.0_f64; latency + 8];
        let trace = reference_gate_expander_process(
            parameters,
            parameters,
            (6.0, 6.0),
            (timing(10.0, 0.0), timing(10.0, 0.0)),
            ReferenceGateLink::DualMono,
            (&left, &right),
            None,
        )
        .expect("reference render");
        assert_eq!(
            trace.level_db_left[latency + 4],
            -40.0,
            "the trigger sample sits exactly on the threshold"
        );
        assert_eq!(trace.phase_left[latency + 3], ReferenceGatePhase::Closed);
        assert_eq!(trace.phase_left[latency + 4], ReferenceGatePhase::Open);
    }

    #[test]
    fn a_level_inside_the_hysteresis_band_keeps_reloading_the_hold() {
        // The hold is partly spent, the level returns to the hysteresis band, and the band must
        // reload it in full. Without the in-band reload the gate would close from whatever was
        // left of the countdown when the band was entered.
        let parameters = ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: 4.0,
            range_db: 48.0,
        };
        let rate = 48_000_u32;
        let latency = rate as usize / 100;
        let hold_ms = 3.0 * 1000.0 / f64::from(rate);
        let band = 10.0_f64.powf(-43.0 / 20.0);
        let frames = latency + 32;
        let mut left = vec![0.0_f64; frames];
        left[0] = 1.0;
        // Frames 1 and 2 are silent, so the countdown falls to 1; frames 3 to 6 sit in the band.
        for sample in left.iter_mut().take(7).skip(3) {
            *sample = band;
        }
        let right = vec![0.0_f64; frames];
        let trace = reference_gate_expander_process(
            parameters,
            parameters,
            (6.0, 6.0),
            (timing(10.0, hold_ms), timing(10.0, hold_ms)),
            ReferenceGateLink::DualMono,
            (&left, &right),
            None,
        )
        .expect("reference render");
        for frame in 0..=9 {
            assert_eq!(
                trace.phase_left[frame],
                ReferenceGatePhase::Open,
                "the band reloaded the hold, so frame {frame} is still open"
            );
        }
        assert_eq!(trace.phase_left[10], ReferenceGatePhase::Closed, "expired");
    }

    #[test]
    fn the_dry_path_is_delayed_by_the_fixed_latency() {
        let parameters = ReferenceGateExpanderParameters {
            threshold_db: -80.0,
            ratio: 1.0,
            range_db: 0.0,
        };
        let latency = 480;
        let mut left = vec![0.0_f64; latency + 4];
        left[0] = 0.75;
        let right = vec![0.0_f64; latency + 4];
        let trace = reference_gate_expander_process(
            parameters,
            parameters,
            (6.0, 6.0),
            (timing(0.0, 0.0), timing(0.0, 0.0)),
            ReferenceGateLink::DualMono,
            (&left, &right),
            None,
        )
        .expect("reference render");
        assert_eq!(trace.dry_left[latency], 0.75);
        assert_eq!(trace.dry_left[latency - 1], 0.0);
    }

    #[test]
    fn a_non_finite_sample_is_rejected() {
        let parameters = ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: 4.0,
            range_db: 48.0,
        };
        let left = vec![f64::NAN; 4];
        let right = vec![0.0_f64; 4];
        assert_eq!(
            reference_gate_expander_process(
                parameters,
                parameters,
                (6.0, 6.0),
                (timing(0.0, 0.0), timing(0.0, 0.0)),
                ReferenceGateLink::DualMono,
                (&left, &right),
                None,
            ),
            Err(ReferenceGateExpanderError::InvalidInput)
        );
    }
}
