#![allow(clippy::disallowed_methods)] // D6 oracle/measurement exemption: compares against the platform deliberately (formerly check-math-policy.sh structural_exempt)
//! Gate 7.8: the frozen contract — descriptor, resources, latency, curve, transition, automation,
//! the connected sidechain and bank binding.
use lane::Backend;

mod support;

use dsp_reference::{
    ReferenceGateExpanderParameters, ReferenceGateLink, ReferenceGatePhase, ReferenceGateTiming,
    reference_gate_expander_gain_reduction_db, reference_gate_expander_process,
};
use effect_contract::{
    AutomationSpanKind, BankWidth, EffectProcessBlock, LatencySamples, LinkMode,
    NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PreparedAutomationSpan,
    PreparedSidechainPort, ProcessReport, TailSamples, validate_descriptor,
};
use gate_expander::{
    GATE_EXPANDER_DESCRIPTOR, GATE_EXPANDER_PARAMETERS, GateExpanderFactory, STATE_LAYOUT_VERSION,
};
use support::{
    Values, initial_values, prepare, render_scalar_sidechain, request, request_at_rate,
    set_parameter, sidechain_port, snapshot,
};

const RAMP_WORD: usize = 7;

fn word(payload: &[u8], index: usize) -> u32 {
    u32::from_le_bytes(payload[index * 4..index * 4 + 4].try_into().expect("word"))
}

fn float(payload: &[u8], index: usize) -> f32 {
    f32::from_bits(word(payload, index))
}

#[test]
fn descriptor_and_exact_resources_are_frozen() {
    validate_descriptor(&GATE_EXPANDER_DESCRIPTOR).expect("descriptor");
    assert_eq!(GATE_EXPANDER_DESCRIPTOR.id.as_str(), "miso.gate-expander");
    assert_eq!(GATE_EXPANDER_DESCRIPTOR.state_layout_version, 2);
    assert_eq!(STATE_LAYOUT_VERSION, 2);
    // Layout 2: per lane `(23 + 2N) * 4` bytes, plus the runtime codec's two-word common header.
    for (quality, (rate, latency, lane_bytes, total)) in
        GATE_EXPANDER_DESCRIPTOR.qualities.iter().zip([
            (44_100_u32, 441_u64, 3_620_u32, 7_248_u64),
            (48_000, 480, 3_932, 7_872),
            (88_200, 882, 7_148, 14_304),
            (96_000, 960, 7_772, 15_552),
        ])
    {
        assert_eq!(quality.sample_rate, rate);
        assert_eq!(quality.latency, LatencySamples(latency));
        assert_eq!(quality.tail, TailSamples::Finite(0));
        assert_eq!(quality.maximum_state.common_bytes, 8);
        assert_eq!(quality.maximum_state.left_bytes, lane_bytes);
        assert_eq!(quality.maximum_state.right_bytes, lane_bytes);
        assert_eq!(quality.maximum_state.total(), Some(total));
        assert_eq!(quality.scratch_fixed_bytes, 64);
        assert_eq!(quality.scratch_bytes_per_frame, 0);
        assert_eq!(
            lane_bytes,
            (23 + 2 * latency as u32) * 4,
            "the layout-2 arithmetic, spelled out"
        );
    }
}

#[test]
fn the_payload_header_names_the_layout() {
    let values = initial_values();
    let effect = prepare(request(&values));
    let (common, left, right) = snapshot(effect.as_ref());
    assert_eq!(common.len(), 8, "two header words");
    assert_eq!(word(&common, 0), 2, "the payload states its own version");
    assert_eq!(
        word(&common, 1) as usize,
        (left.len() + right.len()) / 4,
        "and its own data word count"
    );
}

