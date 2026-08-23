//! Gate 7.8: the frozen contract — descriptor, resources, latency, curve, transition, automation,
//! the connected sidechain and bank binding.

mod support;

use miso_engine_core::KernelBackendV1;
use miso_engine_dsp_reference::{
    ReferenceGateExpanderParameters, ReferenceGateLink, ReferenceGatePhase, ReferenceGateTiming,
    reference_gate_expander_gain_reduction_db, reference_gate_expander_process,
};
use miso_engine_effect_contract::{
    AutomationSpanKind, BankWidth, EffectProcessBlock, LatencySamples, LinkMode,
    NativeEffectFactory, ParameterChannel, PrepareEffectBankRequest, PreparedAutomationSpan,
    PreparedSidechainPort, ProcessReport, TailSamples, validate_descriptor_v1,
};
use miso_engine_gate_expander::{
    GATE_EXPANDER_DESCRIPTOR_V1, GATE_EXPANDER_PARAMETERS_V1, GateExpanderFactory,
    STATE_LAYOUT_VERSION,
};
use support::{
    initial_values, prepare, render_scalar_sidechain, request, request_at_rate, sidechain_port,
    snapshot,
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
    validate_descriptor_v1(&GATE_EXPANDER_DESCRIPTOR_V1).expect("descriptor");
    assert_eq!(
        GATE_EXPANDER_DESCRIPTOR_V1.id.as_str(),
        "miso.gate-expander"
    );
    assert_eq!(GATE_EXPANDER_DESCRIPTOR_V1.state_layout_version, 2);
    assert_eq!(STATE_LAYOUT_VERSION, 2);
    // Layout 2: per lane `(23 + 2N) * 4` bytes, plus the runtime codec's two-word common header.
    for (quality, (rate, latency, lane_bytes, total)) in
        GATE_EXPANDER_DESCRIPTOR_V1.qualities.iter().zip([
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
    for quality in GATE_EXPANDER_DESCRIPTOR_V1.qualities {
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
        attack_ms: f64::from(GATE_EXPANDER_PARAMETERS_V1[4].default_value),
        hold_ms: f64::from(values[10].value),
        release_ms: f64::from(GATE_EXPANDER_PARAMETERS_V1[6].default_value),
        lookahead_ms: 10.0,
    };
    let trace = reference_gate_expander_process(
        ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: f64::from(GATE_EXPANDER_PARAMETERS_V1[1].default_value),
            range_db: f64::from(GATE_EXPANDER_PARAMETERS_V1[2].default_value),
        },
        ReferenceGateExpanderParameters {
            threshold_db: -40.0,
            ratio: f64::from(GATE_EXPANDER_PARAMETERS_V1[1].default_value),
            range_db: f64::from(GATE_EXPANDER_PARAMETERS_V1[2].default_value),
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
            backend: KernelBackendV1::WasmSimd128,
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
        backend: KernelBackendV1::Scalar,
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
        backend: KernelBackendV1::WasmSimd128,
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
                backend: KernelBackendV1::WasmSimd128,
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
                backend: KernelBackendV1::WasmSimd128,
                width: BankWidth::Four,
                requests: &mixed,
            })
            .expect("mixed programs validate")
            .is_none(),
        "a cohort with two programs is unbankable, not an error"
    );
}
