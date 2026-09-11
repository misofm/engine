//! E13 — the current causal compressor contract.
//!
//! Descriptor, resource, zero-latency, causal sample-zero, bank-fallback, link, sidechain and
//! malformed-block behavior are asserted here. The contract intentionally changes the former
//! lookahead payload and parameter menu under issue #737; root-owned current fixture pins move with
//! that amended descriptor.

mod support;

use compressor::{COMPRESSOR_DESCRIPTOR, COMPRESSOR_PARAMETERS, CompressorFactory};
use effect_contract::{
    BankProcessReport, BankWidth, EffectBankProcessBlock, EffectProcessBlock, LatencySamples,
    LinkMode, NativeEffectFactory, PrepareEffectBankRequest, PreparedSidechainPort,
    expected_prepared_metadata, validate_descriptor,
};
use lane::Backend;

use support::{
    PARAMETER_COUNT, STATE_HEADER_WORDS, initial_values, prepare, render_scalar, request,
    sidechain_port, values_with,
};

/// Descriptor rows, latency, payload sizes, scratch and the resource envelope are frozen.
///
/// Red mutation: `scratch_fixed_bytes: 0` or `STATE_HEADER_WORDS = 26` — RED here, which is the
/// point: the causal resource envelope and exact 22-word channel payload are both contract data.
#[test]
fn descriptor_rows_and_resource_envelope_are_frozen() {
    validate_descriptor(&COMPRESSOR_DESCRIPTOR).expect("descriptor");
    assert_eq!(COMPRESSOR_DESCRIPTOR.id.as_str(), "miso.compressor");
    assert_eq!(COMPRESSOR_DESCRIPTOR.state_layout_version, 1);
    assert_eq!(COMPRESSOR_PARAMETERS.len(), PARAMETER_COUNT);
    for (quality, (rate, latency, lane_bytes, total_bytes)) in
        COMPRESSOR_DESCRIPTOR.qualities.iter().zip([
            (44_100_u32, 0_u64, 88_u32, 176_u64),
            (48_000, 0, 88, 176),
            (88_200, 0, 88, 176),
            (96_000, 0, 88, 176),
        ])
    {
        assert_eq!(quality.sample_rate, rate);
        assert_eq!(quality.latency, LatencySamples(latency));
        assert_eq!(quality.maximum_state.common_bytes, 0);
        assert_eq!(quality.maximum_state.left_bytes, lane_bytes);
        assert_eq!(quality.maximum_state.right_bytes, lane_bytes);
        assert_eq!(quality.maximum_state.total(), Some(total_bytes));
        assert_eq!(quality.scratch_fixed_bytes, 64);
        assert_eq!(quality.scratch_bytes_per_frame, 0);
        assert_eq!(lane_bytes as usize, STATE_HEADER_WORDS * 4);
    }
}