#[test]
fn all_rate_caps_lookahead_and_fixed_latency_are_exact() {
    let factory = GateExpanderFactory;
    for quality in GATE_EXPANDER_DESCRIPTOR.qualities {
        let rate = quality.sample_rate;
        let latency = quality.latency.0 as usize;
        for lookahead in [0.0, 2.0, 10.0] {
            let mut values = initial_values();
            values[14].value = lookahead;
            values[15].value = lookahead;
            for bypass in [false, true] {
                let mut preparation = request_at_rate(&values, rate);
                preparation.bypass = bypass;
                let mut effect = factory.prepare(preparation).expect("exact cap prepares");
                assert_eq!(effect.metadata().latency, quality.latency);
                let mut left = vec![0.0; latency + 1];
                let mut right = vec![0.0; latency + 1];
                left[0] = -0.5;
                right[0] = 0.25;
                render_scalar_sidechain(effect.as_mut(), &mut left, &mut right, None, 128, &[], 0);
                assert!(left[..latency].iter().all(|sample| sample.to_bits() == 0));
                assert!(right[..latency].iter().all(|sample| sample.to_bits() == 0));
                assert_eq!(left[latency].to_bits(), (-0.5_f32).to_bits());
                assert_eq!(right[latency].to_bits(), 0.25_f32.to_bits());
            }
        }
        let values = initial_values();
        let mut below = request_at_rate(&values, rate);
        below.limits.maximum_total_state_bytes -= 1;
        assert_eq!(
            factory.prepare(below).err().expect("state cap").code,
            "effect.resource.limit"
        );
        let mut below_scratch = request_at_rate(&values, rate);
        below_scratch.limits.maximum_scratch_bytes -= 1;
        assert_eq!(
            factory
                .prepare(below_scratch)
                .err()
                .expect("scratch cap")
                .code,
            "effect.resource.limit"
        );
    }
}

#[test]
fn the_lookahead_tap_lands_the_detector_exactly_where_the_brief_says() {
    // The detector tap is `latency - lookahead`, so a gate with `L` samples of lookahead reacts to
    // a level change exactly `L` samples before that change reaches the delayed dry output. The
    // observable is the frame the gate stops being an exact identity: while it is open, `G` is
    // exactly `+0.0` and the output is the delayed input bit for bit; on the frame it closes, `G`
    // leaves zero and the output is scaled.
    const FRAMES: usize = 5_000;
    const DROP: usize = 4_000;
    const LATENCY: usize = 480;
    for (lookahead_ms, lookahead) in [(0.0_f32, 0_usize), (2.0, 96), (10.0, 480)] {
        let mut values = initial_values();
        values[0].value = -40.0; // threshold
        values[1].value = -40.0;
        values[10].value = 0.0; // hold 0 ms: the gate closes on the first sample below the band
        values[11].value = 0.0;
        values[14].value = lookahead_ms;
        values[15].value = lookahead_ms;
        let mut source = vec![0.001_f32; FRAMES];
        for sample in source.iter_mut().take(DROP) {
            *sample = 0.5;
        }
        let mut left = source.clone();
        let mut right = source.clone();
        let mut effect = prepare(request(&values));
        render_scalar_sidechain(effect.as_mut(), &mut left, &mut right, None, 128, &[], 0);

        let closes_at = DROP + LATENCY - lookahead;
        // Long before the drop the gate has been open for thousands of samples, so `G` has been
        // flushed to exactly zero and the dry path is bit-exact.
        for frame in 3_000..closes_at {
            assert_eq!(
                left[frame].to_bits(),
                source[frame - LATENCY].to_bits(),
                "lookahead {lookahead_ms} ms: frame {frame} must still be the exact identity"
            );
        }
        assert_ne!(
            left[closes_at].to_bits(),
            source[closes_at - LATENCY].to_bits(),
            "lookahead {lookahead_ms} ms: the gate closes exactly {lookahead} samples early"
        );
    }
}

#[test]
fn independent_curve_and_exact_hold_transitions_agree() {
    let parameters = ReferenceGateExpanderParameters {
        threshold_db: -40.0,
        ratio: 4.0,
        range_db: 12.0,
    };
    assert_eq!(
        reference_gate_expander_gain_reduction_db(-40.0, parameters, ReferenceGatePhase::Closed),
        Ok(0.0)
    );
    assert_eq!(
        reference_gate_expander_gain_reduction_db(-80.0, parameters, ReferenceGatePhase::Closed),
        Ok(-12.0)
    );
    assert_eq!(
        reference_gate_expander_gain_reduction_db(-80.0, parameters, ReferenceGatePhase::Open),
        Ok(0.0)
    );
    assert_eq!(
        reference_gate_expander_gain_reduction_db(
            -80.0,
            ReferenceGateExpanderParameters {
                ratio: 1.0,
                ..parameters
            },
            ReferenceGatePhase::Closed
        ),
        Ok(0.0)
    );

    // The production hold order, observed through the render rather than through a private
    // transition function: a three-sample hold keeps the gate open for exactly three samples
    // below the re-arm threshold, and the in-band level reloads it.
    let hold_samples = 3.0_f32;
    let mut values = initial_values();
    values[0].value = -40.0;
    values[1].value = -40.0;
    values[6].value = 6.0; // hysteresis
    values[7].value = 6.0;
    values[10].value = hold_samples * 1000.0 / 48_000.0;
    values[11].value = hold_samples * 1000.0 / 48_000.0;
    values[14].value = 10.0; // full lookahead, so the detector taps the current sample
    values[15].value = 10.0;
    let frames = 700;
    let mut left = vec![0.0_f32; frames];
    let mut right = vec![0.0_f32; frames];
    // Loud for one sample, then a level inside the hysteresis band, then silence.
    left[0] = 1.0;
    for sample in left.iter_mut().take(6).skip(1) {
        *sample = 10.0_f32.powf(-43.0 / 20.0);
    }
    right.copy_from_slice(&left);
    let source = left.clone();
    let mut effect = prepare(request(&values));
    render_scalar_sidechain(effect.as_mut(), &mut left, &mut right, None, 128, &[], 0);

    let reference_left: Vec<f64> = source.iter().map(|&x| f64::from(x)).collect();
    let timing = ReferenceGateTiming {
        sample_rate: 48_000,
        attack_ms: f64::from(GATE_EXPANDER_PARAMETERS[4].default_value),
        hold_ms: f64::from(values[10].value),
        release_ms: f64::from(GATE_EXPANDER_PARAMETERS[6].default_value),
        lookahead_ms: 10.0,
    };
    let trace = reference_gate_expander_process(
        ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: f64::from(GATE_EXPANDER_PARAMETERS[1].default_value),
            range_db: f64::from(GATE_EXPANDER_PARAMETERS[2].default_value),
        },
        ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: f64::from(GATE_EXPANDER_PARAMETERS[1].default_value),
            range_db: f64::from(GATE_EXPANDER_PARAMETERS[2].default_value),
        },
        (6.0, 6.0),
        (timing, timing),
        ReferenceGateLink::DualMono,
        (&reference_left, &reference_left),
        None,
    )
    .expect("reference render");
    // The band reloaded the hold at frame 5, so the gate closes at frame 5 + 1 + 3 = 9.
    for frame in 0..=8 {
        assert_eq!(
            trace.phase_left[frame],
            ReferenceGatePhase::Open,
            "frame {frame} is inside the hold"
        );
    }
    assert_eq!(trace.phase_left[9], ReferenceGatePhase::Closed);
    // The production render must attenuate from the same frame: the gain is still exactly unity
    // while the gate is open, and moves once it closes.
    assert_eq!(
        trace.gain_db_left[8], 0.0,
        "an open gate is exact unity in the model"
    );
    assert!(
        trace.gain_db_left[9] < 0.0,
        "the model attenuates from the closing frame"
    );
}