/// Every launch rate takes the same direct current-sample path in scalar and the supported native
/// bank. The identity configurations use a sample-zero impulse so a delayed implementation cannot
/// satisfy the bypass, ratio-one, or mix-zero assertions by returning a later sample.
#[test]
fn every_launch_rate_processes_scalar_and_supported_bank_at_zero_latency() {
    const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
    let factory = CompressorFactory;
    let active_values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (3, 0.1), (6, 1.0)]);
    let identity_values = values_with(&[(0, -40.0), (1, 1.0), (2, 0.0), (5, 0.0), (6, 1.0)]);
    let mix_zero_values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (6, 0.0)]);

    for rate in RATES {
        let mut active_request = request(&active_values);
        active_request.sample_rate = rate;
        let mut active = prepare(active_request);
        assert_eq!(active.metadata().latency, LatencySamples(0));
        let mut left = vec![0.0_f32; 128];
        let mut right = vec![0.0_f32; 128];
        left[0] = 0.75;
        right[0] = -0.5;
        render_scalar(active.as_mut(), &mut left, &mut right, 128, 128, &[]);
        assert!(left[0].is_finite() && left[0].to_bits() != 0.75_f32.to_bits());
        assert!(right[0].is_finite() && right[0].to_bits() != (-0.5_f32).to_bits());

        let mut bypass_request = request(&active_values);
        bypass_request.sample_rate = rate;
        bypass_request.bypass = true;
        let mut bypass = prepare(bypass_request);
        let mut bypass_left = vec![0.0_f32; 128];
        let mut bypass_right = vec![0.0_f32; 128];
        bypass_left[0] = 0.75;
        bypass_right[0] = -0.5;
        render_scalar(
            bypass.as_mut(),
            &mut bypass_left,
            &mut bypass_right,
            128,
            128,
            &[],
        );
        assert_eq!(bypass_left[0].to_bits(), 0.75_f32.to_bits());
        assert_eq!(bypass_right[0].to_bits(), (-0.5_f32).to_bits());

        for (name, values) in [
            ("ratio-one", &identity_values),
            ("mix-zero", &mix_zero_values),
        ] {
            let mut identity_request = request(values);
            identity_request.sample_rate = rate;
            let mut identity = prepare(identity_request);
            let mut identity_left = vec![0.0_f32; 128];
            let mut identity_right = vec![0.0_f32; 128];
            identity_left[0] = 0.75;
            identity_right[0] = -0.5;
            render_scalar(
                identity.as_mut(),
                &mut identity_left,
                &mut identity_right,
                128,
                128,
                &[],
            );
            assert_eq!(
                identity_left[0].to_bits(),
                0.75_f32.to_bits(),
                "{name} left rate {rate}"
            );
            assert_eq!(
                identity_right[0].to_bits(),
                (-0.5_f32).to_bits(),
                "{name} right rate {rate}"
            );
        }

        let Some((backend, width)) = support::native_bank_width() else {
            continue;
        };
        let lanes = width.lanes() as usize;
        let requests: Vec<_> = (0..lanes)
            .map(|_| {
                let mut request = request(&active_values);
                request.sample_rate = rate;
                request
            })
            .collect();
        let mut bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &requests,
            })
            .expect("supported bank bind")
            .expect("supported bank");
        let mut bank_left = vec![0.75_f32; lanes];
        let mut bank_right = vec![-0.5_f32; lanes];
        let offsets = vec![0_u32; lanes + 1];
        bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bank_left,
                &mut bank_right,
                None,
                1,
                width,
                0,
                &[],
                &offsets,
                128,
            )
            .expect("bank impulse"),
        );
        assert!(
            bank_left
                .iter()
                .any(|sample| sample.to_bits() != 0.75_f32.to_bits())
        );

        let bypass_requests: Vec<_> = (0..lanes)
            .map(|_| {
                let mut request = request(&active_values);
                request.sample_rate = rate;
                request.bypass = true;
                request
            })
            .collect();
        let mut bypass_bank = factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &bypass_requests,
            })
            .expect("supported bypass bank bind")
            .expect("supported bypass bank");
        let mut bypass_bank_left = vec![0.75_f32; lanes];
        let mut bypass_bank_right = vec![-0.5_f32; lanes];
        bypass_bank.process_bank(
            EffectBankProcessBlock::new(
                &mut bypass_bank_left,
                &mut bypass_bank_right,
                None,
                1,
                width,
                0,
                &[],
                &offsets,
                128,
            )
            .expect("bank bypass impulse"),
        );
        assert!(
            bypass_bank_left
                .iter()
                .all(|sample| sample.to_bits() == 0.75_f32.to_bits())
        );
        assert!(
            bypass_bank_right
                .iter()
                .all(|sample| sample.to_bits() == (-0.5_f32).to_bits())
        );
    }
}

/// The runtime's parameter specs describe exactly the descriptor rows they are derived from.
///
/// The crate carries no domain predicate of its own any more: `params::parameter_value_valid` is
/// the workspace's one implementation and `design::PARAMETER_SPECS` is the descriptor expressed in
/// its shape. This test is what stops the two drifting.
///
/// Red mutation: change any `minimum`/`maximum`/`default_value` in one of the two places — RED.
#[test]
fn every_descriptor_row_admits_exactly_its_own_domain() {
    let factory = CompressorFactory;
    for (index, parameter) in COMPRESSOR_PARAMETERS.iter().enumerate() {
        let minimum = parameter.minimum.expect("continuous minimum");
        let maximum = parameter.maximum.expect("continuous maximum");
        assert!(minimum <= parameter.default_value && parameter.default_value <= maximum);
        for (value, admitted) in [
            (minimum, true),
            (maximum, true),
            (parameter.default_value, true),
            (minimum - 1.0, false),
            (maximum + 1.0, false),
            (f32::NAN, false),
            (f32::INFINITY, false),
        ] {
            let values = values_with(&[(index, value)]);
            let accepted = factory.prepare(request(&values)).is_ok();
            assert_eq!(
                accepted, admitted,
                "parameter {index} value {value}: accepted {accepted}, expected {admitted}"
            );
        }
    }
    // `-0.0` is a preparation-time rejection for every parameter whose domain contains zero.
    for index in [2_usize, 5, 6] {
        let values = values_with(&[(index, -0.0)]);
        assert!(
            factory.prepare(request(&values)).is_err(),
            "parameter {index} must reject -0.0 at preparation"
        );
    }
}