#[test]
fn active_sidechain_and_exact_automation_state_are_observable() {
    let values = initial_values();
    let factory = GateExpanderFactory;
    let mut effect = prepare(request(&values));
    let span = PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel: ParameterChannel::Left,
        parameter_index: 0,
        start_sample: 0,
        end_sample: 0,
        start_value: -20.0,
        end_value: -20.0,
    };
    let mut left = [0.0; 32];
    let mut right = [0.0; 32];
    assert_eq!(
        effect.process(
            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[span], 128).expect("block")
        ),
        ProcessReport::default()
    );
    let (_, left_payload, _) = snapshot(effect.as_ref());
    // 64-sample ramp from -40 towards -20, 32 samples in: `-40 + 32 * (20 / 64)`.
    assert_eq!(float(&left_payload, RAMP_WORD), -30.0);
    assert_eq!(float(&left_payload, RAMP_WORD + 1), -20.0, "target");
    assert_eq!(float(&left_payload, RAMP_WORD + 3), 32.0, "remaining");
    assert_eq!(
        float(&left_payload, RAMP_WORD + 2),
        20.0 / 64.0,
        "the step is precomputed once (D11)"
    );

    let malformed = PreparedAutomationSpan {
        channel: ParameterChannel::Both,
        start_sample: 32,
        end_sample: 32,
        start_value: -60.0,
        end_value: -60.0,
        ..span
    };
    let retarget = PreparedAutomationSpan {
        start_sample: 32,
        end_sample: 32,
        start_value: -60.0,
        end_value: -60.0,
        ..span
    };
    let mut left = [0.0; 64];
    let mut right = [0.0; 64];
    let report = effect.process(
        EffectProcessBlock::new(&mut left, &mut right, None, 32, &[malformed, retarget], 128)
            .expect("block"),
    );
    assert_eq!(report.invalid_spans, 1);
    let (_, left_payload, _) = snapshot(effect.as_ref());
    assert_eq!(
        float(&left_payload, RAMP_WORD).to_bits(),
        (-60.0_f32).to_bits()
    );
    assert_eq!(
        float(&left_payload, RAMP_WORD + 3),
        0.0,
        "the ramp finished"
    );
    assert_eq!(word(&left_payload, RAMP_WORD + 2), 0, "and zeroed its step");

    let mut connected_values = initial_values();
    connected_values[10].value = 0.0;
    connected_values[11].value = 0.0;
    connected_values[12].value = 5.0;
    connected_values[13].value = 5.0;
    connected_values[14].value = 10.0;
    connected_values[15].value = 10.0;
    let mut unconnected = prepare(request(&connected_values));
    let mut connected = request(&connected_values);
    connected.ports.sidechain = PreparedSidechainPort::Connected {
        id: sidechain_port(),
        required: false,
    };
    let mut connected_effect = factory.prepare(connected).expect("connected");
    let mut unconnected_left = vec![0.25; 481];
    let mut unconnected_right = vec![0.25; 481];
    let mut connected_left = unconnected_left.clone();
    let mut connected_right = unconnected_right.clone();
    let side_left = vec![0.0; 481];
    let side_right = vec![0.0; 481];
    render_scalar_sidechain(
        unconnected.as_mut(),
        &mut unconnected_left,
        &mut unconnected_right,
        None,
        128,
        &[],
        0,
    );
    render_scalar_sidechain(
        connected_effect.as_mut(),
        &mut connected_left,
        &mut connected_right,
        Some((&side_left, &side_right)),
        128,
        &[],
        0,
    );
    assert_eq!(unconnected_left[480].to_bits(), 0.25_f32.to_bits());
    assert_eq!(unconnected_right[480].to_bits(), 0.25_f32.to_bits());
    assert!(connected_left[480] < unconnected_left[480]);
    assert!(connected_right[480] < unconnected_right[480]);
}

#[test]
fn bank_validation_precedes_fallback_and_unavailable_w4_is_legal() {
    const WIDTH: usize = 4;
    let factory = GateExpanderFactory;
    let values = vec![initial_values(); WIDTH];
    let requests = values
        .iter()
        .map(|value| request(value))
        .collect::<Vec<_>>();
    let unavailable = factory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd4,
            width: BankWidth::Four,
            requests: &requests,
        })
        .expect("valid W4 request");
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    assert!(
        unavailable.is_none(),
        "an eight-lane build has no four-lane backend, which is a legal scalar fallback"
    );
    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    let _ = unavailable;

    let error = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
        backend: Backend::Scalar,
        width: BankWidth::Four,
        requests: &requests,
    }) {
        Ok(_) => panic!("backend/width mismatch must reject"),
        Err(error) => error,
    };
    assert_eq!(error.code, "effect.bank.requests");

    let mut malformed_values = values.clone();
    malformed_values[WIDTH - 1][0].value = f32::NAN;
    let malformed_requests = malformed_values
        .iter()
        .map(|value| request(value))
        .collect::<Vec<_>>();
    let error = match factory.bind_homogeneous_bank(PrepareEffectBankRequest {
        backend: Backend::Simd4,
        width: BankWidth::Four,
        requests: &malformed_requests,
    }) {
        Ok(_) => panic!("a malformed member must reject before the backend fallback"),
        Err(error) => error,
    };
    assert_eq!(error.code, "effect.parameter.initial");

    let mut connected_requests = values
        .iter()
        .map(|value| request(value))
        .collect::<Vec<_>>();
    for item in &mut connected_requests {
        item.ports.sidechain = PreparedSidechainPort::Connected {
            id: sidechain_port(),
            required: false,
        };
    }
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: Backend::Simd4,
                width: BankWidth::Four,
                requests: &connected_requests,
            })
            .expect("connected requests validate")
            .is_none(),
        "a connected sidechain is unbankable, not an error"
    );

    let mut mixed = values
        .iter()
        .map(|value| request(value))
        .collect::<Vec<_>>();
    mixed[1].link_mode = LinkMode::Maximum;
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: Backend::Simd4,
                width: BankWidth::Four,
                requests: &mixed,
            })
            .expect("mixed programs validate")
            .is_none(),
        "a cohort with two programs is unbankable, not an error"
    );
}

// ---------------------------------------------------------------------------------------------
// The two inclusive comparison boundaries of brief 014, on the production path.
//
// "Comparisons at both boundaries are inclusive on the opening/re-arm side" — a closed gate opens
// at `X >= T`, and an open one re-arms at `X >= T - H`. Both are `ge` in the kernel, and the f64
// oracle pins the distinction too; but a corpus of noise and tones never produces an *exact* `f32`
// equality between `level_db` and a threshold, so `>=` and `>` are indistinguishable there. These
// two tests construct the equality deliberately.
//
// The construction is self-verifying, which is what stops it from being a reconstruction of the
// kernel asserted against itself. Each test runs three renders — at the constructed comparand, one
// `f32` step above it and one step below — and asserts the behaviour is (open, closed, open). If
// the constructed level were not exactly on the boundary it would agree with one neighbour and
// disagree with the other, and the test would fail rather than pass vacuously.
// ---------------------------------------------------------------------------------------------