/// Preparation metadata matches the contract's own derivation, and one byte below either limit
/// rejects.
#[test]
fn preparation_has_expected_metadata_and_one_byte_below_rejects() {
    let values = initial_values();
    let factory = CompressorFactory;
    let effect = factory.prepare(request(&values)).expect("prepare");
    assert_eq!(
        effect.metadata().latency,
        expected_prepared_metadata(&COMPRESSOR_DESCRIPTOR, request(&values))
            .expect("metadata")
            .latency
    );

    let mut below = request(&values);
    below.limits.maximum_total_state_bytes -= 1;
    assert_eq!(
        factory.prepare(below).err().expect("state limit").code,
        "effect.resource.limit"
    );

    let mut below_scratch = request(&values);
    below_scratch.limits.maximum_scratch_bytes -= 1;
    assert_eq!(
        factory
            .prepare(below_scratch)
            .err()
            .expect("scratch limit")
            .code,
        "effect.resource.limit"
    );
}

/// The causal contract reports zero latency and consumes the current sample at sample zero.
#[test]
fn causal_processing_starts_at_sample_zero() {
    let values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (3, 0.1), (6, 1.0)]);
    let mut effect = prepare(request(&values));
    assert_eq!(effect.metadata().latency, LatencySamples(0));
    let mut left = vec![0.5_f32; 128];
    let mut right = vec![0.5_f32; 128];
    render_scalar(effect.as_mut(), &mut left, &mut right, 128, 128, &[]);
    assert!(
        left[0].is_finite() && left[0] > 0.0 && left[0] < 0.5,
        "active compression must produce a finite positive compressed sample at zero: {}",
        left[0]
    );
    assert!(left.iter().all(|sample| sample.is_finite()));
}

/// A bank fallback never hides a malformed or incompatible request.
///
/// Rewritten on `lane::Backend`: the "unavailable backend" is a width this build was
/// not compiled for, and the property under test is unchanged — validation happens **before** any
/// `Ok(None)`.
///
/// Red mutation: move the `Backend::current().width() != lanes` check above the per-request
/// validation loop in `bind_homogeneous_bank` — RED on the first case.
#[test]
fn bank_fallback_never_hides_malformed_or_incompatible_requests() {
    let factory = CompressorFactory;
    // A width this build cannot run, chosen so the fallback path is the one under test.
    let (backend, width) = if cfg!(any(target_arch = "x86", target_arch = "x86_64")) {
        (Backend::Simd4, BankWidth::Four)
    } else {
        (Backend::Simd8, BankWidth::Eight)
    };
    let lanes = width.lanes() as usize;

    let mut malformed = vec![initial_values(); lanes];
    malformed[lanes - 1][0].value = f32::NAN;
    let malformed_requests = malformed.iter().map(|v| request(v)).collect::<Vec<_>>();
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &malformed_requests,
            })
            .err()
            .expect("malformed values must not be hidden")
            .code,
        "effect.parameter.initial"
    );

    let connected_values = vec![initial_values(); lanes];
    let mut connected = connected_values
        .iter()
        .map(|v| request(v))
        .collect::<Vec<_>>();
    for item in &mut connected {
        item.ports.sidechain = PreparedSidechainPort::Connected {
            id: sidechain_port(),
            required: false,
        };
    }
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &connected,
            })
            .expect("connected fallback")
            .is_none()
    );
    connected[lanes - 1].limits.maximum_total_state_bytes -= 1;
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &connected,
            })
            .err()
            .expect("connected fallback still validates every request")
            .code,
        "effect.resource.limit"
    );

    let heterogeneous_values = vec![initial_values(); lanes];
    let mut heterogeneous = heterogeneous_values
        .iter()
        .map(|v| request(v))
        .collect::<Vec<_>>();
    heterogeneous[lanes - 1].bypass = true;
    assert!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend,
                width,
                requests: &heterogeneous,
            })
            .expect("heterogeneous fallback")
            .is_none()
    );

    // A backend and a width that do not describe the same lane count is malformed, not a fallback.
    assert_eq!(
        factory
            .bind_homogeneous_bank(PrepareEffectBankRequest {
                backend: Backend::Scalar,
                width,
                requests: &heterogeneous,
            })
            .err()
            .expect("mismatched backend and width")
            .code,
        "effect.bank.requests"
    );
}