/// The detector level in dB that a constant input of `amplitude` produces, in the kernel's exact
/// operation order: `clamp(fast_level_db(max(u, 1e-8)), -160, 24)` with the D8 select forms of
/// `max`/`min`.
///
/// FAST-DB-RESTATEMENT: this restates the kernel's law in order to *construct a witness on the
/// boundary*, not to pin a
/// value, so it has to track the law: crossing X3 moved the level conversion to the sealed fast
/// tier, and a witness built from the exact tier would no longer land exactly on the threshold.
/// The test's non-vacuity check is what keeps that honest -- each witness is rendered with its two
/// neighbouring `f32` values as well, and the three must behave as (open, closed, open), which a
/// witness sitting one ulp off the boundary cannot do.
fn detector_level_db(amplitude: f32) -> f32 {
    use gate_expander::kernel::{LEVEL_FLOOR, LEVEL_MAX_DB, LEVEL_MIN_DB};
    let floored = if amplitude > LEVEL_FLOOR {
        amplitude
    } else {
        LEVEL_FLOOR
    };
    let raw = math::fast_db::fast_level_db::<f32>(floored);
    let capped = if raw < LEVEL_MAX_DB {
        raw
    } else {
        LEVEL_MAX_DB
    };
    if capped > LEVEL_MIN_DB {
        capped
    } else {
        LEVEL_MIN_DB
    }
}

/// Renders `source` through a scalar instance whose threshold is `threshold`, and returns the
/// output. All other parameters are fixed by the caller's `values`.
fn render_at_threshold(mut values: Values, threshold: f32, source: &[f32]) -> Vec<f32> {
    set_parameter(&mut values, 0, threshold, threshold);
    let mut effect = prepare(request(&values));
    let mut left = source.to_vec();
    let mut right = source.to_vec();
    render_scalar_sidechain(effect.as_mut(), &mut left, &mut right, None, 128, &[], 0);
    left
}

#[test]
fn a_closed_gate_opens_at_a_level_exactly_equal_to_the_threshold() {
    const LATENCY: usize = 480;
    const HOLD_FRAMES: usize = 4_000;
    const PROBE: usize = 200;

    // `a` is the level that must sit exactly on the threshold; `b` is a level inside the
    // hysteresis band, which is where an open gate and a closed one finally differ. At the
    // trigger level itself they do not: `(rho - 1) * (X - T)` is `+0.0` at equality, so a gate
    // that failed to open still applies unity there.
    let trigger = 0.1_f32;
    let band = 0.070_794_58_f32;
    let level = detector_level_db(trigger);
    let band_level = detector_level_db(band);

    let mut values = initial_values();
    set_parameter(&mut values, 1, 20.0, 20.0); // ratio
    set_parameter(&mut values, 2, 48.0, 48.0); // range
    set_parameter(&mut values, 3, 6.0, 6.0); // hysteresis
    set_parameter(&mut values, 4, 1.0, 1.0); // attack ms
    set_parameter(&mut values, 5, 0.0, 0.0); // hold ms: no countdown to hide behind
    set_parameter(&mut values, 6, 5.0, 5.0); // release ms
    set_parameter(&mut values, 7, 0.0, 0.0); // lookahead 0: the detector tap equals the latency

    assert!(
        (-80.0..=0.0).contains(&level),
        "the constructed threshold {level} must be inside the parameter domain"
    );
    assert!(
        band_level < level && band_level > level - 6.0,
        "the band level {band_level} must sit strictly inside (T - H, T)"
    );

    // One `f32` step either side of the constructed threshold. Above it the level is strictly
    // below the threshold; below it, strictly above.
    let above = level.next_up();
    let below = level.next_down();
    assert!(level < above && below < level, "the neighbours bracket it");

    let frames = LATENCY + HOLD_FRAMES + PROBE + 1;
    let mut source = vec![band; frames];
    for sample in source.iter_mut().take(HOLD_FRAMES) {
        *sample = trigger;
    }

    // The gate is closed by the silent pre-roll the ring starts with (hold is zero, so it closes
    // on the first sample below the band), then meets the trigger level at frame `LATENCY`.
    let probe = LATENCY + HOLD_FRAMES + PROBE;
    let expect = |threshold: f32, open: bool, label: &str| {
        let out = render_at_threshold(values, threshold, &source);
        let dry = source[probe - LATENCY];
        assert_eq!(dry.to_bits(), band.to_bits(), "the probe frame is dry");
        if open {
            assert_eq!(
                out[probe].to_bits(),
                band.to_bits(),
                "{label}: the gate opened, so the band level is the exact identity"
            );
        } else {
            assert!(
                out[probe].abs() < band * 0.5,
                "{label}: the gate never opened, so the band level is expanded ({} vs {band})",
                out[probe]
            );
        }
    };

    // `X > T` strictly: both `>=` and `>` open, so this row is the control that the witness is at
    // the boundary rather than below it.
    expect(below, true, "threshold one step below the level");
    // `X < T` strictly: neither opens.
    expect(above, false, "threshold one step above the level");
    // `X == T` exactly. This is the row brief 014 pins, and the row a `>` kernel fails.
    expect(level, true, "threshold exactly equal to the level");
}

#[test]
fn an_open_gate_rearms_at_a_level_exactly_equal_to_the_close_threshold() {
    const LATENCY: usize = 480;
    const PROBE: usize = 1_000;

    // The comparand here is `T - H`, computed inside the kernel as one `f32` subtraction, so the
    // threshold is nudged until that subtraction reproduces the level exactly.
    let hold_level = 0.1_f32;
    let level = detector_level_db(hold_level);
    let hysteresis = 6.0_f32;

    let mut threshold = level + hysteresis;
    while threshold - hysteresis > level {
        threshold = threshold.next_down();
    }
    while threshold - hysteresis < level {
        threshold = threshold.next_up();
    }
    assert_eq!(
        (threshold - hysteresis).to_bits(),
        level.to_bits(),
        "the re-arm boundary must be exactly constructible"
    );
    assert!(
        (-80.0..=0.0).contains(&threshold),
        "the constructed threshold {threshold} must be inside the parameter domain"
    );

    // Step the threshold until `T - H` actually moves off the level in each direction: one step of
    // `T` is not one step of `T - H`, so this searches rather than assuming.
    let mut above = threshold;
    while (above - hysteresis).to_bits() == level.to_bits() {
        above = above.next_up();
    }
    let mut below = threshold;
    while (below - hysteresis).to_bits() == level.to_bits() {
        below = below.next_down();
    }
    assert!(
        above - hysteresis > level && below - hysteresis < level,
        "the neighbours bracket the close threshold"
    );

    let mut values = initial_values();
    set_parameter(&mut values, 1, 20.0, 20.0); // ratio
    set_parameter(&mut values, 2, 48.0, 48.0); // range
    set_parameter(&mut values, 3, hysteresis, hysteresis);
    set_parameter(&mut values, 4, 1.0, 1.0); // attack ms
    set_parameter(&mut values, 5, 0.0, 0.0); // hold ms: nothing to keep it open but the re-arm
    set_parameter(&mut values, 6, 5.0, 5.0); // release ms
    set_parameter(&mut values, 7, 10.0, 10.0); // full lookahead: the detector taps the live sample

    let frames = LATENCY + PROBE + 1;
    let source = vec![hold_level; frames];
    let probe = LATENCY + PROBE;

    let expect = |threshold: f32, open: bool, label: &str| {
        let out = render_at_threshold(values, threshold, &source);
        if open {
            assert_eq!(
                out[probe].to_bits(),
                hold_level.to_bits(),
                "{label}: the gate re-armed, so the output is the exact identity"
            );
        } else {
            assert!(
                out[probe].abs() < hold_level * 0.5,
                "{label}: the gate closed, so the level is expanded ({} vs {hold_level})",
                out[probe]
            );
        }
    };

    // `X > T - H` strictly: both `>=` and `>` re-arm.
    expect(below, true, "close threshold one step below the level");
    // `X < T - H` strictly: neither re-arms, and with a zero hold the gate closes at once.
    expect(above, false, "close threshold one step above the level");
    // `X == T - H` exactly, the row brief 014 pins.
    expect(
        threshold,
        true,
        "close threshold exactly equal to the level",
    );
}