/// The three link laws are exact, and a connected sidechain detects something different from the
/// main input.
#[test]
fn links_are_exact_and_connected_sidechain_is_distinct_from_main_detection() {
    // 0.5 * |l| + 0.5 * |r| in the frozen product order, checked through the rendered output of a
    // configuration whose gain is a pure function of the detector level.
    let values = values_with(&[(0, -40.0), (1, 20.0), (2, 0.0), (3, 0.1), (6, 1.0)]);

    let mut outputs = Vec::new();
    for link in [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average] {
        let mut preparation = request(&values);
        preparation.link_mode = link;
        let mut effect = prepare(preparation);
        let mut left = vec![0.25_f32; 1_024];
        let mut right = vec![0.75_f32; 1_024];
        render_scalar(effect.as_mut(), &mut left, &mut right, 128, 128, &[]);
        outputs.push((left[1_000], right[1_000]));
    }
    // Dual mono: the quieter channel is compressed less than the louder one.
    assert!(outputs[0].0 / 0.25 > outputs[0].1 / 0.75);
    // Maximum: both channels ride the louder one, so both have the louder one's gain.
    assert!((outputs[1].0 / 0.25 - outputs[1].1 / 0.75).abs() < 1.0e-6);
    assert!(outputs[1].0 / 0.25 < outputs[0].0 / 0.25);
    // Average: 0.5*0.25 + 0.5*0.75 = 0.5, between the two, and again equal on both channels.
    assert!((outputs[2].0 / 0.25 - outputs[2].1 / 0.75).abs() < 1.0e-6);
    assert!(outputs[2].0 / 0.25 > outputs[1].0 / 0.25);

    // A connected but silent sidechain detects silence, so the output is the dry signal exactly.
    let mut connected_request = request(&values);
    connected_request.ports.sidechain = PreparedSidechainPort::Connected {
        id: sidechain_port(),
        required: false,
    };
    let mut connected = prepare(connected_request);
    let mut left = vec![0.25_f32; 1_024];
    let mut right = vec![0.25_f32; 1_024];
    let sidechain_left = vec![0.0_f32; 128];
    let sidechain_right = vec![0.0_f32; 128];
    let mut offset = 0;
    while offset < left.len() {
        connected.process(
            EffectProcessBlock::new(
                &mut left[offset..offset + 128],
                &mut right[offset..offset + 128],
                Some((&sidechain_left, &sidechain_right)),
                offset as u64,
                &[],
                128,
            )
            .expect("block"),
        );
        offset += 128;
    }
    assert_eq!(left[1_000].to_bits(), 0.25_f32.to_bits());
    assert_eq!(right[1_000].to_bits(), 0.25_f32.to_bits());
}

/// The bank's per-block guard rejects a malformed block before it indexes anything.
///
/// The pre-audit guard was four inline conditions that accepted `frames == 0`, never checked the
/// slice lengths and indexed `automation_offsets` before checking them. 83c deferred the shared
/// validator to #95; this is the strengthened form in the meantime. A rejected block returns an
/// empty report and leaves the buffers untouched — it never panics and never renders.
///
/// Red mutation: `offsets_are_ordered` returning `true` unconditionally — RED (index out of
/// bounds), which is exactly the failure mode the check exists to prevent.
#[test]
fn a_malformed_bank_block_is_rejected_before_it_is_indexed() {
    let Some((_, width)) = support::native_bank_width() else {
        println!("scalar-only build: no bank to guard");
        return;
    };
    let lanes = width.lanes() as usize;
    let values = vec![initial_values(); lanes];
    let requests: Vec<_> = values.iter().map(|v| request(v)).collect();
    let mut bank = support::bind_bank(&requests).expect("bank");

    let frames = 64_u32;
    let mut left = vec![0.5_f32; frames as usize * lanes];
    let mut right = vec![0.5_f32; frames as usize * lanes];
    let good_offsets = vec![0_u32; lanes + 1];

    // Offsets that run past the end of the span slice.
    let mut bad_offsets = vec![0_u32; lanes + 1];
    bad_offsets[lanes] = 4;
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &bad_offsets,
    });
    assert_eq!(report, BankProcessReport::empty(width));
    assert!(
        left.iter()
            .all(|sample| sample.to_bits() == 0.5_f32.to_bits())
    );

    // Descending offsets.
    let mut descending = vec![0_u32; lanes + 1];
    descending[0] = 1;
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &descending,
    });
    assert_eq!(report, BankProcessReport::empty(width));

    // Zero frames.
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames: 0,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &good_offsets,
    });
    assert_eq!(report, BankProcessReport::empty(width));

    // A slice that does not match `frames * lanes`.
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames: frames + 1,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &good_offsets,
    });
    assert_eq!(report, BankProcessReport::empty(width));

    // Too few offsets to describe the lanes.
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &good_offsets[..lanes],
    });
    assert_eq!(report, BankProcessReport::empty(width));

    // And a well-formed block still renders.
    let report = bank.process_bank(EffectBankProcessBlock {
        left: &mut left,
        right: &mut right,
        sidechain: None,
        frames,
        width,
        first_sample: 0,
        automation: &[],
        automation_offsets: &good_offsets,
    });
    assert_eq!(report, BankProcessReport::empty(width));
    assert!(
        left.iter()
            .any(|sample| sample.to_bits() != 0.5_f32.to_bits()),
        "a well-formed bank block must render rather than leave the input untouched"
    );
}
